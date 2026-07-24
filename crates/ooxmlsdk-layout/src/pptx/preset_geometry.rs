//! DrawingML preset-shape geometry.
//!
//! Adjustment defaults and guide equations follow the preset definitions in
//! ECMA-376 Part 1, §20.1.10.56 (`prstGeom`) and its preset shape definitions.
//! These values are schema geometry data, not Office-golden layout tuning.

use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::{PathCommand, Point, Pt};

// Four cubic Bézier quarters approximate a circle with
// 4 / 3 * tan(π / 8), the standard kappa control distance.
const CIRCLE_CUBIC_BEZIER_KAPPA: f32 = 0.552_284_8;

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
    a::ShapeTypeValues::StraightConnector1 => straight_connector_path(left, top, width, height),
    a::ShapeTypeValues::BentConnector2 => bent_connector_2_path(left, top, width, height),
    a::ShapeTypeValues::BentConnector3 => {
      bent_connector_3_path(left, top, width, height, adjustment(preset, 0, 50_000.0))
    }
    a::ShapeTypeValues::BentConnector4 => bent_connector_4_path(
      left,
      top,
      width,
      height,
      adjustment(preset, 0, 50_000.0),
      adjustment(preset, 1, 50_000.0),
    ),
    a::ShapeTypeValues::BentConnector5 => bent_connector_5_path(
      left,
      top,
      width,
      height,
      adjustment(preset, 0, 50_000.0),
      adjustment(preset, 1, 50_000.0),
      adjustment(preset, 2, 50_000.0),
    ),
    a::ShapeTypeValues::CurvedConnector2 => curved_connector_2_path(left, top, width, height),
    a::ShapeTypeValues::CurvedConnector3 => {
      curved_connector_3_path(left, top, width, height, adjustment(preset, 0, 50_000.0))
    }
    a::ShapeTypeValues::CurvedConnector4 => curved_connector_4_path(
      left,
      top,
      width,
      height,
      adjustment(preset, 0, 50_000.0),
      adjustment(preset, 1, 50_000.0),
    ),
    a::ShapeTypeValues::CurvedConnector5 => curved_connector_5_path(
      left,
      top,
      width,
      height,
      adjustment(preset, 0, 50_000.0),
      adjustment(preset, 1, 50_000.0),
      adjustment(preset, 2, 50_000.0),
    ),
    a::ShapeTypeValues::Line => straight_connector_path(left, top, width, height),
    a::ShapeTypeValues::LineInverse => inverse_line_path(left, top, width, height),
    a::ShapeTypeValues::Ellipse => ellipse_path(left, top, width, height),
    a::ShapeTypeValues::Triangle => {
      triangle_path(left, top, width, height, adjustment(preset, 0, 50_000.0))
    }
    a::ShapeTypeValues::RightTriangle => right_triangle_path(left, top, width, height),
    a::ShapeTypeValues::Diamond => diamond_path(left, top, width, height),
    a::ShapeTypeValues::FlowChartExtract => flow_chart_extract_path(left, top, width, height),
    a::ShapeTypeValues::Hexagon => {
      hexagon_path(left, top, width, height, adjustment(preset, 0, 25_000.0))
    }
    a::ShapeTypeValues::LeftArrow => left_arrow_path(
      left,
      top,
      width,
      height,
      adjustment(preset, 0, 50_000.0),
      adjustment(preset, 1, 50_000.0),
    ),
    a::ShapeTypeValues::UpArrow => up_arrow_path(
      left,
      top,
      width,
      height,
      adjustment(preset, 0, 50_000.0),
      adjustment(preset, 1, 50_000.0),
    ),
    a::ShapeTypeValues::Parallelogram => {
      parallelogram_path(left, top, width, height, adjustment(preset, 0, 25_000.0))
    }
    a::ShapeTypeValues::Trapezoid => {
      trapezoid_path(left, top, width, height, adjustment(preset, 0, 25_000.0))
    }
    a::ShapeTypeValues::Plus => {
      plus_path(left, top, width, height, adjustment(preset, 0, 25_000.0))
    }
    a::ShapeTypeValues::Star5 => {
      star_5_path(left, top, width, height, adjustment(preset, 0, 19_098.0))
    }
    a::ShapeTypeValues::Donut => {
      donut_path(left, top, width, height, adjustment(preset, 0, 25_000.0))
    }
    a::ShapeTypeValues::FlowChartDelay => flow_chart_delay_path(left, top, width, height),
    a::ShapeTypeValues::Frame => {
      frame_path(left, top, width, height, adjustment(preset, 0, 12_500.0))
    }
    a::ShapeTypeValues::WedgeRoundRectangleCallout => wedge_round_rectangle_callout_path(
      left,
      top,
      width,
      height,
      adjustment(preset, 0, -20_833.0),
      adjustment(preset, 1, 62_500.0),
      adjustment(preset, 2, 16_667.0),
    ),
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

