use crate::docx::{Block, DocxDocument, PageSetup, TextStyle};
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
  Line(LineItem),
}

#[derive(Clone, Debug)]
pub(crate) struct TextItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub text: String,
  pub style: TextStyle,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LineItem {
  pub x1_pt: f32,
  pub y1_pt: f32,
  pub x2_pt: f32,
  pub y2_pt: f32,
  pub width_pt: f32,
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
    match block {
      Block::Paragraph(paragraph) => {
        if paragraph.format.page_break_before && !current.items.is_empty() {
          pages.push(current);
          current = Page {
            setup: document.page,
            items: Vec::new(),
          };
          y = document.page.margin_top_pt;
        }

        y += paragraph.format.spacing_before_pt;
        y = layout_paragraph(
          paragraph,
          document.page,
          &mut current,
          &mut pages,
          y,
          content_bottom,
          (content_width - paragraph.format.indent_left_pt - paragraph.format.indent_right_pt)
            .max(DEFAULT_FONT_SIZE_PT),
        );
      }
      Block::Table(table) => {
        y = layout_table(
          table,
          document.page,
          &mut current,
          &mut pages,
          y,
          content_bottom,
          content_width,
        );
      }
    }
  }

  if !current.items.is_empty() || pages.is_empty() {
    pages.push(current);
  }

  Ok(LayoutDocument { pages })
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
    .map(|row| row.cells.len())
    .max()
    .unwrap_or(0);
  if column_count == 0 {
    return y;
  }

  let column_widths = table_column_widths(table, column_count, content_width);
  let table_left = setup.margin_left_pt;
  let table_right = table_left + column_widths.iter().sum::<f32>();

  for row in &table.rows {
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
    push_line(current, table_left, row_top, table_right, row_top);
    push_line(current, table_left, row_bottom, table_right, row_bottom);

    let mut cell_left = table_left;
    for (index, width) in column_widths.iter().enumerate() {
      push_line(current, cell_left, row_top, cell_left, row_bottom);
      if let Some(cell) = row.cells.get(index) {
        layout_table_cell(cell, setup, current, cell_left, row_top, *width, row_height);
      }
      cell_left += *width;
    }
    push_line(current, table_right, row_top, table_right, row_bottom);
    y = row_bottom;
  }

  y + TABLE_SPACING_AFTER_PT
}

fn table_column_widths(
  table: &crate::docx::Table,
  column_count: usize,
  content_width: f32,
) -> Vec<f32> {
  if table.column_widths_pt.len() >= column_count {
    let mut widths = table.column_widths_pt[..column_count].to_vec();
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
  row
    .cells
    .iter()
    .map(|cell| {
      let line_count = cell
        .blocks
        .iter()
        .map(|block| match block {
          Block::Paragraph(paragraph) => paragraph
            .runs
            .iter()
            .filter(|run| !run.text.is_empty())
            .count()
            .max(1),
          Block::Table(table) => table.rows.len().max(1),
        })
        .sum::<usize>()
        .max(1);
      TABLE_CELL_PADDING_PT * 2.0 + DEFAULT_LINE_HEIGHT_PT * line_count as f32
    })
    .fold(TABLE_ROW_MIN_HEIGHT_PT, f32::max)
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
  let mut text_y = y + TABLE_CELL_PADDING_PT + DEFAULT_FONT_SIZE_PT;
  let text_left = x + TABLE_CELL_PADDING_PT;
  let text_right = x + width - TABLE_CELL_PADDING_PT;
  let text_bottom = y + height - TABLE_CELL_PADDING_PT;

  for block in &cell.blocks {
    match block {
      Block::Paragraph(paragraph) => {
        let mut text_x = text_left;
        for run in &paragraph.runs {
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
  let line_left = setup.margin_left_pt + paragraph.format.indent_left_pt;
  let first_line_left =
    (line_left + paragraph.format.first_line_indent_pt).max(setup.margin_left_pt);
  let line_right = line_left + content_width;
  let base_line_height = paragraph
    .format
    .line_height_pt
    .unwrap_or(DEFAULT_LINE_HEIGHT_PT);
  let mut line_height = base_line_height;
  let mut emitted = false;
  let mut x = first_line_left;

  for run in &paragraph.runs {
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

  if emitted {
    y += line_height
      + paragraph
        .format
        .spacing_after_pt
        .max(PARAGRAPH_SPACING_AFTER_PT);
  } else {
    y += base_line_height + paragraph.format.spacing_after_pt;
  }

  y
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

fn push_line(page: &mut Page, x1: f32, y1: f32, x2: f32, y2: f32) {
  page.items.push(PageItem::Line(LineItem {
    x1_pt: x1,
    y1_pt: y1,
    x2_pt: x2,
    y2_pt: y2,
    width_pt: 0.5,
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
