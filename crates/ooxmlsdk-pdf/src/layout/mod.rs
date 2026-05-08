use std::cell::RefCell;
use std::collections::HashSet;

use icu_segmenter::LineSegmenter;

use crate::docx::{
  Block, BorderStyle, DocxDocument, FloatingImagePlacement, HorizontalImageReference,
  ImageWrapMode, InlineItem, PageSetup, ParagraphAlignment, RgbColor, SectionBreakKind,
  SectionColumns, TabStop, TabStopAlignment, TableCellVerticalAlignment, TextStyle,
  VerticalImageReference,
};
use crate::error::Result;
use crate::options::PdfOptions;
use crate::text_metrics::measure_text;

const PARAGRAPH_SPACING_AFTER_PT: f32 = 6.0;
const DEFAULT_FONT_SIZE_PT: f32 = 11.0;
const DEFAULT_LINE_HEIGHT_PT: f32 = 14.0;
const TABLE_ROW_MIN_HEIGHT_PT: f32 = 24.0;
const TABLE_SPACING_AFTER_PT: f32 = 6.0;
const MIN_HEADER_FOOTER_HEIGHT_PT: f32 = 72.0 / 25.4;

#[derive(Clone, Debug)]
pub(crate) struct LayoutDocument {
  pub pages: Vec<Page>,
}

#[derive(Clone, Debug)]
pub(crate) struct Page {
  pub setup: PageSetup,
  pub section_index: usize,
  pub items: Vec<PageItem>,
}

#[derive(Clone, Debug)]
pub(crate) enum PageItem {
  Text(TextItem),
  Image(ImageItem),
  Fill(FillItem),
  Line(LineItem),
}

#[derive(Clone, Debug)]
pub(crate) struct TextItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub text: String,
  pub style: TextStyle,
}

#[derive(Clone, Debug)]
pub(crate) struct ImageItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub data: Vec<u8>,
  pub content_type: Option<String>,
  pub alt_text: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FillItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub color: RgbColor,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LineItem {
  pub x1_pt: f32,
  pub y1_pt: f32,
  pub x2_pt: f32,
  pub y2_pt: f32,
  pub width_pt: f32,
  pub color: RgbColor,
}

#[derive(Clone, Copy, Debug)]
struct FlowContext {
  setup: PageSetup,
  section_index: usize,
  column_index: usize,
  columns: SectionColumns,
  content_left_pt: f32,
  content_bottom: f32,
  content_width: f32,
  default_tab_stop_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct ResolvedTabStop {
  x_pt: f32,
  alignment: TabStopAlignment,
}

#[derive(Clone, Copy, Debug)]
struct BlockArea {
  setup: PageSetup,
  section_index: usize,
  content_left_pt: f32,
  content_bottom: f32,
  content_width: f32,
}

#[derive(Clone, Copy, Debug)]
struct TableRenderArea<'a> {
  block: BlockArea,
  column_widths: &'a [f32],
  left_pt: f32,
  right_pt: f32,
}

pub(crate) fn layout(document: &DocxDocument, _options: &PdfOptions) -> Result<LayoutDocument> {
  let mut pages = Vec::new();
  let mut current = empty_page(document.page, 0);
  let mut y = document.page.margin_top_pt;
  let mut emitted_footnotes = HashSet::new();

  if document.sections.is_empty() {
    let mut flow = flow_context(
      document.page,
      0,
      SectionColumns::default(),
      0,
      document.default_tab_stop_pt,
    );
    for (index, block) in document.blocks.iter().enumerate() {
      let next = document.blocks.get(index + 1);
      (flow, y) = prepare_block_flow(block, next, flow, &mut current, &mut pages, y);
      y = layout_document_block(block, flow, &mut current, &mut pages, y);
      y = layout_referenced_footnotes(
        block,
        document,
        &mut emitted_footnotes,
        flow,
        &mut current,
        &mut pages,
        y,
      );
    }
  } else {
    for (section_index, section) in document.sections.iter().enumerate() {
      if section_index > 0 && starts_new_page(section.break_kind) && !current.items.is_empty() {
        pages.push(std::mem::replace(
          &mut current,
          empty_page(section.page, section_index),
        ));
        if needs_section_parity_blank(section.break_kind, pages.len() + 1) {
          pages.push(empty_page(section.page, section_index));
        }
        y = section.page.margin_top_pt;
      } else if current.items.is_empty() {
        current.setup = section.page;
        current.section_index = section_index;
        y = section.page.margin_top_pt;
      }

      let mut flow = flow_context(
        section.page,
        section_index,
        section.columns,
        0,
        document.default_tab_stop_pt,
      );
      for (index, block) in section.blocks.iter().enumerate() {
        let next = section.blocks.get(index + 1);
        (flow, y) = prepare_block_flow(block, next, flow, &mut current, &mut pages, y);
        if y + DEFAULT_LINE_HEIGHT_PT > flow.content_bottom && !current.items.is_empty() {
          (flow, y) = advance_section_flow(flow, &mut current, &mut pages);
        }
        y = layout_document_block(block, flow, &mut current, &mut pages, y);
        y = layout_referenced_footnotes(
          block,
          document,
          &mut emitted_footnotes,
          flow,
          &mut current,
          &mut pages,
          y,
        );
        if y + DEFAULT_LINE_HEIGHT_PT > flow.content_bottom && !current.items.is_empty() {
          (flow, y) = advance_section_flow(flow, &mut current, &mut pages);
        }
      }
    }
  }

  let note_setup = current.setup;
  let note_flow = flow_context(
    note_setup,
    current.section_index,
    SectionColumns::default(),
    0,
    document.default_tab_stop_pt,
  );
  if document.footnotes.is_empty() && !document.footnote_blocks.is_empty() {
    y = layout_note_separator(
      note_setup,
      &mut current,
      &mut pages,
      y,
      note_flow.content_bottom,
    );
    for block in &document.footnote_blocks {
      y = layout_document_block(block, note_flow, &mut current, &mut pages, y);
    }
  }

  if !document.endnotes.is_empty() {
    y = layout_note_separator(
      note_setup,
      &mut current,
      &mut pages,
      y,
      note_flow.content_bottom,
    );
    let mut emitted_endnotes = HashSet::new();
    for id in document_referenced_endnote_ids(document) {
      if !emitted_endnotes.insert(id) {
        continue;
      }
      if let Some(blocks) = document.endnotes.get(&id) {
        for block in blocks {
          y = layout_document_block(block, note_flow, &mut current, &mut pages, y);
        }
      }
    }
    for (id, blocks) in &document.endnotes {
      if emitted_endnotes.contains(id) {
        continue;
      }
      for block in blocks {
        y = layout_document_block(block, note_flow, &mut current, &mut pages, y);
      }
    }
  } else if !document.endnote_blocks.is_empty() {
    y = layout_note_separator(
      note_setup,
      &mut current,
      &mut pages,
      y,
      note_flow.content_bottom,
    );
    for block in &document.endnote_blocks {
      y = layout_document_block(block, note_flow, &mut current, &mut pages, y);
    }
  }

  if !document.comment_blocks.is_empty() {
    y = layout_note_separator(
      note_setup,
      &mut current,
      &mut pages,
      y,
      note_flow.content_bottom,
    );
    for block in &document.comment_blocks {
      y = layout_document_block(block, note_flow, &mut current, &mut pages, y);
    }
  }

  if !current.items.is_empty() || pages.is_empty() {
    pages.push(current);
  }

  apply_column_separators(document, &mut pages);
  apply_headers_and_footers(document, &mut pages);

  Ok(LayoutDocument { pages })
}

