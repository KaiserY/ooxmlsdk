//! Measure ordinary cell text with the line formatter that owns its lowers.
//!
//! A second, segment-by-segment wrapping estimate can reserve lines which the
//! actual paragraph does not contain. Row growth must use physical line-frame
//! advances, independently of glyph ink, paragraph spacing and cell padding.

use super::{
  BlockArea, DEFAULT_ORPHAN_LINES, DEFAULT_TAB_STOP_PT, EstimatedParagraphContentExtents,
  FlowContext, FrameFragmentKind, InlineItem, PageSetup, TextFrameLayout, TextMetrics,
  TextSegmentation, UNBOUNDED_LAYOUT_EXTENT_PT, cell_horizontal_border_widths, empty_section_page,
  paragraph_content_width, paragraph_frame, resolved_paragraph_indents, row_has_separate_borders,
  wordprocessing_line_geometry,
};

#[derive(Clone, Copy)]
pub(super) struct TableCellMeasureContext {
  pub setup: PageSetup,
  pub horizontal_rule_reference_width_pt: Option<f32>,
  pub default_tab_stop_pt: f32,
  pub horizontal_borders: Option<(f32, f32, bool)>,
}

impl TableCellMeasureContext {
  pub(super) fn with_cell_borders(
    mut self,
    table: &crate::docx::Table,
    row: &crate::docx::TableRow,
    cell: &crate::docx::TableCell,
  ) -> Self {
    let (left, right) = cell_horizontal_border_widths(table, row, cell);
    self.horizontal_borders = Some((left, right, row_has_separate_borders(table, row)));
    self
  }
}

impl From<PageSetup> for TableCellMeasureContext {
  fn from(setup: PageSetup) -> Self {
    Self {
      setup,
      horizontal_rule_reference_width_pt: None,
      default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
      horizontal_borders: None,
    }
  }
}

impl From<BlockArea> for TableCellMeasureContext {
  fn from(area: BlockArea) -> Self {
    Self {
      setup: area.setup,
      horizontal_rule_reference_width_pt: area.horizontal_rule_reference_width_pt,
      default_tab_stop_pt: area.default_tab_stop_pt,
      horizontal_borders: None,
    }
  }
}

