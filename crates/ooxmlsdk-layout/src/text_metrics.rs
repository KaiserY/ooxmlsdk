use crate::fonts::{
  FontFaceData, FontResolver, FontStyleRef, decoration_metrics as font_decoration_metrics,
  font_face_data, shape_text_runs as shape_font_text_runs,
  vertical_metrics as font_vertical_metrics,
};

// Last-resort vertical metrics when no usable font face can be loaded. Keep
// this out of horizontal measurement: LibreOffice and Typst both shape with
// real font data instead of estimating glyph advances by character class.
const FALLBACK_ASCENT_EM: f32 = 0.8;
const FALLBACK_DESCENT_EM: f32 = 0.2;
const FALLBACK_LINE_GAP_EM: f32 = 0.05;
// FontMetricData::ImplInitTextLineSize.
const LO_TEXT_LINE_DESCENT_FALLBACK_DIVISOR: f32 = 10.0;
const LO_TEXT_LINE_MAX_DESCENT_DIVISOR: f32 = 3.0;
const LO_TEXT_LINE_WIDTH_FRACTION_OF_DESCENT: f32 = 0.25;
const LO_TEXT_LINE_MIN_WIDTH_PT: f32 = 1.0;
const LO_TEXT_LINE_WIDTH_HALF_DIVISOR: f32 = 2.0;
const LO_TEXT_LINE_STRIKEOUT_OFFSET_DIVISOR: f32 = 3.0;
const LO_TEXT_LINE_UNDERLINE_BASELINE_OFFSET_PT: f32 = 1.0;

#[derive(Clone, Debug)]
pub struct ShapedText {
  pub glyphs: Vec<ShapedGlyph>,
  pub font_faces: Vec<FontFaceData>,
  pub width_pt: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct TextVerticalMetrics {
  pub ascent_pt: f32,
  pub descent_pt: f32,
  pub line_gap_pt: f32,
}

impl TextVerticalMetrics {
  pub fn ink_height_pt(self) -> f32 {
    self.ascent_pt + self.descent_pt
  }

  pub fn line_height_pt(self) -> f32 {
    self.ink_height_pt() + self.line_gap_pt
  }

  pub fn leading_above_pt(self) -> f32 {
    self.line_gap_pt / 2.0
  }
}

#[derive(Clone, Copy, Debug)]
pub struct TextDecorationMetrics {
  pub underline_offset_pt: f32,
  pub underline_width_pt: f32,
  pub strikethrough_offset_pt: f32,
  pub strikethrough_width_pt: f32,
}

#[derive(Clone, Debug)]
pub struct ShapedGlyph {
  pub font_index: usize,
  pub glyph_id: u32,
  pub text_range: std::ops::Range<usize>,
  pub x_advance_em: f32,
  pub x_offset_em: f32,
  pub y_offset_em: f32,
  pub y_advance_em: f32,
}

#[derive(Debug, Default)]
pub struct TextMetrics {
  fonts: FontResolver,
}

impl TextMetrics {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn into_font_resolver(self) -> FontResolver {
    self.fonts
  }

  pub fn measure_text(&mut self, text: &str, style: &(impl FontStyleRef + ?Sized)) -> f32 {
    if text.is_empty() {
      return 0.0;
    }

    self
      .shape_text(text, style)
      .map_or(0.0, |shaped| shaped.width_pt)
  }

  pub fn shape_text(
    &mut self,
    text: &str,
    style: &(impl FontStyleRef + ?Sized),
  ) -> Option<ShapedText> {
    if text.is_empty() {
      return Some(ShapedText {
        glyphs: Vec::new(),
        font_faces: Vec::new(),
        width_pt: 0.0,
      });
    }

    let runs = self.fonts.shape_text_runs(text, style)?;
    shaped_text_from_runs(runs, style, |font_id| self.fonts.font_face_data(font_id))
  }

  pub fn vertical_metrics(&mut self, style: &(impl FontStyleRef + ?Sized)) -> TextVerticalMetrics {
    self
      .fonts
      .vertical_metrics(style)
      .map(|metrics| TextVerticalMetrics {
        ascent_pt: metrics.ascent_pt,
        descent_pt: metrics.descent_pt,
        line_gap_pt: metrics.line_gap_pt,
      })
      .unwrap_or_else(|| approximate_vertical_metrics(style.font_size_pt()))
  }

  pub fn text_decoration_metrics(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
  ) -> TextDecorationMetrics {
    self
      .fonts
      .decoration_metrics(style)
      .and_then(|metrics| {
        (metrics.underline_thickness_pt > f32::EPSILON
          && metrics.strikeout_thickness_pt > f32::EPSILON)
          .then_some(TextDecorationMetrics {
            underline_offset_pt: metrics.underline_offset_pt,
            underline_width_pt: metrics.underline_thickness_pt,
            strikethrough_offset_pt: metrics.strikeout_offset_pt,
            strikethrough_width_pt: metrics.strikeout_thickness_pt,
          })
      })
      .unwrap_or_else(|| approximate_decoration_metrics(style.font_size_pt()))
  }

