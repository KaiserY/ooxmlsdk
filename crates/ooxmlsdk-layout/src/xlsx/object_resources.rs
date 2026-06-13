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
  pub(crate) relationship_id: Option<String>,
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
  pub(crate) relationship_id: Option<String>,
  pub(crate) binary_data_parts: Vec<BinaryResourceCatalog>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ControlPropertiesResourceCatalog {
  pub(crate) relationship_id: Option<String>,
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
pub(crate) struct BinaryResourceCatalog {
  pub(crate) relationship_id: Option<String>,
}

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
      relationship_id: part.relationship_id().map(ToString::to_string),
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
  let mut shape_depth = 0usize;
  let mut namespace_attrs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        let is_shape = is_xml_local_name(event.name().as_ref(), b"shape");
        if let Some(shape_writer) = writer.as_mut() {
          if is_shape {
            shape_depth = shape_depth.saturating_add(1);
          }
          if shape_writer
            .write_event(Event::Start(event.into_owned()))
            .is_err()
          {
            writer = None;
            shape_depth = 0;
          }
        } else if is_shape {
          let mut shape_writer = Writer::new(Vec::new());
          let mut start = event.into_owned();
          push_missing_xml_namespace_attrs(&mut start, &namespace_attrs);
          if shape_writer.write_event(Event::Start(start)).is_ok() {
            writer = Some(shape_writer);
            shape_depth = 1;
          }
        } else {
          collect_xml_namespace_attrs(&event, &mut namespace_attrs);
        }
      }
      Ok(Event::Empty(event)) => {
        let is_shape = is_xml_local_name(event.name().as_ref(), b"shape");
        if let Some(shape_writer) = writer.as_mut() {
          if shape_writer
            .write_event(Event::Empty(event.into_owned()))
            .is_err()
          {
            writer = None;
            shape_depth = 0;
          }
        } else if is_shape {
          let mut shape_writer = Writer::new(Vec::new());
          let mut start = event.into_owned();
          push_missing_xml_namespace_attrs(&mut start, &namespace_attrs);
          if shape_writer.write_event(Event::Empty(start)).is_ok()
            && let Some(shape) = vml_shape_from_bytes(&shape_writer.into_inner())
          {
            shapes.push(shape);
          }
        } else {
          collect_xml_namespace_attrs(&event, &mut namespace_attrs);
        }
      }
      Ok(Event::End(event)) => {
        let is_shape = is_xml_local_name(event.name().as_ref(), b"shape");
        if let Some(shape_writer) = writer.as_mut()
          && shape_writer
            .write_event(Event::End(event.into_owned()))
            .is_err()
        {
          writer = None;
          shape_depth = 0;
          continue;
        }
        if is_shape && shape_depth > 0 {
          shape_depth -= 1;
          if shape_depth == 0
            && let Some(shape_writer) = writer.take()
            && let Some(shape) = vml_shape_from_bytes(&shape_writer.into_inner())
          {
            shapes.push(shape);
          }
        }
      }
      Ok(Event::Eof) => break,
      Ok(event) => {
        if let Some(shape_writer) = writer.as_mut()
          && shape_writer.write_event(event.into_owned()).is_err()
        {
          writer = None;
          shape_depth = 0;
        }
      }
      Err(_) => break,
    }
  }
  shapes
}

fn vml_shape_from_bytes(bytes: &[u8]) -> Option<VmlShapeModel> {
  let raw = raw_vml_shapes(bytes).into_iter().next();
  let Ok(shape) = vml::Shape::from_bytes(bytes) else {
    return raw;
  };

  let mut model = vml_shape_from_typed(&shape);
  if let Some(raw) = raw {
    merge_raw_vml_shape(&mut model, raw);
  }
  model.text = normalize_vml_text(&model.text);
  Some(model)
}

