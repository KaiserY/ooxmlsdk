mod units;

use std::collections::HashMap;

use ooxmlsdk::parts::{
  endnotes_part::EndnotesPart, footer_part::FooterPart, footnotes_part::FootnotesPart,
  header_part::HeaderPart, image_part::ImagePart, main_document_part::MainDocumentPart,
  wordprocessing_comments_part::WordprocessingCommentsPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::{
  schemas_microsoft_com_vml as v, schemas_openxmlformats_org_wordprocessingml_2006_main as w,
};
use quick_xml::Reader;
use quick_xml::events::Event;

use crate::error::Result;
use crate::options::PdfOptions;

const DEFAULT_TAB_STOP_PT: f32 = 36.0;

#[derive(Clone, Debug)]
pub(crate) struct DocxDocument {
  pub page: PageSetup,
  pub default_tab_stop_pt: f32,
  pub header_blocks: Vec<Block>,
  pub footer_blocks: Vec<Block>,
  pub first_header_blocks: Vec<Block>,
  pub first_footer_blocks: Vec<Block>,
  pub footnote_blocks: Vec<Block>,
  pub endnote_blocks: Vec<Block>,
  pub comment_blocks: Vec<Block>,
  pub title_page: bool,
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
  pub height_pt: Option<f32>,
  pub exact_height: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct TableCell {
  pub blocks: Vec<Block>,
  pub shading: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub grid_span: usize,
  pub vertical_merge_continue: bool,
  pub vertical_alignment: TableCellVerticalAlignment,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TableCellVerticalAlignment {
  #[default]
  Top,
  Center,
  Bottom,
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

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ParagraphFormat {
  pub spacing_before_pt: f32,
  pub spacing_after_pt: f32,
  pub line_height_pt: Option<f32>,
  pub indent_left_pt: f32,
  pub indent_right_pt: f32,
  pub first_line_indent_pt: f32,
  pub tab_stops: Vec<TabStop>,
  pub alignment: ParagraphAlignment,
  pub shading: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub page_break_before: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TabStop {
  pub position_pt: f32,
  pub alignment: TabStopAlignment,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TabStopAlignment {
  #[default]
  Left,
  Center,
  Right,
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
  PageBreak,
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
  pub baseline_shift_pt: f32,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
  pub strikethrough: bool,
  pub uppercase: bool,
  pub color: RgbColor,
  pub highlight: Option<RgbColor>,
}

impl Default for TextStyle {
  fn default() -> Self {
    Self {
      font_size_pt: 11.0,
      baseline_shift_pt: 0.0,
      bold: false,
      italic: false,
      underline: false,
      strikethrough: false,
      uppercase: false,
      color: RgbColor { r: 0, g: 0, b: 0 },
      highlight: None,
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
  let default_tab_stop_pt = default_tab_stop_pt(package, &main);
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
    .and_then(|section| {
      referenced_header_blocks(
        package,
        &main,
        section,
        &styles,
        w::HeaderFooterValues::Default,
      )
    })
    .unwrap_or_default();
  let footer_blocks = section
    .as_ref()
    .and_then(|section| {
      referenced_footer_blocks(
        package,
        &main,
        section,
        &styles,
        w::HeaderFooterValues::Default,
      )
    })
    .unwrap_or_default();
  let first_header_blocks = section
    .as_ref()
    .and_then(|section| {
      referenced_header_blocks(
        package,
        &main,
        section,
        &styles,
        w::HeaderFooterValues::First,
      )
    })
    .unwrap_or_default();
  let first_footer_blocks = section
    .as_ref()
    .and_then(|section| {
      referenced_footer_blocks(
        package,
        &main,
        section,
        &styles,
        w::HeaderFooterValues::First,
      )
    })
    .unwrap_or_default();
  let footnote_blocks = footnote_blocks(package, &main, &styles)?;
  let endnote_blocks = endnote_blocks(package, &main, &styles)?;
  let comment_blocks = comment_blocks(package, &main, &styles)?;
  let title_page = section
    .as_ref()
    .and_then(|section| section.w_title_pg.as_ref())
    .map(|title_page| title_page.val.unwrap_or(true))
    .unwrap_or(false);

  Ok(DocxDocument {
    page,
    default_tab_stop_pt,
    header_blocks,
    footer_blocks,
    first_header_blocks,
    first_footer_blocks,
    footnote_blocks,
    endnote_blocks,
    comment_blocks,
    title_page,
    blocks,
  })
}

fn default_tab_stop_pt(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> f32 {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .w_default_tab_stop
        .as_ref()
        .map(|stop| stop.val as f32 / 20.0)
    })
    .filter(|value| value.is_finite() && *value > 0.0)
    .unwrap_or(DEFAULT_TAB_STOP_PT)
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
      w::BodyChoice::WP(paragraph) => Some(vec![Block::Paragraph(paragraph_model(
        paragraph, styles, numbering, images,
      ))]),
      w::BodyChoice::WTbl(table) => Some(vec![Block::Table(table_model(
        table, styles, numbering, images,
      ))]),
      w::BodyChoice::WSdt(sdt) => Some(sdt_block_blocks(sdt, styles, numbering, images)),
      _ => None,
    })
    .flatten()
    .collect()
}

fn sdt_block_blocks(
  sdt: &w::SdtBlock,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
) -> Vec<Block> {
  let Some(content) = sdt.sdt_content_block.as_ref() else {
    return Vec::new();
  };

  content
    .sdt_content_block_choice
    .iter()
    .filter_map(|choice| match choice {
      w::SdtContentBlockChoice::WP(paragraph) => Some(vec![Block::Paragraph(paragraph_model(
        paragraph.as_ref(),
        styles,
        numbering,
        images,
      ))]),
      w::SdtContentBlockChoice::WTbl(table) => Some(vec![Block::Table(table_model(
        table.as_ref(),
        styles,
        numbering,
        images,
      ))]),
      w::SdtContentBlockChoice::WSdt(sdt) => {
        Some(sdt_block_blocks(sdt.as_ref(), styles, numbering, images))
      }
      _ => None,
    })
    .flatten()
    .collect()
}

fn header_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  header_type: w::HeaderFooterValues,
) -> Option<Vec<Block>> {
  let relationship_id =
    section
      .section_properties_choice
      .iter()
      .find_map(|choice| match choice {
        w::SectionPropertiesChoice::WHeaderReference(reference)
          if reference.r#type == header_type =>
        {
          Some(reference.id.as_str())
        }
        _ => None,
      })?;
  let header_part = main
    .header_parts(package)
    .find(|part| main.get_id_of_part(package, part) == Some(relationship_id))?;
  let images = ImageCatalog::load_from_header(package, &header_part);
  let header = header_part.root_element(package).ok()?;
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

fn referenced_header_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  header_type: w::HeaderFooterValues,
) -> Option<Vec<Block>> {
  header_blocks(package, main, section, styles, header_type)
}

fn footer_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  footer_type: w::HeaderFooterValues,
) -> Option<Vec<Block>> {
  let relationship_id =
    section
      .section_properties_choice
      .iter()
      .find_map(|choice| match choice {
        w::SectionPropertiesChoice::WFooterReference(reference)
          if reference.r#type == footer_type =>
        {
          Some(reference.id.as_str())
        }
        _ => None,
      })?;
  let footer_part = main
    .footer_parts(package)
    .find(|part| main.get_id_of_part(package, part) == Some(relationship_id))?;
  let images = ImageCatalog::load_from_footer(package, &footer_part);
  let footer = footer_part.root_element(package).ok()?;
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

fn referenced_footer_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  section: &w::SectionProperties,
  styles: &StylesCatalog,
  footer_type: w::HeaderFooterValues,
) -> Option<Vec<Block>> {
  footer_blocks(package, main, section, styles, footer_type)
}

fn footnote_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Result<Vec<Block>> {
  let Some(part) = main.footnotes_part(package) else {
    return Ok(Vec::new());
  };
  let images = ImageCatalog::load_from_footnotes(package, &part);
  let footnotes = part.root_element(package)?;
  let mut numbering = NumberingCatalog::default();
  let mut blocks = Vec::new();

  for footnote in &footnotes.w_footnote {
    if footnote.id < 1
      || !matches!(
        footnote.r#type,
        None | Some(w::FootnoteEndnoteValues::Normal)
      )
    {
      continue;
    }
    append_note_blocks(
      &mut blocks,
      format!("{} ", footnote.id),
      footnote
        .footnote_choice
        .iter()
        .filter_map(|choice| match choice {
          w::FootnoteChoice::WP(paragraph) => Some(paragraph.as_ref()),
          _ => None,
        }),
      styles,
      &mut numbering,
      &images,
    );
  }

  Ok(blocks)
}

fn endnote_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Result<Vec<Block>> {
  let Some(part) = main.endnotes_part(package) else {
    return Ok(Vec::new());
  };
  let images = ImageCatalog::load_from_endnotes(package, &part);
  let endnotes = part.root_element(package)?;
  let mut numbering = NumberingCatalog::default();
  let mut blocks = Vec::new();

  for endnote in &endnotes.w_endnote {
    if endnote.id < 1
      || !matches!(
        endnote.r#type,
        None | Some(w::FootnoteEndnoteValues::Normal)
      )
    {
      continue;
    }
    append_note_blocks(
      &mut blocks,
      format!("{} ", endnote.id),
      endnote
        .endnote_choice
        .iter()
        .filter_map(|choice| match choice {
          w::EndnoteChoice::WP(paragraph) => Some(paragraph.as_ref()),
          _ => None,
        }),
      styles,
      &mut numbering,
      &images,
    );
  }

  Ok(blocks)
}

fn comment_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Result<Vec<Block>> {
  let Some(part) = main.wordprocessing_comments_part(package) else {
    return Ok(Vec::new());
  };
  let images = ImageCatalog::load_from_comments(package, &part);
  let comments = part.root_element(package)?;
  let mut numbering = NumberingCatalog::default();
  let mut blocks = Vec::new();

  for comment in &comments.w_comment {
    append_note_blocks(
      &mut blocks,
      format!("[{}] ", comment.id),
      comment
        .comment_choice
        .iter()
        .filter_map(|choice| match choice {
          w::CommentChoice::WP(paragraph) => Some(paragraph.as_ref()),
          _ => None,
        }),
      styles,
      &mut numbering,
      &images,
    );
  }

  Ok(blocks)
}

fn append_note_blocks<'a>(
  blocks: &mut Vec<Block>,
  label: impl Into<String>,
  paragraphs: impl Iterator<Item = &'a w::Paragraph>,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
) {
  let mut is_first_paragraph = true;
  let label = label.into();
  for paragraph in paragraphs {
    let mut model = paragraph_model(paragraph, styles, numbering, images);
    if is_first_paragraph {
      model.list_label = Some(label.clone());
      is_first_paragraph = false;
    }
    blocks.push(Block::Paragraph(model));
  }
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

  let inlines = paragraph_inlines(paragraph, styles.run_style(style_id), styles, images);
  #[cfg(test)]
  let runs = inlines
    .iter()
    .filter_map(|item| match item {
      InlineItem::Text(run) => Some(run.clone()),
      InlineItem::Image(_) => None,
      InlineItem::PageBreak => None,
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
  let (height_pt, exact_height) = table_row_height_properties(row.table_row_properties.as_deref());
  TableRow {
    height_pt,
    exact_height,
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
    grid_span: properties
      .and_then(|properties| properties.grid_span.as_ref())
      .map(|span| span.val.max(1) as usize)
      .unwrap_or(1),
    vertical_merge_continue: properties
      .and_then(|properties| properties.vertical_merge.as_ref())
      .map(|merge| matches!(merge.val, None | Some(w::MergedCellValues::Continue)))
      .unwrap_or(false),
    vertical_alignment: properties
      .and_then(|properties| properties.table_cell_vertical_alignment.as_ref())
      .map(|alignment| match alignment.val {
        w::TableVerticalAlignmentValues::Center => TableCellVerticalAlignment::Center,
        w::TableVerticalAlignmentValues::Bottom => TableCellVerticalAlignment::Bottom,
        w::TableVerticalAlignmentValues::Top => TableCellVerticalAlignment::Top,
      })
      .unwrap_or_default(),
  }
}

fn table_row_height_properties(properties: Option<&w::TableRowProperties>) -> (Option<f32>, bool) {
  let Some(properties) = properties else {
    return (None, false);
  };
  properties
    .table_row_properties_choice1
    .iter()
    .find_map(|choice| match choice {
      w::TableRowPropertiesChoice::WTrHeight(height) => {
        let height_pt = height.val.map(|value| units::twips_to_points(value as f32));
        Some((
          height_pt,
          matches!(height.height_type, Some(w::HeightRuleValues::Exact)),
        ))
      }
      _ => None,
    })
    .unwrap_or((None, false))
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

  if let Some(tabs) = properties.tabs() {
    format.tab_stops = tab_stops(tabs);
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

fn tab_stops(tabs: &w::Tabs) -> Vec<TabStop> {
  let mut stops = tabs
    .w_tab
    .iter()
    .filter_map(|tab| {
      let alignment = match tab.val {
        w::TabStopValues::Left | w::TabStopValues::Start | w::TabStopValues::Decimal => {
          TabStopAlignment::Left
        }
        w::TabStopValues::Center => TabStopAlignment::Center,
        w::TabStopValues::Right | w::TabStopValues::End | w::TabStopValues::Number => {
          TabStopAlignment::Right
        }
        w::TabStopValues::Clear | w::TabStopValues::Bar => return None,
      };
      Some(TabStop {
        position_pt: tab.position as f32 / 20.0,
        alignment,
      })
    })
    .filter(|stop| stop.position_pt.is_finite() && stop.position_pt >= 0.0)
    .collect::<Vec<_>>();
  stops.sort_by(|a, b| a.position_pt.total_cmp(&b.position_pt));
  stops.dedup_by(|a, b| (a.position_pt - b.position_pt).abs() < 0.1);
  stops
}

fn paragraph_inlines(
  paragraph: &w::Paragraph,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) -> Vec<InlineItem> {
  let mut inlines = Vec::new();

  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WR(run) => push_run(run, &mut inlines, base_style, styles, images),
      w::ParagraphChoice::WFldSimple(field) => {
        push_simple_field(field, &mut inlines, base_style, styles, images);
      }
      w::ParagraphChoice::WHyperlink(hyperlink) => {
        for item in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = item {
            push_run(run, &mut inlines, base_style, styles, images);
          }
        }
      }
      w::ParagraphChoice::Choice(choice) => match choice.as_ref() {
        w::ParagraphChoice2::WIns(inserted) => {
          push_inserted_run(inserted, &mut inlines, base_style, styles, images);
        }
        w::ParagraphChoice2::WDel(_)
        | w::ParagraphChoice2::WMoveFrom(_)
        | w::ParagraphChoice2::WMoveTo(_) => {}
        _ => {}
      },
      w::ParagraphChoice::WSdt(sdt) => push_sdt_run(sdt, &mut inlines, base_style, styles, images),
      _ => {}
    }
  }

  inlines
}

fn push_simple_field(
  field: &w::SimpleField,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  for choice in &field.simple_field_choice {
    match choice {
      w::SimpleFieldChoice::WR(run) => push_run(run, inlines, base_style, styles, images),
      w::SimpleFieldChoice::WHyperlink(hyperlink) => {
        for item in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = item {
            push_run(run, inlines, base_style, styles, images);
          }
        }
      }
      w::SimpleFieldChoice::WFldSimple(field) => {
        push_simple_field(field, inlines, base_style, styles, images);
      }
      w::SimpleFieldChoice::WSdt(sdt) => push_sdt_run(sdt, inlines, base_style, styles, images),
      _ => {}
    }
  }
}

fn push_run(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  let style = run_style(run.run_properties.as_deref(), base_style, styles);
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
        Some(w::BreakValues::Page) | Some(w::BreakValues::Column) => {
          flush_run_text(inlines, &mut text, style);
          inlines.push(InlineItem::PageBreak);
        }
        Some(w::BreakValues::TextWrapping) | None => text.push('\n'),
      },
      w::RunChoice::WSym(symbol) => {
        if let Some(symbol) = symbol_text(symbol) {
          text.push(symbol);
        }
      }
      w::RunChoice::WNoBreakHyphen => text.push('\u{2011}'),
      w::RunChoice::WSoftHyphen => text.push('\u{00ad}'),
      w::RunChoice::WFootnoteReference(reference) => {
        flush_run_text(inlines, &mut text, style);
        push_note_reference(inlines, reference.id, style);
      }
      w::RunChoice::WEndnoteReference(reference) => {
        flush_run_text(inlines, &mut text, style);
        push_note_reference(inlines, reference.id, style);
      }
      w::RunChoice::WCommentReference(reference) => {
        flush_run_text(inlines, &mut text, style);
        push_comment_reference(inlines, &reference.id, style);
      }
      w::RunChoice::WDrawing(drawing) => {
        flush_run_text(inlines, &mut text, style);
        if let Some(image) = inline_image(drawing, images) {
          inlines.push(InlineItem::Image(image));
        }
        push_drawing_textboxes(drawing, inlines, style);
      }
      w::RunChoice::WPict(picture) => {
        flush_run_text(inlines, &mut text, style);
        if let Some(image) = pict_image(picture, images) {
          inlines.push(InlineItem::Image(image));
        }
        push_pict_textboxes(picture, inlines, base_style, styles, images);
      }
      w::RunChoice::WPtab(_) => text.push('\t'),
      w::RunChoice::WRuby(ruby) => {
        flush_run_text(inlines, &mut text, style);
        push_ruby_base(ruby, inlines, base_style, styles, images);
      }
      _ => {}
    }
  }

  flush_run_text(inlines, &mut text, style);
}

