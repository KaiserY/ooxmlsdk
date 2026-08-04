use std::borrow::Cow;
use std::sync::Arc;

use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
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
  Group(CompositingGroup<'doc>),
  Path(PathItem<'doc>),
  Rect(RectItem<'doc>),
  Line(LineItem<'doc>),
  LinkArea(LinkArea<'doc>),
  AnnotationHint(AnnotationHint<'doc>),
  Clip(ClipItem),
  Transform(Transform),
}

/// A sequence of display items composed as one graphics group.
///
/// Keeping the items grouped is significant for DrawingML effects. A mask or
/// blend applied independently to overlapping paths changes their composite
/// result at the overlap, while reflections need one transform shared by the
/// content and its fade mask.
#[derive(Clone, Debug, PartialEq)]
pub struct CompositingGroup<'doc> {
  pub mask: Option<ImageItem<'doc>>,
  pub transform: Option<Transform>,
  pub blend_mode: BlendMode,
  pub opacity: f32,
  /// Allow an otherwise-identity wrapper to paint directly into its parent
  /// content stream. Most groups remain isolated because that boundary is
  /// significant to extraction, tagging, and compositing.
  pub flatten_identity: bool,
  /// Whether nested text belongs to the enclosing text line for PDF baseline
  /// and clipping ownership. Drawing-layer ordering containers set this to
  /// false because their text has already been laid out in its own frame.
  pub inherit_text_line_owner: bool,
  pub items: Vec<DisplayItem<'doc>>,
}

impl<'doc> CompositingGroup<'doc> {
  pub fn masked(mask: ImageItem<'doc>, items: Vec<DisplayItem<'doc>>) -> Self {
    Self {
      mask: Some(mask),
      transform: None,
      blend_mode: BlendMode::Normal,
      opacity: 1.0,
      flatten_identity: false,
      inherit_text_line_owner: true,
      items,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BlendMode {
  #[default]
  Normal,
  Multiply,
  Screen,
  Darken,
  Lighten,
  Overlay,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextRun<'doc> {
  pub text: Cow<'doc, str>,
  pub origin: Point,
  pub line_height: Pt,
  /// Optional page-space paint clip. Text remains present in the PDF content
  /// stream and semantic layer while glyph ink outside this rectangle is
  /// suppressed.
  pub paint_clip: Option<Rect>,
  pub style: TextStyle<'doc>,
  pub font_id: Option<FontId>,
  pub color: Color,
  pub rotation_center: Option<Point>,
  pub hyperlink_url: Option<Cow<'doc, str>>,
  pub dynamic_field: Option<DynamicField<'doc>>,
  pub form_widget_id: Option<u32>,
  pub paragraph_bidi: bool,
  pub word_spacing_pt: f32,
  pub preserve_text_portion: bool,
  pub pdf_text_segmentation: PdfTextSegmentation,
  pub source: Option<DisplaySource<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PdfTextSegmentation {
  #[default]
  Line,
  WordLine,
  Portion,
  /// A visible hyphen inserted by Word's automatic hyphenation engine.
  ///
  /// Keep the physical and semantic glyph as a hyphen-minus. PDF text
  /// extractors identify the line-end discretionary use (PDFium reports it as
  /// U+0002), while the same glyph at a page boundary remains a literal `-`.
  AutomaticHyphen,
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
  /// Optional page-space vector path used to clip the image before painting.
  pub clip_path: Vec<PathCommand>,
  pub rotation_degrees: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub content_type: Cow<'doc, str>,
  pub bytes: Arc<[u8]>,
  /// Optional caller-specific realization colors for a one-bit WMF DIB
  /// pattern. Ordinary metafiles retain their embedded palettes.
  pub metafile_monochrome_dib_palette_override: Option<[[u8; 3]; 2]>,
  /// Solid fill painted by the host shape behind an EMF/WMF preview.
  pub metafile_background_color: Option<[u8; 3]>,
  pub relationship_id: Option<Cow<'doc, str>>,
  pub alt_text: Option<Cow<'doc, str>>,
  pub hyperlink_url: Option<Cow<'doc, str>>,
  /// Whether PDF export should recover semantic text from an EMF/WMF OLE preview.
  pub semantic_metafile_text: bool,
  /// Whether a DrawingML metafile preview may use its near-native Header.Frame.
  pub metafile_native_size: bool,
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
  pub commands: Vec<PathCommand>,
  pub closed: bool,
  pub fill: Fill<'doc>,
  pub stroke: Option<Stroke<'doc>>,
}

/// Fill behavior attached to one DrawingML `a:path`.
///
/// This remains separate from [`Fill`] because the lighten and darken variants
/// modify the owning shape fill rather than naming an independent paint.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DrawingPathFillMode {
  None,
  #[default]
  Normal,
  Lighten,
  LightenLess,
  Darken,
  DarkenLess,
}

impl DrawingPathFillMode {
  /// Applies the per-path DrawingML shade/tint mode to resolved paint.
  ///
  /// `[MS-OI29500]` 20.1.10.37 defines Office's strong variants as a 40%
  /// black/white blend and, importantly, the `Less` variants as exactly
  /// `50/255` rather than `0.2`. Keeping the factors as integer channel units
  /// avoids an unnecessary floating-point compatibility boundary.
  pub fn apply_to_fill<'doc>(self, mut fill: Fill<'doc>) -> Fill<'doc> {
    if self == Self::None {
      return Fill::None;
    }
    if self == Self::Normal {
      return fill;
    }
    match &mut fill {
      Fill::Solid(color) => *color = self.apply_to_color(*color),
      Fill::Gradient(gradient) => {
        for stop in &mut gradient.stops {
          stop.color = self.apply_to_color(stop.color);
        }
      }
      Fill::Pattern(pattern) => {
        pattern.foreground = self.apply_to_color(pattern.foreground);
        pattern.background = self.apply_to_color(pattern.background);
      }
      Fill::None | Fill::Theme(_) | Fill::Image { .. } => {}
    }
    fill
  }

  pub fn apply_to_color(self, color: Color) -> Color {
    const CHANNEL_MAX: u16 = u8::MAX as u16;
    const STRONG_BLEND_CHANNELS: u16 = 102;
    const LESS_BLEND_CHANNELS: u16 = 50;
    let (target, target_weight) = match self {
      Self::Lighten => (u8::MAX, STRONG_BLEND_CHANNELS),
      Self::LightenLess => (u8::MAX, LESS_BLEND_CHANNELS),
      Self::Darken => (u8::MIN, STRONG_BLEND_CHANNELS),
      Self::DarkenLess => (u8::MIN, LESS_BLEND_CHANNELS),
      Self::None | Self::Normal => return color,
    };
    let channel = |value: u8| -> u8 {
      let source_weight = CHANNEL_MAX - target_weight;
      ((u16::from(value) * source_weight + u16::from(target) * target_weight) / CHANNEL_MAX) as u8
    };
    Color {
      r: channel(color.r),
      g: channel(color.g),
      b: channel(color.b),
      a: color.a,
    }
  }
}

/// One independently painted path from a DrawingML custom or preset geometry.
///
/// DrawingML geometries may contain several paths with different fill and
/// stroke flags. Keeping those flags beside the commands prevents host
/// renderers from flattening stroke-only and fill-only paths into one shape.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawingPath {
  pub commands: Vec<PathCommand>,
  pub fill_mode: DrawingPathFillMode,
  pub stroke: bool,
  pub extrusion_allowed: bool,
}

impl Default for DrawingPath {
  fn default() -> Self {
    Self {
      commands: Vec::new(),
      fill_mode: DrawingPathFillMode::Normal,
      stroke: true,
      extrusion_allowed: true,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PathCommand {
  MoveTo(Point),
  LineTo(Point),
  CubicTo {
    control1: Point,
    control2: Point,
    end: Point,
  },
  Close,
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

/// WordprocessingML vertical character alignment within a paragraph line.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LineVerticalAlignment {
  #[default]
  Auto,
  Top,
  Center,
  Baseline,
  Bottom,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextStyle<'doc> {
  pub font_family: Option<Cow<'doc, str>>,
  /// Document-scoped substitute preferred before generic system fallbacks.
  pub fallback_font_family: Option<Cow<'doc, str>>,
  pub east_asia_font_family: Option<Cow<'doc, str>>,
  pub complex_font_family: Option<Cow<'doc, str>>,
  pub symbol_font_family: Option<Cow<'doc, str>>,
  pub font_size: Pt,
  pub complex_font_size: Option<Pt>,
  pub complex_script: Option<bool>,
  pub right_to_left: Option<bool>,
  /// Resolved Unicode bidi level for a directionally uniform text portion.
  /// Unlike `right_to_left`, this affects shaping only and does not select
  /// WordprocessingML complex-script formatting.
  pub resolved_bidi_level: Option<u8>,
  pub complex_bold: Option<bool>,
  pub complex_italic: Option<bool>,
  pub kerning_minimum_size: Option<Pt>,
  /// OpenType ligature categories selected by WordprocessingML. `None`
  /// preserves the shaping engine defaults for non-Word document models.
  pub ligatures: Option<OpenTypeLigatures>,
  pub horizontal_scale: Option<f32>,
  pub character_spacing: Pt,
  pub baseline_shift: Pt,
  /// Original font size used by an automatic WordprocessingML
  /// superscript/subscript line box while the painted glyph stays reduced.
  pub automatic_escapement_font_size: Option<Pt>,
  /// Complex-script counterpart of `automatic_escapement_font_size`.
  pub automatic_escapement_complex_font_size: Option<Pt>,
  pub line_vertical_alignment: LineVerticalAlignment,
  /// Retain searchable/taggable text without painting visible glyphs.
  pub semantic_only: bool,
  /// Use legacy Windows/GDI ascent for the first baseline. PowerPoint's PDF
  /// path follows this metric; Word layout retains typographic metrics.
  pub use_windows_font_metrics: bool,
  /// Select Common characters using the WordprocessingML rFonts slot table.
  pub wordprocessingml_font_slots: bool,
  /// Enable Word's document-level East Asian punctuation compression.
  pub cjk_punctuation_compression_ratio: f32,
  /// Paint glyph outlines instead of searchable PDF text. Office uses this
  /// for DrawingML text transforms that its fixed-format writer vectorizes.
  pub pdf_glyph_outlines: bool,
  pub pdf_glyph_outline_options: Option<Arc<PdfGlyphOutlineOptions>>,
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

impl TextStyle<'_> {
  /// Unicode bidi level resolved by the owning paragraph layout, when this
  /// text portion contains only one shaping direction.
  pub fn resolved_bidi_level(&self) -> Option<u8> {
    self.resolved_bidi_level
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PdfGlyphOutlineOptions {
  pub semantic_text_overlay: bool,
  /// Vector paint for outlined DrawingML text. WordArt text fills live on the
  /// WordprocessingML run rather than the owning shape, so retaining the
  /// resolved fill here lets the PDF backend clip the authored gradient or
  /// pattern to the warped glyph outlines.
  pub fill: Option<Fill<'static>>,
  /// Vector paint for a DrawingML character outline. This stays independent
  /// from `fill`: w14:textOutline and a:rPr/a:ln may use a gradient while the
  /// glyph interior uses a solid color (or the reverse).
  pub outline_fill: Option<Fill<'static>>,
  /// Complete DrawingML character-outline style. Keeping the authored stroke
  /// beside the glyph paint preserves preset/custom dashes, cap, join, and
  /// miter semantics when text is vectorized or rasterized for WordArt.
  pub outline_stroke: Option<Stroke<'static>>,
  /// Page-space transform applied only to visible vector glyphs.
  pub transform: Option<crate::common::Transform>,
  /// Non-affine DrawingML WordArt mapping applied after shaping. A one-path
  /// preset follows a centerline; a multi-path preset interpolates piecewise
  /// across every authored warp boundary.
  pub text_warp: Option<Arc<TextWarp>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextWarp {
  pub source_bounds: Rect,
  /// Host-provided paint coordinate space. This is independent of the
  /// destination warp rectangle because Word, PowerPoint, and Excel can bind
  /// WordArt gradient or pattern coordinates to different shape/text frames.
  pub paint_bounds: Rect,
  pub boundaries: Vec<Vec<PathCommand>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OpenTypeLigatures {
  pub standard: bool,
  pub contextual: bool,
  pub historical: bool,
  pub discretionary: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DynamicField<'doc> {
  Page {
    number_format: FieldNumberFormat,
  },
  NumPages {
    number_format: FieldNumberFormat,
  },
  Sequence {
    identifier: Cow<'doc, str>,
    number_format: FieldNumberFormat,
  },
  PageRef {
    bookmark_name: Cow<'doc, str>,
  },
  StyleRef {
    style_name: Cow<'doc, str>,
    from_bottom: bool,
  },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FieldNumberFormat {
  #[default]
  Decimal,
  LowerRoman,
  UpperRoman,
  LowerLetter,
  UpperLetter,
  WordprocessingMl(w::NumberFormatValues),
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn drawing_path_less_fill_modes_use_office_50_over_255_factor() {
    let source = Color {
      r: 100,
      g: 150,
      b: 200,
      a: 77,
    };
    assert_eq!(
      DrawingPathFillMode::DarkenLess.apply_to_color(source),
      Color {
        r: 80,
        g: 120,
        b: 160,
        a: 77,
      }
    );
    assert_eq!(
      DrawingPathFillMode::LightenLess.apply_to_color(source),
      Color {
        r: 130,
        g: 170,
        b: 210,
        a: 77,
      }
    );
  }
}
