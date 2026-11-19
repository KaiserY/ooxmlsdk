//! Source grouping for Word's two-lines-in-one text, ECMA-376 §17.3.2.10.

use super::{InlineItem, ParagraphFieldEvent, ParagraphFormat, TextRun, TwoLinesInline};
use crate::model::WordprocessingTwoLines;

fn settings(run: &TextRun, bidi: bool) -> Option<WordprocessingTwoLines> {
  // MS-OI29500 §2.1.75: fitText, an equation, or a direction differing from
  // the paragraph overrides eastAsianLayout. Preserve each child's source
  // style and dynamic-field identity until measurement/paint.
  (!run.style.wordprocessingml_math_run
    && run.style.wordprocessing_fit_text.is_none()
    // Native CJK/Arabic/numeric controls keep ordinary text in a bidi
    // paragraph or an explicit RTL run, independently of glyph direction.
    && !bidi
    && run.style.right_to_left != Some(true)
    && !run.text.chars().any(char::is_control))
  .then_some(run.style.wordprocessingml_two_lines)
  .flatten()
}

pub(super) fn resolve(
  inlines: &mut Vec<InlineItem>,
  events: &mut [ParagraphFieldEvent],
  format: &ParagraphFormat,
  compatibility_mode: u16,
) {
  if !inlines
    .iter()
    .any(|item| matches!(item, InlineItem::Text(run) if settings(run, format.bidi).is_some()))
  {
    return;
  }
  let mut source = std::mem::take(inlines).into_iter().enumerate().peekable();
  let mut offsets = Vec::with_capacity(source.len() + 1);
  while let Some((_, item)) = source.next() {
    offsets.push(inlines.len());
    let InlineItem::Text(first) = item else {
      inlines.push(item);
      continue;
    };
    let Some(first_settings) = settings(&first, format.bidi) else {
      inlines.push(InlineItem::Text(first));
      continue;
    };
    let mut runs = vec![first];
    let mut metadata = Vec::new();
    while first_settings.id.is_some() {
      match source.peek() {
        Some((_, InlineItem::Text(run)))
          if settings(run, format.bidi).is_some_and(|next| next.id == first_settings.id) =>
        {
          let Some((_, InlineItem::Text(run))) = source.next() else {
            unreachable!("the next source portion was matching text");
          };
          offsets.push(inlines.len());
          runs.push(run);
        }
        Some((_, InlineItem::BookmarkStart(_) | InlineItem::LastRenderedPageBreak)) => {
          let Some((_, item)) = source.next() else {
            unreachable!()
          };
          offsets.push(inlines.len());
          metadata.push(item);
        }
        _ => break,
      }
    }
    // Zero-width metadata cannot split a shared typographic ID. Its field
    // syntax is also retained independently in the paragraph event stream.
    inlines.append(&mut metadata);
    let strict_japanese = format.strict_first_and_last_chars
      && runs
        .first()
        .and_then(|run| {
          run
            .style
            .east_asia_language
            .as_deref()
            .or(run.style.language.as_deref())
        })
        .is_some_and(|language| {
          language
            .split(['-', '_'])
            .next()
            .is_some_and(|primary| primary.eq_ignore_ascii_case("ja"))
        });
    inlines.push(InlineItem::TwoLines(TwoLinesInline {
      settings: first_settings,
      runs,
      justify: format.justification.is_block(),
      auto_space_de: format.auto_space_de.unwrap_or(true),
      auto_space_dn: format.auto_space_dn.unwrap_or(true),
      auto_space_like_word95: format.auto_space_like_word95,
      compatibility_mode,
      strict_japanese,
      kashida: format.justification.kashida.is_some(),
    }));
  }
  offsets.push(inlines.len());
  // Same boundary transport as source text compaction: enclosing field
  // regions and deferred breaks retain their source-inline positions.
  for event in events {
    match event {
      ParagraphFieldEvent::DeferredParagraphBreak { inline_offset }
      | ParagraphFieldEvent::DeferredReferenceParagraphBreak { inline_offset, .. }
      | ParagraphFieldEvent::GeneratedParagraphBreak { inline_offset, .. }
      | ParagraphFieldEvent::PageReferenceResultStyle { inline_offset, .. } => {
        *inline_offset = offsets[*inline_offset];
      }
      ParagraphFieldEvent::ReferenceResultSpan {
        inline_start,
        inline_end,
        ..
      }
      | ParagraphFieldEvent::ConditionalResultSpan {
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
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::TextStyle;

  fn source(text: &str, id: Option<i32>) -> InlineItem {
    InlineItem::Text(TextRun {
      text: text.into(),
      style: TextStyle {
        wordprocessingml_two_lines: Some(WordprocessingTwoLines {
          id,
          brackets: Default::default(),
        }),
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
  fn native_two_lines_shared_ids_cross_bookmarks_but_absent_ids_stay_separate() {
    for id in [None, Some(42)] {
      let mut input = vec![
        source("甲乙", id),
        InlineItem::BookmarkStart("mark".into()),
        source("丙丁戊己", id),
      ];
      resolve(&mut input, &mut [], &ParagraphFormat::default(), 12);
      let groups: Vec<_> = input
        .iter()
        .filter_map(|item| {
          if let InlineItem::TwoLines(group) = item {
            Some(group)
          } else {
            None
          }
        })
        .collect();
      assert_eq!(groups.len(), if id.is_some() { 1 } else { 2 });
      assert_eq!(
        groups
          .iter()
          .flat_map(|group| &group.runs)
          .map(|run| run.text.as_str())
          .collect::<String>(),
        "甲乙丙丁戊己"
      );
      assert!(
        input
          .iter()
          .any(|item| matches!(item, InlineItem::BookmarkStart(name) if name == "mark"))
      );
    }
  }

  #[test]
  fn native_two_lines_retain_full_size_for_bidi_and_explicit_rtl_contexts() {
    for bidi in [false, true] {
      for rtl in [None, Some(false), Some(true)] {
        let mut input = vec![source("甲乙丙丁", Some(42))];
        let InlineItem::Text(run) = &mut input[0] else {
          unreachable!()
        };
        run.style.right_to_left = rtl;
        let format = ParagraphFormat {
          bidi,
          ..Default::default()
        };
        resolve(&mut input, &mut [], &format, 12);
        assert_eq!(
          matches!(input[0], InlineItem::TwoLines(_)),
          !bidi && rtl != Some(true)
        );
      }
    }
  }
}
