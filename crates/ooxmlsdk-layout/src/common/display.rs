use std::borrow::Cow;
use std::sync::Arc;

use ooxmlsdk_fonts::{FontId, ShapedGlyph, ShapedRun};

use crate::common::{Color, Fill, Insets, Point, Pt, Rect, Size, Stroke, Transform};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutDocument<'doc> {
  pub engine_kind: LayoutEngineKind,
  pub options: LayoutOptions,
  pub pages: Vec<DisplayPage<'doc>>,
  pub form_widgets: Vec<FormWidget<'doc>>,
  pub frames: Vec<FrameRecord<'doc>>,
  pub follows: Vec<FrameFollow>,
  pub outline_entries: Vec<OutlineEntry<'doc>>,
  pub anchor_pages: Vec<AnchorPage<'doc>>,
  pub reflow: ReflowDiagnostics<'doc>,
  pub debug_records: Vec<crate::common::DebugRecord<'doc>>,
  pub unsupported: Vec<UnsupportedLayoutFeature<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LayoutEngineKind {
  #[default]
  Docx,
  Xlsx,
  Pptx,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LayoutOptions {
  pub collect_debug: bool,
  pub approximate_unsupported: bool,
  pub preserve_source_links: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorPage<'doc> {
  pub name: Cow<'doc, str>,
  pub page_index: usize,
  pub section_index: usize,
  pub section_page_index: usize,
  pub physical_page_number: usize,
  pub virtual_page_number: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisplayDocument<'doc> {
  pub pages: Vec<DisplayPage<'doc>>,
  pub resources: DisplayResources<'doc>,
  pub outlines: Vec<OutlineItem<'doc>>,
  pub links: Vec<LinkArea<'doc>>,
  pub accessibility_hints: Vec<AccessibilityHint<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisplayPage<'doc> {
  pub name: Option<Cow<'doc, str>>,
  pub section_index: usize,
  pub section_page_index: usize,
  pub setup: PageSetup,
  pub bounds: Rect,
  pub background: Option<Fill<'doc>>,
  pub items: Vec<DisplayItem<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DisplayItem<'doc> {
  Text(TextRun<'doc>),
  Glyphs(GlyphRun<'doc>),
  Image(ImageItem<'doc>),
  Path(PathItem<'doc>),
  Rect(RectItem<'doc>),
  Line(LineItem<'doc>),
  LinkArea(LinkArea<'doc>),
  AnnotationHint(AnnotationHint<'doc>),
  Clip(ClipItem),
  Transform(Transform),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextRun<'doc> {
  pub text: Cow<'doc, str>,
  pub origin: Point,
  pub line_height: Pt,
  pub style: TextStyle<'doc>,
  pub font_id: Option<FontId>,
  pub color: Color,
  pub rotation_center: Option<Point>,
  pub hyperlink_url: Option<Cow<'doc, str>>,
  pub dynamic_field: Option<DynamicField<'doc>>,
  pub form_widget_id: Option<u32>,
  pub paragraph_bidi: bool,
  pub preserve_text_portion: bool,
  pub pdf_text_segmentation: PdfTextSegmentation,
  pub source: Option<DisplaySource<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PdfTextSegmentation {
  #[default]
  Line,
  Portion,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphRun<'doc> {
  pub shaped: ShapedRun<'doc, 'doc>,
  pub origin: Point,
  pub glyphs: Cow<'doc, [ShapedGlyph]>,
  pub color: Color,
  pub source: Option<DisplaySource<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageItem<'doc> {
  pub bounds: Rect,
  pub crop: Option<ImageCrop>,
  pub rotation_degrees: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub content_type: Cow<'doc, str>,
  pub bytes: Arc<[u8]>,
  pub relationship_id: Option<Cow<'doc, str>>,
  pub alt_text: Option<Cow<'doc, str>>,
  pub hyperlink_url: Option<Cow<'doc, str>>,
  pub floating: bool,
  pub behind_text: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ImageCrop {
  pub left: f32,
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PathItem<'doc> {
  pub bounds: Rect,
  pub points: Vec<Point>,
  pub closed: bool,
  pub fill: Fill<'doc>,
  pub stroke: Option<Stroke<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RectItem<'doc> {
  pub bounds: Rect,
  pub fill: Fill<'doc>,
  pub stroke: Option<Stroke<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineItem<'doc> {
  pub start: Point,
  pub end: Point,
  pub stroke: Stroke<'doc>,
  pub kind: LineKind,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LineKind {
  #[default]
  Stroke,
  FilledRect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClipItem {
  pub bounds: Rect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LinkArea<'doc> {
  pub bounds: Rect,
  pub target: Cow<'doc, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationHint<'doc> {
  pub bounds: Rect,
  pub kind: Cow<'doc, str>,
  pub text: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutlineItem<'doc> {
  pub title: Cow<'doc, str>,
  pub page_index: usize,
  pub target: Option<Point>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AccessibilityHint<'doc> {
  pub item_index: usize,
  pub role: Cow<'doc, str>,
  pub label: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DisplayResources<'doc> {
  pub fonts: Vec<FontId>,
  pub images: Vec<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrameRecord<'doc> {
  pub id: FrameId,
  pub parent: Option<FrameId>,
  pub kind: Cow<'doc, str>,
  pub block_index: Option<usize>,
  pub page_index: usize,
  pub section_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub item_range: ItemRange,
  pub split_start: FrameCursor,
  pub split_end: FrameCursor,
  pub bounds: Option<Rect>,
  pub print_bounds: Option<Rect>,
  pub lines: Vec<LineBox>,
  pub fragments: Vec<FrameFragment>,
  pub influences: Vec<FrameInfluence>,
  pub invalidation: FrameInvalidation,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FrameId(pub u32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplaySource<'doc> {
  pub engine: LayoutEngineKind,
  pub path: Vec<usize>,
  pub relationship_id: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnsupportedLayoutFeature<'doc> {
  pub owner: Cow<'doc, str>,
  pub feature: Cow<'doc, str>,
  pub fallback: UnsupportedFallback,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UnsupportedFallback {
  #[default]
  Omitted,
  Approximated,
  Placeholder,
  PreservedForLater,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PageSetup {
  pub size: Size,
  pub margins: Insets,
  pub mirror_margins: bool,
  pub top_margin_was_negative: bool,
  pub bottom_margin_was_negative: bool,
  pub header_distance: Pt,
  pub footer_distance: Pt,
  pub background: Option<Color>,
  pub borders: CellBorders,
  pub borders_offset_from_text: bool,
  pub line_numbering: Option<LineNumbering>,
  pub doc_grid_line_pitch: Option<Pt>,
  pub page_number_start: Option<i32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CellBorders {
  pub top: Option<BorderStyle>,
  pub right: Option<BorderStyle>,
  pub bottom: Option<BorderStyle>,
  pub left: Option<BorderStyle>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderStyle {
  pub width: Pt,
  pub spacing: Pt,
  pub color: Color,
  pub compound: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineNumbering {
  pub count_by: i16,
  pub start: i16,
  pub distance: Pt,
  pub restart_each_page: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextStyle<'doc> {
  pub font_family: Option<Cow<'doc, str>>,
  pub east_asia_font_family: Option<Cow<'doc, str>>,
  pub complex_font_family: Option<Cow<'doc, str>>,
  pub symbol_font_family: Option<Cow<'doc, str>>,
  pub font_size: Pt,
  pub complex_font_size: Option<Pt>,
  pub character_spacing: Pt,
  pub baseline_shift: Pt,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
  pub strikethrough: bool,
  pub uppercase: bool,
  pub small_caps: bool,
  pub hidden: bool,
  pub rotation_degrees: f32,
  pub color: Color,
  pub outline_color: Option<Color>,
  pub outline_width: Pt,
  pub highlight: Option<Color>,
  pub underline_color: Option<Color>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DynamicField<'doc> {
  Page,
  NumPages,
  PageRef {
    bookmark_name: Cow<'doc, str>,
  },
  StyleRef {
    style_name: Cow<'doc, str>,
    from_bottom: bool,
  },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormWidget<'doc> {
  pub id: u32,
  pub kind: FormWidgetKind,
  pub entries: Vec<Cow<'doc, str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormWidgetKind {
  Text,
  DropDownList,
  ComboBox,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OutlineEntry<'doc> {
  pub level: u8,
  pub text: Cow<'doc, str>,
  pub page_index: usize,
  pub target: Point,
  pub merged_hidden_separator: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FrameKind {
  #[default]
  Paragraph,
  Table,
  Notes,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FollowReason {
  KeepTogether,
  Overflow,
  ExplicitBreak,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameFollow {
  pub kind: FrameKind,
  pub reason: FollowReason,
  pub block_index: Option<usize>,
  pub from_page_index: usize,
  pub to_page_index: usize,
  pub from_section_page_index: usize,
  pub to_section_page_index: usize,
  pub from_column_index: usize,
  pub to_column_index: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FrameCursor {
  pub block_index: Option<usize>,
  pub kind: FrameCursorKind,
  pub inline_index: usize,
  pub text_offset: usize,
  pub row_index: usize,
  pub cell_index: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FrameCursorKind {
  #[default]
  BlockStart,
  Inline,
  TableRow,
  TableCell,
  BlockEnd,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ItemRange {
  pub start: usize,
  pub end: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LineBox {
  pub bounds: Rect,
  pub item_range: ItemRange,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameFragmentKind {
  ParagraphLine,
  TableRow,
  TableCell,
  NoteLine,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FragmentSplitKind {
  Complete,
  Master,
  Follow,
  RepeatedHeader,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrameFragment {
  pub kind: FrameFragmentKind,
  pub split: FragmentSplitKind,
  pub index: usize,
  pub row_index: usize,
  pub cell_index: Option<usize>,
  pub item_range: ItemRange,
  pub bounds: Option<Rect>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameInfluenceKind {
  FootnoteReservation,
  FlyWrap,
  TableSplit,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrameInfluence {
  pub kind: FrameInfluenceKind,
  pub count: usize,
  pub block_index: Option<usize>,
  pub item_range: ItemRange,
  pub bounds: Option<Rect>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FrameInvalidation {
  #[default]
  Clean,
  PageItemsDecorated,
  NeedsReflow,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ReflowDiagnostics<'doc> {
  pub page_replays: Vec<PageReplay<'doc>>,
  pub page_replay_applications: Vec<PageReplayApplication>,
  pub backward_moves: Vec<BackwardMove>,
  pub layout_reruns: Vec<LayoutRerun>,
  pub page_invalidations: Vec<PageInvalidation>,
  pub reflow_executions: Vec<ReflowExecution>,
  pub reflow_requests: Vec<ReflowRequest>,
  pub restart_plan: Option<RestartPlan>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PageReplay<'doc> {
  pub page_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub scope: ReflowScope,
  pub item_range: ItemRange,
  pub replacement_items: Vec<DisplayItem<'doc>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageReplayApplication {
  pub page_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub scope: ReflowScope,
  pub item_range: ItemRange,
  pub replacement_count: usize,
  pub applied: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackwardMove {
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
pub struct LayoutRerun {
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
pub struct LayoutRerunConstraint {
  pub kind: FrameInfluenceKind,
  pub scope: ReflowScope,
  pub bounds: Option<Rect>,
  pub content_left: Pt,
  pub content_width: Pt,
  pub content_bottom: Pt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageInvalidation {
  pub page_index: usize,
  pub section_page_index: usize,
  pub first_frame_index: usize,
  pub reason: ReflowReason,
  pub scope: ReflowScope,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReflowExecution {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReflowRequest {
  pub frame_index: usize,
  pub kind: FrameKind,
  pub reason: ReflowReason,
  pub scope: ReflowScope,
  pub restart: FrameCursor,
  pub page_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub influence_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ReflowScope {
  Frame,
  Column,
  Page,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReflowReason {
  DecorationChangedItems,
  InsertionInfluenceChanged,
  InvalidBounds,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReflowAction {
  StabilizedRetainedDecorationItems,
  StabilizedInsertionInfluences,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestartPlan {
  pub page_index: usize,
  pub frame_index: usize,
  pub block_index: Option<usize>,
  pub cursor: FrameCursor,
  pub reason: ReflowReason,
  pub scope: ReflowScope,
}
