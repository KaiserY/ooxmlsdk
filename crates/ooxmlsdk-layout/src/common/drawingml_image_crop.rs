//! Integer source intervals for DrawingML bitmap crop materialization.

/// ECMA-376 Part 1 §20.1.8.55 defines source-relative inset percentages.
/// Word and PowerPoint fixed-output controls round the leading coordinate and
/// the surviving interval independently, as in GDI+ CloneBitmapArea. Rounding
/// both discarded ends instead can add or remove a pixel from the interval.
/// Negative percentages remain destination outsets, not source pixels to crop.
pub(crate) fn source_interval(length: u32, leading: f32, trailing: f32) -> (u32, u32) {
  let leading = f64::from(leading.max(0.0));
  let trailing = f64::from(trailing.max(0.0));
  let length_f64 = f64::from(length);
  let origin = (length_f64 * leading).round().clamp(0.0, length_f64) as u32;
  let visible_ratio = (1.0 - leading - trailing).max(0.0);
  let extent = (length_f64 * visible_ratio)
    .round()
    .clamp(0.0, f64::from(length.saturating_sub(origin))) as u32;
  (origin, extent)
}

#[cfg(test)]
mod tests {
  use super::source_interval;

  #[test]
  fn rounds_source_origin_and_retained_extent_independently() {
    assert_eq!(source_interval(1002, 0.34801, 0.19120), (349, 462));
    assert_eq!(source_interval(563, 0.21384, 0.06061), (120, 408));
    for (leading, expected_origin) in [(0.34779, 348), (0.34781, 349)] {
      for trailing in [0.19111, 0.19113] {
        let (origin, extent) = source_interval(1002, leading, trailing);
        assert_eq!(origin, expected_origin);
        assert_eq!(extent, 462);
      }
    }
    for (leading, expected_origin) in [(0.21401, 120), (0.21405, 121)] {
      for trailing in [0.06126, 0.06130] {
        assert_eq!(
          source_interval(563, leading, trailing),
          (expected_origin, 408)
        );
      }
    }
  }

  #[test]
  fn preserves_outsets_and_bounds_empty_or_excessive_crops() {
    assert_eq!(source_interval(100, -0.2, 0.25), (0, 75));
    assert_eq!(source_interval(100, 0.25, -0.2), (25, 75));
    assert_eq!(source_interval(100, 0.75, 0.5), (75, 0));
    assert_eq!(source_interval(100, 2.0, 0.0), (100, 0));
    assert_eq!(source_interval(0, 0.2, 0.2), (0, 0));
  }
}
