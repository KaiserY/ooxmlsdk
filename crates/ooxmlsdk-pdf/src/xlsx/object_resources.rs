use ooxmlsdk::parts::control_properties_part::ControlPropertiesPart;
use ooxmlsdk::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::vml_drawing_part::VmlDrawingPart;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main as x14;
use ooxmlsdk::sdk::SdkPart;
use quick_xml::events::Event;

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
  pub(crate) legacy_diagram_texts: usize,
  pub(crate) shapes: Vec<VmlShapeModel>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct VmlShapeModel {
  pub(crate) text: String,
  pub(crate) style: Option<String>,
  pub(crate) hidden: bool,
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
      legacy_diagram_texts: part.legacy_diagram_text_parts(package).count(),
      shapes,
    }
  }
}

fn vml_shapes(data: &[u8]) -> Vec<VmlShapeModel> {
  let mut reader = quick_xml::Reader::from_reader(data);
  reader.config_mut().trim_text(false);
  let mut shapes = Vec::new();
  let mut current: Option<VmlShapeModel> = None;
  let mut in_textbox = false;
  loop {
    match reader.read_event() {
      Ok(Event::Start(event)) => {
        let name = event.name();
        if name.as_ref().ends_with(b"shape") {
          let mut shape = VmlShapeModel::default();
          for attr in event.attributes().flatten() {
            let key = attr.key.as_ref();
            let value = String::from_utf8_lossy(attr.value.as_ref()).into_owned();
            if key.ends_with(b"style") {
              shape.style = Some(value);
            } else if key.ends_with(b"hidden") {
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
        } else if name.as_ref().ends_with(b"textbox") {
          in_textbox = true;
        }
      }
      Ok(Event::Text(text)) => {
        if in_textbox
          && let Some(shape) = current.as_mut()
          && let Ok(value) = text.decode()
        {
          shape.text.push_str(&value);
        }
      }
      Ok(Event::CData(text)) => {
        if in_textbox
          && let Some(shape) = current.as_mut()
          && let Ok(value) = text.decode()
        {
          shape.text.push_str(&value);
        }
      }
      Ok(Event::End(event)) => {
        let name = event.name();
        if name.as_ref().ends_with(b"textbox") {
          in_textbox = false;
        } else if name.as_ref().ends_with(b"shape")
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

fn normalize_vml_text(text: &str) -> String {
  text
    .split_whitespace()
    .collect::<Vec<_>>()
    .join(" ")
    .trim()
    .to_string()
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
