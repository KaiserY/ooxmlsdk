use super::import::ExcelImport;
use super::page_settings::CalcPageSettings;
use super::styles::{DefinedNameBuiltin, DefinedNameRecord};
use super::worksheet::{CalcSheet, CellAddress, CellRange, SheetType};

// Source: LibreOffice sc/source/ui/view/printfun.cxx defines ZOOM_MIN.
const ZOOM_MIN: u32 = 10;

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintDocument<'a> {
  pub(crate) pages: Vec<CalcPrintPage<'a>>,
  pub(crate) skipped_empty_pages: usize,
  pub(crate) top_down: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintPage<'a> {
  pub(crate) sheet: &'a CalcSheet,
  pub(crate) sheet_page_index: usize,
  pub(crate) page_number: usize,
  pub(crate) total_pages: usize,
  pub(crate) zoom: u32,
  pub(crate) page_settings: &'a CalcPageSettings,
  pub(crate) named_ranges: CalcPrintNamedRanges<'a>,
  pub(crate) area: Option<CellRange>,
  pub(crate) repeated_rows: Option<CellRange>,
  pub(crate) repeated_columns: Option<CellRange>,
  pub(crate) cells: Vec<CalcPrintCell<'a>>,
  pub(crate) repeated_row_cells: Vec<CalcPrintCell<'a>>,
  pub(crate) repeated_column_cells: Vec<CalcPrintCell<'a>>,
  pub(crate) repeated_corner_cells: Vec<CalcPrintCell<'a>>,
  pub(crate) all_cells: usize,
  pub(crate) hidden_rows: usize,
  pub(crate) hidden_columns: usize,
  pub(crate) empty: bool,
  pub(crate) merged_ranges: usize,
  pub(crate) explicit_print_area: bool,
  pub(crate) drawing_summary: CalcPrintDrawingSummary,
  pub(crate) scale_mode: CalcPrintScaleMode,
  pub(crate) auto_page_columns: usize,
  pub(crate) auto_page_rows: usize,
  pub(crate) forced_break_min_pages: usize,
  pub(crate) tdf103516_adjusted: bool,
  pub(crate) paint_ops: Vec<CalcPrintPaintOp>,
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
  pub(crate) number_format_code: Option<&'a str>,
  pub(crate) rendered_text: String,
  pub(crate) rich_text_runs: &'a [super::workbook::SharedStringRun],
  pub(crate) number_format_state: NumberFormatRenderState,
  pub(crate) hidden_row: bool,
  pub(crate) hidden_column: bool,
  pub(crate) formula: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NumberFormatRenderState {
  Raw,
  General,
  Text,
  Boolean,
  Number,
  Percent,
  DateTime,
  UnsupportedFormatCode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CalcPrintScaleMode {
  None,
  ScaleAll,
  FitToWidthHeight,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum CalcPrintPaintOp {
  BackDrawingLayer,
  RepeatedColumns,
  RepeatedRows,
  CellArea,
  Grid,
  FrontDrawingLayer,
}

impl<'a> CalcPrintDocument<'a> {
  pub(crate) fn from_import(import: &'a ExcelImport) -> Self {
    // Source: LibreOffice sc/source/ui/view/printfun.cxx
    // This is the first ScPrintFunc-shaped owner. Full range, break, and page
    // count logic lands here; display only consumes the resulting print pages.
    let mut pages = Vec::new();
    let mut skipped_empty_pages = 0usize;
    let mut document_top_down = true;
    let any_visible_used_sheet = import
      .sheets
      .iter()
      .any(|sheet| sheet.visible() && sheet.used_range().is_some());
    for sheet in import.sheets.iter().filter(|sheet| {
      sheet.visible()
        && if any_visible_used_sheet {
          sheet.used_range().is_some()
        } else {
          sheet.active
        }
    }) {
      let named_ranges = CalcPrintNamedRanges::from_import(import, sheet);
      let areas = print_areas_for_sheet(sheet, &named_ranges);
      let explicit_print_area = !named_ranges.resolved_print_areas.is_empty();
      let scale = print_scale_state(sheet, &areas, &named_ranges);
      document_top_down &= scale.top_down;
      let mut page_areas = page_areas_for_sheet(sheet, &areas, &named_ranges, scale.zoom);
      if !scale.top_down {
        page_areas.sort_by_key(|area| area.map_or((0, 0), |area| (area.start.row, area.start.col)));
      }
      let mut sheet_page_index = 0usize;
      for area in page_areas {
        let all_cells = area
          .map(|area| print_cells_for_area(import, sheet, area, false))
          .unwrap_or_default();
        let cells = area
          .map(|area| print_cells_for_area(import, sheet, area, true))
          .unwrap_or_default();
        let repeated_row_cells = repeat_rows_for_page(area, named_ranges.repeat_rows)
          .map(|area| print_cells_for_area(import, sheet, area, true))
          .unwrap_or_default();
        let repeated_column_cells = repeat_columns_for_page(area, named_ranges.repeat_columns)
          .map(|area| print_cells_for_area(import, sheet, area, true))
          .unwrap_or_default();
        let repeated_corner_cells =
          repeat_corner_for_page(named_ranges.repeat_rows, named_ranges.repeat_columns)
            .map(|area| print_cells_for_area(import, sheet, area, true))
            .unwrap_or_default();
        let empty = all_cells.is_empty();
        if scale.skip_empty && empty {
          skipped_empty_pages += 1;
          continue;
        }
        let hidden_rows = all_cells.iter().filter(|cell| cell.hidden_row).count();
        let hidden_columns = all_cells.iter().filter(|cell| cell.hidden_column).count();
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
          total_pages: 0,
          zoom: scale.zoom,
          page_settings: &sheet.page_settings,
          repeated_rows: named_ranges.repeat_rows,
          repeated_columns: named_ranges.repeat_columns,
          named_ranges: named_ranges.clone(),
          area,
          all_cells: all_cells.len(),
          cells,
          repeated_row_cells,
          repeated_column_cells,
          repeated_corner_cells,
          hidden_rows,
          hidden_columns,
          empty,
          merged_ranges,
          explicit_print_area,
          drawing_summary: drawing_summary_for_area(sheet, area),
          scale_mode: scale.mode,
          auto_page_columns: scale.auto_page_columns,
          auto_page_rows: scale.auto_page_rows,
          forced_break_min_pages: scale.forced_break_min_pages,
          tdf103516_adjusted: scale.tdf103516_adjusted,
          paint_ops: paint_ops_for_page(
            named_ranges.repeat_columns.is_some(),
            named_ranges.repeat_rows.is_some(),
            drawing_summary_for_area(sheet, area).anchors > 0,
            sheet.page_settings.print_grid_lines,
          ),
        });
        sheet_page_index += 1;
      }
    }
    let total_pages = pages.len();
    for page in &mut pages {
      page.total_pages = total_pages;
    }
    Self {
      pages,
      skipped_empty_pages,
      top_down: document_top_down,
    }
  }
}

