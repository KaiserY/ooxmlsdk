use std::borrow::Cow;
use std::sync::Arc;

use icu_segmenter::{LineSegmenter, LineSegmenterBorrowed, options::LineBreakOptions};
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use unicode_bidi::{BidiInfo, Level};

use crate::common;
use crate::docx::{
  Block, BorderStyle, DocxDocument, DynamicFieldKind, FieldNumberFormat, FloatingFrame,
  FloatingFramePlacement, FloatingImagePlacement, FrameHeightRule, FrameHorizontalAlignment,
  FrameHorizontalAnchor, FrameVerticalAlignment, FrameVerticalAnchor, FrameWrapMode,
  HorizontalImageAlignment, HorizontalImageReference, ImageCrop, ImageWrapMode, ImageWrapSide,
  InlineChart, InlineItem, InlineShape, InlineShapeGeometry, LineHeightRule, LineNumbering,
  NoteNumberingSpec, PRESERVED_WORD_TEXT_TAB, PageSetup, ParagraphAlignment, RgbColor,
  SectionBreakKind, SectionColumns, TabLeader, TabStop, TabStopAlignment, Table, TableAlignment,
  TableCell, TableCellVerticalAlignment, TableRow, TextBoxVerticalAlignment, TextStyle,
  VerticalImageAlignment, VerticalImageReference,
};
use crate::error::Result;
use crate::fonts::effective_font_size_pt;
use crate::model::{
  common_page_setup, common_point, common_rect, common_rgb, common_stroke_from_border,
  common_text_style,
};
use crate::options::{LayoutActionOptions, LayoutOptions};
use crate::pptx::chart::{
  ChartFrame, ChartLayoutProfile, ClusteredColumnStyle, RadialChartStyle,
  lower_clustered_column_chart, lower_radial_chart,
};
use crate::render::chart as shared_chart;
use crate::text_metrics::TextMetrics;
use crate::units;

// Word document defaults used by LibreOffice import/export are 11pt text,
// 0.5in tab stops, and widow/orphan control of two lines.
const PARAGRAPH_SPACING_AFTER_PT: f32 = 0.0;
const DEFAULT_TAB_STOP_PT: f32 = 36.0;
// Word uses a separate 420-twip grid for literal U+0009 characters preserved
// inside w:t. This is distinct from w:defaultTabStop, whose default is 720
// twips and applies to the semantic w:tab element.
const PRESERVED_WORD_TEXT_TAB_STOP_PT: f32 = 21.0;
const DEFAULT_FONT_SIZE_PT: f32 = 11.0;
const DEFAULT_LINE_HEIGHT_PT: f32 = 14.0;
// Writer frame size, including table rows.
const LO_MIN_FRAME_SIZE_PT: f32 = 23.0 / units::TWIPS_PER_POINT;
const TABLE_ROW_MIN_HEIGHT_PT: f32 = LO_MIN_FRAME_SIZE_PT;
const TABLE_SPACING_AFTER_PT: f32 = 0.0;
const DEFAULT_ORPHAN_LINES: usize = 2;
const DEFAULT_WIDOW_LINES: usize = 2;
const MOVE_BACKWARD_SUPPRESS_THRESHOLD: usize = 20;
const LAYOUT_LOOP_CONTROL_MAX: usize = 10_000;
const UNBOUNDED_LAYOUT_EXTENT_PT: f32 = f32::MAX / 4.0;
const MEASURE_SCRATCH_PAGE_HEIGHT_PT: f32 = UNBOUNDED_LAYOUT_EXTENT_PT;
const LAYOUT_EPSILON_PT: f32 = 0.1;
// sets OOXML document defaults to proportional line spacing 115.
const LO_DOCUMENT_DEFAULT_LINE_SPACING_PERCENT: f32 = 115.0;
const PERCENT_SCALE: f32 = 100.0;
const LO_EMPTY_PARAGRAPH_FIRST_LINE_HEIGHT_PER_FONT_SIZE: f32 = 340.0 / 220.0;
const LO_DRAWING_ANCHOR_MARGIN_LINE_HEIGHT_PT: f32 = 288.0 / units::TWIPS_PER_POINT;
// SwPageFootnoteInfo defaults: line width 10 twips, relative width 25%,
// top/bottom distance 57 twips.
const LO_FOOTNOTE_SEPARATOR_WIDTH_FRACTION: f32 = 0.25;
const LO_FOOTNOTE_SEPARATOR_STROKE_PT: f32 = 10.0 / units::TWIPS_PER_POINT;
const LO_FOOTNOTE_SEPARATOR_TOP_DIST_PT: f32 = 57.0 / units::TWIPS_PER_POINT;
const LO_FOOTNOTE_SEPARATOR_BOTTOM_DIST_PT: f32 = 57.0 / units::TWIPS_PER_POINT;
// Word-style endnote separators are 2 inches wide, and inline endnotes keep
// 269 twips of separator area above the endnote text.
const LO_ENDNOTE_SEPARATOR_WIDTH_PT: f32 = 2880.0 / units::TWIPS_PER_POINT;
const LO_ENDNOTE_SEPARATOR_BOTTOM_DIST_PT: f32 = 269.0 / units::TWIPS_PER_POINT;
const LO_PLACEHOLDER_FLOATING_LINE_HEIGHT_PER_FONT_SIZE: f32 = 0.484;

#[derive(Clone, Copy, Debug)]
enum NoteSeparatorKind {
  Footnote,
  Endnote,
  EndnoteContinuation,
}

fn inline_text_height(style: &TextStyle, text_metrics: &mut TextMetrics) -> f32 {
  text_metrics.inline_text_box_height(style)
}

fn paragraph_base_line_style(paragraph: &crate::docx::Paragraph) -> TextStyle {
  if let Some(style) = paragraph.inlines.iter().find_map(|inline| match inline {
    InlineItem::Text(run) if text_run_affects_line_height(&run.text) => Some(run.style.clone()),
    InlineItem::Text(_) => None,
    InlineItem::Image(_)
    | InlineItem::Shape(_)
    | InlineItem::BookmarkStart(_)
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
  style.font_size_pt = TextStyle::default().font_size_pt;
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
  text_metrics: &mut TextMetrics,
) -> f32 {
  match paragraph.format.line_height_rule {
    LineHeightRule::Auto => {
      // ECMA-376 §17.3.1.33 defines an auto `w:line` value in 240ths of
      // the normal single line height. Use the resolved face metrics for that
      // single-line basis; 115% of the nominal font size is only Writer's
      // compatibility approximation and can differ from Word's font height.
      let single_line_height = paragraph
        .inlines
        .iter()
        .find_map(|inline| match inline {
          InlineItem::Text(run) if text_run_affects_line_height(&run.text) => Some(
            inline_text_height_for_text(&run.style, &run.text, text_metrics),
          ),
          _ => None,
        })
        .unwrap_or_else(|| inline_text_height(base_line_style, text_metrics));
      let line_height = paragraph
        .format
        .line_height_pt
        .map_or(single_line_height, |multiple| single_line_height * multiple);
      // SwTextFormatter::CalcRealHeight() uses the imported document grid
      // base height as the auto line real height in grid layout. In Writer this
      // grid snap happens before proportional line spacing is applied, and the
      // first paragraph line keeps the snapped grid height. Applying the
      // proportional multiplier before the snap would round 135% of one grid
      // line up to two grid lines.
      if paragraph.format.snap_to_grid.unwrap_or(true)
        && text_segment_uses_document_grid(text_segmentation, setup)
        && setup.doc_grid_line_pitch_pt.is_some()
      {
        let grid_line_height = inline_text_height(base_line_style, text_metrics);
        snap_line_height_to_doc_grid(grid_line_height, setup.doc_grid_line_pitch_pt)
      } else {
        line_height
      }
    }
    LineHeightRule::AtLeast | LineHeightRule::Exact => paragraph
      .format
      .line_height_pt
      .unwrap_or_else(|| inline_text_height(base_line_style, text_metrics)),
  }
}

fn grid_auto_line_spacing_multiplier(
  paragraph: &crate::docx::Paragraph,
  setup: PageSetup,
  text_segmentation: TextSegmentation,
) -> Option<f32> {
  if !matches!(paragraph.format.line_height_rule, LineHeightRule::Auto)
    || !paragraph.format.snap_to_grid.unwrap_or(true)
    || !text_segment_uses_document_grid(text_segmentation, setup)
    || setup.doc_grid_line_pitch_pt.is_none()
  {
    return None;
  }
  paragraph
    .format
    .line_height_pt
    .map(|multiple| multiple.max(1.0))
}

fn text_segment_uses_document_grid(text_segmentation: TextSegmentation, setup: PageSetup) -> bool {
  matches!(text_segmentation, TextSegmentation::Body)
    || (matches!(text_segmentation, TextSegmentation::TableCell)
      && setup.adjust_table_line_heights_to_grid)
}

fn snap_line_height_to_doc_grid(line_height: f32, doc_grid_line_pitch_pt: Option<f32>) -> f32 {
  let Some(grid_height) = doc_grid_line_pitch_pt else {
    return line_height;
  };
  if grid_height <= LAYOUT_EPSILON_PT || line_height <= grid_height {
    return line_height.max(grid_height);
  }
  // SwTextFormatter::CalcRealHeight() rounds a snapped line up to the next
  // document-grid base-height multiple.
  (line_height / grid_height).ceil() * grid_height
}

fn word_auto_line_height(style: &TextStyle) -> f32 {
  effective_font_size_pt(style, None) * LO_DOCUMENT_DEFAULT_LINE_SPACING_PERCENT / PERCENT_SCALE
}

fn libreoffice_empty_paragraph_first_line_height(style: &TextStyle) -> f32 {
  // first-line box that is taller than the default 115% auto line height.
  effective_font_size_pt(style, None) * LO_EMPTY_PARAGRAPH_FIRST_LINE_HEIGHT_PER_FONT_SIZE
}

fn include_text_height(
  line_height: f32,
  text_frame: TextFrame,
  style: &TextStyle,
  text: &str,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let text_height = if text_frame.script_sensitive_line_height {
    inline_text_height_for_text(style, text, text_metrics)
  } else {
    inline_text_height(style, text_metrics)
  };
  match text_frame.line_height_rule {
    LineHeightRule::Exact => line_height,
    LineHeightRule::Auto | LineHeightRule::AtLeast => line_height.max(text_height),
  }
}

fn inline_drawing_line_height(
  object_height_pt: f32,
  paragraph: &crate::docx::Paragraph,
  text_frame: TextFrame,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if matches!(text_frame.line_height_rule, LineHeightRule::Exact) {
    return object_height_pt;
  }
  // ECMA-376 Part 1 §20.4.2.3 defines an inline drawing as affecting its
  // line like a character glyph of similar size. The object's height is the
  // ascent above the baseline; the paragraph mark still supplies the normal
  // font descent below it. Writer models the same split in
  // SwLineLayout::MaxAscentDescent(), keeping fly-content ascent/descent and
  // ordinary text descent as independent maxima.
  object_height_pt
    + text_metrics
      .vertical_metrics(&paragraph_base_line_style(paragraph))
      .descent_pt
}

fn inline_object_requires_flow_advance(
  y_pt: f32,
  object_line_height_pt: f32,
  content_bottom_pt: f32,
  page_has_body_content: bool,
) -> bool {
  page_has_body_content && y_pt + object_line_height_pt > content_bottom_pt + LAYOUT_EPSILON_PT
}

fn inline_text_height_for_text(
  style: &TextStyle,
  text: &str,
  text_metrics: &mut TextMetrics,
) -> f32 {
  text_metrics.inline_text_box_height_for_text(text, style)
}

fn line_real_height(
  paragraph: &crate::docx::Paragraph,
  line_height: f32,
  line_index: usize,
  grid_auto_line_spacing_multiplier: Option<f32>,
  _has_content_control: bool,
) -> f32 {
  if let Some(height) = placeholder_floating_line_height(paragraph, line_height) {
    return height;
  }
  if line_index > 0
    && let Some(multiple) = grid_auto_line_spacing_multiplier
  {
    return line_height * multiple;
  }
  line_height
}

fn placeholder_floating_line_height(
  paragraph: &crate::docx::Paragraph,
  line_height: f32,
) -> Option<f32> {
  if !paragraph_has_behind_layout_in_cell_floating(paragraph) {
    return None;
  }
  let mut max_font_size: f32 = 0.0;
  let mut saw_text = false;
  for inline in &paragraph.inlines {
    let InlineItem::Text(run) = inline else {
      continue;
    };
    if !text_run_affects_line_height(&run.text) {
      continue;
    }
    if !run.preserve_text_portion {
      return None;
    }
    saw_text = true;
    max_font_size = max_font_size.max(effective_font_size_pt(&run.style, None));
  }
  saw_text
    .then(|| (max_font_size * LO_PLACEHOLDER_FLOATING_LINE_HEIGHT_PER_FONT_SIZE).min(line_height))
}

fn paragraph_has_behind_layout_in_cell_floating(paragraph: &crate::docx::Paragraph) -> bool {
  paragraph.inlines.iter().any(|inline| match inline {
    InlineItem::Image(image) => matches!(
      image.placement,
      crate::docx::ImagePlacement::Floating(placement)
        if placement.behind_text && placement.layout_in_cell
    ),
    InlineItem::Shape(shape) => matches!(
      shape.placement,
      crate::docx::ImagePlacement::Floating(placement)
        if placement.behind_text && placement.layout_in_cell
    ),
    _ => false,
  })
}

#[derive(Clone, Debug)]
pub(crate) struct LayoutDocument {
  pub pages: Vec<Page>,
  pub form_widgets: Vec<crate::docx::FormWidget>,
  pub follows: Vec<FrameFollow>,
  pub frames: Vec<LayoutFrame>,
  pub outline_entries: Vec<OutlineEntry>,
  pub anchor_pages: Vec<AnchorPage>,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AnchorPage {
  pub name: String,
  pub page_index: usize,
  pub section_index: usize,
  pub section_page_index: usize,
  pub physical_page_number: usize,
  pub virtual_page_number: usize,
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
  pub item_count: usize,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct PageReplaySpan {
  page_index: usize,
  section_page_index: usize,
  column_index: usize,
  scope: ReflowScope,
  item_start: usize,
  item_end: usize,
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
struct LineNumberBox {
  item_start: usize,
  y_pt: f32,
  height_pt: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct Page {
  pub setup: PageSetup,
  pub section_index: usize,
  pub section_page_index: usize,
  pub items: Vec<PageItem>,
  body_content_frames: usize,
  explicit_break_target: bool,
  preserve_empty: bool,
  delete_forbidden: bool,
  frame_fragments: Vec<FrameFragment>,
  frame_influences: Vec<FrameInfluence>,
  wrap_exclusions: Vec<WrapExclusion>,
  repeating_wrap_exclusion_catalog: RepeatingWrapExclusionCatalog,
  repeating_wrap_exclusions: Vec<WrapExclusion>,
  repeating_adornment: Option<RepeatingAdornment>,
  pending_floating_table_follows: Vec<PendingFloatingTableFollow>,
}

#[derive(Clone, Debug)]
struct RepeatingAdornment {
  items: Vec<PageItem>,
  frame_fragments: Vec<FrameFragment>,
  frame_influences: Vec<FrameInfluence>,
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
  pending_floating_table_follows: Vec<PendingFloatingTableFollow>,
}

#[derive(Clone, Debug)]
pub(crate) enum PageItem {
  Text(Box<TextItem>),
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
  pub rotation_center_pt: Option<(f32, f32)>,
  pub hyperlink_url: Option<String>,
  pub dynamic_field: Option<DynamicFieldKind>,
  pub style_ref_keys: Vec<Arc<str>>,
  pub style_ref_text: Option<Arc<str>>,
  pub style_ref_numbering_text: Option<Arc<str>>,
  pub form_widget_id: Option<u32>,
  pub paragraph_bidi: bool,
  pub word_spacing_pt: f32,
  pub preserve_text_portion: bool,
  pub decoration_span_start_x_pt: Option<f32>,
  pub pdf_text_segmentation: PdfTextSegmentation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PdfTextSegmentation {
  Line,
  WordLine,
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
  pub data: Arc<[u8]>,
  pub content_type: Option<String>,
  pub alt_text: Option<String>,
  pub hyperlink_url: Option<String>,
  pub semantic_metafile_text: bool,
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
  layout_cell_print_bounds: Option<FrameBounds>,
  default_tab_stop_pt: f32,
  compatibility_mode: u16,
  justify_lines_with_shrinking: bool,
  split_page_break_and_paragraph_mark: bool,
  repeating_slots: RepeatingSlotState,
  text_segmentation: TextSegmentation,
  paragraph_spacing_context: ParagraphSpacingContext,
  preserve_horizontal_on_advance: bool,
  script_sensitive_line_height: bool,
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
  leader: TabLeader,
}

#[derive(Clone, Debug)]
struct PendingAlignedTab {
  stop: ResolvedTabStop,
  leader_start_x_pt: f32,
  y_pt: f32,
  line_height_pt: f32,
  style: TextStyle,
  item_start_index: usize,
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
  compatibility_mode: u16,
  justify_lines_with_shrinking: bool,
  repeating_slots: RepeatingSlotState,
}

pub(crate) fn layout(document: &DocxDocument, options: &LayoutOptions) -> Result<LayoutDocument> {
  if !options.action.calc_layout {
    return Ok(empty_layout_document(document));
  }
  Ok(RootFrameLayout::new(document, options).format())
}

fn empty_layout_document(_document: &DocxDocument) -> LayoutDocument {
  LayoutDocument {
    pages: Vec::new(),
    form_widgets: Vec::new(),
    follows: Vec::new(),
    frames: Vec::new(),
    outline_entries: Vec::new(),
    anchor_pages: Vec::new(),
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

pub(crate) fn layout_summary(layout: LayoutDocument) -> super::DocxLayoutSummary {
  let mut lines = Vec::new();
  let mut rows = Vec::new();
  for frame in layout.frames {
    for fragment in frame.fragments {
      let Some(bounds) = fragment.bounds else {
        continue;
      };
      match fragment.kind {
        FrameFragmentKind::ParagraphLine => lines.push(super::DocxLayoutLineSummary {
          page_index: frame.page_index,
          section_index: frame.section_index,
          section_page_index: frame.section_page_index,
          block_index: frame.block_index,
          line_index: fragment.index,
          x_pt: bounds.x_pt,
          y_pt: bounds.y_pt,
          width_pt: bounds.width_pt,
          height_pt: bounds.height_pt,
        }),
        FrameFragmentKind::TableRow => rows.push(super::DocxLayoutRowSummary {
          page_index: frame.page_index,
          section_index: frame.section_index,
          section_page_index: frame.section_page_index,
          block_index: frame.block_index,
          row_index: fragment.row_index,
          x_pt: bounds.x_pt,
          y_pt: bounds.y_pt,
          width_pt: bounds.width_pt,
          height_pt: bounds.height_pt,
        }),
        FrameFragmentKind::TableCell | FrameFragmentKind::NoteLine => {}
      }
    }
  }
  super::DocxLayoutSummary { lines, rows }
}

pub(crate) fn layout_common_document(
  document: &DocxDocument,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  match layout(document, options) {
    Ok(layout) => into_common_document(layout, options),
    Err(_) => common::LayoutDocument {
      engine_kind: common::LayoutEngineKind::Docx,
      options: common_layout_options(options),
      ..Default::default()
    },
  }
}

fn into_common_document(
  document: LayoutDocument,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  let debug_records = if options.diagnostics.collect_debug_records {
    common_debug_records(&document)
  } else {
    Vec::new()
  };

  common::LayoutDocument {
    engine_kind: common::LayoutEngineKind::Docx,
    options: common_layout_options(options),
    pages: document.pages.into_iter().map(into_common_page).collect(),
    form_widgets: document
      .form_widgets
      .into_iter()
      .map(into_common_form_widget)
      .collect(),
    follows: document
      .follows
      .into_iter()
      .map(into_common_frame_follow)
      .collect(),
    frames: document
      .frames
      .into_iter()
      .enumerate()
      .map(|(index, frame)| into_common_layout_frame(index, frame))
      .collect(),
    outline_entries: document
      .outline_entries
      .into_iter()
      .map(into_common_outline_entry)
      .collect(),
    anchor_pages: document
      .anchor_pages
      .into_iter()
      .map(into_common_anchor_page)
      .collect(),
    reflow: common_reflow_diagnostics(CommonReflowParts {
      page_replays: document.page_replays,
      page_replay_applications: document.page_replay_applications,
      backward_moves: document.backward_moves,
      layout_reruns: document.layout_reruns,
      page_invalidations: document.page_invalidations,
      reflow_executions: document.reflow_executions,
      reflow_requests: document.reflow_requests,
      restart_plan: document.restart_plan,
    }),
    debug_records,
    ..Default::default()
  }
}

fn common_layout_options(options: &LayoutOptions) -> common::LayoutOptions {
  common::LayoutOptions {
    collect_debug: options.diagnostics.collect_debug_records,
    approximate_unsupported: false,
    preserve_source_links: options.diagnostics.preserve_source_links,
  }
}

struct CommonReflowParts {
  page_replays: Vec<PageReplay>,
  page_replay_applications: Vec<PageReplayApplication>,
  backward_moves: Vec<BackwardMove>,
  layout_reruns: Vec<LayoutRerun>,
  page_invalidations: Vec<PageInvalidation>,
  reflow_executions: Vec<ReflowExecution>,
  reflow_requests: Vec<ReflowRequest>,
  restart_plan: Option<RestartPlan>,
}

fn common_reflow_diagnostics(parts: CommonReflowParts) -> common::ReflowDiagnostics<'static> {
  common::ReflowDiagnostics {
    page_replays: parts
      .page_replays
      .into_iter()
      .map(into_common_page_replay)
      .collect(),
    page_replay_applications: parts
      .page_replay_applications
      .into_iter()
      .map(into_common_page_replay_application)
      .collect(),
    backward_moves: parts
      .backward_moves
      .into_iter()
      .map(into_common_backward_move)
      .collect(),
    layout_reruns: parts
      .layout_reruns
      .into_iter()
      .map(into_common_layout_rerun)
      .collect(),
    page_invalidations: parts
      .page_invalidations
      .into_iter()
      .map(into_common_page_invalidation)
      .collect(),
    reflow_executions: parts
      .reflow_executions
      .into_iter()
      .map(into_common_reflow_execution)
      .collect(),
    reflow_requests: parts
      .reflow_requests
      .into_iter()
      .map(into_common_reflow_request)
      .collect(),
    restart_plan: parts.restart_plan.map(into_common_restart_plan),
  }
}

fn into_common_page_replay(replay: PageReplay) -> common::PageReplay<'static> {
  common::PageReplay {
    page_index: replay.page_index,
    section_page_index: replay.section_page_index,
    column_index: replay.column_index,
    scope: common_reflow_scope(replay.scope),
    item_range: common_item_range(replay.item_start, replay.item_end),
    replacement_items: replay
      .replacement_items
      .into_iter()
      .map(into_common_page_item)
      .collect(),
  }
}

fn into_common_page_replay_application(
  application: PageReplayApplication,
) -> common::PageReplayApplication {
  common::PageReplayApplication {
    page_index: application.page_index,
    section_page_index: application.section_page_index,
    column_index: application.column_index,
    scope: common_reflow_scope(application.scope),
    item_range: common_item_range(application.item_start, application.item_end),
    replacement_count: application.replacement_count,
    applied: application.applied,
  }
}

fn into_common_backward_move(move_back: BackwardMove) -> common::BackwardMove {
  common::BackwardMove {
    frame_index: move_back.frame_index,
    replay_start_frame_index: move_back.replay_start_frame_index,
    from_page_index: move_back.from_page_index,
    to_page_index: move_back.to_page_index,
    from_section_page_index: move_back.from_section_page_index,
    to_section_page_index: move_back.to_section_page_index,
    scope: common_reflow_scope(move_back.scope),
    reason: common_reflow_reason(move_back.reason),
    suppressed: move_back.suppressed,
    replayed_frames: move_back.replayed_frames,
    replayed_items: move_back.replayed_items,
  }
}

fn into_common_layout_rerun(rerun: LayoutRerun) -> common::LayoutRerun {
  common::LayoutRerun {
    checkpoint_index: rerun.checkpoint_index,
    section_index: rerun.section_index,
    block_index: rerun.block_index,
    page_index: rerun.page_index,
    frame_index: rerun.frame_index,
    reason: common_reflow_reason(rerun.reason),
    scope: common_reflow_scope(rerun.scope),
    replaced_pages: rerun.replaced_pages,
    produced_pages: rerun.produced_pages,
    produced_frames: rerun.produced_frames,
    constraints: rerun
      .constraints
      .into_iter()
      .map(into_common_layout_rerun_constraint)
      .collect(),
  }
}

fn into_common_layout_rerun_constraint(
  constraint: LayoutRerunConstraint,
) -> common::LayoutRerunConstraint {
  common::LayoutRerunConstraint {
    kind: common_frame_influence_kind(constraint.kind),
    scope: common_reflow_scope(constraint.scope),
    bounds: constraint.bounds.map(common_rect_from_frame_bounds),
    content_left: common::Pt(constraint.content_left_pt),
    content_width: common::Pt(constraint.content_width),
    content_bottom: common::Pt(constraint.content_bottom),
  }
}

fn into_common_page_invalidation(invalidation: PageInvalidation) -> common::PageInvalidation {
  common::PageInvalidation {
    page_index: invalidation.page_index,
    section_page_index: invalidation.section_page_index,
    first_frame_index: invalidation.first_frame_index,
    reason: common_reflow_reason(invalidation.reason),
    scope: common_reflow_scope(invalidation.scope),
  }
}

fn into_common_reflow_execution(execution: ReflowExecution) -> common::ReflowExecution {
  common::ReflowExecution {
    first_page_index: execution.first_page_index,
    request_count: execution.request_count,
    action: match execution.action {
      ReflowAction::StabilizedRetainedDecorationItems => {
        common::ReflowAction::StabilizedRetainedDecorationItems
      }
      ReflowAction::StabilizedInsertionInfluences => {
        common::ReflowAction::StabilizedInsertionInfluences
      }
    },
    scope: common_reflow_scope(execution.scope),
    suppressed_moves: execution.suppressed_moves,
    backward_moves: execution.backward_moves,
    page_replacements: execution.page_replacements,
    replayed_frames: execution.replayed_frames,
    replayed_items: execution.replayed_items,
  }
}

fn into_common_reflow_request(request: ReflowRequest) -> common::ReflowRequest {
  common::ReflowRequest {
    frame_index: request.frame_index,
    kind: common_frame_kind(request.kind),
    reason: common_reflow_reason(request.reason),
    scope: common_reflow_scope(request.scope),
    restart: into_common_frame_cursor(request.restart),
    page_index: request.page_index,
    section_page_index: request.section_page_index,
    column_index: request.column_index,
    influence_count: request.influence_count,
  }
}

fn into_common_restart_plan(plan: RestartPlan) -> common::RestartPlan {
  common::RestartPlan {
    page_index: plan.page_index,
    frame_index: plan.frame_index,
    block_index: plan.block_index,
    cursor: into_common_frame_cursor(plan.cursor),
    reason: common_reflow_reason(plan.reason),
    scope: common_reflow_scope(plan.scope),
  }
}

fn common_debug_records(document: &LayoutDocument) -> Vec<common::DebugRecord<'static>> {
  let page_records = document.pages.iter().enumerate().map(|(index, page)| {
    common::DebugRecord::Page(common::DebugPage {
      index,
      name: None,
      bounds: common_rect(0.0, 0.0, page.setup.width_pt, page.setup.height_pt),
    })
  });
  let frame_records = document.frames.iter().enumerate().map(|(index, frame)| {
    common::DebugRecord::Frame(common::DebugFrame {
      id: common::FrameId(index as u32),
      parent: None,
      kind: Cow::Borrowed(common_frame_kind_name(frame.kind)),
      bounds: frame
        .bounds
        .map(common_rect_from_frame_bounds)
        .unwrap_or_default(),
      print_bounds: frame
        .bounds
        .map(common_rect_from_frame_bounds)
        .unwrap_or_default(),
    })
  });
  let line_records = document
    .frames
    .iter()
    .enumerate()
    .flat_map(|(frame_index, frame)| {
      frame
        .lines
        .iter()
        .enumerate()
        .map(move |(line_index, line)| {
          let line = localize_line_box(*line, frame.item_start);
          common::DebugRecord::TextLine(common::DebugTextLine {
            frame: common::FrameId(frame_index as u32),
            index: line_index,
            text: Cow::Owned(frame_text_for_range(
              &frame.items,
              line.item_start,
              line.item_end,
            )),
            bounds: common_rect(line.x_pt, line.y_pt, line.width_pt, line.height_pt),
          })
        })
    });
  page_records
    .chain(frame_records)
    .chain(line_records)
    .collect()
}

fn frame_text_for_range(items: &[PageItem], start: usize, end: usize) -> String {
  items
    .iter()
    .skip(start)
    .take(end.saturating_sub(start))
    .filter_map(|item| match item {
      PageItem::Text(text) => Some(text.text.as_str()),
      _ => None,
    })
    .collect::<Vec<_>>()
    .join("")
}

fn into_common_anchor_page(anchor: AnchorPage) -> common::AnchorPage<'static> {
  common::AnchorPage {
    name: Cow::Owned(anchor.name),
    page_index: anchor.page_index,
    section_index: anchor.section_index,
    section_page_index: anchor.section_page_index,
    physical_page_number: anchor.physical_page_number,
    virtual_page_number: anchor.virtual_page_number,
  }
}

fn into_common_page(page: Page) -> common::DisplayPage<'static> {
  let setup = common_page_setup(page.setup);
  common::DisplayPage {
    section_index: page.section_index,
    section_page_index: page.section_page_index,
    bounds: common_rect(0.0, 0.0, page.setup.width_pt, page.setup.height_pt),
    background: page
      .setup
      .background
      .map(|color| common::Fill::Solid(common_rgb(color, 1.0))),
    setup,
    items: page.items.into_iter().map(into_common_page_item).collect(),
    ..Default::default()
  }
}

fn into_common_page_item(item: PageItem) -> common::DisplayItem<'static> {
  match item {
    PageItem::Text(item) => common::DisplayItem::Text(into_common_text_run(*item)),
    PageItem::Image(item) => common::DisplayItem::Image(into_common_image_item(item)),
    PageItem::Rect(item) => common::DisplayItem::Rect(into_common_rect_item(item)),
    PageItem::Fill(item) => common::DisplayItem::Rect(common::RectItem {
      bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
      fill: common::Fill::Solid(common_rgb(item.color, 1.0)),
      stroke: None,
    }),
    PageItem::Line(item) => common::DisplayItem::Line(into_common_line_item(item)),
    PageItem::Polyline(item) => common::DisplayItem::Path(into_common_path_item(item)),
  }
}

fn into_common_text_run(item: TextItem) -> common::TextRun<'static> {
  let color = common_rgb(item.style.color, item.style.opacity);
  common::TextRun {
    text: Cow::Owned(item.text),
    origin: common_point(item.x_pt, item.y_pt),
    line_height: common::Pt(item.line_height_pt),
    style: common_text_style(item.style),
    font_id: None,
    color,
    rotation_center: item.rotation_center_pt.map(|(x, y)| common_point(x, y)),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    dynamic_field: item.dynamic_field.map(into_common_dynamic_field),
    form_widget_id: item.form_widget_id,
    paragraph_bidi: item.paragraph_bidi,
    word_spacing_pt: item.word_spacing_pt,
    preserve_text_portion: item.preserve_text_portion,
    pdf_text_segmentation: match item.pdf_text_segmentation {
      PdfTextSegmentation::Line => common::PdfTextSegmentation::Line,
      PdfTextSegmentation::WordLine => common::PdfTextSegmentation::WordLine,
      PdfTextSegmentation::Portion => common::PdfTextSegmentation::Portion,
    },
    source: None,
  }
}

fn into_common_image_item(item: ImageItem) -> common::ImageItem<'static> {
  common::ImageItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    crop: Some(common::ImageCrop {
      left: item.crop.left,
      top: item.crop.top,
      right: item.crop.right,
      bottom: item.crop.bottom,
    }),
    clip_path: Vec::new(),
    rotation_degrees: item.rotation_deg,
    flip_horizontal: item.flip_horizontal,
    flip_vertical: item.flip_vertical,
    content_type: item
      .content_type
      .map(Cow::Owned)
      .unwrap_or(Cow::Borrowed("application/octet-stream")),
    bytes: item.data,
    relationship_id: None,
    alt_text: item.alt_text.map(Cow::Owned),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    semantic_metafile_text: item.semantic_metafile_text,
    floating: item.floating,
    behind_text: item.behind_text,
  }
}

fn into_common_rect_item(item: RectItem) -> common::RectItem<'static> {
  common::RectItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    fill: item
      .fill_color
      .map(|color| common::Fill::Solid(common_rgb(color, item.fill_opacity)))
      .unwrap_or(common::Fill::None),
    stroke: item
      .stroke
      .map(|stroke| common_stroke_from_border(stroke, item.stroke_opacity)),
  }
}

fn into_common_line_item(item: LineItem) -> common::LineItem<'static> {
  common::LineItem {
    start: common_point(item.x1_pt, item.y1_pt),
    end: common_point(item.x2_pt, item.y2_pt),
    stroke: common::Stroke {
      width: common::Pt(item.width_pt),
      color: common_rgb(item.color, 1.0),
      dash: None,
      source_style_id: None,
    },
    kind: match item.kind {
      LineItemKind::Stroke => common::LineKind::Stroke,
      LineItemKind::FilledRect => common::LineKind::FilledRect,
    },
  }
}

fn into_common_path_item(item: PolylineItem) -> common::PathItem<'static> {
  common::PathItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    points: item
      .points
      .into_iter()
      .map(|(x, y)| common_point(item.x_pt + x, item.y_pt + y))
      .collect(),
    commands: Vec::new(),
    closed: item.closed,
    fill: item
      .fill_color
      .map(|color| common::Fill::Solid(common_rgb(color, 1.0)))
      .unwrap_or(common::Fill::None),
    stroke: item
      .stroke
      .map(|stroke| common_stroke_from_border(stroke, 1.0)),
  }
}

fn into_common_dynamic_field(field: DynamicFieldKind) -> common::DynamicField<'static> {
  match field {
    DynamicFieldKind::Page { number_format } => common::DynamicField::Page {
      number_format: common_field_number_format(number_format),
    },
    DynamicFieldKind::NumPages { number_format } => common::DynamicField::NumPages {
      number_format: common_field_number_format(number_format),
    },
    DynamicFieldKind::PageRef { bookmark_name } => common::DynamicField::PageRef {
      bookmark_name: Cow::Owned(bookmark_name.to_string()),
    },
    DynamicFieldKind::StyleRef {
      style_name,
      from_bottom,
      ..
    } => common::DynamicField::StyleRef {
      style_name: Cow::Owned(style_name.to_string()),
      from_bottom,
    },
  }
}

fn common_field_number_format(format: FieldNumberFormat) -> common::FieldNumberFormat {
  match format {
    FieldNumberFormat::Decimal => common::FieldNumberFormat::Decimal,
    FieldNumberFormat::LowerRoman => common::FieldNumberFormat::LowerRoman,
    FieldNumberFormat::UpperRoman => common::FieldNumberFormat::UpperRoman,
    FieldNumberFormat::LowerLetter => common::FieldNumberFormat::LowerLetter,
    FieldNumberFormat::UpperLetter => common::FieldNumberFormat::UpperLetter,
  }
}

fn into_common_form_widget(widget: crate::docx::FormWidget) -> common::FormWidget<'static> {
  common::FormWidget {
    id: widget.id,
    kind: match widget.kind {
      crate::docx::FormWidgetKind::Text => common::FormWidgetKind::Text,
      crate::docx::FormWidgetKind::DropDownList => common::FormWidgetKind::DropDownList,
      crate::docx::FormWidgetKind::ComboBox => common::FormWidgetKind::ComboBox,
    },
    entries: widget.entries.into_iter().map(Cow::Owned).collect(),
  }
}

fn into_common_outline_entry(entry: OutlineEntry) -> common::OutlineEntry<'static> {
  common::OutlineEntry {
    level: entry.level,
    text: Cow::Owned(entry.text),
    page_index: entry.page_index,
    target: common_point(entry.x_pt, entry.y_pt),
    merged_hidden_separator: entry.merged_hidden_separator,
  }
}

fn into_common_layout_frame(index: usize, frame: LayoutFrame) -> common::FrameRecord<'static> {
  let item_offset = frame.item_start;
  common::FrameRecord {
    id: common::FrameId(index as u32),
    parent: None,
    kind: Cow::Borrowed(common_frame_kind_name(frame.kind)),
    block_index: frame.block_index,
    page_index: frame.page_index,
    section_index: frame.section_index,
    section_page_index: frame.section_page_index,
    column_index: frame.column_index,
    // Frame and line ranges address the owning page display list. Keeping
    // them page-relative is required by PDF line ownership: otherwise every
    // later frame starts again at item zero and can assign its baseline and
    // clip to an earlier paragraph's text.
    item_range: common_item_range(frame.item_start, frame.item_end),
    split_start: into_common_frame_cursor(localize_frame_cursor(frame.split_start, item_offset)),
    split_end: into_common_frame_cursor(localize_frame_cursor(frame.split_end, item_offset)),
    bounds: frame.bounds.map(common_rect_from_frame_bounds),
    print_bounds: frame.bounds.map(common_rect_from_frame_bounds),
    lines: frame.lines.into_iter().map(into_common_line_box).collect(),
    fragments: frame
      .fragments
      .into_iter()
      .map(into_common_frame_fragment)
      .collect(),
    influences: frame
      .influences
      .into_iter()
      .map(into_common_frame_influence)
      .collect(),
    invalidation: into_common_frame_invalidation(frame.invalidation),
  }
}

fn into_common_frame_follow(follow: FrameFollow) -> common::FrameFollow {
  common::FrameFollow {
    kind: common_frame_kind(follow.kind),
    reason: match follow.reason {
      FollowReason::KeepTogether => common::FollowReason::KeepTogether,
      FollowReason::Overflow => common::FollowReason::Overflow,
      FollowReason::ExplicitBreak => common::FollowReason::ExplicitBreak,
    },
    block_index: follow.block_index,
    from_page_index: follow.from_page_index,
    to_page_index: follow.to_page_index,
    from_section_page_index: follow.from_section_page_index,
    to_section_page_index: follow.to_section_page_index,
    from_column_index: follow.from_column_index,
    to_column_index: follow.to_column_index,
  }
}

fn into_common_frame_cursor(cursor: FrameCursor) -> common::FrameCursor {
  common::FrameCursor {
    block_index: cursor.block_index,
    kind: match cursor.kind {
      FrameCursorKind::BlockStart => common::FrameCursorKind::BlockStart,
      FrameCursorKind::Inline => common::FrameCursorKind::Inline,
      FrameCursorKind::TableRow => common::FrameCursorKind::TableRow,
      FrameCursorKind::TableCell => common::FrameCursorKind::TableCell,
      FrameCursorKind::BlockEnd => common::FrameCursorKind::BlockEnd,
    },
    inline_index: cursor.inline_index,
    text_offset: cursor.text_offset,
    row_index: cursor.row_index,
    cell_index: cursor.cell_index,
  }
}

fn into_common_line_box(line: LineBox) -> common::LineBox {
  common::LineBox {
    bounds: common_rect(line.x_pt, line.y_pt, line.width_pt, line.height_pt),
    item_range: common_item_range(line.item_start, line.item_end),
  }
}

fn into_common_frame_fragment(fragment: FrameFragment) -> common::FrameFragment {
  common::FrameFragment {
    kind: match fragment.kind {
      FrameFragmentKind::ParagraphLine => common::FrameFragmentKind::ParagraphLine,
      FrameFragmentKind::TableRow => common::FrameFragmentKind::TableRow,
      FrameFragmentKind::TableCell => common::FrameFragmentKind::TableCell,
      FrameFragmentKind::NoteLine => common::FrameFragmentKind::NoteLine,
    },
    split: match fragment.split {
      FragmentSplitKind::Complete => common::FragmentSplitKind::Complete,
      FragmentSplitKind::Master => common::FragmentSplitKind::Master,
      FragmentSplitKind::Follow => common::FragmentSplitKind::Follow,
      FragmentSplitKind::RepeatedHeader => common::FragmentSplitKind::RepeatedHeader,
    },
    index: fragment.index,
    row_index: fragment.row_index,
    cell_index: fragment.cell_index,
    item_range: common_item_range(fragment.item_start, fragment.item_end),
    bounds: fragment.bounds.map(common_rect_from_frame_bounds),
  }
}

fn into_common_frame_influence(influence: FrameInfluence) -> common::FrameInfluence {
  common::FrameInfluence {
    kind: match influence.kind {
      FrameInfluenceKind::FootnoteReservation => common::FrameInfluenceKind::FootnoteReservation,
      FrameInfluenceKind::FlyWrap => common::FrameInfluenceKind::FlyWrap,
      FrameInfluenceKind::TableSplit => common::FrameInfluenceKind::TableSplit,
    },
    count: influence.count,
    block_index: influence.block_index,
    item_range: common_item_range(influence.item_start, influence.item_end),
    bounds: influence.bounds.map(common_rect_from_frame_bounds),
  }
}

fn into_common_frame_invalidation(invalidation: FrameInvalidation) -> common::FrameInvalidation {
  match invalidation {
    FrameInvalidation::Clean => common::FrameInvalidation::Clean,
    FrameInvalidation::PageItemsDecorated => common::FrameInvalidation::PageItemsDecorated,
    FrameInvalidation::NeedsReflow => common::FrameInvalidation::NeedsReflow,
  }
}

fn common_frame_kind(kind: FollowFrameKind) -> common::FrameKind {
  match kind {
    FollowFrameKind::Paragraph => common::FrameKind::Paragraph,
    FollowFrameKind::Table => common::FrameKind::Table,
    FollowFrameKind::Notes => common::FrameKind::Notes,
  }
}

fn common_frame_influence_kind(kind: FrameInfluenceKind) -> common::FrameInfluenceKind {
  match kind {
    FrameInfluenceKind::FootnoteReservation => common::FrameInfluenceKind::FootnoteReservation,
    FrameInfluenceKind::FlyWrap => common::FrameInfluenceKind::FlyWrap,
    FrameInfluenceKind::TableSplit => common::FrameInfluenceKind::TableSplit,
  }
}

fn common_reflow_scope(scope: ReflowScope) -> common::ReflowScope {
  match scope {
    ReflowScope::Frame => common::ReflowScope::Frame,
    ReflowScope::Column => common::ReflowScope::Column,
    ReflowScope::Page => common::ReflowScope::Page,
  }
}

fn common_reflow_reason(reason: ReflowReason) -> common::ReflowReason {
  match reason {
    ReflowReason::DecorationChangedItems => common::ReflowReason::DecorationChangedItems,
    ReflowReason::InsertionInfluenceChanged => common::ReflowReason::InsertionInfluenceChanged,
    ReflowReason::InvalidBounds => common::ReflowReason::InvalidBounds,
  }
}

fn common_frame_kind_name(kind: FollowFrameKind) -> &'static str {
  match kind {
    FollowFrameKind::Paragraph => "paragraph",
    FollowFrameKind::Table => "table",
    FollowFrameKind::Notes => "notes",
  }
}

fn common_rect_from_frame_bounds(bounds: FrameBounds) -> common::Rect {
  common_rect(bounds.x_pt, bounds.y_pt, bounds.width_pt, bounds.height_pt)
}

fn common_item_range(start: usize, end: usize) -> common::ItemRange {
  common::ItemRange { start, end }
}

fn localize_frame_cursor(mut cursor: FrameCursor, item_offset: usize) -> FrameCursor {
  cursor.inline_index = cursor.inline_index.saturating_sub(item_offset);
  cursor
}

fn localize_line_box(mut line: LineBox, item_offset: usize) -> LineBox {
  line.item_start = line.item_start.saturating_sub(item_offset);
  line.item_end = line.item_end.saturating_sub(item_offset);
  line
}

struct RootFrameLayout<'a> {
  document: &'a DocxDocument,
  action: LayoutActionOptions,
  collect_frame_items: bool,
  emit_reflow_diagnostics: bool,
  pages: Vec<Page>,
  current: Page,
  y: f32,
  emitted_footnotes: HashSet<i64>,
  emitted_footnote_order: Vec<i64>,
  follows: Vec<FrameFollow>,
  frames: Vec<LayoutFrame>,
  outline_entries: Vec<OutlineEntry>,
  anchor_pages: Vec<AnchorPage>,
  checkpoints: Vec<LayoutCheckpoint>,
  text_metrics: TextMetrics,
  next_line_number: i16,
  pending_trailing_page_break: bool,
}

#[derive(Clone, Copy, Debug)]
struct LayoutCheckpoint {
  section_index: usize,
  block_index: usize,
  page_index: usize,
  y: f32,
  flow: FlowContext,
  current: PageCheckpoint,
  emitted_footnotes_len: usize,
  follows_len: usize,
  frames_len: usize,
  outline_entries_len: usize,
  anchor_pages_len: usize,
  next_line_number: i16,
}

#[derive(Clone, Copy, Debug)]
struct PageCheckpoint {
  setup: PageSetup,
  section_index: usize,
  section_page_index: usize,
  items_len: usize,
  body_content_frames: usize,
  explicit_break_target: bool,
  preserve_empty: bool,
  delete_forbidden: bool,
  frame_fragments_len: usize,
  frame_influences_len: usize,
  wrap_exclusions_len: usize,
  repeating_wrap_exclusions_len: usize,
  pending_floating_table_follows_len: usize,
}

fn can_reuse_page_for_section(
  current: &Page,
  section_index: usize,
  break_kind: SectionBreakKind,
  previous_section_is_empty: bool,
  current_page_has_body_progress: bool,
) -> bool {
  let populated_explicit_break_target_precedes_next_page_section = break_kind
    == SectionBreakKind::NextPage
    && current.explicit_break_target
    && current_page_has_body_progress;
  current.items.is_empty()
    && (!current.preserve_empty || !previous_section_is_empty)
    && !populated_explicit_break_target_precedes_next_page_section
    && (section_index == 0
      || break_kind == SectionBreakKind::Continuous
      || (break_kind == SectionBreakKind::NextPage && current.section_page_index > 0)
      || (starts_new_page(break_kind) && !current_page_has_body_progress)
      || (!starts_new_page(break_kind) && !current_page_has_body_progress))
}

impl<'a> RootFrameLayout<'a> {
  fn new(document: &'a DocxDocument, options: &LayoutOptions) -> Self {
    Self {
      document,
      action: options.action,
      collect_frame_items: options.diagnostics.collect_debug_records,
      emit_reflow_diagnostics: options.diagnostics.collect_reflow_records,
      pages: Vec::new(),
      current: empty_page(document.page, 0),
      y: document.page.margin_top_pt,
      emitted_footnotes: HashSet::default(),
      emitted_footnote_order: Vec::new(),
      follows: Vec::new(),
      frames: Vec::new(),
      outline_entries: Vec::new(),
      anchor_pages: Vec::new(),
      checkpoints: Vec::new(),
      text_metrics: TextMetrics::new(),
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
    if self.action.check_pages {
      check_page_desc_empty_pages(
        self.document,
        &mut self.pages,
        &mut self.frames,
        &mut self.follows,
        &mut self.outline_entries,
        &mut self.anchor_pages,
      );
    }

    let mut layout_reruns = Vec::new();
    let influence_reflow_requests = reflow_requests_for_frames(&self.frames, true);
    let (mut reflow_requests, mut reflow_executions, mut page_replays, mut backward_moves) =
      execute_reflow_requests(
        &mut self.frames,
        &self.pages,
        influence_reflow_requests,
        self.emit_reflow_diagnostics,
        &mut self.text_metrics,
      );
    if let Some(rerun) = self.apply_checkpoint_rerun(&backward_moves) {
      layout_reruns.push(rerun);
      let influence_reflow_requests = reflow_requests_for_frames(&self.frames, true);
      (
        reflow_requests,
        reflow_executions,
        page_replays,
        backward_moves,
      ) = execute_reflow_requests(
        &mut self.frames,
        &self.pages,
        influence_reflow_requests,
        self.emit_reflow_diagnostics,
        &mut self.text_metrics,
      );
    }
    let mut page_replay_applications = apply_page_replays(
      &mut self.pages,
      &mut page_replays,
      self.emit_reflow_diagnostics,
    );

    if self.action.paint {
      let page_item_counts_before_decoration = self
        .pages
        .iter()
        .map(|page| page.items.len())
        .collect::<Vec<_>>();
      apply_page_backgrounds(&mut self.pages);
      place_floating_images(&mut self.pages);
      apply_column_separators(self.document, &mut self.pages, &self.frames);
      apply_headers_and_footers(self.document, &mut self.pages, &mut self.text_metrics);
      apply_page_borders(&mut self.pages);
      resolve_dynamic_fields(
        &mut self.pages,
        &self.anchor_pages,
        &self.document.footnote_numbering,
        &self.document.endnote_numbering,
      );
      mark_decorated_frame_invalidations(
        &mut self.frames,
        &self.pages,
        &page_item_counts_before_decoration,
      );
      let decoration_reflow_requests = reflow_requests_for_frames(&self.frames, false);
      let (
        remaining_decoration_reflow_requests,
        decoration_reflow_executions,
        mut decoration_page_replays,
        decoration_backward_moves,
      ) = execute_reflow_requests(
        &mut self.frames,
        &self.pages,
        decoration_reflow_requests,
        self.emit_reflow_diagnostics,
        &mut self.text_metrics,
      );
      page_replay_applications.extend(apply_page_replays(
        &mut self.pages,
        &mut decoration_page_replays,
        self.emit_reflow_diagnostics,
      ));
      page_replays.extend(decoration_page_replays);
      backward_moves.extend(decoration_backward_moves);
      reflow_executions.extend(decoration_reflow_executions);
      reflow_requests.extend(remaining_decoration_reflow_requests);
    }
    let page_count = self.pages.len();
    self
      .follows
      .retain(|follow| follow.from_page_index < page_count && follow.to_page_index < page_count);
    normalize_layout_frames(&mut self.frames, &self.pages);
    let (page_invalidations, restart_plan) = if self.emit_reflow_diagnostics {
      normalize_reflow_requests(&mut reflow_requests, &self.frames);
      normalize_page_replays(&mut page_replays, &self.pages);
      normalize_page_replay_applications(&mut page_replay_applications, &self.pages);
      normalize_backward_moves(&mut backward_moves, &self.frames, &self.pages);
      normalize_reflow_executions(&mut reflow_executions, &self.pages, &backward_moves);
      let page_invalidations = page_invalidations_for_reflow_requests(&reflow_requests);
      let restart_plan = restart_plan_for_page_invalidations(&self.frames, &page_invalidations);
      (page_invalidations, restart_plan)
    } else {
      page_replays.clear();
      page_replay_applications.clear();
      backward_moves.clear();
      layout_reruns.clear();
      reflow_executions.clear();
      reflow_requests.clear();
      (Vec::new(), None)
    };

    materialize_table_frame_fragment_bounds(&mut self.pages, &self.frames);
    materialize_repeating_adornments(&mut self.pages, &mut self.frames);

    LayoutDocument {
      pages: self.pages,
      form_widgets: self.document.form_widgets.clone(),
      follows: self.follows,
      frames: self.frames,
      outline_entries: self.outline_entries,
      anchor_pages: self.anchor_pages,
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
    let document = self.document;
    if document.sections.is_empty() {
      let flow = self.body_flow(document_page_frame(
        document.page,
        0,
        SectionColumns::default(),
      ));
      self.y = flow.content_top_pt;
      self.format_block_sequence(&document.blocks, flow);
      return;
    }

    for section_index in 0..document.sections.len() {
      let section = &document.sections[section_index];
      self.start_section_frame(section_index, section);
      let flow = self.body_flow(document_page_frame(
        section.page,
        section_index,
        section.columns,
      ));
      self.y = self.y.max(flow.content_top_pt);
      let previous_section_block = section_index
        .checked_sub(1)
        .and_then(|index| document.sections.get(index))
        .and_then(|section| section.blocks.last());
      if !blocks_have_visible_body_content(&section.blocks) && section_has_repeating_blocks(section)
      {
        self.current.preserve_empty = true;
        self.current.delete_forbidden = true;
      }
      if section.blocks.is_empty()
        && section_index == 0
        && section_index + 1 < document.sections.len()
      {
        // keeps an empty section page when a paragraph-level sectPr precedes a
        // following section break; a nextColumn break without columns then acts
        // as a page break, so later content must not reuse the empty page.
        self.current.preserve_empty = true;
        self.current.delete_forbidden = true;
      }
      self.format_block_sequence_with_previous(&section.blocks, flow, previous_section_block);
    }
  }

  fn body_flow(&mut self, frame: BodyFrame) -> FlowContext {
    let flow = flow_context(
      frame.setup,
      frame.section_index,
      frame.columns,
      0,
      self.document.default_tab_stop_pt,
    );
    body_flow_for_page(
      FlowContext {
        repeating_slots: repeating_slot_state(
          self.document,
          frame.section_index,
          &mut self.text_metrics,
        ),
        compatibility_mode: self.document.compatibility_mode,
        justify_lines_with_shrinking: self.document.justify_lines_with_shrinking,
        split_page_break_and_paragraph_mark: self.document.split_page_break_and_paragraph_mark,
        ..flow
      },
      self.pages.len() + 1,
    )
  }

  fn start_section_frame(&mut self, section_index: usize, section: &crate::docx::ImportedSection) {
    let current_page_has_body_progress = self.current_page_has_body_progress();
    let previous_section_is_empty = section_index > 0
      && self
        .document
        .sections
        .get(section_index - 1)
        .is_some_and(|previous| previous.blocks.is_empty());
    if section_index > 0
      && self.current.items.is_empty()
      && self.current.preserve_empty
      && previous_section_is_empty
      && starts_new_page(section.break_kind)
    {
      // CheckPageDescs() keeps intentionally empty pages for page-style
      // transitions, but a following continuous section does not consume an
      // extra body page before its own content/breaks are laid out.
      self.push_current_page(empty_page(section.page, section_index));
      self.y = body_content_limits_for_page(
        section.page,
        repeating_slot_state(self.document, section_index, &mut self.text_metrics),
        self.pages.len() + 1,
        0,
      )
      .0;
      return;
    }
    if section_index > 0
      && section.break_kind == SectionBreakKind::Continuous
      && section.columns.count > 1
      && section.columns.unbalanced
      && blocks_have_footnote_references(&section.blocks)
      && current_page_has_body_progress
    {
      // moves footnotes in non-balanced column sections to the page frame.
      self.push_current_page(empty_page(section.page, section_index));
      self.y = body_content_limits_for_page(
        section.page,
        repeating_slot_state(self.document, section_index, &mut self.text_metrics),
        self.pages.len() + 1,
        0,
      )
      .0;
      return;
    }
    let reuse_empty_page = can_reuse_page_for_section(
      &self.current,
      section_index,
      section.break_kind,
      previous_section_is_empty,
      current_page_has_body_progress,
    );
    if reuse_empty_page {
      self.current.setup = section.page;
      self.current.section_index = section_index;
      self.current.section_page_index = 0;
      activate_pending_floating_table_follows_for_current(&mut self.current, &mut self.pages);
      self.seed_current_repeating_wrap_exclusions();
      self.y = body_content_limits_for_page(
        section.page,
        repeating_slot_state(self.document, section_index, &mut self.text_metrics),
        self.pages.len() + 1,
        0,
      )
      .0;
    } else if section_index > 0
      && starts_new_page(section.break_kind)
      && current_page_has_body_progress
    {
      self.push_current_page(empty_page(section.page, section_index));
      self.y = body_content_limits_for_page(
        section.page,
        repeating_slot_state(self.document, section_index, &mut self.text_metrics),
        self.pages.len() + 1,
        0,
      )
      .0;
    }
  }

  fn current_page_has_body_progress(&mut self) -> bool {
    let (body_top, body_bottom) = body_content_limits_for_page(
      self.current.setup,
      repeating_slot_state(
        self.document,
        self.current.section_index,
        &mut self.text_metrics,
      ),
      self.pages.len() + 1,
      self.current.section_page_index,
    );
    let body_flow = FlowContext {
      content_top_pt: body_top,
      content_bottom: body_bottom,
      ..flow_context(
        self.current.setup,
        self.current.section_index,
        SectionColumns::default(),
        self.current.section_page_index,
        self.document.default_tab_stop_pt,
      )
    };
    if page_has_body_region_items(&self.current, body_flow) {
      return true;
    }
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
      current: page_checkpoint(&self.current),
      emitted_footnotes_len: self.emitted_footnote_order.len(),
      follows_len: self.follows.len(),
      frames_len: self.frames.len(),
      outline_entries_len: self.outline_entries.len(),
      anchor_pages_len: self.anchor_pages.len(),
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
      // testTdf123636_newlinePageBreak4: with SplitPgBreakAndParaMark, a
      // non-first empty page-break paragraph creates an empty following page
      // but does not move following body text there.
      self.pending_trailing_page_break = true;
      return;
    }
    let kind = follow_kind_for_block(block);
    let (block_flow, footnote_top) = footnote_boss_reserve(
      block,
      self.document,
      &self.emitted_footnotes,
      *flow,
      &mut self.text_metrics,
    );
    let transition = self.follow_transition_start(*flow);
    (*flow, self.y) = prepare_block_flow(
      block,
      next,
      block_flow,
      &mut self.current,
      &mut self.pages,
      &mut self.text_metrics,
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
    let line_number_fragment_start = self.current.frame_fragments.len();
    let frame_influences = block_frame_influences(
      block,
      self.document,
      &self.emitted_footnotes,
      *flow,
      Some(block_index),
      &mut self.text_metrics,
    );
    (*flow, self.y) = layout_document_block(
      previous,
      block,
      next,
      *flow,
      LayoutBlockTarget {
        current: &mut self.current,
        pages: &mut self.pages,
        anchor_pages: Some(&mut self.anchor_pages),
        text_metrics: &mut self.text_metrics,
      },
      self.y,
    );
    self.materialize_pending_floating_table_follow_chain(flow);
    self.add_line_numbers_for_block(
      block,
      *flow,
      frame_start,
      line_number_start,
      line_number_fragment_start,
    );
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
      FootnoteEmission {
        emitted_footnotes: &mut self.emitted_footnotes,
        emitted_footnote_order: &mut self.emitted_footnote_order,
      },
      *flow,
      FootnoteLayoutTarget {
        current: &mut self.current,
        pages: &mut self.pages,
        text_metrics: &mut self.text_metrics,
      },
      footnote_top,
    );
    *flow = restore_body_content_bottom(*flow);
    *flow = self.advance_if_past_body(*flow);
  }

  fn materialize_pending_floating_table_follow_chain(&mut self, flow: &mut FlowContext) {
    if !has_pending_floating_table_follows(&self.current, &self.pages) {
      return;
    }
    // SwFrame::GetNextFlyLeaf() creates the follow fly during layout and
    // chains it to the split anchor. SwTabFrame::Split() then moves rows into
    // the follow table before frame recording/reflow sees the result. Keep the
    // same ordering: a split floating table must be part of the current
    // master/follow page chain, not an orphan materialized after all frames
    // have already been recorded.
    materialize_pending_floating_table_follows_in_local_pages(&mut self.current, &mut self.pages);

    if self.current.section_index != flow.section_index
      || self.current.section_page_index != flow.section_page_index
    {
      let next_flow = body_flow_for_page(
        flow_with_column(
          FlowContext {
            setup: self.current.setup,
            section_index: self.current.section_index,
            section_page_index: self.current.section_page_index,
            ..*flow
          },
          0,
        ),
        self.pages.len() + 1,
      );
      *flow = next_flow;
      self.y = if self
        .current
        .wrap_exclusions
        .iter()
        .any(|exclusion| !exclusion.blocks_flow)
      {
        // A split fly with side wrapping creates its follow before the body
        // text frame on that page. Writer keeps the body cursor at the page
        // top and lets CalcFlyOffsets constrain each line beside the follow;
        // advancing to the fly's painted bottom would turn Around wrapping
        // into an implicit top-and-bottom wrap.
        next_flow.content_top_pt
      } else {
        page_items_vertical_bounds(&self.current.items)
          .map_or(next_flow.content_top_pt, |(_, bottom)| {
            bottom.max(next_flow.content_top_pt)
          })
      };
    }
  }

  fn add_line_numbers_for_block(
    &mut self,
    block: &Block,
    flow: FlowContext,
    frame_start: FrameSegmentStart,
    start_index: usize,
    fragment_start: usize,
  ) {
    if flow.text_segmentation != TextSegmentation::Body {
      return;
    }
    if !matches!(block, Block::Paragraph(_)) {
      return;
    }
    let Some(line_numbering) = flow.setup.line_numbering else {
      return;
    };
    let line_number_style = self.document.line_number_style.clone();

    // A paragraph is one Word line-numbering scope even when its text frame
    // has master/follow fragments on several pages. Add the numbers in page
    // order before advancing to the next block; looking only at `current`
    // loses every line already moved into `pages` by a paragraph split.
    let completed_end = self.pages.len();
    let mut line_number_context = LineNumberPageContext {
      numbering: line_numbering,
      style: &line_number_style,
      content_left_pt: flow.content_left_pt,
      next_number: &mut self.next_line_number,
      text_metrics: &mut self.text_metrics,
    };
    for page_index in frame_start.page_index..completed_end {
      let page = &mut self.pages[page_index];
      let item_start = if page_index == frame_start.page_index {
        start_index
      } else {
        0
      };
      let page_fragment_start = if page_index == frame_start.page_index {
        fragment_start
      } else {
        0
      };
      add_line_numbers_to_page(
        page,
        item_start,
        page_fragment_start,
        &mut line_number_context,
      );
    }

    let current_starts_frame = completed_end == frame_start.page_index;
    add_line_numbers_to_page(
      &mut self.current,
      if current_starts_frame { start_index } else { 0 },
      if current_starts_frame {
        fragment_start
      } else {
        0
      },
      &mut line_number_context,
    );
  }

  fn advance_if_past_body(&mut self, flow: FlowContext) -> FlowContext {
    if self.y + DEFAULT_LINE_HEIGHT_PT > flow.content_bottom
      && page_has_body_region_items(&self.current, flow)
    {
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
      let mut emitted_endnotes = HashSet::default();
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
        &mut self.text_metrics,
      );
      let (_, y) = layout_document_block(
        previous,
        block,
        next,
        flow,
        LayoutBlockTarget {
          current: &mut self.current,
          pages: &mut self.pages,
          anchor_pages: None,
          text_metrics: &mut self.text_metrics,
        },
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
    let checkpoint = *checkpoint;
    let replaced_pages = self.pages.len().saturating_sub(checkpoint.page_index);
    self.restore_layout_checkpoint(&checkpoint)?;
    self.checkpoints.truncate(checkpoint_index + 1);
    self.format_body_from_checkpoint(&checkpoint, &constraints);
    self.format_trailing_note_frames();
    self.finish_current_page();

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
      produced_frames: self.frames.len().saturating_sub(checkpoint.frames_len),
      constraints,
    })
  }

  fn restore_layout_checkpoint(&mut self, checkpoint: &LayoutCheckpoint) -> Option<()> {
    let mut current = if checkpoint.page_index < self.pages.len() {
      let tail = self.pages.split_off(checkpoint.page_index);
      tail.into_iter().next()?
    } else if checkpoint.page_index == self.pages.len() {
      std::mem::replace(
        &mut self.current,
        empty_page(checkpoint.current.setup, checkpoint.current.section_index),
      )
    } else {
      return None;
    };
    restore_page_checkpoint(&mut current, checkpoint.current);
    self.current = current;
    self.y = checkpoint.y;
    while self.emitted_footnote_order.len() > checkpoint.emitted_footnotes_len {
      if let Some(id) = self.emitted_footnote_order.pop() {
        self.emitted_footnotes.remove(&id);
      }
    }
    self.follows.truncate(checkpoint.follows_len);
    self.frames.truncate(checkpoint.frames_len);
    self
      .outline_entries
      .truncate(checkpoint.outline_entries_len);
    self.anchor_pages.truncate(checkpoint.anchor_pages_len);
    self.next_line_number = checkpoint.next_line_number;
    Some(())
  }

  fn format_body_from_checkpoint(
    &mut self,
    checkpoint: &LayoutCheckpoint,
    constraints: &[LayoutRerunConstraint],
  ) {
    let checkpoint_flow = constrained_rerun_flow(checkpoint.flow, constraints);
    self.y = self.y.min(checkpoint_flow.content_bottom);
    let document = self.document;
    if document.sections.is_empty() {
      self.format_block_range(&document.blocks, checkpoint.block_index, checkpoint_flow);
      return;
    }

    for section_index in checkpoint.section_index..document.sections.len() {
      let section = &document.sections[section_index];
      if section_index == checkpoint.section_index {
        self.format_block_range(&section.blocks, checkpoint.block_index, checkpoint_flow);
      } else {
        self.start_section_frame(section_index, section);
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
    if !self.current.items.is_empty()
      || self.current.body_content_frames > 0
      || self.current.preserve_empty
      || self.pages.is_empty()
    {
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
    page.explicit_break_target = true;
    page.preserve_empty = true;
    page.delete_forbidden = true;
    self.pages.push(page);
  }

  fn push_current_page(&mut self, next: Page) {
    self.pages.push(std::mem::replace(&mut self.current, next));
    activate_pending_floating_table_follows_for_current(&mut self.current, &mut self.pages);
    self.seed_current_repeating_wrap_exclusions();
  }

  fn seed_current_repeating_wrap_exclusions(&mut self) {
    let catalog = repeating_wrap_exclusion_catalog_for_page(
      self.document,
      &self.current,
      &mut self.text_metrics,
    );
    let exclusions = catalog
      .selected(self.current.section_page_index, self.pages.len() + 1)
      .to_vec();
    let previous = std::mem::replace(&mut self.current.repeating_wrap_exclusions, exclusions);
    self
      .current
      .wrap_exclusions
      .retain(|exclusion| !previous.contains(exclusion));
    self.current.repeating_wrap_exclusion_catalog = catalog;
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
      let mut fragments = page_frame_fragments(kind, &page.frame_fragments, item_start, item_end);
      if item_start >= item_end && fragments.is_empty() {
        continue;
      }
      let page_items = &page.items[item_start..item_end];
      let bounds = frame_bounds_for_items(page_items, &mut self.text_metrics);
      let lines = line_boxes_for_items(&page.items, item_start, item_end, &mut self.text_metrics);
      if fragments.is_empty() {
        fragments = frame_fragments_for(kind, &lines);
      }
      let item_count = item_end.saturating_sub(item_start);
      let items = if self.collect_frame_items {
        page_items.to_vec()
      } else {
        Vec::new()
      };
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
        item_count,
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
        | InlineItem::BookmarkStart(_)
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
        return Some((page_index, *text.clone()));
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

fn frame_bounds_for_items(
  items: &[PageItem],
  text_metrics: &mut TextMetrics,
) -> Option<FrameBounds> {
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  for item in items {
    let Some((x1, y1, x2, y2)) = item_bounds(item, text_metrics) else {
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

fn line_boxes_for_items(
  items: &[PageItem],
  item_start: usize,
  item_end: usize,
  text_metrics: &mut TextMetrics,
) -> Vec<LineBox> {
  let mut lines = Vec::new();
  let mut index = item_start;
  while index < item_end {
    let Some(y) = item_line_y(&items[index]) else {
      index += 1;
      continue;
    };
    let line_start = index;
    let mut line_end = index + 1;
    let mut bounds = item_bounds(&items[index], text_metrics);
    while line_end < item_end
      && item_line_y(&items[line_end]).is_some_and(|next_y| (next_y - y).abs() < LAYOUT_EPSILON_PT)
    {
      if let Some((x1, y1, x2, y2)) = item_bounds(&items[line_end], text_metrics) {
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

fn line_number_boxes_for_block(
  page: &Page,
  item_start: usize,
  fragment_start: usize,
  text_metrics: &mut TextMetrics,
) -> Vec<LineNumberBox> {
  let item_end = page.items.len();
  let mut boxes = page
    .frame_fragments
    .iter()
    .skip(fragment_start)
    .filter(|fragment| matches!(fragment.kind, FrameFragmentKind::ParagraphLine))
    .filter(|fragment| {
      if fragment.item_start < fragment.item_end {
        fragment.item_start >= item_start && fragment.item_start < item_end
      } else {
        fragment.item_start >= item_start && fragment.item_start <= item_end
      }
    })
    .filter_map(|fragment| {
      let bounds = fragment.bounds?;
      if let Some((y_pt, height_pt)) =
        line_number_text_metrics_for_items(&page.items, fragment.item_start, fragment.item_end)
      {
        return Some(LineNumberBox {
          item_start: fragment.item_start,
          y_pt,
          height_pt,
        });
      }
      Some(LineNumberBox {
        item_start: fragment.item_start,
        y_pt: bounds.y_pt,
        height_pt: bounds.height_pt,
      })
    })
    .collect::<Vec<_>>();

  if boxes.is_empty() {
    boxes.extend(
      line_boxes_for_items(&page.items, item_start, item_end, text_metrics)
        .into_iter()
        .map(|line| LineNumberBox {
          item_start: line.item_start,
          y_pt: line.y_pt,
          height_pt: line.height_pt,
        }),
    );
  }

  boxes
}

struct LineNumberPageContext<'a> {
  numbering: LineNumbering,
  style: &'a TextStyle,
  content_left_pt: f32,
  next_number: &'a mut i16,
  text_metrics: &'a mut TextMetrics,
}

fn add_line_numbers_to_page(
  page: &mut Page,
  item_start: usize,
  fragment_start: usize,
  context: &mut LineNumberPageContext<'_>,
) {
  let line_boxes =
    line_number_boxes_for_block(page, item_start, fragment_start, context.text_metrics);
  let mut items = Vec::new();
  for line_box in line_boxes {
    let number = *context.next_number;
    *context.next_number = context.next_number.saturating_add(1);
    if number < context.numbering.start
      || (number - context.numbering.start) % context.numbering.count_by != 0
    {
      continue;
    }

    let style = context.style.clone();
    let text = number.to_string();
    let width = context.text_metrics.measure_text(&text, &style);
    items.push((
      line_box.item_start,
      PageItem::Text(Box::new(TextItem {
        x_pt: (context.content_left_pt - context.numbering.distance_pt - width).max(0.0),
        y_pt: line_box.y_pt,
        line_height_pt: line_box.height_pt,
        text,
        style,
        rotation_center_pt: None,
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
        form_widget_id: None,
        paragraph_bidi: false,
        word_spacing_pt: 0.0,
        decoration_span_start_x_pt: None,
        pdf_text_segmentation: PdfTextSegmentation::Line,
      })),
    ));
  }
  for (offset, (item_start, item)) in items.into_iter().enumerate() {
    insert_line_number_item(page, item_start + offset, item);
  }
}

fn insert_line_number_item(page: &mut Page, index: usize, item: PageItem) {
  page.items.insert(index, item);
  for fragment in &mut page.frame_fragments {
    if fragment.item_start == index && fragment.item_end == index {
      fragment.item_end += 1;
    } else if fragment.item_start >= index {
      fragment.item_start += 1;
      fragment.item_end += 1;
    } else if fragment.item_end > index {
      fragment.item_end += 1;
    }
  }
  for influence in &mut page.frame_influences {
    if influence.item_start >= index {
      influence.item_start += 1;
      influence.item_end += 1;
    } else if influence.item_end > index {
      influence.item_end += 1;
    }
  }
}

fn line_number_text_metrics_for_items(
  items: &[PageItem],
  item_start: usize,
  item_end: usize,
) -> Option<(f32, f32)> {
  items[item_start..item_end]
    .iter()
    .find_map(|item| match item {
      PageItem::Text(text) => Some((text.y_pt, text.line_height_pt)),
      PageItem::Image(_)
      | PageItem::Rect(_)
      | PageItem::Fill(_)
      | PageItem::Line(_)
      | PageItem::Polyline(_) => None,
    })
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
  text_metrics: &mut TextMetrics,
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
          referenced_footnote_area_height(block, document, emitted_footnotes, flow, text_metrics)
            .max(0.0);
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
        block_frame_influences(
          block,
          document,
          emitted_footnotes,
          flow,
          block_index,
          text_metrics,
        )
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
    .filter(|fragment| {
      if fragment.item_start < fragment.item_end {
        fragment.item_start < item_end && fragment.item_end > item_start
      } else {
        fragment.bounds.is_some()
          && fragment.item_start >= item_start
          && fragment.item_start <= item_end
      }
    })
    .cloned()
    .map(|mut fragment| {
      if matches!(frame_kind, FollowFrameKind::Notes)
        && matches!(fragment.kind, FrameFragmentKind::ParagraphLine)
      {
        fragment.kind = FrameFragmentKind::NoteLine;
      }
      if fragment.item_start < fragment.item_end {
        fragment.item_start = fragment.item_start.max(item_start);
        fragment.item_end = fragment.item_end.min(item_end);
      }
      fragment
    })
    .filter(|fragment| fragment.item_start < fragment.item_end || fragment.bounds.is_some())
    .collect()
}

fn push_page_fragment(
  page: &mut Page,
  fragment: PageFragmentRecord,
  text_metrics: &mut TextMetrics,
) {
  let PageFragmentRecord {
    kind,
    split,
    index,
    row_index,
    cell_index,
    item_start,
    item_end,
    bounds,
  } = fragment;
  if item_start >= item_end && bounds.is_none() {
    return;
  }
  let bounds =
    bounds.or_else(|| frame_bounds_for_items(&page.items[item_start..item_end], text_metrics));
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
  bounds: Option<FrameBounds>,
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
  pages: &[Page],
  requests: Vec<ReflowRequest>,
  collect_page_replays: bool,
  text_metrics: &mut TextMetrics,
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
  let mut move_backward_keys = HashMap::<MoveBackwardKey, usize>::default();
  let mut replayed_influence_frames = HashSet::<usize>::default();

  for request in requests {
    match request.reason {
      ReflowReason::DecorationChangedItems => {
        if let Some(frame) = frames.get_mut(request.frame_index)
          && frame.item_count == frame.item_end.saturating_sub(frame.item_start)
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
          if let Some(backward) = execute_backward_move(
            frames,
            pages,
            &request,
            &mut move_backward_keys,
            collect_page_replays,
            text_metrics,
          ) {
            suppressed_moves += usize::from(backward.move_back.suppressed);
            influence_replayed_frames += backward.move_back.replayed_frames;
            influence_replayed_items += backward.move_back.replayed_items;
            page_replays.extend(backward.pages);
            backward_moves.push(backward.move_back);
          }
          let replay =
            replay_scoped_frames(frames, pages, &request, collect_page_replays, text_metrics);
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

fn apply_page_replays(
  pages: &mut [Page],
  replays: &mut [PageReplay],
  preserve_replays: bool,
) -> Vec<PageReplayApplication> {
  replays.sort_by(|left, right| {
    right
      .page_index
      .cmp(&left.page_index)
      .then_with(|| right.section_page_index.cmp(&left.section_page_index))
      .then_with(|| right.column_index.cmp(&left.column_index))
      .then_with(|| right.scope.cmp(&left.scope))
      .then_with(|| right.item_start.cmp(&left.item_start))
      .then_with(|| right.item_end.cmp(&left.item_end))
  });
  let mut applications = Vec::with_capacity(replays.len());
  for replay in replays {
    let replacement_count = replay.replacement_items.len();
    let applied = pages
      .get_mut(replay.page_index)
      .is_some_and(|page| apply_page_replay(page, replay, preserve_replays));
    applications.push(PageReplayApplication {
      page_index: replay.page_index,
      section_page_index: replay.section_page_index,
      column_index: replay.column_index,
      scope: replay.scope,
      item_start: replay.item_start,
      item_end: replay.item_end,
      replacement_count,
      applied,
    });
  }
  applications
}

fn normalize_page_replays(replays: &mut Vec<PageReplay>, pages: &[Page]) {
  replays.retain(|replay| {
    pages.get(replay.page_index).is_some_and(|page| {
      replay.section_page_index == page.section_page_index
        && replay.item_start <= replay.item_end
        && !replay.replacement_items.is_empty()
    })
  });
}

fn normalize_page_replay_applications(
  applications: &mut Vec<PageReplayApplication>,
  pages: &[Page],
) {
  applications.retain(|application| {
    pages.get(application.page_index).is_some_and(|page| {
      application.section_page_index == page.section_page_index
        && application.item_start <= application.item_end
        && application.replacement_count > 0
    })
  });
}

fn normalize_backward_moves(moves: &mut Vec<BackwardMove>, frames: &[LayoutFrame], pages: &[Page]) {
  moves.retain(|move_back| {
    move_back.frame_index < frames.len()
      && move_back.replay_start_frame_index < frames.len()
      && pages
        .get(move_back.from_page_index)
        .is_some_and(|page| page.section_page_index == move_back.from_section_page_index)
      && pages
        .get(move_back.to_page_index)
        .is_some_and(|page| page.section_page_index == move_back.to_section_page_index)
      && move_back.to_page_index <= move_back.from_page_index
      && (move_back.suppressed || move_back.replayed_frames > 0)
  });
}

fn normalize_reflow_executions(
  executions: &mut Vec<ReflowExecution>,
  pages: &[Page],
  backward_moves: &[BackwardMove],
) {
  // tree after joins and CheckPageDescs() cleanup. These execution records are
  // diagnostics for that normalized tree, so keep their move counts in sync
  // with backward moves that survived normalization.
  executions.retain_mut(|execution| {
    if execution.request_count == 0 || execution.first_page_index >= pages.len() {
      return false;
    }
    execution.backward_moves = execution.backward_moves.min(backward_moves.len());
    true
  });
}

fn apply_page_replay(page: &mut Page, replay: &mut PageReplay, preserve_replay: bool) -> bool {
  if page.section_page_index != replay.section_page_index
    || replay.item_start > replay.item_end
    || replay.item_end > page.items.len()
  {
    return false;
  }

  if preserve_replay {
    page.items.splice(
      replay.item_start..replay.item_end,
      replay.replacement_items.clone(),
    );
  } else {
    page.items.splice(
      replay.item_start..replay.item_end,
      std::mem::take(&mut replay.replacement_items),
    );
  }
  true
}

fn materialize_pending_floating_table_follows(pages: &mut Vec<Page>) {
  for _ in 0..LAYOUT_LOOP_CONTROL_MAX {
    if !pages
      .iter()
      .any(|page| !page.pending_floating_table_follows.is_empty())
    {
      return;
    }
    let mut pending = Vec::<PendingFloatingTableFollow>::new();
    for page in pages.iter_mut() {
      pending.append(&mut page.pending_floating_table_follows);
    }
    for follow in pending {
      let page_index = ensure_section_page_slot(
        pages,
        follow.setup,
        follow.section_index,
        follow.section_page_index,
      );
      apply_pending_floating_table_follow(&mut pages[page_index], follow);
    }
  }
}

fn has_pending_floating_table_follows(current: &Page, pages: &[Page]) -> bool {
  !current.pending_floating_table_follows.is_empty()
    || pages
      .iter()
      .any(|page| !page.pending_floating_table_follows.is_empty())
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

fn activate_pending_floating_table_follows_for_current(current: &mut Page, pages: &mut [Page]) {
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
  page
    .pending_floating_table_follows
    .append(&mut follow.pending_floating_table_follows);
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

  for (insert_index, pending_page_index) in
    (insert_at..).zip(start_section_page..=section_page_index)
  {
    pages.insert(
      insert_index,
      empty_section_page(setup, section_index, pending_page_index),
    );
    if pending_page_index == section_page_index {
      return insert_index;
    }
  }

  insert_at
}

fn execute_backward_move(
  frames: &mut [LayoutFrame],
  pages: &[Page],
  request: &ReflowRequest,
  move_backward_keys: &mut HashMap<MoveBackwardKey, usize>,
  collect_page_replays: bool,
  text_metrics: &mut TextMetrics,
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
  let replay = replay_scoped_frames(
    frames,
    pages,
    &replay_request,
    collect_page_replays,
    text_metrics,
  );
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

fn replay_scoped_frames(
  frames: &mut [LayoutFrame],
  pages: &[Page],
  request: &ReflowRequest,
  collect_page_replays: bool,
  text_metrics: &mut TextMetrics,
) -> ReflowReplayStats {
  let mut stats = ReflowReplayStats::default();
  let Some(start) = frames.get(request.frame_index) else {
    return stats;
  };
  let start_page_index = start.page_index;
  let start_section_page_index = start.section_page_index;
  let start_column_index = start.column_index;

  let mut page_replay_spans = Vec::new();
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
    if collect_page_replays
      && let Some(span) = page_replay_span_for_frame(frame, pages, request.scope)
    {
      page_replay_spans.push(span);
    }
    replay_layout_frame(frame, pages, text_metrics);
    stats.frames += 1;
    stats.items += frame.item_count;
    stats.frame_indices.push(frame_index);
  }
  if collect_page_replays {
    stats.pages = page_replays_for_spans(page_replay_spans, pages);
  }
  stats
}

fn page_replay_span_for_frame(
  frame: &LayoutFrame,
  pages: &[Page],
  scope: ReflowScope,
) -> Option<PageReplaySpan> {
  let page = pages.get(frame.page_index)?;
  if page.section_page_index != frame.section_page_index
    || frame.item_start >= frame.item_end
    || frame.item_end > page.items.len()
  {
    return None;
  }
  Some(PageReplaySpan {
    page_index: frame.page_index,
    section_page_index: frame.section_page_index,
    column_index: frame.column_index,
    scope,
    item_start: frame.item_start,
    item_end: frame.item_end,
  })
}

fn page_replays_for_spans(mut spans: Vec<PageReplaySpan>, pages: &[Page]) -> Vec<PageReplay> {
  if spans.is_empty() {
    return Vec::new();
  }
  spans.sort_by(|left, right| {
    left
      .page_index
      .cmp(&right.page_index)
      .then_with(|| left.section_page_index.cmp(&right.section_page_index))
      .then_with(|| left.column_index.cmp(&right.column_index))
      .then_with(|| left.scope.cmp(&right.scope))
      .then_with(|| left.item_start.cmp(&right.item_start))
      .then_with(|| left.item_end.cmp(&right.item_end))
  });
  let mut coalesced = Vec::<PageReplaySpan>::with_capacity(spans.len());
  for span in spans {
    if let Some(current) = coalesced.last_mut()
      && current.page_index == span.page_index
      && current.section_page_index == span.section_page_index
      && current.column_index == span.column_index
      && current.scope == span.scope
      && span.item_start <= current.item_end
    {
      current.item_end = current.item_end.max(span.item_end);
      continue;
    }
    coalesced.push(span);
  }
  coalesced
    .into_iter()
    .filter_map(|span| {
      let page = pages.get(span.page_index)?;
      if page.section_page_index != span.section_page_index
        || span.item_start >= span.item_end
        || span.item_end > page.items.len()
      {
        return None;
      }
      Some(PageReplay {
        page_index: span.page_index,
        section_page_index: span.section_page_index,
        column_index: span.column_index,
        scope: span.scope,
        item_start: span.item_start,
        item_end: span.item_end,
        replacement_items: page.items[span.item_start..span.item_end].to_vec(),
      })
    })
    .collect()
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

fn replay_layout_frame(frame: &mut LayoutFrame, pages: &[Page], text_metrics: &mut TextMetrics) {
  if let Some(page) = pages.get(frame.page_index) {
    if frame.item_start > page.items.len() {
      frame.item_start = page.items.len();
    }
    frame.item_end = frame.item_end.min(page.items.len()).max(frame.item_start);
    frame.item_count = frame.item_end.saturating_sub(frame.item_start);
    frame.bounds =
      frame_bounds_for_items(&page.items[frame.item_start..frame.item_end], text_metrics);
    frame.lines = line_boxes_for_items(&page.items, frame.item_start, frame.item_end, text_metrics);
  } else if !frame.items.is_empty() {
    frame.item_start = 0;
    frame.item_end = frame.items.len();
    frame.item_count = frame.items.len();
    frame.bounds = frame_bounds_for_items(&frame.items, text_metrics);
    frame.lines = line_boxes_for_items(&frame.items, 0, frame.items.len(), text_metrics);
  } else {
    frame.item_end = frame.item_start;
    frame.item_count = 0;
    frame.bounds = None;
    frame.lines.clear();
  }
  let fallback_fragments = frame_fragments_for(frame.kind, &frame.lines);
  if frame.fragments.is_empty() {
    frame.fragments = fallback_fragments;
  } else {
    let fragment_item_bound = frame_fragment_item_bound(frame);
    normalize_replayed_fragments(
      &mut frame.fragments,
      &fallback_fragments,
      frame.kind,
      fragment_item_bound,
    );
  }
  frame.split_start = frame_cursor(
    frame.block_index,
    frame.kind,
    frame.item_start,
    &frame.lines,
    true,
  );
  frame.split_end = frame_cursor(
    frame.block_index,
    frame.kind,
    frame.item_end,
    &frame.lines,
    false,
  );
  frame.invalidation = FrameInvalidation::Clean;
}

fn frame_fragment_item_bound(frame: &LayoutFrame) -> usize {
  if frame.item_start == 0 {
    frame.item_count
  } else {
    frame.item_end
  }
}

fn normalize_replayed_fragments(
  fragments: &mut Vec<FrameFragment>,
  fallback: &[FrameFragment],
  kind: FollowFrameKind,
  item_len: usize,
) {
  if matches!(kind, FollowFrameKind::Table) {
    if item_len == 0 {
      fragments.retain(|fragment| fragment.bounds.is_some());
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

fn normalize_layout_frames(frames: &mut Vec<LayoutFrame>, pages: &[Page]) {
  frames.retain(|frame| {
    frame.page_index < pages.len()
      && frame.section_index == pages[frame.page_index].section_index
      && frame.section_page_index == pages[frame.page_index].section_page_index
      && frame.item_count > 0
  });
  for frame in frames {
    if frame.fragments.is_empty() {
      frame.fragments = frame_fragments_for(frame.kind, &frame.lines);
    } else {
      let fragment_item_bound = frame_fragment_item_bound(frame);
      normalize_replayed_fragments(
        &mut frame.fragments,
        &frame_fragments_for(frame.kind, &frame.lines),
        frame.kind,
        fragment_item_bound,
      );
    }
    frame.split_start = frame_cursor(
      frame.block_index,
      frame.kind,
      frame.item_start,
      &frame.lines,
      true,
    );
    frame.split_end = frame_cursor(
      frame.block_index,
      frame.kind,
      frame.item_end,
      &frame.lines,
      false,
    );
  }
}

fn normalize_reflow_requests(requests: &mut Vec<ReflowRequest>, frames: &[LayoutFrame]) {
  // after joins, page-desc cleanup and reflow. Keep our diagnostic restart
  // requests attached to the normalized layout frames instead of preserving
  // stale pre-normalization cursors.
  requests.retain_mut(|request| {
    let Some(frame) = frames.get(request.frame_index) else {
      return false;
    };
    request.kind = frame.kind;
    request.restart = frame.split_start;
    request.page_index = frame.page_index;
    request.section_page_index = frame.section_page_index;
    request.column_index = frame.column_index;
    request.influence_count = frame.influences.len();
    true
  });
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

fn item_bounds(item: &PageItem, text_metrics: &mut TextMetrics) -> Option<(f32, f32, f32, f32)> {
  match item {
    PageItem::Text(text) => {
      let width = text_metrics.measure_text(&text.text, &text.style);
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

fn page_items_bounds(
  items: &[PageItem],
  text_metrics: &mut TextMetrics,
) -> Option<(f32, f32, f32, f32)> {
  let mut bounds: Option<(f32, f32, f32, f32)> = None;
  for item in items {
    let Some((x1, y1, x2, y2)) = item_bounds(item, text_metrics) else {
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
      InlineItem::BookmarkStart(_) => {}
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
    InlineItem::BookmarkStart(_) => true,
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
    body_content_frames: 0,
    explicit_break_target: false,
    preserve_empty: false,
    delete_forbidden: false,
    frame_fragments: Vec::new(),
    frame_influences: Vec::new(),
    wrap_exclusions: Vec::new(),
    repeating_wrap_exclusion_catalog: RepeatingWrapExclusionCatalog::default(),
    repeating_wrap_exclusions: Vec::new(),
    repeating_adornment: None,
    pending_floating_table_follows: Vec::new(),
  }
}

fn page_checkpoint(page: &Page) -> PageCheckpoint {
  PageCheckpoint {
    setup: page.setup,
    section_index: page.section_index,
    section_page_index: page.section_page_index,
    items_len: page.items.len(),
    body_content_frames: page.body_content_frames,
    explicit_break_target: page.explicit_break_target,
    preserve_empty: page.preserve_empty,
    delete_forbidden: page.delete_forbidden,
    frame_fragments_len: page.frame_fragments.len(),
    frame_influences_len: page.frame_influences.len(),
    wrap_exclusions_len: page.wrap_exclusions.len(),
    repeating_wrap_exclusions_len: page.repeating_wrap_exclusions.len(),
    pending_floating_table_follows_len: page.pending_floating_table_follows.len(),
  }
}

fn restore_page_checkpoint(page: &mut Page, checkpoint: PageCheckpoint) {
  page.setup = checkpoint.setup;
  page.section_index = checkpoint.section_index;
  page.section_page_index = checkpoint.section_page_index;
  page.items.truncate(checkpoint.items_len);
  page.body_content_frames = checkpoint.body_content_frames;
  page.explicit_break_target = checkpoint.explicit_break_target;
  page.preserve_empty = checkpoint.preserve_empty;
  page.delete_forbidden = checkpoint.delete_forbidden;
  page
    .frame_fragments
    .truncate(checkpoint.frame_fragments_len);
  page
    .frame_influences
    .truncate(checkpoint.frame_influences_len);
  page
    .wrap_exclusions
    .truncate(checkpoint.wrap_exclusions_len);
  page
    .repeating_wrap_exclusions
    .truncate(checkpoint.repeating_wrap_exclusions_len);
  page
    .pending_floating_table_follows
    .truncate(checkpoint.pending_floating_table_follows_len);
}

fn restore_text_frame_start_page(
  current: &mut Page,
  pages: &mut Vec<Page>,
  start_pages_len: usize,
  checkpoint: PageCheckpoint,
  page_snapshot: Option<&Page>,
  pending_snapshot: Option<&TextFramePendingSnapshot>,
) {
  // Once a paragraph creates a follow, its original current page lives at
  // `pages[start_pages_len]` and `current` is the last follow page. Restoring
  // only the item counts on that last follow drops pre-existing content from
  // the master page and can duplicate later paragraphs during the retry.
  if let Some(snapshot) = page_snapshot {
    pages.truncate(start_pages_len);
    current.clone_from(snapshot);
  } else if pages.len() > start_pages_len {
    pages.truncate(start_pages_len + 1);
    *current = pages
      .pop()
      .expect("text-frame master page exists after a follow transition");
    restore_page_checkpoint(current, checkpoint);
  } else {
    pages.truncate(start_pages_len);
    restore_page_checkpoint(current, checkpoint);
  }
  if let Some(snapshot) = pending_snapshot {
    current
      .pending_floating_table_follows
      .clone_from(&snapshot.current);
    for (page, pending) in pages.iter_mut().zip(&snapshot.pages) {
      page.pending_floating_table_follows.clone_from(pending);
    }
  }
}

#[derive(Clone, Debug)]
struct TextFramePendingSnapshot {
  current: Vec<PendingFloatingTableFollow>,
  pages: Vec<Vec<PendingFloatingTableFollow>>,
}

#[derive(Clone)]
struct CheckPage {
  page: Page,
  original_index: Option<usize>,
}

fn check_page_desc_empty_pages(
  document: &DocxDocument,
  pages: &mut Vec<Page>,
  frames: &mut [LayoutFrame],
  follows: &mut Vec<FrameFollow>,
  outline_entries: &mut Vec<OutlineEntry>,
  anchor_pages: &mut Vec<AnchorPage>,
) {
  // SwFrame::CheckPageDescs() walks the page chain, inserts intentional empty
  // pages for right/left page wishes, and after every change re-evaluates the
  // affected previous/next page instead of doing a one-shot filter.
  let original_pages = std::mem::take(pages);
  let original_len = original_pages.len();
  let fallback_page = original_pages.first().cloned();
  let mut page_index_map = vec![usize::MAX; original_len];

  let mut checked = original_pages
    .into_iter()
    .enumerate()
    .map(|(original_index, page)| CheckPage {
      page,
      original_index: Some(original_index),
    })
    .collect::<Vec<_>>();

  let mut index = 0;
  while index < checked.len() {
    let is_intentional_empty = checked[index].page.preserve_empty;
    let is_frame_empty =
      !is_intentional_empty && page_frame_empty_for_check_page(&checked[index], follows);
    let is_empty = is_intentional_empty || is_frame_empty;
    let on_right = physical_page_is_right(index + 1);
    let want_right = page_wants_right_page(
      document,
      &checked[index].page,
      checked[..index].iter().map(|page| &page.page),
      index + 1,
    );

    if !is_empty
      && on_right != want_right
      && ((index == 0 && !want_right)
        || index
          .checked_sub(1)
          .and_then(|previous| checked.get(previous))
          .is_some_and(|previous| !previous.page.preserve_empty))
    {
      let mut empty = empty_section_page(
        checked[index].page.setup,
        checked[index].page.section_index,
        checked[index].page.section_page_index,
      );
      empty.preserve_empty = true;
      checked.insert(
        index,
        CheckPage {
          page: empty,
          original_index: None,
        },
      );
      continue;
    }

    if is_empty && !checked[index].page.delete_forbidden {
      let next_want_right = checked.get(index + 1).map(|next| {
        page_wants_right_page(
          document,
          &next.page,
          checked[..=index].iter().map(|page| &page.page),
          index + 2,
        )
      });
      if is_frame_empty || next_want_right.is_none() || next_want_right == Some(on_right) {
        checked.remove(index);
        if is_frame_empty && index > 0 {
          index -= 1;
        }
        continue;
      }
    }

    index += 1;
  }

  let mut kept = Vec::with_capacity(checked.len().max(1));
  for checked_page in checked {
    if let Some(original_index) = checked_page.original_index {
      page_index_map[original_index] = kept.len();
    }
    kept.push(checked_page.page);
  }

  for frame in frames {
    if let Some(&mapped) = page_index_map.get(frame.page_index)
      && mapped != usize::MAX
    {
      frame.page_index = mapped;
    }
  }
  follows.retain_mut(|follow| {
    let (Some(&from), Some(&to)) = (
      page_index_map.get(follow.from_page_index),
      page_index_map.get(follow.to_page_index),
    ) else {
      return false;
    };
    if from == usize::MAX || to == usize::MAX {
      return false;
    }
    follow.from_page_index = from;
    follow.to_page_index = to;
    true
  });
  outline_entries.retain_mut(|entry| {
    let Some(&mapped) = page_index_map.get(entry.page_index) else {
      return false;
    };
    if mapped == usize::MAX {
      return false;
    }
    entry.page_index = mapped;
    true
  });
  anchor_pages.retain_mut(|anchor| {
    let Some(&mapped) = page_index_map.get(anchor.page_index) else {
      return false;
    };
    if mapped == usize::MAX {
      return false;
    }
    let Some(page) = kept.get(mapped) else {
      return false;
    };
    anchor.page_index = mapped;
    anchor.section_index = page.section_index;
    anchor.section_page_index = page.section_page_index;
    anchor.physical_page_number = mapped + 1;
    anchor.virtual_page_number = virtual_page_number(page.setup, page.section_page_index, mapped);
    true
  });
  if kept.is_empty()
    && let Some(first) = fallback_page
  {
    kept.push(first);
  }
  *pages = kept;
}

fn page_frame_empty_for_check_page(page: &CheckPage, follows: &[FrameFollow]) -> bool {
  let Some(original_index) = page.original_index else {
    return is_page_frame_empty(&page.page);
  };
  is_page_frame_empty_for_check(&page.page, follows, original_index)
}

fn is_page_frame_empty(page: &Page) -> bool {
  // sw::IsPageFrameEmpty() removes only pages without essential body content.
  // Body table frames survive even when they have no rendered text yet; visible
  // body-anchored objects are essential. Page decorations are applied after this
  // pass, so they are not used to decide body emptiness.
  if !page.items.is_empty() {
    return false;
  }
  if page.explicit_break_target {
    return false;
  }
  page.body_content_frames == 0
}

fn is_page_frame_empty_for_check(page: &Page, follows: &[FrameFollow], page_index: usize) -> bool {
  is_page_frame_empty(page)
    && !follows.iter().any(|follow| {
      follow.to_page_index == page_index && matches!(follow.reason, FollowReason::ExplicitBreak)
    })
}

fn page_wants_right_page<'a, I>(
  document: &DocxDocument,
  page: &Page,
  kept_pages: I,
  physical_page_number: usize,
) -> bool
where
  I: Clone + Iterator<Item = &'a Page>,
{
  // SwFrame::WannaRightPage(): prefer the first body content's page number
  // offset, otherwise use the physical page side while ignoring a previous
  // intentional empty page.
  let first_body_content_for_section = !kept_pages.clone().any(|previous| {
    previous.section_index == page.section_index
      && !previous.preserve_empty
      && !is_page_frame_empty(previous)
  });
  let first_body_content_in_document = !kept_pages
    .clone()
    .any(|previous| !previous.preserve_empty && !is_page_frame_empty(previous));
  let previous_is_intentional_empty = kept_pages
    .clone()
    .last()
    .is_some_and(|previous| previous.preserve_empty);
  let mut wants_right = if first_body_content_for_section {
    if first_body_content_in_document {
      physical_page_is_right(physical_page_number) ^ previous_is_intentional_empty
    } else {
      page
        .setup
        .page_number_start
        .map(page_number_offset_wants_right_page)
        .unwrap_or_else(|| {
          physical_page_is_right(physical_page_number) ^ previous_is_intentional_empty
        })
    }
  } else {
    physical_page_is_right(physical_page_number) ^ previous_is_intentional_empty
  };

  if first_body_content_for_section && let Some(section) = document.sections.get(page.section_index)
  {
    match section.break_kind {
      SectionBreakKind::OddPage => wants_right = true,
      SectionBreakKind::EvenPage => wants_right = false,
      SectionBreakKind::Continuous | SectionBreakKind::NextPage | SectionBreakKind::NextColumn => {}
    }
  }
  wants_right
}

fn page_number_offset_wants_right_page(page_number: i32) -> bool {
  // IsRightPageByNumber(), for non-initial content where the document's first
  // virtual page has already established the parity base.
  page_number.rem_euclid(2) == 1
}

fn physical_page_is_right(page_number: usize) -> bool {
  !page_number.is_multiple_of(2)
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
    layout_cell_print_bounds: None,
    default_tab_stop_pt,
    compatibility_mode: 12,
    justify_lines_with_shrinking: false,
    split_page_break_and_paragraph_mark: false,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::Body,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
    preserve_horizontal_on_advance: false,
    script_sensitive_line_height: true,
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
    let mut next_flow = body_flow_for_page(
      flow_with_column(
        FlowContext {
          section_page_index: flow.section_page_index + 1,
          ..flow
        },
        0,
      ),
      pages.len() + 1,
    );
    if flow.preserve_horizontal_on_advance {
      next_flow.content_left_pt = flow.content_left_pt;
      next_flow.content_width = flow.content_width;
    }
    (next_flow, next_flow.content_top_pt)
  }
}

fn prepare_block_flow(
  block: &Block,
  next: Option<&Block>,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  text_metrics: &mut TextMetrics,
  y: f32,
) -> (FlowContext, f32) {
  if !page_has_body_region_items(current, flow) || !block_should_stay_together(block, next) {
    return (flow, y);
  }
  let required_height = keep_group_height(block, next, flow, text_metrics);
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

fn keep_group_height(
  block: &Block,
  next: Option<&Block>,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let mut height = estimated_block_height(block, flow, text_metrics);
  if let Block::Paragraph(paragraph) = block
    && paragraph.format.keep_with_next
    && let Some(Block::Paragraph(next)) = next
  {
    // CheckMoveFwd()/WouldFit() tests whether the next content can start in
    // the remaining upper, not whether the whole next paragraph fits there.
    height += estimated_paragraph_first_line_height(next, flow, text_metrics);
  }
  height
}

fn estimated_block_height(block: &Block, flow: FlowContext, text_metrics: &mut TextMetrics) -> f32 {
  match block {
    Block::Paragraph(paragraph) => estimated_paragraph_height(paragraph, flow, text_metrics),
    Block::Table(table) => estimated_table_height(table, flow, text_metrics),
    Block::Frame(frame) => estimated_frame_height(frame, flow, text_metrics),
  }
}

fn estimated_paragraph_height(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> f32 {
  paragraph.format.spacing_before_pt
    + estimated_paragraph_content_height(paragraph, flow, text_metrics)
    + paragraph_lower_space(paragraph)
}

fn estimated_paragraph_content_height(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let (indent_left_pt, indent_right_pt, first_line_indent_pt) =
    resolved_paragraph_indents(paragraph, text_metrics);
  let content_width =
    (flow.content_width - indent_left_pt - indent_right_pt).max(DEFAULT_FONT_SIZE_PT);
  let base_line_style = paragraph_base_line_style(paragraph);
  let base_line_height = paragraph_line_height_for_setup(
    paragraph,
    &base_line_style,
    flow.setup,
    flow.text_segmentation,
    text_metrics,
  );
  let text_frame = TextFrame {
    default_line_left: 0.0,
    first_line_left: 0.0,
    default_line_right: content_width,
    paragraph_left: 0.0,
    base_line_height,
    line_height_rule: paragraph.format.line_height_rule,
    grid_auto_line_spacing_multiplier: grid_auto_line_spacing_multiplier(
      paragraph,
      flow.setup,
      flow.text_segmentation,
    ),
    script_sensitive_line_height: flow.script_sensitive_line_height,
  };
  let mut line_height = base_line_height;
  let mut line_index = 0usize;
  let mut content_height = 0.0;
  let mut floating_bottom: f32 = 0.0;
  let mut has_flow_content = paragraph.list_label.is_some();
  let mut x = first_line_indent_pt.max(0.0);
  let finish_line = |content_height: &mut f32, line_height: &mut f32, line_index: &mut usize| {
    *content_height += line_real_height(
      paragraph,
      *line_height,
      *line_index,
      text_frame.grid_auto_line_spacing_multiplier,
      false,
    );
    *line_index += 1;
    *line_height = base_line_height;
  };

  for item in &paragraph.inlines {
    match item {
      InlineItem::Text(run) => {
        for segment in text_segments(&run.text) {
          if segment == "\n" {
            // ECMA-376 Part 1 §17.18.4: textWrapping restarts text on the
            // next line. Even when the break is the paragraph's only
            // content, that next (empty) line is part of the paragraph and
            // must be included in table-row measurement just as it is by
            // TextFrameLayout::format.
            finish_line(&mut content_height, &mut line_height, &mut line_index);
            has_flow_content = true;
            x = 0.0;
            continue;
          }
          if segment == "\t" {
            finish_line(&mut content_height, &mut line_height, &mut line_index);
            x = 0.0;
            continue;
          }
          has_flow_content = true;
          let width = text_metrics.measure_text(&segment, &run.style);
          if x + width > content_width && x > 0.0 {
            finish_line(&mut content_height, &mut line_height, &mut line_index);
            x = 0.0;
          }
          if width > content_width && x <= 0.0 && !segment.chars().all(char::is_whitespace) {
            for text in emergency_break_segments(&segment) {
              let width = text_metrics.measure_text(&text, &run.style);
              if width > content_width && text.chars().count() > 1 {
                for ch in text.chars() {
                  let mut encoded = [0; 4];
                  let text = ch.encode_utf8(&mut encoded);
                  let width = text_metrics.measure_text(text, &run.style);
                  if x + width > content_width && x > 0.0 {
                    finish_line(&mut content_height, &mut line_height, &mut line_index);
                    x = 0.0;
                  }
                  x += width;
                  if segment_affects_line_height(text) {
                    line_height =
                      include_text_height(line_height, text_frame, &run.style, text, text_metrics);
                  }
                }
                continue;
              }
              if x + width > content_width && x > 0.0 {
                finish_line(&mut content_height, &mut line_height, &mut line_index);
                x = 0.0;
              }
              x += width;
              if segment_affects_line_height(&text) {
                line_height =
                  include_text_height(line_height, text_frame, &run.style, &text, text_metrics);
              }
            }
            continue;
          }
          x += width.min(content_width);
          if segment_affects_line_height(&segment) {
            line_height =
              include_text_height(line_height, text_frame, &run.style, &segment, text_metrics);
          }
        }
      }
      InlineItem::BookmarkStart(_) => {}
      InlineItem::Image(image) => {
        if let crate::docx::ImagePlacement::Floating(placement) = image.placement
          && (effective_layout_in_cell(placement, flow)
            || flow.text_segmentation == TextSegmentation::RepeatingSlot)
          && matches!(
            flow.text_segmentation,
            TextSegmentation::TableCell | TextSegmentation::RepeatingSlot
          )
        {
          if placement.behind_text {
            continue;
          }
          if flow.text_segmentation == TextSegmentation::RepeatingSlot
            && matches!(
              placement.wrap,
              ImageWrapMode::None | ImageWrapMode::Through | ImageWrapMode::Inline
            )
          {
            continue;
          }
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
        let metrics = inline_image_metrics(image, content_width);
        has_flow_content = true;
        if x + metrics.frame_width_pt > content_width && x > 0.0 {
          finish_line(&mut content_height, &mut line_height, &mut line_index);
        }
        line_height = line_height.max(inline_drawing_line_height(
          metrics.frame_height_pt,
          paragraph,
          text_frame,
          text_metrics,
        ));
        x = metrics.frame_width_pt;
      }
      InlineItem::Shape(shape) => {
        if let crate::docx::ImagePlacement::Floating(placement) = shape.placement
          && (effective_layout_in_cell(placement, flow)
            || flow.text_segmentation == TextSegmentation::RepeatingSlot)
          && matches!(
            flow.text_segmentation,
            TextSegmentation::TableCell | TextSegmentation::RepeatingSlot
          )
        {
          if placement.behind_text {
            continue;
          }
          if flow.text_segmentation == TextSegmentation::RepeatingSlot
            && matches!(
              placement.wrap,
              ImageWrapMode::None | ImageWrapMode::Through | ImageWrapMode::Inline
            )
          {
            continue;
          }
          let width = relative_floating_width(placement, flow).unwrap_or(shape.width_pt);
          let height = relative_floating_height(placement, flow).unwrap_or(shape.height_pt);
          let (_, shape_y) =
            floating_image_position(placement, flow, x, content_height, width, height);
          floating_bottom = floating_bottom.max(shape_y + height + placement.margin_bottom_pt);
          continue;
        }
        let mut compatibility_forced_shape_line = false;
        if x + shape.width_pt > content_width && x > 0.0 {
          finish_line(&mut content_height, &mut line_height, &mut line_index);
          compatibility_forced_shape_line = true;
        }
        has_flow_content = true;
        line_height = if flow.compatibility_mode < 15 && compatibility_forced_shape_line {
          shape.height_pt.max(LAYOUT_EPSILON_PT)
        } else {
          line_height.max(shape.height_pt)
        };
        x = shape.width_pt;
      }
      InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::LastRenderedPageBreak => {}
      InlineItem::PageBreak | InlineItem::ColumnBreak => {
        finish_line(&mut content_height, &mut line_height, &mut line_index);
        x = 0.0;
      }
    }
  }
  if has_flow_content || floating_bottom > 0.0 {
    finish_line(&mut content_height, &mut line_height, &mut line_index);
  }
  content_height = content_height.max(floating_bottom);

  content_height
}

fn estimated_paragraph_first_line_height(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let base_line_style = paragraph_base_line_style(paragraph);
  paragraph.format.spacing_before_pt
    + paragraph_line_height_for_setup(
      paragraph,
      &base_line_style,
      flow.setup,
      flow.text_segmentation,
      text_metrics,
    )
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

fn page_has_body_region_items(page: &Page, flow: FlowContext) -> bool {
  // Page-break movement is based on body text/frame progress. Header/footer
  // content and page decorations may already be present on the physical page,
  // but they must not make body layout create an extra empty body page.
  page.items.iter().any(|item| match item {
    PageItem::Text(text) => {
      text.y_pt >= flow.content_top_pt - LAYOUT_EPSILON_PT
        && text.y_pt <= flow.content_bottom + LAYOUT_EPSILON_PT
    }
    PageItem::Image(image) => {
      image.y_pt >= flow.content_top_pt - LAYOUT_EPSILON_PT
        && image.y_pt <= flow.content_bottom + LAYOUT_EPSILON_PT
    }
    PageItem::Rect(rect) => {
      rect.y_pt >= flow.content_top_pt - LAYOUT_EPSILON_PT
        && rect.y_pt <= flow.content_bottom + LAYOUT_EPSILON_PT
    }
    PageItem::Line(line) => {
      line.y1_pt.min(line.y2_pt) >= flow.content_top_pt - LAYOUT_EPSILON_PT
        && line.y1_pt.min(line.y2_pt) <= flow.content_bottom + LAYOUT_EPSILON_PT
    }
    PageItem::Polyline(polyline) => polyline.points.iter().any(|(_, y)| {
      *y >= flow.content_top_pt - LAYOUT_EPSILON_PT && *y <= flow.content_bottom + LAYOUT_EPSILON_PT
    }),
    PageItem::Fill(_) => false,
  })
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
  if let Some(Block::Paragraph(previous)) = previous {
    return paragraph_spacing_before_after_previous(previous, paragraph);
  }
  paragraph.format.spacing_before_pt
}

fn paragraph_lower_space(paragraph: &crate::docx::Paragraph) -> f32 {
  paragraph
    .format
    .spacing_after_pt
    .max(PARAGRAPH_SPACING_AFTER_PT)
}

fn paragraph_spacing_before_after_previous(
  previous: &crate::docx::Paragraph,
  paragraph: &crate::docx::Paragraph,
) -> f32 {
  // SwFlowFrame::CalcUpperSpace() collapses normal inter-paragraph spacing to
  // max(previous lower, current upper), with contextual-spacing exceptions for
  // identical paragraph styles.
  let lower = paragraph_lower_space(previous);
  if paragraph_styles_identical(previous, paragraph) {
    match (
      previous.format.contextual_spacing,
      paragraph.format.contextual_spacing,
    ) {
      (true, true) => return 0.0,
      (false, true) => return 0.0,
      (true, false) => return paragraph.format.spacing_before_pt,
      (false, false) => {}
    }
  }
  (paragraph.format.spacing_before_pt - lower).max(0.0)
}

fn section_start_spacing_before(
  previous: &crate::docx::Paragraph,
  paragraph: &crate::docx::Paragraph,
) -> f32 {
  if paragraph.format.contextual_spacing && paragraph_styles_identical(previous, paragraph) {
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
        InlineItem::BookmarkStart(_)
          | InlineItem::FormWidgetStart(_)
          | InlineItem::FormWidgetEnd(_)
      )
    })
    .is_some_and(|inline| matches!(inline, InlineItem::LastRenderedPageBreak))
}

fn segment_affects_line_height(text: &str) -> bool {
  !text.is_empty() && !text.chars().all(libreoffice_ignored_line_height_blank)
}

fn libreoffice_ignored_line_height_blank(ch: char) -> bool {
  matches!(ch, ' ' | '\u{2002}' | '\u{2003}' | '\u{2005}' | '\u{3000}')
}

fn paragraph_spacing_after(paragraph: &crate::docx::Paragraph, next: Option<&Block>) -> f32 {
  if let Some(Block::Paragraph(next)) = next
    && paragraph_styles_identical(paragraph, next)
  {
    match (
      paragraph.format.contextual_spacing,
      next.format.contextual_spacing,
    ) {
      (true, true) | (true, false) => return 0.0,
      (false, true) | (false, false) => {}
    }
  }
  paragraph_lower_space(paragraph)
}

fn paragraph_styles_identical(
  first: &crate::docx::Paragraph,
  second: &crate::docx::Paragraph,
) -> bool {
  match (&first.format.style_id, &second.format.style_id) {
    (Some(first_style), Some(second_style)) => first_style == second_style,
    (None, None) => true,
    _ => false,
  }
}

struct LayoutBlockTarget<'a> {
  current: &'a mut Page,
  pages: &'a mut Vec<Page>,
  anchor_pages: Option<&'a mut Vec<AnchorPage>>,
  text_metrics: &'a mut TextMetrics,
}

fn layout_document_block(
  previous: Option<&Block>,
  block: &Block,
  next: Option<&Block>,
  flow: FlowContext,
  target: LayoutBlockTarget<'_>,
  mut y: f32,
) -> (FlowContext, f32) {
  match block {
    Block::Paragraph(paragraph) => {
      let mut ignore_top_margin_at_page_start = false;
      if let Some(frame) = paragraph_frame(paragraph) {
        note_body_content_frame(target.current, flow);
        return layout_floating_frame(
          &frame,
          flow,
          target.current,
          target.pages,
          target.text_metrics,
          y,
        );
      }
      let mut flow = flow;
      let ignore_top_margin_after_page_break = paragraph.format.page_break_before
        && page_has_body_region_items(target.current, flow)
        && flow.text_segmentation == TextSegmentation::Body;
      if paragraph.format.page_break_before
        && page_has_body_region_items(target.current, flow)
        && flow.text_segmentation == TextSegmentation::Body
      {
        (flow, y) = force_page_break(flow, target.current, target.pages);
        ignore_top_margin_at_page_start = true;
      }
      note_body_content_frame(target.current, flow);

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
      y += paragraph_border_layout_extent(paragraph.format.borders.top);
      let (paragraph_flow, y) = layout_paragraph(
        paragraph,
        flow,
        ParagraphLayoutTarget {
          current: target.current,
          pages: target.pages,
          anchor_pages: target.anchor_pages,
          text_metrics: target.text_metrics,
        },
        y,
        paragraph_spacing_after(paragraph, next),
      );
      let y = y + paragraph_border_layout_extent(paragraph.format.borders.bottom);
      (
        FlowContext {
          content_width: flow.content_width,
          ..paragraph_flow
        },
        y,
      )
    }
    Block::Table(table) => {
      let has_ind_prev = table_has_indirect_previous_frame(target.current, flow, y);
      note_body_content_frame(target.current, flow);
      layout_table(
        table,
        flow,
        target.current,
        target.pages,
        target.text_metrics,
        y,
        has_ind_prev,
      )
    }
    Block::Frame(frame) => {
      note_body_content_frame(target.current, flow);
      layout_floating_frame(
        frame,
        flow,
        target.current,
        target.pages,
        target.text_metrics,
        y,
      )
    }
  }
}

fn paragraph_border_layout_extent(border: Option<BorderStyle>) -> f32 {
  border.map_or(0.0, |border| {
    border.spacing_pt + border.width_pt + if border.shadow { border.width_pt } else { 0.0 }
  })
}

fn table_has_indirect_previous_frame(page: &Page, flow: FlowContext, y: f32) -> bool {
  // SwTabFrame::MakeAll() uses GetIndPrev() before the table is split. The
  // current table frame itself must not count as the previous frame.
  if flow.text_segmentation == TextSegmentation::Body {
    return page.body_content_frames > 0;
  }
  page_has_body_region_items(page, flow) || y > flow.content_top_pt + LAYOUT_EPSILON_PT
}

fn note_body_content_frame(current: &mut Page, flow: FlowContext) {
  // sw::IsPageFrameEmpty() keeps pages whose body frame ContainsContent().
  // A Writer content frame exists for an empty paragraph as well; PDF painted
  // items are therefore insufficient for deciding if a body page is empty.
  if flow.text_segmentation == TextSegmentation::Body {
    current.body_content_frames = current.body_content_frames.saturating_add(1);
  }
}

fn paragraph_frame(paragraph: &crate::docx::Paragraph) -> Option<FloatingFrame> {
  let frame = paragraph.format.frame?;
  let mut content = paragraph.clone();
  content.format.frame = None;
  Some(FloatingFrame {
    blocks: vec![Block::paragraph(content)],
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
  text_metrics: &mut TextMetrics,
  y: f32,
) -> (FlowContext, f32) {
  let width = frame
    .width_pt
    .unwrap_or_else(|| default_floating_frame_width(frame.placement, flow))
    .max(DEFAULT_FONT_SIZE_PT);
  let height = estimated_frame_height(frame, flow, text_metrics);
  let (x, frame_y) = paragraph_frame_position(frame.placement, flow, y, width, height);
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
      LayoutBlockTarget {
        current: &mut frame_page,
        pages: &mut frame_pages,
        anchor_pages: None,
        text_metrics: &mut *text_metrics,
      },
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

fn frame_content_height(
  frame: &FloatingFrame,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> f32 {
  frame
    .blocks
    .iter()
    .map(|block| estimated_block_height(block, flow, text_metrics))
    .sum::<f32>()
    .max(DEFAULT_LINE_HEIGHT_PT)
}

fn estimated_frame_height(
  frame: &FloatingFrame,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> f32 {
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
    text_metrics,
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
    compatibility_mode: flow.compatibility_mode,
    justify_lines_with_shrinking: flow.justify_lines_with_shrinking,
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
    layout_cell_print_bounds: None,
    default_tab_stop_pt: area.default_tab_stop_pt,
    compatibility_mode: area.compatibility_mode,
    justify_lines_with_shrinking: area.justify_lines_with_shrinking,
    split_page_break_and_paragraph_mark: false,
    repeating_slots: area.repeating_slots,
    text_segmentation: TextSegmentation::Body,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
    preserve_horizontal_on_advance: false,
    script_sensitive_line_height: true,
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
  text_metrics: &mut TextMetrics,
) -> (FlowContext, Option<f32>) {
  let height =
    referenced_footnote_area_height(block, document, emitted_footnotes, flow, text_metrics);
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
  text_metrics: &mut TextMetrics,
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
    height += measured_note_blocks_height(blocks, flow, text_metrics);
  }
  height
}

fn measured_note_blocks_height(
  blocks: &[Block],
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
) -> f32 {
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
      LayoutBlockTarget {
        current: &mut scratch,
        pages: &mut discarded_pages,
        anchor_pages: None,
        text_metrics: &mut *text_metrics,
      },
      y,
    );
    y = next_y;
  }
  y
}

struct FootnoteEmission<'a> {
  emitted_footnotes: &'a mut HashSet<i64>,
  emitted_footnote_order: &'a mut Vec<i64>,
}

struct FootnoteLayoutTarget<'a> {
  current: &'a mut Page,
  pages: &'a mut Vec<Page>,
  text_metrics: &'a mut TextMetrics,
}

fn footnote_boss_format(
  block: &Block,
  document: &DocxDocument,
  emission: FootnoteEmission<'_>,
  flow: FlowContext,
  target: FootnoteLayoutTarget<'_>,
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
    if !emission.emitted_footnotes.insert(*id) {
      continue;
    }
    emission.emitted_footnote_order.push(*id);
    let Some(blocks) = document.footnotes.get(id) else {
      continue;
    };
    if needs_separator {
      y = layout_note_separator(
        NoteSeparatorKind::Footnote,
        footnote_flow.setup,
        target.current,
        target.pages,
        y,
        footnote_flow.content_bottom,
      );
      needs_separator = false;
    }
    for (index, block) in blocks.iter().enumerate() {
      let previous = index.checked_sub(1).and_then(|index| blocks.get(index));
      let next = blocks.get(index + 1);
      let (_, next_y) = layout_document_block(
        previous,
        block,
        next,
        footnote_flow,
        LayoutBlockTarget {
          current: target.current,
          pages: target.pages,
          anchor_pages: None,
          text_metrics: &mut *target.text_metrics,
        },
        y,
      );
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
  let header_blocks = if first_page_in_section && title_page {
    first_header
  } else if use_even_slot {
    even_header
  } else {
    default_header
  };
  let footer_blocks = if first_page_in_section && title_page {
    first_footer
  } else if use_even_slot {
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
  text_metrics: &mut TextMetrics,
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

  let header_top = page.setup.header_distance_pt.max(0.0);
  let header_repeating_slots = repeating_slot_state(document, page.section_index, text_metrics);
  layout_repeating_blocks_into_page(
    header_blocks,
    &mut adornment,
    &mut discarded_pages,
    text_metrics,
    header_top,
    FlowContext {
      setup: page.setup,
      section_index: page.section_index,
      section_page_index: page.section_page_index,
      column_index: 0,
      columns: SectionColumns::default(),
      content_top_pt: header_top,
      content_left_pt: page.setup.margin_left_pt,
      content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
      body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
      content_width,
      layout_cell_bounds: None,
      layout_cell_print_bounds: None,
      default_tab_stop_pt: document.default_tab_stop_pt,
      compatibility_mode: document.compatibility_mode,
      justify_lines_with_shrinking: document.justify_lines_with_shrinking,
      split_page_break_and_paragraph_mark: document.split_page_break_and_paragraph_mark,
      repeating_slots: header_repeating_slots,
      text_segmentation: TextSegmentation::RepeatingSlot,
      paragraph_spacing_context: ParagraphSpacingContext::Normal,
      preserve_horizontal_on_advance: false,
      script_sensitive_line_height: true,
    },
  );

  let footer_repeating_slots = repeating_slot_state(document, page.section_index, text_metrics);
  let (_, _, _, footer_height, _) =
    repeating_slots_present_for_page(footer_repeating_slots, page_number, page.section_page_index);
  let footer_top = footer_content_top(page.setup, footer_height, !document.has_styles_part);
  layout_repeating_blocks_into_page(
    footer_blocks,
    &mut adornment,
    &mut discarded_pages,
    text_metrics,
    footer_top,
    FlowContext {
      setup: page.setup,
      section_index: page.section_index,
      section_page_index: page.section_page_index,
      column_index: 0,
      columns: SectionColumns::default(),
      content_top_pt: footer_top,
      content_left_pt: page.setup.margin_left_pt,
      content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
      body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
      content_width,
      layout_cell_bounds: None,
      layout_cell_print_bounds: None,
      default_tab_stop_pt: document.default_tab_stop_pt,
      compatibility_mode: document.compatibility_mode,
      justify_lines_with_shrinking: document.justify_lines_with_shrinking,
      split_page_break_and_paragraph_mark: document.split_page_break_and_paragraph_mark,
      repeating_slots: footer_repeating_slots,
      text_segmentation: TextSegmentation::RepeatingSlot,
      paragraph_spacing_context: ParagraphSpacingContext::Normal,
      preserve_horizontal_on_advance: false,
      script_sensitive_line_height: true,
    },
  );

  adornment.wrap_exclusions
}

fn repeating_wrap_exclusion_catalog_for_page(
  document: &DocxDocument,
  page: &Page,
  text_metrics: &mut TextMetrics,
) -> RepeatingWrapExclusionCatalog {
  let first_page_odd = empty_section_page(page.setup, page.section_index, 0);
  let first_page_even = empty_section_page(page.setup, page.section_index, 0);
  let default_page = empty_section_page(page.setup, page.section_index, 1);
  let even_page = empty_section_page(page.setup, page.section_index, 1);
  RepeatingWrapExclusionCatalog {
    first_odd: repeating_slot_wrap_exclusions_for_page(document, &first_page_odd, 1, text_metrics),
    first_even: repeating_slot_wrap_exclusions_for_page(
      document,
      &first_page_even,
      2,
      text_metrics,
    ),
    even: repeating_slot_wrap_exclusions_for_page(document, &even_page, 2, text_metrics),
    default: repeating_slot_wrap_exclusions_for_page(document, &default_page, 1, text_metrics),
  }
}

fn layout_repeating_blocks_into_page(
  blocks: &[Block],
  page: &mut Page,
  discarded_pages: &mut Vec<Page>,
  text_metrics: &mut TextMetrics,
  mut y: f32,
  flow: FlowContext,
) -> f32 {
  for (index, block) in blocks.iter().enumerate() {
    y = layout_repeating_block(
      block,
      page,
      discarded_pages,
      text_metrics,
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

fn apply_headers_and_footers(
  document: &DocxDocument,
  pages: &mut [Page],
  text_metrics: &mut TextMetrics,
) {
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
    let header_top = page.setup.header_distance_pt.max(0.0);
    let header_repeating_slots = repeating_slot_state(document, page.section_index, text_metrics);
    layout_repeating_blocks_into_page(
      header_blocks,
      &mut adornment,
      &mut discarded_pages,
      text_metrics,
      header_top,
      FlowContext {
        setup: page.setup,
        section_index: page.section_index,
        section_page_index: page.section_page_index,
        column_index: 0,
        columns: SectionColumns::default(),
        content_top_pt: header_top,
        content_left_pt: page.setup.margin_left_pt,
        content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
        body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
        content_width,
        layout_cell_bounds: None,
        layout_cell_print_bounds: None,
        default_tab_stop_pt: document.default_tab_stop_pt,
        compatibility_mode: document.compatibility_mode,
        justify_lines_with_shrinking: document.justify_lines_with_shrinking,
        split_page_break_and_paragraph_mark: document.split_page_break_and_paragraph_mark,
        repeating_slots: header_repeating_slots,
        text_segmentation: TextSegmentation::RepeatingSlot,
        paragraph_spacing_context: ParagraphSpacingContext::Normal,
        preserve_horizontal_on_advance: false,
        script_sensitive_line_height: true,
      },
    );

    let footer_repeating_slots = repeating_slot_state(document, page.section_index, text_metrics);
    let (_, _, _, footer_height, _) =
      repeating_slots_present_for_page(footer_repeating_slots, index + 1, page.section_page_index);
    let footer_top = footer_content_top(page.setup, footer_height, !document.has_styles_part);
    layout_repeating_blocks_into_page(
      footer_blocks,
      &mut adornment,
      &mut discarded_pages,
      text_metrics,
      footer_top,
      FlowContext {
        setup: page.setup,
        section_index: page.section_index,
        section_page_index: page.section_page_index,
        column_index: 0,
        columns: SectionColumns::default(),
        content_top_pt: footer_top,
        content_left_pt: page.setup.margin_left_pt,
        content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
        body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
        content_width,
        layout_cell_bounds: None,
        layout_cell_print_bounds: None,
        default_tab_stop_pt: document.default_tab_stop_pt,
        compatibility_mode: document.compatibility_mode,
        justify_lines_with_shrinking: document.justify_lines_with_shrinking,
        split_page_break_and_paragraph_mark: document.split_page_break_and_paragraph_mark,
        repeating_slots: footer_repeating_slots,
        text_segmentation: TextSegmentation::RepeatingSlot,
        paragraph_spacing_context: ParagraphSpacingContext::Normal,
        preserve_horizontal_on_advance: false,
        script_sensitive_line_height: true,
      },
    );

    extend_wrap_exclusions_unique(&mut page.wrap_exclusions, &adornment.wrap_exclusions);
    page.repeating_adornment = Some(RepeatingAdornment {
      items: adornment.items,
      frame_fragments: adornment.frame_fragments,
      frame_influences: adornment.frame_influences,
    });
  }
}

fn materialize_repeating_adornments(pages: &mut [Page], frames: &mut [LayoutFrame]) {
  for (page_index, page) in pages.iter_mut().enumerate() {
    let insertion_index = usize::from(page.items.first().is_some_and(|item| {
      matches!(item, PageItem::Fill(fill) if fill.x_pt == 0.0
        && fill.y_pt == 0.0
        && fill.width_pt == page.setup.width_pt
        && fill.height_pt == page.setup.height_pt)
    }));
    let Some(mut adornment) = page.repeating_adornment.take() else {
      offset_body_frame_records_for_prefix(page, frames, page_index, insertion_index);
      continue;
    };
    let adornment_item_count = adornment.items.len();
    offset_body_frame_records_for_prefix(
      page,
      frames,
      page_index,
      insertion_index + adornment_item_count,
    );
    offset_page_frame_records_raw(
      &mut adornment.frame_fragments,
      &mut adornment.frame_influences,
      insertion_index,
    );
    page
      .items
      .splice(insertion_index..insertion_index, adornment.items);
    page.frame_fragments.extend(adornment.frame_fragments);
    page.frame_influences.extend(adornment.frame_influences);
  }
}

fn offset_body_frame_records_for_prefix(
  page: &mut Page,
  frames: &mut [LayoutFrame],
  page_index: usize,
  item_offset: usize,
) {
  if item_offset == 0 {
    return;
  }
  offset_page_frame_records(page, item_offset);
  for frame in frames
    .iter_mut()
    .filter(|frame| frame.page_index == page_index)
  {
    frame.item_start = frame.item_start.saturating_add(item_offset);
    frame.item_end = frame.item_end.saturating_add(item_offset);
    for line in &mut frame.lines {
      line.item_start = line.item_start.saturating_add(item_offset);
      line.item_end = line.item_end.saturating_add(item_offset);
    }
    for fragment in &mut frame.fragments {
      fragment.item_start = fragment.item_start.saturating_add(item_offset);
      fragment.item_end = fragment.item_end.saturating_add(item_offset);
    }
    for influence in &mut frame.influences {
      influence.item_start = influence.item_start.saturating_add(item_offset);
      influence.item_end = influence.item_end.saturating_add(item_offset);
    }
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

fn blocks_have_footnote_references(blocks: &[Block]) -> bool {
  blocks.iter().any(block_has_footnote_references)
}

fn block_has_footnote_references(block: &Block) -> bool {
  match block {
    Block::Paragraph(paragraph) => !paragraph.footnote_reference_ids.is_empty(),
    Block::Table(table) => table.rows.iter().any(|row| {
      row
        .cells
        .iter()
        .any(|cell| blocks_have_footnote_references(&cell.blocks))
    }),
    Block::Frame(frame) => blocks_have_footnote_references(&frame.blocks),
  }
}

fn block_has_visible_body_content(block: &Block) -> bool {
  match block {
    Block::Paragraph(paragraph) => paragraph.inlines.iter().any(|inline| match inline {
      InlineItem::Text(run) => !run.text.trim().is_empty(),
      InlineItem::Image(_) | InlineItem::Shape(_) => true,
      InlineItem::BookmarkStart(_) => false,
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

fn resolve_dynamic_fields(
  pages: &mut [Page],
  anchor_pages: &[AnchorPage],
  footnote_numbering: &[NoteNumberingSpec],
  endnote_numbering: &[NoteNumberingSpec],
) {
  let total_pages = pages.len();
  let page_refs = anchor_pages
    .iter()
    .map(|anchor| {
      (
        anchor.name.as_str(),
        anchor.virtual_page_number.max(1).to_string(),
      )
    })
    .collect::<HashMap<_, _>>();
  let style_ref_candidates = style_ref_candidates_by_page(pages);
  resolve_page_note_numbering(pages, footnote_numbering, endnote_numbering);
  for (page_index, page) in pages.iter_mut().enumerate() {
    let page_number = page_index + 1;
    let items = page.items.iter_mut().chain(
      page
        .repeating_adornment
        .iter_mut()
        .flat_map(|adornment| adornment.items.iter_mut()),
    );
    for item in items {
      let PageItem::Text(text) = item else {
        continue;
      };
      match &text.dynamic_field {
        Some(DynamicFieldKind::Page { number_format }) => {
          text.text = super::format_field_number(page_number, *number_format);
        }
        Some(DynamicFieldKind::NumPages { number_format }) => {
          text.text = super::format_field_number(total_pages, *number_format);
        }
        Some(DynamicFieldKind::PageRef { bookmark_name }) => {
          if let Some(page_number) = page_refs.get(bookmark_name.as_ref()) {
            text.text.clone_from(page_number);
          }
        }
        Some(DynamicFieldKind::StyleRef {
          style_name,
          from_bottom,
          suppress_non_numerical,
        }) => {
          if let Some(value) = resolve_style_ref(
            &style_ref_candidates,
            page_index,
            style_name,
            *from_bottom,
            *suppress_non_numerical,
          ) {
            text.text = value;
          }
        }
        None => {}
      }
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum PageNoteKind {
  Footnote,
  Endnote,
}

fn resolve_page_note_numbering(
  pages: &mut [Page],
  footnote_numbering: &[NoteNumberingSpec],
  endnote_numbering: &[NoteNumberingSpec],
) {
  for page in pages {
    let footnote_spec = footnote_numbering.get(page.section_index).copied();
    let endnote_spec = endnote_numbering.get(page.section_index).copied();
    if !footnote_spec.is_some_and(note_numbering_restarts_each_page)
      && !endnote_spec.is_some_and(note_numbering_restarts_each_page)
    {
      continue;
    }

    let mut labels = HashMap::default();
    let mut next_footnote = footnote_spec.map_or(1, |spec| spec.start);
    let mut next_endnote = endnote_spec.map_or(1, |spec| spec.start);
    for item in page.items.iter().chain(
      page
        .repeating_adornment
        .iter()
        .flat_map(|adornment| adornment.items.iter()),
    ) {
      let PageItem::Text(text) = item else {
        continue;
      };
      let Some((kind, false, id)) = text.hyperlink_url.as_deref().and_then(page_note_link_parts)
      else {
        continue;
      };
      let (spec, next) = match kind {
        PageNoteKind::Footnote => (footnote_spec, &mut next_footnote),
        PageNoteKind::Endnote => (endnote_spec, &mut next_endnote),
      };
      let Some(spec) = spec.filter(|spec| note_numbering_restarts_each_page(*spec)) else {
        continue;
      };
      labels.entry((kind, id)).or_insert_with(|| {
        let label = format_page_note_number(kind, spec, *next);
        *next = next.saturating_add(1);
        label
      });
    }

    let items = page.items.iter_mut().chain(
      page
        .repeating_adornment
        .iter_mut()
        .flat_map(|adornment| adornment.items.iter_mut()),
    );
    for item in items {
      let PageItem::Text(text) = item else {
        continue;
      };
      let Some((kind, backlink, id)) = text.hyperlink_url.as_deref().and_then(page_note_link_parts)
      else {
        continue;
      };
      let Some(label) = labels.get(&(kind, id)) else {
        continue;
      };
      if backlink {
        text.text = format!("{label} ");
      } else {
        text.text.clone_from(label);
      }
    }
  }
}

fn note_numbering_restarts_each_page(spec: NoteNumberingSpec) -> bool {
  matches!(spec.restart, w::RestartNumberValues::EachPage)
}

fn format_page_note_number(kind: PageNoteKind, spec: NoteNumberingSpec, value: i32) -> String {
  let default_format = match kind {
    PageNoteKind::Footnote => w::NumberFormatValues::Decimal,
    PageNoteKind::Endnote => w::NumberFormatValues::LowerRoman,
  };
  let format = if matches!(spec.format, w::NumberFormatValues::None) {
    default_format
  } else {
    spec.format
  };
  super::format_numbering_value(value, format, false)
}

fn page_note_link_parts(url: &str) -> Option<(PageNoteKind, bool, i64)> {
  let rest = url.strip_prefix("ooxmlsdk-pdf:")?;
  let (kind, id) = rest.rsplit_once(':')?;
  let (kind, backlink) = match kind {
    "footnote-reference" => (PageNoteKind::Footnote, false),
    "footnote-backlink" => (PageNoteKind::Footnote, true),
    "endnote-reference" => (PageNoteKind::Endnote, false),
    "endnote-backlink" => (PageNoteKind::Endnote, true),
    _ => return None,
  };
  Some((kind, backlink, id.parse().ok()?))
}

#[derive(Clone, Debug)]
struct StyleRefCandidate {
  y_pt: f32,
  keys: Vec<Arc<str>>,
  text: Arc<str>,
  numbering_text: Option<Arc<str>>,
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
            && candidate.numbering_text.as_deref() == text.style_ref_numbering_text.as_deref()
        }) {
          continue;
        }
        candidates.push(StyleRefCandidate {
          y_pt: text.y_pt,
          keys: text.style_ref_keys.clone(),
          text: style_ref_text.clone(),
          numbering_text: text.style_ref_numbering_text.clone(),
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
  suppress_non_numerical: bool,
) -> Option<String> {
  if let Some(text) = resolve_style_ref_on_page(
    &pages[page_index],
    style_name,
    from_bottom,
    suppress_non_numerical,
  ) {
    return Some(text);
  }
  for previous_index in (0..page_index).rev() {
    if let Some(text) = resolve_style_ref_on_page(
      &pages[previous_index],
      style_name,
      true,
      suppress_non_numerical,
    ) {
      return Some(text);
    }
  }
  for next_page in pages.iter().skip(page_index + 1) {
    if let Some(text) =
      resolve_style_ref_on_page(next_page, style_name, false, suppress_non_numerical)
    {
      return Some(text);
    }
  }
  None
}

fn resolve_style_ref_on_page(
  candidates: &[StyleRefCandidate],
  style_name: &str,
  from_bottom: bool,
  suppress_non_numerical: bool,
) -> Option<String> {
  let iter: Box<dyn Iterator<Item = &StyleRefCandidate> + '_> = if from_bottom {
    Box::new(candidates.iter().rev())
  } else {
    Box::new(candidates.iter())
  };
  iter
    .filter(|candidate| style_ref_candidate_matches(candidate, style_name))
    .map(|candidate| {
      if suppress_non_numerical {
        candidate
          .numbering_text
          .as_deref()
          .unwrap_or_default()
          .to_string()
      } else {
        candidate.text.to_string()
      }
    })
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
  let mut section_bounds = HashMap::<(usize, usize), (f32, f32)>::default();
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
  let (has_header, has_footer, header_height, footer_height, first_header) =
    repeating_slots_present_for_page(slots, page_number, section_page_index);

  if has_header && !setup.top_margin_was_negative {
    top = top.max(header_slot_bottom(setup));
    if !first_header {
      top = top.max(setup.header_distance_pt.max(0.0) + header_height);
    }
  }
  if has_footer && !setup.bottom_margin_was_negative {
    let footer_bottom = (setup.height_pt - setup.footer_distance_pt.max(0.0))
      .max(0.0)
      .min(setup.height_pt);
    bottom = bottom.min(footer_slot_top(setup).min((footer_bottom - footer_height).max(0.0)));
  }
  // ECMA-376 Part 1 §17.6.5 defines `w:linePitch` as the pitch of each
  // document-grid line; it does not move the section's text area. LibreOffice
  // likewise skips body-grid centering when MS_WORD_COMP_GRID_METRICS is set,
  // which its OOXML importer enables for newly imported Word documents.
  if bottom < top + DEFAULT_LINE_HEIGHT_PT {
    bottom = (top + DEFAULT_LINE_HEIGHT_PT).min(setup.height_pt);
  }

  (top, bottom)
}

fn header_slot_bottom(setup: PageSetup) -> f32 {
  setup.header_distance_pt.max(0.0)
    + header_footer_slot_height(setup.margin_top_pt, setup.header_distance_pt)
}

fn footer_slot_top(setup: PageSetup) -> f32 {
  let footer_bottom = (setup.height_pt - setup.footer_distance_pt.max(0.0))
    .max(0.0)
    .min(setup.height_pt);
  (footer_bottom - header_footer_slot_height(setup.margin_bottom_pt, setup.footer_distance_pt))
    .max(0.0)
    .min(setup.height_pt)
}

fn footer_content_top(
  setup: PageSetup,
  content_height_pt: f32,
  use_missing_styles_alignment: bool,
) -> f32 {
  // ECMA-376 Part 1 §17.6.11 defines `w:footer` as the distance from the
  // bottom edge of the page to the bottom edge of the footer. Word centers
  // short footer content in the nominal slot between that edge and the body
  // bottom margin (observable in the Office fixed output for a style-less,
  // single-line PAGE field). Content taller than the slot still grows upward
  // while retaining the specified footer bottom edge.
  let footer_bottom = (setup.height_pt - setup.footer_distance_pt.max(0.0))
    .max(0.0)
    .min(setup.height_pt);
  let content_height = content_height_pt.max(0.0);
  let slot_height = header_footer_slot_height(setup.margin_bottom_pt, setup.footer_distance_pt);
  let top = if use_missing_styles_alignment && content_height <= slot_height {
    footer_slot_top(setup) + (slot_height - content_height) / 2.0
  } else {
    footer_bottom - content_height
  };
  top.max(0.0).min(setup.height_pt)
}

fn header_footer_slot_height(margin_pt: f32, distance_pt: f32) -> f32 {
  // Header/footer slots are imported from the page margin and distance, not
  // from the measured content in the header/footer stream.
  (margin_pt - distance_pt.max(0.0)).max(units::millimeters_to_points(1.0))
}

fn repeating_slot_state(
  document: &DocxDocument,
  section_index: usize,
  text_metrics: &mut TextMetrics,
) -> RepeatingSlotState {
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
        text_metrics,
      ),
      default_footer_height_pt: measured_repeating_blocks_height(
        &section.footer_blocks,
        section.page,
        document.default_tab_stop_pt,
        text_metrics,
      ),
      first_header_height_pt: measured_repeating_blocks_height(
        &section.first_header_blocks,
        section.page,
        document.default_tab_stop_pt,
        text_metrics,
      ),
      first_footer_height_pt: measured_repeating_blocks_height(
        &section.first_footer_blocks,
        section.page,
        document.default_tab_stop_pt,
        text_metrics,
      ),
      even_header_height_pt: measured_repeating_blocks_height(
        &section.even_header_blocks,
        section.page,
        document.default_tab_stop_pt,
        text_metrics,
      ),
      even_footer_height_pt: measured_repeating_blocks_height(
        &section.even_footer_blocks,
        section.page,
        document.default_tab_stop_pt,
        text_metrics,
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
      text_metrics,
    ),
    default_footer_height_pt: measured_repeating_blocks_height(
      &document.footer_blocks,
      document.page,
      document.default_tab_stop_pt,
      text_metrics,
    ),
    first_header_height_pt: measured_repeating_blocks_height(
      &document.first_header_blocks,
      document.page,
      document.default_tab_stop_pt,
      text_metrics,
    ),
    first_footer_height_pt: measured_repeating_blocks_height(
      &document.first_footer_blocks,
      document.page,
      document.default_tab_stop_pt,
      text_metrics,
    ),
    even_header_height_pt: measured_repeating_blocks_height(
      &document.header_blocks,
      document.page,
      document.default_tab_stop_pt,
      text_metrics,
    ),
    even_footer_height_pt: measured_repeating_blocks_height(
      &document.footer_blocks,
      document.page,
      document.default_tab_stop_pt,
      text_metrics,
    ),
  }
}

fn repeating_slots_present_for_page(
  slots: RepeatingSlotState,
  page_number: usize,
  section_page_index: usize,
) -> (bool, bool, f32, f32, bool) {
  let first_page_in_section = section_page_index == 0;
  let use_even_slot = slots.even_and_odd_headers && page_number.is_multiple_of(2);
  let (header, header_height, first_header) = selected_repeating_slot(
    first_page_in_section,
    use_even_slot,
    slots.title_page,
    (slots.first_header, slots.first_header_height_pt),
    (slots.even_header, slots.even_header_height_pt),
    (slots.default_header, slots.default_header_height_pt),
  );
  let (footer, footer_height, _) = selected_repeating_slot(
    first_page_in_section,
    use_even_slot,
    slots.title_page,
    (slots.first_footer, slots.first_footer_height_pt),
    (slots.even_footer, slots.even_footer_height_pt),
    (slots.default_footer, slots.default_footer_height_pt),
  );
  (header, footer, header_height, footer_height, first_header)
}

fn selected_repeating_slot(
  first_page_in_section: bool,
  use_even_slot: bool,
  title_page: bool,
  first: (bool, f32),
  even: (bool, f32),
  default_: (bool, f32),
) -> (bool, f32, bool) {
  if first_page_in_section && title_page {
    return (first.0, first.1, true);
  }
  if use_even_slot {
    return (even.0, even.1, false);
  }
  if default_.0 {
    return (default_.0, default_.1, false);
  }
  (false, 0.0, false)
}

fn measured_repeating_blocks_height(
  blocks: &[Block],
  setup: PageSetup,
  default_tab_stop_pt: f32,
  text_metrics: &mut TextMetrics,
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
    layout_cell_print_bounds: None,
    default_tab_stop_pt,
    compatibility_mode: 12,
    justify_lines_with_shrinking: false,
    split_page_break_and_paragraph_mark: false,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::RepeatingSlot,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
    preserve_horizontal_on_advance: false,
    script_sensitive_line_height: true,
  };
  let mut y = 0.0;
  for (index, block) in blocks.iter().enumerate() {
    y = layout_repeating_block(
      block,
      &mut scratch,
      &mut discarded_pages,
      text_metrics,
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
  text_metrics: &mut TextMetrics,
  y: f32,
  flow: FlowContext,
  is_last_repeating_block: bool,
) -> f32 {
  match block {
    Block::Paragraph(paragraph) => {
      let spacing_after_pt = if is_last_repeating_block {
        // SwFlowFrame::CalcLowerSpace(), tdf#128195 branch:
        // the text frame already carries its normal paragraph lower spacing,
        // then the header/footer compatibility branch adds the last
        // paragraph's lower spacing again plus SwBorderAttrs::CalcLineSpacing().
        let lower_space = paragraph
          .format
          .spacing_after_pt
          .max(PARAGRAPH_SPACING_AFTER_PT);
        lower_space + lower_space + paragraph_line_spacing_excess(paragraph)
      } else {
        paragraph
          .format
          .spacing_after_pt
          .max(PARAGRAPH_SPACING_AFTER_PT)
      };
      let (_, y) = layout_paragraph(
        paragraph,
        flow,
        ParagraphLayoutTarget {
          current: page,
          pages: discarded_pages,
          anchor_pages: None,
          text_metrics,
        },
        y + paragraph.format.spacing_before_pt,
        spacing_after_pt,
      );
      y
    }
    Block::Table(table) => {
      let has_ind_prev = table_has_indirect_previous_frame(page, flow, y);
      let (_, y) = layout_table(
        table,
        flow,
        page,
        discarded_pages,
        text_metrics,
        y,
        has_ind_prev,
      );
      y
    }
    Block::Frame(frame) => {
      let (_, y) = layout_floating_frame(frame, flow, page, discarded_pages, text_metrics, y);
      y
    }
  }
}

fn layout_table(
  table: &Table,
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  text_metrics: &mut TextMetrics,
  y: f32,
  has_ind_prev: bool,
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
    return TableFrameLayout::new(&inline_table, block_area(flow), false, text_metrics)
      .map_or((flow, y), |layout| {
        layout.format(current, pages, text_metrics, y, has_ind_prev)
      });
  }
  if table.placement.is_some() {
    return layout_floating_table(table, flow, current, pages, text_metrics, y);
  }
  TableFrameLayout::new(table, block_area(flow), false, text_metrics).map_or((flow, y), |layout| {
    layout.format(current, pages, text_metrics, y, has_ind_prev)
  })
}

fn layout_floating_table(
  table: &Table,
  mut flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
  text_metrics: &mut TextMetrics,
  mut y: f32,
) -> (FlowContext, f32) {
  let Some(placement) = table.placement else {
    return (flow, y);
  };
  if table.starts_after_last_rendered_page_break
    && y > flow.content_top_pt + LAYOUT_EPSILON_PT
    && flow.paragraph_spacing_context != ParagraphSpacingContext::SectionStart
  {
    // HandleFramedParagraphPageBreak() inserts a dummy PAGE_BEFORE paragraph
    // before a framed paragraph so the anchored frame is placed on the next
    // page instead of remaining on the pre-break page.
    (flow, y) = advance_section_flow(flow, current, pages);
  }
  let mut effective_table = table.clone();
  effective_table.split_allowed = effective_floating_table_split_allowed(table, flow);
  let Some(layout) = TableFrameLayout::new(&effective_table, block_area(flow), true, text_metrics)
  else {
    return (flow, y);
  };
  let table_width = (layout.frame.right_pt - layout.frame.left_pt).max(DEFAULT_FONT_SIZE_PT);
  let (x, frame_y) = floating_table_position(placement, flow, y, table_width);
  let frame_flow = FlowContext {
    content_top_pt: frame_y,
    content_left_pt: x,
    content_bottom: flow.content_bottom,
    body_content_bottom_pt: flow.body_content_bottom_pt,
    content_width: table_width,
    preserve_horizontal_on_advance: table.following_text_flow,
    script_sensitive_line_height: table.following_text_flow,
    ..flow
  };
  let mut frame_page =
    empty_section_page(flow.setup, current.section_index, flow.section_page_index);
  let mut frame_pages = Vec::new();
  let (_, bottom_y) =
    TableFrameLayout::new(&effective_table, block_area(frame_flow), true, text_metrics).map_or(
      (frame_flow, frame_y),
      |layout| {
        layout.format(
          &mut frame_page,
          &mut frame_pages,
          text_metrics,
          frame_y,
          false,
        )
      },
    );
  frame_pages.push(frame_page);
  materialize_pending_floating_table_follows(&mut frame_pages);
  join_split_fly_table_follows(&mut frame_pages, flow);
  frame_pages.retain(|page| {
    !page.items.is_empty()
      || page_has_painted_table_fragment(page)
      || !page.pending_floating_table_follows.is_empty()
  });
  let synthesized_follow = append_last_rendered_floating_table_follow(
    &effective_table,
    frame_flow,
    &mut frame_pages,
    text_metrics,
  );
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
    append_floating_table_wrap_exclusion(first_page, placement, text_metrics);
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
    current
      .pending_floating_table_follows
      .append(&mut first_page.pending_floating_table_follows);
  }
  for mut follow_page in frame_pages.into_iter().skip(1) {
    if follow_page.items.is_empty()
      && follow_page.frame_fragments.is_empty()
      && follow_page.pending_floating_table_follows.is_empty()
    {
      continue;
    }
    append_floating_table_wrap_exclusion(&mut follow_page, placement, text_metrics);
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
        pending_floating_table_follows: follow_page.pending_floating_table_follows,
      });
  }
  let mut moved_to_follow_page = false;
  if synthesized_follow && !current.pending_floating_table_follows.is_empty() {
    materialize_pending_floating_table_follows_in_local_pages(current, pages);
    if current.section_page_index != flow.section_page_index {
      flow = body_flow_for_page(
        FlowContext {
          section_page_index: current.section_page_index,
          ..flow
        },
        pages.len() + 1,
      );
      y = flow.content_top_pt;
      moved_to_follow_page = true;
    }
  }
  let occupied_bottom = first_page_bottom.unwrap_or(bottom_y) + placement.margin_bottom_pt;
  if moved_to_follow_page {
    return (flow, y);
  }
  if frame_wrap_blocks_flow(placement.wrap) {
    (flow, y.max(occupied_bottom))
  } else {
    (flow, y)
  }
}

fn join_split_fly_table_follows(pages: &mut Vec<Page>, flow: FlowContext) {
  // SwTabFrame::MakeAll(): when a table has a follow directly in a split fly,
  // Writer first asks the split fly if it can grow enough to join follow table
  // frames, then joins only that follow chain. Keep this restricted to pages
  // that actually carry table-row fragments. Whole rows moved to the follow
  // page are still part of the follow table chain even when the individual row
  // fragment is not itself marked as a split follow.
  let mut index = 1usize;
  while index < pages.len() {
    if pages[index].explicit_break_target {
      index += 1;
      continue;
    }
    if !page_has_table_follow_fragment(&pages[index]) {
      index += 1;
      continue;
    }
    let Some((_, previous_bottom)) = page_items_vertical_bounds(&pages[index - 1].items) else {
      index += 1;
      continue;
    };
    let Some((next_top, next_bottom)) = page_items_vertical_bounds(&pages[index].items) else {
      let mut empty = pages.remove(index);
      pages[index - 1]
        .pending_floating_table_follows
        .append(&mut empty.pending_floating_table_follows);
      continue;
    };
    let next_height = (next_bottom - next_top).max(0.0);
    if previous_bottom + next_height > flow.content_bottom + LAYOUT_EPSILON_PT {
      index += 1;
      continue;
    }

    let mut next = pages.remove(index);
    let item_offset = pages[index - 1].items.len();
    let dy = previous_bottom - next_top;
    pages[index - 1].items.extend(
      next
        .items
        .into_iter()
        .map(|item| translate_page_item(item, 0.0, dy)),
    );
    offset_page_frame_records_raw(
      &mut next.frame_fragments,
      &mut next.frame_influences,
      item_offset,
    );
    for fragment in next.frame_fragments {
      pages[index - 1]
        .frame_fragments
        .push(translate_frame_fragment(fragment, 0.0, dy));
    }
    pages[index - 1]
      .frame_influences
      .append(&mut next.frame_influences);
    pages[index - 1]
      .wrap_exclusions
      .append(&mut next.wrap_exclusions);
    pages[index - 1]
      .pending_floating_table_follows
      .append(&mut next.pending_floating_table_follows);
  }
}

fn append_last_rendered_floating_table_follow(
  table: &Table,
  frame_flow: FlowContext,
  frame_pages: &mut Vec<Page>,
  text_metrics: &mut TextMetrics,
) -> bool {
  if frame_pages
    .iter()
    .skip(1)
    .any(page_has_painted_table_fragment)
  {
    return false;
  }
  if table_last_rendered_page_break_count(table) < 2 {
    return false;
  }
  let painted_row_count = frame_pages
    .first()
    .map(painted_table_row_fragment_count)
    .unwrap_or_default();
  if painted_row_count >= table.rows.len() {
    return false;
  }
  let Some(start_row_index) = first_row_with_last_rendered_page_break(table)
    .or_else(|| (painted_row_count < table.rows.len()).then_some(painted_row_count))
  else {
    return false;
  };
  let mut follow_flow = body_flow_for_page(
    FlowContext {
      section_page_index: frame_flow.section_page_index + 1,
      ..frame_flow
    },
    frame_flow.section_page_index + 2,
  );
  follow_flow.content_left_pt = frame_flow.content_left_pt;
  follow_flow.content_width = frame_flow.content_width;
  let Some(layout) = TableFrameLayout::new(table, block_area(follow_flow), true, text_metrics)
  else {
    return false;
  };
  let mut follow_page = empty_section_page(
    follow_flow.setup,
    follow_flow.section_index,
    follow_flow.section_page_index,
  );
  let mut discarded_pages = Vec::new();
  let mut y = follow_flow.content_top_pt;
  for (row_index, row) in table.rows.iter().enumerate().skip(start_row_index) {
    y = layout.row_frame(row, row_index, y).format(
      &mut follow_page,
      &mut discarded_pages,
      text_metrics,
    );
    if row_index + 1 < table.rows.len() {
      y += row_cell_spacing_pt(table, row);
    }
  }
  if follow_page.items.is_empty()
    && follow_page.frame_fragments.is_empty()
    && follow_page.pending_floating_table_follows.is_empty()
  {
    return false;
  }
  frame_pages.push(follow_page);
  frame_pages.append(&mut discarded_pages);
  true
}

fn first_row_with_last_rendered_page_break(table: &Table) -> Option<usize> {
  table
    .rows
    .iter()
    .position(row_contains_last_rendered_page_break)
}

fn table_last_rendered_page_break_count(table: &Table) -> usize {
  table
    .rows
    .iter()
    .map(row_last_rendered_page_break_count)
    .sum()
}

fn row_last_rendered_page_break_count(row: &TableRow) -> usize {
  row
    .cells
    .iter()
    .map(|cell| blocks_last_rendered_page_break_count(&cell.blocks))
    .sum()
}

fn blocks_last_rendered_page_break_count(blocks: &[Block]) -> usize {
  blocks
    .iter()
    .map(|block| match block {
      Block::Paragraph(paragraph) => paragraph
        .inlines
        .iter()
        .filter(|inline| matches!(inline, InlineItem::LastRenderedPageBreak))
        .count(),
      Block::Table(table) => table_last_rendered_page_break_count(table),
      Block::Frame(frame) => blocks_last_rendered_page_break_count(&frame.blocks),
    })
    .sum()
}

fn row_contains_last_rendered_page_break(row: &TableRow) -> bool {
  row
    .cells
    .iter()
    .any(|cell| blocks_contain_last_rendered_page_break(&cell.blocks))
}

fn blocks_contain_last_rendered_page_break(blocks: &[Block]) -> bool {
  blocks.iter().any(|block| match block {
    Block::Paragraph(paragraph) => paragraph
      .inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::LastRenderedPageBreak)),
    Block::Table(table) => table.rows.iter().any(row_contains_last_rendered_page_break),
    Block::Frame(frame) => blocks_contain_last_rendered_page_break(&frame.blocks),
  })
}

fn page_has_table_follow_fragment(page: &Page) -> bool {
  page
    .frame_fragments
    .iter()
    .any(|fragment| matches!(fragment.kind, FrameFragmentKind::TableRow))
}

fn painted_table_row_fragment_count(page: &Page) -> usize {
  page
    .frame_fragments
    .iter()
    .filter(|fragment| {
      matches!(fragment.kind, FrameFragmentKind::TableRow)
        && fragment.item_end > fragment.item_start
    })
    .count()
}

fn page_has_painted_table_fragment(page: &Page) -> bool {
  page.frame_fragments.iter().any(|fragment| {
    matches!(
      fragment.kind,
      FrameFragmentKind::TableRow | FrameFragmentKind::TableCell
    ) && fragment.item_end > fragment.item_start
  })
}

fn effective_floating_table_split_allowed(table: &Table, flow: FlowContext) -> bool {
  if !table.split_allowed {
    return false;
  }
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
  if page.items.is_empty() {
    return;
  }
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

fn materialize_table_frame_fragment_bounds(pages: &mut [Page], frames: &[LayoutFrame]) {
  for frame in frames {
    if !matches!(frame.kind, FollowFrameKind::Table) {
      continue;
    }
    let Some(page) = pages.get_mut(frame.page_index) else {
      continue;
    };
    for fragment in &frame.fragments {
      if !matches!(fragment.kind, FrameFragmentKind::TableRow) {
        continue;
      }
      let Some(bounds) = fragment.bounds else {
        continue;
      };
      if bounds.width_pt <= LAYOUT_EPSILON_PT || bounds.height_pt <= LAYOUT_EPSILON_PT {
        continue;
      }
      if fragment_item_range_has_path(&page.items, fragment.item_start, fragment.item_end) {
        continue;
      }
      page.items.push(PageItem::Rect(RectItem {
        x_pt: bounds.x_pt,
        y_pt: bounds.y_pt,
        width_pt: bounds.width_pt,
        height_pt: bounds.height_pt,
        fill_color: None,
        fill_opacity: 1.0,
        stroke: Some(BorderStyle::default()),
        stroke_opacity: 0.0,
      }));
    }
  }
}

fn fragment_item_range_has_path(items: &[PageItem], item_start: usize, item_end: usize) -> bool {
  let start = item_start.min(items.len());
  let end = item_end.min(items.len());
  if start >= end {
    return false;
  }
  items[start..end].iter().any(page_item_is_path)
}

fn page_item_is_path(item: &PageItem) -> bool {
  match item {
    PageItem::Rect(rect) => rect.fill_color.is_some() || rect.stroke.is_some(),
    PageItem::Line(_) => true,
    PageItem::Polyline(polyline) => polyline.fill_color.is_some() || polyline.stroke.is_some(),
    PageItem::Text(_) | PageItem::Image(_) | PageItem::Fill(_) => false,
  }
}

fn append_floating_table_wrap_exclusion(
  page: &mut Page,
  placement: FloatingFramePlacement,
  text_metrics: &mut TextMetrics,
) {
  // Writer derives the fly exclusion from the table frame, not from the
  // union of everything painted on its page. In particular, a split follow
  // page can already contain non-table adornments whose ink would otherwise
  // widen the exclusion to the whole body and force following text below the
  // table instead of wrapping beside it.
  let table_bounds = page
    .frame_fragments
    .iter()
    .filter(|fragment| matches!(fragment.kind, FrameFragmentKind::TableRow))
    .filter_map(|fragment| fragment.bounds)
    .fold(None::<(f32, f32, f32, f32)>, |bounds, row| {
      Some(match bounds {
        None => (
          row.x_pt,
          row.y_pt,
          row.x_pt + row.width_pt,
          row.y_pt + row.height_pt,
        ),
        Some((left, top, right, bottom)) => (
          left.min(row.x_pt),
          top.min(row.y_pt),
          right.max(row.x_pt + row.width_pt),
          bottom.max(row.y_pt + row.height_pt),
        ),
      })
    });
  let Some((left_pt, top_pt, right_pt, bottom_pt)) =
    table_bounds.or_else(|| page_items_bounds(&page.items, text_metrics))
  else {
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

fn lower_inline_chart(
  chart: &InlineChart,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Vec<PageItem> {
  if let Some(chart_space) = chart.extended_chart_space.as_deref() {
    return crate::xlsx::chartex::lower_extended_chart_cached(
      chart_space,
      ChartFrame {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
      },
      chart.title_style.clone(),
      chart.label_style.clone(),
    )
    .into_iter()
    .filter_map(docx_chart_page_item)
    .collect();
  }
  let Some(chart_space) = chart.chart_space.as_deref() else {
    return Vec::new();
  };
  if let Some(model) = shared_chart::pie_chart_model(chart_space)
    && model.kind == shared_chart::RadialChartKind::Pie
    && model.legend_position.is_none_or(|position| {
      matches!(
        position,
        shared_chart::ChartLegendPosition::Right | shared_chart::ChartLegendPosition::Bottom
      )
    })
    && model.legend_layout.is_none()
    && !model.legend_overlay
    && model
      .values
      .iter()
      .all(|value| value.is_none_or(|value| value.is_finite() && value >= 0.0))
  {
    let expected_texts = if model.legend_position.is_some() {
      model
        .visible_legend_indices
        .iter()
        .filter_map(|index| model.categories.get(*index).cloned())
        .collect::<Vec<_>>()
    } else {
      Vec::new()
    };
    let mut expected_texts = expected_texts;
    expected_texts.extend(model.data_labels.iter().map(|label| label.text.clone()));
    if shared_chart::fixed_output_texts_for_ui_language(chart_space, chart.ui_language.as_deref())
      == expected_texts
    {
      return lower_word_pie_chart(chart, &model, x_pt, y_pt, width_pt, height_pt, text_metrics);
    }
  }

  if let Some(mut model) = shared_chart::pie_chart_model(chart_space) {
    if matches!(model.title, Some(shared_chart::ChartTitleText::Automatic))
      && chart_space.chart.title.is_none()
    {
      model.title = None;
    }
    let lowered = lower_radial_chart(
      ChartFrame {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
      },
      &model,
      &chart.automatic_title,
      &RadialChartStyle {
        layout_profile: ChartLayoutProfile::Word,
        title: chart.title_style.clone(),
        label: chart.label_style.clone(),
        data_label: chart.data_label_style.clone(),
        point_colors: chart.pie_point_colors.clone(),
        data_label_fill_colors: vec![None; model.data_labels.len()],
        chart_area_fill_color: chart.chart_area_fill_color,
        plot_area_fill_color: chart.plot_area_fill_color,
        chart_area_stroke_color: chart.chart_area_stroke_color,
        plot_area_stroke_color: chart.plot_area_stroke_color,
      },
    )
    .into_iter()
    .filter_map(docx_chart_page_item)
    .collect::<Vec<_>>();
    if !lowered.is_empty() {
      return lowered;
    }
  }

  let Some(mut model) =
    shared_chart::cartesian_chart_for_ui_language(chart_space, chart.ui_language.as_deref())
  else {
    return lower_generic_inline_chart(chart, x_pt, y_pt, width_pt, height_pt);
  };
  if model.title.is_none() && shared_chart::has_word_automatic_title_placeholder(&chart_space.chart)
  {
    model.title = Some(shared_chart::ChartTitleText::Automatic);
  }
  if matches!(model.title, Some(shared_chart::ChartTitleText::Automatic))
    && chart_space.chart.title.is_none()
  {
    model.title = None;
  }
  let word_series_names = shared_chart::series_identity_texts_with_uncached_labels(chart_space);
  for (index, series) in model.series.iter_mut().enumerate() {
    if !series.has_explicit_name
      && let Some(name) = word_series_names.get(index)
    {
      series.name.clone_from(name);
    }
  }
  let frame_stroke = (!chart_space
    .shape_properties
    .as_deref()
    .is_some_and(shared_chart::shape_properties_has_no_outline))
  .then_some(BorderStyle {
    width_pt: 0.14,
    color: RgbColor { r: 0, g: 0, b: 0 },
    ..BorderStyle::default()
  });
  let mut items = vec![PageItem::Rect(RectItem {
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    fill_color: None,
    fill_opacity: 1.0,
    stroke: frame_stroke,
    stroke_opacity: 1.0,
  })];
  items.extend(
    lower_clustered_column_chart(
      ChartFrame {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
      },
      &model,
      &chart.automatic_title,
      &ClusteredColumnStyle {
        layout_profile: ChartLayoutProfile::Word,
        modern_excel_profile: false,
        stroke_scale: 1.0,
        has_explicit_title: matches!(model.title, Some(shared_chart::ChartTitleText::Explicit(_))),
        title: chart.title_style.clone(),
        title_fill_color: chart.title_fill_color,
        label: chart.label_style.clone(),
        category_label: chart.label_style.clone(),
        value_label: chart.label_style.clone(),
        data_label: chart.data_label_style.clone(),
        gridline_color: chart.gridline_color,
        series_colors: chart.series_colors.clone(),
        series_point_colors: chart.series_point_colors.clone(),
        data_label_fill_colors: model
          .series
          .iter()
          .map(|series| vec![None; series.data_labels.len()])
          .collect(),
        chart_area_fill_color: chart.chart_area_fill_color,
        plot_area_fill_color: chart.plot_area_fill_color,
        chart_area_stroke_color: chart.chart_area_stroke_color,
        plot_area_stroke_color: chart.plot_area_stroke_color,
      },
    )
    .into_iter()
    .filter_map(docx_chart_page_item),
  );
  items
}

fn lower_word_pie_chart(
  chart: &InlineChart,
  model: &shared_chart::PieChartModel<'_>,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Vec<PageItem> {
  let bottom_legend = model.legend_position == Some(shared_chart::ChartLegendPosition::Bottom);
  let no_legend = model.legend_position.is_none();
  // Word positions this chart frame below the inline drawing's flow origin.
  // Keep the host offset separate from the chart-space ratios below.
  let chart_y_pt = if bottom_legend || no_legend {
    y_pt
  } else {
    y_pt + height_pt * 0.013_606_33
  };
  let frame_stroke = if bottom_legend || no_legend {
    BorderStyle {
      width_pt: 0.75,
      color: RgbColor {
        r: 217,
        g: 217,
        b: 217,
      },
      ..BorderStyle::default()
    }
  } else {
    BorderStyle {
      width_pt: 0.14,
      color: RgbColor { r: 0, g: 0, b: 0 },
      ..BorderStyle::default()
    }
  };
  let mut items = vec![PageItem::Rect(RectItem {
    x_pt,
    y_pt: chart_y_pt,
    width_pt,
    height_pt,
    fill_color: None,
    fill_opacity: 1.0,
    stroke: Some(frame_stroke),
    stroke_opacity: 1.0,
  })];

  let total = model.values.iter().flatten().sum::<f64>();
  if total > 0.0 {
    // Word's automatic right-legend layout reserves the right side of the
    // chart frame and centers the pie in the remaining plot region. These
    // ratios are calibrated against Office fixed output after applying the
    // ECMA/MS-OI29500 data-selection and angle rules above.
    let (center_x, center_y, radius_x, radius_y) = if bottom_legend {
      (
        width_pt * 0.5,
        height_pt * 0.454_6,
        height_pt * 0.410_5,
        height_pt * 0.410_5,
      )
    } else if no_legend {
      (
        width_pt * 0.5,
        height_pt * 0.5,
        height_pt * 0.394_355_15,
        height_pt * 0.394_355_15,
      )
    } else {
      (
        width_pt * 0.444_498_36,
        height_pt * 0.5,
        height_pt * 0.394_355_15,
        height_pt * 0.394_355_15,
      )
    };
    let mut start_angle = model.first_slice_angle_deg.to_radians() as f32;
    for (index, value) in model.values.iter().enumerate() {
      let sweep = value.unwrap_or(0.0) / total * std::f64::consts::TAU;
      if sweep <= 0.0 {
        continue;
      }
      let sweep = sweep as f32;
      let segment_count = ((sweep.to_degrees() / 1.0).ceil() as usize).max(1);
      let mut points = Vec::with_capacity(segment_count + 3);
      points.push((center_x, center_y));
      for segment in 0..=segment_count {
        let angle = start_angle + sweep * segment as f32 / segment_count as f32;
        points.push((
          center_x + angle.sin() * radius_x,
          center_y - angle.cos() * radius_y,
        ));
      }
      items.push(PageItem::Polyline(PolylineItem {
        x_pt,
        y_pt: chart_y_pt,
        width_pt,
        height_pt,
        points,
        closed: true,
        fill_color: chart.pie_point_colors.get(index).copied(),
        stroke: (bottom_legend || no_legend).then_some(BorderStyle {
          width_pt: 1.5,
          color: RgbColor {
            r: 255,
            g: 255,
            b: 255,
          },
          ..BorderStyle::default()
        }),
      }));
      start_angle += sweep;
    }
  }

  if no_legend && !model.data_labels.is_empty() {
    let total = model.values.iter().flatten().sum::<f64>();
    if total > 0.0 {
      let center_x = width_pt * 0.5;
      // Word's automatic pie-label placement is slightly taller than the pie
      // centroid ring: the two-thirds lower slice in tdf123206 is pushed
      // farther down while the upper slice remains above center. Keep the
      // horizontal ring circular enough to preserve centered label origins,
      // but use Office's vertically expanded label ring and center.
      let center_y = height_pt * 0.504_26;
      let label_radius_x = height_pt * 0.394_355_15 * 0.84;
      let label_radius_y = height_pt * 0.348_384;
      let start_angle = model.first_slice_angle_deg.to_radians() as f32;
      for label in &model.data_labels {
        let before = model
          .values
          .iter()
          .take(label.point_index)
          .flatten()
          .sum::<f64>();
        let Some(value) = model.values.get(label.point_index).copied().flatten() else {
          continue;
        };
        let mid_angle =
          start_angle + ((before + value * 0.5) / total * std::f64::consts::TAU) as f32;
        let text_width = text_metrics.measure_text(&label.text, &chart.data_label_style);
        items.push(PageItem::Text(Box::new(TextItem {
          x_pt: x_pt + center_x + mid_angle.sin() * label_radius_x - text_width * 0.5,
          y_pt: chart_y_pt + center_y
            - mid_angle.cos() * label_radius_y
            - chart.data_label_style.font_size_pt * 0.99,
          line_height_pt: chart.data_label_style.font_size_pt * 1.2,
          text: label.text.clone(),
          style: chart.data_label_style.clone(),
          rotation_center_pt: None,
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          form_widget_id: None,
          paragraph_bidi: false,
          word_spacing_pt: 0.0,
          preserve_text_portion: true,
          decoration_span_start_x_pt: None,
          pdf_text_segmentation: PdfTextSegmentation::Line,
        })));
      }
    }
  }

  if bottom_legend {
    let marker_size = chart.label_style.font_size_pt * 0.55;
    let marker_text_gap = chart.label_style.font_size_pt * 0.275;
    let item_gap = chart.label_style.font_size_pt * 0.515;
    let entries = model
      .visible_legend_indices
      .iter()
      .filter_map(|point_index| {
        let text = model.categories.get(*point_index)?;
        let text_width = text_metrics.measure_text(text, &chart.label_style);
        Some((*point_index, text, text_width))
      })
      .collect::<Vec<_>>();
    let content_width = entries
      .iter()
      .map(|(_, _, text_width)| marker_size + marker_text_gap + text_width)
      .sum::<f32>()
      + item_gap * entries.len().saturating_sub(1) as f32;
    let mut item_x = x_pt + (width_pt - content_width) * 0.5 + width_pt * 0.004_6;
    let marker_y = chart_y_pt + height_pt * 0.932_9;
    let text_y = chart_y_pt + height_pt * 0.924_0;
    for (point_index, text, text_width) in entries {
      if let Some(color) = chart.pie_point_colors.get(point_index).copied() {
        items.push(PageItem::Rect(RectItem {
          x_pt: item_x,
          y_pt: marker_y,
          width_pt: marker_size,
          height_pt: marker_size,
          fill_color: Some(color),
          fill_opacity: 1.0,
          stroke: None,
          stroke_opacity: 1.0,
        }));
      }
      items.push(PageItem::Text(Box::new(TextItem {
        x_pt: item_x + marker_size + marker_text_gap,
        y_pt: text_y,
        line_height_pt: chart.label_style.font_size_pt * 1.2,
        text: text.clone(),
        style: chart.label_style.clone(),
        rotation_center_pt: None,
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        form_widget_id: None,
        paragraph_bidi: false,
        word_spacing_pt: 0.0,
        preserve_text_portion: true,
        decoration_span_start_x_pt: None,
        pdf_text_segmentation: PdfTextSegmentation::Line,
      })));
      item_x += marker_size + marker_text_gap + text_width + item_gap;
    }
  } else if model.legend_position.is_some() {
    let marker_size = chart.label_style.font_size_pt * 0.502_87;
    let marker_x = x_pt + width_pt * 0.899_492_86;
    let text_x = x_pt + width_pt * 0.915_457_55;
    let first_marker_y = chart_y_pt + height_pt * 0.389_812;
    // TextItem's visible-text path adds the font ascent before emitting the
    // PDF text matrix; this is the text-box top, not the final PDF baseline.
    let first_text_y = chart_y_pt + height_pt * 0.376_303_4;
    let row_step = height_pt * 0.066_914_94;
    for (row, point_index) in model.visible_legend_indices.iter().copied().enumerate() {
      let row_offset = row as f32 * row_step;
      if let Some(color) = chart.pie_point_colors.get(point_index).copied() {
        items.push(PageItem::Rect(RectItem {
          x_pt: marker_x,
          y_pt: first_marker_y + row_offset,
          width_pt: marker_size,
          height_pt: marker_size,
          fill_color: Some(color),
          fill_opacity: 1.0,
          stroke: None,
          stroke_opacity: 1.0,
        }));
      }
      if let Some(text) = model.categories.get(point_index) {
        items.push(PageItem::Text(Box::new(TextItem {
          x_pt: text_x,
          y_pt: first_text_y + row_offset,
          line_height_pt: chart.label_style.font_size_pt * 1.2,
          text: text.clone(),
          style: chart.label_style.clone(),
          rotation_center_pt: None,
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          form_widget_id: None,
          paragraph_bidi: false,
          word_spacing_pt: 0.0,
          preserve_text_portion: true,
          decoration_span_start_x_pt: None,
          pdf_text_segmentation: PdfTextSegmentation::Line,
        })));
      }
    }
  }
  items
}

fn lower_generic_inline_chart(
  chart: &InlineChart,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
) -> Vec<PageItem> {
  let Some(chart_space) = chart.chart_space.as_deref() else {
    return Vec::new();
  };
  let stroke = Some(BorderStyle::default());
  let mut items = match shared_chart::kind(chart_space) {
    shared_chart::ChartKind::Pie => {
      let diameter_pt = (width_pt.min(height_pt) * 0.911_706_3)
        .min(width_pt)
        .min(height_pt);
      vec![
        PageItem::Rect(RectItem {
          x_pt,
          y_pt,
          width_pt: diameter_pt,
          height_pt: diameter_pt,
          fill_color: None,
          fill_opacity: 1.0,
          stroke,
          stroke_opacity: 1.0,
        }),
        PageItem::Rect(RectItem {
          x_pt,
          y_pt,
          width_pt: diameter_pt,
          height_pt: diameter_pt,
          fill_color: None,
          fill_opacity: 1.0,
          stroke,
          stroke_opacity: 1.0,
        }),
      ]
    }
    shared_chart::ChartKind::Bar => vec![PageItem::Rect(RectItem {
      x_pt,
      y_pt,
      width_pt: width_pt / 3.0,
      height_pt: height_pt * 0.55,
      fill_color: None,
      fill_opacity: 1.0,
      stroke,
      stroke_opacity: 1.0,
    })],
    shared_chart::ChartKind::Area => vec![PageItem::Rect(RectItem {
      x_pt,
      y_pt: y_pt + height_pt * 1.055,
      width_pt,
      height_pt,
      fill_color: None,
      fill_opacity: 1.0,
      stroke,
      stroke_opacity: 1.0,
    })],
    shared_chart::ChartKind::Line
    | shared_chart::ChartKind::Scatter
    | shared_chart::ChartKind::Bubble
    | shared_chart::ChartKind::Radar
    | shared_chart::ChartKind::Stock
    | shared_chart::ChartKind::Surface
    | shared_chart::ChartKind::Other => vec![PageItem::Rect(RectItem {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
      fill_color: None,
      fill_opacity: 1.0,
      stroke,
      stroke_opacity: 1.0,
    })],
  };

  let fixed_text =
    shared_chart::fixed_output_texts_for_ui_language(chart_space, chart.ui_language.as_deref())
      .join(" ");
  if !fixed_text.is_empty() {
    let mut style = chart.label_style.clone();
    // The generic chart visual lowerers are still intentionally conservative,
    // but PDF text/structure must describe the labels that Office exposes at
    // the chart anchor. Invisible semantic text avoids repainting the cached
    // source data as detached Word body paragraphs.
    style.semantic_only = true;
    items.push(PageItem::Text(Box::new(TextItem {
      x_pt,
      y_pt,
      line_height_pt: style.font_size_pt * 1.2,
      text: fixed_text,
      style,
      rotation_center_pt: None,
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: true,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: PdfTextSegmentation::Line,
    })));
  }
  items
}

fn docx_chart_page_item(item: crate::model::PageItem) -> Option<PageItem> {
  match item {
    crate::model::PageItem::Text(text) => Some(PageItem::Text(Box::new(TextItem {
      x_pt: text.x_pt,
      y_pt: text.y_pt,
      line_height_pt: text.line_height_pt,
      text: text.text,
      style: text.style,
      rotation_center_pt: text.rotation_center_pt,
      hyperlink_url: text.hyperlink_url,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      form_widget_id: text.form_widget_id,
      paragraph_bidi: text.paragraph_bidi,
      word_spacing_pt: 0.0,
      preserve_text_portion: text.preserve_text_portion,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: match text.pdf_text_segmentation {
        crate::model::PdfTextSegmentation::Line => PdfTextSegmentation::Line,
        crate::model::PdfTextSegmentation::Portion => PdfTextSegmentation::Portion,
      },
    }))),
    crate::model::PageItem::Rect(rect) => Some(PageItem::Rect(RectItem {
      x_pt: rect.x_pt,
      y_pt: rect.y_pt,
      width_pt: rect.width_pt,
      height_pt: rect.height_pt,
      fill_color: rect.fill_color,
      fill_opacity: rect.fill_opacity,
      stroke: rect.stroke,
      stroke_opacity: rect.stroke_opacity,
    })),
    crate::model::PageItem::Line(line) => Some(PageItem::Line(LineItem {
      x1_pt: line.x1_pt,
      y1_pt: line.y1_pt,
      x2_pt: line.x2_pt,
      y2_pt: line.y2_pt,
      width_pt: line.width_pt,
      color: line.color,
      kind: LineItemKind::Stroke,
    })),
    crate::model::PageItem::Path(path) if !path.points.is_empty() => {
      let fill_color = match path.fill {
        crate::common::Fill::Solid(color) => Some(RgbColor {
          r: color.r,
          g: color.g,
          b: color.b,
        }),
        _ => None,
      };
      let stroke = path.stroke.map(|stroke| BorderStyle {
        width_pt: stroke.width.0,
        color: RgbColor {
          r: stroke.color.r,
          g: stroke.color.g,
          b: stroke.color.b,
        },
        ..BorderStyle::default()
      });
      Some(PageItem::Polyline(PolylineItem {
        x_pt: 0.0,
        y_pt: 0.0,
        width_pt: path.bounds.size.width.0,
        height_pt: path.bounds.size.height.0,
        points: path
          .points
          .into_iter()
          .map(|point| (point.x.0, point.y.0))
          .collect(),
        closed: path.closed,
        fill_color,
        stroke,
      }))
    }
    crate::model::PageItem::Image(_)
    | crate::model::PageItem::LinkArea(_)
    | crate::model::PageItem::Path(_) => None,
  }
}

fn translate_frame_bounds(mut bounds: FrameBounds, dx_pt: f32, dy_pt: f32) -> FrameBounds {
  bounds.x_pt += dx_pt;
  bounds.y_pt += dy_pt;
  bounds
}

fn translate_frame_fragment(mut fragment: FrameFragment, dx_pt: f32, dy_pt: f32) -> FrameFragment {
  fragment.bounds = fragment
    .bounds
    .map(|bounds| translate_frame_bounds(bounds, dx_pt, dy_pt));
  fragment
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

fn flatten_nested_pages_with_fragments(
  first_page: Page,
  discarded_pages: Vec<Page>,
  first_top_pt: f32,
  first_bottom_pt: f32,
) -> (Vec<PageItem>, Vec<FrameFragment>) {
  let mut pages = discarded_pages;
  pages.push(first_page);
  let mut items = Vec::new();
  let mut fragments = Vec::new();
  let mut item_offset = 0;
  let mut virtual_top_pt = first_top_pt;

  for (page_index, page) in pages.into_iter().enumerate() {
    let (page_top_pt, page_bottom_pt) = if page_index == 0 {
      (first_top_pt, first_bottom_pt)
    } else {
      page_content_vertical_bounds(page.setup)
    };
    let dy_pt = virtual_top_pt - page_top_pt;
    fragments.extend(page.frame_fragments.into_iter().map(|mut fragment| {
      fragment.item_start += item_offset;
      fragment.item_end += item_offset;
      translate_frame_fragment(fragment, 0.0, dy_pt)
    }));
    let item_count = page.items.len();
    items.extend(
      page
        .items
        .into_iter()
        .map(|item| translate_page_item(item, 0.0, dy_pt)),
    );
    item_offset += item_count;
    virtual_top_pt += (page_bottom_pt - page_top_pt).max(0.0);
  }

  (items, fragments)
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
  fn new(
    table: &'a Table,
    area: BlockArea,
    allow_width_overflow: bool,
    text_metrics: &mut TextMetrics,
  ) -> Option<Self> {
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
    let row_heights: Arc<[f32]> = table_row_heights_with_widths(
      table,
      &column_widths,
      area.setup,
      area.compatibility_mode,
      text_metrics,
    )
    .into();
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

  fn format(
    self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    text_metrics: &mut TextMetrics,
    mut y: f32,
    mut has_ind_prev: bool,
  ) -> (FlowContext, f32) {
    let mut layout = self;
    let mut flow = flow_from_block_area(layout.frame.block);
    let mut repeating_headers_disabled = false;
    y = layout.dodge_wrap_exclusions(current, y, layout.initial_dodge_height());
    if !layout.frame.split_allowed
      && y > layout.frame.block.content_top_pt + LAYOUT_EPSILON_PT
      && y + layout.frame.total_height > layout.frame.block.content_bottom + LAYOUT_EPSILON_PT
    {
      (flow, y) = advance_section_flow(flow, current, pages);
      has_ind_prev = false;
      if let Some(next_layout) =
        layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
      {
        layout = next_layout;
      }
    }
    let table_row_keep = layout.table_row_keep_enabled();
    let mut left_border_segment = None;
    let mut right_border_segment = None;
    let mut follow_state = TableFollowState::default();
    let mut row_index = 0usize;
    'table_rows: while row_index < layout.table.rows.len() {
      let allow_split_of_keep_row =
        layout.allow_split_of_keep_row(row_index, has_ind_prev, table_row_keep);
      let make_all = layout.make_all_split_plan(
        row_index,
        y,
        has_ind_prev,
        table_row_keep,
        allow_split_of_keep_row,
      );
      if make_all.move_forward {
        flush_border_segment(current, &mut left_border_segment);
        flush_border_segment(current, &mut right_border_segment);
        (flow, y) = advance_section_flow(flow, current, pages);
        has_ind_prev = false;
        if let Some(next_layout) =
          layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
        {
          layout = next_layout;
        }
        y = layout.format_repeated_header_rows(current, pages, text_metrics, y, 0.0);
        continue;
      }
      if make_all.disable_repeating_headers {
        repeating_headers_disabled = true;
        layout = layout.without_repeating_headers();
        continue;
      }
      let split_decision = make_all.split_decision;
      let format_until = split_decision.map_or(layout.table.rows.len(), |decision| {
        decision.master_end_row_index
      });
      while row_index < format_until {
        let row = &layout.table.rows[row_index];
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
          has_ind_prev = split_decision
            .and_then(|decision| decision.follow_start_row_index)
            .is_some();
          if let Some(next_layout) =
            layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
          {
            layout = next_layout;
          }
          y = layout.format_repeated_header_rows(current, pages, text_metrics, y, 0.0);
          continue 'table_rows;
        }
        let row_frame = layout.row_frame(row, row_index, y);
        let row_top = y;
        y = row_frame.format(current, pages, text_metrics);
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
        if row_index < layout.table.rows.len() {
          y += row_cell_spacing_pt(layout.table, row);
        }
      }
      if row_index >= layout.table.rows.len() {
        break;
      }

      let Some(split_decision) = split_decision else {
        break;
      };
      if let Some(follow_start_row_index) = split_decision.follow_start_row_index {
        debug_assert!(follow_start_row_index >= row_index);
      }
      let row = &layout.table.rows[row_index];
      y = layout.dodge_wrap_exclusions(
        current,
        y,
        split_decision
          .split_row_allowed
          .then(|| layout.minimum_split_fragment_height(row_index, row))
          .filter(|height| *height > LAYOUT_EPSILON_PT)
          .or_else(|| {
            split_decision
              .split_row_allowed
              .then(|| layout.row_first_content_line_height(row_index, row, text_metrics))
              .flatten()
          })
          .unwrap_or_else(|| {
            layout
              .frame
              .row_heights
              .get(row_index)
              .copied()
              .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT)
          }),
      );
      if y > layout.frame.block.content_bottom + LAYOUT_EPSILON_PT {
        flush_border_segment(current, &mut left_border_segment);
        flush_border_segment(current, &mut right_border_segment);
        (flow, y) = advance_section_flow(flow, current, pages);
        has_ind_prev = split_decision.follow_start_row_index.is_some();
        if let Some(next_layout) =
          layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
        {
          layout = next_layout;
        }
        y = layout.format_repeated_header_rows(current, pages, text_metrics, y, 0.0);
        continue 'table_rows;
      }
      let row_frame = layout.row_frame(row, row_index, y);
      let row_height = row_frame.height_pt;
      if split_decision.split_row_allowed {
        debug_assert!(split_decision.creates_follow_flow_line);
        let last_rendered_follow_height =
          layout.row_last_rendered_page_break_follow_height(row_index, row, text_metrics);
        let mut last_rendered_follow_height_applied = false;
        let mut remaining_height = row_height;
        let mut content_offset = 0.0;
        while remaining_height > LAYOUT_EPSILON_PT {
          let available_height = (layout.frame.block.content_bottom - y).max(0.0);
          if available_height <= LAYOUT_EPSILON_PT {
            flush_border_segment(current, &mut left_border_segment);
            flush_border_segment(current, &mut right_border_segment);
            (flow, y) = advance_section_flow(flow, current, pages);
            has_ind_prev = true;
            if let Some(next_layout) =
              layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
            {
              layout = next_layout;
            }
            y = layout.format_repeated_header_rows(
              current,
              pages,
              text_metrics,
              y,
              if follow_state.has_follow_flow_line {
                remaining_height
              } else {
                0.0
              },
            );
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
            has_ind_prev = true;
            if let Some(next_layout) =
              layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
            {
              layout = next_layout;
            }
            y = layout.format_repeated_header_rows(
              current,
              pages,
              text_metrics,
              y,
              if follow_state.has_follow_flow_line {
                remaining_height
              } else {
                0.0
              },
            );
            continue;
          }

          let fragment_height = remaining_height.min(available_height);
          layout.row_frame(row, row_index, y).format_fragment(
            current,
            pages,
            text_metrics,
            y,
            y + fragment_height,
            content_offset,
          );
          extend_border_segment(
            current,
            &mut left_border_segment,
            layout
              .row_frame(row, row_index, y)
              .leading_border_segment(y, y + fragment_height),
          );
          extend_border_segment(
            current,
            &mut right_border_segment,
            layout
              .row_frame(row, row_index, y)
              .trailing_border_segment(y, y + fragment_height),
          );
          y += fragment_height;
          remaining_height -= fragment_height;
          content_offset += fragment_height;
          if !last_rendered_follow_height_applied
            && content_offset > LAYOUT_EPSILON_PT
            && let Some(follow_height) = last_rendered_follow_height
          {
            remaining_height = remaining_height.max(follow_height);
            last_rendered_follow_height_applied = true;
          }
          follow_state.set_follow_flow_line(row_index, remaining_height, content_offset);
          if remaining_height > LAYOUT_EPSILON_PT {
            flush_border_segment(current, &mut left_border_segment);
            flush_border_segment(current, &mut right_border_segment);
            (flow, y) = advance_section_flow(flow, current, pages);
            has_ind_prev = true;
            if let Some(next_layout) =
              layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
            {
              layout = next_layout;
            }
            y =
              layout.format_repeated_header_rows(current, pages, text_metrics, y, remaining_height);
          }
        }
        follow_state.clear_follow_flow_line();
        row_index += 1;
        if row_index < layout.table.rows.len() {
          y += row_cell_spacing_pt(layout.table, row);
        }
        continue;
      }

      if !split_decision.move_rows_to_follow {
        let row_top = y;
        y = row_frame.format(current, pages, text_metrics);
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
        if row_index < layout.table.rows.len() {
          y += row_cell_spacing_pt(layout.table, row);
        }
        continue;
      }

      if layout.should_backward_move_follow_row_group(row_index, y, text_metrics) {
        if follow_state.has_follow_flow_line
          && follow_state.split_row_index == Some(row_index.saturating_sub(1))
        {
          if let Some(split_row_index) = follow_state.split_row_index
            && let Some(split_row) = layout.table.rows.get(split_row_index)
            && follow_state.remaining_height_pt > LAYOUT_EPSILON_PT
          {
            let available_height = (layout.frame.block.content_bottom - y).max(0.0);
            if available_height > LAYOUT_EPSILON_PT {
              let fragment_height = follow_state.remaining_height_pt.min(available_height);
              layout
                .row_frame(split_row, split_row_index, y)
                .format_fragment(
                  current,
                  pages,
                  text_metrics,
                  y,
                  y + fragment_height,
                  follow_state.content_offset_pt,
                );
              y += fragment_height;
              follow_state.set_follow_flow_line(
                split_row_index,
                follow_state.remaining_height_pt - fragment_height,
                follow_state.content_offset_pt + fragment_height,
              );
              if follow_state.has_follow_flow_line {
                flush_border_segment(current, &mut left_border_segment);
                flush_border_segment(current, &mut right_border_segment);
                (flow, y) = advance_section_flow(flow, current, pages);
                has_ind_prev = true;
                if let Some(next_layout) =
                  layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
                {
                  layout = next_layout;
                }
                y = layout.format_repeated_header_rows(
                  current,
                  pages,
                  text_metrics,
                  y,
                  follow_state.remaining_height_pt,
                );
                continue;
              }
            }
          }
          if !follow_state.has_follow_flow_line {
            follow_state.clear_follow_flow_line();
          }
        }
        let rows_to_move = layout.maximum_layout_row_span(row_index);
        for moved in 0..rows_to_move {
          let moved_row_index = row_index + moved;
          let Some(moved_row) = layout.table.rows.get(moved_row_index) else {
            break;
          };
          let moved_row_top = y;
          let moved_row_frame = layout.row_frame(moved_row, moved_row_index, y);
          let moved_row_height = moved_row_frame.height_pt;
          if moved == 0
            && moved_row_frame.bottom() > layout.frame.block.content_bottom + LAYOUT_EPSILON_PT
            && layout.row_can_split_at_cut(
              &moved_row_frame,
              table_row_keep,
              allow_split_of_keep_row,
            )
          {
            let row_bottom = y + (layout.frame.block.content_bottom - y).max(0.0);
            moved_row_frame.format_fragment(current, pages, text_metrics, y, row_bottom, 0.0);
            extend_border_segment(
              current,
              &mut left_border_segment,
              moved_row_frame.leading_border_segment(moved_row_top, row_bottom),
            );
            extend_border_segment(
              current,
              &mut right_border_segment,
              moved_row_frame.trailing_border_segment(moved_row_top, row_bottom),
            );
            flush_border_segment(current, &mut left_border_segment);
            flush_border_segment(current, &mut right_border_segment);
            (flow, y) = advance_section_flow(flow, current, pages);
            has_ind_prev = true;
            if let Some(next_layout) =
              layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
            {
              layout = next_layout;
            }
            y = layout.format_repeated_header_rows(
              current,
              pages,
              text_metrics,
              y,
              moved_row_height - (row_bottom - moved_row_top),
            );
            let remaining_height = moved_row_height - (row_bottom - moved_row_top);
            layout
              .row_frame(moved_row, moved_row_index, y)
              .format_fragment(
                current,
                pages,
                text_metrics,
                y,
                y + remaining_height,
                row_bottom - moved_row_top,
              );
            y += remaining_height;
            row_index += 1;
            if row_index < layout.table.rows.len() {
              y += row_cell_spacing_pt(layout.table, moved_row);
            }
          } else {
            y = moved_row_frame.format(current, pages, text_metrics);
            extend_border_segment(
              current,
              &mut left_border_segment,
              moved_row_frame.leading_border_segment(moved_row_top, y),
            );
            extend_border_segment(
              current,
              &mut right_border_segment,
              moved_row_frame.trailing_border_segment(moved_row_top, y),
            );
            row_index += 1;
            if row_index < layout.table.rows.len() {
              y += row_cell_spacing_pt(layout.table, moved_row);
            }
          }
        }
        continue;
      }

      flush_border_segment(current, &mut left_border_segment);
      flush_border_segment(current, &mut right_border_segment);
      (flow, y) = advance_section_flow(flow, current, pages);
      has_ind_prev = split_decision.follow_start_row_index.is_some();
      if let Some(next_layout) =
        layout.layout_for_flow(flow, repeating_headers_disabled, text_metrics)
      {
        layout = next_layout;
      }
      y = layout.format_repeated_header_rows(
        current,
        pages,
        text_metrics,
        y,
        layout.follow_page_required_height(row_index, y),
      );
    }

    flush_border_segment(current, &mut left_border_segment);
    flush_border_segment(current, &mut right_border_segment);

    (flow, y + TABLE_SPACING_AFTER_PT)
  }

  fn table_split_decision(
    &self,
    start_row_index: usize,
    y: f32,
    has_ind_prev: bool,
    try_to_split_row: bool,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> Option<TableSplitDecision> {
    // SwTabFrame::Split(): first identify the row that crosses the cut
    // position and count the rows that remain in the master table, then apply
    // row-split, repeated-headline and keep-with-next rules in that order.
    let repeat_count = if start_row_index == 0 {
      self.frame.repeating_header_count
    } else {
      0
    };
    let mut current_y = y;
    let mut row_count = 0usize;
    let mut cut_row_index = None;
    let mut remaining_space_for_cut_row = 0.0;
    for row_index in start_row_index..self.table.rows.len() {
      let row = &self.table.rows[row_index];
      let row_frame = self.row_frame(row, row_index, current_y);
      if row_frame.bottom() > self.frame.block.content_bottom + LAYOUT_EPSILON_PT {
        cut_row_index = Some(row_index);
        remaining_space_for_cut_row = self.frame.block.content_bottom - current_y;
        break;
      }
      row_count += 1;
      current_y = row_frame.bottom();
      if row_index + 1 < self.table.rows.len() {
        current_y += row_cell_spacing_pt(self.table, row);
      }
    }
    let mut row_index = cut_row_index?;
    let old_row_index = row_index;

    let mut split_row_allowed = try_to_split_row
      && remaining_space_for_cut_row > LAYOUT_EPSILON_PT
      && self.row_can_split_at_cut(
        &self.row_frame(&self.table.rows[row_index], row_index, current_y),
        table_row_keep,
        allow_split_of_keep_row,
      );

    let mut keep_next_row = false;
    if row_count < repeat_count {
      return Some(TableSplitDecision {
        master_end_row_index: row_index,
        follow_start_row_index: None,
        split_row_allowed: false,
        move_rows_to_follow: false,
        creates_follow_flow_line: false,
        move_whole_table: has_ind_prev,
        disable_repeating_headers: !has_ind_prev,
      });
    } else if start_row_index == 0
      && row_count == repeat_count
      && (!split_row_allowed || self.row_contains_unsplittable_nested_table(row_index))
    {
      keep_next_row = true;
    }

    if keep_next_row {
      row_index = start_row_index.max(self.frame.repeating_header_count);
      if row_index < self.table.rows.len() {
        row_index += 1;
        row_count += 1;
      }
      if row_index >= self.table.rows.len() {
        return None;
      }
      split_row_allowed = false;
    }

    split_row_allowed = split_row_allowed
      && (!table_row_keep || !self.table.rows[row_index].keep_with_next || allow_split_of_keep_row);

    if !split_row_allowed && table_row_keep && !allow_split_of_keep_row {
      let mut previous_row_index = row_index.checked_sub(1);
      while let Some(previous) = previous_row_index {
        if previous < start_row_index || !self.table.rows[previous].keep_with_next {
          break;
        }
        if row_count <= repeat_count {
          break;
        }
        row_index = previous;
        row_count -= 1;
        previous_row_index = previous.checked_sub(1);
      }
      if row_count == repeat_count && start_row_index == 0 {
        row_index = old_row_index;
      }
    }

    if !split_row_allowed && self.is_first_non_header_row(row_index) {
      return Some(TableSplitDecision {
        master_end_row_index: row_index,
        follow_start_row_index: None,
        split_row_allowed: false,
        move_rows_to_follow: false,
        creates_follow_flow_line: false,
        move_whole_table: has_ind_prev,
        disable_repeating_headers: false,
      });
    }

    Some(TableSplitDecision {
      master_end_row_index: row_index,
      follow_start_row_index: Some(if split_row_allowed {
        row_index + 1
      } else {
        row_index
      }),
      split_row_allowed,
      move_rows_to_follow: !split_row_allowed,
      creates_follow_flow_line: split_row_allowed,
      move_whole_table: false,
      disable_repeating_headers: false,
    })
  }

  fn make_all_split_plan(
    &self,
    start_row_index: usize,
    y: f32,
    has_ind_prev: bool,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> TableMakeAllPlan {
    // SwTabFrame::MakeAll(): try splitting with row-split enabled first, then
    // retry the same split with bTryToSplit=false when the first attempt would
    // move the whole table or cannot satisfy the break-line precondition.
    if self.repeated_header_fallback_required_at_table_start(start_row_index, y) {
      return TableMakeAllPlan {
        split_decision: None,
        move_forward: false,
        disable_repeating_headers: true,
      };
    }
    let mut try_to_split_row = true;
    loop {
      let Some(decision) = self.table_split_decision(
        start_row_index,
        y,
        has_ind_prev,
        try_to_split_row,
        table_row_keep,
        allow_split_of_keep_row,
      ) else {
        return TableMakeAllPlan {
          split_decision: None,
          move_forward: false,
          disable_repeating_headers: false,
        };
      };
      if decision.disable_repeating_headers {
        return TableMakeAllPlan {
          split_decision: Some(decision),
          move_forward: false,
          disable_repeating_headers: true,
        };
      }
      if decision.move_whole_table && try_to_split_row {
        try_to_split_row = false;
        continue;
      }
      if !self.table_break_line_fits(
        start_row_index,
        y,
        has_ind_prev,
        try_to_split_row,
        table_row_keep,
        allow_split_of_keep_row,
      ) {
        if try_to_split_row {
          try_to_split_row = false;
          continue;
        }
        if self.frame.repeating_header_count > 0 {
          return TableMakeAllPlan {
            split_decision: Some(decision),
            move_forward: false,
            disable_repeating_headers: true,
          };
        }
        return TableMakeAllPlan {
          split_decision: Some(decision),
          move_forward: true,
          disable_repeating_headers: false,
        };
      }
      return TableMakeAllPlan {
        split_decision: Some(decision),
        move_forward: decision.move_whole_table,
        disable_repeating_headers: false,
      };
    }
  }

  fn repeated_header_fallback_required_at_table_start(
    &self,
    start_row_index: usize,
    y: f32,
  ) -> bool {
    // tdf#130639 fallback. If repeated headlines leave no room for the first
    // body row, Writer switches off row repetition and reformats the table in
    // place instead of starting a header-only split chain.
    if start_row_index != 0 || self.frame.repeating_header_count == 0 {
      return false;
    }
    let first_body_row_index = self.frame.repeating_header_count;
    if first_body_row_index >= self.table.rows.len() {
      return false;
    }
    let first_body_row_height = self.row_height(first_body_row_index);
    y + self.frame.repeating_header_height + first_body_row_height
      > self.frame.block.content_bottom + LAYOUT_EPSILON_PT
  }

  fn table_break_line_fits(
    &self,
    start_row_index: usize,
    y: f32,
    has_ind_prev: bool,
    try_to_split_row: bool,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> bool {
    // SwTabFrame::MakeAll() computes nBreakLine from the repeated headlines,
    // the keep-with-next row chain, and the second no-row-split attempt before
    // it calls SwTabFrame::Split(). If that minimum prefix cannot fit and the
    // table has an indirect previous frame, Writer moves the table forward
    // instead of constructing an invalid follow.
    if !has_ind_prev {
      return true;
    }

    let mut rows_needed = if start_row_index == 0 {
      self.frame.repeating_header_count
    } else {
      0
    };
    if table_row_keep && !allow_split_of_keep_row {
      let mut row_index = start_row_index.max(self.frame.repeating_header_count);
      while let Some(row) = self.table.rows.get(row_index) {
        if !row.keep_with_next {
          break;
        }
        rows_needed = rows_needed.max(row_index + 1 - start_row_index);
        row_index += 1;
      }
    }
    if !try_to_split_row {
      rows_needed = rows_needed.saturating_add(1);
    }
    if rows_needed == 0 {
      return true;
    }

    let mut current_y = y;
    for row_index in start_row_index..self.table.rows.len().min(start_row_index + rows_needed) {
      let row = &self.table.rows[row_index];
      current_y += self
        .frame
        .row_heights
        .get(row_index)
        .copied()
        .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT);
      if row_index + 1 < self.table.rows.len() {
        current_y += row_cell_spacing_pt(self.table, row);
      }
    }
    current_y <= self.frame.block.content_bottom + LAYOUT_EPSILON_PT
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
        .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT),
    }
  }

  fn layout_for_flow(
    &self,
    flow: FlowContext,
    repeating_headers_disabled: bool,
    text_metrics: &mut TextMetrics,
  ) -> Option<Self> {
    let area = block_area(flow);
    let mut layout = if area.setup == self.frame.block.setup
      && area.compatibility_mode == self.frame.block.compatibility_mode
      && f32::abs(area.content_width - self.frame.block.content_width) < LAYOUT_EPSILON_PT
    {
      // A Writer table follow keeps the master's column geometry and row
      // metrics. Only its upper/page frame changes. Re-measuring every row for
      // every follow page makes long tables quadratic in the number of rows.
      let mut layout = self.clone();
      let table_width = layout.frame.right_pt - layout.frame.left_pt;
      layout.frame.left_pt = table_left_position(
        layout.table,
        area.content_left_pt,
        area.content_width,
        table_width,
      );
      layout.frame.right_pt = layout.frame.left_pt + table_width;
      layout.frame.block = area;
      layout
    } else {
      TableFrameLayout::new(self.table, area, false, text_metrics)?
    };
    if repeating_headers_disabled {
      layout = layout.without_repeating_headers();
    }
    Some(layout)
  }

  fn without_repeating_headers(mut self) -> Self {
    // SwTabFrame::Split(), tdf#88496 fallback: if repeated headlines do not
    // fit at the start of the upper, disable the table's rows-to-repeat and
    // reformat instead of creating an endless follow chain.
    self.frame.repeating_header_count = 0;
    self.frame.repeating_header_height = 0.0;
    self
  }

  fn initial_dodge_height(&self) -> f32 {
    if !self.frame.split_allowed {
      return self.frame.total_height;
    }
    // SwTabFrame::MakeAll() only needs the repeat/keep break line to fit
    // before trying SwTabFrame::Split(); a split-capable table is not moved
    // forward just because its full height reaches the page bottom.
    let first_body_row_index = self.frame.repeating_header_count;
    let first_body_height = self
      .frame
      .row_heights
      .get(first_body_row_index)
      .copied()
      .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT);
    self.frame.repeating_header_height + first_body_height
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

  fn should_backward_move_follow_row_group(
    &self,
    row_index: usize,
    y: f32,
    text_metrics: &mut TextMetrics,
  ) -> bool {
    // SwTabFrame::MakeAll() calls GetFollow()->ShouldBwdMoved() before it
    // keeps the follow split. ShouldBwdMoved() compares the first content
    // line of the follow with the master upper's remaining space; if it fits,
    // the first follow row (plus covered row-span lines) is pasted back into
    // the master table.
    if row_index >= self.table.rows.len() {
      return false;
    }
    let remaining_space = self.frame.block.content_bottom - y;
    if remaining_space <= LAYOUT_EPSILON_PT {
      return false;
    }
    let first_content_height = self.calc_height_of_first_content_line(
      row_index,
      self.table_row_keep_enabled(),
      text_metrics,
    );
    first_content_height <= remaining_space + LAYOUT_EPSILON_PT
  }

  fn calc_height_of_first_content_line(
    &self,
    row_index: usize,
    table_row_keep: bool,
    text_metrics: &mut TextMetrics,
  ) -> f32 {
    // SwTabFrame::CalcHeightOfFirstContentLine(): for follow tables, repeated
    // headlines are ignored, keep-with-next rows are counted as full rows, and
    // the next splittable row contributes only its first content line.
    if !self.frame.split_allowed {
      return self.frame.total_height;
    }

    let mut first_content_row_index = row_index;
    let mut height = 0.0;
    if table_row_keep {
      while let Some(row) = self.table.rows.get(first_content_row_index) {
        if !row.keep_with_next {
          break;
        }
        height += self.row_height(first_content_row_index);
        first_content_row_index += 1;
      }
    }

    let Some(row) = self.table.rows.get(first_content_row_index) else {
      return height;
    };

    if !self.row_can_contribute_first_content_line(first_content_row_index, row, table_row_keep) {
      return height + self.row_height(first_content_row_index);
    }

    let first_line_height = self
      .row_first_content_line_height(first_content_row_index, row, text_metrics)
      .unwrap_or_else(|| self.row_height(first_content_row_index));
    let minimum_row_height = row.height_pt.unwrap_or(0.0).max(TABLE_ROW_MIN_HEIGHT_PT);
    height + first_line_height.max(minimum_row_height)
  }

  fn row_height(&self, row_index: usize) -> f32 {
    self
      .frame
      .row_heights
      .get(row_index)
      .copied()
      .unwrap_or(TABLE_ROW_MIN_HEIGHT_PT)
  }

  fn row_can_contribute_first_content_line(
    &self,
    row_index: usize,
    row: &TableRow,
    table_row_keep: bool,
  ) -> bool {
    !row.cant_split
      && !row.exact_height
      && !row_repeat_header_effective(self.table, row_index)
      && !row_has_vertical_merge_context(self.table, row_index)
      && (!table_row_keep || !row.keep_with_next)
  }

  fn row_first_content_line_height(
    &self,
    row_index: usize,
    row: &TableRow,
    text_metrics: &mut TextMetrics,
  ) -> Option<f32> {
    let mut grid_index = row.grid_before;
    let mut height: Option<f32> = None;
    for cell in &row.cells {
      let width = spanned_cell_width(cell, &self.frame.column_widths, &mut grid_index);
      if cell.vertical_merge_continue {
        continue;
      }
      let Some(cell_height) = table_cell_first_content_line_height(
        cell,
        width,
        self.frame.block.setup,
        TextSegmentation::TableCell,
        text_metrics,
      ) else {
        continue;
      };
      let row_line_height = cell_height - cell.margins.top_pt - cell.margins.bottom_pt
        + row_top_cell_margin_extent(row)
        + row_bottom_cell_margin_extent(row)
        + row_top_border_space_extent(self.table, row_index, row)
        + row_bottom_border_spacing_extent(self.table, row_index, row);
      height = Some(height.map_or(row_line_height, |current| current.max(row_line_height)));
    }
    height
  }

  fn row_last_rendered_page_break_follow_height(
    &self,
    row_index: usize,
    row: &TableRow,
    text_metrics: &mut TextMetrics,
  ) -> Option<f32> {
    if !row_uses_cached_vertical_merge_split(self.table, row_index, row) {
      return None;
    }
    // SwTextFrame follow from the cursor after a page break. The follow row
    // must therefore reserve the height of the remaining lower content, not
    // only the leftover twips of the original row frame.
    let mut grid_index = row.grid_before;
    let mut content_height = None;
    let row_top_margin = row_top_cell_margin_extent(row);
    let row_bottom_margin = row_bottom_cell_margin_extent(row);
    for cell in &row.cells {
      let width = spanned_cell_width(cell, &self.frame.column_widths, &mut grid_index);
      if cell.vertical_merge_continue {
        continue;
      }
      let Some(blocks) = table_cell_blocks_for_split_fragment(cell, true, true) else {
        continue;
      };
      if blocks.is_empty() {
        continue;
      }
      let mut follow_cell = cell.clone();
      follow_cell.blocks = blocks;
      let cell_height = table_cell_split_follow_content_height(
        &follow_cell,
        width,
        self.frame.block.setup,
        text_metrics,
      ) - cell.margins.top_pt
        - cell.margins.bottom_pt
        + row_top_margin
        + row_bottom_margin
        + row_top_border_space_extent(self.table, row_index, row)
        + row_bottom_border_spacing_extent(self.table, row_index, row);
      content_height =
        Some(content_height.map_or(cell_height, |height: f32| height.max(cell_height)));
    }
    content_height
  }

  fn maximum_layout_row_span(&self, row_index: usize) -> usize {
    // lcl_GetMaximumLayoutRowSpan(): when moving the first follow row
    // backward, include following covered rows so row-span layout stays
    // structurally valid.
    let mut rows_to_move = 1usize;
    let mut next_row_index = row_index + 1;
    while let Some(row) = self.table.rows.get(next_row_index) {
      if !row.cells.iter().any(|cell| cell.vertical_merge_continue) {
        break;
      }
      rows_to_move += 1;
      next_row_index += 1;
    }
    rows_to_move
  }

  fn is_first_non_header_row(&self, row_index: usize) -> bool {
    row_index == self.frame.repeating_header_count
  }

  fn row_can_split_at_cut(
    &self,
    row: &RowFrame<'_, '_>,
    table_row_keep: bool,
    allow_split_of_keep_row: bool,
  ) -> bool {
    let available_height = self.frame.block.content_bottom - row.y;
    let row_split_allowed = !row.row.cant_split || !row.fits_empty_body_region();
    let cached_vertical_merge_split =
      row_uses_cached_vertical_merge_split(self.table, row.row_index, row.row);
    row_split_allowed
      && available_height > LAYOUT_EPSILON_PT
      && (!row.row.exact_height || cached_vertical_merge_split)
      && !row_contains_following_text_flow_cell_floating(row.row)
      && !row_repeat_header_effective(self.table, row.row_index)
      && (!row_has_vertical_merge_context(self.table, row.row_index) || cached_vertical_merge_split)
      && (!table_row_keep || !row.row.keep_with_next || allow_split_of_keep_row)
      && Self::row_minimum_split_fragment_height(self.table, row.row_index, row.row)
        < available_height - LAYOUT_EPSILON_PT
  }

  fn row_contains_unsplittable_nested_table(&self, row_index: usize) -> bool {
    self.table.rows.get(row_index).is_some_and(|row| {
      row.cells.iter().any(|cell| {
        cell
          .blocks
          .iter()
          .any(|block| matches!(block, Block::Table(table) if !table.split_allowed))
      })
    })
  }

  fn table_row_keep_enabled(&self) -> bool {
    self.table.split_allowed && self.table.placement.is_none()
  }

  fn allow_split_of_keep_row(
    &self,
    start_row_index: usize,
    has_ind_prev: bool,
    table_row_keep: bool,
  ) -> bool {
    // SwTabFrame::MakeAll() derives bAllowSplitOfRow from the current
    // master/follow table: no indirect previous frame and all rows starting at
    // the current first non-headline row keep with next.
    table_row_keep
      && !has_ind_prev
      && self
        .are_body_rows_keep_with_next_from(start_row_index.max(self.frame.repeating_header_count))
  }

  fn are_body_rows_keep_with_next_from(&self, mut row_index: usize) -> bool {
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
    text_metrics: &mut TextMetrics,
    mut y: f32,
    row_height: f32,
  ) -> f32 {
    if self.frame.repeating_header_count == 0
      || y + self.frame.repeating_header_height > self.frame.block.content_bottom
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
        .format(current, pages, text_metrics);
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
    Self::row_minimum_split_fragment_height(self.table, row_index, row)
  }

  fn row_minimum_split_fragment_height(table: &Table, row_index: usize, row: &TableRow) -> f32 {
    // lcl_PreprocessRowsInCells() only splits a minimum-height row when the
    // available master fragment is larger than the row's minimum height;
    // otherwise the row is moved to the follow table.
    if row.exact_height {
      return 0.0;
    }
    row.height_pt.map_or(0.0, |height| {
      height
        + row_top_border_space_extent(table, row_index, row)
        + row_bottom_border_spacing_extent(table, row_index, row)
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
  row_heights: Arc<[f32]>,
  repeating_header_count: usize,
  repeating_header_height: f32,
  total_height: f32,
}

#[derive(Clone, Copy, Debug)]
struct TableSplitDecision {
  master_end_row_index: usize,
  follow_start_row_index: Option<usize>,
  split_row_allowed: bool,
  move_rows_to_follow: bool,
  creates_follow_flow_line: bool,
  move_whole_table: bool,
  disable_repeating_headers: bool,
}

#[derive(Clone, Copy, Debug)]
struct TableMakeAllPlan {
  split_decision: Option<TableSplitDecision>,
  move_forward: bool,
  disable_repeating_headers: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct TableFollowState {
  has_follow_flow_line: bool,
  split_row_index: Option<usize>,
  remaining_height_pt: f32,
  content_offset_pt: f32,
}

impl TableFollowState {
  fn set_follow_flow_line(
    &mut self,
    row_index: usize,
    remaining_height_pt: f32,
    content_offset_pt: f32,
  ) {
    self.has_follow_flow_line = remaining_height_pt > LAYOUT_EPSILON_PT;
    self.split_row_index = self.has_follow_flow_line.then_some(row_index);
    self.remaining_height_pt = remaining_height_pt.max(0.0);
    self.content_offset_pt = content_offset_pt.max(0.0);
  }

  fn clear_follow_flow_line(&mut self) {
    self.has_follow_flow_line = false;
    self.split_row_index = None;
    self.remaining_height_pt = 0.0;
    self.content_offset_pt = 0.0;
  }
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

  fn format(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    text_metrics: &mut TextMetrics,
  ) -> f32 {
    let row_top = self.y;
    let row_bottom = self.bottom();
    self.format_fragment(current, pages, text_metrics, row_top, row_bottom, 0.0);
    row_bottom
  }

  fn format_fragment(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    text_metrics: &mut TextMetrics,
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
        cell_frame.format(
          current,
          pages,
          text_metrics,
          row_top,
          row_bottom,
          content_offset,
        );
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
          // A table-cell fragment owns the cell box, not the ink bounds of
          // whichever text item happens to be first. PDF painting uses this
          // box to clip inline cell content at the row edges.
          bounds: Some(FrameBounds {
            x_pt: cell_frame.left_pt,
            y_pt: row_top,
            width_pt: cell_frame.width_pt,
            height_pt: (row_bottom - row_top).max(0.0),
          }),
        },
        text_metrics,
      );
      cell_left += cell_frame.width_pt + cell_spacing_pt;
    }

    self.paint_horizontal_borders(current, row_top, row_bottom);
    let row_left = row_grid_left(self.table_frame, self.row, cell_spacing_pt);
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
        bounds: Some(FrameBounds {
          x_pt: row_left,
          y_pt: row_top,
          width_pt: self.table_frame.right_pt - row_left,
          height_pt: row_bottom - row_top,
        }),
      },
      text_metrics,
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
    text_metrics: &mut TextMetrics,
    row_top: f32,
    row_bottom: f32,
    content_offset: f32,
  ) {
    let fragment_height = (row_bottom - row_top).max(0.0);
    let cell_fragment_height =
      if table_cell_has_vertical_merge_follow(self.table, self.row_index, self.grid_start) {
        self.content_height(text_metrics).max(fragment_height)
      } else {
        fragment_height
      };
    self.paint_vertical_merge_bounds(current, row_top, fragment_height);
    self.paint_background(current, row_top, fragment_height);
    self.paint_leading_border(current, row_top, row_bottom);
    self.paint_trailing_border(current, row_top, row_bottom);
    layout_table_cell(TableCellLayout {
      cell: self.cell,
      table_following_text_flow: self.table.following_text_flow,
      escape_following_text_flow_pages: true,
      setup: self.table_frame.block.setup,
      compatibility_mode: self.table_frame.block.compatibility_mode,
      current,
      pages,
      text_metrics,
      x: self.left_pt,
      y: row_top,
      width: self.width_pt,
      height: cell_fragment_height,
      row_top_margin_pt: row_top_cell_margin_extent(self.row),
      row_bottom_margin_pt: row_bottom_cell_margin_extent(self.row),
      content_offset,
      use_cached_break_inside_block: row_uses_cached_vertical_merge_split(
        self.table,
        self.row_index,
        self.row,
      ),
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

  fn content_height(&self, text_metrics: &mut TextMetrics) -> f32 {
    vertical_merge_content_height(
      self.table,
      &self.table_frame.column_widths,
      VerticalMergeSpan {
        row_index: self.row_index,
        grid_start: self.grid_start,
        current_row_height: self.height_pt,
      },
      self.table_frame.block.setup,
      self.table_frame.block.compatibility_mode,
      text_metrics,
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

#[derive(Clone, Copy)]
struct VerticalMergeSpan {
  row_index: usize,
  grid_start: usize,
  current_row_height: f32,
}

fn vertical_merge_content_height(
  table: &Table,
  column_widths: &[f32],
  span: VerticalMergeSpan,
  setup: PageSetup,
  compatibility_mode: u16,
  text_metrics: &mut TextMetrics,
) -> Option<f32> {
  let mut height = span.current_row_height;
  let mut previous_row = table.rows.get(span.row_index)?;
  let mut has_continuation = false;
  for (follow_row_index, row) in table.rows.iter().enumerate().skip(span.row_index + 1) {
    let Some(cell) = row_cell_at_grid(row, span.grid_start) else {
      break;
    };
    if !cell.vertical_merge_continue {
      break;
    }
    height += row_cell_spacing_pt(table, previous_row);
    height += table_row_height_with_widths(
      table,
      follow_row_index,
      row,
      column_widths,
      setup,
      compatibility_mode,
      text_metrics,
    );
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

fn row_uses_cached_vertical_merge_split(table: &Table, row_index: usize, row: &TableRow) -> bool {
  // Writer normally clips an exact-height row. The fixed-layout cursor in
  // tdf#128959 is the exceptional case: an exact row starts a vertically
  // merged cell whose SwTextFrame already has a page follow. Preserve that
  // source-backed master/follow split without making exact rows generally
  // grow or split. Floating tables own a separate split-fly follow chain.
  table.placement.is_none()
    && row.exact_height
    && row_has_vertical_merge_context(table, row_index)
    && row_contains_last_rendered_page_break(row)
}

fn row_contains_following_text_flow_cell_floating(row: &TableRow) -> bool {
  row
    .cells
    .iter()
    .any(|cell| blocks_contain_following_text_flow_cell_floating(&cell.blocks))
}

fn blocks_contain_following_text_flow_cell_floating(blocks: &[Block]) -> bool {
  blocks.iter().any(|block| match block {
    Block::Paragraph(paragraph) => paragraph.inlines.iter().any(|inline| match inline {
      InlineItem::Image(image) => matches!(
        image.placement,
        crate::docx::ImagePlacement::Floating(placement)
          if placement.layout_in_cell && !placement.behind_text
      ),
      InlineItem::Shape(shape) => matches!(
        shape.placement,
        crate::docx::ImagePlacement::Floating(placement)
          if placement.layout_in_cell && !placement.behind_text
      ),
      InlineItem::Text(_)
      | InlineItem::BookmarkStart(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_)
      | InlineItem::LastRenderedPageBreak
      | InlineItem::PageBreak
      | InlineItem::ColumnBreak => false,
    }),
    Block::Table(table) => table
      .rows
      .iter()
      .any(row_contains_following_text_flow_cell_floating),
    Block::Frame(frame) => blocks_contain_following_text_flow_cell_floating(&frame.blocks),
  })
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
  if table.explicit_no_repeat_header {
    return 0;
  }
  let count = leading_repeat_header_count(table);
  if count == 0
    || count > HEADER_ROW_LIMIT_FOR_MSO_WORKAROUND
    || table_disables_repeating_headers(table, count)
  {
    0
  } else {
    count
  }
}
// HEADER_ROW_LIMIT_FOR_MSO_WORKAROUND. DOCX imports that mark more than this
// many leading rows as headers disable repeating headers to match MS Word.
const HEADER_ROW_LIMIT_FOR_MSO_WORKAROUND: usize = 10;

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
  !(leading_repeat_header_count <= HEADER_ROW_LIMIT_FOR_MSO_WORKAROUND
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
    // ECMA-376 Part 1 §17.18.87 defines an auto/nil `w:tblW` from the
    // row and cell widths. In that case an explicit `w:tblGrid` is the table
    // width, even when it extends beyond the text area. Only an explicit table
    // preferred width constrains and proportionally scales those columns.
    if !allow_width_overflow && preferred_width.is_some() {
      clamp_widths_to_content(&mut widths, content_width);
    }
    return widths;
  }

  if let Some(mut widths) = table_cell_preferred_column_widths(table, column_count, content_width) {
    if let Some(preferred) = preferred_width {
      scale_widths_to_total(&mut widths, preferred);
    }
    if !allow_width_overflow && preferred_width.is_some() {
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

  // Reconstruct malformed or incomplete tblGrid data from the narrowest
  // constraints first. A one-column tcW defines that grid column directly;
  // a later gridSpan cell only constrains the sum. Processing document row
  // order would let an early spanning cell divide its width equally and then
  // prevent narrower explicit cells in a later row from recovering the grid.
  for constraint_span in 1..=column_count {
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
        if span == constraint_span {
          let width = cell
            .preferred_width_pt
            .or_else(|| cell.preferred_width_pct.map(|pct| content_width * pct))
            .unwrap_or(0.0);
          if width > 0.0 {
            saw_preferred_width = true;
            grow_spanned_columns_to_width(&mut widths[grid_index..grid_index + span], width);
          }
        }
        grid_index += span;
      }
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
    TableAlignment::Left => {
      let leading_cell_margin = if table.align_leading_cell_content {
        table
          .rows
          .first()
          .and_then(|row| row.cells.first())
          .map(|cell| cell.margins.left_pt)
          .unwrap_or(0.0)
      } else {
        0.0
      };
      content_left + table.indent_left_pt.min(available) - leading_cell_margin
    }
    TableAlignment::Center => content_left + available / 2.0,
    TableAlignment::Right => content_left + available,
  }
}

fn table_total_height_with_widths(
  table: &Table,
  column_widths: &[f32],
  text_metrics: &mut TextMetrics,
) -> f32 {
  let row_heights =
    table_row_heights_with_widths(table, column_widths, PageSetup::default(), 12, text_metrics);
  table_total_height_from_row_heights(table, &row_heights)
}

fn table_row_heights_with_widths(
  table: &Table,
  column_widths: &[f32],
  setup: PageSetup,
  compatibility_mode: u16,
  text_metrics: &mut TextMetrics,
) -> Vec<f32> {
  table
    .rows
    .iter()
    .enumerate()
    .map(|(row_index, row)| {
      table_row_height_with_widths(
        table,
        row_index,
        row,
        column_widths,
        setup,
        compatibility_mode,
        text_metrics,
      )
    })
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

fn estimated_table_height(table: &Table, flow: FlowContext, text_metrics: &mut TextMetrics) -> f32 {
  TableFrameLayout::new(table, block_area(flow), false, text_metrics)
    .map(|layout| layout.frame.total_height)
    .unwrap_or_else(|| table_total_height_with_widths(table, &[], text_metrics))
}

fn table_row_height_with_widths(
  table: &Table,
  row_index: usize,
  row: &TableRow,
  column_widths: &[f32],
  setup: PageSetup,
  compatibility_mode: u16,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let mut grid_index = row.grid_before;
  let mut content_height = TABLE_ROW_MIN_HEIGHT_PT;
  let row_top_margin = row_top_cell_margin_extent(row);
  let row_bottom_margin = row_bottom_cell_margin_extent(row);
  for cell in &row.cells {
    let width = spanned_cell_width(cell, column_widths, &mut grid_index);
    if !cell.vertical_merge_continue {
      let cell_height = table_cell_content_height_for_table(
        cell,
        width,
        setup,
        table.following_text_flow,
        compatibility_mode,
        text_metrics,
      ) - cell.margins.top_pt
        - cell.margins.bottom_pt
        + row_top_margin
        + row_bottom_margin;
      content_height = content_height.max(cell_height);
    }
  }
  match (row.height_pt, row.exact_height) {
    (Some(_), true) if row_uses_cached_vertical_merge_split(table, row_index, row) => {
      content_height
        + row_top_border_space_extent(table, row_index, row)
        + row_bottom_border_spacing_extent(table, row_index, row)
    }
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
    // A collapsed top/inside border still occupies row geometry. The explicit
    // at-least-height and first-line paths account for the same extent above;
    // automatic rows must do so as well or every following row drifts upward
    // by one border width.
    (None, _) => {
      content_height
        + row_top_border_space_extent(table, row_index, row)
        + row_bottom_border_spacing_extent(table, row_index, row)
    }
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
  table_following_text_flow: bool,
  escape_following_text_flow_pages: bool,
  setup: PageSetup,
  compatibility_mode: u16,
  current: &'a mut Page,
  pages: &'a mut Vec<Page>,
  text_metrics: &'a mut TextMetrics,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  row_top_margin_pt: f32,
  row_bottom_margin_pt: f32,
  content_offset: f32,
  use_cached_break_inside_block: bool,
}

fn layout_table_cell(fragment: TableCellLayout<'_>) {
  let TableCellLayout {
    cell,
    table_following_text_flow,
    escape_following_text_flow_pages,
    setup,
    compatibility_mode,
    current,
    pages,
    text_metrics,
    x,
    y,
    width,
    height,
    row_top_margin_pt,
    row_bottom_margin_pt,
    content_offset,
    use_cached_break_inside_block,
  } = fragment;
  let vertical_text = cell.text_rotation_deg.is_some();
  // ECMA-376 Part 1 §17.18.93 defines btLr/tbRl as vertical writing
  // modes. At final cell layout the physical inner height is therefore the
  // logical line width; rotating a line measured against the table column
  // width makes it overflow instead of forming vertical columns.
  let content_width = if vertical_text {
    (height - cell.margins.top_pt - cell.margins.bottom_pt).max(DEFAULT_FONT_SIZE_PT)
  } else {
    (width - cell.margins.left_pt - cell.margins.right_pt).max(DEFAULT_FONT_SIZE_PT)
  };
  let content_height = table_cell_content_height_for_table(
    cell,
    width,
    setup,
    table_following_text_flow,
    compatibility_mode,
    text_metrics,
  );
  let first_line_style = table_cell_first_line_style(cell);
  let first_line_height = table_cell_first_inline_text_height(cell, true, text_metrics);
  // Table text is stored at its resolved baseline because PDF paint owners do
  // not add a second line offset for table frames. Word's Windows fixed-layout
  // path uses the Windows ascent here ([MS-DOCX] usePre2018iOSMacLayout); using
  // only the OS/2 typographic ascent puts Calibri table text above the Office
  // baseline even though the row box itself is correct.
  let first_line_baseline_offset =
    text_metrics.baseline_offset_in_line_with_windows_metrics(&first_line_style, first_line_height);
  let split_fragment =
    content_offset > LAYOUT_EPSILON_PT || content_height > height + LAYOUT_EPSILON_PT;
  let top_margin_for_lowers = row_top_margin_pt.max(cell.margins.top_pt);
  let bottom_margin_for_lowers = row_bottom_margin_pt.max(cell.margins.bottom_pt);
  let aligned_content_top = if vertical_text {
    y + height - bottom_margin_for_lowers
  } else if split_fragment {
    y + top_margin_for_lowers
  } else {
    match cell.vertical_alignment {
      TableCellVerticalAlignment::Top => y + top_margin_for_lowers,
      TableCellVerticalAlignment::Center => {
        y + ((height - content_height) / 2.0).max(top_margin_for_lowers)
      }
      TableCellVerticalAlignment::Bottom => {
        y + (height - bottom_margin_for_lowers - content_height).max(top_margin_for_lowers)
      }
    }
  };
  let split_blocks = if content_offset > LAYOUT_EPSILON_PT {
    table_cell_blocks_for_split_fragment(cell, true, use_cached_break_inside_block)
  } else if split_fragment && !table_following_text_flow {
    table_cell_blocks_for_split_fragment(cell, false, use_cached_break_inside_block)
  } else {
    None
  };
  if split_blocks.is_some() && content_offset > LAYOUT_EPSILON_PT {
    // real text break position. A follow fragment that starts after that
    // marker is not a candidate for SwTabFrame::Join() back into its master.
    current.explicit_break_target = true;
  }
  let blocks_to_layout = split_blocks.as_deref().unwrap_or(&cell.blocks);
  let effective_content_offset = if split_blocks.is_some() {
    0.0
  } else {
    content_offset
  };
  let mut text_y = aligned_content_top + first_line_baseline_offset - effective_content_offset;
  let content_start_y = text_y;
  let text_left = x + cell.margins.left_pt;
  let text_bottom = y + height - bottom_margin_for_lowers;
  let following_text_flow_bottom = following_text_flow_cell_bottom(current, text_bottom);
  let flow_bottom = if vertical_text {
    UNBOUNDED_LAYOUT_EXTENT_PT
  } else if split_fragment {
    text_bottom
  } else {
    UNBOUNDED_LAYOUT_EXTENT_PT
  };
  let rotation_center = vertical_text.then_some((text_left, aligned_content_top));
  let flow = FlowContext {
    setup,
    section_index: current.section_index,
    section_page_index: current.section_page_index,
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
    layout_cell_print_bounds: Some(FrameBounds {
      x_pt: text_left,
      y_pt: y + top_margin_for_lowers,
      width_pt: content_width,
      height_pt: (height - top_margin_for_lowers - bottom_margin_for_lowers).max(0.0),
    }),
    default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
    compatibility_mode,
    justify_lines_with_shrinking: compatibility_mode >= 15,
    split_page_break_and_paragraph_mark: false,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::TableCell,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
    preserve_horizontal_on_advance: false,
    script_sensitive_line_height: true,
  };
  let mut nested_page =
    empty_section_page(setup, current.section_index, current.section_page_index);
  let mut discarded_pages = Vec::new();
  let has_following_text_flow_table = cell.blocks.iter().any(|block| {
    matches!(
      block,
      Block::Table(table) if table.placement.is_some() && table.following_text_flow
    )
  });

  if has_following_text_flow_table && escape_following_text_flow_pages {
    for (index, block) in blocks_to_layout.iter().enumerate() {
      let previous = index
        .checked_sub(1)
        .and_then(|index| blocks_to_layout.get(index));
      let next = blocks_to_layout.get(index + 1);
      if content_offset > LAYOUT_EPSILON_PT
        && matches!(
          block,
          Block::Table(table) if table.placement.is_some() && table.following_text_flow
        )
      {
        continue;
      }
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
        LayoutBlockTarget {
          current,
          pages,
          anchor_pages: None,
          text_metrics: &mut *text_metrics,
        },
        text_y,
      );
      text_y = next_y;
    }
    return;
  }

  for (index, block) in blocks_to_layout.iter().enumerate() {
    if !vertical_text && text_y > text_bottom {
      break;
    }
    let previous = index
      .checked_sub(1)
      .and_then(|index| blocks_to_layout.get(index));
    let next = blocks_to_layout.get(index + 1);
    if content_offset > LAYOUT_EPSILON_PT
      && matches!(
        block,
        Block::Table(table) if table.placement.is_some() && table.following_text_flow
      )
    {
      continue;
    }
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
      LayoutBlockTarget {
        current: &mut nested_page,
        pages: &mut discarded_pages,
        anchor_pages: None,
        text_metrics: &mut *text_metrics,
      },
      text_y,
    );
    text_y = next_y;
  }
  materialize_pending_floating_table_follows_in_local_pages(&mut nested_page, &mut discarded_pages);

  if has_following_text_flow_table && !split_fragment {
    let mut local_pages = ordered_local_pages(nested_page, discarded_pages).into_iter();
    if let Some(mut first_page) = local_pages.next() {
      let item_start = current.items.len();
      current.items.extend(
        first_page
          .items
          .into_iter()
          .filter(|item| table_cell_item_intersects_vertical_bounds(item, y, text_bottom)),
      );
      let item_end = current.items.len();
      current
        .frame_fragments
        .extend(
          first_page
            .frame_fragments
            .into_iter()
            .filter_map(|mut fragment| {
              let bounds = fragment.bounds?;
              if !frame_bounds_intersects_vertical_bounds(bounds, y, text_bottom) {
                return None;
              }
              fragment.bounds = Some(bounds);
              fragment.item_start = item_start;
              fragment.item_end = item_end;
              Some(fragment)
            }),
        );
      current
        .pending_floating_table_follows
        .append(&mut first_page.pending_floating_table_follows);
    }
    for follow_page in local_pages {
      if follow_page.items.is_empty()
        && follow_page.frame_fragments.is_empty()
        && follow_page.pending_floating_table_follows.is_empty()
      {
        continue;
      }
      current
        .pending_floating_table_follows
        .push(PendingFloatingTableFollow {
          setup: follow_page.setup,
          section_index: follow_page.section_index,
          section_page_index: follow_page.section_page_index,
          items: follow_page.items,
          frame_fragments: follow_page.frame_fragments,
          frame_influences: Vec::new(),
          wrap_exclusions: follow_page.wrap_exclusions,
          pending_floating_table_follows: follow_page.pending_floating_table_follows,
        });
    }
    return;
  }

  let (mut nested_items, nested_fragments) =
    flatten_nested_pages_with_fragments(nested_page, discarded_pages, content_start_y, text_bottom);
  if let Some(rotation_center) = rotation_center {
    for item in &mut nested_items {
      if let PageItem::Text(text) = item {
        text.rotation_center_pt = Some(rotation_center);
      }
    }
  }
  let item_start = current.items.len();
  current
    .items
    .extend(nested_items.into_iter().filter(|item| {
      vertical_text || table_cell_item_intersects_vertical_bounds(item, y, text_bottom)
    }));
  let item_end = current.items.len();
  current
    .frame_fragments
    .extend(nested_fragments.into_iter().filter_map(|mut fragment| {
      let bounds = fragment.bounds?;
      if !frame_bounds_intersects_vertical_bounds(bounds, y, text_bottom) {
        return None;
      }
      fragment.bounds = Some(bounds);
      fragment.item_start = item_start;
      fragment.item_end = item_end;
      Some(fragment)
    }));
}

fn following_text_flow_cell_bottom(current: &Page, text_bottom: f32) -> f32 {
  // SwFlyFrame::Grow_(): split fly growth is limited by the current body
  // deadline. A cell whose estimated height grows past the page bottom must
  // still split the nested following-text-flow floating table on this page.
  let _ = text_bottom;
  current.setup.height_pt - current.setup.margin_bottom_pt
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

fn table_cell_first_inline_text_height(
  cell: &TableCell,
  script_sensitive_line_height: bool,
  text_metrics: &mut TextMetrics,
) -> f32 {
  for block in &cell.blocks {
    let Block::Paragraph(paragraph) = block else {
      continue;
    };
    for inline in &paragraph.inlines {
      let InlineItem::Text(run) = inline else {
        continue;
      };
      if text_run_affects_line_height(&run.text) {
        return if script_sensitive_line_height {
          inline_text_height_for_text(&run.style, &run.text, text_metrics)
        } else {
          inline_text_height(&run.style, text_metrics)
        };
      }
    }
    return if script_sensitive_line_height {
      inline_text_height_for_text(&paragraph.base_style, "", text_metrics)
    } else {
      inline_text_height(&paragraph.base_style, text_metrics)
    };
  }
  inline_text_height(&TextStyle::default(), text_metrics)
}

fn table_cell_blocks_for_split_fragment(
  cell: &TableCell,
  follow_fragment: bool,
  include_contained_break: bool,
) -> Option<Vec<Block>> {
  // evidence on the following text portion. When a table row is already split,
  // SwTextFrame follows are laid out from the text cursor after the break, not
  // from the whole paragraph again; split the imported block stream at that
  // cursor before formatting the master/follow cell fragments.
  let split_index = cell.blocks.iter().position(|block| {
    if include_contained_break {
      block_contains_last_rendered_page_break(block)
    } else {
      block_starts_after_last_rendered_page_break(block)
    }
  })?;

  let mut blocks = Vec::new();
  if !follow_fragment {
    blocks.extend(cell.blocks[..split_index].iter().cloned());
  }

  match &cell.blocks[split_index] {
    Block::Paragraph(paragraph) => {
      if let Some(paragraph) =
        paragraph_fragment_around_last_rendered_page_break(paragraph, follow_fragment)
      {
        blocks.push(Block::paragraph(paragraph));
      }
    }
    block if follow_fragment => blocks.push(block.clone()),
    _ => {}
  }

  if follow_fragment {
    blocks.extend(cell.blocks[split_index + 1..].iter().cloned());
  }
  Some(blocks)
}

fn block_starts_after_last_rendered_page_break(block: &Block) -> bool {
  match block {
    Block::Paragraph(paragraph) => paragraph.starts_after_last_rendered_page_break,
    Block::Table(table) => table.starts_after_last_rendered_page_break,
    Block::Frame(frame) => frame
      .blocks
      .first()
      .is_some_and(block_starts_after_last_rendered_page_break),
  }
}

fn block_contains_last_rendered_page_break(block: &Block) -> bool {
  match block {
    Block::Paragraph(paragraph) => paragraph
      .inlines
      .iter()
      .any(|inline| matches!(inline, InlineItem::LastRenderedPageBreak)),
    Block::Table(table) => table.rows.iter().any(row_contains_last_rendered_page_break),
    Block::Frame(frame) => blocks_contain_last_rendered_page_break(&frame.blocks),
  }
}

fn paragraph_fragment_around_last_rendered_page_break(
  paragraph: &crate::docx::Paragraph,
  follow_fragment: bool,
) -> Option<crate::docx::Paragraph> {
  let split_index = paragraph
    .inlines
    .iter()
    .position(|inline| matches!(inline, InlineItem::LastRenderedPageBreak))?;
  let inlines = if follow_fragment {
    paragraph.inlines[split_index + 1..].to_vec()
  } else {
    paragraph.inlines[..split_index].to_vec()
  };
  if paragraph_inlines_are_layout_empty(&inlines) {
    return None;
  }
  let mut fragment = paragraph.clone();
  fragment.inlines = inlines;
  fragment.starts_after_last_rendered_page_break = false;
  Some(fragment)
}

fn paragraph_inlines_are_layout_empty(inlines: &[InlineItem]) -> bool {
  inlines.iter().all(|inline| match inline {
    InlineItem::Text(run) => run.text.trim().is_empty(),
    InlineItem::BookmarkStart(_) => true,
    InlineItem::FormWidgetStart(_) | InlineItem::FormWidgetEnd(_) => true,
    InlineItem::LastRenderedPageBreak => true,
    InlineItem::Image(_)
    | InlineItem::Shape(_)
    | InlineItem::PageBreak
    | InlineItem::ColumnBreak => false,
  })
}

fn table_cell_first_content_line_height(
  cell: &TableCell,
  width_pt: f32,
  setup: PageSetup,
  text_segmentation: TextSegmentation,
  text_metrics: &mut TextMetrics,
) -> Option<f32> {
  // lcl_CalcHeightOfFirstContentLine(): inspect the first lower frame in the
  // cell. Text contributes FirstLineHeight(), nested tables recurse into
  // CalcHeightOfFirstContentLine().
  let content_width = (width_pt - cell.margins.left_pt - cell.margins.right_pt).max(0.0);
  if let Some(block) = cell.blocks.first() {
    match block {
      Block::Paragraph(paragraph) => {
        let style = paragraph_base_line_style(paragraph);
        let line_height = paragraph_line_height_for_setup(
          paragraph,
          &style,
          setup,
          text_segmentation,
          text_metrics,
        );
        let flow = FlowContext {
          setup,
          section_index: 0,
          section_page_index: 0,
          column_index: 0,
          columns: SectionColumns::default(),
          content_top_pt: 0.0,
          content_left_pt: cell.margins.left_pt,
          content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
          body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
          content_width,
          layout_cell_bounds: None,
          layout_cell_print_bounds: None,
          default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
          compatibility_mode: 12,
          justify_lines_with_shrinking: false,
          split_page_break_and_paragraph_mark: false,
          repeating_slots: RepeatingSlotState::default(),
          text_segmentation,
          paragraph_spacing_context: ParagraphSpacingContext::Normal,
          preserve_horizontal_on_advance: false,
          script_sensitive_line_height: true,
        };
        let next = cell.blocks.get(1);
        let extra_last_lower = if next.is_none() {
          paragraph_add_lower_space_as_last_in_table_cell(paragraph, flow)
        } else {
          0.0
        };
        return Some(
          line_height
            + paragraph_spacing_before(None, paragraph, flow)
            + paragraph_spacing_after(paragraph, next)
            + extra_last_lower,
        );
      }
      Block::Table(table) => {
        let area = BlockArea {
          setup,
          section_index: 0,
          section_page_index: 0,
          column_index: 0,
          columns: SectionColumns::default(),
          content_top_pt: 0.0,
          content_left_pt: 0.0,
          content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
          body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
          content_width,
          default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
          compatibility_mode: 12,
          justify_lines_with_shrinking: false,
          repeating_slots: RepeatingSlotState::default(),
        };
        return TableFrameLayout::new(table, area, false, text_metrics).map(|layout| {
          layout.calc_height_of_first_content_line(0, layout.table_row_keep_enabled(), text_metrics)
        });
      }
      Block::Frame(frame) => {
        return Some(frame.height_pt.unwrap_or_else(|| {
          let flow = FlowContext {
            setup,
            section_index: 0,
            section_page_index: 0,
            column_index: 0,
            columns: SectionColumns::default(),
            content_top_pt: 0.0,
            content_left_pt: 0.0,
            content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
            body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
            content_width,
            layout_cell_bounds: None,
            layout_cell_print_bounds: None,
            default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
            compatibility_mode: 12,
            justify_lines_with_shrinking: false,
            split_page_break_and_paragraph_mark: false,
            repeating_slots: RepeatingSlotState::default(),
            text_segmentation,
            paragraph_spacing_context: ParagraphSpacingContext::Normal,
            preserve_horizontal_on_advance: false,
            script_sensitive_line_height: true,
          };
          frame
            .blocks
            .iter()
            .map(|block| estimated_block_height(block, flow, text_metrics))
            .sum::<f32>()
            .max(TABLE_ROW_MIN_HEIGHT_PT)
        }));
      }
    }
  }
  None
}

fn table_cell_item_intersects_vertical_bounds(item: &PageItem, top: f32, bottom: f32) -> bool {
  match item {
    PageItem::Text(text) => text.y_pt + text.line_height_pt >= top && text.y_pt <= bottom,
    PageItem::Image(image) => image.y_pt + image.height_pt >= top && image.y_pt <= bottom,
    PageItem::Rect(rect) => rect.y_pt + rect.height_pt >= top && rect.y_pt <= bottom,
    PageItem::Fill(_) | PageItem::Line(_) | PageItem::Polyline(_) => true,
  }
}

fn frame_bounds_intersects_vertical_bounds(bounds: FrameBounds, top: f32, bottom: f32) -> bool {
  bounds.y_pt + bounds.height_pt >= top && bounds.y_pt <= bottom
}

#[derive(Clone, Copy)]
struct ShapeTextBoxRect {
  x: f32,
  y: f32,
  width: f32,
  height: f32,
}

fn layout_shape_text_box(
  current: &mut Page,
  parent_flow: FlowContext,
  text_metrics: &mut TextMetrics,
  shape: &crate::docx::InlineShape,
  rect: ShapeTextBoxRect,
) {
  if shape.text_box_blocks.is_empty() {
    return;
  }

  let content_left = rect.x + shape.text_inset_left_pt;
  let content_top = rect.y + shape.text_inset_top_pt;
  let content_width =
    (rect.width - shape.text_inset_left_pt - shape.text_inset_right_pt).max(DEFAULT_FONT_SIZE_PT);
  let content_bottom = rect.y + rect.height - shape.text_inset_bottom_pt;
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
    layout_cell_print_bounds: parent_flow.layout_cell_print_bounds,
    default_tab_stop_pt: parent_flow.default_tab_stop_pt,
    compatibility_mode: parent_flow.compatibility_mode,
    justify_lines_with_shrinking: parent_flow.justify_lines_with_shrinking,
    split_page_break_and_paragraph_mark: parent_flow.split_page_break_and_paragraph_mark,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::TableCell,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
    preserve_horizontal_on_advance: false,
    script_sensitive_line_height: true,
  };
  let content_height = shape
    .text_box_blocks
    .iter()
    .map(|block| estimated_block_height(block, measure_flow, text_metrics))
    .sum::<f32>();
  let text_y = match shape.text_vertical_alignment {
    TextBoxVerticalAlignment::Top => content_top,
    TextBoxVerticalAlignment::Center => {
      content_top + ((content_bottom - content_top - content_height) / 2.0).max(0.0)
    }
    TextBoxVerticalAlignment::Bottom => (content_bottom - content_height).max(content_top),
  };

  let flow = FlowContext {
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
  let shape_y = rect.y;
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
      LayoutBlockTarget {
        current: &mut nested_page,
        pages: &mut discarded_pages,
        anchor_pages: None,
        text_metrics: &mut *text_metrics,
      },
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
            text_metrics,
            rect.x + auto_fit_inset,
            shape_y + auto_fit_inset,
            (rect.width - auto_fit_inset * 2.0).max(DEFAULT_FONT_SIZE_PT),
            (rect.height - auto_fit_inset * 2.0).max(DEFAULT_LINE_HEIGHT_PT),
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

fn shape_has_invisible_auto_fit_textbox_bounds(
  shape: &crate::docx::InlineShape,
  flow: FlowContext,
) -> bool {
  shape.text_box_auto_fit
    && !shape.text_box_blocks.is_empty()
    && flow.text_segmentation == TextSegmentation::RepeatingSlot
    && shape.fill_color.is_none()
    && shape.fill_image.is_none()
    && shape.stroke.is_none()
    && shape.additional_fill_colors.is_empty()
}

fn textbox_item_inside_shape_bounds(
  mut item: PageItem,
  text_metrics: &mut TextMetrics,
  left: f32,
  top: f32,
  width: f32,
  height: f32,
) -> PageItem {
  let right = left + width;
  let bottom = top + height;
  let Some((item_left, item_top, item_right, item_bottom)) = item_bounds(&item, text_metrics)
  else {
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

fn table_cell_content_height_for_table(
  cell: &TableCell,
  cell_width: f32,
  setup: PageSetup,
  script_sensitive_line_height: bool,
  compatibility_mode: u16,
  text_metrics: &mut TextMetrics,
) -> f32 {
  table_cell_content_height_with_mode(
    cell,
    cell_width,
    setup,
    TableCellMeasureMode::WholeCell,
    script_sensitive_line_height,
    compatibility_mode,
    text_metrics,
  )
}

fn table_cell_split_follow_content_height(
  cell: &TableCell,
  cell_width: f32,
  setup: PageSetup,
  text_metrics: &mut TextMetrics,
) -> f32 {
  table_cell_content_height_with_mode(
    cell,
    cell_width,
    setup,
    TableCellMeasureMode::LastRenderedFollow,
    true,
    12,
    text_metrics,
  )
}

fn table_cell_content_height_with_mode(
  cell: &TableCell,
  cell_width: f32,
  setup: PageSetup,
  mode: TableCellMeasureMode,
  script_sensitive_line_height: bool,
  compatibility_mode: u16,
  text_metrics: &mut TextMetrics,
) -> f32 {
  let content_width =
    (cell_width - cell.margins.left_pt - cell.margins.right_pt).max(DEFAULT_FONT_SIZE_PT);
  let flow = FlowContext {
    setup,
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
    layout_cell_print_bounds: Some(FrameBounds {
      x_pt: cell.margins.left_pt,
      y_pt: cell.margins.top_pt,
      width_pt: content_width,
      height_pt: 0.0,
    }),
    default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
    compatibility_mode,
    justify_lines_with_shrinking: compatibility_mode >= 15,
    split_page_break_and_paragraph_mark: false,
    repeating_slots: RepeatingSlotState::default(),
    text_segmentation: TextSegmentation::TableCell,
    paragraph_spacing_context: ParagraphSpacingContext::Normal,
    preserve_horizontal_on_advance: false,
    script_sensitive_line_height: true,
  };
  let flow = FlowContext {
    script_sensitive_line_height: true,
    ..flow
  };
  let content = table_cell_blocks_content_height(&cell.blocks, flow, mode, text_metrics).max(
    table_cell_first_inline_text_height(cell, script_sensitive_line_height, text_metrics),
  );
  cell.margins.top_pt + content + cell.margins.bottom_pt
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TableCellMeasureMode {
  WholeCell,
  LastRenderedFollow,
}

fn table_cell_blocks_content_height(
  blocks: &[Block],
  flow: FlowContext,
  mode: TableCellMeasureMode,
  text_metrics: &mut TextMetrics,
) -> f32 {
  blocks
    .iter()
    .enumerate()
    .map(|(index, block)| {
      let previous = index
        .checked_sub(1)
        .and_then(|previous| blocks.get(previous));
      let next = blocks.get(index + 1);
      match block {
        Block::Paragraph(paragraph) => table_cell_paragraph_height(
          previous,
          paragraph,
          next,
          flow,
          text_metrics,
          index + 1 == blocks.len(),
          mode == TableCellMeasureMode::LastRenderedFollow && index == 0,
        ),
        Block::Table(table) => estimated_table_height(table, flow, text_metrics),
        Block::Frame(frame) => estimated_frame_height(frame, flow, text_metrics),
      }
    })
    .sum()
}

fn table_cell_paragraph_height(
  previous: Option<&Block>,
  paragraph: &crate::docx::Paragraph,
  next: Option<&Block>,
  flow: FlowContext,
  text_metrics: &mut TextMetrics,
  last_in_cell: bool,
  split_follow_text_frame: bool,
) -> f32 {
  // and sw/source/core/layout/tabfrm.cxx table row sizing. A paragraph inside a
  // table cell is measured as a frame in sequence: upper space is collapsed
  // against the previous flow frame, lower space is owned by the current frame,
  // and the final paragraph contributes Word-compatible line-spacing slack at
  // the cell border.
  //
  // lcl_CalcHeightOfFirstContentLine(): a split text frame follow that has a
  // master is not measured with its master's upper/lower spacing again.
  let content = estimated_paragraph_content_height(paragraph, flow, text_metrics);
  let min_height = paragraph_line_height_for_setup(
    paragraph,
    &paragraph_base_line_style(paragraph),
    flow.setup,
    flow.text_segmentation,
    text_metrics,
  );
  let upper = if split_follow_text_frame {
    0.0
  } else {
    paragraph_spacing_before(previous, paragraph, flow)
  };
  let mut lower = if split_follow_text_frame {
    0.0
  } else {
    paragraph_spacing_after(paragraph, next)
  };
  if last_in_cell && !split_follow_text_frame {
    lower += paragraph_add_lower_space_as_last_in_table_cell(paragraph, flow);
  }
  (upper + content + lower).max(min_height)
}

fn paragraph_add_lower_space_as_last_in_table_cell(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
) -> f32 {
  // SwFlowFrame::CalcAddLowerSpaceAsLastInTableCell(): Writer's Word
  // compatibility setting ADD_PARA_SPACING_TO_TABLE_CELLS adds the paragraph's
  // lower space again for the last flow frame in a table cell. Writer enables
  // that for legacy WW8 import; DOCX defaults to compatibilityMode 12 and does
  // not enable the old extra table-cell paragraph spacing.
  if flow.compatibility_mode >= 12 {
    return 0.0;
  }
  paragraph_lower_space(paragraph) + paragraph_line_spacing_excess(paragraph)
}

fn paragraph_line_spacing_excess(paragraph: &crate::docx::Paragraph) -> f32 {
  if !matches!(paragraph.format.line_height_rule, LineHeightRule::Auto) {
    return 0.0;
  }
  let Some(multiple) = paragraph.format.line_height_pt else {
    return 0.0;
  };
  if multiple <= 1.0 {
    return 0.0;
  }
  // SwBorderAttrs::CalcLineSpacing_ adds 115% of the proportional line spacing
  // excess when Word-compatible layout asks for paragraph line spacing.
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
  if effective_layout_in_cell(placement, flow) {
    let cell_bounds = flow.layout_cell_bounds.unwrap_or(FrameBounds {
      x_pt: flow.content_left_pt,
      y_pt: current_y,
      width_pt: flow.content_width,
      height_pt: 0.0,
    });
    let cell_print_bounds = flow.layout_cell_print_bounds.unwrap_or(FrameBounds {
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
      VerticalImageReference::Page => (cell_print_bounds.y_pt, cell_print_bounds.height_pt),
      VerticalImageReference::Margin
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

fn floating_anchor_reference_y(
  placement: FloatingImagePlacement,
  paragraph_top: f32,
  line_top: f32,
) -> f32 {
  match placement.vertical_relative_to {
    // paragraph frame, not to the line where the anchor run is encountered.
    crate::docx::VerticalImageReference::Paragraph => paragraph_top,
    _ => line_top,
  }
}

fn floating_anchor_line_top(
  flow: FlowContext,
  paragraph: &crate::docx::Paragraph,
  line_y: f32,
  line_height: f32,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if flow.text_segmentation != TextSegmentation::TableCell {
    return line_y;
  }
  // Table-cell text items carry a resolved baseline as their layout origin so
  // the PDF table owner can paint them without applying a second baseline
  // offset. Floating anchors still use the SwTextFrame/SwLineLayout top:
  // testTdf162551 requires a compat15 line-relative image to start at that
  // paragraph line top, not at the stored text baseline.
  line_y
    - text_metrics.baseline_offset_in_line_with_windows_metrics(
      &paragraph_base_line_style(paragraph),
      line_height,
    )
}

fn floating_frame_position(
  placement: FloatingFramePlacement,
  flow: FlowContext,
  current_y: f32,
  frame_width: f32,
  frame_height: f32,
  include_wrap_margins: bool,
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
  let margin_left = if include_wrap_margins {
    placement.margin_left_pt
  } else {
    0.0
  };
  let margin_top = if include_wrap_margins {
    placement.margin_top_pt
  } else {
    0.0
  };
  (
    base_x
      + aligned_frame_horizontal_offset(
        placement.horizontal_alignment,
        reference_width,
        frame_width,
      )
      + placement.horizontal_offset_pt
      + margin_left,
    base_y
      + aligned_frame_vertical_offset(placement.vertical_alignment, reference_height, frame_height)
      + placement.vertical_offset_pt
      + margin_top,
  )
}

fn floating_table_position(
  placement: FloatingFramePlacement,
  flow: FlowContext,
  current_y: f32,
  table_width: f32,
) -> (f32, f32) {
  // ECMA-376 Part 1 §17.4.57 defines tblpX/tblpY as the position of the
  // floating table's top-left corner. The *FromText values are minimum wrap
  // distances to surrounding text, not additions to that absolute position.
  // LibreOffice likewise maps them to frame margins separately from
  // HoriOrientPosition/VertOrientPosition in TablePositionHandler.
  floating_frame_position(placement, flow, current_y, table_width, 0.0, false)
}

fn paragraph_frame_position(
  placement: FloatingFramePlacement,
  flow: FlowContext,
  current_y: f32,
  frame_width: f32,
  frame_height: f32,
) -> (f32, f32) {
  // ECMA-376 Part 1 §17.3.1.11 defines hSpace/vSpace as the minimum
  // distance to non-frame text that wraps around the frame. LibreOffice's
  // DomainMapper_Impl::MakeFrameProperties maps them to frame margins while
  // keeping x/y in HoriOrientPosition/VertOrientPosition, so they must not
  // shift the frame's specified position.
  floating_frame_position(placement, flow, current_y, frame_width, frame_height, false)
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

fn effective_layout_in_cell(placement: FloatingImagePlacement, flow: FlowContext) -> bool {
  // compat15 ignores layoutInCell="0" and always lays out shapes in the cell.
  flow.text_segmentation == TextSegmentation::TableCell
    && (placement.layout_in_cell || flow.compatibility_mode >= 15)
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

fn floating_shape_intersects_page(
  shape: &InlineShape,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  flow: FlowContext,
) -> bool {
  let stroke_padding = shape
    .stroke
    .map(|stroke| stroke.width_pt / 2.0)
    .unwrap_or_default();
  let padding = stroke_padding
    .max(shape.effect_left_pt)
    .max(shape.effect_top_pt)
    .max(shape.effect_right_pt)
    .max(shape.effect_bottom_pt)
    .max(LAYOUT_EPSILON_PT);
  x_pt + width_pt + padding >= 0.0
    && x_pt - padding <= flow.setup.width_pt
    && y_pt + height_pt + padding >= 0.0
    && y_pt - padding <= flow.setup.height_pt
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

struct ParagraphLayoutTarget<'a> {
  current: &'a mut Page,
  pages: &'a mut Vec<Page>,
  anchor_pages: Option<&'a mut Vec<AnchorPage>>,
  text_metrics: &'a mut TextMetrics,
}

fn layout_paragraph(
  paragraph: &crate::docx::Paragraph,
  flow: FlowContext,
  target: ParagraphLayoutTarget<'_>,
  y: f32,
  spacing_after_pt: f32,
) -> (FlowContext, f32) {
  let (indent_left_pt, indent_right_pt, _) =
    resolved_paragraph_indents(paragraph, target.text_metrics);
  let flow = FlowContext {
    content_width: paragraph_content_width(flow.content_width, indent_left_pt, indent_right_pt),
    ..flow
  };
  TextFrameLayout::new(paragraph, flow, spacing_after_pt, target.text_metrics).format(
    target.current,
    target.pages,
    target.anchor_pages,
    target.text_metrics,
    y,
  )
}

fn record_anchor_page(
  anchor_pages: &mut Vec<AnchorPage>,
  name: &str,
  current: &Page,
  page_index: usize,
) {
  if name.is_empty() {
    return;
  }
  let anchor = AnchorPage {
    name: name.to_string(),
    page_index,
    section_index: current.section_index,
    section_page_index: current.section_page_index,
    physical_page_number: page_index + 1,
    virtual_page_number: virtual_page_number(current.setup, current.section_page_index, page_index),
  };
  if let Some(existing) = anchor_pages.iter_mut().find(|anchor| anchor.name == name) {
    *existing = anchor;
  } else {
    anchor_pages.push(anchor);
  }
}

fn virtual_page_number(setup: PageSetup, section_page_index: usize, page_index: usize) -> usize {
  setup
    .page_number_start
    .map(|start| start.saturating_add(section_page_index as i32).max(1) as usize)
    .unwrap_or(page_index + 1)
}

#[derive(Clone, Copy, Debug)]
struct TextFrame {
  default_line_left: f32,
  first_line_left: f32,
  default_line_right: f32,
  paragraph_left: f32,
  base_line_height: f32,
  line_height_rule: LineHeightRule,
  grid_auto_line_spacing_multiplier: Option<f32>,
  script_sensitive_line_height: bool,
}

impl TextFrame {
  fn new(
    paragraph: &crate::docx::Paragraph,
    flow: FlowContext,
    text_metrics: &mut TextMetrics,
  ) -> Self {
    let (indent_left_pt, _, first_line_indent_pt) =
      resolved_paragraph_indents(paragraph, text_metrics);
    let (default_line_left, first_line_left, default_line_right) = paragraph_line_bounds(
      flow.content_left_pt,
      flow.content_width,
      indent_left_pt,
      first_line_indent_pt,
    );
    let base_line_style = paragraph_base_line_style(paragraph);
    Self {
      default_line_left,
      first_line_left,
      default_line_right,
      paragraph_left: default_line_left.min(first_line_left),
      base_line_height: paragraph_line_height_for_setup(
        paragraph,
        &base_line_style,
        flow.setup,
        flow.text_segmentation,
        text_metrics,
      ),
      line_height_rule: paragraph.format.line_height_rule,
      grid_auto_line_spacing_multiplier: grid_auto_line_spacing_multiplier(
        paragraph,
        flow.setup,
        flow.text_segmentation,
      ),
      script_sensitive_line_height: flow.script_sensitive_line_height,
    }
  }
}

fn resolved_paragraph_indents(
  paragraph: &crate::docx::Paragraph,
  text_metrics: &mut TextMetrics,
) -> (f32, f32, f32) {
  let format = &paragraph.format;
  let needs_character_unit = [
    format.indent_left_character_units,
    format.indent_right_character_units,
    format.first_line_indent_character_units,
  ]
  .into_iter()
  .flatten()
  .any(|value| value != 0.0);
  let character_unit_pt = needs_character_unit.then(|| {
    // ECMA-376 Part 1 §17.3.1.12 defines these values in hundredths of a
    // character unit. Writer represents that unit as FONT_CJK_ADVANCE, so use
    // the resolved East Asian font's ideographic advance rather than the Latin
    // average character width. Measuring U+6C34 follows the CSS `ic` reference
    // glyph and routes shaping through the run's East Asian font slot.
    let style = paragraph_base_line_style(paragraph);
    let measured = text_metrics.measure_text("\u{6c34}", &style);
    if measured > f32::EPSILON {
      measured
    } else {
      effective_font_size_pt(&style, None)
    }
  });
  let resolve = |physical_pt: f32, character_units: Option<f32>| {
    character_units
      .filter(|value| *value != 0.0)
      .zip(character_unit_pt)
      .map_or(physical_pt, |(units, unit_pt)| units * unit_pt)
  };
  (
    resolve(format.indent_left_pt, format.indent_left_character_units),
    resolve(format.indent_right_pt, format.indent_right_character_units),
    resolve(
      format.first_line_indent_pt,
      format.first_line_indent_character_units,
    ),
  )
}

fn paragraph_line_bounds(
  content_left_pt: f32,
  content_width: f32,
  indent_left_pt: f32,
  first_line_indent_pt: f32,
) -> (f32, f32, f32) {
  // ECMA-376 Part 1 §17.3.1.12 defines start and end as independent
  // distances from the leading and trailing text margins. Moving the leading
  // edge must therefore reduce the line width, not translate the trailing
  // edge. Writer's SwTextMargin::CtorInitTextMargin likewise derives mnRight
  // from the frame print-area end and only subtracts the right margin.
  let default_line_left = content_left_pt + indent_left_pt;
  let first_line_left = default_line_left + first_line_indent_pt;
  // `layout_paragraph` has already removed both paragraph indents from
  // `content_width`. Add the leading indent back only as an origin offset;
  // the trailing indent remains encoded in the reduced width.
  let default_line_right = content_left_pt + content_width + indent_left_pt;
  (default_line_left, first_line_left, default_line_right)
}

fn paragraph_content_width(content_width: f32, indent_left_pt: f32, indent_right_pt: f32) -> f32 {
  (content_width - indent_left_pt - indent_right_pt).max(DEFAULT_FONT_SIZE_PT)
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
  RebalanceWidow { page_index: usize },
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
    if self.page_follows.is_empty() {
      return TextSplitDecision::NoSplit;
    }
    if keep_lines {
      return TextSplitDecision::Rejected;
    }

    let mut previous_start = InlineCursor::default();
    for (index, follow) in self.page_follows.iter().enumerate() {
      let next_start = self
        .page_follows
        .get(index + 1)
        .map(|next| next.start)
        .unwrap_or(InlineCursor {
          inline_index: usize::MAX,
          text_offset: usize::MAX,
        });
      let before = self
        .line_fragments
        .iter()
        .filter(|line| {
          line.start >= previous_start && line.end <= follow.start && line.end > line.start
        })
        .count();
      let after = self
        .line_fragments
        .iter()
        .filter(|line| {
          line.start >= follow.start && line.start < next_start && line.end > line.start
        })
        .count();

      if before == 0 || after == 0 {
        return TextSplitDecision::Forced;
      }
      if before < orphan_lines {
        return TextSplitDecision::Rejected;
      }
      if after < widow_lines {
        // WidowsAndOrphans::FindWidows() asks the master of this follow to
        // give lines to the follow. That is a local master/follow adjustment,
        // not a request to move the whole paragraph from the first page.
        return TextSplitDecision::RebalanceWidow {
          page_index: follow.page_index.saturating_sub(1),
        };
      }
      previous_start = follow.start;
    }
    TextSplitDecision::Allowed
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
  text_metrics: &'a mut TextMetrics,
  wrap_exclusions: &'a mut Vec<WrapExclusion>,
  state: &'a mut TextFrameState,
  active: ActiveTextFrame,
  line_left: f32,
  line_right: f32,
  justify: bool,
  line_item_start_index: &'a mut usize,
  line_has_form_widget: &'a mut bool,
}

struct InlineObjectAdvance<'a> {
  current: &'a mut Page,
  pages: &'a mut Vec<Page>,
  text_metrics: &'a mut TextMetrics,
  wrap_exclusions: &'a mut Vec<WrapExclusion>,
  state: &'a mut TextFrameState,
  flow: FlowContext,
  y: f32,
  object_line_height: f32,
}

struct TextFrameLayout<'a> {
  paragraph: &'a crate::docx::Paragraph,
  flow: FlowContext,
  frame: TextFrame,
  spacing_after_pt: f32,
  widow_rebalance_page_index: Option<usize>,
}

fn can_defer_page_break_for_following_floating_anchor(
  paragraph: &crate::docx::Paragraph,
  next_inline_index: usize,
  has_content_before_break: bool,
  split_page_break_and_paragraph_mark: bool,
) -> bool {
  if split_page_break_and_paragraph_mark {
    return false;
  }
  let Some(anchor_index) = paragraph
    .inlines
    .iter()
    .enumerate()
    .skip(next_inline_index)
    .find_map(|(index, inline)| match inline {
      InlineItem::Image(image)
        if matches!(image.placement, crate::docx::ImagePlacement::Floating(_)) =>
      {
        Some(index)
      }
      InlineItem::Shape(shape)
        if matches!(shape.placement, crate::docx::ImagePlacement::Floating(_)) =>
      {
        Some(index)
      }
      InlineItem::LastRenderedPageBreak => None,
      InlineItem::Text(run) if run.text.trim().is_empty() => None,
      InlineItem::BookmarkStart(_)
      | InlineItem::FormWidgetStart(_)
      | InlineItem::FormWidgetEnd(_) => None,
      InlineItem::Text(_)
      | InlineItem::PageBreak
      | InlineItem::ColumnBreak
      | InlineItem::Image(_)
      | InlineItem::Shape(_) => Some(usize::MAX),
    })
  else {
    return false;
  };
  if anchor_index == usize::MAX {
    return false;
  }
  has_content_before_break
    || paragraph
      .inlines
      .iter()
      .skip(anchor_index + 1)
      .all(|inline| match inline {
        InlineItem::LastRenderedPageBreak => true,
        InlineItem::Text(run) => run.text.trim().is_empty(),
        InlineItem::BookmarkStart(_)
        | InlineItem::FormWidgetStart(_)
        | InlineItem::FormWidgetEnd(_) => true,
        InlineItem::PageBreak
        | InlineItem::ColumnBreak
        | InlineItem::Image(_)
        | InlineItem::Shape(_) => false,
      })
}

impl<'a> TextFrameLayout<'a> {
  fn new(
    paragraph: &'a crate::docx::Paragraph,
    flow: FlowContext,
    spacing_after_pt: f32,
    text_metrics: &mut TextMetrics,
  ) -> Self {
    Self {
      paragraph,
      flow,
      frame: TextFrame::new(paragraph, flow, text_metrics),
      spacing_after_pt,
      widow_rebalance_page_index: None,
    }
  }

  fn with_widow_rebalance_page(mut self, page_index: usize) -> Self {
    self.widow_rebalance_page_index = Some(page_index);
    self
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
    if self.paragraph.format.bidi {
      reorder_bidi_line_items(
        &mut advance.current.items,
        *advance.line_item_start_index,
        y,
        advance.text_metrics,
      );
    }
    finalize_cjk_punctuation_compression(
      &mut advance.current.items,
      *advance.line_item_start_index,
      y,
      advance.line_right,
      advance.text_metrics,
    );
    if advance.justify {
      justify_line_items(
        &mut advance.current.items,
        *advance.line_item_start_index,
        y,
        advance.line_left,
        advance.line_right,
        advance.text_metrics,
      );
    }
    align_text_baseline_to_inline_object(
      &mut advance.current.items,
      *advance.line_item_start_index,
      y,
      advance.text_metrics,
    );
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
        bounds: (*advance.line_item_start_index == advance.current.items.len()).then_some(
          FrameBounds {
            x_pt: advance.line_left,
            y_pt: y,
            width_pt: (advance.line_right - advance.line_left).max(0.0),
            height_pt: *line_height,
          },
        ),
      },
      advance.text_metrics,
    );
    let line_index = advance.state.line_fragments.len();
    let real_height = line_real_height(
      self.paragraph,
      *line_height,
      line_index,
      advance.active.frame.grid_auto_line_spacing_multiplier,
      *advance.line_has_form_widget,
    );
    advance.state.finish_line(y, real_height);
    let mut next_y = y + real_height;
    *advance.line_has_form_widget = false;
    *line_height = advance.active.frame.base_line_height;
    let mut next_flow = advance.active.flow;
    let mut next_frame = advance.active.frame;
    let content_bottom = if self.widow_rebalance_page_index == Some(advance.pages.len()) {
      advance.active.flow.content_bottom - *line_height
    } else {
      advance.active.flow.content_bottom
    };
    if next_y + *line_height > content_bottom
      && page_has_body_region_items(advance.current, advance.active.flow)
    {
      (next_flow, next_y) =
        advance_text_frame_flow(advance.active.flow, advance.current, advance.pages);
      next_frame = TextFrame::new(self.paragraph, next_flow, advance.text_metrics);
      *line_height = next_frame.base_line_height;
      advance.state.note_page_follow(advance.pages.len(), next_y);
      reset_wrap_exclusions_for_y(advance.current, next_y, advance.wrap_exclusions);
    }
    next_y = dodge_text_wrap_exclusions(
      next_y,
      *line_height,
      next_frame.default_line_left,
      next_frame.default_line_right,
      advance.wrap_exclusions,
    );
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
    text_metrics: &mut TextMetrics,
    wrap_exclusions: &mut Vec<WrapExclusion>,
  ) -> (FlowContext, TextFrame, f32, f32, f32, f32) {
    let (next_flow, y) = force_page_break(flow, current, pages);
    let next_frame = TextFrame::new(self.paragraph, next_flow, text_metrics);
    reset_wrap_exclusions_for_y(current, y, wrap_exclusions);
    let y = dodge_text_wrap_exclusions(
      y,
      next_frame.base_line_height,
      next_frame.default_line_left,
      next_frame.default_line_right,
      wrap_exclusions,
    );
    (
      next_flow,
      next_frame,
      y,
      next_frame.first_line_left,
      next_frame.default_line_right,
      next_frame.base_line_height,
    )
  }

  fn advance_for_inline_object(
    &self,
    advance: InlineObjectAdvance<'_>,
  ) -> Option<(FlowContext, TextFrame, f32, f32, f32, f32)> {
    if !inline_object_requires_flow_advance(
      advance.y,
      advance.object_line_height,
      advance.flow.content_bottom,
      page_has_body_region_items(advance.current, advance.flow),
    ) {
      return None;
    }
    let (next_flow, y) = advance_text_frame_flow(advance.flow, advance.current, advance.pages);
    let next_frame = TextFrame::new(self.paragraph, next_flow, advance.text_metrics);
    advance.state.note_page_follow(advance.pages.len(), y);
    reset_wrap_exclusions_for_y(advance.current, y, advance.wrap_exclusions);
    let y = dodge_text_wrap_exclusions(
      y,
      next_frame.base_line_height,
      next_frame.default_line_left,
      next_frame.default_line_right,
      advance.wrap_exclusions,
    );
    Some((
      next_flow,
      next_frame,
      y,
      next_frame.first_line_left,
      next_frame.default_line_right,
      next_frame.base_line_height,
    ))
  }

  fn apply_column_break(
    &self,
    flow: FlowContext,
    current: &mut Page,
    pages: &mut Vec<Page>,
    text_metrics: &mut TextMetrics,
    wrap_exclusions: &mut Vec<WrapExclusion>,
  ) -> (FlowContext, TextFrame, f32, f32, f32, f32, bool) {
    let (next_flow, y) = advance_text_frame_flow(flow, current, pages);
    reset_wrap_exclusions_for_y(current, y, wrap_exclusions);
    let next_frame = TextFrame::new(self.paragraph, next_flow, text_metrics);
    let y = dodge_text_wrap_exclusions(
      y,
      next_frame.base_line_height,
      next_frame.default_line_left,
      next_frame.default_line_right,
      wrap_exclusions,
    );
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

  fn format(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    anchor_pages: Option<&mut Vec<AnchorPage>>,
    text_metrics: &mut TextMetrics,
    y: f32,
  ) -> (FlowContext, f32) {
    self.format_with_reflow(current, pages, anchor_pages, text_metrics, y, true)
  }

  fn format_with_reflow(
    &self,
    current: &mut Page,
    pages: &mut Vec<Page>,
    mut anchor_pages: Option<&mut Vec<AnchorPage>>,
    text_metrics: &mut TextMetrics,
    mut y: f32,
    allow_reflow: bool,
  ) -> (FlowContext, f32) {
    let paragraph = self.paragraph;
    let mut flow = self.flow;
    let mut text_frame = self.frame;
    let start_item_index = current.items.len();
    let start_pages_len = pages.len();
    let start_current = page_checkpoint(current);
    // WidowsAndOrphans::WouldFit() deliberately skips widow prediction when
    // the master and follow can have different line widths. A floating-object
    // exclusion has the same property: reserving one nominal line at the
    // bottom can rewrap the entire follow around a different fly contour.
    let has_pending_floating_table_follows = has_pending_floating_table_follows(current, pages);
    let can_rebalance_widow_by_reserving_line =
      current.wrap_exclusions.is_empty() && !has_pending_floating_table_follows;
    let start_page_snapshot = (!current.wrap_exclusions.is_empty()
      || has_pending_floating_table_follows)
      .then(|| current.clone());
    let start_pending_snapshot =
      has_pending_floating_table_follows.then(|| TextFramePendingSnapshot {
        current: current.pending_floating_table_follows.clone(),
        pages: pages
          .iter()
          .map(|page| page.pending_floating_table_follows.clone())
          .collect(),
      });
    let start_anchor_pages_len = anchor_pages.as_ref().map(|anchors| anchors.len());
    let start_flow = flow;
    let paragraph_top = y;
    // Paragraph-relative drawings use the table paragraph frame origin. Only
    // line-relative drawings need the table-cell baseline translated back to
    // a SwLineLayout top (see `floating_anchor_line_top`). In particular,
    // bottom-aligned table paragraphs keep their aligned frame origin here.
    let paragraph_anchor_top = y;
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
    let mut behind_text_floating_only = false;
    let mut pending_text_page_break = false;
    let mut ended_with_explicit_page_break = false;
    let mut pending_tab: Option<PendingAlignedTab> = None;
    let mut text_state = TextFrameState::new();
    let line = LineFrame::first(text_frame, y, paragraph.list_label.is_some());
    y = line.y_pt;
    let vml_anchor_line_height =
      libreoffice_empty_paragraph_first_line_height(&paragraph_base_line_style(paragraph));
    let mut base_line_height = text_frame.base_line_height;
    let mut line_height = line.height_pt;
    y = dodge_text_wrap_exclusions(
      y,
      line_height,
      text_frame.default_line_left,
      text_frame.default_line_right,
      &wrap_exclusions,
    );
    // Writer's Word-compatibility `ADD_VERTICAL_FLY_OFFSETS` behavior treats
    // the current paragraph's own upper margin as part of its first-line
    // collision strip. When a side-wrapped fly crosses that margin and also
    // reaches the printable line, move the whole paragraph below the fly.
    //
    // This is deliberately separate from ordinary side wrapping: if the fly
    // ends inside the upper margin, the printable line does not intersect it
    // and no width is reserved (tdf#124600). The rule applies only while
    // placing the first line; subsequent lines continue to wrap normally.
    y = shift_first_line_below_upper_margin_exclusions(
      y,
      paragraph.format.spacing_before_pt,
      text_frame.default_line_left,
      text_frame.default_line_right,
      &wrap_exclusions,
    );
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
        let minimum_size = effective_font_size_pt(&base_line_style, None);
        if list_label_style.complex_script == Some(true)
          || list_label_style.right_to_left == Some(true)
        {
          list_label_style.complex_font_size_pt = Some(
            list_label_style
              .complex_font_size_pt
              .unwrap_or(list_label_style.font_size_pt)
              .max(minimum_size),
          );
        } else {
          list_label_style.font_size_pt = list_label_style.font_size_pt.max(minimum_size);
        }
      }
      let (visible_label, label_follow) = if blank_list_label {
        (label.as_str(), None)
      } else if let Some(label) = label.strip_suffix('\t') {
        (label, Some('\t'))
      } else if let Some(label) = label.strip_suffix(' ') {
        (label, Some(' '))
      } else {
        (label.as_str(), None)
      };
      let label_width = if blank_list_label {
        0.0
      } else {
        text_metrics.measure_text(visible_label, &list_label_style)
      };
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
      current.items.push(PageItem::Text(Box::new(TextItem {
        x_pt: if blank_list_label {
          default_line_left
        } else {
          first_line_left
        },
        y_pt: y,
        line_height_pt: line_height,
        text: visible_label.to_string(),
        style: list_label_style.clone(),
        rotation_center_pt: None,
        hyperlink_url: paragraph.list_label_hyperlink_url.clone(),
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
        form_widget_id: None,
        paragraph_bidi: false,
        word_spacing_pt: 0.0,
        decoration_span_start_x_pt: None,
        pdf_text_segmentation: PdfTextSegmentation::Line,
      })));
      x = default_line_left;
      if blank_list_label {
        if let Some(tab_stop_pt) = paragraph.list_label_tab_stop_pt {
          x = x.max(flow.content_left_pt + tab_stop_pt);
        }
      } else {
        let label_end = first_line_left + label_width;
        let label_overflows_reserved_hanging_space = label_end > default_line_left;
        let explicit_list_tab = (paragraph.format.list_label_uses_explicit_tab_stop
          && label_follow == Some('\t'))
        .then_some(paragraph.list_label_tab_stop_pt)
        .flatten()
        .map(|stop| flow.content_left_pt + stop)
        .filter(|stop| *stop > label_end + LAYOUT_EPSILON_PT);
        x = if let Some(tab_stop) = explicit_list_tab {
          // ECMA-376 Part 1 §17.9.29 makes tab the default numbering
          // suffix. A w:lvl/w:pPr number tab is the first stop for that
          // suffix even when a paragraph style overrides the hanging indent.
          tab_stop.max(default_line_left)
        } else if paragraph.format.list_label_width_aware_tab {
          match label_follow {
            // The imported list-label fallback is only used to place an empty
            // label. A visible label followed by a tab advances from its actual
            // painted end to the next document default tab stop; the Office
            // golden for long text numbering demonstrates the resulting
            // progression, and SwNumberPortion::Format likewise sizes the
            // numbering portion from its rendered width.
            Some('\t') => {
              next_tab_stop(label_end, first_line_left, &[], flow.default_tab_stop_pt).x_pt
            }
            Some(' ') => label_end + text_metrics.measure_text(" ", &paragraph.list_label_style),
            _ => label_end,
          }
          .max(default_line_left)
        } else if label_overflows_reserved_hanging_space {
          // ECMA-376 Part 1 §17.9.29 makes the default numbering suffix a tab.
          // The hanging indent is its first implicit stop; when a long label
          // (for example "Article 1.") crosses that stop, Word advances to the
          // next document tab instead of painting paragraph text over the
          // label. This is observable when §17.3.2.36 style separators merge
          // the numbered heading with the following paragraph.
          match label_follow {
            Some('\t') => {
              next_tab_stop(
                label_end,
                flow.content_left_pt,
                &paragraph.format.tab_stops,
                flow.default_tab_stop_pt,
              )
              .x_pt
            }
            Some(' ') => label_end + text_metrics.measure_text(" ", &paragraph.list_label_style),
            _ => label_end,
          }
        } else {
          default_line_left
        };
      }
    }
    let mut line_item_start_index = current.items.len();
    let justification = paragraph.format.justification;
    // The numbering portion is a fixed margin portion, but it does not turn
    // off paragraph adjustment. SwTextAdjuster::CalcNewBlock() still expands
    // the body text of wrapped numbered lines; Office golden output likewise
    // reaches the right paragraph margin for w:jc="both" list paragraphs.
    let justify_wrapped_lines =
      justification.is_block() || paragraph.format.alignment == ParagraphAlignment::Justify;
    let shrink_justified_lines = justify_wrapped_lines
      && flow.justify_lines_with_shrinking
      && justification.can_shrink_word_spacing();
    let mut line_used_shrink_fit = false;
    let mut line_used_punctuation_fit = false;
    let mut line_has_tab = false;
    let mut active_form_widget_ids = Vec::new();
    let mut line_has_form_widget = false;
    let mut tab_over_margin_active = false;

    for (inline_index, item) in paragraph.inlines.iter().enumerate() {
      match item {
        InlineItem::BookmarkStart(name) => {
          text_state.set_position(InlineCursor::after_inline(inline_index));
          pending_tab = None;
          if let Some(anchors) = anchor_pages.as_deref_mut() {
            record_anchor_page(anchors, name, current, pages.len());
          }
        }
        InlineItem::Text(run) => {
          if !run.text.is_empty() {
            ended_with_explicit_page_break = false;
          }
          if pending_text_page_break {
            (flow, text_frame, y, line_left, line_right, line_height) =
              self.force_text_page_break(flow, current, pages, text_metrics, &mut wrap_exclusions);
            default_line_right = text_frame.default_line_right;
            paragraph_left = text_frame.paragraph_left;
            base_line_height = text_frame.base_line_height;
            x = line_left;
            line_item_start_index = current.items.len();
            line_has_form_widget = false;
            pending_text_page_break = false;
            line_used_shrink_fit = false;
            line_used_punctuation_fit = false;
            line_has_tab = false;
          }
          let mut chunk = String::new();
          let mut chunk_x = x;
          let (style_ref_keys, style_ref_text) = if run.style_ref_keys.is_empty() {
            (
              paragraph.style_ref_keys.as_slice(),
              paragraph.style_ref_text.as_ref(),
            )
          } else {
            (run.style_ref_keys.as_slice(), run.style_ref_text.as_ref())
          };
          let style_ref_numbering_text = if run.style_ref_keys.is_empty() {
            paragraph.style_ref_numbering_text.as_ref()
          } else {
            run.style_ref_numbering_text.as_ref()
          };
          let meta = TextChunkMeta {
            hyperlink_url: run.hyperlink_url.as_deref(),
            dynamic_field: run.dynamic_field.as_ref(),
            style_ref_keys,
            style_ref_text,
            style_ref_numbering_text,
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
          for (segment_index, segment) in segments.iter().enumerate() {
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
                &meta,
              );
              apply_pending_aligned_tab(
                current,
                &mut pending_tab,
                text_metrics,
                y,
                line_left,
                line_right,
              );
              (flow, text_frame, y, line_left, line_right) = self.advance_line(
                TextLineAdvance {
                  current,
                  pages,
                  text_metrics: &mut *text_metrics,
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
              line_used_shrink_fit = false;
              line_used_punctuation_fit = false;
              line_has_tab = false;
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
                &meta,
              );
              let tab_stop = next_tab_stop(
                x,
                line_left,
                &paragraph.format.tab_stops,
                flow.default_tab_stop_pt,
              );
              let leader_start_x = x;
              x = tab_stop.x_pt;
              if x > line_right {
                tab_over_margin_active = true;
              }
              line_used_shrink_fit = false;
              line_used_punctuation_fit = false;
              line_has_tab = true;
              chunk_x = x;
              if matches!(
                tab_stop.alignment,
                TabStopAlignment::Center | TabStopAlignment::Right
              ) {
                pending_tab = Some(PendingAlignedTab {
                  stop: tab_stop,
                  leader_start_x_pt: leader_start_x,
                  y_pt: y,
                  line_height_pt: line_height,
                  style: run.style.clone(),
                  item_start_index: current.items.len(),
                });
              } else {
                push_tab_leader(
                  current,
                  TabLeaderPlacement {
                    insert_index: None,
                    leader: tab_stop.leader,
                    start_x: leader_start_x,
                    end_x: tab_stop.x_pt,
                    y,
                    line_height,
                  },
                  &run.style,
                  text_metrics,
                );
                pending_tab = None;
              }
              emitted = true;
              continue;
            }
            if segment.text.starts_with(PRESERVED_WORD_TEXT_TAB) {
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
                &meta,
              );
              let relative_x = (x - line_left).max(0.0);
              x = line_left
                + ((relative_x / PRESERVED_WORD_TEXT_TAB_STOP_PT).floor() + 1.0)
                  * PRESERVED_WORD_TEXT_TAB_STOP_PT;
              if x > line_right {
                tab_over_margin_active = true;
              }
              line_used_shrink_fit = false;
              line_used_punctuation_fit = false;
              line_has_tab = true;
              chunk_x = x;
              pending_tab = None;
              emitted = true;
              continue;
            }

            let width = text_metrics.measure_text(&segment.text, &run.style);
            let fit_width = line_fit_width(&segment.text, &run.style, text_metrics);
            let continuation_fit_width = if flow.text_segmentation != TextSegmentation::DrawingLayer
              && segment_index + 1 == segments.len()
            {
              cross_run_unbreakable_continuation_width(
                &paragraph.inlines,
                inline_index,
                &segment.text,
                text_metrics,
              )
            } else {
              0.0
            };
            let fit_width_with_continuation = fit_width + continuation_fit_width;
            let line_capacity = (line_right - line_left).max(DEFAULT_FONT_SIZE_PT);
            let whitespace = segment.text.chars().all(char::is_whitespace);

            let only_char = segment
              .text
              .chars()
              .next()
              .filter(|_| segment.text.chars().count() == 1);
            let natural_overflow = only_char.is_some_and(cjk_line_character)
              && line_natural_overflows_with_cjk_compression(
                CjkLineFit {
                  x,
                  line_right,
                  items: &current.items,
                  item_start: line_item_start_index,
                  chunk: &chunk,
                  text: &segment.text,
                  style: &run.style,
                },
                text_metrics,
              );
            let blocks_second_extra_character = natural_overflow
              && line_used_punctuation_fit
              && only_char.is_none_or(|ch| !cjk_cannot_start_line(ch));
            let overflows_line = text_overflows_line(x, fit_width_with_continuation, line_right)
              || blocks_second_extra_character;
            let fits_with_shrink = overflows_line
              && shrink_justified_lines
              && !line_used_shrink_fit
              && !line_has_tab
              && pending_tab.is_none()
              && !tab_over_margin_active
              && line_fits_with_word_space_shrink(
                LineShrinkFit {
                  x,
                  width: fit_width_with_continuation,
                  text: &segment.text,
                  line_right,
                  items: &current.items,
                  item_start: line_item_start_index,
                  chunk: &chunk,
                  style: &run.style,
                  minimum_word_spacing_pct: justification.word_spacing.minimum_pct,
                },
                text_metrics,
              );
            if fits_with_shrink {
              line_used_shrink_fit = true;
            }

            let mut started_new_line = false;
            if overflows_line
              && !fits_with_shrink
              && x > line_left
              && pending_tab.is_none()
              && !tab_over_margin_active
            {
              flush_text(
                current,
                TextPlacement {
                  x_pt: chunk_x,
                  y_pt: y,
                  line_height_pt: line_height,
                },
                &mut chunk,
                &run.style,
                &meta,
              );
              apply_pending_aligned_tab(
                current,
                &mut pending_tab,
                text_metrics,
                y,
                line_left,
                line_right,
              );
              (flow, text_frame, y, line_left, line_right) = self.advance_line(
                TextLineAdvance {
                  current,
                  pages,
                  text_metrics: &mut *text_metrics,
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
              line_used_shrink_fit = false;
              line_used_punctuation_fit = false;
              line_has_tab = false;
              pending_tab = None;
              tab_over_margin_active = false;
              started_new_line = true;
              if whitespace {
                emitted = true;
                continue;
              }
            }

            if fit_width > line_capacity && x <= line_left && !whitespace {
              let mut text_offset = segment.start;
              for text in emergency_break_segments(&segment.text) {
                let width = text_metrics.measure_text(&text, &run.style);
                let fit_width = line_fit_width(&text, &run.style, text_metrics);
                if fit_width > line_capacity && text.chars().count() > 1 {
                  for ch in text.chars() {
                    let mut encoded = [0; 4];
                    let text = ch.encode_utf8(&mut encoded);
                    let width = text_metrics.measure_text(text, &run.style);
                    let fit_width = line_fit_width(text, &run.style, text_metrics);
                    let natural_overflow = cjk_line_character(ch)
                      && line_natural_overflows_with_cjk_compression(
                        CjkLineFit {
                          x,
                          line_right,
                          items: &current.items,
                          item_start: line_item_start_index,
                          chunk: &chunk,
                          text,
                          style: &run.style,
                        },
                        text_metrics,
                      );
                    let blocks_second_extra_character =
                      natural_overflow && line_used_punctuation_fit && !cjk_cannot_start_line(ch);
                    text_offset += text.len();

                    let starts_new_line = (text_overflows_line(x, fit_width, line_right)
                      || blocks_second_extra_character)
                      && x > line_left;
                    if starts_new_line {
                      flush_text(
                        current,
                        TextPlacement {
                          x_pt: chunk_x,
                          y_pt: y,
                          line_height_pt: line_height,
                        },
                        &mut chunk,
                        &run.style,
                        &meta,
                      );
                      apply_pending_aligned_tab(
                        current,
                        &mut pending_tab,
                        text_metrics,
                        y,
                        line_left,
                        line_right,
                      );
                      (flow, text_frame, y, line_left, line_right) = self.advance_line(
                        TextLineAdvance {
                          current,
                          pages,
                          text_metrics: &mut *text_metrics,
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
                      line_used_shrink_fit = false;
                      line_used_punctuation_fit = false;
                      line_has_tab = false;
                      pending_tab = None;
                    }

                    if chunk.is_empty() {
                      chunk_x = x;
                    }
                    chunk.push_str(text);
                    x += width;
                    if natural_overflow && !starts_new_line {
                      line_used_punctuation_fit = true;
                    }
                    text_state.set_position(InlineCursor {
                      inline_index,
                      text_offset,
                    });
                    if segment_affects_line_height(text) {
                      line_height = include_text_height(
                        line_height,
                        text_frame,
                        &run.style,
                        text,
                        text_metrics,
                      );
                    }
                    line_has_form_widget |= meta.form_widget_id.is_some();
                    emitted = true;
                  }
                  continue;
                }
                let only_char = text.chars().next().filter(|_| text.chars().count() == 1);
                let natural_overflow = only_char.is_some_and(cjk_line_character)
                  && line_natural_overflows_with_cjk_compression(
                    CjkLineFit {
                      x,
                      line_right,
                      items: &current.items,
                      item_start: line_item_start_index,
                      chunk: &chunk,
                      text: &text,
                      style: &run.style,
                    },
                    text_metrics,
                  );
                let blocks_second_extra_character = natural_overflow
                  && line_used_punctuation_fit
                  && only_char.is_none_or(|ch| !cjk_cannot_start_line(ch));
                text_offset += text.len();

                let starts_new_line = (text_overflows_line(x, fit_width, line_right)
                  || blocks_second_extra_character)
                  && x > line_left;
                if starts_new_line {
                  flush_text(
                    current,
                    TextPlacement {
                      x_pt: chunk_x,
                      y_pt: y,
                      line_height_pt: line_height,
                    },
                    &mut chunk,
                    &run.style,
                    &meta,
                  );
                  apply_pending_aligned_tab(
                    current,
                    &mut pending_tab,
                    text_metrics,
                    y,
                    line_left,
                    line_right,
                  );
                  (flow, text_frame, y, line_left, line_right) = self.advance_line(
                    TextLineAdvance {
                      current,
                      pages,
                      text_metrics: &mut *text_metrics,
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
                  line_used_shrink_fit = false;
                  line_used_punctuation_fit = false;
                  line_has_tab = false;
                  pending_tab = None;
                }

                if chunk.is_empty() {
                  chunk_x = x;
                }
                chunk.push_str(&text);
                x += width;
                if natural_overflow && !starts_new_line {
                  line_used_punctuation_fit = true;
                }
                text_state.set_position(InlineCursor {
                  inline_index,
                  text_offset,
                });
                if segment_affects_line_height(&text) {
                  line_height =
                    include_text_height(line_height, text_frame, &run.style, &text, text_metrics);
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
            if natural_overflow && !started_new_line {
              line_used_punctuation_fit = true;
            }
            text_state.set_position(InlineCursor {
              inline_index,
              text_offset: segment.end,
            });
            if segment_affects_line_height(&segment.text) {
              line_height = include_text_height(
                line_height,
                text_frame,
                &run.style,
                &segment.text,
                text_metrics,
              );
            }
            line_has_form_widget |= meta.form_widget_id.is_some();
            emitted = true;
            tab_over_margin_active = pending_tab
              .as_ref()
              .is_some_and(|tab| tab.stop.x_pt > line_right);
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
                &meta,
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
            &meta,
          );
          apply_pending_aligned_tab(
            current,
            &mut pending_tab,
            text_metrics,
            y,
            line_left,
            line_right,
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
          if flow.text_segmentation == TextSegmentation::Body
            && emitted
            && page_has_body_region_items(current, flow)
            && y + line_height > flow.content_bottom - LAYOUT_EPSILON_PT
          {
            // cursor where the previous layout created a page follow. Writer
            // uses that evidence while laying out body text, so keep the
            // following portions on the follow page instead of reflowing the
            // whole paragraph continuously.
            text_state.finish_line(y, line_height);
            line_has_form_widget = false;
            (flow, text_frame, y, line_left, line_right, line_height) =
              self.force_text_page_break(flow, current, pages, text_metrics, &mut wrap_exclusions);
            default_line_right = text_frame.default_line_right;
            paragraph_left = text_frame.paragraph_left;
            base_line_height = text_frame.base_line_height;
            x = line_left;
            line_item_start_index = current.items.len();
            line_used_shrink_fit = false;
            line_used_punctuation_fit = false;
            line_has_tab = false;
            emitted = false;
          }
          pending_tab = None;
        }
        InlineItem::Image(image) => {
          ended_with_explicit_page_break = false;
          text_state.set_position(InlineCursor::after_inline(inline_index));
          pending_tab = None;
          if let crate::docx::ImagePlacement::Floating(placement) = image.placement {
            let frame_width =
              relative_floating_width(placement, flow).unwrap_or_else(|| image_frame_width(image));
            let frame_height = relative_floating_height(placement, flow)
              .unwrap_or_else(|| image_frame_height(image));
            let (width, height) = fit_image_to_line(frame_width, frame_height, flow.content_width);
            let line_anchor_top =
              floating_anchor_line_top(flow, paragraph, y, line_height, text_metrics);
            let anchor_y =
              floating_anchor_reference_y(placement, paragraph_anchor_top, line_anchor_top);
            let (image_x, image_y) =
              floating_image_position(placement, flow, x, anchor_y, width, height);
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
              semantic_metafile_text: image.semantic_metafile_text,
              floating: true,
              behind_text: placement.behind_text,
            }));
            if effective_layout_in_cell(placement, flow) {
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
                  if y + base_line_height > flow.content_bottom
                    && page_has_body_region_items(current, flow)
                  {
                    (flow, y) = advance_section_flow(flow, current, pages);
                    text_frame = TextFrame::new(self.paragraph, flow, text_metrics);
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
                line_used_shrink_fit = false;
                line_used_punctuation_fit = false;
                line_has_tab = false;
              }
              ImageWrapMode::Square | ImageWrapMode::Tight => {
                // wp:behindDoc controls z-order, not the wrapping contract.
                // Writer still creates a contour/square text exclusion for a
                // behind-document object when wp:wrapTight/wp:wrapSquare is
                // present (tdf#128198).
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
              ImageWrapMode::Through if effective_layout_in_cell(placement, flow) => {
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
                  (line_left, line_right) =
                    self.line_bounds(text_frame, y, line_height, &wrap_exclusions);
                  x = line_left;
                  line_height = base_line_height;
                  line_used_shrink_fit = false;
                  line_used_punctuation_fit = false;
                  line_has_tab = false;
                }
              }
              ImageWrapMode::Through | ImageWrapMode::Inline => {}
            }
            if placement.behind_text {
              behind_text_floating_only = true;
            } else {
              emitted = true;
            }
            continue;
          }
          if pending_text_page_break {
            (flow, text_frame, y, line_left, line_right, line_height) =
              self.force_text_page_break(flow, current, pages, text_metrics, &mut wrap_exclusions);
            default_line_right = text_frame.default_line_right;
            paragraph_left = text_frame.paragraph_left;
            base_line_height = text_frame.base_line_height;
            x = line_left;
            line_item_start_index = current.items.len();
            line_has_form_widget = false;
            pending_text_page_break = false;
            line_used_shrink_fit = false;
            line_used_punctuation_fit = false;
            line_has_tab = false;
          }
          let metrics = inline_image_metrics(image, flow.content_width);
          if x + metrics.frame_width_pt > line_right && x > line_left {
            (flow, text_frame, y, line_left, line_right) = self.advance_line(
              TextLineAdvance {
                current,
                pages,
                text_metrics: &mut *text_metrics,
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
            line_used_shrink_fit = false;
            line_used_punctuation_fit = false;
            line_has_tab = false;
          }
          let object_line_height = inline_drawing_line_height(
            metrics.frame_height_pt,
            paragraph,
            text_frame,
            text_metrics,
          );
          if line_item_start_index == current.items.len()
            && let Some(next) = self.advance_for_inline_object(InlineObjectAdvance {
              current,
              pages,
              text_metrics,
              wrap_exclusions: &mut wrap_exclusions,
              state: &mut text_state,
              flow,
              y,
              object_line_height,
            })
          {
            (flow, text_frame, y, line_left, line_right, line_height) = next;
            default_line_right = text_frame.default_line_right;
            paragraph_left = text_frame.paragraph_left;
            base_line_height = text_frame.base_line_height;
            x = line_left;
            line_item_start_index = current.items.len();
            line_has_form_widget = false;
            line_used_shrink_fit = false;
            line_used_punctuation_fit = false;
            line_has_tab = false;
          }
          current.items.push(PageItem::Image(ImageItem {
            x_pt: x + metrics.content_offset_x_pt,
            y_pt: y + metrics.content_offset_y_pt,
            width_pt: metrics.content_width_pt,
            height_pt: metrics.content_height_pt,
            crop: image.crop,
            rotation_deg: image.rotation_deg,
            flip_horizontal: image.flip_horizontal,
            flip_vertical: image.flip_vertical,
            data: image.data.clone(),
            content_type: image.content_type.clone(),
            alt_text: image.alt_text.clone(),
            hyperlink_url: image.hyperlink_url.clone(),
            semantic_metafile_text: image.semantic_metafile_text,
            floating: false,
            behind_text: false,
          }));
          if flow.text_segmentation == TextSegmentation::Notes {
            current.items.push(PageItem::Rect(RectItem {
              x_pt: x + metrics.content_offset_x_pt,
              y_pt: y + metrics.content_offset_y_pt,
              width_pt: metrics.content_width_pt,
              height_pt: metrics.content_height_pt,
              fill_color: None,
              fill_opacity: 1.0,
              stroke: Some(BorderStyle::default()),
              stroke_opacity: 0.0,
            }));
          }
          x += metrics.frame_width_pt;
          line_height = line_height.max(object_line_height);
          emitted = true;
        }
        InlineItem::Shape(shape) => {
          ended_with_explicit_page_break = false;
          text_state.set_position(InlineCursor::after_inline(inline_index));
          pending_tab = None;
          let place_shape = |current: &mut Page,
                             shape_flow: FlowContext,
                             text_metrics: &mut TextMetrics,
                             x_pt: f32,
                             y_pt: f32,
                             width_pt: f32,
                             height_pt: f32| {
            let item_start = current.items.len();
            if let Some(chart) = &shape.chart {
              current.items.extend(lower_inline_chart(
                chart,
                x_pt,
                y_pt,
                width_pt,
                height_pt,
                text_metrics,
              ));
              return (item_start, current.items.len());
            }
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
                semantic_metafile_text: false,
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
              layout_shape_text_box(
                current,
                shape_flow,
                text_metrics,
                shape,
                ShapeTextBoxRect {
                  x: x_pt,
                  y: y_pt,
                  width: width_pt,
                  height: height_pt,
                },
              );
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
            if shape_has_invisible_auto_fit_textbox_bounds(shape, shape_flow) {
              // content even when the owning draw shape has no fill/stroke,
              // but that layout-only frame is not painted as a polypolygon.
              current.items.push(PageItem::Rect(RectItem {
                x_pt,
                y_pt,
                width_pt,
                height_pt,
                fill_color: None,
                fill_opacity: 1.0,
                stroke: Some(BorderStyle::default()),
                stroke_opacity: 0.0,
              }));
            }
            layout_shape_text_box(
              current,
              shape_flow,
              text_metrics,
              shape,
              ShapeTextBoxRect {
                x: x_pt,
                y: y_pt,
                width: width_pt,
                height: height_pt,
              },
            );
            (item_start, current.items.len())
          };

          match shape.placement {
            crate::docx::ImagePlacement::Floating(placement) => {
              if floating_shape_is_zero_relative_background(placement, shape) {
                continue;
              }
              let width = relative_floating_width(placement, flow).unwrap_or(shape.width_pt);
              let height = relative_floating_height(placement, flow).unwrap_or(shape.height_pt);
              let line_anchor_top =
                floating_anchor_line_top(flow, paragraph, y, line_height, text_metrics);
              let anchor_y =
                floating_anchor_reference_y(placement, paragraph_anchor_top, line_anchor_top);
              let (shape_x, shape_y) =
                floating_image_position(placement, flow, x, anchor_y, width, height);
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
              let shape_paint_y = if effective_layout_in_cell(placement, flow)
                && matches!(placement.wrap, ImageWrapMode::Square | ImageWrapMode::Tight)
                && shape.text_box_blocks.is_empty()
              {
                shape_y - height
              } else {
                shape_y
              };
              if flow.text_segmentation == TextSegmentation::RepeatingSlot
                && !floating_shape_intersects_page(
                  shape,
                  shape_x,
                  shape_paint_y,
                  width,
                  height,
                  flow,
                )
              {
                continue;
              }
              let allows_outside_page =
                shape.allow_outside_page || floating_shape_may_extend_outside_page(placement);
              let allows_repeating_textbox_overflow = flow.text_segmentation
                == TextSegmentation::RepeatingSlot
                && !shape.text_box_blocks.is_empty();
              let (shape_x, shape_paint_y) =
                if allows_outside_page || allows_repeating_textbox_overflow {
                  (shape_x, shape_paint_y)
                } else {
                  keep_floating_shape_inside_page(shape_x, shape_paint_y, width, height, flow)
                };
              let (shape_item_start, shape_item_end) = place_shape(
                current,
                flow,
                text_metrics,
                shape_x,
                shape_paint_y,
                width,
                height,
              );
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
                  height_pt: (shape_paint_y + height - y - BorderStyle::default().width_pt)
                    .max(height),
                  fill_color: None,
                  fill_opacity: 1.0,
                  stroke: Some(BorderStyle::default()),
                  stroke_opacity: 0.0,
                }));
              }
              let influence_bounds = Some(FrameBounds {
                x_pt: shape_x - placement.margin_left_pt,
                y_pt: shape_paint_y - placement.margin_top_pt,
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
                    y = y.max(shape_y + height + placement.margin_bottom_pt);
                    if y + base_line_height > flow.content_bottom
                      && page_has_body_region_items(current, flow)
                    {
                      (flow, y) = advance_section_flow(flow, current, pages);
                      text_frame = TextFrame::new(self.paragraph, flow, text_metrics);
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
                ImageWrapMode::Square | ImageWrapMode::Tight
                  if effective_layout_in_cell(placement, flow) =>
                {
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
                    y = y.max(shape_y + height + placement.margin_bottom_pt);
                    if y + base_line_height > flow.content_bottom
                      && page_has_body_region_items(current, flow)
                    {
                      (flow, y) = advance_section_flow(flow, current, pages);
                      text_frame = TextFrame::new(self.paragraph, flow, text_metrics);
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
                ImageWrapMode::Square | ImageWrapMode::Tight => {
                  // A behind-document shape remains a wrap object. The z-order
                  // bit only decides whether its paint is below document text.
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
                ImageWrapMode::Through if effective_layout_in_cell(placement, flow) => {
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
                    y = y.max(shape_y + height + placement.margin_bottom_pt);
                    if y + base_line_height > flow.content_bottom
                      && page_has_body_region_items(current, flow)
                    {
                      (flow, y) = advance_section_flow(flow, current, pages);
                      text_frame = TextFrame::new(self.paragraph, flow, text_metrics);
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
                ImageWrapMode::Through | ImageWrapMode::Inline => {}
              }
            }
            crate::docx::ImagePlacement::Inline => {
              if pending_text_page_break {
                (flow, text_frame, y, line_left, line_right, line_height) = self
                  .force_text_page_break(flow, current, pages, text_metrics, &mut wrap_exclusions);
                default_line_right = text_frame.default_line_right;
                paragraph_left = text_frame.paragraph_left;
                base_line_height = text_frame.base_line_height;
                x = line_left;
                line_item_start_index = current.items.len();
                line_has_form_widget = false;
                emitted = false;
                pending_text_page_break = false;
              }
              let mut compatibility_forced_shape_line = false;
              if x + shape.width_pt > line_right && x > line_left {
                (flow, text_frame, y, line_left, line_right) = self.advance_line(
                  TextLineAdvance {
                    current,
                    pages,
                    text_metrics: &mut *text_metrics,
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
                compatibility_forced_shape_line = true;
                line_used_shrink_fit = false;
                line_used_punctuation_fit = false;
                line_has_tab = false;
              }
              let object_line_height =
                inline_drawing_line_height(shape.height_pt, paragraph, text_frame, text_metrics);
              if line_item_start_index == current.items.len()
                && let Some(next) = self.advance_for_inline_object(InlineObjectAdvance {
                  current,
                  pages,
                  text_metrics,
                  wrap_exclusions: &mut wrap_exclusions,
                  state: &mut text_state,
                  flow,
                  y,
                  object_line_height,
                })
              {
                (flow, text_frame, y, line_left, line_right, line_height) = next;
                default_line_right = text_frame.default_line_right;
                paragraph_left = text_frame.paragraph_left;
                base_line_height = text_frame.base_line_height;
                x = line_left;
                line_item_start_index = current.items.len();
                line_has_form_widget = false;
                line_used_shrink_fit = false;
                line_used_punctuation_fit = false;
                line_has_tab = false;
              }
              let _ = place_shape(
                current,
                flow,
                text_metrics,
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
              line_height = if flow.compatibility_mode < 15 && compatibility_forced_shape_line {
                shape.height_pt.max(LAYOUT_EPSILON_PT)
              } else {
                line_height.max(object_line_height)
              };
            }
          }
          if matches!(
            shape.placement,
            crate::docx::ImagePlacement::Floating(placement) if placement.behind_text
          ) {
            behind_text_floating_only = true;
          } else {
            emitted = true;
          }
        }
        InlineItem::PageBreak => {
          ended_with_explicit_page_break = true;
          text_state.set_position(InlineCursor::after_inline(inline_index));
          text_state.finish_line(y, line_height);
          line_has_form_widget = false;
          if can_defer_page_break_for_following_floating_anchor(
            paragraph,
            inline_index + 1,
            emitted,
            flow.split_page_break_and_paragraph_mark,
          ) {
            pending_text_page_break = true;
          } else {
            (flow, text_frame, y, line_left, line_right, line_height) =
              self.force_text_page_break(flow, current, pages, text_metrics, &mut wrap_exclusions);
            default_line_right = text_frame.default_line_right;
            paragraph_left = text_frame.paragraph_left;
            base_line_height = text_frame.base_line_height;
            x = line_left;
            line_item_start_index = current.items.len();
            emitted = false;
            line_used_shrink_fit = false;
            line_used_punctuation_fit = false;
            line_has_tab = false;
          }
          pending_tab = None;
        }
        InlineItem::ColumnBreak => {
          ended_with_explicit_page_break = false;
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
          ) = self.apply_column_break(flow, current, pages, text_metrics, &mut wrap_exclusions);
          default_line_right = text_frame.default_line_right;
          paragraph_left = text_frame.paragraph_left;
          base_line_height = text_frame.base_line_height;
          x = line_left;
          line_item_start_index = current.items.len();
          line_has_form_widget = false;
          emitted = column_emitted;
          line_used_shrink_fit = false;
          line_used_punctuation_fit = false;
          line_has_tab = false;
          pending_tab = None;
        }
      }
    }
    if pending_text_page_break {
      (flow, text_frame, y, _, _, line_height) =
        self.force_text_page_break(flow, current, pages, text_metrics, &mut wrap_exclusions);
      default_line_right = text_frame.default_line_right;
      paragraph_left = text_frame.paragraph_left;
      base_line_height = text_frame.base_line_height;
      line_item_start_index = current.items.len();
      line_has_form_widget = false;
      emitted = false;
    }

    let paragraph_bottom;
    if emitted {
      if paragraph.format.bidi {
        reorder_bidi_line_items(&mut current.items, line_item_start_index, y, text_metrics);
      }
      finalize_cjk_punctuation_compression(
        &mut current.items,
        line_item_start_index,
        y,
        line_right,
        text_metrics,
      );
      align_text_baseline_to_inline_object(
        &mut current.items,
        line_item_start_index,
        y,
        text_metrics,
      );
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
          bounds: (line_item_start_index == current.items.len()).then_some(FrameBounds {
            x_pt: line_left,
            y_pt: y,
            width_pt: (line_right - line_left).max(0.0),
            height_pt: line_height,
          }),
        },
        text_metrics,
      );
      let line_index = text_state.line_fragments.len();
      let real_height = line_real_height(
        paragraph,
        line_height,
        line_index,
        text_frame.grid_auto_line_spacing_multiplier,
        line_has_form_widget,
      );
      text_state.finish_paragraph(y, real_height, emitted);
      paragraph_bottom = y + real_height;
      y = paragraph_bottom + self.spacing_after_pt;
    } else if ended_with_explicit_page_break && !flow.split_page_break_and_paragraph_mark {
      // By default, a page break at the end of a paragraph does not allocate
      // an additional empty text line before the following paragraph. The
      // w:splitPgBreakAndParaMark compatibility switch intentionally retains
      // that line and therefore stays on the normal empty-paragraph path.
      paragraph_bottom = y;
    } else if behind_text_floating_only {
      paragraph_bottom = y + base_line_height;
      y = paragraph_bottom + self.spacing_after_pt;
    } else {
      if flow.setup.line_numbering.is_some() {
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
            bounds: Some(FrameBounds {
              x_pt: line_left,
              y_pt: y,
              width_pt: (line_right - line_left).max(0.0),
              height_pt: base_line_height,
            }),
          },
          text_metrics,
        );
        text_state.finish_paragraph(y, base_line_height, true);
      }
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
    let widow_rebalance_has_variable_line_widths = !current.wrap_exclusions.is_empty()
      || pages
        .iter()
        .skip(start_pages_len)
        .any(|page| !page.wrap_exclusions.is_empty());
    if allow_reflow {
      match split_decision {
        TextSplitDecision::Rejected => {
          restore_text_frame_start_page(
            current,
            pages,
            start_pages_len,
            start_current,
            start_page_snapshot.as_ref(),
            start_pending_snapshot.as_ref(),
          );
          if let (Some(anchors), Some(len)) = (anchor_pages.as_deref_mut(), start_anchor_pages_len)
          {
            anchors.truncate(len);
          }
          let (follow_flow, follow_y) = advance_text_frame_flow(start_flow, current, pages);
          return TextFrameLayout::new(paragraph, follow_flow, self.spacing_after_pt, text_metrics)
            .format_with_reflow(current, pages, anchor_pages, text_metrics, follow_y, false);
        }
        TextSplitDecision::RebalanceWidow { page_index }
          if can_rebalance_widow_by_reserving_line && !widow_rebalance_has_variable_line_widths =>
        {
          restore_text_frame_start_page(
            current,
            pages,
            start_pages_len,
            start_current,
            start_page_snapshot.as_ref(),
            start_pending_snapshot.as_ref(),
          );
          if let (Some(anchors), Some(len)) = (anchor_pages.as_deref_mut(), start_anchor_pages_len)
          {
            anchors.truncate(len);
          }
          return TextFrameLayout::new(paragraph, start_flow, self.spacing_after_pt, text_metrics)
            .with_widow_rebalance_page(page_index)
            .format_with_reflow(
              current,
              pages,
              anchor_pages,
              text_metrics,
              paragraph_top,
              false,
            );
        }
        TextSplitDecision::NoSplit
        | TextSplitDecision::Forced
        | TextSplitDecision::Allowed
        | TextSplitDecision::RebalanceWidow { .. } => {}
      }
    }

    if paragraph.list_label.is_none() && start_item_index <= current.items.len() {
      align_paragraph_items(
        &mut current.items[start_item_index..],
        effective_paragraph_alignment(paragraph),
        text_metrics,
        default_line_right,
      );
    }
    if start_item_index <= current.items.len() {
      decorate_paragraph(
        current,
        ParagraphDecoration {
          start_item_index,
          paragraph,
          flow,
          x: paragraph_left,
          y: paragraph_top,
          width: default_line_right - paragraph_left,
          height: paragraph_bottom - paragraph_top,
        },
      );
    }

    (flow, y)
  }
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
    if ch != '\n' && ch != '\t' && ch != PRESERVED_WORD_TEXT_TAB {
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

fn cross_run_unbreakable_continuation_width(
  inlines: &[InlineItem],
  inline_index: usize,
  current_segment: &str,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if current_segment.is_empty()
    || current_segment
      .chars()
      .last()
      .is_some_and(char::is_whitespace)
  {
    return 0.0;
  }

  let mut combined = current_segment.to_string();
  let mut following_text = Vec::new();
  for inline in &inlines[inline_index.saturating_add(1)..] {
    let InlineItem::Text(run) = inline else {
      break;
    };
    if run.text.is_empty() {
      continue;
    }
    combined.push_str(&run.text);
    following_text.push(run);
    let first_segment_len = text_segments(&combined).first().map_or(0, String::len);
    if first_segment_len < combined.len() {
      break;
    }
  }

  let first_segment_len = text_segments(&combined).first().map_or(0, String::len);
  let continuation_len = combined
    .get(current_segment.len()..first_segment_len)
    .unwrap_or_default()
    .trim_end_matches(libreoffice_line_end_elidable_blank)
    .len();
  let mut remaining = continuation_len;
  let mut width = 0.0;
  for run in following_text {
    if remaining == 0 {
      break;
    }
    let take = remaining.min(run.text.len());
    debug_assert!(run.text.is_char_boundary(take));
    width += text_metrics.measure_text(&run.text[..take], &run.style);
    remaining -= take;
  }
  width
}

fn drawing_layer_text_segments_with_offsets(text: &str) -> Vec<TextSegment> {
  let mut output = Vec::new();
  let mut start = 0usize;

  for (index, ch) in text.char_indices() {
    if ch == '\n' || ch == '\t' || ch == PRESERVED_WORD_TEXT_TAB {
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

fn line_fit_width(text: &str, style: &TextStyle, text_metrics: &mut TextMetrics) -> f32 {
  // Spaces from UAX #14 SP/BA classes are elided at line end for line-break
  // fitting. Keep the original segment width for painting and following text
  // advance; only the fit test ignores trailing collapsible blanks.
  let fit_text = text.trim_end_matches(libreoffice_line_end_elidable_blank);
  if fit_text.len() == text.len() {
    text_metrics.measure_text(text, style)
  } else {
    text_metrics.measure_text(fit_text, style)
  }
}

struct CjkLineFit<'a> {
  x: f32,
  line_right: f32,
  items: &'a [PageItem],
  item_start: usize,
  chunk: &'a str,
  text: &'a str,
  style: &'a TextStyle,
}

fn line_natural_overflows_with_cjk_compression(
  fit: CjkLineFit<'_>,
  text_metrics: &mut TextMetrics,
) -> bool {
  if fit.style.cjk_punctuation_compression_ratio <= 0.0 {
    return false;
  }
  let mut capacity = 0.0;
  for item in fit.items.iter().skip(fit.item_start) {
    let PageItem::Text(text) = item else {
      continue;
    };
    capacity += cjk_punctuation_compression_capacity(&text.text, &text.style, text_metrics);
  }
  capacity += cjk_punctuation_compression_capacity(fit.chunk, fit.style, text_metrics);
  let mut natural_style = fit.style.clone();
  natural_style.cjk_punctuation_compression_ratio = 0.0;
  round_style_to_word_print_grid(&mut natural_style);
  let natural_next_width = text_metrics.measure_text(fit.text, &natural_style);
  fit.x + capacity + natural_next_width > fit.line_right
}

fn cjk_punctuation_compression_capacity(
  text: &str,
  style: &TextStyle,
  text_metrics: &mut TextMetrics,
) -> f32 {
  if text.is_empty() || style.cjk_punctuation_compression_ratio <= 0.0 {
    return 0.0;
  }
  let mut natural_style = style.clone();
  natural_style.cjk_punctuation_compression_ratio = 0.0;
  round_style_to_word_print_grid(&mut natural_style);
  let mut maximum_style = style.clone();
  maximum_style.cjk_punctuation_compression_ratio = 1.0;
  (text_metrics.measure_text(text, &natural_style)
    - text_metrics.measure_text(text, &maximum_style))
  .max(0.0)
}

fn word_print_grid_font_size(font_size_pt: f32) -> f32 {
  const WORD_PRINT_DPI: f32 = 600.0;
  (font_size_pt * WORD_PRINT_DPI / 72.0).round() * 72.0 / WORD_PRINT_DPI
}

fn round_style_to_word_print_grid(style: &mut TextStyle) {
  style.font_size_pt = word_print_grid_font_size(style.font_size_pt);
  style.complex_font_size_pt = style.complex_font_size_pt.map(word_print_grid_font_size);
}

fn cjk_cannot_start_line(ch: char) -> bool {
  matches!(
    ch,
    '\u{3001}'
      | '\u{3002}'
      | '\u{3009}'
      | '\u{300B}'
      | '\u{300D}'
      | '\u{300F}'
      | '\u{3011}'
      | '\u{3015}'
      | '\u{3017}'
      | '\u{3019}'
      | '\u{301B}'
      | '\u{301E}'
      | '\u{301F}'
      | '\u{FF01}'
      | '\u{FF09}'
      | '\u{FF0C}'
      | '\u{FF0E}'
      | '\u{FF1A}'
      | '\u{FF1B}'
      | '\u{FF1F}'
      | '\u{FF3D}'
      | '\u{FF5D}'
  )
}

fn cjk_line_character(ch: char) -> bool {
  matches!(
    ch as u32,
    0x2E80..=0x2FFF
      | 0x3000..=0x30FF
      | 0x3100..=0x312F
      | 0x31A0..=0x31BF
      | 0x31F0..=0x31FF
      | 0x3400..=0x4DBF
      | 0x4E00..=0x9FFF
      | 0xAC00..=0xD7AF
      | 0xF900..=0xFAFF
      | 0xFE30..=0xFE4F
      | 0xFF00..=0xFFEF
      | 0x20000..=0x323AF
  )
}

fn text_overflows_line(x: f32, width: f32, line_right: f32) -> bool {
  x + width > line_right
}

struct LineShrinkFit<'a> {
  x: f32,
  width: f32,
  text: &'a str,
  line_right: f32,
  items: &'a [PageItem],
  item_start: usize,
  chunk: &'a str,
  style: &'a TextStyle,
  minimum_word_spacing_pct: u16,
}

fn line_fits_with_word_space_shrink(
  fit: LineShrinkFit<'_>,
  text_metrics: &mut TextMetrics,
) -> bool {
  let overflow = fit.x + fit.width - fit.line_right;
  if overflow <= 0.0 {
    return true;
  }

  let mut space_counter = LineSpaceCounter::default();
  for item in fit.items.iter().skip(fit.item_start) {
    if let PageItem::Text(text) = item {
      space_counter.push_text(&text.text);
    }
  }
  space_counter.push_text(fit.chunk);
  // Writer's second line-break guess includes the extra separator that
  // becomes internal when shrinking admits the next word (tdf#158776).
  // Without the candidate segment, the existing terminal blank is removed
  // from the shrink budget even though it will no longer end the line.
  space_counter.push_text(fit.text);
  let space_count = space_counter.non_terminal_space_count();
  if space_count == 0 {
    return false;
  }
  let space_width = text_metrics.measure_text(" ", fit.style);
  if space_width <= f32::EPSILON {
    return false;
  }
  let minimum_ratio = fit.minimum_word_spacing_pct as f32 / PERCENT_SCALE;
  let shrinkability = space_count as f32 * space_width * (1.0 - minimum_ratio);
  if overflow > shrinkability {
    return false;
  }

  let spaces = space_count as f32;
  let normal_space = space_width + (fit.line_right - fit.x).max(0.0) / spaces;
  let shrunk_space = space_width - overflow / spaces;
  if shrunk_space <= space_width * minimum_ratio {
    return false;
  }

  let expansion_weight = 1.0 / 1.7;
  let z0 = space_width / shrunk_space;
  let z1 = (space_width + (normal_space - space_width) * expansion_weight) / space_width;
  z1 >= z0
}

#[derive(Default)]
struct LineSpaceCounter {
  spaces: usize,
  trailing_spaces: usize,
}

impl LineSpaceCounter {
  fn push_text(&mut self, text: &str) {
    for byte in text.bytes() {
      if byte == b' ' {
        self.spaces += 1;
        self.trailing_spaces += 1;
      } else {
        self.trailing_spaces = 0;
      }
    }
  }

  fn non_terminal_space_count(self) -> usize {
    self.spaces.saturating_sub(self.trailing_spaces)
  }
}

fn libreoffice_line_end_elidable_blank(ch: char) -> bool {
  matches!(ch, ' ' | '\u{3000}' | '\u{2006}')
}

fn push_line_segments(text: &str, segments: &mut Vec<String>) {
  if text.is_empty() {
    return;
  }
  if libreoffice_preserves_latin_line_token(text) {
    push_text_line_segment(text, segments);
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

fn libreoffice_preserves_latin_line_token(text: &str) -> bool {
  // for a break around the current cut position. A Latin token made of letters,
  // numbers and inline punctuation has no internal line break opportunity, even
  // when ICU's generic line segmenter reports punctuation-adjacent boundaries.
  text
    .chars()
    .all(|ch| ch.is_ascii_graphic() && ch != '\n' && ch != '\t')
}

fn push_text_line_segment(text: &str, segments: &mut Vec<String>) {
  if !segments.is_empty()
    && text
      .chars()
      .next()
      .is_some_and(libreoffice_forbidden_line_start_after_text)
  {
    let previous = segments.last_mut().unwrap();
    previous.push_str(text);
    return;
  }
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
  // handling, so opening quotation marks don't remain alone at line end.
  matches!(ch, '“' | '‘')
}

fn libreoffice_forbidden_line_start_after_text(ch: char) -> bool {
  // begin-line characters to i18npool getLineBreak(), preventing punctuation
  // portions such as "," from starting their own line.
  matches!(
    ch,
    ','
      | '.'
      | ';'
      | ':'
      | '!'
      | '?'
      | ')'
      | ']'
      | '}'
      | '”'
      | '’'
      | '，'
      | '。'
      | '、'
      | '；'
      | '：'
      | '！'
      | '？'
      | '）'
      | '］'
      | '｝'
      | '」'
      | '』'
  )
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
      leader: stop.leader,
    };
  }

  let default_tab_stop_pt = default_tab_stop_pt.max(DEFAULT_FONT_SIZE_PT);
  ResolvedTabStop {
    // ECMA-376 Part 1 §17.15.1.25 repeats automatic stops across the
    // displayed paragraph area. Word anchors that grid at the paragraph text
    // origin: when a long numbering label passes its explicit stop, the next
    // automatic stop remains a defaultTabStop multiple from line_left rather
    // than from physical page coordinate zero.
    x_pt: line_left + ((relative_x / default_tab_stop_pt).floor() + 1.0) * default_tab_stop_pt,
    alignment: TabStopAlignment::Left,
    leader: TabLeader::None,
  }
}

fn line_bounds_for_y(
  default_left: f32,
  default_right: f32,
  y: f32,
  line_height: f32,
  exclusions: &[WrapExclusion],
) -> (f32, f32) {
  available_line_bounds_for_y(default_left, default_right, y, line_height, exclusions)
    .unwrap_or((default_left, default_right))
}

fn available_line_bounds_for_y(
  default_left: f32,
  default_right: f32,
  y: f32,
  line_height: f32,
  exclusions: &[WrapExclusion],
) -> Option<(f32, f32)> {
  let mut left = default_left;
  let mut right = default_right;
  let mut obstructed = false;
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
    obstructed = true;

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

  if obstructed && right - left < DEFAULT_FONT_SIZE_PT {
    None
  } else {
    Some((left, right))
  }
}

fn dodge_text_wrap_exclusions(
  mut y: f32,
  line_height: f32,
  default_left: f32,
  default_right: f32,
  exclusions: &[WrapExclusion],
) -> f32 {
  loop {
    let lacks_side_space =
      available_line_bounds_for_y(default_left, default_right, y, line_height, exclusions)
        .is_none();
    let Some(exclusion) = exclusions
      .iter()
      .filter(|exclusion| {
        (exclusion.blocks_flow || lacks_side_space)
          && exclusion.overlaps_vertical_span(y, y + line_height)
          && exclusion.overlaps_horizontal_span(default_left, default_right)
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

fn shift_first_line_below_upper_margin_exclusions(
  mut y: f32,
  upper_margin_pt: f32,
  default_left: f32,
  default_right: f32,
  exclusions: &[WrapExclusion],
) -> f32 {
  if upper_margin_pt <= LAYOUT_EPSILON_PT {
    return y;
  }

  loop {
    let margin_top = y - upper_margin_pt;
    let Some(exclusion) = exclusions
      .iter()
      .filter(|exclusion| {
        !exclusion.blocks_flow
          && exclusion.overlaps_horizontal_span(default_left, default_right)
          && exclusion.top_pt < y - LAYOUT_EPSILON_PT
          && exclusion.bottom_pt > y + LAYOUT_EPSILON_PT
          && exclusion.bottom_pt > margin_top
      })
      .min_by(|a, b| {
        a.bottom_pt
          .partial_cmp(&b.bottom_pt)
          .unwrap_or(std::cmp::Ordering::Equal)
      })
    else {
      return y;
    };
    y = exclusion.bottom_pt;
  }
}

struct ParagraphDecoration<'a> {
  start_item_index: usize,
  paragraph: &'a crate::docx::Paragraph,
  flow: FlowContext,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
}

fn decorate_paragraph(page: &mut Page, decoration: ParagraphDecoration<'_>) {
  let ParagraphDecoration {
    start_item_index,
    paragraph,
    flow,
    x,
    y,
    width,
    height,
  } = decoration;
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

fn align_paragraph_items(
  items: &mut [PageItem],
  alignment: ParagraphAlignment,
  text_metrics: &mut TextMetrics,
  line_right: f32,
) {
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
        && let Some((x, width)) = item_horizontal_bounds(item, text_metrics)
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

fn effective_paragraph_alignment(paragraph: &crate::docx::Paragraph) -> ParagraphAlignment {
  match (paragraph.format.justification.adjust, paragraph.format.bidi) {
    (crate::docx::ParagraphAdjust::Start, true) => ParagraphAlignment::Right,
    (crate::docx::ParagraphAdjust::End, true) => ParagraphAlignment::Left,
    _ => paragraph.format.alignment,
  }
}

fn reorder_bidi_line_items(
  items: &mut [PageItem],
  item_start: usize,
  y: f32,
  text_metrics: &mut TextMetrics,
) {
  let logical_indices = items
    .iter()
    .enumerate()
    .skip(item_start)
    .filter_map(|(index, item)| match item {
      PageItem::Text(text)
        if text.paragraph_bidi && (text.y_pt - y).abs() < 0.01 && !text.text.is_empty() =>
      {
        Some(index)
      }
      _ => None,
    })
    .collect::<Vec<_>>();
  if logical_indices.len() < 2 {
    return;
  }

  let mut logical_text = String::new();
  let mut byte_starts = Vec::with_capacity(logical_indices.len());
  for &index in &logical_indices {
    let PageItem::Text(text) = &items[index] else {
      continue;
    };
    byte_starts.push(logical_text.len());
    logical_text.push_str(&text.text);
  }
  if logical_text.is_empty() {
    return;
  }

  let bidi = BidiInfo::new(&logical_text, Some(Level::rtl()));
  let item_levels = byte_starts
    .iter()
    .map(|&start| bidi.levels[start])
    .collect::<Vec<_>>();
  let visual_order = BidiInfo::reorder_visual(&item_levels);
  if visual_order.iter().copied().eq(0..logical_indices.len()) {
    return;
  }

  let mut left = f32::MAX;
  let mut widths = Vec::with_capacity(logical_indices.len());
  for &index in &logical_indices {
    let Some((x, width)) = item_horizontal_bounds(&items[index], text_metrics) else {
      return;
    };
    left = left.min(x);
    widths.push(width);
  }
  if left == f32::MAX {
    return;
  }

  let mut x = left;
  for logical_index in visual_order {
    let item_index = logical_indices[logical_index];
    let Some((old_x, _)) = item_horizontal_bounds(&items[item_index], text_metrics) else {
      return;
    };
    shift_item_x(&mut items[item_index], x - old_x);
    x += widths[logical_index];
  }
}

fn apply_pending_aligned_tab(
  page: &mut Page,
  pending_tab: &mut Option<PendingAlignedTab>,
  text_metrics: &mut TextMetrics,
  y: f32,
  _line_left: f32,
  line_right: f32,
) {
  let Some(tab) = pending_tab.as_ref() else {
    return;
  };
  if tab.item_start_index >= page.items.len() {
    return;
  }

  let mut min_x = f32::MAX;
  let mut max_x: f32 = tab.stop.x_pt;
  for item in page.items.iter().skip(tab.item_start_index) {
    if let Some(item_y) = item_y(item)
      && f32::abs(item_y - y) < 0.01
      && let Some((x, width)) = item_horizontal_bounds(item, text_metrics)
    {
      min_x = min_x.min(x);
      max_x = max_x.max(x + width);
    }
  }
  if min_x == f32::MAX || max_x <= tab.stop.x_pt {
    return;
  }
  // SwTabPortion::PostFormat() sums the widths of every portion after a
  // center/right tab and then stretches the tab portion so the whole following
  // run, not only the first word, is aligned to the tab position.
  let tab_right = if tab.stop.x_pt > line_right {
    tab.stop.x_pt
  } else {
    tab.stop.x_pt.min(line_right)
  };
  let following_width = max_x - tab.stop.x_pt;
  let aligned_left = match tab.stop.alignment {
    TabStopAlignment::Left => tab.stop.x_pt,
    TabStopAlignment::Center => tab_right - following_width / 2.0,
    TabStopAlignment::Right => tab_right - following_width,
  };
  let dx = aligned_left - min_x;
  let leader = tab.stop.leader;
  let leader_start_x = tab.leader_start_x_pt;
  let leader_y = tab.y_pt;
  let leader_line_height = tab.line_height_pt;
  let leader_style = tab.style.clone();
  let item_start_index = tab.item_start_index;
  if dx.abs() <= LAYOUT_EPSILON_PT {
    push_tab_leader(
      page,
      TabLeaderPlacement {
        insert_index: Some(item_start_index),
        leader,
        start_x: leader_start_x,
        end_x: aligned_left,
        y: leader_y,
        line_height: leader_line_height,
      },
      &leader_style,
      text_metrics,
    );
    *pending_tab = None;
    return;
  }

  for item in page.items.iter_mut().skip(item_start_index) {
    if let Some(item_y) = item_y(item)
      && f32::abs(item_y - y) < 0.01
    {
      shift_item_x(item, dx);
    }
  }
  push_tab_leader(
    page,
    TabLeaderPlacement {
      insert_index: Some(item_start_index),
      leader,
      start_x: leader_start_x,
      end_x: aligned_left,
      y: leader_y,
      line_height: leader_line_height,
    },
    &leader_style,
    text_metrics,
  );
  *pending_tab = None;
}

#[derive(Clone, Copy, Debug)]
struct TabLeaderPlacement {
  insert_index: Option<usize>,
  leader: TabLeader,
  start_x: f32,
  end_x: f32,
  y: f32,
  line_height: f32,
}

fn push_tab_leader(
  page: &mut Page,
  placement: TabLeaderPlacement,
  style: &TextStyle,
  text_metrics: &mut TextMetrics,
) {
  let Some(fill_char) = tab_leader_fill_char(placement.leader) else {
    return;
  };
  let width = placement.end_x - placement.start_x;
  if width <= LAYOUT_EPSILON_PT {
    return;
  }
  let char_width = text_metrics.measure_text(fill_char, style);
  if char_width <= LAYOUT_EPSILON_PT {
    return;
  }
  let count = (width / char_width).floor() as usize;
  if count == 0 {
    return;
  }
  let items = (0..count).map(|index| {
    PageItem::Text(Box::new(TextItem {
      x_pt: placement.start_x + char_width * index as f32,
      y_pt: placement.y,
      line_height_pt: placement.line_height,
      text: fill_char.to_string(),
      style: style.clone(),
      rotation_center_pt: None,
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: true,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: PdfTextSegmentation::Portion,
    }))
  });
  if let Some(index) = placement
    .insert_index
    .filter(|index| *index <= page.items.len())
  {
    page.items.splice(index..index, items);
  } else {
    page.items.extend(items);
  }
}

fn tab_leader_fill_char(leader: TabLeader) -> Option<&'static str> {
  match leader {
    TabLeader::None => None,
    TabLeader::Dot => Some("."),
    TabLeader::Hyphen => Some("-"),
    TabLeader::Underscore => Some("_"),
    TabLeader::Heavy => Some("_"),
    TabLeader::MiddleDot => Some("·"),
  }
}

fn justify_line_items(
  items: &mut [PageItem],
  start_index: usize,
  y: f32,
  line_left: f32,
  line_right: f32,
  text_metrics: &mut TextMetrics,
) {
  let mut last_text_index = None;
  for (index, item) in items.iter().enumerate().skip(start_index) {
    if item_y(item).is_none_or(|item_y| (item_y - y).abs() >= 0.01) {
      continue;
    }
    if !matches!(item, PageItem::Text(_)) {
      // Writer adjusts text portions between fixed margin portions. Images,
      // tabs, and drawing-layer objects need that segmented algorithm rather
      // than treating the whole line as one stretchable text run.
      return;
    }
    last_text_index = Some(index);
  }

  let mut natural_right = line_left;
  let mut space_count = 0usize;
  for (index, item) in items.iter().enumerate().skip(start_index) {
    if item_y(item).is_none_or(|item_y| (item_y - y).abs() >= 0.01) {
      continue;
    }
    let PageItem::Text(text) = item else {
      return;
    };
    // Writer excludes trailing U+0020 blanks from the printable line width
    // and from the glue slots used by block adjustment. The preserved blanks
    // remain in the text portion, but have no following ink to advance.
    let visible_text = if Some(index) == last_text_index {
      text.text.trim_end_matches(' ')
    } else {
      &text.text
    };
    natural_right =
      natural_right.max(text.x_pt + text_metrics.measure_text(visible_text, &text.style));
    space_count += visible_text.matches(' ').count();
  }
  if space_count == 0 {
    return;
  }

  // SwTextAdjuster::CalcNewBlock() divides the printable glue remaining at
  // the right margin by the number of U+0020 blanks on the line. Preserve the
  // line as coalescible text portions and carry the extra advance into glyph
  // positioning; splitting at every word changes Writer's PDF object model.
  let extra_per_space = (line_right - natural_right) / space_count as f32;
  if extra_per_space.abs() < 0.001 {
    return;
  }
  let mut preceding_space_advance = 0.0;
  for item in items.iter_mut().skip(start_index) {
    if item_y(item).is_none_or(|item_y| (item_y - y).abs() >= 0.01) {
      continue;
    }
    let PageItem::Text(text) = item else {
      return;
    };
    text.x_pt += preceding_space_advance;
    text.word_spacing_pt = extra_per_space;
    preceding_space_advance += text.text.matches(' ').count() as f32 * extra_per_space;
  }
}

fn align_text_baseline_to_inline_object(
  items: &mut [PageItem],
  start_index: usize,
  y: f32,
  text_metrics: &mut TextMetrics,
) {
  let object_baseline = items
    .iter()
    .skip(start_index)
    .filter_map(|item| match item {
      PageItem::Image(image) if !image.floating && (image.y_pt - y).abs() < 0.01 => {
        Some(image.y_pt + image.height_pt)
      }
      _ => None,
    })
    .reduce(f32::max);
  let Some(object_baseline) = object_baseline else {
    return;
  };

  // ECMA-376 Part 1 §20.4.2.3 makes an inline drawing participate in its
  // line like a character. Writer's MaxAscentDescent() therefore treats the
  // object height as ascent and keeps the ordinary font descent below it;
  // text sharing the line is bottom-aligned to the object's baseline rather
  // than centered in the enlarged line box.
  for item in items.iter_mut().skip(start_index) {
    let PageItem::Text(text) = item else {
      continue;
    };
    if (text.y_pt - y).abs() >= 0.01 {
      continue;
    }
    let baseline_offset = if text.style.use_windows_font_metrics {
      text_metrics.baseline_offset_in_line_with_windows_metrics_for_text(
        &text.text,
        &text.style,
        text.line_height_pt,
      )
    } else {
      text_metrics.baseline_offset_in_line_for_text(&text.text, &text.style, text.line_height_pt)
    };
    let current_baseline = text.y_pt + baseline_offset;
    if object_baseline > current_baseline {
      text.y_pt += object_baseline - current_baseline;
    }
  }
}

fn finalize_cjk_punctuation_compression(
  items: &mut [PageItem],
  start_index: usize,
  y: f32,
  line_right: f32,
  text_metrics: &mut TextMetrics,
) {
  let mut compressed_right = f32::NEG_INFINITY;
  let mut total_capacity = 0.0;
  for item in items.iter().skip(start_index) {
    let PageItem::Text(text) = item else {
      continue;
    };
    if (text.y_pt - y).abs() >= 0.01 || text.style.cjk_punctuation_compression_ratio <= 0.0 {
      continue;
    }
    let mut natural_style = text.style.clone();
    natural_style.cjk_punctuation_compression_ratio = 0.0;
    let mut maximum_style = text.style.clone();
    maximum_style.cjk_punctuation_compression_ratio = 1.0;
    let natural_width = text_metrics.measure_text(&text.text, &natural_style);
    let maximum_width = text_metrics.measure_text(&text.text, &maximum_style);
    total_capacity += (natural_width - maximum_width).max(0.0);
    compressed_right = compressed_right.max(text.x_pt + maximum_width);
  }
  if total_capacity <= 0.001 || !compressed_right.is_finite() {
    return;
  }

  // Word exposes the maximum removable punctuation side-bearing while it
  // chooses a line break, then gives unused capacity back to the line. This
  // is the same fit/decompress sequence described by the Office
  // lineWrapLikeWord6 note and implemented by Writer's CalcKanaAdj.
  let natural_right = compressed_right + total_capacity;
  let required_compression = (natural_right - line_right).clamp(0.0, total_capacity);
  let ratio = required_compression / total_capacity;
  let expansion_ratio = 1.0 - ratio;
  let mut preceding_expansion = 0.0;
  for item in items.iter_mut().skip(start_index) {
    let PageItem::Text(text) = item else {
      continue;
    };
    if (text.y_pt - y).abs() >= 0.01 || text.style.cjk_punctuation_compression_ratio <= 0.0 {
      continue;
    }
    let mut natural_style = text.style.clone();
    natural_style.cjk_punctuation_compression_ratio = 0.0;
    let mut maximum_style = text.style.clone();
    maximum_style.cjk_punctuation_compression_ratio = 1.0;
    let capacity = (text_metrics.measure_text(&text.text, &natural_style)
      - text_metrics.measure_text(&text.text, &maximum_style))
    .max(0.0);
    text.x_pt += preceding_expansion;
    text.style.cjk_punctuation_compression_ratio = ratio;
    preceding_expansion += capacity * expansion_ratio;
  }
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

fn item_horizontal_bounds(item: &PageItem, text_metrics: &mut TextMetrics) -> Option<(f32, f32)> {
  match item {
    PageItem::Text(text) => Some((
      text.x_pt,
      text_metrics.measure_text(&text.text, &text.style)
        + text.text.matches(' ').count() as f32 * text.word_spacing_pt,
    )),
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

#[derive(Clone, Copy, Debug)]
struct InlineImageMetrics {
  frame_width_pt: f32,
  frame_height_pt: f32,
  content_offset_x_pt: f32,
  content_offset_y_pt: f32,
  content_width_pt: f32,
  content_height_pt: f32,
}

fn inline_image_metrics(image: &crate::docx::InlineImage, max_width_pt: f32) -> InlineImageMetrics {
  let frame_width = image_frame_width(image);
  let frame_height = image_frame_height(image);
  let (frame_width_pt, frame_height_pt) =
    fit_image_to_line(frame_width, frame_height, max_width_pt);
  let scale = if frame_width > f32::EPSILON {
    frame_width_pt / frame_width
  } else {
    0.0
  };
  InlineImageMetrics {
    frame_width_pt,
    frame_height_pt,
    content_offset_x_pt: image.effect_left_pt.max(0.0) * scale,
    content_offset_y_pt: image.effect_top_pt.max(0.0) * scale,
    content_width_pt: visible_image_width(image) * scale,
    content_height_pt: visible_image_height(image) * scale,
  }
}

fn force_page_break(
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
) -> (FlowContext, f32) {
  // transition even when the current page has no painted objects yet; see
  // sw/qa/core/text/itrform2.cxx:testContentControlHeaderPDFExport.
  let mut next_flow = FlowContext {
    section_page_index: flow.section_page_index + 1,
    ..flow
  };
  let next_page_number = pages.len() + 2;
  let mut next_page =
    empty_section_page(flow.setup, flow.section_index, next_flow.section_page_index);
  next_page.explicit_break_target = true;
  next_page.body_content_frames = next_page.body_content_frames.saturating_add(1);
  next_page.repeating_wrap_exclusion_catalog = current.repeating_wrap_exclusion_catalog.clone();
  next_page.repeating_wrap_exclusions = next_page
    .repeating_wrap_exclusion_catalog
    .selected(next_flow.section_page_index, next_page_number)
    .to_vec();
  extend_wrap_exclusions_unique(
    &mut next_page.wrap_exclusions,
    &next_page.repeating_wrap_exclusions,
  );
  pages.push(std::mem::replace(current, next_page));
  activate_pending_floating_table_follows_for_current(current, pages);
  next_flow = body_flow_for_page(flow_with_column(next_flow, 0), pages.len() + 1);
  (next_flow, next_flow.content_top_pt)
}

fn advance_text_frame_flow(
  flow: FlowContext,
  current: &mut Page,
  pages: &mut Vec<Page>,
) -> (FlowContext, f32) {
  let (next_flow, y) = advance_section_flow(flow, current, pages);
  note_body_content_frame(current, next_flow);
  (next_flow, y)
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
  dynamic_field: Option<&'a DynamicFieldKind>,
  style_ref_keys: &'a [Arc<str>],
  style_ref_text: Option<&'a Arc<str>>,
  style_ref_numbering_text: Option<&'a Arc<str>>,
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
  meta: &TextChunkMeta<'_>,
) {
  if chunk.is_empty() {
    return;
  }

  page.items.push(PageItem::Text(Box::new(TextItem {
    x_pt: placement.x_pt,
    y_pt: placement.y_pt,
    line_height_pt: placement.line_height_pt,
    text: std::mem::take(chunk),
    style: style.clone(),
    rotation_center_pt: None,
    hyperlink_url: meta.hyperlink_url.map(ToString::to_string),
    dynamic_field: meta.dynamic_field.cloned(),
    style_ref_keys: meta.style_ref_keys.to_vec(),
    style_ref_text: meta.style_ref_text.cloned(),
    style_ref_numbering_text: meta.style_ref_numbering_text.cloned(),
    form_widget_id: meta.form_widget_id,
    paragraph_bidi: meta.paragraph_bidi,
    word_spacing_pt: 0.0,
    preserve_text_portion: meta.preserve_text_portion,
    decoration_span_start_x_pt: None,
    pdf_text_segmentation: match meta.segmentation {
      TextSegmentation::Body => PdfTextSegmentation::WordLine,
      TextSegmentation::RepeatingSlot => PdfTextSegmentation::WordLine,
      TextSegmentation::TableCell => PdfTextSegmentation::WordLine,
      TextSegmentation::DrawingLayer => PdfTextSegmentation::Portion,
      TextSegmentation::Notes => PdfTextSegmentation::WordLine,
    },
  })));
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
  fn next_page_section_does_not_reuse_explicit_page_break_target() {
    // ECMA-376 Part 1 sections 17.18.4 and 17.6.22 independently move to the
    // next page. The Office tdf168980 golden retains both transitions.
    let mut page = empty_section_page(PageSetup::default(), 0, 3);
    page.explicit_break_target = true;
    page.body_content_frames = 1;

    assert!(!can_reuse_page_for_section(
      &page,
      1,
      SectionBreakKind::NextPage,
      false,
      true,
    ));
    assert!(can_reuse_page_for_section(
      &page,
      1,
      SectionBreakKind::NextPage,
      false,
      false,
    ));

    page.explicit_break_target = false;
    assert!(can_reuse_page_for_section(
      &page,
      1,
      SectionBreakKind::NextPage,
      false,
      true,
    ));
  }

  #[test]
  fn text_moves_below_floating_frame_when_neither_side_can_hold_a_line() {
    let exclusion = WrapExclusion {
      left_pt: 40.0,
      right_pt: 460.0,
      top_pt: 72.0,
      bottom_pt: 108.0,
      side: ImageWrapSide::BothSides,
      blocks_flow: false,
    };

    assert_eq!(
      dodge_text_wrap_exclusions(72.0, 12.0, 36.0, 468.0, &[exclusion]),
      108.0
    );
    assert_eq!(
      line_bounds_for_y(36.0, 468.0, 108.0, 12.0, &[exclusion]),
      (36.0, 468.0)
    );
  }

  #[test]
  fn word_upper_margin_moves_first_line_only_when_fly_reaches_print_area() {
    let crossing = WrapExclusion {
      left_pt: 300.0,
      right_pt: 420.0,
      top_pt: 96.0,
      bottom_pt: 112.0,
      side: ImageWrapSide::BothSides,
      blocks_flow: false,
    };
    assert_eq!(
      shift_first_line_below_upper_margin_exclusions(100.0, 6.0, 72.0, 540.0, &[crossing]),
      112.0
    );

    let margin_only = WrapExclusion {
      bottom_pt: 99.0,
      ..crossing
    };
    assert_eq!(
      shift_first_line_below_upper_margin_exclusions(100.0, 6.0, 72.0, 540.0, &[margin_only]),
      100.0
    );
  }

  #[test]
  fn paragraph_indents_are_measured_from_opposite_text_margins() {
    let width = paragraph_content_width(468.0, 36.0, 18.0);
    assert_eq!(width, 414.0);
    assert_eq!(
      paragraph_line_bounds(72.0, width, 36.0, -12.0),
      (108.0, 96.0, 522.0)
    );
    let width = paragraph_content_width(468.0, -18.0, 0.0);
    assert_eq!(width, 486.0);
    assert_eq!(
      paragraph_line_bounds(72.0, width, -18.0, 0.0),
      (54.0, 54.0, 540.0)
    );
  }

  #[test]
  fn frame_wrap_distances_do_not_shift_the_specified_position() {
    let setup = PageSetup {
      width_pt: 612.0,
      height_pt: 792.0,
      margin_top_pt: 72.0,
      margin_right_pt: 72.0,
      margin_bottom_pt: 72.0,
      margin_left_pt: 72.0,
      ..Default::default()
    };
    let flow = flow_from_block_area(BlockArea {
      setup,
      section_index: 0,
      section_page_index: 0,
      column_index: 0,
      columns: SectionColumns::default(),
      content_top_pt: 72.0,
      content_left_pt: 72.0,
      content_bottom: 720.0,
      body_content_bottom_pt: 720.0,
      content_width: 468.0,
      default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
      compatibility_mode: 15,
      justify_lines_with_shrinking: false,
      repeating_slots: RepeatingSlotState::default(),
    });
    let placement = FloatingFramePlacement {
      horizontal_anchor: FrameHorizontalAnchor::Page,
      vertical_anchor: FrameVerticalAnchor::Page,
      horizontal_offset_pt: 80.0,
      vertical_offset_pt: 90.0,
      margin_left_pt: 9.0,
      margin_top_pt: 11.0,
      ..Default::default()
    };

    assert_eq!(
      floating_table_position(placement, flow, 72.0, 144.0),
      (80.0, 90.0)
    );
    assert_eq!(
      paragraph_frame_position(placement, flow, 72.0, 144.0, 36.0),
      (80.0, 90.0)
    );
  }

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
        shadow: false,
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
        shadow: false,
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
  fn paragraph_line_spacing_excess_matches_writer_compat_extra() {
    let mut paragraph = Paragraph {
      inlines: vec![InlineItem::Text(TextRun {
        text: "double spaced".into(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
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
      style_ref_numbering_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    };

    assert!((paragraph_line_spacing_excess(&paragraph) - 12.65).abs() < 0.01);
    paragraph.format.line_height_pt = Some(1.0);
    assert_eq!(paragraph_line_spacing_excess(&paragraph), 0.0);
  }

  #[test]
  fn table_cell_height_collapses_adjacent_paragraph_spacing() {
    fn table_cell_flow() -> FlowContext {
      FlowContext {
        setup: PageSetup::default(),
        section_index: 0,
        section_page_index: 0,
        column_index: 0,
        columns: SectionColumns::default(),
        content_top_pt: 0.0,
        content_left_pt: 0.0,
        content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
        body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
        content_width: 200.0,
        layout_cell_bounds: None,
        layout_cell_print_bounds: None,
        default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
        compatibility_mode: 12,
        justify_lines_with_shrinking: false,
        split_page_break_and_paragraph_mark: false,
        repeating_slots: RepeatingSlotState::default(),
        text_segmentation: TextSegmentation::TableCell,
        paragraph_spacing_context: ParagraphSpacingContext::Normal,
        preserve_horizontal_on_advance: false,
        script_sensitive_line_height: true,
      }
    }

    fn paragraph(text: &str, style_id: &str, before: f32, after: f32) -> Paragraph {
      Paragraph {
        inlines: vec![InlineItem::Text(TextRun {
          text: text.into(),
          style: TextStyle::default(),
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          preserve_text_portion: false,
        })],
        footnote_reference_ids: Vec::new(),
        endnote_reference_ids: Vec::new(),
        starts_after_last_rendered_page_break: false,
        base_style: TextStyle::default(),
        #[cfg(test)]
        runs: Vec::new(),
        format: Box::new(ParagraphFormat {
          style_id: Some(style_id.into()),
          spacing_before_pt: before,
          spacing_after_pt: after,
          ..Default::default()
        }),
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        list_label: None,
        list_label_style: TextStyle::default(),
        list_label_hyperlink_url: None,
        list_label_tab_stop_pt: None,
      }
    }

    let first = paragraph("YN", "Initials", 0.0, 74.0);
    let second = paragraph("OBJECTIVE", "Heading3", 30.0, 8.0);
    let flow = table_cell_flow();
    let mut text_metrics = TextMetrics::new();
    let first_content = estimated_paragraph_content_height(&first, flow, &mut text_metrics);
    let second_content = estimated_paragraph_content_height(&second, flow, &mut text_metrics);
    let blocks = vec![Block::paragraph(first), Block::paragraph(second)];

    let height = table_cell_blocks_content_height(
      &blocks,
      flow,
      TableCellMeasureMode::WholeCell,
      &mut text_metrics,
    );
    let expected = first_content + 74.0 + second_content + 8.0;
    let naive_uncollapsed = expected + 30.0;

    assert!((height - expected).abs() < 0.01);
    assert!((height - naive_uncollapsed).abs() > 20.0);
  }

  #[test]
  fn table_cell_follow_measurement_omits_master_paragraph_spacing() {
    fn table_cell_flow() -> FlowContext {
      FlowContext {
        setup: PageSetup::default(),
        section_index: 0,
        section_page_index: 0,
        column_index: 0,
        columns: SectionColumns::default(),
        content_top_pt: 0.0,
        content_left_pt: 0.0,
        content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
        body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
        content_width: 200.0,
        layout_cell_bounds: None,
        layout_cell_print_bounds: None,
        default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
        compatibility_mode: 12,
        justify_lines_with_shrinking: false,
        split_page_break_and_paragraph_mark: false,
        repeating_slots: RepeatingSlotState::default(),
        text_segmentation: TextSegmentation::TableCell,
        paragraph_spacing_context: ParagraphSpacingContext::Normal,
        preserve_horizontal_on_advance: false,
        script_sensitive_line_height: true,
      }
    }

    let paragraph = Paragraph {
      inlines: vec![InlineItem::Text(TextRun {
        text: "follow".into(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      })],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: TextStyle::default(),
      #[cfg(test)]
      runs: Vec::new(),
      format: Box::new(ParagraphFormat {
        spacing_before_pt: 24.0,
        spacing_after_pt: 18.0,
        ..Default::default()
      }),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    };
    let flow = table_cell_flow();
    let mut text_metrics = TextMetrics::new();
    let content = estimated_paragraph_content_height(&paragraph, flow, &mut text_metrics);
    let blocks = vec![Block::paragraph(paragraph)];

    let whole = table_cell_blocks_content_height(
      &blocks,
      flow,
      TableCellMeasureMode::WholeCell,
      &mut text_metrics,
    );
    let follow = table_cell_blocks_content_height(
      &blocks,
      flow,
      TableCellMeasureMode::LastRenderedFollow,
      &mut text_metrics,
    );

    assert!((whole - (content + 24.0 + 18.0)).abs() < 0.01);
    assert!((follow - content).abs() < 0.01);
  }

  #[test]
  fn table_cell_empty_paragraph_min_height_applies_to_whole_frame() {
    let flow = FlowContext {
      setup: PageSetup::default(),
      section_index: 0,
      section_page_index: 0,
      column_index: 0,
      columns: SectionColumns::default(),
      content_top_pt: 0.0,
      content_left_pt: 0.0,
      content_bottom: UNBOUNDED_LAYOUT_EXTENT_PT,
      body_content_bottom_pt: UNBOUNDED_LAYOUT_EXTENT_PT,
      content_width: 200.0,
      layout_cell_bounds: None,
      layout_cell_print_bounds: None,
      default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
      compatibility_mode: 12,
      justify_lines_with_shrinking: false,
      split_page_break_and_paragraph_mark: false,
      repeating_slots: RepeatingSlotState::default(),
      text_segmentation: TextSegmentation::TableCell,
      paragraph_spacing_context: ParagraphSpacingContext::Normal,
      preserve_horizontal_on_advance: false,
      script_sensitive_line_height: true,
    };
    let paragraph = Paragraph {
      inlines: Vec::new(),
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: TextStyle::default(),
      #[cfg(test)]
      runs: Vec::new(),
      format: Box::new(ParagraphFormat {
        spacing_before_pt: 2.0,
        spacing_after_pt: 3.0,
        ..Default::default()
      }),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    };
    let mut text_metrics = TextMetrics::new();
    let min_height = paragraph_line_height_for_setup(
      &paragraph,
      &paragraph_base_line_style(&paragraph),
      flow.setup,
      flow.text_segmentation,
      &mut text_metrics,
    );
    let blocks = vec![Block::paragraph(paragraph)];

    let height = table_cell_blocks_content_height(
      &blocks,
      flow,
      TableCellMeasureMode::WholeCell,
      &mut text_metrics,
    );

    assert!((height - min_height).abs() < 0.01);
  }

  #[test]
  fn placeholder_floating_line_height_matches_writer_flow_height() {
    let placeholder_run = TextRun {
      text: "YN".into(),
      style: TextStyle {
        font_size_pt: 55.0,
        ..Default::default()
      },
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: true,
    };
    let floating_shape = InlineShape {
      width_pt: 100.0,
      height_pt: 20.0,
      effect_left_pt: 0.0,
      effect_top_pt: 0.0,
      effect_right_pt: 0.0,
      effect_bottom_pt: 0.0,
      geometry: InlineShapeGeometry::Rectangle,
      offset_x_pt: 0.0,
      offset_y_pt: 0.0,
      fill_color: None,
      additional_fill_colors: Vec::new(),
      fill_image: None,
      stroke: None,
      suppress_zero_relative_background: false,
      allow_outside_page: false,
      inline_anchor_after_line: false,
      placement: crate::docx::ImagePlacement::Floating(FloatingImagePlacement {
        horizontal_relative_to: HorizontalImageReference::Column,
        vertical_relative_to: VerticalImageReference::Page,
        horizontal_alignment: None,
        vertical_alignment: None,
        horizontal_offset_pt: 0.0,
        vertical_offset_pt: 0.0,
        wrap: ImageWrapMode::Through,
        wrap_side: ImageWrapSide::BothSides,
        behind_text: true,
        layout_in_cell: true,
        allow_overlap: true,
        relative_height: 0,
        relative_width_to: None,
        relative_width_pct: None,
        relative_height_to: None,
        relative_height_pct: None,
        margin_top_pt: 0.0,
        margin_right_pt: 0.0,
        margin_bottom_pt: 0.0,
        margin_left_pt: 0.0,
      }),
      chart: None,
      text_box_blocks: Vec::new(),
      text_inset_left_pt: 0.0,
      text_inset_top_pt: 0.0,
      text_inset_right_pt: 0.0,
      text_inset_bottom_pt: 0.0,
      text_box_auto_fit: false,
      text_vertical_alignment: TextBoxVerticalAlignment::Top,
    };
    let mut paragraph = Paragraph {
      inlines: vec![
        InlineItem::Text(placeholder_run),
        InlineItem::Shape(floating_shape),
      ],
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: TextStyle::default(),
      #[cfg(test)]
      runs: Vec::new(),
      format: Box::new(ParagraphFormat::default()),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    };

    assert!((line_real_height(&paragraph, 63.25, 0, None, false) - 26.62).abs() < 0.02);
    if let InlineItem::Text(run) = &mut paragraph.inlines[0] {
      run.preserve_text_portion = false;
    }
    assert_eq!(line_real_height(&paragraph, 63.25, 0, None, false), 63.25);
  }

  #[test]
  fn grid_auto_line_spacing_expands_only_after_first_line() {
    let paragraph = Paragraph {
      inlines: Vec::new(),
      footnote_reference_ids: Vec::new(),
      endnote_reference_ids: Vec::new(),
      starts_after_last_rendered_page_break: false,
      base_style: TextStyle::default(),
      #[cfg(test)]
      runs: Vec::new(),
      format: Box::new(ParagraphFormat {
        line_height_rule: LineHeightRule::Auto,
        line_height_pt: Some(1.35),
        snap_to_grid: Some(true),
        ..Default::default()
      }),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    };
    let setup = PageSetup {
      doc_grid_line_pitch_pt: Some(15.6),
      ..Default::default()
    };
    let multiple = grid_auto_line_spacing_multiplier(&paragraph, setup, TextSegmentation::Body);

    assert_eq!(multiple, Some(1.35));
    assert_eq!(line_real_height(&paragraph, 15.6, 0, multiple, false), 15.6);
    assert!((line_real_height(&paragraph, 15.6, 1, multiple, false) - 21.06).abs() < 0.01);

    assert_eq!(
      grid_auto_line_spacing_multiplier(&paragraph, setup, TextSegmentation::TableCell),
      None
    );
    let table_grid_setup = PageSetup {
      adjust_table_line_heights_to_grid: true,
      ..setup
    };
    assert_eq!(
      grid_auto_line_spacing_multiplier(&paragraph, table_grid_setup, TextSegmentation::TableCell),
      Some(1.35)
    );

    let mut below_single = paragraph;
    below_single.format.line_height_pt = Some(0.75);
    let multiple = grid_auto_line_spacing_multiplier(&below_single, setup, TextSegmentation::Body);
    assert_eq!(multiple, Some(1.0));
    assert_eq!(
      line_real_height(&below_single, 15.6, 1, multiple, false),
      15.6
    );
  }

  #[test]
  fn body_limits_use_imported_header_footer_slots() {
    let setup = PageSetup {
      margin_top_pt: 56.7,
      margin_bottom_pt: 62.35,
      header_distance_pt: 28.35,
      footer_distance_pt: 49.55,
      ..Default::default()
    };
    let slots = RepeatingSlotState {
      default_header: true,
      default_footer: true,
      default_header_height_pt: 20.0,
      default_footer_height_pt: 5.0,
      ..Default::default()
    };

    let (top, bottom) = body_content_limits_for_page(setup, slots, 1, 0);

    assert_eq!(top, 56.7);
    assert!((bottom - (setup.height_pt - 62.35)).abs() < 0.01);
  }

  #[test]
  fn body_limits_do_not_center_the_word_document_grid() {
    let setup = PageSetup {
      margin_top_pt: 72.0,
      margin_bottom_pt: 72.0,
      doc_grid_line_pitch_pt: Some(15.6),
      ..Default::default()
    };

    let (top, bottom) = body_content_limits_for_page(setup, RepeatingSlotState::default(), 1, 0);

    assert_eq!(top, 72.0);
    assert!((bottom - (setup.height_pt - 72.0)).abs() < 0.01);
  }

  #[test]
  fn body_limits_grow_for_layout_affecting_repeating_content() {
    let setup = PageSetup {
      margin_top_pt: 56.7,
      margin_bottom_pt: 62.35,
      header_distance_pt: 28.35,
      footer_distance_pt: 49.55,
      ..Default::default()
    };
    let slots = RepeatingSlotState {
      default_header: true,
      default_footer: true,
      default_header_height_pt: 40.0,
      default_footer_height_pt: 30.0,
      ..Default::default()
    };

    let (top, bottom) = body_content_limits_for_page(setup, slots, 1, 0);

    assert!((top - (setup.header_distance_pt + 40.0)).abs() < 0.01);
    let footer_bottom = setup.height_pt - setup.footer_distance_pt;
    assert!((bottom - (footer_bottom - 30.0)).abs() < 0.01);
  }

  #[test]
  fn footer_content_is_centered_in_the_nominal_slot_and_overflow_grows_upward() {
    let setup = PageSetup {
      height_pt: 792.0,
      margin_bottom_pt: 72.0,
      footer_distance_pt: 36.0,
      ..Default::default()
    };

    assert!((footer_slot_top(setup) - 720.0).abs() < 0.01);
    assert!((footer_content_top(setup, 12.0, true) - 732.0).abs() < 0.01);
    assert!((footer_content_top(setup, 12.0, false) - 744.0).abs() < 0.01);
    assert!((footer_content_top(setup, 48.0, true) - 708.0).abs() < 0.01);
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
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    };
    let blocks = vec![Block::paragraph(Paragraph {
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
      style_ref_numbering_text: None,
      list_label: None,
      list_label_style: TextStyle::default(),
      list_label_hyperlink_url: None,
      list_label_tab_stop_pt: None,
    })];

    let mut text_metrics = TextMetrics::new();
    let measured = measured_note_blocks_height(&blocks, flow, &mut text_metrics);
    let estimated = blocks
      .iter()
      .map(|block| estimated_block_height(block, flow, &mut text_metrics))
      .sum::<f32>();

    assert!(measured > DEFAULT_LINE_HEIGHT_PT);
    assert!((measured - estimated).abs() < DEFAULT_LINE_HEIGHT_PT * 2.0);
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
  fn auto_width_table_keeps_explicit_grid_width_beyond_text_area() {
    let table = Table {
      column_widths_pt: vec![231.0, 231.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: TableAlignment::Left,
      align_leading_cell_content: true,
      placement: None,
      split_allowed: false,
      following_text_flow: false,
      explicit_no_repeat_header: false,
      starts_after_last_rendered_page_break: false,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: Vec::new(),
    };

    assert_eq!(table_column_widths(&table, 2, 451.0, false), [231.0, 231.0]);
  }

  #[test]
  fn minimal_gridless_table_with_leading_cell_breaks_creates_row_frame() {
    fn paragraph(text: &str) -> Block {
      let run = TextRun {
        text: text.to_string(),
        style: TextStyle::default(),
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        preserve_text_portion: false,
      };
      Block::paragraph(Paragraph {
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
        style_ref_numbering_text: None,
        list_label: None,
        list_label_style: TextStyle::default(),
        list_label_hyperlink_url: None,
        list_label_tab_stop_pt: None,
      })
    }

    let table = Table {
      column_widths_pt: Vec::new(),
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: TableAlignment::Left,
      align_leading_cell_content: true,
      placement: None,
      split_allowed: false,
      following_text_flow: false,
      explicit_no_repeat_header: false,
      starts_after_last_rendered_page_break: false,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![TableRow {
        cells: vec![TableCell {
          blocks: vec![paragraph("\n"), paragraph("\n"), paragraph("\ncell text")],
          shading: None,
          borders: CellBordersModel::default(),
          margins: CellMargins::default(),
          preferred_width_pt: None,
          preferred_width_pct: None,
          grid_span: 1,
          vertical_merge_continue: false,
          vertical_alignment: TableCellVerticalAlignment::Top,
          text_rotation_deg: None,
        }],
        height_pt: None,
        exact_height: false,
        repeat_header: false,
        keep_with_next: false,
        cant_split: false,
        cell_spacing_pt: None,
        grid_before: 0,
        grid_after: 0,
        redline_color: None,
      }],
    };
    let area = BlockArea {
      setup: PageSetup::default(),
      section_index: 0,
      section_page_index: 0,
      column_index: 0,
      columns: SectionColumns::default(),
      content_top_pt: 72.0,
      content_left_pt: 72.0,
      content_bottom: 720.0,
      body_content_bottom_pt: 720.0,
      content_width: 451.0,
      default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
      compatibility_mode: 12,
      justify_lines_with_shrinking: false,
      repeating_slots: RepeatingSlotState::default(),
    };
    let mut text_metrics = TextMetrics::new();
    let layout =
      TableFrameLayout::new(&table, area, false, &mut text_metrics).expect("gridless table frame");
    let mut page = empty_page(area.setup, 0);
    let mut pages = Vec::new();
    let (_, bottom) = layout.format(
      &mut page,
      &mut pages,
      &mut text_metrics,
      area.content_top_pt,
      true,
    );

    assert!(bottom > area.content_top_pt);
    assert!(
      page
        .frame_fragments
        .iter()
        .any(|fragment| fragment.kind == FrameFragmentKind::TableRow)
    );
    assert!(
      page
        .items
        .iter()
        .any(|item| { matches!(item, PageItem::Text(text) if text.text.contains("cell text")) })
    );
  }

  #[test]
  fn table_follow_reuses_master_row_metrics_when_page_geometry_is_unchanged() {
    let table = Table {
      column_widths_pt: vec![72.0],
      preferred_width_pt: None,
      preferred_width_pct: None,
      indent_left_pt: 0.0,
      alignment: TableAlignment::Left,
      align_leading_cell_content: true,
      placement: None,
      split_allowed: true,
      following_text_flow: false,
      explicit_no_repeat_header: false,
      starts_after_last_rendered_page_break: false,
      borders: None,
      cell_spacing_pt: 0.0,
      rows: vec![TableRow {
        cells: vec![TableCell {
          blocks: Vec::new(),
          shading: None,
          borders: CellBordersModel::default(),
          margins: CellMargins::default(),
          preferred_width_pt: None,
          preferred_width_pct: None,
          grid_span: 1,
          vertical_merge_continue: false,
          vertical_alignment: TableCellVerticalAlignment::Top,
          text_rotation_deg: None,
        }],
        height_pt: Some(12.0),
        exact_height: true,
        repeat_header: false,
        keep_with_next: false,
        cant_split: false,
        cell_spacing_pt: None,
        grid_before: 0,
        grid_after: 0,
        redline_color: None,
      }],
    };
    let area = BlockArea {
      setup: PageSetup::default(),
      section_index: 0,
      section_page_index: 0,
      column_index: 0,
      columns: SectionColumns::default(),
      content_top_pt: 72.0,
      content_left_pt: 72.0,
      content_bottom: 720.0,
      body_content_bottom_pt: 720.0,
      content_width: 451.0,
      default_tab_stop_pt: DEFAULT_TAB_STOP_PT,
      compatibility_mode: 12,
      justify_lines_with_shrinking: false,
      repeating_slots: RepeatingSlotState::default(),
    };
    let mut text_metrics = TextMetrics::new();
    let master = TableFrameLayout::new(&table, area, false, &mut text_metrics).unwrap();
    let follow_area = BlockArea {
      section_page_index: 1,
      content_left_pt: 80.0,
      ..area
    };

    let follow = master
      .layout_for_flow(flow_from_block_area(follow_area), false, &mut text_metrics)
      .unwrap();

    assert!(Arc::ptr_eq(
      &master.frame.row_heights,
      &follow.frame.row_heights
    ));
    assert_eq!(follow.frame.left_pt - master.frame.left_pt, 8.0);
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
        text_rotation_deg: None,
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
      align_leading_cell_content: true,
      placement: None,
      split_allowed: false,
      following_text_flow: false,
      explicit_no_repeat_header: false,
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
        text_rotation_deg: None,
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
      align_leading_cell_content: true,
      placement: None,
      split_allowed: false,
      following_text_flow: false,
      explicit_no_repeat_header: false,
      starts_after_last_rendered_page_break: false,
      borders: None,
      cell_spacing_pt: 2.0,
      rows: vec![
        row(false, 10.0, Some(3.0)),
        row(true, 11.0, None),
        row(true, 12.0, None),
      ],
    };

    let mut text_metrics = TextMetrics::new();
    assert_eq!(
      vertical_merge_content_height(
        &table,
        &[72.0],
        VerticalMergeSpan {
          row_index: 0,
          grid_start: 0,
          current_row_height: 10.0,
        },
        PageSetup::default(),
        12,
        &mut text_metrics
      ),
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
      TextSplitDecision::RebalanceWidow { page_index: 0 }
    );
  }

  #[test]
  fn justified_line_shrink_counts_separator_before_candidate_word() {
    let mut text_metrics = TextMetrics::new();
    let style = TextStyle::default();
    let candidate = "gamma ";
    let candidate_width = line_fit_width(candidate, &style, &mut text_metrics);
    let space_width = text_metrics.measure_text(" ", &style);
    let line_right = 100.0;
    let overflow = space_width * 0.4;

    assert!(line_fits_with_word_space_shrink(
      LineShrinkFit {
        x: line_right - candidate_width + overflow,
        width: candidate_width,
        text: candidate,
        line_right,
        items: &[],
        item_start: 0,
        chunk: "alpha beta ",
        style: &style,
        minimum_word_spacing_pct: 75,
      },
      &mut text_metrics,
    ));
  }

  #[test]
  fn justified_line_distributes_right_margin_glue_across_word_spaces() {
    fn text_item(x_pt: f32, text: &str) -> PageItem {
      PageItem::Text(Box::new(TextItem {
        x_pt,
        y_pt: 20.0,
        line_height_pt: DEFAULT_LINE_HEIGHT_PT,
        text: text.to_string(),
        style: TextStyle::default(),
        rotation_center_pt: None,
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        form_widget_id: None,
        paragraph_bidi: false,
        word_spacing_pt: 0.0,
        preserve_text_portion: false,
        decoration_span_start_x_pt: None,
        pdf_text_segmentation: PdfTextSegmentation::Line,
      }))
    }

    let mut text_metrics = TextMetrics::new();
    let style = TextStyle::default();
    let first_width = text_metrics.measure_text("alpha ", &style);
    let second_width = text_metrics.measure_text("beta gamma", &style);
    let natural_right = first_width + second_width;
    let line_right = natural_right + 12.0;
    let mut items = vec![
      text_item(0.0, "alpha "),
      text_item(first_width, "beta gamma "),
    ];

    justify_line_items(&mut items, 0, 20.0, 0.0, line_right, &mut text_metrics);

    let PageItem::Text(first) = &items[0] else {
      panic!("expected text item");
    };
    let PageItem::Text(second) = &items[1] else {
      panic!("expected text item");
    };
    assert!((first.word_spacing_pt - 6.0).abs() < 0.001);
    assert!((second.word_spacing_pt - 6.0).abs() < 0.001);
    assert!((second.x_pt - (first_width + 6.0)).abs() < 0.001);
    assert!((second.x_pt + second_width + second.word_spacing_pt - line_right).abs() < 0.001);
  }

  #[test]
  fn text_on_inline_object_line_uses_object_bottom_as_baseline() {
    let style = TextStyle::default();
    let line_height = 124.0;
    let mut items = vec![
      PageItem::Image(ImageItem {
        x_pt: 0.0,
        y_pt: 0.0,
        width_pt: 120.0,
        height_pt: 120.0,
        crop: ImageCrop::default(),
        rotation_deg: 0.0,
        flip_horizontal: false,
        flip_vertical: false,
        data: Arc::from([]),
        content_type: None,
        alt_text: None,
        hyperlink_url: None,
        semantic_metafile_text: false,
        floating: false,
        behind_text: false,
      }),
      PageItem::Text(Box::new(TextItem {
        x_pt: 120.0,
        y_pt: 0.0,
        line_height_pt: line_height,
        text: "label".to_string(),
        style: style.clone(),
        rotation_center_pt: None,
        hyperlink_url: None,
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        form_widget_id: None,
        paragraph_bidi: false,
        word_spacing_pt: 0.0,
        preserve_text_portion: false,
        decoration_span_start_x_pt: None,
        pdf_text_segmentation: PdfTextSegmentation::Line,
      })),
    ];
    let mut text_metrics = TextMetrics::new();

    align_text_baseline_to_inline_object(&mut items, 0, 0.0, &mut text_metrics);

    let PageItem::Text(text) = &items[1] else {
      panic!("expected text item");
    };
    let baseline = text.y_pt + text_metrics.baseline_offset_in_line(&style, line_height);
    assert!((baseline - 120.0).abs() < 0.001);
  }

  #[test]
  fn inline_object_advances_only_from_an_occupied_overflowing_frame() {
    assert!(inline_object_requires_flow_advance(
      600.0, 160.0, 720.0, true
    ));
    assert!(!inline_object_requires_flow_advance(
      600.0, 120.0, 720.0, true
    ));
    assert!(!inline_object_requires_flow_advance(
      72.0, 700.0, 720.0, false
    ));
  }

  #[test]
  fn pageref_dynamic_field_resolves_anchor_virtual_page_number() {
    let mut page = empty_page(
      PageSetup {
        page_number_start: Some(7),
        ..Default::default()
      },
      0,
    );
    page.items.push(PageItem::Text(Box::new(TextItem {
      x_pt: 0.0,
      y_pt: 0.0,
      line_height_pt: DEFAULT_LINE_HEIGHT_PT,
      text: "1".to_string(),
      style: TextStyle::default(),
      rotation_center_pt: None,
      hyperlink_url: None,
      dynamic_field: Some(DynamicFieldKind::PageRef {
        bookmark_name: Arc::<str>::from("_Toc123"),
      }),
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      form_widget_id: None,
      paragraph_bidi: false,
      word_spacing_pt: 0.0,
      preserve_text_portion: false,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: PdfTextSegmentation::Line,
    })));
    let anchors = vec![AnchorPage {
      name: "_Toc123".to_string(),
      page_index: 2,
      section_index: 0,
      section_page_index: 2,
      physical_page_number: 3,
      virtual_page_number: 9,
    }];
    let mut pages = vec![page];

    resolve_dynamic_fields(&mut pages, &anchors, &[], &[]);

    let PageItem::Text(text) = &pages[0].items[0] else {
      panic!("expected text item");
    };
    assert_eq!(text.text, "9");
  }

  #[test]
  fn bidi_line_items_follow_visual_run_order_without_reversing_source_text() {
    let mut text_metrics = TextMetrics::new();
    let style = TextStyle::default();
    let mut x = 0.0;
    let mut items = ["ارمان", "(", "ArmanAg", ")"]
      .into_iter()
      .map(|value| {
        let item = PageItem::Text(Box::new(TextItem {
          x_pt: x,
          y_pt: 20.0,
          line_height_pt: DEFAULT_LINE_HEIGHT_PT,
          text: value.to_string(),
          style: style.clone(),
          rotation_center_pt: None,
          hyperlink_url: None,
          dynamic_field: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          form_widget_id: None,
          paragraph_bidi: true,
          word_spacing_pt: 0.0,
          preserve_text_portion: false,
          decoration_span_start_x_pt: None,
          pdf_text_segmentation: PdfTextSegmentation::Line,
        }));
        x += text_metrics.measure_text(value, &style);
        item
      })
      .collect::<Vec<_>>();

    reorder_bidi_line_items(&mut items, 0, 20.0, &mut text_metrics);

    let x = |index: usize| {
      let PageItem::Text(text) = &items[index] else {
        panic!("expected text");
      };
      text.x_pt
    };
    assert!(x(3) < x(2));
    assert!(x(2) < x(1));
    assert!(x(1) < x(0));
    let PageItem::Text(arabic) = &items[0] else {
      panic!("expected Arabic text");
    };
    assert_eq!(arabic.text, "ارمان");
  }

  #[test]
  fn footnote_numbering_restarts_on_each_laid_out_page() {
    fn note_text(url: &str, text: &str) -> PageItem {
      PageItem::Text(Box::new(TextItem {
        x_pt: 0.0,
        y_pt: 0.0,
        line_height_pt: DEFAULT_LINE_HEIGHT_PT,
        text: text.to_string(),
        style: TextStyle::default(),
        rotation_center_pt: None,
        hyperlink_url: Some(url.to_string()),
        dynamic_field: None,
        style_ref_keys: Vec::new(),
        style_ref_text: None,
        style_ref_numbering_text: None,
        form_widget_id: None,
        paragraph_bidi: false,
        word_spacing_pt: 0.0,
        preserve_text_portion: false,
        decoration_span_start_x_pt: None,
        pdf_text_segmentation: PdfTextSegmentation::Line,
      }))
    }

    let mut first = empty_page(PageSetup::default(), 0);
    first.items = vec![
      note_text("ooxmlsdk-pdf:footnote-reference:11", "8"),
      note_text("ooxmlsdk-pdf:footnote-reference:12", "9"),
      note_text("ooxmlsdk-pdf:footnote-backlink:11", "8 "),
      note_text("ooxmlsdk-pdf:footnote-backlink:12", "9 "),
    ];
    let mut second = empty_page(PageSetup::default(), 0);
    second.items = vec![
      note_text("ooxmlsdk-pdf:footnote-reference:13", "10"),
      note_text("ooxmlsdk-pdf:footnote-backlink:13", "10 "),
    ];
    let mut pages = vec![first, second];
    let numbering = [NoteNumberingSpec {
      format: w::NumberFormatValues::Decimal,
      start: 1,
      restart: w::RestartNumberValues::EachPage,
    }];

    resolve_page_note_numbering(&mut pages, &numbering, &[]);

    let text = |page_index: usize, item_index: usize| {
      let PageItem::Text(text) = &pages[page_index].items[item_index] else {
        panic!("expected note text");
      };
      text.text.as_str()
    };
    assert_eq!(text(0, 0), "1");
    assert_eq!(text(0, 1), "2");
    assert_eq!(text(0, 2), "1 ");
    assert_eq!(text(0, 3), "2 ");
    assert_eq!(text(1, 0), "1");
    assert_eq!(text(1, 1), "1 ");
  }

  #[test]
  fn empty_even_repeating_slot_does_not_fall_back_to_default() {
    assert_eq!(
      selected_repeating_slot(false, true, false, (false, 0.0), (false, 0.0), (true, 12.0),),
      (false, 0.0, false)
    );
  }

  #[test]
  fn empty_first_repeating_slot_does_not_fall_back_to_default() {
    assert_eq!(
      selected_repeating_slot(true, false, true, (false, 0.0), (false, 0.0), (true, 12.0),),
      (false, 0.0, true)
    );
  }
}