fn empty_page(setup: PageSetup, section_index: usize) -> Page {
  Page {
    setup,
    section_index,
    items: Vec::new(),
  }
}

fn flow_context(
  setup: PageSetup,
  section_index: usize,
  columns: SectionColumns,
  column_index: usize,
  default_tab_stop_pt: f32,
) -> FlowContext {
  let geometry = column_geometry(setup, columns);
  let count = geometry.widths.len().max(1);
  let column_index = column_index.min(count - 1);
  let content_left_pt = geometry.lefts[column_index];
  let column_width = geometry.widths[column_index];
  FlowContext {
    setup,
    section_index,
    column_index,
    columns,
    content_left_pt,
    content_bottom: setup.height_pt - setup.margin_bottom_pt,
    content_width: column_width.max(DEFAULT_FONT_SIZE_PT),
    default_tab_stop_pt,
  }
}

#[derive(Clone, Debug)]
struct ColumnGeometry {
  lefts: Vec<f32>,
  widths: Vec<f32>,
  gaps: Vec<f32>,
}

fn column_geometry(setup: PageSetup, columns: SectionColumns) -> ColumnGeometry {
  let count = columns.count.max(1);
  let body_width =
    (setup.width_pt - setup.margin_left_pt - setup.margin_right_pt).max(DEFAULT_FONT_SIZE_PT);
  let (widths, gaps) = if columns.explicit_count == count {
    let raw_gaps = columns
      .explicit_gaps_pt
      .iter()
      .copied()
      .take(count.saturating_sub(1))
      .collect::<Vec<_>>();
    let raw_widths = columns
      .explicit_widths_pt
      .iter()
      .copied()
      .take(count)
      .collect::<Vec<_>>();
    let raw_total = raw_widths.iter().sum::<f32>() + raw_gaps.iter().sum::<f32>();
    let scale = if raw_total > 0.0 {
      body_width / raw_total
    } else {
      1.0
    };
    (
      raw_widths
        .iter()
        .map(|width| (width * scale).max(DEFAULT_FONT_SIZE_PT))
        .collect::<Vec<_>>(),
      raw_gaps.iter().map(|gap| gap * scale).collect::<Vec<_>>(),
    )
  } else {
    let gap_total = columns.gap_pt * count.saturating_sub(1) as f32;
    let column_width = if count > 1 && body_width > gap_total {
      (body_width - gap_total) / count as f32
    } else {
      body_width
    };
    (
      vec![column_width.max(DEFAULT_FONT_SIZE_PT); count],
      vec![columns.gap_pt; count.saturating_sub(1)],
    )
  };

  let mut lefts = Vec::with_capacity(widths.len());
  let mut x = setup.margin_left_pt;
  for (index, width) in widths.iter().enumerate() {
    lefts.push(x);
    x += *width;
    if let Some(gap) = gaps.get(index) {
      x += *gap;
    }
  }

  ColumnGeometry {
    lefts,
    widths,
    gaps,
  }
}

fn advance_section_flow(
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
) -> (FlowContext, f32) {
  let next_column = flow.column_index + 1;
  if next_column < flow.columns.count {
    (
      flow_context(
        flow.setup,
        flow.section_index,
        flow.columns,
        next_column,
        flow.default_tab_stop_pt,
      ),
      flow.setup.margin_top_pt,
    )
  } else {
    pages.push(std::mem::replace(
      current,
      empty_page(flow.setup, flow.section_index),
    ));
    (
      flow_context(
        flow.setup,
        flow.section_index,
        flow.columns,
        0,
        flow.default_tab_stop_pt,
      ),
      flow.setup.margin_top_pt,
    )
  }
}

fn prepare_block_flow(
  block: &Block,
  next: Option<&Block>,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  y: f32,
) -> (FlowContext, f32) {
  if current.items.is_empty() || !block_should_stay_together(block, next) {
    return (flow, y);
  }
  let required_height = keep_group_height(block, next, flow);
  if y + required_height <= flow.content_bottom || y <= flow.setup.margin_top_pt + 0.1 {
    return (flow, y);
  }
  advance_section_flow(flow, current, pages)
}

fn block_should_stay_together(block: &Block, next: Option<&Block>) -> bool {
  match block {
    Block::Paragraph(paragraph) => {
      paragraph.format.keep_lines || (paragraph.format.keep_with_next && next.is_some())
    }
    Block::Table(_) => false,
  }
}

fn keep_group_height(block: &Block, next: Option<&Block>, flow: FlowContext) -> f32 {
  let mut height = estimated_block_height(block, flow);
  if let Block::Paragraph(paragraph) = block
    && paragraph.format.keep_with_next
    && let Some(next) = next
  {
    height += estimated_block_height(next, flow);
  }
  height
}

fn estimated_block_height(block: &Block, flow: FlowContext) -> f32 {
  match block {
    Block::Paragraph(paragraph) => estimated_paragraph_height(paragraph, flow),
    Block::Table(table) => table
      .rows
      .iter()
      .map(table_row_height)
      .sum::<f32>()
      .max(TABLE_ROW_MIN_HEIGHT_PT),
  }
}

fn estimated_paragraph_height(paragraph: &crate::docx::Paragraph, flow: FlowContext) -> f32 {
  let content_width =
    (flow.content_width - paragraph.format.indent_left_pt - paragraph.format.indent_right_pt)
      .max(DEFAULT_FONT_SIZE_PT);
  let line_height = paragraph
    .format
    .line_height_pt
    .unwrap_or(DEFAULT_LINE_HEIGHT_PT);
  let mut line_count = 1usize;
  let mut x = (paragraph.format.first_line_indent_pt).max(0.0);

  for item in &paragraph.inlines {
    match item {
      InlineItem::Text(run) => {
        for segment in text_segments(&run.text) {
          if segment == "\n" || segment == "\t" {
            line_count += 1;
            x = 0.0;
            continue;
          }
          let width = measure_text(&segment, run.style);
          if x + width > content_width && x > 0.0 {
            line_count += 1;
            x = 0.0;
          }
          x += width.min(content_width);
        }
      }
      InlineItem::Image(image) => {
        let (width, height) = fit_image_to_line(image.width_pt, image.height_pt, content_width);
        if x + width > content_width && x > 0.0 {
          line_count += 1;
        }
        line_count += (height / line_height).ceil().max(1.0) as usize - 1;
        x = width;
      }
      InlineItem::PageBreak | InlineItem::ColumnBreak => {
        line_count += 1;
        x = 0.0;
      }
    }
  }

  paragraph.format.spacing_before_pt
    + line_count as f32 * line_height
    + paragraph
      .format
      .spacing_after_pt
      .max(PARAGRAPH_SPACING_AFTER_PT)
}

