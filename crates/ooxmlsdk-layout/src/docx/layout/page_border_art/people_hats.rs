//! ECMA-376 ST_Border/peopleHats: one all-sides character motif.
//! Intrinsic MSART2 resource geometry/palette/pen facts are matched
//! independently to actual native EMF playback. Device boxes and
//! page placement use the existing Word GDI/art coordinate rules.

use super::{ArtPageCoordinates, ArtTile, DOT, Page, PageItem, square_printer_box};
use crate::common::{
  Color, Fill, PathCommand, PathItem, Pt, Stroke, StrokeAlignment, StrokeCap, StrokeJoin,
};
use crate::model::common_rect;
use kurbo::{Ellipse, PathEl, Shape as _};

const WINDOW: f64 = 765.0;
const BLACK: Color = Color {
  r: 0,
  g: 0,
  b: 0,
  a: 255,
};
const WHITE: Color = Color {
  r: 255,
  g: 255,
  b: 255,
  a: 255,
};
const GRAY: Color = Color {
  r: 192,
  g: 192,
  b: 192,
  a: 255,
};
const YELLOW: Color = Color {
  r: 255,
  g: 255,
  b: 128,
  a: 255,
};
const BLUE: Color = Color {
  r: 0,
  g: 0,
  b: 128,
  a: 255,
};

#[derive(Clone, Copy)]
enum Shape {
  Rectangle([i16; 4]),
  Ellipse([i16; 4]),
  Polygon(&'static [(i16, i16)]),
  Line([(i16, i16); 2]),
}

struct Primitive {
  shape: Shape,
  fill: Color,
  pen: Color,
  width: u16,
  inside: bool,
}

const PRIMITIVES: [Primitive; 14] = [
  Primitive {
    shape: Shape::Rectangle([0, 0, 765, 765]),
    fill: WHITE,
    pen: WHITE,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Ellipse([363, 96, 489, 237]),
    fill: GRAY,
    pen: GRAY,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Polygon(&[
      (330, 285),
      (150, 465),
      (195, 480),
      (330, 390),
      (330, 735),
      (375, 735),
      (420, 525),
      (435, 510),
      (495, 735),
      (540, 735),
      (540, 390),
      (660, 180),
      (660, 135),
      (510, 285),
    ]),
    fill: GRAY,
    pen: GRAY,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Ellipse([318, 96, 444, 237]),
    fill: GRAY,
    pen: BLACK,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Polygon(&[
      (285, 270),
      (105, 450),
      (150, 465),
      (285, 375),
      (285, 720),
      (330, 720),
      (375, 510),
      (390, 495),
      (450, 720),
      (495, 720),
      (495, 375),
      (615, 165),
      (615, 120),
      (465, 270),
    ]),
    fill: YELLOW,
    pen: BLACK,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Line([(285, 645), (345, 645)]),
    fill: YELLOW,
    pen: BLACK,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Line([(495, 645), (420, 645)]),
    fill: YELLOW,
    pen: BLACK,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Polygon(&[(330, 660), (300, 660), (300, 705), (315, 705), (330, 660)]),
    fill: GRAY,
    pen: GRAY,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Polygon(&[(450, 660), (480, 660), (480, 705), (465, 705), (450, 675)]),
    fill: GRAY,
    pen: GRAY,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Polygon(&[(600, 150), (600, 150), (585, 195), (570, 180)]),
    fill: GRAY,
    pen: GRAY,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Polygon(&[(135, 435), (165, 405), (150, 450), (135, 450)]),
    fill: GRAY,
    pen: GRAY,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Ellipse([360, 315, 405, 360]),
    fill: BLUE,
    pen: BLUE,
    width: 0,
    inside: false,
  },
  Primitive {
    shape: Shape::Ellipse([300, 75, 450, 150]),
    fill: BLUE,
    pen: BLUE,
    width: 30,
    inside: true,
  },
  Primitive {
    shape: Shape::Ellipse([360, 30, 390, 105]),
    fill: BLUE,
    pen: BLUE,
    width: 30,
    inside: true,
  },
];

pub(super) fn paint(page: &mut Page, coordinates: ArtPageCoordinates, tile: ArtTile) {
  let mut items = Vec::with_capacity(PRIMITIVES.len());
  for primitive in &PRIMITIVES {
    let mut commands = Vec::new();
    let (fill, closed, cap) = match primitive.shape {
      Shape::Rectangle(rect) => {
        let [left, top, right, bottom] = square_printer_box(rect, tile, WINDOW);
        commands.extend([
          PathCommand::MoveTo(coordinates.point(left, top, false)),
          PathCommand::LineTo(coordinates.point(right, top, false)),
          PathCommand::LineTo(coordinates.point(right, bottom, false)),
          PathCommand::LineTo(coordinates.point(left, bottom, false)),
          PathCommand::Close,
        ]);
        (Fill::Solid(primitive.fill), true, StrokeCap::Flat)
      }
      Shape::Ellipse(rect) => {
        let [left, top, right, bottom] = square_printer_box(rect, tile, WINDOW);
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
        (Fill::Solid(primitive.fill), true, StrokeCap::Flat)
      }
      Shape::Polygon(points) => {
        append_points(&mut commands, coordinates, tile, points);
        commands.push(PathCommand::Close);
        (Fill::Solid(primitive.fill), true, StrokeCap::Flat)
      }
      Shape::Line([start, end]) => {
        // Classic cosmetic LineTo excludes its terminal logical pixel;
        // unlike Polygon, the retained last pixel has a square cap.
        let end = (
          end.0 - (end.0 - start.0).signum(),
          end.1 - (end.1 - start.1).signum(),
        );
        append_points(&mut commands, coordinates, tile, &[start, end]);
        (Fill::None, false, StrokeCap::Square)
      }
    };
    let bounds = if let [PathCommand::MoveTo(start), PathCommand::LineTo(end)] = commands.as_slice()
    {
      common_rect(
        start.x.0.min(end.x.0),
        start.y.0.min(end.y.0),
        (end.x.0 - start.x.0).abs(),
        (end.y.0 - start.y.0).abs(),
      )
    } else {
      super::super::drawing_path_bounds(&commands).expect("finite built-in PeopleHats motif")
    };
    let points = commands
      .iter()
      .filter_map(|command| match command {
        PathCommand::MoveTo(p) | PathCommand::LineTo(p) => Some(*p),
        PathCommand::CubicTo { end, .. } => Some(*end),
        _ => None,
      })
      .collect();
    let width = (((f64::from(primitive.width) * f64::from(tile.width) / WINDOW * 5.0).round() * DOT
      / 5.0) as f32)
      .max(0.14);
    items.push(PageItem::path(PathItem {
      bounds,
      points,
      commands,
      closed,
      fill,
      stroke: Some(Stroke {
        width: Pt(width),
        color: primitive.pen,
        cap: Some(cap),
        join: Some(StrokeJoin::Round),
        alignment: primitive.inside.then_some(StrokeAlignment::Inside),
        ..Stroke::default()
      }),
    }));
  }
  // Native classic resources retain page clipping; adjacent white
  // backgrounds cover any shared cosmetic contour in playback order.
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
  points: &[(i16, i16)],
) {
  for (index, &(x, y)) in points.iter().enumerate() {
    let point = coordinates.point(
      f64::from(tile.x) + f64::from(x) * f64::from(tile.width) / WINDOW,
      f64::from(tile.y) + f64::from(y) * f64::from(tile.height) / WINDOW,
      true,
    );
    commands.push(if index == 0 {
      PathCommand::MoveTo(point)
    } else {
      PathCommand::LineTo(point)
    });
  }
}
