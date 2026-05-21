use ooxmlsdk::schemas::schemas_openxmlformats_org_presentationml_2006_main as p;

use super::fill::FillProperties;
use super::line::LineProperties;
use super::shape_properties::EffectProperties;
use super::table::TableProperties;
use super::text_body::TextBody;
use super::text_list_style::TextListStyle;
use crate::pptx::import::PowerPointImport;
use crate::pptx::slide::ShapeLocation;

#[derive(Clone, Debug)]
pub(crate) struct Shape {
  pub(crate) service_name: ShapeService,
  pub(crate) shape_location: Option<ShapeLocation>,
  pub(crate) id: Option<u32>,
  pub(crate) name: Option<String>,
  pub(crate) description: Option<String>,
  pub(crate) title: Option<String>,
  pub(crate) hidden: bool,
  pub(crate) hidden_master_shape: bool,
  pub(crate) locked: bool,
  pub(crate) sub_type: Option<p::PlaceholderValues>,
  pub(crate) sub_type_index: Option<u32>,
  pub(crate) position: Point,
  pub(crate) size: Size,
  pub(crate) child_position: Point,
  pub(crate) child_size: Size,
  pub(crate) rotation: f32,
  pub(crate) diagram_rotation: f32,
  pub(crate) flip_h: bool,
  pub(crate) flip_v: bool,
  pub(crate) children: Vec<Shape>,
  pub(crate) referenced: bool,
  pub(crate) placeholder: Option<Box<Shape>>,
  pub(crate) line_properties: Option<LineProperties>,
  pub(crate) fill_properties: Option<FillProperties>,
  pub(crate) table_properties: Option<TableProperties>,
  pub(crate) picture: Option<PictureRecord>,
  pub(crate) content_part: Option<ContentPartRecord>,
  pub(crate) effect_properties: Option<EffectProperties>,
  pub(crate) text_body: Option<TextBody>,
  pub(crate) master_text_list_style: Option<TextListStyle>,
  pub(crate) shape_ref_line_properties: Option<LineProperties>,
  pub(crate) shape_ref_fill_properties: Option<FillProperties>,
  pub(crate) shape_ref_effect_properties: Option<EffectProperties>,
  pub(crate) frame_type: FrameType,
  pub(crate) graphic_data: Option<GraphicDataRecord>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Point {
  pub(crate) x: i64,
  pub(crate) y: i64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Size {
  pub(crate) cx: i64,
  pub(crate) cy: i64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ShapeService {
  #[default]
  CustomShape,
  GraphicObjectShape,
  GroupShape,
  ConnectorShape,
  Ole2Shape,
  TitleTextShape,
  SubtitleShape,
  OutlinerShape,
  NotesShape,
  DateTimeShape,
  HeaderShape,
  FooterShape,
  SlideNumberShape,
  PageShape,
  ChartShape,
  TableShape,
  MediaShape,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FrameType {
  #[default]
  Generic,
  Chart,
  Table,
  Diagram,
  Ole,
  Media,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GraphicDataRecord {
  pub(crate) uri: String,
  pub(crate) kind: GraphicDataKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PictureRecord {
  pub(crate) embed_relationship_id: Option<String>,
  pub(crate) link_relationship_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ContentPartRecord {
  pub(crate) relationship_id: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum GraphicDataKind {
  Ole,
  Diagram,
  Chart,
  ChartEx,
  Table,
  #[default]
  Unsupported,
}

impl Shape {
  pub(crate) fn new(service_name: ShapeService) -> Self {
    Self {
      service_name,
      shape_location: None,
      id: None,
      name: None,
      description: None,
      title: None,
      hidden: false,
      hidden_master_shape: false,
      locked: false,
      sub_type: None,
      sub_type_index: None,
      position: Point::default(),
      size: Size::default(),
      child_position: Point::default(),
      child_size: Size::default(),
      rotation: 0.0,
      diagram_rotation: 0.0,
      flip_h: false,
      flip_v: false,
      children: Vec::new(),
      referenced: false,
      placeholder: None,
      line_properties: None,
      fill_properties: None,
      table_properties: None,
      picture: None,
      content_part: None,
      effect_properties: None,
      text_body: None,
      master_text_list_style: None,
      shape_ref_line_properties: None,
      shape_ref_fill_properties: None,
      shape_ref_effect_properties: None,
      frame_type: FrameType::Generic,
      graphic_data: None,
    }
  }

  pub(crate) fn set_defaults(&mut self) {}

  pub(crate) fn apply_shape_reference(&mut self, reference: &Shape) {
    // Source: LibreOffice oox/source/drawingml/shape.cxx
    // applyShapeReference copies inherited properties while keeping direct
    // shape properties authoritative.
    self.shape_ref_line_properties = reference.line_properties.clone();
    self.shape_ref_fill_properties = reference.fill_properties.clone();
    self.shape_ref_effect_properties = reference.effect_properties.clone();
    if self.text_body.is_none() {
      self.text_body = reference.text_body.clone();
    }
  }

  pub(crate) fn set_placeholder(&mut self, placeholder: Shape) {
    self.placeholder = Some(Box::new(placeholder));
  }

  pub(crate) fn set_referenced(&mut self, referenced: bool) {
    self.referenced = referenced;
  }

  pub(crate) fn add_shape(&mut self, shape: Shape) {
    self.children.push(shape);
  }

  pub(crate) fn add_children(&mut self, children: Vec<Shape>) {
    self.children.extend(children);
  }

  pub(crate) fn hide_as_master_shape(&mut self) {
    self.hidden_master_shape = true;
    for child in &mut self.children {
      child.hide_as_master_shape();
    }
  }

  pub(crate) fn create_and_insert(&mut self, import: &PowerPointImport) {
    self.finalize_service_name();
    let _ = self.get_actual_fill_properties(import);
    let _ = self.get_actual_line_properties(import);
    let _ = self.get_actual_effect_properties(import);
    self.finalize_x_shape();
    for child in &mut self.children {
      child.create_and_insert(import);
    }
  }

  pub(crate) fn finalize_service_name(&mut self) {
    if let Some(service_name) = self.placeholder_service_name() {
      self.service_name = service_name;
    }
  }

  fn placeholder_service_name(&self) -> Option<ShapeService> {
    match self.sub_type? {
      p::PlaceholderValues::Title | p::PlaceholderValues::CenteredTitle => {
        Some(ShapeService::TitleTextShape)
      }
      p::PlaceholderValues::SubTitle => Some(ShapeService::SubtitleShape),
      p::PlaceholderValues::Body | p::PlaceholderValues::Object => {
        Some(ShapeService::OutlinerShape)
      }
      p::PlaceholderValues::DateAndTime => Some(ShapeService::DateTimeShape),
      p::PlaceholderValues::SlideNumber => Some(ShapeService::SlideNumberShape),
      p::PlaceholderValues::Footer => Some(ShapeService::FooterShape),
      p::PlaceholderValues::Header => Some(ShapeService::HeaderShape),
      p::PlaceholderValues::Chart => Some(ShapeService::ChartShape),
      p::PlaceholderValues::Table => Some(ShapeService::TableShape),
      p::PlaceholderValues::Media => Some(ShapeService::MediaShape),
      p::PlaceholderValues::ClipArt
      | p::PlaceholderValues::Diagram
      | p::PlaceholderValues::SlideImage
      | p::PlaceholderValues::Picture => None,
    }
  }

  pub(crate) fn finalize_x_shape(&mut self) {}

  pub(crate) fn put_property_to_grab_bag(&self) {}

  pub(crate) fn put_properties_to_grab_bag(&self) {}

  pub(crate) fn get_actual_fill_properties(
    &self,
    _import: &PowerPointImport,
  ) -> Option<FillProperties> {
    self
      .shape_ref_fill_properties
      .clone()
      .or_else(|| self.fill_properties.clone())
  }

  pub(crate) fn get_actual_line_properties(
    &self,
    _import: &PowerPointImport,
  ) -> Option<LineProperties> {
    self
      .shape_ref_line_properties
      .clone()
      .or_else(|| self.line_properties.clone())
  }

  pub(crate) fn get_actual_effect_properties(
    &self,
    _import: &PowerPointImport,
  ) -> Option<EffectProperties> {
    self
      .shape_ref_effect_properties
      .clone()
      .or_else(|| self.effect_properties.clone())
  }

  pub(crate) fn set_text_body(&mut self, text_body: TextBody) {
    self.text_body = Some(text_body);
  }

  pub(crate) fn set_master_text_list_style(&mut self, style: TextListStyle) {
    self.master_text_list_style = Some(style);
  }

  pub(crate) fn set_chart_type(&mut self) {
    self.frame_type = FrameType::Chart;
    self.service_name = ShapeService::ChartShape;
  }

  pub(crate) fn set_chart_ex_type(&mut self) {
    self.set_chart_type();
  }

  pub(crate) fn set_table_type(&mut self) {
    self.frame_type = FrameType::Table;
    self.service_name = ShapeService::TableShape;
  }

  pub(crate) fn set_diagram_type(&mut self) {
    self.frame_type = FrameType::Diagram;
    self.service_name = ShapeService::GroupShape;
  }

  pub(crate) fn set_ole_object_type(&mut self) {
    self.frame_type = FrameType::Ole;
    self.service_name = ShapeService::Ole2Shape;
  }

  pub(crate) fn set_graphic_data(&mut self, uri: String, kind: GraphicDataKind) {
    self.graphic_data = Some(GraphicDataRecord { uri, kind });
  }

  pub(crate) fn set_picture(
    &mut self,
    embed_relationship_id: Option<String>,
    link_relationship_id: Option<String>,
  ) {
    self.picture = Some(PictureRecord {
      embed_relationship_id,
      link_relationship_id,
    });
  }

  pub(crate) fn set_content_part(&mut self, relationship_id: String) {
    self.content_part = Some(ContentPartRecord { relationship_id });
    self.frame_type = FrameType::Media;
    self.service_name = ShapeService::MediaShape;
  }

  pub(crate) fn clone_fill_properties(&self) -> Option<FillProperties> {
    self.fill_properties.clone()
  }

  pub(crate) fn keep_diagram_drawing(&mut self) {}

  pub(crate) fn prepare_diagram_helper(&mut self) {}

  pub(crate) fn propagate_diagram_helper(&mut self) {}

  pub(crate) fn migrate_diagram_helper_to_new_shape(&mut self) {}
}
