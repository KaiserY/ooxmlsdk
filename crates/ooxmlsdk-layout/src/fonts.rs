use std::borrow::Cow;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use ooxmlsdk_fonts::{
  FeatureValue, FontBytes, FontCharset, FontFallbackChain, FontFamilyClass, FontFamilySelection,
  FontId, FontRegistry, FontRequest, FontScriptRun, FontSize, FontSlant, FontSource,
  ResolvedFontChain, ScriptScanOptions, ShapeOptions, ShapedRun, TextDirection, TextScript,
  WordprocessingFontSlot, script_direction_runs_with_options,
};
use rustc_hash::FxHashMap as HashMap;
use skrifa::{
  MetadataProvider,
  raw::{FontRef, TableProvider},
};

use crate::common;
use crate::docx::TextStyle;

const WORD_ENCLOSED_NUMBER_LINK_RANGE: std::ops::Range<u32> = 0x2474..0x249c;

/// Private fonts belonging to one document. These resources travel with text
/// styles so fresh measurement and PDF resolvers see the same font collection.
#[derive(Debug)]
pub struct EmbeddedFontSet {
  fingerprint: u64,
  fonts: Vec<EmbeddedFont>,
  registry: OnceLock<FontRegistry<'static>>,
}

impl PartialEq for EmbeddedFontSet {
  fn eq(&self, other: &Self) -> bool {
    self.fingerprint == other.fingerprint && self.fonts == other.fonts
  }
}

impl Eq for EmbeddedFontSet {}

#[derive(Debug, Eq, PartialEq)]
struct EmbeddedFont {
  id: Arc<str>,
  data: FontBytes,
}

impl Hash for EmbeddedFontSet {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.fingerprint.hash(state);
  }
}

impl EmbeddedFontSet {
  pub(crate) fn new(sources: Vec<(String, Vec<u8>)>) -> Option<Arc<Self>> {
    let fonts = sources
      .into_iter()
      .filter_map(|(name, data)| {
        FontRef::from_index(&data, 0).ok()?;
        let mut hash = DefaultHasher::new();
        name.hash(&mut hash);
        data.hash(&mut hash);
        Some(EmbeddedFont {
          id: Arc::from(format!("embedded-ooxml:{:016x}:{name}", hash.finish())),
          data: data.into(),
        })
      })
      .collect::<Vec<_>>();
    if fonts.is_empty() {
      return None;
    }
    let mut hash = DefaultHasher::new();
    for font in &fonts {
      font.id.hash(&mut hash);
    }
    Some(Arc::new(Self {
      fingerprint: hash.finish(),
      fonts,
      registry: OnceLock::new(),
    }))
  }

  fn registry(&self) -> &FontRegistry<'static> {
    self.registry.get_or_init(|| {
      let mut registry = FontRegistry::with_default_policy();
      for font in &self.fonts {
        let _ = registry.register_embedded_font(font.id.to_string(), font.data.clone());
      }
      registry
    })
  }
}

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
  pub document_embedded: bool,
  pub named_instance: Option<Arc<ooxmlsdk_fonts::NamedFontInstance>>,
  fixed_halfwidth_cells: bool,
  id: Arc<str>,
}

impl FontFaceData {
  /// Source pitch weights for the supported Word single/double-byte cells.
  /// Han/fullwidth controls are independent of the face's proportional Latin
  /// metrics. Other wide cells are admitted on fixed halfwidth faces: native
  /// proportional kana/Hangul opposites rule out a blanket EAW-wide doubling.
  pub fn wordprocessingml_balanced_pitch_units(&self, character: char) -> i32 {
    use icu_properties::{CodePointMapData, props::EastAsianWidth};
    if self.fixed_halfwidth_cells
      && CodePointMapData::<EastAsianWidth>::new().get(character) == EastAsianWidth::Ambiguous
      && let Ok(font) = FontRef::from_index(self.data.as_slice(), self.index)
      && let Some(glyph) = font.charmap().map(character)
      && let Ok(head) = font.head()
      && font.hmtx().ok().and_then(|table| table.advance(glyph)) == Some(head.units_per_em())
    {
      // Native SimSun/Mincho quotes and Greek cells carry two pitches even
      // through hAnsi. Their halfwidth bullets/accented letters, and narrow
      // proportional-face quotes, retain one. EAW Ambiguous alone is not a
      // width classification. Query only these tracked, fixed-face cells.
      return 2;
    }
    wordprocessingml_balanced_pitch_units(character, self.fixed_halfwidth_cells)
  }

