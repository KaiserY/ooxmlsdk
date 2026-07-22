use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2008_diagram as dsp;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::common;
use crate::model::{
  BorderStyle, ImageCrop, ImageItem, LineItem, LineItemKind, LinkAreaItem, PageItem, PageSetup,
  PdfTextSegmentation, RectItem, RgbColor, TextItem, TextStyle, common_page_setup, common_point,
  common_rect, common_rgb, common_stroke_from_border, common_text_style,
};
use crate::options::LayoutOptions;
use crate::render::{chart as shared_chart, diagram as shared_diagram, emf_wmf};
use crate::text_metrics::TextMetrics;
use crate::units;

use super::import::ExcelImport;
use super::print::{CalcPrintDocument, CalcPrintPage};
use super::worksheet::{CalcSheet, CellAddress, CellRange, CellRect};
use crate::pptx::chart::{
  ChartFrame, ChartLayoutProfile, ClusteredColumnStyle, lower_clustered_column_chart,
};

const XLSX_HEADER_FOOTER_LINE_HEIGHT_PT: f32 = 12.0;
// margins are 20 twips on each side.
const XLSX_CELL_TEXT_INSET_PT: f32 = 20.0 / crate::units::TWIPS_PER_POINT;
const XLSX_GRID_LINE_WIDTH_PT: f32 = 0.25;

#[derive(Clone, Copy, Debug)]
struct CalcCellOutputArea {
  align_rect: CellRect,
  clip_rect: CellRect,
  left_clip_pt: f32,
  right_clip_pt: f32,
}

pub(crate) fn lower_to_layout_document(
  import: &ExcelImport,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  let mut pages = Vec::new();
  let print_document = CalcPrintDocument::from_import(import);
  pages.extend(print_document.pages.iter().map(|page| {
    let setup = page_setup_from_calc(page);
    (setup, print_page_items(import, page, setup))
  }));
  common_fixed_pages_with_items(pages, options)
}

fn common_fixed_pages_with_items(
  pages: Vec<(PageSetup, Vec<PageItem>)>,
  options: &LayoutOptions,
) -> common::LayoutDocument<'static> {
  let pages = if pages.is_empty() {
    vec![(PageSetup::default(), Vec::new())]
  } else {
    pages
  };
  common::LayoutDocument {
    engine_kind: common::LayoutEngineKind::Xlsx,
    options: common::LayoutOptions {
      collect_debug: options.diagnostics.collect_debug_records,
      approximate_unsupported: false,
      preserve_source_links: options.diagnostics.preserve_source_links,
    },
    pages: pages
      .into_iter()
      .map(|(setup, items)| common_display_page(setup, items))
      .collect(),
    ..Default::default()
  }
}

fn common_display_page(setup: PageSetup, items: Vec<PageItem>) -> common::DisplayPage<'static> {
  let common_setup = common_page_setup(setup);
  common::DisplayPage {
    section_index: 0,
    section_page_index: 0,
    bounds: common_rect(0.0, 0.0, setup.width_pt, setup.height_pt),
    background: setup
      .background
      .map(|color| common::Fill::Solid(common_rgb(color, 1.0))),
    setup: common_setup,
    items: items.into_iter().map(common_display_item).collect(),
    ..Default::default()
  }
}

fn common_display_item(item: PageItem) -> common::DisplayItem<'static> {
  match item {
    PageItem::Text(item) => common::DisplayItem::Text(common_text_run(item)),
    PageItem::Image(item) => common::DisplayItem::Image(common_image_item(item)),
    PageItem::LinkArea(item) => common::DisplayItem::LinkArea(common::LinkArea {
      bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
      target: Cow::Owned(item.hyperlink_url),
    }),
    PageItem::Path(item) => common::DisplayItem::Path(item),
    PageItem::Rect(item) => common::DisplayItem::Rect(common_rect_item(item)),
    PageItem::Line(item) => common::DisplayItem::Line(common_line_item(item)),
  }
}

fn common_text_run(item: TextItem) -> common::TextRun<'static> {
  let color = common_rgb(item.style.color, item.style.opacity);
  common::TextRun {
    text: Cow::Owned(item.text),
    origin: common_point(item.x_pt, item.y_pt),
    line_height: common::Pt(item.line_height_pt),
    style: common_text_style(item.style),
    font_id: None,
    color,
    rotation_center: item.rotation_center_pt.map(|(x, y)| common_point(x, y)),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    dynamic_field: None,
    form_widget_id: item.form_widget_id,
    paragraph_bidi: item.paragraph_bidi,
    word_spacing_pt: 0.0,
    preserve_text_portion: item.preserve_text_portion,
    pdf_text_segmentation: match item.pdf_text_segmentation {
      PdfTextSegmentation::Line => common::PdfTextSegmentation::Line,
      PdfTextSegmentation::Portion => common::PdfTextSegmentation::Portion,
    },
    source: None,
  }
}

fn common_image_item(item: ImageItem) -> common::ImageItem<'static> {
  common::ImageItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    crop: Some(common::ImageCrop {
      left: item.crop.left,
      top: item.crop.top,
      right: item.crop.right,
      bottom: item.crop.bottom,
    }),
    clip_path: item.clip_path,
    rotation_degrees: item.rotation_deg,
    flip_horizontal: item.flip_horizontal,
    flip_vertical: item.flip_vertical,
    content_type: item
      .content_type
      .map(Cow::Owned)
      .unwrap_or(Cow::Borrowed("application/octet-stream")),
    bytes: item.data,
    relationship_id: None,
    alt_text: item.alt_text.map(Cow::Owned),
    hyperlink_url: item.hyperlink_url.map(Cow::Owned),
    floating: item.floating,
    behind_text: item.behind_text,
  }
}

fn common_rect_item(item: RectItem) -> common::RectItem<'static> {
  common::RectItem {
    bounds: common_rect(item.x_pt, item.y_pt, item.width_pt, item.height_pt),
    fill: item
      .fill_color
      .map(|color| common::Fill::Solid(common_rgb(color, item.fill_opacity)))
      .unwrap_or(common::Fill::None),
    stroke: item
      .stroke
      .map(|stroke| common_stroke_from_border(stroke, item.stroke_opacity)),
  }
}

fn common_line_item(item: LineItem) -> common::LineItem<'static> {
  common::LineItem {
    start: common_point(item.x1_pt, item.y1_pt),
    end: common_point(item.x2_pt, item.y2_pt),
    stroke: common::Stroke {
      width: common::Pt(item.width_pt),
      color: common_rgb(item.color, 1.0),
      dash: None,
      source_style_id: None,
    },
    kind: match item.kind {
      LineItemKind::Stroke => common::LineKind::Stroke,
    },
  }
}

