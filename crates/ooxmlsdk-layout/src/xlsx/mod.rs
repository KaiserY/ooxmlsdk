use std::borrow::Cow;

use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

use crate::common::{Fill, Insets, Pt, Rect, Stroke};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct XlsxWorkbook<'doc> {
  pub sheets: Vec<XlsxSheet<'doc>>,
  pub styles: XlsxStyleCatalog<'doc>,
  pub page_styles: Vec<XlsxPageStyle<'doc>>,
  pub drawings: Vec<XlsxDrawing<'doc>>,
  pub print_plan: XlsxPrintPlan<'doc>,
  #[cfg(feature = "xlsx-formula")]
  pub value_model: Option<&'doc ooxmlsdk_formula::WorkbookValueModel<'doc>>,
}

impl<'doc> XlsxWorkbook<'doc> {
  pub fn from_spreadsheet_document(_document: &'doc SpreadsheetDocument) -> Self {
    Self::default()
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
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SheetFormat {
  pub base_column_width: Option<u32>,
  pub default_column_width: Option<f64>,
  pub default_row_height: f64,
  pub custom_height: bool,
  pub zero_height: bool,
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
