use ooxmlsdk::parts::control_properties_part::ControlPropertiesPart;
use ooxmlsdk::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart;
use std::collections::HashMap;

use ooxmlsdk::parts::image_part::ImagePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::vml_drawing_part::VmlDrawingPart;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_excel as xvml;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main as x14;
use ooxmlsdk::schemas::schemas_microsoft_com_vml as vml;
use ooxmlsdk::sdk::{SdkPart, SdkType};
use quick_xml::events::Event;
use quick_xml::{Writer, events::BytesStart};

use super::drawing::ImageResource;
use super::worksheet::{CellAddress, CellRange};
use crate::error::Result;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorksheetObjectResourceCatalog {
  pub(crate) vml_drawings: Vec<VmlDrawingResourceCatalog>,
  pub(crate) controls: Vec<ControlPersistenceResourceCatalog>,
  pub(crate) control_properties: Vec<ControlPropertiesResourceCatalog>,
  pub(crate) embedded_objects: Vec<BinaryResourceCatalog>,
  pub(crate) embedded_packages: Vec<BinaryResourceCatalog>,
  pub(crate) images: usize,
  pub(crate) named_sheet_views: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct VmlDrawingResourceCatalog {
  pub(crate) images: usize,
  pub(crate) image_resources: HashMap<String, ImageResource>,
  pub(crate) legacy_diagram_texts: usize,
  pub(crate) shapes: Vec<VmlShapeModel>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VmlShapeModel {
  pub(crate) text: String,
  pub(crate) style: Option<String>,
  pub(crate) object_type: Option<String>,
  pub(crate) image_relationship_id: Option<String>,
  pub(crate) anchor: Option<VmlClientAnchor>,
  pub(crate) note_row: Option<u32>,
  pub(crate) note_column: Option<u32>,
  pub(crate) print_object: bool,
  pub(crate) visible: bool,
  pub(crate) hidden: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct VmlClientAnchor {
  pub(crate) from_col: u32,
  pub(crate) from_col_offset_px: i32,
  pub(crate) from_row: u32,
  pub(crate) from_row_offset_px: i32,
  pub(crate) to_col: u32,
  pub(crate) to_col_offset_px: i32,
  pub(crate) to_row: u32,
  pub(crate) to_row_offset_px: i32,
}

impl VmlClientAnchor {
  pub(crate) fn cell_range(self) -> CellRange {
    CellRange::new(
      CellAddress {
        col: self.from_col.saturating_add(1),
        row: self.from_row.saturating_add(1),
      },
      CellAddress {
        col: self.to_col.saturating_add(1),
        row: self.to_row.saturating_add(1),
      },
    )
  }
}

impl Default for VmlShapeModel {
  fn default() -> Self {
    Self {
      text: String::new(),
      style: None,
      object_type: None,
      image_relationship_id: None,
      anchor: None,
      note_row: None,
      note_column: None,
      print_object: true,
      visible: false,
      hidden: false,
    }
  }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ControlPersistenceResourceCatalog {
  pub(crate) binary_data_parts: Vec<BinaryResourceCatalog>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ControlPropertiesResourceCatalog {
  pub(crate) has_object_type: bool,
  pub(crate) has_checked: bool,
  pub(crate) boolean_flags: usize,
  pub(crate) numeric_fields: usize,
  pub(crate) formula_fields: usize,
  pub(crate) alignment_fields: usize,
  pub(crate) list_items: usize,
  pub(crate) has_extension_list: bool,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct BinaryResourceCatalog;

impl WorksheetObjectResourceCatalog {
  pub(crate) fn from_worksheet_part(
    package: &mut SpreadsheetDocument,
    part: &WorksheetPart,
  ) -> Result<Self> {
    let vml_drawing_parts = part.vml_drawing_parts(package).collect::<Vec<_>>();
    let control_parts = part
      .embedded_control_persistence_parts(package)
      .collect::<Vec<_>>();
    let control_properties_parts = part.control_properties_parts(package).collect::<Vec<_>>();
    Ok(Self {
      vml_drawings: vml_drawing_parts
        .iter()
        .map(|part| VmlDrawingResourceCatalog::from_part(package, part))
        .collect(),
      controls: control_parts
        .iter()
        .map(|part| ControlPersistenceResourceCatalog::from_part(package, part))
        .collect(),
      control_properties: control_properties_parts
        .iter()
        .map(|part| ControlPropertiesResourceCatalog::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      embedded_objects: part
        .embedded_object_parts(package)
        .map(|part| BinaryResourceCatalog::from_part(&part))
        .collect(),
      embedded_packages: part
        .embedded_package_parts(package)
        .map(|part| BinaryResourceCatalog::from_part(&part))
        .collect(),
      images: part.image_parts(package).count(),
      named_sheet_views: part.named_sheet_views_parts(package).count(),
    })
  }

  pub(crate) fn from_chartsheet_part(
    package: &mut SpreadsheetDocument,
    part: &ooxmlsdk::parts::chartsheet_part::ChartsheetPart,
  ) -> Self {
    let vml_drawing_parts = part.vml_drawing_parts(package).collect::<Vec<_>>();
    Self {
      vml_drawings: vml_drawing_parts
        .iter()
        .map(|part| VmlDrawingResourceCatalog::from_part(package, part))
        .collect(),
      images: part.image_parts(package).count(),
      ..Self::default()
    }
  }
}

impl VmlDrawingResourceCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &VmlDrawingPart) -> Self {
    let shapes = part
      .data_to_vec(package)
      .map(|data| vml_shapes(&data))
      .unwrap_or_default();
    Self {
      images: part.image_parts(package).count(),
      image_resources: collect_vml_image_resources(package, part),
      legacy_diagram_texts: part.legacy_diagram_text_parts(package).count(),
      shapes,
    }
  }
}

fn vml_shapes(data: &[u8]) -> Vec<VmlShapeModel> {
  let mut reader = quick_xml::Reader::from_reader(data);
  reader.config_mut().trim_text(false);

  let mut shapes = Vec::new();
  let mut writer: Option<Writer<Vec<u8>>> = None;
  let mut fragment_depth = 0usize;
  let mut fragment_local_name = Vec::new();
  let mut namespace_attrs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        if let Some(shape_writer) = writer.as_mut() {
          fragment_depth = fragment_depth.saturating_add(1);
          if shape_writer
            .write_event(Event::Start(event.into_owned()))
            .is_err()
          {
            writer = None;
            fragment_depth = 0;
            fragment_local_name.clear();
          }
        } else if is_vml_shape_element(event.name().as_ref()) {
          let mut shape_writer = Writer::new(Vec::new());
          fragment_local_name = xml_local_name(event.name().as_ref()).to_vec();
          let mut start = event.into_owned();
          push_missing_xml_namespace_attrs(&mut start, &namespace_attrs);
          if shape_writer.write_event(Event::Start(start)).is_ok() {
            writer = Some(shape_writer);
            fragment_depth = 1;
          }
        } else {
          collect_xml_namespace_attrs(&event, &mut namespace_attrs);
        }
      }
      Ok(Event::Empty(event)) => {
        if let Some(shape_writer) = writer.as_mut() {
          if shape_writer
            .write_event(Event::Empty(event.into_owned()))
            .is_err()
          {
            writer = None;
            fragment_depth = 0;
            fragment_local_name.clear();
          }
        } else if is_vml_shape_element(event.name().as_ref()) {
          let mut shape_writer = Writer::new(Vec::new());
          let local_name = xml_local_name(event.name().as_ref()).to_vec();
          let mut start = event.into_owned();
          push_missing_xml_namespace_attrs(&mut start, &namespace_attrs);
          if shape_writer.write_event(Event::Empty(start)).is_ok() {
            shapes.extend(vml_shape_models_from_bytes(
              &local_name,
              &shape_writer.into_inner(),
            ));
          }
        } else {
          collect_xml_namespace_attrs(&event, &mut namespace_attrs);
        }
      }
      Ok(Event::End(event)) => {
        if let Some(shape_writer) = writer.as_mut()
          && shape_writer
            .write_event(Event::End(event.into_owned()))
            .is_err()
        {
          writer = None;
          fragment_depth = 0;
          fragment_local_name.clear();
          continue;
        }
        if fragment_depth > 0 {
          fragment_depth -= 1;
          if fragment_depth == 0
            && let Some(shape_writer) = writer.take()
          {
            shapes.extend(vml_shape_models_from_bytes(
              &fragment_local_name,
              &shape_writer.into_inner(),
            ));
            fragment_local_name.clear();
          }
        }
      }
      Ok(Event::Eof) => break,
      Ok(event) => {
        if let Some(shape_writer) = writer.as_mut()
          && shape_writer.write_event(event.into_owned()).is_err()
        {
          writer = None;
          fragment_depth = 0;
          fragment_local_name.clear();
        }
      }
      Err(_) => break,
    }
  }
  shapes
}

fn is_vml_shape_element(name: &[u8]) -> bool {
  matches!(
    xml_local_name(name),
    b"shape"
      | b"group"
      | b"arc"
      | b"curve"
      | b"image"
      | b"line"
      | b"oval"
      | b"polyline"
      | b"rect"
      | b"roundrect"
  )
}

fn vml_shape_models_from_bytes(local_name: &[u8], bytes: &[u8]) -> Vec<VmlShapeModel> {
  macro_rules! typed_model {
    ($type:ty) => {
      <$type>::from_bytes(bytes)
        .ok()
        .map(|shape| vml_shape_from_typed(&shape))
        .into_iter()
        .collect()
    };
  }

  match local_name {
    b"shape" => typed_model!(vml::Shape),
    b"group" => vml::Group::from_bytes(bytes)
      .ok()
      .map(|group| vml_group_shape_models(&group))
      .unwrap_or_default(),
    b"arc" => typed_model!(vml::Arc),
    b"curve" => typed_model!(vml::Curve),
    b"image" => typed_model!(vml::ImageFile),
    b"line" => typed_model!(vml::Line),
    b"oval" => typed_model!(vml::Oval),
    b"polyline" => typed_model!(vml::PolyLine),
    b"rect" => typed_model!(vml::Rectangle),
    b"roundrect" => typed_model!(vml::RoundRectangle),
    _ => Vec::new(),
  }
}

trait VmlShapeElement {
  fn style(&self) -> Option<String>;
  fn user_hidden(&self) -> bool;
  fn collect_model_children(&self, model: &mut VmlShapeModel);
}

macro_rules! impl_vml_shape_element {
  ($type:ident, $children:ident, $choice:ident) => {
    impl VmlShapeElement for vml::$type {
      fn style(&self) -> Option<String> {
        self.style.clone()
      }

      fn user_hidden(&self) -> bool {
        self.user_hidden.is_some_and(|value| value.as_bool())
      }

      fn collect_model_children(&self, model: &mut VmlShapeModel) {
        for child in &self.$children {
          match child {
            vml::$choice::TextBox(text_box) => collect_typed_vml_textbox(model, text_box),
            vml::$choice::ImageData(image_data) => {
              collect_typed_vml_image_data(model, image_data);
            }
            vml::$choice::ClientData(client_data) => {
              collect_typed_vml_client_data(model, client_data);
            }
            _ => {}
          }
        }
      }
    }
  };
}

impl_vml_shape_element!(Shape, shape_choice, ShapeChoice);
impl_vml_shape_element!(Arc, arc_choice, ArcChoice);
impl_vml_shape_element!(Curve, curve_choice, CurveChoice);
impl_vml_shape_element!(ImageFile, image_file_choice, ImageFileChoice);
impl_vml_shape_element!(Line, line_choice, LineChoice);
impl_vml_shape_element!(Oval, oval_choice, OvalChoice);
impl_vml_shape_element!(PolyLine, poly_line_choice, PolyLineChoice);
impl_vml_shape_element!(Rectangle, rectangle_choice, RectangleChoice);
impl_vml_shape_element!(RoundRectangle, round_rectangle_choice, RoundRectangleChoice);

fn vml_shape_from_typed(shape: &impl VmlShapeElement) -> VmlShapeModel {
  let mut model = VmlShapeModel {
    style: shape.style(),
    hidden: shape.user_hidden(),
    ..VmlShapeModel::default()
  };
  if model
    .style
    .as_deref()
    .is_some_and(|style| style.contains("visibility:hidden"))
  {
    model.hidden = true;
  }
  shape.collect_model_children(&mut model);
  model.text = normalize_vml_text(&model.text);
  model
}

fn vml_group_shape_models(group: &vml::Group) -> Vec<VmlShapeModel> {
  let mut shapes = Vec::new();
  let mut group_model = VmlShapeModel {
    style: group.style.clone(),
    hidden: group.user_hidden.is_some_and(|value| value.as_bool()),
    ..VmlShapeModel::default()
  };

  for child in &group.group_choice {
    match child {
      vml::GroupChoice::Group(group) => shapes.extend(vml_group_shape_models(group)),
      vml::GroupChoice::Shape(shape) => shapes.push(vml_shape_from_typed(shape.as_ref())),
      vml::GroupChoice::Arc(shape) => shapes.push(vml_shape_from_typed(shape.as_ref())),
      vml::GroupChoice::Curve(shape) => shapes.push(vml_shape_from_typed(shape.as_ref())),
      vml::GroupChoice::ImageFile(shape) => shapes.push(vml_shape_from_typed(shape.as_ref())),
      vml::GroupChoice::Line(shape) => shapes.push(vml_shape_from_typed(shape.as_ref())),
      vml::GroupChoice::Oval(shape) => shapes.push(vml_shape_from_typed(shape.as_ref())),
      vml::GroupChoice::PolyLine(shape) => shapes.push(vml_shape_from_typed(shape.as_ref())),
      vml::GroupChoice::Rectangle(shape) => shapes.push(vml_shape_from_typed(shape.as_ref())),
      vml::GroupChoice::RoundRectangle(shape) => {
        shapes.push(vml_shape_from_typed(shape.as_ref()));
      }
      vml::GroupChoice::ClientData(client_data) => {
        collect_typed_vml_client_data(&mut group_model, client_data);
      }
      _ => {}
    }
  }

  if group_model.object_type.is_some() || group_model.anchor.is_some() {
    shapes.insert(0, group_model);
  }
  shapes
}

fn collect_typed_vml_textbox(model: &mut VmlShapeModel, text_box: &vml::TextBox) {
  let Some(vml::TextBoxChoice::XmlAny(xml)) = text_box.text_box_choice.as_ref() else {
    return;
  };
  model.text.push_str(&xml_text_content(xml));
}

fn xml_text_content(xml: &[u8]) -> String {
  let mut reader = quick_xml::Reader::from_reader(xml);
  reader.config_mut().trim_text(false);
  let mut text = String::new();
  loop {
    match reader.read_event() {
      Ok(Event::Text(value)) => {
        if let Ok(value) = value.xml10_content() {
          text.push_str(&value);
        }
      }
      Ok(Event::CData(value)) => {
        if let Ok(value) = value.xml10_content() {
          text.push_str(&value);
        }
      }
      Ok(Event::GeneralRef(value)) => {
        if let Ok(value) = value.decode() {
          let reference = format!("&{value};");
          if let Ok(value) = quick_xml::escape::unescape(&reference) {
            text.push_str(&value);
          }
        }
      }
      Ok(Event::Eof) | Err(_) => break,
      _ => {}
    }
  }
  text
}

fn collect_typed_vml_image_data(model: &mut VmlShapeModel, image_data: &vml::ImageData) {
  model.image_relationship_id = image_data
    .relationship_id
    .clone()
    .or_else(|| image_data.rel_id.clone())
    .or_else(|| image_data.picture.clone())
    .or_else(|| image_data.rel_href.clone())
    .or_else(|| image_data.id.clone());
}

fn collect_typed_vml_client_data(model: &mut VmlShapeModel, client_data: &xvml::ClientData) {
  model.object_type = Some(vml_object_type_name(client_data.object_type).to_string());
  for child in &client_data.client_data_choice {
    match child {
      xvml::ClientDataChoice::Anchor(value) => model.anchor = parse_vml_client_anchor(value),
      xvml::ClientDataChoice::PrintObject(value) => {
        model.print_object = typed_vml_bool(*value, true);
      }
      xvml::ClientDataChoice::Visible(value) => {
        model.visible = typed_vml_bool(*value, true);
      }
      xvml::ClientDataChoice::CommentRowTarget(value) => {
        model.note_row = u32::try_from(*value).ok();
      }
      xvml::ClientDataChoice::CommentColumnTarget(value) => {
        model.note_column = u32::try_from(*value).ok();
      }
      _ => {}
    }
  }
}

fn collect_vml_image_resources(
  package: &SpreadsheetDocument,
  part: &VmlDrawingPart,
) -> HashMap<String, ImageResource> {
  part
    .related_parts_of_type::<_, ImagePart>(package)
    .filter_map(|related_part| {
      Some((
        related_part.relationship_id().to_string(),
        ImageResource {
          data: related_part.part().data_to_vec(package)?.into(),
          content_type: related_part
            .part()
            .content_type(package)
            .map(str::to_string),
        },
      ))
    })
    .collect()
}

fn parse_vml_client_anchor(value: &str) -> Option<VmlClientAnchor> {
  let values = value
    .split(',')
    .map(|part| part.trim().parse::<i32>().ok())
    .collect::<Option<Vec<_>>>()?;
  if values.len() != 8 {
    return None;
  }
  Some(VmlClientAnchor {
    from_col: u32::try_from(values[0]).ok()?,
    from_col_offset_px: values[1],
    from_row: u32::try_from(values[2]).ok()?,
    from_row_offset_px: values[3],
    to_col: u32::try_from(values[4]).ok()?,
    to_col_offset_px: values[5],
    to_row: u32::try_from(values[6]).ok()?,
    to_row_offset_px: values[7],
  })
}

fn typed_vml_bool(value: xvml::BooleanEntryWithBlankValues, default: bool) -> bool {
  match value {
    xvml::BooleanEntryWithBlankValues::True | xvml::BooleanEntryWithBlankValues::T => true,
    xvml::BooleanEntryWithBlankValues::False | xvml::BooleanEntryWithBlankValues::F => false,
    xvml::BooleanEntryWithBlankValues::Empty => default,
  }
}

fn normalize_vml_text(text: &str) -> String {
  text
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
    .trim()
    .to_string()
}

fn collect_xml_namespace_attrs(
  event: &BytesStart<'_>,
  namespace_attrs: &mut Vec<(Vec<u8>, Vec<u8>)>,
) {
  for attr in event.attributes().flatten() {
    let key = attr.key.as_ref();
    if is_xml_namespace_attr(key)
      && !namespace_attrs
        .iter()
        .any(|(existing, _)| existing.as_slice() == key)
    {
      namespace_attrs.push((key.to_vec(), attr.value.as_ref().to_vec()));
    }
  }
}

fn push_missing_xml_namespace_attrs(
  event: &mut BytesStart<'static>,
  namespace_attrs: &[(Vec<u8>, Vec<u8>)],
) {
  for (key, value) in namespace_attrs {
    if !bytes_start_has_attr(event, key) {
      event.push_attribute((key.as_slice(), value.as_slice()));
    }
  }
}

fn bytes_start_has_attr(event: &BytesStart<'_>, key: &[u8]) -> bool {
  event
    .attributes()
    .flatten()
    .any(|attr| attr.key.as_ref() == key)
}

fn is_xml_namespace_attr(key: &[u8]) -> bool {
  key == b"xmlns" || key.starts_with(b"xmlns:")
}

fn xml_local_name(name: &[u8]) -> &[u8] {
  name.rsplit(|byte| *byte == b':').next().unwrap_or(name)
}

fn vml_object_type_name(value: xvml::ObjectValues) -> &'static str {
  match value {
    xvml::ObjectValues::Button => "Button",
    xvml::ObjectValues::Checkbox => "Checkbox",
    xvml::ObjectValues::Dialog => "Dialog",
    xvml::ObjectValues::Drop => "Drop",
    xvml::ObjectValues::Edit => "Edit",
    xvml::ObjectValues::GroupBox => "GBox",
    xvml::ObjectValues::Label => "Label",
    xvml::ObjectValues::AuditingLine => "LineA",
    xvml::ObjectValues::List => "List",
    xvml::ObjectValues::Movie => "Movie",
    xvml::ObjectValues::Note => "Note",
    xvml::ObjectValues::Picture => "Pict",
    xvml::ObjectValues::Radio => "Radio",
    xvml::ObjectValues::AuditingRectangle => "RectA",
    xvml::ObjectValues::Scroll => "Scroll",
    xvml::ObjectValues::Spin => "Spin",
    xvml::ObjectValues::Shape => "Shape",
    xvml::ObjectValues::Group => "Group",
    xvml::ObjectValues::Rectangle => "Rect",
  }
}

impl ControlPersistenceResourceCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &EmbeddedControlPersistencePart) -> Self {
    Self {
      binary_data_parts: part
        .embedded_control_persistence_binary_data_parts(package)
        .map(|part| BinaryResourceCatalog::from_part(&part))
        .collect(),
    }
  }
}

