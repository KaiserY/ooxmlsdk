//! Physical row ownership and independent text follows of ordinary rowspans.
//!
//! ISO 29500-1 17.4.84 makes the covered cells one logical cell. Pagination
//! still gives that cell a distinct frame in each physical table fragment.
//! Writer's SwTabFrame::Split inserts a rowspan follow and recalculates the
//! last master row (tabfrm.cxx, lcl_InsertNewFollowFlowLine / lcl_RecalcSplitLine).
//! Native Word controls, with and without lastRenderedPageBreak, likewise
//! assign lowers using only the physical rows retained in the current table.

use std::ops::Range;

use super::{
  HashMap, LAYOUT_EPSILON_PT, Table, row_cell_at_grid, row_cell_spacing_pt,
  vertical_merge_follow_end_row, vertical_merge_origin_row_index,
};

#[derive(Clone, Debug, Default)]
pub(super) struct VerticalMergeFollows {
  rows_on_page: Range<usize>,
  // Only active, cross-page merges are retained. A table follow can clone
  // this small map without copying the metrics of every already closed cell.
  cursors: HashMap<(usize, usize), f32>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct VerticalMergeFragment {
  origin_row: usize,
  end_row: usize,
  first_row: usize,
  pub top_pt: f32,
  pub height_pt: f32,
  pub content_offset_pt: f32,
  pub tracks_content: bool,
}

impl VerticalMergeFragment {
  pub fn owns_content(self, row_index: usize) -> bool {
    self.first_row == row_index && self.tracks_content
  }
}

impl VerticalMergeFollows {
  pub fn set_rows_on_page(&mut self, rows: Range<usize>) {
    self.rows_on_page = rows;
  }

  pub fn content_cursor(&self, origin_row: usize, grid_start: usize) -> Option<f32> {
    self.cursors.get(&(origin_row, grid_start)).copied()
  }

  pub fn fragment(
    &self,
    table: &Table,
    row_heights: &[f32],
    row_index: usize,
    grid_start: usize,
    row_bounds: Range<f32>,
    page_bottom: f32,
  ) -> Option<VerticalMergeFragment> {
    let row_top = row_bounds.start;
    let row_bottom = row_bounds.end;
    if !self.rows_on_page.contains(&row_index) {
      return None;
    }
    let row = table.rows.get(row_index)?;
    let cell = row_cell_at_grid(row, grid_start)?;
    let origin_row = if cell.vertical_merge_continue {
      vertical_merge_origin_row_index(table, row_index, grid_start)?
    } else {
      row_index
    };
    let end_row = vertical_merge_follow_end_row(table, origin_row, grid_start)?;
    if row_index > end_row
      || table.rows[origin_row..=end_row]
        .iter()
        .any(|row| row.exact_height)
      || row_cell_at_grid(&table.rows[origin_row], grid_start)?
        .text_rotation_deg
        .is_some()
    {
      // Fixed-height/cache compatibility has its established follow hierarchy.
      return None;
    }
    if cell.vertical_merge_continue
      && origin_row < self.rows_on_page.start
      && !self.cursors.contains_key(&(origin_row, grid_start))
    {
      // A master outside this owner's scope (for example an overflowing
      // unsplit row) did not create a text follow. Do not replay it from zero.
      return None;
    }
    let first_row = origin_row.max(self.rows_on_page.start);
    let last_row = end_row.min(self.rows_on_page.end.checked_sub(1)?);
    let before_current = height_of_rows(table, row_heights, first_row, row_index)?;
    let top_pt = row_top - before_current;
    let physical_row_height = (row_bottom - row_top).max(0.0);
    let height_pt = height_of_rows(table, row_heights, first_row, last_row)?
      + row_heights.get(last_row)?
      + physical_row_height
      - row_heights.get(row_index)?;
    if top_pt + height_pt > page_bottom + LAYOUT_EPSILON_PT {
      return None;
    }
    Some(VerticalMergeFragment {
      origin_row,
      end_row,
      first_row,
      top_pt,
      height_pt,
      content_offset_pt: self
        .cursors
        .get(&(origin_row, grid_start))
        .copied()
        .unwrap_or(0.0),
      tracks_content: first_row > origin_row
        || last_row < end_row
        || (physical_row_height - row_heights.get(row_index)?).abs() > LAYOUT_EPSILON_PT,
    })
  }

