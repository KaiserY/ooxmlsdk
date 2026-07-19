use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::table_definition_part::TableDefinitionPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::error::Result;
use crate::model::{BorderStyle, RgbColor, TextStyle};

use super::styles::{BorderRecord, StylesCatalog};
use super::worksheet::{CellAddress, CellRange};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableResourceCatalog {
  pub(crate) id: u32,
  pub(crate) name: Option<String>,
  pub(crate) display_name: String,
  pub(crate) reference: String,
  pub(crate) range: Option<CellRange>,
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
      range: CellRange::parse_a1_range(&table.reference),
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

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct BuiltinTableCellStyle {
  pub(crate) fill: Option<RgbColor>,
  pub(crate) text_color: Option<RgbColor>,
  pub(crate) bold: bool,
  pub(crate) borders: BorderRecord,
}

pub(crate) fn builtin_table_style_for_address(
  tables: &[TableResourceCatalog],
  styles: &StylesCatalog,
  address: CellAddress,
) -> BuiltinTableCellStyle {
  let Some(table) = tables
    .iter()
    .find(|table| table.range.is_some_and(|range| range.contains(address)))
  else {
    return BuiltinTableCellStyle::default();
  };
  let Some(range) = table.range else {
    return BuiltinTableCellStyle::default();
  };
  let Some(name) = table.style.name.as_deref() else {
    return BuiltinTableCellStyle::default();
  };
  match name {
    "TableStyleLight1" => light1_style(table, styles, range, address),
    "TableStyleMedium2" => medium2_style(table, styles, range, address),
    "TableStyleMedium9" => medium9_style(table, styles, range, address),
    _ => BuiltinTableCellStyle::default(),
  }
}

fn light1_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
) -> BuiltinTableCellStyle {
  // LibreOffice defaulttablestyles.inc, generated from ECMA-376
  // presetTableStyles.xml: Light1 uses bold dk1 for headers/totals/edge
  // columns, a dk1 rule around the table, and a shaded lt1 first stripe.
  let dark1 = styles.theme_color(1, 0.0);
  let mut result = BuiltinTableCellStyle {
    borders: horizontal_table_rule(range, address, dark1, 0.5),
    ..BuiltinTableCellStyle::default()
  };
  if is_header(table, range, address) {
    result.bold = true;
    result.text_color = dark1;
    result.borders.bottom = border(dark1, 0.5, false);
  } else if is_total(table, range, address) {
    result.bold = true;
    result.text_color = dark1;
    result.borders.top = border(dark1, 0.5, false);
  } else {
    apply_emphasized_edge_columns(table, range, address, dark1, &mut result);
    if is_first_row_stripe(table, range, address) {
      result.fill = styles.theme_color(0, -0.149_998_474_074_526_2);
    }
  }
  result
}

fn medium2_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
) -> BuiltinTableCellStyle {
  // LibreOffice defaulttablestyles.inc: Medium2 header/total are Accent1
  // with bold lt1 text; body rules are Accent1 tint 0.4 and the first stripe
  // is Accent1 tint 0.8.
  let accent1 = styles.theme_color(4, 0.0);
  let light1 = styles.theme_color(0, 0.0);
  let dark1 = styles.theme_color(1, 0.0);
  let rule = styles.theme_color(4, 0.399_975_585_192_419_2);
  let mut result = BuiltinTableCellStyle {
    borders: all_table_rules(range, address, rule, 0.5),
    ..BuiltinTableCellStyle::default()
  };
  if is_header(table, range, address) {
    result.fill = accent1;
    result.text_color = light1;
    result.bold = true;
  } else if is_total(table, range, address) {
    // Medium2 totalRow is DXF 156: bold dk1 with a double Accent1 top rule;
    // unlike Medium9 it deliberately has no fill.
    result.fill = None;
    result.text_color = dark1;
    result.bold = true;
    result.borders.top = border(accent1, 1.0, true);
  } else {
    apply_emphasized_edge_columns(table, range, address, dark1, &mut result);
    if is_first_row_stripe(table, range, address) {
      result.fill = styles.theme_color(4, 0.799_981_688_894_314_4);
    }
  }
  result
}

