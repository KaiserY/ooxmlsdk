use std::io::Cursor;

use emfsdk::emfplus::EmfPlusHatchStyle;
use image::{ColorType, ImageEncoder, codecs::png::PngEncoder};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetPatternValues;

use super::{Color, PatternBitmapSampling};

const VML_HISTORICAL_PATTERN_TILE_SIZE_MILLI_POINTS: u16 = 8_000;
const WORD_GLOBAL_PALETTE_PATTERN_TILE_SIZE_MILLI_POINTS: u16 = 960;
const WORD_GLOBAL_PALETTE_PATTERN_ROWS: [u8; 8] = [0, 0, 0, 0, 0x0f, 0x0f, 0x0f, 0x0f];

/// Resolves a DrawingML preset pattern to its canonical GDI+/EMF+ hatch.
///
/// ECMA-376 Part 1 §20.1.10.51 states that `ST_PresetPatternVal` corresponds
/// to the .NET `HatchStyle` enumeration. DrawingML exposes 54 symbolic names,
/// while `[MS-EMFPLUS]` §2.1.1.13 serializes 53 distinct values: .NET defines
/// both `Cross` and `LargeGrid` as value 4. Keeping that alias collapse here
/// lets all DrawingML hosts share the single Office-verified mask table owned
/// by `emfsdk`.
pub(crate) const fn hatch_style(preset: Option<PresetPatternValues>) -> EmfPlusHatchStyle {
  use EmfPlusHatchStyle as Hatch;
  use PresetPatternValues as Pattern;

  let preset = match preset {
    Some(preset) => preset,
    None => Pattern::Percent5,
  };
  match preset {
    Pattern::Percent5 => Hatch::Percent05,
    Pattern::Percent10 => Hatch::Percent10,
    Pattern::Percent20 => Hatch::Percent20,
    Pattern::Percent25 => Hatch::Percent25,
    Pattern::Percent30 => Hatch::Percent30,
    Pattern::Percent40 => Hatch::Percent40,
    Pattern::Percent50 => Hatch::Percent50,
    Pattern::Percent60 => Hatch::Percent60,
    Pattern::Percent70 => Hatch::Percent70,
    Pattern::Percent75 => Hatch::Percent75,
    Pattern::Percent80 => Hatch::Percent80,
    Pattern::Percent90 => Hatch::Percent90,
    Pattern::Horizontal => Hatch::Horizontal,
    Pattern::Vertical => Hatch::Vertical,
    Pattern::LightHorizontal => Hatch::LightHorizontal,
    Pattern::LightVertical => Hatch::LightVertical,
    Pattern::DarkHorizontal => Hatch::DarkHorizontal,
    Pattern::DarkVertical => Hatch::DarkVertical,
    Pattern::NarrowHorizontal => Hatch::NarrowHorizontal,
    Pattern::NarrowVertical => Hatch::NarrowVertical,
    Pattern::DashedHorizontal => Hatch::DashedHorizontal,
    Pattern::DashedVertical => Hatch::DashedVertical,
    Pattern::Cross | Pattern::LargeGrid => Hatch::LargeGrid,
    Pattern::DownwardDiagonal => Hatch::ForwardDiagonal,
    Pattern::UpwardDiagonal => Hatch::BackwardDiagonal,
    Pattern::LightDownwardDiagonal => Hatch::LightDownwardDiagonal,
    Pattern::LightUpwardDiagonal => Hatch::LightUpwardDiagonal,
    Pattern::DarkDownwardDiagonal => Hatch::DarkDownwardDiagonal,
    Pattern::DarkUpwardDiagonal => Hatch::DarkUpwardDiagonal,
    Pattern::WideDownwardDiagonal => Hatch::WideDownwardDiagonal,
    Pattern::WideUpwardDiagonal => Hatch::WideUpwardDiagonal,
    Pattern::DashedDownwardDiagonal => Hatch::DashedDownwardDiagonal,
    Pattern::DashedUpwardDiagonal => Hatch::DashedUpwardDiagonal,
    Pattern::DiagonalCross => Hatch::DiagonalCross,
    Pattern::SmallCheck => Hatch::SmallCheckerBoard,
    Pattern::LargeCheck => Hatch::LargeCheckerBoard,
    Pattern::SmallGrid => Hatch::SmallGrid,
    Pattern::DotGrid => Hatch::DottedGrid,
    Pattern::SmallConfetti => Hatch::SmallConfetti,
    Pattern::LargeConfetti => Hatch::LargeConfetti,
    Pattern::HorizontalBrick => Hatch::HorizontalBrick,
    Pattern::DiagonalBrick => Hatch::DiagonalBrick,
    Pattern::SolidDiamond => Hatch::SolidDiamond,
    Pattern::OpenDiamond => Hatch::OutlinedDiamond,
    Pattern::DottedDiamond => Hatch::DottedDiamond,
    Pattern::Plaid => Hatch::Plaid,
    Pattern::Sphere => Hatch::Sphere,
    Pattern::Weave => Hatch::Weave,
    Pattern::Divot => Hatch::Divot,
    Pattern::Shingle => Hatch::Shingle,
    Pattern::Wave => Hatch::Wave,
    Pattern::Trellis => Hatch::Trellis,
    Pattern::ZigZag => Hatch::ZigZag,
  }
}