fn open_path(points: impl IntoIterator<Item = Point>) -> Vec<PathCommand> {
  let mut points = points.into_iter();
  let Some(first) = points.next() else {
    return Vec::new();
  };
  let mut commands = vec![PathCommand::MoveTo(first)];
  commands.extend(points.map(PathCommand::LineTo));
  commands
}

fn adjusted_axis(origin: f32, extent: f32, adjustment: f32) -> f32 {
  origin + extent * adjustment / 100_000.0
}

fn straight_connector_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  open_path([point(left, top), point(left + width, top + height)])
}

fn inverse_line_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  open_path([point(left, top + height), point(left + width, top)])
}

fn ellipse_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  let right = left + width;
  let bottom = top + height;
  let center_x = left + width / 2.0;
  let center_y = top + height / 2.0;
  let tangent_x = width / 2.0 * CIRCLE_CUBIC_BEZIER_KAPPA;
  let tangent_y = height / 2.0 * CIRCLE_CUBIC_BEZIER_KAPPA;
  vec![
    PathCommand::MoveTo(point(left, center_y)),
    PathCommand::CubicTo {
      control1: point(left, center_y - tangent_y),
      control2: point(center_x - tangent_x, top),
      end: point(center_x, top),
    },
    PathCommand::CubicTo {
      control1: point(center_x + tangent_x, top),
      control2: point(right, center_y - tangent_y),
      end: point(right, center_y),
    },
    PathCommand::CubicTo {
      control1: point(right, center_y + tangent_y),
      control2: point(center_x + tangent_x, bottom),
      end: point(center_x, bottom),
    },
    PathCommand::CubicTo {
      control1: point(center_x - tangent_x, bottom),
      control2: point(left, center_y + tangent_y),
      end: point(left, center_y),
    },
    PathCommand::Close,
  ]
}

fn wedge_round_rectangle_callout_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  pointer_x_adjustment: f32,
  pointer_y_adjustment: f32,
  radius_adjustment: f32,
) -> Vec<PathCommand> {
  if width <= 0.0 || height <= 0.0 {
    return rectangle_path(left, top, width, height);
  }
  let right = left + width;
  let bottom = top + height;
  let dx = width * pointer_x_adjustment / 100_000.0;
  let dy = height * pointer_y_adjustment / 100_000.0;
  let pointer_x = left + width / 2.0 + dx;
  let pointer_y = top + height / 2.0 + dy;
  let difference = dy.abs() - (dx * height / width).abs();
  let x1 = left + width * if dx > 0.0 { 7.0 } else { 2.0 } / 12.0;
  let x2 = left + width * if dx > 0.0 { 10.0 } else { 5.0 } / 12.0;
  let y1 = top + height * if dy > 0.0 { 7.0 } else { 2.0 } / 12.0;
  let y2 = top + height * if dy > 0.0 { 10.0 } else { 5.0 } / 12.0;
  let left_tip_x = if difference > 0.0 || dx > 0.0 {
    left
  } else {
    pointer_x
  };
  let top_tip_x = if difference > 0.0 {
    if dy > 0.0 { x1 } else { pointer_x }
  } else {
    x1
  };
  let right_tip_x = if difference > 0.0 {
    right
  } else if dx > 0.0 {
    pointer_x
  } else {
    right
  };
  let bottom_tip_x = if difference > 0.0 {
    if dy > 0.0 { pointer_x } else { x1 }
  } else {
    x1
  };
  let left_tip_y = if difference > 0.0 || dx > 0.0 {
    y1
  } else {
    pointer_y
  };
  let top_tip_y = if difference > 0.0 {
    if dy > 0.0 { top } else { pointer_y }
  } else {
    top
  };
  let right_tip_y = if difference > 0.0 {
    y1
  } else if dx > 0.0 {
    pointer_y
  } else {
    y1
  };
  let bottom_tip_y = if difference > 0.0 {
    if dy > 0.0 { pointer_y } else { bottom }
  } else {
    bottom
  };
  let radius = width.min(height) * radius_adjustment.clamp(0.0, 50_000.0) / 100_000.0;
  let kappa = radius * 0.552_284_8;
  vec![
    PathCommand::MoveTo(point(left, top + radius)),
    PathCommand::CubicTo {
      control1: point(left, top + radius - kappa),
      control2: point(left + radius - kappa, top),
      end: point(left + radius, top),
    },
    PathCommand::LineTo(point(x1, top)),
    PathCommand::LineTo(point(top_tip_x, top_tip_y)),
    PathCommand::LineTo(point(x2, top)),
    PathCommand::LineTo(point(right - radius, top)),
    PathCommand::CubicTo {
      control1: point(right - radius + kappa, top),
      control2: point(right, top + radius - kappa),
      end: point(right, top + radius),
    },
    PathCommand::LineTo(point(right, y1)),
    PathCommand::LineTo(point(right_tip_x, right_tip_y)),
    PathCommand::LineTo(point(right, y2)),
    PathCommand::LineTo(point(right, bottom - radius)),
    PathCommand::CubicTo {
      control1: point(right, bottom - radius + kappa),
      control2: point(right - radius + kappa, bottom),
      end: point(right - radius, bottom),
    },
    PathCommand::LineTo(point(x2, bottom)),
    PathCommand::LineTo(point(bottom_tip_x, bottom_tip_y)),
    PathCommand::LineTo(point(x1, bottom)),
    PathCommand::LineTo(point(left + radius, bottom)),
    PathCommand::CubicTo {
      control1: point(left + radius - kappa, bottom),
      control2: point(left, bottom - radius + kappa),
      end: point(left, bottom - radius),
    },
    PathCommand::LineTo(point(left, y2)),
    PathCommand::LineTo(point(left_tip_x, left_tip_y)),
    PathCommand::LineTo(point(left, y1)),
    PathCommand::Close,
  ]
}

