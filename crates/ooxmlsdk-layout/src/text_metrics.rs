use std::sync::Arc;

use ooxmlsdk_fonts::{FeatureValue, TextScript};
use rustc_hash::FxHashMap as HashMap;
use skrifa::{
  GlyphId, MetadataProvider,
  instance::{LocationRef, Size},
  outline::{DrawSettings, HintingInstance, HintingOptions, pen::NullPen},
  raw::{FontRef, TableProvider, types::Tag},
};

use crate::fonts::{FontFaceCacheKey, FontFaceData, FontResolver, FontStyleRef};

/// Font-style view used only for automatic WordprocessingML escapement line
/// metrics. Shaping and painting keep the reduced size on the underlying
/// style; Writer likewise keeps the original ascent/height solely for line
/// formatting.
struct AutomaticEscapementMetricsStyle<'a, S: ?Sized> {
  style: &'a S,
  font_size_pt: f32,
  complex_font_size_pt: Option<f32>,
}

impl<S: FontStyleRef + ?Sized> FontStyleRef for AutomaticEscapementMetricsStyle<'_, S> {
  fn font_family(&self) -> Option<&str> {
    self.style.font_family()
  }

  fn fallback_font_family(&self) -> Option<&str> {
    self.style.fallback_font_family()
  }

  fn font_family_class(&self) -> Option<ooxmlsdk_fonts::FontFamilyClass> {
    self.style.font_family_class()
  }

  fn east_asia_font_family(&self) -> Option<&str> {
    self.style.east_asia_font_family()
  }

  fn complex_font_family(&self) -> Option<&str> {
    self.style.complex_font_family()
  }

  fn font_size_pt(&self) -> f32 {
    self.font_size_pt
  }

  fn complex_font_size_pt(&self) -> Option<f32> {
    self.complex_font_size_pt
  }

  fn complex_script_override(&self) -> Option<bool> {
    self.style.complex_script_override()
  }

  fn right_to_left(&self) -> bool {
    self.style.right_to_left()
  }

  fn resolved_bidi_level(&self) -> Option<u8> {
    self.style.resolved_bidi_level()
  }

  fn complex_bold(&self) -> Option<bool> {
    self.style.complex_bold()
  }

  fn complex_italic(&self) -> Option<bool> {
    self.style.complex_italic()
  }

  fn character_spacing_pt(&self) -> f32 {
    self.style.character_spacing_pt()
  }

  fn baseline_shift_pt(&self) -> f32 {
    self.style.baseline_shift_pt()
  }

  fn bold(&self) -> bool {
    self.style.bold()
  }

  fn italic(&self) -> bool {
    self.style.italic()
  }

  fn small_caps(&self) -> bool {
    self.style.small_caps()
  }

  fn kerning_enabled(&self) -> bool {
    self.style.kerning_enabled()
  }

  fn ligatures(&self) -> Option<crate::common::OpenTypeLigatures> {
    self.style.ligatures()
  }

  fn open_type_features(&self) -> crate::common::OpenTypeFeatureSettings {
    self.style.open_type_features()
  }

  fn horizontal_scale(&self) -> f32 {
    self.style.horizontal_scale()
  }

  fn wordprocessingml_font_slots(&self) -> bool {
    self.style.wordprocessingml_font_slots()
  }

  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    self.style.wordprocessingml_cjk_line_metrics()
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    self.style.cjk_punctuation_compression_ratio()
  }

  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    self
      .style
      .wordprocessingml_balance_single_byte_double_byte_width()
  }
}

// Last-resort vertical metrics when no usable font face can be loaded. Keep
// this out of horizontal measurement: LibreOffice and Typst both shape with
// real font data instead of estimating glyph advances by character class.
const FALLBACK_ASCENT_EM: f32 = 0.8;
const FALLBACK_DESCENT_EM: f32 = 0.2;
const FALLBACK_LINE_GAP_EM: f32 = 0.05;
// Word's `w:noLeading` DOC/DOCX compatibility metrics add this portion of the
// natural font height above and below every line painted with a face whose
// OS/2 code-page ranges advertise CP932/936/949/950. A 25pt DengXian control
// gives a 33.84pt single-line advance from a 26.05pt natural box and moves the
// first baseline down by 3.90pt, independently confirming the two 15% side
// bands. Writer's tdf#129808 path gates the same four code-page bits behind
// MS_WORD_COMP_GRID_METRICS and explicitly confirms Latin text as a control.
const WORDPROCESSINGML_CJK_SIDE_LEADING_RATIO: f32 = 0.15;
// FontMetricData::ImplInitTextLineSize.
const LO_TEXT_LINE_DESCENT_FALLBACK_DIVISOR: f32 = 10.0;
const LO_TEXT_LINE_MAX_DESCENT_DIVISOR: f32 = 3.0;
const LO_TEXT_LINE_WIDTH_FRACTION_OF_DESCENT: f32 = 0.25;
const LO_TEXT_LINE_MIN_WIDTH_PT: f32 = 1.0;
const LO_TEXT_LINE_WIDTH_HALF_DIVISOR: f32 = 2.0;
const LO_TEXT_LINE_STRIKEOUT_OFFSET_DIVISOR: f32 = 3.0;
const LO_TEXT_LINE_UNDERLINE_BASELINE_OFFSET_PT: f32 = 1.0;

#[derive(Clone, Debug)]
pub struct ShapedText {
  pub glyphs: Vec<ShapedGlyph>,
  pub font_faces: Vec<FontFaceData>,
  pub width_pt: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextVerticalMetrics {
  pub ascent_pt: f32,
  pub descent_pt: f32,
  pub line_gap_pt: f32,
  pub baseline_offset_pt: f32,
  pub directwrite_baseline_offset_pt: f32,
  pub wordprocessingml_cjk_line_metrics: bool,
}

impl TextVerticalMetrics {
  pub fn ink_height_pt(self) -> f32 {
    self.ascent_pt + self.descent_pt
  }

  pub fn line_height_pt(self) -> f32 {
    self.ink_height_pt() + self.line_gap_pt
  }

