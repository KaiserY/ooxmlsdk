use ooxmlsdk::parts::chartsheet_part::ChartsheetPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use super::comments::CommentsCatalog;
use super::drawing::DrawingResourceCatalog;
use super::object_resources::WorksheetObjectResourceCatalog;
use super::page_settings::CalcPageSettings;
use super::pivot::PivotTableCatalog;
use super::query::QueryTableCatalog;
use super::sheet_conditions::SheetConditionCatalog;
use super::sheet_objects::SheetObjectCatalog;
use super::sheet_relationships::SheetRelationshipCatalog;
use super::sheet_settings::SheetSettingsCatalog;
use super::sheet_view::SheetViewCatalog;
use super::table::TableResourceCatalog;
use crate::error::Result;

#[derive(Clone, Debug)]
pub(crate) struct CalcSheet {
  pub(crate) workbook_index: usize,
  pub(crate) name: String,
  pub(crate) relationship_id: String,
  pub(crate) sheet_id: u32,
  pub(crate) sheet_type: SheetType,
  pub(crate) state: Option<x::SheetStateValues>,
  pub(crate) active: bool,
  pub(crate) page_settings: CalcPageSettings,
  pub(crate) metrics: SheetMetrics,
  pub(crate) chartsheet_metrics: Option<ChartsheetMetrics>,
  pub(crate) resources: SheetResourceCatalog,
  pub(crate) rows: Vec<CalcRow>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SheetType {
  Worksheet,
  Chartsheet,
  Unresolved,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetResourceCatalog {
  pub(crate) drawings: Vec<DrawingResourceCatalog>,
  pub(crate) object_resources: WorksheetObjectResourceCatalog,
  pub(crate) comments: CommentsCatalog,
  pub(crate) tables: Vec<TableResourceCatalog>,
  pub(crate) pivot_tables: PivotTableCatalog,
  pub(crate) query_tables: QueryTableCatalog,
  pub(crate) relationships: SheetRelationshipCatalog,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetMetrics {
  pub(crate) dimension: Option<String>,
  pub(crate) settings: SheetSettingsCatalog,
  pub(crate) views: SheetViewCatalog,
  pub(crate) format: SheetFormatModel,
  pub(crate) columns: Vec<ColumnModel>,
  pub(crate) merged_ranges: Vec<String>,
  pub(crate) hyperlinks: Vec<HyperlinkModel>,
  pub(crate) row_breaks: Vec<PageBreakModel>,
  pub(crate) column_breaks: Vec<PageBreakModel>,
  pub(crate) conditions: SheetConditionCatalog,
  pub(crate) objects: SheetObjectCatalog,
  pub(crate) protected_ranges: usize,
  pub(crate) scenarios: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ChartsheetMetrics {
  pub(crate) published: bool,
  pub(crate) code_name: Option<String>,
  pub(crate) has_tab_color: bool,
  pub(crate) views: usize,
  pub(crate) selected_views: usize,
  pub(crate) zoom_to_fit_views: usize,
  pub(crate) view_extensions: usize,
  pub(crate) custom_views: usize,
  pub(crate) custom_view_flags: usize,
  pub(crate) protection_flags: usize,
  pub(crate) web_publish_items: usize,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetFormatModel {
  pub(crate) base_column_width: Option<u32>,
  pub(crate) default_column_width: Option<f64>,
  pub(crate) default_row_height: f64,
  pub(crate) custom_height: bool,
  pub(crate) zero_height: bool,
  pub(crate) thick_top: bool,
  pub(crate) thick_bottom: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ColumnModel {
  pub(crate) first: u32,
  pub(crate) last: u32,
  pub(crate) width: Option<f64>,
  pub(crate) style_index: Option<u32>,
  pub(crate) hidden: bool,
  pub(crate) best_fit: bool,
  pub(crate) custom_width: bool,
  pub(crate) phonetic: bool,
  pub(crate) outline_level: u8,
  pub(crate) collapsed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HyperlinkModel {
  pub(crate) reference: String,
  pub(crate) relationship_id: Option<String>,
  pub(crate) location: Option<String>,
  pub(crate) display: Option<String>,
  pub(crate) tooltip: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PageBreakModel {
  pub(crate) id: u32,
  pub(crate) min: u32,
  pub(crate) max: u32,
  pub(crate) manual: bool,
  pub(crate) pivot: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcRow {
  pub(crate) row_index: Option<u32>,
  pub(crate) hidden: bool,
  pub(crate) cells: Vec<CalcCell>,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcCell {
  pub(crate) reference: Option<String>,
  pub(crate) style_index: Option<u32>,
  pub(crate) data_type: Option<x::CellValues>,
  pub(crate) cell_meta_index: Option<u32>,
  pub(crate) value_meta_index: Option<u32>,
  pub(crate) show_phonetic: bool,
  pub(crate) formula: Option<FormulaModel>,
  pub(crate) cached_value: Option<String>,
  pub(crate) display_text: String,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaModel {
  pub(crate) formula_type: x::CellFormulaValues,
  pub(crate) reference: Option<String>,
  pub(crate) shared_index: Option<u32>,
  pub(crate) text: String,
  pub(crate) always_calculate_array: bool,
  pub(crate) calculate_cell: bool,
  pub(crate) data_table_2d: bool,
  pub(crate) data_table_row: bool,
  pub(crate) input1_deleted: bool,
  pub(crate) input2_deleted: bool,
  pub(crate) input1_reference: Option<String>,
  pub(crate) input2_reference: Option<String>,
  pub(crate) assigns_value_to_name: bool,
}

impl CalcSheet {
  pub(crate) fn from_worksheet(
    workbook_index: usize,
    name: String,
    sheet_id: u32,
    relationship_id: String,
    state: Option<x::SheetStateValues>,
    active: bool,
    worksheet: x::Worksheet,
    resources: SheetResourceCatalog,
    shared_strings: &[String],
  ) -> Self {
    let page_settings = CalcPageSettings::from_worksheet(&worksheet);
    let metrics = SheetMetrics::from_worksheet(&worksheet);
    Self {
      workbook_index,
      name,
      relationship_id,
      sheet_id,
      sheet_type: SheetType::Worksheet,
      state,
      active,
      page_settings,
      metrics,
      chartsheet_metrics: None,
      resources,
      rows: worksheet_rows(&worksheet, shared_strings),
    }
  }

  pub(crate) fn from_chartsheet(
    workbook_index: usize,
    name: String,
    sheet_id: u32,
    relationship_id: String,
    state: Option<x::SheetStateValues>,
    active: bool,
    chartsheet: x::Chartsheet,
    resources: SheetResourceCatalog,
  ) -> Self {
    let page_settings = CalcPageSettings::from_chartsheet(&chartsheet);
    Self {
      workbook_index,
      name,
      relationship_id,
      sheet_id,
      sheet_type: SheetType::Chartsheet,
      state,
      active,
      page_settings,
      metrics: SheetMetrics::default(),
      chartsheet_metrics: Some(ChartsheetMetrics::from_chartsheet(&chartsheet)),
      resources,
      rows: Vec::new(),
    }
  }

  pub(crate) fn unresolved(
    workbook_index: usize,
    name: String,
    sheet_id: u32,
    relationship_id: String,
    state: Option<x::SheetStateValues>,
    active: bool,
  ) -> Self {
    Self {
      workbook_index,
      name,
      relationship_id,
      sheet_id,
      sheet_type: SheetType::Unresolved,
      state,
      active,
      page_settings: CalcPageSettings::default(),
      metrics: SheetMetrics::default(),
      chartsheet_metrics: None,
      resources: SheetResourceCatalog::default(),
      rows: Vec::new(),
    }
  }

  pub(crate) fn visible(&self) -> bool {
    self.active
      || !matches!(
        self.state,
        Some(x::SheetStateValues::Hidden | x::SheetStateValues::VeryHidden)
      )
  }
}

impl ChartsheetMetrics {
  fn from_chartsheet(chartsheet: &x::Chartsheet) -> Self {
    // Source: LibreOffice sc/source/filter/oox/chartsheetfragment.cxx imports
    // chartsheet properties/protection/views through WorksheetSettings and
    // SheetViewSettings before final worksheet import.
    let properties = chartsheet.chart_sheet_properties.as_deref();
    let protection = chartsheet.chart_sheet_protection.as_ref();
    Self {
      published: properties
        .and_then(|properties| properties.published)
        .is_some_and(|value| value.as_bool()),
      code_name: properties.and_then(|properties| properties.code_name.clone()),
      has_tab_color: properties.is_some_and(|properties| properties.tab_color.is_some()),
      views: chartsheet.chart_sheet_views.chart_sheet_view.len(),
      selected_views: chartsheet
        .chart_sheet_views
        .chart_sheet_view
        .iter()
        .filter(|view| view.tab_selected.is_some_and(|value| value.as_bool()))
        .count(),
      zoom_to_fit_views: chartsheet
        .chart_sheet_views
        .chart_sheet_view
        .iter()
        .filter(|view| view.zoom_to_fit.is_some_and(|value| value.as_bool()))
        .count(),
      view_extensions: usize::from(chartsheet.chart_sheet_views.extension_list.is_some())
        + chartsheet
          .chart_sheet_views
          .chart_sheet_view
          .iter()
          .filter(|view| view.extension_list.is_some())
          .count(),
      custom_views: chartsheet
        .custom_chartsheet_views
        .as_ref()
        .map_or(0, |views| views.custom_chartsheet_view.len()),
      custom_view_flags: chartsheet
        .custom_chartsheet_views
        .as_ref()
        .map_or(0, |views| {
          views
            .custom_chartsheet_view
            .iter()
            .map(|view| {
              usize::from(view.scale.is_some())
                + usize::from(view.state.is_some())
                + usize::from(view.zoom_to_fit.is_some_and(|value| value.as_bool()))
                + usize::from(view.page_margins.is_some())
                + usize::from(view.chart_sheet_page_setup.is_some())
                + usize::from(view.header_footer.is_some())
                + view.guid.len()
            })
            .sum()
        }),
      protection_flags: protection.map_or(0, |protection| {
        usize::from(protection.password.is_some())
          + usize::from(protection.algorithm_name.is_some())
          + usize::from(protection.hash_value.is_some())
          + usize::from(protection.salt_value.is_some())
          + usize::from(protection.spin_count.is_some())
          + usize::from(protection.content.is_some_and(|value| value.as_bool()))
          + usize::from(protection.objects.is_some_and(|value| value.as_bool()))
      }),
      web_publish_items: chartsheet
        .web_publish_items
        .as_ref()
        .map_or(0, |items| items.web_publish_item.len()),
      has_extensions: chartsheet.extension_list.is_some(),
    }
  }
}

impl SheetMetrics {
  fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    // Source: LibreOffice sc/source/filter/oox/worksheetfragment.cxx
    // WorksheetFragment imports dimensions, sheetFormatPr, cols,
    // mergeCells, hyperlinks, rowBreaks, and colBreaks before page layout.
    Self {
      dimension: worksheet
        .sheet_dimension
        .as_ref()
        .map(|dimension| dimension.reference.clone()),
      settings: SheetSettingsCatalog::from_worksheet(worksheet),
      views: SheetViewCatalog::from_worksheet(worksheet),
      format: worksheet
        .sheet_format_properties
        .as_ref()
        .map(SheetFormatModel::from_sheet_format_properties)
        .unwrap_or_default(),
      columns: worksheet
        .columns
        .iter()
        .flat_map(|columns| columns.column.iter().map(ColumnModel::from_column))
        .collect(),
      merged_ranges: worksheet
        .merge_cells
        .as_ref()
        .map(|merge_cells| {
          merge_cells
            .merge_cell
            .iter()
            .map(|merge_cell| merge_cell.reference.clone())
            .collect()
        })
        .unwrap_or_default(),
      hyperlinks: worksheet
        .hyperlinks
        .as_ref()
        .map(|hyperlinks| {
          hyperlinks
            .hyperlink
            .iter()
            .map(HyperlinkModel::from_hyperlink)
            .collect()
        })
        .unwrap_or_default(),
      row_breaks: worksheet
        .row_breaks
        .as_ref()
        .map(|breaks| {
          breaks
            .r#break
            .iter()
            .map(PageBreakModel::from_break)
            .collect()
        })
        .unwrap_or_default(),
      column_breaks: worksheet
        .column_breaks
        .as_ref()
        .map(|breaks| {
          breaks
            .r#break
            .iter()
            .map(PageBreakModel::from_break)
            .collect()
        })
        .unwrap_or_default(),
      conditions: SheetConditionCatalog::from_worksheet(worksheet),
      objects: SheetObjectCatalog::from_worksheet(worksheet),
      protected_ranges: worksheet
        .protected_ranges
        .as_ref()
        .map_or(0, |ranges| ranges.protected_range.len()),
      scenarios: worksheet
        .scenarios
        .as_ref()
        .map_or(0, |scenarios| scenarios.scenario.len()),
    }
  }
}

impl SheetFormatModel {
  fn from_sheet_format_properties(format: &x::SheetFormatProperties) -> Self {
    Self {
      base_column_width: format.base_column_width,
      default_column_width: format.default_column_width,
      default_row_height: format.default_row_height,
      custom_height: format.custom_height.is_some_and(|value| value.as_bool()),
      zero_height: format.zero_height.is_some_and(|value| value.as_bool()),
      thick_top: format.thick_top.is_some_and(|value| value.as_bool()),
      thick_bottom: format.thick_bottom.is_some_and(|value| value.as_bool()),
    }
  }
}

impl ColumnModel {
  fn from_column(column: &x::Column) -> Self {
    Self {
      first: column.min,
      last: column.max,
      width: column.width,
      style_index: column.style,
      hidden: column.hidden.is_some_and(|value| value.as_bool()),
      best_fit: column.best_fit.is_some_and(|value| value.as_bool()),
      custom_width: column.custom_width.is_some_and(|value| value.as_bool()),
      phonetic: column.phonetic.is_some_and(|value| value.as_bool()),
      outline_level: column.outline_level.unwrap_or(0),
      collapsed: column.collapsed.is_some_and(|value| value.as_bool()),
    }
  }
}

impl HyperlinkModel {
  fn from_hyperlink(hyperlink: &x::Hyperlink) -> Self {
    Self {
      reference: hyperlink.reference.clone(),
      relationship_id: hyperlink.id.clone(),
      location: hyperlink.location.clone(),
      display: hyperlink.display.clone(),
      tooltip: hyperlink.tooltip.clone(),
    }
  }
}

impl PageBreakModel {
  fn from_break(page_break: &x::Break) -> Self {
    let id = page_break.id.unwrap_or(0);
    Self {
      id,
      min: page_break.min.unwrap_or(id),
      max: page_break.max.unwrap_or(id),
      manual: page_break
        .manual_page_break
        .is_some_and(|value| value.as_bool()),
      pivot: page_break
        .pivot_table_page_break
        .is_some_and(|value| value.as_bool()),
    }
  }
}

impl SheetResourceCatalog {
  pub(crate) fn from_worksheet_part(
    package: &mut SpreadsheetDocument,
    part: &WorksheetPart,
  ) -> Result<Self> {
    let drawings = part
      .drawings_part(package)
      .map(|drawing| DrawingResourceCatalog::from_part(package, &drawing))
      .transpose()?
      .map(|drawing| vec![drawing])
      .unwrap_or_default();
    let table_parts = part.table_definition_parts(package).collect::<Vec<_>>();
    let tables = table_parts
      .iter()
      .map(|table| TableResourceCatalog::from_part(package, table))
      .collect::<Result<Vec<_>>>()?;
    let comments_part = part.worksheet_comments_part(package);
    let threaded_comment_parts = part
      .worksheet_threaded_comments_parts(package)
      .collect::<Vec<_>>();
    let comments =
      CommentsCatalog::from_worksheet_part(package, comments_part, threaded_comment_parts)?;
    let pivot_table_parts = part.pivot_table_parts(package).collect::<Vec<_>>();
    let pivot_tables = PivotTableCatalog::from_parts(package, &pivot_table_parts)?;
    let query_table_parts = part.query_table_parts(package).collect::<Vec<_>>();
    let query_tables = QueryTableCatalog::from_parts(package, &query_table_parts)?;
    let relationships = SheetRelationshipCatalog::from_worksheet_part(package, part)?;
    let object_resources = WorksheetObjectResourceCatalog::from_worksheet_part(package, part)?;
    Ok(Self {
      drawings,
      object_resources,
      comments,
      tables,
      pivot_tables,
      query_tables,
      relationships,
    })
  }

  pub(crate) fn from_chartsheet_part(
    package: &mut SpreadsheetDocument,
    part: &ChartsheetPart,
  ) -> Result<Self> {
    let drawings = part
      .drawings_part(package)
      .map(|drawing| DrawingResourceCatalog::from_part(package, &drawing))
      .transpose()?
      .map(|drawing| vec![drawing])
      .unwrap_or_default();
    Ok(Self {
      drawings,
      object_resources: WorksheetObjectResourceCatalog::from_chartsheet_part(package, part),
      ..Self::default()
    })
  }
}

fn worksheet_rows(worksheet: &x::Worksheet, shared_strings: &[String]) -> Vec<CalcRow> {
  worksheet
    .sheet_data
    .row
    .iter()
    .map(|row| CalcRow {
      row_index: row.row_index,
      hidden: row.hidden.is_some_and(|value| value.as_bool()),
      cells: row
        .cell
        .iter()
        .map(|cell| CalcCell::from_cell(cell, shared_strings))
        .collect(),
    })
    .collect()
}

impl CalcCell {
  fn from_cell(cell: &x::Cell, shared_strings: &[String]) -> Self {
    let cached_value = cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .map(ToString::to_string);
    Self {
      reference: cell.cell_reference.as_ref().map(ToString::to_string),
      style_index: cell.style_index,
      data_type: cell.data_type,
      cell_meta_index: cell.cell_meta_index,
      value_meta_index: cell.value_meta_index,
      show_phonetic: cell.show_phonetic.is_some_and(|value| value.as_bool()),
      formula: cell.cell_formula.as_ref().map(FormulaModel::from_formula),
      cached_value,
      display_text: cell_text(cell, shared_strings),
      has_extensions: cell.extension_list.is_some(),
    }
  }
}

impl FormulaModel {
  fn from_formula(formula: &x::CellFormula) -> Self {
    // Source: LibreOffice sc/source/filter/oox/sheetdatacontext.cxx and
    // sheetdatabuffer.cxx. These fields decide whether the cell is imported
    // as a normal formula, shared formula, array formula, or data-table
    // operation. Token conversion is a later FormulaBuffer responsibility.
    Self {
      formula_type: formula.formula_type.unwrap_or_default(),
      reference: formula.reference.clone(),
      shared_index: formula.shared_index,
      text: formula.xml_content.clone().unwrap_or_default(),
      always_calculate_array: formula
        .always_calculate_array
        .is_some_and(|value| value.as_bool()),
      calculate_cell: formula.calculate_cell.is_some_and(|value| value.as_bool()),
      data_table_2d: formula.data_table2_d.is_some_and(|value| value.as_bool()),
      data_table_row: formula.data_table_row.is_some_and(|value| value.as_bool()),
      input1_deleted: formula.input1_deleted.is_some_and(|value| value.as_bool()),
      input2_deleted: formula.input2_deleted.is_some_and(|value| value.as_bool()),
      input1_reference: formula.r1.clone(),
      input2_reference: formula.r2.clone(),
      assigns_value_to_name: formula.bx.is_some_and(|value| value.as_bool()),
    }
  }
}

fn inline_string_text(value: &x::InlineString) -> String {
  if let Some(text) = &value.text
    && let Some(content) = &text.xml_content
  {
    return content.to_string();
  }

  value
    .run
    .iter()
    .filter_map(|run| run.text.xml_content.as_deref())
    .collect()
}

fn cell_text(cell: &x::Cell, shared_strings: &[String]) -> String {
  match cell.data_type {
    Some(x::CellValues::SharedString) => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .and_then(|index| index.parse::<usize>().ok())
      .and_then(|index| shared_strings.get(index))
      .cloned()
      .unwrap_or_default(),
    Some(x::CellValues::InlineString) => cell
      .inline_string
      .as_deref()
      .map(inline_string_text)
      .unwrap_or_default(),
    Some(x::CellValues::Boolean) => match cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
    {
      Some("1") => "TRUE".to_string(),
      Some("0") => "FALSE".to_string(),
      Some(value) => value.to_string(),
      None => String::new(),
    },
    _ => cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.as_deref())
      .map(ToString::to_string)
      .unwrap_or_default(),
  }
}
