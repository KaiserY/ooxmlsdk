//! Standard ST_Border/gems artwork: left corner, edge and right corner.
//! The three intrinsic WMF resources (MSART14 Gems) and independent native
//! page EMFs agree on these primitive coordinates, order and gray palette.
//! Runtime needs no Office resources or document/reference bytes.

use super::{
  ArtCell, ArtPageCoordinates, ArtTile, DOT, Page, PageItem, square_printer_box as printer_box,
};
use crate::common::{
  Color, Fill, PathCommand, PathItem, Pt, Stroke, StrokeAlignment, StrokeCap, StrokeJoin,
};
use kurbo::{Ellipse, PathEl, Shape};

#[derive(Clone, Copy)]
enum Primitive {
  Rectangle([i16; 4], u8, u16),
  Ellipse([i16; 4], u8, u16),
  Polygon(&'static [(i16, i16)], u8),
}

const LEFT: &[Primitive] = &[
  Primitive::Rectangle([0, 0, 2410, 2409], 255, 0),
  Primitive::Rectangle([384, 396, 2019, 2079], 0, 0),
  Primitive::Rectangle([560, 588, 1831, 1887], 255, 0),
  Primitive::Rectangle([1428, 0, 2415, 987], 0, 0),
  Primitive::Rectangle([1512, 96, 2319, 915], 255, 0),
  Primitive::Polygon(&[(1512, 900), (1512, 216), (2196, 900)], 128),
  Primitive::Rectangle([1572, 156, 2271, 855], 255, 45),
  Primitive::Polygon(&[(2184, 228), (2184, 888), (1524, 228)], 192),
  Primitive::Ellipse([0, 720, 975, 1683], 0, 0),
  Primitive::Ellipse([72, 780, 915, 1611], 255, 0),
  Primitive::Polygon(
    &[
      (232, 876),
      (804, 1448),
      (749, 1515),
      (684, 1555),
      (618, 1581),
      (564, 1595),
      (370, 1608),
      (401, 1605),
      (326, 1568),
      (261, 1553),
      (208, 1514),
      (154, 1462),
      (139, 1422),
      (99, 1355),
      (73, 1299),
      (60, 1235),
      (60, 1116),
      (87, 1050),
      (125, 982),
      (178, 916),
      (218, 876),
    ],
    128,
  ),
  Primitive::Ellipse([120, 828, 867, 1575], 192, 60),
  Primitive::Polygon(
    &[
      (252, 912),
      (768, 1428),
      (708, 1488),
      (660, 1524),
      (600, 1548),
      (552, 1560),
      (420, 1572),
      (360, 1548),
      (348, 1548),
      (300, 1524),
      (252, 1488),
      (204, 1440),
      (168, 1404),
      (132, 1344),
      (120, 1272),
      (108, 1236),
      (108, 1128),
      (132, 1080),
      (156, 1008),
      (204, 948),
      (240, 912),
    ],
    255,
  ),
  Primitive::Polygon(
    &[
      (252, 900),
      (180, 960),
      (132, 1056),
      (120, 1104),
      (108, 1128),
      (108, 1272),
      (132, 1344),
      (144, 1380),
      (252, 1500),
      (360, 1560),
      (408, 1572),
      (552, 1572),
      (600, 1560),
      (672, 1524),
      (720, 1476),
      (744, 1464),
      (768, 1428),
      (732, 1392),
      (648, 1464),
      (564, 1512),
      (420, 1524),
      (384, 1512),
      (336, 1488),
      (264, 1428),
      (204, 1356),
      (180, 1308),
      (168, 1176),
      (180, 1104),
      (204, 1032),
      (240, 984),
      (288, 948),
    ],
    0,
  ),
  Primitive::Rectangle([1428, 1428, 2415, 2415], 0, 0),
  Primitive::Rectangle([1512, 1524, 2319, 2343], 255, 0),
  Primitive::Polygon(&[(1512, 2328), (1512, 1644), (2196, 2328)], 128),
  Primitive::Rectangle([1572, 1584, 2271, 2283], 255, 45),
  Primitive::Polygon(&[(2184, 1656), (2184, 2316), (1524, 1656)], 192),
];

const EDGE: &[Primitive] = &[
  Primitive::Rectangle([5, 0, 2415, 2409], 255, 0),
  Primitive::Rectangle([396, 396, 2031, 2079], 0, 0),
  Primitive::Rectangle([584, 588, 1855, 1887], 255, 0),
  Primitive::Rectangle([0, 0, 987, 987], 0, 0),
  Primitive::Rectangle([96, 96, 903, 915], 255, 0),
  Primitive::Polygon(&[(888, 900), (888, 216), (204, 900)], 128),
  Primitive::Rectangle([144, 156, 843, 855], 255, 45),
  Primitive::Polygon(&[(216, 228), (216, 888), (876, 228)], 192),
  Primitive::Rectangle([1440, 1440, 2427, 2427], 0, 0),
  Primitive::Rectangle([1524, 1512, 2331, 2331], 255, 0),
  Primitive::Polygon(&[(1524, 1512), (1524, 2196), (2208, 1512)], 128),
  Primitive::Rectangle([1584, 1572, 2283, 2271], 255, 45),
  Primitive::Polygon(&[(2196, 2184), (2196, 1524), (1536, 2184)], 192),
  Primitive::Rectangle([1440, 0, 2427, 987], 0, 0),
  Primitive::Rectangle([1512, 96, 2331, 903], 255, 0),
  Primitive::Polygon(&[(1512, 888), (2196, 888), (1512, 204)], 128),
  Primitive::Rectangle([1572, 144, 2271, 843], 255, 45),
  Primitive::Polygon(&[(2184, 216), (1524, 216), (2184, 876)], 192),
  Primitive::Rectangle([12, 1428, 999, 2415], 0, 0),
  Primitive::Rectangle([108, 1512, 927, 2319], 255, 0),
  Primitive::Polygon(&[(912, 1512), (228, 1512), (912, 2196)], 128),
  Primitive::Rectangle([168, 1572, 867, 2271], 255, 45),
  Primitive::Polygon(&[(240, 2184), (900, 2184), (240, 1524)], 192),
];

const RIGHT: &[Primitive] = &[
  Primitive::Rectangle([5, 6, 2415, 2415], 255, 0),
  Primitive::Rectangle([396, 336, 2031, 2019], 0, 0),
  Primitive::Rectangle([584, 528, 1855, 1827], 255, 0),
  Primitive::Rectangle([0, 1428, 987, 2415], 0, 0),
  Primitive::Rectangle([96, 1500, 903, 2319], 255, 0),
  Primitive::Polygon(&[(888, 1500), (888, 2184), (204, 1500)], 128),
  Primitive::Rectangle([144, 1560, 843, 2259], 255, 45),
  Primitive::Polygon(&[(216, 2172), (216, 1512), (876, 2172)], 192),
  Primitive::Ellipse([1440, 732, 2415, 1695], 0, 0),
  Primitive::Ellipse([1500, 804, 2343, 1635], 255, 0),
  Primitive::Polygon(
    &[
      (2168, 1524),
      (1596, 952),
      (1651, 885),
      (1716, 845),
      (1782, 819),
      (1836, 805),
      (2030, 792),
      (1999, 795),
      (2074, 832),
      (2139, 847),
      (2192, 886),
      (2246, 938),
      (2261, 978),
      (2301, 1045),
      (2327, 1101),
      (2340, 1165),
      (2340, 1284),
      (2313, 1350),
      (2275, 1418),
      (2222, 1484),
      (2182, 1524),
    ],
    128,
  ),
  Primitive::Ellipse([1548, 840, 2295, 1587], 192, 60),
  Primitive::Polygon(
    &[
      (2148, 1488),
      (1632, 972),
      (1692, 912),
      (1740, 876),
      (1800, 852),
      (1848, 840),
      (1980, 828),
      (2040, 852),
      (2052, 852),
      (2100, 876),
      (2148, 912),
      (2196, 960),
      (2232, 996),
      (2268, 1056),
      (2280, 1128),
      (2292, 1164),
      (2292, 1272),
      (2268, 1320),
      (2244, 1392),
      (2196, 1452),
      (2160, 1488),
    ],
    255,
  ),
  Primitive::Polygon(
    &[
      (2148, 1500),
      (2220, 1440),
      (2268, 1344),
      (2280, 1296),
      (2292, 1272),
      (2292, 1128),
      (2268, 1056),
      (2256, 1020),
      (2148, 900),
      (2040, 840),
      (1992, 828),
      (1848, 828),
      (1800, 840),
      (1728, 876),
      (1680, 924),
      (1656, 936),
      (1632, 972),
      (1668, 1008),
      (1752, 936),
      (1836, 888),
      (1980, 876),
      (2016, 888),
      (2064, 912),
      (2136, 972),
      (2196, 1044),
      (2220, 1092),
      (2232, 1224),
      (2220, 1296),
      (2196, 1368),
      (2160, 1416),
      (2112, 1452),
    ],
    0,
  ),
  Primitive::Rectangle([0, 0, 987, 987], 0, 0),
  Primitive::Rectangle([96, 72, 903, 891], 255, 0),
  Primitive::Polygon(&[(888, 72), (888, 756), (204, 72)], 128),
  Primitive::Rectangle([144, 132, 843, 831], 255, 45),
  Primitive::Polygon(&[(216, 744), (216, 84), (876, 744)], 192),
];

pub(super) fn paint(page: &mut Page, coordinates: ArtPageCoordinates, tile: ArtTile, ink: Color) {
  let (source, window) = match tile.cell {
    ArtCell::TopLeft | ArtCell::BottomLeft => (LEFT, 2415.0),
    ArtCell::Top | ArtCell::Bottom | ArtCell::Left | ArtCell::Right => (EDGE, 2427.0),
    ArtCell::TopRight | ArtCell::BottomRight => (RIGHT, 2415.0),
  };
  let mut items = Vec::with_capacity(source.len());
  for primitive in source {
    let mut commands = Vec::new();
    let (shade, pen) = match *primitive {
      Primitive::Rectangle(rect, shade, pen) => {
        let [left, top, right, bottom] = printer_box(rect, tile, window);
        commands.extend([
          PathCommand::MoveTo(coordinates.point(left, top, false)),
          PathCommand::LineTo(coordinates.point(right, top, false)),
          PathCommand::LineTo(coordinates.point(right, bottom, false)),
          PathCommand::LineTo(coordinates.point(left, bottom, false)),
          PathCommand::Close,
        ]);
        (shade, pen)
      }
      Primitive::Ellipse(rect, shade, pen) => {
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
        (shade, pen)
      }
      Primitive::Polygon(points, shade) => {
        for (index, &(x, y)) in points.iter().enumerate() {
          let point = coordinates.point(
            f64::from(tile.x) + f64::from(x) * f64::from(tile.width) / window,
            f64::from(tile.y) + f64::from(y) * f64::from(tile.height) / window,
            false,
          );
          commands.push(if index == 0 {
            PathCommand::MoveTo(point)
          } else {
            PathCommand::LineTo(point)
          });
        }
        commands.push(PathCommand::Close);
        (shade, 0)
      }
    };
    let color = if shade == 0 {
      ink
    } else {
      Color {
        r: shade,
        g: shade,
        b: shade,
        a: 255,
      }
    };
    let bounds = super::super::drawing_path_bounds(&commands).expect("finite built-in Gems motif");
    let points = commands
      .iter()
      .filter_map(|c| match c {
        PathCommand::MoveTo(p) | PathCommand::LineTo(p) => Some(*p),
        PathCommand::CubicTo { end, .. } => Some(*end),
        PathCommand::Close => None,
      })
      .collect();
    let stroke = (pen > 0).then(|| Stroke {
      width: Pt(
        (((f64::from(pen) * f64::from(tile.width) / window * 5.0).round() * DOT / 5.0) as f32)
          .max(0.14),
      ),
      color: ink,
      cap: Some(StrokeCap::Flat),
      join: Some(StrokeJoin::Round),
      alignment: Some(StrokeAlignment::Inside),
      ..Stroke::default()
    });
    items.push(PageItem::path(PathItem {
      bounds,
      points,
      commands,
      closed: true,
      fill: Fill::Solid(color),
      stroke,
    }));
  }
  let start = coordinates.point(f64::from(tile.x), f64::from(tile.y), false);
  let end = coordinates.point(
    f64::from(tile.x + tile.width),
    f64::from(tile.y + tile.height),
    false,
  );
  page.items.push(PageItem::CompositingGroup {
    items,
    opacity: 1.0,
    clip: Some(crate::model::common_rect(
      start.x.0,
      start.y.0,
      end.x.0 - start.x.0,
      end.y.0 - start.y.0,
    )),
  });
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn native_gems_rectangles_keep_device_end_exclusion() {
    let tile = ArtTile {
      x: 0,
      y: 200,
      width: 67,
      height: 67,
      cell: ArtCell::TopLeft,
    };
    let b = printer_box([384, 396, 2019, 2079], tile, 2415.0);
    let coordinates = ArtPageCoordinates::new(612.0, 792.0);
    let start = coordinates.point(b[0], b[1], false);
    let end = coordinates.point(b[2], b[3], false);
    // Independent Office PDF first black frame in the8pt top-left cell.
    assert!((start.x.0 - 1.272).abs() < 0.0001);
    assert!((end.x.0 - 6.6).abs() < 0.0001);
    assert!((start.y.0 - 25.324).abs() < 0.025);
    assert!((end.y.0 - 30.82).abs() < 0.025);
  }
  #[test]
  fn intrinsic_gems_templates_retain_native_corner_and_facet_counts() {
    for (source, rectangles, polygons, ellipses) in
      [(LEFT, 9, 7, 3), (EDGE, 15, 8, 0), (RIGHT, 9, 7, 3)]
    {
      assert_eq!(
        source
          .iter()
          .filter(|p| matches!(p, Primitive::Rectangle(..)))
          .count(),
        rectangles
      );
      assert_eq!(
        source
          .iter()
          .filter(|p| matches!(p, Primitive::Polygon(..)))
          .count(),
        polygons
      );
      assert_eq!(
        source
          .iter()
          .filter(|p| matches!(p, Primitive::Ellipse(..)))
          .count(),
        ellipses
      );
    }
  }
}