/// Applies VML's historical 8×8 pattern palette.
///
/// VML pattern images encode the foreground as white and the background as
/// black; `v:fill@color` and `color2` replace those two entries. LibreOffice
/// performs the same conversion in `oox/source/vml/vmlformatting.cxx`.
/// Rejecting images that are not exactly an opaque black/white 8×8 mask keeps
/// ordinary tile images byte-for-byte unchanged.
pub(crate) fn recolor_vml_historical_pattern(
  data: &[u8],
  foreground: Color,
  background: Color,
) -> Option<Vec<u8>> {
  vml_historical_pattern_rows(data)?;
  let mut image = image::load_from_memory(data).ok()?.to_rgba8();
  for pixel in image.pixels_mut() {
    let color = if pixel[0] == 255 {
      foreground
    } else {
      background
    };
    *pixel = image::Rgba([color.r, color.g, color.b, color.a]);
  }
  let mut output = Vec::new();
  PngEncoder::new(Cursor::new(&mut output))
    .write_image(image.as_raw(), 8, 8, ColorType::Rgba8.into())
    .ok()?;
  Some(output)
}

/// Decodes VML's historical opaque 8×8 black/white pattern image.
///
/// White bits are foreground bits, independently of palette order or the
/// first decoded pixel. This polarity is defined by Office's VML behavior and
/// is also used by LibreOffice's VML importer.
pub(crate) fn vml_historical_pattern_rows(data: &[u8]) -> Option<[u8; 8]> {
  let image = image::load_from_memory(data).ok()?.to_rgba8();
  if image.dimensions() != (8, 8) {
    return None;
  }
  let mut rows = [0_u8; 8];
  for (y, row) in rows.iter_mut().enumerate() {
    for x in 0..8 {
      let pixel = image.get_pixel(x, y as u32).0;
      if pixel[3] != u8::MAX
        || !matches!((pixel[0], pixel[1], pixel[2]), (0, 0, 0) | (255, 255, 255))
      {
        return None;
      }
      if pixel[0] == u8::MAX {
        *row |= 0x80 >> x;
      }
    }
  }
  Some(rows)
}

