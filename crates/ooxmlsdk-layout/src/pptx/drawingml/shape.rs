use ooxmlsdk::schemas::{
  schemas_openxmlformats_org_drawingml_2006_main as a,
  schemas_openxmlformats_org_presentationml_2006_main as p,
};

use super::fill::{FillKind, FillProperties};
use super::line::{LineFill, LineProperties};
use super::shape_properties::EffectProperties;
use super::table::TableProperties;
use super::text_body::TextBody;
use super::text_list_style::TextListStyle;
use crate::docx::ImageCrop;
use crate::pptx::import::PowerPointImport;
use crate::pptx::slide::{
  BinaryResource, ChartResource, DiagramColorResource, DiagramDataResource, DiagramLayoutResource,
  DiagramStyleResource, ExtendedChartResource, ImageResource, MediaResource, ShapeLocation,
};

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
  pub(crate) flip_h: bool,
  pub(crate) flip_v: bool,
  pub(crate) children: Vec<Shape>,
  pub(crate) referenced: bool,
  pub(crate) placeholder: Option<Box<Shape>>,
  pub(crate) hyperlink_url: Option<String>,
  pub(crate) line_properties: Option<LineProperties>,
  pub(crate) fill_properties: Option<FillProperties>,
  pub(crate) custom_shape_properties: CustomShapeProperties,
  pub(crate) table_properties: Option<TableProperties>,
  pub(crate) picture: Option<PictureRecord>,
  pub(crate) media: Option<MediaRecord>,
  pub(crate) content_part: Option<ContentPartRecord>,
  pub(crate) effect_properties: Option<EffectProperties>,
  pub(crate) text_body: Option<TextBody>,
  pub(crate) master_text_list_style: Option<TextListStyle>,
  pub(crate) shape_ref_line_properties: Option<LineProperties>,
  pub(crate) shape_ref_fill_properties: Option<FillProperties>,
  pub(crate) shape_ref_effect_properties: Option<EffectProperties>,
  pub(crate) actual_line_properties: Option<LineProperties>,
  pub(crate) actual_fill_properties: Option<FillProperties>,
  pub(crate) actual_effect_properties: Option<EffectProperties>,
  pub(crate) shape_style_refs: Option<ShapeStyleRefs>,
  pub(crate) connector_shape_properties: Vec<ConnectorShapeProperties>,
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
  Custom,
  GraphicObject,
  Group,
  Line,
  Connector,
  Ole2,
  TitleText,
  Subtitle,
  Outliner,
  DateTime,
  Header,
  Footer,
  SlideNumber,
  Chart,
  Table,
  Media,
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GraphicDataRecord {
  pub(crate) uri: String,
  pub(crate) kind: GraphicDataKind,
  pub(crate) chart_relationship_id: Option<String>,
  pub(crate) chart_resource: Option<ChartResource>,
  pub(crate) extended_chart_resource: Option<ExtendedChartResource>,
  pub(crate) has_inline_chart_space: bool,
  pub(crate) diagram_relationship_ids: Option<DiagramRelationshipIds>,
  pub(crate) diagram_data_resource: Option<DiagramDataResource>,
  pub(crate) diagram_layout_resource: Option<DiagramLayoutResource>,
  pub(crate) diagram_style_resource: Option<DiagramStyleResource>,
  pub(crate) diagram_color_resource: Option<DiagramColorResource>,
  pub(crate) ole_object: Option<OleObjectRecord>,
  pub(crate) ole_binary_resource: Option<BinaryResource>,
  pub(crate) embedded_package_resource: Option<BinaryResource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DiagramRelationshipIds {
  pub(crate) data_part: String,
  pub(crate) layout_part: String,
  pub(crate) style_part: String,
  pub(crate) color_part: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OleObjectRecord {
  pub(crate) relationship_id: Option<String>,
  pub(crate) name: Option<String>,
  pub(crate) prog_id: Option<String>,
  pub(crate) show_as_icon: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PictureRecord {
  pub(crate) embed_relationship_id: Option<String>,
  pub(crate) link_relationship_id: Option<String>,
  pub(crate) crop: ImageCrop,
  pub(crate) blip_choices: Vec<a::BlipChoice>,
  pub(crate) image_resource: Option<ImageResource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MediaRecord {
  pub(crate) kind: MediaKind,
  pub(crate) embed_relationship_id: Option<String>,
  pub(crate) link_relationship_id: Option<String>,
  pub(crate) resource: Option<MediaResource>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum MediaKind {
  Audio,
  Video,
  #[default]
  Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ContentPartRecord {
  pub(crate) relationship_id: String,
  pub(crate) resource: Option<MediaResource>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct CustomShapeProperties {
  pub(crate) geometry: Option<CustomShapeGeometry>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CustomShapeGeometry {
  Custom(std::boxed::Box<a::CustomGeometry>),
  Preset(std::boxed::Box<a::PresetGeometry>),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ShapeStyleRefs {
  pub(crate) line_reference: ShapeStyleReference,
  pub(crate) fill_reference: ShapeStyleReference,
  pub(crate) effect_reference: ShapeStyleReference,
  pub(crate) font_reference: FontStyleReference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ShapeStyleReference {
  pub(crate) index: u32,
  pub(crate) placeholder_color: Option<super::color::Color>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FontStyleReference {
  pub(crate) index: a::FontCollectionIndexValues,
  pub(crate) placeholder_color: Option<super::color::Color>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ConnectorShapeProperties {
  pub(crate) start_shape: bool,
  pub(crate) destination_shape_id: u32,
  pub(crate) destination_glue_id: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ShapeMapEntry {
  pub(crate) id: u32,
  pub(crate) service_name: ShapeService,
  pub(crate) name: Option<String>,
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
      flip_h: false,
      flip_v: false,
      children: Vec::new(),
      referenced: false,
      placeholder: None,
      hyperlink_url: None,
      line_properties: None,
      fill_properties: None,
      custom_shape_properties: CustomShapeProperties::default(),
      table_properties: None,
      picture: None,
      media: None,
      content_part: None,
      effect_properties: None,
      text_body: None,
      master_text_list_style: None,
      shape_ref_line_properties: None,
      shape_ref_fill_properties: None,
      shape_ref_effect_properties: None,
      actual_line_properties: None,
      actual_fill_properties: None,
      actual_effect_properties: None,
      shape_style_refs: None,
      connector_shape_properties: Vec::new(),
      frame_type: FrameType::Generic,
      graphic_data: None,
    }
  }

  pub(crate) fn apply_shape_reference(&mut self, reference: &Shape) {
    self.apply_shape_reference_with_text(reference, true);
  }

  pub(crate) fn apply_shape_reference_with_text(&mut self, reference: &Shape, use_text: bool) {
    // applyShapeReference copies inherited properties while keeping direct
    // shape properties authoritative. LibreOffice inherits getActual*()
    // results from placeholders, not just the placeholder's direct properties.
    self.shape_ref_line_properties = reference
      .actual_line_properties
      .clone()
      .or_else(|| reference.line_properties.clone());
    self.shape_ref_fill_properties = reference
      .actual_fill_properties
      .clone()
      .or_else(|| reference.fill_properties.clone());
    self.shape_ref_effect_properties = reference
      .actual_effect_properties
      .clone()
      .or_else(|| reference.effect_properties.clone());
    self.custom_shape_properties = reference.custom_shape_properties.clone();
    if use_text {
      self.text_body = reference.text_body.clone();
    } else {
      self.text_body = None;
    }
    self.table_properties = reference.table_properties.clone();
    self.master_text_list_style = reference.master_text_list_style.clone();
    self.position = reference.position;
    self.size = reference.size;
    self.rotation = reference.rotation;
    self.flip_h = reference.flip_h;
    self.flip_v = reference.flip_v;
    self.hidden = reference.hidden;
    self.locked = reference.locked;
  }

  pub(crate) fn set_placeholder(&mut self, placeholder: Shape) {
    self.placeholder = Some(Box::new(placeholder));
  }

  pub(crate) fn set_referenced(&mut self, referenced: bool) {
    self.referenced = referenced;
  }

  pub(crate) fn hide_as_master_shape(&mut self) {
    self.hidden_master_shape = true;
    for child in &mut self.children {
      child.hide_as_master_shape();
    }
  }

  pub(crate) fn hide_if_master_location(&mut self) {
    if self.shape_location == Some(ShapeLocation::Master) {
      self.hidden_master_shape = true;
    }
    for child in &mut self.children {
      child.hide_if_master_location();
    }
  }

  pub(crate) fn create_and_insert(&mut self, import: &PowerPointImport) {
    self.create_and_insert_with_parent_fill(import, None);
  }

  fn create_and_insert_with_parent_fill(
    &mut self,
    import: &PowerPointImport,
    parent_fill: Option<&FillProperties>,
  ) {
    self.finalize_service_name();
    self.actual_fill_properties = self.get_actual_fill_properties(import, parent_fill);
    self.actual_line_properties = self.get_actual_line_properties(import);
    self.actual_effect_properties = self.get_actual_effect_properties(import);
    self.finalize_x_shape();

    let child_parent_fill = self
      .actual_fill_properties
      .clone()
      .or_else(|| parent_fill.cloned());
    for child in &mut self.children {
      child.create_and_insert_with_parent_fill(import, child_parent_fill.as_ref());
    }
  }

  pub(crate) fn collect_shape_maps(
    &self,
    shape_map: &mut Vec<ShapeMapEntry>,
    connector_shape_map: &mut Vec<ShapeMapEntry>,
  ) {
    if let Some(id) = self.id {
      let entry = ShapeMapEntry {
        id,
        service_name: self.service_name,
        name: self.name.clone(),
      };
      shape_map.push(entry.clone());
      if self.service_name == ShapeService::Connector {
        connector_shape_map.push(entry);
      }
    }
    for child in &self.children {
      child.collect_shape_maps(shape_map, connector_shape_map);
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
        Some(ShapeService::TitleText)
      }
      p::PlaceholderValues::SubTitle => Some(ShapeService::Subtitle),
      p::PlaceholderValues::Body | p::PlaceholderValues::Object => Some(ShapeService::Outliner),
      p::PlaceholderValues::DateAndTime => Some(ShapeService::DateTime),
      p::PlaceholderValues::SlideNumber => Some(ShapeService::SlideNumber),
      p::PlaceholderValues::Footer => Some(ShapeService::Footer),
      p::PlaceholderValues::Header => Some(ShapeService::Header),
      p::PlaceholderValues::Chart => Some(ShapeService::Chart),
      p::PlaceholderValues::Table => Some(ShapeService::Table),
      p::PlaceholderValues::Media => Some(ShapeService::Media),
      p::PlaceholderValues::ClipArt
      | p::PlaceholderValues::Diagram
      | p::PlaceholderValues::SlideImage
      | p::PlaceholderValues::Picture => None,
    }
  }

  pub(crate) fn finalize_x_shape(&mut self) {
    // expands undersized rows to the graphical frame height, while still using
    // the DrawingML grid/row dimensions when they exceed the frame.
    if self.service_name == ShapeService::Table
      && let Some(table) = &self.table_properties
    {
      let width = table.grid.iter().copied().sum::<i64>();
      let height = table.rows.iter().map(|row| row.height).sum::<i64>();
      if width > self.size.cx {
        self.size.cx = width;
      }
      if height > self.size.cy {
        self.size.cy = height;
      }
    }
  }

  pub(crate) fn get_actual_fill_properties(
    &self,
    import: &PowerPointImport,
    parent_fill: Option<&FillProperties>,
  ) -> Option<FillProperties> {
    // getActualFillProperties applies reference/theme/direct properties, then
    // replaces direct grpFill with the parent group fill when one exists.
    let mut actual = self.shape_ref_fill_properties.clone();
    if let Some(style_refs) = &self.shape_style_refs
      && let Some(fill_ref) = import
        .get_theme_fill_style(style_refs.fill_reference.index)
        .map(|fill| {
          fill.with_placeholder_color(style_refs.fill_reference.placeholder_color.clone())
        })
    {
      actual = Some(fill_ref);
    }
    if let Some(fill) = &self.fill_properties {
      actual = match fill.kind {
        FillKind::Group => parent_fill.cloned().or_else(|| Some(fill.clone())),
        _ => Some(fill.clone()),
      };
    }
    actual
  }

  pub(crate) fn get_actual_line_properties(
    &self,
    import: &PowerPointImport,
  ) -> Option<LineProperties> {
    let themed = self.shape_style_refs.as_ref().and_then(|refs| {
      import
        .get_theme_line_style(refs.line_reference.index)
        .map(|line| line.with_placeholder_color(refs.line_reference.placeholder_color.clone()))
    });
    let inherited = merge_line_properties(self.shape_ref_line_properties.clone(), themed);
    merge_line_properties(inherited, self.line_properties.clone())
  }

  pub(crate) fn get_actual_effect_properties(
    &self,
    import: &PowerPointImport,
  ) -> Option<EffectProperties> {
    let themed = self
      .shape_style_refs
      .as_ref()
      .and_then(|refs| import.get_theme_effect_style(refs.effect_reference.index));
    let inherited = merge_effect_properties(self.shape_ref_effect_properties.clone(), themed);
    merge_effect_properties(inherited, self.effect_properties.clone())
  }

  pub(crate) fn set_text_body(&mut self, text_body: TextBody) {
    self.text_body = Some(text_body);
  }

  pub(crate) fn set_master_text_list_style(&mut self, style: TextListStyle) {
    self.master_text_list_style = Some(style);
  }

  pub(crate) fn apply_text_styles(&mut self) {
    if let Some(text_body) = &mut self.text_body {
      text_body.apply_text_styles(self.master_text_list_style.as_ref());
    }
    for child in &mut self.children {
      child.apply_text_styles();
    }
  }

  pub(crate) fn set_chart_type(&mut self) {
    self.frame_type = FrameType::Chart;
    self.service_name = ShapeService::Chart;
  }

  pub(crate) fn set_chart_ex_type(&mut self) {
    self.set_chart_type();
  }

  pub(crate) fn set_table_type(&mut self) {
    self.frame_type = FrameType::Table;
    self.service_name = ShapeService::Table;
  }

  pub(crate) fn set_diagram_type(&mut self) {
    self.frame_type = FrameType::Diagram;
    self.service_name = ShapeService::Group;
  }

  pub(crate) fn set_ole_object_type(&mut self) {
    self.frame_type = FrameType::Ole;
    self.service_name = ShapeService::Ole2;
  }

  pub(crate) fn set_graphic_data_record(&mut self, record: GraphicDataRecord) {
    self.graphic_data = Some(record);
  }

  pub(crate) fn set_custom_shape_geometry(&mut self, geometry: CustomShapeGeometry) {
    if let CustomShapeGeometry::Preset(preset) = &geometry
      && preset.preset == a::ShapeTypeValues::Line
      && self.service_name != ShapeService::Connector
    {
      self.service_name = ShapeService::Line;
    }
    self.custom_shape_properties.geometry = Some(geometry);
  }

  pub(crate) fn set_shape_style_refs(&mut self, style: &p::ShapeStyle) {
    self.shape_style_refs = Some(ShapeStyleRefs {
      line_reference: ShapeStyleReference {
        index: style.line_reference.index,
        placeholder_color: style
          .line_reference
          .line_reference_choice
          .as_ref()
          .and_then(super::color::Color::from_line_reference_choice),
      },
      fill_reference: ShapeStyleReference {
        index: style.fill_reference.index,
        placeholder_color: style
          .fill_reference
          .fill_reference_choice
          .as_ref()
          .and_then(super::color::Color::from_fill_reference_choice),
      },
      effect_reference: ShapeStyleReference {
        index: style.effect_reference.index,
        placeholder_color: style
          .effect_reference
          .effect_reference_choice
          .as_ref()
          .and_then(super::color::Color::from_effect_reference_choice),
      },
      font_reference: FontStyleReference {
        index: style.font_reference.index,
        placeholder_color: style
          .font_reference
          .font_reference_choice
          .as_ref()
          .and_then(super::color::Color::from_font_reference_choice)
          .or_else(|| {
            Some(super::color::Color::Scheme(super::color::SchemeColor {
              value: a::SchemeColorValues::Text1,
              transformations: Vec::new(),
            }))
          }),
      },
    });
  }

  pub(crate) fn set_picture(
    &mut self,
    embed_relationship_id: Option<String>,
    link_relationship_id: Option<String>,
    crop: ImageCrop,
    blip_choices: Vec<a::BlipChoice>,
    image_resource: Option<ImageResource>,
  ) {
    self.picture = Some(PictureRecord {
      embed_relationship_id,
      link_relationship_id,
      crop,
      blip_choices,
      image_resource,
    });
  }

  pub(crate) fn set_content_part(&mut self, relationship_id: String) {
    self.content_part = Some(ContentPartRecord {
      relationship_id,
      resource: None,
    });
    self.frame_type = FrameType::Media;
    self.service_name = ShapeService::Media;
  }

  pub(crate) fn set_content_part_resource(&mut self, resource: Option<MediaResource>) {
    if let Some(content_part) = &mut self.content_part {
      content_part.resource = resource;
    }
  }

  pub(crate) fn set_media(
    &mut self,
    kind: MediaKind,
    embed_relationship_id: Option<String>,
    link_relationship_id: Option<String>,
    resource: Option<MediaResource>,
  ) {
    self.media = Some(MediaRecord {
      kind,
      embed_relationship_id,
      link_relationship_id,
      resource,
    });
    self.frame_type = FrameType::Media;
    self.service_name = ShapeService::Media;
  }

  pub(crate) fn add_connector_shape_properties(
    &mut self,
    start_shape: bool,
    destination_shape_id: u32,
    destination_glue_id: u32,
  ) {
    self
      .connector_shape_properties
      .push(ConnectorShapeProperties {
        start_shape,
        destination_shape_id,
        destination_glue_id,
      });
  }

  pub(crate) fn keep_diagram_drawing(&mut self) {
    self.frame_type = FrameType::Diagram;
  }
}

fn merge_line_properties(
  base: Option<LineProperties>,
  direct: Option<LineProperties>,
) -> Option<LineProperties> {
  match (base, direct) {
    (Some(mut base), Some(direct)) => {
      if direct.fill != LineFill::Unspecified {
        base.fill = direct.fill;
        base.placeholder_color = direct.placeholder_color;
      }
      if direct.width_emu.is_some() {
        base.width_emu = direct.width_emu;
      }
      Some(base)
    }
    (Some(base), None) => Some(base),
    (None, Some(direct)) => Some(direct),
    (None, None) => None,
  }
}

fn merge_effect_properties(
  base: Option<EffectProperties>,
  direct: Option<EffectProperties>,
) -> Option<EffectProperties> {
  match (base, direct) {
    (Some(mut base), Some(direct)) => {
      if direct.clears_inherited {
        return Some(direct);
      }
      if direct.has_effect() {
        base.merge_from(&direct);
      }
      Some(base)
    }
    (Some(base), None) => Some(base),
    (None, Some(direct)) => Some(direct),
    (None, None) => None,
  }
}
