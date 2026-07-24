use emfsdk::emfplus::EmfPlusHatchStyle;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetPatternValues;

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
}
