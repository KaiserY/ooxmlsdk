//! Paragraph borders are adjoining printer-grid brush bands.
//! Native width23/24/25, single-side, mixed-style and unequal-width controls
//! distinguish their corners and frame outsets from independent strokes.
use super::*;

#[derive(Clone, Copy, Default)]
struct Bands {
  widths: [i64; 3],
  colors: [Option<RgbColor>; 3],
}

impl Bands {
  fn from_border(border: Option<BorderStyle>, trailing: bool, ordinary: bool) -> Option<Self> {
    let Some(border) = border else {
      return Some(Self::default());
    };
    if border.relief.is_some() || !border.width_pt.is_finite() {
      return None;
    }
    if ordinary {
      if !ordinary_border(border) {
        return None;
      }
      if border.compound {
        let [a, b, c, 0, 0] = super::super::border_bands::device_widths(border)? else {
          return None;
        };
        return Some(Self {
          widths: [a, b, c],
          colors: [Some(border.color), None, Some(border.color)],
        });
      }
      return Some(Self {
        widths: [
          paragraph_border_device::thickness(BorderStyle {
            shadow: false,
            ..border
          })?,
          0,
          0,
        ],
        colors: [Some(border.color), None, None],
      });
    }
    if border.shadow {
      return None;
    }
    let widths = super::super::border_3d::device_band_widths(border)?;
    let mut colors = super::super::border_3d::band_colors(border);
    if trailing {
      colors.reverse();
    }
    Some(Self {
      widths,
      colors: colors.map(Some),
    })
  }

  fn extent(self) -> i64 {
    self.widths.into_iter().sum()
  }
}

fn ordinary_border(border: BorderStyle) -> bool {
  (!border.compound
    || matches!(
      border.compound_pattern,
      BorderCompoundPattern::Equal | BorderCompoundPattern::Double
    ))
    && paragraph_border_device::solid(BorderStyle {
      compound: false,
      shadow: false,
      ..border
    })
}

struct Painter<'a> {
  page: &'a mut Page,
  device: wordprocessing_table_device::PageCoordinates,
}

impl Painter<'_> {
  fn fill(&mut self, (left, top, right, bottom): (i64, i64, i64, i64), color: RgbColor) {
    if right <= left || bottom <= top {
      return;
    }
    let (x, y, width, height) =
      self
        .device
        .map_device_rectangle((left, top, right - left, bottom - top));
    self.page.items.push(PageItem::Fill(FillItem {
      x_pt: x,
      y_pt: y,
      width_pt: width,
      height_pt: height,
      paint: ShadingPaint::Solid(color),
    }));
  }

  fn horizontal(&mut self, bands: Bands, bounds: (i64, i64, i64, i64), trailing: bool) {
    let (left, top, right, bottom) = bounds;
    let mut offset = 0;
    for (width, color) in bands.widths.into_iter().zip(bands.colors) {
      let y = if trailing {
        bottom - offset - width
      } else {
        top + offset
      };
      if let Some(color) = color {
        self.fill((left, y, right, y + width), color);
      }
      offset += width;
    }
  }

  fn vertical(&mut self, bands: Bands, bounds: (i64, i64, i64, i64), trailing: bool) {
    let (left, top, right, bottom) = bounds;
    let mut offset = 0;
    for (width, color) in bands.widths.into_iter().zip(bands.colors) {
      let x = if trailing {
        right - offset - width
      } else {
        left + offset
      };
      if let Some(color) = color {
        self.fill((x, top, x + width, bottom), color);
      }
      offset += width;
    }
  }

  fn corner(
    &mut self,
    horizontal: Bands,
    vertical: Bands,
    bounds: (i64, i64, i64, i64),
    right_corner: bool,
    bottom_corner: bool,
  ) {
    let (left, top, right, bottom) = bounds;
    if horizontal.widths != vertical.widths {
      // Native unequal joins belong entirely to the horizontal rule.
      self.horizontal(horizontal, bounds, bottom_corner);
      return;
    }
    let mut offset = 0;
    for (index, width) in horizontal.widths.into_iter().enumerate() {
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
      // Alternating vertical/horizontal fills retain the native shared square.
      if let Some(color) = vertical.colors[index] {
        self.fill((x, y_start, x + width, y_end), color);
      }
      if let Some(color) = horizontal.colors[index] {
        self.fill((x_start, y, x_end, y + width), color);
      }
      offset += width;
    }
  }
}

