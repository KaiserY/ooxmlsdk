//! Filled outer joins for the opaque bands of collapsed table borders.
use super::*;

#[derive(Clone, Copy)]
struct Bands {
  widths: [i64; 3],
  colors: [RgbColor; 3],
}

impl Bands {
  fn new(border: BorderStyle) -> Option<Self> {
    if border.shadow
      || border.relief.is_some()
      || border.spacing_pt > 0.0
      || border.dash_pattern != BorderDashPattern::Solid
      || !border.width_pt.is_finite()
      || border.width_pt <= 0.0
    {
      return None;
    }
    if let Some(widths) = super::super::border_3d::device_band_widths(border) {
      return Some(Self {
        widths,
        colors: super::super::border_3d::band_colors(border),
      });
    }
    if border.compound {
      // A table double border's clear gap has a separate junction owner.
      // Do not import the page-frame painter's opaque-white gap policy.
      return None;
    }
    let twips = (f64::from(border.width_pt) * 20.0).floor() as i64;
    Some(Self {
      widths: [(twips * 5 / 12).max(1), 0, 0],
      colors: [border.color; 3],
    })
  }
}

pub(super) fn paint(
  page: &mut Page,
  (grid_x, horizontal_center_y): (f32, f32),
  horizontal: BorderStyle,
  vertical: BorderStyle,
  leading: bool,
  top: bool,
) -> bool {
  let (Some(mut h), Some(mut v)) = (Bands::new(horizontal), Bands::new(vertical)) else {
    return false;
  };
  if h.widths != v.widths {
    return false;
  }
  let left = wordprocessing_table_device::source_coordinate(grid_x - vertical.width_pt / 2.0);
  let upper =
    wordprocessing_table_device::source_coordinate(horizontal_center_y - horizontal.width_pt / 2.0);
  let right = left + v.widths.into_iter().sum::<i64>();
  // Word reserves the full top border in layout, even when individually
  // truncated three-D bands paint one source dot less. Native upper and
  // lower L controls distinguish that reservation from the lower ink extent.
  let height = if top {
    (f64::from(horizontal.width_pt) * 20.0).floor() as i64 * 5 / 12
  } else {
    h.widths.into_iter().sum()
  };
  let bottom = upper + height;
  if !leading {
    v.widths.reverse();
    v.colors.reverse();
  }
  if !top {
    h.widths.reverse();
    h.colors.reverse();
  }
  let mut fill = |bounds, color| {
    let (x_pt, y_pt, width_pt, height_pt) = word_table_device_rectangle(page, bounds);
    if width_pt > 0.0 && height_pt > 0.0 {
      page.items.push(PageItem::Fill(FillItem {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        paint: ShadingPaint::Solid(color),
      }));
    }
  };
  let mut offset = 0;
  for index in 0..3 {
    let width = h.widths[index];
    let x = if leading {
      left + offset
    } else {
      right - offset - width
    };
    let y = if top {
      upper + offset
    } else {
      bottom - offset - width
    };
    let (x_start, x_end) = if leading {
      (left + offset, right)
    } else {
      (left, right - offset)
    };
    let (y_start, y_end) = if top {
      (upper + offset, bottom)
    } else {
      (upper, bottom - offset)
    };
    // Equal-band native L joins interleave the vertical then horizontal
    // faces. In particular, an ordinary single corner belongs to the
    // horizontal color, while a three-D corner retains its stepped faces.
    fill((x, y_start, x + width, y_end), v.colors[index]);
    fill((x_start, y, x_end, y + width), h.colors[index]);
    offset += width;
  }
  true
}

pub(super) fn trim_outer_opaque_body(
  page: &mut Page,
  body: std::ops::Range<usize>,
  grid_x: f32,
  horizontal: BorderStyle,
  vertical: BorderStyle,
  leading: bool,
) {
  let (Some(h), Some(v)) = (Bands::new(horizontal), Bands::new(vertical)) else {
    return;
  };
  if super::super::border_3d::band_widths(horizontal).is_none() || h.widths != v.widths {
    return;
  }
  let left = wordprocessing_table_device::source_coordinate(grid_x - vertical.width_pt / 2.0);
  let right = left + v.widths.into_iter().sum::<i64>();
  let (x, _, width, _) = word_table_device_rectangle(page, (left, 0, right, 1));
  for item in &mut page.items[body] {
    let PageItem::Fill(fill) = item else { return };
    let end = fill.x_pt + fill.width_pt;
    if leading {
      fill.x_pt = fill.x_pt.max(x + width);
      fill.width_pt = (end - fill.x_pt).max(0.0)
    } else {
      fill.width_pt = (end.min(x) - fill.x_pt).max(0.0)
    }
  }
}

