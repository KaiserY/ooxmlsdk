use std::collections::BTreeMap;
use std::sync::Arc;

use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

use super::units;

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
  pub list_label_hyperlink_url: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
  pub column_widths_pt: Vec<f32>,
  pub preferred_width_pt: Option<f32>,
  pub preferred_width_pct: Option<f32>,
  pub indent_left_pt: f32,
  pub alignment: TableAlignment,
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
  pub cant_split: bool,
  pub cell_spacing_pt: Option<f32>,
  pub grid_before: usize,
  pub grid_after: usize,
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

const DEFAULT_TABLE_CELL_SIDE_MARGIN_TWIPS: f32 = 108.0;

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
  pub compound: bool,
}

impl Default for BorderStyle {
  fn default() -> Self {
    Self {
      width_pt: 0.5,
      spacing_pt: 0.0,
      color: RgbColor { r: 0, g: 0, b: 0 },
      compound: false,
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
  pub bidi: bool,
  pub shading: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub page_break_before: bool,
  pub keep_with_next: bool,
  pub keep_lines: bool,
  pub contextual_spacing: bool,
  pub hidden_separator: bool,
  pub outline_level: Option<u8>,
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

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct FormWidgetIdAllocator {
  next_id: u32,
}

impl FormWidgetIdAllocator {
  pub(crate) fn next_id(&mut self) -> u32 {
    let id = self.next_id;
    self.next_id = self.next_id.saturating_add(1);
    id
  }
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
  Shape(InlineShape),
  FormWidgetStart(u32),
  FormWidgetEnd(u32),
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
  pub hyperlink_url: Option<String>,
  pub placement: ImagePlacement,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineShape {
  pub width_pt: f32,
  pub height_pt: f32,
  pub offset_x_pt: f32,
  pub offset_y_pt: f32,
  pub fill_color: Option<RgbColor>,
  pub stroke: Option<BorderStyle>,
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextStyle {
  pub font_family: Option<Arc<str>>,
  pub font_size_pt: f32,
  pub character_spacing_pt: f32,
  pub baseline_shift_pt: f32,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
  pub strikethrough: bool,
  pub uppercase: bool,
  pub hidden: bool,
  pub color: RgbColor,
  pub opacity: f32,
  pub outline_color: Option<RgbColor>,
  pub outline_opacity: f32,
  pub outline_width_pt: f32,
  pub highlight: Option<RgbColor>,
}

impl Default for TextStyle {
  fn default() -> Self {
    Self {
      font_family: None,
      font_size_pt: 11.0,
      character_spacing_pt: 0.0,
      baseline_shift_pt: 0.0,
      bold: false,
      italic: false,
      underline: false,
      strikethrough: false,
      uppercase: false,
      hidden: false,
      color: RgbColor { r: 0, g: 0, b: 0 },
      opacity: 1.0,
      outline_color: None,
      outline_opacity: 1.0,
      outline_width_pt: 0.0,
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
  pub top_margin_was_negative: bool,
  pub bottom_margin_was_negative: bool,
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
      top_margin_was_negative: false,
      bottom_margin_was_negative: false,
      header_distance_pt: 36.0,
      footer_distance_pt: 36.0,
      background: None,
      borders: CellBordersModel::default(),
      borders_offset_from_text: false,
    }
  }
}
