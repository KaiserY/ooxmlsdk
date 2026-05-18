use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use icu_segmenter::{LineSegmenter, LineSegmenterBorrowed, options::LineBreakOptions};

use crate::docx::{
  Block, BorderStyle, DocxDocument, DynamicFieldKind, FloatingFrame, FloatingFramePlacement,
  FloatingImagePlacement, FrameHeightRule, FrameHorizontalAlignment, FrameHorizontalAnchor,
  FrameVerticalAlignment, FrameVerticalAnchor, FrameWrapMode, HorizontalImageAlignment,
  HorizontalImageReference, ImageCrop, ImageWrapMode, ImageWrapSide, InlineItem, InlineShape,
  InlineShapeGeometry, LineHeightRule, PageSetup, ParagraphAlignment, RgbColor, SectionBreakKind,
  SectionColumns, TabStop, TabStopAlignment, Table, TableAlignment, TableCell,
  TableCellVerticalAlignment, TableRow, TextBoxVerticalAlignment, TextStyle,
  VerticalImageAlignment, VerticalImageReference,
};
use crate::error::Result;
use crate::options::PdfOptions;
use crate::text_metrics::{baseline_offset_in_line, inline_text_box_height, measure_text};
use crate::units;

// Word document defaults used by LibreOffice import/export are 11pt text,
// 0.5in tab stops, and widow/orphan control of two lines.
const PARAGRAPH_SPACING_AFTER_PT: f32 = 0.0;
const DEFAULT_TAB_STOP_PT: f32 = 36.0;
const DEFAULT_FONT_SIZE_PT: f32 = 11.0;
const DEFAULT_LINE_HEIGHT_PT: f32 = 14.0;
// Source: LibreOffice sw/inc/swtypes.hxx defines MINLAY as the minimal
// Writer frame size, including table rows.
const LO_MIN_FRAME_SIZE_PT: f32 = 23.0 / units::TWIPS_PER_POINT;
const TABLE_ROW_MIN_HEIGHT_PT: f32 = LO_MIN_FRAME_SIZE_PT;
const TABLE_SPACING_AFTER_PT: f32 = 0.0;
const DEFAULT_ORPHAN_LINES: usize = 2;
const DEFAULT_WIDOW_LINES: usize = 2;
const MOVE_BACKWARD_SUPPRESS_THRESHOLD: usize = 20;
const UNBOUNDED_LAYOUT_EXTENT_PT: f32 = f32::MAX / 4.0;
const MEASURE_SCRATCH_PAGE_HEIGHT_PT: f32 = UNBOUNDED_LAYOUT_EXTENT_PT;
const LAYOUT_EPSILON_PT: f32 = 0.1;
// Source: LibreOffice sw/source/writerfilter/dmapper/DomainMapper_Impl.cxx
// sets OOXML document defaults to proportional line spacing 115.
const LO_DOCUMENT_DEFAULT_LINE_SPACING_PERCENT: f32 = 115.0;
const PERCENT_SCALE: f32 = 100.0;
const LO_EMPTY_PARAGRAPH_FIRST_LINE_HEIGHT_PER_FONT_SIZE: f32 = 340.0 / 220.0;
const LO_DRAWING_ANCHOR_MARGIN_LINE_HEIGHT_PT: f32 = 288.0 / units::TWIPS_PER_POINT;
// Source: LibreOffice sw/source/core/layout/pagedesc.cxx
// SwPageFootnoteInfo defaults: line width 10 twips, relative width 25%,
// top/bottom distance 57 twips.
const LO_FOOTNOTE_SEPARATOR_WIDTH_FRACTION: f32 = 0.25;
const LO_FOOTNOTE_SEPARATOR_STROKE_PT: f32 = 10.0 / units::TWIPS_PER_POINT;
const LO_FOOTNOTE_SEPARATOR_TOP_DIST_PT: f32 = 57.0 / units::TWIPS_PER_POINT;
const LO_FOOTNOTE_SEPARATOR_BOTTOM_DIST_PT: f32 = 57.0 / units::TWIPS_PER_POINT;
// Source: LibreOffice sw/qa/core/layout/paintfrm.cxx and ftnfrm.cxx
// Word-style endnote separators are 2 inches wide, and inline endnotes keep
// 269 twips of separator area above the endnote text.
const LO_ENDNOTE_SEPARATOR_WIDTH_PT: f32 = 2880.0 / units::TWIPS_PER_POINT;
const LO_ENDNOTE_SEPARATOR_BOTTOM_DIST_PT: f32 = 269.0 / units::TWIPS_PER_POINT;
// Source: LibreOffice sw/source/core/text/itrform2.cxx keeps the laid-out
// SwLineLayout real height separate from the content-control GetCharRect()
// widget rectangle, whose block is expanded by 20 twips on both vertical
// sides in SwContentControlPortion::DescribePDFControl().
const LO_CONTENT_CONTROL_WIDGET_BLOCK_EXPANSION_PT: f32 = 40.0 / units::TWIPS_PER_POINT;

#[derive(Clone, Copy, Debug)]
enum NoteSeparatorKind {
  Footnote,
  Endnote,
  EndnoteContinuation,
}

fn inline_text_height(style: &TextStyle) -> f32 {
  inline_text_box_height(style)
}

fn paragraph_base_line_style(paragraph: &crate::docx::Paragraph) -> TextStyle {
  if let Some(style) = paragraph.inlines.iter().find_map(|inline| match inline {
    InlineItem::Text(run) if text_run_affects_line_height(&run.text) => Some(run.style.clone()),
    InlineItem::Text(_) => None,
    InlineItem::Image(_)
    | InlineItem::Shape(_)
    | InlineItem::FormWidgetStart(_)
    | InlineItem::FormWidgetEnd(_)
    | InlineItem::LastRenderedPageBreak
    | InlineItem::PageBreak
    | InlineItem::ColumnBreak => None,
  }) {
    return style;
  }

  paragraph_ignored_blank_line_style(paragraph).unwrap_or_else(|| paragraph.base_style.clone())
}

fn text_run_affects_line_height(text: &str) -> bool {
  text
    .chars()
    .any(|ch| ch != '\n' && ch != '\t' && !libreoffice_ignored_line_height_blank(ch))
}

fn paragraph_ignored_blank_line_style(paragraph: &crate::docx::Paragraph) -> Option<TextStyle> {
  let mut style = paragraph.base_style.clone();
  let mut found = false;
  for inline in &paragraph.inlines {
    let InlineItem::Text(run) = inline else {
      continue;
    };
    if text_run_affects_line_height(&run.text) {
      continue;
    }
    if !found || run.style.font_size_pt < style.font_size_pt {
      style.font_size_pt = run.style.font_size_pt;
    }
    found = true;
  }
  found.then_some(style)
}

fn paragraph_line_height_for_setup(
  paragraph: &crate::docx::Paragraph,
  base_line_style: &TextStyle,
  setup: PageSetup,
  text_segmentation: TextSegmentation,
) -> f32 {
  match paragraph.format.line_height_rule {
    LineHeightRule::Auto => {
      let line_height = paragraph
        .format
        .line_height_pt
        .map(|multiple| word_auto_line_height(base_line_style) * multiple)
        .unwrap_or_else(|| inline_text_height(base_line_style));
      // Source: LibreOffice sw/source/core/text/itrform2.cxx
      // SwTextFormatter::CalcRealHeight() uses the imported document grid
      // base height as the auto line real height in grid layout.
      if text_segmentation == TextSegmentation::Body {
        line_height.max(setup.doc_grid_line_pitch_pt.unwrap_or(0.0))
      } else {
        line_height
      }
    }
    LineHeightRule::AtLeast | LineHeightRule::Exact => paragraph
      .format
      .line_height_pt
      .unwrap_or_else(|| inline_text_height(base_line_style)),
  }
}

fn word_auto_line_height(style: &TextStyle) -> f32 {
  style.font_size_pt * LO_DOCUMENT_DEFAULT_LINE_SPACING_PERCENT / PERCENT_SCALE
}

fn libreoffice_empty_paragraph_first_line_height(style: &TextStyle) -> f32 {
  // Source: LibreOffice keeps VML/text anchor offsets based on an empty-paragraph
  // first-line box that is taller than the default 115% auto line height.
  style.font_size_pt * LO_EMPTY_PARAGRAPH_FIRST_LINE_HEIGHT_PER_FONT_SIZE
}

fn include_text_height(line_height: f32, text_frame: TextFrame, style: &TextStyle) -> f32 {
  match text_frame.line_height_rule {
    LineHeightRule::Exact => line_height,
    LineHeightRule::Auto | LineHeightRule::AtLeast => line_height.max(inline_text_height(style)),
  }
}

fn line_real_height(
  paragraph: &crate::docx::Paragraph,
  line_height: f32,
  has_content_control: bool,
) -> f32 {
  if paragraph.format.bidi && has_content_control {
    line_height + LO_CONTENT_CONTROL_WIDGET_BLOCK_EXPANSION_PT
  } else {
    line_height
  }
}

#[derive(Clone, Debug)]
pub(crate) struct LayoutDocument {
  pub pages: Vec<Page>,
  pub form_widgets: Vec<crate::docx::FormWidget>,
  pub follows: Vec<FrameFollow>,
  pub frames: Vec<LayoutFrame>,
  pub outline_entries: Vec<OutlineEntry>,
  pub page_replays: Vec<PageReplay>,
  pub page_replay_applications: Vec<PageReplayApplication>,
  pub backward_moves: Vec<BackwardMove>,
  pub layout_reruns: Vec<LayoutRerun>,
  pub page_invalidations: Vec<PageInvalidation>,
  pub reflow_executions: Vec<ReflowExecution>,
  pub reflow_requests: Vec<ReflowRequest>,
  pub restart_plan: Option<RestartPlan>,
}

