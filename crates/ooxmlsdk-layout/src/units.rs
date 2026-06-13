use ooxmlsdk::units as sdk_units;

pub const POINTS_PER_INCH: f32 = 72.0;
pub const MILLIMETERS_PER_INCH: f32 = 25.4;
pub const CENTIMETERS_PER_INCH: f32 = 2.54;
pub const POINTS_PER_CSS_PIXEL: f32 = POINTS_PER_INCH / 96.0;
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
