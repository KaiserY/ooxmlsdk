mod units;

use std::collections::HashMap;

use ooxmlsdk::parts::{
  main_document_part::MainDocumentPart, wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use quick_xml::Reader;
use quick_xml::events::Event;

use crate::error::Result;
use crate::options::PdfOptions;

#[derive(Clone, Debug)]
pub(crate) struct DocxDocument {
  pub page: PageSetup,
  pub header_blocks: Vec<Block>,
  pub footer_blocks: Vec<Block>,
  pub blocks: Vec<Block>,
}

#[derive(Clone, Debug)]
pub(crate) enum Block {
  Paragraph(Paragraph),
  Table(Table),
}

#[derive(Clone, Debug)]
pub(crate) struct Paragraph {
  pub inlines: Vec<InlineItem>,
  #[cfg(test)]
  pub runs: Vec<TextRun>,
  pub format: ParagraphFormat,
  pub list_label: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
  pub column_widths_pt: Vec<f32>,
  pub preferred_width_pt: Option<f32>,
  pub borders: Option<TableBordersModel>,
  pub rows: Vec<TableRow>,
}

#[derive(Clone, Debug)]
pub(crate) struct TableRow {
  pub cells: Vec<TableCell>,
}

#[derive(Clone, Debug)]
pub(crate) struct TableCell {
  pub blocks: Vec<Block>,
  pub shading: Option<RgbColor>,
  pub borders: CellBordersModel,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct TableBordersModel {
  pub top: Option<BorderStyle>,
  pub right: Option<BorderStyle>,
  pub bottom: Option<BorderStyle>,
  pub left: Option<BorderStyle>,
  pub inside_horizontal: Option<BorderStyle>,
  pub inside_vertical: Option<BorderStyle>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CellBordersModel {
  pub top: Option<BorderStyle>,
  pub right: Option<BorderStyle>,
  pub bottom: Option<BorderStyle>,
  pub left: Option<BorderStyle>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct BorderStyle {
  pub width_pt: f32,
  pub color: RgbColor,
}

impl Default for BorderStyle {
  fn default() -> Self {
    Self {
      width_pt: 0.5,
      color: RgbColor { r: 0, g: 0, b: 0 },
    }
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ParagraphFormat {
  pub spacing_before_pt: f32,
  pub spacing_after_pt: f32,
  pub line_height_pt: Option<f32>,
  pub indent_left_pt: f32,
  pub indent_right_pt: f32,
  pub first_line_indent_pt: f32,
  pub alignment: ParagraphAlignment,
  pub shading: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub page_break_before: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ParagraphAlignment {
  #[default]
  Left,
  Center,
  Right,
  Justify,
}

#[derive(Clone, Debug)]
pub(crate) struct TextRun {
  pub text: String,
  pub style: TextStyle,
}

#[derive(Clone, Debug)]
pub(crate) enum InlineItem {
  Text(TextRun),
  Image(InlineImage),
}

#[derive(Clone, Debug)]
pub(crate) struct InlineImage {
  pub data: Vec<u8>,
  pub content_type: Option<String>,
  pub width_pt: f32,
  pub height_pt: f32,
  pub alt_text: Option<String>,
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
  pub header_distance_pt: f32,
  pub footer_distance_pt: f32,
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
      header_distance_pt: 36.0,
      footer_distance_pt: 36.0,
    }
  }
}

pub(crate) fn extract(
  package: &mut WordprocessingDocument,
  _options: &PdfOptions,
) -> Result<DocxDocument> {
  let main = package.main_document_part()?;
  let styles = StylesCatalog::load(package, &main)?;
  let mut numbering = NumberingCatalog::load(package, &main)?;
  let images = ImageCatalog::load(package, &main);
  let document = main.root_element(package)?;
  let section = document
    .body
    .as_deref()
    .and_then(|body| body.w_sect_pr.as_deref())
    .cloned();
  let page = section.as_ref().map(page_setup).unwrap_or_default();
  let blocks = document
    .body
    .as_deref()
    .map(|body| body_blocks(body, &styles, &mut numbering, &images))
    .unwrap_or_default();
  let header_blocks = section
    .as_ref()
    .and_then(|section| header_blocks(package, &main, section, &styles))
    .unwrap_or_default();
  let footer_blocks = section
    .as_ref()
    .and_then(|section| footer_blocks(package, &main, section, &styles))
    .unwrap_or_default();

  Ok(DocxDocument {
    page,
    header_blocks,
    footer_blocks,
    blocks,
  })
}

fn body_blocks(
  body: &w::Body,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
) -> Vec<Block> {
  body
    .body_choice
    .iter()
    .filter_map(|choice| match choice {
      w::BodyChoice::WP(paragraph) => Some(Block::Paragraph(paragraph_model(
        paragraph, styles, numbering, images,
      ))),
      w::BodyChoice::WTbl(table) => {
        Some(Block::Table(table_model(table, styles, numbering, images)))
      }
      _ => None,
    })
    .collect()
}

fn header_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
) -> Option<Vec<Block>> {
  let relationship_id =
    section
      .section_properties_choice
      .iter()
      .find_map(|choice| match choice {
        w::SectionPropertiesChoice::WHeaderReference(reference)
          if reference.r#type == w::HeaderFooterValues::Default =>
        {
          Some(reference.id.as_str())
        }
        _ => None,
      })?;
  let header_part = main
    .header_parts(package)
    .find(|part| main.get_id_of_part(package, part) == Some(relationship_id))?;
  let header = header_part.root_element(package).ok()?;
  let images = ImageCatalog::default();
  let mut numbering = NumberingCatalog::default();
  Some(
    header
      .header_choice
      .iter()
      .filter_map(|choice| match choice {
        w::HeaderChoice::WP(paragraph) => Some(Block::Paragraph(paragraph_model(
          paragraph,
          styles,
          &mut numbering,
          &images,
        ))),
        w::HeaderChoice::WTbl(table) => Some(Block::Table(table_model(
          table,
          styles,
          &mut numbering,
          &images,
        ))),
        _ => None,
      })
      .collect(),
  )
}

fn footer_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
) -> Option<Vec<Block>> {
  let relationship_id =
    section
      .section_properties_choice
      .iter()
      .find_map(|choice| match choice {
        w::SectionPropertiesChoice::WFooterReference(reference)
          if reference.r#type == w::HeaderFooterValues::Default =>
        {
          Some(reference.id.as_str())
        }
        _ => None,
      })?;
  let footer_part = main
    .footer_parts(package)
    .find(|part| main.get_id_of_part(package, part) == Some(relationship_id))?;
  let footer = footer_part.root_element(package).ok()?;
  let images = ImageCatalog::default();
  let mut numbering = NumberingCatalog::default();
  Some(
    footer
      .footer_choice
      .iter()
      .filter_map(|choice| match choice {
        w::FooterChoice::WP(paragraph) => Some(Block::Paragraph(paragraph_model(
          paragraph,
          styles,
          &mut numbering,
          &images,
        ))),
        w::FooterChoice::WTbl(table) => Some(Block::Table(table_model(
          table,
          styles,
          &mut numbering,
          &images,
        ))),
        _ => None,
      })
      .collect(),
  )
}

fn paragraph_model(
  paragraph: &w::Paragraph,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
) -> Paragraph {
  let style_id = paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.paragraph_style_id.as_ref())
    .map(|style| style.val.as_str());
  let mut format = styles.paragraph_format(style_id);
  merge_paragraph_format(
    &mut format,
    paragraph
      .paragraph_properties
      .as_deref()
      .map(ParagraphProps::Direct),
  );
  let list_label = paragraph
    .paragraph_properties
    .as_deref()
    .and_then(|properties| properties.numbering_properties.as_deref())
    .and_then(|properties| numbering.next_label(properties, &mut format));

  let inlines = paragraph_inlines(paragraph, styles.run_style(style_id), images);
  #[cfg(test)]
  let runs = inlines
    .iter()
    .filter_map(|item| match item {
      InlineItem::Text(run) => Some(run.clone()),
      InlineItem::Image(_) => None,
    })
    .collect();

  Paragraph {
    inlines,
    #[cfg(test)]
    runs,
    format,
    list_label,
  }
}

fn table_model(
  table: &w::Table,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
) -> Table {
  let properties = table.w_tbl_pr.as_deref();
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
    preferred_width_pt: properties
      .and_then(|properties| properties.table_width.as_ref())
      .and_then(table_width_to_points),
    borders: properties
      .and_then(|properties| properties.table_borders.as_deref())
      .map(table_borders_model),
    rows: table
      .table_choice2
      .iter()
      .filter_map(|choice| match choice {
        w::TableChoice2::WTr(row) => Some(table_row_model(row, styles, numbering, images)),
        _ => None,
      })
      .collect(),
  }
}

fn table_row_model(
  row: &w::TableRow,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
) -> TableRow {
  TableRow {
    cells: row
      .table_row_choice
      .iter()
      .filter_map(|choice| match choice {
        w::TableRowChoice::WTc(cell) => Some(table_cell_model(cell, styles, numbering, images)),
        _ => None,
      })
      .collect(),
  }
}

fn table_cell_model(
  cell: &w::TableCell,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
) -> TableCell {
  let properties = cell.table_cell_properties.as_deref();
  TableCell {
    blocks: cell
      .table_cell_choice
      .iter()
      .filter_map(|choice| match choice {
        w::TableCellChoice::WP(paragraph) => Some(Block::Paragraph(paragraph_model(
          paragraph, styles, numbering, images,
        ))),
        w::TableCellChoice::WTbl(table) => {
          Some(Block::Table(table_model(table, styles, numbering, images)))
        }
        _ => None,
      })
      .collect(),
    shading: properties
      .and_then(|properties| properties.shading.as_ref())
      .and_then(shading_fill),
    borders: properties
      .and_then(|properties| properties.table_cell_borders.as_deref())
      .map(cell_borders_model)
      .unwrap_or_default(),
  }
}

fn table_width_to_points(width: &w::TableWidth) -> Option<f32> {
  match width.r#type {
    Some(w::TableWidthUnitValues::Dxa) | None => {
      width.width.as_deref().and_then(twips_attr_to_points)
    }
    _ => None,
  }
}