  pub fn leading_above_pt(self) -> f32 {
    self.line_gap_pt / 2.0
  }
}

#[derive(Clone, Copy, Debug)]
pub struct TextDecorationMetrics {
  pub underline_offset_pt: f32,
  pub underline_width_pt: f32,
  pub strikethrough_offset_pt: f32,
  pub strikethrough_width_pt: f32,
}

/// OpenType MATH constants scaled to the requested text size.
///
/// The `read-fonts` version re-exported by `skrifa` deliberately exposes an
/// untyped MATH table, so this reads the fixed MathConstants prefix directly
/// from the font-defined binary layout.  Values fall back to the OpenType
/// recommendations when the selected face has no usable MATH table.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MathFontMetrics {
  pub script_scale: f32,
  pub script_script_scale: f32,
  /// Effective OfficeMath minimum for display n-ary operators. Word uses the
  /// MATH table's `delimitedSubFormulaMinHeight` field here, despite the
  /// OpenType name `displayOperatorMinHeight` describing a different field.
  pub office_display_operator_min_height_pt: f32,
  pub math_leading_pt: f32,
  pub axis_height_pt: f32,
  pub accent_base_height_pt: f32,
  pub flattened_accent_base_height_pt: f32,
  pub subscript_shift_down_pt: f32,
  pub subscript_top_max_pt: f32,
  pub subscript_baseline_drop_min_pt: f32,
  pub superscript_shift_up_pt: f32,
  pub superscript_shift_up_cramped_pt: f32,
  pub superscript_bottom_min_pt: f32,
  pub superscript_baseline_drop_max_pt: f32,
  pub sub_superscript_gap_min_pt: f32,
  pub superscript_bottom_max_with_subscript_pt: f32,
  pub space_after_script_pt: f32,
  pub upper_limit_gap_min_pt: f32,
  pub upper_limit_baseline_rise_min_pt: f32,
  pub lower_limit_gap_min_pt: f32,
  pub lower_limit_baseline_drop_min_pt: f32,
  pub stack_top_shift_up_pt: f32,
  pub stack_top_display_style_shift_up_pt: f32,
  pub stack_bottom_shift_down_pt: f32,
  pub stack_bottom_display_style_shift_down_pt: f32,
  pub stack_gap_min_pt: f32,
  pub stack_display_style_gap_min_pt: f32,
  pub fraction_numerator_shift_up_pt: f32,
  pub fraction_numerator_display_style_shift_up_pt: f32,
  pub fraction_denominator_shift_down_pt: f32,
  pub fraction_denominator_display_style_shift_down_pt: f32,
  pub fraction_numerator_gap_min_pt: f32,
  pub fraction_num_display_style_gap_min_pt: f32,
  pub fraction_rule_thickness_pt: f32,
  pub fraction_denominator_gap_min_pt: f32,
  pub fraction_denom_display_style_gap_min_pt: f32,
  pub skewed_fraction_horizontal_gap_pt: f32,
  pub skewed_fraction_vertical_gap_pt: f32,
  pub overbar_vertical_gap_pt: f32,
  pub overbar_rule_thickness_pt: f32,
  pub underbar_vertical_gap_pt: f32,
  pub underbar_rule_thickness_pt: f32,
  pub radical_vertical_gap_pt: f32,
  pub radical_display_style_vertical_gap_pt: f32,
  pub radical_rule_thickness_pt: f32,
  pub radical_extra_ascender_pt: f32,
  pub radical_kern_before_degree_pt: f32,
  pub radical_kern_after_degree_pt: f32,
  pub radical_degree_bottom_raise_percent: f32,
}

impl MathFontMetrics {
  fn recommended(font_size_pt: f32) -> Self {
    let em = font_size_pt.max(1.0);
    let rule = (em * 0.04).max(0.4);
    Self {
      script_scale: 0.8,
      script_script_scale: 0.6,
      // OpenType suggests normal line height × 1.5 for a delimited
      // sub-formula minimum. This fallback is used only when no MATH table
      // is available; a real MATH face supplies the authored value below.
      // Keep the pre-MATH renderer default for faces without an explicit
      // OfficeMath constant; only a real MATH table supplies the Word-specific
      // `delimitedSubFormulaMinHeight` value below.
      office_display_operator_min_height_pt: em * 1.3,
      // OpenType defines MathLeading as font-authored whitespace between
      // formulas. A face without a usable MATH table has no such authored
      // whitespace; its ordinary text line metrics remain authoritative.
      math_leading_pt: 0.0,
      axis_height_pt: em * 0.25,
      // OpenType recommends the face's x-height and cap height. These ratios
      // are used only when no face can be resolved at all; a resolved
      // MATH-less face is completed from its actual OS/2 metrics below.
      accent_base_height_pt: em * 0.45,
      flattened_accent_base_height_pt: em * 0.7,
      subscript_shift_down_pt: em * 0.2,
      subscript_top_max_pt: em * 0.36,
      subscript_baseline_drop_min_pt: 0.0,
      superscript_shift_up_pt: em * 0.36,
      superscript_shift_up_cramped_pt: 0.0,
      superscript_bottom_min_pt: em * 0.1125,
      superscript_baseline_drop_max_pt: 0.0,
      sub_superscript_gap_min_pt: em * 0.2,
      superscript_bottom_max_with_subscript_pt: em * 0.36,
      space_after_script_pt: em * 0.05,
      upper_limit_gap_min_pt: em * 0.12,
      upper_limit_baseline_rise_min_pt: 0.0,
      lower_limit_gap_min_pt: em * 0.12,
      lower_limit_baseline_drop_min_pt: 0.0,
      // MathML Core's MATH-less fallback uses zero preferred stack shifts;
      // the minimum gaps below then determine a non-overlapping placement.
      stack_top_shift_up_pt: 0.0,
      stack_top_display_style_shift_up_pt: 0.0,
      stack_bottom_shift_down_pt: 0.0,
      stack_bottom_display_style_shift_down_pt: 0.0,
      stack_gap_min_pt: rule * 3.0,
      stack_display_style_gap_min_pt: rule * 7.0,
      fraction_numerator_shift_up_pt: em * 0.4,
      fraction_numerator_display_style_shift_up_pt: em * 0.4,
      fraction_denominator_shift_down_pt: em * 0.4,
      fraction_denominator_display_style_shift_down_pt: em * 0.4,
      fraction_numerator_gap_min_pt: em * 0.12,
      fraction_num_display_style_gap_min_pt: rule * 3.0,
      fraction_rule_thickness_pt: rule,
      fraction_denominator_gap_min_pt: em * 0.12,
      fraction_denom_display_style_gap_min_pt: rule * 3.0,
      // MathML Core does not supply skewed-fraction fallback constants;
      // Typst's documented MATH-less fallback keeps zero vertical gap and a
      // half-em horizontal gap.
      skewed_fraction_horizontal_gap_pt: em * 0.5,
      skewed_fraction_vertical_gap_pt: 0.0,
      overbar_vertical_gap_pt: em * 0.08,
      overbar_rule_thickness_pt: (em * 0.04).max(0.4),
      underbar_vertical_gap_pt: em * 0.08,
      underbar_rule_thickness_pt: (em * 0.04).max(0.4),
      radical_vertical_gap_pt: em * 0.08,
      // OpenType recommends the default rule thickness plus one quarter of
      // the face's x-height. The resolved face path below replaces this
      // provisional x-height ratio with the actual OS/2 metric.
      radical_display_style_vertical_gap_pt: rule + em * 0.45 * 0.25,
      radical_rule_thickness_pt: (em * 0.04).max(0.4),
      radical_extra_ascender_pt: em * 0.08,
      radical_kern_before_degree_pt: em * 0.04,
      radical_kern_after_degree_pt: -em * 0.1,
      radical_degree_bottom_raise_percent: 60.0,
    }
  }
}

