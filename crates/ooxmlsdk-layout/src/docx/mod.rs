use std::borrow::Cow;

use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
use ooxmlsdk::schemas::w;
use ooxmlsdk_fonts::FontRequest;

use crate::common::{
  Fill, Insets, LayoutFontRequest, Pt, Rect, ScriptFontFamilies, Size, Stroke, Twips,
};

// Source: LibreOffice sw/source/writerfilter/dmapper/DomainMapper_Impl.cxx PageMar.
const DEFAULT_PAGE_MARGIN_TWIPS: i32 = 1440;
const DEFAULT_HEADER_FOOTER_TWIPS: i32 = 720;
// OOXML/Word fallback page size in twips used when sectPr omits pgSz.
const DEFAULT_PAGE_WIDTH_TWIPS: i32 = 11906;
const DEFAULT_PAGE_HEIGHT_TWIPS: i32 = 16838;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxDocument<'doc> {
  pub settings: DocxSettings,
  pub styles: DocxStyleCatalog<'doc>,
  pub numbering: NumberingCatalog<'doc>,
  pub resources: DocxResources<'doc>,
  pub sections: Vec<DocxSection<'doc>>,
  pub notes: NoteCatalog<'doc>,
  pub comments: Vec<DocxComment<'doc>>,
}

impl<'doc> DocxDocument<'doc> {
  pub fn from_wordprocessing_document(document: &'doc mut WordprocessingDocument) -> Self {
    let Ok(main_part) = document.main_document_part() else {
      return Self::default();
    };
    let main_part = main_part.clone();
    let Ok(root) = main_part.root_element(document) else {
      return Self::default();
    };
    import_document_root(root)
  }
}

fn import_document_root<'doc>(root: &w::Document) -> DocxDocument<'doc> {
  let mut document = DocxDocument::default();
  let Some(body) = root.body.as_deref() else {
    return document;
  };

  let mut section = DocxSection {
    page_desc: body
      .section_properties
      .as_deref()
      .map(import_section_page_desc)
      .unwrap_or_default(),
    ..DocxSection::default()
  };
  for choice in &body.body_choice {
    match choice {
      w::BodyChoice::Paragraph(paragraph) => section
        .body_blocks
        .push(DocxBlock::Paragraph(import_paragraph(paragraph))),
      w::BodyChoice::Table(table) => section
        .body_blocks
        .push(DocxBlock::Table(import_table(table))),
      w::BodyChoice::AltChunk(_) => {
        section
          .body_blocks
          .push(DocxBlock::FloatingFrame(FloatingFrame {
            placement: FloatingPlacement::Inline,
            wrap: WrapMode::None,
            ..FloatingFrame::default()
          }))
      }
      _ => {}
    }
  }
  document.sections.push(section);
  document
}

