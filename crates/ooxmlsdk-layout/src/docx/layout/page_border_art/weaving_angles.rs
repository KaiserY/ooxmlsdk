//! ST_Border/weavingAngles has four corner and four elongated edge images
//! (ECMA-376 Part1,17.18.2/AnnexF). Intrinsic MSART6 WMF coordinates below
//! agree with all eight directional resources in independent native page EMFs.

use super::{ArtCell, ArtPageCoordinates, ArtTile, Page, PageItem, distribute_edge_cells};
use crate::common::{Color, Fill, PathCommand, PathItem, Pt, Stroke, StrokeCap, StrokeJoin};
use crate::model::common_rect;
use kurbo::{Ellipse, PathEl, Shape};

#[derive(Clone, Copy)]
enum Primitive {
  Rectangle([i16; 4]),
  Ellipse([i16; 4]),
  Polygon(&'static [(i16, i16)]),
  Polyline(&'static [(i16, i16)]),
  Line([(i16, i16); 2]),
}

const TOP_LEFT: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 375, 375]),
  Primitive::Polyline(&[(360, 0), (0, 0), (0, 375)]),
  Primitive::Polyline(&[(45, 360), (45, 45), (375, 45)]),
  Primitive::Polygon(&[(375, 300), (300, 375), (375, 375)]),
  Primitive::Ellipse([181, 157, 246, 216]),
  Primitive::Polygon(&[(188, 251), (159, 240), (147, 226), (141, 203), (91, 301)]),
  Primitive::Polygon(&[(223, 251), (252, 240), (264, 226), (270, 203), (320, 301)]),
  Primitive::Polygon(&[(188, 127), (159, 138), (147, 152), (141, 175), (91, 77)]),
  Primitive::Polygon(&[(223, 127), (252, 138), (264, 152), (270, 175), (320, 77)]),
];

const TOP: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 1035, 375]),
  Primitive::Polygon(&[
    (0, 300),
    (270, 30),
    (780, 30),
    (1035, 285),
    (1035, 360),
    (750, 75),
    (300, 75),
    (0, 375),
  ]),
  Primitive::Polyline(&[(0, 60), (270, 330), (765, 330), (1035, 60)]),
  Primitive::Polyline(&[(0, 0), (285, 285), (750, 285), (1035, 0)]),
  Primitive::Ellipse([476, 153, 559, 222]),
  Primitive::Polygon(&[(580, 149), (601, 170), (601, 201), (580, 222), (797, 180)]),
  Primitive::Polygon(&[(455, 149), (434, 170), (434, 201), (455, 222), (238, 180)]),
];

const TOP_RIGHT: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 375, 375]),
  Primitive::Polygon(&[(0, 300), (75, 375), (0, 375)]),
  Primitive::Ellipse([129, 157, 194, 216]),
  Primitive::Polygon(&[(187, 251), (216, 240), (228, 226), (234, 203), (284, 301)]),
  Primitive::Polygon(&[(152, 251), (123, 240), (111, 226), (105, 203), (55, 301)]),
  Primitive::Polygon(&[(187, 127), (216, 138), (228, 152), (234, 175), (284, 77)]),
  Primitive::Polygon(&[(152, 127), (123, 138), (111, 152), (105, 175), (55, 77)]),
  Primitive::Line([(0, 0), (375, 0)]),
  Primitive::Line([(360, 0), (360, 375)]),
  Primitive::Line([(0, 45), (330, 45)]),
  Primitive::Line([(315, 45), (315, 375)]),
];

const RIGHT: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 375, 1035]),
  Primitive::Polygon(&[
    (15, 0),
    (90, 0),
    (345, 255),
    (345, 765),
    (75, 1035),
    (0, 1035),
    (300, 735),
    (300, 285),
    (15, 0),
  ]),
  Primitive::Polyline(&[(360, 0), (60, 300), (60, 735), (375, 1035)]),
  Primitive::Polyline(&[(300, 0), (15, 285), (15, 750), (300, 1035)]),
  Primitive::Ellipse([138, 461, 207, 544]),
  Primitive::Polygon(&[(211, 440), (190, 419), (159, 419), (138, 440), (180, 223)]),
  Primitive::Polygon(&[(211, 565), (190, 586), (159, 586), (138, 565), (180, 782)]),
];

