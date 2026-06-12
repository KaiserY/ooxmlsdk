use std::borrow::Cow;

use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::schemas::x;
use ooxmlsdk_formula::{
  BuiltInName, CellValueProvider, QualifiedRange as FormulaQualifiedRange, SheetId,
};

use crate::common::{Fill, Insets, Point, Pt, Rect, Size, Stroke, Twips};

// Source: LibreOffice sc/inc/global.hxx STD_COL_WIDTH = convert(64pt, twip).
const CALC_STANDARD_COLUMN_WIDTH_TWIPS: i32 = 1280;
// OOXML sheetFormatPr defaultRowHeight is commonly 15pt; LO may derive this
// from the standard row height when the attribute is absent.
const DEFAULT_ROW_HEIGHT_PT: f64 = 15.0;
const POINTS_PER_INCH: f64 = 72.0;
// Source: LibreOffice sc/source/ui/view/printfun.cxx uses A4 as print fallback.
const A4_PORTRAIT_WIDTH_PT: f32 = 595.2756;
const A4_PORTRAIT_HEIGHT_PT: f32 = 841.8898;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxWorkbook<'doc> {
  pub sheets: Vec<XlsxSheet<'doc>>,
  pub styles: XlsxStyleCatalog<'doc>,
  pub page_styles: Vec<XlsxPageStyle<'doc>>,
  pub drawings: Vec<XlsxDrawing<'doc>>,
  pub print_plan: XlsxPrintPlan<'doc>,
  pub value_model: Option<&'doc ooxmlsdk_formula::WorkbookValueModel<'doc>>,
}

impl<'doc> XlsxWorkbook<'doc> {
  pub fn from_spreadsheet_document(document: &'doc mut SpreadsheetDocument) -> Self {
    Self::from_spreadsheet_document_with_values(document, None)
  }

  pub fn from_spreadsheet_document_with_values(
    document: &'doc mut SpreadsheetDocument,
    value_model: Option<&'doc ooxmlsdk_formula::WorkbookValueModel<'doc>>,
  ) -> Self {
    let Ok(workbook_part) = document.workbook_part() else {
      return Self::default();
    };
    let workbook_part = workbook_part.clone();
    let Ok(workbook_root) = workbook_part.root_element(document) else {
      return Self::default();
    };
    let sheet_identities = workbook_root.sheets.sheet.clone();
    let worksheet_parts = workbook_part.worksheet_parts(document).collect::<Vec<_>>();
    let mut sheets = Vec::new();
    for (index, sheet) in sheet_identities.iter().enumerate() {
      let worksheet = worksheet_parts
        .get(index)
        .and_then(|part| part.root_element(document).ok());
      sheets.push(import_sheet(index, sheet, worksheet, value_model));
    }
    if let Some(value_model) = value_model {
      apply_defined_names_to_sheets(&mut sheets, value_model);
    }

    Self {
      sheets,
      value_model,
      ..Self::default()
    }
  }

  pub fn build_print_plan(&self) -> XlsxPrintPlan<'doc> {
    build_print_plan(self)
  }
}

fn import_sheet<'doc>(
  index: usize,
  sheet: &x::Sheet,
  worksheet: Option<&x::Worksheet>,
  value_model: Option<&'doc ooxmlsdk_formula::WorkbookValueModel<'doc>>,
) -> XlsxSheet<'doc> {
  let Some(worksheet) = worksheet else {
    return XlsxSheet {
      workbook_index: index,
      name: Cow::Owned(sheet.name.clone()),
      state: sheet_state(sheet.state),
      ..XlsxSheet::default()
    };
  };
  XlsxSheet {
    workbook_index: index,
    name: Cow::Owned(sheet.name.clone()),
    state: sheet_state(sheet.state),
    page_setup: import_page_setup(worksheet),
    metrics: import_sheet_metrics(worksheet),
    rows: worksheet
      .sheet_data
      .row
      .iter()
      .enumerate()
      .map(|(row_position, row)| {
        import_row(
          row_position,
          row,
          value_model,
          formula_sheet_id(index, sheet, value_model),
        )
      })
      .collect(),
    ..XlsxSheet::default()
  }
}

fn formula_sheet_id<'doc>(
  index: usize,
  sheet: &x::Sheet,
  value_model: Option<&'doc ooxmlsdk_formula::WorkbookValueModel<'doc>>,
) -> SheetId {
  value_model
    .and_then(|model| model.identity.sheets.get(index))
    .map(|identity| identity.id)
    .unwrap_or(SheetId(sheet.sheet_id))
}

