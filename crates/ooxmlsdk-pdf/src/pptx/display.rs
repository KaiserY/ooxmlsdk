use std::sync::Arc;

use crate::docx::{BorderStyle, RgbColor, TextStyle};
use crate::layout::{
  self, ImageItem, LineItem, LineItemKind, LinkAreaItem, PageItem, PdfTextSegmentation, RectItem,
  TextItem,
};
use crate::text_metrics::measure_text;
use crate::units;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;

use super::drawingml::color::Color;
use super::drawingml::fill::{FillKind, FillProperties};
use super::drawingml::line::{LineFill, LineProperties};
use super::drawingml::shape::{
  FontStyleReference, GraphicDataKind, GraphicDataRecord, Shape, ShapeService,
};
use super::drawingml::table::{
  TableCell, TableCellBorders, TableProperties, TableStyle, TableStyleBorders, TableStylePart,
  TableStyleTextProperties, predefined_table_style,
};
use super::drawingml::text_body::{TextBody, TextParagraph, TextRun, TextRunKind};
use super::drawingml::text_list_style::{TextListLevelParagraphProperties, TextListParagraphStyle};
use super::import::PowerPointImport;
use super::slide::{BackgroundKind, BackgroundProperties, SlidePersist};

const DEFAULT_TEXT_FONT_SIZE_PT: f32 = 18.0;
const DEFAULT_TEXT_LINE_HEIGHT_SCALE: f32 = 1.2;
const DEFAULT_TEXT_INSET_EMU: i64 = 91_440;
const DEFAULT_BULLET_INDENT_PT: f32 = 18.0;
const DEFAULT_TABLE_BORDER_PT: f32 = 0.75;

pub(crate) fn lower_to_layout_document(import: &PowerPointImport) -> layout::LayoutDocument {
  let pages = import
    .draw_pages
    .iter()
    .map(|slide| (slide.size.to_page_setup(), lower_slide_items(import, slide)))
    .collect();
  layout::fixed_pages_with_items(pages)
}

fn lower_slide_items(import: &PowerPointImport, slide: &SlidePersist) -> Vec<PageItem> {
  let mut items = Vec::new();
  let _has_structured_comment_identity = slide.comments.iter().any(|comment| comment.has_payload())
    || slide
      .comment_authors
      .iter()
      .any(|author| author.has_payload());
  let _has_header_footer_identity = slide.header_footer.has_visible_slot();
  let master_page = slide
    .master_page_index
    .and_then(|master_page_index| import.master_pages.get(master_page_index));
  if let Some(background) = slide
    .background_properties
    .as_ref()
    .or_else(|| master_page.and_then(|master_page| master_page.background_properties.as_ref()))
  {
    lower_background(import, slide, background, &mut items);
  }
  lower_shapes(import, &slide.shapes, &mut items);
  items
}

fn lower_background(
  import: &PowerPointImport,
  slide: &SlidePersist,
  background: &BackgroundProperties,
  items: &mut Vec<PageItem>,
) {
  let fill_properties = match &background.kind {
    BackgroundKind::Properties(fill_properties) => Some(
      fill_properties
        .clone()
        .with_placeholder_color(slide.background_color.clone()),
    ),
    BackgroundKind::StyleReference {
      style_index,
      placeholder_color,
    } => import.get_theme_fill_style(*style_index).map(|fill| {
      fill.with_placeholder_color(
        placeholder_color
          .clone()
          .or_else(|| slide.background_color.clone()),
      )
    }),
  };
  let Some(fill_properties) = fill_properties else {
    return;
  };
  let Some(fill_paint) = fill_paint(import, &fill_properties) else {
    return;
  };
  items.push(PageItem::Rect(RectItem {
    x_pt: 0.0,
    y_pt: 0.0,
    width_pt: slide.size.width_pt,
    height_pt: slide.size.height_pt,
    fill_color: Some(fill_paint.color),
    fill_opacity: fill_paint.opacity,
    stroke: None,
    stroke_opacity: 1.0,
  }));
}

fn lower_shapes(import: &PowerPointImport, shapes: &[Shape], items: &mut Vec<PageItem>) {
  for shape in shapes {
    lower_shape(import, shape, DisplayOffset::default(), items);
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct DisplayOffset {
  x_emu: i64,
  y_emu: i64,
}

fn lower_shape(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  if shape.hidden || shape.hidden_master_shape || shape.referenced {
    return;
  }

  lower_shape_bounds(import, shape, offset, items);
  lower_picture(shape, offset, items);
  lower_shape_hyperlink(shape, offset, items);
  let _has_structured_media_identity = shape.media.as_ref().is_some_and(|media| {
    !matches!(media.kind, super::drawingml::shape::MediaKind::Unknown)
      || media.embed_relationship_id.is_some()
      || media.link_relationship_id.is_some()
      || media
        .resource
        .as_ref()
        .is_some_and(|resource| resource.has_payload())
  });
  let _has_structured_content_part_identity =
    shape.content_part.as_ref().is_some_and(|content_part| {
      !content_part.relationship_id.is_empty()
        || content_part
          .resource
          .as_ref()
          .is_some_and(|resource| resource.has_payload())
    });
  let _has_structured_graphic_identity = shape
    .graphic_data
    .as_ref()
    .is_some_and(graphic_data_has_structured_identity);

  if let Some(table) = &shape.table_properties
    && shape.service_name == ShapeService::TableShape
  {
    lower_table(import, shape, offset, table, items);
  }

  if let Some(text_body) = &shape.text_body {
    lower_text_body(import, shape, offset, text_body, items);
  }

  let child_offset = child_display_offset(shape, offset);
  for child in &shape.children {
    lower_shape(import, child, child_offset, items);
  }
}

fn graphic_data_has_structured_identity(record: &GraphicDataRecord) -> bool {
  !record.uri.is_empty()
    || !matches!(record.kind, GraphicDataKind::Unsupported)
    || record.chart_relationship_id.is_some()
    || record
      .chart_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .extended_chart_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record.has_inline_chart_space
    || record.diagram_relationship_ids.as_ref().is_some_and(|ids| {
      !ids.data_part.is_empty()
        || !ids.layout_part.is_empty()
        || !ids.style_part.is_empty()
        || !ids.color_part.is_empty()
    })
    || record
      .diagram_data_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_layout_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_style_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .diagram_color_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record.ole_object.as_ref().is_some_and(|ole| {
      ole.relationship_id.is_some()
        || ole.name.is_some()
        || ole.prog_id.is_some()
        || ole.show_as_icon
    })
    || record
      .ole_binary_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
    || record
      .embedded_package_resource
      .as_ref()
      .is_some_and(|resource| resource.has_payload())
}

fn lower_picture(shape: &Shape, offset: DisplayOffset, items: &mut Vec<PageItem>) {
  let Some(picture) = &shape.picture else {
    return;
  };
  if shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  let _embed_relationship_id = picture.embed_relationship_id.as_deref();
  let _link_relationship_id = picture.link_relationship_id.as_deref();
  let Some(resource) = picture.image_resource.as_ref() else {
    return;
  };
  items.push(PageItem::Image(ImageItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    crop: picture.crop,
    rotation_deg: shape.rotation,
    flip_horizontal: shape.flip_h,
    flip_vertical: shape.flip_v,
    data: resource.data.clone(),
    content_type: resource.content_type.clone(),
    alt_text: shape
      .description
      .clone()
      .or_else(|| shape.title.clone())
      .or_else(|| shape.name.clone()),
    hyperlink_url: shape.hyperlink_url.clone(),
    floating: false,
    behind_text: false,
  }));
}

fn lower_shape_hyperlink(shape: &Shape, offset: DisplayOffset, items: &mut Vec<PageItem>) {
  let Some(hyperlink_url) = &shape.hyperlink_url else {
    return;
  };
  if shape.service_name == ShapeService::GroupShape || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }
  items.push(PageItem::LinkArea(LinkAreaItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    hyperlink_url: hyperlink_url.clone(),
  }));
}