  pub fn finish_cell(
    &mut self,
    row_index: usize,
    grid_start: usize,
    fragment: VerticalMergeFragment,
    cursor: Option<f32>,
  ) {
    let key = (fragment.origin_row, grid_start);
    if fragment.owns_content(row_index) {
      self.cursors.insert(
        key,
        cursor
          .unwrap_or(fragment.content_offset_pt)
          .max(fragment.content_offset_pt),
      );
    }
    if row_index == fragment.end_row {
      self.cursors.remove(&key);
    }
  }
}

fn height_of_rows(table: &Table, row_heights: &[f32], first: usize, end: usize) -> Option<f32> {
  Some(
    row_heights.get(first..end)?.iter().sum::<f32>()
      + table
        .rows
        .get(first..end)?
        .iter()
        .map(|row| row_cell_spacing_pt(table, row))
        .sum::<f32>(),
  )
}

#[cfg(test)]
mod tests {
  use super::super::*;
  use crate::docx::{
    CellBorderSuppressions, CellBordersModel, CellMargins, Paragraph, ParagraphFormat,
  };

  fn paragraph(text: &str, cached: bool) -> Block {
    let style = TextStyle {
      font_family: Some(Arc::from("Times New Roman")),
      font_size_pt: 11.0,
      use_windows_font_metrics: true,
      ..TextStyle::default()
    };
    let run = TextRun {
      text: text.into(),
      style: style.clone(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    };
    let mut inlines = vec![InlineItem::Text(run.clone())];
    if cached {
      inlines.insert(0, InlineItem::LastRenderedPageBreak);
    }
    Block::paragraph(Paragraph {
      inlines,
      runs: vec![run],
      field_events: Vec::new(),
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: style,
      format: Box::new(ParagraphFormat {
        indent_left_pt: 4.25,
        indent_right_pt: 4.25,
        alignment: ParagraphAlignment::Center,
        widow_control: Some(false),
        ..ParagraphFormat::default()
      }),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_image: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    })
  }

  fn table(row_count: usize, cached: bool) -> Table {
    let border = BorderStyle {
      width_pt: 0.75,
      ..BorderStyle::default()
    };
    let base_cell = TableCell {
      blocks: Vec::new(),
      shading: None,
      borders: CellBordersModel {
        left: Some(border),
        right: Some(border),
        ..Default::default()
      },
      border_suppressions: CellBorderSuppressions::default(),
      margins: CellMargins::zero(),
      preferred_width_pt: Some(53.0),
      preferred_width_pct: None,
      grid_span: 1,
      vertical_merge_continue: false,
      no_wrap: false,
      fit_text: false,
      hide_end_mark: false,
      vertical_alignment: TableCellVerticalAlignment::Top,
      text_rotation_deg: None,
    };
    Table {
      recovered_absolute_grid: false,
      column_widths_pt: vec![53.0; 3],
      preferred_width_pt: Some(159.0),
      preferred_width_pct: None,
      layout: TableLayoutMode::Fixed,
      containing_table_layout: None,
      indent_left_pt: 0.0,
      alignment: TableAlignment::Left,
      right_to_left: false,
      align_leading_cell_content: true,
      in_header_footer: false,
      placement: None,
      allow_overlap: true,
      split_allowed: false,
      following_text_flow: false,
      explicit_no_repeat_header: false,
      page_break_before: false,
      starts_after_last_rendered_page_break: false,
      borders: None,
      cell_spacing: TableCellSpacing::Collapsed,
      rows: (0..row_count)
        .map(|index| {
          let first = index == 0;
          let mut amount = base_cell.clone();
          amount.vertical_merge_continue = !first;
          amount.blocks = vec![paragraph(
            if first {
              if row_count == 2 {
                "15 214,05"
              } else {
                "one\ntwo\nthree"
              }
            } else {
              ""
            },
            cached,
          )];
          let mut short = amount.clone();
          short.blocks = vec![paragraph(if first { "tag" } else { "" }, cached)];
          let mut unit = base_cell.clone();
          unit.blocks = vec![paragraph(if first { "5" } else { "unit" }, cached)];
          unit.borders.top = first.then_some(border);
          unit.borders.bottom = (index + 1 == row_count).then_some(border);
          unit.border_suppressions.top = !first;
          unit.border_suppressions.bottom = index + 1 != row_count;
          for cell in [&mut amount, &mut short] {
            cell.borders.top = first.then_some(border);
            cell.borders.bottom = (index + 1 == row_count).then_some(border);
            cell.border_suppressions.top = !first;
            cell.border_suppressions.bottom = index + 1 != row_count;
          }
          TableRow {
            cells: vec![amount, short, unit],
            height_pt: Some(if first { 13.3 } else { 13.85 }),
            exact_height: false,
            repeat_header: false,
            keep_with_next: false,
            cant_split: false,
            cell_spacing: None,
            grid_before: 0,
            grid_after: 0,
            width_before_pt: None,
            width_after_pt: None,
            layout: None,
            borders: None,
            spacing_shading: None,
            redline_color: None,
          }
        })
        .collect(),
    }
  }

  fn format(table: &Table, body_height: f32) -> Vec<Page> {
    format_after_prefix(table, body_height, 0.0)
  }

  fn format_after_prefix(table: &Table, body_height: f32, prefix_height: f32) -> Vec<Page> {
    let setup = PageSetup {
      width_pt: 240.0,
      height_pt: 28.35 * 2.0 + body_height,
      margin_left_pt: 28.35,
      margin_right_pt: 28.35,
      margin_top_pt: 28.35,
      margin_bottom_pt: 28.35,
      ..PageSetup::default()
    };
    let flow = flow_context(
      setup,
      0,
      SectionColumns::default(),
      0,
      0,
      DEFAULT_TAB_STOP_PT,
    );
    let mut metrics = TextMetrics::new();
    let layout =
      TableFrameLayout::new(table, block_area(flow), false, false, &mut metrics).unwrap();
    let mut current = empty_section_page(setup, 0, 0);
    let mut pages = Vec::new();
    layout.format(
      &mut current,
      &mut pages,
      &mut metrics,
      setup.margin_top_pt + prefix_height,
      prefix_height > 0.0,
    );
    pages.push(current);
    pages
  }

  fn text_in_column(page: &Page, index: usize) -> String {
    page
      .items
      .iter()
      .filter_map(|item| match item {
        PageItem::Text(text)
          if text.x_pt >= 28.35 + index as f32 * 53.0
            && text.x_pt < 28.35 + (index + 1) as f32 * 53.0 =>
        {
          Some(text.text.as_str())
        }
        _ => None,
      })
      .collect::<String>()
      .chars()
      .filter(|c| !c.is_whitespace())
      .collect()
  }

  #[test]
  fn vertical_merge_follows_consume_short_siblings_and_continue_amounts() {
    for cached in [false, true] {
      for body_height in [16.0, 20.0, 24.0, 28.0] {
        let pages = format(&table(2, cached), body_height);
        assert_eq!(pages.len(), 2, "body {body_height}, cached {cached}");
        assert_eq!(text_in_column(&pages[0], 0), "15");
        assert_eq!(text_in_column(&pages[1], 0), "214,05");
        assert_eq!(text_in_column(&pages[0], 1), "tag");
        assert_eq!(text_in_column(&pages[1], 1), "");
        let amount = pages[1]
          .items
          .iter()
          .find_map(|item| match item {
            PageItem::Text(text) if text.text.contains("214") => Some(text.y_pt),
            _ => None,
          })
          .unwrap();
        let unit = pages[1]
          .items
          .iter()
          .find_map(|item| match item {
            PageItem::Text(text) if text.text == "unit" => Some(text.y_pt),
            _ => None,
          })
          .unwrap();
        assert!(
          (amount - unit).abs() < LAYOUT_EPSILON_PT,
          "the follow owns its current row border"
        );
      }
      for body_height in [32.0, 40.0] {
        let pages = format(&table(2, cached), body_height);
        assert_eq!(pages.len(), 1);
        assert_eq!(text_in_column(&pages[0], 0), "15214,05");
        assert_eq!(text_in_column(&pages[0], 1), "tag");
      }
    }
  }

  #[test]
  fn completed_covered_cells_do_not_hold_a_partial_row_at_the_page_cut() {
    // Native cached/fresh covered-row controls retain a 283-twip minimum
    // and its 15-twip top strip. Extra available space does not grow that
    // physical master after the earlier merged lowers have completed.
    for cached in [false, true] {
      for body_height in [42.0, 44.0, 48.0] {
        let mut table = table(2, cached);
        table.rows[0].height_pt = Some(24.65);
        table.rows[0].cells[0].blocks = vec![paragraph("67,91", cached)];
        table.rows[0].cells[1].blocks = vec![paragraph("60,06", cached)];
        table.rows[0].cells[2].blocks = vec![paragraph("one\ntwo", cached)];
        table.rows[1].height_pt = Some(14.15);
        table.rows[1].cells[2].blocks = vec![paragraph("three\nfour", cached)];
        let border = table.rows[0].cells[2].borders.top;
        table.rows[1].cells[2].borders.top = border;
        table.rows[1].cells[2].border_suppressions.top = false;
        let pages = format(&table, body_height);
        assert_eq!(pages.len(), 2, "body {body_height}, cached {cached}");
        assert_eq!(text_in_column(&pages[0], 2), "onetwothree");
        assert_eq!(text_in_column(&pages[1], 2), "four");
        let master = pages[0]
          .frame_fragments
          .iter()
          .find(|frame| frame.kind == FrameFragmentKind::TableRow && frame.row_index == 1)
          .and_then(|frame| frame.bounds)
          .unwrap();
        assert!(
          (master.height_pt - 14.9).abs() < 0.001,
          "body {body_height}, cached {cached}: {master:?}"
        );
      }
    }
  }

  #[test]
  fn completed_new_rowspan_lowers_allow_the_master_to_end_at_its_last_line() {
    // Native controls vary the body cut while the two starting rowspan
    // masters finish before a longer unmerged cell's third retained line.
    // Their presence does not hold that row at the provisional page bottom.
    for cached in [false, true] {
      for body_height in [43.0, 47.0, 50.0] {
        let mut table = table(2, cached);
        table.rows[0].height_pt = Some(36.0);
        table.rows[0].cells[0].blocks = vec![paragraph("29\n583,81", cached)];
        table.rows[0].cells[1].blocks = vec![paragraph("27\n484,22", cached)];
        table.rows[0].cells[2].blocks = vec![paragraph("one\ntwo\nthree\nfour", cached)];
        table.rows[1].cells[2].blocks = vec![paragraph("five", cached)];
        table.column_widths_pt.push(53.0);
        table.preferred_width_pt = Some(212.0);
        for (index, row) in table.rows.iter_mut().enumerate() {
          let mut short = row.cells[2].clone();
          short.blocks = vec![paragraph(if index == 0 { "11" } else { "1" }, cached)];
          short.borders.top = Some(BorderStyle {
            width_pt: 0.75,
            ..Default::default()
          });
          row.cells.push(short);
        }
        let pages = format(&table, body_height);
        assert_eq!(text_in_column(&pages[0], 2), "onetwothree");
        assert_eq!(text_in_column(&pages[0], 0), "29583,81");
        assert_eq!(text_in_column(&pages[0], 1), "27484,22");
        let master = pages[0]
          .frame_fragments
          .iter()
          .find(|frame| frame.kind == FrameFragmentKind::TableRow && frame.row_index == 0)
          .and_then(|frame| frame.bounds)
          .unwrap();
        assert!(
          (master.height_pt - 38.696_777).abs() < 0.001,
          "body {body_height}, cached {cached}: {master:?}"
        );
        let short = pages[0]
          .frame_fragments
          .iter()
          .find(|frame| frame.kind == FrameFragmentKind::TableCell && frame.cell_index == Some(3))
          .and_then(|frame| frame.bounds)
          .unwrap();
        let center = short.x_pt + short.width_pt / 2.0;
        assert!(
          !pages[0].items.iter().any(|item| {
            matches!(item, PageItem::Fill(fill)
            if fill.width_pt > 20.0 && fill.height_pt < 2.0
            && (fill.y_pt - master.y_pt - master.height_pt).abs() < 0.3
            && fill.x_pt < center && fill.x_pt + fill.width_pt > center)
          }),
          "the following row's top must not replace this master's nil bottom"
        );
      }
    }
  }

  #[test]
  fn vertical_merge_completed_lowers_allow_an_unmerged_cell_to_split() {
    for cached in [false, true] {
      for (height, expected) in [
        (32.0, vec!["onetwo", "threefour", "unit"]),
        (40.0, vec!["onetwothree", "four", "unit"]),
        (48.0, vec!["onetwothree", "four", "unit"]),
        (48.05, vec!["onetwothree", "four", "unit"]),
        (64.0, vec!["onetwothreefour", "unit"]),
        (80.0, vec!["onetwothreefourunit"]),
      ] {
        let mut table = table(2, cached);
        table.rows[0].height_pt = Some(47.3);
        table.rows[0].cells[2].blocks = vec![paragraph("one\ntwo\nthree\nfour", cached)];
        let pages = format(&table, height);
        assert_eq!(
          pages
            .iter()
            .map(|page| text_in_column(page, 2))
            .collect::<Vec<_>>(),
          expected,
          "body {height}, cached {cached}"
        );
        assert_eq!(
          pages
            .iter()
            .map(|page| text_in_column(page, 0))
            .collect::<String>(),
          "15214,05"
        );
        assert_eq!(
          pages
            .iter()
            .map(|page| text_in_column(page, 1))
            .collect::<String>(),
          "tag"
        );
      }
    }
  }

  #[test]
  fn vertical_merge_cut_accounts_for_lowers_in_the_unformatted_master_prefix() {
    for cached in [false, true] {
      for row_count in [2, 3] {
        let mut table = table(row_count, cached);
        table.rows[0].height_pt = Some(26.6);
        table.rows[0].cells[0].blocks = vec![paragraph("15 214,05", cached)];
        table.rows[1].height_pt = Some(14.15);
        table.rows[1].cells[2].blocks = vec![paragraph("one\ntwo", cached)];
        let pages = format(&table, 44.0);
        assert_eq!(pages.len(), 2, "rows {row_count}, cached {cached}");
        assert_eq!(text_in_column(&pages[0], 2), "5one");
        assert_eq!(
          text_in_column(&pages[1], 2),
          if row_count == 2 { "two" } else { "twounit" }
        );
        assert_eq!(text_in_column(&pages[0], 0), "15214,05");
        assert_eq!(text_in_column(&pages[1], 0), "");
        assert_eq!(text_in_column(&pages[0], 1), "tag");
        assert_eq!(text_in_column(&pages[1], 1), "");
      }
    }
  }

  #[test]
  fn vertical_merge_covered_row_cut_reserves_closing_border_and_print_top() {
    for cached in [false, true] {
      for (body_height, bottom_width, minimum, split) in [
        (65.5, 0.75, 36.85, false),
        (66.0, 0.75, 36.85, true),
        (65.5, 0.0, 36.85, true),
        (66.0, 0.0, 36.85, true),
        (66.0, 1.5, 36.85, false),
        (65.5, 0.75, 0.0, true),
      ] {
        let mut table = table(2, cached);
        table.rows[0].height_pt = Some(26.6);
        table.rows[1].height_pt = Some(minimum);
        table.rows[1].cells[2].blocks = vec![paragraph("one\ntwo\nthree", cached)];
        table.rows[1].cells[2].borders.top = Some(BorderStyle {
          width_pt: 0.75,
          ..BorderStyle::default()
        });
        table.rows[1].cells[2].border_suppressions.top = false;
        for cell in &mut table.rows[1].cells {
          cell.borders.bottom = (bottom_width > 0.0).then_some(BorderStyle {
            width_pt: bottom_width,
            ..BorderStyle::default()
          });
          cell.border_suppressions.bottom = bottom_width == 0.0;
        }
        let pages = format(&table, body_height);
        assert_eq!(pages.len(), 2);
        let expected = if split {
          vec!["5onetwo", "three"]
        } else {
          vec!["5", "onetwothree"]
        };
        assert_eq!(
          pages
            .iter()
            .map(|page| text_in_column(page, 2))
            .collect::<Vec<_>>(),
          expected,
          "body {body_height}, bottom {bottom_width}, min {minimum}, cached {cached}"
        );
        assert_eq!(text_in_column(&pages[0], 0), "15214,05");
        assert_eq!(text_in_column(&pages[1], 0), "");
        assert_eq!(text_in_column(&pages[0], 1), "tag");
        assert_eq!(text_in_column(&pages[1], 1), "");
      }
    }
  }

  #[test]
  fn vertical_merge_oversized_minimum_uses_the_body_after_its_closing_border() {
    let mut table = table(2, false);
    table.rows[0].height_pt = Some(19.0);
    table.rows[0].cells[0].blocks = vec![paragraph("amount", false)];
    table.rows[0].cells[2].blocks = vec![paragraph("one\ntwo\nthree\nfour", false)];
    let border = Some(BorderStyle {
      width_pt: 0.75,
      ..BorderStyle::default()
    });
    table.rows[0].cells[2].borders.bottom = border;
    table.rows[0].cells[2].border_suppressions.bottom = false;
    table.rows[1].cells[2].borders.top = border;
    table.rows[1].cells[2].border_suppressions.top = false;
    let pages = format(&table, 20.0);
    assert_eq!(
      pages
        .iter()
        .map(|page| text_in_column(page, 2))
        .collect::<Vec<_>>(),
      ["one", "two", "three", "four", "unit"]
    );
    assert_eq!(
      pages
        .iter()
        .map(|page| text_in_column(page, 0))
        .collect::<String>(),
      "amount"
    );
    assert_eq!(
      pages
        .iter()
        .map(|page| text_in_column(page, 1))
        .collect::<String>(),
      "tag"
    );
  }

  #[test]
  fn vertical_merge_row_cut_accepts_the_authored_minimum_in_twips() {
    let mut table = table(2, false);
    table.rows[0].height_pt = Some(47.3);
    table.rows[0].cells[2].blocks = vec![paragraph("one\ntwo\nthree\nfour", false)];
    for (available, expected) in [
      (48.0, vec!["", "onetwothreefourunit"]),
      (48.05, vec!["onetwothree", "fourunit"]),
      (48.1, vec!["onetwothree", "fourunit"]),
    ] {
      let pages = format_after_prefix(&table, 200.0, 200.0 - available);
      assert_eq!(
        pages
          .iter()
          .map(|page| text_in_column(page, 2))
          .collect::<Vec<_>>(),
        expected,
        "available {available}"
      );
    }
  }

  #[test]
  fn vertical_merge_follows_keep_the_complete_page_local_span() {
    for cached in [false, true] {
      for (height, expected) in [
        (20.0, vec!["one", "two", "three"]),
        (32.0, vec!["onetwo", "three"]),
        (48.0, vec!["onetwothree"]),
      ] {
        let pages = format(&table(3, cached), height);
        assert_eq!(
          pages
            .iter()
            .map(|page| text_in_column(page, 0))
            .collect::<Vec<_>>(),
          expected
        );
        assert_eq!(
          pages
            .iter()
            .map(|page| text_in_column(page, 1))
            .collect::<String>(),
          "tag"
        );
        for page in &pages {
          let cells = page
            .frame_fragments
            .iter()
            .filter(|fragment| {
              fragment.kind == FrameFragmentKind::TableCell && fragment.cell_index == Some(0)
            })
            .filter_map(|fragment| fragment.bounds)
            .collect::<Vec<_>>();
          assert!(
            cells.windows(2).all(|pair| {
              (pair[0].x_pt - pair[1].x_pt).abs() < 0.001
                && (pair[0].y_pt - pair[1].y_pt).abs() < 0.001
                && (pair[0].width_pt - pair[1].width_pt).abs() < 0.001
                && (pair[0].height_pt - pair[1].height_pt).abs() < 0.001
            }),
            "each covered row has the same physical clip: height {height}, cached {cached}, {cells:?}"
          );
        }
      }
    }
  }

  #[test]
  fn merged_cell_backgrounds_precede_text_on_each_physical_page() {
    for height in [20.0, 32.0, 48.0, 160.0] {
      let mut table = table(3, false);
      let paint = ShadingPaint::Solid(RgbColor {
        r: 242,
        g: 242,
        b: 242,
      });
      for row in &mut table.rows {
        for cell in &mut row.cells[..2] {
          cell.shading = Some(paint);
          cell.vertical_alignment = TableCellVerticalAlignment::Center;
        }
      }
      let pages = format(&table, height);
      let mut checked = 0;
      for page in &pages {
        for (text_index, item) in page.items.iter().enumerate() {
          let PageItem::Text(text) = item else { continue };
          if text.x_pt >= 28.35 + 106.0 || text.text.trim().is_empty() {
            continue;
          }
          let covering = page
            .items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| {
              let PageItem::Fill(fill) = item else {
                return None;
              };
              (fill.paint == paint
                && text.x_pt >= fill.x_pt
                && text.x_pt < fill.x_pt + fill.width_pt
                && text.y_pt >= fill.y_pt
                && text.y_pt < fill.y_pt + fill.height_pt)
                .then_some(index)
            })
            .collect::<Vec<_>>();
          assert!(
            !covering.is_empty(),
            "missing merged background: height {height}, {text:?}"
          );
          assert!(
            covering.iter().all(|index| *index < text_index),
            "covered row overpainted merged text: height {height}, {text:?}"
          );
          checked += 1;
        }
      }
      assert!(checked > 0);
    }
  }
}
