use ooxmlsdk::parts::calculation_chain_part::CalculationChainPart;
use ooxmlsdk::parts::cell_metadata_part::CellMetadataPart;
use ooxmlsdk::parts::custom_data_properties_part::CustomDataPropertiesPart;
use ooxmlsdk::parts::custom_xml_part::CustomXmlPart;
use ooxmlsdk::parts::external_workbook_part::ExternalWorkbookPart;
use ooxmlsdk::parts::feature_property_bags_part::FeaturePropertyBagsPart;
use ooxmlsdk::parts::rd_array_part::RdArrayPart;
use ooxmlsdk::parts::rd_rich_value_part::RdRichValuePart;
use ooxmlsdk::parts::rd_rich_value_structure_part::RdRichValueStructurePart;
use ooxmlsdk::parts::rd_rich_value_types_part::RdRichValueTypesPart;
use ooxmlsdk::parts::rd_rich_value_web_image_part::RdRichValueWebImagePart;
use ooxmlsdk::parts::rd_supporting_property_bag_part::RdSupportingPropertyBagPart;
use ooxmlsdk::parts::rd_supporting_property_bag_structure_part::RdSupportingPropertyBagStructurePart;
use ooxmlsdk::parts::rich_styles_part::RichStylesPart;
use ooxmlsdk::parts::slicer_cache_part::SlicerCachePart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::time_line_cache_part::TimeLineCachePart;
use ooxmlsdk::parts::volatile_dependencies_part::VolatileDependenciesPart;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::parts::workbook_person_part::WorkbookPersonPart;
use ooxmlsdk::parts::workbook_user_data_part::WorkbookUserDataPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2017_richdata2 as xlrd2;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments as tc;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2022_featurepropertybag as xfpb;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

