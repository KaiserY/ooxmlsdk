use super::{
  CrossRunContinuation, InlineCursor, InlineItem, LineShrinkFit, LineSpaceCounter, PageItem,
  TextMetrics, TextSegment, TextStyle,
};

/// Native Word modes 12/14 admit one additional complete token at the selected
/// font's minimum spaces. Modern cost comparison remains a separate policy.
pub(super) fn fits(fit: LineShrinkFit<'_>, metrics: &mut TextMetrics) -> bool {
  fits_with_tail(fit, std::iter::empty(), metrics)
}

fn fits_with_tail<'a>(
  fit: LineShrinkFit<'_>,
  tail: impl Iterator<Item = (&'a str, &'a TextStyle)>,
  metrics: &mut TextMetrics,
) -> bool {
  let overflow = fit.x + fit.width - fit.line_right;
  if overflow <= 0.0 {
    return true;
  }
  let mut counter = LineSpaceCounter::default();
  let mut capacity = 0.0;
  let mut trailing = 0.0;
  let mut push = |text: &str, style: &TextStyle| {
    counter.push_text(text);
    let spaces = text.bytes().filter(|byte| *byte == b' ').count();
    let per_space = if spaces == 0 {
      0.0
    } else {
      metrics
        .legacy_word_space_contraction_capacity(style)
        .unwrap_or_default()
    };
    capacity += spaces as f32 * per_space;
    let suffix = text.bytes().rev().take_while(|byte| *byte == b' ').count();
    if suffix == text.len() {
      trailing += suffix as f32 * per_space;
    } else {
      trailing = suffix as f32 * per_space;
    }
  };
  for item in fit.items.iter().skip(fit.item_start) {
    if let PageItem::Text(text) = item {
      push(&text.text, &text.style);
    }
  }
  push(fit.chunk, fit.style);
  push(fit.text, fit.style);
  for (text, style) in tail {
    push(text, style);
  }
  counter.non_terminal_space_count() > 0 && overflow <= capacity - trailing
}

/// Retry an authored discretionary prefix, retaining its source operator.
/// Native Word's legacy retry starts only when the visible prefix itself
/// overflows. If that prefix fits naturally but its hyphen does not, Word
/// moves the word; it does not initiate contraction for the marker alone.
pub(super) fn soft_hyphen(
  inlines: &[InlineItem],
  start: InlineCursor,
  segment: &TextSegment,
  continuation: Option<CrossRunContinuation>,
  fit: LineShrinkFit<'_>,
  metrics: &mut TextMetrics,
) -> Option<InlineCursor> {
  let current_marker = (!fit.style.wordprocessingml_literal_soft_hyphen)
    .then(|| segment.text.find(super::super::hyphenation::SOFT_HYPHEN))
    .flatten();
  if current_marker.is_none()
    && continuation.is_none_or(|tail| tail.first_discretionary_width_pt.is_none())
  {
    return None;
  }
  let prefix = &segment.text[..current_marker.unwrap_or(segment.text.len())];
  let mut width = super::appended_text_width(fit.chunk, prefix, fit.style, metrics);
  let (marker, marker_style) = if let Some(offset) = current_marker {
    (
      InlineCursor {
        inline_index: start.inline_index,
        text_offset: segment.start + offset,
      },
      fit.style,
    )
  } else {
    let end = continuation?.source_end;
    let mut found = None;
    for (index, inline) in inlines
      .iter()
      .enumerate()
      .take(end.inline_index + 1)
      .skip(start.inline_index + 1)
    {
      let InlineItem::Text(run) = inline else {
        continue;
      };
      let text = if index == end.inline_index {
        &run.text[..end.text_offset]
      } else {
        &run.text
      };
      if !run.style.wordprocessingml_literal_soft_hyphen
        && let Some(offset) = text.find(super::super::hyphenation::SOFT_HYPHEN)
      {
        width += metrics.measure_text(&text[..offset], &run.style);
        found = Some((
          InlineCursor {
            inline_index: index,
            text_offset: offset,
          },
          &run.style,
        ));
        break;
      }
      width += metrics.measure_text(text, &run.style);
    }
    found?
  };
  if !super::text_overflows_line(fit.x, width, fit.line_right) {
    return None;
  }
  width += metrics.measure_text("-", marker_style);
  let tail = inlines
    .iter()
    .enumerate()
    .take(marker.inline_index + 1)
    .skip(start.inline_index + 1)
    .filter_map(|(index, inline)| {
      let InlineItem::Text(run) = inline else {
        return None;
      };
      let text = if index == marker.inline_index {
        &run.text[..marker.text_offset]
      } else {
        &run.text
      };
      Some((text, &run.style))
    })
    .chain(std::iter::once(("-", marker_style)));
  fits_with_tail(
    LineShrinkFit {
      width,
      text: prefix,
      ..fit
    },
    tail,
    metrics,
  )
  .then_some(marker)
}