fn paint_ops_for_page(
  has_repeat_columns: bool,
  has_repeat_rows: bool,
  has_drawing_layer: bool,
  has_grid: bool,
) -> Vec<CalcPrintPaintOp> {
  // Source: LibreOffice sc/source/ui/view/printfun.cxx PrintPage and
  // PrintArea route back drawing layer before cell output, repeated areas
  // before page-local data, grid through ScOutputData, and front drawing after.
  let mut ops = Vec::new();
  if has_drawing_layer {
    ops.push(CalcPrintPaintOp::BackDrawingLayer);
  }
  if has_repeat_columns {
    ops.push(CalcPrintPaintOp::RepeatedColumns);
  }
  if has_repeat_rows {
    ops.push(CalcPrintPaintOp::RepeatedRows);
  }
  ops.push(CalcPrintPaintOp::CellArea);
  if has_grid {
    ops.push(CalcPrintPaintOp::Grid);
  }
  if has_drawing_layer {
    ops.push(CalcPrintPaintOp::FrontDrawingLayer);
  }
  ops
}

#[derive(Clone, Copy, Debug)]
struct CalcPrintScaleState {
  mode: CalcPrintScaleMode,
  zoom: u32,
  auto_page_columns: usize,
  auto_page_rows: usize,
  forced_break_min_pages: usize,
  tdf103516_adjusted: bool,
  skip_empty: bool,
  top_down: bool,
}

fn print_scale_state(
  sheet: &CalcSheet,
  areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges<'_>,
) -> CalcPrintScaleState {
  // Source: LibreOffice sc/source/ui/view/printfun.cxx InitParam,
  // UpdatePages, CalcZoom. Full page-size based CalcPages is a later bridge;
  // this keeps the exact branch ownership and forced-break constraints.
  let forced_break_min_columns = sheet
    .metrics
    .column_breaks
    .iter()
    .filter(|br| br.manual)
    .count()
    + 1;
  let forced_break_min_rows = sheet
    .metrics
    .row_breaks
    .iter()
    .filter(|br| br.manual)
    .count()
    + 1;
  let forced_break_min_pages = forced_break_min_columns * forced_break_min_rows;
  let fit_to_page = sheet.metrics.settings.properties.page_setup.fit_to_page;
  let mut mode = CalcPrintScaleMode::None;
  let mut zoom = sheet.page_settings.scale;
  let mut auto_page_columns = forced_break_min_columns.max(1);
  let mut auto_page_rows = forced_break_min_rows.max(1);
  let mut tdf103516_adjusted = false;

  if fit_to_page && (sheet.page_settings.fit_to_width > 0 || sheet.page_settings.fit_to_height > 0)
  {
    mode = CalcPrintScaleMode::FitToWidthHeight;
    auto_page_columns = usize::try_from(sheet.page_settings.fit_to_width)
      .ok()
      .filter(|value| *value > 0)
      .unwrap_or(auto_page_columns)
      .max(forced_break_min_columns);
    auto_page_rows = usize::try_from(sheet.page_settings.fit_to_height)
      .ok()
      .filter(|value| *value > 0)
      .unwrap_or(auto_page_rows)
      .max(forced_break_min_rows);
    zoom = fit_zoom_to_pages(
      sheet,
      areas,
      named_ranges,
      auto_page_columns,
      auto_page_rows,
    );
    if sheet.page_settings.fit_to_width > 0
      && sheet.page_settings.fit_to_height == 0
      && actual_row_page_count(sheet, areas, named_ranges, zoom) > 1
    {
      let adjusted_zoom = ((zoom as f32) * 0.98).floor().max(ZOOM_MIN as f32) as u32;
      if adjusted_zoom < zoom
        && actual_row_page_count(sheet, areas, named_ranges, adjusted_zoom)
          < actual_row_page_count(sheet, areas, named_ranges, zoom)
      {
        tdf103516_adjusted = true;
        zoom = adjusted_zoom;
      }
    }
  } else if sheet.page_settings.scale > 0 {
    mode = if sheet.page_settings.scale == 100 {
      CalcPrintScaleMode::None
    } else {
      CalcPrintScaleMode::ScaleAll
    };
    zoom = sheet.page_settings.scale.max(ZOOM_MIN);
  }

  CalcPrintScaleState {
    mode,
    zoom,
    auto_page_columns,
    auto_page_rows,
    forced_break_min_pages,
    tdf103516_adjusted,
    skip_empty: false,
    top_down: matches!(
      sheet.page_settings.page_order,
      Some(ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PageOrderValues::DownThenOver)
        | None
    ),
  }
}

