use ooxmlsdk::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart;
use ooxmlsdk::parts::pivot_table_part::PivotTablePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

use super::worksheet::{CellAddress, CellRange};
use crate::error::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct PivotCacheCatalog {
  pub(crate) caches: Vec<PivotCacheModel>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PivotCacheModel {
  pub(crate) relationship_id: Option<String>,
  pub(crate) workbook_cache_id: Option<u32>,
  pub(crate) workbook_relationship_id: Option<String>,
  pub(crate) definition_relationship_id: Option<String>,
  pub(crate) cache_fields: usize,
  pub(crate) record_count: Option<u32>,
  pub(crate) refresh_on_load: bool,
  pub(crate) save_data: Option<bool>,
  pub(crate) invalid: bool,
  pub(crate) has_records_part: bool,
  pub(crate) has_cache_source: bool,
  pub(crate) has_extensions: bool,
  pub(crate) optional_child_count: usize,
}

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
  pub(crate) row_field_names: Vec<String>,
  pub(crate) column_field_names: Vec<String>,
  pub(crate) data_field_names: Vec<String>,
  pub(crate) has_cache_definition_part: bool,
  pub(crate) has_extensions: bool,
  pub(crate) flag_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PivotDataLayoutAxis {
  Columns,
  Rows,
  Hidden,
}

impl PivotCacheCatalog {
  pub(crate) fn from_workbook_part(
    package: &mut SpreadsheetDocument,
    workbook_part: &ooxmlsdk::parts::workbook_part::WorkbookPart,
    workbook: &x::Workbook,
  ) -> Result<Self> {
    // Source: LibreOffice sc/source/filter/oox/workbookfragment.cxx
    // Pivot cache ids are registered from workbook.xml and cache definitions
    // are imported on demand by pivot tables.
    let workbook_caches = workbook
      .pivot_caches
      .as_ref()
      .map(|caches| caches.pivot_cache.clone())
      .unwrap_or_default();
    let cache_parts = workbook_part
      .pivot_table_cache_definition_parts(package)
      .collect::<Vec<_>>();
    let mut caches = Vec::new();
    for (index, part) in cache_parts.iter().enumerate() {
      let workbook_cache = workbook_caches
        .iter()
        .find(|cache| part.relationship_id() == Some(cache.id.as_str()))
        .or_else(|| workbook_caches.get(index));
      caches.push(PivotCacheModel::from_part(package, part, workbook_cache)?);
    }
    Ok(Self { caches })
  }
}

impl PivotCacheModel {
  fn from_part(
    package: &mut SpreadsheetDocument,
    part: &PivotTableCacheDefinitionPart,
    workbook_cache: Option<&x::PivotCache>,
  ) -> Result<Self> {
    let has_records_part = part.pivot_table_cache_records_part(package).is_some();
    let definition = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      workbook_cache_id: workbook_cache.map(|cache| cache.cache_id),
      workbook_relationship_id: workbook_cache.map(|cache| cache.id.clone()),
      definition_relationship_id: definition.id.clone(),
      cache_fields: definition.cache_fields.cache_field.len(),
      record_count: definition.record_count,
      refresh_on_load: definition
        .refresh_on_load
        .is_some_and(|value| value.as_bool()),
      save_data: definition.save_data.map(|value| value.as_bool()),
      invalid: definition.invalid.is_some_and(|value| value.as_bool()),
      has_records_part,
      has_cache_source: true,
      has_extensions: definition.pivot_cache_definition_extension_list.is_some(),
      optional_child_count: usize::from(definition.cache_hierarchies.is_some())
        + usize::from(definition.kpis.is_some())
        + usize::from(definition.tuple_cache.is_some())
        + usize::from(definition.calculated_items.is_some())
        + usize::from(definition.calculated_members.is_some())
        + usize::from(definition.dimensions.is_some())
        + usize::from(definition.measure_groups.is_some())
        + usize::from(definition.maps.is_some()),
    })
  }
}

impl PivotTableCatalog {
  pub(crate) fn from_parts(
    package: &mut SpreadsheetDocument,
    parts: &[PivotTablePart],
  ) -> Result<Self> {
    Ok(Self {
      tables: parts
        .iter()
        .map(|part| PivotTableModel::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
    })
  }
}

impl PivotTableModel {
  fn from_part(package: &mut SpreadsheetDocument, part: &PivotTablePart) -> Result<Self> {
    let cache_field_names = {
      part
        .pivot_table_cache_definition_part(package)
        .and_then(|part| part.root_element(package).ok())
        .map(|cache| pivot_cache_field_names(cache))
        .unwrap_or_default()
    };
    let calculated_cache_fields = part
      .pivot_table_cache_definition_part(package)
      .and_then(|part| part.root_element(package).ok())
      .map(|cache| calculated_cache_field_indexes(cache))
      .unwrap_or_default();
    let definition = part.root_element(package)?;
    let has_cache_definition_part = !cache_field_names.is_empty();
    let (data_fields, unsupported_data_fields) =
      data_field_counts(definition.data_fields.as_ref(), &calculated_cache_fields);
    let calculated_only_data_fields = data_fields > 0 && data_fields == unsupported_data_fields;
    let data_layout_axis = data_layout_axis(definition);
    let printable_location_reference = printable_location_reference(
      &definition.location.reference,
      unsupported_data_fields,
      calculated_only_data_fields,
      data_layout_axis,
    );
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      name: definition.name.clone(),
      cache_id: definition.cache_id,
      location_reference: definition.location.reference.clone(),
      printable_location_reference,
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
      row_field_names: row_field_names(definition.row_fields.as_ref(), &cache_field_names),
      column_field_names: column_field_names(definition.column_fields.as_ref(), &cache_field_names),
      data_field_names: data_field_names(
        definition.data_fields.as_ref(),
        &cache_field_names,
        &calculated_cache_fields,
      ),
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

fn data_field_counts(
  fields: Option<&x::DataFields>,
  calculated_cache_fields: &[usize],
) -> (u32, u32) {
  let Some(fields) = fields else {
    return (0, 0);
  };
  let total = fields.data_field.len() as u32;
  let calculated = fields
    .data_field
    .iter()
    .filter(|field| calculated_cache_fields.contains(&(field.field as usize)))
    .count() as u32;
  (total, calculated)
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

fn data_field_names(
  fields: Option<&x::DataFields>,
  cache_field_names: &[String],
  calculated_cache_fields: &[usize],
) -> Vec<String> {
  fields
    .map(|fields| {
      fields
        .data_field
        .iter()
        .filter(|field| !calculated_cache_fields.contains(&(field.field as usize)))
        .map(|field| {
          field
            .name
            .clone()
            .or_else(|| cache_field_names.get(field.field as usize).cloned())
            .unwrap_or_default()
        })
        .filter(|name| !name.is_empty())
        .collect()
    })
    .unwrap_or_default()
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
