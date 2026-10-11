use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use ooxmlsdk_fonts::{
  FeatureValue, FontBytes, FontCharset, FontFallbackChain, FontFamilyClass, FontFamilySelection,
  FontId, FontRegistry, FontRequest, FontScriptRun, FontSize, FontSlant, ResolvedFontChain,
  ScriptScanOptions, ShapeOptions, ShapedRun, TextDirection, TextScript, WordprocessingFontSlot,
  script_direction_runs_with_options,
};
use rustc_hash::FxHashMap as HashMap;
use skrifa::raw::{FontRef, TableProvider};

use crate::common;
use crate::docx::TextStyle;

fn font_timing<T>(label: &str, work: impl FnOnce() -> T) -> T {
  static ENABLED: OnceLock<bool> = OnceLock::new();
  if !ENABLED.get_or_init(|| std::env::var_os("OOXMLSDK_FONT_TIMING").is_some()) {
    return work();
  }
  let start = Instant::now();
  let output = work();
  eprintln!("[ooxmlsdk-layout] {label}: {:?}", start.elapsed());
  output
}

#[derive(Clone, Debug)]
pub struct FontFaceData {
  pub data: Arc<FontBytes>,
  pub index: u32,
  pub synthetic_bold: bool,
  pub synthetic_italic: bool,
  id: Arc<str>,
}

impl FontFaceData {
  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn cache_key(&self) -> FontFaceCacheKey {
    FontFaceCacheKey {
      id: self.id.clone(),
      index: self.index,
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FontFaceCacheKey {
  id: Arc<str>,
  index: u32,
}

impl FontFaceCacheKey {
  pub fn matches_face(&self, face: &FontFaceData) -> bool {
    self.index == face.index && self.id == face.id
  }
}

impl PartialEq for FontFaceData {
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index && self.id == other.id
  }
}

impl Eq for FontFaceData {}

impl Hash for FontFaceData {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.index.hash(state);
    self.id.hash(state);
  }
}

pub trait FontStyleRef {
  fn font_family(&self) -> Option<&str>;
  fn symbol_font_family(&self) -> Option<&str> {
    None
  }
  fn high_ansi_font_family(&self) -> Option<&str> {
    self.font_family()
  }
  fn fallback_font_family(&self) -> Option<&str> {
    None
  }
  fn high_ansi_fallback_font_family(&self) -> Option<&str> {
    self.fallback_font_family()
  }
  fn east_asia_fallback_font_family(&self) -> Option<&str> {
    None
  }
  fn complex_fallback_font_family(&self) -> Option<&str> {
    None
  }
  fn font_family_class(&self) -> Option<FontFamilyClass> {
    None
  }
  fn high_ansi_font_family_class(&self) -> Option<FontFamilyClass> {
    self.font_family_class()
  }
  fn east_asia_font_family_class(&self) -> Option<FontFamilyClass> {
    None
  }
  fn complex_font_family_class(&self) -> Option<FontFamilyClass> {
    None
  }
  fn east_asia_font_family(&self) -> Option<&str> {
    self.font_family()
  }
  fn drawingml_japanese_font_family(&self) -> Option<&str> {
    None
  }
  fn complex_font_family(&self) -> Option<&str> {
    self.font_family()
  }
  fn font_size_pt(&self) -> f32;
  fn complex_font_size_pt(&self) -> Option<f32> {
    None
  }
  fn complex_script_override(&self) -> Option<bool> {
    None
  }
  fn complex_script_property(&self) -> Option<bool> {
    None
  }
  fn right_to_left(&self) -> bool {
    false
  }
  fn resolved_bidi_level(&self) -> Option<u8> {
    None
  }
  fn complex_bold(&self) -> Option<bool> {
    None
  }
  fn complex_italic(&self) -> Option<bool> {
    None
  }
  fn shaping_context(&self) -> Option<&common::TextShapingContext> {
    None
  }
  fn kashida_expansions(&self) -> &[common::KashidaExpansion] {
    &[]
  }
  fn wordprocessing_kashida(&self) -> Option<&common::WordprocessingKashida> {
    None
  }
  fn character_spacing_pt(&self) -> f32;
  fn baseline_shift_pt(&self) -> f32;
  fn automatic_escapement_font_sizes_pt(&self) -> Option<(f32, Option<f32>)> {
    None
  }
  fn bold(&self) -> bool;
  fn italic(&self) -> bool;
  fn small_caps(&self) -> bool;
  fn kerning_enabled(&self) -> bool {
    true
  }
  fn ligatures(&self) -> Option<common::OpenTypeLigatures> {
    None
  }
  fn open_type_features(&self) -> common::OpenTypeFeatureSettings {
    common::OpenTypeFeatureSettings::default()
  }
  fn horizontal_scale(&self) -> f32 {
    1.0
  }
  /// Authored sizes retained when a Word display font has been realized.
  fn wordprocessing_layout_font_sizes(&self) -> Option<common::LayoutFontSizes> {
    None
  }
  /// Word's document measurement profile and authored font width percentage.
  fn wordprocessing_measurement_profile(&self) -> Option<(bool, u16)> {
    None
  }
  fn wordprocessingml_font_slots(&self) -> bool {
    false
  }
  fn wordprocessing_nominal_control_metrics(&self) -> bool {
    false
  }
  fn wordprocessingml_form_text_blank_cell(&self) -> bool {
    false
  }
  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    false
  }
  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    None
  }
  fn wordprocessingml_resolved_font_slot(&self) -> Option<WordprocessingFontSlot> {
    None
  }
  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    false
  }
  fn wordprocessingml_bidi_language_is_hebrew(&self) -> bool {
    false
  }
  fn font_charset(&self) -> Option<FontCharset> {
    None
  }
  fn high_ansi_font_charset(&self) -> Option<FontCharset> {
    self.font_charset()
  }
  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    None
  }
  fn complex_font_charset(&self) -> Option<FontCharset> {
    None
  }
  fn font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    None
  }
  fn high_ansi_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.font_pitch()
  }
  fn east_asia_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    None
  }
  fn complex_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    None
  }
  /// Per-source-character expansion resolved by the completed Word line.
  fn wordprocessing_justification_expansion_pt(&self) -> &[f32] {
    &[]
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    0.0
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    false
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    false
  }
  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    false
  }
}

impl<T: FontStyleRef + ?Sized> FontStyleRef for Box<T> {
  fn font_family(&self) -> Option<&str> {
    (**self).font_family()
  }

  fn symbol_font_family(&self) -> Option<&str> {
    (**self).symbol_font_family()
  }

  fn high_ansi_font_family(&self) -> Option<&str> {
    (**self).high_ansi_font_family()
  }

  fn fallback_font_family(&self) -> Option<&str> {
    (**self).fallback_font_family()
  }

  fn high_ansi_fallback_font_family(&self) -> Option<&str> {
    (**self).high_ansi_fallback_font_family()
  }

  fn east_asia_fallback_font_family(&self) -> Option<&str> {
    (**self).east_asia_fallback_font_family()
  }

  fn complex_fallback_font_family(&self) -> Option<&str> {
    (**self).complex_fallback_font_family()
  }

  fn font_family_class(&self) -> Option<FontFamilyClass> {
    (**self).font_family_class()
  }

  fn high_ansi_font_family_class(&self) -> Option<FontFamilyClass> {
    (**self).high_ansi_font_family_class()
  }

  fn east_asia_font_family_class(&self) -> Option<FontFamilyClass> {
    (**self).east_asia_font_family_class()
  }

  fn complex_font_family_class(&self) -> Option<FontFamilyClass> {
    (**self).complex_font_family_class()
  }

  fn east_asia_font_family(&self) -> Option<&str> {
    (**self).east_asia_font_family()
  }
  fn drawingml_japanese_font_family(&self) -> Option<&str> {
    (**self).drawingml_japanese_font_family()
  }

  fn complex_font_family(&self) -> Option<&str> {
    (**self).complex_font_family()
  }

  fn font_size_pt(&self) -> f32 {
    (**self).font_size_pt()
  }

  fn complex_font_size_pt(&self) -> Option<f32> {
    (**self).complex_font_size_pt()
  }

  fn complex_script_override(&self) -> Option<bool> {
    (**self).complex_script_override()
  }

  fn complex_script_property(&self) -> Option<bool> {
    (**self).complex_script_property()
  }

  fn right_to_left(&self) -> bool {
    (**self).right_to_left()
  }

  fn resolved_bidi_level(&self) -> Option<u8> {
    (**self).resolved_bidi_level()
  }

  fn complex_bold(&self) -> Option<bool> {
    (**self).complex_bold()
  }

  fn complex_italic(&self) -> Option<bool> {
    (**self).complex_italic()
  }

  fn shaping_context(&self) -> Option<&common::TextShapingContext> {
    (**self).shaping_context()
  }
  fn kashida_expansions(&self) -> &[common::KashidaExpansion] {
    (**self).kashida_expansions()
  }
  fn wordprocessing_kashida(&self) -> Option<&common::WordprocessingKashida> {
    (**self).wordprocessing_kashida()
  }

  fn character_spacing_pt(&self) -> f32 {
    (**self).character_spacing_pt()
  }

  fn baseline_shift_pt(&self) -> f32 {
    (**self).baseline_shift_pt()
  }

  fn automatic_escapement_font_sizes_pt(&self) -> Option<(f32, Option<f32>)> {
    (**self).automatic_escapement_font_sizes_pt()
  }

  fn bold(&self) -> bool {
    (**self).bold()
  }

  fn italic(&self) -> bool {
    (**self).italic()
  }

  fn small_caps(&self) -> bool {
    (**self).small_caps()
  }

  fn kerning_enabled(&self) -> bool {
    (**self).kerning_enabled()
  }

  fn ligatures(&self) -> Option<common::OpenTypeLigatures> {
    (**self).ligatures()
  }

  fn open_type_features(&self) -> common::OpenTypeFeatureSettings {
    (**self).open_type_features()
  }

  fn horizontal_scale(&self) -> f32 {
    (**self).horizontal_scale()
  }
  fn wordprocessing_layout_font_sizes(&self) -> Option<common::LayoutFontSizes> {
    (**self).wordprocessing_layout_font_sizes()
  }

  fn wordprocessing_measurement_profile(&self) -> Option<(bool, u16)> {
    (**self).wordprocessing_measurement_profile()
  }

  fn wordprocessingml_font_slots(&self) -> bool {
    (**self).wordprocessingml_font_slots()
  }
  fn wordprocessing_nominal_control_metrics(&self) -> bool {
    (**self).wordprocessing_nominal_control_metrics()
  }

  fn wordprocessingml_form_text_blank_cell(&self) -> bool {
    (**self).wordprocessingml_form_text_blank_cell()
  }

  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    (**self).wordprocessingml_cjk_line_metrics()
  }

  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    (**self).wordprocessingml_font_hint()
  }
  fn wordprocessingml_resolved_font_slot(&self) -> Option<WordprocessingFontSlot> {
    (**self).wordprocessingml_resolved_font_slot()
  }

  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    (**self).wordprocessingml_east_asia_language_is_chinese()
  }
  fn wordprocessingml_bidi_language_is_hebrew(&self) -> bool {
    (**self).wordprocessingml_bidi_language_is_hebrew()
  }

  fn font_charset(&self) -> Option<FontCharset> {
    (**self).font_charset()
  }

  fn high_ansi_font_charset(&self) -> Option<FontCharset> {
    (**self).high_ansi_font_charset()
  }

  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    (**self).wordprocessingml_east_asia_font_charset()
  }

  fn complex_font_charset(&self) -> Option<FontCharset> {
    (**self).complex_font_charset()
  }

  fn font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    (**self).font_pitch()
  }

  fn high_ansi_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    (**self).high_ansi_font_pitch()
  }

  fn east_asia_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    (**self).east_asia_font_pitch()
  }

  fn complex_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    (**self).complex_font_pitch()
  }

  fn wordprocessing_justification_expansion_pt(&self) -> &[f32] {
    (**self).wordprocessing_justification_expansion_pt()
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    (**self).cjk_punctuation_compression_ratio()
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    (**self).wordprocessingml_legacy_punctuation_spacing()
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    (**self).wordprocessingml_punctuation_spacing()
  }

  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    (**self).wordprocessingml_balance_single_byte_double_byte_width()
  }
}

fn complex_script_override(
  complex_script: Option<bool>,
  right_to_left: Option<bool>,
) -> Option<bool> {
  if complex_script == Some(true) || right_to_left == Some(true) {
    Some(true)
  } else {
    None
  }
}

fn uses_complex_run_properties(style: &(impl FontStyleRef + ?Sized)) -> bool {
  // MS-OI29500 §17.3.2.1/.2, §17.3.2.13/.16 and §17.3.2.38/.39:
  // Word selects b/bCs, i/iCs and sz/szCs from the state of cs and rtl.
  // Unicode script classification remains relevant to rFonts only.
  style.complex_script_override() == Some(true)
}

pub(crate) fn script_scan_options(
  style: &(impl FontStyleRef + ?Sized),
  small_caps: bool,
) -> ScriptScanOptions {
  ScriptScanOptions {
    small_caps,
    wordprocessingml_font_slots: style.wordprocessingml_font_slots(),
    wordprocessingml_font_hint: style.wordprocessingml_font_hint(),
    wordprocessingml_resolved_font_slot: style.wordprocessingml_resolved_font_slot(),
    wordprocessingml_east_asia_language_is_chinese: style
      .wordprocessingml_east_asia_language_is_chinese(),
    wordprocessingml_east_asia_font_charset: style.wordprocessingml_east_asia_font_charset(),
    wordprocessingml_complex_font_override: style.complex_script_override() == Some(true),
    wordprocessingml_rtl_font_override: style.right_to_left()
      && style.complex_script_property() != Some(true),
    wordprocessingml_east_asia_uses_ascii: wordprocessingml_east_asia_uses_ascii(style),
    ..ScriptScanOptions::default()
  }
}

