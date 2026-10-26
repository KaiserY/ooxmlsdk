//! Word page-border three-band frames: MS-DOC BrcType 0x18/0x19.
//! Native width, direction, partial/asymmetric-frame and text-offset controls
//! distinguish filled printer-grid bands from independent centered strokes.
use super::{BorderStyle, FillItem, Page, PageItem, RgbColor, ShadingPaint, units};
use crate::docx::{CellBordersModel, border_3d};

#[derive(Clone, Copy, Default)]
struct Bands {
  widths: [f32; 3],
  colors: [RgbColor; 3],
}

impl Bands {
  fn from_border(border: Option<BorderStyle>, reverse: bool) -> Option<Self> {
    let Some(border) = border else {
      return Some(Self::default());
    };
    let widths = border_3d::device_band_widths(border)?
      .map(|dots| dots as f32 * units::POINTS_PER_INCH / super::WORD_FIXED_OUTPUT_DPI);
    let mut colors = border_3d::band_colors(border);
    if reverse {
      colors.reverse();
    }
    Some(Self { widths, colors })
  }

  fn extent(self) -> f32 {
    self.widths.iter().sum()
  }
}

pub(super) fn paint(
  page: &mut Page,
  reference: (f32, f32, f32, f32),
  borders: CellBordersModel,
) -> bool {
  let sources = [borders.top, borders.right, borders.bottom, borders.left];
  if sources.iter().all(Option::is_none) {
    return false;
  }
  let mut bands = [Bands::default(); 4];
  for (index, source) in sources.into_iter().enumerate() {
    let Some(side) = Bands::from_border(source, matches!(index, 1 | 2)) else {
      // Other line/art styles retain their own established painter.
      return false;
    };
    bands[index] = side;
  }
  let [top, right, bottom, left] = bands;
  let dot = units::POINTS_PER_INCH / super::WORD_FIXED_OUTPUT_DPI;
  let text_offset = page.setup.borders_offset_from_text;
  let edge = |position: f32, border: Option<BorderStyle>, band: Bands, trailing: bool| {
    let space = border.map_or(0.0, |border| border.spacing_pt);
    let offset = if text_offset { -space } else { space };
    let direction = if trailing { -1.0 } else { 1.0 };
    let mut value = units::quantize_points_to_office_print_grid(position + direction * offset);
    if trailing {
      value += dot;
    }
    if text_offset {
      value -= direction * band.extent();
    }
    value
  };
  let outer = (
    edge(reference.0, borders.left, left, false),
    edge(reference.1, borders.top, top, false),
    edge(reference.2, borders.right, right, true),
    edge(reference.3, borders.bottom, bottom, true),
  );
  let inner = (
    outer.0 + left.extent(),
    outer.1 + top.extent(),
    outer.2 - right.extent(),
    outer.3 - bottom.extent(),
  );
  if ![outer.0, outer.1, outer.2, outer.3]
    .into_iter()
    .all(f32::is_finite)
    || inner.2 <= inner.0
    || inner.3 <= inner.1
  {
    return false;
  }
  horizontal(page, top, (inner.0, outer.1, inner.2, inner.1), false);
  horizontal(page, bottom, (inner.0, inner.3, inner.2, outer.3), true);
  vertical(page, left, (outer.0, inner.1, inner.0, inner.3), false);
  vertical(page, right, (inner.2, inner.1, outer.2, inner.3), true);
  corner(
    page,
    top,
    left,
    (outer.0, outer.1, inner.0, inner.1),
    false,
    false,
  );
  corner(
    page,
    top,
    right,
    (inner.2, outer.1, outer.2, inner.1),
    true,
    false,
  );
  corner(
    page,
    bottom,
    left,
    (outer.0, inner.3, inner.0, outer.3),
    false,
    true,
  );
  corner(
    page,
    bottom,
    right,
    (inner.2, inner.3, outer.2, outer.3),
    true,
    true,
  );
  true
}