fn shading_fill(shading: &w::Shading) -> Option<RgbColor> {
  shading.fill.as_deref().and_then(parse_hex_color)
}

fn table_borders_model(borders: &w::TableBorders) -> TableBordersModel {
  TableBordersModel {
    top: borders.top_border.as_ref().and_then(top_border_style),
    right: borders
      .end_border
      .as_ref()
      .and_then(end_border_style)
      .or_else(|| borders.right_border.as_ref().and_then(right_border_style)),
    bottom: borders.bottom_border.as_ref().and_then(bottom_border_style),
    left: borders
      .start_border
      .as_ref()
      .and_then(start_border_style)
      .or_else(|| borders.left_border.as_ref().and_then(left_border_style)),
    inside_horizontal: borders
      .inside_horizontal_border
      .as_ref()
      .and_then(inside_horizontal_border_style),
    inside_vertical: borders
      .inside_vertical_border
      .as_ref()
      .and_then(inside_vertical_border_style),
  }
}

fn cell_borders_model(borders: &w::TableCellBorders) -> CellBordersModel {
  CellBordersModel {
    top: borders.top_border.as_ref().and_then(top_border_style),
    right: borders
      .end_border
      .as_ref()
      .and_then(end_border_style)
      .or_else(|| borders.right_border.as_ref().and_then(right_border_style)),
    bottom: borders.bottom_border.as_ref().and_then(bottom_border_style),
    left: borders
      .start_border
      .as_ref()
      .and_then(start_border_style)
      .or_else(|| borders.left_border.as_ref().and_then(left_border_style)),
  }
}

