//! Measure ordinary cell text with the line formatter that owns its lowers.
//!
//! A second, segment-by-segment wrapping estimate can reserve lines which the
//! actual paragraph does not contain. Row growth must use physical line-frame
//! advances, independently of glyph ink, paragraph spacing and cell padding.

use super::{
  BlockArea, DEFAULT_ORPHAN_LINES, DEFAULT_TAB_STOP_PT, EstimatedParagraphContentExtents,
  FlowContext, FrameFragmentKind, InlineItem, PageSetup, TextFrameLayout, TextMetrics,
  TextSegmentation, UNBOUNDED_LAYOUT_EXTENT_PT, empty_section_page, paragraph_content_width,
  paragraph_frame, resolved_paragraph_indents, table_cell_soft_break_line_height_delta,
  table_cell_soft_break_next_line_styles, wordprocessing_line_geometry,
};

#[derive(Clone, Copy)]
pub(super) struct TableCellMeasureContext {
  pub setup: PageSetup,
  pub default_tab_stop_pt: f32,
}

impl From<PageSetup> for TableCellMeasureContext {
  fn from(setup: PageSetup) -> Self {
    Self {
      setup,
      default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
    }
  }
}

impl From<BlockArea> for TableCellMeasureContext {
  fn from(area: BlockArea) -> Self {
    Self {
      setup: area.setup,
      default_tab_stop_pt: area.default_tab_stop_pt,
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
  if count == 2
    && let Some((index, offset)) = paragraph
      .inlines
      .iter()
      .enumerate()
      .filter_map(|(index, inline)| match inline {
        InlineItem::Text(run) => run.text.rfind('\n').map(|offset| (index, offset + 1)),
        _ => None,
      })
      .next_back()
    && let Some((first, following)) =
      table_cell_soft_break_next_line_styles(paragraph, index, offset, flow, layout.frame)
  {
    // Line fragments retain the baseline formatter's paragraph font box.
    // Preserve the independently established following-font row-growth
    // owner, also used by the object/field-aware estimator.
    let delta = table_cell_soft_break_line_height_delta(&first, following, text_metrics);
    height += delta;
    leading[1] += delta;
  }
  Some(EstimatedParagraphContentExtents {
    flow_height: height,
    floating_bottom: 0.0,
    paragraph_floating_bottom: 0.0,
    cell_print_floating_bottom: 0.0,
    leading_line_heights_pt: leading,
    line_count: count,
  })
}

pub(super) fn supports_paragraph(paragraph: &crate::docx::Paragraph) -> bool {
  !(paragraph.inlines.is_empty()
    || paragraph
      .field_events
      .iter()
      .any(|event| !matches!(event, crate::docx::ParagraphFieldEvent::Content))
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
        && !matches!(inline, InlineItem::LastRenderedPageBreak)
    }))
}

#[cfg(test)]
mod tests {
  use super::*;
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
    let (height, floating_bottom) =
      table_cell_paragraph_height(None, &paragraph, None, flow, &mut metrics, false, false);
    assert_eq!(height, 48.0);
    assert_eq!(floating_bottom, 0.0);
  }
}
