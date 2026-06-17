use super::lex::{LexErrorValue, LexOperator, LexToken, LexTokenKind};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SemanticToken {
  pub kind: SemanticTokenKind,
  pub start: usize,
  pub end: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct SemanticSpan {
  pub start: usize,
  pub end: usize,
}

impl SemanticSpan {
  fn offset(self, amount: usize) -> Self {
    Self {
      start: self.start + amount,
      end: self.end + amount,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExternalReferenceSpans {
  pub book: SemanticSpan,
  pub sheet: Option<SemanticSpan>,
  pub name: Option<SemanticSpan>,
}

impl ExternalReferenceSpans {
  fn offset(self, amount: usize) -> Self {
    Self {
      book: self.book.offset(amount),
      sheet: self.sheet.map(|span| span.offset(amount)),
      name: self.name.map(|span| span.offset(amount)),
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SemanticTokenKind {
  Text,
  Number(f64),
  Error(LexErrorValue),
  Operator(LexOperator),
  ArrayOpen,
  ArrayClose,
  ArgumentSeparator,
  RowSeparator,
  ParenOpen,
  ParenClose,
  Function { volatile: bool },
  Boolean(bool),
  ExternalReference(ExternalReferenceSpans),
  ReferenceCandidate,
  Name,
  Unsupported,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SemanticWordKind {
  Boolean(bool),
  ExternalReference(ExternalReferenceSpans),
  ReferenceCandidate,
  Name,
}

pub(super) fn semantic_token_from_lex(source: &str, token: LexToken) -> SemanticToken {
  SemanticToken {
    kind: semantic_token_kind(source, token),
    start: token.start,
    end: token.end,
  }
}

fn semantic_token_kind(source: &str, token: LexToken) -> SemanticTokenKind {
  match token.kind {
    LexTokenKind::Text => SemanticTokenKind::Text,
    LexTokenKind::Number(value) => SemanticTokenKind::Number(value),
    LexTokenKind::Error(value) => SemanticTokenKind::Error(value),
    LexTokenKind::Operator(value) => SemanticTokenKind::Operator(value),
    LexTokenKind::ArrayOpen => SemanticTokenKind::ArrayOpen,
    LexTokenKind::ArrayClose => SemanticTokenKind::ArrayClose,
    LexTokenKind::ArgumentSeparator => SemanticTokenKind::ArgumentSeparator,
    LexTokenKind::RowSeparator => SemanticTokenKind::RowSeparator,
    LexTokenKind::ParenOpen => SemanticTokenKind::ParenOpen,
    LexTokenKind::ParenClose => SemanticTokenKind::ParenClose,
    LexTokenKind::Word => {
      let value = &source[token.start..token.end];
      if next_non_space_char(source, token.end) == Some('(') {
        SemanticTokenKind::Function {
          volatile: is_volatile_function_name(value),
        }
      } else {
        match semantic_word_kind(value) {
          SemanticWordKind::Boolean(value) => SemanticTokenKind::Boolean(value),
          SemanticWordKind::ExternalReference(reference) => {
            SemanticTokenKind::ExternalReference(reference.offset(token.start))
          }
          SemanticWordKind::ReferenceCandidate => SemanticTokenKind::ReferenceCandidate,
          SemanticWordKind::Name => SemanticTokenKind::Name,
        }
      }
    }
    LexTokenKind::Unsupported => SemanticTokenKind::Unsupported,
  }
}

pub(crate) fn semantic_word_kind(source: &str) -> SemanticWordKind {
  if source.eq_ignore_ascii_case("TRUE") {
    return SemanticWordKind::Boolean(true);
  }
  if source.eq_ignore_ascii_case("FALSE") {
    return SemanticWordKind::Boolean(false);
  }
  if let Some(reference) = external_reference_spans(source) {
    return SemanticWordKind::ExternalReference(reference);
  }
  if is_reference_candidate(source) {
    SemanticWordKind::ReferenceCandidate
  } else {
    SemanticWordKind::Name
  }
}

fn is_reference_candidate(source: &str) -> bool {
  source
    .bytes()
    .any(|byte| byte.is_ascii_digit() || matches!(byte, b'$' | b':' | b'!' | b'.' | b'[' | b']'))
}

pub(crate) fn external_reference_spans(source: &str) -> Option<ExternalReferenceSpans> {
  let rest = source.strip_prefix('[')?;
  let book_len = rest.find(']')?;
  let book = SemanticSpan {
    start: 1,
    end: 1 + book_len,
  };
  let rest_start = book.end + 1;
  let rest = source.get(rest_start..)?;
  let (sheet, name) = if let Some(separator) = rest.rfind('!') {
    let sheet = trim_external_sheet_quotes(
      source,
      SemanticSpan {
        start: rest_start,
        end: rest_start + separator,
      },
    );
    let name = SemanticSpan {
      start: rest_start + separator + 1,
      end: source.len(),
    };
    (Some(sheet).filter(|span| span.start < span.end), Some(name))
  } else {
    (
      None,
      (!rest.is_empty()).then_some(SemanticSpan {
        start: rest_start,
        end: source.len(),
      }),
    )
  };
  Some(ExternalReferenceSpans {
    book,
    sheet,
    name: name.filter(|span| span.start < span.end),
  })
}

fn trim_external_sheet_quotes(source: &str, span: SemanticSpan) -> SemanticSpan {
  let mut start = span.start;
  let mut end = span.end;
  let bytes = source.as_bytes();
  while start < end && bytes[start] == b'\'' {
    start += 1;
  }
  while start < end && bytes[end - 1] == b'\'' {
    end -= 1;
  }
  SemanticSpan { start, end }
}

fn next_non_space_char(source: &str, mut index: usize) -> Option<char> {
  while index < source.len() {
    let ch = source[index..].chars().next()?;
    if !ch.is_whitespace() {
      return Some(ch);
    }
    index += ch.len_utf8();
  }
  None
}

fn is_volatile_function_name(value: &str) -> bool {
  value.eq_ignore_ascii_case("RAND")
    || value.eq_ignore_ascii_case("TODAY")
    || value.eq_ignore_ascii_case("NOW")
    || value.eq_ignore_ascii_case("FORMULA")
    || value.eq_ignore_ascii_case("INFO")
    || value.eq_ignore_ascii_case("INDIRECT")
    || value.eq_ignore_ascii_case("OFFSET")
    || value.eq_ignore_ascii_case("DEBUGVAR")
    || value.eq_ignore_ascii_case("RANDARRAY")
    || value.eq_ignore_ascii_case("RANDBETWEEN")
}