use crate::error::{LayoutError, Result};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WorkbookCatalog {
  pub(crate) external_links: Vec<ExternalLinkModel>,
  pub(crate) external_cached_cells: Vec<ExternalCachedCell>,
  pub(crate) xml_maps: Option<XmlMapsModel>,
  pub(crate) persons: Vec<PersonModel>,
  pub(crate) revisions: Option<RevisionHeadersModel>,
  pub(crate) relationship_resources: WorkbookRelationshipResources,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExternalCachedCell {
  pub(crate) link_index: usize,
  pub(crate) sheet_name: String,
  pub(crate) reference: String,
  pub(crate) value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExternalLinkModel {
  pub(crate) relationship_id: Option<String>,
  pub(crate) kind: ExternalLinkKind,
  pub(crate) sheet_names: usize,
  pub(crate) defined_names: usize,
  pub(crate) cached_sheet_data: usize,
  pub(crate) item_count: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ExternalLinkKind {
  ExternalBook {
    relationship_id: String,
  },
  Dde {
    service: String,
    topic: String,
  },
  Ole {
    relationship_id: String,
    prog_id: String,
  },
  Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct XmlMapsModel {
  pub(crate) relationship_id: Option<String>,
  pub(crate) selection_namespaces: String,
  pub(crate) schemas: usize,
  pub(crate) maps: usize,
  pub(crate) schema_ref_count: usize,
  pub(crate) map_flag_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PersonModel {
  pub(crate) relationship_id: Option<String>,
  pub(crate) persons: usize,
  pub(crate) id_text_len: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RevisionHeadersModel {
  pub(crate) relationship_id: Option<String>,
  pub(crate) guid: String,
  pub(crate) last_guid: Option<String>,
  pub(crate) headers: usize,
  pub(crate) revision_logs: usize,
  pub(crate) flag_count: usize,
  pub(crate) user_text_len: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WorkbookRelationshipResources {
  pub(crate) custom_xml_parts: Vec<CustomXmlResource>,
  pub(crate) custom_data_properties: Vec<CustomDataResource>,
  pub(crate) slicer_caches: Vec<SlicerCacheResource>,
  pub(crate) timeline_caches: Vec<TimelineCacheResource>,
  pub(crate) rich_values: Vec<RichValueResource>,
  pub(crate) rich_value_structures: Vec<RichValueStructureResource>,
  pub(crate) arrays: Vec<ArrayDataResource>,
  pub(crate) rich_styles: Vec<RichStylesResource>,
  pub(crate) supporting_property_bags: Vec<SupportingPropertyBagResource>,
  pub(crate) supporting_property_bag_structures: Vec<SupportingPropertyBagStructureResource>,
  pub(crate) rich_value_types: Vec<RichValueTypesResource>,
  pub(crate) rich_value_web_image: Option<RichValueWebImageResource>,
  pub(crate) feature_property_bags: Option<FeaturePropertyBagsResource>,
  pub(crate) has_vba_project: bool,
  pub(crate) has_attached_toolbars: bool,
  pub(crate) user_data: Option<WorkbookUserDataResource>,
  pub(crate) calculation_chain: Option<CalculationChainResource>,
  pub(crate) cell_metadata: Option<CellMetadataResource>,
  pub(crate) volatile_dependencies: Option<VolatileDependenciesResource>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct CustomXmlResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) has_properties: bool,
  pub(crate) schema_refs: usize,
  pub(crate) text_len: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct CustomDataResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) has_custom_data: bool,
  pub(crate) text_len: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SlicerCacheResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) name_len: usize,
  pub(crate) source_name_len: usize,
  pub(crate) pivot_tables: usize,
  pub(crate) has_data: bool,
  pub(crate) extension_markers: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct TimelineCacheResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) name_len: usize,
  pub(crate) source_name_len: usize,
  pub(crate) pivot_tables: usize,
  pub(crate) state_flags: usize,
  pub(crate) text_len: usize,
  pub(crate) extension_markers: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RichValueResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) declared_count: u32,
  pub(crate) rich_values: usize,
  pub(crate) values: usize,
  pub(crate) fallbacks: usize,
  pub(crate) text_len: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RichValueStructureResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) declared_count: u32,
  pub(crate) structures: usize,
  pub(crate) keys: usize,
  pub(crate) text_len: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ArrayDataResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) declared_count: u32,
  pub(crate) arrays: usize,
  pub(crate) values: usize,
  pub(crate) text_len: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RichStylesResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) has_dxfs: bool,
  pub(crate) has_properties: bool,
  pub(crate) styles: usize,
  pub(crate) style_values: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SupportingPropertyBagResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) arrays: usize,
  pub(crate) bags: usize,
  pub(crate) values: usize,
  pub(crate) extension_markers: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SupportingPropertyBagStructureResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) declared_count: u32,
  pub(crate) structures: usize,
  pub(crate) keys: usize,
  pub(crate) text_len: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RichValueTypesResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) has_global_type: bool,
  pub(crate) types: usize,
  pub(crate) key_flags: usize,
  pub(crate) reserved_keys: usize,
  pub(crate) reserved_key_flags: usize,
  pub(crate) text_len: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct RichValueWebImageResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) images: usize,
  pub(crate) address_relationships: usize,
  pub(crate) more_images_relationships: usize,
  pub(crate) blip_relationships: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct FeaturePropertyBagsResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) declared_count: u32,
  pub(crate) bag_extensions: usize,
  pub(crate) bags: usize,
  pub(crate) values: usize,
  pub(crate) text_len: usize,
  pub(crate) extension_markers: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WorkbookUserDataResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) declared_count: u32,
  pub(crate) users: usize,
  pub(crate) text_len: usize,
  pub(crate) extension_markers: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct CalculationChainResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) cells: usize,
  pub(crate) flag_count: usize,
  pub(crate) text_len: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct CellMetadataResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) metadata_types: usize,
  pub(crate) metadata_strings: usize,
  pub(crate) mdx_records: usize,
  pub(crate) future_metadata: usize,
  pub(crate) cell_blocks: usize,
  pub(crate) value_blocks: usize,
  pub(crate) records: usize,
  pub(crate) text_len: usize,
  pub(crate) extension_markers: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct VolatileDependenciesResource {
  pub(crate) relationship_id: Option<String>,
  pub(crate) types: usize,
  pub(crate) has_extensions: bool,
}

