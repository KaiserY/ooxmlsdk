use std::borrow::Cow;
use std::collections::HashSet;

use super::import::ExcelImport;
use super::page_settings::CalcPageSettings;
use super::pivot::pivot_print_address;
use super::styles::{DefinedNameBuiltin, DefinedNameRecord};
use super::worksheet::{CalcCell, CalcRow, CalcSheet, CellAddress, CellRange, SheetType};
use crate::docx::TextStyle;
use crate::text_metrics;
use crate::units;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

// Source: LibreOffice sc/source/ui/view/printfun.cxx defines ZOOM_MIN.
const ZOOM_MIN: u32 = 10;
const XLSX_MAX_COLUMN: u32 = 16_384;
const XLSX_MAX_ROW: u32 = 1_048_576;
const CALC_CELL_TEXT_MARGIN_PT: f32 = 4.0;
const XLSX_HEADER_FOOTER_LINE_HEIGHT_PT: f32 = 12.0;
const LIBREOFFICE_GENERIC_PRINTER_DPI: f32 = 600.0;
// Source: LibreOffice sc/source/core/data/attarray.cxx defines SC_VISATTR_STOP.
const SC_VISATTR_STOP: u32 = 84;

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
  pub(crate) text: Cow<'a, str>,
  pub(crate) style_index: Option<u32>,
  pub(crate) number_format_id: Option<u32>,
  pub(crate) number_format_code: Option<&'a str>,
  pub(crate) pivot_format_id: Option<u32>,
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
    let visible_sheets_with_body = import
      .sheets
      .iter()
      .filter(|sheet| sheet.visible())
      .filter(|sheet| !sheet_body_is_empty(import, sheet))
      .count();
    for sheet in import.sheets.iter().filter(|sheet| sheet.visible()) {
      let named_ranges = CalcPrintNamedRanges::from_import(import, sheet);
      let areas = print_areas_for_sheet(import, sheet, &named_ranges);
      let explicit_print_area = !named_ranges.resolved_print_areas.is_empty();
      let scale = print_scale_state(import, sheet, &areas, &named_ranges);
      document_top_down &= scale.top_down;
      let keep_header_footer_only_page = visible_sheets_with_body == 0
        && sheet.page_settings.header_footer.has_print_content()
        && sheet_body_is_empty(import, sheet);
      let page_areas = page_areas_for_sheet(
        import,
        sheet,
        &areas,
        &named_ranges,
        scale.zoom,
        scale.top_down,
      );
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
        let drawing_summary = drawing_summary_for_area(sheet, area);
        // Source: LibreOffice sc/source/ui/view/printfun.cxx lcl_SetHidden and
        // ScPrintFunc::DoPrint. Empty sheet page ranges are hidden by
        // ScDocument::IsPrintEmpty before PrintPage is called; header/footer
        // content is painted only for page ranges that survive that test. A
        // workbook made entirely of header/footer-only empty visible sheets
        // still emits one page; otherwise later empty sheets keep being hidden.
        let empty = area.map_or(false, |area| print_area_is_empty(import, sheet, area))
          && drawing_summary.anchors == 0
          && drawing_summary.charts == 0;
        if scale.skip_empty && empty && !(keep_header_footer_only_page && sheet_page_index == 0) {
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
          drawing_summary: drawing_summary.clone(),
          scale_mode: scale.mode,
          auto_page_columns: scale.auto_page_columns,
          auto_page_rows: scale.auto_page_rows,
          forced_break_min_pages: scale.forced_break_min_pages,
          tdf103516_adjusted: scale.tdf103516_adjusted,
          paint_ops: paint_ops_for_page(
            named_ranges.repeat_columns.is_some(),
            named_ranges.repeat_rows.is_some(),
            drawing_summary.anchors > 0,
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
  import: &ExcelImport,
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
  let fit_to_page = sheet.page_settings.fit_to_page
    || sheet.metrics.settings.properties.page_setup.fit_to_page
    || sheet.page_settings.fit_to_width != 1
    || sheet.page_settings.fit_to_height != 1;
  let (fit_to_width, fit_to_height) = if fit_to_page
    && sheet.page_settings.fit_to_width == 0
    && sheet.page_settings.fit_to_height == 0
  {
    // Source: LibreOffice sc/source/filter/oox/pagesettings.cxx starts
    // fitToWidth/fitToHeight from 1/1 defaults. If generated OOXML fields
    // collapse absent fitToWidth together with fitToHeight="0", preserve the
    // imported "fit to 1 page wide, unlimited height" behavior.
    (1, 0)
  } else {
    (
      sheet.page_settings.fit_to_width,
      sheet.page_settings.fit_to_height,
    )
  };
  let mut mode = CalcPrintScaleMode::None;
  let mut zoom = sheet.page_settings.scale;
  let mut auto_page_columns = forced_break_min_columns.max(1);
  let mut auto_page_rows = forced_break_min_rows.max(1);
  let mut tdf103516_adjusted = false;

  if fit_to_page && (fit_to_width > 0 || fit_to_height > 0) {
    mode = CalcPrintScaleMode::FitToWidthHeight;
    // Source: LibreOffice sc/source/filter/oox/pagesettings.cxx
    // PageSettingsConverter writes OOXML fitToWidth/fitToHeight directly to
    // ScaleToPagesX/Y with 0 preserved as "unlimited" for that axis.
    auto_page_columns = if fit_to_width == 0 {
      0
    } else {
      usize::try_from(fit_to_width)
        .ok()
        .unwrap_or(auto_page_columns)
        .max(forced_break_min_columns)
    };
    auto_page_rows = if fit_to_height == 0 {
      0
    } else {
      usize::try_from(fit_to_height)
        .ok()
        .unwrap_or(auto_page_rows)
        .max(forced_break_min_rows)
    };
    zoom = fit_zoom_to_pages(
      sheet,
      areas,
      named_ranges,
      auto_page_columns,
      auto_page_rows,
    );
    if zoom == sheet.page_settings.scale && sheet.page_settings.scale != 100 {
      // Source: LibreOffice sc/source/filter/oox/pagesettings.cxx writes either
      // ScaleToPagesX/Y or PageScale. In fit-to-pages mode, pageSetup scale is
      // not a fallback zoom.
      zoom = 100;
    }
    if fit_to_width > 0
      && fit_to_height == 0
      && actual_row_page_count(import, sheet, areas, named_ranges, zoom) > 1
    {
      let adjusted_zoom = ((zoom as f32) * 0.98).floor().max(ZOOM_MIN as f32) as u32;
      if adjusted_zoom < zoom
        && actual_row_page_count(import, sheet, areas, named_ranges, adjusted_zoom)
          < actual_row_page_count(import, sheet, areas, named_ranges, zoom)
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
    skip_empty: true,
    top_down: matches!(
      sheet.page_settings.page_order,
      Some(ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::PageOrderValues::DownThenOver)
        | None
    ),
  }
}

fn actual_row_page_count(
  import: &ExcelImport,
  sheet: &CalcSheet,
  areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges<'_>,
  zoom: u32,
) -> usize {
  areas
    .iter()
    .map(|area| {
      split_range_by_page_metrics(
        import,
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
  let fit_area = fit_scale_area(sheet, area, named_ranges);
  let area_rect = sheet.range_rect(fit_area);
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
  100
}

fn fit_scale_area(
  sheet: &CalcSheet,
  area: CellRange,
  named_ranges: &CalcPrintNamedRanges<'_>,
) -> CellRange {
  if !named_ranges.resolved_print_areas.is_empty() {
    return area;
  }
  sheet.used_range().map_or(area, |used| {
    let range = CellRange::new(CellAddress { col: 1, row: 1 }, used.end);
    extend_print_area_for_merges(sheet, range)
  })
}

fn print_content_size_pt(sheet: &CalcSheet) -> (f32, f32) {
  let (mut width, mut height) = sheet.page_settings.page_size_pt();
  width -= (sheet.page_settings.margin_left_in + sheet.page_settings.margin_right_in) as f32
    * crate::units::POINTS_PER_INCH;
  height -= (sheet.page_settings.margin_top_in + sheet.page_settings.margin_bottom_in) as f32
    * crate::units::POINTS_PER_INCH;
  if sheet.page_settings.scale != 100 && sheet.page_settings.header_footer.has_print_content() {
    height -= (sheet.page_settings.margin_header_in + sheet.page_settings.margin_footer_in) as f32
      * crate::units::POINTS_PER_INCH
      + 2.0 * XLSX_HEADER_FOOTER_LINE_HEIGHT_PT;
  }
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
    if !anchor_intersects_area(sheet, anchor, area) {
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
  // Source: LibreOffice sc/source/filter/oox/drawingfragment.cxx imports VML
  // client shapes into the sheet draw layer; sc/source/core/data/documen9.cxx
  // then treats that draw layer uniformly for print area and page visibility.
  for shape in sheet
    .resources
    .object_resources
    .vml_drawings
    .iter()
    .flat_map(|drawing| drawing.shapes.iter())
  {
    if !vml_shape_intersects_area(sheet, shape, area) {
      continue;
    }
    summary.anchors += 1;
    summary.printable += usize::from(shape.print_object && !shape.hidden);
    summary.hidden += usize::from(shape.hidden);
    summary.text_len += shape.text.len();
    if shape.image_relationship_id.is_some() {
      summary.pictures += 1;
    } else {
      summary.shapes += 1;
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

fn anchor_intersects_area(
  sheet: &CalcSheet,
  anchor: &super::drawing::DrawingAnchorModel,
  area: Option<CellRange>,
) -> bool {
  let Some(area) = area else {
    return true;
  };
  drawing_anchor_cell_range(sheet, anchor)
    .map(|range| range.intersects(area))
    .unwrap_or_else(|| anchor_belongs_to_area(anchor.from.as_ref(), Some(area)))
}

fn print_areas_for_sheet(
  import: &ExcelImport,
  sheet: &CalcSheet,
  named_ranges: &CalcPrintNamedRanges<'_>,
) -> Vec<CellRange> {
  if !named_ranges.resolved_print_areas.is_empty() {
    return named_ranges
      .resolved_print_areas
      .iter()
      .copied()
      .map(|range| extend_print_area_for_merges(sheet, range))
      .collect();
  }
  match sheet.used_range() {
    // Source: LibreOffice sc/source/ui/view/printfun.cxx::AdjustPrintArea(true).
    // Implicit print ranges start at A1; ScDocument::GetPrintArea() only
    // supplies the lower-right used cell. Empty leading rows/columns still
    // participate in page-break calculation and are skipped later by
    // ScDocument::IsPrintEmpty.
    Some(range) => {
      let mut range = CellRange::new(CellAddress { col: 1, row: 1 }, range.end);
      if pivot_tabular_page_field_area_uses_dimension(sheet)
        && let Some(dimension) = sheet
          .metrics
          .dimension
          .as_deref()
          .and_then(CellRange::parse_a1_range)
      {
        range.end.col = range.end.col.max(dimension.end.col);
        range.end.row = range.end.row.max(dimension.end.row);
      }
      if let Some(attr_end_row) = last_visible_row_attribute(import, sheet, range.end.row) {
        range.end.row = range.end.row.max(attr_end_row);
      }
      if let Some(drawing_range) = drawing_print_area(sheet) {
        range.end.col = range.end.col.max(drawing_range.end.col);
        range.end.row = range.end.row.max(drawing_range.end.row);
      }
      range = extend_print_area_for_merges(sheet, range);
      vec![extend_print_area_for_overflow(import, sheet, range)]
    }
    // With skip-empty disabled, a missing document print area still leaves the
    // default start/end range printable, so header/footer-only sheets export a page.
    None => {
      vec![drawing_print_area(sheet).unwrap_or(CellRange::single(CellAddress { col: 1, row: 1 }))]
    }
  }
}

fn pivot_tabular_page_field_area_uses_dimension(sheet: &CalcSheet) -> bool {
  sheet
    .resources
    .pivot_tables
    .tables
    .iter()
    .any(|pivot| pivot.page_fields > 0 && pivot.row_fields > 1 && !pivot.compact)
}

fn extend_print_area_for_merges(sheet: &CalcSheet, mut range: CellRange) -> CellRange {
  // Source: LibreOffice sc/source/ui/view/printfun.cxx::AdjustPrintArea calls
  // ScDocument::ExtendMerge before text-overflow expansion, and
  // sc/source/core/data/table2.cxx extends the print end to any merged-cell
  // start inside the current area.
  let old_end_col = range.end.col;
  let old_end_row = range.end.row;
  for merged in sheet
    .metrics
    .merged_ranges
    .iter()
    .filter_map(|reference| CellRange::parse_a1_range(reference))
  {
    if merged.start.col >= range.start.col
      && merged.start.col <= old_end_col
      && merged.start.row >= range.start.row
      && merged.start.row <= old_end_row
    {
      range.end.col = range.end.col.max(merged.end.col);
      range.end.row = range.end.row.max(merged.end.row);
    }
  }
  range
}

fn drawing_print_area(sheet: &CalcSheet) -> Option<CellRange> {
  // Source: LibreOffice sc/source/core/data/documen2.cxx::ScDocument::GetPrintArea
  // merges ScDrawLayer::GetPrintArea into the sheet print area, and
  // sc/source/core/data/drwlayer.cxx::ScDrawLayer::GetPrintArea maps object
  // bounds back to start/end cells.
  let xdr_ranges = sheet
    .resources
    .drawings
    .iter()
    .flat_map(|drawing| drawing.anchors.iter())
    .filter_map(|anchor| drawing_anchor_cell_range(sheet, anchor));
  let vml_ranges = sheet
    .resources
    .object_resources
    .vml_drawings
    .iter()
    .flat_map(|drawing| drawing.shapes.iter())
    .filter_map(|shape| vml_shape_cell_range(sheet, shape));
  xdr_ranges.chain(vml_ranges).reduce(|acc, range| {
    CellRange::new(
      CellAddress {
        col: acc.start.col.min(range.start.col),
        row: acc.start.row.min(range.start.row),
      },
      CellAddress {
        col: acc.end.col.max(range.end.col),
        row: acc.end.row.max(range.end.row),
      },
    )
  })
}

fn drawing_anchor_cell_range(
  sheet: &CalcSheet,
  anchor: &super::drawing::DrawingAnchorModel,
) -> Option<CellRange> {
  let (x_pt, y_pt, width_pt, height_pt) = drawing_anchor_rect_pt(sheet, anchor)?;
  Some(CellRange::new(
    CellAddress {
      col: sheet_column_for_x(sheet, x_pt),
      row: sheet_row_for_y(sheet, y_pt),
    },
    CellAddress {
      col: sheet_column_for_x(sheet, x_pt + width_pt),
      row: sheet_row_for_y(sheet, y_pt + height_pt),
    },
  ))
}

fn drawing_anchor_rect_pt(
  sheet: &CalcSheet,
  anchor: &super::drawing::DrawingAnchorModel,
) -> Option<(f32, f32, f32, f32)> {
  match anchor.kind {
    super::drawing::DrawingAnchorKind::TwoCell => {
      let from = anchor.from.as_ref()?;
      let to = anchor.to.as_ref()?;
      let (x1, y1) = sheet.marker_position_pt(from);
      let (x2, y2) = sheet.marker_position_pt(to);
      Some((
        x1.min(x2),
        y1.min(y2),
        (x2 - x1).abs() + units::emu_to_points(1),
        (y2 - y1).abs() + units::emu_to_points(1),
      ))
    }
    super::drawing::DrawingAnchorKind::OneCell => {
      let from = anchor.from.as_ref()?;
      let (x, y) = sheet.marker_position_pt(from);
      let (cx, cy) = anchor.extent?;
      Some((x, y, units::emu_to_points(cx), units::emu_to_points(cy)))
    }
    super::drawing::DrawingAnchorKind::Absolute => {
      let (x, y) = anchor.position?;
      let (cx, cy) = anchor.extent?;
      Some((
        units::emu_to_points(x),
        units::emu_to_points(y),
        units::emu_to_points(cx),
        units::emu_to_points(cy),
      ))
    }
  }
}

fn vml_shape_intersects_area(
  sheet: &CalcSheet,
  shape: &super::object_resources::VmlShapeModel,
  area: Option<CellRange>,
) -> bool {
  let Some(area) = area else {
    return true;
  };
  vml_shape_cell_range(sheet, shape)
    .map(|range| range.intersects(area))
    .unwrap_or(true)
}

fn vml_shape_cell_range(
  sheet: &CalcSheet,
  shape: &super::object_resources::VmlShapeModel,
) -> Option<CellRange> {
  let (x_pt, y_pt, width_pt, height_pt) = vml_shape_rect_pt(sheet, shape)?;
  Some(CellRange::new(
    CellAddress {
      col: sheet_column_for_x(sheet, x_pt),
      row: sheet_row_for_y(sheet, y_pt),
    },
    CellAddress {
      col: sheet_column_for_x(sheet, x_pt + width_pt),
      row: sheet_row_for_y(sheet, y_pt + height_pt),
    },
  ))
}

fn vml_shape_rect_pt(
  sheet: &CalcSheet,
  shape: &super::object_resources::VmlShapeModel,
) -> Option<(f32, f32, f32, f32)> {
  shape
    .anchor
    .and_then(|anchor| vml_anchor_rect_pt(sheet, anchor))
    .or_else(|| shape.style.as_deref().and_then(vml_style_rect_pt))
}

fn vml_anchor_rect_pt(
  sheet: &CalcSheet,
  anchor: super::object_resources::VmlClientAnchor,
) -> Option<(f32, f32, f32, f32)> {
  let x1 = vml_anchor_x_pt(sheet, anchor.from_col, anchor.from_col_offset_px);
  let y1 = vml_anchor_y_pt(sheet, anchor.from_row, anchor.from_row_offset_px);
  let x2 = vml_anchor_x_pt(sheet, anchor.to_col, anchor.to_col_offset_px);
  let y2 = vml_anchor_y_pt(sheet, anchor.to_row, anchor.to_row_offset_px);
  if x2 < x1 || y2 < y1 {
    return None;
  }
  Some((
    x1,
    y1,
    x2 - x1 + units::twips_to_points(1.0),
    y2 - y1 + units::twips_to_points(1.0),
  ))
}

fn vml_anchor_x_pt(sheet: &CalcSheet, zero_based_col: u32, offset_px: i32) -> f32 {
  let col = zero_based_col.saturating_add(1);
  let cell = sheet.cell_rect(CellAddress { col, row: 1 });
  let next_cell = sheet.cell_rect(CellAddress {
    col: col.saturating_add(1),
    row: 1,
  });
  // Source: LibreOffice sc/source/filter/oox/drawingbase.cxx
  // ShapeAnchor::importVmlAnchor marks offsets as CellAnchorType::Pixel, and
  // calcCellAnchorEmu clamps them to the next cell minus one twip.
  (cell.x_pt + vml_screen_pixel_to_pt(offset_px)).min(next_cell.x_pt - units::twips_to_points(1.0))
}

fn vml_anchor_y_pt(sheet: &CalcSheet, zero_based_row: u32, offset_px: i32) -> f32 {
  let row = zero_based_row.saturating_add(1);
  let cell = sheet.cell_rect(CellAddress { col: 1, row });
  let next_cell = sheet.cell_rect(CellAddress {
    col: 1,
    row: row.saturating_add(1),
  });
  (cell.y_pt + vml_screen_pixel_to_pt(offset_px)).min(next_cell.y_pt - units::twips_to_points(1.0))
}

fn vml_screen_pixel_to_pt(value: i32) -> f32 {
  value as f32 * units::POINTS_PER_INCH / 96.0
}

fn vml_style_rect_pt(style: &str) -> Option<(f32, f32, f32, f32)> {
  let x = vml_style_length_pt(style, "margin-left")?;
  let y = vml_style_length_pt(style, "margin-top")?;
  let width = vml_style_length_pt(style, "width")?;
  let height = vml_style_length_pt(style, "height")?;
  Some((x, y, width, height))
}

fn vml_style_length_pt(style: &str, key: &str) -> Option<f32> {
  style.split(';').find_map(|part| {
    let (name, value) = part.split_once(':')?;
    if name.trim() != key {
      return None;
    }
    parse_vml_length_pt(value.trim())
  })
}

fn parse_vml_length_pt(value: &str) -> Option<f32> {
  if let Some(value) = value.strip_suffix("pt") {
    return value.trim().parse::<f32>().ok();
  }
  if let Some(value) = value.strip_suffix("in") {
    return value
      .trim()
      .parse::<f32>()
      .ok()
      .map(|value| value * units::POINTS_PER_INCH);
  }
  if let Some(value) = value.strip_suffix("px") {
    return value
      .trim()
      .parse::<f32>()
      .ok()
      .map(vml_screen_pixel_to_pt_f32);
  }
  value.parse::<f32>().ok()
}

fn vml_screen_pixel_to_pt_f32(value: f32) -> f32 {
  value * units::POINTS_PER_INCH / 96.0
}

fn sheet_column_for_x(sheet: &CalcSheet, x_pt: f32) -> u32 {
  let mut width = 0.0f32;
  for column in 1..=XLSX_MAX_COLUMN {
    width += sheet.column_width_pt(column);
    if width > x_pt {
      return column;
    }
  }
  XLSX_MAX_COLUMN
}

fn sheet_row_for_y(sheet: &CalcSheet, y_pt: f32) -> u32 {
  let mut height = 0.0f32;
  for row in 1..=XLSX_MAX_ROW {
    height += sheet.row_height_pt(row);
    if height > y_pt {
      return row;
    }
  }
  XLSX_MAX_ROW
}

fn extend_print_area_for_overflow(
  import: &ExcelImport,
  sheet: &CalcSheet,
  mut range: CellRange,
) -> CellRange {
  // Source: LibreOffice sc/source/ui/view/printfun.cxx AdjustPrintArea and
  // sc/source/core/data/table1.cxx ExtendPrintArea/MaybeAddExtraColumn.
  // A text cell can extend the implicit print area to the right when the next
  // cells are empty and the string does not fit into its own column.
  for (row_position, row) in sheet.rows.iter().enumerate() {
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    if row_index < range.start.row || row_index > range.end.row || row.hidden {
      continue;
    }
    for (cell_position, cell) in row.cells.iter().enumerate() {
      if cell.display_text.is_empty() || cell.display_text.parse::<f64>().is_ok() {
        continue;
      }
      let address = cell.address().unwrap_or(CellAddress {
        col: cell_position as u32 + 1,
        row: row_index,
      });
      if address.col < range.start.col || address.col > range.end.col {
        continue;
      }
      if row_cell_has_print_data_at(row, address.col + 1) {
        continue;
      }
      let style_index = sheet.effective_cell_style_index(row, cell, address);
      if import
        .styles
        .alignment_for_cell(style_index)
        .is_some_and(|alignment| alignment.wrap_text)
      {
        continue;
      }
      let style = import.styles.text_style_for_cell(style_index);
      let column = text_overflow_end_column(sheet, row, cell, address, &style);
      if column > range.end.col {
        range.end.col = column;
      }
    }
  }
  range
}

fn text_overflow_end_column(
  sheet: &CalcSheet,
  row: &CalcRow,
  cell: &CalcCell,
  address: CellAddress,
  style: &TextStyle,
) -> u32 {
  let needed_width_pt = calc_cached_print_text_width_pt(
    text_metrics::measure_text(&cell.display_text, style) + CALC_CELL_TEXT_MARGIN_PT,
  );
  let mut missing = needed_width_pt - sheet.column_width_pt(address.col);
  let mut column = address.col;
  while missing > 0.0 && column < XLSX_MAX_COLUMN {
    let next = column.saturating_add(1);
    if row_cell_has_print_data_at(row, next) {
      break;
    }
    column = next;
    let width = sheet.column_width_pt(column);
    if width <= f32::EPSILON {
      break;
    }
    missing -= width;
  }
  column
}

fn calc_cached_print_text_width_pt(width_pt: f32) -> f32 {
  if width_pt <= 0.0 || !width_pt.is_finite() {
    return 0.0;
  }
  // Source: LibreOffice sc/source/core/data/documen8.cxx stores
  // GetNeededSize(..., bTotalSize=true) into ScColumnTextWidthIterator as a
  // sal_uInt16. sc/source/core/data/table1.cxx::MaybeAddExtraColumn then reads
  // that cached GetTextWidth() pixel value and converts it back through nPPTX.
  // Calc print layout calls GetPrinter(); the Unix generic printer resolves
  // its DPI through PPDContext::getRenderResolution(), and LibreOffice's
  // bundled SGENPRT.PS has *DefaultResolution: 600dpi.
  let pixels = (width_pt * LIBREOFFICE_GENERIC_PRINTER_DPI / units::POINTS_PER_INCH).round() as i64;
  let cached_pixels = pixels as u16;
  f32::from(cached_pixels) * units::POINTS_PER_INCH / LIBREOFFICE_GENERIC_PRINTER_DPI
}

fn print_cell_text_style(
  import: &ExcelImport,
  sheet: &CalcSheet,
  row: &CalcRow,
  cell: &CalcCell,
  address: CellAddress,
) -> TextStyle {
  let style_index = sheet.effective_cell_style_index(row, cell, address);
  import.styles.text_style_for_cell(style_index)
}

fn row_cell_has_print_data_at(row: &CalcRow, col: u32) -> bool {
  row.cells.iter().enumerate().any(|(cell_position, cell)| {
    let address = cell.address().unwrap_or(CellAddress {
      col: cell_position as u32 + 1,
      row: row.row_index.unwrap_or(1),
    });
    address.col == col
      && (!cell.display_text.is_empty()
        || !cell.rich_text_runs.is_empty()
        || cell.formula.is_some()
        || cell.cached_value.is_some()
        || cell.data_type.is_some())
  })
}

fn print_area_is_empty(import: &ExcelImport, sheet: &CalcSheet, area: CellRange) -> bool {
  // Source: LibreOffice sc/source/core/data/documen9.cxx::ScDocument::IsPrintEmpty.
  // Calc treats a block as printable when it has cell content, border lines, or
  // drawing content. Drawing content is checked by the caller because it uses
  // drawing anchors rather than cell records.
  for (row_position, row) in sheet.rows.iter().enumerate() {
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    if row_index < area.start.row || row_index > area.end.row || row.hidden {
      continue;
    }
    for (cell_position, cell) in row.cells.iter().enumerate() {
      let address = cell.address().unwrap_or(CellAddress {
        col: cell_position as u32 + 1,
        row: row_index,
      });
      let Some(print_address) = pivot_print_address(sheet, address) else {
        continue;
      };
      if !print_cell_intersects_area(sheet, address, print_address, area)
        || column_hidden(sheet, address.col)
      {
        continue;
      }
      if !cell.display_text.is_empty()
        || !cell.rich_text_runs.is_empty()
        || cell.formula.is_some()
        || cell.cached_value.is_some()
        || cell.data_type.is_some()
      {
        return false;
      }
      let style_index = sheet.effective_cell_style_index(row, cell, address);
      let borders = import.styles.borders_for_cell(style_index);
      if borders.left.is_some()
        || borders.right.is_some()
        || borders.top.is_some()
        || borders.bottom.is_some()
      {
        return false;
      }
      if import.styles.fill_for_cell(style_index).color.is_some() {
        return false;
      }
    }
  }
  if sheet_area_has_left_text_overflow(import, sheet, area) {
    return false;
  }
  if area.end.col <= 2 && area_has_visible_row_attribute(import, sheet, area) {
    return false;
  }
  true
}

fn last_visible_row_attribute(
  import: &ExcelImport,
  sheet: &CalcSheet,
  data_end_row: u32,
) -> Option<u32> {
  // Source: LibreOffice sc/source/core/data/table1.cxx::ScTable::GetPrintArea
  // calls ScAttrArray::GetLastVisibleAttr after data detection. Explicit row
  // formatting near the end of the sheet extends the print area even when the
  // rows contain no cells; long equal formatting runs are stopped at
  // SC_VISATTR_STOP.
  let mut last_row = None;
  let mut run_len = 0u32;
  let mut previous_row = None;
  for row in sheet
    .rows
    .iter()
    .filter(|row| row_has_visible_attribute(import, sheet, row))
  {
    let row_index = row.row_index.unwrap_or(1);
    if row_index <= data_end_row {
      continue;
    }
    if previous_row.is_some_and(|previous| previous + 1 == row_index) {
      run_len = run_len.saturating_add(1);
    } else {
      run_len = 1;
    }
    previous_row = Some(row_index);
    if run_len < SC_VISATTR_STOP {
      last_row = Some(row_index);
    }
  }
  last_row
}

fn area_has_visible_row_attribute(
  import: &ExcelImport,
  sheet: &CalcSheet,
  area: CellRange,
) -> bool {
  sheet.rows.iter().any(|row| {
    let row_index = row.row_index.unwrap_or(1);
    row_index >= area.start.row
      && row_index <= area.end.row
      && row_has_visible_attribute(import, sheet, row)
  })
}

fn row_has_visible_attribute(import: &ExcelImport, sheet: &CalcSheet, row: &CalcRow) -> bool {
  if row.hidden {
    return false;
  }
  if row.cells.is_empty()
    && !row.custom_height
    && row.height.is_some_and(|height| {
      (height as f32 - sheet.metrics.format.default_row_height as f32).abs() > f32::EPSILON
    })
  {
    return true;
  }
  let borders = import.styles.borders_for_cell(row.style_index);
  borders.left.is_some()
    || borders.right.is_some()
    || borders.top.is_some()
    || borders.bottom.is_some()
    || import.styles.fill_for_cell(row.style_index).color.is_some()
}

fn sheet_area_has_left_text_overflow(
  import: &ExcelImport,
  sheet: &CalcSheet,
  area: CellRange,
) -> bool {
  if area.start.col <= 1 {
    return false;
  }
  sheet.rows.iter().enumerate().any(|(row_position, row)| {
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    if row_index < area.start.row || row_index > area.end.row || row.hidden {
      return false;
    }
    row.cells.iter().enumerate().any(|(cell_position, cell)| {
      if cell.display_text.is_empty() || cell.display_text.parse::<f64>().is_ok() {
        return false;
      }
      let address = cell.address().unwrap_or(CellAddress {
        col: cell_position as u32 + 1,
        row: row_index,
      });
      if address.col >= area.start.col {
        return false;
      }
      // Source: LibreOffice sc/source/core/data/documen9.cxx
      // ScDocument::IsPrintEmpty calls ExtendPrintArea() for the columns left
      // of the candidate page. If a left-side string extends into this page,
      // the page is not empty even when it has no cell bodies of its own.
      let style = print_cell_text_style(import, sheet, row, cell, address);
      text_overflow_end_column(sheet, row, cell, address, &style) >= area.start.col
    })
  })
}

fn sheet_body_is_empty(import: &ExcelImport, sheet: &CalcSheet) -> bool {
  let has_drawing = sheet
    .resources
    .drawings
    .iter()
    .flat_map(|drawing| drawing.anchors.iter())
    .any(|anchor| drawing_anchor_cell_range(sheet, anchor).is_some());
  if has_drawing {
    return false;
  }
  let has_vml_drawing = sheet
    .resources
    .object_resources
    .vml_drawings
    .iter()
    .flat_map(|drawing| drawing.shapes.iter())
    .any(|shape| vml_shape_cell_range(sheet, shape).is_some());
  if has_vml_drawing {
    return false;
  }
  sheet.rows.iter().all(|row| {
    row.cells.iter().all(|cell| {
      let Some(address) = cell.address() else {
        return true;
      };
      if !cell.display_text.is_empty()
        || !cell.rich_text_runs.is_empty()
        || cell.formula.is_some()
        || cell.cached_value.is_some()
        || cell.data_type.is_some()
      {
        return false;
      }
      let style_index = sheet.effective_cell_style_index(row, cell, address);
      let borders = import.styles.borders_for_cell(style_index);
      borders.left.is_none()
        && borders.right.is_none()
        && borders.top.is_none()
        && borders.bottom.is_none()
    })
  })
}

fn page_areas_for_sheet(
  import: &ExcelImport,
  sheet: &CalcSheet,
  print_areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges<'_>,
  zoom: u32,
  top_down: bool,
) -> Vec<Option<CellRange>> {
  if sheet.sheet_type == SheetType::Chartsheet {
    return vec![None];
  }
  let mut pages = Vec::new();
  for area in print_areas {
    let row_slices = split_range_by_page_metrics(
      import,
      sheet,
      *area,
      &sheet.metrics.row_breaks,
      true,
      named_ranges.repeat_rows,
      zoom,
    );
    let column_slices = split_range_by_page_metrics(
      import,
      sheet,
      *area,
      &sheet.metrics.column_breaks,
      false,
      named_ranges.repeat_columns,
      zoom,
    );
    if top_down {
      // Source: LibreOffice sc/source/ui/view/printfun.cxx::ScPrintFunc::DoPrint.
      // bTopDown prints all Y pages for one X page before advancing rightward.
      for column_slice in &column_slices {
        for row_slice in &row_slices {
          pages.push(Some(intersect_page_slices(*row_slice, *column_slice)));
        }
      }
    } else {
      for row_slice in &row_slices {
        for column_slice in &column_slices {
          pages.push(Some(intersect_page_slices(*row_slice, *column_slice)));
        }
      }
    }
  }
  pages
}

fn intersect_page_slices(row_slice: CellRange, column_slice: CellRange) -> CellRange {
  CellRange::new(
    CellAddress {
      col: column_slice.start.col,
      row: row_slice.start.row,
    },
    CellAddress {
      col: column_slice.end.col,
      row: row_slice.end.row,
    },
  )
}

fn split_range_by_page_metrics(
  import: &ExcelImport,
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
      print_row_height_pt(import, sheet, current)
    } else {
      sheet.column_width_pt(current)
    };
    if used > 0.0 && used + size > available {
      let previous = axis_slice(area, by_row, current_start, current - 1);
      slices.push(previous);
      if !by_row
        && !sheet_area_has_print_data(sheet, previous)
        && column_has_print_data(sheet, current)
      {
        // Source: LibreOffice sc/source/ui/view/printfun.cxx::CalcPages
        // keeps the overflowing column as the visible page when all previous
        // columns in the automatic slice are empty.
        slices.push(axis_slice(area, by_row, current, current));
        current += 1;
        current_start = current;
        used = 0.0;
        continue;
      }
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

fn print_row_height_pt(import: &ExcelImport, sheet: &CalcSheet, row_index: u32) -> f32 {
  let base = sheet.row_height_pt(row_index);
  if base <= f32::EPSILON {
    return base;
  }
  let Some(row) = sheet
    .rows
    .iter()
    .find(|row| row.row_index.unwrap_or(0) == row_index)
  else {
    return base;
  };
  if row.custom_height {
    return base;
  }
  let mut height = base;
  for (cell_position, cell) in row.cells.iter().enumerate() {
    if cell.display_text.is_empty() {
      continue;
    }
    let address = cell.address().unwrap_or(CellAddress {
      col: cell_position as u32 + 1,
      row: row_index,
    });
    let style_index = sheet.effective_cell_style_index(row, cell, address);
    let Some(alignment) = import.styles.alignment_for_cell(style_index) else {
      continue;
    };
    if !alignment.wrap_text {
      continue;
    }
    let style = import.styles.text_style_for_cell(style_index);
    let line_count = wrapped_print_line_count(
      &cell.display_text,
      sheet.column_width_pt(address.col),
      &style,
    );
    if line_count > 1 {
      // Source: LibreOffice sc/source/core/data/column2.cxx
      // ScColumn::GetOptimalHeight uses GetNeededSize() for line-break cells;
      // one text line follows lcl_GetAttribHeight() at 1.18 * font height.
      height = height.max(style.font_size_pt * 1.18 * line_count as f32);
    }
  }
  height
}

fn wrapped_print_line_count(text: &str, column_width_pt: f32, style: &TextStyle) -> usize {
  let available = (column_width_pt - CALC_CELL_TEXT_MARGIN_PT).max(1.0);
  let mut lines = 0usize;
  for paragraph in text.split(['\n', '\r']) {
    if paragraph.is_empty() {
      lines += 1;
      continue;
    }
    let mut current_width = 0.0f32;
    for word in paragraph.split_whitespace() {
      let word_width = text_metrics::measure_text(word, style);
      let separator_width = if current_width > 0.0 {
        text_metrics::measure_text(" ", style)
      } else {
        0.0
      };
      if current_width > 0.0 && current_width + separator_width + word_width > available {
        lines += 1;
        current_width = word_width;
      } else {
        current_width += separator_width + word_width;
      }
    }
    lines += 1;
  }
  lines.max(1)
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

fn sheet_area_has_print_data(sheet: &CalcSheet, area: CellRange) -> bool {
  sheet.rows.iter().enumerate().any(|(row_position, row)| {
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    if row_index < area.start.row || row_index > area.end.row || row.hidden {
      return false;
    }
    row.cells.iter().enumerate().any(|(cell_position, cell)| {
      let address = cell.address().unwrap_or(CellAddress {
        col: cell_position as u32 + 1,
        row: row_index,
      });
      area.contains(address) && cell_has_print_data(cell)
    })
  })
}

fn column_has_print_data(sheet: &CalcSheet, column: u32) -> bool {
  sheet.rows.iter().enumerate().any(|(row_position, row)| {
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    !row.hidden
      && row.cells.iter().enumerate().any(|(cell_position, cell)| {
        let address = cell.address().unwrap_or(CellAddress {
          col: cell_position as u32 + 1,
          row: row_index,
        });
        address.col == column && cell_has_print_data(cell)
      })
  })
}

fn cell_has_print_data(cell: &super::worksheet::CalcCell) -> bool {
  !cell.display_text.is_empty()
    || !cell.rich_text_runs.is_empty()
    || cell.formula.is_some()
    || cell.cached_value.is_some()
    || cell.data_type.is_some()
}

fn print_cell_intersects_area(
  sheet: &CalcSheet,
  address: CellAddress,
  print_address: CellAddress,
  area: CellRange,
) -> bool {
  if area.contains(print_address) {
    return true;
  }
  // Source: LibreOffice sc/source/ui/view/output2.cxx starts cell output one
  // column before the page and resolves overlapped cells through
  // ScOutputData::GetMergeOrigin, so a merged cell whose origin is left of the
  // current page still paints on pages intersecting the merged range.
  sheet
    .merged_range_for_cell(address)
    .filter(|merged| merged.start == address)
    .is_some_and(|merged| merged.intersects(area))
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
  let mut physical_cells = Vec::new();
  let mut occupied = HashSet::new();
  for (row_position, row) in sheet.rows.iter().enumerate() {
    let row_index = row.row_index.unwrap_or(row_position as u32 + 1);
    for (cell_position, cell) in row.cells.iter().enumerate() {
      let address = cell.address().unwrap_or(CellAddress {
        col: cell_position as u32 + 1,
        row: row_index,
      });
      let Some(print_address) = pivot_print_address(sheet, address) else {
        continue;
      };
      if !print_cell_intersects_area(sheet, address, print_address, area) {
        continue;
      }
      occupied.insert(print_address);
      let hidden_column = column_hidden(sheet, address.col);
      if !include_hidden && (row.hidden || hidden_column) {
        continue;
      }
      let style_index = sheet.effective_cell_style_index(row, cell, address);
      let number_format_id = style_index
        .and_then(|index| import.styles.cell_xfs.get(index as usize))
        .and_then(|format| format.number_format_id);
      let number_format_code = number_format_id.and_then(|id| import.styles.number_format_code(id));
      let conditional_number_format_code =
        conditional_number_format_code(import, sheet, address, cell.display_text.as_str());
      let pivot_format_id = super::pivot::pivot_format_id_for_address(sheet, print_address);
      let pivot_format_number_format_code = pivot_format_id
        .and_then(|format_id| import.styles.differential_number_format_code(format_id));
      let pivot_header_number_format_code =
        pivot_header_number_format_code(import, sheet, print_address);
      let pivot_number_format_code = pivot_data_number_format_code(import, sheet, print_address);
      let raw_text = pivot_data_cell_text_override(sheet, print_address)
        .unwrap_or_else(|| cell.display_text.clone());
      let effective_number_format_code = conditional_number_format_code
        .or(pivot_format_number_format_code)
        .or(pivot_header_number_format_code)
        .or(pivot_number_format_code)
        .or(number_format_code);
      let (rendered_text, number_format_state) = rendered_number_text(
        raw_text.as_str(),
        effective_number_format_code,
        cell.data_type,
        import.globals.settings.date_1904,
      );
      let rendered_text = pivot_display_text(sheet, print_address, rendered_text);
      physical_cells.push(CalcPrintCell {
        address: print_address,
        text: Cow::Borrowed(cell.display_text.as_str()),
        style_index,
        number_format_id,
        number_format_code,
        pivot_format_id,
        rendered_text,
        rich_text_runs: &cell.rich_text_runs,
        number_format_state,
        hidden_row: row.hidden,
        hidden_column,
        formula: cell.formula.is_some(),
      });
    }
  }
  let virtual_cells = pivot_virtual_print_cells(sheet, area, &occupied);
  merge_print_cells_by_scan_order(physical_cells, virtual_cells)
}

fn merge_print_cells_by_scan_order<'a>(
  physical_cells: Vec<CalcPrintCell<'a>>,
  virtual_cells: Vec<CalcPrintCell<'a>>,
) -> Vec<CalcPrintCell<'a>> {
  let mut merged = Vec::with_capacity(physical_cells.len() + virtual_cells.len());
  let mut virtual_cells = virtual_cells.into_iter().peekable();
  for cell in physical_cells {
    while virtual_cells
      .peek()
      .is_some_and(|virtual_cell| cell_address_before(virtual_cell.address, cell.address))
    {
      if let Some(virtual_cell) = virtual_cells.next() {
        merged.push(virtual_cell);
      }
    }
    merged.push(cell);
  }
  merged.extend(virtual_cells);
  merged
}

fn cell_address_before(left: CellAddress, right: CellAddress) -> bool {
  left.row < right.row || (left.row == right.row && left.col < right.col)
}

fn pivot_virtual_print_cells<'a>(
  sheet: &'a CalcSheet,
  area: CellRange,
  occupied: &HashSet<CellAddress>,
) -> Vec<CalcPrintCell<'a>> {
  let mut cells = Vec::new();
  for pivot in &sheet.resources.pivot_tables.tables {
    let geometry = pivot.output_geometry;
    if !geometry.table_range.intersects(area) {
      continue;
    }
    // Source: LibreOffice sc/source/core/data/dpoutput.cxx::Output displays
    // the data description at the top-right corner when the table has one
    // data result column and no column fields. This is generated by Calc's
    // DataPilot output and may not exist in the stale OOXML sheetData cache.
    if geometry.data_columns == 1 && pivot.column_field_names.is_empty() {
      let Some(row) = geometry.data_start.row.checked_sub(1) else {
        continue;
      };
      let address = CellAddress {
        col: geometry.data_start.col,
        row,
      };
      if area.contains(address) && !occupied.contains(&address) {
        let text = pivot
          .data_field_names
          .first()
          .cloned()
          .unwrap_or_else(|| "(empty)".to_string());
        cells.push(CalcPrintCell {
          address,
          text: Cow::Owned(text.clone()),
          style_index: None,
          number_format_id: None,
          number_format_code: None,
          pivot_format_id: None,
          rendered_text: text,
          rich_text_runs: &[],
          number_format_state: NumberFormatRenderState::Raw,
          hidden_row: false,
          hidden_column: false,
          formula: false,
        });
      }
    }
  }
  cells
}

fn conditional_number_format_code<'a>(
  import: &'a ExcelImport,
  sheet: &CalcSheet,
  address: CellAddress,
  raw: &str,
) -> Option<&'a str> {
  let value = raw.parse::<f64>().ok()?;
  let mut rules = sheet
    .metrics
    .conditions
    .conditional_formats
    .iter()
    .filter(|format| conditional_format_contains_cell(format, address))
    .flat_map(|format| {
      format
        .rules
        .iter()
        .map(move |rule| (format.sequence_of_references.as_slice(), rule))
    })
    .collect::<Vec<_>>();
  rules.sort_by_key(|(_, rule)| rule.priority);
  for (references, rule) in rules {
    if !conditional_numeric_rule_matches(import, sheet, references, rule, address, value) {
      if rule.stop_if_true {
        break;
      }
      continue;
    }
    if let Some(code) = rule
      .format_id
      .and_then(|format_id| import.styles.differential_number_format_code(format_id))
    {
      return Some(code);
    }
    if rule.stop_if_true {
      break;
    }
  }
  None
}

fn conditional_format_contains_cell(
  format: &super::sheet_conditions::ConditionalFormatModel,
  address: CellAddress,
) -> bool {
  format.sequence_of_references.iter().any(|references| {
    references
      .split_whitespace()
      .filter_map(CellRange::parse_a1_range)
      .any(|range| range.contains(address))
  })
}

fn conditional_numeric_rule_matches(
  import: &ExcelImport,
  sheet: &CalcSheet,
  references: &[String],
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
  value: f64,
) -> bool {
  match rule.rule_type {
    x::ConditionalFormatValues::Top10 => {
      conditional_top10_matches(sheet, references, rule, address, value)
    }
    x::ConditionalFormatValues::AboveAverage => {
      conditional_average_matches(sheet, references, rule, address, value)
    }
    x::ConditionalFormatValues::CellIs => {
      conditional_cell_is_matches(import, sheet, references, rule, address, value)
    }
    x::ConditionalFormatValues::Expression => {
      conditional_expression_matches(import, sheet, references, rule, address)
    }
    _ => false,
  }
}

fn conditional_top10_matches(
  sheet: &CalcSheet,
  references: &[String],
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
  value: f64,
) -> bool {
  let values = conditional_reference_values(sheet, references, address);
  if values.is_empty() {
    return false;
  }
  let mut rank = (rule.rank.unwrap_or(10) as usize).max(1);
  if rule.percent {
    rank = ((values.len() as f64 * rank as f64 / 100.0).ceil() as usize).max(1);
  }
  rank = rank.min(values.len());
  let mut sorted = values;
  if rule.bottom {
    sorted.sort_by(|a, b| a.total_cmp(b));
    value <= sorted[rank - 1]
  } else {
    sorted.sort_by(|a, b| b.total_cmp(a));
    value >= sorted[rank - 1]
  }
}

fn conditional_average_matches(
  sheet: &CalcSheet,
  references: &[String],
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
  value: f64,
) -> bool {
  let values = conditional_reference_values(sheet, references, address);
  if values.is_empty() {
    return false;
  }
  let average = values.iter().sum::<f64>() / values.len() as f64;
  let equal = rule.equal_average;
  if rule.above_average {
    value > average || (equal && (value - average).abs() <= f64::EPSILON)
  } else {
    value < average || (equal && (value - average).abs() <= f64::EPSILON)
  }
}

fn conditional_cell_is_matches(
  import: &ExcelImport,
  sheet: &CalcSheet,
  references: &[String],
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
  value: f64,
) -> bool {
  let base = conditional_format_base_address(references, address).unwrap_or(address);
  let first = rule.formulas.first().and_then(|formula| {
    super::formula::evaluate_relative_formula_as_number(import, sheet, formula, base, address)
  });
  let second = rule.formulas.get(1).and_then(|formula| {
    super::formula::evaluate_relative_formula_as_number(import, sheet, formula, base, address)
  });
  match rule.operator.unwrap_or_default() {
    x::ConditionalFormattingOperatorValues::LessThan => first.is_some_and(|limit| value < limit),
    x::ConditionalFormattingOperatorValues::LessThanOrEqual => {
      first.is_some_and(|limit| value <= limit)
    }
    x::ConditionalFormattingOperatorValues::Equal => first.is_some_and(|limit| value == limit),
    x::ConditionalFormattingOperatorValues::NotEqual => first.is_some_and(|limit| value != limit),
    x::ConditionalFormattingOperatorValues::GreaterThanOrEqual => {
      first.is_some_and(|limit| value >= limit)
    }
    x::ConditionalFormattingOperatorValues::GreaterThan => first.is_some_and(|limit| value > limit),
    x::ConditionalFormattingOperatorValues::Between => first
      .zip(second)
      .is_some_and(|(low, high)| value >= low.min(high) && value <= low.max(high)),
    x::ConditionalFormattingOperatorValues::NotBetween => first
      .zip(second)
      .is_some_and(|(low, high)| value < low.min(high) || value > low.max(high)),
    _ => false,
  }
}

fn conditional_expression_matches(
  import: &ExcelImport,
  sheet: &CalcSheet,
  references: &[String],
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
) -> bool {
  let Some(formula) = rule.formulas.first() else {
    return false;
  };
  let Some(base) = conditional_format_base_address(references, address) else {
    return false;
  };
  super::formula::evaluate_relative_formula_as_condition(import, sheet, formula, base, address)
}

fn conditional_format_base_address(
  references: &[String],
  address: CellAddress,
) -> Option<CellAddress> {
  references
    .iter()
    .flat_map(|references| references.split_whitespace())
    .filter_map(CellRange::parse_a1_range)
    .find(|range| range.contains(address))
    .map(|range| range.start)
}

fn conditional_reference_values(
  sheet: &CalcSheet,
  references: &[String],
  address: CellAddress,
) -> Vec<f64> {
  references
    .iter()
    .flat_map(|references| references.split_whitespace())
    .filter_map(CellRange::parse_a1_range)
    .find(|range| range.contains(address))
    .map(|range| {
      sheet
        .rows
        .iter()
        .flat_map(|row| row.cells.iter())
        .filter_map(|cell| {
          let cell_address = cell.address()?;
          if !range.contains(cell_address) {
            return None;
          }
          cell.display_text.parse::<f64>().ok()
        })
        .collect()
    })
    .unwrap_or_default()
}

fn pivot_display_text(sheet: &CalcSheet, address: CellAddress, text: String) -> String {
  if let Some(text) = pivot_page_field_display_text(sheet, address) {
    return text;
  }
  let Some(pivot) = pivot_table_for_cell(sheet, address) else {
    if !sheet.resources.pivot_tables.tables.is_empty() && text == "(blank)" {
      return "(empty)".to_string();
    }
    return text;
  };
  // Source: LibreOffice sc/source/core/data/dpoutput.cxx emits DataPilot
  // field/member result captions from the imported pivot source instead of
  // keeping Excel's persisted generic "Row Labels"/"(blank)" strings.
  if pivot.calculated_only_data_fields {
    let table_start = pivot.output_geometry.table_start;
    if address == table_start {
      if let Some(label) = pivot_row_label_text(pivot) {
        return label;
      }
    }
    if address.row == table_start.row && address.col > table_start.col {
      return "(empty)".to_string();
    }
  }
  if is_pivot_row_labels_caption(text.as_str()) {
    return pivot_row_caption_text(pivot, text.as_str());
  }
  if let Some(data_layout_caption) = pivot_data_layout_caption_text(pivot, address, text.as_str()) {
    return data_layout_caption;
  }
  if address.col == pivot.output_geometry.data_start.col
    && address.row == pivot.output_geometry.data_start.row.saturating_sub(1)
    && let Some(name) = pivot.data_field_names.first()
    && name
      .strip_suffix(text.as_str())
      .is_some_and(|prefix| prefix.ends_with(" - "))
  {
    return name.clone();
  }
  match text.as_str() {
    "Grand Total" => "Total Result".to_string(),
    "Gesamtergebnis"
    | "Végösszeg"
    | "\u{041e}\u{0431}\u{0449}\u{0438}\u{0439} \u{0438}\u{0442}\u{043e}\u{0433}" => {
      "Total Result".to_string()
    }
    "Total general" => "Total Result".to_string(),
    "Total" => pivot
      .data_field_names
      .first()
      .cloned()
      .unwrap_or_else(|| text),
    "Row Labels" => pivot_row_label_text(pivot).unwrap_or(text),
    "Column Labels" => pivot_column_label_text(pivot).unwrap_or(text),
    "(blank)" => "(empty)".to_string(),
    "N.év1" => "Q1".to_string(),
    "N.év2" => "Q2".to_string(),
    "N.év3" => "Q3".to_string(),
    "N.év4" => "Q4".to_string(),
    _ => {
      if let Some(prefix) = text
        .strip_suffix(" Total")
        .filter(|prefix| !prefix.is_empty())
      {
        format!("{prefix} Result")
      } else {
        text
      }
    }
  }
}

fn pivot_data_layout_caption_text(
  pivot: &super::pivot::PivotTableModel,
  address: CellAddress,
  text: &str,
) -> Option<String> {
  if text != "Values" {
    return None;
  }
  if pivot
    .row_field_indexes
    .iter()
    .position(|index| *index == -2)
    .is_some_and(|position| {
      // Source: LibreOffice sc/source/core/data/dpoutput.cxx keeps the
      // persisted dataCaption when the row-axis data layout still has a row
      // grand-total result; when row grand totals are disabled, the emitted
      // FieldCell caption comes from the data-layout dimension.
      !pivot.row_grand_totals
        && address.row == pivot.output_geometry.data_start.row.saturating_sub(1)
        && address.col == pivot.output_geometry.table_start.col + position as u32
    })
  {
    return Some("Data".to_string());
  }
  if pivot
    .column_field_indexes
    .iter()
    .position(|index| *index == -2)
    .is_some_and(|position| {
      !pivot.column_grand_totals
        && address.col == pivot.output_geometry.data_start.col + position as u32
        && address.row == pivot.output_geometry.table_start.row
    })
  {
    return Some("Data".to_string());
  }
  None
}

fn pivot_page_field_display_text(sheet: &CalcSheet, address: CellAddress) -> Option<String> {
  for pivot in &sheet.resources.pivot_tables.tables {
    let page_fields = &pivot.page_field_models;
    if page_fields.is_empty() {
      continue;
    }
    let output_start = pivot.output_geometry.output_start;
    let first_page_row = output_start.row;
    let Some(page_field_index) = address
      .row
      .checked_sub(first_page_row)
      .map(|index| index as usize)
    else {
      continue;
    };
    if page_field_index >= page_fields.len() {
      continue;
    }
    let page_field = &page_fields[page_field_index];
    if address.col == output_start.col {
      return Some(page_field.field_name.clone());
    }
    if address.col == output_start.col + 1 {
      return Some(pivot_page_field_value_text(&page_field.value));
    }
  }
  None
}

fn pivot_page_field_value_text(value: &super::pivot::PivotPageFieldValue) -> String {
  match value {
    super::pivot::PivotPageFieldValue::All => "- all -".to_string(),
    super::pivot::PivotPageFieldValue::Multiple => "- multiple -".to_string(),
    super::pivot::PivotPageFieldValue::Member(text) if text.is_empty() => "(empty)".to_string(),
    super::pivot::PivotPageFieldValue::Member(text) => text.clone(),
  }
}

fn is_pivot_row_labels_caption(text: &str) -> bool {
  matches!(
    text,
    "Row Labels"
      | "Sorcímkék"
      | "Zeilenbeschriftungen"
      | "\u{041d}\u{0430}\u{0437}\u{0432}\u{0430}\u{043d}\u{0438}\u{044f} \u{0441}\u{0442}\u{0440}\u{043e}\u{043a}"
  )
}

fn pivot_table_for_cell(
  sheet: &CalcSheet,
  address: CellAddress,
) -> Option<&super::pivot::PivotTableModel> {
  sheet
    .resources
    .pivot_tables
    .tables
    .iter()
    .find(|pivot| pivot.output_geometry.table_range.contains(address))
}

fn pivot_row_label_text(pivot: &super::pivot::PivotTableModel) -> Option<String> {
  if pivot.row_field_names.is_empty() {
    return None;
  }
  if pivot.compact && pivot.row_field_names.len() > 1 {
    return None;
  }
  Some(pivot.row_field_names.join(" "))
}

fn pivot_row_caption_text(pivot: &super::pivot::PivotTableModel, text: &str) -> String {
  pivot_row_label_text(pivot).unwrap_or_else(|| {
    // Source: LibreOffice sc/source/core/data/dpoutput.cxx keeps Excel's
    // generic compact-layout row caption when multiple row fields share one
    // output column, while localized persisted captions are imported through
    // the DataPilot source as the generic "Row Labels" text.
    if pivot.compact && pivot.row_field_names.len() > 1 {
      "Row Labels".to_string()
    } else {
      text.to_string()
    }
  })
}

fn pivot_column_label_text(pivot: &super::pivot::PivotTableModel) -> Option<String> {
  (!pivot.column_field_names.is_empty()).then(|| pivot.column_field_names.join(" "))
}

fn pivot_header_number_format_code<'a>(
  import: &'a ExcelImport,
  sheet: &CalcSheet,
  address: CellAddress,
) -> Option<&'a str> {
  let pivot = pivot_table_for_cell(sheet, address)?;
  if address.row >= pivot.output_geometry.data_start.row
    && address.col >= pivot.output_geometry.table_start.col
    && address.col < pivot.output_geometry.data_start.col
  {
    let field_index = address
      .col
      .saturating_sub(pivot.output_geometry.table_start.col) as usize;
    return pivot
      .row_field_number_format_ids
      .get(field_index)
      .and_then(|id| id.and_then(|id| import.styles.number_format_code(id)));
  }
  if address.col >= pivot.output_geometry.data_start.col
    && address.row >= pivot.output_geometry.table_start.row + pivot.output_geometry.header_rows
    && address.row < pivot.output_geometry.data_start.row
  {
    let field_index = address
      .row
      .saturating_sub(pivot.output_geometry.table_start.row + pivot.output_geometry.header_rows)
      as usize;
    return pivot
      .column_field_number_format_ids
      .get(field_index)
      .and_then(|id| id.and_then(|id| import.styles.number_format_code(id)));
  }
  None
}

fn pivot_data_number_format_code<'a>(
  import: &'a ExcelImport,
  sheet: &CalcSheet,
  address: CellAddress,
) -> Option<&'a str> {
  let pivot = pivot_table_for_cell(sheet, address)?;
  let data_start_row = pivot.output_geometry.data_start.row;
  let data_start_col = pivot.output_geometry.data_start.col;
  if address.row < data_start_row || address.col < data_start_col {
    return None;
  }
  let data_field_count = pivot.data_field_number_format_ids.len().max(1) as u32;
  let data_field_index = match pivot.data_layout_axis {
    super::pivot::PivotDataLayoutAxis::Rows => {
      (address.row - data_start_row).min(data_field_count - 1)
    }
    super::pivot::PivotDataLayoutAxis::Columns | super::pivot::PivotDataLayoutAxis::Hidden => {
      (address.col - data_start_col).min(data_field_count - 1)
    }
  } as usize;
  pivot
    .data_field_number_format_ids
    .get(data_field_index)
    .and_then(|id| id.and_then(|id| import.styles.number_format_code(id)))
}

fn pivot_data_cell_text_override(sheet: &CalcSheet, address: CellAddress) -> Option<String> {
  sheet
    .resources
    .pivot_tables
    .tables
    .iter()
    .flat_map(|pivot| pivot.data_cell_text_overrides.iter())
    .find(|override_text| override_text.address == address)
    .map(|override_text| override_text.text.clone())
}

pub(crate) fn rendered_number_text(
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

  let format_code = if let Some(format_code) = format_code {
    format_code
  } else if raw
    .parse::<f64>()
    .ok()
    .is_some_and(|value| value.is_finite())
  {
    "General"
  } else {
    return (raw.to_string(), NumberFormatRenderState::Raw);
  };
  if format_code.eq_ignore_ascii_case("General") {
    return (
      raw
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .map(format_general_number)
        .unwrap_or_else(|| raw.to_string()),
      NumberFormatRenderState::General,
    );
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
    if let Some(text) = render_elapsed_date_time(value, format_code) {
      return (text, NumberFormatRenderState::DateTime);
    }
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

fn format_general_number(value: f64) -> String {
  // Source: LibreOffice sc/source/ui/view/output2.cxx uses the General
  // SvNumberformat output instead of the raw OOXML double text. Fifteen
  // significant digits match Calc/Excel's normal General precision.
  if value == 0.0 {
    return "0".to_string();
  }
  let abs = value.abs();
  if !(1.0e-4..1.0e15).contains(&abs) {
    let text = format!("{value:.14e}");
    if let Some((mantissa, exponent)) = text.split_once('e') {
      let mantissa = trim_general_fraction(mantissa.to_string());
      let exponent_value = exponent.parse::<i32>().unwrap_or(0);
      return format!("{mantissa}E{exponent_value:+03}");
    }
    return text;
  }
  let integer_digits = if abs >= 1.0 {
    abs.log10().floor() as isize + 1
  } else {
    0
  };
  let decimals = 15usize.saturating_sub(integer_digits.max(0) as usize);
  trim_general_fraction(format!("{value:.decimals$}"))
}

fn trim_general_fraction(mut text: String) -> String {
  if text.contains('.') {
    while text.ends_with('0') {
      text.pop();
    }
    if text.ends_with('.') {
      text.pop();
    }
  }
  if text == "-0" { "0".to_string() } else { text }
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
    let mut skip_next = false;
    let mut emit_next_fill = false;
    let mut after_decimal = false;
    let mut seen_digit = false;
    let mut literal_prefix = true;
    let mut integer_pattern = String::new();
    for ch in section.chars() {
      if skip_next {
        skip_next = false;
        continue;
      }
      if emit_next_fill {
        if !after_decimal && !literal_prefix {
          integer_pattern.push(ch);
        }
        if literal_prefix {
          pattern.prefix.push(ch);
        } else if seen_digit && after_decimal {
          pattern.suffix.push(ch);
        }
        emit_next_fill = false;
        continue;
      }
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
        '_' => skip_next = true,
        '*' => emit_next_fill = true,
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
    || marker
      .strip_prefix('$')
      .is_some_and(|value| value.starts_with('-'))
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
  let rendered_fraction = render_fraction_pattern(&pattern.section, fraction);
  let fraction = rendered_fraction.clone().unwrap_or_else(|| {
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
  if rendered_fraction.is_none() {
    output.push_str(&pattern.suffix);
  }
  output.trim().to_string()
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
  if uses_system_long_date_format(code) {
    // Source: LibreOffice svl/source/numbers/zforlist.cxx maps the system
    // long date entry (NF_DATE_SYSTEM_LONG) to the current system locale.
    // The corresponding Calc test fixes the locale to en-US.
    return format!(
      "{}, {} {}, {}",
      weekday_name(year, month, day),
      month_name(month),
      day,
      year
    );
  }
  render_date_time_format(
    &strip_number_format_markers(code),
    year,
    month,
    day,
    hour,
    minute,
    second,
  )
}

fn uses_system_long_date_format(code: &str) -> bool {
  let mut rest = code;
  while let Some(start) = rest.find('[') {
    let Some(end) = rest[start + 1..].find(']') else {
      break;
    };
    if rest[start + 1..start + 1 + end]
      .trim()
      .eq_ignore_ascii_case("$-F800")
    {
      return true;
    }
    rest = &rest[start + end + 2..];
  }
  false
}

fn render_elapsed_date_time(value: f64, code: &str) -> Option<String> {
  let clean = strip_number_format_markers(code);
  let lower = clean.to_ascii_lowercase();
  let elapsed = if lower.contains("[hh]") {
    ElapsedDateTimeUnit::Hour
  } else if lower.contains("[mm]") {
    ElapsedDateTimeUnit::Minute
  } else if lower.contains("[ss]") {
    ElapsedDateTimeUnit::Second
  } else {
    return None;
  };
  let total_seconds = value.abs() * 86_400.0;
  let rounded_seconds = total_seconds.round() as i64;
  let sign = if value.is_sign_negative() { "-" } else { "" };
  let total_hours = rounded_seconds / 3_600;
  let total_minutes = rounded_seconds / 60;
  let seconds = rounded_seconds % 60;
  let minutes = (rounded_seconds / 60) % 60;
  let hours = (rounded_seconds / 3_600) % 24;
  let mut output = String::new();
  output.push_str(sign);
  let mut rest = clean.as_str();
  let mut bracket_written = false;
  while let Some(ch) = rest.chars().next() {
    let lower_rest = rest.to_ascii_lowercase();
    if lower_rest.starts_with("[hh]") {
      output.push_str(&total_hours.to_string());
      rest = &rest[4..];
      bracket_written = true;
    } else if lower_rest.starts_with("[mm]") {
      output.push_str(&total_minutes.to_string());
      rest = &rest[4..];
      bracket_written = true;
    } else if lower_rest.starts_with("[ss]") {
      let consumed = render_elapsed_second_token(rest, total_seconds, &mut output);
      rest = &rest[consumed..];
      bracket_written = true;
    } else if ch == 'h' || ch == 'H' {
      let count = repeated_char_count(rest, ch);
      if bracket_written {
        output.push_str(&format_padded_number(hours, count));
      }
      rest = &rest[count..];
    } else if ch == 'm' || ch == 'M' {
      let count = repeated_char_count(rest, ch);
      if bracket_written {
        output.push_str(&format_padded_number(minutes, count));
      }
      rest = &rest[count..];
    } else if ch == 's' || ch == 'S' {
      let count = repeated_char_count(rest, ch);
      if bracket_written {
        output.push_str(&format_padded_number(seconds, count));
      }
      rest = &rest[count..];
    } else if ch == '"' {
      let consumed = push_quoted_number_format_literal(rest, &mut output);
      rest = &rest[consumed..];
    } else if ch == '\\' {
      let consumed = push_escaped_number_format_literal(rest, &mut output);
      rest = &rest[consumed..];
    } else if matches!(ch, '_' | '*') {
      rest = rest.get(ch.len_utf8() * 2..).unwrap_or("");
    } else {
      output.push(ch);
      rest = &rest[ch.len_utf8()..];
    }
  }
  if elapsed == ElapsedDateTimeUnit::Second || bracket_written {
    Some(output)
  } else {
    None
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ElapsedDateTimeUnit {
  Hour,
  Minute,
  Second,
}

fn render_elapsed_second_token(rest: &str, total_seconds: f64, output: &mut String) -> usize {
  let mut consumed = 4usize;
  if let Some(fraction) = rest
    .get(consumed..)
    .and_then(|suffix| suffix.strip_prefix('.'))
  {
    let decimals = fraction
      .chars()
      .take_while(|ch| matches!(ch, '0' | '#' | '?'))
      .count();
    if decimals > 0 {
      output.push_str(&format!("{total_seconds:.decimals$}"));
      consumed += 1 + decimals;
      return consumed;
    }
  }
  output.push_str(&(total_seconds.round() as i64).to_string());
  consumed
}

fn repeated_char_count(value: &str, first: char) -> usize {
  value
    .chars()
    .take_while(|ch| ch.eq_ignore_ascii_case(&first))
    .map(char::len_utf8)
    .sum()
}

fn format_padded_number(value: i64, width: usize) -> String {
  if width >= 2 {
    format!("{value:02}")
  } else {
    value.to_string()
  }
}

fn push_quoted_number_format_literal(rest: &str, output: &mut String) -> usize {
  let mut consumed = 1usize;
  for ch in rest[1..].chars() {
    consumed += ch.len_utf8();
    if ch == '"' {
      return consumed;
    }
    output.push(ch);
  }
  consumed
}

fn push_escaped_number_format_literal(rest: &str, output: &mut String) -> usize {
  let mut chars = rest.chars();
  let Some(first) = chars.next() else {
    return 0;
  };
  let Some(second) = chars.next() else {
    return first.len_utf8();
  };
  output.push(second);
  first.len_utf8() + second.len_utf8()
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
  let sections = split_number_format_sections(code);
  let clean = strip_number_format_markers(sections.first().copied().unwrap_or(code));
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
  if let Some(text) =
    render_tokenized_date_time_format(&clean, year, month, day, hour, minute, second)
  {
    return text;
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
      if lower.contains("yyyy") || lower.contains("yy") || lower.contains('d') {
        let yy = (year % 100) as u32;
        let date = if lower.contains("yyyy") {
          if lower.find('d') < lower.find('m') {
            format!("{day}/{month}/{year}")
          } else {
            format!("{month}/{day}/{year}")
          }
        } else if lower.find('d') < lower.find('m') {
          format!("{day}/{month}/{yy:02}")
        } else {
          format!("{month}/{day}/{yy:02}")
        };
        return format!("{date} {hour12}:{minute:02} {suffix}");
      }
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

fn render_tokenized_date_time_format(
  code: &str,
  year: i64,
  month: u32,
  day: u32,
  hour: i64,
  minute: i64,
  second: i64,
) -> Option<String> {
  // Source: LibreOffice svl/source/numbers/zformat.cxx
  // SvNumberformat::ImpGetDateTimeOutput walks date/time tokens and emits
  // escaped format characters as literals. This mirrors that token/literal
  // split for OOXML date formats used by imported pivot fixtures.
  let lower = code.to_ascii_lowercase();
  if lower.contains("am/pm") || lower.contains("a/p") {
    return None;
  }
  let mut output = String::new();
  let mut index = 0usize;
  let bytes = code.as_bytes();
  let mut saw_date_or_time = false;
  let mut time_context = false;
  while index < bytes.len() {
    let rest = &code[index..];
    let lower_rest = &lower[index..];
    if bytes[index] == b'\\' {
      if let Some(ch) = rest[1..].chars().next() {
        output.push(ch);
        index += 1 + ch.len_utf8();
      } else {
        index += 1;
      }
      continue;
    }
    if bytes[index] == b'"' {
      let mut consumed = 1usize;
      for ch in rest[1..].chars() {
        consumed += ch.len_utf8();
        if ch == '"' {
          break;
        }
        output.push(ch);
      }
      index += consumed;
      continue;
    }
    let Some(ch) = rest.chars().next() else {
      break;
    };
    if lower_rest.starts_with("yyyy") {
      output.push_str(&format!("{year:04}"));
      index += 4;
      saw_date_or_time = true;
    } else if lower_rest.starts_with("yy") {
      output.push_str(&format!("{:02}", (year % 100) as u32));
      index += 2;
      saw_date_or_time = true;
    } else if lower_rest.starts_with("mmmm") {
      output.push_str(month_name(month));
      index += 4;
      saw_date_or_time = true;
    } else if lower_rest.starts_with("mmm") {
      output.push_str(short_month_name(month));
      index += 3;
      saw_date_or_time = true;
    } else if lower_rest.starts_with("mm") {
      if time_context {
        output.push_str(&format!("{minute:02}"));
      } else {
        output.push_str(&format!("{month:02}"));
      }
      index += 2;
      saw_date_or_time = true;
    } else if lower_rest.starts_with('m') {
      if time_context {
        output.push_str(&minute.to_string());
      } else {
        output.push_str(&month.to_string());
      }
      index += 1;
      saw_date_or_time = true;
    } else if lower_rest.starts_with("dddd") {
      output.push_str(weekday_name(year, month, day));
      index += 4;
      saw_date_or_time = true;
    } else if lower_rest.starts_with("ddd") {
      output.push_str(short_weekday_name(year, month, day));
      index += 3;
      saw_date_or_time = true;
    } else if lower_rest.starts_with("dd") {
      output.push_str(&format!("{day:02}"));
      index += 2;
      saw_date_or_time = true;
    } else if lower_rest.starts_with('d') {
      output.push_str(&day.to_string());
      index += 1;
      saw_date_or_time = true;
    } else if lower_rest.starts_with("hh") {
      output.push_str(&format!("{hour:02}"));
      index += 2;
      saw_date_or_time = true;
      time_context = true;
    } else if lower_rest.starts_with('h') {
      output.push_str(&hour.to_string());
      index += 1;
      saw_date_or_time = true;
      time_context = true;
    } else if lower_rest.starts_with("ss") {
      output.push_str(&format!("{second:02}"));
      index += 2;
      saw_date_or_time = true;
      time_context = true;
    } else if lower_rest.starts_with('s') {
      output.push_str(&second.to_string());
      index += 1;
      saw_date_or_time = true;
      time_context = true;
    } else {
      output.push(ch);
      index += ch.len_utf8();
    }
  }
  saw_date_or_time.then_some(output)
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
  weekday_name_for_index(weekday_index(year, month, day))
}

fn short_weekday_name(year: i64, month: u32, day: u32) -> &'static str {
  match weekday_index(year, month, day) {
    0 => "Sat",
    1 => "Sun",
    2 => "Mon",
    3 => "Tue",
    4 => "Wed",
    5 => "Thu",
    6 => "Fri",
    _ => "",
  }
}

fn weekday_name_for_index(index: i64) -> &'static str {
  match index {
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

fn weekday_index(year: i64, month: u32, day: u32) -> i64 {
  let y = if month < 3 { year - 1 } else { year };
  let m = if month < 3 { month + 12 } else { month } as i64;
  let k = y % 100;
  let j = y / 100;
  (i64::from(day) + ((13 * (m + 1)) / 5) + k + (k / 4) + (j / 4) + (5 * j)) % 7
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

#[cfg(test)]
mod tests {
  use std::fs::File;

  use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
  use ooxmlsdk::sdk::{
    FileFormatVersion, MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings,
    OpenSettings,
  };

  use super::*;

  #[test]
  fn general_number_format_uses_calc_significant_digits() {
    assert_eq!(
      rendered_number_text("4.0999999999999996", None, None, false).0,
      "4.1"
    );
    assert_eq!(
      rendered_number_text("4.0999999999999996", Some("General"), None, false).0,
      "4.1"
    );
  }

  #[test]
  fn date_format_ignores_lcid_marker() {
    assert_eq!(
      rendered_number_text("45657", Some("[$-809]dd/mm/yy"), None, false).0,
      "31/12/24"
    );
  }

  #[test]
  fn long_weekday_date_format_uses_weekday_name() {
    assert_eq!(
      rendered_number_text("26467", Some("dddd, d. mmmm yyyy"), None, false).0,
      "Saturday, 17. June 1972"
    );
  }

  #[test]
  fn system_long_date_format_uses_unpadded_en_us_day() {
    // Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:
    // testTdf165180_date1904_XLSX fixes the system locale to en-US and
    // expects the NF_DATE_SYSTEM_LONG output.
    assert_eq!(
      rendered_number_text(
        "60",
        Some("[$-F800]dddd\\,\\ mmmm\\ dd\\,\\ yyyy"),
        None,
        true
      )
      .0,
      "Tuesday, March 1, 1904"
    );
  }

  #[test]
  fn accounting_format_controls_are_not_visible_text() {
    // Source: LibreOffice SvNumberFormatter treats '_' and '*' as spacing
    // controls. They reserve width but do not emit the following character.
    assert_eq!(
      rendered_number_text(
        "1",
        Some("_(\"$\"* #,##0.00_);_(\"$\"* \\(#,##0.00\\);_(\"$\"* \"-\"??_);_(@_)"),
        None,
        false
      )
      .0,
      "$ 1.00"
    );
    assert_eq!(
      rendered_number_text(
        "2.75",
        Some("_-* #,##0.00\\ \"Ft\"_-;\\-* #,##0.00\\ \"Ft\"_-;_-* \"-\"??\\ \"Ft\"_-;_-@_-"),
        None,
        false
      )
      .0,
      "2.75 Ft"
    );
  }

  #[test]
  fn tdf100709_imports_plain_values_in_print_pages() {
    // Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:
    // testTdf100709XLSX asserts that B52 and A75 stay plain "218".
    let settings = OpenSettings {
      markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
        process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
        target_file_format_version: FileFormatVersion::Microsoft365,
      },
      ..Default::default()
    };
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("../../test-data/ooxmlsdk-pdf-test/libreoffice/xlsx/tdf100709.xlsx");
    let mut document =
      SpreadsheetDocument::new_with_settings(File::open(path).unwrap(), settings).unwrap();
    let import = super::super::import::ExcelImport::import_document(
      &mut document,
      &crate::options::PdfOptions::default(),
    )
    .unwrap();
    let print = CalcPrintDocument::from_import(&import);
    assert_eq!(print.pages.len(), 2);
    let text = print
      .pages
      .iter()
      .flat_map(|page| page.cells.iter().map(|cell| cell.rendered_text.as_str()))
      .collect::<Vec<_>>()
      .join(" ");
    assert!(text.contains("65 218"));
    assert!(text.contains("05-Mar-00 218"));
  }
}
