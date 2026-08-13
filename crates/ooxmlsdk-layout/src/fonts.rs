use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use ooxmlsdk_fonts::{
  FeatureValue, FontBytes, FontCharset, FontFallbackChain, FontFamilyClass, FontId, FontRegistry,
  FontRequest, FontSize, ResolvedFontChain, ScriptScanOptions, ShapeOptions, ShapedRun,
  TextDirection, TextScript, WordprocessingFontSlot, script_direction_runs_with_options,
};
use rustc_hash::FxHashMap as HashMap;

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
  fn wordprocessingml_font_slots(&self) -> bool {
    false
  }
  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    None
  }
  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    false
  }
  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    None
  }
  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    0.0
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

  fn wordprocessingml_font_slots(&self) -> bool {
    (**self).wordprocessingml_font_slots()
  }

  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    (**self).wordprocessingml_font_hint()
  }

  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    (**self).wordprocessingml_east_asia_language_is_chinese()
  }

  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    (**self).wordprocessingml_east_asia_font_charset()
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    (**self).cjk_punctuation_compression_ratio()
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

fn script_scan_options(
  style: &(impl FontStyleRef + ?Sized),
  small_caps: bool,
) -> ScriptScanOptions {
  ScriptScanOptions {
    small_caps,
    wordprocessingml_font_slots: style.wordprocessingml_font_slots(),
    wordprocessingml_font_hint: style.wordprocessingml_font_hint(),
    wordprocessingml_east_asia_language_is_chinese: style
      .wordprocessingml_east_asia_language_is_chinese(),
    wordprocessingml_east_asia_font_charset: style.wordprocessingml_east_asia_font_charset(),
    wordprocessingml_complex_font_override: style.complex_script_override() == Some(true),
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

  fn right_to_left(&self) -> bool {
    self.right_to_left == Some(true)
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
    self
      .kerning_minimum_size_pt
      .is_none_or(|minimum| effective_font_size_pt(self, None) + f32::EPSILON >= minimum)
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

  fn wordprocessingml_font_slots(&self) -> bool {
    self.wordprocessingml_font_slots
  }

  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    self.wordprocessingml_font_hint
  }

  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    self
      .east_asia_language
      .as_deref()
      .and_then(|language| language.split(['-', '_']).next())
      .is_some_and(|language| language.eq_ignore_ascii_case("zh"))
  }

  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    self.east_asia_font_charset
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    self.cjk_punctuation_compression_ratio
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

  fn right_to_left(&self) -> bool {
    self.right_to_left == Some(true)
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
    self
      .kerning_minimum_size
      .is_none_or(|minimum| effective_font_size_pt(self, None) + f32::EPSILON >= minimum.0)
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

  fn wordprocessingml_font_slots(&self) -> bool {
    self.wordprocessingml_font_slots
  }

  fn wordprocessingml_font_hint(&self) -> Option<ooxmlsdk_fonts::WordprocessingFontTypeHint> {
    self.wordprocessingml_font_hint
  }

  fn wordprocessingml_east_asia_language_is_chinese(&self) -> bool {
    self.wordprocessingml_east_asia_language_is_chinese
  }

  fn wordprocessingml_east_asia_font_charset(&self) -> Option<ooxmlsdk_fonts::FontCharset> {
    self.wordprocessingml_east_asia_font_charset
  }

  fn cjk_punctuation_compression_ratio(&self) -> f32 {
    self.cjk_punctuation_compression_ratio
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
    '\u{3001}' | '\u{3002}' | '\u{FF0C}' | '\u{FF0E}' | '\u{FF1A}' | '\u{FF1B}' => Some(Middle),
    _ => None,
  }
}

fn apply_wordprocessingml_punctuation_compression(run: &mut ShapedRun<'_, '_>, ratio: f32) {
  let ratio = ratio.clamp(0.0, 1.0);
  if ratio <= f32::EPSILON {
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
    // ECMA-376 Part 1 §17.15.1.18 limits this setting to full-width
    // punctuation. A full-width punctuation cell has at most one half-em of
    // removable side-bearing; the line formatter below returns whatever
    // fraction is not needed for the selected break.
    let reduction = glyph.x_advance_pt * 0.5 * ratio;
    glyph.x_advance_pt -= reduction;
    match side {
      FullWidthPunctuationSide::Left => {}
      FullWidthPunctuationSide::Right => glyph.x_offset_pt -= reduction,
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
    let previous_matches = inside_cjk_script
      || (local_start > 0 && run.text[..local_start].chars().next_back() == Some(' '));
    let next_matches = inside_cjk_script
      || (local_end < run.text.len() && run.text[local_end..].chars().next() == Some(' '));
    if !previous_matches && !next_matches {
      continue;
    }

    total_adjustment += target_space_advance - glyph.x_advance_pt;
    glyph.x_advance_pt = target_space_advance;
  }
  run.advance_pt += total_adjustment;
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
  pub fn load_text_face(&mut self, style: &(impl FontStyleRef + ?Sized)) -> Option<FontFaceData> {
    let request = font_request(style, None);
    let registry = self.style_font_registry(style, None);
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

  pub(crate) fn max_text_line_height(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<f32> {
    self
      .text_vertical_metrics(text, style)
      .map(|metrics| metrics.ascent_pt + metrics.descent_pt + metrics.line_gap_pt)
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
    for run in script_runs {
      let metrics_slot =
        wordprocessing_line_metrics_font_slot(style, run.wordprocessingml_font_slot);
      let metrics = self
        .font_metrics_for_slot(style, Some(run.script), metrics_slot)?
        .vertical;
      if let Some(combined) = &mut combined {
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
      } else {
        combined = Some(metrics);
      }
    }
    combined
  }

  fn font_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
  ) -> Option<FontMetrics> {
    self.font_metrics_for_slot(style, script, None)
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
    let request = font_request_for_slot(style, script, wordprocessingml_font_slot);
    let registry = self.style_font_registry_for_slot(style, script, wordprocessingml_font_slot);
    let resolved = registry.resolve(&request).ok()?;
    let metrics_at_size = resolved.metrics_at_size(FontSize(effective_font_size_pt(style, script)));
    let metrics = FontMetrics {
      vertical: metrics_at_size.vertical,
      decoration: metrics_at_size.decoration,
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
    let script_runs = script_direction_runs_with_options(
      text,
      FontSize(base_size),
      script_scan_options(style, style.small_caps()),
    );
    let mut output = Vec::with_capacity(script_runs.len());
    for script_run in script_runs {
      let slot = script_run.wordprocessingml_font_slot;
      let key = FontFaceKey::from_style_for_slot(style, Some(script_run.script), slot);
      let registry = self.style_font_registry_for_slot(style, Some(script_run.script), slot);
      let mut request = font_request_for_slot(style, Some(script_run.script), slot);
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
      options.character_spacing_pt = style.character_spacing_pt();
      options.horizontal_scale = style.horizontal_scale();
      options.small_caps = script_run.small_caps;
      options.scan_registered_fallbacks = false;
      let segment_text = &text[script_run.text_range.clone()];
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
        apply_wordprocessingml_punctuation_compression(
          run,
          style.cjk_punctuation_compression_ratio(),
        );
        run.offset_text_range(script_run.text_range.start);
      }
      output.extend(runs);
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
    value: u32::from(style.kerning_enabled()),
  }];
  if let Some(ligatures) = style.ligatures() {
    // [MS-DOCX] 2.3.32 maps the four Word ligature categories to the
    // corresponding OpenType feature tags defined by ISO/IEC 14496-22.
    features.extend([
      FeatureValue {
        tag: Cow::Borrowed("liga"),
        value: u32::from(ligatures.standard),
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
      value: u32::from(enabled),
    });
  } else if style.wordprocessingml_font_slots() {
    // [MS-DOCX] cntxtAlts specifies that contextual alternates are disabled
    // when w14:cntxtAlts is absent. HarfBuzz enables `calt` by default, so
    // WordprocessingML requests must carry the explicit zero.
    features.push(FeatureValue {
      tag: Cow::Borrowed("calt"),
      value: 0,
    });
  }
  FontRequest {
    family: script_font_family_for_slot(style, script, wordprocessingml_font_slot)
      .filter(|family| !family.trim().is_empty())
      .map(Cow::Borrowed),
    bold: effective_bold(style, script),
    italic: effective_italic(style, script),
    size_pt: FontSize(effective_font_size_pt(style, script)),
    script,
    family_class: script_font_family_class_for_slot(style, script, wordprocessingml_font_slot),
    charset: symbol_charset_for_slot(style, script, wordprocessingml_font_slot),
    features,
    ..FontRequest::default()
  }
}

fn script_font_family_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> Option<&str> {
  // Word fixed output keeps U+0030..U+0039 on the ASCII rFonts family even
  // when w:cs/w:rtl selects complex-script formatting for the run. The font
  // scanner emits Ascii under that narrow exception; let it precede the run
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
  if let Some(slot) = wordprocessingml_font_slot {
    return match slot {
      WordprocessingFontSlot::Ascii => style.font_family(),
      WordprocessingFontSlot::HighAnsi => style.high_ansi_font_family(),
      WordprocessingFontSlot::EastAsia => style.east_asia_font_family(),
      WordprocessingFontSlot::ComplexScript => style.complex_font_family(),
    };
  }
  match script {
    Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul) => {
      style.east_asia_font_family()
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
    _ => None,
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
    return (!force_complex)
      .then(|| style.font_family_class())
      .flatten();
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

fn build_style_font_registry_for_slot(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
) -> FontRegistry<'static> {
  font_timing("build style font registry", || {
    let mut request = font_request_for_slot(style, script, wordprocessingml_font_slot);
    request.script = script;
    let mut registry = FontRegistry::with_default_policy();
    if let Some(requested_family) = request.family.as_deref() {
      let mut families: Vec<Cow<'static, str>> = Vec::new();
      if let Some(fallback_family) =
        script_fallback_font_family_for_slot(style, script, wordprocessingml_font_slot)
        && !requested_family.eq_ignore_ascii_case(fallback_family)
      {
        families.push(Cow::Owned(fallback_family.to_string()));
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
            requested_family: Some(Cow::Owned(requested_family.to_string())),
            script,
            language: None,
            families,
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
  fallback_family: Option<String>,
  family_class: Option<FontFamilyClass>,
  charset: Option<FontCharset>,
  bold: bool,
  italic: bool,
  script: Option<TextScript>,
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
      fallback_family: script_fallback_font_family_for_slot(
        style,
        script,
        wordprocessingml_font_slot,
      )
      .map(str::to_string),
      family_class: script_font_family_class_for_slot(style, script, wordprocessingml_font_slot),
      charset: symbol_charset_for_slot(style, script, wordprocessingml_font_slot),
      bold: effective_bold(style, script),
      italic: effective_italic(style, script),
      script,
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
      && self.fallback_family.as_deref()
        == script_fallback_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.family_class
        == script_font_family_class_for_slot(style, script, wordprocessingml_font_slot)
      && self.charset == symbol_charset_for_slot(style, script, wordprocessingml_font_slot)
      && self.bold == effective_bold(style, script)
      && self.italic == effective_italic(style, script)
      && self.script == script
  }
}

#[derive(Clone, Copy, Debug)]
struct FontMetrics {
  vertical: ooxmlsdk_fonts::VerticalMetrics,
  decoration: ooxmlsdk_fonts::DecorationMetrics,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontMetricsKey {
  family: Option<String>,
  fallback_family: Option<String>,
  family_class: Option<FontFamilyClass>,
  charset: Option<FontCharset>,
  bold: bool,
  italic: bool,
  script: Option<TextScript>,
  size_pt_bits: u32,
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
      fallback_family: script_fallback_font_family_for_slot(
        style,
        script,
        wordprocessingml_font_slot,
      )
      .map(str::to_string),
      family_class: script_font_family_class_for_slot(style, script, wordprocessingml_font_slot),
      charset: symbol_charset_for_slot(style, script, wordprocessingml_font_slot),
      bold: effective_bold(style, script),
      italic: effective_italic(style, script),
      script,
      size_pt_bits: effective_font_size_pt(style, script).to_bits(),
    }
  }

  fn matches_style_for_slot(
    &self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
    wordprocessingml_font_slot: Option<WordprocessingFontSlot>,
  ) -> bool {
    self.family.as_deref() == script_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.fallback_family.as_deref()
        == script_fallback_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.family_class
        == script_font_family_class_for_slot(style, script, wordprocessingml_font_slot)
      && self.charset == symbol_charset_for_slot(style, script, wordprocessingml_font_slot)
      && self.bold == effective_bold(style, script)
      && self.italic == effective_italic(style, script)
      && self.script == script
      && self.size_pt_bits == effective_font_size_pt(style, script).to_bits()
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
  };

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
  fn wordprocessing_rtl_digits_keep_ascii_family_and_complex_run_properties() {
    let text = "1A";
    let style = TextStyle {
      font_family: Some(Arc::from("Latin Face")),
      complex_font_family: Some(Arc::from("Complex Face")),
      font_size_pt: 10.0,
      complex_font_size_pt: Some(20.0),
      right_to_left: Some(true),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };

    let runs = script_direction_runs_with_options(
      text,
      FontSize(style.font_size_pt),
      script_scan_options(&style, false),
    );
    assert_eq!(runs.len(), 2);
    assert_eq!(&text[runs[0].text_range.clone()], "1");
    assert_eq!(
      runs[0].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::Ascii)
    );
    let digit = font_request_for_slot(
      &style,
      Some(runs[0].script),
      runs[0].wordprocessingml_font_slot,
    );
    assert_eq!(digit.family.as_deref(), Some("Latin Face"));
    assert_eq!(digit.size_pt.0, 20.0);
    assert_eq!(
      wordprocessing_line_metrics_font_slot(&style, runs[0].wordprocessingml_font_slot),
      Some(WordprocessingFontSlot::ComplexScript)
    );

    assert_eq!(&text[runs[1].text_range.clone()], "A");
    assert_eq!(
      runs[1].wordprocessingml_font_slot,
      Some(WordprocessingFontSlot::ComplexScript)
    );
    let letter = font_request_for_slot(
      &style,
      Some(runs[1].script),
      runs[1].wordprocessingml_font_slot,
    );
    assert_eq!(letter.family.as_deref(), Some("Complex Face"));
    assert_eq!(letter.size_pt.0, 20.0);

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
  fn missing_named_font_uses_system_fallback() {
    let style = TextStyle {
      font_family: Some(Arc::from("CodexDefinitelyMissingFont")),
      ..Default::default()
    };

    assert!(load_text_face(&style).is_some());
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
