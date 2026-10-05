//! Office fixed-output paint replays a 600-DPI EMF at 3000 DPI.
//!
//! Native Word and Excel EnumEnhMetaFile/GetWorldTransform/LPtoDP captures
//! establish the exclusive EMF frame, FLOAT arithmetic and POINTFIX rounding.
//! This conversion belongs to paint output; it does not change source layout,
//! printer-font realization, font advances or pagination.

#[cfg(test)]
use crate::model::PageSetup;

const SOURCE_DPI: f64 = 600.0;
const OUTPUT_DPI: f64 = 3000.0;

#[derive(Clone, Copy)]
struct Axis {
  scale: f32,
}

impl Axis {
  fn new(viewport_dots: i32, reference_dots: i32, reference_mm: i32) -> Self {
    let frame = ((f64::from(viewport_dots) - 1.0) * 100.0 * f64::from(reference_mm)
      / f64::from(reference_dots))
    .round() as f32;
    // Preserve Windows' separate FLOAT divisions and multiplication. Computing
    // the equivalent ratio in f64 changes the A4 vertical matrix by two ULPs,
    // and changes an observed 31-output-dot border into a 30-dot border.
    let frame_ratio = (viewport_dots * 5) as f32 / frame;
    let reference_pitch = (reference_mm as f32 * 100.0) / reference_dots as f32;
    Self {
      scale: frame_ratio * reference_pitch,
    }
  }

  fn map_source(self, source_dots: f32) -> f32 {
    let mapped = source_dots * self.scale;
    // GDI first rounds to POINTFIX (28.4), then rounds that fixed point to a
    // device integer. Keep this order, including negative coordinate ties.
    let fixed = (f64::from(mapped) * 16.0).round();
    let output_dots = ((fixed + 8.0) / 16.0).floor();
    (output_dots * 72.0 / OUTPUT_DPI) as f32
  }

  fn map_points(self, points: f32) -> f32 {
    self.map_source((f64::from(points) * SOURCE_DPI / 72.0) as f32)
  }
}

#[derive(Clone, Copy)]
pub struct PageCoordinates {
  x: Axis,
  y: Axis,
}

impl PageCoordinates {
  pub fn for_standard_size(width_pt: f32, height_pt: f32) -> Option<Self> {
    let viewport =
      [width_pt, height_pt].map(|pt| (f64::from(pt) * SOURCE_DPI / 72.0).round() as i32);
    // Reference DC sizes from the configured Office printer: Letter, Legal,
    // A3, A4, A5 and JIS B5, in either orientation. Only select a profile when
    // the actual page has its device extent. Other/custom paper keeps its
    // existing paint path until its reference-DC selection is established;
    // deriving a reference DC from arbitrary page millimeters is incorrect.
    for (dots, mm) in [
      ([5100, 6600], [216, 279]),
      ([5100, 8400], [216, 356]),
      ([7016, 9921], [297, 420]),
      ([4961, 7016], [210, 297]),
      ([3496, 4961], [148, 210]),
      ([4299, 6071], [182, 257]),
    ] {
      for (dots, mm) in [(dots, mm), ([dots[1], dots[0]], [mm[1], mm[0]])] {
        if viewport == dots {
          return Some(Self {
            x: Axis::new(viewport[0], dots[0], mm[0]),
            y: Axis::new(viewport[1], dots[1], mm[1]),
          });
        }
      }
    }
    None
  }

  /// Replay an integral source printer reference without a point round trip.
  pub fn map_device_point(&self, x: i64, y: i64) -> (f32, f32) {
    (self.x.map_source(x as f32), self.y.map_source(y as f32))
  }

  /// Map a paint origin through the final PDF device transform. Source layout
  /// coordinates remain independent: this does not guess their earlier
  /// integer-device quantization or change font size, advances or line flow.
  pub fn map_point(&self, x: f32, y: f32) -> (f32, f32) {
    (self.x.map_points(x), self.y.map_points(y))
  }

  pub fn map_rectangle(&self, rect: (f32, f32, f32, f32)) -> (f32, f32, f32, f32) {
    let (x, y, width, height) = rect;
    let left = self.x.map_points(x);
    let top = self.y.map_points(y);
    let right = self.x.map_points(x + width);
    let bottom = self.y.map_points(y + height);
    (left, top, right - left, bottom - top)
  }

  /// Replay integral printer endpoints without a point-coordinate round trip.
  pub fn map_device_rectangle(&self, rect: (i64, i64, i64, i64)) -> (f32, f32, f32, f32) {
    let (x, y, width, height) = rect;
    let left = self.x.map_source(x as f32);
    let top = self.y.map_source(y as f32);
    let right = self.x.map_source((x + width) as f32);
    let bottom = self.y.map_source((y + height) as f32);
    (left, top, right - left, bottom - top)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn word_text_device_matches_native_emf_run_origins() {
    let device = PageCoordinates::for_standard_size(595.32, 841.92).unwrap();
    // Actual EMR_EXTTEXTOUTW references and PDF text matrices, independently
    // replayed across 1,105 ordinary runs. PDF decimal serialization is a
    // later boundary; retain the exact 3000-DPI point here.
    for (source, expected) in [
      ([2459.0, 628.0], [295.128, 75.36]),
      ([794.0, 856.0], [95.304, 102.744]),
      ([1213.0, 856.0], [145.584, 102.744]),
    ] {
      let actual = device.map_point(source[0] * 0.12, source[1] * 0.12);
      assert!((actual.0 - expected[0]).abs() < 0.00005);
      assert!((actual.1 - expected[1]).abs() < 0.00005);
    }
  }

  #[test]
  fn table_device_matches_native_float_matrix_and_endpoint_phase() {
    let x = Axis::new(4961, 4961, 210);
    let y = Axis::new(7016, 7016, 297);
    assert_eq!(x.scale, 5.0009527_f32);
    assert_eq!(y.scale, 5.000674_f32);
    // Actual Windows LPtoDP: the endpoints of this six-source-dot border
    // are 25,708 and 25,739, rather than an independently rounded width30.
    assert!((y.map_source(5141.0) - 616.992).abs() < 0.0001);
    assert!((y.map_source(5147.0) - y.map_source(5141.0) - 0.744).abs() < 0.0001);
    assert!((x.map_source(725.0) - 87.024).abs() < 0.00001);
  }

  #[test]
  fn table_device_custom_frame_retains_reference_device_pitch() {
    // A 600x800pt native control uses a Letter DC, not the page's own mm.
    let x = Axis::new(5000, 5100, 216);
    let y = Axis::new(6667, 6600, 279);
    assert_eq!(x.scale, 5.001056_f32);
    assert_eq!(y.scale, 5.00075_f32);
  }

  #[test]
  fn table_device_standard_profile_rotates_both_reference_axes() {
    let portrait = PageCoordinates::for_standard_size(
      PageSetup::default().width_pt,
      PageSetup::default().height_pt,
    )
    .unwrap();
    let mut setup = PageSetup::default();
    std::mem::swap(&mut setup.width_pt, &mut setup.height_pt);
    let landscape = PageCoordinates::for_standard_size(setup.width_pt, setup.height_pt).unwrap();
    assert_eq!(portrait.x.scale, landscape.y.scale);
    assert_eq!(portrait.y.scale, landscape.x.scale);
    setup.width_pt = 600.0;
    setup.height_pt = 800.0;
    assert!(PageCoordinates::for_standard_size(setup.width_pt, setup.height_pt).is_none());
  }
}
