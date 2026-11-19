//! Source-owned analysis for ordinary, static Word direction regions.
//!
//! Word's completed line analyzes its own source plus a compatibility-dependent
//! UTF16 lookahead. It does not reuse an arbitrarily longer draft paragraph.

use std::borrow::Cow;

use super::{BidiClass, BidiInfo, InlineCursor, InlineItem, Level, bidi_class};

pub(super) struct Analysis {
  text: String,
  inline_starts: Vec<usize>,
  levels: Vec<Level>,
  simple_latin_glyphs: bool,
  lookahead: usize,
}

#[derive(Clone)]
pub(super) struct Line<'a> {
  pub text: &'a str,
  pub levels: Vec<Level>,
  pub simple_latin_glyphs: bool,
}

impl Analysis {
  pub fn for_paragraph(
    paragraph: &crate::docx::Paragraph,
    compatibility_mode: u16,
  ) -> Option<Self> {
    let mut has_rtl = false;
    for inline in &paragraph.inlines {
      match inline {
        InlineItem::Text(run) => {
          if !run.style.wordprocessingml_font_slots
            || run.style.right_to_left == Some(true)
            || run.style.wordprocessing_bidi_scopes.is_some()
            || run.dynamic_field.is_some()
            || run.text.contains(['\t', '\r', '\n', '\u{00ad}'])
          {
            return None;
          }
          has_rtl |= run
            .text
            .chars()
            .any(|ch| matches!(bidi_class(ch), BidiClass::R | BidiClass::AL));
        }
        InlineItem::BookmarkStart(_) | InlineItem::LastRenderedPageBreak => {}
        _ => return None,
      }
    }
    if !has_rtl {
      return None;
    }
    let mut text = String::new();
    let mut inline_starts = Vec::with_capacity(paragraph.inlines.len() + 1);
    for inline in &paragraph.inlines {
      inline_starts.push(text.len());
      if let InlineItem::Text(run) = inline {
        text.push_str(&run.text);
      }
    }
    inline_starts.push(text.len());
    let simple_latin_glyphs = !has_explicit_controls(&text);
    // Authored controls retain their paragraph embedding state. Ordinary
    // completed lines resolve only their actual native source window below.
    let levels = if simple_latin_glyphs {
      Vec::new()
    } else {
      BidiInfo::new(&text, Some(Level::ltr())).levels
    };
    Some(Self {
      text,
      inline_starts,
      levels,
      simple_latin_glyphs,
      lookahead: if compatibility_mode < 15 { 2 } else { 0 },
    })
  }

  fn byte_offset(&self, cursor: InlineCursor) -> Option<usize> {
    let &start = self.inline_starts.get(cursor.inline_index)?;
    let end = self
      .inline_starts
      .get(cursor.inline_index + 1)
      .copied()
      .unwrap_or(start);
    (cursor.text_offset <= end - start).then_some(start + cursor.text_offset)
  }

  pub fn line(&self, start: InlineCursor, end: InlineCursor) -> Option<Line<'_>> {
    let start = self.byte_offset(start)?;
    let end = self.byte_offset(end)?;
    let text = self.text.get(start..end)?;
    let levels = if self.simple_latin_glyphs {
      let context = line_context(self.text.get(start..)?, text.len(), self.lookahead);
      let mut levels = BidiInfo::new(&context, Some(Level::ltr())).levels;
      levels.truncate(text.len());
      levels
    } else {
      self.levels.get(start..end)?.to_vec()
    };
    Some(Line {
      text,
      levels,
      simple_latin_glyphs: self.simple_latin_glyphs,
    })
  }
}

fn line_context(text: &str, line_end: usize, lookahead: usize) -> Cow<'_, str> {
  let mut end = line_end;
  let mut units = 0;
  for ch in text[line_end..].chars() {
    if units == lookahead {
      break;
    }
    if units + ch.len_utf16() > lookahead {
      // Native mode12/14 really submits two UTF16 units, including a lone
      // high surrogate if the second unit cuts a pair. Preserve its neutral
      // replacement in analysis without changing any source/paint character.
      let mut context = text[..end].to_owned();
      context.push(char::REPLACEMENT_CHARACTER);
      return Cow::Owned(context);
    }
    units += ch.len_utf16();
    end += ch.len_utf8();
  }
  Cow::Borrowed(&text[..end])
}

