use std::borrow::Cow;
use std::collections::HashMap;

use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2008_diagram as dsp;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::compat::{
  self as layout, ImageItem, LayoutDocument, LineItem, LineItemKind, LinkAreaItem, PageItem,
  PdfTextSegmentation, RectItem, TextItem,
};
use crate::compat::{BorderStyle, ImageCrop, PageSetup, RgbColor, TextStyle};
use crate::render::{diagram as shared_diagram, emf_wmf};
use crate::text_metrics::TextMetrics;
use crate::units;

use super::import::ExcelImport;
use super::print::{CalcPrintDocument, CalcPrintPage};
use super::worksheet::{CalcSheet, CellAddress, CellRange, CellRect};

const XLSX_HEADER_FOOTER_LINE_HEIGHT_PT: f32 = 12.0;
// margins are 20 twips on each side.
const XLSX_CELL_TEXT_INSET_PT: f32 = 20.0 / crate::units::TWIPS_PER_POINT;
const XLSX_GRID_LINE_WIDTH_PT: f32 = 0.25;

#[derive(Clone, Debug)]
struct CalcCellTextFragment {
  address: CellAddress,
  rect: CellRect,
  text: TextItem,
}

#[derive(Clone, Copy, Debug)]
struct CalcCellTextOrigin {
  address: CellAddress,
  rect: CellRect,
}

