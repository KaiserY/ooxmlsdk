//! Simple page-border shadows (ECMA-376 17.6.2/7/15/21).
//! Native independent-side, width, color, missing-side and text-offset controls
//! distinguish the border's reserved frame from its colored and shadow bands.
use super::{
  BorderDashPattern, BorderStyle, FillItem, Page, PageItem, RgbColor, ShadingPaint, units,
};
use crate::docx::CellBordersModel;

const BLACK: RgbColor = RgbColor { r: 0, g: 0, b: 0 };
const DOT: f32 = units::POINTS_PER_INCH / super::WORD_FIXED_OUTPUT_DPI;

#[derive(Clone, Copy, Default)]
struct Side {
  source: Option<BorderStyle>,
  width: f32,
  shadow: bool,
  color: RgbColor,
}

impl Side {
  fn new(source: Option<BorderStyle>) -> Option<Self> {
    let Some(border) = source else {
      return Some(Self::default());
    };
    if border.compound
      || border.relief.is_some()
      || border.dash_pattern != BorderDashPattern::Solid
      || !border.width_pt.is_finite()
      || border.width_pt <= 0.0
      || !border.spacing_pt.is_finite()
    {
      return None;
    }
    // As with the three-band page painter, Word first truncates to twips,
    // then to printer dots. Native sz8/18/24/32 give 8/18/25/33 dots.
    let twips = (f64::from(border.width_pt) * 20.0).trunc();
    let dots = (twips * 5.0 / 12.0).trunc().max(1.0);
    Some(Self {
      source,
      width: dots as f32 * DOT,
      shadow: border.shadow,
      color: border.color,
    })
  }

  fn joins(self, other: Self) -> bool {
    matches!((self.source, other.source), (Some(a), Some(b))
      if a.width_pt == b.width_pt && a.shadow == b.shadow)
  }

  fn reserve(self) -> f32 {
    self.width * if self.shadow { 2.0 } else { 1.0 }
  }

  fn edge(self, position: f32, trailing: bool, from_text: bool) -> f32 {
    let space = self.source.map_or(0.0, |border| border.spacing_pt);
    let offset = if from_text {
      -space - self.reserve()
    } else {
      space
    };
    let direction = if trailing { -1.0 } else { 1.0 };
    units::quantize_points_to_office_print_grid(position + direction * offset)
      + if trailing { DOT } else { 0.0 }
  }
}

