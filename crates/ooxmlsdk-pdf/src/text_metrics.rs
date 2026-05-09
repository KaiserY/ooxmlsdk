use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use rustybuzz::{Direction, Face, Script, UnicodeBuffer};
use unicode_bidi::{Direction as BidiDirection, get_base_direction};
use unicode_script::{Script as UnicodeScriptValue, UnicodeScript};

use crate::docx::TextStyle;
use crate::fonts::{FontFaceData, load_text_face};

#[derive(Clone, Debug)]
pub(crate) struct ShapedText {
  pub glyphs: Vec<ShapedGlyph>,
  pub width_pt: f32,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TextVerticalMetrics {
  pub ascent_pt: f32,
  pub descent_pt: f32,
  pub line_gap_pt: f32,
}

impl TextVerticalMetrics {
  pub(crate) fn ink_height_pt(self) -> f32 {
    self.ascent_pt + self.descent_pt
  }

  pub(crate) fn line_height_pt(self) -> f32 {
    self.ink_height_pt() + self.line_gap_pt
  }

  pub(crate) fn leading_above_pt(self) -> f32 {
    self.line_gap_pt / 2.0
  }
}

#[derive(Clone, Debug)]
pub(crate) struct ShapedGlyph {
  pub glyph_id: u32,
  pub text_range: std::ops::Range<usize>,
  pub x_advance_em: f32,
  pub x_offset_em: f32,
  pub y_offset_em: f32,
  pub y_advance_em: f32,
}

pub(crate) fn measure_text(text: &str, style: &TextStyle) -> f32 {
  if text.is_empty() {
    return 0.0;
  }

  FontMetricsCache::get()
    .measure(text, style)
    .unwrap_or_else(|| approximate_text_width(text, style.font_size_pt))
}

pub(crate) fn shape_text(text: &str, style: &TextStyle) -> Option<ShapedText> {
  if text.is_empty() {
    return Some(ShapedText {
      glyphs: Vec::new(),
      width_pt: 0.0,
    });
  }

  FontMetricsCache::get().shape(text, style)
}

pub(crate) fn vertical_metrics(style: &TextStyle) -> TextVerticalMetrics {
  normalize_vertical_metrics(
    FontMetricsCache::get()
      .vertical_metrics(style)
      .unwrap_or_else(|| approximate_vertical_metrics(style.font_size_pt)),
    style.font_size_pt,
  )
}

pub(crate) fn inline_text_box_height(style: &TextStyle) -> f32 {
  vertical_metrics(style).line_height_pt() + style.baseline_shift_pt.abs()
}

pub(crate) fn baseline_offset_in_line(style: &TextStyle, line_height_pt: f32) -> f32 {
  let metrics = vertical_metrics(style);
  let natural_height_pt = metrics.line_height_pt() + style.baseline_shift_pt.abs();
  let extra_leading_pt = (line_height_pt - natural_height_pt).max(0.0) / 2.0;
  extra_leading_pt + metrics.leading_above_pt() + metrics.ascent_pt - style.baseline_shift_pt
}

fn approximate_text_width(text: &str, font_size: f32) -> f32 {
  text
    .chars()
    .map(|ch| match ch {
      ' ' => font_size * 0.28,
      '\t' => font_size * 1.12,
      ch if ch.is_ascii_punctuation() => font_size * 0.3,
      ch if ch.is_ascii() => font_size * 0.52,
      _ => font_size,
    })
    .sum()
}

fn approximate_vertical_metrics(font_size: f32) -> TextVerticalMetrics {
  TextVerticalMetrics {
    ascent_pt: font_size * 0.8,
    descent_pt: font_size * 0.2,
    line_gap_pt: font_size * 0.05,
  }
}

fn normalize_vertical_metrics(
  mut metrics: TextVerticalMetrics,
  font_size: f32,
) -> TextVerticalMetrics {
  let min_line_height_pt = font_size * 1.25;
  if metrics.line_height_pt() < min_line_height_pt {
    metrics.line_gap_pt += min_line_height_pt - metrics.line_height_pt();
  }
  metrics
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontKey {
  family: Option<String>,
  bold: bool,
  italic: bool,
}

#[derive(Default)]
struct FontMetricsCache {
  fonts: Mutex<HashMap<FontKey, Option<FontMetrics>>>,
}

impl FontMetricsCache {
  fn get() -> &'static Self {
    static METRICS: OnceLock<FontMetricsCache> = OnceLock::new();
    METRICS.get_or_init(Self::default)
  }

  fn measure(&self, text: &str, style: &TextStyle) -> Option<f32> {
    let font = self.select(style)?;
    let mut width = 0.0;
    for run in script_runs(text) {
      width += font.measure(
        run.text,
        style.font_size_pt,
        style.character_spacing_pt,
        run.script,
      )?;
    }
    Some(width)
  }

  fn shape(&self, text: &str, style: &TextStyle) -> Option<ShapedText> {
    let font = self.select(style)?;
    let mut glyphs = Vec::new();
    let mut width_pt = 0.0;
    for run in script_runs(text) {
      let shaped = font.shape(
        run.text,
        style.font_size_pt,
        style.character_spacing_pt,
        run.script,
        run.start,
      )?;
      width_pt += shaped.width_pt;
      glyphs.extend(shaped.glyphs);
    }
    Some(ShapedText { glyphs, width_pt })
  }

  fn vertical_metrics(&self, style: &TextStyle) -> Option<TextVerticalMetrics> {
    let font = self.select(style)?;
    Some(font.vertical_metrics(style.font_size_pt))
  }

  fn select(&self, style: &TextStyle) -> Option<FontMetrics> {
    let key = FontKey {
      family: style.font_family.as_deref().map(str::to_string),
      bold: style.bold,
      italic: style.italic,
    };
    let mut fonts = self.fonts.lock().ok()?;
    if let Some(font) = fonts.get(&key) {
      return font.clone();
    }
    let font = load_font_metrics(style);
    fonts.insert(key, font.clone());
    font
  }
}

#[derive(Clone, Debug)]
struct FontMetrics {
  face: FontFaceData,
  vertical: FaceVerticalMetrics,
}

impl FontMetrics {
  fn measure(
    &self,
    text: &str,
    font_size: f32,
    character_spacing_pt: f32,
    script: UnicodeScriptValue,
  ) -> Option<f32> {
    Some(
      self
        .shape(text, font_size, character_spacing_pt, script, 0)?
        .width_pt,
    )
  }

