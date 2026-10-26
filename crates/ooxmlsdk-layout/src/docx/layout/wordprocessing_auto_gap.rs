//! Native Word can contract automatic script gaps to half their nominal width.
use super::*;

pub(super) fn materialize(
  items: &mut [PageItem],
  format: &crate::docx::ParagraphFormat,
  flow: FlowContext,
  metrics: &mut TextMetrics,
) {
  if format.bidi
    || format.wordprocessing_shape_story
    || format.word_text_frame_story
    || format.vertical_text_flow.is_some()
    || (format.snap_to_grid.unwrap_or(true) && flow.setup.doc_grid_character_spacing_pt.is_some())
  {
    return;
  }
  let mut previous: Option<(usize, AutoScriptEdge, f32)> = None;
  let mut gaps = Vec::new();
  for (index, item) in items.iter().enumerate() {
    let PageItem::Text(text) = item else {
      previous = None;
      continue;
    };
    if text.pdf_text_segmentation != PdfTextSegmentation::WordLine
      || text.paragraph_bidi
      || text.style.right_to_left == Some(true)
      || text.style.semantic_only
      || text.style.rotation_deg != 0.0
      || text.style.horizontal_scale.unwrap_or(1.0) != 1.0
      || text.style.character_spacing_pt != 0.0
      || text.word_spacing_pt != 0.0
    {
      previous = None;
      continue;
    }
    if let (Some((previous_index, left, end)), Some(right)) = (
      previous,
      auto_script_first_edge(
        &text.text,
        &text.style,
        format.auto_space_like_word95,
        metrics,
      ),
    ) {
      let gap = text.x_pt - end;
      if auto_script_spacing_enabled(left.class, right.class, format)
        && gap > LAYOUT_EPSILON_PT
        && gap >= left.normal_gap_pt * 0.5 - LAYOUT_EPSILON_PT
        && gap <= left.normal_gap_pt + LAYOUT_EPSILON_PT
      {
        gaps.push((previous_index, gap));
      }
    }
    let edge = text
      .text
      .chars()
      .rev()
      .find(|&character| {
        auto_script_class(character, format.auto_space_like_word95) != AutoScriptClass::Transparent
      })
      .and_then(|character| {
        auto_script_edge(
          character,
          &text.style,
          format.auto_space_like_word95,
          metrics,
        )
      });
    previous = edge.map(|edge| {
      (
        index,
        edge,
        text.x_pt + metrics.measure_text(&text.text, &text.style),
      )
    });
  }
  for (index, gap) in gaps {
    let PageItem::Text(text) = &mut items[index] else {
      unreachable!()
    };
    let count = text.text.chars().count();
    if count == 0 {
      continue;
    }
    let mut adjustments = text
      .style
      .wordprocessing_justification_expansion_pt
      .as_deref()
      .map_or_else(|| vec![0.0; count], <[f32]>::to_vec);
    if adjustments.len() != count {
      continue;
    }
    // Actual MSLS70 arrays append automatic spacing to the preceding
    // character. Its source X already reserves the gap: keep all origins and
    // transfer ownership to that character's completed advance exactly once.
    // A second pass observes adjacency and therefore cannot add it again.
    adjustments[count - 1] += gap;
    text.style.wordprocessing_justification_expansion_pt = Some(adjustments.into());
  }
}

pub(super) struct Fit<'a> {
  pub items: &'a mut [PageItem],
  pub start: usize,
  pub y: f32,
  pub chunk: &'a str,
  pub chunk_x: &'a mut f32,
  pub x: &'a mut f32,
  pub leading: &'a mut f32,
  pub text: &'a str,
  pub style: &'a TextStyle,
  pub format: &'a crate::docx::ParagraphFormat,
  pub right: f32,
  pub width: f32,
  pub overflow_punctuation: WordprocessingOverflowPunctuationPolicy,
  pub line_used_punctuation_fit: bool,
}

