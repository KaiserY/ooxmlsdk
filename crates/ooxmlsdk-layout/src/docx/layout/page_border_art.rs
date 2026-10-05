//! Built-in Word page-border artwork, independent of document content.

use super::{Page, PageItem, WORD_FIXED_OUTPUT_DPI};
use crate::common::{Color, Fill, PathCommand, PathItem, Pt, Stroke, StrokeCap, StrokeJoin};
use crate::docx::CellBordersModel;
use crate::model::{common_point, common_rect};
use crate::units;

const DOT: f64 = units::POINTS_PER_INCH as f64 / WORD_FIXED_OUTPUT_DPI as f64;

/// Distribute complete art cells along an edge. Word reserves the two corner
/// cells at their authored size and stretches the interior cells to cover the
/// remaining integral printer dots. The larger cells come last, rather than
/// alternating with smaller cells. Native EMF viewport extents establish this
/// ordering across three art sizes and two paper sizes.
fn edge_cells(start: i32, end: i32, corner: i32) -> Vec<(i32, i32)> {
  let span = end - start;
  if corner <= 0 || span < corner * 2 {
    return Vec::new();
  }
  // The tile-count calculation includes the terminal device pixel, whereas
  // EMF viewport extents store the coordinate difference. Six-point art is
  // a useful counterexample to simply dividing by the viewport extent.
  let count = (span / (corner + 1)).max(2);
  let mut cells = Vec::with_capacity(count as usize);
  cells.push((start, corner));
  if count > 2 {
    let inner_count = count - 2;
    let inner_span = span - 2 * corner;
    let width = inner_span / inner_count;
    let small_count = inner_count - inner_span % inner_count;
    let mut cursor = start + corner;
    for index in 0..inner_count {
      let size = width + i32::from(index >= small_count);
      cells.push((cursor, size));
      cursor += size;
    }
  }
  cells.push((end - corner, corner));
  cells
}

pub(super) fn paint_maple_muffins(
  page: &mut Page,
  (left, top, right, bottom): (f32, f32, f32, f32),
  borders: CellBordersModel,
) {
  let (Some(t), Some(r), Some(b), Some(l)) =
    (borders.top, borders.right, borders.bottom, borders.left)
  else {
    return;
  };
  let (left, top, right, bottom) = if page.setup.borders_offset_from_text {
    (
      left - l.spacing_pt - l.width_pt,
      top - t.spacing_pt - t.width_pt,
      right + r.spacing_pt + r.width_pt,
      bottom + b.spacing_pt + b.width_pt,
    )
  } else {
    (
      left + l.spacing_pt,
      top + t.spacing_pt,
      right - r.spacing_pt,
      bottom - b.spacing_pt,
    )
  };
  if ![left, top, right, bottom, t.width_pt]
    .iter()
    .all(|v| v.is_finite())
  {
    return;
  }
  let dot = |value: f32| (f64::from(value) / DOT).round() as i32;
  let size = dot(t.width_pt);
  let columns = edge_cells(dot(left), dot(right), size);
  let rows = edge_cells(dot(top), dot(bottom), size);
  if columns.is_empty() || rows.is_empty() {
    return;
  }
  let (x_first, _) = columns[0];
  let (x_last, _) = columns[columns.len() - 1];
  let (y_first, _) = rows[0];
  let (y_last, _) = rows[rows.len() - 1];
  let coordinates = ArtPageCoordinates::new(page.setup.width_pt, page.setup.height_pt);
  // The art remains upright on all four edges, including corners. There is
  // one corner image, shared by the two adjoining edges.
  for (x, width) in columns {
    paint_muffin(page, coordinates, x, y_first, width, size);
    paint_muffin(page, coordinates, x, y_last, width, size);
  }
  for &(y, height) in &rows[1..rows.len() - 1] {
    paint_muffin(page, coordinates, x_first, y, size, height);
    paint_muffin(page, coordinates, x_last, y, size, height);
  }
}

#[derive(Clone, Copy)]
struct ArtPageCoordinates {
  scale_x: f64,
  scale_y: f64,
}

