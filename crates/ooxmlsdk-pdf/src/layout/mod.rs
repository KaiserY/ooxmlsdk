use crate::docx::{
  Block, BorderStyle, DocxDocument, InlineItem, PageSetup, ParagraphAlignment, RgbColor,
  TableCellVerticalAlignment, TextStyle,
};
use crate::error::Result;
use crate::options::PdfOptions;

const PARAGRAPH_SPACING_AFTER_PT: f32 = 6.0;
const DEFAULT_FONT_SIZE_PT: f32 = 11.0;
const DEFAULT_LINE_HEIGHT_PT: f32 = 14.0;
const TABLE_CELL_PADDING_PT: f32 = 4.0;
const TABLE_ROW_MIN_HEIGHT_PT: f32 = 24.0;
const TABLE_SPACING_AFTER_PT: f32 = 6.0;

#[derive(Clone, Debug)]
pub(crate) struct LayoutDocument {
  pub pages: Vec<Page>,
}

#[derive(Clone, Debug)]
pub(crate) struct Page {
  pub setup: PageSetup,
  pub items: Vec<PageItem>,
}

#[derive(Clone, Debug)]
pub(crate) enum PageItem {
  Text(TextItem),
  Image(ImageItem),
  Fill(FillItem),
  Line(LineItem),
}

#[derive(Clone, Debug)]
pub(crate) struct TextItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub text: String,
  pub style: TextStyle,
}

#[derive(Clone, Debug)]
pub(crate) struct ImageItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub data: Vec<u8>,
  pub content_type: Option<String>,
  pub alt_text: Option<String>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct FillItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub color: RgbColor,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LineItem {
  pub x1_pt: f32,
  pub y1_pt: f32,
  pub x2_pt: f32,
  pub y2_pt: f32,
  pub width_pt: f32,
  pub color: RgbColor,
}

pub(crate) fn layout(document: &DocxDocument, _options: &PdfOptions) -> Result<LayoutDocument> {
  let mut pages = Vec::new();
  let mut current = Page {
    setup: document.page,
    items: Vec::new(),
  };
  let mut y = document.page.margin_top_pt;
  let content_bottom = document.page.height_pt - document.page.margin_bottom_pt;
  let content_width =
    (document.page.width_pt - document.page.margin_left_pt - document.page.margin_right_pt)
      .max(DEFAULT_FONT_SIZE_PT);

  for block in &document.blocks {
    y = layout_document_block(
      block,
      document.page,
      &mut current,
      &mut pages,
      y,
      content_bottom,
      content_width,
    );
  }

  if !document.footnote_blocks.is_empty() {
    y = layout_note_separator(document.page, &mut current, &mut pages, y, content_bottom);
    for block in &document.footnote_blocks {
      y = layout_document_block(
        block,
        document.page,
        &mut current,
        &mut pages,
        y,
        content_bottom,
        content_width,
      );
    }
  }

  if !document.endnote_blocks.is_empty() {
    y = layout_note_separator(document.page, &mut current, &mut pages, y, content_bottom);
    for block in &document.endnote_blocks {
      y = layout_document_block(
        block,
        document.page,
        &mut current,
        &mut pages,
        y,
        content_bottom,
        content_width,
      );
    }
  }

  if !document.comment_blocks.is_empty() {
    y = layout_note_separator(document.page, &mut current, &mut pages, y, content_bottom);
    for block in &document.comment_blocks {
      y = layout_document_block(
        block,
        document.page,
        &mut current,
        &mut pages,
        y,
        content_bottom,
        content_width,
      );
    }
  }

  if !current.items.is_empty() || pages.is_empty() {
    pages.push(current);
  }

  apply_headers_and_footers(document, &mut pages, content_width);

  Ok(LayoutDocument { pages })
}

fn layout_document_block(
  block: &Block,
  setup: PageSetup,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
  content_bottom: f32,
  content_width: f32,
) -> f32 {
  match block {
    Block::Paragraph(paragraph) => {
      if paragraph.format.page_break_before && !current.items.is_empty() {
        pages.push(std::mem::replace(
          current,
          Page {
            setup,
            items: Vec::new(),
          },
        ));
        y = setup.margin_top_pt;
      }

      y += paragraph.format.spacing_before_pt;
      layout_paragraph(
        paragraph,
        setup,
        current,
        pages,
        y,
        content_bottom,
        (content_width - paragraph.format.indent_left_pt - paragraph.format.indent_right_pt)
          .max(DEFAULT_FONT_SIZE_PT),
      )
    }
    Block::Table(table) => layout_table(
      table,
      setup,
      current,
      pages,
      y,
      content_bottom,
      content_width,
    ),
  }
}