fn vml_shape_from_typed(shape: &vml::Shape) -> VmlShapeModel {
  let mut model = VmlShapeModel {
    style: shape.style.clone(),
    hidden: shape.user_hidden.is_some_and(|value| value.as_bool()),
    ..VmlShapeModel::default()
  };
  if model
    .style
    .as_deref()
    .is_some_and(|style| style.contains("visibility:hidden"))
  {
    model.hidden = true;
  }

  for child in &shape.shape_choice {
    match child {
      vml::ShapeChoice::ImageData(data) => collect_typed_vml_image_data(&mut model, data),
      vml::ShapeChoice::ClientData(client_data) => {
        collect_typed_vml_client_data(&mut model, client_data);
      }
      _ => {}
    }
  }
  model
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

fn merge_raw_vml_shape(model: &mut VmlShapeModel, raw: VmlShapeModel) {
  if model.text.is_empty() {
    model.text = raw.text;
  }
  if model.style.is_none() {
    model.style = raw.style;
  }
  if model.object_type.is_none() {
    model.object_type = raw.object_type;
  }
  if model.image_relationship_id.is_none() {
    model.image_relationship_id = raw.image_relationship_id;
  }
  if model.anchor.is_none() {
    model.anchor = raw.anchor;
  }
  if model.note_row.is_none() {
    model.note_row = raw.note_row;
  }
  if model.note_column.is_none() {
    model.note_column = raw.note_column;
  }
  if !raw.print_object {
    model.print_object = false;
  }
  if raw.visible {
    model.visible = true;
  }
  if raw.hidden {
    model.hidden = true;
  }
}

fn raw_vml_shapes(data: &[u8]) -> Vec<VmlShapeModel> {
  let mut reader = quick_xml::Reader::from_reader(data);
  reader.config_mut().trim_text(false);
  let mut shapes = Vec::new();
  let mut current: Option<VmlShapeModel> = None;
  let mut in_textbox = false;
  let mut in_anchor = false;
  let mut in_print_object = false;
  let mut in_visible = false;
  let mut in_note_row = false;
  let mut in_note_column = false;
  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        let name = event.name();
        if is_xml_local_name(name.as_ref(), b"shape") {
          let mut shape = VmlShapeModel::default();
          for attr in event.attributes().flatten() {
            let key = attr.key.as_ref();
            let value = String::from_utf8_lossy(attr.value.as_ref()).into_owned();
            if is_xml_local_name(key, b"style") {
              shape.style = Some(value);
            } else if is_xml_local_name(key, b"hidden") {
              shape.hidden = matches!(value.as_str(), "true" | "1" | "t");
            }
          }
          if shape
            .style
            .as_deref()
            .is_some_and(|style| style.contains("visibility:hidden"))
          {
            shape.hidden = true;
          }
          current = Some(shape);
        } else if is_xml_local_name(name.as_ref(), b"textbox") {
          in_textbox = true;
        } else if is_xml_local_name(name.as_ref(), b"imagedata") {
          collect_vml_image_relationship(current.as_mut(), event.attributes().flatten());
        } else if is_xml_local_name(name.as_ref(), b"ClientData") {
          collect_vml_object_type(current.as_mut(), event.attributes().flatten());
        } else if is_xml_local_name(name.as_ref(), b"Anchor") {
          in_anchor = true;
        } else if is_xml_local_name(name.as_ref(), b"PrintObject") {
          in_print_object = true;
        } else if is_xml_local_name(name.as_ref(), b"Visible") {
          in_visible = true;
        } else if is_xml_local_name(name.as_ref(), b"Row") {
          in_note_row = true;
        } else if is_xml_local_name(name.as_ref(), b"Column") {
          in_note_column = true;
        }
      }
      Ok(Event::Text(text)) => {
        if let Ok(value) = text.decode() {
          collect_vml_shape_text(
            current.as_mut(),
            &value,
            VmlTextContext {
              in_textbox,
              in_anchor,
              in_print_object,
              in_visible,
              in_note_row,
              in_note_column,
            },
          );
        }
      }
      Ok(Event::CData(text)) => {
        if let Ok(value) = text.decode() {
          collect_vml_shape_text(
            current.as_mut(),
            &value,
            VmlTextContext {
              in_textbox,
              in_anchor,
              in_print_object,
              in_visible,
              in_note_row,
              in_note_column,
            },
          );
        }
      }
      Ok(Event::Empty(event)) => {
        if is_xml_local_name(event.name().as_ref(), b"imagedata") {
          collect_vml_image_relationship(current.as_mut(), event.attributes().flatten());
        } else if is_xml_local_name(event.name().as_ref(), b"ClientData") {
          collect_vml_object_type(current.as_mut(), event.attributes().flatten());
        } else if is_xml_local_name(event.name().as_ref(), b"Visible")
          && let Some(shape) = current.as_mut()
        {
          shape.visible = true;
        }
      }
      Ok(Event::End(event)) => {
        let name = event.name();
        if is_xml_local_name(name.as_ref(), b"textbox") {
          in_textbox = false;
        } else if is_xml_local_name(name.as_ref(), b"Anchor") {
          in_anchor = false;
        } else if is_xml_local_name(name.as_ref(), b"PrintObject") {
          in_print_object = false;
        } else if is_xml_local_name(name.as_ref(), b"Visible") {
          in_visible = false;
        } else if is_xml_local_name(name.as_ref(), b"Row") {
          in_note_row = false;
        } else if is_xml_local_name(name.as_ref(), b"Column") {
          in_note_column = false;
        } else if is_xml_local_name(name.as_ref(), b"shape")
          && let Some(mut shape) = current.take()
        {
          shape.text = normalize_vml_text(&shape.text);
          shapes.push(shape);
        }
      }
      Ok(Event::Eof) => break,
      Err(_) => break,
      _ => {}
    }
  }
  shapes
}

