use std::cell::RefCell;
use std::collections::HashSet;

use icu_segmenter::LineSegmenter;

use crate::docx::{
  Block, BorderStyle, DocxDocument, DynamicFieldKind, FloatingImagePlacement,
  HorizontalImageReference, ImageWrapMode, InlineItem, PageSetup, ParagraphAlignment, RgbColor,
  SectionBreakKind, SectionColumns, TabStop, TabStopAlignment, Table, TableAlignment, TableCell,
  TableCellVerticalAlignment, TableRow, TextStyle, VerticalImageReference,
};
use crate::error::Result;
use crate::options::PdfOptions;
use crate::text_metrics::measure_text;

const PARAGRAPH_SPACING_AFTER_PT: f32 = 6.0;
const DEFAULT_TAB_STOP_PT: f32 = 36.0;
const DEFAULT_FONT_SIZE_PT: f32 = 11.0;
const DEFAULT_LINE_HEIGHT_PT: f32 = 14.0;
const TABLE_ROW_MIN_HEIGHT_PT: f32 = 24.0;
const TABLE_SPACING_AFTER_PT: f32 = 6.0;
const MIN_HEADER_FOOTER_HEIGHT_PT: f32 = 72.0 / 25.4;
const FOOTNOTE_AREA_MAX_FRACTION: f32 = 0.4;
const DEFAULT_ORPHAN_LINES: usize = 2;
const DEFAULT_WIDOW_LINES: usize = 2;

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
  pub hyperlink_url: Option<String>,
  pub dynamic_field: Option<DynamicFieldKind>,
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
  pub behind_text: bool,
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
struct WrapExclusion {
  left_pt: f32,
  right_pt: f32,
  top_pt: f32,
  bottom_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct BlockArea {
  setup: PageSetup,
  section_index: usize,
  column_index: usize,
  columns: SectionColumns,
  content_left_pt: f32,
  content_bottom: f32,
  content_width: f32,
  default_tab_stop_pt: f32,
}

pub(crate) fn layout(document: &DocxDocument, _options: &PdfOptions) -> Result<LayoutDocument> {
  Ok(RootFrameLayout::new(document).format())
}

struct RootFrameLayout<'a> {
  document: &'a DocxDocument,
  pages: Vec<Page>,
  current: Page,
  y: f32,
  emitted_footnotes: HashSet<i64>,
}

impl<'a> RootFrameLayout<'a> {
  fn new(document: &'a DocxDocument) -> Self {
    Self {
      document,
      pages: Vec::new(),
      current: empty_page(document.page, 0),
      y: document.page.margin_top_pt,
      emitted_footnotes: HashSet::new(),
    }
  }

  fn format(mut self) -> LayoutDocument {
    self.format_body_frames();
    self.format_trailing_note_frames();
    self.finish_current_page();

    apply_page_backgrounds(&mut self.pages);
    place_behind_text_images(&mut self.pages);
    apply_column_separators(self.document, &mut self.pages);
    apply_headers_and_footers(self.document, &mut self.pages);
    apply_page_borders(&mut self.pages);
    resolve_dynamic_fields(&mut self.pages);

    LayoutDocument { pages: self.pages }
  }

  fn format_body_frames(&mut self) {
    if self.document.sections.is_empty() {
      let blocks = self.document.blocks.clone();
      let flow = self.body_flow(document_page_frame(
        self.document.page,
        0,
        SectionColumns::default(),
      ));
      self.format_block_sequence(&blocks, flow);
      return;
    }

    for section_index in 0..self.document.sections.len() {
      let section = self.document.sections[section_index].clone();
      self.start_section_frame(section_index, &section);
      let flow = self.body_flow(document_page_frame(
        section.page,
        section_index,
        section.columns,
      ));
      self.format_block_sequence(&section.blocks, flow);
    }
  }

  fn body_flow(&self, frame: BodyFrame) -> FlowContext {
    flow_context(
      frame.setup,
      frame.section_index,
      frame.columns,
      0,
      self.document.default_tab_stop_pt,
    )
  }

  fn start_section_frame(&mut self, section_index: usize, section: &crate::docx::ImportedSection) {
    if section_index > 0 && starts_new_page(section.break_kind) && !self.current.items.is_empty() {
      self.push_current_page(empty_page(section.page, section_index));
      if needs_section_parity_blank(section.break_kind, self.pages.len() + 1) {
        self.pages.push(empty_page(section.page, section_index));
      }
      self.y = section.page.margin_top_pt;
    } else if self.current.items.is_empty() {
      self.current.setup = section.page;
      self.current.section_index = section_index;
      self.y = section.page.margin_top_pt;
    }
  }

  fn format_block_sequence(&mut self, blocks: &[Block], mut flow: FlowContext) {
    for (index, block) in blocks.iter().enumerate() {
      let next = blocks.get(index + 1);
      self.format_block(block, next, &mut flow);
    }
  }

  fn format_block(&mut self, block: &Block, next: Option<&Block>, flow: &mut FlowContext) {
    let (block_flow, footnote_top) =
      footnote_boss_reserve(block, self.document, &self.emitted_footnotes, *flow);
    (*flow, self.y) = prepare_block_flow(
      block,
      next,
      block_flow,
      &mut self.current,
      &mut self.pages,
      self.y,
    );
    *flow = self.advance_if_past_body(*flow);
    (*flow, self.y) =
      layout_document_block(block, *flow, &mut self.current, &mut self.pages, self.y);
    footnote_boss_format(
      block,
      self.document,
      &mut self.emitted_footnotes,
      *flow,
      &mut self.current,
      &mut self.pages,
      footnote_top,
    );
    *flow = restore_body_content_bottom(*flow);
    *flow = self.advance_if_past_body(*flow);
  }

  fn advance_if_past_body(&mut self, flow: FlowContext) -> FlowContext {
    if self.y + DEFAULT_LINE_HEIGHT_PT > flow.content_bottom && !self.current.items.is_empty() {
      let (next_flow, next_y) = advance_section_flow(flow, &mut self.current, &mut self.pages);
      self.y = next_y;
      next_flow
    } else {
      flow
    }
  }

  fn format_trailing_note_frames(&mut self) {
    let note_setup = self.current.setup;
    let note_flow = flow_context(
      note_setup,
      self.current.section_index,
      SectionColumns::default(),
      0,
      self.document.default_tab_stop_pt,
    );

    if self.document.footnotes.is_empty() && !self.document.footnote_blocks.is_empty() {
      self.format_note_block_sequence(note_setup, note_flow, &self.document.footnote_blocks);
    }

    if !self.document.endnotes.is_empty() {
      self.y = layout_note_separator(
        note_setup,
        &mut self.current,
        &mut self.pages,
        self.y,
        note_flow.content_bottom,
      );
      let mut emitted_endnotes = HashSet::new();
      for id in document_referenced_endnote_ids(self.document) {
        if !emitted_endnotes.insert(id) {
          continue;
        }
        if let Some(blocks) = self.document.endnotes.get(&id) {
          self.format_blocks_in_flow(blocks, note_flow);
        }
      }
      for (id, blocks) in &self.document.endnotes {
        if !emitted_endnotes.contains(id) {
          self.format_blocks_in_flow(blocks, note_flow);
        }
      }
    } else if !self.document.endnote_blocks.is_empty() {
      self.format_note_block_sequence(note_setup, note_flow, &self.document.endnote_blocks);
    }

    if !self.document.comment_blocks.is_empty() {
      self.format_note_block_sequence(note_setup, note_flow, &self.document.comment_blocks);
    }
  }

