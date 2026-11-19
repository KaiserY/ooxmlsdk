//! Preserve the cell-grid and row-frame owners of Word table rules.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Coordinates {
  logical_grid_left: f64,
  logical_grid_right: f64,
  body_x: i64,
  first_grid_twips: i64,
  leading_half_pixels: i64,
  dotted_first_row: bool,
  dotted_last_row: bool,
  lower_edge: bool,
  bitmap_story: bool,
}

fn border_twips(border: Option<BorderStyle>) -> i64 {
  border.map_or(0, |border| {
    (f64::from(border.width_pt) * 20.0).floor() as i64
  })
}

fn paint_width(border: Option<BorderStyle>) -> i64 {
  border.map_or(0, |border| {
    if border.width_pt > 0.0 {
      (border_twips(Some(border)) * 5 / 12).max(1)
    } else {
      0
    }
  })
}

fn simple(border: Option<BorderStyle>) -> bool {
  border.is_none_or(|border| {
    !border.compound && border.relief.is_none() && border.dash_pattern == BorderDashPattern::Solid
  })
}

fn simple_row(table: &Table, row: &TableRow) -> bool {
  row_table_borders(table, row).is_none_or(|borders| {
    [
      borders.top,
      borders.right,
      borders.bottom,
      borders.left,
      borders.inside_horizontal,
      borders.inside_vertical,
    ]
    .into_iter()
    .all(simple)
  }) && row.cells.iter().all(|cell| {
    [
      cell.borders.top,
      cell.borders.right,
      cell.borders.bottom,
      cell.borders.left,
    ]
    .into_iter()
    .all(simple)
  })
}

// Uniform dotted rules share the solid grid producer, but retain separate
// stroked cell strips and junction squares. Other pattern/compound producers
// and mixed-width junctions keep their existing ownership.
pub(super) fn dotted_row(table: &Table, row: &TableRow) -> bool {
  let Some(borders) = row_table_borders(table, row) else {
    return false;
  };
  let inherited = [
    borders.top,
    borders.right,
    borders.bottom,
    borders.left,
    borders.inside_horizontal,
    borders.inside_vertical,
  ];
  let Some(selected) = inherited.into_iter().flatten().next() else {
    return false;
  };
  if selected.compound
    || selected.relief.is_some()
    || selected.dash_pattern != BorderDashPattern::Dotted
    || selected.width_pt <= 0.0
  {
    return false;
  }
  inherited
    .into_iter()
    .all(|border| border.is_none_or(|border| border == selected))
    && row.cells.iter().all(|cell| {
      [
        cell.borders.top,
        cell.borders.right,
        cell.borders.bottom,
        cell.borders.left,
      ]
      .into_iter()
      .all(|border| border.is_none_or(|border| border == selected))
    })
}

impl Coordinates {
  pub(super) fn for_row(table: &Table, frame: &TableFrame, row: &TableRow) -> Option<Self> {
    Self::for_owned_row(table, frame, row, false)
  }

  pub(super) fn for_vertical_row(
    table: &Table,
    frame: &TableFrame,
    row: &TableRow,
  ) -> Option<Self> {
    Self::for_owned_row(table, frame, row, table.wordprocessing_shape_paint_grid)
  }

