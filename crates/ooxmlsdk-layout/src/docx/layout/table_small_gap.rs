//! Small-gap table rails and their native rectangular corner connectors.
use super::*;

#[derive(Clone, Copy, Default)]
struct Bands {
  widths: [i64; 5],
  colors: [Option<RgbColor>; 5],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ConnectionKind {
  Corner,
  Tee,
  Crossing,
}

impl From<HorizontalTableBorderJunction> for ConnectionKind {
  fn from(junction: HorizontalTableBorderJunction) -> Self {
    if junction.is_horizontal_outer {
      Self::Corner
    } else if junction.has_horizontal_continuation {
      Self::Crossing
    } else {
      Self::Tee
    }
  }
}

pub(super) fn is_style(border: BorderStyle) -> bool {
  border.compound
    && border.dash_pattern == BorderDashPattern::Solid
    && matches!(
      border.compound_pattern,
      BorderCompoundPattern::ThinThickSmallGap
        | BorderCompoundPattern::ThickThinSmallGap
        | BorderCompoundPattern::ThinThickThinSmallGap
    )
}

impl Bands {
  fn from_border(border: Option<BorderStyle>, reverse: bool) -> Option<Self> {
    let Some(border) = border else {
      return Some(Self::default());
    };
    let mut bands = if is_style(border) {
      Self {
        widths: word_table_compound_rails(border)?,
        colors: [
          Some(border.color),
          None,
          Some(border.color),
          None,
          Some(border.color),
        ],
      }
    } else if !border.compound
      && border.relief.is_none()
      && border.dash_pattern == BorderDashPattern::Solid
    {
      let twips = (f64::from(border.width_pt) * 20.0).floor() as i64;
      Self {
        widths: [(twips * 5 / 12).max(1), 0, 0, 0, 0],
        colors: [Some(border.color), None, None, None, None],
      }
    } else {
      return None;
    };
    if reverse {
      bands.widths.reverse();
      bands.colors.reverse();
    }
    Some(bands)
  }

  fn extent(self) -> i64 {
    self.widths.into_iter().sum()
  }
}

pub(super) fn paint_horizontal(
  page: &mut Page,
  (left, y, right): (f32, f32, f32),
  border: BorderStyle,
  edge: HorizontalTableBorderEdge,
  (leading, trailing): (HorizontalTableBorderJunction, HorizontalTableBorderJunction),
) -> bool {
  if !is_style(border)
    || (border.compound_pattern != BorderCompoundPattern::ThinThickThinSmallGap
      && (!leading.is_horizontal_outer || !trailing.is_horizontal_outer))
  {
    return false;
  }
  // An internal row separator uses its near source edge even when emitted
  // as the preceding row's Lower border. Only the table's outside lower
  // frame anchors to the far edge; its truncated paint has another owner.
  let lower = matches!(edge, HorizontalTableBorderEdge::Lower)
    && leading.is_horizontal_outer
    && trailing.is_horizontal_outer;
  let left_border = stronger_border(leading.above, leading.below);
  let right_border = stronger_border(trailing.above, trailing.below);
  let Some(horizontal) = Bands::from_border(Some(border), lower) else {
    return false;
  };
  let (Some(vertical_left), Some(vertical_right)) = (
    Bands::from_border(left_border, false),
    Bands::from_border(right_border, true),
  ) else {
    return false;
  };
  let origin = |x: f32, perpendicular: Option<BorderStyle>| {
    wordprocessing_table_device::source_coordinate(
      x - perpendicular.map_or(0.0, |border| border.width_pt / 2.0),
    )
  };
  let outer_left = origin(left, left_border);
  let inner_left = outer_left + vertical_left.extent();
  let inner_right = origin(right, right_border);
  let outer_right = inner_right + vertical_right.extent();
  let top = wordprocessing_table_device::source_coordinate(y - border.width_pt / 2.0);
  // Both edges retain the layout interval independently of the truncated
  // component extent (native50vs49dots). The lower strip anchors its bands
  // to the far edge; its leading residual belongs to the perpendicular
  // connector. Reconstructing that far edge from the paint extent loses it.
  let bottom = wordprocessing_table_device::source_coordinate(y + border.width_pt / 2.0);
  horizontal_strip(
    page,
    horizontal,
    (inner_left, top, inner_right, bottom),
    lower,
  );
  connection(
    page,
    horizontal,
    vertical_left,
    (outer_left, top, inner_left, bottom),
    false,
    lower,
    ConnectionKind::from(leading),
  );
  connection(
    page,
    horizontal,
    vertical_right,
    (inner_right, top, outer_right, bottom),
    true,
    lower,
    ConnectionKind::from(trailing),
  );
  true
}

fn fill(page: &mut Page, bounds: (i64, i64, i64, i64), color: Option<RgbColor>) {
  let (left, top, right, bottom) = bounds;
  let Some(color) = color.filter(|_| right > left && bottom > top) else {
    return;
  };
  let (x_pt, y_pt, width_pt, height_pt) = word_table_device_rectangle(page, bounds);
  page.items.push(PageItem::Fill(FillItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    paint: ShadingPaint::Solid(color),
  }));
}

