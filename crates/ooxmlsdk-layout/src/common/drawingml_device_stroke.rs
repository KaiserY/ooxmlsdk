//! Device-width realization, before stroke widening, dashes and line ends.
//!
//! This is an Office paint policy, not a general Direct2D requirement. The
//! arithmetic was checked against the actual width-helper inputs and outputs
//! of 17 width and 12 rotation/position controls. Keep its caller-supplied
//! logical angle, minimum width and transform distinct from raster coverage.

/// Tests the exact DrawingML angle, not a rounded degree value or path slope.
/// This only answers the rotation condition; the owning paint state can
/// independently disable device snapping.
pub(crate) fn is_quarter_turn(rotation: i32) -> bool {
  rotation.rem_euclid(5_400_000) == 0
}

/// Retains the precision boundary between layout bounds and final device paint.
#[derive(Clone, Copy, Debug)]
pub(crate) enum TransformPrecision {
  // Bounds precision is a reference control; production realizes paint only.
  #[cfg(test)]
  Bounds,
  Paint,
}

/// A local-to-device linear transform. Translation does not affect pen width.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DeviceStrokeTransform {
  coefficients: [f64; 4],
}

impl DeviceStrokeTransform {
  /// Convert the *completed* device transform to the native paint precision.
  /// Converting a page-space scale first, then dividing by EMUs/point, changes
  /// half-pixel decisions. Bounds retain f64; native paint promotes f32 to f64.
  pub(crate) fn new(coefficients: [f64; 4], precision: TransformPrecision) -> Option<Self> {
    if !coefficients.iter().all(|value| value.is_finite()) {
      return None;
    }
    let coefficients = match precision {
      #[cfg(test)]
      TransformPrecision::Bounds => coefficients,
      TransformPrecision::Paint => coefficients.map(|value| f64::from(value as f32)),
    };
    coefficients
      .iter()
      .all(|value| value.is_finite())
      .then_some(Self { coefficients })
  }

  /// Return the realized width in the same local units as `width`.
  ///
  /// The inverse transform is applied to a device x-axis width vector, not
  /// to the transformed local direction. These differ for general affines.
  /// Preserve that distinction instead of reducing the matrix to one scale.
  pub(crate) fn realize_width(
    self,
    width: f64,
    snap: bool,
    minimum_pixels: f32,
    suppressed: bool,
  ) -> Option<f64> {
    if !width.is_finite() || width < 0.0 || !minimum_pixels.is_finite() {
      return None;
    }
    const MINIMUM_EPSILON: f32 = 1.0e-6;
    if suppressed || (!snap && minimum_pixels <= MINIMUM_EPSILON) {
      return Some(width);
    }
    let [a, b, c, d] = self.coefficients;
    let determinant = a * d - b * c;
    if !determinant.is_finite() {
      return None;
    }
    if determinant == 0.0 {
      return Some(width);
    }
    let x = a * width;
    let y = b * width;
    // Keep the observed multiply/add/sqrt evaluation, including its rounding
    // boundary. `hypot` or fused multiply-add need not round identically.
    let mut device_width = (x * x + y * y).sqrt();
    if !device_width.is_finite() {
      return None;
    }
    if minimum_pixels > MINIMUM_EPSILON {
      device_width = device_width.max(f64::from(minimum_pixels));
    }
    if snap {
      // CRT round: halfway away from zero, NOT ties-even or floor(x + .5).
      device_width = device_width.round().max(1.0);
    }
    let local_x = device_width * (d / determinant);
    let local_y = device_width * (-b / determinant);
    let realized = (local_x * local_x + local_y * local_y).sqrt();
    realized.is_finite().then_some(realized)
  }
}