fn import_section_page_desc(properties: &w::SectionProperties) -> PageDesc {
  let page_size = properties
    .page_size
    .as_ref()
    .map(|page_size| Size {
      width: Twips(
        page_size
          .width
          .map(|value| value.to_twips())
          .unwrap_or(DEFAULT_PAGE_WIDTH_TWIPS as i64) as i32,
      )
      .to_pt(),
      height: Twips(
        page_size
          .height
          .map(|value| value.to_twips())
          .unwrap_or(DEFAULT_PAGE_HEIGHT_TWIPS as i64) as i32,
      )
      .to_pt(),
    })
    .unwrap_or(Size {
      width: Twips(DEFAULT_PAGE_WIDTH_TWIPS).to_pt(),
      height: Twips(DEFAULT_PAGE_HEIGHT_TWIPS).to_pt(),
    });
  let margins = properties
    .page_margin
    .as_ref()
    .map(|margin| Insets {
      top: Twips(
        margin
          .top
          .map(|value| value.to_twips())
          .unwrap_or(DEFAULT_PAGE_MARGIN_TWIPS as i64) as i32,
      )
      .to_pt(),
      right: Twips(
        margin
          .right
          .map(|value| value.to_twips())
          .unwrap_or(DEFAULT_PAGE_MARGIN_TWIPS as i64) as i32,
      )
      .to_pt(),
      bottom: Twips(
        margin
          .bottom
          .map(|value| value.to_twips())
          .unwrap_or(DEFAULT_PAGE_MARGIN_TWIPS as i64) as i32,
      )
      .to_pt(),
      left: Twips(
        margin
          .left
          .map(|value| value.to_twips())
          .unwrap_or(DEFAULT_PAGE_MARGIN_TWIPS as i64) as i32,
      )
      .to_pt(),
    })
    .unwrap_or(Insets {
      top: Twips(DEFAULT_PAGE_MARGIN_TWIPS).to_pt(),
      right: Twips(DEFAULT_PAGE_MARGIN_TWIPS).to_pt(),
      bottom: Twips(DEFAULT_PAGE_MARGIN_TWIPS).to_pt(),
      left: Twips(DEFAULT_PAGE_MARGIN_TWIPS).to_pt(),
    });
  PageDesc {
    page_size,
    margins,
    header_distance: properties
      .page_margin
      .as_ref()
      .and_then(|margin| margin.header)
      .map(|value| Twips(value.to_twips() as i32).to_pt())
      .unwrap_or_else(|| Twips(DEFAULT_HEADER_FOOTER_TWIPS).to_pt()),
    footer_distance: properties
      .page_margin
      .as_ref()
      .and_then(|margin| margin.footer)
      .map(|value| Twips(value.to_twips() as i32).to_pt())
      .unwrap_or_else(|| Twips(DEFAULT_HEADER_FOOTER_TWIPS).to_pt()),
  }
}

fn import_paragraph<'doc>(paragraph: &w::Paragraph) -> DocxParagraph<'doc> {
  let mut imported = DocxParagraph::default();
  for choice in &paragraph.paragraph_choice {
    match choice {
      w::ParagraphChoice::WRun(run) => imported.inlines.extend(import_run(run)),
      w::ParagraphChoice::SimpleField(field) => {
        imported.inlines.push(InlineItem::Field(FieldRun {
          instruction: Cow::Owned(field.instruction.clone()),
          display_text: Cow::Owned(simple_field_text(field)),
        }))
      }
      w::ParagraphChoice::Hyperlink(hyperlink) => {
        imported.inlines.push(InlineItem::HyperlinkStart(Hyperlink {
          relationship_id: hyperlink.id.clone().map(Cow::Owned),
          anchor: hyperlink.anchor.clone().map(Cow::Owned),
          tooltip: hyperlink.tooltip.clone().map(Cow::Owned),
        }));
        for child in &hyperlink.hyperlink_choice {
          if let w::HyperlinkChoice::WRun(run) = child {
            imported.inlines.extend(import_run(run));
          }
        }
        imported.inlines.push(InlineItem::HyperlinkEnd);
      }
      w::ParagraphChoice::BookmarkStart(bookmark) => {
        imported.inlines.push(InlineItem::BookmarkStart(Bookmark {
          id: bookmark.id.to_string().into(),
          name: Cow::Owned(bookmark.name.clone()),
        }));
      }
      w::ParagraphChoice::BookmarkEnd(bookmark) => {
        imported
          .inlines
          .push(InlineItem::BookmarkEnd(Cow::Owned(bookmark.id.to_string())));
      }
      w::ParagraphChoice::CommentRangeStart(comment) => {
        imported
          .inlines
          .push(InlineItem::CommentRangeStart(Cow::Owned(
            comment.id.to_string(),
          )))
      }
      w::ParagraphChoice::CommentRangeEnd(comment) => {
        imported
          .inlines
          .push(InlineItem::CommentRangeEnd(Cow::Owned(
            comment.id.to_string(),
          )))
      }
      _ => {}
    }
  }
  imported
}