fn apply_defined_names_to_sheets<'doc>(
  sheets: &mut [XlsxSheet<'doc>],
  value_model: &ooxmlsdk_formula::WorkbookValueModel<'doc>,
) {
  for defined_name in &value_model.defined_names {
    let Some(built_in) = defined_name.built_in else {
      continue;
    };
    if !matches!(built_in, BuiltInName::PrintArea | BuiltInName::PrintTitles) {
      continue;
    }
    for range_text in split_defined_name_ranges(&defined_name.formula_text) {
      let sheet_hint = defined_name.sheet.unwrap_or_default();
      let Ok(range) = FormulaQualifiedRange::parse_a1(sheet_hint, range_text) else {
        continue;
      };
      let Some(sheet_index) = defined_range_sheet_index(sheets, defined_name.sheet, &range) else {
        continue;
      };
      let whole_rows = range.start_flags.whole_row && range.end_flags.whole_row;
      let whole_columns = range.start_flags.whole_column && range.end_flags.whole_column;
      let range = layout_cell_range(range.range);
      let metrics = &mut sheets[sheet_index].metrics;
      match built_in {
        BuiltInName::PrintArea => metrics.print_ranges.push(range),
        BuiltInName::PrintTitles => {
          if whole_rows {
            metrics.repeated_rows.push(range);
          } else if whole_columns {
            metrics.repeated_columns.push(range);
          }
        }
        _ => {}
      }
    }
  }
}

