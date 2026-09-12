use std::io::Write;
use std::sync::Arc;

use flate2::Compression;
use flate2::write::ZlibEncoder;
use pdf_writer::types::{PaintType, TilingType};
use pdf_writer::{Content, Filter, Finish, Name, Pdf, Rect, Ref, Settings};

use super::direct::RefAllocator;
use super::direct_image::{DirectImageSet, RegisteredImage};
use super::image::{
  DirectRasterColorSpace, DirectRasterEncoding, DirectRasterImage, PdfRasterPixels,
  PreparedRasterImage,
};
use crate::error::{PdfError, Result};
use crate::options::PdfOptimizeFor;
use ooxmlsdk_layout::common;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RegisteredTilingPattern {
  pub(super) name: Vec<u8>,
  pub(super) id: Ref,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PatternKey {
  fill: common::PatternFill,
  sampling: PatternSampling,
  matrix_bits: [u32; 6],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PatternSampling {
  image_size_px: u8,
  tile_repetitions: u8,
}

#[derive(Debug)]
struct PatternImage {
  fill: common::PatternFill,
  sampling: PatternSampling,
  registered: RegisteredImage,
}

#[derive(Debug)]
struct PatternObject {
  key: PatternKey,
  registered: RegisteredTilingPattern,
  image: RegisteredImage,
  pattern_units: f32,
  matrix: [f32; 6],
  content: Vec<u8>,
}

/// Document-wide colored tiling-pattern registry.
///
/// The layout layer preserves both the semantic 8x8 period and Office's
/// authored bitmap sampling lattice. The PDF object keeps that sampled tile
/// as a non-interpolated image and maps it into page space with a pattern
/// matrix. Pages and isolated form XObjects own only resource-name mappings.
#[derive(Debug)]
pub(super) struct DirectPatternSet {
  compress_streams: bool,
  engine_kind: common::LayoutEngineKind,
  optimize_for: PdfOptimizeFor,
  images: Vec<PatternImage>,
  patterns: Vec<PatternObject>,
}

impl DirectPatternSet {
  pub(super) fn new(
    compress_streams: bool,
    engine_kind: common::LayoutEngineKind,
    optimize_for: PdfOptimizeFor,
  ) -> Self {
    Self {
      compress_streams,
      engine_kind,
      optimize_for,
      images: Vec::new(),
      patterns: Vec::new(),
    }
  }

  pub(super) fn validate(fill: common::PatternFill) -> Result<()> {
    let tile_size_pt = fill.bitmap_tile_size_points();
    let pattern_units = f32::from(fill.bitmap_sampling.image_size_px());
    let scale = tile_size_pt / pattern_units;
    if !tile_size_pt.is_finite() || tile_size_pt <= 0.0 {
      return Err(PdfError::Writer(
        "tiling-pattern cell size must be positive".to_string(),
      ));
    }
    if !pattern_units.is_finite() || pattern_units <= 0.0 || !scale.is_finite() || scale <= 0.0 {
      return Err(PdfError::Writer(
        "tiling-pattern sampling lattice must be positive".to_string(),
      ));
    }
    Ok(())
  }

  pub(super) fn register(
    &mut self,
    fill: common::PatternFill,
    origin_x_pt: f32,
    origin_y_pt: f32,
    page_height_pt: f32,
    images: &mut DirectImageSet,
    refs: &mut RefAllocator,
  ) -> Result<RegisteredTilingPattern> {
    Self::validate(fill)?;
    let sampling = self.resolved_sampling(fill);
    let (pattern_units, matrix) =
      resolved_pattern_geometry(fill, sampling, origin_x_pt, origin_y_pt, page_height_pt)?;
    let key = PatternKey {
      fill,
      sampling,
      matrix_bits: matrix.map(f32::to_bits),
    };
    if let Some(pattern) = self.patterns.iter().find(|pattern| pattern.key == key) {
      return Ok(pattern.registered.clone());
    }

    let image = if let Some(image) = self
      .images
      .iter()
      .find(|image| image.fill == fill && image.sampling == sampling)
    {
      image.registered.clone()
    } else {
      let prepared = pattern_tile_image(fill, sampling)?;
      let registered = images.register(prepared, || refs.alloc())?;
      self.images.push(PatternImage {
        fill,
        sampling,
        registered: registered.clone(),
      });
      registered
    };

    let index = self.patterns.len();
    let registered = RegisteredTilingPattern {
      name: format!("TP{index}").into_bytes(),
      id: refs.alloc()?,
    };
    let mut content = Content::with_settings(Settings { pretty: false });
    content
      .save_state()
      .transform([pattern_units, 0.0, 0.0, pattern_units, 0.0, 0.0])
      .x_object(Name(&image.name))
      .restore_state();
    self.patterns.push(PatternObject {
      key,
      registered: registered.clone(),
      image,
      pattern_units,
      matrix,
      content: content.finish().into_vec(),
    });
    Ok(registered)
  }

  fn resolved_sampling(&self, fill: common::PatternFill) -> PatternSampling {
    if self.engine_kind == common::LayoutEngineKind::Docx
      && self.optimize_for == PdfOptimizeFor::Screen
      && matches!(fill.mask, common::PatternMask::EmfPlusHatch(_))
    {
      // Word's screen fixed-output path samples one semantic 6pt hatch cell
      // into a 7x7 bitmap. Its print path samples the same mask into 16x16.
      // An exact-config Office matrix covering foreground/background alpha at
      // 0%, 50%, 80%, 99.999%, and 100% keeps this topology invariant: output
      // quality, not transparency, owns the branch. VML Bitmap8 brushes keep
      // their separately authored sampling topology.
      PatternSampling {
        image_size_px: 7,
        tile_repetitions: 1,
      }
    } else {
      PatternSampling {
        image_size_px: fill.bitmap_sampling.image_size_px(),
        tile_repetitions: fill.bitmap_sampling.tile_repetitions(),
      }
    }
  }

  pub(super) fn write_objects(&self, pdf: &mut Pdf) -> Result<()> {
    for pattern in &self.patterns {
      let compressed;
      let content = if self.compress_streams {
        compressed = deflate(&pattern.content)?;
        compressed.as_slice()
      } else {
        pattern.content.as_slice()
      };
      i32::try_from(content.len()).map_err(|_| {
        PdfError::Writer("tiling-pattern stream exceeds the PDF integer range".to_string())
      })?;

      let mut object = pdf.tiling_pattern(pattern.registered.id, content);
      if self.compress_streams {
        object.filter(Filter::FlateDecode);
      }
      object
        .paint_type(PaintType::Colored)
        .tiling_type(TilingType::NoDistortion)
        .bbox(Rect::new(
          0.0,
          0.0,
          pattern.pattern_units,
          pattern.pattern_units,
        ))
        .x_step(pattern.pattern_units)
        .y_step(pattern.pattern_units)
        .matrix(pattern.matrix);
      {
        let mut resources = object.resources();
        resources
          .x_objects()
          .pair(Name(&pattern.image.name), pattern.image.id);
        resources.finish();
      }
      object.finish();
    }
    Ok(())
  }
}

fn resolved_pattern_geometry(
  fill: common::PatternFill,
  sampling: PatternSampling,
  origin_x_pt: f32,
  origin_y_pt: f32,
  page_height_pt: f32,
) -> Result<(f32, [f32; 6])> {
  if !origin_x_pt.is_finite() || !origin_y_pt.is_finite() || !page_height_pt.is_finite() {
    return Err(PdfError::Writer(
      "tiling-pattern origin and page height must be finite".to_string(),
    ));
  }
  if page_height_pt <= 0.0 {
    return Err(PdfError::Writer(
      "tiling-pattern page height must be positive".to_string(),
    ));
  }

  let tile_size_pt = fill.tile_size_points() * f32::from(sampling.tile_repetitions);
  let pattern_units = f32::from(sampling.image_size_px);
  let scale = if sampling.image_size_px == 7 && sampling.tile_repetitions == 1 {
    // Word's PDF serializer writes the exact-config screen lattice at five
    // decimal places rather than serializing the longer binary value of 6/7.
    0.857_14
  } else {
    tile_size_pt / pattern_units
  };
  let phase_x = if fill.page_origin {
    0.0
  } else {
    pattern_origin(origin_x_pt, tile_size_pt)
  };
  // Layout coordinates are y-down, but PDF pattern matrices are expressed in
  // the parent stream's default y-up user space. This is the same boundary as
  // the page-root reflection and matches Office's fixed-output matrices.
  let phase_y = page_height_pt
    - if fill.page_origin {
      0.0
    } else {
      pattern_origin(origin_y_pt, tile_size_pt)
    };
  let matrix = [scale, 0.0, 0.0, scale, phase_x, phase_y];
  if matrix.into_iter().any(|value| !value.is_finite()) {
    return Err(PdfError::Writer(
      "tiling-pattern matrix contains a non-finite value".to_string(),
    ));
  }
  Ok((pattern_units, matrix))
}

fn pattern_origin(value: f32, tile_size_pt: f32) -> f32 {
  (value / tile_size_pt).floor() * tile_size_pt
}

fn pattern_tile_image(
  fill: common::PatternFill,
  sampling: PatternSampling,
) -> Result<PreparedRasterImage> {
  let image_size = u32::from(sampling.image_size_px);
  let pixel_count = usize::try_from(image_size)
    .ok()
    .and_then(|size| size.checked_mul(size))
    .ok_or_else(|| PdfError::Writer("tiling-pattern sample count overflows usize".to_string()))?;
  let mut rgb = Vec::with_capacity(pixel_count * 3);
  let has_alpha = fill.foreground.a != u8::MAX || fill.background.a != u8::MAX;
  let mut alpha = has_alpha.then(|| Vec::with_capacity(pixel_count));
  let mask_scale = 8 * u32::from(sampling.tile_repetitions);
  for y in 0..image_size {
    for x in 0..image_size {
      let column = x.saturating_mul(mask_scale) / image_size;
      let row = y.saturating_mul(mask_scale) / image_size;
      let color = if fill.is_foreground(column as i32, row as i32) {
        fill.foreground
      } else {
        fill.background
      };
      rgb.extend_from_slice(&[color.r, color.g, color.b]);
      if let Some(alpha) = &mut alpha {
        alpha.push(color.a);
      }
    }
  }

  Ok(PreparedRasterImage::new(DirectRasterImage {
    width: image_size,
    height: image_size,
    color_space: DirectRasterColorSpace::Rgb,
    bits_per_component: 8,
    encoding: DirectRasterEncoding::Sampled {
      pixels: Arc::new(PdfRasterPixels {
        width: image_size,
        height: image_size,
        rgb,
        alpha,
        icc_profile: None,
      }),
    },
    // Office preserves the authored hatch lattice. Interpolation would blend
    // foreground and background cells and alter both density and RGB output.
    interpolate: false,
    soft_mask_interpolate: false,
    matte: None,
  }))
}

fn deflate(content: &[u8]) -> Result<Vec<u8>> {
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
  encoder
    .write_all(content)
    .map_err(|error| PdfError::Writer(format!("tiling-pattern compression failed: {error}")))?;
  encoder
    .finish()
    .map_err(|error| PdfError::Writer(format!("tiling-pattern compression failed: {error}")))
}

#[cfg(test)]
mod tests {
  use super::*;
  use emfsdk::emfplus::EmfPlusHatchStyle;

  fn color(r: u8, g: u8, b: u8, a: u8) -> common::Color {
    common::Color { r, g, b, a }
  }

  #[test]
  fn drawingml_pattern_matches_office_sampling_and_page_phase() {
    let fill = common::PatternFill::drawingml(
      EmfPlusHatchStyle::LightHorizontal,
      color(153, 255, 102, 255),
      color(255, 255, 255, 255),
    );
    let mut refs = RefAllocator::default();
    let mut images = DirectImageSet::default();
    let mut patterns =
      DirectPatternSet::new(false, common::LayoutEngineKind::Docx, PdfOptimizeFor::Print);

    let registered = patterns
      .register(fill, 243.125, 101.681_526, 792.0, &mut images, &mut refs)
      .unwrap();
    let same_cell = patterns
      .register(fill, 245.0, 101.9, 792.0, &mut images, &mut refs)
      .unwrap();
    assert_eq!(registered, same_cell);

    let mut pdf = Pdf::new();
    patterns.write_objects(&mut pdf).unwrap();
    images.write_objects(&mut pdf).unwrap();
    let bytes = pdf.finish();
    let source = String::from_utf8_lossy(&bytes);
    assert!(source.contains("/PatternType 1"));
    assert!(source.contains("/PaintType 1"));
    assert!(source.contains("/TilingType 2"));
    assert!(source.contains("/BBox [0 0 16 16]"));
    assert!(source.contains("/XStep 16"));
    assert!(source.contains("/YStep 16"));
    assert!(source.contains("/Matrix [0.375 0 0 0.375 240 696]"));
    assert!(source.contains("q 16 0 0 16 0 0 cm/Im0 Do\nQ"));
    assert!(!source.contains("/Interpolate true"));
  }

  #[test]
  fn word_screen_pattern_uses_the_quality_owned_seven_pixel_lattice() {
    let fill = common::PatternFill::drawingml(
      EmfPlusHatchStyle::LightVertical,
      color(155, 187, 89, 204),
      color(255, 255, 255, 204),
    );
    let mut refs = RefAllocator::default();
    let mut images = DirectImageSet::default();
    let mut patterns = DirectPatternSet::new(
      false,
      common::LayoutEngineKind::Docx,
      PdfOptimizeFor::Screen,
    );
    patterns
      .register(fill, 367.197_72, 0.4, 792.0, &mut images, &mut refs)
      .unwrap();

    let mut pdf = Pdf::new();
    patterns.write_objects(&mut pdf).unwrap();
    images.write_objects(&mut pdf).unwrap();
    let bytes = pdf.finish();
    let source = String::from_utf8_lossy(&bytes);
    assert!(source.contains("/BBox [0 0 7 7]"));
    assert!(source.contains("/XStep 7"));
    assert!(source.contains("/YStep 7"));
    assert!(source.contains("/Matrix [0.85714 0 0 0.85714 366 792]"));
    assert!(source.contains("q 7 0 0 7 0 0 cm/Im0 Do\nQ"));

    let pptx_screen = DirectPatternSet::new(
      false,
      common::LayoutEngineKind::Pptx,
      PdfOptimizeFor::Screen,
    );
    assert_eq!(
      pptx_screen.resolved_sampling(fill),
      PatternSampling {
        image_size_px: 16,
        tile_repetitions: 1,
      }
    );
    let bitmap = common::PatternFill::bitmap8(
      [0xAA; 8],
      8_000,
      color(0, 0, 0, 255),
      color(255, 255, 255, 255),
    );
    assert_eq!(
      patterns.resolved_sampling(bitmap),
      PatternSampling {
        image_size_px: 8,
        tile_repetitions: 1,
      }
    );
  }

  #[test]
  fn worksheet_pattern_keeps_page_phase_across_cell_origins() {
    let mut fill = common::PatternFill::bitmap8(
      [0x18, 0x30, 0x60, 0xc0, 0x81, 0x03, 0x06, 0x0c],
      960,
      color(0, 0, 0, 255),
      color(255, 255, 255, 255),
    );
    let sampling = PatternSampling {
      image_size_px: 8,
      tile_repetitions: 1,
    };
    let local = resolved_pattern_geometry(fill, sampling, 54.0, 87.3, 841.92).unwrap();
    fill.page_origin = true;
    let first = resolved_pattern_geometry(fill, sampling, 54.0, 87.3, 841.92).unwrap();
    let second = resolved_pattern_geometry(fill, sampling, 106.32, 103.68, 841.92).unwrap();
    assert_eq!(first, second);
    assert_eq!(first, (8.0, [0.12, 0.0, 0.0, 0.12, 0.0, 841.92]));
    assert_ne!(first, local);
  }

  #[test]
  fn pattern_alpha_is_owned_by_the_tile_image_soft_mask() {
    let fill = common::PatternFill::bitmap8(
      [0xAA; 8],
      8_000,
      color(255, 0, 0, 128),
      color(0, 0, 255, 64),
    );
    let mut refs = RefAllocator::default();
    let mut images = DirectImageSet::default();
    let mut patterns =
      DirectPatternSet::new(false, common::LayoutEngineKind::Docx, PdfOptimizeFor::Print);
    patterns
      .register(fill, 0.0, 0.0, 792.0, &mut images, &mut refs)
      .unwrap();

    let mut pdf = Pdf::new();
    patterns.write_objects(&mut pdf).unwrap();
    images.write_objects(&mut pdf).unwrap();
    let bytes = pdf.finish();
    let source = String::from_utf8_lossy(&bytes);
    assert!(source.contains("/SMask"));
    assert!(source.contains("/DeviceGray"));
  }

  #[test]
  fn invalid_zero_sized_pattern_is_rejected_before_pdf_writer() {
    let fill =
      common::PatternFill::bitmap8([0; 8], 0, color(0, 0, 0, 255), color(255, 255, 255, 255));
    assert!(matches!(
      DirectPatternSet::validate(fill),
      Err(PdfError::Writer(message)) if message.contains("cell size")
    ));
  }
}
