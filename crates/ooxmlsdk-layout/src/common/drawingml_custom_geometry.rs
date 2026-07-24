use std::collections::HashMap;

use kurbo::{Affine, Arc, BezPath, ParamCurve};
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::PathCommand;

use super::drawingml_geometry::{append_transformed_arc, bez_path_commands};

/// Lowers DrawingML custom-geometry paths into page-space commands.
///
/// ECMA-376 gives each `a:path` its own coordinate space through `w` and `h`.
/// Guides are evaluated in that space, then points are scaled into the shape
/// frame. Unsupported commands reject the custom path as a unit so callers can
/// retain their existing conservative fallback.
pub(crate) fn path_commands(
  geometry: &a::CustomGeometry,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> Option<Vec<PathCommand>> {
  let mut output = BezPath::new();
  for path in &geometry.path_list.path {
    if matches!(path.fill, Some(a::PathFillModeValues::None)) {
      continue;
    }
    let viewport_width = path
      .width
      .map(|value| value.to_emu() as f64)
      .filter(|value| *value > 0.0)
      .unwrap_or(f64::from(width));
    let viewport_height = path
      .height
      .map(|value| value.to_emu() as f64)
      .filter(|value| *value > 0.0)
      .unwrap_or(f64::from(height));
    if viewport_width <= 0.0 || viewport_height <= 0.0 {
      return None;
    }
    let guides = evaluate_guides(geometry, viewport_width, viewport_height)?;
    let map_coordinates = Affine::new([
      f64::from(width) / viewport_width,
      0.0,
      0.0,
      f64::from(height) / viewport_height,
      f64::from(left),
      f64::from(top),
    ]);
    let point_coordinates = |point: &a::Point| -> Option<(f64, f64)> {
      Some((
        coordinate(&point.x, &guides, viewport_width, viewport_height)?,
        coordinate(&point.y, &guides, viewport_width, viewport_height)?,
      ))
    };
    let mut current = None::<(f64, f64)>;
    let mut subpath_start = None::<(f64, f64)>;
    for choice in &path.path_choice {
      match choice {
        a::PathChoice::CloseShapePath => {
          output.close_path();
          current = subpath_start;
        }
        a::PathChoice::MoveTo(move_to) => {
          let point = point_coordinates(&move_to.point)?;
          output.move_to(map_coordinates * kurbo::Point::new(point.0, point.1));
          current = Some(point);
          subpath_start = Some(point);
        }
        a::PathChoice::LineTo(line_to) => {
          let point = point_coordinates(&line_to.point)?;
          output.line_to(map_coordinates * kurbo::Point::new(point.0, point.1));
          current = Some(point);
        }
        a::PathChoice::QuadraticBezierCurveTo(curve) => {
          let [control, end] = curve.point.as_slice() else {
            return None;
          };
          let control = point_coordinates(control)?;
          let end = point_coordinates(end)?;
          current?;
          output.quad_to(
            map_coordinates * kurbo::Point::new(control.0, control.1),
            map_coordinates * kurbo::Point::new(end.0, end.1),
          );
          current = Some(end);
        }
        a::PathChoice::CubicBezierCurveTo(curve) => {
          let [control1, control2, end] = curve.point.as_slice() else {
            return None;
          };
          let control1 = point_coordinates(control1)?;
          let control2 = point_coordinates(control2)?;
          let end = point_coordinates(end)?;
          current?;
          output.curve_to(
            map_coordinates * kurbo::Point::new(control1.0, control1.1),
            map_coordinates * kurbo::Point::new(control2.0, control2.1),
            map_coordinates * kurbo::Point::new(end.0, end.1),
          );
          current = Some(end);
        }
        a::PathChoice::ArcTo(arc) => {
          let start = current?;
          let radius_x = coordinate(&arc.width_radius, &guides, viewport_width, viewport_height)?;
          let radius_y = coordinate(&arc.height_radius, &guides, viewport_width, viewport_height)?;
          let start_angle = coordinate(&arc.start_angle, &guides, viewport_width, viewport_height)?;
          let sweep_angle = coordinate(&arc.swing_angle, &guides, viewport_width, viewport_height)?;
          let arc = drawingml_arc(start, radius_x, radius_y, start_angle, sweep_angle)?;
          let end = arc.eval(1.0);
          append_transformed_arc(&mut output, arc, map_coordinates);
          current = Some((end.x, end.y));
        }
      }
    }
  }
  (!output.is_empty()).then(|| bez_path_commands(output))
}

/// ECMA-376 Part 1 §20.1.9.4 anchors the ellipse at the current pen position
/// and measures both angles clockwise in 60,000ths of a degree. Kurbo owns
/// the tolerance-bounded conversion from this analytical arc to cubic Béziers.
fn drawingml_arc(
  start: (f64, f64),
  radius_x: f64,
  radius_y: f64,
  start_angle: f64,
  sweep_angle: f64,
) -> Option<Arc> {
  if radius_x <= 0.0 || radius_y <= 0.0 || !sweep_angle.is_finite() {
    return None;
  }

  let start_radians = angle_radians(start_angle);
  let sweep_radians = angle_radians(sweep_angle);
  Some(Arc::new(
    (
      start.0 - radius_x * start_radians.cos(),
      start.1 - radius_y * start_radians.sin(),
    ),
    (radius_x, radius_y),
    start_radians,
    sweep_radians,
    0.0,
  ))
}

fn evaluate_guides(
  geometry: &a::CustomGeometry,
  width: f64,
  height: f64,
) -> Option<HashMap<String, f64>> {
  let mut guides = HashMap::new();
  for guide in geometry
    .adjust_value_list
    .iter()
    .flat_map(|list| &list.shape_guide)
    .chain(
      geometry
        .shape_guide_list
        .iter()
        .flat_map(|list| &list.shape_guide),
    )
  {
    let value = formula(&guide.formula, &guides, width, height)?;
    guides.insert(guide.name.clone(), value);
  }
  Some(guides)
}

fn formula(formula: &str, guides: &HashMap<String, f64>, width: f64, height: f64) -> Option<f64> {
  let tokens = formula.split_ascii_whitespace().collect::<Vec<_>>();
  let value = |index: usize| coordinate(tokens.get(index)?, guides, width, height);
  match *tokens.first()? {
    "val" => value(1),
    "*/" => safe_div(value(1)? * value(2)?, value(3)?),
    "+-" => Some(value(1)? + value(2)? - value(3)?),
    "+/" => safe_div(value(1)? + value(2)?, value(3)?),
    "?:" => Some(if value(1)? > 0.0 {
      value(2)?
    } else {
      value(3)?
    }),
    "abs" => Some(value(1)?.abs()),
    "at2" => Some(value(2)?.atan2(value(1)?).to_degrees() * 60_000.0),
    "cat2" => Some(value(1)? * value(3)?.atan2(value(2)?).cos()),
    "cos" => Some(value(1)? * angle_radians(value(2)?).cos()),
    "max" => Some(value(1)?.max(value(2)?)),
    "min" => Some(value(1)?.min(value(2)?)),
    "mod" => Some((value(1)?.powi(2) + value(2)?.powi(2) + value(3)?.powi(2)).sqrt()),
    "pin" => {
      let minimum = value(1)?;
      let maximum = value(3)?;
      Some(value(2)?.clamp(minimum.min(maximum), minimum.max(maximum)))
    }
    "sat2" => Some(value(1)? * value(3)?.atan2(value(2)?).sin()),
    "sin" => Some(value(1)? * angle_radians(value(2)?).sin()),
    "sqrt" => Some(value(1)?.max(0.0).sqrt()),
    "tan" => Some(value(1)? * angle_radians(value(2)?).tan()),
    _ => None,
  }
}

fn safe_div(numerator: f64, denominator: f64) -> Option<f64> {
  (denominator.abs() > f64::EPSILON).then_some(numerator / denominator)
}

fn angle_radians(value: f64) -> f64 {
  (value / 60_000.0).to_radians()
}

fn coordinate(value: &str, guides: &HashMap<String, f64>, width: f64, height: f64) -> Option<f64> {
  if let Ok(value) = value.parse::<f64>() {
    return Some(value);
  }
  if let Some(value) = guides.get(value) {
    return Some(*value);
  }
  let short_side = width.min(height);
  let long_side = width.max(height);
  match value {
    "l" | "t" => Some(0.0),
    "r" | "w" => Some(width),
    "b" | "h" => Some(height),
    "hc" => Some(width / 2.0),
    "vc" => Some(height / 2.0),
    "ss" => Some(short_side),
    "ls" => Some(long_side),
    "cd2" => Some(10_800_000.0),
    "cd3" => Some(7_200_000.0),
    "cd4" => Some(5_400_000.0),
    "cd8" => Some(2_700_000.0),
    "3cd4" => Some(16_200_000.0),
    "3cd8" => Some(8_100_000.0),
    "5cd8" => Some(13_500_000.0),
    "7cd8" => Some(18_900_000.0),
    _ => divided_builtin(value, "wd", width)
      .or_else(|| divided_builtin(value, "hd", height))
      .or_else(|| divided_builtin(value, "ssd", short_side)),
  }
}

fn divided_builtin(value: &str, prefix: &str, base: f64) -> Option<f64> {
  let divisor = value.strip_prefix(prefix)?.parse::<f64>().ok()?;
  safe_div(base, divisor)
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

  use super::{formula, path_commands};
  use crate::common::{PathCommand, Point, Pt};

  #[test]
  fn scales_custom_path_coordinates_into_the_shape_frame() {
    let geometry = a::CustomGeometry {
      path_list: a::PathList {
        path: vec![a::Path {
          width: Some("200".parse().unwrap()),
          height: Some("100".parse().unwrap()),
          path_choice: vec![
            a::PathChoice::MoveTo(Box::new(a::MoveTo {
              point: a::Point {
                x: "0".into(),
                y: "0".into(),
              },
            })),
            a::PathChoice::LineTo(Box::new(a::LineTo {
              point: a::Point {
                x: "200".into(),
                y: "50".into(),
              },
            })),
            a::PathChoice::CloseShapePath,
          ],
          ..Default::default()
        }],
      },
      ..Default::default()
    };

    assert_eq!(
      path_commands(&geometry, 10.0, 20.0, 100.0, 200.0).unwrap(),
      [
        PathCommand::MoveTo(Point {
          x: Pt(10.0),
          y: Pt(20.0)
        }),
        PathCommand::LineTo(Point {
          x: Pt(110.0),
          y: Pt(120.0)
        }),
        PathCommand::Close,
      ]
    );
  }

  #[test]
  fn evaluates_drawingml_guide_operators_in_path_coordinates() {
    let guides = HashMap::new();
    assert_eq!(formula("*/ w 1 2", &guides, 200.0, 100.0), Some(100.0));
    assert_eq!(formula("+- h 30 10", &guides, 200.0, 100.0), Some(120.0));
    assert_eq!(formula("pin 10 25 20", &guides, 200.0, 100.0), Some(20.0));
  }

  #[test]
  fn converts_clockwise_drawingml_arcs_from_the_current_pen_position() {
    let geometry = a::CustomGeometry {
      path_list: a::PathList {
        path: vec![a::Path {
          width: Some("200".parse().unwrap()),
          height: Some("100".parse().unwrap()),
          path_choice: vec![
            a::PathChoice::MoveTo(Box::new(a::MoveTo {
              point: a::Point {
                x: "200".into(),
                y: "50".into(),
              },
            })),
            a::PathChoice::ArcTo(Box::new(a::ArcTo {
              width_radius: "100".into(),
              height_radius: "50".into(),
              start_angle: "0".into(),
              swing_angle: "cd2".into(),
            })),
            a::PathChoice::CloseShapePath,
          ],
          ..Default::default()
        }],
      },
      ..Default::default()
    };

    let commands = path_commands(&geometry, 10.0, 20.0, 100.0, 200.0).unwrap();
    assert_eq!(commands.len(), 4);
    let PathCommand::CubicTo { end, .. } = commands[2] else {
      panic!("a half turn must end with a cubic segment");
    };
    assert!((end.x.0 - 10.0).abs() < 0.001);
    assert!((end.y.0 - 120.0).abs() < 0.001);
    assert_eq!(commands[3], PathCommand::Close);
  }
}