  fn format_note_block_sequence(&mut self, setup: PageSetup, flow: FlowContext, blocks: &[Block]) {
    self.y = layout_note_separator(
      setup,
      &mut self.current,
      &mut self.pages,
      self.y,
      flow.content_bottom,
    );
    self.format_blocks_in_flow(blocks, flow);
  }

  fn format_blocks_in_flow(&mut self, blocks: &[Block], flow: FlowContext) {
    for block in blocks {
      let (_, y) = layout_document_block(block, flow, &mut self.current, &mut self.pages, self.y);
      self.y = y;
    }
  }

  fn finish_current_page(&mut self) {
    if !self.current.items.is_empty() || self.pages.is_empty() {
      self.pages.push(std::mem::replace(
        &mut self.current,
        empty_page(self.document.page, 0),
      ));
    }
  }

  fn push_current_page(&mut self, next: Page) {
    self.pages.push(std::mem::replace(&mut self.current, next));
  }
}

#[derive(Clone, Copy, Debug)]
struct BodyFrame {
  setup: PageSetup,
  section_index: usize,
  columns: SectionColumns,
}

fn document_page_frame(
  setup: PageSetup,
  section_index: usize,
  columns: SectionColumns,
) -> BodyFrame {
  BodyFrame {
    setup,
    section_index,
    columns,
  }
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
) -> (FlowContext, f32) {
  match block {
    Block::Paragraph(paragraph) => {
      let mut flow = flow;
      if paragraph.format.page_break_before && !current.items.is_empty() {
        pages.push(std::mem::replace(
          current,
          empty_page(flow.setup, flow.section_index),
        ));
        y = flow.setup.margin_top_pt;
        flow = flow_context(
          flow.setup,
          flow.section_index,
          flow.columns,
          0,
          flow.default_tab_stop_pt,
        );
      }

      y += paragraph.format.spacing_before_pt;
      let paragraph_flow = FlowContext {
        content_width: (flow.content_width
          - paragraph.format.indent_left_pt
          - paragraph.format.indent_right_pt)
          .max(DEFAULT_FONT_SIZE_PT),
        ..flow
      };
      let (paragraph_flow, y) = layout_paragraph(paragraph, paragraph_flow, current, pages, y);
      (
        FlowContext {
          content_width: flow.content_width,
          ..paragraph_flow
        },
        y,
      )
    }
    Block::Table(table) => layout_table(table, flow, current, pages, y),
  }
}

fn block_area(flow: FlowContext) -> BlockArea {
  BlockArea {
    setup: flow.setup,
    section_index: flow.section_index,
    column_index: flow.column_index,
    columns: flow.columns,
    content_left_pt: flow.content_left_pt,
    content_bottom: flow.content_bottom,
    content_width: flow.content_width,
    default_tab_stop_pt: flow.default_tab_stop_pt,
  }
}

fn flow_from_block_area(area: BlockArea) -> FlowContext {
  FlowContext {
    setup: area.setup,
    section_index: area.section_index,
    column_index: area.column_index,
    columns: area.columns,
    content_left_pt: area.content_left_pt,
    content_bottom: area.content_bottom,
    content_width: area.content_width,
    default_tab_stop_pt: area.default_tab_stop_pt,
  }
}