impl ArtPageCoordinates {
  fn new(width_pt: f32, height_pt: f32) -> Self {
    // Word lays out the art viewports on the printer coordinate grid, but
    // fixed-format playback uses the inclusive metafile canvas. Native
    // Page.EnhMetaFileBits + GDI+ GetMetafileHeader readbacks distinguish
    // 4961x7016 coordinate extents from a 4962x7017 canvas (and 3333x4167
    // from 3334x4168). Keep the extra endpoint out of cell partitioning;
    // apply it only when mapping art geometry to the fixed-format page.
    let scale = |extent: f32| {
      let dots = (f64::from(extent) / DOT).round();
      if dots.is_finite() && dots > 0.0 {
        (dots + 1.0) / dots
      } else {
        1.0
      }
    };
    Self {
      scale_x: scale(width_pt),
      scale_y: scale(height_pt),
    }
  }

  fn point(self, x: f64, y: f64, outlined: bool) -> crate::common::Point {
    let offset = if outlined { DOT / 2.0 } else { 0.0 };
    let snap = |value: f64| ((value * 5.0).round() * DOT / 5.0 + offset) as f32;
    common_point(snap(x * self.scale_x), snap(y * self.scale_y))
  }
}

// ECMA-376 ST_Border/mapleMuffins identifies the built-in motif. Its native
// EMF records give a 990 x 960 logical window and these polygon/polyline
// coordinates. These are reusable artwork coordinates, not page positions.
// The two-color motif ignores CT_Border/@color (including explicit red).
const TOP: &[(i16, i16)] = &[
  (750, 480),
  (825, 510),
  (900, 480),
  (915, 360),
  (915, 240),
  (765, 120),
  (420, 75),
  (165, 90),
  (60, 210),
  (45, 345),
  (45, 480),
  (135, 540),
];
const CUP: &[(i16, i16)] = &[
  (195, 420),
  (255, 465),
  (315, 435),
  (360, 480),
  (420, 435),
  (480, 480),
  (540, 435),
  (630, 495),
  (660, 450),
  (720, 495),
  (750, 420),
  (690, 900),
  (600, 915),
  (330, 915),
  (255, 885),
  (255, 885),
  (180, 465),
];
const CREASES: [[(i16, i16); 2]; 5] = [
  [(255, 480), (330, 885)],
  [(360, 510), (420, 900)],
  [(465, 510), (480, 840)],
  [(585, 525), (555, 870)],
  [(705, 510), (615, 870)],
];
const PATCHES: [&[(i16, i16)]; 2] = [
  &[(180, 263), (290, 150), (495, 150), (317, 220), (235, 375)],
  &[(525, 271), (662, 195), (885, 315), (696, 271)],
];
const CONTOURS: [&[(i16, i16)]; 2] = [
  &[(630, 105), (299, 177), (225, 285)],
  &[(900, 360), (795, 270), (480, 255), (435, 300)],
];

