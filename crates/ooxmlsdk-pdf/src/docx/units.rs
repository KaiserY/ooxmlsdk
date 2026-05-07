pub(crate) fn twips_to_points(value: f32) -> f32 {
  value / 20.0
}

pub(crate) fn emu_to_points(value: i64) -> f32 {
  value as f32 / 12_700.0
}