pub(super) fn contract(fit: Fit<'_>, metrics: &mut TextMetrics) {
  let needed = ((*fit.x + *fit.leading + fit.width - fit.right) * 4096.0).round() as i64;
  if needed <= 0 {
    return;
  }
  let mut gaps = Vec::new();
  let mut previous: Option<(AutoScriptEdge, f32)> = None;
  let old95 = fit.format.auto_space_like_word95;
  for (index, item) in fit.items.iter().enumerate().skip(fit.start) {
    if item_y(item).is_none_or(|y| (y - fit.y).abs() >= 0.01) {
      continue;
    }
    let PageItem::Text(text) = item else { return };
    if text.text.is_empty() {
      continue;
    }
    observe(
      &mut previous,
      &mut gaps,
      Portion {
        boundary: index,
        text: &text.text,
        x: text.x_pt,
        width: metrics.measure_text(&text.text, &text.style),
        style: &text.style,
      },
      fit.format,
      metrics,
    );
  }
  let chunk_boundary = fit.items.len();
  if !fit.chunk.is_empty() {
    observe(
      &mut previous,
      &mut gaps,
      Portion {
        boundary: chunk_boundary,
        text: fit.chunk,
        x: *fit.chunk_x,
        width: *fit.x - *fit.chunk_x,
        style: fit.style,
      },
      fit.format,
      metrics,
    );
  }
  // The candidate's leading gap may not have been materialized into a text
  // portion yet. It owns a separate cursor adjustment from earlier gaps.
  if *fit.leading > 0.0
    && let (Some((left, _)), Some(right)) = (
      previous,
      auto_script_first_edge(fit.text, fit.style, old95, metrics),
    )
  {
    append_gap(
      &mut gaps,
      chunk_boundary + 1,
      left,
      right,
      *fit.leading,
      fit.format,
    );
  }
  let capacity: i64 = gaps.iter().map(|&(_, capacity)| capacity).sum();
  if needed > capacity {
    return;
  }
  // Do not consume glue for a candidate which the independent punctuation
  // pull-in rule still rejects. That would alter a line before moving it on.
  let (natural, limit) = line_cjk_compression_fit(
    CjkLineFit {
      overflow_punctuation: fit.overflow_punctuation,
      x: *fit.x + *fit.leading - needed as f32 / 4096.0,
      line_right: fit.right,
      items: fit.items,
      item_start: fit.start,
      chunk: fit.chunk,
      text: fit.text,
      style: fit.style,
    },
    metrics,
  );
  let character = fit
    .text
    .chars()
    .next()
    .filter(|_| fit.text.chars().count() == 1);
  if blocks_cjk_compression_only_fit(natural, limit, fit.line_used_punctuation_fit, character) {
    return;
  }
  let mut cumulative = 0i64;
  let mut allocated = 0i64;
  for (boundary, capacity_here) in gaps {
    cumulative += capacity_here;
    let target = needed * cumulative / capacity;
    let reduction = (target - allocated) as f32 / 4096.0;
    allocated = target;
    if boundary > chunk_boundary {
      *fit.leading -= reduction;
      continue;
    }
    for item in fit.items.iter_mut().skip(boundary) {
      if let PageItem::Text(text) = item
        && (text.y_pt - fit.y).abs() < 0.01
      {
        text.x_pt -= reduction;
      }
    }
    *fit.chunk_x -= reduction;
    *fit.x -= reduction;
  }
}

fn append_gap(
  gaps: &mut Vec<(usize, i64)>,
  boundary: usize,
  left: AutoScriptEdge,
  right: AutoScriptEdge,
  gap: f32,
  format: &crate::docx::ParagraphFormat,
) {
  if !auto_script_spacing_enabled(left.class, right.class, format) {
    return;
  }
  let units = (gap * 4096.0).round() as i64;
  let normal = (left.normal_gap_pt * 4096.0).round() as i64;
  let minimum = normal / 2;
  // Recognize only the automatic gap's native interval, not a tab or an
  // independently positioned portion. Already contracted gaps stay eligible.
  if units > minimum && units <= normal {
    gaps.push((boundary, units - minimum));
  }
}

struct Portion<'a> {
  boundary: usize,
  text: &'a str,
  x: f32,
  width: f32,
  style: &'a TextStyle,
}

fn observe(
  previous: &mut Option<(AutoScriptEdge, f32)>,
  gaps: &mut Vec<(usize, i64)>,
  portion: Portion<'_>,
  format: &crate::docx::ParagraphFormat,
  metrics: &mut TextMetrics,
) {
  let Portion {
    boundary,
    text,
    x,
    width,
    style,
  } = portion;
  let word95 = format.auto_space_like_word95;
  if let (Some((left, end)), Some(right)) = (
    *previous,
    auto_script_first_edge(text, style, word95, metrics),
  ) {
    append_gap(gaps, boundary, left, right, x - end, format);
  }
  let edge = text
    .chars()
    .rev()
    .find(|&character| auto_script_class(character, word95) != AutoScriptClass::Transparent)
    .and_then(|character| auto_script_edge(character, style, word95, metrics));
  *previous = edge.map(|edge| (edge, x + width));
}
