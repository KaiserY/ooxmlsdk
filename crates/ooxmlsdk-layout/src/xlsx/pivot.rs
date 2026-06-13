use std::collections::HashMap;

use ooxmlsdk::parts::pivot_table_part::PivotTablePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

use super::print::rendered_number_text;
use super::styles::BorderRecord;
use super::styles::StylesCatalog;
use super::text::decode_excel_escaped_text;
use super::workbook::SharedStringModel;
use super::worksheet::{CalcSheet, CellAddress, CellRange};
use crate::compat::{BorderStyle, RgbColor};
use crate::error::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PivotTableCatalog {
  pub(crate) tables: Vec<PivotTableModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PivotTableModel {
  pub(crate) relationship_id: Option<String>,
  pub(crate) name: String,
  pub(crate) cache_id: u32,
  pub(crate) location_reference: String,
  pub(crate) printable_location_reference: String,
  pub(crate) output_geometry: PivotOutputGeometry,
  pub(crate) first_header_row: u32,
  pub(crate) first_data_row: u32,
  pub(crate) first_data_column: u32,
  pub(crate) calculated_only_data_fields: bool,
  pub(crate) data_layout_axis: PivotDataLayoutAxis,
  pub(crate) style_name: Option<String>,
  pub(crate) pivot_fields: usize,
  pub(crate) row_fields: usize,
  pub(crate) column_fields: usize,
  pub(crate) page_fields: usize,
  pub(crate) data_fields: usize,
  pub(crate) filters: usize,
  pub(crate) formats: usize,
  pub(crate) compact: bool,
  pub(crate) row_grand_totals: bool,
  pub(crate) column_grand_totals: bool,
  pub(crate) row_field_indexes: Vec<i32>,
  pub(crate) column_field_indexes: Vec<i32>,
  pub(crate) row_field_names: Vec<String>,
  pub(crate) column_field_names: Vec<String>,
  pub(crate) row_field_number_format_ids: Vec<Option<u32>>,
  pub(crate) column_field_number_format_ids: Vec<Option<u32>>,
  pub(crate) data_field_names: Vec<String>,
  pub(crate) data_field_number_format_ids: Vec<Option<u32>>,
  pub(crate) data_cell_text_overrides: Vec<PivotDataCellTextOverride>,
  pub(crate) page_field_models: Vec<PivotPageFieldModel>,
  pub(crate) format_models: Vec<PivotTableFormatModel>,
  pub(crate) format_item_names: Vec<Vec<String>>,
  pub(crate) format_row_lines: Vec<PivotFormatLineData>,
  pub(crate) format_column_lines: Vec<PivotFormatLineData>,
  pub(crate) builtin_frame_ranges: Vec<PivotFrameRange>,
  pub(crate) has_cache_definition_part: bool,
  pub(crate) has_extensions: bool,
  pub(crate) flag_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PivotPageFieldModel {
  pub(crate) field_name: String,
  pub(crate) value: PivotPageFieldValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PivotDataCellTextOverride {
  pub(crate) address: CellAddress,
  pub(crate) text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PivotPageFieldValue {
  All,
  Multiple,
  Member(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PivotOutputGeometry {
  pub(crate) whole_range: CellRange,
  pub(crate) table_range: CellRange,
  pub(crate) result_range: CellRange,
  pub(crate) output_start: CellAddress,
  pub(crate) table_start: CellAddress,
  pub(crate) data_start: CellAddress,
  pub(crate) row_field_columns: u32,
  pub(crate) column_field_rows: u32,
  pub(crate) header_rows: u32,
  pub(crate) data_rows: u32,
  pub(crate) data_columns: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PivotTableFormatModel {
  pub(crate) format_id: Option<u32>,
  pub(crate) kind: PivotTableFormatKind,
  pub(crate) data_only: bool,
  pub(crate) label_only: bool,
  pub(crate) outline: bool,
  pub(crate) field_position: Option<u32>,
  pub(crate) selections: Vec<PivotTableFormatSelection>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum PivotTableFormatKind {
  #[default]
  None,
  Data,
  Label,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PivotTableFormatSelection {
  pub(crate) selected: bool,
  pub(crate) field: u32,
  pub(crate) item_indexes: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct PivotBuiltinCellStyle {
  pub(crate) bold: bool,
  pub(crate) left_align: bool,
  pub(crate) borders: BorderRecord,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PivotFormatLineData {
  line: u32,
  position: u32,
  fields: Vec<PivotFormatFieldData>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PivotFormatFieldData {
  dimension: i32,
  name: String,
  index: i32,
  is_set: bool,
  is_member: bool,
  subtotal: bool,
  continues: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PivotFormatOutputField {
  dimension: i32,
  name: String,
  index: i32,
  matches_all: bool,
  set: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PivotFrameRange {
  range: CellRange,
  horizontal_inner: bool,
  field_frame: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PivotFormatLineMatch {
  Exact,
  Maybe,
  None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PivotFormatAxis {
  Row,
  Column,
}

const PIVOT_FORMAT_DATA_DIMENSION: i32 = -2;

#[derive(Clone, Debug)]
enum PivotCacheItemValue {
  Empty,
  String(String),
  Value(f64),
  Boolean(bool),
  DateTime { serial: f64, text: String },
  Error(String),
}

impl PivotCacheItemValue {
  fn text(&self) -> String {
    match self {
      Self::Empty => String::new(),
      Self::String(text) | Self::Error(text) => text.clone(),
      Self::DateTime { text, .. } => text.clone(),
      Self::Value(value) => format_pivot_cache_number(*value),
      Self::Boolean(value) => {
        if *value {
          "TRUE".to_string()
        } else {
          "FALSE".to_string()
        }
      }
    }
  }

  fn is_empty(&self) -> bool {
    matches!(self, Self::Empty)
  }
}

impl PartialEq for PivotCacheItemValue {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (Self::Empty, Self::Empty) => true,
      (Self::String(left), Self::String(right)) | (Self::Error(left), Self::Error(right)) => {
        left == right
      }
      (Self::Value(left), Self::Value(right)) => (left - right).abs() <= f64::EPSILON,
      (Self::DateTime { serial: left, .. }, Self::DateTime { serial: right, .. }) => {
        (left - right).abs() <= f64::EPSILON
      }
      (Self::Boolean(left), Self::Boolean(right)) => left == right,
      _ => false,
    }
  }
}

impl Eq for PivotCacheItemValue {}

pub(crate) fn pivot_format_id_for_address(sheet: &CalcSheet, address: CellAddress) -> Option<u32> {
  let mut format_id = None;
  for pivot in &sheet.resources.pivot_tables.tables {
    if !pivot.output_geometry.table_range.contains(address) {
      continue;
    }
    for format in &pivot.format_models {
      if !pivot_format_kind_contains_address(pivot, format.kind, address)
        || !pivot_format_selection_matches(sheet, pivot, format, address)
      {
        continue;
      }
      if let Some(id) = format.format_id {
        format_id = Some(id);
      }
    }
  }
  format_id
}

pub(crate) fn pivot_table_contains_address(sheet: &CalcSheet, address: CellAddress) -> bool {
  sheet
    .resources
    .pivot_tables
    .tables
    .iter()
    .any(|pivot| pivot.output_geometry.table_range.contains(address))
}

pub(crate) fn pivot_builtin_style_for_address(
  sheet: &CalcSheet,
  address: CellAddress,
) -> PivotBuiltinCellStyle {
  let Some(pivot) = sheet
    .resources
    .pivot_tables
    .tables
    .iter()
    .find(|pivot| pivot.output_geometry.table_range.contains(address))
  else {
    return PivotBuiltinCellStyle::default();
  };
  let mut style = PivotBuiltinCellStyle::default();
  if pivot.output_geometry.result_range.contains(address) {
    style.bold = pivot
      .format_row_lines
      .iter()
      .find(|line| line.line == address.row)
      .is_some_and(pivot_format_line_has_subtotal)
      || pivot
        .format_column_lines
        .iter()
        .find(|line| line.line == address.col)
        .is_some_and(pivot_format_line_has_subtotal);
    style.borders = pivot_builtin_borders_for_address(pivot, address);
    return style;
  }
  if let Some(line) = pivot
    .format_row_lines
    .iter()
    .find(|line| line.line == address.row && line.position == address.col)
  {
    style.left_align = true;
    style.bold = pivot_format_line_has_subtotal(line);
    return style;
  }
  if let Some(line) = pivot
    .format_column_lines
    .iter()
    .find(|line| line.line == address.col && line.position == address.row)
  {
    style.left_align = true;
    style.bold = pivot_format_line_has_subtotal(line);
  }
  style.borders = pivot_builtin_borders_for_address(pivot, address);
  style
}

pub(crate) fn pivot_print_address(sheet: &CalcSheet, address: CellAddress) -> Option<CellAddress> {
  for pivot in &sheet.resources.pivot_tables.tables {
    let Some(location) = CellRange::parse_a1_range(&pivot.location_reference) else {
      continue;
    };
    let printable_location = pivot.output_geometry.table_range;
    if !location.contains(address) {
      continue;
    }
    let mut print_address = address;
    if pivot.first_header_row > 1 {
      // Source: LibreOffice sc/source/filter/oox/pivottablebuffer.cxx
      // clears the persisted pivot cache range and inserts ScDPOutput at the
      // location start. Cached rows before firstHeaderRow are not emitted; the
      // first cached header row maps to ScDPOutput::mnTabStartRow.
      let cached_header_row = location.start.row + pivot.first_header_row - 1;
      if address.row < cached_header_row {
        return None;
      }
      let row_shift = cached_header_row.saturating_sub(pivot.output_geometry.table_start.row);
      print_address.row = address.row.saturating_sub(row_shift);
    }
    let cached_data_start_col = location.start.col + pivot.first_data_column;
    let calc_data_start_col =
      pivot.output_geometry.table_start.col + pivot.output_geometry.row_field_columns;
    if cached_data_start_col > calc_data_start_col {
      let col_shift = cached_data_start_col - calc_data_start_col;
      if print_address.col < location.start.col + col_shift {
        return None;
      }
      print_address.col = print_address.col.saturating_sub(col_shift);
    }
    if pivot.calculated_only_data_fields
      && pivot.data_layout_axis == PivotDataLayoutAxis::Columns
      && print_address.col == location.start.col
      && print_address.row > location.start.row
    {
      let shifted = CellAddress {
        col: print_address.col,
        row: print_address.row - 1,
      };
      return printable_location.contains(shifted).then_some(shifted);
    }
    if !pivot.output_geometry.whole_range.contains(print_address) {
      return None;
    }
    if !pivot.calculated_only_data_fields {
      return Some(print_address);
    }
    let suppress = match pivot.data_layout_axis {
      PivotDataLayoutAxis::Rows => {
        print_address.row > printable_location.start.row
          && print_address.col > printable_location.start.col
      }
      PivotDataLayoutAxis::Columns | PivotDataLayoutAxis::Hidden => {
        print_address.col > printable_location.start.col
          && print_address.row > printable_location.start.row
      }
    };
    return (!suppress).then_some(print_address);
  }
  Some(address)
}

fn pivot_format_line_has_subtotal(line: &PivotFormatLineData) -> bool {
  line.fields.iter().any(|field| field.subtotal)
}

fn pivot_builtin_borders_for_address(
  pivot: &PivotTableModel,
  address: CellAddress,
) -> BorderRecord {
  let mut borders = BorderRecord::default();
  for frame in &pivot.builtin_frame_ranges {
    if !frame.range.contains(address) {
      continue;
    }
    let inner = pivot_inner_frame_border();
    let left = if frame.field_frame {
      inner
    } else if frame.range.start.col == pivot.output_geometry.table_start.col {
      pivot_outer_frame_border()
    } else {
      inner
    };
    let top = if frame.field_frame {
      inner
    } else if frame.range.start.row == pivot.output_geometry.table_start.row {
      pivot_outer_frame_border()
    } else {
      inner
    };
    let right = if frame.field_frame {
      inner
    } else if frame.range.end.col == pivot.output_geometry.table_range.end.col {
      pivot_outer_frame_border()
    } else {
      inner
    };
    let bottom = if frame.field_frame {
      inner
    } else if frame.range.end.row == pivot.output_geometry.table_range.end.row {
      pivot_outer_frame_border()
    } else {
      inner
    };
    if address.col == frame.range.start.col {
      borders.left = Some(left);
    }
    if address.row == frame.range.start.row {
      borders.top = Some(top);
    }
    if address.col == frame.range.end.col {
      borders.right = Some(right);
    }
    if address.row == frame.range.end.row {
      borders.bottom = Some(bottom);
    }
    if frame.horizontal_inner && address.row > frame.range.start.row {
      borders.top = Some(inner);
    }
  }
  borders
}

fn pivot_inner_frame_border() -> BorderStyle {
  pivot_frame_border(20)
}

fn pivot_outer_frame_border() -> BorderStyle {
  pivot_frame_border(40)
}

fn pivot_frame_border(width_twips: u16) -> BorderStyle {
  BorderStyle {
    width_pt: f32::from(width_twips) / 20.0,
    color: RgbColor { r: 0, g: 0, b: 0 },
    ..BorderStyle::default()
  }
}

fn pivot_format_kind_contains_address(
  pivot: &PivotTableModel,
  kind: PivotTableFormatKind,
  address: CellAddress,
) -> bool {
  match kind {
    PivotTableFormatKind::Data => pivot.output_geometry.result_range.contains(address),
    PivotTableFormatKind::Label => {
      pivot.output_geometry.table_range.contains(address)
        && !pivot.output_geometry.result_range.contains(address)
    }
    PivotTableFormatKind::None => false,
  }
}

fn pivot_format_selection_matches(
  _sheet: &CalcSheet,
  pivot: &PivotTableModel,
  format: &PivotTableFormatModel,
  address: CellAddress,
) -> bool {
  let max_selection_indexes = format
    .selections
    .iter()
    .map(|selection| selection.item_indexes.len().max(1))
    .max()
    .unwrap_or(1);
  (0..max_selection_indexes).any(|selection_index| {
    pivot_format_selection_index_matches(pivot, format, address, selection_index)
  })
}

fn pivot_format_selection_index_matches(
  pivot: &PivotTableModel,
  format: &PivotTableFormatModel,
  address: CellAddress,
  selection_index: usize,
) -> bool {
  let row_fields =
    pivot_format_output_fields(&pivot.row_field_indexes, format, pivot, selection_index);
  let column_fields =
    pivot_format_output_fields(&pivot.column_field_indexes, format, pivot, selection_index);
  match format.kind {
    PivotTableFormatKind::Data => {
      let row_match = pivot_format_line_match_for_data_axis(
        pivot
          .format_row_lines
          .iter()
          .find(|line| line.line == address.row),
        &row_fields,
        pivot.format_row_lines.is_empty(),
      );
      let column_match = pivot_format_line_match_for_data_axis(
        pivot
          .format_column_lines
          .iter()
          .find(|line| line.line == address.col),
        &column_fields,
        pivot.format_column_lines.is_empty(),
      );
      row_match != PivotFormatLineMatch::None && column_match != PivotFormatLineMatch::None
    }
    PivotTableFormatKind::Label => {
      pivot_format_label_line_matches(
        pivot
          .format_row_lines
          .iter()
          .find(|line| line.line == address.row && line.position == address.col),
        &row_fields,
      ) || pivot_format_label_line_matches(
        pivot
          .format_column_lines
          .iter()
          .find(|line| line.line == address.col && line.position == address.row),
        &column_fields,
      )
    }
    PivotTableFormatKind::None => false,
  }
}

fn pivot_format_output_fields(
  field_indexes: &[i32],
  format: &PivotTableFormatModel,
  pivot: &PivotTableModel,
  selection_index: usize,
) -> Vec<PivotFormatOutputField> {
  field_indexes
    .iter()
    .map(|dimension| {
      let mut output = PivotFormatOutputField {
        dimension: *dimension,
        name: String::new(),
        index: -1,
        matches_all: false,
        set: false,
      };
      let Some(selection) = format
        .selections
        .iter()
        .find(|selection| pivot_format_selection_dimension(selection) == *dimension)
      else {
        return output;
      };
      if selection.item_indexes.is_empty() {
        output.matches_all = true;
      } else {
        output.index =
          if selection.item_indexes.len() > 1 && selection.item_indexes.len() > selection_index {
            selection.item_indexes[selection_index] as i32
          } else {
            selection.item_indexes[0] as i32
          };
        output.name = if *dimension == PIVOT_FORMAT_DATA_DIMENSION {
          "DATA".to_string()
        } else {
          pivot
            .format_item_names
            .get(selection.field as usize)
            .and_then(|items| items.get(output.index as usize))
            .cloned()
            .unwrap_or_default()
        };
      }
      output.set = true;
      output
    })
    .collect()
}

fn pivot_format_selection_dimension(selection: &PivotTableFormatSelection) -> i32 {
  if selection.field == (PIVOT_FORMAT_DATA_DIMENSION as u32) {
    PIVOT_FORMAT_DATA_DIMENSION
  } else {
    selection.field as i32
  }
}

fn pivot_format_line_match_for_data_axis(
  line: Option<&PivotFormatLineData>,
  output_fields: &[PivotFormatOutputField],
  axis_has_no_fields: bool,
) -> PivotFormatLineMatch {
  if axis_has_no_fields {
    return PivotFormatLineMatch::Exact;
  }
  line
    .map(|line| pivot_format_check_line(line, output_fields, PivotTableFormatKind::Data))
    .unwrap_or(PivotFormatLineMatch::None)
}

fn pivot_format_label_line_matches(
  line: Option<&PivotFormatLineData>,
  output_fields: &[PivotFormatOutputField],
) -> bool {
  line
    .map(|line| {
      pivot_format_check_line(line, output_fields, PivotTableFormatKind::Label)
        == PivotFormatLineMatch::Exact
    })
    .unwrap_or(false)
}

fn pivot_format_check_line(
  line: &PivotFormatLineData,
  output_fields: &[PivotFormatOutputField],
  kind: PivotTableFormatKind,
) -> PivotFormatLineMatch {
  if line.fields.is_empty() && output_fields.is_empty() {
    return PivotFormatLineMatch::Exact;
  }
  let mut matches = 0usize;
  let mut maybe = 0usize;
  for (field, output) in line.fields.iter().zip(output_fields.iter()) {
    let mut field_match = false;
    let mut field_maybe = false;
    if field.dimension == output.dimension {
      if output.set {
        field_match = (output.matches_all && !field.subtotal)
          || (field.dimension == PIVOT_FORMAT_DATA_DIMENSION && field.index == output.index)
          || (field.dimension != PIVOT_FORMAT_DATA_DIMENSION && field.name == output.name);
      } else if kind == PivotTableFormatKind::Data && !field.is_member && !field.continues {
        field_match = true;
      } else {
        field_maybe = true;
      }
    }
    if !field_match && !field_maybe {
      return PivotFormatLineMatch::None;
    }
    matches += usize::from(field_match);
    maybe += usize::from(field_maybe);
  }
  if matches == line.fields.len() {
    PivotFormatLineMatch::Exact
  } else if matches + maybe == line.fields.len() {
    PivotFormatLineMatch::Maybe
  } else {
    PivotFormatLineMatch::None
  }
}

#[derive(Clone, Debug, PartialEq)]
struct PivotPageRecordFilter {
  field_index: usize,
  kind: PivotSourceFilterKind,
}

#[derive(Clone, Debug, PartialEq)]
enum PivotSourceFilterKind {
  Group(Vec<PivotCacheItemValue>),
}

#[derive(Clone, Debug, Default, PartialEq)]
struct PivotSourceCacheTable {
  rows: Vec<Vec<PivotCacheItemValue>>,
}

impl PivotSourceCacheTable {
  fn from_worksheet_source(
    worksheet: &x::Worksheet,
    source_range: CellRange,
    record_count: Option<u32>,
    shared_strings: &[SharedStringModel],
  ) -> Self {
    // Source: LibreOffice sc/source/filter/oox/pivottablebuffer.cxx
    // PivotTable::finalizeImport creates the DataPilot descriptor from the
    // worksheet source range.  The cache records are supplemental import data,
    // but source-backed pivots are recalculated from the sheet cells.
    let cells = worksheet
      .sheet_data
      .row
      .iter()
      .flat_map(|row| row.cell.iter())
      .filter_map(|cell| {
        let address = cell
          .cell_reference
          .as_deref()
          .and_then(CellAddress::parse_a1)?;
        source_range.contains(address).then_some((address, cell))
      })
      .collect::<HashMap<_, _>>();
    let last_data_row = cells
      .keys()
      .map(|address| address.row)
      .max()
      .unwrap_or(source_range.start.row);
    let record_end_row = record_count
      .map(|count| source_range.start.row.saturating_add(count))
      .unwrap_or(source_range.end.row);
    let end_row = source_range.end.row.min(last_data_row).min(record_end_row);
    let rows = (source_range.start.row + 1..=end_row)
      .map(|row| {
        (source_range.start.col..=source_range.end.col)
          .map(|col| {
            let address = CellAddress { col, row };
            cells
              .get(&address)
              .map(|cell| pivot_source_cell_value(cell, shared_strings))
              .unwrap_or(PivotCacheItemValue::Empty)
          })
          .collect()
      })
      .collect();
    Self { rows }
  }

  fn from_records(
    records: &x::PivotCacheRecords,
    cache_field_item_values: &[Vec<PivotCacheItemValue>],
  ) -> Self {
    // Source: LibreOffice sc/source/filter/oox/pivotcachebuffer.cxx
    // PivotCacheField::writeSourceDataCell.  Indexed record items are resolved
    // through shared items; missing items do not write a source cell.
    let rows = records
      .pivot_cache_record
      .iter()
      .map(|record| {
        (0..cache_field_item_values.len())
          .map(|field_index| record_cache_item_value(record, field_index, cache_field_item_values))
          .collect()
      })
      .collect();
    Self { rows }
  }

  fn item(
    &self,
    mut row_index: usize,
    field_index: usize,
    repeat_if_empty: bool,
  ) -> PivotCacheItemValue {
    loop {
      let value = self
        .rows
        .get(row_index)
        .and_then(|row| row.get(field_index))
        .cloned()
        .unwrap_or(PivotCacheItemValue::Empty);
      if !repeat_if_empty || !value.is_empty() || row_index == 0 {
        return value;
      }
      row_index -= 1;
    }
  }
}

fn pivot_source_cell_value(
  cell: &x::Cell,
  shared_strings: &[SharedStringModel],
) -> PivotCacheItemValue {
  let text = match cell.data_type {
    Some(x::CellValues::SharedString) => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .and_then(|index| index.parse::<usize>().ok())
      .and_then(|index| shared_strings.get(index))
      .map(|shared| shared.text.clone())
      .unwrap_or_default(),
    Some(x::CellValues::InlineString) => cell
      .inline_string
      .as_deref()
      .map(pivot_inline_string_text)
      .unwrap_or_default(),
    Some(x::CellValues::Boolean) => {
      let value = cell
        .cell_value
        .as_ref()
        .and_then(|value| value.xml_content.as_deref())
        .is_some_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
      return PivotCacheItemValue::Boolean(value);
    }
    Some(x::CellValues::Error) => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .map(ToString::to_string)
      .unwrap_or_default(),
    _ => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .map(decode_excel_escaped_text)
      .unwrap_or_default(),
  };
  if text.is_empty() {
    PivotCacheItemValue::Empty
  } else if cell.data_type == Some(x::CellValues::Error) {
    PivotCacheItemValue::Error(text)
  } else if cell.data_type.is_none() {
    text
      .parse::<f64>()
      .map(PivotCacheItemValue::Value)
      .unwrap_or(PivotCacheItemValue::String(text))
  } else {
    PivotCacheItemValue::String(text)
  }
}

fn pivot_inline_string_text(value: &x::InlineString) -> String {
  if let Some(text) = &value.text
    && let Some(content) = &text.xml_content
  {
    return decode_excel_escaped_text(content);
  }

  decode_excel_escaped_text(
    &value
      .run
      .iter()
      .filter_map(|run| run.text.xml_content.as_deref())
      .collect::<String>(),
  )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PivotDataLayoutAxis {
  Columns,
  Rows,
  Hidden,
}

#[derive(Clone, Copy)]
pub(crate) struct PivotTableImportContext<'a> {
  pub(crate) current_worksheet: &'a x::Worksheet,
  pub(crate) current_sheet_name: &'a str,
  pub(crate) shared_strings: &'a [SharedStringModel],
  pub(crate) styles: &'a StylesCatalog,
  pub(crate) date_1904: bool,
}

impl PivotTableCatalog {
  pub(crate) fn from_parts(
    package: &mut SpreadsheetDocument,
    parts: &[PivotTablePart],
    context: PivotTableImportContext<'_>,
  ) -> Result<Self> {
    Ok(Self {
      tables: parts
        .iter()
        .map(|part| PivotTableModel::from_part(package, part, context))
        .collect::<Result<Vec<_>>>()?,
    })
  }
}

impl PivotTableModel {
  fn from_part(
    package: &mut SpreadsheetDocument,
    part: &PivotTablePart,
    context: PivotTableImportContext<'_>,
  ) -> Result<Self> {
    let cache_definition = part
      .pivot_table_cache_definition_part(package)
      .and_then(|part| part.root_element(package).ok().cloned());
    let cache_field_names = cache_definition
      .as_ref()
      .map(pivot_cache_field_names)
      .unwrap_or_default();
    let cache_field_items = cache_definition
      .as_ref()
      .map(pivot_cache_field_items)
      .unwrap_or_default();
    let cache_field_item_values = cache_definition
      .as_ref()
      .map(pivot_cache_field_item_values)
      .unwrap_or_default();
    let cache_field_number_format_ids = cache_definition
      .as_ref()
      .map(pivot_cache_field_number_format_ids)
      .unwrap_or_default();
    let cache_field_grouped = cache_definition
      .as_ref()
      .map(pivot_cache_field_grouped)
      .unwrap_or_default();
    let source_field_number_format_ids = cache_definition
      .as_ref()
      .map(|cache| pivot_cache_source_number_format_ids(package, cache).unwrap_or_default())
      .unwrap_or_default();
    let source_field_names = cache_definition.as_ref().map_or(Ok(Vec::new()), |cache| {
      pivot_cache_current_sheet_source_field_names(
        cache,
        context.current_sheet_name,
        context.current_worksheet,
        context.shared_strings,
      )
      .map_or_else(
        || pivot_cache_source_field_names(package, cache, context.shared_strings),
        Ok,
      )
    })?;
    let source_field_number_format_codes = source_field_number_format_ids
      .iter()
      .map(|id| {
        id.and_then(|id| {
          context
            .styles
            .number_format_code(id)
            .map(ToString::to_string)
        })
      })
      .collect::<Vec<_>>();
    let source_cache = cache_definition.as_ref().and_then(|cache| {
      pivot_source_cache_table(package, cache, context.shared_strings)
        .ok()
        .flatten()
    });
    let cache_records = if let Some(cache_part) = part.pivot_table_cache_definition_part(package) {
      cache_part
        .pivot_table_cache_records_part(package)
        .and_then(|records_part| records_part.root_element(package).ok().cloned())
    } else {
      None
    };
    let calculated_cache_fields = cache_definition
      .as_ref()
      .map(calculated_cache_field_indexes)
      .unwrap_or_default();
    let stale_source_data_fields = cache_definition
      .as_ref()
      .map(|cache| stale_source_data_field_indexes(cache, &source_field_names))
      .unwrap_or_default();
    let definition = part.root_element(package)?;
    let has_cache_definition_part = !cache_field_names.is_empty();
    let unsupported_data_field_positions = unsupported_data_field_positions(
      definition.data_fields.as_ref(),
      &calculated_cache_fields,
      &stale_source_data_fields,
    );
    let (data_fields, unsupported_data_fields) = data_field_counts(
      definition.data_fields.as_ref(),
      &unsupported_data_field_positions,
    );
    let calculated_only_data_fields = data_fields > 0 && data_fields == unsupported_data_fields;
    let data_layout_axis = data_layout_axis(definition);
    let printable_location_reference = printable_location_reference(
      &definition.location.reference,
      unsupported_data_fields,
      calculated_only_data_fields,
      data_layout_axis,
    );
    let output_geometry = pivot_output_geometry(definition, &printable_location_reference);
    let row_field_indexes = row_field_indexes(definition.row_fields.as_ref());
    let column_field_indexes = column_field_indexes(definition.column_fields.as_ref());
    let format_item_names = cache_field_items.clone();
    let format_row_lines = pivot_format_axis_lines(
      definition
        .row_items
        .as_ref()
        .map(|items| items.row_item.as_slice()),
      &row_field_indexes,
      &format_item_names,
      &output_geometry,
      PivotFormatAxis::Row,
    );
    let format_column_lines = pivot_format_axis_lines(
      definition
        .column_items
        .as_ref()
        .map(|items| items.row_item.as_slice()),
      &column_field_indexes,
      &format_item_names,
      &output_geometry,
      PivotFormatAxis::Column,
    );
    let builtin_frame_ranges = pivot_builtin_frame_ranges(
      definition,
      &output_geometry,
      &format_row_lines,
      &format_column_lines,
    );
    let row_field_number_format_ids = pivot_axis_field_number_format_ids(
      definition,
      &row_field_indexes,
      &cache_field_number_format_ids,
      &source_field_number_format_ids,
    );
    let column_field_number_format_ids = pivot_axis_field_number_format_ids(
      definition,
      &column_field_indexes,
      &cache_field_number_format_ids,
      &source_field_number_format_ids,
    );
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      name: definition.name.clone(),
      cache_id: definition.cache_id,
      location_reference: definition.location.reference.clone(),
      printable_location_reference,
      output_geometry,
      first_header_row: definition.location.first_header_row,
      first_data_row: definition.location.first_data_row,
      first_data_column: definition.location.first_data_column,
      calculated_only_data_fields,
      data_layout_axis,
      style_name: definition
        .pivot_table_style
        .as_ref()
        .and_then(|style| style.name.clone())
        .or_else(|| definition.pivot_table_style_name.clone()),
      pivot_fields: definition
        .pivot_fields
        .as_ref()
        .map_or(0, |fields| fields.pivot_field.len()),
      row_fields: definition
        .row_fields
        .as_ref()
        .map_or(0, |fields| fields.field.len()),
      column_fields: definition
        .column_fields
        .as_ref()
        .map_or(0, |fields| fields.field.len()),
      page_fields: definition
        .page_fields
        .as_ref()
        .map_or(0, |fields| fields.page_field.len()),
      data_fields: definition
        .data_fields
        .as_ref()
        .map_or(0, |fields| fields.data_field.len()),
      filters: definition
        .pivot_filters
        .as_ref()
        .map_or(0, |filters| filters.pivot_filter.len()),
      formats: definition
        .formats
        .as_ref()
        .map_or(0, |formats| formats.format.len()),
      compact: definition.compact.is_none_or(|value| value.as_bool()),
      row_grand_totals: definition
        .row_grand_totals
        .is_none_or(|value| value.as_bool()),
      column_grand_totals: definition
        .column_grand_totals
        .is_none_or(|value| value.as_bool()),
      row_field_indexes,
      column_field_indexes,
      row_field_names: row_field_names(definition.row_fields.as_ref(), &cache_field_names),
      column_field_names: column_field_names(definition.column_fields.as_ref(), &cache_field_names),
      row_field_number_format_ids,
      column_field_number_format_ids,
      data_field_names: data_field_names(
        definition.data_fields.as_ref(),
        &cache_field_names,
        &unsupported_data_field_positions,
      ),
      data_field_number_format_ids: data_field_number_format_ids(
        definition,
        &cache_field_number_format_ids,
        &source_field_number_format_ids,
        &unsupported_data_field_positions,
      ),
      data_cell_text_overrides: pivot_count_data_cell_text_overrides(
        definition,
        PivotCountOverrideContext {
          records: cache_records.as_ref(),
          source_cache: source_cache.as_ref(),
          cache_field_items: &cache_field_items,
          cache_field_item_values: &cache_field_item_values,
          cache_field_grouped: &cache_field_grouped,
          source_field_number_format_codes: &source_field_number_format_codes,
          date_1904: context.date_1904,
        },
      ),
      page_field_models: page_field_models(definition, &cache_field_names, &cache_field_items),
      format_models: pivot_table_format_models(definition),
      format_item_names,
      format_row_lines,
      format_column_lines,
      builtin_frame_ranges,
      has_cache_definition_part,
      has_extensions: definition.pivot_table_definition_extension_list.is_some(),
      flag_count: pivot_table_flag_count(definition),
    })
  }
}

fn pivot_cache_field_names(cache: &x::PivotCacheDefinition) -> Vec<String> {
  cache
    .cache_fields
    .cache_field
    .iter()
    .map(|field| field.caption.clone().unwrap_or_else(|| field.name.clone()))
    .collect()
}

fn pivot_cache_field_raw_names(cache: &x::PivotCacheDefinition) -> Vec<String> {
  cache
    .cache_fields
    .cache_field
    .iter()
    .map(|field| field.name.clone())
    .collect()
}

fn pivot_cache_field_items(cache: &x::PivotCacheDefinition) -> Vec<Vec<String>> {
  cache
    .cache_fields
    .cache_field
    .iter()
    .map(|field| {
      field
        .shared_items
        .as_ref()
        .map(|shared| {
          shared
            .shared_items_choice
            .iter()
            .map(pivot_cache_shared_item_text)
            .collect()
        })
        .unwrap_or_default()
    })
    .collect()
}

fn pivot_cache_field_item_values(cache: &x::PivotCacheDefinition) -> Vec<Vec<PivotCacheItemValue>> {
  cache
    .cache_fields
    .cache_field
    .iter()
    .map(|field| {
      field
        .shared_items
        .as_ref()
        .map(|shared| {
          shared
            .shared_items_choice
            .iter()
            .map(pivot_cache_shared_item_value)
            .collect()
        })
        .unwrap_or_default()
    })
    .collect()
}

fn pivot_cache_field_number_format_ids(cache: &x::PivotCacheDefinition) -> Vec<Option<u32>> {
  cache
    .cache_fields
    .cache_field
    .iter()
    .map(|field| field.number_format_id)
    .collect()
}

fn pivot_cache_field_grouped(cache: &x::PivotCacheDefinition) -> Vec<bool> {
  cache
    .cache_fields
    .cache_field
    .iter()
    .map(|field| field.field_group.is_some())
    .collect()
}

fn pivot_cache_source_number_format_ids(
  package: &mut SpreadsheetDocument,
  cache: &x::PivotCacheDefinition,
) -> Result<Vec<Option<u32>>> {
  let Some(x::CacheSourceChoice::WorksheetSource(source)) =
    cache.cache_source.cache_source_choice.as_ref()
  else {
    return Ok(Vec::new());
  };
  let (Some(sheet_name), Some(reference)) = (source.sheet.as_deref(), source.reference.as_deref())
  else {
    return Ok(Vec::new());
  };
  let Some(source_range) = CellRange::parse_a1_range(reference) else {
    return Ok(Vec::new());
  };
  let workbook_part = package.workbook_part()?;
  let workbook = workbook_part.root_element(package)?.clone();
  let Some(workbook_sheet) = workbook
    .sheets
    .sheet
    .iter()
    .find(|sheet| sheet.name == sheet_name)
  else {
    return Ok(Vec::new());
  };
  let Some(worksheet_part) = workbook_part
    .worksheet_parts(package)
    .find(|part| part.relationship_id() == Some(workbook_sheet.id.as_str()))
  else {
    return Ok(Vec::new());
  };
  let styles = StylesCatalog::from_workbook_part(package, &workbook_part)?;
  let worksheet = worksheet_part.root_element(package)?.clone();
  let mut field_formats = Vec::new();
  for column in source_range.start.col..=source_range.end.col {
    field_formats.push(source_column_number_format_id(
      &worksheet,
      &styles,
      source_range,
      column,
    ));
  }
  Ok(field_formats)
}

fn pivot_cache_source_field_names(
  package: &mut SpreadsheetDocument,
  cache: &x::PivotCacheDefinition,
  shared_strings: &[SharedStringModel],
) -> Result<Vec<String>> {
  let Some(x::CacheSourceChoice::WorksheetSource(source)) =
    cache.cache_source.cache_source_choice.as_ref()
  else {
    return Ok(Vec::new());
  };
  let (Some(sheet_name), Some(reference)) = (source.sheet.as_deref(), source.reference.as_deref())
  else {
    return Ok(Vec::new());
  };
  let Some(source_range) = CellRange::parse_a1_range(reference) else {
    return Ok(Vec::new());
  };
  let workbook_part = package.workbook_part()?;
  let workbook = workbook_part.root_element(package)?.clone();
  let Some((workbook_sheet_index, workbook_sheet)) = workbook
    .sheets
    .sheet
    .iter()
    .enumerate()
    .find(|(_, sheet)| sheet.name == sheet_name)
  else {
    return Ok(Vec::new());
  };
  let worksheet_parts = workbook_part.worksheet_parts(package).collect::<Vec<_>>();
  let Some(worksheet_part) = worksheet_parts
    .iter()
    .find(|part| part.relationship_id() == Some(workbook_sheet.id.as_str()))
    .or_else(|| worksheet_parts.get(workbook_sheet_index))
  else {
    return Ok(Vec::new());
  };
  let worksheet = worksheet_part.root_element(package)?.clone();
  Ok(pivot_source_field_names_from_worksheet(
    &worksheet,
    source_range,
    shared_strings,
  ))
}

fn pivot_cache_current_sheet_source_field_names(
  cache: &x::PivotCacheDefinition,
  current_sheet_name: &str,
  worksheet: &x::Worksheet,
  shared_strings: &[SharedStringModel],
) -> Option<Vec<String>> {
  let x::CacheSourceChoice::WorksheetSource(source) =
    cache.cache_source.cache_source_choice.as_ref()?
  else {
    return None;
  };
  let (Some(sheet_name), Some(reference)) = (source.sheet.as_deref(), source.reference.as_deref())
  else {
    return None;
  };
  if sheet_name != current_sheet_name {
    return None;
  }
  let source_range = CellRange::parse_a1_range(reference)?;
  Some(pivot_source_field_names_from_worksheet(
    worksheet,
    source_range,
    shared_strings,
  ))
}

fn pivot_source_field_names_from_worksheet(
  worksheet: &x::Worksheet,
  source_range: CellRange,
  shared_strings: &[SharedStringModel],
) -> Vec<String> {
  let header_cells = worksheet
    .sheet_data
    .row
    .iter()
    .flat_map(|row| row.cell.iter())
    .filter_map(|cell| {
      let address = cell
        .cell_reference
        .as_deref()
        .and_then(CellAddress::parse_a1)?;
      (address.row == source_range.start.row
        && address.col >= source_range.start.col
        && address.col <= source_range.end.col)
        .then_some((address.col, cell))
    })
    .collect::<HashMap<_, _>>();
  (source_range.start.col..=source_range.end.col)
    .map(|col| {
      header_cells
        .get(&col)
        .map(|cell| pivot_source_cell_value(cell, shared_strings).text())
        .unwrap_or_default()
    })
    .collect()
}

fn pivot_source_cache_table(
  package: &mut SpreadsheetDocument,
  cache: &x::PivotCacheDefinition,
  shared_strings: &[SharedStringModel],
) -> Result<Option<PivotSourceCacheTable>> {
  let Some(x::CacheSourceChoice::WorksheetSource(source)) =
    cache.cache_source.cache_source_choice.as_ref()
  else {
    return Ok(None);
  };
  let (Some(sheet_name), Some(reference)) = (source.sheet.as_deref(), source.reference.as_deref())
  else {
    return Ok(None);
  };
  let Some(source_range) = CellRange::parse_a1_range(reference) else {
    return Ok(None);
  };
  let workbook_part = package.workbook_part()?;
  let workbook = workbook_part.root_element(package)?.clone();
  let Some((workbook_sheet_index, workbook_sheet)) = workbook
    .sheets
    .sheet
    .iter()
    .enumerate()
    .find(|(_, sheet)| sheet.name == sheet_name)
  else {
    return Ok(None);
  };
  let worksheet_parts = workbook_part.worksheet_parts(package).collect::<Vec<_>>();
  let Some(worksheet_part) = worksheet_parts
    .iter()
    .find(|part| part.relationship_id() == Some(workbook_sheet.id.as_str()))
    .or_else(|| worksheet_parts.get(workbook_sheet_index))
  else {
    return Ok(None);
  };
  let worksheet = worksheet_part.root_element(package)?.clone();
  Ok(Some(PivotSourceCacheTable::from_worksheet_source(
    &worksheet,
    source_range,
    cache.record_count,
    shared_strings,
  )))
}

fn source_column_number_format_id(
  worksheet: &x::Worksheet,
  styles: &StylesCatalog,
  source_range: CellRange,
  column: u32,
) -> Option<u32> {
  worksheet
    .sheet_data
    .row
    .iter()
    .filter(|row| {
      row.row_index.is_some_and(|row_index| {
        row_index > source_range.start.row && row_index <= source_range.end.row
      })
    })
    .flat_map(|row| row.cell.iter())
    .filter(|cell| {
      cell
        .cell_reference
        .as_deref()
        .and_then(CellAddress::parse_a1)
        .is_some_and(|address| address.col == column)
    })
    .filter_map(|cell| {
      cell
        .style_index
        .and_then(|style_index| styles.cell_xfs.get(style_index as usize))
        .and_then(|format| format.number_format_id)
        .filter(|id| *id != 0)
    })
    .next()
}

fn pivot_cache_shared_item_text(item: &x::SharedItemsChoice) -> String {
  pivot_cache_shared_item_value(item).text()
}

fn pivot_cache_shared_item_value(item: &x::SharedItemsChoice) -> PivotCacheItemValue {
  match item {
    x::SharedItemsChoice::MissingItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or(PivotCacheItemValue::Empty),
    x::SharedItemsChoice::NumberItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or(PivotCacheItemValue::Value(item.val)),
    x::SharedItemsChoice::BooleanItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or_else(|| PivotCacheItemValue::Boolean(item.val.as_bool())),
    x::SharedItemsChoice::ErrorItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or_else(|| PivotCacheItemValue::Error(item.val.clone())),
    x::SharedItemsChoice::StringItem(item) => {
      PivotCacheItemValue::String(item.caption.clone().unwrap_or_else(|| item.val.clone()))
    }
    x::SharedItemsChoice::DateTimeItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or_else(|| {
        pivot_cache_datetime_serial(&item.val)
          .map(|serial| PivotCacheItemValue::DateTime {
            serial,
            text: item.val.clone(),
          })
          .unwrap_or_else(|| PivotCacheItemValue::String(item.val.clone()))
      }),
  }
}

fn format_pivot_cache_number(value: f64) -> String {
  if value.fract() == 0.0 {
    format!("{value:.0}")
  } else {
    let mut text = value.to_string();
    if text.contains('.') {
      while text.ends_with('0') {
        text.pop();
      }
      if text.ends_with('.') {
        text.pop();
      }
    }
    text
  }
}

fn pivot_cache_datetime_serial(text: &str) -> Option<f64> {
  // Source: LibreOffice sc/source/filter/oox/pivotcachebuffer.cxx
  // PivotCacheItem::readDate/writeItemToSourceDataCell writes an actual
  // DateTime cell, and DataPilot stores it as a numeric item.
  let (date, time) = text.split_once('T')?;
  let mut date_parts = date.split('-');
  let year = date_parts.next()?.parse::<i64>().ok()?;
  let month = date_parts.next()?.parse::<i64>().ok()?;
  let day = date_parts.next()?.parse::<i64>().ok()?;
  let mut time_parts = time.split(':');
  let hour = time_parts.next()?.parse::<i64>().ok()?;
  let minute = time_parts.next()?.parse::<i64>().ok()?;
  let second = time_parts.next()?.parse::<i64>().ok()?;
  let unix_days = days_from_civil(year, month, day);
  let excel_epoch_unix_days = days_from_civil(1899, 12, 30);
  let day_serial = (unix_days - excel_epoch_unix_days) as f64;
  let time_serial = (hour * 3_600 + minute * 60 + second) as f64 / 86_400.0;
  Some(day_serial + time_serial)
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
  let year = year - i64::from(month <= 2);
  let era = if year >= 0 { year } else { year - 399 } / 400;
  let year_of_era = year - era * 400;
  let month_prime = month + if month > 2 { -3 } else { 9 };
  let day_of_year = (153 * month_prime + 2) / 5 + day - 1;
  let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
  era * 146_097 + day_of_era - 719_468
}

fn calculated_cache_field_indexes(cache: &x::PivotCacheDefinition) -> Vec<usize> {
  cache
    .cache_fields
    .cache_field
    .iter()
    .enumerate()
    .filter_map(|(index, field)| {
      field
        .formula
        .as_ref()
        .is_some_and(|formula| !formula.is_empty())
        .then_some(index)
    })
    .collect()
}

fn pivot_table_format_models(definition: &x::PivotTableDefinition) -> Vec<PivotTableFormatModel> {
  let Some(formats) = definition.formats.as_ref() else {
    return Vec::new();
  };
  formats
    .format
    .iter()
    .map(|format| {
      // Source: LibreOffice sc/source/filter/oox/PivotTableFormat.cxx
      // PivotTableFormat::importFormat/importPivotArea/finalizeImport.
      let pivot_area = &format.pivot_area;
      let data_only = pivot_area.data_only.is_none_or(|value| value.as_bool());
      let label_only = pivot_area.label_only.is_some_and(|value| value.as_bool());
      let kind = if data_only {
        PivotTableFormatKind::Data
      } else if label_only {
        PivotTableFormatKind::Label
      } else {
        PivotTableFormatKind::None
      };
      PivotTableFormatModel {
        format_id: format.format_id,
        kind,
        data_only,
        label_only,
        outline: pivot_area.outline.is_none_or(|value| value.as_bool()),
        field_position: pivot_area.field_position,
        selections: pivot_area
          .pivot_area_references
          .as_ref()
          .map(|references| {
            references
              .pivot_area_reference
              .iter()
              .filter_map(|reference| {
                Some(PivotTableFormatSelection {
                  selected: reference.selected.is_none_or(|value| value.as_bool()),
                  field: reference.field?,
                  item_indexes: reference.field_item.iter().map(|item| item.val).collect(),
                })
              })
              .collect()
          })
          .unwrap_or_default(),
      }
    })
    .collect()
}

fn pivot_format_axis_lines(
  items: Option<&[x::RowItem]>,
  field_indexes: &[i32],
  item_names: &[Vec<String>],
  geometry: &PivotOutputGeometry,
  axis: PivotFormatAxis,
) -> Vec<PivotFormatLineData> {
  let Some(items) = items else {
    return Vec::new();
  };
  items
    .iter()
    .enumerate()
    .map(|(index, item)| {
      let (line, position) = match axis {
        PivotFormatAxis::Row => {
          let field_position = pivot_format_row_field_position(item, field_indexes);
          (
            geometry.data_start.row + index as u32,
            geometry.table_start.col + field_position,
          )
        }
        PivotFormatAxis::Column => {
          let field_position = pivot_format_column_field_position(item, field_indexes);
          (
            geometry.data_start.col + index as u32,
            geometry.table_start.row + geometry.header_rows + field_position,
          )
        }
      };
      PivotFormatLineData {
        line,
        position,
        fields: pivot_format_item_fields(item, field_indexes, item_names),
      }
    })
    .collect()
}

fn pivot_builtin_frame_ranges(
  definition: &x::PivotTableDefinition,
  geometry: &PivotOutputGeometry,
  row_lines: &[PivotFormatLineData],
  column_lines: &[PivotFormatLineData],
) -> Vec<PivotFrameRange> {
  let mut ranges = Vec::new();
  let table_end = geometry.table_range.end;
  if geometry.data_start.col > geometry.table_start.col {
    if geometry.data_start.row > geometry.table_start.row {
      ranges.push(PivotFrameRange {
        range: CellRange::new(
          geometry.table_start,
          CellAddress {
            col: geometry.data_start.col - 1,
            row: geometry.data_start.row - 1,
          },
        ),
        horizontal_inner: false,
        field_frame: false,
      });
    }
    ranges.push(PivotFrameRange {
      range: CellRange::new(
        CellAddress {
          col: geometry.table_start.col,
          row: geometry.data_start.row,
        },
        CellAddress {
          col: geometry.data_start.col - 1,
          row: table_end.row,
        },
      ),
      horizontal_inner: false,
      field_frame: false,
    });
  }
  if geometry.data_start.row > geometry.table_start.row {
    ranges.push(PivotFrameRange {
      range: CellRange::new(
        CellAddress {
          col: geometry.data_start.col,
          row: geometry.table_start.row,
        },
        CellAddress {
          col: table_end.col,
          row: geometry.data_start.row - 1,
        },
      ),
      horizontal_inner: false,
      field_frame: false,
    });
  }
  ranges.extend(pivot_data_block_frame_ranges(
    geometry,
    row_lines,
    column_lines,
  ));
  ranges.extend(pivot_field_cell_frame_ranges(definition, geometry));
  ranges
}

fn pivot_data_block_frame_ranges(
  geometry: &PivotOutputGeometry,
  row_lines: &[PivotFormatLineData],
  column_lines: &[PivotFormatLineData],
) -> Vec<PivotFrameRange> {
  let mut rows = row_lines
    .iter()
    .filter(|line| pivot_format_line_adds_block_boundary(line, PivotFormatAxis::Row))
    .map(|line| line.line)
    .filter(|row| *row >= geometry.data_start.row && *row <= geometry.table_range.end.row)
    .collect::<Vec<_>>();
  rows.sort_unstable();
  rows.dedup();
  if rows.first().copied() != Some(geometry.data_start.row) {
    rows.insert(0, geometry.data_start.row);
  }
  rows.push(geometry.table_range.end.row + 1);

  let mut cols = column_lines
    .iter()
    .filter(|line| pivot_format_line_adds_block_boundary(line, PivotFormatAxis::Column))
    .map(|line| line.line)
    .filter(|col| *col >= geometry.data_start.col && *col <= geometry.table_range.end.col)
    .collect::<Vec<_>>();
  cols.sort_unstable();
  cols.dedup();
  if cols.first().copied() != Some(geometry.data_start.col) {
    cols.insert(0, geometry.data_start.col);
  }
  cols.push(geometry.table_range.end.col + 1);

  let mut ranges = Vec::new();
  for row_pair in rows.windows(2) {
    for col_pair in cols.windows(2) {
      let start = CellAddress {
        col: col_pair[0],
        row: row_pair[0],
      };
      let end = CellAddress {
        col: col_pair[1] - 1,
        row: row_pair[1] - 1,
      };
      if start.col <= end.col && start.row <= end.row {
        ranges.push(PivotFrameRange {
          range: CellRange::new(start, end),
          horizontal_inner: row_pair[1] - row_pair[0] > 1,
          field_frame: false,
        });
      }
    }
  }
  ranges
}

fn pivot_format_line_adds_block_boundary(
  line: &PivotFormatLineData,
  axis: PivotFormatAxis,
) -> bool {
  if pivot_format_line_has_subtotal(line) {
    return true;
  }
  let field_count = line.fields.len();
  line.fields.iter().enumerate().any(|(index, field)| {
    field.is_member
      && !field.subtotal
      && match axis {
        PivotFormatAxis::Row => index + 1 < field_count,
        PivotFormatAxis::Column => index + 1 < field_count,
      }
  })
}

fn pivot_field_cell_frame_ranges(
  definition: &x::PivotTableDefinition,
  geometry: &PivotOutputGeometry,
) -> Vec<PivotFrameRange> {
  let mut ranges = Vec::new();
  if geometry.header_rows > 0 {
    let row_field_count = definition
      .row_fields
      .as_ref()
      .map_or(0, |fields| fields.field.len() as u32);
    if row_field_count == 1 || !pivot_has_compact_row_field(definition) {
      for offset in 0..row_field_count {
        let address = CellAddress {
          col: geometry.table_start.col + offset,
          row: geometry.data_start.row - 1,
        };
        ranges.push(PivotFrameRange {
          range: CellRange::new(address, address),
          horizontal_inner: false,
          field_frame: true,
        });
      }
    }
    let column_field_count = pivot_column_field_count(definition);
    if column_field_count == 1 || !pivot_has_compact_row_field(definition) {
      for offset in 0..column_field_count {
        let address = CellAddress {
          col: geometry.data_start.col + offset,
          row: geometry.table_start.row,
        };
        ranges.push(PivotFrameRange {
          range: CellRange::new(address, address),
          horizontal_inner: false,
          field_frame: true,
        });
      }
    }
  }
  for offset in 0..definition
    .page_fields
    .as_ref()
    .map_or(0, |fields| fields.page_field.len() as u32)
  {
    let address = CellAddress {
      col: geometry.output_start.col + 1,
      row: geometry.output_start.row + offset,
    };
    ranges.push(PivotFrameRange {
      range: CellRange::new(address, address),
      horizontal_inner: false,
      field_frame: true,
    });
  }
  ranges
}

fn pivot_format_row_field_position(item: &x::RowItem, field_indexes: &[i32]) -> u32 {
  let field_count = field_indexes.len().max(1) as u32;
  item.repeated_item_count.unwrap_or(0).min(field_count - 1)
}

fn pivot_format_column_field_position(item: &x::RowItem, field_indexes: &[i32]) -> u32 {
  let field_count = field_indexes.len().max(1) as u32;
  item.repeated_item_count.unwrap_or(0).min(field_count - 1)
}

fn pivot_format_item_fields(
  item: &x::RowItem,
  field_indexes: &[i32],
  item_names: &[Vec<String>],
) -> Vec<PivotFormatFieldData> {
  field_indexes
    .iter()
    .enumerate()
    .map(|(field_position, dimension)| {
      let item_type = item.item_type.unwrap_or_default();
      let continues = pivot_format_item_continues(item, field_position);
      let index = if *dimension == PIVOT_FORMAT_DATA_DIMENSION {
        item.index.map(|index| index as i32).unwrap_or(0)
      } else {
        pivot_format_member_index(item, field_position)
      };
      let name = if *dimension == PIVOT_FORMAT_DATA_DIMENSION {
        "DATA".to_string()
      } else {
        usize::try_from(*dimension)
          .ok()
          .and_then(|dimension| item_names.get(dimension))
          .and_then(|names| {
            usize::try_from(index)
              .ok()
              .and_then(|index| names.get(index))
          })
          .cloned()
          .unwrap_or_default()
      };
      PivotFormatFieldData {
        dimension: *dimension,
        name,
        index,
        is_set: !continues,
        is_member: item_type == x::ItemValues::Data && !continues,
        subtotal: pivot_format_item_is_subtotal(item_type),
        continues,
      }
    })
    .collect()
}

fn pivot_format_member_index(item: &x::RowItem, field_position: usize) -> i32 {
  let repeated = item.repeated_item_count.unwrap_or(0) as usize;
  if field_position < repeated {
    return -1;
  }
  item
    .member_property_index
    .get(field_position - repeated)
    .and_then(|member| member.val)
    .unwrap_or(0)
}

fn pivot_format_item_continues(item: &x::RowItem, field_position: usize) -> bool {
  let repeated = item.repeated_item_count.unwrap_or(0) as usize;
  field_position < repeated || field_position - repeated >= item.member_property_index.len()
}

fn pivot_format_item_is_subtotal(item_type: x::ItemValues) -> bool {
  matches!(
    item_type,
    x::ItemValues::Sum
      | x::ItemValues::CountA
      | x::ItemValues::Average
      | x::ItemValues::Maximum
      | x::ItemValues::Minimum
      | x::ItemValues::Product
      | x::ItemValues::Count
      | x::ItemValues::StandardDeviation
      | x::ItemValues::StandardDeviationP
      | x::ItemValues::Variance
      | x::ItemValues::VarianceP
      | x::ItemValues::Grand
  )
}

fn page_field_models(
  definition: &x::PivotTableDefinition,
  cache_field_names: &[String],
  cache_field_items: &[Vec<String>],
) -> Vec<PivotPageFieldModel> {
  let Some(page_fields) = definition.page_fields.as_ref() else {
    return Vec::new();
  };
  page_fields
    .page_field
    .iter()
    .filter_map(|page_field| {
      let field_index = usize::try_from(page_field.field).ok()?;
      let field_name = page_field
        .caption
        .clone()
        .or_else(|| page_field.name.clone())
        .or_else(|| cache_field_names.get(field_index).cloned())
        .unwrap_or_default();
      if field_name.is_empty() {
        return None;
      }
      let pivot_field = definition
        .pivot_fields
        .as_ref()
        .and_then(|fields| fields.pivot_field.get(field_index));
      let value = page_field_value(page_field, pivot_field, cache_field_items.get(field_index));
      Some(PivotPageFieldModel { field_name, value })
    })
    .collect()
}

fn page_field_value(
  page_field: &x::PageField,
  pivot_field: Option<&x::PivotField>,
  cache_items: Option<&Vec<String>>,
) -> PivotPageFieldValue {
  let Some(pivot_field) = pivot_field else {
    return PivotPageFieldValue::All;
  };
  if let Some(item_index) = page_field.item {
    return pivot_field
      .items
      .as_ref()
      .and_then(|items| items.item.get(item_index as usize))
      .and_then(|item| page_item_member_text(item, cache_items))
      .map(PivotPageFieldValue::Member)
      .unwrap_or(PivotPageFieldValue::All);
  }
  let Some(items) = pivot_field.items.as_ref() else {
    return PivotPageFieldValue::All;
  };
  let hidden_data_items = items
    .item
    .iter()
    .filter(|item| page_item_is_data(item) && item.hidden.is_some_and(|hidden| hidden.as_bool()))
    .count();
  if hidden_data_items == 0
    && !pivot_field
      .multiple_item_selection_allowed
      .is_some_and(|value| value.as_bool())
  {
    return PivotPageFieldValue::All;
  }
  let mut visible = items
    .item
    .iter()
    .filter(|item| page_item_is_data(item) && !item.hidden.is_some_and(|hidden| hidden.as_bool()))
    .filter_map(|item| page_item_member_text(item, cache_items));
  let Some(first) = visible.next() else {
    return PivotPageFieldValue::All;
  };
  if visible.next().is_some() {
    PivotPageFieldValue::Multiple
  } else {
    PivotPageFieldValue::Member(first)
  }
}

fn page_item_is_data(item: &x::Item) -> bool {
  item
    .item_type
    .is_none_or(|item_type| item_type == x::ItemValues::Data)
}

fn page_item_member_text(item: &x::Item, cache_items: Option<&Vec<String>>) -> Option<String> {
  if let Some(name) = item.item_name.as_ref().filter(|name| !name.is_empty()) {
    return Some(name.clone());
  }
  let cache_index = item.index? as usize;
  cache_items.and_then(|items| items.get(cache_index).cloned())
}

struct PivotCountOverrideContext<'a> {
  records: Option<&'a x::PivotCacheRecords>,
  source_cache: Option<&'a PivotSourceCacheTable>,
  cache_field_items: &'a [Vec<String>],
  cache_field_item_values: &'a [Vec<PivotCacheItemValue>],
  cache_field_grouped: &'a [bool],
  source_field_number_format_codes: &'a [Option<String>],
  date_1904: bool,
}

fn pivot_count_data_cell_text_overrides(
  definition: &x::PivotTableDefinition,
  context: PivotCountOverrideContext<'_>,
) -> Vec<PivotDataCellTextOverride> {
  let Some(location) = CellRange::parse_a1_range(&definition.location.reference) else {
    return Vec::new();
  };
  if definition
    .column_fields
    .as_ref()
    .is_some_and(|fields| !fields.field.is_empty())
  {
    return Vec::new();
  }
  let row_field_indexes = definition
    .row_fields
    .as_ref()
    .map(|fields| {
      fields
        .field
        .iter()
        .filter_map(|field| usize::try_from(field.index).ok())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
  if row_field_indexes.is_empty() {
    return Vec::new();
  }
  let Some(data_field) = definition
    .data_fields
    .as_ref()
    .and_then(|fields| fields.data_field.first())
  else {
    return Vec::new();
  };
  if data_field.subtotal != Some(x::DataConsolidateFunctionValues::Count) {
    return Vec::new();
  }
  let cache_records_table;
  let source_cache = if let Some(source_cache) = context.source_cache {
    source_cache
  } else if let Some(records) = context.records {
    cache_records_table =
      PivotSourceCacheTable::from_records(records, context.cache_field_item_values);
    &cache_records_table
  } else {
    return Vec::new();
  };
  let page_filters = pivot_cache_page_filters(
    definition,
    source_cache,
    context.cache_field_item_values,
    context.cache_field_grouped,
    context.source_field_number_format_codes,
    context.date_1904,
  );
  let row_items = pivot_count_row_items(
    definition,
    &row_field_indexes,
    context.cache_field_items,
    context.cache_field_item_values,
  );
  if row_items.is_empty() {
    return Vec::new();
  }
  let mut counts = vec![0u32; row_items.len()];
  let mut detail_total = 0u32;
  let mut missing_detail_counts = HashMap::<Vec<String>, u32>::new();
  for record_index in 0..source_cache.rows.len() {
    if !source_cache_matches_page_filters(source_cache, record_index, &page_filters, false) {
      continue;
    }
    let mut matched_detail = false;
    for (row_position, row_item) in row_items.iter().enumerate() {
      if row_item.grand_total {
        continue;
      }
      if row_item.matches_source_record(source_cache, record_index, &row_field_indexes) {
        matched_detail |= row_item.detail;
        counts[row_position] += 1;
      }
    }
    if matched_detail {
      detail_total += 1;
    } else if let Some(values) =
      pivot_count_source_row_values(source_cache, record_index, &row_field_indexes)
    {
      *missing_detail_counts.entry(values).or_default() += 1;
    }
  }
  let total = detail_total
    + missing_detail_counts
      .values()
      .copied()
      .filter(|count| *count >= 2)
      .sum::<u32>();
  for (position, row_item) in row_items.iter().enumerate() {
    if row_item.grand_total {
      counts[position] = total;
    }
  }
  let data_col = location.start.col + definition.location.first_data_column;
  let data_row = location.start.row + definition.location.first_data_row;
  let mut overrides = counts
    .into_iter()
    .enumerate()
    .map(|(index, count)| PivotDataCellTextOverride {
      address: CellAddress {
        col: data_col,
        row: data_row + index as u32,
      },
      text: count.to_string(),
    })
    .collect::<Vec<_>>();
  if total > 0 {
    overrides.push(PivotDataCellTextOverride {
      address: CellAddress {
        col: data_col,
        row: data_row + row_items.len() as u32,
      },
      text: total.to_string(),
    });
  }
  overrides
}

#[derive(Clone, Debug)]
struct PivotCountRowItem {
  values: Vec<PivotCacheItemValue>,
  depth: usize,
  grand_total: bool,
  detail: bool,
}

impl PivotCountRowItem {
  fn matches_source_record(
    &self,
    source_cache: &PivotSourceCacheTable,
    record_index: usize,
    row_field_indexes: &[usize],
  ) -> bool {
    if self.grand_total {
      return true;
    }
    self
      .values
      .iter()
      .zip(row_field_indexes.iter())
      .take(self.depth)
      .all(|(value, field_index)| source_cache.item(record_index, *field_index, false) == *value)
  }
}

fn pivot_count_row_items(
  definition: &x::PivotTableDefinition,
  row_field_indexes: &[usize],
  cache_field_items: &[Vec<String>],
  cache_field_item_values: &[Vec<PivotCacheItemValue>],
) -> Vec<PivotCountRowItem> {
  let Some(row_items) = definition.row_items.as_ref() else {
    return Vec::new();
  };
  let mut current_values = vec![PivotCacheItemValue::Empty; row_field_indexes.len()];
  row_items
    .row_item
    .iter()
    .map(|row_item| {
      let item_type = row_item.item_type.unwrap_or(x::ItemValues::Data);
      let grand_total = item_type == x::ItemValues::Grand;
      let repeated = row_item.repeated_item_count.unwrap_or(0) as usize;
      let raw_depth = repeated + row_item.member_property_index.len();
      let depth = if grand_total {
        0
      } else if pivot_format_item_is_subtotal(item_type)
        || (item_type == x::ItemValues::Default && raw_depth < row_field_indexes.len())
      {
        raw_depth
      } else {
        row_field_indexes.len()
      }
      .min(row_field_indexes.len());
      let mut member_properties = row_item.member_property_index.iter();
      for field_position in 0..row_field_indexes.len() {
        if field_position >= repeated {
          let Some(member_index) = member_properties
            .next()
            .map(|property| property.val.unwrap_or(0))
            .and_then(|value| usize::try_from(value).ok())
          else {
            break;
          };
          let field_index = row_field_indexes[field_position];
          current_values[field_position] = pivot_row_item_cache_value(
            definition,
            field_index,
            member_index,
            cache_field_items,
            cache_field_item_values,
          )
          .unwrap_or(PivotCacheItemValue::Empty);
        }
      }
      PivotCountRowItem {
        values: current_values.clone(),
        depth,
        grand_total,
        detail: !grand_total && depth == row_field_indexes.len(),
      }
    })
    .collect()
}

fn pivot_count_source_row_values(
  source_cache: &PivotSourceCacheTable,
  record_index: usize,
  row_field_indexes: &[usize],
) -> Option<Vec<String>> {
  let values = row_field_indexes
    .iter()
    .map(|field_index| source_cache.item(record_index, *field_index, false))
    .collect::<Vec<_>>();
  values
    .iter()
    .all(|value| !value.is_empty())
    .then(|| values.into_iter().map(|value| value.text()).collect())
}

fn pivot_row_item_cache_value(
  definition: &x::PivotTableDefinition,
  field_index: usize,
  member_index: usize,
  cache_field_items: &[Vec<String>],
  cache_field_item_values: &[Vec<PivotCacheItemValue>],
) -> Option<PivotCacheItemValue> {
  let cache_index = definition
    .pivot_fields
    .as_ref()
    .and_then(|fields| fields.pivot_field.get(field_index))
    .and_then(|field| field.items.as_ref())
    .and_then(|items| items.item.get(member_index))
    .and_then(|item| item.index)
    .map(|index| index as usize)?;
  cache_field_item_values
    .get(field_index)
    .and_then(|items| items.get(cache_index))
    .cloned()
    .or_else(|| {
      cache_field_items
        .get(field_index)
        .and_then(|items| items.get(cache_index))
        .map(|text| PivotCacheItemValue::String(text.clone()))
    })
}

fn pivot_cache_page_filters(
  definition: &x::PivotTableDefinition,
  source_cache: &PivotSourceCacheTable,
  cache_field_item_values: &[Vec<PivotCacheItemValue>],
  cache_field_grouped: &[bool],
  source_field_number_format_codes: &[Option<String>],
  date_1904: bool,
) -> Vec<PivotPageRecordFilter> {
  let Some(page_fields) = definition.page_fields.as_ref() else {
    return Vec::new();
  };
  page_fields
    .page_field
    .iter()
    .flat_map(|page_field| {
      let Some(field_index) = usize::try_from(page_field.field).ok() else {
        return Vec::new();
      };
      if cache_field_grouped
        .get(field_index)
        .copied()
        .unwrap_or(false)
      {
        return Vec::new();
      }
      let Some(pivot_field) = definition
        .pivot_fields
        .as_ref()
        .and_then(|fields| fields.pivot_field.get(field_index))
      else {
        return Vec::new();
      };
      let visible_items = pivot_field_source_visible_items(
        pivot_field,
        source_cache,
        field_index,
        cache_field_item_values.get(field_index).map(Vec::as_slice),
        source_field_number_format_codes
          .get(field_index)
          .and_then(Option::as_deref),
        date_1904,
      );
      let visible_filter = visible_items.map(|items| PivotPageRecordFilter {
        field_index,
        kind: PivotSourceFilterKind::Group(items),
      });
      // Source: LibreOffice sc/source/filter/oox/pivottablebuffer.cxx
      // PivotTableField::convertPageField sets PROP_SelectedPage, which
      // ScDPSaveDimension::SetCurrentPage applies only to already registered
      // save members.  The source cache is filtered later from member
      // visibility in dptabsrc.cxx::FilterCacheByPageDimensions; it is not an
      // unconditional source-record equality filter.
      visible_filter.into_iter().collect::<Vec<_>>()
    })
    .collect()
}

fn pivot_field_source_visible_items(
  pivot_field: &x::PivotField,
  source_cache: &PivotSourceCacheTable,
  field_index: usize,
  cache_items: Option<&[PivotCacheItemValue]>,
  number_format_code: Option<&str>,
  date_1904: bool,
) -> Option<Vec<PivotCacheItemValue>> {
  let items = pivot_field.items.as_ref()?;
  let mut source_members =
    pivot_source_field_members(source_cache, field_index, number_format_code, date_1904);
  if source_members.is_empty() {
    return None;
  }
  let mut changed = false;
  for item in &items.item {
    if !page_item_is_data(item) {
      continue;
    }
    let Some(value) = pivot_item_cache_value(item, cache_items) else {
      continue;
    };
    let text = pivot_member_display_text(&value, number_format_code, date_1904);
    if let Some(member) = source_members
      .iter_mut()
      .rev()
      .find(|member| member.display_text == text)
    {
      member.visible = !item.hidden.is_some_and(|hidden| hidden.as_bool());
      changed = true;
    }
  }
  changed.then(|| {
    source_members
      .into_iter()
      .filter(|member| member.visible)
      .map(|member| member.value)
      .collect()
  })
}

#[derive(Clone, Debug)]
struct PivotSourceFieldMember {
  value: PivotCacheItemValue,
  display_text: String,
  visible: bool,
}

fn pivot_source_field_members(
  source_cache: &PivotSourceCacheTable,
  field_index: usize,
  number_format_code: Option<&str>,
  date_1904: bool,
) -> Vec<PivotSourceFieldMember> {
  let mut values = Vec::new();
  for row in &source_cache.rows {
    let value = row
      .get(field_index)
      .cloned()
      .unwrap_or(PivotCacheItemValue::Empty);
    if !values.contains(&value) {
      values.push(value);
    }
  }
  values.sort_by(pivot_cache_item_compare);
  values
    .into_iter()
    .map(|value| PivotSourceFieldMember {
      display_text: pivot_member_display_text(&value, number_format_code, date_1904),
      value,
      visible: true,
    })
    .collect()
}

fn pivot_member_display_text(
  value: &PivotCacheItemValue,
  number_format_code: Option<&str>,
  date_1904: bool,
) -> String {
  match value {
    PivotCacheItemValue::Value(value) => {
      rendered_number_text(&value.to_string(), number_format_code, None, date_1904).0
    }
    PivotCacheItemValue::DateTime { serial, .. } => {
      rendered_number_text(&serial.to_string(), number_format_code, None, date_1904).0
    }
    _ => value.text(),
  }
}

fn pivot_cache_item_compare(
  left: &PivotCacheItemValue,
  right: &PivotCacheItemValue,
) -> std::cmp::Ordering {
  let left_rank = pivot_cache_item_type_rank(left);
  let right_rank = pivot_cache_item_type_rank(right);
  left_rank
    .cmp(&right_rank)
    .then_with(|| match (left, right) {
      (PivotCacheItemValue::Value(left), PivotCacheItemValue::Value(right))
      | (
        PivotCacheItemValue::DateTime { serial: left, .. },
        PivotCacheItemValue::DateTime { serial: right, .. },
      )
      | (PivotCacheItemValue::Value(left), PivotCacheItemValue::DateTime { serial: right, .. })
      | (PivotCacheItemValue::DateTime { serial: left, .. }, PivotCacheItemValue::Value(right)) => {
        left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
      }
      (PivotCacheItemValue::String(left), PivotCacheItemValue::String(right))
      | (PivotCacheItemValue::Error(left), PivotCacheItemValue::Error(right)) => left.cmp(right),
      (PivotCacheItemValue::Boolean(left), PivotCacheItemValue::Boolean(right)) => left.cmp(right),
      _ => std::cmp::Ordering::Equal,
    })
}

fn pivot_cache_item_type_rank(value: &PivotCacheItemValue) -> u8 {
  match value {
    PivotCacheItemValue::Value(_) | PivotCacheItemValue::DateTime { .. } => 0,
    PivotCacheItemValue::Boolean(_) => 1,
    PivotCacheItemValue::String(_) | PivotCacheItemValue::Error(_) => 2,
    PivotCacheItemValue::Empty => 3,
  }
}

fn pivot_item_cache_value(
  item: &x::Item,
  cache_items: Option<&[PivotCacheItemValue]>,
) -> Option<PivotCacheItemValue> {
  if let Some(name) = item.item_name.as_ref().filter(|name| !name.is_empty()) {
    return Some(PivotCacheItemValue::String(name.clone()));
  }
  let cache_index = item.index? as usize;
  cache_items.and_then(|items| items.get(cache_index).cloned())
}

fn source_cache_matches_page_filters(
  source_cache: &PivotSourceCacheTable,
  record_index: usize,
  page_filters: &[PivotPageRecordFilter],
  repeat_if_empty: bool,
) -> bool {
  page_filters.iter().all(|filter| {
    let value = source_cache.item(record_index, filter.field_index, repeat_if_empty);
    filter.kind.matches(&value)
  })
}

impl PivotSourceFilterKind {
  fn matches(&self, value: &PivotCacheItemValue) -> bool {
    match self {
      Self::Group(items) => items.contains(value),
    }
  }
}

fn record_cache_item_value(
  record: &x::PivotCacheRecord,
  field_index: usize,
  cache_field_item_values: &[Vec<PivotCacheItemValue>],
) -> PivotCacheItemValue {
  let Some(choice) = record.pivot_cache_record_choice.get(field_index) else {
    return PivotCacheItemValue::Empty;
  };
  match choice {
    x::PivotCacheRecordChoice::MissingItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or(PivotCacheItemValue::Empty),
    x::PivotCacheRecordChoice::NumberItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or(PivotCacheItemValue::Value(item.val)),
    x::PivotCacheRecordChoice::BooleanItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or_else(|| PivotCacheItemValue::Boolean(item.val.as_bool())),
    x::PivotCacheRecordChoice::ErrorItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or_else(|| PivotCacheItemValue::Error(item.val.clone())),
    x::PivotCacheRecordChoice::StringItem(item) => {
      PivotCacheItemValue::String(item.caption.clone().unwrap_or_else(|| item.val.clone()))
    }
    x::PivotCacheRecordChoice::DateTimeItem(item) => item
      .caption
      .as_ref()
      .map(|caption| PivotCacheItemValue::String(caption.clone()))
      .unwrap_or_else(|| {
        pivot_cache_datetime_serial(&item.val)
          .map(|serial| PivotCacheItemValue::DateTime {
            serial,
            text: item.val.clone(),
          })
          .unwrap_or_else(|| PivotCacheItemValue::String(item.val.clone()))
      }),
    x::PivotCacheRecordChoice::FieldItem(item) => cache_field_item_values
      .get(field_index)
      .and_then(|items| items.get(item.val as usize))
      .cloned()
      .unwrap_or(PivotCacheItemValue::Empty),
  }
}

fn stale_source_data_field_indexes(
  cache: &x::PivotCacheDefinition,
  source_field_names: &[String],
) -> Vec<usize> {
  if source_field_names.is_empty() {
    return Vec::new();
  }
  let cache_field_names = pivot_cache_field_raw_names(cache);
  cache
    .cache_fields
    .cache_field
    .iter()
    .enumerate()
    .filter_map(|(field_index, field)| {
      let database_field = field.database_field.is_none_or(|value| value.as_bool());
      let name = cache_field_names.get(field_index)?;
      (database_field
        && !name.is_empty()
        && !source_field_names
          .iter()
          .any(|source_name| source_name.eq_ignore_ascii_case(name)))
      .then_some(field_index)
    })
    .collect()
}

fn unsupported_data_field_positions(
  fields: Option<&x::DataFields>,
  calculated_cache_fields: &[usize],
  stale_source_data_fields: &[usize],
) -> Vec<usize> {
  let Some(fields) = fields else {
    return Vec::new();
  };
  fields
    .data_field
    .iter()
    .enumerate()
    .filter_map(|(position, field)| {
      let field_index = field.field as usize;
      (calculated_cache_fields.contains(&field_index)
        || stale_source_data_fields.contains(&field_index))
      .then_some(position)
    })
    .collect()
}

fn data_field_counts(
  fields: Option<&x::DataFields>,
  unsupported_data_field_positions: &[usize],
) -> (u32, u32) {
  let Some(fields) = fields else {
    return (0, 0);
  };
  let total = fields.data_field.len() as u32;
  let unsupported = unsupported_data_field_positions.len() as u32;
  (total, unsupported)
}

fn printable_location_reference(
  reference: &str,
  unsupported_data_fields: u32,
  calculated_only_data_fields: bool,
  data_layout_axis: PivotDataLayoutAxis,
) -> String {
  if unsupported_data_fields == 0 {
    return reference.to_string();
  }
  let Some(range) = CellRange::parse_a1_range(reference) else {
    return reference.to_string();
  };
  // Source: LibreOffice sc/source/core/data/dpoutput.cxx::CalcSizes.
  // Data dimensions expand the generated DataPilot output along the data
  // layout axis. The stale OOXML destination cache is reduced along that same
  // axis when calculated data fields are not part of the generated PDF model.
  match data_layout_axis {
    PivotDataLayoutAxis::Columns | PivotDataLayoutAxis::Hidden => {
      let width = range.end.col.saturating_sub(range.start.col) + 1;
      let printable_width = width.saturating_sub(unsupported_data_fields).max(2);
      let printable_end_col = range.start.col + printable_width - 1;
      let printable_end_row = if calculated_only_data_fields
        && data_layout_axis == PivotDataLayoutAxis::Columns
        && range.end.row > range.start.row
      {
        range.end.row - 1
      } else {
        range.end.row
      };
      CellRange::new(
        range.start,
        CellAddress {
          col: printable_end_col.min(range.end.col),
          row: printable_end_row,
        },
      )
    }
    PivotDataLayoutAxis::Rows => {
      let height = range.end.row.saturating_sub(range.start.row) + 1;
      let printable_height = height.saturating_sub(unsupported_data_fields).max(2);
      let printable_end_row = range.start.row + printable_height - 1;
      CellRange::new(
        range.start,
        CellAddress {
          col: range.end.col,
          row: printable_end_row.min(range.end.row),
        },
      )
    }
  }
  .to_a1_range_string()
}

fn pivot_output_geometry(
  definition: &x::PivotTableDefinition,
  printable_reference: &str,
) -> PivotOutputGeometry {
  let location = CellRange::parse_a1_range(printable_reference)
    .or_else(|| CellRange::parse_a1_range(&definition.location.reference))
    .unwrap_or_default();
  // Source: LibreOffice sc/source/core/data/dpoutput.cxx::CalcSizes.
  // OOXML location is the insertion start plus a persisted cached area; Calc
  // recomputes member/data starts and table/result ends from row/column fields
  // and the data result matrix.
  let page_fields = definition
    .page_fields
    .as_ref()
    .map_or(0u32, |fields| fields.page_field.len() as u32);
  let page_rows = if page_fields > 0 { page_fields + 1 } else { 0 };
  let output_start = CellAddress {
    col: location.start.col,
    row: location.start.row.saturating_sub(page_rows),
  };
  let table_start = CellAddress {
    col: output_start.col,
    row: output_start.row + page_rows,
  };
  let header_rows = if definition.location.first_header_row == 0 {
    0
  } else if pivot_header_layout(definition) && pivot_column_field_count(definition) == 0 {
    2
  } else {
    1
  };
  let row_field_columns = pivot_columns_for_row_fields(definition);
  let column_field_rows = pivot_column_field_count(definition);
  // Source: LibreOffice sc/source/core/data/dpoutput.cxx
  // ScDPOutput::CalcSizes recomputes mnDataStartCol from the generated row
  // field column count, and mnDataStartRow from header rows plus column field
  // levels.  The OOXML firstDataCol/firstDataRow values describe Excel's stale
  // cached output range and are not used as lower bounds by Calc.
  let data_start_col_offset = row_field_columns;
  let data_start_row_offset = header_rows + column_field_rows;
  let data_start = CellAddress {
    col: table_start.col + data_start_col_offset,
    row: table_start.row + data_start_row_offset,
  };
  // Source: LibreOffice sc/source/core/data/dpoutput.cxx::CalcSizes.
  // Calc takes mnRowCount/mnColCount from the generated DataPilot result
  // matrix.  The PDF bridge still prints the imported cached sheetData, so the
  // printable location range is the closest upstream-backed matrix boundary;
  // printable_location_reference() has already reduced it for unsupported data
  // fields.
  let data_rows = location.end.row.saturating_sub(data_start.row) + 1;
  let data_columns = location.end.col.saturating_sub(data_start.col) + 1;
  let end = CellAddress {
    col: data_start.col + data_columns - 1,
    row: data_start.row + data_rows - 1,
  };
  let whole_range = CellRange::new(output_start, end);
  let table_range = CellRange::new(
    table_start,
    CellAddress {
      col: end.col,
      row: end.row,
    },
  );
  let result_range = CellRange::new(
    data_start,
    CellAddress {
      col: end.col,
      row: end.row,
    },
  );
  PivotOutputGeometry {
    whole_range,
    table_range,
    result_range,
    output_start,
    table_start,
    data_start,
    row_field_columns,
    column_field_rows,
    header_rows,
    data_rows,
    data_columns,
  }
}

fn pivot_header_layout(_definition: &x::PivotTableDefinition) -> bool {
  false
}

fn pivot_columns_for_row_fields(definition: &x::PivotTableDefinition) -> u32 {
  let Some(row_fields) = definition.row_fields.as_ref() else {
    return 0;
  };
  let row_field_count = row_fields.field.len() as u32;
  if !pivot_has_compact_row_field(definition) {
    return row_field_count;
  }
  let compact_flags = row_fields
    .field
    .iter()
    .map(|field| {
      usize::try_from(field.index)
        .ok()
        .and_then(|index| {
          definition
            .pivot_fields
            .as_ref()
            .and_then(|fields| fields.pivot_field.get(index))
        })
        .is_none_or(|field| field.compact.is_none_or(|value| value.as_bool()))
    })
    .collect::<Vec<_>>();
  let mut columns = compact_flags.iter().filter(|compact| !**compact).count() as u32;
  if compact_flags.last().copied().unwrap_or(false) {
    columns += 1;
  }
  columns
}

fn pivot_has_compact_row_field(definition: &x::PivotTableDefinition) -> bool {
  definition.row_fields.as_ref().is_some_and(|row_fields| {
    row_fields.field.iter().any(|field| {
      usize::try_from(field.index)
        .ok()
        .and_then(|index| {
          definition
            .pivot_fields
            .as_ref()
            .and_then(|fields| fields.pivot_field.get(index))
        })
        .is_some_and(|field| field.compact.is_none_or(|value| value.as_bool()))
    })
  })
}

fn pivot_column_field_count(definition: &x::PivotTableDefinition) -> u32 {
  definition
    .column_fields
    .as_ref()
    .map_or(0, |fields| fields.field.len() as u32)
}

fn data_layout_axis(definition: &x::PivotTableDefinition) -> PivotDataLayoutAxis {
  if fields_contain_data_layout(definition.row_fields.as_ref()) {
    PivotDataLayoutAxis::Rows
  } else if fields_contain_data_layout(definition.column_fields.as_ref()) {
    PivotDataLayoutAxis::Columns
  } else if definition.data_on_rows.is_some_and(|value| value.as_bool()) {
    PivotDataLayoutAxis::Rows
  } else if definition
    .data_fields
    .as_ref()
    .is_some_and(|fields| fields.data_field.len() > 1)
  {
    PivotDataLayoutAxis::Columns
  } else {
    PivotDataLayoutAxis::Hidden
  }
}

fn fields_contain_data_layout<T>(fields: Option<&T>) -> bool
where
  T: PivotAxisFields,
{
  fields.is_some_and(|fields| fields.fields().iter().any(|field| field.index == -2))
}

trait PivotAxisFields {
  fn fields(&self) -> &[x::Field];
}

impl PivotAxisFields for x::RowFields {
  fn fields(&self) -> &[x::Field] {
    &self.field
  }
}

impl PivotAxisFields for x::ColumnFields {
  fn fields(&self) -> &[x::Field] {
    &self.field
  }
}

trait PivotCellRangeFormat {
  fn to_a1_range_string(self) -> String;
}

impl PivotCellRangeFormat for CellRange {
  fn to_a1_range_string(self) -> String {
    format!(
      "{}{}:{}{}",
      column_name(self.start.col),
      self.start.row,
      column_name(self.end.col),
      self.end.row
    )
  }
}

fn column_name(mut col: u32) -> String {
  let mut chars = Vec::new();
  while col > 0 {
    col -= 1;
    chars.push((b'A' + (col % 26) as u8) as char);
    col /= 26;
  }
  chars.iter().rev().collect()
}

fn row_field_names(fields: Option<&x::RowFields>, cache_field_names: &[String]) -> Vec<String> {
  fields
    .map(|fields| {
      fields
        .field
        .iter()
        .filter_map(|field| usize::try_from(field.index).ok())
        .filter_map(|index| cache_field_names.get(index).cloned())
        .collect()
    })
    .unwrap_or_default()
}

fn row_field_indexes(fields: Option<&x::RowFields>) -> Vec<i32> {
  fields
    .map(|fields| fields.field.iter().map(|field| field.index).collect())
    .unwrap_or_default()
}

fn column_field_names(
  fields: Option<&x::ColumnFields>,
  cache_field_names: &[String],
) -> Vec<String> {
  fields
    .map(|fields| {
      fields
        .field
        .iter()
        .filter_map(|field| usize::try_from(field.index).ok())
        .filter_map(|index| cache_field_names.get(index).cloned())
        .collect()
    })
    .unwrap_or_default()
}

fn column_field_indexes(fields: Option<&x::ColumnFields>) -> Vec<i32> {
  fields
    .map(|fields| fields.field.iter().map(|field| field.index).collect())
    .unwrap_or_default()
}

fn data_field_names(
  fields: Option<&x::DataFields>,
  cache_field_names: &[String],
  unsupported_data_field_positions: &[usize],
) -> Vec<String> {
  fields
    .map(|fields| {
      fields
        .data_field
        .iter()
        .enumerate()
        .filter(|(position, _)| !unsupported_data_field_positions.contains(position))
        .map(|(_, field)| field)
        .map(|field| {
          if let Some(name) = field.name.as_ref().filter(|name| !name.is_empty()) {
            return name.clone();
          }
          let field_name = cache_field_names
            .get(field.field as usize)
            .cloned()
            .unwrap_or_default();
          if field_name.is_empty() {
            return String::new();
          }
          data_field_default_name(field.subtotal, &field_name)
        })
        .filter(|name| !name.is_empty())
        .collect()
    })
    .unwrap_or_default()
}

fn data_field_default_name(
  subtotal: Option<x::DataConsolidateFunctionValues>,
  field_name: &str,
) -> String {
  let function = match subtotal.unwrap_or(x::DataConsolidateFunctionValues::Sum) {
    x::DataConsolidateFunctionValues::Average => "Average",
    x::DataConsolidateFunctionValues::Count => "Count",
    x::DataConsolidateFunctionValues::CountNumbers => "Count",
    x::DataConsolidateFunctionValues::Maximum => "Max",
    x::DataConsolidateFunctionValues::Minimum => "Min",
    x::DataConsolidateFunctionValues::Product => "Product",
    x::DataConsolidateFunctionValues::StandardDeviation => "StdDev",
    x::DataConsolidateFunctionValues::StandardDeviationP => "StdDevP",
    x::DataConsolidateFunctionValues::Sum => "Sum",
    x::DataConsolidateFunctionValues::Variance => "Var",
    x::DataConsolidateFunctionValues::VarianceP => "VarP",
  };
  format!("{function} - {field_name}")
}

fn data_field_number_format_ids(
  definition: &x::PivotTableDefinition,
  cache_field_number_format_ids: &[Option<u32>],
  source_field_number_format_ids: &[Option<u32>],
  unsupported_data_field_positions: &[usize],
) -> Vec<Option<u32>> {
  definition
    .data_fields
    .as_ref()
    .map(|fields| {
      fields
        .data_field
        .iter()
        .enumerate()
        .filter(|(position, _)| !unsupported_data_field_positions.contains(position))
        .map(|(_, data_field)| data_field)
        .map(|data_field| {
          data_field
            .number_format_id
            .filter(|id| *id != 0)
            .or_else(|| {
              definition
                .pivot_fields
                .as_ref()
                .and_then(|fields| fields.pivot_field.get(data_field.field as usize))
                .and_then(|field| field.number_format_id)
                .filter(|id| *id != 0)
            })
            .or_else(|| {
              cache_field_number_format_ids
                .get(data_field.field as usize)
                .copied()
                .flatten()
                .filter(|id| *id != 0)
            })
            .or_else(|| {
              source_field_number_format_ids
                .get(data_field.field as usize)
                .copied()
                .flatten()
                .filter(|id| *id != 0)
            })
        })
        .collect()
    })
    .unwrap_or_default()
}

fn pivot_axis_field_number_format_ids(
  definition: &x::PivotTableDefinition,
  field_indexes: &[i32],
  cache_field_number_format_ids: &[Option<u32>],
  source_field_number_format_ids: &[Option<u32>],
) -> Vec<Option<u32>> {
  field_indexes
    .iter()
    .map(|field_index| {
      if *field_index < 0 {
        return None;
      }
      let field_index = *field_index as usize;
      definition
        .pivot_fields
        .as_ref()
        .and_then(|fields| fields.pivot_field.get(field_index))
        .and_then(|field| field.number_format_id)
        .filter(|id| *id != 0)
        .or_else(|| {
          cache_field_number_format_ids
            .get(field_index)
            .copied()
            .flatten()
            .filter(|id| *id != 0)
        })
        .or_else(|| {
          source_field_number_format_ids
            .get(field_index)
            .copied()
            .flatten()
            .filter(|id| *id != 0)
        })
    })
    .collect()
}

fn pivot_table_flag_count(definition: &x::PivotTableDefinition) -> usize {
  [
    definition.data_on_rows,
    definition.apply_number_formats,
    definition.apply_border_formats,
    definition.apply_font_formats,
    definition.apply_pattern_formats,
    definition.apply_alignment_formats,
    definition.apply_width_height_formats,
    definition.show_error,
    definition.show_missing,
    definition.show_drill,
    definition.print_drill,
    definition.preserve_formatting,
    definition.row_grand_totals,
    definition.column_grand_totals,
    definition.field_print_titles,
    definition.item_print_titles,
    definition.multiple_field_filters,
  ]
  .iter()
  .filter(|value| value.is_some_and(|value| value.as_bool()))
  .count()
}
