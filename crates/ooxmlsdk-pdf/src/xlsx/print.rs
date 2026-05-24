use super::import::ExcelImport;
use super::page_settings::CalcPageSettings;
use super::styles::{DefinedNameBuiltin, DefinedNameRecord};
use super::worksheet::{CalcSheet, CellAddress, CellRange, SheetType};

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintDocument<'a> {
  pub(crate) pages: Vec<CalcPrintPage<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintPage<'a> {
  pub(crate) sheet: &'a CalcSheet,
  pub(crate) sheet_page_index: usize,
  pub(crate) page_number: usize,
  pub(crate) zoom: u32,
  pub(crate) page_settings: &'a CalcPageSettings,
  pub(crate) named_ranges: CalcPrintNamedRanges<'a>,
  pub(crate) area: Option<CellRange>,
  pub(crate) repeated_rows: Option<CellRange>,
  pub(crate) repeated_columns: Option<CellRange>,
  pub(crate) cells: Vec<CalcPrintCell<'a>>,
  pub(crate) hidden_rows: usize,
  pub(crate) hidden_columns: usize,
  pub(crate) merged_ranges: usize,
  pub(crate) explicit_print_area: bool,
  pub(crate) drawing_summary: CalcPrintDrawingSummary,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CalcPrintNamedRanges<'a> {
  pub(crate) print_areas: Vec<&'a DefinedNameRecord>,
  pub(crate) print_titles: Vec<&'a DefinedNameRecord>,
  pub(crate) filter_databases: Vec<&'a DefinedNameRecord>,
  pub(crate) resolved_print_areas: Vec<CellRange>,
  pub(crate) repeat_rows: Option<CellRange>,
  pub(crate) repeat_columns: Option<CellRange>,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintCell<'a> {
  pub(crate) address: CellAddress,
  pub(crate) text: &'a str,
  pub(crate) style_index: Option<u32>,
  pub(crate) number_format_id: Option<u32>,
  pub(crate) hidden_row: bool,
  pub(crate) hidden_column: bool,
  pub(crate) formula: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CalcPrintDrawingSummary {
  pub(crate) anchors: usize,
  pub(crate) pictures: usize,
  pub(crate) charts: usize,
  pub(crate) graphic_frames: usize,
  pub(crate) shapes: usize,
  pub(crate) groups: usize,
  pub(crate) connectors: usize,
  pub(crate) content_parts: usize,
  pub(crate) hidden: usize,
  pub(crate) printable: usize,
  pub(crate) text_len: usize,
}

impl<'a> CalcPrintDocument<'a> {
  pub(crate) fn from_import(import: &'a ExcelImport) -> Self {
    // Source: LibreOffice sc/source/ui/view/printfun.cxx
    // This is the first ScPrintFunc-shaped owner. Full range, break, and page
    // count logic lands here; display only consumes the resulting print pages.
    let mut pages = Vec::new();
    for sheet in import.sheets.iter().filter(|sheet| sheet.visible()) {
      let named_ranges = CalcPrintNamedRanges::from_import(import, sheet);
      let areas = print_areas_for_sheet(sheet, &named_ranges);
      let explicit_print_area = !named_ranges.resolved_print_areas.is_empty();
      let mut sheet_page_index = 0usize;
      for area in page_areas_for_sheet(sheet, &areas) {
        let cells = area
          .map(|area| print_cells_for_area(import, sheet, area))
          .unwrap_or_default();
        let hidden_rows = cells.iter().filter(|cell| cell.hidden_row).count();
        let hidden_columns = cells.iter().filter(|cell| cell.hidden_column).count();
        let merged_ranges = area.map_or(0, |area| {
          sheet
            .metrics
            .merged_ranges
            .iter()
            .filter_map(|reference| CellRange::parse_a1_range(reference))
            .filter(|merged| merged.intersects(area))
            .count()
        });
        pages.push(CalcPrintPage {
          sheet,
          sheet_page_index,
          page_number: pages.len() + 1,
          zoom: sheet.page_settings.scale,
          page_settings: &sheet.page_settings,
          repeated_rows: named_ranges.repeat_rows,
          repeated_columns: named_ranges.repeat_columns,
          named_ranges: named_ranges.clone(),
          area,
          cells,
          hidden_rows,
          hidden_columns,
          merged_ranges,
          explicit_print_area,
          drawing_summary: drawing_summary_for_area(sheet, area),
        });
        sheet_page_index += 1;
      }
    }
    Self { pages }
  }
}

