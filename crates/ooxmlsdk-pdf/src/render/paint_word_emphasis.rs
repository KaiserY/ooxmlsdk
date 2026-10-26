//! Word emphasis is a second glyph pass over finalized character cells.
//!
//! The language mapping comes from MS-DOC sprmCKcd, qualified by modern
//! Office controls. Native PDF/EMF and the observed emphasis caller establish
//! the language ratios and signed integer division; annotation placement is
//! independent of the body's ink bounding box.
use super::{
  Cow, HashMap, PaintGlyph, PaintGlyphFontRun, PaintGlyphFontRuns, PaintItem, PaintText,
  PaintTextPortion, PaintTextPortionKind, TextMetrics, TextStyle, common, shaped_pdf_glyphs,
  word_advances::NaturalMetricsCache,
};
use common::{EmphasisLanguage, WordprocessingEmphasis};

const DEVICE_POINT: f32 = 72.0 / 600.0;

#[derive(Clone)]
struct Annotation {
  glyph: PaintGlyph,
  face: ooxmlsdk_layout::fonts::FontFaceData,
  size_pt: f32,
  advance_px: i32,
  vertical_offset_px: i32,
}

#[derive(Default)]
pub(super) struct EmphasisCache {
  annotations: HashMap<(WordprocessingEmphasis, u32), Option<Annotation>>,
}

impl EmphasisCache {
  fn annotation(
    &mut self,
    emphasis: WordprocessingEmphasis,
    size_pt: f32,
    metrics: &mut TextMetrics,
    natural: &mut NaturalMetricsCache,
  ) -> Option<&Annotation> {
    self
      .annotations
      .entry((emphasis, size_pt.to_bits()))
      .or_insert_with(|| {
        let (character, family) = emphasis.glyph();
        let style = TextStyle {
          font_family: Some(Cow::Borrowed(family)),
          high_ansi_font_family: Some(Cow::Borrowed(family)),
          east_asia_font_family: Some(Cow::Borrowed(family)),
          complex_font_family: Some(Cow::Borrowed(family)),
          font_size_pt: size_pt,
          use_windows_font_metrics: true,
          opacity: 1.0,
          ..TextStyle::default()
        };
        let mut shaped = shaped_pdf_glyphs(character, &style, 0.0, metrics)?;
        if shaped.font_runs.len() != 1 || shaped.font_runs[0].glyphs.len() != 1 {
          return None;
        }
        let mut run = shaped.font_runs.pop()?;
        let glyph = run.glyphs.pop()?;
        let advance_px =
          natural.advance(&run.font_face, run.font_size_pt, glyph.glyph_id, character)?;
        let physical = metrics.vertical_metrics_for_text(character, &style);
        // The annotation cache uses one CJK side-leading contribution. This
        // differs from the two sides reserved by an ordinary CJK text line.
        // Preserve design-grid rounding before realizing each device metric.
        let side = ooxmlsdk_layout::text_metrics::wordprocessingml_cjk_side_leading_pt(
          physical.ink_height_pt(),
          physical.font_design_unit_pt,
        );
        let height_px = (physical.ascent_pt / DEVICE_POINT).round() as i32
          + (physical.descent_pt / DEVICE_POINT).round() as i32
          + (side / DEVICE_POINT).round() as i32;
        Some(Annotation {
          glyph,
          face: run.font_face,
          size_pt: run.font_size_pt,
          advance_px,
          vertical_offset_px: vertical_offset(emphasis.paint_language(), height_px),
        })
      })
      .as_ref()
  }

