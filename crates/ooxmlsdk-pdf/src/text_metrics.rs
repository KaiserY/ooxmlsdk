use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use rustybuzz::{Direction, Face as RbFace, Script, UnicodeBuffer};
use ttf_parser::Face as TtfFace;
use unicode_bidi::{Direction as BidiDirection, get_base_direction};
use unicode_script::{Script as UnicodeScriptValue, UnicodeScript};

use crate::docx::TextStyle;
use crate::fonts::{FontFaceData, load_text_faces};

// Last-resort vertical metrics when no usable font face can be loaded. Keep
// this out of horizontal measurement: LibreOffice and Typst both shape with
// real font data instead of estimating glyph advances by character class.
const FALLBACK_ASCENT_EM: f32 = 0.8;
const FALLBACK_DESCENT_EM: f32 = 0.2;
const FALLBACK_LINE_GAP_EM: f32 = 0.05;
// Source: LibreOffice vcl/source/font/fontmetric.cxx
// FontMetricData::ImplInitTextLineSize.
const LO_TEXT_LINE_DESCENT_FALLBACK_DIVISOR: f32 = 10.0;
const LO_TEXT_LINE_MAX_DESCENT_DIVISOR: f32 = 3.0;
const LO_TEXT_LINE_WIDTH_FRACTION_OF_DESCENT: f32 = 0.25;
const LO_TEXT_LINE_MIN_WIDTH_PT: f32 = 1.0;
const LO_TEXT_LINE_WIDTH_HALF_DIVISOR: f32 = 2.0;
const LO_TEXT_LINE_STRIKEOUT_OFFSET_DIVISOR: f32 = 3.0;
const LO_TEXT_LINE_UNDERLINE_BASELINE_OFFSET_PT: f32 = 1.0;
// Source: LibreOffice include/editeng/svxfont.hxx SMALL_CAPS_PERCENTAGE.
const LO_SMALL_CAPS_FONT_SCALE: f32 = 0.80;

#[derive(Clone, Debug)]
pub(crate) struct ShapedText {
  pub glyphs: Vec<ShapedGlyph>,
  pub font_faces: Vec<FontFaceData>,
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

#[derive(Clone, Copy, Debug)]
pub(crate) struct TextDecorationMetrics {
  pub underline_offset_pt: f32,
  pub underline_width_pt: f32,
  pub strikethrough_offset_pt: f32,
  pub strikethrough_width_pt: f32,
}

#[derive(Clone, Debug)]
pub(crate) struct ShapedGlyph {
  pub font_index: usize,
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

  FontMetricsCache::get().measure(text, style).unwrap_or(0.0)
}

pub(crate) fn shape_text(text: &str, style: &TextStyle) -> Option<ShapedText> {
  if text.is_empty() {
    return Some(ShapedText {
      glyphs: Vec::new(),
      font_faces: Vec::new(),
      width_pt: 0.0,
    });
  }

  FontMetricsCache::get().shape(text, style)
}

pub(crate) fn vertical_metrics(style: &TextStyle) -> TextVerticalMetrics {
  FontMetricsCache::get()
    .vertical_metrics(style)
    .unwrap_or_else(|| approximate_vertical_metrics(style.font_size_pt))
}

pub(crate) fn text_decoration_metrics(style: &TextStyle) -> TextDecorationMetrics {
  FontMetricsCache::get()
    .decoration_metrics(style)
    .unwrap_or_else(|| approximate_decoration_metrics(style.font_size_pt))
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

fn approximate_vertical_metrics(font_size: f32) -> TextVerticalMetrics {
  TextVerticalMetrics {
    ascent_pt: font_size * FALLBACK_ASCENT_EM,
    descent_pt: font_size * FALLBACK_DESCENT_EM,
    line_gap_pt: font_size * FALLBACK_LINE_GAP_EM,
  }
}

fn approximate_decoration_metrics(font_size: f32) -> TextDecorationMetrics {
  // Source: LibreOffice vcl/source/font/fontmetric.cxx
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontKey {
  family: Option<String>,
  bold: bool,
  italic: bool,
}

#[derive(Default)]
struct FontMetricsCache {
  fonts: Mutex<HashMap<FontKey, Vec<FontMetrics>>>,
}

impl FontMetricsCache {
  fn get() -> &'static Self {
    static METRICS: OnceLock<FontMetricsCache> = OnceLock::new();
    METRICS.get_or_init(Self::default)
  }

  fn measure(&self, text: &str, style: &TextStyle) -> Option<f32> {
    let fonts = self.select(style)?;
    let mut width = 0.0;
    for run in case_mapped_script_runs(text, style) {
      let shaped = shape_with_fallback(
        &fonts,
        &run.text,
        run.font_size_pt,
        style.character_spacing_pt,
        run.script,
        run.source_start,
      )?;
      width += shaped.width_pt;
    }
    Some(width)
  }

  fn shape(&self, text: &str, style: &TextStyle) -> Option<ShapedText> {
    let fonts = self.select(style)?;
    let mut glyphs = Vec::new();
    let mut font_faces = Vec::new();
    let mut width_pt = 0.0;
    for run in case_mapped_script_runs(text, style) {
      let shaped = shape_with_fallback(
        &fonts,
        &run.text,
        run.font_size_pt,
        style.character_spacing_pt,
        run.script,
        run.source_start,
      )?;
      let font_offset = font_faces.len();
      width_pt += shaped.width_pt;
      font_faces.extend(shaped.font_faces);
      glyphs.extend(shaped.glyphs.into_iter().map(|mut glyph| {
        glyph.font_index += font_offset;
        glyph
      }));
    }
    Some(ShapedText {
      glyphs,
      font_faces,
      width_pt,
    })
  }

  fn vertical_metrics(&self, style: &TextStyle) -> Option<TextVerticalMetrics> {
    let font = self.select(style)?.into_iter().next()?;
    Some(font.vertical_metrics(style.font_size_pt))
  }

  fn decoration_metrics(&self, style: &TextStyle) -> Option<TextDecorationMetrics> {
    let font = self.select(style)?.into_iter().next()?;
    font.decoration_metrics(style.font_size_pt)
  }

  fn select(&self, style: &TextStyle) -> Option<Vec<FontMetrics>> {
    let key = FontKey {
      family: style.font_family.as_deref().map(str::to_string),
      bold: style.bold,
      italic: style.italic,
    };
    if let Ok(fonts) = self.fonts.lock()
      && let Some(font) = fonts.get(&key)
    {
      return Some(font.clone());
    }

    let loaded = load_font_metrics(style);
    let mut fonts = self.fonts.lock().ok()?;
    let font = fonts.entry(key).or_insert(loaded);
    Some(font.clone())
  }
}

#[derive(Clone, Debug)]
struct FontMetrics {
  face: FontFaceData,
  vertical: FaceVerticalMetrics,
}

impl FontMetrics {
  fn shape(
    &self,
    text: &str,
    font_size: f32,
    character_spacing_pt: f32,
    script: UnicodeScriptValue,
    text_offset: usize,
  ) -> Option<ShapedText> {
    let face = RbFace::from_slice(self.face.data.as_slice(), self.face.index)?;
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
        font_index: 0,
        glyph_id: infos[index].glyph_id,
        text_range: glyph_text_range(text, infos, index, text_offset),
        x_advance_em,
        x_offset_em: position.x_offset as f32 / units_per_em,
        y_offset_em: position.y_offset as f32 / units_per_em,
        y_advance_em: position.y_advance as f32 / units_per_em,
      });
    }