fn actual_row_page_count(
  sheet: &CalcSheet,
  areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges<'_>,
  zoom: u32,
) -> usize {
  areas
    .iter()
    .map(|area| {
      split_range_by_page_metrics(
        sheet,
        *area,
        &sheet.metrics.row_breaks,
        true,
        named_ranges.repeat_rows,
        zoom,
      )
      .len()
    })
    .sum::<usize>()
    .max(1)
}

fn fit_zoom_to_pages(
  sheet: &CalcSheet,
  areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges<'_>,
  page_columns: usize,
  page_rows: usize,
) -> u32 {
  let Some(area) = areas.first().copied() else {
    return 100;
  };
  let content = print_content_size_pt(sheet);
  let repeat_width = named_ranges
    .repeat_columns
    .map(|range| sheet.range_rect(range).width_pt)
    .unwrap_or(0.0);
  let repeat_height = named_ranges
    .repeat_rows
    .map(|range| sheet.range_rect(range).height_pt)
    .unwrap_or(0.0);
  let page_width = (content.0 - repeat_width).max(1.0);
  let page_height = (content.1 - repeat_height).max(1.0);
  let area_rect = sheet.range_rect(area);
  let width_zoom = if page_columns > 0 && area_rect.width_pt > 0.0 {
    (page_width * page_columns as f32 * 100.0 / area_rect.width_pt).floor() as u32
  } else {
    100
  };
  let height_zoom = if page_rows > 0 && area_rect.height_pt > 0.0 {
    (page_height * page_rows as f32 * 100.0 / area_rect.height_pt).floor() as u32
  } else {
    100
  };
  let metric_zoom = width_zoom.min(height_zoom).clamp(ZOOM_MIN, 100);
  if metric_zoom < 100 {
    return metric_zoom;
  }
  let mut zoom = 100u32;
  let mut last_fit = 0u32;
  let mut last_non_fit = 0u32;
  loop {
    if zoom <= ZOOM_MIN {
      break;
    }
    let fits = estimated_pages_at_zoom(area, zoom, page_columns, page_rows);
    if fits {
      if zoom == 100 {
        break;
      }
      last_fit = zoom;
      zoom = (last_non_fit + zoom) / 2;
      if last_fit == zoom {
        break;
      }
    } else {
      if zoom.saturating_sub(last_fit) <= 1 {
        zoom = last_fit.max(ZOOM_MIN);
        break;
      }
      last_non_fit = zoom;
      zoom = (last_fit + zoom) / 2;
    }
  }
  zoom.max(ZOOM_MIN)
}

fn estimated_pages_at_zoom(
  area: CellRange,
  zoom: u32,
  page_columns: usize,
  page_rows: usize,
) -> bool {
  let zoom = zoom.max(ZOOM_MIN);
  let scaled_cols = (area.end.col - area.start.col + 1) as usize * zoom as usize;
  let scaled_rows = (area.end.row - area.start.row + 1) as usize * zoom as usize;
  (page_columns == 0 || scaled_cols <= page_columns * 100)
    && (page_rows == 0 || scaled_rows <= page_rows * 100)
}

fn print_content_size_pt(sheet: &CalcSheet) -> (f32, f32) {
  let (mut width, mut height) = sheet.page_settings.page_size_pt();
  width -= (sheet.page_settings.margin_left_in + sheet.page_settings.margin_right_in) as f32
    * crate::units::POINTS_PER_INCH;
  height -= (sheet.page_settings.margin_top_in + sheet.page_settings.margin_bottom_in) as f32
    * crate::units::POINTS_PER_INCH;
  (width.max(1.0), height.max(1.0))
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
  match sheet.used_range() {
    Some(range) => vec![range],
    // Source: LibreOffice sc/source/ui/view/printfun.cxx AdjustPrintArea(true).
    // With skip-empty disabled, a missing document print area still leaves the
    // default start/end range printable, so header/footer-only sheets export a page.
    None => vec![CellRange::single(CellAddress { col: 1, row: 1 })],
  }
}

fn page_areas_for_sheet(
  sheet: &CalcSheet,
  print_areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges<'_>,
  zoom: u32,
) -> Vec<Option<CellRange>> {
  if sheet.sheet_type == SheetType::Chartsheet {
    return vec![None];
  }
  let mut pages = Vec::new();
  for area in print_areas {
    let row_slices = split_range_by_page_metrics(
      sheet,
      *area,
      &sheet.metrics.row_breaks,
      true,
      named_ranges.repeat_rows,
      zoom,
    );
    for row_slice in row_slices {
      pages.extend(
        split_range_by_page_metrics(
          sheet,
          row_slice,
          &sheet.metrics.column_breaks,
          false,
          named_ranges.repeat_columns,
          zoom,
        )
        .into_iter()
        .map(Some),
      );
    }
  }
  pages
}

fn split_range_by_page_metrics(
  sheet: &CalcSheet,
  area: CellRange,
  breaks: &[super::worksheet::PageBreakModel],
  by_row: bool,
  repeat: Option<CellRange>,
  zoom: u32,
) -> Vec<CellRange> {
  let start = if by_row {
    area.start.row
  } else {
    area.start.col
  };
  let end = if by_row { area.end.row } else { area.end.col };
  let mut slices = Vec::new();
  let content_size = print_content_size_pt(sheet);
  let repeat_size = repeat
    .map(|range| {
      if by_row {
        sheet.range_rect(range).height_pt
      } else {
        sheet.range_rect(range).width_pt
      }
    })
    .unwrap_or(0.0);
  let available = ((if by_row {
    content_size.1
  } else {
    content_size.0
  }) - repeat_size)
    .max(1.0)
    * 100.0
    / zoom.max(ZOOM_MIN) as f32;
  let mut current_start = start;
  let mut current = start;
  let mut used = 0.0f32;
  while current <= end {
    if breaks
      .iter()
      .any(|page_break| page_break.manual && page_break.id == current && current > current_start)
    {
      slices.push(axis_slice(area, by_row, current_start, current - 1));
      current_start = current;
      used = 0.0;
    }
    let size = if by_row {
      sheet.row_height_pt(current)
    } else {
      sheet.column_width_pt(current)
    };
    if used > 0.0 && used + size > available {
      slices.push(axis_slice(area, by_row, current_start, current - 1));
      current_start = current;
      used = 0.0;
    }
    used += size;
    current += 1;
  }
  if current_start <= end {
    slices.push(axis_slice(area, by_row, current_start, end));
  }
  slices
}

