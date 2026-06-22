use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct LayoutDocument {
  pub pages: Vec<Page>,
  pub form_widgets: Vec<FormWidget>,
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

pub(crate) fn item_pages(pages: Vec<(PageSetup, Vec<PageItem>)>) -> LayoutDocument {
  fixed_pages_with_items(pages)
}

pub(crate) fn fixed_pages_with_items(pages: Vec<(PageSetup, Vec<PageItem>)>) -> LayoutDocument {
  let mut output_pages = Vec::new();
  for (setup, items) in pages {
    output_pages.push(Page {
      setup,
      section_index: 0,
      section_page_index: 0,
      items,
    });
  }

  if output_pages.is_empty() {
    output_pages.push(Page {
      setup: PageSetup::default(),
      section_index: 0,
      section_page_index: 0,
      items: Vec::new(),
    });
  }

  LayoutDocument {
    pages: output_pages,
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

#[derive(Clone, Debug)]
pub struct OutlineEntry {
  pub level: u8,
  pub text: String,
  pub page_index: usize,
  pub x_pt: f32,
  pub y_pt: f32,
  pub merged_hidden_separator: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnchorPage {
  pub name: String,
  pub page_index: usize,
  pub section_index: usize,
  pub section_page_index: usize,
  pub physical_page_number: usize,
  pub virtual_page_number: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FollowFrameKind {
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
pub struct LayoutFrame {
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
pub struct FrameCursor {
  pub block_index: Option<usize>,
  pub kind: FrameCursorKind,
  pub inline_index: usize,
  pub text_offset: usize,
  pub row_index: usize,
  pub cell_index: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameCursorKind {
  BlockStart,
  Inline,
  TableRow,
  TableCell,
  BlockEnd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameFragmentKind {
  ParagraphLine,
  TableRow,
  TableCell,
  NoteLine,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrameFragment {
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
pub enum FragmentSplitKind {
  Complete,
  Master,
  Follow,
  RepeatedHeader,
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
  pub item_start: usize,
  pub item_end: usize,
  pub bounds: Option<FrameBounds>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrameInvalidation {
  Clean,
  PageItemsDecorated,
  NeedsReflow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReflowRequest {
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
pub enum ReflowReason {
  DecorationChangedItems,
  InsertionInfluenceChanged,
  InvalidBounds,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ReflowScope {
  Frame,
  Column,
  Page,
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

#[derive(Clone, Debug)]
pub struct PageReplay {
  pub page_index: usize,
  pub section_page_index: usize,
  pub column_index: usize,
  pub scope: ReflowScope,
  pub item_start: usize,
  pub item_end: usize,
  pub replacement_items: Vec<PageItem>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageReplayApplication {
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
  pub bounds: Option<FrameBounds>,
  pub content_left_pt: f32,
  pub content_width: f32,
  pub content_bottom: f32,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrameBounds {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineBox {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub item_start: usize,
  pub item_end: usize,
}

#[derive(Clone, Debug)]
pub struct Page {
  pub setup: PageSetup,
  pub section_index: usize,
  pub section_page_index: usize,
  pub items: Vec<PageItem>,
}

#[derive(Clone, Debug)]
pub enum PageItem {
  Text(TextItem),
  Image(ImageItem),
  LinkArea(LinkAreaItem),
  Rect(RectItem),
  Fill(FillItem),
  Line(LineItem),
  Polyline(PolylineItem),
}

#[derive(Clone, Debug)]
pub struct TextItem {
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
  pub form_widget_id: Option<u32>,
  pub paragraph_bidi: bool,
  pub preserve_text_portion: bool,
  pub decoration_span_start_x_pt: Option<f32>,
  pub pdf_text_segmentation: PdfTextSegmentation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfTextSegmentation {
  Line,
  Portion,
}

#[derive(Clone, Debug)]
pub struct ImageItem {
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
  pub floating: bool,
  pub behind_text: bool,
}

#[derive(Clone, Debug)]
pub struct LinkAreaItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub hyperlink_url: String,
}

#[derive(Clone, Copy, Debug)]
pub struct RectItem {
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
pub struct FillItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub color: RgbColor,
}

#[derive(Clone, Copy, Debug)]
pub struct LineItem {
  pub x1_pt: f32,
  pub y1_pt: f32,
  pub x2_pt: f32,
  pub y2_pt: f32,
  pub width_pt: f32,
  pub color: RgbColor,
  pub kind: LineItemKind,
}

#[derive(Clone, Debug)]
pub struct PolylineItem {
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
pub enum LineItemKind {
  Stroke,
  FilledRect,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CellBordersModel {
  pub top: Option<BorderStyle>,
  pub right: Option<BorderStyle>,
  pub bottom: Option<BorderStyle>,
  pub left: Option<BorderStyle>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderStyle {
  pub width_pt: f32,
  pub spacing_pt: f32,
  pub color: RgbColor,
  pub compound: bool,
}

impl Default for BorderStyle {
  fn default() -> Self {
    Self {
      width_pt: 0.5,
      spacing_pt: 0.0,
      color: RgbColor { r: 0, g: 0, b: 0 },
      compound: false,
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
  pub font_family: Option<Arc<str>>,
  pub symbol_font_family: Option<Arc<str>>,
  pub font_size_pt: f32,
  pub complex_font_size_pt: Option<f32>,
  pub character_spacing_pt: f32,
  pub baseline_shift_pt: f32,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
  pub strikethrough: bool,
  pub uppercase: bool,
  pub small_caps: bool,
  pub hidden: bool,
  pub rotation_deg: f32,
  pub color: RgbColor,
  pub opacity: f32,
  pub outline_color: Option<RgbColor>,
  pub outline_opacity: f32,
  pub outline_width_pt: f32,
  pub highlight: Option<RgbColor>,
  pub underline_color: Option<RgbColor>,
}

impl Default for TextStyle {
  fn default() -> Self {
    Self {
      font_family: None,
      symbol_font_family: None,
      font_size_pt: 11.0,
      complex_font_size_pt: None,
      character_spacing_pt: 0.0,
      baseline_shift_pt: 0.0,
      bold: false,
      italic: false,
      underline: false,
      strikethrough: false,
      uppercase: false,
      small_caps: false,
      hidden: false,
      rotation_deg: 0.0,
      color: RgbColor { r: 0, g: 0, b: 0 },
      opacity: 1.0,
      outline_color: None,
      outline_opacity: 1.0,
      outline_width_pt: 0.0,
      highlight: None,
      underline_color: None,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RgbColor {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ImageCrop {
  pub left: f32,
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct PageSetup {
  pub width_pt: f32,
  pub height_pt: f32,
  pub margin_top_pt: f32,
  pub margin_right_pt: f32,
  pub margin_bottom_pt: f32,
  pub margin_left_pt: f32,
  pub mirror_margins: bool,
  pub top_margin_was_negative: bool,
  pub bottom_margin_was_negative: bool,
  pub header_distance_pt: f32,
  pub footer_distance_pt: f32,
  pub background: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub borders_offset_from_text: bool,
  pub line_numbering: Option<LineNumbering>,
  pub doc_grid_line_pitch_pt: Option<f32>,
  pub page_number_start: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineNumbering {
  pub count_by: i16,
  pub start: i16,
  pub distance_pt: f32,
  pub restart_each_page: bool,
}

impl Default for PageSetup {
  fn default() -> Self {
    Self {
      width_pt: 612.0,
      height_pt: 792.0,
      margin_top_pt: 72.0,
      margin_right_pt: 72.0,
      margin_bottom_pt: 72.0,
      margin_left_pt: 72.0,
      mirror_margins: false,
      top_margin_was_negative: false,
      bottom_margin_was_negative: false,
      header_distance_pt: 36.0,
      footer_distance_pt: 36.0,
      background: None,
      borders: CellBordersModel::default(),
      borders_offset_from_text: false,
      line_numbering: None,
      doc_grid_line_pitch_pt: None,
      page_number_start: None,
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DynamicFieldKind {
  Page,
  NumPages,
  StyleRef {
    style_name: Arc<str>,
    from_bottom: bool,
  },
}

#[derive(Clone, Debug)]
pub struct FormWidget {
  pub id: u32,
  pub kind: FormWidgetKind,
  pub entries: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormWidgetKind {
  Text,
  DropDownList,
  ComboBox,
}