    Some(ShapedText {
      glyphs,
      font_faces: vec![self.face.clone()],
      width_pt,
    })
  }

  fn vertical_metrics(&self, font_size: f32) -> TextVerticalMetrics {
    let scale = font_size / self.vertical.units_per_em;
    TextVerticalMetrics {
      ascent_pt: self.vertical.ascent_units * scale,
      descent_pt: self.vertical.descent_units * scale,
      line_gap_pt: self.vertical.line_gap_units * scale,
    }
  }

  fn decoration_metrics(&self, font_size: f32) -> Option<TextDecorationMetrics> {
    let face = ttf_parser::Face::parse(self.face.data.as_slice(), self.face.index).ok()?;
    let scale = font_size / face.units_per_em().max(1) as f32;
    let underline = face.underline_metrics()?;
    let strikethrough = face.strikeout_metrics()?;
    let underline_width_pt = (underline.thickness as f32 * scale).abs();
    let strikethrough_width_pt = (strikethrough.thickness as f32 * scale).abs();
    if underline_width_pt <= f32::EPSILON || strikethrough_width_pt <= f32::EPSILON {
      return None;
    }
    Some(TextDecorationMetrics {
      underline_offset_pt: -(underline.position as f32) * scale,
      underline_width_pt,
      strikethrough_offset_pt: strikethrough.position as f32 * scale,
      strikethrough_width_pt,
    })
  }
}

fn shape_with_fallback(
  fonts: &[FontMetrics],
  text: &str,
  font_size: f32,
  character_spacing_pt: f32,
  script: UnicodeScriptValue,
  text_offset: usize,
) -> Option<ShapedText> {
  let mut glyphs = Vec::new();
  let mut font_faces = Vec::new();
  let mut width_pt = 0.0;
  let mut start = 0usize;
  let mut active_font = None;
  let parsed_faces: Vec<_> = fonts
    .iter()
    .map(|font| TtfFace::parse(font.face.data.as_slice(), font.face.index).ok())
    .collect();

  for (index, ch) in text.char_indices() {
    let font_index = font_for_char(&parsed_faces, ch)?;
    if active_font.is_some_and(|active| active != font_index) {
      shape_fallback_segment(
        FallbackSegment {
          fonts,
          text,
          range: start..index,
          source_font_index: active_font.unwrap_or(font_index),
          font_size,
          character_spacing_pt,
          script,
          text_offset,
        },
        &mut FallbackShapeOutput {
          glyphs: &mut glyphs,
          font_faces: &mut font_faces,
          width_pt: &mut width_pt,
        },
      )?;
      start = index;
    }
    active_font = Some(font_index);
  }
  if start < text.len() {
    shape_fallback_segment(
      FallbackSegment {
        fonts,
        text,
        range: start..text.len(),
        source_font_index: active_font?,
        font_size,
        character_spacing_pt,
        script,
        text_offset,
      },
      &mut FallbackShapeOutput {
        glyphs: &mut glyphs,
        font_faces: &mut font_faces,
        width_pt: &mut width_pt,
      },
    )?;
  }

  Some(ShapedText {
    glyphs,
    font_faces,
    width_pt,
  })
}