fn import_run<'doc>(run: &w::Run) -> Vec<InlineItem<'doc>> {
  let style = run
    .run_properties
    .as_deref()
    .map(import_run_style)
    .unwrap_or_default();
  let mut inlines = Vec::new();
  for choice in &run.run_choice {
    match choice {
      w::RunChoice::Text(text) => inlines.push(InlineItem::Text(DocxTextRun {
        text: Cow::Owned(text.0.xml_content.clone().unwrap_or_default()),
        style: style.clone(),
      })),
      w::RunChoice::FieldCode(code) => inlines.push(InlineItem::Field(FieldRun {
        instruction: Cow::Owned(code.xml_content.clone().unwrap_or_default()),
        display_text: Cow::Borrowed(""),
      })),
      w::RunChoice::Break(break_value) => match break_value.r#type.unwrap_or_default() {
        w::BreakValues::Page => inlines.push(InlineItem::PageBreak),
        w::BreakValues::Column => inlines.push(InlineItem::ColumnBreak),
        w::BreakValues::TextWrapping => inlines.push(InlineItem::Text(DocxTextRun {
          text: Cow::Borrowed("\n"),
          style: style.clone(),
        })),
      },
      w::RunChoice::TabChar => inlines.push(InlineItem::Text(DocxTextRun {
        text: Cow::Borrowed("\t"),
        style: style.clone(),
      })),
      w::RunChoice::CarriageReturn => inlines.push(InlineItem::Text(DocxTextRun {
        text: Cow::Borrowed("\n"),
        style: style.clone(),
      })),
      w::RunChoice::FootnoteReference(reference) => {
        inlines.push(InlineItem::FootnoteReference(reference.id));
      }
      w::RunChoice::EndnoteReference(reference) => {
        inlines.push(InlineItem::EndnoteReference(reference.id));
      }
      w::RunChoice::LastRenderedPageBreak => inlines.push(InlineItem::LastRenderedPageBreak),
      w::RunChoice::Drawing(_) | w::RunChoice::Picture(_) | w::RunChoice::EmbeddedObject(_) => {
        inlines.push(InlineItem::InlineShape(InlineShape::default()));
      }
      _ => {}
    }
  }
  inlines
}

fn import_run_style<'doc>(properties: &w::RunProperties) -> TextStyle<'doc> {
  let mut style = TextStyle::default();
  for choice in &properties.run_properties_choice {
    match choice {
      w::RunPropertiesChoice::RunFonts(fonts) => {
        style.font_families = Box::new(ScriptFontFamilies {
          latin: fonts.ascii.clone().map(Cow::Owned),
          high_ansi: fonts.high_ansi.clone().map(Cow::Owned),
          east_asian: fonts.east_asia.clone().map(Cow::Owned),
          complex_script: fonts.complex_script.clone().map(Cow::Owned),
          ..ScriptFontFamilies::default()
        });
        style.font.family = style
          .font_families
          .high_ansi
          .clone()
          .or_else(|| style.font_families.latin.clone())
          .or_else(|| style.font_families.east_asian.clone())
          .or_else(|| style.font_families.complex_script.clone());
      }
      w::RunPropertiesChoice::Bold(value) => {
        style.bold = value.val.map(|value| value.as_bool()).unwrap_or(true);
        style.font.bold = style.bold;
      }
      w::RunPropertiesChoice::Italic(value) => {
        style.italic = value.val.map(|value| value.as_bool()).unwrap_or(true);
        style.font.italic = style.italic;
      }
      w::RunPropertiesChoice::Underline(_) => style.underline = true,
      w::RunPropertiesChoice::Strike(value) => {
        style.strikeout = value.val.map(|value| value.as_bool()).unwrap_or(true);
      }
      w::RunPropertiesChoice::SmallCaps(value) => {
        style.small_caps = value.val.map(|value| value.as_bool()).unwrap_or(true);
      }
      w::RunPropertiesChoice::Caps(value) => {
        style.all_caps = value.val.map(|value| value.as_bool()).unwrap_or(true);
      }
      w::RunPropertiesChoice::FontSize(size) => {
        style.font.size_pt = ooxmlsdk_fonts::FontSize(
          size
            .val
            .parse::<f32>()
            .map(|value| value / 2.0)
            .unwrap_or(11.0),
        );
      }
      _ => {}
    }
  }
  style
}

fn simple_field_text(field: &w::SimpleField) -> String {
  let mut text = String::new();
  for choice in &field.simple_field_choice {
    if let w::SimpleFieldChoice::WRun(run) = choice {
      for inline in import_run(run) {
        if let InlineItem::Text(run) = inline {
          text.push_str(&run.text);
        }
      }
    }
  }
  text
}