impl WorkbookCatalog {
  pub(crate) fn from_workbook_part(
    package: &mut SpreadsheetDocument,
    workbook_part: &WorkbookPart,
  ) -> Result<Self> {
    // Source: LibreOffice sc/source/filter/oox/workbookfragment.cxx
    // finalizeImport imports persons, connections, custom XML, xmlMaps,
    // external links, revisions, and VBA through workbook-level relationships.
    let external_reference_ids = workbook_part
      .root_element(package)?
      .external_references
      .as_ref()
      .map(|references| {
        references
          .external_reference
          .iter()
          .map(|reference| reference.id.clone())
          .collect::<Vec<_>>()
      })
      .unwrap_or_default();
    let external_parts =
      ordered_external_workbook_parts(package, workbook_part, &external_reference_ids);
    let person_parts = workbook_part
      .workbook_person_parts(package)
      .collect::<Vec<_>>();
    let xml_maps_part = workbook_part.custom_xml_mappings_part(package);
    let revision_part = workbook_part.workbook_revision_header_part(package);
    let relationship_resources =
      WorkbookRelationshipResources::from_workbook_part(package, workbook_part)?;

    let external_links = external_parts
      .iter()
      .map(|part| ExternalLinkModel::from_part(package, part))
      .collect::<Result<Vec<_>>>()?;
    let external_cached_cells = external_parts
      .iter()
      .enumerate()
      .map(|(index, part)| external_cached_cells_from_part(package, part, index + 1))
      .collect::<Result<Vec<_>>>()?
      .into_iter()
      .flatten()
      .collect();

    Ok(Self {
      external_links,
      external_cached_cells,
      xml_maps: xml_maps_part
        .as_ref()
        .map(|part| XmlMapsModel::from_part(package, part))
        .transpose()?,
      persons: person_parts
        .iter()
        .map(|part| PersonModel::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      revisions: revision_part
        .as_ref()
        .map(|part| RevisionHeadersModel::from_part(package, part))
        .transpose()?,
      relationship_resources,
    })
  }
}

fn ordered_external_workbook_parts(
  package: &SpreadsheetDocument,
  workbook_part: &WorkbookPart,
  reference_ids: &[String],
) -> Vec<ExternalWorkbookPart> {
  let parts = workbook_part
    .external_workbook_parts(package)
    .collect::<Vec<_>>();
  if reference_ids.is_empty() {
    return parts;
  }

  let mut ordered = Vec::with_capacity(parts.len());
  for reference_id in reference_ids {
    if let Some(part) = parts
      .iter()
      .find(|part| workbook_part.get_id_of_part(package, *part) == Some(reference_id.as_str()))
    {
      ordered.push(part.clone());
    }
  }
  for part in parts {
    if !ordered.iter().any(|ordered_part| ordered_part == &part) {
      ordered.push(part);
    }
  }
  ordered
}

fn external_cached_cells_from_part(
  package: &mut SpreadsheetDocument,
  part: &ExternalWorkbookPart,
  link_index: usize,
) -> Result<Vec<ExternalCachedCell>> {
  let link = part.root_element(package)?;
  let Some(x::ExternalLinkChoice::ExternalBook(book)) = &link.external_link_choice else {
    return Ok(Vec::new());
  };
  let sheet_names = book
    .sheet_names
    .as_ref()
    .map(|names| {
      names
        .sheet_name
        .iter()
        .map(|name| name.val.clone().unwrap_or_default())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
  let Some(data_set) = &book.sheet_data_set else {
    return Ok(Vec::new());
  };
  let mut cells = Vec::new();
  for sheet_data in &data_set.external_sheet_data {
    let sheet_name = sheet_names
      .get(sheet_data.sheet_id as usize)
      .cloned()
      .unwrap_or_else(|| sheet_data.sheet_id.to_string());
    for row in &sheet_data.external_row {
      for cell in &row.external_cell {
        if let Some(value) = cell
          .xstring
          .as_ref()
          .and_then(|value| value.xml_content.clone())
        {
          cells.push(ExternalCachedCell {
            link_index,
            sheet_name: sheet_name.clone(),
            reference: cell.cell_reference.clone(),
            value,
          });
        }
      }
    }
  }
  Ok(cells)
}

impl ExternalLinkModel {
  fn from_part(package: &mut SpreadsheetDocument, part: &ExternalWorkbookPart) -> Result<Self> {
    let link = part.root_element(package)?;
    let mut model = Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      kind: ExternalLinkKind::Unknown,
      sheet_names: 0,
      defined_names: 0,
      cached_sheet_data: 0,
      item_count: 0,
      has_extensions: link.extension_list.is_some(),
    };

    match &link.external_link_choice {
      Some(x::ExternalLinkChoice::ExternalBook(book)) => {
        model.kind = ExternalLinkKind::ExternalBook {
          relationship_id: book.id.clone(),
        };
        model.sheet_names = book
          .sheet_names
          .as_ref()
          .map_or(0, |names| names.sheet_name.len());
        model.defined_names = book
          .external_defined_names
          .as_ref()
          .map_or(0, |names| names.external_defined_name.len());
        model.cached_sheet_data = book
          .sheet_data_set
          .as_ref()
          .map_or(0, |data| data.external_sheet_data.len());
      }
      Some(x::ExternalLinkChoice::DdeLink(dde)) => {
        model.kind = ExternalLinkKind::Dde {
          service: dde.dde_service.clone(),
          topic: dde.dde_topic.clone(),
        };
        model.item_count = dde
          .dde_items
          .as_ref()
          .map_or(0, |items| items.dde_item.len());
      }
      Some(x::ExternalLinkChoice::OleLink(ole)) => {
        model.kind = ExternalLinkKind::Ole {
          relationship_id: ole.id.clone(),
          prog_id: ole.prog_id.clone(),
        };
        model.item_count = ole
          .ole_items
          .as_ref()
          .map_or(0, |items| items.ole_items_choice.len());
      }
      None => {}
    }
    Ok(model)
  }
}

impl XmlMapsModel {
  fn from_part(
    package: &mut SpreadsheetDocument,
    part: &ooxmlsdk::parts::custom_xml_mappings_part::CustomXmlMappingsPart,
  ) -> Result<Self> {
    let maps = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      selection_namespaces: maps.selection_namespaces.clone(),
      schemas: maps.schema.len(),
      maps: maps.map.len(),
      schema_ref_count: maps
        .schema
        .iter()
        .filter(|schema| schema.schema_reference.is_some() || schema.namespace.is_some())
        .count(),
      map_flag_count: maps
        .map
        .iter()
        .map(|map| {
          usize::from(map.show_import_export_errors.as_bool())
            + usize::from(map.auto_fit.as_bool())
            + usize::from(map.append_data.as_bool())
            + usize::from(map.preserve_auto_filter_state.as_bool())
        })
        .sum(),
    })
  }
}

