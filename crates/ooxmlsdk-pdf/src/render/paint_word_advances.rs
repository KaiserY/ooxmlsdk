//! Word's ordinary horizontal text realization before PDF serialization.
//!
//! Native MSLS70 observations replay 726 complete line batches exactly. Keep
//! ideal line accumulation across face/style runs, independently of PDF Tc/TJ.
use super::{HashMap, PaintItem, PaintText, PaintTextPortionKind, common};
use emfsdk::render::GdiNaturalMetrics;
use ooxmlsdk_layout::fonts::{FontFaceCacheKey, FontFaceData, FontStyleRef};
use skrifa::{MetadataProvider, raw::TableProvider};
use std::sync::Arc;

const POINTS_PER_PIXEL: f32 = 72.0 / 600.0;
const IDEAL_UNITS_PER_POINT: f32 = 4096.0;
use common::wordprocessing_device::{
  MAX_IDEAL_ADVANCE, balance_advances, line_origin as realize_frame_origin,
};

#[derive(Default)]
pub(super) struct NaturalMetricsCache {
  bytes: HashMap<FontFaceCacheKey, Arc<[u8]>>,
  instances: HashMap<(FontFaceCacheKey, u16), Option<NaturalFontMetrics>>,
  design_units: HashMap<FontFaceCacheKey, Option<u16>>,
  top_offsets: HashMap<(FontFaceCacheKey, u16, bool), Option<i32>>,
}
enum NaturalFontMetrics {
  Hinted(Box<GdiNaturalMetrics>),
  // The head table promises that instructions do not change advances. Keep
  // location-aware variation metrics without running a second interpreter.
  LinearVariable(HashMap<u32, Option<i32>>),
}

impl NaturalMetricsCache {
  fn top_offset(&mut self, face: &FontFaceData, size_pt: f32, ansi: bool) -> Option<i32> {
    let pixels = (f64::from(size_pt) * 600.0 / 72.0).round();
    if !pixels.is_finite() || !(1.0..=f64::from(u16::MAX)).contains(&pixels) {
      return None;
    }
    let ppem = pixels as u16;
    *self
      .top_offsets
      .entry((face.cache_key(), ppem, ansi))
      .or_insert_with(|| {
        let font = skrifa::FontRef::from_index(face.data.as_slice(), face.index).ok()?;
        let units = font.head().ok()?.units_per_em();
        if units == 0 {
          return None;
        }
        // VDMX owns actual GDI ascent at this printer em. Falling back to
        // independently rounded Win metrics matches GetTextMetrics when the
        // exact device record is absent. Synthetic bold retains these metrics.
        let hinted = font.table_data(skrifa::Tag::new(b"VDMX")).and_then(|data| {
          emfsdk::font::vdmx_vertical_device_metrics(data.as_bytes(), ppem, u8::from(!ansi))
        });
        let ascent = if let Some(metrics) = hinted {
          metrics.ascent
        } else {
          (f64::from(font.os2().ok()?.us_win_ascent()) * pixels / f64::from(units)).round() as i32
        };
        // Native top-aligned font caches keep one device pixel above the
        // logical anchor and (ascent-1) in their separate glyph displacement.
        Some(ascent.saturating_sub(1))
      })
  }

  fn device_advance(
    &mut self,
    face: &FontFaceData,
    size_pt: f32,
    glyph: u32,
    source: &str,
    kerning_advance_em: Option<f32>,
  ) -> Option<i32> {
    // These Word separators have an independent unshaped/dash-division
    // metric owner, even when their surrounding run enables kerning.
    if let Some(advance_em) = kerning_advance_em
      && !matches!(source, "\u{2004}" | "\u{2006}" | "\u{2028}" | "\u{2029}")
    {
      return self.kerning_advance(face, size_pt, advance_em);
    }
    self.advance(face, size_pt, glyph, source)
  }

  fn kerning_advance(&mut self, face: &FontFaceData, size_pt: f32, advance_em: f32) -> Option<i32> {
    if face.synthetic_bold || face.synthetic_italic || !advance_em.is_finite() {
      return None;
    }
    let pixels = (size_pt / POINTS_PER_PIXEL).round();
    if !pixels.is_finite() || !(1.0..=u16::MAX as f32).contains(&pixels) {
      return None;
    }
    let units = *self
      .design_units
      .entry(face.cache_key())
      .or_insert_with(|| {
        let font = skrifa::FontRef::from_index(face.data.as_slice(), face.index).ok()?;
        font.glyf().ok()?;
        let units = font.head().ok()?.units_per_em();
        (units != 0).then_some(units)
      });
    let units = f64::from(units?);
    // Native Word's enabled w:kern path rounds the positioned design advance
    // on the realized printer em. It does not use the hinted Natural width.
    // Recover integral font units before rounding exact half-pixel advances.
    let design = (f64::from(advance_em) * units).round();
    let advance = (design * f64::from(pixels) / units + 0.5).trunc();
    (advance >= 0.0 && advance <= f64::from(i32::MAX)).then_some(advance as i32)
  }