fn triangle_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment: f32,
) -> Vec<PathCommand> {
  let apex_x = adjusted_axis(left, width, adjustment.clamp(0.0, 100_000.0));
  polygon([
    point(left, top + height),
    point(apex_x, top),
    point(left + width, top + height),
  ])
}

fn right_triangle_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  polygon([
    point(left, top + height),
    point(left, top),
    point(left + width, top + height),
  ])
}

fn diamond_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  polygon([
    point(left, top + height / 2.0),
    point(left + width / 2.0, top),
    point(left + width, top + height / 2.0),
    point(left + width / 2.0, top + height),
  ])
}

fn flow_chart_extract_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  polygon([
    point(left, top + height),
    point(left + width / 2.0, top),
    point(left + width, top + height),
  ])
}

fn hexagon_path(left: f32, top: f32, width: f32, height: f32, adjustment: f32) -> Vec<PathCommand> {
  let short_side = width.min(height);
  if short_side <= 0.0 {
    return rectangle_path(left, top, width, height);
  }
  let max_adjustment = 50_000.0 * width / short_side;
  let inset = short_side * adjustment.clamp(0.0, max_adjustment) / 100_000.0;
  let half_scaled_height = height / 2.0 * 115_470.0 / 100_000.0;
  let vertical_delta = half_scaled_height * (60.0_f32.to_radians().sin());
  let center_y = top + height / 2.0;
  let upper = center_y - vertical_delta;
  let lower = center_y + vertical_delta;
  polygon([
    point(left, center_y),
    point(left + inset, upper),
    point(left + width - inset, upper),
    point(left + width, center_y),
    point(left + width - inset, lower),
    point(left + inset, lower),
  ])
}

fn left_arrow_path(
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
  let thickness = thickness_adjustment.clamp(0.0, 100_000.0);
  let max_head = 100_000.0 * width / short_side;
  let head = head_adjustment.clamp(0.0, max_head);
  let right = left + width;
  let bottom = top + height;
  let center_y = top + height / 2.0;
  let head_right = left + short_side * head / 100_000.0;
  let half_shaft = height * thickness / 200_000.0;
  let shaft_top = center_y - half_shaft;
  let shaft_bottom = center_y + half_shaft;
  polygon([
    point(left, center_y),
    point(head_right, top),
    point(head_right, shaft_top),
    point(right, shaft_top),
    point(right, shaft_bottom),
    point(head_right, shaft_bottom),
    point(head_right, bottom),
  ])
}