#[derive(Clone, Debug)]
pub struct ShapedGlyph {
  pub font_index: usize,
  pub font_size_pt: f32,
  pub glyph_id: u32,
  pub text_range: std::ops::Range<usize>,
  pub x_advance_em: f32,
  pub x_offset_em: f32,
  pub y_offset_em: f32,
  pub y_advance_em: f32,
  pub bounds_em: Option<ShapedGlyphBounds>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ShapedGlyphBounds {
  pub x_min_em: f32,
  pub y_min_em: f32,
  pub x_max_em: f32,
  pub y_max_em: f32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MeasureStyleKey {
  font_family: Option<Box<str>>,
  high_ansi_font_family: Option<Box<str>>,
  fallback_font_family: Option<Box<str>>,
  high_ansi_fallback_font_family: Option<Box<str>>,
  east_asia_fallback_font_family: Option<Box<str>>,
  complex_fallback_font_family: Option<Box<str>>,
  font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  high_ansi_font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  east_asia_font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  complex_font_family_class: Option<ooxmlsdk_fonts::FontFamilyClass>,
  east_asia_font_family: Option<Box<str>>,
  complex_font_family: Option<Box<str>>,
  font_size_bits: u32,
  complex_font_size_bits: Option<u32>,
  complex_script_override: Option<bool>,
  right_to_left: bool,
  resolved_bidi_level: Option<u8>,
  character_spacing_bits: u32,
  horizontal_scale_bits: u32,
  bold: bool,
  italic: bool,
  complex_bold: Option<bool>,
  complex_italic: Option<bool>,
  small_caps: bool,
  kerning_enabled: bool,
  wordprocessingml_font_slots: bool,
  wordprocessingml_cjk_line_metrics: bool,
  wordprocessingml_font_hint: Option<ooxmlsdk_fonts::WordprocessingFontTypeHint>,
  wordprocessingml_east_asia_language_is_chinese: bool,
  font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  high_ansi_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  wordprocessingml_east_asia_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  complex_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  high_ansi_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  east_asia_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  complex_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  cjk_punctuation_compression_ratio_bits: u32,
  wordprocessingml_balance_single_byte_double_byte_width: bool,
}

impl MeasureStyleKey {
  fn from_style(style: &(impl FontStyleRef + ?Sized)) -> Self {
    Self {
      font_family: style.font_family().map(Into::into),
      high_ansi_font_family: style.high_ansi_font_family().map(Into::into),
      fallback_font_family: style.fallback_font_family().map(Into::into),
      high_ansi_fallback_font_family: style.high_ansi_fallback_font_family().map(Into::into),
      east_asia_fallback_font_family: style.east_asia_fallback_font_family().map(Into::into),
      complex_fallback_font_family: style.complex_fallback_font_family().map(Into::into),
      font_family_class: style.font_family_class(),
      high_ansi_font_family_class: style.high_ansi_font_family_class(),
      east_asia_font_family_class: style.east_asia_font_family_class(),
      complex_font_family_class: style.complex_font_family_class(),
      east_asia_font_family: style.east_asia_font_family().map(Into::into),
      complex_font_family: style.complex_font_family().map(Into::into),
      font_size_bits: style.font_size_pt().to_bits(),
      complex_font_size_bits: style.complex_font_size_pt().map(f32::to_bits),
      complex_script_override: style.complex_script_override(),
      right_to_left: style.right_to_left(),
      resolved_bidi_level: style.resolved_bidi_level(),
      character_spacing_bits: style.character_spacing_pt().to_bits(),
      horizontal_scale_bits: style.horizontal_scale().to_bits(),
      bold: style.bold(),
      italic: style.italic(),
      complex_bold: style.complex_bold(),
      complex_italic: style.complex_italic(),
      small_caps: style.small_caps(),
      kerning_enabled: style.kerning_enabled(),
      wordprocessingml_font_slots: style.wordprocessingml_font_slots(),
      wordprocessingml_cjk_line_metrics: style.wordprocessingml_cjk_line_metrics(),
      wordprocessingml_font_hint: style.wordprocessingml_font_hint(),
      wordprocessingml_east_asia_language_is_chinese: style
        .wordprocessingml_east_asia_language_is_chinese(),
      font_charset: style.font_charset(),
      high_ansi_font_charset: style.high_ansi_font_charset(),
      wordprocessingml_east_asia_font_charset: style.wordprocessingml_east_asia_font_charset(),
      complex_font_charset: style.complex_font_charset(),
      font_pitch: style.font_pitch(),
      high_ansi_font_pitch: style.high_ansi_font_pitch(),
      east_asia_font_pitch: style.east_asia_font_pitch(),
      complex_font_pitch: style.complex_font_pitch(),
      cjk_punctuation_compression_ratio_bits: style.cjk_punctuation_compression_ratio().to_bits(),
      wordprocessingml_balance_single_byte_double_byte_width: style
        .wordprocessingml_balance_single_byte_double_byte_width(),
    }
  }

  fn matches(&self, style: &(impl FontStyleRef + ?Sized)) -> bool {
    self.font_family.as_deref() == style.font_family()
      && self.high_ansi_font_family.as_deref() == style.high_ansi_font_family()
      && self.fallback_font_family.as_deref() == style.fallback_font_family()
      && self.high_ansi_fallback_font_family.as_deref() == style.high_ansi_fallback_font_family()
      && self.east_asia_fallback_font_family.as_deref() == style.east_asia_fallback_font_family()
      && self.complex_fallback_font_family.as_deref() == style.complex_fallback_font_family()
      && self.font_family_class == style.font_family_class()
      && self.high_ansi_font_family_class == style.high_ansi_font_family_class()
      && self.east_asia_font_family_class == style.east_asia_font_family_class()
      && self.complex_font_family_class == style.complex_font_family_class()
      && self.east_asia_font_family.as_deref() == style.east_asia_font_family()
      && self.complex_font_family.as_deref() == style.complex_font_family()
      && self.font_size_bits == style.font_size_pt().to_bits()
      && self.complex_font_size_bits == style.complex_font_size_pt().map(f32::to_bits)
      && self.complex_script_override == style.complex_script_override()
      && self.right_to_left == style.right_to_left()
      && self.resolved_bidi_level == style.resolved_bidi_level()
      && self.character_spacing_bits == style.character_spacing_pt().to_bits()
      && self.horizontal_scale_bits == style.horizontal_scale().to_bits()
      && self.bold == style.bold()
      && self.italic == style.italic()
      && self.complex_bold == style.complex_bold()
      && self.complex_italic == style.complex_italic()
      && self.small_caps == style.small_caps()
      && self.kerning_enabled == style.kerning_enabled()
      && self.wordprocessingml_font_slots == style.wordprocessingml_font_slots()
      && self.wordprocessingml_cjk_line_metrics == style.wordprocessingml_cjk_line_metrics()
      && self.wordprocessingml_font_hint == style.wordprocessingml_font_hint()
      && self.wordprocessingml_east_asia_language_is_chinese
        == style.wordprocessingml_east_asia_language_is_chinese()
      && self.font_charset == style.font_charset()
      && self.high_ansi_font_charset == style.high_ansi_font_charset()
      && self.wordprocessingml_east_asia_font_charset
        == style.wordprocessingml_east_asia_font_charset()
      && self.complex_font_charset == style.complex_font_charset()
      && self.font_pitch == style.font_pitch()
      && self.high_ansi_font_pitch == style.high_ansi_font_pitch()
      && self.east_asia_font_pitch == style.east_asia_font_pitch()
      && self.complex_font_pitch == style.complex_font_pitch()
      && self.cjk_punctuation_compression_ratio_bits
        == style.cjk_punctuation_compression_ratio().to_bits()
      && self.wordprocessingml_balance_single_byte_double_byte_width
        == style.wordprocessingml_balance_single_byte_double_byte_width()
  }
}

#[derive(Debug, Default)]
pub struct TextMetrics {
  fonts: FontResolver,
  measure_styles: Vec<MeasureStyleKey>,
  measure_widths: Vec<HashMap<Arc<str>, f32>>,
  gdi_hinted_extents: Vec<HashMap<u32, HashMap<Arc<str>, Option<f32>>>>,
  gdi_hinting_instances: GdiHintingInstanceCache,
  last_measure_style: Option<usize>,
}

#[derive(Default)]
struct GdiHintingInstanceCache {
  instances: HashMap<(FontFaceCacheKey, u8), HintingInstance>,
}

impl std::fmt::Debug for GdiHintingInstanceCache {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    formatter
      .debug_struct("GdiHintingInstanceCache")
      .field("len", &self.instances.len())
      .finish()
  }
}

impl TextMetrics {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn into_font_resolver(self) -> FontResolver {
    self.fonts
  }

  pub fn measure_text(&mut self, text: &str, style: &(impl FontStyleRef + ?Sized)) -> f32 {
    if text.is_empty() {
      return 0.0;
    }

    let style_index = self.measure_style_index(style);
    if let Some(width) = self.measure_widths[style_index].get(text) {
      return *width;
    }
    let width = self
      .shape_text(text, style)
      .map_or(0.0, |shaped| shaped.width_pt);
    self.measure_widths[style_index].insert(Arc::from(text), width);
    width
  }