impl PersonModel {
  fn from_part(package: &mut SpreadsheetDocument, part: &WorkbookPersonPart) -> Result<Self> {
    let persons = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      persons: persons.person.len(),
      id_text_len: persons.person.iter().map(person_text_len).sum(),
      has_extensions: persons.extension_list.is_some(),
    })
  }
}

fn person_text_len(person: &tc::Person) -> usize {
  person.display_name.len()
    + person.id.len()
    + person.user_id.as_ref().map_or(0, |value| value.len())
    + person.provider_id.as_ref().map_or(0, |value| value.len())
    + usize::from(person.extension_list.is_some())
}

impl RevisionHeadersModel {
  fn from_part(
    package: &mut SpreadsheetDocument,
    part: &ooxmlsdk::parts::workbook_revision_header_part::WorkbookRevisionHeaderPart,
  ) -> Result<Self> {
    let revision_logs = part.workbook_revision_log_parts(package).count();
    let headers = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      guid: headers.guid.clone(),
      last_guid: headers.last_guid.clone(),
      headers: headers.header.len(),
      revision_logs,
      flag_count: [
        headers.shared,
        headers.disk_revisions,
        headers.history,
        headers.track_revisions,
        headers.exclusive,
        headers.keep_change_history,
        headers.protected,
      ]
      .iter()
      .filter(|value| value.is_some_and(|value| value.as_bool()))
      .count(),
      user_text_len: headers
        .header
        .iter()
        .map(|header| {
          header.guid.len()
            + header.user_name.len()
            + header.id.len()
            + header.date_time.len()
            + header.max_sheet_id as usize
            + header.min_revision_id.unwrap_or_default() as usize
            + header.max_revision_id.unwrap_or_default() as usize
            + usize::from(header.extension_list.is_some())
        })
        .sum(),
    })
  }
}

