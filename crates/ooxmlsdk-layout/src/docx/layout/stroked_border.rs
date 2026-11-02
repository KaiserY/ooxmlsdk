//! The two-dimensional dashDotStroked brush (MS-DOC BrcType 0x17).
use super::*;

const PERIOD: i64 = 48;
const THICKNESS: i64 = 25;
const CONTENT_TYPE: &str = "application/vnd.ooxmlsdk.wordprocessing-stroked-border+png";

fn bitmap_bounds(page: &Page, rectangle: (i64, i64, i64, i64)) -> (f32, f32, f32, f32) {
  let (left, top, right, bottom) = rectangle;
  wordprocessing_table_device::page_coordinates(page.setup.width_pt, page.setup.height_pt)
    .map_or_else(
      || word_table_device_rectangle(page, rectangle),
      |device| device.map_device_bitmap_rectangle((left, top, right - left, bottom - top)),
    )
}

fn coverage(axis: i64, cross: i64) -> u8 {
  // Native 600-DPI Word EMR_BITBLT sources contain cosmetic diagonal lines
  // six dots apart, in groups of five and one. The final twelve-dot gap
  // completes the 48-dot period. Preserve this source grid until PDF output
  // selects Screen/Print sampling; a PDF dash array cannot express it.
  if matches!(
    (axis + cross + 6).rem_euclid(PERIOD),
    0 | 6 | 12 | 18 | 24 | 36
  ) {
    u8::MAX
  } else {
    0
  }
}

fn tile(rectangle: (i64, i64, i64, i64), horizontal: bool, color: RgbColor) -> Option<Bytes> {
  let (left, top, right, bottom) = rectangle;
  let axis = if horizontal { left } else { top - 1 };
  encode_word_table_border_bitmap(
    (right - left) as u32,
    (bottom - top) as u32,
    color,
    |x, y| {
      let (along, cross) = if horizontal { (x, y) } else { (y, x) };
      // The cosmetic pen occupies 24 rows/columns in its 25-dot allocation.
      // Native page EMFs expose that transparent final edge; PDF controls
      // independently retain the vertical producer's preceding-dot phase.
      if i64::from(cross) < THICKNESS - 1 {
        coverage(axis + i64::from(along), i64::from(cross))
      } else {
        0
      }
    },
  )
}

fn strip(page: &mut Page, start: i64, end: i64, cross: i64, horizontal: bool, color: RgbColor) {
  let mut start = start;
  let mut full_tile = None;
  while start < end {
    let extent = (end - start).min(PERIOD);
    let rectangle = if horizontal {
      (start, cross, start + extent, cross + THICKNESS)
    } else {
      (cross, start, cross + THICKNESS, start + extent)
    };
    // Encode one shared period and at most one remainder, independently of
    // the rule length. PDF image deduplication can then reuse their samples.
    let data = if extent == PERIOD && full_tile.is_some() {
      full_tile.clone()
    } else {
      let data = tile(rectangle, horizontal, color);
      if extent == PERIOD {
        full_tile = data.clone();
      }
      data
    };
    if let Some(data) = data {
      let bounds = bitmap_bounds(page, rectangle);
      push_word_border_bitmap(page, bounds, data, CONTENT_TYPE);
    }
    start += extent;
  }
}

pub(super) fn paint_horizontal(
  page: &mut Page,
  bounds: (f32, f32, f32),
  grid_origin: f32,
  border: BorderStyle,
  edge: HorizontalTableBorderEdge,
  junctions: (HorizontalTableBorderJunction, HorizontalTableBorderJunction),
) {
  let (left, y, right) = bounds;
  let cross = wordprocessing_table_device::source_coordinate(y - 1.5);
  let grid_x = |x: f32| {
    wordprocessing_table_device::source_coordinate(grid_origin)
      + wordprocessing_table_device::source_coordinate_precise(
        ((f64::from(x) - f64::from(grid_origin)) * 20.0).round() / 20.0,
      )
  };
  let corner = |junction: HorizontalTableBorderJunction, x: f32| {
    stronger_border(junction.above, junction.below).map(|vertical| {
      let width = ((f64::from(vertical.width_pt) * 20.0).floor() as i64 * 5 / 12).max(1);
      (
        grid_x(x) - (width + 1) / 2,
        edge == HorizontalTableBorderEdge::Lower && junction.above == Some(vertical),
      )
    })
  };
  let leading = corner(junctions.0, left);
  let trailing = corner(junctions.1, right);
  let start = leading.map_or_else(
    || {
      grid_x(left)
        - if edge == HorizontalTableBorderEdge::Lower {
          6
        } else {
          0
        }
    },
    |(start, _)| start + THICKNESS,
  );
  let end = trailing.map_or_else(|| grid_x(right), |(start, _)| start);
  // Native edge-subset controls separate the junction squares from the
  // repeated horizontal brush. The upper junction belongs to the vertical
  // producer; the lower junction belongs to the horizontal producer.
  for (start, horizontal) in [leading, trailing].into_iter().flatten() {
    let rectangle = (start, cross, start + THICKNESS, cross + THICKNESS);
    if let Some(data) = tile(rectangle, horizontal, border.color) {
      let bounds = bitmap_bounds(page, rectangle);
      push_word_border_bitmap(page, bounds, data, CONTENT_TYPE);
    }
  }
  strip(page, start, end, cross, true, border.color);
}

pub(super) fn paint(
  page: &mut Page,
  (x1, y1, x2, y2): (f32, f32, f32, f32),
  border: BorderStyle,
) -> bool {
  if border.compound || border.dash_pattern != BorderDashPattern::DashDotStroked {
    return false;
  }
  let horizontal = (y2 - y1).abs() <= (x2 - x1).abs();
  let (start, end, cross) = if horizontal {
    (x1.min(x2), x1.max(x2), y1 - 1.5)
  } else {
    (y1.min(y2), y1.max(y2), x1 - 1.5)
  };
  strip(
    page,
    wordprocessing_table_device::source_coordinate(start),
    wordprocessing_table_device::source_coordinate(end),
    if horizontal {
      wordprocessing_table_device::source_coordinate(cross)
    } else {
      // Quantize the grid and the odd-width pen independently; converting
      // the point-space half-width together moves a 25-dot pen by one dot.
      wordprocessing_table_device::source_coordinate(x1) - (THICKNESS + 1) / 2
    },
    horizontal,
    border.color,
  );
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_diagonal_strokes_keep_five_and_one_groups_and_page_phase() {
    // First row of the visible source region of native Page1's horizontal
    // tile at x=683 (600DPI). Independent Word EMR_BITBLT, not SDK output.
    let marked: Vec<_> = (0..48).filter(|x| coverage(683 + x, 0) != 0).collect();
    assert_eq!(marked, [1, 7, 19, 31, 37, 43]);
    for axis in -48..96 {
      for cross in 0..25 {
        assert_eq!(coverage(axis, cross), coverage(axis + 48, cross));
        assert_eq!(coverage(axis + 1, cross), coverage(axis, cross + 1));
      }
    }
  }
}
