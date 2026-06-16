use super::semantic::SemanticSpan;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ReferenceParts {
  Single(SemanticSpan),
  Range {
    start: SemanticSpan,
    end: SemanticSpan,
  },
}

pub(crate) fn reference_parts(source: &str) -> Option<ReferenceParts> {
  let span = trim_span(
    source,
    SemanticSpan {
      start: 0,
      end: source.len(),
    },
  );
  let separator = range_separator(source, span)?;
  if let Some(separator) = separator {
    let start = trim_span(
      source,
      SemanticSpan {
        start: span.start,
        end: separator,
      },
    );
    let end = trim_span(
      source,
      SemanticSpan {
        start: separator + 1,
        end: span.end,
      },
    );
    if start.start < start.end && end.start < end.end {
      return Some(ReferenceParts::Range { start, end });
    }
  }
  (span.start < span.end).then_some(ReferenceParts::Single(span))
}

fn range_separator(source: &str, span: SemanticSpan) -> Option<Option<usize>> {
  let mut quoted = false;
  let mut bracket_depth = 0u32;
  let mut index = span.start;
  while index < span.end {
    let ch = source[index..span.end].chars().next()?;
    if ch == '\'' {
      let next = index + ch.len_utf8();
      if quoted && source[next..span.end].starts_with('\'') {
        index = next + '\''.len_utf8();
        continue;
      }
      quoted = !quoted;
      index = next;
      continue;
    }
    if !quoted {
      match ch {
        '[' => bracket_depth = bracket_depth.saturating_add(1),
        ']' => bracket_depth = bracket_depth.saturating_sub(1),
        ':' if bracket_depth == 0 => return Some(Some(index)),
        _ => {}
      }
    }
    index += ch.len_utf8();
  }
  Some(None)
}

fn trim_span(source: &str, span: SemanticSpan) -> SemanticSpan {
  let mut start = span.start;
  let mut end = span.end;
  while start < end {
    let Some(ch) = source[start..end].chars().next() else {
      break;
    };
    if !ch.is_whitespace() {
      break;
    }
    start += ch.len_utf8();
  }
  while start < end {
    let Some(ch) = source[..end].chars().next_back() else {
      break;
    };
    if !ch.is_whitespace() {
      break;
    }
    end -= ch.len_utf8();
  }
  SemanticSpan { start, end }
}
