use kurbo::{PathEl, Point as KurboPoint, flatten};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::simple_type::DrawingmlPercentageValue;

use super::{
  Color, GradientFill, GradientInterpolation, GradientPath, GradientPathKind, GradientStop,
  PathCommand, Rect, RelativeRect, Transform,
};

const PATH_GRADIENT_BINARY_STEPS: usize = 10;
const OFFICE_DEFAULT_PATH_FOCUS: RelativeRect = RelativeRect {
  left: 0.5,
  top: 0.5,
  right: 0.5,
  bottom: 0.5,
};

/// Resolves shared DrawingML path-gradient defaults and tile geometry.
///
/// ISO/IEC 29500 gives each attribute on a present `CT_RelativeRect` a
/// schema default of zero. Office's shape-property merge defaults are a
/// separate layer: when `fillToRect` itself is absent, all four focus insets
/// are 50%. Keeping those cases distinct is observable in presets such as
/// "From Top Left Corner", which writes only `r` and `b`.
pub(crate) fn resolve_path_gradient(
  source: &a::GradientFill,
  path: &a::PathGradientFill,
  shape_transform: Transform,
) -> GradientPath {
  let fill_to_shape = normalize_focus_rect(
    path
      .fill_to_rectangle
      .as_ref()
      .map(|rect| {
        drawingml_relative_rect(
          rect.left.as_ref(),
          rect.top.as_ref(),
          rect.right.as_ref(),
          rect.bottom.as_ref(),
        )
      })
      .unwrap_or(OFFICE_DEFAULT_PATH_FOCUS),
  );
  let tile = source.tile_rectangle.as_ref().map(|rect| {
    drawingml_relative_rect(
      rect.left.as_ref(),
      rect.top.as_ref(),
      rect.right.as_ref(),
      rect.bottom.as_ref(),
    )
  });
  let kind = match path.path.unwrap_or(a::PathShadeValues::Shape) {
    a::PathShadeValues::Shape => GradientPathKind::Shape,
    a::PathShadeValues::Circle => GradientPathKind::Circle,
    a::PathShadeValues::Rectangle => GradientPathKind::Rectangle,
  };
  let (fill_to, transform, mirror_tile) =
    resolve_path_gradient_tile(fill_to_shape, tile, shape_transform);
  GradientPath {
    kind,
    fill_to,
    transform,
    mirror_tile,
  }
}

/// Binds a path-gradient transform expressed in owning-shape unit space to
/// final page bounds. This composes rather than replaces the transform so an
/// authored `tileRect` remains observable after deferred host layout.
pub(crate) fn bind_path_transform_to_bounds(normalized: Transform, bounds: Rect) -> Transform {
  Transform {
    m11: bounds.size.width.0 * normalized.m11,
    m12: bounds.size.height.0 * normalized.m12,
    m21: bounds.size.width.0 * normalized.m21,
    m22: bounds.size.height.0 * normalized.m22,
    dx: super::Pt(bounds.origin.x.0 + bounds.size.width.0 * normalized.dx.0),
    dy: super::Pt(bounds.origin.y.0 + bounds.size.height.0 * normalized.dy.0),
  }
}

fn drawingml_relative_rect(
  left: Option<&DrawingmlPercentageValue>,
  top: Option<&DrawingmlPercentageValue>,
  right: Option<&DrawingmlPercentageValue>,
  bottom: Option<&DrawingmlPercentageValue>,
) -> RelativeRect {
  let ratio = |value: Option<&DrawingmlPercentageValue>| {
    value.map(|value| value.as_ratio() as f32).unwrap_or(0.0)
  };
  RelativeRect {
    left: ratio(left),
    top: ratio(top),
    right: ratio(right),
    bottom: ratio(bottom),
  }
}

/// Converts the two authored edge positions into geometric focus bounds.
///
/// ISO/IEC 29500-1 §20.1.8.31 defines `fillToRect` as a rectangle whose
/// edges are offsets from the corresponding shape edges. Office-authored
/// content can place the nominal left edge to the right of the nominal right
/// edge (the equivalent VML uses a negative `focussize`). A rectangle is
/// orientation-independent, so retain its covered region while putting the
/// edges back into the monotonic form used by the path-gradient sampler.
pub(crate) fn normalize_focus_rect(rect: RelativeRect) -> RelativeRect {
  let authored_left = rect.left;
  let authored_top = rect.top;
  let authored_right = 1.0 - rect.right;
  let authored_bottom = 1.0 - rect.bottom;
  let left = authored_left.min(authored_right);
  let top = authored_top.min(authored_bottom);
  let right = authored_left.max(authored_right);
  let bottom = authored_top.max(authored_bottom);
  RelativeRect {
    left,
    top,
    right: 1.0 - right,
    bottom: 1.0 - bottom,
  }
}

