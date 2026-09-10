use super::paint::PolylineItem;
use ooxmlsdk_layout::common;

const MIN_MARKER_BASE_PT: f32 = 70.0 * 72.0 / 2_540.0;

#[derive(Clone, Debug, PartialEq)]
pub(super) enum StrokeMarkerGeometry {
  Filled {
    points: Vec<(f32, f32)>,
  },
  StrokedOpen {
    points: [(f32, f32); 3],
    width_pt: f32,
  },
}

pub(super) fn stroke_marker_geometries(
  polyline: &PolylineItem<'_>,
  stroke: &common::Stroke<'static>,
) -> [Option<StrokeMarkerGeometry>; 2] {
  let Some(endpoints) = path_endpoints(polyline) else {
    return [None, None];
  };
  [
    stroke
      .head_end
      .and_then(|marker| marker_geometry(marker, endpoints.start, endpoints.start_outward, stroke)),
    stroke
      .tail_end
      .and_then(|marker| marker_geometry(marker, endpoints.end, endpoints.end_outward, stroke)),
  ]
}

pub(super) fn shortened_straight_polyline_points(
  polyline: &PolylineItem<'_>,
) -> Option<((f32, f32), (f32, f32))> {
  if polyline.closed {
    return None;
  }
  let (start, end) = if polyline.commands.is_empty() {
    let [start, end] = polyline.points else {
      return None;
    };
    ((start.x.0, start.y.0), (end.x.0, end.y.0))
  } else {
    let [
      common::PathCommand::MoveTo(start),
      common::PathCommand::LineTo(end),
    ] = polyline.commands
    else {
      return None;
    };
    ((start.x.0, start.y.0), (end.x.0, end.y.0))
  };
  let stroke = polyline.stroke?;
  let head_inset = stroke
    .head_end
    .filter(|marker| uses_stroked_open_arrow(*marker, stroke.width.0))
    .map(|_| stroke.width.0)
    .unwrap_or_default();
  let tail_inset = stroke
    .tail_end
    .filter(|marker| uses_stroked_open_arrow(*marker, stroke.width.0))
    .map(|_| stroke.width.0)
    .unwrap_or_default();
  if head_inset <= 0.0 && tail_inset <= 0.0 {
    return None;
  }
  let dx = end.0 - start.0;
  let dy = end.1 - start.1;
  let length = dx.hypot(dy);
  if length <= head_inset + tail_inset || length <= f32::EPSILON {
    return None;
  }
  let direction = (dx / length, dy / length);
  Some((
    (
      start.0 + direction.0 * head_inset,
      start.1 + direction.1 * head_inset,
    ),
    (
      end.0 - direction.0 * tail_inset,
      end.1 - direction.1 * tail_inset,
    ),
  ))
}

fn marker_geometry(
  marker: common::StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  stroke: &common::Stroke<'static>,
) -> Option<StrokeMarkerGeometry> {
  if uses_stroked_open_arrow(marker, stroke.width.0) {
    return stroked_open_arrow_geometry(marker, endpoint, outward, stroke.width.0);
  }
  filled_marker_geometry(marker, endpoint, outward, stroke.width.0)
}

struct PathEndpoints {
  start: (f32, f32),
  start_outward: (f32, f32),
  end: (f32, f32),
  end_outward: (f32, f32),
}

