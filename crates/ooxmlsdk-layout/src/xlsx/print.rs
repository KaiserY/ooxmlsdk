use std::borrow::Cow;
use std::collections::{HashMap, HashSet};

use super::import::ExcelImport;
use super::model::split_defined_name_ranges;
use super::page_settings::CalcPageSettings;
use super::pivot::pivot_print_address;
use super::styles::DefinedNameBuiltin;
use super::worksheet::{CalcCell, CalcRow, CalcSheet, CellAddress, CellRange, SheetType};
use crate::localization::OfficeStringCatalog;
use crate::model::{RgbColor, TextStyle};
use crate::text_metrics::TextMetrics;
use crate::units;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk_formula::calc::numeric::round_to_decimal_places;
const ZOOM_MIN: u32 = 10;
// Excel's indexed-scatter printer profile retains one four-pixel band at the
// authored 300dpi in each horizontal page clip. The band extends the paint
// clip; it does not change the column pagination or drawing origin.
pub(super) const INDEXED_SCATTER_HORIZONTAL_CLIP_EXTENSION_PT: f32 = 0.96;
const XLSX_MAX_COLUMN: u32 = 16_384;
const XLSX_MAX_ROW: u32 = 1_048_576;
const CALC_CELL_TEXT_MARGIN_PT: f32 = 4.0;
// LibreOffice sc/source/ui/view/printfun.cxx::ScPrintFunc::CalcZoom applies
// this interoperability adjustment for width-only fit-to-page output.
const LIBREOFFICE_FIT_TO_WIDTH_ZOOM_FACTOR: f32 = 0.98;
// LibreOffice sc/source/core/data/attarray.cxx::SC_VISATTR_STOP.
const SC_VISATTR_STOP: u32 = 84;