fn wordprocessing_line_metrics_font_slot(
  style: &(impl FontStyleRef + ?Sized),
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<WordprocessingFontSlot> {
  // [MS-OI29500] §2.1.88 states that w:cs/w:rtl selects the cs face
  // regardless of the run's Unicode values. Word fixed output paints Basic
  // Latin decimal digits with the ASCII family, but Comment066 demonstrates
  // that those glyphs do not contribute ASCII-face ascender/descender values
  // to the line box. Keep the paint exception out of line measurement.
  if style.complex_script_override() == Some(true)
    && wordprocessingml_font_slot == Some(WordprocessingFontSlot::Ascii)
  {
    Some(WordprocessingFontSlot::ComplexScript)
  } else {
    wordprocessingml_font_slot
  }
}

fn wordprocessingml_east_asia_uses_ascii(style: &(impl FontStyleRef + ?Sized)) -> bool {
  style
    .east_asia_font_family()
    .is_some_and(|family| family.eq_ignore_ascii_case("Times New Roman"))
    && style
      .font_family()
      .zip(style.high_ansi_font_family())
      .is_some_and(|(ascii, high_ansi)| ascii.eq_ignore_ascii_case(high_ansi))
}

pub(crate) fn effective_font_size_pt(
  style: &(impl FontStyleRef + ?Sized),
  _script: Option<TextScript>,
) -> f32 {
  if uses_complex_run_properties(style) {
    style.complex_font_size_pt().unwrap_or(style.font_size_pt())
  } else {
    style.font_size_pt()
  }
}

fn font_size_reaches_kerning_minimum(font_size_pt: f32, minimum_size_pt: f32) -> bool {
  if !minimum_size_pt.is_finite() {
    return false;
  }
  if !font_size_pt.is_finite() {
    return font_size_pt.is_sign_positive();
  }
  // DrawingML stores both `sz` and `kern` in hundredths of a point, but a
  // group transform can turn an authored equality such as 66.00 == 66.00
  // into adjacent f32 values. Keep the comparison inclusive as specified,
  // while limiting the tolerance to ordinary floating-point accumulation;
  // a real one-hundredth-point counterexample must remain below threshold.
  let tolerance = 16.0 * f32::EPSILON * font_size_pt.abs().max(minimum_size_pt.abs()).max(1.0);
  font_size_pt + tolerance >= minimum_size_pt
}

fn effective_bold(style: &(impl FontStyleRef + ?Sized), _script: Option<TextScript>) -> bool {
  if uses_complex_run_properties(style) {
    style.complex_bold().unwrap_or(false)
  } else {
    style.bold()
  }
}

fn effective_italic(style: &(impl FontStyleRef + ?Sized), _script: Option<TextScript>) -> bool {
  if uses_complex_run_properties(style) {
    style.complex_italic().unwrap_or(false)
  } else {
    style.italic()
  }
}

fn wordprocessingml_synthetic_rtl_italic(style: &(impl FontStyleRef + ?Sized)) -> bool {
  style.wordprocessingml_font_slots()
    && style.right_to_left()
    && effective_italic(style, None)
    && !style.wordprocessingml_bidi_language_is_hebrew()
}

fn apply_wordprocessingml_rtl_italic_face(
  request: &mut FontRequest<'_>,
  registry: &FontRegistry<'_>,
  style: &(impl FontStyleRef + ?Sized),
) {
  if !wordprocessingml_synthetic_rtl_italic(style) {
    return;
  }
  let Ok(primary) = registry.resolve(request) else {
    return;
  };
  if registry
    .face(&primary.font_id)
    .is_none_or(|face| face.slant == FontSlant::Upright)
    || registered_font_has_arabic_charset(registry, &primary.font_id) != Some(false)
  {
    return;
  }
  let mut upright_request = request.clone();
  upright_request.slant = Some(FontSlant::Upright);
  if let Ok(upright) = registry.resolve(&upright_request)
    && upright
      .resolved_family
      .eq_ignore_ascii_case(&primary.resolved_family)
    && registry
      .face(&upright.font_id)
      .is_some_and(|face| face.slant == FontSlant::Upright)
    && registered_font_has_arabic_charset(registry, &upright.font_id) == Some(true)
  {
    // Word's Arabic RTL font realization preserves the family when its
    // italic face lacks Arabic: use the upright face with synthetic italics,
    // including Latin portions of that run. A real Arabic italic face (e.g.
    // Amiri) remains preferred. Hebrew bidi language and w:cs alone retain
    // ordinary italic selection. This is font selection, not text fallback.
    request.slant = Some(FontSlant::Upright);
  }
}

fn registered_font_has_arabic_charset(
  registry: &FontRegistry<'_>,
  font_id: &FontId,
) -> Option<bool> {
  let (data, index) = registry.font_face_binary(font_id)?;
  let face = FontRef::from_index(data.as_ref(), index).ok()?;
  // OpenType OS/2 ulCodePageRange1 bit 6 is Windows Arabic (1256), the
  // physical-font charset used by GDI. Missing metadata is not a mismatch.
  Some(face.os2().ok()?.ul_code_page_range_1()? & (1 << 6) != 0)
}

pub(crate) fn arabic_shaping_properties_match(left: &TextStyle, right: &TextStyle) -> bool {
  let script = Some(TextScript::Arabic);
  let same_family = match (
    script_font_family_for_slot(left, script, None),
    script_font_family_for_slot(right, script, None),
  ) {
    (Some(left), Some(right)) => left.eq_ignore_ascii_case(right),
    (None, None) => true,
    _ => false,
  };
  // Word ends Arabic joining at an authored font, size, emphasis, spacing
  // or scaling change,
  // even when different missing families resolve to the same fallback face.
  // Paint, language and baseline changes retain the joining context. Compare
  // the selected properties, so an unused ASCII font change in a w:rtl run
  // cannot interrupt its complex-script text.
  same_family
    && effective_font_size_pt(left, script) == effective_font_size_pt(right, script)
    && effective_bold(left, script) == effective_bold(right, script)
    && effective_italic(left, script) == effective_italic(right, script)
    && left.character_spacing_pt == right.character_spacing_pt
    && left.horizontal_scale() == right.horizontal_scale()
}

pub(crate) fn arabic_nonspacing_mark(character: char) -> bool {
  use icu_properties::{CodePointMapData, props::GeneralCategory};
  use unicode_script::{Script, UnicodeScript};

  let scripts = character.script_extension();
  // Common/Inherited extensions intersect every script; they do not establish
  // Arabic ownership. In particular, emoji variation selectors must remain
  // ignorable rather than acquiring Arabic BOT/dotted-circle behavior.
  CodePointMapData::<GeneralCategory>::new().get(character) == GeneralCategory::NonspacingMark
    && !scripts.is_common()
    && !scripts.is_inherited()
    && scripts.contains_script(Script::Arabic)
}

pub(crate) fn materialize_wordprocessingml_source_font_slot(
  style: &TextStyle,
  source_character: char,
) -> TextStyle {
  if !style.wordprocessingml_font_slots {
    return style.clone();
  }

  let mut encoded = [0; 4];
  let source = source_character.encode_utf8(&mut encoded);
  let slot = script_direction_runs_with_options(
    source,
    FontSize(style.font_size_pt),
    script_scan_options(style, false),
  )
  .first()
  .and_then(|run| run.wordprocessingml_font_slot);
  let Some(slot) = slot else {
    return style.clone();
  };

  // ECMA-376 Part 1 §17.3.2.26 selects the WordprocessingML rFonts slot
  // from the serialized character. OfficeMath §§22.1.2.94 and 22.1.2.111
  // then realize m:scr/m:sty through a Unicode mathematical-alphabet scalar.
  // Materialize the already-selected source slot into every family route so
  // the realized scalar cannot be classified a second time. Keep the Word
  // font-slot mode enabled because it also carries Word OpenType defaults.
  let family = script_font_family_for_slot(style, None, Some(slot)).map(Arc::<str>::from);
  let fallback =
    script_fallback_font_family_for_slot(style, None, Some(slot)).map(Arc::<str>::from);
  let family_class = script_font_family_class_for_slot(style, None, Some(slot));
  let charset = font_charset_for_slot(style, None, Some(slot));
  let pitch = font_pitch_for_slot(style, None, Some(slot));
  let mut materialized = style.clone();
  materialized.font_family = family.clone();
  materialized.high_ansi_font_family = family.clone();
  materialized.east_asia_font_family = family.clone();
  materialized.complex_font_family = family;
  materialized.fallback_font_family = fallback.clone();
  materialized.high_ansi_fallback_font_family = fallback.clone();
  materialized.east_asia_fallback_font_family = fallback.clone();
  materialized.complex_fallback_font_family = fallback;
  materialized.font_family_class = family_class;
  materialized.high_ansi_font_family_class = family_class;
  materialized.east_asia_font_family_class = family_class;
  materialized.complex_font_family_class = family_class;
  materialized.font_charset = charset;
  materialized.high_ansi_font_charset = charset;
  materialized.east_asia_font_charset = charset;
  materialized.complex_font_charset = charset;
  materialized.font_pitch = pitch;
  materialized.high_ansi_font_pitch = pitch;
  materialized.east_asia_font_pitch = pitch;
  materialized.complex_font_pitch = pitch;
  materialized
}

impl FontStyleRef for TextStyle {
  fn font_family(&self) -> Option<&str> {
    self.font_family.as_deref()
  }

  fn symbol_font_family(&self) -> Option<&str> {
    self.symbol_font_family.as_deref()
  }

  fn high_ansi_font_family(&self) -> Option<&str> {
    self
      .high_ansi_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn fallback_font_family(&self) -> Option<&str> {
    self.fallback_font_family.as_deref()
  }

  fn high_ansi_fallback_font_family(&self) -> Option<&str> {
    self
      .high_ansi_fallback_font_family
      .as_deref()
      .or_else(|| self.fallback_font_family())
  }

  fn east_asia_fallback_font_family(&self) -> Option<&str> {
    self.east_asia_fallback_font_family.as_deref()
  }

  fn complex_fallback_font_family(&self) -> Option<&str> {
    self.complex_fallback_font_family.as_deref()
  }

  fn font_family_class(&self) -> Option<FontFamilyClass> {
    self.font_family_class
  }

  fn high_ansi_font_family_class(&self) -> Option<FontFamilyClass> {
    self.high_ansi_font_family_class.or(self.font_family_class)
  }

  fn east_asia_font_family_class(&self) -> Option<FontFamilyClass> {
    self.east_asia_font_family_class
  }

  fn complex_font_family_class(&self) -> Option<FontFamilyClass> {
    self.complex_font_family_class
  }

  fn east_asia_font_family(&self) -> Option<&str> {
    self
      .east_asia_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn drawingml_japanese_font_family(&self) -> Option<&str> {
    self.drawingml_japanese_font_family.as_deref()
  }

  fn complex_font_family(&self) -> Option<&str> {
    self
      .complex_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn font_size_pt(&self) -> f32 {
    self.font_size_pt
  }

  fn complex_font_size_pt(&self) -> Option<f32> {
    self.complex_font_size_pt
  }

  fn complex_script_override(&self) -> Option<bool> {
    complex_script_override(self.complex_script, self.right_to_left)
  }

  fn complex_script_property(&self) -> Option<bool> {
    self.complex_script
  }

  fn right_to_left(&self) -> bool {
    self.right_to_left == Some(true)
  }

  fn wordprocessing_nominal_control_metrics(&self) -> bool {
    self.wordprocessing_nominal_control_metrics
  }

  fn resolved_bidi_level(&self) -> Option<u8> {
    self.resolved_bidi_level
  }

  fn complex_bold(&self) -> Option<bool> {
    self.complex_bold
  }

  fn complex_italic(&self) -> Option<bool> {
    self.complex_italic
  }

  fn shaping_context(&self) -> Option<&common::TextShapingContext> {
    self.shaping_context.as_deref()
  }
  fn kashida_expansions(&self) -> &[common::KashidaExpansion] {
    self.kashida_expansions.as_deref().unwrap_or_default()
  }
  fn wordprocessing_kashida(&self) -> Option<&common::WordprocessingKashida> {
    self.wordprocessing_kashida.as_deref()
  }

  fn character_spacing_pt(&self) -> f32 {
    self.character_spacing_pt
  }

  fn baseline_shift_pt(&self) -> f32 {
    self.baseline_shift_pt
  }

  fn automatic_escapement_font_sizes_pt(&self) -> Option<(f32, Option<f32>)> {
    self
      .automatic_escapement_font_size_pt
      .map(|size| (size, self.automatic_escapement_complex_font_size_pt))
  }

  fn bold(&self) -> bool {
    self.bold
  }

  fn italic(&self) -> bool {
    self.italic
  }

  fn small_caps(&self) -> bool {
    self.small_caps
  }

  fn kerning_enabled(&self) -> bool {
    self.kerning_minimum_size_pt.is_none_or(|minimum| {
      font_size_reaches_kerning_minimum(effective_font_size_pt(self, None), minimum)
    })
  }

  fn ligatures(&self) -> Option<common::OpenTypeLigatures> {
    self.ligatures
  }

  fn open_type_features(&self) -> common::OpenTypeFeatureSettings {
    self.open_type_features
  }

  fn horizontal_scale(&self) -> f32 {
    self.horizontal_scale.unwrap_or(1.0)
  }
  fn wordprocessing_measurement_profile(&self) -> Option<(bool, u16)> {
    self.wordprocessing_legacy_font_measurement.map(|legacy| {
      (
        legacy,
        self.wordprocessing_font_width_percent.unwrap_or(100),
      )
    })
  }

  fn wordprocessingml_font_slots(&self) -> bool {
    self.wordprocessingml_font_slots
  }

  fn wordprocessingml_form_text_blank_cell(&self) -> bool {
    self.wordprocessingml_form_text_blank_cell
  }

  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    self.wordprocessingml_cjk_line_metrics
  }

  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    self.wordprocessingml_font_hint
  }
  fn wordprocessingml_resolved_font_slot(&self) -> Option<WordprocessingFontSlot> {
    self.wordprocessingml_resolved_font_slot
  }

  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    self
      .east_asia_language
      .as_deref()
      .and_then(|language| language.split(['-', '_']).next())
      .is_some_and(|language| language.eq_ignore_ascii_case("zh"))
  }
  fn wordprocessingml_bidi_language_is_hebrew(&self) -> bool {
    self
      .bidi_language
      .as_deref()
      .and_then(|language| language.split(['-', '_']).next())
      .is_some_and(|language| {
        language.eq_ignore_ascii_case("he") || language.eq_ignore_ascii_case("iw")
      })
  }

  fn font_charset(&self) -> Option<FontCharset> {
    self.font_charset
  }

  fn high_ansi_font_charset(&self) -> Option<FontCharset> {
    self.high_ansi_font_charset.or(self.font_charset)
  }

  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    self.east_asia_font_charset
  }

  fn complex_font_charset(&self) -> Option<FontCharset> {
    self.complex_font_charset
  }

  fn font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.font_pitch
  }

  fn high_ansi_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.high_ansi_font_pitch.or(self.font_pitch)
  }

  fn east_asia_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.east_asia_font_pitch
  }

  fn complex_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.complex_font_pitch
  }

  fn wordprocessing_justification_expansion_pt(&self) -> &[f32] {
    self
      .wordprocessing_justification_expansion_pt
      .as_deref()
      .unwrap_or_default()
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    self.cjk_punctuation_compression_ratio
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    self.wordprocessingml_legacy_punctuation_spacing
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    self.wordprocessingml_punctuation_spacing
  }

  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    self.wordprocessingml_balance_single_byte_double_byte_width
  }
}

impl FontStyleRef for common::TextStyle<'_> {
  fn font_family(&self) -> Option<&str> {
    self.font_family.as_deref()
  }

  fn symbol_font_family(&self) -> Option<&str> {
    self.symbol_font_family.as_deref()
  }

  fn high_ansi_font_family(&self) -> Option<&str> {
    self
      .high_ansi_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn fallback_font_family(&self) -> Option<&str> {
    self.fallback_font_family.as_deref()
  }

  fn high_ansi_fallback_font_family(&self) -> Option<&str> {
    self
      .high_ansi_fallback_font_family
      .as_deref()
      .or_else(|| self.fallback_font_family())
  }

  fn east_asia_fallback_font_family(&self) -> Option<&str> {
    self.east_asia_fallback_font_family.as_deref()
  }

  fn complex_fallback_font_family(&self) -> Option<&str> {
    self.complex_fallback_font_family.as_deref()
  }

  fn font_family_class(&self) -> Option<FontFamilyClass> {
    self.font_family_class
  }

  fn high_ansi_font_family_class(&self) -> Option<FontFamilyClass> {
    self.high_ansi_font_family_class.or(self.font_family_class)
  }

  fn east_asia_font_family_class(&self) -> Option<FontFamilyClass> {
    self.east_asia_font_family_class
  }

  fn complex_font_family_class(&self) -> Option<FontFamilyClass> {
    self.complex_font_family_class
  }

  fn east_asia_font_family(&self) -> Option<&str> {
    self
      .east_asia_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn drawingml_japanese_font_family(&self) -> Option<&str> {
    self.drawingml_japanese_font_family.as_deref()
  }

  fn complex_font_family(&self) -> Option<&str> {
    self
      .complex_font_family
      .as_deref()
      .or_else(|| self.font_family())
  }

  fn font_size_pt(&self) -> f32 {
    self.font_size.0
  }

  fn complex_font_size_pt(&self) -> Option<f32> {
    self.complex_font_size.map(|size| size.0)
  }

  fn complex_script_override(&self) -> Option<bool> {
    complex_script_override(self.complex_script, self.right_to_left)
  }

  fn complex_script_property(&self) -> Option<bool> {
    self.complex_script
  }

  fn right_to_left(&self) -> bool {
    self.right_to_left == Some(true)
  }

  fn wordprocessing_nominal_control_metrics(&self) -> bool {
    self.wordprocessing_nominal_control_metrics
  }

  fn resolved_bidi_level(&self) -> Option<u8> {
    self.resolved_bidi_level
  }

  fn complex_bold(&self) -> Option<bool> {
    self.complex_bold
  }

  fn complex_italic(&self) -> Option<bool> {
    self.complex_italic
  }

  fn shaping_context(&self) -> Option<&common::TextShapingContext> {
    self.shaping_context.as_deref()
  }
  fn kashida_expansions(&self) -> &[common::KashidaExpansion] {
    self.kashida_expansions.as_deref().unwrap_or_default()
  }
  fn wordprocessing_kashida(&self) -> Option<&common::WordprocessingKashida> {
    self.wordprocessing_kashida.as_deref()
  }

  fn character_spacing_pt(&self) -> f32 {
    self.character_spacing.0
  }

  fn baseline_shift_pt(&self) -> f32 {
    self.baseline_shift.0
  }

  fn automatic_escapement_font_sizes_pt(&self) -> Option<(f32, Option<f32>)> {
    self.automatic_escapement_font_size.map(|size| {
      (
        size.0,
        self
          .automatic_escapement_complex_font_size
          .map(|size| size.0),
      )
    })
  }

  fn bold(&self) -> bool {
    self.bold
  }

  fn italic(&self) -> bool {
    self.italic
  }

  fn small_caps(&self) -> bool {
    self.small_caps
  }

  fn kerning_enabled(&self) -> bool {
    self.kerning_minimum_size.is_none_or(|minimum| {
      font_size_reaches_kerning_minimum(effective_font_size_pt(self, None), minimum.0)
    })
  }

  fn ligatures(&self) -> Option<common::OpenTypeLigatures> {
    self.ligatures
  }

  fn open_type_features(&self) -> common::OpenTypeFeatureSettings {
    self.open_type_features
  }

  fn horizontal_scale(&self) -> f32 {
    self.horizontal_scale.unwrap_or(1.0)
  }
  fn wordprocessing_layout_font_sizes(&self) -> Option<common::LayoutFontSizes> {
    self.layout_font_sizes
  }

  fn wordprocessing_measurement_profile(&self) -> Option<(bool, u16)> {
    // Realized display fonts own paint metrics, not Word's ideal layout font.
    if self.layout_font_sizes.is_some() {
      return None;
    }
    self.wordprocessing_legacy_font_measurement.map(|legacy| {
      (
        legacy,
        self.wordprocessing_font_width_percent.unwrap_or(100),
      )
    })
  }

  fn wordprocessingml_font_slots(&self) -> bool {
    self.wordprocessingml_font_slots
  }

  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    self.wordprocessingml_cjk_line_metrics
  }

  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    self.wordprocessingml_font_hint
  }
  fn wordprocessingml_resolved_font_slot(&self) -> Option<WordprocessingFontSlot> {
    self.wordprocessingml_resolved_font_slot
  }

  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    self.wordprocessingml_east_asia_language_is_chinese
  }
  fn wordprocessingml_bidi_language_is_hebrew(&self) -> bool {
    self.wordprocessingml_bidi_language_is_hebrew
  }

  fn font_charset(&self) -> Option<FontCharset> {
    self.font_charset
  }

  fn high_ansi_font_charset(&self) -> Option<FontCharset> {
    self.high_ansi_font_charset.or(self.font_charset)
  }

  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    self.wordprocessingml_east_asia_font_charset
  }

  fn complex_font_charset(&self) -> Option<FontCharset> {
    self.complex_font_charset
  }

  fn font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.font_pitch
  }

  fn high_ansi_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.high_ansi_font_pitch.or(self.font_pitch)
  }

  fn east_asia_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.east_asia_font_pitch
  }

  fn complex_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self.complex_font_pitch
  }

  fn wordprocessing_justification_expansion_pt(&self) -> &[f32] {
    self
      .wordprocessing_justification_expansion_pt
      .as_deref()
      .unwrap_or_default()
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    self.cjk_punctuation_compression_ratio
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    self.wordprocessingml_legacy_punctuation_spacing
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    self.wordprocessingml_punctuation_spacing
  }

  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    self.wordprocessingml_balance_single_byte_double_byte_width
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FullWidthPunctuationSide {
  Left,
  Right,
  Middle,
}

fn full_width_punctuation_side(ch: char) -> Option<FullWidthPunctuationSide> {
  use FullWidthPunctuationSide::{Left, Middle, Right};
  match ch {
    '\u{3008}' | '\u{300A}' | '\u{300C}' | '\u{300E}' | '\u{3010}' | '\u{3014}' | '\u{3016}'
    | '\u{3018}' | '\u{301A}' | '\u{301D}' | '\u{FF08}' | '\u{FF3B}' | '\u{FF5B}' => Some(Left),
    '\u{3009}' | '\u{300B}' | '\u{300D}' | '\u{300F}' | '\u{3011}' | '\u{3015}' | '\u{3017}'
    | '\u{3019}' | '\u{301B}' | '\u{301E}' | '\u{301F}' | '\u{FF09}' | '\u{FF3D}' | '\u{FF5D}' => {
      Some(Right)
    }
    '\u{3001}' | '\u{3002}' | '\u{FF0C}' | '\u{FF0E}' => Some(Right),
    '\u{FF1A}' | '\u{FF1B}' => Some(Middle),
    _ => None,
  }
}

fn punctuation_pair_tightens(
  first: Option<FullWidthPunctuationSide>,
  second: Option<FullWidthPunctuationSide>,
  legacy: bool,
) -> bool {
  use FullWidthPunctuationSide::{Left, Middle, Right};
  first.is_some()
    && second.is_some()
    && !(legacy && (first == Some(Middle) || second == Some(Middle)))
    && (second == Some(Left) || first == Some(Right))
}

pub(crate) fn wordprocessingml_punctuation_pair_tightens(
  left: Option<char>,
  right: Option<char>,
  style: &(impl FontStyleRef + ?Sized),
) -> bool {
  style.wordprocessingml_punctuation_spacing()
    && punctuation_pair_tightens(
      left.and_then(full_width_punctuation_side),
      right.and_then(full_width_punctuation_side),
      style.wordprocessingml_legacy_punctuation_spacing(),
    )
}

/// Whether Word's formatter changed punctuation advances from the face's
/// ordinary metrics. PDF natural-width balancing must preserve these explicit
/// spacing decisions instead of treating them as device rounding residuals.
pub fn wordprocessingml_punctuation_affects_advances(
  text: &str,
  style: &(impl FontStyleRef + ?Sized),
) -> bool {
  let mut previous = None;
  for ch in text.chars() {
    let current = full_width_punctuation_side(ch);
    if current.is_some() && style.cjk_punctuation_compression_ratio() > 0.0 {
      return true;
    }
    if style.wordprocessingml_punctuation_spacing()
      && punctuation_pair_tightens(
        previous,
        current,
        style.wordprocessingml_legacy_punctuation_spacing(),
      )
    {
      return true;
    }
    previous = current;
  }
  false
}

#[cfg(test)]
mod punctuation_spacing_tests {
  use super::*;

  #[test]
  fn word_justification_expansion_preserves_script_offsets_and_measure_cache() {
    let mut metrics = crate::text_metrics::TextMetrics::new();
    let mut style = TextStyle {
      font_family: Some("Century".into()),
      east_asia_font_family: Some("ＭＳ 明朝".into()),
      font_size_pt: 12.0,
      wordprocessingml_font_slots: true,
      ..TextStyle::default()
    };
    let text = "漢AB漢";
    let natural = metrics.shape_text(text, &style).unwrap();
    let width = metrics.measure_text(text, &style);
    // Native12pt automatic script gaps use3pt on either side of AB. The
    // source-character slots must survive the Latin/CJK shaping-run split.
    style.wordprocessing_justification_expansion_pt = Some(Arc::from([3.0, 0.0, 3.0, 0.0]));
    let expanded = metrics.shape_text(text, &style).unwrap();
    for ((before, after), extra) in natural
      .glyphs
      .iter()
      .zip(&expanded.glyphs)
      .zip([3.0, 0.0, 3.0, 0.0])
    {
      assert_eq!(before.glyph_id, after.glyph_id);
      assert_eq!(before.text_range, after.text_range);
      assert!(
        (after.x_advance_em * after.font_size_pt
          - before.x_advance_em * before.font_size_pt
          - extra)
          .abs()
          < 0.001
      );
    }
    assert!((metrics.measure_text(text, &style) - width - 6.0).abs() < 0.001);
    style.wordprocessing_justification_expansion_pt = Some(Arc::from([1.0, 0.0, 1.0, 0.0]));
    assert!((metrics.measure_text(text, &style) - width - 2.0).abs() < 0.001);
    style.wordprocessing_justification_expansion_pt = None;
    assert_eq!(metrics.measure_text(text, &style), width);
  }

  #[test]
  fn word_pair_spacing_is_independent_of_line_compression() {
    let mut resolver = FontResolver::default();
    let mut style = TextStyle {
      font_family: Some("ＭＳ 明朝".into()),
      east_asia_font_family: Some("ＭＳ 明朝".into()),
      font_size_pt: 12.0,
      wordprocessingml_font_slots: true,
      wordprocessingml_punctuation_spacing: true,
      ..Default::default()
    };
    for (text, modern, legacy) in [
      ("漢。（漢", 42.0, 42.0),
      ("漢（（漢", 42.0, 42.0),
      ("漢）、漢", 42.0, 42.0),
      ("漢：（漢", 42.0, 48.0),
      ("漢）：漢", 42.0, 48.0),
      ("漢：；漢", 48.0, 48.0),
      ("漢（）漢", 48.0, 48.0),
      ("漢。漢", 36.0, 36.0),
    ] {
      for (old_spacing, expected) in [(false, modern), (true, legacy)] {
        style.wordprocessingml_legacy_punctuation_spacing = old_spacing;
        let runs = resolver.shape_text_runs(text, &style).unwrap();
        let width: f32 = runs.iter().map(|run| run.advance_pt).sum();
        assert!((width - expected).abs() < 0.001, "{text}: {width}");
      }
    }
    style.wordprocessingml_font_slots = false;
    style.wordprocessingml_punctuation_spacing = false;
    let width: f32 = resolver
      .shape_text_runs("漢。（漢", &style)
      .unwrap()
      .iter()
      .map(|run| run.advance_pt)
      .sum();
    assert_eq!(width, 48.0);
  }

