//! Word's hosted orthographic scene allocation. Layout, the printer grid and
//! the continuous camera window are separate coordinate domains.

const SOURCE_DPI: f64 = 294_912.0; // Word fixed-format layout: 4096 units/point.
const PRINTER_DPI: f64 = 600.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct SourceFrame {
  local_twips: [f64; 4],
  parent: [f64; 2],
  insets_twips: [f64; 2],
}

impl SourceFrame {
  pub(super) fn new(
    local: [f32; 2],
    parent: [f32; 2],
    extent: [f32; 2],
    vertical_insets: [f32; 2],
  ) -> Option<Self> {
    if local
      .into_iter()
      .chain(parent)
      .chain(extent)
      .chain(vertical_insets)
      .any(|v| !v.is_finite())
      || extent.into_iter().any(|v| v <= 0.0)
    {
      return None;
    }
    Some(Self {
      local_twips: [
        local[0],
        local[1],
        local[0] + extent[0],
        local[1] + extent[1],
      ]
      .map(|v| (f64::from(v) * 20.0).round()),
      parent: parent.map(|v| (f64::from(v) * 4096.0).round()),
      insets_twips: vertical_insets.map(|v| (f64::from(v) * 20.0).round()),
    })
  }

  pub(super) fn translate(&mut self, dx: f32, dy: f32) {
    for (axis, delta) in [dx, dy].into_iter().enumerate() {
      self.parent[axis] += (f64::from(delta) * 4096.0).round();
    }
  }

  fn device_rect(self, dpi: f64) -> [f64; 4] {
    std::array::from_fn(|i| {
      (self.local_twips[i] * dpi / 1440.0).round() + (self.parent[i % 2] * dpi / SOURCE_DPI).round()
    })
  }

  pub(super) fn printer_point(self, local: [f32; 2]) -> [f32; 2] {
    let frame = self.device_rect(PRINTER_DPI);
    std::array::from_fn(|axis| {
      ((frame[axis] + (f64::from(local[axis]) * PRINTER_DPI / 72.0).round()) * 72.0 / PRINTER_DPI)
        as f32
    })
  }