#[derive(Clone, Debug)]
pub(crate) struct OutlineEntry {
  pub level: u8,
  pub text: String,
  pub page_index: usize,
  pub x_pt: f32,
  pub y_pt: f32,
  pub merged_hidden_separator: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FollowFrameKind {
  Paragraph,
  Table,
  Notes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FollowReason {
  KeepTogether,
  Overflow,
  ExplicitBreak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrameFollow {
  pub kind: FollowFrameKind,
  pub reason: FollowReason,
  pub block_index: Option<usize>,
  pub from_page_index: usize,
  pub to_page_index: usize,
  pub from_section_page_index: usize,
  pub to_section_page_index: usize,
  pub from_column_index: usize,
  pub to_column_index: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct LayoutFrame {
  pub kind: FollowFrameKind,
  pub block_index: Option<usize>,
  pub split_start: FrameCursor,
  pub split_end: FrameCursor,
  pub page_index: usize,
  pub section_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub items: Vec<PageItem>,
  pub item_start: usize,
  pub item_end: usize,
  pub bounds: Option<FrameBounds>,
  pub lines: Vec<LineBox>,
  pub fragments: Vec<FrameFragment>,
  pub influences: Vec<FrameInfluence>,
  pub invalidation: FrameInvalidation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FrameCursor {
  pub block_index: Option<usize>,
  pub kind: FrameCursorKind,
  pub inline_index: usize,
  pub text_offset: usize,
  pub row_index: usize,
  pub cell_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FrameCursorKind {
  BlockStart,
  Inline,
  TableRow,
  TableCell,
  BlockEnd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FrameFragmentKind {
  ParagraphLine,
  TableRow,
  TableCell,
  NoteLine,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FrameFragment {
  pub kind: FrameFragmentKind,
  pub split: FragmentSplitKind,
  pub index: usize,
  pub row_index: usize,
  pub cell_index: Option<usize>,
  pub item_start: usize,
  pub item_end: usize,
  pub bounds: Option<FrameBounds>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FragmentSplitKind {
  Complete,
  Master,
  Follow,
  RepeatedHeader,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FrameInfluenceKind {
  FootnoteReservation,
  FlyWrap,
  TableSplit,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FrameInfluence {
  pub kind: FrameInfluenceKind,
  pub count: usize,
  pub block_index: Option<usize>,
  pub item_start: usize,
  pub item_end: usize,
  pub bounds: Option<FrameBounds>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FrameInvalidation {
  Clean,
  PageItemsDecorated,
  NeedsReflow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReflowRequest {
  pub frame_index: usize,
  pub kind: FollowFrameKind,
  pub reason: ReflowReason,
  pub scope: ReflowScope,
  pub restart: FrameCursor,
  pub page_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub influence_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReflowReason {
  DecorationChangedItems,
  InsertionInfluenceChanged,
  InvalidBounds,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub(crate) enum ReflowScope {
  Frame,
  Column,
  Page,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PageInvalidation {
  pub page_index: usize,
  pub section_page_index: usize,
  pub first_frame_index: usize,
  pub reason: ReflowReason,
  pub scope: ReflowScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ReflowExecution {
  pub first_page_index: usize,
  pub request_count: usize,
  pub action: ReflowAction,
  pub scope: ReflowScope,
  pub suppressed_moves: usize,
  pub backward_moves: usize,
  pub page_replacements: usize,
  pub replayed_frames: usize,
  pub replayed_items: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct PageReplay {
  pub page_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub scope: ReflowScope,
  pub item_start: usize,
  pub item_end: usize,
  pub replacement_items: Vec<PageItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PageReplayApplication {
  pub page_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub scope: ReflowScope,
  pub item_start: usize,
  pub item_end: usize,
  pub replacement_count: usize,
  pub applied: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct BackwardMove {
  pub frame_index: usize,
  pub replay_start_frame_index: usize,
  pub from_page_index: usize,
  pub to_page_index: usize,
  pub from_section_page_index: usize,
  pub to_section_page_index: usize,
  pub scope: ReflowScope,
  pub reason: ReflowReason,
  pub suppressed: bool,
  pub replayed_frames: usize,
  pub replayed_items: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LayoutRerun {
  pub checkpoint_index: usize,
  pub section_index: usize,
  pub block_index: usize,
  pub page_index: usize,
  pub frame_index: usize,
  pub reason: ReflowReason,
  pub scope: ReflowScope,
  pub replaced_pages: usize,
  pub produced_pages: usize,
  pub produced_frames: usize,
  pub constraints: Vec<LayoutRerunConstraint>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LayoutRerunConstraint {
  pub kind: FrameInfluenceKind,
  pub scope: ReflowScope,
  pub bounds: Option<FrameBounds>,
  pub content_left_pt: f32,
  pub content_width: f32,
  pub content_bottom: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReflowAction {
  StabilizedRetainedDecorationItems,
  StabilizedInsertionInfluences,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RestartPlan {
  pub page_index: usize,
  pub frame_index: usize,
  pub block_index: Option<usize>,
  pub cursor: FrameCursor,
  pub reason: ReflowReason,
  pub scope: ReflowScope,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct MoveBackwardKey {
  frame_index: usize,
  page_index: usize,
  section_page_index: usize,
  column_index: usize,
  scope: ReflowScope,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FrameBounds {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct LineBox {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub item_start: usize,
  pub item_end: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Page {
  pub setup: PageSetup,
  pub section_index: usize,
  pub section_page_index: usize,
  pub items: Vec<PageItem>,
  preserve_empty: bool,
  frame_fragments: Vec<FrameFragment>,
  frame_influences: Vec<FrameInfluence>,
  wrap_exclusions: Vec<WrapExclusion>,
  repeating_wrap_exclusion_catalog: RepeatingWrapExclusionCatalog,
  repeating_wrap_exclusions: Vec<WrapExclusion>,
  pending_floating_table_follows: Vec<PendingFloatingTableFollow>,
}

#[derive(Clone, Debug)]
struct PendingFloatingTableFollow {
  setup: PageSetup,
  section_index: usize,
  section_page_index: usize,
  items: Vec<PageItem>,
  frame_fragments: Vec<FrameFragment>,
  frame_influences: Vec<FrameInfluence>,
  wrap_exclusions: Vec<WrapExclusion>,
}

#[derive(Clone, Debug)]
pub(crate) enum PageItem {
  Text(TextItem),
  Image(ImageItem),
  Rect(RectItem),
  Fill(FillItem),
  Line(LineItem),
  Polyline(PolylineItem),
}

#[derive(Clone, Debug)]
pub(crate) struct TextItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub line_height_pt: f32,
  pub text: String,
  pub style: TextStyle,
  pub hyperlink_url: Option<String>,
  pub dynamic_field: Option<DynamicFieldKind>,
  pub style_ref_keys: Vec<Arc<str>>,
  pub style_ref_text: Option<Arc<str>>,
  pub form_widget_id: Option<u32>,
  pub paragraph_bidi: bool,
  pub preserve_text_portion: bool,
  pub decoration_span_start_x_pt: Option<f32>,
  pub pdf_text_segmentation: PdfTextSegmentation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PdfTextSegmentation {
  Line,
  Portion,
}

#[derive(Clone, Debug)]
pub(crate) struct ImageItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub crop: ImageCrop,
  pub rotation_deg: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub data: Vec<u8>,
  pub content_type: Option<String>,
  pub alt_text: Option<String>,
  pub hyperlink_url: Option<String>,
  pub floating: bool,
  pub behind_text: bool,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RectItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub fill_color: Option<RgbColor>,
  pub fill_opacity: f32,
  pub stroke: Option<BorderStyle>,
  pub stroke_opacity: f32,
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
  pub kind: LineItemKind,
}

#[derive(Clone, Debug)]
pub(crate) struct PolylineItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub points: Vec<(f32, f32)>,
  pub closed: bool,
  pub fill_color: Option<RgbColor>,
  pub stroke: Option<BorderStyle>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LineItemKind {
  Stroke,
  FilledRect,
}

#[derive(Clone, Copy, Debug)]
struct FlowContext {
  setup: PageSetup,
  section_index: usize,
  section_page_index: usize,
  column_index: usize,
  columns: SectionColumns,
  content_top_pt: f32,
  content_left_pt: f32,
  content_bottom: f32,
  body_content_bottom_pt: f32,
  content_width: f32,
  layout_cell_bounds: Option<FrameBounds>,
  default_tab_stop_pt: f32,
  repeating_slots: RepeatingSlotState,
  text_segmentation: TextSegmentation,
  paragraph_spacing_context: ParagraphSpacingContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TextSegmentation {
  Body,
  RepeatingSlot,
  TableCell,
  DrawingLayer,
  Notes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ParagraphSpacingContext {
  Normal,
  SectionStart,
}

#[derive(Clone, Copy, Debug, Default)]
struct RepeatingSlotState {
  title_page: bool,
  even_and_odd_headers: bool,
  default_header: bool,
  default_footer: bool,
  first_header: bool,
  first_footer: bool,
  even_header: bool,
  even_footer: bool,
  default_header_height_pt: f32,
  default_footer_height_pt: f32,
  first_header_height_pt: f32,
  first_footer_height_pt: f32,
  even_header_height_pt: f32,
  even_footer_height_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct ResolvedTabStop {
  x_pt: f32,
  alignment: TabStopAlignment,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct WrapExclusion {
  left_pt: f32,
  right_pt: f32,
  top_pt: f32,
  bottom_pt: f32,
  side: ImageWrapSide,
  blocks_flow: bool,
}

impl WrapExclusion {
  fn overlaps_vertical_span(&self, top_pt: f32, bottom_pt: f32) -> bool {
    bottom_pt > self.top_pt && top_pt < self.bottom_pt
  }

  fn overlaps_horizontal_span(&self, left_pt: f32, right_pt: f32) -> bool {
    right_pt > self.left_pt && left_pt < self.right_pt
  }
}

#[derive(Clone, Debug, Default)]
struct RepeatingWrapExclusionCatalog {
  first_odd: Vec<WrapExclusion>,
  first_even: Vec<WrapExclusion>,
  even: Vec<WrapExclusion>,
  default: Vec<WrapExclusion>,
}

impl RepeatingWrapExclusionCatalog {
  fn selected(&self, section_page_index: usize, page_number: usize) -> &[WrapExclusion] {
    let even_page = page_number.is_multiple_of(2);
    if section_page_index == 0 {
      if even_page {
        &self.first_even
      } else {
        &self.first_odd
      }
    } else if even_page {
      &self.even
    } else {
      &self.default
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct BlockArea {
  setup: PageSetup,
  section_index: usize,
  section_page_index: usize,
  column_index: usize,
  columns: SectionColumns,
  content_top_pt: f32,
  content_left_pt: f32,
  content_bottom: f32,
  body_content_bottom_pt: f32,
  content_width: f32,
  default_tab_stop_pt: f32,
  repeating_slots: RepeatingSlotState,
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
  follows: Vec<FrameFollow>,
  frames: Vec<LayoutFrame>,
  outline_entries: Vec<OutlineEntry>,
  checkpoints: Vec<LayoutCheckpoint>,
  next_line_number: i16,
  pending_trailing_page_break: bool,
}

#[derive(Clone, Debug)]
struct LayoutCheckpoint {
  section_index: usize,
  block_index: usize,
  page_index: usize,
  y: f32,
  flow: FlowContext,
  pages: Vec<Page>,
  current: Page,
  emitted_footnotes: HashSet<i64>,
  follows: Vec<FrameFollow>,
  frames: Vec<LayoutFrame>,
  outline_entries: Vec<OutlineEntry>,
  next_line_number: i16,
}

impl<'a> RootFrameLayout<'a> {
  fn new(document: &'a DocxDocument) -> Self {
    Self {
      document,
      pages: Vec::new(),
      current: empty_page(document.page, 0),
      y: document.page.margin_top_pt,
      emitted_footnotes: HashSet::new(),
      follows: Vec::new(),
      frames: Vec::new(),
      outline_entries: Vec::new(),
      checkpoints: Vec::new(),
      next_line_number: document
        .page
        .line_numbering
        .map(|line_numbering| line_numbering.start)
        .unwrap_or(1),
      pending_trailing_page_break: false,
    }
  }

  fn format(mut self) -> LayoutDocument {
    self.format_body_frames();
    self.format_trailing_note_frames();
    self.finish_current_page();
    self.finish_pending_trailing_page_break();
    materialize_pending_floating_table_follows(&mut self.pages);

    let mut layout_reruns = Vec::new();
    let influence_reflow_requests = reflow_requests_for_frames(&self.frames, true);
    let (mut reflow_requests, mut reflow_executions, mut page_replays, mut backward_moves) =
      execute_reflow_requests(&mut self.frames, influence_reflow_requests);
    if let Some(rerun) = self.apply_checkpoint_rerun(&backward_moves) {
      layout_reruns.push(rerun);
      let influence_reflow_requests = reflow_requests_for_frames(&self.frames, true);
      (
        reflow_requests,
        reflow_executions,
        page_replays,
        backward_moves,
      ) = execute_reflow_requests(&mut self.frames, influence_reflow_requests);
    }
    let mut page_replay_applications = apply_page_replays(&mut self.pages, &page_replays);

    let page_item_counts_before_decoration = self
      .pages
      .iter()
      .map(|page| page.items.len())
      .collect::<Vec<_>>();
    apply_page_backgrounds(&mut self.pages);
    place_floating_images(&mut self.pages);
    apply_column_separators(self.document, &mut self.pages, &self.frames);
    apply_headers_and_footers(self.document, &mut self.pages);
    apply_page_borders(&mut self.pages);
    resolve_dynamic_fields(&mut self.pages);
    mark_decorated_frame_invalidations(
      &mut self.frames,
      &self.pages,
      &page_item_counts_before_decoration,
    );
    let decoration_reflow_requests = reflow_requests_for_frames(&self.frames, false);
    let (
      remaining_decoration_reflow_requests,
      decoration_reflow_executions,
      decoration_page_replays,
      decoration_backward_moves,
    ) = execute_reflow_requests(&mut self.frames, decoration_reflow_requests);
    page_replay_applications.extend(apply_page_replays(
      &mut self.pages,
      &decoration_page_replays,
    ));
    page_replays.extend(decoration_page_replays);
    backward_moves.extend(decoration_backward_moves);
    reflow_executions.extend(decoration_reflow_executions);
    reflow_requests.extend(remaining_decoration_reflow_requests);
    let page_invalidations = page_invalidations_for_reflow_requests(&reflow_requests);
    let restart_plan = restart_plan_for_page_invalidations(&self.frames, &page_invalidations);
    let page_count = self.pages.len();
    self
      .follows
      .retain(|follow| follow.from_page_index < page_count && follow.to_page_index < page_count);

    LayoutDocument {
      pages: self.pages,
      form_widgets: self.document.form_widgets.clone(),
      follows: self.follows,
      frames: self.frames,
      outline_entries: self.outline_entries,
      page_replays,
      page_replay_applications,
      backward_moves,
      layout_reruns,
      page_invalidations,
      reflow_executions,
      reflow_requests,
      restart_plan,
    }
  }

  fn format_body_frames(&mut self) {
    self.seed_current_repeating_wrap_exclusions();
    if self.document.sections.is_empty() {
      let blocks = self.document.blocks.clone();
      let flow = self.body_flow(document_page_frame(
        self.document.page,
        0,
        SectionColumns::default(),
      ));
      self.y = flow.content_top_pt;
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
      self.y = self.y.max(flow.content_top_pt);
      let previous_section_block = section_index
        .checked_sub(1)
        .and_then(|index| self.document.sections.get(index))
        .and_then(|section| section.blocks.last());
      if !blocks_have_visible_body_content(&section.blocks)
        && section_has_repeating_blocks(&section)
      {
        self.current.preserve_empty = true;
      }
      self.format_block_sequence_with_previous(&section.blocks, flow, previous_section_block);
    }
  }

  fn body_flow(&self, frame: BodyFrame) -> FlowContext {
    let flow = flow_context(
      frame.setup,
      frame.section_index,
      frame.columns,
      0,
      self.document.default_tab_stop_pt,
    );
    body_flow_for_page(
      FlowContext {
        repeating_slots: repeating_slot_state(self.document, frame.section_index),
        ..flow
      },
      self.pages.len() + 1,
    )
  }

  fn start_section_frame(&mut self, section_index: usize, section: &crate::docx::ImportedSection) {
    let current_page_has_body_progress = self.current_page_has_body_progress();
    let reuse_empty_page = self.current.items.is_empty()
      && (section_index == 0
        || section.break_kind == SectionBreakKind::Continuous
        || (section.break_kind == SectionBreakKind::NextPage
          && self.current.section_page_index > 0)
        || (!starts_new_page(section.break_kind) && !current_page_has_body_progress));
    if reuse_empty_page {
      self.current.setup = section.page;
      self.current.section_index = section_index;
      self.current.section_page_index = 0;
      activate_pending_floating_table_follows_for_current(&mut self.current, &mut self.pages);
      self.seed_current_repeating_wrap_exclusions();
      self.y = body_content_limits_for_page(
        section.page,
        repeating_slot_state(self.document, section_index),
        self.pages.len() + 1,
        0,
      )
      .0;
    } else if section_index > 0
      && starts_new_page(section.break_kind)
      && current_page_has_body_progress
    {
      self.push_current_page(empty_page(section.page, section_index));
      let mut section_page_index = 0;
      if needs_section_parity_blank(section.break_kind, self.pages.len() + 1) {
        self.pages.push(empty_page(section.page, section_index));
        section_page_index = 1;
        self.current.section_page_index = section_page_index;
        activate_pending_floating_table_follows_for_current(&mut self.current, &mut self.pages);
        self.seed_current_repeating_wrap_exclusions();
      }
      self.y = body_content_limits_for_page(
        section.page,
        repeating_slot_state(self.document, section_index),
        self.pages.len() + 1,
        section_page_index,
      )
      .0;
    }
  }

  fn current_page_has_body_progress(&self) -> bool {
    if !self.current.items.is_empty() {
      return true;
    }
    let body_top = body_content_limits_for_page(
      self.current.setup,
      repeating_slot_state(self.document, self.current.section_index),
      self.pages.len() + 1,
      self.current.section_page_index,
    )
    .0;
    self.y > body_top + LAYOUT_EPSILON_PT
  }

  fn format_block_sequence(&mut self, blocks: &[Block], mut flow: FlowContext) {
    for (index, block) in blocks.iter().enumerate() {
      self.record_layout_checkpoint(index, flow);
      let previous = index.checked_sub(1).and_then(|index| blocks.get(index));
      let next = blocks.get(index + 1);
      self.format_block(index, previous, block, next, &mut flow);
    }
  }

  fn format_block_sequence_with_previous(
    &mut self,
    blocks: &[Block],
    mut flow: FlowContext,
    previous_block: Option<&Block>,
  ) {
    for (index, block) in blocks.iter().enumerate() {
      self.record_layout_checkpoint(index, flow);
      let previous = index
        .checked_sub(1)
        .and_then(|index| blocks.get(index))
        .or_else(|| (index == 0).then_some(previous_block).flatten());
      let next = blocks.get(index + 1);
      if index == 0 && previous_block.is_some() {
        flow.paragraph_spacing_context = ParagraphSpacingContext::SectionStart;
      }
      self.format_block(index, previous, block, next, &mut flow);
      flow.paragraph_spacing_context = ParagraphSpacingContext::Normal;
    }
  }

  fn format_block_range(&mut self, blocks: &[Block], start_index: usize, mut flow: FlowContext) {
    for index in start_index..blocks.len() {
      let previous = index.checked_sub(1).and_then(|index| blocks.get(index));
      let block = &blocks[index];
      let next = blocks.get(index + 1);
      self.record_layout_checkpoint(index, flow);
      self.format_block(index, previous, block, next, &mut flow);
    }
  }

  fn record_layout_checkpoint(&mut self, block_index: usize, flow: FlowContext) {
    self.checkpoints.push(LayoutCheckpoint {
      section_index: flow.section_index,
      block_index,
      page_index: self.pages.len(),
      y: self.y,
      flow,
      pages: self.pages.clone(),
      current: self.current.clone(),
      emitted_footnotes: self.emitted_footnotes.clone(),
      follows: self.follows.clone(),
      frames: self.frames.clone(),
      outline_entries: self.outline_entries.clone(),
      next_line_number: self.next_line_number,
    });
  }

  fn format_block(
    &mut self,
    block_index: usize,
    previous: Option<&Block>,
    block: &Block,
    next: Option<&Block>,
    flow: &mut FlowContext,
  ) {
    if block_index == 1
      && previous.is_some_and(block_is_empty_paragraph)
      && next.is_some()
      && flow.text_segmentation == TextSegmentation::Body
      && block_is_page_break_only_paragraph(block)
    {
      self.pending_trailing_page_break = true;
      return;
    }
    let kind = follow_kind_for_block(block);
    let (block_flow, footnote_top) =
      footnote_boss_reserve(block, self.document, &self.emitted_footnotes, *flow);
    let transition = self.follow_transition_start(*flow);
    (*flow, self.y) = prepare_block_flow(
      block,
      next,
      block_flow,
      &mut self.current,
      &mut self.pages,
      self.y,
    );
    self.record_follow_transition(
      transition,
      *flow,
      kind,
      FollowReason::KeepTogether,
      Some(block_index),
    );
    *flow = self.advance_if_past_body(*flow);
    let transition = self.follow_transition_start(*flow);
    let frame_start = self.frame_segment_start(*flow);
    let line_number_start = self.current.items.len();
    let frame_influences = block_frame_influences(
      block,
      self.document,
      &self.emitted_footnotes,
      *flow,
      Some(block_index),
    );
    (*flow, self.y) = layout_document_block(
      previous,
      block,
      next,
      *flow,
      &mut self.current,
      &mut self.pages,
      self.y,
    );
    self.add_line_number_for_block(block, *flow, line_number_start);
    self.record_layout_frame_segments(
      frame_start,
      *flow,
      kind,
      Some(block_index),
      &frame_influences,
    );
    self.record_outline_entry(block, frame_start);
    self.record_follow_transition(
      transition,
      *flow,
      kind,
      layout_follow_reason_for_block(block),
      Some(block_index),
    );
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

  fn add_line_number_for_block(&mut self, block: &Block, flow: FlowContext, start_index: usize) {
    if flow.text_segmentation != TextSegmentation::Body {
      return;
    }
    if !matches!(block, Block::Paragraph(_)) {
      return;
    }
    let Some(line_numbering) = flow.setup.line_numbering else {
      return;
    };
    let Some(first_text) = self.current.items[start_index..].iter().find_map(|item| {
      if let PageItem::Text(text) = item {
        Some(text)
      } else {
        None
      }
    }) else {
      return;
    };

    let number = self.next_line_number;
    self.next_line_number = self.next_line_number.saturating_add(1);
    if number < line_numbering.start
      || (number - line_numbering.start) % line_numbering.count_by != 0
    {
      return;
    }

    let mut style = TextStyle::default();
    style.font_size_pt = first_text.style.font_size_pt;
    let text = number.to_string();
    let width = measure_text(&text, &style);
    self.current.items.insert(
      start_index,
      PageItem::Text(TextItem {
        x_pt: (flow.content_left_pt - line_numbering.distance_pt - width).max(0.0),
        y_pt: first_text.y_pt,
        line_height_pt: first_text.line_height_pt,
        text,
        style,
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        preserve_text_portion: false,
        form_widget_id: None,
        paragraph_bidi: false,
        decoration_span_start_x_pt: None,
        pdf_text_segmentation: PdfTextSegmentation::Line,
      }),
    );
  }

  fn advance_if_past_body(&mut self, flow: FlowContext) -> FlowContext {
    if self.y + DEFAULT_LINE_HEIGHT_PT > flow.content_bottom && !self.current.items.is_empty() {
      let transition = self.follow_transition_start(flow);
      let (next_flow, next_y) = advance_section_flow(flow, &mut self.current, &mut self.pages);
      self.y = next_y;
      self.record_follow_transition(
        transition,
        next_flow,
        FollowFrameKind::Paragraph,
        FollowReason::Overflow,
        None,
      );
      next_flow
    } else {
      flow
    }
  }

  fn format_trailing_note_frames(&mut self) {
    let note_setup = self.current.setup;
    let mut note_flow = flow_context(
      note_setup,
      self.current.section_index,
      SectionColumns::default(),
      0,
      self.document.default_tab_stop_pt,
    );
    note_flow.text_segmentation = TextSegmentation::Notes;

    if self.document.footnotes.is_empty() && !self.document.footnote_blocks.is_empty() {
      self.format_note_block_sequence(
        NoteSeparatorKind::Footnote,
        note_setup,
        note_flow,
        &self.document.footnote_blocks,
      );
    }

    if !self.document.endnotes.is_empty() {
      let endnote_start_page_index = self.pages.len();
      self.y = layout_note_separator(
        NoteSeparatorKind::Endnote,
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
      add_endnote_continuation_separators(
        note_setup,
        &mut self.pages,
        &mut self.current,
        endnote_start_page_index,
      );
    } else if !self.document.endnote_blocks.is_empty() {
      let endnote_start_page_index = self.pages.len();
      self.format_note_block_sequence(
        NoteSeparatorKind::Endnote,
        note_setup,
        note_flow,
        &self.document.endnote_blocks,
      );
      add_endnote_continuation_separators(
        note_setup,
        &mut self.pages,
        &mut self.current,
        endnote_start_page_index,
      );
    }

    if !self.document.comment_blocks.is_empty() {
      self.format_note_block_sequence(
        NoteSeparatorKind::Endnote,
        note_setup,
        note_flow,
        &self.document.comment_blocks,
      );
    }
  }

  fn format_note_block_sequence(
    &mut self,
    separator_kind: NoteSeparatorKind,
    setup: PageSetup,
    flow: FlowContext,
    blocks: &[Block],
  ) {
    self.y = layout_note_separator(
      separator_kind,
      setup,
      &mut self.current,
      &mut self.pages,
      self.y,
      flow.content_bottom,
    );
    self.format_blocks_in_flow(blocks, flow);
  }

  fn format_blocks_in_flow(&mut self, blocks: &[Block], flow: FlowContext) {
    for (index, block) in blocks.iter().enumerate() {
      let previous = index.checked_sub(1).and_then(|index| blocks.get(index));
      let next = blocks.get(index + 1);
      let transition = self.follow_transition_start(flow);
      let frame_start = self.frame_segment_start(flow);
      let frame_influences = block_frame_influences(
        block,
        self.document,
        &self.emitted_footnotes,
        flow,
        Some(index),
      );
      let (_, y) = layout_document_block(
        previous,
        block,
        next,
        flow,
        &mut self.current,
        &mut self.pages,
        self.y,
      );
      self.y = y;
      self.record_layout_frame_segments(
        frame_start,
        flow,
        FollowFrameKind::Notes,
        Some(index),
        &frame_influences,
      );
      self.record_follow_transition(
        transition,
        flow,
        FollowFrameKind::Notes,
        FollowReason::Overflow,
        Some(index),
      );
    }
  }

  fn apply_checkpoint_rerun(&mut self, backward_moves: &[BackwardMove]) -> Option<LayoutRerun> {
    let move_back = backward_moves
      .iter()
      .filter(|move_back| !move_back.suppressed)
      .min_by_key(|move_back| move_back.replay_start_frame_index)?;
    let start_frame = self.frames.get(move_back.replay_start_frame_index)?;
    let block_index = start_frame.block_index?;
    let constraints = rerun_constraints_for_frame(start_frame, move_back.scope);
    let (checkpoint_index, checkpoint) =
      self
        .checkpoints
        .iter()
        .enumerate()
        .rev()
        .find(|(_, checkpoint)| {
          checkpoint.section_index == start_frame.section_index
            && checkpoint.block_index <= block_index
        })?;
    let checkpoint = checkpoint.clone();
    let replaced_pages = self.pages.len().saturating_sub(checkpoint.page_index);
    let mut rerun = RootFrameLayout {
      document: self.document,
      pages: checkpoint.pages.clone(),
      current: checkpoint.current.clone(),
      y: checkpoint.y,
      emitted_footnotes: checkpoint.emitted_footnotes.clone(),
      follows: checkpoint.follows.clone(),
      frames: checkpoint.frames.clone(),
      outline_entries: checkpoint.outline_entries.clone(),
      checkpoints: self.checkpoints[..=checkpoint_index].to_vec(),
      next_line_number: checkpoint.next_line_number,
      pending_trailing_page_break: self.pending_trailing_page_break,
    };
    rerun.format_body_from_checkpoint(&checkpoint, &constraints);
    rerun.format_trailing_note_frames();
    rerun.finish_current_page();

    self.pages = rerun.pages;
    self.current = rerun.current;
    self.y = rerun.y;
    self.emitted_footnotes = rerun.emitted_footnotes;
    self.follows = rerun.follows;
    self.frames = rerun.frames;
    self.outline_entries = rerun.outline_entries;
    self.checkpoints = rerun.checkpoints;
    self.next_line_number = rerun.next_line_number;

    Some(LayoutRerun {
      checkpoint_index,
      section_index: checkpoint.section_index,
      block_index: checkpoint.block_index,
      page_index: checkpoint.page_index,
      frame_index: move_back.replay_start_frame_index,
      reason: move_back.reason,
      scope: move_back.scope,
      replaced_pages,
      produced_pages: self.pages.len().saturating_sub(checkpoint.page_index),
      produced_frames: self.frames.len().saturating_sub(checkpoint.frames.len()),
      constraints,
    })
  }

  fn format_body_from_checkpoint(
    &mut self,
    checkpoint: &LayoutCheckpoint,
    constraints: &[LayoutRerunConstraint],
  ) {
    let checkpoint_flow = constrained_rerun_flow(checkpoint.flow, constraints);
    self.y = self.y.min(checkpoint_flow.content_bottom);
    if self.document.sections.is_empty() {
      let blocks = self.document.blocks.clone();
      self.format_block_range(&blocks, checkpoint.block_index, checkpoint_flow);
      return;
    }

    for section_index in checkpoint.section_index..self.document.sections.len() {
      let section = self.document.sections[section_index].clone();
      if section_index == checkpoint.section_index {
        self.format_block_range(&section.blocks, checkpoint.block_index, checkpoint_flow);
      } else {
        self.start_section_frame(section_index, &section);
        let flow = self.body_flow(document_page_frame(
          section.page,
          section_index,
          section.columns,
        ));
        self.y = self.y.max(flow.content_top_pt);
        self.format_block_sequence(&section.blocks, flow);
      }
    }
  }

  fn finish_current_page(&mut self) {
    if !self.current.items.is_empty() || self.current.preserve_empty || self.pages.is_empty() {
      self.pages.push(std::mem::replace(
        &mut self.current,
        empty_page(self.document.page, 0),
      ));
    }
  }

  fn finish_pending_trailing_page_break(&mut self) {
    if !self.pending_trailing_page_break {
      return;
    }
    let Some(previous) = self.pages.last() else {
      return;
    };
    let mut page = empty_section_page(
      previous.setup,
      previous.section_index,
      previous.section_page_index + 1,
    );
    page.repeating_wrap_exclusion_catalog = previous.repeating_wrap_exclusion_catalog.clone();
    page.repeating_wrap_exclusions = page
      .repeating_wrap_exclusion_catalog
      .selected(page.section_page_index, self.pages.len() + 1)
      .to_vec();
    extend_wrap_exclusions_unique(&mut page.wrap_exclusions, &page.repeating_wrap_exclusions);
    self.pages.push(page);
  }

  fn push_current_page(&mut self, next: Page) {
    self.pages.push(std::mem::replace(&mut self.current, next));
    activate_pending_floating_table_follows_for_current(&mut self.current, &mut self.pages);
    self.seed_current_repeating_wrap_exclusions();
  }

  fn seed_current_repeating_wrap_exclusions(&mut self) {
    let catalog = repeating_wrap_exclusion_catalog_for_page(self.document, &self.current);
    let exclusions = catalog
      .selected(self.current.section_page_index, self.pages.len() + 1)
      .to_vec();
    let previous = self.current.repeating_wrap_exclusions.clone();
    self
      .current
      .wrap_exclusions
      .retain(|exclusion| !previous.contains(exclusion));
    self.current.repeating_wrap_exclusion_catalog = catalog;
    self.current.repeating_wrap_exclusions = exclusions;
    extend_wrap_exclusions_unique(
      &mut self.current.wrap_exclusions,
      &self.current.repeating_wrap_exclusions,
    );
  }

  fn follow_transition_start(&self, flow: FlowContext) -> FollowTransitionStart {
    FollowTransitionStart {
      page_index: self.pages.len(),
      section_page_index: flow.section_page_index,
      column_index: flow.column_index,
    }
  }

  fn record_follow_transition(
    &mut self,
    start: FollowTransitionStart,
    flow: FlowContext,
    kind: FollowFrameKind,
    reason: FollowReason,
    block_index: Option<usize>,
  ) {
    let to_page_index = self.pages.len();
    if start.page_index == to_page_index
      && start.section_page_index == flow.section_page_index
      && start.column_index == flow.column_index
    {
      return;
    }
    if self.follows.last().is_some_and(|follow| {
      follow.kind == kind
        && follow.reason == reason
        && follow.block_index == block_index
        && follow.from_page_index == start.page_index
        && follow.to_page_index == to_page_index
        && follow.from_section_page_index == start.section_page_index
        && follow.to_section_page_index == flow.section_page_index
        && follow.from_column_index == start.column_index
        && follow.to_column_index == flow.column_index
    }) {
      return;
    }
    self.follows.push(FrameFollow {
      kind,
      reason,
      block_index,
      from_page_index: start.page_index,
      to_page_index,
      from_section_page_index: start.section_page_index,
      to_section_page_index: flow.section_page_index,
      from_column_index: start.column_index,
      to_column_index: flow.column_index,
    });
  }

  fn frame_segment_start(&self, flow: FlowContext) -> FrameSegmentStart {
    FrameSegmentStart {
      page_index: self.pages.len(),
      item_index: self.current.items.len(),
      column_index: flow.column_index,
    }
  }

  fn record_layout_frame_segments(
    &mut self,
    start: FrameSegmentStart,
    flow: FlowContext,
    kind: FollowFrameKind,
    block_index: Option<usize>,
    influences: &[FrameInfluence],
  ) {
    let end_page_index = self.pages.len();
    for page_index in start.page_index..=end_page_index {
      let page = if page_index < self.pages.len() {
        &self.pages[page_index]
      } else {
        &self.current
      };
      let item_start = if page_index == start.page_index {
        start.item_index
      } else {
        0
      };
      let item_end = page.items.len();
      if item_start >= item_end {
        continue;
      }
      let items = page.items[item_start..item_end].to_vec();
      let bounds = frame_bounds_for_items(&items);
      let lines = line_boxes_for_items(&page.items, item_start, item_end);
      let mut fragments = page_frame_fragments(kind, &page.frame_fragments, item_start, item_end);
      if fragments.is_empty() {
        fragments = frame_fragments_for(kind, &lines);
      }
      let bounds = bounds.or_else(|| frame_bounds_for_items(&items));
      let mut frame_influences = frame_influences_for_segment(influences, block_index, bounds);
      frame_influences.extend(page_frame_influences(
        &page.frame_influences,
        item_start,
        item_end,
        block_index,
      ));
      self.frames.push(LayoutFrame {
        kind,
        block_index,
        split_start: frame_cursor(block_index, kind, item_start, &lines, true),
        split_end: frame_cursor(block_index, kind, item_end, &lines, false),
        page_index,
        section_index: page.section_index,
        section_page_index: page.section_page_index,
        column_index: if page_index == start.page_index {
          start.column_index
        } else {
          flow.column_index
        },
        items,
        item_start,
        item_end,
        bounds,
        lines,
        fragments,
        influences: frame_influences,
        invalidation: FrameInvalidation::Clean,
      });
    }
  }

  fn record_outline_entry(&mut self, block: &Block, start: FrameSegmentStart) {
    let Block::Paragraph(paragraph) = block else {
      return;
    };
    let Some(level) = paragraph.format.outline_level else {
      return;
    };
    let text = paragraph_outline_text(paragraph);
    if text.is_empty() {
      return;
    }
    // Source: LibreOffice sw/source/core/text/EnhancedPDFExportHelper.cxx
    // iterates the document outline node list while
    // DocumentOutlineNodesManager::GetExpandTextMerged() consults merged
    // paragraph layout. In tdf131728 this leaves the last style-separator
    // child visible once more before the next top-level heading.
    if level == 0
      && self
        .outline_entries
        .last()
        .is_some_and(|entry| entry.level > level && entry.merged_hidden_separator)
      && let Some(previous) = self.outline_entries.last().cloned()
    {
      self.outline_entries.push(previous);
    }
    if let Some((page_index, text_item)) = first_text_item_from(&self.pages, &self.current, start) {
      self.outline_entries.push(OutlineEntry {
        level,
        text,
        page_index,
        x_pt: text_item.x_pt,
        y_pt: text_item.y_pt,
        merged_hidden_separator: paragraph.format.outline_text_inlines.is_some(),
      });
    }
  }
}

fn paragraph_outline_text(paragraph: &crate::docx::Paragraph) -> String {
  let mut text = paragraph.list_label.clone().unwrap_or_default();
  let inline_count = paragraph
    .format
    .outline_text_inlines
    .unwrap_or(paragraph.inlines.len());
  text.push_str(
    &paragraph
      .inlines
      .iter()
      .take(inline_count)
      .filter_map(|inline| match inline {
        InlineItem::Text(text) => Some(text.text.as_str()),
        InlineItem::Image(_)
        | InlineItem::Shape(_)
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_)
        | InlineItem::LastRenderedPageBreak
        | InlineItem::PageBreak
        | InlineItem::ColumnBreak => None,
      })
      .collect::<String>(),
  );
  text.trim_end().to_string()
}

fn first_text_item_from(
  pages: &[Page],
  current: &Page,
  start: FrameSegmentStart,
) -> Option<(usize, TextItem)> {
  for page_index in start.page_index..=pages.len() {
    let page = if page_index < pages.len() {
      &pages[page_index]
    } else {
      current
    };
    let item_start = if page_index == start.page_index {
      start.item_index
    } else {
      0
    };
    for item in page.items.iter().skip(item_start) {
      if let PageItem::Text(text) = item
        && !text.text.trim().is_empty()
      {
        return Some((page_index, text.clone()));
      }
    }
  }
  None
}

fn rerun_constraints_for_frame(
  frame: &LayoutFrame,
  scope: ReflowScope,
) -> Vec<LayoutRerunConstraint> {
  let mut constraints = Vec::new();
  for influence in &frame.influences {
    let Some(bounds) = influence.bounds else {
      continue;
    };
    let (content_left_pt, content_width, content_bottom) = match influence.kind {
      FrameInfluenceKind::FootnoteReservation | FrameInfluenceKind::TableSplit => (
        frame
          .bounds
          .map(|bounds| bounds.x_pt)
          .unwrap_or(bounds.x_pt),
        frame
          .bounds
          .map(|bounds| bounds.width_pt)
          .unwrap_or(bounds.width_pt),
        bounds.y_pt,
      ),
      FrameInfluenceKind::FlyWrap => {
        let frame_bounds = frame.bounds.unwrap_or(bounds);
        if bounds.x_pt <= frame_bounds.x_pt {
          let left = bounds.x_pt + bounds.width_pt;
          let right = frame_bounds.x_pt + frame_bounds.width_pt;
          (
            left,
            (right - left).max(DEFAULT_FONT_SIZE_PT),
            frame_bounds.y_pt + frame_bounds.height_pt,
          )
        } else {
          (
            frame_bounds.x_pt,
            (bounds.x_pt - frame_bounds.x_pt).max(DEFAULT_FONT_SIZE_PT),
            frame_bounds.y_pt + frame_bounds.height_pt,
          )
        }
      }
    };
    constraints.push(LayoutRerunConstraint {
      kind: influence.kind,
      scope,
      bounds: Some(bounds),
      content_left_pt,
      content_width,
      content_bottom,
    });
  }
  constraints
}

fn constrained_rerun_flow(
  mut flow: FlowContext,
  constraints: &[LayoutRerunConstraint],
) -> FlowContext {
  for constraint in constraints {
    if !constraint_applies_to_flow(constraint, flow) {
      continue;
    }
    match constraint.kind {
      FrameInfluenceKind::FootnoteReservation | FrameInfluenceKind::TableSplit => {
        flow.content_bottom = flow
          .content_bottom
          .min(constraint.content_bottom)
          .max(flow.content_top_pt + DEFAULT_LINE_HEIGHT_PT);
        flow.body_content_bottom_pt = flow.body_content_bottom_pt.min(flow.content_bottom);
      }
      FrameInfluenceKind::FlyWrap => {
        flow.content_left_pt = constraint.content_left_pt;
        flow.content_width = constraint.content_width.max(DEFAULT_FONT_SIZE_PT);
      }
    }
  }
  flow
}

fn constraint_applies_to_flow(constraint: &LayoutRerunConstraint, flow: FlowContext) -> bool {
  let Some(bounds) = constraint.bounds else {
    return false;
  };
  let vertical_overlap =
    bounds.y_pt < flow.content_bottom && bounds.y_pt + bounds.height_pt > flow.content_top_pt;
  vertical_overlap
    && match constraint.scope {
      ReflowScope::Frame => true,
      ReflowScope::Column => true,
      ReflowScope::Page => true,
    }
}

#[derive(Clone, Copy, Debug)]
struct FollowTransitionStart {
  page_index: usize,
  section_page_index: usize,
  column_index: usize,
}

#[derive(Clone, Copy, Debug)]
struct FrameSegmentStart {
  page_index: usize,
  item_index: usize,
  column_index: usize,
}

fn follow_kind_for_block(block: &Block) -> FollowFrameKind {
  match block {
    Block::Paragraph(_) | Block::Frame(_) => FollowFrameKind::Paragraph,
    Block::Table(_) => FollowFrameKind::Table,
  }
}

fn frame_bounds_for_items(items: &[PageItem]) -> Option<FrameBounds> {
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  for item in items {
    let Some((x1, y1, x2, y2)) = item_bounds(item) else {
      continue;
    };
    bounds = Some(match bounds {
      Some((min_x, min_y, max_x, max_y)) => {
        (min_x.min(x1), min_y.min(y1), max_x.max(x2), max_y.max(y2))
      }
      None => (x1, y1, x2, y2),
    });
  }
  bounds.map(|(x1, y1, x2, y2)| FrameBounds {
    x_pt: x1,
    y_pt: y1,
    width_pt: (x2 - x1).max(0.0),
    height_pt: (y2 - y1).max(0.0),
  })
}

fn line_boxes_for_items(items: &[PageItem], item_start: usize, item_end: usize) -> Vec<LineBox> {
  let mut lines = Vec::new();
  let mut index = item_start;
  while index < item_end {
    let Some(y) = item_line_y(&items[index]) else {
      index += 1;
      continue;
    };
    let line_start = index;
    let mut line_end = index + 1;
    let mut bounds = item_bounds(&items[index]);
    while line_end < item_end
      && item_line_y(&items[line_end]).is_some_and(|next_y| (next_y - y).abs() < LAYOUT_EPSILON_PT)
    {
      if let Some((x1, y1, x2, y2)) = item_bounds(&items[line_end]) {
        bounds = Some(match bounds {
          Some((min_x, min_y, max_x, max_y)) => {
            (min_x.min(x1), min_y.min(y1), max_x.max(x2), max_y.max(y2))
          }
          None => (x1, y1, x2, y2),
        });
      }
      line_end += 1;
    }
    if let Some((x1, y1, x2, y2)) = bounds {
      lines.push(LineBox {
        x_pt: x1,
        y_pt: y1,
        width_pt: (x2 - x1).max(0.0),
        height_pt: (y2 - y1).max(0.0),
        item_start: line_start,
        item_end: line_end,
      });
    }
    index = line_end;
  }
  lines
}

fn frame_fragments_for(kind: FollowFrameKind, lines: &[LineBox]) -> Vec<FrameFragment> {
  lines
    .iter()
    .enumerate()
    .map(|(index, line)| FrameFragment {
      kind: match kind {
        FollowFrameKind::Paragraph => FrameFragmentKind::ParagraphLine,
        FollowFrameKind::Table => FrameFragmentKind::TableRow,
        FollowFrameKind::Notes => FrameFragmentKind::NoteLine,
      },
      split: FragmentSplitKind::Complete,
      index,
      row_index: index,
      cell_index: None,
      item_start: line.item_start,
      item_end: line.item_end,
      bounds: Some(FrameBounds {
        x_pt: line.x_pt,
        y_pt: line.y_pt,
        width_pt: line.width_pt,
        height_pt: line.height_pt,
      }),
    })
    .collect()
}

fn block_frame_influences(
  block: &Block,
  document: &DocxDocument,
  emitted_footnotes: &HashSet<i64>,
  flow: FlowContext,
  block_index: Option<usize>,
) -> Vec<FrameInfluence> {
  let mut influences = Vec::new();
  match block {
    Block::Paragraph(paragraph) => {
      let footnote_count = paragraph
        .footnote_reference_ids
        .iter()
        .filter(|id| !emitted_footnotes.contains(id) && document.footnotes.contains_key(id))
        .count();
      if footnote_count > 0 {
        let height =
          referenced_footnote_area_height(block, document, emitted_footnotes, flow).max(0.0);
        influences.push(FrameInfluence {
          kind: FrameInfluenceKind::FootnoteReservation,
          count: footnote_count,
          block_index,
          item_start: 0,
          item_end: usize::MAX,
          bounds: Some(FrameBounds {
            x_pt: flow.setup.margin_left_pt,
            y_pt: (flow.content_bottom - height).max(flow.content_top_pt),
            width_pt: (flow.setup.width_pt
              - flow.setup.margin_left_pt
              - flow.setup.margin_right_pt)
              .max(DEFAULT_FONT_SIZE_PT),
            height_pt: height,
          }),
        });
      }

      let fly_count = paragraph
        .inlines
        .iter()
        .filter(|inline| {
          matches!(
            inline,
            InlineItem::Image(image)
              if matches!(
                image.placement,
                crate::docx::ImagePlacement::Floating(placement)
                  if !placement.behind_text
                    && !matches!(placement.wrap, ImageWrapMode::Through | ImageWrapMode::Inline)
              )
          ) || matches!(
            inline,
            InlineItem::Shape(shape)
              if matches!(
                shape.placement,
                crate::docx::ImagePlacement::Floating(placement)
                  if !placement.behind_text
                    && !matches!(placement.wrap, ImageWrapMode::Through | ImageWrapMode::Inline)
              )
          )
        })
        .count();
      if fly_count > 0 {
        influences.push(FrameInfluence {
          kind: FrameInfluenceKind::FlyWrap,
          count: fly_count,
          block_index,
          item_start: 0,
          item_end: usize::MAX,
          bounds: None,
        });
      }
    }
    Block::Table(table) => {
      let split_sensitive_rows = table
        .rows
        .iter()
        .enumerate()
        .filter(|(row_index, row)| {
          row.cant_split
            || row_repeat_header_effective(table, *row_index)
            || row.cells.iter().any(|cell| cell.grid_span > 1)
        })
        .count();
      if split_sensitive_rows > 0 {
        influences.push(FrameInfluence {
          kind: FrameInfluenceKind::TableSplit,
          count: split_sensitive_rows,
          block_index,
          item_start: 0,
          item_end: usize::MAX,
          bounds: None,
        });
      }
    }
    Block::Frame(frame) => {
      influences.extend(frame.blocks.iter().flat_map(|block| {
        block_frame_influences(block, document, emitted_footnotes, flow, block_index)
      }));
    }
  }
  influences
}

fn frame_influences_for_segment(
  influences: &[FrameInfluence],
  block_index: Option<usize>,
  bounds: Option<FrameBounds>,
) -> Vec<FrameInfluence> {
  influences
    .iter()
    .cloned()
    .map(|mut influence| {
      influence.block_index = influence.block_index.or(block_index);
      if influence.bounds.is_none() {
        influence.bounds = bounds;
      }
      influence.item_start = 0;
      influence.item_end = usize::MAX;
      influence
    })
    .collect()
}

fn page_frame_influences(
  influences: &[FrameInfluence],
  item_start: usize,
  item_end: usize,
  block_index: Option<usize>,
) -> Vec<FrameInfluence> {
  influences
    .iter()
    .filter(|influence| influence.item_start < item_end && influence.item_end > item_start)
    .cloned()
    .map(|mut influence| {
      influence.block_index = influence.block_index.or(block_index);
      influence
    })
    .collect()
}

fn page_frame_fragments(
  frame_kind: FollowFrameKind,
  fragments: &[FrameFragment],
  item_start: usize,
  item_end: usize,
) -> Vec<FrameFragment> {
  fragments
    .iter()
    .filter(|fragment| fragment.item_start < item_end && fragment.item_end > item_start)
    .cloned()
    .map(|mut fragment| {
      if matches!(frame_kind, FollowFrameKind::Notes)
        && matches!(fragment.kind, FrameFragmentKind::ParagraphLine)
      {
        fragment.kind = FrameFragmentKind::NoteLine;
      }
      fragment
    })
    .collect()
}

fn push_page_fragment(page: &mut Page, fragment: PageFragmentRecord) {
  let PageFragmentRecord {
    kind,
    split,
    index,
    row_index,
    cell_index,
    item_start,
    item_end,
  } = fragment;
  if item_start >= item_end {
    return;
  }
  let bounds = frame_bounds_for_items(&page.items[item_start..item_end]);
  page.frame_fragments.push(FrameFragment {
    kind,
    split,
    index,
    row_index,
    cell_index,
    item_start,
    item_end,
    bounds,
  });
}

struct PageFragmentRecord {
  kind: FrameFragmentKind,
  split: FragmentSplitKind,
  index: usize,
  row_index: usize,
  cell_index: Option<usize>,
  item_start: usize,
  item_end: usize,
}

fn push_page_influence(
  page: &mut Page,
  kind: FrameInfluenceKind,
  item_start: usize,
  item_end: usize,
  bounds: Option<FrameBounds>,
) {
  if item_start >= item_end {
    return;
  }
  page.frame_influences.push(FrameInfluence {
    kind,
    count: 1,
    block_index: None,
    item_start,
    item_end,
    bounds,
  });
}

fn frame_cursor(
  block_index: Option<usize>,
  frame_kind: FollowFrameKind,
  item_index: usize,
  lines: &[LineBox],
  start: bool,
) -> FrameCursor {
  if let Some((line_index, _line)) = lines.iter().enumerate().find(|(_, line)| {
    if start {
      item_index <= line.item_end
    } else {
      item_index <= line.item_end.saturating_add(1)
    }
  }) {
    return FrameCursor {
      block_index,
      kind: match frame_kind {
        FollowFrameKind::Paragraph | FollowFrameKind::Notes => FrameCursorKind::Inline,
        FollowFrameKind::Table => {
          if start {
            FrameCursorKind::TableRow
          } else {
            FrameCursorKind::TableCell
          }
        }
      },
      inline_index: if matches!(
        frame_kind,
        FollowFrameKind::Paragraph | FollowFrameKind::Notes
      ) {
        line_index
      } else {
        0
      },
      text_offset: 0,
      row_index: if matches!(frame_kind, FollowFrameKind::Table) {
        line_index
      } else {
        0
      },
      cell_index: 0,
    };
  }

  FrameCursor {
    block_index,
    kind: if start {
      FrameCursorKind::BlockStart
    } else {
      FrameCursorKind::BlockEnd
    },
    inline_index: 0,
    text_offset: 0,
    row_index: 0,
    cell_index: 0,
  }
}

fn mark_decorated_frame_invalidations(
  frames: &mut [LayoutFrame],
  pages: &[Page],
  original_page_item_counts: &[usize],
) {
  for frame in frames {
    let Some(page) = pages.get(frame.page_index) else {
      frame.invalidation = FrameInvalidation::NeedsReflow;
      continue;
    };
    let original_count = original_page_item_counts
      .get(frame.page_index)
      .copied()
      .unwrap_or(page.items.len());
    if page.items.len() != original_count {
      frame.invalidation = FrameInvalidation::PageItemsDecorated;
    }
    if frame
      .bounds
      .is_none_or(|bounds| bounds.width_pt <= 0.0 || bounds.height_pt <= 0.0)
    {
      frame.invalidation = FrameInvalidation::NeedsReflow;
    }
  }
}

fn reflow_requests_for_frames(
  frames: &[LayoutFrame],
  include_insertion_influences: bool,
) -> Vec<ReflowRequest> {
  frames
    .iter()
    .enumerate()
    .filter_map(|(frame_index, frame)| {
      let reason = match frame.invalidation {
        FrameInvalidation::Clean if frame.influences.is_empty() => return None,
        FrameInvalidation::Clean if !include_insertion_influences => return None,
        FrameInvalidation::Clean => ReflowReason::InsertionInfluenceChanged,
        FrameInvalidation::PageItemsDecorated => ReflowReason::DecorationChangedItems,
        FrameInvalidation::NeedsReflow => ReflowReason::InvalidBounds,
      };
      Some(ReflowRequest {
        frame_index,
        kind: frame.kind,
        reason,
        scope: reflow_scope_for_frame(frame),
        restart: frame.split_start,
        page_index: frame.page_index,
        section_page_index: frame.section_page_index,
        column_index: frame.column_index,
        influence_count: frame.influences.len(),
      })
    })
    .collect()
}

fn reflow_scope_for_frame(frame: &LayoutFrame) -> ReflowScope {
  frame
    .influences
    .iter()
    .fold(ReflowScope::Frame, |scope, influence| {
      scope.max(match influence.kind {
        FrameInfluenceKind::FootnoteReservation => ReflowScope::Page,
        FrameInfluenceKind::FlyWrap => ReflowScope::Column,
        FrameInfluenceKind::TableSplit => ReflowScope::Frame,
      })
    })
}

fn execute_reflow_requests(
  frames: &mut [LayoutFrame],
  requests: Vec<ReflowRequest>,
) -> (
  Vec<ReflowRequest>,
  Vec<ReflowExecution>,
  Vec<PageReplay>,
  Vec<BackwardMove>,
) {
  let mut remaining = Vec::new();
  let mut page_replays = Vec::new();
  let mut backward_moves = Vec::new();
  let mut stabilized_count = 0usize;
  let mut first_stabilized_page = usize::MAX;
  let mut influence_count = 0usize;
  let mut first_influence_page = usize::MAX;
  let mut influence_scope = ReflowScope::Frame;
  let mut influence_replayed_frames = 0usize;
  let mut influence_replayed_items = 0usize;
  let mut suppressed_moves = 0usize;
  let mut move_backward_keys = HashMap::<MoveBackwardKey, usize>::new();
  let mut replayed_influence_frames = HashSet::<usize>::new();

  for request in requests {
    match request.reason {
      ReflowReason::DecorationChangedItems => {
        if let Some(frame) = frames.get_mut(request.frame_index)
          && frame.items.len() == frame.item_end.saturating_sub(frame.item_start)
        {
          frame.invalidation = FrameInvalidation::Clean;
          stabilized_count += 1;
          first_stabilized_page = first_stabilized_page.min(request.page_index);
          continue;
        }
        remaining.push(request);
      }
      ReflowReason::InsertionInfluenceChanged => {
        if replayed_influence_frames.contains(&request.frame_index) {
          influence_count += 1;
          first_influence_page = first_influence_page.min(request.page_index);
          influence_scope = influence_scope.max(request.scope);
          continue;
        }
        if let Some(frame) = frames.get(request.frame_index)
          && !frame.influences.is_empty()
          && !frame.fragments.is_empty()
        {
          influence_count += 1;
          first_influence_page = first_influence_page.min(request.page_index);
          influence_scope = influence_scope.max(request.scope);
          if let Some(backward) = execute_backward_move(frames, &request, &mut move_backward_keys) {
            suppressed_moves += usize::from(backward.move_back.suppressed);
            influence_replayed_frames += backward.move_back.replayed_frames;
            influence_replayed_items += backward.move_back.replayed_items;
            page_replays.extend(backward.pages);
            backward_moves.push(backward.move_back);
          }
          let replay = replay_scoped_frames(frames, &request);
          influence_replayed_frames += replay.frames;
          influence_replayed_items += replay.items;
          replayed_influence_frames.extend(replay.frame_indices);
          page_replays.extend(replay.pages);
          continue;
        }
        remaining.push(request);
      }
      ReflowReason::InvalidBounds => remaining.push(request),
    }
  }

  for request in &remaining {
    let key = MoveBackwardKey {
      frame_index: request.frame_index,
      page_index: request.page_index,
      section_page_index: request.section_page_index,
      column_index: request.column_index,
      scope: request.scope,
    };
    let count = move_backward_keys.entry(key).or_default();
    *count += 1;
    if *count > MOVE_BACKWARD_SUPPRESS_THRESHOLD {
      suppressed_moves += 1;
    }
  }

  let mut executions = Vec::new();
  if stabilized_count > 0 {
    executions.push(ReflowExecution {
      first_page_index: first_stabilized_page,
      request_count: stabilized_count,
      action: ReflowAction::StabilizedRetainedDecorationItems,
      scope: ReflowScope::Frame,
      suppressed_moves: 0,
      backward_moves: 0,
      page_replacements: 0,
      replayed_frames: 0,
      replayed_items: 0,
    });
  }
  if influence_count > 0 {
    executions.push(ReflowExecution {
      first_page_index: first_influence_page,
      request_count: influence_count,
      action: ReflowAction::StabilizedInsertionInfluences,
      scope: influence_scope,
      suppressed_moves,
      backward_moves: backward_moves.len(),
      page_replacements: page_replays.len(),
      replayed_frames: influence_replayed_frames,
      replayed_items: influence_replayed_items,
    });
  }

  (remaining, executions, page_replays, backward_moves)
}

fn apply_page_replays(pages: &mut [Page], replays: &[PageReplay]) -> Vec<PageReplayApplication> {
  let mut applications = Vec::with_capacity(replays.len());
  for replay in replays {
    let applied = pages
      .get_mut(replay.page_index)
      .is_some_and(|page| apply_page_replay(page, replay));
    applications.push(PageReplayApplication {
      page_index: replay.page_index,
      section_page_index: replay.section_page_index,
      column_index: replay.column_index,
      scope: replay.scope,
      item_start: replay.item_start,
      item_end: replay.item_end,
      replacement_count: replay.replacement_items.len(),
      applied,
    });
  }
  applications
}

fn apply_page_replay(page: &mut Page, replay: &PageReplay) -> bool {
  if page.section_page_index != replay.section_page_index
    || replay.item_start > replay.item_end
    || replay.item_end > page.items.len()
  {
    return false;
  }

  page.items.splice(
    replay.item_start..replay.item_end,
    replay.replacement_items.clone(),
  );
  true
}

fn materialize_pending_floating_table_follows(pages: &mut Vec<Page>) {
  let mut pending = Vec::<PendingFloatingTableFollow>::new();
  for page in pages.iter_mut() {
    pending.append(&mut page.pending_floating_table_follows);
  }
  for mut follow in pending {
    let page_index = ensure_section_page_slot(
      pages,
      follow.setup,
      follow.section_index,
      follow.section_page_index,
    );
    let target = &mut pages[page_index];
    let item_offset = target.items.len();
    offset_page_frame_records_raw(
      &mut follow.frame_fragments,
      &mut follow.frame_influences,
      item_offset,
    );
    target.items.append(&mut follow.items);
    target.frame_fragments.append(&mut follow.frame_fragments);
    target.frame_influences.append(&mut follow.frame_influences);
    target.wrap_exclusions.append(&mut follow.wrap_exclusions);
  }
}

fn materialize_pending_floating_table_follows_in_local_pages(
  current: &mut Page,
  pages: &mut Vec<Page>,
) {
  let replacement = empty_section_page(
    current.setup,
    current.section_index,
    current.section_page_index,
  );
  let mut all_pages = std::mem::take(pages);
  all_pages.push(std::mem::replace(current, replacement));
  materialize_pending_floating_table_follows(&mut all_pages);
  if let Some(last_page) = all_pages.pop() {
    *current = last_page;
  }
  *pages = all_pages;
}

fn activate_pending_floating_table_follows_for_current(current: &mut Page, pages: &mut Vec<Page>) {
  let section_index = current.section_index;
  let section_page_index = current.section_page_index;
  let mut pending = Vec::<PendingFloatingTableFollow>::new();

  let mut remaining_current = Vec::new();
  for follow in current.pending_floating_table_follows.drain(..) {
    if follow.section_index == section_index && follow.section_page_index == section_page_index {
      pending.push(follow);
    } else {
      remaining_current.push(follow);
    }
  }
  current.pending_floating_table_follows = remaining_current;

  for page in pages.iter_mut() {
    let mut remaining = Vec::new();
    for follow in page.pending_floating_table_follows.drain(..) {
      if follow.section_index == section_index && follow.section_page_index == section_page_index {
        pending.push(follow);
      } else {
        remaining.push(follow);
      }
    }
    page.pending_floating_table_follows = remaining;
  }

  for follow in pending {
    apply_pending_floating_table_follow(current, follow);
  }
}

fn apply_pending_floating_table_follow(page: &mut Page, mut follow: PendingFloatingTableFollow) {
  let item_offset = page.items.len();
  offset_page_frame_records_raw(
    &mut follow.frame_fragments,
    &mut follow.frame_influences,
    item_offset,
  );
  page.items.append(&mut follow.items);
  page.frame_fragments.append(&mut follow.frame_fragments);
  page.frame_influences.append(&mut follow.frame_influences);
  page.wrap_exclusions.append(&mut follow.wrap_exclusions);
}

fn ensure_section_page_slot(
  pages: &mut Vec<Page>,
  setup: PageSetup,
  section_index: usize,
  section_page_index: usize,
) -> usize {
  if let Some(index) = pages.iter().position(|page| {
    page.section_index == section_index && page.section_page_index == section_page_index
  }) {
    return index;
  }

  let insert_at = pages
    .iter()
    .position(|page| {
      page.section_index > section_index
        || (page.section_index == section_index && page.section_page_index > section_page_index)
    })
    .unwrap_or(pages.len());

  let start_section_page = pages
    .iter()
    .filter(|page| page.section_index == section_index)
    .map(|page| page.section_page_index)
    .max()
    .map(|page_index| page_index.saturating_add(1))
    .unwrap_or(0)
    .min(section_page_index);

  let mut insert_index = insert_at;
  for pending_page_index in start_section_page..=section_page_index {
    pages.insert(
      insert_index,
      empty_section_page(setup, section_index, pending_page_index),
    );
    if pending_page_index == section_page_index {
      return insert_index;
    }
    insert_index += 1;
  }

  insert_at
}

fn execute_backward_move(
  frames: &mut [LayoutFrame],
  request: &ReflowRequest,
  move_backward_keys: &mut HashMap<MoveBackwardKey, usize>,
) -> Option<BackwardMoveExecution> {
  if request.page_index == 0 || matches!(request.scope, ReflowScope::Frame) {
    return None;
  }
  let replay_start_frame_index = previous_page_replay_start_frame(frames, request)?;
  let start_frame = frames.get(replay_start_frame_index)?;
  let key = MoveBackwardKey {
    frame_index: request.frame_index,
    page_index: start_frame.page_index,
    section_page_index: start_frame.section_page_index,
    column_index: start_frame.column_index,
    scope: request.scope,
  };
  let count = move_backward_keys.entry(key).or_default();
  *count += 1;
  let suppressed = *count > MOVE_BACKWARD_SUPPRESS_THRESHOLD;
  let mut move_back = BackwardMove {
    frame_index: request.frame_index,
    replay_start_frame_index,
    from_page_index: request.page_index,
    to_page_index: start_frame.page_index,
    from_section_page_index: request.section_page_index,
    to_section_page_index: start_frame.section_page_index,
    scope: request.scope,
    reason: request.reason,
    suppressed,
    replayed_frames: 0,
    replayed_items: 0,
  };
  if suppressed {
    return Some(BackwardMoveExecution {
      move_back,
      pages: Vec::new(),
    });
  }

  let replay_request = ReflowRequest {
    frame_index: replay_start_frame_index,
    kind: start_frame.kind,
    reason: request.reason,
    scope: request.scope,
    restart: start_frame.split_start,
    page_index: start_frame.page_index,
    section_page_index: start_frame.section_page_index,
    column_index: start_frame.column_index,
    influence_count: start_frame.influences.len(),
  };
  let replay = replay_scoped_frames(frames, &replay_request);
  move_back.replayed_frames = replay.frames;
  move_back.replayed_items = replay.items;
  Some(BackwardMoveExecution {
    move_back,
    pages: replay.pages,
  })
}

struct BackwardMoveExecution {
  move_back: BackwardMove,
  pages: Vec<PageReplay>,
}

fn previous_page_replay_start_frame(
  frames: &[LayoutFrame],
  request: &ReflowRequest,
) -> Option<usize> {
  let target_page_index = request.page_index.checked_sub(1)?;
  frames
    .iter()
    .enumerate()
    .filter(|(_, frame)| {
      frame.page_index == target_page_index
        && (matches!(request.scope, ReflowScope::Page)
          || frame.section_page_index + 1 >= request.section_page_index)
    })
    .map(|(index, _)| index)
    .next()
}

#[derive(Clone, Debug, Default)]
struct ReflowReplayStats {
  frames: usize,
  items: usize,
  frame_indices: Vec<usize>,
  pages: Vec<PageReplay>,
}

fn replay_scoped_frames(frames: &mut [LayoutFrame], request: &ReflowRequest) -> ReflowReplayStats {
  let mut stats = ReflowReplayStats::default();
  let Some(start) = frames.get(request.frame_index) else {
    return stats;
  };
  let start_page_index = start.page_index;
  let start_section_page_index = start.section_page_index;
  let start_column_index = start.column_index;

  for (frame_index, frame) in frames.iter_mut().enumerate().skip(request.frame_index) {
    if !frame_in_reflow_scope(
      frame,
      request.scope,
      start_page_index,
      start_section_page_index,
      start_column_index,
    ) {
      break;
    }
    let replay = page_replay_for_frame(frame, request.scope);
    replay_layout_frame(frame);
    stats.frames += 1;
    stats.items += frame.items.len();
    stats.frame_indices.push(frame_index);
    stats.pages.push(replay);
  }
  stats
}

fn page_replay_for_frame(frame: &LayoutFrame, scope: ReflowScope) -> PageReplay {
  PageReplay {
    page_index: frame.page_index,
    section_page_index: frame.section_page_index,
    column_index: frame.column_index,
    scope,
    item_start: frame.item_start,
    item_end: frame.item_end,
    replacement_items: frame.items.clone(),
  }
}

fn frame_in_reflow_scope(
  frame: &LayoutFrame,
  scope: ReflowScope,
  page_index: usize,
  section_page_index: usize,
  column_index: usize,
) -> bool {
  match scope {
    ReflowScope::Frame => frame.page_index == page_index && frame.column_index == column_index,
    ReflowScope::Column => {
      frame.page_index == page_index
        && frame.section_page_index == section_page_index
        && frame.column_index == column_index
    }
    ReflowScope::Page => frame.page_index == page_index,
  }
}

fn replay_layout_frame(frame: &mut LayoutFrame) {
  frame.item_start = 0;
  frame.item_end = frame.items.len();
  frame.bounds = frame_bounds_for_items(&frame.items);
  frame.lines = line_boxes_for_items(&frame.items, 0, frame.items.len());
  let fallback_fragments = frame_fragments_for(frame.kind, &frame.lines);
  if frame.fragments.is_empty() {
    frame.fragments = fallback_fragments;
  } else {
    normalize_replayed_fragments(
      &mut frame.fragments,
      &fallback_fragments,
      frame.kind,
      frame.items.len(),
    );
  }
  frame.split_start = frame_cursor(frame.block_index, frame.kind, 0, &frame.lines, true);
  frame.split_end = frame_cursor(
    frame.block_index,
    frame.kind,
    frame.items.len(),
    &frame.lines,
    false,
  );
  frame.invalidation = FrameInvalidation::Clean;
}

fn normalize_replayed_fragments(
  fragments: &mut Vec<FrameFragment>,
  fallback: &[FrameFragment],
  kind: FollowFrameKind,
  item_len: usize,
) {
  if matches!(kind, FollowFrameKind::Table) {
    if item_len == 0 {
      fragments.clear();
      return;
    }
    for fragment in fragments {
      fragment.item_start = fragment.item_start.min(item_len.saturating_sub(1));
      fragment.item_end = fragment.item_end.min(item_len).max(fragment.item_start + 1);
      if fragment.bounds.is_none() {
        fragment.bounds = fallback.first().and_then(|fragment| fragment.bounds);
      }
    }
    return;
  }
  if fragments.len() != fallback.len() {
    *fragments = fallback.to_vec();
    return;
  }
  for (fragment, fallback) in fragments.iter_mut().zip(fallback) {
    fragment.kind = match kind {
      FollowFrameKind::Paragraph => FrameFragmentKind::ParagraphLine,
      FollowFrameKind::Table => fragment.kind,
      FollowFrameKind::Notes => FrameFragmentKind::NoteLine,
    };
    fragment.item_start = fallback.item_start;
    fragment.item_end = fallback.item_end;
    fragment.bounds = fallback.bounds;
  }
}

fn page_invalidations_for_reflow_requests(requests: &[ReflowRequest]) -> Vec<PageInvalidation> {
  let mut invalidations = Vec::<PageInvalidation>::new();
  for request in requests {
    match invalidations
      .iter_mut()
      .find(|invalidation| invalidation.page_index == request.page_index)
    {
      Some(invalidation) => {
        invalidation.first_frame_index = invalidation.first_frame_index.min(request.frame_index);
        invalidation.scope = invalidation.scope.max(request.scope);
        if reflow_reason_priority(request.reason) > reflow_reason_priority(invalidation.reason) {
          invalidation.reason = request.reason;
        }
      }
      None => invalidations.push(PageInvalidation {
        page_index: request.page_index,
        section_page_index: request.section_page_index,
        first_frame_index: request.frame_index,
        reason: request.reason,
        scope: request.scope,
      }),
    }
  }
  invalidations
    .sort_by_key(|invalidation| (invalidation.page_index, invalidation.first_frame_index));
  invalidations
}

fn reflow_reason_priority(reason: ReflowReason) -> u8 {
  match reason {
    ReflowReason::DecorationChangedItems => 0,
    ReflowReason::InsertionInfluenceChanged => 1,
    ReflowReason::InvalidBounds => 2,
  }
}

fn restart_plan_for_page_invalidations(
  frames: &[LayoutFrame],
  invalidations: &[PageInvalidation],
) -> Option<RestartPlan> {
  let invalidation = invalidations.first()?;
  let frame = frames.get(invalidation.first_frame_index)?;
  Some(RestartPlan {
    page_index: invalidation.page_index,
    frame_index: invalidation.first_frame_index,
    block_index: frame.block_index,
    cursor: frame.split_start,
    reason: invalidation.reason,
    scope: invalidation.scope,
  })
}

fn item_line_y(item: &PageItem) -> Option<f32> {
  match item {
    PageItem::Text(text) => Some(text.y_pt),
    PageItem::Image(image) if !image.floating => Some(image.y_pt),
    PageItem::Image(_)
    | PageItem::Rect(_)
    | PageItem::Fill(_)
    | PageItem::Line(_)
    | PageItem::Polyline(_) => None,
  }
}

fn item_bounds(item: &PageItem) -> Option<(f32, f32, f32, f32)> {
  match item {
    PageItem::Text(text) => {
      let width = measure_text(&text.text, &text.style);
      Some((
        text.x_pt,
        text.y_pt,
        text.x_pt + width,
        text.y_pt + text.line_height_pt,
      ))
    }
    PageItem::Image(image) => Some((
      image.x_pt,
      image.y_pt,
      image.x_pt + image.width_pt,
      image.y_pt + image.height_pt,
    )),
    PageItem::Rect(rect) => Some((
      rect.x_pt,
      rect.y_pt,
      rect.x_pt + rect.width_pt,
      rect.y_pt + rect.height_pt,
    )),
    PageItem::Fill(fill) => Some((
      fill.x_pt,
      fill.y_pt,
      fill.x_pt + fill.width_pt,
      fill.y_pt + fill.height_pt,
    )),
    PageItem::Line(line) => {
      let half_width = line.width_pt / 2.0;
      Some((
        line.x1_pt.min(line.x2_pt) - half_width,
        line.y1_pt.min(line.y2_pt) - half_width,
        line.x1_pt.max(line.x2_pt) + half_width,
        line.y1_pt.max(line.y2_pt) + half_width,
      ))
    }
    PageItem::Polyline(polyline) => Some((
      polyline.x_pt,
      polyline.y_pt,
      polyline.x_pt + polyline.width_pt,
      polyline.y_pt + polyline.height_pt,
    )),
  }
}

fn page_items_bounds(items: &[PageItem]) -> Option<(f32, f32, f32, f32)> {
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  for item in items {
    let Some((x1, y1, x2, y2)) = item_bounds(item) else {
      continue;
    };
    bounds = Some(match bounds {
      Some((left, top, right, bottom)) => {
        (left.min(x1), top.min(y1), right.max(x2), bottom.max(y2))
      }
      None => (x1, y1, x2, y2),
    });
  }
  bounds
}

fn item_vertical_bounds(item: &PageItem) -> (f32, f32) {
  match item {
    PageItem::Text(text) => (text.y_pt, text.y_pt + text.line_height_pt),
    PageItem::Image(image) => (image.y_pt, image.y_pt + image.height_pt),
    PageItem::Rect(rect) => (rect.y_pt, rect.y_pt + rect.height_pt),
    PageItem::Fill(fill) => (fill.y_pt, fill.y_pt + fill.height_pt),
    PageItem::Line(line) => {
      let half_width = line.width_pt / 2.0;
      (
        line.y1_pt.min(line.y2_pt) - half_width,
        line.y1_pt.max(line.y2_pt) + half_width,
      )
    }
    PageItem::Polyline(polyline) => (polyline.y_pt, polyline.y_pt + polyline.height_pt),
  }
}

fn page_items_vertical_bounds(items: &[PageItem]) -> Option<(f32, f32)> {
  let mut bounds: Option<(f32, f32)> = None;
  for item in items {
    let (top, bottom) = item_vertical_bounds(item);
    bounds = Some(match bounds {
      Some((current_top, current_bottom)) => (current_top.min(top), current_bottom.max(bottom)),
      None => (top, bottom),
    });
  }
  bounds
}

fn layout_follow_reason_for_block(block: &Block) -> FollowReason {
  let Block::Paragraph(paragraph) = block else {
    return FollowReason::Overflow;
  };
  if paragraph.format.page_break_before
    || paragraph
      .inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::PageBreak | InlineItem::ColumnBreak))
  {
    FollowReason::ExplicitBreak
  } else {
    FollowReason::Overflow
  }
}

fn block_is_page_break_only_paragraph(block: &Block) -> bool {
  let Block::Paragraph(paragraph) = block else {
    return false;
  };
  let mut saw_page_break = false;
  for inline in &paragraph.inlines {
    match inline {
      InlineItem::PageBreak => saw_page_break = true,
      InlineItem::LastRenderedPageBreak | InlineItem::ColumnBreak => {}
      InlineItem::Text(run) if run.text.trim().is_empty() => {}
      InlineItem::Text(_) | InlineItem::Image(_) | InlineItem::Shape(_) => return false,
      InlineItem::FormWidgetStart(_) | InlineItem::FormWidgetEnd(_) => return false,
    }
  }
  saw_page_break
}

fn block_is_empty_paragraph(block: &Block) -> bool {
  let Block::Paragraph(paragraph) = block else {
    return false;
  };
  paragraph.inlines.iter().all(|inline| match inline {
    InlineItem::LastRenderedPageBreak => true,
    InlineItem::Text(run) => run.text.trim().is_empty(),
    InlineItem::PageBreak
    | InlineItem::ColumnBreak
    | InlineItem::Image(_)
    | InlineItem::Shape(_)
    | InlineItem::FormWidgetStart(_)
    | InlineItem::FormWidgetEnd(_) => false,
  })
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
  empty_section_page(setup, section_index, 0)
}

fn empty_section_page(setup: PageSetup, section_index: usize, section_page_index: usize) -> Page {
  Page {
    setup,
    section_index,
    section_page_index,
    items: Vec::new(),
    preserve_empty: false,
    frame_fragments: Vec::new(),
    frame_influences: Vec::new(),
    wrap_exclusions: Vec::new(),
    repeating_wrap_exclusion_catalog: RepeatingWrapExclusionCatalog::default(),
    repeating_wrap_exclusions: Vec::new(),
    pending_floating_table_follows: Vec::new(),
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
    section_page_index: 0,
    column_index,
    columns,
    content_top_pt: setup.margin_top_pt,
    content_left_pt,
    content_bottom: setup.height_pt - setup.margin_bottom_pt,
    body_content_bottom_pt: setup.height_pt - setup.margin_bottom_pt,
    content_width: column_width.max(DEFAULT_FONT_SIZE_PT),
    layout_cell_bounds: None,
    default_tab_stop_pt,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::Body,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
  }
}

fn flow_with_column(flow: FlowContext, column_index: usize) -> FlowContext {
  let geometry = column_geometry(flow.setup, flow.columns);
  let count = geometry.widths.len().max(1);
  let column_index = column_index.min(count - 1);
  FlowContext {
    column_index,
    content_left_pt: geometry.lefts[column_index],
    content_width: geometry.widths[column_index].max(DEFAULT_FONT_SIZE_PT),
    ..flow
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
    let next_flow = flow_with_column(flow, next_column);
    (next_flow, next_flow.content_top_pt)
  } else {
    let next_page_number = pages.len() + 2;
    let next_section_page_index = flow.section_page_index + 1;
    let mut next_page = empty_section_page(flow.setup, flow.section_index, next_section_page_index);
    next_page.repeating_wrap_exclusion_catalog = current.repeating_wrap_exclusion_catalog.clone();
    next_page.repeating_wrap_exclusions = next_page
      .repeating_wrap_exclusion_catalog
      .selected(next_section_page_index, next_page_number)
      .to_vec();
    extend_wrap_exclusions_unique(
      &mut next_page.wrap_exclusions,
      &next_page.repeating_wrap_exclusions,
    );
    pages.push(std::mem::replace(current, next_page));
    activate_pending_floating_table_follows_for_current(current, pages);
    let next_flow = body_flow_for_page(
      flow_with_column(
        FlowContext {
          section_page_index: flow.section_page_index + 1,
          ..flow
        },
        0,
      ),
      pages.len() + 1,
    );
    (next_flow, next_flow.content_top_pt)
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
  if y + required_height <= flow.content_bottom || y <= flow.content_top_pt + LAYOUT_EPSILON_PT {
    return (flow, y);
  }
  advance_section_flow(flow, current, pages)
}

fn block_should_stay_together(block: &Block, next: Option<&Block>) -> bool {
  match block {
    Block::Paragraph(paragraph) => {
      paragraph.format.keep_lines
        || (paragraph.format.keep_with_next && matches!(next, Some(Block::Paragraph(_))))
    }
    Block::Table(_) | Block::Frame(_) => false,
  }
}

fn keep_group_height(block: &Block, next: Option<&Block>, flow: FlowContext) -> f32 {
  let mut height = estimated_block_height(block, flow);
  if let Block::Paragraph(paragraph) = block
    && paragraph.format.keep_with_next
    && let Some(Block::Paragraph(next)) = next
  {
    height += estimated_paragraph_height(next, flow);
  }
  height
}

fn estimated_block_height(block: &Block, flow: FlowContext) -> f32 {
  match block {
    Block::Paragraph(paragraph) => estimated_paragraph_height(paragraph, flow),
    Block::Table(table) => estimated_table_height(table, flow),
    Block::Frame(frame) => estimated_frame_height(frame, flow),
  }
}

fn estimated_paragraph_height(paragraph: &crate::docx::Paragraph, flow: FlowContext) -> f32 {
  let content_width =
    (flow.content_width - paragraph.format.indent_left_pt - paragraph.format.indent_right_pt)
      .max(DEFAULT_FONT_SIZE_PT);
  let base_line_style = paragraph_base_line_style(paragraph);
  let base_line_height = paragraph_line_height_for_setup(
    paragraph,
    &base_line_style,
    flow.setup,
    flow.text_segmentation,
  );
  let text_frame = TextFrame {
    default_line_left: 0.0,
    first_line_left: 0.0,
    default_line_right: content_width,
    paragraph_left: 0.0,
    base_line_height,
    line_height_rule: paragraph.format.line_height_rule,
  };
  let mut line_height = base_line_height;
  let mut content_height = 0.0;
  let mut floating_bottom: f32 = 0.0;
  let mut x = (paragraph.format.first_line_indent_pt).max(0.0);
  let finish_line = |content_height: &mut f32, line_height: &mut f32| {
    *content_height += line_real_height(paragraph, *line_height, false);
    *line_height = base_line_height;
  };

  for item in &paragraph.inlines {
    match item {
      InlineItem::Text(run) => {
        for segment in text_segments(&run.text) {
          if segment == "\n" || segment == "\t" {
            finish_line(&mut content_height, &mut line_height);
            x = 0.0;
            continue;
          }
          let width = measure_text(&segment, &run.style);
          if x + width > content_width && x > 0.0 {
            finish_line(&mut content_height, &mut line_height);
            x = 0.0;
          }
          x += width.min(content_width);
          if segment_affects_line_height(&segment) {
            line_height = include_text_height(line_height, text_frame, &run.style);
          }
        }
      }
      InlineItem::Image(image) => {
        if let crate::docx::ImagePlacement::Floating(placement) = image.placement
          && (placement.layout_in_cell || flow.text_segmentation == TextSegmentation::RepeatingSlot)
          && matches!(
            flow.text_segmentation,
            TextSegmentation::TableCell | TextSegmentation::RepeatingSlot
          )
        {
          let frame_width =
            relative_floating_width(placement, flow).unwrap_or_else(|| image_frame_width(image));
          let frame_height =
            relative_floating_height(placement, flow).unwrap_or_else(|| image_frame_height(image));
          let (width, height) = fit_image_to_line(frame_width, frame_height, flow.content_width);
          let (_, image_y) =
            floating_image_position(placement, flow, x, content_height, width, height);
          floating_bottom = floating_bottom.max(image_y + height + placement.margin_bottom_pt);
          continue;
        }
        let (width, height) = fit_image_to_line(
          visible_image_width(image),
          visible_image_height(image),
          content_width,
        );
        if x + width > content_width && x > 0.0 {
          finish_line(&mut content_height, &mut line_height);
        }
        line_height = line_height.max(height.min(base_line_height));
        x = width;
      }
      InlineItem::Shape(shape) => {
        if let crate::docx::ImagePlacement::Floating(placement) = shape.placement
          && (placement.layout_in_cell || flow.text_segmentation == TextSegmentation::RepeatingSlot)
          && matches!(
            flow.text_segmentation,
            TextSegmentation::TableCell | TextSegmentation::RepeatingSlot
          )
        {
          let width = relative_floating_width(placement, flow).unwrap_or(shape.width_pt);
          let height = relative_floating_height(placement, flow).unwrap_or(shape.height_pt);
          let (_, shape_y) =
            floating_image_position(placement, flow, x, content_height, width, height);
          floating_bottom = floating_bottom.max(shape_y + height + placement.margin_bottom_pt);
          continue;
        }
        if x + shape.width_pt > content_width && x > 0.0 {
          finish_line(&mut content_height, &mut line_height);
        }
        line_height = line_height.max(shape.height_pt);
        x = shape.width_pt;
      }
      InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::LastRenderedPageBreak => {}
      InlineItem::PageBreak | InlineItem::ColumnBreak => {
        finish_line(&mut content_height, &mut line_height);
        x = 0.0;
      }
    }
  }
  finish_line(&mut content_height, &mut line_height);
  content_height = content_height.max(floating_bottom);

  paragraph.format.spacing_before_pt
    + content_height
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

fn paragraph_spacing_before(
  previous: Option<&Block>,
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
) -> f32 {
  if previous.is_none()
    && matches!(
      flow.text_segmentation,
      TextSegmentation::Body | TextSegmentation::RepeatingSlot
    )
    && flow.section_page_index > 0
    && flow.paragraph_spacing_context != ParagraphSpacingContext::SectionStart
  {
    return 0.0;
  }
  if previous.is_none()
    && flow.text_segmentation == TextSegmentation::TableCell
    && paragraph_starts_with_last_rendered_page_break(paragraph)
  {
    return 0.0;
  }
  if flow.paragraph_spacing_context == ParagraphSpacingContext::SectionStart
    && let Some(Block::Paragraph(previous)) = previous
  {
    return section_start_spacing_before(previous, paragraph);
  }
  if let Some(Block::Paragraph(previous)) = previous
    && suppress_contextual_spacing(previous, paragraph)
  {
    return 0.0;
  }
  paragraph.format.spacing_before_pt
}

fn section_start_spacing_before(
  previous: &crate::docx::Paragraph,
  paragraph: &crate::docx::Paragraph,
) -> f32 {
  if paragraph.format.contextual_spacing {
    return 0.0;
  }
  (paragraph.format.spacing_before_pt - previous.format.spacing_after_pt).max(0.0)
}

fn paragraph_starts_with_last_rendered_page_break(paragraph: &crate::docx::Paragraph) -> bool {
  paragraph
    .inlines
    .iter()
    .find(|inline| {
      !matches!(
        inline,
        InlineItem::FormWidgetStart(_) | InlineItem::FormWidgetEnd(_)
      )
    })
    .is_some_and(|inline| matches!(inline, InlineItem::LastRenderedPageBreak))
}

fn segment_affects_line_height(text: &str) -> bool {
  !text.is_empty() && !text.chars().all(libreoffice_ignored_line_height_blank)
}

fn libreoffice_ignored_line_height_blank(ch: char) -> bool {
  // Source: LibreOffice sw/source/core/text/porlay.cxx lcl_HasOnlyBlanks().
  matches!(ch, ' ' | '\u{2002}' | '\u{2003}' | '\u{2005}' | '\u{3000}')
}

fn paragraph_spacing_after(paragraph: &crate::docx::Paragraph, next: Option<&Block>) -> f32 {
  if let Some(Block::Paragraph(next)) = next
    && suppress_contextual_spacing(paragraph, next)
  {
    return 0.0;
  }
  paragraph
    .format
    .spacing_after_pt
    .max(PARAGRAPH_SPACING_AFTER_PT)
}

fn suppress_contextual_spacing(
  first: &crate::docx::Paragraph,
  second: &crate::docx::Paragraph,
) -> bool {
  if !first.format.contextual_spacing || !second.format.contextual_spacing {
    return false;
  }
  match (&first.format.style_id, &second.format.style_id) {
    (Some(first_style), Some(second_style)) => first_style == second_style,
    _ => first.format == second.format,
  }
}

fn layout_document_block(
  previous: Option<&Block>,
  block: &Block,
  next: Option<&Block>,
  mut flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
) -> (FlowContext, f32) {
  match block {
    Block::Paragraph(paragraph) => {
      let mut ignore_top_margin_at_page_start = false;
      if paragraph.starts_after_last_rendered_page_break
        && flow.text_segmentation == TextSegmentation::Body
        && y > flow.content_top_pt + LAYOUT_EPSILON_PT
        && !current.items.is_empty()
      {
        (flow, y) = force_page_break(flow, current, pages);
        ignore_top_margin_at_page_start = true;
      }
      if let Some(frame) = paragraph_frame(paragraph) {
        return layout_floating_frame(&frame, flow, current, pages, y);
      }
      let mut flow = flow;
      let ignore_top_margin_after_page_break = paragraph.format.page_break_before
        && !current.items.is_empty()
        && flow.text_segmentation == TextSegmentation::Body;
      if paragraph.format.page_break_before
        && !current.items.is_empty()
        && flow.text_segmentation == TextSegmentation::Body
      {
        (flow, y) = force_page_break(flow, current, pages);
        ignore_top_margin_at_page_start = true;
      }

      if paragraph.starts_after_last_rendered_page_break
        && flow.text_segmentation == TextSegmentation::Body
        && flow.paragraph_spacing_context != ParagraphSpacingContext::SectionStart
        && y <= flow.content_top_pt + LAYOUT_EPSILON_PT
      {
        ignore_top_margin_at_page_start = true;
      }

      if !ignore_top_margin_after_page_break && !ignore_top_margin_at_page_start {
        y += paragraph_spacing_before(previous, paragraph, flow);
      }
      let paragraph_flow = FlowContext {
        content_width: (flow.content_width
          - paragraph.format.indent_left_pt
          - paragraph.format.indent_right_pt)
          .max(DEFAULT_FONT_SIZE_PT),
        ..flow
      };
      let (paragraph_flow, y) = layout_paragraph(
        paragraph,
        paragraph_flow,
        current,
        pages,
        y,
        paragraph_spacing_after(paragraph, next),
      );
      (
        FlowContext {
          content_width: flow.content_width,
          ..paragraph_flow
        },
        y,
      )
    }
    Block::Table(table) => layout_table(table, flow, current, pages, y),
    Block::Frame(frame) => layout_floating_frame(frame, flow, current, pages, y),
  }
}

fn paragraph_frame(paragraph: &crate::docx::Paragraph) -> Option<FloatingFrame> {
  let frame = paragraph.format.frame?;
  let mut content = paragraph.clone();
  content.format.frame = None;
  Some(FloatingFrame {
    blocks: vec![Block::Paragraph(content)],
    width_pt: frame.width_pt,
    height_pt: frame.height_pt,
    height_rule: frame.height_rule,
    placement: frame.placement,
    fill_color: paragraph.format.shading,
    borders: paragraph.format.borders,
  })
}

fn layout_floating_frame(
  frame: &FloatingFrame,
  flow: FlowContext,
  current: &mut Page,
  _pages: &mut Vec<Page>,
  y: f32,
) -> (FlowContext, f32) {
  let width = frame
    .width_pt
    .unwrap_or_else(|| default_floating_frame_width(frame.placement, flow))
    .max(DEFAULT_FONT_SIZE_PT);
  let height = estimated_frame_height(frame, flow);
  let (x, frame_y) = floating_frame_position(frame.placement, flow, y, width, height);
  let frame_stroke = frame
    .borders
    .top
    .or(frame.borders.left)
    .or(frame.borders.right)
    .or(frame.borders.bottom);
  current.items.push(PageItem::Rect(RectItem {
    x_pt: x,
    y_pt: frame_y,
    width_pt: width,
    height_pt: height,
    fill_color: frame.fill_color,
    fill_opacity: 1.0,
    stroke: frame_stroke.or_else(|| frame.fill_color.is_none().then_some(BorderStyle::default())),
    stroke_opacity: if frame_stroke.is_some() { 1.0 } else { 0.0 },
  }));
  let frame_flow = FlowContext {
    content_top_pt: frame_y,
    content_left_pt: x,
    content_bottom: if matches!(frame.height_rule, FrameHeightRule::Exact) {
      frame_y + height
    } else {
      UNBOUNDED_LAYOUT_EXTENT_PT
    },
    body_content_bottom_pt: if matches!(frame.height_rule, FrameHeightRule::Exact) {
      frame_y + height
    } else {
      UNBOUNDED_LAYOUT_EXTENT_PT
    },
    content_width: width,
    ..flow
  };
  let mut frame_page = empty_page(flow.setup, current.section_index);
  let mut frame_pages = Vec::new();
  let mut block_y = frame_y;
  for (index, block) in frame.blocks.iter().enumerate() {
    let previous = index
      .checked_sub(1)
      .and_then(|index| frame.blocks.get(index));
    let next = frame.blocks.get(index + 1);
    let (_, next_y) = layout_document_block(
      previous,
      block,
      next,
      frame_flow,
      &mut frame_page,
      &mut frame_pages,
      block_y,
    );
    block_y = next_y;
  }
  let visible_page = frame_pages.into_iter().next().unwrap_or(frame_page);
  current
    .items
    .extend(visible_page.items.into_iter().filter(|item| {
      !matches!(frame.height_rule, FrameHeightRule::Exact)
        || table_cell_item_intersects_vertical_bounds(item, frame_y, frame_y + height)
    }));
  let occupied_bottom = frame_y + height.max(block_y - frame_y) + frame.placement.margin_bottom_pt;
  if floating_frame_blocks_flow(frame) {
    (flow, y.max(occupied_bottom))
  } else {
    (flow, y)
  }
}

fn default_floating_frame_width(placement: FloatingFramePlacement, flow: FlowContext) -> f32 {
  match placement.horizontal_anchor {
    FrameHorizontalAnchor::Page => flow.setup.width_pt,
    FrameHorizontalAnchor::Margin => {
      flow.setup.width_pt - flow.setup.margin_left_pt - flow.setup.margin_right_pt
    }
    FrameHorizontalAnchor::Text => flow.content_width,
  }
  .max(DEFAULT_FONT_SIZE_PT)
}

fn frame_content_height(frame: &FloatingFrame, flow: FlowContext) -> f32 {
  frame
    .blocks
    .iter()
    .map(|block| estimated_block_height(block, flow))
    .sum::<f32>()
    .max(DEFAULT_LINE_HEIGHT_PT)
}

fn estimated_frame_height(frame: &FloatingFrame, flow: FlowContext) -> f32 {
  let width = frame
    .width_pt
    .unwrap_or_else(|| default_floating_frame_width(frame.placement, flow))
    .max(DEFAULT_FONT_SIZE_PT);
  let measured_height = frame_content_height(
    frame,
    FlowContext {
      content_width: width,
      ..flow
    },
  );
  match frame.height_rule {
    FrameHeightRule::Auto => frame.height_pt.unwrap_or(measured_height),
    FrameHeightRule::AtLeast => frame.height_pt.unwrap_or(0.0).max(measured_height),
    FrameHeightRule::Exact => frame.height_pt.unwrap_or(measured_height),
  }
  .max(DEFAULT_LINE_HEIGHT_PT)
}

fn block_area(flow: FlowContext) -> BlockArea {
  BlockArea {
    setup: flow.setup,
    section_index: flow.section_index,
    section_page_index: flow.section_page_index,
    column_index: flow.column_index,
    columns: flow.columns,
    content_top_pt: flow.content_top_pt,
    content_left_pt: flow.content_left_pt,
    content_bottom: flow.content_bottom,
    body_content_bottom_pt: flow.body_content_bottom_pt,
    content_width: flow.content_width,
    default_tab_stop_pt: flow.default_tab_stop_pt,
    repeating_slots: flow.repeating_slots,
  }
}

fn flow_from_block_area(area: BlockArea) -> FlowContext {
  FlowContext {
    setup: area.setup,
    section_index: area.section_index,
    section_page_index: area.section_page_index,
    column_index: area.column_index,
    columns: area.columns,
    content_top_pt: area.content_top_pt,
    content_left_pt: area.content_left_pt,
    content_bottom: area.content_bottom,
    body_content_bottom_pt: area.body_content_bottom_pt,
    content_width: area.content_width,
    layout_cell_bounds: None,
    default_tab_stop_pt: area.default_tab_stop_pt,
    repeating_slots: area.repeating_slots,
    text_segmentation: TextSegmentation::Body,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
  }
}

fn restore_body_content_bottom(flow: FlowContext) -> FlowContext {
  FlowContext {
    content_bottom: flow.body_content_bottom_pt,
    ..flow
  }
}

fn add_endnote_continuation_separators(
  setup: PageSetup,
  pages: &mut [Page],
  current: &mut Page,
  endnote_start_page_index: usize,
) {
  for page in pages.iter_mut().skip(endnote_start_page_index + 1) {
    add_note_separator_line(
      NoteSeparatorKind::EndnoteContinuation,
      setup,
      page,
      setup.margin_top_pt,
    );
  }
  if pages.len() > endnote_start_page_index {
    add_note_separator_line(
      NoteSeparatorKind::EndnoteContinuation,
      setup,
      current,
      setup.margin_top_pt,
    );
  }
}

fn layout_note_separator(
  kind: NoteSeparatorKind,
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

  y += LO_FOOTNOTE_SEPARATOR_TOP_DIST_PT;
  add_note_separator_line(kind, setup, current, y);
  let bottom_dist = match kind {
    NoteSeparatorKind::Footnote => LO_FOOTNOTE_SEPARATOR_BOTTOM_DIST_PT,
    NoteSeparatorKind::Endnote | NoteSeparatorKind::EndnoteContinuation => {
      LO_ENDNOTE_SEPARATOR_BOTTOM_DIST_PT
    }
  };
  y + bottom_dist
}

fn add_note_separator_line(kind: NoteSeparatorKind, setup: PageSetup, current: &mut Page, y: f32) {
  let content_width = (setup.width_pt - setup.margin_left_pt - setup.margin_right_pt).max(0.0);
  let separator_width = match kind {
    NoteSeparatorKind::Footnote => content_width * LO_FOOTNOTE_SEPARATOR_WIDTH_FRACTION,
    NoteSeparatorKind::Endnote => LO_ENDNOTE_SEPARATOR_WIDTH_PT.min(content_width),
    NoteSeparatorKind::EndnoteContinuation => content_width,
  };
  current.items.push(PageItem::Line(LineItem {
    x1_pt: setup.margin_left_pt,
    y1_pt: y,
    x2_pt: setup.margin_left_pt + separator_width,
    y2_pt: y + LO_FOOTNOTE_SEPARATOR_STROKE_PT,
    width_pt: LO_FOOTNOTE_SEPARATOR_STROKE_PT,
    color: RgbColor { r: 0, g: 0, b: 0 },
    kind: LineItemKind::FilledRect,
  }));
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

  let available_height = flow.content_bottom - flow.content_top_pt;
  let reserved_height = height.min(available_height);
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
    height += measured_note_blocks_height(blocks, flow);
  }
  height
}

fn measured_note_blocks_height(blocks: &[Block], flow: FlowContext) -> f32 {
  let mut scratch = empty_section_page(
    PageSetup {
      width_pt: flow.setup.width_pt,
      height_pt: MEASURE_SCRATCH_PAGE_HEIGHT_PT,
      margin_left_pt: flow.setup.margin_left_pt,
      margin_right_pt: flow.setup.margin_right_pt,
      margin_top_pt: 0.0,
      margin_bottom_pt: 0.0,
      ..flow.setup
    },
    flow.section_index,
    flow.section_page_index,
  );
  let mut discarded_pages = Vec::new();
  let note_flow = FlowContext {
    setup: scratch.setup,
    content_top_pt: 0.0,
    content_left_pt: flow.setup.margin_left_pt,
    content_width: flow.setup.width_pt - flow.setup.margin_left_pt - flow.setup.margin_right_pt,
    content_bottom: scratch.setup.height_pt,
    body_content_bottom_pt: scratch.setup.height_pt,
    columns: SectionColumns::default(),
    column_index: 0,
    repeating_slots: RepeatingSlotState::default(),
    ..flow
  };
  let mut y = 0.0;
  for (index, block) in blocks.iter().enumerate() {
    let previous = index.checked_sub(1).and_then(|index| blocks.get(index));
    let next = blocks.get(index + 1);
    let (_, next_y) = layout_document_block(
      previous,
      block,
      next,
      note_flow,
      &mut scratch,
      &mut discarded_pages,
      y,
    );
    y = next_y;
  }
  y
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
    content_top_pt: y,
    content_left_pt: flow.setup.margin_left_pt,
    content_width: flow.setup.width_pt - flow.setup.margin_left_pt - flow.setup.margin_right_pt,
    content_bottom: flow.setup.height_pt - flow.setup.margin_bottom_pt,
    body_content_bottom_pt: flow.setup.height_pt - flow.setup.margin_bottom_pt,
    columns: SectionColumns::default(),
    column_index: 0,
    text_segmentation: TextSegmentation::Notes,
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
        NoteSeparatorKind::Footnote,
        footnote_flow.setup,
        current,
        pages,
        y,
        footnote_flow.content_bottom,
      );
      needs_separator = false;
    }
    for (index, block) in blocks.iter().enumerate() {
      let previous = index.checked_sub(1).and_then(|index| blocks.get(index));
      let next = blocks.get(index + 1);
      let (_, next_y) =
        layout_document_block(previous, block, next, footnote_flow, current, pages, y);
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
      Block::Frame(frame) => collect_endnote_ids_from_blocks(&frame.blocks, ids),
    }
  }
}

fn repeating_slot_blocks_for_page<'a>(
  document: &'a DocxDocument,
  page: &Page,
  page_number: usize,
) -> (&'a [Block], &'a [Block]) {
  let first_page_in_section = page.section_page_index == 0;
  let section = document.sections.get(page.section_index);
  let title_page = section
    .map(|section| section.title_page)
    .unwrap_or(document.title_page);
  let (default_header, default_footer, first_header, first_footer, even_header, even_footer) =
    section
      .map(|section| {
        (
          section.header_blocks.as_slice(),
          section.footer_blocks.as_slice(),
          section.first_header_blocks.as_slice(),
          section.first_footer_blocks.as_slice(),
          section.even_header_blocks.as_slice(),
          section.even_footer_blocks.as_slice(),
        )
      })
      .unwrap_or((
        document.header_blocks.as_slice(),
        document.footer_blocks.as_slice(),
        document.first_header_blocks.as_slice(),
        document.first_footer_blocks.as_slice(),
        document.header_blocks.as_slice(),
        document.footer_blocks.as_slice(),
      ));
  let use_even_slot = document.even_and_odd_headers && page_number.is_multiple_of(2);
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
  (header_blocks, footer_blocks)
}

fn repeating_slot_wrap_exclusions_for_page(
  document: &DocxDocument,
  page: &Page,
  page_number: usize,
) -> Vec<WrapExclusion> {
  let (header_blocks, footer_blocks) = repeating_slot_blocks_for_page(document, page, page_number);
  if header_blocks.is_empty() && footer_blocks.is_empty() {
    return Vec::new();
  }

  let content_width =
    (page.setup.width_pt - page.setup.margin_left_pt - page.setup.margin_right_pt)
      .max(DEFAULT_FONT_SIZE_PT);
  let mut adornment = empty_section_page(page.setup, page.section_index, page.section_page_index);
  let mut discarded_pages = Vec::new();

  let header_height =
    measured_repeating_blocks_height(header_blocks, page.setup, document.default_tab_stop_pt);
  let header_top = page.setup.header_distance_pt.max(0.0);
  layout_repeating_blocks_into_page(
    header_blocks,
    &mut adornment,
    &mut discarded_pages,
    header_top,
    FlowContext {
      setup: page.setup,
      section_index: page.section_index,
      section_page_index: page.section_page_index,
      column_index: 0,
      columns: SectionColumns::default(),
      content_top_pt: header_top,
      content_left_pt: page.setup.margin_left_pt,
      content_bottom: header_top + header_height + DEFAULT_LINE_HEIGHT_PT,
      body_content_bottom_pt: header_top + header_height + DEFAULT_LINE_HEIGHT_PT,
      content_width,
      layout_cell_bounds: None,
      default_tab_stop_pt: document.default_tab_stop_pt,
      repeating_slots: repeating_slot_state(document, page.section_index),
      text_segmentation: TextSegmentation::RepeatingSlot,
      paragraph_spacing_context: ParagraphSpacingContext::Normal,
    },
  );

  let footer_height =
    measured_repeating_blocks_height(footer_blocks, page.setup, document.default_tab_stop_pt);
  let footer_bottom = (page.setup.height_pt - page.setup.footer_distance_pt.max(0.0))
    .max(0.0)
    .min(page.setup.height_pt);
  let footer_top = (footer_bottom - footer_height).max(0.0);
  layout_repeating_blocks_into_page(
    footer_blocks,
    &mut adornment,
    &mut discarded_pages,
    footer_top,
    FlowContext {
      setup: page.setup,
      section_index: page.section_index,
      section_page_index: page.section_page_index,
      column_index: 0,
      columns: SectionColumns::default(),
      content_top_pt: footer_top,
      content_left_pt: page.setup.margin_left_pt,
      content_bottom: footer_bottom + DEFAULT_LINE_HEIGHT_PT,
      body_content_bottom_pt: footer_bottom + DEFAULT_LINE_HEIGHT_PT,
      content_width,
      layout_cell_bounds: None,
      default_tab_stop_pt: document.default_tab_stop_pt,
      repeating_slots: repeating_slot_state(document, page.section_index),
      text_segmentation: TextSegmentation::RepeatingSlot,
      paragraph_spacing_context: ParagraphSpacingContext::Normal,
    },
  );

  adornment.wrap_exclusions
}

fn repeating_wrap_exclusion_catalog_for_page(
  document: &DocxDocument,
  page: &Page,
) -> RepeatingWrapExclusionCatalog {
  let first_page_odd = empty_section_page(page.setup, page.section_index, 0);
  let first_page_even = empty_section_page(page.setup, page.section_index, 0);
  let default_page = empty_section_page(page.setup, page.section_index, 1);
  let even_page = empty_section_page(page.setup, page.section_index, 1);
  RepeatingWrapExclusionCatalog {
    first_odd: repeating_slot_wrap_exclusions_for_page(document, &first_page_odd, 1),
    first_even: repeating_slot_wrap_exclusions_for_page(document, &first_page_even, 2),
    even: repeating_slot_wrap_exclusions_for_page(document, &even_page, 2),
    default: repeating_slot_wrap_exclusions_for_page(document, &default_page, 1),
  }
}

fn layout_repeating_blocks_into_page(
  blocks: &[Block],
  page: &mut Page,
  discarded_pages: &mut Vec<Page>,
  mut y: f32,
  flow: FlowContext,
) -> f32 {
  for (index, block) in blocks.iter().enumerate() {
    y = layout_repeating_block(
      block,
      page,
      discarded_pages,
      y,
      flow,
      index + 1 == blocks.len(),
    );
  }
  y
}

fn extend_wrap_exclusions_unique(target: &mut Vec<WrapExclusion>, exclusions: &[WrapExclusion]) {
  for exclusion in exclusions {
    if !target.contains(exclusion) {
      target.push(*exclusion);
    }
  }
}

fn reset_wrap_exclusions_for_y(page: &Page, y: f32, target: &mut Vec<WrapExclusion>) {
  target.clear();
  target.extend(
    page
      .wrap_exclusions
      .iter()
      .copied()
      .filter(|exclusion| exclusion.bottom_pt > y),
  );
}

fn append_vertical_wrap_exclusion(
  page: &mut Page,
  flow: FlowContext,
  left_pt: f32,
  top_pt: f32,
  right_pt: f32,
  bottom_pt: f32,
) {
  if bottom_pt <= top_pt + LAYOUT_EPSILON_PT {
    return;
  }
  let block_left = flow.content_left_pt;
  let block_right = block_left + flow.content_width;
  let exclusion = WrapExclusion {
    left_pt: left_pt.min(block_left),
    right_pt: right_pt.max(block_right),
    top_pt,
    bottom_pt,
    side: ImageWrapSide::BothSides,
    blocks_flow: true,
  };
  extend_wrap_exclusions_unique(&mut page.wrap_exclusions, &[exclusion]);
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

  for (index, page) in pages.iter_mut().enumerate() {
    let (header_blocks, footer_blocks) = repeating_slot_blocks_for_page(document, page, index + 1);

    if header_blocks.is_empty() && footer_blocks.is_empty() {
      continue;
    }

    let content_width =
      (page.setup.width_pt - page.setup.margin_left_pt - page.setup.margin_right_pt)
        .max(DEFAULT_FONT_SIZE_PT);
    let mut adornment = empty_section_page(page.setup, page.section_index, page.section_page_index);
    let mut discarded_pages = Vec::new();
    let header_height =
      measured_repeating_blocks_height(header_blocks, page.setup, document.default_tab_stop_pt);
    let header_top = page.setup.header_distance_pt.max(0.0);
    layout_repeating_blocks_into_page(
      header_blocks,
      &mut adornment,
      &mut discarded_pages,
      header_top,
      FlowContext {
        setup: page.setup,
        section_index: page.section_index,
        section_page_index: page.section_page_index,
        column_index: 0,
        columns: SectionColumns::default(),
        content_top_pt: header_top,
        content_left_pt: page.setup.margin_left_pt,
        content_bottom: header_top + header_height + DEFAULT_LINE_HEIGHT_PT,
        body_content_bottom_pt: header_top + header_height + DEFAULT_LINE_HEIGHT_PT,
        content_width,
        layout_cell_bounds: None,
        default_tab_stop_pt: document.default_tab_stop_pt,
        repeating_slots: RepeatingSlotState::default(),
        text_segmentation: TextSegmentation::RepeatingSlot,
        paragraph_spacing_context: ParagraphSpacingContext::Normal,
      },
    );

    let footer_height =
      measured_repeating_blocks_height(footer_blocks, page.setup, document.default_tab_stop_pt);
    let footer_bottom = (page.setup.height_pt - page.setup.footer_distance_pt.max(0.0))
      .max(0.0)
      .min(page.setup.height_pt);
    let footer_top = (footer_bottom - footer_height).max(0.0);
    layout_repeating_blocks_into_page(
      footer_blocks,
      &mut adornment,
      &mut discarded_pages,
      footer_top,
      FlowContext {
        setup: page.setup,
        section_index: page.section_index,
        section_page_index: page.section_page_index,
        column_index: 0,
        columns: SectionColumns::default(),
        content_top_pt: footer_top,
        content_left_pt: page.setup.margin_left_pt,
        content_bottom: footer_bottom + DEFAULT_LINE_HEIGHT_PT,
        body_content_bottom_pt: footer_bottom + DEFAULT_LINE_HEIGHT_PT,
        content_width,
        layout_cell_bounds: None,
        default_tab_stop_pt: document.default_tab_stop_pt,
        repeating_slots: RepeatingSlotState::default(),
        text_segmentation: TextSegmentation::RepeatingSlot,
        paragraph_spacing_context: ParagraphSpacingContext::Normal,
      },
    );

    let item_offset = page.items.len();
    offset_page_frame_records(&mut adornment, item_offset);
    page.items.extend(adornment.items);
    page.frame_fragments.extend(adornment.frame_fragments);
    page.frame_influences.extend(adornment.frame_influences);
    extend_wrap_exclusions_unique(&mut page.wrap_exclusions, &adornment.wrap_exclusions);
  }
}

fn section_has_repeating_blocks(section: &crate::docx::ImportedSection) -> bool {
  !section.header_blocks.is_empty()
    || !section.footer_blocks.is_empty()
    || !section.first_header_blocks.is_empty()
    || !section.first_footer_blocks.is_empty()
    || !section.even_header_blocks.is_empty()
    || !section.even_footer_blocks.is_empty()
}

fn blocks_have_visible_body_content(blocks: &[Block]) -> bool {
  blocks.iter().any(block_has_visible_body_content)
}

fn block_has_visible_body_content(block: &Block) -> bool {
  match block {
    Block::Paragraph(paragraph) => paragraph.inlines.iter().any(|inline| match inline {
      InlineItem::Text(run) => !run.text.trim().is_empty(),
      InlineItem::Image(_) | InlineItem::Shape(_) => true,
      InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::LastRenderedPageBreak
      | InlineItem::PageBreak
      | InlineItem::ColumnBreak => false,
    }),
    Block::Table(_) | Block::Frame(_) => true,
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

fn place_floating_images(pages: &mut [Page]) {
  for page in pages {
    let mut behind_images = Vec::new();
    let mut foreground_images = Vec::new();
    let mut body_items = Vec::with_capacity(page.items.len());
    let mut background_count = 0;

    for item in page.items.drain(..) {
      if matches!(&item, PageItem::Image(image) if image.floating && image.behind_text) {
        behind_images.push(item);
      } else if matches!(&item, PageItem::Image(image) if image.floating) {
        foreground_images.push(item);
      } else {
        if is_page_background_item(&item, page.setup) {
          background_count += 1;
        }
        body_items.push(item);
      }
    }

    if behind_images.is_empty() && foreground_images.is_empty() {
      page.items = body_items;
      continue;
    }

    let mut ordered =
      Vec::with_capacity(body_items.len() + behind_images.len() + foreground_images.len());
    ordered.extend(body_items.drain(..background_count));
    ordered.extend(behind_images);
    ordered.extend(body_items);
    ordered.extend(foreground_images);
    page.items = ordered;
  }
}

fn is_page_background_item(item: &PageItem, setup: PageSetup) -> bool {
  matches!(
    item,
    PageItem::Fill(fill)
      if fill.x_pt == 0.0
        && fill.y_pt == 0.0
        && (fill.width_pt - setup.width_pt).abs() <= LAYOUT_EPSILON_PT
        && (fill.height_pt - setup.height_pt).abs() <= LAYOUT_EPSILON_PT
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
  let style_ref_candidates = style_ref_candidates_by_page(pages);
  for (page_index, page) in pages.iter_mut().enumerate() {
    let page_number = (page_index + 1).to_string();
    for item in &mut page.items {
      let PageItem::Text(text) = item else {
        continue;
      };
      match &text.dynamic_field {
        Some(DynamicFieldKind::Page) => text.text.clone_from(&page_number),
        Some(DynamicFieldKind::NumPages) => text.text.clone_from(&total_pages),
        Some(DynamicFieldKind::StyleRef {
          style_name,
          from_bottom,
        }) => {
          if let Some(value) =
            resolve_style_ref(&style_ref_candidates, page_index, style_name, *from_bottom)
          {
            text.text = value;
          }
        }
        None => {}
      }
    }
  }
}

#[derive(Clone, Debug)]
struct StyleRefCandidate {
  y_pt: f32,
  keys: Vec<Arc<str>>,
  text: Arc<str>,
}

fn style_ref_candidates_by_page(pages: &[Page]) -> Vec<Vec<StyleRefCandidate>> {
  pages
    .iter()
    .map(|page| {
      let mut candidates = Vec::new();
      for item in &page.items {
        let PageItem::Text(text) = item else {
          continue;
        };
        if text.style_ref_keys.is_empty() || text.dynamic_field.is_some() {
          continue;
        }
        let Some(style_ref_text) = &text.style_ref_text else {
          continue;
        };
        if candidates.iter().any(|candidate: &StyleRefCandidate| {
          f32::abs(candidate.y_pt - text.y_pt) < 0.01
            && candidate.keys == text.style_ref_keys
            && candidate.text == *style_ref_text
        }) {
          continue;
        }
        candidates.push(StyleRefCandidate {
          y_pt: text.y_pt,
          keys: text.style_ref_keys.clone(),
          text: style_ref_text.clone(),
        });
      }
      candidates.sort_by(|a, b| a.y_pt.total_cmp(&b.y_pt));
      candidates
    })
    .collect()
}

fn resolve_style_ref(
  pages: &[Vec<StyleRefCandidate>],
  page_index: usize,
  style_name: &str,
  from_bottom: bool,
) -> Option<String> {
  if let Some(text) = resolve_style_ref_on_page(&pages[page_index], style_name, from_bottom) {
    return Some(text);
  }
  for previous_index in (0..page_index).rev() {
    if let Some(text) = resolve_style_ref_on_page(&pages[previous_index], style_name, true) {
      return Some(text);
    }
  }
  for next_page in pages.iter().skip(page_index + 1) {
    if let Some(text) = resolve_style_ref_on_page(next_page, style_name, false) {
      return Some(text);
    }
  }
  None
}

fn resolve_style_ref_on_page(
  candidates: &[StyleRefCandidate],
  style_name: &str,
  from_bottom: bool,
) -> Option<String> {
  let iter: Box<dyn Iterator<Item = &StyleRefCandidate> + '_> = if from_bottom {
    Box::new(candidates.iter().rev())
  } else {
    Box::new(candidates.iter())
  };
  iter
    .filter(|candidate| style_ref_candidate_matches(candidate, style_name))
    .map(|candidate| candidate.text.to_string())
    .next()
}

fn style_ref_candidate_matches(candidate: &StyleRefCandidate, style_name: &str) -> bool {
  let target = normalized_style_ref_name(style_name);
  candidate
    .keys
    .iter()
    .any(|key| normalized_style_ref_name(key) == target)
}

fn normalized_style_ref_name(name: &str) -> String {
  let name = match name.trim() {
    "1" => "Heading 1",
    "2" => "Heading 2",
    "3" => "Heading 3",
    "4" => "Heading 4",
    "5" => "Heading 5",
    "6" => "Heading 6",
    "7" => "Heading 7",
    "8" => "Heading 8",
    "9" => "Heading 9",
    other => other,
  };
  name
    .chars()
    .filter(|ch| !ch.is_whitespace() && *ch != '-' && *ch != '_')
    .flat_map(char::to_lowercase)
    .collect()
}

fn apply_column_separators(document: &DocxDocument, pages: &mut [Page], frames: &[LayoutFrame]) {
  let mut section_bounds = HashMap::<(usize, usize), (f32, f32)>::new();
  for frame in frames
    .iter()
    .filter(|frame| matches!(frame.kind, FollowFrameKind::Paragraph))
  {
    let Some(section) = document.sections.get(frame.section_index) else {
      continue;
    };
    if !section.columns.separator || section.columns.count <= 1 {
      continue;
    }
    for line in &frame.lines {
      let top = line.y_pt;
      let bottom = line.y_pt + line.height_pt;
      section_bounds
        .entry((frame.page_index, frame.section_index))
        .and_modify(|bounds| {
          bounds.0 = bounds.0.min(top);
          bounds.1 = bounds.1.max(bottom);
        })
        .or_insert((top, bottom));
    }
  }

  for (page_index, page) in pages.iter_mut().enumerate() {
    for section_index in 0..document.sections.len() {
      let Some(&(top, bottom)) = section_bounds.get(&(page_index, section_index)) else {
        continue;
      };
      let Some(section) = document.sections.get(section_index) else {
        continue;
      };
      let columns = section.columns;
      if !columns.separator || columns.count <= 1 || bottom <= top {
        continue;
      }
      let separator_top = top + ((bottom - top) - DEFAULT_LINE_HEIGHT_PT).max(0.0);
      let separator_bottom = (bottom - LAYOUT_EPSILON_PT).max(separator_top);
      let geometry = column_geometry(page.setup, columns);
      for column_index in 1..geometry.widths.len() {
        let gap = geometry.gaps.get(column_index - 1).copied().unwrap_or(0.0);
        let x_left = geometry.lefts[column_index] - gap / 2.0;
        let x_right = x_left + LAYOUT_EPSILON_PT;
        page.items.push(PageItem::Line(LineItem {
          x1_pt: x_left,
          y1_pt: separator_top,
          x2_pt: x_right,
          y2_pt: separator_bottom,
          width_pt: 0.2,
          color: RgbColor { r: 0, g: 0, b: 0 },
          kind: LineItemKind::FilledRect,
        }));
      }
    }
  }
}

fn body_flow_for_page(flow: FlowContext, page_number: usize) -> FlowContext {
  let (content_top_pt, content_bottom) = body_content_limits_for_page(
    flow.setup,
    flow.repeating_slots,
    page_number,
    flow.section_page_index,
  );
  FlowContext {
    content_top_pt,
    content_bottom,
    body_content_bottom_pt: content_bottom,
    ..flow
  }
}

fn body_content_limits_for_page(
  setup: PageSetup,
  slots: RepeatingSlotState,
  page_number: usize,
  section_page_index: usize,
) -> (f32, f32) {
  let mut top = setup.margin_top_pt;
  let mut bottom = setup.height_pt - setup.margin_bottom_pt;
  let (has_header, has_footer, header_height, footer_height) =
    repeating_slots_present_for_page(slots, page_number, section_page_index);

  if has_header && !setup.top_margin_was_negative {
    top = top.max(setup.header_distance_pt.max(0.0) + header_height);
  }
  if has_footer && !setup.bottom_margin_was_negative {
    bottom = bottom.min(
      (setup.height_pt - setup.footer_distance_pt.max(0.0) - footer_height)
        .max(0.0)
        .min(setup.height_pt),
    );
  }
  if bottom < top + DEFAULT_LINE_HEIGHT_PT {
    bottom = (top + DEFAULT_LINE_HEIGHT_PT).min(setup.height_pt);
  }

  (top, bottom)
}

fn repeating_slot_state(document: &DocxDocument, section_index: usize) -> RepeatingSlotState {
  if let Some(section) = document.sections.get(section_index) {
    return RepeatingSlotState {
      title_page: section.title_page,
      even_and_odd_headers: document.even_and_odd_headers,
      default_header: !section.header_blocks.is_empty(),
      default_footer: !section.footer_blocks.is_empty(),
      first_header: !section.first_header_blocks.is_empty(),
      first_footer: !section.first_footer_blocks.is_empty(),
      even_header: !section.even_header_blocks.is_empty(),
      even_footer: !section.even_footer_blocks.is_empty(),
      default_header_height_pt: measured_repeating_blocks_height(
        &section.header_blocks,
        section.page,
        document.default_tab_stop_pt,
      ),
      default_footer_height_pt: measured_repeating_blocks_height(
        &section.footer_blocks,
        section.page,
        document.default_tab_stop_pt,
      ),
      first_header_height_pt: measured_repeating_blocks_height(
        &section.first_header_blocks,
        section.page,
        document.default_tab_stop_pt,
      ),
      first_footer_height_pt: measured_repeating_blocks_height(
        &section.first_footer_blocks,
        section.page,
        document.default_tab_stop_pt,
      ),
      even_header_height_pt: measured_repeating_blocks_height(
        &section.even_header_blocks,
        section.page,
        document.default_tab_stop_pt,
      ),
      even_footer_height_pt: measured_repeating_blocks_height(
        &section.even_footer_blocks,
        section.page,
        document.default_tab_stop_pt,
      ),
    };
  }

  RepeatingSlotState {
    title_page: document.title_page,
    even_and_odd_headers: document.even_and_odd_headers,
    default_header: !document.header_blocks.is_empty(),
    default_footer: !document.footer_blocks.is_empty(),
    first_header: !document.first_header_blocks.is_empty(),
    first_footer: !document.first_footer_blocks.is_empty(),
    even_header: !document.header_blocks.is_empty(),
    even_footer: !document.footer_blocks.is_empty(),
    default_header_height_pt: measured_repeating_blocks_height(
      &document.header_blocks,
      document.page,
      document.default_tab_stop_pt,
    ),
    default_footer_height_pt: measured_repeating_blocks_height(
      &document.footer_blocks,
      document.page,
      document.default_tab_stop_pt,
    ),
    first_header_height_pt: measured_repeating_blocks_height(
      &document.first_header_blocks,
      document.page,
      document.default_tab_stop_pt,
    ),
    first_footer_height_pt: measured_repeating_blocks_height(
      &document.first_footer_blocks,
      document.page,
      document.default_tab_stop_pt,
    ),
    even_header_height_pt: measured_repeating_blocks_height(
      &document.header_blocks,
      document.page,
      document.default_tab_stop_pt,
    ),
    even_footer_height_pt: measured_repeating_blocks_height(
      &document.footer_blocks,
      document.page,
      document.default_tab_stop_pt,
    ),
  }
}

fn repeating_slots_present_for_page(
  slots: RepeatingSlotState,
  page_number: usize,
  section_page_index: usize,
) -> (bool, bool, f32, f32) {
  let first_page_in_section = section_page_index == 0;
  let use_even_slot = slots.even_and_odd_headers && page_number.is_multiple_of(2);
  let (header, header_height) = selected_repeating_slot(
    first_page_in_section,
    use_even_slot,
    slots.title_page,
    (slots.first_header, slots.first_header_height_pt),
    (slots.even_header, slots.even_header_height_pt),
    (slots.default_header, slots.default_header_height_pt),
  );
  let (footer, footer_height) = selected_repeating_slot(
    first_page_in_section,
    use_even_slot,
    slots.title_page,
    (slots.first_footer, slots.first_footer_height_pt),
    (slots.even_footer, slots.even_footer_height_pt),
    (slots.default_footer, slots.default_footer_height_pt),
  );
  (header, footer, header_height, footer_height)
}

fn selected_repeating_slot(
  first_page_in_section: bool,
  use_even_slot: bool,
  title_page: bool,
  first: (bool, f32),
  even: (bool, f32),
  default_: (bool, f32),
) -> (bool, f32) {
  if first_page_in_section && title_page && first.0 {
    return first;
  }
  if use_even_slot && even.0 {
    return even;
  }
  if default_.0 {
    return default_;
  }
  (false, 0.0)
}

fn measured_repeating_blocks_height(
  blocks: &[Block],
  setup: PageSetup,
  default_tab_stop_pt: f32,
) -> f32 {
  if blocks.is_empty() {
    return 0.0;
  }

  let mut scratch = empty_section_page(
    PageSetup {
      margin_top_pt: 0.0,
      margin_bottom_pt: 0.0,
      ..setup
    },
    0,
    0,
  );
  let mut discarded_pages = Vec::new();
  let content_width =
    (setup.width_pt - setup.margin_left_pt - setup.margin_right_pt).max(DEFAULT_FONT_SIZE_PT);
  let flow = FlowContext {
    setup: scratch.setup,
    section_index: 0,
    section_page_index: 0,
    column_index: 0,
    columns: SectionColumns::default(),
    content_top_pt: 0.0,
    content_left_pt: setup.margin_left_pt,
    content_bottom: MEASURE_SCRATCH_PAGE_HEIGHT_PT,
    body_content_bottom_pt: MEASURE_SCRATCH_PAGE_HEIGHT_PT,
    content_width,
    layout_cell_bounds: None,
    default_tab_stop_pt,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::RepeatingSlot,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
  };
  let mut y = 0.0;
  for (index, block) in blocks.iter().enumerate() {
    y = layout_repeating_block(
      block,
      &mut scratch,
      &mut discarded_pages,
      y,
      flow,
      index + 1 == blocks.len(),
    );
  }
  y
}

fn layout_repeating_block(
  block: &Block,
  page: &mut Page,
  discarded_pages: &mut Vec<Page>,
  y: f32,
  flow: FlowContext,
  is_last_repeating_block: bool,
) -> f32 {
  match block {
    Block::Paragraph(paragraph) => {
      let spacing_after_pt = if is_last_repeating_block {
        // Source: LibreOffice sw/source/core/layout/flowfrm.cxx
        // SwFlowFrame::CalcLowerSpace(), tdf#128195 branch:
        // HeaderSpacingBelowLastPara adds the last header/footer paragraph's
        // lower paragraph space plus SwBorderAttrs::CalcLineSpacing().
        paragraph
          .format
          .spacing_after_pt
          .max(PARAGRAPH_SPACING_AFTER_PT)
          + table_cell_line_spacing_before_border(paragraph)
      } else {
        paragraph
          .format
          .spacing_after_pt
          .max(PARAGRAPH_SPACING_AFTER_PT)
      };
      let (_, y) = layout_paragraph(
        paragraph,
        flow,
        page,
        discarded_pages,
        y + paragraph.format.spacing_before_pt,
        spacing_after_pt,
      );
      y
    }
    Block::Table(table) => {
      let (_, y) = layout_table(table, flow, page, discarded_pages, y);
      y
    }
    Block::Frame(frame) => {
      let (_, y) = layout_floating_frame(frame, flow, page, discarded_pages, y);
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
          line_height_pt: DEFAULT_LINE_HEIGHT_PT,
          text: line,
          style: TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          preserve_text_portion: false,
          form_widget_id: None,
          paragraph_bidi: false,
          decoration_span_start_x_pt: None,
          pdf_text_segmentation: PdfTextSegmentation::Line,
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
    form_widgets: Vec::new(),
    follows: Vec::new(),
    frames: Vec::new(),
    outline_entries: Vec::new(),
    page_replays: Vec::new(),
    page_replay_applications: Vec::new(),
    backward_moves: Vec::new(),
    layout_reruns: Vec::new(),
    page_invalidations: Vec::new(),
    reflow_executions: Vec::new(),
    reflow_requests: Vec::new(),
    restart_plan: None,
  }
}

fn layout_table(
  table: &Table,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  y: f32,
) -> (FlowContext, f32) {
  if table.placement.is_some()
    && table.following_text_flow
    && matches!(
      flow.text_segmentation,
      TextSegmentation::RepeatingSlot | TextSegmentation::Notes
    )
  {
    let mut inline_table = table.clone();
    inline_table.placement = None;
    return TableFrameLayout::new(&inline_table, block_area(flow), false)
      .map_or((flow, y), |layout| layout.format(current, pages, y));
  }
  if table.placement.is_some() {
    return layout_floating_table(table, flow, current, pages, y);
  }
  TableFrameLayout::new(table, block_area(flow), false)
    .map_or((flow, y), |layout| layout.format(current, pages, y))
}

fn layout_floating_table(
  table: &Table,
  mut flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
) -> (FlowContext, f32) {
  let Some(placement) = table.placement else {
    return (flow, y);
  };
  if table.starts_after_last_rendered_page_break && y > flow.content_top_pt + LAYOUT_EPSILON_PT {
    current.preserve_empty = true;
    (flow, y) = advance_section_flow(flow, current, pages);
  }
  let mut effective_table = table.clone();
  effective_table.split_allowed = effective_floating_table_split_allowed(table, flow);
  let Some(layout) = TableFrameLayout::new(&effective_table, block_area(flow), true) else {
    return (flow, y);
  };
  let table_width = (layout.frame.right_pt - layout.frame.left_pt).max(DEFAULT_FONT_SIZE_PT);
  let (x, frame_y) = floating_frame_position(placement, flow, y, table_width, 0.0);
  let frame_flow = FlowContext {
    content_top_pt: frame_y,
    content_left_pt: x,
    content_bottom: flow.content_bottom,
    body_content_bottom_pt: flow.body_content_bottom_pt,
    content_width: table_width,
    ..flow
  };
  let mut frame_page =
    empty_section_page(flow.setup, current.section_index, flow.section_page_index);
  let mut frame_pages = Vec::new();
  let (_, bottom_y) = TableFrameLayout::new(&effective_table, block_area(frame_flow), true)
    .map_or((frame_flow, frame_y), |layout| {
      layout.format(&mut frame_page, &mut frame_pages, frame_y)
    });
  frame_pages.push(frame_page);
  let total_frame_pages = frame_pages.len();
  for (page_index, page) in frame_pages.iter_mut().enumerate() {
    decorate_floating_table_page_bounds(
      page,
      x,
      table_width,
      if page_index == 0 {
        frame_y
      } else {
        page_items_vertical_bounds(&page.items).map_or(flow.content_top_pt, |(top, _)| top)
      },
      if page_index + 1 == total_frame_pages {
        Some(bottom_y)
      } else {
        None
      },
    );
  }
  let first_page_bottom = frame_pages
    .first()
    .and_then(|page| page_items_vertical_bounds(&page.items).map(|(_, bottom)| bottom));
  if let Some(first_page) = frame_pages.first_mut() {
    append_floating_table_wrap_exclusion(first_page, placement);
    let item_offset = current.items.len();
    offset_page_frame_records(first_page, item_offset);
    current.items.append(&mut first_page.items);
    current
      .frame_fragments
      .append(&mut first_page.frame_fragments);
    current
      .frame_influences
      .append(&mut first_page.frame_influences);
    current
      .wrap_exclusions
      .append(&mut first_page.wrap_exclusions);
  }
  for mut follow_page in frame_pages.into_iter().skip(1) {
    if follow_page.items.is_empty() {
      continue;
    }
    append_floating_table_wrap_exclusion(&mut follow_page, placement);
    current
      .pending_floating_table_follows
      .push(PendingFloatingTableFollow {
        setup: follow_page.setup,
        section_index: follow_page.section_index,
        section_page_index: follow_page.section_page_index,
        items: follow_page.items,
        frame_fragments: follow_page.frame_fragments,
        frame_influences: follow_page.frame_influences,
        wrap_exclusions: follow_page.wrap_exclusions,
      });
  }
  let occupied_bottom = first_page_bottom.unwrap_or(bottom_y) + placement.margin_bottom_pt;
  if frame_wrap_blocks_flow(placement.wrap) {
    (flow, y.max(occupied_bottom))
  } else {
    (flow, y)
  }
}

fn effective_floating_table_split_allowed(table: &Table, flow: FlowContext) -> bool {
  if !table.split_allowed {
    return false;
  }
  // Source: LibreOffice sw/source/core/layout/fly.cxx disables split flys in
  // multi-column sections and for bottom-growing body-relative flys.
  if flow.columns.count > 1 {
    return false;
  }
  if matches!(
    flow.text_segmentation,
    TextSegmentation::RepeatingSlot | TextSegmentation::Notes
  ) {
    return false;
  }
  let Some(placement) = table.placement else {
    return table.split_allowed;
  };
  if matches!(
    placement.vertical_alignment,
    Some(FrameVerticalAlignment::Bottom) | Some(FrameVerticalAlignment::Outside)
  ) && matches!(placement.vertical_anchor, FrameVerticalAnchor::Margin)
  {
    return false;
  }
  true
}

fn decorate_floating_table_page_bounds(
  page: &mut Page,
  x_pt: f32,
  width_pt: f32,
  default_top_pt: f32,
  default_bottom_pt: Option<f32>,
) {
  let (top_pt, bottom_pt) = page_items_vertical_bounds(&page.items).unwrap_or_else(|| {
    let bottom = default_bottom_pt.unwrap_or(default_top_pt + DEFAULT_LINE_HEIGHT_PT);
    (default_top_pt, bottom)
  });
  page.items.insert(
    0,
    PageItem::Rect(RectItem {
      x_pt,
      y_pt: top_pt,
      width_pt,
      height_pt: (default_bottom_pt.unwrap_or(bottom_pt) - top_pt).max(DEFAULT_LINE_HEIGHT_PT),
      fill_color: None,
      fill_opacity: 1.0,
      stroke: Some(BorderStyle::default()),
      stroke_opacity: 0.0,
    }),
  );
}

fn append_floating_table_wrap_exclusion(page: &mut Page, placement: FloatingFramePlacement) {
  let Some((left_pt, top_pt, right_pt, bottom_pt)) = page_items_bounds(&page.items) else {
    return;
  };
  let blocks_flow = frame_wrap_blocks_flow(placement.wrap);
  let exclusion = WrapExclusion {
    left_pt: if blocks_flow {
      0.0
    } else {
      left_pt - placement.margin_left_pt
    },
    right_pt: if blocks_flow {
      page.setup.width_pt
    } else {
      right_pt + placement.margin_right_pt
    },
    top_pt: top_pt - placement.margin_top_pt,
    bottom_pt: bottom_pt + placement.margin_bottom_pt,
    side: ImageWrapSide::BothSides,
    blocks_flow,
  };
  page.wrap_exclusions.push(exclusion);
}

fn offset_page_frame_records(page: &mut Page, item_offset: usize) {
  if item_offset == 0 {
    return;
  }
  offset_page_frame_records_raw(
    &mut page.frame_fragments,
    &mut page.frame_influences,
    item_offset,
  );
}

fn offset_page_frame_records_raw(
  frame_fragments: &mut [FrameFragment],
  frame_influences: &mut [FrameInfluence],
  item_offset: usize,
) {
  if item_offset == 0 {
    return;
  }
  for fragment in frame_fragments {
    fragment.item_start += item_offset;
    fragment.item_end += item_offset;
  }
  for influence in frame_influences {
    influence.item_start += item_offset;
    influence.item_end += item_offset;
  }
}

fn translate_page_item(mut item: PageItem, dx_pt: f32, dy_pt: f32) -> PageItem {
  match &mut item {
    PageItem::Text(text) => {
      text.x_pt += dx_pt;
      text.y_pt += dy_pt;
      if let Some(span_start) = &mut text.decoration_span_start_x_pt {
        *span_start += dx_pt;
      }
    }
    PageItem::Image(image) => {
      image.x_pt += dx_pt;
      image.y_pt += dy_pt;
    }
    PageItem::Rect(rect) => {
      rect.x_pt += dx_pt;
      rect.y_pt += dy_pt;
    }
    PageItem::Fill(fill) => {
      fill.x_pt += dx_pt;
      fill.y_pt += dy_pt;
    }
    PageItem::Line(line) => {
      line.x1_pt += dx_pt;
      line.y1_pt += dy_pt;
      line.x2_pt += dx_pt;
      line.y2_pt += dy_pt;
    }
    PageItem::Polyline(polyline) => {
      polyline.x_pt += dx_pt;
      polyline.y_pt += dy_pt;
      for point in &mut polyline.points {
        point.0 += dx_pt;
        point.1 += dy_pt;
      }
    }
  }
  item
}

fn page_content_vertical_bounds(setup: PageSetup) -> (f32, f32) {
  (
    setup.margin_top_pt,
    setup.height_pt - setup.margin_bottom_pt,
  )
}

fn flatten_nested_pages(
  first_page: Page,
  discarded_pages: Vec<Page>,
  first_top_pt: f32,
  first_bottom_pt: f32,
) -> Vec<PageItem> {
  let mut pages = discarded_pages;
  pages.push(first_page);
  let mut items = Vec::new();
  let mut virtual_top_pt = first_top_pt;

  for (page_index, page) in pages.into_iter().enumerate() {
    let (page_top_pt, page_bottom_pt) = if page_index == 0 {
      (first_top_pt, first_bottom_pt)
    } else {
      page_content_vertical_bounds(page.setup)
    };
    let dy_pt = virtual_top_pt - page_top_pt;
    items.extend(
      page
        .items
        .into_iter()
        .map(|item| translate_page_item(item, 0.0, dy_pt)),
    );
    virtual_top_pt += (page_bottom_pt - page_top_pt).max(0.0);
  }

  items
}

fn ordered_local_pages(first_page: Page, mut discarded_pages: Vec<Page>) -> Vec<Page> {
  discarded_pages.push(first_page);
  discarded_pages
}

#[derive(Clone, Debug)]
struct TableFrameLayout<'a> {
  table: &'a Table,
  frame: TableFrame,
}

#[derive(Clone, Copy, Debug)]
struct PendingBorderSegment {
  x_pt: f32,
  start_y_pt: f32,
  end_y_pt: f32,
  border: BorderStyle,
}

impl<'a> TableFrameLayout<'a> {
  fn new(table: &'a Table, area: BlockArea, allow_width_overflow: bool) -> Option<Self> {
    let column_count = table_column_count(table);
    if column_count == 0 {
      return None;
    }

    let max_cell_spacing_pt = table_max_cell_spacing_pt(table);
    let available_width = (area.content_width
      - max_cell_spacing_pt * column_count.saturating_sub(1) as f32)
      .max(DEFAULT_FONT_SIZE_PT);
    let column_widths =
      table_column_widths(table, column_count, available_width, allow_width_overflow);
    let table_width = column_widths.iter().sum::<f32>()
      + max_cell_spacing_pt * column_count.saturating_sub(1) as f32;
    let left_pt = table_left_position(table, area.content_left_pt, area.content_width, table_width);
    let full_width_horizontal_borders =
      allow_width_overflow || table.preferred_width_pct.is_some_and(|pct| pct >= 0.999);
    let repeating_header_count = table_repeating_header_count(table);
    let coalesce_row_shading = table.preferred_width_pct.is_some_and(|pct| pct >= 0.999);
    let split_allowed = table_split_allowed(table);
    let row_heights = table_row_heights_with_widths(table, &column_widths);
    let repeating_header_height =
      table_repeating_header_height_from_row_heights(table, repeating_header_count, &row_heights);
    let total_height = table_total_height_from_row_heights(table, &row_heights);

    Some(Self {
      table,
      frame: TableFrame {
        block: area,
        column_widths,
        left_pt,
        right_pt: left_pt + table_width,
        full_width_horizontal_borders,
        coalesce_row_shading,
        split_allowed,
        row_heights,
        repeating_header_count,
        repeating_header_height,
        total_height,
      },
    })
  }

  fn format(&self, current: &mut Page, pages: &mut Vec<Page>, mut y: f32) -> (FlowContext, f32) {
    let mut flow = flow_from_block_area(self.frame.block);
    let mut layout = self.clone();
    y = layout.dodge_wrap_exclusions(current, y, layout.frame.total_height);
    if !layout.frame.split_allowed
      && y > layout.frame.block.content_top_pt + LAYOUT_EPSILON_PT
      && y + layout.frame.total_height > layout.frame.block.content_bottom + LAYOUT_EPSILON_PT
    {
      (flow, y) = advance_section_flow(flow, current, pages);
      if let Some(next_layout) = TableFrameLayout::new(self.table, block_area(flow), false) {
        layout = next_layout;
      }
    }
    let table_row_keep = layout.table_row_keep_enabled();
    let allow_split_of_keep_row = table_row_keep
      && layout.are_all_body_rows_keep_with_next()
      && y <= layout.frame.block.content_top_pt + LAYOUT_EPSILON_PT;
    let mut rows_moved_to_follow = HashSet::new();
    let mut left_border_segment = None;
    let mut right_border_segment = None;
    let mut row_index = 0usize;
    while row_index < self.table.rows.len() {
      let problem_row_index =
        layout.first_problem_row_index(row_index, y, table_row_keep, allow_split_of_keep_row);
      let format_until = problem_row_index.unwrap_or(self.table.rows.len());
      while row_index < format_until {
        let row = &self.table.rows[row_index];
        y = layout.dodge_wrap_exclusions(
          current,
          y,
          layout
            .frame
            .row_heights
            .get(row_index)
            .copied()
            .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT),
        );
        if y > layout.frame.block.content_bottom + LAYOUT_EPSILON_PT {
          flush_border_segment(current, &mut left_border_segment);
          flush_border_segment(current, &mut right_border_segment);
          (flow, y) = advance_section_flow(flow, current, pages);
          if let Some(next_layout) = TableFrameLayout::new(self.table, block_area(flow), false) {
            layout = next_layout;
          }
          y = layout.format_repeated_header_rows(current, pages, y, 0.0);
        }
        let row_frame = layout.row_frame(row, row_index, y);
        let row_top = y;
        y = row_frame.format(current, pages);
        extend_border_segment(
          current,
          &mut left_border_segment,
          row_frame.leading_border_segment(row_top, y),
        );
        extend_border_segment(
          current,
          &mut right_border_segment,
          row_frame.trailing_border_segment(row_top, y),
        );
        row_index += 1;
        if row_index < self.table.rows.len() {
          y += row_cell_spacing_pt(self.table, row);
        }
      }
      if row_index >= self.table.rows.len() {
        break;
      }

      let row = &self.table.rows[row_index];
      y = layout.dodge_wrap_exclusions(
        current,
        y,
        layout
          .frame
          .row_heights
          .get(row_index)
          .copied()
          .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT),
      );
      if y > layout.frame.block.content_bottom + LAYOUT_EPSILON_PT {
        flush_border_segment(current, &mut left_border_segment);
        flush_border_segment(current, &mut right_border_segment);
        (flow, y) = advance_section_flow(flow, current, pages);
        if let Some(next_layout) = TableFrameLayout::new(self.table, block_area(flow), false) {
          layout = next_layout;
        }
        y = layout.format_repeated_header_rows(current, pages, y, 0.0);
      }
      let mut row_frame = layout.row_frame(row, row_index, y);
      let row_height = row_frame.height_pt;
      if layout.should_split_row(&row_frame, table_row_keep, allow_split_of_keep_row) {
        let mut remaining_height = row_height;
        let mut content_offset = 0.0;
        while remaining_height > LAYOUT_EPSILON_PT {
          let available_height = (layout.frame.block.content_bottom - y).max(0.0);
          if available_height <= LAYOUT_EPSILON_PT {
            flush_border_segment(current, &mut left_border_segment);
            flush_border_segment(current, &mut right_border_segment);
            (flow, y) = advance_section_flow(flow, current, pages);
            if let Some(next_layout) = TableFrameLayout::new(self.table, block_area(flow), false) {
              layout = next_layout;
            }
            y = layout.format_repeated_header_rows(current, pages, y, remaining_height);
            continue;
          }

          let min_split_height = layout.minimum_split_fragment_height(row_index, row);
          if remaining_height > available_height
            && min_split_height > 0.0
            && available_height + LAYOUT_EPSILON_PT < min_split_height
          {
            flush_border_segment(current, &mut left_border_segment);
            flush_border_segment(current, &mut right_border_segment);
            (flow, y) = advance_section_flow(flow, current, pages);
            if let Some(next_layout) = TableFrameLayout::new(self.table, block_area(flow), false) {
              layout = next_layout;
            }
            y = layout.format_repeated_header_rows(current, pages, y, remaining_height);
            continue;
          }

          let fragment_height = remaining_height.min(available_height);
          row_frame = layout.row_frame(row, row_index, y);
          row_frame.format_fragment(current, pages, y, y + fragment_height, content_offset);
          extend_border_segment(
            current,
            &mut left_border_segment,
            row_frame.leading_border_segment(y, y + fragment_height),
          );
          extend_border_segment(
            current,
            &mut right_border_segment,
            row_frame.trailing_border_segment(y, y + fragment_height),
          );
          y += fragment_height;
          remaining_height -= fragment_height;
          content_offset += fragment_height;
          if remaining_height > LAYOUT_EPSILON_PT {
            flush_border_segment(current, &mut left_border_segment);
            flush_border_segment(current, &mut right_border_segment);
            (flow, y) = advance_section_flow(flow, current, pages);
            if let Some(next_layout) = TableFrameLayout::new(self.table, block_area(flow), false) {
              layout = next_layout;
            }
            y = layout.format_repeated_header_rows(current, pages, y, remaining_height);
          }
        }
        row_index += 1;
        if row_index < self.table.rows.len() {
          y += row_cell_spacing_pt(self.table, row);
        }
        continue;
      }

      let already_moved = rows_moved_to_follow.contains(&row_index);
      if !layout.should_move_row_to_follow(&row_frame, already_moved) {
        let row_top = y;
        y = row_frame.format(current, pages);
        extend_border_segment(
          current,
          &mut left_border_segment,
          row_frame.leading_border_segment(row_top, y),
        );
        extend_border_segment(
          current,
          &mut right_border_segment,
          row_frame.trailing_border_segment(row_top, y),
        );
        row_index += 1;
        if row_index < self.table.rows.len() {
          y += row_cell_spacing_pt(self.table, row);
        }
        continue;
      }

      flush_border_segment(current, &mut left_border_segment);
      flush_border_segment(current, &mut right_border_segment);
      rows_moved_to_follow.insert(row_index);
      (flow, y) = advance_section_flow(flow, current, pages);
      if let Some(next_layout) = TableFrameLayout::new(self.table, block_area(flow), false) {
        layout = next_layout;
      }
      y = layout.format_repeated_header_rows(
        current,
        pages,
        y,
        layout.follow_page_required_height(row_index, y),
      );
    }

    flush_border_segment(current, &mut left_border_segment);
    flush_border_segment(current, &mut right_border_segment);

    (flow, y + TABLE_SPACING_AFTER_PT)
  }

  fn first_problem_row_index(
    &self,
    start_row_index: usize,
    y: f32,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> Option<usize> {
    if let Some(problem_row_index) =
      self.page_start_problem_row_index(start_row_index, y, table_row_keep, allow_split_of_keep_row)
    {
      return Some(self.adjust_problem_row_index_for_keep_with_next(
        problem_row_index,
        start_row_index,
        table_row_keep,
        allow_split_of_keep_row,
      ));
    }
    let mut current_y = y;
    for row_index in start_row_index..self.table.rows.len() {
      if self.should_move_keep_with_next_chain_to_follow(
        row_index,
        current_y,
        table_row_keep,
        allow_split_of_keep_row,
      ) {
        return Some(self.adjust_problem_row_index_for_keep_with_next(
          row_index,
          start_row_index,
          table_row_keep,
          allow_split_of_keep_row,
        ));
      }
      let row = &self.table.rows[row_index];
      let row_frame = self.row_frame(row, row_index, current_y);
      if row_frame.bottom() > self.frame.block.content_bottom + LAYOUT_EPSILON_PT {
        return Some(self.adjust_problem_row_index_for_keep_with_next(
          row_index,
          start_row_index,
          table_row_keep,
          allow_split_of_keep_row,
        ));
      }
      current_y = row_frame.bottom();
      if row_index + 1 < self.table.rows.len() {
        current_y += row_cell_spacing_pt(self.table, row);
      }
    }
    None
  }

  fn adjust_problem_row_index_for_keep_with_next(
    &self,
    mut row_index: usize,
    start_row_index: usize,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> usize {
    if !table_row_keep || allow_split_of_keep_row {
      return row_index;
    }
    let floor = start_row_index.max(self.frame.repeating_header_count);
    while row_index > floor && self.table.rows[row_index - 1].keep_with_next {
      row_index -= 1;
    }
    row_index
  }

  fn page_start_problem_row_index(
    &self,
    start_row_index: usize,
    y: f32,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> Option<usize> {
    if !table_row_keep || allow_split_of_keep_row {
      return None;
    }
    if y > self.frame.block.content_top_pt + LAYOUT_EPSILON_PT {
      return None;
    }
    let first_body_row_index = start_row_index.max(self.frame.repeating_header_count);
    let Some(first_body_row) = self.table.rows.get(first_body_row_index) else {
      return None;
    };
    let required_bottom = self.keep_with_next_chain_bottom(first_body_row_index, y);
    if required_bottom <= self.frame.block.content_bottom + LAYOUT_EPSILON_PT {
      return None;
    }
    let first_body_row_frame = self.row_frame(first_body_row, first_body_row_index, y);
    if self.should_split_row(
      &first_body_row_frame,
      table_row_keep,
      allow_split_of_keep_row,
    ) {
      return None;
    }
    if self.frame.repeating_header_count > 0
      && start_row_index <= self.frame.repeating_header_count
      && first_body_row_index + 1 < self.table.rows.len()
    {
      return Some(first_body_row_index + 1);
    }
    None
  }

  fn row_frame(&self, row: &'a TableRow, row_index: usize, y: f32) -> RowFrame<'a, '_> {
    RowFrame {
      table: self.table,
      table_frame: &self.frame,
      row,
      row_index,
      y,
      height_pt: self
        .frame
        .row_heights
        .get(row_index)
        .copied()
        .unwrap_or_else(|| {
          table_row_height_with_widths(self.table, row_index, row, &self.frame.column_widths)
        }),
    }
  }

  fn should_move_keep_with_next_chain_to_follow(
    &self,
    row_index: usize,
    y: f32,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> bool {
    if row_index + 1 >= self.table.rows.len()
      || !table_row_keep
      || allow_split_of_keep_row
      || !self.table.rows[row_index].keep_with_next
      || y <= self.frame.block.content_top_pt + LAYOUT_EPSILON_PT
    {
      return false;
    }
    self.keep_with_next_chain_bottom(row_index, y)
      > self.frame.block.content_bottom + LAYOUT_EPSILON_PT
  }

  fn keep_with_next_chain_bottom(&self, row_index: usize, y: f32) -> f32 {
    let mut current_y = y;
    let mut bottom = y;
    let mut current_row_index = row_index;
    while let Some(row) = self.table.rows.get(current_row_index) {
      let row_frame = self.row_frame(row, current_row_index, current_y);
      bottom = row_frame.bottom();
      if !row.keep_with_next || current_row_index + 1 >= self.table.rows.len() {
        break;
      }
      current_y = bottom + row_cell_spacing_pt(self.table, row);
      current_row_index += 1;
    }
    bottom
  }

  fn follow_page_required_height(&self, row_index: usize, y: f32) -> f32 {
    (self.keep_with_next_chain_bottom(row_index, y) - y).max(0.0)
  }

  fn should_move_row_to_follow(&self, row: &RowFrame<'_, '_>, already_moved: bool) -> bool {
    if already_moved || row.bottom() <= self.frame.block.content_bottom {
      return false;
    }
    if self.is_first_non_header_row(row.row_index) {
      return false;
    }

    !row.row.cant_split
      || row.fits_empty_body_region()
      || row.y > self.frame.block.content_top_pt + LAYOUT_EPSILON_PT
  }

  fn is_first_non_header_row(&self, row_index: usize) -> bool {
    row_index == self.frame.repeating_header_count
  }

  fn should_split_row(
    &self,
    row: &RowFrame<'_, '_>,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> bool {
    let row_split_allowed = !row.row.cant_split || !row.fits_empty_body_region();
    row_split_allowed
      && !row.row.exact_height
      && !row_repeat_header_effective(self.table, row.row_index)
      && !row_has_vertical_merge_context(self.table, row.row_index)
      && (!table_row_keep || !row.row.keep_with_next || allow_split_of_keep_row)
      && row.bottom() > self.frame.block.content_bottom
      && row.y < self.frame.block.content_bottom
  }

  fn table_row_keep_enabled(&self) -> bool {
    self.table.split_allowed && self.table.placement.is_none()
  }

  fn are_all_body_rows_keep_with_next(&self) -> bool {
    let mut row_index = self.frame.repeating_header_count;
    let Some(row) = self.table.rows.get(row_index) else {
      return false;
    };
    if !row.keep_with_next {
      return false;
    }
    row_index += 1;
    while let Some(row) = self.table.rows.get(row_index) {
      if !row.keep_with_next {
        return false;
      }
      row_index += 1;
    }
    true
  }

  fn format_repeated_header_rows(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    mut y: f32,
    row_height: f32,
  ) -> f32 {
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
      y = self
        .row_frame(header, header_index, y)
        .format(current, pages);
      if header_index + 1 < self.frame.repeating_header_count || row_height > 0.0 {
        y += row_cell_spacing_pt(self.table, header);
      }
    }
    y
  }

  fn minimum_split_fragment_height(&self, row_index: usize, row: &TableRow) -> f32 {
    if (self.table.placement.is_none() && !self.table.following_text_flow)
      || !self.table.split_allowed
      || row.exact_height
    {
      return 0.0;
    }
    row.height_pt.map_or(0.0, |height| {
      height
        + row_top_border_space_extent(self.table, row_index, row)
        + row_bottom_border_spacing_extent(self.table, row_index, row)
    })
  }

  fn dodge_wrap_exclusions(&self, current: &Page, mut y: f32, height_pt: f32) -> f32 {
    let height_pt = height_pt.max(TABLE_ROW_MIN_HEIGHT_PT);
    loop {
      let Some(exclusion) = self.blocking_wrap_exclusion(current, y, y + height_pt) else {
        return y;
      };
      let next_y = exclusion.bottom_pt;
      if next_y <= y + LAYOUT_EPSILON_PT {
        return y;
      }
      y = next_y;
    }
  }

  fn blocking_wrap_exclusion(
    &self,
    current: &Page,
    top_pt: f32,
    bottom_pt: f32,
  ) -> Option<WrapExclusion> {
    current
      .wrap_exclusions
      .iter()
      .copied()
      .filter(|exclusion| {
        exclusion.overlaps_vertical_span(top_pt, bottom_pt)
          && exclusion.overlaps_horizontal_span(self.frame.left_pt, self.frame.right_pt)
          && self.table_needs_vertical_dodge(*exclusion)
      })
      .min_by(|a, b| {
        a.bottom_pt
          .partial_cmp(&b.bottom_pt)
          .unwrap_or(std::cmp::Ordering::Equal)
      })
  }

  fn table_needs_vertical_dodge(&self, exclusion: WrapExclusion) -> bool {
    if exclusion.blocks_flow {
      return true;
    }
    let table_width = self.frame.right_pt - self.frame.left_pt;
    let block_left = self.frame.block.content_left_pt;
    let block_right = block_left + self.frame.block.content_width;
    let left_space = (exclusion.left_pt - block_left).max(0.0);
    let right_space = (block_right - exclusion.right_pt).max(0.0);

    // Source: LibreOffice sw/source/core/layout/tabfrm.cxx
    // SwTabFrame::CalcFlyOffsets() shifts the table down for no-wrap flys and
    // only keeps side wrapping when the table print area can fit beside the fly.
    match exclusion.side {
      ImageWrapSide::Left => left_space + LAYOUT_EPSILON_PT < table_width,
      ImageWrapSide::Right => right_space + LAYOUT_EPSILON_PT < table_width,
      ImageWrapSide::BothSides | ImageWrapSide::Largest => {
        left_space.max(right_space) + LAYOUT_EPSILON_PT < table_width
      }
    }
  }
}

fn extend_border_segment(
  page: &mut Page,
  pending: &mut Option<PendingBorderSegment>,
  next: Option<PendingBorderSegment>,
) {
  let Some(next) = next else {
    return;
  };
  match pending {
    Some(current)
      if f32::abs(current.x_pt - next.x_pt) < 0.01
        && current.border == next.border
        && f32::abs(current.end_y_pt - next.start_y_pt) < LAYOUT_EPSILON_PT =>
    {
      current.end_y_pt = next.end_y_pt;
    }
    Some(_) => {
      flush_border_segment(page, pending);
      *pending = Some(next);
    }
    None => {
      *pending = Some(next);
    }
  }
}

fn flush_border_segment(page: &mut Page, pending: &mut Option<PendingBorderSegment>) {
  let Some(segment) = pending.take() else {
    return;
  };
  push_styled_line(
    page,
    segment.x_pt,
    segment.start_y_pt,
    segment.x_pt,
    segment.end_y_pt,
    segment.border,
  );
}

#[derive(Clone, Debug)]
struct TableFrame {
  block: BlockArea,
  column_widths: Vec<f32>,
  left_pt: f32,
  right_pt: f32,
  full_width_horizontal_borders: bool,
  coalesce_row_shading: bool,
  split_allowed: bool,
  row_heights: Vec<f32>,
  repeating_header_count: usize,
  repeating_header_height: f32,
  total_height: f32,
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
    self.height_pt <= self.table_frame.block.content_bottom - self.table_frame.block.content_top_pt
  }

  fn format(&self, current: &mut Page, pages: &mut Vec<Page>) -> f32 {
    let row_top = self.y;
    let row_bottom = self.bottom();
    self.format_fragment(current, pages, row_top, row_bottom, 0.0);
    row_bottom
  }

  fn format_fragment(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    row_top: f32,
    row_bottom: f32,
    content_offset: f32,
  ) {
    let row_item_start = current.items.len();
    let split = self.fragment_split(row_bottom, content_offset);
    let cell_spacing_pt = self.cell_spacing_pt();
    let mut cell_left = row_grid_left(self.table_frame, self.row, cell_spacing_pt);
    if let Some(color) = self.row.redline_color {
      current.items.push(PageItem::Fill(FillItem {
        x_pt: cell_left,
        y_pt: row_top,
        width_pt: self.table_frame.right_pt - cell_left,
        height_pt: row_bottom - row_top,
        color,
      }));
    }
    if self.table_frame.coalesce_row_shading
      && let Some(color) = self.coalesced_row_shading()
    {
      current.items.push(PageItem::Fill(FillItem {
        x_pt: cell_left,
        y_pt: row_top,
        width_pt: self.table_frame.right_pt - cell_left,
        height_pt: row_bottom - row_top,
        color,
      }));
    }
    let mut grid_index = self.row.grid_before;
    for (cell_index, cell) in self.row.cells.iter().enumerate() {
      let cell_item_start = current.items.len();
      let grid_start = grid_index;
      let cell_frame = match self.cell_frame(
        cell,
        cell_index,
        cell_left,
        row_bottom - row_top,
        &mut grid_index,
      ) {
        Some(cell_frame) => cell_frame,
        None => break,
      };
      if cell.vertical_merge_continue {
        let origin = self.vertical_merge_origin_cell(grid_start);
        cell_frame.format_merged_continue(current, row_top, row_bottom, origin);
      } else {
        cell_frame.format(current, pages, row_top, row_bottom, content_offset);
      }
      push_page_fragment(
        current,
        PageFragmentRecord {
          kind: FrameFragmentKind::TableCell,
          split,
          index: self.row_index,
          row_index: self.row_index,
          cell_index: Some(cell_index),
          item_start: cell_item_start,
          item_end: current.items.len(),
        },
      );
      cell_left += cell_frame.width_pt + cell_spacing_pt;
    }

    self.paint_horizontal_borders(current, row_top, row_bottom);
    push_page_fragment(
      current,
      PageFragmentRecord {
        kind: FrameFragmentKind::TableRow,
        split,
        index: self.row_index,
        row_index: self.row_index,
        cell_index: None,
        item_start: row_item_start,
        item_end: current.items.len(),
      },
    );
  }

  fn fragment_split(&self, row_bottom: f32, content_offset: f32) -> FragmentSplitKind {
    if row_repeat_header_effective(self.table, self.row_index) {
      return FragmentSplitKind::RepeatedHeader;
    }
    if content_offset > LAYOUT_EPSILON_PT {
      return FragmentSplitKind::Follow;
    }
    if row_bottom + LAYOUT_EPSILON_PT < self.bottom() {
      return FragmentSplitKind::Master;
    }
    FragmentSplitKind::Complete
  }

  fn cell_frame<'s>(
    &'s self,
    cell: &'s TableCell,
    cell_index: usize,
    left_pt: f32,
    height_pt: f32,
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
      row_index: self.row_index,
      grid_start: *grid_index - span,
      cell_index,
      left_pt,
      width_pt,
      height_pt,
    })
  }

  fn vertical_merge_origin_cell(&self, grid_start: usize) -> Option<&TableCell> {
    vertical_merge_origin_cell(self.table, self.row_index, grid_start)
  }

  fn coalesced_row_shading(&self) -> Option<RgbColor> {
    let mut color = None;
    for cell in &self.row.cells {
      if cell.vertical_merge_continue {
        return None;
      }
      match (color, cell.shading) {
        (None, Some(cell_color)) => color = Some(cell_color),
        (Some(color), Some(cell_color)) if color == cell_color => {}
        _ => return None,
      }
    }
    color
  }

  fn paint_horizontal_borders(&self, current: &mut Page, row_top: f32, row_bottom: f32) {
    if self.table_frame.full_width_horizontal_borders {
      if self.row_index == 0
        && let Some(border) = self.table.borders.and_then(|borders| borders.top)
      {
        let inset = border.width_pt / 2.0;
        push_styled_line(
          current,
          self.table_frame.left_pt + inset,
          row_top,
          self.table_frame.right_pt - inset,
          row_top,
          border,
        );
      }
      if self.row_index + 1 == self.table.rows.len()
        && let Some(border) = self.table.borders.and_then(|borders| borders.bottom)
      {
        let inset = border.width_pt / 2.0;
        push_styled_line(
          current,
          self.table_frame.left_pt + inset,
          row_bottom,
          self.table_frame.right_pt - inset,
          row_bottom,
          border,
        );
      }
    }

    let cell_spacing_pt = self.cell_spacing_pt();
    let mut left_pt = row_grid_left(self.table_frame, self.row, cell_spacing_pt);
    let mut grid_index = self.row.grid_before;
    for cell in &self.row.cells {
      if grid_index >= self.table_frame.column_widths.len() {
        break;
      }
      let span = cell.grid_span.max(1).min(
        self
          .table_frame
          .column_widths
          .len()
          .saturating_sub(grid_index),
      );
      let width_pt = self.table_frame.column_widths[grid_index..grid_index + span]
        .iter()
        .sum::<f32>();
      let right_pt = left_pt + width_pt;

      if (self.row_index == 0 || cell_spacing_pt > 0.0)
        && !cell.vertical_merge_continue
        && let Some(border) =
          cell_horizontal_border(self.table, self.row_index, grid_index, cell, true)
      {
        let (border_left, border_right) =
          self.inset_horizontal_border_for_bounds(left_pt, right_pt, border);
        push_styled_line(current, border_left, row_top, border_right, row_top, border);
      }

      let continues_into_next = self
        .table
        .rows
        .get(self.row_index + 1)
        .and_then(|row| row_cell_at_grid(row, grid_index))
        .is_some_and(|next_cell| next_cell.vertical_merge_continue);
      if !continues_into_next
        && let Some(border) =
          cell_horizontal_border(self.table, self.row_index, grid_index, cell, false)
      {
        let (border_left, border_right) = self.inset_inside_horizontal_border(left_pt, right_pt);
        let (border_left, border_right) =
          self.inset_horizontal_border_for_bounds(border_left, border_right, border);
        push_styled_line(
          current,
          border_left,
          row_bottom,
          border_right,
          row_bottom,
          border,
        );
      }

      left_pt = right_pt + cell_spacing_pt;
      grid_index += span;
    }
  }

  fn leading_border_segment(&self, row_top: f32, row_bottom: f32) -> Option<PendingBorderSegment> {
    let first_cell = self.row.cells.first()?;
    let border = vertical_border(self.table, self.row, 0, true)
      .or(first_cell.borders.left)
      .or_else(|| self.table.borders.and_then(|borders| borders.left))?;
    Some(PendingBorderSegment {
      x_pt: row_grid_left(self.table_frame, self.row, self.cell_spacing_pt()),
      start_y_pt: row_top,
      end_y_pt: row_bottom,
      border,
    })
  }

  fn trailing_border_segment(&self, row_top: f32, row_bottom: f32) -> Option<PendingBorderSegment> {
    let border = self
      .row
      .cells
      .last()
      .and_then(|cell| cell.borders.right)
      .or_else(|| self.table.borders.and_then(|borders| borders.right))?;
    Some(PendingBorderSegment {
      x_pt: self.table_frame.right_pt,
      start_y_pt: row_top,
      end_y_pt: row_bottom,
      border,
    })
  }

  fn cell_spacing_pt(&self) -> f32 {
    row_cell_spacing_pt(self.table, self.row)
  }

  fn inset_inside_horizontal_border(&self, left_pt: f32, right_pt: f32) -> (f32, f32) {
    if self.row_index + 1 == self.table.rows.len() {
      return (left_pt, right_pt);
    }
    let Some(borders) = self.table.borders else {
      return (left_pt, right_pt);
    };
    let mut left_pt = left_pt;
    let mut right_pt = right_pt;
    if (left_pt - self.table_frame.left_pt).abs() < LAYOUT_EPSILON_PT
      && let Some(border) = borders.left
    {
      left_pt += border.width_pt;
    }
    if (right_pt - self.table_frame.right_pt).abs() < LAYOUT_EPSILON_PT
      && let Some(border) = borders.right
    {
      right_pt -= border.width_pt;
    }
    (left_pt.min(right_pt), right_pt)
  }

  fn inset_horizontal_border_for_bounds(
    &self,
    left_pt: f32,
    right_pt: f32,
    border: BorderStyle,
  ) -> (f32, f32) {
    if !self.table_frame.full_width_horizontal_borders {
      return (left_pt, right_pt);
    }
    let inset = border.width_pt;
    (
      (left_pt + inset).min(right_pt),
      (right_pt - inset).max(left_pt),
    )
  }
}

struct CellFrame<'a, 'f> {
  table: &'a Table,
  table_frame: &'f TableFrame,
  row: &'a TableRow,
  cell: &'a TableCell,
  row_index: usize,
  grid_start: usize,
  cell_index: usize,
  left_pt: f32,
  width_pt: f32,
  height_pt: f32,
}

impl CellFrame<'_, '_> {
  fn format(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    row_top: f32,
    row_bottom: f32,
    content_offset: f32,
  ) {
    let fragment_height = (row_bottom - row_top).max(0.0);
    let content_height = self.content_height();
    let cell_fragment_height = if content_offset > LAYOUT_EPSILON_PT
      || fragment_height + LAYOUT_EPSILON_PT < content_height
    {
      fragment_height
    } else {
      content_height
    };
    self.paint_vertical_merge_bounds(current, row_top, fragment_height);
    self.paint_background(current, row_top, fragment_height);
    self.paint_leading_border(current, row_top, row_bottom);
    self.paint_trailing_border(current, row_top, row_bottom);
    layout_table_cell(TableCellLayout {
      cell: self.cell,
      escape_following_text_flow_pages: true,
      setup: self.table_frame.block.setup,
      current,
      pages,
      x: self.left_pt,
      y: row_top,
      width: self.width_pt,
      height: cell_fragment_height,
      content_offset,
    });
  }

  fn format_merged_continue(
    &self,
    current: &mut Page,
    row_top: f32,
    row_bottom: f32,
    origin: Option<&TableCell>,
  ) {
    let paint_cell = origin.unwrap_or(self.cell);
    let fragment_height = (row_bottom - row_top).max(0.0);
    self.paint_vertical_merge_bounds(current, row_top, fragment_height);
    self.paint_cell_background(current, row_top, fragment_height, paint_cell);
    self.paint_leading_border_for_cell(current, row_top, row_bottom, paint_cell);
  }

  fn paint_background(&self, current: &mut Page, row_top: f32, height_pt: f32) {
    self.paint_cell_background(current, row_top, height_pt, self.cell);
  }

  fn paint_cell_background(
    &self,
    current: &mut Page,
    row_top: f32,
    height_pt: f32,
    cell: &TableCell,
  ) {
    if let Some(color) = cell.shading {
      current.items.push(PageItem::Fill(FillItem {
        x_pt: self.left_pt,
        y_pt: row_top,
        width_pt: self.width_pt,
        height_pt,
        color,
      }));
    }
  }

  fn paint_vertical_merge_bounds(&self, current: &mut Page, row_top: f32, height_pt: f32) {
    if !self.cell.vertical_merge_continue
      && !table_cell_has_vertical_merge_follow(self.table, self.row_index, self.grid_start)
    {
      return;
    }
    current.items.push(PageItem::Rect(RectItem {
      x_pt: self.left_pt,
      y_pt: row_top,
      width_pt: self.width_pt,
      height_pt,
      fill_color: None,
      fill_opacity: 1.0,
      stroke: Some(BorderStyle::default()),
      stroke_opacity: 0.0,
    }));
  }

  fn paint_leading_border(&self, current: &mut Page, row_top: f32, row_bottom: f32) {
    self.paint_leading_border_for_cell(current, row_top, row_bottom, self.cell);
  }

  fn paint_trailing_border(&self, current: &mut Page, row_top: f32, row_bottom: f32) {
    if row_cell_spacing_pt(self.table, self.row) <= 0.0 {
      return;
    }
    if let Some(border) = vertical_border(self.table, self.row, self.cell_index, false) {
      push_styled_line(
        current,
        self.left_pt + self.width_pt,
        row_top,
        self.left_pt + self.width_pt,
        row_bottom,
        border,
      );
    }
  }

  fn paint_leading_border_for_cell(
    &self,
    current: &mut Page,
    row_top: f32,
    row_bottom: f32,
    cell: &TableCell,
  ) {
    if row_cell_spacing_pt(self.table, self.row) <= 0.0 && self.cell_index == 0 {
      return;
    }
    if let Some(border) = vertical_border(self.table, self.row, self.cell_index, true) {
      push_styled_line(
        current,
        self.left_pt,
        row_top,
        self.left_pt,
        row_bottom,
        border,
      );
    } else if let Some(border) = cell.borders.left {
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

  fn content_height(&self) -> f32 {
    vertical_merge_content_height(
      self.table,
      &self.table_frame.column_widths,
      self.row_index,
      self.grid_start,
      self.height_pt,
    )
    .unwrap_or(self.height_pt)
  }
}

fn row_cell_at_grid(row: &TableRow, grid_index: usize) -> Option<&TableCell> {
  let mut current_grid = row.grid_before;
  for cell in &row.cells {
    let span = cell.grid_span.max(1);
    if grid_index >= current_grid && grid_index < current_grid + span {
      return Some(cell);
    }
    current_grid += span;
  }
  None
}

fn table_cell_has_vertical_merge_follow(
  table: &Table,
  row_index: usize,
  grid_start: usize,
) -> bool {
  table
    .rows
    .iter()
    .skip(row_index + 1)
    .filter_map(|row| row_cell_at_grid(row, grid_start))
    .any(|cell| cell.vertical_merge_continue)
}

fn vertical_merge_origin_cell(
  table: &Table,
  row_index: usize,
  grid_start: usize,
) -> Option<&TableCell> {
  table
    .rows
    .iter()
    .take(row_index)
    .rev()
    .filter_map(|row| row_cell_at_grid(row, grid_start))
    .find(|cell| !cell.vertical_merge_continue)
}

fn vertical_merge_content_height(
  table: &Table,
  column_widths: &[f32],
  row_index: usize,
  grid_start: usize,
  current_row_height: f32,
) -> Option<f32> {
  let mut height = current_row_height;
  let mut previous_row = table.rows.get(row_index)?;
  let mut has_continuation = false;
  for (follow_row_index, row) in table.rows.iter().enumerate().skip(row_index + 1) {
    let Some(cell) = row_cell_at_grid(row, grid_start) else {
      break;
    };
    if !cell.vertical_merge_continue {
      break;
    }
    height += row_cell_spacing_pt(table, previous_row);
    height += table_row_height_with_widths(table, follow_row_index, row, column_widths);
    previous_row = row;
    has_continuation = true;
  }
  has_continuation.then_some(height)
}

fn row_has_vertical_merge_context(table: &Table, row_index: usize) -> bool {
  let Some(row) = table.rows.get(row_index) else {
    return false;
  };
  let mut grid_index = row.grid_before;
  for cell in &row.cells {
    let grid_start = grid_index;
    grid_index += cell.grid_span.max(1);
    if cell.vertical_merge_continue {
      return true;
    }
    if table
      .rows
      .get(row_index + 1)
      .and_then(|next| row_cell_at_grid(next, grid_start))
      .is_some_and(|next_cell| next_cell.vertical_merge_continue)
    {
      return true;
    }
  }
  false
}

fn row_cell_spacing_pt(table: &Table, row: &TableRow) -> f32 {
  row.cell_spacing_pt.unwrap_or(table.cell_spacing_pt)
}

fn table_max_cell_spacing_pt(table: &Table) -> f32 {
  table
    .rows
    .iter()
    .map(|row| row_cell_spacing_pt(table, row))
    .fold(table.cell_spacing_pt, f32::max)
}

fn row_grid_left(table: &TableFrame, row: &TableRow, cell_spacing_pt: f32) -> f32 {
  table.left_pt
    + table
      .column_widths
      .iter()
      .take(row.grid_before)
      .sum::<f32>()
    + cell_spacing_pt * row.grid_before as f32
}

fn table_column_count(table: &Table) -> usize {
  table
    .rows
    .iter()
    .map(|row| {
      row.grid_before + row.cells.iter().map(|cell| cell.grid_span).sum::<usize>() + row.grid_after
    })
    .max()
    .unwrap_or(0)
}

fn table_repeating_header_count(table: &Table) -> usize {
  let count = leading_repeat_header_count(table);
  if count == 0 || count > 10 || table_disables_repeating_headers(table, count) {
    0
  } else {
    count
  }
}

fn leading_repeat_header_count(table: &Table) -> usize {
  table
    .rows
    .iter()
    .take_while(|row| row.repeat_header)
    .count()
}

fn row_repeat_header_effective(table: &Table, row_index: usize) -> bool {
  row_index < table_repeating_header_count(table)
}

fn table_disables_repeating_headers(table: &Table, repeating_header_count: usize) -> bool {
  repeating_header_count == table.rows.len() && table.rows.len() > 1
}

fn table_repeating_header_height_from_row_heights(
  table: &Table,
  repeating_header_count: usize,
  row_heights: &[f32],
) -> f32 {
  table
    .rows
    .iter()
    .enumerate()
    .take(repeating_header_count)
    .map(|(row_index, row)| {
      row_heights
        .get(row_index)
        .copied()
        .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT)
        + row_cell_spacing_pt(table, row)
    })
    .sum()
}

fn table_split_allowed(table: &Table) -> bool {
  if table.placement.is_some() || table.following_text_flow {
    return table.split_allowed;
  }
  if table.rows.len() == 1 && table.rows[0].cant_split {
    return false;
  }
  let leading_repeat_header_count = leading_repeat_header_count(table);
  !(leading_repeat_header_count <= 10
    && table_disables_repeating_headers(table, leading_repeat_header_count))
}

fn cell_horizontal_border(
  table: &Table,
  row_index: usize,
  grid_index: usize,
  cell: &TableCell,
  top_edge: bool,
) -> Option<BorderStyle> {
  let borders = table.borders;
  if top_edge {
    let table_border = borders.and_then(|borders| {
      if row_index == 0 {
        borders.top
      } else {
        borders.inside_horizontal
      }
    });
    stronger_border(
      cell.borders.top,
      row_index
        .checked_sub(1)
        .and_then(|previous| {
          table
            .rows
            .get(previous)
            .and_then(|row| row_cell_at_grid(row, grid_index))
            .and_then(|previous_cell| previous_cell.borders.bottom)
        })
        .or(table_border),
    )
  } else {
    let table_border = borders.and_then(|borders| {
      if row_index + 1 == table.rows.len() {
        borders.bottom
      } else {
        borders.inside_horizontal
      }
    });
    stronger_border(
      cell.borders.bottom,
      table
        .rows
        .get(row_index + 1)
        .and_then(|row| row_cell_at_grid(row, grid_index))
        .and_then(|next_cell| next_cell.borders.top),
    )
    .or(table_border)
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
    let neighbor = if cell_index > 0 {
      row
        .cells
        .get(cell_index - 1)
        .and_then(|previous| previous.borders.right)
    } else {
      None
    };
    stronger_border(cell.borders.left, neighbor).or_else(|| {
      borders.and_then(|borders| {
        if cell_index == 0 && row.grid_before == 0 {
          borders.left
        } else {
          borders.inside_vertical
        }
      })
    })
  } else {
    cell.borders.right.or_else(|| {
      borders.and_then(|borders| {
        if cell_index + 1 == row.cells.len() && row.grid_after == 0 {
          borders.right
        } else {
          borders.inside_vertical
        }
      })
    })
  }
}

fn stronger_border(first: Option<BorderStyle>, second: Option<BorderStyle>) -> Option<BorderStyle> {
  match (first, second) {
    (Some(first), Some(second)) if border_has_priority(second, first) => Some(second),
    (Some(first), Some(_)) => Some(first),
    (None, Some(second)) => Some(second),
    (Some(first), None) => Some(first),
    (None, None) => None,
  }
}

fn border_has_priority(candidate: BorderStyle, current: BorderStyle) -> bool {
  candidate.width_pt > current.width_pt
    || (candidate.width_pt == current.width_pt && current.compound && !candidate.compound)
}

fn table_column_widths(
  table: &Table,
  column_count: usize,
  content_width: f32,
  allow_width_overflow: bool,
) -> Vec<f32> {
  let preferred_width = table_preferred_width(table, content_width, allow_width_overflow);
  if table.column_widths_pt.len() >= column_count {
    let mut widths = table.column_widths_pt[..column_count].to_vec();
    if let Some(preferred) = preferred_width
      && preferred > 0.0
    {
      scale_widths_to_total(&mut widths, preferred);
    }
    if !allow_width_overflow {
      clamp_widths_to_content(&mut widths, content_width);
    }
    return widths;
  }

  if let Some(mut widths) = table_cell_preferred_column_widths(table, column_count, content_width) {
    if let Some(preferred) = preferred_width {
      scale_widths_to_total(&mut widths, preferred);
    }
    if !allow_width_overflow {
      clamp_widths_to_content(&mut widths, content_width);
    }
    return widths;
  }

  let width = if allow_width_overflow {
    preferred_width.unwrap_or(content_width)
  } else {
    preferred_width.unwrap_or(content_width).min(content_width)
  };
  vec![width / column_count as f32; column_count]
}

fn table_cell_preferred_column_widths(
  table: &Table,
  column_count: usize,
  content_width: f32,
) -> Option<Vec<f32>> {
  let mut widths = vec![0.0; column_count];
  let mut saw_preferred_width = false;

  for row in &table.rows {
    let mut grid_index = row.grid_before;
    for cell in &row.cells {
      if grid_index >= column_count {
        break;
      }
      let span = cell
        .grid_span
        .max(1)
        .min(column_count.saturating_sub(grid_index));
      let width = cell
        .preferred_width_pt
        .or_else(|| cell.preferred_width_pct.map(|pct| content_width * pct))
        .unwrap_or(0.0);
      if width > 0.0 {
        saw_preferred_width = true;
        grow_spanned_columns_to_width(&mut widths[grid_index..grid_index + span], width);
      }
      grid_index += span;
    }
  }

  if saw_preferred_width {
    fill_empty_table_columns(&mut widths, content_width);
    Some(widths)
  } else {
    None
  }
}

fn grow_spanned_columns_to_width(widths: &mut [f32], preferred_width: f32) {
  if widths.is_empty() || preferred_width <= 0.0 {
    return;
  }
  let current = widths.iter().sum::<f32>();
  if current >= preferred_width {
    return;
  }
  let extra = (preferred_width - current) / widths.len() as f32;
  for width in widths {
    *width += extra;
  }
}

fn fill_empty_table_columns(widths: &mut [f32], content_width: f32) {
  let empty_count = widths.iter().filter(|width| **width <= 0.0).count();
  if empty_count == 0 {
    return;
  }
  let used = widths.iter().sum::<f32>();
  let fallback = ((content_width - used).max(0.0) / empty_count as f32).max(DEFAULT_FONT_SIZE_PT);
  for width in widths {
    if *width <= 0.0 {
      *width = fallback;
    }
  }
}

fn scale_widths_to_total(widths: &mut [f32], target_total: f32) {
  if target_total <= 0.0 {
    return;
  }
  let total = widths.iter().sum::<f32>();
  if total <= 0.0 {
    return;
  }
  let scale = target_total / total;
  for width in widths {
    *width *= scale;
  }
}

fn clamp_widths_to_content(widths: &mut [f32], content_width: f32) {
  let total = widths.iter().sum::<f32>();
  if total <= content_width || total <= 0.0 {
    return;
  }
  let scale = content_width / total;
  for width in widths {
    *width *= scale;
  }
}

fn table_preferred_width(
  table: &Table,
  content_width: f32,
  allow_width_overflow: bool,
) -> Option<f32> {
  table
    .preferred_width_pt
    .or_else(|| table.preferred_width_pct.map(|pct| content_width * pct))
    .map(|width| {
      if allow_width_overflow {
        width.max(0.0)
      } else {
        width.min(content_width).max(0.0)
      }
    })
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

fn table_total_height_with_widths(table: &Table, column_widths: &[f32]) -> f32 {
  let row_heights = table_row_heights_with_widths(table, column_widths);
  table_total_height_from_row_heights(table, &row_heights)
}

fn table_row_heights_with_widths(table: &Table, column_widths: &[f32]) -> Vec<f32> {
  table
    .rows
    .iter()
    .enumerate()
    .map(|(row_index, row)| table_row_height_with_widths(table, row_index, row, column_widths))
    .collect()
}

fn table_total_height_from_row_heights(table: &Table, row_heights: &[f32]) -> f32 {
  let mut height = 0.0;
  for (row_index, row) in table.rows.iter().enumerate() {
    height += row_heights
      .get(row_index)
      .copied()
      .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT);
    if row_index + 1 < table.rows.len() {
      height += row_cell_spacing_pt(table, row);
    }
  }
  height.max(TABLE_ROW_MIN_HEIGHT_PT)
}

fn estimated_table_height(table: &Table, flow: FlowContext) -> f32 {
  TableFrameLayout::new(table, block_area(flow), false)
    .map(|layout| layout.frame.total_height)
    .unwrap_or_else(|| table_total_height_with_widths(table, &[]))
}

fn table_row_height_with_widths(
  table: &Table,
  row_index: usize,
  row: &TableRow,
  column_widths: &[f32],
) -> f32 {
  let mut grid_index = row.grid_before;
  let mut content_height = TABLE_ROW_MIN_HEIGHT_PT;
  for cell in &row.cells {
    let width = spanned_cell_width(cell, column_widths, &mut grid_index);
    if !cell.vertical_merge_continue {
      content_height = content_height.max(table_cell_content_height(cell, width));
    }
  }
  match (row.height_pt, row.exact_height) {
    (Some(height), true) => {
      height
        + row_bottom_cell_margin_extent(row)
        + row_bottom_border_spacing_extent(table, row_index, row)
    }
    (Some(height), false) => content_height.max(
      height
        + row_top_cell_margin_extent(row)
        + row_bottom_cell_margin_extent(row)
        + row_top_border_space_extent(table, row_index, row)
        + row_bottom_border_spacing_extent(table, row_index, row),
    ),
    (None, _) => content_height,
  }
}

fn row_top_cell_margin_extent(row: &TableRow) -> f32 {
  row
    .cells
    .iter()
    .filter(|cell| !cell.vertical_merge_continue)
    .map(|cell| cell.margins.top_pt)
    .fold(0.0, f32::max)
}

fn row_bottom_cell_margin_extent(row: &TableRow) -> f32 {
  row
    .cells
    .iter()
    .filter(|cell| !cell.vertical_merge_continue)
    .map(|cell| cell.margins.bottom_pt)
    .fold(0.0, f32::max)
}

fn row_top_border_space_extent(table: &Table, row_index: usize, row: &TableRow) -> f32 {
  let mut grid_index = row.grid_before;
  let mut extent: f32 = 0.0;
  for cell in &row.cells {
    let cell_grid_index = grid_index;
    grid_index += cell.grid_span.max(1);
    if cell.vertical_merge_continue {
      continue;
    }
    extent = extent.max(top_border_space_extent(cell_horizontal_border(
      table,
      row_index,
      cell_grid_index,
      cell,
      true,
    )));
  }
  extent
}

fn row_bottom_border_spacing_extent(table: &Table, row_index: usize, row: &TableRow) -> f32 {
  let mut grid_index = row.grid_before;
  let mut extent: f32 = 0.0;
  for cell in &row.cells {
    let cell_grid_index = grid_index;
    grid_index += cell.grid_span.max(1);
    if cell.vertical_merge_continue {
      continue;
    }
    let continues_into_next = table
      .rows
      .get(row_index + 1)
      .and_then(|next| row_cell_at_grid(next, cell_grid_index))
      .is_some_and(|next_cell| next_cell.vertical_merge_continue);
    if continues_into_next {
      continue;
    }
    extent = extent.max(bottom_border_spacing_extent(cell_horizontal_border(
      table,
      row_index,
      cell_grid_index,
      cell,
      false,
    )));
  }
  extent
}

fn top_border_space_extent(border: Option<BorderStyle>) -> f32 {
  border.map_or(0.0, |border| border.spacing_pt + border.width_pt)
}

fn bottom_border_spacing_extent(border: Option<BorderStyle>) -> f32 {
  border.map_or(0.0, |border| border.spacing_pt)
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

struct TableCellLayout<'a> {
  cell: &'a TableCell,
  escape_following_text_flow_pages: bool,
  setup: PageSetup,
  current: &'a mut Page,
  pages: &'a mut Vec<Page>,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  content_offset: f32,
}

fn layout_table_cell(fragment: TableCellLayout<'_>) {
  let TableCellLayout {
    cell,
    escape_following_text_flow_pages,
    setup,
    current,
    pages,
    x,
    y,
    width,
    height,
    content_offset,
  } = fragment;
  let content_width =
    (width - cell.margins.left_pt - cell.margins.right_pt).max(DEFAULT_FONT_SIZE_PT);
  let content_height = table_cell_content_height(cell, width);
  let first_line_style = table_cell_first_line_style(cell);
  let first_line_height = inline_text_height(&first_line_style);
  let first_line_baseline_offset = baseline_offset_in_line(&first_line_style, first_line_height);
  let split_fragment =
    content_offset > LAYOUT_EPSILON_PT || content_height > height + LAYOUT_EPSILON_PT;
  let aligned_content_top = if split_fragment {
    y + cell.margins.top_pt
  } else {
    match cell.vertical_alignment {
      TableCellVerticalAlignment::Top => y + cell.margins.top_pt,
      TableCellVerticalAlignment::Center => {
        y + ((height - content_height) / 2.0).max(cell.margins.top_pt)
      }
      TableCellVerticalAlignment::Bottom => {
        y + (height - cell.margins.bottom_pt - content_height).max(cell.margins.top_pt)
      }
    }
  };
  let mut text_y = aligned_content_top + first_line_baseline_offset - content_offset;
  let content_start_y = text_y;
  let text_left = x + cell.margins.left_pt;
  let text_bottom = y + height - cell.margins.bottom_pt;
  let following_text_flow_bottom = following_text_flow_cell_bottom(current, text_bottom);
  let flow_bottom = if split_fragment {
    text_bottom
  } else {
    UNBOUNDED_LAYOUT_EXTENT_PT
  };
  let flow = FlowContext {
    setup,
    section_index: current.section_index,
    section_page_index: 0,
    column_index: 0,
    columns: SectionColumns::default(),
    content_top_pt: text_y,
    content_left_pt: text_left,
    content_bottom: flow_bottom,
    body_content_bottom_pt: flow_bottom,
    content_width,
    layout_cell_bounds: Some(FrameBounds {
      x_pt: x,
      y_pt: y,
      width_pt: width,
      height_pt: height,
    }),
    default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::TableCell,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
  };
  let mut nested_page = empty_page(setup, current.section_index);
  let mut discarded_pages = Vec::new();
  let has_following_text_flow_table = cell.blocks.iter().any(|block| {
    matches!(
      block,
      Block::Table(table) if table.placement.is_some() && table.following_text_flow
    )
  });

  if has_following_text_flow_table && escape_following_text_flow_pages && !split_fragment {
    for (index, block) in cell.blocks.iter().enumerate() {
      let previous = index
        .checked_sub(1)
        .and_then(|index| cell.blocks.get(index));
      let next = cell.blocks.get(index + 1);
      let block_flow = match block {
        Block::Table(table) if table.placement.is_some() && table.following_text_flow => {
          FlowContext {
            content_bottom: following_text_flow_bottom.max(text_y + DEFAULT_LINE_HEIGHT_PT),
            body_content_bottom_pt: following_text_flow_bottom.max(text_y + DEFAULT_LINE_HEIGHT_PT),
            ..flow
          }
        }
        _ => flow,
      };
      let (_, next_y) =
        layout_document_block(previous, block, next, block_flow, current, pages, text_y);
      text_y = next_y;
    }
    return;
  }

  for (index, block) in cell.blocks.iter().enumerate() {
    if text_y > text_bottom {
      break;
    }
    let previous = index
      .checked_sub(1)
      .and_then(|index| cell.blocks.get(index));
    let next = cell.blocks.get(index + 1);
    let block_flow = match block {
      Block::Table(table) if table.placement.is_some() && table.following_text_flow => {
        FlowContext {
          content_bottom: following_text_flow_bottom.max(text_y + DEFAULT_LINE_HEIGHT_PT),
          body_content_bottom_pt: following_text_flow_bottom.max(text_y + DEFAULT_LINE_HEIGHT_PT),
          ..flow
        }
      }
      _ => flow,
    };
    let (_, next_y) = layout_document_block(
      previous,
      block,
      next,
      block_flow,
      &mut nested_page,
      &mut discarded_pages,
      text_y,
    );
    text_y = next_y;
  }
  materialize_pending_floating_table_follows_in_local_pages(&mut nested_page, &mut discarded_pages);

  if has_following_text_flow_table && !split_fragment {
    let mut local_pages = ordered_local_pages(nested_page, discarded_pages).into_iter();
    if let Some(first_page) = local_pages.next() {
      current.items.extend(
        first_page
          .items
          .into_iter()
          .filter(|item| table_cell_item_intersects_vertical_bounds(item, y, text_bottom)),
      );
    }
    for follow_page in local_pages {
      if follow_page.items.is_empty() {
        continue;
      }
      current
        .pending_floating_table_follows
        .push(PendingFloatingTableFollow {
          setup: follow_page.setup,
          section_index: follow_page.section_index,
          section_page_index: follow_page.section_page_index,
          items: follow_page.items,
          frame_fragments: Vec::new(),
          frame_influences: Vec::new(),
          wrap_exclusions: follow_page.wrap_exclusions,
        });
    }
    return;
  }

  current.items.extend(
    flatten_nested_pages(nested_page, discarded_pages, content_start_y, text_bottom)
      .into_iter()
      .filter(|item| table_cell_item_intersects_vertical_bounds(item, y, text_bottom)),
  );
}

fn following_text_flow_cell_bottom(current: &Page, text_bottom: f32) -> f32 {
  // Source: LibreOffice sw/source/core/layout/fly.cxx GetFlyAnchorBottom() and
  // SwFlyFrame::Grow_(): split fly growth is limited by the current body
  // deadline. A cell whose estimated height grows past the page bottom must
  // still split the nested following-text-flow floating table on this page.
  text_bottom.min(current.setup.height_pt - current.setup.margin_bottom_pt)
}

fn table_cell_first_line_style(cell: &TableCell) -> TextStyle {
  for block in &cell.blocks {
    let Block::Paragraph(paragraph) = block else {
      continue;
    };
    return paragraph_base_line_style(paragraph);
  }
  TextStyle::default()
}

fn table_cell_item_intersects_vertical_bounds(item: &PageItem, top: f32, bottom: f32) -> bool {
  match item {
    PageItem::Text(text) => text.y_pt + text.line_height_pt >= top && text.y_pt <= bottom,
    PageItem::Image(image) => image.y_pt + image.height_pt >= top && image.y_pt <= bottom,
    PageItem::Rect(rect) => rect.y_pt + rect.height_pt >= top && rect.y_pt <= bottom,
    PageItem::Fill(_) | PageItem::Line(_) | PageItem::Polyline(_) => true,
  }
}

fn layout_shape_text_box(
  current: &mut Page,
  parent_flow: FlowContext,
  shape: &crate::docx::InlineShape,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
) {
  if shape.text_box_blocks.is_empty() {
    return;
  }

  let content_left = x + shape.text_inset_left_pt;
  let content_top = y + shape.text_inset_top_pt;
  let content_width =
    (width - shape.text_inset_left_pt - shape.text_inset_right_pt).max(DEFAULT_FONT_SIZE_PT);
  let content_bottom = y + height - shape.text_inset_bottom_pt;
  if content_bottom <= content_top {
    return;
  }

  let measure_flow = FlowContext {
    setup: parent_flow.setup,
    section_index: current.section_index,
    section_page_index: parent_flow.section_page_index,
    column_index: parent_flow.column_index,
    columns: SectionColumns::default(),
    content_top_pt: content_top,
    content_left_pt: content_left,
    content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
    body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
    content_width,
    layout_cell_bounds: parent_flow.layout_cell_bounds,
    default_tab_stop_pt: parent_flow.default_tab_stop_pt,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::TableCell,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
  };
  let content_height = shape
    .text_box_blocks
    .iter()
    .map(|block| estimated_block_height(block, measure_flow))
    .sum::<f32>();
  let text_y = match shape.text_vertical_alignment {
    TextBoxVerticalAlignment::Top => content_top,
    TextBoxVerticalAlignment::Center => {
      content_top + ((content_bottom - content_top - content_height) / 2.0).max(0.0)
    }
    TextBoxVerticalAlignment::Bottom => (content_bottom - content_height).max(content_top),
  };

  let flow = FlowContext {
    // Source: LibreOffice imports WPS text boxes as drawing shapes
    // (oox/source/shape/WpsContext.cxx + drawingml/shape.cxx). Shape text is
    // formatted inside the Sdr object and clipped/overflowed there; it does
    // not create follow pages in the document body when it reaches the shape
    // bottom.
    content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
    body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
    ..measure_flow
  };
  let mut nested_page = empty_page(parent_flow.setup, current.section_index);
  let mut discarded_pages = Vec::new();
  let shape_y = y;
  let mut text_cursor_y = text_y;
  for (index, block) in shape.text_box_blocks.iter().enumerate() {
    if text_cursor_y > content_bottom {
      break;
    }
    let previous = index
      .checked_sub(1)
      .and_then(|index| shape.text_box_blocks.get(index));
    let next = shape.text_box_blocks.get(index + 1);
    let (_, next_y) = layout_document_block(
      previous,
      block,
      next,
      flow,
      &mut nested_page,
      &mut discarded_pages,
      text_cursor_y,
    );
    text_cursor_y = next_y;
  }
  materialize_pending_floating_table_follows_in_local_pages(&mut nested_page, &mut discarded_pages);

  let auto_fit_inset = textbox_auto_fit_bounds_inset(shape);
  current.items.extend(
    flatten_nested_pages(nested_page, discarded_pages, text_y, content_bottom)
      .into_iter()
      .filter_map(|item| {
        let item = if shape.text_box_auto_fit {
          textbox_item_inside_shape_bounds(
            item,
            x + auto_fit_inset,
            shape_y + auto_fit_inset,
            (width - auto_fit_inset * 2.0).max(DEFAULT_FONT_SIZE_PT),
            (height - auto_fit_inset * 2.0).max(DEFAULT_LINE_HEIGHT_PT),
          )
        } else {
          item
        };
        table_cell_item_intersects_vertical_bounds(&item, content_top, content_bottom)
          .then_some(item)
      }),
  );
}

fn textbox_auto_fit_bounds_inset(shape: &crate::docx::InlineShape) -> f32 {
  shape
    .stroke
    .map(|stroke| stroke.width_pt)
    .unwrap_or(BorderStyle::default().width_pt / 2.0)
}

fn shape_has_invisible_auto_fit_textbox_bounds(shape: &crate::docx::InlineShape) -> bool {
  shape.text_box_auto_fit
    && !shape.text_box_blocks.is_empty()
    && shape.fill_color.is_none()
    && shape.fill_image.is_none()
    && shape.stroke.is_none()
    && shape.additional_fill_colors.is_empty()
}

fn textbox_item_inside_shape_bounds(
  mut item: PageItem,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> PageItem {
  let right = left + width;
  let bottom = top + height;
  let Some((item_left, item_top, item_right, item_bottom)) = item_bounds(&item) else {
    return item;
  };
  let dx = if item_left < left {
    left - item_left
  } else if item_right > right {
    right - item_right
  } else {
    0.0
  };
  let dy = if item_top < top {
    top - item_top
  } else if item_bottom > bottom {
    bottom - item_bottom
  } else {
    0.0
  };
  if dx.abs() > LAYOUT_EPSILON_PT || dy.abs() > LAYOUT_EPSILON_PT {
    shift_item(&mut item, dx, dy);
  }
  item
}

fn table_cell_content_height(cell: &TableCell, cell_width: f32) -> f32 {
  let content_width =
    (cell_width - cell.margins.left_pt - cell.margins.right_pt).max(DEFAULT_FONT_SIZE_PT);
  let flow = FlowContext {
    setup: PageSetup::default(),
    section_index: 0,
    section_page_index: 0,
    column_index: 0,
    columns: SectionColumns::default(),
    content_top_pt: 0.0,
    content_left_pt: cell.margins.left_pt,
    content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
    body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
    content_width,
    layout_cell_bounds: Some(FrameBounds {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: cell_width,
      height_pt: 0.0,
    }),
    default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::TableCell,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
  };
  let content = cell
    .blocks
    .iter()
    .enumerate()
    .map(|(index, block)| match block {
      Block::Paragraph(paragraph) => {
        let estimated = estimated_paragraph_height(paragraph, flow);
        let min_height = paragraph_line_height_for_setup(
          paragraph,
          &paragraph_base_line_style(paragraph),
          flow.setup,
          flow.text_segmentation,
        );
        let cell_border_spacing = if index + 1 == cell.blocks.len() {
          table_cell_line_spacing_before_border(paragraph)
        } else {
          0.0
        };
        estimated.max(min_height) + cell_border_spacing
      }
      Block::Table(table) => estimated_table_height(table, flow),
      Block::Frame(frame) => estimated_frame_height(frame, flow),
    })
    .sum::<f32>()
    .max(inline_text_height(&table_cell_first_line_style(cell)));
  cell.margins.top_pt + content + cell.margins.bottom_pt
}

fn table_cell_line_spacing_before_border(paragraph: &crate::docx::Paragraph) -> f32 {
  if !matches!(paragraph.format.line_height_rule, LineHeightRule::Auto) {
    return 0.0;
  }
  let Some(multiple) = paragraph.format.line_height_pt else {
    return 0.0;
  };
  if multiple <= 1.0 {
    return 0.0;
  }
  // Source: LibreOffice sw/source/core/layout/frmtool.cxx
  // SwBorderAttrs::CalcLineSpacing_ adds 115% of the proportional line spacing
  // excess before a table cell border for Word-compatible DOCX layout.
  word_auto_line_height(&paragraph_base_line_style(paragraph)) * (multiple - 1.0)
}

fn floating_image_position(
  placement: FloatingImagePlacement,
  flow: FlowContext,
  current_x: f32,
  current_y: f32,
  image_width: f32,
  image_height: f32,
) -> (f32, f32) {
  if placement.layout_in_cell && flow.text_segmentation == TextSegmentation::TableCell {
    let cell_bounds = flow.layout_cell_bounds.unwrap_or(FrameBounds {
      x_pt: flow.content_left_pt,
      y_pt: current_y,
      width_pt: flow.content_width,
      height_pt: 0.0,
    });
    let (base_x, reference_width) = match placement.horizontal_relative_to {
      HorizontalImageReference::Column | HorizontalImageReference::Character => {
        (flow.content_left_pt, flow.content_width)
      }
      HorizontalImageReference::Page
      | HorizontalImageReference::Margin
      | HorizontalImageReference::LeftMargin
      | HorizontalImageReference::RightMargin
      | HorizontalImageReference::InsideMargin
      | HorizontalImageReference::OutsideMargin => (cell_bounds.x_pt, cell_bounds.width_pt),
    };
    let (base_y, reference_height) = match placement.vertical_relative_to {
      VerticalImageReference::Paragraph | VerticalImageReference::Line => (current_y, 0.0),
      VerticalImageReference::Page
      | VerticalImageReference::Margin
      | VerticalImageReference::TopMargin
      | VerticalImageReference::BottomMargin
      | VerticalImageReference::InsideMargin
      | VerticalImageReference::OutsideMargin => (cell_bounds.y_pt, cell_bounds.height_pt),
    };
    return (
      base_x
        + aligned_horizontal_offset(placement.horizontal_alignment, reference_width, image_width)
        + placement.horizontal_offset_pt,
      base_y
        + aligned_vertical_offset(placement.vertical_alignment, reference_height, image_height)
        + placement.vertical_offset_pt,
    );
  }

  let horizontal_reference = effective_horizontal_reference(placement);
  let vertical_reference = effective_vertical_reference(placement);
  let (base_x, reference_width) = match horizontal_reference {
    HorizontalImageReference::Page => (0.0, flow.setup.width_pt),
    HorizontalImageReference::Margin => (
      flow.setup.margin_left_pt,
      flow.setup.width_pt - flow.setup.margin_left_pt - flow.setup.margin_right_pt,
    ),
    HorizontalImageReference::Column => (flow.content_left_pt, flow.content_width),
    HorizontalImageReference::Character => (current_x, 0.0),
    HorizontalImageReference::LeftMargin => (0.0, horizontal_page_left_width(flow)),
    HorizontalImageReference::RightMargin => (
      flow.setup.width_pt - horizontal_page_right_width(flow),
      horizontal_page_right_width(flow),
    ),
    HorizontalImageReference::InsideMargin => horizontal_inside_outside_base(true, flow),
    HorizontalImageReference::OutsideMargin => horizontal_inside_outside_base(false, flow),
  };
  let (base_y, reference_height) = match vertical_reference {
    VerticalImageReference::Page => (0.0, flow.setup.height_pt),
    VerticalImageReference::Margin => (
      flow.setup.margin_top_pt,
      flow.setup.height_pt - flow.setup.margin_top_pt - flow.setup.margin_bottom_pt,
    ),
    VerticalImageReference::Paragraph | VerticalImageReference::Line => (current_y, 0.0),
    VerticalImageReference::TopMargin => (0.0, flow.setup.margin_top_pt),
    VerticalImageReference::BottomMargin => (
      flow.setup.height_pt - flow.setup.margin_bottom_pt,
      flow.setup.margin_bottom_pt,
    ),
    VerticalImageReference::InsideMargin => (0.0, flow.setup.margin_top_pt),
    VerticalImageReference::OutsideMargin => (
      flow.setup.height_pt - flow.setup.margin_bottom_pt,
      flow.setup.margin_bottom_pt,
    ),
  };
  (
    base_x
      + aligned_horizontal_offset(placement.horizontal_alignment, reference_width, image_width)
      + placement.horizontal_offset_pt,
    base_y
      + aligned_vertical_offset(placement.vertical_alignment, reference_height, image_height)
      + placement.vertical_offset_pt,
  )
}

fn floating_frame_position(
  placement: FloatingFramePlacement,
  flow: FlowContext,
  current_y: f32,
  frame_width: f32,
  frame_height: f32,
) -> (f32, f32) {
  let (base_x, reference_width) = match placement.horizontal_anchor {
    FrameHorizontalAnchor::Text => (flow.content_left_pt, flow.content_width),
    FrameHorizontalAnchor::Margin => (
      flow.setup.margin_left_pt,
      flow.setup.width_pt - flow.setup.margin_left_pt - flow.setup.margin_right_pt,
    ),
    FrameHorizontalAnchor::Page => (0.0, flow.setup.width_pt),
  };
  let (base_y, reference_height) = match placement.vertical_anchor {
    FrameVerticalAnchor::Text => (current_y, 0.0),
    FrameVerticalAnchor::Margin => (
      flow.setup.margin_top_pt,
      flow.setup.height_pt - flow.setup.margin_top_pt - flow.setup.margin_bottom_pt,
    ),
    FrameVerticalAnchor::Page => (0.0, flow.setup.height_pt),
  };
  (
    base_x
      + aligned_frame_horizontal_offset(
        placement.horizontal_alignment,
        reference_width,
        frame_width,
      )
      + placement.horizontal_offset_pt
      + placement.margin_left_pt,
    base_y
      + aligned_frame_vertical_offset(placement.vertical_alignment, reference_height, frame_height)
      + placement.vertical_offset_pt
      + placement.margin_top_pt,
  )
}

fn aligned_frame_horizontal_offset(
  alignment: Option<FrameHorizontalAlignment>,
  reference_width: f32,
  frame_width: f32,
) -> f32 {
  let available = (reference_width - frame_width).max(0.0);
  match alignment {
    Some(FrameHorizontalAlignment::Center) => available / 2.0,
    Some(FrameHorizontalAlignment::Right) | Some(FrameHorizontalAlignment::Outside) => available,
    Some(FrameHorizontalAlignment::Left) | Some(FrameHorizontalAlignment::Inside) | None => 0.0,
  }
}

fn aligned_frame_vertical_offset(
  alignment: Option<FrameVerticalAlignment>,
  reference_height: f32,
  frame_height: f32,
) -> f32 {
  let available = (reference_height - frame_height).max(0.0);
  match alignment {
    Some(FrameVerticalAlignment::Center) => available / 2.0,
    Some(FrameVerticalAlignment::Bottom) | Some(FrameVerticalAlignment::Outside) => available,
    Some(FrameVerticalAlignment::Top)
    | Some(FrameVerticalAlignment::Inside)
    | Some(FrameVerticalAlignment::Inline)
    | None => 0.0,
  }
}

fn frame_wrap_blocks_flow(wrap: FrameWrapMode) -> bool {
  matches!(wrap, FrameWrapMode::None | FrameWrapMode::NotBeside)
}

fn floating_frame_blocks_flow(frame: &FloatingFrame) -> bool {
  if matches!(frame.height_rule, FrameHeightRule::Exact)
    && frame.placement.vertical_offset_explicit
    && matches!(frame.placement.vertical_anchor, FrameVerticalAnchor::Text)
  {
    return false;
  }
  frame_wrap_blocks_flow(frame.placement.wrap)
}

fn effective_horizontal_reference(placement: FloatingImagePlacement) -> HorizontalImageReference {
  if placement.layout_in_cell {
    return placement.horizontal_relative_to;
  }
  match placement.horizontal_relative_to {
    HorizontalImageReference::Column | HorizontalImageReference::Character => {
      HorizontalImageReference::Margin
    }
    reference => reference,
  }
}

fn effective_vertical_reference(placement: FloatingImagePlacement) -> VerticalImageReference {
  if placement.layout_in_cell {
    return placement.vertical_relative_to;
  }
  match placement.vertical_relative_to {
    VerticalImageReference::Paragraph | VerticalImageReference::Line => {
      VerticalImageReference::Page
    }
    reference => reference,
  }
}

fn relative_floating_width(placement: FloatingImagePlacement, flow: FlowContext) -> Option<f32> {
  let reference = placement.relative_width_to?;
  let pct = placement.relative_width_pct?;
  if pct <= 0.0 {
    return None;
  }
  Some((horizontal_reference_width(reference, flow) * pct).max(0.0))
}

fn relative_floating_height(placement: FloatingImagePlacement, flow: FlowContext) -> Option<f32> {
  let reference = placement.relative_height_to?;
  let pct = placement.relative_height_pct?;
  if pct <= 0.0 {
    return None;
  }
  Some((vertical_reference_height(reference, flow) * pct).max(0.0))
}

fn floating_shape_is_zero_relative_background(
  placement: FloatingImagePlacement,
  shape: &InlineShape,
) -> bool {
  shape.suppress_zero_relative_background
    && (shape.width_pt <= LAYOUT_EPSILON_PT
      || shape.height_pt <= LAYOUT_EPSILON_PT
      || (shape.effect_left_pt <= LAYOUT_EPSILON_PT
        && shape.effect_top_pt <= LAYOUT_EPSILON_PT
        && shape.effect_right_pt <= LAYOUT_EPSILON_PT
        && shape.effect_bottom_pt <= LAYOUT_EPSILON_PT))
    && placement.relative_width_pct.is_some_and(|pct| pct <= 0.0)
    && placement.relative_height_pct.is_some_and(|pct| pct <= 0.0)
    && shape.fill_color.is_some()
    && shape.fill_image.is_none()
    && shape.stroke.is_none()
    && shape.text_box_blocks.is_empty()
}

fn floating_shape_may_extend_outside_page(placement: FloatingImagePlacement) -> bool {
  matches!(
    (placement.vertical_relative_to, placement.vertical_alignment),
    (
      VerticalImageReference::BottomMargin,
      Some(VerticalImageAlignment::Outside)
    )
  )
}

fn adjusted_floating_shape_y(
  placement: FloatingImagePlacement,
  shape: &InlineShape,
  y_pt: f32,
) -> f32 {
  if shape.effect_top_pt <= LAYOUT_EPSILON_PT && shape.effect_bottom_pt <= LAYOUT_EPSILON_PT {
    return y_pt;
  }
  match placement.vertical_alignment {
    Some(VerticalImageAlignment::Top) | Some(VerticalImageAlignment::Inside) => {
      y_pt + (shape.effect_top_pt - BorderStyle::default().width_pt / 2.0).max(0.0)
    }
    Some(VerticalImageAlignment::Bottom) | Some(VerticalImageAlignment::Outside) => {
      y_pt - (shape.effect_bottom_pt + BorderStyle::default().width_pt / 2.0)
    }
    Some(VerticalImageAlignment::Center) => y_pt,
    None => y_pt,
  }
}

fn adjusted_floating_shape_x(
  placement: FloatingImagePlacement,
  shape: &InlineShape,
  x_pt: f32,
) -> f32 {
  if shape.effect_left_pt <= LAYOUT_EPSILON_PT && shape.effect_right_pt <= LAYOUT_EPSILON_PT {
    return x_pt;
  }
  match placement.horizontal_alignment {
    Some(HorizontalImageAlignment::Left) | Some(HorizontalImageAlignment::Inside) => {
      x_pt + (shape.effect_left_pt - BorderStyle::default().width_pt / 2.0).max(0.0)
    }
    Some(HorizontalImageAlignment::Right) | Some(HorizontalImageAlignment::Outside) => {
      x_pt - (shape.effect_right_pt + BorderStyle::default().width_pt / 2.0)
    }
    Some(HorizontalImageAlignment::Center) | None => x_pt,
  }
}

fn horizontal_reference_width(reference: HorizontalImageReference, flow: FlowContext) -> f32 {
  match reference {
    HorizontalImageReference::Page => flow.setup.width_pt,
    HorizontalImageReference::Margin => {
      flow.setup.width_pt - flow.setup.margin_left_pt - flow.setup.margin_right_pt
    }
    HorizontalImageReference::Column => flow.content_width,
    HorizontalImageReference::Character => 0.0,
    HorizontalImageReference::LeftMargin => horizontal_page_left_width(flow),
    HorizontalImageReference::RightMargin => horizontal_page_right_width(flow),
    HorizontalImageReference::InsideMargin => horizontal_inside_outside_width(true, flow),
    HorizontalImageReference::OutsideMargin => horizontal_inside_outside_width(false, flow),
  }
}

fn horizontal_page_left_width(flow: FlowContext) -> f32 {
  if flow.setup.mirror_margins {
    horizontal_inside_outside_width(false, flow)
  } else {
    flow.setup.margin_left_pt
  }
}

fn horizontal_page_right_width(flow: FlowContext) -> f32 {
  if flow.setup.mirror_margins {
    horizontal_inside_outside_width(true, flow)
  } else {
    flow.setup.margin_right_pt
  }
}

fn horizontal_inside_outside_width(inside: bool, flow: FlowContext) -> f32 {
  let odd_page = flow.section_page_index.is_multiple_of(2);
  match (inside, odd_page) {
    (true, true) | (false, false) => flow.setup.margin_right_pt,
    (true, false) | (false, true) => flow.setup.margin_left_pt,
  }
}

fn horizontal_inside_outside_base(inside: bool, flow: FlowContext) -> (f32, f32) {
  let width = horizontal_inside_outside_width(inside, flow);
  let odd_page = flow.section_page_index.is_multiple_of(2);
  let left_side = matches!((inside, odd_page), (false, true) | (true, false));
  if left_side {
    (0.0, width)
  } else {
    (flow.setup.width_pt - width, width)
  }
}

fn vertical_reference_height(reference: VerticalImageReference, flow: FlowContext) -> f32 {
  match reference {
    VerticalImageReference::Page => flow.setup.height_pt,
    VerticalImageReference::Margin => {
      flow.setup.height_pt - flow.setup.margin_top_pt - flow.setup.margin_bottom_pt
    }
    VerticalImageReference::Paragraph | VerticalImageReference::Line => 0.0,
    VerticalImageReference::TopMargin | VerticalImageReference::InsideMargin => {
      flow.setup.margin_top_pt
    }
    VerticalImageReference::BottomMargin | VerticalImageReference::OutsideMargin => {
      flow.setup.margin_bottom_pt
    }
  }
}

fn aligned_horizontal_offset(
  alignment: Option<HorizontalImageAlignment>,
  reference_width: f32,
  image_width: f32,
) -> f32 {
  let available = (reference_width - image_width).max(0.0);
  match alignment {
    Some(HorizontalImageAlignment::Center) => available / 2.0,
    Some(HorizontalImageAlignment::Right) | Some(HorizontalImageAlignment::Outside) => available,
    Some(HorizontalImageAlignment::Left) | Some(HorizontalImageAlignment::Inside) | None => 0.0,
  }
}

fn aligned_vertical_offset(
  alignment: Option<VerticalImageAlignment>,
  reference_height: f32,
  image_height: f32,
) -> f32 {
  let available = (reference_height - image_height).max(0.0);
  match alignment {
    Some(VerticalImageAlignment::Center) => available / 2.0,
    Some(VerticalImageAlignment::Bottom) | Some(VerticalImageAlignment::Outside) => available,
    Some(VerticalImageAlignment::Top) | Some(VerticalImageAlignment::Inside) | None => 0.0,
  }
}

fn layout_paragraph(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  y: f32,
  spacing_after_pt: f32,
) -> (FlowContext, f32) {
  TextFrameLayout::new(paragraph, flow, spacing_after_pt).format(current, pages, y)
}

#[derive(Clone, Copy, Debug)]
struct TextFrame {
  default_line_left: f32,
  first_line_left: f32,
  default_line_right: f32,
  paragraph_left: f32,
  base_line_height: f32,
  line_height_rule: LineHeightRule,
}

impl TextFrame {
  fn new(paragraph: &crate::docx::Paragraph, flow: FlowContext) -> Self {
    let default_line_left = flow.content_left_pt + paragraph.format.indent_left_pt;
    let first_line_left =
      (default_line_left + paragraph.format.first_line_indent_pt).max(flow.content_left_pt);
    let base_line_style = paragraph_base_line_style(paragraph);
    Self {
      default_line_left,
      first_line_left,
      default_line_right: default_line_left + flow.content_width,
      paragraph_left: default_line_left.min(first_line_left),
      base_line_height: paragraph_line_height_for_setup(
        paragraph,
        &base_line_style,
        flow.setup,
        flow.text_segmentation,
      ),
      line_height_rule: paragraph.format.line_height_rule,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct LineFrame {
  left_pt: f32,
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
  wrap_exclusions: &'a mut Vec<WrapExclusion>,
  state: &'a mut TextFrameState,
  active: ActiveTextFrame,
  line_left: f32,
  line_right: f32,
  justify: bool,
  line_item_start_index: &'a mut usize,
  line_has_form_widget: &'a mut bool,
}

struct TextFrameLayout<'a> {
  paragraph: &'a crate::docx::Paragraph,
  flow: FlowContext,
  frame: TextFrame,
  spacing_after_pt: f32,
}

impl<'a> TextFrameLayout<'a> {
  fn new(paragraph: &'a crate::docx::Paragraph, flow: FlowContext, spacing_after_pt: f32) -> Self {
    Self {
      paragraph,
      flow,
      frame: TextFrame::new(paragraph, flow),
      spacing_after_pt,
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
    if advance.justify {
      justify_line_items(
        &mut advance.current.items,
        *advance.line_item_start_index,
        y,
        advance.line_left,
        advance.line_right,
      );
    }
    push_page_fragment(
      advance.current,
      PageFragmentRecord {
        kind: FrameFragmentKind::ParagraphLine,
        split: FragmentSplitKind::Complete,
        index: advance.state.line_fragments.len(),
        row_index: advance.state.line_fragments.len(),
        cell_index: None,
        item_start: *advance.line_item_start_index,
        item_end: advance.current.items.len(),
      },
    );
    let real_height = line_real_height(self.paragraph, *line_height, *advance.line_has_form_widget);
    advance.state.finish_line(y, real_height);
    let mut next_y = y + real_height;
    *advance.line_has_form_widget = false;
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
      reset_wrap_exclusions_for_y(advance.current, next_y, advance.wrap_exclusions);
    }
    next_y = dodge_text_wrap_exclusions(next_y, *line_height, advance.wrap_exclusions);
    let (line_left, line_right) =
      self.line_bounds(next_frame, next_y, *line_height, advance.wrap_exclusions);
    *advance.line_item_start_index = advance.current.items.len();
    (next_flow, next_frame, next_y, line_left, line_right)
  }

  fn force_text_page_break(
    &self,
    flow: FlowContext,
    current: &mut Page,
    pages: &mut Vec<Page>,
    wrap_exclusions: &mut Vec<WrapExclusion>,
  ) -> (FlowContext, TextFrame, f32, f32, f32, f32) {
    let (next_flow, y) = force_page_break(flow, current, pages);
    let next_frame = TextFrame::new(self.paragraph, next_flow);
    reset_wrap_exclusions_for_y(current, y, wrap_exclusions);
    let y = dodge_text_wrap_exclusions(y, next_frame.base_line_height, wrap_exclusions);
    (
      next_flow,
      next_frame,
      y,
      next_frame.first_line_left,
      next_frame.default_line_right,
      next_frame.base_line_height,
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
    reset_wrap_exclusions_for_y(current, y, wrap_exclusions);
    let next_frame = TextFrame::new(self.paragraph, next_flow);
    let y = dodge_text_wrap_exclusions(y, next_frame.base_line_height, wrap_exclusions);
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
    let mut wrap_exclusions = current
      .wrap_exclusions
      .iter()
      .copied()
      .filter(|exclusion| exclusion.bottom_pt > y)
      .collect::<Vec<_>>();
    let mut emitted = paragraph.list_label.is_some();
    let mut pending_tab: Option<ResolvedTabStop> = None;
    let mut text_state = TextFrameState::new();
    let line = LineFrame::first(text_frame, y, paragraph.list_label.is_some());
    y = line.y_pt;
    let vml_anchor_line_height =
      libreoffice_empty_paragraph_first_line_height(&paragraph_base_line_style(paragraph));
    let mut base_line_height = text_frame.base_line_height;
    let mut line_height = line.height_pt;
    y = dodge_text_wrap_exclusions(y, line_height, &wrap_exclusions);
    let (mut line_left, mut line_right) =
      self.line_bounds(text_frame, y, line_height, &wrap_exclusions);
    if paragraph.list_label.is_some() {
      line_left = line.left_pt.max(line_left);
    }
    let mut x = line.x_pt.max(line_left);
    if let Some(label) = &paragraph.list_label {
      let blank_list_label = label.chars().all(char::is_whitespace);
      let mut list_label_style = paragraph.list_label_style.clone();
      if blank_list_label && label.starts_with(' ') {
        let base_line_style = paragraph_base_line_style(paragraph);
        list_label_style.font_size_pt = list_label_style
          .font_size_pt
          .max(base_line_style.font_size_pt);
      }
      if let Some(highlight) = paragraph.list_label_style.highlight {
        let highlight_left = flow.content_left_pt.min(first_line_left);
        let highlight_right = default_line_left.max(first_line_left);
        if highlight_right > highlight_left {
          current.items.push(PageItem::Rect(RectItem {
            x_pt: highlight_left,
            y_pt: y,
            width_pt: highlight_right - highlight_left,
            height_pt: line_height,
            fill_color: Some(highlight),
            fill_opacity: 1.0,
            stroke: None,
            stroke_opacity: 1.0,
          }));
        }
      }
      current.items.push(PageItem::Text(TextItem {
        x_pt: if blank_list_label {
          default_line_left
        } else {
          first_line_left
        },
        y_pt: y,
        line_height_pt: line_height,
        text: label.clone(),
        style: list_label_style,
        hyperlink_url: paragraph.list_label_hyperlink_url.clone(),
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        preserve_text_portion: false,
        form_widget_id: None,
        paragraph_bidi: false,
        decoration_span_start_x_pt: None,
        pdf_text_segmentation: PdfTextSegmentation::Line,
      }));
      x = default_line_left;
      if blank_list_label && let Some(tab_stop_pt) = paragraph.list_label_tab_stop_pt {
        x = x.max(flow.content_left_pt + tab_stop_pt);
      }
    }
    let mut line_item_start_index = current.items.len();
    let justify_wrapped_lines =
      paragraph.format.alignment == ParagraphAlignment::Justify && paragraph.list_label.is_none();
    let mut active_form_widget_ids = Vec::new();
    let mut line_has_form_widget = false;
    let mut tab_over_margin_active = false;

    for (inline_index, item) in paragraph.inlines.iter().enumerate() {
      match item {
        InlineItem::Text(run) => {
          let mut chunk = String::new();
          let mut chunk_x = x;
          let meta = TextChunkMeta {
            hyperlink_url: run.hyperlink_url.as_deref(),
            dynamic_field: run.dynamic_field.clone(),
            style_ref_keys: if run.style_ref_keys.is_empty() {
              paragraph.style_ref_keys.clone()
            } else {
              run.style_ref_keys.clone()
            },
            style_ref_text: run
              .style_ref_text
              .clone()
              .or_else(|| paragraph.style_ref_text.clone()),
            form_widget_id: active_form_widget_ids.last().copied(),
            paragraph_bidi: paragraph.format.bidi,
            preserve_text_portion: run.preserve_text_portion,
            segmentation: flow.text_segmentation,
          };

          let segments = if flow.text_segmentation == TextSegmentation::DrawingLayer {
            drawing_layer_text_segments_with_offsets(&run.text)
          } else {
            text_segments_with_offsets(&run.text)
          };
          for segment in segments {
            if segment.text == "\n" {
              text_state.set_position(InlineCursor {
                inline_index,
                text_offset: segment.end,
              });
              flush_text(
                current,
                TextPlacement {
                  x_pt: chunk_x,
                  y_pt: y,
                  line_height_pt: line_height,
                },
                &mut chunk,
                &run.style,
                meta.clone(),
              );
              (flow, text_frame, y, line_left, line_right) = self.advance_line(
                TextLineAdvance {
                  current,
                  pages,
                  wrap_exclusions: &mut wrap_exclusions,
                  state: &mut text_state,
                  active: ActiveTextFrame {
                    flow,
                    frame: text_frame,
                  },
                  line_left,
                  line_right,
                  justify: false,
                  line_item_start_index: &mut line_item_start_index,
                  line_has_form_widget: &mut line_has_form_widget,
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
              tab_over_margin_active = false;
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
                TextPlacement {
                  x_pt: chunk_x,
                  y_pt: y,
                  line_height_pt: line_height,
                },
                &mut chunk,
                &run.style,
                meta.clone(),
              );
              let tab_stop = next_tab_stop(
                x,
                line_left,
                &paragraph.format.tab_stops,
                flow.default_tab_stop_pt,
              );
              x = tab_stop.x_pt;
              if x > line_right {
                tab_over_margin_active = true;
              }
              chunk_x = x;
              pending_tab = Some(tab_stop);
              emitted = true;
              continue;
            }

            let width = measure_text(&segment.text, &run.style);
            let line_capacity = (line_right - line_left).max(DEFAULT_FONT_SIZE_PT);
            let whitespace = segment.text.chars().all(char::is_whitespace);
            if let Some(tab_stop) = pending_tab.take()
              && !whitespace
            {
              x = aligned_tab_x(tab_stop, width, line_left, line_right);
              chunk_x = x;
              tab_over_margin_active |= tab_stop.x_pt > line_right;
            }

            if x + width > line_right && x > line_left && !tab_over_margin_active {
              flush_text(
                current,
                TextPlacement {
                  x_pt: chunk_x,
                  y_pt: y,
                  line_height_pt: line_height,
                },
                &mut chunk,
                &run.style,
                meta.clone(),
              );
              (flow, text_frame, y, line_left, line_right) = self.advance_line(
                TextLineAdvance {
                  current,
                  pages,
                  wrap_exclusions: &mut wrap_exclusions,
                  state: &mut text_state,
                  active: ActiveTextFrame {
                    flow,
                    frame: text_frame,
                  },
                  line_left,
                  line_right,
                  justify: justify_wrapped_lines,
                  line_item_start_index: &mut line_item_start_index,
                  line_has_form_widget: &mut line_has_form_widget,
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
              tab_over_margin_active = false;
              if whitespace {
                emitted = true;
                continue;
              }
            }

            if width > line_capacity && x <= line_left && !whitespace {
              let mut text_offset = segment.start;
              for text in emergency_break_segments(&segment.text) {
                let width = measure_text(&text, &run.style);
                if width > line_capacity && text.chars().count() > 1 {
                  for ch in text.chars() {
                    let mut encoded = [0; 4];
                    let text = ch.encode_utf8(&mut encoded);
                    let width = measure_text(text, &run.style);
                    text_offset += text.len();

                    if x + width > line_right && x > line_left {
                      flush_text(
                        current,
                        TextPlacement {
                          x_pt: chunk_x,
                          y_pt: y,
                          line_height_pt: line_height,
                        },
                        &mut chunk,
                        &run.style,
                        meta.clone(),
                      );
                      (flow, text_frame, y, line_left, line_right) = self.advance_line(
                        TextLineAdvance {
                          current,
                          pages,
                          wrap_exclusions: &mut wrap_exclusions,
                          state: &mut text_state,
                          active: ActiveTextFrame {
                            flow,
                            frame: text_frame,
                          },
                          line_left,
                          line_right,
                          justify: justify_wrapped_lines,
                          line_item_start_index: &mut line_item_start_index,
                          line_has_form_widget: &mut line_has_form_widget,
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
                    if segment_affects_line_height(text) {
                      line_height = include_text_height(line_height, text_frame, &run.style);
                    }
                    line_has_form_widget |= meta.form_widget_id.is_some();
                    emitted = true;
                  }
                  continue;
                }
                text_offset += text.len();

                if x + width > line_right && x > line_left {
                  flush_text(
                    current,
                    TextPlacement {
                      x_pt: chunk_x,
                      y_pt: y,
                      line_height_pt: line_height,
                    },
                    &mut chunk,
                    &run.style,
                    meta.clone(),
                  );
                  (flow, text_frame, y, line_left, line_right) = self.advance_line(
                    TextLineAdvance {
                      current,
                      pages,
                      wrap_exclusions: &mut wrap_exclusions,
                      state: &mut text_state,
                      active: ActiveTextFrame {
                        flow,
                        frame: text_frame,
                      },
                      line_left,
                      line_right,
                      justify: justify_wrapped_lines,
                      line_item_start_index: &mut line_item_start_index,
                      line_has_form_widget: &mut line_has_form_widget,
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
                if segment_affects_line_height(&text) {
                  line_height = include_text_height(line_height, text_frame, &run.style);
                }
                line_has_form_widget |= meta.form_widget_id.is_some();
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
            if segment_affects_line_height(&segment.text) {
              line_height = include_text_height(line_height, text_frame, &run.style);
            }
            line_has_form_widget |= meta.form_widget_id.is_some();
            emitted = true;
            tab_over_margin_active = false;
            if flow.text_segmentation == TextSegmentation::DrawingLayer {
              flush_text(
                current,
                TextPlacement {
                  x_pt: chunk_x,
                  y_pt: y,
                  line_height_pt: line_height,
                },
                &mut chunk,
                &run.style,
                meta.clone(),
              );
            }
          }

          flush_text(
            current,
            TextPlacement {
              x_pt: chunk_x,
              y_pt: y,
              line_height_pt: line_height,
            },
            &mut chunk,
            &run.style,
            meta.clone(),
          );
        }
        InlineItem::FormWidgetStart(widget_id) => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          pending_tab = None;
          active_form_widget_ids.push(*widget_id);
        }
        InlineItem::FormWidgetEnd(widget_id) => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          pending_tab = None;
          if let Some(position) = active_form_widget_ids
            .iter()
            .rposition(|active_widget_id| active_widget_id == widget_id)
          {
            active_form_widget_ids.truncate(position);
          }
        }
        InlineItem::LastRenderedPageBreak => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          pending_tab = None;
        }
        InlineItem::Image(image) => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          pending_tab = None;
          if let crate::docx::ImagePlacement::Floating(placement) = image.placement {
            let frame_width =
              relative_floating_width(placement, flow).unwrap_or_else(|| image_frame_width(image));
            let frame_height = relative_floating_height(placement, flow)
              .unwrap_or_else(|| image_frame_height(image));
            let (width, height) = fit_image_to_line(frame_width, frame_height, flow.content_width);
            let (image_x, image_y) = floating_image_position(placement, flow, x, y, width, height);
            let image_item_start = current.items.len();
            current.items.push(PageItem::Image(ImageItem {
              x_pt: image_x,
              y_pt: image_y,
              width_pt: width,
              height_pt: height,
              crop: image.crop,
              rotation_deg: image.rotation_deg,
              flip_horizontal: image.flip_horizontal,
              flip_vertical: image.flip_vertical,
              data: image.data.clone(),
              content_type: image.content_type.clone(),
              alt_text: image.alt_text.clone(),
              hyperlink_url: image.hyperlink_url.clone(),
              floating: true,
              behind_text: placement.behind_text,
            }));
            if placement.layout_in_cell && flow.text_segmentation == TextSegmentation::TableCell {
              current.items.push(PageItem::Rect(RectItem {
                x_pt: image_x,
                y_pt: image_y,
                width_pt: width,
                height_pt: height,
                fill_color: None,
                fill_opacity: 1.0,
                stroke: Some(BorderStyle::default()),
                stroke_opacity: 0.0,
              }));
            }
            let influence_bounds = Some(FrameBounds {
              x_pt: image_x - placement.margin_left_pt,
              y_pt: image_y - placement.margin_top_pt,
              width_pt: width + placement.margin_left_pt + placement.margin_right_pt,
              height_pt: height + placement.margin_top_pt + placement.margin_bottom_pt,
            });
            match placement.wrap {
              ImageWrapMode::TopBottom | ImageWrapMode::None => {
                if !placement.behind_text {
                  append_vertical_wrap_exclusion(
                    current,
                    flow,
                    image_x - placement.margin_left_pt,
                    image_y - placement.margin_top_pt,
                    image_x + width + placement.margin_right_pt,
                    image_y + height + placement.margin_bottom_pt,
                  );
                  reset_wrap_exclusions_for_y(current, y, &mut wrap_exclusions);
                  push_page_influence(
                    current,
                    FrameInfluenceKind::FlyWrap,
                    image_item_start,
                    current.items.len(),
                    influence_bounds,
                  );
                  y = y.max(image_y + height + placement.margin_bottom_pt);
                  if y + base_line_height > flow.content_bottom && !current.items.is_empty() {
                    (flow, y) = advance_section_flow(flow, current, pages);
                    text_frame = TextFrame::new(self.paragraph, flow);
                    text_state.note_page_follow(pages.len(), y);
                    reset_wrap_exclusions_for_y(current, y, &mut wrap_exclusions);
                    default_line_right = text_frame.default_line_right;
                    paragraph_left = text_frame.paragraph_left;
                    base_line_height = text_frame.base_line_height;
                    line_height = base_line_height;
                    line_item_start_index = current.items.len();
                  }
                }
                (line_left, line_right) =
                  self.line_bounds(text_frame, y, line_height, &wrap_exclusions);
                x = line_left;
                line_height = base_line_height;
              }
              ImageWrapMode::Square | ImageWrapMode::Tight if !placement.behind_text => {
                let exclusion = WrapExclusion {
                  left_pt: image_x - placement.margin_left_pt,
                  right_pt: image_x + width + placement.margin_right_pt,
                  top_pt: image_y - placement.margin_top_pt,
                  bottom_pt: image_y + height + placement.margin_bottom_pt,
                  side: placement.wrap_side,
                  blocks_flow: false,
                };
                wrap_exclusions.push(exclusion);
                current.wrap_exclusions.push(exclusion);
                push_page_influence(
                  current,
                  FrameInfluenceKind::FlyWrap,
                  image_item_start,
                  current.items.len(),
                  influence_bounds,
                );
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
          let (width, height) = fit_image_to_line(
            visible_image_width(image),
            visible_image_height(image),
            flow.content_width,
          );
          if x + width > line_right && x > line_left {
            (flow, text_frame, y, line_left, line_right) = self.advance_line(
              TextLineAdvance {
                current,
                pages,
                wrap_exclusions: &mut wrap_exclusions,
                state: &mut text_state,
                active: ActiveTextFrame {
                  flow,
                  frame: text_frame,
                },
                line_left,
                line_right,
                justify: justify_wrapped_lines,
                line_item_start_index: &mut line_item_start_index,
                line_has_form_widget: &mut line_has_form_widget,
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
            crop: image.crop,
            rotation_deg: image.rotation_deg,
            flip_horizontal: image.flip_horizontal,
            flip_vertical: image.flip_vertical,
            data: image.data.clone(),
            content_type: image.content_type.clone(),
            alt_text: image.alt_text.clone(),
            hyperlink_url: image.hyperlink_url.clone(),
            floating: false,
            behind_text: false,
          }));
          if flow.text_segmentation == TextSegmentation::Notes {
            current.items.push(PageItem::Rect(RectItem {
              x_pt: x,
              y_pt: y,
              width_pt: width,
              height_pt: height,
              fill_color: None,
              fill_opacity: 1.0,
              stroke: Some(BorderStyle::default()),
              stroke_opacity: 0.0,
            }));
          }
          x += width;
          line_height = line_height.max(height);
          emitted = true;
        }
        InlineItem::Shape(shape) => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          pending_tab = None;
          let place_shape = |current: &mut Page,
                             shape_flow: FlowContext,
                             x_pt: f32,
                             y_pt: f32,
                             width_pt: f32,
                             height_pt: f32| {
            let item_start = current.items.len();
            if let Some(fill_image) = &shape.fill_image {
              current.items.push(PageItem::Image(ImageItem {
                x_pt,
                y_pt,
                width_pt,
                height_pt,
                crop: fill_image.crop,
                rotation_deg: fill_image.rotation_deg,
                flip_horizontal: fill_image.flip_horizontal,
                flip_vertical: fill_image.flip_vertical,
                data: fill_image.data.clone(),
                content_type: fill_image.content_type.clone(),
                alt_text: None,
                hyperlink_url: None,
                floating: matches!(shape.placement, crate::docx::ImagePlacement::Floating(_)),
                behind_text: false,
              }));
            }
            if shape.geometry == InlineShapeGeometry::Line
              && shape.fill_color.is_none()
              && let Some(stroke) = shape.stroke
            {
              push_styled_line(
                current,
                x_pt,
                y_pt,
                x_pt + width_pt,
                y_pt + height_pt,
                stroke,
              );
              return (item_start, current.items.len());
            }
            if let InlineShapeGeometry::Polyline { points, closed } = &shape.geometry {
              current.items.push(PageItem::Polyline(PolylineItem {
                x_pt,
                y_pt,
                width_pt,
                height_pt,
                points: points.clone(),
                closed: *closed,
                fill_color: shape.fill_color,
                stroke: shape.stroke,
              }));
              for color in &shape.additional_fill_colors {
                current.items.push(PageItem::Polyline(PolylineItem {
                  x_pt,
                  y_pt,
                  width_pt,
                  height_pt,
                  points: points.clone(),
                  closed: *closed,
                  fill_color: Some(*color),
                  stroke: None,
                }));
              }
              layout_shape_text_box(current, shape_flow, shape, x_pt, y_pt, width_pt, height_pt);
              return (item_start, current.items.len());
            }
            if shape.fill_color.is_some() || shape.stroke.is_some() {
              current.items.push(PageItem::Rect(RectItem {
                x_pt,
                y_pt,
                width_pt,
                height_pt,
                fill_color: shape.fill_color,
                fill_opacity: 1.0,
                stroke: shape.stroke,
                stroke_opacity: 1.0,
              }));
            }
            for color in &shape.additional_fill_colors {
              current.items.push(PageItem::Rect(RectItem {
                x_pt,
                y_pt,
                width_pt,
                height_pt,
                fill_color: Some(*color),
                fill_opacity: 1.0,
                stroke: None,
                stroke_opacity: 1.0,
              }));
            }
            if shape_has_invisible_auto_fit_textbox_bounds(shape) {
              // Source: LibreOffice keeps a Writer fly frame for textbox
              // content even when the owning draw shape has no fill/stroke,
              // but that layout-only frame is not painted as a polypolygon.
              current.items.push(PageItem::Rect(RectItem {
                x_pt,
                y_pt,
                width_pt,
                height_pt,
                fill_color: None,
                fill_opacity: 1.0,
                stroke: None,
                stroke_opacity: 0.0,
              }));
            }
            layout_shape_text_box(current, shape_flow, shape, x_pt, y_pt, width_pt, height_pt);
            (item_start, current.items.len())
          };

          match shape.placement {
            crate::docx::ImagePlacement::Floating(placement) => {
              if floating_shape_is_zero_relative_background(placement, shape) {
                continue;
              }
              let width = relative_floating_width(placement, flow).unwrap_or(shape.width_pt);
              let height = relative_floating_height(placement, flow).unwrap_or(shape.height_pt);
              let (shape_x, shape_y) =
                floating_image_position(placement, flow, x, y, width, height);
              let shape_x = shape_x + shape.offset_x_pt;
              let text_anchor_offset = match placement.vertical_relative_to {
                crate::docx::VerticalImageReference::Paragraph
                | crate::docx::VerticalImageReference::Line
                  if shape.allow_outside_page =>
                {
                  vml_anchor_line_height
                }
                crate::docx::VerticalImageReference::TopMargin
                | crate::docx::VerticalImageReference::BottomMargin
                  if placement.vertical_alignment != Some(VerticalImageAlignment::Inside) =>
                {
                  LO_DRAWING_ANCHOR_MARGIN_LINE_HEIGHT_PT
                }
                crate::docx::VerticalImageReference::BottomMargin => line_height,
                _ => 0.0,
              };
              let shape_y = shape_y + shape.offset_y_pt + text_anchor_offset;
              let shape_x = adjusted_floating_shape_x(placement, shape, shape_x);
              let shape_y = adjusted_floating_shape_y(placement, shape, shape_y);
              let allows_outside_page =
                shape.allow_outside_page || floating_shape_may_extend_outside_page(placement);
              let (shape_x, shape_y) = if allows_outside_page {
                (shape_x, shape_y)
              } else {
                keep_floating_shape_inside_page(shape_x, shape_y, width, height, flow)
              };
              let (shape_item_start, shape_item_end) =
                place_shape(current, flow, shape_x, shape_y, width, height);
              if placement.behind_text
                && shape.fill_color.is_none()
                && shape.fill_image.is_none()
                && shape.stroke.is_none()
                && shape.additional_fill_colors.is_empty()
                && !shape.text_box_blocks.is_empty()
              {
                current.items.push(PageItem::Rect(RectItem {
                  x_pt: shape_x,
                  y_pt: y,
                  width_pt: width,
                  height_pt: (shape_y + height - y - BorderStyle::default().width_pt).max(height),
                  fill_color: None,
                  fill_opacity: 1.0,
                  stroke: Some(BorderStyle::default()),
                  stroke_opacity: 0.0,
                }));
              }
              let influence_bounds = Some(FrameBounds {
                x_pt: shape_x - placement.margin_left_pt,
                y_pt: shape_y - placement.margin_top_pt,
                width_pt: width + placement.margin_left_pt + placement.margin_right_pt,
                height_pt: height + placement.margin_top_pt + placement.margin_bottom_pt,
              });
              match placement.wrap {
                ImageWrapMode::TopBottom | ImageWrapMode::None => {
                  if !placement.behind_text {
                    append_vertical_wrap_exclusion(
                      current,
                      flow,
                      shape_x - placement.margin_left_pt,
                      shape_y - placement.margin_top_pt,
                      shape_x + width + placement.margin_right_pt,
                      shape_y + height + placement.margin_bottom_pt,
                    );
                    reset_wrap_exclusions_for_y(current, y, &mut wrap_exclusions);
                    push_page_influence(
                      current,
                      FrameInfluenceKind::FlyWrap,
                      shape_item_start,
                      shape_item_end,
                      influence_bounds,
                    );
                  }
                  y = y.max(shape_y + height + placement.margin_bottom_pt);
                  if y + base_line_height > flow.content_bottom && !current.items.is_empty() {
                    (flow, y) = advance_section_flow(flow, current, pages);
                    text_frame = TextFrame::new(self.paragraph, flow);
                    text_state.note_page_follow(pages.len(), y);
                    reset_wrap_exclusions_for_y(current, y, &mut wrap_exclusions);
                    default_line_right = text_frame.default_line_right;
                    paragraph_left = text_frame.paragraph_left;
                    base_line_height = text_frame.base_line_height;
                    line_height = base_line_height;
                    line_item_start_index = current.items.len();
                  }
                  (line_left, line_right) =
                    self.line_bounds(text_frame, y, line_height, &wrap_exclusions);
                  x = line_left;
                  line_height = base_line_height;
                }
                ImageWrapMode::Square | ImageWrapMode::Tight if !placement.behind_text => {
                  let exclusion = WrapExclusion {
                    left_pt: shape_x - placement.margin_left_pt,
                    right_pt: shape_x + width + placement.margin_right_pt,
                    top_pt: shape_y - placement.margin_top_pt,
                    bottom_pt: shape_y + height + placement.margin_bottom_pt,
                    side: placement.wrap_side,
                    blocks_flow: false,
                  };
                  wrap_exclusions.push(exclusion);
                  current.wrap_exclusions.push(exclusion);
                  push_page_influence(
                    current,
                    FrameInfluenceKind::FlyWrap,
                    shape_item_start,
                    shape_item_end,
                    influence_bounds,
                  );
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
            }
            crate::docx::ImagePlacement::Inline => {
              if x + shape.width_pt > line_right && x > line_left {
                (flow, text_frame, y, line_left, line_right) = self.advance_line(
                  TextLineAdvance {
                    current,
                    pages,
                    wrap_exclusions: &mut wrap_exclusions,
                    state: &mut text_state,
                    active: ActiveTextFrame {
                      flow,
                      frame: text_frame,
                    },
                    line_left,
                    line_right,
                    justify: justify_wrapped_lines,
                    line_item_start_index: &mut line_item_start_index,
                    line_has_form_widget: &mut line_has_form_widget,
                  },
                  y,
                  &mut line_height,
                );
                default_line_right = text_frame.default_line_right;
                paragraph_left = text_frame.paragraph_left;
                base_line_height = text_frame.base_line_height;
                x = line_left;
              }
              let _ = place_shape(
                current,
                flow,
                x + shape.offset_x_pt,
                y + shape.offset_y_pt
                  + if shape.inline_anchor_after_line {
                    base_line_height.max(line_height) * LO_DOCUMENT_DEFAULT_LINE_SPACING_PERCENT
                      / PERCENT_SCALE
                  } else {
                    0.0
                  },
                shape.width_pt,
                shape.height_pt,
              );
              x += shape.width_pt;
              line_height = line_height.max(shape.height_pt);
            }
          }
          emitted = true;
        }
        InlineItem::PageBreak => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          text_state.finish_line(y, line_height);
          (flow, text_frame, y, line_left, line_right, line_height) =
            self.force_text_page_break(flow, current, pages, &mut wrap_exclusions);
          default_line_right = text_frame.default_line_right;
          paragraph_left = text_frame.paragraph_left;
          base_line_height = text_frame.base_line_height;
          x = line_left;
          line_item_start_index = current.items.len();
          line_has_form_widget = false;
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
          line_item_start_index = current.items.len();
          line_has_form_widget = false;
          emitted = column_emitted;
          pending_tab = None;
        }
      }
    }

    let paragraph_bottom;
    if emitted {
      push_page_fragment(
        current,
        PageFragmentRecord {
          kind: FrameFragmentKind::ParagraphLine,
          split: FragmentSplitKind::Complete,
          index: text_state.line_fragments.len(),
          row_index: text_state.line_fragments.len(),
          cell_index: None,
          item_start: line_item_start_index,
          item_end: current.items.len(),
        },
      );
      let real_height = line_real_height(paragraph, line_height, line_has_form_widget);
      text_state.finish_paragraph(y, real_height, emitted);
      paragraph_bottom = y + real_height;
      y = paragraph_bottom + self.spacing_after_pt;
    } else {
      paragraph_bottom = y + base_line_height;
      y = paragraph_bottom + self.spacing_after_pt;
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
      return TextFrameLayout::new(paragraph, follow_flow, self.spacing_after_pt)
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
        flow,
        paragraph_left,
        paragraph_top,
        default_line_right - paragraph_left,
        paragraph_bottom - paragraph_top,
      );
    }

    (flow, y)
  }
}

fn keep_floating_shape_inside_page(
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  flow: FlowContext,
) -> (f32, f32) {
  let stroke_padding_pt = 1.0;
  (
    x_pt.clamp(
      stroke_padding_pt,
      (flow.setup.width_pt - width_pt - stroke_padding_pt).max(stroke_padding_pt),
    ),
    y_pt.clamp(
      stroke_padding_pt,
      (flow.setup.height_pt - height_pt - stroke_padding_pt).max(stroke_padding_pt),
    ),
  )
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

fn drawing_layer_text_segments_with_offsets(text: &str) -> Vec<TextSegment> {
  let mut output = Vec::new();
  let mut start = 0usize;

  for (index, ch) in text.char_indices() {
    if ch == '\n' || ch == '\t' {
      if start < index {
        output.push(TextSegment {
          text: text[start..index].to_string(),
          start,
          end: index,
        });
      }
      let end = index + ch.len_utf8();
      output.push(TextSegment {
        text: ch.to_string(),
        start: index,
        end,
      });
      start = end;
      continue;
    }
    if !ch.is_whitespace() {
      continue;
    }
    if start < index {
      output.push(TextSegment {
        text: text[start..index].to_string(),
        start,
        end: index,
      });
      start = index;
    }
  }

  if start < text.len() {
    output.push(TextSegment {
      text: text[start..].to_string(),
      start,
      end: text.len(),
    });
  }

  output
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
    static LINE_SEGMENTER: LineSegmenterBorrowed<'static> =
      LineSegmenter::new_auto(LineBreakOptions::default());
  }

  LINE_SEGMENTER.with(|segmenter| {
    let mut start = 0;
    for point in segmenter.segment_str(text) {
      if point == 0 {
        continue;
      }
      if start < point {
        push_text_line_segment(&text[start..point], segments);
      }
      start = point;
    }

    if start < text.len() {
      push_text_line_segment(&text[start..], segments);
    }
  });
}

fn push_text_line_segment(text: &str, segments: &mut Vec<String>) {
  if segments.last().is_some_and(|segment| {
    segment
      .chars()
      .all(libreoffice_forbidden_line_end_before_text)
  }) {
    let previous = segments.last_mut().unwrap();
    previous.push_str(text);
  } else {
    segments.push(text.to_string());
  }
}

fn libreoffice_forbidden_line_end_before_text(ch: char) -> bool {
  // Source: LibreOffice uses i18npool breakiterator forbidden end-character
  // handling, so opening quotation marks don't remain alone at line end.
  matches!(ch, '“' | '‘')
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
    .find(|stop| stop.position_pt > relative_x + LAYOUT_EPSILON_PT)
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
  _line_right: f32,
) -> f32 {
  let x = match tab_stop.alignment {
    TabStopAlignment::Left => tab_stop.x_pt,
    TabStopAlignment::Center => tab_stop.x_pt - text_width / 2.0,
    TabStopAlignment::Right => tab_stop.x_pt - text_width,
  };
  x.max(line_left)
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
    if exclusion.blocks_flow {
      continue;
    }
    if y + line_height <= exclusion.top_pt || y >= exclusion.bottom_pt {
      continue;
    }
    if exclusion.right_pt <= default_left || exclusion.left_pt >= default_right {
      continue;
    }

    let exclude_left = exclusion.left_pt.max(default_left);
    let exclude_right = exclusion.right_pt.min(default_right);
    match exclusion.side {
      ImageWrapSide::Left => {
        right = right.min(exclude_left);
      }
      ImageWrapSide::Right => {
        left = left.max(exclude_right);
      }
      ImageWrapSide::BothSides | ImageWrapSide::Largest => {
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
    }
  }

  if right - left < DEFAULT_FONT_SIZE_PT {
    (default_left, default_right)
  } else {
    (left, right)
  }
}

fn dodge_text_wrap_exclusions(mut y: f32, line_height: f32, exclusions: &[WrapExclusion]) -> f32 {
  loop {
    let Some(exclusion) = exclusions
      .iter()
      .filter(|exclusion| {
        exclusion.blocks_flow && exclusion.overlaps_vertical_span(y, y + line_height)
      })
      .min_by(|a, b| {
        a.bottom_pt
          .partial_cmp(&b.bottom_pt)
          .unwrap_or(std::cmp::Ordering::Equal)
      })
    else {
      return y;
    };
    if exclusion.bottom_pt <= y + LAYOUT_EPSILON_PT {
      return y;
    }
    y = exclusion.bottom_pt;
  }
}

fn decorate_paragraph(
  page: &mut Page,
  start_item_index: usize,
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
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
    let (x, y, width, height) =
      table_cell_paragraph_shading_bounds(flow).unwrap_or((x, y, width, height));
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

fn table_cell_paragraph_shading_bounds(flow: FlowContext) -> Option<(f32, f32, f32, f32)> {
  if !matches!(flow.text_segmentation, TextSegmentation::TableCell) {
    return None;
  }
  let bounds = flow.layout_cell_bounds?;
  (bounds.height_pt > LAYOUT_EPSILON_PT).then_some((
    bounds.x_pt,
    bounds.y_pt,
    bounds.width_pt,
    bounds.height_pt,
  ))
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

fn justify_line_items(
  items: &mut Vec<PageItem>,
  start_index: usize,
  y: f32,
  line_left: f32,
  line_right: f32,
) {
  // Source: LibreOffice vcl/source/pdf/pdfwriter_impl.cxx::drawLayout()
  // writes one PDF text object for a laid-out line and carries justification
  // through glyph positioning / text arrays. Splitting a justified Writer line
  // into per-word text objects changes PDFium object counts and does not match
  // Writer's export model.
  let _ = (items, start_index, y, line_left, line_right);
}

fn item_y(item: &PageItem) -> Option<f32> {
  match item {
    PageItem::Text(text) => Some(text.y_pt),
    PageItem::Image(image) => Some(image.y_pt),
    PageItem::Rect(rect) => Some(rect.y_pt),
    PageItem::Fill(_) => None,
    PageItem::Line(_) => None,
    PageItem::Polyline(_) => None,
  }
}

fn item_horizontal_bounds(item: &PageItem) -> Option<(f32, f32)> {
  match item {
    PageItem::Text(text) => Some((text.x_pt, measure_text(&text.text, &text.style))),
    PageItem::Image(image) => Some((image.x_pt, image.width_pt)),
    PageItem::Rect(rect) => Some((rect.x_pt, rect.width_pt)),
    PageItem::Fill(_) => None,
    PageItem::Line(_) => None,
    PageItem::Polyline(_) => None,
  }
}

fn shift_item_x(item: &mut PageItem, offset: f32) {
  match item {
    PageItem::Text(text) => text.x_pt += offset,
    PageItem::Image(image) => image.x_pt += offset,
    PageItem::Rect(rect) => rect.x_pt += offset,
    PageItem::Fill(_) => {}
    PageItem::Line(_) => {}
    PageItem::Polyline(_) => {}
  }
}

fn shift_item(item: &mut PageItem, dx: f32, dy: f32) {
  match item {
    PageItem::Text(text) => {
      text.x_pt += dx;
      text.y_pt += dy;
    }
    PageItem::Image(image) => {
      image.x_pt += dx;
      image.y_pt += dy;
    }
    PageItem::Rect(rect) => {
      rect.x_pt += dx;
      rect.y_pt += dy;
    }
    PageItem::Fill(_) | PageItem::Line(_) | PageItem::Polyline(_) => {}
  }
}

fn fit_image_to_line(width: f32, height: f32, max_width: f32) -> (f32, f32) {
  if width <= f32::EPSILON || height <= f32::EPSILON || max_width <= f32::EPSILON {
    return (0.0, 0.0);
  }
  if width <= max_width {
    (width, height)
  } else {
    let scale = max_width / width;
    (width * scale, height * scale)
  }
}

fn image_frame_width(image: &crate::docx::InlineImage) -> f32 {
  (image.width_pt + image.effect_left_pt + image.effect_right_pt).max(0.0)
}

fn image_frame_height(image: &crate::docx::InlineImage) -> f32 {
  (image.height_pt + image.effect_top_pt + image.effect_bottom_pt).max(0.0)
}

fn visible_image_width(image: &crate::docx::InlineImage) -> f32 {
  image.width_pt.max(0.0)
}

fn visible_image_height(image: &crate::docx::InlineImage) -> f32 {
  image.height_pt.max(0.0)
}

fn force_page_break(
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
) -> (FlowContext, f32) {
  // Source: Writer treats an explicit text page break as a real page
  // transition even when the current page has no painted objects yet; see
  // sw/qa/core/text/itrform2.cxx:testContentControlHeaderPDFExport.
  let mut next_flow = FlowContext {
    section_page_index: flow.section_page_index + 1,
    ..flow
  };
  let next_page_number = pages.len() + 2;
  let mut next_page =
    empty_section_page(flow.setup, flow.section_index, next_flow.section_page_index);
  next_page.repeating_wrap_exclusion_catalog = current.repeating_wrap_exclusion_catalog.clone();
  next_page.repeating_wrap_exclusions = next_page
    .repeating_wrap_exclusion_catalog
    .selected(next_flow.section_page_index, next_page_number)
    .to_vec();
  extend_wrap_exclusions_unique(
    &mut next_page.wrap_exclusions,
    &next_page.repeating_wrap_exclusions,
  );
  next_page.preserve_empty = true;
  pages.push(std::mem::replace(current, next_page));
  activate_pending_floating_table_follows_for_current(current, pages);
  next_flow = body_flow_for_page(flow_with_column(next_flow, 0), pages.len() + 1);
  (next_flow, next_flow.content_top_pt)
}

#[derive(Clone, Copy, Debug)]
struct TextPlacement {
  x_pt: f32,
  y_pt: f32,
  line_height_pt: f32,
}

#[derive(Clone, Debug)]
struct TextChunkMeta<'a> {
  hyperlink_url: Option<&'a str>,
  dynamic_field: Option<DynamicFieldKind>,
  style_ref_keys: Vec<Arc<str>>,
  style_ref_text: Option<Arc<str>>,
  form_widget_id: Option<u32>,
  paragraph_bidi: bool,
  preserve_text_portion: bool,
  segmentation: TextSegmentation,
}

fn flush_text(
  page: &mut Page,
  placement: TextPlacement,
  chunk: &mut String,
  style: &TextStyle,
  meta: TextChunkMeta<'_>,
) {
  if chunk.is_empty() {
    return;
  }

  page.items.push(PageItem::Text(TextItem {
    x_pt: placement.x_pt,
    y_pt: placement.y_pt,
    line_height_pt: placement.line_height_pt,
    text: std::mem::take(chunk),
    style: style.clone(),
    hyperlink_url: meta.hyperlink_url.map(ToString::to_string),
    dynamic_field: meta.dynamic_field,
    style_ref_keys: meta.style_ref_keys,
    style_ref_text: meta.style_ref_text,
    form_widget_id: meta.form_widget_id,
    paragraph_bidi: meta.paragraph_bidi,
    preserve_text_portion: meta.preserve_text_portion,
    decoration_span_start_x_pt: None,
    pdf_text_segmentation: match meta.segmentation {
      TextSegmentation::Body => PdfTextSegmentation::Line,
      TextSegmentation::RepeatingSlot => PdfTextSegmentation::Line,
      TextSegmentation::TableCell => PdfTextSegmentation::Line,
      TextSegmentation::DrawingLayer => PdfTextSegmentation::Portion,
      TextSegmentation::Notes => PdfTextSegmentation::Line,
    },
  }));
}

fn push_styled_line(page: &mut Page, x1: f32, y1: f32, x2: f32, y2: f32, border: BorderStyle) {
  if border.compound {
    let stroke_width = border.width_pt / 3.0;
    if stroke_width <= f32::EPSILON {
      return;
    }
    let offset = stroke_width;
    if f32::abs(y2 - y1) < f32::abs(x2 - x1) {
      push_line_item(
        page,
        x1,
        y1 - offset,
        x2,
        y2 - offset,
        stroke_width,
        border.color,
      );
      push_line_item(
        page,
        x1,
        y1 + offset,
        x2,
        y2 + offset,
        stroke_width,
        border.color,
      );
    } else {
      push_line_item(
        page,
        x1 - offset,
        y1,
        x2 - offset,
        y2,
        stroke_width,
        border.color,
      );
      push_line_item(
        page,
        x1 + offset,
        y1,
        x2 + offset,
        y2,
        stroke_width,
        border.color,
      );
    }
    return;
  }
  push_line_item(page, x1, y1, x2, y2, border.width_pt, border.color);
}

fn push_line_item(
  page: &mut Page,
  x1: f32,
  y1: f32,
  x2: f32,
  y2: f32,
  width: f32,
  color: RgbColor,
) {
  page.items.push(PageItem::Line(LineItem {
    x1_pt: x1,
    y1_pt: y1,
    x2_pt: x2,
    y2_pt: y2,
    width_pt: width,
    color,
    kind: LineItemKind::Stroke,
  }));
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::{CellBordersModel, CellMargins, Paragraph, ParagraphFormat, TextRun};

  #[test]
  fn compound_horizontal_border_paints_two_parallel_strokes() {
    let mut page = empty_page(PageSetup::default(), 0);
    push_styled_line(
      &mut page,
      10.0,
      20.0,
      50.0,
      20.0,
      BorderStyle {
        width_pt: 6.0,
        spacing_pt: 0.0,
        color: RgbColor::default(),
        compound: true,
      },
    );

    let lines: Vec<_> = page
      .items
      .iter()
      .filter_map(|item| match item {
        PageItem::Line(line) => Some(line),
        _ => None,
      })
      .collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].width_pt, 2.0);
    assert_eq!(lines[1].width_pt, 2.0);
    assert_eq!(lines[0].y1_pt, 18.0);
    assert_eq!(lines[1].y1_pt, 22.0);
  }

  #[test]
  fn compound_vertical_border_paints_two_parallel_strokes() {
    let mut page = empty_page(PageSetup::default(), 0);
    push_styled_line(
      &mut page,
      10.0,
      20.0,
      10.0,
      60.0,
      BorderStyle {
        width_pt: 6.0,
        spacing_pt: 0.0,
        color: RgbColor::default(),
        compound: true,
      },
    );

    let lines: Vec<_> = page
      .items
      .iter()
      .filter_map(|item| match item {
        PageItem::Line(line) => Some(line),
        _ => None,
      })
      .collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].width_pt, 2.0);
    assert_eq!(lines[1].width_pt, 2.0);
    assert_eq!(lines[0].x1_pt, 8.0);
    assert_eq!(lines[1].x1_pt, 12.0);
  }

  #[test]
  fn table_cell_line_spacing_before_border_matches_writer_compat_extra() {
    let mut paragraph = Paragraph {
      inlines: vec![InlineItem::Text(TextRun {
        text: "double spaced".into(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        preserve_text_portion: false,
      })],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: TextStyle::default(),
      #[cfg(test)]
      runs: Vec::new(),
      format: Box::new(ParagraphFormat {
        line_height_rule: LineHeightRule::Auto,
        line_height_pt: Some(2.0),
        ..Default::default()
      }),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    };

    assert!((table_cell_line_spacing_before_border(&paragraph) - 12.65).abs() < 0.01);
    paragraph.format.line_height_pt = Some(1.0);
    assert_eq!(table_cell_line_spacing_before_border(&paragraph), 0.0);
  }

  #[test]
  fn body_limits_reserve_measured_header_footer_height() {
    let setup = PageSetup::default();
    let slots = RepeatingSlotState {
      default_header: true,
      default_footer: true,
      default_header_height_pt: 20.0,
      default_footer_height_pt: 30.0,
      ..Default::default()
    };

    let (top, bottom) = body_content_limits_for_page(setup, slots, 1, 0);

    assert_eq!(top, 56.0);
    assert_eq!(bottom, 690.0);
  }

  #[test]
  fn negative_header_footer_margins_do_not_reserve_body_space() {
    let setup = PageSetup {
      margin_top_pt: 56.7,
      margin_bottom_pt: 62.35,
      top_margin_was_negative: true,
      bottom_margin_was_negative: true,
      ..Default::default()
    };
    let slots = RepeatingSlotState {
      default_header: true,
      default_footer: true,
      ..Default::default()
    };

    let (top, bottom) = body_content_limits_for_page(setup, slots, 1, 0);

    assert_eq!(top, setup.margin_top_pt);
    assert_eq!(bottom, setup.height_pt - setup.margin_bottom_pt);
  }

  #[test]
  fn footnote_reservation_uses_measured_wrapped_note_height() {
    let flow = flow_context(
      PageSetup {
        width_pt: 120.0,
        height_pt: 400.0,
        margin_left_pt: 10.0,
        margin_right_pt: 10.0,
        margin_top_pt: 10.0,
        margin_bottom_pt: 10.0,
        ..Default::default()
      },
      0,
      SectionColumns::default(),
      0,
      DEFAULT_TAB_STOP_PT,
    );
    let run = TextRun {
      text: "A long footnote body wraps into several lines when measured.".into(),
      style: TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      preserve_text_portion: false,
    };
    let blocks = vec![Block::Paragraph(Paragraph {
      inlines: vec![InlineItem::Text(run.clone())],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: TextStyle::default(),
      #[cfg(test)]
      runs: vec![run],
      format: Box::new(ParagraphFormat::default()),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    })];

    let measured = measured_note_blocks_height(&blocks, flow);
    let estimated = blocks
      .iter()
      .map(|block| estimated_block_height(block, flow))
      .sum::<f32>();

    assert!(measured > DEFAULT_LINE_HEIGHT_PT);
    assert!((measured - estimated).abs() < DEFAULT_LINE_HEIGHT_PT * 2.0);
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
  fn adjacent_border_priority_matches_writer_width_then_compound_rule() {
    fn border(width_pt: f32, compound: bool) -> BorderStyle {
      BorderStyle {
        width_pt,
        compound,
        ..Default::default()
      }
    }

    assert_eq!(
      stronger_border(Some(border(0.5, false)), Some(border(2.0, true))).unwrap(),
      border(2.0, true)
    );
    assert_eq!(
      stronger_border(Some(border(1.0, true)), Some(border(1.0, false))).unwrap(),
      border(1.0, false)
    );
    assert_eq!(
      stronger_border(Some(border(1.0, false)), Some(border(1.0, true))).unwrap(),
      border(1.0, false)
    );
  }

  #[test]
  fn vertical_merge_origin_skips_intermediate_continuations() {
    fn cell(continue_merge: bool, color: Option<RgbColor>) -> TableCell {
      TableCell {
        blocks: Vec::new(),
        shading: color,
        borders: CellBordersModel::default(),
        margins: CellMargins::default(),
        preferred_width_pt: None,
        preferred_width_pct: None,
        grid_span: 1,
        vertical_merge_continue: continue_merge,
        vertical_alignment: TableCellVerticalAlignment::Top,
      }
    }

    let origin_color = RgbColor {
      r: 0xAA,
      g: 0xBB,
      b: 0xCC,
    };
    let table = Table {
      column_widths_pt: vec![72.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: TableAlignment::Left,
      placement: None,
      split_allowed: false,
      following_text_flow: false,
      starts_after_last_rendered_page_break: false,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![
        TableRow {
          cells: vec![cell(false, Some(origin_color))],
          height_pt: None,
          exact_height: false,
          repeat_header: false,
          keep_with_next: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
          redline_color: None,
        },
        TableRow {
          cells: vec![cell(true, None)],
          height_pt: None,
          exact_height: false,
          repeat_header: false,
          keep_with_next: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
          redline_color: None,
        },
        TableRow {
          cells: vec![cell(true, None)],
          height_pt: None,
          exact_height: false,
          repeat_header: false,
          keep_with_next: false,
          cant_split: false,
          cell_spacing_pt: None,
          grid_before: 0,
          grid_after: 0,
          redline_color: None,
        },
      ],
    };

    assert_eq!(
      vertical_merge_origin_cell(&table, 2, 0).and_then(|cell| cell.shading),
      Some(origin_color)
    );
  }

  #[test]
  fn vertical_merge_origin_content_height_spans_continuation_rows() {
    fn cell(continue_merge: bool) -> TableCell {
      TableCell {
        blocks: Vec::new(),
        shading: None,
        borders: CellBordersModel::default(),
        margins: CellMargins::default(),
        preferred_width_pt: None,
        preferred_width_pct: None,
        grid_span: 1,
        vertical_merge_continue: continue_merge,
        vertical_alignment: TableCellVerticalAlignment::Top,
      }
    }

    fn row(continue_merge: bool, height_pt: f32, spacing_pt: Option<f32>) -> TableRow {
      TableRow {
        cells: vec![cell(continue_merge)],
        height_pt: Some(height_pt),
        exact_height: true,
        repeat_header: false,
        keep_with_next: false,
        cant_split: false,
        cell_spacing_pt: spacing_pt,
        grid_before: 0,
        grid_after: 0,
        redline_color: None,
      }
    }

    let table = Table {
      column_widths_pt: vec![72.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: TableAlignment::Left,
      placement: None,
      split_allowed: false,
      following_text_flow: false,
      starts_after_last_rendered_page_break: false,
      borders: None,
      cell_spacing_pt: 2.0,
      rows: vec![
        row(false, 10.0, Some(3.0)),
        row(true, 11.0, None),
        row(true, 12.0, None),
      ],
    };

    assert_eq!(
      vertical_merge_content_height(&table, &[72.0], 0, 0, 10.0),
      Some(38.0)
    );
    assert!(row_has_vertical_merge_context(&table, 0));
    assert!(row_has_vertical_merge_context(&table, 1));
    assert!(row_has_vertical_merge_context(&table, 2));
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