pub(super) fn paint(
  page: &mut Page,
  reference: (f32, f32, f32, f32),
  borders: CellBordersModel,
) -> bool {
  let sources = [borders.top, borders.right, borders.bottom, borders.left];
  if !sources.iter().flatten().any(|border| border.shadow) {
    return false;
  }
  let mut sides = [Side::default(); 4];
  for (side, source) in sides.iter_mut().zip(sources) {
    let Some(value) = Side::new(source) else {
      return false;
    };
    *side = value;
  }
  let [top, right, bottom, left] = sides;
  let from_text = page.setup.borders_offset_from_text;
  let outer = (
    left.edge(reference.0, false, from_text),
    top.edge(reference.1, false, from_text),
    right.edge(reference.2, true, from_text),
    bottom.edge(reference.3, true, from_text),
  );
  let x_left = outer.0 + left.reserve() - left.width;
  let x_right = outer.2 - right.reserve();
  let y_top = outer.1 + top.reserve() - top.width;
  let y_bottom = outer.3 - bottom.reserve();
  let inner_top = y_top + top.width;
  if ![outer.0, outer.1, outer.2, outer.3]
    .into_iter()
    .all(f32::is_finite)
    || x_right <= x_left + left.width
    || y_bottom <= inner_top
  {
    return false;
  }

  fill(
    page,
    (x_left, inner_top, x_left + left.width, y_bottom),
    left.color,
  );
  fill(
    page,
    (x_right, inner_top, x_right + right.width, y_bottom),
    right.color,
  );
  if right.shadow {
    fill(
      page,
      (x_right + right.width, inner_top, outer.2, y_bottom),
      BLACK,
    );
  }

  // Equal-width/equal-shadow sides join. Otherwise the horizontal border
  // owns the corner's full reserved width. Color alone does not break a join.
  let top_left = if top.joins(left) { x_left } else { outer.0 };
  fill(page, (top_left, y_top, x_right, inner_top), top.color);
  if top.joins(right) {
    fill(
      page,
      (x_right, y_top, x_right + right.width, inner_top),
      top.color,
    );
  } else {
    if top.shadow {
      fill(page, (x_right, outer.1, outer.2, y_top), BLACK);
    }
    fill(page, (x_right, y_top, outer.2, inner_top), top.color);
  }

  let bottom_left = if bottom.joins(left) { x_left } else { outer.0 };
  fill(
    page,
    (bottom_left, y_bottom, x_right, y_bottom + bottom.width),
    bottom.color,
  );
  if bottom.shadow {
    let shadow_left = if bottom.joins(left) {
      x_left + left.width
    } else {
      outer.0
    };
    fill(
      page,
      (shadow_left, y_bottom + bottom.width, x_right, outer.3),
      BLACK,
    );
  }
  if bottom.joins(right) {
    if bottom.shadow {
      // Native differently colored corner controls retain each border's
      // brush in the joined shadow extensions. Matching colors use black.
      let same_color = bottom.color == right.color;
      fill(
        page,
        (x_right + right.width, y_bottom, outer.2, outer.3),
        if same_color { BLACK } else { right.color },
      );
      fill(
        page,
        (x_right, y_bottom + bottom.width, outer.2, outer.3),
        if same_color { BLACK } else { bottom.color },
      );
    }
    fill(
      page,
      (
        x_right,
        y_bottom,
        x_right + right.width,
        y_bottom + bottom.width,
      ),
      bottom.color,
    );
  } else if bottom.shadow {
    fill(
      page,
      (x_right, y_bottom, outer.2, y_bottom + bottom.width),
      BLACK,
    );
    fill(
      page,
      (x_right, y_bottom + bottom.width, outer.2, outer.3),
      bottom.color,
    );
  } else {
    fill(page, (x_right, y_bottom, outer.2, outer.3), bottom.color);
  }
  true
}