  #[test]
  fn compression_moves_opening_ink_but_preserves_trailing_punctuation_ink() {
    let mut resolver = FontResolver::default();
    let style = TextStyle {
      font_family: Some("ＭＳ 明朝".into()),
      east_asia_font_family: Some("ＭＳ 明朝".into()),
      font_size_pt: 12.0,
      wordprocessingml_font_slots: true,
      wordprocessingml_punctuation_spacing: true,
      cjk_punctuation_compression_ratio: 1.0,
      ..Default::default()
    };
    for (text, offset) in [("（", -6.0), ("）", 0.0), ("。", 0.0), ("：", -3.0)] {
      let runs = resolver.shape_text_runs(text, &style).unwrap();
      assert_eq!(runs[0].advance_pt, 6.0);
      assert_eq!(runs[0].glyphs[0].x_offset_pt, offset);
    }
  }
}

fn apply_wordprocessingml_punctuation_compression(
  run: &mut ShapedRun<'_, '_>,
  text: &str,
  ratio: f32,
  wordprocessing: bool,
  legacy_spacing: bool,
) {
  let ratio = ratio.clamp(0.0, 1.0);
  if ratio <= f32::EPSILON && !wordprocessing {
    return;
  }
  let minimum_full_width = run.font_size_pt.0 * 0.75;
  let mut total_reduction = 0.0;
  for glyph in run.glyphs.to_mut() {
    let Some(side) = glyph.source_char.and_then(full_width_punctuation_side) else {
      continue;
    };
    if glyph.x_advance_pt < minimum_full_width {
      continue;
    }
    let next = text
      .get(glyph.text_range.end..)
      .and_then(|tail| tail.chars().next())
      .and_then(full_width_punctuation_side);
    // JLREQ §3.1.4: consecutive opening/closing punctuation shares its
    // inter-character half-em space. Word applies this natural pair spacing
    // even with characterSpacingControl=doNotCompress; it is not capacity
    // which the line formatter may later return. Native Mincho/SimSun pair
    // matrices also distinguish centered colons from trailing commas/stops.
    if wordprocessing && punctuation_pair_tightens(Some(side), next, legacy_spacing) {
      let reduction = glyph.x_advance_pt * 0.5;
      glyph.x_advance_pt -= reduction;
      total_reduction += reduction;
      continue;
    }
    // ECMA-376 Part 1 §17.15.1.18 limits this setting to full-width
    // punctuation. A full-width punctuation cell has at most one half-em of
    // removable side-bearing; the line formatter below returns whatever
    // fraction is not needed for the selected break.
    let reduction = glyph.x_advance_pt * 0.5 * ratio;
    glyph.x_advance_pt -= reduction;
    match side {
      FullWidthPunctuationSide::Left => glyph.x_offset_pt -= reduction,
      FullWidthPunctuationSide::Right => {}
      FullWidthPunctuationSide::Middle => glyph.x_offset_pt -= reduction * 0.5,
    }
    total_reduction += reduction;
  }
  run.advance_pt = (run.advance_pt - total_reduction).max(0.0);
}

fn apply_wordprocessingml_single_double_byte_width_balance(
  run: &mut ShapedRun<'_, '_>,
  horizontal_scale: f32,
  character_spacing_pt: f32,
) {
  let inside_cjk_script = matches!(
    run.script,
    Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul)
  );
  let target_space_advance =
    run.font_size_pt.0 * 0.5 * horizontal_scale.max(f32::EPSILON) + character_spacing_pt;
  let run_start = run.text_range.start;
  let mut total_adjustment = 0.0;

  for glyph in run.glyphs.to_mut() {
    if glyph.source_char != Some(' ') {
      continue;
    }
    let Some(local_start) = glyph.text_range.start.checked_sub(run_start) else {
      continue;
    };
    let Some(local_end) = glyph.text_range.end.checked_sub(run_start) else {
      continue;
    };
    if local_start > local_end || local_end > run.text.len() {
      continue;
    }
    // This function also sees trial prefixes while Word line fitting. A
    // shaping-fragment edge is not a logical text edge, so it cannot qualify
    // an otherwise isolated Latin space. Office's Indent_Spacing.Template
    // output and Writer's tdf#88908 test both retain the legacy adjustment for
    // adjacent spaces in proportional faces.
    let previous_matches =
      inside_cjk_script || (local_start > 0 && run.text[..local_start].ends_with(' '));
    let next_matches =
      inside_cjk_script || (local_end < run.text.len() && run.text[local_end..].starts_with(' '));
    if !previous_matches && !next_matches {
      continue;
    }

    total_adjustment += target_space_advance - glyph.x_advance_pt;
    glyph.x_advance_pt = target_space_advance;
  }
  run.advance_pt += total_adjustment;
}

pub(crate) fn named_font_has_direct_symbol_byte(
  style: &(impl FontStyleRef + ?Sized),
  byte: u8,
) -> bool {
  let mut request = font_request(style, None);
  let Some(family) = request.family.as_deref() else {
    return false;
  };
  // This is a query for an exact authored face, not a charset substitution
  // request. In particular, do not trigger a characteristics search across
  // system fonts because stale Word metadata says Symbol. Platform family
  // queries and parsed face metadata use the existing shared caches.
  request.charset = None;
  request.family_class = None;
  request.pitch = None;
  let mut registry = FontRegistry::new();
  if registry.register_system_query_fonts(&request).is_err() {
    return false;
  }
  let Ok(resolved) = registry.resolve(&request) else {
    return false;
  };
  registry
    .face(&resolved.font_id)
    .is_some_and(|face| font_face_has_direct_symbol_byte(face, family, byte))
}

fn font_face_has_direct_symbol_byte(
  face: &ooxmlsdk_fonts::FontFaceInfo<'_>,
  family: &str,
  byte: u8,
) -> bool {
  // A substitute cannot establish the original face's encoding. Limit this
  // correction to an exact, nonsymbolic face with one unambiguous Unicode
  // selector. A dual-mapped font requires separate evidence of equivalence.
  !face.flags.symbolic
    && face
      .family_names
      .iter()
      .any(|name| name.trim().eq_ignore_ascii_case(family.trim()))
    && face.coverage.contains_char(char::from(byte))
    && !face
      .coverage
      .contains_char(char::from_u32(0xF000 | u32::from(byte)).expect("legacy symbol byte"))
}

pub fn load_text_face(style: &(impl FontStyleRef + ?Sized)) -> Option<FontFaceData> {
  FontResolver::default().load_text_face(style)
}

#[derive(Debug, Default)]
pub struct FontResolver {
  font_data_cache: HashMap<FontId, FontFaceData>,
  font_synthesis_cache: HashMap<FontId, (bool, bool)>,
  font_registry_cache: HashMap<FontFaceKey, Arc<FontRegistry<'static>>>,
  font_selection_cache: HashMap<FontFaceKey, ResolvedFontChain<'static>>,
  font_face_cache: HashMap<FontFaceKey, FontFaceData>,
  font_metrics_cache: HashMap<FontMetricsKey, FontMetrics>,
  last_font_registry: Option<(FontFaceKey, Arc<FontRegistry<'static>>)>,
  last_font_face: Option<FontFaceKey>,
  last_font_metrics: Option<(FontMetricsKey, FontMetrics)>,
}

impl FontResolver {
  pub(crate) fn has_exact_ascii_face(&mut self, style: &(impl FontStyleRef + ?Sized)) -> bool {
    let script = Some(TextScript::Latin);
    let slot = Some(WordprocessingFontSlot::Ascii);
    let request = font_request_for_slot(style, script, slot);
    let Some(family) = request.family.as_deref() else {
      return false;
    };
    let registry = self.style_font_registry_for_slot(style, script, slot);
    registry
      .resolve(&request)
      .ok()
      .and_then(|resolved| registry.face(&resolved.font_id))
      .is_some_and(|face| {
        face
          .family_names
          .iter()
          .any(|name| name.trim().eq_ignore_ascii_case(family.trim()))
      })
  }

  pub fn load_text_face(&mut self, style: &(impl FontStyleRef + ?Sized)) -> Option<FontFaceData> {
    let mut request = font_request(style, None);
    let registry = self.style_font_registry(style, None);
    apply_wordprocessingml_rtl_italic_face(&mut request, &registry, style);
    let resolved = registry.resolve(&request).ok()?;
    self.font_synthesis_cache.insert(
      resolved.font_id.clone(),
      (resolved.synthetic_bold, resolved.synthetic_italic),
    );
    self.font_face_data_from_registry(&registry, &resolved.font_id)
  }

  pub fn cached_text_face(&mut self, style: &(impl FontStyleRef + ?Sized)) -> Option<FontFaceData> {
    self.with_cached_text_face(style, Clone::clone)
  }

  pub fn with_cached_text_face<T>(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    read: impl FnOnce(&FontFaceData) -> T,
  ) -> Option<T> {
    if let Some(key) = &self.last_font_face
      && key.matches_style(style, None)
    {
      return self.font_face_cache.get(key).map(read);
    }
    let key = FontFaceKey::from_style(style, None);
    if !self.font_face_cache.contains_key(&key) {
      let face = self.load_text_face(style)?;
      self.font_face_cache.insert(key.clone(), face);
    }
    self.last_font_face = Some(key.clone());
    self.font_face_cache.get(&key).map(read)
  }

  pub fn shape_text_runs<'text>(
    &mut self,
    text: &'text str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<Vec<ShapedRun<'text, 'static>>> {
    font_timing("shape text runs", || {
      self.shape_text_runs_inner(text, style, &[])
    })
  }

  pub(crate) fn shape_text_runs_with_features<'text>(
    &mut self,
    text: &'text str,
    style: &(impl FontStyleRef + ?Sized),
    features: &[FeatureValue<'_>],
  ) -> Option<Vec<ShapedRun<'text, 'static>>> {
    font_timing("shape text runs with features", || {
      self.shape_text_runs_inner(text, style, features)
    })
  }

  pub fn font_face_data(&self, font_id: &FontId) -> Option<FontFaceData> {
    let mut face = self.font_data_cache.get(font_id).cloned()?;
    if let Some((synthetic_bold, synthetic_italic)) = self.font_synthesis_cache.get(font_id) {
      face.synthetic_bold = *synthetic_bold;
      face.synthetic_italic = *synthetic_italic;
    }
    Some(face)
  }

  pub fn vertical_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<ooxmlsdk_fonts::VerticalMetrics> {
    self
      .font_metrics(style, None)
      .map(|metrics| metrics.vertical)
  }

  pub(crate) fn vertical_metrics_for_script(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: TextScript,
  ) -> Option<ooxmlsdk_fonts::VerticalMetrics> {
    self
      .font_metrics(style, Some(script))
      .map(|metrics| metrics.vertical)
  }

  pub fn decoration_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<ooxmlsdk_fonts::DecorationMetrics> {
    self
      .font_metrics(style, None)
      .map(|metrics| metrics.decoration)
  }

  pub(crate) fn text_vertical_metrics(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<ooxmlsdk_fonts::VerticalMetrics> {
    let script_runs = script_direction_runs_with_options(
      text,
      FontSize(style.font_size_pt()),
      script_scan_options(style, style.small_caps()),
    );
    let needs_script_metrics = style.wordprocessingml_font_slots()
      || style.complex_script_override() == Some(true)
      || script_runs.iter().any(|run| {
        matches!(
          run.script,
          TextScript::Arabic | TextScript::Hebrew | TextScript::Devanagari | TextScript::Thai
        )
      });
    if !needs_script_metrics {
      return self.vertical_metrics(style);
    }

    let mut combined: Option<ooxmlsdk_fonts::VerticalMetrics> = None;
    self.for_each_script_vertical_metric(text, style, &script_runs, |metrics| {
      include_vertical_metrics(&mut combined, metrics);
    })?;
    combined
  }

  fn for_each_script_vertical_metric(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
    script_runs: &[FontScriptRun],
    mut include: impl FnMut(ooxmlsdk_fonts::VerticalMetrics),
  ) -> Option<()> {
    if !script_runs.iter().any(|run| {
      wordprocessingml_east_asia_font_mapping(
        style,
        wordprocessing_line_metrics_font_slot(style, run.wordprocessingml_font_slot),
      )
    }) {
      for run in script_runs {
        let segment = &text[run.text_range.clone()];
        if style.wordprocessingml_font_slots()
          && !segment.is_empty()
          && segment.chars().all(wordprocessingml_join_control)
        {
          continue;
        }
        include(
          self
            .font_metrics_for_slot(
              style,
              Some(run.script),
              wordprocessing_line_metrics_font_slot(style, run.wordprocessingml_font_slot),
            )?
            .vertical,
        );
      }
      return Some(());
    }

    // The EA mapper and subsequent glyph link can select different physical
    // faces for one declared font. Word's line and paint baselines use those
    // realized faces, not the unsupported request. Shape the complete source
    // once so neutral characters keep the same script context as the painter.
    let shaped_runs = self.shape_text_runs(text, style)?;
    for shaped in shaped_runs {
      if !shaped.text.is_empty() && shaped.text.chars().all(wordprocessingml_join_control) {
        // A zero-width joining control can link to a different font, but its
        // invisible glyph does not add that face's box to the Word line.
        // Native 14/56pt ZWJ/ZWNJ controls retain the adjacent 14pt text box
        // for Latin, Arabic-substitute and physical CJK font requests alike.
        // Keep the control in shaping and PDF source text.
        continue;
      }
      let index = script_runs.partition_point(|run| run.text_range.end <= shaped.text_range.start);
      let run = script_runs.get(index)?;
      let slot = wordprocessing_line_metrics_font_slot(style, run.wordprocessingml_font_slot);
      let vertical = if wordprocessingml_east_asia_font_mapping(style, slot) {
        let registry = self.style_font_registry_for_slot(style, Some(run.script), slot);
        registry
          .face(&shaped.font_id)?
          .metrics
          // Automatic escapement and small caps keep the authored line size;
          // their reduced display glyphs must not shrink the surrounding box.
          .scaled(effective_font_size_pt(style, Some(run.script)))
          .vertical
      } else {
        self
          .font_metrics_for_slot(style, Some(run.script), slot)?
          .vertical
      };
      include(vertical);
    }
    Some(())
  }

  pub(crate) fn realized_cjk_vertical_metrics(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<ooxmlsdk_fonts::VerticalMetrics> {
    let runs = self.shape_text_runs(text, style)?;
    let mut combined = None;
    for run in runs {
      if !matches!(
        run.script,
        Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul)
      ) || run.text.chars().all(char::is_whitespace)
      {
        continue;
      }
      let slot = style
        .wordprocessingml_font_slots()
        .then_some(WordprocessingFontSlot::EastAsia);
      let registry = self.style_font_registry_for_slot(style, run.script, slot);
      // A requested EA face may lack Hangul, for example SimSun. Use the
      // face that actually shaped the glyphs, including registered fallback,
      // rather than measuring the unsupported request or a Latin separator.
      let metrics = registry
        .face(&run.font_id)?
        .metrics
        .scaled(run.font_size_pt.0);
      include_vertical_metrics(&mut combined, metrics.vertical);
    }
    combined
  }

  pub(crate) fn wordprocessingml_line_metric_runs(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<Vec<ooxmlsdk_fonts::VerticalMetrics>> {
    if !style.wordprocessingml_font_slots() {
      return None;
    }
    let runs = script_direction_runs_with_options(
      text,
      FontSize(style.font_size_pt()),
      script_scan_options(style, style.small_caps()),
    );
    let mut metrics = Vec::new();
    self.for_each_script_vertical_metric(text, style, &runs, |vertical| metrics.push(vertical))?;
    (!metrics.is_empty()).then_some(metrics)
  }

  fn font_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
  ) -> Option<FontMetrics> {
    self.font_metrics_for_slot(style, script, None)
  }

  pub(crate) fn wordprocessingml_default_charset_line_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    text: &str,
  ) -> Option<ooxmlsdk_fonts::VerticalMetrics> {
    if !style.wordprocessingml_font_slots()
      || ![
        style.font_charset(),
        style.high_ansi_font_charset(),
        style.wordprocessingml_east_asia_font_charset(),
        style.complex_font_charset(),
      ]
      .contains(&Some(FontCharset::Other(1)))
    {
      return None;
    }
    let mut combined = None;
    let mut has_default_charset = false;
    for run in script_direction_runs_with_options(
      text,
      FontSize(style.font_size_pt()),
      script_scan_options(style, style.small_caps()),
    ) {
      let slot = wordprocessing_line_metrics_font_slot(style, run.wordprocessingml_font_slot);
      let metrics = self.font_metrics_for_slot(style, Some(run.script), slot)?;
      let vertical = if let Some(logical) = metrics.wordprocessingml_default_charset {
        has_default_charset = true;
        word_font_side_leading(logical)
      } else if style.wordprocessingml_cjk_line_metrics()
        && metrics.vertical.wordprocessingml_cjk_line_metrics
      {
        word_font_side_leading(metrics.vertical)
      } else {
        metrics.vertical
      };
      include_vertical_metrics(&mut combined, vertical);
    }
    has_default_charset.then_some(combined).flatten()
  }

  fn font_metrics_for_slot(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
    wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  ) -> Option<FontMetrics> {
    if let Some((key, metrics)) = &self.last_font_metrics
      && key.matches_style_for_slot(style, script, wordprocessingml_font_slot)
    {
      return Some(*metrics);
    }
    let key = FontMetricsKey::from_style_for_slot(style, script, wordprocessingml_font_slot);
    if let Some(metrics) = self.font_metrics_cache.get(&key) {
      let metrics = *metrics;
      self.last_font_metrics = Some((key, metrics));
      return Some(metrics);
    }
    let mut request = font_request_for_slot(style, script, wordprocessingml_font_slot);
    let registry = self.style_font_registry_for_slot(style, script, wordprocessingml_font_slot);
    apply_wordprocessingml_east_asia_font_mapping(
      &mut request,
      &registry,
      style,
      wordprocessingml_font_slot,
    );
    apply_wordprocessingml_rtl_italic_face(&mut request, &registry, style);
    let resolved = registry.resolve(&request).ok()?;
    let metrics_at_size = resolved.metrics_at_size(FontSize(effective_font_size_pt(style, script)));
    let metrics = FontMetrics {
      vertical: metrics_at_size.vertical,
      decoration: metrics_at_size.decoration,
      wordprocessingml_default_charset: wordprocessingml_default_charset_metrics(
        &request, &resolved,
      ),
    };
    self.font_metrics_cache.insert(key.clone(), metrics);
    self.last_font_metrics = Some((key, metrics));
    Some(metrics)
  }