fn starts_new_page(kind: SectionBreakKind) -> bool {
  matches!(
    kind,
    SectionBreakKind::NextPage
      | SectionBreakKind::NextColumn
      | SectionBreakKind::EvenPage
      | SectionBreakKind::OddPage
  )
}

fn needs_section_parity_blank(kind: SectionBreakKind, next_page_number: usize) -> bool {
  match kind {
    SectionBreakKind::EvenPage => !next_page_number.is_multiple_of(2),
    SectionBreakKind::OddPage => next_page_number.is_multiple_of(2),
    SectionBreakKind::Continuous | SectionBreakKind::NextPage | SectionBreakKind::NextColumn => {
      false
    }
  }
}

fn layout_document_block(
  block: &Block,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
) -> f32 {
  match block {
    Block::Paragraph(paragraph) => {
      if paragraph.format.page_break_before && !current.items.is_empty() {
        pages.push(std::mem::replace(
          current,
          empty_page(flow.setup, flow.section_index),
        ));
        y = flow.setup.margin_top_pt;
      }

      y += paragraph.format.spacing_before_pt;
      let paragraph_flow = FlowContext {
        content_width: (flow.content_width
          - paragraph.format.indent_left_pt
          - paragraph.format.indent_right_pt)
          .max(DEFAULT_FONT_SIZE_PT),
        ..flow
      };
      layout_paragraph(paragraph, paragraph_flow, current, pages, y)
    }
    Block::Table(table) => layout_table(table, block_area(flow), current, pages, y),
  }
}

fn block_area(flow: FlowContext) -> BlockArea {
  BlockArea {
    setup: flow.setup,
    section_index: flow.section_index,
    content_left_pt: flow.content_left_pt,
    content_bottom: flow.content_bottom,
    content_width: flow.content_width,
  }
}

fn layout_note_separator(
  setup: PageSetup,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
  content_bottom: f32,
) -> f32 {
  if y + DEFAULT_LINE_HEIGHT_PT > content_bottom && !current.items.is_empty() {
    pages.push(std::mem::replace(
      current,
      empty_page(setup, current.section_index),
    ));
    y = setup.margin_top_pt;
  }

  y += 4.0;
  current.items.push(PageItem::Line(LineItem {
    x1_pt: setup.margin_left_pt,
    y1_pt: y,
    x2_pt: setup.margin_left_pt + 120.0,
    y2_pt: y,
    width_pt: 0.5,
    color: RgbColor { r: 0, g: 0, b: 0 },
  }));
  y + 8.0
}

fn layout_referenced_footnotes(
  block: &Block,
  document: &DocxDocument,
  emitted_footnotes: &mut HashSet<i64>,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
) -> f32 {
  let Block::Paragraph(paragraph) = block else {
    return y;
  };
  let mut needs_separator = true;
  for id in &paragraph.footnote_reference_ids {
    if !emitted_footnotes.insert(*id) {
      continue;
    }
    let Some(blocks) = document.footnotes.get(id) else {
      continue;
    };
    if needs_separator {
      y = layout_note_separator(flow.setup, current, pages, y, flow.content_bottom);
      needs_separator = false;
    }
    for block in blocks {
      y = layout_document_block(block, flow, current, pages, y);
    }
  }
  y
}

fn document_referenced_endnote_ids(document: &DocxDocument) -> Vec<i64> {
  let blocks = if document.sections.is_empty() {
    EitherBlocks::Flat(&document.blocks)
  } else {
    EitherBlocks::Sections(&document.sections)
  };
  let mut ids = Vec::new();
  match blocks {
    EitherBlocks::Flat(blocks) => collect_endnote_ids_from_blocks(blocks, &mut ids),
    EitherBlocks::Sections(sections) => {
      for section in sections {
        collect_endnote_ids_from_blocks(&section.blocks, &mut ids);
      }
    }
  }
  ids
}

