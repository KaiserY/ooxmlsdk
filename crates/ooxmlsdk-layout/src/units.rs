use ooxmlsdk::units as sdk_units;

pub const POINTS_PER_INCH: f32 = 72.0;
pub const MILLIMETERS_PER_INCH: f32 = 25.4;
pub const CENTIMETERS_PER_INCH: f32 = 2.54;
pub const POINTS_PER_CENTIMETER: f32 = POINTS_PER_INCH / CENTIMETERS_PER_INCH;
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
    Some(0.0) => OFFICE_VML_HAIRLINE_WIDTH_PT,
    Some(width) => width,
    None => default,
  }
}

#[inline]
pub fn quantize_points_to_office_print_grid(value: f32) -> f32 {
  (value * OFFICE_FIXED_OUTPUT_DPI / POINTS_PER_INCH).round() * POINTS_PER_INCH
    / OFFICE_FIXED_OUTPUT_DPI
}

/// A device-space allocation for a Word static-3-D bitmap.
///
/// Keep the origin as well as the dimensions: independently rounded edges
/// depend on the rectangle's position on the device lattice. The PDF image
/// rectangle is a separate, possibly inset rectangle and is not this input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WordStatic3dDeviceRasterRect {
  pub left: i32,
  pub top: i32,
  pub right: i32,
  pub bottom: i32,
}

/// Resolves one natural physical interval on Word's target-device grid.
/// Keep both edges: equal lengths at different origins can allocate differently.
pub(crate) fn word_static_3d_natural_raster_axis(
  bounds_pt: [f64; 2],
  dpi: u32,
) -> Option<(i32, i32)> {
  if dpi == 0 || bounds_pt.iter().any(|value| !value.is_finite()) || bounds_pt[1] <= bounds_pt[0] {
    return None;
  }
  let scale = f64::from(dpi) / f64::from(POINTS_PER_INCH);
  const SAMPLE_GUARD: f64 = 1.0 / 16.0;
  let near = (bounds_pt[0] * scale + SAMPLE_GUARD).floor();
  let far = (bounds_pt[1] * scale - SAMPLE_GUARD).ceil();
  (near >= f64::from(i32::MIN)
    && near <= f64::from(i32::MAX)
    && far >= f64::from(i32::MIN)
    && far <= f64::from(i32::MAX)
    && far > near)
    .then_some((near as i32, far as i32))
}

impl WordStatic3dDeviceRasterRect {
  /// Converts a natural physical rectangle `[left, top, right, bottom]` in
  /// points to its target-device allocation. Empty or unrepresentable
  /// rectangles are rejected rather than wrapped or silently saturated.
  ///
  /// Office's GEL bitmap tiler converts the rectangle through the device
  /// transform, then uses floor(near + 1/16) and ceil(far - 1/16).
  /// Actual Rasterizer construction and 318 exact-config Word controls cover
  /// this boundary; a length-only ceil + 1 misses 22 of those controls.
  pub fn from_natural_bounds(bounds_pt: [f64; 4], dpi: u32) -> Option<Self> {
    let (left, right) = word_static_3d_natural_raster_axis([bounds_pt[0], bounds_pt[2]], dpi)?;
    let (top, bottom) = word_static_3d_natural_raster_axis([bounds_pt[1], bounds_pt[3]], dpi)?;
    let rect = Self {
      left,
      top,
      right,
      bottom,
    };
    rect.dimensions()?;
    Some(rect)
  }

  pub fn dimensions(self) -> Option<(u32, u32)> {
    let width = u32::try_from(i64::from(self.right) - i64::from(self.left)).ok()?;
    let height = u32::try_from(i64::from(self.bottom) - i64::from(self.top)).ok()?;
    (width > 0 && height > 0).then_some((width, height))
  }
}

