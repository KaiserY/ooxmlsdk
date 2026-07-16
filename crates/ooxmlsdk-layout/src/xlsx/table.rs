use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::table_definition_part::TableDefinitionPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::error::Result;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableResourceCatalog {
  pub(crate) id: u32,
  pub(crate) name: Option<String>,
  pub(crate) display_name: String,
  pub(crate) reference: String,
  pub(crate) table_type: Option<x::TableValues>,
  pub(crate) header_rows: u32,
  pub(crate) totals_rows: u32,
  pub(crate) columns: Vec<TableColumnModel>,
  pub(crate) has_auto_filter: bool,
  pub(crate) has_sort_state: bool,
  pub(crate) style: TableStyleModel,
  pub(crate) has_extensions: bool,
  pub(crate) query_tables: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableColumnModel {
  pub(crate) id: u32,
  pub(crate) name: String,
  pub(crate) unique_name: Option<String>,
  pub(crate) totals_row_function: Option<x::TotalsRowFunctionValues>,
  pub(crate) totals_row_label: Option<String>,
  pub(crate) query_table_field_id: Option<u32>,
  pub(crate) has_calculated_formula: bool,
  pub(crate) has_totals_formula: bool,
  pub(crate) has_xml_column_properties: bool,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TableStyleModel {
  pub(crate) name: Option<String>,
  pub(crate) show_first_column: bool,
  pub(crate) show_last_column: bool,
  pub(crate) show_row_stripes: bool,
  pub(crate) show_column_stripes: bool,
}

impl TableResourceCatalog {
  pub(crate) fn from_part(
    package: &mut SpreadsheetDocument,
    part: &TableDefinitionPart,
  ) -> Result<Self> {
    let table = part.root_element(package)?;
    Ok(Self {
      id: table.id,
      name: table.name.clone(),
      display_name: table.display_name.clone(),
      reference: table.reference.clone(),
      table_type: table.table_type,
      header_rows: table.header_row_count.unwrap_or(1),
      totals_rows: table.totals_row_count.unwrap_or(0),
      columns: table
        .table_columns
        .table_column
        .iter()
        .map(TableColumnModel::from_table_column)
        .collect(),
      has_auto_filter: table.auto_filter.is_some(),
      has_sort_state: table.sort_state.is_some(),
      style: table
        .table_style_info
        .as_ref()
        .map(TableStyleModel::from_table_style_info)
        .unwrap_or_default(),
      has_extensions: table.table_extension_list.is_some(),
      query_tables: part.query_table_parts(package).count(),
    })
  }
}

impl TableColumnModel {
  fn from_table_column(column: &x::TableColumn) -> Self {
    Self {
      id: column.id,
      name: column.name.clone(),
      unique_name: column.unique_name.clone(),
      totals_row_function: column.totals_row_function,
      totals_row_label: column.totals_row_label.clone(),
      query_table_field_id: column.query_table_field_id,
      has_calculated_formula: column.calculated_column_formula.is_some(),
      has_totals_formula: column.totals_row_formula.is_some(),
      has_xml_column_properties: column.xml_column_properties.is_some(),
      has_extensions: column.extension_list.is_some(),
    }
  }
}

impl TableStyleModel {
  fn from_table_style_info(style: &x::TableStyleInfo) -> Self {
    Self {
      name: style.name.clone(),
      show_first_column: style.show_first_column.is_some_and(|value| value.as_bool()),
      show_last_column: style.show_last_column.is_some_and(|value| value.as_bool()),
      show_row_stripes: style.show_row_stripes.is_some_and(|value| value.as_bool()),
      show_column_stripes: style
        .show_column_stripes
        .is_some_and(|value| value.as_bool()),
    }
  }
}
