//! Word calculation diagnostics, before value/reference evaluation.
//!
//! The configured native grammar uses dot decimals and comma list/grouping
//! symbols. Diagnose its deterministic punctuation errors in source order;
//! leave unsupported value syntax on the existing evaluator/cache path.
use crate::localization::OfficeNumberSeparators;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Diagnostic {
  Syntax(char),
  MissingOperator,
  UnexpectedEnd,
  MissingTableCell(CellReference),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct TableBounds {
  pub rows: usize,
  pub columns: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CellReference {
  column: usize,
  row: usize,
}

impl CellReference {
  fn parse(name: &str) -> Option<Self> {
    let split = name.bytes().take_while(u8::is_ascii_alphabetic).count();
    let (letters, digits) = name.split_at(split);
    if letters.is_empty() || digits.is_empty() || !digits.bytes().all(|ch| ch.is_ascii_digit()) {
      return None;
    }
    let column = letters.bytes().try_fold(0_usize, |column, ch| {
      column
        .checked_mul(26)?
        .checked_add(usize::from(ch.to_ascii_uppercase() - b'A') + 1)
    })?;
    let row = digits.parse::<usize>().ok()?;
    // The A1 reference grammar cited byMS-OI29500§2.1.455 has the standard
    // XFD/1048576 bounds. Other identifiers keep their bookmark/name owner.
    (column <= 16384 && (1..=1048576).contains(&row)).then_some(Self { column, row })
  }

  pub(super) fn display(self) -> String {
    format!(
      "{}{}",
      super::alpha_number(self.column as i32, true),
      self.row
    )
  }
}

enum Failure {
  Diagnostic(Diagnostic),
  Unsupported,
}

type ParseResult<T> = Result<T, Failure>;

#[derive(Clone, Copy, PartialEq)]
enum Operand {
  Number,
  Reference,
  Expression,
}

struct Function {
  minimum: usize,
  maximum: Option<usize>,
  aggregate: bool,
  conditional: bool,
}

fn function(name: &str) -> Option<Function> {
  // ECMA-376 §17.16.3.3 and Word's documented table functions. TRUE/FALSE
  // are constants: native TRUE() reports the opening parenthesis.
  let (minimum, maximum, aggregate, conditional) =
    if ["SUM", "AVERAGE", "COUNT", "PRODUCT", "MIN", "MAX"]
      .iter()
      .any(|known| name.eq_ignore_ascii_case(known))
    {
      (1, None, true, false)
    } else if ["ABS", "INT", "SIGN", "NOT", "DEFINED"]
      .iter()
      .any(|known| name.eq_ignore_ascii_case(known))
    {
      (1, Some(1), false, false)
    } else if ["ROUND", "MOD", "OR"]
      .iter()
      .any(|known| name.eq_ignore_ascii_case(known))
    {
      (2, Some(2), false, false)
    } else if name.eq_ignore_ascii_case("AND") {
      (2, None, false, false)
    } else if name.eq_ignore_ascii_case("IF") {
      (3, Some(3), false, true)
    } else {
      return None;
    };
  Some(Function {
    minimum,
    maximum,
    aggregate,
    conditional,
  })
}

pub(super) fn diagnostic(instruction: &str, symbols: OfficeNumberSeparators) -> Option<Diagnostic> {
  // A locale tag supplies number symbols, not arbitrary Windows custom list
  // settings. Other separator profiles need their own observed grammar.
  if symbols != OfficeNumberSeparators::default() || instruction.len() > 4096 {
    return None;
  }
  let expression = instruction.trim_start().strip_prefix('=')?;
  // Semicolons in a numeric picture belong to the formatting switch.
  let expression = expression.split('\\').next()?;
  let mut parser = Parser {
    source: expression,
    offset: 0,
    remaining: 256,
    table_bounds: None,
    missing_cell: None,
  };
  let result = parser
    .expression(0)
    .and_then(|operand| match parser.peek() {
      None => Ok(()),
      Some(',') if operand == Operand::Reference => Err(Failure::Unsupported),
      Some(ch) if ch.is_ascii_punctuation() => Err(Failure::Diagnostic(Diagnostic::Syntax(ch))),
      Some(_) => Err(Failure::Unsupported),
    });
  match result {
    Err(Failure::Diagnostic(diagnostic)) => Some(diagnostic),
    Ok(()) | Err(Failure::Unsupported) => None,
  }
}

pub(super) fn missing_table_cell(
  instruction: &str,
  symbols: OfficeNumberSeparators,
  bounds: TableBounds,
) -> Option<Diagnostic> {
  if symbols != OfficeNumberSeparators::default() || instruction.len() > 4096 {
    return None;
  }
  let expression = instruction
    .trim_start()
    .strip_prefix('=')?
    .split('\\')
    .next()?;
  let mut parser = Parser {
    source: expression,
    offset: 0,
    remaining: 256,
    table_bounds: Some(bounds),
    missing_cell: None,
  };
  parser.expression(0).ok()?;
  if parser.peek().is_some() {
    return None;
  }
  parser.missing_cell.map(Diagnostic::MissingTableCell)
}

struct Parser<'a> {
  source: &'a str,
  offset: usize,
  remaining: usize,
  table_bounds: Option<TableBounds>,
  missing_cell: Option<CellReference>,
}

impl Parser<'_> {
  fn peek(&mut self) -> Option<char> {
    while let Some(ch) = self.source[self.offset..].chars().next() {
      if !ch.is_whitespace() {
        return Some(ch);
      }
      self.offset += ch.len_utf8();
    }
    None
  }

  fn take(&mut self) -> ParseResult<char> {
    self.remaining = self.remaining.checked_sub(1).ok_or(Failure::Unsupported)?;
    let ch = self
      .peek()
      .ok_or(Failure::Diagnostic(Diagnostic::UnexpectedEnd))?;
    self.offset += ch.len_utf8();
    Ok(ch)
  }

  fn error(&mut self) -> Failure {
    match self.peek() {
      None => Failure::Diagnostic(Diagnostic::UnexpectedEnd),
      Some(ch) if ch.is_ascii_punctuation() => Failure::Diagnostic(Diagnostic::Syntax(ch)),
      Some(_) => Failure::Unsupported,
    }
  }

  fn expression(&mut self, depth: usize) -> ParseResult<Operand> {
    let mut operand = self.value(depth)?;
    loop {
      match self.peek() {
        Some('%') => {
          self.take()?;
          operand = Operand::Expression;
        }
        Some(op @ ('+' | '-' | '*' | '/' | '^' | '=' | '<' | '>' | ':')) => {
          self.take()?;
          if matches!(op, '<' | '>') && matches!(self.peek(), Some('='))
            || op == '<' && self.peek() == Some('>')
          {
            self.take()?;
          }
          self.value(depth)?;
          operand = if op == ':' {
            Operand::Reference
          } else {
            Operand::Expression
          };
        }
        Some('(') if operand != Operand::Reference => {
          return Err(Failure::Diagnostic(Diagnostic::MissingOperator));
        }
        Some(ch) if ch.is_ascii_digit() || ch == '.' => {
          return Err(if operand == Operand::Reference {
            Failure::Unsupported
          } else {
            Failure::Diagnostic(Diagnostic::MissingOperator)
          });
        }
        Some('[' | ']' | '$' | '@' | '{' | '}' | '#') => return Err(Failure::Unsupported),
        _ => return Ok(operand),
      }
    }
  }

  fn value(&mut self, depth: usize) -> ParseResult<Operand> {
    if depth >= 64 {
      return Err(Failure::Unsupported);
    }
    while self.peek() == Some('-') {
      self.take()?;
    }
    match self.peek() {
      Some(ch) if ch.is_ascii_digit() || ch == '.' => self.number(),
      Some(ch) if name_start(ch) => {
        let start = self.offset;
        self.name()?;
        let end = self.offset;
        let name = &self.source[start..end];
        if self.peek() == Some('(') {
          if let Some(function) = function(name) {
            // Aggregate/reference functions have independent missing-range
            // behavior (nativeSUM(E2)=0). Do not apply the scalar operand
            // diagnostic to a function's arguments without its evaluator.
            let previous = self.table_bounds.take();
            let result = self.arguments(function, depth + 1);
            self.table_bounds = previous;
            return result;
          }
          if name.eq_ignore_ascii_case("TRUE") || name.eq_ignore_ascii_case("FALSE") {
            return Err(Failure::Diagnostic(Diagnostic::Syntax('(')));
          }
          // Unknown function/bookmark diagnostics have a separate token and
          // name-resolution owner; do not blame a later separator instead.
          return Err(Failure::Unsupported);
        }
        // Word also accepts a bookmarked table followed by a cell/range name.
        let mut qualified = false;
        while self.peek().is_some_and(name_start) {
          self.name()?;
          qualified = true;
        }
        if !qualified
          && self.missing_cell.is_none()
          && let Some(bounds) = self.table_bounds
          && let Some(cell) = CellReference::parse(name)
          && (cell.column > bounds.columns || cell.row > bounds.rows)
        {
          self.missing_cell = Some(cell);
        }
        Ok(Operand::Reference)
      }
      Some('(') => {
        self.take()?;
        self.expression(depth + 1)?;
        if self.peek() != Some(')') {
          return Err(self.error());
        }
        self.take()?;
        Ok(Operand::Expression)
      }
      Some('[' | ']' | '$' | '@' | '{' | '}' | '#') => Err(Failure::Unsupported),
      _ => Err(self.error()),
    }
  }

  fn name(&mut self) -> ParseResult<()> {
    self.take()?;
    while self.source[self.offset..]
      .chars()
      .next()
      .is_some_and(|ch| ch.is_alphanumeric() || ch == '_')
    {
      self.take()?;
    }
    Ok(())
  }

  fn number(&mut self) -> ParseResult<Operand> {
    let mut digits = 0;
    while self.peek().is_some_and(|ch| ch.is_ascii_digit()) {
      self.take()?;
      digits += 1;
      // A space between numbers is an operator boundary, not more digits.
      if self.source[self.offset..]
        .chars()
        .next()
        .is_some_and(char::is_whitespace)
      {
        break;
      }
    }
    while self.source[self.offset..].starts_with(',') {
      let following = self.source[self.offset + 1..]
        .bytes()
        .take_while(u8::is_ascii_digit)
        .count();
      // Native Word accepts 1,2345 and 1234,567, but rejects 1,23. This is
      // tokenization only; existing arithmetic owns values and formatting.
      if digits == 0 || following < 3 {
        break;
      }
      self.take()?;
      for _ in 0..following {
        self.take()?;
        digits += 1;
      }
    }
    if self.source[self.offset..].starts_with('.') {
      self.take()?;
      while self.source[self.offset..]
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_digit())
      {
        self.take()?;
        digits += 1;
      }
    }
    if digits == 0 {
      return Err(Failure::Diagnostic(Diagnostic::Syntax('.')));
    }
    if self.source[self.offset..]
      .chars()
      .next()
      .is_some_and(name_start)
    {
      // Scientific/alphanumeric tokens produce a different native token
      // diagnostic, sometimes with an unstable suffix. They remain unmodeled.
      return Err(Failure::Unsupported);
    }
    Ok(Operand::Number)
  }

  fn arguments(&mut self, function: Function, depth: usize) -> ParseResult<Operand> {
    self.take()?;
    let mut count = 0;
    loop {
      if self.peek() == Some(')') {
        return Err(self.error());
      }
      let operand = self.expression(depth)?;
      count += 1;
      match self.peek() {
        Some(',') => {
          if function.maximum.is_some_and(|maximum| count >= maximum) {
            return Err(self.error());
          }
          self.take()?;
        }
        Some(')') => {
          if count < function.minimum {
            return Err(if function.conditional {
              self.error()
            } else {
              Failure::Unsupported
            });
          }
          if function.aggregate && count == 1 && operand != Operand::Reference {
            return Err(Failure::Unsupported);
          }
          self.take()?;
          return Ok(Operand::Expression);
        }
        _ => return Err(self.error()),
      }
    }
  }
}