  fn for_owned_row(
    table: &Table,
    frame: &TableFrame,
    row: &TableRow,
    bitmap_story: bool,
  ) -> Option<Self> {
    let dotted = dotted_row(table, row);
    if bitmap_story && !simple_row(table, row) {
      return None;
    }
    // Cell-authored and inherited table edges share the same grid producer
    // (native table-grid controls, two widths across eight frame phases).
    // Legacy content alignment changes the grid origin, not the device
    // painter. Paired inline/floating Native controls retain that same grid
    // for legacy top-level flies. Their overflow flag changes width budgeting,
    // not the rule producer. Other floating/nested stories stay independent.
    let legacy_floating_grid = table.placement.is_some()
      && table.align_leading_cell_content
      && frame.block.compatibility_mode < 15
      && table.containing_table_layout.is_none()
      && simple_row(table, row)
      && wordprocessing_table_device::page_coordinates(
        frame.block.setup.width_pt,
        frame.block.setup.height_pt,
      )
      .is_some();
    // Modern percentage widths resolve the column grid without selecting
    // another rule painter. Native width-type, pen-width and margin-phase
    // controls retain the same device origins at 80% and 100%. Legacy
    // percentage frames have an independently positioned grid origin; keep
    // that producer, nested/overflow stories and non-solid rules separate.
    let inline_percentage_grid = frame.full_width_horizontal_borders
      && frame.block.compatibility_mode >= 15
      && !table.align_leading_cell_content
      && table.placement.is_none()
      && table.containing_table_layout.is_none()
      && table.preferred_width_pct.is_some()
      && simple_row(table, row);
    if (table.placement.is_some() && !legacy_floating_grid)
      || table.following_text_flow
      || table.right_to_left
      || !matches!(table.alignment, TableAlignment::Left | TableAlignment::Center)
      || table.in_header_footer
      || (frame.direct_text_frame_story_owner && !bitmap_story)
      || (frame.full_width_horizontal_borders
        && !legacy_floating_grid
        && !inline_percentage_grid
        && !dotted)
      // Margin-consuming rows partition their print frame before drawing
      // the rule (native table-indent controls). Their vertical parent is
      // not the unpartitioned grid supplied to this primitive.
      || inline_collapsed_row_has_vertical_margin_overlap(table, row)
      || row_has_separate_borders(table, row)
      || row.grid_before != 0
      || row.grid_after != 0
    {
      return None;
    }
    let first_row = table.rows.first()?;
    if first_row.grid_before != 0 || first_row.grid_after != 0 {
      return None;
    }
    let first = vertical_border(table, first_row, 0, true);
    let leading = vertical_border(table, row, 0, true);
    if (!simple(first) || !simple(leading)) && !dotted {
      return None;
    }
    // A legacy left-aligned top-level grid already aligns the first cell's print edge.
    // Its positioned centre converts independently of the row pen. Modern
    // frames instead include the first physical row's leading border half.
    // Native mode12/omitted-mode controls cover three widths and four phases.
    // Centered tables align the outside frame instead, including in legacy
    // modes. Native inline/nested controls at .5/1.5pt retain the first
    // border half; justification changes the frame, not the rule producer.
    let content_aligned =
      table.align_leading_cell_content && table.alignment == TableAlignment::Left;
    let first_half = if content_aligned {
      0.0
    } else {
      first.map_or(0.0, |border| f64::from(border.width_pt) / 2.0)
    };
    Some(Self {
      logical_grid_left: f64::from(frame.row_grid_origin(row)),
      logical_grid_right: f64::from(frame.row_grid_end(row)),
      body_x: wordprocessing_table_device::source_coordinate_precise(
        f64::from(frame.row_grid_origin(row)) - first_half,
      ),
      // Native cell coordinates contain the first physical row's leading
      // half in whole twips, including the odd twip. Later rows retain that
      // grid, even when their own leading rule differs or the first is nil.
      first_grid_twips: if content_aligned {
        0
      } else {
        (border_twips(first) + 1) / 2
      },
      leading_half_pixels: (paint_width(leading) + 1) / 2,
      dotted_first_row: table
        .rows
        .first()
        .is_some_and(|first| std::ptr::eq(first, row)),
      dotted_last_row: table
        .rows
        .last()
        .is_some_and(|last| std::ptr::eq(last, row)),
      lower_edge: false,
      bitmap_story,
    })
  }

  fn grid_x(self, logical_x: f32) -> i64 {
    let local_twips = ((f64::from(logical_x) - self.logical_grid_left) * 20.0).round() as i64;
    let numerator = (self.first_grid_twips + local_twips) * 600;
    let local = (numerator + numerator.signum() * 720) / 1440;
    self.body_x - self.leading_half_pixels + local
  }

  pub(super) fn lower_edge(mut self) -> Self {
    self.lower_edge = true;
    self
  }

