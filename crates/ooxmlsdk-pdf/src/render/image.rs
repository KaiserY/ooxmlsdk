use std::hash::{Hash, Hasher};
use std::io::{Cursor, Write};
use std::sync::{Arc, OnceLock};

use flate2::{Compression, write::ZlibEncoder};
use image::codecs::png::PngEncoder;
use image::metadata::Orientation;
use image::{
  ColorType, DynamicImage, GenericImageView, ImageDecoder, ImageEncoder,
  ImageFormat as RasterImageFormat, ImageReader, Rgba, imageops::FilterType,
};
use jpeg_encoder::{ColorType as JpegColorType, Encoder as JpegEncoder, SamplingFactor};
use rustc_hash::FxHashMap as HashMap;

use crate::error::{PdfError, Result};
use crate::options::{PdfDocumentKind, PdfImageOptimizationPolicy, PdfOptimizeFor, PdfOptions};
use ooxmlsdk_layout::render::emf_wmf;

use super::native_png::NativeIndexedPng;

const WORD_STATIC_3D_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-static-3d+png";
const POWERPOINT_STATIC_3D_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.powerpoint-static-3d+png";
const POWERPOINT_MODEL3D_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.powerpoint-model3d+png";
const WORD_STATIC_3D_PHYSICAL_RGBA_CHUNK: [u8; 4] = *b"oxPr";
const WORD_STATIC_3D_EFFECT_RGBA_CHUNK: [u8; 4] = *b"oxEr";
const WORD_STATIC_3D_PHYSICAL_MAPPING_CHUNK: [u8; 4] = *b"oxMr";
const WORD_STATIC_3D_FINAL_GRID_RGBA_CHUNK: [u8; 4] = *b"oxSr";
const WORD_STATIC_3D_BACKDROP_GRID_RGBA_CHUNK: [u8; 4] = *b"oxBr";
const WORD_LOCKED_CANVAS_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-locked-canvas+png";
const WORD_LOCKED_CANVAS_DEVICE_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-locked-canvas-device+png";
const WORD_SHAPE_STORY_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-shape-story+png";
const WORD_GROUP_GLOW_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-group-glow+png";
const WORD_SHAPE_GLOW_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-shape-glow+png";
const WORD_SHAPE_SHADOW_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-shape-shadow+png";
const WORD_TABLE_BORDER_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.wordprocessing-table-border+png";
const SOURCE_RECTANGLE_CROP_BITMAP_CONTENT_TYPE: &str =
  "application/vnd.ooxmlsdk.source-rectangle-crop+png";
const OFFICE_SMALL_RASTER_UNCOMPRESSED_RGB_BYTES: u64 = 64 * 1024;

/// Backend-neutral raster selected by the image policy.
///
/// Decoding, resampling, JPEG selection, hidden-color cleanup, interpolation,
/// ICC ownership, and matte association happen once. Cloning and identity
/// lookup remain O(1) even for large images.
#[derive(Clone, Debug)]
pub(super) struct PreparedRasterImage(Arc<PreparedRasterImageInner>);

#[derive(Debug)]
struct PreparedRasterImageInner {
  direct: DirectRasterImage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DirectRasterColorSpace {
  Gray,
  Rgb,
  Cmyk,
}

impl DirectRasterColorSpace {
  pub(super) const fn components(self) -> i32 {
    match self {
      Self::Gray => 1,
      Self::Rgb => 3,
      Self::Cmyk => 4,
    }
  }

  fn accepts_icc(self, profile: &[u8]) -> bool {
    let expected = match self {
      Self::Gray => b"GRAY".as_slice(),
      Self::Rgb => b"RGB ".as_slice(),
      Self::Cmyk => b"CMYK".as_slice(),
    };
    let declared_length = profile
      .get(..4)
      .and_then(|bytes| <[u8; 4]>::try_from(bytes).ok())
      .map(u32::from_be_bytes)
      .and_then(|length| usize::try_from(length).ok());
    profile.get(16..20) == Some(expected)
      && profile.get(36..40) == Some(b"acsp")
      && declared_length.is_some_and(|length| length >= 132 && length == profile.len())
  }
}

#[derive(Clone, Debug)]
pub(super) enum DirectRasterEncoding {
  /// Unencoded component samples. The direct writer compresses each color and
  /// alpha plane once when it writes the shared image object.
  Sampled { pixels: Arc<PdfRasterPixels> },
  /// A complete baseline/progressive JPEG stream consumed by `/DCTDecode`.
  Dct {
    data: Arc<[u8]>,
    icc_profile: Option<Arc<[u8]>>,
    invert_cmyk: bool,
    /// Optional 8-bit soft-mask samples stored independently from the JPEG
    /// color stream. PDF permits a DCT parent image to reference an ordinary
    /// grayscale image XObject through `/SMask`.
    alpha: Option<Arc<[u8]>>,
  },
  /// A validated indexed PNG IDAT stream consumed through PDF's PNG predictor.
  IndexedPng {
    data: Arc<[u8]>,
    palette: Arc<[u8]>,
    icc_profile: Option<Arc<[u8]>>,
  },
}

#[derive(Clone, Debug)]
pub(super) struct DirectRasterImage {
  pub(super) width: u32,
  pub(super) height: u32,
  pub(super) color_space: DirectRasterColorSpace,
  pub(super) bits_per_component: u8,
  pub(super) encoding: DirectRasterEncoding,
  pub(super) interpolate: bool,
  /// Soft masks have their own image dictionary and interpolation policy.
  pub(super) soft_mask_interpolate: bool,
  /// Present only when the color plane is associated with this matte.
  /// PDF writes the value on the linked soft-mask image, not on the parent.
  pub(super) matte: Option<[f32; 3]>,
}

impl DirectRasterImage {
  pub(super) fn icc_profile(&self) -> Option<&[u8]> {
    let profile = match &self.encoding {
      DirectRasterEncoding::Sampled { pixels } => pixels.icc_profile.as_deref(),
      DirectRasterEncoding::Dct { icc_profile, .. }
      | DirectRasterEncoding::IndexedPng { icc_profile, .. } => icc_profile.as_deref(),
    }?;
    self.color_space.accepts_icc(profile).then_some(profile)
  }

  pub(super) fn alpha(&self) -> Option<&[u8]> {
    match &self.encoding {
      DirectRasterEncoding::Sampled { pixels } => pixels.alpha.as_deref(),
      DirectRasterEncoding::Dct { alpha, .. } => alpha.as_deref(),
      DirectRasterEncoding::IndexedPng { .. } => None,
    }
  }
}

impl PreparedRasterImage {
  pub(super) fn new(direct: DirectRasterImage) -> Self {
    Self(Arc::new(PreparedRasterImageInner { direct }))
  }

  pub(super) fn direct(&self) -> &DirectRasterImage {
    &self.0.direct
  }

  pub(super) fn identity(&self) -> usize {
    Arc::as_ptr(&self.0) as usize
  }

  #[cfg(test)]
  fn size(&self) -> (u32, u32) {
    (self.0.direct.width, self.0.direct.height)
  }
}

#[derive(Default)]
pub(super) struct ImageSet {
  rasters: HashMap<(usize, usize), Vec<CachedRaster>>,
  svgs: HashMap<(usize, usize), Arc<super::direct_svg::PreparedSvg>>,
  svg_rasters: HashMap<(usize, usize, u32, u32), PreparedRasterImage>,
  office_math_svgs: HashMap<(usize, usize), Arc<super::direct_svg::PreparedOfficeMathSvg>>,
}

struct CachedRaster {
  content_type: Option<String>,
  metafile_render_options: Option<emf_wmf::RenderOptions>,
  export_options: RasterExportOptions,
  image: PreparedRasterImage,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RasterExportOptions {
  use_lossless_compression: bool,
  jpeg_quality: Option<u8>,
  max_size_px: Option<RasterPixelLimits>,
  downsample_trigger_px: Option<RasterPixelLimits>,
  word_print_jpeg_downsample_trigger_px: Option<RasterPixelLimits>,
  exact_downsample_trigger: bool,
  use_gdiplus_jpeg_resampler: bool,
  allow_interpolation: bool,
  profile: RasterExportProfile,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RasterExportProfile {
  Requested,
  MicrosoftOfficeFixedOutput {
    document_kind: PdfDocumentKind,
    optimize_for: PdfOptimizeFor,
  },
}

impl RasterExportProfile {
  fn from_options(options: &PdfOptions) -> Self {
    match options.images.optimization_policy {
      PdfImageOptimizationPolicy::Requested => Self::Requested,
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(document_kind) => {
        Self::MicrosoftOfficeFixedOutput {
          document_kind,
          optimize_for: options.optimize_for,
        }
      }
    }
  }

  fn is_office_fixed_output(self) -> bool {
    matches!(self, Self::MicrosoftOfficeFixedOutput { .. })
  }

  fn is_office_screen(self) -> bool {
    matches!(
      self,
      Self::MicrosoftOfficeFixedOutput {
        optimize_for: PdfOptimizeFor::Screen,
        ..
      }
    )
  }

  fn is_word_print(self) -> bool {
    matches!(
      self,
      Self::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Docx,
        optimize_for: PdfOptimizeFor::Print,
      }
    )
  }

  fn is_word(self) -> bool {
    matches!(
      self,
      Self::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Docx,
        ..
      }
    )
  }

  fn is_power_point_screen(self) -> bool {
    matches!(
      self,
      Self::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Pptx,
        optimize_for: PdfOptimizeFor::Screen,
      }
    )
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RasterOwner {
  Source,
  MaterializedSourceRectangleCrop,
  WordGroupGlow,
  WordShapeGlow,
  WordShapeShadow,
  WordLockedCanvas,
  WordLockedCanvasDevice,
  WordShapeStory,
  WordStatic3d,
  PowerPointStatic3d,
  WordTableBorder,
  MetafilePreview,
}

impl RasterOwner {
  fn from_content_type(content_type: Option<&str>) -> Self {
    if content_type
      .is_some_and(|value| value.eq_ignore_ascii_case(SOURCE_RECTANGLE_CROP_BITMAP_CONTENT_TYPE))
    {
      Self::MaterializedSourceRectangleCrop
    } else if content_type
      .is_some_and(|value| value.eq_ignore_ascii_case(WORD_GROUP_GLOW_BITMAP_CONTENT_TYPE))
    {
      Self::WordGroupGlow
    } else if content_type
      .is_some_and(|value| value.eq_ignore_ascii_case(WORD_SHAPE_GLOW_BITMAP_CONTENT_TYPE))
    {
      Self::WordShapeGlow
    } else if content_type
      .is_some_and(|value| value.eq_ignore_ascii_case(WORD_SHAPE_SHADOW_BITMAP_CONTENT_TYPE))
    {
      Self::WordShapeShadow
    } else if content_type
      .is_some_and(|value| value.eq_ignore_ascii_case(WORD_LOCKED_CANVAS_BITMAP_CONTENT_TYPE))
    {
      Self::WordLockedCanvas
    } else if content_type.is_some_and(|value| {
      value.eq_ignore_ascii_case(WORD_LOCKED_CANVAS_DEVICE_BITMAP_CONTENT_TYPE)
    }) {
      Self::WordLockedCanvasDevice
    } else if content_type
      .is_some_and(|value| value.eq_ignore_ascii_case(WORD_SHAPE_STORY_BITMAP_CONTENT_TYPE))
    {
      Self::WordShapeStory
    } else if content_type
      .is_some_and(|value| value.eq_ignore_ascii_case(WORD_STATIC_3D_BITMAP_CONTENT_TYPE))
    {
      Self::WordStatic3d
    } else if content_type.is_some_and(|value| {
      value.eq_ignore_ascii_case(POWERPOINT_STATIC_3D_BITMAP_CONTENT_TYPE)
        || value.eq_ignore_ascii_case(POWERPOINT_MODEL3D_BITMAP_CONTENT_TYPE)
    }) {
      // Both layout-owned 3-D surfaces retain their final grid and use the
      // same independent associated RGB/A8 encoding. Their geometry and
      // cached-model realization remain separate layout implementations.
      Self::PowerPointStatic3d
    } else if content_type
      .is_some_and(|value| value.eq_ignore_ascii_case(WORD_TABLE_BORDER_BITMAP_CONTENT_TYPE))
    {
      Self::WordTableBorder
    } else {
      Self::Source
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RasterPixelLimits {
  width_bits: u64,
  height_bits: u64,
}

impl RasterPixelLimits {
  fn from_display_size(width_pt: f32, height_pt: f32, dpi: u32) -> Option<Self> {
    if !width_pt.is_finite() || width_pt <= 0.0 || !height_pt.is_finite() || height_pt <= 0.0 {
      return None;
    }
    Self::from_pixels(
      f64::from(width_pt) * f64::from(dpi) / 72.0,
      f64::from(height_pt) * f64::from(dpi) / 72.0,
    )
  }

  fn from_office_fixed_output_display_size(
    width_pt: f32,
    height_pt: f32,
    dpi: u32,
  ) -> Option<Self> {
    if !width_pt.is_finite() || width_pt <= 0.0 || !height_pt.is_finite() || height_pt <= 0.0 {
      return None;
    }
    // Office lays out DrawingML extents on its integer-twip grid before
    // allocating a fixed-output bitmap. The GDI destination rectangle then
    // counts inclusive device endpoints: an exact N-pixel logical extent
    // contains N-1 samples, while any positive fraction reaches sample N.
    // Controlled Word and PowerPoint Print/Screen ladders expose the same
    // `ceil(twips * dpi / 1440) - 1` rule.
    let inclusive_device_extent = |points: f32| {
      let twips = (f64::from(points) * 20.0).floor();
      ((twips * f64::from(dpi) / 1440.0).ceil() - 1.0).max(1.0)
    };
    Self::from_pixels(
      inclusive_device_extent(width_pt),
      inclusive_device_extent(height_pt),
    )
  }

  fn from_word_static_3d_fixed_output_display_size(
    width_pt: f32,
    height_pt: f32,
    dpi: u32,
  ) -> Option<Self> {
    // Compatibility sizing for surfaces without a layout-owned final grid.
    // A PDF display extent is not the natural raster allocation rectangle:
    // retained Office PDF/EMF controls include counterexamples to this rule.
    // When layout supplies a final grid, its dimensions take precedence at
    // export; do not discard that grid using this display-derived estimate.
    let (width, height) = ooxmlsdk_layout::units::word_static_3d_fixed_output_raster_dimensions(
      width_pt, height_pt, dpi,
    )?;
    Self::from_pixels(f64::from(width), f64::from(height))
  }

  fn from_pixels(width: f64, height: f64) -> Option<Self> {
    (width.is_finite() && width > 0.0 && height.is_finite() && height > 0.0).then_some(Self {
      width_bits: width.to_bits(),
      height_bits: height.to_bits(),
    })
  }

  fn pixels(self) -> (f64, f64) {
    (
      f64::from_bits(self.width_bits),
      f64::from_bits(self.height_bits),
    )
  }
}

pub(super) fn office_fixed_output_raster_dimensions(
  width_pt: f32,
  height_pt: f32,
  dpi: u32,
) -> Option<(u32, u32)> {
  let (width, height) =
    RasterPixelLimits::from_office_fixed_output_display_size(width_pt, height_pt, dpi)?.pixels();
  let dimension = |value: f64| {
    (value.is_finite() && value >= 1.0 && value <= f64::from(u32::MAX)).then_some(value as u32)
  };
  Some((dimension(width)?, dimension(height)?))
}

impl RasterExportOptions {
  fn new(options: &PdfOptions, display_width_pt: f32, display_height_pt: f32) -> Self {
    let profile = RasterExportProfile::from_options(options);
    let office_fixed_output = profile.is_office_fixed_output();
    let archival = options
      .standards
      .iter()
      .any(|standard| standard.is_archival());
    let configured_max_dpi = (!office_fixed_output && options.images.reduce_resolution)
      .then_some(options.images.max_resolution_dpi)
      .flatten()
      .filter(|dpi| *dpi > 50);
    // Word's WdExportOptimizeFor, Excel's XlFixedFormatQuality, and
    // PowerPoint's fixed-format Intent expose one 96-DPI Screen / 200-DPI
    // Print host profile. Controlled exports show that the same selector owns
    // JPEG quantization (the Screen table is byte-identical to IJG quality
    // 59; Print uses quality 75). Word's ordinary JPEG path has independently
    // measured 150-DPI Screen and 275-DPI Print reduction boundaries; other
    // raster owners retain their separately established trigger policy.
    // PowerPoint Print is the one fixed-output exception: setting UseISO19005_1 changes
    // its photographic encoder from quality 75 to quality 90. An exact
    // PDF/A-off/on Office task pair emits identical 250x250 h2v2 images at
    // those two quantization levels; Screen retains the quality-59 DQT in
    // both modes.
    // Requested public options remain independent: merely selecting Screen must not
    // invent an implicit resolution ceiling.
    let optimize_for_max_dpi = if office_fixed_output {
      Some(match options.optimize_for {
        PdfOptimizeFor::Screen => 96,
        PdfOptimizeFor::Print => 200,
      })
    } else {
      None
    };
    let target_dpi = configured_max_dpi
      .into_iter()
      .chain(optimize_for_max_dpi)
      .min();
    let max_size_px = target_dpi.and_then(|dpi| {
      if office_fixed_output {
        RasterPixelLimits::from_office_fixed_output_display_size(
          display_width_pt,
          display_height_pt,
          dpi,
        )
      } else {
        RasterPixelLimits::from_display_size(display_width_pt, display_height_pt, dpi)
      }
    });
    let downsample_trigger_px = office_fixed_output
      .then_some(match options.optimize_for {
        PdfOptimizeFor::Screen => 150,
        PdfOptimizeFor::Print => 300,
      })
      .and_then(|dpi| {
        RasterPixelLimits::from_display_size(display_width_pt, display_height_pt, dpi)
      });
    Self {
      use_lossless_compression: if office_fixed_output {
        false
      } else {
        options.images.use_lossless_compression
      },
      jpeg_quality: if office_fixed_output {
        Some(match profile {
          RasterExportProfile::MicrosoftOfficeFixedOutput {
            document_kind: PdfDocumentKind::Pptx,
            optimize_for: PdfOptimizeFor::Print,
          } if archival => 90,
          RasterExportProfile::MicrosoftOfficeFixedOutput {
            optimize_for: PdfOptimizeFor::Screen,
            ..
          } => 59,
          RasterExportProfile::MicrosoftOfficeFixedOutput {
            optimize_for: PdfOptimizeFor::Print,
            ..
          } => 75,
          RasterExportProfile::Requested => unreachable!("Office profile was established above"),
        })
      } else {
        options.effective_jpeg_quality()
      },
      max_size_px,
      downsample_trigger_px,
      word_print_jpeg_downsample_trigger_px: profile
        .is_word_print()
        .then(|| RasterPixelLimits::from_display_size(display_width_pt, display_height_pt, 275))
        .flatten(),
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      // ISO 19005 forbids the image Interpolate key with a true value.
      // Preserve Office's ordinary-PDF smoothing policy, but force the
      // explicitly false archival form for every PDF/A profile.
      allow_interpolation: !archival,
      profile,
    }
  }

  fn for_raster_format(mut self, format: RasterImageFormat) -> Self {
    if self.profile.is_word() && format == RasterImageFormat::Jpeg {
      // Exact Word ladders place the first ordinary-JPEG reduction at the
      // selected density itself: 150 DPI for Screen and 275 DPI for Print.
      // The four-pixel tolerance belongs to other raster-owner policies and
      // must not pull the JPEG boundary below those measured values.
      self.exact_downsample_trigger = true;
      // Word Print and Screen share the GDI+ bitmap realization. The Screen
      // 419/420/421 and independent-axis controls reproduce the same Q16
      // area switch; quality and target density remain profile-specific.
      self.use_gdiplus_jpeg_resampler = true;
      if self.profile.is_word_print() {
        self.downsample_trigger_px = self.word_print_jpeg_downsample_trigger_px;
      }
    }
    // PowerPoint's JPEG/PNG 419/420/421 controls on both source axes expose
    // the same bitmap area kernel. Its density/trigger policy is independent
    // of Word's: selecting a sampler must not change the allocation boundary.
    if self.profile.is_power_point_screen() && format == RasterImageFormat::Jpeg {
      self.use_gdiplus_jpeg_resampler = true;
    }
    self
  }

  fn for_owner(
    mut self,
    owner: RasterOwner,
    display_width_pt: f32,
    display_height_pt: f32,
  ) -> Self {
    if owner != RasterOwner::WordStatic3d {
      return self;
    }
    let dpi = match self.profile {
      RasterExportProfile::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Docx,
        optimize_for: PdfOptimizeFor::Screen,
      } => 96,
      RasterExportProfile::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Docx,
        optimize_for: PdfOptimizeFor::Print,
      } => 200,
      RasterExportProfile::Requested | RasterExportProfile::MicrosoftOfficeFixedOutput { .. } => {
        return self;
      }
    };
    self.max_size_px = RasterPixelLimits::from_word_static_3d_fixed_output_display_size(
      display_width_pt,
      display_height_pt,
      dpi,
    );
    self
  }

  fn without_downsampling(mut self) -> Self {
    self.max_size_px = None;
    self.downsample_trigger_px = None;
    self.word_print_jpeg_downsample_trigger_px = None;
    self.use_gdiplus_jpeg_resampler = false;
    self
  }

  fn downsample_size(self, size: (u32, u32)) -> Option<(u32, u32)> {
    let max_size = self.max_size_px?;
    if let Some(trigger) = self.downsample_trigger_px {
      let (trigger_width, trigger_height) = trigger.pixels();
      let reaches_trigger = if self.exact_downsample_trigger {
        f64::from(size.0) >= trigger_width || f64::from(size.1) >= trigger_height
      } else {
        f64::from(size.0) + 4.0 >= trigger_width || f64::from(size.1) + 4.0 >= trigger_height
      };
      if !reaches_trigger {
        return None;
      }
    }
    if self.profile.is_office_fixed_output() {
      downsample_office_fixed_output(size, max_size)
    } else {
      downsample_size(size, max_size)
    }
  }

  fn downsample_source_size(
    self,
    size: (u32, u32),
    format: RasterImageFormat,
    owner: RasterOwner,
  ) -> Option<(u32, u32)> {
    if owner != RasterOwner::Source
      || !self.profile.is_power_point_screen()
      || !matches!(format, RasterImageFormat::Png | RasterImageFormat::Jpeg)
    {
      return self.downsample_size(size);
    }
    let (target_width, target_height) = self.max_size_px?.pixels();
    let Some(trigger) = self.downsample_trigger_px else {
      return self.downsample_size(size);
    };
    let (trigger_width, trigger_height) = trigger.pixels();
    if size.0 <= 50 || size.1 <= 50 {
      return None;
    }
    // PowerPoint Screen tests each source axis against 150 DPI independently.
    // Palette+alpha PNG, RGB PNG and JPEG controls retain 149.987 DPI, but
    // reduce at 150 DPI. A four-pixel trigger allowance reduces too early;
    // an OR test followed by resizing both axes also destroys untouched samples.
    let axis = |source: u32, threshold: f64, target: f64| {
      if f64::from(source) >= threshold {
        (target.round() as u32).clamp(1, source)
      } else {
        source
      }
    };
    let target = (
      axis(size.0, trigger_width, target_width),
      axis(size.1, trigger_height, target_height),
    );
    (target != size).then_some(target)
  }
}

impl ImageSet {
  pub(super) fn svg(&mut self, data: &[u8]) -> Result<Arc<super::direct_svg::PreparedSvg>> {
    let key = image_data_key(data);
    if let Some(scene) = self.svgs.get(&key) {
      return Ok(scene.clone());
    }
    let scene = Arc::new(super::direct_svg::prepare_svg(data, svg_options())?);
    self.svgs.insert(key, scene.clone());
    Ok(scene)
  }

  pub(super) fn rasterized_svg(
    &mut self,
    data: &[u8],
    scene: &super::direct_svg::PreparedSvg,
    options: &PdfOptions,
    display_width_pt: f32,
    display_height_pt: f32,
  ) -> Result<PreparedRasterImage> {
    let (width, height) = svg_raster_dimensions(options, display_width_pt, display_height_pt)?;
    let interpolate =
      RasterExportOptions::new(options, display_width_pt, display_height_pt).allow_interpolation;
    let data_key = image_data_key(data);
    let key = (data_key.0, data_key.1, width, height);
    if let Some(image) = self.svg_rasters.get(&key) {
      return Ok(image.clone());
    }

    let tree = scene.tree();
    let tree_size = tree.size();
    if !tree_size.width().is_finite()
      || !tree_size.height().is_finite()
      || tree_size.width() <= 0.0
      || tree_size.height() <= 0.0
    {
      return Err(PdfError::Image(
        "SVG image has an invalid intrinsic size".to_string(),
      ));
    }
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height).ok_or_else(|| {
      PdfError::Image(format!(
        "could not allocate {width}x{height} SVG raster fallback"
      ))
    })?;
    resvg::render(
      tree,
      resvg::tiny_skia::Transform::from_scale(
        width as f32 / tree_size.width(),
        height as f32 / tree_size.height(),
      ),
      &mut pixmap.as_mut(),
    );