const BOTTOM_RIGHT: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 375, 375]),
  Primitive::Polygon(&[(0, 63), (60, 1), (0, 1)]),
  Primitive::Ellipse([144, 150, 209, 212]),
  Primitive::Polygon(&[(187, 114), (216, 125), (228, 140), (234, 164), (284, 62)]),
  Primitive::Polygon(&[(152, 114), (123, 125), (111, 140), (105, 164), (55, 62)]),
  Primitive::Polygon(&[(187, 243), (216, 232), (228, 217), (234, 193), (284, 295)]),
  Primitive::Polygon(&[(152, 243), (123, 232), (111, 217), (105, 193), (55, 295)]),
  Primitive::Polyline(&[(360, 0), (360, 375), (0, 375)]),
  Primitive::Polyline(&[(315, 0), (315, 330), (0, 330)]),
];

const BOTTOM: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 1035, 375]),
  Primitive::Polygon(&[
    (0, 75),
    (270, 345),
    (780, 345),
    (1035, 90),
    (1035, 15),
    (750, 300),
    (300, 300),
    (0, 0),
  ]),
  Primitive::Polyline(&[(0, 315), (270, 45), (765, 45), (1035, 315)]),
  Primitive::Polyline(&[(0, 375), (285, 90), (750, 90), (1035, 375)]),
  Primitive::Ellipse([476, 153, 559, 222]),
  Primitive::Polygon(&[(580, 226), (601, 205), (601, 174), (580, 153), (797, 195)]),
  Primitive::Polygon(&[(455, 226), (434, 205), (434, 174), (455, 153), (238, 195)]),
];

const BOTTOM_LEFT: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 375, 375]),
  Primitive::Polygon(&[(375, 66), (315, 1), (375, 1)]),
  Primitive::Ellipse([181, 157, 246, 220]),
  Primitive::Polygon(&[(188, 119), (159, 131), (147, 146), (141, 171), (91, 65)]),
  Primitive::Polygon(&[(223, 119), (252, 131), (264, 146), (270, 171), (320, 65)]),
  Primitive::Polygon(&[(188, 253), (159, 241), (147, 226), (141, 201), (91, 307)]),
  Primitive::Polygon(&[(223, 253), (252, 241), (264, 226), (270, 201), (320, 307)]),
  Primitive::Polyline(&[(0, 0), (0, 375), (375, 375)]),
  Primitive::Polyline(&[(45, 0), (45, 330), (375, 330)]),
];

const LEFT: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 375, 1035]),
  Primitive::Polygon(&[
    (360, 0),
    (285, 0),
    (30, 255),
    (30, 765),
    (300, 1035),
    (375, 1035),
    (75, 735),
    (75, 285),
    (360, 0),
  ]),
  Primitive::Polyline(&[(15, 0), (315, 300), (315, 735), (15, 1035)]),
  Primitive::Polyline(&[(75, 0), (360, 285), (360, 750), (75, 1035)]),
  Primitive::Ellipse([168, 461, 237, 544]),
  Primitive::Polygon(&[(164, 440), (185, 419), (216, 419), (237, 440), (195, 223)]),
  Primitive::Polygon(&[(164, 565), (185, 586), (216, 586), (237, 565), (195, 782)]),
];

pub(super) fn edge_cells(start: i32, end: i32, corner: i32) -> Vec<(i32, i32)> {
  let span = end - start;
  if corner <= 0 || span < 2 * corner {
    return Vec::new();
  }
  // The source middle window is1035x375, whereas corners are375x375.
  // Native count transitions at6,9,13,16,17,23and31pt establish rounding
  // the scaled middle extent first, then adding its two endpoint pixels.
  let pitch = (f64::from(corner) * 1035.0 / 375.0).round() as i32 + 2;
  let count = (span - 2 * corner) / pitch + 2;
  distribute_edge_cells(start, end, corner, count)
}