fn push_ruby_base(
  ruby: &w::Ruby,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  for choice in &ruby.ruby_base.ruby_base_choice {
    match choice {
      w::RubyBaseChoice::WR(run) => push_run(run, inlines, base_style, styles, images),
      w::RubyBaseChoice::WIns(inserted) => {
        push_inserted_run(inserted, inlines, base_style, styles, images);
      }
      _ => {}
    }
  }
}

fn push_sdt_run(
  sdt: &w::SdtRun,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  let Some(content) = sdt.sdt_content_run.as_ref() else {
    return;
  };

  for choice in &content.sdt_content_run_choice {
    match choice {
      w::SdtContentRunChoice::WR(run) => {
        push_run(run.as_ref(), inlines, base_style, styles, images)
      }
      w::SdtContentRunChoice::WFldSimple(field) => {
        push_simple_field(field.as_ref(), inlines, base_style, styles, images);
      }
      w::SdtContentRunChoice::WHyperlink(hyperlink) => {
        for item in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = item {
            push_run(run, inlines, base_style, styles, images);
          }
        }
      }
      w::SdtContentRunChoice::WSdt(sdt) => {
        push_sdt_run(sdt.as_ref(), inlines, base_style, styles, images)
      }
      w::SdtContentRunChoice::WIns(inserted) => {
        push_inserted_run(inserted.as_ref(), inlines, base_style, styles, images);
      }
      w::SdtContentRunChoice::WDel(_)
      | w::SdtContentRunChoice::WMoveFrom(_)
      | w::SdtContentRunChoice::WMoveTo(_) => {}
      _ => {}
    }
  }
}