  pub fn location(&self) -> skrifa::instance::LocationRef<'_> {
    self
      .named_instance
      .as_ref()
      .map_or_else(skrifa::instance::LocationRef::default, |instance| {
        (&instance.location).into()
      })
  }

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
  fn embedded_fonts(&self) -> Option<&Arc<EmbeddedFontSet>> {
    None
  }
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
  fn wordprocessing_simple_latin_glyphs(&self) -> Option<bool> {
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
  /// A logical reference-font metric query has no glyph to remap to CJK.
  fn wordprocessingml_reference_font(&self) -> bool {
    false
  }
  fn wordprocessingml_complex_numbering(&self) -> bool {
    false
  }

  fn wordprocessingml_no_break_hyphen(&self) -> bool {
    false
  }
  fn wordprocessingml_literal_soft_hyphen(&self) -> bool {
    false
  }
  fn wordprocessingml_han_fallback(&self) -> bool {
    false
  }
  fn wordprocessingml_symbol_fallback(&self) -> bool {
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
  fn wordprocessing_emphasis(&self) -> Option<common::WordprocessingEmphasis> {
    None
  }
  fn wordprocessingml_no_leading(&self) -> bool {
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
  fn wordprocessingml_bidi_language(&self) -> Option<&str> {
    None
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
  fn cjk_bracket_compression_ratio(&self) -> f32 {
    self.cjk_punctuation_compression_ratio()
  }
  fn cjk_quotation_compression_ratio(&self) -> f32 {
    0.0
  }
  fn wordprocessing_terminal_punctuation(&self) -> Option<(usize, f32)> {
    None
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    false
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    false
  }
  fn wordprocessingml_bracket_spacing(&self) -> bool {
    self.wordprocessingml_punctuation_spacing()
  }
  fn wordprocessingml_vertical_punctuation(&self) -> bool {
    false
  }
  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    false
  }

  fn wordprocessingml_balanced_space_portion(&self) -> bool {
    false
  }

  fn wordprocessingml_balance_reference_font_family(&self) -> Option<&str> {
    None
  }
}

impl<T: FontStyleRef + ?Sized> FontStyleRef for Box<T> {
  fn embedded_fonts(&self) -> Option<&Arc<EmbeddedFontSet>> {
    (**self).embedded_fonts()
  }
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

  fn wordprocessing_simple_latin_glyphs(&self) -> Option<bool> {
    (**self).wordprocessing_simple_latin_glyphs()
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
  fn wordprocessingml_reference_font(&self) -> bool {
    (**self).wordprocessingml_reference_font()
  }
  fn wordprocessingml_complex_numbering(&self) -> bool {
    (**self).wordprocessingml_complex_numbering()
  }

  fn wordprocessingml_no_break_hyphen(&self) -> bool {
    (**self).wordprocessingml_no_break_hyphen()
  }
  fn wordprocessingml_literal_soft_hyphen(&self) -> bool {
    (**self).wordprocessingml_literal_soft_hyphen()
  }
  fn wordprocessingml_han_fallback(&self) -> bool {
    (**self).wordprocessingml_han_fallback()
  }
  fn wordprocessingml_symbol_fallback(&self) -> bool {
    (**self).wordprocessingml_symbol_fallback()
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
  fn wordprocessing_emphasis(&self) -> Option<common::WordprocessingEmphasis> {
    (**self).wordprocessing_emphasis()
  }
  fn wordprocessingml_no_leading(&self) -> bool {
    (**self).wordprocessingml_no_leading()
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
  fn wordprocessingml_bidi_language(&self) -> Option<&str> {
    (**self).wordprocessingml_bidi_language()
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
  fn cjk_bracket_compression_ratio(&self) -> f32 {
    (**self).cjk_bracket_compression_ratio()
  }
  fn cjk_quotation_compression_ratio(&self) -> f32 {
    (**self).cjk_quotation_compression_ratio()
  }
  fn wordprocessing_terminal_punctuation(&self) -> Option<(usize, f32)> {
    (**self).wordprocessing_terminal_punctuation()
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    (**self).wordprocessingml_legacy_punctuation_spacing()
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    (**self).wordprocessingml_punctuation_spacing()
  }
  fn wordprocessingml_bracket_spacing(&self) -> bool {
    (**self).wordprocessingml_bracket_spacing()
  }
  fn wordprocessingml_vertical_punctuation(&self) -> bool {
    (**self).wordprocessingml_vertical_punctuation()
  }

  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    (**self).wordprocessingml_balance_single_byte_double_byte_width()
  }

  fn wordprocessingml_balanced_space_portion(&self) -> bool {
    (**self).wordprocessingml_balanced_space_portion()
  }

  fn wordprocessingml_balance_reference_font_family(&self) -> Option<&str> {
    (**self).wordprocessingml_balance_reference_font_family()
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

/// Borrowed physical reference for balanced spaces. The painted run keeps its
/// own slots and shaping cache; this view only enters the existing metric cache.
struct WordSpaceReferenceStyle<'a, S: ?Sized> {
  style: &'a S,
  family: &'a str,
  size_pt: f32,
  source_east_asia: bool,
}

impl<'a, S: FontStyleRef + ?Sized> WordSpaceReferenceStyle<'a, S> {
  fn new(style: &'a S, family: &'a str, size_pt: f32) -> Self {
    Self {
      style,
      family,
      size_pt,
      source_east_asia: style
        .east_asia_font_family()
        .is_some_and(|source| source.eq_ignore_ascii_case(family)),
    }
  }
}

impl<S: FontStyleRef + ?Sized> FontStyleRef for WordSpaceReferenceStyle<'_, S> {
  fn wordprocessingml_reference_font(&self) -> bool {
    true
  }
  fn wordprocessingml_bidi_language(&self) -> Option<&str> {
    self.style.wordprocessingml_bidi_language()
  }
  fn embedded_fonts(&self) -> Option<&Arc<EmbeddedFontSet>> {
    self.style.embedded_fonts()
  }
  fn font_family(&self) -> Option<&str> {
    Some(self.family)
  }
  fn font_size_pt(&self) -> f32 {
    self.size_pt
  }
  fn character_spacing_pt(&self) -> f32 {
    0.0
  }
  fn baseline_shift_pt(&self) -> f32 {
    0.0
  }
  fn bold(&self) -> bool {
    effective_bold(self.style, None)
  }
  fn italic(&self) -> bool {
    effective_italic(self.style, None)
  }
  fn small_caps(&self) -> bool {
    false
  }
  fn wordprocessingml_font_slots(&self) -> bool {
    true
  }
  fn east_asia_fallback_font_family(&self) -> Option<&str> {
    self
      .source_east_asia
      .then(|| self.style.east_asia_fallback_font_family())
      .flatten()
  }
  fn east_asia_font_family_class(&self) -> Option<FontFamilyClass> {
    self
      .source_east_asia
      .then(|| self.style.east_asia_font_family_class())
      .flatten()
  }
  fn wordprocessingml_east_asia_font_charset(&self) -> Option<FontCharset> {
    self
      .source_east_asia
      .then(|| self.style.wordprocessingml_east_asia_font_charset())
      .flatten()
  }
  fn east_asia_font_pitch(&self) -> Option<ooxmlsdk_fonts::FontPitch> {
    self
      .source_east_asia
      .then(|| self.style.east_asia_font_pitch())
      .flatten()
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
    wordprocessingml_complex_numbering: style.wordprocessingml_complex_numbering(),
    wordprocessingml_no_break_hyphen: style.wordprocessingml_no_break_hyphen(),
    wordprocessingml_font_hint: style.wordprocessingml_font_hint(),
    wordprocessingml_resolved_font_slot: style.wordprocessingml_resolved_font_slot(),
    wordprocessingml_east_asia_language_is_chinese: style
      .wordprocessingml_east_asia_language_is_chinese(),
    wordprocessingml_east_asia_font_charset: style.wordprocessingml_east_asia_font_charset(),
    wordprocessingml_complex_font_override: style.complex_script_override() == Some(true)
      && !style.wordprocessingml_complex_numbering(),
    wordprocessingml_rtl_font_override: style.right_to_left()
      && style.complex_script_property() != Some(true)
      && !style.wordprocessingml_complex_numbering(),
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

fn wordprocessingml_realized_line_metrics(
  style: &(impl FontStyleRef + ?Sized),
  slot: Option<WordprocessingFontSlot>,
  text: &str,
) -> bool {
  wordprocessingml_east_asia_font_mapping(style, slot)
    || (style.wordprocessingml_font_slots()
      && style.wordprocessingml_symbol_fallback()
      && text
        .chars()
        .any(|ch| WORD_ENCLOSED_NUMBER_LINK_RANGE.contains(&u32::from(ch))))
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
  script: Option<TextScript>,
) -> f32 {
  let complex_size = if style.wordprocessingml_complex_numbering() {
    matches!(script, Some(TextScript::Hebrew | TextScript::Arabic))
  } else {
    uses_complex_run_properties(style)
  };
  if complex_size {
    style.complex_font_size_pt().unwrap_or(style.font_size_pt())
  } else {
    style.font_size_pt()
  }
}

pub(crate) fn wordprocessingml_character_indent_unit_pt(
  style: &(impl FontStyleRef + ?Sized),
) -> f32 {
  // Native firstLineChars controls keep authored pitch outside horizontal
  // scaling. The virtual fullwidth character receives double pitch under
  // balanceSingleByteDoubleByteWidth, independently of the first glyph.
  let pitch_units = if style.wordprocessingml_balance_single_byte_double_byte_width() {
    2.0
  } else {
    1.0
  };
  effective_font_size_pt(style, None) * style.horizontal_scale()
    + pitch_units * style.character_spacing_pt()
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

fn wordprocessingml_arabic_joining_break(character: char) -> bool {
  matches!(character, '\u{200b}' | '\u{200e}' | '\u{200f}' | '\u{2060}')
}

fn wordprocessingml_arabic_shaping_runs(
  text: &str,
  runs: Vec<FontScriptRun>,
  separate_words: bool,
  separate_simple_latin: bool,
) -> Vec<FontScriptRun> {
  let simple_latin = |character: char| {
    separate_simple_latin && character.is_ascii() && !character.is_ascii_control()
  };
  let boundary = |character| {
    wordprocessingml_arabic_joining_break(character)
      || (separate_words && character == ' ')
      || simple_latin(character)
  };
  if !runs.iter().any(|run| {
    run.script == TextScript::Arabic && text[run.text_range.clone()].chars().any(boundary)
  }) {
    return runs;
  }
  // Word's RLM/LRM, zero-width-space and word-joiner controls terminate
  // Arabic joining, even when they retain the surrounding bidi level.
  // Isolated three-font Office controls match ZWNJ on both sides; ZWJ and
  // an ordinary covered ALM retain their independent shaping behavior.
  // An ordinary, non-complex Word run additionally shapes Arabic words and
  // SP separately. Its ASCII punctuation and adjacent blanks also use the
  // simple Latin glyph path, even if script scanning merged them with Arabic.
  // Native calls retain kerning inside each word, but not across its blank.
  // Explicit cs/rtl and authored bidi controls retain whole-portion GPOS.
  // This boundary owns both fitting and paint; applying it only during PDF
  // export leaves whole words incorrectly admitted to the preceding line.
  // Keep every source byte/range, and let the shaper hide each control.
  let mut separated = Vec::with_capacity(runs.len() + 2);
  for run in runs {
    if run.script != TextScript::Arabic {
      separated.push(run);
      continue;
    }
    let mut start = run.text_range.start;
    let mut latin = false;
    let mut append = |range, latin| {
      let mut part = run.clone();
      part.text_range = range;
      if latin {
        part.script = TextScript::Latin;
        part.direction = TextDirection::LeftToRight;
      }
      separated.push(part);
    };
    for (offset, character) in text[run.text_range.clone()].char_indices() {
      let at = run.text_range.start + offset;
      let current_latin = simple_latin(character);
      let isolated = wordprocessingml_arabic_joining_break(character)
        || (separate_words && character == ' ' && !current_latin);
      if (isolated || current_latin != latin) && start < at {
        append(start..at, latin);
        start = at;
      }
      latin = current_latin;
      if isolated {
        let end = at + character.len_utf8();
        append(at..end, false);
        start = end;
      }
    }
    if start < run.text_range.end {
      append(start..run.text_range.end, latin);
    }
  }
  separated
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
  fn wordprocessingml_bidi_language(&self) -> Option<&str> {
    self.bidi_language.as_deref()
  }
  fn embedded_fonts(&self) -> Option<&Arc<EmbeddedFontSet>> {
    self.embedded_fonts.as_ref()
  }
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

  fn wordprocessing_simple_latin_glyphs(&self) -> Option<bool> {
    self
      .wordprocessing_simple_latin_glyphs
      .or_else(|| self.wordprocessing_bidi_scopes.as_ref().map(|_| false))
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
  fn wordprocessingml_complex_numbering(&self) -> bool {
    self.wordprocessingml_complex_numbering
  }

  fn wordprocessingml_no_break_hyphen(&self) -> bool {
    self.wordprocessingml_no_break_hyphen
  }
  fn wordprocessingml_literal_soft_hyphen(&self) -> bool {
    self.wordprocessingml_literal_soft_hyphen
  }
  fn wordprocessingml_han_fallback(&self) -> bool {
    self.wordprocessingml_han_fallback
  }
  fn wordprocessingml_symbol_fallback(&self) -> bool {
    self.wordprocessingml_symbol_fallback
  }

  fn wordprocessingml_form_text_blank_cell(&self) -> bool {
    self.wordprocessingml_form_text_blank_cell
  }

  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    self.wordprocessingml_cjk_line_metrics
  }
  fn wordprocessing_emphasis(&self) -> Option<common::WordprocessingEmphasis> {
    self.resolved_wordprocessing_emphasis()
  }
  fn wordprocessingml_no_leading(&self) -> bool {
    self.wordprocessingml_no_leading
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
    self.effective_cjk_punctuation_compression_ratio()
  }
  fn cjk_bracket_compression_ratio(&self) -> f32 {
    self.effective_cjk_bracket_compression_ratio()
  }
  fn cjk_quotation_compression_ratio(&self) -> f32 {
    self.effective_cjk_quotation_compression_ratio()
  }
  fn wordprocessing_terminal_punctuation(&self) -> Option<(usize, f32)> {
    self.wordprocessing_terminal_punctuation
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    self.wordprocessingml_legacy_punctuation_spacing
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    self.effective_wordprocessing_punctuation_spacing()
  }
  fn wordprocessingml_bracket_spacing(&self) -> bool {
    self.effective_wordprocessing_bracket_spacing()
  }
  fn wordprocessingml_vertical_punctuation(&self) -> bool {
    self.wordprocessingml_vertical_punctuation
  }

  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    self.wordprocessingml_balance_single_byte_double_byte_width
  }

  fn wordprocessingml_balanced_space_portion(&self) -> bool {
    self.wordprocessingml_balanced_space_portion
  }

  fn wordprocessingml_balance_reference_font_family(&self) -> Option<&str> {
    self
      .wordprocessingml_balance_single_byte_double_byte_width
      .then_some(
        self
          .wordprocessingml_balance_reference_font_family
          .as_deref(),
      )
      .flatten()
  }
}

impl FontStyleRef for common::TextStyle<'_> {
  fn wordprocessingml_bidi_language(&self) -> Option<&str> {
    self.wordprocessingml_bidi_language.as_deref()
  }
  fn embedded_fonts(&self) -> Option<&Arc<EmbeddedFontSet>> {
    self.embedded_fonts.as_ref()
  }
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

  fn wordprocessing_simple_latin_glyphs(&self) -> Option<bool> {
    self.wordprocessing_simple_latin_glyphs
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
  fn wordprocessingml_complex_numbering(&self) -> bool {
    self.wordprocessingml_complex_numbering
  }

  fn wordprocessingml_no_break_hyphen(&self) -> bool {
    self.wordprocessingml_no_break_hyphen
  }
  fn wordprocessingml_literal_soft_hyphen(&self) -> bool {
    self.wordprocessingml_literal_soft_hyphen
  }
  fn wordprocessingml_han_fallback(&self) -> bool {
    self.wordprocessingml_han_fallback
  }
  fn wordprocessingml_symbol_fallback(&self) -> bool {
    self.wordprocessingml_symbol_fallback
  }

  fn wordprocessingml_cjk_line_metrics(&self) -> bool {
    self.wordprocessingml_cjk_line_metrics
  }
  fn wordprocessing_emphasis(&self) -> Option<common::WordprocessingEmphasis> {
    self.wordprocessing_emphasis
  }
  fn wordprocessingml_no_leading(&self) -> bool {
    self.wordprocessingml_no_leading
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
  fn cjk_bracket_compression_ratio(&self) -> f32 {
    self
      .cjk_bracket_compression_ratio
      .max(self.cjk_punctuation_compression_ratio)
  }
  fn cjk_quotation_compression_ratio(&self) -> f32 {
    self.cjk_quotation_compression_ratio
  }
  fn wordprocessing_terminal_punctuation(&self) -> Option<(usize, f32)> {
    self.wordprocessing_terminal_punctuation
  }
  fn wordprocessingml_legacy_punctuation_spacing(&self) -> bool {
    self.wordprocessingml_legacy_punctuation_spacing
  }
  fn wordprocessingml_punctuation_spacing(&self) -> bool {
    self.wordprocessingml_punctuation_spacing
  }
  fn wordprocessingml_bracket_spacing(&self) -> bool {
    self.wordprocessingml_bracket_spacing || self.wordprocessingml_punctuation_spacing
  }
  fn wordprocessingml_vertical_punctuation(&self) -> bool {
    self.wordprocessingml_vertical_punctuation
  }

  fn wordprocessingml_balance_single_byte_double_byte_width(&self) -> bool {
    self.wordprocessingml_balance_single_byte_double_byte_width
  }

  fn wordprocessingml_balanced_space_portion(&self) -> bool {
    self.wordprocessingml_balanced_space_portion
  }

  fn wordprocessingml_balance_reference_font_family(&self) -> Option<&str> {
    self
      .wordprocessingml_balance_single_byte_double_byte_width
      .then_some(
        self
          .wordprocessingml_balance_reference_font_family
          .as_deref(),
      )
      .flatten()
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

fn full_width_bracket(ch: char) -> bool {
  full_width_punctuation_side(ch).is_some()
    && !matches!(
      ch,
      '\u{3001}' | '\u{3002}' | '\u{FF0C}' | '\u{FF0E}' | '\u{FF1A}' | '\u{FF1B}'
    )
}

fn curly_quotation_side(ch: char) -> Option<FullWidthPunctuationSide> {
  match ch {
    '\u{2018}' | '\u{201C}' => Some(FullWidthPunctuationSide::Left),
    '\u{2019}' | '\u{201D}' => Some(FullWidthPunctuationSide::Right),
    _ => None,
  }
}

#[derive(Clone, Copy)]
struct WordprocessingPunctuationSpacing {
  ratio: f32,
  bracket_ratio: f32,
  quotation_ratio: f32,
  terminal: Option<(usize, f32)>,
  pair: bool,
  bracket_pair: bool,
  quotation_slot: bool,
  quotation_pair: bool,
  legacy: bool,
}

impl WordprocessingPunctuationSpacing {
  fn new(style: &(impl FontStyleRef + ?Sized)) -> Self {
    let pair = style.wordprocessingml_punctuation_spacing();
    let terminal = style.wordprocessing_terminal_punctuation();
    let quotation_ratio = style.cjk_quotation_compression_ratio().clamp(0.0, 1.0);
    // MS-OI29500 §2.1.88 assigns these ambiguous quotation characters to
    // eastAsia only under its hint/override rules. Native Word keeps hAnsi
    // quotes out of punctuation kerning even when both slots name SimSun.
    // Use the existing scan options without allocating a one-character run.
    let quotation_slot = style.wordprocessingml_font_slots()
      && (pair || quotation_ratio > 0.0 || terminal.is_some_and(|(_, ratio)| ratio > 0.0))
      && {
        let options = script_scan_options(style, false);
        options.wordprocessingml_resolved_font_slot.map_or_else(
          || {
            !options.wordprocessingml_complex_font_override
              && !options.wordprocessingml_rtl_font_override
              && !options.wordprocessingml_east_asia_uses_ascii
              && options.wordprocessingml_font_hint
                == Some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia)
          },
          |slot| slot == WordprocessingFontSlot::EastAsia,
        )
      };
    Self {
      ratio: style.cjk_punctuation_compression_ratio().clamp(0.0, 1.0),
      bracket_ratio: style.cjk_bracket_compression_ratio().clamp(0.0, 1.0),
      quotation_ratio: if quotation_slot { quotation_ratio } else { 0.0 },
      terminal,
      pair,
      bracket_pair: style.wordprocessingml_bracket_spacing(),
      quotation_slot,
      quotation_pair: pair && quotation_slot,
      legacy: style.wordprocessingml_legacy_punctuation_spacing(),
    }
  }

  fn ratio(self, character: char) -> f32 {
    if curly_quotation_side(character).is_some() {
      self.quotation_ratio
    } else if full_width_bracket(character) {
      self.bracket_ratio
    } else {
      self.ratio
    }
  }

  fn compression_side(self, character: char) -> Option<FullWidthPunctuationSide> {
    full_width_punctuation_side(character).or_else(|| {
      self
        .quotation_slot
        .then(|| curly_quotation_side(character))
        .flatten()
    })
  }

  fn pair_side(self, character: char) -> Option<FullWidthPunctuationSide> {
    let enabled = if full_width_bracket(character) {
      self.bracket_pair
    } else {
      self.pair
    };
    enabled
      .then(|| full_width_punctuation_side(character))
      .flatten()
      .or_else(|| {
        self
          .quotation_pair
          .then(|| curly_quotation_side(character))
          .flatten()
      })
  }
}

pub(crate) fn wordprocessingml_punctuation_compression_enabled(
  text: &str,
  style: &(impl FontStyleRef + ?Sized),
) -> bool {
  let spacing = WordprocessingPunctuationSpacing::new(style);
  text.chars().any(|character| {
    spacing.compression_side(character).is_some() && spacing.ratio(character) > 0.0
  })
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
  let spacing = WordprocessingPunctuationSpacing::new(style);
  punctuation_pair_tightens(
    left.and_then(|character| spacing.pair_side(character)),
    right.and_then(|character| spacing.pair_side(character)),
    spacing.legacy,
  )
}

/// Whether Word's formatter changed punctuation advances from the face's
/// ordinary metrics. PDF natural-width balancing must preserve these explicit
/// spacing decisions instead of treating them as device rounding residuals.
pub fn wordprocessingml_punctuation_affects_advances(
  text: &str,
  style: &(impl FontStyleRef + ?Sized),
) -> bool {
  let spacing = WordprocessingPunctuationSpacing::new(style);
  if let Some((index, ratio)) = style.wordprocessing_terminal_punctuation()
    && ratio > 0.0
    && text
      .get(index..)
      .and_then(|suffix| suffix.chars().next())
      .and_then(|character| spacing.compression_side(character))
      .is_some()
  {
    return true;
  }
  if style.wordprocessingml_font_slots()
    && style.wordprocessingml_vertical_punctuation()
    && text.chars().any(word_vertical_quotation)
  {
    return true;
  }
  let mut previous = None;
  for ch in text.chars() {
    let current = spacing.compression_side(ch);
    if current.is_some() && spacing.ratio(ch) > 0.0 {
      return true;
    }
    let current = spacing.pair_side(ch);
    if punctuation_pair_tightens(previous, current, spacing.legacy) {
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
  fn curly_quotation_pressure_follows_native_slot_language_and_full_cell() {
    let mut metrics = crate::text_metrics::TextMetrics::new();
    // Native pressure on/off opposites include same-face hAnsi negatives,
    // four quote directions and narrow physical cells with kerning disabled.
    for family in ["SimSun", "Microsoft YaHei", "Malgun Gothic"] {
      for language in [
        None,
        Some("en-US"),
        Some("zh-CN"),
        Some("zh-TW"),
        Some("ja-JP"),
        Some("ko-KR"),
      ] {
        for hint in [false, true] {
          for same_primary in [false, true] {
            let mut style = TextStyle {
              font_family: Some(if same_primary { family } else { "Calibri" }.into()),
              high_ansi_font_family: Some(if same_primary { family } else { "Calibri" }.into()),
              east_asia_font_family: Some(family.into()),
              east_asia_language: language.map(Into::into),
              font_size_pt: 15.0,
              kerning_minimum_size_pt: Some(500.0),
              wordprocessingml_font_slots: true,
              wordprocessingml_font_hint: hint
                .then_some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia),
              wordprocessingml_punctuation_spacing: true,
              ..Default::default()
            };
            let eligible = hint
              && family != "Malgun Gothic"
              && matches!(language, None | Some("zh-CN" | "ja-JP"));
            for quotation in ['“', '”', '‘', '’'] {
              let text = format!("甲{quotation}乙");
              style.cjk_punctuation_compression_ratio = 0.0;
              let natural = metrics.measure_text(&text, &style);
              style.cjk_punctuation_compression_ratio = 1.0;
              let compressed = metrics.measure_text(&text, &style);
              assert_eq!(
                ((natural - compressed) * 4096.0).round() as i32,
                if eligible { 30720 } else { 0 },
                "{family}/{language:?}/{hint}/{same_primary}/{quotation}"
              );
              let boxed = Box::new(style.clone());
              let common = crate::model::common_text_style(style.clone());
              assert_eq!(metrics.measure_text(&text, &boxed), compressed);
              assert_eq!(metrics.measure_text(&text, &common), compressed);
            }
          }
        }
      }
    }
  }

  #[test]
  fn word_punctuation_compression_follows_inherited_east_asia_language() {
    let mut metrics = crate::text_metrics::TextMetrics::new();
    for family in ["宋体", "ＭＳ 明朝"] {
      let mut style = TextStyle {
        font_family: Some("Calibri".into()),
        east_asia_font_family: Some(family.into()),
        font_size_pt: 10.0,
        wordprocessingml_font_slots: true,
        cjk_punctuation_compression_ratio: 1.0,
        ..Default::default()
      };
      // Alternate languages on the same resolver: cache keys must retain
      // the effective compression, including a later inherited override.
      for (language, expected) in [
        ("en-US", 30.0),
        ("zh-CN", 25.0),
        ("ko-KR", 30.0),
        ("ja-JP", 25.0),
        ("fr-FR", 30.0),
        ("zh-TW", 25.0),
        ("ar-SA", 30.0),
        ("und", 30.0),
      ] {
        style.east_asia_language = Some(language.into());
        let measured = metrics.measure_text("甲，乙", &style);
        assert!(
          (measured - expected).abs() < 0.001,
          "{family}/{language}: {measured} vs {expected}"
        );
        assert_eq!(style.cjk_punctuation_compression_ratio, 1.0);
      }
    }
  }

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
  fn native_pair_spacing_requires_kerning_and_both_language_classes() {
    let mut metrics = crate::text_metrics::TextMetrics::new();
    for family in ["宋体", "ＭＳ 明朝"] {
      for language in ["en-US", "zh-CN", "zh-TW", "ja-JP", "ko-KR"] {
        for minimum in [1.0, 10.0, 10.5, f32::INFINITY] {
          let style = TextStyle {
            font_family: Some("Calibri".into()),
            east_asia_font_family: Some(family.into()),
            east_asia_language: Some(language.into()),
            font_size_pt: 10.0,
            kerning_minimum_size_pt: Some(minimum),
            wordprocessingml_font_slots: true,
            wordprocessingml_punctuation_spacing: true,
            ..Default::default()
          };
          for (text, language_allows) in [
            ("甲》，乙", matches!(language, "zh-CN" | "zh-TW" | "ja-JP")),
            ("甲（（乙", language != "ko-KR"),
            ("甲（）乙", false),
          ] {
            let expected = if minimum <= 10.0 && language_allows {
              35.0
            } else {
              40.0
            };
            assert!(
              (metrics.measure_text(text, &style) - expected).abs() < 0.001,
              "{family}/{language}/{minimum}/{text}"
            );
            let common = crate::model::common_text_style(style.clone());
            assert!((metrics.measure_text(text, &common) - expected).abs() < 0.001);
          }
        }
      }
    }
  }

  #[test]
  fn native_curly_pairs_require_east_asia_cells_and_active_kerning() {
    // Native MSLS vectors: /tmp/ owner table-2006/native-curly-pairs-v5.
    // Modes12/15 agree: 9pt full-cell pairs lose18432 ideal units (4.5pt).
    let mut metrics = crate::text_metrics::TextMetrics::new();
    for family in ["宋体", "ＭＳ 明朝"] {
      for language in ["en-US", "zh-CN", "ja-JP", "ko-KR"] {
        for same_primary in [false, true] {
          for hinted in [false, true] {
            let mut style = TextStyle {
              font_family: Some(if same_primary { family } else { "Calibri" }.into()),
              east_asia_font_family: Some(family.into()),
              east_asia_language: Some(language.into()),
              wordprocessingml_font_hint: hinted
                .then_some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia),
              font_size_pt: 9.0,
              kerning_minimum_size_pt: Some(500.0),
              wordprocessingml_font_slots: true,
              wordprocessingml_punctuation_spacing: true,
              ..Default::default()
            };
            for text in [
              "甲，“乙",
              "甲，‘乙",
              "甲”，乙",
              "甲’，乙",
              "甲“（乙",
              "甲““乙",
              "甲””乙",
              "甲“”乙",
            ] {
              let natural = metrics.measure_text(text, &style);
              style.kerning_minimum_size_pt = Some(1.0);
              let reduction = if hinted && matches!(language, "zh-CN" | "ja-JP") && text != "甲“”乙"
              {
                4.5
              } else {
                0.0
              };
              let actual = metrics.measure_text(text, &style);
              assert!(
                (natural - actual - reduction).abs() < 0.001,
                "{family}/{language}/{same_primary}/{hinted}/{text}: {natural} -> {actual}, reduction {reduction}"
              );
              let common = crate::model::common_text_style(style.clone());
              assert!((metrics.measure_text(text, &common) - actual).abs() < 0.001);
              style.kerning_minimum_size_pt = Some(500.0);
            }
          }
        }
      }
    }
  }

  #[test]
  fn native_narrow_east_asia_quotes_keep_their_natural_advances() {
    // The v6 native opposites keep Malgun's narrow quotes, although all
    // punctuation uses eastAsia. A source hint alone is insufficient.
    let mut metrics = crate::text_metrics::TextMetrics::new();
    let mut style = TextStyle {
      font_family: Some("Calibri".into()),
      east_asia_font_family: Some("Malgun Gothic".into()),
      east_asia_language: Some("zh-CN".into()),
      wordprocessingml_font_hint: Some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia),
      font_size_pt: 9.0,
      kerning_minimum_size_pt: Some(500.0),
      wordprocessingml_font_slots: true,
      wordprocessingml_punctuation_spacing: true,
      ..Default::default()
    };
    for language in ["zh-CN", "ja-JP"] {
      style.east_asia_language = Some(language.into());
      for text in [
        "甲，“乙",
        "甲，‘乙",
        "甲”，乙",
        "甲’，乙",
        "甲“（乙",
        "甲““乙",
        "甲””乙",
        "甲“”乙",
      ] {
        let natural = metrics.measure_text(text, &style);
        style.kerning_minimum_size_pt = Some(1.0);
        assert!(
          (metrics.measure_text(text, &style) - natural).abs() < 0.001,
          "{language}/{text}"
        );
        style.kerning_minimum_size_pt = Some(500.0);
      }
    }
    style.kerning_minimum_size_pt = Some(1.0);
    let natural = metrics.measure_text("甲“乙", &style);
    style.cjk_punctuation_compression_ratio = 1.0;
    assert_eq!(metrics.measure_text("甲“乙", &style), natural);
    // Native pressure controls keep these narrow EA cells unchanged, but a
    // full-cell face on the same warm resolver has a returnable half-cell.
    style.east_asia_font_family = Some("宋体".into());
    style.cjk_punctuation_compression_ratio = 0.0;
    let natural = metrics.measure_text("甲“乙", &style);
    style.cjk_punctuation_compression_ratio = 1.0;
    assert_eq!(metrics.measure_text("甲“乙", &style), natural - 4.5);
  }

  #[test]
  fn western_bracket_compression_keeps_other_punctuation_and_cache_owners() {
    let mut metrics = crate::text_metrics::TextMetrics::new();
    let mut style = TextStyle {
      font_family: Some("Calibri".into()),
      east_asia_font_family: Some("宋体".into()),
      font_size_pt: 10.0,
      kerning_minimum_size_pt: Some(f32::INFINITY),
      wordprocessingml_font_slots: true,
      wordprocessingml_punctuation_spacing: true,
      cjk_punctuation_compression_ratio: 1.0,
      ..Default::default()
    };
    for language in ["en-US", "ko-KR", "zh-CN", "en-US"] {
      style.east_asia_language = Some(language.into());
      for (text, bracket) in [
        ("甲《乙", true),
        ("甲）乙", true),
        ("甲，乙", false),
        ("甲：乙", false),
      ] {
        let enabled = language == "zh-CN" || (bracket && language == "en-US");
        let expected = if enabled { 25.0 } else { 30.0 };
        assert!((metrics.measure_text(text, &style) - expected).abs() < 0.001);
        let common = crate::model::common_text_style(style.clone());
        assert!((metrics.measure_text(text, &common) - expected).abs() < 0.001);
        assert!((metrics.measure_text(text, &Box::new(style.clone())) - expected).abs() < 0.001);
      }
    }
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

fn word_vertical_quotation(character: char) -> bool {
  matches!(character, '\u{2018}' | '\u{2019}' | '\u{201c}' | '\u{201d}')
}

fn apply_word_vertical_quotation_cells(
  run: &mut ShapedRun<'_, '_>,
  face: &FontFaceData,
  horizontal_scale: f32,
) {
  if !run
    .glyphs
    .iter()
    .any(|glyph| glyph.source_char.is_some_and(word_vertical_quotation))
  {
    return;
  }
  let Ok(font) = FontRef::from_index(face.data.as_slice(), face.index) else {
    return;
  };
  let (Ok(head), Ok(horizontal)) = (font.head(), font.hmtx()) else {
    return;
  };
  let units = head.units_per_em();
  if units == 0 {
    return;
  }
  let vertical = font.vmtx().ok();
  let fallback = font
    .os2()
    .ok()
    .map(|os2| f32::from(os2.s_typo_ascender()) - f32::from(os2.s_typo_descender()));
  let scale = run.font_size_pt.0 * horizontal_scale / f32::from(units);
  let mut added = 0.0;
  for glyph in run.glyphs.to_mut() {
    if !glyph.source_char.is_some_and(word_vertical_quotation) {
      continue;
    }
    let id = skrifa::GlyphId::new(glyph.glyph_id);
    let Some(horizontal) = horizontal.advance(id) else {
      continue;
    };
    let Some(vertical) = vertical
      .as_ref()
      .and_then(|metrics| metrics.advance(id))
      .map(f32::from)
      .or(fallback)
      .filter(|height| *height > 0.0)
    else {
      continue;
    };
    // DirectWrite GetDesignGlyphMetrics exposes vmtx advanceHeight, falling
    // back to the OS/2 typographic cell for Latin fonts without vmtx. Native
    // Word uses this cell for all four curly quotes in btLr/tbRl frames while
    // preserving the ordinary glyph and the advance of straight quotes.
    // Add only the metric difference: kerning, tracking and synthetic glyph
    // adjustments already belonging to this run retain their ownership.
    let difference = (vertical - f32::from(horizontal)) * scale;
    glyph.x_advance_pt += difference;
    added += difference;
  }
  run.advance_pt += added;
}

fn apply_wordprocessingml_punctuation_compression(
  run: &mut ShapedRun<'_, '_>,
  text: &str,
  spacing: WordprocessingPunctuationSpacing,
  full_width_quotations: &[usize],
) {
  if spacing.ratio <= f32::EPSILON
    && spacing.bracket_ratio <= f32::EPSILON
    && spacing.quotation_ratio <= f32::EPSILON
    && spacing
      .terminal
      .is_none_or(|(_, value)| value <= f32::EPSILON)
    && !spacing.pair
    && !spacing.bracket_pair
  {
    return;
  }
  let minimum_full_width = run.font_size_pt.0 * 0.75;
  let mut total_reduction = 0.0;
  for glyph in run.glyphs.to_mut() {
    let Some(character) = glyph.source_char else {
      continue;
    };
    let side = spacing.compression_side(character);
    if side.is_none()
      && full_width_quotations
        .binary_search(&glyph.text_range.start)
        .is_err()
    {
      continue;
    }
    if glyph.x_advance_pt < minimum_full_width {
      continue;
    }
    let next = text
      .get(glyph.text_range.end..)
      .and_then(|tail| tail.chars().next())
      .filter(|&character| {
        curly_quotation_side(character).is_none()
          || full_width_quotations
            .binary_search(&glyph.text_range.end)
            .is_ok()
      })
      .and_then(|character| spacing.pair_side(character));
    // JLREQ §3.1.4: consecutive opening/closing punctuation shares its
    // inter-character half-em space. Word applies this natural pair spacing
    // with active run kerning even if characterSpacingControl=doNotCompress;
    // both punctuation classes must be eligible in the run's EA language.
    // It is not capacity
    // which the line formatter may later return. Native Mincho/SimSun pair
    // matrices also distinguish centered colons from trailing commas/stops.
    if punctuation_pair_tightens(spacing.pair_side(character), next, spacing.legacy) {
      let reduction = glyph.x_advance_pt * 0.5;
      glyph.x_advance_pt -= reduction;
      total_reduction += reduction;
      continue;
    }
    // Returnable quotation pressure has its own source-slot/language ratio,
    // independently of natural pair kerning. The actual advance threshold
    // above keeps narrow quotation cells out of both adjustments.
    let Some(side) = side else {
      continue;
    };
    // ECMA-376 Part 1 §17.15.1.18 limits this setting to full-width
    // punctuation. A full-width punctuation cell has at most one half-em of
    // removable side-bearing; the line formatter below returns whatever
    // fraction is not needed for the selected break.
    let terminal_ratio = spacing
      .terminal
      .filter(|&(index, _)| index == glyph.text_range.start);
    let ratio = terminal_ratio.map_or(spacing.ratio(character), |(_, ratio)| ratio.clamp(0.0, 1.0));
    let reduction = glyph.x_advance_pt * 0.5 * ratio;
    glyph.x_advance_pt -= reduction;
    if terminal_ratio.is_none() {
      match side {
        FullWidthPunctuationSide::Left => glyph.x_offset_pt -= reduction,
        FullWidthPunctuationSide::Right => {}
        FullWidthPunctuationSide::Middle => glyph.x_offset_pt -= reduction * 0.5,
      }
    }
    total_reduction += reduction;
  }
  run.advance_pt = (run.advance_pt - total_reduction).max(0.0);
}

pub(crate) fn wordprocessingml_balanced_space_neighbor(
  character: char,
  style: &(impl FontStyleRef + ?Sized),
) -> bool {
  use icu_properties::{CodePointMapData, props::EastAsianWidth};
  match CodePointMapData::<EastAsianWidth>::new().get(character) {
    EastAsianWidth::Wide | EastAsianWidth::Fullwidth => true,
    EastAsianWidth::Ambiguous if style.wordprocessingml_font_slots() => {
      // Space qualification follows the source neighbor's Word font slot,
      // not the physical glyph width. Native hinted narrow quotes/bullets
      // qualify a preceding blank; the same hAnsi glyphs do not.
      let mut encoded = [0; 4];
      script_direction_runs_with_options(
        character.encode_utf8(&mut encoded),
        FontSize(style.font_size_pt()),
        script_scan_options(style, false),
      )
      .first()
      .is_some_and(|run| run.wordprocessingml_font_slot == Some(WordprocessingFontSlot::EastAsia))
    }
    _ => false,
  }
}

fn wordprocessingml_balanced_pitch_units(character: char, fixed_halfwidth_cells: bool) -> i32 {
  use icu_properties::{
    CodePointMapData,
    props::{EastAsianWidth, Script},
  };
  match CodePointMapData::<EastAsianWidth>::new().get(character) {
    EastAsianWidth::Fullwidth if !character.is_whitespace() => 2,
    EastAsianWidth::Wide | EastAsianWidth::Fullwidth
      if fixed_halfwidth_cells
        || CodePointMapData::<Script>::new().get(character) == Script::Han =>
    {
      2
    }
    _ => 1,
  }
}

fn wordprocessingml_has_qualifying_balanced_space(
  run: &ShapedRun<'_, '_>,
  source_space_portion: bool,
) -> bool {
  // Native source/adjacent-space controls establish the Western reference
  // path. CJK-script spaces retain their independent existing half-em rule.
  if matches!(
    run.script,
    Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul)
  ) {
    return false;
  }
  let bytes = run.text.as_bytes();
  bytes.contains(&b' ')
    && ((source_space_portion && bytes.iter().all(|byte| *byte == b' '))
      || bytes.windows(2).any(|pair| pair == b"  "))
}

fn apply_wordprocessingml_single_double_byte_width_balance(
  run: &mut ShapedRun<'_, '_>,
  horizontal_scale: f32,
  character_spacing_pt: f32,
  source_space_portion: bool,
  reference_space_advance: Option<f32>,
  face: Option<&FontFaceData>,
) {
  let source_space_portion = source_space_portion && run.text.bytes().all(|byte| byte == b' ');
  let inside_cjk_script = matches!(
    run.script,
    Some(TextScript::Han | TextScript::Hiragana | TextScript::Katakana | TextScript::Hangul)
  );
  let target_space_advance = reference_space_advance.unwrap_or(run.font_size_pt.0 * 0.5)
    * horizontal_scale.max(f32::EPSILON)
    + 2.0 * character_spacing_pt;
  let run_start = run.text_range.start;
  let mut total_adjustment = 0.0;

  for glyph in run.glyphs.to_mut() {
    // balanceSingleByteDoubleByteWidth balances authored pitch as well as
    // space metrics. Preserve the resolved face's fixed-cell qualification;
    // EAW-wide alone also includes proportional kana/Hangul cells.
    if character_spacing_pt != 0.0
      && glyph.x_advance_pt > 0.0
      && glyph.source_char.is_some_and(|ch| {
        face.map_or_else(
          || wordprocessingml_balanced_pitch_units(ch, false),
          |face| face.wordprocessingml_balanced_pitch_units(ch),
        ) == 2
      })
    {
      glyph.x_advance_pt += character_spacing_pt;
      total_adjustment += character_spacing_pt;
    }
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
    if !source_space_portion && !previous_matches && !next_matches {
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
  fn wordprocessing_balanced_space_width(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    run: &ShapedRun<'_, '_>,
  ) -> Option<f32> {
    if !style.wordprocessingml_balance_single_byte_double_byte_width()
      || !wordprocessingml_has_qualifying_balanced_space(
        run,
        style.wordprocessingml_balanced_space_portion(),
      )
    {
      return None;
    }
    let family = style.wordprocessingml_balance_reference_font_family()?;
    let reference_style = WordSpaceReferenceStyle::new(style, family, run.font_size_pt.0);
    let metrics = self.font_metrics_for_slot(
      &reference_style,
      None,
      Some(WordprocessingFontSlot::EastAsia),
    )?;
    metrics.reference_widths.balanced_space_pt
  }

  pub(crate) fn wordprocessing_auto_script_spacing(
    &mut self,
    character: char,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<(bool, f32)> {
    if !style.wordprocessingml_font_slots()
      || style.horizontal_scale() != 1.0
      || style
        .wordprocessing_measurement_profile()
        .is_some_and(|(_, width)| width != 100)
    {
      return None;
    }
    let mut encoded = [0; 4];
    let text = character.encode_utf8(&mut encoded);
    let runs = script_direction_runs_with_options(
      text,
      FontSize(style.font_size_pt()),
      script_scan_options(style, false),
    );
    let run = runs.first()?;
    let metrics =
      self.font_metrics_for_slot(style, Some(run.script), run.wordprocessingml_font_slot)?;
    let east_asian = run.wordprocessingml_font_slot == Some(WordprocessingFontSlot::EastAsia)
      || metrics.vertical.wordprocessingml_cjk_line_metrics;
    let quarter_em = effective_font_size_pt(style, Some(run.script)) * 0.25;
    let balanced = style.wordprocessingml_balance_single_byte_double_byte_width();
    let gap = if balanced {
      quarter_em
    } else {
      metrics
        .reference_widths
        .half_average_pt
        .unwrap_or(quarter_em)
    };
    // Actual unpressured Word on/off arrays include authored tracking in
    // this reference cell, before quartering it. Balanced cells receive
    // two source pitch units. Convert pitch once to native ideal units;
    // integer division preserves the observed signed-pitch boundaries.
    let reference_ideal = (gap * 16384.0).round() as i64;
    let pitch_ideal = (style.character_spacing_pt() * 4096.0).trunc() as i64;
    let gap = ((reference_ideal + (1 + i64::from(balanced)) * pitch_ideal) / 4) as f32 / 4096.0;
    Some((east_asian, gap))
  }

  pub(crate) fn wordprocessing_legacy_kerning_uses_glyphs(
    &mut self,
    character: char,
    style: &(impl FontStyleRef + ?Sized),
  ) -> bool {
    if !style.kerning_enabled() {
      return false;
    }
    let mut encoded = [0; 4];
    let text = character.encode_utf8(&mut encoded);
    let runs = script_direction_runs_with_options(
      text,
      FontSize(style.font_size_pt()),
      script_scan_options(style, false),
    );
    let Some(run) = runs.first() else {
      return false;
    };
    self
      .font_metrics_for_slot(style, Some(run.script), run.wordprocessingml_font_slot)
      .is_some_and(|metrics| metrics.legacy_kerning_uses_glyphs)
  }

  pub(crate) fn has_exact_ascii_face(&mut self, style: &(impl FontStyleRef + ?Sized)) -> bool {
    self.has_exact_face_for_slot(
      style,
      TextScript::Latin,
      WordprocessingFontSlot::Ascii,
      None,
    )
  }

  pub(crate) fn has_exact_east_asia_face_for_char(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    character: char,
  ) -> bool {
    self.has_exact_face_for_slot(
      style,
      TextScript::Han,
      WordprocessingFontSlot::EastAsia,
      Some(character),
    )
  }

  fn has_exact_face_for_slot(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: TextScript,
    slot: WordprocessingFontSlot,
    character: Option<char>,
  ) -> bool {
    let script = Some(script);
    let slot = Some(slot);
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
          && character.is_none_or(|character| face.coverage.contains_char(character))
      })
  }

  pub(crate) fn exact_family_coverage(
    &mut self,
    family: &str,
  ) -> Option<ooxmlsdk_fonts::FontCoverage> {
    let style = TextStyle {
      font_family: Some(Arc::from(family)),
      ..TextStyle::default()
    };
    let request = font_request(&style, None);
    let registry = self.style_font_registry(&style, None);
    let resolved = registry.resolve(&request).ok()?;
    let face = registry.face(&resolved.font_id)?;
    face
      .family_names
      .iter()
      .any(|name| name.trim().eq_ignore_ascii_case(family.trim()))
      .then(|| face.coverage.clone())
  }

  pub fn load_text_face(&mut self, style: &(impl FontStyleRef + ?Sized)) -> Option<FontFaceData> {
    let mut request = font_request(style, None);
    let registry = self.style_font_registry(style, None);
    apply_wordprocessingml_retained_font_charset(&mut request, &registry, style, None);
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
      self.shape_text_runs_inner(text, style, &[], None)
    })
  }

  pub(crate) fn shape_text_runs_with_features<'text>(
    &mut self,
    text: &'text str,
    style: &(impl FontStyleRef + ?Sized),
    features: &[FeatureValue<'_>],
  ) -> Option<Vec<ShapedRun<'text, 'static>>> {
    font_timing("shape text runs with features", || {
      self.shape_text_runs_inner(text, style, features, None)
    })
  }

  pub(crate) fn shape_vertical_text_runs<'text>(
    &mut self,
    text: &'text str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<Vec<ShapedRun<'text, 'static>>> {
    self.shape_text_runs_inner(text, style, &[], Some(TextDirection::TopToBottom))
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

  pub(crate) fn script_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<ooxmlsdk_fonts::ScriptMetrics> {
    self.font_metrics(style, None).map(|metrics| metrics.script)
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
      wordprocessingml_realized_line_metrics(
        style,
        wordprocessing_line_metrics_font_slot(style, run.wordprocessingml_font_slot),
        &text[run.text_range.clone()],
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

    // The EA mapper and Word's enclosed-number glyph link can select different
    // physical faces for one declared font. Native linked and direct SimSun
    // portions have the same line cell. Keep the retained DEFAULT_CHARSET
    // caller's earlier logical-cell policy, and shape the complete source
    // once so neutral characters keep the same context as the painter.
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
      let vertical = if wordprocessingml_realized_line_metrics(style, slot, shaped.text) {
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
      } else if metrics.vertical.wordprocessingml_cjk_line_metrics {
        // A neighbouring retained DEFAULT_CHARSET run must not suppress
        // this face's CJK side leading. Native mixed-font/charset/noLeading
        // controls retain the same CJK ascent, just as ordinary text lines.
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
    apply_wordprocessingml_retained_font_charset(
      &mut request,
      &registry,
      style,
      wordprocessingml_font_slot,
    );
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
      script: metrics_at_size.script,
      legacy_kerning_uses_glyphs: wordprocessing_legacy_kerning_font(&resolved),
      reference_widths: wordprocessing_reference_widths(&resolved, request.size_pt.0),
      wordprocessingml_default_charset: wordprocessingml_default_charset_metrics(
        &request, &resolved,
      ),
    };
    self.font_metrics_cache.insert(key.clone(), metrics);
    self.last_font_metrics = Some((key, metrics));
    Some(metrics)
  }

  fn word_del_font_selection(
    &mut self,
    key: &FontFaceKey,
    registry: &mut Arc<FontRegistry<'static>>,
    request: &FontRequest<'_>,
  ) -> Option<&ResolvedFontChain<'static>> {
    let mut del_key = key.clone();
    del_key.wordprocessing_del_fallback = true;
    if !self.font_selection_cache.contains_key(&del_key) {
      let mut del_request = request.clone();
      del_request.family = Some(Cow::Borrowed("Roboto"));
      del_request.family_class = None;
      del_request.charset = None;
      del_request.pitch = None;
      // A style registry loads its requested faces lazily. Changing the
      // request alone cannot select a face that has not been registered.
      // Fork this local registry once, preserving the shared immutable
      // registry and making the new physical face available to metrics too.
      let mut linked = registry.as_ref().clone();
      linked
        .register_system_query_fonts(&del_request)
        .unwrap_or_default();
      *registry = Arc::new(linked);
      self
        .font_registry_cache
        .insert(key.clone(), registry.clone());
      if let Some((last_key, last_registry)) = &mut self.last_font_registry
        && last_key == key
      {
        *last_registry = registry.clone();
      }
      let selection = registry.resolve_font_chain(&del_request).ok()?;
      self.font_selection_cache.insert(del_key.clone(), selection);
    }
    self.font_selection_cache.get(&del_key)
  }

  fn shape_text_runs_inner<'text>(
    &mut self,
    text: &'text str,
    style: &(impl FontStyleRef + ?Sized),
    additional_features: &[FeatureValue<'_>],
    direction_override: Option<TextDirection>,
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
    if style.wordprocessingml_font_slots() {
      let implicit_glyphs = style
        .wordprocessing_simple_latin_glyphs()
        .unwrap_or_else(|| {
          !text
            .chars()
            .any(|ch| matches!(ch, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'))
        });
      let separate_words = !uses_complex_run_properties(style) && implicit_glyphs;
      script_runs = wordprocessingml_arabic_shaping_runs(
        text,
        script_runs,
        separate_words,
        separate_words
          && (style.wordprocessing_simple_latin_glyphs() == Some(true)
            || style.resolved_bidi_level().is_none()),
      );
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
      let mut registry = self.style_font_registry_for_slot(style, Some(script_run.script), slot);
      let mut request = font_request_for_slot(style, Some(script_run.script), slot);
      apply_wordprocessingml_retained_font_charset(&mut request, &registry, style, slot);
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
      // Native Word separates layout's automatic odd levels from its simple
      // Latin glyph path: nominal parentheses and LTR quote/blank order are
      // retained there. Explicit CS/RTL and authored directional controls use
      // the complex path and keep Unicode mirroring/RTL glyph order. The
      // paragraph owner supplies eligibility; script/font-run order remains
      // governed by the outer bidi level below.
      let direction = if style.wordprocessingml_font_slots()
        && style.wordprocessing_simple_latin_glyphs() == Some(true)
        && !uses_complex_run_properties(style)
        && matches!(script_run.script, TextScript::Latin | TextScript::Common)
      {
        TextDirection::LeftToRight
      } else {
        style
          .resolved_bidi_level()
          .map_or(script_run.direction, |level| {
            if level % 2 == 0 {
              TextDirection::LeftToRight
            } else {
              TextDirection::RightToLeft
            }
          })
      };
      let mut options =
        ShapeOptions::from_request(&request, direction_override.unwrap_or(direction));
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
      options.wordprocessingml_no_break_hyphen = style.wordprocessingml_no_break_hyphen();
      options.wordprocessingml_fixed_cjk_metrics = style.wordprocessingml_font_slots();
      options.horizontal_scale = style.horizontal_scale();
      options.preserve_default_ignorables = !segment_text.is_empty()
        && ((style.wordprocessing_nominal_control_metrics()
          && segment_text.chars().all(|ch| ch == '\u{202c}'))
          || (style.wordprocessingml_literal_soft_hyphen()
            && segment_text.chars().all(|ch| ch == '\u{00ad}')));
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
      options.wordprocessingml_arabic_lookup_order =
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
        let mut before = format!("{}{}", context.before, &text[..script_run.text_range.start]);
        let mut after = format!("{}{}", &text[script_run.text_range.end..], context.after);
        if style.wordprocessingml_font_slots() && script_run.script == TextScript::Arabic {
          // A neighbouring formatting run may supply context from beyond
          // the control. Preserve ordinary cross-run Arabic joining, but
          // never let that context bridge a native joining boundary.
          if let Some((index, character)) = before
            .char_indices()
            .rfind(|&(_, character)| wordprocessingml_arabic_joining_break(character))
          {
            before.drain(..index + character.len_utf8());
          }
          if let Some(index) = after.find(wordprocessingml_arabic_joining_break) {
            after.truncate(index);
          }
        }
        options.pre_context = Some(Cow::Owned(before));
        options.post_context = Some(Cow::Owned(after));
      }
      if !self.font_selection_cache.contains_key(&key) {
        let selection = registry.resolve_font_chain(&request).ok()?;
        self.font_selection_cache.insert(key.clone(), selection);
      }
      let (mut runs, mut synthesis) = {
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
      if style.wordprocessingml_font_slots()
        && segment_text == "\u{7f}"
        && !request.bold
        && !request.italic
        && runs
          .iter()
          .any(|run| run.glyphs.iter().any(|g| g.glyph_id == 0))
        && let Some(selection) = self.word_del_font_selection(&key, &mut registry, &request)
        && selection
          .resolved_fonts()
          .next()
          .is_some_and(|font| font.resolved_family.eq_ignore_ascii_case("Roboto"))
      {
        // Native PDF and XPS controls agree on regular Roboto .notdef for
        // missing DEL, including its own advance. Bold/italic controls keep
        // the selected face. Do not turn every missing C1 into this link.
        runs = registry
          .shape_text_runs_with_font_chain(selection, segment_text, &options)
          .ok()?;
        synthesis.extend(selection.resolved_fonts().map(|font| {
          (
            font.font_id.clone(),
            font.synthetic_bold,
            font.synthetic_italic,
          )
        }));
      }
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
            common::wordprocessing_device::MeasurementFont::new(&face, legacy, percent)
        {
          measurement.apply(run, style.horizontal_scale(), style.character_spacing_pt());
        } else if matches!(script_run.script, TextScript::Latin | TextScript::Common)
          && style.wordprocessingml_font_slots()
          && style.wordprocessing_measurement_profile() == Some((true, 100))
          && style.wordprocessing_kashida().is_none()
          && !style.kerning_enabled()
          && !style.small_caps()
          && style.horizontal_scale() == 1.0
          && style.character_spacing_pt() == 0.0
          && additional_features.is_empty()
          && segment_text.is_ascii()
          && run.glyphs.iter().all(|glyph| {
            glyph.text_range.len() == 1 && glyph.x_offset_pt == 0.0 && glyph.y_offset_pt == 0.0
          })
          && let Some(face) = self.font_face_data(&run.font_id)
          && let Some(measurement) =
            common::wordprocessing_device::MeasurementFont::new(&face, true, 100)
        {
          // The existing profile already excludes realized printer styles
          // and selects Word97 measurement only before compatibility mode15.
          // Native ASCII controls establish this nominal, untracked path;
          // kerning, ligatures and externally stretched runs keep their own
          // shaping/measurement owner. Never add a line-fitting epsilon.
          measurement.apply_simple_latin(run);
        }
        if style.wordprocessingml_balance_single_byte_double_byte_width() {
          // ECMA-376 Part 1 §17.15.3.3 balances qualifying spaces. Native
          // Word uses its logical reference font's average/fullwidth metric;
          // like Writer's BalanceCjkSpaces, apply this before justification.
          let reference_space_advance = self.wordprocessing_balanced_space_width(style, run);
          let face = (style.character_spacing_pt() != 0.0)
            .then(|| self.font_face_data(&run.font_id))
            .flatten();
          apply_wordprocessingml_single_double_byte_width_balance(
            run,
            style.horizontal_scale(),
            style.character_spacing_pt(),
            style.wordprocessingml_balanced_space_portion(),
            reference_space_advance,
            face.as_ref(),
          );
        }
        if style.wordprocessingml_font_slots()
          && style.wordprocessingml_vertical_punctuation()
          && let Some(face) = self.font_face_data(&run.font_id)
        {
          apply_word_vertical_quotation_cells(run, &face, style.horizontal_scale());
        }
        run.offset_text_range(script_run.text_range.start);
      }
      output.extend(runs);
    }
    let spacing = WordprocessingPunctuationSpacing::new(style);
    let mut full_width_quotations = Vec::new();
    if spacing.quotation_slot
      && (spacing.quotation_pair
        || spacing.quotation_ratio > 0.0
        || spacing.terminal.is_some_and(|(_, ratio)| ratio > 0.0))
      && text
        .chars()
        .any(|character| curly_quotation_side(character).is_some())
    {
      for run in &output {
        for glyph in run.glyphs.iter() {
          if glyph
            .source_char
            .is_some_and(|ch| curly_quotation_side(ch).is_some())
            && glyph.x_advance_pt >= run.font_size_pt.0 * 0.75
          {
            full_width_quotations.push(glyph.text_range.start);
          }
        }
      }
      full_width_quotations.sort_unstable();
    }
    // A punctuation neighbor may belong to another physical shaping run.
    // Qualify ambiguous cells from all actual advances before mutating any;
    // ordinary text allocates no quotation index and needs no font lookup.
    for run in &mut output {
      apply_wordprocessingml_punctuation_compression(run, text, spacing, &full_width_quotations);
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
  // Native Word retains Font.Ligatures but suppresses Latin standard and
  // contextual substitutions when character pitch is nonzero. The source
  // typography property and the effective shaping switches are separate.
  let suppress_ligatures = style.wordprocessingml_font_slots()
    && style.character_spacing_pt() != 0.0
    && matches!(script, Some(TextScript::Latin | TextScript::Common));
  if let Some(ligatures) = style
    .ligatures()
    .or_else(|| suppress_ligatures.then(common::OpenTypeLigatures::default))
  {
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
          (ligatures.standard && !suppress_ligatures)
            || (style.wordprocessingml_font_slots() && script == Some(TextScript::Arabic)),
        ),
      },
      FeatureValue {
        tag: Cow::Borrowed("clig"),
        value: u32::from(ligatures.contextual && !suppress_ligatures),
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
    language: wordprocessingml_shaping_language(style, script).map(Cow::Borrowed),
    features,
    ..FontRequest::default()
  }
}

fn wordprocessingml_shaping_language(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
) -> Option<&str> {
  // w:lang/@bidi owns language-system selection when w:cs or w:rtl selects
  // complex-script run properties. An automatically resolved Arabic bidi level
  // alone does not select that language. Native rtl/cs opposites retain the
  // default Arabic lookups until either property is true, independently of the
  // ordinary proofing language and the optional contextual-alternate switch.
  (style.wordprocessingml_font_slots()
    && script == Some(TextScript::Arabic)
    && uses_complex_run_properties(style))
  .then(|| style.wordprocessingml_bidi_language())
  .flatten()
  .filter(|language| !language.trim().is_empty())
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
  if let Some(force_complex) = style.complex_script_override()
    && !style.wordprocessingml_complex_numbering()
  {
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
  if let Some(force_complex) = style.complex_script_override()
    && !style.wordprocessingml_complex_numbering()
  {
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
  if let Some(force_complex) = style.complex_script_override()
    && !style.wordprocessingml_complex_numbering()
  {
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
  if let Some(force_complex) = style.complex_script_override()
    && !style.wordprocessingml_complex_numbering()
  {
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
    && !style.wordprocessingml_reference_font()
    && slot == Some(WordprocessingFontSlot::EastAsia)
}

fn wordprocessingml_unlisted_unicode_family(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
  slot: Option<WordprocessingFontSlot>,
) -> Option<&str> {
  if !style.wordprocessingml_font_slots() || !style.wordprocessingml_han_fallback() {
    return None;
  }
  let family = script_font_family_for_slot(style, script, slot)?.trim();
  (!family.is_ascii()
    && font_charset_for_slot(style, script, slot).is_none()
    && script_font_family_class_for_slot(style, script, slot).is_none()
    && font_pitch_for_slot(style, script, slot).is_none()
    && script_fallback_font_family_for_slot(style, script, slot).is_none())
  .then_some(family)
}

fn register_wordprocessingml_unlisted_unicode_family(
  registry: &mut FontRegistry<'static>,
  request: &FontRequest<'_>,
  family: &str,
) {
  if registry.book.faces.iter().any(|face| {
    face
      .family_names
      .iter()
      .any(|name| name.eq_ignore_ascii_case(family))
  }) {
    return;
  }
  if registry.resolve(request).is_ok_and(|resolved| {
    resolved.substitution.is_some_and(|substitution| {
      substitution.reason == ooxmlsdk_fonts::FontSubstitutionReason::Alias
        && substitution.requested_family.eq_ignore_ascii_case(family)
    })
  }) {
    return;
  }
  // Word's unlisted Unicode names create a separate GB2312/Roman fallback
  // record before font creation. Native fontTable and GDI observations agree
  // on SimSun for missing Han, Cyrillic and accented names, including names
  // whose semicolon prefix happens to be installed. Declared records (even
  // empty ones), installed/embedded full names and authored aliases stay above.
  // Build the complete-key alias once per cached face registry, not per glyph.
  registry.book.family_aliases.insert(
    0,
    ooxmlsdk_fonts::FontFamilyAlias {
      from: Cow::Owned(family.to_string()),
      to: Cow::Borrowed("SimSun"),
    },
  );
  let mut mapped = request.clone();
  mapped.family = Some(Cow::Borrowed("SimSun"));
  if !registry
    .resolve(&mapped)
    .is_ok_and(|resolved| resolved.resolved_family.eq_ignore_ascii_case("SimSun"))
  {
    registry
      .register_system_query_fonts(&mapped)
      .unwrap_or_default();
  }
}

pub(crate) fn wordprocessingml_join_control(ch: char) -> bool {
  // Unicode Core Specification, chapter 23: these format characters affect
  // adjacent joining/ligatures rather than supplying a visible glyph.
  matches!(ch, '\u{200c}' | '\u{200d}')
}

fn wordprocessingml_retained_font_charset_family(
  request: &FontRequest<'_>,
  registry: &FontRegistry<'_>,
  style: &(impl FontStyleRef + ?Sized),
  slot: Option<WordprocessingFontSlot>,
) -> Option<&'static str> {
  use ooxmlsdk_fonts::FontPitch;
  if !style.wordprocessingml_font_slots()
    || request.charset != Some(FontCharset::ShiftJis)
    || request.pitch != Some(FontPitch::Variable)
    || wordprocessingml_missing_family_fallback_for_slot(style, request.script, slot).is_none()
  {
    return None;
  }
  let family_class = request.family_class?;
  let replacement = match family_class {
    FontFamilyClass::Serif => "MS PMincho",
    FontFamilyClass::SansSerif => "MS PGothic",
    _ => return None,
  };
  let primary = registry.resolve(request).ok()?;
  if matches!(primary.source, FontSource::EmbeddedOoxml { .. }) {
    return None;
  }
  let requested = request.family.as_deref()?.split(';').next()?.trim();
  let face = registry.face(&primary.font_id)?;
  if !face
    .family_names
    .iter()
    .any(|name| name.eq_ignore_ascii_case(requested))
    || face.family_class != Some(family_class)
    || face.pitch != FontPitch::Variable
    || registered_font_has_shift_jis_charset(registry, &primary.font_id) != Some(false)
  {
    return None;
  }
  // Word retains a font-table charset when the installed face agrees with
  // its explicit family/pitch; mismatching records are rebuilt as the Latin
  // face. Native Word font-table readbacks and GDI LOGFONT controls establish
  // the proportional Japanese mapper result independently of glyph coverage.
  Some(replacement)
}

fn registered_font_has_shift_jis_charset(
  registry: &FontRegistry<'_>,
  font_id: &FontId,
) -> Option<bool> {
  if let Some((data, index)) = registry.font_face_binary(font_id) {
    let face = FontRef::from_index(data.as_ref(), index).ok()?;
    // A single ANSI registration does not exclude another physical code
    // page. Real bytes own capability; absent OS/2 fields stay unknown.
    return Some(face.os2().ok()?.ul_code_page_range_1()? & (1 << 17) != 0);
  }
  registry
    .faces
    .iter()
    .find(|face| face.source.id() == Some(font_id.0.as_ref()))
    .and_then(|face| face.charset)
    .map(|charset| charset == FontCharset::ShiftJis)
}

pub(crate) fn font_table_named_face(
  registry: &mut FontRegistry<'static>,
  family: &str,
) -> Option<FontId> {
  let request = FontRequest {
    family: Some(Cow::Borrowed(family)),
    ..Default::default()
  };
  registry
    .register_system_query_fonts(&request)
    .unwrap_or_default();
  let resolved = registry.resolve(&request).ok()?;
  if matches!(resolved.source, FontSource::EmbeddedOoxml { .. })
    || registry.face(&resolved.font_id).is_none_or(|face| {
      !face
        .family_names
        .iter()
        .any(|name| name.eq_ignore_ascii_case(family))
    })
  {
    return None;
  }
  Some(resolved.font_id)
}

pub(crate) fn font_table_named_true_type_font(
  registry: &mut FontRegistry<'static>,
  family: &str,
) -> bool {
  let Some(id) = font_table_named_face(registry, family) else {
    return false;
  };
  let Some((data, index)) = registry.font_face_binary(&id) else {
    return false;
  };
  FontRef::from_index(data.as_ref(), index).is_ok_and(|face| {
    // TrueType/OpenType with TrueType outlines and legacy Apple TrueType.
    // A CFF sfnt or unavailable/embedded face is not this measured mismatch.
    matches!(
      face.table_directory().sfnt_version(),
      0x0001_0000 | 0x7472_7565
    )
  })
}

fn apply_wordprocessingml_retained_font_charset(
  request: &mut FontRequest<'_>,
  registry: &FontRegistry<'_>,
  style: &(impl FontStyleRef + ?Sized),
  slot: Option<WordprocessingFontSlot>,
) {
  let Some(family) = wordprocessingml_retained_font_charset_family(request, registry, style, slot)
  else {
    return;
  };
  let mut mapped = request.clone();
  mapped.family = Some(Cow::Borrowed(family));
  if registry.resolve(&mapped).is_ok_and(|resolved| {
    resolved.resolved_family.eq_ignore_ascii_case(family)
      && registered_font_has_shift_jis_charset(registry, &resolved.font_id) == Some(true)
  }) {
    // Map only this primary request; preserve independent glyph-link families.
    request.family = mapped.family;
  }
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
  // MS-OI29500 2.1.88 / ECMA-376 17.3.2.26 assigns fullwidth/CJK ranges to
  // the EA slot without a hint; ambiguous symbols/Greek/Cyrillic use the hint.
  // Native ordinary-run controls map Latin-only EA faces to SimSun in both
  // cases, even when the face covers the character. Slot selection has already
  // resolved that distinction. A physical CJK face stays selected; missing
  // SimSun glyphs then follow the ordinary glyph-link policy.
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
    // Parsing the document's private font metadata belongs to its collection,
    // not to each size/script/style key or each fresh PDF resolver.
    let mut registry = style
      .embedded_fonts()
      .map(|embedded| embedded.registry().clone())
      .unwrap_or_else(FontRegistry::with_default_policy);
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
    if style.wordprocessingml_symbol_fallback() && script.is_some() {
      // Word links missing parenthesized digits and digits with a full stop
      // to SimSun, in ordinary text as well as generated numbering. Native
      // font-slot and complete 1..20 controls retain covered Noto/Segoe/SimSun
      // primary faces and exclude the adjoining circled/letter characters.
      // This is a glyph link, not the separate circle-numbering EA context.
      registry.book.fallback_chains.insert(
        0,
        FontFallbackChain {
          unicode_ranges: std::iter::once(WORD_ENCLOSED_NUMBER_LINK_RANGE).collect(),
          script: Some(TextScript::Common),
          families: vec![Cow::Borrowed("SimSun")],
          ..Default::default()
        },
      );
      // Native Word's complete Geometric Shapes coverage matrix keeps the
      // authored face first, then tries styled Times New Roman and Cambria
      // Math. Coverage varies with weight/slant; do not map individual glyphs
      // or replace covered primary faces. Keep the two default-emoji squares
      // outside this text-symbol link and retain generic symbol fallback.
      registry.book.fallback_chains.insert(
        0,
        FontFallbackChain {
          unicode_ranges: vec![0x25a0..0x25fd, 0x25ff..0x2600],
          script: Some(TextScript::Common),
          families: vec![
            Cow::Borrowed("Times New Roman"),
            Cow::Borrowed("Cambria Math"),
          ],
          ..Default::default()
        },
      );
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
    if let Some(family) =
      wordprocessingml_unlisted_unicode_family(style, script, wordprocessingml_font_slot)
    {
      register_wordprocessingml_unlisted_unicode_family(&mut registry, &request, family);
    }
    if let Some(family) = wordprocessingml_retained_font_charset_family(
      &request,
      &registry,
      style,
      wordprocessingml_font_slot,
    ) {
      let mut mapped = request.clone();
      mapped.family = Some(Cow::Borrowed(family));
      registry
        .register_system_query_fonts(&mapped)
        .unwrap_or_default();
    }
    if style.wordprocessingml_han_fallback()
      && script == Some(TextScript::Han)
      && registry
        .resolve(&request)
        .is_ok_and(|primary| !primary.metrics.vertical.wordprocessingml_cjk_line_metrics)
    {
      // Native ordinary-run controls (UI zh-CN/zh-TW/en-US, modes12/15)
      // link missing Han from non-CJK logical fonts to SimSun. A CJK font
      // retains its different missing-glyph chain: MS Mincho's missing 块
      // still links to Microsoft YaHei. Keep both primary coverage and
      // authored family-specific links ahead of this Word-only policy.
      let insertion = registry
        .book
        .fallback_chains
        .iter()
        .position(|chain| chain.requested_family.is_none())
        .unwrap_or(registry.book.fallback_chains.len());
      registry.book.fallback_chains.insert(
        insertion,
        FontFallbackChain {
          script: Some(TextScript::Han),
          families: vec![Cow::Borrowed("SimSun")],
          ..Default::default()
        },
      );
      let mut fallback = request.clone();
      fallback.family = Some(Cow::Borrowed("SimSun"));
      registry
        .register_system_query_fonts(&fallback)
        .unwrap_or_default();
    }
    if style.wordprocessingml_han_fallback()
      && script == Some(TextScript::Hangul)
      && registry.resolve(&request).is_ok_and(|primary| {
        !primary.metrics.vertical.wordprocessingml_cjk_line_metrics
          || [
            "SimSun",
            "宋体",
            "DengXian",
            "等线",
            "MS PMincho",
            "ＭＳ Ｐ明朝",
          ]
          .iter()
          .any(|family| primary.resolved_family.eq_ignore_ascii_case(family))
      })
    {
      // Native ordinary-run and calculation controls link missing Hangul
      // from Latin, SimSun, DengXian and proportional Mincho to Batang.
      // Fixed Mincho, Meiryo and YaHei retain their Malgun Gothic links;
      // code-page/Japanese classification alone cannot distinguish them.
      // Keep primary glyph coverage and existing family-specific chains first.
      let insertion = registry
        .book
        .fallback_chains
        .iter()
        .position(|chain| chain.requested_family.is_none())
        .unwrap_or(registry.book.fallback_chains.len());
      registry.book.fallback_chains.insert(
        insertion,
        FontFallbackChain {
          script: Some(TextScript::Hangul),
          families: vec![Cow::Borrowed("Batang")],
          ..Default::default()
        },
      );
      let mut fallback = request.clone();
      fallback.family = Some(Cow::Borrowed("Batang"));
      registry
        .register_system_query_fonts(&fallback)
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
  let fixed_halfwidth_cells = FontRef::from_index(data.as_slice(), index)
    .ok()
    .is_some_and(|font| {
      if font
        .post()
        .ok()
        .is_some_and(|post| post.is_fixed_pitch() != 0)
      {
        return true;
      }
      let (Ok(head), Ok(horizontal)) = (font.head(), font.hmtx()) else {
        return false;
      };
      // Legacy SimSun omits post.isFixedPitch although its ASCII cells are
      // uniformly half an em. Read the actual design widths once per face.
      let cmap = font.charmap();
      [' ', 'I', 'W'].into_iter().all(|ch| {
        cmap
          .map(ch)
          .and_then(|glyph| horizontal.advance(glyph))
          .is_some_and(|advance| u32::from(advance) * 2 == u32::from(head.units_per_em()))
      })
    });
  Some(FontFaceData {
    data: Arc::new(data),
    index,
    synthetic_bold: false,
    synthetic_italic: false,
    document_embedded: registry.sources.iter().any(|source| {
      matches!(source, FontSource::EmbeddedOoxml { id, .. } if id.as_ref() == font_id.0.as_ref())
    }),
    named_instance: registry.face(font_id)?.named_instance.clone(),
    fixed_halfwidth_cells,
    id: font_id.0.clone(),
  })
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontFaceKey {
  embedded_fonts: Option<Arc<EmbeddedFontSet>>,
  family: Option<String>,
  wordprocessingml_font_slots: bool,
  wordprocessingml_unlisted_unicode_family: bool,
  wordprocessingml_han_fallback: bool,
  wordprocessingml_symbol_fallback: bool,
  fallback_family: Option<String>,
  family_class: Option<FontFamilyClass>,
  charset: Option<FontCharset>,
  pitch: Option<ooxmlsdk_fonts::FontPitch>,
  bold: bool,
  italic: bool,
  synthetic_rtl_italic: bool,
  script: Option<TextScript>,
  language: Option<Box<str>>,
  wordprocessingml_missing_family_fallback: Option<&'static str>,
  wordprocessing_nominal_control_metrics: bool,
  wordprocessing_del_fallback: bool,
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
      embedded_fonts: style.embedded_fonts().cloned(),
      family: script_font_family_for_slot(style, script, wordprocessingml_font_slot)
        .map(str::to_string),
      wordprocessingml_font_slots: style.wordprocessingml_font_slots(),
      wordprocessingml_unlisted_unicode_family: wordprocessingml_unlisted_unicode_family(
        style,
        script,
        wordprocessingml_font_slot,
      )
      .is_some(),
      wordprocessingml_han_fallback: style.wordprocessingml_han_fallback()
        && matches!(script, Some(TextScript::Han | TextScript::Hangul)),
      wordprocessingml_symbol_fallback: style.wordprocessingml_symbol_fallback()
        && script.is_some(),
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
      wordprocessing_del_fallback: false,
      wordprocessingml_east_asia_font_mapping: wordprocessingml_east_asia_font_mapping(
        style,
        wordprocessingml_font_slot,
      ),
      script,
      language: wordprocessingml_shaping_language(style, script).map(Into::into),
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
    self.embedded_fonts.as_ref() == style.embedded_fonts()
      && !self.wordprocessing_del_fallback
      && self.family.as_deref()
        == script_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.wordprocessingml_font_slots == style.wordprocessingml_font_slots()
      && self.wordprocessingml_unlisted_unicode_family
        == wordprocessingml_unlisted_unicode_family(style, script, wordprocessingml_font_slot)
          .is_some()
      && self.wordprocessingml_han_fallback
        == (style.wordprocessingml_han_fallback()
          && matches!(script, Some(TextScript::Han | TextScript::Hangul)))
      && self.wordprocessingml_symbol_fallback
        == (style.wordprocessingml_symbol_fallback() && script.is_some())
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
      && self.language.as_deref() == wordprocessingml_shaping_language(style, script)
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
  script: ooxmlsdk_fonts::ScriptMetrics,
  legacy_kerning_uses_glyphs: bool,
  reference_widths: WordReferenceWidths,
  wordprocessingml_default_charset: Option<ooxmlsdk_fonts::VerticalMetrics>,
}

#[derive(Clone, Copy, Debug, Default)]
struct WordReferenceWidths {
  // Preserve old auto-gap behavior: integral average is halved afterwards.
  half_average_pt: Option<f32>,
  balanced_space_pt: Option<f32>,
}

fn ideal_reference_width(design: u32, units_per_em: u16, size_pt: f32) -> Option<i32> {
  if design == 0 || units_per_em == 0 || !size_pt.is_finite() || size_pt <= 0.0 {
    return None;
  }
  let ideal = (f64::from(design) * f64::from(size_pt) * 4096.0 / f64::from(units_per_em)).round();
  (ideal > 0.0 && ideal <= f64::from(i32::MAX)).then_some(ideal as i32)
}

fn reference_widths_from_design(
  average: u32,
  fullwidth: Option<u32>,
  units_per_em: u16,
  size_pt: f32,
  cjk_reference: bool,
  synthetic_bold: bool,
) -> WordReferenceWidths {
  let average_ideal = ideal_reference_width(average, units_per_em, size_pt);
  // Keep the previously established synthetic-negative auto-gap contract.
  let half_average_pt = (!synthetic_bold)
    .then_some(average_ideal)
    .flatten()
    .map(|units| (units / 2) as f32 / 4096.0);
  let bold_pixel = u32::from(synthetic_bold);
  let balanced_space_pt = if cjk_reference {
    // Actual Word reference font is created at lfHeight=-UPEM. A simulated
    // GDI bold adds one logical pixel before taking half of fullwidth.
    fullwidth
      .filter(|width| *width > 0)
      .and_then(|width| width.checked_add(bold_pixel))
      .and_then(|width| ideal_reference_width(width, units_per_em, size_pt))
      .map(|units| (units / 2) as f32 / 4096.0)
  } else {
    (average > 0)
      .then(|| average.checked_add(bold_pixel))
      .flatten()
      .and_then(|width| ideal_reference_width(width, units_per_em, size_pt))
      .map(|units| units as f32 / 4096.0)
  };
  WordReferenceWidths {
    half_average_pt,
    balanced_space_pt,
  }
}

fn wordprocessing_reference_widths(
  resolved: &ooxmlsdk_fonts::ResolvedFont<'_>,
  size: f32,
) -> WordReferenceWidths {
  let Some(data) = resolved.source.data() else {
    return WordReferenceWidths::default();
  };
  let Ok(face) = FontRef::from_index(data, resolved.face_index) else {
    return WordReferenceWidths::default();
  };
  let Ok(head) = face.head() else {
    return WordReferenceWidths::default();
  };
  let average = face
    .os2()
    .ok()
    .and_then(|os2| u32::try_from(os2.x_avg_char_width()).ok())
    .unwrap_or(0);
  let cjk_reference = resolved.metrics.vertical.wordprocessingml_cjk_line_metrics;
  let fullwidth = if cjk_reference {
    face
      .charmap()
      .map('\u{3000}')
      .filter(|glyph| glyph.to_u32() != 0)
      .and_then(|glyph| face.hmtx().ok()?.advance(glyph))
      .map(u32::from)
  } else {
    None
  };
  // Word's native reference font is created at -UPEM. Its fullwidth/average
  // design metric (including the one-pixel simulated-bold addition) is scaled
  // to integral 1/4096pt units before the CJK fullwidth is halved. The separate
  // auto-gap value keeps the previously established half-average behavior.
  reference_widths_from_design(
    average,
    fullwidth,
    head.units_per_em(),
    size,
    cjk_reference,
    resolved.synthetic_bold,
  )
}

fn wordprocessing_legacy_kerning_font(resolved: &ooxmlsdk_fonts::ResolvedFont<'_>) -> bool {
  let Some(data) = resolved.source.data() else {
    return false;
  };
  let Ok(face) = FontRef::from_index(data, resolved.face_index) else {
    return false;
  };
  let Ok(gpos) = face.gpos() else {
    return false;
  };
  // Native mode14 kerning controls distinguish the legacy character and
  // OpenType glyph paths. Western GPOS faces take the latter even without
  // a kern lookup (Courier New); CJK faces without Latin kerning retain the
  // character path (SimHei, MS Mincho, Microsoft YaHei). Meiryo's Latin kern
  // feature is an independent positive. Cache this with the selected logical
  // face, before a missing punctuation glyph can select a linked CJK face.
  if !resolved.metrics.vertical.wordprocessingml_cjk_line_metrics {
    return true;
  }
  let Ok(scripts) = gpos.script_list() else {
    return false;
  };
  let Some(script) = scripts
    .script_records()
    .iter()
    .find(|script| script.script_tag().to_be_bytes() == *b"latn")
    .and_then(|record| record.script(scripts.offset_data()).ok())
  else {
    return false;
  };
  let Some(Ok(language)) = script.default_lang_sys() else {
    return false;
  };
  let Ok(features) = gpos.feature_list() else {
    return false;
  };
  language.feature_indices().iter().any(|index| {
    features
      .feature_records()
      .get(usize::from(index.get()))
      .is_some_and(|feature| feature.feature_tag().to_be_bytes() == *b"kern")
  })
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
  // saved font tables normalize DEFAULT_CHARSET to ANSI for the Latin
  // families below, or ARABIC_CHARSET for Traditional Arabic. Native Arabic
  // controls with default/ANSI/Arabic/omitted charsets share the physical cell.
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
    "Traditional Arabic",
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
  // East Asian faces realize their concrete charset (for example PMingLiU
  // realizes BIG5_CHARSET=136). Keep their independent compatibility path.
  let code_pages = os2.ul_code_page_range_1().unwrap_or_default();
  if code_pages & 0x001e_0000 != 0 {
    return None;
  }
  let fixed = face.post().is_ok_and(|post| post.is_fixed_pitch() != 0);
  let pitch = if fixed {
    ooxmlsdk_fonts::FontPitch::Fixed
  } else {
    ooxmlsdk_fonts::FontPitch::Variable
  };
  let panose = os2.panose_10();
  let pictorial = panose[0] == 5 && code_pages & 0x8000_0000 != 0;
  if code_pages & 0x8000_0000 != 0 && !pictorial {
    return None;
  }
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
  // A pictorial face retains DEFAULT_CHARSET with family=auto; requesting
  // decorative instead normalizes it to ANSI. Native OpenSymbol controls
  // separately vary charset, family and pitch, in ordinary and numbered text.
  let family_matches = if pictorial {
    request.family_class.is_none()
  } else {
    family.is_some() && request.family_class == family
  };
  if !family_matches || request.pitch != Some(pitch) {
    return None;
  }
  let scale = request.size_pt.0 / f32::from(face.head().ok()?.units_per_em().max(1));
  if pictorial
    && os2.version() >= 4
    && os2
      .fs_selection()
      .contains(skrifa::raw::tables::os2::SelectionFlags::USE_TYPO_METRICS)
  {
    // This logical symbol cell honors typographic opt-in and owns the gap
    // above its baseline, before DEFAULT_CHARSET side leading is added.
    // Renamed-font Office controls vary each Typo/Win/hhea field and the
    // opt-in bit independently: only Typo fields affect the opted-in cell;
    // clearing the bit restores the raw Windows cell below. The physical
    // face and its ordinary ANSI/SYMBOL metrics stay unchanged.
    let ascent =
      f32::from(os2.s_typo_ascender()) * scale + f32::from(os2.s_typo_line_gap()) * scale;
    let descent = -f32::from(os2.s_typo_descender()) * scale;
    return Some(ooxmlsdk_fonts::VerticalMetrics {
      ascent_pt: ascent,
      descent_pt: descent,
      windows_line_height_pt: ascent + descent,
      baseline_offset_pt: ascent,
      directwrite_baseline_offset_pt: ascent,
      line_gap_pt: 0.0,
      ink_height_pt: ascent + descent,
      ..resolved.metrics_at_size(request.size_pt).vertical
    });
  }
  // The retained legacy charset uses the GDI/Windows font box even when
  // the face opts into USE_TYPO_METRICS. Calibri's native charset01 control
  // distinguishes this from simply scaling the typographic line box.
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
    if combined.font_design_unit_pt != metrics.font_design_unit_pt {
      // A synthetic box assembled from independent faces has no single
      // design grid. Formatted Word lines adjust each face before combining.
      combined.font_design_unit_pt = 0.0;
    }
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
  let side = (crate::text_metrics::wordprocessingml_cjk_side_leading_pt(
    metrics.ascent_pt + metrics.descent_pt,
    metrics.font_design_unit_pt,
  ) - metrics.line_gap_pt / 2.0)
    .max(0.0);
  // Match the formatted Word CJK cell, including retained DEFAULT_CHARSET
  // callers, without changing the physical face metrics used for paint.
  let external_leading_above = (metrics.directwrite_baseline_offset_pt
    - metrics.baseline_offset_pt)
    .max(0.0)
    .min(metrics.line_gap_pt)
    * 0.5;
  metrics.ascent_pt += side;
  metrics.descent_pt += side;
  metrics.windows_line_height_pt += side * 2.0;
  metrics.baseline_offset_pt += side;
  metrics.directwrite_baseline_offset_pt += side - external_leading_above;
  metrics
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontMetricsKey {
  embedded_fonts: Option<Arc<EmbeddedFontSet>>,
  family: Option<String>,
  wordprocessingml_font_slots: bool,
  wordprocessingml_unlisted_unicode_family: bool,
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
      embedded_fonts: style.embedded_fonts().cloned(),
      family: script_font_family_for_slot(style, script, wordprocessingml_font_slot)
        .map(str::to_string),
      wordprocessingml_font_slots: style.wordprocessingml_font_slots(),
      wordprocessingml_unlisted_unicode_family: wordprocessingml_unlisted_unicode_family(
        style,
        script,
        wordprocessingml_font_slot,
      )
      .is_some(),
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
    self.embedded_fonts.as_ref() == style.embedded_fonts()
      && self.family.as_deref()
        == script_font_family_for_slot(style, script, wordprocessingml_font_slot)
      && self.wordprocessingml_font_slots == style.wordprocessingml_font_slots()
      && self.wordprocessingml_unlisted_unicode_family
        == wordprocessingml_unlisted_unicode_family(style, script, wordprocessingml_font_slot)
          .is_some()
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

  #[test]
  fn complex_numbering_retains_script_sizes_and_western_suffix() {
    let style = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      high_ansi_font_family: Some(Arc::from("Cambria")),
      complex_font_family: Some(Arc::from("Arial")),
      font_size_pt: 11.0,
      complex_font_size_pt: Some(16.0),
      bold: false,
      complex_bold: Some(true),
      complex_script: Some(true),
      wordprocessingml_font_slots: true,
      wordprocessingml_complex_numbering: true,
      ..Default::default()
    };
    let common = crate::model::common_text_style(style.clone());
    for style in [&style as &dyn super::FontStyleRef, &common] {
      for text in ["א.", "أ."] {
        let runs = script_direction_runs_with_options(
          text,
          FontSize(11.0),
          super::script_scan_options(style, false),
        );
        assert_eq!(runs.len(), 2);
        for (run, family, size, slot) in [
          (
            &runs[0],
            "Arial",
            16.0,
            WordprocessingFontSlot::ComplexScript,
          ),
          (&runs[1], "Calibri", 11.0, WordprocessingFontSlot::Ascii),
        ] {
          assert_eq!(run.wordprocessingml_font_slot, Some(slot));
          let request = super::font_request_for_slot(style, Some(run.script), Some(slot));
          assert_eq!(request.family.as_deref(), Some(family));
          assert_eq!(request.size_pt.0, size);
          assert!(super::effective_bold(style, Some(run.script)));
        }
      }
      let prefixed = script_direction_runs_with_options(
        "Ωא.",
        FontSize(11.0),
        super::script_scan_options(style, false),
      );
      assert_eq!(prefixed.len(), 3);
      assert_eq!(
        prefixed[0].wordprocessingml_font_slot,
        Some(WordprocessingFontSlot::Ascii)
      );
      let request = super::font_request_for_slot(
        style,
        Some(prefixed[0].script),
        prefixed[0].wordprocessingml_font_slot,
      );
      assert_eq!(request.family.as_deref(), Some("Calibri"));
      assert_eq!(request.size_pt.0, 11.0);
    }
    let ordinary = TextStyle {
      complex_script: None,
      wordprocessingml_complex_numbering: false,
      ..style.clone()
    };
    let mut resolver = super::FontResolver::default();
    let before = resolver.shape_text_runs("א.", &ordinary).unwrap();
    let marker = resolver.shape_text_runs("א.", &style).unwrap();
    let after = resolver.shape_text_runs("א.", &ordinary).unwrap();
    assert_eq!(marker.len(), 2);
    assert_eq!(marker[0].font_size_pt.0, 16.0);
    assert_eq!(marker[1].font_size_pt.0, 11.0);
    assert_ne!(marker[0].font_id, before[0].font_id);
    assert_eq!(before, after);
  }

  use super::{
    apply_wordprocessingml_single_double_byte_width_balance, effective_font_size_pt, font_request,
    font_request_for_slot, load_text_face, materialize_wordprocessingml_source_font_slot,
    script_fallback_font_family_for_slot, script_font_family_for_slot, script_scan_options,
    shape_text_runs, wordprocessing_line_metrics_font_slot,
    wordprocessingml_missing_family_fallback_for_slot,
  };

  #[test]
  fn unlisted_unicode_font_recovery_preserves_exact_names_and_cache_owners() {
    use ooxmlsdk_fonts::{FontFaceInfo, FontFamilySelection, FontRegistry, FontSource};
    let mut style = TextStyle {
      font_family: Some(Arc::from("ＭＳ 明朝;MS Mincho")),
      wordprocessingml_font_slots: true,
      wordprocessingml_han_fallback: true,
      ..Default::default()
    };
    let script = Some(TextScript::Latin);
    let slot = Some(WordprocessingFontSlot::Ascii);
    assert!(super::wordprocessingml_unlisted_unicode_family(&style, script, slot).is_some());
    let face_key = super::FontFaceKey::from_style_for_slot(&style, script, slot);
    let metrics_key = super::FontMetricsKey::from_style_for_slot(&style, script, slot);
    style.wordprocessingml_han_fallback = false;
    assert!(super::wordprocessingml_unlisted_unicode_family(&style, script, slot).is_none());
    assert!(!face_key.matches_style_for_slot(&style, script, slot));
    assert!(!metrics_key.matches_style_for_slot(&style, script, slot));
    style.wordprocessingml_han_fallback = true;
    style.font_charset = Some(FontCharset::Ansi);
    assert!(super::wordprocessingml_unlisted_unicode_family(&style, script, slot).is_none());
    assert!(!face_key.matches_style_for_slot(&style, script, slot));
    style.font_charset = None;

    for exact_full_name in [false, true] {
      let family = style.font_family.as_deref().unwrap();
      let mut registry = FontRegistry::new();
      for name in ["ＭＳ 明朝", "MS Mincho", "SimSun"] {
        registry.register_face(FontSource::System, FontFaceInfo::synthetic(name, name));
      }
      if exact_full_name {
        registry.register_face(
          FontSource::System,
          FontFaceInfo::synthetic(family, family.to_string()),
        );
      }
      let request = super::font_request_for_slot(&style, script, slot);
      assert_eq!(request.family_selection, FontFamilySelection::First);
      super::register_wordprocessingml_unlisted_unicode_family(&mut registry, &request, family);
      let alias = registry
        .book
        .family_aliases
        .iter()
        .any(|alias| alias.from == family);
      assert_eq!(alias, !exact_full_name);
      if !exact_full_name {
        assert_eq!(
          registry.resolve(&request).unwrap().resolved_family,
          "SimSun"
        );
      }
    }
  }

  #[test]
  fn embedded_font_collections_isolate_warm_face_caches_and_survive_style_adapters() {
    let plain = TextStyle {
      font_family: Some(Arc::from("Arial")),
      ..Default::default()
    };
    let system = load_text_face(&plain).expect("font test environment has a default face");
    let bytes = system.data.as_slice().to_vec();
    let first = super::EmbeddedFontSet::new(vec![("first-document".into(), bytes.clone())])
      .expect("valid system font");
    let second = super::EmbeddedFontSet::new(vec![("second-document".into(), bytes)])
      .expect("valid system font");
    let first_style = TextStyle {
      embedded_fonts: Some(first.clone()),
      ..plain.clone()
    };
    let second_style = TextStyle {
      embedded_fonts: Some(second),
      ..plain.clone()
    };
    let mut resolver = super::FontResolver::default();
    let first_face = resolver.cached_text_face(&first_style).unwrap();
    let second_face = resolver
      .cached_text_face(&Box::new(second_style.clone()))
      .unwrap();
    assert!(first_face.document_embedded && second_face.document_embedded);
    assert_ne!(first_face.id(), second_face.id());
    assert_eq!(first_face.data, second_face.data);
    let mut metrics = crate::text_metrics::TextMetrics::new();
    assert_eq!(
      metrics.cached_text_face(&first_style).unwrap().id(),
      first_face.id()
    );
    assert_eq!(
      metrics.cached_text_face(&second_style).unwrap().id(),
      second_face.id()
    );
    assert_eq!(
      metrics.cached_text_face(&first_style).unwrap().id(),
      first_face.id()
    );
    assert!(!metrics.cached_text_face(&plain).unwrap().document_embedded);
    let common = crate::model::common_text_style(first_style);
    assert!(Arc::ptr_eq(common.embedded_fonts.as_ref().unwrap(), &first));
    assert_eq!(
      resolver.cached_text_face(&common).unwrap().id(),
      first_face.id()
    );
    assert!(!resolver.cached_text_face(&plain).unwrap().document_embedded);
    assert!(super::EmbeddedFontSet::new(vec![("invalid".into(), vec![0; 32])]).is_none());
  }

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
  fn word_balance_reference_skips_unqualified_text_and_reuses_complete_metric_keys() {
    use super::{
      FontFaceKey, FontMetrics, FontMetricsKey, FontResolver, WordReferenceWidths,
      WordSpaceReferenceStyle,
    };
    let style = TextStyle {
      font_family: Some(Arc::from("Times New Roman")),
      wordprocessingml_balance_single_byte_double_byte_width: true,
      wordprocessingml_balance_reference_font_family: Some(Arc::from("SimSun")),
      ..Default::default()
    };
    let mut resolver = FontResolver::default();
    for text in ["abc", "a b", " "] {
      let run = synthetic_space_run(text, TextScript::Latin);
      assert_eq!(
        resolver.wordprocessing_balanced_space_width(&style, &run),
        None
      );
    }
    let source_portion = TextStyle {
      wordprocessingml_balanced_space_portion: true,
      ..style.clone()
    };
    for script in [
      TextScript::Han,
      TextScript::Hiragana,
      TextScript::Katakana,
      TextScript::Hangul,
    ] {
      for text in ["甲 乙", "甲  乙", " "] {
        let run = synthetic_space_run(text, script);
        assert_eq!(
          resolver.wordprocessing_balanced_space_width(&source_portion, &run),
          None
        );
      }
    }
    assert!(resolver.font_metrics_cache.is_empty());
    assert!(resolver.font_registry_cache.is_empty());
    let mut run = synthetic_space_run("  ", TextScript::Latin);
    run.font_size_pt = FontSize(8.0);
    let mut widths = Vec::new();
    for (family, width) in [
      ("SimSun", Some(4.0)),
      ("Times New Roman", Some(3.207_031_3)),
      ("NoWidth", None),
    ] {
      let mut candidate = style.clone();
      candidate.wordprocessingml_balance_reference_font_family = Some(Arc::from(family));
      assert_eq!(
        FontFaceKey::from_style(&candidate, None),
        FontFaceKey::from_style(&style, None)
      );
      let reference = WordSpaceReferenceStyle::new(&candidate, family, 8.0);
      let key = FontMetricsKey::from_style_for_slot(
        &reference,
        None,
        Some(WordprocessingFontSlot::EastAsia),
      );
      assert!(key.matches_style_for_slot(&reference, None, Some(WordprocessingFontSlot::EastAsia)));
      let other_size = WordSpaceReferenceStyle::new(&candidate, family, 4.0);
      assert!(!key.matches_style_for_slot(
        &other_size,
        None,
        Some(WordprocessingFontSlot::EastAsia)
      ));
      assert!(
        resolver
          .font_metrics_cache
          .insert(
            key,
            FontMetrics {
              vertical: Default::default(),
              decoration: Default::default(),
              script: Default::default(),
              legacy_kerning_uses_glyphs: false,
              reference_widths: WordReferenceWidths {
                half_average_pt: None,
                balanced_space_pt: width
              },
              wordprocessingml_default_charset: None,
            }
          )
          .is_none()
      );
      widths.push((candidate, width));
    }
    for (candidate, width) in widths.iter().rev().chain(widths.iter()) {
      assert_eq!(
        resolver.wordprocessing_balanced_space_width(candidate, &run),
        *width
      );
    }
    assert_eq!(resolver.font_metrics_cache.len(), 3);
    assert!(
      resolver.font_registry_cache.is_empty(),
      "cache hits never resolve a physical font again"
    );
    let disabled = TextStyle {
      wordprocessingml_balance_single_byte_double_byte_width: false,
      ..style
    };
    assert_eq!(
      resolver.wordprocessing_balanced_space_width(&disabled, &run),
      None
    );
    assert_eq!(
      super::FontStyleRef::wordprocessingml_balance_reference_font_family(&disabled),
      None
    );
  }

  #[test]
  fn word_balance_reference_keeps_metadata_with_its_own_source_face() {
    use super::{FontMetricsKey, FontStyleRef, WordSpaceReferenceStyle};
    let style = TextStyle {
      east_asia_font_family: Some(Arc::from("Times New Roman")),
      east_asia_font_family_class: Some(super::FontFamilyClass::Serif),
      east_asia_font_charset: Some(FontCharset::Ansi),
      east_asia_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
      east_asia_fallback_font_family: Some(Arc::from("Liberation Serif")),
      ..Default::default()
    };
    let direct = WordSpaceReferenceStyle::new(&style, "Times New Roman", 8.0);
    assert_eq!(
      direct.east_asia_font_family_class(),
      Some(super::FontFamilyClass::Serif)
    );
    assert_eq!(
      direct.wordprocessingml_east_asia_font_charset(),
      Some(FontCharset::Ansi)
    );
    assert_eq!(
      direct.east_asia_fallback_font_family(),
      Some("Liberation Serif")
    );
    let themed = WordSpaceReferenceStyle::new(&style, "SimSun", 8.0);
    assert_eq!(themed.east_asia_font_family_class(), None);
    assert_eq!(themed.wordprocessingml_east_asia_font_charset(), None);
    assert_eq!(themed.east_asia_fallback_font_family(), None);
    let private_style = |fingerprint| TextStyle {
      embedded_fonts: Some(Arc::new(super::EmbeddedFontSet {
        fingerprint,
        fonts: Vec::new(),
        registry: std::sync::OnceLock::new(),
      })),
      ..style.clone()
    };
    let first = private_style(1);
    let second = private_style(2);
    let first_reference = WordSpaceReferenceStyle::new(&first, "SimSun", 8.0);
    let second_reference = WordSpaceReferenceStyle::new(&second, "SimSun", 8.0);
    let key = FontMetricsKey::from_style_for_slot(
      &first_reference,
      None,
      Some(WordprocessingFontSlot::EastAsia),
    );
    assert!(key.matches_style_for_slot(
      &first_reference,
      None,
      Some(WordprocessingFontSlot::EastAsia)
    ));
    assert!(!key.matches_style_for_slot(
      &second_reference,
      None,
      Some(WordprocessingFontSlot::EastAsia)
    ));
  }

  #[test]
  fn word_auto_script_gap_uses_the_tracked_logical_reference_cell() {
    // Actual unconstrained MSLS70 on/off arrays, across four EA proofing
    // languages. Keep both logical source slots distinct when unbalanced.
    let mut resolver = super::FontResolver::default();
    for (balance, pitch, han_ideal, latin_ideal) in [
      (false, -0.2, 15155, 15800),
      (false, 0.0, 15360, 16005),
      (false, 0.2, 15564, 16209),
      (false, 0.75, 16128, 16773),
      (true, -0.2, 14950, 14950),
      (true, 0.0, 15360, 15360),
      (true, 0.2, 15769, 15769),
      (true, 0.75, 16896, 16896),
    ] {
      for language in ["en-US", "zh-CN", "zh-TW", "ja-JP"] {
        let style = TextStyle {
          font_family: Some("Calibri".into()),
          high_ansi_font_family: Some("Calibri".into()),
          east_asia_font_family: Some("SimSun".into()),
          east_asia_language: Some(language.into()),
          font_size_pt: 15.0,
          character_spacing_pt: pitch,
          wordprocessingml_font_slots: true,
          wordprocessingml_font_hint: Some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia),
          wordprocessingml_balance_single_byte_double_byte_width: balance,
          ..Default::default()
        };
        for (character, expected_slot, expected) in
          [('甲', true, han_ideal), ('1', false, latin_ideal)]
        {
          let (slot, gap) = resolver
            .wordprocessing_auto_script_spacing(character, &style)
            .unwrap();
          assert_eq!(slot, expected_slot);
          assert_eq!(
            (gap * 4096.0).round() as i32,
            expected,
            "{language}/{balance}/{pitch}/{character}"
          );
        }
      }
    }
  }

  #[test]
  fn word_character_indent_keeps_tracking_outside_authored_font_scale() {
    for (scale, tracking, balance, expected) in [
      (0.8, -0.2, true, 11.6),
      (0.8, 0.2, true, 12.4),
      (1.0, 0.2, true, 15.4),
      (1.2, 0.2, true, 18.4),
      (0.8, 0.5, true, 13.0),
      (0.8, 0.2, false, 12.2),
      (1.2, 0.2, false, 18.2),
    ] {
      let style = TextStyle {
        font_size_pt: 15.0,
        horizontal_scale: Some(scale),
        character_spacing_pt: tracking,
        wordprocessingml_balance_single_byte_double_byte_width: balance,
        ..Default::default()
      };
      assert!((super::wordprocessingml_character_indent_unit_pt(&style) - expected).abs() < 0.001);
    }
  }

  #[test]
  fn word_balance_reference_keeps_latin_metrics_separate_from_ea_glyph_mapping() {
    use super::{FontMetricsKey, FontResolver, WordSpaceReferenceStyle, WordprocessingFontSlot};

    let style = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      east_asia_font_family: Some(Arc::from("Calibri")),
      font_size_pt: 7.5,
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let reference = WordSpaceReferenceStyle::new(&style, "Calibri", 7.5);
    let slot = Some(WordprocessingFontSlot::EastAsia);
    let glyph_key = FontMetricsKey::from_style_for_slot(&style, None, slot);
    let reference_key = FontMetricsKey::from_style_for_slot(&reference, None, slot);
    assert_ne!(glyph_key, reference_key);
    let mut resolver = FontResolver::default();
    let first = resolver.font_metrics_for_slot(&style, None, slot).unwrap();
    let logical = resolver
      .font_metrics_for_slot(&reference, None, slot)
      .unwrap();
    assert!(first.vertical.wordprocessingml_cjk_line_metrics);
    assert!(!logical.vertical.wordprocessingml_cjk_line_metrics);
    // Native Calibri reference is the covered primary face, whose SFNT
    // xAvgCharWidth is1067/2048em. Its mapped SimSun glyph uses half-em.
    let expected = (1067.0_f64 / 2048.0 * 7.5 * 4096.0).round() as f32 / 4096.0;
    assert_eq!(logical.reference_widths.balanced_space_pt, Some(expected));
    assert_eq!(
      resolver
        .font_metrics_for_slot(&style, None, slot)
        .unwrap()
        .reference_widths
        .balanced_space_pt,
      first.reference_widths.balanced_space_pt
    );
  }

  #[test]
  fn word_balance_reference_is_applied_before_scale_and_character_spacing() {
    let mut run = synthetic_space_run("A  B", TextScript::Latin);
    let before = run.advance_pt;
    apply_wordprocessingml_single_double_byte_width_balance(
      &mut run,
      0.8,
      0.25,
      false,
      Some(4.015625),
      None,
    );
    let target = 4.015625 * 0.8 + 0.5;
    assert_eq!(run.glyphs[1].x_advance_pt, target);
    assert_eq!(run.glyphs[2].x_advance_pt, target);
    assert!((run.advance_pt - (before + 2.0 * (target - 2.5))).abs() < 0.00001);
    assert_eq!(run.glyphs[0].x_advance_pt, 6.0);
    assert_eq!(run.glyphs[3].x_advance_pt, 6.0);
  }

  #[test]
  fn word_balance_reference_metrics_keep_native_ideal_units() {
    use super::reference_widths_from_design;
    for (size, expected) in [(4.0, 8224), (8.0, 16448), (12.0, 24672)] {
      let v = reference_widths_from_design(128, Some(256), 256, size, true, true);
      assert_eq!(v.balanced_space_pt, Some(expected as f32 / 4096.0));
      assert_eq!(v.half_average_pt, None);
    }
    for average in [821, 874] {
      let v = reference_widths_from_design(average, None, 2048, 8.0, false, false);
      assert_eq!(v.balanced_space_pt, Some((average * 16) as f32 / 4096.0));
    }
    // Correctly declared YaHei has a physical bold face; its different xAvg
    // values must not replace the fullwidth reference used for CJK balancing.
    for average in [987, 1040] {
      let v = reference_widths_from_design(average, Some(2048), 2048, 8.0, true, false);
      assert_eq!(v.balanced_space_pt, Some(4.0));
    }
    // Existing independent auto-gap Native case:17241 is halved after rounding.
    assert_eq!(
      reference_widths_from_design(821, None, 2048, 10.5, false, false).half_average_pt,
      Some(8620.0 / 4096.0)
    );
    assert_eq!(
      reference_widths_from_design(0, None, 2048, 8.0, false, true).balanced_space_pt,
      None
    );
    assert_eq!(
      reference_widths_from_design(128, Some(0), 256, 8.0, true, true).balanced_space_pt,
      None
    );
  }

  #[test]
  fn single_double_byte_balance_expands_only_adjacent_latin_spaces() {
    let mut trailing = synthetic_space_run("A: ", TextScript::Latin);
    apply_wordprocessingml_single_double_byte_width_balance(
      &mut trailing,
      1.0,
      0.0,
      false,
      None,
      None,
    );
    assert_eq!(trailing.glyphs[2].x_advance_pt, 2.5);
    assert_eq!(trailing.advance_pt, 14.5);

    let mut internal = synthetic_space_run("A B", TextScript::Latin);
    apply_wordprocessingml_single_double_byte_width_balance(
      &mut internal,
      1.0,
      0.0,
      false,
      None,
      None,
    );
    assert_eq!(internal.glyphs[1].x_advance_pt, 2.5);
    assert_eq!(internal.advance_pt, 14.5);

    let mut adjacent = synthetic_space_run("A  B", TextScript::Latin);
    apply_wordprocessingml_single_double_byte_width_balance(
      &mut adjacent,
      1.0,
      0.0,
      false,
      None,
      None,
    );
    assert_eq!(adjacent.glyphs[1].x_advance_pt, 5.0);
    assert_eq!(adjacent.glyphs[2].x_advance_pt, 5.0);
  }

  #[test]
  fn single_double_byte_balance_source_portion_is_not_a_trial_text_edge() {
    let mut isolated = synthetic_space_run(" ", TextScript::Latin);
    apply_wordprocessingml_single_double_byte_width_balance(
      &mut isolated,
      1.0,
      0.0,
      false,
      None,
      None,
    );
    assert_eq!(isolated.advance_pt, 2.5);
    apply_wordprocessingml_single_double_byte_width_balance(
      &mut isolated,
      1.0,
      0.0,
      true,
      None,
      None,
    );
    assert_eq!(isolated.advance_pt, 5.0);

    let mut internal = synthetic_space_run("A B", TextScript::Latin);
    apply_wordprocessingml_single_double_byte_width_balance(
      &mut internal,
      1.0,
      0.0,
      true,
      None,
      None,
    );
    assert_eq!(internal.glyphs[1].x_advance_pt, 2.5);
    let plain = TextStyle::default();
    let qualified = TextStyle {
      wordprocessingml_balanced_space_portion: true,
      ..plain.clone()
    };
    assert_eq!(
      super::FontFaceKey::from_style_for_slot(&plain, Some(TextScript::Latin), None),
      super::FontFaceKey::from_style_for_slot(&qualified, Some(TextScript::Latin), None),
      "source blank qualification does not create a new font-face cache entry",
    );
  }

  #[test]
  fn single_double_byte_balance_resets_cjk_spaces_before_other_spacing() {
    let mut run = synthetic_space_run("甲 乙", TextScript::Han);
    apply_wordprocessingml_single_double_byte_width_balance(&mut run, 0.8, 0.25, false, None, None);
    assert_eq!(run.glyphs[1].x_advance_pt, 4.5);
    assert_eq!(run.glyphs[0].x_advance_pt, 6.25);
    assert_eq!(run.glyphs[2].x_advance_pt, 6.25);
    assert_eq!(run.advance_pt, 17.0);
  }

  #[test]
  fn balanced_pitch_keeps_proportional_kana_and_hangul_source_units() {
    let mut metrics = crate::text_metrics::TextMetrics::new();
    for (family, text, extra_units) in [
      ("SimSun", "あいう", 3.0),
      ("MS Mincho", "あいう", 3.0),
      ("Malgun Gothic", "あいう", 0.0),
      ("Malgun Gothic", "한글", 0.0),
      ("Malgun Gothic", "甲　乙", 2.0),
      ("MS PMincho", "あいう", 0.0),
      ("MS PMincho", "ＡＢ", 2.0),
    ] {
      let mut style = TextStyle {
        font_family: Some("Calibri".into()),
        east_asia_font_family: Some(family.into()),
        font_size_pt: 11.0,
        character_spacing_pt: 0.75,
        wordprocessingml_font_slots: true,
        wordprocessingml_font_hint: Some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia),
        language: Some("en-US".into()),
        east_asia_language: Some("zh-CN".into()),
        ..Default::default()
      };
      let ordinary = metrics.measure_text(text, &style);
      style.wordprocessingml_balance_single_byte_double_byte_width = true;
      let balanced = metrics.measure_text(text, &style);
      assert!(
        (balanced - ordinary - extra_units * 0.75).abs() < 0.001,
        "{family}/{text}: {ordinary}->{balanced}"
      );
    }
  }

  #[test]
  fn balanced_ambiguous_pitch_uses_actual_cell_not_source_font_slot() {
    let mut metrics = crate::text_metrics::TextMetrics::new();
    for (family, fullwidth) in [
      ("SimSun", true),
      ("MS Mincho", true),
      ("Malgun Gothic", false),
    ] {
      let face = super::load_text_face(&TextStyle {
        font_family: Some(family.into()),
        ..Default::default()
      })
      .expect("native control font");
      for (ch, units) in [
        ('“', if fullwidth { 2 } else { 1 }),
        ('Ω', if fullwidth { 2 } else { 1 }),
        ('•', 1),
        ('é', 1),
      ] {
        assert_eq!(
          face.wordprocessingml_balanced_pitch_units(ch),
          units,
          "{family}/{ch}"
        );
      }
      for hint in [false, true] {
        for same_primary in [false, true] {
          let mut style = TextStyle {
            font_family: Some(if same_primary { family } else { "Calibri" }.into()),
            high_ansi_font_family: Some(if same_primary { family } else { "Calibri" }.into()),
            east_asia_font_family: Some(family.into()),
            east_asia_language: Some("en-US".into()),
            font_size_pt: 11.0,
            character_spacing_pt: 0.75,
            wordprocessingml_font_slots: true,
            wordprocessingml_font_hint: hint
              .then_some(ooxmlsdk_fonts::WordprocessingFontTypeHint::EastAsia),
            ..Default::default()
          };
          let ordinary = metrics.measure_text("“Ω•é", &style);
          style.wordprocessingml_balance_single_byte_double_byte_width = true;
          let extra = metrics.measure_text("“Ω•é", &style) - ordinary;
          let expected = if fullwidth && (hint || same_primary) {
            1.5
          } else {
            0.0
          };
          assert!(
            (extra - expected).abs() < 0.001,
            "{family}/{hint}/{same_primary}: {extra}"
          );
        }
      }
    }
  }

  #[test]
  fn balanced_wide_character_pitch_matches_word_native_signed_vectors() {
    // Native Word11pt observations at pitch -0.25/0/+0.75: wide cells are
    // 10.5/11/12.5pt before the independent600DPI paint realization. Cover
    // complete shaping, not a line-prefix or a manually adjusted glyph.
    let mut metrics = crate::text_metrics::TextMetrics::new();
    for family in ["SimSun", "MS Mincho", "Malgun Gothic"] {
      for (pitch, expected) in [(-0.25, 21.0), (0.0, 22.0), (0.75, 25.0)] {
        let style = TextStyle {
          font_family: Some("Calibri".into()),
          east_asia_font_family: Some(family.into()),
          font_size_pt: 11.0,
          character_spacing_pt: pitch,
          wordprocessingml_font_slots: true,
          wordprocessingml_balance_single_byte_double_byte_width: true,
          ..Default::default()
        };
        let width = metrics.measure_text("甲乙", &style);
        assert!(
          (width - expected).abs() < 0.001,
          "{family}/{pitch}: {width}"
        );
        let plain = TextStyle {
          wordprocessingml_balance_single_byte_double_byte_width: false,
          ..style.clone()
        };
        for text in ["ABC DEF", "A\u{00a0}B"] {
          assert_eq!(
            metrics.measure_text(text, &style),
            metrics.measure_text(text, &plain),
            "an isolated Western space or NBSP is not a balanced cell"
          );
        }
      }
    }
  }

  #[test]
  fn word_fixed_cjk_metrics_preserve_cells_without_disabling_latin_kerning() {
    let mut resolver = super::FontResolver::default();
    let style = TextStyle {
      font_family: Some(Arc::from("Yu Mincho")),
      east_asia_font_family: Some(Arc::from("Yu Mincho")),
      font_size_pt: 10.5,
      wordprocessingml_font_slots: true,
      kerning_minimum_size_pt: Some(1.0),
      ..Default::default()
    };
    let mut width = |text: &str, style: &TextStyle| {
      resolver
        .shape_text_runs(text, style)
        .unwrap()
        .iter()
        .map(|run| run.advance_pt)
        .sum::<f32>()
    };
    for text in ["げる", "じっ", "って", "ＡＶ"] {
      assert!(
        (width(text, &style) - text.chars().count() as f32 * 10.5).abs() < 0.001,
        "{text}"
      );
    }
    let generic = TextStyle {
      wordprocessingml_font_slots: false,
      ..style.clone()
    };
    assert!(width("げる", &generic) < width("げる", &style) - 0.5);
    let latin = TextStyle {
      font_family: Some(Arc::from("Arial")),
      ..style.clone()
    };
    let unkerned = TextStyle {
      kerning_minimum_size_pt: Some(100.0),
      ..latin.clone()
    };
    assert!(width("AV", &latin) < width("AV", &unkerned));
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
  fn word_enclosed_number_link_uses_the_realized_line_cell() {
    use ooxmlsdk_fonts::FontFamilyClass;

    let mut metrics = crate::text_metrics::TextMetrics::new();
    for size in [6.0, 11.0, 22.0] {
      for charset in [FontCharset::Ansi, FontCharset::Other(1)] {
        for text in ["⑴.", "⒈."] {
          let linked = TextStyle {
            font_family: Some(Arc::from("Calibri")),
            high_ansi_font_family: Some(Arc::from("Calibri")),
            font_family_class: Some(FontFamilyClass::SansSerif),
            high_ansi_font_family_class: Some(FontFamilyClass::SansSerif),
            font_charset: Some(charset),
            high_ansi_font_charset: Some(charset),
            font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
            high_ansi_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Variable),
            font_size_pt: size,
            wordprocessingml_font_slots: true,
            wordprocessingml_symbol_fallback: true,
            wordprocessingml_cjk_line_metrics: true,
            use_windows_font_metrics: true,
            ..Default::default()
          };
          let direct = TextStyle {
            high_ansi_font_family: Some(Arc::from("SimSun")),
            high_ansi_font_family_class: Some(FontFamilyClass::Serif),
            high_ansi_font_charset: None,
            high_ansi_font_pitch: Some(ooxmlsdk_fonts::FontPitch::Fixed),
            ..linked.clone()
          };
          // Native ANSI controls use the same cell for a linked and declared
          // CJK face. DEFAULT_CHARSET instead retains the Calibri logical
          // cell, also used by its ordinary ASCII text. Changing the declared
          // hAnsi family would change that request's charset and cell policy.
          let actual = metrics.line_vertical_metrics_for_text(text, &linked);
          let expected = if charset == FontCharset::Ansi {
            metrics.line_vertical_metrics_for_text(text, &direct)
          } else {
            metrics.line_vertical_metrics_for_text("Probe", &linked)
          };
          assert!(
            (actual.line_height_pt() - expected.line_height_pt()).abs() < 0.0001,
            "{size}/{charset:?}/{text}: {actual:?} vs {expected:?}"
          );
          assert!(
            (actual.directwrite_baseline_offset_pt - expected.directwrite_baseline_offset_pt).abs()
              < 0.0001,
            "{size}/{charset:?}/{text}: {actual:?} vs {expected:?}"
          );
          let common = Box::new(crate::model::common_text_style(linked));
          assert_eq!(
            metrics.line_vertical_metrics_for_text(text, &common),
            actual
          );
        }
      }
    }
  }

  #[test]
  fn word_del_control_uses_regular_link_without_polluting_other_font_cache_entries() {
    let mut resolver = super::FontResolver::default();
    let roboto = resolver.cached_text_face(&TextStyle {
      font_family: Some(Arc::from("Roboto")),
      ..Default::default()
    });
    // The Office-cloud Roboto face is optional, like other native-control
    // fonts in this module. The engine retains its primary when unavailable.
    if !roboto.is_some_and(|face| face.id().contains("Roboto")) {
      return;
    }
    let text = "AA\u{7f}BB";
    for family in ["Arial", "Calibri", "Times New Roman", "Consolas"] {
      for (bold, italic) in [(false, false), (true, false), (false, true), (true, true)] {
        let style = TextStyle {
          font_family: Some(Arc::from(family)),
          high_ansi_font_family: Some(Arc::from(family)),
          font_size_pt: 12.0,
          bold,
          italic,
          wordprocessingml_font_slots: true,
          ..Default::default()
        };
        for _ in 0..2 {
          let runs = resolver.shape_text_runs(text, &style).unwrap();
          assert_eq!(runs.iter().map(|run| run.text).collect::<String>(), text);
          for run in runs {
            let linked = !bold && !italic && run.text == "\u{7f}";
            assert_eq!(
              run.font_id.0.contains("Roboto"),
              linked,
              "{family}/{bold}/{italic}: {run:?}"
            );
            if run.text == "\u{7f}" {
              assert_eq!(run.glyphs.len(), 1);
              assert_eq!(run.glyphs[0].glyph_id, 0);
              assert!(run.glyphs[0].x_advance_pt > 0.0);
            }
          }
          let ordinary = resolver
            .shape_text_runs(
              text,
              &TextStyle {
                wordprocessingml_font_slots: false,
                ..style.clone()
              },
            )
            .unwrap();
          assert!(ordinary.iter().all(|run| !run.font_id.0.contains("Roboto")));
        }
      }
    }
  }

  #[test]
  fn word_enclosed_number_fallback_keeps_covered_faces_and_neighboring_symbols() {
    let mut resolver = super::FontResolver::default();
    let digits: String = (0x2474..0x249c)
      .map(|value| char::from_u32(value).unwrap())
      .collect();
    for (family, text, expected) in [
      ("Calibri", digits.as_str(), "SimSun"),
      ("Cambria Math", "⑴⒈", "SimSun"),
      ("Segoe UI Symbol", "⑴⒈", "SegoeUISymbol"),
      ("SimSun", "⑴⒈", "SimSun"),
      ("Noto Serif CJK SC", "⑴⒈", "NotoSerifCJKsc"),
    ] {
      let style = TextStyle {
        font_family: Some(Arc::from(family)),
        high_ansi_font_family: Some(Arc::from(family)),
        east_asia_font_family: Some(Arc::from("MS Mincho")),
        wordprocessingml_font_slots: true,
        wordprocessingml_symbol_fallback: true,
        ..Default::default()
      };
      let runs = resolver.shape_text_runs(text, &style).unwrap();
      assert!(!runs.is_empty());
      for run in runs {
        assert!(
          run.font_id.0.contains(expected),
          "{family}, {text}: {run:?}"
        );
        assert!(!run.approximate);
        assert!(run.glyphs.iter().all(|glyph| glyph.glyph_id != 0));
      }
    }
    let style = TextStyle {
      font_family: Some(Arc::from("Calibri")),
      high_ansi_font_family: Some(Arc::from("Calibri")),
      wordprocessingml_font_slots: true,
      wordprocessingml_symbol_fallback: true,
      ..Default::default()
    };
    for text in ["①", "⑳", "⒜", "Ⓐ", "☐"] {
      let word = resolver.shape_text_runs(text, &style).unwrap();
      let ordinary = resolver
        .shape_text_runs(
          text,
          &TextStyle {
            wordprocessingml_symbol_fallback: false,
            ..style.clone()
          },
        )
        .unwrap();
      assert_eq!(word.len(), ordinary.len());
      for (word, ordinary) in word.iter().zip(&ordinary) {
        assert_eq!(word.font_id, ordinary.font_id, "{text}");
        assert_eq!(word.glyphs, ordinary.glyphs, "{text}");
        assert_eq!(word.advance_pt, ordinary.advance_pt, "{text}");
      }
    }
  }

  #[test]
  fn word_enclosed_number_fallback_isolated_in_warm_policy_and_adapter_caches() {
    use super::FontStyleRef;

    for text in ["⑴", "⒈"] {
      for first in [false, true] {
        let mut resolver = super::FontResolver::default();
        let mut metrics = crate::text_metrics::TextMetrics::new();
        let mut widths = Vec::new();
        for word in [first, !first, first, !first] {
          let style = TextStyle {
            font_family: Some(Arc::from("Calibri")),
            high_ansi_font_family: Some(Arc::from("Calibri")),
            wordprocessingml_font_slots: true,
            wordprocessingml_symbol_fallback: word,
            ..Default::default()
          };
          let expected = if word { "SimSun" } else { "SegoeUISymbol" };
          let shaped = resolver.shape_text_runs(text, &style).unwrap();
          assert_eq!(shaped.len(), 1);
          assert!(shaped[0].font_id.0.contains(expected), "{shaped:?}");
          let common = Box::new(crate::model::common_text_style(style.clone()));
          assert_eq!(common.wordprocessingml_symbol_fallback(), word);
          assert_eq!(resolver.shape_text_runs(text, &common).unwrap(), shaped);
          let width = metrics.measure_text(text, &style);
          assert_eq!(metrics.measure_text(text, &common), width);
          widths.push(width);
        }
        assert_ne!(widths[0], widths[1]);
        assert_eq!(widths[0], widths[2]);
        assert_eq!(widths[1], widths[3]);
      }
    }
  }

  #[test]
  fn word_symbol_fallback_keeps_covered_fonts_and_other_symbol_blocks() {
    let mut resolver = super::FontResolver::default();
    for family in ["Verdana", "Calibri", "Georgia"] {
      for (bold, italic) in [(false, false), (true, false), (false, true), (true, true)] {
        let style = TextStyle {
          font_family: Some(Arc::from(family)),
          high_ansi_font_family: Some(Arc::from(family)),
          wordprocessingml_font_slots: true,
          wordprocessingml_symbol_fallback: true,
          bold,
          italic,
          ..Default::default()
        };
        for (text, expected) in [("■", "TimesNewRoman"), ("□", family), ("▢", "CambriaMath")]
        {
          let runs = resolver.shape_text_runs(text, &style).unwrap();
          assert_eq!(runs.len(), 1, "{family}, {bold}/{italic}, {text}: {runs:?}");
          assert!(runs[0].font_id.0.contains(expected), "{runs:?}");
          assert!(!runs[0].approximate);
          assert!(runs[0].glyphs.iter().all(|glyph| glyph.glyph_id != 0));
        }
        let runs = resolver.shape_text_runs("ABC ■ XYZ", &style).unwrap();
        assert!(
          runs
            .iter()
            .any(|run| run.text == "■" && run.font_id.0.contains("TimesNewRoman"))
        );
        // Checkboxes, miscellaneous symbols and default-emoji squares keep
        // their independent generic/presentation fallback in both policies.
        for text in ["★", "☐", "◽", "◾"] {
          let word = resolver.shape_text_runs(text, &style).unwrap();
          let ordinary = resolver
            .shape_text_runs(
              text,
              &TextStyle {
                wordprocessingml_symbol_fallback: false,
                ..style.clone()
              },
            )
            .unwrap();
          assert_eq!(word.len(), ordinary.len());
          // Diagnostic fallback levels count candidates, including the new
          // range-limited link. Compare the actual faces and positioned glyphs.
          for (word, ordinary) in word.iter().zip(&ordinary) {
            assert_eq!(
              word.font_id, ordinary.font_id,
              "{family}, {bold}/{italic}, {text}"
            );
            assert_eq!(word.font_size_pt, ordinary.font_size_pt);
            assert_eq!(word.glyphs, ordinary.glyphs);
            assert_eq!(word.advance_pt, ordinary.advance_pt);
            assert_eq!(word.direction, ordinary.direction);
          }
        }
      }
    }
    for family in [
      "Arial",
      "Tahoma",
      "Cambria",
      "Courier New",
      "Times New Roman",
      "Segoe UI",
    ] {
      let style = TextStyle {
        font_family: Some(Arc::from(family)),
        wordprocessingml_symbol_fallback: true,
        ..Default::default()
      };
      let word = resolver.shape_text_runs("■", &style).unwrap();
      let ordinary = resolver
        .shape_text_runs(
          "■",
          &TextStyle {
            wordprocessingml_symbol_fallback: false,
            ..style
          },
        )
        .unwrap();
      assert_eq!(word, ordinary, "covered primary {family}");
    }
  }

  #[test]
  fn word_symbol_fallback_survives_warm_width_font_and_common_adapter_caches() {
    use super::FontStyleRef;

    for first in [false, true] {
      let mut resolver = super::FontResolver::default();
      let mut metrics = crate::text_metrics::TextMetrics::new();
      let mut widths = Vec::new();
      for word in [first, !first, first, !first] {
        let style = TextStyle {
          font_family: Some(Arc::from("Verdana")),
          wordprocessingml_font_slots: true,
          wordprocessingml_symbol_fallback: word,
          ..Default::default()
        };
        let expected = if word {
          "TimesNewRoman"
        } else {
          "SegoeUISymbol"
        };
        let shaped = resolver.shape_text_runs("■", &style).unwrap();
        assert!(shaped[0].font_id.0.contains(expected), "{shaped:?}");
        let common = Box::new(crate::model::common_text_style(style.clone()));
        assert_eq!(common.wordprocessingml_symbol_fallback(), word);
        assert_eq!(resolver.shape_text_runs("■", &common).unwrap(), shaped);
        let width = metrics.measure_text("■", &style);
        assert_eq!(metrics.measure_text("■", &common), width);
        widths.push(width);
      }
      assert_ne!(widths[0], widths[1]);
      assert_eq!(widths[0], widths[2]);
      assert_eq!(widths[1], widths[3]);
    }
  }

  #[test]
  fn word_han_fallback_preserves_primary_cjk_faces_and_their_missing_glyph_links() {
    use super::FontResolver;

    let mut resolver = FontResolver::default();
    let base = TextStyle {
      font_family: Some(Arc::from("Garamond")),
      high_ansi_font_family: Some(Arc::from("Garamond")),
      wordprocessingml_font_slots: true,
      wordprocessingml_han_fallback: true,
      language: Some(Arc::from("it-IT")),
      ..Default::default()
    };
    for family in [
      "Calibri",
      "Arial",
      "Garamond",
      "Times New Roman",
      "Cambria",
      "Consolas",
      "Aptos",
      "Courier New",
      "Tahoma",
      "Segoe UI",
      "Microsoft Sans Serif",
      "Liberation Sans",
    ] {
      for language in ["en-US", "zh-CN", "zh-TW", "ja-JP"] {
        for (bold, italic) in [(false, false), (true, false), (false, true)] {
          let style = TextStyle {
            east_asia_font_family: Some(Arc::from(family)),
            east_asia_language: Some(Arc::from(language)),
            bold,
            italic,
            ..base.clone()
          };
          let runs = resolver.shape_text_runs("地址块地址區塊", &style).unwrap();
          assert!(
            runs.iter().all(|run| run.font_id.0.contains("SimSun")
              && !run.approximate
              && run.glyphs.iter().all(|glyph| glyph.glyph_id != 0)),
            "{family}, {language}, {bold}/{italic}: {runs:?}"
          );
          assert_eq!(style.east_asia_font_family.as_deref(), Some(family));
        }
      }
    }
    for family in [
      "DengXian",
      "Microsoft YaHei",
      "SimSun",
      "NSimSun",
      "MingLiU",
    ] {
      let style = TextStyle {
        east_asia_font_family: Some(Arc::from(family)),
        ..base.clone()
      };
      let runs = resolver.shape_text_runs("地址块地址區塊", &style).unwrap();
      let name = family.replace(' ', "");
      assert!(
        runs
          .iter()
          .all(|run| run.font_id.0.contains(&name) && !run.approximate),
        "{family}: {runs:?}"
      );
    }
    let style = TextStyle {
      east_asia_font_family: Some(Arc::from("MS Mincho")),
      ..base
    };
    let runs = resolver.shape_text_runs("地址块地址區塊", &style).unwrap();
    assert!(
      runs
        .iter()
        .any(|run| run.text == "块" && run.font_id.0.contains("MicrosoftYaHei")),
      "Native MS Mincho missing-character counterexample: {runs:?}"
    );
    assert!(
      runs
        .iter()
        .filter(|run| run.text != "块")
        .all(|run| run.font_id.0.contains("MS-Mincho") || run.font_id.0.contains("MSMincho")),
      "covered Mincho glyphs stay on Mincho: {runs:?}"
    );
  }

  #[test]
  fn word_hangul_font_links_preserve_primary_faces_and_separate_cached_policies() {
    let mut resolver = super::FontResolver::default();
    for (family, expected) in [
      ("Calibri", "Batang"),
      ("Times New Roman", "Batang"),
      ("SimSun", "Batang"),
      ("DengXian", "Batang"),
      ("MS PMincho", "Batang"),
      ("MS Mincho", "MalgunGothic"),
      ("Meiryo", "MalgunGothic"),
      ("Microsoft YaHei", "MalgunGothic"),
      ("Malgun Gothic", "MalgunGothic"),
      ("Batang", "Batang"),
    ] {
      for word_first in [true, false] {
        for word in [word_first, !word_first, word_first] {
          let style = TextStyle {
            font_family: Some(Arc::from("Calibri")),
            high_ansi_font_family: Some(Arc::from("Calibri")),
            east_asia_font_family: Some(Arc::from(family)),
            wordprocessingml_font_slots: true,
            wordprocessingml_han_fallback: word,
            ..Default::default()
          };
          let expected = if word || family == "Batang" {
            expected
          } else {
            "MalgunGothic"
          };
          for common in [false, true] {
            let runs = if common {
              resolver.shape_text_runs("구문오류", &crate::model::common_text_style(style.clone()))
            } else {
              resolver.shape_text_runs("구문오류", &style)
            }
            .unwrap();
            assert!(
              runs.iter().all(|run| run.font_id.0.contains(expected)
                && !run.approximate
                && run.glyphs.iter().all(|glyph| glyph.glyph_id != 0)),
              "{family},Word={word},common={common}: {runs:?}"
            );
          }
          assert!(
            resolver.shape_text_runs("ABC", &style).unwrap()[0]
              .font_id
              .0
              .contains("Calibri")
          );
        }
      }
    }
  }

  #[test]
  fn word_han_fallback_cache_distinguishes_latin_slots_and_common_adapters() {
    use super::{FontResolver, FontStyleRef};

    // MS-OI29500 2.1.88: EA=Times New Roman with equal ascii/hAnsi selects
    // the Latin face. Exercise missing-Han fallback after that slot choice;
    // a Latin-only EA face instead maps to SimSun before glyph fallback.
    // Alternate policies in both orders through one resolver/width cache.
    for word_first in [false, true] {
      let mut resolver = FontResolver::default();
      let mut metrics = crate::text_metrics::TextMetrics::new();
      let mut style = TextStyle {
        font_family: Some(Arc::from("Calibri")),
        high_ansi_font_family: Some(Arc::from("Calibri")),
        east_asia_font_family: Some(Arc::from("Times New Roman")),
        wordprocessingml_font_slots: true,
        ..Default::default()
      };
      for word in [word_first, !word_first, word_first, !word_first] {
        style.wordprocessingml_han_fallback = word;
        let expected = if word { "SimSun" } else { "MicrosoftYaHei" };
        let runs = resolver.shape_text_runs("地址块", &style).unwrap();
        assert!(
          runs.iter().all(|run| run.font_id.0.contains(expected)),
          "Word={word}: {runs:?}"
        );
        let common = crate::model::common_text_style(style.clone());
        let boxed = Box::new(common.clone());
        assert_eq!(boxed.wordprocessingml_han_fallback(), word);
        let common_runs = resolver.shape_text_runs("地址块", &boxed).unwrap();
        assert!(
          common_runs
            .iter()
            .all(|run| run.font_id.0.contains(expected))
        );
        let width = metrics.measure_text("地址块", &style);
        assert_eq!(width, metrics.measure_text("地址块", &common));
        assert!(
          resolver.shape_text_runs("ABC", &style).unwrap()[0]
            .font_id
            .0
            .contains("Calibri")
        );
      }
    }
  }

  #[test]
  fn wordprocessingml_unhinted_east_asia_ranges_map_latin_faces() {
    use super::FontResolver;
    use ooxmlsdk_fonts::WordprocessingFontTypeHint;

    let mut resolver = FontResolver::default();
    for family in ["Calibri", "Arial", "MS Gothic"] {
      for hint in [None, Some(WordprocessingFontTypeHint::EastAsia)] {
        let style = TextStyle {
          font_family: Some(Arc::from(family)),
          high_ansi_font_family: Some(Arc::from(family)),
          east_asia_font_family: Some(Arc::from(family)),
          wordprocessingml_font_slots: true,
          wordprocessingml_font_hint: hint,
          ..Default::default()
        };
        let latin = resolver.shape_text_runs("A", &style).unwrap();
        for text in ["（ＡＢＣ，１２３）．", "カナ"] {
          let runs = resolver.shape_text_runs(text, &style).unwrap();
          assert!(
            runs.iter().all(|run| if family == "MS Gothic" {
              run.font_id == latin[0].font_id
            } else {
              run.font_id.0.contains("SimSun")
            }),
            "{family} {hint:?} {text}: {runs:?}"
          );
        }
        // Hint-free Greek/Cyrillic remain in hAnsi even though the same run's
        // fullwidth characters use eastAsia. Do not force a hint on the run.
        if hint.is_none() && family != "MS Gothic" {
          let runs = resolver.shape_text_runs("ΩА", &style).unwrap();
          assert!(runs.iter().all(|run| !run.font_id.0.contains("SimSun")));
        }
      }
    }
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
      .font_metrics_for_slot(
        &style,
        Some(TextScript::Common),
        Some(WordprocessingFontSlot::Ascii),
      )
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
    // Fixed East Asian ranges use this slot without a hint too. The cache
    // must separate actual slots, rather than invent a Latin EA-slot result.
    assert_eq!(
      resolver
        .font_metrics_for_slot(&style, Some(TextScript::Common), slot)
        .unwrap()
        .vertical,
      mapped
    );
    assert_eq!(
      resolver
        .font_metrics_for_slot(
          &style,
          Some(TextScript::Common),
          Some(WordprocessingFontSlot::Ascii),
        )
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
      Some(WordprocessingFontSlot::Ascii)
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

  fn retained_font_table_registry() -> ooxmlsdk_fonts::FontRegistry<'static> {
    use ooxmlsdk_fonts::{FontFaceInfo, FontFamilyClass, FontPitch, FontRegistry, FontSource};
    let mut registry = FontRegistry::new();
    for (family, class, pitch, charset) in [
      (
        "Liberation Serif",
        FontFamilyClass::Serif,
        FontPitch::Variable,
        FontCharset::Ansi,
      ),
      (
        "Calibri",
        FontFamilyClass::SansSerif,
        FontPitch::Variable,
        FontCharset::Ansi,
      ),
      (
        "Times New Roman",
        FontFamilyClass::Serif,
        FontPitch::Variable,
        FontCharset::Ansi,
      ),
      (
        "Arial",
        FontFamilyClass::SansSerif,
        FontPitch::Variable,
        FontCharset::Ansi,
      ),
      (
        "MS PMincho",
        FontFamilyClass::Serif,
        FontPitch::Variable,
        FontCharset::ShiftJis,
      ),
      (
        "MS PGothic",
        FontFamilyClass::SansSerif,
        FontPitch::Variable,
        FontCharset::ShiftJis,
      ),
      (
        "MS Mincho",
        FontFamilyClass::Serif,
        FontPitch::Fixed,
        FontCharset::ShiftJis,
      ),
    ] {
      let mut face = FontFaceInfo::synthetic(family, family);
      face.family_class = Some(class);
      face.pitch = pitch;
      // This test supplies registered metadata, not a fake binary font or a
      // dependency on the machine's installed fonts. No file is loaded.
      registry.register_face(
        FontSource::Path {
          id: Cow::Borrowed(family),
          path: std::path::PathBuf::new(),
          data: None,
        },
        face,
      );
      registry.faces.last_mut().unwrap().charset = Some(charset);
    }
    registry
  }

  fn font_table_os2_metadata(version: u16, code_page_range: u32) -> Vec<u8> {
    let table_size = if version == 0 { 78usize } else { 86 };
    let mut data = vec![0; 28 + table_size.next_multiple_of(4)];
    data[..4].copy_from_slice(&0x0001_0000u32.to_be_bytes());
    data[4..6].copy_from_slice(&1u16.to_be_bytes());
    data[6..8].copy_from_slice(&16u16.to_be_bytes());
    data[12..16].copy_from_slice(b"OS/2");
    data[20..24].copy_from_slice(&28u32.to_be_bytes());
    data[24..28].copy_from_slice(&(table_size as u32).to_be_bytes());
    data[28..30].copy_from_slice(&version.to_be_bytes());
    if version != 0 {
      data[106..110].copy_from_slice(&code_page_range.to_be_bytes());
    }
    let checksum = data[28..]
      .as_chunks::<4>()
      .0
      .iter()
      .fold(0u32, |sum, bytes| {
        sum.wrapping_add(u32::from_be_bytes(*bytes))
      });
    data[16..20].copy_from_slice(&checksum.to_be_bytes());
    data
  }

  #[test]
  fn word_font_table_retained_charset_uses_physical_os2_before_registered_charset() {
    use ooxmlsdk_fonts::FontSource;
    for (version, code_pages, registered_charset, expected) in [
      (1, 1 << 17, FontCharset::Ansi, Some(true)),
      (1, 0, FontCharset::ShiftJis, Some(false)),
      (0, 0, FontCharset::Ansi, None),
    ] {
      let mut registry = retained_font_table_registry();
      let data = font_table_os2_metadata(version, code_pages);
      let parsed = super::FontRef::from_index(&data, 0).unwrap();
      use skrifa::raw::TableProvider;
      assert_eq!(parsed.os2().unwrap().version(), version);
      assert_eq!(
        parsed.os2().unwrap().ul_code_page_range_1(),
        (version != 0).then_some(code_pages)
      );
      registry.faces[0].source = FontSource::Memory {
        id: Cow::Borrowed("Liberation Serif"),
        data: data.into(),
      };
      registry.faces[0].charset = Some(registered_charset);
      assert_eq!(
        super::registered_font_has_shift_jis_charset(
          &registry,
          &FontId(Arc::from("Liberation Serif"))
        ),
        expected
      );
    }
  }

  #[test]
  fn word_font_table_retained_shift_jis_matches_native_installed_controls() {
    use ooxmlsdk_fonts::{FontFamilyClass as Class, FontPitch as Pitch};
    let registry = retained_font_table_registry();
    // Native fontTable readbacks distinguish retained SHIFTJIS records from
    // rebuilt family/pitch records. v1-09 is a separate missing-face owner.
    for (control, family, charset, class, pitch, alternate, expected) in [
      (
        "v1-00 ANSI",
        "Liberation Serif",
        Some(FontCharset::Ansi),
        Class::Serif,
        Pitch::Variable,
        Some("Times New Roman"),
        "Liberation Serif",
      ),
      (
        "v1-01 DEFAULT",
        "Liberation Serif",
        Some(FontCharset::Other(1)),
        Class::Serif,
        Pitch::Variable,
        Some("Times New Roman"),
        "Liberation Serif",
      ),
      (
        "v1-02 charset absent",
        "Liberation Serif",
        None,
        Class::Serif,
        Pitch::Variable,
        Some("Times New Roman"),
        "Liberation Serif",
      ),
      (
        "v1-03 alternate absent",
        "Liberation Serif",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Variable,
        None,
        "MS PMincho",
      ),
      (
        "v1-04 alternate Arial",
        "Liberation Serif",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Variable,
        Some("Arial"),
        "MS PMincho",
      ),
      (
        "v1-05 mismatched Swiss",
        "Liberation Serif",
        Some(FontCharset::ShiftJis),
        Class::SansSerif,
        Pitch::Variable,
        Some("Times New Roman"),
        "Liberation Serif",
      ),
      (
        "v1-06 mismatched fixed",
        "Liberation Serif",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Fixed,
        Some("Times New Roman"),
        "Liberation Serif",
      ),
      (
        "v1-07 mismatched Calibri Roman",
        "Calibri",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Variable,
        Some("Times New Roman"),
        "Calibri",
      ),
      (
        "v1-08 physical SHIFTJIS",
        "MS PMincho",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Variable,
        Some("Times New Roman"),
        "MS PMincho",
      ),
      (
        "v2-00 retained repeat",
        "Liberation Serif",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Variable,
        None,
        "MS PMincho",
      ),
      (
        "v2-01 Swiss repeat",
        "Liberation Serif",
        Some(FontCharset::ShiftJis),
        Class::SansSerif,
        Pitch::Variable,
        Some("Times New Roman"),
        "Liberation Serif",
      ),
      (
        "v2-02 fixed repeat",
        "Liberation Serif",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Fixed,
        Some("Times New Roman"),
        "Liberation Serif",
      ),
      (
        "v2-03 Calibri Roman repeat",
        "Calibri",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Variable,
        Some("Times New Roman"),
        "Calibri",
      ),
      (
        "v2-04 retained Calibri Swiss",
        "Calibri",
        Some(FontCharset::ShiftJis),
        Class::SansSerif,
        Pitch::Variable,
        Some("Times New Roman"),
        "MS PGothic",
      ),
      (
        "v2-05 mode15 same font metadata",
        "Liberation Serif",
        Some(FontCharset::ShiftJis),
        Class::Serif,
        Pitch::Variable,
        None,
        "MS PMincho",
      ),
    ] {
      let style = TextStyle {
        font_family: Some(Arc::from(family)),
        font_charset: charset,
        font_family_class: Some(class),
        font_pitch: Some(pitch),
        fallback_font_family: alternate.map(Arc::from),
        wordprocessingml_font_slots: true,
        ..Default::default()
      };
      let slot = Some(WordprocessingFontSlot::Ascii);
      let mut request = super::font_request_for_slot(&style, Some(TextScript::Latin), slot);
      // The generic resolver remains name-first. Only Word's primary request
      // changes; retained document alternates are not rewritten globally.
      assert_eq!(
        registry.resolve(&request).unwrap().resolved_family,
        family,
        "{control}"
      );
      super::apply_wordprocessingml_retained_font_charset(&mut request, &registry, &style, slot);
      assert_eq!(
        registry.resolve(&request).unwrap().resolved_family,
        expected,
        "{control}"
      );
      assert_eq!(
        style.fallback_font_family.as_deref(),
        alternate,
        "{control}"
      );
    }
  }

  #[test]
  fn word_font_table_retained_charset_preserves_generic_and_unknown_records() {
    use ooxmlsdk_fonts::{FontFamilyClass, FontPitch};
    let baseline = TextStyle {
      font_family: Some(Arc::from("Liberation Serif")),
      font_charset: Some(FontCharset::ShiftJis),
      font_family_class: Some(FontFamilyClass::Serif),
      font_pitch: Some(FontPitch::Variable),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    for boundary in [
      "generic",
      "class absent",
      "pitch absent",
      "physical class unknown",
      "physical charset unknown",
    ] {
      let mut style = baseline.clone();
      let mut registry = retained_font_table_registry();
      match boundary {
        "generic" => style.wordprocessingml_font_slots = false,
        "class absent" => style.font_family_class = None,
        "pitch absent" => style.font_pitch = None,
        "physical class unknown" => registry.book.faces[0].family_class = None,
        "physical charset unknown" => registry.faces[0].charset = None,
        _ => unreachable!(),
      }
      let slot = Some(WordprocessingFontSlot::Ascii);
      let mut request = super::font_request_for_slot(&style, Some(TextScript::Latin), slot);
      let before = request.clone();
      super::apply_wordprocessingml_retained_font_charset(&mut request, &registry, &style, slot);
      assert_eq!(request, before, "{boundary}");
      assert_eq!(
        registry.resolve(&request).unwrap().resolved_family,
        "Liberation Serif",
        "{boundary}"
      );
    }
  }

  #[test]
  fn word_font_table_retained_charset_preserves_missing_face_and_target_ownership() {
    use ooxmlsdk_fonts::{FontFamilyClass, FontPitch};
    let style = TextStyle {
      font_family: Some(Arc::from("Liberation Serif")),
      font_charset: Some(FontCharset::ShiftJis),
      font_family_class: Some(FontFamilyClass::Serif),
      font_pitch: Some(FontPitch::Variable),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let slot = Some(WordprocessingFontSlot::Ascii);
    let mut registry = retained_font_table_registry();
    let mut request = super::font_request_for_slot(&style, Some(TextScript::Latin), slot);
    registry
      .book
      .faces
      .retain(|face| !face.family_names.iter().any(|name| name == "MS PMincho"));
    super::apply_wordprocessingml_retained_font_charset(&mut request, &registry, &style, slot);
    assert_eq!(request.family.as_deref(), Some("Liberation Serif"));
    // Native v1-09 selects MS Mincho through a distinct missing-record path.
    // This installed-face mapper must not take ownership of that request.
    request.family = Some(Cow::Borrowed("OOXMLSDK Missing Serif 47669"));
    let before = request.clone();
    super::apply_wordprocessingml_retained_font_charset(&mut request, &registry, &style, slot);
    assert_eq!(request, before);
  }

  #[test]
  fn word_font_table_retained_charset_does_not_alias_glyph_links() {
    use ooxmlsdk_fonts::{FontFallbackChain, FontFamilyClass, FontPitch};
    let mut registry = retained_font_table_registry();
    registry.book.fallback_chains.push(FontFallbackChain {
      families: vec![Cow::Borrowed("Liberation Serif")],
      ..Default::default()
    });
    let style = TextStyle {
      font_family: Some(Arc::from("Liberation Serif")),
      font_charset: Some(FontCharset::ShiftJis),
      font_family_class: Some(FontFamilyClass::Serif),
      font_pitch: Some(FontPitch::Variable),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let slot = Some(WordprocessingFontSlot::Ascii);
    let mut request = super::font_request_for_slot(&style, Some(TextScript::Latin), slot);
    super::apply_wordprocessingml_retained_font_charset(&mut request, &registry, &style, slot);
    let chain = registry.resolve_font_chain(&request).unwrap();
    let families = chain
      .resolved_fonts()
      .map(|font| font.resolved_family.as_ref())
      .collect::<Vec<_>>();
    assert_eq!(families, ["MS PMincho", "Liberation Serif"]);
    let original = super::font_request_for_slot(&style, Some(TextScript::Latin), slot);
    assert_eq!(
      registry.resolve(&original).unwrap().resolved_family,
      "Liberation Serif"
    );
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
    // An odd paragraph level does not itself force w:rtl font formatting.
    // The leading ASCII punctuation and Arabic retain separate physical
    // faces, and their complete run sequence must follow the visual level.
    let text = "; اطفال ";
    let rtl_style = TextStyle {
      font_family: Some(Arc::from("Cambria")),
      high_ansi_font_family: Some(Arc::from("Cambria")),
      complex_font_family: Some(Arc::from("Times New Roman")),
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
      vec![text.len() - 1..text.len(), 2..text.len() - 1, 0..2]
    );
    assert!(
      rtl
        .iter()
        .all(|run| run.direction == TextDirection::RightToLeft)
    );

    // ECMA-376 §17.3.2.30 and MS-OI29500 §2.1.88: explicit w:rtl also
    // selects the cs face for the punctuation, as in Comment066.docx.
    let forced = shape_text_runs(
      text,
      &TextStyle {
        right_to_left: Some(true),
        ..rtl_style.clone()
      },
    )
    .expect("explicit rtl formatting");
    assert_eq!(forced.len(), 1);
    assert_eq!(forced[0].text_range, 0..text.len());
    let cs_face = load_text_face(&TextStyle {
      font_family: Some(Arc::from("Times New Roman")),
      ..Default::default()
    })
    .expect("explicit cs face");
    assert_eq!(forced[0].font_id.0.as_ref(), cs_face.id());
    assert_eq!(forced[0].direction, TextDirection::RightToLeft);

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
      vec![0..2, 2..text.len() - 1, text.len() - 1..text.len()]
    );
  }

  #[test]
  fn native_simple_latin_spans_are_separate_from_arabic_glyph_calls() {
    // Native ordinary controls call GetGlyphPlacements on each Arabic word,
    // excluding ASCII punctuation/SP. CS, RTL and literal RLO call it on the
    // complete region instead. Uniform faces rule out a font-slot difference.
    for source in ["اب \" اب", "اب ( اب", "اب\" اب", "اب / اب", "اب - اب"] {
      for simple in [false, true] {
        let style = TextStyle {
          font_family: Some(Arc::from("Calibri")),
          complex_font_family: Some(Arc::from("Calibri")),
          font_size_pt: 11.0,
          wordprocessingml_font_slots: true,
          resolved_bidi_level: Some(1),
          wordprocessing_simple_latin_glyphs: Some(simple),
          ..Default::default()
        };
        let actual = shape_text_runs(source, &style).unwrap();
        if simple {
          let arabic = actual
            .iter()
            .filter(|r| r.script == Some(TextScript::Arabic))
            .map(|r| &source[r.text_range.clone()])
            .collect::<Vec<_>>();
          assert_eq!(arabic, ["اب", "اب"], "{source:?}");
          assert!(
            actual
              .iter()
              .filter(|r| r.script == Some(TextScript::Latin))
              .all(|r| r.direction == TextDirection::LeftToRight)
          );
          if source.contains('(') {
            let native = shape_text_runs(
              "(",
              &TextStyle {
                resolved_bidi_level: Some(0),
                ..style.clone()
              },
            )
            .unwrap();
            let glyph = actual
              .iter()
              .flat_map(|r| r.glyphs.iter())
              .find(|g| g.source_char == Some('('))
              .unwrap();
            assert_eq!(glyph.glyph_id, native[0].glyphs[0].glyph_id);
          }
        } else {
          // A ShapedRun also preserves scanner/font boundaries; it need not
          // correspond one-to-one to a native API call. Every retained source
          // portion must use the complex RTL glyph protocol in this control.
          assert!(
            actual
              .iter()
              .all(|r| r.direction == TextDirection::RightToLeft)
          );
          assert_eq!(
            actual.iter().map(|r| r.text_range.len()).sum::<usize>(),
            source.len()
          );
        }
      }
    }
  }

  #[test]
  fn implicit_word_latin_glyphs_keep_nominal_punctuation_and_source_order() {
    // Actual uniform-Calibri native controls: a quote followed by SP retains
    // LTR order, and '(' keeps its nominal glyph. Explicit RTL/CS retains the
    // complex path. Literal RLO is excluded by the paragraph owner.
    for source in ["\" ", "(", "[", "/ ", "- ", ": "] {
      for cs in [false, true] {
        for rtl in [false, true] {
          let base = TextStyle {
            font_family: Some(Arc::from("Calibri")),
            complex_font_family: Some(Arc::from("Calibri")),
            font_size_pt: 11.0,
            complex_script: Some(cs),
            right_to_left: Some(rtl),
            wordprocessingml_font_slots: true,
            ..Default::default()
          };
          let direction = if cs || rtl { 1 } else { 0 };
          let expected = shape_text_runs(
            source,
            &TextStyle {
              resolved_bidi_level: Some(direction),
              ..base.clone()
            },
          )
          .unwrap();
          let actual = shape_text_runs(
            source,
            &TextStyle {
              resolved_bidi_level: Some(1),
              wordprocessing_simple_latin_glyphs: Some(true),
              ..base
            },
          )
          .unwrap();
          let glyphs = |runs: &[ShapedRun]| {
            runs
              .iter()
              .flat_map(|r| r.glyphs.iter())
              .map(|g| (g.glyph_id, g.text_range.clone()))
              .collect::<Vec<_>>()
          };
          assert_eq!(
            glyphs(&actual),
            glyphs(&expected),
            "{source:?} cs={cs} rtl={rtl}"
          );
          assert_eq!(actual[0].direction, expected[0].direction);
        }
      }
    }
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
  fn wordprocessingml_arabic_shaping_uses_bidi_language_independently_of_primary_language() {
    let mut resolver = super::FontResolver::default();
    let target = "يَعْبَؤُا";
    for primary in ["en-US", "fa-IR"] {
      for bidi in ["ar-SA", "fa-IR", "en-US", "ur-PK"] {
        for alternate in [None, Some(false), Some(true)] {
          let mut style = TextStyle {
            font_family: Some("Traditional Arabic".into()),
            complex_font_family: Some("Traditional Arabic".into()),
            font_size_pt: 15.0,
            complex_font_size_pt: Some(15.0),
            wordprocessingml_font_slots: true,
            right_to_left: Some(true),
            language: Some(primary.into()),
            bidi_language: Some(bidi.into()),
            ..Default::default()
          };
          style.open_type_features.contextual_alternates = alternate;
          let expected = if bidi == "fa-IR" {
            vec![166, 201, 410, 200, 422, 204, 480, 200, 186, 519]
          } else {
            vec![166, 201, 410, 200, 186, 422, 204, 480, 200, 186, 519]
          };
          let common = crate::model::common_text_style(style.clone());
          for (text, native) in [
            (target, expected),
            ("يُعَرّف", vec![187, 203, 450, 200, 480, 201, 186, 519]),
            ("الله", vec![614]),
            ("لله", vec![389]),
            ("اللّه", vec![203, 589, 166]),
          ] {
            let glyphs = |runs: Vec<ooxmlsdk_fonts::ShapedRun<'_, '_>>| {
              runs
                .iter()
                .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
                .collect::<Vec<_>>()
            };
            assert_eq!(
              glyphs(resolver.shape_text_runs(text, &style).unwrap()),
              native,
              "{primary}/{bidi}/{alternate:?}: {text}"
            );
            assert_eq!(
              glyphs(resolver.shape_text_runs(text, &common).unwrap()),
              native,
              "display adapter: {primary}/{bidi}/{alternate:?}: {text}"
            );
          }
          assert_eq!(
            super::font_request(&style, Some(TextScript::Arabic))
              .language
              .as_deref(),
            Some(bidi)
          );
          assert!(
            super::font_request(&style, Some(TextScript::Latin))
              .language
              .is_none()
          );
        }
      }
    }
  }

  #[test]
  fn wordprocessingml_arabic_language_follows_complex_run_properties() {
    let mut resolver = super::FontResolver::default();
    for bidi in ["ar-SA", "fa-IR"] {
      for rtl in [None, Some(false), Some(true)] {
        for cs in [None, Some(false), Some(true)] {
          let style = TextStyle {
            font_family: Some("Traditional Arabic".into()),
            complex_font_family: Some("Traditional Arabic".into()),
            font_size_pt: 15.0,
            complex_font_size_pt: Some(15.0),
            wordprocessingml_font_slots: true,
            right_to_left: rtl,
            complex_script: cs,
            language: Some("fa-IR".into()),
            bidi_language: Some(bidi.into()),
            resolved_bidi_level: Some(1),
            ..Default::default()
          };
          let complex = rtl == Some(true) || cs == Some(true);
          let expected = if complex && bidi == "fa-IR" {
            vec![166, 201, 410, 200, 422, 204, 480, 200, 186, 519]
          } else {
            vec![166, 201, 410, 200, 186, 422, 204, 480, 200, 186, 519]
          };
          let common = crate::model::common_text_style(style.clone());
          for proxy in [&style as &dyn super::FontStyleRef, &common] {
            let glyphs = resolver
              .shape_text_runs("يَعْبَؤُا", proxy)
              .unwrap()
              .iter()
              .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
              .collect::<Vec<_>>();
            assert_eq!(glyphs, expected, "{bidi}/{rtl:?}/{cs:?}");
            assert_eq!(
              super::font_request(proxy, Some(TextScript::Arabic))
                .language
                .as_deref(),
              complex.then_some(bidi)
            );
          }
        }
      }
    }
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
  fn wordprocessingml_simple_latin_uses_native_legacy_measurement_font() {
    // Independent Word MSLS before-paint arrays for BodySpacePreserve.
    // Modes11/12/14 select these legacy arrays only with Word97 enabled;
    // disabled-flag and mode15 controls retain the design-font arrays.
    // The same six font/size controls also match all95 printable ASCII values.
    let cases = [
      (
        "Arial",
        9.0,
        false,
        [
          24588, 20502, 20502, 18432, 24588, 20502, 20502, 18432, 20502, 24588, 12276, 20502,
          18432, 20502, 12276, 18432, 20502,
        ],
      ),
      (
        "Arial",
        11.0,
        false,
        [
          30052, 25058, 25058, 22528, 30052, 25058, 25058, 22528, 25058, 30052, 15004, 25058,
          22528, 25058, 15004, 22528, 25058,
        ],
      ),
      (
        "Times New Roman",
        9.0,
        false,
        [
          24588, 18432, 18432, 18432, 20502, 18432, 16362, 16362, 16362, 20502, 12276, 16362,
          14346, 16362, 12276, 18432, 16362,
        ],
      ),
      (
        "Times New Roman",
        13.0,
        false,
        [
          35516, 26624, 26624, 26624, 29614, 26624, 23634, 23634, 23634, 29614, 17732, 23634,
          20722, 23634, 17732, 26624, 23634,
        ],
      ),
      (
        "Calibri",
        9.0,
        false,
        [
          20052, 19440, 19368, 16686, 16938, 19368, 17658, 15588, 18342, 19044, 12852, 18342,
          14418, 18342, 12852, 16650, 18342,
        ],
      ),
      (
        "Calibri",
        12.0,
        false,
        [
          26736, 25920, 25824, 22248, 22584, 25824, 23544, 20784, 24456, 25392, 17136, 24456,
          19224, 24456, 17136, 22200, 24456,
        ],
      ),
      (
        "Arial",
        9.0,
        true,
        [
          24588, 20496, 20496, 18432, 24588, 20496, 20496, 18432, 20496, 24588, 12275, 20496,
          18432, 20496, 12275, 18432, 20496,
        ],
      ),
      (
        "Arial",
        11.0,
        true,
        [
          30052, 25051, 25051, 22528, 30052, 25051, 25051, 22528, 25051, 30052, 15003, 25051,
          22528, 25051, 15003, 22528, 25051,
        ],
      ),
      (
        "Times New Roman",
        9.0,
        true,
        [
          24588, 18432, 18432, 18432, 20496, 18432, 16367, 16367, 16367, 20496, 12275, 16367,
          14340, 16367, 12275, 18432, 16367,
        ],
      ),
      (
        "Times New Roman",
        13.0,
        true,
        [
          35516, 26624, 26624, 26624, 29605, 26624, 23642, 23642, 23642, 29605, 17731, 23642,
          20713, 23642, 17731, 26624, 23642,
        ],
      ),
      (
        "Calibri",
        9.0,
        true,
        [
          20054, 19427, 19353, 16699, 16957, 19353, 17657, 15593, 18358, 19058, 12865, 18358,
          14413, 18358, 12865, 16662, 18358,
        ],
      ),
      (
        "Calibri",
        12.0,
        true,
        [
          26738, 25903, 25804, 22265, 22609, 25804, 23543, 20791, 24477, 25411, 17154, 24477,
          19218, 24477, 17154, 22216, 24477,
        ],
      ),
    ];
    let mut resolver = super::FontResolver::default();
    for (family, size, legacy, expected) in cases.into_iter().cycle().take(24) {
      let style = TextStyle {
        font_family: Some(Arc::from(family)),
        font_size_pt: size,
        kerning_minimum_size_pt: Some(f32::INFINITY),
        ligatures: Some(OpenTypeLigatures::default()),
        wordprocessingml_font_slots: true,
        wordprocessing_legacy_font_measurement: Some(legacy),
        ..Default::default()
      };
      let runs = resolver
        .shape_text_runs("BodySpacePreserve", &style)
        .unwrap();
      let actual = runs
        .iter()
        .flat_map(|run| run.glyphs.iter())
        .map(|glyph| (glyph.x_advance_pt * 4096.0).round() as i32)
        .collect::<Vec<_>>();
      assert_eq!(actual, expected, "{family} {size} legacy={legacy}");
    }
  }

  #[test]
  fn wordprocessingml_implicit_arabic_owns_words_and_blanks_separately() {
    // Uniform-Calibri native controls: absent/false cs/rtl uses independent
    // words. Explicit cs/rtl retains the complete portion. Both modes keep
    // Arabic kern enabled; the 500-unit difference is across SP boundaries.
    let text = "السرير الصغير كان الطفل";
    let mut resolver = super::FontResolver::default();
    for (rtl, cs, native_units) in [
      (None, None, 16_658),
      (Some(false), None, 16_658),
      (Some(false), Some(false), 16_658),
      (Some(true), None, 16_158),
      (None, Some(true), 16_158),
      (Some(false), Some(true), 16_158),
    ] {
      let style = TextStyle {
        font_family: Some(Arc::from("Calibri")),
        complex_font_family: Some(Arc::from("Calibri")),
        font_size_pt: 11.0,
        complex_font_size_pt: Some(11.0),
        right_to_left: rtl,
        complex_script: cs,
        wordprocessingml_font_slots: true,
        wordprocessing_legacy_font_measurement: Some(false),
        ..Default::default()
      };
      let runs = resolver.shape_text_runs(text, &style).unwrap();
      let advance = runs.iter().map(|run| run.advance_pt).sum::<f32>();
      assert!((advance - native_units as f32 * 11.0 / 2048.0).abs() < 0.001);
      if native_units == 16_658 {
        let blank_advances = runs
          .iter()
          .flat_map(|run| run.glyphs.iter())
          .filter(|glyph| text.get(glyph.text_range.clone()) == Some(" "))
          .map(|glyph| glyph.x_advance_pt)
          .collect::<Vec<_>>();
        assert_eq!(blank_advances.len(), 3);
        assert!(
          blank_advances
            .iter()
            .all(|advance| (*advance - 463.0 * 11.0 / 2048.0).abs() < 0.001)
        );
      }
      // The same family in an ordinary non-Word client keeps normal GPOS.
      let ordinary = TextStyle {
        wordprocessingml_font_slots: false,
        ..style
      };
      let runs = resolver.shape_text_runs(text, &ordinary).unwrap();
      assert!(
        (runs.iter().map(|run| run.advance_pt).sum::<f32>() - 16_158.0 * 11.0 / 2048.0).abs()
          < 0.001
      );
    }
  }

  #[test]
  fn wordprocessingml_simple_latin_measurement_preserves_other_owners() {
    let base = TextStyle {
      font_family: Some(Arc::from("Arial")),
      font_size_pt: 9.0,
      kerning_minimum_size_pt: Some(f32::INFINITY),
      ligatures: Some(OpenTypeLigatures::default()),
      wordprocessingml_font_slots: true,
      wordprocessing_legacy_font_measurement: Some(true),
      ..Default::default()
    };
    let mut resolver = super::FontResolver::default();
    for variant in 0..6 {
      let mut style = base.clone();
      match variant {
        0 => style.kerning_minimum_size_pt = Some(0.0),
        1 => style.character_spacing_pt = 0.1,
        2 => style.horizontal_scale = Some(1.03),
        3 => style.small_caps = true,
        4 => style.wordprocessingml_font_slots = false,
        5 => style.wordprocessing_font_width_percent = Some(90),
        _ => unreachable!(),
      }
      let actual = resolver
        .shape_text_runs("AVATAR office 0123", &style)
        .unwrap();
      style.wordprocessing_legacy_font_measurement = None;
      let expected = resolver
        .shape_text_runs("AVATAR office 0123", &style)
        .unwrap();
      assert_eq!(actual.len(), expected.len());
      for (actual, expected) in actual.iter().zip(&expected) {
        assert_eq!(actual.glyphs, expected.glyphs, "variant {variant}");
      }
    }
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
  fn wordprocessingml_arabic_controls_end_joining_without_changing_source_ranges() {
    use super::FontResolver;

    let mut resolver = FontResolver::default();
    for family in ["Traditional Arabic", "Tahoma", "Arial"] {
      let style = TextStyle {
        font_family: Some(family.into()),
        complex_font_family: Some(family.into()),
        font_size_pt: 15.0,
        complex_font_size_pt: Some(15.0),
        right_to_left: Some(true),
        resolved_bidi_level: Some(1),
        wordprocessingml_font_slots: true,
        ..Default::default()
      };
      let signature = |runs: &[ShapedRun<'_, '_>]| {
        let glyphs = runs
          .iter()
          .flat_map(|run| run.glyphs.iter())
          .filter(|glyph| glyph.x_advance_pt != 0.0)
          .map(|glyph| glyph.glyph_id)
          .collect::<Vec<_>>();
        let advance = runs.iter().map(|run| run.advance_pt).sum::<f32>();
        (glyphs, advance)
      };
      let reference = resolver
        .shape_text_runs("فهرست\u{200c}نگاران", &style)
        .unwrap();
      let reference = signature(&reference);
      for control in ['\u{200b}', '\u{200e}', '\u{200f}', '\u{2060}'] {
        let text = format!("فهرست{control}نگاران");
        let runs = resolver.shape_text_runs(&text, &style).unwrap();
        let actual = signature(&runs);
        assert_eq!(actual.0, reference.0, "{family} {control:?}");
        assert!((actual.1 - reference.1).abs() < 0.0001);
        let mut ranges = runs
          .iter()
          .map(|run| {
            assert_eq!(run.text, &text[run.text_range.clone()]);
            run.text_range.clone()
          })
          .collect::<Vec<_>>();
        ranges.sort_by_key(|range| range.start);
        let mut start = 0;
        for range in ranges {
          assert_eq!(range.start, start);
          start = range.end;
        }
        assert_eq!(start, text.len());
      }
      // The join-causing control remains effective; other consumers retain
      // their Unicode shaping policy rather than Word's control boundaries.
      let joined = signature(&resolver.shape_text_runs("فهرستنگاران", &style).unwrap());
      let zwj = signature(
        &resolver
          .shape_text_runs("فهرست\u{200d}نگاران", &style)
          .unwrap(),
      );
      assert_eq!(zwj, joined);
      assert_ne!(joined.0, reference.0);
      let generic = TextStyle {
        wordprocessingml_font_slots: false,
        ..style
      };
      assert_eq!(
        signature(
          &resolver
            .shape_text_runs("فهرست\u{200f}نگاران", &generic)
            .unwrap()
        ),
        signature(&resolver.shape_text_runs("فهرستنگاران", &generic).unwrap())
      );
    }
  }

  #[test]
  fn wordprocessingml_arabic_context_stops_at_native_control_boundaries() {
    use super::FontResolver;
    use crate::common::TextShapingContext;

    let mut resolver = FontResolver::default();
    let style = TextStyle {
      font_family: Some("Traditional Arabic".into()),
      complex_font_family: Some("Traditional Arabic".into()),
      font_size_pt: 15.0,
      complex_font_size_pt: Some(15.0),
      right_to_left: Some(true),
      resolved_bidi_level: Some(1),
      wordprocessingml_font_slots: true,
      ..Default::default()
    };
    let glyphs = |resolver: &mut FontResolver, text: &str, style: &TextStyle| {
      resolver
        .shape_text_runs(text, style)
        .unwrap()
        .iter()
        .flat_map(|run| run.glyphs.iter().map(|glyph| glyph.glyph_id))
        .collect::<Vec<_>>()
    };
    let before = glyphs(&mut resolver, "فهرست", &style);
    let after = glyphs(&mut resolver, "نگاران", &style);
    for control in ['\u{200b}', '\u{200e}', '\u{200f}', '\u{2060}'] {
      let mut contextual = style.clone();
      contextual.shaping_context = Some(Arc::new(TextShapingContext {
        before: "".into(),
        after: format!("{control}نگاران").into(),
        leading_marks_only: false,
      }));
      assert_eq!(glyphs(&mut resolver, "فهرست", &contextual), before);
      contextual.shaping_context = Some(Arc::new(TextShapingContext {
        before: format!("فهرست{control}").into(),
        after: "".into(),
        leading_marks_only: false,
      }));
      assert_eq!(glyphs(&mut resolver, "نگاران", &contextual), after);
    }
    let contextual = TextStyle {
      shaping_context: Some(Arc::new(TextShapingContext {
        before: "".into(),
        after: "\u{200d}نگاران".into(),
        leading_marks_only: false,
      })),
      ..style
    };
    assert_ne!(glyphs(&mut resolver, "فهرست", &contextual), before);
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
  fn word_tracking_suppresses_requested_latin_ligatures_at_shaping() {
    // Actual Word glyph IDs for explicit standard/contextual ligatures,
    // zero and signed character pitch. The requested property stays enabled.
    const PLAIN: &[u32] = &[
      381, 296, 296, 349, 272, 286, 3, 258, 296, 296, 349, 374, 286, 3, 296, 367, 349, 336, 346,
      410, 3, 296, 349, 3, 296, 367, 3, 296, 410, 3, 296, 296, 349, 3, 296, 296, 3, 296, 349, 374,
    ];
    const LIGATURES: &[u32] = &[
      381, 312, 272, 286, 3, 258, 312, 374, 286, 3, 327, 349, 336, 346, 410, 3, 302, 3, 327, 3,
      332, 3, 312, 3, 299, 3, 302, 374,
    ];
    let source = "office affine flight fi fl ft ffi ff fin";
    for word in [false, true] {
      for pitch in [-0.1, 0.0, 0.1] {
        for enabled in [false, true] {
          let style = TextStyle {
            font_family: Some(Arc::from("Calibri")),
            font_size_pt: 11.0,
            kerning_minimum_size_pt: Some(1.0),
            character_spacing_pt: pitch,
            wordprocessingml_font_slots: word,
            ligatures: Some(OpenTypeLigatures {
              standard: enabled,
              contextual: enabled,
              ..Default::default()
            }),
            ..Default::default()
          };
          let runs = shape_text_runs(source, &style).expect("native control font");
          let glyphs = runs
            .iter()
            .flat_map(|r| r.glyphs.iter().map(|g| g.glyph_id))
            .collect::<Vec<_>>();
          let expected = if enabled && (!word || pitch == 0.0) {
            LIGATURES
          } else {
            PLAIN
          };
          assert_eq!(
            glyphs, expected,
            "Word={word} pitch={pitch} enabled={enabled}"
          );
        }
      }
    }
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