fn layout_note_separator(
  setup: PageSetup,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
  content_bottom: f32,
) -> f32 {
  if y + DEFAULT_LINE_HEIGHT_PT > content_bottom && !current.items.is_empty() {
    pages.push(std::mem::replace(
      current,
      Page {
        setup,
        items: Vec::new(),
      },
    ));
    y = setup.margin_top_pt;
  }

  y += 4.0;
  current.items.push(PageItem::Line(LineItem {
    x1_pt: setup.margin_left_pt,
    y1_pt: y,
    x2_pt: setup.margin_left_pt + 120.0,
    y2_pt: y,
    width_pt: 0.5,
    color: RgbColor { r: 0, g: 0, b: 0 },
  }));
  y + 8.0
}

fn apply_headers_and_footers(document: &DocxDocument, pages: &mut [Page], content_width: f32) {
  if document.header_blocks.is_empty()
    && document.footer_blocks.is_empty()
    && document.first_header_blocks.is_empty()
    && document.first_footer_blocks.is_empty()
  {
    return;
  }

  for (index, page) in pages.iter_mut().enumerate() {
    let header_blocks =
      if index == 0 && document.title_page && !document.first_header_blocks.is_empty() {
        &document.first_header_blocks
      } else {
        &document.header_blocks
      };
    let footer_blocks =
      if index == 0 && document.title_page && !document.first_footer_blocks.is_empty() {
        &document.first_footer_blocks
      } else {
        &document.footer_blocks
      };
    let mut adornment = Page {
      setup: page.setup,
      items: Vec::new(),
    };
    let mut discarded_pages = Vec::new();
    let header_bottom = page
      .setup
      .margin_top_pt
      .max(page.setup.header_distance_pt + 1.0);
    let mut y = page.setup.header_distance_pt;
    for block in header_blocks {
      y = layout_repeating_block(
        block,
        page.setup,
        &mut adornment,
        &mut discarded_pages,
        y,
        header_bottom,
        content_width,
      );
    }

    let mut y =
      (page.setup.height_pt - page.setup.footer_distance_pt - DEFAULT_LINE_HEIGHT_PT).max(0.0);
    let footer_bottom = page.setup.height_pt - 1.0;
    for block in footer_blocks {
      y = layout_repeating_block(
        block,
        page.setup,
        &mut adornment,
        &mut discarded_pages,
        y,
        footer_bottom,
        content_width,
      );
    }

    page.items.extend(adornment.items);
  }
}

fn layout_repeating_block(
  block: &Block,
  setup: PageSetup,
  page: &mut Page,
  discarded_pages: &mut Vec<Page>,
  y: f32,
  content_bottom: f32,
  content_width: f32,
) -> f32 {
  match block {
    Block::Paragraph(paragraph) => layout_paragraph(
      paragraph,
      setup,
      page,
      discarded_pages,
      y + paragraph.format.spacing_before_pt,
      content_bottom,
      content_width,
    ),
    Block::Table(table) => layout_table(
      table,
      setup,
      page,
      discarded_pages,
      y,
      content_bottom,
      content_width,
    ),
  }
}

pub(crate) fn text_pages(pages: Vec<(PageSetup, Vec<String>)>) -> LayoutDocument {
  let mut output_pages = Vec::new();
  for (setup, lines) in pages {
    let mut page = Page {
      setup,
      items: Vec::new(),
    };
    let mut y = setup.margin_top_pt;
    let content_bottom = setup.height_pt - setup.margin_bottom_pt;
    for line in lines {
      if y + DEFAULT_LINE_HEIGHT_PT > content_bottom {
        output_pages.push(page);
        page = Page {
          setup,
          items: Vec::new(),
        };
        y = setup.margin_top_pt;
      }
      if !line.is_empty() {
        page.items.push(PageItem::Text(TextItem {
          x_pt: setup.margin_left_pt,
          y_pt: y,
          text: line,
          style: TextStyle::default(),
        }));
      }
      y += DEFAULT_LINE_HEIGHT_PT;
    }
    output_pages.push(page);
  }

  if output_pages.is_empty() {
    output_pages.push(Page {
      setup: PageSetup::default(),
      items: Vec::new(),
    });
  }

  LayoutDocument {
    pages: output_pages,
  }
}

