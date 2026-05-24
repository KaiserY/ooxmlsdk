use ooxmlsdk::parts::chartsheet_part::ChartsheetPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use super::drawing::DrawingResourceCatalog;
use super::page_settings::CalcPageSettings;
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
  pub(crate) vml_drawings: usize,
  pub(crate) comments: usize,
  pub(crate) threaded_comments: usize,
  pub(crate) tables: Vec<TableResourceCatalog>,
  pub(crate) pivot_tables: usize,
  pub(crate) query_tables: usize,
  pub(crate) controls: usize,
  pub(crate) control_properties: usize,
  pub(crate) embedded_objects: usize,
  pub(crate) embedded_packages: usize,
  pub(crate) images: usize,
  pub(crate) named_sheet_views: usize,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SheetMetrics {
  pub(crate) dimension: Option<String>,
  pub(crate) format: SheetFormatModel,
  pub(crate) columns: Vec<ColumnModel>,
  pub(crate) merged_ranges: Vec<String>,
  pub(crate) hyperlinks: Vec<HyperlinkModel>,
  pub(crate) row_breaks: Vec<PageBreakModel>,
  pub(crate) column_breaks: Vec<PageBreakModel>,
  pub(crate) conditional_formats: usize,
  pub(crate) data_validations: usize,
  pub(crate) protected_ranges: usize,
  pub(crate) scenarios: usize,
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
  pub(crate) formula: Option<String>,
  pub(crate) cached_value: Option<String>,
  pub(crate) display_text: String,
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
      conditional_formats: worksheet.conditional_formatting.len(),
      data_validations: worksheet
        .data_validations
        .as_ref()
        .map_or(0, |validations| validations.data_validation.len()),
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
      .map(|drawing| vec![DrawingResourceCatalog::from_part(package, &drawing)])
      .unwrap_or_default();
    let table_parts = part.table_definition_parts(package).collect::<Vec<_>>();
    let tables = table_parts
      .iter()
      .map(|table| TableResourceCatalog::from_part(package, table))
      .collect::<Result<Vec<_>>>()?;
    Ok(Self {
      drawings,
      vml_drawings: part.vml_drawing_parts(package).count(),
      comments: usize::from(part.worksheet_comments_part(package).is_some()),
      threaded_comments: part.worksheet_threaded_comments_parts(package).count(),
      tables,
      pivot_tables: part.pivot_table_parts(package).count(),
      query_tables: part.query_table_parts(package).count(),
      controls: part.embedded_control_persistence_parts(package).count(),
      control_properties: part.control_properties_parts(package).count(),
      embedded_objects: part.embedded_object_parts(package).count(),
      embedded_packages: part.embedded_package_parts(package).count(),
      images: part.image_parts(package).count(),
      named_sheet_views: part.named_sheet_views_parts(package).count(),
    })
  }

  pub(crate) fn from_chartsheet_part(
    package: &mut SpreadsheetDocument,
    part: &ChartsheetPart,
  ) -> Self {
    let drawings = part
      .drawings_part(package)
      .map(|drawing| vec![DrawingResourceCatalog::from_part(package, &drawing)])
      .unwrap_or_default();
    Self {
      drawings,
      vml_drawings: part.vml_drawing_parts(package).count(),
      images: part.image_parts(package).count(),
      ..Self::default()
    }
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
      formula: cell
        .cell_formula
        .as_ref()
        .and_then(|formula| formula.xml_content.as_deref())
        .map(ToString::to_string),
      cached_value,
      display_text: cell_text(cell, shared_strings),
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