fn print_page_items(
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let body_origin_y = setup.margin_top_pt;
  let zoom_scale = page.zoom as f32 / 100.0;
  let heading_width = if page.page_settings.print_headings {
    page.sheet.column_width_pt(1) * zoom_scale
  } else {
    0.0
  };
  let heading_height = if page.page_settings.print_headings {
    page.sheet.row_height_pt(1) * zoom_scale
  } else {
    0.0
  };
  let body_origin_x = setup.margin_left_pt + heading_width;
  let body_origin_y = body_origin_y + heading_height;
  let mut text_metrics = TextMetrics::new();
  let repeat_width = page
    .repeated_columns
    .map(|range| page.sheet.range_rect(range).width_pt * zoom_scale)
    .unwrap_or(0.0);
  let repeat_height = page
    .repeated_rows
    .map(|range| page.sheet.range_rect(range).height_pt * zoom_scale)
    .unwrap_or(0.0);

  // ECMA-376 §18.3.1.46 defines these as the printed page header and
  // footer. Keep the PDF content stream in the same semantic order exposed
  // by Microsoft Office fixed output: header, sheet body, then footer.
  render_header_or_footer(
    &mut items,
    page,
    setup,
    true,
    &import.styles,
    &mut text_metrics,
  );

  if let Some(area) = repeat_corner_for_page(page) {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.repeated_corner_cells,
      area,
      CellAreaRenderLayout {
        origin_x_pt: body_origin_x,
        origin_y_pt: body_origin_y,
        zoom_scale,
      },
      &mut text_metrics,
    );
  }
  if let Some(area) = repeat_rows_for_page(page) {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.repeated_row_cells,
      area,
      CellAreaRenderLayout {
        origin_x_pt: body_origin_x + repeat_width,
        origin_y_pt: body_origin_y,
        zoom_scale,
      },
      &mut text_metrics,
    );
  }
  if let Some(area) = repeat_columns_for_page(page) {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.repeated_column_cells,
      area,
      CellAreaRenderLayout {
        origin_x_pt: body_origin_x,
        origin_y_pt: body_origin_y + repeat_height,
        zoom_scale,
      },
      &mut text_metrics,
    );
  }
  if let Some(area) = page.area {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.cells,
      area,
      CellAreaRenderLayout {
        origin_x_pt: body_origin_x + repeat_width,
        origin_y_pt: body_origin_y + repeat_height,
        zoom_scale,
      },
      &mut text_metrics,
    );
    if page.page_settings.print_headings {
      render_headings(
        &mut items,
        page,
        area,
        HeadingRenderLayout {
          row_header_x_pt: setup.margin_left_pt,
          row_header_y_pt: body_origin_y + repeat_height,
          col_header_x_pt: body_origin_x + repeat_width,
          col_header_y_pt: body_origin_y - heading_height,
          zoom_scale,
        },
      );
    }
  }

  items.extend(print_page_image_items(
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_shape_items(
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_diagram_items(
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_drawing_text_items(
    import,
    page,
    setup,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  items.extend(print_page_vml_text_items(
    page,
    body_origin_x + repeat_width,
    body_origin_y + repeat_height,
    zoom_scale,
  ));
  render_header_or_footer(
    &mut items,
    page,
    setup,
    false,
    &import.styles,
    &mut text_metrics,
  );
  items
}

fn page_setup_from_calc(page: &CalcPrintPage<'_>) -> PageSetup {
  let mut setup = PageSetup::default();
  let (width_pt, height_pt) = page.page_settings.page_size_pt();
  setup.width_pt = width_pt;
  setup.height_pt = height_pt;
  setup.margin_left_pt = page.page_settings.margin_left_in as f32 * units::POINTS_PER_INCH;
  setup.margin_right_pt = page.page_settings.margin_right_in as f32 * units::POINTS_PER_INCH;
  setup.margin_top_pt = page.page_settings.margin_top_in as f32 * units::POINTS_PER_INCH;
  setup.margin_bottom_pt = page.page_settings.margin_bottom_in as f32 * units::POINTS_PER_INCH;
  setup.header_distance_pt = page.page_settings.margin_header_in as f32 * units::POINTS_PER_INCH;
  setup.footer_distance_pt = page.page_settings.margin_footer_in as f32 * units::POINTS_PER_INCH;
  setup
}

#[derive(Clone, Copy, Debug)]
struct CellAreaRenderLayout {
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
}

fn render_cell_area(
  items: &mut Vec<PageItem>,
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  cells: &[super::print::CalcPrintCell<'_>],
  area: super::worksheet::CellRange,
  layout: CellAreaRenderLayout,
  text_metrics: &mut TextMetrics,
) {
  let area_rect = page.sheet.range_rect(area);
  let occupied_cells = calc_occupied_text_cells(cells);
  for cell in cells {
    if page.sheet.is_covered_merged_cell(cell.address) {
      continue;
    }
    let rect = page.sheet.cell_rect(cell.address);
    if rect.width_pt <= 0.0 || rect.height_pt <= 0.0 {
      continue;
    }
    let x_pt = layout.origin_x_pt + (rect.x_pt - area_rect.x_pt) * layout.zoom_scale;
    let y_pt = layout.origin_y_pt + (rect.y_pt - area_rect.y_pt) * layout.zoom_scale;
    let width_pt = rect.width_pt * layout.zoom_scale;
    let height_pt = rect.height_pt * layout.zoom_scale;
    let table_builtin_style = super::table::builtin_table_style_for_address(
      &page.sheet.resources.tables,
      &import.styles,
      cell.address,
    );
    if let Some(fill_color) = conditional_fill_color(import, page.sheet, cell)
      .or(table_builtin_style.fill)
      .or_else(|| pivot_format_fill_color(import, cell))
      .or_else(|| import.styles.fill_color_for_cell(cell.style_index))
    {
      items.push(PageItem::Rect(RectItem {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        fill_color: Some(fill_color),
        fill_opacity: 1.0,
        stroke: None,
        stroke_opacity: 1.0,
      }));
    }
    let mut borders = import.styles.borders_for_cell(cell.style_index);
    merge_cell_borders(&mut borders, table_builtin_style.borders);
    let pivot_builtin_style =
      super::pivot::pivot_builtin_style_for_address(page.sheet, cell.address);
    merge_cell_borders(&mut borders, pivot_builtin_style.borders);
    if let Some(format_id) = cell.pivot_format_id
      && let Some(pivot_borders) = import.styles.differential_borders(format_id)
    {
      merge_cell_borders(&mut borders, pivot_borders);
    }
    render_cell_borders(
      items,
      CellRect {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
      },
      borders,
    );
    if cell.rendered_text.is_empty() {
      continue;
    }
    let hyperlink_url = hyperlink_for_cell(page, cell.address);
    let cell_rect = CellRect {
      x_pt,
      y_pt,
      width_pt,
      height_pt,
    };
    let mut measurement_style = import.styles.text_style_for_cell(cell.style_index);
    super::table::apply_builtin_table_text_style(table_builtin_style, &mut measurement_style);
    apply_conditional_text_style(import, page.sheet, cell, &mut measurement_style);
    // sc/source/ui/view/output2.cxx ScDrawStringsVars::SetPattern(). Calc's
    // print map mode scales cell geometry and the font used for measurement.
    let pivot_builtin_style =
      super::pivot::pivot_builtin_style_for_address(page.sheet, cell.address);
    if pivot_builtin_style.bold {
      measurement_style.bold = true;
    }
    if let Some(format_id) = cell.pivot_format_id {
      import
        .styles
        .apply_differential_text_style(format_id, &mut measurement_style);
    }
    measurement_style.font_size_pt *= layout.zoom_scale;
    let render_style = measurement_style.clone();
    let mut alignment = import.styles.alignment_for_cell(cell.style_index);
    if pivot_builtin_style.left_align {
      let mut pivot_alignment = alignment.unwrap_or_default();
      pivot_alignment.horizontal = Some(x::HorizontalAlignmentValues::Left);
      alignment = Some(pivot_alignment);
    }
    if let Some(format_id) = cell.pivot_format_id
      && let Some(pivot_alignment) = import.styles.differential_alignment(format_id)
    {
      alignment = Some(pivot_alignment);
    }
    let output_area = calc_cell_output_area(
      CalcCellOutputContext {
        sheet: page.sheet,
        occupied_cells: &occupied_cells,
        text_metrics,
      },
      cell,
      cell_rect,
      &measurement_style,
      alignment,
      layout.zoom_scale,
    );
    let rendered_text = calc_cell_visible_text(
      page.sheet,
      cell,
      &measurement_style,
      output_area,
      text_metrics,
    );
    let horizontal_alignment = calc_cell_horizontal_alignment(cell, alignment);
    let mut rendered_text_items = Vec::new();
    if !cell.rich_text_runs.is_empty() && rendered_text.as_ref() == cell.text.as_ref() {
      render_cell_rich_text(
        &mut rendered_text_items,
        cell.rich_text_runs,
        output_area.align_rect,
        render_style,
        horizontal_alignment,
        hyperlink_url.clone(),
        text_metrics,
      );
    } else {
      render_cell_text(
        &mut rendered_text_items,
        rendered_text.as_ref(),
        output_area.align_rect,
        render_style,
        CellTextRenderOptions {
          alignment,
          horizontal_alignment,
          hyperlink_url: hyperlink_url.clone(),
          formula: cell.formula,
        },
        text_metrics,
      );
    }
    items.extend(
      rendered_text_items
        .into_iter()
        .filter(|item| matches!(item, PageItem::Text(_))),
    );
    if let Some(hyperlink_url) = hyperlink_url {
      items.push(PageItem::LinkArea(LinkAreaItem {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
        hyperlink_url,
      }));
    }
  }
  if page.page_settings.print_grid_lines {
    render_grid(
      items,
      page,
      area,
      layout.origin_x_pt,
      layout.origin_y_pt,
      layout.zoom_scale,
    );
  }
}

fn calc_occupied_text_cells(
  cells: &[super::print::CalcPrintCell<'_>],
) -> HashMap<(u32, u32), bool> {
  cells
    .iter()
    .filter(|cell| !cell.rendered_text.is_empty())
    .map(|cell| ((cell.address.row, cell.address.col), true))
    .collect()
}

struct CalcCellOutputContext<'a> {
  sheet: &'a CalcSheet,
  occupied_cells: &'a HashMap<(u32, u32), bool>,
  text_metrics: &'a mut TextMetrics,
}

fn calc_cell_output_area(
  context: CalcCellOutputContext<'_>,
  cell: &super::print::CalcPrintCell<'_>,
  rect: CellRect,
  style: &TextStyle,
  alignment: Option<super::styles::AlignmentRecord>,
  zoom_scale: f32,
) -> CalcCellOutputArea {
  let text_width_pt = context
    .text_metrics
    .measure_text(&cell.rendered_text, style);
  let needed_width_pt = text_width_pt + XLSX_CELL_TEXT_INSET_PT * 2.0;
  let mut output = CalcCellOutputArea {
    align_rect: rect,
    clip_rect: rect,
    left_clip_pt: 0.0,
    right_clip_pt: 0.0,
  };
  if needed_width_pt <= rect.width_pt {
    return output;
  }

  let missing_width_pt = needed_width_pt - rect.width_pt;
  let (mut left_missing_pt, mut right_missing_pt) =
    calc_cell_missing_width_by_alignment(missing_width_pt, cell, alignment);

  if !calc_cell_is_value(cell) && !alignment.is_some_and(|alignment| alignment.wrap_text) {
    let mut right_col = cell.address.col;
    while right_missing_pt > 0.0
      && output_column_available(
        context.sheet,
        context.occupied_cells,
        right_col + 1,
        cell.address.row,
      )
    {
      right_col += 1;
      let column_width_pt = context.sheet.column_width_pt(right_col) * zoom_scale;
      if column_width_pt <= f32::EPSILON {
        break;
      }
      output.clip_rect.width_pt += column_width_pt;
      right_missing_pt -= column_width_pt;
    }
    let mut left_col = cell.address.col;
    while left_missing_pt > 0.0
      && left_col > 1
      && output_column_available(
        context.sheet,
        context.occupied_cells,
        left_col - 1,
        cell.address.row,
      )
    {
      left_col -= 1;
      let column_width_pt = context.sheet.column_width_pt(left_col) * zoom_scale;
      if column_width_pt <= f32::EPSILON {
        break;
      }
      output.clip_rect.x_pt -= column_width_pt;
      output.clip_rect.width_pt += column_width_pt;
      left_missing_pt -= column_width_pt;
    }
  }

  output.left_clip_pt = left_missing_pt.max(0.0);
  output.right_clip_pt = right_missing_pt.max(0.0);
  output
}

fn calc_cell_missing_width_by_alignment(
  missing_width_pt: f32,
  cell: &super::print::CalcPrintCell<'_>,
  alignment: Option<super::styles::AlignmentRecord>,
) -> (f32, f32) {
  match calc_cell_horizontal_alignment(cell, alignment) {
    x::HorizontalAlignmentValues::Right => (missing_width_pt, 0.0),
    x::HorizontalAlignmentValues::Center | x::HorizontalAlignmentValues::CenterContinuous => {
      let left = missing_width_pt / 2.0;
      (left, missing_width_pt - left)
    }
    _ => (0.0, missing_width_pt),
  }
}

fn calc_cell_horizontal_alignment(
  cell: &super::print::CalcPrintCell<'_>,
  alignment: Option<super::styles::AlignmentRecord>,
) -> x::HorizontalAlignmentValues {
  match alignment.and_then(|alignment| alignment.horizontal) {
    Some(x::HorizontalAlignmentValues::General) | None => {
      match cell.number_format_state {
        // ECMA-376 Part 1, General Format / Alignment: Boolean and error
        // values are centered; numbers are right-aligned and strings left.
        super::print::NumberFormatRenderState::Boolean
        | super::print::NumberFormatRenderState::Error => x::HorizontalAlignmentValues::Center,
        _ if calc_cell_is_value(cell) => x::HorizontalAlignmentValues::Right,
        _ => x::HorizontalAlignmentValues::Left,
      }
    }
    Some(value) => value,
  }
}

fn output_column_available(
  sheet: &CalcSheet,
  occupied_cells: &HashMap<(u32, u32), bool>,
  column: u32,
  row: u32,
) -> bool {
  let address = CellAddress { col: column, row };
  !occupied_cells.contains_key(&(row, column))
    && !sheet.is_covered_merged_cell(address)
    && sheet.column_width_pt(column) > f32::EPSILON
}

fn calc_cell_visible_text<'a>(
  sheet: &CalcSheet,
  cell: &'a super::print::CalcPrintCell<'_>,
  style: &TextStyle,
  output_area: CalcCellOutputArea,
  text_metrics: &mut TextMetrics,
) -> std::borrow::Cow<'a, str> {
  if output_area.left_clip_pt <= f32::EPSILON && output_area.right_clip_pt <= f32::EPSILON {
    return std::borrow::Cow::Borrowed(&cell.rendered_text);
  }
  if calc_cell_is_value(cell) {
    if cell.number_format_state == super::print::NumberFormatRenderState::General
      && let Some(text) =
        calc_fit_general_number_text(cell, style, output_area.align_rect.width_pt, text_metrics)
    {
      return std::borrow::Cow::Owned(text);
    }
    return if calc_cell_value_can_hash(cell) {
      std::borrow::Cow::Owned(calc_cell_overflow_hash_text(
        style,
        output_area.align_rect.width_pt,
        text_metrics,
      ))
    } else {
      std::borrow::Cow::Borrowed(&cell.rendered_text)
    };
  }
  // strings with a clip region from GetOutputArea/Clip. krilla text extraction
  // exposes full glyph payloads even when clipped, so trim the extracted text
  // to the visible prefix/suffix while keeping the same clip decision.
  if super::pivot::pivot_table_contains_address(sheet, cell.address) {
    clipped_string_text(cell, style, output_area, text_metrics)
      .map(std::borrow::Cow::Owned)
      .unwrap_or_else(|| std::borrow::Cow::Borrowed(cell.rendered_text.as_str()))
  } else {
    std::borrow::Cow::Borrowed(cell.rendered_text.as_str())
  }
}

fn calc_cell_overflow_hash_text(
  style: &TextStyle,
  cell_width_pt: f32,
  text_metrics: &mut TextMetrics,
) -> String {
  let hash_width_pt = text_metrics.measure_text("#", style);
  let available_width_pt = (cell_width_pt - XLSX_CELL_TEXT_INSET_PT * 2.0).max(0.0);
  let count = calc_cell_overflow_hash_count(available_width_pt, hash_width_pt);
  "#".repeat(count)
}

fn calc_cell_overflow_hash_count(available_width_pt: f32, hash_width_pt: f32) -> usize {
  if !available_width_pt.is_finite()
    || !hash_width_pt.is_finite()
    || available_width_pt <= f32::EPSILON
    || hash_width_pt <= f32::EPSILON
  {
    return 1;
  }
  (available_width_pt / hash_width_pt).floor().max(1.0) as usize
}

fn calc_cell_is_value(cell: &super::print::CalcPrintCell<'_>) -> bool {
  matches!(
    cell.number_format_state,
    super::print::NumberFormatRenderState::Raw
      | super::print::NumberFormatRenderState::General
      | super::print::NumberFormatRenderState::Number
      | super::print::NumberFormatRenderState::Percent
      | super::print::NumberFormatRenderState::DateTime
  ) && cell.text.as_ref().parse::<f64>().is_ok()
}

fn calc_cell_value_can_hash(cell: &super::print::CalcPrintCell<'_>) -> bool {
  matches!(
    cell.number_format_state,
    super::print::NumberFormatRenderState::Raw
      | super::print::NumberFormatRenderState::General
      | super::print::NumberFormatRenderState::Number
      | super::print::NumberFormatRenderState::Percent
      | super::print::NumberFormatRenderState::DateTime
      | super::print::NumberFormatRenderState::UnsupportedFormatCode
  )
}

fn clipped_string_text(
  cell: &super::print::CalcPrintCell<'_>,
  style: &TextStyle,
  output_area: CalcCellOutputArea,
  text_metrics: &mut TextMetrics,
) -> Option<String> {
  let text = cell.rendered_text.as_str();
  if text.is_empty() {
    return None;
  }
  let text_width = text_metrics.measure_text(text, style);
  let visible_width = (text_width - output_area.right_clip_pt - output_area.left_clip_pt).max(0.0);
  if visible_width <= f32::EPSILON || visible_width >= text_width {
    return None;
  }
  if output_area.left_clip_pt > output_area.right_clip_pt {
    let mut start = text.len();
    for (index, _) in text.char_indices().rev() {
      if text_metrics.measure_text(&text[index..], style) <= visible_width {
        start = index;
      } else {
        break;
      }
    }
    return (start > 0 && start < text.len()).then(|| text[start..].to_string());
  }
  let mut end = 0usize;
  for (index, ch) in text.char_indices() {
    let next = index + ch.len_utf8();
    if text_metrics.measure_text(&text[..next], style) <= visible_width {
      end = next;
    } else {
      break;
    }
  }
  (end > 0 && end < text.len()).then(|| text[..end].to_string())
}

fn calc_fit_general_number_text(
  cell: &super::print::CalcPrintCell<'_>,
  style: &TextStyle,
  column_width_pt: f32,
  text_metrics: &mut TextMetrics,
) -> Option<String> {
  let value = cell.text.as_ref().parse::<f64>().ok()?;
  if !value.is_finite() {
    return None;
  }
  let available_width = column_width_pt - XLSX_CELL_TEXT_INSET_PT * 2.0;
  if available_width <= f32::EPSILON {
    return None;
  }
  for significant_digits in (1..=15).rev() {
    let text = format_general_number_with_significant_digits(value, significant_digits);
    if text_metrics.measure_text(&text, style) <= available_width {
      return Some(text);
    }
  }
  None
}

fn format_general_number_with_significant_digits(value: f64, significant_digits: usize) -> String {
  if value == 0.0 {
    return "0".to_string();
  }
  let abs = value.abs();
  let integer_digits = if abs >= 1.0 {
    abs.log10().floor() as isize + 1
  } else {
    0
  };
  let decimals = if integer_digits >= significant_digits as isize {
    0
  } else {
    significant_digits.saturating_sub(integer_digits.max(0) as usize)
  };
  let mut text = format!("{value:.decimals$}");
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

fn calc_text_can_shape_as_line(text: &str) -> bool {
  text.chars().all(|ch| {
    ch.is_ascii_alphanumeric()
      || ch.is_ascii_whitespace()
      || matches!(ch, '/' | '-' | '+' | '.' | ',' | ':' | ';' | '(' | ')')
      || !ch.is_ascii()
  })
}

fn render_cell_rich_text(
  items: &mut Vec<PageItem>,
  runs: &[super::workbook::SharedStringRun],
  rect: CellRect,
  base_style: TextStyle,
  horizontal_alignment: x::HorizontalAlignmentValues,
  hyperlink_url: Option<String>,
  text_metrics: &mut TextMetrics,
) {
  let mut text = String::new();
  let mut style = base_style;
  let mut style_initialized = false;
  for run in runs.iter().filter(|run| !run.text.is_empty()) {
    if !style_initialized {
      if let Some(font_size_pt) = run.font_size_pt {
        style.font_size_pt = font_size_pt;
      }
      if let Some(color) = run.color {
        style.color = color;
      }
      style.bold = run.bold;
      style.italic = run.italic;
      style.underline = run.underline;
      style.strikethrough = run.strikethrough;
      style_initialized = true;
    }
    text.push_str(&run.text.replace(['\r', '\n'], ""));
  }
  if text.is_empty() {
    return;
  }
  let y_pt = rect.y_pt + XLSX_CELL_TEXT_INSET_PT;
  let line_height = (style.font_size_pt * 1.15).max(1.0);
  let text_width_pt = text_metrics.measure_text(&text, &style);
  let x_pt = cell_text_x_pt(rect, text_width_pt, horizontal_alignment, 0.0);
  let preserve_text_portion = !text.is_ascii() && !calc_text_can_shape_as_line(&text);
  items.push(PageItem::Text(TextItem {
    x_pt,
    y_pt,
    line_height_pt: line_height,
    text,
    style,
    rotation_center_pt: None,
    hyperlink_url,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion,
    pdf_text_segmentation: if preserve_text_portion {
      PdfTextSegmentation::Portion
    } else {
      PdfTextSegmentation::Line
    },
  }));
}

fn conditional_fill_color(
  import: &ExcelImport,
  sheet: &CalcSheet,
  cell: &super::print::CalcPrintCell<'_>,
) -> Option<RgbColor> {
  let mut rules = sheet
    .metrics
    .conditions
    .conditional_formats
    .iter()
    .filter(|format| conditional_format_contains_cell(format, cell.address))
    .flat_map(|format| format.rules.iter())
    .collect::<Vec<_>>();
  rules.sort_by_key(|rule| rule.priority);
  for rule in rules {
    if !conditional_rule_matches(rule, cell) {
      continue;
    }
    if let Some(color) = rule
      .format_id
      .and_then(|format_id| import.styles.differential_fill_color(format_id))
    {
      return Some(color);
    }
    if rule.stop_if_true {
      break;
    }
  }
  None
}

fn apply_conditional_text_style(
  import: &ExcelImport,
  sheet: &CalcSheet,
  cell: &super::print::CalcPrintCell<'_>,
  style: &mut TextStyle,
) {
  let mut rules = sheet
    .metrics
    .conditions
    .conditional_formats
    .iter()
    .filter(|format| conditional_format_contains_cell(format, cell.address))
    .flat_map(|format| format.rules.iter())
    .collect::<Vec<_>>();
  // sc/source/filter/oox/condformatbuffer.cxx sorts imported rules by
  // priority before applying their differential formats.
  rules.sort_by_key(|rule| rule.priority);
  for rule in rules {
    if !conditional_rule_matches(rule, cell) {
      continue;
    }
    if let Some(format_id) = rule
      .format_id
      .filter(|format_id| import.styles.differential_has_font(*format_id))
    {
      import
        .styles
        .apply_differential_text_style(format_id, style);
      return;
    }
    if rule.stop_if_true {
      break;
    }
  }
}

fn pivot_format_fill_color(
  import: &ExcelImport,
  cell: &super::print::CalcPrintCell<'_>,
) -> Option<RgbColor> {
  cell
    .pivot_format_id
    .and_then(|format_id| import.styles.differential_fill_color(format_id))
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

fn conditional_rule_matches(
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  cell: &super::print::CalcPrintCell<'_>,
) -> bool {
  match rule.rule_type {
    x::ConditionalFormatValues::CellIs => conditional_cell_is_matches(rule, cell),
    x::ConditionalFormatValues::ContainsText => rule.text.as_ref().is_some_and(|needle| {
      cell.rendered_text.contains(needle) || cell.text.as_ref().contains(needle)
    }),
    x::ConditionalFormatValues::NotContainsText => rule.text.as_ref().is_some_and(|needle| {
      !cell.rendered_text.contains(needle) && !cell.text.as_ref().contains(needle)
    }),
    x::ConditionalFormatValues::BeginsWith => rule.text.as_ref().is_some_and(|needle| {
      cell.rendered_text.starts_with(needle) || cell.text.as_ref().starts_with(needle)
    }),
    x::ConditionalFormatValues::EndsWith => rule.text.as_ref().is_some_and(|needle| {
      cell.rendered_text.ends_with(needle) || cell.text.as_ref().ends_with(needle)
    }),
    x::ConditionalFormatValues::ContainsBlanks => {
      cell.text.as_ref().is_empty() && cell.rendered_text.is_empty()
    }
    x::ConditionalFormatValues::NotContainsBlanks => {
      !cell.text.as_ref().is_empty() || !cell.rendered_text.is_empty()
    }
    x::ConditionalFormatValues::Expression => expression_rule_matches(rule),
    _ => false,
  }
}

fn conditional_cell_is_matches(
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  cell: &super::print::CalcPrintCell<'_>,
) -> bool {
  let Some(value) = cell.text.as_ref().parse::<f64>().ok() else {
    return false;
  };
  let first = rule
    .formulas
    .first()
    .and_then(|formula| formula.trim().parse::<f64>().ok());
  let second = rule
    .formulas
    .get(1)
    .and_then(|formula| formula.trim().parse::<f64>().ok());
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

fn expression_rule_matches(rule: &super::sheet_conditions::ConditionalFormatRuleModel) -> bool {
  rule.formulas.first().is_some_and(|formula| {
    matches!(
      formula.trim().to_ascii_uppercase().as_str(),
      "TRUE" | "1" | "=TRUE" | "=1"
    )
  })
}

#[derive(Clone, Debug)]
struct CellTextRenderOptions {
  alignment: Option<super::styles::AlignmentRecord>,
  horizontal_alignment: x::HorizontalAlignmentValues,
  hyperlink_url: Option<String>,
  formula: bool,
}

fn render_cell_text(
  items: &mut Vec<PageItem>,
  text: &str,
  rect: CellRect,
  mut style: TextStyle,
  options: CellTextRenderOptions,
  text_metrics: &mut TextMetrics,
) {
  let line_height = (style.font_size_pt * 1.15).max(1.0);
  let alignment = options.alignment;
  let wrap_text = alignment.is_some_and(|alignment| alignment.wrap_text);
  let fill_text;
  let text = if options.horizontal_alignment == x::HorizontalAlignmentValues::Fill && !wrap_text {
    fill_text = repeat_cell_text_to_fill(text, rect.width_pt, &style, text_metrics);
    fill_text.as_str()
  } else {
    text
  };
  let rendered_text;
  let lines = if text.contains('\n') || text.contains('\r') {
    rendered_text = if wrap_text || options.formula {
      text.lines().collect::<Vec<_>>().join(" ")
    } else {
      text.lines().collect::<String>()
    };
    vec![rendered_text.as_str()]
  } else if wrap_text {
    text.lines().collect::<Vec<_>>()
  } else {
    vec![text.lines().next().unwrap_or(text)]
  };
  let text_height = line_height * lines.len().max(1) as f32;
  let mut y_pt = match alignment.and_then(|alignment| alignment.vertical) {
    Some(x::VerticalAlignmentValues::Center) => {
      rect.y_pt + ((rect.height_pt - text_height) / 2.0).max(0.0)
    }
    Some(x::VerticalAlignmentValues::Bottom) => rect.y_pt + (rect.height_pt - text_height).max(0.0),
    _ => rect.y_pt,
  };
  if let Some(rotation) = alignment.and_then(|alignment| alignment.text_rotation) {
    style.rotation_deg = match rotation {
      1..=90 => rotation as f32,
      91..=180 => 90.0 - rotation as f32,
      255 => 90.0,
      _ => 0.0,
    };
  }
  for line in lines {
    let full_line_width_pt = text_metrics.measure_text(line, &style);
    let preserve_text_portion = !line.is_ascii() && !calc_text_can_shape_as_line(line);
    items.push(PageItem::Text(TextItem {
      x_pt: cell_text_x_pt(rect, full_line_width_pt, options.horizontal_alignment, 0.0),
      y_pt,
      line_height_pt: line_height,
      text: line.to_string(),
      style: style.clone(),
      rotation_center_pt: (style.rotation_deg != 0.0).then_some((
        rect.x_pt + rect.width_pt / 2.0,
        rect.y_pt + rect.height_pt / 2.0,
      )),
      hyperlink_url: options.hyperlink_url.clone(),
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion,
      pdf_text_segmentation: if preserve_text_portion {
        PdfTextSegmentation::Portion
      } else {
        PdfTextSegmentation::Line
      },
    }));
    y_pt += line_height;
  }
}

fn repeat_cell_text_to_fill(
  text: &str,
  width_pt: f32,
  style: &TextStyle,
  text_metrics: &mut TextMetrics,
) -> String {
  if text.is_empty() || width_pt <= f32::EPSILON {
    return text.to_string();
  }
  let text_width_pt = text_metrics.measure_text(text, style);
  if text_width_pt <= f32::EPSILON {
    return text.to_string();
  }
  let repeat_count = (width_pt / text_width_pt).ceil().max(1.0) as usize;
  text.repeat(repeat_count)
}

fn cell_text_x_pt(
  rect: CellRect,
  text_width_pt: f32,
  horizontal_alignment: x::HorizontalAlignmentValues,
  leading_offset_pt: f32,
) -> f32 {
  let available_width_pt = (rect.width_pt - XLSX_CELL_TEXT_INSET_PT * 2.0).max(0.0);
  let text_start_pt = match horizontal_alignment {
    x::HorizontalAlignmentValues::Right => {
      rect.x_pt + XLSX_CELL_TEXT_INSET_PT + available_width_pt - text_width_pt
    }
    x::HorizontalAlignmentValues::Center | x::HorizontalAlignmentValues::CenterContinuous => {
      rect.x_pt + XLSX_CELL_TEXT_INSET_PT + (available_width_pt - text_width_pt) / 2.0
    }
    _ => rect.x_pt + XLSX_CELL_TEXT_INSET_PT,
  };
  text_start_pt + leading_offset_pt
}

fn render_cell_borders(
  items: &mut Vec<PageItem>,
  rect: CellRect,
  borders: super::styles::BorderRecord,
) {
  if let Some(border) = borders.left {
    items.push(PageItem::Line(LineItem {
      x1_pt: rect.x_pt,
      y1_pt: rect.y_pt,
      x2_pt: rect.x_pt,
      y2_pt: rect.y_pt + rect.height_pt,
      width_pt: border.width_pt,
      color: border.color,
      kind: LineItemKind::Stroke,
    }));
  }
  if let Some(border) = borders.right {
    items.push(PageItem::Line(LineItem {
      x1_pt: rect.x_pt + rect.width_pt,
      y1_pt: rect.y_pt,
      x2_pt: rect.x_pt + rect.width_pt,
      y2_pt: rect.y_pt + rect.height_pt,
      width_pt: border.width_pt,
      color: border.color,
      kind: LineItemKind::Stroke,
    }));
  }
  if let Some(border) = borders.top {
    items.push(PageItem::Line(LineItem {
      x1_pt: rect.x_pt,
      y1_pt: rect.y_pt,
      x2_pt: rect.x_pt + rect.width_pt,
      y2_pt: rect.y_pt,
      width_pt: border.width_pt,
      color: border.color,
      kind: LineItemKind::Stroke,
    }));
  }
  if let Some(border) = borders.bottom {
    items.push(PageItem::Line(LineItem {
      x1_pt: rect.x_pt,
      y1_pt: rect.y_pt + rect.height_pt,
      x2_pt: rect.x_pt + rect.width_pt,
      y2_pt: rect.y_pt + rect.height_pt,
      width_pt: border.width_pt,
      color: border.color,
      kind: LineItemKind::Stroke,
    }));
  }
}

fn merge_cell_borders(
  target: &mut super::styles::BorderRecord,
  source: super::styles::BorderRecord,
) {
  if source.left.is_some() {
    target.left = source.left;
  }
  if source.right.is_some() {
    target.right = source.right;
  }
  if source.top.is_some() {
    target.top = source.top;
  }
  if source.bottom.is_some() {
    target.bottom = source.bottom;
  }
}

fn render_grid(
  items: &mut Vec<PageItem>,
  page: &CalcPrintPage<'_>,
  area: super::worksheet::CellRange,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) {
  let width = page.sheet.range_rect(area).width_pt * zoom_scale;
  let height = page.sheet.range_rect(area).height_pt * zoom_scale;
  let color = RgbColor { r: 0, g: 0, b: 0 };
  let mut x = origin_x_pt;
  for col in area.start.col..=area.end.col + 1 {
    if col > area.start.col {
      x += page.sheet.column_width_pt(col - 1) * zoom_scale;
    }
    items.push(PageItem::Line(LineItem {
      x1_pt: x,
      y1_pt: origin_y_pt,
      x2_pt: x,
      y2_pt: origin_y_pt + height,
      width_pt: XLSX_GRID_LINE_WIDTH_PT,
      color,
      kind: LineItemKind::Stroke,
    }));
  }
  let mut y = origin_y_pt;
  for row in area.start.row..=area.end.row + 1 {
    if row > area.start.row {
      y += page.sheet.row_height_pt(row - 1) * zoom_scale;
    }
    items.push(PageItem::Line(LineItem {
      x1_pt: origin_x_pt,
      y1_pt: y,
      x2_pt: origin_x_pt + width,
      y2_pt: y,
      width_pt: XLSX_GRID_LINE_WIDTH_PT,
      color,
      kind: LineItemKind::Stroke,
    }));
  }
}

#[derive(Clone, Copy, Debug)]
struct HeadingRenderLayout {
  row_header_x_pt: f32,
  row_header_y_pt: f32,
  col_header_x_pt: f32,
  col_header_y_pt: f32,
  zoom_scale: f32,
}

fn render_headings(
  items: &mut Vec<PageItem>,
  page: &CalcPrintPage<'_>,
  area: super::worksheet::CellRange,
  layout: HeadingRenderLayout,
) {
  let mut x = layout.col_header_x_pt;
  for col in area.start.col..=area.end.col {
    let width = page.sheet.column_width_pt(col) * layout.zoom_scale;
    items.push(header_text(
      x + XLSX_CELL_TEXT_INSET_PT,
      layout.col_header_y_pt,
      column_label(col),
    ));
    x += width;
  }
  let mut y = layout.row_header_y_pt;
  for row in area.start.row..=area.end.row {
    let height = page.sheet.row_height_pt(row) * layout.zoom_scale;
    items.push(header_text(
      layout.row_header_x_pt + XLSX_CELL_TEXT_INSET_PT,
      y,
      row.to_string(),
    ));
    y += height;
  }
}

fn header_text(x_pt: f32, y_pt: f32, text: String) -> PageItem {
  styled_header_text(x_pt, y_pt, text, TextStyle::default())
}

fn styled_header_text(x_pt: f32, y_pt: f32, text: String, style: TextStyle) -> PageItem {
  styled_header_text_with_line_height(x_pt, y_pt, text, style, XLSX_HEADER_FOOTER_LINE_HEIGHT_PT)
}

fn styled_header_text_with_line_height(
  x_pt: f32,
  y_pt: f32,
  text: String,
  style: TextStyle,
  line_height_pt: f32,
) -> PageItem {
  PageItem::Text(TextItem {
    x_pt,
    y_pt,
    line_height_pt,
    text,
    style,
    rotation_center_pt: None,
    hyperlink_url: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: false,
    pdf_text_segmentation: PdfTextSegmentation::Line,
  })
}

fn column_label(mut col: u32) -> String {
  let mut label = Vec::new();
  while col > 0 {
    col -= 1;
    label.push((b'A' + (col % 26) as u8) as char);
    col /= 26;
  }
  label.iter().rev().collect()
}

fn print_page_image_items(
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_area_rect = page.area.map(|area| page.sheet.range_rect(area));
  for drawing in &page.sheet.resources.drawings {
    for anchor in &drawing.anchors {
      if anchor.object.hidden || !anchor.print_with_sheet {
        continue;
      }
      if !drawing_anchor_intersects_page(page, anchor) {
        continue;
      }
      if anchor.object.kind != super::drawing::DrawingObjectKind::Picture {
        continue;
      }
      let Some(relationship_id) = anchor.object.relationship_id.as_deref() else {
        continue;
      };
      let Some(resource) = drawing.image_resources.get(relationship_id) else {
        continue;
      };
      let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
        continue;
      };
      if width_pt <= 0.0 || height_pt <= 0.0 {
        continue;
      }
      let (x_pt, y_pt) =
        page_area_rect.map_or((x_pt, y_pt), |rect| (x_pt - rect.x_pt, y_pt - rect.y_pt));
      let hyperlink_url = drawing_object_hyperlink_url(drawing, &anchor.object);
      items.push(PageItem::Image(ImageItem {
        x_pt: origin_x_pt + x_pt * zoom_scale,
        y_pt: origin_y_pt + y_pt * zoom_scale,
        width_pt: width_pt * zoom_scale,
        height_pt: height_pt * zoom_scale,
        crop: ImageCrop::default(),
        clip_path: Vec::new(),
        rotation_deg: 0.0,
        flip_horizontal: false,
        flip_vertical: false,
        data: resource.data.clone(),
        content_type: resource.content_type.clone(),
        alt_text: anchor
          .object
          .description
          .clone()
          .or_else(|| anchor.object.name.clone()),
        hyperlink_url: hyperlink_url.as_deref().map(ToString::to_string),
        floating: false,
        behind_text: false,
      }));
      render_metafile_texts(
        &mut items,
        resource,
        origin_x_pt + x_pt * zoom_scale,
        origin_y_pt + y_pt * zoom_scale,
        width_pt * zoom_scale,
        height_pt * zoom_scale,
        hyperlink_url.as_deref(),
      );
    }
  }
  for drawing in &page.sheet.resources.object_resources.vml_drawings {
    for shape in &drawing.shapes {
      if shape.hidden || !shape.print_object {
        continue;
      }
      if !vml_shape_intersects_page(page, shape) {
        continue;
      }
      let Some(relationship_id) = shape.image_relationship_id.as_deref() else {
        continue;
      };
      let Some(resource) = drawing.image_resources.get(relationship_id) else {
        continue;
      };
      let Some((x_pt, y_pt, width_pt, height_pt)) = vml_shape_rect(page.sheet, shape) else {
        continue;
      };
      if width_pt <= 0.0 || height_pt <= 0.0 {
        continue;
      }
      let (x_pt, y_pt) =
        page_area_rect.map_or((x_pt, y_pt), |rect| (x_pt - rect.x_pt, y_pt - rect.y_pt));
      items.push(PageItem::Image(ImageItem {
        x_pt: origin_x_pt + x_pt * zoom_scale,
        y_pt: origin_y_pt + y_pt * zoom_scale,
        width_pt: width_pt * zoom_scale,
        height_pt: height_pt * zoom_scale,
        crop: ImageCrop::default(),
        clip_path: Vec::new(),
        rotation_deg: 0.0,
        flip_horizontal: false,
        flip_vertical: false,
        data: resource.data.clone(),
        content_type: resource.content_type.clone(),
        alt_text: None,
        hyperlink_url: None,
        floating: false,
        behind_text: false,
      }));
      render_metafile_texts(
        &mut items,
        resource,
        origin_x_pt + x_pt * zoom_scale,
        origin_y_pt + y_pt * zoom_scale,
        width_pt * zoom_scale,
        height_pt * zoom_scale,
        None,
      );
    }
  }
  items
}

fn print_page_shape_items(
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_area_rect = page.area.map(|area| page.sheet.range_rect(area));
  for anchor in page
    .sheet
    .resources
    .drawings
    .iter()
    .flat_map(|drawing| drawing.anchors.iter())
  {
    if anchor.object.hidden || !anchor.print_with_sheet {
      continue;
    }
    if !drawing_anchor_intersects_page(page, anchor) {
      continue;
    }
    if !matches!(
      anchor.object.kind,
      super::drawing::DrawingObjectKind::Shape
        | super::drawing::DrawingObjectKind::GroupShape
        | super::drawing::DrawingObjectKind::ConnectionShape
    ) {
      continue;
    }
    let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
      continue;
    };
    if width_pt <= 0.0 || height_pt <= 0.0 {
      continue;
    }
    let (x_pt, y_pt) =
      page_area_rect.map_or((x_pt, y_pt), |rect| (x_pt - rect.x_pt, y_pt - rect.y_pt));
    items.push(PageItem::Rect(RectItem {
      x_pt: origin_x_pt + x_pt * zoom_scale,
      y_pt: origin_y_pt + y_pt * zoom_scale,
      width_pt: width_pt * zoom_scale,
      height_pt: height_pt * zoom_scale,
      fill_color: anchor.object.fill_color,
      fill_opacity: 1.0,
      stroke: shape_stroke(&anchor.object),
      stroke_opacity: 1.0,
    }));
  }
  items
}

fn print_page_diagram_items(
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_area_rect = page.area.map(|area| page.sheet.range_rect(area));
  for drawing in &page.sheet.resources.drawings {
    for anchor in &drawing.anchors {
      if anchor.object.hidden
        || !anchor.print_with_sheet
        || anchor.object.kind != super::drawing::DrawingObjectKind::GraphicFrame
      {
        continue;
      }
      if !drawing_anchor_intersects_page(page, anchor) {
        continue;
      }
      let Some(relationship_id) = anchor.object.relationship_id.as_deref() else {
        continue;
      };
      let Some(data) = drawing
        .diagrams
        .data_parts
        .iter()
        .find(|data| data.relationship_id.as_deref() == Some(relationship_id))
        .or_else(|| drawing.diagrams.data_parts.first())
      else {
        continue;
      };
      let Some(data_model) = data.data_model.as_deref() else {
        continue;
      };
      let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
        continue;
      };
      if width_pt <= 0.0 || height_pt <= 0.0 {
        continue;
      }
      let (x_pt, y_pt) =
        page_area_rect.map_or((x_pt, y_pt), |rect| (x_pt - rect.x_pt, y_pt - rect.y_pt));
      let bounds = shared_diagram::DiagramBounds {
        x: origin_x_pt + x_pt * zoom_scale,
        y: origin_y_pt + y_pt * zoom_scale,
        width: width_pt * zoom_scale,
        height: height_pt * zoom_scale,
      };
      if let Some(drawing) = persisted_diagram_drawing(&drawing.diagrams, data_model)
        && push_persisted_diagram_items(&mut items, drawing, bounds)
      {
        continue;
      }
      for shape in shared_diagram::layout_shapes(
        data_model,
        drawing
          .diagrams
          .layout_parts
          .iter()
          .find_map(|layout| layout.layout.as_deref()),
        None,
        None,
        bounds,
        RgbColor {
          r: 0x4f,
          g: 0x81,
          b: 0xbd,
        },
      ) {
        push_diagram_shape_items(&mut items, &shape);
      }
    }
  }
  items
}

fn persisted_diagram_drawing<'a>(
  diagrams: &'a super::drawing::DiagramResourceCatalog,
  data: &dgm::DataModelRoot,
) -> Option<&'a dsp::Drawing> {
  let relationship_id = data
    .data_model_extension_list
    .as_ref()?
    .data_model_extension
    .iter()
    .find_map(
      |extension| match extension.data_model_extension_choice.as_ref()? {
        a::DataModelExtensionChoice::DataModelExtensionBlock(block) => block.rel_id.as_deref(),
        _ => None,
      },
    )?;
  diagrams
    .drawing_parts
    .iter()
    .find(|drawing| drawing.relationship_id.as_deref() == Some(relationship_id))
    .and_then(|drawing| drawing.drawing.as_deref())
}

fn push_persisted_diagram_items(
  items: &mut Vec<PageItem>,
  drawing: &dsp::Drawing,
  bounds: shared_diagram::DiagramBounds,
) -> bool {
  let content_bounds = persisted_diagram_content_bounds(drawing);
  let start_len = items.len();
  for choice in &drawing.shape_tree.shape_tree_choice {
    match choice {
      dsp::ShapeTreeChoice::Shape(shape) => {
        push_persisted_diagram_shape(items, shape, bounds, content_bounds);
      }
      dsp::ShapeTreeChoice::GroupShape(group) => {
        push_persisted_diagram_group(items, group, bounds, content_bounds);
      }
    }
  }
  items.len() > start_len
}

fn push_persisted_diagram_group(
  items: &mut Vec<PageItem>,
  group: &dsp::GroupShape,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) {
  for choice in &group.group_shape_choice {
    match choice {
      dsp::GroupShapeChoice::Shape(shape) => {
        push_persisted_diagram_shape(items, shape, bounds, content_bounds);
      }
      dsp::GroupShapeChoice::GroupShape(group) => {
        push_persisted_diagram_group(items, group, bounds, content_bounds);
      }
    }
  }
}

fn push_persisted_diagram_shape(
  items: &mut Vec<PageItem>,
  shape: &dsp::Shape,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) {
  let Some((x, y, width, height)) = transform_bounds_pt(
    shape.shape_properties.transform2_d.as_deref(),
    bounds,
    content_bounds,
  ) else {
    return;
  };
  items.push(PageItem::Rect(RectItem {
    x_pt: x,
    y_pt: y,
    width_pt: width,
    height_pt: height,
    fill_color: Some(RgbColor {
      r: 0x4f,
      g: 0x81,
      b: 0xbd,
    }),
    fill_opacity: 1.0,
    stroke: Some(BorderStyle::default()),
    stroke_opacity: 1.0,
  }));
  let Some(text_body) = shape.text_body.as_deref() else {
    return;
  };
  let text = dml_paragraph_texts(&text_body.paragraph);
  if text.trim().is_empty() {
    return;
  }
  let (text_x, text_y, text_width, text_height) =
    transform_bounds_pt_dsp(shape.transform2_d.as_deref(), bounds, content_bounds)
      .unwrap_or((x, y, width, height));
  render_drawing_text(
    items,
    &text,
    CellRect {
      x_pt: text_x,
      y_pt: text_y,
      width_pt: text_width,
      height_pt: text_height,
    },
    persisted_diagram_text_style(text_body),
    None,
    None,
  );
}

fn persisted_diagram_text_style(text_body: &dsp::TextBody) -> Option<TextStyle> {
  let mut style = TextStyle::default();
  let mut changed = false;
  for choice in text_body
    .paragraph
    .iter()
    .flat_map(|paragraph| paragraph.paragraph_choice.iter())
  {
    if let a::ParagraphChoice::Run(run) = choice
      && let Some(properties) = run.run_properties.as_deref()
    {
      if let Some(font_size) = properties.font_size {
        style.font_size_pt = font_size as f32 / 100.0;
        changed = true;
      }
      break;
    }
  }
  changed.then_some(style)
}

fn dml_paragraph_texts(paragraphs: &[a::Paragraph]) -> String {
  paragraphs
    .iter()
    .map(|paragraph| {
      paragraph
        .paragraph_choice
        .iter()
        .filter_map(|choice| match choice {
          a::ParagraphChoice::Run(run) => Some(run.text.as_str()),
          a::ParagraphChoice::Field(field) => field.text.as_deref(),
          _ => None,
        })
        .collect::<String>()
    })
    .filter(|line| !line.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}

fn transform_bounds_pt(
  transform: Option<&a::Transform2D>,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) -> Option<(f32, f32, f32, f32)> {
  let transform = transform?;
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  transform_bounds_from_emu(
    offset.x.to_emu(),
    offset.y.to_emu(),
    extents.cx.to_emu(),
    extents.cy.to_emu(),
    bounds,
    content_bounds,
  )
}

fn transform_bounds_pt_dsp(
  transform: Option<&dsp::Transform2D>,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) -> Option<(f32, f32, f32, f32)> {
  let transform = transform?;
  let offset = transform.offset.as_ref()?;
  let extents = transform.extents.as_ref()?;
  transform_bounds_from_emu(
    offset.x.to_emu(),
    offset.y.to_emu(),
    extents.cx.to_emu(),
    extents.cy.to_emu(),
    bounds,
    content_bounds,
  )
}

fn transform_bounds_from_emu(
  x_emu: i64,
  y_emu: i64,
  width_emu: i64,
  height_emu: i64,
  bounds: shared_diagram::DiagramBounds,
  content_bounds: Option<(i64, i64, i64, i64)>,
) -> Option<(f32, f32, f32, f32)> {
  let (min_x, min_y, max_x, max_y) =
    content_bounds.unwrap_or((x_emu, y_emu, x_emu + width_emu, y_emu + height_emu));
  let content_width = (max_x - min_x).max(1) as f32;
  let content_height = (max_y - min_y).max(1) as f32;
  Some((
    bounds.x + (x_emu - min_x) as f32 / content_width * bounds.width,
    bounds.y + (y_emu - min_y) as f32 / content_height * bounds.height,
    width_emu as f32 / content_width * bounds.width,
    height_emu as f32 / content_height * bounds.height,
  ))
}

fn persisted_diagram_content_bounds(drawing: &dsp::Drawing) -> Option<(i64, i64, i64, i64)> {
  let mut bounds = None;
  for choice in &drawing.shape_tree.shape_tree_choice {
    collect_persisted_diagram_bounds(choice, &mut bounds);
  }
  bounds
}

fn collect_persisted_diagram_bounds(
  choice: &dsp::ShapeTreeChoice,
  bounds: &mut Option<(i64, i64, i64, i64)>,
) {
  match choice {
    dsp::ShapeTreeChoice::Shape(shape) => {
      if let Some(transform) = shape.shape_properties.transform2_d.as_deref()
        && let (Some(offset), Some(extents)) = (&transform.offset, &transform.extents)
      {
        let shape_bounds = (
          offset.x.to_emu(),
          offset.y.to_emu(),
          offset.x.to_emu() + extents.cx.to_emu(),
          offset.y.to_emu() + extents.cy.to_emu(),
        );
        *bounds = Some(match *bounds {
          Some((min_x, min_y, max_x, max_y)) => (
            min_x.min(shape_bounds.0),
            min_y.min(shape_bounds.1),
            max_x.max(shape_bounds.2),
            max_y.max(shape_bounds.3),
          ),
          None => shape_bounds,
        });
      }
    }
    dsp::ShapeTreeChoice::GroupShape(group) => {
      collect_persisted_group_bounds(group, bounds);
    }
  }
}

fn collect_persisted_group_bounds(
  group: &dsp::GroupShape,
  bounds: &mut Option<(i64, i64, i64, i64)>,
) {
  for child in &group.group_shape_choice {
    match child {
      dsp::GroupShapeChoice::Shape(shape) => {
        if let Some(transform) = shape.shape_properties.transform2_d.as_deref()
          && let (Some(offset), Some(extents)) = (&transform.offset, &transform.extents)
        {
          let shape_bounds = (
            offset.x.to_emu(),
            offset.y.to_emu(),
            offset.x.to_emu() + extents.cx.to_emu(),
            offset.y.to_emu() + extents.cy.to_emu(),
          );
          *bounds = Some(match *bounds {
            Some((min_x, min_y, max_x, max_y)) => (
              min_x.min(shape_bounds.0),
              min_y.min(shape_bounds.1),
              max_x.max(shape_bounds.2),
              max_y.max(shape_bounds.3),
            ),
            None => shape_bounds,
          });
        }
      }
      dsp::GroupShapeChoice::GroupShape(group) => collect_persisted_group_bounds(group, bounds),
    }
  }
}

fn push_diagram_shape_items(items: &mut Vec<PageItem>, shape: &shared_diagram::DiagramShape) {
  if shape.is_connector {
    items.push(PageItem::Line(diagram_connector_line_item(shape)));
  } else {
    items.push(PageItem::Rect(RectItem {
      x_pt: shape.x,
      y_pt: shape.y,
      width_pt: shape.width,
      height_pt: shape.height,
      fill_color: Some(shape.fill),
      fill_opacity: 1.0,
      stroke: Some(BorderStyle::default()),
      stroke_opacity: 1.0,
    }));
  }
  let text = diagram_text_body_text(&shape.text_body);
  if !text.trim().is_empty() {
    render_drawing_text(
      items,
      &text,
      CellRect {
        x_pt: shape.x,
        y_pt: shape.y,
        width_pt: shape.width,
        height_pt: shape.height,
      },
      None,
      None,
      None,
    );
  }
}

fn diagram_connector_line_item(shape: &shared_diagram::DiagramShape) -> LineItem {
  let center_x = shape.x + shape.width / 2.0;
  let center_y = shape.y + shape.height / 2.0;
  let length = shape.width.max(shape.height).max(1.0);
  let radians = shape.connector_angle_deg.to_radians();
  let dx = radians.cos() * length / 2.0;
  let dy = radians.sin() * length / 2.0;
  LineItem {
    x1_pt: center_x - dx,
    y1_pt: center_y - dy,
    x2_pt: center_x + dx,
    y2_pt: center_y + dy,
    width_pt: 1.0,
    color: RgbColor { r: 0, g: 0, b: 0 },
    kind: LineItemKind::Stroke,
  }
}

fn diagram_text_body_text(text_body: &shared_diagram::DiagramTextBody) -> String {
  text_body
    .paragraphs
    .iter()
    .map(|paragraph| {
      paragraph
        .runs
        .iter()
        .map(|run| run.text.as_str())
        .collect::<String>()
    })
    .filter(|line| !line.trim().is_empty())
    .collect::<Vec<_>>()
    .join("\n")
}

fn print_page_drawing_text_items(
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_area_rect = page.area.map(|area| page.sheet.range_rect(area));
  let page_clip_rect = page_area_rect.map_or(
    CellRect {
      x_pt: 0.0,
      y_pt: 0.0,
      width_pt: setup.width_pt,
      height_pt: setup.height_pt,
    },
    |rect| CellRect {
      x_pt: origin_x_pt,
      y_pt: origin_y_pt,
      width_pt: rect.width_pt * zoom_scale,
      height_pt: rect.height_pt * zoom_scale,
    },
  );
  for drawing in &page.sheet.resources.drawings {
    for anchor in &drawing.anchors {
      if anchor.object.hidden || !anchor.print_with_sheet {
        continue;
      }
      if !drawing_anchor_intersects_page(page, anchor) {
        continue;
      }
      let Some((x_pt, y_pt, width_pt, height_pt)) = anchor_rect_pt(page.sheet, anchor) else {
        continue;
      };
      if width_pt <= 0.0 || height_pt <= 0.0 {
        continue;
      }
      let text = drawing_anchor_text(drawing, anchor);
      if text.trim().is_empty() {
        continue;
      }
      let (x_pt, y_pt) =
        page_area_rect.map_or((x_pt, y_pt), |rect| (x_pt - rect.x_pt, y_pt - rect.y_pt));
      let drawing_rect = CellRect {
        x_pt: origin_x_pt + x_pt * zoom_scale,
        y_pt: origin_y_pt + y_pt * zoom_scale,
        width_pt: width_pt * zoom_scale,
        height_pt: height_pt * zoom_scale,
      };
      if let Some(chart_items) =
        lower_drawing_chart(import, drawing, anchor, drawing_rect, page_clip_rect)
        && !chart_items.is_empty()
      {
        items.extend(chart_items);
        continue;
      }
      let hyperlink_url = drawing_object_hyperlink_url(drawing, &anchor.object);
      render_drawing_text(
        &mut items,
        &text,
        drawing_rect,
        drawing_object_text_style(import, &anchor.object),
        Some(drawing_object_text_layout(&anchor.object)),
        hyperlink_url.as_deref(),
      );
    }
  }
  items
}

fn lower_drawing_chart(
  import: &ExcelImport,
  drawing: &super::drawing::DrawingResourceCatalog,
  anchor: &super::drawing::DrawingAnchorModel,
  rect: CellRect,
  page_clip_rect: CellRect,
) -> Option<Vec<PageItem>> {
  if anchor.object.kind != super::drawing::DrawingObjectKind::GraphicFrame {
    return None;
  }
  let relationship_id = anchor.object.relationship_id.as_deref()?;
  let resource = drawing
    .charts
    .iter()
    .chain(drawing.extended_charts.iter())
    .find(|chart| chart.relationship_id.as_deref() == Some(relationship_id))?;
  let chart_space = resource.chart_space.as_deref()?;
  let chart = shared_chart::clustered_column_chart(chart_space)?;
  let series_colors = chart
    .series
    .iter()
    .enumerate()
    .map(|(index, series)| {
      series
        .solid_fill
        .and_then(|fill| xlsx_chart_solid_fill_color(fill, import))
        .or_else(|| import.styles.theme_color(4 + index as u32 % 6, 0.0))
        .unwrap_or(XLSX_DEFAULT_CHART_SERIES_COLORS[index % 6])
    })
    .collect();
  let mut title_style = import.styles.default_chart_text_style();
  title_style.font_size_pt = 14.0;
  title_style.bold = true;
  let mut label_style = import.styles.default_chart_text_style();
  label_style.font_size_pt = 10.0;
  if let Some(typeface) = xlsx_chart_latin_typeface(chart_space) {
    let typeface = Arc::from(typeface);
    title_style.font_family = Some(Arc::clone(&typeface));
    label_style.font_family = Some(typeface);
  }
  let mut items = lower_clustered_column_chart(
    ChartFrame {
      x_pt: rect.x_pt,
      y_pt: rect.y_pt,
      width_pt: rect.width_pt,
      height_pt: rect.height_pt,
    },
    &chart,
    shared_chart::automatic_chart_title(None),
    &ClusteredColumnStyle {
      layout_profile: ChartLayoutProfile::Excel,
      has_explicit_title: false,
      title: title_style,
      title_fill_color: None,
      label: label_style,
      gridline_color: RgbColor {
        r: 0xd9,
        g: 0xd9,
        b: 0xd9,
      },
      series_colors,
    },
  );
  let mut metrics = TextMetrics::new();
  items.retain_mut(|item| clip_chart_item_to_rect(item, page_clip_rect, &mut metrics));
  Some(items)
}

fn xlsx_chart_latin_typeface(chart_space: &c::ChartSpace) -> Option<&str> {
  for properties in [
    chart_space.text_properties.as_deref(),
    chart_space
      .chart
      .title
      .as_deref()
      .and_then(|title| title.text_properties.as_deref()),
    chart_space
      .chart
      .legend
      .as_deref()
      .and_then(|legend| legend.text_properties.as_deref()),
  ]
  .into_iter()
  .flatten()
  {
    if let Some(typeface) = chart_text_properties_latin_typeface(properties) {
      return Some(typeface);
    }
  }
  chart_space
    .chart
    .plot_area
    .plot_area_choice2
    .iter()
    .find_map(|axis| {
      let properties = match axis {
        c::PlotAreaChoice2::ValueAxis(axis) => axis.text_properties.as_deref(),
        c::PlotAreaChoice2::CategoryAxis(axis) => axis.text_properties.as_deref(),
        c::PlotAreaChoice2::DateAxis(axis) => axis.text_properties.as_deref(),
        c::PlotAreaChoice2::SeriesAxis(axis) => axis.text_properties.as_deref(),
      }?;
      chart_text_properties_latin_typeface(properties)
    })
}

fn chart_text_properties_latin_typeface(properties: &c::TextProperties) -> Option<&str> {
  properties
    .paragraph
    .iter()
    .filter_map(|paragraph| paragraph.paragraph_properties.as_deref())
    .filter_map(|paragraph| paragraph.default_run_properties.as_deref())
    .find_map(|properties| properties.latin_font.as_ref()?.typeface.as_deref())
    .or_else(|| {
      properties
        .list_style
        .as_deref()
        .and_then(|style| style.default_paragraph_properties.as_deref())
        .and_then(|paragraph| paragraph.default_run_properties.as_deref())
        .and_then(|properties| properties.latin_font.as_ref()?.typeface.as_deref())
    })
}

fn clip_chart_item_to_rect(item: &mut PageItem, clip: CellRect, metrics: &mut TextMetrics) -> bool {
  match item {
    PageItem::Text(text) => rect_intersects_clip(
      text.x_pt,
      text.y_pt,
      text.x_pt + metrics.measure_text(&text.text, &text.style),
      text.y_pt + text.line_height_pt,
      clip,
    ),
    PageItem::Rect(rect) => {
      let left = rect.x_pt.max(clip.x_pt);
      let top = rect.y_pt.max(clip.y_pt);
      let right = (rect.x_pt + rect.width_pt).min(clip.x_pt + clip.width_pt);
      let bottom = (rect.y_pt + rect.height_pt).min(clip.y_pt + clip.height_pt);
      if right <= left || bottom <= top {
        return false;
      }
      rect.x_pt = left;
      rect.y_pt = top;
      rect.width_pt = right - left;
      rect.height_pt = bottom - top;
      true
    }
    PageItem::Line(line) if line.y1_pt == line.y2_pt => {
      if line.y1_pt < clip.y_pt || line.y1_pt > clip.y_pt + clip.height_pt {
        return false;
      }
      line.x1_pt = line.x1_pt.clamp(clip.x_pt, clip.x_pt + clip.width_pt);
      line.x2_pt = line.x2_pt.clamp(clip.x_pt, clip.x_pt + clip.width_pt);
      line.x1_pt != line.x2_pt
    }
    PageItem::Line(line) if line.x1_pt == line.x2_pt => {
      if line.x1_pt < clip.x_pt || line.x1_pt > clip.x_pt + clip.width_pt {
        return false;
      }
      line.y1_pt = line.y1_pt.clamp(clip.y_pt, clip.y_pt + clip.height_pt);
      line.y2_pt = line.y2_pt.clamp(clip.y_pt, clip.y_pt + clip.height_pt);
      line.y1_pt != line.y2_pt
    }
    PageItem::Line(line) => rect_intersects_clip(
      line.x1_pt.min(line.x2_pt),
      line.y1_pt.min(line.y2_pt),
      line.x1_pt.max(line.x2_pt),
      line.y1_pt.max(line.y2_pt),
      clip,
    ),
    PageItem::Image(_) | PageItem::LinkArea(_) | PageItem::Path(_) => true,
  }
}

fn rect_intersects_clip(left: f32, top: f32, right: f32, bottom: f32, clip: CellRect) -> bool {
  right > clip.x_pt
    && bottom > clip.y_pt
    && left < clip.x_pt + clip.width_pt
    && top < clip.y_pt + clip.height_pt
}

const XLSX_DEFAULT_CHART_SERIES_COLORS: [RgbColor; 6] = [
  RgbColor {
    r: 0x44,
    g: 0x72,
    b: 0xc4,
  },
  RgbColor {
    r: 0xed,
    g: 0x7d,
    b: 0x31,
  },
  RgbColor {
    r: 0xa5,
    g: 0xa5,
    b: 0xa5,
  },
  RgbColor {
    r: 0xff,
    g: 0xc0,
    b: 0x00,
  },
  RgbColor {
    r: 0x5b,
    g: 0x9b,
    b: 0xd5,
  },
  RgbColor {
    r: 0x70,
    g: 0xad,
    b: 0x47,
  },
];

fn xlsx_chart_solid_fill_color(fill: &a::SolidFill, import: &ExcelImport) -> Option<RgbColor> {
  match fill.solid_fill_choice.as_ref()? {
    a::SolidFillChoice::RgbColorModelHex(color) => {
      let bytes = color.val.as_bytes();
      if bytes.len() != 6 {
        return None;
      }
      Some(RgbColor {
        r: u8::from_str_radix(std::str::from_utf8(&bytes[0..2]).ok()?, 16).ok()?,
        g: u8::from_str_radix(std::str::from_utf8(&bytes[2..4]).ok()?, 16).ok()?,
        b: u8::from_str_radix(std::str::from_utf8(&bytes[4..6]).ok()?, 16).ok()?,
      })
    }
    a::SolidFillChoice::SchemeColor(color) => {
      let index = match color.val {
        a::SchemeColorValues::Light1 => 0,
        a::SchemeColorValues::Dark1 => 1,
        a::SchemeColorValues::Light2 => 2,
        a::SchemeColorValues::Dark2 => 3,
        a::SchemeColorValues::Accent1 => 4,
        a::SchemeColorValues::Accent2 => 5,
        a::SchemeColorValues::Accent3 => 6,
        a::SchemeColorValues::Accent4 => 7,
        a::SchemeColorValues::Accent5 => 8,
        a::SchemeColorValues::Accent6 => 9,
        _ => return None,
      };
      import.styles.theme_color(index, 0.0)
    }
    _ => None,
  }
}

fn drawing_anchor_text<'a>(
  drawing: &'a super::drawing::DrawingResourceCatalog,
  anchor: &'a super::drawing::DrawingAnchorModel,
) -> Cow<'a, str> {
  if !anchor.object.text.is_empty() {
    return Cow::Borrowed(anchor.object.text.as_str());
  }
  if anchor.object.kind == super::drawing::DrawingObjectKind::GraphicFrame
    && let Some(relationship_id) = anchor.object.relationship_id.as_deref()
  {
    let chart_text = drawing
      .charts
      .iter()
      .chain(drawing.extended_charts.iter())
      .find(|chart| chart.relationship_id.as_deref() == Some(relationship_id))
      .map(|chart| chart.visible_texts.join("\n"));
    if let Some(chart_text) = chart_text.filter(|text| !text.is_empty()) {
      return Cow::Owned(chart_text);
    }
    if drawing
      .diagrams
      .data_parts
      .iter()
      .any(|data| data.relationship_id.as_deref() == Some(relationship_id))
    {
      return Cow::Borrowed("");
    }
  }
  Cow::Borrowed("")
}

