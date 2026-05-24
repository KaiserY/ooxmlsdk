use ooxmlsdk::parts::connections_part::ConnectionsPart;
use ooxmlsdk::parts::query_table_part::QueryTablePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

use crate::error::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ConnectionsCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) connections: Vec<ConnectionModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ConnectionModel {
  pub(crate) id: u32,
  pub(crate) name: Option<String>,
  pub(crate) connection_type: Option<u32>,
  pub(crate) source_file: Option<String>,
  pub(crate) connection_file: Option<String>,
  pub(crate) refreshed_version: u8,
  pub(crate) refresh_on_load: bool,
  pub(crate) save_data: Option<bool>,
  pub(crate) has_database_properties: bool,
  pub(crate) has_olap_properties: bool,
  pub(crate) has_web_query_properties: bool,
  pub(crate) has_text_properties: bool,
  pub(crate) parameter_count: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct QueryTableCatalog {
  pub(crate) query_tables: Vec<QueryTableModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct QueryTableModel {
  pub(crate) relationship_id: Option<String>,
  pub(crate) name: String,
  pub(crate) connection_id: u32,
  pub(crate) refresh_fields: usize,
  pub(crate) deleted_fields: usize,
  pub(crate) has_sort_state: bool,
  pub(crate) has_refresh_extensions: bool,
  pub(crate) has_extensions: bool,
  pub(crate) flag_count: usize,
}

impl ConnectionsCatalog {
  pub(crate) fn from_part(
    package: &mut SpreadsheetDocument,
    part: Option<ConnectionsPart>,
  ) -> Result<Self> {
    // Source: LibreOffice sc/source/filter/oox/connectionsbuffer.cxx.
    // Connections are workbook-global and query tables resolve them by id.
    let Some(part) = part else {
      return Ok(Self::default());
    };
    let relationship_id = part.relationship_id().map(ToString::to_string);
    let connections = part.root_element(package)?;
    Ok(Self {
      relationship_id,
      connections: connections
        .connection
        .iter()
        .map(ConnectionModel::from_connection)
        .collect(),
    })
  }
}

impl ConnectionModel {
  fn from_connection(connection: &x::Connection) -> Self {
    Self {
      id: connection.id,
      name: connection.name.clone(),
      connection_type: connection.r#type,
      source_file: connection.source_file.clone(),
      connection_file: connection.connection_file.clone(),
      refreshed_version: connection.refreshed_version,
      refresh_on_load: connection
        .refresh_on_load
        .is_some_and(|value| value.as_bool()),
      save_data: connection.save_data.map(|value| value.as_bool()),
      has_database_properties: connection.database_properties.is_some(),
      has_olap_properties: connection.olap_properties.is_some(),
      has_web_query_properties: connection.web_query_properties.is_some(),
      has_text_properties: connection.text_properties.is_some(),
      parameter_count: connection
        .parameters
        .as_ref()
        .map_or(0, |parameters| parameters.parameter.len()),
      has_extensions: connection.connection_extension_list.is_some(),
    }
  }
}

impl QueryTableCatalog {
  pub(crate) fn from_parts(
    package: &mut SpreadsheetDocument,
    parts: &[QueryTablePart],
  ) -> Result<Self> {
    Ok(Self {
      query_tables: parts
        .iter()
        .map(|part| QueryTableModel::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
    })
  }
}

impl QueryTableModel {
  fn from_part(package: &mut SpreadsheetDocument, part: &QueryTablePart) -> Result<Self> {
    let relationship_id = part.relationship_id().map(ToString::to_string);
    let query_table = part.root_element(package)?;
    let refresh = query_table.query_table_refresh.as_deref();
    Ok(Self {
      relationship_id,
      name: query_table.name.clone(),
      connection_id: query_table.connection_id,
      refresh_fields: refresh.map_or(0, |refresh| {
        refresh.query_table_fields.query_table_field.len()
      }),
      deleted_fields: refresh
        .and_then(|refresh| refresh.query_table_deleted_fields.as_ref())
        .map_or(0, |fields| fields.deleted_field.len()),
      has_sort_state: refresh.is_some_and(|refresh| refresh.sort_state.is_some()),
      has_refresh_extensions: refresh.is_some_and(|refresh| refresh.extension_list.is_some()),
      has_extensions: query_table.query_table_extension_list.is_some(),
      flag_count: query_table_flag_count(query_table),
    })
  }
}

fn query_table_flag_count(query_table: &x::QueryTable) -> usize {
  [
    query_table.headers,
    query_table.row_numbers,
    query_table.disable_refresh,
    query_table.background_refresh,
    query_table.first_background_refresh,
    query_table.refresh_on_load,
    query_table.fill_formulas,
    query_table.remove_data_on_save,
    query_table.disable_edit,
    query_table.preserve_formatting,
    query_table.adjust_column_width,
    query_table.intermediate,
    query_table.apply_number_formats,
    query_table.apply_border_formats,
    query_table.apply_font_formats,
    query_table.apply_pattern_formats,
    query_table.apply_alignment_formats,
    query_table.apply_width_height_formats,
  ]
  .iter()
  .filter(|value| value.is_some_and(|value| value.as_bool()))
  .count()
    + usize::from(query_table.grow_shrink_type.is_some())
    + usize::from(query_table.auto_format_id.is_some())
}
