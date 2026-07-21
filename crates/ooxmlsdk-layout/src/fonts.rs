use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use ooxmlsdk_fonts::{
  FontBytes, FontFallbackChain, FontId, FontRegistry, FontRequest, FontSize, ResolvedFontChain,
  ShapeOptions, ShapedRun, TextScript, script_direction_runs,
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
  fn fallback_font_family(&self) -> Option<&str> {
    None
  }
  fn east_asia_font_family(&self) -> Option<&str> {
    self.font_family()
  }
  fn complex_font_family(&self) -> Option<&str> {
    self.font_family()
  }
  fn font_size_pt(&self) -> f32;
  fn character_spacing_pt(&self) -> f32;
  fn baseline_shift_pt(&self) -> f32;
  fn bold(&self) -> bool;
  fn italic(&self) -> bool;
  fn small_caps(&self) -> bool;
}

impl FontStyleRef for TextStyle {
  fn font_family(&self) -> Option<&str> {
    self.font_family.as_deref()
  }

  fn fallback_font_family(&self) -> Option<&str> {
    self.fallback_font_family.as_deref()
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

  fn character_spacing_pt(&self) -> f32 {
    self.character_spacing_pt
  }

  fn baseline_shift_pt(&self) -> f32 {
    self.baseline_shift_pt
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
}

impl FontStyleRef for common::TextStyle<'_> {
  fn font_family(&self) -> Option<&str> {
    self.font_family.as_deref()
  }

  fn fallback_font_family(&self) -> Option<&str> {
    self.fallback_font_family.as_deref()
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

  fn character_spacing_pt(&self) -> f32 {
    self.character_spacing.0
  }

  fn baseline_shift_pt(&self) -> f32 {
    self.baseline_shift.0
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
      self.shape_text_runs_inner(text, style)
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

  pub fn decoration_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<ooxmlsdk_fonts::DecorationMetrics> {
    self
      .font_metrics(style, None)
      .map(|metrics| metrics.decoration)
  }

  fn font_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
  ) -> Option<FontMetrics> {
    if let Some((key, metrics)) = &self.last_font_metrics
      && key.matches_style(style, script)
    {
      return Some(*metrics);
    }
    let key = FontMetricsKey::from_style(style, script);
    if let Some(metrics) = self.font_metrics_cache.get(&key) {
      let metrics = *metrics;
      self.last_font_metrics = Some((key, metrics));
      return Some(metrics);
    }
    let request = font_request(style, script);
    let registry = self.style_font_registry(style, script);
    let resolved = registry.resolve(&request).ok()?;
    let metrics_at_size = resolved.metrics_at_size(FontSize(style.font_size_pt()));
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
  ) -> Option<Vec<ShapedRun<'text, 'static>>> {
    let script_runs =
      script_direction_runs(text, FontSize(style.font_size_pt()), style.small_caps());
    let mut output = Vec::with_capacity(script_runs.len());
    for script_run in script_runs {
      let key = FontFaceKey::from_style(style, Some(script_run.script));
      let registry = self.style_font_registry(style, Some(script_run.script));
      let mut request = font_request(style, Some(script_run.script));
      request.size_pt = script_run.size_pt;
      request.script = Some(script_run.script);
      let mut options = ShapeOptions::from_request(&request, script_run.direction);
      options.character_spacing_pt = style.character_spacing_pt();
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
        run.offset_text_range(script_run.text_range.start);
      }
      output.extend(runs);
    }
    Some(output)
  }

  fn style_font_registry(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
  ) -> Arc<FontRegistry<'static>> {
    if let Some((key, registry)) = &self.last_font_registry
      && key.matches_style(style, script)
    {
      return registry.clone();
    }
    let key = FontFaceKey::from_style(style, script);
    if let Some(registry) = self.font_registry_cache.get(&key) {
      let registry = registry.clone();
      self.last_font_registry = Some((key, registry.clone()));
      return registry;
    }
    let registry = Arc::new(build_style_font_registry(style, script));
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
  FontRequest {
    family: script_font_family(style, script)
      .filter(|family| !family.trim().is_empty())
      .map(Cow::Borrowed),
    bold: style.bold(),
    italic: style.italic(),
    size_pt: FontSize(style.font_size_pt()),
    script,
    ..FontRequest::default()
  }
}

fn script_font_family(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
) -> Option<&str> {
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

fn script_fallback_font_family(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
) -> Option<&str> {
  match script {
    None
    | Some(TextScript::Common | TextScript::Latin | TextScript::Cyrillic | TextScript::Greek) => {
      style.fallback_font_family()
    }
    _ => None,
  }
}

fn build_style_font_registry(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
) -> FontRegistry<'static> {
  font_timing("build style font registry", || {
    let mut request = font_request(style, script);
    request.script = script;
    let mut registry = FontRegistry::with_default_policy();
    if let (Some(requested_family), Some(fallback_family)) = (
      request.family.as_deref(),
      script_fallback_font_family(style, script),
    ) && !requested_family.eq_ignore_ascii_case(fallback_family)
    {
      // ECMA-376 Part 1 §21.1.2.5 requires DrawingML font substitution
      // when the requested typeface is unavailable. Keep the requested face
      // primary, but place the document-scoped substitute before generic
      // platform fallbacks.
      registry.book.fallback_chains.insert(
        0,
        FontFallbackChain {
          requested_family: Some(Cow::Owned(requested_family.to_string())),
          script,
          language: None,
          families: vec![Cow::Owned(fallback_family.to_string())],
        },
      );
    }
    let mut registered = registry
      .register_system_query_fonts(&request)
      .unwrap_or_default();
    if registered == 0 {
      registered += registry.register_office_fallback_path_font(&request);
    }
    if registered == 0 {
      let mut fallback_request = font_request(style, script);
      fallback_request.script = script;
      fallback_request.family = None;
      registered += registry
        .register_system_query_fonts(&fallback_request)
        .unwrap_or_default();
      if registered == 0 {
        registry.register_office_fallback_path_font(&fallback_request);
      }
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
  bold: bool,
  italic: bool,
  script: Option<TextScript>,
}

impl FontFaceKey {
  fn from_style(style: &(impl FontStyleRef + ?Sized), script: Option<TextScript>) -> Self {
    Self {
      family: script_font_family(style, script).map(str::to_string),
      fallback_family: script_fallback_font_family(style, script).map(str::to_string),
      bold: style.bold(),
      italic: style.italic(),
      script,
    }
  }

  fn matches_style(
    &self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
  ) -> bool {
    self.family.as_deref() == script_font_family(style, script)
      && self.fallback_family.as_deref() == script_fallback_font_family(style, script)
      && self.bold == style.bold()
      && self.italic == style.italic()
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
  bold: bool,
  italic: bool,
  script: Option<TextScript>,
  size_pt_bits: u32,
}

impl FontMetricsKey {
  fn from_style(style: &(impl FontStyleRef + ?Sized), script: Option<TextScript>) -> Self {
    Self {
      family: script_font_family(style, script).map(str::to_string),
      fallback_family: script_fallback_font_family(style, script).map(str::to_string),
      bold: style.bold(),
      italic: style.italic(),
      script,
      size_pt_bits: style.font_size_pt().to_bits(),
    }
  }

  fn matches_style(
    &self,
    style: &(impl FontStyleRef + ?Sized),
    script: Option<TextScript>,
  ) -> bool {
    self.family.as_deref() == script_font_family(style, script)
      && self.fallback_family.as_deref() == script_fallback_font_family(style, script)
      && self.bold == style.bold()
      && self.italic == style.italic()
      && self.script == script
      && self.size_pt_bits == style.font_size_pt().to_bits()
  }
}

pub fn cached_text_face(style: &(impl FontStyleRef + ?Sized)) -> Option<FontFaceData> {
  FontResolver::default().cached_text_face(style)
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use crate::docx::TextStyle;

  use super::load_text_face;

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
