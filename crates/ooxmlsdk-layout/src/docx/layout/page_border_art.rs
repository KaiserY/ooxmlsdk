//! Built-in Word page-border artwork, independent of document content.

use super::{Page, PageItem, WORD_FIXED_OUTPUT_DPI};
use crate::common::{Color, Fill, PathCommand, PathItem, Pt, Stroke, StrokeCap, StrokeJoin};
use crate::docx::CellBordersModel;
use crate::model::{common_point, common_rect};
use crate::units;

mod cake_slice;
mod gems;
mod holly;
mod people_hats;
mod weaving_angles;

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
  distribute_edge_cells(start, end, corner, count)
}

fn distribute_edge_cells(start: i32, end: i32, corner: i32, count: i32) -> Vec<(i32, i32)> {
  let span = end - start;
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
  reference: (f32, f32, f32, f32),
  borders: CellBordersModel,
) {
  paint_motif(
    page,
    reference,
    borders,
    edge_cells,
    |page, coordinates, tile| {
      paint_muffin(page, coordinates, tile.x, tile.y, tile.width, tile.height);
    },
  );
}

pub(super) fn paint_holly(
  page: &mut Page,
  reference: (f32, f32, f32, f32),
  borders: CellBordersModel,
) {
  paint_motif(
    page,
    reference,
    borders,
    edge_cells,
    |page, coordinates, tile| {
      holly::paint(page, coordinates, tile.x, tile.y, tile.width, tile.height);
    },
  );
}

pub(super) fn paint_cake_slice(
  page: &mut Page,
  reference: (f32, f32, f32, f32),
  borders: CellBordersModel,
) {
  // ECMA-376 ST_Border gives one all-sides image. Native frames retain its
  // intrinsic palette and orientation, including explicitly colored slots.
  paint_motif(page, reference, borders, edge_cells, cake_slice::paint);
}

pub(super) fn paint_people_hats(
  page: &mut Page,
  reference: (f32, f32, f32, f32),
  borders: CellBordersModel,
) {
  paint_motif(page, reference, borders, edge_cells, people_hats::paint);
}

pub(super) fn paint_gems(
  page: &mut Page,
  reference: (f32, f32, f32, f32),
  borders: CellBordersModel,
) {
  // Like the resource identity, the foreground color comes from the final
  // authored slot. Native red/blue and Maple/nil opposites retain the gray
  // facets while replacing only the resource's black foreground.
  let Some(color) = foreground_color(borders) else {
    return;
  };
  paint_motif(
    page,
    reference,
    borders,
    edge_cells,
    |page, coordinates, tile| {
      gems::paint(page, coordinates, tile, color);
    },
  );
}

pub(super) fn paint_weaving_angles(
  page: &mut Page,
  reference: (f32, f32, f32, f32),
  borders: CellBordersModel,
) {
  let Some(color) = foreground_color(borders) else {
    return;
  };
  paint_motif(
    page,
    reference,
    borders,
    weaving_angles::edge_cells,
    |page, coordinates, tile| weaving_angles::paint(page, coordinates, tile, color),
  );
}