  fn shape_text_runs_inner<'text>(
    &mut self,
    text: &'text str,
    style: &(impl FontStyleRef + ?Sized),
    additional_features: &[FeatureValue<'_>],
  ) -> Option<Vec<ShapedRun<'text, 'static>>> {
    let base_size = style.font_size_pt();
    let mut script_runs = script_direction_runs_with_options(
      text,
      FontSize(base_size),
      script_scan_options(style, style.small_caps()),
    );
    if style.wordprocessingml_font_slots()
      && style
        .shaping_context()
        .is_none_or(|context| context.before.is_empty())
      && script_runs.iter().any(|run| {
        text[run.text_range.clone()]
          .chars()
          .take_while(|&ch| arabic_nonspacing_mark(ch))
          .count()
          > 1
      })
    {
      // Uniscribe gives each baseless Arabic mark its own dotted-circle
      // base. HarfRust supplies one for a whole leading cluster instead.
      // Preserve source ranges and let its normal single-mark shaping own
      // the glyph, fallback, positioning and advance of each orphan.
      let mut separated = Vec::with_capacity(script_runs.len());
      let combining =
        icu_properties::CodePointMapData::<icu_properties::props::CanonicalCombiningClass>::new();
      for run in script_runs {
        let mut marks = text[run.text_range.clone()]
          .char_indices()
          .take_while(|&(_, ch)| arabic_nonspacing_mark(ch))
          .collect::<Vec<_>>();
        if marks.len() <= 1 {
          separated.push(run);
          continue;
        }
        let prefix_end = run.text_range.start
          + marks
            .last()
            .map_or(0, |&(offset, ch)| offset + ch.len_utf8());
        // Word's Arabic mark ordering puts shadda before the vowel marks,
        // independently of their serialized order. Splitting must retain
        // that order without changing the source text or character ranges.
        marks.sort_by_key(|&(_, ch)| (ch != '\u{0651}', combining.get(ch)));
        for (offset, ch) in marks {
          let mut mark = run.clone();
          let start = run.text_range.start + offset;
          mark.text_range = start..start + ch.len_utf8();
          separated.push(mark);
        }
        if prefix_end < run.text_range.end {
          let mut suffix = run;
          suffix.text_range.start = prefix_end;
          separated.push(suffix);
        }
      }
      script_runs = separated;
    }
    let mut output = Vec::with_capacity(script_runs.len());
    for mut script_run in script_runs {
      let segment_text = &text[script_run.text_range.clone()];
      let leading_arabic_mark = segment_text
        .chars()
        .next()
        .is_some_and(arabic_nonspacing_mark);
      if style.wordprocessingml_font_slots()
        && script_run.script == TextScript::Common
        && leading_arabic_mark
        && segment_text.chars().all(arabic_nonspacing_mark)
      {
        // A standalone Arabic diacritic has Unicode Script=Inherited.
        // Word still uses the Arabic engine for its dotted-circle fallback,
        // including Arabic mark positioning and ideal font measurements.
        script_run.script = TextScript::Arabic;
        script_run.direction = TextDirection::RightToLeft;
      }
      let slot = script_run.wordprocessingml_font_slot;
      let key = FontFaceKey::from_style_for_slot(style, Some(script_run.script), slot);
      let registry = self.style_font_registry_for_slot(style, Some(script_run.script), slot);
      let mut request = font_request_for_slot(style, Some(script_run.script), slot);
      apply_wordprocessingml_east_asia_font_mapping(&mut request, &registry, style, slot);
      apply_wordprocessingml_rtl_italic_face(&mut request, &registry, style);
      request
        .features
        .extend(additional_features.iter().map(|feature| FeatureValue {
          tag: Cow::Owned(feature.tag.to_string()),
          value: feature.value,
        }));
      let small_caps_scale = if base_size > f32::EPSILON {
        script_run.size_pt.0 / base_size
      } else {
        1.0
      };
      request.size_pt =
        FontSize(effective_font_size_pt(style, Some(script_run.script)) * small_caps_scale);
      request.script = Some(script_run.script);
      // w:rtl selects complex-script run properties, while ECMA-376 Part 1
      // Annex I.7 delegates visual order and mirroring to the resolved Unicode
      // bidi levels. Keep those inputs separate: an automatic odd level must
      // mirror neutral glyphs without switching to szCs/bCs/iCs/rFonts@cs.
      let direction = style
        .resolved_bidi_level()
        .map_or(script_run.direction, |level| {
          if level % 2 == 0 {
            TextDirection::LeftToRight
          } else {
            TextDirection::RightToLeft
          }
        });
      let mut options = ShapeOptions::from_request(&request, direction);
      // Completed Word lines add authored pitch on the source device grid,
      // after glyph placement. Keep it separate from the design advance so
      // the device owner rounds the font and the pitch independently.
      options.character_spacing_pt = if style.wordprocessing_kashida().is_some() {
        0.0
      } else {
        style.character_spacing_pt()
      };
      options.character_spacing_by_cluster = style.wordprocessingml_font_slots();
      options.wordprocessingml_blank_separators = style.wordprocessingml_font_slots();
      options.horizontal_scale = style.horizontal_scale();
      options.preserve_default_ignorables = style.wordprocessing_nominal_control_metrics()
        && !segment_text.is_empty()
        && segment_text.chars().all(|ch| ch == '\u{202c}');
      options.small_caps = script_run.small_caps;
      if options.small_caps && style.wordprocessingml_font_slots() {
        let authored_sizes = style.wordprocessing_layout_font_sizes();
        let nominal_size = authored_sizes.map_or(request.size_pt.0, |sizes| {
          if uses_complex_run_properties(style) {
            sizes.complex.unwrap_or(sizes.primary).0
          } else {
            sizes.primary.0
          }
        });
        // Word synthesizes small caps on its half-point font-size grid,
        // then realizes that font on the output device. Scaling an already
        // realized em changes both ink and advances (16pt -> 13pt -> 12.96pt).
        let nominal_small_caps = (nominal_size * 0.8 * 2.0).round() / 2.0;
        options.small_caps_size_pt = Some(FontSize(if authored_sizes.is_some() {
          crate::units::quantize_points_to_office_print_grid(nominal_small_caps)
        } else {
          nominal_small_caps
        }));
      }
      options.preserve_arabic_mark_components =
        style.wordprocessingml_font_slots() && script_run.script == TextScript::Arabic;
      // Uniscribe inserts a dotted circle for an independent leading mark;
      // it does not add one when the caller supplies a valid preceding base.
      // HarfRust implements this with BOT and explicit pre-context. Leave
      // other consumers' shaping-boundary policy unchanged.
      options.beginning_of_text = style.wordprocessingml_font_slots() && leading_arabic_mark;
      options.scan_registered_fallbacks = false;
      if let Some(context) = style.shaping_context()
        && (!context.leading_marks_only || leading_arabic_mark)
      {
        options.pre_context = Some(Cow::Owned(format!(
          "{}{}",
          context.before,
          &text[..script_run.text_range.start]
        )));
        options.post_context = Some(Cow::Owned(format!(
          "{}{}",
          &text[script_run.text_range.end..],
          context.after
        )));
      }
      if !self.font_selection_cache.contains_key(&key) {
        let selection = registry.resolve_font_chain(&request).ok()?;
        self.font_selection_cache.insert(key.clone(), selection);
      }
      let (mut runs, synthesis) = {
        let selection = self.font_selection_cache.get(&key)?;
        let synthesis = selection
          .resolved_fonts()
          .map(|font| {
            (
              font.font_id.clone(),
              font.synthetic_bold,
              font.synthetic_italic,
            )
          })
          .collect::<Vec<_>>();
        let runs = registry
          .shape_text_runs_with_font_chain(selection, segment_text, &options)
          .ok()?;
        (runs, synthesis)
      };
      self.font_synthesis_cache.extend(
        synthesis
          .into_iter()
          .map(|(font_id, bold, italic)| (font_id, (bold, italic))),
      );
      for run in &runs {
        let _ = self.font_face_data_from_registry(&registry, &run.font_id);
      }
      for run in &mut runs {
        if script_run.script == TextScript::Arabic
          && style.wordprocessingml_font_slots()
          && style.wordprocessing_kashida().is_none()
          && !style.small_caps()
          && style.character_spacing_pt().is_finite()
          && let Some((legacy, percent)) = style.wordprocessing_measurement_profile()
          && (style.horizontal_scale() - f32::from(percent) / 100.0).abs() < 0.00001
          && let Some(face) = self.font_face_data(&run.font_id)
          && let Some(measurement) =
            common::wordprocessing_device::ArabicMeasurement::new(&face, legacy, percent)
        {
          measurement.apply(run, style.horizontal_scale(), style.character_spacing_pt());
        }
        if style.wordprocessingml_balance_single_byte_double_byte_width() {
          // ECMA-376 Part 1 §17.15.3.3 balances half-width and full-width
          // spaces at 1:2. Writer's BalanceCjkSpaces applies the adjustment
          // to raw advances before every other kind of justification.
          apply_wordprocessingml_single_double_byte_width_balance(
            run,
            style.horizontal_scale(),
            style.character_spacing_pt(),
          );
        }
        run.offset_text_range(script_run.text_range.start);
        apply_wordprocessingml_punctuation_compression(
          run,
          text,
          style.cjk_punctuation_compression_ratio(),
          style.wordprocessingml_punctuation_spacing(),
          style.wordprocessingml_legacy_punctuation_spacing(),
        );
      }
      output.extend(runs);
    }
    let expansion = style.wordprocessing_justification_expansion_pt();
    if !expansion.is_empty() && expansion.len() == text.chars().count() {
      let starts: Vec<_> = text.char_indices().map(|(index, _)| index).collect();
      for run in &mut output {
        let mut added = 0.0;
        for glyph in run.glyphs.to_mut() {
          let start = starts.partition_point(|&byte| byte < glyph.text_range.start);
          let end = starts.partition_point(|&byte| byte < glyph.text_range.end);
          let extra: f32 = expansion[start..end].iter().sum();
          glyph.x_advance_pt += extra;
          added += extra;
        }
        run.advance_pt += added;
      }
    }
    if style
      .resolved_bidi_level()
      .is_some_and(|level| level % 2 == 1)
    {
      // This function receives one directionally uniform bidi portion. UAX #9
      // rule L2 therefore reverses its complete font/script-run sequence at an
      // odd level. HarfBuzz has already put the glyphs inside each run in RTL
      // order, so only the sequence of independently shaped runs belongs here.
      output.reverse();
    }
    Some(output)
  }

  fn style_font_registry(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
  ) -> Arc<FontRegistry<'static>> {
    self.style_font_registry_for_slot(style, script, None)
  }

  fn style_font_registry_for_slot(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
    wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  ) -> Arc<FontRegistry<'static>> {
    if let Some((key, registry)) = &self.last_font_registry
      && key.matches_style_for_slot(style, script, wordprocessingml_font_slot)
    {
      return registry.clone();
    }
    let key = FontFaceKey::from_style_for_slot(style, script, wordprocessingml_font_slot);
    if let Some(registry) = self.font_registry_cache.get(&key) {
      let registry = registry.clone();
      self.last_font_registry = Some((key, registry.clone()));
      return registry;
    }
    let registry = Arc::new(build_style_font_registry_for_slot(
      style,
      script,
      wordprocessingml_font_slot,
    ));
    self
      .font_registry_cache
      .insert(key.clone(), registry.clone());
    self.last_font_registry = Some((key, registry.clone()));
    registry
  }

  fn font_face_data_from_registry(
    &mut self,
    registry: &FontRegistry<'static>,
    font_id: &FontId,
  ) -> Option<FontFaceData> {
    if self.font_data_cache.contains_key(font_id) {
      return self.font_face_data(font_id);
    }
    let face = font_face_data_from_registry_binary(font_id, registry)?;
    self.font_data_cache.insert(font_id.clone(), face.clone());
    self.font_face_data(font_id)
  }
}

pub fn shape_text_runs<'text>(
  text: &'text str,
  style: &(impl FontStyleRef + ?Sized),
) -> Option<Vec<ShapedRun<'text, 'static>>> {
  FontResolver::default().shape_text_runs(text, style)
}

pub fn vertical_metrics(
  style: &(impl FontStyleRef + ?Sized),
) -> Option<ooxmlsdk_fonts::VerticalMetrics> {
  FontResolver::default().vertical_metrics(style)
}

pub fn decoration_metrics(
  style: &(impl FontStyleRef + ?Sized),
) -> Option<ooxmlsdk_fonts::DecorationMetrics> {
  FontResolver::default().decoration_metrics(style)
}

fn font_request<'a>(
  style: &'a (impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
) -> FontRequest<'a> {
  font_request_for_slot(style, script, None)
}

fn font_request_for_slot<'a>(
  style: &'a (impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> FontRequest<'a> {
  let mut features = vec![FeatureValue {
    tag: Cow::Borrowed("kern"),
    // Arabic kern is a required positioning feature in Microsoft's Arabic
    // shaping model. Native Word controls preserve its placement arrays when
    // w:kern exceeds the run size; that threshold still owns Western kerning.
    value: u32::from(
      style.kerning_enabled()
        || (style.wordprocessingml_font_slots() && script == Some(TextScript::Arabic)),
    ),
  }];
  if let Some(ligatures) = style.ligatures() {
    // [MS-DOCX] 2.3.32 maps the four Word ligature categories to the
    // corresponding OpenType feature tags defined by ISO/IEC 14496-22.
    features.extend([
      FeatureValue {
        tag: Cow::Borrowed("liga"),
        // Word's optional typography switch does not disable the Arabic
        // shaping engine's standard ligatures. Native controls retain these
        // with w14:ligatures absent or explicitly "none", including Arial's
        // Allah ligature stored in GSUB liga rather than rlig. Keep this
        // script-specific policy out of Latin and non-Word font requests.
        value: u32::from(
          ligatures.standard
            || (style.wordprocessingml_font_slots() && script == Some(TextScript::Arabic)),
        ),
      },
      FeatureValue {
        tag: Cow::Borrowed("clig"),
        value: u32::from(ligatures.contextual),
      },
      FeatureValue {
        tag: Cow::Borrowed("hlig"),
        value: u32::from(ligatures.historical),
      },
      FeatureValue {
        tag: Cow::Borrowed("dlig"),
        value: u32::from(ligatures.discretionary),
      },
    ]);
  }
  let open_type_features = style.open_type_features();
  if let Some(vertical_feature) = open_type_features.vertical_feature {
    features.push(FeatureValue {
      tag: Cow::Borrowed(match vertical_feature {
        common::OpenTypeVerticalFeature::VerticalAlternates => "vert",
        common::OpenTypeVerticalFeature::VerticalAlternatesAndRotation => "vrt2",
      }),
      value: 1,
    });
  }
  if let Some(number_form) = open_type_features.number_form {
    let tag = match number_form {
      common::OpenTypeNumberForm::Default => None,
      common::OpenTypeNumberForm::Lining => Some("lnum"),
      common::OpenTypeNumberForm::OldStyle => Some("onum"),
    };
    if let Some(tag) = tag {
      features.push(FeatureValue {
        tag: Cow::Borrowed(tag),
        value: 1,
      });
    }
  }
  if let Some(number_spacing) = open_type_features.number_spacing {
    let tag = match number_spacing {
      common::OpenTypeNumberSpacing::Default => None,
      common::OpenTypeNumberSpacing::Proportional => Some("pnum"),
      common::OpenTypeNumberSpacing::Tabular => Some("tnum"),
    };
    if let Some(tag) = tag {
      features.push(FeatureValue {
        tag: Cow::Borrowed(tag),
        value: 1,
      });
    }
  }
  if let Some(stylistic_sets) = open_type_features.stylistic_sets {
    const TAGS: [&str; 20] = [
      "ss01", "ss02", "ss03", "ss04", "ss05", "ss06", "ss07", "ss08", "ss09", "ss10", "ss11",
      "ss12", "ss13", "ss14", "ss15", "ss16", "ss17", "ss18", "ss19", "ss20",
    ];
    features.extend(stylistic_sets.enabled_ids().map(|id| FeatureValue {
      tag: Cow::Borrowed(TAGS[usize::from(id - 1)]),
      value: 1,
    }));
  }
  if let Some(enabled) = open_type_features.contextual_alternates {
    features.push(FeatureValue {
      tag: Cow::Borrowed("calt"),
      value: u32::from(
        enabled || (style.wordprocessingml_font_slots() && script == Some(TextScript::Arabic)),
      ),
    });
  } else if style.wordprocessingml_font_slots() {
    // Microsoft's Arabic shaping model always applies calt for connection
    // forms, including older fonts without rclt. Native Word retains these
    // forms with cntxtAlts absent or false. Its optional typography switch
    // still owns Western alternates; preserve their explicit default zero.
    features.push(FeatureValue {
      tag: Cow::Borrowed("calt"),
      value: u32::from(script == Some(TextScript::Arabic)),
    });
  }
  FontRequest {
    family: script_font_family_for_slot(style, script, wordprocessingml_font_slot)
      .filter(|family| !family.trim().is_empty())
      .map(Cow::Borrowed),
    family_selection: if style.wordprocessingml_font_slots() {
      FontFamilySelection::First
    } else {
      FontFamilySelection::List
    },
    bold: effective_bold(style, script),
    italic: effective_italic(style, script),
    size_pt: FontSize(effective_font_size_pt(style, script)),
    script,
    family_class: script_font_family_class_for_slot(style, script, wordprocessingml_font_slot),
    charset: font_charset_for_slot(style, script, wordprocessingml_font_slot),
    pitch: font_pitch_for_slot(style, script, wordprocessingml_font_slot),
    features,
    ..FontRequest::default()
  }
}

fn script_font_family_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<&str> {
  // Word fixed output keeps ASCII digits and their numeric separators on
  // the ASCII rFonts family even when w:cs/w:rtl selects complex-script
  // formatting. The scanner emits Ascii under that exception; precede the run
  // override without changing szCs/bCs/iCs selection.
  if wordprocessingml_font_slot == Some(WordprocessingFontSlot::Ascii) {
    return style.font_family();
  }
  if let Some(force_complex) = style.complex_script_override() {
    return if force_complex {
      style.complex_font_family()
    } else {
      style.font_family()
    };
  }
  let east_asia_family = || {
    if matches!(script, Some(TextScript::Hiragana | TextScript::Katakana)) {
      style
        .drawingml_japanese_font_family()
        .or_else(|| style.east_asia_font_family())
    } else {
      style.east_asia_font_family()
    }
  };
  if let Some(slot) = wordprocessingml_font_slot {
    return match slot {
      WordprocessingFontSlot::Ascii => style.font_family(),
      WordprocessingFontSlot::HighAnsi => style.high_ansi_font_family(),
      WordprocessingFontSlot::EastAsia => east_asia_family(),
      WordprocessingFontSlot::ComplexScript => style.complex_font_family(),
    };
  }
  match script {
    Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul) => {
      east_asia_family()
    }
    Some(TextScript::Arabic | TextScript::Hebrew | TextScript::Devanagari | TextScript::Thai) => {
      style.complex_font_family()
    }
    _ => style.font_family(),
  }
}

fn symbol_charset_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<FontCharset> {
  // ECMA-376 Part 1 §17.3.3.30 makes w:sym independent of the run's rFonts
  // slots.  The DOCX importer represents that boundary by disabling slot
  // selection on the isolated symbol run.  DrawingML similarly materializes
  // only the symbol segment with its symbol face.  Require both that boundary
  // and the actually selected family match, so an alternate symbol face
  // retained on an ordinary DrawingML text segment cannot affect that text.
  if style.wordprocessingml_font_slots() {
    return None;
  }
  let selected = script_font_family_for_slot(style, script, wordprocessingml_font_slot)?;
  let symbol = style.symbol_font_family()?.trim();
  (!symbol.is_empty() && selected.trim().eq_ignore_ascii_case(symbol))
    .then_some(FontCharset::Symbol)
}

fn font_charset_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<FontCharset> {
  if let Some(symbol) = symbol_charset_for_slot(style, script, wordprocessingml_font_slot) {
    return Some(symbol);
  }
  if wordprocessingml_font_slot == Some(WordprocessingFontSlot::Ascii) {
    return style.font_charset();
  }
  if let Some(force_complex) = style.complex_script_override() {
    return if force_complex {
      style.complex_font_charset()
    } else {
      style.font_charset()
    };
  }
  if let Some(slot) = wordprocessingml_font_slot {
    return match slot {
      WordprocessingFontSlot::Ascii => style.font_charset(),
      WordprocessingFontSlot::HighAnsi => style.high_ansi_font_charset(),
      WordprocessingFontSlot::EastAsia => style.wordprocessingml_east_asia_font_charset(),
      WordprocessingFontSlot::ComplexScript => style.complex_font_charset(),
    };
  }
  match script {
    Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul) => {
      style.wordprocessingml_east_asia_font_charset()
    }
    Some(TextScript::Arabic | TextScript::Hebrew | TextScript::Devanagari | TextScript::Thai) => {
      style.complex_font_charset()
    }
    _ => style.font_charset(),
  }
}

fn font_pitch_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<ooxmlsdk_fonts::FontPitch> {
  if wordprocessingml_font_slot == Some(WordprocessingFontSlot::Ascii) {
    return style.font_pitch();
  }
  if let Some(force_complex) = style.complex_script_override() {
    return if force_complex {
      style.complex_font_pitch()
    } else {
      style.font_pitch()
    };
  }
  if let Some(slot) = wordprocessingml_font_slot {
    return match slot {
      WordprocessingFontSlot::Ascii => style.font_pitch(),
      WordprocessingFontSlot::HighAnsi => style.high_ansi_font_pitch(),
      WordprocessingFontSlot::EastAsia => style.east_asia_font_pitch(),
      WordprocessingFontSlot::ComplexScript => style.complex_font_pitch(),
    };
  }
  match script {
    Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul) => {
      style.east_asia_font_pitch()
    }
    Some(TextScript::Arabic | TextScript::Hebrew | TextScript::Devanagari | TextScript::Thai) => {
      style.complex_font_pitch()
    }
    _ => style.font_pitch(),
  }
}