impl ControlPropertiesResourceCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &ControlPropertiesPart) -> Result<Self> {
    let properties = part.root_element(package)?;
    Ok(Self::from_properties(properties))
  }

  fn from_properties(properties: &x14::FormControlProperties) -> Self {
    Self {
      has_object_type: properties.object_type.is_some(),
      has_checked: properties.checked.is_some(),
      boolean_flags: bool_attr_count([
        properties.colored,
        properties.first_button,
        properties.horizontal,
        properties.just_last_x,
        properties.lock_text,
        properties.no_three_d,
        properties.no_three_d2,
        properties.multiple_lines,
        properties.vertical_bar,
        properties.password_edit,
      ]),
      numeric_fields: usize::from(properties.drop_lines.is_some())
        + usize::from(properties.scroll_bar_width.is_some())
        + usize::from(properties.incremental.is_some())
        + usize::from(properties.max.is_some())
        + usize::from(properties.min.is_some())
        + usize::from(properties.page.is_some())
        + usize::from(properties.selected.is_some())
        + usize::from(properties.val.is_some())
        + usize::from(properties.minimum_width.is_some()),
      formula_fields: usize::from(properties.fmla_group.is_some())
        + usize::from(properties.fmla_link.is_some())
        + usize::from(properties.fmla_range.is_some())
        + usize::from(properties.fmla_textbox.is_some()),
      alignment_fields: usize::from(properties.drop_style.is_some())
        + usize::from(properties.selection_type.is_some())
        + usize::from(properties.text_horizontal_align.is_some())
        + usize::from(properties.text_vertical_align.is_some())
        + usize::from(properties.edit_val.is_some()),
      list_items: properties
        .list_items
        .as_ref()
        .map_or(0, |items| items.list_item.len()),
      has_extension_list: properties.extension_list.is_some(),
      text_len: properties
        .fmla_group
        .as_ref()
        .map_or(0, |value| value.len())
        + properties.fmla_link.as_ref().map_or(0, |value| value.len())
        + properties
          .fmla_range
          .as_ref()
          .map_or(0, |value| value.len())
        + properties
          .fmla_textbox
          .as_ref()
          .map_or(0, |value| value.len())
        + properties
          .multiple_selection
          .as_ref()
          .map_or(0, |value| value.len()),
    }
  }
}

