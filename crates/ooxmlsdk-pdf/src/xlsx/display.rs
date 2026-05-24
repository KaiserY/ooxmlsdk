use std::collections::HashMap;

use ooxmlsdk::schemas::schemas_microsoft_com_office_drawing_2008_diagram as dsp;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_diagram as dgm;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::docx::{BorderStyle, ImageCrop, PageSetup, RgbColor, TextStyle};
use crate::layout::{
  self, ImageItem, LayoutDocument, LineItem, LineItemKind, LinkAreaItem, PageItem,
  PdfTextSegmentation, RectItem, TextItem,
};
use crate::render::{diagram as shared_diagram, emf_wmf};
use crate::units;

use super::import::ExcelImport;
use super::print::{CalcPrintDocument, CalcPrintPage};
use super::worksheet::{CalcSheet, CellAddress, CellRange, CellRect, SheetType};

const XLSX_HEADER_FOOTER_LINE_HEIGHT_PT: f32 = 12.0;
const XLSX_CELL_TEXT_INSET_PT: f32 = 2.0;
const XLSX_GRID_LINE_WIDTH_PT: f32 = 0.25;

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
      body_origin_x,
      body_origin_y,
      zoom_scale,
    );
  }
  if let Some(area) = repeat_rows_for_page(page) {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.repeated_row_cells,
      area,
      body_origin_x + repeat_width,
      body_origin_y,
      zoom_scale,
    );
  }
  if let Some(area) = repeat_columns_for_page(page) {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.repeated_column_cells,
      area,
      body_origin_x,
      body_origin_y + repeat_height,
      zoom_scale,
    );
  }
  if let Some(area) = page.area {
    render_cell_area(
      &mut items,
      import,
      page,
      &page.cells,
      area,
      body_origin_x + repeat_width,
      body_origin_y + repeat_height,
      zoom_scale,
    );
    if page.page_settings.print_headings {
      render_headings(
        &mut items,
        page,
        area,
        setup.margin_left_pt,
        body_origin_y + repeat_height,
        body_origin_x + repeat_width,
        body_origin_y - heading_height,
        zoom_scale,
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

fn render_cell_area(
  items: &mut Vec<PageItem>,
  import: &ExcelImport,
  page: &CalcPrintPage<'_>,
  cells: &[super::print::CalcPrintCell<'_>],
  area: super::worksheet::CellRange,
  origin_x_pt: f32,
  origin_y_pt: f32,
  zoom_scale: f32,
) {
  let area_rect = page.sheet.range_rect(area);
  for cell in cells {
    if page.sheet.is_covered_merged_cell(cell.address) {
      continue;
    }
    let rect = page.sheet.cell_rect(cell.address);
    if rect.width_pt <= 0.0 || rect.height_pt <= 0.0 {
      continue;
    }
    let x_pt = origin_x_pt + (rect.x_pt - area_rect.x_pt) * zoom_scale;
    let y_pt = origin_y_pt + (rect.y_pt - area_rect.y_pt) * zoom_scale;
    let width_pt = rect.width_pt * zoom_scale;
    let height_pt = rect.height_pt * zoom_scale;
    if let Some(fill_color) = conditional_fill_color(import, page.sheet, cell)
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
    render_cell_borders(
      items,
      CellRect {
        x_pt,
        y_pt,
        width_pt,
        height_pt,
      },
      import.styles.borders_for_cell(cell.style_index),
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
    let base_style = import.styles.text_style_for_cell(cell.style_index);
    if !cell.rich_text_runs.is_empty() && cell.rendered_text == cell.text {
      render_cell_rich_text(
        items,
        cell.rich_text_runs,
        cell_rect,
        base_style,
        hyperlink_url.clone(),
      );
    } else {
      render_cell_text(
        items,
        &cell.rendered_text,
        cell_rect,
        base_style,
        import.styles.alignment_for_cell(cell.style_index),
        hyperlink_url.clone(),
      );
    }
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
    render_grid(items, page, area, origin_x_pt, origin_y_pt, zoom_scale);
  }
}

fn render_cell_rich_text(
  items: &mut Vec<PageItem>,
  runs: &[super::workbook::SharedStringRun],
  rect: CellRect,
  base_style: TextStyle,
  hyperlink_url: Option<String>,
) {
  let mut x_pt = rect.x_pt + XLSX_CELL_TEXT_INSET_PT;
  let y_pt = rect.y_pt + XLSX_CELL_TEXT_INSET_PT;
  for run in runs.iter().filter(|run| !run.text.is_empty()) {
    let mut style = base_style.clone();
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
    let line_height = (style.font_size_pt * 1.15).max(1.0);
    items.push(PageItem::Text(TextItem {
      x_pt,
      y_pt,
      line_height_pt: line_height,
      text: run.text.clone(),
      style: style.clone(),
      rotation_center_pt: None,
      hyperlink_url: hyperlink_url.clone(),
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      form_widget_id: None,
      paragraph_bidi: false,
      preserve_text_portion: true,
      decoration_span_start_x_pt: None,
      pdf_text_segmentation: PdfTextSegmentation::Portion,
    }));
    x_pt += approximate_text_width_pt(&run.text, style.font_size_pt);
    if x_pt > rect.x_pt + rect.width_pt {
      break;
    }
  }
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
    x::ConditionalFormatValues::ContainsText => rule
      .text
      .as_ref()
      .is_some_and(|needle| cell.rendered_text.contains(needle) || cell.text.contains(needle)),
    x::ConditionalFormatValues::NotContainsText => rule
      .text
      .as_ref()
      .is_some_and(|needle| !cell.rendered_text.contains(needle) && !cell.text.contains(needle)),
    x::ConditionalFormatValues::BeginsWith => rule.text.as_ref().is_some_and(|needle| {
      cell.rendered_text.starts_with(needle) || cell.text.starts_with(needle)
    }),
    x::ConditionalFormatValues::EndsWith => rule
      .text
      .as_ref()
      .is_some_and(|needle| cell.rendered_text.ends_with(needle) || cell.text.ends_with(needle)),
    x::ConditionalFormatValues::ContainsBlanks => {
      cell.text.is_empty() && cell.rendered_text.is_empty()
    }
    x::ConditionalFormatValues::NotContainsBlanks => {
      !cell.text.is_empty() || !cell.rendered_text.is_empty()
    }
    x::ConditionalFormatValues::Expression => expression_rule_matches(rule),
    _ => false,
  }
}

fn conditional_cell_is_matches(
  rule: &super::sheet_conditions::ConditionalFormatRuleModel,
  cell: &super::print::CalcPrintCell<'_>,
) -> bool {
  let Some(value) = cell.text.parse::<f64>().ok() else {
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

fn render_cell_text(
  items: &mut Vec<PageItem>,
  text: &str,
  rect: CellRect,
  mut style: TextStyle,
  alignment: Option<super::styles::AlignmentRecord>,
  hyperlink_url: Option<String>,
) {
  let line_height = (style.font_size_pt * 1.15).max(1.0);
  let lines = if alignment.is_some_and(|alignment| alignment.wrap_text) {
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
    let preserve_text_portion = line.chars().any(|ch| !ch.is_ascii());
    items.push(PageItem::Text(TextItem {
      x_pt: rect.x_pt + XLSX_CELL_TEXT_INSET_PT,
      y_pt,
      line_height_pt: line_height,
      text: line.to_string(),
      style: style.clone(),
      rotation_center_pt: (style.rotation_deg != 0.0).then_some((
        rect.x_pt + rect.width_pt / 2.0,
        rect.y_pt + rect.height_pt / 2.0,
      )),
      hyperlink_url: hyperlink_url.clone(),
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

fn render_headings(
  items: &mut Vec<PageItem>,
  page: &CalcPrintPage<'_>,
  area: super::worksheet::CellRange,
  row_header_x_pt: f32,
  row_header_y_pt: f32,
  col_header_x_pt: f32,
  col_header_y_pt: f32,
  zoom_scale: f32,
) {
  let mut x = col_header_x_pt;
  for col in area.start.col..=area.end.col {
    let width = page.sheet.column_width_pt(col) * zoom_scale;
    items.push(header_text(
      x + XLSX_CELL_TEXT_INSET_PT,
      col_header_y_pt,
      column_label(col),
    ));
    x += width;
  }
  let mut y = row_header_y_pt;
  for row in area.start.row..=area.end.row {
    let height = page.sheet.row_height_pt(row) * zoom_scale;
    items.push(header_text(
      row_header_x_pt + XLSX_CELL_TEXT_INSET_PT,
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
        hyperlink_url: drawing_object_hyperlink_url(drawing, &anchor.object),
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
        drawing_object_hyperlink_url(drawing, &anchor.object),
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
    let styles = diagram_styles(&drawing.diagrams);
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
        styles.as_ref(),
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
    text_x,
    text_y,
    text_width,
    text_height,
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

fn diagram_styles(
  diagrams: &super::drawing::DiagramResourceCatalog,
) -> Option<shared_diagram::DiagramStyles> {
  let style_by_label: HashMap<String, _> = diagrams
    .style_parts
    .iter()
    .filter_map(|part| part.style.as_deref())
    .flat_map(|style| style.style_label.iter())
    .filter_map(|label| Some((label.name.clone(), label.style.clone()?)))
    .collect();
  (!style_by_label.is_empty()).then_some(shared_diagram::DiagramStyles { style_by_label })
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
      shape.x,
      shape.y,
      shape.width,
      shape.height,
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
      render_drawing_text(
        &mut items,
        &text,
        origin_x_pt + x_pt * zoom_scale,
        origin_y_pt + y_pt * zoom_scale,
        width_pt * zoom_scale,
        height_pt * zoom_scale,
        drawing_object_text_style(&anchor.object),
        drawing_object_hyperlink_url(drawing, &anchor.object),
      );
    }
  }
  items
}

fn drawing_anchor_text(
  drawing: &super::drawing::DrawingResourceCatalog,
  anchor: &super::drawing::DrawingAnchorModel,
) -> String {
  if !anchor.object.text.is_empty() {
    return anchor.object.text.clone();
  }
  if anchor.object.kind == super::drawing::DrawingObjectKind::GraphicFrame
    && let Some(relationship_id) = anchor.object.relationship_id.as_deref()
  {
    let chart_text = drawing
      .charts
      .iter()
      .chain(drawing.extended_charts.iter())
      .find(|chart| chart.relationship_id.as_deref() == Some(relationship_id))
      .map(|chart| chart.visible_texts.join("\n"))
      .unwrap_or_default();
    if !chart_text.is_empty() {
      return chart_text;
    }
    if drawing
      .diagrams
      .data_parts
      .iter()
      .any(|data| data.relationship_id.as_deref() == Some(relationship_id))
    {
      return String::new();
    }
  }
  String::new()
}

fn render_drawing_text(
  items: &mut Vec<PageItem>,
  text: &str,
  x_pt: f32,
  y_pt: f32,
  _width_pt: f32,
  height_pt: f32,
  style: Option<TextStyle>,
  hyperlink_url: Option<String>,
) {
  let style = style.unwrap_or_default();
  let line_height = (style.font_size_pt * 1.15).max(1.0);
  for (index, line) in text.lines().filter(|line| !line.is_empty()).enumerate() {
    let y = y_pt + XLSX_CELL_TEXT_INSET_PT + index as f32 * line_height;
    if y > y_pt + height_pt {
      break;
    }
    items.push(PageItem::Text(TextItem {
      x_pt: x_pt + XLSX_CELL_TEXT_INSET_PT,
      y_pt: y,
      line_height_pt: line_height,
      text: line.to_string(),
      style: style.clone(),
      rotation_center_pt: None,
      hyperlink_url: hyperlink_url.clone(),
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
  hyperlink_url: Option<String>,
) {
  let texts = emf_wmf::extract_metafile_texts(&resource.data, resource.content_type.as_deref());
  if texts.is_empty() {
    return;
  }
  render_drawing_text(
    items,
    &texts.join("\n"),
    x_pt,
    y_pt,
    width_pt,
    height_pt,
    None,
    hyperlink_url,
  );
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
    if shape.hidden || !shape.print_object || shape.text.trim().is_empty() {
      continue;
    }
    let Some((x_pt, y_pt, width_pt, height_pt)) = vml_shape_rect(page.sheet, shape) else {
      continue;
    };
    let (x_pt, y_pt) =
      page_area_rect.map_or((x_pt, y_pt), |rect| (x_pt - rect.x_pt, y_pt - rect.y_pt));
    render_drawing_text(
      &mut items,
      &shape.text,
      origin_x_pt + x_pt * zoom_scale,
      origin_y_pt + y_pt * zoom_scale,
      width_pt * zoom_scale,
      height_pt * zoom_scale,
      None,
      None,
    );
  }
  items
}

fn drawing_object_hyperlink_url(
  drawing: &super::drawing::DrawingResourceCatalog,
  object: &super::drawing::DrawingObjectModel,
) -> Option<String> {
  object
    .hyperlink_relationship_id
    .as_deref()
    .and_then(|relationship_id| drawing.hyperlink_targets.get(relationship_id))
    .cloned()
    .or_else(|| object.hyperlink_invalid_url.clone())
    .or_else(|| {
      object
        .hyperlink_action
        .clone()
        .and_then(drawing_hyperlink_action_url)
    })
}

fn drawing_hyperlink_action_url(action: String) -> Option<String> {
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
      &header,
    );
  }
  if let Some(footer) = footer {
    render_header_footer_line(
      items,
      setup.margin_left_pt,
      setup.height_pt - setup.footer_distance_pt - XLSX_HEADER_FOOTER_LINE_HEIGHT_PT,
      page,
      setup,
      &footer,
    );
  }
}

fn header_footer_text(page: &CalcPrintPage<'_>, header: bool) -> Option<String> {
  let model = &page.page_settings.header_footer;
  let text = if page.sheet_page_index == 0 && model.different_first {
    if header {
      model.first_header.as_ref()
    } else {
      model.first_footer.as_ref()
    }
  } else if page.sheet_page_index % 2 == 1 && model.different_odd_even {
    if header {
      model.even_header.as_ref()
    } else {
      model.even_footer.as_ref()
    }
  } else if header {
    model.odd_header.as_ref()
  } else {
    model.odd_footer.as_ref()
  }?;
  Some(expand_header_footer_tokens(text, page))
}

fn render_header_footer_line(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  _page: &CalcPrintPage<'_>,
  setup: PageSetup,
  text: &str,
) {
  for (align, value) in split_header_footer_sections(text) {
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

fn expand_header_footer_tokens(text: &str, page: &CalcPrintPage<'_>) -> String {
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
      }
      Some(ch) if ch.is_ascii_digit() => {
        while chars.peek().is_some_and(|peek| peek.is_ascii_digit()) {
          chars.next();
        }
      }
      Some(ch) => output.push(ch),
      None => output.push('&'),
    }
  }
  output
}

fn workbook_lines(import: &ExcelImport) -> Vec<String> {
  let mut lines = vec![format!(
    "Workbook sheets={} theme={} styles={} sharedStrings={} definedNames={} externalWorkbooks={} pivotCaches={} persons={}",
    import.workbook.sheets.sheet.len(),
    import.workbook_resources.has_theme,
    import.workbook_resources.has_styles,
    import.shared_strings.len(),
    import.defined_names.records.len(),
    import.workbook_resources.external_workbooks,
    import.workbook_resources.pivot_cache_definitions,
    import.workbook_resources.workbook_persons
  )];

  if !import.pivot_caches.caches.is_empty() {
    let cache_fields = import
      .pivot_caches
      .caches
      .iter()
      .map(|cache| cache.cache_fields)
      .sum::<usize>();
    let cache_records = import
      .pivot_caches
      .caches
      .iter()
      .filter_map(|cache| cache.record_count)
      .sum::<u32>();
    let cache_flags = import
      .pivot_caches
      .caches
      .iter()
      .filter(|cache| {
        cache.refresh_on_load
          || cache.save_data.is_some()
          || cache.invalid
          || cache.has_records_part
          || cache.has_cache_source
          || cache.has_extensions
      })
      .count();
    let cache_ref_len = import
      .pivot_caches
      .caches
      .iter()
      .map(|cache| {
        cache
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + cache.workbook_cache_id.unwrap_or_default() as usize
          + cache
            .workbook_relationship_id
            .as_ref()
            .map_or(0, |value| value.len())
          + cache
            .definition_relationship_id
            .as_ref()
            .map_or(0, |value| value.len())
          + cache.optional_child_count
      })
      .sum::<usize>();
    lines.push(format!(
      "pivotCaches caches={} fields={} records={} flags={} refLen={}",
      import.pivot_caches.caches.len(),
      cache_fields,
      cache_records,
      cache_flags,
      cache_ref_len
    ));
  }

  if !import.connections.connections.is_empty() {
    let connection_flags = import
      .connections
      .connections
      .iter()
      .filter(|connection| {
        connection.refresh_on_load
          || connection.save_data.is_some()
          || connection.has_database_properties
          || connection.has_olap_properties
          || connection.has_web_query_properties
          || connection.has_text_properties
          || connection.has_extensions
      })
      .count();
    let parameter_count = import
      .connections
      .connections
      .iter()
      .map(|connection| connection.parameter_count)
      .sum::<usize>();
    let connection_ref_len = import
      .connections
      .connections
      .iter()
      .map(|connection| {
        connection.id as usize
          + connection.name.as_ref().map_or(0, |value| value.len())
          + connection.connection_type.unwrap_or_default() as usize
          + connection
            .source_file
            .as_ref()
            .map_or(0, |value| value.len())
          + connection
            .connection_file
            .as_ref()
            .map_or(0, |value| value.len())
          + connection.refreshed_version as usize
      })
      .sum::<usize>();
    lines.push(format!(
      "connections rel={} count={} flags={} parameters={} refLen={}",
      import.connections.relationship_id.as_deref().unwrap_or(""),
      import.connections.connections.len(),
      connection_flags,
      parameter_count,
      connection_ref_len
    ));
  }

  if !import.workbook_catalog.external_links.is_empty() {
    let external_book_links = import
      .workbook_catalog
      .external_links
      .iter()
      .filter(|link| {
        matches!(
          link.kind,
          super::workbook_catalog::ExternalLinkKind::ExternalBook { .. }
        )
      })
      .count();
    let dde_links = import
      .workbook_catalog
      .external_links
      .iter()
      .filter(|link| {
        matches!(
          link.kind,
          super::workbook_catalog::ExternalLinkKind::Dde { .. }
        )
      })
      .count();
    let ole_links = import
      .workbook_catalog
      .external_links
      .iter()
      .filter(|link| {
        matches!(
          link.kind,
          super::workbook_catalog::ExternalLinkKind::Ole { .. }
        )
      })
      .count();
    let external_items = import
      .workbook_catalog
      .external_links
      .iter()
      .map(|link| {
        link.sheet_names
          + link.defined_names
          + link.cached_sheet_data
          + link.item_count
          + usize::from(link.has_extensions)
          + link.relationship_id.as_ref().map_or(0, |value| value.len())
          + external_link_kind_len(&link.kind)
      })
      .sum::<usize>();
    lines.push(format!(
      "externalLinks links={} books={} dde={} ole={} items={}",
      import.workbook_catalog.external_links.len(),
      external_book_links,
      dde_links,
      ole_links,
      external_items
    ));
  }

  if let Some(xml_maps) = &import.workbook_catalog.xml_maps {
    lines.push(format!(
      "xmlMaps rel={} schemas={} maps={} schemaRefs={} mapFlags={} namespaceLen={}",
      xml_maps.relationship_id.as_deref().unwrap_or(""),
      xml_maps.schemas,
      xml_maps.maps,
      xml_maps.schema_ref_count,
      xml_maps.map_flag_count,
      xml_maps.selection_namespaces.len()
    ));
  }

  if !import.workbook_catalog.persons.is_empty() {
    let person_count = import
      .workbook_catalog
      .persons
      .iter()
      .map(|part| part.persons)
      .sum::<usize>();
    let person_text_len = import
      .workbook_catalog
      .persons
      .iter()
      .map(|part| {
        part.id_text_len
          + part.relationship_id.as_ref().map_or(0, |value| value.len())
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    lines.push(format!(
      "persons parts={} persons={} textLen={}",
      import.workbook_catalog.persons.len(),
      person_count,
      person_text_len
    ));
  }

  if let Some(revisions) = &import.workbook_catalog.revisions {
    lines.push(format!(
      "revisions rel={} headers={} logs={} flags={} textLen={} guidLen={}",
      revisions.relationship_id.as_deref().unwrap_or(""),
      revisions.headers,
      revisions.revision_logs,
      revisions.flag_count,
      revisions.user_text_len,
      revisions.guid.len() + revisions.last_guid.as_ref().map_or(0, |value| value.len())
    ));
  }

  let relationship_resources = &import.workbook_catalog.relationship_resources;
  let relationship_resource_count = relationship_resources.custom_xml_parts.len()
    + relationship_resources.custom_data_properties.len()
    + relationship_resources.slicer_caches.len()
    + relationship_resources.timeline_caches.len()
    + relationship_resources.rich_values.len()
    + relationship_resources.rich_value_structures.len()
    + relationship_resources.arrays.len()
    + relationship_resources.rich_styles.len()
    + relationship_resources.supporting_property_bags.len()
    + relationship_resources
      .supporting_property_bag_structures
      .len()
    + relationship_resources.rich_value_types.len()
    + usize::from(relationship_resources.rich_value_web_image.is_some())
    + usize::from(relationship_resources.feature_property_bags.is_some())
    + usize::from(relationship_resources.has_vba_project)
    + usize::from(relationship_resources.has_attached_toolbars)
    + usize::from(relationship_resources.user_data.is_some())
    + usize::from(relationship_resources.calculation_chain.is_some())
    + usize::from(relationship_resources.cell_metadata.is_some())
    + usize::from(relationship_resources.volatile_dependencies.is_some());
  if relationship_resource_count > 0 {
    let custom_xml_flags = relationship_resources
      .custom_xml_parts
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + usize::from(part.has_properties)
          + part.schema_refs
          + part.text_len
      })
      .sum::<usize>();
    let custom_data_flags = relationship_resources
      .custom_data_properties
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + usize::from(part.has_custom_data)
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let slicer_flags = relationship_resources
      .slicer_caches
      .iter()
      .map(|cache| {
        usize::from(cache.relationship_id.is_some())
          + cache.name_len
          + cache.source_name_len
          + cache.pivot_tables
          + usize::from(cache.has_data)
          + cache.extension_markers
      })
      .sum::<usize>();
    let timeline_flags = relationship_resources
      .timeline_caches
      .iter()
      .map(|cache| {
        usize::from(cache.relationship_id.is_some())
          + cache.name_len
          + cache.source_name_len
          + cache.pivot_tables
          + cache.state_flags
          + cache.text_len
          + cache.extension_markers
      })
      .sum::<usize>();
    let rich_value_items = relationship_resources
      .rich_values
      .iter()
      .map(|part| part.rich_values + part.values + part.fallbacks)
      .sum::<usize>();
    let rich_value_flags = relationship_resources
      .rich_values
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let rich_structure_items = relationship_resources
      .rich_value_structures
      .iter()
      .map(|part| part.structures + part.keys)
      .sum::<usize>();
    let rich_structure_flags = relationship_resources
      .rich_value_structures
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let array_items = relationship_resources
      .arrays
      .iter()
      .map(|part| part.arrays + part.values)
      .sum::<usize>();
    let array_flags = relationship_resources
      .arrays
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let rich_style_flags = relationship_resources
      .rich_styles
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + usize::from(part.has_dxfs)
          + usize::from(part.has_properties)
          + part.styles
          + part.style_values
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let property_bag_flags = relationship_resources
      .supporting_property_bags
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.arrays
          + part.bags
          + part.values
          + part.extension_markers
      })
      .sum::<usize>();
    let property_bag_structure_flags = relationship_resources
      .supporting_property_bag_structures
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.structures
          + part.keys
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let rich_value_type_flags = relationship_resources
      .rich_value_types
      .iter()
      .map(|part| {
        usize::from(part.relationship_id.is_some())
          + usize::from(part.has_global_type)
          + part.types
          + part.key_flags
          + part.reserved_keys
          + part.reserved_key_flags
          + part.text_len
          + usize::from(part.has_extensions)
      })
      .sum::<usize>();
    let web_image_flags = relationship_resources
      .rich_value_web_image
      .as_ref()
      .map_or(0, |part| {
        usize::from(part.relationship_id.is_some())
          + part.images
          + part.address_relationships
          + part.more_images_relationships
          + part.blip_relationships
          + usize::from(part.has_extensions)
      });
    let feature_bag_flags = relationship_resources
      .feature_property_bags
      .as_ref()
      .map_or(0, |part| {
        usize::from(part.relationship_id.is_some())
          + part.declared_count as usize
          + part.bag_extensions
          + part.bags
          + part.values
          + part.text_len
          + part.extension_markers
      });
    let user_data_flags = relationship_resources.user_data.as_ref().map_or(0, |part| {
      usize::from(part.relationship_id.is_some())
        + part.declared_count as usize
        + part.users
        + part.text_len
        + part.extension_markers
    });
    let calculation_chain_flags =
      relationship_resources
        .calculation_chain
        .as_ref()
        .map_or(0, |part| {
          usize::from(part.relationship_id.is_some())
            + part.cells
            + part.flag_count
            + part.text_len
            + usize::from(part.has_extensions)
        });
    let cell_metadata_flags = relationship_resources
      .cell_metadata
      .as_ref()
      .map_or(0, |part| {
        usize::from(part.relationship_id.is_some())
          + part.metadata_types
          + part.metadata_strings
          + part.mdx_records
          + part.future_metadata
          + part.cell_blocks
          + part.value_blocks
          + part.records
          + part.text_len
          + part.extension_markers
      });
    let volatile_dependency_flags = relationship_resources
      .volatile_dependencies
      .as_ref()
      .map_or(0, |part| {
        usize::from(part.relationship_id.is_some()) + part.types + usize::from(part.has_extensions)
      });
    lines.push(format!(
      "workbook relationshipResources customXml={} customXmlFlags={} customDataProps={} customDataFlags={} slicers={} slicerFlags={} timelines={} timelineFlags={} richValues={} richValueItems={} richValueFlags={} richValueStructs={} richStructItems={} richStructFlags={} arrays={} arrayItems={} arrayFlags={} richStyles={} richStyleFlags={} propBags={} propBagFlags={} propBagStructs={} propBagStructFlags={} richValueTypes={} richValueTypeFlags={} webImage={} webImageFlags={} featureBags={} featureBagFlags={} vba={} toolbars={} users={} userFlags={} calcChain={} calcChainFlags={} cellMetadata={} cellMetadataFlags={} volatileDeps={} volatileDepFlags={}",
      relationship_resources.custom_xml_parts.len(),
      custom_xml_flags,
      relationship_resources.custom_data_properties.len(),
      custom_data_flags,
      relationship_resources.slicer_caches.len(),
      slicer_flags,
      relationship_resources.timeline_caches.len(),
      timeline_flags,
      relationship_resources.rich_values.len(),
      rich_value_items,
      rich_value_flags,
      relationship_resources.rich_value_structures.len(),
      rich_structure_items,
      rich_structure_flags,
      relationship_resources.arrays.len(),
      array_items,
      array_flags,
      relationship_resources.rich_styles.len(),
      rich_style_flags,
      relationship_resources.supporting_property_bags.len(),
      property_bag_flags,
      relationship_resources.supporting_property_bag_structures.len(),
      property_bag_structure_flags,
      relationship_resources.rich_value_types.len(),
      rich_value_type_flags,
      relationship_resources.rich_value_web_image.is_some(),
      web_image_flags,
      relationship_resources.feature_property_bags.is_some(),
      feature_bag_flags,
      relationship_resources.has_vba_project,
      relationship_resources.has_attached_toolbars,
      relationship_resources.user_data.is_some(),
      user_data_flags,
      relationship_resources.calculation_chain.is_some(),
      calculation_chain_flags,
      relationship_resources.cell_metadata.is_some(),
      cell_metadata_flags,
      relationship_resources.volatile_dependencies.is_some(),
      volatile_dependency_flags
    ));
  }

  if import.workbook_resources.has_styles {
    let style_xf_flags = import
      .styles
      .style_xfs
      .iter()
      .map(|format| format.used_flag_count())
      .sum::<usize>();
    let cell_xf_flags = import
      .styles
      .cell_xfs
      .iter()
      .map(|format| format.used_flag_count())
      .sum::<usize>();
    let cell_xf_refs = import
      .styles
      .cell_xfs
      .iter()
      .map(|format| {
        usize::from(format.number_format_id.is_some())
          + usize::from(format.font_id.is_some())
          + usize::from(format.fill_id.is_some())
          + usize::from(format.border_id.is_some())
          + usize::from(format.style_xf_id.is_some())
      })
      .sum::<usize>();
    lines.push(format!(
      "styles numFmts={} fonts={} fills={} borders={} cellStyleXfs={} styleXfFlags={} cellXfs={} cellXfFlags={} cellXfRefs={} cellStyles={} dxfs={} dxfFlags={} tableStyles={} defaultTableStyle={} defaultPivotStyle={} colors={} indexedColors={} extensions={}",
      import.styles.custom_number_formats.len(),
      import.styles.fonts,
      import.styles.fills,
      import.styles.borders,
      import.styles.cell_style_formats,
      style_xf_flags,
      import.styles.cell_formats,
      cell_xf_flags,
      cell_xf_refs,
      import.styles.cell_styles,
      import.styles.differential_formats,
      import.styles.differential_format_flag_count(),
      import.styles.table_styles,
      import.styles.default_table_style.as_deref().unwrap_or(""),
      import.styles.default_pivot_style.as_deref().unwrap_or(""),
      import.styles.has_colors,
      import.styles.indexed_colors.len(),
      import.styles.has_extensions
    ));
  }

  if !import.defined_names.records.is_empty() {
    lines.push(format!(
      "definedNames printAreas={} printTitles={} filters={} local={} hidden={}",
      import.defined_names.print_areas,
      import.defined_names.print_titles,
      import.defined_names.filter_databases,
      import.defined_names.local_names,
      import.defined_names.hidden_names
    ));
  }

  lines.push(format!(
    "workbook settings date1904={} dateCompatibility={} showObjects={:?} saveExternalLinks={} updateLinks={:?} codeName={} defaultThemeVersion={} fileVersion={} fileApp={} readOnlyRecommended={} lockStructure={} lockWindows={} lockRevision={} workbookPassword={} revisionPassword={} views={} customViews={} externalRefs={} functionGroups={} pivotCachesXml={} webPublishObjects={} recoveryProps={} extensions={} oleSize={}",
    import.globals.settings.date_1904,
    import
      .globals
      .settings
      .date_compatibility
      .map_or(String::new(), |value| value.to_string()),
    import.globals.settings.show_objects,
    import
      .globals
      .settings
      .save_external_link_values
      .map_or(String::new(), |value| value.to_string()),
    import.globals.settings.update_links,
    import.globals.settings.code_name.as_deref().unwrap_or(""),
    import
      .globals
      .settings
      .default_theme_version
      .map_or(String::new(), |value| value.to_string()),
    import.globals.settings.has_file_version,
    import
      .globals
      .settings
      .file_application_name
      .as_deref()
      .unwrap_or(""),
    import.globals.settings.read_only_recommended,
    import
      .globals
      .settings
      .workbook_protection
      .lock_structure,
    import.globals.settings.workbook_protection.lock_windows,
    import.globals.settings.workbook_protection.lock_revision,
    import
      .globals
      .settings
      .workbook_protection
      .has_workbook_password,
    import
      .globals
      .settings
      .workbook_protection
      .has_revision_password,
    import.globals.views.len(),
    import.globals.custom_workbook_views,
    import.globals.external_references,
    import.globals.function_groups,
    import.globals.pivot_caches,
    import.globals.web_publish_objects,
    import.globals.file_recovery_properties,
    import.globals.has_workbook_extensions,
    import.globals.ole_size_reference.as_deref().unwrap_or("")
  ));

  if let Some(view) = import.globals.views.first() {
    lines.push(format!(
      "workbook view visibility={:?} minimized={} hScroll={} vScroll={} sheetTabs={} firstSheet={} activeTab={} tabRatio={} window={}x{} extensions={}",
      view.visibility,
      view.minimized,
      view
        .show_horizontal_scroll
        .map_or(String::new(), |value| value.to_string()),
      view
        .show_vertical_scroll
        .map_or(String::new(), |value| value.to_string()),
      view
        .show_sheet_tabs
        .map_or(String::new(), |value| value.to_string()),
      view.first_sheet.map_or(String::new(), |value| value.to_string()),
      view.active_tab.map_or(String::new(), |value| value.to_string()),
      view.tab_ratio.map_or(String::new(), |value| value.to_string()),
      view
        .window_width
        .map_or(String::new(), |value| value.to_string()),
      view
        .window_height
        .map_or(String::new(), |value| value.to_string()),
      view.has_extensions
    ));
  }

  lines.push(format!(
    "calcPr id={} mode={:?} refMode={:?} fullCalcOnLoad={} forceFullCalc={} iterate={} iterateCount={} iterateDelta={} fullPrecision={} calcCompleted={} calcOnSave={} concurrentCalc={} concurrentManualCount={}",
    import
      .globals
      .calculation
      .calculation_id
      .map_or(String::new(), |value| value.to_string()),
    import.globals.calculation.calculation_mode,
    import.globals.calculation.reference_mode,
    import.globals.calculation.full_calculation_on_load,
    import.globals.calculation.force_full_calculation,
    import.globals.calculation.iterate,
    import
      .globals
      .calculation
      .iterate_count
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .iterate_delta
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .full_precision
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .calculation_completed
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .calculation_on_save
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .concurrent_calculation
      .map_or(String::new(), |value| value.to_string()),
    import
      .globals
      .calculation
      .concurrent_manual_count
      .map_or(String::new(), |value| value.to_string())
  ));

  lines
}

fn external_link_kind_len(kind: &super::workbook_catalog::ExternalLinkKind) -> usize {
  match kind {
    super::workbook_catalog::ExternalLinkKind::ExternalBook { relationship_id } => {
      relationship_id.len()
    }
    super::workbook_catalog::ExternalLinkKind::Dde { service, topic } => {
      service.len() + topic.len()
    }
    super::workbook_catalog::ExternalLinkKind::Ole {
      relationship_id,
      prog_id,
    } => relationship_id.len() + prog_id.len(),
    super::workbook_catalog::ExternalLinkKind::Unknown => 0,
  }
}

fn print_page_lines(import: &ExcelImport, page: &CalcPrintPage<'_>) -> Vec<String> {
  let mut lines = sheet_lines(import, page.sheet);
  lines.insert(
    1,
    format!(
      "print page={} totalPages={} sheetPage={} zoom={} scaleMode={:?} autoPages={}x{} forcedBreakMinPages={} tdf103516={} paper={} scale={} fit={}x{} dpi={}x{} margins={} grid={} headings={} headerFooterFlags={} headerFooterTextLen={} headerFooterDrawings={}",
      page.page_number,
      page.total_pages,
      page.sheet_page_index + 1,
      page.zoom,
      page.scale_mode,
      page.auto_page_columns,
      page.auto_page_rows,
      page.forced_break_min_pages,
      page.tdf103516_adjusted,
      page.page_settings.paper_size,
      page.page_settings.scale,
      page.page_settings.fit_to_width,
      page.page_settings.fit_to_height,
      page.page_settings.horizontal_dpi,
      page.page_settings.vertical_dpi,
      page.page_settings.has_margins,
      page.page_settings.print_grid_lines,
      page.page_settings.print_headings,
      page.page_settings.header_footer.flag_count(),
      page.page_settings.header_footer.text_len(),
      page.page_settings.header_footer.drawing_slot_count
    ),
  );
  lines.insert(
    2,
    format!(
      "print area={} explicit={} cells={} repeatRowCells={} repeatColCells={} repeatCornerCells={} allCells={} empty={} textLen={} renderedTextLen={} formulas={} hiddenRows={} hiddenCols={} merges={} repeatRows={} repeatCols={} cellHint={}",
      format_print_area(page.area),
      page.explicit_print_area,
      page.cells.len(),
      page.repeated_row_cells.len(),
      page.repeated_column_cells.len(),
      page.repeated_corner_cells.len(),
      page.all_cells,
      page.empty,
      page.cells.iter().map(|cell| cell.text.len()).sum::<usize>(),
      page
        .cells
        .iter()
        .map(|cell| cell.rendered_text.len())
        .sum::<usize>(),
      page.cells.iter().filter(|cell| cell.formula).count(),
      page.hidden_rows,
      page.hidden_columns,
      page.merged_ranges,
      format_print_area(page.repeated_rows),
      format_print_area(page.repeated_columns),
      page.area.map_or(0, |area| area.cell_count_hint())
    ),
  );
  if !page.cells.is_empty() {
    lines.insert(
      3,
      format!(
        "print cells first={} last={} styled={} formatted={} formatCodes={} general={} textFmt={} boolFmt={} unsupportedFmt={}",
        page
          .cells
          .first()
          .map_or(String::new(), |cell| format_cell_address(cell.address)),
        page
          .cells
          .last()
          .map_or(String::new(), |cell| format_cell_address(cell.address)),
        page
          .cells
          .iter()
          .filter(|cell| cell.style_index.is_some())
          .count(),
        page
          .cells
          .iter()
          .filter(|cell| cell.number_format_id.is_some())
          .count(),
        page
          .cells
          .iter()
          .filter(|cell| cell.number_format_code.is_some())
          .count(),
        page
          .cells
          .iter()
          .filter(|cell| {
            cell.number_format_state == super::print::NumberFormatRenderState::General
          })
          .count(),
        page
          .cells
          .iter()
          .filter(|cell| cell.number_format_state == super::print::NumberFormatRenderState::Text)
          .count(),
        page
          .cells
          .iter()
          .filter(|cell| {
            cell.number_format_state == super::print::NumberFormatRenderState::Boolean
          })
          .count(),
        page
          .cells
          .iter()
          .filter(|cell| {
            cell.number_format_state
              == super::print::NumberFormatRenderState::UnsupportedFormatCode
          })
          .count()
      ),
    );
  }
  if page.drawing_summary.anchors > 0 || page.drawing_summary.charts > 0 {
    lines.insert(
      if page.cells.is_empty() { 3 } else { 4 },
      format!(
      "print drawings anchors={} printable={} hidden={} pictures={} charts={} graphicFrames={} shapes={} groups={} connectors={} contentParts={} textLen={}",
        page.drawing_summary.anchors,
        page.drawing_summary.printable,
        page.drawing_summary.hidden,
        page.drawing_summary.pictures,
        page.drawing_summary.charts,
        page.drawing_summary.graphic_frames,
        page.drawing_summary.shapes,
        page.drawing_summary.groups,
        page.drawing_summary.connectors,
        page.drawing_summary.content_parts,
        page.drawing_summary.text_len
      ),
    );
  }
  if !page.paint_ops.is_empty() {
    let insert_index = 3
      + usize::from(!page.cells.is_empty())
      + usize::from(page.drawing_summary.anchors > 0 || page.drawing_summary.charts > 0);
    lines.insert(
      insert_index,
      format!(
        "print paintOps={} backDrawing={} repeatCols={} repeatRows={} cellArea={} grid={} frontDrawing={}",
        page.paint_ops.len(),
        page
          .paint_ops
          .iter()
          .filter(|op| **op == super::print::CalcPrintPaintOp::BackDrawingLayer)
          .count(),
        page
          .paint_ops
          .iter()
          .filter(|op| **op == super::print::CalcPrintPaintOp::RepeatedColumns)
          .count(),
        page
          .paint_ops
          .iter()
          .filter(|op| **op == super::print::CalcPrintPaintOp::RepeatedRows)
          .count(),
        page
          .paint_ops
          .iter()
          .filter(|op| **op == super::print::CalcPrintPaintOp::CellArea)
          .count(),
        page
          .paint_ops
          .iter()
          .filter(|op| **op == super::print::CalcPrintPaintOp::Grid)
          .count(),
        page
          .paint_ops
          .iter()
          .filter(|op| **op == super::print::CalcPrintPaintOp::FrontDrawingLayer)
          .count()
      ),
    );
  }
  if !page.named_ranges.print_areas.is_empty()
    || !page.named_ranges.print_titles.is_empty()
    || !page.named_ranges.filter_databases.is_empty()
  {
    let insert_index = 3
      + usize::from(!page.cells.is_empty())
      + usize::from(page.drawing_summary.anchors > 0 || page.drawing_summary.charts > 0)
      + usize::from(!page.paint_ops.is_empty());
    lines.insert(
      insert_index,
      format!(
        "print names areas={} resolvedAreas={} titles={} filters={} unresolvedFormulas={}",
        page.named_ranges.print_areas.len(),
        page.named_ranges.resolved_print_areas.len(),
        page.named_ranges.print_titles.len(),
        page.named_ranges.filter_databases.len(),
        page.named_ranges.unresolved_formula_count()
      ),
    );
  }
  lines
}

fn format_print_area(area: Option<super::worksheet::CellRange>) -> String {
  area.map_or(String::new(), |area| {
    format!(
      "R{}C{}:R{}C{}",
      area.start.row, area.start.col, area.end.row, area.end.col
    )
  })
}

fn format_cell_address(address: super::worksheet::CellAddress) -> String {
  format!("R{}C{}", address.row, address.col)
}

fn sheet_lines(import: &ExcelImport, sheet: &CalcSheet) -> Vec<String> {
  let mut lines = vec![format!(
    "Sheet {}: {} id={} rel={}{}",
    sheet.workbook_index + 1,
    sheet.name,
    sheet.sheet_id,
    sheet.relationship_id,
    if sheet.visible() { "" } else { " (hidden)" }
  )];

  if sheet.sheet_type == SheetType::Chartsheet {
    lines.push("Chartsheet".to_string());
    if let Some(metrics) = &sheet.chartsheet_metrics {
      lines.push(format!(
        "chartsheet published={} codeName={} tabColor={} views={} selectedViews={} zoomToFitViews={} viewExt={} customViews={} customViewFlags={} protectionFlags={} webPublishItems={} extensions={}",
        metrics.published,
        metrics.code_name.as_deref().unwrap_or(""),
        metrics.has_tab_color,
        metrics.views,
        metrics.selected_views,
        metrics.zoom_to_fit_views,
        metrics.view_extensions,
        metrics.custom_views,
        metrics.custom_view_flags,
        metrics.protection_flags,
        metrics.web_publish_items,
        metrics.has_extensions
      ));
    }
  }

  let drawing_count = sheet.resources.drawings.len();
  let chart_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.chart_count())
    .sum::<usize>();
  let diagram_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.diagram_resource_count())
    .sum::<usize>();
  let drawing_image_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.images)
    .sum::<usize>();
  let drawing_anchor_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.anchors.len())
    .sum::<usize>();
  let chart_child_resource_count = sheet
    .resources
    .drawings
    .iter()
    .map(|drawing| drawing.chart_child_resource_count())
    .sum::<usize>();

  if drawing_count > 0
    || !sheet.resources.object_resources.vml_drawings.is_empty()
    || sheet.resources.comments.legacy_count() > 0
    || sheet.resources.comments.threaded_count() > 0
    || !sheet.resources.tables.is_empty()
    || !sheet.resources.pivot_tables.tables.is_empty()
    || !sheet.resources.query_tables.query_tables.is_empty()
    || !sheet.metrics.objects.ole_objects.is_empty()
    || !sheet.metrics.objects.controls.is_empty()
    || sheet.metrics.objects.unknown_controls > 0
    || sheet_relationship_count(sheet) > 0
  {
    lines.push(format!(
      "resources drawings={} anchors={} charts={} chartResources={} diagrams={} drawingImages={} vml={} comments={} threadedComments={} tables={} pivots={} queries={} oleObjects={} controls={} unknownControls={} sheetRelationships={}",
      drawing_count,
      drawing_anchor_count,
      chart_count,
      chart_child_resource_count,
      diagram_count,
      drawing_image_count,
      sheet.resources.object_resources.vml_drawings.len(),
      sheet.resources.comments.legacy_count(),
      sheet.resources.comments.threaded_count(),
      sheet.resources.tables.len(),
      sheet.resources.pivot_tables.tables.len(),
      sheet.resources.query_tables.query_tables.len(),
      sheet.metrics.objects.ole_objects.len(),
      sheet.metrics.objects.controls.len(),
      sheet.metrics.objects.unknown_controls,
      sheet_relationship_count(sheet)
    ));
  }

  if drawing_anchor_count > 0 {
    let mut two_cell = 0usize;
    let mut one_cell = 0usize;
    let mut absolute = 0usize;
    let mut pictures = 0usize;
    let mut graphic_frames = 0usize;
    let mut shapes = 0usize;
    let mut groups = 0usize;
    let mut connectors = 0usize;
    let mut content_parts = 0usize;
    let mut unknown = 0usize;
    let mut flags = 0usize;
    let mut ref_len = 0usize;
    for anchor in sheet
      .resources
      .drawings
      .iter()
      .flat_map(|drawing| drawing.anchors.iter())
    {
      match anchor.kind {
        super::drawing::DrawingAnchorKind::TwoCell => two_cell += 1,
        super::drawing::DrawingAnchorKind::OneCell => one_cell += 1,
        super::drawing::DrawingAnchorKind::Absolute => absolute += 1,
      }
      match anchor.object.kind {
        super::drawing::DrawingObjectKind::Shape => shapes += 1,
        super::drawing::DrawingObjectKind::GroupShape => groups += 1,
        super::drawing::DrawingObjectKind::GraphicFrame => graphic_frames += 1,
        super::drawing::DrawingObjectKind::ConnectionShape => connectors += 1,
        super::drawing::DrawingObjectKind::Picture => pictures += 1,
        super::drawing::DrawingObjectKind::ContentPart => content_parts += 1,
        super::drawing::DrawingObjectKind::Unknown => unknown += 1,
      }
      flags += usize::from(anchor.edit_as.is_some())
        + usize::from(anchor.lock_with_sheet)
        + usize::from(anchor.print_with_sheet)
        + usize::from(anchor.object.hidden)
        + usize::from(anchor.object.has_style);
      ref_len += anchor.object.id.unwrap_or_default() as usize
        + anchor.object.name.as_ref().map_or(0, |value| value.len())
        + anchor
          .object
          .description
          .as_ref()
          .map_or(0, |value| value.len())
        + anchor
          .object
          .macro_name
          .as_ref()
          .map_or(0, |value| value.len())
        + anchor
          .object
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
        + anchor
          .object
          .graphic_uri
          .as_ref()
          .map_or(0, |value| value.len())
        + anchor.object.text_len
        + anchor.object.child_objects
        + drawing_marker_len(anchor.from.as_ref())
        + drawing_marker_len(anchor.to.as_ref())
        + anchor.position.map_or(0, |(x, y)| {
          x.unsigned_abs() as usize + y.unsigned_abs() as usize
        })
        + anchor.extent.map_or(0, |(cx, cy)| {
          cx.unsigned_abs() as usize + cy.unsigned_abs() as usize
        });
    }
    lines.push(format!(
      "drawing anchors twoCell={} oneCell={} absolute={} pictures={} graphicFrames={} shapes={} groups={} connectors={} contentParts={} unknown={} flags={} refLen={}",
      two_cell,
      one_cell,
      absolute,
      pictures,
      graphic_frames,
      shapes,
      groups,
      connectors,
      content_parts,
      unknown,
      flags,
      ref_len
    ));
  }

  if chart_count > 0 {
    let charts = sheet
      .resources
      .drawings
      .iter()
      .flat_map(|drawing| drawing.charts.iter().chain(drawing.extended_charts.iter()));
    let mut extended = 0usize;
    let mut chart_types = 0usize;
    let mut axes = 0usize;
    let mut flags = 0usize;
    let mut relationships = 0usize;
    let mut text_len = 0usize;
    let mut data_sets = 0usize;
    let mut series = 0usize;
    for chart in charts {
      extended += usize::from(chart.extended);
      chart_types += chart.chart_type_groups;
      axes += chart.axes;
      flags += usize::from(chart.has_fallback_image)
        + usize::from(chart.date1904)
        + usize::from(chart.rounded_corners)
        + usize::from(chart.has_style)
        + usize::from(chart.has_pivot_source)
        + chart.protection_flags
        + usize::from(chart.has_title)
        + usize::from(chart.has_3d_view)
        + usize::from(chart.has_legend)
        + chart.chart_flags
        + usize::from(chart.has_root_shape_properties)
        + usize::from(chart.has_text_properties)
        + usize::from(chart.external_data_auto_update)
        + usize::from(chart.has_print_settings)
        + usize::from(chart.has_user_shapes_reference)
        + chart.extension_markers;
      relationships += usize::from(chart.relationship_id.is_some())
        + usize::from(chart.external_data_relationship_id.is_some())
        + usize::from(chart.has_chart_drawing)
        + usize::from(chart.has_embedded_package)
        + chart.images
        + usize::from(chart.has_theme_override)
        + chart.styles
        + chart.color_styles;
      text_len += chart
        .relationship_id
        .as_ref()
        .map_or(0, |value| value.len())
        + chart.version_len
        + chart.feature_list_len
        + chart
          .external_data_relationship_id
          .as_ref()
          .map_or(0, |value| value.len());
      data_sets += chart.chartex_data_sets;
      series += chart.chartex_series;
    }
    lines.push(format!(
      "charts total={} extended={} typeGroups={} axes={} flags={} relationships={} textLen={} chartexData={} chartexSeries={}",
      chart_count,
      extended,
      chart_types,
      axes,
      flags,
      relationships,
      text_len,
      data_sets,
      series
    ));
  }

  if diagram_count > 0 {
    let diagrams = sheet
      .resources
      .drawings
      .iter()
      .map(|drawing| &drawing.diagrams);
    let mut data_parts = 0usize;
    let mut layout_parts = 0usize;
    let mut style_parts = 0usize;
    let mut color_parts = 0usize;
    let mut drawing_parts = 0usize;
    let mut points = 0usize;
    let mut connections = 0usize;
    let mut layout_nodes = 0usize;
    let mut algorithms = 0usize;
    let mut persisted_shapes = 0usize;
    let mut flags = 0usize;
    let mut relationships = 0usize;
    let mut images = 0usize;
    let mut text_len = 0usize;
    for diagram in diagrams {
      data_parts += diagram.data_parts.len();
      layout_parts += diagram.layout_parts.len();
      style_parts += diagram.style_parts.len();
      color_parts += diagram.color_parts.len();
      drawing_parts += diagram.drawing_parts.len();
      for data in &diagram.data_parts {
        points += data.points + data.unknown_points;
        connections += data.connections;
        flags += data.text_points
          + data.property_sets
          + data.shape_properties
          + usize::from(data.background)
          + usize::from(data.whole)
          + data.extension_markers;
        relationships +=
          usize::from(data.relationship_id.is_some()) + data.slides + data.worksheets;
        images += data.images;
        text_len += data.relationship_id.as_ref().map_or(0, |value| value.len()) + data.text_len;
      }
      for layout in &diagram.layout_parts {
        layout_nodes += layout.layout_nodes;
        algorithms += layout.algorithms;
        flags += layout.titles
          + layout.descriptions
          + usize::from(layout.has_category_list)
          + usize::from(layout.has_sample_data)
          + usize::from(layout.has_style_data)
          + usize::from(layout.has_color_data)
          + layout.shapes
          + layout.presentation_of
          + layout.constraints
          + layout.rules
          + layout.variables
          + layout.for_each
          + layout.choose
          + layout.extension_markers;
        relationships += usize::from(layout.relationship_id.is_some());
        images += layout.images;
        text_len += layout
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + layout.text_len;
      }
      for style in &diagram.style_parts {
        flags += style.titles
          + style.descriptions
          + usize::from(style.has_categories)
          + usize::from(style.has_scene3d)
          + style.labels
          + style.extension_markers;
        relationships += usize::from(style.relationship_id.is_some());
        text_len += style
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + style.text_len;
      }
      for color in &diagram.color_parts {
        flags += color.titles
          + color.descriptions
          + usize::from(color.has_categories)
          + color.labels
          + color.extension_markers;
        relationships += usize::from(color.relationship_id.is_some());
        text_len += color
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + color.text_len;
      }
      for drawing in &diagram.drawing_parts {
        persisted_shapes += drawing.shapes + drawing.groups;
        flags += drawing.text_shapes
          + drawing.styled_shapes
          + drawing.transformed_shapes
          + drawing.extension_markers;
        relationships += usize::from(drawing.relationship_id.is_some());
        images += drawing.images;
        text_len += drawing
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + drawing.text_len;
      }
    }
    lines.push(format!(
      "diagrams data={} layouts={} styles={} colors={} drawings={} points={} connections={} layoutNodes={} algorithms={} persistedShapes={} flags={} relationships={} images={} textLen={}",
      data_parts,
      layout_parts,
      style_parts,
      color_parts,
      drawing_parts,
      points,
      connections,
      layout_nodes,
      algorithms,
      persisted_shapes,
      flags,
      relationships,
      images,
      text_len
    ));
  }

  if sheet_relationship_count(sheet) > 0 {
    let relationships = &sheet.resources.relationships;
    let single_xml_cells = relationships
      .single_xml_cells
      .iter()
      .map(|part| part.cells)
      .sum::<usize>();
    let single_xml_len = relationships
      .single_xml_cells
      .iter()
      .map(|part| {
        part.relationship_id.as_ref().map_or(0, |value| value.len())
          + part.ref_text_len
          + part.unique_name_len
          + part.xpath_len
          + part.id_sum
          + part.extension_cells
      })
      .sum::<usize>();
    let named_views = relationships
      .named_sheet_views
      .iter()
      .map(|part| part.views)
      .sum::<usize>();
    let named_view_filters = relationships
      .named_sheet_views
      .iter()
      .map(|part| part.filters + part.column_filters + part.sort_rules)
      .sum::<usize>();
    let named_view_len = relationships
      .named_sheet_views
      .iter()
      .map(|part| {
        part.relationship_id.as_ref().map_or(0, |value| value.len())
          + part.text_len
          + part.extensions
      })
      .sum::<usize>();
    let slicers = relationships
      .slicers
      .iter()
      .map(|part| part.slicers)
      .sum::<usize>();
    let slicer_len = relationships
      .slicers
      .iter()
      .map(|part| {
        part.relationship_id.as_ref().map_or(0, |value| value.len())
          + part.text_len
          + part.flags
          + part.extensions
      })
      .sum::<usize>();
    let timelines = relationships
      .timelines
      .iter()
      .map(|part| part.timelines)
      .sum::<usize>();
    let timeline_len = relationships
      .timelines
      .iter()
      .map(|part| {
        part.relationship_id.as_ref().map_or(0, |value| value.len())
          + part.text_len
          + part.flags
          + part.extensions
      })
      .sum::<usize>();
    let sort_map_items = relationships
      .sort_map
      .as_ref()
      .map_or(0, |sort_map| sort_map.row_items + sort_map.column_items);
    let sort_map_len = relationships.sort_map.as_ref().map_or(0, |sort_map| {
      sort_map
        .relationship_id
        .as_ref()
        .map_or(0, |value| value.len())
        + sort_map.ref_text_len
        + sort_map.declared_count
    });
    lines.push(format!(
      "sheet relationships singleXmlParts={} singleXmlCells={} singleXmlLen={} namedViewParts={} namedViews={} namedViewFilters={} namedViewLen={} slicerParts={} slicers={} slicerLen={} timelineParts={} timelines={} timelineLen={} sortMapItems={} sortMapLen={} customProps={} printerSettings={} slicerRels={} timelineRels={} model3dRels={} activeXBinaryRels={}",
      relationships.single_xml_cells.len(),
      single_xml_cells,
      single_xml_len,
      relationships.named_sheet_views.len(),
      named_views,
      named_view_filters,
      named_view_len,
      relationships.slicers.len(),
      slicers,
      slicer_len,
      relationships.timelines.len(),
      timelines,
      timeline_len,
      sort_map_items,
      sort_map_len,
      relationships.custom_properties,
      relationships.printer_settings,
      relationships.slicer_relationships,
      relationships.timeline_relationships,
      relationships.model3d_relationships,
      relationships.active_x_binary_relationships
    ));
  }

  if !sheet.metrics.objects.ole_objects.is_empty() || !sheet.metrics.objects.controls.is_empty() {
    let ole_flags = sheet
      .metrics
      .objects
      .ole_objects
      .iter()
      .map(|object| {
        object.property_flags
          + usize::from(object.auto_load)
          + usize::from(object.show_as_icon)
          + usize::from(object.has_embedded_properties)
          + usize::from(object.link.is_some())
          + usize::from(object.relationship_id.is_some())
      })
      .sum::<usize>();
    let ole_ref_len = sheet
      .metrics
      .objects
      .ole_objects
      .iter()
      .map(|object| {
        object.shape_id as usize
          + object
            .relationship_id
            .as_ref()
            .map_or(0, |value| value.len())
          + object.prog_id.as_ref().map_or(0, |value| value.len())
          + object.link.as_ref().map_or(0, |value| value.len())
          + object.property_text_len
          + usize::from(object.data_or_view_aspect.is_some())
          + usize::from(object.ole_update.is_some())
      })
      .sum::<usize>();
    let control_flags = sheet
      .metrics
      .objects
      .controls
      .iter()
      .map(|control| control.property_flags + usize::from(control.has_control_properties))
      .sum::<usize>();
    let control_ref_len = sheet
      .metrics
      .objects
      .controls
      .iter()
      .map(|control| {
        control.shape_id as usize
          + control.relationship_id.len()
          + control.name.as_ref().map_or(0, |value| value.len())
          + control.property_text_len
      })
      .sum::<usize>();
    let anchor_count = sheet
      .metrics
      .objects
      .ole_objects
      .iter()
      .filter(|object| object.anchor.is_some())
      .count()
      + sheet
        .metrics
        .objects
        .controls
        .iter()
        .filter(|control| control.anchor.is_some())
        .count();
    lines.push(format!(
      "objects ole={} oleFlags={} oleRefLen={} controls={} controlFlags={} controlRefLen={} anchors={}",
      sheet.metrics.objects.ole_objects.len(),
      ole_flags,
      ole_ref_len,
      sheet.metrics.objects.controls.len(),
      control_flags,
      control_ref_len,
      anchor_count
    ));
  }

  if !sheet.resources.object_resources.vml_drawings.is_empty()
    || !sheet.resources.object_resources.controls.is_empty()
    || !sheet
      .resources
      .object_resources
      .control_properties
      .is_empty()
    || !sheet.resources.object_resources.embedded_objects.is_empty()
    || !sheet
      .resources
      .object_resources
      .embedded_packages
      .is_empty()
  {
    let resources = &sheet.resources.object_resources;
    let vml_images = resources
      .vml_drawings
      .iter()
      .map(|part| part.images)
      .sum::<usize>();
    let vml_legacy_diagram_texts = resources
      .vml_drawings
      .iter()
      .map(|part| part.legacy_diagram_texts)
      .sum::<usize>();
    let control_binaries = resources
      .controls
      .iter()
      .map(|part| part.binary_data_parts.len())
      .sum::<usize>();
    let control_property_flags = resources
      .control_properties
      .iter()
      .map(|part| {
        usize::from(part.has_object_type)
          + usize::from(part.has_checked)
          + part.boolean_flags
          + part.numeric_fields
          + part.formula_fields
          + part.alignment_fields
          + part.list_items
          + usize::from(part.has_extension_list)
      })
      .sum::<usize>();
    let relationship_count = resources
      .vml_drawings
      .iter()
      .filter(|part| part.relationship_id.is_some())
      .count()
      + resources
        .controls
        .iter()
        .filter(|part| part.relationship_id.is_some())
        .count()
      + resources
        .control_properties
        .iter()
        .filter(|part| part.relationship_id.is_some())
        .count()
      + resources
        .embedded_objects
        .iter()
        .filter(|part| part.relationship_id.is_some())
        .count()
      + resources
        .embedded_packages
        .iter()
        .filter(|part| part.relationship_id.is_some())
        .count();
    let text_len = resources
      .vml_drawings
      .iter()
      .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()))
      .sum::<usize>()
      + resources
        .controls
        .iter()
        .map(|part| {
          part.relationship_id.as_ref().map_or(0, |value| value.len())
            + part
              .binary_data_parts
              .iter()
              .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()))
              .sum::<usize>()
        })
        .sum::<usize>()
      + resources
        .control_properties
        .iter()
        .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()) + part.text_len)
        .sum::<usize>()
      + resources
        .embedded_objects
        .iter()
        .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()))
        .sum::<usize>()
      + resources
        .embedded_packages
        .iter()
        .map(|part| part.relationship_id.as_ref().map_or(0, |value| value.len()))
        .sum::<usize>();
    lines.push(format!(
      "objectResources vml={} vmlImages={} legacyDiagramTexts={} controls={} controlBinaries={} controlProps={} controlPropFlags={} embeddedObjects={} embeddedPackages={} relationships={} textLen={}",
      resources.vml_drawings.len(),
      vml_images,
      vml_legacy_diagram_texts,
      resources.controls.len(),
      control_binaries,
      resources.control_properties.len(),
      control_property_flags,
      resources.embedded_objects.len(),
      resources.embedded_packages.len(),
      relationship_count,
      text_len
    ));
  }

  if !sheet.resources.query_tables.query_tables.is_empty() {
    let refresh_fields = sheet
      .resources
      .query_tables
      .query_tables
      .iter()
      .map(|query| query.refresh_fields)
      .sum::<usize>();
    let deleted_fields = sheet
      .resources
      .query_tables
      .query_tables
      .iter()
      .map(|query| query.deleted_fields)
      .sum::<usize>();
    let query_flags = sheet
      .resources
      .query_tables
      .query_tables
      .iter()
      .map(|query| {
        query.flag_count
          + usize::from(query.has_sort_state)
          + usize::from(query.has_refresh_extensions)
          + usize::from(query.has_extensions)
      })
      .sum::<usize>();
    let query_ref_len = sheet
      .resources
      .query_tables
      .query_tables
      .iter()
      .map(|query| {
        query
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + query.name.len()
          + query.connection_id as usize
      })
      .sum::<usize>();
    lines.push(format!(
      "queryTables count={} refreshFields={} deletedFields={} flags={} refLen={}",
      sheet.resources.query_tables.query_tables.len(),
      refresh_fields,
      deleted_fields,
      query_flags,
      query_ref_len
    ));
  }

  if !sheet.resources.pivot_tables.tables.is_empty() {
    let pivot_fields = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| pivot.pivot_fields)
      .sum::<usize>();
    let axis_fields = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| pivot.row_fields + pivot.column_fields + pivot.page_fields + pivot.data_fields)
      .sum::<usize>();
    let filters = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| pivot.filters)
      .sum::<usize>();
    let formats = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| pivot.formats)
      .sum::<usize>();
    let pivot_flags = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| {
        pivot.flag_count
          + usize::from(pivot.style_name.is_some())
          + usize::from(pivot.has_cache_definition_part)
          + usize::from(pivot.has_extensions)
      })
      .sum::<usize>();
    let pivot_ref_len = sheet
      .resources
      .pivot_tables
      .tables
      .iter()
      .map(|pivot| {
        pivot
          .relationship_id
          .as_ref()
          .map_or(0, |value| value.len())
          + pivot.name.len()
          + pivot.location_reference.len()
          + pivot.cache_id as usize
      })
      .sum::<usize>();
    lines.push(format!(
      "pivots fields={} axisFields={} filters={} formats={} flags={} refLen={}",
      pivot_fields, axis_fields, filters, formats, pivot_flags, pivot_ref_len
    ));
  }

  if sheet.resources.comments.legacy_count() > 0 || sheet.resources.comments.threaded_count() > 0 {
    let legacy_authors = sheet
      .resources
      .comments
      .legacy
      .as_ref()
      .map_or(0, |legacy| legacy.authors.len());
    let legacy_text_len = sheet
      .resources
      .comments
      .legacy
      .as_ref()
      .map_or(0, |legacy| {
        legacy
          .comments
          .iter()
          .map(|comment| {
            comment.reference.len()
              + comment.author.as_ref().map_or(0, |author| author.len())
              + comment.guid.as_ref().map_or(0, |guid| guid.len())
              + comment.text.len()
              + comment.shape_id.unwrap_or_default() as usize
              + comment.author_id as usize
              + comment.rich_runs
              + comment.phonetic_runs
              + usize::from(comment.has_comment_properties)
          })
          .sum()
      });
    let legacy_extensions = sheet
      .resources
      .comments
      .legacy
      .as_ref()
      .is_some_and(|legacy| legacy.has_extensions);
    let legacy_rel = sheet
      .resources
      .comments
      .legacy
      .as_ref()
      .and_then(|legacy| legacy.relationship_id.as_deref())
      .unwrap_or("");
    let threaded_roots = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .filter(|comment| comment.parent_id.is_none())
      .count();
    let threaded_replies = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .filter(|comment| comment.parent_id.is_some())
      .count();
    let threaded_done = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .filter(|comment| comment.done)
      .count();
    let threaded_mentions = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .map(|comment| comment.mentions)
      .sum::<usize>();
    let threaded_text_len = sheet
      .resources
      .comments
      .threaded
      .iter()
      .flat_map(|threaded| &threaded.comments)
      .map(|comment| {
        comment.reference.as_ref().map_or(0, |value| value.len())
          + comment.id.len()
          + comment.parent_id.as_ref().map_or(0, |value| value.len())
          + comment.person_id.len()
          + comment.date_time.as_ref().map_or(0, |value| value.len())
          + comment.text.as_ref().map_or(0, |value| value.len())
          + usize::from(comment.has_extensions)
      })
      .sum::<usize>();
    let threaded_extensions = sheet
      .resources
      .comments
      .threaded
      .iter()
      .filter(|threaded| threaded.has_extensions)
      .count();
    let threaded_rel_len = sheet
      .resources
      .comments
      .threaded
      .iter()
      .map(|threaded| threaded.relationship_id.as_ref().map_or(0, |id| id.len()))
      .sum::<usize>();
    lines.push(format!(
      "comments legacyAuthors={} legacyRel={} legacyExt={} legacyTextLen={} threadedParts={} threadedRoots={} threadedReplies={} threadedDone={} threadedMentions={} threadedExt={} threadedRelLen={} threadedTextLen={}",
      legacy_authors,
      legacy_rel,
      legacy_extensions,
      legacy_text_len,
      sheet.resources.comments.threaded.len(),
      threaded_roots,
      threaded_replies,
      threaded_done,
      threaded_mentions,
      threaded_extensions,
      threaded_rel_len,
      threaded_text_len
    ));
  }

  if !sheet.resources.tables.is_empty() {
    let table_columns = sheet
      .resources
      .tables
      .iter()
      .map(|table| table.columns.len())
      .sum::<usize>();
    let auto_filters = sheet
      .resources
      .tables
      .iter()
      .filter(|table| table.has_auto_filter)
      .count();
    let sort_states = sheet
      .resources
      .tables
      .iter()
      .filter(|table| table.has_sort_state)
      .count();
    let styled_tables = sheet
      .resources
      .tables
      .iter()
      .filter(|table| table.style.name.is_some())
      .count();
    let query_tables = sheet
      .resources
      .tables
      .iter()
      .map(|table| table.query_tables)
      .sum::<usize>();
    let formula_columns = sheet
      .resources
      .tables
      .iter()
      .flat_map(|table| &table.columns)
      .filter(|column| column.has_calculated_formula || column.has_totals_formula)
      .count();
    let xml_columns = sheet
      .resources
      .tables
      .iter()
      .flat_map(|table| &table.columns)
      .filter(|column| column.has_xml_column_properties)
      .count();
    let table_name_len = sheet
      .resources
      .tables
      .iter()
      .map(|table| {
        table.display_name.len()
          + table.name.as_ref().map_or(0, |name| name.len())
          + table.reference.len()
          + table.relationship_id.as_ref().map_or(0, |id| id.len())
          + usize::from(table.table_type.is_some())
          + table.id as usize
          + table.header_rows as usize
          + table.totals_rows as usize
      })
      .sum::<usize>();
    let table_style_flags = sheet
      .resources
      .tables
      .iter()
      .filter(|table| {
        table.style.show_first_column
          || table.style.show_last_column
          || table.style.show_row_stripes
          || table.style.show_column_stripes
          || table.has_extensions
      })
      .count();
    let table_column_name_len = sheet
      .resources
      .tables
      .iter()
      .flat_map(|table| &table.columns)
      .map(|column| {
        column.name.len()
          + column.unique_name.as_ref().map_or(0, |name| name.len())
          + column
            .totals_row_label
            .as_ref()
            .map_or(0, |label| label.len())
          + usize::from(column.totals_row_function.is_some())
          + column.query_table_field_id.unwrap_or_default() as usize
          + column.id as usize
          + usize::from(column.has_extensions)
      })
      .sum::<usize>();
    lines.push(format!(
      "tables columns={} autoFilters={} sortStates={} styled={} styleFlags={} queryTables={} formulaColumns={} xmlColumns={} tableNameLen={} columnNameLen={}",
      table_columns,
      auto_filters,
      sort_states,
      styled_tables,
      table_style_flags,
      query_tables,
      formula_columns,
      xml_columns,
      table_name_len,
      table_column_name_len
    ));
  }

  if sheet.metrics.dimension.is_some()
    || sheet.metrics.settings.properties.code_name.is_some()
    || sheet.metrics.settings.auto_filter.is_some()
    || sheet.metrics.settings.sort_state.is_some()
    || sheet.metrics.settings.protection.sheet
    || !sheet.metrics.views.views.is_empty()
    || !sheet.metrics.columns.is_empty()
    || !sheet.metrics.merged_ranges.is_empty()
    || !sheet.metrics.hyperlinks.is_empty()
    || !sheet.metrics.row_breaks.is_empty()
    || !sheet.metrics.column_breaks.is_empty()
    || !sheet.metrics.conditions.conditional_formats.is_empty()
    || !sheet.metrics.conditions.data_validations.is_empty()
    || sheet.metrics.protected_ranges > 0
    || sheet.metrics.scenarios > 0
  {
    let hidden_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.hidden)
      .count();
    let styled_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.style_index.is_some())
      .count();
    let styled_rows = sheet
      .rows
      .iter()
      .filter(|row| row.style_index.is_some())
      .count();
    let custom_height_rows = sheet.rows.iter().filter(|row| row.custom_height).count();
    let custom_width_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.custom_width)
      .count();
    let best_fit_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.best_fit)
      .count();
    let collapsed_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.collapsed)
      .count();
    let phonetic_columns = sheet
      .metrics
      .columns
      .iter()
      .filter(|column| column.phonetic)
      .count();
    let max_outline = sheet
      .metrics
      .columns
      .iter()
      .map(|column| column.outline_level)
      .max()
      .unwrap_or(0);
    let custom_width_sum = sheet
      .metrics
      .columns
      .iter()
      .filter_map(|column| column.width)
      .sum::<f64>();
    let column_span_count = sheet
      .metrics
      .columns
      .iter()
      .map(|column| column.last.saturating_sub(column.first) + 1)
      .sum::<u32>();
    let manual_breaks = sheet
      .metrics
      .row_breaks
      .iter()
      .chain(&sheet.metrics.column_breaks)
      .filter(|page_break| page_break.manual)
      .count();
    let pivot_breaks = sheet
      .metrics
      .row_breaks
      .iter()
      .chain(&sheet.metrics.column_breaks)
      .filter(|page_break| page_break.pivot)
      .count();
    let break_extent_sum = sheet
      .metrics
      .row_breaks
      .iter()
      .chain(&sheet.metrics.column_breaks)
      .map(|page_break| page_break.max.saturating_sub(page_break.min) + page_break.id)
      .sum::<u32>();
    let relationship_hyperlinks = sheet
      .metrics
      .hyperlinks
      .iter()
      .filter(|hyperlink| hyperlink.relationship_id.is_some())
      .count();
    let local_hyperlinks = sheet
      .metrics
      .hyperlinks
      .iter()
      .filter(|hyperlink| hyperlink.location.is_some())
      .count();
    let displayed_hyperlinks = sheet
      .metrics
      .hyperlinks
      .iter()
      .filter(|hyperlink| hyperlink.display.is_some() || hyperlink.tooltip.is_some())
      .count();
    let hyperlink_ref_len = sheet
      .metrics
      .hyperlinks
      .iter()
      .map(|hyperlink| hyperlink.reference.len())
      .sum::<usize>();
    let selected_views = sheet
      .metrics
      .views
      .views
      .iter()
      .filter(|view| view.tab_selected == Some(true))
      .count();
    let panes = sheet
      .metrics
      .views
      .views
      .iter()
      .filter(|view| view.pane.is_some())
      .count();
    let selections = sheet
      .metrics
      .views
      .views
      .iter()
      .map(|view| view.selections.len())
      .sum::<usize>();
    let pivot_selections = sheet
      .metrics
      .views
      .views
      .iter()
      .map(|view| view.pivot_selections)
      .sum::<usize>();
    let view_flags = sheet
      .metrics
      .views
      .views
      .iter()
      .filter(|view| {
        view.window_protection.is_some()
          || view.show_formulas.is_some()
          || view.show_grid_lines.is_some()
          || view.show_row_col_headers.is_some()
          || view.show_zeros.is_some()
          || view.right_to_left.is_some()
          || view.show_outline_symbols.is_some()
          || view.default_grid_color.is_some()
          || view.has_extensions
      })
      .count();
    let view_ref_len = sheet
      .metrics
      .views
      .views
      .iter()
      .map(|view| {
        view.top_left_cell.as_ref().map_or(0, |value| value.len())
          + view.color_id.unwrap_or_default() as usize
          + view.zoom_scale.unwrap_or_default() as usize
          + view.zoom_scale_normal.unwrap_or_default() as usize
          + view.zoom_scale_sheet_layout_view.unwrap_or_default() as usize
          + view.zoom_scale_page_layout_view.unwrap_or_default() as usize
          + view.workbook_view_id as usize
          + usize::from(view.view_type.is_some())
          + view.pane.as_ref().map_or(0, |pane| {
            pane.top_left_cell.as_ref().map_or(0, |value| value.len())
              + pane.horizontal_split.unwrap_or_default() as usize
              + pane.vertical_split.unwrap_or_default() as usize
              + usize::from(pane.active_pane.is_some())
              + usize::from(pane.state.is_some())
          })
          + view
            .selections
            .iter()
            .map(|selection| {
              selection
                .active_cell
                .as_ref()
                .map_or(0, |value| value.len())
                + selection.active_cell_id.unwrap_or_default() as usize
                + selection.sequence_of_references.len()
                + usize::from(selection.pane.is_some())
            })
            .sum::<usize>()
      })
      .sum::<usize>();
    let settings = &sheet.metrics.settings;
    let property_flags = usize::from(settings.properties.filter_mode)
      + usize::from(settings.properties.published.is_some())
      + usize::from(settings.properties.sync_horizontal.is_some())
      + usize::from(settings.properties.sync_vertical.is_some())
      + usize::from(settings.properties.sync_reference.is_some())
      + usize::from(settings.properties.transition_evaluation.is_some())
      + usize::from(settings.properties.transition_entry.is_some())
      + usize::from(
        settings
          .properties
          .enable_format_conditions_calculation
          .is_some(),
      )
      + usize::from(settings.properties.has_tab_color)
      + usize::from(settings.properties.outline.apply_styles)
      + usize::from(settings.properties.outline.summary_below.is_some())
      + usize::from(settings.properties.outline.summary_right.is_some())
      + usize::from(settings.properties.outline.show_outline_symbols.is_some())
      + usize::from(settings.properties.page_setup.auto_page_breaks.is_some())
      + usize::from(settings.properties.page_setup.fit_to_page);
    let protection_flags = usize::from(settings.protection.has_password)
      + usize::from(settings.protection.has_hash)
      + usize::from(settings.protection.algorithm_name.is_some())
      + usize::from(settings.protection.spin_count.is_some())
      + usize::from(settings.protection.sheet)
      + usize::from(settings.protection.objects)
      + usize::from(settings.protection.scenarios)
      + settings.protection.locked_options
      + usize::from(settings.protection.unlocked_selection)
      + usize::from(settings.protection.locked_selection);
    let auto_filter_columns = settings
      .auto_filter
      .as_ref()
      .map_or(0, |auto_filter| auto_filter.filter_columns);
    let auto_filter_flags = settings.auto_filter.as_ref().map_or(0, |auto_filter| {
      usize::from(auto_filter.reference.is_some())
        + usize::from(auto_filter.sort_state.is_some())
        + usize::from(auto_filter.has_extensions)
    });
    let sort_flags = settings
      .sort_state
      .as_ref()
      .map_or(0, sort_state_flag_count)
      + settings
        .auto_filter
        .as_ref()
        .and_then(|auto_filter| auto_filter.sort_state.as_ref())
        .map_or(0, sort_state_flag_count);
    lines.push(format!(
      "sheet metrics dimension={} settingsCode={} propertyFlags={} protectionFlags={} autoFilterColumns={} autoFilterFlags={} sortFlags={} views={} selectedViews={} panes={} selections={} pivotSelections={} viewFlags={} viewExt={} viewRefLen={} baseColWidth={} defaultColWidth={} defaultRowHeight={} customHeight={} zeroHeight={} thickTop={} thickBottom={} columns={} columnSpans={} hiddenColumns={} styledColumns={} customWidthColumns={} bestFitColumns={} collapsedColumns={} phoneticColumns={} maxOutline={} widthSum={} styledRows={} customHeightRows={} merges={} hyperlinks={} hyperlinkRels={} hyperlinkTargets={} localHyperlinks={} displayedHyperlinks={} hyperlinkRefLen={} rowBreaks={} colBreaks={} manualBreaks={} pivotBreaks={} breakExtentSum={} condFmt={} validations={} protectedRanges={} scenarios={}",
      sheet.metrics.dimension.as_deref().unwrap_or(""),
      sheet
        .metrics
        .settings
        .properties
        .code_name
        .as_deref()
        .unwrap_or(""),
      property_flags,
      protection_flags,
      auto_filter_columns,
      auto_filter_flags,
      sort_flags,
      sheet.metrics.views.views.len(),
      selected_views,
      panes,
      selections,
      pivot_selections,
      view_flags,
      sheet.metrics.views.has_extensions,
      view_ref_len,
      sheet
        .metrics
        .format
        .base_column_width
        .map_or(String::new(), |value| value.to_string()),
      sheet
        .metrics
        .format
        .default_column_width
        .map_or(String::new(), |value| value.to_string()),
      sheet.metrics.format.default_row_height,
      sheet.metrics.format.custom_height,
      sheet.metrics.format.zero_height,
      sheet.metrics.format.thick_top,
      sheet.metrics.format.thick_bottom,
      sheet.metrics.columns.len(),
      column_span_count,
      hidden_columns,
      styled_columns,
      custom_width_columns,
      best_fit_columns,
      collapsed_columns,
      phonetic_columns,
      max_outline,
      custom_width_sum,
      styled_rows,
      custom_height_rows,
      sheet.metrics.merged_ranges.len(),
      sheet.metrics.hyperlinks.len(),
      relationship_hyperlinks,
      sheet.resources.relationships.hyperlink_targets.len(),
      local_hyperlinks,
      displayed_hyperlinks,
      hyperlink_ref_len,
      sheet.metrics.row_breaks.len(),
      sheet.metrics.column_breaks.len(),
      manual_breaks,
      pivot_breaks,
      break_extent_sum,
      sheet.metrics.conditions.conditional_formats.len(),
      sheet.metrics.conditions.data_validations.len(),
      sheet.metrics.protected_ranges,
      sheet.metrics.scenarios
    ));
  }

  if !sheet.metrics.conditions.conditional_formats.is_empty()
    || !sheet.metrics.conditions.data_validations.is_empty()
    || !sheet
      .metrics
      .conditions
      .extension_conditions
      .conditional_formats
      .is_empty()
    || !sheet
      .metrics
      .conditions
      .extension_conditions
      .data_validations
      .is_empty()
    || !sheet
      .metrics
      .conditions
      .extension_conditions
      .sparkline_groups
      .is_empty()
    || !sheet
      .metrics
      .conditions
      .extension_conditions
      .ignored_errors
      .is_empty()
  {
    let cf_rules = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .map(|format| format.rules.len())
      .sum::<usize>();
    let cf_formulas = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .flat_map(|conditional_format| &conditional_format.rules)
      .map(|rule| rule.formulas.len())
      .sum::<usize>();
    let cf_extensions = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .filter(|format| format.has_extensions)
      .count()
      + sheet
        .metrics
        .conditions
        .conditional_formats
        .iter()
        .flat_map(|format| &format.rules)
        .filter(|rule| rule.has_extensions)
        .count();
    let cf_visual_rules = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .flat_map(|conditional_format| &conditional_format.rules)
      .filter(|rule| rule.has_color_scale || rule.has_data_bar || rule.has_icon_set)
      .count();
    let cf_pivot = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .filter(|format| format.pivot)
      .count();
    let cf_ref_count = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .map(|conditional_format| conditional_format.sequence_of_references.len())
      .sum::<usize>();
    let cf_priority_sum = sheet
      .metrics
      .conditions
      .conditional_formats
      .iter()
      .flat_map(|conditional_format| &conditional_format.rules)
      .map(|rule| {
        rule.priority as i64
          + rule.format_id.unwrap_or_default() as i64
          + rule.rank.unwrap_or_default() as i64
          + rule.std_dev.unwrap_or_default() as i64
          + i64::from(rule.stop_if_true)
          + i64::from(rule.operator.is_some())
          + i64::from(rule.time_period.is_some())
          + i64::from(rule.text.is_some())
          + format!("{:?}", rule.rule_type).len() as i64
      })
      .sum::<i64>();
    let validation_formulas = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .filter(|validation| validation.formula1.is_some() || validation.formula2.is_some())
      .count();
    let validation_messages = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .filter(|validation| {
        validation.error_title.is_some()
          || validation.error.is_some()
          || validation.prompt_title.is_some()
          || validation.prompt.is_some()
      })
      .count();
    let validation_lists = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .filter(|validation| validation.list.is_some())
      .count();
    let validation_flags = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .filter(|validation| {
        validation.allow_blank
          || validation.no_drop_down
          || validation.show_input_message
          || validation.show_error_message
          || validation.error_style.is_some()
          || validation.ime_mode.is_some()
          || validation.operator.is_some()
          || validation.validation_type.is_some()
      })
      .count();
    let validation_ref_count = sheet
      .metrics
      .conditions
      .data_validations
      .iter()
      .map(|validation| validation.sequence_of_references.len())
      .sum::<usize>();
    lines.push(format!(
      "conditions cf={} cfRules={} cfFormulas={} cfVisual={} cfPivot={} cfRefs={} cfExt={} cfPrioritySum={} validations={} validationRefs={} validationFormulas={} validationLists={} validationMessages={} validationFlags={} disablePrompts={} validationWindow={}",
      sheet.metrics.conditions.conditional_formats.len(),
      cf_rules,
      cf_formulas,
      cf_visual_rules,
      cf_pivot,
      cf_ref_count,
      cf_extensions,
      cf_priority_sum,
      sheet.metrics.conditions.data_validations.len(),
      validation_ref_count,
      validation_formulas,
      validation_lists,
      validation_messages,
      validation_flags,
      sheet.metrics.conditions.validations_disable_prompts,
      sheet
        .metrics
        .conditions
        .validation_window
        .map_or(String::new(), |(x, y)| format!("{x},{y}"))
    ));

    let ext = &sheet.metrics.conditions.extension_conditions;
    if !ext.conditional_formats.is_empty()
      || !ext.data_validations.is_empty()
      || !ext.sparkline_groups.is_empty()
      || !ext.ignored_errors.is_empty()
      || ext.slicer_refs > 0
      || ext.protected_ranges > 0
      || ext.web_extensions > 0
      || ext.timeline_refs > 0
      || ext.unknown_extensions > 0
    {
      let ext_cf_rules = ext
        .conditional_formats
        .iter()
        .map(|format| format.rules.len())
        .sum::<usize>();
      let ext_cf_formulas = ext
        .conditional_formats
        .iter()
        .flat_map(|format| &format.rules)
        .map(|rule| rule.formulas.len())
        .sum::<usize>();
      let ext_cf_visual = ext
        .conditional_formats
        .iter()
        .flat_map(|format| &format.rules)
        .filter(|rule| rule.has_color_scale || rule.has_data_bar || rule.has_icon_set)
        .count();
      let ext_cf_flags = ext
        .conditional_formats
        .iter()
        .map(|format| {
          usize::from(format.pivot)
            + format.sequence_of_references.len()
            + usize::from(format.has_extensions)
            + format
              .rules
              .iter()
              .map(|rule| {
                usize::from(rule.rule_type.is_some())
                  + usize::from(rule.priority.is_some())
                  + usize::from(rule.stop_if_true)
                  + rule.boolean_flags
                  + usize::from(rule.operator.is_some())
                  + usize::from(rule.text.is_some())
                  + usize::from(rule.time_period.is_some())
                  + usize::from(rule.rank.is_some())
                  + usize::from(rule.std_dev.is_some())
                  + usize::from(rule.id.is_some())
                  + usize::from(rule.has_differential_format)
                  + usize::from(rule.has_extensions)
              })
              .sum::<usize>()
        })
        .sum::<usize>();
      let ext_validation_flags = ext
        .data_validations
        .iter()
        .map(|validation| {
          usize::from(validation.validation_type.is_some())
            + usize::from(validation.error_style.is_some())
            + usize::from(validation.ime_mode.is_some())
            + usize::from(validation.operator.is_some())
            + usize::from(validation.allow_blank)
            + usize::from(validation.no_drop_down)
            + usize::from(validation.show_input_message)
            + usize::from(validation.show_error_message)
            + usize::from(validation.error_title.is_some())
            + usize::from(validation.error.is_some())
            + usize::from(validation.prompt_title.is_some())
            + usize::from(validation.prompt.is_some())
            + usize::from(validation.formula1.is_some())
            + usize::from(validation.formula2.is_some())
            + validation.sequence_of_references.len()
        })
        .sum::<usize>();
      let sparklines = ext
        .sparkline_groups
        .iter()
        .map(|group| group.sparklines)
        .sum::<usize>();
      let sparkline_flags = ext
        .sparkline_groups
        .iter()
        .map(|group| {
          usize::from(group.formula.is_some())
            + group.color_count
            + group.flag_count
            + group.sparkline_formula_text_len
            + group.reference_text_len
        })
        .sum::<usize>();
      let ignored_errors = ext
        .ignored_errors
        .iter()
        .map(|errors| errors.ignored_errors)
        .sum::<usize>();
      let ignored_error_flags = ext
        .ignored_errors
        .iter()
        .map(|errors| {
          errors.flag_count + errors.reference_text_len + usize::from(errors.has_extensions)
        })
        .sum::<usize>();
      lines.push(format!(
        "conditionExt cf={} cfRules={} cfFormulas={} cfVisual={} cfFlags={} validations={} validationFlags={} sparklineGroups={} sparklines={} sparklineFlags={} ignoredErrors={} ignoredErrorFlags={} slicerRefs={} protectedRanges={} webExtensions={} timelineRefs={} unknown={} uriLen={}",
        ext.conditional_formats.len(),
        ext_cf_rules,
        ext_cf_formulas,
        ext_cf_visual,
        ext_cf_flags,
        ext.data_validations.len(),
        ext_validation_flags,
        ext.sparkline_groups.len(),
        sparklines,
        sparkline_flags,
        ignored_errors,
        ignored_error_flags,
        ext.slicer_refs,
        ext.protected_ranges,
        ext.web_extensions,
        ext.timeline_refs,
        ext.unknown_extensions,
        ext.uri_text_len
      ));
    }
  }

  let formula_cells = sheet
    .rows
    .iter()
    .flat_map(|row| &row.cells)
    .filter_map(|cell| cell.formula.as_ref())
    .collect::<Vec<_>>();
  if !formula_cells.is_empty() {
    let shared = formula_cells
      .iter()
      .filter(|formula| formula.formula_type == x::CellFormulaValues::Shared)
      .count();
    let arrays = formula_cells
      .iter()
      .filter(|formula| formula.formula_type == x::CellFormulaValues::Array)
      .count();
    let data_tables = formula_cells
      .iter()
      .filter(|formula| formula.formula_type == x::CellFormulaValues::DataTable)
      .count();
    let normal = formula_cells
      .iter()
      .filter(|formula| formula.formula_type == x::CellFormulaValues::Normal)
      .count();
    let shared_ids = formula_cells
      .iter()
      .filter(|formula| formula.shared_index.is_some())
      .count();
    let references = formula_cells
      .iter()
      .filter(|formula| formula.reference.is_some())
      .count();
    let formula_flags = formula_cells
      .iter()
      .filter(|formula| {
        formula.always_calculate_array
          || formula.calculate_cell
          || formula.data_table_2d
          || formula.data_table_row
          || formula.input1_deleted
          || formula.input2_deleted
          || formula.assigns_value_to_name
      })
      .count();
    let formula_text_len = formula_cells
      .iter()
      .map(|formula| {
        formula.text.len()
          + formula
            .input1_reference
            .as_ref()
            .map_or(0, |value| value.len())
          + formula
            .input2_reference
            .as_ref()
            .map_or(0, |value| value.len())
      })
      .sum::<usize>();
    lines.push(format!(
      "formulas total={} normal={} shared={} arrays={} dataTables={} sharedIds={} refs={} flags={} textLen={}",
      formula_cells.len(),
      normal,
      shared,
      arrays,
      data_tables,
      shared_ids,
      references,
      formula_flags,
      formula_text_len
    ));
  }

  for row in &sheet.rows {
    let values = row
      .cells
      .iter()
      .map(|cell| {
        let text = if cell.formula.is_some() {
          cell
            .cached_value
            .clone()
            .unwrap_or_else(|| cell.display_text.clone())
        } else {
          cell.display_text.clone()
        };
        let mut parts = Vec::new();
        if let Some(reference) = &cell.reference {
          parts.push(reference.clone());
        }
        if let Some(style_index) = cell.style_index {
          parts.push(format!("s={style_index}"));
        }
        if let Some(data_type) = cell.data_type {
          parts.push(format!("t={data_type:?}"));
        }
        if let Some(cell_meta_index) = cell.cell_meta_index {
          parts.push(format!("cm={cell_meta_index}"));
        }
        if let Some(value_meta_index) = cell.value_meta_index {
          parts.push(format!("vm={value_meta_index}"));
        }
        if cell.show_phonetic {
          parts.push("ph".to_string());
        }
        if cell.has_extensions {
          parts.push("ext".to_string());
        }
        if parts.is_empty() || text.is_empty() {
          text
        } else {
          format!("{}={text}", parts.join("/"))
        }
      })
      .collect::<Vec<_>>();
    if values.iter().any(|value| !value.is_empty()) {
      let mut row_flags = Vec::new();
      if row.hidden {
        row_flags.push("hidden".to_string());
      }
      if row.custom_height {
        row_flags.push("customHeight".to_string());
      }
      if let Some(style_index) = row.style_index {
        row_flags.push(format!("s={style_index}"));
      }
      if let Some(height) = row.height {
        row_flags.push(format!("ht={height}"));
      }
      let prefix = match (row.row_index, row_flags.is_empty()) {
        (Some(index), true) => format!("row {index}: "),
        (Some(index), false) => format!("row {index} {}: ", row_flags.join("/")),
        (None, true) => String::new(),
        (None, false) => format!("row {}: ", row_flags.join("/")),
      };
      lines.push(format!("{prefix}{}", values.join("    ")));
    }
  }

  if lines.len() == 1 && !import.shared_strings.is_empty() {
    lines.push(format!("sharedStrings={}", import.shared_strings.len()));
  }

  lines
}