fn script_fallback_font_family_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<&str> {
  if symbol_charset_for_slot(style, script, wordprocessingml_font_slot).is_some() {
    // w:font names the symbol face independently from Unicode script, and
    // the font table's w:altName is its only authored substitute. PUA text is
    // commonly classified as Other, so the ordinary script fallback table
    // must not discard that explicit alternate.
    return style.fallback_font_family();
  }
  if let Some(slot) = wordprocessingml_font_slot {
    return match slot {
      WordprocessingFontSlot::Ascii => style.fallback_font_family(),
      WordprocessingFontSlot::HighAnsi => style.high_ansi_fallback_font_family(),
      WordprocessingFontSlot::EastAsia => style.east_asia_fallback_font_family(),
      WordprocessingFontSlot::ComplexScript => style.complex_fallback_font_family(),
    };
  }
  match script {
    None
    | Some(TextScript::Common | TextScript::Latin | TextScript::Cyrillic | TextScript::Greek) => {
      style.fallback_font_family()
    }
    Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul) => {
      style.east_asia_fallback_font_family()
    }
    Some(TextScript::Arabic | TextScript::Hebrew | TextScript::Devanagari | TextScript::Thai) => {
      style.complex_fallback_font_family()
    }
    Some(TextScript::Other) => None,
  }
}

fn script_font_family_class_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<FontFamilyClass> {
  if symbol_charset_for_slot(style, script, wordprocessingml_font_slot).is_some() {
    // The w:sym PUA transport deliberately suppresses generic font matching;
    // only the declared face and its document-authored alternate are valid.
    return None;
  }
  if wordprocessingml_font_slot == Some(WordprocessingFontSlot::Ascii) {
    return style.font_family_class();
  }
  if let Some(force_complex) = style.complex_script_override() {
    // The override selects the complex-script face and its font-table
    // metadata together, just as it does for charset and pitch. Dropping
    // family here bypasses the document's substitution classification.
    return if force_complex {
      style.complex_font_family_class()
    } else {
      style.font_family_class()
    };
  }
  if let Some(slot) = wordprocessingml_font_slot {
    return match slot {
      WordprocessingFontSlot::Ascii => style.font_family_class(),
      WordprocessingFontSlot::HighAnsi => style.high_ansi_font_family_class(),
      WordprocessingFontSlot::EastAsia => style.east_asia_font_family_class(),
      WordprocessingFontSlot::ComplexScript => style.complex_font_family_class(),
    };
  }
  match script {
    None
    | Some(TextScript::Common | TextScript::Latin | TextScript::Cyrillic | TextScript::Greek) => {
      style.font_family_class()
    }
    _ => None,
  }
}

fn wordprocessingml_missing_family_fallback_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<&'static str> {
  if !style.wordprocessingml_font_slots()
    || symbol_charset_for_slot(style, script, wordprocessingml_font_slot).is_some()
  {
    return None;
  }

  let latin_slot = match wordprocessingml_font_slot {
    Some(WordprocessingFontSlot::Ascii | WordprocessingFontSlot::HighAnsi) => true,
    Some(WordprocessingFontSlot::EastAsia | WordprocessingFontSlot::ComplexScript) => false,
    None => matches!(
      script,
      None
        | Some(TextScript::Common | TextScript::Latin | TextScript::Cyrillic | TextScript::Greek)
    ),
  };

  // Word fixed output on Windows substitutes Cambria for an explicitly
  // named, unavailable Latin face when the package supplies neither an
  // alternate name nor font-family metadata. This is distinct from the
  // application-defined rFonts value used when no face was authored at all.
  // Keep document-authored alternates and pitch/family substitutions ahead of
  // this final Word font-mapper choice, and do not leak it into DrawingML or
  // East Asian/complex-script slots.
  latin_slot.then_some("Cambria")
}

fn wordprocessingml_east_asia_font_mapping(
  style: &(impl FontStyleRef + ?Sized),
  slot: Option<WordprocessingFontSlot>,
) -> bool {
  style.wordprocessingml_font_slots()
    && style.wordprocessingml_font_hint()
      == Some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia)
    && slot == Some(WordprocessingFontSlot::EastAsia)
}

pub(crate) fn wordprocessingml_join_control(ch: char) -> bool {
  // Unicode Core Specification, chapter 23: these format characters affect
  // adjacent joining/ligatures rather than supplying a visible glyph.
  matches!(ch, '\u{200c}' | '\u{200d}')
}

fn apply_wordprocessingml_east_asia_font_mapping(
  request: &mut FontRequest<'_>,
  registry: &FontRegistry<'static>,
  style: &(impl FontStyleRef + ?Sized),
  slot: Option<WordprocessingFontSlot>,
) {
  if !wordprocessingml_east_asia_font_mapping(style, slot) {
    return;
  }
  let Ok(primary) = registry.resolve(request) else {
    return;
  };
  if primary.metrics.vertical.wordprocessingml_cjk_line_metrics {
    return;
  }
  // MS-OI29500 17.3.2.26 assigns hinted symbols/Greek/Cyrillic to the EA
  // slot independently of Unicode script. Isolated Word font/repertoire
  // controls map Latin-only EA faces to SimSun, even when the authored face
  // covers the symbol. A physical CJK face (for example Meiryo) stays selected;
  // missing SimSun glyphs then follow the ordinary glyph-link policy.
  let mut east_asia = request.clone();
  east_asia.family = Some(Cow::Borrowed("SimSun"));
  if registry.resolve(&east_asia).is_ok_and(|resolved| {
    resolved.resolved_family.eq_ignore_ascii_case("SimSun")
      && resolved.metrics.vertical.wordprocessingml_cjk_line_metrics
  }) {
    // Change the primary request only. A family-wide alias would also redirect
    // a later Segoe UI Symbol glyph link back to SimSun and lose its coverage.
    request.family = east_asia.family;
  }
}

fn build_style_font_registry_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> FontRegistry<'static> {
  font_timing("build style font registry", || {
    let mut request = font_request_for_slot(style, script, wordprocessingml_font_slot);
    request.script = script;
    let mut registry = FontRegistry::with_default_policy();
    if style.wordprocessing_nominal_control_metrics() {
      // Read-only Word font creation and placement agree on MS Mincho
      // (256 units/em, U+202C glyph854/advance128) when the authored face
      // lacks this nominal control. Keep covered primary glyphs, and scope
      // the link to measured controls rather than all Common symbols.
      registry.book.fallback_chains.insert(
        0,
        FontFallbackChain {
          unicode_ranges: std::iter::once(0x202c..0x202d).collect(),
          families: vec![Cow::Borrowed("MS Mincho")],
          ..Default::default()
        },
      );
    }
    if let Some(requested_family) = request.family.as_deref() {
      let mut families: Vec<Cow<'static, str>> = Vec::new();
      if let Some(fallback_family) =
        script_fallback_font_family_for_slot(style, script, wordprocessingml_font_slot)
        && !requested_family.eq_ignore_ascii_case(fallback_family)
      {
        families.push(Cow::Owned(fallback_family.to_string()));
        // Arial Unicode MS is a distinct legacy face, but the default policy
        // aliases it to Arial for documents without mapper metadata. When a
        // format supplies a verified missing-family result, keep the exact
        // face eligible and let that document-scoped result precede the
        // compatibility alias.
        if requested_family.eq_ignore_ascii_case("Arial Unicode MS") {
          registry
            .book
            .family_aliases
            .retain(|alias| !alias.from.as_ref().eq_ignore_ascii_case(requested_family));
        }
      }
      if requested_family.eq_ignore_ascii_case("Eurostile")
        && wordprocessingml_missing_family_fallback_for_slot(
          style,
          script,
          wordprocessingml_font_slot,
        )
        .is_some()
      {
        // Word's named missing-face mapping precedes a generic family class.
        // Native body/WPS/WPG controls recover Eurostile as Agency FB; saving
        // the document materializes that altName. Authored alternate names
        // above still win, and an installed Eurostile remains the primary.
        families.push(Cow::Borrowed("Agency FB"));
      }
      let family_class_fallback = match request.family_class {
        Some(FontFamilyClass::Serif | FontFamilyClass::OldStyle | FontFamilyClass::Schoolbook) => {
          Some("Times New Roman")
        }
        Some(FontFamilyClass::SansSerif) => Some("Arial"),
        Some(FontFamilyClass::Fixed) => Some("Courier New"),
        _ => None,
      };
      if let Some(family) = family_class_fallback
        && !requested_family.eq_ignore_ascii_case(family)
        && !families
          .iter()
          .any(|existing| existing.eq_ignore_ascii_case(family))
      {
        families.push(Cow::Borrowed(family));
      }
      // ECMA-376 Part 1 §21.1.2.5 requires DrawingML font substitution
      // when the requested typeface is unavailable. Keep these document-scoped
      // alternate names in the missing-family phase; glyph fallback remains a
      // separate coverage decision after the primary face has been selected.
      if !families.is_empty() {
        registry.book.family_substitution_chains.insert(
          0,
          FontFallbackChain {
            unicode_ranges: Vec::new(),
            requested_family: Some(Cow::Owned(requested_family.to_string())),
            script,
            language: None,
            families,
          },
        );
      }
      if let Some(family) =
        wordprocessingml_missing_family_fallback_for_slot(style, script, wordprocessingml_font_slot)
        && !requested_family.eq_ignore_ascii_case(family)
      {
        // Default policy begins with known metric-compatible substitutions
        // and ends with a cross-platform generic family. Insert Word's
        // unknown-family result between them: known names such as Calibri
        // still reach Carlito, while truly unknown Word fonts reach Cambria
        // instead of the host's arbitrary sans-serif default.
        let insertion = registry
          .book
          .family_substitution_chains
          .iter()
          .position(|chain| chain.requested_family.is_none())
          .unwrap_or(registry.book.family_substitution_chains.len());
        registry.book.family_substitution_chains.insert(
          insertion,
          FontFallbackChain {
            unicode_ranges: Vec::new(),
            requested_family: Some(Cow::Owned(requested_family.to_string())),
            script,
            language: None,
            families: vec![Cow::Borrowed(family)],
          },
        );
      }
    }
    let registered = registry
      .register_system_query_fonts(&request)
      .unwrap_or_default();
    if registered == 0 {
      let mut fallback_request = font_request_for_slot(style, script, wordprocessingml_font_slot);
      fallback_request.script = script;
      fallback_request.family = None;
      registry
        .register_system_query_fonts(&fallback_request)
        .unwrap_or_default();
    }
    if wordprocessingml_east_asia_font_mapping(style, wordprocessingml_font_slot)
      && registry
        .resolve(&request)
        .is_ok_and(|primary| !primary.metrics.vertical.wordprocessingml_cjk_line_metrics)
    {
      let mut east_asia = request.clone();
      east_asia.family = Some(Cow::Borrowed("SimSun"));
      registry
        .register_system_query_fonts(&east_asia)
        .unwrap_or_default();
    }
    if wordprocessingml_synthetic_rtl_italic(style)
      && let Ok(primary) = registry.resolve(&request)
      && registry
        .face(&primary.font_id)
        .is_some_and(|face| face.slant != FontSlant::Upright)
      && registered_font_has_arabic_charset(&registry, &primary.font_id) == Some(false)
    {
      let mut upright = request.clone();
      upright.family = Some(primary.resolved_family);
      upright.slant = Some(FontSlant::Upright);
      registry
        .register_system_query_fonts(&upright)
        .unwrap_or_default();
    }
    registry
  })
}