fn paragraph_borders_model(borders: &w::ParagraphBorders) -> CellBordersModel {
  CellBordersModel {
    top: borders.top_border.as_ref().and_then(top_border_style),
    right: borders.right_border.as_ref().and_then(right_border_style),
    bottom: borders.bottom_border.as_ref().and_then(bottom_border_style),
    left: borders.left_border.as_ref().and_then(left_border_style),
  }
}

macro_rules! border_style_fn {
  ($name:ident, $ty:ty) => {
    fn $name(border: &$ty) -> Option<BorderStyle> {
      border_style(border.val, border.size, border.color.as_deref())
    }
  };
}

border_style_fn!(top_border_style, w::TopBorder);
border_style_fn!(right_border_style, w::RightBorder);
border_style_fn!(bottom_border_style, w::BottomBorder);
border_style_fn!(left_border_style, w::LeftBorder);
border_style_fn!(start_border_style, w::StartBorder);
border_style_fn!(end_border_style, w::EndBorder);
border_style_fn!(inside_horizontal_border_style, w::InsideHorizontalBorder);
border_style_fn!(inside_vertical_border_style, w::InsideVerticalBorder);

fn border_style(
  value: w::BorderValues,
  size: Option<u32>,
  color: Option<&str>,
) -> Option<BorderStyle> {
  if matches!(value, w::BorderValues::Nil | w::BorderValues::None) {
    return None;
  }

  Some(BorderStyle {
    width_pt: size
      .map(|value| value as f32 / 8.0)
      .unwrap_or(0.5)
      .max(0.25),
    color: color.and_then(parse_hex_color).unwrap_or_default(),
  })
}

