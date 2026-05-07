use std::sync::OnceLock;

use rustybuzz::{Direction, Face, Script, UnicodeBuffer};
use unicode_bidi::{Direction as BidiDirection, get_base_direction};
use unicode_script::{Script as UnicodeScriptValue, UnicodeScript};

use crate::docx::TextStyle;
use crate::fonts::{FontFaceData, load_sans_face};

pub(crate) fn measure_text(text: &str, style: TextStyle) -> f32 {
  if text.is_empty() {
    return 0.0;
  }

  FontMetricsSet::get()
    .and_then(|set| set.measure(text, style))
    .unwrap_or_else(|| approximate_text_width(text, style.font_size_pt))
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
}

#[derive(Clone, Debug)]
struct FontMetrics {
  face: FontFaceData,
}

impl FontMetrics {
  fn measure(&self, text: &str, font_size: f32, script: UnicodeScriptValue) -> Option<f32> {
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
    let glyphs = rustybuzz::shape(&face, &[], buffer);
    let advance = glyphs
      .glyph_positions()
      .iter()
      .map(|position| position.x_advance as f32)
      .sum::<f32>();
    Some(advance * font_size / units_per_em)
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
      });
      start = index;
      active = script;
    }
  }

  if start < text.len() {
    runs.push(ScriptRun {
      text: &text[start..],
      script: active,
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
    assert_eq!(runs[1].text, "商务");
  }
}