  fn measure_style_index(&mut self, style: &(impl FontStyleRef + ?Sized)) -> usize {
    if let Some(index) = self.last_measure_style
      && self.measure_styles[index].matches(style)
    {
      return index;
    }
    if let Some(index) = self
      .measure_styles
      .iter()
      .position(|key| key.matches(style))
    {
      self.last_measure_style = Some(index);
      return index;
    }
    let index = self.measure_styles.len();
    self.measure_styles.push(MeasureStyleKey::from_style(style));
    self.measure_widths.push(HashMap::default());
    self.gdi_hinted_extents.push(HashMap::default());
    self.last_measure_style = Some(index);
    index
  }

  pub fn shape_text(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<ShapedText> {
    if text.is_empty() {
      return Some(ShapedText {
        glyphs: Vec::new(),
        font_faces: Vec::new(),
        width_pt: 0.0,
      });
    }

    let runs = self.fonts.shape_text_runs(text, style)?;
    shaped_text_from_runs(runs, |font_id| self.fonts.font_face_data(font_id))
  }

  pub(crate) fn shape_text_with_features(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    features: &[FeatureValue<'_>],
  ) -> Option<ShapedText> {
    if text.is_empty() {
      return Some(ShapedText {
        glyphs: Vec::new(),
        font_faces: Vec::new(),
        width_pt: 0.0,
      });
    }

    let runs = self
      .fonts
      .shape_text_runs_with_features(text, style, features)?;
    shaped_text_from_runs(runs, |font_id| self.fonts.font_face_data(font_id))
  }

  /// Returns the integer device advances used by classic GDI for a simple
  /// one-glyph-per-character text run.
  ///
  /// TrueType `hdmx` records contain the hinted advance of every glyph at an
  /// exact integer ppem. When that optional table is absent, classic GDI still
  /// exposes an integer device width: Wine's `get_advance_metric` rounds the
  /// scaled advance up to the next 26.6 pixel boundary before
  /// `GetTextExtentExPoint` accumulates it. Complex clusters deliberately fall
  /// back to the normal shaping path.
  pub(crate) fn gdi_device_character_advances_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    device_dpi: f32,
  ) -> Option<Arc<[f32]>> {
    if text.is_empty()
      || !device_dpi.is_finite()
      || device_dpi <= 0.0
      || style.character_spacing_pt().abs() > f32::EPSILON
      || (style.horizontal_scale() - 1.0).abs() > f32::EPSILON
    {
      return None;
    }

    let shaped = self.shape_text(text, style)?;
    let character_indices = one_glyph_per_character_indices(text, &shaped.glyphs)?;

    let mut advances = vec![None; character_indices.len()];
    for (glyph, character_index) in shaped.glyphs.iter().zip(character_indices) {
      let face = shaped.font_faces.get(glyph.font_index)?;
      let font = FontRef::from_index(face.data.as_ref(), face.index).ok()?;
      let ppem = gdi_device_ppem(glyph.font_size_pt, device_dpi)?;
      let glyph_index = usize::try_from(glyph.glyph_id).ok()?;
      let advance_pt = font
        .hdmx()
        .ok()
        .and_then(|table| table.record_for_size(ppem))
        .and_then(|record| record.widths().get(glyph_index).copied())
        .map(|width_px| gdi_device_advance_pt(width_px, device_dpi))
        .or_else(|| {
          gdi_scaled_device_advance_pt(glyph.x_advance_em * glyph.font_size_pt, device_dpi)
        })?;
      advances[character_index] = Some(advance_pt);
    }

    advances
      .into_iter()
      .collect::<Option<Vec<_>>>()
      .map(Arc::from)
  }

  /// Returns the cumulative FreeType-compatible hinted extent for a simple
  /// text run while preserving shaping adjustments such as kerning.
  ///
  /// Classic GDI loads each glyph with `FT_LOAD_DEFAULT`, accumulates its
  /// hinted 26.6 advance, and rounds the complete extent for
  /// `GetTextExtentExPoint`. Skrifa exposes the same adjusted advance through
  /// its embedded TrueType hinter. Unsupported clusters and synthesized bold
  /// faces stay on the ordinary shaping path.
  pub(crate) fn gdi_hinted_text_extent_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    device_dpi: f32,
  ) -> Option<f32> {
    if text.is_empty() {
      return Some(0.0);
    }
    if !device_dpi.is_finite()
      || device_dpi <= 0.0
      || style.character_spacing_pt().abs() > f32::EPSILON
      || (style.horizontal_scale() - 1.0).abs() > f32::EPSILON
    {
      return None;
    }

    let style_index = self.measure_style_index(style);
    let device_dpi_bits = device_dpi.to_bits();
    if let Some(cached) = self.gdi_hinted_extents[style_index]
      .get(&device_dpi_bits)
      .and_then(|extents| extents.get(text))
    {
      return *cached;
    }

    let extent = self.compute_gdi_hinted_text_extent_pt(text, style, device_dpi);
    let text_key = self.measure_widths[style_index]
      .get_key_value(text)
      .map_or_else(|| Arc::from(text), |(text, _)| text.clone());
    self.gdi_hinted_extents[style_index]
      .entry(device_dpi_bits)
      .or_default()
      .insert(text_key, extent);
    extent
  }

  fn compute_gdi_hinted_text_extent_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    device_dpi: f32,
  ) -> Option<f32> {
    let shaped = self.shape_text(text, style)?;
    if shaped.font_faces.iter().any(|face| face.synthetic_bold) {
      return None;
    }
    one_glyph_per_character_indices(text, &shaped.glyphs)?;

    for glyph in &shaped.glyphs {
      let ppem = gdi_device_ppem(glyph.font_size_pt, device_dpi)?;
      let face = shaped.font_faces.get(glyph.font_index)?;
      let key = (face.cache_key(), ppem);
      if self.gdi_hinting_instances.instances.contains_key(&key) {
        continue;
      }
      let font = FontRef::from_index(face.data.as_ref(), face.index).ok()?;
      let outlines = font.outline_glyphs();
      let instance = HintingInstance::new(
        &outlines,
        Size::new(f32::from(ppem)),
        LocationRef::default(),
        HintingOptions::default(),
      )
      .ok()?;
      // A configured TrueType hinter owns the font-level fpgm/prep state and
      // can hint any number of glyphs for the same face and ppem. Recreating
      // it per run repeats those programs for every line-fit probe.
      self.gdi_hinting_instances.instances.insert(key, instance);
    }

    let points_per_device_pixel = crate::units::POINTS_PER_INCH / device_dpi;
    let mut width_pt = shaped.width_pt;
    for glyph in &shaped.glyphs {
      let ppem = gdi_device_ppem(glyph.font_size_pt, device_dpi)?;
      let face = shaped.font_faces.get(glyph.font_index)?;
      let key = (face.cache_key(), ppem);
      let instance = self.gdi_hinting_instances.instances.get(&key)?;
      let font = FontRef::from_index(face.data.as_ref(), face.index).ok()?;
      let outline = font.outline_glyphs().get(GlyphId::new(glyph.glyph_id))?;
      let unhinted_advance = outline
        .draw(
          DrawSettings::unhinted(Size::new(f32::from(ppem)), LocationRef::default()),
          &mut NullPen,
        )
        .ok()?
        .advance_width?;
      let hinted_advance = outline
        .draw(DrawSettings::hinted(instance, false), &mut NullPen)
        .ok()?
        .advance_width?;
      width_pt += (hinted_advance - unhinted_advance) * points_per_device_pixel;
    }
    let width_device_pixels = width_pt / points_per_device_pixel;
    width_device_pixels
      .is_finite()
      .then_some(width_device_pixels.round() * points_per_device_pixel)
  }

