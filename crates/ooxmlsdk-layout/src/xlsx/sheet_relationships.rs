use ooxmlsdk::parts::named_sheet_views_part::NamedSheetViewsPart;
use ooxmlsdk::parts::single_cell_table_part::SingleCellTablePart;
use ooxmlsdk::parts::slicers_part::SlicersPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::time_line_part::TimeLinePart;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::parts::worksheet_sort_map_part::WorksheetSortMapPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2009_9_main as x14;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2010_11_main as x15;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2019_namedsheetviews as xnsv;
use ooxmlsdk::sdk::SdkPart;

use crate::error::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetRelationshipCatalog {
  pub(crate) single_xml_cells: Vec<SingleXmlCellCatalog>,
  pub(crate) named_sheet_views: Vec<NamedSheetViewCatalog>,
  pub(crate) slicers: Vec<SlicerCatalog>,
  pub(crate) timelines: Vec<TimelineCatalog>,
  pub(crate) sort_map: Option<WorksheetSortMapCatalog>,
  pub(crate) custom_properties: usize,
  pub(crate) printer_settings: usize,
  pub(crate) slicer_relationships: usize,
  pub(crate) timeline_relationships: usize,
  pub(crate) model3d_relationships: usize,
  pub(crate) active_x_binary_relationships: usize,
  pub(crate) hyperlink_targets: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SingleXmlCellCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) cells: usize,
  pub(crate) extension_cells: usize,
  pub(crate) ref_text_len: usize,
  pub(crate) unique_name_len: usize,
  pub(crate) xpath_len: usize,
  pub(crate) id_sum: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NamedSheetViewCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) views: usize,
  pub(crate) filters: usize,
  pub(crate) column_filters: usize,
  pub(crate) sort_rules: usize,
  pub(crate) extensions: usize,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SlicerCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) slicers: usize,
  pub(crate) extensions: usize,
  pub(crate) flags: usize,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TimelineCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) timelines: usize,
  pub(crate) extensions: usize,
  pub(crate) flags: usize,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct WorksheetSortMapCatalog {
  pub(crate) relationship_id: Option<String>,
  pub(crate) row_items: usize,
  pub(crate) column_items: usize,
  pub(crate) ref_text_len: usize,
  pub(crate) declared_count: usize,
}

impl SheetRelationshipCatalog {
  pub(crate) fn from_worksheet_part(
    package: &mut SpreadsheetDocument,
    part: &WorksheetPart,
  ) -> Result<Self> {
    // Source: LibreOffice sc/source/filter/oox/worksheetfragment.cxx imports
    // relationship-scoped worksheet helpers before final sheet/layout
    // finalization. Modern extension resources stay worksheet-owned here.
    let single_cell_parts = part
      .single_cell_table_part(package)
      .into_iter()
      .collect::<Vec<_>>();
    let named_sheet_view_parts = part.named_sheet_views_parts(package).collect::<Vec<_>>();
    let slicer_parts = part.slicers_parts(package).collect::<Vec<_>>();
    let timeline_parts = part.time_line_parts(package).collect::<Vec<_>>();
    let sort_map_part = part.worksheet_sort_map_part(package);

    Ok(Self {
      single_xml_cells: single_cell_parts
        .iter()
        .map(|part| SingleXmlCellCatalog::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      named_sheet_views: named_sheet_view_parts
        .iter()
        .map(|part| NamedSheetViewCatalog::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      slicers: slicer_parts
        .iter()
        .map(|part| SlicerCatalog::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      timelines: timeline_parts
        .iter()
        .map(|part| TimelineCatalog::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      sort_map: sort_map_part
        .as_ref()
        .map(|part| WorksheetSortMapCatalog::from_part(package, part))
        .transpose()?,
      custom_properties: part.custom_property_parts(package).count(),
      printer_settings: part.spreadsheet_printer_settings_parts(package).count(),
      slicer_relationships: part.slicers_parts(package).count(),
      timeline_relationships: part.time_line_parts(package).count(),
      model3d_relationships: part.model3_d_reference_relationship_parts(package).count(),
      active_x_binary_relationships: part
        .embedded_control_persistence_binary_data_parts(package)
        .count(),
      hyperlink_targets: part
        .hyperlink_relationships(package)
        .map(|relationship| {
          (
            relationship.id().to_string(),
            relationship.target().to_string(),
          )
        })
        .collect(),
    })
  }
}

impl SingleXmlCellCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &SingleCellTablePart) -> Result<Self> {
    let cells = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      cells: cells.single_xml_cell.len(),
      extension_cells: cells
        .single_xml_cell
        .iter()
        .filter(|cell| {
          cell.extension_list.is_some()
            || cell.xml_cell_properties.extension_list.is_some()
            || cell
              .xml_cell_properties
              .xml_properties
              .extension_list
              .is_some()
        })
        .count(),
      ref_text_len: cells
        .single_xml_cell
        .iter()
        .map(|cell| cell.cell_reference.len())
        .sum(),
      unique_name_len: cells
        .single_xml_cell
        .iter()
        .map(|cell| cell.xml_cell_properties.unique_name.len())
        .sum(),
      xpath_len: cells
        .single_xml_cell
        .iter()
        .map(|cell| cell.xml_cell_properties.xml_properties.x_path.len())
        .sum(),
      id_sum: cells
        .single_xml_cell
        .iter()
        .map(|cell| {
          cell.id as usize
            + cell.connection_id as usize
            + cell.xml_cell_properties.id as usize
            + cell.xml_cell_properties.xml_properties.map_id as usize
        })
        .sum(),
    })
  }
}

