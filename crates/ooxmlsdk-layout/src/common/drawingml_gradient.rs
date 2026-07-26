use kurbo::{PathEl, Point as KurboPoint, flatten};

use super::{
  Color, GradientFill, GradientInterpolation, GradientPath, GradientPathKind, GradientStop,
  PathCommand, Transform,
};

const PATH_GRADIENT_BINARY_STEPS: usize = 10;

pub(crate) fn resolved_stops(gradient: &GradientFill<'static>) -> Vec<GradientStop<'static>> {
  if gradient.interpolation != GradientInterpolation::PowerPointGammaSigma
    || gradient.stops.len() < 2
  {
    return gradient.stops.clone();
  }

  // Windows GDI+ SetSigmaBellShape(1, 1), retained as fixed samples so the
  // vector PDF and bounded effect raster use the same deterministic curve.
  const SIGMA_BLEND_U8: [u8; 33] = [
    0, 2, 5, 8, 12, 17, 22, 29, 36, 45, 54, 65, 76, 88, 101, 114, 128, 141, 154, 167, 179, 190,
    201, 210, 219, 226, 233, 238, 243, 247, 250, 253, 255,
  ];
  let mut stops = Vec::with_capacity((gradient.stops.len() - 1) * 32 + 1);
  for pair in gradient.stops.windows(2) {
    let start = &pair[0];
    let end = &pair[1];
    for (step, blend) in SIGMA_BLEND_U8[..32].iter().enumerate() {
      let position_ratio = step as f32 / 32.0;
      let blend = f32::from(*blend) / 255.0;
      stops.push(GradientStop {
        position: start.position + (end.position - start.position) * position_ratio,
        color: gamma_correct_color(start.color, end.color, blend),
        scheme: None,
      });
    }
  }
  stops.push(gradient.stops.last().expect("two gradient stops").clone());
  stops
}

pub(crate) fn sample(stops: &[GradientStop<'static>], position: f32) -> Color {
  let Some(first) = stops.first() else {
    return Color::default();
  };
  if position <= first.position {
    return first.color;
  }
  for pair in stops.windows(2) {
    let start = &pair[0];
    let end = &pair[1];
    if position <= end.position {
      let span = end.position - start.position;
      let ratio = if span.abs() <= f32::EPSILON {
        1.0
      } else {
        ((position - start.position) / span).clamp(0.0, 1.0)
      };
      let channel = |start: u8, end: u8| {
        (f32::from(start) + (f32::from(end) - f32::from(start)) * ratio)
          .round()
          .clamp(0.0, 255.0) as u8
      };
      return Color {
        r: channel(start.color.r, end.color.r),
        g: channel(start.color.g, end.color.g),
        b: channel(start.color.b, end.color.b),
        a: channel(start.color.a, end.color.a),
      };
    }
  }
  stops.last().map_or(first.color, |stop| stop.color)
}

pub(crate) fn inverse_point(transform: Transform, page_x: f64, page_y: f64) -> Option<KurboPoint> {
  let m11 = f64::from(transform.m11);
  let m12 = f64::from(transform.m12);
  let m21 = f64::from(transform.m21);
  let m22 = f64::from(transform.m22);
  let determinant = m11 * m22 - m12 * m21;
  if !determinant.is_finite() || determinant.abs() <= f64::from(f32::EPSILON) {
    return None;
  }
  let x = page_x - f64::from(transform.dx.0);
  let y = page_y - f64::from(transform.dy.0);
  Some(KurboPoint::new(
    (m22 * x - m21 * y) / determinant,
    (-m12 * x + m11 * y) / determinant,
  ))
}

pub(crate) fn position(
  path: GradientPath,
  mut point: KurboPoint,
  shape: Option<&[Vec<KurboPoint>]>,
) -> Option<f32> {
  if path.mirror_tile {
    point.x = mirrored_tile_coordinate(point.x);
    point.y = mirrored_tile_coordinate(point.y);
  }
  if !contains(path, point, 1.0, shape)? {
    return Some(1.0);
  }
  if contains(path, point, 0.0, shape)? {
    return Some(0.0);
  }
  let mut outside = 0.0;
  let mut inside = 1.0;
  for _ in 0..PATH_GRADIENT_BINARY_STEPS {
    let middle = (outside + inside) / 2.0;
    if contains(path, point, middle, shape)? {
      inside = middle;
    } else {
      outside = middle;
    }
  }
  // DrawingML path gradients number the focus path as stop position 0 and
  // grow toward the outer boundary at position 1. LibreOffice consequently
  // reverses the imported stop list when adapting this model to its
  // outer-to-inner BGradient API (oox/source/drawingml/fillproperties.cxx).
  Some(inside.clamp(0.0, 1.0) as f32)
}

fn mirrored_tile_coordinate(value: f64) -> f64 {
  let tile = value.floor();
  let fraction = value - tile;
  if tile.rem_euclid(2.0) < 1.0 {
    fraction
  } else {
    1.0 - fraction
  }
}

pub(crate) fn shape_polygons(
  commands: &[PathCommand],
  transform: Transform,
) -> Option<Vec<Vec<KurboPoint>>> {
  let mut elements = Vec::with_capacity(commands.len());
  for command in commands {
    match *command {
      PathCommand::MoveTo(point) => elements.push(PathEl::MoveTo(inverse_point(
        transform,
        f64::from(point.x.0),
        f64::from(point.y.0),
      )?)),
      PathCommand::LineTo(point) => elements.push(PathEl::LineTo(inverse_point(
        transform,
        f64::from(point.x.0),
        f64::from(point.y.0),
      )?)),
      PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => elements.push(PathEl::CurveTo(
        inverse_point(transform, f64::from(control1.x.0), f64::from(control1.y.0))?,
        inverse_point(transform, f64::from(control2.x.0), f64::from(control2.y.0))?,
        inverse_point(transform, f64::from(end.x.0), f64::from(end.y.0))?,
      )),
      PathCommand::Close => elements.push(PathEl::ClosePath),
    }
  }
  let mut polygons = Vec::new();
  let mut polygon = Vec::new();
  flatten(elements, 0.0005, |element| match element {
    PathEl::MoveTo(point) => {
      finish_polygon(&mut polygons, &mut polygon);
      polygon.push(point);
    }
    PathEl::LineTo(point) => polygon.push(point),
    PathEl::ClosePath => finish_polygon(&mut polygons, &mut polygon),
    PathEl::QuadTo(_, _) | PathEl::CurveTo(_, _, _) => {
      unreachable!("kurbo::flatten emits only lines")
    }
  });
  finish_polygon(&mut polygons, &mut polygon);
  (!polygons.is_empty()).then_some(polygons)
}

fn contains(
  path: GradientPath,
  point: KurboPoint,
  outer_ratio: f64,
  shape: Option<&[Vec<KurboPoint>]>,
) -> Option<bool> {
  let focus_width = 1.0 - f64::from(path.fill_to.left) - f64::from(path.fill_to.right);
  let focus_height = 1.0 - f64::from(path.fill_to.top) - f64::from(path.fill_to.bottom);
  let scale_x = focus_width + (1.0 - focus_width) * outer_ratio;
  let scale_y = focus_height + (1.0 - focus_height) * outer_ratio;
  let offset_x = f64::from(path.fill_to.left) * (1.0 - outer_ratio);
  let offset_y = f64::from(path.fill_to.top) * (1.0 - outer_ratio);
  if scale_x.abs() <= f64::EPSILON || scale_y.abs() <= f64::EPSILON {
    return Some(
      (point.x - offset_x).abs() <= f64::EPSILON && (point.y - offset_y).abs() <= f64::EPSILON,
    );
  }
  let base = KurboPoint::new(
    (point.x - offset_x) / scale_x,
    (point.y - offset_y) / scale_y,
  );
  Some(match path.kind {
    GradientPathKind::Circle => {
      let x = (base.x - 0.5) * 2.0;
      let y = (base.y - 0.5) * 2.0;
      x.mul_add(x, y * y) <= 1.0
    }
    GradientPathKind::Rectangle => (0.0..=1.0).contains(&base.x) && (0.0..=1.0).contains(&base.y),
    GradientPathKind::Shape => point_in_polygons(base, shape?),
  })
}

fn finish_polygon(polygons: &mut Vec<Vec<KurboPoint>>, polygon: &mut Vec<KurboPoint>) {
  if polygon.len() >= 3 {
    if polygon.first() != polygon.last() {
      polygon.push(polygon[0]);
    }
    polygons.push(std::mem::take(polygon));
  } else {
    polygon.clear();
  }
}

fn point_in_polygons(point: KurboPoint, polygons: &[Vec<KurboPoint>]) -> bool {
  let mut inside = false;
  for polygon in polygons {
    for edge in polygon.windows(2) {
      let (x1, y1) = (edge[0].x, edge[0].y);
      let (x2, y2) = (edge[1].x, edge[1].y);
      if (y1 > point.y) != (y2 > point.y) && point.x < (x2 - x1) * (point.y - y1) / (y2 - y1) + x1 {
        inside = !inside;
      }
    }
  }
  inside
}

fn gamma_correct_color(start: Color, end: Color, blend: f32) -> Color {
  let channel = |start: u8, end: u8| {
    let start = (f32::from(start) / 255.0).powf(2.2);
    let end = (f32::from(end) / 255.0).powf(2.2);
    ((start + (end - start) * blend).powf(1.0 / 2.2) * 255.0)
      .round()
      .clamp(0.0, 255.0) as u8
  };
  Color {
    r: channel(start.r, end.r),
    g: channel(start.g, end.g),
    b: channel(start.b, end.b),
    a: (f32::from(start.a) + (f32::from(end.a) - f32::from(start.a)) * blend)
      .round()
      .clamp(0.0, 255.0) as u8,
  }
}