pub(super) struct Portion {
  pub index: usize,
  pub x: f32,
  pub spaces: usize,
  pub adjustment: f32,
}

pub(super) fn contract(
  portions: &mut [Portion],
  items: &[PageItem],
  overflow: f32,
  metrics: &mut TextMetrics,
) {
  let original = portions.first().map_or(0.0, |portion| portion.adjustment);
  for portion in portions.iter_mut() {
    portion.adjustment = if portion.spaces > 0
      && let PageItem::Text(text) = &items[portion.index]
    {
      metrics
        .legacy_word_space_contraction_capacity(&text.style)
        .unwrap_or_default()
    } else {
      0.0
    };
  }
  if !distribute(portions, overflow) {
    // Other adjustment owners can supply negative glue without a legacy
    // shrink retry. Preserve their existing distribution if these capacities
    // do not account for that budget.
    for portion in portions {
      portion.adjustment = original;
    }
  }
}

fn distribute(portions: &mut [Portion], overflow: f32) -> bool {
  let capacity = portions
    .iter()
    .map(|p| p.spaces as f32 * p.adjustment)
    .sum::<f32>();
  let mut spaces = portions.iter().map(|p| p.spaces).sum::<usize>();
  if spaces == 0 || overflow > capacity + super::LAYOUT_EPSILON_PT {
    return false;
  }
  // Actual MSLS70 ideal arrays place every blank at its own minimum, then
  // share the unused capacity equally, capped at each natural width. This
  // differs from assigning every font the same negative advance. Sort the
  // existing portion buffer to remove saturated slots in O(n log n), without
  // per-character allocation or a convergence loop. The caller restores
  // visual order before applying the advances.
  portions.sort_by(|a, b| a.adjustment.total_cmp(&b.adjustment));
  let mut unused = (capacity - overflow).max(0.0);
  for portion in portions {
    if portion.spaces == 0 {
      portion.adjustment = 0.0;
      continue;
    }
    let share = unused / spaces as f32;
    let retained = share.min(portion.adjustment);
    unused = (unused - retained * portion.spaces as f32).max(0.0);
    spaces -= portion.spaces;
    portion.adjustment = retained - portion.adjustment;
  }
  true
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mixed_fonts_follow_native_legacy_contraction_allocations() {
    // Native mode14 Verdana10, five regular and four bold interior blanks.
    // Captured ideal advances are 2.5747/2.5750 and 2.8728pt respectively;
    // natural advances are 3.515625 and 3.41796875pt. These are independent
    // Word observations. The second, smaller budget separately checks that
    // the lower-capacity blanks can remain natural without losing glue.
    for (overflow, expected) in [(6.885_132, [-0.9408, -0.5453]), (1.3859863, [-0.2772, 0.0])] {
      let mut portions = [
        Portion {
          index: 0,
          x: 0.0,
          spaces: 5,
          adjustment: 0.9741211,
        },
        Portion {
          index: 1,
          x: 1.0,
          spaces: 4,
          adjustment: 0.5786133,
        },
      ];
      assert!(distribute(&mut portions, overflow));
      for portion in &portions {
        assert!((portion.adjustment - expected[portion.index]).abs() < 1.0 / 4096.0);
      }
      let applied = portions
        .iter()
        .map(|p| p.spaces as f32 * p.adjustment)
        .sum::<f32>();
      assert!((applied + overflow).abs() < 0.00001);
    }
  }
}
