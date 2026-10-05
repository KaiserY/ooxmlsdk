//! Word's contiguous manual-width run regions (ECMA-376 §17.3.2.14).
//!
//! Resolve their pitch after run/style import, preserving the ordinary line
//! breaker, shaping and decoration paths. Native Word changes character origins,
//! not the outlines' horizontal scale. Ruby is a separate compound-text owner.

use icu_segmenter::GraphemeClusterSegmenter;

use super::{InlineItem, ParagraphFieldEvent, TextRun};
use crate::model::WordprocessingFitText;
use crate::text_metrics::TextMetrics;

fn fitted_runs(item: &InlineItem) -> Option<(WordprocessingFitText, &[TextRun])> {
  let (fit, runs) = match item {
    InlineItem::Text(run) => (
      run.style.wordprocessing_fit_text?,
      std::slice::from_ref(run),
    ),
    InlineItem::Ruby(ruby) => (ruby.fit_text?, ruby.base.as_slice()),
    _ => return None,
  };
  runs
    .iter()
    .all(|run| run.dynamic_field.is_none() && !run.text.chars().any(char::is_control))
    .then_some((fit, runs))
}

fn character_boundaries(text: &str) -> Vec<usize> {
  GraphemeClusterSegmenter::new().segment_str(text).collect()
}

pub(super) fn resolve(inlines: &mut Vec<InlineItem>, field_events: &mut [ParagraphFieldEvent]) {
  if !inlines.iter().any(|item| fitted_runs(item).is_some()) {
    return;
  }
  let mut metrics = TextMetrics::new();
  let mut tracking = vec![None; inlines.len()];
  let mut terminal = vec![false; inlines.len()];
  let mut start = 0;
  while start < inlines.len() {
    let Some((fit, _)) = fitted_runs(&inlines[start]) else {
      start += 1;
      continue;
    };
    let mut end = start;
    let mut count = 0;
    let mut width = 0.0;
    let mut last = start;
    while end < inlines.len() {
      if matches!(inlines[end], InlineItem::BookmarkStart(_)) {
        end += 1;
        continue;
      }
      let Some((current, runs)) = fitted_runs(&inlines[end]) else {
        break;
      };
      if current.id != fit.id {
        break;
      }
      for run in runs {
        let mut style = run.style.clone();
        // MS-OE376 §2.1.79: fitText takes precedence over authored spacing.
        style.character_spacing_pt = 0.0;
        width += metrics.measure_text(&run.text, &style);
        let characters = character_boundaries(&run.text).len().saturating_sub(1);
        count += characters;
        if characters > 0 {
          last = end;
        }
      }
      end += 1;
    }
    // Native Word treats a zero width as ordinary text, including its
    // original pitch. The first run owns a shared ID's target width.
    if fit.width_pt > 0.0 && count > 0 {
      // Word condenses the final character's advance as well. Expansion
      // instead leaves its natural advance after the last glyph. Native
      // fixed-pitch Latin/CJK controls establish both sides of this boundary.
      let expanding = fit.width_pt > width;
      let intervals = if expanding {
        count.saturating_sub(1).max(1)
      } else {
        count
      };
      let pitch = (fit.width_pt - width) / intervals as f32;
      for (index, value) in tracking.iter_mut().enumerate().take(end).skip(start) {
        if fitted_runs(&inlines[index]).is_some() {
          *value = Some(pitch);
        }
      }
      terminal[last] = expanding && count > 1;
    }
    start = end;
  }
  let mut output = Vec::with_capacity(inlines.len());
  let mut offsets = Vec::with_capacity(inlines.len() + 1);
  for (index, mut item) in inlines.drain(..).enumerate() {
    offsets.push(output.len());
    if let Some(pitch) = tracking[index] {
      match &mut item {
        InlineItem::Text(run) => {
          if let Some(final_run) = apply_pitch(run, pitch, terminal[index]) {
            if !run.text.is_empty() {
              output.push(item);
            }
            output.push(InlineItem::Text(final_run));
            continue;
          }
        }
        InlineItem::Ruby(ruby) => {
          let is_terminal = terminal[index];
          let last = ruby.base.iter().rposition(|run| !run.text.is_empty());
          let mut tail = None;
          for (index, run) in ruby.base.iter_mut().enumerate() {
            if let Some(final_run) = apply_pitch(run, pitch, is_terminal && Some(index) == last) {
              tail = Some((index + 1, final_run));
            }
          }
          if let Some((index, final_run)) = tail {
            ruby.base.insert(index, final_run);
          }
          // Native outer fitText controls override pitch on both ruby rows.
          // The guide retains rubyAlign over the newly allocated base box;
          // it is not another region to expand to the full document target.
          for run in &mut ruby.guide {
            run.style.character_spacing_pt = 0.0;
          }
        }
        _ => {}
      }
    }
    output.push(item);
  }
  offsets.push(output.len());
  // Source field spans still refer to their original complete runs. Splitting
  // the final character must not move a reference field or paragraph break.
  for event in field_events {
    match event {
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
      | ParagraphFieldEvent::DeferredReferenceParagraphBreak { inline_offset, .. } => {
        *inline_offset = offsets[*inline_offset];
      }
      ParagraphFieldEvent::ReferenceResultSpan {
        inline_start,
        inline_end,
        ..
      } => {
        *inline_start = offsets[*inline_start];
        *inline_end = offsets[*inline_end];
      }
      _ => {}
    }
  }
  *inlines = output;
}