fn merge_paragraph_format(format: &mut ParagraphFormat, properties: Option<ParagraphProps<'_>>) {
  let Some(properties) = properties else {
    return;
  };

  if let Some(page_break_before) = properties.page_break_before() {
    format.page_break_before = page_break_before.val.unwrap_or(true);
  }

  if let Some(spacing) = properties.spacing_between_lines() {
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

  if let Some(indentation) = properties.indentation() {
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

  if let Some(justification) = properties.justification() {
    format.alignment = match justification.val {
      w::JustificationValues::Center => ParagraphAlignment::Center,
      w::JustificationValues::Right | w::JustificationValues::End => ParagraphAlignment::Right,
      w::JustificationValues::Both
      | w::JustificationValues::Distribute
      | w::JustificationValues::MediumKashida
      | w::JustificationValues::HighKashida
      | w::JustificationValues::LowKashida
      | w::JustificationValues::ThaiDistribute => ParagraphAlignment::Justify,
      w::JustificationValues::Left
      | w::JustificationValues::Start
      | w::JustificationValues::NumTab => ParagraphAlignment::Left,
    };
  }

  if let Some(shading) = properties.shading() {
    format.shading = shading_fill(shading);
  }

  if let Some(borders) = properties.paragraph_borders() {
    format.borders = paragraph_borders_model(borders);
  }
}

fn paragraph_inlines(
  paragraph: &w::Paragraph,
  base_style: TextStyle,
  images: &ImageCatalog,
) -> Vec<InlineItem> {
  let mut inlines = Vec::new();

  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WR(run) => push_run(run, &mut inlines, base_style, images),
      w::ParagraphChoice::WHyperlink(hyperlink) => {
        for item in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = item {
            push_run(run, &mut inlines, base_style, images);
          }
        }
      }
      _ => {}
    }
  }

  inlines
}

fn push_run(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  images: &ImageCatalog,
) {
  let style = run_style(run.run_properties.as_deref(), base_style);
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
      w::RunChoice::WDrawing(drawing) => {
        flush_run_text(inlines, &mut text, style);
        if let Some(image) = inline_image(drawing, images) {
          inlines.push(InlineItem::Image(image));
        }
      }
      _ => {}
    }
  }

  flush_run_text(inlines, &mut text, style);
}

fn flush_run_text(inlines: &mut Vec<InlineItem>, text: &mut String, style: TextStyle) {
  if !text.is_empty() {
    inlines.push(InlineItem::Text(TextRun {
      text: std::mem::take(text),
      style,
    }));
  }
}

fn inline_image(drawing: &w::Drawing, images: &ImageCatalog) -> Option<InlineImage> {
  let w::DrawingChoice::WpInline(inline) = drawing.drawing_choice.as_ref()? else {
    return None;
  };
  let relationship_id = embedded_image_relationship_id(&inline.graphic.graphic_data)?;
  let resource = images.by_relationship_id.get(&relationship_id)?;
  Some(InlineImage {
    data: resource.data.clone(),
    content_type: resource.content_type.clone(),
    width_pt: units::emu_to_points(inline.extent.cx),
    height_pt: units::emu_to_points(inline.extent.cy),
    alt_text: inline.doc_properties.description.clone(),
  })
}