  pub(super) fn advance(
    &mut self,
    face: &FontFaceData,
    size_pt: f32,
    glyph: u32,
    source: &str,
  ) -> Option<i32> {
    if face.synthetic_bold || face.synthetic_italic {
      return None;
    }
    let pixels = (size_pt / POINTS_PER_PIXEL).round();
    if !pixels.is_finite() || !(1.0..=u16::MAX as f32).contains(&pixels) {
      return None;
    }
    if matches!(source, "\u{2028}" | "\u{2029}") {
      let font = skrifa::FontRef::from_index(face.data.as_slice(), face.index).ok()?;
      let charmap = font.charmap();
      if charmap
        .map(source.chars().next()?)
        .is_none_or(|mapped| mapped.to_u32() == 0)
        && charmap
          .map(' ')
          .is_some_and(|mapped| mapped.to_u32() == glyph)
      {
        // The shaper uses the face's blank for a missing Word separator,
        // but its natural width is half an em, not that glyph's space width.
        // Native 8/11/16/22pt controls round an odd device em upward here.
        return Some((pixels * 0.5).round() as i32);
      }
    }
    let fractional_space = if matches!(source, "\u{2004}" | "\u{2006}") {
      let font = skrifa::FontRef::from_index(face.data.as_slice(), face.index).ok()?;
      ooxmlsdk_fonts::wordprocessingml_fractional_space(&font, source.chars().next()?)
    } else {
      None
    };
    let (measured_glyph, divisor) = fractional_space.unwrap_or((glyph, 1));
    let key = face.cache_key();
    let data = self
      .bytes
      .entry(key.clone())
      .or_insert_with(|| Arc::from(face.data.as_slice()));
    let instance = self
      .instances
      .entry((key, pixels as u16))
      .or_insert_with(|| {
        let font = skrifa::FontRef::from_index(data, face.index).ok()?;
        if font.fvar().is_ok()
          && !font
            .head()
            .ok()?
            .flags()
            .contains(skrifa::raw::tables::head::Flags::INSTRUCTIONS_MAY_ALTER_ADVANCE_WIDTH)
        {
          Some(NaturalFontMetrics::LinearVariable(HashMap::default()))
        } else {
          GdiNaturalMetrics::new(data.clone(), face.index, pixels as u16)
            .map(|metrics| NaturalFontMetrics::Hinted(Box::new(metrics)))
        }
      })
      .as_mut()?;
    let advance = match instance {
      NaturalFontMetrics::Hinted(metrics) => metrics.glyph_advance_px(measured_glyph)?,
      NaturalFontMetrics::LinearVariable(advances) => {
        (*advances.entry(measured_glyph).or_insert_with(|| {
          let font = skrifa::FontRef::from_index(data, face.index).ok()?;
          font
            .glyph_metrics(skrifa::instance::Size::new(pixels), face.location())
            .advance_width(skrifa::GlyphId::new(measured_glyph))
            .map(|advance| advance.round() as i32)
        }))?
      }
    };
    // Divide the already realized dash width, independently of ideal layout.
    // For example, a 12pt MS Mincho dash is50px; U+2006 is(50+3)/6=8px.
    Some((advance + i32::from(divisor / 2)) / i32::from(divisor))
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum LineOwner {
  Frame {
    frame: usize,
    line: usize,
  },
  /// A completed nested story can outlive its page-level frame. Its group,
  /// resolved baseline and clip still keep unrelated stories independent.
  Detached {
    group: usize,
    baseline: u32,
  },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct LineKey {
  owner: LineOwner,
  clip: Option<[u32; 4]>,
  variable_numbering: bool,
}

fn variable_numbering(text: &PaintText<'_>) -> bool {
  text.item.pdf_text_segmentation == common::PdfTextSegmentation::Line
    && !text.portions.is_empty()
    && text.portions.iter().all(|portion| {
      portion.glyphs.as_ref().is_some_and(|runs| {
        !runs.is_empty()
          && runs.iter().all(|run| {
            skrifa::FontRef::from_index(run.font_face.data.as_slice(), run.font_face.index)
              .is_ok_and(|font| font.fvar().is_ok())
          })
      })
    })
}

fn collect_text<'a, 'doc>(
  items: &'a mut [PaintItem<'doc>],
  transformed: bool,
  group: usize,
  next_group: &mut usize,
  out: &mut Vec<(bool, &'a mut PaintText<'doc>)>,
  lines: &mut HashMap<LineKey, Vec<usize>>,
) {
  for item in items {
    match item {
      PaintItem::Text(text) => {
        let owner = match (text.source_frame_index, text.source_line_index) {
          (Some(frame), Some(line)) => Some(LineOwner::Frame { frame, line }),
          (None, None)
            if text.item.origin_is_baseline
              && text.item.style.use_windows_font_metrics
              && text.item.style.wordprocessingml_font_slots
              && text.item.pdf_text_segmentation == common::PdfTextSegmentation::WordLine =>
          {
            Some(LineOwner::Detached {
              group,
              baseline: text.item.y_pt.to_bits(),
            })
          }
          _ => None,
        };
        if let Some(owner) = owner {
          let clip = text
            .portions
            .first()
            .and_then(|portion| portion.clip)
            .map(|clip| {
              [
                clip.x_pt.to_bits(),
                clip.y_pt.to_bits(),
                clip.width_pt.to_bits(),
                clip.height_pt.to_bits(),
              ]
            });
          lines
            .entry(LineKey {
              owner,
              clip,
              variable_numbering: variable_numbering(text),
            })
            .or_default()
            .push(out.len());
        }
        out.push((transformed, text));
      }
      PaintItem::Group {
        transform, items, ..
      } => {
        *next_group += 1;
        collect_text(
          items,
          transformed || transform.is_some(),
          *next_group,
          next_group,
          out,
          lines,
        );
      }
      _ => {}
    }
  }
}

pub(super) fn realize(items: &mut [PaintItem<'_>], cache: &mut NaturalMetricsCache) {
  let mut texts = Vec::new();
  let mut lines = HashMap::<LineKey, Vec<usize>>::default();
  let mut next_group = 0;
  collect_text(items, false, 0, &mut next_group, &mut texts, &mut lines);
  // Vertical metrics belong to each completed Word line independently of
  // horizontal segmentation. Number portions use Line rather than WordLine;
  // they must not disable native baseline realization for the whole line.
  for (transformed, text) in &mut texts {
    realize_text_baseline(text, *transformed, cache);
  }
  // Word sends the number label and the body as separate Line Services batches.
  // A variable-font label must not disable the body's device realization.
  for (key, indices) in &mut lines {
    indices.sort_by(|&a, &b| texts[a].1.item.x_pt.total_cmp(&texts[b].1.item.x_pt));
    if indices
      .iter()
      .any(|&i| texts[i].1.item.style.wordprocessing_tab_leader.is_some())
    {
      realize_tab_line(&mut texts, indices, cache, key.variable_numbering);
      continue;
    }
    // A shared baseline/clip is only a candidate for a detached line. Cells
    // inside the same flattened story can have real gaps or overlap. Confirm
    // uninterrupted logical advances before moving any of their origins.
    // Explicit frame owners retain their independent origin realization even
    // when custom spacing prevents natural-width balancing.
    let measured = if matches!(key.owner, LineOwner::Detached { .. }) {
      let Some(advances) = measure_line(&texts, indices, cache, key.variable_numbering) else {
        continue;
      };
      Some(advances)
    } else {
      None
    };
    let Some(anchor) = realize_line_origin(&mut texts, indices, key.variable_numbering) else {
      continue;
    };
    let Some(advances) =
      measured.or_else(|| measure_line(&texts, indices, cache, key.variable_numbering))
    else {
      continue;
    };
    apply_line_advances(&mut texts, indices, anchor, &advances);
  }
}

/// Native tab portions separate otherwise contiguous Line Services batches.
/// An explicit leader must not disable the adjacent ordinary text's cells.
fn realize_tab_line(
  texts: &mut [(bool, &mut PaintText<'_>)],
  indices: &[usize],
  cache: &mut NaturalMetricsCache,
  variable_numbering: bool,
) {
  let mut span = Vec::new();
  let mut values = Vec::new();
  let mut expected = None;
  for &index in indices {
    let measured = measure_line(
      texts,
      std::slice::from_ref(&index),
      cache,
      variable_numbering,
    );
    let x = texts[index].1.item.x_pt;
    let contiguous = expected.is_none_or(|end: f32| {
      (x - end).abs() <= f32::EPSILON * (values.len() + 8) as f32 * x.abs().max(end.abs()).max(1.0)
    });
    if measured.is_none() || !contiguous {
      if let Some(anchor) = realize_line_origin(texts, &span, variable_numbering) {
        apply_line_advances(texts, &span, anchor, &values);
      }
      span.clear();
      values.clear();
      expected = None;
    }
    if let Some(measured) = measured {
      let width = measured
        .iter()
        .map(|&(ideal, _)| f64::from(ideal))
        .sum::<f64>()
        / 4096.0;
      expected = Some(x + width as f32);
      span.push(index);
      values.extend(measured);
    }
  }
  if let Some(anchor) = realize_line_origin(texts, &span, variable_numbering) {
    apply_line_advances(texts, &span, anchor, &values);
  }
}

fn apply_line_advances(
  texts: &mut [(bool, &mut PaintText<'_>)],
  indices: &[usize],
  anchor: f32,
  advances: &[(i32, i32)],
) {
  let advances = balance_advances(advances);
  let mut advances = advances.into_iter();
  let mut device_x = 0;
  for &index in indices.iter() {
    let text = &mut texts[index].1;
    let start = device_x;
    for portion in &mut text.portions {
      portion.x_pt = anchor + device_x as f32 * POINTS_PER_PIXEL;
      let portion_start = device_x;
      for run in portion.glyphs.as_mut().expect("measured glyphs") {
        run.x_offset_pt = (device_x - portion_start) as f32 * POINTS_PER_PIXEL;
        for glyph in &mut run.glyphs {
          let advance = advances.next().expect("measured glyph count");
          glyph.x_advance = advance as f32 * POINTS_PER_PIXEL / run.font_size_pt;
          device_x += advance;
        }
      }
      portion.width_pt = (device_x - portion_start) as f32 * POINTS_PER_PIXEL;
    }
    text.width_pt = (device_x - start) as f32 * POINTS_PER_PIXEL;
  }
}

fn realize_text_baseline(
  text: &mut PaintText<'_>,
  transformed: bool,
  cache: &mut NaturalMetricsCache,
) {
  let style = &text.item.style;
  if transformed
    || text.item.paragraph_bidi
    || style.rotation_deg != 0.0
    || style.semantic_only
    || style.horizontal_scale.unwrap_or(1.0) != 1.0
    || style.pdf_glyph_outline_options.is_some()
    // Native tab leaders carry explicit horizontal advances, but still
    // share the completed Word line's baseline (including blank-only lines).
    || (style.semantic_character_advances_pt.is_some()
      && style.wordprocessing_tab_leader.is_none())
  {
    return;
  }
  let Some(baseline) = text
    .item
    .wordprocessing_line_metrics
    .and_then(|metrics| super::word_baselines::realize(metrics, text.item.y_pt))
  else {
    return;
  };
  let top_aligned = style.line_vertical_alignment == common::LineVerticalAlignment::Top;
  let (offset, shift) = if top_aligned {
    use ooxmlsdk_fonts::FontCharset;
    let ansi = matches!(
      style.font_charset,
      None | Some(FontCharset::Ansi | FontCharset::Other(0 | 1))
    );
    let mut offset = None;
    for run in text
      .portions
      .iter()
      .filter_map(|portion| portion.glyphs.as_ref())
      .flatten()
    {
      let Some(value) = cache.top_offset(&run.font_face, run.font_size_pt, ansi) else {
        return;
      };
      offset = Some(offset.map_or(value, |current: i32| current.max(value)));
    }
    let Some(offset) = offset else { return };
    (
      offset as f32 * POINTS_PER_PIXEL,
      common::wordprocessing_device::source_coordinate(style.baseline_shift_pt) as f32
        * POINTS_PER_PIXEL,
    )
  } else {
    (0.0, style.baseline_shift_pt)
  };
  let delta = baseline + offset - shift - text.baseline_y;
  text.baseline_y += delta;
  for portion in &mut text.portions {
    portion.baseline_y += delta;
    if top_aligned {
      if let Some(highlight) = &mut portion.highlight {
        highlight.y_pt += delta;
      }
      for line in [&mut portion.underline, &mut portion.strikethrough]
        .into_iter()
        .flatten()
      {
        line.y1_pt += delta;
        line.y2_pt += delta;
      }
      if let Some(link) = &mut portion.link {
        link.y_pt += delta;
      }
    }
  }
}

fn realize_line_origin(
  texts: &mut [(bool, &mut PaintText<'_>)],
  indices: &[usize],
  variable_numbering: bool,
) -> Option<f32> {
  for &index in indices {
    let (transformed, text) = &texts[index];
    let style = &text.item.style;
    if *transformed
      || (text.item.pdf_text_segmentation != common::PdfTextSegmentation::WordLine
        && !variable_numbering)
      || text.item.paragraph_bidi
      || style.rotation_deg != 0.0
      || style.semantic_only
      || style.horizontal_scale.unwrap_or(1.0) != 1.0
      || style.pdf_glyph_outline_options.is_some()
      || style.semantic_character_advances_pt.is_some()
    {
      return None;
    }
  }
  // Word converts the containing frame before applying Line Services' local
  // paragraph offset. Natural-width eligibility must not own this conversion:
  // native adjacent lines with ordinary and custom character spacing use
  // the same source origin even though their advance realization differs.
  let first = &texts[*indices.first()?].1.item;
  let source = first.x_pt;
  let anchor = first.wordprocessing_line_metrics.map_or_else(
    || ((f64::from(source) * 600.0 / 72.0).round() * 72.0 / 600.0) as f32,
    |metrics| realize_frame_origin(source, metrics.frame_origin_offset_x_pt, metrics.alignment),
  );
  let offset = anchor - source;
  for &index in indices {
    let text = &mut texts[index].1;
    for portion in &mut text.portions {
      portion.x_pt += offset;
    }
  }
  Some(anchor)
}

/// Each pair is (ideal advance in 1/4096pt, natural device advance in pixels).
fn measure_line(
  texts: &[(bool, &mut PaintText<'_>)],
  indices: &[usize],
  cache: &mut NaturalMetricsCache,
  variable_numbering: bool,
) -> Option<Vec<(i32, i32)>> {
  let mut values = Vec::new();
  let mut expected_x = texts[*indices.first()?].1.item.x_pt;
  let mut magnitude = expected_x.abs();
  let mut count = 0;
  for &index in indices {
    let (transformed, text) = &texts[index];
    let style = &text.item.style;
    let adjustments = style.wordprocessing_justification_expansion_pt.as_deref();
    if *transformed
      || (text.item.pdf_text_segmentation != common::PdfTextSegmentation::WordLine
        && !variable_numbering)
      || text.item.paragraph_bidi
      || style.wordprocessing_kashida.is_some()
      || style.rotation_deg != 0.0
      || style.semantic_only
      || style.horizontal_scale.unwrap_or(1.0) != 1.0
      || !style.character_spacing_pt.is_finite()
      || (style.character_spacing_pt != 0.0
        && style.wordprocessing_legacy_font_measurement != Some(false))
      || !text.item.word_spacing_pt.is_finite()
      || (text.item.word_spacing_pt != 0.0
        && (style.character_spacing_pt == 0.0 || text.item.word_spacing_pt < 0.0))
      || style.pdf_glyph_outline_options.is_some()
      || style.semantic_character_advances_pt.is_some()
      || ooxmlsdk_layout::fonts::wordprocessingml_punctuation_affects_advances(
        &text.item.text,
        style,
      )
      || text.item.text.is_empty()
    {
      return None;
    }
    // Admit only FLOAT accumulation error, never a tab/positioned layout gap.
    let error = f32::EPSILON * (count + 8) as f32 * magnitude.max(1.0);
    if (text.item.x_pt - expected_x).abs() > error {
      return None;
    }
    let tracking_pixels = i32::try_from(common::wordprocessing_device::source_coordinate(
      style.character_spacing_pt,
    ))
    .ok()?;
    let logical_size = style
      .layout_font_sizes
      .map_or(style.font_size_pt, |sizes| sizes.primary.0);
    let character_starts = if let Some(adjustments) = adjustments {
      let starts = text
        .item
        .text
        .char_indices()
        .map(|(start, _)| start)
        .collect::<Vec<_>>();
      if starts.len() != adjustments.len() || adjustments.iter().any(|value| !value.is_finite()) {
        return None;
      }
      Some(starts)
    } else {
      None
    };
    for portion in &text.portions {
      if portion.kind == PaintTextPortionKind::Tab {
        return None;
      }
      for run in portion.glyphs.as_ref()? {
        // Mixed script sizes need their own ideal/realized em ownership.
        if run.font_size_pt != style.font_size_pt {
          return None;
        }
        for glyph in &run.glyphs {
          let cluster = text.item.text.get(glyph.text_range.clone())?;
          if cluster.chars().count() != 1
            || cluster.chars().any(char::is_control)
            || glyph.x_advance <= 0.0
            || glyph.x_offset != 0.0
            || glyph.y_offset != 0.0
            || glyph.y_advance != 0.0
          {
            return None;
          }
          // Completed-line adjustments already preserve logical positions in
          // glyph advances normalized by the realized PDF em. Do not scale
          // them a second time by the unrounded layout em.
          let adjustment = if let (Some(values), Some(starts)) = (adjustments, &character_starts) {
            values[starts.binary_search(&glyph.text_range.start).ok()?]
          } else {
            0.0
          } + if cluster == " " {
            // Ordinary positive word expansion shares the source adjustment
            // owner below. Keep it separate from both the font's positioned
            // design advance and authored character pitch.
            text.item.word_spacing_pt
          } else {
            0.0
          };
          let logical_spacing = adjustments.is_some() || style.character_spacing_pt != 0.0;
          let ideal_pt = glyph.x_advance
            * if logical_spacing {
              run.font_size_pt
            } else {
              logical_size
            };
          let ideal = (ideal_pt * IDEAL_UNITS_PER_POINT).round();
          if !ideal.is_finite() || ideal < 0.0 || ideal > MAX_IDEAL_ADVANCE as f32 {
            return None;
          }
          let kerning_advance_em = style.kerning_enabled().then(|| {
            if logical_spacing {
              // A completed-line correction is normalized by the PDF em.
              // Recover its unadjusted logical design advance; the existing
              // independent device correction below still owns compression.
              (ideal_pt - adjustment - style.character_spacing_pt) / logical_size
            } else {
              glyph.x_advance
            }
          });
          let mut natural = cache.device_advance(
            &run.font_face,
            run.font_size_pt,
            glyph.glyph_id,
            cluster,
            kerning_advance_em,
          )?;
          // Word adds authored tracking after realizing the natural font.
          // Native +/-2-twip controls retain +/-410 ideal units but +/-1
          // printer dot. Neither the logical em nor the font's rounding owns
          // this independent source-device contribution.
          natural = natural.checked_add(tracking_pixels)?;
          if adjustment != 0.0 {
            // Line Services compresses the realized trailing bearing before
            // balancing ideal/device widths; the quarter-em comma keeps its
            // origin and its natural advance changes from 88px to 66px.
            let device_adjustment = if adjustment < 0.0 {
              adjustment / logical_size * run.font_size_pt
            } else {
              // Native automatic-gap/CJK-justification arrays convert the
              // completed source adjustment independently of the realized em.
              // A4.11987pt gap adds34dots to a10.56pt font, not35.
              adjustment
            };
            natural = natural.checked_add((device_adjustment / POINTS_PER_PIXEL).round() as i32)?;
          }
          if natural < 0 {
            return None;
          }
          values.push((ideal as i32, natural));
          expected_x += ideal_pt;
          magnitude += ideal_pt.abs();
          count += 1;
        }
      }
    }
  }
  (!values.is_empty()).then_some(values)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn top_aligned_font_offsets_match_native_printer_metrics() {
    let mut cache = NaturalMetricsCache::default();
    // Actual Word 600-DPI cache glyph offsets, regular/bold, 11/25pt.
    // Nyala synthesizes bold; Arial has different regular/bold VDMX records.
    for (family, bold, expected) in [
      ("Nyala", false, [81, 185]),
      ("Nyala", true, [81, 185]),
      ("Arial", false, [82, 185]),
      ("Arial", true, [85, 192]),
      ("Calibri", false, [87, 197]),
      ("Calibri", true, [87, 197]),
    ] {
      let style = super::super::TextStyle {
        font_family: Some(family.into()),
        bold,
        ..Default::default()
      };
      let face = ooxmlsdk_layout::fonts::load_text_face(&style).expect("native control font");
      for (size, expected) in [11.0, 25.0].into_iter().zip(expected) {
        assert_eq!(
          cache.top_offset(&face, size, true),
          Some(expected),
          "{family} {bold} {size}"
        );
      }
    }
  }

  #[test]
  fn variable_font_metrics_match_native_line_service_inputs() {
    let style = super::super::TextStyle {
      font_family: Some("Bahnschrift Light".into()),
      font_size_pt: 11.04,
      ..Default::default()
    };
    let Some(face) =
      ooxmlsdk_layout::fonts::load_text_face(&style).filter(|face| face.named_instance.is_some())
    else {
      return;
    };
    let font = skrifa::FontRef::from_index(face.data.as_slice(), face.index).unwrap();
    let mut cache = NaturalMetricsCache::default();
    // Actual MSLS input, before line-wide balancing; GPOS f/r widths differ
    // from their unpositioned GDI character cells.
    let text = "Default style has roman ";
    let ideal = [
      29568, 24156, 13398, 23980, 25058, 13046, 14696, 12100, 23364, 14696, 21758, 13046, 24156,
      12100, 25168, 23980, 23364, 12100, 18788, 24552, 38588, 23980, 25168, 12100,
    ];
    let native = [
      60, 49, 27, 49, 51, 27, 30, 25, 48, 30, 44, 27, 49, 25, 51, 49, 48, 25, 38, 50, 79, 49, 51,
      25,
    ];
    for (i, ch) in text.chars().enumerate() {
      let glyph = font.charmap().map(ch).unwrap().to_u32();
      assert_eq!(
        cache.device_advance(
          &face,
          11.04,
          glyph,
          &ch.to_string(),
          Some(ideal[i] as f32 / (11.0 * IDEAL_UNITS_PER_POINT)),
        ),
        Some(native[i]),
        "{i}: {ch}",
      );
    }
    // Native GGO metrics: head bit4 is clear. J distinguishes linear advance
    // rounding from a hinted-outline API that reports 47 for this font.
    for (ch, width) in [('i', 23), ('x', 48), ('J', 46), ('f', 29)] {
      let glyph = font.charmap().map(ch).unwrap().to_u32();
      assert_eq!(
        cache.advance(&face, 11.04, glyph, &ch.to_string()),
        Some(width),
      );
    }
  }

  #[test]
  fn native_fractional_spaces_divide_the_realized_dash() {
    let mut cache = NaturalMetricsCache::default();
    for (family, expected) in [
      ("MS Mincho", [[17, 8], [25, 13]]),
      ("MS Gothic", [[17, 8], [25, 13]]),
      ("MS PMincho", [[22, 11], [33, 17]]),
      ("MS PGothic", [[22, 11], [33, 17]]),
    ] {
      let style = super::super::TextStyle {
        font_family: Some(family.into()),
        high_ansi_font_family: Some(family.into()),
        font_size_pt: 12.0,
        ..Default::default()
      };
      let face = ooxmlsdk_layout::fonts::load_text_face(&style).expect("native control face");
      let font = skrifa::FontRef::from_index(face.data.as_slice(), face.index).unwrap();
      for (size, widths) in [12.0, 18.0].into_iter().zip(expected) {
        for (source, width) in ["\u{2004}", "\u{2006}"].into_iter().zip(widths) {
          let glyph = font.charmap().map(source.chars().next().unwrap()).unwrap();
          assert_eq!(
            cache.advance(&face, size, glyph.to_u32(), source),
            Some(width),
            "{family} {size} {source:?}"
          );
        }
      }
    }
  }

  #[test]
  fn native_enabled_kerning_uses_shaped_design_device_metrics() {
    // Actual MSLS70 input arrays: 3 fonts, 14pt logical / 117px realized,
    // same text with a 1pt kerning threshold and a nonbinding 200pt threshold.
    let text = "Universal PROBE minimum WHITE 01234";
    let controls = [
      (
        "Times New Roman",
        [
          41412, 28672, 15932, 28672, 25452, 19096, 22316, 25452, 15932, 14336, 31892, 38248,
          41412, 38248, 35028, 14336, 44604, 15932, 28672, 15932, 44604, 28672, 44604, 13300,
          54124, 41412, 19096, 35028, 35028, 14336, 28672, 28672, 28672, 28672, 28672,
        ],
        [
          84, 59, 33, 59, 52, 39, 46, 52, 33, 29, 65, 78, 84, 78, 71, 29, 91, 33, 59, 33, 91, 59,
          91, 27, 110, 84, 39, 71, 71, 29, 59, 59, 59, 59, 59,
        ],
        [
          85, 59, 33, 59, 52, 39, 46, 52, 33, 29, 65, 78, 85, 78, 72, 29, 91, 33, 59, 33, 91, 59,
          91, 29, 110, 85, 39, 72, 72, 29, 59, 59, 59, 59, 59,
        ],
      ),
      (
        "Arial",
        [
          41412, 31892, 12740, 28672, 31892, 19096, 28672, 31892, 12740, 15932, 38248, 41412,
          44604, 38248, 38248, 15932, 47768, 12740, 31892, 12740, 47768, 31892, 47768, 15932,
          54124, 41412, 15932, 35028, 38248, 15932, 31892, 31892, 31892, 31892, 31892,
        ],
        [
          84, 65, 26, 59, 65, 39, 59, 65, 26, 33, 78, 84, 91, 78, 78, 33, 97, 26, 65, 26, 97, 65,
          97, 33, 110, 84, 33, 71, 78, 33, 65, 65, 65, 65, 65,
        ],
        [
          85, 65, 26, 59, 65, 39, 59, 65, 26, 33, 78, 85, 91, 78, 78, 33, 98, 26, 65, 26, 98, 65,
          98, 33, 111, 85, 33, 72, 78, 33, 65, 65, 65, 65, 65,
        ],
      ),
      (
        "Calibri",
        [
          36792, 30128, 13160, 25340, 28532, 19012, 22428, 27468, 13160, 12964, 29624, 30576,
          37968, 31192, 28000, 12964, 45808, 13160, 30128, 13160, 45808, 30128, 45808, 12964,
          51016, 35728, 14448, 27944, 28000, 12964, 29064, 29064, 29064, 29064, 29064,
        ],
        [
          75, 61, 27, 52, 58, 39, 46, 56, 27, 26, 60, 62, 77, 64, 57, 26, 93, 27, 61, 27, 93, 61,
          93, 26, 104, 73, 29, 57, 57, 26, 59, 59, 59, 59, 59,
        ],
        [
          75, 62, 27, 53, 58, 41, 46, 56, 27, 26, 60, 64, 78, 64, 57, 26, 94, 27, 62, 27, 94, 62,
          94, 26, 104, 73, 30, 57, 57, 26, 59, 59, 59, 59, 59,
        ],
      ),
    ];
    let mut cache = NaturalMetricsCache::default();
    for (family, ideal, positioned, natural) in controls {
      let style = super::super::TextStyle {
        font_family: Some(family.into()),
        high_ansi_font_family: Some(family.into()),
        font_size_pt: 14.04,
        ..Default::default()
      };
      let face = ooxmlsdk_layout::fonts::load_text_face(&style).expect("native control face");
      let font = skrifa::FontRef::from_index(face.data.as_slice(), face.index).unwrap();
      for (index, character) in text.chars().enumerate() {
        let glyph = font.charmap().map(character).unwrap().to_u32();
        let source = character.to_string();
        let advance_em = ideal[index] as f32 / (14.0 * IDEAL_UNITS_PER_POINT);
        assert_eq!(
          cache.device_advance(&face, 14.04, glyph, &source, Some(advance_em)),
          Some(positioned[index]),
          "{family} kerning {index}:{character:?}"
        );
        assert_eq!(
          cache.device_advance(&face, 14.04, glyph, &source, None),
          Some(natural[index]),
          "{family} Natural {index}:{character:?}"
        );
      }
    }
    assert_eq!(cache.design_units.len(), 3);
  }

  #[test]
  fn native_line_frame_and_paragraph_offsets_have_separate_device_owners() {
    // Configured native Word line-input/convert/draw controls, including a
    // plain paragraph and the same paragraph in the original merged cell.
    // Both convert frame232243 to472 dots, then add the85-twip indent35.
    assert_eq!(realize_frame_origin(60.95, -4.25, None), 60.84);
    // The parameter row's third cell has frame699802 ->1424 device dots.
    assert_eq!(realize_frame_origin(175.10, -4.25, None), 175.08);
    assert_eq!(realize_frame_origin(56.7, 0.0, None), 56.64);
    // Native centered control: frame472 + indent35 + alignment2791 dots.
    let source = 60.95_f32;
    let moved = source + 335.00787;
    let actual_offset = f64::from(moved) - f64::from(source);
    let alignment = common::WordprocessingLineAlignment {
      logical_offset_pt: actual_offset,
      device_offset_px: 2791,
    };
    assert_eq!(
      realize_frame_origin(moved, -4.25 - actual_offset, Some(alignment)),
      395.76
    );
  }
  #[test]
  fn native_line_adjustment_preserves_natural_widths_and_balances_end() {
    let ideal = [
      35496, 13656, 24576, 27336, 16368, 12288, 19128, 24576, 40944, 27336, 13656, 21816,
    ];
    let natural = [72, 28, 50, 56, 33, 25, 39, 50, 84, 56, 28, 44];
    let input: Vec<_> = ideal.into_iter().zip(natural).collect();
    assert_eq!(
      balance_advances(&input),
      [72, 28, 50, 56, 33, 25, 39, 50, 84, 56, 28, 43]
    );
  }
  #[test]
  fn native_line_adjustment_overflow() {
    // Readbacks from the Office line-service call, before and after adjustment.
    let ideal = [
      37632, 32768, 32704, 29888, 50464, 29088, 32960, 29088, 32768, 35072, 35072, 26880, 32768,
      30944, 29088, 31840, 29088, 30144, 16384, 32768, 29088, 35072, 32768, 35072, 35072, 29088,
      30144, 16384, 33344, 35072, 33344, 32704, 35072, 32768, 28640, 29088, 31840, 29088,
    ];
    let natural = [
      76, 67, 66, 61, 102, 59, 67, 59, 67, 71, 71, 55, 67, 63, 59, 65, 59, 61, 33, 67, 59, 71, 67,
      71, 71, 59, 61, 33, 68, 71, 68, 66, 71, 67, 58, 59, 65, 59,
    ];
    let expected = [
      76, 67, 66, 61, 102, 60, 67, 59, 67, 71, 71, 55, 67, 63, 60, 65, 59, 61, 33, 67, 59, 71, 67,
      71, 71, 60, 61, 33, 68, 71, 69, 66, 71, 67, 58, 59, 65, 60,
    ];
    let input: Vec<_> = ideal.into_iter().zip(natural).collect();
    assert_eq!(balance_advances(&input), expected);
  }

  #[test]
  fn native_line_adjustment_mixed_runs() {
    // Readbacks from the Office line-service call, before and after adjustment.
    let ideal = [
      32784, 24576, 21816, 26304, 20160, 21816, 24528, 22608, 12288, 16368, 24576, 24576, 24576,
      24576, 16368, 24576, 24576, 24576, 24576, 16368,
    ];
    let natural = [
      67, 50, 44, 54, 41, 44, 50, 46, 25, 33, 50, 50, 50, 50, 33, 50, 50, 50, 50, 33,
    ];
    let expected = [
      67, 50, 44, 54, 41, 44, 50, 46, 25, 33, 50, 50, 50, 50, 33, 50, 50, 50, 50, 34,
    ];
    let input: Vec<_> = ideal.into_iter().zip(natural).collect();
    assert_eq!(balance_advances(&input), expected);
  }
}
