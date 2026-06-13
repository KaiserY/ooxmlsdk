use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Instant;

use ooxmlsdk_fonts::{
  FontRegistry, FontRequest, FontSize, ShapeOptions, TextScript, script_direction_runs,
};

use crate::compat::TextStyle;

fn font_timing<T>(label: &str, work: impl FnOnce() -> T) -> T {
  if std::env::var_os("OOXMLSDK_FONT_TIMING").is_none() {
    return work();
  }
  let start = Instant::now();
  let output = work();
  eprintln!("[ooxmlsdk-layout] {label}: {:?}", start.elapsed());
  output
}

pub(crate) fn measure_text_width(text: &str, style: &TextStyle) -> Option<f32> {
  font_timing("measure text width", || {
    measure_text_width_inner(text, style)
  })
}

fn measure_text_width_inner(text: &str, style: &TextStyle) -> Option<f32> {
  let mut width = 0.0;
  for script_run in script_direction_runs(
    text.to_string(),
    FontSize(style.font_size_pt),
    style.small_caps,
  ) {
    let registry = style_font_registry(style, Some(script_run.script));
    let mut request = font_request(style);
    request.size_pt = script_run.size_pt;
    request.script = Some(script_run.script);
    let mut options = ShapeOptions::from_request(&request, script_run.direction);
    options.character_spacing_pt = style.character_spacing_pt;
    options.small_caps = false;
    options.scan_registered_fallbacks = false;
    let runs = registry
      .shape_text_runs_with_options(&request, script_run.text, &options)
      .ok()?;
    width += runs.iter().map(|run| run.advance_pt).sum::<f32>();
  }
  Some(width)
}

fn font_request(style: &TextStyle) -> FontRequest<'static> {
  FontRequest {
    family: style
      .font_family
      .as_deref()
      .filter(|family| !family.trim().is_empty())
      .map(|family| Cow::Owned(family.to_string())),
    bold: style.bold,
    italic: style.italic,
    size_pt: FontSize(style.font_size_pt),
    ..FontRequest::default()
  }
}

fn style_font_registry(
  style: &TextStyle,
  script: Option<TextScript>,
) -> Arc<FontRegistry<'static>> {
  let key = FontFaceKey {
    family: style.font_family.as_deref().map(str::to_string),
    bold: style.bold,
    italic: style.italic,
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
  style: &TextStyle,
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

fn font_registry_cache() -> &'static Mutex<HashMap<FontFaceKey, Arc<FontRegistry<'static>>>> {
  static CACHE: OnceLock<Mutex<HashMap<FontFaceKey, Arc<FontRegistry<'static>>>>> = OnceLock::new();
  CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontFaceKey {
  family: Option<String>,
  bold: bool,
  italic: bool,
  script: Option<TextScript>,
}