fn drawing_summary_for_area(sheet: &CalcSheet, area: Option<CellRange>) -> CalcPrintDrawingSummary {
  let mut summary = CalcPrintDrawingSummary::default();
  for anchor in sheet
    .resources
    .drawings
    .iter()
    .flat_map(|drawing| drawing.anchors.iter())
  {
    if !anchor_belongs_to_area(anchor.from.as_ref(), area) {
      continue;
    }
    summary.anchors += 1;
    summary.printable += usize::from(anchor.print_with_sheet);
    summary.hidden += usize::from(anchor.object.hidden);
    summary.text_len += anchor.object.text_len
      + anchor.object.name.as_ref().map_or(0, |value| value.len())
      + anchor
        .object
        .description
        .as_ref()
        .map_or(0, |value| value.len());
    match anchor.object.kind {
      super::drawing::DrawingObjectKind::Shape => summary.shapes += 1,
      super::drawing::DrawingObjectKind::GroupShape => summary.groups += 1,
      super::drawing::DrawingObjectKind::GraphicFrame => summary.graphic_frames += 1,
      super::drawing::DrawingObjectKind::ConnectionShape => summary.connectors += 1,
      super::drawing::DrawingObjectKind::Picture => summary.pictures += 1,
      super::drawing::DrawingObjectKind::ContentPart => summary.content_parts += 1,
      super::drawing::DrawingObjectKind::Unknown => {}
    }
  }
  summary.charts = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.chart_count())
    .sum();
  summary
}

fn anchor_belongs_to_area(
  marker: Option<&super::drawing::DrawingMarkerModel>,
  area: Option<CellRange>,
) -> bool {
  match (marker, area) {
    (_, None) => true,
    (None, Some(_)) => true,
    (Some(marker), Some(area)) => {
      let col = u32::try_from(marker.column)
        .ok()
        .and_then(|col| col.checked_add(1));
      let row = u32::try_from(marker.row)
        .ok()
        .and_then(|row| row.checked_add(1));
      col
        .zip(row)
        .is_some_and(|(col, row)| area.contains(CellAddress { col, row }))
    }
  }
}

impl<'a> CalcPrintNamedRanges<'a> {
  fn from_import(import: &'a ExcelImport, sheet: &CalcSheet) -> Self {
    // Source: LibreOffice sc/source/filter/oox/defnamesbuffer.cxx
    // DefinedName::convertFormula extracts print areas, repeated titles, and
    // filter database ranges after the defined-name formula token model exists.
    // Keep the built-in defined names attached to the ScPrintFunc owner; range
    // extraction belongs here once the Calc formula parser lands.
    let print_areas = import
      .defined_names
      .records_for_sheet(sheet.workbook_index, DefinedNameBuiltin::PrintArea);
    let print_titles = import
      .defined_names
      .records_for_sheet(sheet.workbook_index, DefinedNameBuiltin::PrintTitles);
    let filter_databases = import
      .defined_names
      .records_for_sheet(sheet.workbook_index, DefinedNameBuiltin::FilterDatabase);
    let resolved_print_areas = print_areas
      .iter()
      .flat_map(|record| parse_defined_name_ranges(&record.formula))
      .collect();
    let (repeat_rows, repeat_columns) =
      print_titles
        .iter()
        .fold((None, None), |(rows, columns), record| {
          let rows = rows.or_else(|| parse_print_title_rows(&record.formula));
          let columns = columns.or_else(|| parse_print_title_columns(&record.formula));
          (rows, columns)
        });
    Self {
      print_areas,
      print_titles,
      filter_databases,
      resolved_print_areas,
      repeat_rows,
      repeat_columns,
    }
  }

  pub(crate) fn unresolved_formula_count(&self) -> usize {
    self
      .print_areas
      .iter()
      .chain(&self.print_titles)
      .chain(&self.filter_databases)
      .filter(|record| !record.formula.is_empty())
      .count()
  }
}

fn print_areas_for_sheet(
  sheet: &CalcSheet,
  named_ranges: &CalcPrintNamedRanges<'_>,
) -> Vec<CellRange> {
  if !named_ranges.resolved_print_areas.is_empty() {
    return named_ranges.resolved_print_areas.clone();
  }
  sheet.used_range().into_iter().collect()
}

