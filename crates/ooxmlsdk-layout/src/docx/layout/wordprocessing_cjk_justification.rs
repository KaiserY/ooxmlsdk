//! Character glue for ordinary, ungridded Word CJK lines.
use super::*;

pub(super) struct SpaceContraction {
  start: usize,
  y: f32,
  slots: Vec<(usize, usize, i64)>,
  capacity: i64,
}

impl SpaceContraction {
  pub(super) fn prepare(
    items: &[PageItem],
    start: usize,
    y: f32,
    compatibility_mode: u16,
    content_end: Option<CjkLineEnd>,
    metrics: &mut TextMetrics,
  ) -> Option<Self> {
    let mut slots = Vec::new();
    let mut capacity = 0;
    let mut seen_content = false;
    for (index, item) in items.iter().enumerate().skip(start) {
      if inline_justification_horizontal_bounds(item).is_some() {
        seen_content = true;
        continue;
      }
      if item_y(item).is_none_or(|item_y| (item_y - y).abs() >= 0.01) {
        continue;
      }
      let PageItem::Text(text) = item else {
        return None;
      };
      let visible = content_end.map_or(text.text.as_str(), |end| {
        end.visible_text(index, &text.text)
      });
      if visible.is_empty() {
        continue;
      }
      if text.paragraph_bidi
        || text.style.right_to_left == Some(true)
        || text.style.semantic_only
        || text.style.rotation_deg != 0.0
        || text.style.semantic_character_advances_pt.is_some()
        || text
          .style
          .wordprocessing_justification_expansion_pt
          .is_some()
        || text.word_spacing_pt != 0.0
      {
        return None;
      }
      let legacy = compatibility_mode < 15;
      let ordinary = !text.style.wordprocessingml_balanced_space_portion;
      let has_space = ordinary && visible.contains(' ');
      let has_nbsp = legacy && visible.contains('\u{a0}');
      let space_capacity = if has_space || has_nbsp {
        if legacy {
          // CJK width balancing replaces the raw OS/2 average with Word's
          // logical reference width. Reuse its cached half-average, also
          // used by the automatic script-gap and source-space owners.
          if let Some((_, minimum)) = metrics.wordprocessing_auto_script_spacing(' ', &text.style) {
            (metrics.measure_text(" ", &text.style) - minimum).max(0.0)
          } else {
            metrics.legacy_word_space_contraction_capacity(&text.style)?
          }
        } else {
          metrics.measure_text(" ", &text.style) * 0.25
        }
      } else {
        0.0
      };
      let nbsp_capacity = if has_nbsp {
        (metrics.measure_text("\u{a0}", &text.style) - metrics.measure_text(" ", &text.style)
          + space_capacity)
          .max(0.0)
      } else {
        0.0
      };
      for (position, character) in visible.chars().enumerate() {
        if !character.is_whitespace() {
          seen_content = true;
        }
        let removable = match character {
          ' ' if seen_content && ordinary => space_capacity,
          '\u{a0}' if seen_content && legacy => nbsp_capacity,
          _ => continue,
        };
        let units = (removable * 4096.0).round() as i64;
        if units > 0 {
          slots.push((index, position, units));
          capacity += units;
        }
      }
    }
    Some(Self {
      start,
      y,
      slots,
      capacity,
    })
  }

  pub(super) fn capacity_pt(&self) -> f32 {
    self.capacity as f32 / 4096.0
  }