fn restore_body_content_bottom(flow: FlowContext) -> FlowContext {
  FlowContext {
    content_bottom: flow.setup.height_pt - flow.setup.margin_bottom_pt,
    ..flow
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

fn footnote_boss_reserve(
  block: &Block,
  document: &DocxDocument,
  emitted_footnotes: &HashSet<i64>,
  flow: FlowContext,
) -> (FlowContext, Option<f32>) {
  let height = referenced_footnote_area_height(block, document, emitted_footnotes, flow);
  if height <= 0.0 {
    return (flow, None);
  }

  let available_height = flow.content_bottom - flow.setup.margin_top_pt;
  let reserved_height = height.min(available_height * FOOTNOTE_AREA_MAX_FRACTION);
  let footnote_top = flow.content_bottom - reserved_height;
  (
    FlowContext {
      content_bottom: footnote_top,
      ..flow
    },
    Some(footnote_top),
  )
}

fn referenced_footnote_area_height(
  block: &Block,
  document: &DocxDocument,
  emitted_footnotes: &HashSet<i64>,
  flow: FlowContext,
) -> f32 {
  let Block::Paragraph(paragraph) = block else {
    return 0.0;
  };

  let mut height = 0.0;
  let mut has_note = false;
  for id in &paragraph.footnote_reference_ids {
    if emitted_footnotes.contains(id) {
      continue;
    }
    let Some(blocks) = document.footnotes.get(id) else {
      continue;
    };
    if !has_note {
      height += DEFAULT_LINE_HEIGHT_PT;
      has_note = true;
    }
    for block in blocks {
      height += estimated_block_height(block, flow);
    }
  }
  height
}

fn footnote_boss_format(
  block: &Block,
  document: &DocxDocument,
  emitted_footnotes: &mut HashSet<i64>,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  footnote_top: Option<f32>,
) {
  let Block::Paragraph(paragraph) = block else {
    return;
  };
  let Some(mut y) = footnote_top else {
    return;
  };

  let mut needs_separator = true;
  let footnote_flow = FlowContext {
    content_left_pt: flow.setup.margin_left_pt,
    content_width: flow.setup.width_pt - flow.setup.margin_left_pt - flow.setup.margin_right_pt,
    content_bottom: flow.setup.height_pt - flow.setup.margin_bottom_pt,
    columns: SectionColumns::default(),
    column_index: 0,
    ..flow
  };
  for id in &paragraph.footnote_reference_ids {
    if !emitted_footnotes.insert(*id) {
      continue;
    }
    let Some(blocks) = document.footnotes.get(id) else {
      continue;
    };
    if needs_separator {
      y = layout_note_separator(
        footnote_flow.setup,
        current,
        pages,
        y,
        footnote_flow.content_bottom,
      );
      needs_separator = false;
    }
    for block in blocks {
      let (_, next_y) = layout_document_block(block, footnote_flow, current, pages, y);
      y = next_y;
    }
  }
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

fn apply_page_backgrounds(pages: &mut [Page]) {
  for page in pages {
    let Some(color) = page.setup.background else {
      continue;
    };
    page.items.insert(
      0,
      PageItem::Fill(FillItem {
        x_pt: 0.0,
        y_pt: 0.0,
        width_pt: page.setup.width_pt,
        height_pt: page.setup.height_pt,
        color,
      }),
    );
  }
}

fn place_behind_text_images(pages: &mut [Page]) {
  for page in pages {
    let mut behind_images = Vec::new();
    let mut foreground_items = Vec::with_capacity(page.items.len());
    let mut background_count = 0;

    for item in page.items.drain(..) {
      if matches!(&item, PageItem::Image(image) if image.behind_text) {
        behind_images.push(item);
      } else {
        if is_page_background_item(&item, page.setup) {
          background_count += 1;
        }
        foreground_items.push(item);
      }
    }

    if behind_images.is_empty() {
      page.items = foreground_items;
      continue;
    }

    let mut ordered = Vec::with_capacity(foreground_items.len() + behind_images.len());
    ordered.extend(foreground_items.drain(..background_count));
    ordered.extend(behind_images);
    ordered.extend(foreground_items);
    page.items = ordered;
  }
}

fn is_page_background_item(item: &PageItem, setup: PageSetup) -> bool {
  matches!(
    item,
    PageItem::Fill(fill)
      if fill.x_pt == 0.0
        && fill.y_pt == 0.0
        && (fill.width_pt - setup.width_pt).abs() <= 0.1
        && (fill.height_pt - setup.height_pt).abs() <= 0.1
  )
}

fn apply_page_borders(pages: &mut [Page]) {
  for page in pages {
    let borders = page.setup.borders;
    if borders == crate::docx::CellBordersModel::default() {
      continue;
    }

    let (left, top, right, bottom) = page_border_reference_rect(page.setup);
    if let Some(border) = borders.top {
      let y = top + border.spacing_pt + border.width_pt / 2.0;
      push_styled_line(page, left, y, right, y, border);
    }
    if let Some(border) = borders.right {
      let x = right - border.spacing_pt - border.width_pt / 2.0;
      push_styled_line(page, x, top, x, bottom, border);
    }
    if let Some(border) = borders.bottom {
      let y = bottom - border.spacing_pt - border.width_pt / 2.0;
      push_styled_line(page, left, y, right, y, border);
    }
    if let Some(border) = borders.left {
      let x = left + border.spacing_pt + border.width_pt / 2.0;
      push_styled_line(page, x, top, x, bottom, border);
    }
  }
}

fn page_border_reference_rect(setup: PageSetup) -> (f32, f32, f32, f32) {
  if setup.borders_offset_from_text {
    (
      setup.margin_left_pt,
      setup.margin_top_pt,
      setup.width_pt - setup.margin_right_pt,
      setup.height_pt - setup.margin_bottom_pt,
    )
  } else {
    (0.0, 0.0, setup.width_pt, setup.height_pt)
  }
}

fn resolve_dynamic_fields(pages: &mut [Page]) {
  let total_pages = pages.len().to_string();
  for (page_index, page) in pages.iter_mut().enumerate() {
    let page_number = (page_index + 1).to_string();
    for item in &mut page.items {
      let PageItem::Text(text) = item else {
        continue;
      };
      match text.dynamic_field {
        Some(DynamicFieldKind::Page) => text.text.clone_from(&page_number),
        Some(DynamicFieldKind::NumPages) => text.text.clone_from(&total_pages),
        None => {}
      }
    }
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
    Block::Paragraph(paragraph) => {
      let (_, y) = layout_paragraph(
        paragraph,
        flow,
        page,
        discarded_pages,
        y + paragraph.format.spacing_before_pt,
      );
      y
    }
    Block::Table(table) => {
      let (_, y) = layout_table(table, flow, page, discarded_pages, y);
      y
    }
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
          hyperlink_url: None,
          dynamic_field: None,
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
  table: &Table,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  y: f32,
) -> (FlowContext, f32) {
  TableFrameLayout::new(table, block_area(flow))
    .map_or((flow, y), |layout| layout.format(current, pages, y))
}

#[derive(Clone, Debug)]
struct TableFrameLayout<'a> {
  table: &'a Table,
  frame: TableFrame,
}

impl<'a> TableFrameLayout<'a> {
  fn new(table: &'a Table, area: BlockArea) -> Option<Self> {
    let _table_cell_margins = table.cell_margins;
    let column_count = table_column_count(table);
    if column_count == 0 {
      return None;
    }

    let column_widths = table_column_widths(table, column_count, area.content_width);
    let table_width = column_widths.iter().sum::<f32>();
    let left_pt = table_left_position(table, area.content_left_pt, area.content_width, table_width);
    let repeating_header_count = table_repeating_header_count(table);
    let repeating_header_height = table.rows[..repeating_header_count]
      .iter()
      .map(|row| table_row_height_with_widths(row, &column_widths))
      .sum::<f32>();

    Some(Self {
      table,
      frame: TableFrame {
        block: area,
        column_widths,
        left_pt,
        right_pt: left_pt + table_width,
        repeating_header_count,
        repeating_header_height,
      },
    })
  }

  fn format(&self, current: &mut Page, pages: &mut Vec<Page>, mut y: f32) -> (FlowContext, f32) {
    let mut flow = flow_from_block_area(self.frame.block);
    let mut layout = self.clone();
    let mut rows_moved_to_follow = HashSet::new();
    for (row_index, row) in self.table.rows.iter().enumerate() {
      let mut row_frame = layout.row_frame(row, row_index, y);
      let row_height = row_frame.height_pt;
      let move_to_follow = layout.should_move_row_to_follow(
        &row_frame,
        current,
        rows_moved_to_follow.contains(&row_index),
      );
      if move_to_follow {
        rows_moved_to_follow.insert(row_index);
        (flow, y) = advance_section_flow(flow, current, pages);
        if let Some(next_layout) = TableFrameLayout::new(self.table, block_area(flow)) {
          layout = next_layout;
        }
        y = layout.format_repeated_header_rows(current, y, row_height);
        row_frame = layout.row_frame(row, row_index, y);
      }

      y = row_frame.format(current);
    }

    (flow, y + TABLE_SPACING_AFTER_PT)
  }

  fn row_frame(&self, row: &'a TableRow, row_index: usize, y: f32) -> RowFrame<'a, '_> {
    RowFrame {
      table: self.table,
      table_frame: &self.frame,
      row,
      row_index,
      y,
      height_pt: table_row_height_with_widths(row, &self.frame.column_widths),
    }
  }

  fn should_move_row_to_follow(
    &self,
    row: &RowFrame<'_, '_>,
    current: &Page,
    already_moved: bool,
  ) -> bool {
    if already_moved || row.bottom() <= self.frame.block.content_bottom || current.items.is_empty()
    {
      return false;
    }

    !row.row.cant_split
      || row.fits_empty_body_region()
      || row.y > self.frame.block.setup.margin_top_pt + 0.1
  }

  fn format_repeated_header_rows(&self, current: &mut Page, mut y: f32, row_height: f32) -> f32 {
    if self.frame.repeating_header_count == 0
      || y + self.frame.repeating_header_height + row_height > self.frame.block.content_bottom
    {
      return y;
    }

    for (header_index, header) in self
      .table
      .rows
      .iter()
      .take(self.frame.repeating_header_count)
      .enumerate()
    {
      y = self.row_frame(header, header_index, y).format(current);
    }
    y
  }
}

#[derive(Clone, Debug)]
struct TableFrame {
  block: BlockArea,
  column_widths: Vec<f32>,
  left_pt: f32,
  right_pt: f32,
  repeating_header_count: usize,
  repeating_header_height: f32,
}

struct RowFrame<'a, 'f> {
  table: &'a Table,
  table_frame: &'f TableFrame,
  row: &'a TableRow,
  row_index: usize,
  y: f32,
  height_pt: f32,
}

impl RowFrame<'_, '_> {
  fn bottom(&self) -> f32 {
    self.y + self.height_pt
  }

  fn fits_empty_body_region(&self) -> bool {
    self.height_pt
      <= self.table_frame.block.content_bottom - self.table_frame.block.setup.margin_top_pt
  }

  fn format(&self, current: &mut Page) -> f32 {
    let row_top = self.y;
    let row_bottom = self.bottom();

    let mut cell_left = self.table_frame.left_pt;
    let mut grid_index = 0;
    for (cell_index, cell) in self.row.cells.iter().enumerate() {
      let cell_frame = match self.cell_frame(cell, cell_index, cell_left, &mut grid_index) {
        Some(cell_frame) => cell_frame,
        None => break,
      };
      if !cell.vertical_merge_continue {
        cell_frame.format(current, row_top, row_bottom);
      }
      cell_left += cell_frame.width_pt;
    }

    self.paint_horizontal_borders(current, row_top, row_bottom);
    self.paint_trailing_border(current, row_top, row_bottom);
    row_bottom
  }

  fn cell_frame<'s>(
    &'s self,
    cell: &'s TableCell,
    cell_index: usize,
    left_pt: f32,
    grid_index: &mut usize,
  ) -> Option<CellFrame<'s, 's>> {
    if *grid_index >= self.table_frame.column_widths.len() {
      return None;
    }
    let span = cell.grid_span.max(1).min(
      self
        .table_frame
        .column_widths
        .len()
        .saturating_sub(*grid_index),
    );
    let width_pt = self.table_frame.column_widths[*grid_index..*grid_index + span]
      .iter()
      .sum::<f32>();
    *grid_index += span;

    Some(CellFrame {
      table: self.table,
      table_frame: self.table_frame,
      row: self.row,
      cell,
      cell_index,
      left_pt,
      width_pt,
      height_pt: self.height_pt,
    })
  }

  fn paint_horizontal_borders(&self, current: &mut Page, row_top: f32, row_bottom: f32) {
    if let Some(border) = horizontal_border(self.table, self.row_index, true) {
      push_styled_line(
        current,
        self.table_frame.left_pt,
        row_top,
        self.table_frame.right_pt,
        row_top,
        border,
      );
    }
    if let Some(border) = horizontal_border(self.table, self.row_index, false) {
      push_styled_line(
        current,
        self.table_frame.left_pt,
        row_bottom,
        self.table_frame.right_pt,
        row_bottom,
        border,
      );
    }
  }

  fn paint_trailing_border(&self, current: &mut Page, row_top: f32, row_bottom: f32) {
    if let Some(border) = self
      .row
      .cells
      .last()
      .and_then(|cell| cell.borders.right)
      .or_else(|| self.table.borders.and_then(|borders| borders.right))
      .or(Some(BorderStyle::default()))
    {
      push_styled_line(
        current,
        self.table_frame.right_pt,
        row_top,
        self.table_frame.right_pt,
        row_bottom,
        border,
      );
    }
  }
}

struct CellFrame<'a, 'f> {
  table: &'a Table,
  table_frame: &'f TableFrame,
  row: &'a TableRow,
  cell: &'a TableCell,
  cell_index: usize,
  left_pt: f32,
  width_pt: f32,
  height_pt: f32,
}

impl CellFrame<'_, '_> {
  fn format(&self, current: &mut Page, row_top: f32, row_bottom: f32) {
    self.paint_background(current, row_top);
    self.paint_leading_border(current, row_top, row_bottom);
    layout_table_cell(
      self.cell,
      self.table_frame.block.setup,
      current,
      self.left_pt,
      row_top,
      self.width_pt,
      self.height_pt,
    );
  }

  fn paint_background(&self, current: &mut Page, row_top: f32) {
    if let Some(color) = self.cell.shading {
      current.items.push(PageItem::Fill(FillItem {
        x_pt: self.left_pt,
        y_pt: row_top,
        width_pt: self.width_pt,
        height_pt: self.height_pt,
        color,
      }));
    }
  }

  fn paint_leading_border(&self, current: &mut Page, row_top: f32, row_bottom: f32) {
    if let Some(border) = vertical_border(self.table, self.row, self.cell_index, true) {
      push_styled_line(
        current,
        self.left_pt,
        row_top,
        self.left_pt,
        row_bottom,
        border,
      );
    }
  }
}

fn table_column_count(table: &Table) -> usize {
  table
    .rows
    .iter()
    .map(|row| row.cells.iter().map(|cell| cell.grid_span).sum::<usize>())
    .max()
    .unwrap_or(0)
}

fn table_repeating_header_count(table: &Table) -> usize {
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

fn horizontal_border(table: &Table, row_index: usize, top_edge: bool) -> Option<BorderStyle> {
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
  table: &Table,
  row: &TableRow,
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

fn table_column_widths(table: &Table, column_count: usize, content_width: f32) -> Vec<f32> {
  let preferred_width = table_preferred_width(table, content_width);
  if table.column_widths_pt.len() >= column_count {
    let mut widths = table.column_widths_pt[..column_count].to_vec();
    if let Some(preferred) = preferred_width
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

  let width = preferred_width.unwrap_or(content_width).min(content_width);
  vec![width / column_count as f32; column_count]
}

fn table_preferred_width(table: &Table, content_width: f32) -> Option<f32> {
  table
    .preferred_width_pt
    .or_else(|| table.preferred_width_pct.map(|pct| content_width * pct))
    .map(|width| width.min(content_width).max(0.0))
}

fn table_left_position(
  table: &Table,
  content_left: f32,
  content_width: f32,
  table_width: f32,
) -> f32 {
  let available = (content_width - table_width).max(0.0);
  match table.alignment {
    TableAlignment::Left => content_left + table.indent_left_pt.min(available),
    TableAlignment::Center => content_left + available / 2.0,
    TableAlignment::Right => content_left + available,
  }
}

fn table_row_height(row: &TableRow) -> f32 {
  table_row_height_with_widths(row, &[])
}

fn table_row_height_with_widths(row: &TableRow, column_widths: &[f32]) -> f32 {
  let mut grid_index = 0;
  let mut content_height = TABLE_ROW_MIN_HEIGHT_PT;
  for cell in &row.cells {
    let width = spanned_cell_width(cell, column_widths, &mut grid_index);
    if !cell.vertical_merge_continue {
      content_height = content_height.max(table_cell_content_height(cell, width));
    }
  }
  match (row.height_pt, row.exact_height) {
    (Some(height), true) => height,
    (Some(height), false) => content_height.max(height),
    (None, _) => content_height,
  }
}

fn spanned_cell_width(cell: &TableCell, column_widths: &[f32], grid_index: &mut usize) -> f32 {
  if column_widths.is_empty() || *grid_index >= column_widths.len() {
    return 0.0;
  }
  let span = cell
    .grid_span
    .max(1)
    .min(column_widths.len().saturating_sub(*grid_index));
  let width = column_widths[*grid_index..*grid_index + span]
    .iter()
    .sum::<f32>();
  *grid_index += span;
  width
}

fn layout_table_cell(
  cell: &TableCell,
  setup: PageSetup,
  page: &mut Page,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
) {
  let content_width =
    (width - cell.margins.left_pt - cell.margins.right_pt).max(DEFAULT_FONT_SIZE_PT);
  let content_height = table_cell_content_height(cell, width);
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
  let text_bottom = y + height - cell.margins.bottom_pt;
  let flow = FlowContext {
    setup,
    section_index: page.section_index,
    column_index: 0,
    columns: SectionColumns::default(),
    content_left_pt: text_left,
    content_bottom: text_bottom,
    content_width,
    default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
  };
  let mut nested_page = empty_page(setup, page.section_index);
  let mut discarded_pages = Vec::new();

  for block in &cell.blocks {
    if text_y > text_bottom {
      break;
    }
    let (_, next_y) =
      layout_document_block(block, flow, &mut nested_page, &mut discarded_pages, text_y);
    text_y = next_y;
  }

  page.items.extend(
    nested_page
      .items
      .into_iter()
      .filter(|item| item_y(item).is_none_or(|item_y| item_y <= text_bottom)),
  );
  let _overflow_pages = discarded_pages;
}

fn table_cell_content_height(cell: &TableCell, cell_width: f32) -> f32 {
  let content_width =
    (cell_width - cell.margins.left_pt - cell.margins.right_pt).max(DEFAULT_FONT_SIZE_PT);
  let flow = FlowContext {
    setup: PageSetup::default(),
    section_index: 0,
    column_index: 0,
    columns: SectionColumns::default(),
    content_left_pt: 0.0,
    content_bottom: f32::MAX / 4.0,
    content_width,
    default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
  };
  let content = cell
    .blocks
    .iter()
    .map(|block| match block {
      Block::Paragraph(paragraph) => estimated_paragraph_height(paragraph, flow),
      Block::Table(table) => table
        .rows
        .iter()
        .map(table_row_height)
        .sum::<f32>()
        .max(TABLE_ROW_MIN_HEIGHT_PT),
    })
    .sum::<f32>()
    .max(DEFAULT_LINE_HEIGHT_PT);
  cell.margins.top_pt + content + cell.margins.bottom_pt
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
  y: f32,
) -> (FlowContext, f32) {
  TextFrameLayout::new(paragraph, flow).format(current, pages, y)
}

#[derive(Clone, Copy, Debug)]
struct TextFrame {
  setup: PageSetup,
  default_line_left: f32,
  first_line_left: f32,
  default_line_right: f32,
  paragraph_left: f32,
  base_line_height: f32,
}

impl TextFrame {
  fn new(paragraph: &crate::docx::Paragraph, flow: FlowContext) -> Self {
    let default_line_left = flow.content_left_pt + paragraph.format.indent_left_pt;
    let first_line_left =
      (default_line_left + paragraph.format.first_line_indent_pt).max(flow.content_left_pt);
    Self {
      setup: flow.setup,
      default_line_left,
      first_line_left,
      default_line_right: default_line_left + flow.content_width,
      paragraph_left: default_line_left.min(first_line_left),
      base_line_height: paragraph
        .format
        .line_height_pt
        .unwrap_or(DEFAULT_LINE_HEIGHT_PT),
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct LineFrame {
  left_pt: f32,
  right_pt: f32,
  y_pt: f32,
  height_pt: f32,
  x_pt: f32,
}

impl LineFrame {
  fn first(text_frame: TextFrame, y: f32, has_list_label: bool) -> Self {
    let left_pt = if has_list_label {
      text_frame.default_line_left
    } else {
      text_frame.first_line_left
    };
    Self {
      left_pt,
      right_pt: text_frame.default_line_right,
      y_pt: y,
      height_pt: text_frame.base_line_height,
      x_pt: left_pt,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
struct InlineCursor {
  inline_index: usize,
  text_offset: usize,
}

impl InlineCursor {
  fn after_inline(inline_index: usize) -> Self {
    Self {
      inline_index: inline_index + 1,
      text_offset: 0,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct LineFragment {
  start: InlineCursor,
  end: InlineCursor,
  y_pt: f32,
  height_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct TextFrameFollow {
  start: InlineCursor,
  page_index: usize,
  y_pt: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TextSplitDecision {
  NoSplit,
  Forced,
  Allowed,
  Rejected,
}

#[derive(Clone, Debug)]
struct TextFrameState {
  current_line_start: InlineCursor,
  current_position: InlineCursor,
  line_fragments: Vec<LineFragment>,
  page_follows: Vec<TextFrameFollow>,
}

impl TextFrameState {
  fn new() -> Self {
    Self {
      current_line_start: InlineCursor::default(),
      current_position: InlineCursor::default(),
      line_fragments: Vec::new(),
      page_follows: Vec::new(),
    }
  }

  fn set_position(&mut self, cursor: InlineCursor) {
    self.current_position = cursor;
  }

  fn finish_line(&mut self, y_pt: f32, height_pt: f32) {
    self.line_fragments.push(LineFragment {
      start: self.current_line_start,
      end: self.current_position,
      y_pt,
      height_pt,
    });
    self.current_line_start = self.current_position;
  }

  fn note_page_follow(&mut self, page_index: usize, y_pt: f32) {
    self.page_follows.push(TextFrameFollow {
      start: self.current_line_start,
      page_index,
      y_pt,
    });
  }

  fn finish_paragraph(&mut self, y_pt: f32, height_pt: f32, emitted: bool) {
    if emitted && self.current_position >= self.current_line_start {
      self.finish_line(y_pt, height_pt);
    }
  }

  fn page_split_decision(
    &self,
    keep_lines: bool,
    orphan_lines: usize,
    widow_lines: usize,
  ) -> TextSplitDecision {
    let Some(follow) = self.page_follows.last() else {
      return TextSplitDecision::NoSplit;
    };
    if keep_lines {
      return TextSplitDecision::Rejected;
    }

    let before = self
      .line_fragments
      .iter()
      .filter(|line| line.end <= follow.start && line.end > line.start)
      .count();
    let after = self
      .line_fragments
      .iter()
      .filter(|line| line.start >= follow.start && line.end > line.start)
      .count();

    if before == 0 || after == 0 {
      TextSplitDecision::Forced
    } else if before >= orphan_lines && after >= widow_lines {
      TextSplitDecision::Allowed
    } else {
      TextSplitDecision::Rejected
    }
  }

  fn split_candidates_are_ordered(&self) -> bool {
    self
      .line_fragments
      .windows(2)
      .all(|lines| lines[0].end <= lines[1].start)
      && self
        .line_fragments
        .iter()
        .all(|line| line.start <= line.end && line.y_pt.is_finite() && line.height_pt >= 0.0)
      && self
        .page_follows
        .iter()
        .all(|follow| follow.y_pt.is_finite())
      && self.page_follows.windows(2).all(|follows| {
        follows[0].start <= follows[1].start && follows[0].page_index <= follows[1].page_index
      })
  }
}

#[derive(Clone, Debug)]
struct TextSegment {
  text: String,
  start: usize,
  end: usize,
}

#[derive(Clone, Copy, Debug)]
struct ActiveTextFrame {
  flow: FlowContext,
  frame: TextFrame,
}

struct TextLineAdvance<'a> {
  current: &'a mut Page,
  pages: &'a mut Vec<Page>,
  wrap_exclusions: &'a [WrapExclusion],
  state: &'a mut TextFrameState,
  active: ActiveTextFrame,
}

struct TextFrameLayout<'a> {
  paragraph: &'a crate::docx::Paragraph,
  flow: FlowContext,
  frame: TextFrame,
}

impl<'a> TextFrameLayout<'a> {
  fn new(paragraph: &'a crate::docx::Paragraph, flow: FlowContext) -> Self {
    Self {
      paragraph,
      flow,
      frame: TextFrame::new(paragraph, flow),
    }
  }

  fn line_bounds(
    &self,
    frame: TextFrame,
    y: f32,
    line_height: f32,
    wrap_exclusions: &[WrapExclusion],
  ) -> (f32, f32) {
    line_bounds_for_y(
      frame.default_line_left,
      frame.default_line_right,
      y,
      line_height,
      wrap_exclusions,
    )
  }

  fn advance_line(
    &self,
    advance: TextLineAdvance<'_>,
    y: f32,
    line_height: &mut f32,
  ) -> (FlowContext, TextFrame, f32, f32, f32) {
    advance.state.finish_line(y, *line_height);
    let mut next_y = y + *line_height;
    *line_height = advance.active.frame.base_line_height;
    let mut next_flow = advance.active.flow;
    let mut next_frame = advance.active.frame;
    if next_y + *line_height > advance.active.flow.content_bottom
      && !advance.current.items.is_empty()
    {
      (next_flow, next_y) =
        advance_section_flow(advance.active.flow, advance.current, advance.pages);
      next_frame = TextFrame::new(self.paragraph, next_flow);
      *line_height = next_frame.base_line_height;
      advance.state.note_page_follow(advance.pages.len(), next_y);
    }
    let (line_left, line_right) =
      self.line_bounds(next_frame, next_y, *line_height, advance.wrap_exclusions);
    (next_flow, next_frame, next_y, line_left, line_right)
  }

  fn force_text_page_break(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    wrap_exclusions: &mut Vec<WrapExclusion>,
  ) -> (f32, f32, f32, f32) {
    let y = force_page_break(self.frame.setup, current, pages);
    wrap_exclusions.clear();
    (
      y,
      self.frame.first_line_left,
      self.frame.default_line_right,
      self.frame.base_line_height,
    )
  }

  fn apply_column_break(
    &self,
    flow: FlowContext,
    current: &mut Page,
    pages: &mut Vec<Page>,
    wrap_exclusions: &mut Vec<WrapExclusion>,
  ) -> (FlowContext, TextFrame, f32, f32, f32, f32, bool) {
    let (next_flow, y) = advance_section_flow(flow, current, pages);
    wrap_exclusions.clear();
    let next_frame = TextFrame::new(self.paragraph, next_flow);
    (
      next_flow,
      next_frame,
      y,
      next_frame.default_line_left,
      next_frame.default_line_right,
      next_frame.base_line_height,
      flow.columns.count > 1 && flow.column_index + 1 < flow.columns.count,
    )
  }

  fn format(&self, current: &mut Page, pages: &mut Vec<Page>, y: f32) -> (FlowContext, f32) {
    self.format_with_reflow(current, pages, y, true)
  }

  fn format_with_reflow(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    mut y: f32,
    allow_reflow: bool,
  ) -> (FlowContext, f32) {
    let paragraph = self.paragraph;
    let mut flow = self.flow;
    let mut text_frame = self.frame;
    let start_item_index = current.items.len();
    let start_pages_len = pages.len();
    let start_current = current.clone();
    let start_flow = flow;
    let paragraph_top = y;
    let default_line_left = text_frame.default_line_left;
    let first_line_left = text_frame.first_line_left;
    let mut default_line_right = text_frame.default_line_right;
    let mut paragraph_left = text_frame.paragraph_left;
    let mut wrap_exclusions = Vec::new();
    let mut emitted = paragraph.list_label.is_some();
    let mut pending_tab: Option<ResolvedTabStop> = None;
    let mut text_state = TextFrameState::new();
    let line = LineFrame::first(text_frame, y, paragraph.list_label.is_some());
    y = line.y_pt;
    let mut base_line_height = text_frame.base_line_height;
    let mut line_height = line.height_pt;
    let mut line_left = line.left_pt;
    let mut line_right = line.right_pt;
    let mut x = line.x_pt;
    if let Some(label) = &paragraph.list_label {
      current.items.push(PageItem::Text(TextItem {
        x_pt: first_line_left,
        y_pt: y,
        text: label.clone(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
      }));
      x = default_line_left;
    }

    for (inline_index, item) in paragraph.inlines.iter().enumerate() {
      match item {
        InlineItem::Text(run) => {
          let mut chunk = String::new();
          let mut chunk_x = x;

          for segment in text_segments_with_offsets(&run.text) {
            if segment.text == "\n" {
              text_state.set_position(InlineCursor {
                inline_index,
                text_offset: segment.end,
              });
              flush_text(
                current,
                chunk_x,
                y,
                &mut chunk,
                run.style,
                run.hyperlink_url.as_deref(),
                run.dynamic_field,
              );
              (flow, text_frame, y, line_left, line_right) = self.advance_line(
                TextLineAdvance {
                  current,
                  pages,
                  wrap_exclusions: &wrap_exclusions,
                  state: &mut text_state,
                  active: ActiveTextFrame {
                    flow,
                    frame: text_frame,
                  },
                },
                y,
                &mut line_height,
              );
              default_line_right = text_frame.default_line_right;
              paragraph_left = text_frame.paragraph_left;
              base_line_height = text_frame.base_line_height;
              x = line_left;
              chunk_x = x;
              pending_tab = None;
              emitted = true;
              continue;
            }
            if segment.text == "\t" {
              text_state.set_position(InlineCursor {
                inline_index,
                text_offset: segment.end,
              });
              flush_text(
                current,
                chunk_x,
                y,
                &mut chunk,
                run.style,
                run.hyperlink_url.as_deref(),
                run.dynamic_field,
              );
              let mut tab_stop = next_tab_stop(
                x,
                line_left,
                &paragraph.format.tab_stops,
                flow.default_tab_stop_pt,
              );
              x = tab_stop.x_pt;
              if x > line_right {
                (flow, text_frame, y, line_left, line_right) = self.advance_line(
                  TextLineAdvance {
                    current,
                    pages,
                    wrap_exclusions: &wrap_exclusions,
                    state: &mut text_state,
                    active: ActiveTextFrame {
                      flow,
                      frame: text_frame,
                    },
                  },
                  y,
                  &mut line_height,
                );
                default_line_right = text_frame.default_line_right;
                paragraph_left = text_frame.paragraph_left;
                base_line_height = text_frame.base_line_height;
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

            let width = measure_text(&segment.text, run.style);
            let line_capacity = (line_right - line_left).max(DEFAULT_FONT_SIZE_PT);
            let whitespace = segment.text.chars().all(char::is_whitespace);
            if let Some(tab_stop) = pending_tab.take()
              && !whitespace
            {
              x = aligned_tab_x(tab_stop, width, line_left, line_right);
              chunk_x = x;
            }

            if x + width > line_right && x > line_left {
              flush_text(
                current,
                chunk_x,
                y,
                &mut chunk,
                run.style,
                run.hyperlink_url.as_deref(),
                run.dynamic_field,
              );
              (flow, text_frame, y, line_left, line_right) = self.advance_line(
                TextLineAdvance {
                  current,
                  pages,
                  wrap_exclusions: &wrap_exclusions,
                  state: &mut text_state,
                  active: ActiveTextFrame {
                    flow,
                    frame: text_frame,
                  },
                },
                y,
                &mut line_height,
              );
              default_line_right = text_frame.default_line_right;
              paragraph_left = text_frame.paragraph_left;
              base_line_height = text_frame.base_line_height;
              x = line_left;
              chunk_x = x;
              pending_tab = None;
              if whitespace {
                emitted = true;
                continue;
              }
            }

            if width > line_capacity && x <= line_left && !whitespace {
              let mut text_offset = segment.start;
              for text in emergency_break_segments(&segment.text) {
                let width = measure_text(&text, run.style);
                if width > line_capacity && text.chars().count() > 1 {
                  for ch in text.chars() {
                    let mut encoded = [0; 4];
                    let text = ch.encode_utf8(&mut encoded);
                    let width = measure_text(text, run.style);
                    text_offset += text.len();

                    if x + width > line_right && x > line_left {
                      flush_text(
                        current,
                        chunk_x,
                        y,
                        &mut chunk,
                        run.style,
                        run.hyperlink_url.as_deref(),
                        run.dynamic_field,
                      );
                      (flow, text_frame, y, line_left, line_right) = self.advance_line(
                        TextLineAdvance {
                          current,
                          pages,
                          wrap_exclusions: &wrap_exclusions,
                          state: &mut text_state,
                          active: ActiveTextFrame {
                            flow,
                            frame: text_frame,
                          },
                        },
                        y,
                        &mut line_height,
                      );
                      default_line_right = text_frame.default_line_right;
                      paragraph_left = text_frame.paragraph_left;
                      base_line_height = text_frame.base_line_height;
                      x = line_left;
                      chunk_x = x;
                      pending_tab = None;
                    }

                    if chunk.is_empty() {
                      chunk_x = x;
                    }
                    chunk.push_str(text);
                    x += width;
                    text_state.set_position(InlineCursor {
                      inline_index,
                      text_offset,
                    });
                    line_height = line_height.max(run.style.font_size_pt * 1.25);
                    emitted = true;
                  }
                  continue;
                }
                text_offset += text.len();

                if x + width > line_right && x > line_left {
                  flush_text(
                    current,
                    chunk_x,
                    y,
                    &mut chunk,
                    run.style,
                    run.hyperlink_url.as_deref(),
                    run.dynamic_field,
                  );
                  (flow, text_frame, y, line_left, line_right) = self.advance_line(
                    TextLineAdvance {
                      current,
                      pages,
                      wrap_exclusions: &wrap_exclusions,
                      state: &mut text_state,
                      active: ActiveTextFrame {
                        flow,
                        frame: text_frame,
                      },
                    },
                    y,
                    &mut line_height,
                  );
                  default_line_right = text_frame.default_line_right;
                  paragraph_left = text_frame.paragraph_left;
                  base_line_height = text_frame.base_line_height;
                  x = line_left;
                  chunk_x = x;
                  pending_tab = None;
                }

                if chunk.is_empty() {
                  chunk_x = x;
                }
                chunk.push_str(&text);
                x += width;
                text_state.set_position(InlineCursor {
                  inline_index,
                  text_offset,
                });
                line_height = line_height.max(run.style.font_size_pt * 1.25);
                emitted = true;
              }
              continue;
            }

            if chunk.is_empty() {
              chunk_x = x;
            }
            chunk.push_str(&segment.text);
            x += width;
            text_state.set_position(InlineCursor {
              inline_index,
              text_offset: segment.end,
            });
            line_height = line_height.max(run.style.font_size_pt * 1.25);
            emitted = true;
          }

          flush_text(
            current,
            chunk_x,
            y,
            &mut chunk,
            run.style,
            run.hyperlink_url.as_deref(),
            run.dynamic_field,
          );
        }
        InlineItem::Image(image) => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
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
              behind_text: placement.behind_text,
            }));
            match placement.wrap {
              ImageWrapMode::TopBottom | ImageWrapMode::None => {
                y = y.max(image_y + height + placement.margin_bottom_pt);
                if y + base_line_height > flow.content_bottom && !current.items.is_empty() {
                  (flow, y) = advance_section_flow(flow, current, pages);
                  text_frame = TextFrame::new(self.paragraph, flow);
                  text_state.note_page_follow(pages.len(), y);
                  wrap_exclusions.clear();
                  default_line_right = text_frame.default_line_right;
                  paragraph_left = text_frame.paragraph_left;
                  base_line_height = text_frame.base_line_height;
                  line_height = base_line_height;
                }
                (line_left, line_right) =
                  self.line_bounds(text_frame, y, line_height, &wrap_exclusions);
                x = line_left;
                line_height = base_line_height;
              }
              ImageWrapMode::Square | ImageWrapMode::Tight if !placement.behind_text => {
                wrap_exclusions.push(WrapExclusion {
                  left_pt: image_x - placement.margin_left_pt,
                  right_pt: image_x + width + placement.margin_right_pt,
                  top_pt: image_y - placement.margin_top_pt,
                  bottom_pt: image_y + height + placement.margin_bottom_pt,
                });
                (line_left, line_right) =
                  self.line_bounds(text_frame, y, line_height, &wrap_exclusions);
                x = x.max(line_left).min(line_right);
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
            (flow, text_frame, y, line_left, line_right) = self.advance_line(
              TextLineAdvance {
                current,
                pages,
                wrap_exclusions: &wrap_exclusions,
                state: &mut text_state,
                active: ActiveTextFrame {
                  flow,
                  frame: text_frame,
                },
              },
              y,
              &mut line_height,
            );
            default_line_right = text_frame.default_line_right;
            paragraph_left = text_frame.paragraph_left;
            base_line_height = text_frame.base_line_height;
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
            behind_text: false,
          }));
          x += width;
          line_height = line_height.max(height);
          emitted = true;
        }
        InlineItem::PageBreak => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          text_state.finish_line(y, line_height);
          (y, line_left, line_right, line_height) =
            self.force_text_page_break(current, pages, &mut wrap_exclusions);
          x = line_left;
          emitted = false;
          pending_tab = None;
        }
        InlineItem::ColumnBreak => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          text_state.finish_line(y, line_height);
          let column_emitted;
          (
            flow,
            text_frame,
            y,
            line_left,
            line_right,
            line_height,
            column_emitted,
          ) = self.apply_column_break(flow, current, pages, &mut wrap_exclusions);
          default_line_right = text_frame.default_line_right;
          paragraph_left = text_frame.paragraph_left;
          base_line_height = text_frame.base_line_height;
          x = line_left;
          emitted = column_emitted;
          pending_tab = None;
        }
      }
    }

    let paragraph_bottom;
    if emitted {
      text_state.finish_paragraph(y, line_height, emitted);
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
    debug_assert!(text_state.split_candidates_are_ordered());
    let split_decision = text_state.page_split_decision(
      paragraph.format.keep_lines,
      DEFAULT_ORPHAN_LINES,
      DEFAULT_WIDOW_LINES,
    );
    debug_assert!(
      !matches!(split_decision, TextSplitDecision::Rejected) || !text_state.page_follows.is_empty()
    );
    if allow_reflow && matches!(split_decision, TextSplitDecision::Rejected) {
      pages.truncate(start_pages_len);
      *current = start_current;
      let (follow_flow, follow_y) = advance_section_flow(start_flow, current, pages);
      return TextFrameLayout::new(paragraph, follow_flow)
        .format_with_reflow(current, pages, follow_y, false);
    }

    if paragraph.list_label.is_none() && start_item_index <= current.items.len() {
      align_paragraph_items(
        &mut current.items[start_item_index..],
        paragraph.format.alignment,
        default_line_right,
      );
    }
    if start_item_index <= current.items.len() {
      decorate_paragraph(
        current,
        start_item_index,
        paragraph,
        paragraph_left,
        paragraph_top,
        default_line_right - paragraph_left,
        paragraph_bottom - paragraph_top,
      );
    }

    (flow, y)
  }
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

fn text_segments_with_offsets(text: &str) -> Vec<TextSegment> {
  let mut offset = 0;
  text_segments(text)
    .into_iter()
    .map(|text| {
      let start = offset;
      offset += text.len();
      TextSegment {
        text,
        start,
        end: offset,
      }
    })
    .collect()
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

fn line_bounds_for_y(
  default_left: f32,
  default_right: f32,
  y: f32,
  line_height: f32,
  exclusions: &[WrapExclusion],
) -> (f32, f32) {
  let mut left = default_left;
  let mut right = default_right;
  for exclusion in exclusions {
    if y + line_height <= exclusion.top_pt || y >= exclusion.bottom_pt {
      continue;
    }
    if exclusion.right_pt <= default_left || exclusion.left_pt >= default_right {
      continue;
    }

    let exclude_left = exclusion.left_pt.max(default_left);
    let exclude_right = exclusion.right_pt.min(default_right);
    if exclude_left <= default_left {
      left = left.max(exclude_right);
    } else if exclude_right >= default_right {
      right = right.min(exclude_left);
    } else {
      let left_space = exclude_left - default_left;
      let right_space = default_right - exclude_right;
      if right_space >= left_space {
        left = left.max(exclude_right);
      } else {
        right = right.min(exclude_left);
      }
    }
  }

  if right - left < DEFAULT_FONT_SIZE_PT {
    (default_left, default_right)
  } else {
    (left, right)
  }
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

fn force_page_break(setup: PageSetup, current: &mut Page, pages: &mut Vec<Page>) -> f32 {
  if !current.items.is_empty() {
    pages.push(std::mem::replace(
      current,
      empty_page(setup, current.section_index),
    ));
  }
  setup.margin_top_pt
}

fn flush_text(
  page: &mut Page,
  x: f32,
  y: f32,
  chunk: &mut String,
  style: TextStyle,
  hyperlink_url: Option<&str>,
  dynamic_field: Option<DynamicFieldKind>,
) {
  if chunk.is_empty() {
    return;
  }

  page.items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    text: std::mem::take(chunk),
    style,
    hyperlink_url: hyperlink_url.map(ToString::to_string),
    dynamic_field,
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

  #[test]
  fn text_frame_split_policy_matches_orphan_and_widow_minimums() {
    let mut state = TextFrameState::new();
    for index in 0..4 {
      state.set_position(InlineCursor {
        inline_index: 0,
        text_offset: index + 1,
      });
      state.finish_line(10.0 + index as f32 * 14.0, 14.0);
      if index == 1 {
        state.note_page_follow(1, 10.0);
      }
    }

    assert_eq!(
      state.page_split_decision(false, DEFAULT_ORPHAN_LINES, DEFAULT_WIDOW_LINES),
      TextSplitDecision::Allowed
    );
    assert_eq!(
      state.page_split_decision(true, DEFAULT_ORPHAN_LINES, DEFAULT_WIDOW_LINES),
      TextSplitDecision::Rejected
    );
  }

  #[test]
  fn text_frame_split_policy_rejects_widow_line() {
    let mut state = TextFrameState::new();
    for index in 0..3 {
      state.set_position(InlineCursor {
        inline_index: 0,
        text_offset: index + 1,
      });
      state.finish_line(10.0 + index as f32 * 14.0, 14.0);
      if index == 1 {
        state.note_page_follow(1, 10.0);
      }
    }

    assert_eq!(
      state.page_split_decision(false, DEFAULT_ORPHAN_LINES, DEFAULT_WIDOW_LINES),
      TextSplitDecision::Rejected
    );
  }
}
