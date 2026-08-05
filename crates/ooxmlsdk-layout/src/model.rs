use std::borrow::Cow;
use std::sync::Arc;

use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;

use crate::common;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CellBordersModel {
  pub top: Option<BorderStyle>,
  pub right: Option<BorderStyle>,
  pub bottom: Option<BorderStyle>,
  pub left: Option<BorderStyle>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BorderStyle {
  pub width_pt: f32,
  pub spacing_pt: f32,
  pub color: RgbColor,
  pub compound: bool,
  pub dash_pattern: BorderDashPattern,
  pub shadow: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum BorderDashPattern {
  #[default]
  Solid,
  Dotted,
  Dashed,
  FineDashed,
  DashDot,
  DashDotDot,
}

impl BorderDashPattern {
  pub(crate) fn common_dash(self, width_pt: f32) -> Option<Vec<common::Pt>> {
    let multipliers: &[f32] = match self {
      Self::Solid => return None,
      Self::Dotted => &[1.0, 1.0],
      Self::Dashed => &[3.0, 1.0],
      // Word's fixed-format writer emits a 0.5 pt dashSmallGap border as a
      // repeating six-pixel mask spanning 2.4 pt: five opaque pixels followed
      // by one transparent pixel. That is a four-width dash and a 0.8-width
      // gap. The other patterns match the width-relative GDI+ DashStyle arrays.
      Self::FineDashed => &[4.0, 0.8],
      Self::DashDot => &[3.0, 1.0, 1.0, 1.0],
      Self::DashDotDot => &[3.0, 1.0, 1.0, 1.0, 1.0, 1.0],
    };
    Some(
      multipliers
        .iter()
        .map(|multiplier| common::Pt(multiplier * width_pt))
        .collect(),
    )
  }
}

impl Default for BorderStyle {
  fn default() -> Self {
    Self {
      width_pt: 0.5,
      spacing_pt: 0.0,
      color: RgbColor { r: 0, g: 0, b: 0 },
      compound: false,
      dash_pattern: BorderDashPattern::Solid,
      shadow: false,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) enum LegacyTextRelief {
  #[default]
  None,
  Embossed,
  Engraved,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
  pub font_family: Option<Arc<str>>,
  pub fallback_font_family: Option<Arc<str>>,
  /// OOXML font-table family classification used when the named face and its
  /// explicit alternate names are unavailable.
  pub font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  pub east_asia_font_family: Option<Arc<str>>,
  pub complex_font_family: Option<Arc<str>>,
  pub symbol_font_family: Option<Arc<str>>,
  /// Resolved WordprocessingML `w:lang/@w:val` language tag.
  pub language: Option<Arc<str>>,
  /// Resolved WordprocessingML `w:lang/@w:eastAsia` language tag.
  pub east_asia_language: Option<Arc<str>>,
  /// Resolved WordprocessingML `w:lang/@w:bidi` language tag.
  pub bidi_language: Option<Arc<str>>,
  pub font_size_pt: f32,
  pub complex_font_size_pt: Option<f32>,
  /// Complex-script formatting selected by WordprocessingML `w:cs`.
  /// `None` leaves script selection to the Unicode content.
  pub complex_script: Option<bool>,
  /// Right-to-left run override selected by WordprocessingML `w:rtl`.
  /// `None` leaves direction and script selection to the Unicode content.
  pub right_to_left: Option<bool>,
  /// Resolved Unicode bidi level for one directionally uniform laid-out
  /// portion. This controls glyph order and mirroring without selecting the
  /// WordprocessingML complex-script formatting attached to `w:rtl`.
  pub(crate) resolved_bidi_level: Option<u8>,
  pub complex_bold: Option<bool>,
  pub complex_italic: Option<bool>,
  /// Minimum WordprocessingML font size at which OpenType kerning is active.
  /// `None` leaves the shaping engine's native default unchanged.
  pub kerning_minimum_size_pt: Option<f32>,
  /// OpenType ligature categories selected by the source document. `None`
  /// leaves the shaping engine's native defaults unchanged.
  pub ligatures: Option<common::OpenTypeLigatures>,
  /// Word 2010 numeral, contextual-alternate, and stylistic-set controls.
  pub open_type_features: common::OpenTypeFeatureSettings,
  /// Horizontal WordprocessingML character scale. `None` means 100% and
  /// preserves explicit 100% overrides in style inheritance.
  pub horizontal_scale: Option<f32>,
  /// Explicit PDF-semantic distances between consecutive character origins.
  /// This is populated only for GDI `ExtTextOut` replacement layers whose
  /// nonuniform `Dx` array cannot be represented by ordinary character
  /// spacing or a single horizontal scale.
  pub(crate) semantic_character_advances_pt: Option<Arc<[f32]>>,
  pub character_spacing_pt: f32,
  pub baseline_shift_pt: f32,
  /// Original WordprocessingML font size retained for the line box when
  /// automatic `w:vertAlign` shrinks and shifts the painted glyph.
  pub(crate) automatic_escapement_font_size_pt: Option<f32>,
  /// Complex-script counterpart of `automatic_escapement_font_size_pt`.
  pub(crate) automatic_escapement_complex_font_size_pt: Option<f32>,
  /// Layout-only minimum line box for generated resources whose Office UI
  /// metrics are taller than their embedded glyph bounds.
  pub(crate) line_height_override_pt: Option<f32>,
  pub line_vertical_alignment: common::LineVerticalAlignment,
  pub semantic_only: bool,
  pub use_windows_font_metrics: bool,
  /// Select Common characters using the WordprocessingML rFonts slot table.
  pub wordprocessingml_font_slots: bool,
  /// Fraction of Word's maximum full-width punctuation compression to apply.
  pub cjk_punctuation_compression_ratio: f32,
  pub pdf_glyph_outlines: bool,
  pub pdf_glyph_outline_options: Option<Arc<common::PdfGlyphOutlineOptions>>,
  pub(crate) text_glow: Option<common::drawingml_image_effects::WordprocessingTextGlow>,
  pub(crate) text_shadow: Option<common::drawingml_image_effects::WordprocessingTextShadow>,
  pub(crate) text_reflection: Option<common::drawingml_image_effects::WordprocessingTextReflection>,
  /// A Word 2010 `w14:scene3d` or `w14:props3d` is present in the effective
  /// run style. Word's fixed-format writer flattens this text together with
  /// its fill, outline, and 2-D effects instead of retaining a text object.
  pub(crate) wordprocessing_text_3d: bool,
  /// Complete `w14:scene3d`/`w14:props3d` run data. The two elements inherit
  /// independently and are combined with a DrawingML text body's 3-D scene
  /// only at fixed-output materialization time.
  pub(crate) wordprocessing_text_3d_parts: Option<common::drawingml_3d::Static3dStyleParts>,
  /// Resolved legacy WordprocessingML `w:outline` toggle. This remains
  /// separate from DrawingML/w14 outlines until fixed-output materialization
  /// because Word uses a distinct one-pixel glyph contour.
  pub(crate) legacy_outline: bool,
  /// Resolved legacy WordprocessingML `w:shadow` toggle.
  pub(crate) legacy_shadow: bool,
  /// Resolved legacy WordprocessingML `w:emboss`/`w:imprint` relief.
  pub(crate) legacy_relief: LegacyTextRelief,
  /// Resolved DrawingML `a:effectLst`/`a:effectDag` attached to character
  /// properties. This remains on the implementation-side style until the
  /// owning text body materializes the visible glyph raster.
  pub(crate) drawingml_text_effects: Option<common::drawingml_image_effects::ImageEffectContainer>,
  /// DrawingML `a:bodyPr/a:scene3d` plus `a:sp3d` text extrusion.
  pub(crate) drawingml_text_static3d: Option<common::drawingml_3d::Static3dStyle>,
  /// Direct WordprocessingML `w:b` state applicable to generated field text.
  ///
  /// This is distinct from `bold`: a generated Word field diagnostic supplies
  /// its own bold default, but explicit result-run or paragraph-mark
  /// `w:b w:val="false"` suppresses that resource formatting.
  pub(crate) wordprocessingml_field_bold_override: Option<bool>,
  /// Whether this run contains an application-generated field diagnostic.
  ///
  /// Word's legacy aligned-tab paint reserves the field portion's boundary
  /// advance before repeating a leader; persisted numeric results do not use
  /// that diagnostic boundary policy.
  pub(crate) wordprocessingml_generated_field_diagnostic: bool,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
  pub strikethrough: bool,
  pub uppercase: bool,
  pub small_caps: bool,
  pub hidden: bool,
  pub rotation_deg: f32,
  pub color: RgbColor,
  /// Whether the WordprocessingML text color is still automatic and may
  /// adapt to an inherited run background.
  pub(crate) color_is_automatic: bool,
  pub opacity: f32,
  pub outline_color: Option<RgbColor>,
  pub outline_opacity: f32,
  pub outline_width_pt: f32,
  pub highlight: Option<RgbColor>,
  pub underline_color: Option<RgbColor>,
}

impl Default for TextStyle {
  fn default() -> Self {
    Self {
      font_family: None,
      fallback_font_family: None,
      font_family_class: None,
      east_asia_font_family: None,
      complex_font_family: None,
      symbol_font_family: None,
      language: None,
      east_asia_language: None,
      bidi_language: None,
      font_size_pt: 11.0,
      complex_font_size_pt: None,
      complex_script: None,
      right_to_left: None,
      resolved_bidi_level: None,
      complex_bold: None,
      complex_italic: None,
      kerning_minimum_size_pt: None,
      ligatures: None,
      open_type_features: common::OpenTypeFeatureSettings::default(),
      horizontal_scale: None,
      semantic_character_advances_pt: None,
      character_spacing_pt: 0.0,
      baseline_shift_pt: 0.0,
      automatic_escapement_font_size_pt: None,
      automatic_escapement_complex_font_size_pt: None,
      line_height_override_pt: None,
      line_vertical_alignment: common::LineVerticalAlignment::Auto,
      semantic_only: false,
      use_windows_font_metrics: false,
      wordprocessingml_font_slots: false,
      cjk_punctuation_compression_ratio: 0.0,
      pdf_glyph_outlines: false,
      pdf_glyph_outline_options: None,
      text_glow: None,
      text_shadow: None,
      text_reflection: None,
      wordprocessing_text_3d: false,
      wordprocessing_text_3d_parts: None,
      legacy_outline: false,
      legacy_shadow: false,
      legacy_relief: LegacyTextRelief::None,
      drawingml_text_effects: None,
      drawingml_text_static3d: None,
      wordprocessingml_field_bold_override: None,
      wordprocessingml_generated_field_diagnostic: false,
      bold: false,
      italic: false,
      underline: false,
      strikethrough: false,
      uppercase: false,
      small_caps: false,
      hidden: false,
      rotation_deg: 0.0,
      color: RgbColor { r: 0, g: 0, b: 0 },
      color_is_automatic: true,
      opacity: 1.0,
      outline_color: None,
      outline_opacity: 1.0,
      outline_width_pt: 0.0,
      highlight: None,
      underline_color: None,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RgbColor {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ImageCrop {
  pub left: f32,
  pub top: f32,
  pub right: f32,
  pub bottom: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PageSetup {
  pub width_pt: f32,
  pub height_pt: f32,
  pub margin_top_pt: f32,
  pub margin_right_pt: f32,
  pub margin_bottom_pt: f32,
  pub margin_left_pt: f32,
  pub gutter_pt: f32,
  pub gutter_at_top: bool,
  pub rtl_gutter: bool,
  pub mirror_margins: bool,
  pub top_margin_was_negative: bool,
  pub bottom_margin_was_negative: bool,
  pub header_distance_pt: f32,
  pub footer_distance_pt: f32,
  pub background: Option<RgbColor>,
  pub borders: CellBordersModel,
  pub borders_offset_from_text: bool,
  pub line_numbering: Option<LineNumbering>,
  pub doc_grid_line_pitch_pt: Option<f32>,
  pub doc_grid_character_spacing_pt: Option<f32>,
  pub adjust_table_line_heights_to_grid: bool,
  pub page_number_start: Option<i32>,
  /// Display format from WordprocessingML `w:pgNumType/@w:fmt`.
  pub page_number_format: FieldNumberFormat,
}

impl Default for PageSetup {
  fn default() -> Self {
    Self {
      // The fixed-output reference environment uses A4 as its default paper.
      // Explicit w:pgSz and SpreadsheetML paper sizes replace these values.
      width_pt: 595.2756,
      height_pt: 841.8898,
      margin_top_pt: 72.0,
      margin_right_pt: 72.0,
      margin_bottom_pt: 72.0,
      margin_left_pt: 72.0,
      gutter_pt: 0.0,
      gutter_at_top: false,
      rtl_gutter: false,
      mirror_margins: false,
      top_margin_was_negative: false,
      bottom_margin_was_negative: false,
      header_distance_pt: 36.0,
      footer_distance_pt: 36.0,
      background: None,
      borders: CellBordersModel::default(),
      borders_offset_from_text: false,
      line_numbering: None,
      doc_grid_line_pitch_pt: None,
      doc_grid_character_spacing_pt: None,
      adjust_table_line_heights_to_grid: false,
      page_number_start: None,
      page_number_format: FieldNumberFormat::Decimal,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LineNumbering {
  pub count_by: i16,
  pub start: i16,
  pub distance_pt: f32,
  pub restart_each_page: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DynamicFieldKind {
  Page {
    number_format: FieldNumberFormat,
  },
  NumPages {
    number_format: FieldNumberFormat,
  },
  Sequence {
    identifier: Arc<str>,
    number_format: FieldNumberFormat,
  },
  PageRef {
    bookmark_name: Arc<str>,
    number_format: FieldNumberFormat,
    relative_position: bool,
  },
  StyleRef {
    style_name: Arc<str>,
    from_bottom: bool,
    numbering_only: bool,
    suppress_non_numerical: bool,
  },
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FieldNumberFormat {
  /// Use the numbering format of the page on which the field target lies.
  PageStyle,
  #[default]
  Decimal,
  LowerRoman,
  UpperRoman,
  LowerLetter,
  UpperLetter,
  /// Preserve any page-number format authored through
  /// WordprocessingML `w:pgNumType/@w:fmt`.
  WordprocessingMl(w::NumberFormatValues),
}

#[derive(Clone, Debug)]
pub struct FormWidget {
  pub id: u32,
  pub kind: FormWidgetKind,
  pub entries: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormWidgetKind {
  Text,
  DropDownList,
  ComboBox,
}

#[derive(Clone, Debug)]
pub(crate) enum PageItem {
  Text(TextItem),
  Image(ImageItem),
  Group {
    mask: Option<ImageItem>,
    transform: Option<common::Transform>,
    blend_mode: common::BlendMode,
    opacity: f32,
    items: Vec<PageItem>,
  },
  LinkArea(LinkAreaItem),
  Path(common::PathItem<'static>),
  Rect(RectItem),
  Line(LineItem),
}

#[derive(Clone, Debug)]
pub(crate) struct TextItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub line_height_pt: f32,
  pub paint_clip: Option<common::Rect>,
  pub discard_if_horizontally_clipped: bool,
  pub text: String,
  pub style: TextStyle,
  pub rotation_center_pt: Option<(f32, f32)>,
  pub hyperlink_url: Option<String>,
  pub form_widget_id: Option<u32>,
  pub paragraph_bidi: bool,
  pub preserve_text_portion: bool,
  pub pdf_text_segmentation: PdfTextSegmentation,
  pub source_path: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PdfTextSegmentation {
  Line,
  WordLine,
  Portion,
}

#[derive(Clone, Debug)]
pub(crate) struct ImageItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub crop: ImageCrop,
  pub clip_path: Vec<common::PathCommand>,
  pub rotation_deg: f32,
  pub flip_horizontal: bool,
  pub flip_vertical: bool,
  pub data: Arc<[u8]>,
  pub content_type: Option<String>,
  pub metafile_monochrome_dib_palette_override: Option<[[u8; 3]; 2]>,
  pub metafile_background_color: Option<[u8; 3]>,
  pub metafile_external_header: Option<crate::render::emf_wmf::WmfExternalHeader>,
  pub metafile_semantic_text_includes_raster_backdrop: bool,
  pub alt_text: Option<String>,
  pub hyperlink_url: Option<String>,
  pub floating: bool,
  pub behind_text: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct LinkAreaItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub hyperlink_url: String,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RectItem {
  pub x_pt: f32,
  pub y_pt: f32,
  pub width_pt: f32,
  pub height_pt: f32,
  pub fill_color: Option<RgbColor>,
  pub fill_opacity: f32,
  pub stroke: Option<BorderStyle>,
  pub stroke_opacity: f32,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct LineItem {
  pub x1_pt: f32,
  pub y1_pt: f32,
  pub x2_pt: f32,
  pub y2_pt: f32,
  pub width_pt: f32,
  pub color: RgbColor,
  pub kind: LineItemKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LineItemKind {
  Stroke,
}

pub(crate) fn common_page_setup(setup: PageSetup) -> common::PageSetup {
  common::PageSetup {
    size: common::Size {
      width: common::Pt(setup.width_pt),
      height: common::Pt(setup.height_pt),
    },
    margins: common::Insets {
      top: common::Pt(setup.margin_top_pt),
      right: common::Pt(setup.margin_right_pt),
      bottom: common::Pt(setup.margin_bottom_pt),
      left: common::Pt(setup.margin_left_pt),
    },
    mirror_margins: setup.mirror_margins,
    top_margin_was_negative: setup.top_margin_was_negative,
    bottom_margin_was_negative: setup.bottom_margin_was_negative,
    header_distance: common::Pt(setup.header_distance_pt),
    footer_distance: common::Pt(setup.footer_distance_pt),
    background: setup.background.map(|color| common_rgb(color, 1.0)),
    borders: common::CellBorders {
      top: setup.borders.top.map(common_border_style),
      right: setup.borders.right.map(common_border_style),
      bottom: setup.borders.bottom.map(common_border_style),
      left: setup.borders.left.map(common_border_style),
    },
    borders_offset_from_text: setup.borders_offset_from_text,
    line_numbering: setup.line_numbering.map(|line| common::LineNumbering {
      count_by: line.count_by,
      start: line.start,
      distance: common::Pt(line.distance_pt),
      restart_each_page: line.restart_each_page,
    }),
    doc_grid_line_pitch: setup.doc_grid_line_pitch_pt.map(common::Pt),
    page_number_start: setup.page_number_start,
  }
}

pub(crate) fn common_text_style(style: TextStyle) -> common::TextStyle<'static> {
  common::TextStyle {
    font_family: style.font_family.map(|value| Cow::Owned(value.to_string())),
    fallback_font_family: style
      .fallback_font_family
      .map(|value| Cow::Owned(value.to_string())),
    east_asia_font_family: style
      .east_asia_font_family
      .map(|value| Cow::Owned(value.to_string())),
    complex_font_family: style
      .complex_font_family
      .map(|value| Cow::Owned(value.to_string())),
    symbol_font_family: style
      .symbol_font_family
      .map(|value| Cow::Owned(value.to_string())),
    font_size: common::Pt(style.font_size_pt),
    complex_font_size: style.complex_font_size_pt.map(common::Pt),
    complex_script: style.complex_script,
    right_to_left: style.right_to_left,
    resolved_bidi_level: style.resolved_bidi_level,
    complex_bold: style.complex_bold,
    complex_italic: style.complex_italic,
    kerning_minimum_size: style.kerning_minimum_size_pt.map(common::Pt),
    ligatures: style.ligatures,
    open_type_features: style.open_type_features,
    horizontal_scale: style.horizontal_scale,
    semantic_character_advances_pt: style.semantic_character_advances_pt,
    character_spacing: common::Pt(style.character_spacing_pt),
    baseline_shift: common::Pt(style.baseline_shift_pt),
    automatic_escapement_font_size: style.automatic_escapement_font_size_pt.map(common::Pt),
    automatic_escapement_complex_font_size: style
      .automatic_escapement_complex_font_size_pt
      .map(common::Pt),
    line_vertical_alignment: style.line_vertical_alignment,
    semantic_only: style.semantic_only,
    use_windows_font_metrics: style.use_windows_font_metrics,
    wordprocessingml_font_slots: style.wordprocessingml_font_slots,
    cjk_punctuation_compression_ratio: style.cjk_punctuation_compression_ratio,
    pdf_glyph_outlines: style.pdf_glyph_outlines,
    pdf_glyph_outline_options: style.pdf_glyph_outline_options,
    bold: style.bold,
    italic: style.italic,
    underline: style.underline,
    strikethrough: style.strikethrough,
    uppercase: style.uppercase,
    small_caps: style.small_caps,
    hidden: style.hidden,
    rotation_degrees: style.rotation_deg,
    color: common_rgb(style.color, style.opacity),
    outline_color: style
      .outline_color
      .map(|color| common_rgb(color, style.outline_opacity)),
    outline_width: common::Pt(style.outline_width_pt),
    highlight: style.highlight.map(|color| common_rgb(color, 1.0)),
    underline_color: style.underline_color.map(|color| common_rgb(color, 1.0)),
  }
}

pub(crate) fn common_border_style(style: BorderStyle) -> common::BorderStyle {
  common::BorderStyle {
    width: common::Pt(style.width_pt),
    spacing: common::Pt(style.spacing_pt),
    color: common_rgb(style.color, 1.0),
    compound: style.compound,
  }
}

pub(crate) fn common_stroke_from_border(
  style: BorderStyle,
  opacity: f32,
) -> common::Stroke<'static> {
  common::Stroke {
    width: common::Pt(style.width_pt),
    color: common_rgb(style.color, opacity),
    dash: style.dash_pattern.common_dash(style.width_pt),
    source_style_id: None,
    ..Default::default()
  }
}

pub(crate) fn common_rect(x: f32, y: f32, width: f32, height: f32) -> common::Rect {
  common::Rect {
    origin: common_point(x, y),
    size: common::Size {
      width: common::Pt(width),
      height: common::Pt(height),
    },
  }
}

pub(crate) fn common_point(x: f32, y: f32) -> common::Point {
  common::Point {
    x: common::Pt(x),
    y: common::Pt(y),
  }
}

pub(crate) fn common_rgb(color: RgbColor, opacity: f32) -> common::Color {
  common::Color {
    r: color.r,
    g: color.g,
    b: color.b,
    a: (opacity.clamp(0.0, 1.0) * 255.0).round() as u8,
  }
}