impl BinaryResourceCatalog {
  fn from_part(_part: &impl SdkPart) -> Self {
    Self
  }
}

fn bool_attr_count<const N: usize>(
  values: [Option<ooxmlsdk::simple_type::BooleanValue>; N],
) -> usize {
  values
    .into_iter()
    .filter(|value| value.is_some_and(|value| value.as_bool()))
    .count()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn vml_drawing_dispatches_known_shape_types_to_static_models() {
    vml::RoundRectangle::from_bytes(
      br#"<v:roundrect xmlns:v="urn:schemas-microsoft-com:vml" xmlns:x="urn:schemas-microsoft-com:office:excel"><v:textbox><div>Round</div></v:textbox><x:ClientData ObjectType="Button"/></v:roundrect>"#,
    )
    .expect("typed round rectangle");
    vml::Line::from_bytes(
      br#"<v:line xmlns:v="urn:schemas-microsoft-com:vml" xmlns:x="urn:schemas-microsoft-com:office:excel" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" from="0,0" to="12,12"><v:imagedata r:id="rId7"/><x:ClientData ObjectType="Pict"><x:Visible/></x:ClientData></v:line>"#,
    )
    .expect("typed line");

    let xml = br#"<xml xmlns:v="urn:schemas-microsoft-com:vml"
        xmlns:o="urn:schemas-microsoft-com:office:office"
        xmlns:x="urn:schemas-microsoft-com:office:excel"
        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
      <v:roundrect style="position:absolute;width:72pt;height:24pt">
        <v:textbox><div><font>Round &amp; ready</font></div></v:textbox>
        <x:ClientData ObjectType="Button">
          <x:Anchor>1, 2, 3, 4, 5, 6, 7, 8</x:Anchor>
          <x:PrintObject>False</x:PrintObject>
        </x:ClientData>
      </v:roundrect>
      <v:line style="position:absolute;width:12pt;height:12pt" from="0,0" to="12,12">
        <v:imagedata r:id="rId7"/>
        <x:ClientData ObjectType="Pict"><x:Visible/></x:ClientData>
      </v:line>
      <v:group>
        <v:shape style="position:absolute;width:36pt;height:18pt">
          <v:textbox><div><b>Nested</b></div></v:textbox>
        </v:shape>
      </v:group>
    </xml>"#;

    let shapes = vml_shapes(xml);
    assert_eq!(shapes.len(), 3);

    assert_eq!(shapes[0].text, "Round & ready");
    assert_eq!(shapes[0].object_type.as_deref(), Some("Button"));
    assert_eq!(
      shapes[0].anchor,
      Some(VmlClientAnchor {
        from_col: 1,
        from_col_offset_px: 2,
        from_row: 3,
        from_row_offset_px: 4,
        to_col: 5,
        to_col_offset_px: 6,
        to_row: 7,
        to_row_offset_px: 8,
      })
    );
    assert!(!shapes[0].print_object);

    assert_eq!(shapes[1].image_relationship_id.as_deref(), Some("rId7"));
    assert_eq!(shapes[1].object_type.as_deref(), Some("Pict"));
    assert!(shapes[1].visible);
    assert_eq!(shapes[2].text, "Nested");
  }

  #[test]
  fn xml_text_content_is_limited_to_vml_textbox_wildcard_payload() {
    assert_eq!(
      normalize_vml_text(&xml_text_content(
        br#"<div>one<![CDATA[ two ]]><span>three &amp; four</span></div>"#
      )),
      "one two three & four"
    );
  }
}