  /// Returns a uniform spacing adjustment when classic GDI's hinted device
  /// advances differ from the shaped advances by the same amount for every
  /// character. Proportional differences, clusters, and fonts without an
  /// exact `hdmx` record deliberately remain on the normal shaping path.
  pub(crate) fn gdi_uniform_device_character_spacing_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    device_dpi: f32,
  ) -> Option<f32> {
    let device_advances = self.gdi_device_character_advances_pt(text, style, device_dpi)?;
    let shaped = self.shape_text(text, style)?;
    let character_indices = one_glyph_per_character_indices(text, &shaped.glyphs)?;
    let mut natural_advances = vec![None; character_indices.len()];
    for (glyph, character_index) in shaped.glyphs.iter().zip(character_indices) {
      natural_advances[character_index] = Some(glyph.x_advance_em * glyph.font_size_pt);
    }
    let natural_advances = natural_advances.into_iter().collect::<Option<Vec<_>>>()?;
    uniform_character_spacing_from_advances(&natural_advances, &device_advances)
  }

  pub fn vertical_metrics(&mut self, style: &(impl FontStyleRef + ?Sized)) -> TextVerticalMetrics {
    self
      .fonts
      .vertical_metrics(style)
      .map(text_vertical_metrics_from_font_metrics)
      .unwrap_or_else(|| approximate_vertical_metrics(style.font_size_pt()))
  }

  pub(crate) fn vertical_metrics_for_script(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: TextScript,
  ) -> TextVerticalMetrics {
    self
      .fonts
      .vertical_metrics_for_script(style, script)
      .or_else(|| self.fonts.vertical_metrics(style))
      .map(text_vertical_metrics_from_font_metrics)
      .unwrap_or_else(|| approximate_vertical_metrics(style.font_size_pt()))
  }

  pub fn vertical_metrics_for_text(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> TextVerticalMetrics {
    self
      .fonts
      .text_vertical_metrics(text, style)
      .or_else(|| self.fonts.vertical_metrics(style))
      .map(text_vertical_metrics_from_font_metrics)
      .unwrap_or_else(|| approximate_vertical_metrics(style.font_size_pt()))
  }

  pub fn text_decoration_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> TextDecorationMetrics {
    self
      .fonts
      .decoration_metrics(style)
      .and_then(|metrics| {
        (metrics.underline_thickness_pt > f32::EPSILON
          && metrics.strikeout_thickness_pt > f32::EPSILON)
          .then_some(TextDecorationMetrics {
            underline_offset_pt: metrics.underline_offset_pt,
            underline_width_pt: metrics.underline_thickness_pt,
            strikethrough_offset_pt: metrics.strikeout_offset_pt,
            strikethrough_width_pt: metrics.strikeout_thickness_pt,
          })
      })
      .unwrap_or_else(|| approximate_decoration_metrics(style.font_size_pt()))
  }

  pub(crate) fn math_font_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> MathFontMetrics {
    let fallback = MathFontMetrics::recommended(style.font_size_pt());
    self
      .fonts
      .with_cached_text_face(style, |face| {
        math_font_metrics_from_face(face, style.font_size_pt())
      })
      .flatten()
      .unwrap_or(fallback)
  }

  pub fn baseline_offset_in_line(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    line_height_pt: f32,
  ) -> f32 {
    baseline_offset_in_line_from_metrics(self.line_vertical_metrics(style), style, line_height_pt)
  }

  pub fn baseline_offset_in_line_for_text(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    line_height_pt: f32,
  ) -> f32 {
    baseline_offset_in_line_from_metrics(
      self.line_vertical_metrics_for_text(text, style),
      style,
      line_height_pt,
    )
  }

  pub fn baseline_offset_in_line_with_windows_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    line_height_pt: f32,
  ) -> f32 {
    baseline_offset_in_line_with_windows_metrics_from_metrics(
      self.line_vertical_metrics(style),
      style,
      line_height_pt,
    )
  }

  pub fn baseline_offset_in_line_with_windows_metrics_for_text(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    line_height_pt: f32,
  ) -> f32 {
    baseline_offset_in_line_with_windows_metrics_from_metrics(
      self.line_vertical_metrics_for_text(text, style),
      style,
      line_height_pt,
    )
  }

  pub fn inline_text_box_height(&mut self, style: &(impl FontStyleRef + ?Sized)) -> f32 {
    let automatic_escapement = style.automatic_escapement_font_sizes_pt().is_some();
    self.line_vertical_metrics(style).line_height_pt()
      + if automatic_escapement {
        0.0
      } else {
        style.baseline_shift_pt().abs()
      }
  }

  pub fn inline_text_box_height_for_text(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> f32 {
    let automatic_escapement = style.automatic_escapement_font_sizes_pt().is_some();
    self.line_text_height(text, style)
      + if automatic_escapement {
        0.0
      } else {
        style.baseline_shift_pt().abs()
      }
  }

  pub fn line_vertical_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> TextVerticalMetrics {
    let metrics = if let Some((font_size_pt, complex_font_size_pt)) =
      style.automatic_escapement_font_sizes_pt()
    {
      self.vertical_metrics(&AutomaticEscapementMetricsStyle {
        style,
        font_size_pt,
        complex_font_size_pt,
      })
    } else {
      self.vertical_metrics(style)
    };
    wordprocessingml_line_vertical_metrics(style, metrics)
  }

  pub fn line_vertical_metrics_for_text(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> TextVerticalMetrics {
    let metrics = if let Some((font_size_pt, complex_font_size_pt)) =
      style.automatic_escapement_font_sizes_pt()
    {
      self.vertical_metrics_for_text(
        text,
        &AutomaticEscapementMetricsStyle {
          style,
          font_size_pt,
          complex_font_size_pt,
        },
      )
    } else {
      self.vertical_metrics_for_text(text, style)
    };
    wordprocessingml_line_vertical_metrics(style, metrics)
  }

  fn line_text_height(&mut self, text: &str, style: &(impl FontStyleRef + ?Sized)) -> f32 {
    self
      .line_vertical_metrics_for_text(text, style)
      .line_height_pt()
  }
}

fn wordprocessingml_line_vertical_metrics(
  style: &(impl FontStyleRef + ?Sized),
  mut metrics: TextVerticalMetrics,
) -> TextVerticalMetrics {
  // Keep the physical font metrics available to callers that are sizing an
  // implicit paragraph mark, drawing, or other non-line geometry. Word's
  // DOC/DOCX CJK compatibility leading belongs to formatted text lines only.
  // In particular, tscp.docx selects an East Asian face for its legacy
  // automatic-superscript paragraph mark while its visible Latin line keeps
  // the unscaled paragraph-mark box.
  if !style.wordprocessingml_font_slots()
    || !style.wordprocessingml_cjk_line_metrics()
    || !metrics.wordprocessingml_cjk_line_metrics
  {
    return metrics;
  }

  let side_leading_pt = metrics.ink_height_pt() * WORDPROCESSINGML_CJK_SIDE_LEADING_RATIO;
  metrics.ascent_pt += side_leading_pt;
  metrics.descent_pt += side_leading_pt;
  metrics.baseline_offset_pt += side_leading_pt;
  metrics.directwrite_baseline_offset_pt += side_leading_pt;
  metrics
}

fn text_vertical_metrics_from_font_metrics(
  metrics: ooxmlsdk_fonts::VerticalMetrics,
) -> TextVerticalMetrics {
  TextVerticalMetrics {
    ascent_pt: metrics.ascent_pt,
    descent_pt: metrics.descent_pt,
    line_gap_pt: metrics.line_gap_pt,
    baseline_offset_pt: metrics.baseline_offset_pt,
    directwrite_baseline_offset_pt: metrics.directwrite_baseline_offset_pt,
    wordprocessingml_cjk_line_metrics: metrics.wordprocessingml_cjk_line_metrics,
  }
}

fn gdi_device_ppem(font_size_pt: f32, device_dpi: f32) -> Option<u8> {
  let ppem = (font_size_pt * device_dpi / crate::units::POINTS_PER_INCH).round();
  (ppem.is_finite() && (1.0..=f32::from(u8::MAX)).contains(&ppem)).then_some(ppem as u8)
}

fn gdi_device_advance_pt(width_px: u8, device_dpi: f32) -> f32 {
  f32::from(width_px) * crate::units::POINTS_PER_INCH / device_dpi
}

fn gdi_scaled_device_advance_pt(natural_advance_pt: f32, device_dpi: f32) -> Option<f32> {
  let device_advance = natural_advance_pt * device_dpi / crate::units::POINTS_PER_INCH;
  if !device_advance.is_finite() || device_advance < 0.0 {
    return None;
  }
  let nearest_device_pixel = device_advance.round();
  let integer_device_advance = if (device_advance - nearest_device_pixel).abs() <= 1.0e-4 {
    nearest_device_pixel
  } else {
    device_advance.ceil()
  };
  Some(integer_device_advance * crate::units::POINTS_PER_INCH / device_dpi)
}

fn uniform_character_spacing_from_advances(
  natural_advances: &[f32],
  device_advances: &[f32],
) -> Option<f32> {
  const UNIFORM_SPACING_EPSILON_PT: f32 = 1.0e-4;

  if natural_advances.len() < 2 || natural_advances.len() != device_advances.len() {
    return None;
  }
  let spacing = device_advances[0] - natural_advances[0];
  if !spacing.is_finite() || spacing.abs() <= UNIFORM_SPACING_EPSILON_PT {
    return None;
  }
  natural_advances
    .iter()
    .zip(device_advances)
    .all(|(natural, device)| {
      natural.is_finite()
        && device.is_finite()
        && ((device - natural) - spacing).abs() <= UNIFORM_SPACING_EPSILON_PT
    })
    .then_some(spacing)
}

fn math_font_metrics_from_face(face: &FontFaceData, font_size_pt: f32) -> Option<MathFontMetrics> {
  let font = FontRef::from_index(face.data.as_ref(), face.index).ok()?;
  let units_per_em = f32::from(font.head().ok()?.units_per_em()).max(1.0);
  let font_size_pt = font_size_pt.max(1.0);
  let mut fallback = MathFontMetrics::recommended(font_size_pt);
  let global_metrics = font.metrics(Size::new(font_size_pt), LocationRef::default());
  let x_height_pt = global_metrics
    .x_height
    .filter(|value| *value > 0.0)
    .unwrap_or(fallback.accent_base_height_pt);
  fallback.accent_base_height_pt = x_height_pt;
  fallback.flattened_accent_base_height_pt = global_metrics
    .cap_height
    .filter(|value| *value > 0.0)
    .unwrap_or(fallback.flattened_accent_base_height_pt);
  // OpenType's recommendations for the side-script ink constraints are
  // expressed in terms of the selected face's x-height. Complete a MATH-less
  // face from that authored metric rather than from the requested em size.
  fallback.subscript_top_max_pt = x_height_pt * 0.8;
  fallback.superscript_bottom_min_pt = x_height_pt * 0.25;
  fallback.superscript_bottom_max_with_subscript_pt = x_height_pt * 0.8;
  fallback.radical_display_style_vertical_gap_pt =
    fallback.radical_rule_thickness_pt + x_height_pt * 0.25;
  let Some(table) = font.table_data(Tag::new(b"MATH")) else {
    return Some(fallback);
  };
  let bytes = table.as_bytes();
  let constants_offset = usize::from(be_u16(bytes, 4)?);
  let scale = font_size_pt / units_per_em;
  let value = |index: usize| -> Option<f32> {
    be_i16(
      bytes,
      constants_offset.checked_add(8 + index.checked_mul(4)?)?,
    )
    .map(|value| f32::from(value) * scale)
  };
  let percentage = |offset: usize| -> Option<f32> {
    be_i16(bytes, constants_offset.checked_add(offset)?)
      .map(|value| f32::from(value) / 100.0)
      .filter(|value| *value > 0.0 && *value <= 1.0)
  };

  Some(MathFontMetrics {
    script_scale: percentage(0).unwrap_or(0.8),
    script_script_scale: percentage(2).unwrap_or(0.6),
    // Word selects display n-ary variants with delimitedSubFormulaMinHeight
    // (the third MathConstants field), not displayOperatorMinHeight (the
    // fourth field prescribed by OpenType).
    office_display_operator_min_height_pt: f32::from(be_u16(bytes, constants_offset + 4)?) * scale,
    math_leading_pt: value(0)?,
    axis_height_pt: value(1)?,
    accent_base_height_pt: value(2)?,
    flattened_accent_base_height_pt: value(3)?,
    subscript_shift_down_pt: value(4)?,
    subscript_top_max_pt: value(5)?,
    subscript_baseline_drop_min_pt: value(6)?,
    superscript_shift_up_pt: value(7)?,
    superscript_shift_up_cramped_pt: value(8)?,
    superscript_bottom_min_pt: value(9)?,
    superscript_baseline_drop_max_pt: value(10)?,
    sub_superscript_gap_min_pt: value(11)?,
    superscript_bottom_max_with_subscript_pt: value(12)?,
    space_after_script_pt: value(13)?,
    upper_limit_gap_min_pt: value(14)?,
    upper_limit_baseline_rise_min_pt: value(15)?,
    lower_limit_gap_min_pt: value(16)?,
    lower_limit_baseline_drop_min_pt: value(17)?,
    stack_top_shift_up_pt: value(18)?,
    stack_top_display_style_shift_up_pt: value(19)?,
    stack_bottom_shift_down_pt: value(20)?,
    stack_bottom_display_style_shift_down_pt: value(21)?,
    stack_gap_min_pt: value(22)?,
    stack_display_style_gap_min_pt: value(23)?,
    fraction_numerator_shift_up_pt: value(28)?,
    fraction_numerator_display_style_shift_up_pt: value(29)?,
    fraction_denominator_shift_down_pt: value(30)?,
    fraction_denominator_display_style_shift_down_pt: value(31)?,
    fraction_numerator_gap_min_pt: value(32)?,
    fraction_num_display_style_gap_min_pt: value(33)?,
    fraction_rule_thickness_pt: value(34)?.max(0.2),
    fraction_denominator_gap_min_pt: value(35)?,
    fraction_denom_display_style_gap_min_pt: value(36)?,
    skewed_fraction_horizontal_gap_pt: value(37)?,
    skewed_fraction_vertical_gap_pt: value(38)?,
    overbar_vertical_gap_pt: value(39)?,
    overbar_rule_thickness_pt: value(40)?.max(0.2),
    underbar_vertical_gap_pt: value(42)?,
    underbar_rule_thickness_pt: value(43)?.max(0.2),
    radical_vertical_gap_pt: value(45)?,
    radical_display_style_vertical_gap_pt: value(46)?,
    radical_rule_thickness_pt: value(47)?.max(0.2),
    radical_extra_ascender_pt: value(48)?,
    radical_kern_before_degree_pt: value(49)?,
    radical_kern_after_degree_pt: value(50)?,
    radical_degree_bottom_raise_percent: f32::from(be_u16(bytes, constants_offset + 212)?),
  })
}

fn be_u16(bytes: &[u8], offset: usize) -> Option<u16> {
  let value = bytes.get(offset..offset.checked_add(2)?)?;
  Some(u16::from_be_bytes([value[0], value[1]]))
}

fn be_i16(bytes: &[u8], offset: usize) -> Option<i16> {
  let value = bytes.get(offset..offset.checked_add(2)?)?;
  Some(i16::from_be_bytes([value[0], value[1]]))
}

fn baseline_offset_in_line_from_metrics(
  metrics: TextVerticalMetrics,
  style: &(impl FontStyleRef + ?Sized),
  line_height_pt: f32,
) -> f32 {
  let natural_height_pt = metrics.line_height_pt() + style.baseline_shift_pt().abs();
  let extra_leading_pt = (line_height_pt - natural_height_pt).max(0.0) / 2.0;
  extra_leading_pt + metrics.leading_above_pt() + metrics.ascent_pt - style.baseline_shift_pt()
}

fn baseline_offset_in_line_with_windows_metrics_from_metrics(
  metrics: TextVerticalMetrics,
  style: &(impl FontStyleRef + ?Sized),
  line_height_pt: f32,
) -> f32 {
  let natural_height_pt = metrics.line_height_pt() + style.baseline_shift_pt().abs();
  let extra_leading_pt = (line_height_pt - natural_height_pt).max(0.0) / 2.0;
  let baseline_offset_pt = if metrics.baseline_offset_pt > 0.0 {
    metrics.baseline_offset_pt
  } else {
    metrics.leading_above_pt() + metrics.ascent_pt
  };
  let fitted_baseline_pt =
    fit_windows_baseline_to_line(baseline_offset_pt, metrics.descent_pt, line_height_pt);
  if fitted_baseline_pt < baseline_offset_pt {
    fitted_baseline_pt - style.baseline_shift_pt()
  } else {
    extra_leading_pt + baseline_offset_pt - style.baseline_shift_pt()
  }
}

fn fit_windows_baseline_to_line(
  baseline_offset_pt: f32,
  descent_pt: f32,
  line_height_pt: f32,
) -> f32 {
  // OS/2 usWinAscent/usWinDescent are clipping extents and can exceed the
  // actual PowerPoint line box (Arial Black is a common example). PowerPoint
  // preserves their ascent/descent ratio while fitting that box, rather than
  // placing the raw clipping ascent below the next line.
  let windows_height_pt = baseline_offset_pt + descent_pt;
  if windows_height_pt > line_height_pt && windows_height_pt > f32::EPSILON {
    baseline_offset_pt * line_height_pt / windows_height_pt
  } else {
    baseline_offset_pt
  }
}

fn one_glyph_per_character_indices(text: &str, glyphs: &[ShapedGlyph]) -> Option<Vec<usize>> {
  let mut characters = text
    .char_indices()
    .enumerate()
    .map(|(index, (start, character))| ((start, start + character.len_utf8()), index))
    .collect::<HashMap<_, _>>();
  if glyphs.len() != characters.len() {
    return None;
  }
  glyphs
    .iter()
    .map(|glyph| characters.remove(&(glyph.text_range.start, glyph.text_range.end)))
    .collect()
}

pub fn measure_text(text: &str, style: &(impl FontStyleRef + ?Sized)) -> f32 {
  TextMetrics::new().measure_text(text, style)
}

pub fn shape_text(text: &str, style: &(impl FontStyleRef + ?Sized)) -> Option<ShapedText> {
  TextMetrics::new().shape_text(text, style)
}

fn shaped_text_from_runs(
  runs: Vec<ooxmlsdk_fonts::ShapedRun<'_, '_>>,
  mut font_face: impl FnMut(&ooxmlsdk_fonts::FontId) -> Option<FontFaceData>,
) -> Option<ShapedText> {
  let glyph_count = runs.iter().map(|run| run.glyphs.len()).sum();
  let mut glyphs = Vec::with_capacity(glyph_count);
  let mut font_faces = Vec::with_capacity(runs.len());
  let mut width_pt = 0.0;
  for run in runs {
    let font_index = font_faces.len();
    font_faces.push(font_face(&run.font_id)?);
    width_pt += run.advance_pt;
    let font_size_pt = run.font_size_pt.0;
    let em_divisor = font_size_pt.max(f32::EPSILON);
    glyphs.extend(run.glyphs.iter().map(|glyph| ShapedGlyph {
      font_index,
      font_size_pt,
      glyph_id: glyph.glyph_id,
      text_range: glyph.text_range.clone(),
      x_advance_em: glyph.x_advance_pt / em_divisor,
      x_offset_em: glyph.x_offset_pt / em_divisor,
      y_offset_em: glyph.y_offset_pt / em_divisor,
      y_advance_em: glyph.y_advance_pt / em_divisor,
      bounds_em: glyph.bounds.map(|bounds| ShapedGlyphBounds {
        x_min_em: bounds.x_min_pt / em_divisor,
        y_min_em: bounds.y_min_pt / em_divisor,
        x_max_em: bounds.x_max_pt / em_divisor,
        y_max_em: bounds.y_max_pt / em_divisor,
      }),
    }));
  }

  Some(ShapedText {
    glyphs,
    font_faces,
    width_pt,
  })
}

pub fn vertical_metrics(style: &(impl FontStyleRef + ?Sized)) -> TextVerticalMetrics {
  TextMetrics::new().vertical_metrics(style)
}

pub fn text_decoration_metrics(style: &(impl FontStyleRef + ?Sized)) -> TextDecorationMetrics {
  TextMetrics::new().text_decoration_metrics(style)
}

pub fn inline_text_box_height(style: &(impl FontStyleRef + ?Sized)) -> f32 {
  vertical_metrics(style).line_height_pt() + style.baseline_shift_pt().abs()
}

pub fn baseline_offset_in_line(style: &(impl FontStyleRef + ?Sized), line_height_pt: f32) -> f32 {
  let metrics = vertical_metrics(style);
  let natural_height_pt = metrics.line_height_pt() + style.baseline_shift_pt().abs();
  let extra_leading_pt = (line_height_pt - natural_height_pt).max(0.0) / 2.0;
  extra_leading_pt + metrics.leading_above_pt() + metrics.ascent_pt - style.baseline_shift_pt()
}

fn approximate_vertical_metrics(font_size: f32) -> TextVerticalMetrics {
  TextVerticalMetrics {
    ascent_pt: font_size * FALLBACK_ASCENT_EM,
    descent_pt: font_size * FALLBACK_DESCENT_EM,
    line_gap_pt: font_size * FALLBACK_LINE_GAP_EM,
    baseline_offset_pt: font_size * (FALLBACK_ASCENT_EM + FALLBACK_LINE_GAP_EM / 2.0),
    directwrite_baseline_offset_pt: font_size * (FALLBACK_ASCENT_EM + FALLBACK_LINE_GAP_EM),
    wordprocessingml_cjk_line_metrics: false,
  }
}

fn approximate_decoration_metrics(font_size: f32) -> TextDecorationMetrics {
  // FontMetricData::ImplInitTextLineSize. This branch is only used when no
  // usable OpenType underline/strikeout metrics can be loaded for the face.
  let metrics = approximate_vertical_metrics(font_size);
  let descent = if metrics.descent_pt > 0.0 {
    metrics.descent_pt
  } else {
    (metrics.ascent_pt / LO_TEXT_LINE_DESCENT_FALLBACK_DIVISOR).max(LO_TEXT_LINE_MIN_WIDTH_PT)
  };
  let descent = if LO_TEXT_LINE_MAX_DESCENT_DIVISOR * descent > metrics.ascent_pt {
    metrics.ascent_pt / LO_TEXT_LINE_MAX_DESCENT_DIVISOR
  } else {
    descent
  };
  let line_width =
    (descent * LO_TEXT_LINE_WIDTH_FRACTION_OF_DESCENT).max(LO_TEXT_LINE_MIN_WIDTH_PT);
  let half_line_width =
    (line_width / LO_TEXT_LINE_WIDTH_HALF_DIVISOR).max(LO_TEXT_LINE_MIN_WIDTH_PT);
  TextDecorationMetrics {
    underline_offset_pt: descent / LO_TEXT_LINE_WIDTH_HALF_DIVISOR
      + LO_TEXT_LINE_UNDERLINE_BASELINE_OFFSET_PT
      - half_line_width,
    underline_width_pt: line_width,
    strikethrough_offset_pt: (metrics.ascent_pt - metrics.line_gap_pt)
      / LO_TEXT_LINE_STRIKEOUT_OFFSET_DIVISOR
      + half_line_width,
    strikethrough_width_pt: line_width,
  }
}

#[cfg(test)]
mod tests {
  use crate::common::{Pt, TextStyle};

  use super::*;

  #[test]
  fn shaped_measurement_handles_ligatures_and_cjk() {
    let style = test_style();

    assert!(measure_text("office", &style) > 0.0);
    assert!(measure_text("商务文档", &style) > measure_text("abc", &style));
  }

  #[test]
  fn shaped_text_exposes_glyph_advances_for_pdf_paint() {
    let style = test_style();
    let shaped = shape_text("office", &style).expect("shaped text");

    assert!(!shaped.glyphs.is_empty());
    assert!(shaped.width_pt > 0.0);
    assert!(
      shaped
        .glyphs
        .iter()
        .all(|glyph| glyph.text_range.end <= "office".len())
    );
    assert!(shaped.glyphs.iter().any(|glyph| glyph.bounds_em.is_some()));
  }

  #[test]
  fn shaped_text_preserves_synthesized_small_caps_run_sizes() {
    // LibreOffice sw/source/core/txtnode/fntcap.cxx renders lowercase small
    // capitals at 80%, while ISO/IEC 29500-1 §17.3.2.33 leaves
    // non-alphabetic characters unchanged. PDF must retain both shaped sizes
    // instead of reshaping the original lowercase text.
    let mut style = test_style();
    style.small_caps = true;
    let shaped = shape_text("Aa,1", &style).expect("small-caps shaped text");

    assert!(
      shaped
        .glyphs
        .iter()
        .any(|glyph| (glyph.font_size_pt - style.font_size.0).abs() < 0.01)
    );
    assert!(
      shaped
        .glyphs
        .iter()
        .any(|glyph| glyph.font_size_pt < style.font_size.0)
    );
    assert!(
      shaped
        .glyphs
        .iter()
        .filter(|glyph| glyph.text_range.start >= 2)
        .all(|glyph| (glyph.font_size_pt - style.font_size.0).abs() < 0.01)
    );
  }

  #[test]
  fn repeated_measurement_reuses_the_shaped_width() {
    let style = test_style();
    let mut metrics = TextMetrics::new();

    let first = metrics.measure_text("repeated", &style);
    let second = metrics.measure_text("repeated", &style);

    assert_eq!(first, second);
    assert_eq!(metrics.measure_styles.len(), 1);
    assert_eq!(metrics.measure_widths[0].len(), 1);
  }

  #[test]
  fn gdi_device_metrics_use_integer_ppem_and_device_pixel_advances() {
    assert_eq!(gdi_device_ppem(8.04, 600.0), Some(67));
    assert_eq!(gdi_device_ppem(8.0, 0.0), None);
    assert_eq!(gdi_device_ppem(72.0, 300.0), None);
    assert!((gdi_device_advance_pt(36, 600.0) - 4.32).abs() < 0.0001);
    assert!((gdi_scaled_device_advance_pt(5.22, 600.0).unwrap() - 5.28).abs() < 0.0001);
    assert!((gdi_scaled_device_advance_pt(5.28, 600.0).unwrap() - 5.28).abs() < 0.0001);
  }

  #[test]
  fn gdi_hinted_extent_rounds_the_complete_run_to_a_device_pixel() {
    let style = test_style();
    let mut metrics = TextMetrics::new();
    let device_dpi = 600.0;
    let extent_pt = metrics
      .gdi_hinted_text_extent_pt("iiii", &style, device_dpi)
      .expect("simple hinted run");
    let extent_px = extent_pt * device_dpi / crate::units::POINTS_PER_INCH;

    assert!((extent_px - extent_px.round()).abs() < 0.0001);
  }

  #[test]
  fn gdi_hinting_instance_is_reused_across_text_runs() {
    let style = test_style();
    let mut metrics = TextMetrics::new();
    let device_dpi = 600.0;

    metrics
      .gdi_hinted_text_extent_pt("iiii", &style, device_dpi)
      .expect("first simple hinted run");
    let instance_count = metrics.gdi_hinting_instances.instances.len();
    assert!(instance_count > 0);

    metrics
      .gdi_hinted_text_extent_pt("WWWW", &style, device_dpi)
      .expect("second simple hinted run");
    assert_eq!(
      metrics.gdi_hinting_instances.instances.len(),
      instance_count
    );
  }

  #[test]
  fn gdi_uniform_device_spacing_rejects_nonuniform_advance_changes() {
    let spacing = uniform_character_spacing_from_advances(&[5.22, 5.22, 5.22], &[5.28, 5.28, 5.28])
      .expect("uniform spacing");
    assert!((spacing - 0.06).abs() < 0.0001);
    assert!(uniform_character_spacing_from_advances(&[5.22, 5.22], &[5.28, 5.16]).is_none());
    assert!(uniform_character_spacing_from_advances(&[5.22], &[5.28]).is_none());
  }

  #[test]
  fn oversized_windows_metrics_are_fitted_proportionally_into_the_line_box() {
    let font_size_pt = 24.0;
    let line_height_pt = font_size_pt * 1.2;
    let windows_ascent_pt = 2_254.0 / 2_048.0 * font_size_pt;
    let windows_descent_pt = 634.0 / 2_048.0 * font_size_pt;

    let baseline =
      fit_windows_baseline_to_line(windows_ascent_pt, windows_descent_pt, line_height_pt);

    assert!((baseline - 22.48).abs() < 0.01);
    assert_eq!(fit_windows_baseline_to_line(9.0, 3.0, 14.4), 9.0);
  }

  #[test]
  fn wordprocessingml_cjk_capability_adds_symmetric_side_leading() {
    let metrics = TextVerticalMetrics {
      ascent_pt: 20.0,
      descent_pt: 6.0,
      line_gap_pt: 0.0,
      baseline_offset_pt: 20.0,
      directwrite_baseline_offset_pt: 20.0,
      wordprocessingml_cjk_line_metrics: true,
    };
    let style = TextStyle {
      wordprocessingml_font_slots: true,
      wordprocessingml_cjk_line_metrics: true,
      ..TextStyle::default()
    };

    let adjusted = wordprocessingml_line_vertical_metrics(&style, metrics);
    assert!((adjusted.ascent_pt - 23.9).abs() < 0.0001);
    assert!((adjusted.descent_pt - 9.9).abs() < 0.0001);
    assert!((adjusted.line_height_pt() - 33.8).abs() < 0.0001);
    assert!((adjusted.baseline_offset_pt - 23.9).abs() < 0.0001);
    assert!((adjusted.directwrite_baseline_offset_pt - 23.9).abs() < 0.0001);

    let drawingml = TextStyle::default();
    assert_eq!(
      wordprocessingml_line_vertical_metrics(&drawingml, metrics),
      metrics
    );
    let no_leading_unset = TextStyle {
      wordprocessingml_font_slots: true,
      ..TextStyle::default()
    };
    assert_eq!(
      wordprocessingml_line_vertical_metrics(&no_leading_unset, metrics),
      metrics
    );
    assert_eq!(
      wordprocessingml_line_vertical_metrics(
        &style,
        TextVerticalMetrics {
          wordprocessingml_cjk_line_metrics: false,
          ..metrics
        },
      ),
      TextVerticalMetrics {
        wordprocessingml_cjk_line_metrics: false,
        ..metrics
      }
    );
  }

  fn test_style() -> TextStyle<'static> {
    TextStyle {
      font_size: Pt(11.0),
      ..TextStyle::default()
    }
  }
}