pub(super) fn paint(
  page: &mut Page,
  frame: (f32, f32, f32, f32),
  borders: ParagraphBordersModel,
) -> bool {
  let sources = [borders.top, borders.right, borders.bottom, borders.left];
  if sources.iter().all(Option::is_none)
    || ![frame.0, frame.1, frame.2, frame.3]
      .into_iter()
      .all(f32::is_finite)
  {
    return false;
  }
  let mut bands = [Bands::default(); 4];
  // Native single/double controls retain gaps and black shadow brushes.
  // Three-D colors keep their separate owner; unsupported families fall back.
  let ordinary = sources.iter().flatten().copied().all(ordinary_border);
  let shadow = sources
    .iter()
    .flatten()
    .next()
    .is_some_and(|border| border.shadow);
  if ordinary
    && sources
      .iter()
      .flatten()
      .any(|border| border.shadow != shadow)
  {
    // Native mixed-shadow controls have a separate corner owner, including
    // black corner caps. Keep that existing route until it is represented.
    return false;
  }
  for (index, source) in sources.into_iter().enumerate() {
    let Some(side) = Bands::from_border(source, matches!(index, 1 | 2), ordinary) else {
      // Other border families keep the existing styled-line painter.
      return false;
    };
    bands[index] = side;
  }
  let Some(device) =
    wordprocessing_table_device::page_coordinates(page.setup.width_pt, page.setup.height_pt)
  else {
    return false;
  };
  let coordinate = wordprocessing_table_device::source_coordinate;
  let mut distances = [0; 4];
  for (index, source) in sources.into_iter().enumerate() {
    if let Some(border) = source {
      distances[index] = if ordinary {
        let Some(distance) = paragraph_border_device::distance(BorderStyle {
          compound: false,
          shadow: false,
          ..border
        }) else {
          return false;
        };
        distance
      } else {
        coordinate(border.spacing_pt)
      };
    }
  }
  let [top_space, right_space, bottom_space, left_space] = distances;
  // Word first realizes the text frame and its twelve-dot horizontal
  // outsets, independently of border distances and each band's thickness.
  let inner = (
    coordinate(frame.0) - 12 - left_space,
    coordinate(frame.1) - top_space,
    coordinate(frame.2) + 12 + right_space,
    coordinate(frame.3) + bottom_space,
  );
  if inner.2 <= inner.0 || inner.3 <= inner.1 {
    return false;
  }
  let [top, right, bottom, left] = bands;
  let outer = (
    inner.0 - left.extent(),
    inner.1 - top.extent(),
    inner.2 + right.extent(),
    inner.3 + bottom.extent(),
  );
  let mut painter = Painter { page, device };
  painter.corner(
    top,
    left,
    (outer.0, outer.1, inner.0, inner.1),
    false,
    false,
  );
  painter.horizontal(top, (inner.0, outer.1, inner.2, inner.1), false);
  painter.corner(
    top,
    right,
    (inner.2, outer.1, outer.2, inner.1),
    true,
    false,
  );
  painter.corner(
    bottom,
    left,
    (outer.0, inner.3, inner.0, outer.3),
    false,
    true,
  );
  painter.horizontal(bottom, (inner.0, inner.3, inner.2, outer.3), true);
  painter.corner(
    bottom,
    right,
    (inner.2, inner.3, outer.2, outer.3),
    true,
    true,
  );
  painter.vertical(left, (outer.0, inner.1, inner.0, inner.3), false);
  painter.vertical(right, (inner.2, inner.1, outer.2, inner.3), true);
  if ordinary && shadow {
    // Paragraph shadows are solid black brushes of the complete BRC extent,
    // including a double rule's gap. Native colored/partial-edge controls
    // distinguish these strips from offset copies of the individual rails.
    // Top and left add no strip; the lower-right corner joins both shadows.
    let black = RgbColor { r: 0, g: 0, b: 0 };
    painter.fill(
      (
        inner.0,
        outer.3,
        outer.2 + right.extent(),
        outer.3 + bottom.extent(),
      ),
      black,
    );
    painter.fill(
      (
        outer.2,
        inner.1,
        outer.2 + right.extent(),
        outer.3 + bottom.extent(),
      ),
      black,
    );
  }
  true
}