fn push_inserted_run(
  inserted: &w::InsertedRun,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  for choice in &inserted.inserted_run_choice {
    match choice {
      w::InsertedRunChoice::WR(run) => push_run(run, inlines, base_style, styles, images),
      w::InsertedRunChoice::Choice(choice) => match choice.as_ref() {
        w::InsertedRunChoice2::WIns(nested) => {
          push_inserted_run(nested, inlines, base_style, styles, images);
        }
        w::InsertedRunChoice2::WDel(_)
        | w::InsertedRunChoice2::WMoveFrom(_)
        | w::InsertedRunChoice2::WMoveTo(_) => {}
        _ => {}
      },
      _ => {}
    }
  }
}

fn push_note_reference(inlines: &mut Vec<InlineItem>, id: i64, style: TextStyle) {
  if id < 1 {
    return;
  }
  inlines.push(InlineItem::Text(TextRun {
    text: id.to_string(),
    style: TextStyle {
      font_size_pt: (style.font_size_pt * 0.75).max(1.0),
      ..style
    },
  }));
}

fn push_comment_reference(inlines: &mut Vec<InlineItem>, id: &str, style: TextStyle) {
  inlines.push(InlineItem::Text(TextRun {
    text: format!("[{id}]"),
    style: TextStyle {
      font_size_pt: (style.font_size_pt * 0.75).max(1.0),
      color: RgbColor {
        r: 0x80,
        g: 0x40,
        b: 0x00,
      },
      ..style
    },
  }));
}

