//! Completed Word RTL lines allocate Kashida on the source printer grid.
//! Ideal document metrics retain line fitting. Read-only Office observations
//! establish integer glyph advances, minimum extenders, and space remainders;
//! the per-run expansion contract is GetJustifiedGlyphs (dwrite_1.h).

use super::{LineJustificationContext, PageItem, TextMetrics, common, units};
use std::sync::Arc;

const PIXEL_PT: f32 = units::POINTS_PER_INCH / units::OFFICE_FIXED_OUTPUT_DPI;

pub(super) fn adjust(
  items: &mut [PageItem],
  line: LineJustificationContext,
  last_text_index: Option<usize>,
  metrics: &mut TextMetrics,
) -> Option<bool> {
  if line.terminal_overhang_pt != 0.0 {
    return None;
  }
  let mut visual = Vec::new();
  let mut natural_pixels = 0i64;
  let mut tail_pixels = 0i64;
  let mut natural_tail_pixels = 0i64;
  let mut tail_is_rtl = true;
  let mut ltr_blank_tail = false;
  for (index, item) in items.iter().enumerate().skip(line.item_start) {
    let PageItem::Text(text) = item else {
      continue;
    };
    if (text.y_pt - line.y_pt).abs() >= 0.01 {
      continue;
    }
    // Keep the existing path for geometric stretching,
    // leading preserved blanks, and non-Word or non-RTL paragraph owners.
    if !text.paragraph_bidi
      || text.style.wordprocessing_legacy_font_measurement.is_none()
      || !text.style.character_spacing_pt.is_finite()
      || text.style.small_caps
      || text.style.kashida_expansions.is_some()
      || (text.text.starts_with(' ') && index == line.item_start)
    {
      return None;
    }
    let mut style = text.style.clone();
    style.wordprocessing_kashida = Some(Arc::new(common::WordprocessingKashida {
      retain_trailing_blank: false,
      unshaped_blanks: !text.text.is_empty()
        && text.text.bytes().all(|byte| byte == b' ')
        && style.right_to_left != Some(true)
        && style.complex_script != Some(true),
      font_width_percent: style.wordprocessing_font_width_percent.unwrap_or(100),
      space_expansions: Vec::new(),
    }));
    let shaped = metrics.shape_text(&text.text, &style)?;
    let width = (shaped.width_pt / PIXEL_PT).round() as i64;
    if Some(index) == last_text_index {
      tail_is_rtl = style
        .resolved_bidi_level
        .map_or(style.right_to_left.unwrap_or(false), |level| level % 2 != 0);
      ltr_blank_tail = !tail_is_rtl && text.text.bytes().all(|byte| byte == b' ');
      let visible = text.text.trim_end_matches(' ');
      natural_tail_pixels =
        width - (metrics.measure_text(visible, &style) / PIXEL_PT).round() as i64;
      // A Word bidi embedding owns its complete portion, including the
      // logical final blank. Native w:dir controls retain that blank inside
      // the device target; ordinary RTL portions leave it outside the line.
      let embedded_tail = style
        .wordprocessing_bidi_scopes
        .as_deref()
        .is_some_and(|scopes| !scopes.is_empty())
        && style.resolved_bidi_level.is_none_or(|level| level > 1);
      if tail_is_rtl && !embedded_tail {
        tail_pixels = natural_tail_pixels;
      }
    }
    natural_pixels += width;
    visual.push((index, text.x_pt, style, width));
  }
  if visual.is_empty() {
    return None;
  }
  // Native Word retains the cell-local line width when a table moves on the
  // page. Rounding its two page coordinates separately changes the budget by
  // a device pixel and can add or remove an extender at a copy-count boundary.
  let target_pixels = device_target_width(line);
  visual.sort_by(|a, b| a.1.total_cmp(&b.1));
  let mut candidates = Vec::new();
  for (index, _, style, _) in &visual {
    let PageItem::Text(text) = &items[*index] else {
      unreachable!("visual indices contain only text");
    };
    let mut opportunities = metrics.kashida_opportunities(&text.text, style);
    opportunities.reverse();
    candidates.extend(opportunities.into_iter().map(|position| {
      (
        *index,
        position.byte_index,
        (position.minimum_width_pt / PIXEL_PT).round() as i64,
        position.glyph_width_pt,
      )
    }));
  }
  let candidate_count = candidates.len();
  // Native lowKashida foreign lines use integer advances and interword glue
  // even when no Arabic connection is available (ScriptJustify's fallback).
  // Their LTR tail stays inside the allocated line, unlike an RTL semantic
  // tail on the physical left. A separate LTR blank likewise owns its full
  // width; native controls distinguish ordinary, complex-script and RTL
  // blanks. Mixed foreign ink additionally contributes its ordinary spaces
  // to the weighted budget below.
  let foreign_line = candidate_count == 0 && !tail_is_rtl;
  let mixed_foreign_ink = candidate_count != 0
    && !tail_is_rtl
    && !ltr_blank_tail
    && visual.iter().any(|(index, _, style, _)| {
      matches!(&items[*index], PageItem::Text(text) if text.text.contains(' '))
        && style.right_to_left != Some(true)
        && style.complex_script != Some(true)
        && style
          .resolved_bidi_level
          .is_some_and(|level| level % 2 == 0)
    });
  // A wrapped foreign line leaves its semantic final blanks beyond the
  // justification width. Native Word retains those blanks inside a fixed
  // tab span or at an authored soft break, which have separate boundaries.
  let exclude_foreign_tail = foreign_line && !line.preserve_foreign_tail;
  if exclude_foreign_tail {
    tail_pixels = natural_tail_pixels;
  }
  let extra = target_pixels - (natural_pixels - tail_pixels);
  if extra <= 0 || extra > i64::from(i32::MAX) {
    // A mixed line whose device advances already fill its target has no
    // Kashida budget either. Falling back to ideal advances here invents
    // extenders; ordinary space contraction retains its existing owner.
    return (tail_is_rtl || mixed_foreign_ink).then_some(false);
  }
  if (candidate_count == 0 && !foreign_line)
    || (candidate_count != 0 && !tail_is_rtl && !ltr_blank_tail && !mixed_foreign_ink)
  {
    return None;
  }
  let mut minimum_pixels = candidates
    .iter()
    .map(|(_, _, minimum, _)| minimum)
    .sum::<i64>();
  let mut foreign_spaces = Vec::new();
  if !foreign_line {
    for (index, _, style, width) in &visual {
      let unshaped_blanks = style
        .wordprocessing_kashida
        .as_ref()
        .is_some_and(|device| device.unshaped_blanks);
      let foreign_text = mixed_foreign_ink
        && style.right_to_left != Some(true)
        && style.complex_script != Some(true)
        && style
          .resolved_bidi_level
          .is_some_and(|level| level % 2 == 0);
      if !unshaped_blanks && !foreign_text {
        continue;
      }
      let PageItem::Text(text) = &items[*index] else {
        unreachable!("visual indices contain only text");
      };
      let minimum = if unshaped_blanks {
        width / text.text.len() as i64
      } else {
        (metrics.measure_text(" ", style) / PIXEL_PT).round() as i64
      };
      foreign_spaces.extend(
        text
          .text
          .char_indices()
          .filter(|(_, ch)| *ch == ' ')
          .map(|(byte, _)| (*index, byte, minimum)),
      );
    }
  }
  let mut foreign_minimum = foreign_spaces
    .iter()
    .map(|(_, _, minimum)| minimum)
    .sum::<i64>();
  let partial_foreign = mixed_foreign_ink && minimum_pixels + foreign_minimum > extra;
  if !partial_foreign
    && (!tail_is_rtl || foreign_minimum != 0)
    && minimum_pixels + foreign_minimum > extra
  {
    // Full-selection native controls establish the shared weighted budget.
    // Preserve the previous owner for unprobed partial foreign selections.
    return None;
  }
  if minimum_pixels > extra || (partial_foreign && minimum_pixels == extra) {
    // A partial selection keeps positive space slack. Native Word selects
    // eight seven-pixel connections from a 63-pixel budget, but retains all
    // fourteen when their complete minimum is exactly 98 pixels.
    while !candidates.is_empty() && minimum_pixels >= extra {
      minimum_pixels -= candidates.pop().expect("nonempty selection").2;
    }
  }
  // Both source spaces and the fractional join allocation follow logical
  // order. Selection above retains visual order when only some joins fit.
  let mut spaces = Vec::new();
  for (index, item) in items.iter().enumerate().skip(line.item_start) {
    let PageItem::Text(text) = item else {
      continue;
    };
    if (text.y_pt - line.y_pt).abs() >= 0.01 {
      continue;
    }
    let visible = if (tail_is_rtl || exclude_foreign_tail) && Some(index) == last_text_index {
      text.text.trim_end_matches(' ')
    } else {
      &text.text
    };
    spaces.extend(
      visible
        .char_indices()
        .filter(|(_, character)| *character == ' ')
        .filter(|(byte, _)| {
          !partial_foreign
            || foreign_spaces
              .iter()
              .any(|(owner, slot, _)| *owner == index && slot == byte)
        })
        .map(|(byte_index, _)| (index, byte_index)),
    );
  }
  let limited = (partial_foreign || candidates.len() < candidate_count) && !spaces.is_empty();
  // Native mixed-line controls retain each selected join's minimum until
  // the complete set of foreign space widths can also fit. The remaining
  // pixels belong only to those foreign spaces, with a cumulative quotient.
  // Once complete, the same natural space widths weight the shared budget.
  if partial_foreign {
    foreign_spaces.clear();
    foreign_minimum = 0;
  }
  let mut weighted = candidates
    .iter()
    .map(|&(index, byte, minimum, glyph_width)| (index, byte, minimum, Some(glyph_width)))
    .chain(
      foreign_spaces
        .iter()
        .map(|&(index, byte, minimum)| (index, byte, minimum, None)),
    )
    .collect::<Vec<_>>();
  weighted.sort_by_key(|&(index, byte, _, _)| (index, byte));
  minimum_pixels += foreign_minimum;
  let mut expansions = Vec::new();
  let mut weighted_spaces = Vec::new();
  let mut preceding_minimum = 0;
  for (index, byte_index, minimum, glyph_width_pt) in &weighted {
    let pixels = if limited {
      *minimum
    } else {
      // Native Word weights each connection by its integer minimum, carrying
      // the fractional quotient across fonts, sizes and bidi portions. Equal
      // minima retain the uniform carry; nominal outline widths do not own it.
      (preceding_minimum + minimum) * extra / minimum_pixels
        - preceding_minimum * extra / minimum_pixels
    };
    preceding_minimum += minimum;
    let Some(glyph_width_pt) = glyph_width_pt else {
      weighted_spaces.push((*index, *byte_index, pixels));
      continue;
    };
    let advance = pixels as f32 * PIXEL_PT;
    expansions.push((
      *index,
      pixels,
      common::KashidaExpansion {
        byte_index: *byte_index,
        advance: common::Pt(advance),
        glyph_count: (advance / *glyph_width_pt).ceil().max(1.0) as usize,
      },
    ));
  }
  let space_extra = extra - expansions.iter().map(|(_, pixels, _)| *pixels).sum::<i64>();
  if space_extra > 0 && spaces.is_empty() {
    return Some(false);
  }
  let mut cursor = line.left_pt
    - if tail_is_rtl {
      tail_pixels as f32 * PIXEL_PT
    } else {
      0.0
    };
  for (index, _, mut style, width) in visual {
    let PageItem::Text(text) = &mut items[index] else {
      unreachable!("visual indices contain only text");
    };
    let selected = expansions
      .iter()
      .filter(|(owner, _, _)| *owner == index)
      .map(|(_, _, expansion)| expansion.clone())
      .collect::<Vec<_>>();
    let device = Arc::make_mut(style.wordprocessing_kashida.as_mut().expect("device owner"));
    device.retain_trailing_blank =
      !tail_is_rtl && !exclude_foreign_tail && Some(index) == last_text_index;
    for (order, (owner, byte_index)) in spaces.iter().enumerate() {
      if *owner == index {
        let pixels = if foreign_minimum != 0 {
          // Ordinary foreign blanks share the complete-selection budget with
          // joins, weighted by their own natural device width. Arabic shaped
          // blanks and explicit complex-script blanks do not add this weight.
          weighted_spaces
            .iter()
            .find(|(slot_owner, slot_byte, _)| slot_owner == owner && slot_byte == byte_index)
            .map_or(0, |(_, _, pixels)| *pixels)
        } else if foreign_line || partial_foreign {
          // Office XPS controls retain the fractional quotient between LTR
          // blanks, including the trailing blank. Do not put every remainder
          // at the start or exclude that invisible tail from the budget.
          (order as i64 + 1) * space_extra / spaces.len() as i64
            - order as i64 * space_extra / spaces.len() as i64
        } else {
          space_extra / spaces.len() as i64
            + i64::from((order as i64) < space_extra % spaces.len() as i64)
        };
        if pixels != 0 {
          device.space_expansions.push((*byte_index, pixels as i32));
        }
      }
    }
    text.x_pt = cursor;
    cursor += width as f32 * PIXEL_PT
      + selected
        .iter()
        .map(|expansion| expansion.advance.0)
        .sum::<f32>()
      + device
        .space_expansions
        .iter()
        .map(|(_, pixels)| *pixels as f32 * PIXEL_PT)
        .sum::<f32>();
    if !selected.is_empty() {
      style.kashida_expansions = Some(selected.into());
    }
    text.style = style;
    text.word_spacing_pt = 0.0;
  }
  Some(true)
}