impl WorkbookRelationshipResources {
  fn from_workbook_part(
    package: &mut SpreadsheetDocument,
    workbook_part: &WorkbookPart,
  ) -> Result<Self> {
    let custom_xml_parts = workbook_part.custom_xml_parts(package).collect::<Vec<_>>();
    let custom_data_properties_parts = workbook_part
      .custom_data_properties_parts(package)
      .collect::<Vec<_>>();
    let slicer_cache_parts = workbook_part
      .slicer_cache_parts(package)
      .collect::<Vec<_>>();
    let timeline_cache_parts = workbook_part
      .time_line_cache_parts(package)
      .collect::<Vec<_>>();
    let rich_value_parts = workbook_part
      .rd_rich_value_parts(package)
      .collect::<Vec<_>>();
    let rich_value_structure_parts = workbook_part
      .ct_rd_rich_value_structure_parts(package)
      .collect::<Vec<_>>();
    let array_parts = workbook_part.rd_array_parts(package).collect::<Vec<_>>();
    let rich_styles_parts = workbook_part.rich_styles_parts(package).collect::<Vec<_>>();
    let supporting_property_bag_parts = workbook_part
      .rd_supporting_property_bag_parts(package)
      .collect::<Vec<_>>();
    let supporting_property_bag_structure_parts = workbook_part
      .rd_supporting_property_bag_structure_parts(package)
      .collect::<Vec<_>>();
    let rich_value_types_parts = workbook_part
      .rd_rich_value_types_parts(package)
      .collect::<Vec<_>>();
    let rich_value_web_image_part = workbook_part.rd_rich_value_web_image_part(package);
    let feature_property_bags_part = workbook_part.feature_property_bags_part(package);
    let user_data_part = workbook_part.workbook_user_data_part(package);
    let calculation_chain_part = workbook_part.calculation_chain_part(package);
    let cell_metadata_part = workbook_part.cell_metadata_part(package);
    let volatile_dependencies_part = workbook_part.volatile_dependencies_part(package);
    Ok(Self {
      custom_xml_parts: custom_xml_parts
        .iter()
        .map(|part| CustomXmlResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      custom_data_properties: custom_data_properties_parts
        .iter()
        .map(|part| CustomDataResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      slicer_caches: slicer_cache_parts
        .iter()
        .map(|part| SlicerCacheResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      timeline_caches: timeline_cache_parts
        .iter()
        .map(|part| TimelineCacheResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      rich_values: rich_value_parts
        .iter()
        .map(|part| RichValueResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      rich_value_structures: rich_value_structure_parts
        .iter()
        .map(|part| RichValueStructureResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      arrays: array_parts
        .iter()
        .map(|part| ArrayDataResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      rich_styles: rich_styles_parts
        .iter()
        .map(|part| RichStylesResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      supporting_property_bags: supporting_property_bag_parts
        .iter()
        .map(|part| SupportingPropertyBagResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      supporting_property_bag_structures: supporting_property_bag_structure_parts
        .iter()
        .map(|part| SupportingPropertyBagStructureResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      rich_value_types: rich_value_types_parts
        .iter()
        .map(|part| RichValueTypesResource::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
      rich_value_web_image: rich_value_web_image_part
        .as_ref()
        .map(|part| RichValueWebImageResource::from_part(package, part))
        .transpose()?,
      feature_property_bags: feature_property_bags_part
        .as_ref()
        .map(|part| FeaturePropertyBagsResource::from_part(package, part))
        .transpose()?,
      has_vba_project: workbook_part.vba_project_part(package).is_some(),
      has_attached_toolbars: workbook_part
        .excel_attached_toolbars_part(package)
        .is_some(),
      user_data: user_data_part
        .as_ref()
        .map(|part| WorkbookUserDataResource::from_part(package, part))
        .transpose()?,
      calculation_chain: calculation_chain_part
        .as_ref()
        .map(|part| CalculationChainResource::from_part(package, part))
        .transpose()?,
      cell_metadata: cell_metadata_part
        .as_ref()
        .map(|part| CellMetadataResource::from_part(package, part))
        .transpose()?,
      volatile_dependencies: volatile_dependencies_part
        .as_ref()
        .map(|part| VolatileDependenciesResource::from_part(package, part))
        .transpose()?,
    })
  }
}

impl CustomXmlResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &CustomXmlPart) -> Result<Self> {
    let properties = part.custom_xml_properties_part(package);
    let (schema_refs, text_len) = properties
      .as_ref()
      .map(|props| {
        let item = props.root_element(package)?;
        Ok::<(usize, usize), LayoutError>((
          item
            .schema_references
            .as_ref()
            .map_or(0, |refs| refs.schema_reference.len()),
          item.item_id.len()
            + item.schema_references.as_ref().map_or(0, |refs| {
              refs
                .schema_reference
                .iter()
                .map(|schema| schema.uri.len())
                .sum()
            }),
        ))
      })
      .transpose()?
      .unwrap_or_default();
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      has_properties: properties.is_some(),
      schema_refs,
      text_len,
    })
  }
}

impl CustomDataResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &CustomDataPropertiesPart) -> Result<Self> {
    let has_custom_data = part.custom_data_part(package).is_some();
    let item = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      has_custom_data,
      text_len: item.id.len(),
      has_extensions: item.extension_list.is_some(),
    })
  }
}

impl SlicerCacheResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &SlicerCachePart) -> Result<Self> {
    let cache = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      name_len: cache.name.len(),
      source_name_len: cache.source_name.len(),
      pivot_tables: cache
        .slicer_cache_pivot_tables
        .as_ref()
        .map_or(0, |tables| tables.slicer_cache_pivot_table.len()),
      has_data: cache.slicer_cache_data.is_some(),
      extension_markers: cache
        .slicer_cache_definition_extension_list
        .as_ref()
        .map_or(0, |list| list.slicer_cache_definition_extension.len()),
    })
  }
}

impl TimelineCacheResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &TimeLineCachePart) -> Result<Self> {
    let cache = part.root_element(package)?;
    let state = &cache.timeline_state;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      name_len: cache.name.len(),
      source_name_len: cache.source_name.len(),
      pivot_tables: cache
        .timeline_cache_pivot_tables
        .as_ref()
        .map_or(0, |tables| tables.timeline_cache_pivot_table.len()),
      state_flags: usize::from(
        state
          .single_range_filter_state
          .is_some_and(|value| value.as_bool()),
      ) + usize::from(state.filter_id.is_some())
        + usize::from(state.filter_tab_id.is_some())
        + usize::from(state.selection_timeline_range.is_some())
        + usize::from(state.moving_period_state.is_some())
        + usize::from(state.extension_list.is_some()),
      text_len: state
        .filter_pivot_name
        .as_ref()
        .map_or(0, |value| value.len())
        + state.minimal_refresh_version as usize
        + state.last_refresh_version as usize
        + state.pivot_cache_id as usize
        + state.filter_id.unwrap_or_default() as usize
        + state.filter_tab_id.unwrap_or_default() as usize,
      extension_markers: usize::from(cache.extension_list.is_some()),
    })
  }
}

