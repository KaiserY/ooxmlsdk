use std::collections::BTreeMap;
use std::sync::Arc;

use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

pub(crate) use crate::model::{
  BorderStyle, CellBordersModel, DynamicFieldKind, FieldNumberFormat, FormWidget, FormWidgetKind,
  ImageCrop, LineNumbering, PageSetup, RgbColor, TextStyle,
};
use crate::{common, units};

#[derive(Clone, Debug)]
pub(crate) struct DocxDocument {
  pub page: PageSetup,
  pub line_number_style: TextStyle,
  pub has_styles_part: bool,
  pub default_tab_stop_pt: f32,
  pub compatibility_mode: u16,
  pub justify_lines_with_shrinking: bool,
  pub even_and_odd_headers: bool,
  pub split_page_break_and_paragraph_mark: bool,
  pub form_widgets: Vec<FormWidget>,
  pub sections: Vec<ImportedSection>,
  pub header_blocks: Vec<Block>,
  pub footer_blocks: Vec<Block>,
  pub first_header_blocks: Vec<Block>,
  pub first_footer_blocks: Vec<Block>,
  pub footnote_blocks: Vec<Block>,
  pub footnotes: BTreeMap<i64, Vec<Block>>,
  pub footnote_numbering: Vec<NoteNumberingSpec>,
  pub endnote_blocks: Vec<Block>,
  pub endnotes: BTreeMap<i64, Vec<Block>>,
  pub endnote_numbering: Vec<NoteNumberingSpec>,
  pub title_page: bool,
  pub blocks: Vec<Block>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NoteNumberingSpec {
  pub format: w::NumberFormatValues,
  pub start: i32,
  pub restart: w::RestartNumberValues,
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
  pub unbalanced: bool,
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
      unbalanced: false,
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
  Paragraph(Box<Paragraph>),
  Table(Table),
  Frame(FloatingFrame),
}

impl Block {
  pub(crate) fn paragraph(paragraph: Paragraph) -> Self {
    Self::Paragraph(Box::new(paragraph))
  }
}

#[derive(Clone, Debug)]
pub(crate) struct FloatingFrame {
  pub blocks: Vec<Block>,
  pub width_pt: Option<f32>,
  pub height_pt: Option<f32>,
  pub height_rule: FrameHeightRule,
  pub placement: FloatingFramePlacement,
  pub fill_color: Option<RgbColor>,
  pub borders: CellBordersModel,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FrameHeightRule {
  #[default]
  Auto,
  AtLeast,
  Exact,
}

#[derive(Clone, Debug)]
pub(crate) struct Paragraph {
  pub inlines: Vec<InlineItem>,
  pub footnote_reference_ids: Vec<i64>,
  pub endnote_reference_ids: Vec<i64>,
  pub starts_after_last_rendered_page_break: bool,
  pub base_style: TextStyle,
  #[cfg(test)]
  pub runs: Vec<TextRun>,
  pub format: Box<ParagraphFormat>,
  pub style_ref_keys: Vec<Arc<str>>,
  pub style_ref_text: Option<Arc<str>>,
  pub style_ref_numbering_text: Option<Arc<str>>,
  pub list_label: Option<String>,
  pub list_label_style: TextStyle,
  pub list_label_hyperlink_url: Option<String>,
  pub list_label_tab_stop_pt: Option<f32>,
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
  pub column_widths_pt: Vec<f32>,
  pub preferred_width_pt: Option<f32>,
  pub preferred_width_pct: Option<f32>,
  pub indent_left_pt: f32,
  pub alignment: TableAlignment,
  pub align_leading_cell_content: bool,
  pub placement: Option<FloatingFramePlacement>,
  pub split_allowed: bool,
  pub following_text_flow: bool,
  pub explicit_no_repeat_header: bool,
  pub starts_after_last_rendered_page_break: bool,
  pub borders: Option<TableBordersModel>,
  pub cell_spacing_pt: f32,
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
  pub keep_with_next: bool,
  pub cant_split: bool,
  pub cell_spacing_pt: Option<f32>,
  pub grid_before: usize,
  pub grid_after: usize,
  pub redline_color: Option<RgbColor>,
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
  pub text_rotation_deg: Option<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct CellMargins {
  pub top_pt: f32,
  pub right_pt: f32,
  pub bottom_pt: f32,
  pub left_pt: f32,
}

// ECMA-376 Part 1, 17.4.11 and 17.4.34 specify 115 twentieths of a
// point when the style hierarchy supplies no trailing/leading cell margin.
const DEFAULT_TABLE_CELL_SIDE_MARGIN_TWIPS: f32 = 115.0;

impl Default for CellMargins {
  fn default() -> Self {
    Self {
      top_pt: 0.0,
      right_pt: units::twips_to_points(DEFAULT_TABLE_CELL_SIDE_MARGIN_TWIPS),
      bottom_pt: 0.0,
      left_pt: units::twips_to_points(DEFAULT_TABLE_CELL_SIDE_MARGIN_TWIPS),
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

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ParagraphFormat {
  pub style_id: Option<Arc<str>>,
  pub spacing_before_pt: f32,
  pub spacing_after_pt: f32,
  pub spacing_before_set: bool,
  pub spacing_after_set: bool,
  pub line_height_pt: Option<f32>,
  pub line_height_rule: LineHeightRule,
  pub snap_to_grid: Option<bool>,
  pub line_vertical_alignment: Option<common::LineVerticalAlignment>,
  pub indent_left_pt: f32,
  pub indent_right_pt: f32,
  pub first_line_indent_pt: f32,
  pub indent_left_character_units: Option<f32>,
  pub indent_right_character_units: Option<f32>,
  pub first_line_indent_character_units: Option<f32>,
  pub indent_left_set: bool,
  pub indent_right_set: bool,
  pub first_line_indent_set: bool,
  pub tab_stops: Vec<TabStop>,
  pub tab_stops_set: bool,
  pub list_label_width_aware_tab: bool,
  pub list_label_uses_explicit_tab_stop: bool,
  pub alignment: ParagraphAlignment,
  pub justification: ParagraphJustification,
  pub bidi: bool,
  pub shading: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub page_break_before: bool,
  pub page_break_before_set: bool,
  pub keep_with_next: bool,
  pub keep_with_next_set: bool,
  pub keep_lines: bool,
  pub keep_lines_set: bool,
  pub contextual_spacing: bool,
  pub contextual_spacing_set: bool,
  pub hidden_separator: bool,
  pub outline_text_inlines: Option<usize>,
  pub outline_level: Option<u8>,
  pub frame: Option<ParagraphFrameProperties>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct ParagraphFrameProperties {
  pub width_pt: Option<f32>,
  pub height_pt: Option<f32>,
  pub height_rule: FrameHeightRule,
  pub placement: FloatingFramePlacement,
  pub drop_cap: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FloatingFramePlacement {
  pub horizontal_anchor: FrameHorizontalAnchor,
  pub vertical_anchor: FrameVerticalAnchor,
  pub horizontal_alignment: Option<FrameHorizontalAlignment>,
  pub vertical_alignment: Option<FrameVerticalAlignment>,
  pub horizontal_offset_pt: f32,
  pub vertical_offset_pt: f32,
  pub vertical_offset_explicit: bool,
  pub wrap: FrameWrapMode,
  pub margin_top_pt: f32,
  pub margin_right_pt: f32,
  pub margin_bottom_pt: f32,
  pub margin_left_pt: f32,
}

impl Default for FloatingFramePlacement {
  fn default() -> Self {
    Self {
      horizontal_anchor: FrameHorizontalAnchor::Text,
      vertical_anchor: FrameVerticalAnchor::Text,
      horizontal_alignment: None,
      vertical_alignment: None,
      horizontal_offset_pt: 0.0,
      vertical_offset_pt: 0.0,
      vertical_offset_explicit: false,
      wrap: FrameWrapMode::Around,
      margin_top_pt: 0.0,
      margin_right_pt: 0.0,
      margin_bottom_pt: 0.0,
      margin_left_pt: 0.0,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FrameHorizontalAnchor {
  #[default]
  Text,
  Margin,
  Page,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FrameVerticalAnchor {
  #[default]
  Text,
  Margin,
  Page,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FrameHorizontalAlignment {
  Left,
  Center,
  Right,
  Inside,
  Outside,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FrameVerticalAlignment {
  Inline,
  Top,
  Center,
  Bottom,
  Inside,
  Outside,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FrameWrapMode {
  #[default]
  Auto,
  Around,
  Tight,
  Through,
  None,
  NotBeside,
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
  pub leader: TabLeader,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TabStopAlignment {
  #[default]
  Left,
  Center,
  Right,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TabLeader {
  #[default]
  None,
  Dot,
  Hyphen,
  Underscore,
  Heavy,
  MiddleDot,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ParagraphAlignment {
  #[default]
  Left,
  Center,
  Right,
  Justify,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ParagraphJustification {
  pub adjust: ParagraphAdjust,
  pub one_word_adjust: ParagraphAdjust,
  pub last_line_adjust: ParagraphAdjust,
  pub word_spacing: JustificationWordSpacing,
  pub letter_spacing_minimum_pct: i16,
  pub letter_spacing_maximum_pct: i16,
  pub scale_width_minimum_pct: i16,
  pub scale_width_maximum_pct: i16,
  pub paragraph_composer: bool,
}

impl Default for ParagraphJustification {
  fn default() -> Self {
    Self {
      adjust: ParagraphAdjust::Left,
      one_word_adjust: ParagraphAdjust::Left,
      last_line_adjust: ParagraphAdjust::Left,
      word_spacing: JustificationWordSpacing::default(),
      letter_spacing_minimum_pct: 0,
      letter_spacing_maximum_pct: 0,
      scale_width_minimum_pct: 100,
      scale_width_maximum_pct: 100,
      paragraph_composer: false,
    }
  }
}

impl ParagraphJustification {
  pub(crate) fn alignment(self) -> ParagraphAlignment {
    match self.adjust {
      ParagraphAdjust::Center => ParagraphAlignment::Center,
      ParagraphAdjust::Right | ParagraphAdjust::End => ParagraphAlignment::Right,
      ParagraphAdjust::Block => ParagraphAlignment::Justify,
      ParagraphAdjust::Left | ParagraphAdjust::Start => ParagraphAlignment::Left,
    }
  }

  pub(crate) fn is_block(self) -> bool {
    self.adjust == ParagraphAdjust::Block
  }

  pub(crate) fn can_shrink_word_spacing(self) -> bool {
    self.word_spacing.minimum_pct < self.word_spacing.desired_pct
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct JustificationWordSpacing {
  pub desired_pct: u16,
  pub minimum_pct: u16,
  pub maximum_pct: u16,
}

impl Default for JustificationWordSpacing {
  fn default() -> Self {
    Self {
      desired_pct: 100,
      minimum_pct: 100,
      maximum_pct: 100,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum ParagraphAdjust {
  #[default]
  Left,
  Right,
  Center,
  Block,
  Start,
  End,
}

#[derive(Clone, Debug)]
pub(crate) struct TextRun {
  pub text: String,
  pub style: TextStyle,
  pub hyperlink_url: Option<String>,
  pub dynamic_field: Option<DynamicFieldKind>,
  pub style_ref_keys: Vec<Arc<str>>,
  pub style_ref_text: Option<Arc<str>>,
  pub style_ref_numbering_text: Option<Arc<str>>,
  pub preserve_text_portion: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct FormWidgetIdAllocator {
  next_id: u32,
  widgets: Vec<FormWidget>,
}

impl FormWidgetIdAllocator {
  pub(crate) fn next_widget(&mut self, kind: FormWidgetKind, entries: Vec<String>) -> u32 {
    let id = self.next_id;
    self.next_id = self.next_id.saturating_add(1);
    self.widgets.push(FormWidget { id, kind, entries });
    id
  }

  pub(crate) fn into_widgets(self) -> Vec<FormWidget> {
    self.widgets
  }
}

#[derive(Clone, Debug)]
pub(crate) enum InlineItem {
  Text(TextRun),
  Image(InlineImage),
  Shape(InlineShape),
  BookmarkStart(String),
  FormWidgetStart(u32),
  FormWidgetEnd(u32),
  LastRenderedPageBreak,
  PageBreak,
  ColumnBreak,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineImage {
  pub data: Arc<[u8]>,
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
  pub hyperlink_url: Option<String>,
  pub semantic_metafile_text: bool,
  pub placement: ImagePlacement,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineShape {
  pub width_pt: f32,
  pub height_pt: f32,
  pub effect_left_pt: f32,
  pub effect_top_pt: f32,
  pub effect_right_pt: f32,
  pub effect_bottom_pt: f32,
  pub geometry: InlineShapeGeometry,
  pub offset_x_pt: f32,
  pub offset_y_pt: f32,
  pub fill_color: Option<RgbColor>,
  pub additional_fill_colors: Vec<RgbColor>,
  pub fill_image: Option<InlineShapeImageFill>,
  pub stroke: Option<BorderStyle>,
  pub suppress_zero_relative_background: bool,
  pub allow_outside_page: bool,
  pub inline_anchor_after_line: bool,
  pub placement: ImagePlacement,
  pub chart: Option<Box<InlineChart>>,
  pub text_box_blocks: Vec<Block>,
  pub text_inset_left_pt: f32,
  pub text_inset_top_pt: f32,
  pub text_inset_right_pt: f32,
  pub text_inset_bottom_pt: f32,
  pub text_box_auto_fit: bool,
  pub text_vertical_alignment: TextBoxVerticalAlignment,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineChart {
  pub chart_space: Box<c::ChartSpace>,
  pub ui_language: Option<String>,
  pub automatic_title: String,
  pub title_style: TextStyle,
  pub label_style: TextStyle,
  pub gridline_color: RgbColor,
  pub series_colors: Vec<RgbColor>,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineShapeImageFill {
  pub data: Arc<[u8]>,
  pub content_type: Option<String>,
  pub crop: ImageCrop,
  pub rotation_deg: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum InlineShapeGeometry {
  Rectangle,
  Line,
  Polyline {
    points: Vec<(f32, f32)>,
    closed: bool,
  },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TextBoxVerticalAlignment {
  #[default]
  Top,
  Center,
  Bottom,
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
  pub layout_in_cell: bool,
  pub allow_overlap: bool,
  pub relative_height: u32,
  pub relative_width_to: Option<HorizontalImageReference>,
  pub relative_width_pct: Option<f32>,
  pub relative_height_to: Option<VerticalImageReference>,
  pub relative_height_pct: Option<f32>,
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
  Inside,
  Outside,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum VerticalImageAlignment {
  Top,
  Center,
  Bottom,
  Inside,
  Outside,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum HorizontalImageReference {
  Page,
  #[default]
  Margin,
  Column,
  Character,
  LeftMargin,
  RightMargin,
  InsideMargin,
  OutsideMargin,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum VerticalImageReference {
  Page,
  #[default]
  Margin,
  Paragraph,
  Line,
  TopMargin,
  BottomMargin,
  InsideMargin,
  OutsideMargin,
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