fn lower_table(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  table: &TableProperties,
  items: &mut Vec<PageItem>,
) {
  // Source: LibreOffice oox/source/drawingml/shape.cxx uses the DrawingML
  // table grid and row heights as the visible TableShape size.
  let x0 = units::emu_to_points(offset.x_emu + shape.position.x);
  let y0 = units::emu_to_points(offset.y_emu + shape.position.y);
  let table_width = units::emu_to_points(table.grid.iter().copied().sum::<i64>());
  let table_height = units::emu_to_points(table.rows.iter().map(|row| row.height).sum::<i64>());
  if table_width <= 0.0 || table_height <= 0.0 {
    return;
  }

  let package_table_style = table
    .inline_style
    .as_ref()
    .or_else(|| import.get_table_style(table.style_id.as_deref()));
  let predefined_table_style = if package_table_style.is_none() {
    predefined_table_style(table.style_id.as_deref())
  } else {
    None
  };
  let table_style = package_table_style.or(predefined_table_style.as_ref());
  let table_background = table_style.and_then(|style| {
    table_style_part_fill(import, &style.table_background)
      .and_then(|fill| fill_paint(import, &fill))
  });
  let border_color = RgbColor { r: 0, g: 0, b: 0 };
  let draw_fallback_grid = table_style.is_none() && !table_has_visible_direct_borders(table);
  if draw_fallback_grid {
    push_table_line(items, x0, y0, x0 + table_width, y0, border_color);
    push_table_line(items, x0, y0, x0, y0 + table_height, border_color);
  }

  let mut y = y0;
  let max_row = table.rows.len().saturating_sub(1);
  let max_column = table.grid.len().saturating_sub(1);
  for (row_index, row) in table.rows.iter().enumerate() {
    let row_height = units::emu_to_points(row.height);
    let mut x = x0;
    let mut grid_index = 0usize;
    for cell in &row.cells {
      let span = table_cell_grid_span(cell);
      let width_emu = if grid_index < table.grid.len() {
        table.grid[grid_index..table.grid.len().min(grid_index + span)]
          .iter()
          .copied()
          .sum::<i64>()
      } else {
        0
      };
      let cell_width = units::emu_to_points(width_emu);
      if !cell.horizontal_merge && !cell.vertical_merge {
        let style_part = table_style.map(|style| {
          table_cell_style_part(
            import, table, style, grid_index, max_column, row_index, max_row,
          )
        });
        lower_table_cell(
          import,
          cell,
          style_part.as_ref(),
          table_background,
          x,
          y,
          cell_width,
          row_height,
          items,
        );
      }
      x += cell_width;
      grid_index = grid_index.saturating_add(span);
    }
    y += row_height;
    if draw_fallback_grid {
      push_table_line(items, x0, y, x0 + table_width, y, border_color);
    }
  }

  if draw_fallback_grid {
    let mut x = x0;
    for width in &table.grid {
      x += units::emu_to_points(*width);
      push_table_line(items, x, y0, x, y0 + table_height, border_color);
    }
  }
}

