use kurbo::{Affine, Arc, BezPath, PathEl, QuadBez, Shape};

use crate::common::{PathCommand, Point, Pt};

// Kurbo's SVG arc lowering uses the same 0.1-unit tolerance when converting
// analytical arcs to cubic Béziers. DrawingML page coordinates are points, so
// keep that library-established curve tolerance here.
pub(crate) const CURVE_APPROXIMATION_TOLERANCE_PT: f64 = 0.1;

pub(crate) fn append_arc(path: &mut BezPath, arc: Arc) {
  arc.to_cubic_beziers(
    CURVE_APPROXIMATION_TOLERANCE_PT,
    |control1, control2, end| path.curve_to(control1, control2, end),
  );
}

pub(crate) fn append_transformed_arc(path: &mut BezPath, arc: Arc, transform: Affine) {
  arc.to_cubic_beziers(
    CURVE_APPROXIMATION_TOLERANCE_PT,
    |control1, control2, end| {
      path.curve_to(transform * control1, transform * control2, transform * end);
    },
  );
}

pub(crate) fn shape_commands<S: Shape>(shape: &S, reverse: bool) -> Vec<PathCommand> {
  let mut elements = shape
    .path_elements(CURVE_APPROXIMATION_TOLERANCE_PT)
    .collect::<Vec<_>>();
  if !matches!(elements.last(), Some(PathEl::ClosePath)) {
    elements.push(PathEl::ClosePath);
  }
  let path: BezPath = elements.into_iter().collect();
  let path = if reverse {
    path.reverse_subpaths()
  } else {
    path
  };
  bez_path_commands(path)
}

pub(crate) fn bez_path_commands(path: BezPath) -> Vec<PathCommand> {
  path_elements_to_commands(path.into_elements())
}

pub(crate) fn path_elements_to_commands(
  elements: impl IntoIterator<Item = PathEl>,
) -> Vec<PathCommand> {
  let mut commands = Vec::new();
  let mut current = None;
  let mut subpath_start = None;
  for element in elements {
    match element {
      PathEl::MoveTo(point) => {
        commands.push(PathCommand::MoveTo(common_point(point)));
        current = Some(point);
        subpath_start = Some(point);
      }
      PathEl::LineTo(point) => {
        commands.push(PathCommand::LineTo(common_point(point)));
        current = Some(point);
      }
      PathEl::QuadTo(control, end) => {
        let Some(start) = current else {
          continue;
        };
        let cubic = QuadBez::new(start, control, end).raise();
        commands.push(PathCommand::CubicTo {
          control1: common_point(cubic.p1),
          control2: common_point(cubic.p2),
          end: common_point(cubic.p3),
        });
        current = Some(end);
      }
      PathEl::CurveTo(control1, control2, end) => {
        commands.push(PathCommand::CubicTo {
          control1: common_point(control1),
          control2: common_point(control2),
          end: common_point(end),
        });
        current = Some(end);
      }
      PathEl::ClosePath => {
        commands.push(PathCommand::Close);
        current = subpath_start;
      }
    }
  }
  commands
}

pub(crate) fn mapped_path_elements(
  commands: &[PathCommand],
  map: impl Fn(Point) -> kurbo::Point,
) -> Vec<PathEl> {
  commands
    .iter()
    .map(|command| match *command {
      PathCommand::MoveTo(point) => PathEl::MoveTo(map(point)),
      PathCommand::LineTo(point) => PathEl::LineTo(map(point)),
      PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => PathEl::CurveTo(map(control1), map(control2), map(end)),
      PathCommand::Close => PathEl::ClosePath,
    })
    .collect()
}