fn sort_state_flag_count(sort_state: &super::sheet_settings::SortStateModel) -> usize {
  sort_state.reference.len()
    + usize::from(sort_state.column_sort)
    + usize::from(sort_state.case_sensitive)
    + usize::from(sort_state.sort_method.is_some())
    + usize::from(sort_state.has_sort_condition)
    + usize::from(sort_state.has_extensions)
}

fn sheet_relationship_count(sheet: &CalcSheet) -> usize {
  let relationships = &sheet.resources.relationships;
  relationships.single_xml_cells.len()
    + relationships.named_sheet_views.len()
    + relationships.slicers.len()
    + relationships.timelines.len()
    + usize::from(relationships.sort_map.is_some())
    + relationships.custom_properties
    + relationships.printer_settings
    + relationships.slicer_relationships
    + relationships.timeline_relationships
    + relationships.model3d_relationships
    + relationships.active_x_binary_relationships
}

fn drawing_marker_len(marker: Option<&super::drawing::DrawingMarkerModel>) -> usize {
  marker.map_or(0, |marker| {
    marker.column.unsigned_abs() as usize
      + marker.row.unsigned_abs() as usize
      + marker.column_offset_emu.unsigned_abs() as usize
      + marker.row_offset_emu.unsigned_abs() as usize
  })
}