impl RichValueResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &RdRichValuePart) -> Result<Self> {
    let data = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      declared_count: data.count,
      rich_values: data.rich_value.len(),
      values: data.rich_value.iter().map(|value| value.value.len()).sum(),
      fallbacks: data
        .rich_value
        .iter()
        .filter(|value| value.rich_value_fallback.is_some())
        .count(),
      text_len: data
        .rich_value
        .iter()
        .map(|value| {
          value.s as usize
            + value.value.iter().map(|value| value.len()).sum::<usize>()
            + value
              .rich_value_fallback
              .as_ref()
              .and_then(|fallback| fallback.xml_content.as_ref())
              .map_or(0, |value| value.len())
        })
        .sum(),
      has_extensions: data.extension_list.is_some(),
    })
  }
}

impl RichValueStructureResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &RdRichValueStructurePart) -> Result<Self> {
    let structures = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      declared_count: structures.count,
      structures: structures.rich_value_structure.len(),
      keys: structures
        .rich_value_structure
        .iter()
        .map(|structure| structure.key.len())
        .sum(),
      text_len: structures
        .rich_value_structure
        .iter()
        .map(|structure| {
          structure.t.len() + structure.key.iter().map(|key| key.n.len()).sum::<usize>()
        })
        .sum(),
      has_extensions: structures.extension_list.is_some(),
    })
  }
}

impl ArrayDataResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &RdArrayPart) -> Result<Self> {
    let arrays = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      declared_count: arrays.count,
      arrays: arrays.array.len(),
      values: arrays
        .array
        .iter()
        .map(|array| array.array_value.len())
        .sum(),
      text_len: arrays
        .array
        .iter()
        .map(|array| {
          array.r as usize
            + array.c.unwrap_or_default() as usize
            + array
              .array_value
              .iter()
              .filter_map(|value| value.xml_content.as_ref())
              .map(|value| value.len())
              .sum::<usize>()
        })
        .sum(),
      has_extensions: arrays.extension_list.is_some(),
    })
  }
}

impl RichStylesResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &RichStylesPart) -> Result<Self> {
    let stylesheet = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      has_dxfs: stylesheet.dxfs.is_some(),
      has_properties: stylesheet.rich_format_properties.is_some(),
      styles: stylesheet
        .rich_styles
        .as_ref()
        .map_or(0, |styles| styles.rich_style.len()),
      style_values: stylesheet.rich_styles.as_ref().map_or(0, |styles| {
        styles
          .rich_style
          .iter()
          .map(|style| style.rich_style_property_value.len())
          .sum()
      }),
      has_extensions: stylesheet.extension_list.is_some(),
    })
  }
}

impl SupportingPropertyBagResource {
  fn from_part(
    package: &mut SpreadsheetDocument,
    part: &RdSupportingPropertyBagPart,
  ) -> Result<Self> {
    let bags = part.root_element(package)?;
    let arrays = bags
      .supporting_property_bag_array_data
      .as_ref()
      .map_or(0, |data| data.supporting_property_bag_array.len());
    let array_extensions = bags
      .supporting_property_bag_array_data
      .as_ref()
      .map_or(0, |data| usize::from(data.extension_list.is_some()));
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      arrays,
      bags: bags
        .supporting_property_bag_data
        .supporting_property_bag
        .len(),
      values: bags
        .supporting_property_bag_data
        .supporting_property_bag
        .iter()
        .map(|bag| bag.supporting_property_bag_value.len())
        .sum::<usize>()
        + bags
          .supporting_property_bag_array_data
          .as_ref()
          .map_or(0, |data| {
            data
              .supporting_property_bag_array
              .iter()
              .map(|array| array.supporting_property_bag_array_value.len())
              .sum()
          }),
      extension_markers: usize::from(bags.supporting_property_bag_data.extension_list.is_some())
        + array_extensions,
    })
  }
}

impl SupportingPropertyBagStructureResource {
  fn from_part(
    package: &mut SpreadsheetDocument,
    part: &RdSupportingPropertyBagStructurePart,
  ) -> Result<Self> {
    let structures = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      declared_count: structures.count,
      structures: structures.supporting_property_bag_structure.len(),
      keys: structures
        .supporting_property_bag_structure
        .iter()
        .map(|structure| structure.supporting_property_bag_key.len())
        .sum(),
      text_len: structures
        .supporting_property_bag_structure
        .iter()
        .flat_map(|structure| structure.supporting_property_bag_key.iter())
        .map(|key| key.n.len())
        .sum(),
      has_extensions: structures.extension_list.is_some(),
    })
  }
}

