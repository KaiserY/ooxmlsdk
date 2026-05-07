mod units;

use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

use crate::error::Result;
use crate::options::PdfOptions;

#[derive(Clone, Debug)]
pub(crate) struct DocxDocument {
  pub page: PageSetup,
  pub blocks: Vec<Block>,
}

#[derive(Clone, Debug)]
pub(crate) enum Block {
  Paragraph(Paragraph),
  Table(Table),
}

#[derive(Clone, Debug)]
pub(crate) struct Paragraph {
  pub runs: Vec<TextRun>,
  pub format: ParagraphFormat,
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
  pub column_widths_pt: Vec<f32>,
  pub rows: Vec<TableRow>,
}

#[derive(Clone, Debug)]
pub(crate) struct TableRow {
  pub cells: Vec<TableCell>,
}

#[derive(Clone, Debug)]
pub(crate) struct TableCell {
  pub blocks: Vec<Block>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ParagraphFormat {
  pub spacing_before_pt: f32,
  pub spacing_after_pt: f32,
  pub line_height_pt: Option<f32>,
  pub indent_left_pt: f32,
  pub indent_right_pt: f32,
  pub first_line_indent_pt: f32,
  pub page_break_before: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct TextRun {
  pub text: String,
  pub style: TextStyle,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TextStyle {
  pub font_size_pt: f32,
  pub bold: bool,
  pub italic: bool,
  pub color: RgbColor,
}

impl Default for TextStyle {
  fn default() -> Self {
    Self {
      font_size_pt: 11.0,
      bold: false,
      italic: false,
      color: RgbColor { r: 0, g: 0, b: 0 },
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RgbColor {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct PageSetup {
  pub width_pt: f32,
  pub height_pt: f32,
  pub margin_top_pt: f32,
  pub margin_right_pt: f32,
  pub margin_bottom_pt: f32,
  pub margin_left_pt: f32,
}

impl Default for PageSetup {
  fn default() -> Self {
    // Word's default Letter page with one-inch margins.
    Self {
      width_pt: 612.0,
      height_pt: 792.0,
      margin_top_pt: 72.0,
      margin_right_pt: 72.0,
      margin_bottom_pt: 72.0,
      margin_left_pt: 72.0,
    }
  }
}

pub(crate) fn extract(
  package: &mut WordprocessingDocument,
  _options: &PdfOptions,
) -> Result<DocxDocument> {
  let main = package.main_document_part()?;
  let document = main.root_element(package)?;
  let section = document
    .body
    .as_deref()
    .and_then(|body| body.w_sect_pr.as_deref());

  Ok(DocxDocument {
    page: section.map(page_setup).unwrap_or_default(),
    blocks: document
      .body
      .as_deref()
      .map(body_blocks)
      .unwrap_or_default(),
  })
}

fn body_blocks(body: &w::Body) -> Vec<Block> {
  body
    .body_choice
    .iter()
    .filter_map(|choice| match choice {
      w::BodyChoice::WP(paragraph) => Some(Block::Paragraph(Paragraph {
        runs: paragraph_runs(paragraph),
        format: paragraph_format(paragraph.paragraph_properties.as_deref()),
      })),
      w::BodyChoice::WTbl(table) => Some(Block::Table(table_model(table))),
      _ => None,
    })
    .collect()
}

fn table_model(table: &w::Table) -> Table {
  Table {
    column_widths_pt: table
      .w_tbl_grid
      .as_deref()
      .map(|grid| {
        grid
          .w_grid_col
          .iter()
          .filter_map(|column| column.width.as_deref().and_then(twips_attr_to_points))
          .collect()
      })
      .unwrap_or_default(),
    rows: table
      .table_choice2
      .iter()
      .filter_map(|choice| match choice {
        w::TableChoice2::WTr(row) => Some(table_row_model(row)),
        _ => None,
      })
      .collect(),
  }
}

fn table_row_model(row: &w::TableRow) -> TableRow {
  TableRow {
    cells: row
      .table_row_choice
      .iter()
      .filter_map(|choice| match choice {
        w::TableRowChoice::WTc(cell) => Some(table_cell_model(cell)),
        _ => None,
      })
      .collect(),
  }
}

fn table_cell_model(cell: &w::TableCell) -> TableCell {
  TableCell {
    blocks: cell
      .table_cell_choice
      .iter()
      .filter_map(|choice| match choice {
        w::TableCellChoice::WP(paragraph) => Some(Block::Paragraph(Paragraph {
          runs: paragraph_runs(paragraph),
          format: paragraph_format(paragraph.paragraph_properties.as_deref()),
        })),
        w::TableCellChoice::WTbl(table) => Some(Block::Table(table_model(table))),
        _ => None,
      })
      .collect(),
  }
}

fn paragraph_format(properties: Option<&w::ParagraphProperties>) -> ParagraphFormat {
  let mut format = ParagraphFormat::default();
  let Some(properties) = properties else {
    return format;
  };

  format.page_break_before = properties
    .page_break_before
    .as_ref()
    .map(|value| value.val.unwrap_or(true))
    .unwrap_or(false);

  if let Some(spacing) = &properties.spacing_between_lines {
    format.spacing_before_pt = spacing
      .before
      .as_deref()
      .and_then(twips_attr_to_points)
      .unwrap_or(0.0);
    format.spacing_after_pt = spacing
      .after
      .as_deref()
      .and_then(twips_attr_to_points)
      .unwrap_or(0.0);
    format.line_height_pt = spacing.line.as_deref().and_then(twips_attr_to_points);
    if let Some(line) = spacing.line.as_deref()
      && matches!(
        spacing.line_rule,
        None | Some(w::LineSpacingRuleValues::Auto)
      )
      && let Ok(value) = line.parse::<f32>()
    {
      format.line_height_pt = Some(14.0 * (value / 240.0).max(0.1));
    }
  }

  if let Some(indentation) = &properties.indentation {
    format.indent_left_pt = indentation
      .start
      .as_deref()
      .or(indentation.left.as_deref())
      .and_then(twips_attr_to_points)
      .unwrap_or(0.0);
    format.indent_right_pt = indentation
      .end
      .as_deref()
      .or(indentation.right.as_deref())
      .and_then(twips_attr_to_points)
      .unwrap_or(0.0);
    let first_line = indentation
      .first_line
      .as_deref()
      .and_then(twips_attr_to_points)
      .unwrap_or(0.0);
    let hanging = indentation
      .hanging
      .as_deref()
      .and_then(twips_attr_to_points)
      .unwrap_or(0.0);
    format.first_line_indent_pt = first_line - hanging;
  }

  format
}

fn paragraph_runs(paragraph: &w::Paragraph) -> Vec<TextRun> {
  let mut runs = Vec::new();

  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WR(run) => push_run(run, &mut runs),
      w::ParagraphChoice::WHyperlink(hyperlink) => {
        for item in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = item {
            push_run(run, &mut runs);
          }
        }
      }
      _ => {}
    }
  }

  runs
}

fn push_run(run: &w::Run, runs: &mut Vec<TextRun>) {
  let style = run_style(run.run_properties.as_deref());
  let mut text = String::new();

  for choice in &run.run_choice {
    match choice {
      w::RunChoice::WT(text_node) => {
        if let Some(content) = &text_node.xml_content {
          text.push_str(content);
        }
      }
      w::RunChoice::WTab => text.push('\t'),
      w::RunChoice::WCr => text.push('\n'),
      w::RunChoice::WBr(br) => match br.r#type {
        Some(w::BreakValues::Page) | Some(w::BreakValues::Column) => text.push('\n'),
        Some(w::BreakValues::TextWrapping) | None => text.push('\n'),
      },
      w::RunChoice::WNoBreakHyphen => text.push('\u{2011}'),
      w::RunChoice::WSoftHyphen => text.push('\u{00ad}'),
      _ => {}
    }
  }

  if !text.is_empty() {
    runs.push(TextRun { text, style });
  }
}

fn run_style(properties: Option<&w::RunProperties>) -> TextStyle {
  let mut style = TextStyle::default();
  let Some(properties) = properties else {
    return style;
  };

  if let Some(bold) = &properties.bold {
    style.bold = bold.val.unwrap_or(true);
  }
  if let Some(italic) = &properties.italic {
    style.italic = italic.val.unwrap_or(true);
  }
  if let Some(font_size) = &properties.font_size
    && let Ok(half_points) = font_size.val.parse::<f32>()
  {
    style.font_size_pt = (half_points / 2.0).max(1.0);
  }
  if let Some(color) = &properties.color
    && let Some(rgb) = parse_hex_color(&color.val)
  {
    style.color = rgb;
  }

  style
}

fn parse_hex_color(value: &str) -> Option<RgbColor> {
  if value.eq_ignore_ascii_case("auto") {
    return None;
  }

  let expanded;
  let hex = if value.len() == 3 {
    expanded = value.chars().flat_map(|ch| [ch, ch]).collect::<String>();
    expanded.as_str()
  } else {
    value
  };

  if hex.len() != 6 {
    return None;
  }

  Some(RgbColor {
    r: u8::from_str_radix(&hex[0..2], 16).ok()?,
    g: u8::from_str_radix(&hex[2..4], 16).ok()?,
    b: u8::from_str_radix(&hex[4..6], 16).ok()?,
  })
}

fn twips_attr_to_points(value: &str) -> Option<f32> {
  value.parse::<f32>().ok().map(units::twips_to_points)
}

fn page_setup(section: &w::SectionProperties) -> PageSetup {
  let mut setup = PageSetup::default();

  if let Some(size) = &section.w_pg_sz {
    if let Some(width) = size.width {
      setup.width_pt = units::twips_to_points(width as f32);
    }
    if let Some(height) = size.height {
      setup.height_pt = units::twips_to_points(height as f32);
    }
  }

  if let Some(margin) = &section.w_pg_mar {
    if let Some(top) = margin.top {
      setup.margin_top_pt = units::twips_to_points(top.max(0) as f32);
    }
    if let Some(right) = margin.right {
      setup.margin_right_pt = units::twips_to_points(right as f32);
    }
    if let Some(bottom) = margin.bottom {
      setup.margin_bottom_pt = units::twips_to_points(bottom.max(0) as f32);
    }
    if let Some(left) = margin.left {
      setup.margin_left_pt = units::twips_to_points(left as f32);
    }
  }

  setup
}