fn up_arrow_path(
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
  let thickness = thickness_adjustment.clamp(0.0, 100_000.0);
  let max_head = 100_000.0 * height / short_side;
  let head = head_adjustment.clamp(0.0, max_head);
  let right = left + width;
  let bottom = top + height;
  let center_x = left + width / 2.0;
  let head_bottom = top + short_side * head / 100_000.0;
  let half_shaft = width * thickness / 200_000.0;
  let shaft_left = center_x - half_shaft;
  let shaft_right = center_x + half_shaft;
  polygon([
    point(left, head_bottom),
    point(center_x, top),
    point(right, head_bottom),
    point(shaft_right, head_bottom),
    point(shaft_right, bottom),
    point(shaft_left, bottom),
    point(shaft_left, head_bottom),
  ])
}

fn parallelogram_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment: f32,
) -> Vec<PathCommand> {
  let short_side = width.min(height);
  if short_side <= 0.0 {
    return rectangle_path(left, top, width, height);
  }
  let max_adjustment = 100_000.0 * width / short_side;
  let inset = short_side * adjustment.clamp(0.0, max_adjustment) / 100_000.0;
  polygon([
    point(left, top + height),
    point(left + inset, top),
    point(left + width, top),
    point(left + width - inset, top + height),
  ])
}

fn trapezoid_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment: f32,
) -> Vec<PathCommand> {
  let short_side = width.min(height);
  if short_side <= 0.0 {
    return rectangle_path(left, top, width, height);
  }
  let max_adjustment = 50_000.0 * width / short_side;
  let inset = short_side * adjustment.clamp(0.0, max_adjustment) / 100_000.0;
  polygon([
    point(left, top + height),
    point(left + inset, top),
    point(left + width - inset, top),
    point(left + width, top + height),
  ])
}

fn plus_path(left: f32, top: f32, width: f32, height: f32, adjustment: f32) -> Vec<PathCommand> {
  let short_side = width.min(height);
  let inset = short_side * adjustment.clamp(0.0, 50_000.0) / 100_000.0;
  let right = left + width;
  let bottom = top + height;
  let inner_right = right - inset;
  let inner_bottom = bottom - inset;
  polygon([
    point(left, top + inset),
    point(left + inset, top + inset),
    point(left + inset, top),
    point(inner_right, top),
    point(inner_right, top + inset),
    point(right, top + inset),
    point(right, inner_bottom),
    point(inner_right, inner_bottom),
    point(inner_right, bottom),
    point(left + inset, bottom),
    point(left + inset, inner_bottom),
    point(left, inner_bottom),
  ])
}

fn star_5_path(left: f32, top: f32, width: f32, height: f32, adjustment: f32) -> Vec<PathCommand> {
  let center_x = left + width / 2.0;
  // These two correction factors are part of the normative preset definition:
  // they keep a five-point star visually regular in a non-square shape frame.
  let outer_radius_x = width / 2.0 * 105_146.0 / 100_000.0;
  let outer_radius_y = height / 2.0 * 110_557.0 / 100_000.0;
  let center_y = top + height / 2.0 * 110_557.0 / 100_000.0;
  let inner_scale = adjustment.clamp(0.0, 50_000.0) / 50_000.0;
  let inner_radius_x = outer_radius_x * inner_scale;
  let inner_radius_y = outer_radius_y * inner_scale;
  let polar_point = |radius_x: f32, radius_y: f32, angle: f32| {
    let radians = angle.to_radians();
    point(
      center_x + radius_x * radians.cos(),
      center_y + radius_y * radians.sin(),
    )
  };
  polygon([
    polar_point(outer_radius_x, outer_radius_y, 198.0),
    polar_point(inner_radius_x, inner_radius_y, 234.0),
    point(center_x, top),
    polar_point(inner_radius_x, inner_radius_y, 306.0),
    polar_point(outer_radius_x, outer_radius_y, 342.0),
    polar_point(inner_radius_x, inner_radius_y, 18.0),
    polar_point(outer_radius_x, outer_radius_y, 54.0),
    polar_point(inner_radius_x, inner_radius_y, 90.0),
    polar_point(outer_radius_x, outer_radius_y, 126.0),
    polar_point(inner_radius_x, inner_radius_y, 162.0),
  ])
}

