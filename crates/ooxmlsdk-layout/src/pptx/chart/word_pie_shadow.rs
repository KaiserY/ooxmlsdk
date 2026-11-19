//! Word's shadowed pie fits the far-plane shadow support before projection.

use super::{Chart3DView, PlotRect, RadialExtrusionProjection, RadialPerspectiveProjection};

pub(super) const MODEL_RADIUS: f64 = 100.0;
pub(super) const SHADOW_SUPPORT_OUTSET: f64 = 6.0;
const SUPPORT_ANGLE_STEP: f64 = 0.125_663_708;
const FRUSTUM_ENVELOPE: f64 = 1.028;

#[derive(Clone, Copy)]
struct Camera {
  eye_y: f64,
  distance: f64,
}

fn camera(viewport: (f64, f64), view: Chart3DView, explosion: f32) -> Camera {
  let angle = f64::from(view.rotate_x_deg.clamp(0.0, 90.0)).to_radians();
  let (sine, cosine) = angle.sin_cos();
  let fov = if view.perspective_half_degrees <= f32::EPSILON {
    0.1
  } else {
    f64::from(view.perspective_half_degrees) * 0.5
  };
  let tangent = (fov * 0.5).to_radians().tan();
  let tangent_x = tangent * viewport.0 / viewport.1;
  // The native support producer first realizes its model extents as integers.
  // The physical pie remains independent of these conservative fit vertices.
  // Explosion is authored in whole percentage points. Recover that integer
  // from the normalized f32; truncating 100 * 0.01 would incorrectly lose 1%.
  let radius = MODEL_RADIUS + (MODEL_RADIUS * f64::from(explosion.clamp(0.0, 1.0))).round();
  let half_height = (12.0 * f64::from(view.height_percent.clamp(5.0, 500.0)) / 100.0).trunc();
  let mut high = [f64::NEG_INFINITY; 2];
  let mut low = [f64::INFINITY; 2];
  let mut support = |x: f64, y: f64, z: f64| {
    // Each native support vertex is stored as f32 before camera fitting.
    let x = f64::from(x as f32);
    let y = f64::from(y as f32);
    let z = f64::from(z as f32);
    let projected_y = y * sine - z * cosine;
    let projected_z = y * cosine + z * sine;
    high[0] = high[0].max(x + tangent_x * projected_z);
    low[0] = low[0].min(x - tangent_x * projected_z);
    high[1] = high[1].max(projected_y + tangent * projected_z);
    low[1] = low[1].min(projected_y - tangent * projected_z);
  };
  // Eight cardinal vertices plus 25 reflected samples on each plane. Only
  // the far plane expands: enlarging both planes loses the native eye offset.
  let planes = [
    (-half_height, radius + SHADOW_SUPPORT_OUTSET),
    (half_height, radius),
  ];
  for (z, plane_radius) in planes {
    for (x, y) in [
      (plane_radius, 0.0),
      (-plane_radius, 0.0),
      (0.0, plane_radius),
      (0.0, -plane_radius),
    ] {
      support(x, y, z);
    }
  }
  for index in 0..25 {
    let (sine, cosine) = (f64::from(index) * SUPPORT_ANGLE_STEP).sin_cos();
    for (z, plane_radius) in planes {
      for x_sign in [-1.0, 1.0] {
        for y_sign in [-1.0, 1.0] {
          support(
            x_sign * plane_radius * sine,
            y_sign * plane_radius * cosine,
            z,
          );
        }
      }
    }
  }
  Camera {
    eye_y: (high[1] + low[1]) * 0.5 / MODEL_RADIUS,
    distance: ((high[0] - low[0]) / (2.0 * tangent_x)).max((high[1] - low[1]) / (2.0 * tangent))
      * FRUSTUM_ENVELOPE
      / MODEL_RADIUS,
  }
}

pub(super) fn projection(
  plot: PlotRect,
  view: Chart3DView,
  explosion: f32,
) -> Option<RadialPerspectiveProjection> {
  // Drawing-mode camera fitting uses the physical viewport. The object's
  // larger nominal viewport belongs to a different branch of the native fit.
  let fit_width = f64::from(plot.width);
  let fit_height = f64::from(plot.height);
  if view.right_angle_axes || fit_width <= 0.0 || fit_height <= 0.0 {
    return None;
  }
  let camera = camera((fit_width, fit_height), view, explosion);
  let (sine, cosine) = f64::from(view.rotate_x_deg.clamp(0.0, 90.0))
    .to_radians()
    .sin_cos();
  let thickness = 0.24 * f64::from((view.height_percent / 100.0).clamp(0.05, 5.0));
  let fov = if view.perspective_half_degrees <= f32::EPSILON {
    0.1
  } else {
    f64::from(view.perspective_half_degrees) * 0.5
  };
  let focal = f64::from(plot.height) / (2.0 * (fov * 0.5).to_radians().tan());
  let top_distance = camera.distance - thickness * sine * 0.5;
  let top_y = -thickness * cosine * 0.5 - camera.eye_y;
  let denominator = top_distance * top_distance - cosine * cosine;
  if !denominator.is_finite() || denominator <= 0.0 {
    return None;
  }
  let strength = cosine / top_distance;
  let radius_x = focal / denominator.sqrt();
  let radius_y = focal * (sine * top_distance + top_y * cosine) / denominator;
  let hub_offset = focal * top_y / top_distance;
  let depth_denominator = thickness * sine / top_distance;
  Some(RadialPerspectiveProjection {
    conic_center: (
      plot.left + plot.width * 0.5,
      (f64::from(plot.top + plot.height * 0.5) + hub_offset + radius_y * strength) as f32,
    ),
    radii: (radius_x as f32, radius_y as f32),
    strength: strength as f32,
    plane_offset: (0.0, 0.0),
    extrusion: Some(RadialExtrusionProjection {
      denominator: depth_denominator as f32,
      numerator: (focal * thickness * cosine / top_distance - hub_offset * depth_denominator)
        as f32,
    }),
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn shadow_support_retains_native_far_plane_and_camera_at_three_elevations() {
    for (elevation, eye_y, distance) in [
      (30.0, 1.572_606_2, 108_017.17),
      (70.0, 2.839_982_3, 118_852.61),
      (90.0, 0.0, 124_855.76),
    ] {
      let view = Chart3DView {
        rotate_x_deg: elevation,
        rotate_y_deg: 0.0,
        height_percent: 100.0,
        height_percent_is_explicit: false,
        depth_percent: 100.0,
        depth_percent_is_explicit: false,
        right_angle_axes: false,
        perspective_half_degrees: 0.0,
      };
      let actual = camera((384.175_433, 332.35), view, 0.0);
      let expected_eye = eye_y / MODEL_RADIUS;
      let expected_distance = distance / MODEL_RADIUS;
      assert!((actual.eye_y - expected_eye).abs() < 8.0 * f64::from(f32::EPSILON));
      assert!(
        (actual.distance - expected_distance).abs()
          < 4.0 * f64::from(f32::EPSILON) * expected_distance,
        "elevation={elevation}: {} vs {expected_distance}",
        actual.distance
      );
    }
  }
}