  pub fn baseline_offset_in_line(
    &mut self,
    style: &(impl FontStyleRef + ?Sized),
    line_height_pt: f32,
  ) -> f32 {
    let metrics = self.vertical_metrics(style);
    let natural_height_pt = metrics.line_height_pt() + style.baseline_shift_pt().abs();
    let extra_leading_pt = (line_height_pt - natural_height_pt).max(0.0) / 2.0;
    extra_leading_pt + metrics.leading_above_pt() + metrics.ascent_pt - style.baseline_shift_pt()
  }
}

pub fn measure_text(text: &str, style: &(impl FontStyleRef + ?Sized)) -> f32 {
  if text.is_empty() {
    return 0.0;
  }

  shape_text(text, style).map_or(0.0, |shaped| shaped.width_pt)
}

pub fn shape_text(text: &str, style: &(impl FontStyleRef + ?Sized)) -> Option<ShapedText> {
  if text.is_empty() {
    return Some(ShapedText {
      glyphs: Vec::new(),
      font_faces: Vec::new(),
      width_pt: 0.0,
    });
  }

  let runs = shape_font_text_runs(text, style)?;
  shaped_text_from_runs(runs, style, font_face_data)
}

fn shaped_text_from_runs(
  runs: Vec<ooxmlsdk_fonts::ShapedRun<'static>>,
  style: &(impl FontStyleRef + ?Sized),
  mut font_face: impl FnMut(&ooxmlsdk_fonts::FontId) -> Option<FontFaceData>,
) -> Option<ShapedText> {
  let glyph_count = runs.iter().map(|run| run.glyphs.len()).sum();
  let mut glyphs = Vec::with_capacity(glyph_count);
  let mut font_faces = Vec::with_capacity(runs.len());
  let mut width_pt = 0.0;
  let em_divisor = style.font_size_pt().max(f32::EPSILON);

  for run in runs {
    let font_index = font_faces.len();
    font_faces.push(font_face(&run.font_id)?);
    width_pt += run.advance_pt;
    glyphs.extend(run.glyphs.iter().map(|glyph| ShapedGlyph {
      font_index,
      glyph_id: glyph.glyph_id,
      text_range: glyph.text_range.clone(),
      x_advance_em: glyph.x_advance_pt / em_divisor,
      x_offset_em: glyph.x_offset_pt / em_divisor,
      y_offset_em: glyph.y_offset_pt / em_divisor,
      y_advance_em: glyph.y_advance_pt / em_divisor,
    }));
  }

  Some(ShapedText {
    glyphs,
    font_faces,
    width_pt,
  })
}

pub fn vertical_metrics(style: &(impl FontStyleRef + ?Sized)) -> TextVerticalMetrics {
  font_vertical_metrics(style)
    .map(|metrics| TextVerticalMetrics {
      ascent_pt: metrics.ascent_pt,
      descent_pt: metrics.descent_pt,
      line_gap_pt: metrics.line_gap_pt,
    })
    .unwrap_or_else(|| approximate_vertical_metrics(style.font_size_pt()))
}

pub fn text_decoration_metrics(style: &(impl FontStyleRef + ?Sized)) -> TextDecorationMetrics {
  font_decoration_metrics(style)
    .and_then(|metrics| {
      (metrics.underline_thickness_pt > f32::EPSILON
        && metrics.strikeout_thickness_pt > f32::EPSILON)
        .then_some(TextDecorationMetrics {
          underline_offset_pt: metrics.underline_offset_pt,
          underline_width_pt: metrics.underline_thickness_pt,
          strikethrough_offset_pt: metrics.strikeout_offset_pt,
          strikethrough_width_pt: metrics.strikeout_thickness_pt,
        })
    })
    .unwrap_or_else(|| approximate_decoration_metrics(style.font_size_pt()))
}

pub fn inline_text_box_height(style: &(impl FontStyleRef + ?Sized)) -> f32 {
  vertical_metrics(style).line_height_pt() + style.baseline_shift_pt().abs()
}

pub fn baseline_offset_in_line(style: &(impl FontStyleRef + ?Sized), line_height_pt: f32) -> f32 {
  let metrics = vertical_metrics(style);
  let natural_height_pt = metrics.line_height_pt() + style.baseline_shift_pt().abs();
  let extra_leading_pt = (line_height_pt - natural_height_pt).max(0.0) / 2.0;
  extra_leading_pt + metrics.leading_above_pt() + metrics.ascent_pt - style.baseline_shift_pt()
}

fn approximate_vertical_metrics(font_size: f32) -> TextVerticalMetrics {
  TextVerticalMetrics {
    ascent_pt: font_size * FALLBACK_ASCENT_EM,
    descent_pt: font_size * FALLBACK_DESCENT_EM,
    line_gap_pt: font_size * FALLBACK_LINE_GAP_EM,
  }
}

fn approximate_decoration_metrics(font_size: f32) -> TextDecorationMetrics {
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

#[cfg(test)]
mod tests {
  use crate::common::{Pt, TextStyle};

  use super::*;

  #[test]
  fn shaped_measurement_handles_ligatures_and_cjk() {
    let style = test_style();

    assert!(measure_text("office", &style) > 0.0);
    assert!(measure_text("商务文档", &style) > measure_text("abc", &style));
  }

  #[test]
  fn shaped_text_exposes_glyph_advances_for_pdf_paint() {
    let style = test_style();
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

  fn test_style() -> TextStyle<'static> {
    TextStyle {
      font_size: Pt(11.0),
      ..TextStyle::default()
    }
  }
}