/// Builds the page/world-coordinate brush used for an Office VML pattern.
///
/// ECMA-376 and [MS-OI29500]/[MS-OE376] define the general behavior: the
/// black/white image is colorized and tiled. Word 2010/365 fixed output gives
/// the historical 8×8 Office mask an effective 8pt period and repeats it four
/// times in a 32×32 sampled tile. One separately encoded form seen in Office
/// fixed output — a GIF with a global palette and no local image palette — is
/// interpreted as Word's 25% 0.96pt brush and sampled as 2×2 rather than as
/// its decoded mask. Keep that compatibility branch deliberately narrow; it
/// is observed behavior, not a general GIF or VML rule.
pub(crate) fn vml_historical_pattern_fill(
  data: &[u8],
  foreground: Color,
  background: Color,
) -> Option<super::PatternFill> {
  let rows = vml_historical_pattern_rows(data)?;
  let (rows, tile_size_milli_points, bitmap_sampling) = if gif_uses_global_palette_only(data) {
    (
      WORD_GLOBAL_PALETTE_PATTERN_ROWS,
      WORD_GLOBAL_PALETTE_PATTERN_TILE_SIZE_MILLI_POINTS,
      PatternBitmapSampling::from_lattice(2, 1),
    )
  } else {
    (
      rows,
      VML_HISTORICAL_PATTERN_TILE_SIZE_MILLI_POINTS,
      PatternBitmapSampling::from_lattice(32, 4),
    )
  };
  Some(super::PatternFill::bitmap8_sampled(
    rows,
    tile_size_milli_points,
    bitmap_sampling,
    foreground,
    background,
  ))
}

fn gif_uses_global_palette_only(data: &[u8]) -> bool {
  if data.len() < 13
    || (&data[..6] != b"GIF87a" && &data[..6] != b"GIF89a")
    || u16::from_le_bytes([data[6], data[7]]) != 8
    || u16::from_le_bytes([data[8], data[9]]) != 8
  {
    return false;
  }
  let packed = data[10];
  if packed & 0x80 == 0 {
    return false;
  }
  let table_entries = 1_usize << (usize::from(packed & 0x07) + 1);
  let mut offset = 13_usize.saturating_add(table_entries.saturating_mul(3));
  while let Some(kind) = data.get(offset).copied() {
    match kind {
      0x2c => {
        let Some(descriptor_packed) = data.get(offset + 9).copied() else {
          return false;
        };
        return descriptor_packed & 0x80 == 0;
      }
      0x21 => {
        offset = match skip_gif_sub_blocks(data, offset.saturating_add(2)) {
          Some(offset) => offset,
          None => return false,
        };
      }
      0x3b => return false,
      _ => return false,
    }
  }
  false
}