fn render_drawing_text(
  items: &mut Vec<PageItem>,
  text: &str,
  rect: CellRect,
  style: Option<TextStyle>,
  layout: Option<DrawingTextLayout>,
  hyperlink_url: Option<&str>,
) {
  let style = style.unwrap_or_default();
  let layout = layout.unwrap_or_default();
  let line_height = (style.font_size_pt * 1.15).max(1.0);
  let mut text_metrics = TextMetrics::new();
  for (index, line) in text.lines().filter(|line| !line.is_empty()).enumerate() {
    let y = rect.y_pt + layout.top_inset_pt + index as f32 * line_height;
    if y > rect.y_pt + rect.height_pt - layout.bottom_inset_pt {
      break;
    }
    let available_width = (rect.width_pt - layout.left_inset_pt - layout.right_inset_pt).max(0.0);
    let text_width = text_metrics.measure_text(line, &style);
    let x = match layout.alignment {
      a::TextAlignmentTypeValues::Center => {
        rect.x_pt + layout.left_inset_pt + (available_width - text_width).max(0.0) / 2.0
      }
      a::TextAlignmentTypeValues::Right => {
        rect.x_pt + layout.left_inset_pt + (available_width - text_width).max(0.0)
      }
      _ => rect.x_pt + layout.left_inset_pt,
    };
    items.push(PageItem::Text(TextItem {
      x_pt: x,
      y_pt: y,
      line_height_pt: line_height,
      text: line.to_string(),
      style: style.clone(),
      rotation_center_pt: None,
      hyperlink_url: hyperlink_url.map(ToString::to_string),
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: false,
      pdf_text_segmentation: PdfTextSegmentation::Line,
    }));
  }
}

