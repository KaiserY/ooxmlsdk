use std::borrow::Cow;

use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk_fonts::FontRequest;

use crate::common::{Fill, Insets, Stroke, Transform};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxPresentation<'doc> {
  pub page_size: crate::common::Size,
  pub slides: Vec<PptxSlide<'doc>>,
  pub masters: Vec<PptxMaster<'doc>>,
  pub layouts: Vec<PptxLayout<'doc>>,
  pub theme: PptxTheme<'doc>,
  pub resources: PptxResources<'doc>,
  pub shows: Vec<CustomShow<'doc>>,
}

impl<'doc> PptxPresentation<'doc> {
  pub fn from_presentation_document(_document: &'doc PresentationDocument) -> Self {
    Self::default()
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxSlide<'doc> {
  pub relationship_id: Option<Cow<'doc, str>>,
  pub name: Option<Cow<'doc, str>>,
  pub hidden: bool,
  pub layout_ref: Option<Cow<'doc, str>>,
  pub background: Option<SlideBackground<'doc>>,
  pub transition: Option<SlideTransition<'doc>>,
  pub timing: Option<TimingTree<'doc>>,
  pub shapes: Vec<PptxShape<'doc>>,
  pub notes: Option<PptxNotes<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxMaster<'doc> {
  pub relationship_id: Option<Cow<'doc, str>>,
  pub shapes: Vec<PptxShape<'doc>>,
  pub text_styles: TextListStyles<'doc>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxLayout<'doc> {
  pub relationship_id: Option<Cow<'doc, str>>,
  pub name: Option<Cow<'doc, str>>,
  pub shapes: Vec<PptxShape<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxNotes<'doc> {
  pub shapes: Vec<PptxShape<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxShape<'doc> {
  pub id: Option<u32>,
  pub name: Option<Cow<'doc, str>>,
  pub kind: PptxShapeKind<'doc>,
  pub transform: Transform,
  pub geometry: Option<Cow<'doc, str>>,
  pub custom_geometry: Option<CustomGeometry<'doc>>,
  pub fill: Fill<'doc>,
  pub line: Option<Stroke<'doc>>,
  pub effects: Vec<ShapeEffect<'doc>>,
  pub text_body: Option<TextBody<'doc>>,
  pub placeholder: Option<Placeholder<'doc>>,
  pub hidden: bool,
  pub decorative: bool,
  pub children: Vec<PptxShape<'doc>>,
}

impl Default for PptxShape<'_> {
  fn default() -> Self {
    Self {
      id: None,
      name: None,
      kind: PptxShapeKind::Shape,
      transform: Transform::default(),
      geometry: None,
      custom_geometry: None,
      fill: Fill::None,
      line: None,
      effects: Vec::new(),
      text_body: None,
      placeholder: None,
      hidden: false,
      decorative: false,
      children: Vec::new(),
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CustomGeometry<'doc> {
  pub preset: Option<Cow<'doc, str>>,
  pub adjustments: Vec<(Cow<'doc, str>, f32)>,
  pub text_rect: Option<crate::common::Rect>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeEffect<'doc> {
  Shadow {
    color: crate::common::Color,
    blur: crate::common::Pt,
    distance: crate::common::Pt,
    direction_degrees: f32,
  },
  Glow {
    color: crate::common::Color,
    radius: crate::common::Pt,
  },
  Reflection,
  SoftEdge(crate::common::Pt),
  Unsupported(Cow<'doc, str>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum PptxShapeKind<'doc> {
  #[default]
  Shape,
  Group,
  Picture {
    relationship_id: Option<Cow<'doc, str>>,
  },
  Media {
    relationship_id: Option<Cow<'doc, str>>,
    media_type: MediaType,
  },
  GraphicFrame {
    uri: Option<Cow<'doc, str>>,
  },
  Table(PptxTable<'doc>),
  Chart {
    relationship_id: Option<Cow<'doc, str>>,
  },
  Diagram {
    relationship_id: Option<Cow<'doc, str>>,
  },
  OleObject {
    relationship_id: Option<Cow<'doc, str>>,
  },
  Connector,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MediaType {
  #[default]
  Unknown,
  Audio,
  Video,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextBody<'doc> {
  pub body_properties: TextBodyProperties,
  pub list_style: TextListStyles<'doc>,
  pub paragraphs: Vec<TextParagraph<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextBodyProperties {
  pub insets: Insets,
  pub anchor: TextAnchor,
  pub rotation_degrees: f32,
  pub columns: Option<u16>,
  pub autofit: TextAutofit,
  pub vertical: TextVerticalMode,
  pub wrap: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextAnchor {
  #[default]
  Top,
  Middle,
  Bottom,
  Distributed,
  Justified,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextAutofit {
  #[default]
  None,
  Normal,
  Shape,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextVerticalMode {
  #[default]
  Horizontal,
  Vertical,
  Vertical270,
  WordArtVertical,
  MongolianVertical,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextListStyles<'doc> {
  pub levels: Vec<TextParagraphStyle<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextParagraph<'doc> {
  pub runs: Vec<TextRun<'doc>>,
  pub style: TextParagraphStyle<'doc>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextRun<'doc> {
  pub text: Cow<'doc, str>,
  pub style: TextRunStyle<'doc>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextParagraphStyle<'doc> {
  pub level: u8,
  pub margin_left: Option<crate::common::Pt>,
  pub indent: Option<crate::common::Pt>,
  pub alignment: TextAlignment,
  pub bullet: Option<TextBullet<'doc>>,
  pub default_run_style: TextRunStyle<'doc>,
  pub line_spacing: Option<TextSpacing>,
  pub space_before: Option<TextSpacing>,
  pub space_after: Option<TextSpacing>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextSpacing {
  Points(crate::common::Pt),
  Percent(f32),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextAlignment {
  #[default]
  Left,
  Center,
  Right,
  Justify,
  Distributed,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextBullet<'doc> {
  None,
  Character {
    character: Cow<'doc, str>,
    font: Option<Cow<'doc, str>>,
  },
  AutoNumber {
    scheme: Cow<'doc, str>,
    start_at: Option<u32>,
  },
  Picture {
    relationship_id: Cow<'doc, str>,
  },
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextRunStyle<'doc> {
  pub font: FontRequest<'doc>,
  pub color: Option<crate::common::Color>,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Placeholder<'doc> {
  pub kind: PlaceholderKind,
  pub index: Option<u32>,
  pub source: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SlideBackground<'doc> {
  pub fill: Fill<'doc>,
  pub follows_master: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxTable<'doc> {
  pub rows: Vec<PptxTableRow<'doc>>,
  pub columns: Vec<crate::common::Pt>,
  pub style_id: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxTableRow<'doc> {
  pub height: Option<crate::common::Pt>,
  pub cells: Vec<PptxTableCell<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxTableCell<'doc> {
  pub text_body: Option<TextBody<'doc>>,
  pub row_span: u32,
  pub grid_span: u32,
  pub fill: Fill<'doc>,
  pub borders: PptxTableCellBorders<'doc>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxTableCellBorders<'doc> {
  pub top: Option<Stroke<'doc>>,
  pub right: Option<Stroke<'doc>>,
  pub bottom: Option<Stroke<'doc>>,
  pub left: Option<Stroke<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PlaceholderKind {
  #[default]
  Body,
  Title,
  CenteredTitle,
  Subtitle,
  Date,
  Footer,
  SlideNumber,
  Object,
  Chart,
  Table,
  Picture,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxTheme<'doc> {
  pub name: Option<Cow<'doc, str>>,
  pub color_scheme: Vec<(Cow<'doc, str>, crate::common::Color)>,
  pub font_scheme: Vec<(Cow<'doc, str>, Cow<'doc, str>)>,
  pub fill_styles: Vec<Fill<'doc>>,
  pub line_styles: Vec<Stroke<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PptxResources<'doc> {
  pub relationships: Vec<PptxRelationship<'doc>>,
  pub media: Vec<PptxBinaryResource<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxRelationship<'doc> {
  pub id: Cow<'doc, str>,
  pub relationship_type: Cow<'doc, str>,
  pub target: Cow<'doc, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxBinaryResource<'doc> {
  pub relationship_id: Option<Cow<'doc, str>>,
  pub content_type: Cow<'doc, str>,
  pub bytes: Cow<'doc, [u8]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CustomShow<'doc> {
  pub name: Cow<'doc, str>,
  pub slide_relationship_ids: Vec<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SlideTransition<'doc> {
  pub kind: Cow<'doc, str>,
  pub duration_ms: Option<u32>,
  pub advance_after_ms: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TimingTree<'doc> {
  pub nodes: Vec<TimingNode<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TimingNode<'doc> {
  pub id: Option<u32>,
  pub target_shape_id: Option<u32>,
  pub kind: Cow<'doc, str>,
  pub children: Vec<TimingNode<'doc>>,
}
