use std::collections::HashMap;

use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::{PathCommand, Point, Pt};

/// Lowers DrawingML custom-geometry paths into page-space commands.
///
/// ECMA-376 gives each `a:path` its own coordinate space through `w` and `h`.
/// Guides are evaluated in that space, then points are scaled into the shape
/// frame. Unsupported commands reject the custom path as a unit so callers can
/// retain their existing conservative fallback.
pub(super) fn path_commands(
  geometry: &a::CustomGeometry,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> Option<Vec<PathCommand>> {
  let mut commands = Vec::new();
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
    let map_point = |point: &a::Point| -> Option<Point> {
      let x = coordinate(&point.x, &guides, viewport_width, viewport_height)?;
      let y = coordinate(&point.y, &guides, viewport_width, viewport_height)?;
      Some(Point {
        x: Pt(left + (x / viewport_width) as f32 * width),
        y: Pt(top + (y / viewport_height) as f32 * height),
      })
    };
    let mut current = None;
    for choice in &path.path_choice {
      match choice {
        a::PathChoice::CloseShapePath => {
          commands.push(PathCommand::Close);
          current = None;
        }
        a::PathChoice::MoveTo(move_to) => {
          let point = map_point(&move_to.point)?;
          commands.push(PathCommand::MoveTo(point));
          current = Some(point);
        }
        a::PathChoice::LineTo(line_to) => {
          let point = map_point(&line_to.point)?;
          commands.push(PathCommand::LineTo(point));
          current = Some(point);
        }
        a::PathChoice::QuadraticBezierCurveTo(curve) => {
          let [control, end] = curve.point.as_slice() else {
            return None;
          };
          let start = current?;
          let control = map_point(control)?;
          let end = map_point(end)?;
          commands.push(PathCommand::CubicTo {
            control1: interpolate(start, control, 2.0 / 3.0),
            control2: interpolate(end, control, 2.0 / 3.0),
            end,
          });
          current = Some(end);
        }
        a::PathChoice::CubicBezierCurveTo(curve) => {
          let [control1, control2, end] = curve.point.as_slice() else {
            return None;
          };
          let control1 = map_point(control1)?;
          let control2 = map_point(control2)?;
          let end = map_point(end)?;
          commands.push(PathCommand::CubicTo {
            control1,
            control2,
            end,
          });
          current = Some(end);
        }
        a::PathChoice::ArcTo(_) => return None,
      }
    }
  }
  (!commands.is_empty()).then_some(commands)
}

fn interpolate(from: Point, to: Point, amount: f32) -> Point {
  Point {
    x: Pt(from.x.0 + (to.x.0 - from.x.0) * amount),
    y: Pt(from.y.0 + (to.y.0 - from.y.0) * amount),
  }
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
}