#[derive(Clone, Copy, Debug)]
struct DrawingTextLayout {
  alignment: a::TextAlignmentTypeValues,
  left_inset_pt: f32,
  top_inset_pt: f32,
  right_inset_pt: f32,
  bottom_inset_pt: f32,
}

impl Default for DrawingTextLayout {
  fn default() -> Self {
    Self {
      alignment: a::TextAlignmentTypeValues::Left,
      left_inset_pt: XLSX_CELL_TEXT_INSET_PT,
      top_inset_pt: XLSX_CELL_TEXT_INSET_PT,
      right_inset_pt: XLSX_CELL_TEXT_INSET_PT,
      bottom_inset_pt: XLSX_CELL_TEXT_INSET_PT,
    }
  }
}

fn drawing_object_text_layout(object: &super::drawing::DrawingObjectModel) -> DrawingTextLayout {
  DrawingTextLayout {
    alignment: object.text_alignment.unwrap_or_default(),
    left_inset_pt: object
      .text_left_inset_emu
      .map_or(XLSX_CELL_TEXT_INSET_PT, units::emu_to_points),
    top_inset_pt: object
      .text_top_inset_emu
      .map_or(XLSX_CELL_TEXT_INSET_PT, units::emu_to_points),
    right_inset_pt: object
      .text_right_inset_emu
      .map_or(XLSX_CELL_TEXT_INSET_PT, units::emu_to_points),
    bottom_inset_pt: object
      .text_bottom_inset_emu
      .map_or(XLSX_CELL_TEXT_INSET_PT, units::emu_to_points),
  }
}