fn split_defined_name_ranges(value: &str) -> Vec<&str> {
  let value = value.trim().trim_start_matches('=');
  let mut quoted = false;
  let mut start = 0;
  let mut ranges = Vec::new();
  let mut chars = value.char_indices().peekable();
  while let Some((index, ch)) = chars.next() {
    match ch {
      '\'' => {
        if quoted && chars.peek().is_some_and(|(_, next)| *next == '\'') {
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      ',' if !quoted => {
        push_defined_name_range(&mut ranges, &value[start..index]);
        start = index + ch.len_utf8();
      }
      _ => {}
    }
  }
  push_defined_name_range(&mut ranges, &value[start..]);
  ranges
}

fn push_defined_name_range<'a>(ranges: &mut Vec<&'a str>, value: &'a str) {
  let value = value.trim();
  if !value.is_empty() {
    ranges.push(value);
  }
}

fn defined_range_sheet_index(
  sheets: &[XlsxSheet<'_>],
  local_sheet: Option<SheetId>,
  range: &FormulaQualifiedRange<'_>,
) -> Option<usize> {
  if let Some(sheet_name) = &range.sheet_name {
    return sheets
      .iter()
      .position(|sheet| sheet.name.as_ref() == sheet_name.0.as_ref());
  }
  local_sheet
    .map(|sheet| sheet.0 as usize)
    .filter(|index| *index < sheets.len())
}

fn layout_cell_range(range: ooxmlsdk_formula::CellRange) -> CellRange {
  CellRange {
    start: CellAddress {
      column: range.start.column,
      row: range.start.row,
    },
    end: CellAddress {
      column: range.end.column,
      row: range.end.row,
    },
  }
}

fn sheet_state(value: Option<x::SheetStateValues>) -> SheetState {
  match value.unwrap_or_default() {
    x::SheetStateValues::Visible | x::SheetStateValues::Show => SheetState::Visible,
    x::SheetStateValues::Hidden => SheetState::Hidden,
    x::SheetStateValues::VeryHidden => SheetState::VeryHidden,
  }
}

fn import_page_setup<'doc>(worksheet: &x::Worksheet) -> XlsxPageSetup<'doc> {
  let mut setup = XlsxPageSetup::default();
  if let Some(page_setup) = &worksheet.page_setup {
    setup.paper_size = page_setup.paper_size;
    setup.scale_percent = page_setup.scale.map(|value| value as u16);
    setup.fit_to_width = page_setup.fit_to_width.map(|value| value as u16);
    setup.fit_to_height = page_setup.fit_to_height.map(|value| value as u16);
    setup.horizontal_dpi = page_setup.horizontal_dpi;
    setup.vertical_dpi = page_setup.vertical_dpi;
    setup.orientation = match page_setup.orientation.unwrap_or_default() {
      x::OrientationValues::Landscape => PageOrientation::Landscape,
      x::OrientationValues::Default | x::OrientationValues::Portrait => PageOrientation::Portrait,
    };
  }
  if let Some(margins) = &worksheet.page_margins {
    setup.margins = Insets {
      top: Pt((margins.top * POINTS_PER_INCH) as f32),
      right: Pt((margins.right * POINTS_PER_INCH) as f32),
      bottom: Pt((margins.bottom * POINTS_PER_INCH) as f32),
      left: Pt((margins.left * POINTS_PER_INCH) as f32),
    };
  }
  if let Some(print_options) = &worksheet.print_options {
    setup.print_options = PrintOptions {
      grid_lines: print_options
        .grid_lines
        .is_some_and(|value| value.as_bool()),
      headings: print_options.headings.is_some_and(|value| value.as_bool()),
      horizontal_centered: print_options
        .horizontal_centered
        .is_some_and(|value| value.as_bool()),
      vertical_centered: print_options
        .vertical_centered
        .is_some_and(|value| value.as_bool()),
    };
  }
  if let Some(header_footer) = &worksheet.header_footer {
    setup.header_footer = HeaderFooterText {
      odd_header: header_footer
        .odd_header
        .as_ref()
        .and_then(|value| value.0.xml_content.clone())
        .map(Cow::Owned),
      odd_footer: header_footer
        .odd_footer
        .as_ref()
        .and_then(|value| value.0.xml_content.clone())
        .map(Cow::Owned),
      even_header: header_footer
        .even_header
        .as_ref()
        .and_then(|value| value.0.xml_content.clone())
        .map(Cow::Owned),
      even_footer: header_footer
        .even_footer
        .as_ref()
        .and_then(|value| value.0.xml_content.clone())
        .map(Cow::Owned),
      first_header: header_footer
        .first_header
        .as_ref()
        .and_then(|value| value.0.xml_content.clone())
        .map(Cow::Owned),
      first_footer: header_footer
        .first_footer
        .as_ref()
        .and_then(|value| value.0.xml_content.clone())
        .map(Cow::Owned),
    };
  }
  setup
}

fn import_sheet_metrics<'doc>(worksheet: &x::Worksheet) -> SheetMetrics<'doc> {
  let first_view = worksheet
    .sheet_views
    .as_deref()
    .and_then(|views| views.sheet_view.first());
  SheetMetrics {
    dimension: worksheet
      .sheet_dimension
      .as_ref()
      .map(|dimension| Cow::Owned(dimension.reference.clone())),
    format: worksheet
      .sheet_format_properties
      .as_ref()
      .map(import_sheet_format)
      .unwrap_or_default(),
    columns: worksheet
      .columns
      .iter()
      .flat_map(|columns| columns.column.iter())
      .map(import_column)
      .collect(),
    merged_ranges: worksheet
      .merge_cells
      .as_ref()
      .into_iter()
      .flat_map(|merges| merges.merge_cell.iter())
      .filter_map(|merge| parse_layout_range(&merge.reference))
      .collect(),
    row_breaks: worksheet
      .row_breaks
      .as_ref()
      .into_iter()
      .flat_map(|breaks| breaks.r#break.iter())
      .map(import_page_break)
      .collect(),
    column_breaks: worksheet
      .column_breaks
      .as_ref()
      .into_iter()
      .flat_map(|breaks| breaks.r#break.iter())
      .map(import_page_break)
      .collect(),
    viewport: SheetViewport {
      active_cell: first_view
        .and_then(|view| view.selection.first())
        .and_then(|selection| selection.active_cell.as_deref())
        .and_then(parse_layout_address),
      top_left_cell: first_view
        .and_then(|view| view.top_left_cell.as_deref())
        .and_then(parse_layout_address),
      frozen_panes: first_view
        .and_then(|view| view.pane.as_ref())
        .and_then(|pane| {
          Some(FrozenPane {
            split_column: pane.horizontal_split? as u32,
            split_row: pane.vertical_split? as u32,
          })
        }),
    },
    print_ranges: Vec::new(),
    repeated_rows: Vec::new(),
    repeated_columns: Vec::new(),
    hyperlinks: Vec::new(),
  }
}

fn import_sheet_format(format: &x::SheetFormatProperties) -> SheetFormat {
  let mut default_row_height = if format.default_row_height > 0.0 {
    format.default_row_height
  } else {
    DEFAULT_ROW_HEIGHT_PT
  };
  default_row_height -= default_row_height % 0.75;
  SheetFormat {
    base_column_width: format.base_column_width,
    default_column_width: format.default_column_width,
    default_row_height,
    custom_height: format.custom_height.is_some_and(|value| value.as_bool()),
    zero_height: format.zero_height.is_some_and(|value| value.as_bool()),
  }
}

fn import_column(column: &x::Column) -> ColumnModel {
  ColumnModel {
    first: column.min.saturating_sub(1),
    last: column.max.saturating_sub(1),
    width: column.width,
    style_index: column.style,
    hidden: column.hidden.is_some_and(|value| value.as_bool()),
    best_fit: column.best_fit.is_some_and(|value| value.as_bool()),
    custom_width: column.custom_width.is_some_and(|value| value.as_bool()),
    outline_level: column.outline_level.unwrap_or_default(),
    collapsed: column.collapsed.is_some_and(|value| value.as_bool()),
  }
}

fn import_page_break(value: &x::Break) -> PageBreak {
  PageBreak {
    id: value.id.unwrap_or_default(),
    min: value.min.unwrap_or_default(),
    max: value.max.unwrap_or_default(),
    manual: value.manual_page_break.is_some_and(|value| value.as_bool()),
    pivot: value
      .pivot_table_page_break
      .is_some_and(|value| value.as_bool()),
  }
}

fn import_row<'doc>(
  row_position: usize,
  row: &x::Row,
  value_model: Option<&'doc ooxmlsdk_formula::WorkbookValueModel<'doc>>,
  sheet_id: ooxmlsdk_formula::SheetId,
) -> XlsxRow<'doc> {
  let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
  let mut current_column = 0u32;
  XlsxRow {
    row_index: Some(row_index),
    height: row.height,
    custom_height: row.custom_height.is_some_and(|value| value.as_bool()),
    style_index: row.style_index,
    hidden: row.hidden.is_some_and(|value| value.as_bool()),
    cells: row
      .cell
      .iter()
      .map(|cell| {
        let address = cell
          .cell_reference
          .as_deref()
          .and_then(parse_layout_address)
          .unwrap_or_else(|| {
            let address = CellAddress {
              column: current_column,
              row: row_index.saturating_sub(1),
            };
            current_column = current_column.saturating_add(1);
            address
          });
        current_column = address.column.saturating_add(1);
        import_cell(cell, address, value_model, sheet_id)
      })
      .collect(),
  }
}

fn import_cell<'doc>(
  cell: &x::Cell,
  address: CellAddress,
  value_model: Option<&'doc ooxmlsdk_formula::WorkbookValueModel<'doc>>,
  sheet_id: ooxmlsdk_formula::SheetId,
) -> XlsxCell<'doc> {
  let formula_address = ooxmlsdk_formula::CellAddress {
    column: address.column,
    row: address.row,
  };
  let display = value_model
    .and_then(|model| model.display_text(sheet_id, formula_address))
    .map(|value| value.text)
    .unwrap_or_else(|| {
      Cow::Owned(
        cell
          .cell_value
          .as_ref()
          .and_then(|value| value.xml_content.clone())
          .unwrap_or_default(),
      )
    });

  XlsxCell {
    reference: cell.cell_reference.clone().map(Cow::Owned),
    address: Some(address),
    style_index: cell.style_index,
    data_type: cell.data_type.map(cell_data_type),
    cached_value: cell
      .cell_value
      .as_ref()
      .and_then(|value| value.xml_content.clone())
      .map(Cow::Owned),
    display_text: display,
    formula_state: cell
      .cell_formula
      .as_ref()
      .map(|_| XlsxFormulaState::CachedOnly),
    ..XlsxCell::default()
  }
}