  fn shape(
    &self,
    text: &str,
    font_size: f32,
    character_spacing_pt: f32,
    script: UnicodeScriptValue,
    text_offset: usize,
  ) -> Option<ShapedText> {
    let face = Face::from_slice(&self.face.data, self.face.index)?;
    let units_per_em = face.units_per_em().max(1) as f32;
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);
    buffer.guess_segment_properties();
    if let Some(script) = rustybuzz_script(script) {
      buffer.set_script(script);
    }
    match bidi_direction(text) {
      BidiDirection::Rtl => buffer.set_direction(Direction::RightToLeft),
      BidiDirection::Ltr => buffer.set_direction(Direction::LeftToRight),
      BidiDirection::Mixed => {}
    }
    let shaped = rustybuzz::shape(&face, &[], buffer);
    let infos = shaped.glyph_infos();
    let positions = shaped.glyph_positions();
    let mut glyphs = Vec::with_capacity(infos.len());
    let mut width_pt = 0.0;
    let tracking_em = if font_size.abs() > f32::EPSILON {
      character_spacing_pt / font_size
    } else {
      0.0
    };

    for index in 0..infos.len() {
      let position = positions[index];
      let mut x_advance_em = position.x_advance as f32 / units_per_em;
      if tracking_em.abs() > f32::EPSILON && index + 1 < infos.len() {
        x_advance_em += tracking_em;
      }
      width_pt += x_advance_em * font_size;
      glyphs.push(ShapedGlyph {
        glyph_id: infos[index].glyph_id,
        text_range: glyph_text_range(text, infos, index, text_offset),
        x_advance_em,
        x_offset_em: position.x_offset as f32 / units_per_em,
        y_offset_em: position.y_offset as f32 / units_per_em,
        y_advance_em: position.y_advance as f32 / units_per_em,
      });
    }

    Some(ShapedText { glyphs, width_pt })
  }

  fn vertical_metrics(&self, font_size: f32) -> TextVerticalMetrics {
    let scale = font_size / self.vertical.units_per_em;
    TextVerticalMetrics {
      ascent_pt: self.vertical.ascent_units * scale,
      descent_pt: self.vertical.descent_units * scale,
      line_gap_pt: self.vertical.line_gap_units * scale,
    }
  }
}

