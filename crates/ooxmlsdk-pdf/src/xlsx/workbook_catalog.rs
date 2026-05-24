use ooxmlsdk::parts::external_workbook_part::ExternalWorkbookPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::parts::workbook_person_part::WorkbookPersonPart;
use ooxmlsdk::schemas::schemas_microsoft_com_office_spreadsheetml_2018_threadedcomments as tc;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

use crate::error::Result;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WorkbookCatalog {
  pub(crate) external_links: Vec<ExternalLinkModel>,
  pub(crate) xml_maps: Option<XmlMapsModel>,
  pub(crate) persons: Vec<PersonModel>,
  pub(crate) revisions: Option<RevisionHeadersModel>,
  pub(crate) relationship_resources: WorkbookRelationshipResources,
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
  pub(crate) custom_xml_parts: usize,
  pub(crate) custom_data_properties: usize,
  pub(crate) slicer_caches: usize,
  pub(crate) timeline_caches: usize,
  pub(crate) rich_value_parts: usize,
  pub(crate) rich_value_structure_parts: usize,
  pub(crate) rd_array_parts: usize,
  pub(crate) rich_styles_parts: usize,
  pub(crate) supporting_property_bags: usize,
  pub(crate) supporting_property_bag_structures: usize,
  pub(crate) rich_value_types: usize,
  pub(crate) has_rich_value_web_image: bool,
  pub(crate) has_feature_property_bags: bool,
  pub(crate) has_vba_project: bool,
  pub(crate) has_attached_toolbars: bool,
  pub(crate) has_user_data: bool,
  pub(crate) has_calculation_chain: bool,
  pub(crate) has_cell_metadata: bool,
  pub(crate) has_volatile_dependencies: bool,
}

impl WorkbookCatalog {
  pub(crate) fn from_workbook_part(
    package: &mut SpreadsheetDocument,
    workbook_part: &WorkbookPart,
  ) -> Result<Self> {
    // Source: LibreOffice sc/source/filter/oox/workbookfragment.cxx
    // finalizeImport imports persons, connections, custom XML, xmlMaps,
    // external links, revisions, and VBA through workbook-level relationships.
    let external_parts = workbook_part
      .external_workbook_parts(package)
      .collect::<Vec<_>>();
    let person_parts = workbook_part
      .workbook_person_parts(package)
      .collect::<Vec<_>>();
    let xml_maps_part = workbook_part.custom_xml_mappings_part(package);
    let revision_part = workbook_part.workbook_revision_header_part(package);
    let relationship_resources =
      WorkbookRelationshipResources::from_workbook_part(package, workbook_part);

    Ok(Self {
      external_links: external_parts
        .iter()
        .map(|part| ExternalLinkModel::from_part(package, part))
        .collect::<Result<Vec<_>>>()?,
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
  fn from_workbook_part(package: &mut SpreadsheetDocument, workbook_part: &WorkbookPart) -> Self {
    Self {
      custom_xml_parts: workbook_part.custom_xml_parts(package).count(),
      custom_data_properties: workbook_part.custom_data_properties_parts(package).count(),
      slicer_caches: workbook_part.slicer_cache_parts(package).count(),
      timeline_caches: workbook_part.time_line_cache_parts(package).count(),
      rich_value_parts: workbook_part.rd_rich_value_parts(package).count(),
      rich_value_structure_parts: workbook_part
        .ct_rd_rich_value_structure_parts(package)
        .count(),
      rd_array_parts: workbook_part.rd_array_parts(package).count(),
      rich_styles_parts: workbook_part.rich_styles_parts(package).count(),
      supporting_property_bags: workbook_part
        .rd_supporting_property_bag_parts(package)
        .count(),
      supporting_property_bag_structures: workbook_part
        .rd_supporting_property_bag_structure_parts(package)
        .count(),
      rich_value_types: workbook_part.rd_rich_value_types_parts(package).count(),
      has_rich_value_web_image: workbook_part
        .rd_rich_value_web_image_part(package)
        .is_some(),
      has_feature_property_bags: workbook_part.feature_property_bags_part(package).is_some(),
      has_vba_project: workbook_part.vba_project_part(package).is_some(),
      has_attached_toolbars: workbook_part
        .excel_attached_toolbars_part(package)
        .is_some(),
      has_user_data: workbook_part.workbook_user_data_part(package).is_some(),
      has_calculation_chain: workbook_part.calculation_chain_part(package).is_some(),
      has_cell_metadata: workbook_part.cell_metadata_part(package).is_some(),
      has_volatile_dependencies: workbook_part.volatile_dependencies_part(package).is_some(),
    }
  }
}
