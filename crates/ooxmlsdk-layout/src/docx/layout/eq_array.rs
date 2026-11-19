//! Legacy EQ arrays own their row cells and one surrounding-line baseline.
//!
//! ECMA-376 Part4 §14.10.4.6 defines row-major cells, column alignment and
//! inter-cell gaps. Native font/spacing/mixed-line controls distinguish the
//! logical inline box from its GDI printer rows: paragraph proportional
//! spacing adds one font-line gap after the box, never between its rows.

use smallvec::SmallVec;

use super::*;
use crate::docx::EqArrayInline;

#[derive(Clone, Copy, Debug, Default)]
struct Row {
  ascent: f32,
  descent: f32,
  printer_ascent: f32,
  printer_descent: f32,
}

impl Row {
  fn height(self) -> f32 {
    self.ascent + self.descent
  }

  fn printer_height(self) -> f32 {
    self.printer_ascent + self.printer_descent
  }

  fn include(&mut self, other: Self) {
    self.ascent = self.ascent.max(other.ascent);
    self.descent = self.descent.max(other.descent);
    self.printer_ascent = self.printer_ascent.max(other.printer_ascent);
    self.printer_descent = self.printer_descent.max(other.printer_descent);
  }
}

fn font_row(text: &str, style: &TextStyle, text_metrics: &mut TextMetrics) -> Row {
  let font = text_metrics.line_vertical_metrics_for_text(text, style);
  let ascent = font.directwrite_baseline_offset_pt;
  let descent = (font.line_height_pt() - ascent).max(0.0);
  let mut physical = style.clone();
  crate::docx::quantize_word_fixed_output_text_style(&mut physical);
  let (printer_ascent, printer_descent) = text_metrics
    .gdi_printer_line_metrics(text, &physical, 600.0)
    .unwrap_or_else(|| {
      (
        units::quantize_points_to_office_print_grid(ascent),
        units::quantize_points_to_office_print_grid(descent),
      )
    });
  Row {
    ascent: (ascent + style.baseline_shift_pt).max(0.0),
    descent: (descent - style.baseline_shift_pt).max(0.0),
    printer_ascent: (printer_ascent + style.baseline_shift_pt).max(0.0),
    printer_descent: (printer_descent - style.baseline_shift_pt).max(0.0),
  }
}

#[derive(Debug)]
struct ArrayLayout {
  columns: SmallVec<[f32; 4]>,
  rows: SmallVec<[Row; 4]>,
  ascent: f32,
  height: f32,
  printer_ascent: f32,
  font_height: f32,
}

fn layout(array: &EqArrayInline, text_metrics: &mut TextMetrics) -> ArrayLayout {
  let reference = font_row("A", &array.style, text_metrics);
  let mut columns: SmallVec<[f32; 4]> = SmallVec::from_elem(0.0, array.columns);
  let mut rows: SmallVec<[Row; 4]> =
    SmallVec::from_elem(Row::default(), array.cells.len().div_ceil(array.columns));
  for (index, cell) in array.cells.iter().enumerate() {
    columns[index % array.columns] =
      columns[index % array.columns].max(text_runs_width(cell, text_metrics));
    let row = &mut rows[index / array.columns];
    for run in cell {
      row.include(font_row(&run.text, &run.style, text_metrics));
    }
  }
  for row in &mut rows {
    if row.height() == 0.0 {
      *row = reference;
    }
  }
  let gaps = rows.len().saturating_sub(1) as f32;
  let height = rows.iter().map(|row| row.height()).sum::<f32>() + gaps * array.vertical_gap_pt;
  let printer_height = rows.iter().map(|row| row.printer_height()).sum::<f32>()
    + gaps * units::quantize_points_to_office_print_grid(array.vertical_gap_pt);
  ArrayLayout {
    columns,
    font_height: rows
      .iter()
      .fold(reference.height(), |height, row| height.max(row.height())),
    rows,
    // The invisible character box centres the array around half the
    // carrier's ascent. Its logical height does not inherit printer hinting.
    ascent: (height + reference.ascent) / 2.0,
    height,
    printer_ascent: (printer_height + reference.printer_ascent) / 2.0,
  }
}

pub(super) fn metrics(
  array: &EqArrayInline,
  text_metrics: &mut TextMetrics,
) -> CompoundTextMetrics {
  let layout = layout(array, text_metrics);
  let descent = (layout.height - layout.ascent).max(0.0);
  let extents = WordLineTextExtents {
    ascent_pt: layout.ascent,
    descent_pt: descent,
    font_ascent_pt: layout.ascent,
    font_descent_pt: descent,
    centered_font_ascent_pt: layout.ascent,
    line_spacing_height_pt: layout.font_height,
    has_spacing_excluded_portion: true,
    has_legacy_equation: true,
    has_text: true,
    ..Default::default()
  };
  CompoundTextMetrics {
    width_pt: layout.columns.iter().sum::<f32>() + array.columns as f32 * array.horizontal_gap_pt,
    height_pt: extents.height_pt(),
    line_extents: Some(extents),
  }
}