impl From<&CalcCellTextFragment> for CalcCellTextOrigin {
  fn from(fragment: &CalcCellTextFragment) -> Self {
    Self {
      address: fragment.address,
      rect: fragment.rect,
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct CalcCellOutputArea {
  align_rect: CellRect,
  clip_rect: CellRect,
  left_clip_pt: f32,
  right_clip_pt: f32,
}

pub(crate) fn lower_to_layout_document(import: &ExcelImport) -> LayoutDocument {
  let mut pages = Vec::new();
  let print_document = CalcPrintDocument::from_import(import);
  pages.extend(print_document.pages.iter().map(|page| {
    let setup = page_setup_from_calc(page);
    (setup, print_page_items(import, page, setup))
  }));
  layout::item_pages(pages)
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
    page,
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
  render_header_footer(&mut items, page, setup);
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
  let mut cell_text_items = Vec::new();
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
    if let Some(fill_color) = conditional_fill_color(import, page.sheet, cell)
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
    // sc/source/ui/view/output2.cxx ScDrawStringsVars::SetPattern(). Calc's
    // print map mode scales cell geometry and the font used for measurement.
    measurement_style.font_size_pt *= layout.zoom_scale;
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
    cell_text_items.extend(rendered_text_items.into_iter().filter_map(|item| {
      let PageItem::Text(text) = item else {
        return None;
      };
      Some(CalcCellTextFragment {
        address: cell.address,
        rect: cell_rect,
        text,
      })
    }));
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
  items.extend(coalesced_calc_row_text_items(cell_text_items, text_metrics));
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
      if calc_cell_is_value(cell) {
        x::HorizontalAlignmentValues::Right
      } else {
        x::HorizontalAlignmentValues::Left
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
      std::borrow::Cow::Borrowed("###")
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

fn coalesced_calc_row_text_items(
  items: Vec<CalcCellTextFragment>,
  text_metrics: &mut TextMetrics,
) -> Vec<PageItem> {
  let mut output = Vec::with_capacity(items.len());
  let mut previous_source: Option<CalcCellTextOrigin> = None;
  let mut items = items.into_iter().peekable();
  while let Some(item) = items.next() {
    let next_is_row_end = !matches!(
      items.peek(),
      Some(following) if (following.text.y_pt - item.text.y_pt).abs() < 0.01
    );
    if let Some(PageItem::Text(previous)) = output.last_mut()
      && let Some(source) = previous_source
      && calc_row_text_items_coalesce(previous, &item.text)
    {
      let spaces =
        calc_row_inter_cell_spaces(source, previous, &item, next_is_row_end, text_metrics);
      previous.text.extend(std::iter::repeat_n(' ', spaces));
      previous.text.push_str(&item.text.text);
      previous.line_height_pt = previous.line_height_pt.max(item.text.line_height_pt);
      previous_source = Some(CalcCellTextOrigin::from(&item));
      continue;
    }
    previous_source = Some(CalcCellTextOrigin::from(&item));
    output.push(PageItem::Text(item.text));
  }
  output
}

fn calc_row_text_items_coalesce(current: &TextItem, next: &TextItem) -> bool {
  if current.form_widget_id.is_some()
    || next.form_widget_id.is_some()
    || current.hyperlink_url != next.hyperlink_url
    || current.dynamic_field != next.dynamic_field
    || current.paragraph_bidi != next.paragraph_bidi
    || current.rotation_center_pt.is_some()
    || next.rotation_center_pt.is_some()
    || current.decoration_span_start_x_pt != next.decoration_span_start_x_pt
    || current.pdf_text_segmentation != PdfTextSegmentation::Line
    || next.pdf_text_segmentation != PdfTextSegmentation::Line
    || current.preserve_text_portion
    || next.preserve_text_portion
    || (current.y_pt - next.y_pt).abs() >= 0.01
    || (current.line_height_pt - next.line_height_pt).abs() >= 0.01
  {
    return false;
  }
  next.x_pt > current.x_pt
}

fn calc_row_inter_cell_spaces(
  current: CalcCellTextOrigin,
  current_text: &TextItem,
  next: &CalcCellTextFragment,
  next_is_row_end: bool,
  text_metrics: &mut TextMetrics,
) -> usize {
  let next_text = &next.text;
  if current_text.text.ends_with(char::is_whitespace)
    || next_text.text.starts_with(char::is_whitespace)
  {
    return 0;
  }
  if calc_text_number_like(&current_text.text) && calc_text_number_like(&next_text.text) {
    return 1;
  }
  let current_right =
    current_text.x_pt + text_metrics.measure_text(&current_text.text, &current_text.style);
  let current_layout_right = current_text.x_pt
    + approximate_text_width_pt(&current_text.text, current_text.style.font_size_pt);
  let current_cell_right = current.rect.x_pt + current.rect.width_pt;
  let gap = next_text.x_pt - current_right;
  let space_width = text_metrics
    .measure_text(" ", &current_text.style)
    .max(current_text.style.font_size_pt * 0.25)
    .max(1.0);
  if current.address.row == next.address.row
    && current.address.col + 1 == next.address.col
    && gap < space_width * 0.5
  {
    return 0;
  }
  // replaces clipped numeric output with "###", then SetClipMarks() only
  // adjusts the clip region. Keep PDF text extraction joined when the actual
  // glyph positions are closer than a printable word gap.
  if next_text.text.chars().all(|ch| ch == '#')
    && current.address.row == next.address.row
    && current.address.col + 1 == next.address.col
    && gap < space_width * 1.5
  {
    return 0;
  }
  if current.address.row == next.address.row
    && current.address.col != next.address.col
    && (current_text.text.contains(char::is_whitespace)
      || next_text.text.contains(char::is_whitespace))
  {
    return 1;
  }
  if current.address.row == next.address.row
    && current.address.col != next.address.col
    && current_layout_right <= current_cell_right + space_width * 0.25
  {
    return 1;
  }
  if gap <= 0.0 {
    return usize::from(next_is_row_end);
  }
  if gap < space_width * 1.5 {
    1
  } else {
    let spaces = (gap / space_width).round() as usize;
    if next_is_row_end {
      spaces.max(1)
    } else {
      spaces
    }
  }
}

fn calc_text_number_like(text: &str) -> bool {
  let trimmed = text.trim();
  !trimmed.is_empty()
    && trimmed
      .chars()
      .all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | ',' | '-' | '+' | '%' | '/' | ':'))
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
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion,
    decoration_span_start_x_pt: None,
    pdf_text_segmentation: if preserve_text_portion {
      PdfTextSegmentation::Portion
    } else {
      PdfTextSegmentation::Line
    },
  }));
}