impl RichValueTypesResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &RdRichValueTypesPart) -> Result<Self> {
    let info = part.root_element(package)?;
    let type_records = info
      .rich_value_types
      .as_ref()
      .map_or(&[][..], |types| types.rich_value_type.as_slice());
    let global_reserved = info
      .rich_value_global_type
      .as_ref()
      .and_then(|global| global.rich_value_type_key_flags.as_ref())
      .map_or(0, |flags| flags.rich_value_type_reserved_key.len());
    let type_reserved = type_records
      .iter()
      .filter_map(|record| record.rich_value_type_key_flags.as_ref())
      .map(|flags| flags.rich_value_type_reserved_key.len())
      .sum::<usize>();
    let reserved_key_flags = info
      .rich_value_global_type
      .as_ref()
      .and_then(|global| global.rich_value_type_key_flags.as_ref())
      .map_or(0, rich_value_reserved_key_flag_count)
      + type_records
        .iter()
        .filter_map(|record| record.rich_value_type_key_flags.as_ref())
        .map(rich_value_reserved_key_flag_count)
        .sum::<usize>();
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      has_global_type: info.rich_value_global_type.is_some(),
      types: type_records.len(),
      key_flags: usize::from(
        info
          .rich_value_global_type
          .as_ref()
          .and_then(|global| global.rich_value_type_key_flags.as_ref())
          .is_some(),
      ) + type_records
        .iter()
        .filter(|record| record.rich_value_type_key_flags.is_some())
        .count(),
      reserved_keys: global_reserved + type_reserved,
      reserved_key_flags,
      text_len: type_records
        .iter()
        .map(|record| {
          record.name.len()
            + record
              .rich_value_type_key_flags
              .as_ref()
              .map_or(0, rich_value_reserved_key_text_len)
        })
        .sum::<usize>(),
      has_extensions: info.extension_list.is_some()
        || info
          .rich_value_global_type
          .as_ref()
          .is_some_and(|global| global.extension_list.is_some())
        || type_records
          .iter()
          .any(|record| record.extension_list.is_some()),
    })
  }
}

impl RichValueWebImageResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &RdRichValueWebImagePart) -> Result<Self> {
    let images = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      images: images.web_image_supporting_rich_data.len(),
      address_relationships: images.web_image_supporting_rich_data.len(),
      more_images_relationships: images
        .web_image_supporting_rich_data
        .iter()
        .filter(|image| {
          image
            .more_images_address_web_image_supporting_rich_data_relationship
            .is_some()
        })
        .count(),
      blip_relationships: images
        .web_image_supporting_rich_data
        .iter()
        .filter(|image| {
          image
            .blip_web_image_supporting_rich_data_relationship
            .is_some()
        })
        .count(),
      has_extensions: images.extension_list.is_some(),
    })
  }
}

impl FeaturePropertyBagsResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &FeaturePropertyBagsPart) -> Result<Self> {
    let bags = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      declared_count: bags.count.unwrap_or_default(),
      bag_extensions: bags.bag_extensions.len(),
      bags: bags.feature_property_bag.len(),
      values: bags
        .feature_property_bag
        .iter()
        .map(|bag| bag.feature_property_bag_choice.len())
        .sum(),
      text_len: bags
        .feature_property_bag
        .iter()
        .map(feature_property_bag_text_len)
        .sum(),
      extension_markers: usize::from(bags.extension_list.is_some())
        + bags
          .bag_extensions
          .iter()
          .filter(|bag| bag.extension_list.is_some())
          .count(),
    })
  }
}

impl WorkbookUserDataResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &WorkbookUserDataPart) -> Result<Self> {
    let users = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      declared_count: users.count.unwrap_or_default(),
      users: users.user_info.len(),
      text_len: users
        .user_info
        .iter()
        .map(|user| user.guid.len() + user.name.len() + user.date_time.len())
        .sum(),
      extension_markers: users
        .user_info
        .iter()
        .filter(|user| user.extension_list.is_some())
        .count(),
    })
  }
}

impl CalculationChainResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &CalculationChainPart) -> Result<Self> {
    let chain = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      cells: chain.calculation_cell.len(),
      flag_count: chain
        .calculation_cell
        .iter()
        .map(|cell| {
          usize::from(cell.in_child_chain.is_some_and(|value| value.as_bool()))
            + usize::from(cell.new_level.is_some_and(|value| value.as_bool()))
            + usize::from(cell.new_thread.is_some_and(|value| value.as_bool()))
            + usize::from(cell.array.is_some_and(|value| value.as_bool()))
        })
        .sum(),
      text_len: chain
        .calculation_cell
        .iter()
        .map(|cell| {
          cell.cell_reference.len() + cell.sheet_id.unwrap_or_default().unsigned_abs() as usize
        })
        .sum(),
      has_extensions: chain.extension_list.is_some(),
    })
  }
}