    // tiny-skia stores premultiplied sRGB RGBA. PDF's sampled image and SMask
    // require independent straight RGB and alpha planes.
    let mut rgb = Vec::with_capacity(width as usize * height as usize * 3);
    let mut alpha = Vec::with_capacity(width as usize * height as usize);
    let mut has_transparency = false;
    for pixel in pixmap.data().as_chunks::<4>().0.iter() {
      let a = u16::from(pixel[3]);
      has_transparency |= a != 255;
      for channel in &pixel[..3] {
        let unassociated = (u16::from(*channel) * 255 + a / 2)
          .checked_div(a)
          .unwrap_or_default();
        rgb.push(unassociated.min(255) as u8);
      }
      alpha.push(pixel[3]);
    }
    let image = PreparedRasterImage::new(DirectRasterImage {
      width,
      height,
      color_space: DirectRasterColorSpace::Rgb,
      bits_per_component: 8,
      encoding: DirectRasterEncoding::Sampled {
        pixels: Arc::new(PdfRasterPixels {
          width,
          height,
          rgb,
          alpha: has_transparency.then_some(alpha),
          icc_profile: None,
        }),
      },
      interpolate,
      soft_mask_interpolate: interpolate,
      matte: None,
    });
    self.svg_rasters.insert(key, image.clone());
    Ok(image)
  }

  pub(super) fn office_math_svg(
    &mut self,
    data: &[u8],
  ) -> Result<Arc<super::direct_svg::PreparedOfficeMathSvg>> {
    let key = image_data_key(data);
    if let Some(scene) = self.office_math_svgs.get(&key) {
      return Ok(scene.clone());
    }
    let scene = Arc::new(super::direct_svg::prepare_office_math_svg(
      data,
      svg_options(),
    )?);
    self.office_math_svgs.insert(key, scene.clone());
    Ok(scene)
  }

  pub(super) fn raster_direct(
    &mut self,
    data: &[u8],
    content_type: Option<&str>,
    options: &PdfOptions,
    metafile_render_options: Option<emf_wmf::RenderOptions>,
    display_width_pt: f32,
    display_height_pt: f32,
  ) -> Result<PreparedRasterImage> {
    self.prepared_raster(
      data,
      content_type,
      options,
      metafile_render_options,
      display_width_pt,
      display_height_pt,
    )
  }

