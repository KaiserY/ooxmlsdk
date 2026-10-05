//! Natural device realization of completed Arabic text before a fixed tab.
//!
//! Word keeps ideal advances for fitting, then retains printer glyph widths
//! and absorbs positive ideal/device error at interword blanks. Read-only MSO
//! glyph arrays and GDI tab observations establish these separate owners.

use super::{PageItem, ParagraphAlignment, TextMetrics, TextStyle, common};
use icu_properties::{CodePointMapData, props::Script};
use std::sync::Arc;

const PIXEL_PT: f32 = 72.0 / 600.0;

#[derive(Default)]
struct SpaceBalance {
  ideal: i64,
  device: i64,
}

impl SpaceBalance {
  fn advance(&mut self, ideal: i64, natural: i64, blank: bool) -> Option<i64> {
    self.ideal = self.ideal.checked_add(ideal)?;
    // Line Services' 21-bit scale, also used by ordinary Word text. Keep
    // its fixed-point conversion: exact rational DPI differs at half ties.
    let target = self.ideal.checked_mul(4267)?.checked_add(1 << 20)? >> 21;
    let result = if blank {
      natural.max(target.checked_sub(self.device)?)
    } else {
      natural
    };
    self.device = self.device.checked_add(result)?;
    Some(result)
  }
}

struct RunPlan {
  index: usize,
  style: TextStyle,
  pixels: i64,
}

fn plan(items: &[PageItem], y: f32, metrics: &mut TextMetrics) -> Option<(f32, Vec<RunPlan>)> {
  let scripts = CodePointMapData::<Script>::new();
  let mut indices = Vec::new();
  let mut anchor = None;
  let mut has_arabic = false;
  for (index, item) in items.iter().enumerate() {
    let PageItem::Text(text) = item else {
      return None;
    };
    if (text.y_pt - y).abs() >= 0.01
      || !text.paragraph_bidi
      || text.dynamic_field.is_some()
      || text.word_spacing_pt != 0.0
      || text.style.wordprocessing_legacy_font_measurement != Some(false)
      || text.style.wordprocessing_kashida.is_some()
      || text.style.kashida_expansions.is_some()
      || text.style.semantic_character_advances_pt.is_some()
      || text.style.small_caps
      || text
        .style
        .resolved_bidi_level
        .is_none_or(|level| level % 2 == 0)
    {
      return None;
    }
    let line_anchor = text.dynamic_field_line_anchor?;
    // A left tab in the authored RTL paragraph becomes a physical right
    // anchor. Center/right tabs and fields retain their deferred owners.
    if line_anchor.alignment != ParagraphAlignment::Right
      || line_anchor.aligned_tab.is_none()
      || anchor.is_some_and(|x| x != line_anchor.position_pt)
    {
      return None;
    }
    anchor = Some(line_anchor.position_pt);
    for ch in text.text.chars() {
      match scripts.get(ch) {
        Script::Arabic => has_arabic = true,
        Script::Common | Script::Inherited => {}
        _ => return None,
      }
    }
    indices.push(index);
  }
  if !has_arabic {
    return None;
  }
  // After UAX #9 ordering, logical RTL runs proceed from right to left.
  indices.sort_by(|&a, &b| {
    let (PageItem::Text(a), PageItem::Text(b)) = (&items[a], &items[b]) else {
      unreachable!("text indices");
    };
    b.x_pt.total_cmp(&a.x_pt)
  });
  let mut balance = SpaceBalance::default();
  let mut plans = Vec::new();
  for index in indices {
    let PageItem::Text(text) = &items[index] else {
      unreachable!("text indices");
    };
    let ideal = metrics.shape_text(&text.text, &text.style)?;
    // Zero-width directional controls do not become printer glyphs merely
    // because the preceding visible text now has device metrics.
    if ideal.width_pt == 0.0 {
      plans.push(RunPlan {
        index,
        style: text.style.clone(),
        pixels: 0,
      });
      continue;
    }
    let mut style = text.style.clone();
    let mut device = common::WordprocessingKashida {
      retain_trailing_blank: true,
      unshaped_blanks: false,
      font_width_percent: style.wordprocessing_font_width_percent.unwrap_or(100),
      space_expansions: Vec::new(),
    };
    style.wordprocessing_kashida = Some(Arc::new(device.clone()));
    let natural = metrics.shape_text(&text.text, &style)?;
    if ideal.glyphs.len() != natural.glyphs.len() {
      return None;
    }
    let start = balance.device;
    for (i, n) in ideal.glyphs.iter().zip(&natural.glyphs).rev() {
      if i.glyph_id != n.glyph_id || i.text_range != n.text_range {
        return None;
      }
      let ideal_width = i.x_advance_em * i.font_size_pt * 4096.0;
      let natural_width = n.x_advance_em * n.font_size_pt / PIXEL_PT;
      if !ideal_width.is_finite() || !natural_width.is_finite() {
        return None;
      }
      let blank = text.text.get(i.text_range.clone()) == Some(" ");
      let natural_pixels = natural_width.round() as i64;
      let advance = balance.advance(ideal_width.round() as i64, natural_pixels, blank)?;
      if advance != natural_pixels {
        device.space_expansions.push((
          i.text_range.start,
          i32::try_from(advance - natural_pixels).ok()?,
        ));
      }
    }
    style.wordprocessing_kashida = Some(Arc::new(device));
    plans.push(RunPlan {
      index,
      style,
      pixels: balance.device - start,
    });
  }
  Some((anchor?, plans))
}

