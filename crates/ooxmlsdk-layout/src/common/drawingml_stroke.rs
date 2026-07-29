use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use crate::common::{
  PathCommand, PathItem, Point, Pt, Rect, Size, Stroke, StrokeAlignment, StrokeCap, StrokeCompound,
  StrokeDashPreset, StrokeEnd, StrokeEndKind, StrokeEndSize, StrokeJoin,
};

const MIN_MARKER_BASE_PT: f32 = 70.0 * 72.0 / 2_540.0;

pub(crate) fn apply_outline_style(stroke: &mut Stroke<'_>, outline: &a::Outline) {
  stroke.cap = outline.cap_type.map(line_cap);
  stroke.compound = outline.compound_line_type.map(compound);
  stroke.alignment = outline.alignment.map(alignment);
  match outline.outline_choice2.as_ref() {
    Some(a::OutlineChoice2::PresetDash(dash)) => {
      stroke.preset_dash = dash.val.map(preset_dash);
      stroke.dash = None;
    }
    Some(a::OutlineChoice2::CustomDash(dash)) => {
      stroke.preset_dash = None;
      stroke.dash = Some(
        dash
          .dash_stop
          .iter()
          .flat_map(|stop| {
            [
              Pt(stroke.width.0 * stop.dash_length.as_ratio() as f32),
              Pt(stroke.width.0 * stop.space_length.as_ratio() as f32),
            ]
          })
          .collect(),
      );
    }
    None => {}
  }
  stroke.join = match outline.outline_choice3.as_ref() {
    Some(a::OutlineChoice3::Round) => Some(StrokeJoin::Round),
    Some(a::OutlineChoice3::LineJoinBevel) => Some(StrokeJoin::Bevel),
    Some(a::OutlineChoice3::Miter(miter)) => Some(StrokeJoin::Miter {
      limit: miter.limit.map(|limit| limit.as_ratio() as f32),
    }),
    None => None,
  };
  stroke.head_end = outline.head_end.as_ref().map(|end| StrokeEnd {
    kind: end.r#type.map(line_end).unwrap_or(StrokeEndKind::None),
    width: end.width.map(end_width).unwrap_or(StrokeEndSize::Medium),
    length: end.length.map(end_length).unwrap_or(StrokeEndSize::Medium),
  });
  stroke.tail_end = outline.tail_end.as_ref().map(|end| StrokeEnd {
    kind: end.r#type.map(line_end).unwrap_or(StrokeEndKind::None),
    width: end.width.map(end_width).unwrap_or(StrokeEndSize::Medium),
    length: end.length.map(end_length).unwrap_or(StrokeEndSize::Medium),
  });
}

fn line_cap(value: a::LineCapValues) -> StrokeCap {
  match value {
    a::LineCapValues::Round => StrokeCap::Round,
    a::LineCapValues::Square => StrokeCap::Square,
    a::LineCapValues::Flat => StrokeCap::Flat,
  }
}

fn compound(value: a::CompoundLineValues) -> StrokeCompound {
  match value {
    a::CompoundLineValues::Single => StrokeCompound::Single,
    a::CompoundLineValues::Double => StrokeCompound::Double,
    a::CompoundLineValues::ThickThin => StrokeCompound::ThickThin,
    a::CompoundLineValues::ThinThick => StrokeCompound::ThinThick,
    a::CompoundLineValues::Triple => StrokeCompound::Triple,
  }
}

fn alignment(value: a::PenAlignmentValues) -> StrokeAlignment {
  match value {
    a::PenAlignmentValues::Center => StrokeAlignment::Center,
    a::PenAlignmentValues::Insert => StrokeAlignment::Inside,
  }
}

