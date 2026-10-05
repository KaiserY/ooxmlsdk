//! Preserve the cell-grid and row-frame owners of solid Word table rules.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct Coordinates {
  logical_grid_left: f64,
  body_x: i64,
  first_grid_twips: i64,
  leading_half_pixels: i64,
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

impl Coordinates {
  pub(super) fn for_row(table: &Table, frame: &TableFrame, row: &TableRow) -> Option<Self> {
    // Cell-authored and inherited table edges share the same grid producer
    // (native table-grid controls, two widths across eight frame phases).
    // Legacy print-edge alignment and floating/nested stories keep their
    // different frame producers.
    if table.placement.is_some()
      || table.following_text_flow
      || table.right_to_left
      || table.align_leading_cell_content
      || table.alignment != TableAlignment::Left
      || table.in_header_footer
      || frame.direct_text_frame_story_owner
      || frame.full_width_horizontal_borders
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
    if !simple(first) || !simple(leading) {
      return None;
    }
    let first_half = first.map_or(0.0, |border| f64::from(border.width_pt) / 2.0);
    Some(Self {
      logical_grid_left: f64::from(frame.left_pt),
      body_x: wordprocessing_table_device::source_coordinate_precise(
        f64::from(frame.left_pt) - first_half,
      ),
      // Native cell coordinates contain the first physical row's leading
      // half in whole twips, including the odd twip. Later rows retain that
      // grid, even when their own leading rule differs or the first is nil.
      first_grid_twips: (border_twips(first) + 1) / 2,
      leading_half_pixels: (paint_width(leading) + 1) / 2,
    })
  }

  fn grid_x(self, logical_x: f32) -> i64 {
    let local_twips = ((f64::from(logical_x) - self.logical_grid_left) * 20.0).round() as i64;
    let numerator = (self.first_grid_twips + local_twips) * 600;
    let local = (numerator + numerator.signum() * 720) / 1440;
    self.body_x - self.leading_half_pixels + local
  }

  pub(super) fn paint(
    self,
    page: &mut Page,
    bounds: (f32, f32, f32, f32),
    border: BorderStyle,
  ) -> bool {
    if !simple(Some(border)) || border.width_pt <= 0.0 {
      return false;
    }
    let Some(device) = wordprocessing_table_device::PageCoordinates::for_standard_size(
      page.setup.width_pt,
      page.setup.height_pt,
    ) else {
      return false;
    };
    let (x1, y1, x2, y2) = bounds;
    let thickness = paint_width(Some(border));
    let rectangle = if (y2 - y1).abs() <= (x2 - x1).abs() {
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
          body_x: wordprocessing_table_device::source_coordinate_precise(
            logical_grid_left - f64::from(border.width_pt) / 2.0,
          ),
          first_grid_twips: (border_twips(Some(border)) + 1) / 2,
          leading_half_pixels: (expected_width + 1) / 2,
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
      body_x: 472,
      first_grid_twips: 0,
      leading_half_pixels: 3,
    };
    for (source, expected) in [(56.7, 469), (86.75, 719), (170.1, 1414), (813.6, 6777)] {
      assert_eq!(coordinates.grid_x(source), expected);
    }
  }
}