  pub(super) fn apply(self, items: &mut [PageItem], loss: f32) -> f32 {
    let loss = ((loss * 4096.0).round() as i64).clamp(0, self.capacity);
    if loss == 0 {
      return 0.0;
    }
    // Native Word first contracts ordinary blanks to their mode-specific
    // minimum, then spends internal punctuation capacity. Source-balanced
    // CJK blanks retain their own width; modern NBSP is not ordinary glue.
    let (level, mut residual) =
      common_contraction(self.slots.iter().map(|slot| slot.2), self.capacity - loss);
    let mut eligible = self.slots.iter().filter(|slot| slot.2 > level).count() as i64;
    let mut slots = self.slots.into_iter().peekable();
    let mut preceding = 0_i64;
    for (index, item) in items.iter_mut().enumerate().skip(self.start) {
      let PageItem::Text(text) = item else {
        if preceding != 0 && inline_justification_horizontal_bounds(item).is_some() {
          shift_item_x(item, -(preceding as f32 / 4096.0));
        }
        continue;
      };
      if (text.y_pt - self.y).abs() >= 0.01 {
        continue;
      }
      text.x_pt -= preceding as f32 / 4096.0;
      if slots.peek().is_none_or(|slot| slot.0 != index) {
        continue;
      }
      let mut adjustments = vec![0.0; text.text.chars().count()];
      while slots.peek().is_some_and(|slot| slot.0 == index) {
        let (_, position, cap) = slots.next().expect("space slot");
        let extra = i64::from(cap > level && eligible <= residual);
        eligible -= i64::from(cap > level);
        residual -= extra;
        let contraction = cap - cap.min(level) - extra;
        adjustments[position] = -(contraction as f32 / 4096.0);
        preceding += contraction;
      }
      text.style.wordprocessing_justification_expansion_pt = Some(adjustments.into());
    }
    loss as f32 / 4096.0
  }
}

struct Portion {
  index: usize,
  text: Vec<char>,
  x: f32,
  width: f32,
  expansion: Vec<f32>,
  glyphs: Vec<crate::text_metrics::ShapedGlyph>,
  maximum_space: f32,
}

pub(super) fn adjust_legacy_spaced(
  items: &mut [PageItem],
  line: LineJustificationContext,
  natural_right: f32,
  has_inline_frame: bool,
  text_metrics: &mut TextMetrics,
) -> bool {
  // Legacy ideographic lines retain character glue around SP/NBSP. Native
  // modern, Hangul and Western-word controls have separate spacing owners.
  // Reject those contexts before shaping or allocating.
  let mut cjk = false;
  let mut space = false;
  for item in items.iter().skip(line.item_start) {
    let PageItem::Text(text) = item else {
      if inline_justification_horizontal_bounds(item).is_some() {
        continue;
      }
      return false;
    };
    if (text.y_pt - line.y_pt).abs() >= 0.01 {
      continue;
    }
    if !text.style.wordprocessingml_font_slots {
      return false;
    }
    for character in text.text.chars() {
      if character.script() == Script::Hangul
        || (!has_inline_frame && character.is_alphanumeric() && !cjk_line_character(character))
      {
        return false;
      }
      cjk |= cjk_line_character(character) && !character.is_whitespace();
      space |= matches!(character, ' ' | '\u{a0}');
    }
  }
  cjk && space && adjust_impl(items, line, natural_right, None, true, text_metrics)
}