fn horizontal_strip(
  page: &mut Page,
  bands: Bands,
  (left, top, right, bottom): (i64, i64, i64, i64),
  lower: bool,
) {
  let mut offset = 0;
  for (width, color) in bands.widths.into_iter().zip(bands.colors) {
    let y = if lower {
      bottom - offset - width
    } else {
      top + offset
    };
    fill(page, (left, y, right, y + width), color);
    offset += width;
  }
}

fn vertical_strip(
  page: &mut Page,
  bands: Bands,
  (left, top, right, bottom): (i64, i64, i64, i64),
  trailing: bool,
) {
  let mut offset = 0;
  for (width, color) in bands.widths.into_iter().zip(bands.colors) {
    let x = if trailing {
      right - offset - width
    } else {
      left + offset
    };
    fill(page, (x, top, x + width, bottom), color);
    offset += width;
  }
}

fn corner(
  page: &mut Page,
  horizontal: Bands,
  vertical: Bands,
  bounds: (i64, i64, i64, i64),
  trailing: bool,
  lower: bool,
) {
  let (left, top, right, bottom) = bounds;
  if right <= left || bottom <= top {
    return;
  }
  if horizontal.extent() != vertical.extent() {
    // Native unequal extents give the painted corner to the horizontal
    // border. Only the row-inset residual retains its vertical bands.
    horizontal_strip(page, horizontal, bounds, lower);
    let residual = if lower {
      (left, top, right, bottom - horizontal.extent())
    } else {
      (left, top + horizontal.extent(), right, bottom)
    };
    vertical_strip(page, vertical, residual, trailing);
    return;
  }
  let (mut x_offset, mut y_offset) = (0, 0);
  for index in 0..horizontal.widths.len() {
    let x_width = vertical.widths[index];
    let y_width = horizontal.widths[index];
    let x = if trailing {
      right - x_offset - x_width
    } else {
      left + x_offset
    };
    let y = if lower {
      bottom - y_offset - y_width
    } else {
      top + y_offset
    };
    fill(
      page,
      (
        x,
        if lower { top } else { top + y_offset },
        x + x_width,
        if lower { bottom - y_offset } else { bottom },
      ),
      vertical.colors[index],
    );
    fill(
      page,
      (
        if trailing { left } else { left + x_offset },
        y,
        if trailing { right - x_offset } else { right },
        y + y_width,
      ),
      horizontal.colors[index],
    );
    x_offset += x_width;
    y_offset += y_width;
  }
}