fn donut_path(left: f32, top: f32, width: f32, height: f32, adjustment: f32) -> Vec<PathCommand> {
  let inset = width.min(height) * adjustment.clamp(0.0, 50_000.0) / 100_000.0;
  let mut commands = ellipse_path(left, top, width, height);
  commands.extend(reverse_ellipse_path(
    left + inset,
    top + inset,
    width - 2.0 * inset,
    height - 2.0 * inset,
  ));
  commands
}

fn reverse_ellipse_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  let right = left + width;
  let bottom = top + height;
  let center_x = left + width / 2.0;
  let center_y = top + height / 2.0;
  let tangent_x = width / 2.0 * CIRCLE_CUBIC_BEZIER_KAPPA;
  let tangent_y = height / 2.0 * CIRCLE_CUBIC_BEZIER_KAPPA;
  vec![
    PathCommand::MoveTo(point(left, center_y)),
    PathCommand::CubicTo {
      control1: point(left, center_y + tangent_y),
      control2: point(center_x - tangent_x, bottom),
      end: point(center_x, bottom),
    },
    PathCommand::CubicTo {
      control1: point(center_x + tangent_x, bottom),
      control2: point(right, center_y + tangent_y),
      end: point(right, center_y),
    },
    PathCommand::CubicTo {
      control1: point(right, center_y - tangent_y),
      control2: point(center_x + tangent_x, top),
      end: point(center_x, top),
    },
    PathCommand::CubicTo {
      control1: point(center_x - tangent_x, top),
      control2: point(left, center_y - tangent_y),
      end: point(left, center_y),
    },
    PathCommand::Close,
  ]
}

fn flow_chart_delay_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  let center_x = left + width / 2.0;
  let center_y = top + height / 2.0;
  let right = left + width;
  let bottom = top + height;
  let tangent_x = width / 2.0 * CIRCLE_CUBIC_BEZIER_KAPPA;
  let tangent_y = height / 2.0 * CIRCLE_CUBIC_BEZIER_KAPPA;
  vec![
    PathCommand::MoveTo(point(left, top)),
    PathCommand::LineTo(point(center_x, top)),
    PathCommand::CubicTo {
      control1: point(center_x + tangent_x, top),
      control2: point(right, center_y - tangent_y),
      end: point(right, center_y),
    },
    PathCommand::CubicTo {
      control1: point(right, center_y + tangent_y),
      control2: point(center_x + tangent_x, bottom),
      end: point(center_x, bottom),
    },
    PathCommand::LineTo(point(left, bottom)),
    PathCommand::Close,
  ]
}

fn frame_path(left: f32, top: f32, width: f32, height: f32, adjustment: f32) -> Vec<PathCommand> {
  let inset = width.min(height) * adjustment.clamp(0.0, 50_000.0) / 100_000.0;
  let right = left + width;
  let bottom = top + height;
  let mut commands = rectangle_path(left, top, width, height);
  commands.extend(polygon([
    point(left + inset, top + inset),
    point(left + inset, bottom - inset),
    point(right - inset, bottom - inset),
    point(right - inset, top + inset),
  ]));
  commands
}

fn bent_connector_2_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  let right = left + width;
  open_path([
    point(left, top),
    point(right, top),
    point(right, top + height),
  ])
}

fn bent_connector_3_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment_1: f32,
) -> Vec<PathCommand> {
  let x1 = adjusted_axis(left, width, adjustment_1);
  open_path([
    point(left, top),
    point(x1, top),
    point(x1, top + height),
    point(left + width, top + height),
  ])
}

fn bent_connector_4_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment_1: f32,
  adjustment_2: f32,
) -> Vec<PathCommand> {
  let x1 = adjusted_axis(left, width, adjustment_1);
  let y2 = adjusted_axis(top, height, adjustment_2);
  open_path([
    point(left, top),
    point(x1, top),
    point(x1, y2),
    point(left + width, y2),
    point(left + width, top + height),
  ])
}

fn bent_connector_5_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment_1: f32,
  adjustment_2: f32,
  adjustment_3: f32,
) -> Vec<PathCommand> {
  let x1 = adjusted_axis(left, width, adjustment_1);
  let x3 = adjusted_axis(left, width, adjustment_3);
  let y2 = adjusted_axis(top, height, adjustment_2);
  open_path([
    point(left, top),
    point(x1, top),
    point(x1, y2),
    point(x3, y2),
    point(x3, top + height),
    point(left + width, top + height),
  ])
}