pub(super) fn shading_rectangle(
  page: &Page,
  frame: (f32, f32, f32, f32),
  borders: ParagraphBordersModel,
) -> Option<(f32, f32, f32, f32)> {
  if ![frame.0, frame.1, frame.2, frame.3]
    .into_iter()
    .all(f32::is_finite)
  {
    return None;
  }
  let device =
    wordprocessing_table_device::page_coordinates(page.setup.width_pt, page.setup.height_pt)?;
  let distance = |border: Option<BorderStyle>| {
    border.map_or(Some(0), |border| {
      paragraph_border_device::distance(BorderStyle {
        compound: false,
        shadow: false,
        ..border
      })
    })
  };
  // Native PatBlt background and border controls use the same realized
  // frame/outsets. Both horizontal background gaps belong to the right BRC;
  // absent sides remain unexpanded, independently of their ink families.
  let horizontal = distance(borders.right)?;
  let coordinate = wordprocessing_table_device::source_coordinate;
  let left = coordinate(frame.0)
    - 12
    - if borders.left.is_some() {
      horizontal
    } else {
      0
    };
  let right = coordinate(frame.2)
    + 12
    + if borders.right.is_some() {
      horizontal
    } else {
      0
    };
  let top = coordinate(frame.1) - distance(borders.top)?;
  let bottom = coordinate(frame.3) + distance(borders.bottom)?;
  (right > left && bottom > top)
    .then(|| device.map_device_rectangle((left, top, right - left, bottom - top)))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn bordered_background_uses_native_frame_outsets_and_truncated_distances() {
    let page = empty_page(
      PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        ..Default::default()
      },
      0,
    );
    let border = BorderStyle {
      width_pt: 0.75,
      spacing_pt: 1.0,
      ..Default::default()
    };
    let mut borders = ParagraphBordersModel {
      top: Some(border),
      bottom: Some(border),
      left: Some(border),
      right: Some(border),
      ..Default::default()
    };
    let frame = (72.0, 73.8, 540.0, 82.92);
    let device = wordprocessing_table_device::page_coordinates(612.0, 792.0).unwrap();
    assert_eq!(
      shading_rectangle(&page, frame, borders),
      Some(device.map_device_rectangle((580, 607, 3940, 92)))
    );
    borders.right = None;
    assert_eq!(
      shading_rectangle(&page, frame, borders),
      Some(device.map_device_rectangle((588, 607, 3924, 92)))
    );
  }
  use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

  fn border(value: w::BorderValues, size: u32) -> BorderStyle {
    crate::docx::top_border_style(&w::TopBorder {
      val: value,
      size: Some(size),
      ..Default::default()
    })
    .unwrap()
  }

  #[test]
  fn native_solid_unequal_corner_keeps_horizontal_color_and_complete_brush() {
    let mut page = empty_page(
      PageSetup {
        width_pt: 595.3,
        height_pt: 841.9,
        ..Default::default()
      },
      0,
    );
    let device = wordprocessing_table_device::page_coordinates(595.3, 841.9).unwrap();
    let horizontal = BorderStyle {
      width_pt: 0.5,
      color: RgbColor {
        r: 204,
        g: 32,
        b: 32,
      },
      ..Default::default()
    };
    let vertical = BorderStyle {
      width_pt: 3.0,
      color: RgbColor {
        r: 170,
        g: 32,
        b: 170,
      },
      ..Default::default()
    };
    let h = Bands::from_border(Some(horizontal), false, true).unwrap();
    let v = Bands::from_border(Some(vertical), false, true).unwrap();
    // Actual unequal all-four-edge native control: the complete 3pt x.48pt
    // upper-left join is red, including the outermost left strip.
    Painter {
      page: &mut page,
      device,
    }
    .corner(h, v, (680, 616, 705, 620), false, false);
    let [PageItem::Fill(fill)] = page.items.as_slice() else {
      panic!("one opaque brush corner")
    };
    assert_eq!(fill.paint, ShadingPaint::Solid(horizontal.color));
    for (actual, expected) in [
      (fill.x_pt, 81.624),
      (fill.y_pt, 73.92),
      (fill.width_pt, 3.0),
      (fill.height_pt, 0.48),
    ] {
      assert!(
        (actual - expected).abs() <= 0.02401,
        "{actual} != {expected}"
      );
    }
  }

  #[test]
  fn native_paragraph_band_widths_and_trailing_colors_are_independent() {
    for (size, widths) in [
      (6, [6, 6, 6]),
      (23, [6, 23, 6]),
      (24, [12, 25, 12]),
      (25, [12, 25, 12]),
      (48, [12, 50, 12]),
    ] {
      for (value, shades) in [
        (w::BorderValues::ThreeDEmboss, [192, 96, 0]),
        (w::BorderValues::ThreeDEngrave, [0, 96, 192]),
      ] {
        let border = border(value, size);
        let leading = Bands::from_border(Some(border), false, false).unwrap();
        let trailing = Bands::from_border(Some(border), true, false).unwrap();
        assert_eq!(leading.widths, widths);
        assert_eq!(trailing.widths, widths);
        assert_eq!(leading.colors.map(|c| c.unwrap().r), shades);
        assert_eq!(
          trailing.colors.map(|c| c.unwrap().r),
          [shades[2], shades[1], shades[0]]
        );
      }
    }
  }

  #[test]
  fn ordinary_double_rails_keep_the_transparent_middle_band() {
    for (size, dots) in [(6, 6), (12, 12), (18, 18)] {
      let mut border = border(w::BorderValues::Double, size);
      border.color = RgbColor {
        r: 192,
        g: 48,
        b: 32,
      };
      for trailing in [false, true] {
        let bands = Bands::from_border(Some(border), trailing, true).unwrap();
        assert_eq!(bands.widths, [dots; 3]);
        assert_eq!(bands.colors, [Some(border.color), None, Some(border.color)]);
      }
    }
  }

  #[test]
  fn paragraph_double_shadow_is_a_complete_black_brush_outside_colored_rails() {
    let mut border = border(w::BorderValues::Double, 6);
    border.shadow = true;
    border.color = RgbColor {
      r: 192,
      g: 48,
      b: 32,
    };
    let setup = PageSetup {
      width_pt: 612.0,
      height_pt: 792.0,
      ..Default::default()
    };
    let mut page = empty_page(setup, 0);
    let frame = (72.0, 100.0, 540.0, 118.0);
    assert!(paint(
      &mut page,
      frame,
      ParagraphBordersModel {
        top: Some(border),
        left: Some(border),
        bottom: Some(border),
        right: Some(border),
        ..Default::default()
      },
    ));
    let coordinate = wordprocessing_table_device::source_coordinate;
    let device = wordprocessing_table_device::page_coordinates(612.0, 792.0).unwrap();
    let inner = (
      coordinate(frame.0) - 12,
      coordinate(frame.1),
      coordinate(frame.2) + 12,
      coordinate(frame.3),
    );
    let expected = [
      (inner.0, inner.3 + 18, inner.2 + 36 - inner.0, 18),
      (inner.2 + 18, inner.1, 18, inner.3 + 36 - inner.1),
    ];
    for (item, rectangle) in page.items.iter().rev().take(2).rev().zip(expected) {
      let PageItem::Fill(fill) = item else {
        panic!("native black shadow brush")
      };
      let (x, y, width, height) = device.map_device_rectangle(rectangle);
      assert_eq!(
        (fill.x_pt, fill.y_pt, fill.width_pt, fill.height_pt),
        (x, y, width, height)
      );
      assert_eq!(
        fill.paint,
        ShadingPaint::Solid(RgbColor { r: 0, g: 0, b: 0 })
      );
    }
    // The main double line's gap stays transparent. The shadow below it is
    // eighteen dots deep, rather than two thin copies with a six-dot gap.
    let (gap_x, gap_y, _, _) =
      device.map_device_rectangle(((inner.0 + inner.2) / 2, inner.3 + 9, 1, 1));
    assert!(
      !page
        .items
        .iter()
        .any(|item| matches!(item, PageItem::Fill(fill)
      if gap_x > fill.x_pt && gap_x < fill.x_pt + fill.width_pt
        && gap_y > fill.y_pt && gap_y < fill.y_pt + fill.height_pt))
    );
  }

  #[test]
  fn top_left_shadow_is_inert_and_mixed_shadow_caps_keep_their_existing_owner() {
    let setup = PageSetup {
      width_pt: 612.0,
      height_pt: 792.0,
      ..Default::default()
    };
    let border = border(w::BorderValues::Double, 6);
    let frame = (72.0, 100.0, 540.0, 118.0);
    let mut pages = [empty_page(setup, 0), empty_page(setup, 0)];
    for (page, shadow) in pages.iter_mut().zip([false, true]) {
      let border = BorderStyle { shadow, ..border };
      assert!(paint(
        page,
        frame,
        ParagraphBordersModel {
          top: Some(border),
          left: Some(border),
          ..Default::default()
        },
      ));
    }
    assert_eq!(pages[0].items.len(), pages[1].items.len());
    for (a, b) in pages[0].items.iter().zip(&pages[1].items) {
      let (PageItem::Fill(a), PageItem::Fill(b)) = (a, b) else {
        panic!("native border bands")
      };
      assert_eq!(
        (a.x_pt, a.y_pt, a.width_pt, a.height_pt, a.paint),
        (b.x_pt, b.y_pt, b.width_pt, b.height_pt, b.paint)
      );
    }
    let mut page = empty_page(setup, 0);
    assert!(!paint(
      &mut page,
      frame,
      ParagraphBordersModel {
        top: Some(BorderStyle {
          shadow: true,
          ..border
        }),
        right: Some(border),
        ..Default::default()
      },
    ));
    assert!(page.items.is_empty());
  }

  #[test]
  fn native_paragraph_corners_interleave_neighbor_faces() {
    let mut page = empty_page(
      PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        ..Default::default()
      },
      0,
    );
    let device = wordprocessing_table_device::page_coordinates(612.0, 792.0).unwrap();
    let h = Bands::from_border(
      Some(border(w::BorderValues::ThreeDEmboss, 24)),
      false,
      false,
    )
    .unwrap();
    let v = Bands::from_border(
      Some(border(w::BorderValues::ThreeDEngrave, 24)),
      false,
      false,
    )
    .unwrap();
    Painter {
      page: &mut page,
      device,
    }
    .corner(h, v, (506, 601, 555, 650), false, false);
    let expected = [
      ((506, 601, 12, 49), 0),
      ((506, 601, 49, 12), 192),
      ((518, 613, 25, 37), 96),
      ((518, 613, 37, 25), 96),
      ((543, 638, 12, 12), 192),
      ((543, 638, 12, 12), 0),
    ];
    assert_eq!(page.items.len(), expected.len());
    for (item, (rect, shade)) in page.items.iter().zip(expected) {
      let PageItem::Fill(fill) = item else {
        panic!("brush rectangle")
      };
      let (x, y, width, height) = device.map_device_rectangle(rect);
      assert_eq!(
        (fill.x_pt, fill.y_pt, fill.width_pt, fill.height_pt),
        (x, y, width, height)
      );
      assert_eq!(
        fill.paint,
        ShadingPaint::Solid(RgbColor {
          r: shade,
          g: shade,
          b: shade
        })
      );
    }
  }

  #[test]
  fn unequal_paragraph_joins_belong_to_horizontal_band_and_other_families_fall_back() {
    let mut page = empty_page(
      PageSetup {
        width_pt: 612.0,
        height_pt: 792.0,
        ..Default::default()
      },
      0,
    );
    let device = wordprocessing_table_device::page_coordinates(612.0, 792.0).unwrap();
    let h =
      Bands::from_border(Some(border(w::BorderValues::ThreeDEmboss, 6)), false, false).unwrap();
    let v = Bands::from_border(
      Some(border(w::BorderValues::ThreeDEngrave, 24)),
      false,
      false,
    )
    .unwrap();
    Painter {
      page: &mut page,
      device,
    }
    .corner(h, v, (506, 601, 555, 619), false, false);
    assert_eq!(page.items.len(), 3);
    for (item, shade) in page.items.iter().zip([192, 96, 0]) {
      let PageItem::Fill(fill) = item else {
        panic!("horizontal brush")
      };
      assert_eq!(
        fill.paint,
        ShadingPaint::Solid(RgbColor {
          r: shade,
          g: shade,
          b: shade
        })
      );
    }
    let count = page.items.len();
    assert!(!paint(
      &mut page,
      (72.0, 79.0, 540.0, 95.0),
      ParagraphBordersModel {
        top: Some(border(w::BorderValues::Dotted, 24)),
        ..Default::default()
      }
    ));
    assert_eq!(page.items.len(), count);
  }
}