fn connection(
  page: &mut Page,
  horizontal: Bands,
  vertical: Bands,
  bounds: (i64, i64, i64, i64),
  trailing: bool,
  lower: bool,
  kind: ConnectionKind,
) {
  if kind == ConnectionKind::Corner
    || horizontal.widths != vertical.widths
    || horizontal.widths[4] == 0
  {
    corner(page, horizontal, vertical, bounds, trailing, lower);
    return;
  }
  let (left, top, right, bottom) = bounds;
  let (paint_top, paint_bottom) = if lower {
    (bottom - horizontal.extent(), bottom)
  } else {
    (top, top + horizontal.extent())
  };
  // Native three-rail T joins retain the outer and middle vertical rails.
  // The two thin horizontal rails meet the innermost vertical rail; the
  // central horizontal rail meets the middle one. The horizontal border
  // owns the connector's ink, including when the adjoining row rails differ
  // in color. Do not substitute an outside L corner or a solid square.
  let mut offset = 0;
  for index in 0..horizontal.widths.len() {
    let width = horizontal.widths[index];
    if index % 2 == 0 {
      // A four-way crossing leaves the two thin vertical rails open in
      // the horizontal gaps. Paired facing caps supply only their four
      // corner patches; the thick middle rail continues through the join.
      if index == 2 || (kind == ConnectionKind::Tee && index == 0) {
        let x = if trailing {
          right - offset - width
        } else {
          left + offset
        };
        fill(
          page,
          (x, paint_top, x + width, paint_bottom),
          horizontal.colors[index],
        );
      }
      let target = if index == 2 { 2 } else { 4 };
      let inset = vertical.widths[..target].iter().sum::<i64>();
      let y = if lower {
        paint_bottom - offset - width
      } else {
        paint_top + offset
      };
      let (x1, x2) = if trailing {
        (left, right - inset)
      } else {
        (left + inset, right)
      };
      fill(page, (x1, y, x2, y + width), horizontal.colors[index]);
    }
    offset += width;
  }
  // Layout can retain a final source dot beyond the truncated paint bands.
  // That row-inset residual remains owned by the perpendicular border.
  let residual = if lower {
    (left, top, right, paint_top)
  } else {
    (left, paint_bottom, right, bottom)
  };
  vertical_strip(page, vertical, residual, trailing);
}

#[cfg(test)]
mod tests {
  use super::*;

  const RED: RgbColor = RgbColor { r: 255, g: 0, b: 0 };
  const BLUE: RgbColor = RgbColor { r: 0, g: 0, b: 255 };

  fn color_at(page: &Page, x: f32, y: f32) -> Option<RgbColor> {
    page.items.iter().rev().find_map(|item| match item {
      PageItem::Fill(fill)
        if x >= fill.x_pt
          && x < fill.x_pt + fill.width_pt
          && y >= fill.y_pt
          && y < fill.y_pt + fill.height_pt =>
      {
        match fill.paint {
          ShadingPaint::Solid(color) => Some(color),
          _ => None,
        }
      }
      _ => None,
    })
  }

