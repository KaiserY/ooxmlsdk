use std::sync::OnceLock;

use rustybuzz::{Direction, Face, Script, UnicodeBuffer};
use unicode_bidi::{Direction as BidiDirection, get_base_direction};
use unicode_script::{Script as UnicodeScriptValue, UnicodeScript};

use crate::docx::TextStyle;
use crate::fonts::{FontFaceData, load_sans_face};

#[derive(Clone, Debug)]
pub(crate) struct ShapedText {
  pub glyphs: Vec<ShapedGlyph>,
  pub width_pt: f32,
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

pub(crate) fn measure_text(text: &str, style: TextStyle) -> f32 {
  if text.is_empty() {
    return 0.0;
  }

  FontMetricsSet::get()
    .and_then(|set| set.measure(text, style))
    .unwrap_or_else(|| approximate_text_width(text, style.font_size_pt))
}

pub(crate) fn shape_text(text: &str, style: TextStyle) -> Option<ShapedText> {
  if text.is_empty() {
    return Some(ShapedText {
      glyphs: Vec::new(),
      width_pt: 0.0,
    });
  }

  FontMetricsSet::get()?.shape(text, style)
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

#[derive(Clone, Debug)]
struct FontMetricsSet {
  regular: FontMetrics,
  bold: FontMetrics,
  italic: FontMetrics,
  bold_italic: FontMetrics,
}

impl FontMetricsSet {
  fn get() -> Option<&'static Self> {
    static METRICS: OnceLock<Option<FontMetricsSet>> = OnceLock::new();
    METRICS.get_or_init(Self::load).as_ref()
  }

  fn load() -> Option<Self> {
    let regular = load_font_metrics(false, false)?;
    let bold = load_font_metrics(true, false).unwrap_or_else(|| regular.clone());
    let italic = load_font_metrics(false, true).unwrap_or_else(|| regular.clone());
    let bold_italic = load_font_metrics(true, true).unwrap_or_else(|| bold.clone());

    Some(Self {
      regular,
      bold,
      italic,
      bold_italic,
    })
  }

  fn select(&self, style: TextStyle) -> &FontMetrics {
    match (style.bold, style.italic) {
      (true, true) => &self.bold_italic,
      (true, false) => &self.bold,
      (false, true) => &self.italic,
      (false, false) => &self.regular,
    }
  }

  fn measure(&self, text: &str, style: TextStyle) -> Option<f32> {
    let font = self.select(style);
    let mut width = 0.0;
    for run in script_runs(text) {
      width += font.measure(run.text, style.font_size_pt, run.script)?;
    }
    Some(width)
  }

  fn shape(&self, text: &str, style: TextStyle) -> Option<ShapedText> {
    let font = self.select(style);
    let mut glyphs = Vec::new();
    let mut width_pt = 0.0;
    for run in script_runs(text) {
      let shaped = font.shape(run.text, style.font_size_pt, run.script, run.start)?;
      width_pt += shaped.width_pt;
      glyphs.extend(shaped.glyphs);
    }
    Some(ShapedText { glyphs, width_pt })
  }
}

#[derive(Clone, Debug)]
struct FontMetrics {
  face: FontFaceData,
}

impl FontMetrics {
  fn measure(&self, text: &str, font_size: f32, script: UnicodeScriptValue) -> Option<f32> {
    Some(self.shape(text, font_size, script, 0)?.width_pt)
  }

  fn shape(
    &self,
    text: &str,
    font_size: f32,
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

    for index in 0..infos.len() {
      let position = positions[index];
      width_pt += position.x_advance as f32 * font_size / units_per_em;
      glyphs.push(ShapedGlyph {
        glyph_id: infos[index].glyph_id,
        text_range: glyph_text_range(text, infos, index, text_offset),
        x_advance_em: position.x_advance as f32 / units_per_em,
        x_offset_em: position.x_offset as f32 / units_per_em,
        y_offset_em: position.y_offset as f32 / units_per_em,
        y_advance_em: position.y_advance as f32 / units_per_em,
      });
    }

    Some(ShapedText { glyphs, width_pt })
  }
}

fn load_font_metrics(bold: bool, italic: bool) -> Option<FontMetrics> {
  load_sans_face(bold, italic).and_then(|face| {
    Face::from_slice(&face.data, face.index)?;
    Some(FontMetrics { face })
  })
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

    assert!(measure_text("office", style) > 0.0);
    assert!(measure_text("商务文档", style) > measure_text("abc", style));
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
    let shaped = shape_text("office", TextStyle::default()).expect("shaped text");

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