pub(crate) fn transform_commands(
  commands: impl IntoIterator<Item = PathCommand>,
  transform: Affine,
) -> Vec<PathCommand> {
  let elements = commands.into_iter().map(|command| match command {
    PathCommand::MoveTo(point) => PathEl::MoveTo(transform * kurbo_point(point)),
    PathCommand::LineTo(point) => PathEl::LineTo(transform * kurbo_point(point)),
    PathCommand::CubicTo {
      control1,
      control2,
      end,
    } => PathEl::CurveTo(
      transform * kurbo_point(control1),
      transform * kurbo_point(control2),
      transform * kurbo_point(end),
    ),
    PathCommand::Close => PathEl::ClosePath,
  });
  path_elements_to_commands(elements)
}

pub(crate) fn transform_point(point: Point, transform: Affine) -> Point {
  common_point(transform * kurbo_point(point))
}

pub(crate) fn transform_rect_bounds(rect: kurbo::Rect, transform: Affine) -> kurbo::Rect {
  (transform * rect.to_path(0.0)).bounding_box()
}

pub(crate) fn transform_vector(vector: kurbo::Vec2, transform: Affine) -> kurbo::Vec2 {
  let [m11, m12, m21, m22, _, _] = transform.as_coeffs();
  kurbo::Vec2::new(
    m11 * vector.x + m21 * vector.y,
    m12 * vector.x + m22 * vector.y,
  )
}

pub(crate) fn group_child_affine(
  offset: kurbo::Point,
  extents: kurbo::Vec2,
  child_offset: kurbo::Point,
  child_extents: kurbo::Vec2,
) -> Affine {
  let scale_x = if child_extents.x != 0.0 {
    extents.x / child_extents.x
  } else {
    1.0
  };
  let scale_y = if child_extents.y != 0.0 {
    extents.y / child_extents.y
  } else {
    1.0
  };
  Affine::translate((-child_offset.x, -child_offset.y))
    .then_scale_non_uniform(scale_x, scale_y)
    .then_translate(offset.to_vec2())
}

pub(crate) fn common_transform(transform: Affine) -> crate::common::Transform {
  let [m11, m12, m21, m22, dx, dy] = transform.as_coeffs();
  crate::common::Transform {
    m11: m11 as f32,
    m12: m12 as f32,
    m21: m21 as f32,
    m22: m22 as f32,
    dx: Pt(dx as f32),
    dy: Pt(dy as f32),
  }
}

pub(crate) fn point_bounds(points: impl IntoIterator<Item = kurbo::Point>) -> Option<kurbo::Rect> {
  let mut points = points.into_iter();
  let first = points.next()?;
  Some(points.fold(
    kurbo::Rect::new(first.x, first.y, first.x, first.y),
    |bounds, point| bounds.union_pt(point),
  ))
}

pub(crate) fn kurbo_point(point: Point) -> kurbo::Point {
  kurbo::Point::new(f64::from(point.x.0), f64::from(point.y.0))
}

fn common_point(point: kurbo::Point) -> Point {
  Point {
    x: Pt(point.x as f32),
    y: Pt(point.y as f32),
  }
}

#[cfg(test)]
mod tests {
  use kurbo::{Affine, Point, Rect, Vec2};

  use super::{group_child_affine, transform_rect_bounds, transform_vector};

  #[test]
  fn group_child_affine_maps_declared_child_space_to_parent_extents() {
    let transform = group_child_affine(
      Point::new(10.0, 20.0),
      Vec2::new(200.0, 100.0),
      Point::new(1_000.0, 2_000.0),
      Vec2::new(4_000.0, 2_000.0),
    );
    let bounds = transform_rect_bounds(Rect::new(1_000.0, 2_000.0, 5_000.0, 4_000.0), transform);

    assert_eq!(bounds, Rect::new(10.0, 20.0, 210.0, 120.0));
  }

  #[test]
  fn vector_transform_uses_only_the_affine_linear_terms() {
    let transform = Affine::scale_non_uniform(2.0, 3.0).then_translate(Vec2::new(10.0, 20.0));

    assert_eq!(
      transform_vector(Vec2::new(4.0, 5.0), transform),
      Vec2::new(8.0, 15.0)
    );
  }
}