fn embedded_image_relationship_id(
  graphic_data: &ooxmlsdk::schemas::a::GraphicData,
) -> Option<String> {
  graphic_data
    .xml_children
    .iter()
    .find_map(|child| blip_embed_relationship_id(child))
}

fn blip_embed_relationship_id(xml: &str) -> Option<String> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(true);
  loop {
    match reader.read_event().ok()? {
      Event::Empty(event) | Event::Start(event) if event.name().as_ref().ends_with(b":blip") => {
        for attr in event.attributes().with_checks(false).flatten() {
          if attr.key.as_ref().ends_with(b":embed") || attr.key.as_ref() == b"embed" {
            return attr
              .decode_and_unescape_value(reader.decoder())
              .ok()
              .map(|value| value.into_owned());
          }
        }
      }
      Event::Eof => return None,
      _ => {}
    }
  }
}

#[derive(Clone, Debug, Default)]
struct ImageCatalog {
  by_relationship_id: HashMap<String, ImageResource>,
}

#[derive(Clone, Debug)]
struct ImageResource {
  data: Vec<u8>,
  content_type: Option<String>,
}

impl ImageCatalog {
  fn load(package: &WordprocessingDocument, main: &MainDocumentPart) -> Self {
    let mut by_relationship_id = HashMap::new();
    for image_part in main.image_parts(package) {
      let Some(relationship_id) = main.get_id_of_part(package, &image_part) else {
        continue;
      };
      let Some(data) = image_part.data_to_vec(package) else {
        continue;
      };
      by_relationship_id.insert(
        relationship_id.to_string(),
        ImageResource {
          data,
          content_type: image_part.content_type(package).map(str::to_string),
        },
      );
    }

    Self { by_relationship_id }
  }
}

fn run_style(properties: Option<&w::RunProperties>, base_style: TextStyle) -> TextStyle {
  let mut style = base_style;
  let Some(properties) = properties else {
    return style;
  };

  merge_run_style(&mut style, Some(RunProps::Direct(properties)));
  style
}

fn merge_run_style(style: &mut TextStyle, properties: Option<RunProps<'_>>) {
  let Some(properties) = properties else {
    return;
  };

  if let Some(bold) = properties.bold() {
    style.bold = bold.val.unwrap_or(true);
  }
  if let Some(italic) = properties.italic() {
    style.italic = italic.val.unwrap_or(true);
  }
  if let Some(font_size) = properties.font_size()
    && let Ok(half_points) = font_size.val.parse::<f32>()
  {
    style.font_size_pt = (half_points / 2.0).max(1.0);
  }
  if let Some(color) = properties.color()
    && let Some(rgb) = parse_hex_color(&color.val)
  {
    style.color = rgb;
  }
}

#[derive(Clone, Debug, Default)]
struct StylesCatalog {
  doc_default_paragraph: ParagraphFormat,
  doc_default_run: TextStyle,
  default_paragraph_style_id: Option<String>,
  styles: HashMap<String, StyleEntry>,
}

#[derive(Clone, Debug, Default)]
struct StyleEntry {
  based_on: Option<String>,
  paragraph_format: ParagraphFormat,
  run_style: TextStyle,
}

