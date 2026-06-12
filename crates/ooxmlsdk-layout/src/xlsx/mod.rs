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

  pub fn build_print_plan(&self) -> XlsxPrintPlan<'doc> {
    build_print_plan(self)
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
  Some(XlsxPrintedSheet {
    sheet_index,
    sheet_name: sheet.name.clone(),
    pages: vec![XlsxPrintPage {
      page_index: 0,
      sheet_range,
      paper_bounds: Rect::default(),
      content_bounds: Rect::default(),
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
      fragments.push(XlsxCellFragment {
        address,
        bounds: Rect::default(),
        text: cell.display_text.clone(),
        style_index: cell.style_index,
        merged_range: sheet
          .metrics
          .merged_ranges
          .iter()
          .copied()
          .find(|range| range_contains(*range, address)),
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