fn flush_run_text(inlines: &mut Vec<InlineItem>, text: &mut String, style: TextStyle) {
  if !text.is_empty() {
    inlines.push(InlineItem::Text(TextRun {
      text: run_display_text(std::mem::take(text), style),
      style,
    }));
  }
}

fn run_display_text(text: String, style: TextStyle) -> String {
  if style.uppercase {
    text.to_uppercase()
  } else {
    text
  }
}

fn symbol_text(symbol: &w::SymbolChar) -> Option<char> {
  let code = u32::from_str_radix(symbol.char.as_deref()?, 16).ok()?;
  let low_byte = code & 0xFF;
  let font = symbol.font.as_deref().unwrap_or("").to_ascii_lowercase();

  if font.contains("wingdings") {
    return wingdings_symbol(low_byte).or_else(|| char::from_u32(code));
  }
  if font == "symbol" || font.ends_with(" symbol") {
    return symbol_font_symbol(low_byte).or_else(|| char::from_u32(code));
  }

  char::from_u32(code).or_else(|| {
    if (0xF000..=0xF0FF).contains(&code) {
      char::from_u32(low_byte)
    } else {
      None
    }
  })
}

fn symbol_font_symbol(code: u32) -> Option<char> {
  Some(match code {
    0x41 => 'Α',
    0x42 => 'Β',
    0x43 => 'Χ',
    0x44 => 'Δ',
    0x45 => 'Ε',
    0x46 => 'Φ',
    0x47 => 'Γ',
    0x48 => 'Η',
    0x49 => 'Ι',
    0x4A => 'ϑ',
    0x4B => 'Κ',
    0x4C => 'Λ',
    0x4D => 'Μ',
    0x4E => 'Ν',
    0x4F => 'Ο',
    0x50 => 'Π',
    0x51 => 'Θ',
    0x52 => 'Ρ',
    0x53 => 'Σ',
    0x54 => 'Τ',
    0x55 => 'Υ',
    0x56 => 'ς',
    0x57 => 'Ω',
    0x58 => 'Ξ',
    0x59 => 'Ψ',
    0x5A => 'Ζ',
    0x61 => 'α',
    0x62 => 'β',
    0x63 => 'χ',
    0x64 => 'δ',
    0x65 => 'ε',
    0x66 => 'φ',
    0x67 => 'γ',
    0x68 => 'η',
    0x69 => 'ι',
    0x6A => 'ϕ',
    0x6B => 'κ',
    0x6C => 'λ',
    0x6D => 'μ',
    0x6E => 'ν',
    0x6F => 'ο',
    0x70 => 'π',
    0x71 => 'θ',
    0x72 => 'ρ',
    0x73 => 'σ',
    0x74 => 'τ',
    0x75 => 'υ',
    0x76 => 'ϖ',
    0x77 => 'ω',
    0x78 => 'ξ',
    0x79 => 'ψ',
    0x7A => 'ζ',
    0xA2 => '′',
    0xA3 => '≤',
    0xA5 => '∞',
    0xA7 => '♣',
    0xA8 => '♦',
    0xA9 => '♥',
    0xAA => '♠',
    0xB1 => '±',
    0xB4 => '×',
    0xB5 => '∝',
    0xB6 => '∂',
    0xB7 => '•',
    0xB8 => '÷',
    0xB9 => '≠',
    0xBA => '≡',
    0xBB => '≈',
    0xBC => '…',
    0xBD => '⏐',
    0xBE => '⎯',
    0xBF => '↵',
    0xC0 => 'ℵ',
    0xC1 => 'ℑ',
    0xC2 => 'ℜ',
    0xC3 => '℘',
    0xC4 => '⊗',
    0xC5 => '⊕',
    0xC6 => '∅',
    0xC7 => '∩',
    0xC8 => '∪',
    0xC9 => '⊃',
    0xCA => '⊇',
    0xCB => '⊄',
    0xCC => '⊂',
    0xCD => '⊆',
    0xCE => '∈',
    0xCF => '∉',
    0xD0 => '∠',
    0xD1 => '∇',
    0xD2 => '®',
    0xD3 => '©',
    0xD4 => '™',
    0xD5 => '∏',
    0xD6 => '√',
    0xD7 => '⋅',
    0xD8 => '¬',
    0xD9 => '∧',
    0xDA => '∨',
    0xDB => '⇔',
    0xDC => '⇐',
    0xDD => '⇑',
    0xDE => '⇒',
    0xDF => '⇓',
    0xE0 => '◊',
    0xE1 => '〈',
    0xE2 => '®',
    0xE3 => '©',
    0xE4 => '™',
    0xE5 => '∑',
    0xE6 => '⎛',
    0xE7 => '⎜',
    0xE8 => '⎝',
    0xE9 => '⎡',
    0xEA => '⎢',
    0xEB => '⎣',
    0xEC => '⎧',
    0xED => '⎨',
    0xEE => '⎩',
    0xEF => '⎪',
    0xF1 => '〉',
    0xF2 => '∫',
    0xF3 => '⌠',
    0xF4 => '⎮',
    0xF5 => '⌡',
    0xF6 => '⎞',
    0xF7 => '⎟',
    0xF8 => '⎠',
    0xF9 => '⎤',
    0xFA => '⎥',
    0xFB => '⎦',
    0xFC => '⎫',
    0xFD => '⎬',
    0xFE => '⎭',
    _ => return char::from_u32(code),
  })
}

fn wingdings_symbol(code: u32) -> Option<char> {
  Some(match code {
    0x4A => '☺',
    0x4C => '●',
    0x6C => '●',
    0x6D => '■',
    0x6E => '□',
    0x71 => '❑',
    0x72 => '❒',
    0x73 => '⬧',
    0x74 => '◆',
    0x75 => '❖',
    0x76 => '⬥',
    0x77 => '⌧',
    0x78 => '⌦',
    0x9F => '•',
    0xA8 => '◻',
    0xF0 => '➔',
    0xFC => '✓',
    0xFD => '☒',
    0xFE => '☑',
    _ => return None,
  })
}

