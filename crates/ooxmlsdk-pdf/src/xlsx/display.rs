use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::docx::PageSetup;
use crate::layout::{self, LayoutDocument};

use super::import::ExcelImport;
use super::print::{CalcPrintDocument, CalcPrintPage};
use super::worksheet::{CalcSheet, SheetType};

pub(crate) fn lower_to_layout_document(import: &ExcelImport) -> LayoutDocument {
  let mut pages = Vec::new();
  if import.workbook_resources.has_theme
    || import.workbook_resources.has_styles
    || import.workbook_resources.external_workbooks > 0
    || import.workbook_resources.pivot_cache_definitions > 0
    || import.workbook_resources.workbook_persons > 0
    || !import.workbook_catalog.external_links.is_empty()
    || import.workbook_catalog.xml_maps.is_some()
    || import.workbook_catalog.revisions.is_some()
  {
    pages.push((PageSetup::default(), workbook_lines(import)));
  }
  pages.extend(
    CalcPrintDocument::from_import(import)
      .pages
      .iter()
      .map(|page| (PageSetup::default(), print_page_lines(import, page))),
  );
  layout::text_pages(pages)
}

fn workbook_lines(import: &ExcelImport) -> Vec<String> {
  let mut lines = vec![format!(
    "Workbook sheets={} theme={} styles={} sharedStrings={} definedNames={} externalWorkbooks={} pivotCaches={} persons={}",
    import.workbook.sheets.sheet.len(),
    import.workbook_resources.has_theme,
    import.workbook_resources.has_styles,
    import.shared_strings.len(),
    import.defined_names.records.len(),
    import.workbook_resources.external_workbooks,
    import.workbook_resources.pivot_cache_definitions,
    import.workbook_resources.workbook_persons
  )];

  if !import.pivot_caches.caches.is_empty() {
    let cache_fields = import
      .pivot_caches
      .caches
      .iter()
      .map(|cache| cache.cache_fields)
      .sum::<usize>();
    let cache_records = import
      .pivot_caches
      .caches
      .iter()
      .filter_map(|cache| cache.record_count)
      .sum::<u32>();
    let cache_flags = import
      .pivot_caches
      .caches
      .iter()
      .filter(|cache| {
        cache.refresh_on_load
          || cache.save_data.is_some()
          || cache.invalid
          || cache.has_records_part
          || cache.has_cache_source
          || cache.has_extensions
      })
      .count();
    let cache_ref_len = import
      .pivot_caches
      .caches
      .iter()
      .map(|cache| {
        cache
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + cache.workbook_cache_id.unwrap_or_default() as usize
          + cache
            .workbook_relationship_id
            .as_ref()
            .map_or(0, |value| value.len())
          + cache
            .definition_relationship_id
            .as_ref()
            .map_or(0, |value| value.len())
          + cache.optional_child_count
      })
      .sum::<usize>();
    lines.push(format!(
      "pivotCaches caches={} fields={} records={} flags={} refLen={}",
      import.pivot_caches.caches.len(),
      cache_fields,
      cache_records,
      cache_flags,
      cache_ref_len
    ));
  }

  if !import.connections.connections.is_empty() {
    let connection_flags = import
      .connections
      .connections
      .iter()
      .filter(|connection| {
        connection.refresh_on_load
          || connection.save_data.is_some()
          || connection.has_database_properties
          || connection.has_olap_properties
          || connection.has_web_query_properties
          || connection.has_text_properties
          || connection.has_extensions
      })
      .count();
    let parameter_count = import
      .connections
      .connections
      .iter()
      .map(|connection| connection.parameter_count)
      .sum::<usize>();
    let connection_ref_len = import
      .connections
      .connections
      .iter()
      .map(|connection| {
        connection.id as usize
          + connection.name.as_ref().map_or(0, |value| value.len())
          + connection.connection_type.unwrap_or_default() as usize
          + connection
            .source_file
            .as_ref()
            .map_or(0, |value| value.len())
          + connection
            .connection_file
            .as_ref()
            .map_or(0, |value| value.len())
          + connection.refreshed_version as usize
      })
      .sum::<usize>();
    lines.push(format!(
      "connections rel={} count={} flags={} parameters={} refLen={}",
      import.connections.relationship_id.as_deref().unwrap_or(""),
      import.connections.connections.len(),
      connection_flags,
      parameter_count,
      connection_ref_len
    ));
  }

  if !import.workbook_catalog.external_links.is_empty() {
    let external_book_links = import
      .workbook_catalog
      .external_links
      .iter()
      .filter(|link| {
        matches!(
          link.kind,
          super::workbook_catalog::ExternalLinkKind::ExternalBook { .. }
        )
      })
      .count();
    let dde_links = import
      .workbook_catalog
      .external_links
      .iter()
      .filter(|link| {
        matches!(
          link.kind,
          super::workbook_catalog::ExternalLinkKind::Dde { .. }
        )
      })
      .count();
    let ole_links = import
      .workbook_catalog
      .external_links
      .iter()
      .filter(|link| {
        matches!(
          link.kind,
          super::workbook_catalog::ExternalLinkKind::Ole { .. }
        )
      })
      .count();
    let external_items = import
      .workbook_catalog
      .external_links
      .iter()
      .map(|link| {
        link.sheet_names
          + link.defined_names
          + link.cached_sheet_data
          + link.item_count
          + usize::from(link.has_extensions)
          + link.relationship_id.as_ref().map_or(0, |value| value.len())
          + external_link_kind_len(&link.kind)
      })
      .sum::<usize>();
    lines.push(format!(
      "externalLinks links={} books={} dde={} ole={} items={}",
      import.workbook_catalog.external_links.len(),
      external_book_links,
      dde_links,
      ole_links,
      external_items
    ));
  }

  if let Some(xml_maps) = &import.workbook_catalog.xml_maps {
    lines.push(format!(
      "xmlMaps rel={} schemas={} maps={} schemaRefs={} mapFlags={} namespaceLen={}",
      xml_maps.relationship_id.as_deref().unwrap_or(""),
      xml_maps.schemas,
      xml_maps.maps,
      xml_maps.schema_ref_count,
      xml_maps.map_flag_count,
      xml_maps.selection_namespaces.len()
    ));
  }

  if !import.workbook_catalog.persons.is_empty() {
    let person_count = import
      .workbook_catalog
      .persons
      .iter()
      .map(|part| part.persons)
      .sum::<usize>();
    let person_text_len = import
      .workbook_catalog
      .persons
      .iter()
      .map(|part| {
        part.id_text_len
          + part.relationship_id.as_ref().map_or(0, |value| value.len())
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    lines.push(format!(
      "persons parts={} persons={} textLen={}",
      import.workbook_catalog.persons.len(),
      person_count,
      person_text_len
    ));
  }

  if let Some(revisions) = &import.workbook_catalog.revisions {
    lines.push(format!(
      "revisions rel={} headers={} logs={} flags={} textLen={} guidLen={}",
      revisions.relationship_id.as_deref().unwrap_or(""),
      revisions.headers,
      revisions.revision_logs,
      revisions.flag_count,
      revisions.user_text_len,
      revisions.guid.len() + revisions.last_guid.as_ref().map_or(0, |value| value.len())
    ));
  }

  let relationship_resources = &import.workbook_catalog.relationship_resources;
  let relationship_resource_count = relationship_resources.custom_xml_parts.len()
    + relationship_resources.custom_data_properties.len()
    + relationship_resources.slicer_caches.len()
    + relationship_resources.timeline_caches.len()
    + relationship_resources.rich_values.len()
    + relationship_resources.rich_value_structures.len()
    + relationship_resources.arrays.len()
    + relationship_resources.rich_styles.len()
    + relationship_resources.supporting_property_bags.len()
    + relationship_resources
      .supporting_property_bag_structures
      .len()
    + relationship_resources.rich_value_types.len()
    + usize::from(relationship_resources.rich_value_web_image.is_some())
    + usize::from(relationship_resources.feature_property_bags.is_some())
    + usize::from(relationship_resources.has_vba_project)
    + usize::from(relationship_resources.has_attached_toolbars)
    + usize::from(relationship_resources.user_data.is_some())
    + usize::from(relationship_resources.calculation_chain.is_some())
    + usize::from(relationship_resources.cell_metadata.is_some())
    + usize::from(relationship_resources.volatile_dependencies.is_some());
  if relationship_resource_count > 0 {
    let custom_xml_flags = relationship_resources
      .custom_xml_parts
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + usize::from(part.has_properties)
          + part.schema_refs
          + part.text_len
      })
      .sum::<usize>();
    let custom_data_flags = relationship_resources
      .custom_data_properties
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + usize::from(part.has_custom_data)
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let slicer_flags = relationship_resources
      .slicer_caches
      .iter()
      .map(|cache| {
        usize::from(cache.relationship_id.is_some())
          + cache.name_len
          + cache.source_name_len
          + cache.pivot_tables
          + usize::from(cache.has_data)
          + cache.extension_markers
      })
      .sum::<usize>();
    let timeline_flags = relationship_resources
      .timeline_caches
      .iter()
      .map(|cache| {
        usize::from(cache.relationship_id.is_some())
          + cache.name_len
          + cache.source_name_len
          + cache.pivot_tables
          + cache.state_flags
          + cache.text_len
          + cache.extension_markers
      })
      .sum::<usize>();
    let rich_value_items = relationship_resources
      .rich_values
      .iter()
      .map(|part| part.rich_values + part.values + part.fallbacks)
      .sum::<usize>();
    let rich_value_flags = relationship_resources
      .rich_values
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let rich_structure_items = relationship_resources
      .rich_value_structures
      .iter()
      .map(|part| part.structures + part.keys)
      .sum::<usize>();
    let rich_structure_flags = relationship_resources
      .rich_value_structures
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let array_items = relationship_resources
      .arrays
      .iter()
      .map(|part| part.arrays + part.values)
      .sum::<usize>();
    let array_flags = relationship_resources
      .arrays
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let rich_style_flags = relationship_resources
      .rich_styles
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + usize::from(part.has_dxfs)
          + usize::from(part.has_properties)
          + part.styles
          + part.style_values
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let property_bag_flags = relationship_resources
      .supporting_property_bags
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.arrays
          + part.bags
          + part.values
          + part.extension_markers
      })
      .sum::<usize>();
    let property_bag_structure_flags = relationship_resources
      .supporting_property_bag_structures
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.structures
          + part.keys
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let rich_value_type_flags = relationship_resources
      .rich_value_types
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + usize::from(part.has_global_type)
          + part.types
          + part.key_flags
          + part.reserved_keys
          + part.reserved_key_flags
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let web_image_flags = relationship_resources
      .rich_value_web_image
      .as_ref()
      .map_or(0, |part| {
        usize::from(part.relationship_id.is_some())
          + part.images
          + part.address_relationships
          + part.more_images_relationships
          + part.blip_relationships
          + usize::from(part.has_extensions)
      });
    let feature_bag_flags = relationship_resources
      .feature_property_bags
      .as_ref()
      .map_or(0, |part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.bag_extensions
          + part.bags
          + part.values
          + part.text_len
          + part.extension_markers
      });
    let user_data_flags = relationship_resources.user_data.as_ref().map_or(0, |part| {
      usize::from(part.relationship_id.is_some())
        + part.declared_count as usize
        + part.users
        + part.text_len
        + part.extension_markers
    });
    let calculation_chain_flags =
      relationship_resources
        .calculation_chain
        .as_ref()
        .map_or(0, |part| {
          usize::from(part.relationship_id.is_some())
            + part.cells
            + part.flag_count
            + part.text_len
            + usize::from(part.has_extensions)
        });
    let cell_metadata_flags = relationship_resources
      .cell_metadata
      .as_ref()
      .map_or(0, |part| {
        usize::from(part.relationship_id.is_some())
          + part.metadata_types
          + part.metadata_strings
          + part.mdx_records
          + part.future_metadata
          + part.cell_blocks
          + part.value_blocks
          + part.records
          + part.text_len
          + part.extension_markers
      });
    let volatile_dependency_flags = relationship_resources
      .volatile_dependencies
      .as_ref()
      .map_or(0, |part| {
        usize::from(part.relationship_id.is_some()) + part.types + usize::from(part.has_extensions)
      });
    lines.push(format!(
      "workbook relationshipResources customXml={} customXmlFlags={} customDataProps={} customDataFlags={} slicers={} slicerFlags={} timelines={} timelineFlags={} richValues={} richValueItems={} richValueFlags={} richValueStructs={} richStructItems={} richStructFlags={} arrays={} arrayItems={} arrayFlags={} richStyles={} richStyleFlags={} propBags={} propBagFlags={} propBagStructs={} propBagStructFlags={} richValueTypes={} richValueTypeFlags={} webImage={} webImageFlags={} featureBags={} featureBagFlags={} vba={} toolbars={} users={} userFlags={} calcChain={} calcChainFlags={} cellMetadata={} cellMetadataFlags={} volatileDeps={} volatileDepFlags={}",
      relationship_resources.custom_xml_parts.len(),
      custom_xml_flags,
      relationship_resources.custom_data_properties.len(),
      custom_data_flags,
      relationship_resources.slicer_caches.len(),
      slicer_flags,
      relationship_resources.timeline_caches.len(),
      timeline_flags,
      relationship_resources.rich_values.len(),
      rich_value_items,
      rich_value_flags,
      relationship_resources.rich_value_structures.len(),
      rich_structure_items,
      rich_structure_flags,
      relationship_resources.arrays.len(),
      array_items,
      array_flags,
      relationship_resources.rich_styles.len(),
      rich_style_flags,
      relationship_resources.supporting_property_bags.len(),
      property_bag_flags,
      relationship_resources.supporting_property_bag_structures.len(),
      property_bag_structure_flags,
      relationship_resources.rich_value_types.len(),
      rich_value_type_flags,
      relationship_resources.rich_value_web_image.is_some(),
      web_image_flags,
      relationship_resources.feature_property_bags.is_some(),
      feature_bag_flags,
      relationship_resources.has_vba_project,
      relationship_resources.has_attached_toolbars,
      relationship_resources.user_data.is_some(),
      user_data_flags,
      relationship_resources.calculation_chain.is_some(),
      calculation_chain_flags,
      relationship_resources.cell_metadata.is_some(),
      cell_metadata_flags,
      relationship_resources.volatile_dependencies.is_some(),
      volatile_dependency_flags
    ));
  }

  if import.workbook_resources.has_styles {
    lines.push(format!(
      "styles numFmts={} fonts={} fills={} borders={} cellStyleXfs={} cellXfs={} cellStyles={} dxfs={} tableStyles={} defaultTableStyle={} defaultPivotStyle={} colors={} extensions={}",
      import.styles.custom_number_formats.len(),
      import.styles.fonts,
      import.styles.fills,
      import.styles.borders,
      import.styles.cell_style_formats,
      import.styles.cell_formats,
      import.styles.cell_styles,
      import.styles.differential_formats,
      import.styles.table_styles,
      import.styles.default_table_style.as_deref().unwrap_or(""),
      import.styles.default_pivot_style.as_deref().unwrap_or(""),
      import.styles.has_colors,
      import.styles.has_extensions
    ));
  }

  if !import.defined_names.records.is_empty() {
    lines.push(format!(
      "definedNames printAreas={} printTitles={} filters={} local={} hidden={}",
      import.defined_names.print_areas,
      import.defined_names.print_titles,
      import.defined_names.filter_databases,
      import.defined_names.local_names,
      import.defined_names.hidden_names
    ));
  }

  lines.push(format!(
    "workbook settings date1904={} dateCompatibility={} showObjects={:?} saveExternalLinks={} updateLinks={:?} codeName={} defaultThemeVersion={} fileVersion={} fileApp={} readOnlyRecommended={} lockStructure={} lockWindows={} lockRevision={} workbookPassword={} revisionPassword={} views={} customViews={} externalRefs={} functionGroups={} pivotCachesXml={} webPublishObjects={} recoveryProps={} extensions={} oleSize={}",
    import.globals.settings.date_1904,
    import
      .globals
      .settings
      .date_compatibility
      .map_or(String::new(), |value| value.to_string()),
    import.globals.settings.show_objects,
    import
      .globals
      .settings
      .save_external_link_values
      .map_or(String::new(), |value| value.to_string()),
    import.globals.settings.update_links,
    import.globals.settings.code_name.as_deref().unwrap_or(""),
    import
      .globals
      .settings
      .default_theme_version
      .map_or(String::new(), |value| value.to_string()),
    import.globals.settings.has_file_version,
    import
      .globals
      .settings
      .file_application_name
      .as_deref()
      .unwrap_or(""),
    import.globals.settings.read_only_recommended,
    import
      .globals
      .settings
      .workbook_protection
      .lock_structure,
    import.globals.settings.workbook_protection.lock_windows,
    import.globals.settings.workbook_protection.lock_revision,
    import
      .globals
      .settings
      .workbook_protection
      .has_workbook_password,
    import
      .globals
      .settings
      .workbook_protection
      .has_revision_password,
    import.globals.views.len(),
    import.globals.custom_workbook_views,
    import.globals.external_references,
    import.globals.function_groups,
    import.globals.pivot_caches,
    import.globals.web_publish_objects,
    import.globals.file_recovery_properties,
    import.globals.has_workbook_extensions,
    import.globals.ole_size_reference.as_deref().unwrap_or("")
  ));

  if let Some(view) = import.globals.views.first() {
    lines.push(format!(
      "workbook view visibility={:?} minimized={} hScroll={} vScroll={} sheetTabs={} firstSheet={} activeTab={} tabRatio={} window={}x{} extensions={}",
      view.visibility,
      view.minimized,
      view
        .show_horizontal_scroll
        .map_or(String::new(), |value| value.to_string()),
      view
        .show_vertical_scroll
        .map_or(String::new(), |value| value.to_string()),
      view
        .show_sheet_tabs
        .map_or(String::new(), |value| value.to_string()),
      view.first_sheet.map_or(String::new(), |value| value.to_string()),
      view.active_tab.map_or(String::new(), |value| value.to_string()),
      view.tab_ratio.map_or(String::new(), |value| value.to_string()),
      view
        .window_width
        .map_or(String::new(), |value| value.to_string()),
      view
        .window_height
        .map_or(String::new(), |value| value.to_string()),
      view.has_extensions
    ));
  }

  lines.push(format!(
    "calcPr id={} mode={:?} refMode={:?} fullCalcOnLoad={} forceFullCalc={} iterate={} iterateCount={} iterateDelta={} fullPrecision={} calcCompleted={} calcOnSave={} concurrentCalc={} concurrentManualCount={}",
    import
      .globals
      .calculation
      .calculation_id
      .map_or(String::new(), |value| value.to_string()),
    import.globals.calculation.calculation_mode,
    import.globals.calculation.reference_mode,
    import.globals.calculation.full_calculation_on_load,
    import.globals.calculation.force_full_calculation,
    import.globals.calculation.iterate,
    import
      .globals
      .calculation
      .iterate_count
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .iterate_delta
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .full_precision
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .calculation_completed
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .calculation_on_save
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .concurrent_calculation
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .concurrent_manual_count
      .map_or(String::new(), |value| value.to_string())
  ));

  lines
}