fn cell_data_type(value: x::CellValues) -> CellDataType {
  match value {
    x::CellValues::Boolean => CellDataType::Boolean,
    x::CellValues::Number => CellDataType::Number,
    x::CellValues::Error => CellDataType::Error,
    x::CellValues::SharedString => CellDataType::SharedString,
    x::CellValues::String => CellDataType::String,
    x::CellValues::InlineString => CellDataType::InlineString,
    x::CellValues::Date => CellDataType::Date,
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxSheet<'doc> {
  pub workbook_index: usize,
  pub name: Cow<'doc, str>,
  pub state: SheetState,
  pub page_setup: XlsxPageSetup<'doc>,
  pub metrics: SheetMetrics<'doc>,
  pub rows: Vec<XlsxRow<'doc>>,
  pub drawings: Vec<XlsxDrawing<'doc>>,
  pub tables: Vec<XlsxTable<'doc>>,
  pub auto_filter: Option<AutoFilter<'doc>>,
  pub conditions: Vec<ConditionalFormat<'doc>>,
  pub notes: Vec<XlsxNote<'doc>>,
  pub protected_ranges: Vec<ProtectedRange<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SheetState {
  #[default]
  Visible,
  Hidden,
  VeryHidden,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxPageSetup<'doc> {
  pub paper_size: Option<u32>,
  pub orientation: PageOrientation,
  pub margins: Insets,
  pub scale_percent: Option<u16>,
  pub fit_to_width: Option<u16>,
  pub fit_to_height: Option<u16>,
  pub horizontal_dpi: Option<u32>,
  pub vertical_dpi: Option<u32>,
  pub header_footer: HeaderFooterText<'doc>,
  pub print_options: PrintOptions,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PageOrientation {
  #[default]
  Portrait,
  Landscape,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeaderFooterText<'doc> {
  pub odd_header: Option<Cow<'doc, str>>,
  pub odd_footer: Option<Cow<'doc, str>>,
  pub even_header: Option<Cow<'doc, str>>,
  pub even_footer: Option<Cow<'doc, str>>,
  pub first_header: Option<Cow<'doc, str>>,
  pub first_footer: Option<Cow<'doc, str>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrintOptions {
  pub grid_lines: bool,
  pub headings: bool,
  pub horizontal_centered: bool,
  pub vertical_centered: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SheetMetrics<'doc> {
  pub dimension: Option<Cow<'doc, str>>,
  pub format: SheetFormat,
  pub columns: Vec<ColumnModel>,
  pub merged_ranges: Vec<CellRange>,
  pub hyperlinks: Vec<Hyperlink<'doc>>,
  pub row_breaks: Vec<PageBreak>,
  pub column_breaks: Vec<PageBreak>,
  pub print_ranges: Vec<CellRange>,
  pub repeated_rows: Vec<CellRange>,
  pub repeated_columns: Vec<CellRange>,
  pub viewport: SheetViewport,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SheetViewport {
  pub active_cell: Option<CellAddress>,
  pub top_left_cell: Option<CellAddress>,
  pub frozen_panes: Option<FrozenPane>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrozenPane {
  pub split_column: u32,
  pub split_row: u32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SheetFormat {
  pub base_column_width: Option<u32>,
  pub default_column_width: Option<f64>,
  pub default_row_height: f64,
  pub custom_height: bool,
  pub zero_height: bool,
}

impl Default for SheetFormat {
  fn default() -> Self {
    Self {
      base_column_width: None,
      default_column_width: None,
      default_row_height: DEFAULT_ROW_HEIGHT_PT,
      custom_height: false,
      zero_height: false,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct CellAddress {
  pub column: u32,
  pub row: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct CellRange {
  pub start: CellAddress,
  pub end: CellAddress,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColumnModel {
  pub first: u32,
  pub last: u32,
  pub width: Option<f64>,
  pub style_index: Option<u32>,
  pub hidden: bool,
  pub best_fit: bool,
  pub custom_width: bool,
  pub outline_level: u8,
  pub collapsed: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxRow<'doc> {
  pub row_index: Option<u32>,
  pub height: Option<f64>,
  pub custom_height: bool,
  pub style_index: Option<u32>,
  pub hidden: bool,
  pub cells: Vec<XlsxCell<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxCell<'doc> {
  pub reference: Option<Cow<'doc, str>>,
  pub address: Option<CellAddress>,
  pub style_index: Option<u32>,
  pub data_type: Option<CellDataType>,
  pub cached_value: Option<Cow<'doc, str>>,
  pub display_text: Cow<'doc, str>,
  pub rich_text_runs: Vec<XlsxRichTextRun<'doc>>,
  pub formula_state: Option<XlsxFormulaState>,
  pub horizontal_alignment: CellHorizontalAlignment,
  pub vertical_alignment: CellVerticalAlignment,
  pub wrap_text: bool,
  pub shrink_to_fit: bool,
  pub rotation_degrees: Option<i16>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CellHorizontalAlignment {
  #[default]
  General,
  Left,
  Center,
  Right,
  Fill,
  Justify,
  CenterContinuous,
  Distributed,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CellVerticalAlignment {
  Top,
  Center,
  #[default]
  Bottom,
  Justify,
  Distributed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CellDataType {
  Boolean,
  Number,
  Error,
  SharedString,
  String,
  InlineString,
  Date,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum XlsxFormulaState {
  Clean,
  CachedOnly,
  Stale,
  Unsupported,
  Error,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxRichTextRun<'doc> {
  pub text: Cow<'doc, str>,
  pub style: XlsxTextStyle<'doc>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxTextStyle<'doc> {
  pub font_family: Option<Cow<'doc, str>>,
  pub size: Option<Pt>,
  pub bold: bool,
  pub italic: bool,
  pub color: Option<crate::common::Color>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxStyleCatalog<'doc> {
  pub cell_formats: Vec<CellFormat<'doc>>,
  pub number_formats: Vec<NumberFormat<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CellFormat<'doc> {
  pub number_format_id: Option<u32>,
  pub text_style: XlsxTextStyle<'doc>,
  pub fill: Fill<'doc>,
  pub borders: CellBorders<'doc>,
  pub alignment: CellAlignment,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CellAlignment {
  pub horizontal: CellHorizontalAlignment,
  pub vertical: CellVerticalAlignment,
  pub wrap_text: bool,
  pub shrink_to_fit: bool,
  pub indent: u8,
  pub rotation_degrees: Option<i16>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumberFormat<'doc> {
  pub id: u32,
  pub code: Cow<'doc, str>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CellBorders<'doc> {
  pub top: Option<Stroke<'doc>>,
  pub right: Option<Stroke<'doc>>,
  pub bottom: Option<Stroke<'doc>>,
  pub left: Option<Stroke<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxPageStyle<'doc> {
  pub name: Option<Cow<'doc, str>>,
  pub setup: XlsxPageSetup<'doc>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hyperlink<'doc> {
  pub reference: CellRange,
  pub relationship_id: Option<Cow<'doc, str>>,
  pub location: Option<Cow<'doc, str>>,
  pub display: Option<Cow<'doc, str>>,
  pub tooltip: Option<Cow<'doc, str>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PageBreak {
  pub id: u32,
  pub min: u32,
  pub max: u32,
  pub manual: bool,
  pub pivot: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxDrawing<'doc> {
  pub anchors: Vec<DrawingAnchor<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DrawingAnchor<'doc> {
  pub kind: DrawingAnchorKind,
  pub from: CellAddress,
  pub to: Option<CellAddress>,
  pub bounds: Rect,
  pub relationship_id: Option<Cow<'doc, str>>,
  pub print_with_sheet: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DrawingAnchorKind {
  #[default]
  TwoCell,
  OneCell,
  Absolute,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XlsxTable<'doc> {
  pub name: Cow<'doc, str>,
  pub display_name: Cow<'doc, str>,
  pub range: CellRange,
  pub style_name: Option<Cow<'doc, str>>,
  pub show_header_row: bool,
  pub show_totals_row: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AutoFilter<'doc> {
  pub range: CellRange,
  pub filters: Vec<FilterColumn<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FilterColumn<'doc> {
  pub column_id: u32,
  pub values: Vec<Cow<'doc, str>>,
  pub hidden_button: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConditionalFormat<'doc> {
  pub ranges: Vec<CellRange>,
  pub priority: u32,
  pub kind: ConditionalFormatKind<'doc>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConditionalFormatKind<'doc> {
  Expression(Cow<'doc, str>),
  CellIs(Cow<'doc, str>),
  ColorScale,
  DataBar,
  IconSet,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XlsxNote<'doc> {
  pub cell: CellAddress,
  pub author: Option<Cow<'doc, str>>,
  pub text: Cow<'doc, str>,
  pub visible: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProtectedRange<'doc> {
  pub name: Option<Cow<'doc, str>>,
  pub range: CellRange,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxPrintPlan<'doc> {
  pub sheet_pages: Vec<XlsxPrintedSheet<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxPrintedSheet<'doc> {
  pub sheet_index: usize,
  pub sheet_name: Cow<'doc, str>,
  pub pages: Vec<XlsxPrintPage<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxPrintPage<'doc> {
  pub page_index: usize,
  pub sheet_range: CellRange,
  pub paper_bounds: Rect,
  pub content_bounds: Rect,
  pub cells: Vec<XlsxCellFragment<'doc>>,
  pub drawings: Vec<XlsxDrawingFragment<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XlsxCellFragment<'doc> {
  pub address: CellAddress,
  pub bounds: Rect,
  pub text: Cow<'doc, str>,
  pub style_index: Option<u32>,
  pub merged_range: Option<CellRange>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct XlsxDrawingFragment<'doc> {
  pub anchor: DrawingAnchor<'doc>,
  pub bounds: Rect,
}

pub fn build_print_plan<'doc>(workbook: &XlsxWorkbook<'doc>) -> XlsxPrintPlan<'doc> {
  XlsxPrintPlan {
    sheet_pages: workbook
      .sheets
      .iter()
      .enumerate()
      .filter(|(_, sheet)| sheet.state == SheetState::Visible)
      .filter_map(|(sheet_index, sheet)| printed_sheet(sheet_index, sheet))
      .collect(),
  }
}

fn printed_sheet<'doc>(
  sheet_index: usize,
  sheet: &XlsxSheet<'doc>,
) -> Option<XlsxPrintedSheet<'doc>> {
  let sheet_range = sheet_print_range(sheet)?;
  let content_bounds = sheet_range_bounds(sheet, sheet_range);
  Some(XlsxPrintedSheet {
    sheet_index,
    sheet_name: sheet.name.clone(),
    pages: vec![XlsxPrintPage {
      page_index: 0,
      sheet_range,
      paper_bounds: paper_bounds(sheet),
      content_bounds,
      cells: sheet_cell_fragments(sheet, sheet_range),
      drawings: sheet_drawing_fragments(sheet, sheet_range),
    }],
  })
}

fn sheet_print_range(sheet: &XlsxSheet<'_>) -> Option<CellRange> {
  sheet
    .metrics
    .print_ranges
    .first()
    .copied()
    .map(|range| extend_print_range_for_merges(sheet, range))
    .or_else(|| occupied_range(sheet))
}

fn occupied_range(sheet: &XlsxSheet<'_>) -> Option<CellRange> {
  let mut range = None;
  for (row_position, row) in sheet.rows.iter().enumerate() {
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    let mut current_column = 0u32;
    for cell in &row.cells {
      let address = cell.address.unwrap_or_else(|| {
        let address = CellAddress {
          column: current_column,
          row: row_index.saturating_sub(1),
        };
        current_column = current_column.saturating_add(1);
        address
      });
      current_column = address.column.saturating_add(1);
      extend_range(&mut range, address);
    }
  }
  for drawing_range in drawing_ranges(sheet) {
    extend_range(&mut range, drawing_range.start);
    extend_range(&mut range, drawing_range.end);
  }
  range.map(|range| {
    extend_print_range_for_merges(
      sheet,
      CellRange {
        start: CellAddress { column: 0, row: 0 },
        end: range.end,
      },
    )
  })
}

fn sheet_cell_fragments<'doc>(
  sheet: &XlsxSheet<'doc>,
  sheet_range: CellRange,
) -> Vec<XlsxCellFragment<'doc>> {
  let mut fragments = Vec::new();
  for (row_position, row) in sheet.rows.iter().enumerate() {
    if row.hidden {
      continue;
    }
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    let mut current_column = 0u32;
    for cell in &row.cells {
      let address = cell.address.unwrap_or_else(|| {
        let address = CellAddress {
          column: current_column,
          row: row_index.saturating_sub(1),
        };
        current_column = current_column.saturating_add(1);
        address
      });
      current_column = address.column.saturating_add(1);
      if !range_contains(sheet_range, address) {
        continue;
      }
      let merged_range = sheet
        .metrics
        .merged_ranges
        .iter()
        .copied()
        .find(|range| range_contains(*range, address));
      fragments.push(XlsxCellFragment {
        address,
        bounds: sheet_range_bounds(
          sheet,
          merged_range.unwrap_or(CellRange {
            start: address,
            end: address,
          }),
        ),
        text: cell.display_text.clone(),
        style_index: cell.style_index,
        merged_range,
      });
    }
  }
  fragments
}

fn sheet_drawing_fragments<'doc>(
  sheet: &XlsxSheet<'doc>,
  sheet_range: CellRange,
) -> Vec<XlsxDrawingFragment<'doc>> {
  sheet
    .drawings
    .iter()
    .flat_map(|drawing| drawing.anchors.iter())
    .filter(|anchor| anchor.print_with_sheet)
    .filter(|anchor| range_contains(sheet_range, anchor.from))
    .map(|anchor| XlsxDrawingFragment {
      anchor: anchor.clone(),
      bounds: anchor.bounds,
    })
    .collect()
}

fn extend_print_range_for_merges(sheet: &XlsxSheet<'_>, mut range: CellRange) -> CellRange {
  let old_end = range.end;
  for merged in &sheet.metrics.merged_ranges {
    if range_contains(
      CellRange {
        start: range.start,
        end: old_end,
      },
      merged.start,
    ) {
      range.end.column = range.end.column.max(merged.end.column);
      range.end.row = range.end.row.max(merged.end.row);
    }
  }
  range
}

fn drawing_ranges<'a, 'doc>(sheet: &'a XlsxSheet<'doc>) -> impl Iterator<Item = CellRange> + 'a {
  sheet
    .drawings
    .iter()
    .flat_map(|drawing| drawing.anchors.iter())
    .filter(|anchor| anchor.print_with_sheet)
    .map(|anchor| CellRange {
      start: anchor.from,
      end: anchor.to.unwrap_or(anchor.from),
    })
}

fn extend_range(range: &mut Option<CellRange>, address: CellAddress) {
  *range = Some(match *range {
    Some(existing) => CellRange {
      start: CellAddress {
        column: existing.start.column.min(address.column),
        row: existing.start.row.min(address.row),
      },
      end: CellAddress {
        column: existing.end.column.max(address.column),
        row: existing.end.row.max(address.row),
      },
    },
    None => CellRange {
      start: address,
      end: address,
    },
  });
}

fn range_contains(range: CellRange, address: CellAddress) -> bool {
  (range.start.column..=range.end.column).contains(&address.column)
    && (range.start.row..=range.end.row).contains(&address.row)
}

fn parse_layout_address(value: &str) -> Option<CellAddress> {
  ooxmlsdk_formula::QualifiedAddress::parse_a1(ooxmlsdk_formula::SheetId(0), value)
    .ok()
    .map(|address| CellAddress {
      column: address.cell.column,
      row: address.cell.row,
    })
}

fn parse_layout_range(value: &str) -> Option<CellRange> {
  let range =
    ooxmlsdk_formula::QualifiedRange::parse_a1(ooxmlsdk_formula::SheetId(0), value).ok()?;
  Some(CellRange {
    start: CellAddress {
      column: range.range.start.column,
      row: range.range.start.row,
    },
    end: CellAddress {
      column: range.range.end.column,
      row: range.range.end.row,
    },
  })
}

fn paper_bounds(sheet: &XlsxSheet<'_>) -> Rect {
  let (width, height) = match sheet.page_setup.orientation {
    PageOrientation::Portrait => (A4_PORTRAIT_WIDTH_PT, A4_PORTRAIT_HEIGHT_PT),
    PageOrientation::Landscape => (A4_PORTRAIT_HEIGHT_PT, A4_PORTRAIT_WIDTH_PT),
  };
  Rect {
    origin: Point {
      x: Pt(0.0),
      y: Pt(0.0),
    },
    size: Size {
      width: Pt(width),
      height: Pt(height),
    },
  }
}

fn sheet_range_bounds(sheet: &XlsxSheet<'_>, range: CellRange) -> Rect {
  let x = (0..range.start.column)
    .map(|column| column_width_pt(sheet, column))
    .sum::<f32>();
  let y = (0..range.start.row)
    .map(|row| row_height_pt(sheet, row))
    .sum::<f32>();
  let width = (range.start.column..=range.end.column)
    .map(|column| column_width_pt(sheet, column))
    .sum::<f32>();
  let height = (range.start.row..=range.end.row)
    .map(|row| row_height_pt(sheet, row))
    .sum::<f32>();
  Rect {
    origin: Point { x: Pt(x), y: Pt(y) },
    size: Size {
      width: Pt(width),
      height: Pt(height),
    },
  }
}

fn column_width_pt(sheet: &XlsxSheet<'_>, column: u32) -> f32 {
  let width_twips = sheet
    .metrics
    .columns
    .iter()
    .find(|model| (model.first..=model.last).contains(&column))
    .and_then(|model| model.width.or(sheet.metrics.format.default_column_width))
    .map(excel_column_width_to_twips)
    .unwrap_or(CALC_STANDARD_COLUMN_WIDTH_TWIPS);
  Twips(width_twips).to_pt().0.max(0.0)
}

fn row_height_pt(sheet: &XlsxSheet<'_>, row: u32) -> f32 {
  sheet
    .rows
    .iter()
    .find(|model| model.row_index == Some(row + 1))
    .and_then(|model| model.height)
    .unwrap_or(sheet.metrics.format.default_row_height)
    .max(0.0) as f32
}

fn excel_column_width_to_twips(width: f64) -> i32 {
  // OOXML stores width in character units. LibreOffice's default geometry is
  // twips-based; scale from the Calc standard width instead of using pixel
  // guesses so print bounds stay in the same unit family as LO.
  (width / 8.0 * CALC_STANDARD_COLUMN_WIDTH_TWIPS as f64).round() as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn builds_print_plan_from_visible_cells_and_print_range() {
    let workbook = XlsxWorkbook {
      sheets: vec![XlsxSheet {
        name: Cow::Borrowed("Sheet1"),
        metrics: SheetMetrics {
          print_ranges: vec![CellRange {
            start: CellAddress { column: 0, row: 0 },
            end: CellAddress { column: 0, row: 0 },
          }],
          merged_ranges: vec![CellRange {
            start: CellAddress { column: 0, row: 0 },
            end: CellAddress { column: 1, row: 0 },
          }],
          ..SheetMetrics::default()
        },
        rows: vec![
          XlsxRow {
            row_index: Some(1),
            cells: vec![
              XlsxCell {
                address: Some(CellAddress { column: 0, row: 0 }),
                display_text: Cow::Borrowed("A"),
                ..XlsxCell::default()
              },
              XlsxCell {
                address: Some(CellAddress { column: 2, row: 0 }),
                display_text: Cow::Borrowed("C"),
                ..XlsxCell::default()
              },
            ],
            ..XlsxRow::default()
          },
          XlsxRow {
            row_index: Some(2),
            hidden: true,
            cells: vec![XlsxCell {
              address: Some(CellAddress { column: 0, row: 1 }),
              display_text: Cow::Borrowed("hidden"),
              ..XlsxCell::default()
            }],
            ..XlsxRow::default()
          },
        ],
        ..XlsxSheet::default()
      }],
      ..XlsxWorkbook::default()
    };

    let plan = workbook.build_print_plan();

    assert_eq!(plan.sheet_pages.len(), 1);
    assert_eq!(plan.sheet_pages[0].pages.len(), 1);
    assert_eq!(
      plan.sheet_pages[0].pages[0].sheet_range,
      CellRange {
        start: CellAddress { column: 0, row: 0 },
        end: CellAddress { column: 1, row: 0 }
      }
    );
    assert_eq!(plan.sheet_pages[0].pages[0].cells.len(), 1);
    assert_eq!(plan.sheet_pages[0].pages[0].cells[0].text, "A");
    assert_eq!(
      plan.sheet_pages[0].pages[0].cells[0].merged_range,
      Some(CellRange {
        start: CellAddress { column: 0, row: 0 },
        end: CellAddress { column: 1, row: 0 }
      })
    );
  }

  #[test]
  fn applies_formula_defined_names_to_sheet_print_metrics() {
    let mut sheets = vec![XlsxSheet {
      name: Cow::Borrowed("Q1, North"),
      ..XlsxSheet::default()
    }];
    let value_model = ooxmlsdk_formula::WorkbookValueModel {
      defined_names: vec![
        ooxmlsdk_formula::DefinedName {
          name: Cow::Borrowed("_xlnm.Print_Area"),
          sheet: Some(SheetId(0)),
          formula_text: Cow::Borrowed("'Q1, North'!$B$2:$D$5"),
          hidden: false,
          built_in: Some(BuiltInName::PrintArea),
        },
        ooxmlsdk_formula::DefinedName {
          name: Cow::Borrowed("_xlnm.Print_Titles"),
          sheet: Some(SheetId(0)),
          formula_text: Cow::Borrowed("'Q1, North'!$1:$2,'Q1, North'!$A:$C"),
          hidden: false,
          built_in: Some(BuiltInName::PrintTitles),
        },
      ],
      ..ooxmlsdk_formula::WorkbookValueModel::default()
    };

    apply_defined_names_to_sheets(&mut sheets, &value_model);

    let expected_repeated_rows = layout_cell_range(
      FormulaQualifiedRange::parse_a1(SheetId(0), "$1:$2")
        .unwrap()
        .range,
    );
    let expected_repeated_columns = layout_cell_range(
      FormulaQualifiedRange::parse_a1(SheetId(0), "$A:$C")
        .unwrap()
        .range,
    );

    assert_eq!(
      sheets[0].metrics.print_ranges,
      vec![CellRange {
        start: CellAddress { column: 1, row: 1 },
        end: CellAddress { column: 3, row: 4 }
      }]
    );
    assert_eq!(
      sheets[0].metrics.repeated_rows,
      vec![expected_repeated_rows]
    );
    assert_eq!(
      sheets[0].metrics.repeated_columns,
      vec![expected_repeated_columns]
    );
  }

  #[test]
  fn implicit_print_range_starts_at_a1_and_includes_drawings() {
    let workbook = XlsxWorkbook {
      sheets: vec![XlsxSheet {
        name: Cow::Borrowed("Sheet1"),
        rows: vec![XlsxRow {
          row_index: Some(3),
          cells: vec![XlsxCell {
            address: Some(CellAddress { column: 2, row: 2 }),
            display_text: Cow::Borrowed("C3"),
            ..XlsxCell::default()
          }],
          ..XlsxRow::default()
        }],
        drawings: vec![XlsxDrawing {
          anchors: vec![DrawingAnchor {
            kind: DrawingAnchorKind::TwoCell,
            from: CellAddress { column: 4, row: 4 },
            to: Some(CellAddress { column: 5, row: 6 }),
            bounds: Rect::default(),
            relationship_id: Some(Cow::Borrowed("rId1")),
            print_with_sheet: true,
          }],
        }],
        ..XlsxSheet::default()
      }],
      ..XlsxWorkbook::default()
    };

    let plan = workbook.build_print_plan();

    assert_eq!(
      plan.sheet_pages[0].pages[0].sheet_range,
      CellRange {
        start: CellAddress { column: 0, row: 0 },
        end: CellAddress { column: 5, row: 6 }
      }
    );
  }
}
