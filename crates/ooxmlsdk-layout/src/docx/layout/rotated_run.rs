//! Word's rotated inline text has a font cell, distinct from glyph rotation.
//!
//! Writer's SwRotatedPortion likewise swaps the horizontal font cell and text
//! advance and centres its ascent. Word's native mixed-line F/AB controls retain
//! the unbounded centred ascent (Writer subsequently clamps it). The 600-DPI
//! EMF text records establish the cell, baseline, and unchanged horizontal DX.

use super::{PageItem, TextMetrics, TextStyle, WordLineTextExtents};
use crate::text_metrics::TextVerticalMetrics;

#[derive(Clone, Copy, Debug)]
pub(super) struct Metrics {
  pub inline_width_pt: f32,
  source_ascent_pt: f32,
  advance_pt: f32,
  ascent_pt: f32,
}

impl Metrics {
  pub(super) fn line_metrics(self) -> TextVerticalMetrics {
    let descent_pt = (self.advance_pt - self.ascent_pt).max(0.0);
    TextVerticalMetrics {
      font_design_unit_pt: 0.0,
      ascent_pt: self.ascent_pt,
      descent_pt,
      windows_line_height_pt: self.ascent_pt + descent_pt,
      line_gap_pt: 0.0,
      baseline_offset_pt: self.ascent_pt,
      directwrite_baseline_offset_pt: self.ascent_pt,
      wordprocessingml_cjk_line_metrics: false,
    }
  }
}

pub(super) fn metrics(
  text: &str,
  style: &TextStyle,
  text_metrics: &mut TextMetrics,
) -> Option<Metrics> {
  if !style.wordprocessingml_vertical_run || text.is_empty() {
    return None;
  }
  let mut physical_style = style.clone();
  crate::docx::quantize_word_fixed_output_text_style(&mut physical_style);
  // Reuse the established GDI quarter-turn 1/64-em reserves without changing
  // the physical font cache or the glyph metrics used by the PDF writer.
  let font = text_metrics.rotated_cell_line_vertical_metrics(Some(text), &physical_style);
  let inline_width_pt = crate::units::quantize_points_to_office_print_grid(font.line_height_pt());
  let source_ascent_pt =
    crate::units::quantize_points_to_office_print_grid(font.directwrite_baseline_offset_pt);
  let advance_pt = text_metrics
    .gdi_printer_text_extents_pt(text, &physical_style, 600.0)
    .map_or_else(
      || text_metrics.measure_text(text, &physical_style),
      |extent| extent.positioned_width_pt,
    );
  Some(Metrics {
    inline_width_pt,
    source_ascent_pt,
    advance_pt,
    ascent_pt: source_ascent_pt + (advance_pt - inline_width_pt) / 2.0,
  })
}

pub(super) fn realize_line(
  items: &mut [PageItem],
  line: WordLineTextExtents,
  text_metrics: &mut TextMetrics,
) {
  if !line.has_rotated_run {
    return;
  }
  for item in items {
    let PageItem::Text(text) = item else {
      continue;
    };
    let Some(portion) = metrics(&text.text, &text.style, text_metrics) else {
      continue;
    };
    let baseline_pt = if text.origin_is_baseline {
      text.y_pt
    } else {
      // Like other inline objects, the turned cell shares Line Services'
      // Windows baseline. The line's typographic ascent can differ by the
      // hhea/Windows leading even when ordinary text paints at this baseline.
      text.y_pt + line.font_ascent_pt
    };
    text.x_pt += portion.source_ascent_pt;
    text.y_pt = baseline_pt + portion.advance_pt - portion.ascent_pt;
    text.origin_is_baseline = true;
    text.rotation_center_pt = Some((text.x_pt, text.y_pt));
    text.wordprocessing_line_metrics = None;
    text.style.wordprocessingml_vertical_run = false;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn east_asian_vertical_run_uses_native_font_cell_and_unchanged_advances() {
    let mut text_metrics = TextMetrics::new();
    // Actual 600-DPI Word EMF font cells and text DX, not PDF text-box bounds.
    for (family, size, text, ascent_dots, width_dots, advance_dots) in [
      ("Calibri", 8.0, "F", 65.0, 83.0, 31.0),
      ("Calibri", 11.0, "F", 89.0, 115.0, 42.0),
      ("Calibri", 11.0, "AB", 89.0, 115.0, 103.0),
      ("Calibri", 11.0, "ABCDEFG", 89.0, 115.0, 353.0),
      ("Calibri", 20.0, "F", 162.0, 209.0, 77.0),
      ("Times New Roman", 11.0, "F", 87.0, 108.0, 51.0),
      ("Times New Roman", 11.0, "AB", 87.0, 108.0, 127.0),
    ] {
      let style = TextStyle {
        font_family: Some(family.into()),
        font_size_pt: size,
        wordprocessingml_font_slots: true,
        wordprocessingml_vertical_run: true,
        rotation_deg: -90.0,
        ..Default::default()
      };
      let actual = metrics(text, &style, &mut text_metrics).unwrap();
      for (observed, native) in [
        (actual.source_ascent_pt, ascent_dots),
        (actual.inline_width_pt, width_dots),
        (actual.advance_pt, advance_dots),
      ] {
        assert!(
          (observed / 0.12 - native).abs() <= 1.01,
          "{family}/{size}/{text}: {actual:?}, expected {native} dots"
        );
      }
      // A short rotated portion can have its common ascent below the glyph's
      // complete vertical advance. Word retains it instead of clamping to DX.
      if text == "F" {
        assert!(actual.ascent_pt > actual.advance_pt);
      }
    }
  }

  #[test]
  fn east_asian_vertical_run_does_not_claim_whole_cell_rotation() {
    let mut text_metrics = TextMetrics::new();
    for rotation_deg in [-90.0, 0.0, 90.0] {
      let style = TextStyle {
        rotation_deg,
        ..Default::default()
      };
      assert!(metrics("F", &style, &mut text_metrics).is_none());
    }
  }
}