fn external_link_kind_len(kind: &super::workbook_catalog::ExternalLinkKind) -> usize {
  match kind {
    super::workbook_catalog::ExternalLinkKind::ExternalBook { relationship_id } => {
      relationship_id.len()
    }
    super::workbook_catalog::ExternalLinkKind::Dde { service, topic } => {
      service.len() + topic.len()
    }
    super::workbook_catalog::ExternalLinkKind::Ole {
      relationship_id,
      prog_id,
    } => relationship_id.len() + prog_id.len(),
    super::workbook_catalog::ExternalLinkKind::Unknown => 0,
  }
}

fn print_page_lines(import: &ExcelImport, page: &CalcPrintPage<'_>) -> Vec<String> {
  let mut lines = sheet_lines(import, page.sheet);
  lines.insert(
    1,
    format!(
      "print page={} sheetPage={} zoom={} paper={} scale={} fit={}x{} dpi={}x{} margins={} grid={} headings={} headerFooterFlags={} headerFooterTextLen={} headerFooterDrawings={}",
      page.page_number,
      page.sheet_page_index + 1,
      page.zoom,
      page.page_settings.paper_size,
      page.page_settings.scale,
      page.page_settings.fit_to_width,
      page.page_settings.fit_to_height,
      page.page_settings.horizontal_dpi,
      page.page_settings.vertical_dpi,
      page.page_settings.has_margins,
      page.page_settings.print_grid_lines,
      page.page_settings.print_headings,
      page.page_settings.header_footer.flag_count(),
      page.page_settings.header_footer.text_len(),
      page.page_settings.header_footer.drawing_slot_count
    ),
  );
  if !page.named_ranges.print_areas.is_empty()
    || !page.named_ranges.print_titles.is_empty()
    || !page.named_ranges.filter_databases.is_empty()
  {
    lines.insert(
      2,
      format!(
        "print names areas={} titles={} filters={} unresolvedFormulas={}",
        page.named_ranges.print_areas.len(),
        page.named_ranges.print_titles.len(),
        page.named_ranges.filter_databases.len(),
        page.named_ranges.unresolved_formula_count()
      ),
    );
  }
  lines
}