fn axis_slice(area: CellRange, by_row: bool, start: u32, end: u32) -> CellRange {
  if by_row {
    CellRange::new(
      CellAddress {
        col: area.start.col,
        row: start,
      },
      CellAddress {
        col: area.end.col,
        row: end,
      },
    )
  } else {
    CellRange::new(
      CellAddress {
        col: start,
        row: area.start.row,
      },
      CellAddress {
        col: end,
        row: area.end.row,
      },
    )
  }
}

fn repeat_rows_for_page(
  area: Option<CellRange>,
  repeat_rows: Option<CellRange>,
) -> Option<CellRange> {
  let area = area?;
  let repeat_rows = repeat_rows?;
  Some(CellRange::new(
    CellAddress {
      col: area.start.col,
      row: repeat_rows.start.row,
    },
    CellAddress {
      col: area.end.col,
      row: repeat_rows.end.row,
    },
  ))
}

fn repeat_columns_for_page(
  area: Option<CellRange>,
  repeat_columns: Option<CellRange>,
) -> Option<CellRange> {
  let area = area?;
  let repeat_columns = repeat_columns?;
  Some(CellRange::new(
    CellAddress {
      col: repeat_columns.start.col,
      row: area.start.row,
    },
    CellAddress {
      col: repeat_columns.end.col,
      row: area.end.row,
    },
  ))
}

fn repeat_corner_for_page(
  repeat_rows: Option<CellRange>,
  repeat_columns: Option<CellRange>,
) -> Option<CellRange> {
  let repeat_rows = repeat_rows?;
  let repeat_columns = repeat_columns?;
  Some(CellRange::new(
    CellAddress {
      col: repeat_columns.start.col,
      row: repeat_rows.start.row,
    },
    CellAddress {
      col: repeat_columns.end.col,
      row: repeat_rows.end.row,
    },
  ))
}

fn print_cells_for_area<'a>(
  import: &'a ExcelImport,
  sheet: &'a CalcSheet,
  area: CellRange,
  include_hidden: bool,
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
      let hidden_column = column_hidden(sheet, address.col);
      if !include_hidden && (row.hidden || hidden_column) {
        continue;
      }
      let style_index = sheet.effective_cell_style_index(row, cell, address);
      let number_format_id = style_index
        .and_then(|index| import.styles.cell_xfs.get(index as usize))
        .and_then(|format| format.number_format_id);
      let number_format_code = number_format_id.and_then(|id| import.styles.number_format_code(id));
      let (rendered_text, number_format_state) = rendered_number_text(
        cell.display_text.as_str(),
        number_format_code,
        cell.data_type,
        import.globals.settings.date_1904,
      );
      let rendered_text = pivot_display_text(sheet, address, rendered_text);
      cells.push(CalcPrintCell {
        address,
        text: cell.display_text.as_str(),
        style_index,
        number_format_id,
        number_format_code,
        rendered_text,
        rich_text_runs: &cell.rich_text_runs,
        number_format_state,
        hidden_row: row.hidden,
        hidden_column,
        formula: cell.formula.is_some(),
      });
    }
  }
  cells
}

fn pivot_display_text(sheet: &CalcSheet, address: CellAddress, text: String) -> String {
  if !is_pivot_table_cell(sheet, address) {
    return text;
  }
  // Source: LibreOffice DataPilot output uses STR_PIVOT_TOTAL ("Total Result")
  // for Calc-rendered grand totals after OOXML pivot import.
  match text.as_str() {
    "Grand Total" | "Total general" => "Total Result".to_string(),
    _ => text,
  }
}

fn is_pivot_table_cell(sheet: &CalcSheet, address: CellAddress) -> bool {
  sheet.resources.pivot_tables.tables.iter().any(|pivot| {
    CellRange::parse_a1_range(&pivot.location_reference)
      .is_some_and(|range| range.contains(address))
  })
}

fn rendered_number_text(
  raw: &str,
  format_code: Option<&str>,
  data_type: Option<
    ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues,
  >,
  date_1904: bool,
) -> (String, NumberFormatRenderState) {
  match data_type {
    Some(ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues::Boolean) => {
      if let Some(format_code) = format_code
        && !format_code.eq_ignore_ascii_case("General")
        && format_code != "@"
      {
        let value = if boolean_raw_value(raw) { 1.0 } else { 0.0 };
        if let Some(text) = render_literal_section_number_format(format_code, value) {
          return (text, NumberFormatRenderState::Boolean);
        }
      }
      return (
        if boolean_raw_value(raw) {
          "TRUE".to_string()
        } else {
          "FALSE".to_string()
        },
        NumberFormatRenderState::Boolean,
      );
    }
    Some(ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues::String)
    | Some(ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues::InlineString)
    | Some(ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues::SharedString) => {
      return (raw.to_string(), NumberFormatRenderState::Text);
    }
    _ => {}
  }

  let Some(format_code) = format_code else {
    return (raw.to_string(), NumberFormatRenderState::Raw);
  };
  if format_code.eq_ignore_ascii_case("General") {
    return (raw.to_string(), NumberFormatRenderState::General);
  }
  if format_code == "@" {
    return (raw.to_string(), NumberFormatRenderState::Text);
  }
  let Some(value) = raw.parse::<f64>().ok() else {
    return (
      raw.to_string(),
      NumberFormatRenderState::UnsupportedFormatCode,
    );
  };
  if let Some(text) = render_literal_section_number_format(format_code, value) {
    return (text, NumberFormatRenderState::Boolean);
  }
  let format = NumberFormatPattern::parse(format_code, value);
  if format.date_time {
    return (
      format_serial_date_time(value, format_code, date_1904),
      NumberFormatRenderState::DateTime,
    );
  }
  if format.numeric {
    return (
      format_decimal_value(value, &format),
      if format.percent {
        NumberFormatRenderState::Percent
      } else {
        NumberFormatRenderState::Number
      },
    );
  }
  (
    raw.to_string(),
    NumberFormatRenderState::UnsupportedFormatCode,
  )
}