fn medium9_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
) -> BuiltinTableCellStyle {
  // LibreOffice defaulttablestyles.inc: Medium9 has an Accent1 tint 0.8
  // whole-table fill, tint 0.6 first stripes, and Accent1 header/total rows.
  let accent1 = styles.theme_color(4, 0.0);
  let light1 = styles.theme_color(0, 0.0);
  let dark1 = styles.theme_color(1, 0.0);
  let mut result = BuiltinTableCellStyle {
    fill: styles.theme_color(4, 0.799_981_688_894_314_4),
    borders: inner_table_rules(range, address, light1, 0.5),
    text_color: dark1,
    ..BuiltinTableCellStyle::default()
  };
  if is_header(table, range, address) {
    result.fill = accent1;
    result.text_color = light1;
    result.bold = true;
  } else if is_total(table, range, address) {
    result.fill = accent1;
    result.text_color = light1;
    result.bold = true;
    result.borders.bottom = border(light1, 1.5, false);
  } else {
    apply_emphasized_edge_columns(table, range, address, dark1, &mut result);
    if is_first_row_stripe(table, range, address) {
      result.fill = styles.theme_color(4, 0.599_993_896_298_104_8);
    }
  }
  result
}

fn apply_emphasized_edge_columns(
  table: &TableResourceCatalog,
  range: CellRange,
  address: CellAddress,
  color: Option<RgbColor>,
  result: &mut BuiltinTableCellStyle,
) {
  if (table.style.show_first_column && address.col == range.start.col)
    || (table.style.show_last_column && address.col == range.end.col)
  {
    result.bold = true;
    result.text_color = color;
  }
}

fn is_header(table: &TableResourceCatalog, range: CellRange, address: CellAddress) -> bool {
  table.header_rows > 0 && address.row < range.start.row.saturating_add(table.header_rows)
}

fn is_total(table: &TableResourceCatalog, range: CellRange, address: CellAddress) -> bool {
  table.totals_rows > 0 && address.row > range.end.row.saturating_sub(table.totals_rows)
}

fn is_first_row_stripe(
  table: &TableResourceCatalog,
  range: CellRange,
  address: CellAddress,
) -> bool {
  if !table.style.show_row_stripes {
    return false;
  }
  let data_start = range.start.row.saturating_add(table.header_rows);
  address.row >= data_start && (address.row - data_start).is_multiple_of(2)
}

fn border(color: Option<RgbColor>, width_pt: f32, compound: bool) -> Option<BorderStyle> {
  color.map(|color| BorderStyle {
    width_pt,
    color,
    compound,
    ..BorderStyle::default()
  })
}

fn horizontal_table_rule(
  _range: CellRange,
  _address: CellAddress,
  color: Option<RgbColor>,
  width_pt: f32,
) -> BorderRecord {
  BorderRecord {
    top: border(color, width_pt, false),
    bottom: border(color, width_pt, false),
    ..BorderRecord::default()
  }
}

fn all_table_rules(
  range: CellRange,
  address: CellAddress,
  color: Option<RgbColor>,
  width_pt: f32,
) -> BorderRecord {
  let line = border(color, width_pt, false);
  BorderRecord {
    left: (address.col == range.start.col).then_some(line).flatten(),
    right: (address.col == range.end.col).then_some(line).flatten(),
    top: line,
    bottom: line,
  }
}

fn inner_table_rules(
  range: CellRange,
  address: CellAddress,
  color: Option<RgbColor>,
  width_pt: f32,
) -> BorderRecord {
  let line = border(color, width_pt, false);
  BorderRecord {
    left: (address.col > range.start.col).then_some(line).flatten(),
    right: (address.col < range.end.col).then_some(line).flatten(),
    top: (address.row > range.start.row).then_some(line).flatten(),
    bottom: (address.row < range.end.row).then_some(line).flatten(),
  }
}

pub(crate) fn apply_builtin_table_text_style(
  style: BuiltinTableCellStyle,
  text_style: &mut TextStyle,
) {
  if let Some(color) = style.text_color {
    text_style.color = color;
  }
  if style.bold {
    text_style.bold = true;
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
