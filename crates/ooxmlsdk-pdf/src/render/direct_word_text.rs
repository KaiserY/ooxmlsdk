//! Word fixed-output text-array realization, independent of line layout.
//!
//! Native PDF-export observations cover 1,737 runs and 5,840 adjustments.
//! The exporter accumulates nominal PDF widths and requested paint advances
//! separately in FLOAT, then emits an integer adjustment at each boundary.

use super::common;

/// Realize authored Word character width on the selected physical font.
/// Native CreateFontIndirectW observations use integer lfWidth relative to
/// tmAveCharWidth. Word's PDF transform is serialized with five significant
/// digits. Layout advances remain independently owned by the completed line.
pub(super) fn font_horizontal_scale(
  face: &ooxmlsdk_layout::fonts::FontFaceData,
  font_size_pt: f32,
  percent: u16,
) -> Option<f32> {
  let ratio = common::wordprocessing_device::font_width_ratio(face, font_size_pt, percent)?;
  Some(serialize_font_width_ratio(ratio))
}

fn serialize_font_width_ratio(ratio: f64) -> f32 {
  let decimal_scale = 10.0_f64.powi(4 - ratio.log10().floor() as i32);
  ((ratio * decimal_scale).round() / decimal_scale) as f32
}

/// Native ordinary leader operations serialize Tc with three significant
/// digits, after computing it using the unserialized LOGFONT width ratio.
/// This is independent of the five-digit text matrix and RTL TD operations.
pub(super) fn serialize_leader_character_spacing(spacing: f32) -> f32 {
  if spacing == 0.0 || !spacing.is_finite() {
    return spacing;
  }
  let spacing = f64::from(spacing);
  let scale = 10.0_f64.powi(2 - spacing.abs().log10().floor() as i32);
  ((spacing * scale).round() / scale) as f32
}

#[cfg(test)]
fn font_horizontal_scale_from_average(average_pixels: u32, percent: u16) -> Option<f32> {
  common::wordprocessing_device::font_width_ratio_from_average(average_pixels, percent)
    .map(serialize_font_width_ratio)
}

pub(super) struct WordTextPositioning {
  pub character_spacing_pt: f32,
  /// Index of the last glyph before the adjustment, and its PDF text units.
  pub adjustments: Vec<(usize, i32)>,
}

impl WordTextPositioning {
  pub fn new(widths: &[f32], advances_pt: &[f32], size_pt: f32, scale: f32) -> Self {
    debug_assert_eq!(widths.len(), advances_pt.len());
    // Word tests at most the first seven nonterminal advances for a uniform
    // character-space term. The final advance has no following glyph to move.
    let mut differences = widths
      .iter()
      .zip(advances_pt)
      .take(widths.len().saturating_sub(1).min(7))
      .map(|(&width, &advance)| advance / scale - width * size_pt / 1000.0);
    let character_spacing_pt = differences.next().map_or(0.0, |first| {
      if differences.all(|delta| f64::from((first - delta).abs()) <= 0.01) {
        first
      } else {
        0.0
      }
    });
    // The native paint tolerance has a three-text-unit floor. Keep the two
    // FLOAT divisions separate, as at the writer's device/EM normalization.
    let threshold = (0.3 / (size_pt * scale) / 1000.0).max(0.003);
    let mut nominal = 0.0f32;
    let mut requested = 0.0f32;
    let mut adjustments = Vec::new();
    for (index, (&width, &advance)) in widths.iter().zip(advances_pt).enumerate() {
      nominal += character_spacing_pt / size_pt + width / 1000.0;
      requested += advance / size_pt / scale;
      let delta = nominal - requested;
      if delta.abs() > threshold || index + 1 == widths.len() {
        if index + 1 != widths.len() {
          // Observed conversion is +0.5 followed by truncation, including
          // negative values; Rust round() would give a different negative TJ.
          adjustments.push((index, (delta * 1000.0 + 0.5) as i32));
        }
        nominal = 0.0;
        requested = 0.0;
      }
    }
    Self {
      character_spacing_pt,
      adjustments,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::{WordTextPositioning, font_horizontal_scale_from_average};

  #[test]
  fn native_leader_tc_uses_three_significant_digits() {
    for (value, expected) in [
      (0.05088, 0.0509),
      (0.10155, 0.102),
      (3.05088, 3.05),
      (-2.94912, -2.95),
      (0.0, 0.0),
    ] {
      assert_eq!(super::serialize_leader_character_spacing(value), expected);
    }
  }

  #[test]
  fn native_word_font_width_uses_integer_average_width_and_pdf_precision() {
    // Independent Office PDF transforms and actual LOGFONT captures.
    for (average, percentage, expected) in [
      (41, 103, 1.0244),
      (57, 103, 1.0175),
      (41, 110, 1.0976),
      (57, 110, 1.0877),
      (41, 80, 0.78049),
      (44, 80, 0.79545),
      (55, 50, 0.49091),
      (57, 50, 0.49123),
      (67, 80, 0.79104),
      (76, 80, 0.78947),
      (41, 1, 0.02439),
      (57, 1, 0.017544),
    ] {
      assert_eq!(
        font_horizontal_scale_from_average(average, percentage),
        Some(expected)
      );
    }
  }

  #[test]
  fn native_word_text_array_accumulates_and_resets_at_observed_boundaries() {
    // Times New Roman Bold 12pt, native "Left sample". Source advances
    // are the FLOAT values observed at the PDF writer, not fitted positions.
    let widths = [
      667., 444., 333., 333., 250., 389., 500., 833., 556., 278., 444.,
    ];
    let advances = [
      0x4100a3d7, 0x40a8f5c2, 0x407d70a4, 0x407d70a4, 0x40400000, 0x4095c28f, 0x40c00000,
      0x412147ae, 0x40d70a3e, 0x40570a3e, 0x40a8f5c2,
    ]
    .map(f32::from_bits);
    let result = WordTextPositioning::new(&widths, &advances, 12.0, 1.0);
    assert_eq!(result.character_spacing_pt, 0.0);
    assert_eq!(
      result.adjustments,
      [(0, -2), (1, 4), (3, 6), (7, -7), (8, -3)]
    );
  }

  #[test]
  fn native_word_character_space_ignores_the_terminal_advance() {
    let widths = [500.; 4];
    let advances = [67., 67., 67., 66.].map(|dots| dots * 0.12);
    let result = WordTextPositioning::new(&widths, &advances, 15.96, 1.0);
    assert_eq!(result.character_spacing_pt, 0.059999943);
    assert!(result.adjustments.is_empty());
  }
}