fn name_start(ch: char) -> bool {
  ch.is_alphabetic() || ch == '_'
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn missing_scalar_cell_uses_the_proven_table_address_space() {
    let bounds = TableBounds {
      rows: 2,
      columns: 1,
    };
    for (instruction, expected) in [
      ("=E2", "E2"),
      ("=e2+1", "E2"),
      ("=G8+1", "G8"),
      ("=(A3+1)", "A3"),
    ] {
      let Some(Diagnostic::MissingTableCell(cell)) =
        missing_table_cell(instruction, OfficeNumberSeparators::default(), bounds)
      else {
        panic!("missing scalar reference {instruction}")
      };
      assert_eq!(cell.display(), expected);
    }
    for instruction in [
      "=A1",
      "=A2+1",
      "=SUM(E2)",
      "=IF(E2,1,0)",
      "=NamedTable E2",
      "=E2+;",
      "=A0",
      "=UnknownBookmark",
    ] {
      assert!(
        missing_table_cell(instruction, OfficeNumberSeparators::default(), bounds).is_none(),
        "{instruction}"
      );
    }
  }

  #[test]
  fn native_calculation_diagnostics_follow_the_first_grammar_error() {
    for (expression, expected) in [
      ("ROUND(2,345;1)", Diagnostic::Syntax(';')),
      ("ROUND(A1;2)", Diagnostic::Syntax(';')),
      ("A1*10,1", Diagnostic::Syntax(',')),
      ("SUM(LEFT;RIGHT)", Diagnostic::Syntax(';')),
      ("SUM(1,ROUND(2;3))", Diagnostic::Syntax(';')),
      ("SUM(\"1;2\",3)", Diagnostic::Syntax('"')),
      ("1,2 \\# \"0.00\"", Diagnostic::Syntax(',')),
      ("1+(2,3)", Diagnostic::Syntax(',')),
      ("SUM((1,2))", Diagnostic::Syntax(',')),
      ("IF(1=1;2;3)", Diagnostic::Syntax(';')),
      ("BOGUS;2", Diagnostic::Syntax(';')),
      (";1+2", Diagnostic::Syntax(';')),
      ("1+2;", Diagnostic::Syntax(';')),
      ("1**2", Diagnostic::Syntax('*')),
      ("1+*2", Diagnostic::Syntax('*')),
      ("+1", Diagnostic::Syntax('+')),
      ("2(3)", Diagnostic::MissingOperator),
      ("1 2", Diagnostic::MissingOperator),
      ("1+", Diagnostic::UnexpectedEnd),
      ("(1+2", Diagnostic::UnexpectedEnd),
      ("ROUND(1,2", Diagnostic::UnexpectedEnd),
      ("IF(1=1,2)", Diagnostic::Syntax(')')),
      ("SUM(1,,2)", Diagnostic::Syntax(',')),
      ("SUM(1,)", Diagnostic::Syntax(')')),
      ("SUM()", Diagnostic::Syntax(')')),
      ("TRUE()", Diagnostic::Syntax('(')),
      ("ABS()", Diagnostic::Syntax(')')),
      ("NOT()", Diagnostic::Syntax(')')),
      ("SUM(A1:B2;3)", Diagnostic::Syntax(';')),
      ("", Diagnostic::UnexpectedEnd),
    ] {
      assert_eq!(
        diagnostic(
          &format!(" ={expression} "),
          OfficeNumberSeparators::default()
        ),
        Some(expected),
        "{expression}"
      );
    }
  }

  #[test]
  fn native_valid_and_unmodeled_values_keep_their_existing_evaluator() {
    for expression in [
      "2,345",
      "1,2345",
      "1234,567",
      "1,234,567",
      "1,234+5",
      "(1,234)",
      "SUM(1,2)",
      "ROUND(2.345,1)",
      "ROUND(1,234,2)",
      "A1*10.1",
      "2+3",
      "--1",
      "2^-2",
      "1=1",
      "1<>2",
      "SUM(LEFT)",
      "1 \\# \"0;(-0);zero\"",
      "1e3;2",
      "NOTSUPPORTED(1;2)",
      "BOGUS(2)",
      "SUM(1,234)",
      "ROUND(1)",
      "1/0",
    ] {
      assert_eq!(
        diagnostic(
          &format!(" ={expression} "),
          OfficeNumberSeparators::default()
        ),
        None,
        "{expression}"
      );
    }
    assert_eq!(
      diagnostic("STYLEREF 1", OfficeNumberSeparators::default()),
      None
    );
    assert_eq!(
      diagnostic(
        &format!("={}1", "(".repeat(80)),
        OfficeNumberSeparators::default()
      ),
      None
    );
  }
}
