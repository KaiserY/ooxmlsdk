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
  if extra <= LAYOUT_EPSILON_PT {
    return false;
  }
  let mut portions = Vec::new();
  let mut size = None;
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
    // Mixed ems and complex clusters retain their existing line owner until
    // its expansion priorities are established independently.
    if size.is_some_and(|size| size != text.style.font_size_pt) {
      return false;
    }
    size = Some(text.style.font_size_pt);
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
  // Existing singleton grid portions have their own column-pitch algorithm.
  if portions.iter().all(|p| p.text.len() == 1) {
    return false;
  }
  let Some(size) = size else { return false };
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
        // Native Word expands an existing automatic script gap to half an
        // em before spreading the remaining glue between CJK characters.
        capacity = ((size * 0.5 - gap).max(0.0) * 4096.0).round() as i64;
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
  let mut accumulated = 0i64;
  let mut allocated = 0i64;
  for (index, &(p, c, cap)) in slots.iter().enumerate() {
    accumulated += cap;
    let target = if capacity == 0 {
      0
    } else {
      priority * accumulated / capacity
    };
    let units =
      target - allocated + remainder / count + i64::from((index as i64) < remainder % count);
    portions[p].expansion[c] += units as f32 / 4096.0;
    allocated = target;
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