  fn prepared_raster(
    &mut self,
    data: &[u8],
    content_type: Option<&str>,
    options: &PdfOptions,
    metafile_render_options: Option<emf_wmf::RenderOptions>,
    display_width_pt: f32,
    display_height_pt: f32,
  ) -> Result<PreparedRasterImage> {
    let owner = RasterOwner::from_content_type(content_type);
    let export_options = RasterExportOptions::new(options, display_width_pt, display_height_pt)
      .for_owner(owner, display_width_pt, display_height_pt);
    let key = image_data_key(data);
    if let Some(image) = self.rasters.get(&key).and_then(|images| {
      images.iter().find(|image| {
        image.content_type.as_deref() == content_type
          && image.metafile_render_options == metafile_render_options
          && image.export_options == export_options
      })
    }) {
      return Ok(image.image.clone());
    }
    let image = decode_image(data, content_type, export_options, metafile_render_options)?;
    self.rasters.entry(key).or_default().push(CachedRaster {
      content_type: content_type.map(str::to_string),
      metafile_render_options,
      export_options,
      image: image.clone(),
    });
    Ok(image)
  }
}

fn svg_raster_dimensions(
  options: &PdfOptions,
  display_width_pt: f32,
  display_height_pt: f32,
) -> Result<(u32, u32)> {
  const MAX_SVG_RASTER_DIMENSION: f64 = 16_384.0;
  const MAX_SVG_RASTER_PIXELS: f64 = 16_777_216.0;

  if !display_width_pt.is_finite()
    || !display_height_pt.is_finite()
    || display_width_pt <= 0.0
    || display_height_pt <= 0.0
  {
    return Err(PdfError::Image(
      "SVG image has invalid display dimensions".to_string(),
    ));
  }
  let dpi = match options.optimize_for {
    PdfOptimizeFor::Print => f64::from(ooxmlsdk_layout::units::OFFICE_FIXED_OUTPUT_RASTER_DPI),
    PdfOptimizeFor::Screen => f64::from(ooxmlsdk_layout::units::CSS_PIXELS_PER_INCH),
  };
  let mut width = (f64::from(display_width_pt) * dpi
    / f64::from(ooxmlsdk_layout::units::POINTS_PER_INCH))
  .ceil()
  .clamp(1.0, MAX_SVG_RASTER_DIMENSION);
  let mut height = (f64::from(display_height_pt) * dpi
    / f64::from(ooxmlsdk_layout::units::POINTS_PER_INCH))
  .ceil()
  .clamp(1.0, MAX_SVG_RASTER_DIMENSION);
  let pixels = width * height;
  if pixels > MAX_SVG_RASTER_PIXELS {
    let scale = (MAX_SVG_RASTER_PIXELS / pixels).sqrt();
    width = (width * scale).floor().max(1.0);
    height = (height * scale).floor().max(1.0);
  }
  Ok((width as u32, height as u32))
}

fn svg_options() -> &'static usvg::Options<'static> {
  static OPTIONS: OnceLock<usvg::Options<'static>> = OnceLock::new();
  OPTIONS.get_or_init(|| {
    let mut options = usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    options
  })
}

fn image_data_key(data: &[u8]) -> (usize, usize) {
  // ImageItem owns its Arc-backed bytes for the whole render. Using that stable
  // allocation identity avoids hashing large images on every repeated draw.
  (data.as_ptr() as usize, data.len())
}

fn decode_image(
  data: &[u8],
  content_type: Option<&str>,
  export_options: RasterExportOptions,
  metafile_render_options: Option<emf_wmf::RenderOptions>,
) -> Result<PreparedRasterImage> {
  let owner = RasterOwner::from_content_type(content_type);
  if owner == RasterOwner::PowerPointStatic3d {
    // Layout has already realized the configured scene grid. Transparent
    // working padding is not source-picture content and must not drive a
    // second resample or the color classifier.
    return export_decoded_image(
      decode_dynamic_image(data, RasterImageFormat::Png)?,
      RasterImageFormat::Png,
      export_options.without_downsampling(),
      owner,
    );
  }
  if owner == RasterOwner::MaterializedSourceRectangleCrop {
    // `a:srcRect` has already been rounded against and materialized from the
    // source pixels by the DOCX importer. Word embeds that visible source
    // rectangle at its native sample count even under Screen optimization;
    // applying the displayed-frame DPI cap here would downsample it twice.
    // Compression remains active and may still choose a JPEG color stream.
    return export_decoded_image(
      decode_dynamic_image(data, RasterImageFormat::Png)?,
      RasterImageFormat::Png,
      export_options.without_downsampling(),
      owner,
    );
  }

  if owner == RasterOwner::WordShapeStory {
    return export_wordprocessing_shape_story_image(
      decode_dynamic_image(data, RasterImageFormat::Png)?,
      export_options,
    );
  }

  if matches!(
    owner,
    RasterOwner::WordGroupGlow | RasterOwner::WordShapeGlow | RasterOwner::WordShapeShadow
  ) {
    return export_wordprocessing_black_matte_effect_image(decode_dynamic_image(
      data,
      RasterImageFormat::Png,
    )?);
  }

  if owner == RasterOwner::WordLockedCanvas {
    return export_wordprocessing_locked_canvas_image(
      decode_dynamic_image(data, RasterImageFormat::Png)?,
      export_options,
    );
  }
  if owner == RasterOwner::WordLockedCanvasDevice {
    return export_wordprocessing_locked_canvas_device_image(
      decode_dynamic_image(data, RasterImageFormat::Png)?,
      export_options,
    );
  }

  if owner == RasterOwner::WordStatic3d {
    let layers = decode_wordprocessing_static_3d_layers(data);
    return export_wordprocessing_static_3d_image(
      decode_dynamic_image(data, RasterImageFormat::Png)?,
      export_options,
      layers,
    );
  }

  if owner == RasterOwner::WordTableBorder {
    let raster = decode_dynamic_image(data, RasterImageFormat::Png)?;
    return prepare_sampled_image(
      PdfRasterImage::from_dynamic_preserving_hidden_rgb(raster.image, raster.icc_profile),
      false,
      None,
    );
  }

  let metafile_raster = match metafile_render_options {
    Some(render_options) => {
      emf_wmf::decode_metafile_as_raster_with_options(data, content_type, render_options)
    }
    None => emf_wmf::decode_metafile_as_raster(data, content_type),
  };
  if let Some(raster) = metafile_raster
    .map_err(|err| PdfError::Image(format!("failed to decode EMF/WMF image: {err}")))?
  {
    return match raster.content_type {
      "image/jpeg"
        if export_options.use_lossless_compression || export_options.jpeg_quality.is_some() =>
      {
        export_decoded_image(
          decode_dynamic_image(&raster.data, RasterImageFormat::Jpeg)?,
          RasterImageFormat::Jpeg,
          export_options.for_raster_format(RasterImageFormat::Jpeg),
          RasterOwner::MetafilePreview,
        )
      }
      "image/jpeg" => prepare_native_jpeg_image(raster.data, export_options.allow_interpolation),
      "image/png" => {
        let image = decode_png_relaxed(&raster.data)
          .map_err(|err| PdfError::Image(format!("failed to decode EMF/WMF PNG: {err}")))?;
        // Office fixed output keeps generated metafile previews lossless and
        // marks their image XObjects `/Interpolate false`. Do not apply the
        // ordinary source-bitmap downsampling/JPEG policy to a completed GDI
        // replay; the PDF image matrix consumes that device surface directly
        // and preserves its reconstructed soft mask. The decoder returns
        // straight-alpha PNG samples, whereas Office associates the generated
        // color plane with black and records /Matte on the mask. Carry both
        // parts of that contract, including for binary coverage: a PDF reader
        // can resample the color and mask before compositing them.
        prepare_metafile_preview(image, export_options.profile.is_office_fixed_output())
      }
      content_type => Err(PdfError::Image(format!(
        "unsupported EMF/WMF raster content type: {content_type}"
      ))),
    };
  }

  let format = raster_format(data, content_type);

  if let Some(format) = format {
    let format_export_options = export_options.for_raster_format(format);
    let metadata = raster_metadata(data, format)?;
    let plan = RasterPlan::new(format, metadata, owner, format_export_options);
    match plan.realization {
      RasterRealization::Decoded => {
        let raster = if format == RasterImageFormat::Jpeg
          && (format_export_options.use_gdiplus_jpeg_resampler
            || (format_export_options.profile.is_word()
              && direct_jpeg_metadata(data)
                .is_ok_and(|metadata| metadata.color_space == DirectRasterColorSpace::Gray)))
        {
          decode_gdiplus_jpeg(data)?
        } else {
          decode_dynamic_image(data, format)?
        };
        return export_decoded_image(raster, format, format_export_options, owner);
      }
      RasterRealization::NativeJpeg => {
        if let Ok(image) =
          prepare_native_jpeg_image(data.to_vec(), format_export_options.allow_interpolation)
        {
          // Preserve the native ICC profile and compressed JPEG stream.
          return Ok(image);
        }
      }
      RasterRealization::NativePng => {
        let indexed = format_export_options
          .profile
          .is_office_fixed_output()
          .then(|| NativeIndexedPng::parse(data))
          .flatten();
        if let Ok(image) = prepare_native_png_image(data.to_vec(), false, indexed) {
          // Preparation validates the PNG once and carries the original IDAT,
          // palette and component width directly to the image writer.
          return Ok(image);
        }
      }
    }
  }
  if matches!(format, Some(RasterImageFormat::Png))
    && let Ok(image) = decode_png_relaxed(data)
  {
    return prepare_sampled_image(image, false, None);
  }

  let format = format.ok_or_else(|| PdfError::Image("unknown raster image format".to_string()))?;
  let raster = decode_dynamic_image(data, format)?;
  export_decoded_image(
    raster,
    format,
    export_options.for_raster_format(format),
    RasterOwner::Source,
  )
}

fn raster_interpolation(format: RasterImageFormat, export_options: RasterExportOptions) -> bool {
  // Word's fixed-format output marks photographic JPEG XObjects for smooth
  // interpolation while leaving lossless pixel graphics such as PNG
  // placeholders un-interpolated. Make that choice explicit instead of
  // inheriting one blanket backend default for every raster format.
  format == RasterImageFormat::Jpeg && export_options.allow_interpolation
}

fn prepare_metafile_preview(
  mut image: PdfRasterImage,
  office_fixed_output: bool,
) -> Result<PreparedRasterImage> {
  let associate_with_black = office_fixed_output && image.pixels.alpha.is_some();
  if associate_with_black {
    let mut pixels = Arc::unwrap_or_clone(image.pixels);
    if let Some(alpha) = &pixels.alpha {
      for (rgb, alpha) in pixels.rgb.as_chunks_mut::<3>().0.iter_mut().zip(alpha) {
        for channel in rgb {
          *channel = ((u16::from(*channel) * u16::from(*alpha) + 127) / 255) as u8;
        }
      }
    }
    image.pixels = Arc::new(pixels);
  }
  prepare_sampled_image(image, false, associate_with_black.then_some([0.0; 3]))
}

fn raster_compression_change_requested(
  format: RasterImageFormat,
  export_options: RasterExportOptions,
  owner: RasterOwner,
) -> bool {
  (format == RasterImageFormat::Jpeg && export_options.use_lossless_compression)
    || should_try_jpeg(format, export_options, owner)
}

fn should_try_jpeg(
  format: RasterImageFormat,
  export_options: RasterExportOptions,
  owner: RasterOwner,
) -> bool {
  let has_jpeg_profile =
    !export_options.use_lossless_compression && export_options.jpeg_quality.is_some();
  if !has_jpeg_profile {
    return false;
  }

  match export_options.profile {
    RasterExportProfile::Requested => true,
    RasterExportProfile::MicrosoftOfficeFixedOutput { .. } => match owner {
      // Office owns the decoded representation for ordinary sources which
      // were not selected by the native-image plan. Controlled opaque,
      // transparent, physical-resolution, and no-resolution PNG matrices all
      // use the same content-sensitive JPEG-versus-Flate comparison.
      RasterOwner::Source
      | RasterOwner::MaterializedSourceRectangleCrop
      | RasterOwner::PowerPointStatic3d => true,
      // Generated metafile previews remain lossless; only an existing JPEG
      // preview participates in the host's JPEG recompression profile.
      RasterOwner::MetafilePreview => format == RasterImageFormat::Jpeg,
      RasterOwner::WordGroupGlow
      | RasterOwner::WordShapeGlow
      | RasterOwner::WordShapeShadow
      | RasterOwner::WordLockedCanvas
      | RasterOwner::WordLockedCanvasDevice
      | RasterOwner::WordShapeStory
      | RasterOwner::WordStatic3d
      | RasterOwner::WordTableBorder => false,
    },
  }
}

#[derive(Clone, Copy, Debug)]
struct RasterMetadata {
  size: (u32, u32),
  orientation: Orientation,
  png: Option<PngMetadata>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PngMetadata {
  color_type: png::ColorType,
  bit_depth: png::BitDepth,
  has_transparency: bool,
  has_real_physical_resolution: bool,
}

impl PngMetadata {
  fn is_proven_office_native_indexed(self) -> bool {
    self.color_type == png::ColorType::Indexed
      && self.bit_depth == png::BitDepth::One
      && !self.has_transparency
      && self.has_real_physical_resolution
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct JpegMetadata {
  density_unit: u8,
  density_x: u16,
  density_y: u16,
}

impl JpegMetadata {
  fn has_real_physical_resolution(self) -> bool {
    matches!(self.density_unit, 1 | 2) && self.density_x != 0 && self.density_y != 0
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RasterRealization {
  NativeJpeg,
  NativePng,
  Decoded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RasterPlan {
  realization: RasterRealization,
}

impl RasterPlan {
  fn new(
    format: RasterImageFormat,
    metadata: Option<RasterMetadata>,
    owner: RasterOwner,
    export_options: RasterExportOptions,
  ) -> Self {
    let needs_orientation =
      metadata.is_some_and(|value| value.orientation != Orientation::NoTransforms);
    let needs_downsampling = metadata.is_some_and(|value| {
      export_options
        .downsample_source_size(value.size, format, owner)
        .is_some()
    });

    // GDI+/WIC exposes a metric pHYs chunk as a real-DPI image flag. Office
    // preserves opaque one-bit indexed sources carrying that flag at their
    // native sample count even above the ordinary 150/300-DPI reduction
    // trigger. Unit=0, absent pHYs, and every transparent control enter the
    // decoded owner instead.
    let office_native_png = owner == RasterOwner::Source
      && export_options.profile.is_office_fixed_output()
      && metadata
        .and_then(|value| value.png)
        .is_some_and(PngMetadata::is_proven_office_native_indexed);
    let needs_compression_change =
      raster_compression_change_requested(format, export_options, owner);
    let transparent_png = metadata
      .and_then(|value| value.png)
      .is_some_and(|value| value.has_transparency);

    let realization = if !needs_orientation && office_native_png {
      RasterRealization::NativePng
    } else if needs_orientation || needs_downsampling || needs_compression_change {
      RasterRealization::Decoded
    } else if format == RasterImageFormat::Jpeg {
      RasterRealization::NativeJpeg
    } else if format == RasterImageFormat::Png && !transparent_png {
      RasterRealization::NativePng
    } else {
      // Decode transparent PNGs even when no other transform is active.
      // Office and Cairo normalize RGB beneath alpha-zero to black before
      // emitting the separate SMask; raw IDAT embedding would preserve hidden
      // colors and expose viewer-dependent resampling halos.
      RasterRealization::Decoded
    };
    Self { realization }
  }
}

fn raster_metadata(data: &[u8], format: RasterImageFormat) -> Result<Option<RasterMetadata>> {
  let mut decoder = match ImageReader::with_format(Cursor::new(data), format).into_decoder() {
    Ok(decoder) => decoder,
    Err(_) => return Ok(None),
  };
  let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);
  let mut size = decoder.dimensions();
  if matches!(
    orientation,
    Orientation::Rotate90
      | Orientation::Rotate270
      | Orientation::Rotate90FlipH
      | Orientation::Rotate270FlipH
  ) {
    size = (size.1, size.0);
  }
  let png = (format == RasterImageFormat::Png)
    .then(|| png_metadata(data))
    .flatten();
  Ok(Some(RasterMetadata {
    size,
    orientation,
    png,
  }))
}

fn png_metadata(data: &[u8]) -> Option<PngMetadata> {
  let reader = png::Decoder::new(Cursor::new(data)).read_info().ok()?;
  let info = reader.info();
  let has_transparency = matches!(
    info.color_type,
    png::ColorType::GrayscaleAlpha | png::ColorType::Rgba
  ) || info.trns.is_some();
  let has_real_physical_resolution = info.pixel_dims.is_some_and(|dimensions| {
    dimensions.unit == png::Unit::Meter && dimensions.xppu > 0 && dimensions.yppu > 0
  });
  Some(PngMetadata {
    color_type: info.color_type,
    bit_depth: info.bit_depth,
    has_transparency,
    has_real_physical_resolution,
  })
}

fn jpeg_metadata(data: &[u8]) -> Option<JpegMetadata> {
  if !data.starts_with(&[0xff, 0xd8]) {
    return None;
  }

  let mut offset = 2;
  while offset < data.len() {
    while data.get(offset) == Some(&0xff) {
      offset += 1;
    }
    let marker = *data.get(offset)?;
    offset += 1;
    if marker == 0xda || marker == 0xd9 {
      break;
    }
    if marker == 0x01 || (0xd0..=0xd8).contains(&marker) {
      continue;
    }

    let length = usize::from(u16::from_be_bytes([
      *data.get(offset)?,
      *data.get(offset + 1)?,
    ]));
    if length < 2 {
      return None;
    }
    let end = offset.checked_add(length)?;
    let payload = data.get(offset + 2..end)?;
    if marker == 0xe0 && payload.starts_with(b"JFIF\0") && payload.len() >= 12 {
      return Some(JpegMetadata {
        density_unit: payload[7],
        density_x: u16::from_be_bytes([payload[8], payload[9]]),
        density_y: u16::from_be_bytes([payload[10], payload[11]]),
      });
    }
    offset = end;
  }
  None
}

struct DecodedRasterImage {
  image: DynamicImage,
  icc_profile: Option<Vec<u8>>,
  jpeg_has_real_physical_resolution: bool,
}

fn decode_dynamic_image(data: &[u8], format: RasterImageFormat) -> Result<DecodedRasterImage> {
  let mut decoder = ImageReader::with_format(Cursor::new(data), format)
    .into_decoder()
    .map_err(|err| PdfError::Image(format!("failed to open raster image: {err}")))?;
  let orientation = decoder.orientation().unwrap_or(Orientation::NoTransforms);
  let icc_profile = decoder.icc_profile().unwrap_or_default();
  let mut image = DynamicImage::from_decoder(decoder)
    .map_err(|err| PdfError::Image(format!("failed to decode raster image: {err}")))?;
  image.apply_orientation(orientation);
  let jpeg_has_real_physical_resolution = format == RasterImageFormat::Jpeg
    && jpeg_metadata(data).is_some_and(JpegMetadata::has_real_physical_resolution);
  Ok(DecodedRasterImage {
    image,
    icc_profile,
    jpeg_has_real_physical_resolution,
  })
}

fn decode_gdiplus_jpeg(data: &[u8]) -> Result<DecodedRasterImage> {
  let mut metadata_decoder = ImageReader::with_format(Cursor::new(data), RasterImageFormat::Jpeg)
    .into_decoder()
    .map_err(|err| PdfError::Image(format!("failed to open JPEG image: {err}")))?;
  let orientation = metadata_decoder
    .orientation()
    .unwrap_or(Orientation::NoTransforms);
  let icc_profile = metadata_decoder.icc_profile().unwrap_or_default();

  let mut image = if metadata_decoder.color_type() == image::ColorType::L8
    && let Some(image) = super::jpeg_islow::decode_gray(data)
  {
    DynamicImage::ImageLuma8(image)
  } else if let Some(image) = super::jpeg_islow::decode_rgb(data) {
    DynamicImage::ImageRgb8(image)
  } else {
    let mut decoder = jpeg_decoder::Decoder::new(Cursor::new(data));
    let pixels = decoder
      .decode()
      .map_err(|err| PdfError::Image(format!("failed to decode Office JPEG image: {err}")))?;
    let info = decoder
      .info()
      .ok_or_else(|| PdfError::Image("Office JPEG has no image information".to_string()))?;
    let dimensions = (u32::from(info.width), u32::from(info.height));
    match info.pixel_format {
      jpeg_decoder::PixelFormat::L8 => {
        image::GrayImage::from_raw(dimensions.0, dimensions.1, pixels).map(DynamicImage::ImageLuma8)
      }
      jpeg_decoder::PixelFormat::RGB24 => {
        image::RgbImage::from_raw(dimensions.0, dimensions.1, pixels).map(DynamicImage::ImageRgb8)
      }
      // Keep the established decoder for formats whose GDI+ conversion has
      // not been isolated. Native JPEG embedding never enters this path.
      jpeg_decoder::PixelFormat::L16 | jpeg_decoder::PixelFormat::CMYK32 => {
        return decode_dynamic_image(data, RasterImageFormat::Jpeg);
      }
    }
    .ok_or_else(|| PdfError::Image("Office JPEG has an invalid pixel buffer".to_string()))?
  };

  // System.Drawing's default Bitmap resize matches libjpeg's accurate integer
  // decoded source plane in the controlled JPEG matrix. Apply the package's
  // EXIF transform before that already-proven resize, as the ordinary decoder
  // does, while retaining its ICC ownership.
  image.apply_orientation(orientation);
  Ok(DecodedRasterImage {
    image,
    icc_profile,
    jpeg_has_real_physical_resolution: jpeg_metadata(data)
      .is_some_and(JpegMetadata::has_real_physical_resolution),
  })
}

fn prepare_sampled_image(
  image: PdfRasterImage,
  interpolate: bool,
  matte: Option<[f32; 3]>,
) -> Result<PreparedRasterImage> {
  let direct = DirectRasterImage {
    width: image.pixels.width,
    height: image.pixels.height,
    color_space: DirectRasterColorSpace::Rgb,
    bits_per_component: 8,
    encoding: DirectRasterEncoding::Sampled {
      pixels: image.pixels.clone(),
    },
    interpolate,
    soft_mask_interpolate: interpolate,
    matte,
  };
  Ok(PreparedRasterImage::new(direct))
}

fn prepare_native_png_image(
  data: Vec<u8>,
  interpolate: bool,
  indexed: Option<NativeIndexedPng>,
) -> Result<PreparedRasterImage> {
  let decoded = decode_dynamic_image(&data, RasterImageFormat::Png)?;
  let sampled = PdfRasterImage::from_dynamic_with_icc(decoded.image, decoded.icc_profile);
  let direct = if let Some(indexed) = indexed {
    DirectRasterImage {
      width: indexed.width,
      height: indexed.height,
      color_space: DirectRasterColorSpace::Rgb,
      bits_per_component: indexed.bit_depth as u8,
      encoding: DirectRasterEncoding::IndexedPng {
        data: indexed.idat.into(),
        palette: indexed.palette.into(),
        icc_profile: sampled.pixels.icc_profile.clone().map(Into::into),
      },
      interpolate,
      soft_mask_interpolate: false,
      matte: None,
    }
  } else {
    DirectRasterImage {
      width: sampled.pixels.width,
      height: sampled.pixels.height,
      color_space: DirectRasterColorSpace::Rgb,
      bits_per_component: 8,
      encoding: DirectRasterEncoding::Sampled {
        pixels: sampled.pixels,
      },
      interpolate,
      soft_mask_interpolate: interpolate,
      matte: None,
    }
  };
  Ok(PreparedRasterImage::new(direct))
}

#[derive(Debug)]
struct DirectJpegMetadata {
  width: u32,
  height: u32,
  color_space: DirectRasterColorSpace,
  invert_cmyk: bool,
  icc_profile: Option<Vec<u8>>,
}

fn direct_jpeg_metadata(data: &[u8]) -> Result<DirectJpegMetadata> {
  use zune_jpeg::zune_core::colorspace::ColorSpace;

  let mut decoder = zune_jpeg::JpegDecoder::new(Cursor::new(data));
  decoder
    .decode_headers()
    .map_err(|error| PdfError::Image(format!("failed to read JPEG headers: {error}")))?;
  let (width, height) = decoder
    .dimensions()
    .ok_or_else(|| PdfError::Image("JPEG has no dimensions".to_string()))?;
  let input = decoder
    .input_colorspace()
    .ok_or_else(|| PdfError::Image("JPEG has no input color space".to_string()))?;
  let (color_space, invert_cmyk) = match input {
    ColorSpace::Luma => (DirectRasterColorSpace::Gray, false),
    ColorSpace::RGB | ColorSpace::YCbCr => (DirectRasterColorSpace::Rgb, false),
    ColorSpace::CMYK | ColorSpace::YCCK => (DirectRasterColorSpace::Cmyk, true),
    _ => {
      return Err(PdfError::Image(format!(
        "unsupported JPEG input color space {input:?}"
      )));
    }
  };
  Ok(DirectJpegMetadata {
    width: u32::try_from(width)
      .map_err(|_| PdfError::Image("JPEG width exceeds u32".to_string()))?,
    height: u32::try_from(height)
      .map_err(|_| PdfError::Image("JPEG height exceeds u32".to_string()))?,
    color_space,
    invert_cmyk,
    icc_profile: decoder.icc_profile(),
  })
}

fn prepare_native_jpeg_image(data: Vec<u8>, interpolate: bool) -> Result<PreparedRasterImage> {
  let metadata = direct_jpeg_metadata(&data)?;
  let direct = DirectRasterImage {
    width: metadata.width,
    height: metadata.height,
    color_space: metadata.color_space,
    bits_per_component: 8,
    encoding: DirectRasterEncoding::Dct {
      data: data.into(),
      icc_profile: metadata.icc_profile.map(Into::into),
      invert_cmyk: metadata.invert_cmyk,
      alpha: None,
    },
    interpolate,
    soft_mask_interpolate: false,
    matte: None,
  };
  Ok(PreparedRasterImage::new(direct))
}

fn prepare_encoded_jpeg_image(
  data: Vec<u8>,
  icc_profile: Option<Vec<u8>>,
  interpolate: bool,
) -> Result<PreparedRasterImage> {
  let metadata = direct_jpeg_metadata(&data)?;
  let direct = DirectRasterImage {
    width: metadata.width,
    height: metadata.height,
    color_space: metadata.color_space,
    bits_per_component: 8,
    encoding: DirectRasterEncoding::Dct {
      data: data.into(),
      icc_profile: icc_profile.map(Into::into),
      invert_cmyk: metadata.invert_cmyk,
      alpha: None,
    },
    interpolate,
    soft_mask_interpolate: false,
    matte: None,
  };
  Ok(PreparedRasterImage::new(direct))
}

fn prepare_encoded_jpeg_image_with_soft_mask(
  data: Vec<u8>,
  icc_profile: Option<Vec<u8>>,
  alpha_source: &image::RgbaImage,
  interpolate: bool,
  soft_mask_interpolate: bool,
  matte: [f32; 3],
) -> Result<PreparedRasterImage> {
  let metadata = direct_jpeg_metadata(&data)?;
  if metadata.color_space != DirectRasterColorSpace::Rgb {
    return Err(PdfError::Image(
      "JPEG with a soft mask must use an RGB color space".to_string(),
    ));
  }
  if (metadata.width, metadata.height) != alpha_source.dimensions() {
    return Err(PdfError::Image(format!(
      "JPEG and soft-mask dimensions differ: {}x{} versus {}x{}",
      metadata.width,
      metadata.height,
      alpha_source.width(),
      alpha_source.height()
    )));
  }
  let alpha = alpha_source
    .pixels()
    .map(|pixel| pixel[3])
    .collect::<Vec<_>>();
  Ok(PreparedRasterImage::new(DirectRasterImage {
    width: metadata.width,
    height: metadata.height,
    color_space: metadata.color_space,
    bits_per_component: 8,
    encoding: DirectRasterEncoding::Dct {
      data: data.into(),
      icc_profile: icc_profile.map(Into::into),
      invert_cmyk: metadata.invert_cmyk,
      alpha: Some(alpha.into()),
    },
    interpolate,
    soft_mask_interpolate,
    matte: Some(matte),
  }))
}

fn export_decoded_image(
  mut raster: DecodedRasterImage,
  format: RasterImageFormat,
  export_options: RasterExportOptions,
  owner: RasterOwner,
) -> Result<PreparedRasterImage> {
  let office_gray_jpeg = format == RasterImageFormat::Jpeg
    && owner == RasterOwner::Source
    && export_options.profile.is_word()
    && raster.image.color() == image::ColorType::L8;
  let mut resized = false;
  if let Some(target_size) =
    export_options.downsample_source_size(raster.image.dimensions(), format, owner)
  {
    raster.image = if format == RasterImageFormat::Png
      && owner == RasterOwner::Source
      && export_options.profile.is_word()
      && export_options.profile.is_office_screen()
    {
      resize_for_word_screen_png(raster.image, target_size)
    } else if format == RasterImageFormat::Png
      && owner == RasterOwner::Source
      && export_options.profile.is_power_point_screen()
      && (!raster.image.color().has_alpha()
        || raster
          .image
          .pixels()
          .all(|(_, _, pixel)| pixel[3] == u8::MAX))
    {
      // Include opaque RGBA produced by DrawingML blip effects. The native
      // opaque PNG controls establish this path; transparent PowerPoint
      // surfaces retain their independently owned alpha realization.
      resize_for_gdiplus_bitmap(raster.image, target_size)
    } else {
      resize_for_export(raster.image, target_size, export_options)
    };
    resized = true;
  }
  if resized && format == RasterImageFormat::Jpeg && export_options.profile.is_office_fixed_output()
  {
    // Controlled Word Print/Screen matrices vary ICC, Photoshop 8BIM, JFIF
    // density, and the display-DPI boundary while keeping the JPEG scan bytes
    // fixed. Every reduced result discards the source profile and declares the
    // PDF image as DeviceRGB; an ICCBased resource changes the RGB result.
    // Requested exports still preserve the caller's profile below.
    raster.icc_profile = None;
  }

  let mut interpolate = raster_interpolation(format, export_options);
  if should_try_jpeg(format, export_options, owner) {
    let quality = export_options.jpeg_quality.unwrap_or(90);
    if office_gray_jpeg {
      // Preserve the component model before choosing its representation.
      // Comparing an expanded RGB JPEG against deflated RGB misclassifies the
      // native Gray JPEGs in Word's fixed-output references. Current Office
      // also retains DCT for tiny Gray sources when its stream exceeds the
      // raw RGB plane: independently verified for Print/Screen, dimensions,
      // content, JFIF metadata and three authored tiny-image documents.
      let jpeg = super::jpeg_islow_encoder::encode_gray(&raster.image.to_luma8(), quality)
        .ok_or_else(|| {
          PdfError::Image("failed to encode bounded Office grayscale JPEG image".to_string())
        })?;
      return prepare_encoded_jpeg_image(
        jpeg,
        raster.icc_profile,
        export_options.allow_interpolation,
      );
    }
    let rgba = raster.image.to_rgba8();
    let has_alpha = rgba.pixels().any(|pixel| pixel[3] != u8::MAX);
    let jpeg_source = if has_alpha {
      DynamicImage::ImageRgba8(apply_black_matte(&rgba))
    } else {
      raster.image.clone()
    };
    let jpeg_rgba = jpeg_source.to_rgba8();
    if !office_fixed_output_prefers_lossless(
      &jpeg_rgba,
      export_options,
      owner,
      raster.jpeg_has_real_physical_resolution,
    ) {
      let jpeg = if export_options.profile.is_office_fixed_output() {
        // All Office fixed-output raster owners use libjpeg's h2v2 box-filtered
        // chroma contract. Representation selection must compare the same bytes
        // Word could emit; the generic encoder's point-sampled 4:2:0 stream can
        // be smaller and incorrectly switch flat indexed graphics from Flate to
        // DCT even though both advertise the same sampling factors.
        encode_office_h2v2_jpeg(&jpeg_rgba, quality)?
      } else {
        encode_jpeg(&jpeg_source, quality)?
      };
      let lossless_color_bytes = deflated_rgb_size(&rgba);
      if jpeg.len() < lossless_color_bytes {
        if has_alpha {
          if owner == RasterOwner::PowerPointStatic3d
            || format == RasterImageFormat::Png
              && owner == RasterOwner::Source
              && export_options.profile.is_office_screen()
              && (export_options.profile.is_power_point_screen()
                || (resized && export_options.profile.is_word()))
          {
            // Ordinary PowerPoint Screen PNGs and reduced Word Screen PNGs
            // keep the associated JPEG color and independent A8 plane. Their
            // native Office controls use black Matte and interpolate the
            // JPEG color, not SMask. Decoding JPEG back to straight RGB loses
            // that independent reconstruction order at transparency edges.
            return prepare_encoded_jpeg_image_with_soft_mask(
              jpeg,
              raster.icc_profile,
              &rgba,
              export_options.allow_interpolation,
              false,
              [0.0, 0.0, 0.0],
            );
          }
          let compressed_rgb = decode_dynamic_image(&jpeg, RasterImageFormat::Jpeg)?
            .image
            .to_rgb8();
          // Other owners retain their existing straight-RGB reconstruction.
          // Unassociation agrees with black Matte at decoded sample points,
          // but interpolation order differs. Migrate each owner's contract
          // with native evidence, independently of the verified path above.
          let rgb = remove_black_matte(compressed_rgb, &rgba);
          return prepare_sampled_image(
            PdfRasterImage::from_rgb_with_alpha(rgb, &rgba, raster.icc_profile),
            export_options.allow_interpolation,
            None,
          );
        }
        return prepare_encoded_jpeg_image(
          jpeg,
          raster.icc_profile,
          export_options.allow_interpolation,
        );
      }
    }

    // Word fixed output does not pay the JPEG header/DCT overhead for tiny
    // rasters when it is larger than the decoded color plane. Its independent
    // 2x2 JPEG fixtures become 12-byte RGB XObjects with `/Interpolate false`,
    // while a 14x22 JPEG whose compressed stream is smaller remains DCT data.
    // The same filter-level policy applies to ordinary PNG color planes:
    // testWPGtextboxes' 39x29 opaque PNG becomes a quality-60 JPEG in Office.
    interpolate = false;
  }

  let associated_alpha = (resized && export_options.profile.is_office_screen()
    || owner == RasterOwner::PowerPointStatic3d)
    && raster.image.color().has_alpha()
    && raster
      .image
      .to_rgba8()
      .pixels()
      .any(|pixel| pixel[3] != u8::MAX);
  let image = if associated_alpha {
    // GDI+ samples transparent bitmaps in associated-alpha space. Office
    // keeps those black-preblended color samples and records the association
    // with SMask/Matte. Earlier we unassociated the samples solely because
    // the backend could not write Matte; the direct image now carries that
    // association without a post-serialization dictionary replacement.
    // PowerPoint's static-3D Flate surfaces use the same independent RGB/A8
    // association as its JPEG surfaces (Scene3d_shape_rotation and the
    // opaque-picture controls). Compression choice cannot change that order.
    DynamicImage::ImageRgba8(apply_black_matte(&raster.image.to_rgba8()))
  } else {
    raster.image
  };
  let pdf_raster = PdfRasterImage::from_dynamic_with_icc(image, raster.icc_profile);
  prepare_sampled_image(
    pdf_raster,
    interpolate,
    associated_alpha.then_some([0.0, 0.0, 0.0]),
  )
}

fn export_wordprocessing_locked_canvas_image(
  mut raster: DecodedRasterImage,
  export_options: RasterExportOptions,
) -> Result<PreparedRasterImage> {
  // Layout supplies a 200-DPI working canvas; the export profile owns the
  // final device grid. Honor that grid before either compression branch.
  // In particular, Screen's target must not be lost for lossless canvases.
  if let Some(target_size) = export_options.downsample_size(raster.image.dimensions()) {
    raster.image = resize_for_export(raster.image, target_size, export_options);
  }
  export_wordprocessing_locked_canvas_resolved(raster, export_options, false)
}

fn export_wordprocessing_locked_canvas_device_image(
  mut raster: DecodedRasterImage,
  export_options: RasterExportOptions,
) -> Result<PreparedRasterImage> {
  // The native canvas is the working surface, not an already reduced
  // 200-DPI preview. Resolve exactly once to the configured output grid.
  // Actual Office source/PDF captures across17 widths give exact A8 for this
  // fixed-area resolve; associated RGB differs by at most2 byte levels from
  // its additional native ARGB/PARGB conversions, not a second resize kernel.
  raster.image = DynamicImage::ImageRgba8(apply_black_matte(&raster.image.to_rgba8()));
  if let Some(target_size) = export_options.downsample_size(raster.image.dimensions()) {
    raster.image = resize_for_gdiplus_fixed_area(raster.image, target_size);
  }
  export_wordprocessing_locked_canvas_resolved(raster, export_options, true)
}

fn export_wordprocessing_locked_canvas_resolved(
  raster: DecodedRasterImage,
  export_options: RasterExportOptions,
  associated: bool,
) -> Result<PreparedRasterImage> {
  if export_options.use_lossless_compression || export_options.jpeg_quality.is_none() {
    return prepare_sampled_image(
      PdfRasterImage::from_dynamic_with_icc(raster.image, raster.icc_profile),
      false,
      associated.then_some([0.0, 0.0, 0.0]),
    );
  }

  let rgba = raster.image.to_rgba8();
  let premultiplied = if associated {
    rgba.clone()
  } else {
    apply_black_matte(&rgba)
  };
  let quality = std::env::var("OOXMLSDK_LOCKED_CANVAS_JPEG_QUALITY_PROBE")
    .ok()
    .and_then(|value| value.parse::<u8>().ok())
    .unwrap_or_else(|| export_options.jpeg_quality.unwrap_or(75));
  let jpeg = encode_office_h2v2_jpeg(&premultiplied, quality)?;
  // Word classifies the completed legacy surface, not the transparent PNG
  // transport used between layout and PDF rendering. Compare the two color
  // representations after black-matte association; counting transparent
  // black as a palette-dominant source would incorrectly force this surface
  // onto the ordinary PNG/lossless classifier.
  if jpeg.len() >= deflated_rgb_size(&premultiplied) {
    return prepare_sampled_image(
      PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(rgba), raster.icc_profile),
      false,
      associated.then_some([0.0, 0.0, 0.0]),
    );
  }
  let compressed_rgb = decode_dynamic_image(&jpeg, RasterImageFormat::Jpeg)?
    .image
    .to_rgb8();
  // PDF Reference 1.5 §7.5.4 defines SMask/Matte for exactly these associated
  // color samples. Keeping the decoded JPEG plane black-preblended avoids the
  // lossy divide/re-multiply round trip at partial-alpha glyph edges.
  let pdf_raster = PdfRasterImage::from_rgb_with_alpha(compressed_rgb, &rgba, raster.icc_profile);
  prepare_sampled_image(
    pdf_raster,
    export_options.allow_interpolation,
    Some([0.0, 0.0, 0.0]),
  )
}

fn export_wordprocessing_black_matte_effect_image(
  raster: DecodedRasterImage,
) -> Result<PreparedRasterImage> {
  // Adobe PDF Reference 1.6, soft-mask images, requires a Matte entry when
  // the parent color samples are preblended. Word's independently controlled
  // WPG/WPS glow and WPS outer-shadow outputs supply black-associated RGB,
  // the unchanged A8 mask, `/Interpolate false`, and `/Matte [0 0 0]`. The
  // layout transport is already associated, so preserve it without a second
  // premultiply or source-image resampling.
  let pdf_raster = PdfRasterImage::from_dynamic_with_icc(raster.image, raster.icc_profile);
  prepare_sampled_image(pdf_raster, false, Some([0.0, 0.0, 0.0]))
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct WordStatic3dPhysicalMapping {
  source_offset_x: f64,
  source_offset_y: f64,
  source_scale_x: f64,
  source_scale_y: f64,
}

impl Default for WordStatic3dPhysicalMapping {
  fn default() -> Self {
    Self {
      source_offset_x: 0.0,
      source_offset_y: 0.0,
      source_scale_x: 1.0,
      source_scale_y: 1.0,
    }
  }
}

impl WordStatic3dPhysicalMapping {
  fn decode(encoded: &[u8]) -> Option<Self> {
    const VERSION: u8 = 1;
    const ENCODED_LENGTH: usize = 1 + 4 * std::mem::size_of::<f32>();
    if encoded.len() != ENCODED_LENGTH || encoded[0] != VERSION {
      return None;
    }
    let value = |index: usize| {
      let start = 1 + index * std::mem::size_of::<f32>();
      let bytes: [u8; 4] = encoded
        .get(start..start + std::mem::size_of::<f32>())?
        .try_into()
        .ok()?;
      Some(f64::from(f32::from_be_bytes(bytes)))
    };
    let mapping = Self {
      source_offset_x: value(0)?,
      source_offset_y: value(1)?,
      source_scale_x: value(2)?,
      source_scale_y: value(3)?,
    };
    (mapping.source_offset_x.is_finite()
      && mapping.source_offset_y.is_finite()
      && mapping.source_scale_x.is_finite()
      && mapping.source_scale_y.is_finite()
      && mapping.source_scale_x > f64::EPSILON
      && mapping.source_scale_y > f64::EPSILON)
      .then_some(mapping)
  }
}

#[derive(Debug)]
struct WordStatic3dLayers {
  physical: image::RgbaImage,
  effects: Option<image::RgbaImage>,
  physical_mapping: WordStatic3dPhysicalMapping,
  final_grid: Option<image::RgbaImage>,
  backdrop_grid: Option<image::RgbaImage>,
}

fn png_chunk_data(data: &[u8], wanted: [u8; 4]) -> Option<&[u8]> {
  const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
  if data.get(..PNG_SIGNATURE.len()) != Some(PNG_SIGNATURE) {
    return None;
  }
  let mut offset = PNG_SIGNATURE.len();
  while offset.checked_add(12)? <= data.len() {
    let length = u32::from_be_bytes(data.get(offset..offset + 4)?.try_into().ok()?) as usize;
    let kind: [u8; 4] = data.get(offset + 4..offset + 8)?.try_into().ok()?;
    let payload_start = offset.checked_add(8)?;
    let payload_end = payload_start.checked_add(length)?;
    let chunk_end = payload_end.checked_add(4)?;
    if chunk_end > data.len() {
      return None;
    }
    if kind == wanted {
      return data.get(payload_start..payload_end);
    }
    if kind == *b"IEND" {
      break;
    }
    offset = chunk_end;
  }
  None
}

fn decode_wordprocessing_static_3d_layers(data: &[u8]) -> Option<WordStatic3dLayers> {
  let decode_rgba = |chunk| {
    image::load_from_memory_with_format(chunk, RasterImageFormat::Png)
      .ok()
      .map(DynamicImage::into_rgba8)
  };
  let physical = decode_rgba(png_chunk_data(data, WORD_STATIC_3D_PHYSICAL_RGBA_CHUNK)?)?;
  let effects = match png_chunk_data(data, WORD_STATIC_3D_EFFECT_RGBA_CHUNK) {
    Some(chunk) => Some(decode_rgba(chunk)?),
    None => None,
  };
  if effects
    .as_ref()
    .is_some_and(|effects| effects.dimensions() != physical.dimensions())
  {
    return None;
  }
  let physical_mapping = match png_chunk_data(data, WORD_STATIC_3D_PHYSICAL_MAPPING_CHUNK) {
    Some(encoded) => WordStatic3dPhysicalMapping::decode(encoded)?,
    None => WordStatic3dPhysicalMapping::default(),
  };
  let final_grid = match png_chunk_data(data, WORD_STATIC_3D_FINAL_GRID_RGBA_CHUNK) {
    Some(chunk) => Some(decode_rgba(chunk)?),
    None => None,
  };
  let backdrop_grid = match png_chunk_data(data, WORD_STATIC_3D_BACKDROP_GRID_RGBA_CHUNK) {
    Some(chunk) => Some(decode_rgba(chunk)?),
    None => None,
  };
  if backdrop_grid.as_ref().is_some_and(|backdrop| {
    final_grid
      .as_ref()
      .is_none_or(|physical| physical.dimensions() != backdrop.dimensions())
  }) {
    return None;
  }
  Some(WordStatic3dLayers {
    physical,
    effects,
    physical_mapping,
    final_grid,
    backdrop_grid,
  })
}

fn bilinear_premultiplied_rgba_sample(source: &image::RgbaImage, x: f64, y: f64) -> [f64; 4] {
  if source.width() == 0 || source.height() == 0 {
    return [0.0; 4];
  }
  let floor_x = x.floor();
  let floor_y = y.floor();
  let horizontal = x - floor_x;
  let vertical = y - floor_y;
  let left = floor_x.clamp(0.0, f64::from(source.width() - 1)) as u32;
  let top = floor_y.clamp(0.0, f64::from(source.height() - 1)) as u32;
  // Clamp the neighbors independently. Computing the next index from an
  // already-clamped negative index incorrectly blends the first interior
  // column/row into samples outside the leading edge.
  let right = (floor_x + 1.0).clamp(0.0, f64::from(source.width() - 1)) as u32;
  let bottom = (floor_y + 1.0).clamp(0.0, f64::from(source.height() - 1)) as u32;
  let samples = [
    source.get_pixel(left, top),
    source.get_pixel(right, top),
    source.get_pixel(left, bottom),
    source.get_pixel(right, bottom),
  ];
  let weights = [
    (1.0 - horizontal) * (1.0 - vertical),
    horizontal * (1.0 - vertical),
    (1.0 - horizontal) * vertical,
    horizontal * vertical,
  ];
  let mut result = [0.0; 4];
  for (sample, weight) in samples.into_iter().zip(weights) {
    let alpha = f64::from(sample[3]);
    for channel in 0..3 {
      result[channel] += f64::from(sample[channel]) * alpha / 255.0 * weight;
    }
    result[3] += alpha * weight;
  }
  result
}

fn resolve_wordprocessing_static_3d_screen_image(
  layers: &WordStatic3dLayers,
  target_size: (u32, u32),
) -> Option<image::RgbaImage> {
  let (target_width, target_height) = target_size;
  let (source_width, source_height) = layers.physical.dimensions();
  let direct_final_grid = match layers.final_grid.as_ref() {
    Some(grid) if grid.dimensions() != target_size => return None,
    grid => grid.cloned(),
  };
  if layers
    .backdrop_grid
    .as_ref()
    .is_some_and(|backdrop| direct_final_grid.is_none() || backdrop.dimensions() != target_size)
  {
    return None;
  }
  if source_width == 0
    || source_height == 0
    || target_width == 0
    || target_height == 0
    || (direct_final_grid.is_none()
      && (target_width > source_width || target_height > source_height))
  {
    return None;
  }

  // D3D9's standard eight-position pattern. The 48-cell fixed-CTM phase
  // matrix selects this exact x/y pairing over six independent alternatives,
  // and a separate 256-cell physical-surface validation keeps it uniquely
  // best across glyph, phase, camera, depth, bevel, and contour controls.
  const SAMPLES: [(f64, f64); 8] = [
    (9.0 / 16.0, 5.0 / 16.0),
    (7.0 / 16.0, 11.0 / 16.0),
    (13.0 / 16.0, 9.0 / 16.0),
    (5.0 / 16.0, 3.0 / 16.0),
    (3.0 / 16.0, 13.0 / 16.0),
    (1.0 / 16.0, 7.0 / 16.0),
    (11.0 / 16.0, 15.0 / 16.0),
    (15.0 / 16.0, 1.0 / 16.0),
  ];
  // The source mask addresses high-density pixel centers. Exact horizontal
  // and diagonal controls pin the conversion to a -1/2 center transform plus
  // a -1/4 source-pixel registration; the vertical registration cancels its
  // center transform. These values were frozen before the independent
  // physical-surface validation and are not fitted to the target document.
  const SOURCE_PHASE_X: f64 = -0.75;
  const SOURCE_PHASE_Y: f64 = 0.0;
  let mut physical = direct_final_grid.unwrap_or_else(|| {
    let mapping = layers.physical_mapping;
    let mut physical = image::RgbaImage::new(target_width, target_height);
    for y in 0..target_height {
      for x in 0..target_width {
        let mut covered = 0_u16;
        let mut accumulated_rgb = [0.0_f64; 3];
        for (sample_x, sample_y) in SAMPLES {
          let normalized_x = mapping.source_offset_x
            + (f64::from(x) + sample_x) / f64::from(target_width) * mapping.source_scale_x;
          let normalized_y = mapping.source_offset_y
            + (f64::from(y) + sample_y) / f64::from(target_height) * mapping.source_scale_y;
          let source_x = normalized_x * f64::from(source_width) + SOURCE_PHASE_X;
          let source_y = normalized_y * f64::from(source_height) + SOURCE_PHASE_Y;
          let sample = bilinear_premultiplied_rgba_sample(&layers.physical, source_x, source_y);
          if sample[3] >= 128.0 {
            covered += 1;
            for channel in 0..3 {
              accumulated_rgb[channel] += sample[channel];
            }
          }
        }
        let alpha = ((covered * 255 + 4) / 8) as u8;
        let mut pixel = [0_u8; 4];
        pixel[3] = alpha;
        if alpha != 0 {
          for channel in 0..3 {
            let premultiplied = accumulated_rgb[channel] / SAMPLES.len() as f64;
            pixel[channel] = (premultiplied * 255.0 / f64::from(alpha))
              .round()
              .clamp(0.0, 255.0) as u8;
          }
        }
        physical.put_pixel(x, y, Rgba(pixel));
      }
    }
    physical
  });

  // A completed scene-target plane has already sampled its independent source
  // texture. Reapplying the ordinary bitmap resizer would introduce another
  // filter and a different lattice. Legacy transports keep their old path.
  let effects = layers.backdrop_grid.clone().or_else(|| {
    layers.effects.as_ref().map(|effects| {
      resize_for_office_screen(DynamicImage::ImageRgba8(effects.clone()), target_size).to_rgba8()
    })
  });
  if let Some(effects) = effects.as_ref() {
    for (physical, effect) in physical.pixels_mut().zip(effects.pixels()) {
      let foreground_alpha = f64::from(physical[3]) / 255.0;
      let backdrop_alpha = f64::from(effect[3]) / 255.0;
      let output_alpha = foreground_alpha + backdrop_alpha * (1.0 - foreground_alpha);
      let mut output = [0_u8; 4];
      output[3] = (output_alpha * 255.0).round().clamp(0.0, 255.0) as u8;
      if output_alpha > f64::EPSILON {
        for channel in 0..3 {
          let premultiplied = f64::from(physical[channel]) * foreground_alpha
            + f64::from(effect[channel]) * backdrop_alpha * (1.0 - foreground_alpha);
          output[channel] = (premultiplied / output_alpha).round().clamp(0.0, 255.0) as u8;
        }
      }
      *physical = Rgba(output);
    }
  }
  Some(physical)
}

fn export_wordprocessing_static_3d_image(
  mut raster: DecodedRasterImage,
  export_options: RasterExportOptions,
  layers: Option<WordStatic3dLayers>,
) -> Result<PreparedRasterImage> {
  let resolved = export_options
    .profile
    .is_office_screen()
    .then(|| {
      let layers = layers.as_ref()?;
      // The physical surface is already sampled on layout's final lattice.
      // Its integer allocation and the PDF image matrix are separate domains;
      // a display-derived size or ordinary-image downsampling threshold must
      // neither replace the allocation nor bypass deferred effect composition.
      let target_size = layers
        .final_grid
        .as_ref()
        .map(image::RgbaImage::dimensions)
        .or_else(|| export_options.downsample_size(raster.image.dimensions()))?;
      resolve_wordprocessing_static_3d_screen_image(layers, target_size)
    })
    .flatten();
  if let Some(resolved) = resolved {
    raster.image = DynamicImage::ImageRgba8(resolved);
  } else if let Some(target_size) = export_options.downsample_size(raster.image.dimensions()) {
    raster.image = resize_for_export(raster.image, target_size, export_options);
  }

  let rgba = raster.image.to_rgba8();
  let premultiplied = apply_black_matte(&rgba);
  if export_options.use_lossless_compression {
    return prepare_sampled_image(
      PdfRasterImage::from_dynamic_with_icc(
        DynamicImage::ImageRgba8(premultiplied),
        raster.icc_profile,
      ),
      false,
      Some([0.0, 0.0, 0.0]),
    );
  }

  // Word classifies the completed black-associated static-3-D surface before
  // comparing encoded stream sizes. Exact-config content controls pin two
  // independent lossless owners: every RGB plane no larger than 64 KiB, and
  // larger planes whose per-channel top-ten values cover at least 60% of all
  // channel samples. The shared top-256 palette gate remains an earlier
  // positive for neutral/palette surfaces. Only a surface missing all three
  // gates reaches the JPEG-versus-Deflate comparison below.
  if export_options.profile.is_office_fixed_output()
    && office_static_3d_prefers_lossless(&premultiplied)
  {
    return prepare_sampled_image(
      PdfRasterImage::from_dynamic_with_icc(
        DynamicImage::ImageRgba8(premultiplied),
        raster.icc_profile,
      ),
      false,
      Some([0.0, 0.0, 0.0]),
    );
  }

  let quality = export_options.jpeg_quality.unwrap_or(75);
  let jpeg = encode_office_h2v2_jpeg(&premultiplied, quality)?;
  // Word applies the same content-sensitive color-stream choice to static-3-D
  // surfaces as to ordinary raster images.  Controlled Screen and Print
  // exports of tdf97371 keep its nearly solid surface as Flate with
  // `/Interpolate false`, while the textured surfaces in
  // shape-3d-effect-preservation use DCT with `/Interpolate true` under the
  // same PDF options.  Compare the two representations Word actually writes:
  // an h2v2 JPEG and a level-6 deflate of the black-matted RGB plane.
  if jpeg.len() >= deflated_rgb_size(&premultiplied) {
    return prepare_sampled_image(
      PdfRasterImage::from_dynamic_with_icc(
        DynamicImage::ImageRgba8(premultiplied),
        raster.icc_profile,
      ),
      false,
      Some([0.0, 0.0, 0.0]),
    );
  }
  // Keep Office's encoded, black-associated color plane intact. The direct
  // writer carries the original A8 samples in an independent SMask and writes
  // `/Matte [0 0 0]`, so PDF readers undo the association after decoding and
  // interpolation instead of amplifying JPEG error through a CPU-side divide.
  prepare_encoded_jpeg_image_with_soft_mask(
    jpeg,
    raster.icc_profile,
    &rgba,
    export_options.allow_interpolation,
    false,
    [0.0, 0.0, 0.0],
  )
}

fn export_wordprocessing_shape_story_image(
  mut raster: DecodedRasterImage,
  export_options: RasterExportOptions,
) -> Result<PreparedRasterImage> {
  if let Some(max_size) = export_options.max_size_px
    && let Some(target_size) = downsample_size(raster.image.dimensions(), max_size)
  {
    raster.image = resize_for_export(raster.image, target_size, export_options);
  }

  let rgba = resample_wordprocessing_shape_story_bitmap(&raster.image.to_rgba8());
  if export_options.use_lossless_compression {
    return prepare_sampled_image(
      PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(rgba), raster.icc_profile),
      export_options.allow_interpolation,
      None,
    );
  }

  let premultiplied = apply_black_matte(&rgba);
  let quality = export_options.jpeg_quality.unwrap_or(75);
  let jpeg = encode_office_h2v2_jpeg(&premultiplied, quality)?;
  let lossless_color_bytes = u64::from(rgba.width()) * u64::from(rgba.height()) * 3;
  if (jpeg.len() as u64) < lossless_color_bytes {
    // Office keeps the WPG color plane as DCT and associates its independent
    // A8 mask with black Matte (testWPGtextboxes objects 15/16). Preserve the
    // encoded samples: CPU-side unassociation would clip JPEG ringing before
    // the PDF reader can apply the mask and its interpolation contract.
    return prepare_encoded_jpeg_image_with_soft_mask(
      jpeg,
      raster.icc_profile,
      &rgba,
      export_options.allow_interpolation,
      false,
      [0.0, 0.0, 0.0],
    );
  }

  prepare_sampled_image(
    PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(rgba), raster.icc_profile),
    false,
    None,
  )
}

fn resample_wordprocessing_shape_story_bitmap(source: &image::RgbaImage) -> image::RgbaImage {
  // testWPGtextboxes' opaque 39x29 PNG acquires an alpha ramp of 167/191/223
  // at the top-left/top/left edges in both Word's 96-DPI screen export and its
  // independent 200-DPI print export. That is rectangular WPG coverage at a
  // 1/8px horizontal and 1/4px vertical phase, not transparent source-image
  // padding: the print image is rescaled to 81x60 while retaining the same
  // three coverage values. Keep source sampling edge-clamped and apply the
  // geometric coverage separately.
  //
  // Word's screen export does not sample this WPG picture directly at 96 DPI.
  // Controlled print exports expose a 200-DPI color surface; feeding that
  // surface through Windows GDI+ bilinear scaling reproduces the screen color
  // plane substantially more closely than a one-stage draw. Recreate the same
  // two resolutions here. Lanczos3 is the closest available premultiplied
  // approximation to the first high-quality Windows resample; the second pass
  // is the observed bilinear reduction.
  let intermediate_width = (u64::from(source.width()) * 200 / 96).max(1) as u32;
  let intermediate_height = (u64::from(source.height()) * 200 / 96).max(1) as u32;
  let intermediate = resize_for_pdf(
    DynamicImage::ImageRgba8(source.clone()),
    (intermediate_width, intermediate_height),
  )
  .to_rgba8();

  // The color plane and the WPG surface coverage are independent. A GDI+
  // control sweep places the 200-to-96-DPI color reduction at 3/8 pixel on
  // both axes, while Word's alpha mask retains the original 1/8-by-1/4 edge
  // coverage measured above.
  const COLOR_PHASE: f64 = 3.0 / 8.0;
  const COVERAGE_DENOMINATOR: f64 = 32.0;
  image::RgbaImage::from_fn(source.width(), source.height(), |x, y| {
    let axis = |index: u32, target_size: u32, source_size: u32| {
      let position = ((f64::from(index) + 0.5 - COLOR_PHASE) * f64::from(source_size)
        / f64::from(target_size)
        - 0.5)
        .clamp(0.0, f64::from(source_size.saturating_sub(1)));
      let lower = position.floor() as u32;
      let upper = lower.saturating_add(1).min(source_size.saturating_sub(1));
      (lower, upper, position - f64::from(lower))
    };
    let (left, right, horizontal) = axis(x, source.width(), intermediate.width());
    let (top, bottom, vertical) = axis(y, source.height(), intermediate.height());
    let samples = [
      ((1.0 - horizontal) * (1.0 - vertical), (left, top)),
      (horizontal * (1.0 - vertical), (right, top)),
      ((1.0 - horizontal) * vertical, (left, bottom)),
      (horizontal * vertical, (right, bottom)),
    ];
    let mut weighted_alpha = 0.0;
    let mut weighted_premultiplied = [0.0; 3];
    for (weight, (sample_x, sample_y)) in samples {
      let pixel = intermediate.get_pixel(sample_x, sample_y);
      let alpha = f64::from(pixel[3]);
      weighted_alpha += weight * alpha;
      for (channel, sum) in weighted_premultiplied.iter_mut().enumerate() {
        *sum += weight * alpha * f64::from(pixel[channel]);
      }
    }
    if weighted_alpha <= f64::EPSILON {
      return Rgba([0, 0, 0, 0]);
    }
    let coverage_x = if x == 0 { 28.0 } else { COVERAGE_DENOMINATOR };
    let coverage_y = if y == 0 { 24.0 } else { COVERAGE_DENOMINATOR };
    let alpha = (weighted_alpha * coverage_x * coverage_y / COVERAGE_DENOMINATOR.powi(2))
      .round()
      .clamp(0.0, 255.0) as u8;
    let color =
      weighted_premultiplied.map(|sum| (sum / weighted_alpha).round().clamp(0.0, 255.0) as u8);
    Rgba([color[0], color[1], color[2], alpha])
  })
}

fn apply_black_matte(image: &image::RgbaImage) -> image::RgbaImage {
  image::RgbaImage::from_fn(image.width(), image.height(), |x, y| {
    let pixel = image.get_pixel(x, y);
    let alpha = u16::from(pixel[3]);
    Rgba([
      ((u16::from(pixel[0]) * alpha + 127) / 255) as u8,
      ((u16::from(pixel[1]) * alpha + 127) / 255) as u8,
      ((u16::from(pixel[2]) * alpha + 127) / 255) as u8,
      pixel[3],
    ])
  })
}

fn remove_black_matte_component(matted: u8, alpha: u8) -> u8 {
  // A black Matte associates a color sample with its soft-mask sample:
  // `matted = straight * alpha`. Undo that association before handing the
  // image to a backend that only accepts ordinary straight RGB plus SMask.
  // The alpha-zero color is unobservable, so choose zero deterministically.
  // This is the same rounded integer form used by Cairo's PDF image path.
  let alpha = u16::from(alpha);
  (u16::from(matted) * 255 + alpha / 2)
    .checked_div(alpha)
    .unwrap_or_default()
    .min(255) as u8
}

fn remove_black_matte(
  mut image: image::RgbImage,
  alpha_source: &image::RgbaImage,
) -> image::RgbImage {
  assert_eq!(image.dimensions(), alpha_source.dimensions());
  for (pixel, source) in image.pixels_mut().zip(alpha_source.pixels()) {
    for channel in &mut pixel.0 {
      *channel = remove_black_matte_component(*channel, source[3]);
    }
  }
  image
}

fn deflated_rgb_size(image: &image::RgbaImage) -> usize {
  let mut rgb = Vec::with_capacity(image.width() as usize * image.height() as usize * 3);
  for pixel in image.pixels() {
    rgb.extend_from_slice(&pixel.0[..3]);
  }

  // Krilla writes custom RGB image planes through a level-6 zlib stream.
  // Compare the configured JPEG against that actual competing representation,
  // not against the uncompressed pixel count. Word makes the same content-
  // sensitive choice: screen-optimized 3-D shapes can be JPEG+Interpolate,
  // while flat or tiny surfaces remain Flate+non-interpolated under the same
  // JPEG-quality option.
  let mut encoder = ZlibEncoder::new(Vec::new(), Compression::new(6));
  encoder
    .write_all(&rgb)
    .expect("writing RGB bytes to an in-memory zlib stream cannot fail");
  encoder
    .finish()
    .expect("finishing an in-memory zlib stream cannot fail")
    .len()
}

fn office_fixed_output_prefers_lossless(
  image: &image::RgbaImage,
  export_options: RasterExportOptions,
  owner: RasterOwner,
  jpeg_has_real_physical_resolution: bool,
) -> bool {
  if !export_options.profile.is_office_fixed_output()
    || export_options.profile.is_office_screen()
    || !matches!(
      owner,
      RasterOwner::Source
        | RasterOwner::MaterializedSourceRectangleCrop
        | RasterOwner::PowerPointStatic3d
    )
  {
    return false;
  }

  // WIC/GDI+ treats JFIF unit 0 as an aspect ratio only. Exact-config Office
  // matrices which changed only that byte show that a real DPI declaration
  // enters the photographic owner, while unit 0 and absent JFIF metadata keep
  // the same small logo in the content classifier.
  if jpeg_has_real_physical_resolution {
    return false;
  }

  let metrics = if owner == RasterOwner::PowerPointStatic3d {
    // Scene3d_cropped_image exports DCT even though its transparent canvas
    // makes the full plane look like a >95% palette image. Only covered
    // samples describe the projected scene's color content.
    office_rgb_histogram_metrics_from_pixels(
      image
        .pixels()
        .filter(|pixel| pixel[3] != 0)
        .map(|pixel| [pixel[0], pixel[1], pixel[2]]),
    )
  } else {
    office_rgb_histogram_metrics(image)
  };
  // Microsoft's document-image compression algorithm first keeps images whose
  // 256 most common exact colors cover at least 95% of the samples on the
  // palette/lossless path. Indexed and true-color encodings of the same 254
  // colors produce the same Office PDF decision, so this is a pixel property,
  // not a PNG-format exception.
  if office_top_256_palette_gate_from_metrics(metrics) {
    return true;
  }

  // The next type-dependent gate uses uncompressed image size. Word and
  // PowerPoint agree that 154x141 RGB (65,142 bytes) remains lossless while
  // 155x142 RGB (66,030 bytes) enters JPEG analysis; 150x150 independently
  // rules out a longest-edge threshold and pins the boundary at 64 KiB.
  // Unlike the color histogram, this gate owns the complete stored plane.
  let stored_samples = u64::from(image.width()) * u64::from(image.height());
  if stored_samples.saturating_mul(3) > OFFICE_SMALL_RASTER_UNCOMPRESSED_RGB_BYTES {
    return false;
  }

  // For the remaining small non-palette bitmaps, Office uses the three-channel
  // histogram described by Microsoft's document compression algorithm. A
  // controlled 138x126 interpolation matrix holds size and palette coverage
  // fixed: 60.3136% in the per-channel top ten stays Flate, while 59.6599%
  // switches to JPEG. Compare as exact integers at the 60% boundary.
  office_per_channel_top_10_gate_from_metrics(metrics)
}

fn office_static_3d_prefers_lossless(image: &image::RgbaImage) -> bool {
  let stored_sample_count = u64::from(image.width()) * u64::from(image.height());
  // The size gate owns the complete stored RGB plane. The color classifier,
  // however, owns only samples which contribute to the independently stored
  // SMask: counting black-associated, fully transparent padding as image
  // content turns every sparse 3-D text surface into a false monochrome
  // positive. Exact-config Word controls with 44%--64% transparent padding
  // retain the full plane dimensions but select DCT from the covered colors.
  let visible_metrics = office_rgb_histogram_metrics_from_pixels(
    image
      .pixels()
      .filter(|pixel| pixel[3] != 0)
      .map(|pixel| [pixel[0], pixel[1], pixel[2]]),
  );
  stored_sample_count.saturating_mul(3) <= OFFICE_SMALL_RASTER_UNCOMPRESSED_RGB_BYTES
    || office_top_256_palette_gate_from_metrics(visible_metrics)
    || office_per_channel_top_10_gate_from_metrics(visible_metrics)
}

fn office_top_256_palette_gate_from_metrics(metrics: OfficeRgbHistogramMetrics) -> bool {
  metrics.top_256_color_samples.saturating_mul(100) >= metrics.sample_count.saturating_mul(95)
}

fn office_per_channel_top_10_gate_from_metrics(metrics: OfficeRgbHistogramMetrics) -> bool {
  metrics.per_channel_top_10_samples.saturating_mul(5) >= metrics.sample_count.saturating_mul(9)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OfficeRgbHistogramMetrics {
  sample_count: u64,
  top_256_color_samples: u64,
  per_channel_top_10_samples: u64,
}

fn office_rgb_histogram_metrics(image: &image::RgbaImage) -> OfficeRgbHistogramMetrics {
  office_rgb_histogram_metrics_from_pixels(
    image.pixels().map(|pixel| [pixel[0], pixel[1], pixel[2]]),
  )
}

fn office_rgb_histogram_metrics_from_pixels(
  pixels: impl Iterator<Item = [u8; 3]>,
) -> OfficeRgbHistogramMetrics {
  let mut sample_count = 0_u64;
  let mut channel_histograms = [[0_u64; 256]; 3];
  let mut exact_colors: HashMap<u32, u64> = HashMap::default();
  for pixel in pixels {
    sample_count += 1;
    for channel in 0..3 {
      channel_histograms[channel][usize::from(pixel[channel])] += 1;
    }
    let color = (u32::from(pixel[0]) << 16) | (u32::from(pixel[1]) << 8) | u32::from(pixel[2]);
    *exact_colors.entry(color).or_default() += 1;
  }

  if sample_count == 0 {
    return OfficeRgbHistogramMetrics {
      sample_count: 0,
      top_256_color_samples: 0,
      per_channel_top_10_samples: 0,
    };
  }

  let per_channel_top_10_samples = channel_histograms
    .iter_mut()
    .map(|histogram| {
      histogram.sort_unstable();
      histogram[histogram.len() - 10..].iter().sum::<u64>()
    })
    .sum();
  let mut exact_counts = exact_colors.into_values().collect::<Vec<_>>();
  let top_256_color_samples = if exact_counts.len() <= 256 {
    sample_count
  } else {
    let top_start = exact_counts.len() - 256;
    let _ = exact_counts.select_nth_unstable(top_start);
    exact_counts[top_start..].iter().sum()
  };

  OfficeRgbHistogramMetrics {
    sample_count,
    top_256_color_samples,
    per_channel_top_10_samples,
  }
}

fn downsample_size(size: (u32, u32), max_size: RasterPixelLimits) -> Option<(u32, u32)> {
  let (width, height) = size;
  let (max_width, max_height) = max_size.pixels();
  if width <= 50
    || height <= 50
    || (f64::from(width) <= max_width + 4.0 && f64::from(height) <= max_height + 4.0)
  {
    return None;
  }

  let scale = (max_width / f64::from(width)).min(max_height / f64::from(height));
  let target_width = (f64::from(width) * scale).round() as u32;
  let target_height = (f64::from(height) * scale).round() as u32;
  (target_width > 0 && target_height > 0).then_some((target_width, target_height))
}

fn downsample_office_fixed_output(
  size: (u32, u32),
  display_surface: RasterPixelLimits,
) -> Option<(u32, u32)> {
  let (width, height) = size;
  let (surface_width, surface_height) = display_surface.pixels();
  if width <= 50
    || height <= 50
    || (f64::from(width) <= surface_width + 4.0 && f64::from(height) <= surface_height + 4.0)
  {
    return None;
  }

  // The DrawingML/VML display frame owns Office's fixed-output bitmap surface.
  // Its axes can intentionally differ from the embedded source aspect ratio;
  // fitting the source proportionally a second time changes the device width.
  // Never enlarge a source axis, but otherwise allocate both frame axes
  // independently at the profile's device density.
  let target_width = (surface_width.round() as u32).clamp(1, width);
  let target_height = (surface_height.round() as u32).clamp(1, height);
  (target_width != width || target_height != height).then_some((target_width, target_height))
}

fn resize_for_export(
  image: DynamicImage,
  target_size: (u32, u32),
  export_options: RasterExportOptions,
) -> DynamicImage {
  if export_options.use_gdiplus_jpeg_resampler {
    resize_for_gdiplus_bitmap(image, target_size)
  } else if export_options.profile.is_office_screen() {
    resize_for_office_screen(image, target_size)
  } else {
    resize_for_pdf(image, target_size)
  }
}

const GDIPLUS_FIXED_ONE: i64 = 1 << 16;
const GDIPLUS_FIXED_HALF: i64 = GDIPLUS_FIXED_ONE / 2;
const GDIPLUS_BITMAP_AREA_THRESHOLD: u32 = 420;

#[derive(Clone, Copy, Debug)]
struct GdiplusAreaSpan {
  left: i64,
  right: i64,
  first: u32,
  last: u32,
}

#[derive(Debug)]
struct GdiplusAreaAxis {
  spans: Vec<GdiplusAreaSpan>,
  reciprocal: u64,
}

fn resize_for_gdiplus_bitmap(image: DynamicImage, target_size: (u32, u32)) -> DynamicImage {
  let source_size = image.dimensions();
  if source_size.0.max(source_size.1) < GDIPLUS_BITMAP_AREA_THRESHOLD {
    return resize_for_office_screen(image, target_size);
  }
  resize_for_gdiplus_fixed_area(image, target_size)
}

fn resize_for_word_screen_png(image: DynamicImage, target_size: (u32, u32)) -> DynamicImage {
  let source_size = image.dimensions();
  if source_size.0.max(source_size.1) < GDIPLUS_BITMAP_AREA_THRESHOLD {
    return resize_for_office_screen(image, target_size);
  }
  if !image.color().has_alpha() {
    return resize_for_gdiplus_fixed_area(image, target_size);
  }

  // PNG controls at 419/420 on either axis expose the same Q16 area kernel
  // as large Word Print JPEGs. Filter associated color and coverage together;
  // hidden RGB must not leak through a transparent sample. This helper keeps
  // the caller's straight-alpha contract: its later black-matte association
  // exactly recovers every filtered associated byte (including partial alpha).
  let associated = apply_black_matte(&image.to_rgba8());
  let mut resized =
    resize_for_gdiplus_fixed_area(DynamicImage::ImageRgba8(associated), target_size).to_rgba8();
  for pixel in resized.pixels_mut() {
    let alpha = pixel[3];
    for channel in &mut pixel.0[..3] {
      *channel = remove_black_matte_component(*channel, alpha);
    }
  }
  DynamicImage::ImageRgba8(resized)
}

fn resize_for_gdiplus_fixed_area(image: DynamicImage, target_size: (u32, u32)) -> DynamicImage {
  let source = image.to_rgba8();
  let (source_width, source_height) = source.dimensions();
  let (target_width, target_height) = target_size;
  let Some(horizontal_axis) = gdiplus_area_axis(source_width, target_width) else {
    return DynamicImage::ImageRgba8(source);
  };
  let Some(vertical_axis) = gdiplus_area_axis(source_height, target_height) else {
    return DynamicImage::ImageRgba8(source);
  };

  // `Bitmap(Image, width, height)` is the byte-exact producer for every
  // reduced Word Print JPEG in the controlled host/content/scale matrix. Its
  // large-bitmap path is a separable pixel-area scaler, implemented with two
  // independently truncated 16.16 factors. An exact impulse/ramp/checker
  // matrix fixes all observable arithmetic below: source coverage uses
  // `floor(src * 2^16 / dst)`, normalization uses
  // `floor(dst * 2^16 / src)`, and each pass rounds the Q32 product once.
  let mut horizontal = image::RgbaImage::new(target_width, source_height);
  for y in 0..source_height {
    for (x, span) in (0..target_width).zip(&horizontal_axis.spans) {
      let mut components = [0_u128; 4];
      for source_x in span.first..=span.last {
        let weight = gdiplus_area_overlap(*span, source_x);
        let pixel = source.get_pixel(source_x, y);
        for (component, sample) in components.iter_mut().zip(pixel.0) {
          *component += u128::from(sample) * u128::from(weight);
        }
      }
      horizontal.put_pixel(
        x,
        y,
        Rgba(
          components.map(|component| gdiplus_area_component(component, horizontal_axis.reciprocal)),
        ),
      );
    }
  }

  let mut output = image::RgbaImage::new(target_width, target_height);
  for (y, span) in (0..target_height).zip(&vertical_axis.spans) {
    for x in 0..target_width {
      let mut components = [0_u128; 4];
      for source_y in span.first..=span.last {
        let weight = gdiplus_area_overlap(*span, source_y);
        let pixel = horizontal.get_pixel(x, source_y);
        for (component, sample) in components.iter_mut().zip(pixel.0) {
          *component += u128::from(sample) * u128::from(weight);
        }
      }
      output.put_pixel(
        x,
        y,
        Rgba(
          components.map(|component| gdiplus_area_component(component, vertical_axis.reciprocal)),
        ),
      );
    }
  }
  DynamicImage::ImageRgba8(output)
}

fn gdiplus_area_axis(source: u32, target: u32) -> Option<GdiplusAreaAxis> {
  if source == 0 || target == 0 || target > source {
    return None;
  }
  let fixed_one = u64::try_from(GDIPLUS_FIXED_ONE).ok()?;
  let step = u64::from(source).checked_mul(fixed_one)? / u64::from(target);
  let reciprocal = u64::from(target).checked_mul(fixed_one)? / u64::from(source);
  let step = i64::try_from(step).ok()?;
  if step <= 0 || reciprocal == 0 {
    return None;
  }

  let mut spans = Vec::with_capacity(usize::try_from(target).ok()?);
  for destination in 0..target {
    let left = (-GDIPLUS_FIXED_HALF).checked_add(i64::from(destination).checked_mul(step)?)?;
    let right = left.checked_add(step)?;
    let first = (left + GDIPLUS_FIXED_HALF)
      .div_euclid(GDIPLUS_FIXED_ONE)
      .clamp(0, i64::from(source - 1)) as u32;
    let last = (right + GDIPLUS_FIXED_HALF - 1)
      .div_euclid(GDIPLUS_FIXED_ONE)
      .clamp(0, i64::from(source - 1)) as u32;
    spans.push(GdiplusAreaSpan {
      left,
      right,
      first,
      last,
    });
  }
  Some(GdiplusAreaAxis { spans, reciprocal })
}

fn gdiplus_area_overlap(span: GdiplusAreaSpan, sample: u32) -> u64 {
  let center = i64::from(sample) * GDIPLUS_FIXED_ONE;
  let left = span.left.max(center - GDIPLUS_FIXED_HALF);
  let right = span.right.min(center + GDIPLUS_FIXED_HALF);
  u64::try_from((right - left).max(0)).unwrap_or(0)
}

fn gdiplus_area_component(accumulated: u128, reciprocal: u64) -> u8 {
  let value = (accumulated * u128::from(reciprocal) + (1_u128 << 31)) >> 32;
  u8::try_from(value).unwrap_or(u8::MAX)
}

fn resize_for_office_screen(image: DynamicImage, target_size: (u32, u32)) -> DynamicImage {
  let source = image.to_rgba8();
  let (source_width, source_height) = source.dimensions();
  let (target_width, target_height) = target_size;
  if source_width == 0 || source_height == 0 || target_width == 0 || target_height == 0 {
    return DynamicImage::ImageRgba8(source);
  }

  // GDI+ DrawImage maps integer destination coordinates through the inverse
  // transform without adding a half pixel. Its default Graphics state uses
  // bilinear interpolation; Wine and libgdiplus preserve the same contract.
  // Controlled Office screen exports make both parts observable: a 116x72
  // alpha impulse reduced to 55x34 yields alpha 97 at x=24 and 165 at y=14,
  // exactly the four-point weights for 24*116/55 and 14*72/34.
  DynamicImage::ImageRgba8(image::RgbaImage::from_fn(
    target_width,
    target_height,
    |x, y| {
      let source_x = x as f64 * f64::from(source_width) / f64::from(target_width);
      let source_y = y as f64 * f64::from(source_height) / f64::from(target_height);
      gdiplus_bilinear_sample(&source, source_x, source_y)
    },
  ))
}

fn gdiplus_bilinear_sample(source: &image::RgbaImage, x: f64, y: f64) -> Rgba<u8> {
  let left = x.floor() as u32;
  let top = y.floor() as u32;
  let right = x.ceil() as u32;
  let bottom = y.ceil() as u32;
  let horizontal = x - x.floor();
  let vertical = y - y.floor();
  let sample = |sample_x: u32, sample_y: u32| {
    source
      .get_pixel(
        sample_x.min(source.width().saturating_sub(1)),
        sample_y.min(source.height().saturating_sub(1)),
      )
      .0
  };
  let samples = [
    sample(left, top),
    sample(right, top),
    sample(left, bottom),
    sample(right, bottom),
  ];
  let weights = [
    (1.0 - horizontal) * (1.0 - vertical),
    horizontal * (1.0 - vertical),
    (1.0 - horizontal) * vertical,
    horizontal * vertical,
  ];
  let alpha = samples
    .iter()
    .zip(weights)
    .map(|(sample, weight)| f64::from(sample[3]) * weight)
    .sum::<f64>();
  if alpha <= f64::EPSILON {
    return Rgba([0; 4]);
  }
  let mut result = [0_u8; 4];
  result[3] = alpha.round().clamp(0.0, 255.0) as u8;
  for channel in 0..3 {
    let premultiplied = samples
      .iter()
      .zip(weights)
      .map(|(sample, weight)| f64::from(sample[channel]) * f64::from(sample[3]) / 255.0 * weight)
      .sum::<f64>();
    result[channel] = (premultiplied * 255.0 / alpha).round().clamp(0.0, 255.0) as u8;
  }
  Rgba(result)
}

fn resize_for_pdf(image: DynamicImage, target_size: (u32, u32)) -> DynamicImage {
  if !image.color().has_alpha() {
    return image.resize_exact(target_size.0, target_size.1, FilterType::Lanczos3);
  }

  let source = image.to_rgba8();
  if source.pixels().all(|pixel| pixel[3] == 255) {
    return DynamicImage::ImageRgba8(source).resize_exact(
      target_size.0,
      target_size.1,
      FilterType::Lanczos3,
    );
  }

  // LibreOffice's convolution scaler operates N32BitTcRgba in premultiplied
  // space. This also matches Word's transparent bitmap XObjects: hidden RGB
  // samples cannot bleed into a partially covered edge during downsampling.
  let premultiplied = apply_black_matte(&source);
  let mut resized = DynamicImage::ImageRgba8(premultiplied)
    .resize_exact(target_size.0, target_size.1, FilterType::Lanczos3)
    .to_rgba8();
  for pixel in resized.pixels_mut() {
    let alpha = pixel[3];
    for channel in &mut pixel.0[..3] {
      *channel = remove_black_matte_component(*channel, alpha);
    }
  }
  DynamicImage::ImageRgba8(resized)
}

fn encode_jpeg(image: &image::DynamicImage, quality: u8) -> Result<Vec<u8>> {
  let rgb = image.to_rgb8();
  let (width, height) = rgb.dimensions();
  let width = u16::try_from(width)
    .map_err(|_| PdfError::Image("JPEG width exceeds 65535 pixels".to_string()))?;
  let height = u16::try_from(height)
    .map_err(|_| PdfError::Image("JPEG height exceeds 65535 pixels".to_string()))?;
  let mut jpeg = Vec::new();
  let mut encoder = JpegEncoder::new(&mut jpeg, quality);
  // Office fixed-format JPEG XObjects use conventional 4:2:0 chroma
  // subsampling. `image`'s encoder currently emits 4:4:4 regardless of the
  // requested quality, so use an encoder with an explicit sampling contract.
  encoder.set_sampling_factor(SamplingFactor::R_4_2_0);
  encoder
    .encode(rgb.as_raw(), width, height, JpegColorType::Rgb)
    .map_err(|err| PdfError::Image(format!("failed to encode JPEG image: {err}")))?;
  Ok(jpeg)
}

fn encode_office_h2v2_jpeg(image: &image::RgbaImage, quality: u8) -> Result<Vec<u8>> {
  super::jpeg_islow_encoder::encode_rgba_h2v2(image, quality).ok_or_else(|| {
    PdfError::Image("failed to encode bounded Office baseline JPEG image".to_string())
  })
}

/// Reproduce PowerPoint's fixed-output treatment of bitmap pixels lifted from
/// an ActiveX WMF preview: JPEG-compress the WMF DIB color plane while
/// preserving its alpha plane exactly.
pub(super) fn powerpoint_activex_bitmap_png(data: &[u8], quality: u8) -> Result<Vec<u8>> {
  let original = decode_dynamic_image(data, RasterImageFormat::Png)?
    .image
    .to_rgba8();
  let (width, height) = original.dimensions();
  let jpeg = encode_office_h2v2_jpeg(&original, quality)?;
  let recompressed = decode_dynamic_image(&jpeg, RasterImageFormat::Jpeg)?
    .image
    .to_rgb8();
  let opaque = original.pixels().all(|pixel| pixel[3] == u8::MAX);
  let mut output = Vec::new();
  if opaque {
    PngEncoder::new(&mut output)
      .write_image(recompressed.as_raw(), width, height, ColorType::Rgb8.into())
      .map_err(|err| PdfError::Image(format!("failed to encode ActiveX bitmap PNG: {err}")))?;
  } else {
    let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
    for (color, source) in recompressed.pixels().zip(original.pixels()) {
      rgba.extend_from_slice(&[color[0], color[1], color[2], source[3]]);
    }
    PngEncoder::new(&mut output)
      .write_image(&rgba, width, height, ColorType::Rgba8.into())
      .map_err(|err| PdfError::Image(format!("failed to encode ActiveX bitmap PNG: {err}")))?;
  }
  Ok(output)
}

fn decode_png_relaxed(data: &[u8]) -> std::result::Result<PdfRasterImage, String> {
  let mut decoder = png::Decoder::new(Cursor::new(data));
  decoder.ignore_checksums(true);
  decoder.set_transformations(png::Transformations::normalize_to_color8());
  let mut reader = decoder.read_info().map_err(|err| err.to_string())?;
  let buffer_size = reader
    .output_buffer_size()
    .ok_or_else(|| "PNG output buffer size is unavailable".to_string())?;
  let mut buffer = vec![0; buffer_size];
  let info = reader
    .next_frame(&mut buffer)
    .map_err(|err| err.to_string())?;
  buffer.truncate(info.buffer_size());
  Ok(PdfRasterImage::from_png_frame(
    info.width,
    info.height,
    info.color_type,
    &buffer,
  ))
}

fn image_format_from_content_type(content_type: &str) -> Option<RasterImageFormat> {
  match content_type {
    "image/png" => Some(RasterImageFormat::Png),
    "image/jpeg" | "image/jpg" => Some(RasterImageFormat::Jpeg),
    "image/gif" => Some(RasterImageFormat::Gif),
    "image/tif" | "image/tiff" => Some(RasterImageFormat::Tiff),
    "image/webp" => Some(RasterImageFormat::WebP),
    _ => None,
  }
}

fn raster_format(data: &[u8], content_type: Option<&str>) -> Option<RasterImageFormat> {
  // Office and LibreOffice discover an image decoder from the stream's
  // identifying bytes. The package media type remains the fallback for
  // formats whose bytes are not recognized, so malformed declared PNGs still
  // fail through the PNG decoder instead of being accepted as unknown data.
  image::guess_format(data)
    .ok()
    .or_else(|| content_type.and_then(image_format_from_content_type))
}

#[derive(Clone, Debug)]
struct PdfRasterImage {
  pixels: Arc<PdfRasterPixels>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct PdfRasterPixels {
  pub(super) width: u32,
  pub(super) height: u32,
  pub(super) rgb: Vec<u8>,
  pub(super) alpha: Option<Vec<u8>>,
  pub(super) icc_profile: Option<Vec<u8>>,
}

impl PdfRasterImage {
  fn from_dynamic_with_icc(image: image::DynamicImage, icc_profile: Option<Vec<u8>>) -> Self {
    Self::from_dynamic_with_icc_and_hidden_rgb(image, icc_profile, false)
  }

  fn from_dynamic_preserving_hidden_rgb(
    image: image::DynamicImage,
    icc_profile: Option<Vec<u8>>,
  ) -> Self {
    Self::from_dynamic_with_icc_and_hidden_rgb(image, icc_profile, true)
  }

  fn from_dynamic_with_icc_and_hidden_rgb(
    image: image::DynamicImage,
    icc_profile: Option<Vec<u8>>,
    preserve_hidden_rgb: bool,
  ) -> Self {
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8();
    let mut rgb = Vec::with_capacity(width as usize * height as usize * 3);
    let mut alpha = Vec::with_capacity(width as usize * height as usize);
    let mut opaque = true;

    for Rgba([r, g, b, a]) in rgba.pixels() {
      let sample = if preserve_hidden_rgb {
        [*r, *g, *b]
      } else {
        visible_rgb(*r, *g, *b, *a)
      };
      rgb.extend_from_slice(&sample);
      alpha.push(*a);
      opaque &= *a == u8::MAX;
    }

    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha: (!opaque).then_some(alpha),
        icc_profile,
      }),
    }
  }

  fn from_png_frame(width: u32, height: u32, color_type: png::ColorType, data: &[u8]) -> Self {
    let pixel_count = width as usize * height as usize;
    let mut rgb = Vec::with_capacity(pixel_count * 3);
    let mut alpha = Vec::with_capacity(pixel_count);
    let mut opaque = true;

    match color_type {
      png::ColorType::Grayscale => {
        for value in data {
          rgb.extend_from_slice(&[*value, *value, *value]);
        }
      }
      png::ColorType::GrayscaleAlpha => {
        for pixel in data.as_chunks::<2>().0.iter() {
          rgb.extend_from_slice(&visible_rgb(pixel[0], pixel[0], pixel[0], pixel[1]));
          alpha.push(pixel[1]);
          opaque &= pixel[1] == u8::MAX;
        }
      }
      png::ColorType::Rgb => {
        rgb.extend_from_slice(data);
      }
      png::ColorType::Rgba => {
        for pixel in data.as_chunks::<4>().0.iter() {
          rgb.extend_from_slice(&visible_rgb(pixel[0], pixel[1], pixel[2], pixel[3]));
          alpha.push(pixel[3]);
          opaque &= pixel[3] == u8::MAX;
        }
      }
      png::ColorType::Indexed => {}
    }

    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha: (!opaque && !alpha.is_empty()).then_some(alpha),
        icc_profile: None,
      }),
    }
  }

