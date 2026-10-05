//! Word's fixed-output paint coordinates travel through a 600-DPI EMF.
//!
//! The PDF exporter replays that EMF at 3000 DPI. Its frame excludes the last
//! source device dot; `Page.EnhMetaFileBits` instead exposes an inclusive frame.
//! Native EnumEnhMetaFile/GetWorldTransform/LPtoDP readbacks establish the FLOAT
//! arithmetic and POINTFIX conversion below. This is a paint transform, not a
//! correction to row heights or table placement.

// Retain the Word-facing API while sharing the unchanged replay algorithm
// with Excel's ordinary worksheet-cell output.
pub use super::fixed_output_device::PageCoordinates;

/// Convert one integral Word layout point to the source printer coordinate.
/// Frame points and paragraph-local distances retain independent ownership.
pub fn source_coordinate(pt: f32) -> i64 {
  source_coordinate_precise(f64::from(pt))
}

/// Preserve a frame and its local distances through floating-point movement.
pub fn source_coordinate_precise(pt: f64) -> i64 {
  let reference = (pt * 4096.0).round() as i64;
  let product = i128::from(reference) * 600;
  ((product + product.signum() * (294912 / 2)) / 294912) as i64
}

/// Unserialized Word LOGFONT width ratio on the selected physical face.
/// Native font-creation probes establish integer average-width ownership.
pub fn font_width_ratio(
  face: &crate::fonts::FontFaceData,
  font_size_pt: f32,
  percent: u16,
) -> Option<f64> {
  use skrifa::{FontRef, raw::TableProvider};
  if !(1..=600).contains(&percent)
    || !font_size_pt.is_finite()
    || font_size_pt <= 0.0
    || face.synthetic_bold
  {
    return None;
  }
  if percent == 100 {
    return Some(1.0);
  }
  let font = FontRef::from_index(face.data.as_slice(), face.index).ok()?;
  // GDI's synthesized italic slants the outline without changing the
  // average character width. Native font creation and TEXTMETRIC readbacks
  // retain the same lfWidth/tmAveCharWidth pair for regular and italic faces.
  // Synthetic bold keeps its separate advance/average-width policy.
  let units = font.head().ok()?.units_per_em();
  let average = font.os2().ok()?.x_avg_char_width();
  if units == 0 || average <= 0 {
    return None;
  }
  let pixels = (f64::from(font_size_pt) * 600.0 / 72.0).round();
  let average_pixels = (f64::from(average) * pixels / f64::from(units))
    .round()
    .max(1.0);
  if average_pixels > f64::from(u32::MAX / 600) {
    return None;
  }
  font_width_ratio_from_average(average_pixels as u32, percent)
}

/// Ratio before PDF transform serialization or device-advance rounding.
pub fn font_width_ratio_from_average(average_pixels: u32, percent: u16) -> Option<f64> {
  if average_pixels == 0 || !(1..=600).contains(&percent) {
    return None;
  }
  let width = (average_pixels.checked_mul(u32::from(percent))? / 100).max(1);
  Some(f64::from(width) / f64::from(average_pixels))
}

/// Word's shaped advances retain signed GPOS adjustments. Native call/return
/// observations establish addition of 0.5 followed by truncation, including
/// negative cursive advances; rounding away from zero changes whole lines.
pub(crate) fn glyph_advance(value: f64) -> f64 {
  (value + 0.5).trunc()
}

/// Source printer ownership retained by a generated Word tab leader.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabLeader {
  pub source_advance_pt: f32,
  pub paragraph_bidi: bool,
}

/// Ideal Arabic glyph advances in Word's document measurement coordinates.
/// This is independent of the 600-DPI font used to paint the resulting line.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ArabicMeasurement {
  units_per_em: f64,
  measurement_em: f64,
  width_ratio: f64,
}

impl ArabicMeasurement {
  pub(crate) fn new(
    face: &crate::fonts::FontFaceData,
    legacy_font_measurement: bool,
    percent: u16,
  ) -> Option<Self> {
    use skrifa::{FontRef, raw::TableProvider};
    // Word's regular/synthesized-italic controls retain identical ideal
    // arrays as well as identical device arrays. The slant is paint state;
    // it does not change the measurement font's advances or average width.
    if face.synthetic_bold {
      return None;
    }
    let font = FontRef::from_index(face.data.as_slice(), face.index).ok()?;
    font.glyf().ok()?;
    if font.fvar().is_ok() {
      return None;
    }
    Self::from_metrics(
      font.head().ok()?.units_per_em(),
      font.os2().ok()?.x_avg_char_width(),
      legacy_font_measurement,
      percent,
    )
  }