fn load_font_metrics(style: &TextStyle) -> Option<FontMetrics> {
  load_text_face(style).and_then(|face| {
    let parsed = Face::from_slice(&face.data, face.index)?;
    Some(FontMetrics {
      vertical: FaceVerticalMetrics::from_face(&parsed),
      face,
    })
  })
}

#[derive(Clone, Copy, Debug)]
struct FaceVerticalMetrics {
  units_per_em: f32,
  ascent_units: f32,
  descent_units: f32,
  line_gap_units: f32,
}

impl FaceVerticalMetrics {
  fn from_face(face: &Face<'_>) -> Self {
    let units_per_em = face.units_per_em().max(1) as f32;
    let ascent_units = face.ascender().max(0) as f32;
    let descent_units = (-face.descender()).max(0) as f32;
    let line_gap_units = face.line_gap().max(0) as f32;
    let fallback_gap = (units_per_em - (ascent_units + descent_units)).max(0.0);
    Self {
      units_per_em,
      ascent_units,
      descent_units,
      line_gap_units: if line_gap_units > 0.0 {
        line_gap_units
      } else {
        fallback_gap
      },
    }
  }
}

#[derive(Clone, Copy, Debug)]
struct ScriptRun<'a> {
  text: &'a str,
  script: UnicodeScriptValue,
  start: usize,
}

fn script_runs(text: &str) -> Vec<ScriptRun<'_>> {
  let mut runs = Vec::new();
  let mut start = 0;
  let mut active = UnicodeScriptValue::Unknown;

  for (index, ch) in text.char_indices() {
    let script = stable_script(ch.script(), active);
    if index == 0 {
      active = script;
      continue;
    }
    if script != active {
      runs.push(ScriptRun {
        text: &text[start..index],
        script: active,
        start,
      });
      start = index;
      active = script;
    }
  }

  if start < text.len() {
    runs.push(ScriptRun {
      text: &text[start..],
      script: active,
      start,
    });
  }

  runs
}

fn stable_script(script: UnicodeScriptValue, active: UnicodeScriptValue) -> UnicodeScriptValue {
  match script {
    UnicodeScriptValue::Common | UnicodeScriptValue::Inherited | UnicodeScriptValue::Unknown
      if active != UnicodeScriptValue::Unknown =>
    {
      active
    }
    _ => script,
  }
}

fn rustybuzz_script(script: UnicodeScriptValue) -> Option<Script> {
  let tag = ttf_parser::Tag::from_bytes(script.short_name().as_bytes().try_into().ok()?);
  Script::from_iso15924_tag(tag)
}

fn bidi_direction(text: &str) -> BidiDirection {
  get_base_direction(text)
}

fn glyph_text_range(
  text: &str,
  infos: &[rustybuzz::GlyphInfo],
  index: usize,
  text_offset: usize,
) -> std::ops::Range<usize> {
  let cluster = infos[index].cluster as usize;
  let next_cluster = infos
    .iter()
    .enumerate()
    .filter_map(|(candidate_index, info)| {
      let candidate = info.cluster as usize;
      (candidate_index != index && candidate > cluster).then_some(candidate)
    })
    .min()
    .unwrap_or(text.len());
  text_offset + cluster.min(text.len())..text_offset + next_cluster.min(text.len())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn shaped_measurement_handles_ligatures_and_cjk() {
    let style = TextStyle::default();

    assert!(measure_text("office", &style) > 0.0);
    assert!(measure_text("商务文档", &style) > measure_text("abc", &style));
  }

  #[test]
  fn script_runs_keep_common_characters_with_surrounding_text() {
    let runs = script_runs("abc, 商务");

    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].text, "abc, ");
    assert_eq!(runs[0].start, 0);
    assert_eq!(runs[1].text, "商务");
    assert_eq!(runs[1].start, "abc, ".len());
  }

  #[test]
  fn shaped_text_exposes_glyph_advances_for_pdf_paint() {
    let style = TextStyle::default();
    let shaped = shape_text("office", &style).expect("shaped text");

    assert!(!shaped.glyphs.is_empty());
    assert!(shaped.width_pt > 0.0);
    assert!(
      shaped
        .glyphs
        .iter()
        .all(|glyph| glyph.text_range.end <= "office".len())
    );
  }
}