pub(super) fn push_items(
  page: &mut Page,
  paragraph: &crate::docx::Paragraph,
  array: &EqArrayInline,
  placement: TextPlacement,
  segmentation: TextSegmentation,
  text_metrics: &mut TextMetrics,
) {
  let layout = layout(array, text_metrics);
  let mut row_top = 0.0;
  for (index, row) in layout.rows.iter().enumerate() {
    let printer_gap = units::quantize_points_to_office_print_grid(array.horizontal_gap_pt);
    // Word's native hs0/3/6 controls retain half an outer column gap.
    let mut x_pt = placement.x_pt + (printer_gap / 0.12 / 2.0).floor() * 0.12;
    let row_baseline = row_top + row.printer_ascent;
    for (column, width) in layout.columns.iter().enumerate() {
      if let Some(cell) = array.cells.get(index * array.columns + column) {
        push_ruby_horizontal_line(
          page,
          paragraph,
          cell,
          RubyLineLayout {
            alignment: array.alignment,
            target_width: *width,
            baseline_shift_pt: layout.printer_ascent - row_baseline,
            segmentation,
            placement: TextPlacement { x_pt, ..placement },
          },
          text_metrics,
        );
      }
      x_pt += width + printer_gap;
    }
    row_top +=
      row.printer_height() + units::quantize_points_to_office_print_grid(array.vertical_gap_pt);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn style(family: &str, size: f32) -> TextStyle {
    TextStyle {
      font_family: Some(family.into()),
      font_size_pt: size,
      wordprocessingml_font_slots: true,
      use_windows_font_metrics: true,
      ..Default::default()
    }
  }

  #[test]
  fn array_printer_rows_match_native_gdi_cells_including_vdmx_and_external_leading() {
    let mut text_metrics = TextMetrics::new();
    // Actual GetTextMetricsW at600DPI, independently matched to Word EMF DX/DY.
    for (family, size, ascent, descent) in [
      ("Calibri", 8.0, 64.0, 18.0),
      ("Calibri", 11.0, 88.0, 25.0),
      ("Calibri", 20.0, 159.0, 45.0),
      ("Times New Roman", 11.0, 86.0, 22.0),
      ("Cambria", 11.0, 87.0, 20.0),
    ] {
      let row = font_row("Milk", &style(family, size), &mut text_metrics);
      assert!(
        (row.printer_ascent / 0.12 - ascent).abs() < 0.001,
        "{family}/{size}: {row:?}"
      );
      assert!(
        (row.printer_descent / 0.12 - descent).abs() < 0.001,
        "{family}/{size}: {row:?}"
      );
    }
  }

  #[test]
  fn array_logical_box_keeps_proportional_spacing_outside_its_rows() {
    let mut text_metrics = TextMetrics::new();
    let field = crate::docx::eq::array(
      r"EQ \a\al(Milk,Eggs,Detergent,Veggies)",
      &[],
      &style("Calibri", 11.0),
      None,
    )
    .unwrap();
    let actual = metrics(&field.portion, &mut text_metrics);
    let extents = actual.line_extents.unwrap();
    // Independent single/1.15/double following-paragraph controls: the
    // array consumes53.76pt; the gaps add1.92/13.32pt, not15%/100% of four rows.
    assert!((actual.height_pt - 53.76).abs() < 0.24);
    for (multiple, native_gap) in [(1.0, 0.0), (1.15, 1.92), (2.0, 13.32)] {
      assert!(
        (extents.height_with_line_spacing(multiple) - actual.height_pt - native_gap).abs() < 0.12
      );
    }
    let layout = layout(&field.portion, &mut text_metrics);
    assert!(!layout.columns.spilled() && !layout.rows.spilled());
    assert!((layout.rows[0].printer_height() - 13.56).abs() < 0.001);
  }

  #[test]
  fn array_mixed_operand_sizes_use_separate_row_ascent_and_descent() {
    let code = r"EQ \a\al(Milk,Eggs,Detergent,Veggies)";
    let base = style("Calibri", 11.0);
    let start = code.find("Detergent").unwrap();
    let field = crate::docx::eq::array(
      code,
      &[
        (0, base.clone()),
        (start, style("Calibri", 20.0)),
        (start + 9, base.clone()),
      ],
      &base,
      None,
    )
    .unwrap();
    let mut text_metrics = TextMetrics::new();
    let actual = layout(&field.portion, &mut text_metrics);
    // Native baselines:686,799,983,1116. The larger third operand changes
    // both adjacent pitches instead of applying one largest-font pitch globally.
    for (pair, native_dots) in actual.rows.windows(2).zip([113.0, 184.0, 133.0]) {
      let pitch = pair[0].printer_descent + pair[1].printer_ascent;
      assert!((pitch / 0.12 - native_dots).abs() < 0.001);
    }
  }
}
