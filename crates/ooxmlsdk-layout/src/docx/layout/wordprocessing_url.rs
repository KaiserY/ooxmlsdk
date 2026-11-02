use std::ops::Range;

use super::{InlineCursor, InlineItem};

/// Automatic proofing sees a URL as one source token, even when formatting or
/// font selection divides it into runs. Native HTTP/HTTPS controls retain the
/// authored '-' opportunities but never insert dictionary hyphens. Writer's
/// SwTextGuess similarly disables automatic hyphenation for URL tokens.
#[derive(Default)]
pub(super) struct Ranges(Vec<Range<InlineCursor>>);

impl Ranges {
  pub(super) fn new(inlines: &[InlineItem]) -> Self {
    // Stream the source once without concatenating runs or rescanning a long
    // URL on every wrapped line. Non-text content ends the token; source-only
    // markers do not. The empty result allocates no buffer.
    let mut source = inlines
      .iter()
      .enumerate()
      .flat_map(|(inline_index, inline)| {
        let (text, separator) = match inline {
          InlineItem::Text(run) => (run.text.as_str(), None),
          InlineItem::BookmarkStart(_)
          | InlineItem::FormWidgetStart(_)
          | InlineItem::FormWidgetEnd(_)
          | InlineItem::LastRenderedPageBreak => ("", None),
          _ => ("", Some(b' ')),
        };
        text
          .bytes()
          .enumerate()
          .map(move |(text_offset, byte)| {
            (
              InlineCursor {
                inline_index,
                text_offset,
              },
              byte,
            )
          })
          .chain(separator.map(move |byte| {
            (
              InlineCursor {
                inline_index,
                text_offset: 0,
              },
              byte,
            )
          }))
      })
      .peekable();
    let mut ranges = Vec::new();
    while let Some((start, byte)) = source.next() {
      if byte != b'h' {
        continue;
      }
      let mut after_scheme = source.clone();
      if !b"ttp".iter().all(|expected| {
        after_scheme
          .next()
          .is_some_and(|(_, byte)| byte == *expected)
      }) {
        continue;
      }
      let Some((_, suffix)) = after_scheme.next() else {
        continue;
      };
      if (suffix != b':'
        && (suffix != b's' || after_scheme.next().is_none_or(|(_, byte)| byte != b':')))
        || !b"//".iter().all(|expected| {
          after_scheme
            .next()
            .is_some_and(|(_, byte)| byte == *expected)
        })
        || after_scheme
          .peek()
          .is_none_or(|(_, byte)| !byte.is_ascii_graphic())
      {
        continue;
      }
      source = after_scheme;
      let mut end = start;
      while source
        .peek()
        .is_some_and(|(_, byte)| byte.is_ascii_graphic())
      {
        let (cursor, _) = source.next().expect("URL byte checked");
        end = InlineCursor {
          text_offset: cursor.text_offset + 1,
          ..cursor
        };
      }
      ranges.push(start..end);
    }
    Self(ranges)
  }

  pub(super) fn overlaps(&self, start: InlineCursor, end: InlineCursor) -> bool {
    if start >= end {
      return false;
    }
    let index = self.0.partition_point(|range| range.end <= start);
    self.0.get(index).is_some_and(|range| range.start < end)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::docx::{TextRun, TextStyle};

  fn run(text: &str) -> InlineItem {
    InlineItem::Text(TextRun {
      text: text.into(),
      style: TextStyle::default(),
      hyperlink_url: None,
      dynamic_field: None,
      style_ref_keys: Vec::new(),
      style_ref_text: None,
      style_ref_numbering_text: None,
      preserve_text_portion: false,
    })
  }

  #[test]
  fn url_source_ranges_keep_formatted_and_cached_break_portions_together() {
    for scheme in ["http", "https"] {
      let text = format!("Prefix {scheme}://www.hightech-strategie.de/a Tail");
      let start = text.find(scheme).unwrap();
      let end = text.find(" Tail").unwrap();
      for cut in start..=end {
        let inlines = [
          run(&text[..cut]),
          InlineItem::BookmarkStart("marker".into()),
          InlineItem::LastRenderedPageBreak,
          run(&text[cut..]),
        ];
        let urls = Ranges::new(&inlines);
        for (index, inline) in inlines.iter().enumerate() {
          let InlineItem::Text(run) = inline else {
            continue;
          };
          let base = if index == 0 { 0 } else { cut };
          for offset in 0..run.text.len() {
            assert_eq!(
              urls.overlaps(
                InlineCursor {
                  inline_index: index,
                  text_offset: offset
                },
                InlineCursor {
                  inline_index: index,
                  text_offset: offset + 1
                },
              ),
              (start..end).contains(&(base + offset)),
              "{scheme}, cut={cut}, inline={index}, offset={offset}"
            );
          }
        }
      }
    }
  }

  #[test]
  fn url_source_ranges_match_existing_ascii_line_break_recognition() {
    for text in [
      "ordinary Forschungseinrichtungen",
      "http:// https:// http:example httpx://example",
      "Prefix http://example/a-branch Tail https://example/b",
      "P,”http://example/a",
      "http://example/?next=https://example/nested Tail",
      "https://example/a\nhttp://example/b",
    ] {
      let inlines = [run(text)];
      let actual = Ranges::new(&inlines);
      let expected: Vec<_> = super::super::word_absolute_url_ranges(text).collect();
      for offset in 0..text.len() {
        assert_eq!(
          actual.overlaps(
            InlineCursor {
              inline_index: 0,
              text_offset: offset
            },
            InlineCursor {
              inline_index: 0,
              text_offset: offset + 1
            },
          ),
          expected.iter().any(|range| range.contains(&offset)),
          "{text}, offset={offset}"
        );
      }
    }
    let inlines = [
      run("http://example/"),
      InlineItem::PageBreak,
      run("ordinary"),
    ];
    assert!(!Ranges::new(&inlines).overlaps(
      InlineCursor {
        inline_index: 2,
        text_offset: 0
      },
      InlineCursor {
        inline_index: 2,
        text_offset: 8
      },
    ));
  }
}