fn fill(page: &mut Page, bounds: (f32, f32, f32, f32), color: RgbColor) {
  let (left, top, right, bottom) = bounds;
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

fn horizontal(page: &mut Page, bands: Bands, bounds: (f32, f32, f32, f32), reverse: bool) {
  let (left, top, right, bottom) = bounds;
  let mut offset = 0.0;
  for (width, color) in bands.widths.into_iter().zip(bands.colors) {
    let y = if reverse {
      bottom - offset - width
    } else {
      top + offset
    };
    fill(page, (left, y, right, y + width), color);
    offset += width;
  }
}

fn vertical(page: &mut Page, bands: Bands, bounds: (f32, f32, f32, f32), reverse: bool) {
  let (left, top, right, bottom) = bounds;
  let mut offset = 0.0;
  for (width, color) in bands.widths.into_iter().zip(bands.colors) {
    let x = if reverse {
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
  horizontal_bands: Bands,
  vertical_bands: Bands,
  bounds: (f32, f32, f32, f32),
  right_corner: bool,
  bottom_corner: bool,
) {
  let (left, top, right, bottom) = bounds;
  if right <= left || bottom <= top {
    return;
  }
  if horizontal_bands.widths != vertical_bands.widths {
    // Native unequal-width corners give the whole join to the horizontal
    // border. Equal-width corners instead interleave the two side colors.
    horizontal(page, horizontal_bands, bounds, bottom_corner);
    return;
  }
  let mut offset = 0.0;
  for index in 0..3 {
    let width = horizontal_bands.widths[index];
    let x = if right_corner {
      right - offset - width
    } else {
      left + offset
    };
    let y = if bottom_corner {
      bottom - offset - width
    } else {
      top + offset
    };
    let (x_start, x_end) = if right_corner {
      (left, right - offset)
    } else {
      (left + offset, right)
    };
    let (y_start, y_end) = if bottom_corner {
      (top, bottom - offset)
    } else {
      (top + offset, bottom)
    };
    fill(
      page,
      (x, y_start, x + width, y_end),
      vertical_bands.colors[index],
    );
    fill(
      page,
      (x_start, y, x_end, y + width),
      horizontal_bands.colors[index],
    );
    offset += width;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::layout::{PageSetup, empty_page};
  use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

  fn border(value: w::BorderValues, size: u32) -> BorderStyle {
    crate::docx::page_border_style(value, Some(size), Some(24), Some("auto"), None).unwrap()
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
          panic!("solid band")
        };
        Some(color)
      }
      _ => None,
    })
  }

  #[test]
  fn three_d_page_native_grid_bands_and_trailing_dot() {
    for (size, widths) in [
      (2, [0.72, 0.24, 0.72]),
      (6, [0.72, 0.72, 0.72]),
      (23, [0.72, 2.76, 0.72]),
      (24, [1.44, 3.0, 1.44]),
      (25, [1.44, 3.0, 1.44]),
      (48, [1.44, 6.0, 1.44]),
    ] {
      let mut page = frame();
      let borders = CellBordersModel {
        top: Some(border(w::BorderValues::ThreeDEmboss, size)),
        left: Some(border(w::BorderValues::ThreeDEmboss, size)),
        bottom: Some(border(w::BorderValues::ThreeDEngrave, size)),
        right: Some(border(w::BorderValues::ThreeDEngrave, size)),
      };
      assert!(paint(&mut page, (0.0, 0.0, 612.0, 792.0), borders));
      let mut offset = 0.0;
      for (width, shade) in widths.into_iter().zip([192, 96, 0]) {
        let color = Some(RgbColor {
          r: shade,
          g: shade,
          b: shade,
        });
        assert_eq!(pixel(&page, 306.0, 24.0 + offset + width / 2.0), color);
        assert_eq!(pixel(&page, 588.12 - offset - width / 2.0, 396.0), color);
        offset += width;
      }
      assert_eq!(pixel(&page, 588.13, 396.0), None);
      assert_eq!(pixel(&page, 306.0, 768.13), None);
      assert!(
        page
          .items
          .iter()
          .all(|item| matches!(item, PageItem::Fill(_)))
      );
    }
  }

  #[test]
  fn three_d_page_mixed_directions_and_unequal_corners_keep_side_ownership() {
    let emboss = border(w::BorderValues::ThreeDEmboss, 6);
    let mut borders = CellBordersModel {
      top: Some(emboss),
      left: Some(emboss),
      right: Some(emboss),
      bottom: Some(emboss),
    };
    let mut page = frame();
    assert!(paint(&mut page, (0.0, 0.0, 612.0, 792.0), borders));
    assert_eq!(
      pixel(&page, 587.8, 24.2),
      Some(RgbColor {
        r: 192,
        g: 192,
        b: 192
      })
    );
    assert_eq!(
      pixel(&page, 587.8, 25.5),
      Some(RgbColor { r: 0, g: 0, b: 0 })
    );
    borders.right = Some(border(w::BorderValues::ThreeDEngrave, 24));
    page.items.clear();
    assert!(paint(&mut page, (0.0, 0.0, 612.0, 792.0), borders));
    assert_eq!(
      pixel(&page, 587.8, 25.5),
      Some(RgbColor { r: 0, g: 0, b: 0 })
    );
    assert_eq!(
      pixel(&page, 587.8, 26.5),
      Some(RgbColor {
        r: 192,
        g: 192,
        b: 192
      })
    );
  }

  #[test]
  fn three_d_page_missing_side_does_not_shorten_remaining_borders() {
    let emboss = border(w::BorderValues::ThreeDEmboss, 6);
    let borders = CellBordersModel {
      top: Some(emboss),
      left: Some(emboss),
      right: Some(border(w::BorderValues::ThreeDEngrave, 6)),
      bottom: None,
    };
    let mut page = frame();
    assert!(paint(&mut page, (0.0, 0.0, 612.0, 792.0), borders));
    assert_eq!(
      pixel(&page, 24.2, 792.05),
      Some(RgbColor {
        r: 192,
        g: 192,
        b: 192
      })
    );
    assert_eq!(pixel(&page, 306.0, 780.0), None);
  }
}
