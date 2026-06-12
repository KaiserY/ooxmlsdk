use std::borrow::Cow;

use ooxmlsdk::parts::presentation_document::PresentationDocument;
use ooxmlsdk::schemas::{a, p};
use ooxmlsdk::sdk::SdkPart;
use ooxmlsdk_fonts::FontRequest;

use crate::common::{Emu, Fill, Insets, Stroke, Transform};

// Source: ECMA-376 DrawingML uses 914400 EMU per inch and 60000 angle units per degree.
const EMU_PER_INCH: i64 = 914_400;
const DEFAULT_SLIDE_WIDTH_EMU: i64 = 10 * EMU_PER_INCH;
const DEFAULT_SLIDE_HEIGHT_EMU: i64 = 7 * EMU_PER_INCH + EMU_PER_INCH / 2;
const DRAWINGML_ANGLE_UNITS_PER_DEGREE: f32 = 60_000.0;
// Source: LibreOffice oox WpsContext/text-body import defaults use 254/127 hmm.
const DEFAULT_TEXT_LEFT_RIGHT_INSET_EMU: i64 = 91_440;
const DEFAULT_TEXT_TOP_BOTTOM_INSET_EMU: i64 = 45_720;

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
  pub fn from_presentation_document(document: &'doc mut PresentationDocument) -> Self {
    let Ok(presentation_part) = document.presentation_part() else {
      return Self::default();
    };
    let presentation_part = presentation_part.clone();
    let Ok(root) = presentation_part.root_element(document) else {
      return Self::default();
    };
    let page_size = root
      .slide_size
      .as_ref()
      .map(|size| crate::common::Size {
        width: Emu(size.cx as i64).to_pt(),
        height: Emu(size.cy as i64).to_pt(),
      })
      .unwrap_or(crate::common::Size {
        width: Emu(DEFAULT_SLIDE_WIDTH_EMU).to_pt(),
        height: Emu(DEFAULT_SLIDE_HEIGHT_EMU).to_pt(),
      });
    let _ = root;

    let slide_parts = presentation_part.slide_parts(document).collect::<Vec<_>>();
    let slides = slide_parts
      .iter()
      .filter_map(|part| {
        part.root_element(document).ok().map(|slide| {
          import_slide(
            slide,
            part.relationship_id().map(|id| Cow::Owned(id.to_owned())),
          )
        })
      })
      .collect();
    Self {
      page_size,
      slides,
      ..Self::default()
    }
  }
}

fn import_slide<'doc>(
  slide: &p::Slide,
  relationship_id: Option<Cow<'doc, str>>,
) -> PptxSlide<'doc> {
  PptxSlide {
    relationship_id,
    name: slide.common_slide_data.name.clone().map(Cow::Owned),
    hidden: slide.show.is_some_and(|value| !value.as_bool()),
    shapes: slide
      .common_slide_data
      .shape_tree
      .shape_tree_choice
      .iter()
      .filter_map(import_shape_tree_choice)
      .collect(),
    ..PptxSlide::default()
  }
}

fn import_shape_tree_choice<'doc>(choice: &p::ShapeTreeChoice) -> Option<PptxShape<'doc>> {
  match choice {
    p::ShapeTreeChoice::Shape(shape) => Some(import_shape(shape)),
    p::ShapeTreeChoice::GroupShape(group) => Some(import_group_shape(group)),
    p::ShapeTreeChoice::GraphicFrame(frame) => Some(import_graphic_frame(frame)),
    p::ShapeTreeChoice::ConnectionShape(connection) => Some(import_connection_shape(connection)),
    p::ShapeTreeChoice::Picture(picture) => Some(import_picture(picture)),
    p::ShapeTreeChoice::ContentPart(_) | p::ShapeTreeChoice::XmlAny(_) => None,
  }
}

fn import_group_shape_choice<'doc>(choice: &p::GroupShapeChoice) -> Option<PptxShape<'doc>> {
  match choice {
    p::GroupShapeChoice::Shape(shape) => Some(import_shape(shape)),
    p::GroupShapeChoice::GroupShape(group) => Some(import_group_shape(group)),
    p::GroupShapeChoice::GraphicFrame(frame) => Some(import_graphic_frame(frame)),
    p::GroupShapeChoice::ConnectionShape(connection) => Some(import_connection_shape(connection)),
    p::GroupShapeChoice::Picture(picture) => Some(import_picture(picture)),
    p::GroupShapeChoice::ContentPart(_) | p::GroupShapeChoice::XmlAny(_) => None,
  }
}

