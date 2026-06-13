use ooxmlsdk::units as sdk_units;

pub(crate) const POINTS_PER_INCH: f32 = 72.0;
pub(crate) const MILLIMETERS_PER_INCH: f32 = 25.4;
pub(crate) const TWIPS_PER_POINT: f32 = sdk_units::TWIPS_PER_POINT as f32;

#[inline]
pub(crate) fn twips_to_points(value: f32) -> f32 {
  value / TWIPS_PER_POINT
}

#[inline]
pub(crate) fn emu_to_points(value: i64) -> f32 {
  sdk_units::emu_to_points(value) as f32
}

#[inline]
pub(crate) fn millimeters_to_points(value: f32) -> f32 {
  value * POINTS_PER_INCH / MILLIMETERS_PER_INCH
}