fn curved_connector_2_path(left: f32, top: f32, width: f32, height: f32) -> Vec<PathCommand> {
  vec![
    PathCommand::MoveTo(point(left, top)),
    PathCommand::CubicTo {
      control1: point(left + width / 2.0, top),
      control2: point(left + width, top + height / 2.0),
      end: point(left + width, top + height),
    },
  ]
}

fn curved_connector_3_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment_1: f32,
) -> Vec<PathCommand> {
  let right = left + width;
  let bottom = top + height;
  let x2 = adjusted_axis(left, width, adjustment_1);
  let x1 = (left + x2) / 2.0;
  let x3 = (right + x2) / 2.0;
  vec![
    PathCommand::MoveTo(point(left, top)),
    PathCommand::CubicTo {
      control1: point(x1, top),
      control2: point(x2, top + height / 4.0),
      end: point(x2, top + height / 2.0),
    },
    PathCommand::CubicTo {
      control1: point(x2, top + height * 3.0 / 4.0),
      control2: point(x3, bottom),
      end: point(right, bottom),
    },
  ]
}

fn curved_connector_4_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment_1: f32,
  adjustment_2: f32,
) -> Vec<PathCommand> {
  let right = left + width;
  let bottom = top + height;
  let x2 = adjusted_axis(left, width, adjustment_1);
  let x1 = (left + x2) / 2.0;
  let x3 = (right + x2) / 2.0;
  let x4 = (x2 + x3) / 2.0;
  let x5 = (x3 + right) / 2.0;
  let y4 = adjusted_axis(top, height, adjustment_2);
  let y1 = (top + y4) / 2.0;
  let y2 = (top + y1) / 2.0;
  let y3 = (y1 + y4) / 2.0;
  let y5 = (bottom + y4) / 2.0;
  vec![
    PathCommand::MoveTo(point(left, top)),
    PathCommand::CubicTo {
      control1: point(x1, top),
      control2: point(x2, y2),
      end: point(x2, y1),
    },
    PathCommand::CubicTo {
      control1: point(x2, y3),
      control2: point(x4, y4),
      end: point(x3, y4),
    },
    PathCommand::CubicTo {
      control1: point(x5, y4),
      control2: point(right, y5),
      end: point(right, bottom),
    },
  ]
}