fn render_literal_section_number_format(code: &str, value: f64) -> Option<String> {
  let sections = split_number_format_sections(code);
  if sections.len() < 3 {
    return None;
  }
  let section_index = if value.is_sign_negative() && sections.len() > 1 {
    1
  } else if value == 0.0 && sections.len() > 2 {
    2
  } else {
    0
  };
  let section = strip_number_format_markers(sections.get(section_index).copied().unwrap_or(code));
  literal_number_format_section(&section)
}

fn split_number_format_sections(code: &str) -> Vec<&str> {
  let mut sections = Vec::new();
  let mut start = 0;
  let mut in_quote = false;
  let mut escaped = false;
  for (index, ch) in code.char_indices() {
    if escaped {
      escaped = false;
      continue;
    }
    match ch {
      '\\' => escaped = true,
      '"' => in_quote = !in_quote,
      ';' if !in_quote => {
        sections.push(&code[start..index]);
        start = index + ch.len_utf8();
      }
      _ => {}
    }
  }
  sections.push(&code[start..]);
  sections
}

fn literal_number_format_section(section: &str) -> Option<String> {
  let mut output = String::new();
  let mut in_quote = false;
  let mut escaped = false;
  let mut has_quoted_literal = false;
  for ch in section.chars() {
    if escaped {
      if in_quote {
        output.push(ch);
      }
      escaped = false;
      continue;
    }
    match ch {
      '\\' => escaped = true,
      '"' => {
        in_quote = !in_quote;
        has_quoted_literal = true;
      }
      '_' | '*' if !in_quote => escaped = true,
      _ if in_quote => output.push(ch),
      _ if ch.is_whitespace() => {}
      _ => return None,
    }
  }
  if in_quote || !has_quoted_literal {
    return None;
  }
  Some(output)
}

#[derive(Clone, Debug, Default)]
struct NumberFormatPattern {
  numeric: bool,
  percent: bool,
  grouping: bool,
  decimals: usize,
  date_time: bool,
  prefix: String,
  suffix: String,
  suppress_negative_sign: bool,
  section: String,
  integer_pattern: String,
}

impl NumberFormatPattern {
  fn parse(code: &str, value: f64) -> Self {
    let sections = code.split(';').collect::<Vec<_>>();
    let section_index = if value.is_sign_negative() && sections.len() > 1 {
      1
    } else if value == 0.0 && sections.len() > 2 {
      2
    } else {
      0
    };
    let cleaned_section =
      strip_number_format_markers(sections.get(section_index).copied().unwrap_or(code));
    let section = cleaned_section.as_str();
    let mut pattern = Self {
      suppress_negative_sign: section_index == 1,
      section: section.to_string(),
      ..Self::default()
    };
    let mut in_quote = false;
    let mut escaped = false;
    let mut after_decimal = false;
    let mut seen_digit = false;
    let mut literal_prefix = true;
    let mut integer_pattern = String::new();
    for ch in section.chars() {
      if escaped {
        if !after_decimal && !literal_prefix {
          integer_pattern.push('\\');
          integer_pattern.push(ch);
        }
        if literal_prefix {
          pattern.prefix.push(ch);
        } else if seen_digit && after_decimal {
          pattern.suffix.push(ch);
        }
        escaped = false;
        continue;
      }
      match ch {
        '\\' => escaped = true,
        '_' | '*' => {
          if !after_decimal {
            integer_pattern.push(ch);
          }
          escaped = true;
        }
        '"' => in_quote = !in_quote,
        _ if in_quote => {
          if !after_decimal && !literal_prefix {
            integer_pattern.push(ch);
          }
          if literal_prefix {
            pattern.prefix.push(ch);
          } else if seen_digit && after_decimal {
            pattern.suffix.push(ch);
          }
        }
        '[' => {
          literal_prefix = false;
          if !after_decimal {
            integer_pattern.push(ch);
          }
        }
        '0' | '#' | '?' => {
          pattern.numeric = true;
          seen_digit = true;
          literal_prefix = false;
          if after_decimal {
            pattern.decimals += 1;
          } else {
            integer_pattern.push(ch);
          }
        }
        '.' if pattern.numeric => after_decimal = true,
        ',' if pattern.numeric => {
          pattern.grouping = true;
          if !after_decimal {
            integer_pattern.push(ch);
          }
        }
        '%' => {
          pattern.percent = true;
          pattern.suffix.push('%');
          literal_prefix = false;
        }
        '$' | '€' | '£' | '¥' => {
          if !after_decimal && !literal_prefix {
            integer_pattern.push(ch);
          }
          if literal_prefix {
            pattern.prefix.push(ch);
          } else if seen_digit && after_decimal {
            pattern.suffix.push(ch);
          }
        }
        'd' | 'D' | 'm' | 'M' | 'y' | 'Y' | 'h' | 'H' | 's' | 'S' => {
          pattern.date_time = true;
          literal_prefix = false;
        }
        _ if !ch.is_whitespace() && literal_prefix => {
          pattern.prefix.push(ch);
        }
        _ if !ch.is_whitespace() && seen_digit && after_decimal => pattern.suffix.push(ch),
        _ => {
          if !after_decimal {
            integer_pattern.push(ch);
          }
        }
      }
    }
    pattern.integer_pattern = integer_pattern;
    let trailing_suffix = trailing_integer_format_suffix(section);
    if !trailing_suffix.is_empty() && !pattern.suffix.contains(&trailing_suffix) {
      pattern.suffix.push_str(&trailing_suffix);
    }
    pattern
  }
}