impl CellMetadataResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &CellMetadataPart) -> Result<Self> {
    let metadata = part.root_element(package)?;
    let cell_blocks = metadata
      .cell_metadata
      .as_ref()
      .map_or(0, |blocks| blocks.metadata_block.len());
    let value_blocks = metadata
      .value_metadata
      .as_ref()
      .map_or(0, |blocks| blocks.metadata_block.len());
    let cell_records = metadata.cell_metadata.as_ref().map_or(0, |blocks| {
      blocks
        .metadata_block
        .iter()
        .map(|block| block.metadata_record.len())
        .sum()
    });
    let value_records = metadata.value_metadata.as_ref().map_or(0, |blocks| {
      blocks
        .metadata_block
        .iter()
        .map(|block| block.metadata_record.len())
        .sum()
    });
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      metadata_types: metadata
        .metadata_types
        .as_ref()
        .map_or(0, |types| types.metadata_type.len()),
      metadata_strings: metadata
        .metadata_strings
        .as_ref()
        .map_or(0, |strings| strings.character_value.len()),
      mdx_records: metadata
        .mdx_metadata
        .as_ref()
        .map_or(0, |mdx| mdx.mdx.len()),
      future_metadata: metadata.future_metadata.len(),
      cell_blocks,
      value_blocks,
      records: cell_records + value_records,
      text_len: metadata.metadata_types.as_ref().map_or(0, |types| {
        types
          .metadata_type
          .iter()
          .map(|metadata_type| metadata_type.name.len())
          .sum()
      }) + metadata.metadata_strings.as_ref().map_or(0, |strings| {
        strings
          .character_value
          .iter()
          .map(|value| value.val.len())
          .sum()
      }) + metadata
        .future_metadata
        .iter()
        .map(|future| future.name.len())
        .sum::<usize>(),
      extension_markers: usize::from(metadata.extension_list.is_some())
        + metadata
          .future_metadata
          .iter()
          .filter(|future| future.extension_list.is_some())
          .count(),
    })
  }
}

impl VolatileDependenciesResource {
  fn from_part(package: &mut SpreadsheetDocument, part: &VolatileDependenciesPart) -> Result<Self> {
    let dependencies = part.root_element(package)?;
    Ok(Self {
      relationship_id: part.relationship_id().map(ToString::to_string),
      types: dependencies.volatile_type.len(),
      has_extensions: dependencies.extension_list.is_some(),
    })
  }
}

fn feature_property_bag_text_len(bag: &xfpb::FeaturePropertyBag) -> usize {
  bag.r#type.len()
    + bag.ext_ref.as_ref().map_or(0, |value| value.len())
    + bag.att.as_ref().map_or(0, |value| value.len())
    + bag
      .feature_property_bag_choice
      .iter()
      .map(feature_property_bag_choice_text_len)
      .sum::<usize>()
}

fn feature_property_bag_choice_text_len(choice: &xfpb::FeaturePropertyBagChoice) -> usize {
  match choice {
    xfpb::FeaturePropertyBagChoice::ArrayFeatureProperty(property) => {
      property.k.len() + property.array_feature_property_choice.len()
    }
    xfpb::FeaturePropertyBagChoice::BagFeatureProperty(property) => {
      property.k.len() + usize::from(property.xml_content.is_some())
    }
    xfpb::FeaturePropertyBagChoice::IntFeatureProperty(property) => {
      property.k.len() + usize::from(property.xml_content.is_some())
    }
    xfpb::FeaturePropertyBagChoice::StringFeatureProperty(property) => {
      property.k.len() + property.xml_content.as_ref().map_or(0, |value| value.len())
    }
    xfpb::FeaturePropertyBagChoice::BoolFeatureProperty(property) => {
      property.k.len()
        + property
          .xml_content
          .as_ref()
          .map_or(0, |value| usize::from(value.as_bool()))
    }
    xfpb::FeaturePropertyBagChoice::DecimalFeatureProperty(property) => {
      property.k.len() + usize::from(property.xml_content.is_some())
    }
    xfpb::FeaturePropertyBagChoice::RelFeatureProperty(property) => {
      property.k.len() + property.xml_content.as_ref().map_or(0, |value| value.len())
    }
  }
}

fn rich_value_reserved_key_flag_count(flags: &xlrd2::RichValueTypeKeyFlags) -> usize {
  flags
    .rich_value_type_reserved_key
    .iter()
    .map(|key| key.rich_value_type_reserved_key_flag.len())
    .sum()
}

fn rich_value_reserved_key_text_len(flags: &xlrd2::RichValueTypeKeyFlags) -> usize {
  flags
    .rich_value_type_reserved_key
    .iter()
    .map(|key| {
      key.name.len()
        + key
          .rich_value_type_reserved_key_flag
          .iter()
          .map(|flag| flag.name.len() + usize::from(flag.value.as_bool()))
          .sum::<usize>()
    })
    .sum()
}
