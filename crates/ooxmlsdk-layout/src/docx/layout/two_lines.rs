//! Word two-lines-in-one boxes (ECMA-376 §17.3.2.10).
//!
//! Native Office matrices distinguish logical half-size advances from the
//! printer-realized outlines, legal balanced breaks, the paragraph's block
//! adjustment, and each sub-line's own printer cell. Reuse the ordinary font,
//! script-spacing and justification owners; do not scale painted page images.

use super::*;
use crate::docx::TwoLinesInline;

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
  ascent: f32,
  descent: f32,
}

impl Cell {
  fn include(&mut self, other: Self) {
    self.ascent = self.ascent.max(other.ascent);
    self.descent = self.descent.max(other.descent);
  }
}

fn printer_cell(runs: &[TextRun], metrics: &mut TextMetrics) -> Cell {
  let mut cell = Cell::default();
  for run in runs {
    let mut physical = run.style.clone();
    crate::docx::quantize_word_fixed_output_text_style(&mut physical);
    // Different source slots can select different physical CJK/Latin cells.
    // Combining their raw metrics before applying side leading fabricates a
    // taller font (the same distinction as ordinary Word line geometry).
    for segment in ooxmlsdk_fonts::script_direction_runs_with_options(
      &run.text,
      ooxmlsdk_fonts::FontSize(physical.font_size_pt),
      crate::fonts::script_scan_options(&physical, false),
    ) {
      let text = &run.text[segment.text_range];
      let style = materialize_wordprocessingml_source_font_slot(
        &physical,
        text.chars().next().unwrap_or(' '),
      );
      let raw = metrics.vertical_metrics_for_text(text, &style);
      let (mut ascent, mut descent) = metrics
        .gdi_printer_line_metrics(text, &style, 600.0)
        .unwrap_or((
          raw.directwrite_baseline_offset_pt,
          raw.line_height_pt() - raw.directwrite_baseline_offset_pt,
        ));
      if raw.wordprocessingml_cjk_line_metrics {
        let external = units::quantize_points_to_office_print_grid(
          (raw.directwrite_baseline_offset_pt - raw.baseline_offset_pt)
            .max(0.0)
            .min(raw.line_gap_pt),
        );
        let ink_height = (ascent + descent - external).max(0.0);
        // Native GDI+Office controls use the hinted printer cell here;
        // the ordinary unhinted/design-grid line owner remains unchanged.
        let side = units::quantize_points_to_office_print_grid(ink_height * 0.15);
        ascent += side - external;
        // Native4/6/8pt and independent paragraph-style struts establish
        // Word's two-point CJK descent minimum; Latin cells have no minimum.
        descent = (descent + side).max(units::quantize_points_to_office_print_grid(2.0));
      }
      cell.include(Cell {
        ascent: (ascent + style.baseline_shift_pt).max(0.0),
        descent: (descent - style.baseline_shift_pt).max(0.0),
      });
    }
  }
  cell
}

#[derive(Clone, Copy)]
struct Cluster {
  start: usize,
  end: usize,
  run: usize,
  character_end: usize,
  leading: f32,
  advance: f32,
}

struct Row {
  runs: Vec<TextRun>,
  width: f32,
  cell: Cell,
}

struct Layout {
  rows: Vec<Row>,
  width: f32,
  brackets: Option<(TextRun, TextRun)>,
  opening_width: f32,
}

fn miniature_runs(combined: &TwoLinesInline) -> Vec<TextRun> {
  let blank = combined
    .runs
    .iter()
    .all(|run| run.text.chars().all(char::is_whitespace));
  combined
    .runs
    .iter()
    .filter(|source| !source.style.hidden)
    .map(|source| {
      let mut run = source.clone();
      run.style.wordprocessingml_two_lines = None;
      run.style.font_size_pt *= 0.5;
      run.style.complex_font_size_pt = run.style.complex_font_size_pt.map(|size| size * 0.5);
      // Advances retain the logical half size. The existing fixed-output
      // adapter realizes outlines on the600DPI grid (15/2 becomes7.56pt).
      if blank
        && run
          .style
          .wordprocessingml_balance_single_byte_double_byte_width
      {
        run.style.wordprocessingml_balanced_space_portion = true;
      }
      run
    })
    // Keep authored source context while obtaining the prefix advances.
    // Logical reference-font spacing and CJK glyph linking retain their
    // separate owners; splitting these runs early can lose adjacent blanks.
    .collect()
}