pub(super) fn adjust_hanging(
  items: &mut [PageItem],
  line: LineJustificationContext,
  mut policy: WordprocessingOverflowPunctuationPolicy,
  contract_auto_gaps: bool,
  text_metrics: &mut TextMetrics,
) -> bool {
  if !policy.enabled
    || line.has_character_grid
    || line
      .source_frame_right_pt
      .is_some_and(|right| line.right_pt < right - LAYOUT_EPSILON_PT)
  {
    return false;
  }
  // Most wrapped lines end in an ordinary letter. Reject them before any
  // font measurement; only a possible hanging CJK terminal needs a scan.
  let terminal = items.iter().skip(line.item_start).rev().find_map(|item| {
    let PageItem::Text(text) = item else {
      return None;
    };
    ((text.y_pt - line.y_pt).abs() < 0.01
      && !text
        .text
        .trim_end_matches(libreoffice_line_end_elidable_blank)
        .is_empty())
    .then_some(text)
  });
  if terminal.is_none_or(|text| {
    cjk_punctuation_compression_enabled(&text.text, &text.style)
      || text.style.wordprocessing_terminal_punctuation.is_some()
      || !text
        .text
        .trim_end_matches(libreoffice_line_end_elidable_blank)
        .chars()
        .next_back()
        .is_some_and(cjk_line_character)
  }) {
    return false;
  }
  let mut last = None;
  let mut natural_right = line.left_pt;
  for (index, item) in items.iter().enumerate().skip(line.item_start) {
    if item_y(item).is_none_or(|y| (y - line.y_pt).abs() >= 0.01) {
      continue;
    }
    let PageItem::Text(text) = item else {
      return false;
    };
    // Active punctuation compression owns a different terminal bearing.
    // Native en/zh and on/off opposites distinguish this from full-cell
    // hanging, whose preceding letters expand even in a left-aligned line.
    if cjk_punctuation_compression_enabled(&text.text, &text.style) || text.word_spacing_pt != 0.0 {
      return false;
    }
    let visible = text
      .text
      .trim_end_matches(libreoffice_line_end_elidable_blank);
    if visible.is_empty() {
      continue;
    }
    let width = text_metrics.measure_text(visible, &text.style);
    natural_right = natural_right.max(text.x_pt + width);
    last = Some((index, visible.len(), width, policy.preceding_language));
    if let Some(language) = wordprocessing_text_language(visible, &text.style) {
      policy.preceding_language = language;
    }
  }
  let Some((index, end, width, preceding_language)) = last else {
    return false;
  };
  let PageItem::Text(text) = &items[index] else {
    return false;
  };
  let visible = &text.text[..end];
  if !visible.chars().next_back().is_some_and(cjk_line_character) {
    return false;
  }
  policy.preceding_language = preceding_language;
  let fit = wordprocessing_line_fit_text("", visible, &text.style, policy, text_metrics);
  if fit.len() == visible.len() {
    return false;
  }
  let overhang = width - text_metrics.measure_text(fit, &text.style);
  let fitted_right = natural_right - overhang;
  // Full-cell hanging is the next adjustment level. Native Word first fits
  // the complete terminal by changing existing automatic script gaps within
  // their eighth/half-cell interval, without spreading glue across CJK text.
  if items.iter().skip(line.item_start).any(|item| {
    matches!(item, PageItem::Text(text) if (text.y_pt - line.y_pt).abs() < 0.01
      && text.text.chars().any(|ch| !cjk_line_character(ch) && ch.is_alphanumeric()))
  }) && adjust_impl(
    items,
    line,
    natural_right,
    Some(contract_auto_gaps),
    false,
    text_metrics,
  ) {
    return true;
  }
  if !text_overflows_line(natural_right, 0.0, line.right_pt)
    || text_overflows_line(fitted_right, 0.0, line.right_pt)
  {
    return false;
  }
  // Native MSLS: a37-cell body plus hanging comma in415.3pt receives
  // 31*919 +6*918 ideal units, then retains the comma's natural advance.
  // The existing CJK glue allocator owns this exact1/4096pt distribution.
  adjust(items, line, fitted_right, text_metrics)
}

pub(super) fn adjust(
  items: &mut [PageItem],
  line: LineJustificationContext,
  natural_right: f32,
  text_metrics: &mut TextMetrics,
) -> bool {
  adjust_impl(items, line, natural_right, None, false, text_metrics)
}

pub(super) fn adjust_full_terminal_hanging(
  items: &mut [PageItem],
  line: LineJustificationContext,
  metrics: &mut TextMetrics,
) -> bool {
  let mut right = line.left_pt;
  for item in items.iter().skip(line.item_start) {
    if item_y(item).is_none_or(|y| (y - line.y_pt).abs() >= 0.01) {
      continue;
    }
    let PageItem::Text(text) = item else {
      return false;
    };
    right = right.max(text.x_pt + metrics.measure_text(&text.text, &text.style));
  }
  // The compression finalizer has already selected and preserved the
  // terminal's advance. Fill its printable prefix without changing that
  // half-cell or spending an ordinary word's punctuation pull-in budget.
  adjust(items, line, right - line.terminal_overhang_pt, metrics)
}