impl super::DrawingMlDeviceStroke {
  pub(crate) fn realize(&mut self, dpi: f64, precision: TransformPrecision) -> Option<()> {
    if !dpi.is_finite() || dpi <= 0.0 {
      return None;
    }
    let transform = DeviceStrokeTransform::new(
      self.emu_to_points.map(|value| value * (dpi / 72.0)),
      precision,
    )?;
    self.realized_width_emu =
      Some(transform.realize_width(self.width_emu as f64, self.snap, 1.0, false)?);
    Some(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn transform(values: [f64; 4]) -> DeviceStrokeTransform {
    DeviceStrokeTransform::new(values, TransformPrecision::Bounds).unwrap()
  }

  #[test]
  fn device_stroke_exact_angle_does_not_round_near_quarter_turns() {
    for turn in -7..=7 {
      let angle = turn * 5_400_000;
      assert!(is_quarter_turn(angle));
      assert!(!is_quarter_turn(angle - 1));
      assert!(!is_quarter_turn(angle + 1));
    }
    assert!(!is_quarter_turn(i32::MIN));
    assert!(!is_quarter_turn(i32::MAX));
  }

  #[test]
  fn device_stroke_bounds_and_paint_keep_distinct_half_pixel_decisions() {
    let scale = 600.0 / 914_400.0;
    let values = [0.0, scale, scale, 0.0];
    let bounds = DeviceStrokeTransform::new(values, TransformPrecision::Bounds).unwrap();
    let paint = DeviceStrokeTransform::new(values, TransformPrecision::Paint).unwrap();
    // Authored widths are EMUs. A prior conversion to f32 points would erase
    // precisely the .3/.9/1.5pt boundaries exercised by these controls.
    for (width, bounds_pixels, paint_pixels) in [
      (3810.0, 3.0, 2.0),
      (11430.0, 8.0, 7.0),
      (19050.0, 13.0, 12.0),
    ] {
      assert_eq!(
        bounds.realize_width(width, true, 1.0, false),
        Some(bounds_pixels / scale)
      );
      assert_eq!(
        paint.realize_width(width, true, 1.0, false),
        Some(paint_pixels / f64::from(scale as f32))
      );
    }
  }

  #[test]
  fn device_stroke_minimum_suppression_singular_and_exact_rounding() {
    let identity = transform([1.0, 0.0, 0.0, 1.0]);
    assert_eq!(identity.realize_width(0.0, true, 1.0, false), Some(1.0));
    assert_eq!(identity.realize_width(0.1, false, 1.0, false), Some(1.0));
    assert_eq!(identity.realize_width(0.0, true, 1.0, true), Some(0.0));
    assert_eq!(
      identity.realize_width(2.25, false, 1.0e-6, false),
      Some(2.25)
    );
    assert_eq!(identity.realize_width(2.5, true, 1.0, false), Some(3.0));
    assert_eq!(
      identity.realize_width(2.5_f64.next_down(), true, 1.0, false),
      Some(2.0)
    );
    assert_eq!(
      transform([1.0, 2.0, 2.0, 4.0]).realize_width(0.1, true, 1.0, false),
      Some(0.1)
    );
    assert!(identity.realize_width(f64::NAN, true, 1.0, false).is_none());
    assert!(identity.realize_width(-1.0, true, 1.0, false).is_none());
    assert!(
      identity
        .realize_width(1.0, true, f32::INFINITY, false)
        .is_none()
    );
    assert!(DeviceStrokeTransform::new([f64::MAX; 4], TransformPrecision::Paint).is_none());
  }

  #[test]
  fn device_stroke_affine_inverse_is_not_a_scalar_scale() {
    let matrix = transform([2.0, 1.0, 3.0, 4.0]);
    let expected = ((4.0_f64 / 5.0).powi(2) + (-1.0_f64 / 5.0).powi(2)).sqrt();
    assert_eq!(matrix.realize_width(0.25, true, 1.0, false), Some(expected));
    let reflected = transform([-2.0, -1.0, 3.0, 4.0]);
    assert_eq!(
      reflected.realize_width(0.25, true, 1.0, false),
      Some(expected)
    );
  }
}
