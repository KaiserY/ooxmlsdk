use std::collections::BTreeMap;
use std::sync::Arc;

use bytes::Bytes;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_chart as c;
use ooxmlsdk::schemas::schemas_openxmlformats_org_drawingml_2006_main as a;
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use ooxmlsdk::schemas::{
  schemas_microsoft_com_office_drawing_2012_chart_style as cs,
  schemas_microsoft_com_office_drawing_2014_chartex as cx,
};

use crate::model::common_rgb;
pub(crate) use crate::model::{
  BorderDashPattern, BorderStyle, CellBordersModel, DynamicFieldKind, FieldNumberFormat,
  FormWidget, FormWidgetKind, ImageCrop, LegacyTextRelief, LineNumbering, PageSetup, RgbColor,
  TextStyle,
};
use crate::{common, units};

#[derive(Clone, Debug)]
pub(crate) struct DocxDocument {
  pub page: PageSetup,
  pub page_background_pattern: Option<common::PatternFill>,
  pub line_number_style: TextStyle,
  pub note_separator_style: TextStyle,
  pub footnote_separator_stories: NoteSeparatorStories,
  pub endnote_separator_stories: NoteSeparatorStories,
  pub uses_office_recovered_paragraph_defaults: bool,
  pub default_tab_stop_pt: f32,
  pub hyphenation: HyphenationSettings,
  pub compatibility_mode: u16,
  pub justify_lines_with_shrinking: bool,
  pub do_not_expand_shift_return: bool,
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
  pub footnote_positions: Vec<w::FootnotePositionValues>,
  pub endnotes: BTreeMap<i64, Vec<Block>>,
  pub endnote_numbering: Vec<NoteNumberingSpec>,
  pub endnote_position: w::EndnotePositionValues,
  pub title_page: bool,
  pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct NoteSeparatorStories {
  pub separator: Vec<Block>,
  pub continuation_separator: Vec<Block>,
  pub continuation_notice: Vec<Block>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HyphenationSettings {
  pub automatic: bool,
  pub consecutive_line_limit: u16,
  pub zone_pt: f32,
  pub do_not_hyphenate_caps: bool,
  pub page_bottom: PageBottomHyphenation,
}

impl Default for HyphenationSettings {
  fn default() -> Self {
    Self {
      automatic: false,
      consecutive_line_limit: 0,
      // ECMA-376 Part 1 §17.15.1.53 initializes an omitted zone to
      // 360 twentieths of a point.
      zone_pt: 18.0,
      do_not_hyphenate_caps: false,
      // [MS-DOCX] makes an omitted useWord2013TrackBottomHyphenation setting
      // use the Word 2013 behavior: move the whole bottom line.
      page_bottom: PageBottomHyphenation::MoveLine,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum PageBottomHyphenation {
  Allow,
  #[default]
  MoveLine,
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
  pub discarded_carrier_spacing_after_pt: Option<f32>,
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
  pub balanced_height_pt: Option<f32>,
  /// Layout-only sum of the content heights in columns completed before the
  /// current column. Import initializes this to zero; pagination distinguishes
  /// a naturally filled column from an authored early column break.
  pub completed_content_height_pt: f32,
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
      balanced_height_pt: None,
      completed_content_height_pt: 0.0,
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
  /// The effective `w:pageBreakBefore` from the paragraph which starts this
  /// frame. Word applies the break to the frame's anchor in the outer story,
  /// not to the paragraph after it has moved into the frame story.
  pub page_break_before: bool,
  pub width_pt: Option<f32>,
  pub height_pt: Option<f32>,
  pub height_rule: FrameHeightRule,
  pub vertical_text_flow: Option<VerticalTextFlow>,
  pub placement: FloatingFramePlacement,
  pub suppress_overlap: bool,
  /// Decoration supplied by a non-content anchor paragraph.
  ///
  /// Ordinary `framePr` paragraphs retain their own `pBdr`/`shd` because
  /// adjacent paragraphs can share one frame while carrying different
  /// paragraph decoration.
  pub outer_fill_color: Option<ShadingPaint>,
  pub outer_borders: ParagraphBordersModel,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum VerticalTextFlow {
  TopToBottomRightToLeft,
  BottomToTopLeftToRight,
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
  /// Ordered field/bookmark markers retained from the source paragraph.
  ///
  /// Complex Word fields are allowed to start in one paragraph and end in a
  /// later paragraph (a TOC normally does exactly that), so the inline result
  /// alone is not enough to reconstruct document-level field ownership.
  pub field_events: Vec<ParagraphFieldEvent>,
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
  pub list_label_image: Option<ListLabelImage>,
  pub list_label_style: TextStyle,
  pub list_label_hyperlink_url: Option<String>,
  pub list_label_tab_stop_pt: Option<f32>,
}

#[derive(Clone, Debug)]
pub(crate) struct ListLabelImage {
  pub image: InlineImage,
  /// The visible `w:lvlText` characters replaced by instances of the image.
  pub replacement_text: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ParagraphFieldEvent {
  /// Visible story content. Field scanning uses this marker to distinguish a
  /// field that owns its boundary paragraphs from one embedded in surrounding
  /// paragraph text; the content itself remains in `Paragraph::inlines`.
  Content,
  Begin {
    locked: bool,
    dirty: bool,
  },
  Instruction(String),
  Separate,
  End,
  Simple {
    instruction: String,
    locked: bool,
    dirty: bool,
  },
  BookmarkStart {
    id: String,
    name: String,
  },
  BookmarkEnd {
    id: String,
  },
  /// The source paragraph delimiter is not part of the imported story.
  ///
  /// Word IF instructions can span source paragraphs even though the
  /// resulting conditional text is a single paragraph.  A completed IF/REF
  /// result uses the same suppression until its field closes, but remembers
  /// the delimiter so it can be materialized at that exact close position.
  SuppressParagraphBreak {
    deferred: bool,
  },
  /// Suppress a cached-result delimiter owned by one unlocked REF field.
  ///
  /// Until fixed-output refresh resolves that field from its bookmark, this
  /// has the same meaning as `SuppressParagraphBreak { deferred: true }`.
  /// The import-local id lets refresh remove only this REF's cached paragraph
  /// structure without changing nested or neighboring IF/REF fields.
  SuppressReferenceParagraphBreak {
    field_id: u64,
  },
  /// Materialize a paragraph delimiter that was deferred by an open IF/REF
  /// field. `inline_offset` is measured in the paragraph's imported inline
  /// sequence, immediately after the closing field result.
  DeferredParagraphBreak {
    inline_offset: usize,
  },
  /// Cached REF-result delimiter paired with
  /// `SuppressReferenceParagraphBreak` by `field_id`.
  DeferredReferenceParagraphBreak {
    field_id: u64,
    inline_offset: usize,
  },
  /// Exact inline span occupied by one top-level unlocked REF result.
  ///
  /// Word's fixed-format exporter refreshes this span from the referenced
  /// bookmark without mutating the open document. Keep the span internal to
  /// import/refresh so unrelated cached fields remain authoritative.
  ReferenceResultSpan {
    field_id: u64,
    bookmark_name: String,
    inline_start: usize,
    inline_end: usize,
    merge_format: bool,
  },
}

#[derive(Clone, Debug)]
pub(crate) struct Table {
  pub column_widths_pt: Vec<f32>,
  pub preferred_width_pt: Option<f32>,
  pub preferred_width_pct: Option<f32>,
  pub layout: TableLayoutMode,
  pub indent_left_pt: f32,
  pub alignment: TableAlignment,
  pub right_to_left: bool,
  pub align_leading_cell_content: bool,
  pub in_header_footer: bool,
  pub placement: Option<FloatingFramePlacement>,
  pub allow_overlap: bool,
  pub split_allowed: bool,
  pub following_text_flow: bool,
  pub explicit_no_repeat_header: bool,
  pub page_break_before: bool,
  pub starts_after_last_rendered_page_break: bool,
  pub borders: Option<TableBordersModel>,
  pub cell_spacing_pt: f32,
  pub rows: Vec<TableRow>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TableLayoutMode {
  #[default]
  AutoFit,
  Fixed,
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
  pub width_before_pt: Option<f32>,
  pub width_after_pt: Option<f32>,
  pub layout: Option<TableLayoutMode>,
  pub borders: Option<TableBordersModel>,
  /// Row-scoped `w:tblPrEx/w:shd`, with the table-level paint as its base.
  pub spacing_shading: Option<ShadingPaint>,
  pub redline_color: Option<RgbColor>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ShadingPaint {
  None,
  Solid(RgbColor),
  Pattern(common::PatternFill),
}

impl ShadingPaint {
  pub(crate) fn solid_color(self) -> Option<RgbColor> {
    match self {
      Self::Solid(color) => Some(color),
      Self::None | Self::Pattern(_) => None,
    }
  }

  pub(crate) fn common_fill(self) -> Option<common::Fill<'static>> {
    match self {
      Self::None => None,
      Self::Solid(color) => Some(common::Fill::Solid(common_rgb(color, 1.0))),
      Self::Pattern(pattern) => Some(common::Fill::Pattern(pattern)),
    }
  }

  pub(crate) fn is_visible(self) -> bool {
    !matches!(self, Self::None)
  }
}

#[derive(Clone, Debug)]
pub(crate) struct TableCell {
  pub blocks: Vec<Block>,
  pub shading: Option<ShadingPaint>,
  pub borders: CellBordersModel,
  pub border_suppressions: CellBorderSuppressions,
  pub margins: CellMargins,
  pub preferred_width_pt: Option<f32>,
  pub preferred_width_pct: Option<f32>,
  pub grid_span: usize,
  pub vertical_merge_continue: bool,
  pub no_wrap: bool,
  pub fit_text: bool,
  pub hide_end_mark: bool,
  pub vertical_alignment: TableCellVerticalAlignment,
  pub text_rotation_deg: Option<f32>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct CellBorderSuppressions {
  pub top: bool,
  pub right: bool,
  pub bottom: bool,
  pub left: bool,
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

impl CellMargins {
  pub(crate) const fn zero() -> Self {
    Self {
      top_pt: 0.0,
      right_pt: 0.0,
      bottom_pt: 0.0,
      left_pt: 0.0,
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
pub(crate) struct ParagraphBordersModel {
  pub top: Option<BorderStyle>,
  pub right: Option<BorderStyle>,
  pub bottom: Option<BorderStyle>,
  pub left: Option<BorderStyle>,
  pub between: Option<BorderStyle>,
  pub bar: Option<BorderStyle>,
}

impl ParagraphBordersModel {
  pub(crate) fn is_empty(self) -> bool {
    self == Self::default()
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ParagraphBorderOverrides {
  pub top: bool,
  pub right: bool,
  pub bottom: bool,
  pub left: bool,
  pub between: bool,
  pub bar: bool,
}

impl ParagraphBorderOverrides {
  pub(crate) fn merge(self, target: &mut ParagraphBordersModel, values: ParagraphBordersModel) {
    if self.top {
      target.top = values.top;
    }
    if self.right {
      target.right = values.right;
    }
    if self.bottom {
      target.bottom = values.bottom;
    }
    if self.left {
      target.left = values.left;
    }
    if self.between {
      target.between = values.between;
    }
    if self.bar {
      target.bar = values.bar;
    }
  }

  pub(crate) fn include(&mut self, values: Self) {
    self.top |= values.top;
    self.right |= values.right;
    self.bottom |= values.bottom;
    self.left |= values.left;
    self.between |= values.between;
    self.bar |= values.bar;
  }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct ParagraphFormat {
  pub style_id: Option<Arc<str>>,
  pub numbering_id: Option<i32>,
  /// Absolute and line-unit spacing remain independent through the style
  /// hierarchy. Word gives a nonzero line-unit value precedence at final
  /// resolution, while a later zero line-unit value exposes the inherited or
  /// authored absolute fallback ([MS-OI29500] Part 1 §17.3.1.33(a)).
  pub spacing_before_pt: f32,
  pub spacing_before_lines: Option<f32>,
  pub spacing_before_auto: Option<bool>,
  pub spacing_before_auto_pt: Option<f32>,
  pub spacing_after_pt: f32,
  pub spacing_after_lines: Option<f32>,
  pub spacing_after_auto: Option<bool>,
  pub spacing_after_auto_pt: Option<f32>,
  pub spacing_before_set: bool,
  pub spacing_after_set: bool,
  /// `w:doNotUseHTMLParagraphAutoSpacing` selects Word's historical
  /// non-HTML paragraph-spacing model. Adjacent ordinary margins add instead
  /// of collapsing to their maximum; keep the document setting on each
  /// imported paragraph so nested stories use the same rule during layout.
  pub additive_paragraph_spacing: bool,
  /// Word repaired an undefined built-in heading through its application
  /// heading-base parent. The recovered upper spacing is real style state,
  /// but Word suppresses that application-supplied value on the first body
  /// paragraph; authored first-paragraph spacing must remain distinguishable.
  pub office_recovered_builtin_heading_spacing_before: bool,
  pub line_height_pt: Option<f32>,
  pub line_height_set: bool,
  /// Word supplied the line multiple while repairing a missing application
  /// paragraph default. ECMA-376 Part 1 §17.3.1.33 says an omitted direct
  /// `w:spacing@line` retains the value already set in the style hierarchy, so
  /// that authored value must remain distinct: both resolve with
  /// `line_height_set == false`, but only the repaired value permits the legacy
  /// East Asian paragraph-mark line box to remain authoritative.
  pub office_recovered_line_height: bool,
  /// The application line multiple was recovered while the package also
  /// omitted its Settings part. This is narrower than missing pPrDefault:
  /// packages with a real Settings part can carry document-grid and
  /// compatibility state even when their style defaults are incomplete.
  pub office_recovered_line_height_without_settings_part: bool,
  /// A direct w:pPr/w:rPr font size formats the physical paragraph mark.
  /// Keep this separate from an empty w:r/w:t insertion range: Word bases an
  /// otherwise empty line on the mark, while direct paragraph-mark formatting
  /// remains authoritative over application-recovered compatibility metrics.
  pub paragraph_mark_font_size_set: bool,
  /// The active numbering level owns an explicit `w:lvl/w:rPr/w:sz` or
  /// `w:szCs`.  Word/Writer keep that synthesized number portion's font box
  /// independent from paragraph hard attributes; retain the source bit after
  /// style resolution so legacy line-height recovery cannot scale it twice.
  pub numbering_level_font_size_set: bool,
  pub line_height_rule: LineHeightRule,
  /// A numbered source paragraph directly paints its mark with `w:highlight`
  /// or `w:shd`. Word retains that painted numbering mark as an independent
  /// CJK line-box participant; the resolved [`TextStyle`] alone cannot
  /// preserve which layer supplied the paint after style merging.
  pub numbered_paragraph_mark_background: bool,
  /// WPS `bodyPr@compatLnSpc` is a text-body compatibility switch, not a
  /// paragraph-style property. The importer annotates every paragraph in the
  /// completed textbox story so line layout can keep the state scoped to that
  /// shape; omission and an explicit false both remain the default here.
  pub wordprocessing_shape_compatible_line_spacing: bool,
  /// A direct paragraph in `wps:txbx/w:txbxContent` belongs to an independent
  /// WordprocessingML text-frame story. Layout currently reuses the table-cell
  /// formatter for clipping and pagination, so retain this owner bit to avoid
  /// inheriting real table-cell grid behavior from that implementation detail.
  /// Paragraphs in an actual table nested inside the textbox remain ordinary
  /// table-cell stories and deliberately leave this false.
  pub wordprocessing_shape_story: bool,
  pub snap_to_grid: Option<bool>,
  pub line_vertical_alignment: Option<common::LineVerticalAlignment>,
  pub indent_left_pt: f32,
  pub indent_right_pt: f32,
  pub first_line_indent_pt: f32,
  pub indent_left_character_units: Option<f32>,
  pub indent_right_character_units: Option<f32>,
  pub first_line_indent_character_units: Option<f32>,
  pub character_indent_unit_pt: Option<f32>,
  pub indent_left_set: bool,
  pub indent_right_set: bool,
  pub first_line_indent_set: bool,
  pub tab_stops: Vec<TabStop>,
  pub tab_stop_clear_positions_pt: Vec<f32>,
  pub tab_stops_set: bool,
  pub list_label_width_aware_tab: bool,
  pub list_label_uses_explicit_tab_stop: bool,
  pub list_label_justification: w::LevelJustificationValues,
  pub alignment: ParagraphAlignment,
  pub justification: ParagraphJustification,
  pub justification_set: bool,
  pub bidi: bool,
  pub bidi_set: bool,
  /// Presence records an authored/inherited `w:shd`; `None` inside the paint
  /// is represented by [`ShadingPaint::None`] so `w:val="nil"` can cancel an
  /// inherited value without becoming indistinguishable from omission.
  pub shading: Option<ShadingPaint>,
  pub borders: ParagraphBordersModel,
  /// Tracks which individual `w:pBdr` children were authored. A missing side
  /// inherits through the style hierarchy, while `w:val="none"` is an
  /// authored side whose effective border is deliberately empty.
  pub border_overrides: ParagraphBorderOverrides,
  pub page_break_before: bool,
  pub page_break_before_set: bool,
  pub keep_with_next: bool,
  pub keep_with_next_set: bool,
  pub keep_lines: bool,
  pub keep_lines_set: bool,
  pub widow_control: Option<bool>,
  pub contextual_spacing: bool,
  pub contextual_spacing_set: bool,
  pub suppress_auto_hyphens: Option<bool>,
  pub suppress_line_numbers: Option<bool>,
  pub suppress_overlap: Option<bool>,
  pub auto_space_de: Option<bool>,
  pub auto_space_dn: Option<bool>,
  /// Effective `w:overflowPunct` value. ECMA-376 Part 1 §17.3.1.21
  /// defines omission as true, so `None` is resolved at line layout rather
  /// than collapsed here; an inherited or direct explicit false must remain
  /// distinguishable from omission while paragraph styles are merged.
  pub overflow_punctuation: Option<bool>,
  pub hidden_separator: bool,
  pub deleted_separator: bool,
  pub outline_text_inlines: Option<usize>,
  /// Effective outline level supplied by the paragraph style hierarchy,
  /// before direct paragraph properties are applied. TOC `\o` uses this
  /// value; `\u` uses the final `outline_level` below.
  pub style_outline_level: Option<u8>,
  pub outline_level: Option<u8>,
  pub vertical_text_flow: Option<VerticalTextFlow>,
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

#[derive(Clone, Debug)]
pub(crate) struct PositionalTab {
  pub alignment: TabStopAlignment,
  pub relative_to: PositionalTabBase,
  pub leader: TabLeader,
  pub style: TextStyle,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum PositionalTabBase {
  #[default]
  Margin,
  Indent,
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
  /// The effective style-hierarchy value came from the logical `w:jc=start`
  /// token, rather than the physically spelled `left` or legacy `numTab`
  /// tokens which can resolve to the same painted edge in an LTR paragraph.
  pub logical_start: bool,
  /// The effective style-hierarchy value came from the explicit physical
  /// `w:jc=left` token. Keep it separate from `logical_start`, `numTab`, and
  /// the omitted/default state even when import resolves them to one edge.
  pub physical_left: bool,
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
      logical_start: false,
      physical_left: false,
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum RubyAlignment {
  #[default]
  Center,
  DistributeLetter,
  DistributeSpace,
  Left,
  Right,
  RightVertical,
}

#[derive(Clone, Debug)]
pub(crate) struct RubyInline {
  pub base: Vec<TextRun>,
  pub guide: Vec<TextRun>,
  pub alignment: RubyAlignment,
  pub raise_pt: f32,
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
  NoteReferenceMark(NoteReferenceMark),
  NoteSeparatorMark(NoteSeparatorMark),
  ClearLineBreak(LineBreakClear),
  PositionalTab(PositionalTab),
  Ruby(RubyInline),
  LegacyFormCheckBox(LegacyFormCheckBox),
  Image(InlineImage),
  Shape(InlineShape),
  DrawingGroupStart(InlineDrawingGroupEffect),
  DrawingGroupEnd,
  BookmarkStart(String),
  FormWidgetStart(u32),
  FormWidgetEnd(u32),
  LastRenderedPageBreak,
  PageBreak,
  ColumnBreak,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum NoteReferenceKind {
  Footnote,
  Endnote,
}

#[derive(Clone, Debug)]
pub(crate) struct NoteReferenceMark {
  pub kind: NoteReferenceKind,
  pub style: TextStyle,
  pub style_ref_keys: Vec<Arc<str>>,
}

#[derive(Clone, Debug)]
pub(crate) struct NoteSeparatorMark {
  pub continuation: bool,
  pub style: TextStyle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LineBreakClear {
  Left,
  Right,
  All,
}

#[derive(Clone, Debug)]
pub(crate) struct LegacyFormCheckBox {
  pub checked: bool,
  pub style: TextStyle,
  pub hyperlink_url: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineDrawingGroupEffect {
  pub effects: common::DrawingEffectSource,
  pub rotation_deg: f32,
  pub placement: ImagePlacement,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum InlineImageLineBox {
  /// The host paragraph computes the line box around an ordinary inline
  /// picture, shape, OLE preview, or metafile.
  #[default]
  CharacterLike,
  /// A legacy embedded-object preview remains a character-like frame for
  /// line measurement, while its parent run's w:position supplies the shared
  /// inline baseline. This is distinct from an object's intrinsic baseline:
  /// the run displacement must not shrink a grid line around the cached
  /// presentation.
  EmbeddedObjectRunPosition,
  /// The OfficeMath renderer has already expanded the ink bounds with the
  /// OpenType MATH `MathLeading` contract. The host must not add the ordinary
  /// paragraph auto-line-spacing excess a second time.
  OfficeMathExternal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OfficeMathBreakKind {
  Automatic,
  Manual {
    align_at: Option<u8>,
  },
  /// Boundary between adjacent `m:oMath` instances in one `m:oMathPara`.
  /// Each instance is a separate display equation rather than a wrapping
  /// continuation of the preceding equation.
  Equation,
}

#[derive(Clone, Debug)]
pub(crate) struct OfficeMathLineFragment {
  pub image: InlineImage,
  /// Discardable OfficeMath atom spacing between the preceding fragment and
  /// this fragment when both remain on the same physical line.
  pub same_line_gap_pt: f32,
  pub break_before: Option<OfficeMathBreakKind>,
  /// `brkBin=repeat` realizes a second operator only after an actual wrap.
  pub wrapped_prefix: Option<InlineImage>,
  /// Alternate previous-fragment paint used only when a repeated subtraction
  /// changes the sign at the physical end of the line.
  pub line_end_variant: Option<InlineImage>,
  /// Offset of the first binary/relation operator from this fragment's frame
  /// origin, used by m:brk/@alnAt continuation alignment.
  pub first_operator_offset_pt: Option<f32>,
}

#[derive(Clone, Debug)]
pub(crate) struct OfficeMathLineLayout {
  pub fragments: Vec<OfficeMathLineFragment>,
  pub has_manual_break: bool,
  pub display_wrap_indent_pt: Option<f32>,
  pub display_wrap_right: bool,
  /// Operator offsets in the unwrapped zone, counted from its frame origin.
  pub operator_offsets_pt: Vec<f32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OfficeMathDisplayAlignment {
  Left,
  Center,
  CenterGroup,
  Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct OfficeMathDisplayLayout {
  /// `None` means that Word's `m:dispDef=off` path delegates horizontal
  /// adjustment to the owning WordprocessingML paragraph.
  pub alignment: Option<OfficeMathDisplayAlignment>,
  pub left_margin_pt: f32,
  pub right_margin_pt: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineImage {
  pub data: Bytes,
  pub content_type: Option<String>,
  pub picture_frame: Option<Box<InlineShape>>,
  /// Whether the picture frame geometry clips the image surface. DrawingML
  /// picture geometry does; VML `imagedata` is painted on top of its host
  /// shape and therefore keeps the image's implied rectangle un-clipped.
  pub picture_frame_clips_image: bool,
  pub effects: Option<common::DrawingEffectSource>,
  pub static3d: Option<common::drawingml_3d::Static3dStyle>,
  pub width_pt: f32,
  pub height_pt: f32,
  /// Child-space offset inside an inline DrawingML group or canvas.
  pub inline_offset_x_pt: f32,
  pub inline_offset_y_pt: f32,
  pub effect_left_pt: f32,
  pub effect_top_pt: f32,
  pub effect_right_pt: f32,
  pub effect_bottom_pt: f32,
  /// Distance from the visible object's bottom edge to its inline baseline.
  ///
  /// Ordinary pictures derive this from `effectExtent`; Office Math uses a
  /// negative value because its baseline lies above scripts below the axis.
  pub inline_baseline_gap_pt: Option<f32>,
  pub line_box: InlineImageLineBox,
  /// Natural OfficeMath is retained as this image. When it cannot fit a line,
  /// the host lays out these source-backed break fragments without scaling the
  /// complete formula as though it were an ordinary picture.
  pub office_math_line_layout: Option<Arc<OfficeMathLineLayout>>,
  /// Display-zone alignment is independent of the owning `w:p` alignment.
  /// Keep it on the realized OfficeMath object so only its physical lines are
  /// adjusted; ordinary text lines in the same `w:p` remain untouched.
  pub office_math_display_layout: Option<OfficeMathDisplayLayout>,
  pub crop: ImageCrop,
  pub rotation_deg: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub metafile_background_color: Option<[u8; 3]>,
  pub alt_text: Option<String>,
  pub hyperlink_url: Option<String>,
  pub semantic_metafile_text: bool,
  /// Whether the metafile text is the visible fixed-output representation,
  /// rather than only an invisible semantic overlay over a raster preview.
  pub metafile_semantic_text_includes_raster_backdrop: bool,
  pub signature_line: Option<common::SignatureLineProperties<'static>>,
  /// ActiveX TextProps font resolved from the control persistence. This is
  /// separate from the static preview LOGFONT because the Office host may
  /// provide its document font when TextProps omits FontName.
  pub semantic_metafile_font_family: Option<Arc<str>>,
  pub native_ole_equation: Option<super::math_type::MathTypeEquation>,
  /// Whether Word should paint a near-native EMF Header.Frame inside the
  /// authored DrawingML extent.
  pub metafile_native_size: bool,
  pub placement: ImagePlacement,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineShape {
  pub width_pt: f32,
  pub height_pt: f32,
  /// Inline line-box size owned by the enclosing `wp:inline` object.
  ///
  /// A WPG child keeps its mapped child geometry in `width_pt`/`height_pt`,
  /// while the parent `wp:extent` remains the character-like object which
  /// changes the paragraph line height.  Keep those two coordinate-space
  /// contracts separate so table-cell clipping cannot mistake a short,
  /// offset child for the complete inline object.
  pub inline_frame_size_pt: Option<(f32, f32)>,
  pub effect_left_pt: f32,
  pub effect_top_pt: f32,
  pub effect_right_pt: f32,
  pub effect_bottom_pt: f32,
  pub geometry: InlineShapeGeometry,
  pub offset_x_pt: f32,
  pub offset_y_pt: f32,
  pub rotation_deg: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub fill_color: Option<RgbColor>,
  pub fill_pattern: Option<common::PatternFill>,
  pub fill_override: Option<Box<common::Fill<'static>>>,
  pub additional_fill_colors: Vec<RgbColor>,
  pub fill_image: Option<InlineShapeImageFill>,
  pub stroke: Option<BorderStyle>,
  pub stroke_pattern: Option<common::PatternFill>,
  pub stroke_override: Option<Box<common::Stroke<'static>>>,
  pub suppress_zero_relative_background: bool,
  pub allow_outside_page: bool,
  pub placement: ImagePlacement,
  pub chart: Option<Box<InlineChart>>,
  pub text_warp: Option<Box<a::PresetTextWarp>>,
  pub text_fill: Option<Box<common::Fill<'static>>>,
  pub effects: Option<common::DrawingEffectSource>,
  pub static3d: Option<common::drawingml_3d::Static3dStyle>,
  pub text_upright: bool,
  pub text_box_writing_mode: TextBoxWritingMode,
  /// Whether the WPS non-visual properties explicitly mark this shape as a
  /// Word text frame (`wps:cNvSpPr/@txBox`).  The marker is independent of
  /// `text_box_blocks`: Word text frames may intentionally have an empty
  /// textbox story while still participating in paragraph wrapping.
  pub word_text_frame: bool,
  pub text_box_blocks: Vec<Block>,
  pub text_inset_left_pt: f32,
  pub text_inset_top_pt: f32,
  pub text_inset_right_pt: f32,
  pub text_inset_bottom_pt: f32,
  pub text_box_auto_fit: bool,
  pub text_box_resizes_to_fit: bool,
  pub text_box_word_wrap: bool,
  pub text_box_clip_vertical_overflow: bool,
  pub text_vertical_alignment: TextBoxVerticalAlignment,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum TextBoxWritingMode {
  #[default]
  Horizontal,
  TopToBottomRightToLeft,
  BottomToTopLeftToRight,
  EastAsianVerticalRightToLeft,
  MongolianVerticalLeftToRight,
  StackedLeftToRight,
  StackedRightToLeft,
}

impl TextBoxWritingMode {
  pub(crate) fn is_vertical(self) -> bool {
    !matches!(self, Self::Horizontal)
  }
}

#[derive(Clone, Debug)]
pub(crate) struct InlineChart {
  pub chart_space: Option<Box<c::ChartSpace>>,
  pub extended_chart_space: Option<Box<cx::ChartSpace>>,
  pub extended_chart_styles: Vec<cs::ChartStyle>,
  pub extended_chart_color_styles: Vec<cs::ColorStyle>,
  pub extended_chart_theme: crate::render::chartex::ChartExTheme,
  pub ui_language: Option<String>,
  pub automatic_title: String,
  pub title_style: TextStyle,
  /// Legend text. Axis titles and tick labels have independent OOXML text
  /// bodies and therefore use the role-specific styles below.
  pub label_style: TextStyle,
  pub category_axis_title_style: TextStyle,
  pub value_axis_title_style: TextStyle,
  pub additional_axis_title_styles: Vec<TextStyle>,
  pub category_label_style: TextStyle,
  pub value_label_style: TextStyle,
  pub series_label_style: TextStyle,
  pub data_label_style: TextStyle,
  pub data_label_styles: Vec<Vec<Option<TextStyle>>>,
  pub data_label_rich_text_styles: Vec<Vec<Vec<TextStyle>>>,
  pub gridline_color: RgbColor,
  pub automatic_chart_area_line_width_pt: f32,
  pub automatic_series_line_width_pt: f32,
  pub value_gridline_width_pt: Option<f32>,
  pub axis_line_width_pt: Option<f32>,
  pub category_major_gridline: Option<(RgbColor, f32)>,
  pub category_minor_gridline: Option<(RgbColor, f32)>,
  pub series_colors: Vec<RgbColor>,
  pub series_point_colors: Vec<Vec<Option<RgbColor>>>,
  pub series_styles: Vec<common::ShapeStyle<'static>>,
  pub series_marker_styles: Vec<common::ShapeStyle<'static>>,
  pub series_point_marker_styles: Vec<Vec<Option<common::ShapeStyle<'static>>>>,
  pub automatic_series_marker_strokes: Vec<common::ShapeStyleValue<common::Stroke<'static>>>,
  pub automatic_series_fills: Vec<common::ShapeStyleValue<common::Fill<'static>>>,
  pub automatic_series_point_fills:
    Vec<Vec<Option<common::ShapeStyleValue<common::Fill<'static>>>>>,
  pub automatic_series_point_marker_strokes:
    Vec<Vec<Option<common::ShapeStyleValue<common::Stroke<'static>>>>>,
  pub data_point_effect_style: crate::render::chart::ChartShapeEffects,
  pub series_effect_styles: Vec<crate::render::chart::ChartShapeEffects>,
  pub series_marker_effect_styles: Vec<crate::render::chart::ChartShapeEffects>,
  pub series_point_effect_styles: Vec<Vec<Option<crate::render::chart::ChartShapeEffects>>>,
  pub series_point_marker_effect_styles: Vec<Vec<Option<crate::render::chart::ChartShapeEffects>>>,
  pub trendline_styles: Vec<Vec<common::ShapeStyle<'static>>>,
  pub error_bar_styles: Vec<Vec<common::ShapeStyle<'static>>>,
  pub group_decoration_styles: Vec<crate::pptx::chart::CartesianChartGroupDecorationStyle>,
  pub series_point_styles: Vec<Vec<Option<common::ShapeStyle<'static>>>>,
  pub surface_band_colors: Vec<Vec<(u32, RgbColor)>>,
  pub data_label_fill_colors: Vec<Vec<Option<RgbColor>>>,
  pub pie_point_colors: Vec<RgbColor>,
  pub pie_point_styles: Vec<common::ShapeStyle<'static>>,
  pub leader_line_style: common::ShapeStyle<'static>,
  pub title_fill_color: Option<RgbColor>,
  pub legend_frame_style: common::ShapeStyle<'static>,
  pub chart_area_style: common::ShapeStyle<'static>,
  pub plot_area_style: common::ShapeStyle<'static>,
  pub floor_style: common::ShapeStyle<'static>,
  pub side_wall_style: common::ShapeStyle<'static>,
  pub back_wall_style: common::ShapeStyle<'static>,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineShapeImageFill {
  pub data: Bytes,
  pub content_type: Option<String>,
  pub crop: ImageCrop,
  pub rotation_deg: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub rotate_with_shape: bool,
  pub mode: InlineShapeImageFillMode,
}

#[derive(Clone, Debug)]
pub(crate) enum InlineShapeImageFillMode {
  Stretch,
  Contain,
  Cover,
  DrawingMlTile(Box<a::Tile>),
  Tile {
    size: Option<String>,
    origin: Option<String>,
    position: Option<String>,
  },
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum InlineShapeGeometry {
  Rectangle,
  Line,
  Path {
    paths: Vec<common::DrawingPath>,
    outline: Option<Box<a::Outline>>,
  },
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
  /// The shared host extent used by `wp:align` for children of a DrawingML
  /// group. Child geometry still keeps its own mapped width and height.
  pub alignment_extent: Option<FloatingAlignmentExtent>,
  /// Child offset inside a percentage-sized VML group host. The host extent
  /// owns alignment; this offset is scaled with that host at layout time.
  pub group_child_offset_x_pt: f32,
  pub group_child_offset_y_pt: f32,
  pub horizontal_offset_pt: f32,
  pub vertical_offset_pt: f32,
  pub horizontal_offset_pct: Option<f32>,
  pub vertical_offset_pct: Option<f32>,
  pub wrap: ImageWrapMode,
  pub wrap_side: ImageWrapSide,
  pub behind_text: bool,
  pub layout_in_cell: bool,
  pub allow_overlap: bool,
  pub paint_order: FloatingPaintOrder,
  pub relative_width_to: Option<HorizontalImageReference>,
  pub relative_width_pct: Option<f32>,
  pub relative_height_to: Option<VerticalImageReference>,
  pub relative_height_pct: Option<f32>,
  pub margin_top_pt: f32,
  pub margin_right_pt: f32,
  pub margin_bottom_pt: f32,
  pub margin_left_pt: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FloatingAlignmentExtent {
  pub width_pt: f32,
  pub height_pt: f32,
  pub relative_width_to: Option<HorizontalImageReference>,
  pub relative_width_pct: Option<f32>,
  pub relative_height_to: Option<VerticalImageReference>,
  pub relative_height_pct: Option<f32>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum FloatingPaintOrder {
  #[default]
  Unspecified,
  DrawingMlRelativeHeight(u32),
  VmlZIndex(i32),
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