  #[test]
  fn triple_small_gap_internal_t_join_matches_native_colored_bands() {
    let horizontal = BorderStyle {
      width_pt: 6.0,
      compound: true,
      compound_pattern: BorderCompoundPattern::ThinThickThinSmallGap,
      color: RED,
      ..Default::default()
    };
    let vertical = BorderStyle {
      color: BLUE,
      ..horizontal
    };
    let native = [
      [Some(RED), None, Some(RED), None, Some(RED)],
      [Some(RED), None, Some(RED), None, None],
      [Some(RED), None, Some(RED), Some(RED), Some(RED)],
      [Some(RED), None, Some(RED), None, None],
      [Some(RED), None, Some(RED), None, Some(RED)],
    ];
    for trailing in [false, true] {
      let mut page = empty_page(
        PageSetup {
          width_pt: 612.0,
          height_pt: 792.0,
          ..Default::default()
        },
        0,
      );
      connection(
        &mut page,
        Bands::from_border(Some(horizontal), false).unwrap(),
        Bands::from_border(Some(vertical), trailing).unwrap(),
        (530, 762, 579, 812),
        trailing,
        false,
        ConnectionKind::Tee,
      );
      // Actual Word red/blue control: left sourceX530, first internal row
      // sourceY762; 49 painted dots plus one blue row-inset residual.
      for (row, dy) in [3, 9, 24, 40, 46].into_iter().enumerate() {
        for (column, dx) in [3, 9, 24, 40, 46].into_iter().enumerate() {
          let (x, y, w, h) =
            word_table_device_rectangle(&page, (530 + dx, 762 + dy, 531 + dx, 763 + dy));
          assert_eq!(
            color_at(&page, x + w / 2.0, y + h / 2.0),
            native[row][if trailing { 4 - column } else { column }]
          );
        }
      }
      let (x, y, w, h) = word_table_device_rectangle(&page, (533, 811, 534, 812));
      assert_eq!(color_at(&page, x + w / 2.0, y + h / 2.0), Some(BLUE));
      assert!(paint_horizontal(
        &mut page,
        (66.6, 94.427_734, 226.2),
        horizontal,
        HorizontalTableBorderEdge::Upper,
        (
          HorizontalTableBorderJunction::internal(Some(vertical), Some(vertical)),
          HorizontalTableBorderJunction::internal(Some(vertical), Some(vertical))
        ),
      ));
      let mut lower_page = empty_page(
        PageSetup {
          width_pt: 612.0,
          height_pt: 792.0,
          ..Default::default()
        },
        0,
      );
      assert!(paint_horizontal(
        &mut lower_page,
        (66.6, 94.427_734, 226.2),
        horizontal,
        HorizontalTableBorderEdge::Lower,
        (
          HorizontalTableBorderJunction::internal(Some(vertical), Some(vertical)),
          HorizontalTableBorderJunction::internal(Some(vertical), Some(vertical))
        ),
      ));
      let mut rails = lower_page
        .items
        .iter()
        .filter_map(|item| match item {
          PageItem::Fill(fill) if fill.x_pt < 100.0 && fill.x_pt + fill.width_pt > 100.0 => {
            Some((fill.y_pt, fill.height_pt))
          }
          _ => None,
        })
        .collect::<Vec<_>>();
      rails.sort_by(|a, b| a.0.total_cmp(&b.0));
      assert_eq!(rails.len(), 3);
      for ((y, height), (native_y, native_height)) in
        rails
          .into_iter()
          .zip([(91.46, 0.72), (92.9, 3.0), (96.62, 0.72)])
      {
        assert!((y - native_y).abs() < 0.01, "{y} versus {native_y}");
        assert!((height - native_height).abs() < 0.01);
      }
    }
  }