pub(super) fn fixed_output_content_scale(
  worksheet_scale_percent: u32,
  paper_scale_percent: u32,
) -> f32 {
  worksheet_scale_percent as f32 * paper_scale_percent as f32 / 10_000.0
}

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintDocument<'a> {
  pub(crate) pages: Vec<CalcPrintPage<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintPage<'a> {
  pub(crate) sheet: &'a CalcSheet,
  pub(crate) sheet_page_index: usize,
  pub(crate) starts_print_area_row: bool,
  pub(crate) has_explicit_print_area: bool,
  pub(crate) page_number: usize,
  pub(crate) total_pages: usize,
  /// Worksheet print zoom before the fixed-output paper transform.
  pub(crate) zoom: u32,
  pub(crate) paper_scale_percent: u32,
  pub(crate) pagination_paper_scale_percent: u32,
  pub(crate) page_settings: &'a CalcPageSettings,
  pub(crate) area: Option<CellRange>,
  pub(crate) repeated_rows: Option<CellRange>,
  pub(crate) repeated_columns: Option<CellRange>,
  pub(crate) cells: Vec<CalcPrintCell<'a>>,
  pub(crate) repeated_row_cells: Vec<CalcPrintCell<'a>>,
  pub(crate) repeated_column_cells: Vec<CalcPrintCell<'a>>,
  pub(crate) repeated_corner_cells: Vec<CalcPrintCell<'a>>,
  pub(crate) drawing_anchor_count: usize,
  pub(crate) chart_count: usize,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CalcPrintNamedRanges {
  pub(crate) resolved_print_areas: Vec<CellRange>,
  pub(crate) repeat_rows: Option<CellRange>,
  pub(crate) repeat_columns: Option<CellRange>,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintCell<'a> {
  pub(crate) address: CellAddress,
  pub(crate) text: Cow<'a, str>,
  pub(crate) data_type: Option<x::CellValues>,
  pub(crate) style_index: Option<u32>,
  pub(crate) pivot_format_id: Option<u32>,
  pub(crate) rendered_text: String,
  pub(crate) rich_text_runs: &'a [super::workbook::SharedStringRun],
  pub(crate) number_format_state: NumberFormatRenderState,
  pub(crate) number_format_color: Option<RgbColor>,
  pub(crate) number_format_layout: Option<Vec<NumberFormatLayoutItem>>,
  pub(crate) formula: bool,
  pub(crate) icon_set: Option<CalcPrintIconSet>,
  pub(crate) data_bar: Option<CalcPrintDataBar>,
  pub(crate) color_scale_fill: Option<CalcPrintColorScaleFill>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CalcPrintDataBar {
  pub(crate) start: f32,
  pub(crate) end: f32,
  pub(crate) axis: Option<f32>,
  pub(crate) color: RgbColor,
  pub(crate) border_color: Option<RgbColor>,
  pub(crate) axis_color: RgbColor,
  pub(crate) gradient: bool,
  pub(crate) show_value: bool,
  pub(crate) direction: ooxmlsdk::schemas::x14::DataBarDirectionValues,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CalcPrintIconSet {
  pub(crate) icon: Option<(super::sheet_conditions::IconSetType, usize)>,
  pub(crate) show_value: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct CalcPrintColorScaleFill {
  pub(crate) priority: i32,
  pub(crate) color: RgbColor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NumberFormatRenderState {
  Raw,
  General,
  Text,
  Boolean,
  Error,
  Number,
  Percent,
  DateTime,
  UnsupportedFormatCode,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum NumberFormatLayoutItem {
  Text(String),
  Reserve(char),
  Fill(char),
}

#[derive(Clone, Debug, Default)]
struct CalcPrintDrawingSummary {
  anchors: usize,
  charts: usize,
}

impl<'a> CalcPrintDocument<'a> {
  pub(crate) fn from_import(import: &'a ExcelImport) -> Self {
    // This is the first ScPrintFunc-shaped owner. Full range, break, and page
    // count logic lands here; display only consumes the resulting print pages.
    let mut pages = Vec::new();
    let mut text_metrics = TextMetrics::new();
    for sheet in import.sheets.iter().filter(|sheet| sheet.visible()) {
      let mut conditional_eval_cache = ConditionalFormatEvalCache::default();
      let named_ranges = CalcPrintNamedRanges::from_import(import, sheet);
      let ordinary_used_range = sheet.used_range(&import.styles);
      let areas = print_areas_for_sheet(import, sheet, &named_ranges, &mut text_metrics);
      let scale = print_scale_state(import, sheet, &areas, &named_ranges, &mut text_metrics);
      let uses_formatted_implicit_header_footer_extent =
        implicit_header_footer_uses_formatted_cell_extent(import, sheet, &named_ranges);
      // An explicit print area gives an otherwise empty worksheet printable
      // pages. Office preserves separate areas and manual page breaks even
      // without cell paint, headings, or headers/footers. An implicit blank
      // sheet still has no printable pages.
      let keep_empty_explicit_sheet_pages =
        !named_ranges.resolved_print_areas.is_empty() && sheet_body_is_empty(import, sheet);
      let page_areas = page_areas_for_sheet(
        import,
        sheet,
        &areas,
        &named_ranges,
        fixed_output_content_scale(scale.zoom, scale.pagination_paper_scale_percent) * 100.0,
        scale.top_down,
        &mut text_metrics,
      );
      let page_states = page_areas
        .iter()
        .map(|area| {
          let drawing_summary = drawing_summary_for_area(sheet, *area);
          let empty = area
            .is_some_and(|area| print_area_is_empty(import, sheet, area, &mut text_metrics))
            && drawing_summary.anchors == 0
            && drawing_summary.charts == 0;
          (drawing_summary, empty)
        })
        .collect::<Vec<_>>();
      let last_implicit_printable_page = named_ranges
        .resolved_print_areas
        .is_empty()
        .then(|| page_states.iter().rposition(|(_, empty)| !empty))
        .flatten();
      let mut sheet_page_index = 0usize;
      for (page_area_index, (area, (drawing_summary, empty))) in
        page_areas.into_iter().zip(page_states).enumerate()
      {
        let starts_print_area_row = area.is_some_and(|page_area| {
          areas.iter().any(|print_area| {
            print_area.contains(page_area.start) && page_area.start.row == print_area.start.row
          })
        });
        let cells = area
          .map(|area| {
            print_cells_for_area(
              import,
              sheet,
              page_cell_scan_area(area),
              &mut conditional_eval_cache,
            )
          })
          .unwrap_or_default();
        let repeated_row_cells = repeat_rows_for_page(area, named_ranges.repeat_rows)
          .map(|area| print_cells_for_area(import, sheet, area, &mut conditional_eval_cache))
          .unwrap_or_default();
        let repeated_column_cells = repeat_columns_for_page(area, named_ranges.repeat_columns)
          .map(|area| print_cells_for_area(import, sheet, area, &mut conditional_eval_cache))
          .unwrap_or_default();
        let repeated_corner_cells =
          repeat_corner_for_page(area, named_ranges.repeat_rows, named_ranges.repeat_columns)
            .map(|area| print_cells_for_area(import, sheet, area, &mut conditional_eval_cache))
            .unwrap_or_default();
        // ScPrintFunc::DoPrint. Empty sheet page ranges are hidden by
        // ScDocument::IsPrintEmpty before PrintPage is called; header/footer
        // content is painted only for page ranges that survive that test.
        // Excel does not create an implicit print page solely for headers or
        // footers (empty-noconf.xlsx and WorkbookProperties.xlsx). Keep the
        // explicit empty-sheet print areas separate from this implicit case.
        // Excel fixed output retains blank pages ahead of later printable
        // cells or drawings in the actual page order. Ordinary implicit
        // ranges exclude invisible blank-cell XF metadata, so a style-only
        // tail such as tdf131536 does not create pages. A nonempty sheet with
        // a configured header/footer uses the separately selected formatted-
        // cell extent, but Office does not retain every empty rectangle in
        // that Cartesian extent. Its fixed output keeps a horizontal
        // continuation only when that page contains a directly authored XF
        // delta. Vertical style tails, cross-product holes, and default-
        // equivalent XFs remain hidden.
        let implicit_page_before_content = last_implicit_printable_page
          .is_some_and(|last_printable| page_area_index < last_printable);
        let keep_formatted_horizontal_header_footer_page = area.is_some_and(|area| {
          keep_formatted_horizontal_header_footer_page(
            uses_formatted_implicit_header_footer_extent,
            ordinary_used_range,
            area,
            sheet.has_nondefault_direct_cell_formatting_in_range(&import.styles, area),
          )
        });
        if should_skip_empty_print_page(
          scale.skip_empty,
          empty,
          implicit_page_before_content,
          keep_formatted_horizontal_header_footer_page,
          keep_empty_explicit_sheet_pages,
        ) {
          continue;
        }
        pages.push(CalcPrintPage {
          sheet,
          sheet_page_index,
          starts_print_area_row,
          has_explicit_print_area: !named_ranges.resolved_print_areas.is_empty(),
          page_number: pages.len() + 1,
          total_pages: 0,
          zoom: scale.zoom,
          paper_scale_percent: scale.paper_scale_percent,
          pagination_paper_scale_percent: scale.pagination_paper_scale_percent,
          page_settings: &sheet.page_settings,
          repeated_rows: named_ranges.repeat_rows,
          repeated_columns: named_ranges.repeat_columns,
          area,
          cells,
          repeated_row_cells,
          repeated_column_cells,
          repeated_corner_cells,
          drawing_anchor_count: drawing_summary.anchors,
          chart_count: drawing_summary.charts,
        });
        sheet_page_index += 1;
      }
    }
    let total_pages = pages.len();
    for page in &mut pages {
      page.total_pages = total_pages;
    }
    Self { pages }
  }
}

fn should_skip_empty_print_page(
  skip_empty: bool,
  empty: bool,
  implicit_page_before_content: bool,
  keep_implicit_header_footer_page: bool,
  keep_empty_explicit_sheet_pages: bool,
) -> bool {
  skip_empty
    && empty
    && !implicit_page_before_content
    && !keep_implicit_header_footer_page
    && !keep_empty_explicit_sheet_pages
}

fn keep_formatted_horizontal_header_footer_page(
  uses_formatted_extent: bool,
  ordinary_used_range: Option<CellRange>,
  page_area: CellRange,
  has_nondefault_direct_cell_formatting: bool,
) -> bool {
  uses_formatted_extent
    && has_nondefault_direct_cell_formatting
    && ordinary_used_range.is_some_and(|used| page_area.start.col > used.end.col)
}

fn page_cell_scan_area(area: CellRange) -> CellRange {
  // A string can start several columns before this horizontal page and still
  // overflow into it. The Strict CGM-picture workbook's I13 text continues on
  // its second page. Retain preceding cells as overflow/occupancy context;
  // display culls their text against the page and paints no cell bodies.
  // Calc's FillInfo builds ScCellInfo through nCol2 + 1 so the logical page
  // can resolve occupied neighbours and text overflowing back from the first
  // following column. ScOutputData still owns paint only through mnX2; display
  // keeps the extra column as scan context rather than ordinary page content.
  CellRange::new(
    CellAddress {
      col: 1,
      row: area.start.row,
    },
    CellAddress {
      col: area.end.col.saturating_add(1).min(XLSX_MAX_COLUMN),
      row: area.end.row,
    },
  )
}

#[derive(Clone, Copy, Debug)]
struct CalcPrintScaleState {
  zoom: u32,
  paper_scale_percent: u32,
  pagination_paper_scale_percent: u32,
  skip_empty: bool,
  top_down: bool,
}

fn print_scale_state(
  import: &ExcelImport,
  sheet: &CalcSheet,
  areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges,
  text_metrics: &mut TextMetrics,
) -> CalcPrintScaleState {
  if sheet.sheet_type == SheetType::Chartsheet {
    // A chartsheet's absolute anchors are already physical point sizes.
    // Office keeps both dimensions when paper, margins or orientation change.
    return CalcPrintScaleState {
      zoom: 100,
      paper_scale_percent: 100,
      pagination_paper_scale_percent: 100,
      skip_empty: true,
      top_down: true,
    };
  }
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
  // ECMA-376 keeps the fit-to-pages mode bit in sheetPr/pageSetUpPr,
  // separately from pageSetup's width/height operands. LibreOffice mirrors
  // that split in PageSettingsModel::mbFitToPages and uses PageScale whenever
  // the mode bit is false. Do not infer the mode from stale or inactive
  // fitToWidth/fitToHeight values: Excel's 49156.xlsx is the counterexample,
  // with scale="47", fitToHeight="2", and no pageSetUpPr.
  let fit_to_page =
    sheet.page_settings.fit_to_page || sheet.metrics.settings.properties.page_setup.fit_to_page;
  let (fit_to_width, fit_to_height) = if fit_to_page
    && sheet.page_settings.fit_to_width == 0
    && sheet.page_settings.fit_to_height == 0
  {
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
  let mut zoom = sheet.page_settings.scale;
  let mut auto_page_columns = forced_break_min_columns.max(1);
  let mut auto_page_rows = forced_break_min_rows.max(1);

  if fit_to_page && (fit_to_width > 0 || fit_to_height > 0) {
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
      import,
      sheet,
      areas,
      named_ranges,
      auto_page_columns,
      auto_page_rows,
    );
    // ECMA-376 pageSetup says fitToWidth/fitToHeight override scale. Keep the
    // metric-derived fit zoom even when it happens to equal the serialized
    // scale; equality does not switch the worksheet back to 100 percent.
    if fit_to_width > 0
      && fit_to_height == 0
      && actual_row_page_count(import, sheet, areas, named_ranges, zoom, text_metrics) > 1
    {
      let adjusted_zoom = ((zoom as f32) * LIBREOFFICE_FIT_TO_WIDTH_ZOOM_FACTOR)
        .floor()
        .max(ZOOM_MIN as f32) as u32;
      if adjusted_zoom < zoom
        && actual_row_page_count(
          import,
          sheet,
          areas,
          named_ranges,
          adjusted_zoom,
          text_metrics,
        ) < actual_row_page_count(import, sheet, areas, named_ranges, zoom, text_metrics)
      {
        zoom = adjusted_zoom;
      }
    }
  } else if sheet.page_settings.scale > 0 {
    zoom = sheet.page_settings.scale.max(ZOOM_MIN);
  }
  // Paper conversion is an output-device transform, not the serialized
  // worksheet zoom. Keep it sheet-level so continuation pages preserve a
  // chart/printer canvas even when that individual page has no chart anchor.
  // The page splitter consumes the composed float because the transformed
  // content footprint determines page boundaries, but `zoom` remains the
  // authored/fit-derived worksheet value.
  let has_chart = sheet
    .resources
    .drawings
    .iter()
    .any(|drawing| !drawing.charts.is_empty() || !drawing.extended_charts.is_empty());
  let paper_scale_percent = sheet
    .page_settings
    .fixed_output_paper_scale_percent(has_chart);
  let pagination_paper_scale_percent = sheet
    .page_settings
    .fixed_output_pagination_paper_scale_percent(has_chart);

  CalcPrintScaleState {
    zoom,
    paper_scale_percent,
    pagination_paper_scale_percent,
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
  named_ranges: &CalcPrintNamedRanges,
  zoom: u32,
  text_metrics: &mut TextMetrics,
) -> usize {
  areas
    .iter()
    .map(|area| {
      split_range_by_page_metrics(
        import,
        sheet,
        *area,
        PageMetricSplit {
          breaks: &sheet.metrics.row_breaks,
          by_row: true,
          repeat: named_ranges.repeat_rows,
          zoom_percent: zoom as f32,
          text_metrics,
        },
      )
      .len()
    })
    .sum::<usize>()
    .max(1)
}

fn fit_zoom_to_pages(
  import: &ExcelImport,
  sheet: &CalcSheet,
  areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges,
  page_columns: usize,
  page_rows: usize,
) -> u32 {
  let Some(area) = areas.first().copied() else {
    return 100;
  };
  let content = print_content_size_pt(&sheet.page_settings);
  let repeat_width = named_ranges
    .repeat_columns
    .map(|range| sheet.range_rect(range).width_pt)
    .unwrap_or(0.0);
  let repeat_height = named_ranges
    .repeat_rows
    .map(|range| sheet.range_rect(range).height_pt)
    .unwrap_or(0.0);
  let page_width = (content.0 - repeat_width).max(1.0);
  let page_height =
    (print_row_capacity_pt(&sheet.page_settings, sheet.page_settings.page_size_pt().1)
      - repeat_height)
      .max(1.0);
  let fit_area = fit_scale_area(import, sheet, area, named_ranges);
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
  import: &ExcelImport,
  sheet: &CalcSheet,
  area: CellRange,
  named_ranges: &CalcPrintNamedRanges,
) -> CellRange {
  if !named_ranges.resolved_print_areas.is_empty() {
    return area;
  }
  implicit_used_range(import, sheet, named_ranges).map_or(area, |used| {
    let range = CellRange::new(CellAddress { col: 1, row: 1 }, used.end);
    extend_print_area_for_merges(sheet, range)
  })
}

fn print_content_size_pt(page_settings: &CalcPageSettings) -> (f32, f32) {
  print_content_size_for_page(page_settings, page_settings.page_size_pt())
}

fn print_content_size_for_page(
  page_settings: &CalcPageSettings,
  (mut width, mut height): (f32, f32),
) -> (f32, f32) {
  width -= (page_settings.margin_left_in + page_settings.margin_right_in) as f32
    * crate::units::POINTS_PER_INCH;
  height -= (page_settings.margin_top_in + page_settings.margin_bottom_in) as f32
    * crate::units::POINTS_PER_INCH;
  // ECMA-376 Part 1 §18.3.1.62 defines top/bottom as page margins and
  // header/footer as positions within those margins. LibreOffice's OOXML
  // PageSettingsConverter makes the same ownership explicit: with a header,
  // Calc TopMargin is the OOXML header margin and HeaderHeight is
  // top - header, so ScPrintFunc::GetDocPageSize subtracts exactly `top`.
  // Footer geometry is symmetric. Scaling changes worksheet content, not
  // these physical margins; subtracting header/footer again shrinks the body.
  (width.max(1.0), height.max(1.0))
}

fn print_row_capacity_pt(page_settings: &CalcPageSettings, page_height_pt: f32) -> f32 {
  // Excel's fixed-output row page breaks reserve eight 600dpi dots inside
  // the physical margins. Original/mixed-font and uniform-row Office controls
  // at 50/100/200% keep this physical allowance unchanged; print/screen PDF
  // quality also has no effect. Each margin is truncated on the device grid
  // before subtraction: 0.740 and 0.739 inches distinguish that boundary.
  let dpi = f64::from(crate::units::OFFICE_FIXED_OUTPUT_DPI);
  let points_per_inch = f64::from(crate::units::POINTS_PER_INCH);
  let page_dots = (f64::from(page_height_pt) * dpi / points_per_inch).round();
  let top_dots = (page_settings.margin_top_in * dpi).floor();
  let bottom_dots = (page_settings.margin_bottom_in * dpi).floor();
  ((page_dots - top_dots - bottom_dots - 8.0) * points_per_inch / dpi).max(1.0) as f32
}

fn drawing_summary_for_area(sheet: &CalcSheet, area: Option<CellRange>) -> CalcPrintDrawingSummary {
  let mut summary = CalcPrintDrawingSummary::default();
  for drawing in &sheet.resources.drawings {
    for anchor in &drawing.anchors {
      if !anchor_intersects_area(sheet, anchor, area) {
        continue;
      }
      summary.anchors += 1;
      if anchor.object.kind == super::drawing::DrawingObjectKind::GraphicFrame {
        summary.charts += 1;
      }
    }
  }
  // client shapes into the sheet draw layer; sc/source/core/data/documen9.cxx
  // then treats that draw layer uniformly for print area and page visibility.
  for shape in sheet
    .resources
    .object_resources
    .vml_drawings
    .iter()
    .flat_map(|drawing| drawing.shapes.iter())
  {
    if !sheet.vml_note_prints_in_place(shape) || !vml_shape_intersects_area(sheet, shape, area) {
      continue;
    }
    summary.anchors += 1;
  }
  summary
}

fn anchor_belongs_to_area(
  marker: Option<&super::drawing::DrawingMarkerModel>,
  area: Option<CellRange>,
) -> bool {
  match (marker, area) {
    (_, None) => true,
    (None, Some(_)) => false,
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

impl CalcPrintNamedRanges {
  fn from_import(import: &ExcelImport, sheet: &CalcSheet) -> Self {
    // DefinedName::convertFormula extracts print areas, repeated titles, and
    // filter database ranges. Keep built-in print defined names attached to
    // the ScPrintFunc owner; scalar formula evaluation lives in
    // ooxmlsdk-formula.
    let print_areas = import
      .defined_names
      .records_for_sheet(sheet.workbook_index, DefinedNameBuiltin::PrintArea);
    let print_titles = import
      .defined_names
      .records_for_sheet(sheet.workbook_index, DefinedNameBuiltin::PrintTitles);
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
      resolved_print_areas,
      repeat_rows,
      repeat_columns,
    }
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
  named_ranges: &CalcPrintNamedRanges,
  text_metrics: &mut TextMetrics,
) -> Vec<CellRange> {
  if !named_ranges.resolved_print_areas.is_empty() {
    // _xlnm.Print_Area is an explicit output boundary. A merged cell can
    // intersect that boundary without admitting unrelated cells beyond it:
    // Office prints A:F in tdf100034 even though its title merges A1:G1.
    // Merge expansion belongs only to the implicit used-range path below.
    return named_ranges.resolved_print_areas.clone();
  }
  match implicit_used_range(import, sheet, named_ranges) {
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
      vec![extend_print_area_for_overflow(
        import,
        sheet,
        range,
        text_metrics,
      )]
    }
    // With skip-empty disabled, a missing document print area still leaves the
    // default start/end range printable, so header/footer-only sheets export a page.
    None => {
      vec![drawing_print_area(sheet).unwrap_or(CellRange::single(CellAddress { col: 1, row: 1 }))]
    }
  }
}

fn implicit_used_range(
  import: &ExcelImport,
  sheet: &CalcSheet,
  named_ranges: &CalcPrintNamedRanges,
) -> Option<CellRange> {
  if implicit_header_footer_uses_formatted_cell_extent(import, sheet, named_ranges) {
    sheet.used_range_including_direct_cell_formatting(&import.styles)
  } else {
    sheet.used_range(&import.styles)
  }
}

fn implicit_header_footer_uses_formatted_cell_extent(
  import: &ExcelImport,
  sheet: &CalcSheet,
  named_ranges: &CalcPrintNamedRanges,
) -> bool {
  // Fit-to-pages uses the visible body extent for both zoom and pagination.
  // ExpenseReport's font/number-format-only M cells must neither shrink its
  // A:L body nor add a header/footer-only page after that body has been fitted.
  // Fixed-scale printing keeps the wider formatted extent below.
  if sheet.page_settings.fit_to_page || sheet.metrics.settings.properties.page_setup.fit_to_page {
    return false;
  }
  use_formatted_cell_extent_for_implicit_header_footer(
    !named_ranges.resolved_print_areas.is_empty(),
    sheet.page_settings.header_footer.has_print_content(),
    sheet_body_is_empty(import, sheet),
  )
}

fn use_formatted_cell_extent_for_implicit_header_footer(
  has_explicit_print_area: bool,
  has_header_footer: bool,
  body_empty: bool,
) -> bool {
  // ECMA-376 Part 1 section 18.3.1.35 and Microsoft's binary Dimensions
  // records include directly formatted cells in the worksheet used range.
  // Office paints a configured header/footer on every resulting page, even
  // when that page's cell body has no ink. Keep this extent separate from the
  // ordinary visible-body range: tdf131536 has thousands of style-only cells
  // but no header/footer and must not acquire those trailing pages.
  !has_explicit_print_area && has_header_footer && !body_empty
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
  // merges ScDrawLayer::GetPrintArea into the sheet print area, and
  // sc/source/core/data/drwlayer.cxx::ScDrawLayer::GetPrintArea maps object
  // bounds back to start/end cells while excluding the hidden layer.
  let xdr_ranges = sheet
    .resources
    .drawings
    .iter()
    .flat_map(|drawing| drawing.anchors.iter())
    .filter(|anchor| !anchor.object.hidden)
    .filter_map(|anchor| drawing_anchor_cell_range(sheet, anchor));
  let vml_ranges = sheet
    .resources
    .object_resources
    .vml_drawings
    .iter()
    .flat_map(|drawing| drawing.shapes.iter())
    // Notes omitted from the printout must not create pages or enlarge them.
    .filter(|shape| sheet.vml_note_prints_in_place(shape))
    // Hidden notes also stay out of the printed area.
    .filter(|shape| !(shape.hidden && shape.object_type.as_deref() == Some("Note")))
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
  // ECMA-376 Part 1 20.5.2.1, 20.5.2.24, and 20.5.2.33 make the
  // SpreadsheetDrawing anchor the sheet-placement owner. Calc likewise
  // replaces a top-level shape's imported position and size with the anchor
  // rectangle before adding it to the draw layer. An inner a:xfrm can contain
  // stale coordinates and must not extend the worksheet print area.
  let rect = match anchor.kind {
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
  }?;
  let (x, y, width, height) = excel_unrotated_drawing_bounds(rect, anchor.object.rotation_deg);
  Some(rotated_drawing_bounds(
    x,
    y,
    width,
    height,
    anchor.object.rotation_deg,
  ))
}

fn excel_unrotated_drawing_bounds(
  (x, y, width, height): (f32, f32, f32, f32),
  rotation_deg: f32,
) -> (f32, f32, f32, f32) {
  // sc/source/filter/oox/drawingfragment.cxx (tdf#83593): Excel rewrites
  // drawing anchors using a quarter-turn in these angular ranges. Restore the
  // unrotated center-preserving rectangle before applying the authored angle.
  let rotation_deg = rotation_deg.rem_euclid(360.0);
  if (45.0..135.0).contains(&rotation_deg) || (225.0..315.0).contains(&rotation_deg) {
    let center_x = x + width / 2.0;
    let center_y = y + height / 2.0;
    (
      center_x - height / 2.0,
      center_y - width / 2.0,
      height,
      width,
    )
  } else {
    (x, y, width, height)
  }
}

fn rotated_drawing_bounds(
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  rotation_deg: f32,
) -> (f32, f32, f32, f32) {
  // ScDrawLayer::GetPrintArea observes the transformed drawing-layer object,
  // not the unrotated a:xfrm extent. This matters for imported two-cell
  // anchors whose from/to markers already enclose a quarter-turned object:
  // using only the cached a:off/a:ext can suppress an otherwise printable
  // continuation page. Rotate around DrawingML's shape center and return the
  // axis-aligned bounds consumed by Calc's print-area cell lookup.
  let angle = rotation_deg.to_radians();
  let sin = angle.sin().abs();
  let cos = angle.cos().abs();
  let rotated_width = width * cos + height * sin;
  let rotated_height = width * sin + height * cos;
  let center_x = x + width / 2.0;
  let center_y = y + height / 2.0;
  (
    center_x - rotated_width / 2.0,
    center_y - rotated_height / 2.0,
    rotated_width,
    rotated_height,
  )
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
    .unwrap_or(false)
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
  sheet.object_anchor_rect_pt(shape).or_else(|| {
    shape
      .style
      .as_deref()
      .and_then(vml_style_rect_pt)
      .or_else(|| {
        shape
          .anchor
          .and_then(|anchor| vml_anchor_rect_pt(sheet, shape, anchor))
      })
  })
}

fn vml_anchor_rect_pt(
  sheet: &CalcSheet,
  shape: &super::object_resources::VmlShapeModel,
  anchor: super::object_resources::VmlClientAnchor,
) -> Option<(f32, f32, f32, f32)> {
  let x1 = vml_anchor_x_pt(sheet, shape, anchor.from_col, anchor.from_col_offset_px);
  let y1 = vml_anchor_y_pt(sheet, shape, anchor.from_row, anchor.from_row_offset_px);
  let x2 = vml_anchor_x_pt(sheet, shape, anchor.to_col, anchor.to_col_offset_px);
  let y2 = vml_anchor_y_pt(sheet, shape, anchor.to_row, anchor.to_row_offset_px);
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

fn vml_anchor_x_pt(
  sheet: &CalcSheet,
  shape: &super::object_resources::VmlShapeModel,
  zero_based_col: u32,
  offset_px: i32,
) -> f32 {
  let col = zero_based_col.saturating_add(1);
  let cell = sheet.cell_rect(CellAddress { col, row: 1 });
  let next_cell = sheet.cell_rect(CellAddress {
    col: col.saturating_add(1),
    row: 1,
  });
  // ShapeAnchor::importVmlAnchor marks offsets as CellAnchorType::Pixel, and
  // calcCellAnchorEmu clamps them to the next cell minus one twip.
  (cell.x_pt + sheet.vml_anchor_offset_pt(shape, offset_px))
    .min(next_cell.x_pt - units::twips_to_points(1.0))
}

fn vml_anchor_y_pt(
  sheet: &CalcSheet,
  shape: &super::object_resources::VmlShapeModel,
  zero_based_row: u32,
  offset_px: i32,
) -> f32 {
  let row = zero_based_row.saturating_add(1);
  let cell = sheet.cell_rect(CellAddress { col: 1, row });
  let next_cell = sheet.cell_rect(CellAddress {
    col: 1,
    row: row.saturating_add(1),
  });
  (cell.y_pt + sheet.vml_anchor_offset_pt(shape, offset_px))
    .min(next_cell.y_pt - units::twips_to_points(1.0))
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
  value * units::POINTS_PER_INCH / units::CSS_PIXELS_PER_INCH
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
  text_metrics: &mut TextMetrics,
) -> CellRange {
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
      let style_index = cell.style_index;
      if import
        .styles
        .alignment_for_cell(style_index)
        .is_some_and(|alignment| alignment.wrap_text)
      {
        continue;
      }
      let style = import.styles.text_style_for_cell(style_index);
      let column = text_overflow_end_column(sheet, row, cell, address, &style, text_metrics);
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
  text_metrics: &mut TextMetrics,
) -> u32 {
  let needed_width_pt = calc_cached_print_text_width_pt(
    text_metrics.measure_text(&cell.display_text, style) + CALC_CELL_TEXT_MARGIN_PT,
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
  // GetNeededSize(..., bTotalSize=true) into ScColumnTextWidthIterator as a
  // sal_uInt16. sc/source/core/data/table1.cxx::MaybeAddExtraColumn then reads
  // that cached GetTextWidth() pixel value and converts it back through nPPTX.
  // Calc print layout calls GetPrinter(); the Unix generic printer resolves
  // its DPI through PPDContext::getRenderResolution(), and LibreOffice's
  // bundled SGENPRT.PS has *DefaultResolution: 600dpi.
  let pixels = (width_pt * units::OFFICE_FIXED_OUTPUT_DPI / units::POINTS_PER_INCH).round() as i64;
  let cached_pixels = pixels as u16;
  f32::from(cached_pixels) * units::POINTS_PER_INCH / units::OFFICE_FIXED_OUTPUT_DPI
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

fn print_area_is_empty(
  import: &ExcelImport,
  sheet: &CalcSheet,
  area: CellRange,
  text_metrics: &mut TextMetrics,
) -> bool {
  // Excel retains pages containing visible cell paint even when their cells
  // have no values. This matters for wide formatted ranges whose trailing
  // columns fall on a separate horizontal page. Drawing content is checked by
  // the caller because it uses drawing anchors rather than cell records.
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
      let style_index = cell.style_index;
      let borders = import.styles.borders_for_cell(style_index);
      if borders.left.is_some()
        || borders.right.is_some()
        || borders.top.is_some()
        || borders.bottom.is_some()
        || import.styles.fill_for_cell(style_index).color.is_some()
      {
        return false;
      }
    }
  }
  if sheet
    .metrics
    .merged_ranges
    .iter()
    .filter_map(|reference| CellRange::parse_a1_range(reference))
    .any(|merged| merged.intersects(area))
  {
    // An authored merged range owns its complete rectangular print extent,
    // even when its anchor has no text or visible fill. This is narrower than
    // treating every styled blank cell as printable: repeated font/date-style
    // tails remain skippable, while a merge that crosses an automatic page
    // break retains that page in Excel fixed output.
    return false;
  }
  if sheet_area_has_left_text_overflow(import, sheet, area, text_metrics) {
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
  // calls ScAttrArray::GetLastVisibleAttr after data detection. Explicit row
  // formatting near the data can extend the print area even when the rows
  // contain no cells. The scan stops at the first SC_VISATTR_STOP-sized run,
  // including the implicit default-format gap between authored row records;
  // attributes beyond that run are stale tail metadata, not print content.
  let mut last_row = None;
  let mut run_len = 0u32;
  let mut previous_row = None;
  let mut last_row_before_run = None;
  for row in sheet
    .rows
    .iter()
    .filter(|row| row_has_visible_attribute(import, sheet, row))
  {
    let row_index = row.row_index.unwrap_or(1);
    if row_index <= data_end_row {
      continue;
    }
    let previous = previous_row.unwrap_or(data_end_row);
    let default_gap = row_index.saturating_sub(previous.saturating_add(1));
    if default_gap >= SC_VISATTR_STOP {
      break;
    }
    if previous_row.is_some_and(|previous| previous + 1 == row_index) {
      run_len = run_len.saturating_add(1);
    } else {
      run_len = 1;
      last_row_before_run = last_row;
    }
    previous_row = Some(row_index);
    if run_len >= SC_VISATTR_STOP {
      last_row = last_row_before_run;
      break;
    }
    last_row = Some(row_index);
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
  text_metrics: &mut TextMetrics,
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
      // ScDocument::IsPrintEmpty calls ExtendPrintArea() for the columns left
      // of the candidate page. If a left-side string extends into this page,
      // the page is not empty even when it has no cell bodies of its own.
      let style = import.styles.text_style_for_cell(cell.style_index);
      text_overflow_end_column(sheet, row, cell, address, &style, text_metrics) >= area.start.col
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
    .any(|shape| {
      sheet.vml_note_prints_in_place(shape) && vml_shape_cell_range(sheet, shape).is_some()
    });
  if has_vml_drawing {
    return false;
  }
  sheet.rows.iter().all(|row| {
    row.cells.iter().all(|cell| {
      if cell.address().is_none() {
        return true;
      }
      if !cell.display_text.is_empty()
        || !cell.rich_text_runs.is_empty()
        || cell.formula.is_some()
        || cell.cached_value.is_some()
        || cell.data_type.is_some()
      {
        return false;
      }
      let style_index = cell.style_index;
      let borders = import.styles.borders_for_cell(style_index);
      borders.left.is_none()
        && borders.right.is_none()
        && borders.top.is_none()
        && borders.bottom.is_none()
        && import.styles.fill_for_cell(style_index).color.is_none()
    })
  })
}

fn page_areas_for_sheet(
  import: &ExcelImport,
  sheet: &CalcSheet,
  print_areas: &[CellRange],
  named_ranges: &CalcPrintNamedRanges,
  zoom_percent: f32,
  top_down: bool,
  text_metrics: &mut TextMetrics,
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
      PageMetricSplit {
        breaks: &sheet.metrics.row_breaks,
        by_row: true,
        repeat: named_ranges.repeat_rows,
        zoom_percent,
        text_metrics,
      },
    );
    let column_slices = split_range_by_page_metrics(
      import,
      sheet,
      *area,
      PageMetricSplit {
        breaks: &sheet.metrics.column_breaks,
        by_row: false,
        repeat: named_ranges.repeat_columns,
        zoom_percent,
        text_metrics,
      },
    );
    if top_down {
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

struct PageMetricSplit<'a> {
  breaks: &'a [super::worksheet::PageBreakModel],
  by_row: bool,
  repeat: Option<CellRange>,
  zoom_percent: f32,
  text_metrics: &'a mut TextMetrics,
}

fn split_range_by_page_metrics(
  import: &ExcelImport,
  sheet: &CalcSheet,
  area: CellRange,
  split: PageMetricSplit<'_>,
) -> Vec<CellRange> {
  let start = if split.by_row {
    area.start.row
  } else {
    area.start.col
  };
  let end = if split.by_row {
    area.end.row
  } else {
    area.end.col
  };
  let mut slices = Vec::new();
  let has_chart = sheet
    .resources
    .drawings
    .iter()
    .any(|drawing| !drawing.charts.is_empty() || !drawing.extended_charts.is_empty());
  let page_size = sheet
    .page_settings
    .fixed_output_pagination_page_size_pt(has_chart);
  let content_size = print_content_size_for_page(&sheet.page_settings, page_size);
  let repeat_size = split
    .repeat
    .map(|range| {
      if split.by_row {
        sheet.range_rect(range).height_pt
      } else {
        sheet.range_rect(range).width_pt
      }
    })
    .unwrap_or(0.0);
  let page_capacity = (if split.by_row {
    print_row_capacity_pt(&sheet.page_settings, page_size.1)
  } else {
    content_size.0
  })
  .max(1.0)
    * 100.0
    / split.zoom_percent.max(ZOOM_MIN as f32);
  let mut current_start = start;
  let mut current = start;
  let mut used = 0.0f32;
  while current <= end {
    if split
      .breaks
      .iter()
      .any(|page_break| manual_page_break_starts_at(page_break, current, current_start))
    {
      slices.push(axis_slice(area, split.by_row, current_start, current - 1));
      current_start = current;
      used = 0.0;
    }
    // Titles already inside this slice occupy their ordinary row/column
    // space. Only continuation slices prepend them. Both the title extent
    // and row/column sizes are unscaled here; subtracting the title before
    // undoing page zoom reserves too much space below 100%.
    let repeat_is_prepended = split.repeat.is_some_and(|range| {
      current_start
        > if split.by_row {
          range.end.row
        } else {
          range.end.col
        }
    });
    let available = (page_capacity
      - if repeat_is_prepended {
        repeat_size
      } else {
        0.0
      })
    .max(1.0);
    let size = if split.by_row {
      print_row_height_pt(import, sheet, current, &mut *split.text_metrics)
    } else {
      sheet.column_width_pt(current)
    };
    let exceeds_page = if split.by_row {
      // Compare realized row bottoms, not sub-dot accumulation noise.
      // Two 50% rows fit at the same device boundary as one 100% row;
      // floating-point summation must not move that break.
      let dots_per_point = split.zoom_percent.max(ZOOM_MIN as f32) / 100.0
        * crate::units::OFFICE_FIXED_OUTPUT_DPI
        / crate::units::POINTS_PER_INCH;
      ((used + size) * dots_per_point).round() > (available * dots_per_point).round()
    } else {
      used + size > available
    };
    if used > 0.0 && exceeds_page {
      // CalcPages first derives every page boundary from the sheet metrics;
      // lcl_SetHidden/IsPrintEmpty then decides whether each complete slice
      // is printable.  Content in the first column after a break therefore
      // does not turn that column into a one-column page: it remains the first
      // column of the next metric-derived slice.
      slices.push(axis_slice(area, split.by_row, current_start, current - 1));
      current_start = current;
      used = 0.0;
    }
    used += size;
    current += 1;
  }
  if current_start <= end {
    slices.push(axis_slice(area, split.by_row, current_start, end));
  }
  slices
}

fn manual_page_break_starts_at(
  page_break: &super::worksheet::PageBreakModel,
  current: u32,
  current_start: u32,
) -> bool {
  // ECMA-376 Part 1 §18.3.1.3 stores brk@id as a zero-based row or
  // column index and places the break above/left of that position. CalcSheet
  // addresses are one-based, so id=51 starts the next page at row 52. POI's
  // XSSFSheet::setBreak independently serializes its zero-based API index as
  // id + 1, which is the same boundary viewed from the caller side.
  page_break.manual && page_break.id.checked_add(1) == Some(current) && current > current_start
}

fn print_row_height_pt(
  import: &ExcelImport,
  sheet: &CalcSheet,
  row_index: u32,
  text_metrics: &mut TextMetrics,
) -> f32 {
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
    let style_index = cell.style_index;
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
      text_metrics,
    );
    if line_count > 1 {
      // ScColumn::GetOptimalHeight uses GetNeededSize() for line-break cells;
      // one text line follows lcl_GetAttribHeight() at 1.18 * font height.
      height = height.max(style.font_size_pt * 1.18 * line_count as f32);
    }
  }
  height
}

fn wrapped_print_line_count(
  text: &str,
  column_width_pt: f32,
  style: &TextStyle,
  text_metrics: &mut TextMetrics,
) -> usize {
  let available = (column_width_pt - CALC_CELL_TEXT_MARGIN_PT).max(1.0);
  let mut lines = 0usize;
  for paragraph in text.split(['\n', '\r']) {
    if paragraph.is_empty() {
      lines += 1;
      continue;
    }
    let mut current_width = 0.0f32;
    for word in paragraph.split_whitespace() {
      let word_width = text_metrics.measure_text(word, style);
      let separator_width = if current_width > 0.0 {
        text_metrics.measure_text(" ", style)
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

fn print_cell_intersects_area(
  sheet: &CalcSheet,
  address: CellAddress,
  print_address: CellAddress,
  area: CellRange,
) -> bool {
  if area.contains(print_address) {
    return true;
  }
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
  if area.start.row <= repeat_rows.end.row {
    return None;
  }
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
  if area.start.col <= repeat_columns.end.col {
    return None;
  }
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
  area: Option<CellRange>,
  repeat_rows: Option<CellRange>,
  repeat_columns: Option<CellRange>,
) -> Option<CellRange> {
  let area = area?;
  let repeat_rows = repeat_rows?;
  let repeat_columns = repeat_columns?;
  if area.start.row <= repeat_rows.end.row || area.start.col <= repeat_columns.end.col {
    return None;
  }
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
  conditional_eval_cache: &mut ConditionalFormatEvalCache,
) -> Vec<CalcPrintCell<'a>> {
  let strings = OfficeStringCatalog::for_ui_language(Some(import.styles.output_ui_language()));
  let mut physical_cells = Vec::new();
  let mut occupied = HashSet::new();
  let mut visit_cell = |row_index: u32, row: &'a super::worksheet::CalcRow| {
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
      if row.hidden || hidden_column {
        continue;
      }
      let style_index = cell.style_index;
      let number_format_id = style_index
        .and_then(|index| import.styles.cell_xfs.get(index as usize))
        .and_then(|format| format.number_format_id);
      let number_format_code = number_format_id.and_then(|id| import.styles.number_format_code(id));
      let conditional_number_format_code = conditional_number_format_code(
        import,
        sheet,
        address,
        cell.display_text.as_str(),
        conditional_eval_cache,
      );
      let pivot_format_id = super::pivot::pivot_format_id_for_address(sheet, print_address);
      let pivot_format_number_format_code = pivot_format_id
        .and_then(|format_id| import.styles.differential_number_format_code(format_id));
      let pivot_header_number_format_code =
        pivot_header_number_format_code(import, sheet, print_address);
      let pivot_number_format_code = pivot_data_number_format_code(import, sheet, print_address);
      let raw_text =
        pivot_data_cell_text_override(sheet, print_address).unwrap_or(cell.display_text.as_str());
      let effective_number_format_code = conditional_number_format_code
        .or(pivot_format_number_format_code)
        .or(pivot_header_number_format_code)
        .or(pivot_number_format_code)
        .or(number_format_code);
      let (rendered_text, number_format_state) = rendered_number_text_for_locale(
        raw_text,
        effective_number_format_code,
        cell.data_type,
        import.globals.settings.date_1904,
        import.styles.output_format_locale(),
      );
      let rendered_text = if number_format_state == NumberFormatRenderState::Boolean {
        // Logical values are Office UI resources. Their cell type remains
        // boolean; text spelling TRUE/FALSE and numeric literal formats do
        // not enter this path and retain their authored contents/alignment.
        crate::localization::office_boolean_text(
          Some(import.styles.output_ui_language()),
          boolean_raw_value(raw_text),
        )
        .to_owned()
      } else if number_format_state == NumberFormatRenderState::Error {
        // Keep the formula model and conditional evaluation in canonical
        // error codes; only typed error cells use localized display text.
        crate::localization::office_spreadsheet_error_text(
          Some(import.styles.output_ui_language()),
          &rendered_text,
        )
        .to_owned()
      } else {
        rendered_text
      };
      let number_format_color = effective_number_format_code
        .and_then(|code| numeric_format_color(code, raw_text, cell.data_type, &import.styles));
      let number_format_layout = effective_number_format_code
        .and_then(|code| number_format_layout(raw_text, code, number_format_state));
      let rendered_text = pivot_display_text(
        sheet,
        print_address,
        rendered_text,
        import.styles.has_explicit_ui_language(),
        strings,
      );
      let icon_set = conditional_statistical_cell_value(&cell.display_text, cell.data_type)
        .and_then(|value| {
          conditional_icon_set(import, sheet, address, value, conditional_eval_cache)
        });
      let color_scale_fill = conditional_statistical_cell_value(&cell.display_text, cell.data_type)
        .and_then(|value| {
          conditional_color_scale_fill(import, sheet, address, value, conditional_eval_cache)
        });
      let data_bar = conditional_statistical_cell_value(&cell.display_text, cell.data_type)
        .and_then(|value| {
          conditional_data_bar(import, sheet, address, value, conditional_eval_cache)
        });
      physical_cells.push(CalcPrintCell {
        address: print_address,
        text: Cow::Borrowed(cell.display_text.as_str()),
        data_type: cell.data_type,
        style_index,
        pivot_format_id,
        rendered_text,
        rich_text_runs: &cell.rich_text_runs,
        number_format_state,
        number_format_color,
        number_format_layout,
        formula: cell.formula.is_some(),
        icon_set,
        data_bar,
        color_scale_fill,
      });
    }
  };
  if sheet.resources.pivot_tables.tables.is_empty() {
    for (row_index, row) in sheet.rows_intersecting_print_area(area) {
      visit_cell(row_index, row);
    }
  } else {
    // Pivot cache rows may move upward when firstHeaderRow is greater than
    // one, so their source row can sit outside the output page area.
    for (row_position, row) in sheet.rows.iter().enumerate() {
      visit_cell(row.row_index.unwrap_or(row_position as u32 + 1), row);
    }
  }
  let mut virtual_cells = pivot_virtual_print_cells(sheet, area, &occupied, strings);
  occupied.extend(virtual_cells.iter().map(|cell| cell.address));
  virtual_cells.extend(table_virtual_print_cells(sheet, area, &occupied));
  occupied.extend(virtual_cells.iter().map(|cell| cell.address));
  virtual_cells.extend(inherited_format_print_cells(import, sheet, area, &occupied));
  virtual_cells.sort_unstable_by_key(|cell| (cell.address.row, cell.address.col));
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
  strings: &'static OfficeStringCatalog,
) -> Vec<CalcPrintCell<'a>> {
  let mut cells = Vec::new();
  for pivot in &sheet.resources.pivot_tables.tables {
    if !pivot.refresh_on_load {
      continue;
    }
    let geometry = pivot.output_geometry;
    if !geometry.table_range.intersects(area) {
      continue;
    }
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
          .map(String::as_str)
          .unwrap_or(strings.pivot_empty());
        cells.push(CalcPrintCell {
          address,
          text: Cow::Borrowed(text),
          data_type: Some(x::CellValues::String),
          style_index: None,
          pivot_format_id: None,
          rendered_text: text.to_string(),
          rich_text_runs: &[],
          number_format_state: NumberFormatRenderState::Raw,
          number_format_color: None,
          number_format_layout: None,
          formula: false,
          icon_set: None,
          data_bar: None,
          color_scale_fill: None,
        });
      }
    }
  }
  cells
}

fn inherited_format_print_cells<'a>(
  import: &ExcelImport,
  sheet: &CalcSheet,
  area: CellRange,
  occupied: &HashSet<CellAddress>,
) -> Vec<CalcPrintCell<'a>> {
  // Row/column styles also paint cells that have no <c> record. Materialize
  // only visible blanks on this already paginated area: formatting must not
  // create values, extend the used range, or become a text-overflow owner.
  let columns = (area.start.col..=area.end.col)
    .filter(|&col| sheet.column_width_pt(col) > 0.0)
    .map(|col| (col, sheet.column_style_index(col)))
    .collect::<Vec<_>>();
  let mut visible_styles = HashMap::new();
  let mut has_paint = |style| {
    *visible_styles
      .entry(style)
      .or_insert_with(|| import.styles.cell_has_visible_paint(style))
  };
  let painted_columns = columns
    .iter()
    .copied()
    .filter(|&(_, style)| has_paint(style))
    .collect::<Vec<_>>();
  let mut cells = Vec::new();
  for row in area.start.row..=area.end.row {
    if sheet.row_height_pt(row) <= 0.0 {
      continue;
    }
    let row_style = sheet.row_style_index(row);
    let candidates = if let Some(style) = row_style {
      // An explicit default row style clears a column's fill as well.
      if !has_paint(Some(style)) {
        continue;
      }
      &columns
    } else {
      &painted_columns
    };
    for &(col, column_style) in candidates {
      let address = CellAddress { col, row };
      if occupied.contains(&address) || sheet.is_covered_merged_cell(address) {
        continue;
      }
      cells.push(CalcPrintCell {
        address,
        text: Cow::Borrowed(""),
        data_type: None,
        style_index: row_style.or(column_style),
        pivot_format_id: None,
        rendered_text: String::new(),
        rich_text_runs: &[],
        number_format_state: NumberFormatRenderState::Raw,
        number_format_color: None,
        number_format_layout: None,
        formula: false,
        icon_set: None,
        data_bar: None,
        color_scale_fill: None,
      });
    }
  }
  cells
}

fn table_virtual_print_cells<'a>(
  sheet: &'a CalcSheet,
  area: CellRange,
  occupied: &HashSet<CellAddress>,
) -> Vec<CalcPrintCell<'a>> {
  let mut cells = Vec::new();
  for table in &sheet.resources.tables {
    let Some(range) = table.range else {
      continue;
    };
    let start_row = range.start.row.max(area.start.row);
    let end_row = range.end.row.min(area.end.row);
    let start_col = range.start.col.max(area.start.col);
    let end_col = range.end.col.min(area.end.col);
    if start_row > end_row || start_col > end_col {
      continue;
    }
    let rows = u64::from(end_row - start_row + 1);
    let columns = u64::from(end_col - start_col + 1);
    let additional = usize::try_from(rows.saturating_mul(columns))
      .unwrap_or(usize::MAX)
      .saturating_sub(occupied.len());
    cells.reserve(additional.min(16_384));
    for row in start_row..=end_row {
      for col in start_col..=end_col {
        let address = CellAddress { col, row };
        if occupied.contains(&address) {
          continue;
        }
        cells.push(CalcPrintCell {
          address,
          text: Cow::Borrowed(""),
          data_type: None,
          style_index: None,
          pivot_format_id: None,
          rendered_text: String::new(),
          rich_text_runs: &[],
          number_format_state: NumberFormatRenderState::Raw,
          number_format_color: None,
          number_format_layout: None,
          formula: false,
          icon_set: None,
          data_bar: None,
          color_scale_fill: None,
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
  cache: &mut ConditionalFormatEvalCache,
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
    if !conditional_numeric_rule_matches(import, sheet, references, rule, address, value, cache) {
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

fn conditional_data_bar(
  import: &ExcelImport,
  sheet: &CalcSheet,
  address: CellAddress,
  value: f64,
  cache: &mut ConditionalFormatEvalCache,
) -> Option<CalcPrintDataBar> {
  let conditions = &sheet.metrics.conditions;
  let extensions = &conditions.extension_conditions.conditional_formats;
  let mut rules = conditions
    .conditional_formats
    .iter()
    .filter(|format| conditional_format_contains_cell(format, address))
    .flat_map(|format| {
      format
        .rules
        .iter()
        .filter(|rule| rule.data_bar.is_some() || rule.stop_if_true)
        .map(move |rule| {
          (
            rule.priority,
            &format.sequence_of_references,
            Some(rule),
            None,
          )
        })
    })
    .collect::<Vec<_>>();
  for format in extensions {
    if conditional_references_contain_cell(&format.sequence_of_references, address) {
      rules.extend(format.rules.iter().filter_map(|rule| {
        Some((
          rule.priority?,
          &format.sequence_of_references,
          None,
          Some(rule.data_bar.as_ref()?),
        ))
      }));
    }
  }
  if !rules.iter().any(|(_, _, rule, standalone)| {
    rule.is_some_and(|rule| rule.data_bar.is_some()) || standalone.is_some()
  }) {
    return None;
  }
  rules.sort_by_key(|rule| rule.0);
  for (_, references, rule, standalone) in rules {
    let base = rule.and_then(|rule| rule.data_bar.as_ref());
    let extended = standalone.or_else(|| {
      let id = rule?.extension_id.as_deref()?;
      extensions
        .iter()
        .flat_map(|format| &format.rules)
        .find(|rule| {
          rule.priority.is_none()
            && rule
              .id
              .as_deref()
              .is_some_and(|other| other.eq_ignore_ascii_case(id))
        })?
        .data_bar
        .as_ref()
    });
    if (base.is_some() || extended.is_some())
      && let Some(bar) = evaluate_data_bar(
        import,
        sheet,
        references,
        (base, extended),
        address,
        value,
        cache,
      )
    {
      return Some(bar);
    }
    if let Some(rule) = rule
      && rule.stop_if_true
      && conditional_numeric_rule_matches(import, sheet, references, rule, address, value, cache)
    {
      break;
    }
  }
  None
}

fn evaluate_data_bar(
  import: &ExcelImport,
  sheet: &CalcSheet,
  references: &[String],
  rules: (
    Option<&x::DataBar>,
    Option<&ooxmlsdk::schemas::x14::DataBar>,
  ),
  address: CellAddress,
  value: f64,
  cache: &mut ConditionalFormatEvalCache,
) -> Option<CalcPrintDataBar> {
  use super::sheet_conditions::{IconSetThresholdModel, IconSetThresholdType};
  use ooxmlsdk::schemas::x14::{DataBarAxisPositionValues as Axis, DataBarDirectionValues};

  let (base, extended) = rules;
  let range = conditional_reference_range(references, address)?;
  let stats = cache.stats_for_range(sheet, range);
  let minimum = *stats.sorted_values.first()?;
  let maximum = *stats.sorted_values.last()?;
  let origin = conditional_format_base_address(references, address)?;
  let threshold = |index: usize| {
    let legacy = base.and_then(|bar| bar.conditional_format_value_object.get(index));
    let extension = extended.and_then(|bar| bar.conditional_formatting_value_object.get(index));
    // Excel's linked rules retain their base threshold values. In particular,
    // ClosedXML CFDataBar and CFDataBarNegative have conflicting x14 numeric
    // placeholders, but Office prints the base min/max/percent scale. The
    // automatic endpoint types only exist in x14 and augment that scale.
    let threshold = match (legacy, extension) {
      (_, Some(point))
        if matches!(
          point.r#type,
          ooxmlsdk::schemas::x14::ConditionalFormattingValueObjectTypeValues::AutoMin
            | ooxmlsdk::schemas::x14::ConditionalFormattingValueObjectTypeValues::AutoMax
        ) =>
      {
        IconSetThresholdModel {
          threshold_type: point.r#type.into(),
          value: None,
          greater_than_or_equal: true,
        }
      }
      (Some(point), _) => IconSetThresholdModel {
        threshold_type: point.r#type.into(),
        value: point.val.clone(),
        greater_than_or_equal: true,
      },
      (None, Some(point)) => IconSetThresholdModel {
        threshold_type: point.r#type.into(),
        value: point.formula.clone().or_else(|| point.value.clone()),
        greater_than_or_equal: true,
      },
      _ => return None,
    };
    let number = match threshold.threshold_type {
      IconSetThresholdType::AutomaticMinimum => minimum.min(0.0),
      IconSetThresholdType::AutomaticMaximum => maximum.max(0.0),
      _ => icon_set_threshold_value(
        &threshold,
        IconSetThresholdContext {
          import,
          sheet,
          base: origin,
          address: origin,
          minimum,
          maximum,
          sorted_values: &stats.sorted_values,
        },
      )?,
    };
    number.is_finite().then_some(number)
  };
  let minimum = threshold(0)?;
  let maximum = threshold(1)?;
  let min_length = extended
    .and_then(|bar| bar.min_length)
    .or_else(|| base.and_then(|bar| bar.min_length))
    .unwrap_or(10)
    .min(100) as f64
    / 100.0;
  let max_length = extended
    .and_then(|bar| bar.max_length)
    .or_else(|| base.and_then(|bar| bar.max_length))
    .unwrap_or(90)
    .min(100) as f64
    / 100.0;
  let axis = extended.map_or(Axis::None, |bar| bar.axis_position.unwrap_or_default());
  let (start, end, axis) = data_bar_extent(value, minimum, maximum, min_length, max_length, axis)?;
  // The schema has distinct color element types with the same CT_Color fields.
  macro_rules! color {
    ($color:expr) => {
      $color.and_then(|color| {
        import.styles.spreadsheet_color(&x::Color {
          auto: color.auto,
          indexed: color.indexed,
          rgb: color.rgb.clone(),
          theme: color.theme,
          tint: color.tint,
        })
      })
    };
  }
  let positive = color!(extended.and_then(|bar| bar.fill_color.as_ref()))
    .or_else(|| base.and_then(|bar| import.styles.spreadsheet_color(&bar.color)))?;
  let negative = value < 0.0
    && extended.is_some_and(|bar| {
      !bar
        .negative_bar_color_same_as_positive
        .is_some_and(|value| value.as_bool())
    });
  let color = if negative {
    color!(extended.and_then(|bar| bar.negative_fill_color.as_ref())).unwrap_or(RgbColor {
      r: 255,
      g: 0,
      b: 0,
    })
  } else {
    positive
  };
  let border_color = if extended.is_some_and(|bar| bar.border.is_some_and(|value| value.as_bool()))
  {
    let positive_border = color!(extended.and_then(|bar| bar.border_color.as_ref()));
    if value < 0.0
      && extended.is_some_and(|bar| {
        bar
          .negative_bar_border_color_same_as_positive
          .is_some_and(|value| !value.as_bool())
      })
    {
      color!(extended.and_then(|bar| bar.negative_border_color.as_ref())).or(positive_border)
    } else {
      positive_border
    }
  } else {
    None
  };
  Some(CalcPrintDataBar {
    start: start as f32,
    end: end as f32,
    axis: axis.map(|value| value as f32),
    color,
    border_color,
    axis_color: color!(extended.and_then(|bar| bar.bar_axis_color.as_ref())).unwrap_or(RgbColor {
      r: 0,
      g: 0,
      b: 0,
    }),
    gradient: extended.is_none_or(|bar| bar.gradient.is_none_or(|value| value.as_bool())),
    show_value: extended
      .and_then(|bar| bar.show_value)
      .or_else(|| base.and_then(|bar| bar.show_value))
      .is_none_or(|value| value.as_bool()),
    direction: extended
      .and_then(|bar| bar.direction)
      .unwrap_or(DataBarDirectionValues::Context),
  })
}

fn data_bar_extent(
  value: f64,
  minimum: f64,
  maximum: f64,
  min_length: f64,
  max_length: f64,
  axis: ooxmlsdk::schemas::x14::DataBarAxisPositionValues,
) -> Option<(f64, f64, Option<f64>)> {
  use ooxmlsdk::schemas::x14::DataBarAxisPositionValues as Axis;
  if minimum > maximum || min_length > max_length {
    return None;
  }
  if axis == Axis::None || (axis == Axis::Automatic && minimum >= 0.0) {
    let fraction = if maximum == minimum {
      1.0
    } else {
      ((value - minimum) / (maximum - minimum)).clamp(0.0, 1.0)
    };
    return Some((0.0, min_length + fraction * (max_length - min_length), None));
  }
  let zero = if axis == Axis::Middle {
    0.5
  } else if maximum <= 0.0 {
    1.0
  } else {
    -minimum / (maximum - minimum)
  };
  let clamped = value.clamp(minimum.min(0.0), maximum.max(0.0));
  let end = if axis == Axis::Middle {
    let magnitude = minimum.abs().max(maximum.abs());
    zero
      + if magnitude > 0.0 {
        clamped / magnitude * 0.5 * max_length
      } else {
        0.0
      }
  } else if clamped < 0.0 {
    let fraction = if minimum == maximum {
      // The constant scale has no interval to divide by, just as for
      // nonnegative bars above. Keep its geometry finite.
      1.0
    } else {
      ((clamped - maximum.min(0.0)) / (minimum - maximum.min(0.0))).clamp(0.0, 1.0)
    };
    zero - zero * fraction
  } else if maximum > 0.0 {
    zero + (1.0 - zero) * (clamped / maximum).clamp(0.0, 1.0)
  } else {
    zero
  };
  Some((zero, end, (zero > 0.0 && zero < 1.0).then_some(zero)))
}

fn conditional_icon_set(
  import: &ExcelImport,
  sheet: &CalcSheet,
  address: CellAddress,
  value: f64,
  cache: &mut ConditionalFormatEvalCache,
) -> Option<CalcPrintIconSet> {
  struct Rule<'a> {
    priority: i32,
    references: &'a [String],
    icon_set: Option<&'a super::sheet_conditions::IconSetModel>,
    stop_rule: Option<&'a super::sheet_conditions::ConditionalFormatRuleModel>,
  }

  let mut rules = Vec::new();
  for format in &sheet
    .metrics
    .conditions
    .extension_conditions
    .conditional_formats
  {
    if !conditional_references_contain_cell(&format.sequence_of_references, address) {
      continue;
    }
    rules.extend(format.rules.iter().filter_map(|rule| {
      Some(Rule {
        priority: rule.priority?,
        references: &format.sequence_of_references,
        icon_set: Some(rule.icon_set.as_ref()?),
        stop_rule: None,
      })
    }));
  }
  for format in &sheet.metrics.conditions.conditional_formats {
    if !conditional_format_contains_cell(format, address) {
      continue;
    }
    // A matching higher-priority rule can stop icon formatting even when
    // it has no differential style or icon of its own (ECMA-376 18.3.1.10).
    rules.extend(
      format
        .rules
        .iter()
        .filter(|rule| rule.icon_set.is_some() || rule.stop_if_true)
        .map(|rule| Rule {
          priority: rule.priority,
          references: &format.sequence_of_references,
          icon_set: rule.icon_set.as_ref(),
          stop_rule: rule.stop_if_true.then_some(rule),
        }),
    );
  }
  if !rules.iter().any(|rule| rule.icon_set.is_some()) {
    return None;
  }
  rules.sort_by_key(|rule| rule.priority);

  for rule in rules {
    if let Some(icon_set) = rule.icon_set
      && let Some(selection) = evaluate_icon_set_rule(
        import,
        sheet,
        rule.references,
        icon_set,
        address,
        value,
        cache,
      )
    {
      return Some(selection);
    }
    if let Some(stop_rule) = rule.stop_rule
      && conditional_numeric_rule_matches(
        import,
        sheet,
        rule.references,
        stop_rule,
        address,
        value,
        cache,
      )
    {
      break;
    }
  }
  None
}

fn conditional_color_scale_fill(
  import: &ExcelImport,
  sheet: &CalcSheet,
  address: CellAddress,
  value: f64,
  cache: &mut ConditionalFormatEvalCache,
) -> Option<CalcPrintColorScaleFill> {
  let mut rules = sheet
    .metrics
    .conditions
    .conditional_formats
    .iter()
    .filter(|format| conditional_format_contains_cell(format, address))
    .flat_map(|format| {
      format.rules.iter().filter_map(|rule| {
        Some((
          rule.priority,
          format.sequence_of_references.as_slice(),
          rule.color_scale.as_ref()?,
        ))
      })
    })
    .collect::<Vec<_>>();
  rules.sort_by_key(|(priority, _, _)| *priority);
  rules.into_iter().find_map(|(priority, references, scale)| {
    evaluate_color_scale_rule(import, sheet, references, scale, address, value, cache)
      .map(|color| CalcPrintColorScaleFill { priority, color })
  })
}

fn evaluate_color_scale_rule(
  import: &ExcelImport,
  sheet: &CalcSheet,
  references: &[String],
  scale: &super::sheet_conditions::ColorScaleModel,
  address: CellAddress,
  value: f64,
  cache: &mut ConditionalFormatEvalCache,
) -> Option<RgbColor> {
  if !(2..=3).contains(&scale.points.len()) || !value.is_finite() {
    return None;
  }
  let range = conditional_reference_range(references, address)?;
  let stats = cache.stats_for_range(sheet, range);
  let minimum = *stats.sorted_values.first()?;
  let maximum = *stats.sorted_values.last()?;
  let base = conditional_format_base_address(references, address)?;
  let points = scale
    .points
    .iter()
    .map(|point| {
      Some((
        color_scale_threshold_value(
          import,
          sheet,
          point,
          base,
          minimum,
          maximum,
          &stats.sorted_values,
        )?,
        import.styles.spreadsheet_color(&point.color)?,
      ))
    })
    .collect::<Option<Vec<_>>>()?;
  if points.len() != scale.points.len() {
    return None;
  }
  let segment = points
    .windows(2)
    .find(|pair| value <= pair[1].0)
    .unwrap_or_else(|| &points[points.len() - 2..]);
  Some(interpolate_color_scale(
    value,
    segment[0].0,
    segment[0].1,
    segment[1].0,
    segment[1].1,
  ))
}

fn color_scale_threshold_value(
  import: &ExcelImport,
  sheet: &CalcSheet,
  point: &super::sheet_conditions::ColorScalePointModel,
  base: CellAddress,
  minimum: f64,
  maximum: f64,
  sorted_values: &[f64],
) -> Option<f64> {
  use super::sheet_conditions::IconSetThresholdType;

  let numeric = || point.value.as_deref()?.parse::<f64>().ok();
  match point.threshold_type {
    IconSetThresholdType::Number => numeric(),
    IconSetThresholdType::Percent => {
      numeric().map(|percent| minimum + (maximum - minimum) * percent / 100.0)
    }
    IconSetThresholdType::Maximum | IconSetThresholdType::AutomaticMaximum => Some(maximum),
    IconSetThresholdType::Minimum | IconSetThresholdType::AutomaticMinimum => Some(minimum),
    IconSetThresholdType::Formula => super::formula::evaluate_relative_formula_as_number(
      import,
      sheet,
      point.value.as_deref()?,
      base,
      base,
    ),
    IconSetThresholdType::Percentile => {
      let percentile = numeric()?.clamp(0.0, 100.0) / 100.0;
      let mut values = sorted_values.to_vec();
      ooxmlsdk_formula::calc::statistics::percentile_sorted(
        &mut values,
        percentile,
        ooxmlsdk_formula::calc::statistics::PercentileKind::Inc,
      )
    }
  }
}

fn interpolate_color_scale(
  value: f64,
  lower_value: f64,
  lower: RgbColor,
  upper_value: f64,
  upper: RgbColor,
) -> RgbColor {
  let ratio = if upper_value <= lower_value {
    if value <= lower_value { 0.0 } else { 1.0 }
  } else {
    ((value - lower_value) / (upper_value - lower_value)).clamp(0.0, 1.0)
  };
  let channel = |lower: u8, upper: u8| {
    (i32::from(lower) + (ratio * f64::from(i32::from(upper) - i32::from(lower))) as i32)
      .clamp(0, 255) as u8
  };
  RgbColor {
    r: channel(lower.r, upper.r),
    g: channel(lower.g, upper.g),
    b: channel(lower.b, upper.b),
  }
}

fn evaluate_icon_set_rule(
  import: &ExcelImport,
  sheet: &CalcSheet,
  references: &[String],
  icon_set: &super::sheet_conditions::IconSetModel,
  address: CellAddress,
  value: f64,
  cache: &mut ConditionalFormatEvalCache,
) -> Option<CalcPrintIconSet> {
  let count = icon_set.icon_set.icon_count();
  if count == 0 || icon_set.thresholds.len() != count {
    return None;
  }
  let range = conditional_reference_range(references, address)?;
  let stats = cache.stats_for_range(sheet, range);
  let minimum = *stats.sorted_values.first()?;
  let maximum = *stats.sorted_values.last()?;
  let base = conditional_format_base_address(references, address)?;
  let thresholds = icon_set
    .thresholds
    .iter()
    .map(|threshold| {
      Some((
        icon_set_threshold_value(
          threshold,
          IconSetThresholdContext {
            import,
            sheet,
            base,
            address,
            minimum,
            maximum,
            sorted_values: &stats.sorted_values,
          },
        )?,
        threshold.greater_than_or_equal,
      ))
    })
    .collect::<Option<Vec<_>>>()?;
  let selected = icon_set_selected_index(value, &thresholds, icon_set.reverse)?;
  let icon = if let Some(custom) = icon_set.custom_icons.as_ref() {
    custom.get(selected).copied().flatten().and_then(|icon| {
      (icon.icon_index < icon.icon_set.icon_count()).then_some((icon.icon_set, icon.icon_index))
    })
  } else {
    Some((icon_set.icon_set, selected))
  };
  Some(CalcPrintIconSet {
    icon,
    show_value: icon_set.show_value,
  })
}

fn icon_set_selected_index(value: f64, thresholds: &[(f64, bool)], reverse: bool) -> Option<usize> {
  let mut selected = thresholds
    .iter()
    .enumerate()
    .filter(|(_, (threshold, greater_than_or_equal))| {
      if *greater_than_or_equal {
        value >= *threshold
      } else {
        value > *threshold
      }
    })
    .map(|(index, _)| index)
    .next_back()?;
  if reverse {
    selected = thresholds.len() - 1 - selected;
  }
  Some(selected)
}

struct IconSetThresholdContext<'a> {
  import: &'a ExcelImport,
  sheet: &'a CalcSheet,
  base: CellAddress,
  address: CellAddress,
  minimum: f64,
  maximum: f64,
  sorted_values: &'a [f64],
}

fn icon_set_threshold_value(
  threshold: &super::sheet_conditions::IconSetThresholdModel,
  context: IconSetThresholdContext<'_>,
) -> Option<f64> {
  use super::sheet_conditions::IconSetThresholdType;

  let numeric = || threshold.value.as_deref()?.parse::<f64>().ok();
  match threshold.threshold_type {
    IconSetThresholdType::Number => numeric(),
    IconSetThresholdType::Percent => numeric()
      .map(|percent| context.minimum + (context.maximum - context.minimum) * percent / 100.0),
    IconSetThresholdType::Maximum | IconSetThresholdType::AutomaticMaximum => Some(context.maximum),
    IconSetThresholdType::Minimum | IconSetThresholdType::AutomaticMinimum => Some(context.minimum),
    IconSetThresholdType::Formula => {
      let formula = threshold.value.as_deref()?;
      super::formula::evaluate_relative_formula_as_number(
        context.import,
        context.sheet,
        formula,
        context.base,
        context.address,
      )
    }
    IconSetThresholdType::Percentile => {
      let percentile = numeric()?.clamp(0.0, 100.0) / 100.0;
      let mut values = context.sorted_values.to_vec();
      ooxmlsdk_formula::calc::statistics::percentile_sorted(
        &mut values,
        percentile,
        ooxmlsdk_formula::calc::statistics::PercentileKind::Inc,
      )
    }
  }
}

fn conditional_format_contains_cell(
  format: &super::sheet_conditions::ConditionalFormatModel,
  address: CellAddress,
) -> bool {
  conditional_references_contain_cell(&format.sequence_of_references, address)
}

fn conditional_references_contain_cell(references: &[String], address: CellAddress) -> bool {
  references.iter().any(|references| {
    references
      .split_whitespace()
      .filter_map(CellRange::parse_a1_range)
      .any(|range| range.contains(address))
  })
}

pub(super) fn conditional_numeric_rule_matches(
  import: &ExcelImport,
  sheet: &CalcSheet,
  references: &[String],
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
  value: f64,
  cache: &mut ConditionalFormatEvalCache,
) -> bool {
  match rule.rule_type {
    x::ConditionalFormatValues::Top10 => {
      conditional_top10_matches(sheet, references, rule, address, value, cache)
    }
    x::ConditionalFormatValues::AboveAverage => {
      conditional_average_matches(sheet, references, rule, address, value, cache)
    }
    x::ConditionalFormatValues::CellIs => {
      conditional_cell_is_matches(import, sheet, references, rule, address)
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
  cache: &mut ConditionalFormatEvalCache,
) -> bool {
  let Some(range) = conditional_reference_range(references, address) else {
    return false;
  };
  let stats = cache.stats_for_range(sheet, range);
  let values = &stats.sorted_values;
  if values.is_empty() {
    return false;
  }
  let mut rank = (rule.rank.unwrap_or(10) as usize).max(1);
  if rule.percent {
    // Excel truncates a percentage rank before selecting the cutoff, with
    // at least one numeric value. Twenty-one values at ten percent select
    // two; ties at that cutoff remain included by the comparisons below.
    rank = ((values.len() as f64 * rank as f64 / 100.0).floor() as usize).max(1);
  }
  rank = rank.min(values.len());
  if rule.bottom {
    value <= values[rank - 1]
  } else {
    value >= values[values.len() - rank]
  }
}

fn conditional_average_matches(
  sheet: &CalcSheet,
  references: &[String],
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
  value: f64,
  cache: &mut ConditionalFormatEvalCache,
) -> bool {
  let Some(range) = conditional_reference_range(references, address) else {
    return false;
  };
  let Some(average) = cache.stats_for_range(sheet, range).average else {
    return false;
  };
  let equal = rule.equal_average;
  if rule.above_average {
    value > average || (equal && (value - average).abs() <= f64::EPSILON)
  } else {
    value < average || (equal && (value - average).abs() <= f64::EPSILON)
  }
}

pub(super) fn conditional_cell_is_matches(
  import: &ExcelImport,
  sheet: &CalcSheet,
  references: &[String],
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
) -> bool {
  let Some(first) = rule.formulas.first() else {
    return false;
  };
  let Some(base) = conditional_format_base_address(references, address) else {
    return false;
  };
  // Office cellIs compares the underlying typed cell value. Numbers, text
  // and logical values keep their ordinary formula comparison ordering;
  // TRUE is not numeric 1 or the string "TRUE". A blank reference can compare
  // equal to either zero or FALSE, while errors never receive the format.
  // Translate the cell reference from the same sqref origin as the limits,
  // preserving relative/mixed references and recalculated formula types.
  let mut column = base.col;
  let mut value = String::new();
  while column > 0 {
    column -= 1;
    value.insert(0, (b'A' + (column % 26) as u8) as char);
    column /= 26;
  }
  value.push_str(&base.row.to_string());
  let first = first.trim().trim_start_matches('=');
  let compare = |operator| format!("{value}{operator}({first})");
  let predicate = match rule.operator.unwrap_or_default() {
    x::ConditionalFormattingOperatorValues::Equal => compare("="),
    x::ConditionalFormattingOperatorValues::NotEqual => compare("<>"),
    x::ConditionalFormattingOperatorValues::LessThan => compare("<"),
    x::ConditionalFormattingOperatorValues::LessThanOrEqual => compare("<="),
    x::ConditionalFormattingOperatorValues::GreaterThan => compare(">"),
    x::ConditionalFormattingOperatorValues::GreaterThanOrEqual => compare(">="),
    x::ConditionalFormattingOperatorValues::Between
    | x::ConditionalFormattingOperatorValues::NotBetween => {
      let Some(second) = rule.formulas.get(1) else {
        return false;
      };
      let second = second.trim().trim_start_matches('=');
      let between = format!(
        "OR(AND({value}>=({first}),{value}<=({second})),AND({value}>=({second}),{value}<=({first})))"
      );
      if rule.operator == Some(x::ConditionalFormattingOperatorValues::NotBetween) {
        format!("NOT({between})")
      } else {
        between
      }
    }
    _ => return false,
  };
  super::formula::evaluate_relative_formula_as_condition(
    import,
    sheet,
    &format!("IFERROR({predicate},FALSE())"),
    base,
    address,
  )
}

pub(super) fn conditional_text_rule_matches(
  import: &ExcelImport,
  sheet: &CalcSheet,
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  address: CellAddress,
) -> bool {
  // ECMA-376 §18.18.12 defines these predicates using SEARCH, LEFT/RIGHT
  // comparisons and LEN(TRIM()). Reuse formula semantics for case handling,
  // wildcard escapes, numeric coercion and errors; formatted display text
  // must not replace the underlying cell value. A1 is a relative placeholder
  // translated by the existing evaluator to the cell currently being styled.
  let predicate = match rule.rule_type {
    x::ConditionalFormatValues::ContainsBlanks => "LEN(TRIM(A1))=0".to_string(),
    x::ConditionalFormatValues::NotContainsBlanks => "LEN(TRIM(A1))>0".to_string(),
    kind => {
      let Some(text) = rule.text.as_deref() else {
        return false;
      };
      let text = format!("\"{}\"", text.replace('"', "\"\""));
      match kind {
        x::ConditionalFormatValues::ContainsText => format!("NOT(ISERROR(SEARCH({text},A1)))"),
        x::ConditionalFormatValues::NotContainsText => format!("ISERROR(SEARCH({text},A1))"),
        x::ConditionalFormatValues::BeginsWith => format!("LEFT(A1,LEN({text}))={text}"),
        x::ConditionalFormatValues::EndsWith => format!("RIGHT(A1,LEN({text}))={text}"),
        _ => return false,
      }
    }
  };
  super::formula::evaluate_relative_formula_as_condition(
    import,
    sheet,
    &predicate,
    CellAddress { col: 1, row: 1 },
    address,
  )
}

pub(super) fn conditional_expression_matches(
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
  if !conditional_references_contain_cell(references, address) {
    return None;
  }
  // The rule has one formula origin across the entire sqref list. Office's
  // conditional_formatting_multiple_ranges.xlsx colors D1 by comparing it
  // with E2, translating B2 from the first range's A1 origin. Restarting at
  // the containing range's D1 would incorrectly compare D1 with B2 instead.
  references
    .iter()
    .flat_map(|references| references.split_whitespace())
    .filter_map(CellRange::parse_a1_range)
    .next()
    .map(|range| range.start)
}

fn conditional_reference_range(references: &[String], address: CellAddress) -> Option<CellRange> {
  references
    .iter()
    .flat_map(|references| references.split_whitespace())
    .filter_map(CellRange::parse_a1_range)
    .find(|range| range.contains(address))
}

#[derive(Debug, Default)]
pub(super) struct ConditionalFormatEvalCache {
  ranges: HashMap<CellRange, ConditionalRangeStats>,
}

pub(super) fn conditional_statistical_cell_value(
  text: &str,
  data_type: Option<x::CellValues>,
) -> Option<f64> {
  // Statistical rules operate on numeric cells, not text that merely parses
  // as a number or a boolean's serialized 0/1. Use the same population for
  // thresholds and for selecting the cells that receive the visual format.
  if !matches!(data_type, None | Some(x::CellValues::Number)) {
    return None;
  }
  text.parse::<f64>().ok().filter(|value| value.is_finite())
}

#[derive(Debug)]
struct ConditionalRangeStats {
  sorted_values: Vec<f64>,
  average: Option<f64>,
}

impl ConditionalFormatEvalCache {
  fn stats_for_range(&mut self, sheet: &CalcSheet, range: CellRange) -> &ConditionalRangeStats {
    self.ranges.entry(range).or_insert_with(|| {
      // Above-average and top/bottom rules use one invariant population for
      // every cell in the same sqref. Calc's conditional format evaluation
      // likewise evaluates the range aggregate independently of the current
      // cell; cache that aggregate instead of rescanning and sorting the
      // worksheet once per formatted cell.
      let mut values = sheet
        .rows
        .iter()
        .flat_map(|row| row.cells.iter())
        .filter_map(|cell| {
          let address = cell.address()?;
          range
            .contains(address)
            .then(|| conditional_statistical_cell_value(&cell.display_text, cell.data_type))
            .flatten()
        })
        .collect::<Vec<_>>();
      let average = (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64);
      values.sort_by(|left, right| left.total_cmp(right));
      ConditionalRangeStats {
        sorted_values: values,
        average,
      }
    })
  }
}

fn pivot_display_text(
  sheet: &CalcSheet,
  address: CellAddress,
  text: String,
  preserve_excel_cache: bool,
  strings: &'static OfficeStringCatalog,
) -> String {
  if preserve_excel_cache {
    // The Office golden path opens workbooks read-only through Workbooks.Open.
    // Excel does not refresh PivotTable reports through that API, so the
    // worksheet's persisted captions and member text remain authoritative.
    // In particular, UI culture is not permission to translate cached labels.
    // Keep the no-UI branch below as the established LibreOffice DataPilot
    // compatibility path used by mapped visible-output fixtures.
    return text;
  }
  if let Some(text) = pivot_page_field_display_text(sheet, address, strings) {
    return text;
  }
  let Some(pivot) = pivot_table_for_cell(sheet, address) else {
    return text;
  };
  // A PivotTable without refreshOnLoad keeps the materialized worksheet
  // report authoritative.  Its captions are persisted application strings,
  // and opening the workbook for fixed output does not turn absence of an
  // explicit UI locale into permission to regenerate or translate them.
  // Reconstruct DataPilot captions only for the same refresh path that moves
  // the cached cells into freshly calculated output geometry.
  if !pivot.refresh_on_load {
    return text;
  }
  if super::pivot::pivot_render_geometry(pivot)
    .result_range
    .contains(address)
  {
    if pivot.show_missing && text.is_empty() {
      return pivot.missing_caption.clone().unwrap_or_default();
    }
    if pivot.show_error && is_spreadsheet_error_text(text.as_str()) {
      return pivot.error_caption.clone().unwrap_or_default();
    }
  }
  // field/member result captions from the imported pivot source instead of
  // keeping Excel's persisted generic "Row Labels"/"(blank)" strings.
  if pivot.calculated_only_data_fields {
    let table_start = pivot.output_geometry.table_start;
    if address == table_start
      && let Some(label) = pivot_row_label_text(pivot)
    {
      return label;
    }
    if address.row == table_start.row && address.col > table_start.col {
      return strings.pivot_empty().to_string();
    }
  }
  if is_pivot_row_labels_caption(pivot, text.as_str()) {
    return pivot_row_caption_text(pivot, text.as_str(), strings);
  }
  if is_pivot_column_labels_caption(pivot, text.as_str()) {
    return pivot_column_label_text(pivot, strings).unwrap_or(text);
  }
  if let Some(data_layout_caption) =
    pivot_data_layout_caption_text(pivot, address, text.as_str(), strings)
  {
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
    value
      if pivot
        .grand_total_caption
        .as_deref()
        .is_some_and(|caption| caption == value) =>
    {
      value.to_string()
    }
    "Grand Total" => pivot_grand_total_caption(pivot, strings),
    "Gesamtergebnis"
    | "Végösszeg"
    | "\u{041e}\u{0431}\u{0449}\u{0438}\u{0439} \u{0438}\u{0442}\u{043e}\u{0433}" => {
      pivot_grand_total_caption(pivot, strings)
    }
    "Total general" => pivot_grand_total_caption(pivot, strings),
    "Total" => pivot.data_field_names.first().cloned().unwrap_or(text),
    "(blank)" => strings.pivot_empty().to_string(),
    "N.év1" => "Q1".to_string(),
    "N.év2" => "Q2".to_string(),
    "N.év3" => "Q3".to_string(),
    "N.év4" => "Q4".to_string(),
    _ => {
      let total_suffix = format!(" {}", strings.pivot_total());
      if let Some(prefix) = text
        .strip_suffix(total_suffix.as_str())
        .filter(|prefix| !prefix.is_empty())
      {
        format!("{prefix} {}", strings.pivot_result())
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
  strings: &'static OfficeStringCatalog,
) -> Option<String> {
  if text != pivot.data_caption.as_str() {
    return None;
  }
  let caption = if pivot.data_caption == "Values" || pivot.data_caption.is_empty() {
    strings.pivot_data()
  } else {
    pivot.data_caption.as_str()
  };
  if pivot
    .row_field_indexes
    .iter()
    .position(|index| *index == -2)
    .is_some_and(|position| {
      // persisted dataCaption when the row-axis data layout still has a row
      // grand-total result; when row grand totals are disabled, the emitted
      // FieldCell caption comes from the data-layout dimension.
      !pivot.row_grand_totals
        && address.row == pivot.output_geometry.data_start.row.saturating_sub(1)
        && address.col == pivot.output_geometry.table_start.col + position as u32
    })
  {
    return Some(caption.to_string());
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
    return Some(caption.to_string());
  }
  None
}

fn pivot_page_field_display_text(
  sheet: &CalcSheet,
  address: CellAddress,
  strings: &'static OfficeStringCatalog,
) -> Option<String> {
  for pivot in &sheet.resources.pivot_tables.tables {
    if !pivot.refresh_on_load {
      continue;
    }
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
      return Some(pivot_page_field_value_text(&page_field.value, strings));
    }
  }
  None
}

fn pivot_page_field_value_text(
  value: &super::pivot::PivotPageFieldValue,
  strings: &'static OfficeStringCatalog,
) -> String {
  match value {
    super::pivot::PivotPageFieldValue::All => strings.pivot_all().to_string(),
    super::pivot::PivotPageFieldValue::Multiple => strings.pivot_multiple().to_string(),
    super::pivot::PivotPageFieldValue::Member(text) if text.is_empty() => {
      strings.pivot_empty().to_string()
    }
    super::pivot::PivotPageFieldValue::Member(text) => text.clone(),
  }
}

fn is_pivot_row_labels_caption(pivot: &super::pivot::PivotTableModel, text: &str) -> bool {
  if pivot
    .row_header_caption
    .as_deref()
    .is_some_and(|caption| caption == text)
  {
    return true;
  }
  matches!(
    text,
    "Row Labels"
      | "Sorcímkék"
      | "Zeilenbeschriftungen"
      | "\u{041d}\u{0430}\u{0437}\u{0432}\u{0430}\u{043d}\u{0438}\u{044f} \u{0441}\u{0442}\u{0440}\u{043e}\u{043a}"
  )
}

fn is_pivot_column_labels_caption(pivot: &super::pivot::PivotTableModel, text: &str) -> bool {
  pivot
    .column_header_caption
    .as_deref()
    .is_some_and(|caption| caption == text)
    || matches!(
      text,
      "Column Labels" | "Spaltenbeschriftungen" | "Oszlopcímkék"
    )
}

fn pivot_grand_total_caption(
  pivot: &super::pivot::PivotTableModel,
  strings: &'static OfficeStringCatalog,
) -> String {
  pivot
    .grand_total_caption
    .clone()
    .unwrap_or_else(|| format!("{} {}", strings.pivot_total(), strings.pivot_result()))
}

fn is_spreadsheet_error_text(text: &str) -> bool {
  text.starts_with('#') && (text.ends_with('!') || text.ends_with('?') || text == "#N/A")
}

fn pivot_table_for_cell(
  sheet: &CalcSheet,
  address: CellAddress,
) -> Option<&super::pivot::PivotTableModel> {
  sheet.resources.pivot_tables.tables.iter().find(|pivot| {
    super::pivot::pivot_render_geometry(pivot)
      .table_range
      .contains(address)
  })
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

fn pivot_row_caption_text(
  pivot: &super::pivot::PivotTableModel,
  text: &str,
  strings: &'static OfficeStringCatalog,
) -> String {
  if let Some(caption) = pivot
    .row_header_caption
    .as_ref()
    .filter(|caption| !caption.is_empty())
  {
    return caption.clone();
  }
  pivot_row_label_text(pivot).unwrap_or_else(|| {
    // generic compact-layout row caption when multiple row fields share one
    // output column, while localized persisted captions are imported through
    // the DataPilot source as the generic "Row Labels" text.
    if pivot.compact && pivot.row_field_names.len() > 1 {
      strings.pivot_row_labels().to_string()
    } else {
      text.to_string()
    }
  })
}

fn pivot_column_label_text(
  pivot: &super::pivot::PivotTableModel,
  strings: &'static OfficeStringCatalog,
) -> Option<String> {
  pivot
    .column_header_caption
    .as_ref()
    .filter(|caption| !caption.is_empty())
    .cloned()
    .or_else(|| (!pivot.column_field_names.is_empty()).then(|| pivot.column_field_names.join(" ")))
    .or_else(|| Some(strings.pivot_column_labels().to_string()))
}

fn pivot_header_number_format_code<'a>(
  import: &'a ExcelImport,
  sheet: &CalcSheet,
  address: CellAddress,
) -> Option<&'a str> {
  let pivot = pivot_table_for_cell(sheet, address)?;
  // Field number formats belong to the reconstructed DataPilot output.  For
  // a non-refreshing PivotTable the persisted worksheet cell XF is the
  // visible Office result and may intentionally differ from the cache field.
  if !pivot.refresh_on_load {
    return None;
  }
  let geometry = super::pivot::pivot_render_geometry(pivot);
  if address.row >= geometry.data_start.row
    && address.col >= geometry.table_range.start.col
    && address.col < geometry.data_start.col
  {
    let field_index = address.col.saturating_sub(geometry.table_range.start.col) as usize;
    return pivot
      .row_field_number_format_ids
      .get(field_index)
      .and_then(|id| id.and_then(|id| import.styles.number_format_code(id)));
  }
  if address.col >= geometry.data_start.col
    && address.row >= geometry.table_range.start.row + pivot.output_geometry.header_rows
    && address.row < geometry.data_start.row
  {
    let field_index = address
      .row
      .saturating_sub(geometry.table_range.start.row + pivot.output_geometry.header_rows)
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
  if !pivot.refresh_on_load {
    return None;
  }
  let geometry = super::pivot::pivot_render_geometry(pivot);
  let data_start_row = geometry.data_start.row;
  let data_start_col = geometry.data_start.col;
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

fn pivot_data_cell_text_override(sheet: &CalcSheet, address: CellAddress) -> Option<&str> {
  sheet
    .resources
    .pivot_tables
    .tables
    .iter()
    .filter(|pivot| pivot.refresh_on_load)
    .flat_map(|pivot| pivot.data_cell_text_overrides.iter())
    .find(|override_text| override_text.address == address)
    .map(|override_text| override_text.text.as_str())
}

pub(crate) fn rendered_number_text(
  raw: &str,
  format_code: Option<&str>,
  data_type: Option<
    ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues,
  >,
  date_1904: bool,
) -> (String, NumberFormatRenderState) {
  rendered_number_text_for_locale(raw, format_code, data_type, date_1904, None)
}

pub(crate) fn rendered_number_text_for_locale(
  raw: &str,
  format_code: Option<&str>,
  data_type: Option<
    ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues,
  >,
  date_1904: bool,
  format_locale: Option<&str>,
) -> (String, NumberFormatRenderState) {
  match data_type {
    Some(ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues::Boolean) => {
      // A SpreadsheetML boolean is a logical value, not a numeric value to
      // which the cell XF's number format is applied. Excel fixed output
      // therefore ignores producer-authored literal sections such as
      // `"IGAZ";"IGAZ";"HAMIS"` (tdf#122191). Emit canonical spelling here;
      // the worksheet print caller applies the Office UI resource afterward.
      return (
        if boolean_raw_value(raw) {
          "TRUE".to_string()
        } else {
          "FALSE".to_string()
        },
        NumberFormatRenderState::Boolean,
      );
    }
    Some(ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues::Error) => {
      return (raw.to_string(), NumberFormatRenderState::Error);
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
  let sections = split_number_format_sections(format_code);
  let Some(section_index) = number_format_section_index(&sections, value) else {
    return (
      raw.to_string(),
      NumberFormatRenderState::UnsupportedFormatCode,
    );
  };
  let section = sections[section_index];
  let cleaned_section = strip_number_format_markers(section);
  if sections.len() > 1 && cleaned_section.is_empty() {
    // An explicitly skipped numeric section suppresses the displayed value
    // (ECMA-376 18.8.31). For example, m/d/yyyy;; hides zero dates in Office.
    // Keep the numeric cell and its formatting so fills/borders still render.
    return (String::new(), NumberFormatRenderState::Number);
  }
  if cleaned_section.eq_ignore_ascii_case("General") {
    return (
      format_general_number(value),
      NumberFormatRenderState::General,
    );
  }
  if let Some(text) = literal_number_format_section(&cleaned_section) {
    // A literal number format changes the displayed text, not the cell's
    // value type. Even a number displayed as "TRUE" remains right-aligned
    // under General; only an actual boolean cell is centered.
    return (text, NumberFormatRenderState::Number);
  }
  let format = NumberFormatPattern::parse_section(section, section_index == 1);
  if format.date_time {
    if let Some(text) = render_elapsed_date_time(value, &format.section) {
      return (text, NumberFormatRenderState::DateTime);
    }
    return (
      format_serial_date_time(value, section, date_1904, format_locale),
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
  // Excel's General display budget is ten digits, counting the leading zero
  // of a fraction. Widening a column does not restore the raw double's extra
  // digits: independent worksheet and pivot controls both retain nine decimal
  // places for 0.7034962745892106 and 3.6512482151539434.
  if value == 0.0 {
    return "0".to_string();
  }
  let abs = value.abs();
  if abs < 1.0e11 {
    let integer_digits = if abs >= 1.0 {
      abs.log10().floor() as usize + 1
    } else {
      1
    };
    let decimals = 10usize.saturating_sub(integer_digits);
    let rounded = round_to_decimal_places(value, decimals as i32);
    let fixed = trim_general_fraction(format!("{rounded:.decimals$}"));
    // Exact small decimals such as 0.000000001 remain fixed. Values needing
    // more fractional places below 0.0001 use a six-digit scientific mantissa.
    // An eleven-digit integer is also fixed until rounding carries to 10^11.
    if rounded.abs() < 1.0e11 && (abs >= 1.0e-4 || fixed.parse::<f64>().ok() == Some(value)) {
      return fixed;
    }
  }
  let pattern = NumberFormatPattern::parse_section("0.#####E+00", false);
  let scientific = format_scientific_value(value, &pattern, pattern.scientific.as_ref().unwrap())
    .unwrap_or_else(|| value.to_string());
  scientific.split_once('E').map_or_else(
    || scientific.clone(),
    |(mantissa, exponent)| format!("{}E{exponent}", trim_general_fraction(mantissa.to_string())),
  )
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

fn numeric_format_color(
  code: &str,
  raw: &str,
  data_type: Option<x::CellValues>,
  styles: &super::styles::StylesCatalog,
) -> Option<RgbColor> {
  if !code.contains('[') {
    return None;
  }
  if matches!(
    data_type,
    Some(
      x::CellValues::Boolean
        | x::CellValues::Error
        | x::CellValues::String
        | x::CellValues::InlineString
        | x::CellValues::SharedString
        | x::CellValues::Date
    )
  ) {
    return None;
  }
  let value = raw.parse::<f64>().ok().filter(|value| value.is_finite())?;
  let sections = split_number_format_sections(code);
  let section = sections[number_format_section_index(&sections, value)?];
  // ECMA-376 18.8.31: color is a directive of the chosen format section,
  // not part of the displayed text. Quoted/escaped brackets remain literal.
  let marker = section.strip_prefix('[')?.split_once(']')?.0;
  styles.number_format_color(marker)
}

#[derive(Clone, Debug)]
enum NumberFormatAtom {
  Literal(char),
  Syntax(char),
  Reserve(char),
  Fill(char),
}

fn number_format_atoms(section: &str) -> Option<Vec<NumberFormatAtom>> {
  let mut atoms = Vec::new();
  let mut quoted = false;
  let mut chars = section.chars();
  while let Some(ch) = chars.next() {
    atoms.push(match ch {
      '"' => {
        quoted = !quoted;
        continue;
      }
      _ if quoted => NumberFormatAtom::Literal(ch),
      '\\' => NumberFormatAtom::Literal(chars.next()?),
      '_' => NumberFormatAtom::Reserve(chars.next()?),
      '*' => NumberFormatAtom::Fill(chars.next()?),
      _ => NumberFormatAtom::Syntax(ch),
    });
  }
  (!quoted).then_some(atoms)
}

fn number_layout_atom(atom: &NumberFormatAtom) -> NumberFormatLayoutItem {
  match *atom {
    NumberFormatAtom::Literal(ch) | NumberFormatAtom::Syntax(ch) => {
      NumberFormatLayoutItem::Text(ch.to_string())
    }
    NumberFormatAtom::Reserve(ch) => NumberFormatLayoutItem::Reserve(ch),
    NumberFormatAtom::Fill(ch) => NumberFormatLayoutItem::Fill(ch),
  }
}

fn coalesce_number_layout(items: Vec<NumberFormatLayoutItem>) -> Vec<NumberFormatLayoutItem> {
  let mut result = Vec::new();
  for item in items {
    if let NumberFormatLayoutItem::Text(text) = &item {
      if text.is_empty() {
        continue;
      }
      if let Some(NumberFormatLayoutItem::Text(previous)) = result.last_mut() {
        previous.push_str(text);
        continue;
      }
    }
    result.push(item);
  }
  result
}

fn number_format_layout(
  raw: &str,
  code: &str,
  state: NumberFormatRenderState,
) -> Option<Vec<NumberFormatLayoutItem>> {
  if !matches!(
    state,
    NumberFormatRenderState::Number
      | NumberFormatRenderState::Percent
      | NumberFormatRenderState::Text
  ) {
    return None;
  }
  let sections = split_number_format_sections(code);
  let text = state == NumberFormatRenderState::Text;
  let value = if text {
    0.0
  } else {
    raw.parse::<f64>().ok().filter(|v| v.is_finite())?
  };
  let index = if text {
    if sections.len() >= 4 {
      3
    } else if sections.len() == 1 {
      0
    } else {
      return None;
    }
  } else {
    number_format_section_index(&sections, value)?
  };
  let section = strip_number_format_markers(sections[index]);
  let atoms = number_format_atoms(&section)?;
  if !atoms.iter().any(|atom| {
    matches!(
      atom,
      NumberFormatAtom::Reserve(_) | NumberFormatAtom::Fill(_) | NumberFormatAtom::Syntax('?')
    )
  }) || atoms
    .iter()
    .filter(|atom| matches!(atom, NumberFormatAtom::Fill(_)))
    .count()
    > 1
  {
    return None;
  }
  if text {
    if !atoms
      .iter()
      .any(|atom| matches!(atom, NumberFormatAtom::Syntax('@')))
    {
      return None;
    }
    return Some(coalesce_number_layout(
      atoms
        .iter()
        .map(|atom| {
          if matches!(atom, NumberFormatAtom::Syntax('@')) {
            NumberFormatLayoutItem::Text(raw.to_string())
          } else {
            number_layout_atom(atom)
          }
        })
        .collect(),
    ));
  }
  let pattern = NumberFormatPattern::parse_section(&section, index == 1);
  // Calendar, scientific and rational pictures retain their existing formatter;
  // their field-width semantics are distinct from decimal placeholders.
  if pattern.date_time
    || pattern.scientific.is_some()
    || atoms
      .iter()
      .any(|atom| matches!(atom, NumberFormatAtom::Syntax('/' | '@' | '[' | ']')))
  {
    return None;
  }
  let digit = |atom: &NumberFormatAtom| matches!(atom, NumberFormatAtom::Syntax('0' | '#' | '?'));
  let Some(first) = atoms.iter().position(digit) else {
    return Some(coalesce_number_layout(
      atoms.iter().map(number_layout_atom).collect(),
    ));
  };
  let last = atoms.iter().rposition(digit)?;
  let decimal = atoms[..=last]
    .iter()
    .position(|atom| matches!(atom, NumberFormatAtom::Syntax('.')));
  let integer_end = decimal.unwrap_or(last + 1);
  let integer_start = first.min(integer_end);
  let integer_atoms = &atoms[integer_start..integer_end];
  let fraction_atoms = decimal.map_or(&[][..], |index| &atoms[index + 1..=last]);
  let decimals = fraction_atoms.iter().filter(|atom| digit(atom)).count();
  if decimals > 30 {
    return None;
  }
  let scaled = value.abs() * if pattern.percent { 100.0 } else { 1.0 }
    / 1000_f64.powi(i32::try_from(pattern.scale_commas).ok()?);
  let rounded = round_to_decimal_places(scaled, decimals as i32);
  let formatted = format!("{rounded:.decimals$}");
  let (integer, fraction) = formatted.split_once('.').unwrap_or((&formatted, ""));
  let required_integer = integer_atoms
    .iter()
    .filter(|atom| matches!(atom, NumberFormatAtom::Syntax('0')))
    .count();
  let integer = if integer == "0" && required_integer == 0 {
    String::new()
  } else {
    format!("{integer:0>required_integer$}")
  };
  let mut result = Vec::new();
  if value.is_sign_negative() && index != 1 {
    result.push(NumberFormatLayoutItem::Text("-".into()));
  }
  result.extend(atoms[..integer_start].iter().map(number_layout_atom));
  let grouped = integer_atoms
    .iter()
    .any(|atom| matches!(atom, NumberFormatAtom::Syntax(',')))
    && integer_atoms
      .iter()
      .all(|atom| matches!(atom, NumberFormatAtom::Syntax('0' | '#' | '?' | ',')));
  if grouped {
    let missing = integer_atoms
      .iter()
      .filter(|atom| digit(atom))
      .count()
      .saturating_sub(integer.len());
    let spaces = integer_atoms
      .iter()
      .filter(|atom| matches!(atom, NumberFormatAtom::Syntax('?')))
      .count()
      .min(missing);
    result.extend(std::iter::repeat_n(
      NumberFormatLayoutItem::Reserve('0'),
      spaces,
    ));
    result.push(NumberFormatLayoutItem::Text(group_integer(&integer)));
  } else {
    let mut digits = integer.chars().rev();
    let mut reversed = Vec::new();
    for atom in integer_atoms.iter().rev() {
      match atom {
        NumberFormatAtom::Syntax('0' | '#' | '?') => {
          if let Some(ch) = digits.next() {
            reversed.push(NumberFormatLayoutItem::Text(ch.to_string()));
          } else if matches!(atom, NumberFormatAtom::Syntax('0')) {
            reversed.push(NumberFormatLayoutItem::Text("0".into()));
          } else if matches!(atom, NumberFormatAtom::Syntax('?')) {
            reversed.push(NumberFormatLayoutItem::Reserve('0'));
          }
        }
        NumberFormatAtom::Syntax(',') => {}
        _ => reversed.push(number_layout_atom(atom)),
      }
    }
    result.push(NumberFormatLayoutItem::Text(
      digits.collect::<Vec<_>>().into_iter().rev().collect(),
    ));
    result.extend(reversed.into_iter().rev());
  }
  if decimal.is_some() {
    result.push(NumberFormatLayoutItem::Text(".".into()));
    let required = fraction_atoms
      .iter()
      .filter(|atom| digit(atom))
      .enumerate()
      .filter(|(_, atom)| matches!(atom, NumberFormatAtom::Syntax('0')))
      .map(|(index, _)| index + 1)
      .last()
      .unwrap_or(0);
    let significant = fraction
      .bytes()
      .rposition(|ch| ch != b'0')
      .map_or(0, |index| index + 1)
      .max(required);
    let mut digits = fraction.chars().enumerate();
    for atom in fraction_atoms {
      if digit(atom) {
        let (index, ch) = digits.next()?;
        if index < significant {
          result.push(NumberFormatLayoutItem::Text(ch.to_string()));
        } else if matches!(atom, NumberFormatAtom::Syntax('?')) {
          result.push(NumberFormatLayoutItem::Reserve('0'));
        }
      } else {
        result.push(number_layout_atom(atom));
      }
    }
  }
  result.extend(
    atoms[last + 1..]
      .iter()
      .skip(pattern.scale_commas)
      .map(number_layout_atom),
  );
  Some(coalesce_number_layout(result))
}

fn number_format_section_index(sections: &[&str], value: f64) -> Option<usize> {
  // Conditions in the first two sections replace their usual value tests;
  // a third numeric section handles the remaining values. Use the same
  // selection for formatting and color (ECMA-376 18.8.31).
  if sections
    .iter()
    .take(2)
    .any(|section| number_format_condition(section, value).is_some())
  {
    sections
      .iter()
      .take(3)
      .enumerate()
      .find(|(index, section)| match *index {
        0 => number_format_condition(section, value).unwrap_or({
          if sections.len() > 2 {
            value > 0.0
          } else {
            value >= 0.0
          }
        }),
        1 => number_format_condition(section, value).unwrap_or(sections.len() <= 2 || value < 0.0),
        _ => true,
      })
      .map(|(index, _)| index)
  } else {
    let index = if value.is_sign_negative() && sections.len() > 1 {
      1
    } else if value == 0.0 && sections.len() > 2 {
      2
    } else {
      0
    };
    (index < sections.len()).then_some(index)
  }
}

fn number_format_condition(mut section: &str, value: f64) -> Option<bool> {
  while let Some(rest) = section.strip_prefix('[') {
    let (marker, remaining) = rest.split_once(']')?;
    for operator in ["<=", ">=", "<>", "<", ">", "="] {
      if let Some(operand) = marker.strip_prefix(operator) {
        let operand = operand.parse::<f64>().ok()?;
        return Some(match operator {
          "<=" => value <= operand,
          ">=" => value >= operand,
          "<>" => value != operand,
          "<" => value < operand,
          ">" => value > operand,
          _ => value == operand,
        });
      }
    }
    section = remaining;
  }
  None
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
  let mut skip_next = false;
  let mut has_literal = false;
  for ch in section.chars() {
    if skip_next {
      skip_next = false;
      continue;
    }
    if escaped {
      output.push(ch);
      escaped = false;
      continue;
    }
    match ch {
      '\\' if !in_quote => {
        escaped = true;
        has_literal = true;
      }
      '"' => {
        in_quote = !in_quote;
        has_literal = true;
      }
      '_' | '*' if !in_quote => skip_next = true,
      _ if in_quote => output.push(ch),
      _ if ch.is_whitespace() => output.push(ch),
      _ => return None,
    }
  }
  if in_quote || escaped || !has_literal {
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
  scale_commas: usize,
  scientific: Option<ScientificNumberFormat>,
}

#[derive(Clone, Debug)]
struct ScientificNumberFormat {
  marker: char,
  positive_sign: bool,
  exponent_width: usize,
  mantissa_section: String,
  exponent_suffix: String,
}

fn split_scientific_number_format(section: &str) -> Option<(String, ScientificNumberFormat)> {
  let mut quoted = false;
  let mut escaped = false;
  for (index, ch) in section.char_indices() {
    if escaped {
      escaped = false;
      continue;
    }
    match ch {
      '"' => quoted = !quoted,
      '\\' | '_' | '*' if !quoted => escaped = true,
      'e' | 'E' if !quoted => {
        let rest = &section[index + 1..];
        let sign = rest.chars().next()?;
        if !matches!(sign, '+' | '-') || integer_placeholder_count(&section[..index]) == 0 {
          continue;
        }
        let exponent_width = rest[1..]
          .bytes()
          .take_while(|ch| matches!(*ch, b'0' | b'#'))
          .count();
        if exponent_width == 0 {
          continue;
        }
        let (mantissa_section, _) = strip_trailing_scaling_commas(&section[..index]);
        let suffix = &rest[1 + exponent_width..];
        let mut numeric_section = section[..index].to_string();
        numeric_section.push_str(suffix);
        return Some((
          numeric_section,
          ScientificNumberFormat {
            marker: ch,
            positive_sign: sign == '+',
            exponent_width,
            mantissa_section,
            exponent_suffix: scientific_number_format_suffix(suffix),
          },
        ));
      }
      _ => {}
    }
  }
  None
}

fn scientific_number_format_suffix(section: &str) -> String {
  let mut output = String::new();
  let mut quoted = false;
  let mut chars = section.chars();
  while let Some(ch) = chars.next() {
    match ch {
      '"' => quoted = !quoted,
      '\\' if !quoted => {
        if let Some(literal) = chars.next() {
          output.push(literal);
        }
      }
      '_' | '*' if !quoted => {
        chars.next();
      }
      _ => output.push(ch),
    }
  }
  output
}

impl NumberFormatPattern {
  fn parse_section(section: &str, suppress_negative_sign: bool) -> Self {
    let cleaned_section = strip_number_format_markers(section);
    let (cleaned_section, scientific) = match split_scientific_number_format(&cleaned_section) {
      Some((section, scientific)) => (section, Some(scientific)),
      None => (cleaned_section, None),
    };
    let (section, scale_commas) = strip_trailing_scaling_commas(&cleaned_section);
    let section = section.as_str();
    let mut pattern = Self {
      suppress_negative_sign,
      section: section.to_string(),
      scale_commas,
      scientific,
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
        '\\' if !in_quote => escaped = true,
        '_' if !in_quote => skip_next = true,
        '*' if !in_quote => emit_next_fill = true,
        '"' => in_quote = !in_quote,
        _ if in_quote => {
          if !after_decimal && !literal_prefix {
            integer_pattern.push('\\');
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
        'd' | 'D' | 'm' | 'M' | 'y' | 'Y' | 'h' | 'H' | 's' | 'S' | 'g' | 'G' | 'e' | 'E' => {
          pattern.date_time = true;
          literal_prefix = false;
        }
        // ECMA-376 18.8.31 lists a space as displayed literal text even
        // without quotes or an escape (for example, "P" #,##0.00).
        _ if literal_prefix => {
          pattern.prefix.push(ch);
        }
        _ if seen_digit && after_decimal => pattern.suffix.push(ch),
        _ => {
          if !after_decimal {
            integer_pattern.push(ch);
          }
        }
      }
    }
    pattern.integer_pattern = integer_pattern;
    pattern.grouping = pattern.integer_pattern.contains(',');
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
  let mut chars = section.chars();
  let mut in_quote = false;
  while let Some(ch) = chars.next() {
    // ECMA-376 §18.8.31: quoted and escaped characters are literal text,
    // even when they spell a color, condition or currency marker. The next
    // character after _ or * belongs to that spacing/fill instruction too.
    if ch == '"' {
      in_quote = !in_quote;
      output.push(ch);
      continue;
    }
    if !in_quote && matches!(ch, '\\' | '_' | '*') {
      output.push(ch);
      if let Some(next) = chars.next() {
        output.push(next);
      }
      continue;
    }
    if ch != '[' || in_quote {
      output.push(ch);
      continue;
    }
    let mut marker = String::new();
    let mut closed = false;
    for next in chars.by_ref() {
      if next == ']' {
        closed = true;
        break;
      }
      marker.push(next);
    }
    if !closed {
      output.push('[');
      output.push_str(&marker);
      break;
    }
    if let Some(currency) = number_format_currency_marker(&marker) {
      // Currency names are literal content, not another format program.
      // For example, USD must not turn into seconds/day tokens, and a
      // currency containing 0 or % must not add numeric placeholders.
      for character in currency.chars() {
        output.push('\\');
        output.push(character);
      }
    } else if !is_ignored_number_format_marker(&marker) {
      output.push('[');
      output.push_str(&marker);
      output.push(']');
    }
  }
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
  let mut value = if pattern.percent {
    value * 100.0
  } else {
    value
  };
  if pattern.scale_commas > 0 {
    value /= 1000_f64.powi(i32::try_from(pattern.scale_commas).unwrap_or(i32::MAX));
  }
  if let Some(scientific) = &pattern.scientific
    && let Some(text) = format_scientific_value(value, pattern, scientific)
  {
    return text;
  }
  // With one format section, Excel supplies the minus before any authored
  // prefix (for example -AMT 12.50). Only a selected negative section owns
  // the sign itself; a currency or text prefix does not suppress it.
  let sign = if value.is_sign_negative() && !pattern.suppress_negative_sign {
    "-"
  } else {
    ""
  };
  if let Some(fraction) = format_fraction_value(value.abs(), &pattern.section) {
    let mut output = String::new();
    output.push_str(sign);
    output.push_str(&pattern.prefix);
    output.push_str(&fraction);
    output.push_str(&pattern.suffix);
    return output.trim_end().to_string();
  }
  let decimals = fraction_placeholder_count(&pattern.section).unwrap_or(pattern.decimals);
  // Office rounds decimal ties away from zero and removes binary scaling
  // noise (decimal-format.xlsx: 1.005 -> 1.01, while 1.004 -> 1.00).
  // Share the spreadsheet ROUND policy before Rust formats the fixed digits.
  let scaled = round_to_decimal_places(value.abs(), i32::try_from(decimals).unwrap_or(i32::MAX));
  let formatted = format!("{:.*}", decimals, scaled);
  let (integer, fraction) = formatted.split_once('.').unwrap_or((&formatted, ""));
  let minimum_integer_digits = integer_pattern_tokens(&pattern.integer_pattern)
    .iter()
    .filter(|token| matches!(token, IntegerPatternToken::Placeholder('0')))
    .count();
  // # and ? do not display an insignificant integer zero. An authored
  // 0 placeholder still requires it (ECMA-376 18.8.31; 52348.xlsx).
  let integer = if integer == "0" && minimum_integer_digits == 0 {
    Cow::Borrowed("")
  } else if integer.len() < minimum_integer_digits {
    Cow::Owned(format!("{integer:0>minimum_integer_digits$}"))
  } else {
    Cow::Borrowed(integer)
  };
  let integer = if integer_pattern_has_literal_between_placeholders(&pattern.integer_pattern) {
    render_integer_pattern(&pattern.integer_pattern, &integer)
  } else if pattern.grouping {
    group_integer(&integer)
  } else {
    integer.to_string()
  };
  let (fraction, fraction_includes_suffix) =
    if let Some(rendered) = render_fraction_pattern(&pattern.section, fraction) {
      (rendered, true)
    } else if pattern.decimals > 0 {
      let mut output = String::from(".");
      output.push_str(fraction);
      (output, false)
    } else {
      (String::new(), false)
    };
  let mut output = String::new();
  output.push_str(sign);
  output.push_str(&pattern.prefix);
  output.push_str(&integer);
  output.push_str(&fraction);
  if !fraction_includes_suffix {
    output.push_str(&pattern.suffix);
  }
  output.trim().to_string()
}

fn format_scientific_value(
  value: f64,
  pattern: &NumberFormatPattern,
  scientific: &ScientificNumberFormat,
) -> Option<String> {
  if !value.is_finite() {
    return None;
  }
  // Separate decimal exponent normalization from binary powi at the source
  // exponent, so subnormal values such as 5e-324 never divide by 10^-324 = 0.
  let decimal = format!("{:e}", value.abs());
  let (mantissa, exponent) = decimal.split_once('e')?;
  let mut mantissa = mantissa.parse::<f64>().ok()?;
  let decimal_exponent = exponent.parse::<i32>().ok()?;
  // Multiple integer placeholders use engineering-style exponent groups.
  // This follows SvNumberformat's nCntPre exponent rescaling.
  let period = i32::try_from(integer_placeholder_count(&pattern.integer_pattern).max(1)).ok()?;
  let threshold = 10.0_f64.powi(period);
  if !threshold.is_finite() {
    return None;
  }
  let mut exponent = decimal_exponent.div_euclid(period) * period;
  mantissa *= 10.0_f64.powi(decimal_exponent - exponent);

  let mut mantissa_pattern =
    NumberFormatPattern::parse_section(&scientific.mantissa_section, false);
  // Percent and comma scaling have already been applied to the original
  // value. Preserve the selected negative section without applying it twice.
  mantissa_pattern.percent = false;
  mantissa_pattern.scale_commas = 0;
  mantissa_pattern.suppress_negative_sign = pattern.suppress_negative_sign;
  let rounded = round_to_decimal_places(mantissa, i32::try_from(mantissa_pattern.decimals).ok()?);
  if rounded >= threshold {
    mantissa /= threshold;
    exponent += period;
  }
  let mantissa = format_decimal_value(mantissa.copysign(value), &mantissa_pattern);
  let sign = if exponent < 0 {
    "-"
  } else if scientific.positive_sign {
    "+"
  } else {
    ""
  };
  let width = scientific.exponent_width;
  Some(format!(
    "{mantissa}{}{sign}{:0width$}{}",
    scientific.marker,
    exponent.unsigned_abs(),
    scientific.exponent_suffix
  ))
}

fn strip_trailing_scaling_commas(section: &str) -> (String, usize) {
  let mut last_placeholder_end = None;
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
      '0' | '#' | '?' if !in_quote => last_placeholder_end = Some(index + ch.len_utf8()),
      _ => {}
    }
  }
  let Some(start) = last_placeholder_end else {
    return (section.to_string(), 0);
  };
  let count = section[start..].chars().take_while(|ch| *ch == ',').count();
  if count == 0 {
    return (section.to_string(), 0);
  }
  let end = start + count;
  let mut normalized = String::with_capacity(section.len() - count);
  normalized.push_str(&section[..start]);
  normalized.push_str(&section[end..]);
  (normalized, count)
}

fn format_fraction_value(value: f64, section: &str) -> Option<String> {
  let slash_index = unescaped_char_index(section, '/')?;
  let denominator_placeholders = integer_placeholder_count(&section[slash_index + 1..]);
  if denominator_placeholders == 0 {
    return None;
  }
  let numerator_tokens = integer_pattern_tokens(&section[..slash_index]);
  let numerator_end = numerator_tokens
    .iter()
    .rposition(|token| matches!(token, IntegerPatternToken::Placeholder(_)))?;
  let numerator_start = numerator_tokens[..=numerator_end]
    .iter()
    .rposition(|token| !matches!(token, IntegerPatternToken::Placeholder(_)))
    .map_or(0, |index| index + 1);
  let integer_tokens = &numerator_tokens[..numerator_start];
  let mixed_fraction = integer_tokens
    .iter()
    .any(|token| matches!(token, IntegerPatternToken::Placeholder(_)));
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
  if !mixed_fraction {
    return Some(format!("{numerator}/{denominator}"));
  }
  // A separate integer field requests a mixed fraction. Without it, Excel
  // keeps an improper fraction such as 7/1 (tdf70455), rather than a plain
  // integer. Divide the rounded ratio so a fractional carry reaches the
  // whole-number field too (tdf81939 uses -3 14093/99532).
  let integer = numerator / denominator;
  let remainder = numerator % denominator;
  let minimum_integer_digits = integer_tokens
    .iter()
    .filter(|token| matches!(token, IntegerPatternToken::Placeholder('0')))
    .count();
  let integer_text = format!("{integer:0minimum_integer_digits$}");
  if remainder == 0 {
    Some(integer_text)
  } else if integer == 0 && minimum_integer_digits == 0 {
    Some(format!("{remainder}/{denominator}"))
  } else {
    Some(format!("{integer_text} {remainder}/{denominator}"))
  }
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
      '\\' if !in_quote => escaped = true,
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
      '\\' if !in_quote => {
        if let Some(next) = chars.next() {
          suffix.push(next);
        }
      }
      '_' if !in_quote => {
        if let Some(next) = chars.next()
          && matches!(next, '$' | '€' | '£' | '¥')
        {
          suffix.push(next);
        }
      }
      '*' if !in_quote => {
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
      '\\' if !in_quote => {
        if let Some(next) = chars.next() {
          tokens.push(IntegerPatternToken::Literal(next));
        }
      }
      '_' | '*' if !in_quote => {
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

fn format_serial_date_time(
  value: f64,
  code: &str,
  date_1904: bool,
  format_locale: Option<&str>,
) -> String {
  let (seconds, code) = rounded_calendar_time_picture((value - value.floor()) * 86_400.0, code);
  let day_carry = seconds / 86_400;
  let code = code.as_ref();
  // Excel rounds even a date-only picture to whole seconds. Rounding at the
  // displayed precision can carry into the next calendar day.
  let days = value.floor() as i64 + day_carry;
  let seconds = seconds - day_carry * 86_400;
  let days_since_unix = if date_1904 {
    days - 24_107
  } else if days < 60 {
    days - 25_568
  } else {
    days - 25_569
  };
  let (year, month, day) = match (date_1904, days) {
    (false, 0) => (1900, 1, 0),
    (false, 60) => (1900, 2, 29),
    _ => civil_from_days(days_since_unix),
  };
  let compatibility_weekday =
    (!date_1904 && (0..=60).contains(&days)).then(|| (days + 5).rem_euclid(7) as u8);
  let hour = seconds / 3_600;
  let minute = (seconds % 3_600) / 60;
  let second = seconds % 60;
  let field_value = u16::try_from(year).ok().and_then(|year| {
    Some(crate::options::FieldUpdateDateTime {
      year,
      month: u8::try_from(month).ok()?,
      day: u8::try_from(day).ok()?,
      hour: u8::try_from(hour).ok()?,
      minute: u8::try_from(minute).ok()?,
      second: u8::try_from(second).ok()?,
    })
  });
  if uses_system_long_date_format(code) {
    // NF_DATE_SYSTEM_LONG is resolved through the caller's format locale,
    // independently of the UI and document languages.
    if let Some(text) = field_value.and_then(|value| {
      crate::field_datetime::format_spreadsheet_system_long_date(
        format_locale,
        value,
        compatibility_weekday,
      )
    }) {
      return text;
    }
  }
  if uses_system_date_time_format(code, "$-F400")
    && let Some(text) = field_value
      .and_then(|value| crate::field_datetime::format_spreadsheet_system_time(format_locale, value))
  {
    return text;
  }
  if let Some(text) = field_value.and_then(|value| {
    crate::field_datetime::format_spreadsheet_date_picture_with_weekday(
      code,
      format_locale,
      value,
      compatibility_weekday,
    )
  }) {
    return text;
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

fn rounded_calendar_time_picture(seconds: f64, code: &str) -> (i64, Cow<'_, str>) {
  let fields = subsecond_format_fields(code);
  let Some(precision) = fields.iter().map(|field| field.len() - 1).max() else {
    return (seconds.round() as i64, Cow::Borrowed(code));
  };
  let scale = 10_i64.pow(precision as u32);
  let ticks = (round_to_decimal_places(seconds, precision as i32) * scale as f64).round() as i64;
  let fraction = ticks % scale;
  let mut picture = String::with_capacity(code.len());
  let mut start = 0;
  for field in fields {
    let width = field.len() - 1;
    let digits = fraction / 10_i64.pow((precision - width) as u32);
    picture.push_str(&code[start..field.start]);
    // The shared calendar formatter takes whole seconds. Pass the already
    // rounded fraction as literal text while keeping its locale/date tokens.
    picture.push_str(&format!("\".{digits:0width$}\""));
    start = field.end;
  }
  picture.push_str(&code[start..]);
  (ticks / scale, Cow::Owned(picture))
}

fn subsecond_format_fields(code: &str) -> Vec<std::ops::Range<usize>> {
  let mut fields = Vec::new();
  let mut chars = code.char_indices().peekable();
  while let Some((start, ch)) = chars.next() {
    match ch {
      ';' => break,
      '\\' | '_' | '*' => {
        chars.next();
      }
      '"' | '[' => {
        let closing = if ch == '[' { ']' } else { '"' };
        for (_, ch) in chars.by_ref() {
          if ch == closing {
            break;
          }
        }
      }
      '.' => {
        let mut end = start + 1;
        while chars.peek().is_some_and(|(_, ch)| *ch == '0') {
          chars.next();
          end += 1;
        }
        // MS-OI29500 NFPartSubSecond permits one to three zero digits.
        if (1..=3).contains(&(end - start - 1)) {
          fields.push(start..end);
        }
      }
      _ => {}
    }
  }
  fields
}

fn uses_system_long_date_format(code: &str) -> bool {
  uses_system_date_time_format(code, "$-F800")
}

fn uses_system_date_time_format(code: &str, system_marker: &str) -> bool {
  let mut chars = code.chars();
  while let Some(ch) = chars.next() {
    match ch {
      ';' => break,
      '\\' | '_' | '*' => {
        chars.next();
      }
      '"' => {
        for ch in chars.by_ref() {
          if ch == '"' {
            break;
          }
        }
      }
      '[' => {
        let mut marker = String::new();
        let mut closed = false;
        for ch in chars.by_ref() {
          if ch == ']' {
            closed = true;
            break;
          }
          marker.push(ch);
        }
        if closed && marker.trim().eq_ignore_ascii_case(system_marker) {
          return true;
        }
      }
      _ => {}
    }
  }
  false
}

fn render_elapsed_date_time(value: f64, code: &str) -> Option<String> {
  let clean = strip_number_format_markers(code);
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
    // ECMA-376 18.8.31 includes the built-in [h]:mm:ss format as well
    // as doubled elapsed fields. Read tokens here so quoted or escaped
    // bracket text cannot turn an ordinary clock format into elapsed time.
    if let Some((unit, width, mut consumed)) = elapsed_date_time_token(rest) {
      match unit {
        ElapsedDateTimeUnit::Hour => {
          output.push_str(&format_padded_number(total_hours, width));
        }
        ElapsedDateTimeUnit::Minute => {
          output.push_str(&format_padded_number(total_minutes, width));
        }
        ElapsedDateTimeUnit::Second => {
          consumed = render_elapsed_second_token(rest, total_seconds, width, &mut output);
        }
      }
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
      let mut chars = rest.chars();
      chars.next();
      chars.next();
      rest = chars.as_str();
    } else {
      output.push(ch);
      rest = &rest[ch.len_utf8()..];
    }
  }
  bracket_written.then_some(output)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ElapsedDateTimeUnit {
  Hour,
  Minute,
  Second,
}

fn elapsed_date_time_token(rest: &str) -> Option<(ElapsedDateTimeUnit, usize, usize)> {
  let (field, _) = rest.strip_prefix('[')?.split_once(']')?;
  let unit = match field.to_ascii_lowercase().as_str() {
    "h" | "hh" => ElapsedDateTimeUnit::Hour,
    "m" | "mm" => ElapsedDateTimeUnit::Minute,
    "s" | "ss" => ElapsedDateTimeUnit::Second,
    _ => return None,
  };
  Some((unit, field.len(), field.len() + 2))
}

fn render_elapsed_second_token(
  rest: &str,
  total_seconds: f64,
  width: usize,
  output: &mut String,
) -> usize {
  let mut consumed = width + 2;
  if let Some(fraction) = rest
    .get(consumed..)
    .and_then(|suffix| suffix.strip_prefix('.'))
  {
    let decimals = fraction
      .chars()
      .take_while(|ch| matches!(ch, '0' | '#' | '?'))
      .count();
    if decimals > 0 {
      let minimum_width = width + 1 + decimals;
      output.push_str(&format!("{total_seconds:0minimum_width$.decimals$}"));
      consumed += 1 + decimals;
      return consumed;
    }
  }
  output.push_str(&format_padded_number(total_seconds.round() as i64, width));
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
  Some(integer_placeholder_count(fraction))
}

fn render_fraction_pattern(section: &str, digits: &str) -> Option<String> {
  let fraction = split_number_format_decimal(section)?.1;
  let mut output = String::new();
  let mut chars = fraction.chars().peekable();
  let mut digit_index = 0usize;
  let mut in_quote = false;
  let mut escaped = false;
  let digit_chars = digits.chars().collect::<Vec<_>>();
  let required_digits = integer_pattern_tokens(fraction)
    .into_iter()
    .filter_map(|token| match token {
      IntegerPatternToken::Placeholder(ch) => Some(ch),
      IntegerPatternToken::Literal(_) => None,
    })
    .enumerate()
    .filter_map(|(index, ch)| (ch == '0').then_some(index + 1))
    .last()
    .unwrap_or(0);
  let significant_digits = digit_chars
    .iter()
    .rposition(|digit| *digit != '0')
    .map_or(0, |index| index + 1)
    .max(required_digits);
  while let Some(ch) = chars.next() {
    if escaped {
      output.push(ch);
      escaped = false;
      continue;
    }
    match ch {
      '\\' if !in_quote => escaped = true,
      '_' | '*' if !in_quote => {
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
        if ch == '0' || digit_index <= significant_digits {
          output.push(digit);
        } else if ch == '?' {
          output.push(' ');
        }
      }
      _ => output.push(ch),
    }
  }
  // A decimal separator remains visible when all fractional # digits are
  // omitted. Returning None here incorrectly reinstates fixed zero digits
  // in format_decimal_value (Office prints 0. and 1000. for #0.#).
  Some(format!(".{}", output.trim_end()))
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
  split_defined_name_ranges(formula)
    .into_iter()
    .filter_map(|range| {
      let range = range.trim().replace('$', "");
      CellRange::parse_a1_range(&range)
    })
    .collect()
}

fn parse_print_title_rows(formula: &str) -> Option<CellRange> {
  split_defined_name_ranges(formula)
    .into_iter()
    .find_map(|range| parse_row_or_column_title(range, true))
}

fn parse_print_title_columns(formula: &str) -> Option<CellRange> {
  split_defined_name_ranges(formula)
    .into_iter()
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
  use super::*;

  #[test]
  fn absent_cells_inherit_row_column_paint_without_creating_print_data() {
    use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
    use ooxmlsdk::parts::workbook_styles_part::WorkbookStylesPart;
    use ooxmlsdk::parts::worksheet_part::WorksheetPart;
    use ooxmlsdk::sdk::SpreadsheetDocumentType;

    // Excel COM and exported PDF controls: serialized cells without s clear
    // inherited fills, row s=0 clears columns, and row s requires customFormat.
    // The same controls verify hidden rows/columns and cleared merged anchors.
    for variant in ["ordinary", "hidden", "merged", "serialized"] {
      let mut package = SpreadsheetDocument::create(SpreadsheetDocumentType::Workbook);
      let workbook = package.add_workbook_part().unwrap();
      let worksheet = workbook
        .add_new_part_auto_id::<_, WorksheetPart>(&mut package)
        .unwrap();
      let id = workbook
        .get_id_of_part(&package, &worksheet)
        .unwrap()
        .to_string();
      workbook.set_data(&mut package, format!(r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="{id}"/></sheets></workbook>"#).into_bytes()).unwrap();
      let styles = workbook
        .add_new_part_auto_id::<_, WorkbookStylesPart>(&mut package)
        .unwrap();
      styles
        .set_data(
          &mut package,
          br#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
        <fills count="5"><fill><patternFill patternType="none"/></fill>
          <fill><patternFill patternType="solid"><fgColor rgb="FFE0FFFF"/></patternFill></fill>
          <fill><patternFill patternType="solid"><fgColor rgb="FF0000FF"/></patternFill></fill>
          <fill><patternFill patternType="solid"><fgColor rgb="FFFFA500"/></patternFill></fill>
          <fill><patternFill patternType="solid"><fgColor rgb="FFFFC0CB"/></patternFill></fill>
        </fills><cellXfs count="5"><xf fillId="0" applyFill="1"/>
          <xf fillId="1" applyFill="1"/><xf fillId="2" applyFill="1"/>
          <xf fillId="3" applyFill="1"/><xf fillId="4" applyFill="1"/>
        </cellXfs></styleSheet>"#
            .to_vec(),
        )
        .unwrap();
      let hidden = if variant == "hidden" {
        r#" hidden="1""#
      } else {
        ""
      };
      let (row1, row2, row3, row4, row5, merges) = match variant {
        "merged" => (
          r#"<c r="C1"/>"#,
          "",
          "",
          r#"<c r="B4" s="0"/>"#,
          "",
          r#"<mergeCells><mergeCell ref="C1:C3"/><mergeCell ref="B4:D5"/></mergeCells>"#,
        ),
        "serialized" => (
          "",
          r#"<c r="A2"><v>2</v></c>"#,
          r#"<c r="D3" s="1"/>"#,
          r#"<c r="A4" s="2"/>"#,
          r#"<c r="B5"/>"#,
          "",
        ),
        _ => ("", "", "", "", "", ""),
      };
      let c2 = if variant == "serialized" {
        r#"<c r="C2"/>"#
      } else {
        ""
      };
      worksheet.set_data(&mut package, format!(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
        <sheetFormatPr defaultColWidth="10" defaultRowHeight="20" customHeight="1"/>
        <cols><col min="1" max="2" style="1"/><col min="3" max="3" style="3"{hidden}/><col min="4" max="16384" style="1"/></cols>
        <sheetData><row r="1"><c r="A1"><v>1</v></c>{row1}</row>
          <row r="2" s="2" customFormat="1">{row2}<c r="B2" s="0"/>{c2}<c r="D2" s="4"/></row>
          <row r="3" s="0" customFormat="1"{hidden}>{row3}</row>
          <row r="4" s="2" customFormat="0">{row4}</row>
          <row r="5" s="2">{row5}</row>
          <row r="6"><c r="E6" s="0"><v>6</v></c></row>
        </sheetData>{merges}</worksheet>"#).into_bytes()).unwrap();
      let import =
        ExcelImport::import_document(&package, &crate::LayoutOptions::default()).unwrap();
      let sheet = &import.sheets[0];
      let used = sheet.used_range(&import.styles);
      assert_eq!(used, CellRange::parse_a1_range("A1:E6"));
      let area = CellRange::parse_a1_range("A1:E7").unwrap();
      let cells = print_cells_for_area(
        &import,
        sheet,
        area,
        &mut ConditionalFormatEvalCache::default(),
      );
      assert!(cells.len() <= 35);
      assert_eq!(sheet.used_range(&import.styles), used);
      assert!(cells.iter().all(|cell| area.contains(cell.address)));
      assert!(
        cells
          .iter()
          .filter(|cell| sheet.cell_at(cell.address).is_none())
          .all(|cell| {
            cell.text.is_empty()
              && cell.rendered_text.is_empty()
              && !cell.formula
              && cell.data_type.is_none()
              && cell.rich_text_runs.is_empty()
          })
      );
      assert_eq!(
        sheet
          .cell_at(CellAddress::parse_a1("A1").unwrap())
          .unwrap()
          .style_index,
        None
      );
      let expected = if variant == "serialized" {
        [
          "WCOCC", "WWWPB", "WWWCW", "BCOCC", "CWOCC", "CCOCW", "CCOCC",
        ]
      } else {
        [
          "WCOCC", "BWBPB", "WWWWW", "CCOCC", "CCOCC", "CCOCW", "CCOCC",
        ]
      };
      for (r, row) in expected.iter().enumerate() {
        for (c, mut color) in row.chars().filter(|c| *c != ' ').enumerate() {
          let address = CellAddress {
            col: c as u32 + 1,
            row: r as u32 + 1,
          };
          if variant == "hidden" && (address.row == 3 || address.col == 3) {
            assert!(!cells.iter().any(|cell| cell.address == address));
            continue;
          }
          if variant == "merged"
            && (address.col == 3 && address.row <= 3
              || (2..=4).contains(&address.col) && (4..=5).contains(&address.row))
          {
            color = 'W';
          }
          let expected = match color {
            'W' => None,
            'C' => Some(RgbColor {
              r: 224,
              g: 255,
              b: 255,
            }),
            'O' => Some(RgbColor {
              r: 255,
              g: 165,
              b: 0,
            }),
            'B' => Some(RgbColor { r: 0, g: 0, b: 255 }),
            'P' => Some(RgbColor {
              r: 255,
              g: 192,
              b: 203,
            }),
            _ => unreachable!(),
          };
          let actual = cells
            .iter()
            .find(|cell| cell.address == address)
            .filter(|_| !sheet.is_covered_merged_cell(address))
            .and_then(|cell| import.styles.fill_for_cell(cell.style_index).color);
          assert_eq!(actual, expected, "{variant} {address:?}");
        }
      }
    }
  }

  #[test]
  fn conditional_percent_ranks_truncate_before_selecting_the_cutoff() {
    use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
    use ooxmlsdk::parts::worksheet_part::WorksheetPart;
    use ooxmlsdk::sdk::SpreadsheetDocumentType;

    // Excel DisplayFormat counts for 1, 5, 10, 15, 25, 50, 99 and 100
    // percent, independently checked against exported PDF glyph colors.
    for (count, expected_counts) in [
      (4, [1, 1, 1, 1, 1, 2, 3, 4]),
      (21, [1, 1, 2, 3, 5, 10, 20, 21]),
      (29, [1, 1, 2, 4, 7, 14, 28, 29]),
    ] {
      let mut package = SpreadsheetDocument::create(SpreadsheetDocumentType::Workbook);
      let workbook = package.add_workbook_part().unwrap();
      let worksheet = workbook
        .add_new_part_auto_id::<_, WorksheetPart>(&mut package)
        .unwrap();
      let id = workbook
        .get_id_of_part(&package, &worksheet)
        .unwrap()
        .to_string();
      workbook.set_data(&mut package, format!(r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="{id}"/></sheets></workbook>"#).into_bytes()).unwrap();
      let mut rows = String::new();
      for row in 1..=count {
        rows.push_str(&format!(
          r#"<row r="{row}"><c r="A{row}"><v>{row}</v></c></row>"#
        ));
      }
      let mut rules = String::new();
      for bottom in [false, true] {
        for percent in [1, 5, 10, 15, 25, 50, 99, 100] {
          rules.push_str(&format!(r#"<conditionalFormatting sqref="A1:A{count}"><cfRule type="top10" percent="1" rank="{percent}" bottom="{}" priority="1"/></conditionalFormatting>"#, u8::from(bottom)));
        }
      }
      worksheet.set_data(&mut package, format!(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>{rows}</sheetData>{rules}</worksheet>"#).into_bytes()).unwrap();
      let import =
        ExcelImport::import_document(&package, &crate::LayoutOptions::default()).unwrap();
      let sheet = &import.sheets[0];
      let mut cache = ConditionalFormatEvalCache::default();
      for (index, format) in sheet
        .metrics
        .conditions
        .conditional_formats
        .iter()
        .enumerate()
      {
        let expected_count = expected_counts[index % 8];
        for row in 1..=count {
          let expected = if index < 8 {
            row > count - expected_count
          } else {
            row <= expected_count
          };
          assert_eq!(
            conditional_top10_matches(
              sheet,
              &format.sequence_of_references,
              &format.rules[0],
              CellAddress { col: 1, row },
              f64::from(row),
              &mut cache
            ),
            expected,
            "count={count} rule={index} row={row}"
          );
        }
      }
    }
  }

  #[test]
  fn conditional_cell_is_keeps_logical_numeric_text_and_blank_types() {
    use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
    use ooxmlsdk::parts::worksheet_part::WorksheetPart;
    use ooxmlsdk::sdk::SpreadsheetDocumentType;

    // Excel fixed-output controls: each row contains nine comparisons.
    // DisplayFormat observations were independently checked against PDF glyph colors.
    let values = [
      ("b", "<v>0</v>"),
      ("b", "<v>1</v>"),
      ("n", "<v>0</v>"),
      ("n", "<v>1</v>"),
      ("n", "<v>-1</v>"),
      ("n", "<v>2</v>"),
      ("inlineStr", "<is><t>FALSE</t></is>"),
      ("inlineStr", "<is><t>TRUE</t></is>"),
      ("inlineStr", "<is><t>0</t></is>"),
      ("inlineStr", "<is><t>1</t></is>"),
      ("inlineStr", "<is><t>a</t></is>"),
      ("e", "<f>1/0</f><v>#DIV/0!</v>"),
      ("e", "<f>NA()</f><v>#N/A</v>"),
      ("b", "<f>TRUE()</f><v>1</v>"),
      ("n", "<f>1</f><v>1</v>"),
      ("str", "<f>\"TRUE\"</f><v>TRUE</v>"),
      ("str", "<f>\"\"</f><v></v>"),
      ("n", ""),
    ];
    let first_limits = [
      "TRUE", "FALSE", "1", "0", "\"TRUE\"", "\"1\"", "$A$1", "TRUE()", "1/0",
    ];
    let second_limits = [
      "FALSE",
      "TRUE",
      "0",
      "1",
      "\"FALSE\"",
      "\"0\"",
      "FALSE()",
      "FALSE()",
      "TRUE",
    ];
    for (operator, expected) in [
      (
        "equal",
        [
          ".Y.......",
          "Y.....YY.",
          "...Y.....",
          "..Y......",
          ".........",
          ".........",
          ".........",
          "....Y....",
          ".........",
          ".....Y...",
          ".........",
          ".........",
          ".........",
          "Y.....YY.",
          "..Y......",
          "....Y....",
          ".........",
          ".Y.Y.....",
        ],
      ),
      (
        "notEqual",
        [
          "Y.YYYYYY.",
          ".YYYYY...",
          "YYY.YYYY.",
          "YY.YYYYY.",
          "YYYYYYYY.",
          "YYYYYYYY.",
          "YYYYYYYY.",
          "YYYY.YYY.",
          "YYYYYYYY.",
          "YYYYY.YY.",
          "YYYYYYYY.",
          ".........",
          ".........",
          ".YYYYY...",
          "YY.YYYYY.",
          "YYYY.YYY.",
          "YYYYYYYY.",
          "Y.Y.YYYY.",
        ],
      ),
      (
        "lessThan",
        [
          "Y.....YY.",
          ".........",
          "YYY.YYYY.",
          "YY..YYYY.",
          "YYYYYYYY.",
          "YY..YYYY.",
          "YY..Y.YY.",
          "YY....YY.",
          "YY..YYYY.",
          "YY..Y.YY.",
          "YY..Y.YY.",
          ".........",
          ".........",
          ".........",
          "YY..YYYY.",
          "YY....YY.",
          "YY..YYYY.",
          "Y.Y.YYYY.",
        ],
      ),
      (
        "lessThanOrEqual",
        [
          "YY....YY.",
          "Y.....YY.",
          "YYYYYYYY.",
          "YYY.YYYY.",
          "YYYYYYYY.",
          "YY..YYYY.",
          "YY..Y.YY.",
          "YY..Y.YY.",
          "YY..YYYY.",
          "YY..YYYY.",
          "YY..Y.YY.",
          ".........",
          ".........",
          "Y.....YY.",
          "YYY.YYYY.",
          "YY..Y.YY.",
          "YY..YYYY.",
          "YYYYYYYY.",
        ],
      ),
      (
        "greaterThan",
        [
          "..YYYY...",
          ".YYYYY...",
          ".........",
          "...Y.....",
          ".........",
          "..YY.....",
          "..YY.Y...",
          "..YY.Y...",
          "..YY.....",
          "..YY.....",
          "..YY.Y...",
          ".........",
          ".........",
          ".YYYYY...",
          "...Y.....",
          "..YY.Y...",
          "..YY.....",
          ".........",
        ],
      ),
      (
        "greaterThanOrEqual",
        [
          ".YYYYY...",
          "YYYYYYYY.",
          "...Y.....",
          "..YY.....",
          ".........",
          "..YY.....",
          "..YY.Y...",
          "..YYYY...",
          "..YY.....",
          "..YY.Y...",
          "..YY.Y...",
          ".........",
          ".........",
          "YYYYYYYY.",
          "..YY.....",
          "..YYYY...",
          "..YY.....",
          ".Y.Y.....",
        ],
      ),
      (
        "between",
        [
          "YY....YY.",
          "YY....YY.",
          "..YY.....",
          "..YY.....",
          ".........",
          ".........",
          "....Y....",
          "....Y....",
          ".....Y...",
          ".....Y...",
          ".........",
          ".........",
          ".........",
          "YY....YY.",
          "..YY.....",
          "....Y....",
          ".........",
          "YYYY..YY.",
        ],
      ),
      (
        "notBetween",
        [
          "..YYYY...",
          "..YYYY...",
          "YY..YYYY.",
          "YY..YYYY.",
          "YYYYYYYY.",
          "YYYYYYYY.",
          "YYYY.YYY.",
          "YYYY.YYY.",
          "YYYYY.YY.",
          "YYYYY.YY.",
          "YYYYYYYY.",
          ".........",
          ".........",
          "..YYYY...",
          "YY..YYYY.",
          "YYYY.YYY.",
          "YYYYYYYY.",
          "....YY...",
        ],
      ),
    ] {
      let mut package = SpreadsheetDocument::create(SpreadsheetDocumentType::Workbook);
      let workbook = package.add_workbook_part().unwrap();
      let worksheet = workbook
        .add_new_part_auto_id::<_, WorksheetPart>(&mut package)
        .unwrap();
      let id = workbook
        .get_id_of_part(&package, &worksheet)
        .unwrap()
        .to_string();
      workbook.set_data(&mut package, format!(r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="{id}"/></sheets></workbook>"#).into_bytes()).unwrap();
      let mut rows = String::from(r#"<row r="1"><c r="A1" t="b"><v>1</v></c></row>"#);
      for (index, (kind, payload)) in values.iter().enumerate() {
        let row = index + 3;
        rows.push_str(&format!(r#"<row r="{row}">"#));
        for col in 'B'..='J' {
          rows.push_str(&format!(r#"<c r="{col}{row}" t="{kind}">{payload}</c>"#));
        }
        rows.push_str("</row>");
      }
      let mut formats = String::new();
      for (index, col) in ('B'..='J').enumerate() {
        let first = first_limits[index];
        let second = if matches!(operator, "between" | "notBetween") {
          format!("<formula>{}</formula>", second_limits[index])
        } else {
          String::new()
        };
        formats.push_str(&format!(r#"<conditionalFormatting sqref="{col}3:{col}20"><cfRule type="cellIs" operator="{operator}" priority="1"><formula>{first}</formula>{second}</cfRule></conditionalFormatting>"#));
      }
      worksheet.set_data(&mut package, format!(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetData>{rows}</sheetData>{formats}</worksheet>"#).into_bytes()).unwrap();
      let import =
        ExcelImport::import_document(&package, &crate::LayoutOptions::default()).unwrap();
      let sheet = &import.sheets[0];
      for (index, pattern) in expected.iter().enumerate() {
        for (column, expected) in pattern.bytes().enumerate() {
          let format = &sheet.metrics.conditions.conditional_formats[column];
          let address = CellAddress {
            col: column as u32 + 2,
            row: index as u32 + 3,
          };
          assert_eq!(
            conditional_cell_is_matches(
              &import,
              sheet,
              &format.sequence_of_references,
              &format.rules[0],
              address
            ),
            expected == b'Y',
            "{operator}: {address:?}"
          );
        }
      }
    }
  }

  #[test]
  fn constant_data_bar_scales_have_finite_geometry() {
    use ooxmlsdk::schemas::x14::DataBarAxisPositionValues as Axis;
    for value in [-3.0, 0.0, 3.0] {
      for axis in [Axis::Automatic, Axis::Middle, Axis::None] {
        let (start, end, axis) = data_bar_extent(value, value, value, 0.0, 1.0, axis).unwrap();
        assert!(start.is_finite() && end.is_finite());
        assert!((0.0..=1.0).contains(&start) && (0.0..=1.0).contains(&end));
        assert!(axis.is_none_or(|axis| axis.is_finite()));
      }
    }
  }

  #[test]
  fn print_area_unions_preserve_quoted_sheet_names() {
    // Splitting inside this actual sheet name used to interpret the fragment
    // 'Sheet1 as a cell address and invent a second empty print area.
    assert_eq!(
      parse_defined_name_ranges("'Sheet1, Sheet3'!$A$1:$H$17"),
      vec![CellRange::new(
        CellAddress { col: 1, row: 1 },
        CellAddress { col: 8, row: 17 },
      )]
    );
    let expected = vec![
      CellRange::new(
        CellAddress { col: 1, row: 1 },
        CellAddress { col: 2, row: 2 },
      ),
      CellRange::new(
        CellAddress { col: 4, row: 3 },
        CellAddress { col: 4, row: 5 },
      ),
    ];
    assert_eq!(
      parse_defined_name_ranges("='O''Brien, Q1'!$A$1:$B$2, 'O''Brien, Q1'!$D$3:$D$5"),
      expected
    );
    assert_eq!(
      parse_defined_name_ranges("Sheet1!$A$1:$B$2,Sheet1!$D$3:$D$5"),
      expected
    );
  }

  #[test]
  fn print_title_unions_preserve_quoted_sheet_names() {
    let formula = "='O''Brien, Q1'!$3:$4, 'O''Brien, Q1'!$C:$D";
    assert_eq!(
      parse_print_title_rows(formula),
      Some(CellRange::new(
        CellAddress { col: 1, row: 3 },
        CellAddress { col: 1, row: 4 },
      ))
    );
    assert_eq!(
      parse_print_title_columns(formula),
      Some(CellRange::new(
        CellAddress { col: 3, row: 1 },
        CellAddress { col: 4, row: 1 },
      ))
    );
    assert_eq!(parse_print_title_rows("'O''Brien, Q1'!$C:$D"), None);
    assert_eq!(parse_print_title_columns("'O''Brien, Q1'!$3:$4"), None);
  }

  #[test]
  fn fit_zoom_ignores_trailing_font_only_cells_retained_for_headers() {
    use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
    use ooxmlsdk::parts::workbook_styles_part::WorkbookStylesPart;
    use ooxmlsdk::parts::worksheet_part::WorksheetPart;
    use ooxmlsdk::sdk::SpreadsheetDocumentType;

    let mut package = SpreadsheetDocument::create(SpreadsheetDocumentType::Workbook);
    let workbook = package.add_workbook_part().unwrap();
    let worksheet = workbook
      .add_new_part_auto_id::<_, WorksheetPart>(&mut package)
      .unwrap();
    let id = workbook
      .get_id_of_part(&package, &worksheet)
      .unwrap()
      .to_string();
    workbook.set_data(&mut package, format!(r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="{id}"/></sheets></workbook>"#).into_bytes()).unwrap();
    let styles = workbook
      .add_new_part_auto_id::<_, WorkbookStylesPart>(&mut package)
      .unwrap();
    styles.set_data(&mut package, br#"<styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><fonts count="2"><font><sz val="10"/><name val="Arial"/></font><font><b/><sz val="10"/><name val="Arial"/></font></fonts><fills count="2"><fill><patternFill patternType="none"/></fill><fill><patternFill patternType="solid"><fgColor rgb="FF000000"/></patternFill></fill></fills><cellXfs count="3"><xf fontId="0"/><xf fontId="1" applyFont="1"/><xf fillId="1" applyFill="1"/></cellXfs></styleSheet>"#.to_vec()).unwrap();
    let mut zooms = Vec::new();
    for (header, tail_style, explicit) in [
      (false, 1, false),
      (true, 1, false),
      (true, 2, false),
      (true, 1, true),
    ] {
      let footer = if header {
        "<headerFooter><oddFooter>Page &amp;P</oddFooter></headerFooter>"
      } else {
        ""
      };
      worksheet.set_data(&mut package, format!(r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetPr><pageSetUpPr fitToPage="1"/></sheetPr><sheetFormatPr defaultColWidth="12" defaultRowHeight="15"/><sheetData><row r="1"><c r="L1"><v>1</v></c><c r="M1" s="{tail_style}"/></row></sheetData><pageMargins left="0.5" right="0.5" top="0.5" bottom="0.5" header="0.25" footer="0.25"/><pageSetup paperSize="9" fitToWidth="1" fitToHeight="0"/>{footer}</worksheet>"#).into_bytes()).unwrap();
      let mut import =
        ExcelImport::import_document(&package, &crate::LayoutOptions::default()).unwrap();
      let sheet = &import.sheets[0];
      let area = CellRange::parse_a1_range("A1:M1").unwrap();
      let named = CalcPrintNamedRanges {
        resolved_print_areas: if explicit { vec![area] } else { Vec::new() },
        ..Default::default()
      };
      if header && !explicit {
        assert_eq!(
          implicit_used_range(&import, sheet, &named).unwrap().end.col,
          if tail_style == 1 { 12 } else { 13 }
        );
      }
      zooms.push(fit_zoom_to_pages(&import, sheet, &[area], &named, 1, 0));
      if !explicit {
        assert_eq!(CalcPrintDocument::from_import(&import).pages.len(), 1);
      }
      if header && !explicit {
        import.sheets[0].page_settings.fit_to_page = false;
        import.sheets[0]
          .metrics
          .settings
          .properties
          .page_setup
          .fit_to_page = false;
        assert_eq!(
          implicit_used_range(&import, &import.sheets[0], &named)
            .unwrap()
            .end
            .col,
          13
        );
      }
    }
    assert!(zooms[0] < 100);
    assert_eq!(zooms[0], zooms[1]);
    assert!(
      zooms[2] < zooms[1],
      "visible fill must enlarge the fit extent"
    );
    assert_eq!(zooms[2], zooms[3], "explicit print area keeps blank cells");
  }

  #[test]
  fn implicit_header_footer_uses_directly_formatted_cell_extent() {
    assert!(use_formatted_cell_extent_for_implicit_header_footer(
      false, true, false
    ));
    assert!(!use_formatted_cell_extent_for_implicit_header_footer(
      false, false, false
    ));
    assert!(!use_formatted_cell_extent_for_implicit_header_footer(
      true, true, false
    ));
    assert!(!use_formatted_cell_extent_for_implicit_header_footer(
      false, true, true
    ));
  }

  #[test]
  fn implicit_header_footer_keeps_only_formatted_horizontal_continuations() {
    let visible = CellRange::new(
      CellAddress { col: 1, row: 1 },
      CellAddress { col: 4, row: 49 },
    );
    let horizontal_continuation = CellRange::new(
      CellAddress { col: 5, row: 1 },
      CellAddress { col: 9, row: 49 },
    );
    assert!(keep_formatted_horizontal_header_footer_page(
      true,
      Some(visible),
      horizontal_continuation,
      true,
    ));

    // Profile: the empty page is the bottom-right Cartesian hole, not a new
    // horizontal band beyond the visible E column.
    assert!(!keep_formatted_horizontal_header_footer_page(
      true,
      Some(CellRange::new(
        CellAddress { col: 1, row: 1 },
        CellAddress { col: 5, row: 54 },
      )),
      CellRange::new(
        CellAddress { col: 5, row: 54 },
        CellAddress { col: 5, row: 54 },
      ),
      true,
    ));
    // Top Five: a vertical centered-format tail is not a horizontal page.
    assert!(!keep_formatted_horizontal_header_footer_page(
      true,
      Some(visible),
      CellRange::new(
        CellAddress { col: 1, row: 45 },
        CellAddress { col: 8, row: 49 },
      ),
      true,
    ));
    // Job Source: an explicit but default-equivalent XF does not qualify.
    assert!(!keep_formatted_horizontal_header_footer_page(
      true,
      Some(visible),
      horizontal_continuation,
      false,
    ));

    assert!(!should_skip_empty_print_page(
      true, true, false, true, false
    ));
    assert!(should_skip_empty_print_page(
      true, true, false, false, false
    ));
  }

  #[test]
  fn scaled_header_footer_stays_inside_authored_page_margins() {
    let mut settings = CalcPageSettings::default();
    settings.margin_left_in = 0.5;
    settings.margin_right_in = 0.5;
    settings.margin_top_in = 1.0;
    settings.margin_bottom_in = 0.5;
    settings.margin_header_in = 0.5;
    settings.margin_footer_in = 0.25;
    settings.scale = 90;
    let without_footer = print_content_size_pt(&settings);
    settings.header_footer.odd_footer = Some("&L&F".to_string());
    let with_footer = print_content_size_pt(&settings);
    let (page_width, page_height) = settings.page_size_pt();

    assert_eq!(with_footer, without_footer);
    assert!((with_footer.0 - (page_width - 72.0)).abs() <= f32::EPSILON);
    assert!((with_footer.1 - (page_height - 108.0)).abs() <= f32::EPSILON);
  }

  #[test]
  fn row_page_breaks_use_physical_grid_capacity_at_each_zoom() {
    use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
    use ooxmlsdk::parts::worksheet_part::WorksheetPart;
    use ooxmlsdk::sdk::SpreadsheetDocumentType;

    // These logical row metrics are isolated from font selection here. The
    // Office controls realize them from DengXian/Courier New; worksheet tests
    // cover that conversion separately. Last rows below are observed PDF and
    // Excel HPageBreaks results, including margins on either side of a dot.
    for (mixed, zoom, top, bottom, last_row) in [
      (true, 100, 0.75, 0.75, 56),
      (true, 100, 0.75, 0.74, 56),
      (true, 100, 0.75, 0.739, 57),
      (true, 100, 0.75, 0.73, 57),
      (true, 100, 0.74, 0.75, 56),
      (true, 100, 0.73, 0.75, 57),
      (false, 100, 0.75, 0.71, 58),
      (false, 100, 0.75, 0.705, 58),
      (false, 100, 0.75, 0.7045, 59),
      (false, 100, 0.75, 0.70, 59),
      (false, 50, 0.75, 0.705, 117),
      (false, 50, 0.75, 0.7045, 118),
      (false, 50, 0.75, 0.70, 118),
      (false, 200, 0.75, 0.525, 30),
      (false, 200, 0.75, 0.510, 30),
    ] {
      let mut package = SpreadsheetDocument::create(SpreadsheetDocumentType::Workbook);
      let workbook = package.add_workbook_part().unwrap();
      let worksheet = workbook
        .add_new_part_auto_id::<_, WorksheetPart>(&mut package)
        .unwrap();
      let id = workbook.get_id_of_part(&package, &worksheet).unwrap();
      let xml = format!(
        r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="{id}"/></sheets></workbook>"#
      );
      workbook.set_data(&mut package, xml.into_bytes()).unwrap();
      let rows = (1..=170)
        .map(|row| {
          let height = if mixed && row == 1 {
            r#" ht="22.2" customHeight="1""#
          } else if mixed && row >= 5 {
            r#" ht="12.72" customHeight="1""#
          } else {
            ""
          };
          format!(r#"<row r="{row}"{height}><c r="A{row}"><v>{row}</v></c></row>"#)
        })
        .collect::<String>();
      let xml = format!(
        r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetFormatPr defaultRowHeight="12.48" customHeight="1"/><sheetData>{rows}</sheetData><pageMargins left="0.7" right="0.7" top="{top}" bottom="{bottom}" header="0.3" footer="0.3"/><pageSetup paperSize="9" scale="{zoom}"/></worksheet>"#
      );
      worksheet.set_data(&mut package, xml.into_bytes()).unwrap();
      let import =
        ExcelImport::import_document(&package, &crate::LayoutOptions::default()).unwrap();
      assert!((import.sheets[0].row_height_pt(2) - 12.48).abs() < 1.0e-5);
      let print = CalcPrintDocument::from_import(&import);
      assert_eq!(
        print.pages[0].area.unwrap().end.row,
        last_row,
        "mixed={mixed} zoom={zoom} top={top} bottom={bottom}"
      );
      assert_eq!(print.pages[1].area.unwrap().start.row, last_row + 1);
    }
  }

  #[test]
  fn repeated_rows_reserve_scaled_space_only_on_continuation_pages() {
    use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
    use ooxmlsdk::parts::worksheet_part::WorksheetPart;
    use ooxmlsdk::sdk::SpreadsheetDocumentType;

    // Isolate pagination from font realization with authored 20pt rows.
    // Office's 71/100% controls keep the same first-page boundary with and
    // without titles; subsequent pages prepend the title at document zoom.
    for (zoom, first_end, second_end) in [(50, 69, 136), (100, 34, 66), (200, 17, 32)] {
      let mut package = SpreadsheetDocument::create(SpreadsheetDocumentType::Workbook);
      let workbook = package.add_workbook_part().unwrap();
      let worksheet = workbook
        .add_new_part_auto_id::<_, WorksheetPart>(&mut package)
        .unwrap();
      let id = workbook.get_id_of_part(&package, &worksheet).unwrap();
      let xml = format!(
        r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Sheet1" sheetId="1" r:id="{id}"/></sheets><definedNames><definedName name="_xlnm.Print_Titles" localSheetId="0">Sheet1!$3:$4</definedName></definedNames></workbook>"#
      );
      workbook.set_data(&mut package, xml.into_bytes()).unwrap();
      let rows = (1..=150)
        .map(|row| {
          format!(r#"<row r="{row}" ht="20" customHeight="1"><c r="A{row}"><v>{row}</v></c></row>"#)
        })
        .collect::<String>();
      let xml = format!(
        r#"<worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheetFormatPr defaultRowHeight="20" customHeight="1"/><sheetData>{rows}</sheetData><pageMargins left="0.5" right="0.5" top="1" bottom="1" header="0.3" footer="0.3"/><pageSetup paperSize="9" scale="{zoom}"/></worksheet>"#
      );
      worksheet.set_data(&mut package, xml.into_bytes()).unwrap();
      let import =
        ExcelImport::import_document(&package, &crate::LayoutOptions::default()).unwrap();
      let print = CalcPrintDocument::from_import(&import);
      assert_eq!(
        print.pages[0].area.unwrap().end.row,
        first_end,
        "zoom={zoom}"
      );
      assert!(repeat_rows_for_page(print.pages[0].area, print.pages[0].repeated_rows).is_none());
      assert_eq!(print.pages[1].area.unwrap().start.row, first_end + 1);
      assert_eq!(
        print.pages[1].area.unwrap().end.row,
        second_end,
        "zoom={zoom}"
      );
      assert_eq!(print.pages[1].repeated_rows.unwrap().start.row, 3);
      assert_eq!(print.pages[1].repeated_rows.unwrap().end.row, 4);
    }
  }

  #[test]
  fn page_cell_scan_includes_preceding_and_one_following_column_without_changing_rows() {
    let page = CellRange::new(
      CellAddress { col: 2, row: 3 },
      CellAddress { col: 7, row: 19 },
    );

    assert_eq!(
      page_cell_scan_area(page),
      CellRange::new(
        CellAddress { col: 1, row: 3 },
        CellAddress { col: 8, row: 19 },
      )
    );
  }

  #[test]
  fn raw_zero_based_manual_break_starts_the_next_one_based_row() {
    let page_break = crate::xlsx::worksheet::PageBreakModel {
      id: 51,
      min: 0,
      max: 9,
      manual: true,
      pivot: false,
    };

    assert!(!manual_page_break_starts_at(&page_break, 51, 1));
    assert!(manual_page_break_starts_at(&page_break, 52, 1));
  }

  #[test]
  fn drawing_print_bounds_include_shape_rotation() {
    let anchor = (10.0, 20.0, 90.0, 30.0);
    let (x, y, width, height) = excel_unrotated_drawing_bounds(anchor, 90.0);
    let (x, y, width, height) = rotated_drawing_bounds(x, y, width, height, 90.0);

    assert!((x - 10.0).abs() < 0.001);
    assert!((y - 20.0).abs() < 0.001);
    assert!((width - 90.0).abs() < 0.001);
    assert!((height - 30.0).abs() < 0.001);
  }

  #[test]
  fn print_titles_start_repeating_only_after_their_source_page() {
    let titles = CellRange::new(
      CellAddress { col: 1, row: 1 },
      CellAddress { col: 1, row: 2 },
    );
    let first_page = CellRange::new(
      CellAddress { col: 1, row: 1 },
      CellAddress { col: 8, row: 20 },
    );
    let later_page = CellRange::new(
      CellAddress { col: 1, row: 21 },
      CellAddress { col: 8, row: 40 },
    );

    assert_eq!(repeat_rows_for_page(Some(first_page), Some(titles)), None);
    assert_eq!(
      repeat_rows_for_page(Some(later_page), Some(titles)),
      Some(CellRange::new(
        CellAddress { col: 1, row: 1 },
        CellAddress { col: 8, row: 2 },
      ))
    );
  }

  #[test]
  fn required_integer_zeroes_are_padded_without_reinterpreting_literals() {
    // ECMA-376 18.8.31: 0 is a required digit, whereas quoted/escaped 0 is text.
    for (raw, code, expected) in [
      ("12", "00000", "00012"),
      ("-12", "00000", "-00012"),
      ("0", "00000", "00000"),
      ("123456", "00000", "123456"),
      ("12.36", "000.0", "012.4"),
      ("12", "000,000", "000,012"),
      ("12", "##0", "12"),
      ("12", r#"0"0"0"#, "102"),
      ("12", r"0\00", "102"),
      ("5", "000.0E+00", "005.0E+00"),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(code), None, false).0,
        expected,
        "{raw}: {code}"
      );
    }
  }

  #[test]
  fn numeric_format_colors_follow_sections_and_ignore_literal_markers() {
    let styles = super::super::styles::StylesCatalog::default();
    let red = Some(RgbColor { r: 255, g: 0, b: 0 });
    let blue = Some(RgbColor { r: 0, g: 0, b: 255 });
    for (raw, code, expected) in [
      ("-130", r##"\€#,##0.00;[RED]"-€"#,##0.00"##, red),
      ("130", r##"\€#,##0.00;[RED]"-€"#,##0.00"##, None),
      ("0", "0;[Red]-0;[Blue]0", blue),
      ("-1", "[Red]0", red),
      ("1", "[Color3]0", red),
      ("1", r#""[Red]"0"#, None),
      ("1", r"\[Red\]0", None),
      ("50", "[Red][<=100]0;[Blue][>100]0", red),
      ("150", "[Red][<=100]0;[Blue][>100]0", blue),
      ("10", "[Red][<0]0;[Blue][>100]0;0", None),
      ("-1", "[Red]0;[Blue][<0]0", blue),
    ] {
      assert_eq!(
        numeric_format_color(code, raw, None, &styles),
        expected,
        "{raw}: {code}"
      );
    }
    for data_type in [
      x::CellValues::Boolean,
      x::CellValues::Error,
      x::CellValues::SharedString,
    ] {
      assert_eq!(
        numeric_format_color("[Red]0", "1", Some(data_type), &styles),
        None
      );
    }
  }

  #[test]
  fn fraction_formats_keep_mixed_parts_and_selected_negative_signs() {
    for (raw, code, expected) in [
      (
        "-3.1415926535897931",
        r"#\ ?/?;[Red]\-#\ #/#####",
        "-3 14093/99532",
      ),
      ("3.1415926535897931", r"#\ ?/?;[Red]\-#\ #/#####", "3 1/7"),
      ("3.1415926535897931", "??/?", "22/7"),
      ("7", "??/?", "7/1"),
      ("16", "??/?", "16/1"),
      ("7", "# ?/?", "7"),
      ("0.25", "# ?/?", "1/4"),
      ("0.25", "0 ?/?", "0 1/4"),
      ("1.999999", "# ?/?", "2"),
      ("-1.5", r##""AMT "# ?/?"##, "-AMT 1 1/2"),
      ("1.5", r#"# ?/?" 00""#, "1 1/2 00"),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(code), None, false).0,
        expected,
        "{raw}: {code}"
      );
    }
  }

  #[test]
  fn system_long_date_markers_ignore_literal_and_later_section_text() {
    assert!(uses_system_long_date_format(
      "[Red][$-f800]dddd, mmmm dd, yyyy"
    ));
    for code in [
      r#""[$-F800]"d-mmm"#,
      r"\[$-F800\]d-mmm",
      "d-mmm;[$-F800]dddd",
      "[$-F800",
    ] {
      assert!(!uses_system_long_date_format(code), "{code}");
    }
    assert_eq!(
      rendered_number_text_for_locale(
        "43486",
        Some(r#""[$-F800]"d-mmm"#),
        None,
        false,
        Some("en-US")
      )
      .0,
      "[$-F800]21-Jan",
    );
  }

  #[test]
  fn optional_number_placeholders_omit_zeroes_but_keep_required_digits() {
    for (raw, code, expected) in [
      ("0", "#", ""),
      ("0", "?", ""),
      ("0", "0", "0"),
      ("0", "000", "000"),
      ("0.1", "#", ""),
      ("1", "#", "1"),
      ("0.25", "#.00", ".25"),
      ("0.25", "0.00", "0.25"),
      ("0", "#0.#", "0."),
      ("1000", "#0.#", "1000."),
      ("600.3", "#0.#", "600.3"),
      ("1.2", "0.###", "1.2"),
      ("1", "0.#0", "1.00"),
      ("1.2", "0.#0#", "1.20"),
      ("1.201", "0.#0#", "1.201"),
      ("1.2", r#"0.??" kg""#, "1.2  kg"),
      ("0", r#"[=0]?;0.00"#, ""),
      ("0", r#"[=0]?;0.00;"unused""#, ""),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(code), None, false).0,
        expected,
        "{raw}: {code}"
      );
    }
  }

  #[test]
  fn conditional_number_format_sections_share_value_and_color_selection() {
    let compact = r#"[>999999]#,,"M";[>999]#,"K";#"#;
    let precise = r#"[>999999]#.000,,"M";[>999]#.000,"K";#.000"#;
    for (raw, code, expected) in [
      ("1.02", compact, "1"),
      ("999", compact, "999"),
      ("1000", compact, "1K"),
      ("102102.102", compact, "102K"),
      ("999999", compact, "1000K"),
      ("1000000", compact, "1M"),
      ("1.02", precise, "1.020"),
      ("1021.02", precise, "1.021K"),
      ("1021021.02", precise, "1.021M"),
      ("1021021021.02", precise, "1021.021M"),
      ("0", r#"[=0]"none";0.00"#, "none"),
      ("0", r#"[=0]"A"\ "B";0"#, "A B"),
      ("0", r#"[=0]"A\B";0"#, r"A\B"),
      ("0.125", r#"[<0]"";0%"#, "13%"),
      (
        "1.25",
        "[=0]?;[<4.16666666666667][hh]:mm:ss;[hh]:mm",
        "30:00:00",
      ),
      ("5", "[=0]?;[<4.16666666666667][hh]:mm:ss;[hh]:mm", "120:00"),
      ("43486", "[>50000][$-804]dddd;[$-409]mmm", "Jan"),
      ("12345", "[>9999]0.00E+00;0.0", "1.23E+04"),
      ("123", "[>9999]0.00E+00;0.0", "123.0"),
      ("1.25", "[>10]0;[<0]0;General", "1.25"),
      ("0", "0;[<0]0", "0"),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(code), None, false).0,
        expected,
        "{raw}: {code}"
      );
    }
    let styles = super::super::styles::StylesCatalog::default();
    let colored = r#"[Red][<=100]0.0" low";[Blue][>100]0.00" high""#;
    for (raw, expected, color) in [
      ("100", "100.0 low", RgbColor { r: 255, g: 0, b: 0 }),
      ("101", "101.00 high", RgbColor { r: 0, g: 0, b: 255 }),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(colored), None, false).0,
        expected
      );
      assert_eq!(
        numeric_format_color(colored, raw, None, &styles),
        Some(color)
      );
    }
  }

  #[test]
  fn elapsed_time_formats_preserve_totals_padding_and_literal_brackets() {
    // Preserve the workbook's cached fifteen-digit decimal serial.
    let cached_serial = "3.14159265358979".parse::<f64>().unwrap();
    for (value, code, expected) in [
      (1.10538194444444, "[h]:mm:ss", "26:31:45"),
      (1.40650462962963, "[h]:mm:ss", "33:45:22"),
      (1.25, "[h]:mm:ss;@", "30:00:00"),
      (0.0625, "[h]:mm", "1:30"),
      (0.0625, "[HH]:mm", "01:30"),
      (90.0 / 86_400.0, "[m]:ss", "1:30"),
      (90.0 / 86_400.0, "[MM]:ss", "01:30"),
      (3_735.8 / 86_400.0, "[mm]:ss", "62:16"),
      (5.0 / 86_400.0, "[s]", "5"),
      (5.0 / 86_400.0, "[SS]", "05"),
      (0.25 / 86_400.0, "[s].00", "0.25"),
      (0.25 / 86_400.0, "[ss].00", "00.25"),
      (cached_serial, "[ss].00", "271433.61"),
      (1.25, r#""[hh] "[h]:mm"#, "[hh] 30:00"),
      (1.25, "_时[h]:mm", "30:00"),
    ] {
      let (text, state) = rendered_number_text(&value.to_string(), Some(code), None, false);
      assert_eq!(text, expected, "{value}: {code}");
      assert_eq!(state, NumberFormatRenderState::DateTime, "{code}");
    }
    for code in [r#""[hh]" h:mm"#, r"\[hh\] h:mm", "h:mm:ss"] {
      assert_eq!(render_elapsed_date_time(1.25, code), None, "{code}");
    }
  }

  #[test]
  fn negative_numbers_keep_the_sign_before_literal_prefixes() {
    for (raw, code, expected) in [
      ("-12.5", r#""AMT"\ #,##0.00"#, "-AMT 12.50"),
      ("12.5", r#""AMT"\ #,##0.00"#, "AMT 12.50"),
      ("-38", r##""$"#,##0"##, "-$38"),
      ("-5.2", r#""NEG"\ 0.00;("NEG"\ 0.00)"#, "(NEG 5.20)"),
      ("-1.2", r#""P "0.00;"N "0.00"#, "N 1.20"),
      ("1.2", r##""P" #,##0.00; "N" #,##0.00;0;@"##, "P 1.20"),
      ("-1.2", r##""P" #,##0.00; "N" #,##0.00;0;@"##, "N 1.20"),
      ("1.2", r#""P"  0.00"#, "P  1.20"),
      ("1.2", r#"0.00 "units""#, "1.20 units"),
      ("-0.125", r#""rate "0.0%"#, "-rate 12.5%"),
      ("-12300", r#""mass "0.00E+00"#, "-mass 1.23E+04"),
      (
        "-12300",
        r#""mass "0.00E+00;("mass "0.00E+00)"#,
        "(mass 1.23E+04)",
      ),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(code), None, false).0,
        expected,
        "{raw}: {code}",
      );
    }
  }

  #[test]
  fn decimal_format_rounding_preserves_half_ties_and_scientific_carries() {
    for (raw, code, expected) in [
      ("1.005", "0.00", "1.01"),
      ("1.004", "0.00", "1.00"),
      ("1.0049", "0.00", "1.00"),
      ("-1.005", "0.00", "-1.01"),
      ("1.25", "0.0", "1.3"),
      ("-1.25", "0.0", "-1.3"),
      ("9.995", "0.00", "10.00"),
      ("999.995", "#,##0.00", "1,000.00"),
      ("0.01005", "0.00%", "1.01%"),
      ("1005", "0.00,", "1.01"),
      ("1.005", "0.00E+00", "1.01E+00"),
      ("9.995", "0.00E+00", "1.00E+01"),
      ("999.95", "##0.0E+00", "1.0E+03"),
      ("-9.995", "0.00E+00;(0.00E+00)", "(1.00E+01)"),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(code), None, false).0,
        expected,
        "{raw}: {code}",
      );
    }
  }

  #[test]
  fn scientific_number_formats_separate_mantissa_and_exponent() {
    // ECMA-376 §18.8.31: E+/E- select the exponent sign policy, and
    // placeholders after E belong to the exponent rather than the fraction.
    for (raw, code, expected) in [
      ("103000000", "0.00E+00", "1.03E+08"),
      ("0.00123", "0.00E+00", "1.23E-03"),
      ("-12300", "0.00E-00", "-1.23E04"),
      ("0", "0.00E+00", "0.00E+00"),
      ("12300", "0.0e+000", "1.2e+004"),
      ("12345", "##0.0E+0", "12.3E+3"),
      ("0.012345", "##0.0E+0", "12.3E-3"),
      ("9.9996", "0.00E+00", "1.00E+01"),
      ("999.96", "##0.0E+0", "1.0E+3"),
      ("5e-324", "0.00E+00", "5.00E-324"),
      ("1.7976931348623157e308", "0.00E+00", "1.80E+308"),
      ("12300", r#""mass "0.00E+00" kg""#, "mass 1.23E+04 kg"),
      ("12300", r#"0.00"x"E+00" kg""#, "1.23xE+04 kg"),
      ("1.26", r#"0.0"0"E+00"#, "1.30E+00"),
      ("-12300", "0.00E+00;(0.00E+00)", "(1.23E+04)"),
      ("12300000", "0.0,,E+00", "1.2E+01"),
      ("12", r#""E+00 "0"#, "E+00 12"),
    ] {
      let (text, state) = rendered_number_text(raw, Some(code), None, false);
      assert_eq!(text, expected, "{raw}: {code}");
      assert_eq!(state, NumberFormatRenderState::Number, "{code}");
    }
    // Scientific notation does not remove the percentage value category.
    assert_eq!(
      rendered_number_text("0.0123", Some("0.00E+00%"), None, false),
      ("1.23E+00%".to_string(), NumberFormatRenderState::Percent)
    );
  }

  #[test]
  fn system_time_formats_use_the_format_locale_instead_of_the_saved_picture() {
    for (value, expected) in [
      (0.0, "0:00:00"),
      (0.25, "6:00:00"),
      (0.375, "9:00:00"),
      (0.5, "12:00:00"),
      (60.25, "6:00:00"),
      (40_000.25, "6:00:00"),
    ] {
      for picture in ["[$-F400]h:mm:ss", "[$-F400]hh:mm:ss"] {
        assert_eq!(
          format_serial_date_time(value, picture, false, Some("zh-CN")),
          expected
        );
      }
    }
    for language in ["zh-CN", "en-US"] {
      let expected = if language == "zh-CN" {
        "13:30:55"
      } else {
        "1:30:55 PM"
      };
      assert_eq!(
        rendered_number_text_for_locale(
          "0.56313888888888886",
          Some(r"[$-F400]h:mm:ss\ AM/PM"),
          None,
          false,
          Some(language)
        )
        .0,
        expected,
      );
    }
    assert!(uses_system_date_time_format("[$-f400]h:mm:ss", "$-F400"));
    for code in [
      r#""[$-F400]"h:mm:ss"#,
      r"\[$-F400\]h:mm:ss",
      "h:mm:ss;[$-F400]h:mm",
      "[$-F400",
    ] {
      assert!(!uses_system_date_time_format(code, "$-F400"), "{code}");
    }
    assert!(!uses_system_long_date_format("[$-F400]h:mm:ss"));
    assert!(!uses_system_date_time_format("[$-F800]dddd", "$-F400"));
  }

  #[test]
  fn excel_1900_calendar_preserves_fictitious_days_and_localized_weekdays() {
    // Excel 20326 controls: serials 0/59/60/61, explicit 409/804 LCIDs,
    // system F800/F400, and the same numeric values under date1904.
    for (serial, iso, english, chinese) in [
      (
        0,
        "1900-01-00",
        "Saturday, January 0, 1900",
        "星期六 一月 0 1900",
      ),
      (
        1,
        "1900-01-01",
        "Sunday, January 1, 1900",
        "星期日 一月 1 1900",
      ),
      (
        59,
        "1900-02-28",
        "Tuesday, February 28, 1900",
        "星期二 二月 28 1900",
      ),
      (
        60,
        "1900-02-29",
        "Wednesday, February 29, 1900",
        "星期三 二月 29 1900",
      ),
      (
        61,
        "1900-03-01",
        "Thursday, March 1, 1900",
        "星期四 三月 1 1900",
      ),
    ] {
      for (picture, expected) in [
        ("yyyy-mm-dd", iso),
        ("[$-409]dddd, mmmm d, yyyy", english),
        ("[$-804]dddd mmmm d yyyy", chinese),
      ] {
        assert_eq!(
          format_serial_date_time(serial as f64, picture, false, None),
          expected
        );
      }
    }
    for (picture, expected) in [
      ("d", "0"),
      ("dd", "00"),
      ("mmmm", "January"),
      ("ddd", "Sat"),
      ("e", "1900"),
      (r#""d"dd"d" \d yyyy-mm-dd"#, "d00d d 1900-01-00"),
    ] {
      assert_eq!(format_serial_date_time(0.0, picture, false, None), expected);
    }
    for (picture, expected) in [
      ("[$-F800]dddd, mmmm dd, yyyy", "1900年1月0日"),
      ("[$-F400]h:mm:ss", "0:00:00"),
    ] {
      assert_eq!(
        format_serial_date_time(0.0, picture, false, Some("zh-CN")),
        expected
      );
    }
    for (serial, expected) in [
      (0, "1904-01-01"),
      (1, "1904-01-02"),
      (59, "1904-02-29"),
      (60, "1904-03-01"),
      (61, "1904-03-02"),
    ] {
      assert_eq!(
        format_serial_date_time(serial as f64, "yyyy-mm-dd", true, None),
        expected
      );
    }
  }

  #[test]
  fn date_only_pictures_carry_at_the_whole_second_boundary() {
    // Office brackets half a second before midnight at these two serials.
    for (serial, expected_date, expected_time) in [
      (0.999_994_21, "1900-01-00", "23:59:59"),
      (0.999_994_22, "1900-01-01", "00:00:00"),
      (1.999_994_21, "1900-01-01", "23:59:59"),
      (1.999_994_22, "1900-01-02", "00:00:00"),
      (59.999_999_995_4, "1900-02-29", "00:00:00"),
      (60.999_999_995_4, "1900-03-01", "00:00:00"),
    ] {
      assert_eq!(
        format_serial_date_time(serial, "yyyy-mm-dd", false, None),
        expected_date
      );
      assert_eq!(
        format_serial_date_time(serial, "yyyy-mm-dd hh:mm:ss", false, None),
        format!("{expected_date} {expected_time}")
      );
    }
  }

  #[test]
  fn calendar_time_formats_preserve_subseconds_and_rounding_carries() {
    for (seconds, code, expected) in [
      (48_655.2, "mm:ss.0", "30:55.2"),
      (1_855.2, "mm:ss.0;@", "30:55.2"),
      (14_706.009, "hh:mm:ss.000", "04:05:06.009"),
      (14_706.009, "h:m:s.00", "4:5:6.01"),
      (59.9996, "mm:ss.000", "01:00.000"),
      (3_599.96, "hh:mm:ss.0", "01:00:00.0"),
      (48_655.2, r#"hh:mm:ss".000""#, "13:30:55.000"),
      (48_655.2, r"hh:mm:ss\.000", "13:30:55.000"),
      (48_655.2, r#"hh:mm:ss.0" seconds""#, "13:30:55.2 seconds"),
    ] {
      assert_eq!(
        rendered_number_text(&(seconds / 86_400.0).to_string(), Some(code), None, false).0,
        expected,
        "{code}"
      );
    }
    for (date_1904, expected) in [
      (false, "1900-01-02 00:00:00.000"),
      (true, "1904-01-03 00:00:00.000"),
    ] {
      let serial = 1.0 + 86_399.999_6 / 86_400.0;
      assert_eq!(
        rendered_number_text(
          &serial.to_string(),
          Some("yyyy-mm-dd hh:mm:ss.000"),
          None,
          date_1904
        )
        .0,
        expected
      );
    }
    let before_midnight = 1.0 + 86_399.999_6 / 86_400.0;
    assert_eq!(
      rendered_number_text(
        &before_midnight.to_string(),
        Some("yyyy-mm-dd"),
        None,
        false
      )
      .0,
      "1900-01-02"
    );
    for code in [
      r#"hh:mm:ss".000""#,
      r"hh:mm:ss\.000",
      "[$-0409]hh:mm:ss;ss.0",
      "hh:mm:ss_.000",
      "hh:mm:ss*时",
      "hh:mm:ss.0000",
    ] {
      assert!(subsecond_format_fields(code).is_empty(), "{code}");
    }
    assert_eq!(
      rendered_number_text_for_locale(
        "37653.170208437499",
        Some(r"[$-0409]dd\-mmm\-yyyy\ hh:mm:ss.000"),
        None,
        false,
        Some("zh-CN")
      )
      .0,
      "01-Feb-2003 04:05:06.009",
    );
  }

  #[test]
  fn empty_numeric_sections_hide_only_the_selected_values() {
    for (raw, code) in [
      ("0", "m/d/yyyy;;"),
      ("-1", "m/d/yyyy;;"),
      ("12", ";0;0"),
      ("-12", "0;"),
      ("0", "0;0;;@"),
      ("0", "0;0;[Red]"),
      ("0", "[=0];0"),
      ("12", ";;;"),
      ("-12", ";;;"),
      ("0", ";;;"),
    ] {
      let (text, state) = rendered_number_text(raw, Some(code), None, false);
      assert!(text.is_empty(), "{raw}: {code}: {text}");
      assert_eq!(state, NumberFormatRenderState::Number, "{code}");
    }
    for (raw, code, expected) in [
      ("1", "m/d/yyyy;;", "1/1/1900"),
      ("12", "0;", "12"),
      ("-12", ";0;0", "12"),
      ("0", ";0;0", "0"),
      ("12", "[=0];0", "12"),
      ("12", "General;;", "12"),
      ("0", r#"0;0;";""#, ";"),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(code), None, false).0,
        expected,
        "{code}"
      );
    }
    assert_eq!(
      rendered_number_text(
        "authored text",
        Some("0;0;;@"),
        Some(x::CellValues::SharedString),
        false
      )
      .0,
      "authored text",
    );
    assert_eq!(
      rendered_number_text("#N/A", Some(";;;"), Some(x::CellValues::Error), false).0,
      "#N/A",
    );
    assert_eq!(
      rendered_number_text("12", Some(""), None, false).1,
      NumberFormatRenderState::UnsupportedFormatCode
    );
  }

  #[test]
  fn numeric_format_sections_preserve_literal_semicolons() {
    for (raw, code, expected) in [
      ("12", r#"0";kg";-0";kg";0"#, "12;kg"),
      ("-12", r#"0";kg";-0";kg";0"#, "-12;kg"),
      ("12", r"0\;0", "1;2"),
    ] {
      assert_eq!(
        rendered_number_text(raw, Some(code), None, false).0,
        expected,
        "{code}"
      );
    }
  }

  #[test]
  fn quoted_fraction_digits_do_not_change_numeric_rounding() {
    for code in [r#"0.0"0""#, r"0.0\0"] {
      assert_eq!(
        rendered_number_text("1.26", Some(code), None, false).0,
        "1.30",
        "{code}"
      );
    }
  }

  #[test]
  fn quoted_number_format_markers_remain_visible_text() {
    for (raw, code, expected) in [
      ("12", r#""[Red]"0"#, "[Red]12"),
      ("12", r#""[>=10]"0"#, "[>=10]12"),
      ("12", r#""[$USD-409]"0"#, "[$USD-409]12"),
      ("12", r"\[0\]", "[12]"),
      ("12", r#"[Red]"[Blue]"0"#, "[Blue]12"),
      ("12", r#""[Red]";"[Blue]";"[Green]""#, "[Red]"),
      ("-12", r#""[Red]";"[Blue]";"[Green]""#, "[Blue]"),
      ("0", r#""[Red]";"[Blue]";"[Green]""#, "[Green]"),
      ("12", "[Red]0", "12"),
      ("12", "[$$-409]0", "$12"),
      ("12", "[$USD-409]0.00", "USD12.00"),
      ("12", "0.00[$USD-409]", "12.00USD"),
      ("12", "[$S/.-280A]0.00", "S/.12.00"),
    ] {
      let (text, state) = rendered_number_text(raw, Some(code), None, false);
      assert_eq!(text, expected, "{raw}: {code}");
      assert_eq!(state, NumberFormatRenderState::Number, "{code}");
    }
    // These brackets are actual elapsed-time/native-number instructions,
    // not color markers to discard or literals to reinterpret.
    assert_eq!(strip_number_format_markers("[Red][h]:mm"), "[h]:mm");
    assert_eq!(
      strip_number_format_markers("[DBNum1][$-804]General"),
      "[DBNum1]General"
    );
    assert_eq!(strip_number_format_markers(r"_[0"), r"_[0");
    assert_eq!(strip_number_format_markers(r"*[0"), r"*[0");
  }

  #[test]
  fn general_number_format_removes_binary_double_noise() {
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
  fn general_number_format_matches_office_wide_column_controls() {
    for (raw, expected) in [
      ("0.7034962745892106", "0.703496275"),
      ("3.6512482151539434", "3.651248215"),
      ("0.123456789012345", "0.123456789"),
      ("-0.123456789012345", "-0.123456789"),
      ("1.23456789012345", "1.23456789"),
      ("1234.5678912345", "1234.567891"),
      ("12345.6789012345", "12345.6789"),
      ("-12345.6789012345", "-12345.6789"),
      ("123456789012.345", "1.23457E+11"),
      ("99999999999", "99999999999"),
      ("100000000000", "1E+11"),
      ("99999999999.9", "1E+11"),
      ("0.9999999995", "1"),
      ("0.0001", "0.0001"),
      ("0.00001", "0.00001"),
      ("0.000000001", "0.000000001"),
      ("0.0000123456789", "1.23457E-05"),
      ("0.00000123456789", "1.23457E-06"),
      ("0.000000123456789", "1.23457E-07"),
    ] {
      assert_eq!(
        rendered_number_text(raw, None, None, false).0,
        expected,
        "{raw}"
      );
    }
  }

  #[test]
  fn error_cell_preserves_its_type_for_general_alignment() {
    let (text, state) = rendered_number_text(
      "#N/A",
      None,
      Some(
        ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues::Error,
      ),
      false,
    );

    assert_eq!(text, "#N/A");
    assert_eq!(state, NumberFormatRenderState::Error);
  }

  #[test]
  fn boolean_cell_ignores_custom_number_format_literals() {
    let boolean =
      ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::CellValues::Boolean;

    assert_eq!(
      rendered_number_text(
        "1",
        Some("\"IGAZ\";\"IGAZ\";\"HAMIS\""),
        Some(boolean),
        false
      )
      .0,
      "TRUE"
    );
    assert_eq!(
      rendered_number_text(
        "0",
        Some("\"IGAZ\";\"IGAZ\";\"HAMIS\""),
        Some(boolean),
        false
      )
      .0,
      "FALSE"
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
  fn japanese_era_number_formats_use_the_serial_date_without_fixed_offsets() {
    for date_1904 in [false, true] {
      let serial = if date_1904 { "43978" } else { "45440" };
      assert_eq!(
        rendered_number_text_for_locale(
          serial,
          Some(r#"[$-ja-JP-x-gannen]ggge"年"m"月"d"日";@"#),
          None,
          date_1904,
          Some("zh-CN"),
        )
        .0,
        "令和6年5月28日",
      );
    }
    assert_eq!(
      rendered_number_text("45440", Some("[$-ja-JP]ge"), None, false).0,
      "R6",
    );
    assert_eq!(
      rendered_number_text("123000", Some("0.00E+00"), None, false).0,
      "1.23E+05",
    );
  }

  #[test]
  fn embedded_date_locale_overrides_the_host_format_locale() {
    assert_eq!(
      rendered_number_text_for_locale("43961", Some("[$-0409]d-mmm"), None, false, Some("zh-CN"),)
        .0,
      "10-May"
    );
    assert_eq!(
      rendered_number_text_for_locale("43961", Some("d-mmm"), None, false, Some("zh-CN"),).0,
      "10-5月"
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
  fn quoted_spacing_operators_are_literal_in_number_affixes() {
    for (code, expected) in [
      ("\"*_\"0", "*_12"),
      ("0\"*_\"", "12*_"),
      ("0.0\"*_\"", "12.0*_"),
    ] {
      assert_eq!(
        rendered_number_text("12", Some(code), None, false).0,
        expected,
        "{code}"
      );
    }
  }

  #[test]
  fn accounting_layout_preserves_fill_and_nonprinting_digit_widths() {
    use NumberFormatLayoutItem::{Fill, Reserve, Text};
    let code = "_(\"$\"* #,##0.00_);_(\"$\"* \\(#,##0.00\\);_(\"$\"* \"-\"??_);_(@_)";
    for (raw, body) in [("1", "1.00"), ("-1", "(1.00)")] {
      let mut expected = vec![Reserve('('), Text("$".into()), Fill(' '), Text(body.into())];
      if raw == "1" {
        expected.push(Reserve(')'));
      }
      assert_eq!(
        number_format_layout(raw, code, NumberFormatRenderState::Number),
        Some(expected)
      );
    }
    assert_eq!(
      number_format_layout("0", code, NumberFormatRenderState::Number),
      Some(vec![
        Reserve('('),
        Text("$".into()),
        Fill(' '),
        Text("-".into()),
        Reserve('0'),
        Reserve('0'),
        Reserve(')'),
      ])
    );
    assert_eq!(
      number_format_layout("Total", code, NumberFormatRenderState::Text),
      Some(vec![Reserve('('), Text("Total".into()), Reserve(')'),])
    );
  }

  #[test]
  fn numeric_layout_reserves_internal_and_optional_placeholder_widths() {
    use NumberFormatLayoutItem::{Reserve, Text};
    for (raw, code, expected) in [
      (
        "12",
        "0_W0",
        vec![Text("1".into()), Reserve('W'), Text("2".into())],
      ),
      (
        "12",
        "_W0_i",
        vec![Reserve('W'), Text("12".into()), Reserve('i')],
      ),
      (
        "1.2",
        "???0.00",
        vec![
          Reserve('0'),
          Reserve('0'),
          Reserve('0'),
          Text("1.20".into()),
        ],
      ),
      ("1.2", "0.??", vec![Text("1.2".into()), Reserve('0')]),
      ("1250", "0.0,,_)", vec![Text("0.0".into()), Reserve(')')]),
    ] {
      assert_eq!(
        number_format_layout(raw, code, NumberFormatRenderState::Number),
        Some(expected),
        "{code}"
      );
    }
    assert!(number_format_layout("12", "\"*_\"0", NumberFormatRenderState::Number).is_none());
    assert!(number_format_layout("0.5", "# ?/?", NumberFormatRenderState::Number).is_none());
  }

  #[test]
  fn accounting_format_controls_are_not_visible_text() {
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
  fn trailing_commas_scale_numbers_by_thousands() {
    // ECMA-376 Part 1's number-format examples define one trailing comma as
    // division by 1,000 and two as division by 1,000,000.
    assert_eq!(
      rendered_number_text("12000", Some("#,"), None, false).0,
      "12"
    );
    assert_eq!(
      rendered_number_text("12200000", Some("0.0,,"), None, false).0,
      "12.2"
    );
    assert_eq!(
      rendered_number_text("25396277490", Some("#,##0,,"), None, false).0,
      "25,396"
    );
  }

  #[test]
  fn icon_set_threshold_respects_exclusive_gte_and_reverse() {
    let exclusive_top = [(0.0, true), (0.0, true), (3.0, false)];
    assert_eq!(icon_set_selected_index(3.0, &exclusive_top, false), Some(1));
    assert_eq!(icon_set_selected_index(3.1, &exclusive_top, false), Some(2));
    assert_eq!(icon_set_selected_index(3.0, &exclusive_top, true), Some(1));

    let inclusive_top = [(0.0, true), (0.0, true), (3.0, true)];
    assert_eq!(icon_set_selected_index(3.0, &inclusive_top, false), Some(2));
    assert_eq!(icon_set_selected_index(-1.0, &inclusive_top, false), None);
  }

  #[test]
  fn color_scale_interpolation_clamps_and_truncates_each_rgb_channel() {
    let red = RgbColor { r: 255, g: 0, b: 0 };
    let blue = RgbColor { r: 0, g: 0, b: 255 };

    assert_eq!(interpolate_color_scale(-1.0, 0.0, red, 2.0, blue), red);
    assert_eq!(
      interpolate_color_scale(1.0, 0.0, red, 2.0, blue),
      RgbColor {
        r: 128,
        g: 0,
        b: 127,
      }
    );
    assert_eq!(interpolate_color_scale(3.0, 0.0, red, 2.0, blue), blue);
  }
}
