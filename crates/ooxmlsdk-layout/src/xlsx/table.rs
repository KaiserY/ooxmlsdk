use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::table_definition_part::TableDefinitionPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::error::Result;
use crate::model::{BorderStyle, RgbColor, TextStyle};

use super::styles::{BorderRecord, StylesCatalog, TableStyleRecord};
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
  differential_format_ids: [Option<u32>; 12],
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
  if let Some(style) = styles.table_style(name) {
    return custom_table_style(table, styles, style, range, address);
  }
  match name {
    "TableStyleLight1" => light1_style(table, styles, range, address),
    "TableStyleLight2" => light_accent_outline_style(table, styles, range, address, 4),
    "TableStyleLight3" => light_accent_outline_style(table, styles, range, address, 5),
    "TableStyleLight4" => light_accent_outline_style(table, styles, range, address, 6),
    "TableStyleLight5" => light_accent_outline_style(table, styles, range, address, 7),
    "TableStyleLight6" => light_accent_outline_style(table, styles, range, address, 8),
    "TableStyleLight7" => light_accent_outline_style(table, styles, range, address, 9),
    "TableStyleLight8" => light_accent_grid_style(table, styles, range, address, 1),
    "TableStyleLight9" => light_accent_grid_style(table, styles, range, address, 4),
    "TableStyleLight10" => light_accent_grid_style(table, styles, range, address, 5),
    "TableStyleLight11" => light_accent_grid_style(table, styles, range, address, 6),
    "TableStyleLight12" => light_accent_grid_style(table, styles, range, address, 7),
    "TableStyleLight13" => light_accent_grid_style(table, styles, range, address, 8),
    "TableStyleLight14" => light_accent_grid_style(table, styles, range, address, 9),
    "TableStyleLight15" => light_tinted_grid_style(table, styles, range, address, 1),
    "TableStyleLight16" => light_tinted_grid_style(table, styles, range, address, 4),
    "TableStyleLight17" => light_tinted_grid_style(table, styles, range, address, 5),
    "TableStyleLight18" => light_tinted_grid_style(table, styles, range, address, 6),
    "TableStyleLight19" => light_tinted_grid_style(table, styles, range, address, 7),
    "TableStyleLight20" => light_tinted_grid_style(table, styles, range, address, 8),
    "TableStyleLight21" => light_tinted_grid_style(table, styles, range, address, 9),
    "TableStyleMedium1" => medium_header_fill_style(
      table,
      styles,
      range,
      address,
      1,
      Some((0, -0.149_998_474_074_526_2)),
    ),
    "TableStyleMedium2" => medium2_style(table, styles, range, address),
    "TableStyleMedium3" => medium_header_fill_style(
      table,
      styles,
      range,
      address,
      5,
      Some((5, 0.799_981_688_894_314_4)),
    ),
    "TableStyleMedium4" => medium_header_fill_style(
      table,
      styles,
      range,
      address,
      6,
      Some((6, 0.799_981_688_894_314_4)),
    ),
    "TableStyleMedium5" => medium_header_fill_style(
      table,
      styles,
      range,
      address,
      7,
      Some((7, 0.799_981_688_894_314_4)),
    ),
    "TableStyleMedium6" => medium_header_fill_style(
      table,
      styles,
      range,
      address,
      8,
      Some((8, 0.799_981_688_894_314_4)),
    ),
    "TableStyleMedium7" => medium_header_fill_style(
      table,
      styles,
      range,
      address,
      9,
      Some((9, 0.799_981_688_894_314_4)),
    ),
    "TableStyleMedium9" => medium9_style(table, styles, range, address),
    "TableStyleMedium21" => medium_dark_header_style(table, styles, range, address, 9),
    "TableStyleMedium24" => medium_tinted_grid_style(table, styles, range, address, 5),
    "TableStyleDark11" => dark_split_accent_style(table, styles, range, address, 8, 9),
    _ => BuiltinTableCellStyle::default(),
  }
}