fn import_shape<'doc>(shape: &p::Shape) -> PptxShape<'doc> {
  let non_visual = &shape
    .non_visual_shape_properties
    .non_visual_drawing_properties;
  PptxShape {
    id: Some(non_visual.id),
    name: Some(Cow::Owned(non_visual.name.clone())),
    kind: PptxShapeKind::Shape,
    transform: transform_from_shape_properties(&shape.shape_properties),
    geometry: shape_geometry(&shape.shape_properties),
    text_body: shape.text_body.as_deref().map(import_text_body),
    placeholder: shape
      .non_visual_shape_properties
      .application_non_visual_drawing_properties
      .placeholder_shape
      .as_deref()
      .map(import_placeholder),
    hidden: non_visual.hidden.is_some_and(|value| value.as_bool()),
    decorative: non_visual.description.as_deref() == Some(""),
    ..PptxShape::default()
  }
}

fn import_group_shape<'doc>(group: &p::GroupShape) -> PptxShape<'doc> {
  let non_visual = &group
    .non_visual_group_shape_properties
    .non_visual_drawing_properties;
  PptxShape {
    id: Some(non_visual.id),
    name: Some(Cow::Owned(non_visual.name.clone())),
    kind: PptxShapeKind::Group,
    children: group
      .group_shape_choice
      .iter()
      .filter_map(import_group_shape_choice)
      .collect(),
    hidden: non_visual.hidden.is_some_and(|value| value.as_bool()),
    ..PptxShape::default()
  }
}

fn import_picture<'doc>(picture: &p::Picture) -> PptxShape<'doc> {
  let non_visual = &picture
    .non_visual_picture_properties
    .non_visual_drawing_properties;
  PptxShape {
    id: Some(non_visual.id),
    name: Some(Cow::Owned(non_visual.name.clone())),
    kind: PptxShapeKind::Picture {
      relationship_id: picture
        .blip_fill
        .as_deref()
        .and_then(|fill| fill.blip.as_deref())
        .and_then(|blip| blip.embed.clone().or_else(|| blip.link.clone()))
        .map(Cow::Owned),
    },
    transform: transform_from_shape_properties(&picture.shape_properties),
    hidden: non_visual.hidden.is_some_and(|value| value.as_bool()),
    ..PptxShape::default()
  }
}

fn import_graphic_frame<'doc>(frame: &p::GraphicFrame) -> PptxShape<'doc> {
  let non_visual = &frame
    .non_visual_graphic_frame_properties
    .non_visual_drawing_properties;
  PptxShape {
    id: Some(non_visual.id),
    name: Some(Cow::Owned(non_visual.name.clone())),
    kind: PptxShapeKind::GraphicFrame {
      uri: Some(Cow::Owned(frame.graphic.graphic_data.uri.clone())),
    },
    transform: transform_from_pml_transform(&frame.transform),
    hidden: non_visual.hidden.is_some_and(|value| value.as_bool()),
    ..PptxShape::default()
  }
}

fn import_connection_shape<'doc>(connection: &p::ConnectionShape) -> PptxShape<'doc> {
  let non_visual = &connection
    .non_visual_connection_shape_properties
    .non_visual_drawing_properties;
  PptxShape {
    id: Some(non_visual.id),
    name: Some(Cow::Owned(non_visual.name.clone())),
    kind: PptxShapeKind::Connector,
    transform: transform_from_shape_properties(&connection.shape_properties),
    hidden: non_visual.hidden.is_some_and(|value| value.as_bool()),
    ..PptxShape::default()
  }
}

fn transform_from_shape_properties(properties: &p::ShapeProperties) -> Transform {
  properties
    .transform2_d
    .as_deref()
    .map(transform_from_dml_transform)
    .unwrap_or_default()
}

fn transform_from_dml_transform(transform: &a::Transform2D) -> Transform {
  Transform {
    dx: transform
      .offset
      .as_ref()
      .map(|offset| Emu(offset.x.to_emu()).to_pt())
      .unwrap_or_default(),
    dy: transform
      .offset
      .as_ref()
      .map(|offset| Emu(offset.y.to_emu()).to_pt())
      .unwrap_or_default(),
    ..Transform::default()
  }
}

fn transform_from_pml_transform(transform: &p::Transform) -> Transform {
  Transform {
    dx: transform
      .offset
      .as_ref()
      .map(|offset| Emu(offset.x.to_emu()).to_pt())
      .unwrap_or_default(),
    dy: transform
      .offset
      .as_ref()
      .map(|offset| Emu(offset.y.to_emu()).to_pt())
      .unwrap_or_default(),
    ..Transform::default()
  }
}