fn drawing_object_text_style(
  import: &ExcelImport,
  object: &super::drawing::DrawingObjectModel,
) -> Option<TextStyle> {
  let mut style = TextStyle::default();
  let mut changed = false;
  if let Some(font_size) = object.text_font_size_points100 {
    style.font_size_pt = font_size as f32 / 100.0;
    changed = true;
  }
  if let Some(color) = object.text_color {
    style.color = color;
    changed = true;
  }
  if let Some(typeface) = object.text_font_family.as_deref() {
    style.font_family = Some(Arc::from(
      import.styles.resolve_drawingml_theme_font(typeface),
    ));
    changed = true;
  }
  if let Some(typeface) = object.text_east_asia_font_family.as_deref() {
    style.east_asia_font_family = Some(Arc::from(
      import.styles.resolve_drawingml_theme_font(typeface),
    ));
    changed = true;
  }
  if let Some(typeface) = object.text_complex_font_family.as_deref() {
    style.complex_font_family = Some(Arc::from(
      import.styles.resolve_drawingml_theme_font(typeface),
    ));
    changed = true;
  }
  changed.then_some(style)
}

fn render_metafile_texts(
  items: &mut Vec<PageItem>,
  resource: &super::drawing::ImageResource,
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
  hyperlink_url: Option<&str>,
) {
  let runs = emf_wmf::extract_metafile_text_runs(&resource.data, resource.content_type.as_deref());
  if runs.is_empty() {
    return;
  }
  for run in runs {
    let mut style = TextStyle::default();
    if let Some(font_size) = run.font_size {
      style.font_size_pt = (font_size * height_pt).max(1.0);
    }
    let line_height = (style.font_size_pt * 1.15).max(1.0);
    items.push(PageItem::Text(TextItem {
      x_pt: x_pt + run.x * width_pt,
      y_pt: y_pt + run.y * height_pt,
      line_height_pt: line_height,
      text: run.text,
      style,
      rotation_center_pt: None,
      hyperlink_url: hyperlink_url.map(ToString::to_string),
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: false,
      pdf_text_segmentation: PdfTextSegmentation::Line,
    }));
  }
}