fn import_table<'doc>(table: &w::Table) -> DocxTable<'doc> {
  let mut imported = DocxTable::default();
  for choice in &table.table_choice2 {
    if let w::TableChoice2::TableRow(row) = choice {
      imported.rows.push(import_table_row(row));
    }
  }
  imported
}

fn import_table_row<'doc>(row: &w::TableRow) -> DocxTableRow<'doc> {
  let mut imported = DocxTableRow::default();
  for choice in &row.table_row_choice {
    if let w::TableRowChoice::TableCell(cell) = choice {
      imported.cells.push(import_table_cell(cell));
    }
  }
  imported
}

fn import_table_cell<'doc>(cell: &w::TableCell) -> DocxTableCell<'doc> {
  let mut imported = DocxTableCell::default();
  for choice in &cell.table_cell_choice {
    match choice {
      w::TableCellChoice::Paragraph(paragraph) => imported
        .blocks
        .push(DocxBlock::Paragraph(import_paragraph(paragraph))),
      w::TableCellChoice::Table(table) => {
        imported.blocks.push(DocxBlock::Table(import_table(table)))
      }
      _ => {}
    }
  }
  imported
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxSettings {
  pub default_tab_stop: Pt,
  pub compatibility_mode: Option<u16>,
  pub even_and_odd_headers: bool,
  pub split_page_break_and_paragraph_mark: bool,
  pub hyphenation: HyphenationSettings,
  pub document_grid: Option<DocumentGrid>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HyphenationSettings {
  pub enabled: bool,
  pub zone: Pt,
  pub consecutive_limit: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DocumentGrid {
  pub line_pitch: Pt,
  pub char_space: Option<Pt>,
  pub snap_to_chars: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxStyleCatalog<'doc> {
  pub paragraph_styles: Vec<DocxStyle<'doc>>,
  pub character_styles: Vec<DocxStyle<'doc>>,
  pub table_styles: Vec<DocxStyle<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxStyle<'doc> {
  pub id: Cow<'doc, str>,
  pub name: Option<Cow<'doc, str>>,
  pub based_on: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NumberingCatalog<'doc> {
  pub definitions: Vec<NumberingDefinition<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NumberingDefinition<'doc> {
  pub id: Cow<'doc, str>,
  pub levels: Vec<NumberingLevel<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NumberingLevel<'doc> {
  pub level: u8,
  pub format: Option<Cow<'doc, str>>,
  pub text: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxResources<'doc> {
  pub relationships: Vec<DocxRelationship<'doc>>,
  pub images: Vec<DocxBinaryResource<'doc>>,
  pub embedded_objects: Vec<DocxBinaryResource<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NoteCatalog<'doc> {
  pub footnotes: Vec<DocxNote<'doc>>,
  pub endnotes: Vec<DocxNote<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxNote<'doc> {
  pub id: i64,
  pub kind: NoteKind,
  pub blocks: Vec<DocxBlock<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum NoteKind {
  #[default]
  Normal,
  Separator,
  ContinuationSeparator,
  ContinuationNotice,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxComment<'doc> {
  pub id: Cow<'doc, str>,
  pub author: Option<Cow<'doc, str>>,
  pub blocks: Vec<DocxBlock<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocxRelationship<'doc> {
  pub id: Cow<'doc, str>,
  pub relationship_type: Cow<'doc, str>,
  pub target: Cow<'doc, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DocxBinaryResource<'doc> {
  pub relationship_id: Option<Cow<'doc, str>>,
  pub content_type: Cow<'doc, str>,
  pub bytes: Cow<'doc, [u8]>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxSection<'doc> {
  pub break_kind: SectionBreakKind,
  pub page_desc: PageDesc,
  pub columns: SectionColumns,
  pub headers: Vec<HeaderFooter<'doc>>,
  pub footers: Vec<HeaderFooter<'doc>>,
  pub body_blocks: Vec<DocxBlock<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SectionBreakKind {
  Continuous,
  #[default]
  NextPage,
  NextColumn,
  EvenPage,
  OddPage,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PageDesc {
  pub page_size: crate::common::Size,
  pub margins: Insets,
  pub header_distance: Pt,
  pub footer_distance: Pt,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SectionColumns {
  pub count: usize,
  pub gap: Pt,
  pub separator: bool,
  pub unbalanced: bool,
  pub explicit_columns: Vec<SectionColumn>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SectionColumn {
  pub width: Pt,
  pub gap: Pt,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeaderFooter<'doc> {
  pub kind: HeaderFooterKind,
  pub blocks: Vec<DocxBlock<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum HeaderFooterKind {
  #[default]
  Default,
  First,
  Even,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DocxBlock<'doc> {
  Paragraph(DocxParagraph<'doc>),
  Table(DocxTable<'doc>),
  FloatingFrame(FloatingFrame<'doc>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxParagraph<'doc> {
  pub inlines: Vec<InlineItem<'doc>>,
  pub base_style: TextStyle<'doc>,
  pub format: ParagraphFormat<'doc>,
  pub style_ref: Option<Cow<'doc, str>>,
  pub list_label: Option<Cow<'doc, str>>,
  pub outline_level: Option<u8>,
  pub bookmarks: Vec<Bookmark<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Bookmark<'doc> {
  pub id: Cow<'doc, str>,
  pub name: Cow<'doc, str>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InlineItem<'doc> {
  Text(DocxTextRun<'doc>),
  Field(FieldRun<'doc>),
  InlineShape(InlineShape<'doc>),
  BookmarkStart(Bookmark<'doc>),
  BookmarkEnd(Cow<'doc, str>),
  CommentRangeStart(Cow<'doc, str>),
  CommentRangeEnd(Cow<'doc, str>),
  HyperlinkStart(Hyperlink<'doc>),
  HyperlinkEnd,
  FootnoteReference(i64),
  EndnoteReference(i64),
  PageBreak,
  ColumnBreak,
  LastRenderedPageBreak,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hyperlink<'doc> {
  pub relationship_id: Option<Cow<'doc, str>>,
  pub anchor: Option<Cow<'doc, str>>,
  pub tooltip: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxTextRun<'doc> {
  pub text: Cow<'doc, str>,
  pub style: TextStyle<'doc>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldRun<'doc> {
  pub instruction: Cow<'doc, str>,
  pub display_text: Cow<'doc, str>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TextStyle<'doc> {
  pub font: FontRequest<'doc>,
  pub font_families: Box<ScriptFontFamilies<'doc>>,
  pub color: crate::common::Color,
  pub highlight: Option<crate::common::Color>,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
  pub strikeout: bool,
  pub small_caps: bool,
  pub all_caps: bool,
  pub character_spacing: Pt,
  pub baseline_shift: Pt,
}

impl<'doc> TextStyle<'doc> {
  pub fn layout_font_request(&self) -> LayoutFontRequest<'doc> {
    LayoutFontRequest {
      base: self.font.clone(),
      families: (*self.font_families).clone(),
      small_caps: self.small_caps,
      character_spacing: self.character_spacing,
    }
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ParagraphFormat<'doc> {
  pub style_id: Option<Cow<'doc, str>>,
  pub alignment: ParagraphAlignment,
  pub margins: Insets,
  pub line_height: LineHeight,
  pub keep_with_next: bool,
  pub keep_together: bool,
  pub widow_control: bool,
  pub page_break_before: bool,
  pub tabs: Vec<TabStop>,
  pub text_direction: TextDirection,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TabStop {
  pub position: Pt,
  pub alignment: TabAlignment,
  pub leader: TabLeader,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabAlignment {
  #[default]
  Left,
  Center,
  Right,
  Decimal,
  Bar,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TabLeader {
  #[default]
  None,
  Dot,
  Hyphen,
  Underscore,
  MiddleDot,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextDirection {
  #[default]
  LeftToRightTopToBottom,
  TopToBottomRightToLeft,
  BottomToTopLeftToRight,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ParagraphAlignment {
  #[default]
  Left,
  Center,
  Right,
  Justify,
  Distributed,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LineHeight {
  pub value: Option<Pt>,
  pub rule: LineHeightRule,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LineHeightRule {
  #[default]
  Auto,
  AtLeast,
  Exact,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxTable<'doc> {
  pub rows: Vec<DocxTableRow<'doc>>,
  pub grid: Vec<Pt>,
  pub preferred_width: Option<Pt>,
  pub indent_left: Pt,
  pub alignment: TableAlignment,
  pub borders: TableBorders<'doc>,
  pub split_allowed: bool,
  pub floating: Option<FloatingPlacement<'doc>>,
  pub cell_spacing: Pt,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxTableRow<'doc> {
  pub cells: Vec<DocxTableCell<'doc>>,
  pub height: Option<Pt>,
  pub exact_height: bool,
  pub repeat_header: bool,
  pub keep_with_next: bool,
  pub cant_split: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxTableCell<'doc> {
  pub blocks: Vec<DocxBlock<'doc>>,
  pub margins: Insets,
  pub preferred_width: Option<Pt>,
  pub grid_span: usize,
  pub vertical_merge_continue: bool,
  pub vertical_alignment: VerticalAlignment,
  pub fill: Fill<'doc>,
  pub borders: TableBorders<'doc>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TableAlignment {
  #[default]
  Left,
  Center,
  Right,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum VerticalAlignment {
  #[default]
  Top,
  Center,
  Bottom,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TableBorders<'doc> {
  pub top: Option<Stroke<'doc>>,
  pub right: Option<Stroke<'doc>>,
  pub bottom: Option<Stroke<'doc>>,
  pub left: Option<Stroke<'doc>>,
  pub inside_horizontal: Option<Stroke<'doc>>,
  pub inside_vertical: Option<Stroke<'doc>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FloatingFrame<'doc> {
  pub blocks: Vec<DocxBlock<'doc>>,
  pub bounds: Rect,
  pub placement: FloatingPlacement<'doc>,
  pub wrap: WrapMode,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum FloatingPlacement<'doc> {
  #[default]
  Inline,
  Anchored(AnchorPosition<'doc>),
  Absolute(AnchorPosition<'doc>),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AnchorPosition<'doc> {
  pub horizontal_anchor: AnchorReference,
  pub vertical_anchor: AnchorReference,
  pub horizontal_alignment: Option<AnchorAlignment>,
  pub vertical_alignment: Option<AnchorAlignment>,
  pub offset_x: Pt,
  pub offset_y: Pt,
  pub relative_from: Option<Cow<'doc, str>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AnchorReference {
  #[default]
  Page,
  Margin,
  Column,
  Paragraph,
  Character,
  Line,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnchorAlignment {
  Left,
  Center,
  Right,
  Top,
  Middle,
  Bottom,
  Inside,
  Outside,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum WrapMode {
  #[default]
  None,
  Square,
  Tight,
  Through,
  TopAndBottom,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct InlineShape<'doc> {
  pub relationship_id: Option<Cow<'doc, str>>,
  pub bounds: Rect,
  pub alt_text: Option<Cow<'doc, str>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrameFollow {
  pub master_frame: crate::common::FrameId,
  pub follow_frame: crate::common::FrameId,
  pub reason: FollowReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FollowReason {
  PageBreak,
  ColumnBreak,
  TableSplit,
  FootnoteContinuation,
  Overflow,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxFrameTree<'doc> {
  pub root: DocxFrame<'doc>,
  pub follows: Vec<FrameFollow>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxFrame<'doc> {
  pub id: crate::common::FrameId,
  pub kind: DocxFrameKind,
  pub bounds: Rect,
  pub print_bounds: Rect,
  pub children: Vec<DocxFrame<'doc>>,
  pub text_lines: Vec<DocxTextLine<'doc>>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum DocxFrameKind {
  #[default]
  Root,
  Page,
  Header,
  Body,
  Footer,
  Footnote,
  Section,
  Column,
  Text,
  Table,
  Row,
  Cell,
  Fly,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DocxTextLine<'doc> {
  pub text_range: std::ops::Range<usize>,
  pub bounds: Rect,
  pub baseline: Pt,
  pub portions: Vec<DocxTextPortion<'doc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DocxTextPortion<'doc> {
  Text(DocxTextRun<'doc>),
  Field(FieldRun<'doc>),
  Tab,
  Numbering(Cow<'doc, str>),
  Bullet(Cow<'doc, str>),
  SoftHyphen,
  Hidden,
  Bookmark,
  Comment,
  ControlChar,
  Combined,
  Ruby,
  Break,
  Footnote,
  Fly,
}