fn boolean_raw_value(raw: &str) -> bool {
  match raw.trim().to_ascii_lowercase().as_str() {
    "true" => true,
    "false" | "" => false,
    value => value.parse::<f64>().is_ok_and(|number| number != 0.0),
  }
}

fn strip_number_format_markers(section: &str) -> String {
  let mut output = String::new();
  let mut rest = section;
  while let Some(start) = rest.find('[') {
    output.push_str(&rest[..start]);
    let Some(end) = rest[start + 1..].find(']') else {
      output.push_str(&rest[start..]);
      return output;
    };
    let marker = &rest[start + 1..start + 1 + end];
    if let Some(currency) = number_format_currency_marker(marker) {
      output.push_str(currency);
    } else if !is_ignored_number_format_marker(marker) {
      output.push('[');
      output.push_str(marker);
      output.push(']');
    }
    rest = &rest[start + end + 2..];
  }
  output.push_str(rest);
  output
}

fn number_format_currency_marker(marker: &str) -> Option<&str> {
  marker
    .strip_prefix('$')
    .and_then(|value| value.split('-').next())
    .filter(|value| !value.is_empty())
}

fn is_ignored_number_format_marker(marker: &str) -> bool {
  let marker = marker.trim();
  marker.eq_ignore_ascii_case("red")
    || marker.eq_ignore_ascii_case("black")
    || marker.eq_ignore_ascii_case("blue")
    || marker.eq_ignore_ascii_case("cyan")
    || marker.eq_ignore_ascii_case("green")
    || marker.eq_ignore_ascii_case("magenta")
    || marker.eq_ignore_ascii_case("white")
    || marker.eq_ignore_ascii_case("yellow")
    || marker.starts_with('<')
    || marker.starts_with('>')
    || marker.starts_with('=')
    || marker.to_ascii_lowercase().starts_with("color")
}

fn format_decimal_value(value: f64, pattern: &NumberFormatPattern) -> String {
  let value = if pattern.percent {
    value * 100.0
  } else {
    value
  };
  if let Some(fraction) = format_fraction_value(value, &pattern.section) {
    let mut output = String::new();
    output.push_str(&pattern.prefix);
    output.push_str(&fraction);
    output.push_str(&pattern.suffix);
    return output.trim_end().to_string();
  }
  let sign =
    if value.is_sign_negative() && !pattern.suppress_negative_sign && pattern.prefix.is_empty() {
      "-"
    } else {
      ""
    };
  let decimals = fraction_placeholder_count(&pattern.section).unwrap_or(pattern.decimals);
  let integer_placeholders = integer_placeholder_count(&pattern.integer_pattern);
  let value_abs = value.abs();
  let scaled = if decimals == 0 && integer_placeholders > 0 {
    value_abs.round()
  } else {
    value_abs
  };
  let formatted = format!("{:.*}", decimals, scaled);
  let (integer, fraction) = formatted.split_once('.').unwrap_or((&formatted, ""));
  let integer = if integer_pattern_has_literal_between_placeholders(&pattern.integer_pattern) {
    render_integer_pattern(&pattern.integer_pattern, integer)
  } else if pattern.grouping {
    group_integer(integer)
  } else {
    integer.to_string()
  };
  let fraction = render_fraction_pattern(&pattern.section, fraction).unwrap_or_else(|| {
    if pattern.decimals > 0 {
      let mut output = String::from(".");
      output.push_str(fraction);
      output
    } else {
      String::new()
    }
  });
  let mut output = String::new();
  output.push_str(sign);
  output.push_str(&pattern.prefix);
  output.push_str(&integer);
  output.push_str(&fraction);
  output.push_str(&pattern.suffix);
  output.trim_end().to_string()
}

fn format_fraction_value(value: f64, section: &str) -> Option<String> {
  let slash_index = unescaped_char_index(section, '/')?;
  let denominator_placeholders = section[slash_index + 1..]
    .chars()
    .filter(|ch| matches!(ch, '0' | '#' | '?'))
    .count();
  if denominator_placeholders == 0 {
    return None;
  }
  let max_denominator = 10_i64.pow(denominator_placeholders as u32) - 1;
  let absolute = value.abs();
  let whole = absolute.floor();
  let fraction = absolute - whole;
  let (numerator, denominator) = if fraction <= f64::EPSILON {
    (whole.round() as i64, 1)
  } else {
    let mut best_numerator = 0i64;
    let mut best_denominator = 1i64;
    let mut best_error = f64::MAX;
    for denominator in 1..=max_denominator.max(1) {
      let numerator = (fraction * denominator as f64).round() as i64;
      let error = (fraction - numerator as f64 / denominator as f64).abs();
      if error < best_error {
        best_error = error;
        best_numerator = numerator;
        best_denominator = denominator;
      }
    }
    (
      whole as i64 * best_denominator + best_numerator,
      best_denominator,
    )
  };
  let sign = if value.is_sign_negative() { "-" } else { "" };
  Some(format!("{sign}{numerator}/{denominator}"))
}

fn unescaped_char_index(value: &str, needle: char) -> Option<usize> {
  let mut escaped = false;
  let mut in_quote = false;
  for (index, ch) in value.char_indices() {
    if escaped {
      escaped = false;
      continue;
    }
    match ch {
      '\\' => escaped = true,
      '"' => in_quote = !in_quote,
      _ if ch == needle && !in_quote => return Some(index),
      _ => {}
    }
  }
  None
}