fn clusters(
  runs: &[TextRun],
  combined: &TwoLinesInline,
  metrics: &mut TextMetrics,
) -> (String, Vec<Cluster>) {
  let mut text = String::new();
  let mut result = Vec::<Cluster>::new();
  let mut previous: Option<(char, usize)> = None;
  for (index, run) in runs.iter().enumerate() {
    let offset = text.len();
    text.push_str(&run.text);
    let mut advances = vec![0.0; run.text.len() + 1];
    if let Some(shape) = metrics.shape_text(&run.text, &run.style) {
      for glyph in shape.glyphs {
        advances[glyph.text_range.end] += glyph.x_advance_em * glyph.font_size_pt;
      }
    } else {
      for (start, character) in run.text.char_indices() {
        let end = start + character.len_utf8();
        advances[end] = metrics.measure_text(&run.text[start..end], &run.style);
      }
    }
    let boundaries: Vec<_> = GraphemeClusterSegmenter::new()
      .segment_str(&run.text)
      .collect();
    let mut character_end = 0;
    for pair in boundaries.windows(2) {
      let start = pair[0];
      let end = pair[1];
      let first = run.text[start..end].chars().next().unwrap_or(' ');
      character_end += run.text[start..end].chars().count();
      let mut leading = 0.0;
      if let Some((last, owner)) = previous {
        let a = auto_script_class(last, combined.auto_space_like_word95);
        let b = auto_script_class(first, combined.auto_space_like_word95);
        let other = if a == AutoScriptClass::EastAsian {
          b
        } else if b == AutoScriptClass::EastAsian {
          a
        } else {
          AutoScriptClass::Boundary
        };
        if (other == AutoScriptClass::Letter && combined.auto_space_de)
          || (other == AutoScriptClass::Number && combined.auto_space_dn)
        {
          leading = auto_script_edge(
            last,
            &runs[owner].style,
            combined.auto_space_like_word95,
            metrics,
          )
          .map_or(0.0, |edge| edge.normal_gap_pt);
        }
      }
      result.push(Cluster {
        start: offset + start,
        end: offset + end,
        run: index,
        character_end,
        leading,
        advance: advances[start + 1..=end].iter().sum(),
      });
      if let Some(last) = run.text[start..end].chars().rev().find(|&character| {
        auto_script_class(character, combined.auto_space_like_word95)
          != AutoScriptClass::Transparent
      }) {
        previous = Some((last, index));
      }
    }
  }
  (text, result)
}

fn balanced_break(
  text: &str,
  clusters: &[Cluster],
  options: WordprocessingLineBreakOptions,
) -> Option<usize> {
  let mut pieces = Vec::new();
  push_unicode_line_segments(text, options, &mut pieces);
  let mut eligible = vec![false; text.len() + 1];
  let mut end = 0;
  for piece in pieces {
    end += piece.len();
    eligible[end] = true;
  }
  // Native compatibility12/15 x space-balance ON/OFF controls distinguish
  // this break policy from the space's measured width. Only legacy layout
  // can cut inside a consecutive blank sequence; modern layout keeps the
  // Unicode word/blank segment together. Both retain trailing advances.
  if options.compatibility_mode < 15 {
    for (start, character) in text.char_indices() {
      if character == ' ' {
        eligible[start + 1] = true;
      }
    }
  }
  let total: f32 = clusters
    .iter()
    .map(|cluster| cluster.leading + cluster.advance)
    .sum();
  let mut prefix = 0.0;
  let mut best = None;
  for (index, cluster) in clusters.iter().enumerate() {
    prefix += cluster.leading + cluster.advance;
    if index + 1 == clusters.len() || !eligible[cluster.end] {
      continue;
    }
    let suffix = total - prefix - clusters[index + 1].leading;
    let difference = ((f64::from(prefix - suffix) * 4096.0).round() as i64).abs();
    if best.is_none_or(|(_, score)| difference <= score) {
      best = Some((cluster.end, difference));
    }
  }
  best.map(|(position, _)| position)
}

