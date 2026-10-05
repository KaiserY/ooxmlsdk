mod kashida;

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

  fn wordprocessing_nominal_control_metrics(&self) -> bool {
    self.style.wordprocessing_nominal_control_metrics()
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
  fn wordprocessing_layout_font_sizes(&self) -> Option<crate::common::LayoutFontSizes> {
    self.style.wordprocessing_layout_font_sizes()
  }

  fn wordprocessing_measurement_profile(&self) -> Option<(bool, u16)> {
    self.style.wordprocessing_measurement_profile()
  }

  fn wordprocessingml_font_slots(&self) -> bool {
    self.style.wordprocessingml_font_slots()
  }

  fn wordprocessingml_form_text_blank_cell(&self) -> bool {
    self.style.wordprocessingml_form_text_blank_cell()
  }

  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    self.style.wordprocessingml_cjk_line_metrics()
  }

  fn wordprocessing_justification_expansion_pt(&self) -> &[f32] {
    self.style.wordprocessing_justification_expansion_pt()
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    self.style.cjk_punctuation_compression_ratio()
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    self.style.wordprocessingml_legacy_punctuation_spacing()
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    self.style.wordprocessingml_punctuation_spacing()
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
// Word's DOC/DOCX compatibility metrics require this much leading on each
// side of the natural font ink box for a face whose OS/2 code-page ranges
// advertise CP932/936/949/950. Existing hhea line gap contributes to those
// side bands; it must not be appended a second time. A 25pt DengXian control
// gives a 33.84pt single-line advance from a 26.05pt ink box and moves the
// first baseline down by 3.90pt, independently confirming the two 15% bands.
// Writer's tdf#129808 path confirms the same four-code-page capability rule.
pub(crate) const WORDPROCESSINGML_CJK_SIDE_LEADING_RATIO: f32 = 0.15;
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
  pub windows_line_height_pt: f32,
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

  pub fn windows_line_height_pt(self) -> f32 {
    self.windows_line_height_pt.max(self.ink_height_pt())
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
  pub safe_to_insert_tatweel: bool,
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
  shaping_context: Option<crate::common::TextShapingContext>,
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
  wordprocessing_nominal_control_metrics: bool,
  character_spacing_bits: u32,
  horizontal_scale_bits: u32,
  wordprocessing_measurement_profile: Option<(bool, u16)>,
  wordprocessing_layout_font_size_bits: Option<(u32, Option<u32>)>,
  bold: bool,
  italic: bool,
  complex_bold: Option<bool>,
  complex_italic: Option<bool>,
  small_caps: bool,
  kerning_enabled: bool,
  ligatures: Option<crate::common::OpenTypeLigatures>,
  open_type_features: crate::common::OpenTypeFeatureSettings,
  wordprocessingml_font_slots: bool,
  wordprocessingml_cjk_line_metrics: bool,
  wordprocessingml_font_hint: Option<ooxmlsdk_fonts::WordprocessingFontTypeHint>,
  wordprocessingml_east_asia_language_is_chinese: bool,
  wordprocessingml_bidi_language_is_hebrew: bool,
  font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  high_ansi_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  wordprocessingml_east_asia_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  complex_font_charset: Option<ooxmlsdk_fonts::FontCharset>,
  font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  high_ansi_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  east_asia_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  complex_font_pitch: Option<ooxmlsdk_fonts::FontPitch>,
  cjk_punctuation_compression_ratio_bits: u32,
  wordprocessing_justification_expansion_bits: Vec<u32>,
  wordprocessingml_legacy_punctuation_spacing: bool,
  wordprocessingml_punctuation_spacing: bool,
  wordprocessingml_balance_single_byte_double_byte_width: bool,
}

impl MeasureStyleKey {
  fn from_style(style: &(impl FontStyleRef + ?Sized)) -> Self {
    Self {
      shaping_context: style.shaping_context().cloned(),
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
      wordprocessing_nominal_control_metrics: style.wordprocessing_nominal_control_metrics(),
      character_spacing_bits: style.character_spacing_pt().to_bits(),
      horizontal_scale_bits: style.horizontal_scale().to_bits(),
      wordprocessing_measurement_profile: style.wordprocessing_measurement_profile(),
      wordprocessing_layout_font_size_bits: style.wordprocessing_layout_font_sizes().map(|sizes| {
        (
          sizes.primary.0.to_bits(),
          sizes.complex.map(|size| size.0.to_bits()),
        )
      }),
      bold: style.bold(),
      italic: style.italic(),
      complex_bold: style.complex_bold(),
      complex_italic: style.complex_italic(),
      small_caps: style.small_caps(),
      kerning_enabled: style.kerning_enabled(),
      ligatures: style.ligatures(),
      open_type_features: style.open_type_features(),
      wordprocessingml_font_slots: style.wordprocessingml_font_slots(),
      wordprocessingml_cjk_line_metrics: style.wordprocessingml_cjk_line_metrics(),
      wordprocessingml_font_hint: style.wordprocessingml_font_hint(),
      wordprocessingml_east_asia_language_is_chinese: style
        .wordprocessingml_east_asia_language_is_chinese(),
      wordprocessingml_bidi_language_is_hebrew: style.wordprocessingml_bidi_language_is_hebrew(),
      font_charset: style.font_charset(),
      high_ansi_font_charset: style.high_ansi_font_charset(),
      wordprocessingml_east_asia_font_charset: style.wordprocessingml_east_asia_font_charset(),
      complex_font_charset: style.complex_font_charset(),
      font_pitch: style.font_pitch(),
      high_ansi_font_pitch: style.high_ansi_font_pitch(),
      east_asia_font_pitch: style.east_asia_font_pitch(),
      complex_font_pitch: style.complex_font_pitch(),
      cjk_punctuation_compression_ratio_bits: style.cjk_punctuation_compression_ratio().to_bits(),
      wordprocessing_justification_expansion_bits: style
        .wordprocessing_justification_expansion_pt()
        .iter()
        .map(|value| value.to_bits())
        .collect(),
      wordprocessingml_legacy_punctuation_spacing: style
        .wordprocessingml_legacy_punctuation_spacing(),
      wordprocessingml_punctuation_spacing: style.wordprocessingml_punctuation_spacing(),
      wordprocessingml_balance_single_byte_double_byte_width: style
        .wordprocessingml_balance_single_byte_double_byte_width(),
    }
  }

  fn matches(&self, style: &(impl FontStyleRef + ?Sized)) -> bool {
    self.shaping_context.as_ref() == style.shaping_context()
      && self.font_family.as_deref() == style.font_family()
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
      && self.wordprocessing_nominal_control_metrics
        == style.wordprocessing_nominal_control_metrics()
      && self.character_spacing_bits == style.character_spacing_pt().to_bits()
      && self.horizontal_scale_bits == style.horizontal_scale().to_bits()
      && self.wordprocessing_measurement_profile == style.wordprocessing_measurement_profile()
      && self.wordprocessing_layout_font_size_bits
        == style.wordprocessing_layout_font_sizes().map(|sizes| {
          (
            sizes.primary.0.to_bits(),
            sizes.complex.map(|size| size.0.to_bits()),
          )
        })
      && self.bold == style.bold()
      && self.italic == style.italic()
      && self.complex_bold == style.complex_bold()
      && self.complex_italic == style.complex_italic()
      && self.small_caps == style.small_caps()
      && self.kerning_enabled == style.kerning_enabled()
      && self.ligatures == style.ligatures()
      && self.open_type_features == style.open_type_features()
      && self.wordprocessingml_font_slots == style.wordprocessingml_font_slots()
      && self.wordprocessingml_cjk_line_metrics == style.wordprocessingml_cjk_line_metrics()
      && self.wordprocessingml_font_hint == style.wordprocessingml_font_hint()
      && self.wordprocessingml_east_asia_language_is_chinese
        == style.wordprocessingml_east_asia_language_is_chinese()
      && self.wordprocessingml_bidi_language_is_hebrew
        == style.wordprocessingml_bidi_language_is_hebrew()
      && self.font_charset == style.font_charset()
      && self.high_ansi_font_charset == style.high_ansi_font_charset()
      && self.wordprocessingml_east_asia_font_charset
        == style.wordprocessingml_east_asia_font_charset()
      && self.complex_font_charset == style.complex_font_charset()
      && self.font_pitch == style.font_pitch()
      && self.high_ansi_font_pitch == style.high_ansi_font_pitch()
      && self.east_asia_font_pitch == style.east_asia_font_pitch()
      && self.complex_font_pitch == style.complex_font_pitch()
      && self
        .wordprocessing_justification_expansion_bits
        .iter()
        .copied()
        .eq(
          style
            .wordprocessing_justification_expansion_pt()
            .iter()
            .map(|value| value.to_bits()),
        )
      && self.cjk_punctuation_compression_ratio_bits
        == style.cjk_punctuation_compression_ratio().to_bits()
      && self.wordprocessingml_legacy_punctuation_spacing
        == style.wordprocessingml_legacy_punctuation_spacing()
      && self.wordprocessingml_punctuation_spacing == style.wordprocessingml_punctuation_spacing()
      && self.wordprocessingml_balance_single_byte_double_byte_width
        == style.wordprocessingml_balance_single_byte_double_byte_width()
  }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct GdiHintedTextExtents {
  pub(crate) positioned_width_pt: f32,
  pub(crate) unpositioned_width_pt: f32,
}

type GdiHintedExtentCache = HashMap<u32, HashMap<Arc<str>, Option<GdiHintedTextExtents>>>;

#[derive(Debug, Default)]
pub struct TextMetrics {
  fonts: FontResolver,
  measure_styles: Vec<MeasureStyleKey>,
  measure_widths: Vec<HashMap<Arc<str>, f32>>,
  gdi_hinted_extents: Vec<GdiHintedExtentCache>,
  gdi_hinting_instances: GdiHintingInstanceCache,
  word_natural_instances: WordNaturalMetricCache,
  excel_cell_text_insets: HashMap<usize, Option<f32>>,
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

#[derive(Default)]
struct WordNaturalMetricCache {
  instances: HashMap<(FontFaceCacheKey, u16), Option<emfsdk::render::GdiNaturalMetrics>>,
}

impl WordNaturalMetricCache {
  fn advance(&mut self, face: &FontFaceData, pixels: u16, glyph: u32) -> Option<i32> {
    self
      .instances
      .entry((face.cache_key(), pixels))
      .or_insert_with(|| {
        emfsdk::render::GdiNaturalMetrics::new(Arc::from(face.data.as_slice()), face.index, pixels)
      })
      .as_mut()?
      .glyph_advance_px(glyph)
  }
}

impl std::fmt::Debug for WordNaturalMetricCache {
  fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    formatter
      .debug_struct("WordNaturalMetricCache")
      .field("len", &self.instances.len())
      .finish()
  }
}

impl TextMetrics {
  pub fn new() -> Self {
    Self::default()
  }

  /// Word dot leaders repeat an integer GDI Natural device advance. RTL
  /// fixed output normalizes that advance to integral hundredths of an em
  /// before converting it back to points. Source counting retains the device
  /// advance. Native ExtTextOut/PDF captures cover 288 independent rows.
  pub(crate) fn word_dot_leader_advances(
    &mut self,
    style: &crate::model::TextStyle,
    bidi: bool,
  ) -> Option<(f32, f32)> {
    let percent = style.wordprocessing_font_width_percent.unwrap_or(100);
    if !style.character_spacing_pt.is_finite()
      || (style.horizontal_scale.unwrap_or(1.0) - f32::from(percent) / 100.0).abs() > 0.00001
    {
      return None;
    }
    let shaped = self.shape_text(".", style)?;
    let glyph = shaped.glyphs.first()?;
    let face = shaped.font_faces.get(glyph.font_index)?;
    let ratio =
      crate::common::wordprocessing_device::font_width_ratio(face, glyph.font_size_pt, percent)?;
    let pixel_pt = crate::units::POINTS_PER_INCH / crate::units::OFFICE_FIXED_OUTPUT_DPI;
    let pixels = (glyph.font_size_pt / pixel_pt).round();
    if !(1.0..=f32::from(u16::MAX)).contains(&pixels) {
      return None;
    }
    let natural = self
      .word_natural_instances
      .advance(face, pixels as u16, glyph.glyph_id)?;
    // Native ExtTextOut advances add authored spacing after the physical
    // font-width transform. Round its printer pixels independently; applying
    // the font ratio to the spacing changes scaled TOC leader counts.
    let advance =
      (f64::from(natural) * ratio).round() as f32 + (style.character_spacing_pt / pixel_pt).round();
    if advance <= 0.0 {
      return None;
    }
    let source = advance * pixel_pt;
    let paint = if bidi && style.right_to_left == Some(true) {
      (advance / pixels * 100.0).round() * (pixels * pixel_pt) / 100.0
    } else {
      source
    };
    Some((source, paint))
  }

  pub fn into_font_resolver(self) -> FontResolver {
    self.fonts
  }

  pub fn measure_text(&mut self, text: &str, style: &(impl FontStyleRef + ?Sized)) -> f32 {
    if text.is_empty() {
      return 0.0;
    }

    // Word's empty legacy text form field reserves five digit cells. Its
    // serialized result can contain en spaces, whose glyph advances are not
    // the field's layout width. Use the authored marker face's digit advance
    // only when that face is available: substituting another face's digits
    // for a missing marker font changes the field width (tdf92472). Preserve
    // the cached en-space fallback when the authored metric is unknown.
    if style.wordprocessingml_form_text_blank_cell()
      && text.chars().all(|character| character == '\u{2002}')
      && self.fonts.has_exact_ascii_face(style)
    {
      return self
        .shape_text(&"0".repeat(text.chars().count()), style)
        .map_or(0.0, |shaped| shaped.width_pt);
    }

    // Justification belongs to this laid-out portion, not the natural-width
    // cache. Subsequent unadjusted measurements must remain unchanged.
    if !style.kashida_expansions().is_empty() || style.wordprocessing_kashida().is_some() {
      return self
        .shape_text(text, style)
        .map_or(0.0, |shaped| shaped.width_pt);
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
    let shaped = shaped_text_from_runs(runs, |font_id| self.fonts.font_face_data(font_id))?;
    let shaped = wordprocessingml_synthetic_bold_advances(text, shaped, style);
    let shaped = if let Some(device) = style.wordprocessing_kashida() {
      kashida::realize_device(
        text,
        shaped,
        style.horizontal_scale(),
        device,
        style.character_spacing_pt(),
        &mut self.word_natural_instances,
      )?
    } else {
      shaped
    };
    Some(kashida::expand(shaped, style))
  }

  pub(crate) fn kashida_opportunities(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Vec<kashida::KashidaOpportunity> {
    self
      .shape_text(text, style)
      .map_or_else(Vec::new, |shaped| {
        kashida::opportunities(text, &shaped, style)
      })
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
      .map(|shaped| wordprocessingml_synthetic_bold_advances(text, shaped, style))
      .map(|shaped| kashida::expand(shaped, style))
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

  /// Returns Excel's unrotated fixed-output character advances.
  ///
  /// Excel's worksheet edit paint path disables pair positioning and standard
  /// ligatures, rounds each unpositioned outline advance to the nearest
  /// printer pixel, and writes the resulting character positions to the PDF
  /// `TJ` array. This differs from the hdmx/ceiling behavior exposed by the
  /// classic GDI helper above and from the projected off-axis path below.
  pub(crate) fn excel_unrotated_character_advances_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<Arc<[f32]>> {
    if text.is_empty()
      || style.character_spacing_pt().abs() > f32::EPSILON
      || (style.horizontal_scale() - 1.0).abs() > f32::EPSILON
    {
      return None;
    }

    let shaped = self.shape_text(text, style)?;
    if shaped.font_faces.iter().any(|face| face.synthetic_bold) {
      return None;
    }
    let character_indices = one_glyph_per_character_indices(text, &shaped.glyphs)?;
    let pixel_pt = crate::units::POINTS_PER_INCH / crate::units::OFFICE_FIXED_OUTPUT_DPI;
    let mut advances = vec![None; character_indices.len()];
    for (glyph, character_index) in shaped.glyphs.iter().zip(character_indices) {
      let device_pixels = (glyph.x_advance_em * glyph.font_size_pt / pixel_pt).round();
      if !device_pixels.is_finite() {
        return None;
      }
      advances[character_index] = Some(device_pixels * pixel_pt);
    }
    advances
      .into_iter()
      .collect::<Option<Vec<_>>>()
      .map(Arc::from)
  }

  /// Returns the quarter-digit inset of Excel's realized cell font.
  ///
  /// A worksheet reuses this metric for every cell and continuation page.
  /// Cache it by the same complete font/shaping key as natural text widths,
  /// rather than shaping the ten digits again for each cell.
  pub(crate) fn excel_cell_text_inset_pt(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<f32> {
    let style_index = self.measure_style_index(style);
    let cacheable = style.kashida_expansions().is_empty();
    if cacheable && let Some(inset) = self.excel_cell_text_insets.get(&style_index) {
      return *inset;
    }
    let inset = self
      .excel_unrotated_character_advances_pt("0123456789", style)
      .map(|advances| {
        let dot = crate::units::POINTS_PER_INCH / crate::units::OFFICE_FIXED_OUTPUT_DPI;
        let digit_dots = advances.iter().copied().fold(0.0_f32, f32::max) / dot;
        (digit_dots / 4.0).ceil() * dot
      });
    if cacheable {
      self.excel_cell_text_insets.insert(style_index, inset);
    }
    inset
  }

  /// The rotated alignment box uses the horizontal printer extent, with
  /// quarter-digit insets on both sides and one final device pixel. This is
  /// distinct from the projected advances used to paint the same run.
  pub(crate) fn excel_rotated_layout_extents_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<(f32, f32)> {
    let pixel = crate::units::POINTS_PER_INCH / crate::units::OFFICE_FIXED_OUTPUT_DPI;
    let extent = |metrics: &mut Self, text: &str| {
      let advances = metrics.gdi_device_character_advances_pt(
        text,
        style,
        crate::units::OFFICE_FIXED_OUTPUT_DPI,
      )?;
      let shaped = metrics.shape_text(text, style)?;
      let indices = one_glyph_per_character_indices(text, &shaped.glyphs)?;
      let mut width = advances.iter().sum::<f32>();
      for glyph in &shaped.glyphs {
        if shaped.font_faces.get(glyph.font_index)?.synthetic_bold {
          width += pixel;
        }
      }
      (indices.len() == advances.len()).then_some(width)
    };
    let width = extent(self, text)?;
    let digit = extent(self, "0")?;
    let padding = (2.0 * ((digit / pixel).round() / 4.0).ceil() + 1.0) * pixel;
    Some((width, padding))
  }

  /// Excel projects the integer advances of a slanted printer-font run onto
  /// the device grid before emitting its PDF/XPS glyph positions. Off-axis
  /// fonts retain fractional outline widths instead of horizontal hdmx widths.
  pub(crate) fn excel_rotated_character_advances_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    degrees: f32,
  ) -> Option<Arc<[f32]>> {
    let angle = degrees.abs() % 90.0;
    if !angle.is_finite() || angle <= f32::EPSILON {
      return None;
    }
    let dpi = crate::units::OFFICE_FIXED_OUTPUT_DPI;
    let shaped = self.shape_text(text, style)?;
    let indices = one_glyph_per_character_indices(text, &shaped.glyphs)?;
    let mut widths = vec![0.0_f64; indices.len()];
    for (glyph, index) in shaped.glyphs.iter().zip(indices) {
      let face = shaped.font_faces.get(glyph.font_index)?;
      // GDI's off-axis ABC widths use sixteenth-pixel outline advances and
      // one extra pixel for synthesized bold. Native SimSun/SimHei and Arial
      // printer-DC controls distinguish this from integer hinted widths.
      let ppem = (glyph.font_size_pt * dpi / crate::units::POINTS_PER_INCH).round();
      let natural = glyph.x_advance_em as f64 * ppem as f64;
      widths[index] = (natural * 16.0).ceil() / 16.0 + if face.synthetic_bold { 1.0 } else { 0.0 };
    }
    let (sin, cos) = angle.to_radians().sin_cos();
    let mut cumulative = 0.0_f64;
    let mut previous = 0.0_f64;
    let advances = widths
      .into_iter()
      .map(|width| {
        cumulative += width;
        let rounded = cumulative.round();
        let advance = (rounded - previous) as f32;
        previous = rounded;
        let x = (advance * cos).trunc();
        let y = (advance * sin).trunc();
        // Office's XPS Indices expose fifth-pixel distances after the
        // integer projection. Keep f32 angles: at 60 degrees the cosine
        // lies just below one half, which decides the device truncation.
        let projected = (x.hypot(y) * 5.0).trunc() / 5.0;
        projected * crate::units::POINTS_PER_INCH / dpi
      })
      .collect::<Vec<_>>();
    Some(Arc::from(advances))
  }

  /// Returns both the shaped and unpositioned GDI-compatible hinted extents.
  ///
  /// Classic GDI accumulates hinted 26.6 advances and rounds the complete
  /// extent; Skrifa exposes the adjusted advances through its TrueType hinter.
  /// `GetTextExtentPoint32W` accumulates the hinted advance of each
  /// character without applying the pairs exposed by `GetKerningPairsW`.
  /// DrawingML paint can independently enable kerning, so callers that model
  /// a GDI measurement rectangle need both widths.
  pub(crate) fn gdi_hinted_text_extents_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    device_dpi: f32,
  ) -> Option<GdiHintedTextExtents> {
    if text.is_empty() {
      return Some(GdiHintedTextExtents {
        positioned_width_pt: 0.0,
        unpositioned_width_pt: 0.0,
      });
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

    let extent = self.compute_gdi_hinted_text_extents_pt(text, style, device_dpi);
    let text_key = self.measure_widths[style_index]
      .get_key_value(text)
      .map_or_else(|| Arc::from(text), |(text, _)| text.clone());
    self.gdi_hinted_extents[style_index]
      .entry(device_dpi_bits)
      .or_default()
      .insert(text_key, extent);
    extent
  }

  fn compute_gdi_hinted_text_extents_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    device_dpi: f32,
  ) -> Option<GdiHintedTextExtents> {
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
    let mut positioned_width_pt = shaped.width_pt;
    let mut unpositioned_width_pt = 0.0;
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
      positioned_width_pt += (hinted_advance - unhinted_advance) * points_per_device_pixel;
      unpositioned_width_pt += hinted_advance * points_per_device_pixel;
    }
    let positioned_device_pixels = positioned_width_pt / points_per_device_pixel;
    let unpositioned_device_pixels = unpositioned_width_pt / points_per_device_pixel;
    (positioned_device_pixels.is_finite() && unpositioned_device_pixels.is_finite()).then_some(
      GdiHintedTextExtents {
        positioned_width_pt: positioned_device_pixels.round() * points_per_device_pixel,
        unpositioned_width_pt: unpositioned_device_pixels.round() * points_per_device_pixel,
      },
    )
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

  /// Classic GDI adds one device pixel to each synthesized-bold advance.
  /// Keep this worksheet adjustment separate from consumers that supply
  /// their own positioned glyph advances or use outline measurements.
  pub(crate) fn gdi_synthetic_bold_character_spacing_pt(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    device_dpi: f32,
  ) -> Option<f32> {
    if text.is_empty() || !style.bold() {
      return None;
    }
    let shaped = self.shape_text(text, style)?;
    if shaped.font_faces.is_empty() || !shaped.font_faces.iter().all(|face| face.synthetic_bold) {
      return None;
    }
    let character_indices = one_glyph_per_character_indices(text, &shaped.glyphs)?;
    let mut natural_advances = vec![0.0; character_indices.len()];
    for (glyph, index) in shaped.glyphs.iter().zip(character_indices) {
      natural_advances[index] = glyph.x_advance_em * glyph.font_size_pt;
    }
    let device_advances = self.gdi_device_character_advances_pt(text, style, device_dpi)?;
    let bold_advances = device_advances
      .iter()
      .map(|advance| advance + crate::units::POINTS_PER_INCH / device_dpi)
      .collect::<Vec<_>>();
    uniform_character_spacing_from_advances(&natural_advances, &bold_advances)
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

  /// Legacy Word's first justification level expands a blank up to the
  /// selected face's OS/2 average character width. This is an expansion hint,
  /// not a replacement for the blank's actual glyph advance.
  pub(crate) fn legacy_word_space_expansion_capacity(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<f32> {
    let shaped = self.shape_text(" ", style)?;
    let glyph = shaped.glyphs.first()?;
    let face = shaped.font_faces.get(glyph.font_index)?;
    let font = FontRef::from_index(face.data.as_ref(), face.index).ok()?;
    let size = Size::new(glyph.font_size_pt);
    let location = LocationRef::default();
    let average = font.metrics(size, location).average_width?;
    let space = font
      .glyph_metrics(size, location)
      .advance_width(GlyphId::new(glyph.glyph_id))?;
    Some((average - space).max(0.0) * style.horizontal_scale())
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
    if style.automatic_escapement_font_sizes_pt().is_none() {
      if let Some(logical) = self
        .fonts
        .wordprocessingml_default_charset_line_metrics(style, text)
      {
        return text_vertical_metrics_from_font_metrics(logical);
      }
      if let Some(runs) = self.fonts.wordprocessingml_line_metric_runs(text, style)
        && runs
          .iter()
          .any(|metrics| metrics.wordprocessingml_cjk_line_metrics)
      {
        // Word adjusts the line box of each selected face before combining
        // portions. Combining Calibri and Batang's raw ascent, descent and
        // gap first fabricates a taller font, then adds CJK leading to that
        // synthetic box. The resulting Korean mixed-script line is about
        // 1.7 pt too tall at 11 pt despite neither face needing that height.
        return combine_wordprocessingml_line_metric_runs(style, &runs);
      }
    }
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
  metrics: TextVerticalMetrics,
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

  wordprocessingml_side_leading(metrics)
}

fn combine_wordprocessingml_line_metric_runs(
  style: &(impl FontStyleRef + ?Sized),
  runs: &[ooxmlsdk_fonts::VerticalMetrics],
) -> TextVerticalMetrics {
  let adjusted = runs
    .iter()
    .map(|metrics| {
      wordprocessingml_line_vertical_metrics(
        style,
        text_vertical_metrics_from_font_metrics(*metrics),
      )
    })
    .collect::<Vec<_>>();
  let top = adjusted
    .iter()
    .map(|metrics| metrics.directwrite_baseline_offset_pt)
    .fold(0.0, f32::max);
  let bottom = adjusted
    .iter()
    .map(|metrics| (metrics.line_height_pt() - metrics.directwrite_baseline_offset_pt).max(0.0))
    .fold(0.0, f32::max);
  if let Some(dominant) = adjusted.iter().find(|metrics| {
    (metrics.directwrite_baseline_offset_pt - top).abs() < f32::EPSILON
      && (metrics.line_height_pt() - metrics.directwrite_baseline_offset_pt - bottom).abs()
        < f32::EPSILON
  }) {
    return *dominant;
  }
  TextVerticalMetrics {
    ascent_pt: top,
    descent_pt: bottom,
    windows_line_height_pt: adjusted
      .iter()
      .map(|metrics| metrics.windows_line_height_pt())
      .fold(0.0, f32::max),
    line_gap_pt: 0.0,
    baseline_offset_pt: adjusted
      .iter()
      .map(|metrics| metrics.baseline_offset_pt)
      .fold(0.0, f32::max),
    directwrite_baseline_offset_pt: top,
    wordprocessingml_cjk_line_metrics: true,
  }
}

fn wordprocessingml_side_leading(mut metrics: TextVerticalMetrics) -> TextVerticalMetrics {
  let required_side_leading_pt = metrics.ink_height_pt() * WORDPROCESSINGML_CJK_SIDE_LEADING_RATIO;
  let additional_side_leading_pt = (required_side_leading_pt - metrics.leading_above_pt()).max(0.0);
  metrics.ascent_pt += additional_side_leading_pt;
  metrics.descent_pt += additional_side_leading_pt;
  metrics.windows_line_height_pt += additional_side_leading_pt * 2.0;
  metrics.baseline_offset_pt += additional_side_leading_pt;
  metrics.directwrite_baseline_offset_pt += additional_side_leading_pt;
  metrics
}

fn text_vertical_metrics_from_font_metrics(
  metrics: ooxmlsdk_fonts::VerticalMetrics,
) -> TextVerticalMetrics {
  TextVerticalMetrics {
    ascent_pt: metrics.ascent_pt,
    descent_pt: metrics.descent_pt,
    windows_line_height_pt: metrics.windows_line_height_pt,
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

fn wordprocessingml_synthetic_bold_advances(
  text: &str,
  mut shaped: ShapedText,
  style: &(impl FontStyleRef + ?Sized),
) -> ShapedText {
  if !style.wordprocessingml_font_slots()
    || !shaped.font_faces.iter().any(|face| face.synthetic_bold)
    || one_glyph_per_character_indices(text, &shaped.glyphs).is_none()
  {
    return shaped;
  }

  // Word obtains its independent-character ideal advances with a font at
  // design-unit height. GDI's extra synthetic-bold pixel consequently owns
  // one design unit, including on blanks. Pinned native MSLS input arrays
  // confirm this for 84 font/size/weight controls, with 256 and 2048 upem.
  // This is distinct from the 600-dpi advance and em/35 painted stroke.
  // Complex clusters retain their existing shaping measurements.
  let additions = shaped
    .font_faces
    .iter()
    .map(|face| {
      if !face.synthetic_bold {
        return 0.0;
      }
      FontRef::from_index(face.data.as_ref(), face.index)
        .ok()
        .and_then(|font| font.head().ok())
        .map(|head| head.units_per_em())
        .filter(|&upem| upem != 0)
        .map_or(0.0, |upem| style.horizontal_scale() / f32::from(upem))
    })
    .collect::<Vec<_>>();
  for glyph in &mut shaped.glyphs {
    if glyph.x_advance_em <= 0.0 {
      continue;
    }
    let addition = additions[glyph.font_index];
    glyph.x_advance_em += addition;
    shaped.width_pt += addition * glyph.font_size_pt;
  }
  shaped
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
      safe_to_insert_tatweel: glyph.safe_to_insert_tatweel,
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
    windows_line_height_pt: font_size * (FALLBACK_ASCENT_EM + FALLBACK_DESCENT_EM),
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
  fn blank_form_cells_measure_digit_advances_without_changing_plain_en_spaces() {
    let mut style = crate::model::TextStyle {
      font_family: Some(Arc::from("Arial")),
      font_size_pt: 10.0,
      ..Default::default()
    };
    let mut metrics = TextMetrics::new();
    let blank = "\u{2002}".repeat(5);
    let plain_width = metrics.measure_text(&blank, &style);
    style.wordprocessingml_form_text_blank_cell = true;
    let form_width = metrics.measure_text(&blank, &style);
    let digit_width = metrics.measure_text("00000", &style);
    assert!((form_width - digit_width).abs() < 0.001);
    style.wordprocessingml_form_text_blank_cell = false;
    assert!((metrics.measure_text(&blank, &style) - plain_width).abs() < 0.001);

    style.font_family = Some(Arc::from("OOXMLSDK Missing Form Placeholder Family"));
    let unavailable_plain_width = metrics.measure_text(&blank, &style);
    style.wordprocessingml_form_text_blank_cell = true;
    assert!((metrics.measure_text(&blank, &style) - unavailable_plain_width).abs() < 0.001);
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
  fn word_arabic_punctuation_retains_required_contextual_positioning() {
    // Native Word's regular/bold and enabled/disabled w:kern controls retain
    // the same Arabic placement arrays. This space has a contextual advance
    // of 200 design units (upem2048), rather than its nominal 500 units.
    let text = "الخصوص. كما ترجو اللجنة ";
    let space = text.find('.').unwrap() + 1;
    let mut metrics = TextMetrics::new();
    for bold in [false, true] {
      for minimum in [Some(7.0), Some(16383.5)] {
        let mut style = crate::model::TextStyle {
          font_family: Some(Arc::from("Traditional Arabic")),
          complex_font_family: Some(Arc::from("Traditional Arabic")),
          font_size_pt: 15.0,
          complex_font_size_pt: Some(15.0),
          bold,
          complex_bold: Some(bold),
          kerning_minimum_size_pt: minimum,
          horizontal_scale: Some(1.03),
          wordprocessing_font_width_percent: Some(103),
          wordprocessing_legacy_font_measurement: Some(true),
          wordprocessingml_font_slots: true,
          right_to_left: Some(true),
          complex_script: Some(true),
          ..Default::default()
        };
        let ideal = metrics.shape_text(text, &style).unwrap();
        let blank = ideal
          .glyphs
          .iter()
          .find(|glyph| glyph.text_range.start == space)
          .unwrap();
        assert_eq!((blank.x_advance_em * 15.0 * 4096.0).round(), 6144.0);
        style.wordprocessing_kashida = Some(Arc::new(crate::common::WordprocessingKashida {
          retain_trailing_blank: false,
          unshaped_blanks: false,
          font_width_percent: 103,
          space_expansions: Vec::new(),
        }));
        let device = metrics.shape_text(text, &style).unwrap();
        let blank = device
          .glyphs
          .iter()
          .find(|glyph| glyph.text_range.start == space)
          .unwrap();
        assert!((blank.x_advance_em * 15.0 - 1.44).abs() < 0.00001);
      }
    }
  }

  #[test]
  fn word_kashida_nbsp_retains_native_unshaped_space_metrics() {
    // Read-only Word lowKashida draw callbacks, before Kashida insertion.
    // U+00A0 is a separate U+0020 Unicode draw run. These are observed whole
    // device widths at 600 DPI, not expectations calculated by this code.
    let cases = [
      (
        "Traditional Arabic",
        false,
        [[24, 25, 26], [31, 32, 34], [37, 38, 40]],
      ),
      (
        "Traditional Arabic",
        true,
        [[24, 25, 26], [31, 32, 34], [37, 38, 41]],
      ),
      ("Arial", false, [[28, 29, 31], [35, 36, 38], [42, 43, 46]]),
      ("Arial", true, [[28, 29, 30], [35, 36, 39], [42, 43, 46]]),
      (
        "Times New Roman",
        false,
        [[25, 26, 28], [31, 32, 34], [38, 39, 42]],
      ),
      (
        "Times New Roman",
        true,
        [[25, 26, 27], [31, 32, 34], [38, 39, 42]],
      ),
    ];
    let mut metrics = TextMetrics::new();
    for (family, bold, widths) in cases {
      for (size_index, size) in [12.0, 15.0, 18.0].into_iter().enumerate() {
        for (percent_index, percent) in [100, 103, 110].into_iter().enumerate() {
          let mut style = crate::model::TextStyle {
            font_family: Some(Arc::from(family)),
            complex_font_family: Some(Arc::from(family)),
            font_size_pt: size,
            complex_font_size_pt: Some(size),
            bold,
            complex_bold: Some(bold),
            horizontal_scale: Some(f32::from(percent) / 100.0),
            wordprocessing_font_width_percent: Some(percent),
            wordprocessing_legacy_font_measurement: Some(true),
            wordprocessingml_font_slots: true,
            right_to_left: Some(true),
            complex_script: Some(true),
            ..Default::default()
          };
          let text = "السياسات\u{00a0}التعليم";
          let ideal = metrics.measure_text(text, &style);
          style.wordprocessing_kashida = Some(Arc::new(crate::common::WordprocessingKashida {
            retain_trailing_blank: false,
            unshaped_blanks: false,
            font_width_percent: percent,
            space_expansions: Vec::new(),
          }));
          let shaped = metrics.shape_text(text, &style).unwrap();
          let blank = shaped
            .glyphs
            .iter()
            .find(|glyph| text.get(glyph.text_range.clone()) == Some("\u{00a0}"))
            .unwrap();
          assert_eq!(
            (blank.x_advance_em * size
              / (crate::units::POINTS_PER_INCH / crate::units::OFFICE_FIXED_OUTPUT_DPI))
              .round() as i32,
            widths[size_index][percent_index],
            "{family}, bold={bold}, size={size}, width={percent}"
          );
          style.wordprocessing_kashida = None;
          assert_eq!(metrics.measure_text(text, &style), ideal);
        }
      }
    }
  }

  #[test]
  fn word_cursive_advances_preserve_native_signed_conversion() {
    let mut metrics = TextMetrics::new();
    for size in [13.0, 15.0] {
      for percent in [100, 103] {
        for italic in [false, true] {
          let mut style = crate::model::TextStyle {
            font_family: Some(Arc::from("Traditional Arabic")),
            complex_font_family: Some(Arc::from("Traditional Arabic")),
            font_size_pt: size,
            complex_font_size_pt: Some(size),
            italic,
            complex_italic: Some(italic),
            horizontal_scale: Some(f32::from(percent) / 100.0),
            wordprocessing_font_width_percent: Some(percent),
            wordprocessing_legacy_font_measurement: Some(true),
            wordprocessingml_font_slots: true,
            right_to_left: Some(true),
            complex_script: Some(true),
            ..Default::default()
          };
          for text in [
            " في مركز بنغلاديش ",
            "قدم هذا المركز اقتراحا بتوفير التدريب التقني لـ ",
          ] {
            // The native placement API retains the negative cursive width;
            // Word converts it to -5 ideal pixels and zero printer pixels.
            let ideal = metrics.shape_text(text, &style).unwrap();
            let glyph = ideal.glyphs.iter().find(|g| g.glyph_id == 450).unwrap();
            let expected = if size == 13.0 { -266.0 } else { -307.0 };
            assert_eq!((glyph.x_advance_em * size * 4096.0).round(), expected);
            style.wordprocessing_kashida = Some(Arc::new(crate::common::WordprocessingKashida {
              retain_trailing_blank: false,
              unshaped_blanks: false,
              font_width_percent: percent,
              space_expansions: Vec::new(),
            }));
            let device = metrics.shape_text(text, &style).unwrap();
            assert!(device.width_pt > 0.0);
            assert_eq!(device.glyphs.len(), ideal.glyphs.len());
            let glyph = device.glyphs.iter().find(|g| g.glyph_id == 450).unwrap();
            assert_eq!(glyph.x_advance_em, 0.0);
            style.wordprocessing_kashida = None;
          }
        }
      }
    }
  }

  #[test]
  fn word_synthetic_italic_preserves_native_ideal_and_device_advances() {
    // Native Word's 32 Arabic controls retain identical ideal and device
    // arrays in every regular/italic pair: two regular-only font families,
    // two sizes, two width percentages, and two paragraph widths. GDI and
    // actual CreateFontIndirectW observations independently retain averages.
    let mut metrics = TextMetrics::new();
    for family in ["Traditional Arabic", "Tahoma"] {
      for size in [15.0, 19.0] {
        for percent in [100, 103] {
          for device in [false, true] {
            let mut style = crate::model::TextStyle {
              font_family: Some(Arc::from(family)),
              complex_font_family: Some(Arc::from(family)),
              font_size_pt: size,
              complex_font_size_pt: Some(size),
              horizontal_scale: Some(f32::from(percent) / 100.0),
              wordprocessing_font_width_percent: Some(percent),
              wordprocessing_legacy_font_measurement: Some(true),
              wordprocessingml_font_slots: true,
              right_to_left: Some(true),
              complex_script: Some(true),
              kerning_minimum_size_pt: Some(32767.0),
              wordprocessing_kashida: device.then(|| {
                Arc::new(crate::common::WordprocessingKashida {
                  retain_trailing_blank: false,
                  unshaped_blanks: false,
                  font_width_percent: percent,
                  space_expansions: Vec::new(),
                })
              }),
              ..Default::default()
            };
            let text = "التدابير الملائمة التدابير الملائمة";
            let regular = metrics.shape_text(text, &style).unwrap();
            style.italic = true;
            style.complex_italic = Some(true);
            let italic = metrics.shape_text(text, &style).unwrap();
            assert!(italic.font_faces.iter().all(|face| face.synthetic_italic));
            assert_eq!(
              regular.width_pt, italic.width_pt,
              "{family}, {size}, {percent}, device={device}"
            );
            assert_eq!(
              regular
                .glyphs
                .iter()
                .map(|g| (g.glyph_id, g.x_advance_em))
                .collect::<Vec<_>>(),
              italic
                .glyphs
                .iter()
                .map(|g| (g.glyph_id, g.x_advance_em))
                .collect::<Vec<_>>(),
              "{family}, {size}, {percent}, device={device}"
            );
          }
        }
      }
    }
  }

  #[test]
  fn word_synthetic_bold_ideal_advances_match_native_font_units() {
    // Native Line Services ideal arrays, captured independently at six sizes
    // for regular-only faces. The space entries also own the extra unit.
    let cases = [
      (
        "Lucida Sans Unicode",
        2048,
        [
          1413, 1289, 1049, 648, 1295, 1295, 1295, 648, 1256, 1070, 1174,
        ],
      ),
      ("Lucida Console", 2048, [1234; 11]),
      (
        "Sylfaen",
        2048,
        [1454, 1075, 913, 512, 1024, 1024, 1024, 512, 999, 1061, 920],
      ),
      ("MS Gothic", 256, [128; 11]),
      (
        "Cambria Math",
        2048,
        [1276, 1121, 903, 451, 1134, 1134, 1134, 451, 990, 1032, 931],
      ),
    ];
    let mut metrics = TextMetrics::new();
    for (family, upem, widths) in cases {
      for size in [8.0, 9.5, 10.0, 12.0, 18.0, 24.0] {
        let style = crate::model::TextStyle {
          font_family: Some(Arc::from(family)),
          font_size_pt: size,
          bold: true,
          // These native controls have no authored Word kerning threshold.
          kerning_minimum_size_pt: Some(f32::MAX),
          wordprocessingml_font_slots: true,
          ..Default::default()
        };
        let shaped = metrics
          .shape_text("Abc 123 xyz", &style)
          .expect("native face");
        assert!(
          shaped.font_faces.iter().all(|face| face.synthetic_bold),
          "{family}"
        );
        assert_eq!(shaped.glyphs.len(), widths.len(), "{family}");
        for (glyph, width) in shaped.glyphs.iter().zip(widths) {
          let expected = (width + 1) as f32 * size / upem as f32;
          assert!(
            (glyph.x_advance_em * glyph.font_size_pt - expected).abs() < 0.0001,
            "{family} {size}: {glyph:?}"
          );
        }
        let expected = (widths.iter().sum::<i32>() + 11) as f32 * size / upem as f32;
        assert!(
          (shaped.width_pt - expected).abs() < 0.0001,
          "{family} {size}"
        );
      }
    }
  }

  #[test]
  fn word_synthetic_bold_prefix_preserves_generic_and_real_bold_metrics() {
    let mut metrics = TextMetrics::new();
    let mut style = crate::model::TextStyle {
      font_family: Some(Arc::from("Lucida Calligraphy")),
      font_size_pt: 16.0,
      bold: true,
      ..Default::default()
    };
    let text = format!("{}L", " ".repeat(27));
    let generic = metrics.measure_text(&text, &style);
    assert!((generic - 154.148_44).abs() < 0.0001);
    style.wordprocessingml_font_slots = true;
    assert!((metrics.measure_text(&text, &style) - 154.367_19).abs() < 0.0001);
    style.wordprocessingml_font_slots = false;
    assert_eq!(metrics.measure_text(&text, &style), generic);

    style.font_family = Some(Arc::from("Times New Roman"));
    let generic = metrics
      .shape_text("Abc 123 xyz", &style)
      .expect("real bold face");
    assert!(generic.font_faces.iter().all(|face| !face.synthetic_bold));
    style.wordprocessingml_font_slots = true;
    let word = metrics
      .shape_text("Abc 123 xyz", &style)
      .expect("real bold face");
    assert_eq!(word.width_pt, generic.width_pt);
    assert_eq!(word.glyphs.len(), generic.glyphs.len());
    for (word, generic) in word.glyphs.iter().zip(&generic.glyphs) {
      assert_eq!(word.glyph_id, generic.glyph_id);
      assert_eq!(word.text_range, generic.text_range);
      assert_eq!(word.x_advance_em, generic.x_advance_em);
      assert_eq!(word.bounds_em, generic.bounds_em);
    }
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
  fn word_small_caps_realizes_half_point_sizes_before_device_sizes() {
    // Native Word Cambria controls: nominal, synthesized nominal, painted em.
    // Capital letters and punctuation retain the full run size.
    let cases = [
      (4.5, 3.5, 3.48),
      (8.0, 6.5, 6.48),
      (8.5, 7.0, 6.96),
      (9.5, 7.5, 7.56),
      (10.0, 8.0, 8.04),
      (11.0, 9.0, 9.0),
      (12.0, 9.5, 9.48),
      (14.0, 11.0, 11.04),
      (16.0, 13.0, 12.96),
      (18.0, 14.5, 14.52),
      (20.0, 16.0, 15.96),
    ];
    let mut metrics = TextMetrics::new();
    for (nominal, synthesized, painted) in cases {
      for realized in [false, true] {
        let full = if realized {
          crate::units::quantize_points_to_office_print_grid(nominal)
        } else {
          nominal
        };
        let style = TextStyle {
          font_family: Some("Cambria".into()),
          font_size: Pt(full),
          small_caps: true,
          wordprocessingml_font_slots: true,
          layout_font_sizes: realized.then_some(crate::common::LayoutFontSizes {
            primary: Pt(nominal),
            complex: None,
          }),
          ..test_style()
        };
        let shaped = metrics.shape_text("Aa,1", &style).expect("small caps");
        assert_eq!(shaped.glyphs.len(), 4);
        for glyph in &shaped.glyphs {
          let expected = if glyph.text_range.start == 1 {
            if realized { painted } else { synthesized }
          } else {
            full
          };
          assert!(
            (glyph.font_size_pt - expected).abs() < 0.001,
            "nominal={nominal} realized={realized}: {:?}",
            glyph
          );
        }
      }
    }
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
  fn measurement_cache_keys_every_shaping_feature() {
    let plain = test_style();
    let mut old_style = plain.clone();
    old_style.open_type_features.number_form = Some(crate::common::OpenTypeNumberForm::OldStyle);
    old_style.open_type_features.number_spacing =
      Some(crate::common::OpenTypeNumberSpacing::Proportional);
    let mut ligatures = plain.clone();
    ligatures.ligatures = Some(crate::common::OpenTypeLigatures {
      standard: true,
      ..Default::default()
    });
    let mut metrics = TextMetrics::new();

    let plain_index = metrics.measure_style_index(&plain);
    let mut nominal_controls = plain.clone();
    nominal_controls.wordprocessing_nominal_control_metrics = true;
    assert_ne!(plain_index, metrics.measure_style_index(&nominal_controls));
    let old_style_index = metrics.measure_style_index(&old_style);
    let ligature_index = metrics.measure_style_index(&ligatures);

    assert_ne!(plain_index, old_style_index);
    assert_ne!(plain_index, ligature_index);
    assert_ne!(old_style_index, ligature_index);
    assert_eq!(old_style_index, metrics.measure_style_index(&old_style));
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
      .gdi_hinted_text_extents_pt("iiii", &style, device_dpi)
      .expect("simple hinted run")
      .positioned_width_pt;
    let extent_px = extent_pt * device_dpi / crate::units::POINTS_PER_INCH;

    assert!((extent_px - extent_px.round()).abs() < 0.0001);
  }

  #[test]
  fn gdi_hinting_instance_is_reused_across_text_runs() {
    let style = test_style();
    let mut metrics = TextMetrics::new();
    let device_dpi = 600.0;

    metrics
      .gdi_hinted_text_extents_pt("iiii", &style, device_dpi)
      .expect("first simple hinted run");
    let instance_count = metrics.gdi_hinting_instances.instances.len();
    assert!(instance_count > 0);

    metrics
      .gdi_hinted_text_extents_pt("WWWW", &style, device_dpi)
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
  fn synthetic_bold_spacing_matches_windows_device_widths() {
    let mut metrics = TextMetrics::new();
    for (size, expected_advance) in [(9.48, 4.92), (13.32, 6.84), (17.16, 8.76)] {
      let style = TextStyle {
        font_family: Some("SimHei".into()),
        font_size: Pt(size),
        bold: true,
        ..TextStyle::default()
      };
      let spacing = metrics
        .gdi_synthetic_bold_character_spacing_pt("Blood Pressure Tracker", &style, 600.0)
        .expect("SimHei synthetic bold");
      assert!((size * 0.5 + spacing - expected_advance).abs() < 0.0001);
      let regular = TextStyle {
        bold: false,
        ..style
      };
      assert!(
        metrics
          .gdi_synthetic_bold_character_spacing_pt("Blood", &regular, 600.0)
          .is_none()
      );
    }
    let real_bold = TextStyle {
      font_family: Some("Arial".into()),
      bold: true,
      ..TextStyle::default()
    };
    assert!(
      metrics
        .gdi_synthetic_bold_character_spacing_pt("Blood", &real_bold, 600.0)
        .is_none()
    );
  }

  #[test]
  fn rotated_alignment_extent_matches_office_top_bottom_controls() {
    let mut metrics = TextMetrics::new();
    for (size, text, width, padding) in [
      (7.08, "CCCCCCCCCC", 37.2, 2.04),
      (4.68, "CCCCCCCCCC", 25.2, 1.56),
      (7.08, "Constructed Key Field", 78.12, 2.04),
    ] {
      let style = TextStyle {
        font_family: Some("SimSun".into()),
        font_size: Pt(size),
        bold: true,
        ..TextStyle::default()
      };
      let actual = metrics
        .excel_rotated_layout_extents_pt(text, &style)
        .expect("simple printer text");
      assert!((actual.0 - width).abs() < 0.0001, "{actual:?}");
      assert!((actual.1 - padding).abs() < 0.0001, "{actual:?}");
    }
  }

  #[test]
  fn rotated_printer_advances_match_office_xps_device_projection() {
    let mut metrics = TextMetrics::new();
    for (bold, angle, expected) in [
      (true, 15.0, [3.6, 3.456]),
      (true, 30.0, [3.6, 3.48]),
      (true, 45.0, [3.552, 3.552]),
      (true, 60.0, [3.6, 3.432]),
      (false, 15.0, [3.456, 3.456]),
      (false, 30.0, [3.48, 3.432]),
      (false, 45.0, [3.552, 3.384]),
      (false, 60.0, [3.432, 3.432]),
    ] {
      let style = TextStyle {
        font_family: Some("SimSun".into()),
        font_size: Pt(7.08),
        bold,
        ..TextStyle::default()
      };
      for sign in [-1.0, 1.0] {
        let advances = metrics
          .excel_rotated_character_advances_pt("CCCCCCCCCC", &style, angle * sign)
          .expect("simple slanted run");
        for (index, advance) in advances.iter().enumerate() {
          assert!(
            (advance - expected[index % 2]).abs() < 0.0001,
            "bold={bold}, angle={angle}, character={index}: {advance}"
          );
        }
      }
      assert!(
        metrics
          .excel_rotated_character_advances_pt("CC", &style, 90.0)
          .is_none()
      );
    }
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
  fn word_default_charset_line_leading_matches_native_font_records() {
    use ooxmlsdk_fonts::{FontCharset, FontFamilyClass, FontPitch};
    let mut metrics = TextMetrics::new();
    for (family, class, pitch, expected_step) in [
      (
        "Liberation Serif",
        FontFamilyClass::Serif,
        FontPitch::Variable,
        17.28,
      ),
      (
        "Liberation Sans",
        FontFamilyClass::SansSerif,
        FontPitch::Variable,
        17.4,
      ),
      (
        "Calibri",
        FontFamilyClass::SansSerif,
        FontPitch::Variable,
        19.08,
      ),
      (
        "Courier New",
        FontFamilyClass::Fixed,
        FontPitch::Fixed,
        17.64,
      ),
      (
        "Arial",
        FontFamilyClass::SansSerif,
        FontPitch::Variable,
        13.8,
      ),
      (
        "Times New Roman",
        FontFamilyClass::Serif,
        FontPitch::Variable,
        13.8,
      ),
    ] {
      let style = TextStyle {
        font_family: Some(family.into()),
        font_family_class: Some(class),
        font_charset: Some(FontCharset::Other(1)),
        font_pitch: Some(pitch),
        font_size: Pt(12.0),
        wordprocessingml_font_slots: true,
        ..TextStyle::default()
      };
      // Native PDF steps are rounded to the export device; allow one twip.
      let actual = metrics.line_vertical_metrics_for_text("Text", &style);
      assert!(
        (actual.line_height_pt() - expected_step).abs() < 0.05,
        "{family}: {actual:?}"
      );
      let physical = metrics.vertical_metrics(&style);
      let drawing = TextStyle {
        wordprocessingml_font_slots: false,
        ..style.clone()
      };
      assert_eq!(
        metrics.line_vertical_metrics_for_text("Text", &drawing),
        physical
      );
      let ansi = TextStyle {
        font_charset: Some(FontCharset::Ansi),
        ..style.clone()
      };
      assert_eq!(
        metrics.line_vertical_metrics_for_text("Text", &ansi),
        physical
      );
      let automatic_family = TextStyle {
        font_family_class: None,
        ..style
      };
      assert_eq!(
        metrics.line_vertical_metrics_for_text("Text", &automatic_family),
        physical
      );
    }
  }

  #[test]
  fn wordprocessingml_cjk_capability_adds_symmetric_side_leading() {
    let metrics = TextVerticalMetrics {
      ascent_pt: 20.0,
      descent_pt: 6.0,
      windows_line_height_pt: 26.0,
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

    let with_intrinsic_leading = TextVerticalMetrics {
      line_gap_pt: 4.0,
      ..metrics
    };
    let adjusted = wordprocessingml_line_vertical_metrics(&style, with_intrinsic_leading);
    assert!((adjusted.ascent_pt - 21.9).abs() < 0.0001);
    assert!((adjusted.descent_pt - 7.9).abs() < 0.0001);
    assert!((adjusted.line_height_pt() - 33.8).abs() < 0.0001);
    assert!((adjusted.baseline_offset_pt - 21.9).abs() < 0.0001);
    assert!((adjusted.directwrite_baseline_offset_pt - 21.9).abs() < 0.0001);

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