  fn from_metrics(
    units_per_em: u16,
    average_width: i16,
    legacy_font_measurement: bool,
    percent: u16,
  ) -> Option<Self> {
    if units_per_em == 0 || average_width <= 0 || !(1..=600).contains(&percent) {
      return None;
    }
    let units_per_em = f64::from(units_per_em);
    // Configured Word font-creation and MSLS ideal-array probes establish
    // 1000-pixel measurement fonts for useWord97LineBreakRules before mode15.
    // With that flag disabled (or mode15), the font uses design-unit height.
    // Each face owns its rounded average and integer lfWidth independently.
    let measurement_em = if legacy_font_measurement {
      1000.0
    } else {
      units_per_em
    };
    let average = (f64::from(average_width) * measurement_em / units_per_em)
      .round()
      .max(1.0) as u32;
    Some(Self {
      units_per_em,
      measurement_em,
      width_ratio: font_width_ratio_from_average(average, percent)?,
    })
  }

  fn advance_pt(self, design_advance: f64, font_size_pt: f64) -> f32 {
    let measured =
      glyph_advance(design_advance * self.measurement_em / self.units_per_em * self.width_ratio);
    // MSLS receives integral 1/4096-point widths. Native readbacks establish
    // truncation after conversion, independently for each shaped glyph.
    ((measured * font_size_pt * 4096.0 / self.measurement_em).trunc() / 4096.0) as f32
  }

  pub(crate) fn apply(
    self,
    run: &mut ooxmlsdk_fonts::ShapedRun<'_, '_>,
    scale: f32,
    character_spacing_pt: f32,
  ) {
    let size = f64::from(run.font_size_pt.0);
    if run.approximate
      || !size.is_finite()
      || size <= 0.0
      || !scale.is_finite()
      || scale <= 0.0
      || !character_spacing_pt.is_finite()
      || run
        .glyphs
        .iter()
        .any(|glyph| !glyph.x_advance_pt.is_finite())
    {
      return;
    }
    let recover_design_units = self.units_per_em / (size * f64::from(scale));
    let mut total = 0.0;
    let glyphs = run.glyphs.to_mut();
    for index in 0..glyphs.len() {
      // The shaper's authored pitch follows source clusters. Font realization
      // owns only the natural advance; keep that pitch outside its integer
      // measurement font and restore it after conversion to document units.
      let pitch =
        if index + 1 == glyphs.len() || glyphs[index].text_range != glyphs[index + 1].text_range {
          character_spacing_pt
        } else {
          0.0
        };
      let glyph = &mut glyphs[index];
      // HarfRust's untracked positions are integral design units. Recover
      // those inputs before realization, avoiding a second f32 half-tie.
      let design_advance =
        ((f64::from(glyph.x_advance_pt) - f64::from(pitch)) * recover_design_units).round();
      glyph.x_advance_pt = self.advance_pt(design_advance, size) + pitch;
      total += glyph.x_advance_pt;
    }
    run.advance_pt = total;
  }
}

#[cfg(test)]
mod tests {
  use super::ArabicMeasurement;

  #[test]
  fn word_arabic_measurement_uses_compatibility_font_and_native_ideal_units() {
    // Read-only configured Word MSLS input arrays: Traditional Arabic,
    // upem2048, average942, shaped advances799/613/500 (baa-alef/hamza/space).
    // No font bytes or SDK-generated expectations are used here.
    for (legacy, size, expected) in [
      (true, 5.0, [8212, 6307, 5140]),
      (true, 15.0, [24637, 18923, 15421]),
      (true, 19.0, [31207, 23969, 19533]),
      (false, 5.0, [8230, 6310, 5150]),
      (false, 15.0, [24690, 18930, 15450]),
    ] {
      let measurement = ArabicMeasurement::from_metrics(2048, 942, legacy, 103).unwrap();
      assert_eq!(
        [799.0, 613.0, 500.0]
          .map(|advance| (measurement.advance_pt(advance, size) * 4096.0) as i32),
        expected,
        "legacy {legacy}, size {size}"
      );
    }
    let legacy = ArabicMeasurement::from_metrics(2048, 942, true, 100).unwrap();
    let modern = ArabicMeasurement::from_metrics(2048, 942, false, 100).unwrap();
    assert_eq!(legacy.advance_pt(500.0, 15.0) * 4096.0, 14991.0);
    assert_eq!(modern.advance_pt(500.0, 15.0) * 4096.0, 15000.0);

    // Native Traditional Arabic cursive attachment: the placement API
    // returns -6.024966 measurement pixels, converted to -5, not -6.
    // The 13/15pt printer equivalents -0.6454687/-0.74527144 become zero.
    let legacy = ArabicMeasurement::from_metrics(2048, 942, true, 103).unwrap();
    assert_eq!(legacy.advance_pt(-12.0, 13.0) * 4096.0, -266.0);
    assert_eq!(legacy.advance_pt(-12.0, 15.0) * 4096.0, -307.0);
    assert_eq!(super::glyph_advance(-6.024966), -5.0);
    assert_eq!(super::glyph_advance(-0.6454687), 0.0);
    assert_eq!(super::glyph_advance(-0.74527144), 0.0);
  }
}