fn row(
  runs: &[TextRun],
  clusters: &[Cluster],
  range: std::ops::Range<usize>,
  metrics: &mut TextMetrics,
) -> Row {
  let mut result = Vec::<TextRun>::new();
  let mut expansions: Vec<Option<Vec<f32>>> = vec![None; runs.len()];
  // Each automatic gap belongs to the preceding source character. A single
  // pass retains whole-word shaping and avoids quadratic prefix reshaping or
  // a font lookup for every glyph. A gap crossing the selected break drops.
  for pair in clusters.windows(2) {
    let previous = pair[0];
    let next = pair[1];
    if previous.start >= range.start && next.start < range.end && next.leading != 0.0 {
      let expansion = expansions[previous.run].get_or_insert_with(|| {
        runs[previous.run]
          .style
          .wordprocessing_justification_expansion_pt
          .as_ref()
          .map_or_else(
            || vec![0.0; runs[previous.run].text.chars().count()],
            |values| values.to_vec(),
          )
      });
      expansion[previous.character_end - 1] += next.leading;
    }
  }
  let mut source_start = 0;
  for (index, source) in runs.iter().enumerate() {
    let source_end = source_start + source.text.len();
    let start = range.start.max(source_start);
    let end = range.end.min(source_end);
    if start < end {
      let mut run = source.clone();
      run.text = source.text[start - source_start..end - source_start].to_string();
      if let Some(additions) = expansions[index].as_deref().or(
        source
          .style
          .wordprocessing_justification_expansion_pt
          .as_deref(),
      ) {
        let begin = source.text[..start - source_start].chars().count();
        let count = run.text.chars().count();
        run.style.wordprocessing_justification_expansion_pt =
          Some(Arc::from(&additions[begin..begin + count]));
      }
      result.push(run);
    }
    source_start = source_end;
  }
  Row {
    width: text_runs_width(&result, metrics),
    cell: printer_cell(&result, metrics),
    runs: result,
  }
}

fn layout(combined: &TwoLinesInline, metrics: &mut TextMetrics) -> Layout {
  let runs = miniature_runs(combined);
  let (text, clusters) = clusters(&runs, combined, metrics);
  let split = balanced_break(
    &text,
    &clusters,
    WordprocessingLineBreakOptions {
      compatibility_mode: combined.compatibility_mode,
      compress_punctuation: runs
        .first()
        .is_some_and(|run| run.style.cjk_punctuation_compression_ratio > 0.0),
      strict_japanese: combined.strict_japanese,
      fullwidth_period_joins_han: runs
        .first()
        .is_none_or(|run| super::fullwidth_period_joins_han(&run.style)),
      kashida: combined.kashida,
    },
  );
  let ranges =
    std::iter::once(0..split.unwrap_or(text.len())).chain(split.map(|at| at..text.len()));
  let rows: Vec<_> = ranges
    .map(|range| row(&runs, &clusters, range, metrics))
    .collect();
  let content_width = rows.iter().fold(0.0_f32, |width, row| width.max(row.width));
  let pair = match combined.settings.brackets {
    w::CombineBracketValues::None => None,
    w::CombineBracketValues::Round => Some(('(', ')')),
    w::CombineBracketValues::Square => Some(('[', ']')),
    w::CombineBracketValues::Angle => Some(('<', '>')),
    w::CombineBracketValues::Curly => Some(('{', '}')),
  };
  let brackets = pair.and_then(|(a, b)| {
    combined.runs.first().map(|source| {
      let mut opening = source.clone();
      opening.text = a.to_string();
      opening.style.wordprocessingml_two_lines = None;
      let mut closing = opening.clone();
      closing.text = b.to_string();
      (opening, closing)
    })
  });
  let mut opening_width = 0.0;
  let mut closing_width = 0.0;
  if let Some((a, b)) = &brackets {
    opening_width = metrics.measure_text(&a.text, &a.style);
    closing_width = metrics.measure_text(&b.text, &b.style);
  }
  Layout {
    rows,
    width: opening_width + content_width + closing_width,
    brackets,
    opening_width,
  }
}