fn layout_table(
  table: &crate::docx::Table,
  setup: PageSetup,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
  content_bottom: f32,
  content_width: f32,
) -> f32 {
  let column_count = table
    .rows
    .iter()
    .map(|row| row.cells.iter().map(|cell| cell.grid_span).sum::<usize>())
    .max()
    .unwrap_or(0);
  if column_count == 0 {
    return y;
  }

  let column_widths = table_column_widths(table, column_count, content_width);
  let table_left = setup.margin_left_pt;
  let table_right = table_left + column_widths.iter().sum::<f32>();

  for (row_index, row) in table.rows.iter().enumerate() {
    let row_height = table_row_height(row);
    if y + row_height > content_bottom && !current.items.is_empty() {
      pages.push(std::mem::replace(
        current,
        Page {
          setup,
          items: Vec::new(),
        },
      ));
      y = setup.margin_top_pt;
    }

    let row_top = y;
    let row_bottom = y + row_height;

    let mut cell_left = table_left;
    let mut grid_index = 0;
    for (cell_index, cell) in row.cells.iter().enumerate() {
      if grid_index >= column_widths.len() {
        break;
      }
      let span = cell
        .grid_span
        .max(1)
        .min(column_widths.len().saturating_sub(grid_index));
      let width = column_widths[grid_index..grid_index + span]
        .iter()
        .sum::<f32>();
      if !cell.vertical_merge_continue {
        if let Some(color) = cell.shading {
          current.items.push(PageItem::Fill(FillItem {
            x_pt: cell_left,
            y_pt: row_top,
            width_pt: width,
            height_pt: row_height,
            color,
          }));
        }
        if let Some(border) = vertical_border(table, row, cell_index, true) {
          push_styled_line(current, cell_left, row_top, cell_left, row_bottom, border);
        }
        layout_table_cell(cell, setup, current, cell_left, row_top, width, row_height);
      }
      cell_left += width;
      grid_index += span;
    }
    if let Some(border) = horizontal_border(table, row_index, true) {
      push_styled_line(current, table_left, row_top, table_right, row_top, border);
    }
    if let Some(border) = horizontal_border(table, row_index, false) {
      push_styled_line(
        current,
        table_left,
        row_bottom,
        table_right,
        row_bottom,
        border,
      );
    }
    if let Some(border) = row
      .cells
      .last()
      .and_then(|cell| cell.borders.right)
      .or_else(|| table.borders.and_then(|borders| borders.right))
      .or(Some(BorderStyle::default()))
    {
      push_styled_line(
        current,
        table_right,
        row_top,
        table_right,
        row_bottom,
        border,
      );
    }
    y = row_bottom;
  }

  y + TABLE_SPACING_AFTER_PT
}

fn horizontal_border(
  table: &crate::docx::Table,
  row_index: usize,
  top_edge: bool,
) -> Option<BorderStyle> {
  let borders = table.borders;
  let row = table.rows.get(row_index)?;
  if top_edge {
    row
      .cells
      .first()
      .and_then(|cell| cell.borders.top)
      .or_else(|| {
        borders.and_then(|borders| {
          if row_index == 0 {
            borders.top
          } else {
            borders.inside_horizontal
          }
        })
      })
      .or(Some(BorderStyle::default()))
  } else {
    row
      .cells
      .first()
      .and_then(|cell| cell.borders.bottom)
      .or_else(|| {
        borders.and_then(|borders| {
          if row_index + 1 == table.rows.len() {
            borders.bottom
          } else {
            borders.inside_horizontal
          }
        })
      })
      .or(Some(BorderStyle::default()))
  }
}