fn page_areas_for_sheet(sheet: &CalcSheet, print_areas: &[CellRange]) -> Vec<Option<CellRange>> {
  if sheet.sheet_type == SheetType::Chartsheet {
    return vec![None];
  }
  let mut pages = Vec::new();
  for area in print_areas {
    let row_slices = split_range_by_breaks(*area, &sheet.metrics.row_breaks, true);
    for row_slice in row_slices {
      pages.extend(
        split_range_by_breaks(row_slice, &sheet.metrics.column_breaks, false)
          .into_iter()
          .map(Some),
      );
    }
  }
  if pages.is_empty() {
    pages.push(None);
  }
  pages
}

fn split_range_by_breaks(
  area: CellRange,
  breaks: &[super::worksheet::PageBreakModel],
  by_row: bool,
) -> Vec<CellRange> {
  let mut starts = vec![if by_row {
    area.start.row
  } else {
    area.start.col
  }];
  let end = if by_row { area.end.row } else { area.end.col };
  for page_break in breaks.iter().filter(|page_break| page_break.manual) {
    let id = page_break.id;
    if id > *starts.last().unwrap_or(&0) && id <= end {
      starts.push(id);
    }
  }
  starts
    .iter()
    .enumerate()
    .map(|(index, start)| {
      let slice_end = starts
        .get(index + 1)
        .map_or(end, |next| next.saturating_sub(1));
      if by_row {
        CellRange::new(
          CellAddress {
            col: area.start.col,
            row: *start,
          },
          CellAddress {
            col: area.end.col,
            row: slice_end,
          },
        )
      } else {
        CellRange::new(
          CellAddress {
            col: *start,
            row: area.start.row,
          },
          CellAddress {
            col: slice_end,
            row: area.end.row,
          },
        )
      }
    })
    .collect()
}

fn print_cells_for_area<'a>(
  import: &'a ExcelImport,
  sheet: &'a CalcSheet,
  area: CellRange,
) -> Vec<CalcPrintCell<'a>> {
  let mut cells = Vec::new();
  for (row_position, row) in sheet.rows.iter().enumerate() {
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    for (cell_position, cell) in row.cells.iter().enumerate() {
      let address = cell.address().unwrap_or(CellAddress {
        col: cell_position as u32 + 1,
        row: row_index,
      });
      if !area.contains(address) {
        continue;
      }
      cells.push(CalcPrintCell {
        address,
        text: cell.display_text.as_str(),
        style_index: cell.style_index,
        number_format_id: cell
          .style_index
          .and_then(|index| import.styles.cell_xfs.get(index as usize))
          .and_then(|format| format.number_format_id),
        hidden_row: row.hidden,
        hidden_column: column_hidden(sheet, address.col),
        formula: cell.formula.is_some(),
      });
    }
  }
  cells
}

fn column_hidden(sheet: &CalcSheet, col: u32) -> bool {
  sheet
    .metrics
    .columns
    .iter()
    .any(|column| column.hidden && col >= column.first && col <= column.last)
}

fn parse_defined_name_ranges(formula: &str) -> Vec<CellRange> {
  formula
    .split(',')
    .filter_map(|range| {
      let range = range.trim().replace('$', "");
      CellRange::parse_a1_range(&range)
    })
    .collect()
}

fn parse_print_title_rows(formula: &str) -> Option<CellRange> {
  formula
    .split(',')
    .find_map(|range| parse_row_or_column_title(range, true))
}

fn parse_print_title_columns(formula: &str) -> Option<CellRange> {
  formula
    .split(',')
    .find_map(|range| parse_row_or_column_title(range, false))
}

fn parse_row_or_column_title(range: &str, rows: bool) -> Option<CellRange> {
  let range = range
    .trim()
    .rsplit_once('!')
    .map_or(range.trim(), |(_, range)| range)
    .replace('$', "");
  let (start, end) = range.split_once(':')?;
  if rows {
    let start = start.parse::<u32>().ok()?;
    let end = end.parse::<u32>().ok()?;
    Some(CellRange::new(
      CellAddress { col: 1, row: start },
      CellAddress { col: 1, row: end },
    ))
  } else {
    let start = column_name_to_index(start)?;
    let end = column_name_to_index(end)?;
    Some(CellRange::new(
      CellAddress { col: start, row: 1 },
      CellAddress { col: end, row: 1 },
    ))
  }
}

fn column_name_to_index(value: &str) -> Option<u32> {
  let mut col = 0u32;
  for ch in value.chars() {
    if !ch.is_ascii_alphabetic() {
      return None;
    }
    col = col
      .saturating_mul(26)
      .saturating_add(ch.to_ascii_uppercase() as u32 - 'A' as u32 + 1);
  }
  (col > 0).then_some(col)
}
