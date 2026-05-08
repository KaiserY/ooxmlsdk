mod units;

use std::collections::{BTreeMap, HashMap};

use ooxmlsdk::common::RelationshipTargetKind;
use ooxmlsdk::parts::{
  endnotes_part::EndnotesPart, footer_part::FooterPart, footnotes_part::FootnotesPart,
  header_part::HeaderPart, image_part::ImagePart, main_document_part::MainDocumentPart,
  wordprocessing_comments_part::WordprocessingCommentsPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::{
  schemas_microsoft_com_vml as v,
  schemas_openxmlformats_org_drawingml_2006_wordprocessing_drawing as wp,
  schemas_openxmlformats_org_wordprocessingml_2006_main as w,
};
use ooxmlsdk::sdk::SdkPart;
use quick_xml::Reader;
use quick_xml::events::Event;

use crate::error::Result;
use crate::options::PdfOptions;

const DEFAULT_TAB_STOP_PT: f32 = 36.0;

#[derive(Clone, Debug)]
pub(crate) struct DocxDocument {
  pub page: PageSetup,
  pub default_tab_stop_pt: f32,
  pub even_and_odd_headers: bool,
  pub sections: Vec<ImportedSection>,
  pub header_blocks: Vec<Block>,
  pub footer_blocks: Vec<Block>,
  pub first_header_blocks: Vec<Block>,
  pub first_footer_blocks: Vec<Block>,
  pub footnote_blocks: Vec<Block>,
  pub footnotes: BTreeMap<i64, Vec<Block>>,
  pub endnote_blocks: Vec<Block>,
  pub endnotes: BTreeMap<i64, Vec<Block>>,
  pub comment_blocks: Vec<Block>,
  pub title_page: bool,
  pub blocks: Vec<Block>,
}

#[derive(Clone, Debug)]
pub(crate) struct ImportedSection {
  pub break_kind: SectionBreakKind,
  pub section_properties: Option<w::SectionProperties>,
  pub page: PageSetup,
  pub columns: SectionColumns,
  pub title_page: bool,
  pub header_blocks: Vec<Block>,
  pub footer_blocks: Vec<Block>,
  pub first_header_blocks: Vec<Block>,
  pub first_footer_blocks: Vec<Block>,
  pub even_header_blocks: Vec<Block>,
  pub even_footer_blocks: Vec<Block>,
  pub blocks: Vec<Block>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SectionColumns {
  pub count: usize,
  pub gap_pt: f32,
  pub separator: bool,
  pub explicit_count: usize,
  pub explicit_widths_pt: [f32; 45],
  pub explicit_gaps_pt: [f32; 44],
}

impl Default for SectionColumns {
  fn default() -> Self {
    Self {
      count: 1,
      gap_pt: 36.0,
      separator: false,
      explicit_count: 0,
      explicit_widths_pt: [0.0; 45],
      explicit_gaps_pt: [0.0; 44],
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SectionBreakKind {
  Continuous,
  NextPage,
  NextColumn,
  EvenPage,
  OddPage,
}

#[derive(Clone, Debug)]
pub(crate) enum Block {
  Paragraph(Paragraph),
  Table(Table),
}

#[derive(Clone, Debug)]
pub(crate) struct Paragraph {
  pub inlines: Vec<InlineItem>,
  pub footnote_reference_ids: Vec<i64>,
  pub endnote_reference_ids: Vec<i64>,
  #[cfg(test)]
  pub runs: Vec<TextRun>,
  pub format: ParagraphFormat,
  pub list_label: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
  pub column_widths_pt: Vec<f32>,
  pub preferred_width_pt: Option<f32>,
  pub preferred_width_pct: Option<f32>,
  pub indent_left_pt: f32,
  pub alignment: TableAlignment,
  pub borders: Option<TableBordersModel>,
  pub cell_margins: CellMargins,
  pub rows: Vec<TableRow>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TableAlignment {
  #[default]
  Left,
  Center,
  Right,
}

#[derive(Clone, Debug)]
pub(crate) struct TableRow {
  pub cells: Vec<TableCell>,
  pub height_pt: Option<f32>,
  pub exact_height: bool,
  pub repeat_header: bool,
  pub cant_split: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct TableCell {
  pub blocks: Vec<Block>,
  pub shading: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub margins: CellMargins,
  pub preferred_width_pt: Option<f32>,
  pub preferred_width_pct: Option<f32>,
  pub grid_span: usize,
  pub vertical_merge_continue: bool,
  pub vertical_alignment: TableCellVerticalAlignment,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CellMargins {
  pub top_pt: f32,
  pub right_pt: f32,
  pub bottom_pt: f32,
  pub left_pt: f32,
}

impl Default for CellMargins {
  fn default() -> Self {
    Self {
      top_pt: 4.0,
      right_pt: 4.0,
      bottom_pt: 4.0,
      left_pt: 4.0,
    }
  }
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
  pub spacing_pt: f32,
  pub color: RgbColor,
}

impl Default for BorderStyle {
  fn default() -> Self {
    Self {
      width_pt: 0.5,
      spacing_pt: 0.0,
      color: RgbColor { r: 0, g: 0, b: 0 },
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ParagraphFormat {
  pub spacing_before_pt: f32,
  pub spacing_after_pt: f32,
  pub line_height_pt: Option<f32>,
  pub line_height_rule: LineHeightRule,
  pub indent_left_pt: f32,
  pub indent_right_pt: f32,
  pub first_line_indent_pt: f32,
  pub tab_stops: Vec<TabStop>,
  pub alignment: ParagraphAlignment,
  pub shading: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub page_break_before: bool,
  pub keep_with_next: bool,
  pub keep_lines: bool,
  pub contextual_spacing: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum LineHeightRule {
  #[default]
  Auto,
  AtLeast,
  Exact,
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
  pub hyperlink_url: Option<String>,
  pub dynamic_field: Option<DynamicFieldKind>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum DynamicFieldKind {
  Page,
  NumPages,
}

#[derive(Clone, Debug)]
pub(crate) enum InlineItem {
  Text(TextRun),
  Image(InlineImage),
  PageBreak,
  ColumnBreak,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineImage {
  pub data: Vec<u8>,
  pub content_type: Option<String>,
  pub width_pt: f32,
  pub height_pt: f32,
  pub effect_left_pt: f32,
  pub effect_top_pt: f32,
  pub effect_right_pt: f32,
  pub effect_bottom_pt: f32,
  pub crop: ImageCrop,
  pub rotation_deg: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub alt_text: Option<String>,
  pub placement: ImagePlacement,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ImageCrop {
  pub left: f32,
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum ImagePlacement {
  #[default]
  Inline,
  Floating(FloatingImagePlacement),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FloatingImagePlacement {
  pub horizontal_relative_to: HorizontalImageReference,
  pub vertical_relative_to: VerticalImageReference,
  pub horizontal_alignment: Option<HorizontalImageAlignment>,
  pub vertical_alignment: Option<VerticalImageAlignment>,
  pub horizontal_offset_pt: f32,
  pub vertical_offset_pt: f32,
  pub wrap: ImageWrapMode,
  pub wrap_side: ImageWrapSide,
  pub behind_text: bool,
  pub margin_top_pt: f32,
  pub margin_right_pt: f32,
  pub margin_bottom_pt: f32,
  pub margin_left_pt: f32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ImageWrapSide {
  #[default]
  BothSides,
  Left,
  Right,
  Largest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum HorizontalImageAlignment {
  Left,
  Center,
  Right,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum VerticalImageAlignment {
  Top,
  Center,
  Bottom,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum HorizontalImageReference {
  Page,
  #[default]
  Margin,
  Column,
  Character,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum VerticalImageReference {
  Page,
  #[default]
  Margin,
  Paragraph,
  Line,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ImageWrapMode {
  #[default]
  Inline,
  Square,
  Tight,
  Through,
  TopBottom,
  None,
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
  pub background: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub borders_offset_from_text: bool,
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
      background: None,
      borders: CellBordersModel::default(),
      borders_offset_from_text: false,
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
  let hyperlinks = HyperlinkCatalog::load(package, &main);
  let default_tab_stop_pt = default_tab_stop_pt(package, &main);
  let even_and_odd_headers = even_and_odd_headers(package, &main);
  let document = main.root_element(package)?;
  let page_background = document
    .document_background
    .as_deref()
    .and_then(document_background_color);
  let mut sections = document
    .body
    .as_deref()
    .map(|body| body_sections(body, &styles, &mut numbering, &images, &hyperlinks))
    .unwrap_or_else(|| vec![default_section(Vec::new())]);
  for section in &mut sections {
    section.page.background = page_background;
  }
  resolve_section_repeating_blocks(package, &main, &styles, &mut sections);
  let page = sections
    .first()
    .map(|section| section.page)
    .unwrap_or_default();
  let blocks = sections
    .iter()
    .flat_map(|section| section.blocks.iter().cloned())
    .collect();
  let header_blocks = sections
    .first()
    .map(|section| section.header_blocks.clone())
    .unwrap_or_default();
  let footer_blocks = sections
    .first()
    .map(|section| section.footer_blocks.clone())
    .unwrap_or_default();
  let first_header_blocks = sections
    .first()
    .map(|section| section.first_header_blocks.clone())
    .unwrap_or_default();
  let first_footer_blocks = sections
    .first()
    .map(|section| section.first_footer_blocks.clone())
    .unwrap_or_default();
  let footnotes = footnotes(package, &main, &styles)?;
  let footnote_blocks = flatten_note_blocks(&footnotes);
  let endnotes = endnotes(package, &main, &styles)?;
  let endnote_blocks = flatten_note_blocks(&endnotes);
  let comment_blocks = comment_blocks(package, &main, &styles)?;
  let title_page = sections
    .first()
    .map(|section| section.title_page)
    .unwrap_or(false);

  Ok(DocxDocument {
    page,
    default_tab_stop_pt,
    even_and_odd_headers,
    sections,
    header_blocks,
    footer_blocks,
    first_header_blocks,
    first_footer_blocks,
    footnote_blocks,
    footnotes,
    endnote_blocks,
    endnotes,
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

fn even_and_odd_headers(package: &mut WordprocessingDocument, main: &MainDocumentPart) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .w_even_and_odd_headers
        .as_ref()
        .map(|setting| setting.val.unwrap_or(true))
    })
    .unwrap_or(false)
}

fn resolve_section_repeating_blocks(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
  sections: &mut [ImportedSection],
) {
  let mut previous_default_header = Vec::new();
  let mut previous_default_footer = Vec::new();
  let mut previous_first_header = Vec::new();
  let mut previous_first_footer = Vec::new();
  let mut previous_even_header = Vec::new();
  let mut previous_even_footer = Vec::new();

  for section in sections {
    let Some(section_properties) = section.section_properties.as_ref() else {
      section.header_blocks.clone_from(&previous_default_header);
      section.footer_blocks.clone_from(&previous_default_footer);
      section
        .first_header_blocks
        .clone_from(&previous_first_header);
      section
        .first_footer_blocks
        .clone_from(&previous_first_footer);
      section.even_header_blocks.clone_from(&previous_even_header);
      section.even_footer_blocks.clone_from(&previous_even_footer);
      continue;
    };

    section.header_blocks = referenced_header_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Default,
    )
    .unwrap_or_else(|| previous_default_header.clone());
    section.footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Default,
    )
    .unwrap_or_else(|| previous_default_footer.clone());
    section.first_header_blocks = referenced_header_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::First,
    )
    .unwrap_or_else(|| previous_first_header.clone());
    section.first_footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::First,
    )
    .unwrap_or_else(|| previous_first_footer.clone());
    section.even_header_blocks = referenced_header_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Even,
    )
    .unwrap_or_else(|| previous_even_header.clone());
    section.even_footer_blocks = referenced_footer_blocks(
      package,
      main,
      section_properties,
      styles,
      w::HeaderFooterValues::Even,
    )
    .unwrap_or_else(|| previous_even_footer.clone());

    previous_default_header.clone_from(&section.header_blocks);
    previous_default_footer.clone_from(&section.footer_blocks);
    previous_first_header.clone_from(&section.first_header_blocks);
    previous_first_footer.clone_from(&section.first_footer_blocks);
    previous_even_header.clone_from(&section.even_header_blocks);
    previous_even_footer.clone_from(&section.even_footer_blocks);
  }
}

fn body_sections(
  body: &w::Body,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) -> Vec<ImportedSection> {
  let mut sections = Vec::new();
  let mut current_blocks = Vec::new();
  let mut previous_properties = None;

  for choice in &body.body_choice {
    match choice {
      w::BodyChoice::WP(paragraph) => {
        current_blocks.push(Block::Paragraph(paragraph_model(
          paragraph, styles, numbering, images, hyperlinks,
        )));
        if let Some(section_properties) = paragraph
          .paragraph_properties
          .as_deref()
          .and_then(|properties| properties.section_properties.as_deref())
          .cloned()
        {
          close_section(
            &mut sections,
            &mut current_blocks,
            Some(section_properties),
            &mut previous_properties,
          );
        }
      }
      w::BodyChoice::WTbl(table) => current_blocks.push(Block::Table(table_model(
        table, styles, numbering, images, hyperlinks,
      ))),
      w::BodyChoice::WSdt(sdt) => {
        current_blocks.extend(sdt_block_blocks(sdt, styles, numbering, images, hyperlinks));
      }
      _ => {}
    }
  }

  if body.w_sect_pr.is_some() || sections.is_empty() || !current_blocks.is_empty() {
    close_section(
      &mut sections,
      &mut current_blocks,
      body.w_sect_pr.as_deref().cloned(),
      &mut previous_properties,
    );
  }

  sections
}

fn close_section(
  sections: &mut Vec<ImportedSection>,
  current_blocks: &mut Vec<Block>,
  section_properties: Option<w::SectionProperties>,
  previous_properties: &mut Option<w::SectionProperties>,
) {
  let break_kind =
    normalized_section_break(section_properties.as_ref(), previous_properties.as_ref());
  let page = section_properties
    .as_ref()
    .map(page_setup)
    .unwrap_or_default();
  let columns = section_properties
    .as_ref()
    .map(section_columns)
    .unwrap_or_default();
  let title_page = section_properties
    .as_ref()
    .and_then(|section| section.w_title_pg.as_ref())
    .map(|title_page| title_page.val.unwrap_or(true))
    .unwrap_or(false);

  sections.push(ImportedSection {
    break_kind,
    section_properties: section_properties.clone(),
    page,
    columns,
    title_page,
    header_blocks: Vec::new(),
    footer_blocks: Vec::new(),
    first_header_blocks: Vec::new(),
    first_footer_blocks: Vec::new(),
    even_header_blocks: Vec::new(),
    even_footer_blocks: Vec::new(),
    blocks: std::mem::take(current_blocks),
  });

  if let Some(section_properties) = section_properties {
    *previous_properties = Some(section_properties);
  }
}

fn default_section(blocks: Vec<Block>) -> ImportedSection {
  ImportedSection {
    break_kind: SectionBreakKind::NextPage,
    section_properties: None,
    page: PageSetup::default(),
    columns: SectionColumns::default(),
    title_page: false,
    header_blocks: Vec::new(),
    footer_blocks: Vec::new(),
    first_header_blocks: Vec::new(),
    first_footer_blocks: Vec::new(),
    even_header_blocks: Vec::new(),
    even_footer_blocks: Vec::new(),
    blocks,
  }
}

fn normalized_section_break(
  section: Option<&w::SectionProperties>,
  previous: Option<&w::SectionProperties>,
) -> SectionBreakKind {
  let Some(section) = section else {
    return SectionBreakKind::NextPage;
  };

  let kind = section
    .w_type
    .as_ref()
    .map(|section_type| match section_type.val {
      w::SectionMarkValues::Continuous => SectionBreakKind::Continuous,
      w::SectionMarkValues::NextColumn => SectionBreakKind::NextColumn,
      w::SectionMarkValues::EvenPage => SectionBreakKind::EvenPage,
      w::SectionMarkValues::OddPage => SectionBreakKind::OddPage,
      w::SectionMarkValues::NextPage => SectionBreakKind::NextPage,
    })
    .unwrap_or(SectionBreakKind::NextPage);

  match kind {
    SectionBreakKind::Continuous
      if previous
        .map(|previous| section_orientation(previous) != section_orientation(section))
        .unwrap_or(false) =>
    {
      SectionBreakKind::NextPage
    }
    SectionBreakKind::NextColumn
      if previous
        .map(|previous| {
          section_column_count(section) <= 1
            || section_column_count(previous) != section_column_count(section)
        })
        .unwrap_or(true) =>
    {
      SectionBreakKind::NextPage
    }
    _ => kind,
  }
}

fn section_orientation(section: &w::SectionProperties) -> w::PageOrientationValues {
  section
    .w_pg_sz
    .as_ref()
    .and_then(|size| size.orient)
    .or_else(|| {
      let size = section.w_pg_sz.as_ref()?;
      Some(if size.width.unwrap_or(0) > size.height.unwrap_or(0) {
        w::PageOrientationValues::Landscape
      } else {
        w::PageOrientationValues::Portrait
      })
    })
    .unwrap_or_default()
}

fn section_column_count(section: &w::SectionProperties) -> i16 {
  let Some(columns) = section.w_cols.as_ref() else {
    return 1;
  };
  if !columns.equal_width.unwrap_or(true) && !columns.w_col.is_empty() {
    return (columns.w_col.len() as i16).max(1);
  }
  columns.column_count.unwrap_or(1).max(1)
}

fn section_columns(section: &w::SectionProperties) -> SectionColumns {
  let Some(columns) = section.w_cols.as_ref() else {
    return SectionColumns::default();
  };
  let equal_width = columns.equal_width.unwrap_or(true);
  let gap_pt = columns
    .space
    .as_deref()
    .and_then(twips_attr_to_points)
    .filter(|gap| gap.is_finite() && *gap >= 0.0)
    .unwrap_or(36.0);
  if !equal_width && !columns.w_col.is_empty() {
    let explicit_widths_pt = columns
      .w_col
      .iter()
      .filter_map(|column| {
        column
          .width
          .as_deref()
          .and_then(twips_attr_to_points)
          .filter(|width| width.is_finite() && *width > 0.0)
      })
      .collect::<Vec<_>>();
    if explicit_widths_pt.len() == columns.w_col.len() {
      let explicit_gaps_pt = columns
        .w_col
        .iter()
        .take(columns.w_col.len().saturating_sub(1))
        .map(|column| {
          column
            .space
            .as_deref()
            .and_then(twips_attr_to_points)
            .filter(|gap| gap.is_finite() && *gap >= 0.0)
            .unwrap_or(gap_pt)
        })
        .collect::<Vec<_>>();
      let explicit_count = explicit_widths_pt.len().min(45);
      let mut widths = [0.0; 45];
      let mut gaps = [0.0; 44];
      for (index, width) in explicit_widths_pt.iter().copied().take(45).enumerate() {
        widths[index] = width;
      }
      for (index, gap) in explicit_gaps_pt.iter().copied().take(44).enumerate() {
        gaps[index] = gap;
      }
      return SectionColumns {
        count: explicit_count.max(1),
        gap_pt,
        separator: columns.separator.unwrap_or(false),
        explicit_count,
        explicit_widths_pt: widths,
        explicit_gaps_pt: gaps,
      };
    }
  }

  let count = columns
    .column_count
    .map(|count| count.max(1) as usize)
    .unwrap_or(1);
  SectionColumns {
    count,
    gap_pt,
    separator: columns.separator.unwrap_or(false),
    explicit_count: 0,
    explicit_widths_pt: [0.0; 45],
    explicit_gaps_pt: [0.0; 44],
  }
}

fn sdt_block_blocks(
  sdt: &w::SdtBlock,
  styles: &StylesCatalog,
  numbering: &mut NumberingCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
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
        hyperlinks,
      ))]),
      w::SdtContentBlockChoice::WTbl(table) => Some(vec![Block::Table(table_model(
        table.as_ref(),
        styles,
        numbering,
        images,
        hyperlinks,
      ))]),
      w::SdtContentBlockChoice::WSdt(sdt) => Some(sdt_block_blocks(
        sdt.as_ref(),
        styles,
        numbering,
        images,
        hyperlinks,
      )),
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
  let hyperlinks = HyperlinkCatalog::load(package, &header_part);
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
          &hyperlinks,
        ))),
        w::HeaderChoice::WTbl(table) => Some(Block::Table(table_model(
          table,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
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
  let hyperlinks = HyperlinkCatalog::load(package, &footer_part);
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
          &hyperlinks,
        ))),
        w::FooterChoice::WTbl(table) => Some(Block::Table(table_model(
          table,
          styles,
          &mut numbering,
          &images,
          &hyperlinks,
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

fn footnotes(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Result<BTreeMap<i64, Vec<Block>>> {
  let Some(part) = main.footnotes_part(package) else {
    return Ok(BTreeMap::new());
  };
  let images = ImageCatalog::load_from_footnotes(package, &part);
  let hyperlinks = HyperlinkCatalog::load(package, &part);
  let footnotes = part.root_element(package)?;
  let mut numbering = NumberingCatalog::default();
  let mut notes = BTreeMap::new();

  for footnote in &footnotes.w_footnote {
    if footnote.id < 1
      || !matches!(
        footnote.r#type,
        None | Some(w::FootnoteEndnoteValues::Normal)
      )
    {
      continue;
    }
    let mut blocks = Vec::new();
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
      &hyperlinks,
    );
    notes.insert(footnote.id, blocks);
  }

  Ok(notes)
}

fn endnotes(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
  styles: &StylesCatalog,
) -> Result<BTreeMap<i64, Vec<Block>>> {
  let Some(part) = main.endnotes_part(package) else {
    return Ok(BTreeMap::new());
  };
  let images = ImageCatalog::load_from_endnotes(package, &part);
  let hyperlinks = HyperlinkCatalog::load(package, &part);
  let endnotes = part.root_element(package)?;
  let mut numbering = NumberingCatalog::default();
  let mut notes = BTreeMap::new();

  for endnote in &endnotes.w_endnote {
    if endnote.id < 1
      || !matches!(
        endnote.r#type,
        None | Some(w::FootnoteEndnoteValues::Normal)
      )
    {
      continue;
    }
    let mut blocks = Vec::new();
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
      &hyperlinks,
    );
    notes.insert(endnote.id, blocks);
  }

  Ok(notes)
}

fn flatten_note_blocks(notes: &BTreeMap<i64, Vec<Block>>) -> Vec<Block> {
  notes
    .values()
    .flat_map(|blocks| blocks.iter().cloned())
    .collect()
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
  let hyperlinks = HyperlinkCatalog::load(package, &part);
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
      &hyperlinks,
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
  hyperlinks: &HyperlinkCatalog,
) {
  let mut is_first_paragraph = true;
  let label = label.into();
  for paragraph in paragraphs {
    let mut model = paragraph_model(paragraph, styles, numbering, images, hyperlinks);
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
  hyperlinks: &HyperlinkCatalog,
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

  let inlines = paragraph_inlines(
    paragraph,
    styles.run_style(style_id),
    styles,
    images,
    hyperlinks,
  );
  let (footnote_reference_ids, endnote_reference_ids) = paragraph_note_reference_ids(paragraph);
  #[cfg(test)]
  let runs = inlines
    .iter()
    .filter_map(|item| match item {
      InlineItem::Text(run) => Some(run.clone()),
      InlineItem::Image(_) => None,
      InlineItem::PageBreak | InlineItem::ColumnBreak => None,
    })
    .collect();

  Paragraph {
    inlines,
    footnote_reference_ids,
    endnote_reference_ids,
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
  hyperlinks: &HyperlinkCatalog,
) -> Table {
  let properties = table.w_tbl_pr.as_deref();
  let cell_margins = properties
    .and_then(|properties| properties.table_cell_margin_default.as_deref())
    .map(table_cell_margin_default)
    .unwrap_or_default();
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
    preferred_width_pct: properties
      .and_then(|properties| properties.table_width.as_ref())
      .and_then(table_width_to_percent),
    indent_left_pt: properties
      .and_then(|properties| properties.table_indentation.as_ref())
      .and_then(table_indentation_to_points)
      .unwrap_or(0.0),
    alignment: properties
      .and_then(|properties| properties.table_justification.as_ref())
      .map(table_alignment)
      .unwrap_or_default(),
    borders: properties
      .and_then(|properties| properties.table_borders.as_deref())
      .map(table_borders_model),
    cell_margins,
    rows: table
      .table_choice2
      .iter()
      .filter_map(|choice| match choice {
        w::TableChoice2::WTr(row) => Some(table_row_model(
          row,
          styles,
          numbering,
          images,
          hyperlinks,
          cell_margins,
        )),
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
  hyperlinks: &HyperlinkCatalog,
  cell_margins: CellMargins,
) -> TableRow {
  let (height_pt, exact_height) = table_row_height_properties(row.table_row_properties.as_deref());
  let (repeat_header, cant_split) = table_row_keep_properties(row.table_row_properties.as_deref());
  TableRow {
    height_pt,
    exact_height,
    repeat_header,
    cant_split,
    cells: row
      .table_row_choice
      .iter()
      .filter_map(|choice| match choice {
        w::TableRowChoice::WTc(cell) => Some(table_cell_model(
          cell,
          styles,
          numbering,
          images,
          hyperlinks,
          cell_margins,
        )),
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
  hyperlinks: &HyperlinkCatalog,
  default_margins: CellMargins,
) -> TableCell {
  let properties = cell.table_cell_properties.as_deref();
  TableCell {
    blocks: cell
      .table_cell_choice
      .iter()
      .filter_map(|choice| match choice {
        w::TableCellChoice::WP(paragraph) => Some(Block::Paragraph(paragraph_model(
          paragraph, styles, numbering, images, hyperlinks,
        ))),
        w::TableCellChoice::WTbl(table) => Some(Block::Table(table_model(
          table, styles, numbering, images, hyperlinks,
        ))),
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
    margins: properties
      .and_then(|properties| properties.table_cell_margin.as_deref())
      .map(|margins| table_cell_margin(margins, default_margins))
      .unwrap_or(default_margins),
    preferred_width_pt: properties
      .and_then(|properties| properties.table_cell_width.as_ref())
      .and_then(table_cell_width_to_points),
    preferred_width_pct: properties
      .and_then(|properties| properties.table_cell_width.as_ref())
      .and_then(table_cell_width_to_percent),
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

fn table_row_keep_properties(properties: Option<&w::TableRowProperties>) -> (bool, bool) {
  let Some(properties) = properties else {
    return (false, false);
  };
  let mut repeat_header = false;
  let mut cant_split = false;
  for choice in &properties.table_row_properties_choice1 {
    match choice {
      w::TableRowPropertiesChoice::WTblHeader(header) => {
        repeat_header = !matches!(header.val, Some(w::OnOffOnlyValues::Off));
      }
      w::TableRowPropertiesChoice::WCantSplit(cant_split_property) => {
        cant_split = !matches!(cant_split_property.val, Some(w::OnOffOnlyValues::Off));
      }
      _ => {}
    }
  }
  (repeat_header, cant_split)
}

fn table_cell_margin_default(margins: &w::TableCellMarginDefault) -> CellMargins {
  let mut model = CellMargins::default();
  if let Some(top) = &margins.top_margin
    && let Some(value) = margin_width_to_points(top.width.as_deref(), top.r#type)
  {
    model.top_pt = value;
  }
  if let Some(bottom) = &margins.bottom_margin
    && let Some(value) = margin_width_to_points(bottom.width.as_deref(), bottom.r#type)
  {
    model.bottom_pt = value;
  }
  if let Some(left) = &margins.table_cell_left_margin
    && matches!(left.r#type, w::TableWidthValues::Dxa)
  {
    model.left_pt = units::twips_to_points(left.width as f32);
  }
  if let Some(start) = &margins.start_margin
    && let Some(value) = margin_width_to_points(start.width.as_deref(), start.r#type)
  {
    model.left_pt = value;
  }
  if let Some(right) = &margins.table_cell_right_margin
    && matches!(right.r#type, w::TableWidthValues::Dxa)
  {
    model.right_pt = units::twips_to_points(right.width as f32);
  }
  if let Some(end) = &margins.end_margin
    && let Some(value) = margin_width_to_points(end.width.as_deref(), end.r#type)
  {
    model.right_pt = value;
  }
  model
}

fn table_cell_margin(margins: &w::TableCellMargin, mut model: CellMargins) -> CellMargins {
  if let Some(top) = &margins.top_margin
    && let Some(value) = margin_width_to_points(top.width.as_deref(), top.r#type)
  {
    model.top_pt = value;
  }
  if let Some(bottom) = &margins.bottom_margin
    && let Some(value) = margin_width_to_points(bottom.width.as_deref(), bottom.r#type)
  {
    model.bottom_pt = value;
  }
  if let Some(left) = &margins.left_margin
    && let Some(value) = margin_width_to_points(left.width.as_deref(), left.r#type)
  {
    model.left_pt = value;
  }
  if let Some(start) = &margins.start_margin
    && let Some(value) = margin_width_to_points(start.width.as_deref(), start.r#type)
  {
    model.left_pt = value;
  }
  if let Some(right) = &margins.right_margin
    && let Some(value) = margin_width_to_points(right.width.as_deref(), right.r#type)
  {
    model.right_pt = value;
  }
  if let Some(end) = &margins.end_margin
    && let Some(value) = margin_width_to_points(end.width.as_deref(), end.r#type)
  {
    model.right_pt = value;
  }
  model
}

fn margin_width_to_points(
  width: Option<&str>,
  width_type: Option<w::TableWidthUnitValues>,
) -> Option<f32> {
  if !matches!(width_type, None | Some(w::TableWidthUnitValues::Dxa)) {
    return None;
  }
  width.and_then(twips_attr_to_points)
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

fn table_width_to_percent(width: &w::TableWidth) -> Option<f32> {
  if !matches!(width.r#type, Some(w::TableWidthUnitValues::Pct)) {
    return None;
  }
  let value = width.width.as_deref()?;
  if let Some(percent) = value.strip_suffix('%') {
    return percent.parse::<f32>().ok().map(|value| value / 100.0);
  }
  value.parse::<f32>().ok().map(|value| value / 5000.0)
}

fn table_cell_width_to_points(width: &w::TableCellWidth) -> Option<f32> {
  match width.r#type {
    Some(w::TableWidthUnitValues::Dxa) | None => {
      width.width.as_deref().and_then(twips_attr_to_points)
    }
    _ => None,
  }
}

fn table_cell_width_to_percent(width: &w::TableCellWidth) -> Option<f32> {
  if !matches!(width.r#type, Some(w::TableWidthUnitValues::Pct)) {
    return None;
  }
  let value = width.width.as_deref()?;
  if let Some(percent) = value.strip_suffix('%') {
    return percent.parse::<f32>().ok().map(|value| value / 100.0);
  }
  value.parse::<f32>().ok().map(|value| value / 5000.0)
}

fn table_indentation_to_points(indentation: &w::TableIndentation) -> Option<f32> {
  if !matches!(
    indentation.r#type,
    None | Some(w::TableWidthUnitValues::Dxa)
  ) {
    return None;
  }
  indentation
    .width
    .map(|width| units::twips_to_points(width as f32))
}

fn table_alignment(justification: &w::TableJustification) -> TableAlignment {
  match justification.val {
    w::TableRowAlignmentValues::Center => TableAlignment::Center,
    w::TableRowAlignmentValues::Right => TableAlignment::Right,
    w::TableRowAlignmentValues::Left => TableAlignment::Left,
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

fn page_borders_model(borders: &w::PageBorders) -> CellBordersModel {
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
      border_style(
        border.val,
        border.size,
        border.space,
        border.color.as_deref(),
      )
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
  space: Option<u32>,
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
    spacing_pt: space.unwrap_or(0) as f32,
    color: color.and_then(parse_hex_color).unwrap_or_default(),
  })
}

fn document_background_color(background: &w::DocumentBackground) -> Option<RgbColor> {
  background.color.as_deref().and_then(parse_hex_color)
}

fn merge_paragraph_format(format: &mut ParagraphFormat, properties: Option<ParagraphProps<'_>>) {
  let Some(properties) = properties else {
    return;
  };

  if let Some(page_break_before) = properties.page_break_before() {
    format.page_break_before = page_break_before.val.unwrap_or(true);
  }
  if let Some(keep_next) = properties.keep_next() {
    format.keep_with_next = keep_next.val.unwrap_or(true);
  }
  if let Some(keep_lines) = properties.keep_lines() {
    format.keep_lines = keep_lines.val.unwrap_or(true);
  }
  if let Some(contextual_spacing) = properties.contextual_spacing() {
    format.contextual_spacing = contextual_spacing.val.unwrap_or(true);
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
    if let Some(line) = spacing.line.as_deref() {
      match spacing.line_rule {
        None | Some(w::LineSpacingRuleValues::Auto) => {
          format.line_height_rule = LineHeightRule::Auto;
          if let Ok(value) = line.parse::<f32>() {
            format.line_height_pt = Some(14.0 * (value / 240.0).max(0.1));
          }
        }
        Some(w::LineSpacingRuleValues::AtLeast) => {
          format.line_height_rule = LineHeightRule::AtLeast;
          format.line_height_pt = twips_attr_to_points(line);
        }
        Some(w::LineSpacingRuleValues::Exact) => {
          format.line_height_rule = LineHeightRule::Exact;
          format.line_height_pt = twips_attr_to_points(line);
        }
      }
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
  hyperlinks: &HyperlinkCatalog,
) -> Vec<InlineItem> {
  let mut inlines = Vec::new();
  let mut complex_field = None;

  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WR(run) => {
        push_run_or_complex_field(
          run,
          &mut inlines,
          base_style,
          styles,
          images,
          None,
          &mut complex_field,
        );
      }
      w::ParagraphChoice::WFldSimple(field) => {
        push_simple_field(field, &mut inlines, base_style, styles, images, hyperlinks);
      }
      w::ParagraphChoice::WHyperlink(hyperlink) => {
        let hyperlink_url = hyperlink_url(hyperlink, hyperlinks);
        for item in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = item {
            push_run_or_complex_field(
              run,
              &mut inlines,
              base_style,
              styles,
              images,
              hyperlink_url.as_deref(),
              &mut complex_field,
            );
          }
        }
      }
      w::ParagraphChoice::Choice(choice) => match choice.as_ref() {
        w::ParagraphChoice2::WIns(inserted) => {
          push_inserted_run(inserted, &mut inlines, base_style, styles, images, None);
        }
        w::ParagraphChoice2::WDel(_)
        | w::ParagraphChoice2::WMoveFrom(_)
        | w::ParagraphChoice2::WMoveTo(_) => {}
        _ => {}
      },
      w::ParagraphChoice::WSdt(sdt) => push_sdt_run(
        sdt,
        &mut inlines,
        base_style,
        styles,
        images,
        hyperlinks,
        None,
      ),
      _ => {}
    }
  }
  flush_unclosed_complex_field(&mut inlines, &mut complex_field);

  inlines
}

#[derive(Clone, Debug)]
struct ComplexFieldState {
  instr: String,
  result: Vec<InlineItem>,
  in_result: bool,
  style: TextStyle,
  hyperlink_url: Option<String>,
}

fn push_run_or_complex_field(
  run: &w::Run,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlink_url: Option<&str>,
  state: &mut Option<ComplexFieldState>,
) {
  if state.is_none() && !run_starts_complex_field(run) {
    push_run(run, inlines, base_style, styles, images, hyperlink_url);
    return;
  }

  let style = run_style(run.run_properties.as_deref(), base_style, styles);
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::WFldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Begin =>
      {
        *state = Some(ComplexFieldState {
          instr: String::new(),
          result: Vec::new(),
          in_result: false,
          style,
          hyperlink_url: hyperlink_url.map(ToString::to_string),
        });
      }
      w::RunChoice::WFldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Separate =>
      {
        if let Some(state) = state {
          state.in_result = true;
        }
      }
      w::RunChoice::WFldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::End =>
      {
        flush_closed_complex_field(inlines, state);
      }
      w::RunChoice::WInstrText(code) => {
        if let Some(state) = state
          && !state.in_result
          && let Some(content) = &code.xml_content
        {
          state.instr.push_str(content);
        }
      }
      _ => {
        if let Some(state) = state
          && state.in_result
        {
          push_run(
            run,
            &mut state.result,
            base_style,
            styles,
            images,
            hyperlink_url,
          );
          break;
        }
      }
    }
  }
}

fn run_starts_complex_field(run: &w::Run) -> bool {
  run.run_choice.iter().any(|choice| {
    matches!(
      choice,
      w::RunChoice::WFldChar(field_char)
        if field_char.field_char_type == w::FieldCharValues::Begin
    )
  })
}

fn flush_closed_complex_field(
  inlines: &mut Vec<InlineItem>,
  state: &mut Option<ComplexFieldState>,
) {
  let Some(state) = state.take() else {
    return;
  };
  if let Some(kind) = dynamic_field_kind(&state.instr) {
    push_dynamic_field(inlines, kind, state.style, state.hyperlink_url.as_deref());
  } else {
    inlines.extend(state.result);
  }
}

fn flush_unclosed_complex_field(
  inlines: &mut Vec<InlineItem>,
  state: &mut Option<ComplexFieldState>,
) {
  if state.is_some() {
    flush_closed_complex_field(inlines, state);
  }
}

fn dynamic_field_kind(instr: &str) -> Option<DynamicFieldKind> {
  let name = instr.split_whitespace().next()?.to_ascii_uppercase();
  match name.as_str() {
    "PAGE" => Some(DynamicFieldKind::Page),
    "NUMPAGES" => Some(DynamicFieldKind::NumPages),
    _ => None,
  }
}

fn push_dynamic_field(
  inlines: &mut Vec<InlineItem>,
  kind: DynamicFieldKind,
  style: TextStyle,
  hyperlink_url: Option<&str>,
) {
  inlines.push(InlineItem::Text(TextRun {
    text: "1".to_string(),
    style,
    hyperlink_url: hyperlink_url.map(ToString::to_string),
    dynamic_field: Some(kind),
  }));
}

fn hyperlink_url(hyperlink: &w::Hyperlink, hyperlinks: &HyperlinkCatalog) -> Option<String> {
  hyperlink
    .id
    .as_deref()
    .and_then(|relationship_id| hyperlinks.target(relationship_id))
    .map(ToString::to_string)
}

fn paragraph_note_reference_ids(paragraph: &w::Paragraph) -> (Vec<i64>, Vec<i64>) {
  let mut footnotes = Vec::new();
  let mut endnotes = Vec::new();
  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WR(run) => {
        collect_run_note_reference_ids(run, &mut footnotes, &mut endnotes)
      }
      w::ParagraphChoice::WFldSimple(field) => {
        for choice in &field.simple_field_choice {
          if let w::SimpleFieldChoice::WR(run) = choice {
            collect_run_note_reference_ids(run, &mut footnotes, &mut endnotes);
          }
        }
      }
      w::ParagraphChoice::WHyperlink(hyperlink) => {
        for choice in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = choice {
            collect_run_note_reference_ids(run, &mut footnotes, &mut endnotes);
          }
        }
      }
      w::ParagraphChoice::Choice(choice) => {
        if let w::ParagraphChoice2::WIns(inserted) = choice.as_ref() {
          for choice in &inserted.inserted_run_choice {
            if let w::InsertedRunChoice::WR(run) = choice {
              collect_run_note_reference_ids(run, &mut footnotes, &mut endnotes);
            }
          }
        }
      }
      _ => {}
    }
  }
  footnotes.sort_unstable();
  footnotes.dedup();
  endnotes.sort_unstable();
  endnotes.dedup();
  (footnotes, endnotes)
}

fn collect_run_note_reference_ids(run: &w::Run, footnotes: &mut Vec<i64>, endnotes: &mut Vec<i64>) {
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::WFootnoteReference(reference) if reference.id > 0 => {
        footnotes.push(reference.id);
      }
      w::RunChoice::WEndnoteReference(reference) if reference.id > 0 => {
        endnotes.push(reference.id);
      }
      _ => {}
    }
  }
}

fn push_simple_field(
  field: &w::SimpleField,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlinks: &HyperlinkCatalog,
) {
  if let Some(kind) = dynamic_field_kind(&field.instruction) {
    push_dynamic_field(inlines, kind, base_style, None);
    return;
  }

  for choice in &field.simple_field_choice {
    match choice {
      w::SimpleFieldChoice::WR(run) => push_run(run, inlines, base_style, styles, images, None),
      w::SimpleFieldChoice::WHyperlink(hyperlink) => {
        let hyperlink_url = hyperlink_url(hyperlink, hyperlinks);
        for item in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = item {
            push_run(
              run,
              inlines,
              base_style,
              styles,
              images,
              hyperlink_url.as_deref(),
            );
          }
        }
      }
      w::SimpleFieldChoice::WFldSimple(field) => {
        push_simple_field(field, inlines, base_style, styles, images, hyperlinks);
      }
      w::SimpleFieldChoice::WSdt(sdt) => {
        push_sdt_run(sdt, inlines, base_style, styles, images, hyperlinks, None)
      }
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
  hyperlink_url: Option<&str>,
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
        Some(w::BreakValues::Page) => {
          flush_run_text(inlines, &mut text, style, hyperlink_url);
          inlines.push(InlineItem::PageBreak);
        }
        Some(w::BreakValues::Column) => {
          flush_run_text(inlines, &mut text, style, hyperlink_url);
          inlines.push(InlineItem::ColumnBreak);
        }
        Some(w::BreakValues::TextWrapping) | None => text.push('\n'),
      },
      w::RunChoice::WSym(symbol) => {
        if let Some(symbol) = symbol_text(symbol) {
          text.push(symbol);
        }
      }
      w::RunChoice::WPgNum => {
        flush_run_text(inlines, &mut text, style, hyperlink_url);
        push_dynamic_field(inlines, DynamicFieldKind::Page, style, hyperlink_url);
      }
      w::RunChoice::WNoBreakHyphen => text.push('\u{2011}'),
      w::RunChoice::WSoftHyphen => text.push('\u{00ad}'),
      w::RunChoice::WFootnoteReference(reference) => {
        flush_run_text(inlines, &mut text, style, hyperlink_url);
        push_note_reference(inlines, reference.id, style);
      }
      w::RunChoice::WEndnoteReference(reference) => {
        flush_run_text(inlines, &mut text, style, hyperlink_url);
        push_note_reference(inlines, reference.id, style);
      }
      w::RunChoice::WCommentReference(reference) => {
        flush_run_text(inlines, &mut text, style, hyperlink_url);
        push_comment_reference(inlines, &reference.id, style);
      }
      w::RunChoice::WDrawing(drawing) => {
        flush_run_text(inlines, &mut text, style, hyperlink_url);
        if let Some(image) = inline_image(drawing, images) {
          inlines.push(InlineItem::Image(image));
        }
        push_drawing_textboxes(drawing, inlines, style);
      }
      w::RunChoice::WPict(picture) => {
        flush_run_text(inlines, &mut text, style, hyperlink_url);
        if let Some(image) = pict_image(picture, images) {
          inlines.push(InlineItem::Image(image));
        }
        push_pict_textboxes(picture, inlines, base_style, styles, images);
      }
      w::RunChoice::WPtab(_) => text.push('\t'),
      w::RunChoice::WRuby(ruby) => {
        flush_run_text(inlines, &mut text, style, hyperlink_url);
        push_ruby_base(ruby, inlines, base_style, styles, images, hyperlink_url);
      }
      _ => {}
    }
  }

  flush_run_text(inlines, &mut text, style, hyperlink_url);
}

fn push_ruby_base(
  ruby: &w::Ruby,
  inlines: &mut Vec<InlineItem>,
  base_style: TextStyle,
  styles: &StylesCatalog,
  images: &ImageCatalog,
  hyperlink_url: Option<&str>,
) {
  for choice in &ruby.ruby_base.ruby_base_choice {
    match choice {
      w::RubyBaseChoice::WR(run) => {
        push_run(run, inlines, base_style, styles, images, hyperlink_url)
      }
      w::RubyBaseChoice::WIns(inserted) => {
        push_inserted_run(inserted, inlines, base_style, styles, images, hyperlink_url);
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
  hyperlinks: &HyperlinkCatalog,
  hyperlink_url: Option<&str>,
) {
  let Some(content) = sdt.sdt_content_run.as_ref() else {
    return;
  };

  for choice in &content.sdt_content_run_choice {
    match choice {
      w::SdtContentRunChoice::WR(run) => push_run(
        run.as_ref(),
        inlines,
        base_style,
        styles,
        images,
        hyperlink_url,
      ),
      w::SdtContentRunChoice::WFldSimple(field) => {
        push_simple_field(
          field.as_ref(),
          inlines,
          base_style,
          styles,
          images,
          hyperlinks,
        );
      }
      w::SdtContentRunChoice::WHyperlink(hyperlink) => {
        let nested_url = self::hyperlink_url(hyperlink, hyperlinks)
          .or_else(|| hyperlink_url.map(ToString::to_string));
        for item in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WR(run) = item {
            push_run(
              run,
              inlines,
              base_style,
              styles,
              images,
              nested_url.as_deref(),
            );
          }
        }
      }
      w::SdtContentRunChoice::WSdt(sdt) => push_sdt_run(
        sdt.as_ref(),
        inlines,
        base_style,
        styles,
        images,
        hyperlinks,
        hyperlink_url,
      ),
      w::SdtContentRunChoice::WIns(inserted) => {
        push_inserted_run(
          inserted.as_ref(),
          inlines,
          base_style,
          styles,
          images,
          hyperlink_url,
        );
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
  hyperlink_url: Option<&str>,
) {
  for choice in &inserted.inserted_run_choice {
    match choice {
      w::InsertedRunChoice::WR(run) => {
        push_run(run, inlines, base_style, styles, images, hyperlink_url)
      }
      w::InsertedRunChoice::Choice(choice) => match choice.as_ref() {
        w::InsertedRunChoice2::WIns(nested) => {
          push_inserted_run(nested, inlines, base_style, styles, images, hyperlink_url);
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
    hyperlink_url: None,
    dynamic_field: None,
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
    hyperlink_url: None,
    dynamic_field: None,
  }));
}

fn flush_run_text(
  inlines: &mut Vec<InlineItem>,
  text: &mut String,
  style: TextStyle,
  hyperlink_url: Option<&str>,
) {
  if !text.is_empty() {
    inlines.push(InlineItem::Text(TextRun {
      text: run_display_text(std::mem::take(text), style),
      style,
      hyperlink_url: hyperlink_url.map(ToString::to_string),
      dynamic_field: None,
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
      let properties = drawing_image_properties(&inline.graphic.graphic_data)?;
      let relationship_id = properties.relationship_id?;
      let resource = images.by_relationship_id.get(&relationship_id)?;
      Some(InlineImage {
        data: resource.data.clone(),
        content_type: resource.content_type.clone(),
        width_pt: units::emu_to_points(inline.extent.cx),
        height_pt: units::emu_to_points(inline.extent.cy),
        effect_left_pt: effect_extent_left(inline.effect_extent.as_ref()),
        effect_top_pt: effect_extent_top(inline.effect_extent.as_ref()),
        effect_right_pt: effect_extent_right(inline.effect_extent.as_ref()),
        effect_bottom_pt: effect_extent_bottom(inline.effect_extent.as_ref()),
        crop: properties.crop,
        rotation_deg: properties.rotation_deg,
        flip_horizontal: properties.flip_horizontal,
        flip_vertical: properties.flip_vertical,
        alt_text: inline.doc_properties.description.clone(),
        placement: ImagePlacement::Inline,
      })
    }
    w::DrawingChoice::WpAnchor(anchor) => {
      let graphic = anchor.a_graphic.as_ref()?;
      let extent = anchor.extent.as_ref()?;
      let properties = drawing_image_properties(&graphic.graphic_data)?;
      let relationship_id = properties.relationship_id?;
      let resource = images.by_relationship_id.get(&relationship_id)?;
      Some(InlineImage {
        data: resource.data.clone(),
        content_type: resource.content_type.clone(),
        width_pt: units::emu_to_points(extent.cx),
        height_pt: units::emu_to_points(extent.cy),
        effect_left_pt: effect_extent_left(anchor.effect_extent.as_ref()),
        effect_top_pt: effect_extent_top(anchor.effect_extent.as_ref()),
        effect_right_pt: effect_extent_right(anchor.effect_extent.as_ref()),
        effect_bottom_pt: effect_extent_bottom(anchor.effect_extent.as_ref()),
        crop: properties.crop,
        rotation_deg: properties.rotation_deg,
        flip_horizontal: properties.flip_horizontal,
        flip_vertical: properties.flip_vertical,
        alt_text: anchor
          .wp_doc_pr
          .as_ref()
          .and_then(|properties| properties.description.clone()),
        placement: ImagePlacement::Floating(floating_image_placement(anchor)),
      })
    }
  }
}

fn effect_extent_left(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.left_edge))
    .unwrap_or(0.0)
}

fn effect_extent_top(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.top_edge))
    .unwrap_or(0.0)
}

fn effect_extent_right(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.right_edge))
    .unwrap_or(0.0)
}

fn effect_extent_bottom(extent: Option<&wp::EffectExtent>) -> f32 {
  extent
    .map(|extent| units::emu_to_points(extent.bottom_edge))
    .unwrap_or(0.0)
}

fn floating_image_placement(anchor: &wp::Anchor) -> FloatingImagePlacement {
  let margins = floating_wrap_margins(anchor);
  FloatingImagePlacement {
    horizontal_relative_to: anchor
      .horizontal_position
      .as_deref()
      .map(horizontal_image_reference)
      .unwrap_or_default(),
    vertical_relative_to: anchor
      .vertical_position
      .as_deref()
      .map(vertical_image_reference)
      .unwrap_or_default(),
    horizontal_alignment: anchor
      .horizontal_position
      .as_deref()
      .and_then(horizontal_position_alignment),
    vertical_alignment: anchor
      .vertical_position
      .as_deref()
      .and_then(vertical_position_alignment),
    horizontal_offset_pt: anchor
      .horizontal_position
      .as_deref()
      .and_then(horizontal_position_offset)
      .unwrap_or(0.0),
    vertical_offset_pt: anchor
      .vertical_position
      .as_deref()
      .and_then(vertical_position_offset)
      .unwrap_or(0.0),
    wrap: anchor
      .anchor_choice
      .as_ref()
      .map(image_wrap_mode)
      .unwrap_or(ImageWrapMode::None),
    wrap_side: anchor
      .anchor_choice
      .as_ref()
      .map(image_wrap_side)
      .unwrap_or_default(),
    behind_text: anchor.behind_doc,
    margin_top_pt: margins.top_pt,
    margin_right_pt: margins.right_pt,
    margin_bottom_pt: margins.bottom_pt,
    margin_left_pt: margins.left_pt,
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct ImageWrapMargins {
  top_pt: f32,
  right_pt: f32,
  bottom_pt: f32,
  left_pt: f32,
}

fn floating_wrap_margins(anchor: &wp::Anchor) -> ImageWrapMargins {
  let mut margins = ImageWrapMargins {
    top_pt: optional_emu_to_points(anchor.distance_from_top),
    right_pt: optional_emu_to_points(anchor.distance_from_right),
    bottom_pt: optional_emu_to_points(anchor.distance_from_bottom),
    left_pt: optional_emu_to_points(anchor.distance_from_left),
  };

  match anchor.anchor_choice.as_ref() {
    Some(wp::AnchorChoice::WpWrapSquare(square)) => {
      margins.top_pt = optional_emu_to_points(square.distance_from_top).max(margins.top_pt);
      margins.right_pt = optional_emu_to_points(square.distance_from_right).max(margins.right_pt);
      margins.bottom_pt =
        optional_emu_to_points(square.distance_from_bottom).max(margins.bottom_pt);
      margins.left_pt = optional_emu_to_points(square.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WpWrapTight(tight)) => {
      margins.right_pt = optional_emu_to_points(tight.distance_from_right).max(margins.right_pt);
      margins.left_pt = optional_emu_to_points(tight.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WpWrapThrough(through)) => {
      margins.right_pt = optional_emu_to_points(through.distance_from_right).max(margins.right_pt);
      margins.left_pt = optional_emu_to_points(through.distance_from_left).max(margins.left_pt);
    }
    Some(wp::AnchorChoice::WpWrapTopAndBottom(top_bottom)) => {
      margins.top_pt = optional_emu_to_points(top_bottom.distance_from_top).max(margins.top_pt);
      margins.bottom_pt =
        optional_emu_to_points(top_bottom.distance_from_bottom).max(margins.bottom_pt);
    }
    Some(wp::AnchorChoice::WpWrapNone) | None => {}
  }

  margins
}

fn optional_emu_to_points(value: Option<u32>) -> f32 {
  value
    .map(|value| units::emu_to_points(value as i64))
    .unwrap_or(0.0)
}

fn horizontal_image_reference(position: &wp::HorizontalPosition) -> HorizontalImageReference {
  match position.relative_from {
    wp::HorizontalRelativePositionValues::Page => HorizontalImageReference::Page,
    wp::HorizontalRelativePositionValues::Column => HorizontalImageReference::Column,
    wp::HorizontalRelativePositionValues::Character => HorizontalImageReference::Character,
    wp::HorizontalRelativePositionValues::Margin
    | wp::HorizontalRelativePositionValues::LeftMargin
    | wp::HorizontalRelativePositionValues::RightMargin
    | wp::HorizontalRelativePositionValues::InsideMargin
    | wp::HorizontalRelativePositionValues::OutsideMargin => HorizontalImageReference::Margin,
  }
}

fn vertical_image_reference(position: &wp::VerticalPosition) -> VerticalImageReference {
  match position.relative_from {
    wp::VerticalRelativePositionValues::Page => VerticalImageReference::Page,
    wp::VerticalRelativePositionValues::Paragraph => VerticalImageReference::Paragraph,
    wp::VerticalRelativePositionValues::Line => VerticalImageReference::Line,
    wp::VerticalRelativePositionValues::Margin
    | wp::VerticalRelativePositionValues::TopMargin
    | wp::VerticalRelativePositionValues::BottomMargin
    | wp::VerticalRelativePositionValues::InsideMargin
    | wp::VerticalRelativePositionValues::OutsideMargin => VerticalImageReference::Margin,
  }
}

fn horizontal_position_offset(position: &wp::HorizontalPosition) -> Option<f32> {
  match position.horizontal_position_choice.as_ref()? {
    wp::HorizontalPositionChoice::WpPosOffset(offset) => Some(units::emu_to_points(*offset as i64)),
    wp::HorizontalPositionChoice::WpAlign(_)
    | wp::HorizontalPositionChoice::Wp14PctPosHOffset(_) => None,
  }
}

fn horizontal_position_alignment(
  position: &wp::HorizontalPosition,
) -> Option<HorizontalImageAlignment> {
  match position.horizontal_position_choice.as_ref()? {
    wp::HorizontalPositionChoice::WpAlign(alignment) => match alignment {
      wp::HorizontalAlignmentValues::Left | wp::HorizontalAlignmentValues::Inside => {
        Some(HorizontalImageAlignment::Left)
      }
      wp::HorizontalAlignmentValues::Center => Some(HorizontalImageAlignment::Center),
      wp::HorizontalAlignmentValues::Right | wp::HorizontalAlignmentValues::Outside => {
        Some(HorizontalImageAlignment::Right)
      }
    },
    wp::HorizontalPositionChoice::WpPosOffset(_)
    | wp::HorizontalPositionChoice::Wp14PctPosHOffset(_) => None,
  }
}

fn vertical_position_offset(position: &wp::VerticalPosition) -> Option<f32> {
  match position.vertical_position_choice.as_ref()? {
    wp::VerticalPositionChoice::WpPosOffset(offset) => Some(units::emu_to_points(*offset as i64)),
    wp::VerticalPositionChoice::WpAlign(_) | wp::VerticalPositionChoice::Wp14PctPosVOffset(_) => {
      None
    }
  }
}

fn vertical_position_alignment(position: &wp::VerticalPosition) -> Option<VerticalImageAlignment> {
  match position.vertical_position_choice.as_ref()? {
    wp::VerticalPositionChoice::WpAlign(alignment) => match alignment {
      wp::VerticalAlignmentValues::Top | wp::VerticalAlignmentValues::Inside => {
        Some(VerticalImageAlignment::Top)
      }
      wp::VerticalAlignmentValues::Center => Some(VerticalImageAlignment::Center),
      wp::VerticalAlignmentValues::Bottom | wp::VerticalAlignmentValues::Outside => {
        Some(VerticalImageAlignment::Bottom)
      }
    },
    wp::VerticalPositionChoice::WpPosOffset(_)
    | wp::VerticalPositionChoice::Wp14PctPosVOffset(_) => None,
  }
}

fn image_wrap_mode(choice: &wp::AnchorChoice) -> ImageWrapMode {
  match choice {
    wp::AnchorChoice::WpWrapNone => ImageWrapMode::Through,
    wp::AnchorChoice::WpWrapSquare(_) => ImageWrapMode::Square,
    wp::AnchorChoice::WpWrapTight(_) => ImageWrapMode::Tight,
    wp::AnchorChoice::WpWrapThrough(_) => ImageWrapMode::Through,
    wp::AnchorChoice::WpWrapTopAndBottom(_) => ImageWrapMode::TopBottom,
  }
}

fn image_wrap_side(choice: &wp::AnchorChoice) -> ImageWrapSide {
  match choice {
    wp::AnchorChoice::WpWrapSquare(square) => wrap_text_side(square.wrap_text),
    wp::AnchorChoice::WpWrapTight(tight) => wrap_text_side(tight.wrap_text),
    wp::AnchorChoice::WpWrapThrough(through) => wrap_text_side(through.wrap_text),
    wp::AnchorChoice::WpWrapNone | wp::AnchorChoice::WpWrapTopAndBottom(_) => {
      ImageWrapSide::BothSides
    }
  }
}

fn wrap_text_side(value: wp::WrapTextValues) -> ImageWrapSide {
  match value {
    wp::WrapTextValues::BothSides => ImageWrapSide::BothSides,
    wp::WrapTextValues::Left => ImageWrapSide::Left,
    wp::WrapTextValues::Right => ImageWrapSide::Right,
    wp::WrapTextValues::Largest => ImageWrapSide::Largest,
  }
}

fn push_drawing_textboxes(drawing: &w::Drawing, inlines: &mut Vec<InlineItem>, style: TextStyle) {
  let Some(graphic_data) = drawing_graphic_data(drawing) else {
    return;
  };

  for child in &graphic_data.xml_children {
    if let Some(text) = drawing_textbox_text(child) {
      inlines.push(InlineItem::Text(TextRun {
        text,
        style,
        hyperlink_url: None,
        dynamic_field: None,
      }));
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
  let hyperlinks = HyperlinkCatalog::default();
  for choice in &content.text_box_content_choice {
    match choice {
      w::TextBoxContentChoice::WP(paragraph) => {
        let paragraph = paragraph_model(paragraph, styles, &mut numbering, images, &hyperlinks);
        inlines.extend(paragraph.inlines);
        inlines.push(InlineItem::Text(TextRun {
          text: "\n".into(),
          style: base_style,
          hyperlink_url: None,
          dynamic_field: None,
        }));
      }
      w::TextBoxContentChoice::WTbl(table) => {
        let table = table_model(table, styles, &mut numbering, images, &hyperlinks);
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
          hyperlink_url: None,
          dynamic_field: None,
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
      hyperlink_url: None,
      dynamic_field: None,
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
    effect_left_pt: 0.0,
    effect_top_pt: 0.0,
    effect_right_pt: 0.0,
    effect_bottom_pt: 0.0,
    crop: ImageCrop::default(),
    rotation_deg: 0.0,
    flip_horizontal: false,
    flip_vertical: false,
    alt_text: alt_text.or_else(|| data.title.clone()),
    placement: ImagePlacement::Inline,
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

#[derive(Clone, Debug, Default)]
struct DrawingImageProperties {
  relationship_id: Option<String>,
  crop: ImageCrop,
  rotation_deg: f32,
  flip_horizontal: bool,
  flip_vertical: bool,
}

fn drawing_image_properties(
  graphic_data: &ooxmlsdk::schemas::a::GraphicData,
) -> Option<DrawingImageProperties> {
  graphic_data
    .xml_children
    .iter()
    .find_map(|child| drawing_image_properties_from_xml(child))
}

fn drawing_image_properties_from_xml(xml: &str) -> Option<DrawingImageProperties> {
  let mut reader = Reader::from_str(xml);
  reader.config_mut().trim_text(true);
  let mut properties = DrawingImageProperties::default();
  loop {
    match reader.read_event().ok()? {
      Event::Empty(event) | Event::Start(event) if event.name().as_ref().ends_with(b":blip") => {
        for attr in event.attributes().with_checks(false).flatten() {
          if attr.key.as_ref().ends_with(b":embed") || attr.key.as_ref() == b"embed" {
            properties.relationship_id = attr
              .decode_and_unescape_value(reader.decoder())
              .ok()
              .map(|value| value.into_owned());
          }
        }
      }
      Event::Empty(event) | Event::Start(event)
        if qname_ends_with(event.name().as_ref(), b"srcRect") =>
      {
        properties.crop = image_crop_from_src_rect(&event, reader.decoder());
      }
      Event::Empty(event) | Event::Start(event)
        if qname_ends_with(event.name().as_ref(), b"xfrm") =>
      {
        apply_image_transform_attrs(&mut properties, &event, reader.decoder());
      }
      Event::Eof => return properties.relationship_id.is_some().then_some(properties),
      _ => {}
    }
  }
}

fn image_crop_from_src_rect(
  event: &quick_xml::events::BytesStart<'_>,
  decoder: quick_xml::Decoder,
) -> ImageCrop {
  let mut crop = ImageCrop::default();
  for attr in event.attributes().with_checks(false).flatten() {
    let value = attr
      .decode_and_unescape_value(decoder)
      .ok()
      .and_then(|value| value.parse::<i32>().ok())
      .map(relative_rect_attr_to_fraction)
      .unwrap_or(0.0);
    match attr.key.as_ref() {
      b"l" => crop.left = value,
      b"t" => crop.top = value,
      b"r" => crop.right = value,
      b"b" => crop.bottom = value,
      _ => {}
    }
  }
  crop
}

fn apply_image_transform_attrs(
  properties: &mut DrawingImageProperties,
  event: &quick_xml::events::BytesStart<'_>,
  decoder: quick_xml::Decoder,
) {
  for attr in event.attributes().with_checks(false).flatten() {
    let value = attr.decode_and_unescape_value(decoder).ok();
    match attr.key.as_ref() {
      b"rot" => {
        properties.rotation_deg = value
          .as_deref()
          .and_then(|value| value.parse::<i32>().ok())
          .map(|value| value as f32 / 60000.0)
          .unwrap_or(0.0);
      }
      b"flipH" => properties.flip_horizontal = value.as_deref().is_some_and(xml_bool),
      b"flipV" => properties.flip_vertical = value.as_deref().is_some_and(xml_bool),
      _ => {}
    }
  }
}

fn relative_rect_attr_to_fraction(value: i32) -> f32 {
  (value as f32 / 100000.0).clamp(0.0, 0.999)
}

fn xml_bool(value: &str) -> bool {
  matches!(value, "1" | "true" | "t" | "on")
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

#[derive(Clone, Debug, Default)]
struct HyperlinkCatalog {
  by_relationship_id: HashMap<String, String>,
}

impl HyperlinkCatalog {
  fn load<P>(package: &WordprocessingDocument, part: &P) -> Self
  where
    P: SdkPart,
  {
    let by_relationship_id = part
      .hyperlink_relationships(package)
      .filter(|relationship| relationship.target_kind() == RelationshipTargetKind::External)
      .map(|relationship| {
        (
          relationship.id().to_string(),
          relationship.target().to_string(),
        )
      })
      .collect();
    Self { by_relationship_id }
  }

  fn target(&self, relationship_id: &str) -> Option<&str> {
    self
      .by_relationship_id
      .get(relationship_id)
      .map(String::as_str)
  }
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
  run_overrides: RunStyleOverrides,
}

#[derive(Clone, Copy, Debug, Default)]
struct RunStyleOverrides {
  bold: Option<bool>,
  italic: Option<bool>,
  underline: Option<bool>,
  strikethrough: Option<bool>,
  uppercase: Option<bool>,
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
        run_overrides: RunStyleOverrides::default(),
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
      entry.run_overrides =
        run_style_overrides(style.style_run_properties.as_deref().map(RunProps::Style));
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
      apply_run_style_overrides(&mut style, entry.run_overrides);
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
        apply_run_style_overrides(&mut style, entry.run_overrides);
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

fn run_style_overrides(properties: Option<RunProps<'_>>) -> RunStyleOverrides {
  let Some(properties) = properties else {
    return RunStyleOverrides::default();
  };

  RunStyleOverrides {
    bold: properties.bold().and_then(|value| value.val),
    italic: properties.italic().and_then(|value| value.val),
    underline: properties
      .underline()
      .map(|value| !matches!(value.val, Some(w::UnderlineValues::None))),
    strikethrough: properties
      .double_strike()
      .and_then(|value| value.val)
      .or_else(|| properties.strike().and_then(|value| value.val)),
    uppercase: properties
      .small_caps()
      .and_then(|value| value.val)
      .or_else(|| properties.caps().and_then(|value| value.val)),
  }
}

fn apply_run_style_overrides(style: &mut TextStyle, overrides: RunStyleOverrides) {
  if let Some(bold) = overrides.bold {
    style.bold = bold;
  }
  if let Some(italic) = overrides.italic {
    style.italic = italic;
  }
  if let Some(underline) = overrides.underline {
    style.underline = underline;
  }
  if let Some(strikethrough) = overrides.strikethrough {
    style.strikethrough = strikethrough;
  }
  if let Some(uppercase) = overrides.uppercase {
    style.uppercase = uppercase;
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
    target.line_height_rule = values.line_height_rule;
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
  if values.keep_with_next {
    target.keep_with_next = true;
  }
  if values.keep_lines {
    target.keep_lines = true;
  }
  if values.contextual_spacing {
    target.contextual_spacing = true;
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

  fn keep_next(&self) -> Option<&'a w::KeepNext> {
    match self {
      Self::Direct(properties) => properties.keep_next.as_ref(),
      Self::Style(properties) => properties.keep_next.as_ref(),
      Self::BaseStyle(properties) => properties.keep_next.as_ref(),
      Self::Previous(properties) => properties.keep_next.as_ref(),
    }
  }

  fn keep_lines(&self) -> Option<&'a w::KeepLines> {
    match self {
      Self::Direct(properties) => properties.keep_lines.as_ref(),
      Self::Style(properties) => properties.keep_lines.as_ref(),
      Self::BaseStyle(properties) => properties.keep_lines.as_ref(),
      Self::Previous(properties) => properties.keep_lines.as_ref(),
    }
  }

  fn contextual_spacing(&self) -> Option<&'a w::ContextualSpacing> {
    match self {
      Self::Direct(properties) => properties.contextual_spacing.as_ref(),
      Self::Style(properties) => properties.contextual_spacing.as_ref(),
      Self::BaseStyle(properties) => properties.contextual_spacing.as_ref(),
      Self::Previous(properties) => properties.contextual_spacing.as_ref(),
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

  if let Some(borders) = &section.w_pg_borders {
    setup.borders = page_borders_model(borders);
    setup.borders_offset_from_text =
      matches!(borders.offset_from, Some(w::PageBorderOffsetValues::Text));
  }

  setup
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn drawing_image_properties_preserve_crop_and_transform() {
    let xml = r#"<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><pic:blipFill><a:blip r:embed="rId7"/><a:srcRect l="10000" t="20000" r="30000" b="40000"/></pic:blipFill><pic:spPr><a:xfrm rot="5400000" flipH="1" flipV="true"/></pic:spPr></pic:pic>"#;

    let properties = drawing_image_properties_from_xml(xml).expect("image properties");

    assert_eq!(properties.relationship_id.as_deref(), Some("rId7"));
    assert!((properties.crop.left - 0.1).abs() < 0.001);
    assert!((properties.crop.top - 0.2).abs() < 0.001);
    assert!((properties.crop.right - 0.3).abs() < 0.001);
    assert!((properties.crop.bottom - 0.4).abs() < 0.001);
    assert!((properties.rotation_deg - 90.0).abs() < 0.001);
    assert!(properties.flip_horizontal);
    assert!(properties.flip_vertical);
  }

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
      None,
    );

    assert_eq!(inline_text(&inlines), "•✓©");
  }

  #[test]
  fn simple_page_fields_emit_dynamic_markers() {
    let mut inlines = Vec::new();
    let field = w::SimpleField {
      instruction: " PAGE ".into(),
      ..Default::default()
    };

    push_simple_field(
      &field,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
    );

    let InlineItem::Text(run) = &inlines[0] else {
      panic!("expected dynamic field text");
    };
    assert_eq!(run.dynamic_field, Some(DynamicFieldKind::Page));
  }

  #[test]
  fn pgnum_runs_emit_dynamic_page_marker() {
    let mut inlines = Vec::new();
    let run = w::Run {
      run_choice: vec![w::RunChoice::WPgNum],
      ..Default::default()
    };

    push_run(
      &run,
      &mut inlines,
      TextStyle::default(),
      &StylesCatalog::default(),
      &ImageCatalog::default(),
      None,
    );

    let InlineItem::Text(run) = &inlines[0] else {
      panic!("expected dynamic page number text");
    };
    assert_eq!(run.dynamic_field, Some(DynamicFieldKind::Page));
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
      None,
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
      None,
    );

    let image = inlines
      .iter()
      .find_map(|item| match item {
        InlineItem::Image(image) => Some(image),
        InlineItem::Text(_) | InlineItem::PageBreak | InlineItem::ColumnBreak => None,
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
      None,
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

  #[test]
  fn style_chain_preserves_explicit_false_run_properties() {
    let mut catalog = StylesCatalog::default();
    catalog.styles.insert(
      "Base".into(),
      StyleEntry {
        style_type: Some(w::StyleValues::Paragraph),
        run_style: TextStyle {
          bold: true,
          italic: true,
          underline: true,
          ..Default::default()
        },
        ..Default::default()
      },
    );
    catalog.styles.insert(
      "Derived".into(),
      StyleEntry {
        style_type: Some(w::StyleValues::Paragraph),
        based_on: Some("Base".into()),
        run_overrides: RunStyleOverrides {
          bold: Some(false),
          underline: Some(false),
          ..Default::default()
        },
        ..Default::default()
      },
    );

    let style = catalog.run_style(Some("Derived"));

    assert!(!style.bold);
    assert!(style.italic);
    assert!(!style.underline);
  }

  #[test]
  fn body_sections_split_paragraph_and_body_section_properties() {
    let body = w::Body {
      body_choice: vec![
        w::BodyChoice::WP(Box::new(paragraph())),
        w::BodyChoice::WP(Box::new(paragraph_with_section(section(
          12240,
          15840,
          w::PageOrientationValues::Portrait,
          None,
        )))),
        w::BodyChoice::WP(Box::new(paragraph())),
      ],
      w_sect_pr: Some(Box::new(section(
        15840,
        12240,
        w::PageOrientationValues::Landscape,
        Some(w::SectionMarkValues::Continuous),
      ))),
      ..Default::default()
    };
    let mut numbering = NumberingCatalog::default();

    let sections = body_sections(
      &body,
      &StylesCatalog::default(),
      &mut numbering,
      &ImageCatalog::default(),
      &HyperlinkCatalog::default(),
    );

    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].blocks.len(), 2);
    assert_eq!(sections[0].break_kind, SectionBreakKind::NextPage);
    assert_eq!(sections[0].page.width_pt, 612.0);
    assert_eq!(sections[0].page.height_pt, 792.0);
    assert_eq!(sections[1].blocks.len(), 1);
    assert_eq!(sections[1].break_kind, SectionBreakKind::NextPage);
    assert_eq!(sections[1].page.width_pt, 792.0);
    assert_eq!(sections[1].page.height_pt, 612.0);
  }

  #[test]
  fn continuous_section_keeps_continuous_when_orientation_matches() {
    let previous = section(
      12240,
      15840,
      w::PageOrientationValues::Portrait,
      Some(w::SectionMarkValues::NextPage),
    );
    let current = section(
      12240,
      15840,
      w::PageOrientationValues::Portrait,
      Some(w::SectionMarkValues::Continuous),
    );

    assert_eq!(
      normalized_section_break(Some(&current), Some(&previous)),
      SectionBreakKind::Continuous
    );
  }

  #[test]
  fn next_column_section_normalizes_to_next_page_without_matching_columns() {
    let previous = section_with_columns(w::SectionMarkValues::NextPage, 2);
    let current = section_with_columns(w::SectionMarkValues::NextColumn, 1);

    assert_eq!(
      normalized_section_break(Some(&current), Some(&previous)),
      SectionBreakKind::NextPage
    );
  }

  #[test]
  fn next_column_section_uses_explicit_column_list_count() {
    let previous = explicit_columns_section(w::SectionMarkValues::NextPage);
    let current = explicit_columns_section(w::SectionMarkValues::NextColumn);

    assert_eq!(
      normalized_section_break(Some(&current), Some(&previous)),
      SectionBreakKind::NextColumn
    );
  }

  fn paragraph() -> w::Paragraph {
    w::Paragraph::default()
  }

  fn paragraph_with_section(section_properties: w::SectionProperties) -> w::Paragraph {
    w::Paragraph {
      paragraph_properties: Some(Box::new(w::ParagraphProperties {
        section_properties: Some(Box::new(section_properties)),
        ..Default::default()
      })),
      ..Default::default()
    }
  }

  fn section(
    width: u32,
    height: u32,
    orient: w::PageOrientationValues,
    break_type: Option<w::SectionMarkValues>,
  ) -> w::SectionProperties {
    w::SectionProperties {
      w_type: break_type.map(|val| w::SectionType { val }),
      w_pg_sz: Some(w::PageSize {
        width: Some(width),
        height: Some(height),
        orient: Some(orient),
        ..Default::default()
      }),
      ..Default::default()
    }
  }

  fn section_with_columns(
    break_type: w::SectionMarkValues,
    column_count: i16,
  ) -> w::SectionProperties {
    w::SectionProperties {
      w_type: Some(w::SectionType { val: break_type }),
      w_cols: Some(w::Columns {
        column_count: Some(column_count),
        ..Default::default()
      }),
      ..Default::default()
    }
  }

  fn explicit_columns_section(break_type: w::SectionMarkValues) -> w::SectionProperties {
    w::SectionProperties {
      w_type: Some(w::SectionType { val: break_type }),
      w_cols: Some(w::Columns {
        equal_width: Some(false),
        w_col: vec![
          w::Column {
            width: Some("1440".into()),
            space: Some("720".into()),
          },
          w::Column {
            width: Some("2880".into()),
            ..Default::default()
          },
        ],
        ..Default::default()
      }),
      ..Default::default()
    }
  }

  fn inline_text(inlines: &[InlineItem]) -> String {
    inlines
      .iter()
      .filter_map(|item| match item {
        InlineItem::Text(run) => Some(run.text.as_str()),
        InlineItem::Image(_) | InlineItem::PageBreak | InlineItem::ColumnBreak => None,
      })
      .collect()
  }
}