fn adjust_impl(
  items: &mut [PageItem],
  line: LineJustificationContext,
  natural_right: f32,
  automatic_gaps_only: Option<bool>,
  include_spaces: bool,
  text_metrics: &mut TextMetrics,
) -> bool {
  let extra = line.right_pt - natural_right;
  if line.has_character_grid
    || if automatic_gaps_only == Some(true) {
      extra.abs() <= LAYOUT_EPSILON_PT
    } else {
      extra <= LAYOUT_EPSILON_PT
    }
  {
    return false;
  }
  let mut portions = Vec::new();
  for (index, item) in items.iter().enumerate().skip(line.item_start) {
    // The line owns the complete as-character frame. Its vertical offset
    // positions its ink independently of its place among the text portions.
    if let Some((left, right)) = inline_justification_horizontal_bounds(item) {
      portions.push(Portion {
        index,
        text: vec!['\u{fffc}'],
        x: left,
        width: right - left,
        expansion: vec![0.0],
        glyphs: Vec::new(),
        maximum_space: 0.0,
      });
      continue;
    }
    if item_y(item).is_none_or(|y| (y - line.y_pt).abs() >= 0.01) {
      continue;
    }
    let PageItem::Text(text) = item else {
      return false;
    };
    if text.text.is_empty() {
      continue;
    }
    if text.paragraph_bidi
      || text.style.right_to_left == Some(true)
      || text.style.semantic_only
      || text.style.rotation_deg != 0.0
      || text.style.horizontal_scale.unwrap_or(1.0) != 1.0
      || text.style.semantic_character_advances_pt.is_some()
      || text
        .style
        .wordprocessing_justification_expansion_pt
        .is_some()
      || text.text.chars().any(char::is_control)
    {
      return false;
    }
    let Some(shaped) = text_metrics.shape_text(&text.text, &text.style) else {
      return false;
    };
    let chars: Vec<_> = text.text.chars().collect();
    if shaped.glyphs.len() != chars.len()
      || shaped
        .glyphs
        .iter()
        .zip(text.text.char_indices())
        .any(|(glyph, (start, ch))| glyph.text_range != (start..start + ch.len_utf8()))
    {
      return false;
    }
    let maximum_space = if include_spaces && chars.contains(&' ') {
      // Native blanks reach the reference average before the remaining
      // character glue. Reuse twice the automatic-gap optimum, including
      // the independent half-width/full-width balance switch.
      let Some((_, gap)) = text_metrics.wordprocessing_auto_script_spacing(' ', &text.style) else {
        return false;
      };
      gap * 2.0
    } else {
      0.0
    };
    portions.push(Portion {
      index,
      x: text.x_pt,
      width: text_metrics.measure_text(&text.text, &text.style),
      expansion: vec![0.0; chars.len()],
      // Retain the already allocated glyph vector only for source-space
      // capacities; an isolated trial-space measurement loses its context.
      glyphs: if include_spaces {
        shaped.glyphs
      } else {
        Vec::new()
      },
      maximum_space,
      text: chars,
    });
  }
  // A source grid has a separate column-pitch owner. Ordinary singleton
  // formatting runs and mixed script ems still receive Word's character glue.
  if !portions
    .iter()
    .any(|p| p.text.iter().any(|&ch| cjk_line_character(ch)))
  {
    return false;
  }
  let positions: Vec<_> = portions
    .iter()
    .enumerate()
    .flat_map(|(p, portion)| {
      portion
        .text
        .iter()
        .enumerate()
        .map(move |(c, &ch)| (p, c, ch, false))
    })
    .collect();
  let mut positions = positions;
  if include_spaces {
    let start = positions
      .iter()
      .position(|position| !position.2.is_whitespace());
    let end = positions
      .iter()
      .rposition(|position| !position.2.is_whitespace());
    if let (Some(start), Some(end)) = (start, end) {
      for position in &mut positions[start..end] {
        position.3 = matches!(position.2, ' ' | '\u{a0}');
      }
    }
  }
  let mut slots = Vec::new();
  for pair in positions.windows(2) {
    let (p, c, left, left_space) = pair[0];
    let (next_p, _, right, right_space) = pair[1];
    if (left.is_whitespace() && !left_space)
      || (right.is_whitespace() && !right_space)
      || !(cjk_line_character(left)
        || cjk_line_character(right)
        || left == '\u{fffc}'
        || right == '\u{fffc}'
        || left_space
        || right_space)
    {
      continue;
    }
    let mut capacity = if left_space && left == ' ' {
      let portion = &portions[p];
      let glyph = &portion.glyphs[c];
      ((portion.maximum_space - glyph.x_advance_em * glyph.font_size_pt).max(0.0) * 4096.0).round()
        as i64
    } else {
      0
    };
    if p != next_p {
      let gap = portions[next_p].x - (portions[p].x + portions[p].width);
      let mixed = matches!(
        (
          auto_script_class(left, false),
          auto_script_class(right, false)
        ),
        (
          AutoScriptClass::EastAsian,
          AutoScriptClass::Letter | AutoScriptClass::Number
        ) | (
          AutoScriptClass::Letter | AutoScriptClass::Number,
          AutoScriptClass::EastAsian
        )
      );
      if mixed && gap > LAYOUT_EPSILON_PT {
        let PageItem::Text(text) = &items[portions[p].index] else {
          unreachable!()
        };
        if let Some(edge) = auto_script_edge(left, &text.style, false, text_metrics)
          && gap <= edge.normal_gap_pt + LAYOUT_EPSILON_PT
          && gap >= edge.minimum_gap_pt - LAYOUT_EPSILON_PT
        {
          // Native expands automatic gaps to the preceding logical average
          // width (twice their nominal width), independently of the next em.
          capacity = (if extra < 0.0 {
            gap - edge.minimum_gap_pt
          } else {
            edge.maximum_gap_pt - gap
          }
          .max(0.0)
            * 4096.0)
            .round() as i64;
        }
      }
    }
    slots.push((p, c, capacity));
  }
  if slots.is_empty() {
    return false;
  }
  let total = (extra.abs() * 4096.0).round() as i64;
  let capacity: i64 = slots.iter().map(|slot| slot.2).sum();
  if automatic_gaps_only.is_some() && (capacity == 0 || total > capacity) {
    return false;
  }
  let priority = total.min(capacity);
  let remainder = total - priority;
  let count = slots.len() as i64;
  let (contraction, mut residual_contraction) =
    common_contraction(slots.iter().map(|slot| slot.2), capacity - priority);
  for (index, &(p, c, cap)) in slots.iter().enumerate() {
    let residual = i64::from(cap > contraction && residual_contraction > 0);
    residual_contraction -= residual;
    let units = cap - cap.min(contraction) - residual
      + remainder / count
      + i64::from((index as i64) < remainder % count);
    portions[p].expansion[c] += units as f32 / 4096.0 * extra.signum();
  }
  let mut preceding = 0.0;
  for portion in portions {
    let PageItem::Text(text) = &mut items[portion.index] else {
      shift_item_x(&mut items[portion.index], preceding);
      preceding += portion.expansion[0];
      continue;
    };
    text.x_pt += preceding;
    preceding += portion.expansion.iter().sum::<f32>();
    text.style.wordprocessing_justification_expansion_pt = Some(portion.expansion.into());
  }
  true
}