pub(super) fn metrics(
  combined: &TwoLinesInline,
  text_metrics: &mut TextMetrics,
) -> CompoundTextMetrics {
  let result = layout(combined, text_metrics);
  // Native paired ordinary/combined paragraphs retain the original font's
  // line cell, including when a2pt carrier/mark cannot supply that strut.
  // The independently hinted miniature cells position the two baselines;
  // charging their CJK descent minima here enlarges every following line.
  let mut extents = WordLineTextExtents::default();
  for run in combined.runs.iter().filter(|run| !run.style.hidden) {
    extents.include(
      TextPortion::from_run(run, &run.text),
      true,
      false,
      false,
      combined.compatibility_mode,
      text_metrics,
    );
  }
  CompoundTextMetrics {
    width_pt: result.width,
    height_pt: extents.height_pt(),
    line_extents: Some(extents),
  }
}

pub(super) fn push_items(
  page: &mut Page,
  paragraph: &crate::docx::Paragraph,
  combined: &TwoLinesInline,
  placement: TextPlacement,
  segmentation: TextSegmentation,
  metrics: &mut TextMetrics,
) {
  let result = layout(combined, metrics);
  let target_width = result
    .rows
    .iter()
    .fold(0.0_f32, |width, row| width.max(row.width));
  if let Some((opening, _)) = &result.brackets {
    push_ruby_text(
      page,
      paragraph,
      opening,
      opening.text.clone(),
      opening.style.clone(),
      placement,
      segmentation,
    );
  }
  for (index, row) in result.rows.iter().enumerate() {
    let start = page.items.len();
    let shift = if result.rows.len() == 1 {
      0.0
    } else if index == 0 {
      result.rows[1].cell.ascent
    } else {
      -result.rows[0].cell.descent
    };
    let row_placement = TextPlacement {
      x_pt: placement.x_pt + result.opening_width,
      ..placement
    };
    push_ruby_horizontal_line(
      page,
      paragraph,
      &row.runs,
      RubyLineLayout {
        alignment: RubyAlignment::Left,
        target_width: row.width,
        baseline_shift_pt: shift,
        segmentation,
        placement: row_placement,
      },
      metrics,
    );
    // Only the shorter row is adjusted (native trailing-blank controls;
    // SwDoubleLinePortion::GetLineDiff/CalcNewBlock). The longer row's
    // authored trailing spaces are part of its width, not spare glue.
    if combined.justify && row.width + LAYOUT_EPSILON_PT < target_width {
      justify_line_items(
        &mut page.items,
        LineJustificationContext {
          item_start: start,
          has_character_grid: false,
          y_pt: placement.y_pt,
          left_pt: row_placement.x_pt,
          right_pt: row_placement.x_pt + target_width,
          terminal_overhang_pt: 0.0,
          source_frame_right_pt: None,
          preserve_foreign_tail: false,
        },
        paragraph.format.justification,
        metrics,
      );
    }
  }
  if let Some((_, closing)) = &result.brackets {
    push_ruby_text(
      page,
      paragraph,
      closing,
      closing.text.clone(),
      closing.style.clone(),
      TextPlacement {
        x_pt: placement.x_pt + result.opening_width + target_width,
        ..placement
      },
      segmentation,
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn run(text: &str, family: &str, points: f32) -> TextRun {
    TextRun {
      text: text.into(),
      style: TextStyle {
        font_family: Some(family.into()),
        high_ansi_font_family: Some(family.into()),
        east_asia_font_family: Some(family.into()),
        font_size_pt: points,
        east_asia_language: Some("zh-CN".into()),
        kerning_minimum_size_pt: Some(500.0),
        wordprocessingml_font_slots: true,
        wordprocessingml_font_hint: Some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia),
        wordprocessingml_balance_single_byte_double_byte_width: true,
        ..Default::default()
      },
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    }
  }

  fn combined(runs: Vec<TextRun>) -> TwoLinesInline {
    TwoLinesInline {
      runs,
      settings: crate::model::WordprocessingTwoLines {
        id: Some(42),
        brackets: w::CombineBracketValues::None,
      },
      justify: false,
      auto_space_de: true,
      auto_space_dn: true,
      auto_space_like_word95: false,
      compatibility_mode: 12,
      strict_japanese: false,
      kashida: false,
    }
  }

  fn row_text(row: &Row) -> String {
    row.runs.iter().map(|run| run.text.as_str()).collect()
  }

  #[test]
  fn native_two_lines_balance_legal_breaks_without_cutting_western_words() {
    let mut metrics = TextMetrics::new();
    for family in ["宋体", "Microsoft YaHei"] {
      for (source, expected) in [
        ("甲乙丙丁戊", vec!["甲乙丙", "丁戊"]),
        ("甲乙丙丁戊己", vec!["甲乙丙", "丁戊己"]),
        ("abcdefg", vec!["abcdefg"]),
        ("abc def ghi", vec!["abc ", "def ghi"]),
      ] {
        let result = layout(&combined(vec![run(source, family, 12.0)]), &mut metrics);
        assert_eq!(
          result.rows.iter().map(row_text).collect::<Vec<_>>(),
          expected,
          "{family}/{source}"
        );
      }
    }
  }

  #[test]
  fn native_two_lines_use_hinted_cjk_cells_and_the_independent_descent_minimum() {
    let mut metrics = TextMetrics::new();
    // Actual Office + GDI vectors, not values derived from this layout.
    for (family, size, bold, ascent, descent) in [
      ("宋体", 12.0, false, 6.12, 2.04),
      ("宋体", 15.0, false, 7.56, 2.16),
      ("Microsoft YaHei", 15.0, false, 9.24, 3.48),
      ("Microsoft YaHei", 15.0, true, 9.60, 3.72),
    ] {
      let mut source = run("甲乙丙丁", family, size);
      source.style.bold = bold;
      let result = layout(&combined(vec![source]), &mut metrics);
      assert_eq!(result.rows.len(), 2);
      for row in result.rows {
        assert!(
          (row.cell.ascent - ascent).abs() < 0.001,
          "{family}/{size}/{bold}: {} vs {ascent}",
          row.cell.ascent
        );
        assert!(
          (row.cell.descent - descent).abs() < 0.001,
          "{family}/{size}/{bold}: {} vs {descent}",
          row.cell.descent
        );
      }
    }
  }

  #[test]
  fn native_two_lines_balance_mixed_sizes_by_advance_and_stack_distinct_cells() {
    let mut metrics = TextMetrics::new();
    let text = "甲乙丙丁戊己";
    for (cut, second_size, first_characters) in [
      (2, 4.0, 2),
      (2, 8.0, 2),
      (2, 24.0, 3),
      (3, 4.0, 2),
      (3, 8.0, 2),
      (3, 24.0, 4),
      (4, 4.0, 2),
      (4, 8.0, 3),
      (4, 24.0, 4),
    ] {
      let split = text.char_indices().nth(cut).unwrap().0;
      let result = layout(
        &combined(vec![
          run(&text[..split], "Microsoft YaHei", 15.0),
          run(&text[split..], "Microsoft YaHei", second_size),
        ]),
        &mut metrics,
      );
      assert_eq!(result.rows.len(), 2);
      assert_eq!(
        row_text(&result.rows[0]).chars().count(),
        first_characters,
        "cut{cut}/size{second_size}"
      );
    }
  }

  #[test]
  fn native_two_lines_keep_the_carrier_font_cell_and_mode_specific_space_breaks() {
    let mut text_metrics = TextMetrics::new();
    let source = combined(vec![run("甲乙丙丁", "宋体", 10.0)]);
    let measured = metrics(&source, &mut text_metrics);
    // Native10pt SimSun paired paragraphs advance16.94pt with4pt after;
    // the full design cell is12.96875pt, not two miniature7.08pt cells.
    assert!((measured.height_pt - 12.96875).abs() < 0.001);
    for (mode, expected) in [(12, vec!["    ", "   "]), (15, vec!["       "])] {
      let mut source = combined(vec![run("       ", "宋体", 15.0)]);
      source.compatibility_mode = mode;
      let result = layout(&source, &mut text_metrics);
      assert_eq!(
        result.rows.iter().map(row_text).collect::<Vec<_>>(),
        expected
      );
    }
  }

  #[test]
  fn native_two_lines_preserve_cjk_space_context_with_a_western_reference_face() {
    let mut metrics = TextMetrics::new();
    let mut source = run("  利润总额     成本费用总额", "Calibri", 15.0);
    source.style.wordprocessingml_balance_reference_font_family = Some(Arc::from("Calibri"));
    let result = layout(&combined(vec![source]), &mut metrics);
    assert_eq!(
      result.rows.iter().map(row_text).collect::<Vec<_>>(),
      vec!["  利润总额   ", "  成本费用总额"]
    );
  }
}
