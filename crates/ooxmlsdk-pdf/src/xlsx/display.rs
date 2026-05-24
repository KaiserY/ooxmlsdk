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

fn print_page_lines(import: &ExcelImport, page: &CalcPrintPage<'_>) -> Vec<String> {
  let mut lines = sheet_lines(import, page.sheet);
  lines.insert(
    1,
    format!(
      "print page={} sheetPage={} zoom={} paper={} scale={} fit={}x{} dpi={}x{} margins={} grid={} headings={}",
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
      page.page_settings.print_headings
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
  let chart_child_resource_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.chart_child_resource_count())
    .sum::<usize>();

  if drawing_count > 0
    || sheet.resources.vml_drawings > 0
    || sheet.resources.comments > 0
    || sheet.resources.threaded_comments > 0
    || !sheet.resources.tables.is_empty()
    || sheet.resources.pivot_tables > 0
    || sheet.resources.query_tables > 0
  {
    lines.push(format!(
      "resources drawings={} charts={} chartResources={} diagrams={} drawingImages={} vml={} comments={} threadedComments={} tables={} pivots={} queries={}",
      drawing_count,
      chart_count,
      chart_child_resource_count,
      diagram_count,
      drawing_image_count,
      sheet.resources.vml_drawings,
      sheet.resources.comments,
      sheet.resources.threaded_comments,
      sheet.resources.tables.len(),
      sheet.resources.pivot_tables,
      sheet.resources.query_tables
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
    || !sheet.metrics.columns.is_empty()
    || !sheet.metrics.merged_ranges.is_empty()
    || !sheet.metrics.hyperlinks.is_empty()
    || !sheet.metrics.row_breaks.is_empty()
    || !sheet.metrics.column_breaks.is_empty()
    || sheet.metrics.conditional_formats > 0
    || sheet.metrics.data_validations > 0
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
    lines.push(format!(
      "sheet metrics dimension={} baseColWidth={} defaultColWidth={} defaultRowHeight={} customHeight={} zeroHeight={} thickTop={} thickBottom={} columns={} columnSpans={} hiddenColumns={} styledColumns={} customWidthColumns={} bestFitColumns={} collapsedColumns={} phoneticColumns={} maxOutline={} widthSum={} merges={} hyperlinks={} hyperlinkRels={} localHyperlinks={} displayedHyperlinks={} hyperlinkRefLen={} rowBreaks={} colBreaks={} manualBreaks={} pivotBreaks={} breakExtentSum={} condFmt={} validations={} protectedRanges={} scenarios={}",
      sheet.metrics.dimension.as_deref().unwrap_or(""),
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
      sheet.metrics.conditional_formats,
      sheet.metrics.data_validations,
      sheet.metrics.protected_ranges,
      sheet.metrics.scenarios
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