fn common_contraction(capacities: impl Iterator<Item = i64> + Clone, loss: i64) -> (i64, i64) {
  // Native heterogeneous-gap arrays subtract one common amount from their
  // maximum capacities, saturating smaller capacities at zero. They do not
  // allocate a proportional share of the available expansion. Find that
  // integer water level without per-gap allocation or quadratic retries.
  let mut low = 0;
  let mut high = capacities.clone().max().unwrap_or(0);
  while low < high {
    let middle = low + (high - low + 1) / 2;
    let used: i64 = capacities.clone().map(|cap| cap.min(middle)).sum();
    if used <= loss {
      low = middle;
    } else {
      high = middle - 1;
    }
  }
  let used: i64 = capacities.map(|cap| cap.min(low)).sum();
  (low, loss - used)
}

#[cfg(test)]
mod tests {
  use super::common_contraction;

  #[test]
  fn heterogeneous_native_gaps_contract_from_their_maximum_capacity() {
    let capacities = [10752, 8620, 8620, 10752, 8620, 8620];
    let loss = capacities.iter().sum::<i64>() - 28189;
    let (level, mut remainder) = common_contraction(capacities.into_iter(), loss);
    let actual = capacities.map(|cap| {
      let extra = i64::from(cap > level && remainder > 0);
      remainder -= extra;
      cap - cap.min(level) - extra
    });
    assert_eq!(actual, [6119, 3987, 3987, 6120, 3988, 3988]);
    assert_eq!(actual.iter().sum::<i64>(), 28189);
    for keep in [0, 1, 4000, 55984] {
      let loss = capacities.iter().sum::<i64>() - keep;
      let (level, remainder) = common_contraction(capacities.into_iter(), loss);
      assert_eq!(
        capacities
          .into_iter()
          .map(|cap| cap.min(level))
          .sum::<i64>()
          + remainder,
        loss
      );
    }
  }
}
