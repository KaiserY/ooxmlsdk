pub(crate) const POINTS_PER_INCH: f32 = 72.0;
pub(crate) const MILLIMETERS_PER_INCH: f32 = 25.4;
pub(crate) const CENTIMETERS_PER_INCH: f32 = 2.54;
pub(crate) const POINTS_PER_PICA: f32 = 12.0;
pub(crate) const POINTS_PER_CSS_PIXEL: f32 = POINTS_PER_INCH / 96.0;
pub(crate) const TWIPS_PER_POINT: f32 = 20.0;
pub(crate) const EMUS_PER_POINT: f32 = 12_700.0;
pub(crate) const MM100_PER_MILLIMETER: f32 = 100.0;
pub(crate) const DRAWINGML_PERCENT_SCALE: f32 = 100_000.0;
pub(crate) const DRAWINGML_ANGLE_UNITS_PER_DEGREE: f32 = 60_000.0;
pub(crate) const DRAWINGML_MAX_FRACTION_BELOW_ONE: f32 = 0.999;
pub(crate) const WORD_PERCENT_MEASURE_SCALE: f32 = 5_000.0;
pub(crate) const VML_PERCENT_SCALE: f32 = 100.0;
pub(crate) const VML_FIXED_SCALE: f32 = 65_536.0;
pub(crate) const BYTE_MAX_AS_FLOAT: f32 = u8::MAX as f32;
pub(crate) const WORD_BORDER_SIZE_UNITS_PER_POINT: f32 = 8.0;
pub(crate) const WORD_LINE_HEIGHT_UNITS_PER_LINE: f32 = 240.0;

pub(crate) fn twips_to_points(value: f32) -> f32 {
  value / TWIPS_PER_POINT
}

pub(crate) fn points_to_twips(value: f32) -> f32 {
  value * TWIPS_PER_POINT
}

pub(crate) fn emu_to_points(value: i64) -> f32 {
  value as f32 / EMUS_PER_POINT
}

pub(crate) fn inches_to_points(value: f32) -> f32 {
  value * POINTS_PER_INCH
}

pub(crate) fn centimeters_to_points(value: f32) -> f32 {
  value * POINTS_PER_INCH / CENTIMETERS_PER_INCH
}

pub(crate) fn millimeters_to_points(value: f32) -> f32 {
  value * POINTS_PER_INCH / MILLIMETERS_PER_INCH
}
