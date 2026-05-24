use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

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
    // Source: LibreOffice sc/source/filter/oox/worksheetfragment.cxx
    // importOleObject/importControl collect worksheet XML object records and
    // register them with the VML drawing owner instead of treating only the
    // relationship parts as printable objects.
    let controls = worksheet.controls.as_ref();
    Self {
      ole_objects: worksheet
        .ole_objects
        .as_ref()
        .map(|objects| {
          objects
            .ole_object
            .iter()
            .map(OleObjectModel::from_ole_object)
            .collect()
        })
        .unwrap_or_default(),
      controls: controls
        .map(|controls| {
          controls
            .xml_children
            .iter()
            .filter_map(|child| match child {
              x::ControlsChoice::Control(control) => Some(ControlModel::from_control(control)),
              x::ControlsChoice::XmlAny(_) => None,
            })
            .collect()
        })
        .unwrap_or_default(),
      unknown_controls: controls.map_or(0, |controls| {
        controls
          .xml_children
          .iter()
          .filter(|child| matches!(child, x::ControlsChoice::XmlAny(_)))
          .count()
      }),
    }
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
