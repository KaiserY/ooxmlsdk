use ooxmlsdk::parts::control_properties_part::ControlPropertiesPart;
use ooxmlsdk::parts::embedded_control_persistence_part::EmbeddedControlPersistencePart;
use std::collections::HashMap;

use ooxmlsdk::parts::image_part::ImagePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::vml_drawing_part::VmlDrawingPart;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main as x14;
use ooxmlsdk::sdk::SdkPart;
use quick_xml::events::Event;

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
  let mut current: Option<VmlShapeModel> = None;
  let mut in_textbox = false;
  let mut in_client_data = false;
  let mut in_anchor = false;
  let mut in_print_object = false;
  let mut in_visible = false;
  let mut in_note_row = false;
  let mut in_note_column = false;
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
        } else if name.as_ref().ends_with(b"imagedata") {
          collect_vml_image_relationship(current.as_mut(), event.attributes().flatten());
        } else if name.as_ref().ends_with(b"ClientData") {
          in_client_data = true;
          collect_vml_object_type(current.as_mut(), event.attributes().flatten());
        } else if name.as_ref().ends_with(b"Anchor") {
          in_anchor = true;
        } else if name.as_ref().ends_with(b"PrintObject") {
          in_print_object = true;
        } else if name.as_ref().ends_with(b"Visible") {
          in_visible = true;
        } else if name.as_ref().ends_with(b"Row") {
          in_note_row = true;
        } else if name.as_ref().ends_with(b"Column") {
          in_note_column = true;
        }
      }
      Ok(Event::Text(text)) => {
        if let Ok(value) = text.decode() {
          collect_vml_shape_text(
            current.as_mut(),
            &value,
            in_textbox,
            in_client_data,
            in_anchor,
            in_print_object,
            in_visible,
            in_note_row,
            in_note_column,
          );
        }
      }
      Ok(Event::CData(text)) => {
        if let Ok(value) = text.decode() {
          collect_vml_shape_text(
            current.as_mut(),
            &value,
            in_textbox,
            in_client_data,
            in_anchor,
            in_print_object,
            in_visible,
            in_note_row,
            in_note_column,
          );
        }
      }
      Ok(Event::Empty(event)) => {
        if event.name().as_ref().ends_with(b"imagedata") {
          collect_vml_image_relationship(current.as_mut(), event.attributes().flatten());
        } else if event.name().as_ref().ends_with(b"ClientData") {
          collect_vml_object_type(current.as_mut(), event.attributes().flatten());
        } else if event.name().as_ref().ends_with(b"Visible")
          && let Some(shape) = current.as_mut()
        {
          shape.visible = true;
        }
      }
      Ok(Event::End(event)) => {
        let name = event.name();
        if name.as_ref().ends_with(b"textbox") {
          in_textbox = false;
        } else if name.as_ref().ends_with(b"ClientData") {
          in_client_data = false;
        } else if name.as_ref().ends_with(b"Anchor") {
          in_anchor = false;
        } else if name.as_ref().ends_with(b"PrintObject") {
          in_print_object = false;
        } else if name.as_ref().ends_with(b"Visible") {
          in_visible = false;
        } else if name.as_ref().ends_with(b"Row") {
          in_note_row = false;
        } else if name.as_ref().ends_with(b"Column") {
          in_note_column = false;
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

fn collect_vml_image_relationship<'a>(
  shape: Option<&mut VmlShapeModel>,
  attributes: impl Iterator<Item = quick_xml::events::attributes::Attribute<'a>>,
) {
  let Some(shape) = shape else {
    return;
  };
  for attr in attributes {
    let key = attr.key.as_ref();
    if key.ends_with(b"relid") || key.ends_with(b"id") {
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
    if attr.key.as_ref().ends_with(b"ObjectType") {
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

fn collect_vml_shape_text(
  shape: Option<&mut VmlShapeModel>,
  value: &str,
  in_textbox: bool,
  _in_client_data: bool,
  in_anchor: bool,
  in_print_object: bool,
  in_visible: bool,
  in_note_row: bool,
  in_note_column: bool,
) {
  let Some(shape) = shape else {
    return;
  };
  if in_textbox {
    shape.text.push_str(value);
  } else if in_anchor {
    shape.anchor = parse_vml_client_anchor(value);
  } else if in_print_object {
    shape.print_object = decode_vml_bool(value, true);
  } else if in_visible {
    shape.visible = decode_vml_bool(value, true);
  } else if in_note_row {
    shape.note_row = value.trim().parse::<u32>().ok();
  } else if in_note_column {
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