enum EitherBlocks<'a> {
  Flat(&'a [Block]),
  Sections(&'a [crate::docx::ImportedSection]),
}

fn collect_endnote_ids_from_blocks(blocks: &[Block], ids: &mut Vec<i64>) {
  for block in blocks {
    match block {
      Block::Paragraph(paragraph) => ids.extend(&paragraph.endnote_reference_ids),
      Block::Table(table) => {
        for row in &table.rows {
          for cell in &row.cells {
            collect_endnote_ids_from_blocks(&cell.blocks, ids);
          }
        }
      }
    }
  }
}

fn apply_headers_and_footers(document: &DocxDocument, pages: &mut [Page]) {
  let document_repeating_blocks_empty = document.header_blocks.is_empty()
    && document.footer_blocks.is_empty()
    && document.first_header_blocks.is_empty()
    && document.first_footer_blocks.is_empty();
  let sections_have_repeating_blocks = document.sections.iter().any(|section| {
    !section.header_blocks.is_empty()
      || !section.footer_blocks.is_empty()
      || !section.first_header_blocks.is_empty()
      || !section.first_footer_blocks.is_empty()
      || !section.even_header_blocks.is_empty()
      || !section.even_footer_blocks.is_empty()
  });
  if document_repeating_blocks_empty && !sections_have_repeating_blocks {
    return;
  }

  let mut previous_section_index = None;
  for (index, page) in pages.iter_mut().enumerate() {
    let first_page_in_section = previous_section_index != Some(page.section_index);
    previous_section_index = Some(page.section_index);
    let section = document.sections.get(page.section_index);
    let title_page = section
      .map(|section| section.title_page)
      .unwrap_or(document.title_page);
    let (default_header, default_footer, first_header, first_footer, even_header, even_footer) =
      section
        .map(|section| {
          (
            &section.header_blocks,
            &section.footer_blocks,
            &section.first_header_blocks,
            &section.first_footer_blocks,
            &section.even_header_blocks,
            &section.even_footer_blocks,
          )
        })
        .unwrap_or((
          &document.header_blocks,
          &document.footer_blocks,
          &document.first_header_blocks,
          &document.first_footer_blocks,
          &document.header_blocks,
          &document.footer_blocks,
        ));
    let use_even_slot = document.even_and_odd_headers && (index + 1) % 2 == 0;
    let header_blocks = if first_page_in_section && title_page && !first_header.is_empty() {
      first_header
    } else if use_even_slot && !even_header.is_empty() {
      even_header
    } else {
      default_header
    };
    let footer_blocks = if first_page_in_section && title_page && !first_footer.is_empty() {
      first_footer
    } else if use_even_slot && !even_footer.is_empty() {
      even_footer
    } else {
      default_footer
    };

    if header_blocks.is_empty() && footer_blocks.is_empty() {
      continue;
    }

    let content_width =
      (page.setup.width_pt - page.setup.margin_left_pt - page.setup.margin_right_pt)
        .max(DEFAULT_FONT_SIZE_PT);
    let mut adornment = empty_page(page.setup, page.section_index);
    let mut discarded_pages = Vec::new();
    let header_area = header_area(page.setup);
    let mut y = header_area.top_pt;
    for block in header_blocks {
      y = layout_repeating_block(
        block,
        &mut adornment,
        &mut discarded_pages,
        y,
        FlowContext {
          setup: page.setup,
          section_index: page.section_index,
          column_index: 0,
          columns: SectionColumns::default(),
          content_left_pt: page.setup.margin_left_pt,
          content_bottom: header_area.bottom_pt,
          content_width,
          default_tab_stop_pt: document.default_tab_stop_pt,
        },
      );
    }

    let footer_area = footer_area(page.setup);
    let mut y = footer_area.top_pt;
    for block in footer_blocks {
      y = layout_repeating_block(
        block,
        &mut adornment,
        &mut discarded_pages,
        y,
        FlowContext {
          setup: page.setup,
          section_index: page.section_index,
          column_index: 0,
          columns: SectionColumns::default(),
          content_left_pt: page.setup.margin_left_pt,
          content_bottom: footer_area.bottom_pt,
          content_width,
          default_tab_stop_pt: document.default_tab_stop_pt,
        },
      );
    }

    page.items.extend(adornment.items);
  }
}

fn apply_column_separators(document: &DocxDocument, pages: &mut [Page]) {
  for page in pages {
    let Some(section) = document.sections.get(page.section_index) else {
      continue;
    };
    let columns = section.columns;
    if !columns.separator || columns.count <= 1 {
      continue;
    }
    let geometry = column_geometry(page.setup, columns);
    for column_index in 1..geometry.widths.len() {
      let gap = geometry.gaps.get(column_index - 1).copied().unwrap_or(0.0);
      let x = geometry.lefts[column_index] - gap / 2.0;
      page.items.push(PageItem::Line(LineItem {
        x1_pt: x,
        y1_pt: page.setup.margin_top_pt,
        x2_pt: x,
        y2_pt: page.setup.height_pt - page.setup.margin_bottom_pt,
        width_pt: 0.5,
        color: RgbColor { r: 0, g: 0, b: 0 },
      }));
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct RepeatingArea {
  top_pt: f32,
  bottom_pt: f32,
}

fn header_area(setup: PageSetup) -> RepeatingArea {
  let top = setup.header_distance_pt.max(0.0);
  let bottom = setup.margin_top_pt.max(top + MIN_HEADER_FOOTER_HEIGHT_PT);
  RepeatingArea {
    top_pt: top,
    bottom_pt: bottom.min(setup.height_pt),
  }
}

fn footer_area(setup: PageSetup) -> RepeatingArea {
  let bottom = (setup.height_pt - setup.footer_distance_pt.max(0.0))
    .max(0.0)
    .min(setup.height_pt);
  let margin_top = (setup.height_pt - setup.margin_bottom_pt).max(0.0);
  let top = margin_top.min((bottom - MIN_HEADER_FOOTER_HEIGHT_PT).max(0.0));
  RepeatingArea {
    top_pt: top,
    bottom_pt: bottom
      .max(top + MIN_HEADER_FOOTER_HEIGHT_PT)
      .min(setup.height_pt),
  }
}

fn layout_repeating_block(
  block: &Block,
  page: &mut Page,
  discarded_pages: &mut Vec<Page>,
  y: f32,
  flow: FlowContext,
) -> f32 {
  match block {
    Block::Paragraph(paragraph) => layout_paragraph(
      paragraph,
      flow,
      page,
      discarded_pages,
      y + paragraph.format.spacing_before_pt,
    ),
    Block::Table(table) => layout_table(table, block_area(flow), page, discarded_pages, y),
  }
}

pub(crate) fn text_pages(pages: Vec<(PageSetup, Vec<String>)>) -> LayoutDocument {
  let mut output_pages = Vec::new();
  for (setup, lines) in pages {
    let mut page = empty_page(setup, 0);
    let mut y = setup.margin_top_pt;
    let content_bottom = setup.height_pt - setup.margin_bottom_pt;
    for line in lines {
      if y + DEFAULT_LINE_HEIGHT_PT > content_bottom {
        output_pages.push(page);
        page = empty_page(setup, 0);
        y = setup.margin_top_pt;
      }
      if !line.is_empty() {
        page.items.push(PageItem::Text(TextItem {
          x_pt: setup.margin_left_pt,
          y_pt: y,
          text: line,
          style: TextStyle::default(),
        }));
      }
      y += DEFAULT_LINE_HEIGHT_PT;
    }
    output_pages.push(page);
  }

  if output_pages.is_empty() {
    output_pages.push(empty_page(PageSetup::default(), 0));
  }

  LayoutDocument {
    pages: output_pages,
  }
}

fn layout_table(
  table: &crate::docx::Table,
  area: BlockArea,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
) -> f32 {
  let _table_cell_margins = table.cell_margins;
  let column_count = table
    .rows
    .iter()
    .map(|row| row.cells.iter().map(|cell| cell.grid_span).sum::<usize>())
    .max()
    .unwrap_or(0);
  if column_count == 0 {
    return y;
  }

  let column_widths = table_column_widths(table, column_count, area.content_width);
  let table_left = area.content_left_pt;
  let table_right = table_left + column_widths.iter().sum::<f32>();
  let render_area = TableRenderArea {
    block: area,
    column_widths: &column_widths,
    left_pt: table_left,
    right_pt: table_right,
  };
  let repeating_header_count = table_repeating_header_count(table);
  let repeating_header_height = table.rows[..repeating_header_count]
    .iter()
    .map(table_row_height)
    .sum::<f32>();

  for (row_index, row) in table.rows.iter().enumerate() {
    let row_height = table_row_height(row);
    let row_overflows = y + row_height > area.content_bottom;
    let row_fits_empty_region = row_height <= area.content_bottom - area.setup.margin_top_pt;
    if row_overflows
      && !current.items.is_empty()
      && (!row.cant_split || row_fits_empty_region || y > area.setup.margin_top_pt + 0.1)
    {
      pages.push(std::mem::replace(
        current,
        empty_page(area.setup, area.section_index),
      ));
      y = area.setup.margin_top_pt;
      if row_index >= repeating_header_count
        && repeating_header_count > 0
        && y + repeating_header_height + row_height <= area.content_bottom
      {
        for (header_index, header) in table.rows[..repeating_header_count].iter().enumerate() {
          y = render_table_row(table, header, header_index, render_area, current, y);
        }
      }
    }

    y = render_table_row(table, row, row_index, render_area, current, y);
  }

  y + TABLE_SPACING_AFTER_PT
}

fn table_repeating_header_count(table: &crate::docx::Table) -> usize {
  let count = table
    .rows
    .iter()
    .take_while(|row| row.repeat_header)
    .count();
  if count == 0 || count > 10 || (count == table.rows.len() && table.rows.len() > 1) {
    0
  } else {
    count
  }
}

fn render_table_row(
  table: &crate::docx::Table,
  row: &crate::docx::TableRow,
  row_index: usize,
  area: TableRenderArea<'_>,
  current: &mut Page,
  y: f32,
) -> f32 {
  let row_height = table_row_height(row);
  let row_top = y;
  let row_bottom = y + row_height;

  let mut cell_left = area.left_pt;
  let mut grid_index = 0;
  for (cell_index, cell) in row.cells.iter().enumerate() {
    if grid_index >= area.column_widths.len() {
      break;
    }
    let span = cell
      .grid_span
      .max(1)
      .min(area.column_widths.len().saturating_sub(grid_index));
    let width = area.column_widths[grid_index..grid_index + span]
      .iter()
      .sum::<f32>();
    if !cell.vertical_merge_continue {
      if let Some(color) = cell.shading {
        current.items.push(PageItem::Fill(FillItem {
          x_pt: cell_left,
          y_pt: row_top,
          width_pt: width,
          height_pt: row_height,
          color,
        }));
      }
      if let Some(border) = vertical_border(table, row, cell_index, true) {
        push_styled_line(current, cell_left, row_top, cell_left, row_bottom, border);
      }
      layout_table_cell(
        cell,
        area.block.setup,
        current,
        cell_left,
        row_top,
        width,
        row_height,
      );
    }
    cell_left += width;
    grid_index += span;
  }
  if let Some(border) = horizontal_border(table, row_index, true) {
    push_styled_line(
      current,
      area.left_pt,
      row_top,
      area.right_pt,
      row_top,
      border,
    );
  }
  if let Some(border) = horizontal_border(table, row_index, false) {
    push_styled_line(
      current,
      area.left_pt,
      row_bottom,
      area.right_pt,
      row_bottom,
      border,
    );
  }
  if let Some(border) = row
    .cells
    .last()
    .and_then(|cell| cell.borders.right)
    .or_else(|| table.borders.and_then(|borders| borders.right))
    .or(Some(BorderStyle::default()))
  {
    push_styled_line(
      current,
      area.right_pt,
      row_top,
      area.right_pt,
      row_bottom,
      border,
    );
  }
  row_bottom
}

fn horizontal_border(
  table: &crate::docx::Table,
  row_index: usize,
  top_edge: bool,
) -> Option<BorderStyle> {
  let borders = table.borders;
  let row = table.rows.get(row_index)?;
  if top_edge {
    row
      .cells
      .first()
      .and_then(|cell| cell.borders.top)
      .or_else(|| {
        borders.and_then(|borders| {
          if row_index == 0 {
            borders.top
          } else {
            borders.inside_horizontal
          }
        })
      })
      .or(Some(BorderStyle::default()))
  } else {
    row
      .cells
      .first()
      .and_then(|cell| cell.borders.bottom)
      .or_else(|| {
        borders.and_then(|borders| {
          if row_index + 1 == table.rows.len() {
            borders.bottom
          } else {
            borders.inside_horizontal
          }
        })
      })
      .or(Some(BorderStyle::default()))
  }
}

fn vertical_border(
  table: &crate::docx::Table,
  row: &crate::docx::TableRow,
  cell_index: usize,
  left_edge: bool,
) -> Option<BorderStyle> {
  let borders = table.borders;
  let cell = row.cells.get(cell_index)?;
  if left_edge {
    cell
      .borders
      .left
      .or_else(|| {
        if cell_index > 0 {
          row
            .cells
            .get(cell_index - 1)
            .and_then(|previous| previous.borders.right)
        } else {
          None
        }
      })
      .or_else(|| {
        borders.and_then(|borders| {
          if cell_index == 0 {
            borders.left
          } else {
            borders.inside_vertical
          }
        })
      })
      .or(Some(BorderStyle::default()))
  } else {
    cell
      .borders
      .right
      .or_else(|| {
        borders.and_then(|borders| {
          if cell_index + 1 == row.cells.len() {
            borders.right
          } else {
            borders.inside_vertical
          }
        })
      })
      .or(Some(BorderStyle::default()))
  }
}

fn table_column_widths(
  table: &crate::docx::Table,
  column_count: usize,
  content_width: f32,
) -> Vec<f32> {
  if table.column_widths_pt.len() >= column_count {
    let mut widths = table.column_widths_pt[..column_count].to_vec();
    if let Some(preferred) = table.preferred_width_pt
      && preferred > 0.0
    {
      let total = widths.iter().sum::<f32>();
      if total > 0.0 {
        let scale = preferred / total;
        for width in &mut widths {
          *width *= scale;
        }
      }
    }
    let total = widths.iter().sum::<f32>();
    if total > content_width && total > 0.0 {
      let scale = content_width / total;
      for width in &mut widths {
        *width *= scale;
      }
    }
    return widths;
  }

  vec![content_width / column_count as f32; column_count]
}

fn table_row_height(row: &crate::docx::TableRow) -> f32 {
  let content_height = row
    .cells
    .iter()
    .filter(|cell| !cell.vertical_merge_continue)
    .map(|cell| {
      let line_count = cell
        .blocks
        .iter()
        .map(|block| match block {
          Block::Paragraph(paragraph) => paragraph
            .inlines
            .iter()
            .filter(|item| match item {
              InlineItem::Text(run) => !run.text.is_empty(),
              InlineItem::Image(_) => true,
              InlineItem::PageBreak | InlineItem::ColumnBreak => false,
            })
            .count()
            .max(1),
          Block::Table(table) => table.rows.len().max(1),
        })
        .sum::<usize>()
        .max(1);
      cell.margins.top_pt + cell.margins.bottom_pt + DEFAULT_LINE_HEIGHT_PT * line_count as f32
    })
    .fold(TABLE_ROW_MIN_HEIGHT_PT, f32::max);
  match (row.height_pt, row.exact_height) {
    (Some(height), true) => height,
    (Some(height), false) => content_height.max(height),
    (None, _) => content_height,
  }
}

fn layout_table_cell(
  cell: &crate::docx::TableCell,
  setup: PageSetup,
  page: &mut Page,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
) {
  let content_height = table_cell_content_height(cell);
  let mut text_y = match cell.vertical_alignment {
    TableCellVerticalAlignment::Top => y + cell.margins.top_pt + DEFAULT_FONT_SIZE_PT,
    TableCellVerticalAlignment::Center => {
      y + ((height - content_height) / 2.0).max(cell.margins.top_pt) + DEFAULT_FONT_SIZE_PT
    }
    TableCellVerticalAlignment::Bottom => {
      y + (height - cell.margins.bottom_pt - content_height).max(cell.margins.top_pt)
        + DEFAULT_FONT_SIZE_PT
    }
  };
  let text_left = x + cell.margins.left_pt;
  let text_right = x + width - cell.margins.right_pt;
  let text_bottom = y + height - cell.margins.bottom_pt;

  for block in &cell.blocks {
    match block {
      Block::Paragraph(paragraph) => {
        let mut text_x = text_left;
        for item in &paragraph.inlines {
          match item {
            InlineItem::Text(run) => {
              if run.text.is_empty() {
                continue;
              }
              if text_y > text_bottom {
                return;
              }
              let available = (text_right - text_x).max(DEFAULT_FONT_SIZE_PT);
              let clipped = clip_to_width(&run.text, run.style, available);
              if clipped.is_empty() {
                continue;
              }
              let width = measure_text(&clipped, run.style);
              page.items.push(PageItem::Text(TextItem {
                x_pt: text_x,
                y_pt: text_y,
                text: clipped,
                style: run.style,
              }));
              text_x += width;
            }
            InlineItem::Image(image) => {
              if text_y > text_bottom {
                return;
              }
              let width = image.width_pt.min((text_right - text_x).max(1.0));
              let height = if image.width_pt > 0.0 {
                image.height_pt * (width / image.width_pt)
              } else {
                image.height_pt
              };
              page.items.push(PageItem::Image(ImageItem {
                x_pt: text_x,
                y_pt: text_y - DEFAULT_FONT_SIZE_PT,
                width_pt: width,
                height_pt: height,
                data: image.data.clone(),
                content_type: image.content_type.clone(),
                alt_text: image.alt_text.clone(),
              }));
              text_x += width;
            }
            InlineItem::PageBreak | InlineItem::ColumnBreak => {
              text_y = text_bottom + 1.0;
            }
          }
        }
        text_y += paragraph
          .format
          .line_height_pt
          .unwrap_or(DEFAULT_LINE_HEIGHT_PT);
      }
      Block::Table(table) => {
        let nested = layout_nested_table(
          table,
          setup,
          page,
          text_left,
          text_y,
          text_right - text_left,
        );
        text_y += nested;
      }
    }
  }
}

fn table_cell_content_height(cell: &crate::docx::TableCell) -> f32 {
  let content = cell
    .blocks
    .iter()
    .map(|block| match block {
      Block::Paragraph(paragraph) => paragraph
        .format
        .line_height_pt
        .unwrap_or(DEFAULT_LINE_HEIGHT_PT),
      Block::Table(table) => table.rows.len().max(1) as f32 * TABLE_ROW_MIN_HEIGHT_PT,
    })
    .sum::<f32>()
    .max(DEFAULT_LINE_HEIGHT_PT);
  cell.margins.top_pt + content + cell.margins.bottom_pt
}

fn layout_nested_table(
  table: &crate::docx::Table,
  setup: PageSetup,
  page: &mut Page,
  x: f32,
  y: f32,
  content_width: f32,
) -> f32 {
  let mut nested_pages = Vec::new();
  let mut nested_page = empty_page(setup, page.section_index);
  let bottom = setup.height_pt;
  let end_y = layout_table(
    table,
    BlockArea {
      setup,
      section_index: page.section_index,
      content_left_pt: x,
      content_bottom: bottom,
      content_width,
    },
    &mut nested_page,
    &mut nested_pages,
    y,
  );
  page.items.extend(nested_page.items);
  end_y - y
}

fn floating_image_position(
  placement: FloatingImagePlacement,
  flow: FlowContext,
  current_x: f32,
  current_y: f32,
) -> (f32, f32) {
  let base_x = match placement.horizontal_relative_to {
    HorizontalImageReference::Page => 0.0,
    HorizontalImageReference::Margin => flow.setup.margin_left_pt,
    HorizontalImageReference::Column => flow.content_left_pt,
    HorizontalImageReference::Character => current_x,
  };
  let base_y = match placement.vertical_relative_to {
    VerticalImageReference::Page => 0.0,
    VerticalImageReference::Margin => flow.setup.margin_top_pt,
    VerticalImageReference::Paragraph | VerticalImageReference::Line => current_y,
  };
  (
    base_x + placement.horizontal_offset_pt,
    base_y + placement.vertical_offset_pt,
  )
}

fn layout_paragraph(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
) -> f32 {
  let start_item_index = current.items.len();
  let paragraph_top = y;
  let setup = flow.setup;
  let line_left = flow.content_left_pt + paragraph.format.indent_left_pt;
  let first_line_left =
    (line_left + paragraph.format.first_line_indent_pt).max(flow.content_left_pt);
  let line_right = line_left + flow.content_width;
  let paragraph_left = line_left.min(first_line_left);
  let base_line_height = paragraph
    .format
    .line_height_pt
    .unwrap_or(DEFAULT_LINE_HEIGHT_PT);
  let mut line_height = base_line_height;
  let mut emitted = paragraph.list_label.is_some();
  let mut pending_tab: Option<ResolvedTabStop> = None;
  let mut x = if let Some(label) = &paragraph.list_label {
    current.items.push(PageItem::Text(TextItem {
      x_pt: first_line_left,
      y_pt: y,
      text: label.clone(),
      style: TextStyle::default(),
    }));
    line_left
  } else {
    first_line_left
  };

  for item in &paragraph.inlines {
    match item {
      InlineItem::Text(run) => {
        let mut chunk = String::new();
        let mut chunk_x = x;

        for segment in text_segments(&run.text) {
          if segment == "\n" {
            flush_text(current, chunk_x, y, &mut chunk, run.style);
            y = next_line(
              setup,
              current,
              pages,
              y,
              &mut line_height,
              flow.content_bottom,
            );
            x = line_left;
            chunk_x = x;
            pending_tab = None;
            emitted = true;
            continue;
          }
          if segment == "\t" {
            flush_text(current, chunk_x, y, &mut chunk, run.style);
            let mut tab_stop = next_tab_stop(
              x,
              line_left,
              &paragraph.format.tab_stops,
              flow.default_tab_stop_pt,
            );
            x = tab_stop.x_pt;
            if x > line_right {
              y = next_line(
                setup,
                current,
                pages,
                y,
                &mut line_height,
                flow.content_bottom,
              );
              tab_stop = next_tab_stop(
                line_left,
                line_left,
                &paragraph.format.tab_stops,
                flow.default_tab_stop_pt,
              );
              x = tab_stop.x_pt;
            }
            chunk_x = x;
            pending_tab = Some(tab_stop);
            line_height = line_height.max(run.style.font_size_pt * 1.25);
            emitted = true;
            continue;
          }

          let width = measure_text(&segment, run.style);
          let line_capacity = (line_right - line_left).max(DEFAULT_FONT_SIZE_PT);
          let whitespace = segment.chars().all(char::is_whitespace);
          if let Some(tab_stop) = pending_tab.take()
            && !whitespace
          {
            x = aligned_tab_x(tab_stop, width, line_left, line_right);
            chunk_x = x;
          }

          if x + width > line_right && x > line_left {
            flush_text(current, chunk_x, y, &mut chunk, run.style);
            y = next_line(
              setup,
              current,
              pages,
              y,
              &mut line_height,
              flow.content_bottom,
            );
            x = line_left;
            chunk_x = x;
            pending_tab = None;
            if whitespace {
              emitted = true;
              continue;
            }
          }

          if width > line_capacity && x <= line_left && !whitespace {
            for text in emergency_break_segments(&segment) {
              let width = measure_text(&text, run.style);
              if width > line_capacity && text.chars().count() > 1 {
                for ch in text.chars() {
                  let mut encoded = [0; 4];
                  let text = ch.encode_utf8(&mut encoded);
                  let width = measure_text(text, run.style);

                  if x + width > line_right && x > line_left {
                    flush_text(current, chunk_x, y, &mut chunk, run.style);
                    y = next_line(
                      setup,
                      current,
                      pages,
                      y,
                      &mut line_height,
                      flow.content_bottom,
                    );
                    x = line_left;
                    chunk_x = x;
                    pending_tab = None;
                  }

                  if chunk.is_empty() {
                    chunk_x = x;
                  }
                  chunk.push_str(text);
                  x += width;
                  line_height = line_height.max(run.style.font_size_pt * 1.25);
                  emitted = true;
                }
                continue;
              }

              if x + width > line_right && x > line_left {
                flush_text(current, chunk_x, y, &mut chunk, run.style);
                y = next_line(
                  setup,
                  current,
                  pages,
                  y,
                  &mut line_height,
                  flow.content_bottom,
                );
                x = line_left;
                chunk_x = x;
                pending_tab = None;
              }

              if chunk.is_empty() {
                chunk_x = x;
              }
              chunk.push_str(&text);
              x += width;
              line_height = line_height.max(run.style.font_size_pt * 1.25);
              emitted = true;
            }
            continue;
          }

          if chunk.is_empty() {
            chunk_x = x;
          }
          chunk.push_str(&segment);
          x += width;
          line_height = line_height.max(run.style.font_size_pt * 1.25);
          emitted = true;
        }

        flush_text(current, chunk_x, y, &mut chunk, run.style);
      }
      InlineItem::Image(image) => {
        pending_tab = None;
        let (width, height) =
          fit_image_to_line(image.width_pt, image.height_pt, flow.content_width);
        if let crate::docx::ImagePlacement::Floating(placement) = image.placement {
          let (image_x, image_y) = floating_image_position(placement, flow, x, y);
          current.items.push(PageItem::Image(ImageItem {
            x_pt: image_x,
            y_pt: image_y,
            width_pt: width,
            height_pt: height,
            data: image.data.clone(),
            content_type: image.content_type.clone(),
            alt_text: image.alt_text.clone(),
          }));
          match placement.wrap {
            ImageWrapMode::TopBottom | ImageWrapMode::None => {
              y = y.max(image_y + height + placement.margin_bottom_pt);
              x = line_left;
              line_height = base_line_height;
            }
            ImageWrapMode::Square | ImageWrapMode::Tight
              if !placement.behind_text
                && y >= image_y - placement.margin_top_pt
                && y <= image_y + height + placement.margin_bottom_pt =>
            {
              let wrapped_x = image_x + width + placement.margin_right_pt;
              if wrapped_x < line_right {
                x = x.max(wrapped_x);
              }
              line_height = line_height.max(height.min(base_line_height));
            }
            ImageWrapMode::Through
            | ImageWrapMode::Inline
            | ImageWrapMode::Square
            | ImageWrapMode::Tight => {}
          }
          emitted = true;
          continue;
        }
        if x + width > line_right && x > line_left {
          y = next_line(
            setup,
            current,
            pages,
            y,
            &mut line_height,
            flow.content_bottom,
          );
          x = line_left;
        }
        current.items.push(PageItem::Image(ImageItem {
          x_pt: x,
          y_pt: y,
          width_pt: width,
          height_pt: height,
          data: image.data.clone(),
          content_type: image.content_type.clone(),
          alt_text: image.alt_text.clone(),
        }));
        x += width;
        line_height = line_height.max(height);
        emitted = true;
      }
      InlineItem::PageBreak => {
        y = force_page_break(setup, current, pages);
        x = first_line_left;
        line_height = base_line_height;
        emitted = false;
        pending_tab = None;
      }
      InlineItem::ColumnBreak => {
        if flow.columns.count <= 1 || flow.column_index + 1 >= flow.columns.count {
          y = force_page_break(setup, current, pages);
          x = first_line_left;
          line_height = base_line_height;
          emitted = false;
        } else {
          y = flow.content_bottom;
          x = line_left;
          emitted = true;
        }
        pending_tab = None;
      }
    }
  }

  let paragraph_bottom;
  if emitted {
    paragraph_bottom = y + line_height;
    y = paragraph_bottom
      + paragraph
        .format
        .spacing_after_pt
        .max(PARAGRAPH_SPACING_AFTER_PT);
  } else {
    paragraph_bottom = y + base_line_height;
    y = paragraph_bottom + paragraph.format.spacing_after_pt;
  }

  if paragraph.list_label.is_none() && start_item_index <= current.items.len() {
    align_paragraph_items(
      &mut current.items[start_item_index..],
      paragraph.format.alignment,
      line_right,
    );
  }
  if start_item_index <= current.items.len() {
    decorate_paragraph(
      current,
      start_item_index,
      paragraph,
      paragraph_left,
      paragraph_top,
      line_right - paragraph_left,
      paragraph_bottom - paragraph_top,
    );
  }

  y
}

fn text_segments(text: &str) -> Vec<String> {
  let mut segments = Vec::new();
  let mut start = 0;

  for (index, ch) in text.char_indices() {
    if ch != '\n' && ch != '\t' {
      continue;
    }

    push_line_segments(&text[start..index], &mut segments);
    segments.push(ch.to_string());
    start = index + ch.len_utf8();
  }

  push_line_segments(&text[start..], &mut segments);
  segments
}

fn emergency_break_segments(text: &str) -> Vec<String> {
  if text.chars().all(|ch| ch.is_ascii_alphabetic()) && text.chars().count() > 8 {
    let mut pieces = hypher::hyphenate(text, hypher::Lang::English)
      .map(str::to_string)
      .collect::<Vec<_>>();
    if pieces.len() > 1 {
      let last = pieces.len() - 1;
      for piece in &mut pieces[..last] {
        piece.push('-');
      }
      return pieces;
    }
  }

  text.chars().map(|ch| ch.to_string()).collect()
}

fn push_line_segments(text: &str, segments: &mut Vec<String>) {
  if text.is_empty() {
    return;
  }

  thread_local! {
    static LINE_SEGMENTER: RefCell<LineSegmenter> = RefCell::new(LineSegmenter::new_auto());
  }

  LINE_SEGMENTER.with_borrow(|segmenter| {
    let mut start = 0;
    for point in segmenter.segment_str(text) {
      if point == 0 {
        continue;
      }
      if start < point {
        segments.push(text[start..point].to_string());
      }
      start = point;
    }

    if start < text.len() {
      segments.push(text[start..].to_string());
    }
  });
}

fn next_tab_stop(
  x: f32,
  line_left: f32,
  tab_stops: &[TabStop],
  default_tab_stop_pt: f32,
) -> ResolvedTabStop {
  let relative_x = (x - line_left).max(0.0);
  if let Some(stop) = tab_stops
    .iter()
    .copied()
    .find(|stop| stop.position_pt > relative_x + 0.1)
  {
    return ResolvedTabStop {
      x_pt: line_left + stop.position_pt,
      alignment: stop.alignment,
    };
  }

  let default_tab_stop_pt = default_tab_stop_pt.max(DEFAULT_FONT_SIZE_PT);
  ResolvedTabStop {
    x_pt: line_left + ((relative_x / default_tab_stop_pt).floor() + 1.0) * default_tab_stop_pt,
    alignment: TabStopAlignment::Left,
  }
}

fn aligned_tab_x(
  tab_stop: ResolvedTabStop,
  text_width: f32,
  line_left: f32,
  line_right: f32,
) -> f32 {
  let x = match tab_stop.alignment {
    TabStopAlignment::Left => tab_stop.x_pt,
    TabStopAlignment::Center => tab_stop.x_pt - text_width / 2.0,
    TabStopAlignment::Right => tab_stop.x_pt - text_width,
  };
  x.clamp(line_left, line_right)
}

fn decorate_paragraph(
  page: &mut Page,
  start_item_index: usize,
  paragraph: &crate::docx::Paragraph,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
) {
  let padding = 2.0;
  let x = x - padding;
  let y = y - padding;
  let width = width + padding * 2.0;
  let height = height + padding * 2.0;

  if let Some(color) = paragraph.format.shading {
    page.items.insert(
      start_item_index,
      PageItem::Fill(FillItem {
        x_pt: x,
        y_pt: y,
        width_pt: width,
        height_pt: height,
        color,
      }),
    );
  }

  if let Some(border) = paragraph.format.borders.top {
    push_styled_line(page, x, y, x + width, y, border);
  }
  if let Some(border) = paragraph.format.borders.right {
    push_styled_line(page, x + width, y, x + width, y + height, border);
  }
  if let Some(border) = paragraph.format.borders.bottom {
    push_styled_line(page, x, y + height, x + width, y + height, border);
  }
  if let Some(border) = paragraph.format.borders.left {
    push_styled_line(page, x, y, x, y + height, border);
  }
}

fn align_paragraph_items(items: &mut [PageItem], alignment: ParagraphAlignment, line_right: f32) {
  if matches!(
    alignment,
    ParagraphAlignment::Left | ParagraphAlignment::Justify
  ) {
    return;
  }

  let mut line_ys = Vec::new();
  for item in items.iter() {
    let Some(y) = item_y(item) else {
      continue;
    };
    if !line_ys.iter().any(|seen| f32::abs(*seen - y) < 0.01) {
      line_ys.push(y);
    }
  }

  for y in line_ys {
    let mut min_x = f32::MAX;
    let mut max_x: f32 = 0.0;
    for item in items.iter() {
      if let Some(item_y) = item_y(item)
        && f32::abs(item_y - y) < 0.01
        && let Some((x, width)) = item_horizontal_bounds(item)
      {
        min_x = min_x.min(x);
        max_x = max_x.max(x + width);
      }
    }

    if min_x == f32::MAX || max_x <= min_x {
      continue;
    }

    let available = line_right - min_x;
    let line_width = max_x - min_x;
    let offset = match alignment {
      ParagraphAlignment::Center => (available - line_width).max(0.0) / 2.0,
      ParagraphAlignment::Right => (available - line_width).max(0.0),
      ParagraphAlignment::Left | ParagraphAlignment::Justify => 0.0,
    };
    if offset <= 0.0 {
      continue;
    }

    for item in items.iter_mut() {
      if let Some(item_y) = item_y(item)
        && f32::abs(item_y - y) < 0.01
      {
        shift_item_x(item, offset);
      }
    }
  }
}

fn item_y(item: &PageItem) -> Option<f32> {
  match item {
    PageItem::Text(text) => Some(text.y_pt),
    PageItem::Image(image) => Some(image.y_pt),
    PageItem::Fill(_) => None,
    PageItem::Line(_) => None,
  }
}

fn item_horizontal_bounds(item: &PageItem) -> Option<(f32, f32)> {
  match item {
    PageItem::Text(text) => Some((text.x_pt, measure_text(&text.text, text.style))),
    PageItem::Image(image) => Some((image.x_pt, image.width_pt)),
    PageItem::Fill(_) => None,
    PageItem::Line(_) => None,
  }
}

fn shift_item_x(item: &mut PageItem, offset: f32) {
  match item {
    PageItem::Text(text) => text.x_pt += offset,
    PageItem::Image(image) => image.x_pt += offset,
    PageItem::Fill(_) => {}
    PageItem::Line(_) => {}
  }
}

fn fit_image_to_line(width: f32, height: f32, max_width: f32) -> (f32, f32) {
  let width = width.max(1.0);
  let height = height.max(1.0);
  if width <= max_width {
    (width, height)
  } else {
    let scale = max_width.max(1.0) / width;
    (width * scale, height * scale)
  }
}

fn next_line(
  setup: PageSetup,
  current: &mut Page,
  pages: &mut Vec<Page>,
  y: f32,
  line_height: &mut f32,
  content_bottom: f32,
) -> f32 {
  let mut next_y = y + *line_height;
  *line_height = DEFAULT_LINE_HEIGHT_PT;
  if next_y + *line_height > content_bottom && !current.items.is_empty() {
    pages.push(std::mem::replace(
      current,
      empty_page(setup, current.section_index),
    ));
    next_y = setup.margin_top_pt;
  }
  next_y
}

fn force_page_break(setup: PageSetup, current: &mut Page, pages: &mut Vec<Page>) -> f32 {
  if !current.items.is_empty() {
    pages.push(std::mem::replace(
      current,
      empty_page(setup, current.section_index),
    ));
  }
  setup.margin_top_pt
}

fn flush_text(page: &mut Page, x: f32, y: f32, chunk: &mut String, style: TextStyle) {
  if chunk.is_empty() {
    return;
  }

  page.items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    text: std::mem::take(chunk),
    style,
  }));
}

fn push_styled_line(page: &mut Page, x1: f32, y1: f32, x2: f32, y2: f32, border: BorderStyle) {
  page.items.push(PageItem::Line(LineItem {
    x1_pt: x1,
    y1_pt: y1,
    x2_pt: x2,
    y2_pt: y2,
    width_pt: border.width_pt,
    color: border.color,
  }));
}

fn clip_to_width(text: &str, style: TextStyle, max_width: f32) -> String {
  let mut width = 0.0;
  let mut clipped = String::new();
  for ch in text.chars() {
    let mut encoded = [0; 4];
    let value = if ch == '\t' {
      "    "
    } else if ch == '\n' {
      break;
    } else {
      ch.encode_utf8(&mut encoded)
    };
    let next = measure_text(value, style);
    if width + next > max_width && !clipped.is_empty() {
      break;
    }
    width += next;
    clipped.push_str(value);
  }
  clipped
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn repeating_areas_follow_word_margin_distances() {
    let setup = PageSetup::default();

    let header = header_area(setup);
    let footer = footer_area(setup);

    assert_eq!(header.top_pt, 36.0);
    assert_eq!(header.bottom_pt, 72.0);
    assert_eq!(footer.top_pt, 720.0);
    assert_eq!(footer.bottom_pt, 756.0);
  }

  #[test]
  fn repeating_areas_keep_minimum_height_when_distances_overlap_margins() {
    let setup = PageSetup {
      margin_top_pt: 20.0,
      margin_bottom_pt: 20.0,
      header_distance_pt: 30.0,
      footer_distance_pt: 30.0,
      ..Default::default()
    };

    let header = header_area(setup);
    let footer = footer_area(setup);

    assert!(header.bottom_pt - header.top_pt + 0.001 >= MIN_HEADER_FOOTER_HEIGHT_PT);
    assert!(footer.bottom_pt - footer.top_pt + 0.001 >= MIN_HEADER_FOOTER_HEIGHT_PT);
  }

  #[test]
  fn even_odd_section_breaks_insert_blank_pages_for_page_parity() {
    assert!(needs_section_parity_blank(SectionBreakKind::EvenPage, 3));
    assert!(!needs_section_parity_blank(SectionBreakKind::EvenPage, 4));
    assert!(needs_section_parity_blank(SectionBreakKind::OddPage, 2));
    assert!(!needs_section_parity_blank(SectionBreakKind::OddPage, 3));
    assert!(!needs_section_parity_blank(SectionBreakKind::NextPage, 3));
  }
}