pub(super) fn realize(
  items: &mut [PageItem],
  item_start: usize,
  tab_starts: &[usize],
  y: f32,
  frame_right: f32,
  metrics: &mut TextMetrics,
) {
  let mut starts = vec![item_start];
  starts.extend(
    tab_starts
      .iter()
      .copied()
      .filter(|&i| i >= item_start && i <= items.len()),
  );
  starts.push(items.len());
  for range in starts.windows(3) {
    let [start, end, next_end] = [range[0], range[1], range[2]];
    if start == end {
      continue;
    }
    let Some((anchor, plans)) = plan(&items[start..end], y, metrics) else {
      continue;
    };
    let source = common::wordprocessing_device::source_coordinate_precise;
    let right = source(f64::from(frame_right)) - source(f64::from(frame_right) - f64::from(anchor));
    let mut cursor = right;
    for run in plans {
      cursor -= run.pixels;
      let PageItem::Text(text) = &mut items[start + run.index] else {
        unreachable!("planned text");
      };
      text.x_pt = cursor as f32 * PIXEL_PT;
      text.style = run.style;
      if let Some(anchor) = &mut text.dynamic_field_line_anchor {
        anchor.position_pt = right as f32 * PIXEL_PT;
      }
    }
    // The following tab keeps its fixed far edge. Only its text-side edge
    // changes when the completed preceding span acquires printer widths.
    for item in &mut items[end..next_end] {
      let PageItem::Text(text) = item else { continue };
      let Some(native) = text.style.wordprocessing_tab_leader else {
        continue;
      };
      if text.paragraph_bidi || (text.y_pt - y).abs() >= 0.01 {
        continue;
      }
      let Some(advance) = text
        .style
        .semantic_character_advances_pt
        .as_deref()
        .and_then(|advances| advances.first())
        .copied()
      else {
        continue;
      };
      let (x, count) = super::word_tab_leader_device_span(
        text.x_pt,
        cursor as f32 * PIXEL_PT,
        native.source_advance_pt,
      );
      text.x_pt = x;
      text.text = ".".repeat(count);
      text.style.semantic_character_advances_pt = Some(vec![advance; count].into());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::SpaceBalance;

  #[test]
  fn modern_arabic_spaces_retain_native_width_and_absorb_positive_error() {
    // Native TA14/90% GetGlyphPlacements -> MSO unexpanded glyph inputs.
    let ideal = [
      7588, 8624, 9548, 9548, 9548, 14364, 12600, 7588, 23828, 24696, 10024, 9548, 14364, 12600,
      13076, 7588, 8624, 10024, 19040, 10584, 9548, 14364, 12600, 7588, 8624, 9548, 19964, 9548,
      10472, 14364,
    ];
    let natural = [
      15, 17, 19, 19, 19, 29, 25, 15, 48, 50, 20, 19, 29, 25, 26, 15, 17, 20, 38, 21, 19, 29, 25,
      15, 17, 19, 40, 19, 21, 29,
    ];
    let expected = [
      15, 17, 19, 19, 19, 29, 28, 15, 48, 50, 20, 19, 29, 28, 26, 15, 17, 20, 38, 21, 19, 29, 30,
      15, 17, 19, 40, 19, 21, 29,
    ];
    let mut balance = SpaceBalance::default();
    for (index, ((ideal, natural), expected)) in
      ideal.into_iter().zip(natural).zip(expected).enumerate()
    {
      assert_eq!(
        balance.advance(ideal, natural, [6, 13, 22].contains(&index)),
        Some(expected)
      );
    }
    assert_eq!(balance.device, 730);
    // Negative error never condenses an unexpanded Arabic blank.
    assert_eq!(balance.advance(0, 25, true), Some(25));
  }
}