fn print_page_vml_text_items(
  page: &CalcPrintPage<'_>,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) -> Vec<PageItem> {
  let mut items = Vec::new();
  let page_area_rect = page.area.map(|area| page.sheet.range_rect(area));
  for shape in page
    .sheet
    .resources
    .object_resources
    .vml_drawings
    .iter()
    .flat_map(|drawing| drawing.shapes.iter())
  {
    if shape.hidden || !shape.print_object {
      continue;
    }
    if !vml_shape_intersects_page(page, shape) {
      continue;
    }
    let text = vml_shape_visible_text(page.sheet, shape);
    if text.trim().is_empty() {
      continue;
    }
    let Some((x_pt, y_pt, width_pt, height_pt)) = vml_shape_rect(page.sheet, shape) else {
      continue;
    };
    let (x_pt, y_pt) =
      page_area_rect.map_or((x_pt, y_pt), |rect| (x_pt - rect.x_pt, y_pt - rect.y_pt));
    render_drawing_text(
      &mut items,
      text,
      CellRect {
        x_pt: origin_x_pt + x_pt * zoom_scale,
        y_pt: origin_y_pt + y_pt * zoom_scale,
        width_pt: width_pt * zoom_scale,
        height_pt: height_pt * zoom_scale,
      },
      None,
      None,
      None,
    );
  }
  items
}

