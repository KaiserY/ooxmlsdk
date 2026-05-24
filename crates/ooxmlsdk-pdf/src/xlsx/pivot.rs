use ooxmlsdk::parts::pivot_table_cache_definition_part::PivotTableCacheDefinitionPart;
use ooxmlsdk::parts::pivot_table_part::PivotTablePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

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
  pub(crate) style_name: Option<String>,
  pub(crate) pivot_fields: usize,
  pub(crate) row_fields: usize,
  pub(crate) column_fields: usize,
  pub(crate) page_fields: usize,
  pub(crate) data_fields: usize,
  pub(crate) filters: usize,
  pub(crate) formats: usize,
  pub(crate) has_cache_definition_part: bool,
  pub(crate) has_extensions: bool,
  pub(crate) flag_count: usize,
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
    let has_cache_definition_part = part.pivot_table_cache_definition_part(package).is_some();
    let definition = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      name: definition.name.clone(),
      cache_id: definition.cache_id,
      location_reference: definition.location.reference.clone(),
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
      has_cache_definition_part,
      has_extensions: definition.pivot_table_definition_extension_list.is_some(),
      flag_count: pivot_table_flag_count(definition),
    })
  }
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