fn has_explicit_controls(text: &str) -> bool {
  text.chars().any(|ch| {
    matches!(
      bidi_class(ch),
      BidiClass::LRE
        | BidiClass::RLE
        | BidiClass::LRO
        | BidiClass::RLO
        | BidiClass::PDF
        | BidiClass::LRI
        | BidiClass::RLI
        | BidiClass::FSI
        | BidiClass::PDI
    )
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn completed_line_uses_native_utf16_window_and_compatibility_boundary() {
    // Native combined source/sink + MSLS line CP observations. The line is
    // four UTF16 units, and modes12/14 submit six; modes15/16 submit four.
    // A following R, L, neutral, NSM and surrogate independently distinguish
    // the source window from either a full paragraph or a next-strong search.
    for (tail, context, dot_level) in [
      ("[ابAA]", "[ا", 1),
      ("[AAاب]", "[A", 0),
      ("[  ابAA]", "[ ", 0),
      ("[ْابAA]", "[ْ", 0),
      ("[😊ابAA]", "[\u{fffd}", 0),
      ("😊ابAA", "😊", 0),
    ] {
      let source = format!("اب. {tail}");
      let end = "اب. ".len();
      assert_eq!(line_context(&source, end, 2), format!("اب. {context}"));
      assert_eq!(line_context(&source, end, 0), "اب. ");
      for mode in [12, 14, 15, 16] {
        let analysis = Analysis {
          inline_starts: vec![0, source.len()],
          text: source.clone(),
          levels: Vec::new(),
          simple_latin_glyphs: true,
          lookahead: if mode < 15 { 2 } else { 0 },
        };
        let line = analysis
          .line(
            InlineCursor::default(),
            InlineCursor {
              inline_index: 0,
              text_offset: end,
            },
          )
          .unwrap();
        let dot = line.text.find('.').unwrap();
        assert_eq!(
          line.levels[dot].number(),
          if mode < 15 { dot_level } else { 0 }
        );
        assert_eq!(line.levels.len(), line.text.len());
      }
    }
  }

  #[test]
  fn authored_directional_controls_keep_the_complex_glyph_protocol() {
    assert!(!has_explicit_controls("اب \" اب"));
    for (open, close) in [
      ('\u{202a}', '\u{202c}'),
      ('\u{202b}', '\u{202c}'),
      ('\u{202d}', '\u{202c}'),
      ('\u{202e}', '\u{202c}'),
      ('\u{2066}', '\u{2069}'),
      ('\u{2067}', '\u{2069}'),
      ('\u{2068}', '\u{2069}'),
    ] {
      assert!(has_explicit_controls(&format!("{open}اب AB اب{close}")));
    }
  }

  #[test]
  fn native_paragraph_levels_survive_physical_line_and_source_run_boundaries() {
    // Actual native source/sink callback, Comment041 derivative control0.
    // Each tuple is a UTF16 source start/length and resolved level. All
    // characters in this independent vector are BMP, so char indices agree.
    let source = "[ارمانAA1] _AFP_ARB.0025 ارا6300 4 ع 3710 قبرص /افب[ارمانAA2] -تصز24 العراق/مجتمع 1 متوقع &HT; اطفال الحظر &HT; (مرفق بصور) &HT; بقلم اسيل طبارة الموفدة الخاصة لوكالة فرانس برس ";
    let native = [
      (0, 1, 0),
      (1, 5, 1),
      (6, 19, 0),
      (25, 3, 1),
      (28, 4, 2),
      (32, 1, 1),
      (33, 1, 2),
      (34, 3, 1),
      (37, 4, 2),
      (41, 10, 1),
      (51, 1, 0),
      (52, 5, 1),
      (57, 6, 0),
      (63, 3, 1),
      (66, 2, 2),
      (68, 14, 1),
      (82, 1, 2),
      (83, 6, 1),
      (89, 6, 0),
      (95, 11, 1),
      (106, 7, 0),
      (113, 9, 1),
      (122, 7, 0),
      (129, 47, 1),
      (176, 1, 0),
    ];
    let text = source.to_owned();
    let levels = BidiInfo::new(&text, Some(Level::ltr())).levels;
    let mut analysis = Analysis {
      text,
      inline_starts: vec![0, source.len()],
      levels,
      simple_latin_glyphs: true,
      lookahead: 2,
    };
    let chars = source.char_indices().collect::<Vec<_>>();
    for (start, length, level) in native {
      for &(byte, ch) in &chars[start..start + length] {
        assert!(
          analysis.levels[byte..byte + ch.len_utf8()]
            .iter()
            .all(|l| l.number() == level)
        );
      }
    }
    let break_byte = source.find("الحظر").unwrap();
    let start = InlineCursor::default();
    let end = InlineCursor {
      inline_index: 0,
      text_offset: break_byte,
    };
    let line = analysis.line(start, end).unwrap();
    let line_levels = line.levels.to_vec();
    assert_eq!(line.levels.last().unwrap().number(), 1);
    let independent = BidiInfo::new(line.text, Some(Level::ltr()));
    assert_eq!(independent.levels.last().unwrap().number(), 0);
    // The real formatter's cursor carries source indices, not an occurrence
    // search. An equivalent source-run split resolves to the same byte range.
    let split = source.find("ل الحظر").unwrap();
    analysis.inline_starts = vec![0, split, source.len()];
    let split_end = InlineCursor {
      inline_index: 1,
      text_offset: break_byte - split,
    };
    assert_eq!(analysis.line(start, split_end).unwrap().levels, line_levels);
    assert!(
      analysis
        .line(
          start,
          InlineCursor {
            inline_index: 0,
            text_offset: split + 1
          }
        )
        .is_none()
    );
  }
}