pub(super) fn paragraph_extents(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> Option<EstimatedParagraphContentExtents> {
  if flow.text_segmentation != TextSegmentation::TableCell
    || !flow.horizontal_table_cell
    || !supports_paragraph(paragraph)
  {
    return None;
  }

  let (left, right, _) = resolved_paragraph_indents(paragraph, text_metrics);
  let frame_width_pt = wordprocessing_line_geometry::frame_width(flow.content_width);
  let flow = FlowContext {
    content_top_pt: 0.0,
    content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
    body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
    content_width: paragraph_content_width(flow.content_width, left, right),
    ..flow
  };
  let mut page = empty_section_page(flow.setup, flow.section_index, flow.section_page_index);
  let mut pages = Vec::new();
  let layout = TextFrameLayout::new(paragraph, flow, frame_width_pt, 0.0, text_metrics);
  layout.format(&mut page, &mut pages, None, text_metrics, 0.0, 0.0);

  let mut height = 0.0;
  let mut leading = [0.0; DEFAULT_ORPHAN_LINES];
  let mut count = 0;
  for line in pages
    .iter()
    .chain(std::iter::once(&page))
    .flat_map(|page| &page.frame_fragments)
    .filter(|line| line.kind == FrameFragmentKind::ParagraphLine)
  {
    let advance = line.content_advance_pt?;
    height += advance;
    if let Some(first) = leading.get_mut(count) {
      *first = advance;
    }
    count += 1;
  }
  Some(EstimatedParagraphContentExtents {
    flow_height: height,
    floating_bottom: 0.0,
    paragraph_floating_bottom: 0.0,
    cell_print_floating_bottom: 0.0,
    leading_line_heights_pt: leading,
    first_line_page_fit_height_pt: leading[0],
    line_count: count,
  })
}

pub(super) fn supports_paragraph(paragraph: &crate::docx::Paragraph) -> bool {
  // Bookmarks carry source anchors, not text advances. Native joined,
  // split and bookmarked controls retain the same physical cell lines.
  // Let their ordinary text use this formatter too: the fallback estimate
  // can count a separate trailing-blank portion as an additional line.
  // Real field instructions/results keep their existing independent owner.
  !(paragraph.inlines.is_empty()
    || paragraph.field_events.iter().any(|event| {
      !matches!(
        event,
        crate::docx::ParagraphFieldEvent::Content
          | crate::docx::ParagraphFieldEvent::BookmarkStart { .. }
          | crate::docx::ParagraphFieldEvent::BookmarkEnd { .. }
      )
    })
    || paragraph_frame(paragraph).is_some()
    || paragraph.format.word_text_frame_story
    || paragraph.format.wordprocessing_shape_story
    || paragraph.list_label.is_some()
    || paragraph.list_label_image.is_some()
    || paragraph
      .format
      .suppressed_picture_bullet_owns_numbering_margin
    || paragraph.inlines.iter().any(|inline| {
      !matches!(inline, InlineItem::Text(run) if run.dynamic_field.is_none())
        && !matches!(
          inline,
          InlineItem::BookmarkStart(_) | InlineItem::LastRenderedPageBreak
        )
    }))
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::layout::BlockNeighbors;
  use crate::docx::{ParagraphFormat, TextRun, TextStyle};

  #[test]
  fn cell_text_measure_owns_lines_but_not_surrounding_paragraph_spacing() {
    use super::super::{
      DEFAULT_TAB_STOP_PT, LineHeightRule, PageSetup, SectionColumns, flow_context,
      table_cell_paragraph_height,
    };

    let style = TextStyle::default();
    let paragraph = crate::docx::Paragraph {
      inlines: vec![InlineItem::Text(TextRun {
        text: "alpha\nbeta".into(),
        style: style.clone(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })],
      format: Box::new(ParagraphFormat {
        line_height_rule: LineHeightRule::Exact,
        line_height_pt: Some(18.0),
        spacing_before_pt: 7.0,
        spacing_after_pt: 5.0,
        ..Default::default()
      }),
      base_style: style.clone(),
      runs: Vec::new(),
      field_events: vec![crate::docx::ParagraphFieldEvent::Content],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_image: None,
      list_label_style: style,
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    };
    let flow = FlowContext {
      text_segmentation: TextSegmentation::TableCell,
      horizontal_table_cell: true,
      ..flow_context(
        PageSetup::default(),
        0,
        SectionColumns::default(),
        0,
        0,
        DEFAULT_TAB_STOP_PT,
      )
    };
    let mut metrics = TextMetrics::new();
    let extents = paragraph_extents(&paragraph, flow, &mut metrics).unwrap();
    assert_eq!(extents.line_count, 2);
    assert_eq!(extents.flow_height, 36.0);
    assert_eq!(extents.leading_line_heights_pt, [18.0, 18.0]);
    let (height, floating_bottom) = table_cell_paragraph_height(
      BlockNeighbors::direct(None, None),
      &paragraph,
      flow,
      &mut metrics,
      false,
      false,
    );
    assert_eq!(height, 48.0);
    assert_eq!(floating_bottom, 0.0);
  }

  #[test]
  fn cell_text_measure_keeps_bookmarks_transparent_to_lines_and_cell_height() {
    use super::super::{
      BlockNeighbors, DEFAULT_TAB_STOP_PT, LineHeightRule, PageSetup, SectionColumns,
      TableCellMeasureMode, flow_context, table_cell_blocks_content_height,
      table_cell_paragraph_height,
    };
    use crate::docx::{Block, ParagraphFieldEvent};

    let style = TextStyle {
      font_size_pt: 8.0,
      ..Default::default()
    };
    let text = |value: &str| {
      InlineItem::Text(TextRun {
        text: value.into(),
        style: style.clone(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })
    };
    let mut paragraph = crate::docx::Paragraph {
      inlines: Vec::new(),
      format: Box::new(ParagraphFormat {
        line_height_rule: LineHeightRule::Exact,
        line_height_pt: Some(18.0),
        spacing_after_pt: 2.0,
        ..Default::default()
      }),
      base_style: style.clone(),
      runs: Vec::new(),
      field_events: vec![ParagraphFieldEvent::Content],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_image: None,
      list_label_style: style.clone(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    };
    let mut metrics = TextMetrics::new();
    let flow = FlowContext {
      text_segmentation: TextSegmentation::TableCell,
      horizontal_table_cell: true,
      content_width: metrics.measure_text("alpha", &style) + 1.0,
      ..flow_context(
        PageSetup::default(),
        0,
        SectionColumns::default(),
        0,
        0,
        DEFAULT_TAB_STOP_PT,
      )
    };
    for bookmarked in [false, true] {
      for split in [false, true] {
        paragraph.inlines = if split {
          vec![text("alpha"), text("                    "), text(" ")]
        } else {
          vec![text("alpha                     ")]
        };
        paragraph.field_events = vec![ParagraphFieldEvent::Content];
        if bookmarked {
          paragraph
            .inlines
            .insert(1, InlineItem::BookmarkStart("anchor".into()));
          paragraph.field_events.extend([
            ParagraphFieldEvent::BookmarkStart {
              id: "1".into(),
              name: "anchor".into(),
            },
            ParagraphFieldEvent::BookmarkEnd { id: "1".into() },
          ]);
        }
        let extents = paragraph_extents(&paragraph, flow, &mut metrics)
          .expect("ordinary bookmarked text is measured by its physical formatter");
        assert_eq!(extents.line_count, 1);
        assert_eq!(extents.flow_height, 18.0);
        let (height, _) = table_cell_paragraph_height(
          BlockNeighbors::direct(None, None),
          &paragraph,
          flow,
          &mut metrics,
          false,
          false,
        );
        assert_eq!(height, 20.0);
        let blocks = vec![Block::Paragraph(Box::new(paragraph.clone())); 3];
        assert_eq!(
          table_cell_blocks_content_height(
            &blocks,
            false,
            flow,
            TableCellMeasureMode::WholeCell,
            &mut metrics,
          ),
          60.0,
        );
      }
    }
    for event in [
      ParagraphFieldEvent::Begin {
        locked: false,
        dirty: false,
        inserted: false,
      },
      ParagraphFieldEvent::Instruction(" PAGE ".into()),
      ParagraphFieldEvent::Separate { style: None },
      ParagraphFieldEvent::End,
      ParagraphFieldEvent::BookmarkReferenceUsesCaret("anchor".into()),
    ] {
      paragraph.field_events = vec![event];
      assert!(!supports_paragraph(&paragraph));
    }
    paragraph.field_events = vec![ParagraphFieldEvent::Content];
    let InlineItem::Text(run) = &mut paragraph.inlines[0] else {
      unreachable!()
    };
    run.dynamic_field = Some(crate::model::DynamicFieldKind::Page {
      number_format: crate::model::FieldNumberFormat::Decimal,
    });
    assert!(!supports_paragraph(&paragraph));
  }
}