fn printer_box(rect: [i16; 4], tile: ArtTile, window: (f64, f64)) -> [f64; 4] {
  let axis = |value: i16, origin: i32, extent: i32, source: f64| {
    let device = f64::from(origin) + f64::from(value) * f64::from(extent) / source;
    (device * 16.0).round() / 16.0
  };
  [
    axis(rect[0], tile.x, tile.width, window.0),
    axis(rect[1], tile.y, tile.height, window.1),
    axis(rect[2], tile.x, tile.width, window.0) - 1.0,
    axis(rect[3], tile.y, tile.height, window.1) - 1.0,
  ]
}

pub(super) fn paint(page: &mut Page, coordinates: ArtPageCoordinates, tile: ArtTile, ink: Color) {
  let (source, window) = match tile.cell {
    ArtCell::TopLeft => (TOP_LEFT, (375.0, 375.0)),
    ArtCell::Top => (TOP, (1035.0, 375.0)),
    ArtCell::TopRight => (TOP_RIGHT, (375.0, 375.0)),
    ArtCell::Right => (RIGHT, (375.0, 1035.0)),
    ArtCell::BottomLeft => (BOTTOM_LEFT, (375.0, 375.0)),
    ArtCell::Bottom => (BOTTOM, (1035.0, 375.0)),
    ArtCell::BottomRight => (BOTTOM_RIGHT, (375.0, 375.0)),
    ArtCell::Left => (LEFT, (375.0, 1035.0)),
  };
  let mut items = Vec::with_capacity(source.len());
  for primitive in source {
    let mut commands = Vec::new();
    let white = Color {
      r: 255,
      g: 255,
      b: 255,
      a: 255,
    };
    let (fill, stroke, cap, closed) = match *primitive {
      Primitive::Rectangle(rect) => {
        let [left, top, right, bottom] = printer_box(rect, tile, window);
        commands.extend([
          PathCommand::MoveTo(coordinates.point(left, top, false)),
          PathCommand::LineTo(coordinates.point(right, top, false)),
          PathCommand::LineTo(coordinates.point(right, bottom, false)),
          PathCommand::LineTo(coordinates.point(left, bottom, false)),
          PathCommand::Close,
        ]);
        (Fill::Solid(white), white, StrokeCap::Flat, true)
      }
      Primitive::Ellipse(rect) => {
        let [left, top, right, bottom] = printer_box(rect, tile, window);
        let ellipse = Ellipse::new(
          ((left + right) / 2.0, (top + bottom) / 2.0),
          ((right - left) / 2.0, (bottom - top) / 2.0),
          0.0,
        );
        for element in ellipse.to_path(0.001).elements() {
          let point = |p: kurbo::Point| coordinates.point(p.x, p.y, false);
          commands.push(match *element {
            PathEl::MoveTo(p) => PathCommand::MoveTo(point(p)),
            PathEl::LineTo(p) => PathCommand::LineTo(point(p)),
            PathEl::CurveTo(a, b, c) => PathCommand::CubicTo {
              control1: point(a),
              control2: point(b),
              end: point(c),
            },
            PathEl::ClosePath => PathCommand::Close,
            PathEl::QuadTo(..) => unreachable!("ellipse has cubic contours"),
          });
        }
        (Fill::Solid(white), ink, StrokeCap::Flat, true)
      }
      Primitive::Polygon(points) | Primitive::Polyline(points) => {
        let closed = matches!(primitive, Primitive::Polygon(_));
        append_points(&mut commands, coordinates, tile, window, points);
        if closed {
          commands.push(PathCommand::Close);
        }
        (
          if closed { Fill::Solid(ink) } else { Fill::None },
          ink,
          StrokeCap::Flat,
          closed,
        )
      }
      Primitive::Line([start, end]) => {
        // LineTo excludes the terminal logical pixel. The existing classic
        // Word pen adapter uses a square cap at that last pixel; Polyline
        // instead retains its complete source points and flat caps.
        let end = (
          end.0 - (end.0 - start.0).signum(),
          end.1 - (end.1 - start.1).signum(),
        );
        append_points(&mut commands, coordinates, tile, window, &[start, end]);
        (Fill::None, ink, StrokeCap::Square, false)
      }
    };
    let bounds = if let [PathCommand::MoveTo(start), PathCommand::LineTo(end)] = commands.as_slice()
    {
      // A horizontal/vertical LineTo has a valid zero-area centerline.
      // The drawing-effect bounds helper deliberately rejects those paths;
      // keep the centerline here and let the stroke own its visible width.
      common_rect(
        start.x.0.min(end.x.0),
        start.y.0.min(end.y.0),
        (end.x.0 - start.x.0).abs(),
        (end.y.0 - start.y.0).abs(),
      )
    } else {
      super::super::drawing_path_bounds(&commands).expect("finite WeavingAngles artwork")
    };
    let points = commands
      .iter()
      .filter_map(|command| match command {
        PathCommand::MoveTo(p) | PathCommand::LineTo(p) => Some(*p),
        PathCommand::CubicTo { end, .. } => Some(*end),
        _ => None,
      })
      .collect();
    items.push(PageItem::path(PathItem {
      bounds,
      points,
      commands,
      closed,
      fill,
      stroke: Some(Stroke {
        width: Pt(0.14),
        color: stroke,
        cap: Some(cap),
        join: Some(StrokeJoin::Round),
        ..Stroke::default()
      }),
    }));
  }
  // Native playback clips to the page, not each tile. Cosmetic contours
  // can cross a cell boundary and are covered by the next resource's white
  // background in paint order.
  page.items.push(PageItem::CompositingGroup {
    items,
    opacity: 1.0,
    clip: Some(common_rect(
      0.0,
      0.0,
      page.setup.width_pt,
      page.setup.height_pt,
    )),
  });
}