/// Compatibility allocation for callers which only retain PDF image extents.
///
/// This length-only approximation is not the general Office allocation rule.
/// New owners must retain the natural rectangle and use
/// [`WordStatic3dDeviceRasterRect`] instead. Do not feed an inset PDF rectangle
/// to that constructor to replace this formula without restoring its origin.
pub fn word_static_3d_fixed_output_raster_dimensions(
  width_pt: f32,
  height_pt: f32,
  dpi: u32,
) -> Option<(u32, u32)> {
  if !width_pt.is_finite()
    || width_pt <= 0.0
    || !height_pt.is_finite()
    || height_pt <= 0.0
    || dpi == 0
  {
    return None;
  }
  let dimension = |points: f32| {
    let pixels = (f64::from(points) * f64::from(dpi) / f64::from(POINTS_PER_INCH)).ceil() + 1.0;
    (pixels.is_finite() && pixels >= 1.0 && pixels <= f64::from(u32::MAX)).then_some(pixels as u32)
  };
  Some((dimension(width_pt)?, dimension(height_pt)?))
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

  #[test]
  fn word_static_3d_compatibility_allocation_retains_existing_output() {
    assert_eq!(
      word_static_3d_fixed_output_raster_dimensions(212.265, 133.8, 96),
      Some((285, 180))
    );
  }

  #[test]
  fn word_static_3d_device_allocation_matches_office_rectangles() {
    // The 26 distinct rectangles from 318 exact-config controls: original,
    // yaw 10/30 degrees, alphabet, glow, shadow, scale, and reflection.
    // Inputs are natural 600-DPI rectangles, not rounded PDF image extents.
    let cases = [
      ([177.60, 159.36, 212.64, 134.16], (285, 180)),
      ([171.00, 160.44, 222.36, 132.24], (297, 178)),
      ([186.96, 158.16, 196.44, 136.08], (263, 183)),
      ([177.60, 159.48, 212.64, 123.84], (285, 166)),
      ([177.60, 159.48, 212.64, 125.40], (285, 168)),
      ([177.60, 159.48, 212.64, 125.28], (285, 168)),
      ([177.60, 159.48, 212.64, 124.92], (285, 168)),
      ([177.60, 159.36, 212.64, 130.44], (285, 175)),
      ([177.60, 159.48, 212.64, 124.80], (285, 167)),
      ([177.60, 159.48, 212.64, 124.20], (285, 167)),
      ([177.60, 159.24, 212.64, 136.08], (285, 182)),
      ([177.60, 159.48, 212.64, 124.68], (285, 167)),
      ([177.60, 159.36, 212.64, 130.20], (285, 175)),
      ([177.60, 159.36, 212.64, 134.04], (285, 180)),
      ([177.60, 159.36, 212.64, 134.40], (285, 180)),
      ([177.60, 159.36, 212.64, 134.76], (285, 181)),
      ([177.60, 159.36, 212.64, 133.80], (285, 179)),
      ([177.60, 159.36, 212.64, 133.92], (285, 179)),
      ([177.60, 159.36, 212.64, 134.52], (285, 180)),
      ([177.60, 159.48, 212.64, 127.80], (285, 171)),
      ([177.60, 159.48, 212.64, 125.04], (285, 168)),
      ([177.60, 159.48, 212.64, 127.56], (285, 171)),
      ([177.60, 159.00, 212.64, 149.40], (285, 200)),
      ([177.60, 158.88, 212.64, 157.92], (285, 212)),
      ([177.60, 159.48, 212.64, 128.52], (285, 172)),
      ([177.60, 159.48, 212.64, 128.16], (285, 172)),
    ];
    for ([x, y, width, height], expected) in cases {
      let actual =
        WordStatic3dDeviceRasterRect::from_natural_bounds([x, y, x + width, y + height], 96)
          .unwrap();
      assert_eq!(actual.dimensions(), Some(expected));
    }
    // The old length-only formula is a genuine stopping counterexample.
    assert_eq!(
      word_static_3d_fixed_output_raster_dimensions(221.99, 131.87, 96),
      Some((297, 177))
    );
  }

  #[test]
  fn word_static_3d_device_allocation_retains_position_and_rounding_boundaries() {
    let rect = |left, right| {
      WordStatic3dDeviceRasterRect::from_natural_bounds([left, 0.0, right, 1.0], 72).unwrap()
    };
    // Equal widths can allocate differently when their origins differ.
    assert_eq!(rect(0.0, 1.0).dimensions(), Some((1, 1)));
    assert_eq!(rect(0.5, 1.5).dimensions(), Some((2, 1)));
    for shift in [-100.0, 0.0, 100.0] {
      let near = rect(shift + 0.9375, shift + 3.0);
      assert_eq!(near.left, shift as i32 + 1);
      assert_eq!(rect(shift + 0.9374, shift + 3.0).left, shift as i32);
      assert_eq!(rect(shift, shift + 2.0625).right, shift as i32 + 2);
      assert_eq!(rect(shift, shift + 2.0626).right, shift as i32 + 3);
    }
  }

  #[test]
  fn word_static_3d_device_allocation_rejects_invalid_and_empty_rectangles() {
    for bounds in [
      [0.0, 0.0, 0.0, 1.0],
      [1.0, 0.0, 0.0, 1.0],
      [0.0, 1.0, 1.0, 0.0],
      [0.0, 0.0, f64::NAN, 1.0],
      [f64::NEG_INFINITY, 0.0, 1.0, 1.0],
      [0.0, 0.0, f64::MAX, 1.0],
      [0.99, 0.0, 1.01, 1.0],
    ] {
      assert_eq!(
        WordStatic3dDeviceRasterRect::from_natural_bounds(bounds, 72),
        None
      );
    }
    assert_eq!(
      WordStatic3dDeviceRasterRect::from_natural_bounds([0.0, 0.0, 1.0, 1.0], 0),
      None
    );
  }
}