  fn from_rgb_with_alpha(
    mut rgb: image::RgbImage,
    alpha_source: &image::RgbaImage,
    icc_profile: Option<Vec<u8>>,
  ) -> Self {
    debug_assert_eq!(rgb.dimensions(), alpha_source.dimensions());
    let (width, height) = rgb.dimensions();
    for (pixel, alpha) in rgb.pixels_mut().zip(alpha_source.pixels()) {
      pixel.0 = visible_rgb(pixel[0], pixel[1], pixel[2], alpha[3]);
    }
    let alpha = alpha_source
      .pixels()
      .map(|pixel| pixel[3])
      .collect::<Vec<_>>();
    let opaque = alpha.iter().all(|alpha| *alpha == u8::MAX);
    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb: rgb.into_raw(),
        alpha: (!opaque).then_some(alpha),
        icc_profile,
      }),
    }
  }
}

fn visible_rgb(red: u8, green: u8, blue: u8, alpha: u8) -> [u8; 3] {
  if alpha == 0 {
    [0, 0, 0]
  } else {
    [red, green, blue]
  }
}

impl Hash for PdfRasterImage {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.pixels.width.hash(state);
    self.pixels.height.hash(state);
    self.pixels.rgb.hash(state);
    self.pixels.alpha.hash(state);
    self.pixels.icc_profile.hash(state);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use image::codecs::{jpeg::JpegEncoder as ImageJpegEncoder, tiff::TiffEncoder};