fn vertical_border(
  table: &crate::docx::Table,
  row: &crate::docx::TableRow,
  cell_index: usize,
  left_edge: bool,
) -> Option<BorderStyle> {
  let borders = table.borders;
  let cell = row.cells.get(cell_index)?;
  if left_edge {
    cell
      .borders
      .left
      .or_else(|| {
        if cell_index > 0 {
          row
            .cells
            .get(cell_index - 1)
            .and_then(|previous| previous.borders.right)
        } else {
          None
        }
      })
      .or_else(|| {
        borders.and_then(|borders| {
          if cell_index == 0 {
            borders.left
          } else {
            borders.inside_vertical
          }
        })
      })
      .or(Some(BorderStyle::default()))
  } else {
    cell
      .borders
      .right
      .or_else(|| {
        borders.and_then(|borders| {
          if cell_index + 1 == row.cells.len() {
            borders.right
          } else {
            borders.inside_vertical
          }
        })
      })
      .or(Some(BorderStyle::default()))
  }
}

fn table_column_widths(
  table: &crate::docx::Table,
  column_count: usize,
  content_width: f32,
) -> Vec<f32> {
  if table.column_widths_pt.len() >= column_count {
    let mut widths = table.column_widths_pt[..column_count].to_vec();
    if let Some(preferred) = table.preferred_width_pt
      && preferred > 0.0
    {
      let total = widths.iter().sum::<f32>();
      if total > 0.0 {
        let scale = preferred / total;
        for width in &mut widths {
          *width *= scale;
        }
      }
    }
    let total = widths.iter().sum::<f32>();
    if total > content_width && total > 0.0 {
      let scale = content_width / total;
      for width in &mut widths {
        *width *= scale;
      }
    }
    return widths;
  }

  vec![content_width / column_count as f32; column_count]
}

fn table_row_height(row: &crate::docx::TableRow) -> f32 {
  let content_height = row
    .cells
    .iter()
    .filter(|cell| !cell.vertical_merge_continue)
    .map(|cell| {
      let line_count = cell
        .blocks
        .iter()
        .map(|block| match block {
          Block::Paragraph(paragraph) => paragraph
            .inlines
            .iter()
            .filter(|item| match item {
              InlineItem::Text(run) => !run.text.is_empty(),
              InlineItem::Image(_) => true,
              InlineItem::PageBreak => false,
            })
            .count()
            .max(1),
          Block::Table(table) => table.rows.len().max(1),
        })
        .sum::<usize>()
        .max(1);
      TABLE_CELL_PADDING_PT * 2.0 + DEFAULT_LINE_HEIGHT_PT * line_count as f32
    })
    .fold(TABLE_ROW_MIN_HEIGHT_PT, f32::max);
  match (row.height_pt, row.exact_height) {
    (Some(height), true) => height,
    (Some(height), false) => content_height.max(height),
    (None, _) => content_height,
  }
}

fn layout_table_cell(
  cell: &crate::docx::TableCell,
  setup: PageSetup,
  page: &mut Page,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
) {
  let content_height = table_cell_content_height(cell);
  let mut text_y = match cell.vertical_alignment {
    TableCellVerticalAlignment::Top => y + TABLE_CELL_PADDING_PT + DEFAULT_FONT_SIZE_PT,
    TableCellVerticalAlignment::Center => {
      y + ((height - content_height) / 2.0).max(TABLE_CELL_PADDING_PT) + DEFAULT_FONT_SIZE_PT
    }
    TableCellVerticalAlignment::Bottom => {
      y + (height - TABLE_CELL_PADDING_PT - content_height).max(TABLE_CELL_PADDING_PT)
        + DEFAULT_FONT_SIZE_PT
    }
  };
  let text_left = x + TABLE_CELL_PADDING_PT;
  let text_right = x + width - TABLE_CELL_PADDING_PT;
  let text_bottom = y + height - TABLE_CELL_PADDING_PT;

  for block in &cell.blocks {
    match block {
      Block::Paragraph(paragraph) => {
        let mut text_x = text_left;
        for item in &paragraph.inlines {
          match item {
            InlineItem::Text(run) => {
              if run.text.is_empty() {
                continue;
              }
              if text_y > text_bottom {
                return;
              }
              let available = (text_right - text_x).max(DEFAULT_FONT_SIZE_PT);
              let clipped = clip_to_width(&run.text, run.style.font_size_pt, available);
              if clipped.is_empty() {
                continue;
              }
              let width = approximate_text_width(&clipped, run.style.font_size_pt);
              page.items.push(PageItem::Text(TextItem {
                x_pt: text_x,
                y_pt: text_y,
                text: clipped,
                style: run.style,
              }));
              text_x += width;
            }
            InlineItem::Image(image) => {
              if text_y > text_bottom {
                return;
              }
              let width = image.width_pt.min((text_right - text_x).max(1.0));
              let height = if image.width_pt > 0.0 {
                image.height_pt * (width / image.width_pt)
              } else {
                image.height_pt
              };
              page.items.push(PageItem::Image(ImageItem {
                x_pt: text_x,
                y_pt: text_y - DEFAULT_FONT_SIZE_PT,
                width_pt: width,
                height_pt: height,
                data: image.data.clone(),
                content_type: image.content_type.clone(),
                alt_text: image.alt_text.clone(),
              }));
              text_x += width;
            }
            InlineItem::PageBreak => {
              text_y = text_bottom + 1.0;
            }
          }
        }
        text_y += paragraph
          .format
          .line_height_pt
          .unwrap_or(DEFAULT_LINE_HEIGHT_PT);
      }
      Block::Table(table) => {
        let nested = layout_nested_table(
          table,
          setup,
          page,
          text_left,
          text_y,
          text_right - text_left,
        );
        text_y += nested;
      }
    }
  }
}