fn preset_dash(value: a::PresetLineDashValues) -> StrokeDashPreset {
  match value {
    a::PresetLineDashValues::Solid => StrokeDashPreset::Solid,
    a::PresetLineDashValues::Dot => StrokeDashPreset::Dot,
    a::PresetLineDashValues::Dash => StrokeDashPreset::Dash,
    a::PresetLineDashValues::LargeDash => StrokeDashPreset::LargeDash,
    a::PresetLineDashValues::DashDot => StrokeDashPreset::DashDot,
    a::PresetLineDashValues::LargeDashDot => StrokeDashPreset::LargeDashDot,
    a::PresetLineDashValues::LargeDashDotDot => StrokeDashPreset::LargeDashDotDot,
    a::PresetLineDashValues::SystemDash => StrokeDashPreset::SystemDash,
    a::PresetLineDashValues::SystemDot => StrokeDashPreset::SystemDot,
    a::PresetLineDashValues::SystemDashDot => StrokeDashPreset::SystemDashDot,
    a::PresetLineDashValues::SystemDashDotDot => StrokeDashPreset::SystemDashDotDot,
  }
}

fn line_end(value: a::LineEndValues) -> StrokeEndKind {
  match value {
    a::LineEndValues::None => StrokeEndKind::None,
    a::LineEndValues::Triangle => StrokeEndKind::Triangle,
    a::LineEndValues::Stealth => StrokeEndKind::Stealth,
    a::LineEndValues::Diamond => StrokeEndKind::Diamond,
    a::LineEndValues::Oval => StrokeEndKind::Oval,
    a::LineEndValues::Arrow => StrokeEndKind::Arrow,
  }
}

fn end_width(value: a::LineEndWidthValues) -> StrokeEndSize {
  match value {
    a::LineEndWidthValues::Small => StrokeEndSize::Small,
    a::LineEndWidthValues::Medium => StrokeEndSize::Medium,
    a::LineEndWidthValues::Large => StrokeEndSize::Large,
  }
}

fn end_length(value: a::LineEndLengthValues) -> StrokeEndSize {
  match value {
    a::LineEndLengthValues::Small => StrokeEndSize::Small,
    a::LineEndLengthValues::Medium => StrokeEndSize::Medium,
    a::LineEndLengthValues::Large => StrokeEndSize::Large,
  }
}