fn custom_table_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  style: &TableStyleRecord,
  table_range: CellRange,
  address: CellAddress,
) -> BuiltinTableCellStyle {
  // ECMA-376 table style elements use the specification precedence captured
  // by POI's TableStyleType: later elements override earlier ones. Resolve
  // each applicable region first, then merge its DXF in that order.
  const PRECEDENCE: [x::TableStyleValues; 13] = [
    x::TableStyleValues::WholeTable,
    x::TableStyleValues::FirstColumnStripe,
    x::TableStyleValues::SecondColumnStripe,
    x::TableStyleValues::FirstRowStripe,
    x::TableStyleValues::SecondRowStripe,
    x::TableStyleValues::LastColumn,
    x::TableStyleValues::FirstColumn,
    x::TableStyleValues::HeaderRow,
    x::TableStyleValues::TotalRow,
    x::TableStyleValues::FirstHeaderCell,
    x::TableStyleValues::LastHeaderCell,
    x::TableStyleValues::FirstTotalCell,
    x::TableStyleValues::LastTotalCell,
  ];

  let mut result = BuiltinTableCellStyle::default();
  for element_type in PRECEDENCE {
    let Some(element) = style
      .elements
      .iter()
      .find(|element| element.r#type == element_type)
    else {
      continue;
    };
    let Some(region) =
      table_style_element_region(table, style, table_range, address, element.r#type)
    else {
      continue;
    };
    let Some(format_id) = element.format_id else {
      continue;
    };
    merge_custom_table_differential(&mut result, styles, format_id, region, address);
  }
  result
}

fn table_style_element_region(
  table: &TableResourceCatalog,
  style: &TableStyleRecord,
  table_range: CellRange,
  address: CellAddress,
  element_type: x::TableStyleValues,
) -> Option<CellRange> {
  let single_column = |col| {
    CellRange::new(
      CellAddress {
        col,
        row: table_range.start.row,
      },
      CellAddress {
        col,
        row: table_range.end.row,
      },
    )
  };
  let single_cell = |col, row| CellRange::new(CellAddress { col, row }, CellAddress { col, row });
  match element_type {
    x::TableStyleValues::WholeTable => Some(table_range),
    x::TableStyleValues::FirstColumn if table.style.show_first_column => {
      Some(single_column(table_range.start.col))
    }
    x::TableStyleValues::LastColumn if table.style.show_last_column => {
      Some(single_column(table_range.end.col))
    }
    x::TableStyleValues::HeaderRow if table.header_rows > 0 => Some(CellRange::new(
      table_range.start,
      CellAddress {
        col: table_range.end.col,
        row: table_range
          .start
          .row
          .saturating_add(table.header_rows.saturating_sub(1)),
      },
    )),
    x::TableStyleValues::TotalRow if table.totals_rows > 0 => Some(CellRange::new(
      CellAddress {
        col: table_range.start.col,
        row: table_range
          .end
          .row
          .saturating_sub(table.totals_rows.saturating_sub(1)),
      },
      table_range.end,
    )),
    x::TableStyleValues::FirstHeaderCell if table.header_rows > 0 => {
      Some(single_cell(table_range.start.col, table_range.start.row))
    }
    x::TableStyleValues::LastHeaderCell if table.header_rows > 0 => {
      Some(single_cell(table_range.end.col, table_range.start.row))
    }
    x::TableStyleValues::FirstTotalCell if table.totals_rows > 0 => {
      Some(single_cell(table_range.start.col, table_range.end.row))
    }
    x::TableStyleValues::LastTotalCell if table.totals_rows > 0 => {
      Some(single_cell(table_range.end.col, table_range.end.row))
    }
    x::TableStyleValues::FirstRowStripe | x::TableStyleValues::SecondRowStripe
      if table.style.show_row_stripes =>
    {
      stripe_region(
        style,
        table_range,
        address,
        true,
        element_type == x::TableStyleValues::FirstRowStripe,
        table.header_rows,
      )
    }
    x::TableStyleValues::FirstColumnStripe | x::TableStyleValues::SecondColumnStripe
      if table.style.show_column_stripes =>
    {
      stripe_region(
        style,
        table_range,
        address,
        false,
        element_type == x::TableStyleValues::FirstColumnStripe,
        0,
      )
    }
    _ => None,
  }
  .filter(|region| region.contains(address))
}

fn stripe_region(
  style: &TableStyleRecord,
  table_range: CellRange,
  address: CellAddress,
  rows: bool,
  first: bool,
  leading_rows: u32,
) -> Option<CellRange> {
  let first_type = if rows {
    x::TableStyleValues::FirstRowStripe
  } else {
    x::TableStyleValues::FirstColumnStripe
  };
  let second_type = if rows {
    x::TableStyleValues::SecondRowStripe
  } else {
    x::TableStyleValues::SecondColumnStripe
  };
  let size = |kind| {
    style
      .elements
      .iter()
      .find(|element| element.r#type == kind)
      .map_or(1, |element| element.size.max(1))
  };
  let first_size = size(first_type);
  let second_size = size(second_type);
  let start = if rows {
    table_range.start.row.saturating_add(leading_rows)
  } else {
    table_range.start.col
  };
  let target = if rows { address.row } else { address.col };
  let end = if rows {
    table_range.end.row
  } else {
    table_range.end.col
  };
  let period = first_size.saturating_add(second_size);
  if target < start || target > end || period == 0 {
    return None;
  }
  let offset = target - start;
  let period_start = start.saturating_add(offset / period * period);
  let (stripe_start, stripe_size) = if first {
    (period_start, first_size)
  } else {
    (period_start.saturating_add(first_size), second_size)
  };
  let stripe_end = stripe_start
    .saturating_add(stripe_size.saturating_sub(1))
    .min(end);
  if target < stripe_start || target > stripe_end {
    return None;
  }
  Some(if rows {
    CellRange::new(
      CellAddress {
        col: table_range.start.col,
        row: stripe_start,
      },
      CellAddress {
        col: table_range.end.col,
        row: stripe_end,
      },
    )
  } else {
    CellRange::new(
      CellAddress {
        col: stripe_start,
        row: table_range.start.row,
      },
      CellAddress {
        col: stripe_end,
        row: table_range.end.row,
      },
    )
  })
}

fn merge_custom_table_differential(
  result: &mut BuiltinTableCellStyle,
  styles: &StylesCatalog,
  format_id: u32,
  region: CellRange,
  address: CellAddress,
) {
  if let Some(slot) = result
    .differential_format_ids
    .iter_mut()
    .find(|slot| slot.is_none())
  {
    *slot = Some(format_id);
  }
  if let Some(fill) = styles.differential_fill_color(format_id) {
    result.fill = Some(fill);
  }
  let Some(border) = styles.differential_borders(format_id) else {
    return;
  };
  if address.col == region.start.col && border.left.is_some() {
    result.borders.left = border.left;
  }
  if address.col == region.end.col && border.right.is_some() {
    result.borders.right = border.right;
  }
  if address.row == region.start.row && border.top.is_some() {
    result.borders.top = border.top;
  }
  if address.row == region.end.row && border.bottom.is_some() {
    result.borders.bottom = border.bottom;
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

fn light_accent_grid_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
  accent_theme: u32,
) -> BuiltinTableCellStyle {
  // POI presetTableStyles.xml and LibreOffice defaulttablestyles.inc:
  // Light8..14 share one definition, varying only the accent theme. The
  // whole-table DXF draws the outside box; both stripe DXFs add the internal
  // top/left rules when their table options are enabled.
  let accent = styles.theme_color(accent_theme, 0.0);
  let light1 = styles.theme_color(0, 0.0);
  let dark1 = styles.theme_color(1, 0.0);
  let mut result = BuiltinTableCellStyle {
    text_color: dark1,
    borders: outer_table_rules(range, address, accent, 0.5),
    ..BuiltinTableCellStyle::default()
  };

  if table.style.show_row_stripes
    && address.row >= range.start.row.saturating_add(table.header_rows)
  {
    result.borders.top = border(accent, 0.5, false);
  }
  if table.style.show_column_stripes {
    result.borders.left = border(accent, 0.5, false);
  }
  apply_emphasized_edge_columns(table, range, address, dark1, &mut result);

  if is_header(table, range, address) {
    result.fill = accent;
    result.text_color = light1;
    result.bold = true;
  } else if is_total(table, range, address) {
    result.text_color = dark1;
    result.bold = true;
    result.borders.top = border(accent, 1.0, true);
  }
  result
}

fn light_accent_outline_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
  accent_theme: u32,
) -> BuiltinTableCellStyle {
  // POI presetTableStyles.xml Light2..7: darkened accent text, accent
  // top/bottom rules, and a tint-0.8 first row/column stripe.
  let accent = styles.theme_color(accent_theme, 0.0);
  let dark_accent = styles.theme_color(accent_theme, -0.249_977_111_117_893);
  let stripe = styles.theme_color(accent_theme, 0.799_981_688_894_314_4);
  let mut result = BuiltinTableCellStyle {
    text_color: dark_accent,
    borders: horizontal_outer_table_rules(range, address, accent, 0.5),
    ..BuiltinTableCellStyle::default()
  };
  apply_first_table_stripe_fill(table, range, address, stripe, &mut result);
  apply_emphasized_edge_columns(table, range, address, dark_accent, &mut result);
  if is_header(table, range, address) {
    result.bold = true;
    result.borders.bottom = border(accent, 0.5, false);
  } else if is_total(table, range, address) {
    result.bold = true;
    result.borders.top = border(accent, 0.5, false);
  }
  result
}

fn light_tinted_grid_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
  accent_theme: u32,
) -> BuiltinTableCellStyle {
  // POI presetTableStyles.xml Light15..21: accent grid, a tint-0.8
  // first stripe, and unfilled bold header/total rows.
  let accent = styles.theme_color(accent_theme, 0.0);
  let dark1 = styles.theme_color(1, 0.0);
  let stripe = if accent_theme == 1 {
    styles.theme_color(0, -0.149_998_474_074_526_2)
  } else {
    styles.theme_color(accent_theme, 0.799_981_688_894_314_4)
  };
  let mut result = BuiltinTableCellStyle {
    text_color: dark1,
    borders: grid_table_rules(accent, 0.5),
    ..BuiltinTableCellStyle::default()
  };
  apply_first_table_stripe_fill(table, range, address, stripe, &mut result);
  apply_emphasized_edge_columns(table, range, address, dark1, &mut result);
  if is_header(table, range, address) {
    result.bold = true;
    result.borders.bottom = border(accent, 1.5, false);
  } else if is_total(table, range, address) {
    result.bold = true;
    result.borders.top = border(accent, 1.0, true);
  }
  result
}

fn medium_header_fill_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
  accent_theme: u32,
  stripe_theme: Option<(u32, f64)>,
) -> BuiltinTableCellStyle {
  // POI presetTableStyles.xml Medium1..7. Medium2 remains on its calibrated
  // path above; the remaining family members use this equivalent preset.
  let accent = styles.theme_color(accent_theme, 0.0);
  let light1 = styles.theme_color(0, 0.0);
  let dark1 = styles.theme_color(1, 0.0);
  let rule = styles.theme_color(accent_theme, 0.399_975_585_192_419_2);
  let stripe = stripe_theme.and_then(|(theme, tint)| styles.theme_color(theme, tint));
  let mut result = BuiltinTableCellStyle {
    text_color: dark1,
    borders: all_table_rules(range, address, rule, 0.5),
    ..BuiltinTableCellStyle::default()
  };
  apply_first_table_stripe_fill(table, range, address, stripe, &mut result);
  apply_emphasized_edge_columns(table, range, address, dark1, &mut result);
  if is_header(table, range, address) {
    result.fill = accent;
    result.text_color = light1;
    result.bold = true;
  } else if is_total(table, range, address) {
    result.text_color = dark1;
    result.bold = true;
    result.borders.top = border(accent, 1.0, true);
  }
  result
}

fn medium_dark_header_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
  accent_theme: u32,
) -> BuiltinTableCellStyle {
  // POI presetTableStyles.xml Medium15..21. This corpus currently exercises
  // Medium21; the definition uses a solid accent header/edge and light stripe.
  let accent = styles.theme_color(accent_theme, 0.0);
  let light1 = styles.theme_color(0, 0.0);
  let dark1 = styles.theme_color(1, 0.0);
  let stripe = styles.theme_color(0, -0.149_998_474_074_526_2);
  let mut result = BuiltinTableCellStyle {
    text_color: dark1,
    borders: horizontal_outer_table_rules(range, address, dark1, 1.5),
    ..BuiltinTableCellStyle::default()
  };
  apply_first_table_stripe_fill(table, range, address, stripe, &mut result);
  if (table.style.show_first_column && address.col == range.start.col)
    || (table.style.show_last_column && address.col == range.end.col)
  {
    result.fill = accent;
    result.text_color = light1;
    result.bold = true;
  }
  if is_header(table, range, address) {
    result.fill = accent;
    result.text_color = light1;
    result.bold = true;
    result.borders.bottom = border(dark1, 1.5, false);
  } else if is_total(table, range, address) {
    result.borders.top = border(dark1, 1.0, true);
  }
  result
}