  pub(super) fn paint(
    self,
    page: &mut Page,
    bounds: (f32, f32, f32, f32),
    border: BorderStyle,
  ) -> bool {
    let dotted = !border.compound && border.dash_pattern == BorderDashPattern::Dotted;
    if (!simple(Some(border)) && !dotted) || border.width_pt <= 0.0 {
      return false;
    }
    let device =
      wordprocessing_table_device::page_coordinates(page.setup.width_pt, page.setup.height_pt);
    if !self.bitmap_story && device.is_none() {
      return false;
    }
    let (x1, y1, x2, y2) = bounds;
    let thickness = paint_width(Some(border));
    let horizontal = (y2 - y1).abs() <= (x2 - x1).abs();
    let rectangle = if horizontal {
      let left = self.grid_x(x1.min(x2));
      let right = self.grid_x(x1.max(x2)) + thickness;
      let top = wordprocessing_table_device::source_coordinate_precise(
        f64::from(y1) - f64::from(border.width_pt) / 2.0,
      );
      (left, top, right - left, thickness)
    } else {
      let top = wordprocessing_table_device::source_coordinate(y1.min(y2));
      let bottom = wordprocessing_table_device::source_coordinate(y1.max(y2));
      let left = if self.bitmap_story {
        // The bitmap story resolves each vertical pen around its own grid
        // coordinate. Page EMF replay instead retains the row's leading half.
        self.grid_x(x1) + self.leading_half_pixels - thickness / 2
      } else {
        // Each physical rule is centred on its own grid edge. The row's
        // leading pen half only establishes the shared grid origin; reusing
        // it for a thinner/thicker internal strip shifts that entire face.
        self.grid_x(x1) + self.leading_half_pixels - (thickness + 1) / 2
      };
      (left, top, thickness, bottom - top)
    };
    if dotted {
      let Some(device) = device else {
        return false;
      };
      let (left, top, width, height) = rectangle;
      // Native pens use the integral lower half for odd source widths. The
      // PDF writer's minimum stroked width is .14pt, independently of the
      // one-dot grid reservation. Dash and gap both use that painted width.
      let pen = (thickness as f32 * 0.12).max(0.14);
      let mut stroke = |from: (i64, i64), to: (i64, i64)| {
        push_line_item(
          page,
          device.map_device_point(from.0, from.1),
          device.map_device_point(to.0, to.1),
          pen,
          border.color,
          BorderDashPattern::Dotted,
        );
      };
      if horizontal {
        let center = top + thickness / 2;
        let right = left + width - thickness;
        stroke((left + thickness, center), (right, center));
        for (x, outer) in [
          (left, x1.min(x2) == self.logical_grid_left as f32),
          (right, x1.max(x2) == self.logical_grid_right as f32),
        ] {
          // Internal upper junctions are horizontal; lower junctions are
          // vertical until the final row. Outside upper corners retain the
          // vertical side's square. Cell strips restart their dash phase.
          let vertical = if self.lower_edge {
            !self.dotted_last_row
          } else {
            outer
          };
          let (from, to) = if vertical {
            (
              (x + thickness / 2, top),
              (x + thickness / 2, top + thickness),
            )
          } else {
            ((x, center), (x + thickness, center))
          };
          // The next cell owns a shared junction; only the table's outside
          // trailing corner is emitted by this cell.
          if x == left || outer {
            stroke(from, to);
            if outer
              && (self.lower_edge && self.dotted_last_row
                || !self.lower_edge && self.dotted_first_row)
            {
              stroke(from, to);
            }
          }
        }
      } else {
        stroke(
          (left + thickness / 2, top),
          (left + thickness / 2, top + height),
        );
      }
      return true;
    }
    let (x_pt, y_pt, width_pt, height_pt) = if self.bitmap_story {
      // WPG/WPC rules are already on the shape's 600-DPI source plane; they
      // do not pass through the page EMF's separate 3000-DPI PDF replay grid.
      let point = |dots: i64| dots as f32 * units::POINTS_PER_INCH / WORD_FIXED_OUTPUT_DPI;
      (
        point(rectangle.0),
        point(rectangle.1),
        point(rectangle.2),
        point(rectangle.3),
      )
    } else {
      let Some(device) = device else {
        return false;
      };
      device.map_device_rectangle(rectangle)
    };
    page.items.push(PageItem::Fill(FillItem {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
      paint: ShadingPaint::Solid(border.color),
    }));
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unequal_vertical_rules_keep_their_own_pen_half_on_a_shared_grid() {
    let coordinates = Coordinates {
      logical_grid_left: 83.825,
      logical_grid_right: 163.725,
      body_x: 699,
      first_grid_twips: 0,
      leading_half_pixels: 9,
      dotted_first_row: true,
      dotted_last_row: true,
      lower_edge: false,
      bitmap_story: false,
    };
    let mut centres = Vec::new();
    for width in [0.75, 2.25, 3.0] {
      let mut page = empty_page(PageSetup::default(), 0);
      assert!(coordinates.paint(
        &mut page,
        (136.375, 100.0, 136.375, 114.0),
        BorderStyle {
          width_pt: width,
          ..Default::default()
        }
      ));
      let [PageItem::Fill(fill)] = page.items.as_slice() else {
        panic!("one opaque brush")
      };
      centres.push(fill.x_pt + fill.width_pt / 2.0);
    }
    // Native changing-width controls keep a common physical grid. An odd
    // printer pen has its lower centre half, at most half a600DPI source dot.
    assert!((centres[0] - centres[1]).abs() < 0.0241);
    assert!((centres[0] - centres[2]).abs() <= 0.0721);
  }

  #[test]
  fn dotted_cell_strips_and_corners_match_native_device_paths() {
    // Independent Word two-row/two-column control: dotted sz2, mode12.
    // Complete path bounds include separately stroked junction squares.
    let mut page = empty_page(PageSetup::default(), 0);
    let border = BorderStyle {
      width_pt: 0.25,
      dash_pattern: BorderDashPattern::Dotted,
      ..BorderStyle::default()
    };
    let mut coordinates = Coordinates {
      logical_grid_left: f64::from(79.65_f32),
      logical_grid_right: f64::from(379.65_f32),
      body_x: 664,
      first_grid_twips: 0,
      leading_half_pixels: 1,
      dotted_first_row: true,
      dotted_last_row: false,
      lower_edge: false,
      bitmap_story: false,
    };
    for (row, top, bottom) in [(0, 221.65, 231.65), (1, 231.65, 241.7)] {
      coordinates.dotted_first_row = row == 0;
      coordinates.dotted_last_row = row == 1;
      for [left, right] in [[79.65, 196.4], [196.4, 379.65]] {
        if row == 0 {
          assert!(coordinates.paint(&mut page, (left, top + 0.125, right, top + 0.125), border));
        }
        assert!(coordinates.lower_edge().paint(
          &mut page,
          (left, bottom + 0.125, right, bottom + 0.125),
          border
        ));
      }
      for x in [79.65, 196.4, 379.65] {
        assert!(coordinates.paint(&mut page, (x, top + 0.25, x, bottom), border));
      }
    }
    let mut actual = page
      .items
      .iter()
      .map(|item| {
        let PageItem::Line(line) = item else {
          panic!("native dotted stroke")
        };
        assert_eq!(line.width_pt, 0.24);
        assert_eq!(line.dash_pattern, BorderDashPattern::Dotted);
        [
          line.x1_pt.min(line.x2_pt),
          line.y1_pt.min(line.y2_pt),
          line.x1_pt.max(line.x2_pt),
          line.y1_pt.max(line.y2_pt),
        ]
      })
      .collect::<Vec<_>>();
    let mut expected = vec![
      [79.704, 221.66, 79.704, 221.9],
      [79.704, 221.66, 79.704, 221.9],
      [79.824, 221.78, 196.37, 221.78],
      [196.37, 221.78, 196.61, 221.78],
      [196.61, 221.78, 379.63, 221.78],
      [379.75, 221.66, 379.75, 221.9],
      [379.75, 221.66, 379.75, 221.9],
      [79.704, 221.9, 79.704, 231.62],
      [196.49, 221.9, 196.49, 231.62],
      [379.75, 221.9, 379.75, 231.62],
      [79.704, 231.62, 79.704, 231.86],
      [79.824, 231.74, 196.37, 231.74],
      [196.49, 231.62, 196.49, 231.86],
      [196.61, 231.74, 379.63, 231.74],
      [379.75, 231.62, 379.75, 231.86],
      [79.704, 231.86, 79.704, 241.7],
      [79.584, 241.82, 79.824, 241.82],
      [79.584, 241.82, 79.824, 241.82],
      [79.824, 241.82, 196.37, 241.82],
      [196.49, 231.86, 196.49, 241.7],
      [196.37, 241.82, 196.61, 241.82],
      [196.61, 241.82, 379.63, 241.82],
      [379.75, 231.86, 379.75, 241.7],
      [379.63, 241.82, 379.87, 241.82],
      [379.63, 241.82, 379.87, 241.82],
    ];
    actual.sort_by(|a, b| a.partial_cmp(b).unwrap());
    expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(actual.len(), expected.len());
    for (actual, expected) in actual.into_iter().zip(expected) {
      for (actual, expected) in actual.into_iter().zip(expected) {
        assert!((actual - expected).abs() < 0.0052, "{actual} != {expected}");
      }
    }
  }

  #[test]
  fn native_first_row_grid_and_row_pen_have_independent_owners() {
    // 64 native controls, eight frame phases and eight border widths. Each
    // observes all four rails. Headerless first-row coordinates differ from
    // a later bordered row following a nil-bordered spanning first row.
    for (size, expected_width, expected_x) in [
      (1, 1, [471, 722, 1416, 6779]),
      (3, 2, [473, 723, 1418, 6780]),
      (4, 4, [472, 723, 1417, 6780]),
      (6, 6, [472, 723, 1417, 6780]),
      (12, 12, [472, 723, 1417, 6780]),
      (13, 13, [472, 722, 1417, 6779]),
      (18, 18, [473, 723, 1418, 6780]),
      (24, 25, [472, 722, 1417, 6779]),
    ] {
      let border = BorderStyle {
        width_pt: size as f32 / 8.0,
        ..BorderStyle::default()
      };
      assert_eq!(paint_width(Some(border)), expected_width);
      for (phase, delta) in [-1, -1, 0, 0, 0, 1, 1, 2].into_iter().enumerate() {
        let margin = (1130 + phase) as f32 / 20.0;
        let logical_grid_left = f64::from(margin + border.width_pt / 2.0);
        let coordinates = Coordinates {
          logical_grid_left,
          logical_grid_right: 0.0,
          body_x: wordprocessing_table_device::source_coordinate_precise(
            logical_grid_left - f64::from(border.width_pt) / 2.0,
          ),
          first_grid_twips: (border_twips(Some(border)) + 1) / 2,
          leading_half_pixels: (expected_width + 1) / 2,
          dotted_first_row: false,
          dotted_last_row: false,
          lower_edge: false,
          bitmap_story: false,
        };
        for (twips, expected) in [0, 601, 2268, 15138].into_iter().zip(expected_x) {
          let source = (logical_grid_left + twips as f64 / 20.0) as f32;
          assert_eq!(
            coordinates.grid_x(source),
            expected + delta,
            "size={size}, phase={phase}"
          );
        }
      }
    }
    let coordinates = Coordinates {
      logical_grid_left: f64::from(56.7_f32),
      logical_grid_right: 0.0,
      body_x: 472,
      first_grid_twips: 0,
      leading_half_pixels: 3,
      dotted_first_row: false,
      dotted_last_row: false,
      lower_edge: false,
      bitmap_story: false,
    };
    for (source, expected) in [(56.7, 469), (86.75, 719), (170.1, 1414), (813.6, 6777)] {
      assert_eq!(coordinates.grid_x(source), expected);
    }
  }

  #[test]
  fn grouped_table_vertical_rules_keep_native_source_grid_and_pen_owners() {
    // Native Screen and Print masks resolve these same five integer source
    // strips in both group members, independently of the page-frame phase.
    for (origin, body_x, image_left) in [(28.886614, 241, 153), (447.73383, 3731, 3643)] {
      let coordinates = Coordinates {
        logical_grid_left: f64::from(origin),
        logical_grid_right: f64::from(origin + 382.2),
        body_x,
        first_grid_twips: 0,
        leading_half_pixels: 0,
        dotted_first_row: true,
        dotted_last_row: false,
        lower_edge: false,
        bitmap_story: true,
      };
      let mut page = empty_page(PageSetup::default(), 0);
      for (twips, width) in [
        (1275, 0.5),
        (2548, 1.5),
        (3823, 0.5),
        (5097, 1.5),
        (6370, 0.5),
      ] {
        let x = origin + twips as f32 / units::TWIPS_PER_POINT;
        assert!(coordinates.paint(
          &mut page,
          (x, 200.0, x, 300.0),
          BorderStyle {
            width_pt: width,
            ..BorderStyle::default()
          },
        ));
      }
      for (item, expected) in page.items.iter().zip([617, 1144, 1679, 2206, 2740]) {
        let PageItem::Fill(fill) = item else {
          panic!("source-device rule");
        };
        let source_x = (f64::from(fill.x_pt) * 600.0 / 72.0).round() as i64;
        assert_eq!(source_x - image_left, expected);
      }
    }
  }
}