struct FallbackSegment<'a> {
  fonts: &'a [FontMetrics],
  text: &'a str,
  range: std::ops::Range<usize>,
  source_font_index: usize,
  font_size: f32,
  character_spacing_pt: f32,
  script: UnicodeScriptValue,
  text_offset: usize,
}

struct FallbackShapeOutput<'a> {
  glyphs: &'a mut Vec<ShapedGlyph>,
  font_faces: &'a mut Vec<FontFaceData>,
  width_pt: &'a mut f32,
}

fn shape_fallback_segment(
  segment: FallbackSegment<'_>,
  output: &mut FallbackShapeOutput<'_>,
) -> Option<()> {
  let font = segment.fonts.get(segment.source_font_index)?;
  let target_font_index = output.font_faces.len();
  output.font_faces.push(font.face.clone());
  let mut shaped = font.shape(
    &segment.text[segment.range.clone()],
    segment.font_size,
    segment.character_spacing_pt,
    segment.script,
    segment.text_offset + segment.range.start,
  )?;
  *output.width_pt += shaped.width_pt;
  output
    .glyphs
    .extend(shaped.glyphs.drain(..).map(|mut glyph| {
      glyph.font_index = target_font_index;
      glyph
    }));
  Some(())
}

fn font_for_char(faces: &[Option<TtfFace<'_>>], ch: char) -> Option<usize> {
  faces
    .iter()
    .position(|face| {
      face
        .as_ref()
        .is_some_and(|face| face.glyph_index(ch).is_some())
    })
    .or_else(|| (!faces.is_empty()).then_some(0))
}

fn load_font_metrics(style: &TextStyle) -> Vec<FontMetrics> {
  load_text_faces(style)
    .into_iter()
    .filter_map(|face| {
      let parsed = TtfFace::parse(face.data.as_slice(), face.index).ok()?;
      Some(FontMetrics {
        vertical: FaceVerticalMetrics::from_face(&parsed),
        face,
      })
    })
    .collect()
}

#[derive(Clone, Copy, Debug)]
struct FaceVerticalMetrics {
  units_per_em: f32,
  ascent_units: f32,
  descent_units: f32,
  line_gap_units: f32,
}

impl FaceVerticalMetrics {
  fn from_face(face: &TtfFace<'_>) -> Self {
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

#[derive(Clone, Debug)]
struct CaseMappedScriptRun {
  text: String,
  script: UnicodeScriptValue,
  source_start: usize,
  font_size_pt: f32,
}

fn case_mapped_script_runs(text: &str, style: &TextStyle) -> Vec<CaseMappedScriptRun> {
  if !style.small_caps {
    return script_runs(text)
      .into_iter()
      .map(|run| CaseMappedScriptRun {
        text: run.text.to_string(),
        script: run.script,
        source_start: run.start,
        font_size_pt: style.font_size_pt,
      })
      .collect();
  }

  let mut runs = Vec::new();
  let mut start = 0;
  let mut active_lowercase = None;
  for (index, ch) in text.char_indices() {
    let lowercase = ch.is_lowercase();
    if let Some(active) = active_lowercase {
      if lowercase != active {
        push_small_caps_case_run(text, start, index, active, style.font_size_pt, &mut runs);
        start = index;
        active_lowercase = Some(lowercase);
      }
    } else {
      active_lowercase = Some(lowercase);
    }
  }
  if let Some(active) = active_lowercase {
    push_small_caps_case_run(
      text,
      start,
      text.len(),
      active,
      style.font_size_pt,
      &mut runs,
    );
  }
  runs
}

fn push_small_caps_case_run(
  text: &str,
  start: usize,
  end: usize,
  lowercase: bool,
  font_size_pt: f32,
  runs: &mut Vec<CaseMappedScriptRun>,
) {
  let mapped = if lowercase {
    text[start..end].to_uppercase()
  } else {
    text[start..end].to_string()
  };
  for run in script_runs(&mapped) {
    runs.push(CaseMappedScriptRun {
      text: run.text.to_string(),
      script: run.script,
      source_start: start + run.start,
      font_size_pt: if lowercase {
        font_size_pt * LO_SMALL_CAPS_FONT_SCALE
      } else {
        font_size_pt
      },
    });
  }
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
  // Source: Typst typst-layout/src/inline/shaping.rs keeps the full source
  // text range on every glyph in the same cluster. Krilla uses these ranges
  // for ActualText/ToUnicode mapping, so empty follow-up ranges make complex
  // and mixed-script PDF text extraction lose or reorder characters.
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