fn path_endpoints(polyline: &PolylineItem<'_>) -> Option<PathEndpoints> {
  if polyline.closed {
    return None;
  }
  if polyline.commands.is_empty() {
    let [first, ..] = polyline.points else {
      return None;
    };
    let last = polyline.points[polyline.points.len() - 1];
    let second = polyline.points.iter().find(|point| *point != first)?;
    let penultimate = polyline.points.iter().rev().find(|point| **point != last)?;
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
  let mut closed = false;
  for command in polyline.commands {
    match *command {
      common::PathCommand::MoveTo(point) => {
        current = Some((point.x.0, point.y.0));
        first.get_or_insert((point.x.0, point.y.0));
      }
      common::PathCommand::LineTo(point) => {
        let start = current?;
        let end = (point.x.0, point.y.0);
        // A collapsed connector segment does not replace the adjacent
        // nonzero tangent used by the endpoint decoration.
        if start != end {
          first_tangent.get_or_insert((start, end));
          last_tangent = Some((start, end));
        }
        current = Some(end);
      }
      common::PathCommand::CubicTo {
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
      common::PathCommand::Close => closed = true,
    }
  }
  if closed {
    return None;
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

fn stroke_end_size_factor(size: common::StrokeEndSize, is_open_arrow: bool) -> f32 {
  use common::StrokeEndSize as Size;
  match (size, is_open_arrow) {
    (Size::Small, false) => 2.0,
    (Size::Medium, false) => 3.0,
    (Size::Large, false) => 5.0,
    (Size::Small, true) => 2.5,
    (Size::Medium, true) => 3.5,
    (Size::Large, true) => 5.5,
  }
}

fn uses_stroked_open_arrow(marker: common::StrokeEnd, line_width: f32) -> bool {
  marker.kind == common::StrokeEndKind::Arrow && line_width >= MIN_MARKER_BASE_PT
}

pub(super) fn stroke_end_dimensions(marker: common::StrokeEnd, line_width: f32) -> (f32, f32) {
  use common::{StrokeEndKind as Kind, StrokeEndSize as Size};

  // LibreOffice's DrawingML importer carries line widths in hundredths of a
  // millimetre here. `lineproperties.cxx::lclPushMarkerProperties` clamps the
  // marker baseline to 70 of those units before applying these multipliers.
  let is_open_arrow = marker.kind == Kind::Arrow;
  let baseline = line_width.max(MIN_MARKER_BASE_PT);
  if marker.kind == Kind::Arrow
    && marker.width == Size::Medium
    && marker.length == Size::Medium
    && !uses_stroked_open_arrow(marker, line_width)
  {
    // Include the stroke envelope of the approximately 30-degree arms at the
    // fixed minimum. Independent 0.75pt and 2pt Word goldens retain this
    // projection; the authored-width branch below uses a stroked centerline.
    return (
      3.5 * baseline + (3.0_f32.sqrt() / 2.0) * line_width,
      3.0 * baseline + 0.75 * line_width,
    );
  }
  (
    stroke_end_size_factor(marker.width, is_open_arrow) * baseline,
    stroke_end_size_factor(marker.length, is_open_arrow) * baseline,
  )
}

fn stroked_open_arrow_geometry(
  marker: common::StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  line_width: f32,
) -> Option<StrokeMarkerGeometry> {
  if !uses_stroked_open_arrow(marker, line_width) {
    return None;
  }
  let baseline = line_width.max(MIN_MARKER_BASE_PT);
  let marker_width = stroke_end_size_factor(marker.width, true) * baseline;
  let marker_length = stroke_end_size_factor(marker.length, true) * baseline;
  let radius = line_width / 2.0;
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
  let point = |back: f32, across: f32| {
    (
      endpoint.0 - outward.0 * back + perpendicular.0 * across,
      endpoint.1 - outward.1 * back + perpendicular.1 * across,
    )
  };
  Some(StrokeMarkerGeometry::StrokedOpen {
    points: [
      point(base_offset, -half_width),
      point(miter_inset, 0.0),
      point(base_offset, half_width),
    ],
    width_pt: line_width,
  })
}

fn filled_marker_geometry(
  marker: common::StrokeEnd,
  endpoint: (f32, f32),
  outward: (f32, f32),
  line_width: f32,
) -> Option<StrokeMarkerGeometry> {
  use common::StrokeEndKind as Kind;
  if marker.kind == Kind::None {
    return None;
  }
  let (width, length) = stroke_end_dimensions(marker, line_width);
  let centered = matches!(marker.kind, Kind::Diamond | Kind::Oval);
  let line_half_width = (50.0 * line_width / width).max(1.0);
  let points: &[(f32, f32)] = match marker.kind {
    Kind::Triangle => &[(50.0, 0.0), (100.0, 100.0), (0.0, 100.0)],
    Kind::Stealth => &[(50.0, 0.0), (100.0, 100.0), (50.0, 60.0), (0.0, 100.0)],
    Kind::Diamond => &[(50.0, 0.0), (100.0, 50.0), (50.0, 100.0), (0.0, 50.0)],
    Kind::Oval => &[
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
    Kind::Arrow => &[
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
    Kind::None => return None,
  };
  let perpendicular = (-outward.1, outward.0);
  let transform = |x: f32, y: f32| {
    let across = (x / 100.0 - 0.5) * width;
    let back = (y / 100.0 - if centered { 0.5 } else { 0.0 }) * length;
    (
      endpoint.0 - outward.0 * back + perpendicular.0 * across,
      endpoint.1 - outward.1 * back + perpendicular.1 * across,
    )
  };
  Some(StrokeMarkerGeometry::Filled {
    points: points.iter().map(|&(x, y)| transform(x, y)).collect(),
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk_layout::common::{Color, Fill, Point, Pt, Stroke, StrokeEnd, StrokeEndKind};

  fn geometry(kind: StrokeEndKind, line_width: f32) -> Option<StrokeMarkerGeometry> {
    let points = [
      Point {
        x: Pt(10.0),
        y: Pt(50.0),
      },
      Point {
        x: Pt(90.0),
        y: Pt(50.0),
      },
    ];
    let stroke = Stroke {
      width: Pt(line_width),
      color: Color {
        r: 10,
        g: 20,
        b: 30,
        a: 255,
      },
      head_end: Some(StrokeEnd {
        kind,
        width: common::StrokeEndSize::Medium,
        length: common::StrokeEndSize::Medium,
      }),
      ..Stroke::default()
    };
    let polyline = PolylineItem {
      x_pt: 10.0,
      y_pt: 50.0,
      width_pt: 80.0,
      height_pt: 0.0,
      points: &points,
      commands: &[],
      closed: false,
      fill: &Fill::None,
      stroke: Some(&stroke),
      separate_fill_and_stroke: false,
    };
    stroke_marker_geometries(&polyline, &stroke)[0].clone()
  }

  #[test]
  fn marker_endpoint_directions_ignore_repeated_vertices() {
    let start = Point {
      x: Pt(0.0),
      y: Pt(0.0),
    };
    let corner = Point {
      x: Pt(10.0),
      y: Pt(0.0),
    };
    let end = Point {
      x: Pt(10.0),
      y: Pt(10.0),
    };
    for leading in 1..=3 {
      for trailing in 1..=3 {
        let mut points = vec![start; leading];
        points.push(corner);
        points.extend(std::iter::repeat_n(end, trailing));
        let mut path = PolylineItem {
          x_pt: 0.0,
          y_pt: 0.0,
          width_pt: 10.0,
          height_pt: 10.0,
          points: &points,
          commands: &[],
          closed: false,
          fill: &Fill::None,
          stroke: None,
          separate_fill_and_stroke: false,
        };
        let check = |path: &PolylineItem<'_>| {
          let endpoints = path_endpoints(path).expect("nondegenerate path");
          assert_eq!(endpoints.start, (0.0, 0.0));
          assert_eq!(endpoints.end, (10.0, 10.0));
          assert_eq!(endpoints.start_outward, (-1.0, 0.0));
          assert_eq!(endpoints.end_outward, (0.0, 1.0));
        };
        check(&path);
        let commands = std::iter::once(common::PathCommand::MoveTo(start))
          .chain(
            points
              .iter()
              .skip(1)
              .copied()
              .map(common::PathCommand::LineTo),
          )
          .collect::<Vec<_>>();
        path.commands = &commands;
        check(&path);
        path.closed = true;
        assert!(path_endpoints(&path).is_none());
      }
    }
  }

  #[test]
  fn every_drawingml_marker_kind_has_a_distinct_complete_geometry() {
    let filled_point_counts = [
      (StrokeEndKind::Triangle, 3),
      (StrokeEndKind::Stealth, 4),
      (StrokeEndKind::Diamond, 4),
      (StrokeEndKind::Oval, 12),
      (StrokeEndKind::Arrow, 9),
    ];
    for (kind, count) in filled_point_counts {
      let Some(StrokeMarkerGeometry::Filled { points }) = geometry(kind, 0.75) else {
        panic!("{kind:?} must have filled marker geometry at the fixed minimum");
      };
      assert_eq!(points.len(), count, "{kind:?}");
      assert!(points.iter().flat_map(|&(x, y)| [x, y]).all(f32::is_finite));
    }
    assert!(geometry(StrokeEndKind::None, 0.75).is_none());

    let Some(StrokeMarkerGeometry::StrokedOpen { points, width_pt }) =
      geometry(StrokeEndKind::Arrow, 2.0)
    else {
      panic!("authored-width open arrows must remain stroked centerlines");
    };
    assert_eq!(width_pt, 2.0);
    assert!(
      points
        .into_iter()
        .flat_map(|(x, y)| [x, y])
        .all(f32::is_finite)
    );
  }
}