fn inline_image(drawing: &w::Drawing, images: &ImageCatalog) -> Option<InlineImage> {
  match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::WpInline(inline) => {
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
    w::DrawingChoice::WpAnchor(anchor) => {
      let graphic = anchor.a_graphic.as_ref()?;
      let extent = anchor.extent.as_ref()?;
      let relationship_id = embedded_image_relationship_id(&graphic.graphic_data)?;
      let resource = images.by_relationship_id.get(&relationship_id)?;
      Some(InlineImage {
        data: resource.data.clone(),
        content_type: resource.content_type.clone(),
        width_pt: units::emu_to_points(extent.cx),
        height_pt: units::emu_to_points(extent.cy),
        alt_text: anchor
          .wp_doc_pr
          .as_ref()
          .and_then(|properties| properties.description.clone()),
      })
    }
  }
}

fn push_drawing_textboxes(drawing: &w::Drawing, inlines: &mut Vec<InlineItem>, style: TextStyle) {
  let Some(graphic_data) = drawing_graphic_data(drawing) else {
    return;
  };

  for child in &graphic_data.xml_children {
    if let Some(text) = drawing_textbox_text(child) {
      inlines.push(InlineItem::Text(TextRun { text, style }));
    }
  }
}

fn drawing_graphic_data(drawing: &w::Drawing) -> Option<&ooxmlsdk::schemas::a::GraphicData> {
  match drawing.drawing_choice.as_ref()? {
    w::DrawingChoice::WpInline(inline) => Some(&inline.graphic.graphic_data),
    w::DrawingChoice::WpAnchor(anchor) => Some(&anchor.a_graphic.as_ref()?.graphic_data),
  }
}

fn pict_image(picture: &w::Picture, images: &ImageCatalog) -> Option<InlineImage> {
  picture
    .picture_choice
    .iter()
    .find_map(|choice| picture_choice_image(choice, images))
}

fn push_pict_textboxes(
  picture: &w::Picture,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  for choice in &picture.picture_choice {
    push_picture_choice_textboxes(choice, inlines, base_style, styles, images);
  }
}

fn picture_choice_image(choice: &w::PictureChoice, images: &ImageCatalog) -> Option<InlineImage> {
  match choice {
    w::PictureChoice::VGroup(group) => group_image(group, images),
    w::PictureChoice::VImage(image) => image_file_image(image, images),
    w::PictureChoice::VRect(rectangle) => rectangle_image(rectangle, images),
    w::PictureChoice::VShape(shape) => shape_image(shape, images),
    _ => None,
  }
}

fn push_picture_choice_textboxes(
  choice: &w::PictureChoice,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  match choice {
    w::PictureChoice::VGroup(group) => {
      push_group_textboxes(group, inlines, base_style, styles, images);
    }
    w::PictureChoice::VImage(image) => {
      push_image_file_textboxes(image, inlines, base_style, styles, images);
    }
    w::PictureChoice::VRect(rectangle) => {
      push_rectangle_textboxes(rectangle, inlines, base_style, styles, images);
    }
    w::PictureChoice::VShape(shape) => {
      push_shape_textboxes(shape, inlines, base_style, styles, images);
    }
    _ => {}
  }
}

fn group_image(group: &v::Group, images: &ImageCatalog) -> Option<InlineImage> {
  group.group_choice.iter().find_map(|choice| match choice {
    v::GroupChoice::VGroup(group) => group_image(group, images),
    v::GroupChoice::VImage(image) => image_file_image(image, images),
    v::GroupChoice::VRect(rectangle) => rectangle_image(rectangle, images),
    v::GroupChoice::VShape(shape) => shape_image(shape, images),
    _ => None,
  })
}

fn push_group_textboxes(
  group: &v::Group,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  for choice in &group.group_choice {
    match choice {
      v::GroupChoice::VGroup(group) => {
        push_group_textboxes(group, inlines, base_style, styles, images);
      }
      v::GroupChoice::VImage(image) => {
        push_image_file_textboxes(image, inlines, base_style, styles, images);
      }
      v::GroupChoice::VRect(rectangle) => {
        push_rectangle_textboxes(rectangle, inlines, base_style, styles, images);
      }
      v::GroupChoice::VShape(shape) => {
        push_shape_textboxes(shape, inlines, base_style, styles, images);
      }
      _ => {}
    }
  }
}

fn image_file_image(image: &v::ImageFile, images: &ImageCatalog) -> Option<InlineImage> {
  image
    .image_file_choice
    .iter()
    .find_map(|choice| match choice {
      v::ImageFileChoice::VImagedata(data) => vml_image_data(
        data,
        image.style.as_deref(),
        image.alternate.clone(),
        images,
      ),
      _ => None,
    })
}

fn push_image_file_textboxes(
  image: &v::ImageFile,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  for choice in &image.image_file_choice {
    if let v::ImageFileChoice::VTextbox(textbox) = choice {
      push_vml_textbox(textbox, inlines, base_style, styles, images);
    }
  }
}

fn rectangle_image(rectangle: &v::Rectangle, images: &ImageCatalog) -> Option<InlineImage> {
  rectangle
    .rectangle_choice
    .iter()
    .find_map(|choice| match choice {
      v::RectangleChoice::VImagedata(data) => vml_image_data(
        data,
        rectangle.style.as_deref(),
        rectangle.alternate.clone(),
        images,
      ),
      _ => None,
    })
}

fn push_rectangle_textboxes(
  rectangle: &v::Rectangle,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  for choice in &rectangle.rectangle_choice {
    if let v::RectangleChoice::VTextbox(textbox) = choice {
      push_vml_textbox(textbox, inlines, base_style, styles, images);
    }
  }
}

fn shape_image(shape: &v::Shape, images: &ImageCatalog) -> Option<InlineImage> {
  shape.shape_choice.iter().find_map(|choice| match choice {
    v::ShapeChoice::VImagedata(data) => vml_image_data(
      data,
      shape.style.as_deref(),
      shape.alternate.clone(),
      images,
    ),
    _ => None,
  })
}

fn push_shape_textboxes(
  shape: &v::Shape,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  for choice in &shape.shape_choice {
    if let v::ShapeChoice::VTextbox(textbox) = choice {
      push_vml_textbox(textbox, inlines, base_style, styles, images);
    }
  }
}

fn push_vml_textbox(
  textbox: &v::TextBox,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  let Some(v::TextBoxChoice::WTxbxContent(content)) = textbox.text_box_choice.as_ref() else {
    return;
  };
  push_textbox_content(content, inlines, base_style, styles, images);
}

fn push_textbox_content(
  content: &w::TextBoxContent,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
) {
  let mut numbering = NumberingCatalog::default();
  for choice in &content.text_box_content_choice {
    match choice {
      w::TextBoxContentChoice::WP(paragraph) => {
        let paragraph = paragraph_model(paragraph, styles, &mut numbering, images);
        inlines.extend(paragraph.inlines);
        inlines.push(InlineItem::Text(TextRun {
          text: "\n".into(),
          style: base_style,
        }));
      }
      w::TextBoxContentChoice::WTbl(table) => {
        let table = table_model(table, styles, &mut numbering, images);
        push_table_text(&table, inlines, base_style);
      }
      _ => {}
    }
  }
}