  pub(super) fn realize<'doc>(
    &mut self,
    items: &mut Vec<PaintItem<'doc>>,
    metrics: &mut TextMetrics,
    natural: &mut NaturalMetricsCache,
  ) {
    // Most documents contain no emphasis. Scan the existing items in place
    // and allocate an expansion vector only when this group has an annotation.
    let mut has_annotation = false;
    for item in items.iter_mut() {
      if let PaintItem::Group { items, .. } = item {
        self.realize(items, metrics, natural);
      }
      has_annotation |=
        matches!(item, PaintItem::Text(text) if text.item.style.wordprocessing_emphasis.is_some());
    }
    if !has_annotation {
      return;
    }
    let source = std::mem::take(items);
    items.reserve(source.len());
    for item in source {
      let annotation = match &item {
        PaintItem::Text(text) => self.paint(text, metrics, natural),
        _ => None,
      };
      items.push(item);
      if let Some(annotation) = annotation {
        items.push(PaintItem::Text(Box::new(annotation)));
      }
    }
  }

  fn paint<'doc>(
    &mut self,
    body: &PaintText<'doc>,
    metrics: &mut TextMetrics,
    natural: &mut NaturalMetricsCache,
  ) -> Option<PaintText<'doc>> {
    let emphasis = body.item.style.wordprocessing_emphasis?;
    if body.item.style.hidden || body.item.style.semantic_only {
      return None;
    }
    let character = emphasis.glyph().0;
    let mut text = String::new();
    let mut portions = Vec::new();
    let scale = body.item.style.horizontal_scale.unwrap_or(1.0);
    for portion in &body.portions {
      let runs = portion.glyphs.as_ref()?;
      for run in runs {
        let annotation = self.annotation(emphasis, run.font_size_pt, metrics, natural)?;
        let mut x = portion.x_pt + run.x_offset_pt * scale;
        let mut glyphs = run.glyphs.iter().peekable();
        while let Some(glyph) = glyphs.next() {
          let range = &glyph.text_range;
          let mut width = glyph.x_advance * run.font_size_pt * scale;
          // The Word callback groups glyphs belonging to one source cluster.
          // Combining glyphs share one mark; spaces have no mark.
          while glyphs.peek().is_some_and(|next| next.text_range == *range) {
            width += glyphs.next().expect("peeked cluster").x_advance * run.font_size_pt * scale;
          }
          if let Some(source) = body.item.text.get(range.clone())
            && width > 0.0
            && source.chars().any(|ch| !ch.is_whitespace())
          {
            let cell_px = (width / DEVICE_POINT).round() as i32;
            let mark_x = x
              + horizontal_offset(emphasis.paint_language(), cell_px, annotation.advance_px) as f32
                * DEVICE_POINT;
            let mark_y = portion.baseline_y + annotation.vertical_offset_px as f32 * DEVICE_POINT;
            let start = text.len();
            text.push_str(character);
            let range = start..text.len();
            let mut glyph = annotation.glyph.clone();
            glyph.text_range = range.clone();
            glyph.x_advance = annotation.advance_px as f32 * DEVICE_POINT / annotation.size_pt;
            let mut glyphs = PaintGlyphFontRuns::new();
            glyphs.push(PaintGlyphFontRun {
              font_face: annotation.face.clone(),
              font_size_pt: annotation.size_pt,
              x_offset_pt: 0.0,
              glyphs: vec![glyph],
            });
            portions.push(PaintTextPortion {
              kind: PaintTextPortionKind::Text,
              text_range: range,
              x_pt: mark_x,
              baseline_y: mark_y,
              width_pt: annotation.advance_px as f32 * DEVICE_POINT,
              clip: portion.clip,
              glyphs: Some(glyphs),
              highlight: None,
              underline: None,
              strikethrough: None,
              link: None,
            });
          }
          x += width;
        }
      }
    }
    let first = portions.first()?;
    let mut item = body.item.clone();
    item.x_pt = first.x_pt;
    item.y_pt = first.baseline_y;
    item.text = Cow::Owned(text);
    item.style = TextStyle {
      font_family: Some(Cow::Borrowed(emphasis.glyph().1)),
      high_ansi_font_family: Some(Cow::Borrowed(emphasis.glyph().1)),
      east_asia_font_family: Some(Cow::Borrowed(emphasis.glyph().1)),
      font_size_pt: body.item.style.font_size_pt,
      color: body.item.style.color,
      opacity: body.item.style.opacity,
      rotation_deg: body.item.style.rotation_deg,
      pdf_glyph_outlines: body.item.style.pdf_glyph_outlines,
      pdf_glyph_outline_options: body.item.style.pdf_glyph_outline_options.clone(),
      ..TextStyle::default()
    };
    item.origin_is_baseline = true;
    item.line_metrics_participant = false;
    item.wordprocessing_line_metrics = None;
    item.dynamic_field = None;
    item.form_widget_id = None;
    item.semantic_target_width_pt = None;
    item.word_spacing_pt = 0.0;
    item.decoration_span_start_x_pt = None;
    item.pdf_text_segmentation = common::PdfTextSegmentation::Portion;
    Some(PaintText {
      item,
      source_frame_index: body.source_frame_index,
      source_line_index: body.source_line_index,
      baseline_y: first.baseline_y,
      width_pt: body.width_pt,
      portions,
    })
  }
}

fn horizontal_offset(language: EmphasisLanguage, cell_px: i32, marker_px: i32) -> i32 {
  let denominator = match language {
    EmphasisLanguage::Japanese => 6,
    EmphasisLanguage::SimplifiedChinese => 5,
    EmphasisLanguage::Korean | EmphasisLanguage::TraditionalChinese => 2,
  };
  // Signed division truncates toward zero, including narrow source cells.
  ((i64::from(cell_px) * denominator - 2 * i64::from(marker_px)) / (2 * denominator)) as i32
}

fn vertical_offset(language: EmphasisLanguage, font_height_px: i32) -> i32 {
  let (numerator, denominator, sign) = match language {
    EmphasisLanguage::Japanese => (7, 8, -1),
    EmphasisLanguage::Korean => (1, 4, -1),
    EmphasisLanguage::TraditionalChinese => (1, 2, -1),
    EmphasisLanguage::SimplifiedChinese => (1, 3, 1),
  };
  sign * ((i64::from(font_height_px) * numerator + denominator / 2) / denominator) as i32
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn word_emphasis_native_integer_anchors() {
    use EmphasisLanguage::*;
    // Actual 600dpi EMF and the observed native call use92px glyph em,
    // one14px side-leading contribution, and26 source cells/22 marks.
    for (language, expected) in [
      (Japanese, -93),
      (Korean, -27),
      (TraditionalChinese, -53),
      (SimplifiedChinese, 35),
    ] {
      assert_eq!(vertical_offset(language, 106), expected);
    }
    for (height, above, circle, below) in [(77, -67, -19, 26), (192, -168, -48, 64)] {
      assert_eq!(vertical_offset(Japanese, height), above);
      assert_eq!(vertical_offset(Korean, height), circle);
      assert_eq!(vertical_offset(SimplifiedChinese, height), below);
    }
    assert_eq!(horizontal_offset(Japanese, 48, 92), 8);
    assert_eq!(horizontal_offset(Japanese, 49, 92), 9);
    assert_eq!(horizontal_offset(Japanese, 30, 92), 0);
    assert_eq!(horizontal_offset(Korean, 39, 29), 5);
    assert_eq!(horizontal_offset(SimplifiedChinese, 48, 92), 5);
    assert_eq!(horizontal_offset(Japanese, 10, 92), -10);
  }
}
