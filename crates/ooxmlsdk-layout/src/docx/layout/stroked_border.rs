//! The two-dimensional dashDotStroked brush (MS-DOC BrcType 0x17).
use super::*;

const PERIOD: i64 = 48;
const THICKNESS: i64 = 25;
const CONTENT_TYPE: &str = "application/vnd.ooxmlsdk.wordprocessing-stroked-border+png";

#[derive(Clone, Copy)]
struct Junction {
  start: i64,
  width: i64,
  vertical: BorderStyle,
  horizontal: bool,
  internal: bool,
  continuous: bool,
}

impl Junction {
  fn leading_extent(self) -> i64 {
    if !self.internal {
      THICKNESS
    } else if self.continuous {
      self.width
    } else {
      self.width + THICKNESS
    }
  }
}

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

fn square(page: &mut Page, start: i64, cross: i64, horizontal: bool, color: RgbColor) {
  let rectangle = (start, cross, start + THICKNESS, cross + THICKNESS);
  if let Some(data) = tile(rectangle, horizontal, color) {
    let bounds = bitmap_bounds(page, rectangle);
    push_word_border_bitmap(page, bounds, data, CONTENT_TYPE);
  }
}

fn fill(page: &mut Page, bounds: (i64, i64, i64, i64), color: RgbColor) {
  let (x_pt, y_pt, width_pt, height_pt) = word_table_device_rectangle(page, bounds);
  page.items.push(PageItem::Fill(FillItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    paint: ShadingPaint::Solid(color),
  }));
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
      let painted =
        word_table_compound_rails(vertical).map_or(width, |bands| bands.into_iter().sum());
      let continuous = junction.above.is_some() && junction.above == junction.below;
      let horizontal = !continuous
        && (vertical.dash_pattern != BorderDashPattern::DashDotStroked
          || (edge == HorizontalTableBorderEdge::Lower && junction.above == Some(vertical)));
      Junction {
        start: grid_x(x) - (width + 1) / 2,
        width: painted,
        vertical,
        horizontal,
        internal: !junction.is_horizontal_outer,
        continuous,
      }
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
    |junction| junction.start + junction.leading_extent(),
  );
  let end = trailing.map_or_else(|| grid_x(right), |junction| junction.start);
  // Native edge-subset controls separate the junction squares from the
  // repeated horizontal brush. The upper junction belongs to the vertical
  // producer; the lower junction belongs to the horizontal producer.
  for (is_leading, junction) in [(true, leading), (false, trailing)] {
    let Some(junction) = junction else {
      continue;
    };
    let Junction {
      start,
      width,
      vertical,
      horizontal,
      internal,
      continuous,
    } = junction;
    let stroked = vertical.dash_pattern == BorderDashPattern::DashDotStroked;
    // A continuous perpendicular keeps its own face across the junction.
    // Native both/above/below controls distinguish this from a one-sided
    // branch, which inserts an additional horizontal square and restarts.
    if internal && continuous {
      if stroked {
        square(page, start, cross, false, border.color);
      } else if let Some(rails) = word_table_compound_rails(vertical) {
        push_word_table_compound_device_bands(
          page,
          (start, cross, cross + THICKNESS),
          false,
          vertical,
          rails,
        );
      } else {
        fill(
          page,
          (start, cross, start + width, cross + THICKNESS),
          vertical.color,
        );
      }
      continue;
    }
    // An internal junction reserves the perpendicular rule before inserting
    // the leading horizontal square. Three-D rails can paint one dot less
    // than their layout width. Native mixed-width/type controls distinguish
    // the reserved strip, the square and the following period restart.
    if internal && is_leading {
      if stroked {
        if vertical.color == border.color {
          // The perpendicular end cap is independent of the following
          // horizontal square. Omitting it loses visible diagonal ink even
          // when that following square happens to sample fully transparent.
          square(page, start, cross, horizontal, border.color);
        } else {
          // Native color opposites paint both opaque faces before the
          // horizontal brush. Preserve their order and device rectangles.
          let bounds = (start, cross, start + width, cross + THICKNESS);
          fill(page, bounds, vertical.color);
          fill(page, bounds, border.color);
        }
      } else {
        strip(page, start, start + width, cross, true, border.color);
      }
    }
    let square = start + if internal && is_leading { width } else { 0 };
    self::square(page, square, cross, horizontal, border.color);
    if internal && !is_leading && horizontal && width > THICKNESS {
      strip(
        page,
        square + THICKNESS,
        start + width,
        cross,
        true,
        border.color,
      );
    }
  }
  strip(page, start, end, cross, true, border.color);
}

