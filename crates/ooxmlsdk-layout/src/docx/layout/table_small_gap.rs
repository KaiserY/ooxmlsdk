//! Small-gap table rails and their native rectangular corner connectors.
use super::*;

#[derive(Clone, Copy, Default)]
struct Bands {
  widths: [i64; 3],
  colors: [Option<RgbColor>; 3],
}

pub(super) fn is_style(border: BorderStyle) -> bool {
  border.compound
    && border.dash_pattern == BorderDashPattern::Solid
    && matches!(
      border.compound_pattern,
      BorderCompoundPattern::ThinThickSmallGap | BorderCompoundPattern::ThickThinSmallGap
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
        colors: [Some(border.color), None, Some(border.color)],
      }
    } else if !border.compound
      && border.relief.is_none()
      && border.dash_pattern == BorderDashPattern::Solid
    {
      let twips = (f64::from(border.width_pt) * 20.0).floor() as i64;
      Self {
        widths: [(twips * 5 / 12).max(1), 0, 0],
        colors: [Some(border.color), None, None],
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
  if !is_style(border) || !leading.is_horizontal_outer || !trailing.is_horizontal_outer {
    return false;
  }
  let lower = matches!(edge, HorizontalTableBorderEdge::Lower);
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
  let bottom = if lower {
    top + horizontal.extent()
  } else {
    // The top connector reaches the row's layout inset, independently of
    // the component-wise truncated paint extent (native 50 vs 49 dots).
    wordprocessing_table_device::source_coordinate(y + border.width_pt / 2.0)
  };
  horizontal_strip(
    page,
    horizontal,
    (inner_left, top, inner_right, bottom),
    lower,
  );
  corner(
    page,
    horizontal,
    vertical_left,
    (outer_left, top, inner_left, bottom),
    false,
    lower,
  );
  corner(
    page,
    horizontal,
    vertical_right,
    (inner_right, top, outer_right, bottom),
    true,
    lower,
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
  for index in 0..3 {
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