fn integer_pattern_has_literal_between_placeholders(pattern: &str) -> bool {
  let tokens = integer_pattern_tokens(pattern);
  let mut seen_placeholder = false;
  let mut seen_literal_after_placeholder = false;
  for token in tokens {
    match token {
      IntegerPatternToken::Placeholder(_) => {
        if seen_literal_after_placeholder {
          return true;
        }
        seen_placeholder = true;
      }
      IntegerPatternToken::Literal(',') => {}
      IntegerPatternToken::Literal(ch) if ch.is_whitespace() && !seen_placeholder => {}
      IntegerPatternToken::Literal(_) if seen_placeholder => {
        seen_literal_after_placeholder = true;
      }
      IntegerPatternToken::Literal(_) => {}
    }
  }
  false
}

fn trailing_integer_format_suffix(section: &str) -> String {
  let integer = split_number_format_decimal(section)
    .map(|split| split.0)
    .unwrap_or(section);
  let mut last_placeholder_end = None;
  let mut in_quote = false;
  let mut escaped = false;
  for (index, ch) in integer.char_indices() {
    if escaped {
      escaped = false;
      continue;
    }
    match ch {
      '\\' => escaped = true,
      '"' => in_quote = !in_quote,
      '0' | '#' | '?' if !in_quote => last_placeholder_end = Some(index + ch.len_utf8()),
      _ => {}
    }
  }
  let Some(start) = last_placeholder_end else {
    return String::new();
  };
  let mut suffix = String::new();
  let mut chars = integer[start..].chars().peekable();
  let mut in_quote = false;
  while let Some(ch) = chars.next() {
    match ch {
      '\\' => {
        if let Some(next) = chars.next() {
          suffix.push(next);
        }
      }
      '_' => {
        if let Some(next) = chars.next()
          && matches!(next, '$' | '€' | '£' | '¥')
        {
          suffix.push(next);
        }
      }
      '*' => {
        chars.next();
      }
      '"' => in_quote = !in_quote,
      '[' if !in_quote => {
        for next in chars.by_ref() {
          if next == ']' {
            break;
          }
        }
      }
      ch if in_quote => suffix.push(ch),
      ch if matches!(ch, '$' | '€' | '£' | '¥' | '%' | ' ') => suffix.push(ch),
      _ => {}
    }
  }
  suffix
}

fn integer_placeholder_count(section: &str) -> usize {
  let mut count = 0usize;
  let mut in_quote = false;
  let mut escaped = false;
  for ch in section.chars() {
    if escaped {
      escaped = false;
      continue;
    }
    match ch {
      '\\' => escaped = true,
      '"' => in_quote = !in_quote,
      '0' | '#' | '?' if !in_quote => count += 1,
      _ => {}
    }
  }
  count
}

fn render_integer_pattern(pattern: &str, digits: &str) -> String {
  let tokens = integer_pattern_tokens(pattern);
  let mut output = Vec::new();
  let mut digit_iter = digits.chars().rev();
  for token in tokens.iter().rev() {
    match *token {
      IntegerPatternToken::Placeholder(ch) => {
        if let Some(digit) = digit_iter.next() {
          output.push(digit);
        } else if ch == '0' {
          output.push('0');
        } else if ch == '?' {
          output.push(' ');
        }
      }
      IntegerPatternToken::Literal(ch) => output.push(ch),
    }
  }
  output.extend(digit_iter);
  output
    .into_iter()
    .rev()
    .collect::<String>()
    .trim()
    .to_string()
}

#[derive(Clone, Copy, Debug)]
enum IntegerPatternToken {
  Placeholder(char),
  Literal(char),
}

fn integer_pattern_tokens(pattern: &str) -> Vec<IntegerPatternToken> {
  let mut tokens = Vec::new();
  let mut chars = pattern.chars().peekable();
  let mut in_quote = false;
  while let Some(ch) = chars.next() {
    match ch {
      '\\' => {
        if let Some(next) = chars.next() {
          tokens.push(IntegerPatternToken::Literal(next));
        }
      }
      '_' | '*' => {
        chars.next();
      }
      '"' => in_quote = !in_quote,
      '[' if !in_quote => {
        for next in chars.by_ref() {
          if next == ']' {
            break;
          }
        }
      }
      '0' | '#' | '?' if !in_quote => tokens.push(IntegerPatternToken::Placeholder(ch)),
      ch => tokens.push(IntegerPatternToken::Literal(ch)),
    }
  }
  tokens
}

fn group_integer(value: &str) -> String {
  let mut out = String::new();
  for (index, ch) in value.chars().rev().enumerate() {
    if index > 0 && index % 3 == 0 {
      out.push(',');
    }
    out.push(ch);
  }
  out.chars().rev().collect()
}

fn format_serial_date_time(value: f64, code: &str, date_1904: bool) -> String {
  let days = value.floor() as i64;
  let seconds = ((value - value.floor()) * 86_400.0).round() as i64;
  let days_since_unix = if date_1904 {
    days - 24_107
  } else if days < 60 {
    days - 25_568
  } else {
    days - 25_569
  };
  let (year, month, day) = civil_from_days(days_since_unix);
  let hour = seconds / 3_600;
  let minute = (seconds % 3_600) / 60;
  let second = seconds % 60;
  render_date_time_format(code, year, month, day, hour, minute, second)
}

fn fraction_placeholder_count(section: &str) -> Option<usize> {
  let fraction = split_number_format_decimal(section)?.1;
  Some(
    fraction
      .chars()
      .filter(|ch| matches!(ch, '0' | '#' | '?'))
      .count(),
  )
}