fn sheet_lines(import: &ExcelImport, sheet: &CalcSheet) -> Vec<String> {
  let mut lines = vec![format!(
    "Sheet {}: {} id={} rel={}{}",
    sheet.workbook_index + 1,
    sheet.name,
    sheet.sheet_id,
    sheet.relationship_id,
    if sheet.visible() { "" } else { " (hidden)" }
  )];

  if sheet.sheet_type == SheetType::Chartsheet {
    lines.push("Chartsheet".to_string());
    if let Some(metrics) = &sheet.chartsheet_metrics {
      lines.push(format!(
        "chartsheet published={} codeName={} tabColor={} views={} selectedViews={} zoomToFitViews={} viewExt={} customViews={} customViewFlags={} protectionFlags={} webPublishItems={} extensions={}",
        metrics.published,
        metrics.code_name.as_deref().unwrap_or(""),
        metrics.has_tab_color,
        metrics.views,
        metrics.selected_views,
        metrics.zoom_to_fit_views,
        metrics.view_extensions,
        metrics.custom_views,
        metrics.custom_view_flags,
        metrics.protection_flags,
        metrics.web_publish_items,
        metrics.has_extensions
      ));
    }
  }

  let drawing_count = sheet.resources.drawings.len();
  let chart_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.chart_count())
    .sum::<usize>();
  let diagram_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.diagram_resource_count())
    .sum::<usize>();
  let drawing_image_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.images)
    .sum::<usize>();
  let drawing_anchor_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.anchors.len())
    .sum::<usize>();
  let chart_child_resource_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.chart_child_resource_count())
    .sum::<usize>();

  if drawing_count > 0
    || !sheet.resources.object_resources.vml_drawings.is_empty()
    || sheet.resources.comments.legacy_count() > 0
    || sheet.resources.comments.threaded_count() > 0
    || !sheet.resources.tables.is_empty()
    || !sheet.resources.pivot_tables.tables.is_empty()
    || !sheet.resources.query_tables.query_tables.is_empty()
    || !sheet.metrics.objects.ole_objects.is_empty()
    || !sheet.metrics.objects.controls.is_empty()
    || sheet.metrics.objects.unknown_controls > 0
    || sheet_relationship_count(sheet) > 0
  {
    lines.push(format!(
      "resources drawings={} anchors={} charts={} chartResources={} diagrams={} drawingImages={} vml={} comments={} threadedComments={} tables={} pivots={} queries={} oleObjects={} controls={} unknownControls={} sheetRelationships={}",
      drawing_count,
      drawing_anchor_count,
      chart_count,
      chart_child_resource_count,
      diagram_count,
      drawing_image_count,
      sheet.resources.object_resources.vml_drawings.len(),
      sheet.resources.comments.legacy_count(),
      sheet.resources.comments.threaded_count(),
      sheet.resources.tables.len(),
      sheet.resources.pivot_tables.tables.len(),
      sheet.resources.query_tables.query_tables.len(),
      sheet.metrics.objects.ole_objects.len(),
      sheet.metrics.objects.controls.len(),
      sheet.metrics.objects.unknown_controls,
      sheet_relationship_count(sheet)
    ));
  }

  if drawing_anchor_count > 0 {
    let mut two_cell = 0usize;
    let mut one_cell = 0usize;
    let mut absolute = 0usize;
    let mut pictures = 0usize;
    let mut graphic_frames = 0usize;
    let mut shapes = 0usize;
    let mut groups = 0usize;
    let mut connectors = 0usize;
    let mut content_parts = 0usize;
    let mut unknown = 0usize;
    let mut flags = 0usize;
    let mut ref_len = 0usize;
    for anchor in sheet
      .resources
      .drawings
      .iter()
      .flat_map(|drawing| drawing.anchors.iter())
    {
      match anchor.kind {
        super::drawing::DrawingAnchorKind::TwoCell => two_cell += 1,
        super::drawing::DrawingAnchorKind::OneCell => one_cell += 1,
        super::drawing::DrawingAnchorKind::Absolute => absolute += 1,
      }
      match anchor.object.kind {
        super::drawing::DrawingObjectKind::Shape => shapes += 1,
        super::drawing::DrawingObjectKind::GroupShape => groups += 1,
        super::drawing::DrawingObjectKind::GraphicFrame => graphic_frames += 1,
        super::drawing::DrawingObjectKind::ConnectionShape => connectors += 1,
        super::drawing::DrawingObjectKind::Picture => pictures += 1,
        super::drawing::DrawingObjectKind::ContentPart => content_parts += 1,
        super::drawing::DrawingObjectKind::Unknown => unknown += 1,
      }
      flags += usize::from(anchor.edit_as.is_some())
        + usize::from(anchor.lock_with_sheet)
        + usize::from(anchor.print_with_sheet)
        + usize::from(anchor.object.hidden)
        + usize::from(anchor.object.has_style);
      ref_len += anchor.object.id.unwrap_or_default() as usize
        + anchor.object.name.as_ref().map_or(0, |value| value.len())
        + anchor
          .object
          .description
          .as_ref()
          .map_or(0, |value| value.len())
        + anchor
          .object
          .macro_name
          .as_ref()
          .map_or(0, |value| value.len())
        + anchor
          .object
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
        + anchor
          .object
          .graphic_uri
          .as_ref()
          .map_or(0, |value| value.len())
        + anchor.object.text_len
        + anchor.object.child_objects
        + drawing_marker_len(anchor.from.as_ref())
        + drawing_marker_len(anchor.to.as_ref())
        + anchor.position.map_or(0, |(x, y)| {
          x.unsigned_abs() as usize + y.unsigned_abs() as usize
        })
        + anchor.extent.map_or(0, |(cx, cy)| {
          cx.unsigned_abs() as usize + cy.unsigned_abs() as usize
        });
    }
    lines.push(format!(
      "drawing anchors twoCell={} oneCell={} absolute={} pictures={} graphicFrames={} shapes={} groups={} connectors={} contentParts={} unknown={} flags={} refLen={}",
      two_cell,
      one_cell,
      absolute,
      pictures,
      graphic_frames,
      shapes,
      groups,
      connectors,
      content_parts,
      unknown,
      flags,
      ref_len
    ));
  }

  if chart_count > 0 {
    let charts = sheet
      .resources
      .drawings
      .iter()
      .flat_map(|drawing| drawing.charts.iter().chain(drawing.extended_charts.iter()));
    let mut extended = 0usize;
    let mut chart_types = 0usize;
    let mut axes = 0usize;
    let mut flags = 0usize;
    let mut relationships = 0usize;
    let mut text_len = 0usize;
    let mut data_sets = 0usize;
    let mut series = 0usize;
    for chart in charts {
      extended += usize::from(chart.extended);
      chart_types += chart.chart_type_groups;
      axes += chart.axes;
      flags += usize::from(chart.has_fallback_image)
        + usize::from(chart.date1904)
        + usize::from(chart.rounded_corners)
        + usize::from(chart.has_style)
        + usize::from(chart.has_pivot_source)
        + chart.protection_flags
        + usize::from(chart.has_title)
        + usize::from(chart.has_3d_view)
        + usize::from(chart.has_legend)
        + chart.chart_flags
        + usize::from(chart.has_root_shape_properties)
        + usize::from(chart.has_text_properties)
        + usize::from(chart.external_data_auto_update)
        + usize::from(chart.has_print_settings)
        + usize::from(chart.has_user_shapes_reference)
        + chart.extension_markers;
      relationships += usize::from(chart.relationship_id.is_some())
        + usize::from(chart.external_data_relationship_id.is_some())
        + usize::from(chart.has_chart_drawing)
        + usize::from(chart.has_embedded_package)
        + chart.images
        + usize::from(chart.has_theme_override)
        + chart.styles
        + chart.color_styles;
      text_len += chart
        .relationship_id
        .as_ref()
        .map_or(0, |value| value.len())
        + chart.version_len
        + chart.feature_list_len
        + chart
          .external_data_relationship_id
          .as_ref()
          .map_or(0, |value| value.len());
      data_sets += chart.chartex_data_sets;
      series += chart.chartex_series;
    }
    lines.push(format!(
      "charts total={} extended={} typeGroups={} axes={} flags={} relationships={} textLen={} chartexData={} chartexSeries={}",
      chart_count,
      extended,
      chart_types,
      axes,
      flags,
      relationships,
      text_len,
      data_sets,
      series
    ));
  }

  if diagram_count > 0 {
    let diagrams = sheet
      .resources
      .drawings
      .iter()
      .map(|drawing| &drawing.diagrams);
    let mut data_parts = 0usize;
    let mut layout_parts = 0usize;
    let mut style_parts = 0usize;
    let mut color_parts = 0usize;
    let mut drawing_parts = 0usize;
    let mut points = 0usize;
    let mut connections = 0usize;
    let mut layout_nodes = 0usize;
    let mut algorithms = 0usize;
    let mut persisted_shapes = 0usize;
    let mut flags = 0usize;
    let mut relationships = 0usize;
    let mut images = 0usize;
    let mut text_len = 0usize;
    for diagram in diagrams {
      data_parts += diagram.data_parts.len();
      layout_parts += diagram.layout_parts.len();
      style_parts += diagram.style_parts.len();
      color_parts += diagram.color_parts.len();
      drawing_parts += diagram.drawing_parts.len();
      for data in &diagram.data_parts {
        points += data.points + data.unknown_points;
        connections += data.connections;
        flags += data.text_points
          + data.property_sets
          + data.shape_properties
          + usize::from(data.background)
          + usize::from(data.whole)
          + data.extension_markers;
        relationships +=
          usize::from(data.relationship_id.is_some()) + data.slides + data.worksheets;
        images += data.images;
        text_len += data.relationship_id.as_ref().map_or(0, |value| value.len()) + data.text_len;
      }
      for layout in &diagram.layout_parts {
        layout_nodes += layout.layout_nodes;
        algorithms += layout.algorithms;
        flags += layout.titles
          + layout.descriptions
          + usize::from(layout.has_category_list)
          + usize::from(layout.has_sample_data)
          + usize::from(layout.has_style_data)
          + usize::from(layout.has_color_data)
          + layout.shapes
          + layout.presentation_of
          + layout.constraints
          + layout.rules
          + layout.variables
          + layout.for_each
          + layout.choose
          + layout.extension_markers;
        relationships += usize::from(layout.relationship_id.is_some());
        images += layout.images;
        text_len += layout
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + layout.text_len;
      }
      for style in &diagram.style_parts {
        flags += style.titles
          + style.descriptions
          + usize::from(style.has_categories)
          + usize::from(style.has_scene3d)
          + style.labels
          + style.extension_markers;
        relationships += usize::from(style.relationship_id.is_some());
        text_len += style
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + style.text_len;
      }
      for color in &diagram.color_parts {
        flags += color.titles
          + color.descriptions
          + usize::from(color.has_categories)
          + color.labels
          + color.extension_markers;
        relationships += usize::from(color.relationship_id.is_some());
        text_len += color
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + color.text_len;
      }
      for drawing in &diagram.drawing_parts {
        persisted_shapes += drawing.shapes + drawing.groups;
        flags += drawing.text_shapes
          + drawing.styled_shapes
          + drawing.transformed_shapes
          + drawing.extension_markers;
        relationships += usize::from(drawing.relationship_id.is_some());
        images += drawing.images;
        text_len += drawing
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + drawing.text_len;
      }
    }
    lines.push(format!(
      "diagrams data={} layouts={} styles={} colors={} drawings={} points={} connections={} layoutNodes={} algorithms={} persistedShapes={} flags={} relationships={} images={} textLen={}",
      data_parts,
      layout_parts,
      style_parts,
      color_parts,
      drawing_parts,
      points,
      connections,
      layout_nodes,
      algorithms,
      persisted_shapes,
      flags,
      relationships,
      images,
      text_len
    ));
  }

  if sheet_relationship_count(sheet) > 0 {
    let relationships = &sheet.resources.relationships;
    let single_xml_cells = relationships
      .single_xml_cells
      .iter()
      .map(|part| part.cells)
      .sum::<usize>();
    let single_xml_len = relationships
      .single_xml_cells
      .iter()
      .map(|part| {
        part.relationship_id.as_ref().map_or(0, |value| value.len())
          + part.ref_text_len
          + part.unique_name_len
          + part.xpath_len
          + part.id_sum
          + part.extension_cells
      })
      .sum::<usize>();
    let named_views = relationships
      .named_sheet_views
      .iter()
      .map(|part| part.views)
      .sum::<usize>();
    let named_view_filters = relationships
      .named_sheet_views
      .iter()
      .map(|part| part.filters + part.column_filters + part.sort_rules)
      .sum::<usize>();
    let named_view_len = relationships
      .named_sheet_views
      .iter()
      .map(|part| {
        part.relationship_id.as_ref().map_or(0, |value| value.len())
          + part.text_len
          + part.extensions
      })
      .sum::<usize>();
    let slicers = relationships
      .slicers
      .iter()
      .map(|part| part.slicers)
      .sum::<usize>();
    let slicer_len = relationships
      .slicers
      .iter()
      .map(|part| {
        part.relationship_id.as_ref().map_or(0, |value| value.len())
          + part.text_len
          + part.flags
          + part.extensions
      })
      .sum::<usize>();
    let timelines = relationships
      .timelines
      .iter()
      .map(|part| part.timelines)
      .sum::<usize>();
    let timeline_len = relationships
      .timelines
      .iter()
      .map(|part| {
        part.relationship_id.as_ref().map_or(0, |value| value.len())
          + part.text_len
          + part.flags
          + part.extensions
      })
      .sum::<usize>();
    let sort_map_items = relationships
      .sort_map
      .as_ref()
      .map_or(0, |sort_map| sort_map.row_items + sort_map.column_items);
    let sort_map_len = relationships.sort_map.as_ref().map_or(0, |sort_map| {
      sort_map
        .relationship_id
        .as_ref()
        .map_or(0, |value| value.len())
        + sort_map.ref_text_len
        + sort_map.declared_count
    });
    lines.push(format!(
      "sheet relationships singleXmlParts={} singleXmlCells={} singleXmlLen={} namedViewParts={} namedViews={} namedViewFilters={} namedViewLen={} slicerParts={} slicers={} slicerLen={} timelineParts={} timelines={} timelineLen={} sortMapItems={} sortMapLen={} customProps={} printerSettings={} slicerRels={} timelineRels={} model3dRels={} activeXBinaryRels={}",
      relationships.single_xml_cells.len(),
      single_xml_cells,
      single_xml_len,
      relationships.named_sheet_views.len(),
      named_views,
      named_view_filters,
      named_view_len,
      relationships.slicers.len(),
      slicers,
      slicer_len,
      relationships.timelines.len(),
      timelines,
      timeline_len,
      sort_map_items,
      sort_map_len,
      relationships.custom_properties,
      relationships.printer_settings,
      relationships.slicer_relationships,
      relationships.timeline_relationships,
      relationships.model3d_relationships,
      relationships.active_x_binary_relationships
    ));
  }

  if !sheet.metrics.objects.ole_objects.is_empty() || !sheet.metrics.objects.controls.is_empty() {
    let ole_flags = sheet
      .metrics
      .objects
      .ole_objects
      .iter()
      .map(|object| {
        object.property_flags
          + usize::from(object.auto_load)
          + usize::from(object.show_as_icon)
          + usize::from(object.has_embedded_properties)
          + usize::from(object.link.is_some())
          + usize::from(object.relationship_id.is_some())
      })
      .sum::<usize>();
    let ole_ref_len = sheet
      .metrics
      .objects
      .ole_objects
      .iter()
      .map(|object| {
        object.shape_id as usize
          + object
            .relationship_id
            .as_ref()
            .map_or(0, |value| value.len())
          + object.prog_id.as_ref().map_or(0, |value| value.len())
          + object.link.as_ref().map_or(0, |value| value.len())
          + object.property_text_len
          + usize::from(object.data_or_view_aspect.is_some())
          + usize::from(object.ole_update.is_some())
      })
      .sum::<usize>();
    let control_flags = sheet
      .metrics
      .objects
      .controls
      .iter()
      .map(|control| control.property_flags + usize::from(control.has_control_properties))
      .sum::<usize>();
    let control_ref_len = sheet
      .metrics
      .objects
      .controls
      .iter()
      .map(|control| {
        control.shape_id as usize
          + control.relationship_id.len()
          + control.name.as_ref().map_or(0, |value| value.len())
          + control.property_text_len
      })
      .sum::<usize>();
    let anchor_count = sheet
      .metrics
      .objects
      .ole_objects
      .iter()
      .filter(|object| object.anchor.is_some())
      .count()
      + sheet
        .metrics
        .objects
        .controls
        .iter()
        .filter(|control| control.anchor.is_some())
        .count();
    lines.push(format!(
      "objects ole={} oleFlags={} oleRefLen={} controls={} controlFlags={} controlRefLen={} anchors={}",
      sheet.metrics.objects.ole_objects.len(),
      ole_flags,
      ole_ref_len,
      sheet.metrics.objects.controls.len(),
      control_flags,
      control_ref_len,
      anchor_count
    ));
  }

  if !sheet.resources.object_resources.vml_drawings.is_empty()
    || !sheet.resources.object_resources.controls.is_empty()
    || !sheet
      .resources
      .object_resources
      .control_properties
      .is_empty()
    || !sheet.resources.object_resources.embedded_objects.is_empty()
    || !sheet
      .resources
      .object_resources
      .embedded_packages
      .is_empty()
  {
    let resources = &sheet.resources.object_resources;
    let vml_images = resources
      .vml_drawings
      .iter()
      .map(|part| part.images)
      .sum::<usize>();
    let vml_legacy_diagram_texts = resources
      .vml_drawings
      .iter()
      .map(|part| part.legacy_diagram_texts)
      .sum::<usize>();
    let control_binaries = resources
      .controls
      .iter()
      .map(|part| part.binary_data_parts.len())
      .sum::<usize>();
    let control_property_flags = resources
      .control_properties
      .iter()
      .map(|part| {
        usize::from(part.has_object_type)
          + usize::from(part.has_checked)
          + part.boolean_flags
          + part.numeric_fields
          + part.formula_fields
          + part.alignment_fields
          + part.list_items
          + usize::from(part.has_extension_list)
      })
      .sum::<usize>();
    let relationship_count = resources
      .vml_drawings
      .iter()
      .filter(|part| part.relationship_id.is_some())
      .count()
      + resources
        .controls
        .iter()
        .filter(|part| part.relationship_id.is_some())
        .count()
      + resources
        .control_properties
        .iter()
        .filter(|part| part.relationship_id.is_some())
        .count()
      + resources
        .embedded_objects
        .iter()
        .filter(|part| part.relationship_id.is_some())
        .count()
      + resources
        .embedded_packages
        .iter()
        .filter(|part| part.relationship_id.is_some())
        .count();
    let text_len = resources
      .vml_drawings
      .iter()
      .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()))
      .sum::<usize>()
      + resources
        .controls
        .iter()
        .map(|part| {
          part.relationship_id.as_ref().map_or(0, |value| value.len())
            + part
              .binary_data_parts
              .iter()
              .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()))
              .sum::<usize>()
        })
        .sum::<usize>()
      + resources
        .control_properties
        .iter()
        .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()) + part.text_len)
        .sum::<usize>()
      + resources
        .embedded_objects
        .iter()
        .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()))
        .sum::<usize>()
      + resources
        .embedded_packages
        .iter()
        .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()))
        .sum::<usize>();
    lines.push(format!(
      "objectResources vml={} vmlImages={} legacyDiagramTexts={} controls={} controlBinaries={} controlProps={} controlPropFlags={} embeddedObjects={} embeddedPackages={} relationships={} textLen={}",
      resources.vml_drawings.len(),
      vml_images,
      vml_legacy_diagram_texts,
      resources.controls.len(),
      control_binaries,
      resources.control_properties.len(),
      control_property_flags,
      resources.embedded_objects.len(),
      resources.embedded_packages.len(),
      relationship_count,
      text_len
    ));
  }

  if !sheet.resources.query_tables.query_tables.is_empty() {
    let refresh_fields = sheet
      .resources
      .query_tables
      .query_tables
      .iter()
      .map(|query| query.refresh_fields)
      .sum::<usize>();
    let deleted_fields = sheet
      .resources
      .query_tables
      .query_tables
      .iter()
      .map(|query| query.deleted_fields)
      .sum::<usize>();
    let query_flags = sheet
      .resources
      .query_tables
      .query_tables
      .iter()
      .map(|query| {
        query.flag_count
          + usize::from(query.has_sort_state)
          + usize::from(query.has_refresh_extensions)
          + usize::from(query.has_extensions)
      })
      .sum::<usize>();
    let query_ref_len = sheet
      .resources
      .query_tables
      .query_tables
      .iter()
      .map(|query| {
        query
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + query.name.len()
          + query.connection_id as usize
      })
      .sum::<usize>();
    lines.push(format!(
      "queryTables count={} refreshFields={} deletedFields={} flags={} refLen={}",
      sheet.resources.query_tables.query_tables.len(),
      refresh_fields,
      deleted_fields,
      query_flags,
      query_ref_len
    ));
  }

  if !sheet.resources.pivot_tables.tables.is_empty() {
    let pivot_fields = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| pivot.pivot_fields)
      .sum::<usize>();
    let axis_fields = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| pivot.row_fields + pivot.column_fields + pivot.page_fields + pivot.data_fields)
      .sum::<usize>();
    let filters = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| pivot.filters)
      .sum::<usize>();
    let formats = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| pivot.formats)
      .sum::<usize>();
    let pivot_flags = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| {
        pivot.flag_count
          + usize::from(pivot.style_name.is_some())
          + usize::from(pivot.has_cache_definition_part)
          + usize::from(pivot.has_extensions)
      })
      .sum::<usize>();
    let pivot_ref_len = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| {
        pivot
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + pivot.name.len()
          + pivot.location_reference.len()
          + pivot.cache_id as usize
      })
      .sum::<usize>();
    lines.push(format!(
      "pivots fields={} axisFields={} filters={} formats={} flags={} refLen={}",
      pivot_fields, axis_fields, filters, formats, pivot_flags, pivot_ref_len
    ));
  }

  if sheet.resources.comments.legacy_count() > 0 || sheet.resources.comments.threaded_count() > 0 {
    let legacy_authors = sheet
      .resources
      .comments
      .legacy
      .as_ref()
      .map_or(0, |legacy| legacy.authors.len());
    let legacy_text_len = sheet
      .resources
      .comments
      .legacy
      .as_ref()
      .map_or(0, |legacy| {
        legacy
          .comments
          .iter()
          .map(|comment| {
            comment.reference.len()
              + comment.author.as_ref().map_or(0, |author| author.len())
              + comment.guid.as_ref().map_or(0, |guid| guid.len())
              + comment.text.len()
              + comment.shape_id.unwrap_or_default() as usize
              + comment.author_id as usize
              + comment.rich_runs
              + comment.phonetic_runs
              + usize::from(comment.has_comment_properties)
          })
          .sum()
      });
    let legacy_extensions = sheet
      .resources
      .comments
      .legacy
      .as_ref()
      .is_some_and(|legacy| legacy.has_extensions);
    let legacy_rel = sheet
      .resources
      .comments
      .legacy
      .as_ref()
      .and_then(|legacy| legacy.relationship_id.as_deref())
      .unwrap_or("");
    let threaded_roots = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .filter(|comment| comment.parent_id.is_none())
      .count();
    let threaded_replies = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .filter(|comment| comment.parent_id.is_some())
      .count();
    let threaded_done = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .filter(|comment| comment.done)
      .count();
    let threaded_mentions = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .map(|comment| comment.mentions)
      .sum::<usize>();
    let threaded_text_len = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .map(|comment| {
        comment.reference.as_ref().map_or(0, |value| value.len())
          + comment.id.len()
          + comment.parent_id.as_ref().map_or(0, |value| value.len())
          + comment.person_id.len()
          + comment.date_time.as_ref().map_or(0, |value| value.len())
          + comment.text.as_ref().map_or(0, |value| value.len())
          + usize::from(comment.has_extensions)
      })
      .sum::<usize>();
    let threaded_extensions = sheet
      .resources
      .comments
      .threaded
      .iter()
      .filter(|threaded| threaded.has_extensions)
      .count();
    let threaded_rel_len = sheet
      .resources
      .comments
      .threaded
      .iter()
      .map(|threaded| threaded.relationship_id.as_ref().map_or(0, |id| id.len()))
      .sum::<usize>();
    lines.push(format!(
      "comments legacyAuthors={} legacyRel={} legacyExt={} legacyTextLen={} threadedParts={} threadedRoots={} threadedReplies={} threadedDone={} threadedMentions={} threadedExt={} threadedRelLen={} threadedTextLen={}",
      legacy_authors,
      legacy_rel,
      legacy_extensions,
      legacy_text_len,
      sheet.resources.comments.threaded.len(),
      threaded_roots,
      threaded_replies,
      threaded_done,
      threaded_mentions,
      threaded_extensions,
      threaded_rel_len,
      threaded_text_len
    ));
  }

  if !sheet.resources.tables.is_empty() {
    let table_columns = sheet
      .resources
      .tables
      .iter()
      .map(|table| table.columns.len())
      .sum::<usize>();
    let auto_filters = sheet
      .resources
      .tables
      .iter()
      .filter(|table| table.has_auto_filter)
      .count();
    let sort_states = sheet
      .resources
      .tables
      .iter()
      .filter(|table| table.has_sort_state)
      .count();
    let styled_tables = sheet
      .resources
      .tables
      .iter()
      .filter(|table| table.style.name.is_some())
      .count();
    let query_tables = sheet
      .resources
      .tables
      .iter()
      .map(|table| table.query_tables)
      .sum::<usize>();
    let formula_columns = sheet
      .resources
      .tables
      .iter()
      .flat_map(|table| &table.columns)
      .filter(|column| column.has_calculated_formula || column.has_totals_formula)
      .count();
    let xml_columns = sheet
      .resources
      .tables
      .iter()
      .flat_map(|table| &table.columns)
      .filter(|column| column.has_xml_column_properties)
      .count();
    let table_name_len = sheet
      .resources
      .tables
      .iter()
      .map(|table| {
        table.display_name.len()
          + table.name.as_ref().map_or(0, |name| name.len())
          + table.reference.len()
          + table.relationship_id.as_ref().map_or(0, |id| id.len())
          + usize::from(table.table_type.is_some())
          + table.id as usize
          + table.header_rows as usize
          + table.totals_rows as usize
      })
      .sum::<usize>();
    let table_style_flags = sheet
      .resources
      .tables
      .iter()
      .filter(|table| {
        table.style.show_first_column
          || table.style.show_last_column
          || table.style.show_row_stripes
          || table.style.show_column_stripes
          || table.has_extensions
      })
      .count();
    let table_column_name_len = sheet
      .resources
      .tables
      .iter()
      .flat_map(|table| &table.columns)
      .map(|column| {
        column.name.len()
          + column.unique_name.as_ref().map_or(0, |name| name.len())
          + column
            .totals_row_label
            .as_ref()
            .map_or(0, |label| label.len())
          + usize::from(column.totals_row_function.is_some())
          + column.query_table_field_id.unwrap_or_default() as usize
          + column.id as usize
          + usize::from(column.has_extensions)
      })
      .sum::<usize>();
    lines.push(format!(
      "tables columns={} autoFilters={} sortStates={} styled={} styleFlags={} queryTables={} formulaColumns={} xmlColumns={} tableNameLen={} columnNameLen={}",
      table_columns,
      auto_filters,
      sort_states,
      styled_tables,
      table_style_flags,
      query_tables,
      formula_columns,
      xml_columns,
      table_name_len,
      table_column_name_len
    ));
  }

  if sheet.metrics.dimension.is_some()
    || sheet.metrics.settings.properties.code_name.is_some()
    || sheet.metrics.settings.auto_filter.is_some()
    || sheet.metrics.settings.sort_state.is_some()
    || sheet.metrics.settings.protection.sheet
    || !sheet.metrics.views.views.is_empty()
    || !sheet.metrics.columns.is_empty()
    || !sheet.metrics.merged_ranges.is_empty()
    || !sheet.metrics.hyperlinks.is_empty()
    || !sheet.metrics.row_breaks.is_empty()
    || !sheet.metrics.column_breaks.is_empty()
    || !sheet.metrics.conditions.conditional_formats.is_empty()
    || !sheet.metrics.conditions.data_validations.is_empty()
    || sheet.metrics.protected_ranges > 0
    || sheet.metrics.scenarios > 0
  {
    let hidden_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.hidden)
      .count();
    let styled_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.style_index.is_some())
      .count();
    let custom_width_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.custom_width)
      .count();
    let best_fit_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.best_fit)
      .count();
    let collapsed_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.collapsed)
      .count();
    let phonetic_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.phonetic)
      .count();
    let max_outline = sheet
      .metrics
      .columns
      .iter()
      .map(|column| column.outline_level)
      .max()
      .unwrap_or(0);
    let custom_width_sum = sheet
      .metrics
      .columns
      .iter()
      .filter_map(|column| column.width)
      .sum::<f64>();
    let column_span_count = sheet
      .metrics
      .columns
      .iter()
      .map(|column| column.last.saturating_sub(column.first) + 1)
      .sum::<u32>();
    let manual_breaks = sheet
      .metrics
      .row_breaks
      .iter()
      .chain(&sheet.metrics.column_breaks)
      .filter(|page_break| page_break.manual)
      .count();
    let pivot_breaks = sheet
      .metrics
      .row_breaks
      .iter()
      .chain(&sheet.metrics.column_breaks)
      .filter(|page_break| page_break.pivot)
      .count();
    let break_extent_sum = sheet
      .metrics
      .row_breaks
      .iter()
      .chain(&sheet.metrics.column_breaks)
      .map(|page_break| page_break.max.saturating_sub(page_break.min) + page_break.id)
      .sum::<u32>();
    let relationship_hyperlinks = sheet
      .metrics
      .hyperlinks
      .iter()
      .filter(|hyperlink| hyperlink.relationship_id.is_some())
      .count();
    let local_hyperlinks = sheet
      .metrics
      .hyperlinks
      .iter()
      .filter(|hyperlink| hyperlink.location.is_some())
      .count();
    let displayed_hyperlinks = sheet
      .metrics
      .hyperlinks
      .iter()
      .filter(|hyperlink| hyperlink.display.is_some() || hyperlink.tooltip.is_some())
      .count();
    let hyperlink_ref_len = sheet
      .metrics
      .hyperlinks
      .iter()
      .map(|hyperlink| hyperlink.reference.len())
      .sum::<usize>();
    let selected_views = sheet
      .metrics
      .views
      .views
      .iter()
      .filter(|view| view.tab_selected == Some(true))
      .count();
    let panes = sheet
      .metrics
      .views
      .views
      .iter()
      .filter(|view| view.pane.is_some())
      .count();
    let selections = sheet
      .metrics
      .views
      .views
      .iter()
      .map(|view| view.selections.len())
      .sum::<usize>();
    let pivot_selections = sheet
      .metrics
      .views
      .views
      .iter()
      .map(|view| view.pivot_selections)
      .sum::<usize>();
    let view_flags = sheet
      .metrics
      .views
      .views
      .iter()
      .filter(|view| {
        view.window_protection.is_some()
          || view.show_formulas.is_some()
          || view.show_grid_lines.is_some()
          || view.show_row_col_headers.is_some()
          || view.show_zeros.is_some()
          || view.right_to_left.is_some()
          || view.show_outline_symbols.is_some()
          || view.default_grid_color.is_some()
          || view.has_extensions
      })
      .count();
    let view_ref_len = sheet
      .metrics
      .views
      .views
      .iter()
      .map(|view| {
        view.top_left_cell.as_ref().map_or(0, |value| value.len())
          + view.color_id.unwrap_or_default() as usize
          + view.zoom_scale.unwrap_or_default() as usize
          + view.zoom_scale_normal.unwrap_or_default() as usize
          + view.zoom_scale_sheet_layout_view.unwrap_or_default() as usize
          + view.zoom_scale_page_layout_view.unwrap_or_default() as usize
          + view.workbook_view_id as usize
          + usize::from(view.view_type.is_some())
          + view.pane.as_ref().map_or(0, |pane| {
            pane.top_left_cell.as_ref().map_or(0, |value| value.len())
              + pane.horizontal_split.unwrap_or_default() as usize
              + pane.vertical_split.unwrap_or_default() as usize
              + usize::from(pane.active_pane.is_some())
              + usize::from(pane.state.is_some())
          })
          + view
            .selections
            .iter()
            .map(|selection| {
              selection
                .active_cell
                .as_ref()
                .map_or(0, |value| value.len())
                + selection.active_cell_id.unwrap_or_default() as usize
                + selection.sequence_of_references.len()
                + usize::from(selection.pane.is_some())
            })
            .sum::<usize>()
      })
      .sum::<usize>();
    let settings = &sheet.metrics.settings;
    let property_flags = usize::from(settings.properties.filter_mode)
      + usize::from(settings.properties.published.is_some())
      + usize::from(settings.properties.sync_horizontal.is_some())
      + usize::from(settings.properties.sync_vertical.is_some())
      + usize::from(settings.properties.sync_reference.is_some())
      + usize::from(settings.properties.transition_evaluation.is_some())
      + usize::from(settings.properties.transition_entry.is_some())
      + usize::from(
        settings
          .properties
          .enable_format_conditions_calculation
          .is_some(),
      )
      + usize::from(settings.properties.has_tab_color)
      + usize::from(settings.properties.outline.apply_styles)
      + usize::from(settings.properties.outline.summary_below.is_some())
      + usize::from(settings.properties.outline.summary_right.is_some())
      + usize::from(settings.properties.outline.show_outline_symbols.is_some())
      + usize::from(settings.properties.page_setup.auto_page_breaks.is_some())
      + usize::from(settings.properties.page_setup.fit_to_page);
    let protection_flags = usize::from(settings.protection.has_password)
      + usize::from(settings.protection.has_hash)
      + usize::from(settings.protection.algorithm_name.is_some())
      + usize::from(settings.protection.spin_count.is_some())
      + usize::from(settings.protection.sheet)
      + usize::from(settings.protection.objects)
      + usize::from(settings.protection.scenarios)
      + settings.protection.locked_options
      + usize::from(settings.protection.unlocked_selection)
      + usize::from(settings.protection.locked_selection);
    let auto_filter_columns = settings
      .auto_filter
      .as_ref()
      .map_or(0, |auto_filter| auto_filter.filter_columns);
    let auto_filter_flags = settings.auto_filter.as_ref().map_or(0, |auto_filter| {
      usize::from(auto_filter.reference.is_some())
        + usize::from(auto_filter.sort_state.is_some())
        + usize::from(auto_filter.has_extensions)
    });
    let sort_flags = settings
      .sort_state
      .as_ref()
      .map_or(0, sort_state_flag_count)
      + settings
        .auto_filter
        .as_ref()
        .and_then(|auto_filter| auto_filter.sort_state.as_ref())
        .map_or(0, sort_state_flag_count);
    lines.push(format!(
      "sheet metrics dimension={} settingsCode={} propertyFlags={} protectionFlags={} autoFilterColumns={} autoFilterFlags={} sortFlags={} views={} selectedViews={} panes={} selections={} pivotSelections={} viewFlags={} viewExt={} viewRefLen={} baseColWidth={} defaultColWidth={} defaultRowHeight={} customHeight={} zeroHeight={} thickTop={} thickBottom={} columns={} columnSpans={} hiddenColumns={} styledColumns={} customWidthColumns={} bestFitColumns={} collapsedColumns={} phoneticColumns={} maxOutline={} widthSum={} merges={} hyperlinks={} hyperlinkRels={} localHyperlinks={} displayedHyperlinks={} hyperlinkRefLen={} rowBreaks={} colBreaks={} manualBreaks={} pivotBreaks={} breakExtentSum={} condFmt={} validations={} protectedRanges={} scenarios={}",
      sheet.metrics.dimension.as_deref().unwrap_or(""),
      sheet
        .metrics
        .settings
        .properties
        .code_name
        .as_deref()
        .unwrap_or(""),
      property_flags,
      protection_flags,
      auto_filter_columns,
      auto_filter_flags,
      sort_flags,
      sheet.metrics.views.views.len(),
      selected_views,
      panes,
      selections,
      pivot_selections,
      view_flags,
      sheet.metrics.views.has_extensions,
      view_ref_len,
      sheet
        .metrics
        .format
        .base_column_width
        .map_or(String::new(), |value| value.to_string()),
      sheet
        .metrics
        .format
        .default_column_width
        .map_or(String::new(), |value| value.to_string()),
      sheet.metrics.format.default_row_height,
      sheet.metrics.format.custom_height,
      sheet.metrics.format.zero_height,
      sheet.metrics.format.thick_top,
      sheet.metrics.format.thick_bottom,
      sheet.metrics.columns.len(),
      column_span_count,
      hidden_columns,
      styled_columns,
      custom_width_columns,
      best_fit_columns,
      collapsed_columns,
      phonetic_columns,
      max_outline,
      custom_width_sum,
      sheet.metrics.merged_ranges.len(),
      sheet.metrics.hyperlinks.len(),
      relationship_hyperlinks,
      local_hyperlinks,
      displayed_hyperlinks,
      hyperlink_ref_len,
      sheet.metrics.row_breaks.len(),
      sheet.metrics.column_breaks.len(),
      manual_breaks,
      pivot_breaks,
      break_extent_sum,
      sheet.metrics.conditions.conditional_formats.len(),
      sheet.metrics.conditions.data_validations.len(),
      sheet.metrics.protected_ranges,
      sheet.metrics.scenarios
    ));
  }

  if !sheet.metrics.conditions.conditional_formats.is_empty()
    || !sheet.metrics.conditions.data_validations.is_empty()
    || !sheet
      .metrics
      .conditions
      .extension_conditions
      .conditional_formats
      .is_empty()
    || !sheet
      .metrics
      .conditions
      .extension_conditions
      .data_validations
      .is_empty()
    || !sheet
      .metrics
      .conditions
      .extension_conditions
      .sparkline_groups
      .is_empty()
    || !sheet
      .metrics
      .conditions
      .extension_conditions
      .ignored_errors
      .is_empty()
  {
    let cf_rules = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .map(|format| format.rules.len())
      .sum::<usize>();
    let cf_formulas = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .flat_map(|conditional_format| &conditional_format.rules)
      .map(|rule| rule.formulas.len())
      .sum::<usize>();
    let cf_extensions = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .filter(|format| format.has_extensions)
      .count()
      + sheet
        .metrics
        .conditions
        .conditional_formats
        .iter()
        .flat_map(|format| &format.rules)
        .filter(|rule| rule.has_extensions)
        .count();
    let cf_visual_rules = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .flat_map(|conditional_format| &conditional_format.rules)
      .filter(|rule| rule.has_color_scale || rule.has_data_bar || rule.has_icon_set)
      .count();
    let cf_pivot = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .filter(|format| format.pivot)
      .count();
    let cf_ref_count = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .map(|conditional_format| conditional_format.sequence_of_references.len())
      .sum::<usize>();
    let cf_priority_sum = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .flat_map(|conditional_format| &conditional_format.rules)
      .map(|rule| {
        rule.priority as i64
          + rule.format_id.unwrap_or_default() as i64
          + rule.rank.unwrap_or_default() as i64
          + rule.std_dev.unwrap_or_default() as i64
          + i64::from(rule.stop_if_true)
          + i64::from(rule.operator.is_some())
          + i64::from(rule.time_period.is_some())
          + i64::from(rule.text.is_some())
          + format!("{:?}", rule.rule_type).len() as i64
      })
      .sum::<i64>();
    let validation_formulas = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .filter(|validation| validation.formula1.is_some() || validation.formula2.is_some())
      .count();
    let validation_messages = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .filter(|validation| {
        validation.error_title.is_some()
          || validation.error.is_some()
          || validation.prompt_title.is_some()
          || validation.prompt.is_some()
      })
      .count();
    let validation_lists = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .filter(|validation| validation.list.is_some())
      .count();
    let validation_flags = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .filter(|validation| {
        validation.allow_blank
          || validation.no_drop_down
          || validation.show_input_message
          || validation.show_error_message
          || validation.error_style.is_some()
          || validation.ime_mode.is_some()
          || validation.operator.is_some()
          || validation.validation_type.is_some()
      })
      .count();
    let validation_ref_count = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .map(|validation| validation.sequence_of_references.len())
      .sum::<usize>();
    lines.push(format!(
      "conditions cf={} cfRules={} cfFormulas={} cfVisual={} cfPivot={} cfRefs={} cfExt={} cfPrioritySum={} validations={} validationRefs={} validationFormulas={} validationLists={} validationMessages={} validationFlags={} disablePrompts={} validationWindow={}",
      sheet.metrics.conditions.conditional_formats.len(),
      cf_rules,
      cf_formulas,
      cf_visual_rules,
      cf_pivot,
      cf_ref_count,
      cf_extensions,
      cf_priority_sum,
      sheet.metrics.conditions.data_validations.len(),
      validation_ref_count,
      validation_formulas,
      validation_lists,
      validation_messages,
      validation_flags,
      sheet.metrics.conditions.validations_disable_prompts,
      sheet
        .metrics
        .conditions
        .validation_window
        .map_or(String::new(), |(x, y)| format!("{x},{y}"))
    ));

    let ext = &sheet.metrics.conditions.extension_conditions;
    if !ext.conditional_formats.is_empty()
      || !ext.data_validations.is_empty()
      || !ext.sparkline_groups.is_empty()
      || !ext.ignored_errors.is_empty()
      || ext.slicer_refs > 0
      || ext.protected_ranges > 0
      || ext.web_extensions > 0
      || ext.timeline_refs > 0
      || ext.unknown_extensions > 0
    {
      let ext_cf_rules = ext
        .conditional_formats
        .iter()
        .map(|format| format.rules.len())
        .sum::<usize>();
      let ext_cf_formulas = ext
        .conditional_formats
        .iter()
        .flat_map(|format| &format.rules)
        .map(|rule| rule.formulas.len())
        .sum::<usize>();
      let ext_cf_visual = ext
        .conditional_formats
        .iter()
        .flat_map(|format| &format.rules)
        .filter(|rule| rule.has_color_scale || rule.has_data_bar || rule.has_icon_set)
        .count();
      let ext_cf_flags = ext
        .conditional_formats
        .iter()
        .map(|format| {
          usize::from(format.pivot)
            + format.sequence_of_references.len()
            + usize::from(format.has_extensions)
            + format
              .rules
              .iter()
              .map(|rule| {
                usize::from(rule.rule_type.is_some())
                  + usize::from(rule.priority.is_some())
                  + usize::from(rule.stop_if_true)
                  + rule.boolean_flags
                  + usize::from(rule.operator.is_some())
                  + usize::from(rule.text.is_some())
                  + usize::from(rule.time_period.is_some())
                  + usize::from(rule.rank.is_some())
                  + usize::from(rule.std_dev.is_some())
                  + usize::from(rule.id.is_some())
                  + usize::from(rule.has_differential_format)
                  + usize::from(rule.has_extensions)
              })
              .sum::<usize>()
        })
        .sum::<usize>();
      let ext_validation_flags = ext
        .data_validations
        .iter()
        .map(|validation| {
          usize::from(validation.validation_type.is_some())
            + usize::from(validation.error_style.is_some())
            + usize::from(validation.ime_mode.is_some())
            + usize::from(validation.operator.is_some())
            + usize::from(validation.allow_blank)
            + usize::from(validation.no_drop_down)
            + usize::from(validation.show_input_message)
            + usize::from(validation.show_error_message)
            + usize::from(validation.error_title.is_some())
            + usize::from(validation.error.is_some())
            + usize::from(validation.prompt_title.is_some())
            + usize::from(validation.prompt.is_some())
            + usize::from(validation.formula1.is_some())
            + usize::from(validation.formula2.is_some())
            + validation.sequence_of_references.len()
        })
        .sum::<usize>();
      let sparklines = ext
        .sparkline_groups
        .iter()
        .map(|group| group.sparklines)
        .sum::<usize>();
      let sparkline_flags = ext
        .sparkline_groups
        .iter()
        .map(|group| {
          usize::from(group.formula.is_some())
            + group.color_count
            + group.flag_count
            + group.sparkline_formula_text_len
            + group.reference_text_len
        })
        .sum::<usize>();
      let ignored_errors = ext
        .ignored_errors
        .iter()
        .map(|errors| errors.ignored_errors)
        .sum::<usize>();
      let ignored_error_flags = ext
        .ignored_errors
        .iter()
        .map(|errors| {
          errors.flag_count + errors.reference_text_len + usize::from(errors.has_extensions)
        })
        .sum::<usize>();
      lines.push(format!(
        "conditionExt cf={} cfRules={} cfFormulas={} cfVisual={} cfFlags={} validations={} validationFlags={} sparklineGroups={} sparklines={} sparklineFlags={} ignoredErrors={} ignoredErrorFlags={} slicerRefs={} protectedRanges={} webExtensions={} timelineRefs={} unknown={} uriLen={}",
        ext.conditional_formats.len(),
        ext_cf_rules,
        ext_cf_formulas,
        ext_cf_visual,
        ext_cf_flags,
        ext.data_validations.len(),
        ext_validation_flags,
        ext.sparkline_groups.len(),
        sparklines,
        sparkline_flags,
        ignored_errors,
        ignored_error_flags,
        ext.slicer_refs,
        ext.protected_ranges,
        ext.web_extensions,
        ext.timeline_refs,
        ext.unknown_extensions,
        ext.uri_text_len
      ));
    }
  }

  let formula_cells = sheet
    .rows
    .iter()
    .flat_map(|row| &row.cells)
    .filter_map(|cell| cell.formula.as_ref())
    .collect::<Vec<_>>();
  if !formula_cells.is_empty() {
    let shared = formula_cells
      .iter()
      .filter(|formula| formula.formula_type == x::CellFormulaValues::Shared)
      .count();
    let arrays = formula_cells
      .iter()
      .filter(|formula| formula.formula_type == x::CellFormulaValues::Array)
      .count();
    let data_tables = formula_cells
      .iter()
      .filter(|formula| formula.formula_type == x::CellFormulaValues::DataTable)
      .count();
    let normal = formula_cells
      .iter()
      .filter(|formula| formula.formula_type == x::CellFormulaValues::Normal)
      .count();
    let shared_ids = formula_cells
      .iter()
      .filter(|formula| formula.shared_index.is_some())
      .count();
    let references = formula_cells
      .iter()
      .filter(|formula| formula.reference.is_some())
      .count();
    let formula_flags = formula_cells
      .iter()
      .filter(|formula| {
        formula.always_calculate_array
          || formula.calculate_cell
          || formula.data_table_2d
          || formula.data_table_row
          || formula.input1_deleted
          || formula.input2_deleted
          || formula.assigns_value_to_name
      })
      .count();
    let formula_text_len = formula_cells
      .iter()
      .map(|formula| {
        formula.text.len()
          + formula
            .input1_reference
            .as_ref()
            .map_or(0, |value| value.len())
          + formula
            .input2_reference
            .as_ref()
            .map_or(0, |value| value.len())
      })
      .sum::<usize>();
    lines.push(format!(
      "formulas total={} normal={} shared={} arrays={} dataTables={} sharedIds={} refs={} flags={} textLen={}",
      formula_cells.len(),
      normal,
      shared,
      arrays,
      data_tables,
      shared_ids,
      references,
      formula_flags,
      formula_text_len
    ));
  }

  for row in &sheet.rows {
    let values = row
      .cells
      .iter()
      .map(|cell| {
        let text = if cell.formula.is_some() {
          cell
            .cached_value
            .clone()
            .unwrap_or_else(|| cell.display_text.clone())
        } else {
          cell.display_text.clone()
        };
        let mut parts = Vec::new();
        if let Some(reference) = &cell.reference {
          parts.push(reference.clone());
        }
        if let Some(style_index) = cell.style_index {
          parts.push(format!("s={style_index}"));
        }
        if let Some(data_type) = cell.data_type {
          parts.push(format!("t={data_type:?}"));
        }
        if let Some(cell_meta_index) = cell.cell_meta_index {
          parts.push(format!("cm={cell_meta_index}"));
        }
        if let Some(value_meta_index) = cell.value_meta_index {
          parts.push(format!("vm={value_meta_index}"));
        }
        if cell.show_phonetic {
          parts.push("ph".to_string());
        }
        if cell.has_extensions {
          parts.push("ext".to_string());
        }
        if parts.is_empty() || text.is_empty() {
          text
        } else {
          format!("{}={text}", parts.join("/"))
        }
      })
      .collect::<Vec<_>>();
    if values.iter().any(|value| !value.is_empty()) {
      let prefix = match (row.row_index, row.hidden) {
        (Some(index), true) => format!("row {index} hidden: "),
        (Some(index), false) => format!("row {index}: "),
        (None, true) => "row hidden: ".to_string(),
        (None, false) => String::new(),
      };
      lines.push(format!("{prefix}{}", values.join("    ")));
    }
  }

  if lines.len() == 1 && !import.shared_strings.is_empty() {
    lines.push(format!("sharedStrings={}", import.shared_strings.len()));
  }

  lines
}

