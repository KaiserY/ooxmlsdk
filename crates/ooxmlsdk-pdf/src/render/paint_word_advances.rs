//! Word's ordinary horizontal text realization before PDF serialization.
//!
//! Native MSLS70 observations replay 726 complete line batches exactly. Keep
//! ideal line accumulation across face/style runs, independently of PDF Tc/TJ.
use super::{HashMap, PaintItem, PaintText, PaintTextPortionKind, common};
use emfsdk::render::GdiNaturalMetrics;
use ooxmlsdk_layout::fonts::{FontFaceCacheKey, FontFaceData};
use skrifa::MetadataProvider;
use std::sync::Arc;

const POINTS_PER_PIXEL: f32 = 72.0 / 600.0;
const IDEAL_UNITS_PER_POINT: f32 = 4096.0;
const SCALE: i64 = 4267; // round(600 * 2^21 / (72 * 4096))
const HALF: i64 = 1 << 20;
const LIMIT: i64 = (i32::MAX as i64 - HALF) / SCALE;

#[derive(Default)]
pub(super) struct NaturalMetricsCache {
  bytes: HashMap<FontFaceCacheKey, Arc<[u8]>>,
  instances: HashMap<(FontFaceCacheKey, u16), Option<GdiNaturalMetrics>>,
}
impl NaturalMetricsCache {
  fn advance(
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
    let advance = self
      .instances
      .entry((key, pixels as u16))
      .or_insert_with(|| GdiNaturalMetrics::new(data.clone(), face.index, pixels as u16))
      .as_mut()?
      .glyph_advance_px(measured_glyph)?;
    // Divide the already realized dash width, independently of ideal layout.
    // For example, a 12pt MS Mincho dash is50px; U+2006 is(50+3)/6=8px.
    Some((advance + i32::from(divisor / 2)) / i32::from(divisor))
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct LineKey {
  frame: usize,
  line: usize,
  clip: Option<[u32; 4]>,
}

fn collect_text<'a, 'doc>(
  items: &'a mut [PaintItem<'doc>],
  transformed: bool,
  out: &mut Vec<(bool, &'a mut PaintText<'doc>)>,
) {
  for item in items {
    match item {
      PaintItem::Text(text) => out.push((transformed, text)),
      PaintItem::Group {
        transform, items, ..
      } => collect_text(items, transformed || transform.is_some(), out),
      _ => {}
    }
  }
}

pub(super) fn realize(items: &mut [PaintItem<'_>], cache: &mut NaturalMetricsCache) {
  let mut texts = Vec::new();
  collect_text(items, false, &mut texts);
  // Vertical metrics belong to each completed Word line independently of
  // horizontal segmentation. Number portions use Line rather than WordLine;
  // they must not disable native baseline realization for the whole line.
  for (transformed, text) in &mut texts {
    realize_text_baseline(text, *transformed);
  }
  let mut lines = HashMap::<LineKey, Vec<usize>>::default();
  for (index, (_, text)) in texts.iter().enumerate() {
    let (Some(frame), Some(line)) = (text.source_frame_index, text.source_line_index) else {
      continue;
    };
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
      .entry(LineKey { frame, line, clip })
      .or_default()
      .push(index);
  }
  for indices in lines.values_mut() {
    indices.sort_by(|&a, &b| texts[a].1.item.x_pt.total_cmp(&texts[b].1.item.x_pt));
    let Some(anchor) = realize_line_origin(&mut texts, indices) else {
      continue;
    };
    let Some(advances) = measure_line(&texts, indices, cache) else {
      continue;
    };
    let advances = balance_advances(&advances);
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
}

fn realize_text_baseline(text: &mut PaintText<'_>, transformed: bool) {
  let style = &text.item.style;
  if transformed
    || text.item.paragraph_bidi
    || style.rotation_deg != 0.0
    || style.semantic_only
    || style.horizontal_scale.unwrap_or(1.0) != 1.0
    || style.pdf_glyph_outline_options.is_some()
    || style.semantic_character_advances_pt.is_some()
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
  let delta = baseline - style.baseline_shift_pt - text.baseline_y;
  text.baseline_y += delta;
  for portion in &mut text.portions {
    portion.baseline_y += delta;
  }
}

fn realize_line_origin(texts: &mut [(bool, &mut PaintText<'_>)], indices: &[usize]) -> Option<f32> {
  for &index in indices {
    let (transformed, text) = &texts[index];
    let style = &text.item.style;
    if *transformed
      || text.item.pdf_text_segmentation != common::PdfTextSegmentation::WordLine
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

fn realize_frame_origin(
  source: f32,
  frame_offset: f64,
  alignment: Option<common::WordprocessingLineAlignment>,
) -> f32 {
  // Actual WWLIB line/convert/draw readbacks distinguish the frame's integral
  // 1/4096pt point from the paragraph-local printer offset. For example,
  // 56.7pt + 4.25pt prints at 472 + 35 dots, not round(60.95pt / .12).
  let device = common::wordprocessing_device::source_coordinate_precise;
  let frame = f64::from(source) + frame_offset;
  let (local, alignment_offset) = alignment.map_or((-frame_offset, 0), |alignment| {
    (
      -frame_offset - alignment.logical_offset_pt,
      alignment.device_offset_px,
    )
  });
  ((device(frame) + device(local) + i64::from(alignment_offset)) as f64 * 72.0 / 600.0) as f32
}

/// Each pair is (ideal advance in 1/4096pt, natural device advance in pixels).
fn measure_line(
  texts: &[(bool, &mut PaintText<'_>)],
  indices: &[usize],
  cache: &mut NaturalMetricsCache,
) -> Option<Vec<(i32, i32)>> {
  let mut values = Vec::new();
  let mut expected_x = texts[*indices.first()?].1.item.x_pt;
  let mut magnitude = expected_x.abs();
  let mut count = 0;
  for &index in indices {
    let (transformed, text) = &texts[index];
    let style = &text.item.style;
    let terminal_adjustment = terminal_compression(&text.item.text, style);
    if *transformed
      || text.item.pdf_text_segmentation != common::PdfTextSegmentation::WordLine
      || text.item.paragraph_bidi
      || style.rotation_deg != 0.0
      || style.semantic_only
      || style.horizontal_scale.unwrap_or(1.0) != 1.0
      || style.character_spacing_pt != 0.0
      || text.item.word_spacing_pt != 0.0
      || (style.wordprocessing_justification_expansion_pt.is_some()
        && terminal_adjustment.is_none())
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
    let logical_size = style
      .layout_font_sizes
      .map_or(style.font_size_pt, |sizes| sizes.primary.0);
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
          let ideal_pt = glyph.x_advance
            * if terminal_adjustment.is_some() {
              run.font_size_pt
            } else {
              logical_size
            };
          let ideal = (ideal_pt * IDEAL_UNITS_PER_POINT).round();
          if !ideal.is_finite() || ideal < 0.0 || ideal > LIMIT as f32 {
            return None;
          }
          let mut natural =
            cache.advance(&run.font_face, run.font_size_pt, glyph.glyph_id, cluster)?;
          if glyph.text_range.end == text.item.text.len()
            && let Some(adjustment) = terminal_adjustment
          {
            // Line Services compresses the realized trailing bearing before
            // balancing ideal/device widths; the quarter-em comma keeps its
            // origin and its natural advance changes from 88px to 66px.
            natural +=
              (adjustment / logical_size * run.font_size_pt / POINTS_PER_PIXEL).round() as i32;
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

fn terminal_compression(text: &str, style: &super::TextStyle<'_>) -> Option<f32> {
  let adjustments = style.wordprocessing_justification_expansion_pt.as_deref()?;
  let (&last, preceding) = adjustments.split_last()?;
  (last < 0.0
    && adjustments.len() == text.chars().count()
    && preceding.iter().all(|&value| value == 0.0))
  .then_some(last)
}

fn balance_advances(advances: &[(i32, i32)]) -> Vec<i32> {
  let mut output: Vec<i32> = Vec::with_capacity(advances.len());
  let mut ideal_sum = 0i64;
  let mut previous_rounded = 0;
  let mut previous_error = 0;
  let mut previous = false;
  for &(ideal, natural) in advances {
    if ideal_sum + i64::from(ideal) > LIMIT {
      // Native restarts before its signed 32-bit multiply would overflow.
      ideal_sum = 0;
      previous_rounded = 0;
      previous = false;
    }
    ideal_sum += i64::from(ideal);
    let rounded = ((ideal_sum * SCALE + HALF) >> 21) as i32;
    let mut value = rounded - previous_rounded;
    let error = natural - value;
    if previous {
      let difference = error - previous_error;
      let floor = difference >> 1;
      let ceil = floor + (difference & 1);
      let prior = output.last_mut().expect("previous advance");
      let delta = if difference > 0 {
        (*prior).min(if previous_error >= -error {
          floor
        } else {
          ceil
        })
      } else if difference < 0 {
        (-value).max(if error >= -previous_error {
          floor
        } else {
          ceil
        })
      } else {
        0
      };
      *prior -= delta;
      value += delta;
    }
    output.push(value);
    previous_error = natural - value;
    previous_rounded = rounded;
    previous = true;
  }
  output
}

#[cfg(test)]
mod tests {
  use super::*;
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
