use std::borrow::Cow;

use ooxmlsdk_fonts::{FontId, ShapedGlyph, ShapedRun};

use crate::common::{Color, Fill, Point, Rect, Stroke, Transform};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct LayoutDocument<'doc> {
  pub engine_kind: LayoutEngineKind,
  pub options: LayoutOptions,
  pub pages: Vec<DisplayPage<'doc>>,
  pub frames: Vec<FrameRecord<'doc>>,
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
  pub font_id: Option<FontId>,
  pub color: Color,
  pub source: Option<DisplaySource<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphRun<'doc> {
  pub shaped: ShapedRun<'doc>,
  pub origin: Point,
  pub glyphs: Cow<'doc, [ShapedGlyph]>,
  pub color: Color,
  pub source: Option<DisplaySource<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageItem<'doc> {
  pub bounds: Rect,
  pub crop: Option<ImageCrop>,
  pub content_type: Cow<'doc, str>,
  pub bytes: Cow<'doc, [u8]>,
  pub relationship_id: Option<Cow<'doc, str>>,
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
  pub bounds: Rect,
  pub print_bounds: Rect,
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