  pub(super) fn orthographic_window(self) -> Window {
    let source = self.device_rect(SOURCE_DPI);
    let origin = [source[0], source[1]].map(|v| (v * 914_400.0 / SOURCE_DPI).round() / 914_400.0);
    let printer = self.device_rect(PRINTER_DPI);
    let s = 1.0 / PRINTER_DPI as f32;
    let inset = self
      .insets_twips
      .map(|v| (v * PRINTER_DPI / 1440.0).round() as f32);
    let bounds = [
      0.0,
      (printer[2] - printer[0]) as f32 * s,
      inset[0] * s,
      ((printer[3] - printer[1]) as f32 - inset[1]) * s,
      -1.0e-6,
      1.0e-6,
    ];
    // GFX converts the primitive translation by division, but the scene
    // translation by multiplication. Their distinct f32 rounding is visible
    // when the final bounding rectangle is rounded outward to printer dots.
    let mut node = identity();
    let mut source_transform = identity();
    source_transform[0] = s;
    source_transform[5] = s;
    for i in 0..2 {
      node[12 + i] = (printer[i] as f32 - (origin[i] / f64::from(s)) as f32) * s;
      source_transform[12 + i] = (-origin[i] * PRINTER_DPI) as f32 * s;
    }
    let b = transform_bounds(bounds, node);
    let mut center = identity();
    center[5] = -1.0;
    center[12] = -(b[0] + b[1]) * 0.5;
    center[13] = (b[2] + b[3]) * 0.5;
    // GFX's spherical camera constructor uses this stored single-precision
    // angle (one ULP below nearest PI/2), including its cosine residual.
    let c = f32::from_bits(0x3fc9_0fda).cos();
    let view = [
      1.0,
      -c,
      -c,
      0.0,
      c,
      1.0,
      0.0,
      0.0,
      -c,
      c * c,
      -1.0,
      0.0,
      0.0,
      0.0,
      1.0,
      1.0,
    ];
    let projected = transform_bounds(bounds, mul(node, mul(center, view)));
    let device_to_center = mul(source_transform, center);
    let near = transform([0.0; 3], device_to_center);
    let far = transform([2.0, 0.0, 0.0], device_to_center);
    let padding =
      ((far[0] - near[0]).powi(2) + (far[1] - near[1]).powi(2) + (far[2] - near[2]).powi(2)).sqrt();
    let vb = [
      projected[0] - padding,
      projected[2] - padding,
      projected[1] + padding,
      projected[3] + padding,
    ];
    let width = vb[2] - vb[0];
    let height = vb[3] - vb[1];
    let mut projection = identity();
    projection[0] = 2.0 / width;
    projection[5] = 2.0 / height;
    projection[12] = -(vb[0] + vb[2]) / width;
    projection[13] = -(vb[1] + vb[3]) / height;
    let combined = mul(view, projection);
    let aspect =
      ((f64::from(vb[2]) - f64::from(vb[0])) / (f64::from(vb[3]) - f64::from(vb[1]))) as f32;
    let near = transform([0.0; 3], combined);
    let far = transform([1.0, 0.0, 0.0], combined);
    let dx = far[0] - near[0];
    let dy = far[1] - near[1];
    let scale = (1.0 / (dy * dy / (aspect * aspect) + dx * dx)).sqrt();
    let mut shift = identity();
    shift[12] = -near[0];
    shift[13] = -near[1];
    let mut scaling = identity();
    scaling[0] = scale;
    scaling[5] = scale / aspect;
    let mut window = mul(shift, scaling);
    let b = transform_bounds(bounds, mul(node, center));
    let midpoint = [(b[0] + b[1]) * 0.5, (b[2] + b[3]) * 0.5, 0.0];
    let mapped = transform(transform(transform(midpoint, view), projection), window);
    let mut correction = identity();
    correction[12] = midpoint[0] - mapped[0];
    correction[13] = midpoint[1] - mapped[1];
    window = mul(window, correction);
    let corners = [
      transform([-1.0, -1.0, 0.0], window),
      transform([1.0, 1.0, 0.0], window),
    ];
    let device = mul(inverse_diagonal(center), inverse_diagonal(source_transform));
    let a = transform(corners[0], device);
    let b = transform(corners[1], device);
    let printer_bounds = [
      a[0].min(b[0]).floor(),
      a[1].min(b[1]).floor(),
      a[0].max(b[0]).ceil(),
      a[1].max(b[1]).ceil(),
    ];
    let a = transform(corners[0], inverse_diagonal(center));
    let b = transform(corners[1], inverse_diagonal(center));
    let physical_bounds = [
      a[0].min(b[0]),
      a[1].min(b[1]),
      a[0].max(b[0]),
      a[1].max(b[1]),
    ];
    Window {
      printer_bounds,
      physical_bounds_pt: std::array::from_fn(|i| {
        ((f64::from(physical_bounds[i]) + origin[i % 2]) * 72.0) as f32
      }),
    }
  }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct Window {
  pub printer_bounds: [f32; 4],
  pub physical_bounds_pt: [f32; 4],
}

impl Window {
  pub(super) fn pdf_bounds(self, target_dpi: f32) -> [f32; 4] {
    let [left, top, _, _] = self
      .printer_bounds
      .map(|v| f64::from(v) * 72.0 / PRINTER_DPI);
    // Native Screen/Print controls retain identical RGB/A8 and allocation,
    // but reserve half a target pixel at the far PDF edges once the page's
    // 2-D graphics context exists. Office writes those extents to 0.01pt.
    // Keep the integer printer rectangle until here: f32 point subtraction
    // can move an exact half-centipoint across the serialization boundary.
    let extent = |value: f64| ((value - 36.0 / f64::from(target_dpi)) * 100.0).round() / 100.0;
    let width = f64::from(self.printer_bounds[2] - self.printer_bounds[0]) * 72.0 / PRINTER_DPI;
    let height = f64::from(self.printer_bounds[3] - self.printer_bounds[1]) * 72.0 / PRINTER_DPI;
    [
      left as f32,
      top as f32,
      (left + extent(width)) as f32,
      (top + extent(height)) as f32,
    ]
  }
}

type Matrix = [f32; 16];
fn identity() -> Matrix {
  std::array::from_fn(|i| if i / 4 == i % 4 { 1.0 } else { 0.0 })
}
fn mul(a: Matrix, b: Matrix) -> Matrix {
  std::array::from_fn(|index| {
    let i = index / 4 * 4;
    let j = index % 4;
    ((a[i + 1] * b[4 + j] + a[i] * b[j]) + a[i + 2] * b[8 + j]) + a[i + 3] * b[12 + j]
  })
}
fn transform(p: [f32; 3], m: Matrix) -> [f32; 3] {
  let v: [f32; 4] =
    std::array::from_fn(|j| ((p[1] * m[4 + j] + p[0] * m[j]) + p[2] * m[8 + j]) + m[12 + j]);
  [v[0] / v[3], v[1] / v[3], v[2] / v[3]]
}
fn transform_bounds(b: [f32; 6], m: Matrix) -> [f32; 6] {
  let mut result = [
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::INFINITY,
    f32::NEG_INFINITY,
    f32::INFINITY,
    f32::NEG_INFINITY,
  ];
  for x in &b[..2] {
    for y in &b[2..4] {
      for z in &b[4..] {
        let p = transform([*x, *y, *z], m);
        for i in 0..3 {
          result[2 * i] = result[2 * i].min(p[i]);
          result[2 * i + 1] = result[2 * i + 1].max(p[i]);
        }
      }
    }
  }
  result
}
fn inverse_diagonal(a: Matrix) -> Matrix {
  let mut b = identity();
  for i in 0..3 {
    b[i * 5] = (1.0 / f64::from(a[i * 5])) as f32;
    b[12 + i] = (-f64::from(a[12 + i]) / f64::from(a[i * 5])) as f32;
  }
  b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn physical_glyph_origin_uses_local_printer_grid() {
    for (width, x, expected_x) in [(200.0, 81.33203, 153.36), (300.0, 131.33202, 203.28)] {
      for (phase, page_shift) in [(0.0, 0.0), (0.05, 0.0), (0.1, 0.12)] {
        let frame = SourceFrame::new(
          [72.0 + phase, 144.0 + phase],
          [0.0; 2],
          [width, 180.0],
          [3.6; 2],
        )
        .unwrap();
        let point = frame.printer_point([x, 48.412506]);
        assert!((point[0] - expected_x - page_shift).abs() < 0.0001);
        assert!((point[1] - 192.36 - page_shift).abs() < 0.0001);
      }
    }
  }

  #[test]
  fn initialized_graphics_context_changes_only_pdf_far_edges() {
    let window = SourceFrame::new([72.0, 144.0], [0.0; 2], [300.0, 180.0], [3.6; 2])
      .unwrap()
      .orthographic_window();
    for (dpi, width, height) in [(96.0, 300.23, 173.03), (200.0, 300.42, 173.22)] {
      let [left, top, right, bottom] = window.pdf_bounds(dpi);
      assert!((left - 71.76).abs() < 0.0001);
      assert!((top - 147.24).abs() < 0.0001);
      assert!((right - left - width).abs() < 0.0001);
      assert!((bottom - top - height).abs() < 0.0001);
    }
    assert_eq!(window.printer_bounds, [598.0, 1227.0, 3103.0, 2672.0]);
  }

  #[test]
  fn native_orthographic_source_rectangles() {
    for (width, height, phase, expected) in [
      (200.0, 120.0, 0.0, [597.0, 1228.0, 2269.0, 2172.0]),
      (200.0, 120.0, 0.05, [597.0, 1228.0, 2269.0, 2173.0]),
      (300.0, 180.0, 0.0, [598.0, 1227.0, 3103.0, 2672.0]),
      (300.0, 180.0, 0.05, [597.0, 1228.0, 3102.0, 2673.0]),
    ] {
      let frame = SourceFrame::new(
        [72.0 + phase, 144.0 + phase],
        [0.0; 2],
        [width, height],
        [3.6; 2],
      )
      .unwrap();
      assert_eq!(frame.orthographic_window().printer_bounds, expected);
    }
    let frame = SourceFrame::new(
      [-7.65, 15.7],
      [85.05, 243.649_22],
      [475.45, 376.2],
      [3.6; 2],
    )
    .unwrap();
    assert_eq!(
      frame.device_rect(PRINTER_DPI),
      [645.0, 2161.0, 4607.0, 5296.0]
    );
    assert_eq!(
      frame.orthographic_window().printer_bounds,
      [642.0, 2188.0, 4610.0, 5269.0]
    );
  }
}