  #[test]
  fn metafile_preview_associates_color_and_declares_matte_together() {
    let rgba = image::RgbaImage::from_raw(
      4,
      1,
      vec![
        200, 100, 50, 255, 200, 100, 50, 128, 200, 100, 50, 1, 200, 100, 50, 0,
      ],
    )
    .unwrap();
    let source = PdfRasterImage::from_dynamic_with_icc(
      DynamicImage::ImageRgba8(rgba.clone()),
      Some(vec![1, 2, 3]),
    );
    let shared_pixels = source.pixels.clone();
    let office = prepare_metafile_preview(source, true).unwrap();
    let expected = PdfRasterImage::from_dynamic_with_icc(
      DynamicImage::ImageRgba8(apply_black_matte(&rgba)),
      Some(vec![1, 2, 3]),
    );
    assert_sampled_image(&office, &expected, false);
    assert_eq!(office.direct().matte, Some([0.0; 3]));
    assert!(!office.direct().soft_mask_interpolate);
    assert_eq!(shared_pixels.rgb[3..6], [200, 100, 50]);
    assert_eq!(office.direct().alpha(), Some([255, 128, 1, 0].as_slice()));

    let requested = prepare_metafile_preview(
      PdfRasterImage {
        pixels: shared_pixels,
      },
      false,
    )
    .unwrap();
    assert!(requested.direct().matte.is_none());
    let DirectRasterEncoding::Sampled { pixels } = &requested.direct().encoding else {
      panic!("metafile preview must remain sampled");
    };
    assert_eq!(pixels.rgb[3..6], [200, 100, 50]);
  }

  #[test]
  fn metafile_preview_binary_mask_retains_matte_but_opaque_preview_does_not() {
    for alpha in [vec![0, 255], vec![255, 255]] {
      let opaque = alpha.iter().all(|value| *value == 255);
      let source = PdfRasterImage {
        pixels: Arc::new(PdfRasterPixels {
          width: 2,
          height: 1,
          rgb: vec![0, 0, 0, 200, 100, 50],
          alpha: (!opaque).then_some(alpha),
          icc_profile: None,
        }),
      };
      let expected = source.clone();
      let actual = prepare_metafile_preview(source, true).unwrap();
      assert_sampled_image(&actual, &expected, false);
      assert_eq!(actual.direct().matte, (!opaque).then_some([0.0; 3]));
    }
  }

  fn indexed_test_png(unit: Option<png::Unit>, transparent: bool) -> Vec<u8> {
    const WIDTH: u32 = 300;
    const HEIGHT: u32 = 300;
    let mut encoded = Vec::new();
    {
      let mut encoder = png::Encoder::new(&mut encoded, WIDTH, HEIGHT);
      encoder.set_color(png::ColorType::Indexed);
      encoder.set_depth(png::BitDepth::One);
      encoder.set_palette(vec![20, 120, 40, 240, 20, 10]);
      if transparent {
        encoder.set_trns(vec![0, 255]);
      }
      encoder.set_pixel_dims(unit.map(|unit| png::PixelDimensions {
        xppu: 3_780,
        yppu: 3_780,
        unit,
      }));
      let mut writer = encoder.write_header().unwrap();
      let row_bytes = WIDTH.div_ceil(8) as usize;
      writer
        .write_image_data(&vec![0; row_bytes * HEIGHT as usize])
        .unwrap();
    }
    encoded
  }

  fn assert_sampled_image(
    actual: &PreparedRasterImage,
    expected: &PdfRasterImage,
    interpolate: bool,
  ) {
    let direct = actual.direct();
    assert_eq!(direct.width, expected.pixels.width);
    assert_eq!(direct.height, expected.pixels.height);
    assert_eq!(direct.color_space, DirectRasterColorSpace::Rgb);
    assert_eq!(direct.bits_per_component, 8);
    assert_eq!(direct.interpolate, interpolate);
    let DirectRasterEncoding::Sampled { pixels } = &direct.encoding else {
      panic!("expected sampled raster, got {:?}", direct.encoding);
    };
    assert_eq!(pixels.as_ref(), expected.pixels.as_ref());
  }

  fn assert_dct_image(actual: &PreparedRasterImage, expected: &[u8], interpolate: bool) {
    let direct = actual.direct();
    assert_eq!(direct.interpolate, interpolate);
    let DirectRasterEncoding::Dct { data, .. } = &direct.encoding else {
      panic!("expected DCT raster, got {:?}", direct.encoding);
    };
    assert_eq!(data.as_ref(), expected);
  }

  #[test]
  fn office_gray_jpeg_preserves_components_across_size_and_density() {
    for optimize_for in [PdfOptimizeFor::Print, PdfOptimizeFor::Screen] {
      let mut options = PdfOptions {
        optimize_for,
        ..PdfOptions::default()
      };
      options.images.optimization_policy =
        PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
      let export_options = RasterExportOptions::new(&options, 270.0, 270.0);
      for (width, height) in [(2, 2), (8, 8), (14, 22), (32, 32), (64, 64), (128, 128)] {
        let image =
          image::GrayImage::from_fn(width, height, |x, y| image::Luma([(x * 13 + y * 29) as u8]));
        let encoded = super::super::jpeg_islow_encoder::encode_gray(&image, 90).unwrap();
        for density_unit in [0, 1, 2] {
          let mut encoded = encoded.clone();
          encoded[13] = density_unit;
          let decoded = super::super::jpeg_islow::decode_gray(&encoded).unwrap();
          let quality = export_options.jpeg_quality.unwrap();
          let expected = super::super::jpeg_islow_encoder::encode_gray(&decoded, quality).unwrap();
          let actual = decode_image(&encoded, Some("image/jpeg"), export_options, None).unwrap();
          assert_eq!(actual.direct().color_space, DirectRasterColorSpace::Gray);
          assert_dct_image(&actual, &expected, true);
        }
      }
    }
  }

  #[test]
  fn tiff_content_types_route_to_the_enabled_decoder() {
    let pixels = [0_u8, 64, 128, 255];
    let mut encoded = Cursor::new(Vec::new());
    TiffEncoder::new(&mut encoded)
      .write_image(&pixels, 2, 2, ColorType::L8.into())
      .unwrap();
    let encoded = encoded.into_inner();

    assert_eq!(
      image_format_from_content_type("image/tif"),
      Some(RasterImageFormat::Tiff)
    );
    assert_eq!(
      image_format_from_content_type("image/tiff"),
      Some(RasterImageFormat::Tiff)
    );
    let decoded = decode_dynamic_image(&encoded, RasterImageFormat::Tiff).unwrap();
    assert_eq!(decoded.image.to_luma8().as_raw(), &pixels);
  }