fn vml_shape_visible_text<'a>(
  sheet: &'a CalcSheet,
  shape: &'a super::object_resources::VmlShapeModel,
) -> &'a str {
  if !shape.text.trim().is_empty() {
    return shape.text.as_str();
  }
  if shape.object_type.as_deref() != Some("Note") || !shape.visible {
    return "";
  }
  let Some(row) = shape.note_row.and_then(|row| row.checked_add(1)) else {
    return "";
  };
  let Some(col) = shape.note_column.and_then(|col| col.checked_add(1)) else {
    return "";
  };
  let address = super::worksheet::CellAddress { col, row };
  // legacy comments against the VML note shape map; visible note captions use
  // the comments part text when the VML textbox itself is empty.
  sheet
    .resources
    .comments
    .legacy
    .as_ref()
    .and_then(|legacy| {
      legacy.comments.iter().find_map(|comment| {
        super::worksheet::CellRange::parse_a1_range(&comment.reference)
          .is_some_and(|range| range.contains(address))
          .then_some(comment.text.as_str())
      })
    })
    .unwrap_or("")
}

fn drawing_object_hyperlink_url<'a>(
  drawing: &'a super::drawing::DrawingResourceCatalog,
  object: &'a super::drawing::DrawingObjectModel,
) -> Option<Cow<'a, str>> {
  object
    .hyperlink_relationship_id
    .as_deref()
    .and_then(|relationship_id| drawing.hyperlink_targets.get(relationship_id))
    .map(|url| Cow::Borrowed(url.as_str()))
    .or_else(|| object.hyperlink_invalid_url.as_deref().map(Cow::Borrowed))
    .or_else(|| {
      object
        .hyperlink_action
        .as_deref()
        .and_then(drawing_hyperlink_action_url)
        .map(Cow::Owned)
    })
}

fn drawing_hyperlink_action_url(action: &str) -> Option<String> {
  action
    .strip_prefix("ppaction://hlinkshowjump?jump=")
    .map(|jump| format!("ooxmlsdk-pdf-action://hlinkshowjump/{jump}"))
}

fn vml_shape_rect(
  sheet: &CalcSheet,
  shape: &super::object_resources::VmlShapeModel,
) -> Option<(f32, f32, f32, f32)> {
  shape
    .anchor
    .and_then(|anchor| vml_anchor_rect(sheet, anchor))
    .or_else(|| shape.style.as_deref().and_then(vml_style_rect))
}

fn vml_anchor_rect(
  sheet: &CalcSheet,
  anchor: super::object_resources::VmlClientAnchor,
) -> Option<(f32, f32, f32, f32)> {
  let x1 = vml_anchor_x(sheet, anchor.from_col, anchor.from_col_offset_px);
  let y1 = vml_anchor_y(sheet, anchor.from_row, anchor.from_row_offset_px);
  let x2 = vml_anchor_x(sheet, anchor.to_col, anchor.to_col_offset_px);
  let y2 = vml_anchor_y(sheet, anchor.to_row, anchor.to_row_offset_px);
  if x2 < x1 || y2 < y1 {
    return None;
  }
  Some((x1, y1, x2 - x1, y2 - y1))
}

fn vml_anchor_x(sheet: &CalcSheet, zero_based_col: u32, offset_px: i32) -> f32 {
  let col = zero_based_col.saturating_add(1);
  let cell = sheet.cell_rect(super::worksheet::CellAddress { col, row: 1 });
  let next_cell = sheet.cell_rect(super::worksheet::CellAddress {
    col: col.saturating_add(1),
    row: 1,
  });
  let x = cell.x_pt + vml_screen_pixel_to_pt(offset_px);
  x.min(next_cell.x_pt - units::twips_to_points(1.0))
}

fn vml_anchor_y(sheet: &CalcSheet, zero_based_row: u32, offset_px: i32) -> f32 {
  let row = zero_based_row.saturating_add(1);
  let cell = sheet.cell_rect(super::worksheet::CellAddress { col: 1, row });
  let next_cell = sheet.cell_rect(super::worksheet::CellAddress {
    col: 1,
    row: row.saturating_add(1),
  });
  let y = cell.y_pt + vml_screen_pixel_to_pt(offset_px);
  y.min(next_cell.y_pt - units::twips_to_points(1.0))
}

fn vml_screen_pixel_to_pt(value: i32) -> f32 {
  value as f32 * units::POINTS_PER_INCH / 96.0
}

fn vml_style_rect(style: &str) -> Option<(f32, f32, f32, f32)> {
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
    return value.trim().parse::<f32>().ok().map(|value| value * 0.75);
  }
  value.parse::<f32>().ok()
}

fn shape_stroke(object: &super::drawing::DrawingObjectModel) -> Option<BorderStyle> {
  if object.no_line {
    return None;
  }
  let color = object.line_color?;
  Some(BorderStyle {
    width_pt: object
      .line_width_emu
      .map(|value| units::emu_to_points(i64::from(value)))
      .unwrap_or(0.75),
    color,
    ..BorderStyle::default()
  })
}