pub(crate) fn stroke_end_marker_polygons(
  path: &PathItem<'_>,
  stroke: &Stroke<'_>,
) -> Vec<Vec<Point>> {
  let Some(endpoints) = path_endpoints(path) else {
    return Vec::new();
  };
  [
    stroke
      .head_end
      .and_then(|marker| marker_polygon(marker, endpoints.start, endpoints.start_outward, stroke)),
    stroke
      .tail_end
      .and_then(|marker| marker_polygon(marker, endpoints.end, endpoints.end_outward, stroke)),
  ]
  .into_iter()
  .flatten()
  .collect()
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct StrokedOpenArrowMarker {
  pub(crate) points: [Point; 3],
  pub(crate) width: Pt,
  pub(crate) bounds: Rect,
}

pub(crate) fn stroked_open_arrow_markers(
  path: &PathItem<'_>,
  stroke: &Stroke<'_>,
) -> Vec<StrokedOpenArrowMarker> {
  let Some(endpoints) = path_endpoints(path) else {
    return Vec::new();
  };
  [
    stroke.head_end.and_then(|marker| {
      stroked_open_arrow(marker, endpoints.start, endpoints.start_outward, stroke)
    }),
    stroke
      .tail_end
      .and_then(|marker| stroked_open_arrow(marker, endpoints.end, endpoints.end_outward, stroke)),
  ]
  .into_iter()
  .flatten()
  .collect()
}

pub(crate) fn stroke_end_shaft_insets(stroke: &Stroke<'_>) -> (f32, f32) {
  (
    stroke
      .head_end
      .filter(|marker| uses_stroked_open_arrow(*marker, stroke.width.0))
      .map(|_| stroke.width.0)
      .unwrap_or_default(),
    stroke
      .tail_end
      .filter(|marker| uses_stroked_open_arrow(*marker, stroke.width.0))
      .map(|_| stroke.width.0)
      .unwrap_or_default(),
  )
}

pub(crate) fn stroke_end_marker_bounds(path: &PathItem<'_>, stroke: &Stroke<'_>) -> Option<Rect> {
  let mut bounds = None;
  for polygon in stroke_end_marker_polygons(path, stroke) {
    bounds = union_optional_rect(bounds, points_bounds(&polygon));
  }
  for marker in stroked_open_arrow_markers(path, stroke) {
    bounds = union_optional_rect(bounds, Some(marker.bounds));
  }
  bounds
}

fn points_bounds(points: &[Point]) -> Option<Rect> {
  let first = points.first()?;
  let (mut left, mut top, mut right, mut bottom) = (first.x.0, first.y.0, first.x.0, first.y.0);
  for point in &points[1..] {
    left = left.min(point.x.0);
    top = top.min(point.y.0);
    right = right.max(point.x.0);
    bottom = bottom.max(point.y.0);
  }
  Some(Rect {
    origin: Point {
      x: Pt(left),
      y: Pt(top),
    },
    size: Size {
      width: Pt(right - left),
      height: Pt(bottom - top),
    },
  })
}

fn union_optional_rect(first: Option<Rect>, second: Option<Rect>) -> Option<Rect> {
  match (first, second) {
    (None, other) | (other, None) => other,
    (Some(first), Some(second)) => {
      let left = first.origin.x.0.min(second.origin.x.0);
      let top = first.origin.y.0.min(second.origin.y.0);
      let right =
        (first.origin.x.0 + first.size.width.0).max(second.origin.x.0 + second.size.width.0);
      let bottom =
        (first.origin.y.0 + first.size.height.0).max(second.origin.y.0 + second.size.height.0);
      Some(Rect {
        origin: Point {
          x: Pt(left),
          y: Pt(top),
        },
        size: Size {
          width: Pt(right - left),
          height: Pt(bottom - top),
        },
      })
    }
  }
}

struct PathEndpoints {
  start: (f32, f32),
  start_outward: (f32, f32),
  end: (f32, f32),
  end_outward: (f32, f32),
}

fn path_endpoints(path: &PathItem<'_>) -> Option<PathEndpoints> {
  if path.closed {
    return None;
  }
  if path.commands.is_empty() {
    let [first, second, ..] = path.points.as_slice() else {
      return None;
    };
    let penultimate = path.points[path.points.len() - 2];
    let last = path.points[path.points.len() - 1];
    return Some(PathEndpoints {
      start: (first.x.0, first.y.0),
      start_outward: normalized_direction(second.x.0, second.y.0, first.x.0, first.y.0)?,
      end: (last.x.0, last.y.0),
      end_outward: normalized_direction(penultimate.x.0, penultimate.y.0, last.x.0, last.y.0)?,
    });
  }
  let mut first = None;
  let mut first_tangent = None;
  let mut current = None;
  let mut last_tangent = None;
  for command in &path.commands {
    match *command {
      PathCommand::MoveTo(point) => {
        current = Some((point.x.0, point.y.0));
        first.get_or_insert((point.x.0, point.y.0));
      }
      PathCommand::LineTo(point) => {
        let start = current?;
        let end = (point.x.0, point.y.0);
        first_tangent.get_or_insert((start, end));
        last_tangent = Some((start, end));
        current = Some(end);
      }
      PathCommand::CubicTo {
        control1,
        control2,
        end,
      } => {
        let start = current?;
        let control1 = (control1.x.0, control1.y.0);
        let control2 = (control2.x.0, control2.y.0);
        let end = (end.x.0, end.y.0);
        first_tangent.get_or_insert((start, if control1 != start { control1 } else { end }));
        last_tangent = Some((if control2 != end { control2 } else { start }, end));
        current = Some(end);
      }
      PathCommand::Close => return None,
    }
  }
  let first = first?;
  let (first_from, first_to) = first_tangent?;
  let (last_from, last) = last_tangent?;
  Some(PathEndpoints {
    start: first,
    start_outward: normalized_direction(first_to.0, first_to.1, first_from.0, first_from.1)?,
    end: last,
    end_outward: normalized_direction(last_from.0, last_from.1, last.0, last.1)?,
  })
}

fn normalized_direction(from_x: f32, from_y: f32, to_x: f32, to_y: f32) -> Option<(f32, f32)> {
  let dx = to_x - from_x;
  let dy = to_y - from_y;
  let length = dx.hypot(dy);
  (length > f32::EPSILON).then_some((dx / length, dy / length))
}

fn marker_dimensions(marker: StrokeEnd, line_width: f32) -> (f32, f32) {
  let is_open_arrow = marker.kind == StrokeEndKind::Arrow;
  let baseline = line_width.max(MIN_MARKER_BASE_PT);
  if marker.kind == StrokeEndKind::Arrow
    && marker.width == StrokeEndSize::Medium
    && marker.length == StrokeEndSize::Medium
    && !uses_stroked_open_arrow(marker, line_width)
  {
    return (
      3.5 * baseline + (3.0_f32.sqrt() / 2.0) * line_width,
      3.0 * baseline + 0.75 * line_width,
    );
  }
  (
    marker_size_factor(marker.width, is_open_arrow) * baseline,
    marker_size_factor(marker.length, is_open_arrow) * baseline,
  )
}

fn marker_size_factor(size: StrokeEndSize, is_open_arrow: bool) -> f32 {
  match (size, is_open_arrow) {
    (StrokeEndSize::Small, false) => 2.0,
    (StrokeEndSize::Medium, false) => 3.0,
    (StrokeEndSize::Large, false) => 5.0,
    (StrokeEndSize::Small, true) => 2.5,
    (StrokeEndSize::Medium, true) => 3.5,
    (StrokeEndSize::Large, true) => 5.5,
  }
}

fn uses_stroked_open_arrow(marker: StrokeEnd, line_width: f32) -> bool {
  // Once the authored line width drives the marker scale, Word fixed output
  // paints an open arrow as a round-capped V and ends the shaft one line width
  // before its tip. Keep the fixed-minimum LibreOffice polygon below this
  // boundary; thin and hairline Office goldens exercise that distinct shape.
  marker.kind == StrokeEndKind::Arrow && line_width >= MIN_MARKER_BASE_PT
}

fn stroked_open_arrow(
  marker: StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  stroke: &Stroke<'_>,
) -> Option<StrokedOpenArrowMarker> {
  if !uses_stroked_open_arrow(marker, stroke.width.0) {
    return None;
  }
  let baseline = stroke.width.0.max(MIN_MARKER_BASE_PT);
  let marker_width = marker_size_factor(marker.width, true) * baseline;
  let marker_length = marker_size_factor(marker.length, true) * baseline;
  let radius = stroke.width.0 / 2.0;
  let half_width = marker_width / 2.0;
  let base_offset = marker_length + radius;
  let coefficient = half_width * half_width - radius * radius;
  let linear = 2.0 * radius * radius * base_offset;
  let constant = -radius * radius * (half_width * half_width + base_offset * base_offset);
  let discriminant = linear * linear - 4.0 * coefficient * constant;
  let miter_inset = if coefficient > f32::EPSILON && discriminant >= 0.0 {
    (-linear + discriminant.sqrt()) / (2.0 * coefficient)
  } else {
    radius
  };
  let perpendicular = (-outward.1, outward.0);
  let point = |back: f32, across: f32| Point {
    x: Pt(endpoint.0 - outward.0 * back + perpendicular.0 * across),
    y: Pt(endpoint.1 - outward.1 * back + perpendicular.1 * across),
  };
  let left = point(base_offset, -half_width);
  let apex = point(miter_inset, 0.0);
  let right = point(base_offset, half_width);
  let cap_bounds = |point: Point| Rect {
    origin: Point {
      x: Pt(point.x.0 - radius),
      y: Pt(point.y.0 - radius),
    },
    size: Size {
      width: Pt(stroke.width.0),
      height: Pt(stroke.width.0),
    },
  };
  let tip_bounds = Rect {
    origin: Point {
      x: Pt(endpoint.0),
      y: Pt(endpoint.1),
    },
    size: Size::default(),
  };
  let bounds = union_optional_rect(
    union_optional_rect(Some(cap_bounds(left)), Some(cap_bounds(right))),
    Some(tip_bounds),
  )?;
  Some(StrokedOpenArrowMarker {
    points: [left, apex, right],
    width: stroke.width,
    bounds,
  })
}

fn marker_polygon(
  marker: StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  stroke: &Stroke<'_>,
) -> Option<Vec<Point>> {
  if marker.kind == StrokeEndKind::None || uses_stroked_open_arrow(marker, stroke.width.0) {
    return None;
  }
  let (width, length) = marker_dimensions(marker, stroke.width.0);
  let centered = matches!(marker.kind, StrokeEndKind::Diamond | StrokeEndKind::Oval);
  let line_half_width = (50.0 * stroke.width.0 / width).max(1.0);
  let points: &[(f32, f32)] = match marker.kind {
    StrokeEndKind::Triangle => &[(50.0, 0.0), (100.0, 100.0), (0.0, 100.0)],
    StrokeEndKind::Stealth => &[(50.0, 0.0), (100.0, 100.0), (50.0, 60.0), (0.0, 100.0)],
    StrokeEndKind::Diamond => &[(50.0, 0.0), (100.0, 50.0), (50.0, 100.0), (0.0, 50.0)],
    StrokeEndKind::Oval => &[
      (50.0, 0.0),
      (75.0, 7.0),
      (93.0, 25.0),
      (100.0, 50.0),
      (93.0, 75.0),
      (75.0, 93.0),
      (50.0, 100.0),
      (25.0, 93.0),
      (7.0, 75.0),
      (0.0, 50.0),
      (7.0, 25.0),
      (25.0, 7.0),
    ],
    StrokeEndKind::Arrow => &[
      (50.0, 0.0),
      (100.0, 100.0 - line_half_width * 1.5),
      (100.0 - line_half_width * 1.5, 100.0),
      (50.0 + line_half_width, 5.5 * line_half_width),
      (50.0 + line_half_width, 100.0),
      (50.0 - line_half_width, 100.0),
      (50.0 - line_half_width, 5.5 * line_half_width),
      (line_half_width * 1.5, 100.0),
      (0.0, 100.0 - line_half_width * 1.5),
    ],
    StrokeEndKind::None => return None,
  };
  let perpendicular = (-outward.1, outward.0);
  Some(
    points
      .iter()
      .map(|&(x, y)| {
        let across = (x / 100.0 - 0.5) * width;
        let back = (y / 100.0 - if centered { 0.5 } else { 0.0 }) * length;
        Point {
          x: Pt(endpoint.0 - outward.0 * back + perpendicular.0 * across),
          y: Pt(endpoint.1 - outward.1 * back + perpendicular.1 * across),
        }
      })
      .collect(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::{Color, Fill};

  #[test]
  fn medium_open_arrow_bounds_include_the_stroked_arm_envelope() {
    let path = PathItem {
      bounds: Rect {
        origin: Point::default(),
        size: Size {
          width: Pt(0.0),
          height: Pt(100.0),
        },
      },
      points: vec![
        Point::default(),
        Point {
          x: Pt(0.0),
          y: Pt(100.0),
        },
      ],
      commands: Vec::new(),
      closed: false,
      fill: Fill::None,
      stroke: None,
    };
    let stroke = Stroke {
      width: Pt(2.0),
      color: Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
      },
      tail_end: Some(StrokeEnd {
        kind: StrokeEndKind::Arrow,
        width: StrokeEndSize::Medium,
        length: StrokeEndSize::Medium,
      }),
      ..Stroke::default()
    };

    let bounds = stroke_end_marker_bounds(&path, &stroke).expect("tail marker bounds");

    assert_eq!(bounds.origin.x.0, -4.5);
    assert_eq!(bounds.size.width.0, 9.0);
    assert_eq!(bounds.origin.y.0, 91.0);
    assert_eq!(bounds.size.height.0, 9.0);

    let markers = stroked_open_arrow_markers(&path, &stroke);
    assert_eq!(markers.len(), 1);
    assert!((markers[0].points[1].y.0 - 98.012_35).abs() < 0.000_1);
  }
}