impl StylesCatalog {
  fn load(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> Result<Self> {
    let Some(styles_part) = main.style_definitions_part(package) else {
      return Ok(Self::default());
    };
    let styles = styles_part.root_element(package)?;
    let mut catalog = Self::default();

    if let Some(defaults) = styles.doc_defaults.as_deref() {
      merge_paragraph_format(
        &mut catalog.doc_default_paragraph,
        defaults
          .paragraph_properties_default
          .as_deref()
          .and_then(|default| default.paragraph_properties_base_style.as_deref())
          .map(ParagraphProps::BaseStyle),
      );
      merge_run_style(
        &mut catalog.doc_default_run,
        defaults
          .run_properties_default
          .as_deref()
          .and_then(|default| default.run_properties_base_style.as_deref())
          .map(RunProps::BaseStyle),
      );
    }

    for style in &styles.w_style {
      let Some(style_id) = &style.style_id else {
        continue;
      };
      if matches!(style.r#type, Some(w::StyleValues::Paragraph)) && style.default.unwrap_or(false) {
        catalog.default_paragraph_style_id = Some(style_id.to_string());
      }
      let mut entry = StyleEntry {
        based_on: style
          .based_on
          .as_ref()
          .map(|based_on| based_on.val.to_string()),
        paragraph_format: ParagraphFormat::default(),
        run_style: TextStyle::default(),
      };
      merge_paragraph_format(
        &mut entry.paragraph_format,
        style
          .style_paragraph_properties
          .as_deref()
          .map(ParagraphProps::Style),
      );
      merge_run_style(
        &mut entry.run_style,
        style.style_run_properties.as_deref().map(RunProps::Style),
      );
      catalog.styles.insert(style_id.to_string(), entry);
    }

    Ok(catalog)
  }

  fn paragraph_format(&self, style_id: Option<&str>) -> ParagraphFormat {
    let mut format = self.doc_default_paragraph;
    let style_id = style_id
      .map(str::to_string)
      .or_else(|| self.default_paragraph_style_id.clone());
    for entry in self.style_chain(style_id.as_deref()) {
      merge_format_values(&mut format, entry.paragraph_format);
    }
    format
  }

  fn run_style(&self, style_id: Option<&str>) -> TextStyle {
    let mut style = self.doc_default_run;
    let style_id = style_id
      .map(str::to_string)
      .or_else(|| self.default_paragraph_style_id.clone());
    for entry in self.style_chain(style_id.as_deref()) {
      merge_style_values(&mut style, entry.run_style);
    }
    style
  }

  fn style_chain(&self, style_id: Option<&str>) -> Vec<&StyleEntry> {
    let mut ids = Vec::new();
    let mut current = style_id;
    while let Some(id) = current {
      if ids.contains(&id) {
        break;
      }
      let Some(entry) = self.styles.get(id) else {
        break;
      };
      ids.push(id);
      current = entry.based_on.as_deref();
    }

    ids
      .into_iter()
      .rev()
      .filter_map(|id| self.styles.get(id))
      .collect()
  }
}

fn merge_format_values(target: &mut ParagraphFormat, values: ParagraphFormat) {
  if values.spacing_before_pt != 0.0 {
    target.spacing_before_pt = values.spacing_before_pt;
  }
  if values.spacing_after_pt != 0.0 {
    target.spacing_after_pt = values.spacing_after_pt;
  }
  if values.line_height_pt.is_some() {
    target.line_height_pt = values.line_height_pt;
  }
  if values.indent_left_pt != 0.0 {
    target.indent_left_pt = values.indent_left_pt;
  }
  if values.indent_right_pt != 0.0 {
    target.indent_right_pt = values.indent_right_pt;
  }
  if values.first_line_indent_pt != 0.0 {
    target.first_line_indent_pt = values.first_line_indent_pt;
  }
  if values.page_break_before {
    target.page_break_before = true;
  }
}

fn merge_style_values(target: &mut TextStyle, values: TextStyle) {
  if (values.font_size_pt - TextStyle::default().font_size_pt).abs() > f32::EPSILON {
    target.font_size_pt = values.font_size_pt;
  }
  if values.bold {
    target.bold = true;
  }
  if values.italic {
    target.italic = true;
  }
  if values.color != TextStyle::default().color {
    target.color = values.color;
  }
}

#[derive(Clone, Debug, Default)]
struct NumberingCatalog {
  nums: HashMap<i32, NumberingInstance>,
  abstract_nums: HashMap<i32, AbstractNumbering>,
  counters: HashMap<(i32, i32), i32>,
}

#[derive(Clone, Debug)]
struct NumberingInstance {
  abstract_num_id: i32,
  overrides: HashMap<i32, LevelOverride>,
}

#[derive(Clone, Debug)]
struct LevelOverride {
  start: Option<i32>,
  level: Option<NumberingLevel>,
}

#[derive(Clone, Debug, Default)]
struct AbstractNumbering {
  levels: HashMap<i32, NumberingLevel>,
}

#[derive(Clone, Debug)]
struct NumberingLevel {
  start: i32,
  format: w::NumberFormatValues,
  text: String,
  format_properties: ParagraphFormat,
}

impl NumberingCatalog {
  fn load(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> Result<Self> {
    let Some(numbering_part) = main.numbering_definitions_part(package) else {
      return Ok(Self::default());
    };
    let numbering = numbering_part.root_element(package)?;
    let mut catalog = Self::default();

    for abstract_num in &numbering.w_abstract_num {
      let mut entry = AbstractNumbering::default();
      for level in &abstract_num.w_lvl {
        entry
          .levels
          .insert(level.level_index, numbering_level_model(level));
      }
      catalog
        .abstract_nums
        .insert(abstract_num.abstract_number_id, entry);
    }

    for num in &numbering.w_num {
      let overrides = num
        .w_lvl_override
        .iter()
        .map(|level| {
          (
            level.level_index,
            LevelOverride {
              start: level
                .start_override_numbering_value
                .as_ref()
                .map(|value| value.val),
              level: level.level.as_deref().map(numbering_level_model),
            },
          )
        })
        .collect();
      catalog.nums.insert(
        num.number_id,
        NumberingInstance {
          abstract_num_id: num.abstract_num_id.val,
          overrides,
        },
      );
    }

    Ok(catalog)
  }

  fn next_label(
    &mut self,
    properties: &w::NumberingProperties,
    format: &mut ParagraphFormat,
  ) -> Option<String> {
    let num_id = properties.numbering_id.as_ref()?.val;
    let level_index = properties
      .numbering_level_reference
      .as_ref()
      .map(|level| level.val)
      .unwrap_or(0);
    let instance = self.nums.get(&num_id)?;
    let abstract_num = self.abstract_nums.get(&instance.abstract_num_id)?;
    let level_override = instance.overrides.get(&level_index);
    let level = level_override
      .and_then(|override_| override_.level.as_ref())
      .or_else(|| abstract_num.levels.get(&level_index))?;

    merge_format_values(format, level.format_properties);
    let start = level_override
      .and_then(|override_| override_.start)
      .unwrap_or(level.start);
    let counter = {
      let counter = self
        .counters
        .entry((num_id, level_index))
        .or_insert(start - 1);
      *counter += 1;
      *counter
    };
    for key_level in (level_index + 1)..=8 {
      self.counters.remove(&(num_id, key_level));
    }

    Some(format_numbering_label(level, counter))
  }
}

fn numbering_level_model(level: &w::Level) -> NumberingLevel {
  let mut format_properties = ParagraphFormat::default();
  merge_paragraph_format(
    &mut format_properties,
    level
      .previous_paragraph_properties
      .as_deref()
      .map(ParagraphProps::Previous),
  );

  NumberingLevel {
    start: level
      .start_numbering_value
      .as_ref()
      .map(|value| value.val)
      .unwrap_or(1),
    format: level
      .numbering_format
      .as_ref()
      .map(|format| format.val)
      .unwrap_or_default(),
    text: level
      .level_text
      .as_ref()
      .and_then(|text| text.val.as_ref())
      .map(ToString::to_string)
      .unwrap_or_else(|| "%1.".to_string()),
    format_properties,
  }
}

fn format_numbering_label(level: &NumberingLevel, value: i32) -> String {
  if matches!(level.format, w::NumberFormatValues::Bullet) {
    return format!("{} ", level.text);
  }

  let value = match level.format {
    w::NumberFormatValues::LowerLetter => alpha_number(value, false),
    w::NumberFormatValues::UpperLetter => alpha_number(value, true),
    w::NumberFormatValues::LowerRoman => roman_number(value).to_lowercase(),
    w::NumberFormatValues::UpperRoman => roman_number(value),
    _ => value.to_string(),
  };
  let text = level
    .text
    .replace("%1", &value)
    .replace("%2", &value)
    .replace("%3", &value);
  format!("{text} ")
}

fn alpha_number(mut value: i32, upper: bool) -> String {
  if value <= 0 {
    return value.to_string();
  }
  let mut chars = Vec::new();
  while value > 0 {
    value -= 1;
    let base = if upper { b'A' } else { b'a' };
    chars.push((base + (value % 26) as u8) as char);
    value /= 26;
  }
  chars.iter().rev().collect()
}

fn roman_number(mut value: i32) -> String {
  if !(1..=3999).contains(&value) {
    return value.to_string();
  }
  let mut output = String::new();
  for (arabic, roman) in [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
  ] {
    while value >= arabic {
      output.push_str(roman);
      value -= arabic;
    }
  }
  output
}

enum ParagraphProps<'a> {
  Direct(&'a w::ParagraphProperties),
  Style(&'a w::StyleParagraphProperties),
  BaseStyle(&'a w::ParagraphPropertiesBaseStyle),
  Previous(&'a w::PreviousParagraphProperties),
}

impl<'a> ParagraphProps<'a> {
  fn page_break_before(&self) -> Option<&'a w::PageBreakBefore> {
    match self {
      Self::Direct(properties) => properties.page_break_before.as_ref(),
      Self::Style(properties) => properties.page_break_before.as_ref(),
      Self::BaseStyle(properties) => properties.page_break_before.as_ref(),
      Self::Previous(properties) => properties.page_break_before.as_ref(),
    }
  }

  fn spacing_between_lines(&self) -> Option<&'a w::SpacingBetweenLines> {
    match self {
      Self::Direct(properties) => properties.spacing_between_lines.as_ref(),
      Self::Style(properties) => properties.spacing_between_lines.as_ref(),
      Self::BaseStyle(properties) => properties.spacing_between_lines.as_ref(),
      Self::Previous(properties) => properties.spacing_between_lines.as_ref(),
    }
  }

  fn indentation(&self) -> Option<&'a w::Indentation> {
    match self {
      Self::Direct(properties) => properties.indentation.as_ref(),
      Self::Style(properties) => properties.indentation.as_ref(),
      Self::BaseStyle(properties) => properties.indentation.as_ref(),
      Self::Previous(properties) => properties.indentation.as_ref(),
    }
  }

  fn justification(&self) -> Option<&'a w::Justification> {
    match self {
      Self::Direct(properties) => properties.justification.as_ref(),
      Self::Style(properties) => properties.justification.as_ref(),
      Self::BaseStyle(properties) => properties.justification.as_ref(),
      Self::Previous(properties) => properties.justification.as_ref(),
    }
  }

