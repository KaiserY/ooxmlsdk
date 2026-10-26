//! Character glue for ordinary, ungridded Word CJK lines.
use super::*;

struct Portion {
  index: usize,
  text: Vec<char>,
  x: f32,
  width: f32,
  expansion: Vec<f32>,
}

pub(super) fn adjust(
  items: &mut [PageItem],
  line: LineJustificationContext,
  natural_right: f32,
  text_metrics: &mut TextMetrics,
) -> bool {
  let extra = line.right_pt - natural_right;
  if extra <= LAYOUT_EPSILON_PT || line.has_character_grid {
    return false;
  }
  let mut portions = Vec::new();
  for (index, item) in items.iter().enumerate().skip(line.item_start) {
    let PageItem::Text(text) = item else {
      return false;
    };
    if (text.y_pt - line.y_pt).abs() >= 0.01 {
      continue;
    }
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
    portions.push(Portion {
      index,
      x: text.x_pt,
      width: text_metrics.measure_text(&text.text, &text.style),
      expansion: vec![0.0; chars.len()],
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
        .map(move |(c, &ch)| (p, c, ch))
    })
    .collect();
  let mut slots = Vec::new();
  for pair in positions.windows(2) {
    let (p, c, left) = pair[0];
    let (next_p, _, right) = pair[1];
    if left.is_whitespace()
      || right.is_whitespace()
      || !(cjk_line_character(left) || cjk_line_character(right))
    {
      continue;
    }
    let mut capacity = 0i64;
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
          && gap >= edge.normal_gap_pt * 0.5 - LAYOUT_EPSILON_PT
        {
          // Native expands automatic gaps to the preceding logical average
          // width (twice their nominal width), independently of the next em.
          capacity = ((edge.normal_gap_pt * 2.0 - gap).max(0.0) * 4096.0).round() as i64;
        }
      }
    }
    slots.push((p, c, capacity));
  }
  if slots.is_empty() {
    return false;
  }
  let total = (extra * 4096.0).round() as i64;
  let capacity: i64 = slots.iter().map(|slot| slot.2).sum();
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
    portions[p].expansion[c] += units as f32 / 4096.0;
  }
  let mut preceding = 0.0;
  for portion in portions {
    let PageItem::Text(text) = &mut items[portion.index] else {
      unreachable!()
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