fn resolve_path_gradient_tile(
  fill_to_shape: RelativeRect,
  tile: Option<RelativeRect>,
  shape_transform: Transform,
) -> (RelativeRect, Transform, bool) {
  let Some(tile) = tile else {
    return (fill_to_shape, shape_transform, false);
  };
  let tile_width = 1.0 - tile.left - tile.right;
  let tile_height = 1.0 - tile.top - tile.bottom;
  if tile_width.abs() <= f32::EPSILON || tile_height.abs() <= f32::EPSILON {
    return (fill_to_shape, shape_transform, false);
  }

  // ISO/IEC 29500-1 §20.1.8.59 maps the gradient to tileRect. Office keeps
  // fillToRect anchored to the owning shape, so convert the focus rectangle
  // to tile-unit coordinates before handing it to the common path sampler.
  let fill_to = RelativeRect {
    left: (fill_to_shape.left - tile.left) / tile_width,
    top: (fill_to_shape.top - tile.top) / tile_height,
    right: (fill_to_shape.right - tile.right) / tile_width,
    bottom: (fill_to_shape.bottom - tile.bottom) / tile_height,
  };
  let transform = Transform {
    m11: shape_transform.m11 * tile_width,
    m12: shape_transform.m12 * tile_width,
    m21: shape_transform.m21 * tile_height,
    m22: shape_transform.m22 * tile_height,
    dx: super::Pt(
      shape_transform.dx.0 + shape_transform.m11 * tile.left + shape_transform.m21 * tile.top,
    ),
    dy: super::Pt(
      shape_transform.dy.0 + shape_transform.m12 * tile.left + shape_transform.m22 * tile.top,
    ),
  };
  let mirror_tile = tile.left > 0.0 || tile.top > 0.0 || tile.right > 0.0 || tile.bottom > 0.0;
  (fill_to, transform, mirror_tile)
}

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
  let focus = normalize_focus_rect(path.fill_to);
  let focus_width = 1.0 - f64::from(focus.left) - f64::from(focus.right);
  let focus_height = 1.0 - f64::from(focus.top) - f64::from(focus.bottom);
  let scale_x = focus_width + (1.0 - focus_width) * outer_ratio;
  let scale_y = focus_height + (1.0 - focus_height) * outer_ratio;
  let offset_x = f64::from(focus.left) * (1.0 - outer_ratio);
  let offset_y = f64::from(focus.top) * (1.0 - outer_ratio);
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn present_relative_rect_uses_schema_defaults_before_tile_mapping() {
    let source = a::GradientFill {
      tile_rectangle: Some(a::TileRectangle {
        left: Some(DrawingmlPercentageValue::Decimal(-100_000)),
        top: Some(DrawingmlPercentageValue::Decimal(-100_000)),
        ..Default::default()
      }),
      ..Default::default()
    };
    let path = a::PathGradientFill {
      path: Some(a::PathShadeValues::Circle),
      fill_to_rectangle: Some(a::FillToRectangle {
        right: Some(DrawingmlPercentageValue::Decimal(100_000)),
        bottom: Some(DrawingmlPercentageValue::Decimal(100_000)),
        ..Default::default()
      }),
    };

    let resolved = resolve_path_gradient(&source, &path, Transform::default());

    assert_eq!(
      resolved.fill_to,
      RelativeRect {
        left: 0.5,
        top: 0.5,
        right: 0.5,
        bottom: 0.5,
      }
    );
    assert_eq!(resolved.transform.m11, 2.0);
    assert_eq!(resolved.transform.m22, 2.0);
    assert_eq!(resolved.transform.dx.0, -1.0);
    assert_eq!(resolved.transform.dy.0, -1.0);
    assert!(!resolved.mirror_tile);
  }

  #[test]
  fn absent_fill_to_rect_uses_office_shape_property_default() {
    let resolved = resolve_path_gradient(
      &a::GradientFill::default(),
      &a::PathGradientFill::default(),
      Transform::default(),
    );

    assert_eq!(resolved.fill_to, OFFICE_DEFAULT_PATH_FOCUS);
  }

  #[test]
  fn inverted_authored_edges_keep_the_same_geometric_focus_rectangle() {
    let source = a::GradientFill::default();
    let path = a::PathGradientFill {
      path: Some(a::PathShadeValues::Circle),
      fill_to_rectangle: Some(a::FillToRectangle {
        left: Some(DrawingmlPercentageValue::Decimal(20_000)),
        top: Some(DrawingmlPercentageValue::Decimal(50_000)),
        right: Some(DrawingmlPercentageValue::Decimal(100_000)),
        bottom: Some(DrawingmlPercentageValue::Decimal(50_000)),
      }),
    };

    let resolved = resolve_path_gradient(&source, &path, Transform::default());

    assert_eq!(
      resolved.fill_to,
      RelativeRect {
        left: 0.0,
        top: 0.5,
        right: 0.8,
        bottom: 0.5,
      }
    );
  }
}
