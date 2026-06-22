use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use ooxmlsdk_fonts::{
  FontBytes, FontFaceData as SharedFontFaceData, FontId, FontRegistry, FontRequest, FontSize,
  ShapeOptions, ShapedRun, TextScript, script_direction_runs,
};
use rustc_hash::FxHashMap as HashMap;

use crate::common;
use crate::docx::TextStyle;

fn font_timing<T>(label: &str, work: impl FnOnce() -> T) -> T {
  if std::env::var_os("OOXMLSDK_FONT_TIMING").is_none() {
    return work();
  }
  let start = Instant::now();
  let output = work();
  eprintln!("[ooxmlsdk-pdf] {label}: {:?}", start.elapsed());
  output
}

#[derive(Clone, Debug)]
pub struct FontFaceData {
  pub data: Arc<FontBytes>,
  pub index: u32,
  id: Arc<str>,
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
  let request = font_request(style);
  let registry = style_font_registry(style, None);
  let resolved = registry.resolve(&request).ok()?;
  font_face_data_from_registry(&registry, &resolved.font_id)
}

pub fn shape_text_runs(
  text: &str,
  style: &(impl FontStyleRef + ?Sized),
) -> Option<Vec<ShapedRun<'static>>> {
  font_timing("shape text runs", || shape_text_runs_inner(text, style))
}

fn shape_text_runs_inner(
  text: &str,
  style: &(impl FontStyleRef + ?Sized),
) -> Option<Vec<ShapedRun<'static>>> {
  let mut output = Vec::new();
  for script_run in script_direction_runs(text, FontSize(style.font_size_pt()), style.small_caps())
  {
    let registry = style_font_registry(style, Some(script_run.script));
    let mut request = font_request(style);
    request.size_pt = script_run.size_pt;
    request.script = Some(script_run.script);
    let mut options = ShapeOptions::from_request(&request, script_run.direction);
    options.character_spacing_pt = style.character_spacing_pt();
    options.small_caps = false;
    options.scan_registered_fallbacks = false;
    let mut runs = registry
      .shape_text_runs_with_options(&request, Cow::Owned(script_run.text.into_owned()), &options)
      .ok()?;
    for run in &runs {
      let _ = font_face_data_from_registry(&registry, &run.font_id);
    }
    for run in &mut runs {
      run.offset_text_range(script_run.text_range.start);
    }
    output.extend(runs);
  }
  Some(output)
}

pub fn font_face_data(font_id: &FontId) -> Option<FontFaceData> {
  font_data_cache().lock().ok()?.get(font_id).cloned()
}

pub fn vertical_metrics(
  style: &(impl FontStyleRef + ?Sized),
) -> Option<ooxmlsdk_fonts::VerticalMetrics> {
  let request = font_request(style);
  let registry = style_font_registry(style, None);
  let resolved = registry.resolve(&request).ok()?;
  Some(
    resolved
      .metrics_at_size(FontSize(style.font_size_pt()))
      .vertical,
  )
}

pub fn decoration_metrics(
  style: &(impl FontStyleRef + ?Sized),
) -> Option<ooxmlsdk_fonts::DecorationMetrics> {
  let request = font_request(style);
  let registry = style_font_registry(style, None);
  let resolved = registry.resolve(&request).ok()?;
  Some(
    resolved
      .metrics_at_size(FontSize(style.font_size_pt()))
      .decoration,
  )
}

fn font_request(style: &(impl FontStyleRef + ?Sized)) -> FontRequest<'static> {
  FontRequest {
    family: style
      .font_family()
      .filter(|family| !family.trim().is_empty())
      .map(|family| Cow::Owned(family.to_string())),
    bold: style.bold(),
    italic: style.italic(),
    size_pt: FontSize(style.font_size_pt()),
    ..FontRequest::default()
  }
}

fn style_font_registry(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
) -> Arc<FontRegistry<'static>> {
  let key = FontFaceKey {
    family: style.font_family().map(str::to_string),
    bold: style.bold(),
    italic: style.italic(),
    script,
  };
  if let Ok(mut cache) = font_registry_cache().lock() {
    if let Some(registry) = cache.get(&key) {
      return registry.clone();
    }
    let registry = Arc::new(build_style_font_registry(style, script));
    cache.insert(key, registry.clone());
    return registry;
  }

  Arc::new(build_style_font_registry(style, script))
}

fn build_style_font_registry(
  style: &(impl FontStyleRef + ?Sized),
  script: Option<TextScript>,
) -> FontRegistry<'static> {
  font_timing("build style font registry", || {
    let mut request = font_request(style);
    request.script = script;
    let mut registry = FontRegistry::with_default_policy();
    let mut registered = registry
      .register_system_query_fonts(&request)
      .unwrap_or_default();
    if registered == 0 {
      registered += registry.register_office_fallback_path_font(&request);
    }
    if registered == 0 {
      let mut fallback_request = request.clone();
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

fn font_face_data_from_registry(
  registry: &FontRegistry<'static>,
  font_id: &FontId,
) -> Option<FontFaceData> {
  if let Ok(cache) = font_data_cache().lock()
    && let Some(face) = cache.get(font_id)
  {
    return Some(face.clone());
  }
  let face = registry.font_face_data(font_id)?;
  let face = font_face_data_from_shared(font_id, face)?;
  if let Ok(mut cache) = font_data_cache().lock() {
    cache.insert(font_id.clone(), face.clone());
  }
  Some(face)
}

fn font_face_data_from_shared(
  font_id: &FontId,
  face: SharedFontFaceData<'_>,
) -> Option<FontFaceData> {
  Some(FontFaceData {
    data: Arc::new(face.data?),
    index: face.face_index,
    id: font_id.0.clone(),
  })
}

fn font_data_cache() -> &'static Mutex<HashMap<FontId, FontFaceData>> {
  static CACHE: OnceLock<Mutex<HashMap<FontId, FontFaceData>>> = OnceLock::new();
  CACHE.get_or_init(|| Mutex::new(HashMap::default()))
}

fn font_registry_cache() -> &'static Mutex<HashMap<FontFaceKey, Arc<FontRegistry<'static>>>> {
  static CACHE: OnceLock<Mutex<HashMap<FontFaceKey, Arc<FontRegistry<'static>>>>> = OnceLock::new();
  CACHE.get_or_init(|| Mutex::new(HashMap::default()))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontFaceKey {
  family: Option<String>,
  bold: bool,
  italic: bool,
  script: Option<TextScript>,
}

impl FontFaceKey {
  fn from_style(style: &(impl FontStyleRef + ?Sized), script: Option<TextScript>) -> Self {
    Self {
      family: style.font_family().map(str::to_string),
      bold: style.bold(),
      italic: style.italic(),
      script,
    }
  }
}

fn font_face_cache() -> &'static Mutex<HashMap<FontFaceKey, FontFaceData>> {
  static CACHE: OnceLock<Mutex<HashMap<FontFaceKey, FontFaceData>>> = OnceLock::new();
  CACHE.get_or_init(|| Mutex::new(HashMap::default()))
}

pub fn cached_text_face(style: &(impl FontStyleRef + ?Sized)) -> Option<FontFaceData> {
  let key = FontFaceKey::from_style(style, None);
  if let Ok(cache) = font_face_cache().lock()
    && let Some(face) = cache.get(&key)
  {
    return Some(face.clone());
  }
  let face = load_text_face(style)?;
  if let Ok(mut cache) = font_face_cache().lock() {
    cache.insert(key, face.clone());
  }
  Some(face)
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
}