fn table_cell_content_height(cell: &crate::docx::TableCell) -> f32 {
  cell
    .blocks
    .iter()
    .map(|block| match block {
      Block::Paragraph(paragraph) => paragraph
        .format
        .line_height_pt
        .unwrap_or(DEFAULT_LINE_HEIGHT_PT),
      Block::Table(table) => table.rows.len().max(1) as f32 * TABLE_ROW_MIN_HEIGHT_PT,
    })
    .sum::<f32>()
    .max(DEFAULT_LINE_HEIGHT_PT)
}

fn layout_nested_table(
  table: &crate::docx::Table,
  setup: PageSetup,
  page: &mut Page,
  x: f32,
  y: f32,
  content_width: f32,
) -> f32 {
  let mut nested_pages = Vec::new();
  let mut nested_page = Page {
    setup,
    items: Vec::new(),
  };
  let bottom = setup.height_pt;
  let end_y = layout_table(
    table,
    PageSetup {
      margin_left_pt: x,
      margin_top_pt: y,
      margin_right_pt: setup.width_pt - x - content_width,
      margin_bottom_pt: 0.0,
      ..setup
    },
    &mut nested_page,
    &mut nested_pages,
    y,
    bottom,
    content_width,
  );
  page.items.extend(nested_page.items);
  end_y - y
}