fn approximate_text_width_pt(text: &str, font_size_pt: f32) -> f32 {
  text
    .chars()
    .map(|ch| if ch.is_ascii_whitespace() { 0.33 } else { 0.55 })
    .sum::<f32>()
    * font_size_pt
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
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion,
      decoration_span_start_x_pt: None,
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
  PageItem::Text(TextItem {
    x_pt,
    y_pt,
    line_height_pt: XLSX_HEADER_FOOTER_LINE_HEIGHT_PT,
    text,
    style: TextStyle::default(),
    rotation_center_pt: None,
    hyperlink_url: None,
    dynamic_field: None,
    style_ref_keys: Vec::new(),
    style_ref_text: None,
    form_widget_id: None,
    paragraph_bidi: false,
    preserve_text_portion: false,
    decoration_span_start_x_pt: None,
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
      let hyperlink_url = drawing_object_hyperlink_url(drawing, &anchor.object);
      render_drawing_text(
        &mut items,
        &text,
        CellRect {
          x_pt: origin_x_pt + x_pt * zoom_scale,
          y_pt: origin_y_pt + y_pt * zoom_scale,
          width_pt: width_pt * zoom_scale,
          height_pt: height_pt * zoom_scale,
        },
        drawing_object_text_style(&anchor.object),
        hyperlink_url.as_deref(),
      );
    }
  }
  items
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
  hyperlink_url: Option<&str>,
) {
  let style = style.unwrap_or_default();
  let line_height = (style.font_size_pt * 1.15).max(1.0);
  for (index, line) in text.lines().filter(|line| !line.is_empty()).enumerate() {
    let y = rect.y_pt + XLSX_CELL_TEXT_INSET_PT + index as f32 * line_height;
    if y > rect.y_pt + rect.height_pt {
      break;
    }
    items.push(PageItem::Text(TextItem {
      x_pt: rect.x_pt + XLSX_CELL_TEXT_INSET_PT,
      y_pt: y,
      line_height_pt: line_height,
      text: line.to_string(),
      style: style.clone(),
      rotation_center_pt: None,
      hyperlink_url: hyperlink_url.map(ToString::to_string),
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: false,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: PdfTextSegmentation::Line,
    }));
  }
}

fn drawing_object_text_style(object: &super::drawing::DrawingObjectModel) -> Option<TextStyle> {
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
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: false,
      decoration_span_start_x_pt: None,
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

fn render_header_footer(items: &mut Vec<PageItem>, page: &CalcPrintPage<'_>, setup: PageSetup) {
  let header = header_footer_text(page, true);
  let footer = header_footer_text(page, false);
  if let Some(header) = header {
    render_header_footer_line(
      items,
      setup.margin_left_pt,
      setup.header_distance_pt,
      page,
      setup,
      header,
    );
  }
  if let Some(footer) = footer {
    render_header_footer_line(
      items,
      setup.margin_left_pt,
      setup.height_pt - setup.footer_distance_pt - XLSX_HEADER_FOOTER_LINE_HEIGHT_PT,
      page,
      setup,
      footer,
    );
  }
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
  x_pt: f32,
  y_pt: f32,
  page: &CalcPrintPage<'_>,
  setup: PageSetup,
  text: &str,
) {
  for (align, value) in split_header_footer_sections(text) {
    if value.is_empty() {
      continue;
    }
    let value = expand_header_footer_tokens(&value, page, align);
    if value.is_empty() {
      continue;
    }
    let x = match align {
      HeaderFooterAlign::Left => x_pt,
      HeaderFooterAlign::Center => setup.width_pt / 2.0,
      HeaderFooterAlign::Right => setup.width_pt - setup.margin_right_pt,
    };
    items.push(header_text(x, y_pt, value));
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

fn expand_header_footer_tokens(
  text: &str,
  page: &CalcPrintPage<'_>,
  align: HeaderFooterAlign,
) -> String {
  let mut output = String::new();
  let mut chars = text.chars().peekable();
  while let Some(ch) = chars.next() {
    if ch != '&' {
      output.push(ch);
      continue;
    }
    match chars.next() {
      Some('P') => output.push_str(&page.page_number.to_string()),
      Some('N') => output.push_str(&page.total_pages.to_string()),
      Some('A') => output.push_str(&page.sheet.name),
      Some('&') => output.push('&'),
      Some('L' | 'C' | 'R') => {}
      Some('"') => {
        for next in chars.by_ref() {
          if next == '"' {
            break;
          }
        }
        if align == HeaderFooterAlign::Left && !output.ends_with(' ') && !output.is_empty() {
          output.push(' ');
        }
      }
      Some('K') => {
        for _ in 0..6 {
          if !chars.peek().is_some_and(|peek| peek.is_ascii_hexdigit()) {
            break;
          }
          chars.next();
        }
      }
      Some(ch) if ch.is_ascii_digit() => {
        while chars.peek().is_some_and(|peek| peek.is_ascii_digit()) {
          chars.next();
        }
        if align == HeaderFooterAlign::Left && !output.ends_with(' ') && !output.is_empty() {
          output.push(' ');
        }
      }
      Some(ch) => output.push(ch),
      None => output.push('&'),
    }
  }
  output
}