  #[test]
  fn raster_signature_precedes_a_conflicting_package_media_type() {
    let source = image::RgbImage::from_fn(8, 8, |x, y| {
      image::Rgb([(x * 31) as u8, (y * 31) as u8, ((x + y) * 15) as u8])
    });
    let mut jpeg = Vec::new();
    ImageJpegEncoder::new(&mut jpeg)
      .write_image(source.as_raw(), 8, 8, ColorType::Rgb8.into())
      .unwrap();
    let png = indexed_test_png(None, false);

    assert_eq!(
      raster_format(&jpeg, Some("image/png")),
      Some(RasterImageFormat::Jpeg)
    );
    assert_eq!(
      raster_format(&png, Some("image/jpeg")),
      Some(RasterImageFormat::Png)
    );

    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx);
    let decoded = decode_image(
      &jpeg,
      Some("image/png"),
      RasterExportOptions::new(&options, 72.0, 72.0),
      None,
    )
    .unwrap();
    assert_eq!((decoded.direct().width, decoded.direct().height), (8, 8));
  }

  #[test]
  fn raster_media_type_remains_the_fallback_when_no_signature_matches() {
    let unrecognized = b"not an image signature";
    assert_eq!(
      raster_format(unrecognized, Some("image/png")),
      Some(RasterImageFormat::Png)
    );
    assert_eq!(raster_format(unrecognized, None), None);

    let error = match decode_dynamic_image(unrecognized, RasterImageFormat::Png) {
      Ok(_) => panic!("unrecognized bytes unexpectedly decoded as PNG"),
      Err(error) => error.to_string(),
    };
    assert!(error.contains("Png"), "unexpected decoder error: {error}");
  }

  #[test]
  fn archival_raster_options_disable_pdf_image_interpolation() {
    let ordinary = RasterExportOptions::new(&PdfOptions::default(), 72.0, 72.0);
    assert!(ordinary.allow_interpolation);
    assert!(raster_interpolation(RasterImageFormat::Jpeg, ordinary));

    let mut archival = PdfOptions::default();
    archival.standards.push(crate::options::PdfStandard::PdfA3a);
    let archival = RasterExportOptions::new(&archival, 72.0, 72.0);
    assert!(!archival.allow_interpolation);
    assert!(!raster_interpolation(RasterImageFormat::Jpeg, archival));
  }

  #[test]
  fn requested_screen_intent_does_not_invent_an_image_resolution_cap() {
    let mut screen = PdfOptions {
      optimize_for: PdfOptimizeFor::Screen,
      ..Default::default()
    };
    let export = RasterExportOptions::new(&screen, 41.4, 25.68);
    assert!(export.max_size_px.is_none());

    screen.images.reduce_resolution = true;
    screen.images.max_resolution_dpi = Some(72);
    let export = RasterExportOptions::new(&screen, 41.4, 25.68);
    let (width, height) = export.max_size_px.unwrap().pixels();
    assert!((width - 41.4).abs() < 0.001);
    assert!((height - 25.68).abs() < 0.001);

    assert!(
      RasterExportOptions::new(&PdfOptions::default(), 41.4, 25.68)
        .max_size_px
        .is_none()
    );
  }

  #[test]
  fn microsoft_office_fixed_output_profile_owns_raster_quality_and_density() {
    let mut options = PdfOptions::default();
    options.images.use_lossless_compression = true;
    options.images.jpeg_quality = Some(90);
    options.images.reduce_resolution = true;
    options.images.max_resolution_dpi = Some(96);
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);

    let print = RasterExportOptions::new(&options, 72.0, 72.0);
    assert!(!print.use_lossless_compression);
    assert_eq!(print.jpeg_quality, Some(75));
    assert_eq!(print.max_size_px.unwrap().pixels(), (199.0, 199.0));
    assert_eq!(print.downsample_size((295, 295)), None);
    assert_eq!(print.downsample_size((300, 300)), Some((199, 199)));
    assert!(!print.use_gdiplus_jpeg_resampler);
    let print_jpeg = print.for_raster_format(RasterImageFormat::Jpeg);
    assert!(print_jpeg.use_gdiplus_jpeg_resampler);
    let (jpeg_width, jpeg_height) = print_jpeg.max_size_px.unwrap().pixels();
    assert_eq!((jpeg_width, jpeg_height), (199.0, 199.0));
    assert_eq!(print_jpeg.downsample_size((274, 274)), None);
    assert_eq!(print_jpeg.downsample_size((275, 275)), Some((199, 199)));

    options.optimize_for = PdfOptimizeFor::Screen;
    let screen = RasterExportOptions::new(&options, 72.0, 72.0);
    assert_eq!(screen.jpeg_quality, Some(59));
    assert_eq!(screen.max_size_px.unwrap().pixels(), (95.0, 95.0));
    assert_eq!(screen.downsample_size((144, 144)), None);
    assert_eq!(screen.downsample_size((150, 150)), Some((95, 95)));
    let screen_jpeg = screen.for_raster_format(RasterImageFormat::Jpeg);
    assert!(screen_jpeg.use_gdiplus_jpeg_resampler);
    assert_eq!(screen_jpeg.downsample_size((149, 149)), None);
    assert_eq!(screen_jpeg.downsample_size((150, 150)), Some((95, 95)));

    let frame_surface = RasterExportOptions::new(&options, 103.75, 19.0);
    assert_eq!(frame_surface.downsample_size((316, 58)), Some((138, 25)));

    options.optimize_for = PdfOptimizeFor::Print;
    for document_kind in [PdfDocumentKind::Xlsx, PdfDocumentKind::Pptx] {
      options.images.optimization_policy =
        PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(document_kind);
      let print =
        RasterExportOptions::new(&options, 72.0, 72.0).for_raster_format(RasterImageFormat::Jpeg);
      assert_eq!(print.jpeg_quality, Some(75));
      assert_eq!(print.max_size_px.unwrap().pixels(), (199.0, 199.0));
      assert!(!print.use_gdiplus_jpeg_resampler);
    }

    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx);
    options.standards.push(crate::options::PdfStandard::PdfA3a);
    let archival_print = RasterExportOptions::new(&options, 72.0, 72.0);
    assert_eq!(archival_print.jpeg_quality, Some(90));
    assert!(!archival_print.allow_interpolation);

    options.optimize_for = PdfOptimizeFor::Screen;
    let archival_screen = RasterExportOptions::new(&options, 72.0, 72.0);
    assert_eq!(archival_screen.jpeg_quality, Some(59));
    assert!(!archival_screen.allow_interpolation);
  }

  #[test]
  fn office_fixed_output_classifies_ordinary_sources_by_sourced_histogram_gates() {
    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let export_options = RasterExportOptions::new(&options, 72.0, 72.0);

    let palette = image::RgbaImage::from_fn(320, 292, |x, y| {
      Rgba([(x % 16) as u8, (y % 16) as u8, 0, 255])
    });
    let palette_metrics = office_rgb_histogram_metrics(&palette);
    assert_eq!(palette_metrics.top_256_color_samples, 320 * 292);
    assert!(office_fixed_output_prefers_lossless(
      &palette,
      export_options,
      RasterOwner::Source,
      false,
    ));

    let line_art = |width, height| {
      image::RgbaImage::from_fn(width, height, |x, y| {
        let index = x + y * width;
        let value = index.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let red = if index % 10 == 0 { value as u8 } else { 32 };
        let green = if (index / 10) % 10 == 0 {
          (value >> 8) as u8
        } else {
          96
        };
        Rgba([red, green, (value >> 16) as u8, 255])
      })
    };
    let small_line_art = line_art(154, 141);
    let small_metrics = office_rgb_histogram_metrics(&small_line_art);
    assert!(small_metrics.top_256_color_samples * 100 < small_metrics.sample_count * 95);
    assert!(small_metrics.per_channel_top_10_samples * 5 >= small_metrics.sample_count * 9);
    assert!(office_fixed_output_prefers_lossless(
      &small_line_art,
      export_options,
      RasterOwner::MaterializedSourceRectangleCrop,
      false,
    ));

    let large_line_art = line_art(155, 142);
    assert_eq!(154_u64 * 141 * 3, 65_142);
    assert_eq!(155_u64 * 142 * 3, 66_030);
    assert!(!office_fixed_output_prefers_lossless(
      &large_line_art,
      export_options,
      RasterOwner::Source,
      false,
    ));
    assert!(!office_fixed_output_prefers_lossless(
      &small_line_art,
      export_options,
      RasterOwner::Source,
      true,
    ));

    options.optimize_for = PdfOptimizeFor::Screen;
    let screen = RasterExportOptions::new(&options, 72.0, 72.0);
    assert!(!office_fixed_output_prefers_lossless(
      &small_line_art,
      screen,
      RasterOwner::Source,
      false,
    ));

    let requested = RasterExportOptions {
      profile: RasterExportProfile::Requested,
      ..export_options
    };
    assert!(!office_fixed_output_prefers_lossless(
      &small_line_art,
      requested,
      RasterOwner::Source,
      false,
    ));
    assert!(!office_fixed_output_prefers_lossless(
      &small_line_art,
      export_options,
      RasterOwner::MetafilePreview,
      false,
    ));
  }

  #[test]
  fn microsoft_office_bitmap_targets_quantize_twips_and_count_inclusive_endpoints() {
    let pixels = |points, dpi| {
      RasterPixelLimits::from_office_fixed_output_display_size(points, points, dpi)
        .unwrap()
        .pixels()
        .0
    };

    assert_eq!(pixels(35.64, 200), 98.0);
    assert_eq!(pixels(35.676, 200), 99.0);
    assert_eq!(pixels(35.82, 200), 99.0);
    assert_eq!(pixels(35.964, 200), 99.0);
    assert_eq!(pixels(36.0, 200), 99.0);
    assert_eq!(pixels(36.036, 200), 99.0);
    assert_eq!(pixels(35.25, 96), 46.0);
    assert_eq!(pixels(35.325, 96), 47.0);
    assert_eq!(pixels(35.625, 96), 47.0);
    assert_eq!(pixels(35.925, 96), 47.0);
    assert_eq!(pixels(36.0, 96), 47.0);
    assert_eq!(pixels(36.075, 96), 48.0);
    assert_eq!(pixels(60.0, 200), 166.0);
  }

  #[test]
  fn microsoft_office_fixed_output_converts_only_decoded_raster_owners() {
    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let print = RasterExportOptions::new(&options, 72.0, 72.0);

    assert!(should_try_jpeg(
      RasterImageFormat::Png,
      print,
      RasterOwner::Source,
    ));
    assert!(should_try_jpeg(
      RasterImageFormat::Gif,
      print,
      RasterOwner::Source,
    ));
    assert!(should_try_jpeg(
      RasterImageFormat::Png,
      print,
      RasterOwner::MaterializedSourceRectangleCrop,
    ));
    assert!(!should_try_jpeg(
      RasterImageFormat::Png,
      print,
      RasterOwner::MetafilePreview,
    ));
    assert!(should_try_jpeg(
      RasterImageFormat::Jpeg,
      print,
      RasterOwner::MetafilePreview,
    ));

    let requested = RasterExportOptions {
      profile: RasterExportProfile::Requested,
      ..print
    };
    assert!(should_try_jpeg(
      RasterImageFormat::Png,
      requested,
      RasterOwner::Source,
    ));
  }

  #[test]
  fn office_native_png_plan_requires_opaque_indexed_pixels_and_metric_phys() {
    let physical = indexed_test_png(Some(png::Unit::Meter), false);
    let unspecified = indexed_test_png(Some(png::Unit::Unspecified), false);
    let absent = indexed_test_png(None, false);
    let transparent = indexed_test_png(Some(png::Unit::Meter), true);

    let physical_metadata = raster_metadata(&physical, RasterImageFormat::Png)
      .unwrap()
      .unwrap();
    assert_eq!(
      physical_metadata.png,
      Some(PngMetadata {
        color_type: png::ColorType::Indexed,
        bit_depth: png::BitDepth::One,
        has_transparency: false,
        has_real_physical_resolution: true,
      })
    );

    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx);
    let export_options = RasterExportOptions::new(&options, 72.0, 72.0);
    let plan = |data: &[u8]| {
      RasterPlan::new(
        RasterImageFormat::Png,
        raster_metadata(data, RasterImageFormat::Png).unwrap(),
        RasterOwner::Source,
        export_options,
      )
      .realization
    };

    // A 300x300 source reaches the Print trigger and would normally reduce to
    // the 199x199 inclusive-endpoint surface. Real-DPI indexed input is the
    // independently proven native exception.
    assert_eq!(plan(&physical), RasterRealization::NativePng);
    assert_eq!(plan(&unspecified), RasterRealization::Decoded);
    assert_eq!(plan(&absent), RasterRealization::Decoded);
    assert_eq!(plan(&transparent), RasterRealization::Decoded);

    let actual = decode_image(&physical, Some("image/png"), export_options, None).unwrap();
    let expected = NativeIndexedPng::parse(&physical).unwrap();
    let direct = actual.direct();
    assert_eq!(
      (direct.width, direct.height),
      (expected.width, expected.height)
    );
    assert_eq!(direct.bits_per_component, expected.bit_depth as u8);
    assert!(!direct.interpolate);
    let DirectRasterEncoding::IndexedPng { data, palette, .. } = &direct.encoding else {
      panic!("expected indexed PNG, got {:?}", direct.encoding);
    };
    assert_eq!(data.as_ref(), expected.idat.as_slice());
    assert_eq!(palette.as_ref(), expected.palette.as_slice());
  }

  #[test]
  fn microsoft_word_fixed_output_preserves_unscaled_binary_alpha_png_losslessly() {
    let source = image::RgbaImage::from_fn(64, 64, |x, y| {
      if (12..52).contains(&x) && (12..52).contains(&y) {
        Rgba([20, 120, 40, 255])
      } else {
        Rgba([255, 255, 255, 0])
      }
    });
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 64, 64, ColorType::Rgba8.into())
      .unwrap();
    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let export_options = RasterExportOptions::new(&options, 72.0, 72.0);

    assert_eq!(export_options.downsample_size((64, 64)), None);
    let actual = decode_image(&png, Some("image/png"), export_options, None).unwrap();
    let expected = PdfRasterImage::from_dynamic_with_icc(
      DynamicImage::ImageRgba8(apply_black_matte(&source)),
      None,
    );
    assert_sampled_image(&actual, &expected, false);
    assert!(!actual.direct().soft_mask_interpolate);
    assert_eq!(actual.direct().matte, Some([0.0, 0.0, 0.0]));
  }

  #[test]
  fn microsoft_word_fixed_output_uses_the_shared_office_h2v2_jpeg_owner() {
    let source = image::RgbaImage::from_fn(64, 64, |x, y| {
      let value = (x + y * 64)
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223);
      Rgba([value as u8, (value >> 8) as u8, (value >> 16) as u8, 255])
    });
    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let export_options =
      RasterExportOptions::new(&options, 72.0, 72.0).for_raster_format(RasterImageFormat::Jpeg);
    let actual = export_decoded_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source.clone()),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      RasterImageFormat::Jpeg,
      export_options,
      RasterOwner::Source,
    )
    .unwrap();
    let expected = encode_office_h2v2_jpeg(&source, 75).unwrap();
    assert_dct_image(&actual, &expected, true);
  }

  #[test]
  fn office_screen_reduction_uses_the_gdiplus_bilinear_phase() {
    let vertical = image::RgbaImage::from_fn(116, 72, |x, _| {
      if x == 50 {
        Rgba([238, 238, 238, 255])
      } else {
        Rgba([238, 238, 238, 0])
      }
    });
    let reduced = resize_for_office_screen(DynamicImage::ImageRgba8(vertical), (55, 34)).to_rgba8();
    assert_eq!(reduced.get_pixel(24, 17)[3], 97);
    assert_eq!(reduced.get_pixel(23, 17)[3], 0);
    assert_eq!(reduced.get_pixel(25, 17)[3], 0);

    let horizontal = image::RgbaImage::from_fn(116, 72, |_, y| {
      if y == 30 {
        Rgba([238, 238, 238, 255])
      } else {
        Rgba([238, 238, 238, 0])
      }
    });
    let reduced =
      resize_for_office_screen(DynamicImage::ImageRgba8(horizontal), (55, 34)).to_rgba8();
    assert_eq!(reduced.get_pixel(27, 14)[3], 165);
    assert_eq!(reduced.get_pixel(27, 13)[3], 0);
    assert_eq!(reduced.get_pixel(27, 15)[3], 0);
  }

  #[test]
  fn word_screen_png_area_boundary_preserves_opaque_color_and_hidden_rgb() {
    for (width, height) in [(419, 270), (420, 270), (270, 419), (270, 420)] {
      let source = DynamicImage::ImageRgba8(image::RgbaImage::from_fn(width, height, |x, y| {
        Rgba([
          (x * 17 + y * 5) as u8,
          (x * 3 + y * 19) as u8,
          (x * 11 + y * 7) as u8,
          255,
        ])
      }));
      let expected = if width.max(height) < GDIPLUS_BITMAP_AREA_THRESHOLD {
        resize_for_office_screen(source.clone(), (55, 34))
      } else {
        resize_for_gdiplus_fixed_area(source.clone(), (55, 34))
      };
      assert_eq!(
        resize_for_word_screen_png(source, (55, 34)).to_rgba8(),
        expected.to_rgba8()
      );
    }
    for hidden in [[0, 0, 0, 0], [0, 0, 255, 0], [255, 255, 255, 0]] {
      let source = image::RgbaImage::from_fn(420, 2, |x, _| {
        if x % 2 == 0 {
          Rgba([200, 0, 0, 255])
        } else {
          Rgba(hidden)
        }
      });
      let resized =
        resize_for_word_screen_png(DynamicImage::ImageRgba8(source), (210, 1)).to_rgba8();
      assert!(resized.pixels().all(|pixel| pixel.0 == [199, 0, 0, 128]));
      assert!(
        apply_black_matte(&resized)
          .pixels()
          .all(|pixel| pixel.0 == [100, 0, 0, 128])
      );
    }
  }

  #[test]
  fn power_point_screen_source_density_triggers_each_axis_independently() {
    let mut options = PdfOptions {
      optimize_for: PdfOptimizeFor::Screen,
      ..Default::default()
    };
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx);
    for format in [RasterImageFormat::Png, RasterImageFormat::Jpeg] {
      for axis in [0, 1] {
        for (dpi, reduced) in [
          (149.0, None),
          (149.987, None),
          (150.0, Some((558, 303))),
          (150.1, Some((557, 303))),
          (150.5, Some((556, 302))),
          (151.0, Some((554, 301))),
        ] {
          let mut density = [149.0, 149.0];
          density[axis] = dpi;
          // Controls vary integer-EMU extents, not rounded pixel dimensions.
          let points = |pixels: f64, dpi: f64| ((pixels * 914400.0 / dpi).round() / 12700.0) as f32;
          let export = RasterExportOptions::new(
            &options,
            points(872.0, density[0]),
            points(475.0, density[1]),
          );
          let expected = reduced.map(|(width, height)| {
            if axis == 0 {
              (width, 475)
            } else {
              (872, height)
            }
          });
          assert_eq!(
            export.downsample_source_size((872, 475), format, RasterOwner::Source),
            expected,
            "{format:?}, axis {axis}, {dpi} DPI",
          );
        }
      }
    }
    for kind in [PdfDocumentKind::Docx, PdfDocumentKind::Xlsx] {
      options.images.optimization_policy =
        PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(kind);
      let export = RasterExportOptions::new(&options, 418.5963, 228.01976);
      assert_eq!(
        export.downsample_source_size((872, 475), RasterImageFormat::Png, RasterOwner::Source),
        export.downsample_size((872, 475)),
      );
    }
  }

  #[test]
  fn power_point_screen_bitmap_sampler_does_not_change_density_or_other_profiles() {
    for document_kind in [PdfDocumentKind::Pptx, PdfDocumentKind::Xlsx] {
      for optimize_for in [PdfOptimizeFor::Screen, PdfOptimizeFor::Print] {
        let mut options = PdfOptions {
          optimize_for,
          ..Default::default()
        };
        options.images.optimization_policy =
          PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(document_kind);
        let base = RasterExportOptions::new(&options, 120.0, 90.0);
        let jpeg = base.for_raster_format(RasterImageFormat::Jpeg);
        assert_eq!(
          jpeg.use_gdiplus_jpeg_resampler,
          document_kind == PdfDocumentKind::Pptx && optimize_for == PdfOptimizeFor::Screen
        );
        assert_eq!(jpeg.max_size_px, base.max_size_px);
        assert_eq!(jpeg.downsample_trigger_px, base.downsample_trigger_px);
        assert_eq!(jpeg.exact_downsample_trigger, base.exact_downsample_trigger);
        assert!(!jpeg.without_downsampling().use_gdiplus_jpeg_resampler);
      }
    }
  }

  #[test]
  fn power_point_screen_opaque_png_uses_bitmap_area_on_either_axis() {
    let mut options = PdfOptions {
      optimize_for: PdfOptimizeFor::Screen,
      ..Default::default()
    };
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx);
    let export = RasterExportOptions::new(&options, 42.0, 26.25);
    for (width, height) in [
      (419, 317),
      (420, 317),
      (421, 317),
      (317, 419),
      (317, 420),
      (317, 421),
    ] {
      let source = image::RgbaImage::from_fn(width, height, |x, y| {
        Rgba([
          (x * 17 + y * 5) as u8,
          (x * 3 + y * 19) as u8,
          (x * 11 + y * 7) as u8,
          255,
        ])
      });
      // Both original RGB PNG and opaque RGBA from blip effects select this
      // realization. Alpha-bearing PowerPoint sources are not reclassified.
      for dynamic in [
        DynamicImage::ImageRgba8(source.clone()),
        DynamicImage::ImageRgb8(DynamicImage::ImageRgba8(source.clone()).to_rgb8()),
      ] {
        let mut png = Vec::new();
        dynamic
          .write_to(&mut Cursor::new(&mut png), RasterImageFormat::Png)
          .unwrap();
        let actual = decode_image(&png, Some("image/png"), export, None).unwrap();
        let expected = if width.max(height) < GDIPLUS_BITMAP_AREA_THRESHOLD {
          resize_for_office_screen(dynamic, (55, 34))
        } else {
          resize_for_gdiplus_fixed_area(dynamic, (55, 34))
        };
        let jpeg = encode_office_h2v2_jpeg(&expected.to_rgba8(), 59).unwrap();
        assert_dct_image(&actual, &jpeg, true);
      }
    }
  }

  #[test]
  fn word_screen_png_reduction_preserves_dct_and_independent_soft_mask() {
    let source = image::RgbaImage::from_fn(420, 270, |x, y| {
      Rgba([
        (x * 17 + y * 5) as u8,
        (x * 3 + y * 19) as u8,
        (x * 11 + y * 7) as u8,
        (x * 7 + y * 3) as u8,
      ])
    });
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 420, 270, ColorType::Rgba8.into())
      .unwrap();
    let mut options = PdfOptions {
      optimize_for: PdfOptimizeFor::Screen,
      ..PdfOptions::default()
    };
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let export_options = RasterExportOptions::new(&options, 42.0, 26.25);
    let actual = decode_image(&png, Some("image/png"), export_options, None).unwrap();
    let sampled = resize_for_word_screen_png(DynamicImage::ImageRgba8(source), (55, 34)).to_rgba8();
    let expected = encode_office_h2v2_jpeg(
      &apply_black_matte(&sampled),
      export_options.jpeg_quality.unwrap(),
    )
    .unwrap();
    assert_dct_image(&actual, &expected, true);
    assert_eq!(actual.direct().matte, Some([0.0, 0.0, 0.0]));
    assert!(!actual.direct().soft_mask_interpolate);
    let DirectRasterEncoding::Dct {
      alpha: Some(alpha), ..
    } = &actual.direct().encoding
    else {
      panic!("expected the independently sampled mask");
    };
    assert_eq!(
      alpha.as_ref(),
      sampled.pixels().map(|pixel| pixel[3]).collect::<Vec<_>>()
    );
  }

  #[test]
  fn powerpoint_screen_png_preserves_dct_and_independent_soft_mask() {
    let source = image::RgbaImage::from_fn(420, 270, |x, y| {
      Rgba([
        (x * 17 + y * 5) as u8,
        (x * 3 + y * 19) as u8,
        (x * 11 + y * 7) as u8,
        (x * 7 + y * 3) as u8,
      ])
    });
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 420, 270, ColorType::Rgba8.into())
      .unwrap();
    let mut options = PdfOptions {
      optimize_for: PdfOptimizeFor::Screen,
      ..PdfOptions::default()
    };
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx);
    for (width_pt, height_pt) in [(420.0, 270.0), (42.0, 26.25)] {
      let export_options = RasterExportOptions::new(&options, width_pt, height_pt);
      let actual = decode_image(&png, Some("image/png"), export_options, None).unwrap();
      let sampled = match export_options.downsample_size(source.dimensions()) {
        Some(size) => resize_for_export(
          DynamicImage::ImageRgba8(source.clone()),
          size,
          export_options,
        )
        .to_rgba8(),
        None => source.clone(),
      };
      let expected = encode_office_h2v2_jpeg(
        &apply_black_matte(&sampled),
        export_options.jpeg_quality.unwrap(),
      )
      .unwrap();
      assert_dct_image(&actual, &expected, true);
      assert_eq!(actual.direct().matte, Some([0.0; 3]));
      assert!(!actual.direct().soft_mask_interpolate);
      let DirectRasterEncoding::Dct {
        alpha: Some(alpha), ..
      } = &actual.direct().encoding
      else {
        panic!("expected the independently sampled mask");
      };
      assert_eq!(
        alpha.as_ref(),
        sampled.pixels().map(|pixel| pixel[3]).collect::<Vec<_>>()
      );
    }
  }

  #[test]
  fn word_print_jpeg_large_bitmap_scaler_matches_gdiplus_q16_area_coverage() {
    let mut source = image::RgbaImage::from_pixel(443, 1, Rgba([0, 0, 0, 255]));
    source.put_pixel(100, 0, Rgba([255, 0, 0, 255]));
    source.put_pixel(101, 0, Rgba([0, 255, 0, 255]));
    source.put_pixel(102, 0, Rgba([0, 0, 255, 255]));

    let reduced = resize_for_gdiplus_bitmap(DynamicImage::ImageRgba8(source), (315, 1)).to_rgba8();
    let non_black = reduced
      .enumerate_pixels()
      .filter(|(_, _, pixel)| pixel.0[..3] != [0, 0, 0])
      .map(|(x, y, pixel)| (x, y, pixel.0))
      .collect::<Vec<_>>();
    assert_eq!(
      non_black,
      vec![
        (71, 0, [181, 47, 0, 255]),
        (72, 0, [0, 135, 120, 255]),
        (73, 0, [0, 0, 61, 255]),
      ]
    );
    assert!(reduced.pixels().all(|pixel| pixel[3] == 255));
  }

  #[test]
  fn word_print_jpeg_bitmap_scaler_switches_on_either_420_pixel_axis() {
    let target = (298, 1);
    let below = DynamicImage::ImageRgba8(image::RgbaImage::from_fn(419, 1, |x, _| {
      Rgba([(x & 1) as u8 * 255, (x % 7) as u8 * 31, 19, 255])
    }));
    assert_eq!(
      resize_for_gdiplus_bitmap(below.clone(), target).to_rgba8(),
      resize_for_office_screen(below, target).to_rgba8()
    );

    let wide = DynamicImage::ImageRgba8(image::RgbaImage::from_fn(420, 1, |x, _| {
      Rgba([(x & 1) as u8 * 255, (x % 7) as u8 * 31, 19, 255])
    }));
    assert_eq!(
      resize_for_gdiplus_bitmap(wide.clone(), target).to_rgba8(),
      resize_for_gdiplus_fixed_area(wide, target).to_rgba8()
    );

    let tall = DynamicImage::ImageRgba8(image::RgbaImage::from_fn(1, 420, |_, y| {
      Rgba([(y & 1) as u8 * 255, (y % 7) as u8 * 31, 19, 255])
    }));
    assert_eq!(
      resize_for_gdiplus_bitmap(tall.clone(), (1, 298)).to_rgba8(),
      resize_for_gdiplus_fixed_area(tall, (1, 298)).to_rgba8()
    );
  }

  #[test]
  fn configured_jpeg_policy_reencodes_an_ordinary_opaque_png_color_plane() {
    let source = image::RgbImage::from_fn(39, 29, |x, y| {
      let value = (x + y * 39)
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223);
      image::Rgb([value as u8, (value >> 8) as u8, (value >> 16) as u8])
    });
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 39, 29, ColorType::Rgb8.into())
      .unwrap();
    let export_options = RasterExportOptions {
      use_lossless_compression: false,
      jpeg_quality: Some(60),
      max_size_px: None,
      downsample_trigger_px: None,
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: false,
      profile: RasterExportProfile::Requested,
    };

    assert!(raster_compression_change_requested(
      RasterImageFormat::Png,
      export_options,
      RasterOwner::Source,
    ));
    let actual = decode_image(&png, Some("image/png"), export_options, None).unwrap();
    let expected = encode_jpeg(&DynamicImage::ImageRgb8(source), 60).unwrap();
    assert_dct_image(&actual, &expected, false);

    let lossless = RasterExportOptions {
      use_lossless_compression: true,
      jpeg_quality: None,
      ..export_options
    };
    assert!(!raster_compression_change_requested(
      RasterImageFormat::Png,
      lossless,
      RasterOwner::Source,
    ));
  }

  #[test]
  fn configured_jpeg_policy_keeps_a_compressible_small_surface_lossless() {
    let source = image::RgbImage::from_pixel(55, 34, image::Rgb([238, 238, 238]));
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 55, 34, ColorType::Rgb8.into())
      .unwrap();
    let export_options = RasterExportOptions {
      use_lossless_compression: false,
      jpeg_quality: Some(75),
      max_size_px: None,
      downsample_trigger_px: None,
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: true,
      profile: RasterExportProfile::Requested,
    };

    let actual = decode_image(&png, Some("image/png"), export_options, None).unwrap();
    let expected = PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgb8(source), None);
    assert_sampled_image(&actual, &expected, false);
  }

  #[test]
  fn word_shape_story_jpeg_preserves_encoded_color_and_independent_soft_mask() {
    // Office testWPGtextboxes: RGB DCT object 15, A8 SMask object 16,
    // black Matte and no mask interpolation. Keep the WPG sampling contract
    // independent of how its completed surface is serialized.
    let source = image::RgbaImage::from_fn(39, 29, |x, y| {
      let value = (x + y * 39)
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223);
      Rgba([value as u8, (value >> 8) as u8, (value >> 16) as u8, 255])
    });
    let sampled = resample_wordprocessing_shape_story_bitmap(&source);
    let expected = encode_office_h2v2_jpeg(&apply_black_matte(&sampled), 60).unwrap();
    assert!(expected.len() < sampled.as_raw().len() / 4 * 3);
    let alpha = sampled.pixels().map(|pixel| pixel[3]).collect::<Vec<_>>();
    for interpolate in [false, true] {
      let options = RasterExportOptions {
        use_lossless_compression: false,
        jpeg_quality: Some(60),
        max_size_px: None,
        downsample_trigger_px: None,
        word_print_jpeg_downsample_trigger_px: None,
        exact_downsample_trigger: false,
        use_gdiplus_jpeg_resampler: false,
        allow_interpolation: interpolate,
        profile: RasterExportProfile::MicrosoftOfficeFixedOutput {
          document_kind: PdfDocumentKind::Docx,
          optimize_for: PdfOptimizeFor::Screen,
        },
      };
      let actual = export_wordprocessing_shape_story_image(
        DecodedRasterImage {
          image: DynamicImage::ImageRgba8(source.clone()),
          icc_profile: None,
          jpeg_has_real_physical_resolution: false,
        },
        options,
      )
      .unwrap();
      assert!(matches!(
        actual.direct().encoding,
        DirectRasterEncoding::Dct { .. }
      ));
      assert_dct_image(&actual, &expected, interpolate);
      assert_eq!(actual.direct().alpha(), Some(alpha.as_slice()));
      assert_eq!(actual.direct().matte, Some([0.0, 0.0, 0.0]));
      assert!(!actual.direct().soft_mask_interpolate);
    }
  }

  #[test]
  fn word_locked_canvas_device_resolves_associated_color_once() {
    let source = image::RgbaImage::from_fn(640, 320, |x, _| {
      if x % 2 == 0 {
        Rgba([255, 0, 0, 128])
      } else {
        Rgba([0, 0, 255, 0])
      }
    });
    let mut options = PdfOptions {
      optimize_for: PdfOptimizeFor::Screen,
      ..PdfOptions::default()
    };
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let options = RasterExportOptions {
      use_lossless_compression: true,
      // Isolate association/resolve from the separately tested Office
      // inclusive-endpoint conversion of a physical display extent.
      max_size_px: RasterPixelLimits::from_pixels(160.0, 80.0),
      ..RasterExportOptions::new(&options, 120.0, 60.0)
    };
    let actual = export_wordprocessing_locked_canvas_device_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      options,
    )
    .unwrap();
    assert_eq!(actual.size(), (160, 80));
    assert_eq!(actual.direct().matte, Some([0.0, 0.0, 0.0]));
    assert!(!actual.direct().interpolate);
    let DirectRasterEncoding::Sampled { pixels } = &actual.direct().encoding else {
      panic!("lossless canvas");
    };
    assert!(
      pixels
        .rgb
        .as_chunks::<3>()
        .0
        .iter()
        .all(|rgb| *rgb == [64, 0, 0])
    );
    assert!(
      actual
        .direct()
        .alpha()
        .unwrap()
        .iter()
        .all(|alpha| *alpha == 64)
    );
    assert_eq!(
      RasterOwner::from_content_type(Some(WORD_LOCKED_CANVAS_DEVICE_BITMAP_CONTENT_TYPE)),
      RasterOwner::WordLockedCanvasDevice
    );
  }

  #[test]
  fn word_locked_canvas_consumes_configured_downsampling_before_compression() {
    // The working canvas is 200 DPI; the fixed-output profile independently
    // selects its final size. Lossless compression must not bypass that step.
    let source = image::RgbaImage::from_pixel(383, 164, Rgba([30, 70, 110, 255]));
    for optimize_for in [PdfOptimizeFor::Screen, PdfOptimizeFor::Print] {
      let mut options = PdfOptions {
        optimize_for,
        ..PdfOptions::default()
      };
      options.images.optimization_policy =
        PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
      let base = RasterExportOptions::new(&options, 138.103, 59.35244);
      let expected_size = match optimize_for {
        PdfOptimizeFor::Screen => (184, 79),
        PdfOptimizeFor::Print => (383, 164),
      };
      assert_eq!(
        base
          .downsample_size(source.dimensions())
          .unwrap_or(source.dimensions()),
        expected_size
      );
      for use_lossless_compression in [false, true] {
        let actual = export_wordprocessing_locked_canvas_image(
          DecodedRasterImage {
            image: DynamicImage::ImageRgba8(source.clone()),
            icc_profile: None,
            jpeg_has_real_physical_resolution: false,
          },
          RasterExportOptions {
            use_lossless_compression,
            ..base
          },
        )
        .unwrap();
        assert_eq!(actual.size(), expected_size);
      }
    }
  }

  #[test]
  fn word_shape_story_bitmap_uses_the_wpg_subpixel_phase() {
    let source = image::RgbaImage::from_pixel(3, 3, Rgba([40, 80, 160, 255]));

    let sampled = resample_wordprocessing_shape_story_bitmap(&source);

    assert_eq!(sampled.get_pixel(0, 0).0, [40, 80, 160, 167]);
    assert_eq!(sampled.get_pixel(2, 0).0, [40, 80, 160, 191]);
    assert_eq!(sampled.get_pixel(0, 2).0, [40, 80, 160, 223]);
    assert_eq!(sampled.get_pixel(2, 2).0, [40, 80, 160, 255]);

    let hidden_color =
      image::RgbaImage::from_raw(2, 1, vec![255, 0, 0, 255, 0, 0, 255, 0]).unwrap();
    let sampled = resample_wordprocessing_shape_story_bitmap(&hidden_color);
    assert_eq!(sampled.get_pixel(1, 0).0, [255, 0, 0, 70]);
  }

  #[test]
  fn custom_raster_preserves_icc_profile() {
    let profile = vec![0_u8; 128];
    let image =
      PdfRasterImage::from_dynamic_with_icc(DynamicImage::new_rgb8(1, 1), Some(profile.clone()));

    assert_eq!(
      image.pixels.icc_profile.as_deref(),
      Some(profile.as_slice())
    );
  }

  #[test]
  fn office_reduced_jpeg_discards_pdf_icc_but_requested_export_preserves_it() {
    let source = image::RgbImage::from_fn(300, 300, |x, y| {
      let value = (x + y * 300)
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223);
      image::Rgb([value as u8, (value >> 8) as u8, (value >> 16) as u8])
    });
    let mut profile = vec![0_u8; 132];
    profile[..4].copy_from_slice(&132_u32.to_be_bytes());
    profile[16..20].copy_from_slice(b"RGB ");
    profile[36..40].copy_from_slice(b"acsp");
    let raster = || DecodedRasterImage {
      image: DynamicImage::ImageRgb8(source.clone()),
      icc_profile: Some(profile.clone()),
      jpeg_has_real_physical_resolution: false,
    };

    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let office =
      RasterExportOptions::new(&options, 72.0, 72.0).for_raster_format(RasterImageFormat::Jpeg);
    let reduced = export_decoded_image(
      raster(),
      RasterImageFormat::Jpeg,
      office,
      RasterOwner::Source,
    )
    .unwrap();
    assert_eq!(reduced.size(), (199, 199));
    let DirectRasterEncoding::Dct { icc_profile, .. } = &reduced.direct().encoding else {
      panic!("expected DCT raster, got {:?}", reduced.direct().encoding);
    };
    assert!(icc_profile.is_none());

    let requested = RasterExportOptions {
      use_lossless_compression: false,
      jpeg_quality: Some(75),
      max_size_px: None,
      downsample_trigger_px: None,
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: true,
      profile: RasterExportProfile::Requested,
    };
    let preserved = export_decoded_image(
      raster(),
      RasterImageFormat::Jpeg,
      requested,
      RasterOwner::Source,
    )
    .unwrap();
    let DirectRasterEncoding::Dct { icc_profile, .. } = &preserved.direct().encoding else {
      panic!("expected DCT raster, got {:?}", preserved.direct().encoding);
    };
    assert_eq!(icc_profile.as_deref(), Some(profile.as_slice()));
  }

  #[test]
  fn word_static_3d_black_matte_round_trip_preserves_alpha_contract() {
    let source = image::RgbaImage::from_raw(2, 1, vec![146, 208, 80, 92, 255, 127, 64, 0]).unwrap();

    let premultiplied = apply_black_matte(&source);
    assert_eq!(premultiplied.get_pixel(0, 0).0, [53, 75, 29, 92]);
    assert_eq!(premultiplied.get_pixel(1, 0).0, [0, 0, 0, 0]);

    let restored = remove_black_matte(
      image::RgbImage::from_fn(2, 1, |x, _| {
        let pixel = premultiplied.get_pixel(x, 0);
        image::Rgb([pixel[0], pixel[1], pixel[2]])
      }),
      &source,
    );
    assert_eq!(restored.get_pixel(0, 0).0, [147, 208, 80]);
    assert_eq!(restored.get_pixel(1, 0).0, [0, 0, 0]);
  }

  #[test]
  fn premultiplied_bilinear_sampling_clamps_each_source_neighbor() {
    let source = image::RgbaImage::from_raw(
      2,
      2,
      vec![
        255, 0, 0, 255, 0, 255, 0, 128, 0, 0, 255, 64, 255, 255, 255, 0,
      ],
    )
    .unwrap();
    // Interior interpolation stays premultiplied; transparent white contributes
    // no RGB. Clamping outside an edge must not pull in the next interior row.
    assert_eq!(
      bilinear_premultiplied_rgba_sample(&source, 0.5, 0.5),
      [63.75, 32.0, 16.0, 111.75]
    );
    let coordinates: [f64; 10] = [-2.75, -1.25, -0.75, -0.25, 0.0, 0.25, 0.75, 1.0, 1.25, 2.75];
    for x in coordinates {
      for y in coordinates {
        assert_eq!(
          bilinear_premultiplied_rgba_sample(&source, x, y),
          bilinear_premultiplied_rgba_sample(&source, x.clamp(0.0, 1.0), y.clamp(0.0, 1.0)),
          "source sample ({x}, {y})"
        );
      }
    }
  }

  #[test]
  fn word_static_3d_legacy_screen_size_is_retained_without_a_final_grid() {
    let mut options = PdfOptions {
      optimize_for: PdfOptimizeFor::Screen,
      ..PdfOptions::default()
    };
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let ordinary = RasterExportOptions::new(&options, 212.28, 133.68);
    assert_eq!(ordinary.max_size_px.unwrap().pixels(), (282.0, 178.0));

    let static_3d = ordinary.for_owner(RasterOwner::WordStatic3d, 212.28, 133.68);
    assert_eq!(static_3d.max_size_px.unwrap().pixels(), (285.0, 180.0));
    assert_eq!(static_3d.downsample_size((589, 371)), Some((285, 180)));
  }

  #[test]
  fn word_static_3d_final_grid_owns_allocation_and_effect_composition() {
    let mut options = PdfOptions {
      optimize_for: PdfOptimizeFor::Screen,
      ..PdfOptions::default()
    };
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
    let mut export_options =
      RasterExportOptions::new(&options, 3.0, 3.0).for_owner(RasterOwner::WordStatic3d, 3.0, 3.0);
    export_options.use_lossless_compression = true;
    // The compatibility estimate is 5x5, intentionally not the 3x2 grid.
    assert_eq!(export_options.max_size_px.unwrap().pixels(), (5.0, 5.0));
    let final_grid = image::RgbaImage::from_fn(3, 2, |x, y| {
      if x == 0 && y == 0 {
        Rgba([0, 0, 0, 0])
      } else {
        Rgba([20 + x as u8, 40 + y as u8, 60, 255])
      }
    });
    for source_size in [(12, 10), (3, 2), (1, 1)] {
      for with_effects in [false, true] {
        let source =
          image::RgbaImage::from_pixel(source_size.0, source_size.1, Rgba([255, 0, 0, 255]));
        let effect = Rgba([200, 100, 50, 128]);
        let layers = WordStatic3dLayers {
          physical: source.clone(),
          effects: with_effects
            .then(|| image::RgbaImage::from_pixel(source_size.0, source_size.1, effect)),
          physical_mapping: WordStatic3dPhysicalMapping::default(),
          final_grid: Some(final_grid.clone()),
          backdrop_grid: None,
        };
        // A contradictory explicit target is invalid, never permission to
        // discard final geometry and silently resolve the red source instead.
        assert!(resolve_wordprocessing_static_3d_screen_image(&layers, (5, 5)).is_none());
        let mut expected = final_grid.clone();
        if with_effects {
          expected.put_pixel(0, 0, effect);
        }
        assert_eq!(
          resolve_wordprocessing_static_3d_screen_image(&layers, (3, 2)).unwrap(),
          expected,
        );
        let actual = export_wordprocessing_static_3d_image(
          DecodedRasterImage {
            image: DynamicImage::ImageRgba8(source),
            icc_profile: None,
            jpeg_has_real_physical_resolution: false,
          },
          export_options,
          Some(layers),
        )
        .unwrap();
        let expected = PdfRasterImage::from_dynamic_with_icc(
          DynamicImage::ImageRgba8(apply_black_matte(&expected)),
          None,
        );
        assert_sampled_image(&actual, &expected, false);
      }
    }
  }

  #[test]
  fn word_static_3d_backdrop_grid_bypasses_legacy_resampling() {
    let backdrop = image::RgbaImage::from_raw(
      3,
      2,
      vec![
        200, 10, 30, 255, 0, 180, 80, 128, 30, 60, 90, 64, 0, 0, 0, 0, 90, 180, 0, 200, 60, 40, 20,
        255,
      ],
    )
    .unwrap();
    let mut layers = WordStatic3dLayers {
      physical: image::RgbaImage::from_pixel(12, 10, Rgba([255, 0, 0, 255])),
      effects: Some(image::RgbaImage::from_pixel(12, 10, Rgba([0, 0, 255, 255]))),
      physical_mapping: WordStatic3dPhysicalMapping::default(),
      final_grid: Some(image::RgbaImage::new(3, 2)),
      backdrop_grid: Some(backdrop.clone()),
    };
    assert_eq!(
      resolve_wordprocessing_static_3d_screen_image(&layers, (3, 2)).unwrap(),
      backdrop
    );
    assert!(resolve_wordprocessing_static_3d_screen_image(&layers, (3, 3)).is_none());
    layers.backdrop_grid = Some(image::RgbaImage::new(2, 3));
    assert!(resolve_wordprocessing_static_3d_screen_image(&layers, (3, 2)).is_none());
    layers.final_grid = None;
    assert!(resolve_wordprocessing_static_3d_screen_image(&layers, (2, 3)).is_none());
    layers.backdrop_grid = None;
    assert!(resolve_wordprocessing_static_3d_screen_image(&layers, (3, 2)).is_some());
  }

  #[test]
  fn word_static_3d_backdrop_transport_validates_both_final_planes() {
    let plane = |width, height, color| {
      let image = image::RgbaImage::from_pixel(width, height, Rgba(color));
      let mut bytes = Vec::new();
      PngEncoder::new(&mut bytes)
        .write_image(image.as_raw(), width, height, ColorType::Rgba8.into())
        .unwrap();
      bytes
    };
    let physical = plane(12, 10, [255, 0, 0, 255]);
    let solid = plane(3, 2, [0; 4]);
    let backdrop = plane(3, 2, [30, 80, 120, 160]);
    let wrong = plane(2, 3, [0; 4]);
    for (solid_chunk, backdrop_chunk, valid) in [
      (Some(solid.as_slice()), Some(backdrop.as_slice()), true),
      (Some(solid.as_slice()), None, true),
      (None, None, true),
      (None, Some(backdrop.as_slice()), false),
      (Some(wrong.as_slice()), Some(backdrop.as_slice()), false),
      (Some(solid.as_slice()), Some(&b"invalid png"[..]), false),
    ] {
      let mut bytes = Vec::new();
      {
        let mut encoder = png::Encoder::new(&mut bytes, 12, 10);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer
          .write_chunk(
            png::chunk::ChunkType(WORD_STATIC_3D_PHYSICAL_RGBA_CHUNK),
            &physical,
          )
          .unwrap();
        if let Some(chunk) = solid_chunk {
          writer
            .write_chunk(
              png::chunk::ChunkType(WORD_STATIC_3D_FINAL_GRID_RGBA_CHUNK),
              chunk,
            )
            .unwrap();
        }
        if let Some(chunk) = backdrop_chunk {
          writer
            .write_chunk(
              png::chunk::ChunkType(WORD_STATIC_3D_BACKDROP_GRID_RGBA_CHUNK),
              chunk,
            )
            .unwrap();
        }
        writer.write_image_data(&vec![0; 12 * 10 * 4]).unwrap();
      }
      let decoded = decode_wordprocessing_static_3d_layers(&bytes);
      assert_eq!(decoded.is_some(), valid);
      if valid && backdrop_chunk.is_some() {
        let result =
          resolve_wordprocessing_static_3d_screen_image(&decoded.unwrap(), (3, 2)).unwrap();
        assert!(result.pixels().all(|p| p.0 == [30, 80, 120, 160]));
      }
    }
  }

  #[test]
  fn word_static_3d_screen_final_grid_does_not_override_other_export_profiles() {
    let source = image::RgbaImage::from_pixel(3, 2, Rgba([255, 0, 0, 255]));
    for office_print in [false, true] {
      let mut options = PdfOptions::default();
      if office_print {
        options.images.optimization_policy =
          PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Docx);
      }
      let mut export_options = RasterExportOptions::new(&options, 72.0, 72.0).for_owner(
        RasterOwner::WordStatic3d,
        72.0,
        72.0,
      );
      export_options.use_lossless_compression = true;
      assert!(!export_options.profile.is_office_screen());
      assert!(
        export_options
          .downsample_size(source.dimensions())
          .is_none()
      );
      let actual = export_wordprocessing_static_3d_image(
        DecodedRasterImage {
          image: DynamicImage::ImageRgba8(source.clone()),
          icc_profile: None,
          jpeg_has_real_physical_resolution: false,
        },
        export_options,
        Some(WordStatic3dLayers {
          physical: source.clone(),
          effects: None,
          physical_mapping: WordStatic3dPhysicalMapping::default(),
          final_grid: Some(image::RgbaImage::from_pixel(1, 1, Rgba([0, 255, 0, 255]))),
          backdrop_grid: Some(image::RgbaImage::from_pixel(1, 1, Rgba([0, 0, 255, 255]))),
        }),
      )
      .unwrap();
      let expected =
        PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(source.clone()), None);
      assert_sampled_image(&actual, &expected, false);
    }
  }

  #[test]
  fn word_static_3d_screen_downsampling_owns_frame_axes_independently() {
    let source = image::RgbaImage::from_pixel(594, 380, Rgba([146, 208, 80, 192]));
    let export_options = RasterExportOptions {
      use_lossless_compression: true,
      jpeg_quality: None,
      max_size_px: RasterPixelLimits::from_pixels(285.0, 182.0),
      downsample_trigger_px: RasterPixelLimits::from_pixels(445.5, 285.0),
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: true,
      profile: RasterExportProfile::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Docx,
        optimize_for: PdfOptimizeFor::Screen,
      },
    };

    let actual = export_wordprocessing_static_3d_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      export_options,
      None,
    )
    .unwrap();

    assert_eq!((actual.direct().width, actual.direct().height), (285, 182));
  }

  #[test]
  fn black_matte_cpu_emulation_preserves_every_associated_sample() {
    for alpha in 0_u16..=255 {
      for straight in 0_u16..=255 {
        let matted = ((straight * alpha + 127) / 255) as u8;
        let restored = remove_black_matte_component(matted, alpha as u8);
        let rematted = ((u16::from(restored) * alpha + 127) / 255) as u8;

        assert_eq!(rematted, matted, "alpha={alpha}, straight={straight}");
      }
    }
  }

  #[test]
  fn word_static_3d_keeps_a_compressible_surface_lossless_and_uninterpolated() {
    let mut source = image::RgbaImage::from_pixel(55, 34, Rgba([238, 238, 238, 255]));
    source.put_pixel(0, 0, Rgba([238, 238, 238, 0]));
    let export_options = RasterExportOptions {
      use_lossless_compression: false,
      jpeg_quality: Some(75),
      max_size_px: None,
      downsample_trigger_px: None,
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: true,
      profile: RasterExportProfile::Requested,
    };

    let actual = export_wordprocessing_static_3d_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source.clone()),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      export_options,
      None,
    )
    .unwrap();
    let expected = PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(source), None);
    assert_sampled_image(&actual, &expected, false);
  }

  #[test]
  fn word_static_3d_keeps_small_complex_surface_lossless_before_stream_size() {
    let source = image::RgbaImage::from_fn(160, 120, |x, y| {
      let linear = x + y * 160;
      let mixed = linear.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
      Rgba([
        mixed as u8,
        (mixed >> 8) as u8,
        (mixed >> 16) as u8,
        if linear == 0 { 0 } else { 255 },
      ])
    });
    let premultiplied = apply_black_matte(&source);
    let metrics = office_rgb_histogram_metrics(&premultiplied);
    assert!(metrics.sample_count * 3 <= OFFICE_SMALL_RASTER_UNCOMPRESSED_RGB_BYTES);
    assert!(!office_top_256_palette_gate_from_metrics(metrics));
    assert!(!office_per_channel_top_10_gate_from_metrics(metrics));
    let jpeg = encode_office_h2v2_jpeg(&premultiplied, 75).unwrap();
    assert!(jpeg.len() < deflated_rgb_size(&premultiplied));

    let actual = export_wordprocessing_static_3d_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      RasterExportOptions {
        use_lossless_compression: false,
        jpeg_quality: Some(75),
        max_size_px: None,
        downsample_trigger_px: None,
        word_print_jpeg_downsample_trigger_px: None,
        exact_downsample_trigger: false,
        use_gdiplus_jpeg_resampler: false,
        allow_interpolation: true,
        profile: RasterExportProfile::MicrosoftOfficeFixedOutput {
          document_kind: PdfDocumentKind::Docx,
          optimize_for: PdfOptimizeFor::Print,
        },
      },
      None,
    )
    .unwrap();
    let expected =
      PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(premultiplied), None);
    assert_sampled_image(&actual, &expected, false);
  }

  #[test]
  fn word_static_3d_applies_the_large_surface_channel_histogram_gate() {
    let source = image::RgbaImage::from_fn(234, 118, |x, y| {
      let linear = x + y * 234;
      let mixed = linear.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
      let common = linear % 4 != 0;
      let channel = |value: u8| {
        if common { value % 10 * 25 } else { value }
      };
      Rgba([
        channel(mixed as u8),
        channel((mixed >> 8) as u8),
        channel((mixed >> 16) as u8),
        if linear == 0 { 0 } else { 255 },
      ])
    });
    let premultiplied = apply_black_matte(&source);
    let metrics = office_rgb_histogram_metrics(&premultiplied);
    assert!(metrics.sample_count * 3 > OFFICE_SMALL_RASTER_UNCOMPRESSED_RGB_BYTES);
    assert!(!office_top_256_palette_gate_from_metrics(metrics));
    assert!(office_per_channel_top_10_gate_from_metrics(metrics));
    let jpeg = encode_office_h2v2_jpeg(&premultiplied, 75).unwrap();
    assert!(jpeg.len() < deflated_rgb_size(&premultiplied));

    let actual = export_wordprocessing_static_3d_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      RasterExportOptions {
        use_lossless_compression: false,
        jpeg_quality: Some(75),
        max_size_px: None,
        downsample_trigger_px: None,
        word_print_jpeg_downsample_trigger_px: None,
        exact_downsample_trigger: false,
        use_gdiplus_jpeg_resampler: false,
        allow_interpolation: true,
        profile: RasterExportProfile::MicrosoftOfficeFixedOutput {
          document_kind: PdfDocumentKind::Docx,
          optimize_for: PdfOptimizeFor::Print,
        },
      },
      None,
    )
    .unwrap();
    let expected =
      PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(premultiplied), None);
    assert_sampled_image(&actual, &expected, false);
  }

  #[test]
  fn word_static_3d_color_classifier_excludes_fully_transparent_padding() {
    let source = image::RgbaImage::from_fn(227, 262, |x, y| {
      if x < 140 {
        return Rgba([193, 71, 29, 0]);
      }
      let linear = (x - 140) + y * 87;
      let mixed = linear.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
      Rgba([mixed as u8, (mixed >> 8) as u8, (mixed >> 16) as u8, 255])
    });
    let premultiplied = apply_black_matte(&source);
    let complete_metrics = office_rgb_histogram_metrics(&premultiplied);
    let visible_metrics = office_rgb_histogram_metrics_from_pixels(
      premultiplied
        .pixels()
        .filter(|pixel| pixel[3] != 0)
        .map(|pixel| [pixel[0], pixel[1], pixel[2]]),
    );

    assert!(complete_metrics.sample_count * 3 > OFFICE_SMALL_RASTER_UNCOMPRESSED_RGB_BYTES);
    assert!(office_per_channel_top_10_gate_from_metrics(
      complete_metrics
    ));
    assert!(!office_per_channel_top_10_gate_from_metrics(
      visible_metrics
    ));
    assert!(!office_static_3d_prefers_lossless(&premultiplied));

    let export_options = RasterExportOptions {
      use_lossless_compression: false,
      jpeg_quality: Some(75),
      max_size_px: None,
      downsample_trigger_px: None,
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: true,
      profile: RasterExportProfile::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Docx,
        optimize_for: PdfOptimizeFor::Print,
      },
    };
    let actual = export_wordprocessing_static_3d_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source.clone()),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      export_options,
      None,
    )
    .unwrap();
    let jpeg = encode_office_h2v2_jpeg(&premultiplied, 75).unwrap();
    assert!(jpeg.len() < deflated_rgb_size(&premultiplied));
    assert_dct_image(&actual, &jpeg, true);
    assert_eq!(
      actual.direct().alpha(),
      Some(
        source
          .pixels()
          .map(|pixel| pixel[3])
          .collect::<Vec<_>>()
          .as_slice()
      )
    );
    assert!(!actual.direct().soft_mask_interpolate);
    assert_eq!(actual.direct().matte, Some([0.0, 0.0, 0.0]));
  }

  #[test]
  fn word_static_3d_applies_the_office_palette_gate_before_stream_size() {
    let source = image::RgbaImage::from_fn(431, 121, |x, y| {
      let linear = x + y * 431;
      let mixed = linear.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
      let index = (mixed ^ (mixed >> 16)) as u8;
      Rgba([index, index.wrapping_mul(73), index.wrapping_mul(151), 255])
    });
    let metrics = office_rgb_histogram_metrics(&source);
    assert_eq!(metrics.top_256_color_samples, metrics.sample_count);
    let jpeg = encode_office_h2v2_jpeg(&source, 75).unwrap();
    assert!(jpeg.len() < deflated_rgb_size(&source));
    let export_options = RasterExportOptions {
      use_lossless_compression: false,
      jpeg_quality: Some(75),
      max_size_px: None,
      downsample_trigger_px: None,
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: true,
      profile: RasterExportProfile::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Docx,
        optimize_for: PdfOptimizeFor::Print,
      },
    };

    let actual = export_wordprocessing_static_3d_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source.clone()),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      export_options,
      None,
    )
    .unwrap();
    let expected = PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(source), None);
    assert_sampled_image(&actual, &expected, false);
  }

  #[test]
  fn word_static_3d_uses_interpolated_jpeg_for_a_complex_color_surface() {
    let source = image::RgbaImage::from_fn(160, 159, |x, y| {
      let value = (x + y * 160)
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223);
      Rgba([
        value as u8,
        (value >> 8) as u8,
        (value >> 16) as u8,
        64 + (value >> 24) as u8 / 2,
      ])
    });
    let premultiplied = apply_black_matte(&source);
    assert!(!office_static_3d_prefers_lossless(&premultiplied));
    let export_options = RasterExportOptions {
      use_lossless_compression: false,
      jpeg_quality: Some(75),
      max_size_px: None,
      downsample_trigger_px: None,
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: true,
      profile: RasterExportProfile::MicrosoftOfficeFixedOutput {
        document_kind: PdfDocumentKind::Docx,
        optimize_for: PdfOptimizeFor::Print,
      },
    };

    let actual = export_wordprocessing_static_3d_image(
      DecodedRasterImage {
        image: DynamicImage::ImageRgba8(source.clone()),
        icc_profile: None,
        jpeg_has_real_physical_resolution: false,
      },
      export_options,
      None,
    )
    .unwrap();
    let jpeg = encode_office_h2v2_jpeg(&premultiplied, 75).unwrap();
    assert!(jpeg.len() < deflated_rgb_size(&premultiplied));
    assert_dct_image(&actual, &jpeg, true);
    assert_eq!(
      actual.direct().alpha(),
      Some(
        source
          .pixels()
          .map(|pixel| pixel[3])
          .collect::<Vec<_>>()
          .as_slice()
      )
    );
    assert!(!actual.direct().soft_mask_interpolate);
    assert_eq!(actual.direct().matte, Some([0.0, 0.0, 0.0]));
  }

  #[test]
  fn powerpoint_activex_jpeg_round_trip_preserves_binary_alpha() {
    let source = [
      240, 20, 10, 255, // opaque red
      10, 220, 30, 0, // transparent green
    ];
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(&source, 2, 1, ColorType::Rgba8.into())
      .unwrap();

    let output = powerpoint_activex_bitmap_png(&png, 75).unwrap();
    let output = image::load_from_memory_with_format(&output, RasterImageFormat::Png)
      .unwrap()
      .to_rgba8();

    assert_eq!(output.dimensions(), (2, 1));
    assert_eq!(output.get_pixel(0, 0)[3], 255);
    assert_eq!(output.get_pixel(1, 0)[3], 0);
  }

  #[test]
  fn powerpoint_static_3d_preserves_scene_grid_and_independent_jpeg_alpha() {
    let source = image::RgbaImage::from_fn(300, 300, |x, y| {
      if x >= 60 || y >= 60 {
        return Rgba([19, 83, 117, 0]);
      }
      let v = (x + y * 60)
        .wrapping_mul(1_664_525)
        .wrapping_add(1_013_904_223);
      Rgba([
        v as u8,
        (v >> 8) as u8,
        (v >> 16) as u8,
        128 + (v >> 25) as u8,
      ])
    });
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 300, 300, ColorType::Rgba8.into())
      .unwrap();
    let mut options = PdfOptions::default();
    options.images.optimization_policy =
      PdfImageOptimizationPolicy::MicrosoftOfficeFixedOutput(PdfDocumentKind::Pptx);
    let export = RasterExportOptions::new(&options, 12.0, 12.0);
    let premultiplied = apply_black_matte(&source);
    assert!(office_fixed_output_prefers_lossless(
      &premultiplied,
      export,
      RasterOwner::Source,
      false
    ));
    assert!(!office_fixed_output_prefers_lossless(
      &premultiplied,
      export,
      RasterOwner::PowerPointStatic3d,
      false
    ));
    let actual = decode_image(
      &png,
      Some(POWERPOINT_STATIC_3D_BITMAP_CONTENT_TYPE),
      export,
      None,
    )
    .unwrap();
    assert_eq!(actual.size(), (300, 300));
    let jpeg = encode_office_h2v2_jpeg(&premultiplied, export.jpeg_quality.unwrap()).unwrap();
    assert_dct_image(&actual, &jpeg, true);
    assert_eq!(
      actual.direct().alpha().unwrap(),
      source.pixels().map(|p| p[3]).collect::<Vec<_>>()
    );
    assert_eq!(actual.direct().matte, Some([0.0; 3]));
    assert!(!actual.direct().soft_mask_interpolate);

    // A flat projected shape remains palette/lossless under the same owner.
    let flat = image::RgbaImage::from_fn(300, 300, |x, y| {
      if x < 60 && y < 60 {
        Rgba([80, 100, 150, 255])
      } else {
        Rgba([0; 4])
      }
    });
    assert!(office_fixed_output_prefers_lossless(
      &flat,
      export,
      RasterOwner::PowerPointStatic3d,
      false
    ));
    let mut flat = flat;
    flat.put_pixel(0, 0, Rgba([80, 100, 150, 128]));
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(flat.as_raw(), 300, 300, ColorType::Rgba8.into())
      .unwrap();
    let actual = decode_image(
      &png,
      Some(POWERPOINT_STATIC_3D_BITMAP_CONTENT_TYPE),
      export,
      None,
    )
    .unwrap();
    let expected = PdfRasterImage::from_dynamic_with_icc(
      DynamicImage::ImageRgba8(apply_black_matte(&flat)),
      None,
    );
    assert_sampled_image(&actual, &expected, false);
    assert_eq!(actual.direct().matte, Some([0.0; 3]));
    assert!(!actual.direct().soft_mask_interpolate);
    assert_eq!(
      actual.direct().alpha().unwrap(),
      flat.pixels().map(|p| p[3]).collect::<Vec<_>>()
    );
  }

  #[test]
  fn image_set_reuses_matching_rasters_and_separates_metafile_options() {
    let mut jpeg = Vec::new();
    ImageJpegEncoder::new(&mut jpeg)
      .encode(&[255, 0, 0], 1, 1, image::ExtendedColorType::Rgb8)
      .unwrap();
    let options = PdfOptions::default();
    let first = emf_wmf::RenderOptions {
      max_pixels: Some(1_000_000),
      ..emf_wmf::RenderOptions::default()
    };
    let second = emf_wmf::RenderOptions {
      max_pixels: Some(2_000_000),
      ..emf_wmf::RenderOptions::default()
    };
    let mut images = ImageSet::default();

    images
      .raster_direct(&jpeg, Some("image/jpeg"), &options, Some(first), 72.0, 72.0)
      .unwrap();
    images
      .raster_direct(&jpeg, Some("image/jpeg"), &options, Some(first), 72.0, 72.0)
      .unwrap();
    assert_eq!(images.rasters.values().map(Vec::len).sum::<usize>(), 1);

    images
      .raster_direct(
        &jpeg,
        Some("image/jpeg"),
        &options,
        Some(second),
        72.0,
        72.0,
      )
      .unwrap();
    assert_eq!(images.rasters.values().map(Vec::len).sum::<usize>(), 2);
  }

  #[test]
  fn image_set_separates_resolution_requests() {
    let mut jpeg = Vec::new();
    ImageJpegEncoder::new(&mut jpeg)
      .encode(
        &vec![127; 60 * 60 * 3],
        60,
        60,
        image::ExtendedColorType::Rgb8,
      )
      .unwrap();
    let mut options = PdfOptions::default();
    options.images.reduce_resolution = true;
    options.images.max_resolution_dpi = Some(72);
    let mut images = ImageSet::default();

    let full = images
      .raster_direct(&jpeg, Some("image/jpeg"), &options, None, 60.0, 60.0)
      .unwrap();
    let reduced = images
      .raster_direct(&jpeg, Some("image/jpeg"), &options, None, 30.0, 30.0)
      .unwrap();

    assert_eq!(full.size(), (60, 60));
    assert_eq!(reduced.size(), (30, 30));
    assert_eq!(images.rasters.values().map(Vec::len).sum::<usize>(), 2);
  }

  #[test]
  fn downsampling_uses_office_small_image_and_rounding_tolerances() {
    let limits = |width, height| RasterPixelLimits::from_pixels(width, height).unwrap();
    assert_eq!(downsample_size((50, 200), limits(25.0, 100.0)), None);
    assert_eq!(downsample_size((104, 104), limits(100.0, 100.0)), None);
    assert_eq!(
      downsample_size((105, 210), limits(100.0, 100.0)),
      Some((50, 100))
    );
  }

  #[test]
  fn downsampling_rounds_only_after_aspect_fit() {
    let limits = RasterPixelLimits::from_display_size(103.75, 19.0, 96).unwrap();

    // LibreOffice PDFWriter keeps the physical-size limits fractional while
    // selecting one aspect-preserving scale, then rounds the final bitmap.
    // Rounding the 25.333px height limit first would incorrectly produce
    // 136x25 for content-control-header.docx instead of Word's 138x25.
    assert_eq!(downsample_size((316, 58), limits), Some((138, 25)));
  }

  #[test]
  fn materialized_source_rectangle_crop_bypasses_second_downsampling() {
    let source = image::RgbImage::from_pixel(440, 356, image::Rgb([120, 140, 160]));
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 440, 356, ColorType::Rgb8.into())
      .unwrap();
    let export_options = RasterExportOptions {
      use_lossless_compression: true,
      jpeg_quality: None,
      max_size_px: RasterPixelLimits::from_display_size(263.25, 213.0, 96),
      downsample_trigger_px: None,
      word_print_jpeg_downsample_trigger_px: None,
      exact_downsample_trigger: false,
      use_gdiplus_jpeg_resampler: false,
      allow_interpolation: true,
      profile: RasterExportProfile::Requested,
    };

    let ordinary = decode_image(&png, Some("image/png"), export_options, None).unwrap();
    assert_eq!(ordinary.size(), (351, 284));

    let cropped = decode_image(
      &png,
      Some(SOURCE_RECTANGLE_CROP_BITMAP_CONTENT_TYPE),
      export_options,
      None,
    )
    .unwrap();
    assert_eq!(cropped.size(), (440, 356));
  }

  #[test]
  fn transparent_downsampling_ignores_hidden_rgb_samples() {
    let image_with_blue_transparency =
      image::RgbaImage::from_raw(2, 1, vec![255, 0, 0, 255, 0, 0, 255, 0]).unwrap();
    let image_with_green_transparency =
      image::RgbaImage::from_raw(2, 1, vec![255, 0, 0, 255, 0, 255, 0, 0]).unwrap();

    let blue = resize_for_pdf(
      DynamicImage::ImageRgba8(image_with_blue_transparency),
      (1, 1),
    );
    let green = resize_for_pdf(
      DynamicImage::ImageRgba8(image_with_green_transparency),
      (1, 1),
    );

    assert_eq!(blue.to_rgba8(), green.to_rgba8());
  }

  #[test]
  fn pdf_soft_masks_zero_only_fully_transparent_hidden_rgb() {
    let source =
      image::RgbaImage::from_raw(3, 1, vec![10, 20, 30, 255, 40, 50, 60, 1, 70, 80, 90, 0])
        .unwrap();
    let expected_rgb = vec![10, 20, 30, 40, 50, 60, 0, 0, 0];
    let expected_alpha = vec![255, 1, 0];

    let dynamic =
      PdfRasterImage::from_dynamic_with_icc(DynamicImage::ImageRgba8(source.clone()), None);
    assert_eq!(dynamic.pixels.rgb, expected_rgb);
    assert_eq!(
      dynamic.pixels.alpha.as_deref(),
      Some(expected_alpha.as_slice())
    );

    let frame = PdfRasterImage::from_png_frame(3, 1, png::ColorType::Rgba, source.as_raw());
    assert_eq!(frame.pixels.rgb, expected_rgb);
    assert_eq!(
      frame.pixels.alpha.as_deref(),
      Some(expected_alpha.as_slice())
    );

    let separate = PdfRasterImage::from_rgb_with_alpha(
      DynamicImage::ImageRgba8(source.clone()).to_rgb8(),
      &source,
      None,
    );
    assert_eq!(separate.pixels.rgb, expected_rgb);
    assert_eq!(
      separate.pixels.alpha.as_deref(),
      Some(expected_alpha.as_slice())
    );
  }

  #[test]
  fn word_table_border_preserves_fully_transparent_hidden_rgb() {
    let source = image::RgbaImage::from_raw(
      3,
      1,
      vec![0, 112, 192, 255, 0, 112, 192, 0, 0, 112, 192, 255],
    )
    .unwrap();
    let mut png = Vec::new();
    PngEncoder::new(&mut png)
      .write_image(source.as_raw(), 3, 1, ColorType::Rgba8.into())
      .unwrap();

    let actual = decode_image(
      &png,
      Some(WORD_TABLE_BORDER_BITMAP_CONTENT_TYPE),
      RasterExportOptions::new(&PdfOptions::default(), 1.2, 0.48),
      None,
    )
    .unwrap();
    let direct = actual.direct();
    assert!(!direct.interpolate);
    let DirectRasterEncoding::Sampled { pixels } = &direct.encoding else {
      panic!(
        "expected sampled table-border raster, got {:?}",
        direct.encoding
      );
    };
    assert_eq!(pixels.rgb, vec![0, 112, 192, 0, 112, 192, 0, 112, 192]);
    assert_eq!(pixels.alpha.as_deref(), Some([255, 0, 255].as_slice()));
    assert_eq!(
      RasterOwner::from_content_type(Some(WORD_TABLE_BORDER_BITMAP_CONTENT_TYPE)),
      RasterOwner::WordTableBorder
    );
  }

  #[test]
  fn word_black_matte_effect_transports_preserve_their_associated_planes() {
    assert_eq!(
      RasterOwner::from_content_type(Some(WORD_GROUP_GLOW_BITMAP_CONTENT_TYPE)),
      RasterOwner::WordGroupGlow
    );
    assert_eq!(
      RasterOwner::from_content_type(Some(WORD_SHAPE_GLOW_BITMAP_CONTENT_TYPE)),
      RasterOwner::WordShapeGlow
    );
    assert_eq!(
      RasterOwner::from_content_type(Some(WORD_SHAPE_SHADOW_BITMAP_CONTENT_TYPE)),
      RasterOwner::WordShapeShadow
    );
    assert_eq!(
      RasterOwner::from_content_type(Some("image/png")),
      RasterOwner::Source
    );

    let rgba = image::RgbaImage::from_raw(2, 1, vec![0, 0, 0, 0, 40, 50, 60, 128]).unwrap();
    let actual = export_wordprocessing_black_matte_effect_image(DecodedRasterImage {
      image: DynamicImage::ImageRgba8(rgba),
      icc_profile: None,
      jpeg_has_real_physical_resolution: false,
    })
    .unwrap();

    let direct = actual.direct();
    let DirectRasterEncoding::Sampled { pixels } = &direct.encoding else {
      panic!("expected sampled effect plane");
    };
    assert_eq!((direct.width, direct.height), (2, 1));
    assert_eq!(pixels.rgb, [0, 0, 0, 40, 50, 60]);
    assert_eq!(direct.alpha(), Some([0, 128].as_slice()));
    assert_eq!(direct.matte, Some([0.0, 0.0, 0.0]));
    assert!(!direct.interpolate);
    assert!(!direct.soft_mask_interpolate);
  }

  #[test]
  fn jpeg_exif_orientation_is_applied_by_both_decoded_export_paths() {
    let mut jpeg = Vec::new();
    ImageJpegEncoder::new(&mut jpeg)
      .encode(
        &[255, 0, 0, 0, 0, 255],
        2,
        1,
        image::ExtendedColorType::Rgb8,
      )
      .unwrap();
    let exif = [
      b'E', b'x', b'i', b'f', 0, 0, b'I', b'I', 0x2a, 0, 8, 0, 0, 0, 1, 0, 0x12, 1, 3, 0, 1, 0, 0,
      0, 6, 0, 0, 0, 0, 0, 0, 0,
    ];
    let mut oriented = Vec::with_capacity(jpeg.len() + exif.len() + 4);
    oriented.extend_from_slice(&jpeg[..2]);
    oriented.extend_from_slice(&[0xff, 0xe1]);
    oriented.extend_from_slice(&u16::try_from(exif.len() + 2).unwrap().to_be_bytes());
    oriented.extend_from_slice(&exif);
    oriented.extend_from_slice(&jpeg[2..]);

    let image = decode_dynamic_image(&oriented, RasterImageFormat::Jpeg).unwrap();
    let word_print_image = decode_gdiplus_jpeg(&oriented).unwrap();

    assert_eq!(image.image.dimensions(), (1, 2));
    assert_eq!(word_print_image.image.dimensions(), (1, 2));
  }

  #[test]
  fn jpeg_jfif_density_requires_a_real_unit_and_nonzero_axes() {
    let mut jpeg = encode_jpeg(&DynamicImage::new_rgb8(16, 16), 75).unwrap();
    let jfif = jpeg
      .windows(5)
      .position(|window| window == b"JFIF\0")
      .expect("JPEG encoder emits an APP0/JFIF header");

    assert_eq!(jpeg_metadata(&jpeg).unwrap().density_unit, 0);
    assert!(!jpeg_metadata(&jpeg).unwrap().has_real_physical_resolution());

    jpeg[jfif + 7] = 1;
    jpeg[jfif + 8..jfif + 10].copy_from_slice(&96_u16.to_be_bytes());
    jpeg[jfif + 10..jfif + 12].copy_from_slice(&96_u16.to_be_bytes());
    assert!(jpeg_metadata(&jpeg).unwrap().has_real_physical_resolution());

    jpeg[jfif + 8..jfif + 10].copy_from_slice(&0_u16.to_be_bytes());
    assert!(!jpeg_metadata(&jpeg).unwrap().has_real_physical_resolution());

    jpeg[jfif + 8..jfif + 10].copy_from_slice(&38_u16.to_be_bytes());
    jpeg[jfif + 7] = 2;
    assert!(jpeg_metadata(&jpeg).unwrap().has_real_physical_resolution());
  }

  #[test]
  fn jpeg_export_uses_office_four_two_zero_sampling() {
    let jpeg = encode_jpeg(&DynamicImage::new_rgb8(16, 16), 75).unwrap();
    let sof = jpeg
      .windows(2)
      .position(|marker| marker == [0xff, 0xc0])
      .expect("baseline JPEG start-of-frame marker");

    assert_eq!(jpeg[sof + 11], 0x22, "luma sampling factors");
    assert_eq!(jpeg[sof + 14], 0x11, "Cb sampling factors");
    assert_eq!(jpeg[sof + 17], 0x11, "Cr sampling factors");
  }
}