fn layout_paragraph(
  paragraph: &crate::docx::Paragraph,
  setup: PageSetup,
  current: &mut Page,
  pages: &mut Vec<Page>,
  mut y: f32,
  content_bottom: f32,
  content_width: f32,
) -> f32 {
  let start_item_index = current.items.len();
  let paragraph_top = y;
  let line_left = setup.margin_left_pt + paragraph.format.indent_left_pt;
  let first_line_left =
    (line_left + paragraph.format.first_line_indent_pt).max(setup.margin_left_pt);
  let line_right = line_left + content_width;
  let paragraph_left = line_left.min(first_line_left);
  let base_line_height = paragraph
    .format
    .line_height_pt
    .unwrap_or(DEFAULT_LINE_HEIGHT_PT);
  let mut line_height = base_line_height;
  let mut emitted = paragraph.list_label.is_some();
  let mut x = if let Some(label) = &paragraph.list_label {
    current.items.push(PageItem::Text(TextItem {
      x_pt: first_line_left,
      y_pt: y,
      text: label.clone(),
      style: TextStyle::default(),
    }));
    line_left
  } else {
    first_line_left
  };

  for item in &paragraph.inlines {
    match item {
      InlineItem::Text(run) => {
        let mut chunk = String::new();
        let mut chunk_x = x;

        for ch in run.text.chars() {
          if ch == '\n' {
            flush_text(current, chunk_x, y, &mut chunk, run.style);
            y = next_line(setup, current, pages, y, &mut line_height, content_bottom);
            x = line_left;
            chunk_x = x;
            emitted = true;
            continue;
          }

          let mut encoded = [0; 4];
          let text = if ch == '\t' {
            "    "
          } else {
            ch.encode_utf8(&mut encoded)
          };
          let width = approximate_text_width(text, run.style.font_size_pt);

          if x + width > line_right && x > line_left {
            flush_text(current, chunk_x, y, &mut chunk, run.style);
            y = next_line(setup, current, pages, y, &mut line_height, content_bottom);
            x = line_left;
            chunk_x = x;
          }

          if chunk.is_empty() {
            chunk_x = x;
          }
          chunk.push_str(text);
          x += width;
          line_height = line_height.max(run.style.font_size_pt * 1.25);
          emitted = true;
        }

        flush_text(current, chunk_x, y, &mut chunk, run.style);
      }
      InlineItem::Image(image) => {
        let (width, height) = fit_image_to_line(image.width_pt, image.height_pt, content_width);
        if x + width > line_right && x > line_left {
          y = next_line(setup, current, pages, y, &mut line_height, content_bottom);
          x = line_left;
        }
        current.items.push(PageItem::Image(ImageItem {
          x_pt: x,
          y_pt: y,
          width_pt: width,
          height_pt: height,
          data: image.data.clone(),
          content_type: image.content_type.clone(),
          alt_text: image.alt_text.clone(),
        }));
        x += width;
        line_height = line_height.max(height);
        emitted = true;
      }
      InlineItem::PageBreak => {
        y = force_page_break(setup, current, pages);
        x = first_line_left;
        line_height = base_line_height;
        emitted = false;
      }
    }
  }

  let paragraph_bottom;
  if emitted {
    paragraph_bottom = y + line_height;
    y = paragraph_bottom
      + paragraph
        .format
        .spacing_after_pt
        .max(PARAGRAPH_SPACING_AFTER_PT);
  } else {
    paragraph_bottom = y + base_line_height;
    y = paragraph_bottom + paragraph.format.spacing_after_pt;
  }

  if paragraph.list_label.is_none() && start_item_index <= current.items.len() {
    align_paragraph_items(
      &mut current.items[start_item_index..],
      paragraph.format.alignment,
      line_right,
    );
  }
  if start_item_index <= current.items.len() {
    decorate_paragraph(
      current,
      start_item_index,
      paragraph,
      paragraph_left,
      paragraph_top,
      line_right - paragraph_left,
      paragraph_bottom - paragraph_top,
    );
  }

  y
}

fn decorate_paragraph(
  page: &mut Page,
  start_item_index: usize,
  paragraph: &crate::docx::Paragraph,
  x: f32,
  y: f32,
  width: f32,
  height: f32,
) {
  let padding = 2.0;
  let x = x - padding;
  let y = y - padding;
  let width = width + padding * 2.0;
  let height = height + padding * 2.0;

  if let Some(color) = paragraph.format.shading {
    page.items.insert(
      start_item_index,
      PageItem::Fill(FillItem {
        x_pt: x,
        y_pt: y,
        width_pt: width,
        height_pt: height,
        color,
      }),
    );
  }

  if let Some(border) = paragraph.format.borders.top {
    push_styled_line(page, x, y, x + width, y, border);
  }
  if let Some(border) = paragraph.format.borders.right {
    push_styled_line(page, x + width, y, x + width, y + height, border);
  }
  if let Some(border) = paragraph.format.borders.bottom {
    push_styled_line(page, x, y + height, x + width, y + height, border);
  }
  if let Some(border) = paragraph.format.borders.left {
    push_styled_line(page, x, y, x, y + height, border);
  }
}

fn align_paragraph_items(items: &mut [PageItem], alignment: ParagraphAlignment, line_right: f32) {
  if matches!(
    alignment,
    ParagraphAlignment::Left | ParagraphAlignment::Justify
  ) {
    return;
  }

  let mut line_ys = Vec::new();
  for item in items.iter() {
    let Some(y) = item_y(item) else {
      continue;
    };
    if !line_ys.iter().any(|seen| f32::abs(*seen - y) < 0.01) {
      line_ys.push(y);
    }
  }

  for y in line_ys {
    let mut min_x = f32::MAX;
    let mut max_x: f32 = 0.0;
    for item in items.iter() {
      if let Some(item_y) = item_y(item)
        && f32::abs(item_y - y) < 0.01
        && let Some((x, width)) = item_horizontal_bounds(item)
      {
        min_x = min_x.min(x);
        max_x = max_x.max(x + width);
      }
    }

    if min_x == f32::MAX || max_x <= min_x {
      continue;
    }

    let available = line_right - min_x;
    let line_width = max_x - min_x;
    let offset = match alignment {
      ParagraphAlignment::Center => (available - line_width).max(0.0) / 2.0,
      ParagraphAlignment::Right => (available - line_width).max(0.0),
      ParagraphAlignment::Left | ParagraphAlignment::Justify => 0.0,
    };
    if offset <= 0.0 {
      continue;
    }

    for item in items.iter_mut() {
      if let Some(item_y) = item_y(item)
        && f32::abs(item_y - y) < 0.01
      {
        shift_item_x(item, offset);
      }
    }
  }
}