fn collect_vml_image_relationship<'a>(
  shape: Option<&mut VmlShapeModel>,
  attributes: impl Iterator<Item = quick_xml::events::attributes::Attribute<'a>>,
) {
  let Some(shape) = shape else {
    return;
  };
  for attr in attributes {
    let key = attr.key.as_ref();
    if is_xml_local_name(key, b"relid") || is_xml_local_name(key, b"id") {
      shape.image_relationship_id = Some(String::from_utf8_lossy(attr.value.as_ref()).into_owned());
    }
  }
}

fn collect_vml_object_type<'a>(
  shape: Option<&mut VmlShapeModel>,
  attributes: impl Iterator<Item = quick_xml::events::attributes::Attribute<'a>>,
) {
  let Some(shape) = shape else {
    return;
  };
  for attr in attributes {
    if is_xml_local_name(attr.key.as_ref(), b"ObjectType") {
      shape.object_type = Some(String::from_utf8_lossy(attr.value.as_ref()).into_owned());
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
          data: related_part.part().data_to_vec(package)?,
          content_type: related_part
            .part()
            .content_type(package)
            .map(str::to_string),
        },
      ))
    })
    .collect()
}

#[derive(Clone, Copy, Debug)]
struct VmlTextContext {
  in_textbox: bool,
  in_anchor: bool,
  in_print_object: bool,
  in_visible: bool,
  in_note_row: bool,
  in_note_column: bool,
}

fn collect_vml_shape_text(shape: Option<&mut VmlShapeModel>, value: &str, context: VmlTextContext) {
  let Some(shape) = shape else {
    return;
  };
  if context.in_textbox {
    shape.text.push_str(value);
  } else if context.in_anchor {
    shape.anchor = parse_vml_client_anchor(value);
  } else if context.in_print_object {
    shape.print_object = decode_vml_bool(value, true);
  } else if context.in_visible {
    shape.visible = decode_vml_bool(value, true);
  } else if context.in_note_row {
    shape.note_row = value.trim().parse::<u32>().ok();
  } else if context.in_note_column {
    shape.note_column = value.trim().parse::<u32>().ok();
  }
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

fn decode_vml_bool(value: &str, default: bool) -> bool {
  let value = value.trim();
  if value.is_empty() {
    return default;
  }
  !matches!(value, "false" | "False" | "FALSE" | "0" | "f" | "F")
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

fn is_xml_local_name(name: &[u8], expected: &[u8]) -> bool {
  xml_local_name(name).eq_ignore_ascii_case(expected)
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
      relationship_id: part.relationship_id().map(ToString::to_string),
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
    Ok(Self::from_properties(
      part.relationship_id().map(ToString::to_string),
      properties,
    ))
  }

  fn from_properties(
    relationship_id: Option<String>,
    properties: &x14::FormControlProperties,
  ) -> Self {
    Self {
      relationship_id,
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
  fn from_part(part: &impl SdkPart) -> Self {
    Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
    }
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
