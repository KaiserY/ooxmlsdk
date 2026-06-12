use std::borrow::Cow;

use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
use ooxmlsdk_fonts::FontRequest;

use crate::common::{Fill, Insets, Pt, Rect, Stroke};

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
  pub fn from_wordprocessing_document(_document: &'doc WordprocessingDocument) -> Self {
    Self::default()
  }
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