fn push_table_text(table: &Table, inlines: &mut Vec<InlineItem>, style: TextStyle) {
  for row in &table.rows {
    for (index, cell) in row.cells.iter().enumerate() {
      if index > 0 {
        inlines.push(InlineItem::Text(TextRun {
          text: "\t".into(),
          style,
        }));
      }
      for block in &cell.blocks {
        match block {
          Block::Paragraph(paragraph) => {
            inlines.extend(paragraph.inlines.clone());
          }
          Block::Table(table) => push_table_text(table, inlines, style),
        }
      }
    }
    inlines.push(InlineItem::Text(TextRun {
      text: "\n".into(),
      style,
    }));
  }
}

fn vml_image_data(
  data: &v::ImageData,
  style: Option<&str>,
  alt_text: Option<String>,
  images: &ImageCatalog,
) -> Option<InlineImage> {
  let relationship_id = data.relationship_id.as_ref().or(data.rel_id.as_ref())?;
  let resource = images.by_relationship_id.get(relationship_id)?;
  let (width_pt, height_pt) = vml_style_size(style).unwrap_or((72.0, 72.0));

  Some(InlineImage {
    data: resource.data.clone(),
    content_type: resource.content_type.clone(),
    width_pt,
    height_pt,
    alt_text: alt_text.or_else(|| data.title.clone()),
  })
}

fn vml_style_size(style: Option<&str>) -> Option<(f32, f32)> {
  let mut width = None;
  let mut height = None;

  for declaration in style?.split(';') {
    let Some((name, value)) = declaration.split_once(':') else {
      continue;
    };
    match name.trim().to_ascii_lowercase().as_str() {
      "width" => width = vml_measure_to_points(value),
      "height" => height = vml_measure_to_points(value),
      _ => {}
    }
  }

  Some((width?, height?))
}