fn shape_geometry<'doc>(properties: &p::ShapeProperties) -> Option<Cow<'doc, str>> {
  match &properties.shape_properties_choice1 {
    Some(p::ShapePropertiesChoice::PresetGeometry(geometry)) => {
      Some(Cow::Owned(format!("{:?}", geometry.preset)))
    }
    Some(p::ShapePropertiesChoice::CustomGeometry(_)) => Some(Cow::Borrowed("custom")),
    None => None,
  }
}

fn import_placeholder<'doc>(placeholder: &p::PlaceholderShape) -> Placeholder<'doc> {
  Placeholder {
    kind: match placeholder.r#type.unwrap_or_default() {
      p::PlaceholderValues::Title => PlaceholderKind::Title,
      p::PlaceholderValues::CenteredTitle => PlaceholderKind::CenteredTitle,
      p::PlaceholderValues::SubTitle => PlaceholderKind::Subtitle,
      p::PlaceholderValues::DateAndTime => PlaceholderKind::Date,
      p::PlaceholderValues::Footer => PlaceholderKind::Footer,
      p::PlaceholderValues::SlideNumber => PlaceholderKind::SlideNumber,
      p::PlaceholderValues::Chart => PlaceholderKind::Chart,
      p::PlaceholderValues::Table => PlaceholderKind::Table,
      p::PlaceholderValues::Picture => PlaceholderKind::Picture,
      p::PlaceholderValues::Object
      | p::PlaceholderValues::ClipArt
      | p::PlaceholderValues::Diagram
      | p::PlaceholderValues::Media
      | p::PlaceholderValues::SlideImage
      | p::PlaceholderValues::Header => PlaceholderKind::Object,
      p::PlaceholderValues::Body => PlaceholderKind::Body,
    },
    index: placeholder.index,
    source: None,
  }
}

fn import_text_body<'doc>(body: &p::TextBody) -> TextBody<'doc> {
  TextBody {
    body_properties: import_text_body_properties(&body.body_properties),
    paragraphs: body.paragraph.iter().map(import_text_paragraph).collect(),
    ..TextBody::default()
  }
}

fn import_text_body_properties(properties: &a::BodyProperties) -> TextBodyProperties {
  TextBodyProperties {
    insets: Insets {
      top: properties
        .top_inset
        .map(|value| Emu(value.to_emu()).to_pt())
        .unwrap_or_else(|| Emu(DEFAULT_TEXT_TOP_BOTTOM_INSET_EMU).to_pt()),
      right: properties
        .right_inset
        .map(|value| Emu(value.to_emu()).to_pt())
        .unwrap_or_else(|| Emu(DEFAULT_TEXT_LEFT_RIGHT_INSET_EMU).to_pt()),
      bottom: properties
        .bottom_inset
        .map(|value| Emu(value.to_emu()).to_pt())
        .unwrap_or_else(|| Emu(DEFAULT_TEXT_TOP_BOTTOM_INSET_EMU).to_pt()),
      left: properties
        .left_inset
        .map(|value| Emu(value.to_emu()).to_pt())
        .unwrap_or_else(|| Emu(DEFAULT_TEXT_LEFT_RIGHT_INSET_EMU).to_pt()),
    },
    rotation_degrees: properties
      .rotation
      .map(|value| value as f32 / DRAWINGML_ANGLE_UNITS_PER_DEGREE)
      .unwrap_or(0.0),
    columns: properties.column_count.map(|value| value as u16),
    wrap: properties.wrap.is_some(),
    ..TextBodyProperties::default()
  }
}

fn import_text_paragraph<'doc>(paragraph: &a::Paragraph) -> TextParagraph<'doc> {
  TextParagraph {
    runs: paragraph
      .paragraph_choice
      .iter()
      .filter_map(|choice| match choice {
        a::ParagraphChoice::Run(run) => Some(TextRun {
          text: Cow::Owned(run.text.clone()),
          style: TextRunStyle::default(),
        }),
        a::ParagraphChoice::Field(field) => field.text.as_ref().map(|text| TextRun {
          text: Cow::Owned(text.clone()),
          style: TextRunStyle::default(),
        }),
        a::ParagraphChoice::Break(_) => Some(TextRun {
          text: Cow::Borrowed("\n"),
          style: TextRunStyle::default(),
        }),
        a::ParagraphChoice::TextMath(_) => None,
      })
      .collect(),
    ..TextParagraph::default()
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