pub(super) fn cap_opaque_stroked_join(
  page: &mut Page,
  body: std::ops::Range<usize>,
  grid_x: f32,
  horizontal: BorderStyle,
  junction: HorizontalTableBorderJunction,
  leading: bool,
) {
  if super::super::border_3d::band_widths(horizontal).is_none() {
    return;
  }
  let vertical = match (junction.above, junction.below) {
    (Some(v), None) | (None, Some(v))
      if !v.compound && v.dash_pattern == BorderDashPattern::DashDotStroked =>
    {
      v
    }
    _ => return,
  };
  let width = ((f64::from(vertical.width_pt) * 20.0).floor() as i64 * 5 / 12).max(1);
  let left = wordprocessing_table_device::source_coordinate(grid_x) - (width + 1) / 2;
  let (x, _, extent, _) = word_table_device_rectangle(page, (left, 0, left + width, 1));
  let mut caps = smallvec::SmallVec::<[FillItem; 3]>::new();
  for item in &mut page.items[body] {
    let PageItem::Fill(fill) = item else { return };
    caps.push(FillItem {
      x_pt: x,
      y_pt: fill.y_pt,
      width_pt: extent,
      height_pt: fill.height_pt,
      paint: fill.paint,
    });
    let end = fill.x_pt + fill.width_pt;
    if leading {
      fill.x_pt = fill.x_pt.max(x + extent);
      fill.width_pt = (end - fill.x_pt).max(0.0)
    } else {
      fill.width_pt = (end.min(x) - fill.x_pt).max(0.0)
    }
  }
  page.items.extend(caps.into_iter().map(PageItem::Fill));
}