fn table_cell_style_part(
  import: &PowerPointImport,
  table: &TableProperties,
  style: &TableStyle,
  column: usize,
  max_column: usize,
  row: usize,
  max_row: usize,
) -> TableStylePart {
  // Source: LibreOffice tablecell.cxx applies table style parts in a fixed
  // order: whole table, first/last row/column, horizontal banding, corners,
  // then vertical banding. Direct tcPr is merged afterwards by the caller.
  let mut result = TableStylePart::default();
  merge_style_part(
    import,
    &mut result,
    &style.whole_table,
    true,
    column,
    max_column,
    row,
    max_row,
  );
  if table.first_row && row == 0 {
    merge_style_part(
      import,
      &mut result,
      &style.first_row,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.last_row && row == max_row {
    merge_style_part(
      import,
      &mut result,
      &style.last_row,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.first_column && column == 0 {
    merge_style_part(
      import,
      &mut result,
      &style.first_column,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.last_column && column == max_column {
    merge_style_part(
      import,
      &mut result,
      &style.last_column,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.band_row
    && (!table.first_row || row != 0)
    && (!table.last_row || row != max_row)
    && (!table.first_column || column != 0 || !table_style_part_has_fill(&style.first_column))
    && (!table.last_column
      || column != max_column
      || !table_style_part_has_fill(&style.last_column))
  {
    let band = row + usize::from(table.first_row);
    let part = if band & 1 == 1 {
      &style.band2_horizontal
    } else {
      &style.band1_horizontal
    };
    merge_style_part(
      import,
      &mut result,
      part,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if row == 0 && column == 0 {
    merge_style_part(
      import,
      &mut result,
      &style.northwest_cell,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if row == max_row && column == 0 {
    merge_style_part(
      import,
      &mut result,
      &style.southwest_cell,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if row == 0 && column == max_column {
    merge_style_part(
      import,
      &mut result,
      &style.northeast_cell,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if row == max_row && column == max_column {
    merge_style_part(
      import,
      &mut result,
      &style.southeast_cell,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  if table.band_column
    && (!table.first_row || row != 0)
    && (!table.last_row || row != max_row)
    && (!table.first_column || column != 0)
    && (!table.last_column || column != max_column)
  {
    let band = column + usize::from(table.first_column);
    let part = if band & 1 == 1 {
      &style.band2_vertical
    } else {
      &style.band1_vertical
    };
    merge_style_part(
      import,
      &mut result,
      part,
      false,
      column,
      max_column,
      row,
      max_row,
    );
  }
  result
}

fn merge_style_part(
  import: &PowerPointImport,
  target: &mut TableStylePart,
  source: &TableStylePart,
  whole_table: bool,
  column: usize,
  max_column: usize,
  row: usize,
  max_row: usize,
) {
  if let Some(fill) = table_style_part_fill(import, source) {
    target.fill_properties = Some(fill);
  }
  let mut borders = TableCellBorders::default();
  merge_style_borders(
    import,
    &mut borders,
    &source.borders,
    whole_table,
    column,
    max_column,
    row,
    max_row,
  );
  merge_cell_borders_from_style(&mut target.borders, &borders);
  target.text.merge_from(&source.text);
}

fn table_style_part_has_fill(part: &TableStylePart) -> bool {
  part.fill_properties.is_some() || part.fill_reference.is_some()
}

fn table_style_part_fill(
  import: &PowerPointImport,
  part: &TableStylePart,
) -> Option<FillProperties> {
  part.fill_properties.clone().or_else(|| {
    part.fill_reference.as_ref().and_then(|reference| {
      import
        .get_theme_fill_style(reference.index)
        .map(|fill| fill.with_placeholder_color(reference.placeholder_color.clone()))
    })
  })
}

fn merge_style_borders(
  import: &PowerPointImport,
  target: &mut TableCellBorders,
  source: &TableStyleBorders,
  whole_table: bool,
  column: usize,
  max_column: usize,
  row: usize,
  max_row: usize,
) {
  if (!whole_table || column == 0)
    && let Some(line) = table_style_border_line(import, &source.left, &source.left_reference)
  {
    target.left = Some(line);
  }
  if (!whole_table || column >= max_column)
    && let Some(line) = table_style_border_line(import, &source.right, &source.right_reference)
  {
    target.right = Some(line);
  }
  if (!whole_table || row == 0)
    && let Some(line) = table_style_border_line(import, &source.top, &source.top_reference)
  {
    target.top = Some(line);
  }
  if (!whole_table || row >= max_row)
    && let Some(line) = table_style_border_line(import, &source.bottom, &source.bottom_reference)
  {
    target.bottom = Some(line);
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.inside_horizontal,
    &source.inside_horizontal_reference,
  ) {
    if row != 0 {
      target.top = Some(line.clone());
    }
    if row != max_row {
      target.bottom = Some(line);
    }
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.inside_vertical,
    &source.inside_vertical_reference,
  ) {
    if column != 0 {
      target.left = Some(line.clone());
    }
    if column != max_column {
      target.right = Some(line);
    }
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.top_left_to_bottom_right,
    &source.top_left_to_bottom_right_reference,
  ) {
    target.top_left_to_bottom_right = Some(line);
  }
  if let Some(line) = table_style_border_line(
    import,
    &source.bottom_left_to_top_right,
    &source.bottom_left_to_top_right_reference,
  ) {
    target.bottom_left_to_top_right = Some(line);
  }
}

fn table_style_border_line(
  import: &PowerPointImport,
  direct: &Option<LineProperties>,
  reference: &Option<super::drawingml::shape::ShapeStyleReference>,
) -> Option<LineProperties> {
  direct.clone().or_else(|| {
    reference.as_ref().and_then(|reference| {
      import
        .get_theme_line_style(reference.index)
        .map(|line| line.with_placeholder_color(reference.placeholder_color.clone()))
    })
  })
}

fn merge_cell_borders_from_style(target: &mut TableStyleBorders, source: &TableCellBorders) {
  if source.left.is_some() {
    target.left = source.left.clone();
  }
  if source.right.is_some() {
    target.right = source.right.clone();
  }
  if source.top.is_some() {
    target.top = source.top.clone();
  }
  if source.bottom.is_some() {
    target.bottom = source.bottom.clone();
  }
  if source.top_left_to_bottom_right.is_some() {
    target.top_left_to_bottom_right = source.top_left_to_bottom_right.clone();
  }
  if source.bottom_left_to_top_right.is_some() {
    target.bottom_left_to_top_right = source.bottom_left_to_top_right.clone();
  }
}

fn table_has_visible_direct_borders(table: &TableProperties) -> bool {
  table.rows.iter().any(|row| {
    row.cells.iter().any(|cell| {
      table_border_line_is_visible(&cell.borders.left)
        || table_border_line_is_visible(&cell.borders.right)
        || table_border_line_is_visible(&cell.borders.top)
        || table_border_line_is_visible(&cell.borders.bottom)
        || table_border_line_is_visible(&cell.borders.top_left_to_bottom_right)
        || table_border_line_is_visible(&cell.borders.bottom_left_to_top_right)
    })
  })
}

fn table_border_line_is_visible(line: &Option<LineProperties>) -> bool {
  matches!(
    line.as_ref().map(|line| &line.fill),
    Some(LineFill::Solid(_) | LineFill::Gradient(_) | LineFill::Pattern(_))
  )
}

fn lower_table_cell(
  import: &PowerPointImport,
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
  table_background: Option<DisplayPaint>,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  items: &mut Vec<PageItem>,
) {
  if width <= 0.0 || height <= 0.0 {
    return;
  }
  let fill = table_cell_fill_paint(import, cell, style_part, table_background);
  if let Some(fill) = fill {
    items.push(PageItem::Rect(RectItem {
      x_pt: x,
      y_pt: y,
      width_pt: width,
      height_pt: height,
      fill_color: Some(fill.color),
      fill_opacity: fill.opacity,
      stroke: None,
      stroke_opacity: 1.0,
    }));
  }
  let borders = table_cell_effective_borders(cell, style_part);
  lower_table_cell_borders(import, &borders, x, y, width, height, items);

  if let Some(text_body) = &cell.text_body {
    let mut text_body = text_body.clone();
    text_body.display_properties.vertical = cell.vertical;
    text_body.display_properties.anchor = cell.anchor;
    text_body.display_properties.anchor_center = cell.anchor_center;
    text_body.display_properties.horizontal_overflow = Some(cell.horizontal_overflow);
    let x = x + units::emu_to_points(i64::from(cell.margins.left));
    let y = y + units::emu_to_points(i64::from(cell.margins.top));
    lower_text_body_at_with_table_style(
      import,
      TextFrame {
        x_pt: x,
        y_pt: y,
        width_pt: (width - units::emu_to_points(i64::from(cell.margins.left + cell.margins.right)))
          .max(0.0),
        height_pt: (height
          - units::emu_to_points(i64::from(cell.margins.top + cell.margins.bottom)))
        .max(0.0),
      },
      &text_body,
      style_part.map(|style| &style.text),
      items,
    );
  }
}

fn table_cell_fill_paint(
  import: &PowerPointImport,
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
  table_background: Option<DisplayPaint>,
) -> Option<DisplayPaint> {
  let cell_fill = cell
    .fill_properties
    .as_ref()
    .map(|fill| fill_paint(import, fill))
    .unwrap_or_else(|| {
      style_part
        .and_then(|style| style.fill_properties.as_ref())
        .and_then(|fill| fill_paint(import, fill))
    });
  match (table_background, cell_fill) {
    (Some(background), Some(cell)) => Some(blend_table_cell_fill(background, cell)),
    (Some(background), None) => Some(background),
    (None, Some(cell)) => Some(cell),
    (None, None) => None,
  }
}

fn blend_table_cell_fill(background: DisplayPaint, cell: DisplayPaint) -> DisplayPaint {
  // Source: LibreOffice tablecell.cxx blends table background and cell fill
  // through basegfx::interpolate(bg, cell, 1 - cellTransparency).
  let cell_weight = cell.opacity.clamp(0.0, 1.0);
  let background_weight = 1.0 - cell_weight;
  DisplayPaint {
    color: RgbColor {
      r: blend_channel(
        background.color.r,
        cell.color.r,
        background_weight,
        cell_weight,
      ),
      g: blend_channel(
        background.color.g,
        cell.color.g,
        background_weight,
        cell_weight,
      ),
      b: blend_channel(
        background.color.b,
        cell.color.b,
        background_weight,
        cell_weight,
      ),
    },
    opacity: background.opacity.max(cell.opacity).clamp(0.0, 1.0),
  }
}

fn blend_channel(background: u8, cell: u8, background_weight: f32, cell_weight: f32) -> u8 {
  (f32::from(background) * background_weight + f32::from(cell) * cell_weight)
    .round()
    .clamp(0.0, 255.0) as u8
}

fn lower_table_cell_borders(
  import: &PowerPointImport,
  borders: &TableCellBorders,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
  items: &mut Vec<PageItem>,
) {
  push_table_border_line(import, &borders.top, x, y, x + width, y, items);
  push_table_border_line(
    import,
    &borders.bottom,
    x,
    y + height,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(import, &borders.left, x, y, x, y + height, items);
  push_table_border_line(
    import,
    &borders.right,
    x + width,
    y,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &borders.top_left_to_bottom_right,
    x,
    y,
    x + width,
    y + height,
    items,
  );
  push_table_border_line(
    import,
    &borders.bottom_left_to_top_right,
    x,
    y + height,
    x + width,
    y,
    items,
  );
}

fn table_cell_effective_borders(
  cell: &TableCell,
  style_part: Option<&TableStylePart>,
) -> TableCellBorders {
  let mut borders = TableCellBorders::default();
  if let Some(style_part) = style_part {
    borders.left = style_part.borders.left.clone();
    borders.right = style_part.borders.right.clone();
    borders.top = style_part.borders.top.clone();
    borders.bottom = style_part.borders.bottom.clone();
    borders.top_left_to_bottom_right = style_part.borders.top_left_to_bottom_right.clone();
    borders.bottom_left_to_top_right = style_part.borders.bottom_left_to_top_right.clone();
  }
  if cell.borders.left.is_some() {
    borders.left = cell.borders.left.clone();
  }
  if cell.borders.right.is_some() {
    borders.right = cell.borders.right.clone();
  }
  if cell.borders.top.is_some() {
    borders.top = cell.borders.top.clone();
  }
  if cell.borders.bottom.is_some() {
    borders.bottom = cell.borders.bottom.clone();
  }
  if cell.borders.top_left_to_bottom_right.is_some() {
    borders.top_left_to_bottom_right = cell.borders.top_left_to_bottom_right.clone();
  }
  if cell.borders.bottom_left_to_top_right.is_some() {
    borders.bottom_left_to_top_right = cell.borders.bottom_left_to_top_right.clone();
  }
  borders
}

fn push_table_border_line(
  import: &PowerPointImport,
  line: &Option<LineProperties>,
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  items: &mut Vec<PageItem>,
) {
  let Some(stroke) = line.as_ref().and_then(|line| line_stroke(import, line)) else {
    return;
  };
  items.push(PageItem::Line(LineItem {
    x1_pt,
    y1_pt,
    x2_pt,
    y2_pt,
    width_pt: stroke.style.width_pt,
    color: stroke.style.color,
    kind: LineItemKind::Stroke,
  }));
}

fn table_cell_grid_span(cell: &TableCell) -> usize {
  cell
    .grid_span
    .and_then(|span| usize::try_from(span).ok())
    .filter(|span| *span > 0)
    .unwrap_or(1)
}

fn push_table_line(
  items: &mut Vec<PageItem>,
  x1_pt: f32,
  y1_pt: f32,
  x2_pt: f32,
  y2_pt: f32,
  color: RgbColor,
) {
  items.push(PageItem::Line(LineItem {
    x1_pt,
    y1_pt,
    x2_pt,
    y2_pt,
    width_pt: DEFAULT_TABLE_BORDER_PT,
    color,
    kind: LineItemKind::Stroke,
  }));
}

fn lower_shape_bounds(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  items: &mut Vec<PageItem>,
) {
  if shape.service_name == ShapeService::GroupShape || shape.size.cx <= 0 || shape.size.cy <= 0 {
    return;
  }

  let fill_paint = shape
    .actual_fill_properties
    .as_ref()
    .and_then(|fill| fill_paint(import, fill));
  let line = shape
    .actual_line_properties
    .as_ref()
    .and_then(|line| line_stroke(import, line));
  if fill_paint.is_none() && line.is_none() {
    return;
  }
  let (stroke, stroke_opacity) = line
    .map(|line| (Some(line.style), line.opacity))
    .unwrap_or((None, 1.0));

  items.push(PageItem::Rect(RectItem {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x),
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y),
    width_pt: units::emu_to_points(shape.size.cx),
    height_pt: units::emu_to_points(shape.size.cy),
    fill_color: fill_paint.map(|fill| fill.color),
    fill_opacity: fill_paint.map(|fill| fill.opacity).unwrap_or(1.0),
    stroke,
    stroke_opacity,
  }));
}

fn child_display_offset(shape: &Shape, offset: DisplayOffset) -> DisplayOffset {
  DisplayOffset {
    x_emu: offset.x_emu + shape.position.x - shape.child_position.x,
    y_emu: offset.y_emu + shape.position.y - shape.child_position.y,
  }
}

fn lower_text_body(
  import: &PowerPointImport,
  shape: &Shape,
  offset: DisplayOffset,
  text_body: &TextBody,
  items: &mut Vec<PageItem>,
) {
  let text_box = text_box_metrics(shape, offset, text_body);
  lower_text_body_at_with_font_ref(
    import,
    text_box,
    text_body,
    shape
      .shape_style_refs
      .as_ref()
      .map(|style| &style.font_reference),
    shape.hyperlink_url.as_deref(),
    items,
  );
}

fn lower_text_body_at_with_table_style(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  table_text_style: Option<&TableStyleTextProperties>,
  items: &mut Vec<PageItem>,
) {
  lower_text_body_at_with_style(
    import,
    frame,
    text_body,
    None,
    table_text_style,
    None,
    items,
  );
}

fn lower_text_body_at_with_font_ref(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  font_reference: Option<&FontStyleReference>,
  shape_hyperlink_url: Option<&str>,
  items: &mut Vec<PageItem>,
) {
  lower_text_body_at_with_style(
    import,
    frame,
    text_body,
    font_reference,
    None,
    shape_hyperlink_url,
    items,
  );
}

fn lower_text_body_at_with_style(
  import: &PowerPointImport,
  frame: TextFrame,
  text_body: &TextBody,
  font_reference: Option<&FontStyleReference>,
  table_text_style: Option<&TableStyleTextProperties>,
  shape_hyperlink_url: Option<&str>,
  items: &mut Vec<PageItem>,
) {
  let options = TextLoweringOptions::from_text_body(text_body);
  let mut base_style = TextStyle {
    font_family: Some(Arc::from("Liberation Sans")),
    font_size_pt: DEFAULT_TEXT_FONT_SIZE_PT * options.font_scale,
    rotation_deg: options.rotation_deg,
    ..TextStyle::default()
  };
  if let Some(font_reference) = font_reference {
    apply_font_reference_text_style(import, font_reference, &mut base_style);
  }
  if let Some(table_text_style) = table_text_style {
    apply_table_text_style(import, table_text_style, &mut base_style);
  }

  let estimated_height = estimate_text_body_height(text_body, &base_style, options.line_scale);
  let y_pt = match text_body.display_properties.anchor {
    a::TextAnchoringTypeValues::Center => {
      frame.y_pt + ((frame.height_pt - estimated_height) / 2.0).max(0.0)
    }
    a::TextAnchoringTypeValues::Bottom => {
      frame.y_pt + (frame.height_pt - estimated_height).max(0.0)
    }
    a::TextAnchoringTypeValues::Top => frame.y_pt,
  };

  let mut cursor = TextCursor {
    x_pt: frame.x_pt,
    y_pt,
    column_index: 0,
  };
  for paragraph in &text_body.paragraphs {
    lower_paragraph(
      import,
      paragraph,
      &base_style,
      &options,
      frame,
      shape_hyperlink_url,
      &mut cursor,
      items,
    );
  }
}

fn apply_font_reference_text_style(
  import: &PowerPointImport,
  reference: &FontStyleReference,
  style: &mut TextStyle,
) {
  if let Some(typeface) = import.get_theme_latin_font(reference.index) {
    style.font_family = Some(Arc::from(typeface));
  }
  if let Some(paint) = reference
    .placeholder_color
    .as_ref()
    .and_then(|color| display_paint(import, color, None))
  {
    style.color = paint.color;
    style.opacity = paint.opacity;
  }
}

fn apply_table_text_style(
  import: &PowerPointImport,
  properties: &TableStyleTextProperties,
  style: &mut TextStyle,
) {
  if let Some(font_reference) = &properties.font_reference {
    apply_font_reference_text_style(import, font_reference, style);
  }
  if let Some(typeface) = properties.fonts.latin.as_deref() {
    style.font_family = Some(Arc::from(typeface));
  }
  if let Some(bold) = properties.bold.and_then(boolean_style_value) {
    style.bold = bold;
  }
  if let Some(italic) = properties.italic.and_then(boolean_style_value) {
    style.italic = italic;
  }
  if let Some(paint) = properties
    .color
    .as_ref()
    .and_then(|color| display_paint(import, color, None))
  {
    style.color = paint.color;
    style.opacity = paint.opacity;
  }
}

fn boolean_style_value(value: a::BooleanStyleValues) -> Option<bool> {
  match value {
    a::BooleanStyleValues::On => Some(true),
    a::BooleanStyleValues::Off => Some(false),
    a::BooleanStyleValues::Default => None,
  }
}

#[derive(Clone, Copy, Debug)]
struct TextFrame {
  x_pt: f32,
  y_pt: f32,
  width_pt: f32,
  height_pt: f32,
}

#[derive(Clone, Copy, Debug)]
struct TextCursor {
  x_pt: f32,
  y_pt: f32,
  column_index: usize,
}

#[derive(Clone, Copy, Debug)]
struct TextLoweringOptions {
  font_scale: f32,
  line_scale: f32,
  rotation_deg: f32,
  column_count: usize,
  column_spacing_pt: f32,
  clip_vertical_overflow: bool,
}

impl TextLoweringOptions {
  fn from_text_body(text_body: &TextBody) -> Self {
    Self {
      font_scale: text_body.display_properties.font_scale(),
      line_scale: text_body.display_properties.line_height_scale(),
      rotation_deg: text_body.display_properties.rotation_degrees(),
      column_count: text_body.display_properties.column_count.max(1),
      column_spacing_pt: units::emu_to_points(text_body.display_properties.column_spacing_emu),
      clip_vertical_overflow: text_body.display_properties.clip_vertical_overflow,
    }
  }

  fn column_width(self, frame: TextFrame) -> f32 {
    if self.column_count <= 1 {
      frame.width_pt
    } else {
      let total_spacing = self.column_spacing_pt * (self.column_count - 1) as f32;
      ((frame.width_pt - total_spacing) / self.column_count as f32).max(0.0)
    }
  }
}

fn text_box_metrics(shape: &Shape, offset: DisplayOffset, text_body: &TextBody) -> TextFrame {
  let body_properties = text_body.body_properties.as_deref();
  let left_inset = body_properties
    .and_then(|properties| properties.left_inset)
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or_else(|| units::emu_to_points(DEFAULT_TEXT_INSET_EMU));
  let top_inset = body_properties
    .and_then(|properties| properties.top_inset)
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or_else(|| units::emu_to_points(DEFAULT_TEXT_INSET_EMU));
  let right_inset = body_properties
    .and_then(|properties| properties.right_inset)
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or_else(|| units::emu_to_points(DEFAULT_TEXT_INSET_EMU));
  let bottom_inset = body_properties
    .and_then(|properties| properties.bottom_inset)
    .map(|value| units::emu_to_points(value.to_emu()))
    .unwrap_or_else(|| units::emu_to_points(DEFAULT_TEXT_INSET_EMU));

  TextFrame {
    x_pt: units::emu_to_points(offset.x_emu + shape.position.x) + left_inset,
    y_pt: units::emu_to_points(offset.y_emu + shape.position.y) + top_inset,
    width_pt: (units::emu_to_points(shape.size.cx) - left_inset - right_inset).max(0.0),
    height_pt: (units::emu_to_points(shape.size.cy) - top_inset - bottom_inset).max(0.0),
  }
}

fn lower_paragraph(
  import: &PowerPointImport,
  paragraph: &TextParagraph,
  base_style: &TextStyle,
  options: &TextLoweringOptions,
  frame: TextFrame,
  shape_hyperlink_url: Option<&str>,
  cursor: &mut TextCursor,
  items: &mut Vec<PageItem>,
) {
  let paragraph_style = ParagraphDisplayStyle::from_paragraph(paragraph);
  let column_width = options.column_width(frame);
  let column_x =
    frame.x_pt + cursor.column_index as f32 * (column_width + options.column_spacing_pt);
  cursor.x_pt = column_x;
  if options.clip_vertical_overflow && cursor.y_pt > frame.y_pt + frame.height_pt {
    return;
  }
  let paragraph_x = cursor.x_pt + paragraph_style.left_margin_pt + paragraph_style.indent_pt;
  let mut segment_start = 0usize;
  let mut is_first_segment = true;

  loop {
    let segment_end = paragraph.runs[segment_start..]
      .iter()
      .position(|run| run.kind == TextRunKind::Break)
      .map(|offset| segment_start + offset)
      .unwrap_or(paragraph.runs.len());
    let line_width = paragraph_run_width(
      import,
      &paragraph_style,
      base_style,
      &paragraph.runs[segment_start..segment_end],
    );
    let mut run_x = aligned_paragraph_x(
      paragraph_x,
      column_width,
      line_width,
      paragraph_style.alignment,
    );
    let mut max_line_height = line_height(base_style, options.line_scale);

    if is_first_segment && let Some(label) = paragraph_style.bullet_label(paragraph) {
      let mut bullet_style = base_style.clone();
      paragraph_style.apply_default_run_style(import, &mut bullet_style);
      max_line_height = max_line_height.max(line_height(&bullet_style, options.line_scale));
      push_text_item(
        items,
        run_x - DEFAULT_BULLET_INDENT_PT,
        cursor.y_pt,
        label,
        bullet_style,
        shape_hyperlink_url.map(ToString::to_string),
        options.line_scale,
      );
    }

    for run in &paragraph.runs[segment_start..segment_end] {
      if !matches!(run.kind, TextRunKind::Run | TextRunKind::Field) || run.text.is_empty() {
        continue;
      }
      let mut style = base_style.clone();
      paragraph_style.apply_default_run_style(import, &mut style);
      apply_run_properties(import, run, &mut style);
      max_line_height = max_line_height.max(line_height(&style, options.line_scale));
      let text = run_text(run, &style);
      push_text_item(
        items,
        run_x,
        cursor.y_pt,
        text.clone(),
        style.clone(),
        run
          .hyperlink_url
          .clone()
          .or_else(|| shape_hyperlink_url.map(ToString::to_string)),
        options.line_scale,
      );
      run_x += measure_text(&text, &style);
    }

    cursor.y_pt += max_line_height;
    advance_text_column_if_needed(cursor, frame, *options);
    if segment_end == paragraph.runs.len() {
      break;
    }
    segment_start = segment_end + 1;
    is_first_segment = false;
  }
}

fn paragraph_run_width(
  import: &PowerPointImport,
  paragraph_style: &ParagraphDisplayStyle,
  base_style: &TextStyle,
  runs: &[TextRun],
) -> f32 {
  runs
    .iter()
    .filter(|run| matches!(run.kind, TextRunKind::Run | TextRunKind::Field) && !run.text.is_empty())
    .map(|run| {
      let mut style = base_style.clone();
      paragraph_style.apply_default_run_style(import, &mut style);
      apply_run_properties(import, run, &mut style);
      measure_text(&run_text(run, &style), &style)
    })
    .sum()
}

fn aligned_paragraph_x(
  paragraph_x: f32,
  column_width: f32,
  line_width: f32,
  alignment: a::TextAlignmentTypeValues,
) -> f32 {
  match alignment {
    a::TextAlignmentTypeValues::Center => {
      paragraph_x + ((column_width - line_width) / 2.0).max(0.0)
    }
    a::TextAlignmentTypeValues::Right => paragraph_x + (column_width - line_width).max(0.0),
    a::TextAlignmentTypeValues::Left
    | a::TextAlignmentTypeValues::Justified
    | a::TextAlignmentTypeValues::JustifiedLow
    | a::TextAlignmentTypeValues::Distributed
    | a::TextAlignmentTypeValues::ThaiDistributed => paragraph_x,
  }
}

fn push_text_item(
  items: &mut Vec<PageItem>,
  x_pt: f32,
  y_pt: f32,
  text: String,
  style: TextStyle,
  hyperlink_url: Option<String>,
  line_scale: f32,
) {
  items.push(PageItem::Text(TextItem {
    x_pt,
    y_pt,
    line_height_pt: line_height(&style, line_scale),
    text,
    style,
    hyperlink_url,
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

fn line_height(style: &TextStyle, line_scale: f32) -> f32 {
  style.font_size_pt * DEFAULT_TEXT_LINE_HEIGHT_SCALE * line_scale
}

fn estimate_text_body_height(text_body: &TextBody, base_style: &TextStyle, line_scale: f32) -> f32 {
  let mut lines = 0usize;
  for paragraph in &text_body.paragraphs {
    lines += paragraph
      .runs
      .iter()
      .filter(|run| run.kind == TextRunKind::Break)
      .count()
      + 1;
  }
  lines as f32 * line_height(base_style, line_scale)
}

fn advance_text_column_if_needed(
  cursor: &mut TextCursor,
  frame: TextFrame,
  options: TextLoweringOptions,
) {
  if options.column_count <= 1 || cursor.y_pt <= frame.y_pt + frame.height_pt {
    return;
  }
  if cursor.column_index + 1 >= options.column_count {
    return;
  }
  cursor.column_index += 1;
  cursor.y_pt = frame.y_pt;
}

fn run_text(run: &TextRun, style: &TextStyle) -> String {
  if style.uppercase {
    run.text.to_uppercase()
  } else {
    run.text.clone()
  }
}

#[derive(Clone, Debug)]
struct ParagraphDisplayStyle {
  left_margin_pt: f32,
  indent_pt: f32,
  alignment: a::TextAlignmentTypeValues,
  bullet: Option<String>,
  default_run_properties: Option<Box<a::DefaultRunProperties>>,
}

impl Default for ParagraphDisplayStyle {
  fn default() -> Self {
    Self {
      left_margin_pt: 0.0,
      indent_pt: 0.0,
      alignment: a::TextAlignmentTypeValues::Left,
      bullet: None,
      default_run_properties: None,
    }
  }
}

impl ParagraphDisplayStyle {
  fn from_paragraph(paragraph: &TextParagraph) -> Self {
    let mut style = Self::default();
    if let Some(master_style) = &paragraph.master_paragraph_style {
      style.apply_text_list_style(master_style);
    }
    if let Some(text_style) = &paragraph.text_paragraph_style {
      style.apply_text_list_style(text_style);
    }
    if let Some(properties) = paragraph.paragraph_properties.as_deref() {
      if let Some(left_margin) = properties.left_margin {
        style.left_margin_pt = units::emu_to_points(i64::from(left_margin));
      }
      if let Some(indent) = properties.indent {
        style.indent_pt = units::emu_to_points(i64::from(indent));
      }
      if let Some(default_run_properties) = &properties.default_run_properties {
        style.default_run_properties = Some(default_run_properties.clone());
      }
      if let Some(alignment) = properties.alignment {
        style.alignment = alignment;
      }
      style.bullet = paragraph_properties_bullet(&properties.paragraph_properties_choice4);
    }
    style
  }

  fn apply_text_list_style(&mut self, style: &TextListParagraphStyle) {
    match style {
      TextListParagraphStyle::Default(properties) => {
        self.left_margin_pt = properties
          .left_margin
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.left_margin_pt);
        self.indent_pt = properties
          .indent
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.indent_pt);
        self.alignment = properties.alignment.unwrap_or(self.alignment);
        self.default_run_properties = properties.default_run_properties.clone();
        self.bullet =
          default_paragraph_properties_bullet(&properties.default_paragraph_properties_choice4);
      }
      TextListParagraphStyle::Level(level) => {
        self.apply_level_style(&level.paragraph_properties);
      }
    }
  }

  fn apply_level_style(&mut self, properties: &TextListLevelParagraphProperties) {
    macro_rules! apply_level {
      ($properties:expr, $bullet_fn:ident, $choice:ident) => {{
        self.left_margin_pt = $properties
          .left_margin
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.left_margin_pt);
        self.indent_pt = $properties
          .indent
          .map(|value| units::emu_to_points(i64::from(value)))
          .unwrap_or(self.indent_pt);
        self.alignment = $properties.alignment.unwrap_or(self.alignment);
        self.default_run_properties = $properties.default_run_properties.clone();
        self.bullet = $bullet_fn(&$properties.$choice);
      }};
    }

    match properties {
      TextListLevelParagraphProperties::Level1(properties) => {
        apply_level!(
          properties,
          level1_paragraph_properties_bullet,
          level1_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level2(properties) => {
        apply_level!(
          properties,
          level2_paragraph_properties_bullet,
          level2_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level3(properties) => {
        apply_level!(
          properties,
          level3_paragraph_properties_bullet,
          level3_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level4(properties) => {
        apply_level!(
          properties,
          level4_paragraph_properties_bullet,
          level4_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level5(properties) => {
        apply_level!(
          properties,
          level5_paragraph_properties_bullet,
          level5_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level6(properties) => {
        apply_level!(
          properties,
          level6_paragraph_properties_bullet,
          level6_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level7(properties) => {
        apply_level!(
          properties,
          level7_paragraph_properties_bullet,
          level7_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level8(properties) => {
        apply_level!(
          properties,
          level8_paragraph_properties_bullet,
          level8_paragraph_properties_choice4
        )
      }
      TextListLevelParagraphProperties::Level9(properties) => {
        apply_level!(
          properties,
          level9_paragraph_properties_bullet,
          level9_paragraph_properties_choice4
        )
      }
    }
  }

  fn bullet_label(&self, paragraph: &TextParagraph) -> Option<String> {
    self.bullet.clone().or_else(|| {
      paragraph
        .level
        .filter(|level| *level > 0)
        .map(|_| "\u{2022}".to_string())
    })
  }

  fn apply_default_run_style(&self, import: &PowerPointImport, style: &mut TextStyle) {
    if let Some(properties) = &self.default_run_properties {
      apply_default_run_properties(import, properties, style);
    }
  }
}

fn paragraph_properties_bullet(choice: &Option<a::ParagraphPropertiesChoice4>) -> Option<String> {
  match choice {
    Some(a::ParagraphPropertiesChoice4::NoBullet) => None,
    Some(a::ParagraphPropertiesChoice4::CharacterBullet(bullet)) => Some(bullet.char.clone()),
    Some(a::ParagraphPropertiesChoice4::AutoNumberedBullet(_)) => Some("1.".to_string()),
    Some(a::ParagraphPropertiesChoice4::PictureBullet(_)) => Some("\u{2022}".to_string()),
    None => None,
  }
}

macro_rules! bullet_fn {
  ($name:ident, $choice_ty:ty) => {
    fn $name(choice: &Option<$choice_ty>) -> Option<String> {
      match choice {
        Some(choice) => level_bullet_label(choice),
        None => None,
      }
    }
  };
}

fn default_paragraph_properties_bullet(
  choice: &Option<a::DefaultParagraphPropertiesChoice4>,
) -> Option<String> {
  match choice {
    Some(a::DefaultParagraphPropertiesChoice4::NoBullet) => None,
    Some(a::DefaultParagraphPropertiesChoice4::CharacterBullet(bullet)) => {
      Some(bullet.char.clone())
    }
    Some(a::DefaultParagraphPropertiesChoice4::AutoNumberedBullet(_)) => Some("1.".to_string()),
    Some(a::DefaultParagraphPropertiesChoice4::PictureBullet(_)) => Some("\u{2022}".to_string()),
    None => None,
  }
}

bullet_fn!(
  level1_paragraph_properties_bullet,
  a::Level1ParagraphPropertiesChoice4
);
bullet_fn!(
  level2_paragraph_properties_bullet,
  a::Level2ParagraphPropertiesChoice4
);
bullet_fn!(
  level3_paragraph_properties_bullet,
  a::Level3ParagraphPropertiesChoice4
);
bullet_fn!(
  level4_paragraph_properties_bullet,
  a::Level4ParagraphPropertiesChoice4
);
bullet_fn!(
  level5_paragraph_properties_bullet,
  a::Level5ParagraphPropertiesChoice4
);
bullet_fn!(
  level6_paragraph_properties_bullet,
  a::Level6ParagraphPropertiesChoice4
);
bullet_fn!(
  level7_paragraph_properties_bullet,
  a::Level7ParagraphPropertiesChoice4
);
bullet_fn!(
  level8_paragraph_properties_bullet,
  a::Level8ParagraphPropertiesChoice4
);
bullet_fn!(
  level9_paragraph_properties_bullet,
  a::Level9ParagraphPropertiesChoice4
);

trait BulletChoice {
  fn no_bullet(&self) -> bool;
  fn character(&self) -> Option<String>;
  fn auto_numbered(&self) -> bool;
  fn picture(&self) -> bool;
}

macro_rules! impl_bullet_choice {
  ($ty:ty) => {
    impl BulletChoice for $ty {
      fn no_bullet(&self) -> bool {
        matches!(self, Self::NoBullet)
      }

      fn character(&self) -> Option<String> {
        match self {
          Self::CharacterBullet(bullet) => Some(bullet.char.clone()),
          _ => None,
        }
      }

      fn auto_numbered(&self) -> bool {
        matches!(self, Self::AutoNumberedBullet(_))
      }

      fn picture(&self) -> bool {
        matches!(self, Self::PictureBullet(_))
      }
    }
  };
}

impl_bullet_choice!(a::Level1ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level2ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level3ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level4ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level5ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level6ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level7ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level8ParagraphPropertiesChoice4);
impl_bullet_choice!(a::Level9ParagraphPropertiesChoice4);

fn level_bullet_label(choice: &impl BulletChoice) -> Option<String> {
  if choice.no_bullet() {
    None
  } else if let Some(character) = choice.character() {
    Some(character)
  } else if choice.auto_numbered() {
    Some("1.".to_string())
  } else if choice.picture() {
    Some("\u{2022}".to_string())
  } else {
    None
  }
}

fn apply_run_properties(import: &PowerPointImport, run: &TextRun, style: &mut TextStyle) {
  let Some(properties) = run.run_properties.as_deref() else {
    return;
  };
  apply_run_common(
    RunCommon {
      font_size: properties.font_size,
      bold: properties.bold.as_ref().map(|value| value.as_bool()),
      italic: properties.italic.as_ref().map(|value| value.as_bool()),
      underline: properties.underline,
      strike: properties.strike,
      capital: properties.capital,
      spacing: properties.spacing,
      baseline: properties.baseline,
      latin_font: properties.latin_font.as_ref(),
    },
    style,
  );
  if let Some(fill) = properties.run_properties_choice1.as_ref() {
    apply_text_fill(import, fill, style);
  }
}

fn apply_default_run_properties(
  import: &PowerPointImport,
  properties: &a::DefaultRunProperties,
  style: &mut TextStyle,
) {
  apply_run_common(
    RunCommon {
      font_size: properties.font_size,
      bold: properties.bold.as_ref().map(|value| value.as_bool()),
      italic: properties.italic.as_ref().map(|value| value.as_bool()),
      underline: properties.underline,
      strike: properties.strike,
      capital: properties.capital,
      spacing: properties.spacing,
      baseline: properties.baseline,
      latin_font: properties.latin_font.as_ref(),
    },
    style,
  );
  if let Some(fill) = properties.default_run_properties_choice1.as_ref() {
    apply_default_text_fill(import, fill, style);
  }
}

struct RunCommon<'a> {
  font_size: Option<i32>,
  bold: Option<bool>,
  italic: Option<bool>,
  underline: Option<a::TextUnderlineValues>,
  strike: Option<a::TextStrikeValues>,
  capital: Option<a::TextCapsValues>,
  spacing: Option<ooxmlsdk::simple_type::TextPointValue>,
  baseline: Option<ooxmlsdk::simple_type::DrawingmlPercentageValue>,
  latin_font: Option<&'a a::LatinFont>,
}

fn apply_run_common(properties: RunCommon<'_>, style: &mut TextStyle) {
  if let Some(font_size) = properties.font_size {
    style.font_size_pt = ooxmlsdk::units::drawingml_text_size_to_points(font_size) as f32;
  }
  if let Some(bold) = properties.bold {
    style.bold = bold;
  }
  if let Some(italic) = properties.italic {
    style.italic = italic;
  }
  if let Some(underline) = properties.underline {
    style.underline = underline != a::TextUnderlineValues::None;
  }
  if let Some(strike) = properties.strike {
    style.strikethrough = strike != a::TextStrikeValues::NoStrike;
  }
  if let Some(capital) = properties.capital {
    style.uppercase = capital == a::TextCapsValues::All;
    style.small_caps = capital == a::TextCapsValues::Small;
  }
  if let Some(spacing) = properties.spacing {
    style.character_spacing_pt = spacing.to_points() as f32;
  }
  if let Some(baseline) = properties.baseline {
    style.baseline_shift_pt =
      style.font_size_pt * baseline.as_drawingml_percent() as f32 / 100_000.0;
  }
  if let Some(typeface) = properties
    .latin_font
    .and_then(|font| font.typeface.as_ref())
    .filter(|typeface| !typeface.is_empty())
  {
    style.font_family = Some(Arc::from(typeface.as_str()));
  }
}

fn apply_text_fill(
  import: &PowerPointImport,
  fill: &a::RunPropertiesChoice,
  style: &mut TextStyle,
) {
  match fill {
    a::RunPropertiesChoice::NoFill(_) => {
      style.hidden = true;
    }
    a::RunPropertiesChoice::SolidFill(fill) => {
      if let Some(color) = fill
        .solid_fill_choice
        .as_ref()
        .and_then(Color::from_solid_fill_choice)
        .and_then(|color| display_paint(import, &color, None))
      {
        style.color = color.color;
        style.opacity = color.opacity;
      }
    }
    a::RunPropertiesChoice::GradientFill(_)
    | a::RunPropertiesChoice::BlipFill(_)
    | a::RunPropertiesChoice::PatternFill(_)
    | a::RunPropertiesChoice::GroupFill => {}
  }
}

fn apply_default_text_fill(
  import: &PowerPointImport,
  fill: &a::DefaultRunPropertiesChoice,
  style: &mut TextStyle,
) {
  match fill {
    a::DefaultRunPropertiesChoice::NoFill(_) => {
      style.hidden = true;
    }
    a::DefaultRunPropertiesChoice::SolidFill(fill) => {
      if let Some(color) = fill
        .solid_fill_choice
        .as_ref()
        .and_then(Color::from_solid_fill_choice)
        .and_then(|color| display_paint(import, &color, None))
      {
        style.color = color.color;
        style.opacity = color.opacity;
      }
    }
    a::DefaultRunPropertiesChoice::GradientFill(_)
    | a::DefaultRunPropertiesChoice::BlipFill(_)
    | a::DefaultRunPropertiesChoice::PatternFill(_)
    | a::DefaultRunPropertiesChoice::GroupFill => {}
  }
}

#[derive(Clone, Copy, Debug)]
struct DisplayPaint {
  color: RgbColor,
  opacity: f32,
}

#[derive(Clone, Copy, Debug)]
struct DisplayStroke {
  style: BorderStyle,
  opacity: f32,
}

fn fill_paint(import: &PowerPointImport, fill: &FillProperties) -> Option<DisplayPaint> {
  match &fill.kind {
    FillKind::Solid(color) => color
      .as_ref()
      .and_then(|color| display_paint(import, color, fill.placeholder_color.as_ref())),
    FillKind::None
    | FillKind::Group
    | FillKind::Gradient(_)
    | FillKind::Blip(_)
    | FillKind::Pattern(_) => None,
  }
}

fn line_stroke(import: &PowerPointImport, line: &LineProperties) -> Option<DisplayStroke> {
  let LineFill::Solid(color) = &line.fill else {
    return None;
  };
  let paint = color
    .as_ref()
    .and_then(|color| display_paint(import, color, line.placeholder_color.as_ref()))?;
  Some(DisplayStroke {
    style: BorderStyle {
      width_pt: line.width_emu.map(units::emu_to_points).unwrap_or(0.75),
      spacing_pt: 0.0,
      color: paint.color,
      compound: false,
    },
    opacity: paint.opacity,
  })
}

fn display_paint(
  import: &PowerPointImport,
  color: &Color,
  placeholder_color: Option<&Color>,
) -> Option<DisplayPaint> {
  let color = import.resolve_color(color, placeholder_color)?;
  Some(DisplayPaint {
    color: RgbColor {
      r: color.r,
      g: color.g,
      b: color.b,
    },
    opacity: color_opacity(color.alpha),
  })
}

fn color_opacity(alpha: i32) -> f32 {
  alpha.clamp(0, 100_000) as f32 / 100_000.0
}