fn apply_pitch(run: &mut TextRun, pitch: f32, terminal: bool) -> Option<TextRun> {
  run.style.character_spacing_pt = pitch;
  if !terminal || run.text.is_empty() {
    return None;
  }
  let boundaries = character_boundaries(&run.text);
  let last = boundaries[boundaries.len() - 2];
  let mut final_run = run.clone();
  final_run.text = run.text.split_off(last);
  final_run.style.character_spacing_pt = 0.0;
  Some(final_run)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::model::{TextStyle, WordprocessingFitText};

  fn run(text: &str, id: i32, width_pt: f32, spacing: f32) -> InlineItem {
    InlineItem::Text(TextRun {
      text: text.to_owned(),
      style: TextStyle {
        font_family: Some("ＭＳ 明朝".into()),
        east_asia_font_family: Some("ＭＳ 明朝".into()),
        font_size_pt: 12.0,
        character_spacing_pt: spacing,
        wordprocessing_fit_text: Some(WordprocessingFitText { id, width_pt }),
        ..Default::default()
      },
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    })
  }

  #[test]
  fn linked_fit_runs_share_width_and_distinct_ids_reserve_independent_widths() {
    for (first, last) in [("電話番", "号"), ("ABCD", "E")] {
      for ids in [[0, 0], [1, 1], [1, 2]] {
        for width in [24.0, 48.0, 84.0] {
          let mut inlines = vec![
            run(first, ids[0], width, 3.0),
            run(last, ids[1], width, 6.0),
          ];
          resolve(&mut inlines, &mut []);
          let mut metrics = TextMetrics::new();
          let measured: f32 = inlines
            .iter()
            .map(|item| {
              let InlineItem::Text(run) = item else {
                unreachable!()
              };
              metrics.measure_text(&run.text, &run.style)
            })
            .sum();
          let expected = width * if ids[0] == ids[1] { 1.0 } else { 2.0 };
          assert!(
            (measured - expected).abs() < 0.001,
            "{first}{last} {ids:?} {width}: {measured}"
          );
        }
      }
    }
  }

  #[test]
  fn outer_fit_region_distributes_base_pitch_across_ruby_boxes() {
    let mut inlines = Vec::new();
    for text in ["担当者", "氏名"] {
      let InlineItem::Text(base) = run(text, 1, 84.0, 3.0) else {
        unreachable!()
      };
      let InlineItem::Text(guide) = run("ふり", 1, 84.0, 3.0) else {
        unreachable!()
      };
      inlines.push(InlineItem::Ruby(super::super::RubyInline {
        fit_text: Some(WordprocessingFitText {
          id: 1,
          width_pt: 84.0,
        }),
        base: vec![base],
        guide: vec![guide],
        alignment: super::super::RubyAlignment::DistributeSpace,
        raise_pt: 9.0,
      }));
    }
    resolve(&mut inlines, &mut []);
    let mut metrics = TextMetrics::new();
    for (item, expected) in inlines.iter().zip([54.0, 30.0]) {
      let InlineItem::Ruby(ruby) = item else {
        unreachable!()
      };
      let width: f32 = ruby
        .base
        .iter()
        .map(|run| metrics.measure_text(&run.text, &run.style))
        .sum();
      assert!((width - expected).abs() < 0.001);
      assert!(
        ruby
          .guide
          .iter()
          .all(|run| run.style.character_spacing_pt == 0.0)
      );
    }
  }

  #[test]
  fn first_fit_width_owns_shared_region_and_zero_keeps_source_pitch() {
    let mut inlines = vec![run("ABC", 1, 48.0, 3.0), run("DE", 1, 84.0, 6.0)];
    let mut events = vec![ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 2 }];
    resolve(&mut inlines, &mut events);
    let mut metrics = TextMetrics::new();
    let width: f32 = inlines
      .iter()
      .map(|item| {
        let InlineItem::Text(run) = item else {
          unreachable!()
        };
        metrics.measure_text(&run.text, &run.style)
      })
      .sum();
    assert!((width - 48.0).abs() < 0.001);
    assert!(matches!(
      events[0],
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset: 3 }
    ));
    let mut ordinary = vec![run("ABC", 1, 0.0, 3.0)];
    resolve(&mut ordinary, &mut []);
    let InlineItem::Text(run) = &ordinary[0] else {
      unreachable!()
    };
    assert_eq!(run.style.character_spacing_pt, 3.0);
  }
}
