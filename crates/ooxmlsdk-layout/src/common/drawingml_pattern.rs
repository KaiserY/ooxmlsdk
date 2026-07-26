use std::io::Cursor;

use emfsdk::emfplus::EmfPlusHatchStyle;
use image::{ColorType, ImageEncoder, codecs::png::PngEncoder};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetPatternValues;

use super::Color;

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
  let mut image = image::load_from_memory(data).ok()?.to_rgba8();
  if image.dimensions() != (8, 8) {
    return None;
  }
  for pixel in image.pixels() {
    if pixel[3] != 255 || !matches!((pixel[0], pixel[1], pixel[2]), (0, 0, 0) | (255, 255, 255)) {
      return None;
    }
  }
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

#[cfg(test)]
mod tests {
  use super::*;

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
}
