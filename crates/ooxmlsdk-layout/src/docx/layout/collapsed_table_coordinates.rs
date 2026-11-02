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
    let dotted = dotted_row(table, row);
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
    if (table.placement.is_some() && !legacy_floating_grid)
      || table.following_text_flow
      || table.right_to_left
      || table.alignment != TableAlignment::Left
      || table.in_header_footer
      || frame.direct_text_frame_story_owner
      || (frame.full_width_horizontal_borders && !legacy_floating_grid && !dotted)
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
    // A legacy top-level grid already aligns the first cell's print edge.
    // Its positioned centre converts independently of the row pen. Modern
    // frames instead include the first physical row's leading border half.
    // Native mode12/omitted-mode controls cover three widths and four phases.
    let first_half = if table.align_leading_cell_content {
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
      first_grid_twips: if table.align_leading_cell_content {
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
    let Some(device) =
      wordprocessing_table_device::page_coordinates(page.setup.width_pt, page.setup.height_pt)
    else {
      return false;
    };
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
      (self.grid_x(x1), top, thickness, bottom - top)
    };
    if dotted {
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
    let (x_pt, y_pt, width_pt, height_pt) = device.map_device_rectangle(rectangle);
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
    };
    for (source, expected) in [(56.7, 469), (86.75, 719), (170.1, 1414), (813.6, 6777)] {
      assert_eq!(coordinates.grid_x(source), expected);
    }
  }
}