  #[test]
  fn triple_small_gap_four_way_crossing_keeps_native_thin_rail_gaps() {
    let border = BorderStyle {
      width_pt: 6.0,
      compound: true,
      compound_pattern: BorderCompoundPattern::ThinThickThinSmallGap,
      color: RED,
      ..Default::default()
    };
    let vertical = BorderStyle {
      color: BLUE,
      ..border
    };
    let native = [
      [Some(RED), None, Some(RED), None, Some(RED)],
      [None, None, Some(RED), None, None],
      [Some(RED); 5],
      [None, None, Some(RED), None, None],
      [Some(RED), None, Some(RED), None, Some(RED)],
    ];
    let mut page = empty_page(
      PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        ..Default::default()
      },
      0,
    );
    for trailing in [false, true] {
      connection(
        &mut page,
        Bands::from_border(Some(border), false).unwrap(),
        Bands::from_border(Some(vertical), trailing).unwrap(),
        (530, 762, 579, 812),
        trailing,
        false,
        ConnectionKind::Crossing,
      );
    }
    for (row, dy) in [3, 9, 24, 40, 46].into_iter().enumerate() {
      for (column, dx) in [3, 9, 24, 40, 46].into_iter().enumerate() {
        let (x, y, w, h) =
          word_table_device_rectangle(&page, (530 + dx, 762 + dy, 531 + dx, 763 + dy));
        assert_eq!(
          color_at(&page, x + w / 2.0, y + h / 2.0),
          native[row][column]
        );
      }
    }
    let mut junction = HorizontalTableBorderJunction::internal(Some(vertical), Some(vertical));
    assert_eq!(ConnectionKind::from(junction), ConnectionKind::Tee);
    junction.has_horizontal_continuation = true;
    assert_eq!(ConnectionKind::from(junction), ConnectionKind::Crossing);
  }

  #[test]
  fn lower_small_gap_uses_native_far_edge_across_row_height_phases() {
    let border = BorderStyle {
      width_pt: 6.0,
      compound: true,
      compound_pattern: BorderCompoundPattern::ThickThinSmallGap,
      color: RED,
      ..Default::default()
    };
    for (phase, expected_top, expected_bottom) in [
      (-0.05, 343.13, 349.01),
      (0.0, 343.13, 349.01),
      (0.05, 343.13, 349.01),
      (0.10, 343.25, 349.13),
    ] {
      let mut page = empty_page(
        PageSetup {
          width_pt: 595.3,
          height_pt: 841.9,
          ..Default::default()
        },
        0,
      );
      assert!(paint_horizontal(
        &mut page,
        (134.8, 345.955_96 + phase, 460.45),
        border,
        HorizontalTableBorderEdge::Lower,
        (
          HorizontalTableBorderJunction::horizontal_outer(None, None),
          HorizontalTableBorderJunction::horizontal_outer(None, None),
        ),
      ));
      let fills = page
        .items
        .iter()
        .filter_map(|item| match item {
          PageItem::Fill(fill) => Some(fill),
          _ => None,
        })
        .collect::<Vec<_>>();
      assert_eq!(fills.len(), 2);
      let top = fills
        .iter()
        .map(|fill| fill.y_pt)
        .fold(f32::INFINITY, f32::min);
      let bottom = fills
        .iter()
        .map(|fill| fill.y_pt + fill.height_pt)
        .fold(f32::NEG_INFINITY, f32::max);
      assert!((top - expected_top).abs() < 0.0052);
      assert!((bottom - expected_bottom).abs() < 0.0052);
    }
  }

  #[test]
  fn native_colored_small_gap_corners_keep_order_and_row_inset_residual() {
    // Independent actual Word A6 controls: equal/reversed rails, unequal
    // extents and a single perpendicular. Gap pixels retain no border ink.
    let top = BorderStyle {
      width_pt: 6.0,
      compound: true,
      compound_pattern: BorderCompoundPattern::ThinThickSmallGap,
      color: RED,
      ..Default::default()
    };
    for (left, grid_left, grid_right, blue_x) in [
      (BorderStyle { color: BLUE, ..top }, 67.85, 351.75, 65.0),
      (
        BorderStyle {
          color: BLUE,
          compound_pattern: BorderCompoundPattern::ThickThinSmallGap,
          ..top
        },
        67.85,
        351.75,
        67.0,
      ),
      (
        BorderStyle {
          color: BLUE,
          width_pt: 2.5,
          ..top
        },
        68.725,
        350.875,
        67.6,
      ),
      (
        BorderStyle {
          color: BLUE,
          compound: false,
          width_pt: 4.5,
          ..top
        },
        68.225,
        351.375,
        66.1,
      ),
    ] {
      let mut page = empty_page(
        PageSetup {
          width_pt: 419.6,
          height_pt: 297.7,
          ..Default::default()
        },
        0,
      );
      assert!(paint_horizontal(
        &mut page,
        (grid_left, 73.85, grid_right),
        top,
        HorizontalTableBorderEdge::Upper,
        (
          HorizontalTableBorderJunction::horizontal_outer(None, Some(left)),
          HorizontalTableBorderJunction::horizontal_outer(None, None)
        ),
      ));
      assert_eq!(color_at(&page, blue_x, 71.0), Some(RED));
      assert_eq!(color_at(&page, blue_x, 76.75), Some(BLUE));
      assert_eq!(color_at(&page, 200.0, 75.5), None);
      assert!(
        page
          .items
          .iter()
          .all(|item| matches!(item, PageItem::Fill(_)))
      );
    }
  }
}
