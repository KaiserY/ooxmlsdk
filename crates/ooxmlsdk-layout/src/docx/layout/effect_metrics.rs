//! Word run-effect layout metrics, independent of the sampled effect graph.
//!
//! MS-DOCX specifies the authored transforms, not their Line Services extent.
//! Read-only WWLIB/GEL metric-query captures establish the synthetic rectangle,
//! normalization domain, effect ordering and integer outsets used here. The
//! reference and printer queries must be evaluated independently: rounding the
//! printer result back into reference units changes the 18pt line baseline.

use kurbo::{Affine, Rect};

use crate::common::drawingml_image_effects::{
  WordprocessingTextGlow, WordprocessingTextReflection, WordprocessingTextShadow,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Outsets {
  pub(super) left: i64,
  pub(super) top: i64,
  pub(super) right: i64,
  pub(super) bottom: i64,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Query {
  pub(super) font_size_pt: f64,
  pub(super) outline_width_pt: f64,
  pub(super) glow: Option<WordprocessingTextGlow>,
  pub(super) shadow: Option<WordprocessingTextShadow>,
  pub(super) reflection: Option<WordprocessingTextReflection>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Extent {
  pub(super) source: Rect,
  pub(super) output: Rect,
  pub(super) outsets: Outsets,
}

/// Printer-device run metrics, before and after effect outsets. This state is
/// independent of the reference-unit metrics which placed the line baseline.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DeviceRun {
  pub(super) ascent: i64,
  pub(super) descent: i64,
  pub(super) outsets: Outsets,
}

impl DeviceRun {
  /// Word first unions compatible run cells, then removes the effect insets.
  /// A non-positive rectangle clears BOTH metrics and uses path bounds. Do
  /// not substitute a glyph name or a font-relative short-string threshold.
  pub(super) fn reflection_metrics(self, union_advance: i64) -> Option<(i64, i64)> {
    let width = union_advance
      .checked_sub(self.outsets.left)?
      .checked_sub(self.outsets.right)?;
    let height = self.ascent.checked_add(self.descent)?;
    (width > 0 && height > 0).then_some((self.ascent, self.descent))
  }
}

impl Query {
  /// Ordinary horizontal run orientation. The caller retains responsibility
  /// for orientation, reference/device font metrics and run aggregation.
  pub(super) fn extent(self, ascent: i64, descent: i64, dpi: i64) -> Option<Extent> {
    if ascent < 0
      || descent < 0
      || dpi <= 0
      || !self.font_size_pt.is_finite()
      || self.font_size_pt < 0.0
      || !self.outline_width_pt.is_finite()
    {
      return None;
    }
    // CHP stores the font in half-points but this GEL query receives its
    // integer point size. Outline width is the complete width, not half-width.
    let font = self.font_size_pt.floor();
    let outline =
      (self.outline_width_pt.max(0.0) * 1_000.0).round() / 1_000.0 * 12_700.0 * dpi as f64
        / 914_400.0;
    let a = ascent as f64 + outline;
    let d = descent as f64 + outline;
    let source = Rect::new(-a, -a, a, d);
    let mut output = source;
    if let Some(glow) = self.glow {
      let radius = normalized_length(authored_length(glow.radius_px, dpi), font, 1.0);
      // Metric support is half the normalized radius times the GEL extent
      // factor. It is NOT the sampled Gaussian's sigma or visible support.
      output = outset(output, radius * 0.5 * 1.5);
    }
    if let Some(shadow) = self.shadow {
      let radius = normalized_length(authored_length(shadow.blur_radius_px, dpi), font, 0.4);
      let distance = normalized_length(authored_length(shadow.distance_px, dpi), font, 0.4);
      let transform = effect_transform(
        source,
        shadow.alignment,
        (shadow.scale_x, shadow.scale_y),
        (shadow.skew_x_degrees, shadow.skew_y_degrees),
        distance,
        shadow.direction_degrees,
      );
      output = output.union(outset(transform.transform_rect_bbox(output), radius));
    }
    if let Some(reflection) = self.reflection {
      let font_scale = if font == 0.0 { 1.0 } else { font / 72.0 };
      let radius = authored_length(reflection.blur_radius_px, dpi) * font_scale;
      let mut distance = authored_length(reflection.distance_px, dpi) * font_scale;
      if a != 0.0 && font * d / a != 0.0 {
        distance = authored_length(reflection.distance_px, dpi) * (font * d / a) * 0.25;
      }
      // Reflection consumes the completed paint horizontally, but uses the
      // original metric reference vertically, with descent removed. Spatial
      // siblings must not replace that vertical reference.
      let anchor = Rect::new(output.x0, source.y0, output.x1, source.y1 - d);
      let transform = effect_transform(
        anchor,
        reflection.alignment,
        (reflection.scale_x, reflection.scale_y),
        (reflection.skew_x_degrees, reflection.skew_y_degrees),
        distance,
        reflection.direction_degrees,
      );
      let mut reflected = transform.transform_rect_bbox(output);
      if f64::from(reflection.end_opacity) < 1.0 / 255.0 {
        let ramp = transform.transform_rect_bbox(anchor);
        // Preserve subtraction before multiplication: the 48pt device
        // capture distinguishes this order by one floating-point ULP.
        reflected.y1 = (ramp.y1 - ramp.y0) * authored_percentage(reflection.end_position) + ramp.y0;
      }
      output = output.union(outset(reflected, radius));
    }
    if !source.is_finite() || !output.is_finite() {
      return None;
    }
    let positive =
      |v: f64| -> Option<i64> { (v < i64::MAX as f64).then(|| v.max(0.0).trunc() as i64) };
    Some(Extent {
      source,
      output,
      outsets: Outsets {
        left: positive(source.x0 - output.x0)?,
        top: positive(source.y0 - output.y0)?,
        right: positive(output.x1 - source.x1)?,
        bottom: positive(output.y1 - source.y1)?,
      },
    })
  }
}

fn outset(r: Rect, radius: f64) -> Rect {
  Rect::new(r.x0 - radius, r.y0 - radius, r.x1 + radius, r.y1 + radius)
}

fn authored_length(css_pixels: f32, dpi: i64) -> f64 {
  // W14's Word wrapper resolves lengths to thousandths of a point before
  // forming the query on the selected device. Ignore raster-length scales.
  let millipoints = (f64::from(css_pixels).max(0.0) * 72.0 / 96.0 * 1_000.0).round();
  millipoints / 1_000.0 * 12_700.0 * dpi as f64 / 914_400.0
}

fn authored_percentage(value: f32) -> f64 {
  (f64::from(value) * 100_000.0).round() / 100_000.0
}

fn authored_angle(value: f32) -> f64 {
  let degrees = (f64::from(value) * 60_000.0).round() / 60_000.0;
  degrees * std::f64::consts::PI / 180.0
}

fn normalized_length(length: f64, font: f64, floor: f64) -> f64 {
  if font == 0.0 {
    length
  } else if length <= floor {
    0.0
  } else {
    (font.powf(0.7) / 72.0_f64.powf(0.7)) * (length - floor) + floor
  }
}

fn effect_transform(
  anchor: Rect,
  alignment: (f32, f32),
  scale: (f32, f32),
  skew: (f32, f32),
  distance: f64,
  direction: f32,
) -> Affine {
  let x = anchor.x0 + anchor.width() * f64::from(alignment.0);
  let y = anchor.y0 + anchor.height() * f64::from(alignment.1);
  let sx = authored_percentage(scale.0);
  let sy = authored_percentage(scale.1);
  let kx = authored_angle(skew.0).tan();
  let ky = authored_angle(skew.1).tan();
  let direction = authored_angle(direction);
  Affine::new([
    sx,
    ky,
    kx,
    sy,
    (direction.cos() * distance + x) - (sx * x + kx * y),
    (direction.sin() * distance + y) - (ky * x + sy * y),
  ])
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::drawingml_image_effects::ResolvedEffectColor;
  use crate::docx::RgbColor;

  #[test]
  fn word_run_device_inset_validity_and_terminal_cell_are_independent() {
    let run = DeviceRun {
      ascent: 286,
      descent: 81,
      outsets: Outsets {
        left: 394,
        right: 84,
        top: 187,
        bottom: 326,
      },
    };
    assert_eq!(run.reflection_metrics(1137), Some((286, 81)));
    assert_eq!(run.reflection_metrics(221), None);
    assert_eq!(run.reflection_metrics(478), None);
    assert_eq!(run.reflection_metrics(479), Some((286, 81)));
    // A compatible terminal cell can make an otherwise empty inset valid;
    // a paint-only gradient advance is not permission to merge this cell.
    assert_eq!(run.reflection_metrics(450), None);
    assert_eq!(run.reflection_metrics(450 + 75), Some((286, 81)));
    assert_eq!(
      DeviceRun {
        ascent: 0,
        descent: 0,
        ..run
      }
      .reflection_metrics(1137),
      None
    );
  }

  fn captured_query(font_size_pt: f64) -> Query {
    let color = ResolvedEffectColor {
      color: RgbColor { r: 0, g: 0, b: 0 },
      alpha: 255,
    };
    Query {
      font_size_pt,
      outline_width_pt: 2.0,
      glow: Some(WordprocessingTextGlow {
        radius_px: 10.0 * 96.0 / 72.0,
        raster_length_scale: 1.0,
        geometry_length_scale: 1.0,
        color,
      }),
      shadow: Some(WordprocessingTextShadow {
        blur_radius_px: 10.0 * 96.0 / 72.0,
        distance_px: 62.0 * 96.0 / 72.0,
        raster_length_scale: 1.0,
        geometry_length_scale: 1.0,
        direction_degrees: 212.0,
        scale_x: 0.7,
        scale_y: 0.7,
        skew_x_degrees: 0.0,
        skew_y_degrees: 0.0,
        alignment: (0.0, 0.5),
        color,
      }),
      reflection: Some(WordprocessingTextReflection {
        blur_radius_px: 11.0 * 96.0 / 72.0,
        raster_length_scale: 1.0,
        geometry_length_scale: 1.0,
        start_opacity: 0.4,
        start_position: 0.0,
        end_opacity: 0.0,
        end_position: 0.85,
        distance_px: 5.0 * 96.0 / 72.0,
        distance_length_scale: 1.0,
        direction_degrees: 90.0,
        fade_direction_degrees: 90.0,
        scale_x: 1.0,
        scale_y: -1.0,
        skew_x_degrees: 0.0,
        skew_y_degrees: 0.0,
        alignment: (0.0, 1.0),
      }),
    }
  }

  #[test]
  fn reference_and_device_metric_outsets_match_office_font_controls() {
    let cases = [
      (
        18.0,
        294_912,
        70_200,
        19_800,
        [116_541, 22_905, 58_706, 82_813],
      ),
      (
        24.0,
        294_912,
        93_600,
        26_400,
        [143_782, 29_256, 70_862, 108_708],
      ),
      (
        36.0,
        294_912,
        140_400,
        39_600,
        [193_551, 41_438, 91_833, 160_322],
      ),
      (
        48.0,
        294_912,
        187_200,
        52_800,
        [239_213, 53_166, 109_892, 211_843],
      ),
      (18.0, 600, 143, 40, [237, 47, 120, 168]),
      (24.0, 600, 190, 54, [293, 59, 144, 221]),
      (36.0, 600, 286, 81, [394, 84, 187, 326]),
      (48.0, 600, 381, 107, [486, 108, 223, 430]),
    ];
    for (font, dpi, a, d, [left, right, top, bottom]) in cases {
      let result = captured_query(font).extent(a, d, dpi).unwrap();
      assert_eq!(
        result.outsets,
        Outsets {
          left,
          top,
          right,
          bottom
        },
        "font={font}, dpi={dpi}"
      );
    }
  }

  #[test]
  fn metric_identity_and_transparent_tail_have_separate_extent_owners() {
    let mut query = captured_query(36.0);
    query.glow = None;
    query.shadow = None;
    query.reflection = None;
    assert_eq!(
      query.extent(286, 81, 600).unwrap().outsets,
      Outsets::default()
    );
    let mut query = captured_query(36.0);
    let transparent = query.extent(286, 81, 600).unwrap();
    query.reflection.as_mut().unwrap().end_opacity = 1.0;
    let opaque = query.extent(286, 81, 600).unwrap();
    assert!(opaque.output.y1 > transparent.output.y1);
    assert_eq!(opaque.output.x0, transparent.output.x0);
    assert_eq!(opaque.output.x1, transparent.output.x1);
  }
}