impl NamedSheetViewCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &NamedSheetViewsPart) -> Result<Self> {
    let views = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      views: views.named_sheet_view.len(),
      filters: views
        .named_sheet_view
        .iter()
        .map(|view| view.nsv_filter.len())
        .sum(),
      column_filters: views
        .named_sheet_view
        .iter()
        .flat_map(|view| view.nsv_filter.iter())
        .map(|filter| filter.column_filter.len())
        .sum(),
      sort_rules: views
        .named_sheet_view
        .iter()
        .flat_map(|view| view.nsv_filter.iter())
        .filter(|filter| filter.sort_rules.is_some())
        .count(),
      extensions: named_sheet_view_extension_count(views),
      text_len: named_sheet_view_text_len(views),
    })
  }
}

impl SlicerCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &SlicersPart) -> Result<Self> {
    let slicers = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      slicers: slicers.slicer.len(),
      extensions: slicers
        .slicer
        .iter()
        .filter(|slicer| slicer.extension_list.is_some())
        .count(),
      flags: slicers.slicer.iter().map(slicer_flag_count).sum(),
      text_len: slicers.slicer.iter().map(slicer_text_len).sum(),
    })
  }
}

impl TimelineCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &TimeLinePart) -> Result<Self> {
    let timelines = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      timelines: timelines.timeline.len(),
      extensions: timelines
        .timeline
        .iter()
        .filter(|timeline| timeline.extension_list.is_some())
        .count(),
      flags: timelines.timeline.iter().map(timeline_flag_count).sum(),
      text_len: timelines.timeline.iter().map(timeline_text_len).sum(),
    })
  }
}

impl WorksheetSortMapCatalog {
  fn from_part(package: &mut SpreadsheetDocument, part: &WorksheetSortMapPart) -> Result<Self> {
    let sort_map = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      row_items: sort_map
        .row_sort_map
        .as_ref()
        .map_or(0, |map| map.row_sort_map_item.len()),
      column_items: sort_map
        .column_sort_map
        .as_ref()
        .map_or(0, |map| map.column_sort_map_item.len()),
      ref_text_len: sort_map
        .row_sort_map
        .as_ref()
        .map_or(0, |map| map.r#ref.len())
        + sort_map
          .column_sort_map
          .as_ref()
          .map_or(0, |map| map.r#ref.len()),
      declared_count: sort_map
        .row_sort_map
        .as_ref()
        .and_then(|map| map.count)
        .unwrap_or_default() as usize
        + sort_map
          .column_sort_map
          .as_ref()
          .and_then(|map| map.count)
          .unwrap_or_default() as usize,
    })
  }
}

fn named_sheet_view_extension_count(views: &xnsv::NamedSheetViews) -> usize {
  usize::from(views.extension_list.is_some())
    + views
      .named_sheet_view
      .iter()
      .map(|view| {
        usize::from(view.extension_list.is_some())
          + view
            .nsv_filter
            .iter()
            .map(|filter| usize::from(filter.extension_list.is_some()))
            .sum::<usize>()
      })
      .sum::<usize>()
}

fn named_sheet_view_text_len(views: &xnsv::NamedSheetViews) -> usize {
  views
    .named_sheet_view
    .iter()
    .map(|view| {
      view.name.len()
        + view.id.len()
        + view
          .nsv_filter
          .iter()
          .map(|filter| {
            filter.filter_id.len()
              + filter.r#ref.as_ref().map_or(0, |value| value.len())
              + filter.table_id.unwrap_or_default() as usize
          })
          .sum::<usize>()
    })
    .sum()
}

fn slicer_flag_count(slicer: &x14::Slicer) -> usize {
  usize::from(slicer.show_caption.is_some_and(|value| value.as_bool()))
    + usize::from(slicer.locked_position.is_some_and(|value| value.as_bool()))
    + usize::from(slicer.start_item.is_some())
    + usize::from(slicer.column_count.is_some())
    + usize::from(slicer.level.is_some())
}

fn slicer_text_len(slicer: &x14::Slicer) -> usize {
  slicer.name.len()
    + slicer.cache.len()
    + slicer.caption.as_ref().map_or(0, |value| value.len())
    + slicer.style.as_ref().map_or(0, |value| value.len())
    + slicer.row_height as usize
}

fn timeline_flag_count(timeline: &x15::Timeline) -> usize {
  usize::from(timeline.show_header.is_some_and(|value| value.as_bool()))
    + usize::from(
      timeline
        .show_selection_label
        .is_some_and(|value| value.as_bool()),
    )
    + usize::from(
      timeline
        .show_time_level
        .is_some_and(|value| value.as_bool()),
    )
    + usize::from(
      timeline
        .show_horizontal_scrollbar
        .is_some_and(|value| value.as_bool()),
    )
}

fn timeline_text_len(timeline: &x15::Timeline) -> usize {
  timeline.name.len()
    + timeline.cache.len()
    + timeline.caption.as_ref().map_or(0, |value| value.len())
    + timeline
      .scroll_position
      .as_ref()
      .map_or(0, |value| value.len())
    + timeline.style.as_ref().map_or(0, |value| value.len())
    + timeline.level as usize
    + timeline.selection_level as usize
}
use std::collections::HashMap;