pub(super) fn join_double_half(
  page: &mut Page,
  body: std::ops::Range<usize>,
  (grid_origin, grid_x): (f32, f32),
  horizontal: BorderStyle,
  junction: HorizontalTableBorderJunction,
  leading: bool,
) -> bool {
  let double = |border: BorderStyle| {
    border.compound
      && matches!(
        border.compound_pattern,
        BorderCompoundPattern::Equal | BorderCompoundPattern::Double
      )
      && border.dash_pattern == BorderDashPattern::Solid
      && border.relief.is_none()
      && !border.shadow
  };
  if !double(horizontal) {
    return false;
  }
  let (Some(above), Some(below)) = (junction.above, junction.below) else {
    return false;
  };
  let Some(vertical) = stronger_border(Some(above), Some(below)).filter(|&border| double(border))
  else {
    return false;
  };
  let Some(rails) = word_table_compound_rails(vertical) else {
    return false;
  };
  if word_table_compound_rails(horizontal) != Some(rails) {
    return false;
  }
  if above != below
    && ![above, below]
      .into_iter()
      .any(|border| border.dash_pattern == BorderDashPattern::DashDotStroked)
  {
    return false;
  }
  let rows = match &page.items[body.clone()] {
    [PageItem::Fill(a), PageItem::Fill(b)] => [(a.y_pt, a.height_pt), (b.y_pt, b.height_pt)],
    _ => return false,
  };
  // Connected horizontal edges retain the row origin and local whole twips
  // independently, including at a mixed stroked/double junction.
  let grid = wordprocessing_table_device::source_coordinate(grid_origin)
    + wordprocessing_table_device::source_coordinate_precise(
      ((f64::from(grid_x) - f64::from(grid_origin)) * 20.0).round() / 20.0,
    );
  let layout_width = (f64::from(vertical.width_pt) * 20.0).floor() as i64 * 5 / 12;
  let left = grid - (layout_width + 1) / 2;
  let right = left + rails.into_iter().sum::<i64>();
  let (outer_x, _, extent, _) = word_table_device_rectangle(page, (left, 0, right, 1));
  for item in &mut page.items[body] {
    let PageItem::Fill(fill) = item else {
      unreachable!("checked horizontal strips")
    };
    let end = fill.x_pt + fill.width_pt;
    if leading {
      fill.x_pt = fill.x_pt.max(outer_x + extent);
      fill.width_pt = (end - fill.x_pt).max(0.0);
    } else {
      fill.width_pt = (end.min(outer_x) - fill.x_pt).max(0.0);
    }
  }
  if above == below {
    // Each horizontal cell owns its facing vertical rail. Their clear gaps
    // stay clear, and adjacent cells do not overpaint the same antialiased
    // corner. A mixed stroked rule retains its independently sampled patch.
    let (start, end) = if leading {
      (right - rails[2], right)
    } else {
      (left, left + rails[0])
    };
    let (x, _, width, _) = word_table_device_rectangle(page, (start, 0, end, 1));
    for (y, height) in rows {
      page.items.push(PageItem::Fill(FillItem {
        x_pt: x,
        y_pt: y,
        width_pt: width,
        height_pt: height,
        paint: ShadingPaint::Solid(horizontal.color),
      }));
    }
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn native_engraved_horizontal_body_and_stroked_cap_have_separate_extents() {
    let h = crate::docx::table_border_style(
      w::BorderValues::ThreeDEngrave,
      Some(48),
      None,
      Some("33CCCC"),
      None,
    )
    .unwrap();
    let v = crate::docx::table_border_style(
      w::BorderValues::DashDotStroked,
      Some(24),
      None,
      Some("000000"),
      None,
    )
    .unwrap();
    let mut page = empty_page(
      PageSetup {
        width_pt: 419.75,
        height_pt: 297.35,
        ..Default::default()
      },
      0,
    );
    assert!(push_word_table_compound_border(
      &mut page,
      (51.3, 61.2, 109.8, 61.2),
      h
    ));
    trim_outer_opaque_body(&mut page, 0..3, 51.3, h, h, true);
    cap_opaque_stroked_join(
      &mut page,
      0..3,
      109.8,
      h,
      HorizontalTableBorderJunction::horizontal_outer(None, Some(v)),
      false,
    );
    assert_eq!(page.items.len(), 6);
    for (i, item) in page.items.iter().enumerate() {
      let PageItem::Fill(fill) = item else {
        panic!("filled native bands")
      };
      let expected = if i < 3 {
        [55.704, 108.264]
      } else {
        [108.26, 111.26]
      };
      for (actual, expected) in [fill.x_pt, fill.x_pt + fill.width_pt]
        .into_iter()
        .zip(expected)
      {
        assert!((actual - expected).abs() < 0.01, "{actual} vs {expected}");
      }
    }
  }

  #[test]
  fn native_double_crossing_keeps_four_purple_tiles_and_clear_middle() {
    let border = |color| {
      crate::docx::table_border_style(w::BorderValues::Double, Some(10), None, Some(color), None)
        .unwrap()
    };
    let h = border("800080");
    let v = border("008000");
    let junction = HorizontalTableBorderJunction::internal(Some(v), Some(v));
    let mut page = empty_page(
      PageSetup {
        width_pt: 419.75,
        height_pt: 297.35,
        ..Default::default()
      },
      0,
    );
    for (left, right, leading) in [(109.8, 171.35, false), (171.35, 233.7, true)] {
      let start = page.items.len();
      assert!(push_word_table_compound_border(
        &mut page,
        (left, 122.208786, right, 122.208786),
        h
      ));
      let end = page.items.len();
      assert!(join_double_half(
        &mut page,
        start..end,
        (51.3, 171.35),
        h,
        junction,
        leading
      ));
    }
    let mut corners = Vec::new();
    for item in &page.items {
      let PageItem::Fill(fill) = item else {
        panic!("filled native strips")
      };
      if fill.x_pt >= 169.4 && fill.x_pt + fill.width_pt <= 173.2 {
        corners.push([
          fill.x_pt,
          fill.y_pt,
          fill.x_pt + fill.width_pt,
          fill.y_pt + fill.height_pt,
        ]);
      }
      assert!(
        !(fill.x_pt < 171.3
          && fill.x_pt + fill.width_pt > 171.3
          && fill.y_pt <= 121.0
          && fill.y_pt + fill.height_pt > 121.0),
        "native middle is clear"
      );
    }
    let expected = [
      [169.49, 120.41, 170.69, 121.61],
      [169.49, 122.81, 170.69, 124.01],
      [171.89, 120.41, 173.09, 121.61],
      [171.89, 122.81, 173.09, 124.01],
    ];
    assert_eq!(corners.len(), 4);
    for (actual, expected) in corners.into_iter().zip(expected) {
      for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.01, "{actual} vs {expected}");
      }
    }
  }

  #[test]
  fn native_upper_engraved_corner_keeps_all_six_filled_faces() {
    let border = crate::docx::table_border_style(
      w::BorderValues::ThreeDEngrave,
      Some(48),
      None,
      Some("33CCCC"),
      None,
    )
    .unwrap();
    let mut page = empty_page(
      PageSetup {
        width_pt: 419.75,
        height_pt: 297.35,
        ..Default::default()
      },
      0,
    );
    assert!(paint(&mut page, (51.3, 61.2), border, border, true, true));
    // Saved Office PDF's actual six corner rectangles, normalized to the
    // top-left page plane. Printed decimal endpoints carry <=.006pt rounding.
    let expected = [
      [46.824, 56.66, 48.264, 65.66],
      [46.824, 56.66, 55.704, 58.10],
      [48.264, 58.10, 54.264, 65.66],
      [48.264, 58.10, 55.704, 64.10],
      [54.264, 64.10, 55.704, 65.66],
      [54.264, 64.10, 55.704, 65.54],
    ];
    assert_eq!(page.items.len(), expected.len());
    for (item, expected) in page.items.iter().zip(expected) {
      let PageItem::Fill(fill) = item else {
        panic!("filled native face")
      };
      let actual = [
        fill.x_pt,
        fill.y_pt,
        fill.x_pt + fill.width_pt,
        fill.y_pt + fill.height_pt,
      ];
      for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.01, "{actual} vs {expected}");
      }
    }
  }
}
