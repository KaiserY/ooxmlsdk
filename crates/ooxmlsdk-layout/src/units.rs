use ooxmlsdk::units as sdk_units;

pub const POINTS_PER_INCH: f32 = 72.0;
pub const MILLIMETERS_PER_INCH: f32 = 25.4;
pub const CENTIMETERS_PER_INCH: f32 = 2.54;
pub const CSS_PIXELS_PER_INCH: f32 = 96.0;
pub const POINTS_PER_CSS_PIXEL: f32 = POINTS_PER_INCH / CSS_PIXELS_PER_INCH;
// Word, Excel, and PowerPoint fixed-output geometry is quantized on the
// 600dpi printer-device grid used by the reference Office environment.
pub const OFFICE_FIXED_OUTPUT_DPI: f32 = 600.0;
// Office materializes bitmap-backed fixed-output effects and legacy control
// hosts at 200 pixels per inch, then positions those images on the 600dpi
// printer-device grid above. Keep the two grids distinct: one owns raster
// allocation and sampling, the other owns PDF coordinates.
pub const OFFICE_FIXED_OUTPUT_RASTER_DPI: f32 = 200.0;
// Word's fixed-output VML path writes an authored `strokeweight="0"` as a
// printable hairline.  The Office reference PDFs serialize that hairline as
// 0.14pt; keep this separate from the 0.75pt Office default used when the
// VML weight is omitted ([MS-OI29500] §19.1, strokeweight note).
pub const OFFICE_VML_HAIRLINE_WIDTH_PT: f32 = 0.14;
pub const TWIPS_PER_POINT: f32 = sdk_units::TWIPS_PER_POINT as f32;
pub const MM100_PER_MILLIMETER: f32 = sdk_units::MM100_PER_MILLIMETER as f32;
pub const DRAWINGML_MAX_FRACTION_BELOW_ONE: f32 = 0.999;
pub const BYTE_MAX_AS_FLOAT: f32 = u8::MAX as f32;
pub const WORD_BORDER_SIZE_UNITS_PER_POINT: f32 = 8.0;
pub const WORD_LINE_HEIGHT_UNITS_PER_LINE: f32 = 240.0;

#[inline]
pub fn twips_to_points(value: f32) -> f32 {
  value / TWIPS_PER_POINT
}

#[inline]
pub fn emu_to_points(value: i64) -> f32 {
  sdk_units::emu_to_points(value) as f32
}

#[inline]
pub fn emu_to_points_f32(value: f32) -> f32 {
  value / sdk_units::EMUS_PER_POINT as f32
}

#[inline]
pub fn millimeters_to_points(value: f32) -> f32 {
  value * POINTS_PER_INCH / MILLIMETERS_PER_INCH
}

/// Converts a VML `strokeweight`/`v:stroke@weight` markup value to points.
///
/// Unitless markup values are EMUs. This is deliberately distinct from VML
/// CSS dimensions, whose omitted unit is pixels, and from scripted
/// `strokeweight` assignment, whose omitted unit is points.
pub fn vml_stroke_weight_to_points(value: &str) -> Option<f32> {
  let value = value.trim();
  if value.is_empty() {
    return None;
  }

  let (number, multiplier) = if let Some(number) = value.strip_suffix("emu") {
    (number, 1.0 / sdk_units::EMUS_PER_POINT as f32)
  } else if let Some(number) = value.strip_suffix("pt") {
    (number, 1.0)
  } else if let Some(number) = value.strip_suffix("in") {
    (number, POINTS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("cm") {
    (number, POINTS_PER_INCH / CENTIMETERS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("mm") {
    (number, POINTS_PER_INCH / MILLIMETERS_PER_INCH)
  } else if let Some(number) = value.strip_suffix("pc") {
    (number, 12.0)
  } else if let Some(number) = value.strip_suffix("px") {
    (number, POINTS_PER_CSS_PIXEL)
  } else {
    (value, 1.0 / sdk_units::EMUS_PER_POINT as f32)
  };

  number
    .trim()
    .parse::<f32>()
    .ok()
    .map(|value| value * multiplier)
}

/// Resolves an authored VML stroke weight using Word's fixed-output defaults.
///
/// An omitted/invalid weight keeps the Office VML default supplied by the
/// caller.  An authored zero is a visible hairline in Word's PDF output, not
/// an invisible stroke and not the omitted-weight default.
#[inline]
pub fn office_vml_stroke_weight_to_points(value: Option<&str>, default: f32) -> f32 {
  match value.and_then(vml_stroke_weight_to_points) {
    Some(width) if width == 0.0 => OFFICE_VML_HAIRLINE_WIDTH_PT,
    Some(width) => width,
    None => default,
  }
}

#[inline]
pub fn quantize_points_to_office_print_grid(value: f32) -> f32 {
  (value * OFFICE_FIXED_OUTPUT_DPI / POINTS_PER_INCH).round() * POINTS_PER_INCH
    / OFFICE_FIXED_OUTPUT_DPI
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_close(actual: Option<f32>, expected: f32) {
    let actual = actual.expect("VML stroke weight");
    assert!(
      (actual - expected).abs() < 0.000_01,
      "actual={actual}, expected={expected}"
    );
  }

  #[test]
  fn unitless_vml_stroke_weight_defaults_to_emu() {
    assert_close(vml_stroke_weight_to_points("12700"), 1.0);
    assert_close(vml_stroke_weight_to_points("28440"), 28440.0 / 12700.0);
    assert_close(vml_stroke_weight_to_points("720"), 720.0 / 12700.0);
  }

  #[test]
  fn explicit_vml_stroke_units_do_not_use_the_emu_default() {
    assert_close(vml_stroke_weight_to_points("1pt"), 1.0);
    assert_close(vml_stroke_weight_to_points("0.5pt"), 0.5);
    assert_close(vml_stroke_weight_to_points("1in"), 72.0);
    assert_close(vml_stroke_weight_to_points("2.54cm"), 72.0);
    assert_close(vml_stroke_weight_to_points("25.4mm"), 72.0);
    assert_close(vml_stroke_weight_to_points("1pc"), 12.0);
    assert_close(vml_stroke_weight_to_points("96px"), 72.0);
    assert_close(vml_stroke_weight_to_points("12700emu"), 1.0);
    assert_eq!(vml_stroke_weight_to_points(""), None);
    assert_eq!(vml_stroke_weight_to_points("auto"), None);
  }

  #[test]
  fn office_vml_zero_weight_is_a_fixed_output_hairline() {
    assert_eq!(
      office_vml_stroke_weight_to_points(Some("0"), 0.75),
      OFFICE_VML_HAIRLINE_WIDTH_PT
    );
    assert_eq!(office_vml_stroke_weight_to_points(None, 0.75), 0.75);
    assert_eq!(office_vml_stroke_weight_to_points(Some("auto"), 0.75), 0.75);
  }
}
