use ooxmlsdk::schemas::schemas_openxmlformats_org_markup_compatibility_2006 as mc;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkType;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetObjectCatalog {
  pub(crate) ole_objects: Vec<OleObjectModel>,
  pub(crate) controls: Vec<ControlModel>,
  pub(crate) unknown_controls: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OleObjectModel {
  pub(crate) shape_id: u32,
  pub(crate) relationship_id: Option<String>,
  pub(crate) prog_id: Option<String>,
  pub(crate) link: Option<String>,
  pub(crate) data_or_view_aspect: Option<x::DataViewAspectValues>,
  pub(crate) ole_update: Option<x::OleUpdateValues>,
  pub(crate) auto_load: bool,
  pub(crate) show_as_icon: bool,
  pub(crate) has_embedded_properties: bool,
  pub(crate) property_flags: usize,
  pub(crate) property_text_len: usize,
  pub(crate) anchor: Option<ObjectAnchorModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ControlModel {
  pub(crate) shape_id: u32,
  pub(crate) relationship_id: String,
  pub(crate) name: Option<String>,
  pub(crate) has_control_properties: bool,
  pub(crate) property_flags: usize,
  pub(crate) property_text_len: usize,
  pub(crate) anchor: Option<ObjectAnchorModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ObjectAnchorModel {
  pub(crate) from_column: i32,
  pub(crate) from_row: i32,
  pub(crate) to_column: i32,
  pub(crate) to_row: i32,
  pub(crate) from_column_offset_emu: i64,
  pub(crate) from_row_offset_emu: i64,
  pub(crate) to_column_offset_emu: i64,
  pub(crate) to_row_offset_emu: i64,
  pub(crate) move_with_cells: bool,
  pub(crate) size_with_cells: bool,
  pub(crate) z_order: Option<u32>,
}

impl SheetObjectCatalog {
  pub(crate) fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    // importOleObject/importControl collect worksheet XML object records and
    // register them with the VML drawing owner instead of treating only the
    // relationship parts as printable objects.
    let mut catalog = Self::default();
    if let Some(objects) = worksheet.ole_objects.as_ref() {
      for child in &objects.xml_children {
        let model = match child {
          x::OleObjectsChoice::OleObject(object) => Some(OleObjectModel::from_ole_object(object)),
          x::OleObjectsChoice::AlternateContent(content) => {
            ole_object_from_alternate_content(content)
          }
        };
        if let Some(model) = model {
          catalog.ole_objects.push(model);
        }
      }
    }
    if let Some(controls) = worksheet.controls.as_ref() {
      for child in &controls.xml_children {
        let model = match child {
          x::ControlsChoice::Control(control) => Some(ControlModel::from_control(control)),
          x::ControlsChoice::AlternateContent(content) => control_from_alternate_content(content),
        };
        if let Some(model) = model {
          catalog.controls.push(model);
        } else {
          catalog.unknown_controls += 1;
        }
      }
    }
    catalog
  }

  pub(crate) fn anchor_for_shape_id(&self, shape_id: u32) -> Option<&ObjectAnchorModel> {
    self
      .ole_objects
      .iter()
      .find(|object| object.shape_id == shape_id)
      .and_then(|object| object.anchor.as_ref())
      .or_else(|| {
        self
          .controls
          .iter()
          .find(|control| control.shape_id == shape_id)
          .and_then(|control| control.anchor.as_ref())
      })
  }

  pub(crate) fn anchor_for_vml_shape(
    &self,
    shape: &super::object_resources::VmlShapeModel,
  ) -> Option<&ObjectAnchorModel> {
    shape
      .shape_id
      .as_deref()
      .into_iter()
      .chain(shape.id.as_deref())
      .find_map(vml_shape_numeric_id)
      .and_then(|shape_id| self.anchor_for_shape_id(shape_id))
  }
}

fn vml_shape_numeric_id(value: &str) -> Option<u32> {
  value
    .rsplit(|ch: char| !ch.is_ascii_digit())
    .next()
    .filter(|suffix| !suffix.is_empty())
    .and_then(|suffix| suffix.parse().ok())
}

fn ole_object_from_alternate_content(content: &mc::AlternateContent) -> Option<OleObjectModel> {
  let mut selected = None;
  for child in alternate_content_child_xml(content) {
    let Ok(object) = x::OleObject::from_bytes(child) else {
      continue;
    };
    let model = OleObjectModel::from_ole_object(&object);
    // Apache POI's XSSFSheet.readOleObject() deliberately prefers the Choice
    // copy containing objectPr over the fallback copy with the same shapeId.
    // The former owns the modern xdr anchor and preview relationship.
    if selected.as_ref().is_none_or(|current: &OleObjectModel| {
      !current.has_embedded_properties && model.has_embedded_properties
    }) {
      selected = Some(model);
    }
  }
  selected
}

fn control_from_alternate_content(content: &mc::AlternateContent) -> Option<ControlModel> {
  let mut selected = None;
  for child in alternate_content_child_xml(content) {
    let Ok(control) = x::Control::from_bytes(child) else {
      continue;
    };
    let model = ControlModel::from_control(&control);
    if selected.as_ref().is_none_or(|current: &ControlModel| {
      !current.has_control_properties && model.has_control_properties
    }) {
      selected = Some(model);
    }
  }
  selected
}

fn alternate_content_child_xml(content: &mc::AlternateContent) -> impl Iterator<Item = &[u8]> {
  content
    .alternate_content_choice
    .iter()
    .flat_map(|branch| match branch {
      mc::AlternateContentChoice::Choice(choice) => choice.xml_children.iter(),
      mc::AlternateContentChoice::Fallback(fallback) => fallback.xml_children.iter(),
    })
    .map(AsRef::as_ref)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn alternate_content_prefers_ole_object_properties_anchor() {
    let worksheet = x::Worksheet::from_bytes(
      br#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
          xmlns:xdr="http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing"
          xmlns:x14="http://schemas.microsoft.com/office/spreadsheetml/2009/9/main"
          xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006">
        <sheetData/>
        <oleObjects>
          <mc:AlternateContent>
            <mc:Choice Requires="x14">
              <oleObject progId="Package" shapeId="1025" r:id="rId4">
                <objectPr defaultSize="0" r:id="rId5">
                  <anchor moveWithCells="1">
                    <from><xdr:col>1</xdr:col><xdr:colOff>0</xdr:colOff><xdr:row>1</xdr:row><xdr:rowOff>0</xdr:rowOff></from>
                    <to><xdr:col>4</xdr:col><xdr:colOff>384810</xdr:colOff><xdr:row>3</xdr:row><xdr:rowOff>156210</xdr:rowOff></to>
                  </anchor>
                </objectPr>
              </oleObject>
            </mc:Choice>
            <mc:Fallback><oleObject progId="Package" shapeId="1025" r:id="rId4"/></mc:Fallback>
          </mc:AlternateContent>
        </oleObjects>
      </worksheet>"#,
    )
    .expect("worksheet");

    let catalog = SheetObjectCatalog::from_worksheet(&worksheet);
    assert_eq!(catalog.ole_objects.len(), 1);
    let object = &catalog.ole_objects[0];
    assert_eq!(object.shape_id, 1025);
    assert!(object.has_embedded_properties);
    let anchor = object.anchor.as_ref().expect("objectPr anchor");
    assert_eq!(anchor.from_column, 1);
    assert_eq!(anchor.from_row, 1);
    assert_eq!(anchor.to_column, 4);
    assert_eq!(anchor.to_row, 3);
    assert_eq!(anchor.from_column_offset_emu, 0);
    assert_eq!(anchor.from_row_offset_emu, 0);
    assert_eq!(anchor.to_column_offset_emu, 384_810);
    assert_eq!(anchor.to_row_offset_emu, 156_210);

    let vml_shape = super::super::object_resources::VmlShapeModel {
      id: Some("_x0000_s1025".into()),
      ..Default::default()
    };
    assert_eq!(catalog.anchor_for_vml_shape(&vml_shape), Some(anchor));
  }
}

impl OleObjectModel {
  fn from_ole_object(object: &x::OleObject) -> Self {
    let properties = object.embedded_object_properties.as_ref();
    Self {
      shape_id: object.shape_id,
      relationship_id: object.id.clone(),
      prog_id: object.prog_id.clone(),
      link: object.link.clone(),
      data_or_view_aspect: object.data_or_view_aspect,
      ole_update: object.ole_update,
      auto_load: object.auto_load.is_some_and(|value| value.as_bool()),
      show_as_icon: object
        .data_or_view_aspect
        .is_some_and(|value| value == x::DataViewAspectValues::DataViewAspectIcon),
      has_embedded_properties: properties.is_some(),
      property_flags: properties.map_or(0, |properties| {
        bool_attr_count([
          properties.locked,
          properties.default_size,
          properties.print,
          properties.disabled,
          properties.ui_object,
          properties.auto_fill,
          properties.auto_line,
          properties.auto_pict,
          properties.dde,
        ])
      }),
      property_text_len: properties.map_or(0, |properties| {
        properties.r#macro.as_ref().map_or(0, |value| value.len())
          + properties.alt_text.as_ref().map_or(0, |value| value.len())
          + properties.r_id.as_ref().map_or(0, |value| value.len())
      }),
      anchor: properties
        .map(|properties| ObjectAnchorModel::from_anchor(&properties.object_anchor)),
    }
  }
}

impl ControlModel {
  fn from_control(control: &x::Control) -> Self {
    let properties = control.control_properties.as_ref();
    Self {
      shape_id: control.shape_id,
      relationship_id: control.id.clone(),
      name: control.name.clone(),
      has_control_properties: properties.is_some(),
      property_flags: properties.map_or(0, |properties| {
        bool_attr_count([
          properties.locked,
          properties.default_size,
          properties.print,
          properties.disabled,
          properties.recalc_always,
          properties.ui_object,
          properties.auto_fill,
          properties.auto_line,
          properties.auto_pict,
        ])
      }),
      property_text_len: properties.map_or(0, |properties| {
        properties.r#macro.as_ref().map_or(0, |value| value.len())
          + properties.alt_text.as_ref().map_or(0, |value| value.len())
          + properties
            .linked_cell
            .as_ref()
            .map_or(0, |value| value.len())
          + properties
            .list_fill_range
            .as_ref()
            .map_or(0, |value| value.len())
          + properties.cf.as_ref().map_or(0, |value| value.len())
          + properties.r_id.as_ref().map_or(0, |value| value.len())
      }),
      anchor: properties
        .map(|properties| ObjectAnchorModel::from_anchor(&properties.object_anchor)),
    }
  }
}

impl ObjectAnchorModel {
  fn from_anchor(anchor: &x::ObjectAnchor) -> Self {
    Self {
      from_column: anchor.from_marker.column_id,
      from_row: anchor.from_marker.row_id,
      to_column: anchor.to_marker.column_id,
      to_row: anchor.to_marker.row_id,
      from_column_offset_emu: anchor.from_marker.column_offset.to_emu(),
      from_row_offset_emu: anchor.from_marker.row_offset.to_emu(),
      to_column_offset_emu: anchor.to_marker.column_offset.to_emu(),
      to_row_offset_emu: anchor.to_marker.row_offset.to_emu(),
      move_with_cells: anchor.move_with_cells.is_some_and(|value| value.as_bool()),
      size_with_cells: anchor.size_with_cells.is_some_and(|value| value.as_bool()),
      z_order: anchor.z_order,
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