  fn paragraph_borders(&self) -> Option<&'a w::ParagraphBorders> {
    match self {
      Self::Direct(properties) => properties.paragraph_borders.as_deref(),
      Self::Style(properties) => properties.paragraph_borders.as_deref(),
      Self::BaseStyle(properties) => properties.paragraph_borders.as_deref(),
      Self::Previous(properties) => properties.paragraph_borders.as_deref(),
    }
  }

  fn shading(&self) -> Option<&'a w::Shading> {
    match self {
      Self::Direct(properties) => properties.shading.as_ref(),
      Self::Style(properties) => properties.shading.as_ref(),
      Self::BaseStyle(properties) => properties.shading.as_ref(),
      Self::Previous(properties) => properties.shading.as_ref(),
    }
  }
}

enum RunProps<'a> {
  Direct(&'a w::RunProperties),
  Style(&'a w::StyleRunProperties),
  BaseStyle(&'a w::RunPropertiesBaseStyle),
}

impl<'a> RunProps<'a> {
  fn bold(&self) -> Option<&'a w::Bold> {
    match self {
      Self::Direct(properties) => properties.bold.as_ref(),
      Self::Style(properties) => properties.bold.as_ref(),
      Self::BaseStyle(properties) => properties.bold.as_ref(),
    }
  }

  fn italic(&self) -> Option<&'a w::Italic> {
    match self {
      Self::Direct(properties) => properties.italic.as_ref(),
      Self::Style(properties) => properties.italic.as_ref(),
      Self::BaseStyle(properties) => properties.italic.as_ref(),
    }
  }

  fn font_size(&self) -> Option<&'a w::FontSize> {
    match self {
      Self::Direct(properties) => properties.font_size.as_ref(),
      Self::Style(properties) => properties.font_size.as_ref(),
      Self::BaseStyle(properties) => properties.font_size.as_ref(),
    }
  }

  fn color(&self) -> Option<&'a w::Color> {
    match self {
      Self::Direct(properties) => properties.color.as_ref(),
      Self::Style(properties) => properties.color.as_ref(),
      Self::BaseStyle(properties) => properties.color.as_ref(),
    }
  }
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
    if let Some(header) = margin.header {
      setup.header_distance_pt = units::twips_to_points(header as f32);
    }
    if let Some(footer) = margin.footer {
      setup.footer_distance_pt = units::twips_to_points(footer as f32);
    }
  }

  setup
}