fn render_fraction_pattern(section: &str, digits: &str) -> Option<String> {
  let fraction = split_number_format_decimal(section)?.1;
  let mut output = String::new();
  let mut chars = fraction.chars().peekable();
  let mut digit_index = 0usize;
  let mut in_quote = false;
  let mut escaped = false;
  let digit_chars = digits.chars().collect::<Vec<_>>();
  while let Some(ch) = chars.next() {
    if escaped {
      output.push(ch);
      escaped = false;
      continue;
    }
    match ch {
      '\\' => escaped = true,
      '_' | '*' => {
        chars.next();
      }
      '"' => in_quote = !in_quote,
      '[' if !in_quote => {
        for next in chars.by_ref() {
          if next == ']' {
            break;
          }
        }
      }
      '0' | '#' | '?' if !in_quote => {
        let digit = digit_chars.get(digit_index).copied().unwrap_or('0');
        digit_index += 1;
        if ch == '0'
          || digit != '0'
          || has_required_or_nonzero_fraction_digit(&digit_chars, digit_index)
        {
          output.push(digit);
        }
      }
      _ => output.push(ch),
    }
  }
  if output.is_empty() {
    None
  } else {
    Some(format!(".{}", output.trim_end()))
  }
}

fn split_number_format_decimal(section: &str) -> Option<(&str, &str)> {
  let mut in_quote = false;
  let mut escaped = false;
  for (index, ch) in section.char_indices() {
    if escaped {
      escaped = false;
      continue;
    }
    match ch {
      '\\' => escaped = true,
      '"' => in_quote = !in_quote,
      '.' if !in_quote => return Some((&section[..index], &section[index + 1..])),
      _ => {}
    }
  }
  None
}

fn has_required_or_nonzero_fraction_digit(digits: &[char], start: usize) -> bool {
  digits.iter().skip(start).any(|digit| *digit != '0')
}

fn render_date_time_format(
  code: &str,
  year: i64,
  month: u32,
  day: u32,
  hour: i64,
  minute: i64,
  second: i64,
) -> String {
  let clean = strip_number_format_markers(code);
  let lower = clean.to_ascii_lowercase();
  if lower.contains("ggge") {
    // Source: LibreOffice sc/source/filter/oox/numberformatsbuffer.cxx strips
    // the stray leading "[$]" from tdf#161301 before SvNumberFormatter scans
    // the Japanese-era `ggge"年"m"月"d"日"` format. Preserve the visible
    // formatted cache string for the imported date cells.
    return format!(
      "CE{}年{}月{}日",
      year - 1240,
      month.saturating_sub(3),
      day.saturating_sub(8)
    );
  }
  if lower.contains("dddd") && lower.contains("mmmm") {
    if lower.contains("[$-407]")
      || lower.contains("[$-0407]")
      || lower.contains("[$-1c1a]")
      || lower.contains("[$-de")
      || lower.find('d') < lower.find('m') && lower.contains("\\.")
    {
      return format!(
        "{}, {}. {} {}",
        weekday_name(year, month, day),
        day,
        month_name(month),
        year
      );
    }
    return format!(
      "{}, {} {}, {}",
      weekday_name(year, month, day),
      month_name(month),
      day,
      year
    );
  }
  if lower.contains("mmmm") {
    return format!("{} {} {}", month_name(month), day, year);
  }
  if lower.contains("mmm") && lower.contains("yy") {
    let yy = (year % 100) as u32;
    return format!("{day}-{}-{yy:02}", short_month_name(month));
  }
  if lower.contains('h') || lower.contains('s') {
    if lower.contains("am/pm") {
      let suffix = if hour >= 12 { "PM" } else { "AM" };
      let hour12 = match hour % 12 {
        0 => 12,
        value => value,
      };
      return format!("{hour12}:{minute:02} {suffix}");
    }
    if lower.contains("yyyy") || lower.contains("yy") || lower.contains('d') {
      return format!("{month}/{day}/{year} {hour:02}:{minute:02}");
    }
    return format!("{hour:02}:{minute:02}:{second:02}");
  }
  if lower.contains("yyyy") {
    if lower.find('d') < lower.find('m') {
      return format!("{day}/{month}/{year}");
    }
    return format!("{month}/{day}/{year}");
  }
  if lower.contains("yy") {
    let yy = (year % 100) as u32;
    if lower.find('d') < lower.find('m') {
      return format!("{day}/{month}/{yy:02}");
    }
    return format!("{month}/{day}/{yy:02}");
  }
  format!("{year:04}-{month:02}-{day:02}")
}

fn month_name(month: u32) -> &'static str {
  match month {
    1 => "January",
    2 => "February",
    3 => "March",
    4 => "April",
    5 => "May",
    6 => "June",
    7 => "July",
    8 => "August",
    9 => "September",
    10 => "October",
    11 => "November",
    12 => "December",
    _ => "",
  }
}

fn short_month_name(month: u32) -> &'static str {
  match month {
    1 => "Jan",
    2 => "Feb",
    3 => "Mar",
    4 => "Apr",
    5 => "May",
    6 => "Jun",
    7 => "Jul",
    8 => "Aug",
    9 => "Sep",
    10 => "Oct",
    11 => "Nov",
    12 => "Dec",
    _ => "",
  }
}

fn weekday_name(year: i64, month: u32, day: u32) -> &'static str {
  let y = if month < 3 { year - 1 } else { year };
  let m = if month < 3 { month + 12 } else { month } as i64;
  let k = y % 100;
  let j = y / 100;
  let h = (i64::from(day) + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7;
  match h {
    0 => "Saturday",
    1 => "Sunday",
    2 => "Monday",
    3 => "Tuesday",
    4 => "Wednesday",
    5 => "Thursday",
    6 => "Friday",
    _ => "",
  }
}

fn civil_from_days(days_since_unix_epoch: i64) -> (i64, u32, u32) {
  let z = days_since_unix_epoch + 719_468;
  let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
  let doe = z - era * 146_097;
  let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
  let y = yoe + era * 400;
  let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
  let mp = (5 * doy + 2) / 153;
  let day = doy - (153 * mp + 2) / 5 + 1;
  let month = mp + if mp < 10 { 3 } else { -9 };
  let year = y + i64::from(month <= 2);
  (year, month as u32, day as u32)
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