fn skip_gif_sub_blocks(data: &[u8], mut offset: usize) -> Option<usize> {
  loop {
    let size = usize::from(*data.get(offset)?);
    offset = offset.checked_add(1)?;
    if size == 0 {
      return Some(offset);
    }
    offset = offset.checked_add(size)?;
    if offset > data.len() {
      return None;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const OFFICE_LOCAL_PALETTE_GIF: &[u8] = &[
    0x47, 0x49, 0x46, 0x38, 0x37, 0x61, 0x08, 0x00, 0x08, 0x00, 0x77, 0x01, 0x00, 0x2c, 0x00, 0x00,
    0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x80, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x02, 0x08, 0x44,
    0x8e, 0xa9, 0xcb, 0x6c, 0x0d, 0x61, 0x01, 0x00, 0x3b,
  ];
  const GLOBAL_PALETTE_GIF: &[u8] = &[
    0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x08, 0x00, 0x08, 0x00, 0x80, 0x00, 0x00, 0xff, 0xff, 0xff,
    0x00, 0x00, 0x00, 0x2c, 0x00, 0x00, 0x00, 0x00, 0x08, 0x00, 0x08, 0x00, 0x00, 0x02, 0x08, 0x0c,
    0x82, 0xa9, 0xcb, 0xcd, 0xe7, 0x62, 0x2c, 0x00, 0x3b,
  ];

  const fn color(r: u8, g: u8, b: u8) -> Color {
    Color {
      r,
      g,
      b,
      a: u8::MAX,
    }
  }

  #[test]
  fn drawingml_cross_names_share_the_documented_hatch_value() {
    assert_eq!(
      hatch_style(Some(PresetPatternValues::Cross)),
      EmfPlusHatchStyle::LargeGrid
    );
    assert_eq!(
      hatch_style(Some(PresetPatternValues::LargeGrid)),
      EmfPlusHatchStyle::LargeGrid
    );
  }

  #[test]
  fn missing_drawingml_pattern_uses_the_office_pct5_default() {
    assert_eq!(hatch_style(None), EmfPlusHatchStyle::Percent05);
  }

  #[test]
  fn drawingml_pattern_preserves_office_fixed_output_sampling_lattice() {
    let style = EmfPlusHatchStyle::WideUpwardDiagonal;
    let fill =
      super::super::PatternFill::drawingml(style, color(0x5e, 0xad, 0x35), color(0x00, 0x79, 0x29));

    assert_eq!(fill.bitmap_sampling.image_size_px(), 16);
    assert_eq!(fill.bitmap_sampling.tile_repetitions(), 1);
    assert_eq!(fill.bitmap_tile_size_points(), 6.0);
    for y in 0..16 {
      for x in 0..16 {
        assert_eq!(
          fill.bitmap_sample_is_foreground(x, y),
          style.is_foreground((x / 2) as i32, (y / 2) as i32),
          "sample ({x}, {y})",
        );
      }
    }
  }

  #[test]
  fn vml_historical_pattern_uses_white_as_foreground() {
    let mut input = Vec::new();
    let mut pixels = vec![0_u8; 8 * 8 * 4];
    for (index, pixel) in pixels.chunks_exact_mut(4).enumerate() {
      let value = if index % 2 == 0 { 255 } else { 0 };
      pixel.copy_from_slice(&[value, value, value, 255]);
    }
    PngEncoder::new(Cursor::new(&mut input))
      .write_image(&pixels, 8, 8, ColorType::Rgba8.into())
      .unwrap();
    let output = recolor_vml_historical_pattern(
      &input,
      Color {
        r: 1,
        g: 2,
        b: 3,
        a: 4,
      },
      Color {
        r: 5,
        g: 6,
        b: 7,
        a: 8,
      },
    )
    .unwrap();
    let output = image::load_from_memory(&output).unwrap().to_rgba8();
    assert_eq!(output.get_pixel(0, 0).0, [1, 2, 3, 4]);
    assert_eq!(output.get_pixel(1, 0).0, [5, 6, 7, 8]);
  }

  #[test]
  fn office_local_palette_mask_keeps_its_authored_eight_point_period() {
    let fill = vml_historical_pattern_fill(
      OFFICE_LOCAL_PALETTE_GIF,
      color(192, 192, 192),
      color(255, 255, 0),
    )
    .expect("Office historical pattern");
    assert_eq!(fill.tile_size_milli_points, 8_000);
    assert_eq!(fill.bitmap_sampling.image_size_px(), 32);
    assert_eq!(fill.bitmap_sampling.tile_repetitions(), 4);
    assert_eq!(fill.bitmap_tile_size_points(), 32.0);
    assert!(fill.bitmap_sample_is_foreground(0, 0));
    assert!(fill.bitmap_sample_is_foreground(8, 0));
    assert!(!fill.bitmap_sample_is_foreground(1, 0));
    assert_eq!(
      fill.mask,
      super::super::PatternMask::Bitmap8([0x80, 0, 0, 0, 0x08, 0, 0, 0])
    );
  }

  #[test]
  fn word_global_palette_form_uses_the_observed_quarter_tone_brush() {
    let fill =
      vml_historical_pattern_fill(GLOBAL_PALETTE_GIF, color(192, 192, 192), color(255, 255, 0))
        .expect("global-palette pattern");
    assert_eq!(fill.tile_size_milli_points, 960);
    assert_eq!(fill.bitmap_sampling.image_size_px(), 2);
    assert_eq!(fill.bitmap_sampling.tile_repetitions(), 1);
    assert_eq!(fill.bitmap_tile_size_points(), 0.96);
    assert!(!fill.bitmap_sample_is_foreground(0, 0));
    assert!(!fill.bitmap_sample_is_foreground(1, 0));
    assert!(!fill.bitmap_sample_is_foreground(0, 1));
    assert!(fill.bitmap_sample_is_foreground(1, 1));
    assert_eq!(
      fill.mask,
      super::super::PatternMask::Bitmap8([0, 0, 0, 0, 0x0f, 0x0f, 0x0f, 0x0f])
    );
  }
}