pub(super) fn paint_vertical_prefix(
  page: &mut Page,
  x: f32,
  start: i64,
  end: i64,
  color: RgbColor,
) {
  strip(
    page,
    start,
    end,
    wordprocessing_table_device::source_coordinate(x) - (THICKNESS + 1) / 2,
    false,
    color,
  );
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
  fn native_stroked_branches_keep_the_end_cap_before_the_extra_square() {
    let horizontal = crate::docx::table_border_style(
      w::BorderValues::DashDotStroked,
      Some(24),
      None,
      Some("000000"),
      None,
    )
    .unwrap();
    // Translate the native source-grid junction to400 dots. First-cell
    // layout origins differ with the outside pen; this test owns the three
    // junction components, independently of that containing-frame policy.
    for above in [true, false] {
      for color in [
        horizontal.color,
        RgbColor {
          r: 51,
          g: 204,
          b: 204,
        },
      ] {
        let vertical = BorderStyle {
          color,
          ..horizontal
        };
        let mut page = empty_page(
          PageSetup {
            width_pt: 419.75,
            height_pt: 297.35,
            ..Default::default()
          },
          0,
        );
        paint_horizontal(
          &mut page,
          (48.0, 103.67, 109.8),
          48.0,
          horizontal,
          HorizontalTableBorderEdge::Lower,
          (
            HorizontalTableBorderJunction::internal(
              above.then_some(vertical),
              (!above).then_some(vertical),
            ),
            HorizontalTableBorderJunction::internal(None, None),
          ),
        );
        if color == horizontal.color {
          let PageItem::Image(cap) = &page.items[0] else {
            panic!("sampled native end cap")
          };
          assert!((cap.x_pt - 46.464).abs() < 0.01);
          assert!((cap.width_pt - 3.0).abs() < 0.01);
        } else {
          for (item, color) in page.items[..2]
            .iter()
            .zip([vertical.color, horizontal.color])
          {
            let PageItem::Fill(cap) = item else {
              panic!("native color faces precede the brush")
            };
            assert!((cap.x_pt - 46.464).abs() < 0.01);
            assert!((cap.width_pt - 3.0).abs() < 0.01);
            assert_eq!(cap.paint, ShadingPaint::Solid(color));
          }
        }
        let images = page
          .items
          .iter()
          .filter_map(|item| match item {
            PageItem::Image(image) => Some(image),
            _ => None,
          })
          .collect::<Vec<_>>();
        let square = images
          .iter()
          .find(|image| (image.x_pt - 49.464).abs() < 0.01)
          .unwrap();
        assert!((square.width_pt - 3.0).abs() < 0.01);
        let first_period = images
          .iter()
          .find(|image| (image.width_pt - 5.76).abs() < 0.01)
          .unwrap();
        assert!((first_period.x_pt - 52.464).abs() < 0.01);
      }
    }
  }

  #[test]
  fn native_continuous_perpendiculars_own_the_face_without_an_extra_square() {
    let horizontal = crate::docx::table_border_style(
      w::BorderValues::DashDotStroked,
      Some(24),
      None,
      Some("000000"),
      None,
    )
    .unwrap();
    for (kind, size, first_period_x, face_count) in [
      (w::BorderValues::ThreeDEngrave, 6, 49.104, 3),
      (w::BorderValues::ThreeDEngrave, 24, 50.904, 3),
      (w::BorderValues::ThreeDEngrave, 48, 52.344, 3),
      (w::BorderValues::Single, 48, 51.024, 1),
      (w::BorderValues::Double, 10, 49.704, 2),
      (w::BorderValues::DashDotStroked, 24, 49.464, 0),
    ] {
      let vertical =
        crate::docx::table_border_style(kind, Some(size), None, Some("33CCCC"), None).unwrap();
      let mut page = empty_page(
        PageSetup {
          width_pt: 419.75,
          height_pt: 297.35,
          ..Default::default()
        },
        0,
      );
      paint_horizontal(
        &mut page,
        (48.0, 103.67, 109.8),
        48.0,
        horizontal,
        HorizontalTableBorderEdge::Lower,
        (
          HorizontalTableBorderJunction::internal(Some(vertical), Some(vertical)),
          HorizontalTableBorderJunction::internal(None, None),
        ),
      );
      assert_eq!(
        page
          .items
          .iter()
          .filter(|item| matches!(item, PageItem::Fill(_)))
          .count(),
        face_count
      );
      let first = page
        .items
        .iter()
        .find_map(|item| match item {
          PageItem::Image(image) if (image.width_pt - 5.76).abs() < 0.01 => Some(image),
          _ => None,
        })
        .unwrap();
      assert!(
        (first.x_pt - first_period_x).abs() < 0.01,
        "{} vs {first_period_x}",
        first.x_pt
      );
    }
  }

  #[test]
  fn native_vertical_join_keeps_its_one_dot_prefix_before_the_reserved_brush() {
    let mut page = empty_page(
      PageSetup {
        width_pt: 419.75,
        height_pt: 297.35,
        ..Default::default()
      },
      0,
    );
    paint_vertical_prefix(&mut page, 109.8, 546, 547, RgbColor { r: 0, g: 0, b: 0 });
    let [PageItem::Image(image)] = page.items.as_slice() else {
      panic!("one native prefix")
    };
    for (actual, expected) in [image.x_pt, image.y_pt, image.width_pt, image.height_pt]
      .into_iter()
      .zip([108.26, 65.539989, 3.0, 0.12])
    {
      assert!((actual - expected).abs() < 0.01, "{actual} vs {expected}");
    }
  }

  #[test]
  fn native_shared_mixed_junctions_reserve_the_painted_perpendicular_extent() {
    let border = |kind, size, color| {
      crate::docx::table_border_style(kind, Some(size), None, Some(color), None).unwrap()
    };
    let horizontal = border(w::BorderValues::DashDotStroked, 24, "000000");
    let engraved = border(w::BorderValues::ThreeDEngrave, 48, "33CCCC");
    let double = border(w::BorderValues::Double, 10, "008000");
    let mut page = empty_page(
      PageSetup {
        width_pt: 419.75,
        height_pt: 297.35,
        ..Default::default()
      },
      0,
    );
    paint_horizontal(
      &mut page,
      (51.3, 92.64189, 109.8),
      51.3,
      horizontal,
      HorizontalTableBorderEdge::Lower,
      (
        HorizontalTableBorderJunction::internal(Some(engraved), None),
        HorizontalTableBorderJunction::internal(Some(horizontal), Some(double)),
      ),
    );
    let mut actual = page
      .items
      .iter()
      .map(|item| {
        let PageItem::Image(image) = item else {
          panic!("native bitmap strip")
        };
        (image.x_pt, image.width_pt)
      })
      .collect::<Vec<_>>();
    actual.sort_by(|a, b| a.0.total_cmp(&b.0));
    // Actual saved/native independent shared-border rectangles: a74-dot
    // leading face,25-dot square,48-dot periods,27-dot tail, then25+5dots.
    let mut expected = vec![(46.824, 5.76), (52.584, 3.12), (55.704, 3.0)];
    expected.extend((0..8).map(|i| (58.704 + i as f32 * 5.76, 5.76)));
    expected.extend([(104.784, 3.24), (108.024, 3.0), (111.024, 0.6)]);
    assert_eq!(actual.len(), expected.len());
    for ((x, width), (expected_x, expected_width)) in actual.into_iter().zip(expected) {
      assert!((x - expected_x).abs() < 0.01, "{x} vs {expected_x}");
      assert!(
        (width - expected_width).abs() < 0.01,
        "{width} vs {expected_width}"
      );
    }
  }

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