fn anchor_rect_pt(
  sheet: &CalcSheet,
  anchor: &super::drawing::DrawingAnchorModel,
) -> Option<(f32, f32, f32, f32)> {
  match anchor.kind {
    super::drawing::DrawingAnchorKind::TwoCell => {
      let from = anchor.from.as_ref()?;
      let to = anchor.to.as_ref()?;
      let (x1, y1) = sheet.marker_position_pt(from);
      let (x2, y2) = sheet.marker_position_pt(to);
      Some((x1.min(x2), y1.min(y2), (x2 - x1).abs(), (y2 - y1).abs()))
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

fn drawing_anchor_intersects_page(
  page: &CalcPrintPage<'_>,
  anchor: &super::drawing::DrawingAnchorModel,
) -> bool {
  let Some(area) = page.area else {
    return true;
  };
  let Some(anchor_rect) = anchor_rect_pt(page.sheet, anchor) else {
    return false;
  };
  tuple_rect_intersects_cell_rect(anchor_rect, page.sheet.range_rect(area))
}

fn vml_shape_intersects_page(
  page: &CalcPrintPage<'_>,
  shape: &super::object_resources::VmlShapeModel,
) -> bool {
  let Some(area) = page.area else {
    return true;
  };
  let Some(shape_rect) = vml_shape_rect(page.sheet, shape) else {
    return false;
  };
  tuple_rect_intersects_cell_rect(shape_rect, page.sheet.range_rect(area))
}

fn tuple_rect_intersects_cell_rect(
  (x, y, width, height): (f32, f32, f32, f32),
  cell_rect: CellRect,
) -> bool {
  width > 0.0
    && height > 0.0
    && cell_rect.width_pt > 0.0
    && cell_rect.height_pt > 0.0
    && x < cell_rect.x_pt + cell_rect.width_pt
    && x + width > cell_rect.x_pt
    && y < cell_rect.y_pt + cell_rect.height_pt
    && y + height > cell_rect.y_pt
}

fn repeat_rows_for_page(page: &CalcPrintPage<'_>) -> Option<super::worksheet::CellRange> {
  let area = page.area?;
  let repeat_rows = page.repeated_rows?;
  Some(super::worksheet::CellRange::new(
    super::worksheet::CellAddress {
      col: area.start.col,
      row: repeat_rows.start.row,
    },
    super::worksheet::CellAddress {
      col: area.end.col,
      row: repeat_rows.end.row,
    },
  ))
}

fn repeat_columns_for_page(page: &CalcPrintPage<'_>) -> Option<super::worksheet::CellRange> {
  let area = page.area?;
  let repeat_columns = page.repeated_columns?;
  Some(super::worksheet::CellRange::new(
    super::worksheet::CellAddress {
      col: repeat_columns.start.col,
      row: area.start.row,
    },
    super::worksheet::CellAddress {
      col: repeat_columns.end.col,
      row: area.end.row,
    },
  ))
}

fn repeat_corner_for_page(page: &CalcPrintPage<'_>) -> Option<super::worksheet::CellRange> {
  let repeat_rows = page.repeated_rows?;
  let repeat_columns = page.repeated_columns?;
  Some(super::worksheet::CellRange::new(
    super::worksheet::CellAddress {
      col: repeat_columns.start.col,
      row: repeat_rows.start.row,
    },
    super::worksheet::CellAddress {
      col: repeat_columns.end.col,
      row: repeat_rows.end.row,
    },
  ))
}

fn hyperlink_for_cell(
  page: &CalcPrintPage<'_>,
  address: super::worksheet::CellAddress,
) -> Option<String> {
  page
    .sheet
    .metrics
    .hyperlinks
    .iter()
    .find(|hyperlink| {
      super::worksheet::CellRange::parse_a1_range(&hyperlink.reference)
        .is_some_and(|range| range.contains(address))
    })
    .and_then(|hyperlink| {
      hyperlink
        .relationship_id
        .as_deref()
        .and_then(|id| page.sheet.resources.relationships.hyperlink_targets.get(id))
        .cloned()
        .or_else(|| {
          hyperlink
            .location
            .as_ref()
            .map(|location| format!("#{location}"))
        })
    })
}

fn render_header_or_footer(
  items: &mut Vec<PageItem>,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
  header: bool,
  styles: &super::styles::StylesCatalog,
  text_metrics: &mut TextMetrics,
) {
  let Some(text) = header_footer_text(page, header) else {
    return;
  };
  render_header_footer_line(items, header, page, setup, text, styles, text_metrics);
}

fn header_footer_text<'a>(page: &CalcPrintPage<'a>, header: bool) -> Option<&'a str> {
  let model = &page.page_settings.header_footer;
  if page.sheet_page_index == 0 && model.different_first {
    if header {
      model.first_header.as_deref()
    } else {
      model.first_footer.as_deref()
    }
  } else if page.sheet_page_index % 2 == 1 && model.different_odd_even {
    if header {
      model.even_header.as_deref()
    } else {
      model.even_footer.as_deref()
    }
  } else if header {
    model.odd_header.as_deref()
  } else {
    model.odd_footer.as_deref()
  }
}

fn render_header_footer_line(
  items: &mut Vec<PageItem>,
  header: bool,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
  text: &str,
  styles: &super::styles::StylesCatalog,
  text_metrics: &mut TextMetrics,
) {
  for (align, value) in split_header_footer_sections(text) {
    if value.is_empty() {
      continue;
    }
    let mut runs = parse_header_footer_runs(
      &value,
      styles.default_font_text_style(),
      HeaderFooterFieldValues {
        page_number: page.page_number,
        total_pages: page.total_pages,
        sheet_name: &page.sheet.name,
      },
    );
    if runs.is_empty() {
      continue;
    }
    if page.page_settings.header_footer.scale_with_doc {
      let print_scale = page.zoom as f32 / 100.0;
      for run in &mut runs {
        run.style.font_size_pt *= print_scale;
      }
    }
    // OOXML pageMargins.header/footer is the distance from the page edge to
    // the start/end of the header/footer. LibreOffice's HeaderFooterParser
    // likewise computes each portion's height from its active font runs, and
    // PageSettingsConverter describes the footer margin as the distance to
    // the bottom of the footer. A fixed 12pt box misplaces any portion whose
    // font metrics or explicit &nn size produce a different line height.
    let line_height_pt = runs
      .iter()
      .map(|run| text_metrics.inline_text_box_height(&run.style))
      .fold(0.0_f32, f32::max)
      .max(1.0);
    let y_pt = if header {
      setup.header_distance_pt
    } else {
      setup.height_pt - setup.footer_distance_pt - line_height_pt
    };
    let total_width = runs
      .iter()
      .map(|run| text_metrics.measure_text(&run.text, &run.style))
      .sum::<f32>();
    let mut x = match align {
      HeaderFooterAlign::Left => setup.margin_left_pt,
      HeaderFooterAlign::Center => (setup.width_pt - total_width) / 2.0,
      HeaderFooterAlign::Right => setup.width_pt - setup.margin_right_pt - total_width,
    };
    for run in runs {
      let width = text_metrics.measure_text(&run.text, &run.style);
      items.push(styled_header_text_with_line_height(
        x,
        y_pt,
        run.text,
        run.style,
        line_height_pt,
      ));
      x += width;
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HeaderFooterAlign {
  Left,
  Center,
  Right,
}

fn split_header_footer_sections(text: &str) -> Vec<(HeaderFooterAlign, String)> {
  let mut output = Vec::new();
  let mut current = HeaderFooterAlign::Center;
  let mut buffer = String::new();
  let mut chars = text.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch == '&' {
      match chars.peek().copied() {
        Some('L') => {
          chars.next();
          push_header_footer_section(&mut output, current, &mut buffer);
          current = HeaderFooterAlign::Left;
          continue;
        }
        Some('C') => {
          chars.next();
          push_header_footer_section(&mut output, current, &mut buffer);
          current = HeaderFooterAlign::Center;
          continue;
        }
        Some('R') => {
          chars.next();
          push_header_footer_section(&mut output, current, &mut buffer);
          current = HeaderFooterAlign::Right;
          continue;
        }
        _ => {}
      }
    }
    buffer.push(ch);
  }
  push_header_footer_section(&mut output, current, &mut buffer);
  output
}

fn push_header_footer_section(
  output: &mut Vec<(HeaderFooterAlign, String)>,
  align: HeaderFooterAlign,
  buffer: &mut String,
) {
  if !buffer.is_empty() {
    output.push((align, buffer.trim().to_string()));
    buffer.clear();
  }
}

#[derive(Clone, Copy, Debug)]
struct HeaderFooterFieldValues<'a> {
  page_number: usize,
  total_pages: usize,
  sheet_name: &'a str,
}

#[derive(Clone, Debug)]
struct HeaderFooterTextRun {
  text: String,
  style: TextStyle,
}

fn parse_header_footer_runs(
  text: &str,
  mut style: TextStyle,
  fields: HeaderFooterFieldValues<'_>,
) -> Vec<HeaderFooterTextRun> {
  // ECMA-376 18.3.1.46 stores formatted header/footer text in one control
  // string. This mirrors LibreOffice HeaderFooterParser: style state changes
  // flush the current range, while fields inherit the active font model.
  let mut runs = Vec::new();
  let mut output = String::new();
  let mut chars = text.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch != '&' {
      output.push(ch);
      continue;
    }
    match chars.next() {
      Some('P' | 'p') => output.push_str(&fields.page_number.to_string()),
      Some('N' | 'n') => output.push_str(&fields.total_pages.to_string()),
      Some('A' | 'a') => output.push_str(fields.sheet_name),
      Some('&') => output.push('&'),
      Some('L' | 'l' | 'C' | 'c' | 'R' | 'r') => {}
      Some('"') => {
        let mut descriptor = String::new();
        for next in chars.by_ref() {
          if next == '"' {
            break;
          }
          descriptor.push(next);
        }
        push_header_footer_run(&mut runs, &mut output, &style);
        let (font_name, font_style) = descriptor
          .split_once(',')
          .unwrap_or((descriptor.as_str(), ""));
        if !font_name.is_empty() && font_name != "-" {
          style.font_family = Some(Arc::from(font_name));
        }
        style.bold = false;
        style.italic = false;
        for name in font_style.split_ascii_whitespace() {
          if header_footer_bold_style(name) {
            style.bold = true;
          } else if header_footer_italic_style(name) {
            style.italic = true;
          }
        }
      }
      Some('K' | 'k') => {
        let color = chars.by_ref().take(6).collect::<String>();
        if color.len() == 6 && color.chars().all(|value| value.is_ascii_hexdigit()) {
          push_header_footer_run(&mut runs, &mut output, &style);
          if let Ok(rgb) = u32::from_str_radix(&color, 16) {
            style.color = RgbColor {
              r: (rgb >> 16) as u8,
              g: (rgb >> 8) as u8,
              b: rgb as u8,
            };
          }
        }
      }
      Some(ch) if ch.is_ascii_digit() => {
        let mut size = ch.to_digit(10).unwrap_or_default();
        while let Some(next) = chars.peek().and_then(|next| next.to_digit(10)) {
          chars.next();
          size = size.saturating_mul(10).saturating_add(next);
        }
        if size > 0 && size <= 1000 {
          push_header_footer_run(&mut runs, &mut output, &style);
          style.font_size_pt = size as f32;
        }
      }
      Some('B' | 'b') => {
        push_header_footer_run(&mut runs, &mut output, &style);
        style.bold = !style.bold;
      }
      Some('I' | 'i') => {
        push_header_footer_run(&mut runs, &mut output, &style);
        style.italic = !style.italic;
      }
      Some('U' | 'u' | 'E' | 'e') => {
        push_header_footer_run(&mut runs, &mut output, &style);
        style.underline = !style.underline;
      }
      Some('S' | 's') => {
        push_header_footer_run(&mut runs, &mut output, &style);
        style.strikethrough = !style.strikethrough;
      }
      Some(ch) => output.push(ch),
      None => output.push('&'),
    }
  }
  push_header_footer_run(&mut runs, &mut output, &style);
  runs
}

fn push_header_footer_run(
  runs: &mut Vec<HeaderFooterTextRun>,
  output: &mut String,
  style: &TextStyle,
) {
  if output.is_empty() {
    return;
  }
  runs.push(HeaderFooterTextRun {
    text: std::mem::take(output),
    style: style.clone(),
  });
}

fn header_footer_bold_style(name: &str) -> bool {
  matches!(
    name.to_ascii_lowercase().as_str(),
    "bold" | "fett" | "demibold" | "halbfett" | "black" | "heavy" | "félkövér"
  )
}

fn header_footer_italic_style(name: &str) -> bool {
  matches!(
    name.to_ascii_lowercase().as_str(),
    "italic" | "kursiv" | "oblique" | "schräg" | "dőlt"
  )
}

#[cfg(test)]
mod drawing_page_tests {
  use super::*;

  #[test]
  fn drawing_page_intersection_requires_positive_area_overlap() {
    let page = CellRect {
      x_pt: 100.0,
      y_pt: 200.0,
      width_pt: 300.0,
      height_pt: 400.0,
    };

    assert!(tuple_rect_intersects_cell_rect(
      (50.0, 250.0, 100.0, 100.0),
      page
    ));
    assert!(!tuple_rect_intersects_cell_rect(
      (0.0, 250.0, 100.0, 100.0),
      page
    ));
    assert!(!tuple_rect_intersects_cell_rect(
      (400.0, 250.0, 100.0, 100.0),
      page
    ));
  }
}

#[cfg(test)]
mod cell_alignment_tests {
  use super::*;

  fn print_cell(
    state: super::super::print::NumberFormatRenderState,
  ) -> super::super::print::CalcPrintCell<'static> {
    super::super::print::CalcPrintCell {
      address: CellAddress { col: 1, row: 1 },
      text: std::borrow::Cow::Borrowed("value"),
      style_index: None,
      pivot_format_id: None,
      rendered_text: "value".to_string(),
      rich_text_runs: &[],
      number_format_state: state,
      formula: false,
    }
  }

  #[test]
  fn general_alignment_centers_boolean_and_error_values() {
    for state in [
      super::super::print::NumberFormatRenderState::Boolean,
      super::super::print::NumberFormatRenderState::Error,
    ] {
      assert_eq!(
        calc_cell_horizontal_alignment(&print_cell(state), None),
        x::HorizontalAlignmentValues::Center
      );
    }
  }

  #[test]
  fn overflow_hashes_fill_the_available_cell_width() {
    assert_eq!(calc_cell_overflow_hash_count(90.0, 6.0), 15);
    assert_eq!(calc_cell_overflow_hash_count(5.0, 6.0), 1);
  }
}

#[cfg(test)]
mod header_footer_tests {
  use super::*;

  #[test]
  fn header_footer_font_descriptor_and_size_apply_to_fields() {
    let runs = parse_header_footer_runs(
      "&\"Times New Roman,Regular\"&12&A",
      TextStyle::default(),
      HeaderFooterFieldValues {
        page_number: 2,
        total_pages: 3,
        sheet_name: "Sheet1",
      },
    );

    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].text, "Sheet1");
    assert_eq!(
      runs[0].style.font_family.as_deref(),
      Some("Times New Roman")
    );
    assert_eq!(runs[0].style.font_size_pt, 12.0);
  }

  #[test]
  fn header_footer_style_changes_create_separate_runs() {
    let runs = parse_header_footer_runs(
      "plain&Bbold&B&P/&N",
      TextStyle::default(),
      HeaderFooterFieldValues {
        page_number: 2,
        total_pages: 3,
        sheet_name: "Sheet1",
      },
    );

    assert_eq!(runs.len(), 3);
    assert_eq!(runs[0].text, "plain");
    assert!(!runs[0].style.bold);
    assert_eq!(runs[1].text, "bold");
    assert!(runs[1].style.bold);
    assert_eq!(runs[2].text, "2/3");
    assert!(!runs[2].style.bold);
  }
}