fn fill(page: &mut Page, (left, top, right, bottom): (f32, f32, f32, f32), color: RgbColor) {
  if right > left && bottom > top {
    page.items.push(PageItem::Fill(FillItem {
      x_pt: left,
      y_pt: top,
      width_pt: right - left,
      height_pt: bottom - top,
      paint: ShadingPaint::Solid(color),
    }));
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::layout::{PageSetup, empty_page};

  const YELLOW: RgbColor = RgbColor {
    r: 255,
    g: 255,
    b: 0,
  };

  fn borders(mask: u8) -> CellBordersModel {
    let side = |bit: u8| {
      Some(BorderStyle {
        width_pt: 6.0,
        spacing_pt: 24.0,
        color: YELLOW,
        shadow: mask & (1u8 << bit) != 0u8,
        ..BorderStyle::default()
      })
    };
    CellBordersModel {
      top: side(0),
      left: side(1),
      bottom: side(2),
      right: side(3),
    }
  }

  fn frame() -> Page {
    empty_page(
      PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        ..Default::default()
      },
      0,
    )
  }

  fn pixel(page: &Page, x: f32, y: f32) -> Option<RgbColor> {
    page.items.iter().rev().find_map(|item| match item {
      PageItem::Fill(fill)
        if x >= fill.x_pt
          && x < fill.x_pt + fill.width_pt
          && y >= fill.y_pt
          && y < fill.y_pt + fill.height_pt =>
      {
        let ShadingPaint::Solid(color) = fill.paint else {
          panic!("solid page border")
        };
        Some(color)
      }
      _ => None,
    })
  }

  #[test]
  fn page_shadow_reserves_space_and_keeps_independent_corner_flags() {
    // Actual native PDF samples; top-only and bottom-only differ at the
    // right corner and must not be normalized to a page-wide shadow flag.
    for (mask, samples) in [
      (
        15,
        [
          (33.0, 33.0, Some(YELLOW)),
          (25.0, 25.0, None),
          (585.0, 40.0, Some(BLACK)),
          (40.0, 765.0, Some(BLACK)),
        ],
      ),
      (
        1,
        [
          (25.0, 33.0, Some(YELLOW)),
          (25.0, 25.0, None),
          (585.0, 27.0, Some(BLACK)),
          (27.0, 40.0, Some(YELLOW)),
        ],
      ),
      (
        4,
        [
          (585.0, 759.0, Some(BLACK)),
          (585.0, 765.0, Some(YELLOW)),
          (25.0, 759.0, Some(YELLOW)),
          (25.0, 765.0, Some(BLACK)),
        ],
      ),
    ] {
      let mut page = frame();
      assert!(paint(&mut page, (0.0, 0.0, 612.0, 792.0), borders(mask)));
      for (x, y, expected) in samples {
        assert_eq!(pixel(&page, x, y), expected, "mask={mask} ({x},{y})");
      }
    }
  }

  #[test]
  fn page_shadow_native_width_and_text_reference_preserve_their_owners() {
    let mut page = frame();
    let mut sides = borders(15);
    for border in [
      &mut sides.top,
      &mut sides.right,
      &mut sides.bottom,
      &mut sides.left,
    ]
    .into_iter()
    .flatten()
    {
      border.width_pt = 2.25;
      border.spacing_pt = 0.0;
    }
    assert!(paint(&mut page, (0.0, 0.0, 612.0, 792.0), sides));
    assert_eq!(pixel(&page, 2.2, 2.2), Some(YELLOW));
    assert_eq!(pixel(&page, 2.0, 2.0), None);
    assert_eq!(pixel(&page, 610.0, 100.0), Some(BLACK));
    let mut page = frame();
    page.setup.borders_offset_from_text = true;
    let mut sides = borders(15);
    for border in [
      &mut sides.top,
      &mut sides.right,
      &mut sides.bottom,
      &mut sides.left,
    ]
    .into_iter()
    .flatten()
    {
      border.spacing_pt = 12.0;
    }
    assert!(paint(&mut page, (70.9, 70.9, 541.1, 721.1), sides));
    assert_eq!(pixel(&page, 55.0, 55.0), Some(YELLOW));
    assert_eq!(pixel(&page, 50.0, 50.0), None);
    assert_eq!(pixel(&page, 562.0, 100.0), Some(BLACK));
  }

  #[test]
  fn page_shadow_colored_join_and_absent_side_follow_native_paint() {
    let mut page = frame();
    let mut sides = borders(15);
    let blue = RgbColor { r: 0, g: 0, b: 255 };
    sides.bottom.as_mut().unwrap().color = blue;
    assert!(paint(&mut page, (0.0, 0.0, 612.0, 792.0), sides));
    assert_eq!(pixel(&page, 585.0, 759.0), Some(YELLOW));
    assert_eq!(pixel(&page, 585.0, 765.0), Some(blue));
    let mut page = frame();
    sides.left = None;
    assert!(paint(&mut page, (0.0, 0.0, 612.0, 792.0), sides));
    assert_eq!(pixel(&page, 1.0, 33.0), Some(YELLOW));
    assert_eq!(pixel(&page, 1.0, 765.0), Some(BLACK));
  }

  #[test]
  fn page_shadow_does_not_take_other_border_painters() {
    for sides in [
      borders(0),
      {
        let mut b = borders(15);
        b.top.as_mut().unwrap().compound = true;
        b
      },
      {
        let mut b = borders(15);
        b.top.as_mut().unwrap().dash_pattern = BorderDashPattern::Dashed;
        b
      },
    ] {
      let mut page = frame();
      assert!(!paint(&mut page, (0.0, 0.0, 612.0, 792.0), sides));
      assert!(page.items.is_empty());
    }
  }
}