fn paint_muffin(
  page: &mut Page,
  coordinates: ArtPageCoordinates,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
) {
  let orange = Color {
    r: 255,
    g: 128,
    b: 0,
    a: 255,
  };
  let brown = Color {
    r: 191,
    g: 64,
    b: 0,
    a: 255,
  };
  let mut path = |points: &[(i16, i16)], fill: Fill<'static>, cap: Option<StrokeCap>| {
    // Native fixed-format art paths resolve to fifths of a printer dot. A
    // cosmetic pen is centered on the printer pixel; unoutlined fills keep
    // the unshifted edge coordinates. Preserve separate fill/stroke paths.
    let points: Vec<_> = points
      .iter()
      .map(|&(px, py)| {
        coordinates.point(
          f64::from(x) + f64::from(px) * f64::from(width) / 990.0,
          f64::from(y) + f64::from(py) * f64::from(height) / 960.0,
          cap.is_some(),
        )
      })
      .collect();
    let closed = !matches!(fill, Fill::None);
    let mut commands = Vec::with_capacity(points.len() + usize::from(closed));
    commands.push(PathCommand::MoveTo(points[0]));
    commands.extend(points[1..].iter().copied().map(PathCommand::LineTo));
    if closed {
      commands.push(PathCommand::Close);
    }
    let left = points.iter().map(|p| p.x.0).fold(f32::INFINITY, f32::min);
    let top = points.iter().map(|p| p.y.0).fold(f32::INFINITY, f32::min);
    let right = points
      .iter()
      .map(|p| p.x.0)
      .fold(f32::NEG_INFINITY, f32::max);
    let bottom = points
      .iter()
      .map(|p| p.y.0)
      .fold(f32::NEG_INFINITY, f32::max);
    page.items.push(PageItem::path(PathItem {
      bounds: common_rect(left, top, right - left, bottom - top),
      points,
      commands,
      closed,
      fill,
      stroke: cap.map(|cap| Stroke {
        width: Pt(0.14),
        color: Color {
          r: 0,
          g: 0,
          b: 0,
          a: 255,
        },
        cap: Some(cap),
        join: Some(StrokeJoin::Round),
        ..Stroke::default()
      }),
    }));
  };
  path(TOP, Fill::Solid(orange), Some(StrokeCap::Flat));
  path(CUP, Fill::Solid(brown), Some(StrokeCap::Flat));
  for crease in CREASES {
    path(&crease, Fill::None, Some(StrokeCap::Square));
  }
  for patch in PATCHES {
    path(patch, Fill::Solid(brown), None);
  }
  for contour in CONTOURS {
    path(contour, Fill::None, Some(StrokeCap::Flat));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_fixed_format_art_coordinates_include_the_canvas_endpoint() {
    // Native PDF first contour point, compared with the matching EMF
    // viewport: two page sizes, corner/interior cells, and two art widths.
    // One 3000-DPI output tick plus PDF decimal serialization is allowed.
    for (page, origin, extent, expected) in [
      (
        (595.3, 841.9),
        (200.0, 6716.0),
        (100.0, 100.0),
        (33.156, 812.1),
      ),
      (
        (595.3, 841.9),
        (4661.0, 200.0),
        (100.0, 100.0),
        (568.57, 30.06),
      ),
      (
        (595.3, 841.9),
        (200.0, 6614.0),
        (100.0, 102.0),
        (33.156, 799.98),
      ),
      (
        (400.0, 500.0),
        (200.0, 3867.0),
        (100.0, 100.0),
        (33.156, 470.22),
      ),
      (
        (400.0, 500.0),
        (200.0, 3765.0),
        (100.0, 102.0),
        (33.156, 458.1),
      ),
      (
        (595.3, 841.9),
        (200.0, 6616.0),
        (200.0, 200.0),
        (42.252, 806.1),
      ),
    ] {
      let point = ArtPageCoordinates::new(page.0, page.1).point(
        origin.0 + 750.0 * extent.0 / 990.0,
        origin.1 + 480.0 * extent.1 / 960.0,
        true,
      );
      assert!((point.x.0 - expected.0).abs() < 0.03);
      assert!((point.y.0 - expected.1).abs() < 0.03);
    }
  }

  #[test]
  fn native_art_edge_partitions_keep_corners_and_assign_remainders_at_end() {
    // Office EMF readbacks: two paper sizes, three widths, 24pt page inset.
    // (full span, corner, total cells, small interior width, small count).
    for (span, corner, count, small, small_count) in [
      (4561, 100, 45, 101, 25),
      (6616, 100, 65, 101, 10),
      (4561, 200, 22, 208, 19),
      (6616, 200, 32, 207, 24),
      (4561, 50, 89, 51, 63),
      (2933, 100, 29, 101, 21),
      (3767, 100, 37, 101, 3),
    ] {
      let cells = edge_cells(200, 200 + span, corner);
      assert_eq!(cells.len(), count);
      assert_eq!(cells[0], (200, corner));
      assert_eq!(cells[count - 1], (200 + span - corner, corner));
      assert_eq!(
        cells[1..count - 1].iter().filter(|c| c.1 == small).count(),
        small_count
      );
      for pair in cells.windows(2) {
        assert_eq!(pair[0].0 + pair[0].1, pair[1].0);
      }
      assert!(cells[1..count - 1].windows(2).all(|p| p[0].1 <= p[1].1));
    }
  }
}
