use super::semantic::SemanticSpan;
use crate::{CellAddress, CellRange, QualifiedAddress, QualifiedRange, SheetId, SheetName};

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

pub(crate) fn parse_formula_range<'doc>(
  sheet: SheetId,
  token: &str,
) -> Option<QualifiedRange<'doc>> {
  if let Some(range) = parse_chained_formula_range(sheet, token) {
    return Some(range);
  }
  if let ReferenceParts::Range { start, end } = reference_parts(token)? {
    let start = span_text(token, start);
    let end = span_text(token, end);
    if let Some(range) = parse_formula_range_from_addresses(sheet, start, end) {
      return Some(range);
    }
  }
  if let Ok(range) = QualifiedRange::parse_a1(sheet, token) {
    return Some(range);
  }
  match reference_parts(token)? {
    ReferenceParts::Range { start, end } => {
      let start = span_text(token, start);
      let end = span_text(token, end);
      parse_formula_range_from_addresses(sheet, start, end)
        .or_else(|| QualifiedRange::parse_a1(sheet, token).ok())
    }
    ReferenceParts::Single(span) => {
      let value = span_text(token, span);
      QualifiedAddress::parse_a1(sheet, value)
        .ok()
        .map(|address| qualified_range_from_address(sheet, address))
    }
  }
}

fn parse_chained_formula_range<'doc>(sheet: SheetId, token: &str) -> Option<QualifiedRange<'doc>> {
  if token.contains('[') {
    return None;
  }
  let mut parts = token.split(':');
  let first = parts.next()?;
  let second = parts.next()?;
  let third = parts.next()?;
  let mut range = parse_formula_range_from_addresses(sheet, first, second)?;
  let third = QualifiedAddress::parse_a1(sheet, third).ok()?;
  range = extend_qualified_range(&range, &qualified_range_from_address(sheet, third))?;
  for part in parts {
    let address = QualifiedAddress::parse_a1(sheet, part).ok()?;
    range = extend_qualified_range(&range, &qualified_range_from_address(sheet, address))?;
  }
  Some(range)
}

fn parse_formula_range_from_addresses<'doc>(
  sheet: SheetId,
  start: &str,
  end: &str,
) -> Option<QualifiedRange<'doc>> {
  let start = QualifiedAddress::parse_a1(sheet, start).ok()?;
  let end = QualifiedAddress::parse_a1(sheet, end).ok()?;
  extend_qualified_range(
    &qualified_range_from_address(sheet, start),
    &qualified_range_from_address(sheet, end),
  )
}

fn qualified_range_from_address<'doc>(
  sheet: SheetId,
  address: QualifiedAddress<'doc>,
) -> QualifiedRange<'doc> {
  QualifiedRange {
    sheet,
    sheet_name: address.sheet_name,
    end_sheet_name: None,
    range: CellRange {
      start: address.cell,
      end: address.cell,
    },
    start_flags: address.flags,
    end_flags: address.flags,
  }
}

fn extend_qualified_range<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<QualifiedRange<'doc>> {
  let (sheet_name, end_sheet_name) = compatible_range_sheet_name(left, right)?;
  if left.sheet != right.sheet {
    return None;
  }
  let left_start_column = left.range.start.column.min(left.range.end.column);
  let left_end_column = left.range.start.column.max(left.range.end.column);
  let left_start_row = left.range.start.row.min(left.range.end.row);
  let left_end_row = left.range.start.row.max(left.range.end.row);
  let right_start_column = right.range.start.column.min(right.range.end.column);
  let right_end_column = right.range.start.column.max(right.range.end.column);
  let right_start_row = right.range.start.row.min(right.range.end.row);
  let right_end_row = right.range.start.row.max(right.range.end.row);

  Some(QualifiedRange {
    sheet: left.sheet,
    sheet_name,
    end_sheet_name,
    range: CellRange::new(
      CellAddress {
        column: left_start_column.min(right_start_column),
        row: left_start_row.min(right_start_row),
      },
      CellAddress {
        column: left_end_column.max(right_end_column),
        row: left_end_row.max(right_end_row),
      },
    ),
    start_flags: left.start_flags,
    end_flags: right.end_flags,
  })
}

fn compatible_range_sheet_name<'doc>(
  left: &QualifiedRange<'doc>,
  right: &QualifiedRange<'doc>,
) -> Option<(Option<SheetName<'doc>>, Option<SheetName<'doc>>)> {
  match (&left.sheet_name, &right.sheet_name) {
    (Some(left), Some(right)) if left != right => Some((Some(left.clone()), Some(right.clone()))),
    (Some(left), _) => Some((Some(left.clone()), None)),
    (_, Some(right)) => Some((Some(right.clone()), None)),
    (None, None) => Some((None, None)),
  }
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

fn span_text(source: &str, span: SemanticSpan) -> &str {
  source.get(span.start..span.end).unwrap_or_default()
}