fn curved_connector_5_path(
  left: f32,
  top: f32,
  width: f32,
  height: f32,
  adjustment_1: f32,
  adjustment_2: f32,
  adjustment_3: f32,
) -> Vec<PathCommand> {
  let right = left + width;
  let bottom = top + height;
  let x3 = adjusted_axis(left, width, adjustment_1);
  let x6 = adjusted_axis(left, width, adjustment_3);
  let x1 = (x3 + x6) / 2.0;
  let x2 = (left + x3) / 2.0;
  let x4 = (x3 + x1) / 2.0;
  let x5 = (x6 + x1) / 2.0;
  let x7 = (x6 + right) / 2.0;
  let y4 = adjusted_axis(top, height, adjustment_2);
  let y1 = (top + y4) / 2.0;
  let y2 = (top + y1) / 2.0;
  let y3 = (y1 + y4) / 2.0;
  let y5 = (bottom + y4) / 2.0;
  let y6 = (y5 + y4) / 2.0;
  let y7 = (y5 + bottom) / 2.0;
  vec![
    PathCommand::MoveTo(point(left, top)),
    PathCommand::CubicTo {
      control1: point(x2, top),
      control2: point(x3, y2),
      end: point(x3, y1),
    },
    PathCommand::CubicTo {
      control1: point(x3, y3),
      control2: point(x4, y4),
      end: point(x1, y4),
    },
    PathCommand::CubicTo {
      control1: point(x5, y4),
      control2: point(x6, y6),
      end: point(x6, y5),
    },
    PathCommand::CubicTo {
      control1: point(x6, y7),
      control2: point(x7, bottom),
      end: point(right, bottom),
    },
  ]
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
  let tangent = radius * CIRCLE_CUBIC_BEZIER_KAPPA;
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
  use super::{
    bent_connector_5_path, curved_connector_4_path, donut_path, down_arrow_path, ellipse_path,
    frame_path, right_arrow_path, right_triangle_path, star_5_path, straight_connector_path,
    triangle_path, wedge_round_rectangle_callout_path,
  };
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
  fn connector_presets_are_open_paths() {
    for commands in [
      straight_connector_path(10.0, 20.0, 100.0, 80.0),
      bent_connector_5_path(10.0, 20.0, 100.0, 80.0, 25_000.0, 75_000.0, 80_000.0),
      curved_connector_4_path(10.0, 20.0, 100.0, 80.0, 25_000.0, 75_000.0),
    ] {
      assert!(
        !commands
          .iter()
          .any(|command| matches!(command, PathCommand::Close))
      );
    }
  }

  #[test]
  fn compound_presets_keep_opposite_inner_subpaths_for_holes() {
    for commands in [
      donut_path(10.0, 20.0, 100.0, 80.0, 25_000.0),
      frame_path(10.0, 20.0, 100.0, 80.0, 12_500.0),
    ] {
      assert_eq!(
        commands
          .iter()
          .filter(|command| matches!(command, PathCommand::MoveTo(_)))
          .count(),
        2
      );
      assert_eq!(
        commands
          .iter()
          .filter(|command| matches!(command, PathCommand::Close))
          .count(),
        2
      );
    }
  }

  #[test]
  fn star_5_uses_the_declared_adjustment_and_top_vertex() {
    let commands = star_5_path(10.0, 20.0, 100.0, 80.0, 19_098.0);
    assert_eq!(commands.len(), 11);
    assert_eq!(
      commands[2],
      PathCommand::LineTo(Point {
        x: Pt(60.0),
        y: Pt(20.0),
      })
    );
    assert_eq!(commands.last(), Some(&PathCommand::Close));
  }

  #[test]
  fn bent_connector_5_matches_the_ecma_guide_equations() {
    assert_eq!(
      line_points(&bent_connector_5_path(
        10.0, 20.0, 100.0, 80.0, 25_000.0, 75_000.0, 80_000.0
      )),
      [
        Point {
          x: Pt(10.0),
          y: Pt(20.0),
        },
        Point {
          x: Pt(35.0),
          y: Pt(20.0),
        },
        Point {
          x: Pt(35.0),
          y: Pt(80.0),
        },
        Point {
          x: Pt(90.0),
          y: Pt(80.0),
        },
        Point {
          x: Pt(90.0),
          y: Pt(100.0),
        },
        Point {
          x: Pt(110.0),
          y: Pt(100.0),
        },
      ]
    );
  }

  #[test]
  fn triangle_adjustment_and_right_triangle_match_the_preset_points() {
    assert_eq!(
      line_points(&triangle_path(10.0, 20.0, 100.0, 80.0, 25_000.0)),
      [
        Point {
          x: Pt(10.0),
          y: Pt(100.0),
        },
        Point {
          x: Pt(35.0),
          y: Pt(20.0),
        },
        Point {
          x: Pt(110.0),
          y: Pt(100.0),
        },
      ]
    );
    assert_eq!(
      line_points(&right_triangle_path(10.0, 20.0, 100.0, 80.0)),
      [
        Point {
          x: Pt(10.0),
          y: Pt(100.0),
        },
        Point {
          x: Pt(10.0),
          y: Pt(20.0),
        },
        Point {
          x: Pt(110.0),
          y: Pt(100.0),
        },
      ]
    );
  }

  #[test]
  fn ellipse_uses_four_closed_cubic_quarters() {
    let commands = ellipse_path(10.0, 20.0, 100.0, 80.0);
    assert_eq!(commands.len(), 6);
    assert!(matches!(commands[0], PathCommand::MoveTo(_)));
    assert_eq!(
      commands
        .iter()
        .filter(|command| matches!(command, PathCommand::CubicTo { .. }))
        .count(),
      4
    );
    assert!(matches!(commands.last(), Some(PathCommand::Close)));
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

  #[test]
  fn default_wedge_round_rectangle_callout_places_the_pointer_below_the_shape() {
    let commands =
      wedge_round_rectangle_callout_path(0.0, 0.0, 100.0, 80.0, -20_833.0, 62_500.0, 16_667.0);
    let PathCommand::LineTo(pointer) = commands[13] else {
      panic!("expected the bottom wedge pointer");
    };
    assert!((pointer.x.0 - 29.167).abs() < 0.001);
    assert_eq!(pointer.y, Pt(90.0));
    assert_eq!(
      commands
        .iter()
        .filter(|command| matches!(command, PathCommand::CubicTo { .. }))
        .count(),
      4
    );
    assert_eq!(commands.last(), Some(&PathCommand::Close));
  }
}