fn append_points(
  commands: &mut Vec<PathCommand>,
  coordinates: ArtPageCoordinates,
  tile: ArtTile,
  window: (f64, f64),
  points: &[(i16, i16)],
) {
  for (index, &(x, y)) in points.iter().enumerate() {
    let point = coordinates.point(
      f64::from(tile.x) + f64::from(x) * f64::from(tile.width) / window.0,
      f64::from(tile.y) + f64::from(y) * f64::from(tile.height) / window.1,
      true,
    );
    commands.push(if index == 0 {
      PathCommand::MoveTo(point)
    } else {
      PathCommand::LineTo(point)
    });
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn weaving_angles_native_count_boundaries_preserve_corners() {
    // Native adjacent-device-dot page widths distinguish rounding the
    // source aspect ratio before versus after reserving the two corners.
    for (corner, span, count) in [
      (50, 1639, 12),
      (50, 1640, 13),
      (75, 1821, 9),
      (75, 1822, 10),
      (108, 1715, 6),
      (108, 1716, 7),
      (133, 2110, 6),
      (133, 2111, 7),
      (142, 1859, 5),
      (142, 1860, 6),
      (192, 2511, 5),
      (192, 2512, 6),
      (258, 3371, 5),
      (258, 3372, 6),
    ] {
      let cells = edge_cells(0, span, corner);
      assert_eq!(cells.len(), count);
      assert_eq!(cells[0], (0, corner));
      assert_eq!(cells[count - 1], (span - corner, corner));
      assert_eq!(cells.iter().map(|(_, w)| w).sum::<i32>(), span);
      assert!(cells[1..count - 1].windows(2).all(|p| p[0].1 <= p[1].1));
    }
  }

  #[test]
  fn weaving_angles_templates_retain_native_direction_and_primitive_kinds() {
    for (source, count, lines) in [
      (TOP_LEFT, 9, 0),
      (TOP, 7, 0),
      (TOP_RIGHT, 11, 4),
      (RIGHT, 7, 0),
      (BOTTOM_LEFT, 9, 0),
      (BOTTOM, 7, 0),
      (BOTTOM_RIGHT, 9, 0),
      (LEFT, 7, 0),
    ] {
      assert_eq!(source.len(), count);
      assert_eq!(
        source
          .iter()
          .filter(|p| matches!(p, Primitive::Line(_)))
          .count(),
        lines
      );
      assert_eq!(
        source
          .iter()
          .filter(|p| matches!(p, Primitive::Ellipse(_)))
          .count(),
        1
      );
      assert!(matches!(source[0], Primitive::Rectangle(_)));
    }
  }
}
