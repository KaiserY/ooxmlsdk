//! The chart lathe consumes bevel width/depth with its own circular profile.
//! Native Circle/Angle/SoftRound controls have identical body meshes; their
//! legend keys retain the distinct DrawingML presets. The lathe normalizes
//! points by plot width and flattens its quarter ellipse in the radius100
//! source model, using the shared GFX/WPF HFD curve flattener.

use super::{
  Point3, Projector, RadialSlice, RgbColor, Scene, cut_light, lighting, pie_ring,
  word_chart_lathe_maximum_step,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

#[derive(Clone, Copy, Debug)]
struct Ring {
  radius: f64,
  z: f64,
  normal_r: f64,
  normal_z: f64,
}

#[derive(Default)]
pub(super) struct Bevels {
  bottom: Vec<Ring>,
  top: Vec<Ring>,
}

impl Bevels {
  pub(super) fn resolve(shape: &a::Shape3DType, width: f32, thickness: f64) -> Option<Self> {
    if width <= 0.0 {
      return None;
    }
    let bevel = |width_emu: Option<i64>, height_emu: Option<i64>, bottom: bool| {
      let w = crate::units::emu_to_points(width_emu.unwrap_or(76_200)) / width;
      let h = crate::units::emu_to_points(height_emu.unwrap_or(76_200)) / width;
      profile(f64::from(w), f64::from(h), thickness, bottom)
    };
    Some(Self {
      bottom: match &shape.bevel_bottom {
        Some(source) => bevel(
          source.width.map(|v| v.to_emu()),
          source.height.map(|v| v.to_emu()),
          true,
        )?,
        None => Vec::new(),
      },
      top: match &shape.bevel_top {
        Some(source) => bevel(
          source.width.map(|v| v.to_emu()),
          source.height.map(|v| v.to_emu()),
          false,
        )?,
        None => Vec::new(),
      },
    })
  }

  pub(super) fn active(&self) -> bool {
    !self.top.is_empty() || !self.bottom.is_empty()
  }
}

fn profile(w: f64, h: f64, thickness: f64, bottom: bool) -> Option<Vec<Ring>> {
  if w <= 0.0 || h <= 0.0 {
    return Some(Vec::new());
  }
  if w >= 1.0 || h > thickness * 0.5 {
    return None;
  }
  let k = 4.0 / 3.0 * (std::f64::consts::PI / 8.0).tan();
  let points = if bottom {
    [[0.0, 0.0], [k * w, 0.0], [w, (1.0 - k) * h], [w, h]]
  } else {
    [[0.0, 0.0], [0.0, k * h], [(1.0 - k) * w, h], [w, h]]
  };
  let parameters = crate::common::drawingml_3d::flatten_cubic_profile_parameters(
    points.map(|p| p.map(|v| v * 100.0)),
    0.01,
  );
  Some(
    parameters
      .into_iter()
      .map(|t| {
        let t = f64::from(t);
        let s = 1.0 - t;
        let [p0, p1, p2, p3] = points;
        let p: [f64; 2] = std::array::from_fn(|i| {
          s.powi(3) * p0[i] + 3.0 * s * s * t * p1[i] + 3.0 * s * t * t * p2[i] + t.powi(3) * p3[i]
        });
        let tangent: [f64; 2] = std::array::from_fn(|i| {
          3.0 * s * s * (p1[i] - p0[i])
            + 6.0 * s * t * (p2[i] - p1[i])
            + 3.0 * t * t * (p3[i] - p2[i])
        });
        let length = tangent[0].hypot(tangent[1]);
        Ring {
          radius: if bottom { 1.0 - w + p[0] } else { 1.0 - p[0] },
          z: if bottom { -thickness + p[1] } else { -h + p[1] },
          normal_r: tangent[1] / length,
          normal_z: tangent[0] / length * if bottom { -1.0 } else { 1.0 },
        }
      })
      .collect(),
  )
}

pub(super) fn body(
  scene: &mut Scene,
  projector: &Projector,
  slice: &RadialSlice,
  paint: (RgbColor, Option<usize>),
  bounds: [f64; 4],
  bevels: &Bevels,
) {
  let (color, material) = paint;
  let segments = (slice.sweep.abs() / word_chart_lathe_maximum_step(1.0))
    .ceil()
    .max(1.0) as usize;
  let arc = pie_ring(slice.start_angle, slice.sweep, segments);
  let mut rings = bevels.bottom.clone();
  if rings.is_empty() {
    rings.push(Ring {
      radius: 1.0,
      z: -projector.thickness,
      normal_r: 0.0,
      normal_z: -1.0,
    });
  }
  rings.push(Ring {
    normal_r: 1.0,
    normal_z: 0.0,
    ..*rings.last().unwrap()
  });
  rings.push(bevels.top.first().copied().unwrap_or(Ring {
    radius: 1.0,
    z: 0.0,
    normal_r: 1.0,
    normal_z: 0.0,
  }));
  rings.extend_from_slice(&bevels.top);
  let bottom = rings[0];
  let top = *rings.last().unwrap();
  let point = |r: Ring, p: Point3| [p[0] * r.radius, p[1] * r.radius, r.z];
  let normal = |ring: Ring, angle: f32| {
    let (s, c) = angle.sin_cos();
    [
      (ring.normal_r * f64::from(s)) as f32,
      (-ring.normal_r * projector.sine * f64::from(c) - ring.normal_z * projector.cosine) as f32,
      (ring.normal_r * projector.cosine * f64::from(c) - ring.normal_z * projector.sine) as f32,
    ]
  };
  let bottom_hub = [0.0, 0.0, -projector.thickness];
  let top_hub = [0.0; 3];
  let bottom_light = lighting(color, [0.0, projector.cosine as f32, projector.sine as f32]);
  let top_light = lighting(
    color,
    [0.0, -projector.cosine as f32, -projector.sine as f32],
  );
  // Native index order keeps the bottom fan, profile bands and top fan in
  // that order for both opaque and translucent points.
  for pair in arc.windows(2) {
    scene.face(
      projector,
      [
        bottom_hub,
        point(bottom, pair[1].1),
        point(bottom, pair[0].1),
      ],
      [bottom_light; 3],
      material,
      bounds,
    );
  }
  for band in rings.windows(2) {
    let [first, last] = [band[0], band[1]];
    if first.radius == last.radius && first.z == last.z {
      continue;
    }
    for pair in arc.windows(2) {
      let [(a, p), (b, q)] = [pair[0], pair[1]];
      let vertices = [
        point(first, p),
        point(last, p),
        point(first, q),
        point(last, q),
      ];
      let lights = [
        lighting(color, normal(first, a)),
        lighting(color, normal(last, a)),
        lighting(color, normal(first, b)),
        lighting(color, normal(last, b)),
      ];
      scene.face(
        projector,
        [vertices[0], vertices[2], vertices[1]],
        [lights[0], lights[2], lights[1]],
        material,
        bounds,
      );
      scene.face(
        projector,
        [vertices[2], vertices[3], vertices[1]],
        [lights[2], lights[3], lights[1]],
        material,
        bounds,
      );
    }
  }
  for pair in arc.windows(2) {
    scene.face(
      projector,
      [top_hub, point(top, pair[0].1), point(top, pair[1].1)],
      [top_light; 3],
      material,
      bounds,
    );
  }
  for (direction, (angle, outer)) in [(1.0, arc[0]), (-1.0, arc[segments])] {
    let light = cut_light(
      color,
      angle,
      projector.sine.atan2(projector.cosine).to_degrees() as f32,
      direction,
    );
    let mut polygon = vec![bottom_hub];
    polygon.extend(rings.iter().map(|r| point(*r, outer)));
    polygon.push(top_hub);
    for i in 1..polygon.len() - 1 {
      // The sector interior is on the increasing-angle side at its start.
      // Cut lighting uses the native inward normal; visible-face winding
      // must independently point outside that interior.
      let triangle = if direction < 0.0 {
        [polygon[0], polygon[i + 1], polygon[i]]
      } else {
        [polygon[0], polygon[i], polygon[i + 1]]
      };
      scene.face(projector, triangle, [light; 3], material, bounds);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::profile;

  #[test]
  fn pie_bevel_uses_the_native_plot_width_and_hfd_profile() {
    let plot_width = 291.2323622047244;
    let top = profile(20.5 / plot_width, 4.0 / plot_width, 0.24, false).unwrap();
    let bottom = profile(6.0 / plot_width, 6.0 / plot_width, 0.24, true).unwrap();
    assert_eq!(top.len(), 21);
    assert_eq!(bottom.len(), 17);
    assert!((top[0].z * 100.0 + 1.373474).abs() < 0.00002);
    assert!((top.last().unwrap().radius * 100.0 - 92.960945).abs() < 0.00002);
    assert!((bottom[0].radius * 100.0 - 97.939789).abs() < 0.00002);
    assert!((bottom[8].z * 100.0 + 23.396578).abs() < 0.00002);
    assert!(
      top
        .iter()
        .all(|r| (r.normal_r.hypot(r.normal_z) - 1.0).abs() < 1e-12)
    );
  }
}