fn item_y(item: &PageItem) -> Option<f32> {
  match item {
    PageItem::Text(text) => Some(text.y_pt),
    PageItem::Image(image) => Some(image.y_pt),
    PageItem::Fill(_) => None,
    PageItem::Line(_) => None,
  }
}

fn item_horizontal_bounds(item: &PageItem) -> Option<(f32, f32)> {
  match item {
    PageItem::Text(text) => Some((
      text.x_pt,
      approximate_text_width(&text.text, text.style.font_size_pt),
    )),
    PageItem::Image(image) => Some((image.x_pt, image.width_pt)),
    PageItem::Fill(_) => None,
    PageItem::Line(_) => None,
  }
}

fn shift_item_x(item: &mut PageItem, offset: f32) {
  match item {
    PageItem::Text(text) => text.x_pt += offset,
    PageItem::Image(image) => image.x_pt += offset,
    PageItem::Fill(_) => {}
    PageItem::Line(_) => {}
  }
}

fn fit_image_to_line(width: f32, height: f32, max_width: f32) -> (f32, f32) {
  let width = width.max(1.0);
  let height = height.max(1.0);
  if width <= max_width {
    (width, height)
  } else {
    let scale = max_width.max(1.0) / width;
    (width * scale, height * scale)
  }
}

fn next_line(
  setup: PageSetup,
  current: &mut Page,
  pages: &mut Vec<Page>,
  y: f32,
  line_height: &mut f32,
  content_bottom: f32,
) -> f32 {
  let mut next_y = y + *line_height;
  *line_height = DEFAULT_LINE_HEIGHT_PT;
  if next_y + *line_height > content_bottom && !current.items.is_empty() {
    pages.push(std::mem::replace(
      current,
      Page {
        setup,
        items: Vec::new(),
      },
    ));
    next_y = setup.margin_top_pt;
  }
  next_y
}

fn force_page_break(setup: PageSetup, current: &mut Page, pages: &mut Vec<Page>) -> f32 {
  if !current.items.is_empty() {
    pages.push(std::mem::replace(
      current,
      Page {
        setup,
        items: Vec::new(),
      },
    ));
  }
  setup.margin_top_pt
}

fn flush_text(page: &mut Page, x: f32, y: f32, chunk: &mut String, style: TextStyle) {
  if chunk.is_empty() {
    return;
  }

  page.items.push(PageItem::Text(TextItem {
    x_pt: x,
    y_pt: y,
    text: std::mem::take(chunk),
    style,
  }));
}

fn push_styled_line(page: &mut Page, x1: f32, y1: f32, x2: f32, y2: f32, border: BorderStyle) {
  page.items.push(PageItem::Line(LineItem {
    x1_pt: x1,
    y1_pt: y1,
    x2_pt: x2,
    y2_pt: y2,
    width_pt: border.width_pt,
    color: border.color,
  }));
}

fn clip_to_width(text: &str, font_size: f32, max_width: f32) -> String {
  let mut width = 0.0;
  let mut clipped = String::new();
  for ch in text.chars() {
    let mut encoded = [0; 4];
    let value = if ch == '\t' {
      "    "
    } else if ch == '\n' {
      break;
    } else {
      ch.encode_utf8(&mut encoded)
    };
    let next = approximate_text_width(value, font_size);
    if width + next > max_width && !clipped.is_empty() {
      break;
    }
    width += next;
    clipped.push_str(value);
  }
  clipped
}

fn approximate_text_width(text: &str, font_size: f32) -> f32 {
  text
    .chars()
    .map(|ch| {
      if ch.is_whitespace() {
        font_size * 0.33
      } else if ch.is_ascii() {
        font_size * 0.55
      } else {
        font_size
      }
    })
    .sum()
}
