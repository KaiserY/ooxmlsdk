use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::{PathCommand, Point, Pt};

pub(super) fn path_commands(
  preset: Option<&a::PresetGeometry>,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> Vec<PathCommand> {
  let Some(preset) = preset else {
    return rectangle_path(left, top, width, height);
  };
  match preset.preset {
    a::ShapeTypeValues::RoundRectangle => {
      let adjustment = adjustment(preset, 0, 16_667.0).clamp(0.0, 50_000.0);
      let radius = width.min(height) * adjustment / 100_000.0;
      round_rectangle_path(left, top, width, height, radius)
    }
    a::ShapeTypeValues::RightArrow => {
      let thickness = adjustment(preset, 0, 50_000.0);
      let head = adjustment(preset, 1, 50_000.0);
      right_arrow_path(left, top, width, height, thickness, head)
    }
    a::ShapeTypeValues::DownArrow => {
      let thickness = adjustment(preset, 0, 50_000.0);
      let head = adjustment(preset, 1, 50_000.0);
      down_arrow_path(left, top, width, height, thickness, head)
    }
    _ => rectangle_path(left, top, width, height),
  }
}

fn adjustment(preset: &a::PresetGeometry, index: usize, default: f32) -> f32 {
  preset
    .adjust_value_list
    .as_ref()
    .and_then(|list| list.shape_guide.get(index))
    .and_then(|guide| guide_value(guide.formula.as_str()))
    .unwrap_or(default)
}

fn guide_value(formula: &str) -> Option<f32> {
  formula
    .strip_prefix("val ")
    .unwrap_or(formula)
    .parse::<f32>()
    .ok()
}

fn point(x: f32, y: f32) -> Point {
  Point { x: Pt(x), y: Pt(y) }
}

fn polygon(points: impl IntoIterator<Item = Point>) -> Vec<PathCommand> {
  let mut points = points.into_iter();
  let Some(first) = points.next() else {
    return Vec::new();
  };
  let mut commands = vec![PathCommand::MoveTo(first)];
  commands.extend(points.map(PathCommand::LineTo));
  commands.push(PathCommand::Close);
  commands
}

fn rectangle_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  let right = left + width;
  let bottom = top + height;
  polygon([
    point(left, top),
    point(right, top),
    point(right, bottom),
    point(left, bottom),
  ])
}

fn round_rectangle_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  radius: f32,
) -> Vec<PathCommand> {
  if radius <= f32::EPSILON {
    return rectangle_path(left, top, width, height);
  }
  let right = left + width;
  let bottom = top + height;
  // Four cubic Bézier arcs are the standard vector representation of the
  // quarter-circle arcs in the DrawingML roundRect preset.
  const KAPPA: f32 = 0.552_284_8;
  let tangent = radius * KAPPA;
  vec![
    PathCommand::MoveTo(point(left + radius, top)),
    PathCommand::LineTo(point(right - radius, top)),
    PathCommand::CubicTo {
      control1: point(right - radius + tangent, top),
      control2: point(right, top + radius - tangent),
      end: point(right, top + radius),
    },
    PathCommand::LineTo(point(right, bottom - radius)),
    PathCommand::CubicTo {
      control1: point(right, bottom - radius + tangent),
      control2: point(right - radius + tangent, bottom),
      end: point(right - radius, bottom),
    },
    PathCommand::LineTo(point(left + radius, bottom)),
    PathCommand::CubicTo {
      control1: point(left + radius - tangent, bottom),
      control2: point(left, bottom - radius + tangent),
      end: point(left, bottom - radius),
    },
    PathCommand::LineTo(point(left, top + radius)),
    PathCommand::CubicTo {
      control1: point(left, top + radius - tangent),
      control2: point(left + radius - tangent, top),
      end: point(left + radius, top),
    },
    PathCommand::Close,
  ]
}