fn foreground_color(borders: CellBordersModel) -> Option<Color> {
  let border = [borders.right, borders.bottom, borders.left, borders.top]
    .into_iter()
    .flatten()
    .next()?;
  Some(Color {
    r: border.color.r,
    g: border.color.g,
    b: border.color.b,
    a: 255,
  })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArtCell {
  TopLeft,
  Top,
  TopRight,
  Right,
  BottomLeft,
  Bottom,
  BottomRight,
  Left,
}

#[derive(Clone, Copy)]
struct ArtTile {
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  cell: ArtCell,
}

fn square_printer_box(rect: [i16; 4], tile: ArtTile, window: f64) -> [f64; 4] {
  let [left, top, right, bottom] = rect;
  let axis = |value: i16, origin: i32, extent: i32| {
    let device = f64::from(origin) + f64::from(value) * f64::from(extent) / window;
    // Classic GDI retains POINTFIX's four fractional bits. Rectangle and
    // Ellipse exclude the last device column/row in compatible graphics mode.
    (device * 16.0).round() / 16.0
  };
  [
    axis(left, tile.x, tile.width),
    axis(top, tile.y, tile.height),
    axis(right, tile.x, tile.width) - 1.0,
    axis(bottom, tile.y, tile.height) - 1.0,
  ]
}

fn paint_motif(
  page: &mut Page,
  (left, top, right, bottom): (f32, f32, f32, f32),
  borders: CellBordersModel,
  partition: fn(i32, i32, i32) -> Vec<(i32, i32)>,
  mut paint: impl FnMut(&mut Page, ArtPageCoordinates, ArtTile),
) {
  let Some(width_pt) = [borders.top, borders.left, borders.bottom, borders.right]
    .into_iter()
    .flatten()
    .map(|border| border.width_pt)
    .find(|width| *width > 0.0)
  else {
    return;
  };
  let spacing = |border: Option<crate::docx::BorderStyle>| border.map_or(0.0, |b| b.spacing_pt);
  let outset =
    |border: Option<crate::docx::BorderStyle>| border.map_or(0.0, |b| b.spacing_pt + b.width_pt);
  let (left, top, right, bottom) = if page.setup.borders_offset_from_text {
    (
      left - outset(borders.left),
      top - outset(borders.top),
      right + outset(borders.right),
      bottom + outset(borders.bottom),
    )
  } else {
    (
      left + spacing(borders.left),
      top + spacing(borders.top),
      right - spacing(borders.right),
      bottom - spacing(borders.bottom),
    )
  };
  if ![left, top, right, bottom, width_pt]
    .iter()
    .all(|v| v.is_finite())
  {
    return;
  }
  let dot = |value: f32| (f64::from(value) / DOT).round() as i32;
  let size = dot(width_pt);
  let columns = partition(dot(left), dot(right), size);
  let rows = partition(dot(top), dot(bottom), size);
  if columns.is_empty() || rows.is_empty() {
    return;
  }
  let (x_first, _) = columns[0];
  let (x_last, _) = columns[columns.len() - 1];
  let (y_first, _) = rows[0];
  let (y_last, _) = rows[rows.len() - 1];
  let coordinates = ArtPageCoordinates::new(page.setup.width_pt, page.setup.height_pt);
  let mut paint_tile = |page: &mut Page, x, y, width, height| {
    let cell = if y == y_first {
      if x == x_first {
        ArtCell::TopLeft
      } else if x == x_last {
        ArtCell::TopRight
      } else {
        ArtCell::Top
      }
    } else if y == y_last {
      if x == x_first {
        ArtCell::BottomLeft
      } else if x == x_last {
        ArtCell::BottomRight
      } else {
        ArtCell::Bottom
      }
    } else if x == x_first {
      ArtCell::Left
    } else {
      ArtCell::Right
    };
    paint(
      page,
      coordinates,
      ArtTile {
        x,
        y,
        width,
        height,
        cell,
      },
    );
  };
  // ST_Border defines eight directional images. A corner is shared by the
  // two adjoining edges; individual motifs can reuse the same resource.
  for (x, width) in columns {
    if borders.top.is_some() {
      paint_tile(page, x, y_first, width, size);
    }
    if borders.bottom.is_some() {
      paint_tile(page, x, y_last, width, size);
    }
  }
  let start = usize::from(borders.top.is_some());
  let end = rows.len() - usize::from(borders.bottom.is_some());
  for &(y, height) in &rows[start..end] {
    if borders.left.is_some() {
      paint_tile(page, x_first, y, size, height);
    }
    if borders.right.is_some() {
      paint_tile(page, x_last, y, size, height);
    }
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
  fn weaving_angles_native_bottom_corners_face_the_adjoining_edge() {
    // Actual single-left/single-right EMFs select different bottom corner
    // resources. The two vertical contours must turn into the adjoining
    // bottom edge, including when that bottom border itself is absent.
    for left in [true, false] {
      let border = crate::docx::BorderStyle {
        width_pt: 16.0,
        spacing_pt: 24.0,
        ..Default::default()
      };
      let setup = super::super::PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        borders_offset_from_text: false,
        ..Default::default()
      };
      let mut page = super::super::empty_section_page(setup, 0, 0);
      paint_weaving_angles(
        &mut page,
        (0.0, 0.0, 612.0, 792.0),
        CellBordersModel {
          left: left.then_some(border),
          right: (!left).then_some(border),
          ..Default::default()
        },
      );
      let Some(PageItem::CompositingGroup { items, .. }) = page.items.last() else {
        panic!("bottom corner artwork")
      };
      let contours: Vec<_> = items
        .iter()
        .filter_map(|item| match item {
          PageItem::Path(path) if matches!(path.fill, Fill::None) && path.points.len() == 3 => {
            Some(path)
          }
          _ => None,
        })
        .collect();
      assert_eq!(contours.len(), 2);
      for path in contours {
        let p = &path.points;
        assert_eq!(p[0].x, p[1].x);
        assert!(p[1].y.0 > p[0].y.0);
        assert_eq!(p[1].y, p[2].y);
        assert_eq!(p[2].x.0 > p[1].x.0, left);
      }
    }
  }

  #[test]
  fn weaving_angles_native_frames_keep_eight_directions_and_last_slot_ink() {
    for (width, cells, primitives) in [(8.0, 116, 822), (16.0, 60, 430), (24.0, 38, 276)] {
      let red = crate::docx::BorderStyle {
        width_pt: width,
        spacing_pt: 24.0,
        color: crate::docx::RgbColor { r: 255, g: 0, b: 0 },
        ..Default::default()
      };
      let blue = crate::docx::BorderStyle {
        color: crate::docx::RgbColor { r: 0, g: 0, b: 255 },
        ..red
      };
      let setup = super::super::PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        borders_offset_from_text: false,
        ..Default::default()
      };
      let mut page = super::super::empty_section_page(setup, 0, 0);
      paint_weaving_angles(
        &mut page,
        (0.0, 0.0, 612.0, 792.0),
        CellBordersModel {
          top: Some(red),
          left: Some(red),
          bottom: Some(red),
          right: Some(blue),
        },
      );
      assert_eq!(page.items.len(), cells);
      let mut paths = 0;
      let mut square_caps = 0;
      let mut white_fills = 0;
      for group in &page.items {
        let PageItem::CompositingGroup { items, clip, .. } = group else {
          panic!("native art cell")
        };
        assert!(clip.is_some());
        paths += items.len();
        for item in items {
          let PageItem::Path(path) = item else {
            panic!("native vector artwork")
          };
          let stroke = path.stroke.as_ref().unwrap();
          assert_eq!(stroke.width.0, 0.14);
          square_caps += usize::from(stroke.cap == Some(StrokeCap::Square));
          for color in [
            Some(stroke.color),
            if let Fill::Solid(c) = path.fill {
              Some(c)
            } else {
              None
            },
          ]
          .into_iter()
          .flatten()
          {
            assert!(
              color
                == Color {
                  r: 0,
                  g: 0,
                  b: 255,
                  a: 255
                }
                || color
                  == Color {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 255
                  }
            );
          }
          white_fills += usize::from(matches!(
            path.fill,
            Fill::Solid(Color {
              r: 255,
              g: 255,
              b: 255,
              ..
            })
          ));
        }
      }
      assert_eq!(paths, primitives);
      assert_eq!(square_caps, 4);
      assert_eq!(white_fills, 2 * cells);
    }
  }

  #[test]
  fn gems_native_frame_keeps_distinct_corners_and_last_slot_ink() {
    let red = crate::docx::BorderStyle {
      width_pt: 16.0,
      spacing_pt: 24.0,
      color: crate::docx::RgbColor { r: 255, g: 0, b: 0 },
      ..Default::default()
    };
    let blue = crate::docx::BorderStyle {
      color: crate::docx::RgbColor { r: 0, g: 0, b: 255 },
      ..red
    };
    let setup = super::super::PageSetup {
      width_pt: 612.0,
      height_pt: 792.0,
      borders_offset_from_text: false,
      ..Default::default()
    };
    let mut page = super::super::empty_section_page(setup, 0, 0);
    paint_gems(
      &mut page,
      (0.0, 0.0, 612.0, 792.0),
      CellBordersModel {
        top: Some(red),
        left: Some(red),
        bottom: Some(red),
        right: Some(blue),
      },
    );
    // Actual16pt complete-frame EMF and PDF counts. The other28controls
    // separately pin corner positions, palette and independent dimensions.
    assert_eq!(page.items.len(), 158);
    let mut primitives = 0;
    let mut strokes = 0;
    let mut corners = 0;
    for tile in &page.items {
      let PageItem::CompositingGroup { items, clip, .. } = tile else {
        panic!("native cell clip")
      };
      assert!(clip.is_some());
      corners += usize::from(items.len() == 19);
      primitives += items.len();
      for item in items {
        let PageItem::Path(path) = item else {
          panic!("native vector resource")
        };
        strokes += usize::from(path.stroke.is_some());
        if let Fill::Solid(color) = path.fill {
          assert!(
            color
              == Color {
                r: 0,
                g: 0,
                b: 255,
                a: 255
              }
              || (color.r == color.g && color.g == color.b && [128, 192, 255].contains(&color.r))
          );
        }
      }
    }
    assert_eq!(corners, 4);
    assert_eq!(primitives, 3618);
    assert_eq!(strokes, 628);
  }

  #[test]
  fn people_hats_art_keeps_native_boxes_limbs_palette_and_inside_hat_pens() {
    for (width, edges, count) in [
      (8.0, [true, false, false, false], 75),
      (8.0, [false, true, false, false], 97),
      (16.0, [true, true, true, true], 158),
    ] {
      let border = crate::docx::BorderStyle {
        width_pt: width,
        spacing_pt: 24.0,
        color: crate::docx::RgbColor { r: 255, g: 0, b: 0 },
        ..Default::default()
      };
      let borders = CellBordersModel {
        top: edges[0].then_some(border),
        right: edges[1].then_some(border),
        bottom: edges[2].then_some(border),
        left: edges[3].then_some(border),
      };
      let setup = super::super::PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        borders_offset_from_text: false,
        ..Default::default()
      };
      let mut page = super::super::empty_section_page(setup, 0, 0);
      paint_people_hats(&mut page, (0.0, 0.0, 612.0, 792.0), borders);
      assert_eq!(page.items.len(), count);
      for tile in &page.items {
        let PageItem::CompositingGroup { items, .. } = tile else {
          panic!("native PeopleHats cell")
        };
        assert_eq!(items.len(), 14);
        let mut inside = 0;
        let mut limbs = 0;
        let mut ellipses = 0;
        let mut fills = Vec::new();
        for item in items {
          let PageItem::Path(path) = item else {
            panic!("native PeopleHats primitive")
          };
          let stroke = path.stroke.as_ref().expect("all14native strokes");
          inside += usize::from(stroke.alignment == Some(crate::common::StrokeAlignment::Inside));
          if !path.closed {
            limbs += 1;
            assert_eq!(path.points.len(), 2);
            assert_eq!(stroke.cap, Some(StrokeCap::Square));
          }
          ellipses += usize::from(
            path
              .commands
              .iter()
              .any(|c| matches!(c, PathCommand::CubicTo { .. })),
          );
          if let Fill::Solid(color) = path.fill {
            fills.push([color.r, color.g, color.b]);
          }
        }
        assert_eq!((inside, limbs, ellipses), (2, 2, 5));
        assert_eq!(fills.iter().filter(|c| **c == [255, 255, 255]).count(), 1);
        assert_eq!(fills.iter().filter(|c| **c == [192, 192, 192]).count(), 7);
        assert_eq!(fills.iter().filter(|c| **c == [255, 255, 128]).count(), 1);
        assert_eq!(fills.iter().filter(|c| **c == [0, 0, 128]).count(), 3);
      }
    }
  }

  #[test]
  fn cake_slice_art_retains_native_five_layers_palette_and_shared_cells() {
    for (width, edges, count) in [
      (8.0, [true, false, false, false], 75),
      (8.0, [false, true, false, false], 97),
      (16.0, [false, false, true, false], 38),
      (16.0, [true, true, true, true], 158),
    ] {
      let border = crate::docx::BorderStyle {
        width_pt: width,
        spacing_pt: 24.0,
        color: crate::docx::RgbColor { r: 0, g: 0, b: 255 },
        ..Default::default()
      };
      let borders = CellBordersModel {
        top: edges[0].then_some(border),
        right: edges[1].then_some(border),
        bottom: edges[2].then_some(border),
        left: edges[3].then_some(border),
      };
      let setup = super::super::PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        borders_offset_from_text: false,
        ..Default::default()
      };
      let mut page = super::super::empty_section_page(setup, 0, 0);
      paint_cake_slice(&mut page, (0.0, 0.0, 612.0, 792.0), borders);
      assert_eq!(page.items.len(), count);
      for tile in &page.items {
        let PageItem::CompositingGroup { items, clip, .. } = tile else {
          panic!("native CakeSlice cell")
        };
        assert!(clip.is_some());
        assert_eq!(items.len(), 5);
        let expected = [
          (153, [0, 0, 0]),
          (151, [255, 238, 202]),
          (113, [255, 238, 202]),
          (113, [255, 153, 194]),
          (49, [255, 153, 194]),
        ];
        for (item, (point_count, rgb)) in items.iter().zip(expected) {
          let PageItem::Path(path) = item else {
            panic!("native CakeSlice polygon")
          };
          assert!(path.closed && path.stroke.is_none());
          assert_eq!(path.points.len(), point_count);
          let Fill::Solid(color) = path.fill else {
            panic!("native intrinsic fill")
          };
          assert_eq!([color.r, color.g, color.b], rgb);
        }
      }
    }
  }

  #[test]
  fn holly_art_repeats_native_cells_on_single_sides_and_shared_corners() {
    for (width, edges, count) in [
      (8.0, [true, false, false, false], 75),
      (8.0, [false, true, false, false], 97),
      (16.0, [false, false, true, false], 38),
      (16.0, [true, false, false, true], 82),
      (16.0, [true, true, true, true], 158),
    ] {
      let border = crate::docx::BorderStyle {
        width_pt: width,
        spacing_pt: 24.0,
        ..Default::default()
      };
      let borders = CellBordersModel {
        top: edges[0].then_some(border),
        right: edges[1].then_some(border),
        bottom: edges[2].then_some(border),
        left: edges[3].then_some(border),
      };
      let setup = super::super::PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        borders_offset_from_text: false,
        ..Default::default()
      };
      let mut page = super::super::empty_section_page(setup, 0, 0);
      paint_holly(&mut page, (0.0, 0.0, 612.0, 792.0), borders);
      assert_eq!(page.items.len(), count);
      for tile in &page.items {
        let PageItem::CompositingGroup { items, clip, .. } = tile else {
          panic!("native tile clip")
        };
        assert!(clip.is_some());
        assert_eq!(items.len(), 12);
        assert_eq!(
          items
            .iter()
            .filter(|p| matches!(p,PageItem::Path(p) if p.stroke.is_some()))
            .count(),
          3
        );
      }
    }
  }

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