fn sort_state_flag_count(sort_state: &super::sheet_settings::SortStateModel) -> usize {
  sort_state.reference.len()
    + usize::from(sort_state.column_sort)
    + usize::from(sort_state.case_sensitive)
    + usize::from(sort_state.sort_method.is_some())
    + usize::from(sort_state.has_sort_condition)
    + usize::from(sort_state.has_extensions)
}

fn sheet_relationship_count(sheet: &CalcSheet) -> usize {
  let relationships = &sheet.resources.relationships;
  relationships.single_xml_cells.len()
    + relationships.named_sheet_views.len()
    + relationships.slicers.len()
    + relationships.timelines.len()
    + usize::from(relationships.sort_map.is_some())
    + relationships.custom_properties
    + relationships.printer_settings
    + relationships.slicer_relationships
    + relationships.timeline_relationships
    + relationships.model3d_relationships
    + relationships.active_x_binary_relationships
}

fn drawing_marker_len(marker: Option<&super::drawing::DrawingMarkerModel>) -> usize {
  marker.map_or(0, |marker| {
    marker.column.unsigned_abs() as usize
      + marker.row.unsigned_abs() as usize
      + marker.column_offset_emu.unsigned_abs() as usize
      + marker.row_offset_emu.unsigned_abs() as usize
  })
}