fn font_face_data_from_registry_binary(
  font_id: &FontId,
  registry: &FontRegistry<'static>,
) -> Option<FontFaceData> {
  let (data, index) = registry.font_face_binary(font_id)?;
  Some(FontFaceData {
    data: Arc::new(data),
    index,
    synthetic_bold: false,
    synthetic_italic: false,
    id: font_id.0.clone(),
  })
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontFaceKey {
  family: Option<String>,
  wordprocessingml_font_slots: bool,
  fallback_family: Option<String>,
  family_class: Option<FontFamilyClass>,
  charset: Option<FontCharset>,
  pitch: Option<ooxmlsdk_fonts::FontPitch>,
  bold: bool,
  italic: bool,
  synthetic_rtl_italic: bool,
  script: Option<TextScript>,
  wordprocessingml_missing_family_fallback: Option<&'static str>,
  wordprocessing_nominal_control_metrics: bool,
  wordprocessingml_east_asia_font_mapping: bool,
}

impl FontFaceKey {
  fn from_style(style: &(impl FontStyleRef + ?Sized), script: Option<TextScript>) -> Self {
    Self::from_style_for_slot(style, script, None)
  }

  fn from_style_for_slot(
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
    wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  ) -> Self {
    Self {
      family: script_font_family_for_slot(style, script, wordprocessingml_font_slot)
        .map(str::to_string),
      wordprocessingml_font_slots: style.wordprocessingml_font_slots(),
      fallback_family: script_fallback_font_family_for_slot(
        style,
        script,
        wordprocessingml_font_slot,
      )
      .map(str::to_string),
      family_class: script_font_family_class_for_slot(style, script, wordprocessingml_font_slot),
      charset: font_charset_for_slot(style, script, wordprocessingml_font_slot),
      pitch: font_pitch_for_slot(style, script, wordprocessingml_font_slot),
      bold: effective_bold(style, script),
      italic: effective_italic(style, script),
      synthetic_rtl_italic: wordprocessingml_synthetic_rtl_italic(style),
      wordprocessing_nominal_control_metrics: style.wordprocessing_nominal_control_metrics(),
      wordprocessingml_east_asia_font_mapping: wordprocessingml_east_asia_font_mapping(
        style,
        wordprocessingml_font_slot,
      ),
      script,
      wordprocessingml_missing_family_fallback: wordprocessingml_missing_family_fallback_for_slot(
        style,
        script,
        wordprocessingml_font_slot,
      ),
    }
  }

  fn matches_style(
    &self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
  ) -> bool {
    self.matches_style_for_slot(style, script, None)
  }

  fn matches_style_for_slot(
    &self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
    wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  ) -> bool {
    self.family.as_deref() == script_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.wordprocessingml_font_slots == style.wordprocessingml_font_slots()
      && self.fallback_family.as_deref()
        == script_fallback_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.family_class
        == script_font_family_class_for_slot(style, script, wordprocessingml_font_slot)
      && self.charset == font_charset_for_slot(style, script, wordprocessingml_font_slot)
      && self.pitch == font_pitch_for_slot(style, script, wordprocessingml_font_slot)
      && self.bold == effective_bold(style, script)
      && self.italic == effective_italic(style, script)
      && self.synthetic_rtl_italic == wordprocessingml_synthetic_rtl_italic(style)
      && self.wordprocessing_nominal_control_metrics
        == style.wordprocessing_nominal_control_metrics()
      && self.wordprocessingml_east_asia_font_mapping
        == wordprocessingml_east_asia_font_mapping(style, wordprocessingml_font_slot)
      && self.script == script
      && self.wordprocessingml_missing_family_fallback
        == wordprocessingml_missing_family_fallback_for_slot(
          style,
          script,
          wordprocessingml_font_slot,
        )
  }
}

#[derive(Clone, Copy, Debug)]
struct FontMetrics {
  vertical: ooxmlsdk_fonts::VerticalMetrics,
  decoration: ooxmlsdk_fonts::DecorationMetrics,
  wordprocessingml_default_charset: Option<ooxmlsdk_fonts::VerticalMetrics>,
}

fn wordprocessingml_default_charset_metrics(
  request: &FontRequest<'_>,
  resolved: &ooxmlsdk_fonts::ResolvedFont<'_>,
) -> Option<ooxmlsdk_fonts::VerticalMetrics> {
  if request.charset != Some(FontCharset::Other(1))
    || !request
      .family
      .as_deref()
      .is_some_and(|family| family.eq_ignore_ascii_case(&resolved.resolved_family))
  {
    return None;
  }
  // Word's logical font record is distinct from GDI's realized TEXTMETRIC.
  // [MS-OI29500] 17.8(a) permits replacement of document font metadata by
  // records already loaded in the session. Isolated Office 16 exports and
  // saved font tables normalize DEFAULT_CHARSET to ANSI for these families;
  // Calibri and Courier New are counterexamples and must not join this set.
  // This is a charset policy, not a table of per-font line-height constants.
  if [
    "Arial",
    "Arial Black",
    "Times New Roman",
    "Cambria",
    "Constantia",
    "Century",
    "Century Gothic",
    "Georgia",
    "Verdana",
  ]
  .iter()
  .any(|family| resolved.resolved_family.eq_ignore_ascii_case(family))
  {
    return None;
  }
  let data = resolved.source.data()?;
  let Ok(face) = FontRef::from_index(data, resolved.face_index) else {
    return None;
  };
  let Ok(os2) = face.os2() else {
    return None;
  };
  // East Asian and symbol faces realize their own concrete charset rather
  // than retaining the Latin DEFAULT_CHARSET record (for example PMingLiU
  // realizes BIG5_CHARSET=136). Their existing compatibility path remains
  // responsible for line metrics.
  if os2
    .ul_code_page_range_1()
    .is_some_and(|bits| bits & 0x801e_0000 != 0)
  {
    return None;
  }
  let fixed = face.post().is_ok_and(|post| post.is_fixed_pitch() != 0);
  let pitch = if fixed {
    ooxmlsdk_fonts::FontPitch::Fixed
  } else {
    ooxmlsdk_fonts::FontPitch::Variable
  };
  let panose = os2.panose_10();
  let family = if fixed {
    Some(FontFamilyClass::Fixed)
  } else if panose[0] == 2 {
    match panose[1] {
      2..=10 => Some(FontFamilyClass::Serif),
      11..=15 => Some(FontFamilyClass::SansSerif),
      _ => None,
    }
  } else {
    None
  };
  // Auto/absent or mismatched family and pitch cause Word to realize a new
  // font record, replacing the authored default charset with its ANSI value.
  if family.is_none() || request.family_class != family || request.pitch != Some(pitch) {
    return None;
  }
  // The retained legacy charset uses the GDI/Windows font box even when
  // the face opts into USE_TYPO_METRICS. Calibri's native charset01 control
  // distinguishes this from simply scaling the typographic line box.
  let scale = request.size_pt.0 / f32::from(face.head().ok()?.units_per_em().max(1));
  let ascent = f32::from(os2.us_win_ascent()) * scale;
  let descent = f32::from(os2.us_win_descent()) * scale;
  let hhea = face.hhea().ok()?;
  let height = f32::from(hhea.ascender().to_i16()) - f32::from(hhea.descender().to_i16())
    + f32::from(hhea.line_gap().to_i16());
  let gap = (height * scale - ascent - descent).max(0.0);
  Some(ooxmlsdk_fonts::VerticalMetrics {
    ascent_pt: ascent,
    descent_pt: descent,
    windows_line_height_pt: ascent + descent,
    baseline_offset_pt: ascent,
    directwrite_baseline_offset_pt: ascent + gap,
    line_gap_pt: gap,
    ink_height_pt: ascent + descent,
    ..resolved.metrics_at_size(request.size_pt).vertical
  })
}

fn include_vertical_metrics(
  combined: &mut Option<ooxmlsdk_fonts::VerticalMetrics>,
  metrics: ooxmlsdk_fonts::VerticalMetrics,
) {
  if let Some(combined) = combined {
    combined.ascent_pt = combined.ascent_pt.max(metrics.ascent_pt);
    combined.descent_pt = combined.descent_pt.max(metrics.descent_pt);
    combined.internal_leading_pt = combined
      .internal_leading_pt
      .max(metrics.internal_leading_pt);
    combined.external_leading_pt = combined
      .external_leading_pt
      .max(metrics.external_leading_pt);
    combined.line_gap_pt = combined.line_gap_pt.max(metrics.line_gap_pt);
    combined.ink_height_pt = combined.ink_height_pt.max(metrics.ink_height_pt);
    combined.baseline_offset_pt = combined.baseline_offset_pt.max(metrics.baseline_offset_pt);
    combined.directwrite_baseline_offset_pt = combined
      .directwrite_baseline_offset_pt
      .max(metrics.directwrite_baseline_offset_pt);
    combined.hanging_baseline_pt = combined
      .hanging_baseline_pt
      .max(metrics.hanging_baseline_pt);
    combined.cjk_horizontal_advance_pt = combined
      .cjk_horizontal_advance_pt
      .max(metrics.cjk_horizontal_advance_pt);
    combined.cjk_vertical_advance_pt = combined
      .cjk_vertical_advance_pt
      .max(metrics.cjk_vertical_advance_pt);
    combined.wordprocessingml_cjk_line_metrics |= metrics.wordprocessingml_cjk_line_metrics;
  } else {
    *combined = Some(metrics);
  }
}

fn word_font_side_leading(
  mut metrics: ooxmlsdk_fonts::VerticalMetrics,
) -> ooxmlsdk_fonts::VerticalMetrics {
  let side = ((metrics.ascent_pt + metrics.descent_pt)
    * crate::text_metrics::WORDPROCESSINGML_CJK_SIDE_LEADING_RATIO
    - metrics.line_gap_pt / 2.0)
    .max(0.0);
  metrics.ascent_pt += side;
  metrics.descent_pt += side;
  metrics.windows_line_height_pt += side * 2.0;
  metrics.baseline_offset_pt += side;
  metrics.directwrite_baseline_offset_pt += side;
  metrics
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontMetricsKey {
  family: Option<String>,
  wordprocessingml_font_slots: bool,
  fallback_family: Option<String>,
  family_class: Option<FontFamilyClass>,
  charset: Option<FontCharset>,
  pitch: Option<ooxmlsdk_fonts::FontPitch>,
  bold: bool,
  italic: bool,
  synthetic_rtl_italic: bool,
  script: Option<TextScript>,
  size_pt_bits: u32,
  wordprocessingml_missing_family_fallback: Option<&'static str>,
  wordprocessingml_east_asia_font_mapping: bool,
}

impl FontMetricsKey {
  fn from_style_for_slot(
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
    wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  ) -> Self {
    Self {
      family: script_font_family_for_slot(style, script, wordprocessingml_font_slot)
        .map(str::to_string),
      wordprocessingml_font_slots: style.wordprocessingml_font_slots(),
      fallback_family: script_fallback_font_family_for_slot(
        style,
        script,
        wordprocessingml_font_slot,
      )
      .map(str::to_string),
      family_class: script_font_family_class_for_slot(style, script, wordprocessingml_font_slot),
      charset: font_charset_for_slot(style, script, wordprocessingml_font_slot),
      pitch: font_pitch_for_slot(style, script, wordprocessingml_font_slot),
      bold: effective_bold(style, script),
      italic: effective_italic(style, script),
      script,
      synthetic_rtl_italic: wordprocessingml_synthetic_rtl_italic(style),
      size_pt_bits: effective_font_size_pt(style, script).to_bits(),
      wordprocessingml_east_asia_font_mapping: wordprocessingml_east_asia_font_mapping(
        style,
        wordprocessingml_font_slot,
      ),
      wordprocessingml_missing_family_fallback: wordprocessingml_missing_family_fallback_for_slot(
        style,
        script,
        wordprocessingml_font_slot,
      ),
    }
  }

  fn matches_style_for_slot(
    &self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
    wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  ) -> bool {
    self.family.as_deref() == script_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.wordprocessingml_font_slots == style.wordprocessingml_font_slots()
      && self.fallback_family.as_deref()
        == script_fallback_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.family_class
        == script_font_family_class_for_slot(style, script, wordprocessingml_font_slot)
      && self.charset == font_charset_for_slot(style, script, wordprocessingml_font_slot)
      && self.pitch == font_pitch_for_slot(style, script, wordprocessingml_font_slot)
      && self.bold == effective_bold(style, script)
      && self.italic == effective_italic(style, script)
      && self.script == script
      && self.synthetic_rtl_italic == wordprocessingml_synthetic_rtl_italic(style)
      && self.size_pt_bits == effective_font_size_pt(style, script).to_bits()
      && self.wordprocessingml_east_asia_font_mapping
        == wordprocessingml_east_asia_font_mapping(style, wordprocessingml_font_slot)
      && self.wordprocessingml_missing_family_fallback
        == wordprocessingml_missing_family_fallback_for_slot(
          style,
          script,
          wordprocessingml_font_slot,
        )
  }
}

pub fn cached_text_face(style: &(impl FontStyleRef + ?Sized)) -> Option<FontFaceData> {
  FontResolver::default().cached_text_face(style)
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;
  use std::sync::Arc;

  use crate::common::{
    OpenTypeFeatureSettings, OpenTypeLigatures, OpenTypeNumberForm, OpenTypeNumberSpacing,
    OpenTypeStylisticSets,
  };
  use crate::docx::TextStyle;
  use ooxmlsdk_fonts::{
    FontCharset, FontId, FontSize, ScriptScanOptions, ShapedGlyph, ShapedRun, ShapingDiagnostics,
    TextDirection, TextScript, WordprocessingFontSlot, script_direction_runs_with_options,
  };

  use super::{
    apply_wordprocessingml_single_double_byte_width_balance, effective_font_size_pt, font_request,
    font_request_for_slot, load_text_face, materialize_wordprocessingml_source_font_slot,
    script_fallback_font_family_for_slot, script_font_family_for_slot, script_scan_options,
    shape_text_runs, wordprocessing_line_metrics_font_slot,
    wordprocessingml_missing_family_fallback_for_slot,
  };

  #[test]
  fn word_primary_family_selection_has_distinct_face_and_metrics_caches() {
    let common = TextStyle {
      font_family: Some(Arc::from("Missing;Arial")),
      high_ansi_font_family: Some(Arc::from("Missing;Arial")),
      east_asia_font_family: Some(Arc::from("Missing;Arial")),
      complex_font_family: Some(Arc::from("Missing;Arial")),
      ..TextStyle::default()
    };
    let word = TextStyle {
      wordprocessingml_font_slots: true,
      ..common.clone()
    };
    // Scriptless/EA requests do not necessarily have Word's Latin missing-
    // face fallback; they must still keep this interpretation in the key.
    for script in [None, Some(TextScript::Latin), Some(TextScript::Han)] {
      assert_eq!(
        font_request(&common, script).family_selection,
        ooxmlsdk_fonts::FontFamilySelection::List
      );
      assert_eq!(
        font_request(&word, script).family_selection,
        ooxmlsdk_fonts::FontFamilySelection::First
      );
      let face = super::FontFaceKey::from_style(&common, script);
      assert_ne!(face, super::FontFaceKey::from_style(&word, script));
      assert!(!face.matches_style(&word, script));
      let metrics = super::FontMetricsKey::from_style_for_slot(&common, script, None);
      assert_ne!(
        metrics,
        super::FontMetricsKey::from_style_for_slot(&word, script, None)
      );
      assert!(!metrics.matches_style_for_slot(&word, script, None));
    }
  }

  fn synthetic_space_run(text: &'static str, script: TextScript) -> ShapedRun<'static, 'static> {
    let glyphs = text
      .char_indices()
      .map(|(start, ch)| ShapedGlyph {
        text_range: start..start + ch.len_utf8(),
        source_char: Some(ch),
        x_advance_pt: if ch == ' ' { 2.5 } else { 6.0 },
        ..ShapedGlyph::default()
      })
      .collect::<Vec<_>>();
    let advance_pt = glyphs.iter().map(|glyph| glyph.x_advance_pt).sum();
    ShapedRun {
      font_id: FontId(Arc::from("synthetic-balance-spaces")),
      font_size_pt: FontSize(10.0),
      text,
      text_range: 0..text.len(),
      glyphs: Cow::Owned(glyphs),
      advance_pt,
      direction: TextDirection::LeftToRight,
      script: Some(script),
      safe_breaks: Vec::new(),
      approximate: false,
      decorations: Vec::new(),
      diagnostics: ShapingDiagnostics::default(),
    }
  }

  #[test]
  fn single_double_byte_balance_expands_only_adjacent_latin_spaces() {
    let mut trailing = synthetic_space_run("A: ", TextScript::Latin);
    apply_wordprocessingml_single_double_byte_width_balance(&mut trailing, 1.0, 0.0);
    assert_eq!(trailing.glyphs[2].x_advance_pt, 2.5);
    assert_eq!(trailing.advance_pt, 14.5);

    let mut internal = synthetic_space_run("A B", TextScript::Latin);
    apply_wordprocessingml_single_double_byte_width_balance(&mut internal, 1.0, 0.0);
    assert_eq!(internal.glyphs[1].x_advance_pt, 2.5);
    assert_eq!(internal.advance_pt, 14.5);

    let mut adjacent = synthetic_space_run("A  B", TextScript::Latin);
    apply_wordprocessingml_single_double_byte_width_balance(&mut adjacent, 1.0, 0.0);
    assert_eq!(adjacent.glyphs[1].x_advance_pt, 5.0);
    assert_eq!(adjacent.glyphs[2].x_advance_pt, 5.0);
  }

  #[test]
  fn single_double_byte_balance_resets_cjk_spaces_before_other_spacing() {
    let mut run = synthetic_space_run("甲 乙", TextScript::Han);
    apply_wordprocessingml_single_double_byte_width_balance(&mut run, 0.8, 0.25);
    assert_eq!(run.glyphs[1].x_advance_pt, 4.25);
    assert_eq!(run.advance_pt, 16.25);
  }

  #[test]
  fn kerning_feature_follows_the_wordprocessingml_size_threshold() {
    let mut style = TextStyle {
      font_size_pt: 11.0,
      kerning_minimum_size_pt: Some(12.0),
      ..Default::default()
    };

    let request = font_request(&style, None);
    assert_eq!(request.features[0].tag, "kern");
    assert_eq!(request.features[0].value, 0);

    style.font_size_pt = 12.0;
    assert_eq!(font_request(&style, None).features[0].value, 1);

    style.font_size_pt = 65.999_99;
    style.kerning_minimum_size_pt = Some(66.0);
    assert_eq!(font_request(&style, None).features[0].value, 1);

    style.font_size_pt = 65.99;
    assert_eq!(font_request(&style, None).features[0].value, 0);

    style.font_size_pt = 66.0;
    style.kerning_minimum_size_pt = Some(66.01);
    assert_eq!(font_request(&style, None).features[0].value, 0);

    style.kerning_minimum_size_pt = Some(f32::INFINITY);
    assert_eq!(font_request(&style, None).features[0].value, 0);
  }

  #[test]
  fn wordprocessing_complex_script_override_controls_the_full_font_request() {
    let style = TextStyle {
      font_family: Some(Arc::from("Latin Face")),
      complex_font_family: Some(Arc::from("Complex Face")),
      font_size_pt: 10.0,
      complex_font_size_pt: Some(20.0),
      complex_script: Some(true),
      bold: false,
      complex_bold: Some(true),
      italic: true,
      complex_italic: Some(false),
      ..Default::default()
    };

    let request = font_request(&style, Some(TextScript::Latin));
    assert_eq!(request.family.as_deref(), Some("Complex Face"));
    assert_eq!(request.size_pt.0, 20.0);
    assert!(request.bold);
    assert!(!request.italic);
  }

  #[test]
  fn wordprocessing_rtl_numeric_portions_keep_complex_run_properties() {
    let style = TextStyle {
      font_family: Some(Arc::from("Latin Face")),
      complex_font_family: Some(Arc::from("Complex Face")),
      font_size_pt: 10.0,
      complex_font_size_pt: Some(20.0),
      right_to_left: Some(true),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };

    // Native Word uses one face for each of these entire portions. Reversing
    // the number/letter order changes the painted face, but never szCs or
    // the complex-script face that contributes the line metrics.
    for (text, explicit_cs, slot, family) in [
      ("1A", None, WordprocessingFontSlot::Ascii, "Latin Face"),
      (
        "A1",
        None,
        WordprocessingFontSlot::ComplexScript,
        "Complex Face",
      ),
      (
        "1A",
        Some(true),
        WordprocessingFontSlot::ComplexScript,
        "Complex Face",
      ),
    ] {
      let style = TextStyle {
        complex_script: explicit_cs,
        ..style.clone()
      };
      let runs = script_direction_runs_with_options(
        text,
        FontSize(style.font_size_pt),
        script_scan_options(&style, false),
      );
      assert_eq!(runs.len(), 1);
      assert_eq!(&text[runs[0].text_range.clone()], text);
      assert_eq!(runs[0].wordprocessingml_font_slot, Some(slot));
      let request = font_request_for_slot(&style, Some(runs[0].script), Some(slot));
      assert_eq!(request.family.as_deref(), Some(family));
      assert_eq!(request.size_pt.0, 20.0);
      assert_eq!(
        wordprocessing_line_metrics_font_slot(&style, Some(slot)),
        Some(WordprocessingFontSlot::ComplexScript)
      );
    }

    let ordinary = TextStyle {
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    assert_eq!(
      wordprocessing_line_metrics_font_slot(&ordinary, Some(WordprocessingFontSlot::Ascii)),
      Some(WordprocessingFontSlot::Ascii)
    );
  }

  #[test]
  fn wordprocessing_rtl_numeric_punctuation_keeps_complex_run_properties() {
    let style = TextStyle {
      font_family: Some(Arc::from("Latin Face")),
      complex_font_family: Some(Arc::from("Complex Face")),
      font_size_pt: 10.0,
      complex_font_size_pt: Some(20.0),
      right_to_left: Some(true),
      bold: false,
      complex_bold: Some(true),
      italic: true,
      complex_italic: Some(false),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    for text in ["44.2", "44,2", "44:2", "44،2", "44٫2"] {
      let runs = script_direction_runs_with_options(
        text,
        FontSize(style.font_size_pt),
        script_scan_options(&style, false),
      );
      for run in runs {
        let request =
          font_request_for_slot(&style, Some(run.script), run.wordprocessingml_font_slot);
        assert_eq!(request.family.as_deref(), Some("Latin Face"), "{text}");
        assert_eq!(request.size_pt.0, 20.0);
        assert!(request.bold);
        assert!(!request.italic);
        assert_eq!(
          wordprocessing_line_metrics_font_slot(&style, run.wordprocessingml_font_slot),
          Some(WordprocessingFontSlot::ComplexScript)
        );
      }
    }
  }

  #[test]
  fn forced_complex_font_keeps_its_own_family_class() {
    use ooxmlsdk_fonts::FontFamilyClass;
    let style = TextStyle {
      font_family: Some(Arc::from("Latin Face")),
      complex_font_family: Some(Arc::from("Missing Roman Face")),
      font_family_class: Some(FontFamilyClass::SansSerif),
      complex_font_family_class: Some(FontFamilyClass::Serif),
      complex_script: Some(true),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    for slot in [None, Some(WordprocessingFontSlot::ComplexScript)] {
      let request = font_request_for_slot(&style, Some(TextScript::Arabic), slot);
      assert_eq!(request.family.as_deref(), Some("Missing Roman Face"));
      assert_eq!(request.family_class, Some(FontFamilyClass::Serif));
    }
    // Word's ASCII-digit exception must retain the Latin face and metadata.
    let digits = font_request_for_slot(
      &style,
      Some(TextScript::Common),
      Some(WordprocessingFontSlot::Ascii),
    );
    assert_eq!(digits.family.as_deref(), Some("Latin Face"));
    assert_eq!(digits.family_class, Some(FontFamilyClass::SansSerif));
    let unclassified = TextStyle {
      complex_font_family_class: None,
      ..style
    };
    assert_eq!(
      font_request(&unclassified, Some(TextScript::Arabic)).family_class,
      None
    );
  }

  #[test]
  fn latin_font_table_family_class_does_not_leak_into_the_east_asian_slot() {
    let style = TextStyle {
      font_family: Some(Arc::from("MetaBook-Roman")),
      east_asia_font_family: Some(Arc::from("AR PL SungtiL GB")),
      font_family_class: Some(ooxmlsdk_fonts::FontFamilyClass::Serif),
      ..Default::default()
    };

    assert_eq!(
      font_request(&style, Some(TextScript::Latin)).family_class,
      Some(ooxmlsdk_fonts::FontFamilyClass::Serif)
    );
    assert_eq!(
      font_request(&style, Some(TextScript::Han))
        .family
        .as_deref(),
      Some("AR PL SungtiL GB")
    );
    assert_eq!(
      font_request(&style, Some(TextScript::Han)).family_class,
      None
    );
  }

  #[test]
  fn drawingml_chart_theme_keeps_han_kana_and_latin_faces_independent() {
    let style = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      east_asia_font_family: Some(Arc::from("SimSun")),
      drawingml_japanese_font_family: Some(Arc::from("MS Mincho")),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    for slot in [None, Some(WordprocessingFontSlot::EastAsia)] {
      for (script, family) in [
        (TextScript::Han, "SimSun"),
        (TextScript::Hiragana, "MS Mincho"),
        (TextScript::Katakana, "MS Mincho"),
      ] {
        let request = font_request_for_slot(&style, Some(script), slot);
        assert_eq!(request.family.as_deref(), Some(family));
      }
    }
    let latin = font_request_for_slot(
      &style,
      Some(TextScript::Katakana),
      Some(WordprocessingFontSlot::Ascii),
    );
    assert_eq!(latin.family.as_deref(), Some("Calibri"));
    let painted = crate::model::common_text_style(style.clone());
    assert_eq!(
      font_request(&painted, Some(TextScript::Katakana))
        .family
        .as_deref(),
      Some("MS Mincho"),
    );
    let explicit = TextStyle {
      drawingml_japanese_font_family: None,
      ..style
    };
    assert_eq!(
      font_request(&explicit, Some(TextScript::Katakana))
        .family
        .as_deref(),
      Some("SimSun"),
    );
  }

  #[test]
  fn wordprocessingml_hinted_east_asia_maps_latin_faces_before_glyph_fallback() {
    use super::FontResolver;
    use ooxmlsdk_fonts::WordprocessingFontTypeHint;

    let mut resolver = FontResolver::default();
    for family in [
      "DejaVu Sans",
      "DejaVu Serif",
      "Arial",
      "Segoe UI Symbol",
      "Cambria Math",
    ] {
      let style = TextStyle {
        font_family: Some(Arc::from(family)),
        high_ansi_font_family: Some(Arc::from(family)),
        east_asia_font_family: Some(Arc::from(family)),
        wordprocessingml_font_slots: true,
        wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
        ..Default::default()
      };
      // Native ordinary-run repertoire controls: mapping precedes coverage,
      // so even a covered star moves to the CJK face. The unsupported ballot
      // boxes link from that face to Segoe UI Symbol, independently of SDTs.
      for (text, expected) in [("★□→√ΩА€§¤", "SimSun"), ("☒☐", "SegoeUISymbol")] {
        let runs = resolver.shape_text_runs(text, &style).unwrap();
        assert!(
          runs.iter().all(|run| run.font_id.0.contains(expected)),
          "{family} {text}: {runs:?}"
        );
      }
      let latin = resolver.shape_text_runs("Aé", &style).unwrap();
      let ordinary = TextStyle {
        wordprocessingml_font_hint: None,
        ..style
      };
      assert_eq!(latin, resolver.shape_text_runs("Aé", &ordinary).unwrap());
    }
  }

  #[test]
  fn wordprocessingml_hinted_east_asia_preserves_physical_cjk_faces_and_other_consumers() {
    use super::FontResolver;
    use ooxmlsdk_fonts::WordprocessingFontTypeHint;

    let mut resolver = FontResolver::default();
    let style = TextStyle {
      font_family: Some(Arc::from("DejaVu Sans")),
      high_ansi_font_family: Some(Arc::from("DejaVu Sans")),
      east_asia_font_family: Some(Arc::from("DejaVu Sans")),
      wordprocessingml_font_slots: true,
      wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
      ..Default::default()
    };
    let ordinary = TextStyle {
      wordprocessingml_font_hint: None,
      ..style.clone()
    };
    let covered = resolver.shape_text_runs("☒", &ordinary).unwrap();
    assert!(covered[0].font_id.0.contains("DejaVuSans"));
    assert_eq!(covered[0].glyphs[0].glyph_id, 3818);
    let mapped = resolver.shape_text_runs("☒", &style).unwrap();
    assert_eq!(mapped[0].glyphs[0].glyph_id, 1409);
    // Reuse the same resolver in both directions to catch registry/selection
    // cache contamination between hinted and ordinary Common-script text.
    assert_eq!(covered, resolver.shape_text_runs("☒", &ordinary).unwrap());
    let non_word = TextStyle {
      wordprocessingml_font_slots: false,
      ..style.clone()
    };
    assert_eq!(
      resolver.shape_text_runs("☒", &non_word).unwrap()[0].font_id,
      covered[0].font_id
    );
    let cjk = TextStyle {
      font_family: Some(Arc::from("Meiryo")),
      high_ansi_font_family: Some(Arc::from("Meiryo")),
      east_asia_font_family: Some(Arc::from("Meiryo")),
      ..style
    };
    let runs = resolver.shape_text_runs("☒★Ωé", &cjk).unwrap();
    assert!(runs.iter().all(|run| run.font_id.0.contains("Meiryo")));
    assert_eq!(runs[0].glyphs[0].glyph_id, 21398);
  }

  #[test]
  fn wordprocessingml_hinted_east_asia_uses_realized_face_line_metrics() {
    use super::FontResolver;
    use ooxmlsdk_fonts::WordprocessingFontTypeHint;

    let mut resolver = FontResolver::default();
    let mut style = TextStyle {
      font_family: Some(Arc::from("DejaVu Sans")),
      high_ansi_font_family: Some(Arc::from("DejaVu Sans")),
      east_asia_font_family: Some(Arc::from("DejaVu Sans")),
      wordprocessingml_font_slots: true,
      wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
      ..Default::default()
    };
    for size in [8.0, 11.0, 14.0] {
      style.font_size_pt = size;
      let actual = resolver.text_vertical_metrics("☒", &style).unwrap();
      // Native physical Segoe metrics are 2210/514/0 at UPEM2048. The
      // declared SimSun mapper face is 220/36/36 at UPEM256 and must not
      // determine this linked glyph's baseline or the following line.
      assert!((actual.ascent_pt - size * 2210.0 / 2048.0).abs() < 0.0001);
      assert!((actual.descent_pt - size * 514.0 / 2048.0).abs() < 0.0001);
      assert_eq!(actual.line_gap_pt, 0.0);
      assert_eq!(actual.directwrite_baseline_offset_pt, actual.ascent_pt);
      assert!(!actual.wordprocessingml_cjk_line_metrics);
      let cjk = resolver.text_vertical_metrics("★", &style).unwrap();
      assert!((cjk.ascent_pt - size * 220.0 / 256.0).abs() < 0.0001);
      assert!(cjk.wordprocessingml_cjk_line_metrics);
    }
    style.wordprocessingml_cjk_line_metrics = true;
    let runs = resolver
      .wordprocessingml_line_metric_runs("☒★", &style)
      .unwrap();
    assert_eq!(runs.len(), 2);
    assert!(!runs[0].wordprocessingml_cjk_line_metrics);
    assert!(runs[1].wordprocessingml_cjk_line_metrics);
  }

  #[test]
  fn wordprocessingml_join_controls_do_not_add_linked_face_metrics() {
    use super::FontResolver;
    use ooxmlsdk_fonts::WordprocessingFontTypeHint;

    let mut resolver = FontResolver::default();
    let style = TextStyle {
      font_family: Some(Arc::from("DejaVu Sans")),
      high_ansi_font_family: Some(Arc::from("DejaVu Sans")),
      east_asia_font_family: Some(Arc::from("DejaVu Sans")),
      wordprocessingml_font_slots: true,
      wordprocessingml_font_hint: Some(WordprocessingFontTypeHint::EastAsia),
      ..Default::default()
    };
    let latin = resolver.text_vertical_metrics("AA", &style).unwrap();
    for text in ["A\u{200c}A", "A\u{200d}A"] {
      let shaped = resolver.shape_text_runs(text, &style).unwrap();
      assert_eq!(shaped.iter().map(|run| run.text).collect::<String>(), text);
      assert_eq!(resolver.text_vertical_metrics(text, &style), Some(latin));
      assert!(
        resolver
          .wordprocessingml_line_metric_runs(
            text,
            &TextStyle {
              wordprocessingml_cjk_line_metrics: true,
              ..style.clone()
            },
          )
          .unwrap()
          .iter()
          .all(|metrics| !metrics.wordprocessingml_cjk_line_metrics)
      );
    }
    // The same link is a real line owner when it paints a visible ballot box.
    let symbol = resolver.text_vertical_metrics("☒", &style).unwrap();
    assert!(symbol.ascent_pt > latin.ascent_pt);
  }

  #[test]
  fn wordprocessingml_hinted_east_asia_separates_primary_metric_caches() {
    use super::FontResolver;
    use ooxmlsdk_fonts::WordprocessingFontTypeHint;

    let mut resolver = FontResolver::default();
    let mut style = TextStyle {
      font_family: Some(Arc::from("DejaVu Sans")),
      east_asia_font_family: Some(Arc::from("DejaVu Sans")),
      font_size_pt: 11.0,
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let slot = Some(WordprocessingFontSlot::EastAsia);
    let original = resolver
      .font_metrics_for_slot(&style, Some(TextScript::Common), slot)
      .unwrap()
      .vertical;
    style.wordprocessingml_font_hint = Some(WordprocessingFontTypeHint::EastAsia);
    let mapped = resolver
      .font_metrics_for_slot(&style, Some(TextScript::Common), slot)
      .unwrap()
      .vertical;
    assert!(!original.wordprocessingml_cjk_line_metrics);
    assert!(mapped.wordprocessingml_cjk_line_metrics);
    style.wordprocessingml_font_hint = None;
    assert_eq!(
      resolver
        .font_metrics_for_slot(&style, Some(TextScript::Common), slot)
        .unwrap()
        .vertical,
      original
    );
  }

  #[test]
  fn wordprocessing_font_slot_selects_face_independently_from_shaping_script() {
    let style = TextStyle {
      font_family: Some(Arc::from("Ascii Face")),
      high_ansi_font_family: Some(Arc::from("High ANSI Face")),
      east_asia_font_family: Some(Arc::from("East Asian Face")),
      complex_font_family: Some(Arc::from("Complex Face")),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };

    let greek_in_east_asia = font_request_for_slot(
      &style,
      Some(TextScript::Greek),
      Some(WordprocessingFontSlot::EastAsia),
    );
    assert_eq!(
      greek_in_east_asia.family.as_deref(),
      Some("East Asian Face")
    );
    assert_eq!(greek_in_east_asia.script, Some(TextScript::Greek));

    let arabic_in_ascii = font_request_for_slot(
      &style,
      Some(TextScript::Arabic),
      Some(WordprocessingFontSlot::Ascii),
    );
    assert_eq!(arabic_in_ascii.family.as_deref(), Some("Ascii Face"));
    assert_eq!(arabic_in_ascii.script, Some(TextScript::Arabic));

    let high_ansi = font_request_for_slot(
      &style,
      Some(TextScript::Latin),
      Some(WordprocessingFontSlot::HighAnsi),
    );
    assert_eq!(high_ansi.family.as_deref(), Some("High ANSI Face"));

    // OfficeMath first selects the slot from serialized text and only then
    // maps m:scr/m:sty to the displayed mathematical-alphabet character.
    // Pin both sides: ASCII `f` remains on the ASCII face after becoming
    // U+1D453, while an authored U+3016 remains on the East Asian face.
    let ascii = materialize_wordprocessingml_source_font_slot(&style, 'f');
    assert_eq!(ascii.font_family.as_deref(), Some("Ascii Face"));
    assert_eq!(ascii.east_asia_font_family.as_deref(), Some("Ascii Face"));
    assert!(ascii.wordprocessingml_font_slots);
    let variant_runs = script_direction_runs_with_options(
      "𝑓",
      FontSize(ascii.font_size_pt),
      script_scan_options(&ascii, false),
    );
    assert_eq!(
      variant_runs[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::EastAsia)
    );
    assert_eq!(
      font_request_for_slot(
        &ascii,
        Some(variant_runs[0].script),
        variant_runs[0].wordprocessingml_font_slot,
      )
      .family
      .as_deref(),
      Some("Ascii Face")
    );

    let east_asian = materialize_wordprocessingml_source_font_slot(&style, '〔');
    assert_eq!(east_asian.font_family.as_deref(), Some("East Asian Face"));
    let bracket_runs = script_direction_runs_with_options(
      "〔",
      FontSize(east_asian.font_size_pt),
      script_scan_options(&east_asian, false),
    );
    assert_eq!(
      font_request_for_slot(
        &east_asian,
        Some(bracket_runs[0].script),
        bracket_runs[0].wordprocessingml_font_slot,
      )
      .family
      .as_deref(),
      Some("East Asian Face")
    );
  }

  #[test]
  fn direct_symbol_byte_requires_an_exact_unambiguous_unicode_face() {
    for family in ["Ordinary Face A", "Ordinary Face B"] {
      let mut face = ooxmlsdk_fonts::FontFaceInfo::synthetic("unicode-face", family);
      face.coverage.unicode_ranges = vec![0x26..0x27, 0x41..0x42];
      for byte in *b"&A" {
        assert!(super::font_face_has_direct_symbol_byte(&face, family, byte));
        assert!(!super::font_face_has_direct_symbol_byte(
          &face,
          "Unavailable Original Face",
          byte,
        ));
      }
      assert!(!super::font_face_has_direct_symbol_byte(
        &face, family, b'$'
      ));

      face.flags.symbolic = true;
      assert!(!super::font_face_has_direct_symbol_byte(
        &face, family, b'&'
      ));
      face.flags.symbolic = false;
      face.coverage.unicode_ranges.push(0xF026..0xF027);
      assert!(!super::font_face_has_direct_symbol_byte(
        &face, family, b'&'
      ));
      assert!(super::font_face_has_direct_symbol_byte(&face, family, b'A'));
    }
  }

  #[test]
  fn isolated_symbol_character_uses_symbol_charset_without_generic_family_matching() {
    let symbol = TextStyle {
      font_family: Some(Arc::from("UniversalMath1 BT")),
      high_ansi_font_family: Some(Arc::from("UniversalMath1 BT")),
      east_asia_font_family: Some(Arc::from("UniversalMath1 BT")),
      complex_font_family: Some(Arc::from("UniversalMath1 BT")),
      symbol_font_family: Some(Arc::from("UniversalMath1 BT")),
      fallback_font_family: Some(Arc::from("Symbol")),
      font_family_class: Some(ooxmlsdk_fonts::FontFamilyClass::Serif),
      wordprocessingml_font_slots: false,
      ..TextStyle::default()
    };

    let request = font_request(&symbol, None);
    assert_eq!(request.family.as_deref(), Some("UniversalMath1 BT"));
    assert_eq!(request.charset, Some(FontCharset::Symbol));
    assert_eq!(request.family_class, None);
    assert_eq!(
      script_fallback_font_family_for_slot(&symbol, Some(TextScript::Other), None),
      Some("Symbol")
    );

    // A retained alternate symbol face on an ordinary Word run is not proof
    // that its High ANSI text is a symbol character.
    let ordinary = TextStyle {
      high_ansi_font_family: Some(Arc::from("High ANSI Face")),
      high_ansi_font_family_class: Some(ooxmlsdk_fonts::FontFamilyClass::Serif),
      wordprocessingml_font_slots: true,
      ..symbol
    };
    let request = font_request_for_slot(
      &ordinary,
      Some(TextScript::Latin),
      Some(WordprocessingFontSlot::HighAnsi),
    );
    assert_eq!(request.family.as_deref(), Some("High ANSI Face"));
    assert_eq!(request.charset, None);
    assert_eq!(
      request.family_class,
      Some(ooxmlsdk_fonts::FontFamilyClass::Serif)
    );
  }

  #[test]
  fn script_specific_fallbacks_are_selected_without_wordprocessingml_slots() {
    let style = TextStyle {
      fallback_font_family: Some(Arc::from("Latin fallback")),
      east_asia_fallback_font_family: Some(Arc::from("East Asian fallback")),
      complex_fallback_font_family: Some(Arc::from("Complex fallback")),
      ..TextStyle::default()
    };

    for (script, expected) in [
      (TextScript::Latin, Some("Latin fallback")),
      (TextScript::Han, Some("East Asian fallback")),
      (TextScript::Arabic, Some("Complex fallback")),
      (TextScript::Other, None),
    ] {
      assert_eq!(
        script_fallback_font_family_for_slot(&style, Some(script), None),
        expected
      );
    }
  }

  #[test]
  fn wordprocessing_font_table_characteristics_follow_the_selected_slot() {
    let style = TextStyle {
      font_family: Some(Arc::from("ASCII Face")),
      high_ansi_font_family: Some(Arc::from("High ANSI Face")),
      east_asia_font_family: Some(Arc::from("East Asian Face")),
      complex_font_family: Some(Arc::from("Complex Face")),
      font_charset: Some(FontCharset::ShiftJis),
      high_ansi_font_charset: Some(FontCharset::Ansi),
      east_asia_font_charset: Some(FontCharset::Gb2312),
      complex_font_charset: Some(FontCharset::Arabic),
      font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
      high_ansi_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
      east_asia_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Fixed),
      complex_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Fixed),
      wordprocessingml_font_slots: true,
      ..TextStyle::default()
    };

    for (slot, expected_charset, expected_pitch) in [
      (
        WordprocessingFontSlot::Ascii,
        FontCharset::ShiftJis,
        ooxmlsdk_fonts::FontPitch::Variable,
      ),
      (
        WordprocessingFontSlot::HighAnsi,
        FontCharset::Ansi,
        ooxmlsdk_fonts::FontPitch::Variable,
      ),
      (
        WordprocessingFontSlot::EastAsia,
        FontCharset::Gb2312,
        ooxmlsdk_fonts::FontPitch::Fixed,
      ),
      (
        WordprocessingFontSlot::ComplexScript,
        FontCharset::Arabic,
        ooxmlsdk_fonts::FontPitch::Fixed,
      ),
    ] {
      let request = font_request_for_slot(&style, Some(TextScript::Latin), Some(slot));
      assert_eq!(request.charset, Some(expected_charset), "slot={slot:?}");
      assert_eq!(request.pitch, Some(expected_pitch), "slot={slot:?}");
    }
  }

  #[test]
  fn explicit_false_keeps_unicode_font_selection_and_normal_run_properties() {
    let style = TextStyle {
      font_family: Some(Arc::from("Latin Face")),
      complex_font_family: Some(Arc::from("Complex Face")),
      font_size_pt: 10.0,
      complex_font_size_pt: Some(20.0),
      complex_script: Some(false),
      ..Default::default()
    };

    assert_eq!(
      script_font_family_for_slot(&style, Some(TextScript::Arabic), None),
      Some("Complex Face")
    );
    assert_eq!(
      effective_font_size_pt(&style, Some(TextScript::Arabic)),
      10.0
    );
  }

  #[test]
  fn unicode_script_selects_complex_font_but_not_complex_run_properties() {
    let style = TextStyle {
      font_family: Some(Arc::from("Latin Face")),
      complex_font_family: Some(Arc::from("Complex Face")),
      font_size_pt: 10.0,
      complex_font_size_pt: Some(20.0),
      ..Default::default()
    };

    assert_eq!(
      script_font_family_for_slot(&style, Some(TextScript::Arabic), None),
      Some("Complex Face")
    );
    assert_eq!(
      effective_font_size_pt(&style, Some(TextScript::Arabic)),
      10.0
    );
    assert_eq!(
      effective_font_size_pt(&style, Some(TextScript::Latin)),
      10.0
    );
  }

  #[test]
  fn explicit_right_to_left_selects_complex_properties_without_reversing_latin() {
    let style = TextStyle {
      right_to_left: Some(true),
      complex_font_size_pt: Some(18.0),
      ..Default::default()
    };

    assert_eq!(
      effective_font_size_pt(&style, Some(TextScript::Latin)),
      18.0
    );
    let runs = script_direction_runs_with_options(
      "placeholder",
      FontSize(style.font_size_pt),
      ScriptScanOptions {
        wordprocessingml_font_slots: true,
        ..ScriptScanOptions::default()
      },
    );
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].direction, TextDirection::LeftToRight);
  }

  #[test]
  fn resolved_bidi_level_mirrors_glyphs_without_selecting_complex_formatting() {
    let rtl_style = TextStyle {
      font_size_pt: 10.0,
      complex_font_size_pt: Some(20.0),
      resolved_bidi_level: Some(1),
      ..Default::default()
    };
    assert_eq!(
      effective_font_size_pt(&rtl_style, Some(TextScript::Common)),
      10.0
    );
    let rtl = shape_text_runs("(", &rtl_style).expect("RTL opening parenthesis");
    assert_eq!(rtl.len(), 1);
    assert_eq!(rtl[0].direction, TextDirection::RightToLeft);
    assert_eq!(rtl[0].glyphs[0].source_char, Some('('));

    let ltr_style = TextStyle {
      resolved_bidi_level: Some(0),
      ..rtl_style
    };
    let ltr = shape_text_runs(")", &ltr_style).expect("LTR closing parenthesis");
    assert_eq!(ltr.len(), 1);
    assert_eq!(ltr[0].direction, TextDirection::LeftToRight);
    assert_eq!(rtl[0].glyphs[0].glyph_id, ltr[0].glyphs[0].glyph_id);
  }

  #[test]
  fn resolved_odd_bidi_level_reverses_the_complete_shaping_run_sequence() {
    // Comment066.docx contains this exact w:rtl fragment. Word's ASCII font
    // slot splits the leading neutral punctuation from the Arabic script, but
    // both pieces have the same resolved level and form one visual RTL run.
    let text = "; اطفال ";
    let rtl_style = TextStyle {
      font_family: Some(Arc::from("Cambria")),
      high_ansi_font_family: Some(Arc::from("Cambria")),
      complex_font_family: Some(Arc::from("Times New Roman")),
      right_to_left: Some(true),
      resolved_bidi_level: Some(1),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let rtl = shape_text_runs(text, &rtl_style).expect("resolved RTL fragment");
    assert_eq!(
      rtl
        .iter()
        .map(|run| run.text_range.clone())
        .collect::<Vec<_>>(),
      vec![2..text.len(), 0..2]
    );
    assert!(
      rtl
        .iter()
        .all(|run| run.direction == TextDirection::RightToLeft)
    );

    let ltr_style = TextStyle {
      resolved_bidi_level: Some(0),
      ..rtl_style
    };
    let ltr = shape_text_runs(text, &ltr_style).expect("resolved LTR counterexample");
    assert_eq!(
      ltr
        .iter()
        .map(|run| run.text_range.clone())
        .collect::<Vec<_>>(),
      vec![0..2, 2..text.len()]
    );
  }

  #[test]
  fn ligature_categories_map_to_opentype_features() {
    let style = TextStyle {
      ligatures: Some(OpenTypeLigatures {
        standard: true,
        contextual: false,
        historical: true,
        discretionary: false,
      }),
      ..Default::default()
    };

    let request = font_request(&style, None);
    let features = request
      .features
      .iter()
      .map(|feature| (feature.tag.as_ref(), feature.value))
      .collect::<Vec<_>>();
    assert_eq!(
      features,
      vec![
        ("kern", 1),
        ("liga", 1),
        ("clig", 0),
        ("hlig", 1),
        ("dlig", 0)
      ]
    );
  }

  #[test]
  fn word_2010_typography_maps_to_opentype_features() {
    let mut stylistic_sets = OpenTypeStylisticSets::default();
    stylistic_sets.enable(1);
    stylistic_sets.enable(20);
    let style = TextStyle {
      open_type_features: OpenTypeFeatureSettings {
        number_form: Some(OpenTypeNumberForm::OldStyle),
        number_spacing: Some(OpenTypeNumberSpacing::Tabular),
        contextual_alternates: Some(false),
        stylistic_sets: Some(stylistic_sets),
        vertical_feature: None,
      },
      ..Default::default()
    };

    let features = font_request(&style, None)
      .features
      .into_iter()
      .map(|feature| (feature.tag.into_owned(), feature.value))
      .collect::<Vec<_>>();
    assert_eq!(
      features,
      vec![
        ("kern".to_string(), 1),
        ("onum".to_string(), 1),
        ("tnum".to_string(), 1),
        ("ss01".to_string(), 1),
        ("ss20".to_string(), 1),
        ("calt".to_string(), 0),
      ]
    );
  }

  #[test]
  fn wordprocessingml_disables_contextual_alternates_when_the_extension_is_absent() {
    let style = TextStyle {
      wordprocessingml_font_slots: true,
      ..Default::default()
    };

    assert!(
      font_request(&style, None)
        .features
        .iter()
        .any(|feature| feature.tag == "calt" && feature.value == 0)
    );
  }

  #[test]
  fn wordprocessingml_arabic_connection_forms_survive_optional_alternate_switches() {
    let mut style = TextStyle {
      font_family: Some(Arc::from("Traditional Arabic")),
      complex_font_family: Some(Arc::from("Traditional Arabic")),
      font_size_pt: 15.0,
      complex_font_size_pt: Some(15.0),
      wordprocessingml_font_slots: true,
      right_to_left: Some(true),
      ..Default::default()
    };
    let mut resolver = super::FontResolver::default();
    for enabled in [None, Some(false), Some(true)] {
      style.open_type_features.contextual_alternates = enabled;
      for (text, native) in [
        ("يُعَرّف", [187, 203, 450, 200, 480, 201, 186, 519]),
        ("تُنَظَّم", [502, 267, 476, 200, 508, 201, 186, 427]),
      ] {
        let runs = resolver.shape_text_runs(text, &style).unwrap();
        let glyphs = runs
          .iter()
          .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
          .collect::<Vec<_>>();
        // Native natural glyphs include GSUB's Tatweel before justification.
        // All three authored switch states give the same connection forms.
        assert_eq!(glyphs, native, "{text} {enabled:?}");
      }
      let western = super::font_request(&style, Some(ooxmlsdk_fonts::TextScript::Latin));
      assert!(western.features.iter().any(|feature| {
        feature.tag == "calt" && feature.value == u32::from(enabled.unwrap_or(false))
      }));
    }
  }

  #[test]
  fn wordprocessingml_rtl_italic_uses_upright_outlines_without_crossing_cached_styles() {
    let mut style = TextStyle {
      font_family: Some(Arc::from("Arial")),
      complex_font_family: Some(Arc::from("Arial")),
      wordprocessingml_font_slots: true,
      right_to_left: Some(true),
      italic: true,
      complex_italic: Some(true),
      ..Default::default()
    };
    let mut resolver = super::FontResolver::default();
    for bold in [false, true] {
      style.bold = bold;
      style.complex_bold = Some(bold);
      let mut upright = style.clone();
      upright.italic = false;
      upright.complex_italic = Some(false);
      for text in ["بسم", "abc 123", "אבג"] {
        let expected = resolver.shape_text_runs(text, &upright).unwrap();
        let actual = resolver.shape_text_runs(text, &style).unwrap();
        assert_eq!(actual.len(), expected.len());
        for (actual, expected) in actual.iter().zip(&expected) {
          assert_eq!(actual.font_id, expected.font_id);
          assert_eq!(actual.glyphs, expected.glyphs);
          assert!(
            resolver
              .font_face_data(&actual.font_id)
              .unwrap()
              .synthetic_italic
          );
        }
      }
    }

    // The same resolver must not reuse the synthetic face for ordinary
    // italics or w:cs, even when family, weight and effective italics agree.
    let synthetic = resolver.cached_text_face(&style).unwrap();
    for complex_script in [None, Some(true)] {
      let mut ordinary = style.clone();
      ordinary.right_to_left = Some(false);
      ordinary.complex_script = complex_script;
      ordinary.resolved_bidi_level = Some(1);
      assert_eq!(font_request(&ordinary, None).slant, None);
      let face = resolver.cached_text_face(&ordinary).unwrap();
      assert!(!face.synthetic_italic);
      assert_ne!(face.id(), synthetic.id());
      assert_eq!(
        resolver.cached_text_face(&style).unwrap().id(),
        synthetic.id()
      );
    }
    let mut generic = style.clone();
    generic.wordprocessingml_font_slots = false;
    assert_eq!(font_request(&generic, None).slant, None);
    let mut hebrew = style.clone();
    hebrew.bidi_language = Some(Arc::from("he-IL"));
    let face = resolver.cached_text_face(&hebrew).unwrap();
    assert!(!face.synthetic_italic);
    assert_ne!(face.id(), synthetic.id());
    assert_eq!(
      resolver.cached_text_face(&style).unwrap().id(),
      synthetic.id()
    );

    // Arabic-capable italic families must retain their authored face. This
    // covers table-ltr.docx, which mixes real Amiri italics with upright runs.
    let mut amiri = style.clone();
    amiri.font_family = Some(Arc::from("Amiri"));
    amiri.complex_font_family = Some(Arc::from("Amiri"));
    let face = resolver.cached_text_face(&amiri).unwrap();
    assert!(!face.synthetic_italic);
    assert!(face.id().to_ascii_lowercase().contains("italic"));
  }

  #[test]
  fn wordprocessingml_tracked_arabic_retains_native_measurement_width() {
    use super::FontResolver;

    let mut resolver = FontResolver::default();
    // Native Word placement/conversion arrays for "بالقرآن ", width90:
    // mode14 without Word97 measurement and mode15 use em2048, lfWidth847;
    // mode14 with Word97 measurement uses em1000, lfWidth414. Source pitch
    // changes neither font request nor the integer natural glyph advances.
    for (legacy, expected_units) in [
      (false, [21540, 9240, 16350, 10740, 12930, 23460, 13500]),
      (true, [21565, 9216, 16343, 10752, 12902, 23470, 13516]),
    ] {
      for spacing in [-0.1, 0.1] {
        let style = TextStyle {
          font_family: Some(Arc::from("Traditional Arabic")),
          complex_font_family: Some(Arc::from("Traditional Arabic")),
          font_size_pt: 15.0,
          horizontal_scale: Some(0.9),
          wordprocessing_font_width_percent: Some(90),
          wordprocessing_legacy_font_measurement: Some(legacy),
          character_spacing_pt: spacing,
          right_to_left: Some(true),
          resolved_bidi_level: Some(1),
          wordprocessingml_font_slots: true,
          ..Default::default()
        };
        let runs = resolver.shape_text_runs("بالقرآن ", &style).unwrap();
        let glyphs = runs
          .iter()
          .flat_map(|run| run.glyphs.iter())
          .collect::<Vec<_>>();
        assert_eq!(
          glyphs
            .iter()
            .rev()
            .map(|glyph| glyph.glyph_id)
            .collect::<Vec<_>>(),
          [682, 499, 492, 450, 161, 192, 3]
        );
        assert_eq!(
          glyphs
            .iter()
            .rev()
            .map(|glyph| ((glyph.x_advance_pt - spacing) * 4096.0).round() as i32)
            .collect::<Vec<_>>(),
          expected_units,
          "legacy {legacy}, pitch {spacing}"
        );
      }
    }
  }

  #[test]
  fn wordprocessingml_arabic_orphan_mark_groups_match_native_glyphs() {
    use super::FontResolver;

    let cases = [
      ("َّ", [200, 203].as_slice(), [267].as_slice()),
      ("َّ", [200, 203].as_slice(), [267].as_slice()),
      ("ُّ", [201, 203].as_slice(), [268].as_slice()),
      ("ُّ", [201, 203].as_slice(), [268].as_slice()),
      ("ِّ", [202, 203].as_slice(), [202, 203].as_slice()),
      ("ِّ", [202, 203].as_slice(), [202, 203].as_slice()),
      ("ُُ", [201, 201].as_slice(), [201, 201].as_slice()),
      ("ُِّ", [202, 201, 203].as_slice(), [202, 268].as_slice()),
    ];
    let mut resolver = FontResolver::default();
    // Native Word's 64 controls: two fonts, eight mark sequences, and
    // standalone/leading/attached/foreign-language boundaries. The glyph
    // arrays below omit only the native run's empty trailing space glyph.
    for (family, circle, base) in [("Traditional Arabic", 588, 167), ("Arial", 2825, 911)] {
      let style = TextStyle {
        font_family: Some(Arc::from(family)),
        complex_font_family: Some(Arc::from(family)),
        font_size_pt: 15.0,
        horizontal_scale: Some(1.03),
        wordprocessing_font_width_percent: Some(103),
        wordprocessing_legacy_font_measurement: Some(true),
        right_to_left: Some(true),
        resolved_bidi_level: Some(1),
        wordprocessingml_font_slots: true,
        ..Default::default()
      };
      for (sequence, marks, attached_marks) in cases {
        let map = |id| match (family, id) {
          ("Arial", 200) => 756,
          ("Arial", 201) => 757,
          ("Arial", 202) => 758,
          ("Arial", 203) => 759,
          ("Arial", 267) => 841,
          ("Arial", 268) => 842,
          _ => id,
        };
        let expected = marks
          .iter()
          .flat_map(|&id| [map(id), circle])
          .collect::<Vec<_>>();
        for (text, attached) in [
          (sequence.to_owned(), false),
          (format!("{sequence}ب"), false),
          (format!("ب{sequence}"), true),
        ] {
          let runs = resolver.shape_text_runs(&text, &style).unwrap();
          let glyphs = runs
            .iter()
            .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
            .collect::<Vec<_>>();
          let expected = if attached {
            attached_marks
              .iter()
              .map(|&id| map(id))
              .chain([base])
              .collect::<Vec<_>>()
          } else if text.ends_with('ب') {
            std::iter::once(base)
              .chain(expected.iter().copied())
              .collect()
          } else {
            expected.clone()
          };
          assert_eq!(glyphs, expected, "{family} {text:?}");
          for run in &runs {
            for glyph in run.glyphs.iter() {
              assert!(text.is_char_boundary(glyph.text_range.start));
              assert!(text.is_char_boundary(glyph.text_range.end));
              assert!(glyph.text_range.end <= text.len());
            }
          }
        }
        // A valid preceding Arabic base retains one continuous mark group
        // even though the text passed to the shaper starts with the marks.
        let mut contextual = style.clone();
        contextual.shaping_context = Some(Arc::new(crate::common::TextShapingContext {
          before: Arc::from("ب"),
          after: Arc::from(""),
          leading_marks_only: true,
        }));
        let runs = resolver.shape_text_runs(sequence, &contextual).unwrap();
        assert!(
          runs
            .iter()
            .flat_map(|run| run.glyphs.iter())
            .all(|glyph| glyph.glyph_id != circle)
        );
      }
      // Additional native basic-mark pairs distinguish canonical mark
      // ordering from preserving serialized order after separating circles.
      for (sequence, native) in [
        ("ُِ", [202, 201]),
        ("َُ", [201, 200]),
        ("ًَ", [200, 197]),
        ("ّْ", [204, 203]),
      ] {
        let glyphs = resolver
          .shape_text_runs(sequence, &style)
          .unwrap()
          .iter()
          .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
          .collect::<Vec<_>>();
        let expected = native
          .into_iter()
          .flat_map(|id| {
            let id = if family == "Arial" { id + 556 } else { id };
            [id, circle]
          })
          .collect::<Vec<_>>();
        assert_eq!(glyphs, expected, "{family} {sequence:?}");
      }
    }
  }

  #[test]
  fn wordprocessingml_arabic_leading_marks_preserve_valid_base_context() {
    use super::FontResolver;
    use skrifa::MetadataProvider;

    let style = TextStyle {
      font_family: Some(Arc::from("Traditional Arabic")),
      complex_font_family: Some(Arc::from("Traditional Arabic")),
      font_size_pt: 15.0,
      right_to_left: Some(true),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let mut resolver = FontResolver::default();
    let orphan = resolver.shape_text_runs("َ", &style).unwrap();
    assert_eq!(orphan.len(), 1);
    assert_eq!(orphan[0].script, Some(TextScript::Arabic));
    let face = resolver.font_face_data(&orphan[0].font_id).unwrap();
    let font = skrifa::FontRef::from_index(face.data.as_slice(), face.index).unwrap();
    let circle = font.charmap().map('\u{25cc}').unwrap().to_u32();
    assert_eq!(orphan[0].glyphs.len(), 2);
    assert!(
      orphan[0]
        .glyphs
        .iter()
        .any(|glyph| glyph.glyph_id == circle)
    );
    assert!(orphan[0].advance_pt > 0.0);
    assert!(
      orphan[0]
        .glyphs
        .iter()
        .all(|glyph| glyph.text_range == (0.."َ".len()))
    );

    let mut attached = style.clone();
    attached.shaping_context = Some(Arc::new(crate::common::TextShapingContext {
      before: Arc::from("هدف"),
      after: Arc::from(""),
      leading_marks_only: true,
    }));
    let mark = resolver.shape_text_runs("َ", &attached).unwrap();
    assert_eq!(mark[0].glyphs.len(), 1);
    assert_eq!(mark[0].advance_pt, 0.0);
    // Wrapping the rest of a run must not apply its leading mark's context
    // to a later ordinary Arabic word.
    let later = "كفالة";
    assert_eq!(
      resolver.shape_text_runs(later, &style).unwrap(),
      resolver.shape_text_runs(later, &attached).unwrap()
    );
    let mut generic = style;
    generic.wordprocessingml_font_slots = false;
    let mark = resolver.shape_text_runs("َ", &generic).unwrap();
    assert_eq!(mark[0].glyphs.len(), 1);
    assert_eq!(mark[0].advance_pt, 0.0);
  }

  #[test]
  fn wordprocessingml_variation_selectors_do_not_acquire_arabic_mark_fallback() {
    use super::arabic_nonspacing_mark;

    for mark in ['\u{064b}', '\u{064e}', '\u{0651}', '\u{06e1}'] {
      assert!(arabic_nonspacing_mark(mark), "U+{:04X}", u32::from(mark));
    }
    let style = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      complex_font_family: Some(Arc::from("Calibri")),
      font_size_pt: 11.0,
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let mut resolver = super::FontResolver::default();
    for selector in ['\u{fe0e}', '\u{fe0f}', '\u{e0100}'] {
      assert!(!arabic_nonspacing_mark(selector));
      let text = selector.to_string();
      let runs = resolver.shape_text_runs(&text, &style).unwrap();
      assert_eq!(runs.iter().map(|run| run.glyphs.len()).sum::<usize>(), 1);
      assert_eq!(runs.iter().map(|run| run.advance_pt).sum::<f32>(), 0.0);
    }
    for mark in ['\u{0305}', '\u{05b0}', '\u{093a}'] {
      assert!(!arabic_nonspacing_mark(mark), "U+{:04X}", u32::from(mark));
    }
  }

  #[test]
  fn wordprocessingml_preserves_explicit_arabic_mark_components() {
    let style = TextStyle {
      font_family: Some(Arc::from("Arial")),
      complex_font_family: Some(Arc::from("Arial")),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let mut generic = style.clone();
    generic.wordprocessingml_font_slots = false;
    let text = "لَآ";
    let word = shape_text_runs(text, &style).unwrap();
    let ordinary = shape_text_runs(text, &generic).unwrap();
    assert_eq!(word.iter().map(|r| r.text).collect::<String>(), text);
    assert_eq!(
      word.iter().map(|r| r.glyphs.len()).sum::<usize>(),
      ordinary.iter().map(|r| r.glyphs.len()).sum::<usize>() + 1
    );
    // An authored precomposed alef remains precomposed in both policies.
    let glyph_ids = |style: &TextStyle| {
      shape_text_runs("لَآ", style)
        .unwrap()
        .iter()
        .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
        .collect::<Vec<_>>()
    };
    assert_eq!(glyph_ids(&style), glyph_ids(&generic));
  }

  #[test]
  fn wordprocessingml_arabic_retains_standard_shaping_ligatures() {
    let style = TextStyle {
      font_family: Some(Arc::from("Arial")),
      complex_font_family: Some(Arc::from("Arial")),
      wordprocessingml_font_slots: true,
      ligatures: Some(OpenTypeLigatures::default()),
      ..Default::default()
    };
    let liga = |style: &TextStyle, script| {
      font_request(style, Some(script))
        .features
        .iter()
        .find(|feature| feature.tag == "liga")
        .expect("explicit standard ligature setting")
        .value
    };
    assert_eq!(liga(&style, TextScript::Arabic), 1);
    assert_eq!(liga(&style, TextScript::Latin), 0);
    let mut generic = style.clone();
    generic.wordprocessingml_font_slots = false;
    assert_eq!(liga(&generic, TextScript::Arabic), 0);

    // Exercise shaping as well as feature selection: a Word request whose
    // optional ligatures are off must retain the engine's Arabic ligature
    // glyphs and advances. Semantic source ranges still cover the full word.
    let word = "الله";
    let actual = shape_text_runs(word, &style).expect("Word Arabic shaping");
    let mut enabled = style.clone();
    enabled.ligatures.as_mut().unwrap().standard = true;
    let expected = shape_text_runs(word, &enabled).expect("Arabic standard ligatures");
    let glyphs = |runs: &[ShapedRun<'_, '_>]| {
      runs
        .iter()
        .flat_map(|run| run.glyphs.iter())
        .map(|glyph| (glyph.glyph_id, glyph.text_range.clone(), glyph.x_advance_pt))
        .collect::<Vec<_>>()
    };
    assert_eq!(glyphs(&actual), glyphs(&expected));
    assert!(
      actual
        .iter()
        .flat_map(|run| run.glyphs.iter())
        .any(|glyph| { glyph.text_range.start == 0 && glyph.text_range.end == word.len() })
    );
  }

  #[test]
  fn word_fractional_blanks_follow_unshaped_font_metrics() {
    // Actual Word ideal arrays at12pt. Proportional Mincho's em dash is171
    // design units /256, not one em. Meiryo's Latin GPOS kern feature selects
    // shaped metrics; Consolas lacks the East Asian code-page capability.
    for (family, thirds, sixths) in [
      ("MS Mincho", 8192, 4096),
      ("MS Gothic", 8192, 4096),
      ("MS PMincho", 10944, 5472),
      ("MS PGothic", 10944, 5472),
      ("Meiryo", 16392, 8184),
      ("Consolas", 27024, 27024),
    ] {
      for size in [12.0, 18.0] {
        for (ch, ideal) in [('\u{2004}', thirds), ('\u{2006}', sixths)] {
          let text = format!("A{ch}{ch}{ch}A");
          let mut style = TextStyle {
            font_family: Some(Arc::from(family)),
            high_ansi_font_family: Some(Arc::from(family)),
            font_size_pt: size,
            wordprocessingml_font_slots: true,
            ..Default::default()
          };
          let runs = shape_text_runs(&text, &style).expect("Word fractional space");
          let blanks = runs
            .iter()
            .flat_map(|run| run.glyphs.iter())
            .filter(|glyph| glyph.source_char == Some(ch))
            .collect::<Vec<_>>();
          assert_eq!(blanks.len(), 3);
          for glyph in blanks {
            let expected = ideal as f32 / 4096.0 * size / 12.0;
            assert!(
              (glyph.x_advance_pt - expected).abs() < 0.0003,
              "{family} {size} {ch:?}: {} != {expected}",
              glyph.x_advance_pt
            );
            assert_eq!(&text[glyph.text_range.clone()], ch.to_string());
          }
          style.wordprocessingml_font_slots = false;
          let generic = shape_text_runs(&text, &style).expect("generic fractional space");
          if matches!(family, "MS Mincho" | "MS Gothic") {
            assert!(
              generic
                .iter()
                .flat_map(|run| run.glyphs.iter())
                .all(|glyph| {
                  glyph.source_char != Some(ch) || (glyph.x_advance_pt - size * 0.5).abs() < 0.001
                })
            );
          }
        }
      }
    }
  }

  #[test]
  fn word_literal_separators_keep_primary_face_and_native_blank_width() {
    for (family, em_width) in [("Calibri", 0.5), ("Arial", 0.5), ("Times New Roman", 1.0)] {
      for size in [8.0, 11.0, 16.0, 22.0] {
        for text in ["\u{2028}", "\u{2029}"] {
          let style = TextStyle {
            font_family: Some(Arc::from(family)),
            high_ansi_font_family: Some(Arc::from(family)),
            font_size_pt: size,
            wordprocessingml_font_slots: true,
            ..Default::default()
          };
          let primary = load_text_face(&style).expect("primary Office font");
          let runs = shape_text_runs(text, &style).expect("literal separator");
          assert_eq!(runs.len(), 1);
          assert_eq!(runs[0].font_id.0.as_ref(), primary.id());
          assert!((runs[0].advance_pt - size * em_width).abs() < 0.01);
          assert_eq!(runs[0].glyphs[0].source_char, text.chars().next());
          assert_eq!(runs[0].glyphs[0].text_range, 0..text.len());
          if family != "Times New Roman" {
            assert!(
              runs[0].glyphs[0].bounds.is_none_or(|bounds| {
                bounds.x_min_pt == bounds.x_max_pt || bounds.y_min_pt == bounds.y_max_pt
              }),
              "{family} {size} {text:?}: {:?}",
              runs[0].glyphs[0]
            );
          } else {
            // This face contains visible LS/PS control glyphs; native Word
            // retains both their outlines and their full-em advances.
            assert!(runs[0].glyphs[0].bounds.is_some());
          }
          let mut generic = style.clone();
          generic.wordprocessingml_font_slots = false;
          let generic_runs = shape_text_runs(text, &generic).expect("generic separator");
          if family != "Times New Roman" {
            assert_ne!(generic_runs[0].font_id, runs[0].font_id);
          }
        }
      }
    }
  }

  #[test]
  fn missing_named_font_uses_system_fallback() {
    let style = TextStyle {
      font_family: Some(Arc::from("CodexDefinitelyMissingFont")),
      ..Default::default()
    };

    assert!(load_text_face(&style).is_some());
  }

  #[test]
  fn wordprocessingml_unknown_latin_font_uses_office_fallback() {
    let style = TextStyle {
      font_family: Some(Arc::from("CodexDefinitelyMissingWordFont")),
      high_ansi_font_family: Some(Arc::from("CodexDefinitelyMissingWordFont")),
      east_asia_font_family: Some(Arc::from("CodexDefinitelyMissingEastAsiaFont")),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };

    let face = load_text_face(&style).expect("Word missing-family fallback");
    assert!(
      face.id().to_ascii_lowercase().contains("cambria"),
      "unexpected fallback {}",
      face.id()
    );
    assert_eq!(
      wordprocessingml_missing_family_fallback_for_slot(
        &style,
        Some(TextScript::Latin),
        Some(WordprocessingFontSlot::Ascii),
      ),
      Some("Cambria")
    );
    assert_eq!(
      wordprocessingml_missing_family_fallback_for_slot(
        &style,
        Some(TextScript::Han),
        Some(WordprocessingFontSlot::EastAsia),
      ),
      None
    );
  }

  #[test]
  fn din_bold_uses_system_fallback_when_family_is_not_installed() {
    let style = TextStyle {
      font_family: Some(Arc::from("DIN-Bold")),
      ..Default::default()
    };

    assert!(load_text_face(&style).is_some());
  }

  #[test]
  fn document_fallback_precedes_generic_system_fallback() {
    let style = TextStyle {
      font_family: Some(Arc::from("CodexDefinitelyMissingFont")),
      fallback_font_family: Some(Arc::from("DejaVu Serif")),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };

    let face = load_text_face(&style).expect("document fallback font");
    assert!(
      face.id().to_ascii_lowercase().contains("dejavuserif"),
      "unexpected fallback {}",
      face.id()
    );
  }
}