fn right_arrow_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  thickness_adjustment: f32,
  head_adjustment: f32,
) -> Vec<PathCommand> {
  let short_side = width.min(height);
  if short_side <= 0.0 || height <= 0.0 {
    return rectangle_path(left, top, width, height);
  }
  // ECMA-376 preset rightArrow guide equations, mirrored by LibreOffice's
  // oox/source/drawingml/customshapes/presetShapeDefinitions.xml.
  let thickness = thickness_adjustment.clamp(0.0, 100_000.0);
  let max_head = 100_000.0 * width / short_side;
  let head = head_adjustment.clamp(0.0, max_head);
  let right = left + width;
  let bottom = top + height;
  let center_y = top + height / 2.0;
  let head_left = right - short_side * head / 100_000.0;
  let half_shaft = height * thickness / 200_000.0;
  let shaft_top = center_y - half_shaft;
  let shaft_bottom = center_y + half_shaft;
  polygon([
    point(left, shaft_top),
    point(head_left, shaft_top),
    point(head_left, top),
    point(right, center_y),
    point(head_left, bottom),
    point(head_left, shaft_bottom),
    point(left, shaft_bottom),
  ])
}

fn down_arrow_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  thickness_adjustment: f32,
  head_adjustment: f32,
) -> Vec<PathCommand> {
  let short_side = width.min(height);
  if short_side <= 0.0 || width <= 0.0 {
    return rectangle_path(left, top, width, height);
  }
  // ECMA-376 preset downArrow uses the same two guide concepts with the axes
  // exchanged; keep the declared preset path order instead of rotating a
  // right-arrow approximation.
  let thickness = thickness_adjustment.clamp(0.0, 100_000.0);
  let max_head = 100_000.0 * height / short_side;
  let head = head_adjustment.clamp(0.0, max_head);
  let right = left + width;
  let bottom = top + height;
  let center_x = left + width / 2.0;
  let head_top = bottom - short_side * head / 100_000.0;
  let half_shaft = width * thickness / 200_000.0;
  let shaft_left = center_x - half_shaft;
  let shaft_right = center_x + half_shaft;
  polygon([
    point(left, head_top),
    point(shaft_left, head_top),
    point(shaft_left, top),
    point(shaft_right, top),
    point(shaft_right, head_top),
    point(right, head_top),
    point(center_x, bottom),
  ])
}

#[cfg(test)]
mod tests {
  use super::{down_arrow_path, right_arrow_path};
  use crate::common::{PathCommand, Point, Pt};

  fn line_points(commands: &[PathCommand]) -> Vec<Point> {
    commands
      .iter()
      .filter_map(|command| match command {
        PathCommand::MoveTo(point) | PathCommand::LineTo(point) => Some(*point),
        PathCommand::CubicTo { .. } | PathCommand::Close => None,
      })
      .collect()
  }

  #[test]
  fn default_right_arrow_matches_the_ecma_preset_path() {
    assert_eq!(
      line_points(&right_arrow_path(
        0.0, 0.0, 100.0, 100.0, 50_000.0, 50_000.0
      )),
      [
        Point {
          x: Pt(0.0),
          y: Pt(25.0)
        },
        Point {
          x: Pt(50.0),
          y: Pt(25.0)
        },
        Point {
          x: Pt(50.0),
          y: Pt(0.0)
        },
        Point {
          x: Pt(100.0),
          y: Pt(50.0)
        },
        Point {
          x: Pt(50.0),
          y: Pt(100.0)
        },
        Point {
          x: Pt(50.0),
          y: Pt(75.0)
        },
        Point {
          x: Pt(0.0),
          y: Pt(75.0)
        },
      ]
    );
  }

  #[test]
  fn default_down_arrow_matches_the_ecma_preset_path() {
    assert_eq!(
      line_points(&down_arrow_path(0.0, 0.0, 100.0, 100.0, 50_000.0, 50_000.0)),
      [
        Point {
          x: Pt(0.0),
          y: Pt(50.0)
        },
        Point {
          x: Pt(25.0),
          y: Pt(50.0)
        },
        Point {
          x: Pt(25.0),
          y: Pt(0.0)
        },
        Point {
          x: Pt(75.0),
          y: Pt(0.0)
        },
        Point {
          x: Pt(75.0),
          y: Pt(50.0)
        },
        Point {
          x: Pt(100.0),
          y: Pt(50.0)
        },
        Point {
          x: Pt(50.0),
          y: Pt(100.0)
        },
      ]
    );
  }
}