fn device_target_width(line: LineJustificationContext) -> i64 {
  use common::wordprocessing_device::{source_coordinate, source_coordinate_precise};

  if let Some(edge) = line.source_frame_right_pt {
    // Word independently converts the endpoints relative to their containing
    // frame, including paragraph indents and fixed tab stops. Converting only
    // the span loses a pixel at some boundaries; using absolute page positions
    // instead makes the allocation depend on table or margin translation.
    let edge = f64::from(edge);
    return source_coordinate_precise(edge - f64::from(line.left_pt))
      - source_coordinate_precise(edge - f64::from(line.right_pt));
  }
  source_coordinate(line.right_pt - line.left_pt)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::layout::PdfTextSegmentation;
  use crate::docx::{TextStyle, layout::TextItem};

  #[test]
  fn source_frame_endpoints_match_native_tab_and_indent_budgets() {
    // Office's 32 crossed controls: four tab stops, four end indents, and
    // two translated frames. Both first and continuation lines are observed.
    let first_targets = [
      [2694, 2693, 2693, 2692],
      [2693, 2692, 2692, 2691],
      [2693, 2692, 2692, 2691],
      [2692, 2691, 2691, 2690],
    ];
    let following_targets = [2977, 2976, 2976, 2975];
    for shift in [0.0, 0.05] {
      for (tab, targets) in first_targets.iter().enumerate() {
        for (indent, target) in targets.iter().enumerate() {
          let line = LineJustificationContext {
            item_start: 0,
            y_pt: 0.0,
            left_pt: 119.05 + indent as f32 * 0.05 + shift,
            right_pt: 442.25 - tab as f32 * 0.05 + shift,
            terminal_overhang_pt: 0.0,
            source_frame_right_pt: Some(538.65 + shift),
            preserve_foreign_tail: true,
          };
          assert_eq!(device_target_width(line), *target);
          assert_eq!(
            device_target_width(LineJustificationContext {
              right_pt: 476.3 + shift,
              ..line
            }),
            following_targets[indent]
          );
        }
      }
    }
  }

  fn word_device_items(portions: &[(f32, &str, u8)]) -> Vec<PageItem> {
    let style = TextStyle {
      font_family: Some(Arc::from("Times New Roman")),
      complex_font_family: Some(Arc::from("Traditional Arabic")),
      font_size_pt: 11.0,
      complex_font_size_pt: Some(15.0),
      horizontal_scale: Some(1.03),
      wordprocessing_font_width_percent: Some(103),
      wordprocessing_legacy_font_measurement: Some(true),
      wordprocessingml_font_slots: true,
      right_to_left: Some(true),
      resolved_bidi_level: Some(1),
      ..Default::default()
    };
    portions
      .iter()
      .map(|&(x_pt, text, level)| {
        PageItem::Text(Box::new(TextItem {
          x_pt,
          y_pt: 20.0,
          line_height_pt: 20.0,
          wordprocessing_auto_line_spacing_units: None,
          wordprocessing_line_metrics: None,
          origin_is_baseline: false,
          line_metrics_participant: true,
          paint_clip: None,
          wordprocessing_effect_host: None,
          wordprocessing_terminal_effect_style: None,
          text: text.to_owned(),
          style: TextStyle {
            resolved_bidi_level: Some(level),
            ..style.clone()
          },
          rotation_center_pt: None,
          hyperlink_url: None,
          dynamic_field: None,
          dynamic_field_line_anchor: None,
          style_ref_keys: Vec::new(),
          style_ref_text: None,
          style_ref_numbering_text: None,
          form_widget_id: None,
          paragraph_bidi: true,
          word_spacing_pt: 0.0,
          preserve_text_portion: false,
          decoration_span_start_x_pt: None,
          pdf_text_segmentation: PdfTextSegmentation::Line,
        }))
      })
      .collect()
  }

  #[test]
  fn word_device_mixed_foreign_spaces_share_complete_and_partial_budgets() {
    // Read-only native MSO arrays and XPS: the tracked mixed line has a
    // 15-pixel budget, a six-pixel Arabic join, and nine pixels shared by
    // twelve Latin blanks. The untracked shorter line has 312 pixels and
    // weights the join against eleven 19-pixel blanks: join8, spaces28/27/28.
    let latin = "Fundamental Rights Agency, “Apprehension of migrants in an irregular situation – ";
    let mut metrics = TextMetrics::new();
    for (tracking, suffix, right, expected) in [
      (-0.1, "fundamental ", 476.3, Some(6)),
      (0.0, "", 476.3, Some(8)),
      (0.0, "fundamental ", 484.3, None),
    ] {
      let source = format!("{latin}{suffix}");
      let mut items = word_device_items(&[(450.0, "انظر ", 1), (119.05, &source, 2)]);
      for (index, item) in items.iter_mut().enumerate() {
        let PageItem::Text(text) = item else {
          unreachable!()
        };
        text.style.font_size_pt = 9.0;
        text.style.complex_font_size_pt = Some(13.0);
        text.style.horizontal_scale = Some(1.0);
        text.style.wordprocessing_font_width_percent = Some(100);
        text.style.wordprocessing_legacy_font_measurement = Some(false);
        text.style.kerning_minimum_size_pt = Some(f32::INFINITY);
        text.style.ligatures = Some(common::OpenTypeLigatures::default());
        text.style.use_windows_font_metrics = true;
        text.style.character_spacing_pt = tracking;
        text.style.right_to_left = (index == 0).then_some(true);
      }
      assert_eq!(
        adjust(
          &mut items,
          LineJustificationContext {
            item_start: 0,
            y_pt: 20.0,
            left_pt: 119.05,
            right_pt: right,
            terminal_overhang_pt: 0.0,
            source_frame_right_pt: None,
            preserve_foreign_tail: true,
          },
          Some(1),
          &mut metrics,
        ),
        Some(expected.is_some()),
      );
      let PageItem::Text(arabic) = &items[0] else {
        unreachable!()
      };
      let Some(pixels) = expected else {
        assert!(arabic.style.kashida_expansions.is_none());
        continue;
      };
      let expansions = arabic.style.kashida_expansions.as_deref().unwrap();
      assert_eq!(expansions.len(), 1);
      assert!((expansions[0].advance.0 / PIXEL_PT - pixels as f32).abs() < 0.001);
      assert_eq!(expansions[0].glyph_count, 2);
      assert!(
        arabic
          .style
          .wordprocessing_kashida
          .as_ref()
          .unwrap()
          .space_expansions
          .is_empty()
      );
      let PageItem::Text(english) = &items[1] else {
        unreachable!()
      };
      let spaces = &english
        .style
        .wordprocessing_kashida
        .as_ref()
        .unwrap()
        .space_expansions;
      if tracking < 0.0 {
        assert_eq!(spaces.iter().map(|(_, pixels)| pixels).sum::<i32>(), 9);
        assert!(spaces.iter().all(|(_, pixels)| *pixels == 1));
        assert_eq!(spaces[0].0, source.match_indices(' ').nth(1).unwrap().0);
      } else {
        assert_eq!(
          spaces
            .iter()
            .take(3)
            .map(|(_, pixels)| *pixels)
            .collect::<Vec<_>>(),
          [28, 27, 28]
        );
        assert_eq!(spaces.iter().map(|(_, pixels)| pixels).sum::<i32>(), 304);
      }
    }
  }

  #[test]
  fn word_device_embedding_retains_its_logical_tail_in_the_line_budget() {
    // Independent native Word controls at 15pt/103%: the same wrapped
    // Arabic line owns 2977 pixels inside w:dir, including its 31-pixel
    // final blank. The ordinary RTL line leaves that blank outside its
    // target and paints 24 extenders instead of 16.
    let source = "بالقرآن بسم الله المغتربين وأيضاً أثناء المراهقة وتنمية المهارات وتعزيز الصحة بالقرآن بسم الله ";
    let mut metrics = TextMetrics::new();
    for (direction, consumed) in [
      (None, false),
      (Some(false), false),
      (Some(true), false),
      (Some(false), true),
      (Some(true), true),
    ] {
      let mut items = word_device_items(&[(119.05, source, 1)]);
      let PageItem::Text(text) = &mut items[0] else {
        unreachable!()
      };
      if let Some(right_to_left) = direction {
        text.style.resolved_bidi_level = Some(3);
        text.style.wordprocessing_bidi_scopes =
          Some(Arc::from(vec![crate::model::WordprocessingBidiScope {
            id: 1,
            right_to_left,
            override_direction: false,
          }]));
      }
      if direction.is_some() {
        let PageItem::Text(text) = &mut items[0] else {
          unreachable!()
        };
        text.style.wordprocessing_bidi_prefix = consumed.then(|| Arc::from("\u{202c}"));
        super::super::reorder_bidi_line_items(&mut items, 0, 20.0, &mut metrics);
        let PageItem::Text(text) = &items[0] else {
          unreachable!()
        };
        assert_eq!(
          text.style.resolved_bidi_level,
          Some(if consumed { 1 } else { 3 })
        );
      }
      let embedded = direction.is_some() && !consumed;
      assert_eq!(
        adjust(
          &mut items,
          LineJustificationContext {
            item_start: 0,
            y_pt: 20.0,
            left_pt: 119.05,
            right_pt: 476.3,
            terminal_overhang_pt: 0.0,
            source_frame_right_pt: None,
            preserve_foreign_tail: false,
          },
          Some(0),
          &mut metrics,
        ),
        Some(true)
      );
      let PageItem::Text(text) = &items[0] else {
        unreachable!()
      };
      let shape = metrics.shape_text(&text.text, &text.style).unwrap();
      assert_eq!(
        (shape.width_pt / PIXEL_PT).round() as i32,
        if embedded { 2977 } else { 3008 },
        "embedding direction {direction:?}"
      );
      assert_eq!(
        text
          .style
          .kashida_expansions
          .as_deref()
          .unwrap()
          .iter()
          .map(|join| join.glyph_count)
          .sum::<usize>(),
        if embedded { 16 } else { 24 },
        "embedding direction {direction:?}"
      );
    }
  }

  #[test]
  fn word_device_kashida_retains_local_width_across_table_translations() {
    let mut metrics = TextMetrics::new();
    // Native fixed-table controls keep these advances and extender counts
    // across nine independent page-margin translations. The 33.95pt italic
    // header needs 151 pixels; the 51.05pt body line needs 68 + 69 pixels.
    for (content, width, italic, expected, copies) in [
      ("السنة ", 33.95, true, vec![151], 29),
      ("المساواة بين ", 51.05, false, vec![68, 69], 26),
    ] {
      for origin in [7.2, 125.225, 501.175] {
        for delta in -4..=4 {
          let left = origin + delta as f32 / 20.0;
          let mut items = word_device_items(&[(left, content, 1)]);
          let PageItem::Text(text) = &mut items[0] else {
            unreachable!()
          };
          text.style.font_size_pt = 12.0;
          text.style.complex_font_size_pt = Some(12.0);
          text.style.italic = italic;
          text.style.complex_italic = Some(italic);
          let line = LineJustificationContext {
            item_start: 0,
            y_pt: 20.0,
            left_pt: left,
            right_pt: left + width,
            terminal_overhang_pt: 0.0,
            source_frame_right_pt: None,
            preserve_foreign_tail: false,
          };
          assert_eq!(adjust(&mut items, line, Some(0), &mut metrics), Some(true));
          let PageItem::Text(text) = &items[0] else {
            unreachable!()
          };
          let expansions = text.style.kashida_expansions.as_deref().unwrap();
          let advances = expansions
            .iter()
            .map(|expansion| (expansion.advance.0 / PIXEL_PT).round() as i32)
            .collect::<Vec<_>>();
          assert_eq!(advances, expected, "{content:?}, left={left}");
          assert_eq!(
            expansions
              .iter()
              .map(|expansion| expansion.glyph_count)
              .sum::<usize>(),
            copies,
            "{content:?}, left={left}"
          );
        }
      }
    }
  }

  #[test]
  fn word_device_kashida_weights_independent_foreign_blank() {
    let mut metrics = TextMetrics::new();
    // Native source238 GetJustifiedGlyphs inputs.13 seven-pixel connections
    // share the complete-selection budget with an ordinary foreign blank;
    // explicit cs retains a shaped blank, while rtl excludes its semantic tail.
    for (family, size, cs, rtl, expected, copies, blank_width) in [
      (
        "Times New Roman",
        10.0,
        false,
        false,
        [11, 12, 11, 12, 11, 12, 11, 12, 11, 12, 11, 12, 11],
        26,
        56,
      ),
      (
        "Traditional Arabic",
        15.0,
        false,
        false,
        [9, 10, 10, 10, 10, 10, 9, 10, 10, 10, 10, 10, 9],
        26,
        78,
      ),
      (
        "Times New Roman",
        10.0,
        true,
        false,
        [13, 13, 14, 13, 13, 14, 13, 14, 13, 13, 14, 13, 14],
        31,
        31,
      ),
      (
        "Times New Roman",
        10.0,
        false,
        true,
        [15, 16, 16, 16, 15, 16, 16, 16, 15, 16, 16, 16, 16],
        39,
        31,
      ),
    ] {
      let mut items = word_device_items(&[
        (446.74, "125", 2),
        (
          125.675,
          " - وثمة خمسة عشر فريق رصد ما انفكت تعمل على مسائل الأجر المتساوي مقابل",
          1,
        ),
        (123.1, " ", if rtl { 1 } else { 2 }),
      ]);
      for item in &mut items {
        let PageItem::Text(text) = item else {
          unreachable!()
        };
        text.style.font_size_pt = 10.0;
      }
      let PageItem::Text(blank) = &mut items[2] else {
        unreachable!()
      };
      blank.style.font_family = Some(Arc::from(family));
      blank.style.font_size_pt = size;
      blank.style.complex_script = Some(cs);
      blank.style.right_to_left = Some(rtl);
      let line = LineJustificationContext {
        item_start: 0,
        y_pt: 20.0,
        left_pt: 123.1,
        right_pt: 488.9,
        terminal_overhang_pt: 0.0,
        source_frame_right_pt: None,
        preserve_foreign_tail: false,
      };
      assert_eq!(adjust(&mut items, line, Some(2), &mut metrics), Some(true));
      let PageItem::Text(arabic) = &items[1] else {
        unreachable!()
      };
      let joins = arabic.style.kashida_expansions.as_deref().unwrap();
      let actual = joins
        .iter()
        .map(|join| (join.advance.0 / PIXEL_PT).round() as i32)
        .collect::<Vec<_>>();
      assert_eq!(
        actual, expected,
        "{family}, size={size}, cs={cs}, rtl={rtl}"
      );
      assert_eq!(
        joins.iter().map(|join| join.glyph_count).sum::<usize>(),
        copies
      );
      let PageItem::Text(blank) = &items[2] else {
        unreachable!()
      };
      assert_eq!(
        (metrics.measure_text(&blank.text, &blank.style) / PIXEL_PT).round() as i32,
        blank_width,
      );
    }
  }

  #[test]
  fn word_device_foreign_justification_retains_ltr_tail_and_space_carry() {
    let mut metrics = TextMetrics::new();
    // Exact native Word lowKashida XPS advances at10pt and103% width.
    // The final blank has no explicit XPS advance, but owns the last51 pixels
    // of the2772-pixel line. Splitting formatting portions preserves carry.
    let source = "General Economics Division (GED), Planning Commission, October 2005, ";
    let expected = [
      60, 37, 42, 37, 28, 37, 23, 50, 51, 37, 42, 42, 42, 65, 23, 37, 32, 51, 60, 23, 42, 23, 32,
      23, 42, 42, 50, 28, 60, 51, 60, 28, 21, 51, 46, 23, 37, 42, 42, 23, 42, 42, 51, 55, 42, 65,
      65, 23, 32, 32, 23, 42, 42, 21, 50, 60, 37, 23, 42, 42, 37, 28, 51, 42, 42, 42, 42, 21, 51,
    ];
    for split in [0, 8, 18, 34, 43, 55, 63] {
      let portions = if split == 0 {
        vec![(123.1, source, 2)]
      } else {
        vec![(123.1, &source[..split], 2), (200.0, &source[split..], 2)]
      };
      let mut items = word_device_items(&portions);
      for item in &mut items {
        let PageItem::Text(text) = item else {
          unreachable!()
        };
        text.style.font_size_pt = 10.0;
        text.style.right_to_left = Some(false);
      }
      let line = LineJustificationContext {
        item_start: 0,
        y_pt: 20.0,
        left_pt: 123.1,
        right_pt: 455.75,
        terminal_overhang_pt: 0.0,
        source_frame_right_pt: None,
        preserve_foreign_tail: true,
      };
      let last = items.len() - 1;
      assert_eq!(
        adjust(&mut items, line, Some(last), &mut metrics),
        Some(true)
      );
      let mut actual = Vec::new();
      for item in &items {
        let PageItem::Text(text) = item else {
          unreachable!()
        };
        assert!(text.style.kashida_expansions.is_none());
        assert_eq!(text.word_spacing_pt, 0.0);
        actual.extend(
          metrics
            .shape_text(&text.text, &text.style)
            .unwrap()
            .glyphs
            .iter()
            .map(|glyph| (glyph.x_advance_em * glyph.font_size_pt / PIXEL_PT).round() as i32),
        );
      }
      assert_eq!(actual, expected, "split={split}");
      let (left, right) = super::super::line_horizontal_bounds_for_alignment(
        &items,
        line.y_pt,
        &mut metrics,
        false,
        true,
        false,
      )
      .unwrap();
      assert!((left - line.left_pt).abs() < 0.0001, "split={split}");
      assert!((right - (line.left_pt + 2772.0 * PIXEL_PT)).abs() < 0.0001);
    }
  }

  #[test]
  fn word_device_wrapped_foreign_line_leaves_its_tail_outside_the_visible_span() {
    let mut metrics = TextMetrics::new();
    let content = "Status of CEDAW in Bangladesh: Keynote Presentation by Ferdous Ara Begum, ";
    // Native XPS explicitly advances the 73 preceding glyphs by 2772 device
    // pixels. The omitted last blank retains its natural 21-pixel advance.
    for separate_tail in [false, true] {
      let portions = if separate_tail {
        vec![(123.1, content.trim_end(), 2), (453.1, " ", 2)]
      } else {
        vec![(123.1, content, 2)]
      };
      let mut items = word_device_items(&portions);
      for item in &mut items {
        let PageItem::Text(text) = item else {
          unreachable!()
        };
        text.style.font_size_pt = 10.0;
        text.style.right_to_left = None;
      }
      let line = LineJustificationContext {
        item_start: 0,
        y_pt: 20.0,
        left_pt: 123.1,
        right_pt: 455.75,
        terminal_overhang_pt: 0.0,
        source_frame_right_pt: None,
        preserve_foreign_tail: false,
      };
      let last = items.len() - 1;
      assert_eq!(
        adjust(&mut items, line, Some(last), &mut metrics),
        Some(true)
      );
      let mut full_pixels = 0;
      for item in &items {
        let PageItem::Text(text) = item else {
          unreachable!()
        };
        let device = text.style.wordprocessing_kashida.as_ref().unwrap();
        assert!(!device.retain_trailing_blank);
        assert!(
          device
            .space_expansions
            .iter()
            .all(|&(byte, _)| byte + 1 < text.text.len())
        );
        full_pixels += (metrics
          .shape_text(&text.text, &text.style)
          .unwrap()
          .width_pt
          / PIXEL_PT)
          .round() as i32;
      }
      assert_eq!(full_pixels, 2772 + 21);
      let (left, right) = super::super::line_horizontal_bounds_for_alignment(
        &items,
        line.y_pt,
        &mut metrics,
        false,
        true,
        false,
      )
      .unwrap();
      assert!((left - line.left_pt).abs() < 0.0001);
      assert!((right - (line.left_pt + 2772.0 * PIXEL_PT)).abs() < 0.0001);
    }
  }

  #[test]
  fn word_device_kashida_carries_join_remainders_across_bidi_portions() {
    let mut metrics = TextMetrics::new();
    // Actual native GetJustifiedGlyphs input arrays for two complete Word
    // lines. Numeric portions divide the runs without resetting the carry.
    let cases = [
      (
        [
          (258.17145, "ويشمل الهدف الإنمائي العام لتمكين المرأة: ’", 1),
          (250.44644, "1", 2),
          (123.1, "‘ النهوض بحقوق المرأة وحمايتها؛ ", 1),
        ],
        455.75,
        vec![vec![26, 26, 26, 26, 27, 26], vec![], vec![26, 26, 26, 27]],
      ),
      (
        [
          (
            233.39229,
            "العاملات المشتركات في القطاعات الموجهة نحو التصدير؛ ’",
            1,
          ),
          (220.0, "18", 2),
          (123.1, "‘ إدماج شواغل المساواة ", 1),
        ],
        488.9,
        vec![vec![33, 33, 34, 33, 34, 33], vec![], vec![34, 33, 34]],
      ),
    ];
    for (portions, right_pt, expected) in cases {
      let mut items = word_device_items(&portions);
      let line = LineJustificationContext {
        item_start: 0,
        y_pt: 20.0,
        left_pt: 123.1,
        right_pt,
        terminal_overhang_pt: 0.0,
        source_frame_right_pt: None,
        preserve_foreign_tail: false,
      };
      assert_eq!(adjust(&mut items, line, Some(2), &mut metrics), Some(true));
      let actual = items
        .iter()
        .map(|item| {
          let PageItem::Text(text) = item else {
            unreachable!()
          };
          text
            .style
            .kashida_expansions
            .as_deref()
            .unwrap_or_default()
            .iter()
            .map(|expansion| (expansion.advance.0 / PIXEL_PT).round() as i32)
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
      assert_eq!(actual, expected, "{}", portions[0].1);
    }
  }

  #[test]
  fn word_device_kashida_weights_connections_by_device_minimum() {
    let mut metrics = TextMetrics::new();
    // Native 150pt soft-break controls. A 12pt bold connection and a 15pt
    // regular connection both need seven device pixels despite different
    // nominal extender widths; 15pt bold instead needs nine pixels.
    for (first_size, first_bold, expected, expected_copies) in [
      (12.0, false, [46, 47, 66, 66], 38),
      (12.0, true, [48, 48, 48, 48], 30),
      (15.0, false, [27, 27, 27, 28], 20),
      (15.0, true, [19, 19, 15, 16], 12),
    ] {
      let mut items = word_device_items(&[
        (210.0, "السياسات التعليم ", 1),
        (123.1, "السياسات التعليم ", 1),
      ]);
      for (index, item) in items.iter_mut().enumerate() {
        let PageItem::Text(text) = item else {
          unreachable!()
        };
        let size = if index == 0 { first_size } else { 15.0 };
        text.style.font_size_pt = size;
        text.style.complex_font_size_pt = Some(size);
        text.style.bold = index == 0 && first_bold;
        text.style.complex_bold = Some(text.style.bold);
      }
      let line = LineJustificationContext {
        item_start: 0,
        y_pt: 20.0,
        left_pt: 123.1,
        right_pt: 273.1,
        terminal_overhang_pt: 0.0,
        source_frame_right_pt: None,
        preserve_foreign_tail: false,
      };
      assert_eq!(adjust(&mut items, line, Some(1), &mut metrics), Some(true));
      let mut actual = Vec::new();
      let mut copies = 0;
      for item in &items {
        let PageItem::Text(text) = item else {
          unreachable!()
        };
        for expansion in text.style.kashida_expansions.as_deref().unwrap_or_default() {
          actual.push((expansion.advance.0 / PIXEL_PT).round() as i32);
          copies += expansion.glyph_count;
        }
      }
      assert_eq!(actual, expected, "size={first_size}, bold={first_bold}");
      assert_eq!(copies, expected_copies);
    }
  }

  #[test]
  fn word_device_kashida_distinguishes_partial_and_complete_minimum_ties() {
    let mut metrics = TextMetrics::new();
    // Native two-word controls have natural width555 device pixels and two
    // seven-pixel connections. A partial selection reserves positive space
    // slack; the complete selection accepts its exact fourteen-pixel minimum.
    for (extra, expected, space_extra) in [
      (7, vec![], 7),
      (13, vec![7], 6),
      (14, vec![7, 7], 0),
      (15, vec![7, 8], 0),
    ] {
      let mut items = word_device_items(&[(123.1, "السياسات التعليم", 1)]);
      let line = LineJustificationContext {
        item_start: 0,
        y_pt: 20.0,
        left_pt: 123.1,
        right_pt: 123.1 + (555 + extra) as f32 * PIXEL_PT,
        terminal_overhang_pt: 0.0,
        source_frame_right_pt: None,
        preserve_foreign_tail: false,
      };
      assert_eq!(adjust(&mut items, line, Some(0), &mut metrics), Some(true));
      let PageItem::Text(text) = &items[0] else {
        unreachable!()
      };
      let expansions = text.style.kashida_expansions.as_deref().unwrap_or_default();
      let actual = expansions
        .iter()
        .map(|expansion| (expansion.advance.0 / PIXEL_PT).round() as i32)
        .collect::<Vec<_>>();
      assert_eq!(actual, expected, "extra={extra}");
      if extra == 13 {
        assert_eq!(expansions[0].byte_index, 27);
      }
      let device = text.style.wordprocessing_kashida.as_ref().unwrap();
      assert_eq!(
        device
          .space_expansions
          .iter()
          .map(|(_, pixels)| pixels)
          .sum::<i32>(),
        space_extra
      );
    }

    // The unspaced single-word counterexample accepts its complete minimum.
    let mut items = word_device_items(&[(123.1, "السياسات", 1)]);
    let line = LineJustificationContext {
      item_start: 0,
      y_pt: 20.0,
      left_pt: 123.1,
      right_pt: 123.1 + 337.0 * PIXEL_PT,
      terminal_overhang_pt: 0.0,
      source_frame_right_pt: None,
      preserve_foreign_tail: false,
    };
    assert_eq!(adjust(&mut items, line, Some(0), &mut metrics), Some(true));
    let PageItem::Text(text) = &items[0] else {
      unreachable!()
    };
    let expansions = text.style.kashida_expansions.as_deref().unwrap();
    assert_eq!(expansions.len(), 1);
    assert_eq!((expansions[0].advance.0 / PIXEL_PT).round() as i32, 7);
    assert!(
      text
        .style
        .wordprocessing_kashida
        .as_ref()
        .unwrap()
        .space_expansions
        .is_empty()
    );
  }

  #[test]
  fn word_device_kashida_retains_blanks_between_bidi_portions() {
    let mut metrics = TextMetrics::new();
    // Native completed line: natural2740 printer pixels, target3079 including
    // the semantic tail. Ten connections receive33/34 pixels, yielding59
    // nominal extenders. Internal blanks retain their separate bidi portions.
    let mut items = word_device_items(&[
      (434.529, "قرابة ", 1),
      (419.079, "98", 2),
      (
        142.322,
        " موظف تنسيق وموظف تنسيق معاوناً لإدماج المرأة في عملية التنمية في ",
        1,
      ),
      (126.872, "49", 2),
      (123.1, " ", 1),
    ]);
    let line = LineJustificationContext {
      item_start: 0,
      y_pt: 20.0,
      left_pt: 123.1,
      right_pt: 488.9,
      terminal_overhang_pt: 0.0,
      source_frame_right_pt: None,
      preserve_foreign_tail: false,
    };
    let mut leading = items.clone();
    let PageItem::Text(first) = &mut leading[0] else {
      unreachable!()
    };
    first.text.insert(0, ' ');
    assert_eq!(adjust(&mut leading, line, Some(4), &mut metrics), None);

    // Note stories preserve text portions to retain source boundaries.
    // Their internal blanks still participate in device justification; that
    // boundary flag does not turn an internal blank into a line-edge blank.
    for item in &mut items {
      let PageItem::Text(text) = item else {
        unreachable!()
      };
      text.preserve_text_portion = true;
    }
    assert_eq!(adjust(&mut items, line, Some(4), &mut metrics), Some(true));
    let mut advances = Vec::new();
    let mut copies = 0;
    for item in &items {
      let PageItem::Text(text) = item else {
        unreachable!()
      };
      assert_eq!(text.word_spacing_pt, 0.0);
      assert!(text.style.wordprocessing_kashida.is_some());
      if let Some(expansions) = &text.style.kashida_expansions {
        for expansion in expansions.iter() {
          advances.push((expansion.advance.0 / PIXEL_PT).round() as i32);
          copies += expansion.glyph_count;
        }
      }
    }
    assert_eq!(advances.len(), 10);
    assert_eq!(advances.iter().sum::<i32>(), 339);
    assert_eq!(copies, 59);
  }
}