fn medium_tinted_grid_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
  accent_theme: u32,
) -> BuiltinTableCellStyle {
  // POI presetTableStyles.xml Medium22..28: tint-0.8 base, tint-0.6
  // first stripe, and tint-0.4 full cell grid.
  let accent = styles.theme_color(accent_theme, 0.0);
  let dark1 = styles.theme_color(1, 0.0);
  let mut result = BuiltinTableCellStyle {
    fill: styles.theme_color(accent_theme, 0.799_981_688_894_314_4),
    text_color: dark1,
    borders: grid_table_rules(
      styles.theme_color(accent_theme, 0.399_975_585_192_419_2),
      0.5,
    ),
    ..BuiltinTableCellStyle::default()
  };
  apply_first_table_stripe_fill(
    table,
    range,
    address,
    styles.theme_color(accent_theme, 0.599_993_896_298_104_8),
    &mut result,
  );
  apply_emphasized_edge_columns(table, range, address, dark1, &mut result);
  if is_header(table, range, address) {
    result.bold = true;
  } else if is_total(table, range, address) {
    result.bold = true;
    result.borders.top = border(accent, 1.5, false);
  }
  result
}

fn dark_split_accent_style(
  table: &TableResourceCatalog,
  styles: &StylesCatalog,
  range: CellRange,
  address: CellAddress,
  body_theme: u32,
  header_theme: u32,
) -> BuiltinTableCellStyle {
  // POI presetTableStyles.xml Dark11: tint-0.8 body, tint-0.6 first stripe,
  // and a contrasting Accent6 header.
  let dark1 = styles.theme_color(1, 0.0);
  let light1 = styles.theme_color(0, 0.0);
  let mut result = BuiltinTableCellStyle {
    fill: styles.theme_color(body_theme, 0.799_981_688_894_314_4),
    ..BuiltinTableCellStyle::default()
  };
  apply_first_table_stripe_fill(
    table,
    range,
    address,
    styles.theme_color(body_theme, 0.599_993_896_298_104_8),
    &mut result,
  );
  apply_emphasized_edge_columns(table, range, address, dark1, &mut result);
  if is_header(table, range, address) {
    result.fill = styles.theme_color(header_theme, 0.0);
    result.text_color = light1;
  } else if is_total(table, range, address) {
    result.text_color = dark1;
    result.bold = true;
    result.borders.top = border(dark1, 1.0, true);
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

fn is_first_column_stripe(
  table: &TableResourceCatalog,
  range: CellRange,
  address: CellAddress,
) -> bool {
  table.style.show_column_stripes
    && address.col >= range.start.col
    && (address.col - range.start.col).is_multiple_of(2)
}

fn apply_first_table_stripe_fill(
  table: &TableResourceCatalog,
  range: CellRange,
  address: CellAddress,
  fill: Option<RgbColor>,
  result: &mut BuiltinTableCellStyle,
) {
  if is_first_row_stripe(table, range, address) || is_first_column_stripe(table, range, address) {
    result.fill = fill;
  }
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

fn outer_table_rules(
  range: CellRange,
  address: CellAddress,
  color: Option<RgbColor>,
  width_pt: f32,
) -> BorderRecord {
  let line = border(color, width_pt, false);
  BorderRecord {
    left: (address.col == range.start.col).then_some(line).flatten(),
    right: (address.col == range.end.col).then_some(line).flatten(),
    top: (address.row == range.start.row).then_some(line).flatten(),
    bottom: (address.row == range.end.row).then_some(line).flatten(),
  }
}

fn horizontal_outer_table_rules(
  range: CellRange,
  address: CellAddress,
  color: Option<RgbColor>,
  width_pt: f32,
) -> BorderRecord {
  let line = border(color, width_pt, false);
  BorderRecord {
    top: (address.row == range.start.row).then_some(line).flatten(),
    bottom: (address.row == range.end.row).then_some(line).flatten(),
    ..BorderRecord::default()
  }
}

fn grid_table_rules(color: Option<RgbColor>, width_pt: f32) -> BorderRecord {
  let line = border(color, width_pt, false);
  BorderRecord {
    left: line,
    right: line,
    top: line,
    bottom: line,
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
  styles: &StylesCatalog,
  text_style: &mut TextStyle,
) {
  for format_id in style.differential_format_ids.into_iter().flatten() {
    styles.apply_differential_text_style(format_id, text_style);
  }
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

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use super::super::styles::{
    DifferentialFormatRecord, FillRecord, FontRecord, TableStyleElementRecord,
  };
  use super::*;

  fn custom_table(style_name: &str) -> TableResourceCatalog {
    TableResourceCatalog {
      id: 1,
      name: Some("Table1".to_string()),
      display_name: "Table1".to_string(),
      reference: "A1:C5".to_string(),
      range: Some(CellRange::new(
        CellAddress { col: 1, row: 1 },
        CellAddress { col: 3, row: 5 },
      )),
      table_type: None,
      header_rows: 1,
      totals_rows: 1,
      columns: Vec::new(),
      has_auto_filter: true,
      has_sort_state: false,
      style: TableStyleModel {
        name: Some(style_name.to_string()),
        show_row_stripes: true,
        ..TableStyleModel::default()
      },
      has_extensions: false,
      query_tables: 0,
    }
  }

  #[test]
  fn custom_table_style_applies_spec_precedence_and_stripe_sizes() {
    let red = RgbColor { r: 255, g: 0, b: 0 };
    let blue = RgbColor { r: 0, g: 0, b: 255 };
    let white = RgbColor {
      r: 255,
      g: 255,
      b: 255,
    };
    let mut styles = StylesCatalog::default();
    styles.differential_format_records = vec![
      DifferentialFormatRecord {
        fill: Some(FillRecord { color: Some(red) }),
        ..DifferentialFormatRecord::default()
      },
      DifferentialFormatRecord {
        fill: Some(FillRecord { color: Some(blue) }),
        font: Some(FontRecord {
          name: Some(Arc::from("Arial")),
          color: Some(white),
          bold: true,
          ..FontRecord::default()
        }),
        ..DifferentialFormatRecord::default()
      },
    ];
    styles.table_style_records = vec![TableStyleRecord {
      name: "CustomStyle".to_string(),
      elements: vec![
        TableStyleElementRecord {
          r#type: x::TableStyleValues::WholeTable,
          size: 1,
          format_id: Some(0),
        },
        TableStyleElementRecord {
          r#type: x::TableStyleValues::HeaderRow,
          size: 1,
          format_id: Some(1),
        },
        TableStyleElementRecord {
          r#type: x::TableStyleValues::FirstRowStripe,
          size: 2,
          format_id: Some(1),
        },
      ],
    }];
    let table = custom_table("CustomStyle");

    let header = builtin_table_style_for_address(
      std::slice::from_ref(&table),
      &styles,
      CellAddress { col: 2, row: 1 },
    );
    assert_eq!(header.fill, Some(blue));
    let mut header_text = TextStyle::default();
    apply_builtin_table_text_style(header, &styles, &mut header_text);
    assert_eq!(header_text.font_family.as_deref(), Some("Arial"));
    assert_eq!(header_text.color, white);
    assert!(header_text.bold);

    let first_stripe = builtin_table_style_for_address(
      std::slice::from_ref(&table),
      &styles,
      CellAddress { col: 2, row: 3 },
    );
    assert_eq!(first_stripe.fill, Some(blue));
    let second_stripe = builtin_table_style_for_address(
      std::slice::from_ref(&table),
      &styles,
      CellAddress { col: 2, row: 4 },
    );
    assert_eq!(second_stripe.fill, Some(red));
  }

  #[test]
  fn light_accent_grid_family_routes_header_and_total_regions() {
    for style_name in [
      "TableStyleLight8",
      "TableStyleLight9",
      "TableStyleLight10",
      "TableStyleLight14",
    ] {
      let table = custom_table(style_name);
      let styles = StylesCatalog::default();
      let header = builtin_table_style_for_address(
        std::slice::from_ref(&table),
        &styles,
        CellAddress { col: 2, row: 1 },
      );
      let total = builtin_table_style_for_address(
        std::slice::from_ref(&table),
        &styles,
        CellAddress { col: 2, row: 5 },
      );
      assert!(header.bold, "{style_name} header");
      assert!(total.bold, "{style_name} total");
    }
  }

  #[test]
  fn corpus_builtin_table_style_families_route_their_regions() {
    for style_name in [
      "TableStyleLight3",
      "TableStyleLight21",
      "TableStyleMedium1",
      "TableStyleMedium4",
      "TableStyleMedium6",
      "TableStyleMedium7",
      "TableStyleMedium21",
      "TableStyleMedium24",
      "TableStyleDark11",
    ] {
      let table = custom_table(style_name);
      let styles = StylesCatalog::default();
      let total = builtin_table_style_for_address(
        std::slice::from_ref(&table),
        &styles,
        CellAddress { col: 2, row: 5 },
      );
      if style_name != "TableStyleMedium21" {
        assert!(total.bold, "{style_name} total");
      }
    }
  }
}