fn vml_measure_to_points(value: &str) -> Option<f32> {
  let value = value.trim();
  if value.is_empty() {
    return None;
  }

  let (number, multiplier) = if let Some(number) = value.strip_suffix("pt") {
    (number, 1.0)
  } else if let Some(number) = value.strip_suffix("in") {
    (number, 72.0)
  } else if let Some(number) = value.strip_suffix("cm") {
    (number, 72.0 / 2.54)
  } else if let Some(number) = value.strip_suffix("mm") {
    (number, 72.0 / 25.4)
  } else if let Some(number) = value.strip_suffix("px") {
    (number, 0.75)
  } else {
    (value, 1.0)
  };

  number
    .trim()
    .parse::<f32>()
    .ok()
    .map(|points| points * multiplier)
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

fn drawing_textbox_text(xml: &str) -> Option<String> {
  if !xml.contains("txbxContent") {
    return None;
  }

  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(true);
  let mut textbox_depth = 0usize;
  let mut paragraph_depth = 0usize;
  let mut in_text = false;
  let mut output = String::new();

  loop {
    match reader.read_event().ok()? {
      Event::Start(event) => {
        if qname_ends_with(event.name().as_ref(), b"txbxContent") {
          textbox_depth += 1;
        } else if textbox_depth > 0 && qname_ends_with(event.name().as_ref(), b"p") {
          paragraph_depth += 1;
        } else if textbox_depth > 0 && qname_ends_with(event.name().as_ref(), b"t") {
          in_text = true;
        }
      }
      Event::End(event) => {
        if qname_ends_with(event.name().as_ref(), b"t") {
          in_text = false;
        } else if textbox_depth > 0 && qname_ends_with(event.name().as_ref(), b"p") {
          paragraph_depth = paragraph_depth.saturating_sub(1);
          output.push('\n');
        } else if qname_ends_with(event.name().as_ref(), b"txbxContent") {
          textbox_depth = textbox_depth.saturating_sub(1);
        }
      }
      Event::Text(event) if textbox_depth > 0 && in_text => {
        output.push_str(event.xml10_content().ok()?.as_ref());
      }
      Event::CData(event) if textbox_depth > 0 && in_text => {
        output.push_str(event.xml10_content().ok()?.as_ref());
      }
      Event::Eof => break,
      _ => {}
    }
  }

  if paragraph_depth > 0 {
    output.push('\n');
  }
  (!output.is_empty()).then_some(output)
}

fn qname_ends_with(qname: &[u8], local_name: &[u8]) -> bool {
  qname == local_name
    || qname
      .strip_suffix(local_name)
      .is_some_and(|prefix| prefix.ends_with(b":"))
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
    Self::from_image_parts(package, main.image_parts(package), |image_part| {
      main.get_id_of_part(package, image_part)
    })
  }

  fn load_from_header(package: &WordprocessingDocument, header: &HeaderPart) -> Self {
    Self::from_image_parts(package, header.image_parts(package), |image_part| {
      header.get_id_of_part(package, image_part)
    })
  }

  fn load_from_footer(package: &WordprocessingDocument, footer: &FooterPart) -> Self {
    Self::from_image_parts(package, footer.image_parts(package), |image_part| {
      footer.get_id_of_part(package, image_part)
    })
  }

  fn load_from_footnotes(package: &WordprocessingDocument, footnotes: &FootnotesPart) -> Self {
    Self::from_image_parts(package, footnotes.image_parts(package), |image_part| {
      footnotes.get_id_of_part(package, image_part)
    })
  }

  fn load_from_endnotes(package: &WordprocessingDocument, endnotes: &EndnotesPart) -> Self {
    Self::from_image_parts(package, endnotes.image_parts(package), |image_part| {
      endnotes.get_id_of_part(package, image_part)
    })
  }

  fn load_from_comments(
    package: &WordprocessingDocument,
    comments: &WordprocessingCommentsPart,
  ) -> Self {
    Self::from_image_parts(package, comments.image_parts(package), |image_part| {
      comments.get_id_of_part(package, image_part)
    })
  }

  fn from_image_parts<'a>(
    package: &WordprocessingDocument,
    image_parts: impl Iterator<Item = ImagePart> + 'a,
    relationship_id: impl Fn(&ImagePart) -> Option<&'a str>,
  ) -> Self {
    let mut by_relationship_id = HashMap::new();
    for image_part in image_parts {
      let Some(relationship_id) = relationship_id(&image_part) else {
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

fn run_style(
  properties: Option<&w::RunProperties>,
  base_style: TextStyle,
  styles: &StylesCatalog,
) -> TextStyle {
  let mut style = base_style;
  let Some(properties) = properties else {
    return style;
  };

  style = styles.character_run_style(
    properties
      .run_style
      .as_ref()
      .map(|run_style| run_style.val.as_str()),
    style,
  );
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
  if let Some(underline) = properties.underline() {
    style.underline = !matches!(underline.val, Some(w::UnderlineValues::None));
  }
  if let Some(strike) = properties.strike() {
    style.strikethrough = strike.val.unwrap_or(true);
  }
  if let Some(double_strike) = properties.double_strike() {
    style.strikethrough = double_strike.val.unwrap_or(true);
  }
  if let Some(caps) = properties.caps() {
    style.uppercase = caps.val.unwrap_or(true);
  }
  if let Some(small_caps) = properties.small_caps()
    && small_caps.val.unwrap_or(true)
  {
    style.uppercase = true;
    style.font_size_pt = (style.font_size_pt * 0.85).max(1.0);
  }
  if let Some(vertical_alignment) = properties.vertical_text_alignment() {
    match vertical_alignment.val {
      w::VerticalPositionValues::Superscript => {
        style.baseline_shift_pt = style.font_size_pt * 0.35;
        style.font_size_pt = (style.font_size_pt * 0.75).max(1.0);
      }
      w::VerticalPositionValues::Subscript => {
        style.baseline_shift_pt = -(style.font_size_pt * 0.2);
        style.font_size_pt = (style.font_size_pt * 0.75).max(1.0);
      }
      w::VerticalPositionValues::Baseline => {
        style.baseline_shift_pt = 0.0;
      }
    }
  }
  if let Some(highlight) = properties.highlight() {
    style.highlight = highlight_color(highlight.val);
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
  style_type: Option<w::StyleValues>,
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
        style_type: style.r#type,
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
    let mut format = self.doc_default_paragraph.clone();
    let style_id = style_id
      .map(str::to_string)
      .or_else(|| self.default_paragraph_style_id.clone());
    for entry in self.style_chain(style_id.as_deref()) {
      merge_format_values(&mut format, entry.paragraph_format.clone());
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

  fn character_run_style(&self, style_id: Option<&str>, base_style: TextStyle) -> TextStyle {
    let Some(style_id) = style_id else {
      return base_style;
    };
    let mut style = base_style;
    let mut matched = false;
    for entry in self.style_chain(Some(style_id)) {
      if matches!(entry.style_type, Some(w::StyleValues::Character)) {
        matched = true;
        merge_style_values(&mut style, entry.run_style);
      }
    }
    if !matched {
      merge_builtin_character_style(&mut style, style_id);
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

fn merge_builtin_character_style(style: &mut TextStyle, style_id: &str) {
  if style_id.eq_ignore_ascii_case("Hyperlink") {
    style.underline = true;
    style.color = RgbColor {
      r: 0x05,
      g: 0x63,
      b: 0xC1,
    };
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
  if !values.tab_stops.is_empty() {
    target.tab_stops = values.tab_stops;
  }
  if values.alignment != ParagraphAlignment::default() {
    target.alignment = values.alignment;
  }
  if values.shading.is_some() {
    target.shading = values.shading;
  }
  if values.borders != CellBordersModel::default() {
    target.borders = values.borders;
  }
  if values.page_break_before {
    target.page_break_before = true;
  }
}

fn merge_style_values(target: &mut TextStyle, values: TextStyle) {
  if (values.font_size_pt - TextStyle::default().font_size_pt).abs() > f32::EPSILON {
    target.font_size_pt = values.font_size_pt;
  }
  if values.baseline_shift_pt.abs() > f32::EPSILON {
    target.baseline_shift_pt = values.baseline_shift_pt;
  }
  if values.bold {
    target.bold = true;
  }
  if values.italic {
    target.italic = true;
  }
  if values.underline {
    target.underline = true;
  }
  if values.strikethrough {
    target.strikethrough = true;
  }
  if values.uppercase {
    target.uppercase = true;
  }
  if values.color != TextStyle::default().color {
    target.color = values.color;
  }
  if values.highlight.is_some() {
    target.highlight = values.highlight;
  }
}

fn highlight_color(value: w::HighlightColorValues) -> Option<RgbColor> {
  Some(match value {
    w::HighlightColorValues::Black => RgbColor { r: 0, g: 0, b: 0 },
    w::HighlightColorValues::Blue => RgbColor { r: 0, g: 0, b: 255 },
    w::HighlightColorValues::Cyan => RgbColor {
      r: 0,
      g: 255,
      b: 255,
    },
    w::HighlightColorValues::Green => RgbColor { r: 0, g: 255, b: 0 },
    w::HighlightColorValues::Magenta => RgbColor {
      r: 255,
      g: 0,
      b: 255,
    },
    w::HighlightColorValues::Red => RgbColor { r: 255, g: 0, b: 0 },
    w::HighlightColorValues::Yellow => RgbColor {
      r: 255,
      g: 255,
      b: 0,
    },
    w::HighlightColorValues::White => RgbColor {
      r: 255,
      g: 255,
      b: 255,
    },
    w::HighlightColorValues::DarkBlue => RgbColor { r: 0, g: 0, b: 128 },
    w::HighlightColorValues::DarkCyan => RgbColor {
      r: 0,
      g: 128,
      b: 128,
    },
    w::HighlightColorValues::DarkGreen => RgbColor { r: 0, g: 128, b: 0 },
    w::HighlightColorValues::DarkMagenta => RgbColor {
      r: 128,
      g: 0,
      b: 128,
    },
    w::HighlightColorValues::DarkRed => RgbColor { r: 128, g: 0, b: 0 },
    w::HighlightColorValues::DarkYellow => RgbColor {
      r: 128,
      g: 128,
      b: 0,
    },
    w::HighlightColorValues::DarkGray => RgbColor {
      r: 128,
      g: 128,
      b: 128,
    },
    w::HighlightColorValues::LightGray => RgbColor {
      r: 192,
      g: 192,
      b: 192,
    },
    w::HighlightColorValues::None => return None,
  })
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

    merge_format_values(format, level.format_properties.clone());
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

  fn tabs(&self) -> Option<&'a w::Tabs> {
    match self {
      Self::Direct(properties) => properties.tabs.as_ref(),
      Self::Style(properties) => properties.tabs.as_ref(),
      Self::BaseStyle(properties) => properties.tabs.as_ref(),
      Self::Previous(properties) => properties.tabs.as_ref(),
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

  fn underline(&self) -> Option<&'a w::Underline> {
    match self {
      Self::Direct(properties) => properties.underline.as_ref(),
      Self::Style(properties) => properties.underline.as_ref(),
      Self::BaseStyle(properties) => properties.underline.as_ref(),
    }
  }

  fn strike(&self) -> Option<&'a w::Strike> {
    match self {
      Self::Direct(properties) => properties.strike.as_ref(),
      Self::Style(properties) => properties.strike.as_ref(),
      Self::BaseStyle(properties) => properties.strike.as_ref(),
    }
  }

  fn double_strike(&self) -> Option<&'a w::DoubleStrike> {
    match self {
      Self::Direct(properties) => properties.double_strike.as_ref(),
      Self::Style(properties) => properties.double_strike.as_ref(),
      Self::BaseStyle(properties) => properties.double_strike.as_ref(),
    }
  }

  fn caps(&self) -> Option<&'a w::Caps> {
    match self {
      Self::Direct(properties) => properties.caps.as_ref(),
      Self::Style(properties) => properties.caps.as_ref(),
      Self::BaseStyle(properties) => properties.caps.as_ref(),
    }
  }

  fn small_caps(&self) -> Option<&'a w::SmallCaps> {
    match self {
      Self::Direct(properties) => properties.small_caps.as_ref(),
      Self::Style(properties) => properties.small_caps.as_ref(),
      Self::BaseStyle(properties) => properties.small_caps.as_ref(),
    }
  }

  fn vertical_text_alignment(&self) -> Option<&'a w::VerticalTextAlignment> {
    match self {
      Self::Direct(properties) => properties.vertical_text_alignment.as_ref(),
      Self::Style(properties) => properties.vertical_text_alignment.as_ref(),
      Self::BaseStyle(properties) => properties.vertical_text_alignment.as_ref(),
    }
  }

  fn highlight(&self) -> Option<&'a w::Highlight> {
    match self {
      Self::Direct(properties) => properties.highlight.as_ref(),
      Self::Style(_) | Self::BaseStyle(_) => None,
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn symbol_runs_emit_unicode_text() {
    let mut inlines = Vec::new();
    let run = w::Run {
      run_choice: vec![
        w::RunChoice::WSym(Box::new(w::SymbolChar {
          font: Some("Symbol".into()),
          char: Some("F0B7".into()),
        })),
        w::RunChoice::WSym(Box::new(w::SymbolChar {
          font: Some("Wingdings".into()),
          char: Some("F0FC".into()),
        })),
        w::RunChoice::WSym(Box::new(w::SymbolChar {
          font: None,
          char: Some("00A9".into()),
        })),
      ],
      ..Default::default()
    };

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
    );

    assert_eq!(inline_text(&inlines), "•✓©");
  }

  #[test]
  fn ruby_runs_emit_base_text() {
    let mut inlines = Vec::new();
    let ruby = w::Ruby {
      ruby_base: Box::new(w::RubyBase {
        ruby_base_choice: vec![w::RubyBaseChoice::WR(Box::new(w::Run {
          run_choice: vec![w::RunChoice::WT(Box::new(w::Text {
            xml_content: Some("漢".into()),
            ..Default::default()
          }))],
          ..Default::default()
        }))],
        ..Default::default()
      }),
      ..Default::default()
    };
    let run = w::Run {
      run_choice: vec![
        w::RunChoice::WT(Box::new(w::Text {
          xml_content: Some("Before ".into()),
          ..Default::default()
        })),
        w::RunChoice::WRuby(Box::new(ruby)),
        w::RunChoice::WT(Box::new(w::Text {
          xml_content: Some(" after".into()),
          ..Default::default()
        })),
      ],
      ..Default::default()
    };

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
    );

    assert_eq!(inline_text(&inlines), "Before 漢 after");
  }

  #[test]
  fn vml_pict_runs_emit_images() {
    let mut catalog = ImageCatalog::default();
    catalog.by_relationship_id.insert(
      "rId1".into(),
      ImageResource {
        data: vec![1, 2, 3],
        content_type: Some("image/png".into()),
      },
    );
    let run = w::Run {
      run_choice: vec![w::RunChoice::WPict(Box::new(w::Picture {
        picture_choice: vec![w::PictureChoice::VShape(Box::new(v::Shape {
          style: Some("width:1in;height:24pt".into()),
          alternate: Some("VML image".into()),
          shape_choice: vec![v::ShapeChoice::VImagedata(Box::new(v::ImageData {
            relationship_id: Some("rId1".into()),
            ..Default::default()
          }))],
          ..Default::default()
        }))],
        ..Default::default()
      }))],
      ..Default::default()
    };
    let mut inlines = Vec::new();

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &catalog,
    );

    let image = inlines
      .iter()
      .find_map(|item| match item {
        InlineItem::Image(image) => Some(image),
        InlineItem::Text(_) | InlineItem::PageBreak => None,
      })
      .expect("VML image");
    assert_eq!(image.content_type.as_deref(), Some("image/png"));
    assert_eq!(image.width_pt, 72.0);
    assert_eq!(image.height_pt, 24.0);
    assert_eq!(image.alt_text.as_deref(), Some("VML image"));
  }

  #[test]
  fn vml_textboxes_emit_text_content() {
    let run = w::Run {
      run_choice: vec![w::RunChoice::WPict(Box::new(w::Picture {
        picture_choice: vec![w::PictureChoice::VShape(Box::new(v::Shape {
          shape_choice: vec![v::ShapeChoice::VTextbox(Box::new(v::TextBox {
            text_box_choice: Some(v::TextBoxChoice::WTxbxContent(Box::new(
              w::TextBoxContent {
                text_box_content_choice: vec![w::TextBoxContentChoice::WP(Box::new(
                  w::Paragraph {
                    paragraph_choice: vec![w::ParagraphChoice::WR(Box::new(w::Run {
                      run_choice: vec![w::RunChoice::WT(Box::new(w::Text {
                        xml_content: Some("Text inside VML box".into()),
                        ..Default::default()
                      }))],
                      ..Default::default()
                    }))],
                    ..Default::default()
                  },
                ))],
                ..Default::default()
              },
            ))),
            ..Default::default()
          }))],
          ..Default::default()
        }))],
        ..Default::default()
      }))],
      ..Default::default()
    };
    let mut inlines = Vec::new();

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
    );

    assert!(inline_text(&inlines).contains("Text inside VML box"));
  }

  #[test]
  fn drawing_textboxes_extract_cached_text() {
    let xml = r#"<wps:wsp xmlns:wps="http://schemas.microsoft.com/office/word/2010/wordprocessingShape" xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <wps:txbx>
    <w:txbxContent>
      <w:p><w:r><w:t>Modern text box</w:t></w:r></w:p>
      <w:p><w:r><w:t>Second line</w:t></w:r></w:p>
    </w:txbxContent>
  </wps:txbx>
</wps:wsp>"#;

    assert_eq!(
      drawing_textbox_text(xml).as_deref(),
      Some("Modern text box\nSecond line\n")
    );
  }

  fn inline_text(inlines: &[InlineItem]) -> String {
    inlines
      .iter()
      .filter_map(|item| match item {
        InlineItem::Text(run) => Some(run.text.as_str()),
        InlineItem::Image(_) | InlineItem::PageBreak => None,
      })
      .collect()
  }
}
