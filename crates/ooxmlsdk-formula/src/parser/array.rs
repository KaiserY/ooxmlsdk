use super::lex::{
  FormulaCursor, LexErrorValue, LexOperator, LexTokenKind, TextLiteral, formula_body_start,
  formula_text_literal,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ArrayConstant<'a> {
  pub rows: Vec<Vec<ArrayConstantValue<'a>>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ArrayConstantValue<'a> {
  Blank,
  Number(f64),
  Boolean(bool),
  Error(LexErrorValue),
  Text(TextLiteral<'a>),
}

pub(crate) fn parse_array_constant(source: &str) -> Option<ArrayConstant<'_>> {
  let source = source.trim();
  let source = source.get(formula_body_start(source)..)?;
  let mut cursor = FormulaCursor::new(source);
  cursor.skip_ws();
  cursor.consume_token_kind(LexTokenKind::ArrayOpen)?;

  let mut rows = Vec::new();
  let mut row = Vec::new();
  let mut column_count = None::<usize>;
  loop {
    row.push(parse_array_constant_value(source, &mut cursor)?);
    cursor.skip_ws();
    if cursor
      .consume_token_kind(LexTokenKind::ArgumentSeparator)
      .is_some()
    {
      continue;
    }
    if cursor
      .consume_token_kind(LexTokenKind::RowSeparator)
      .is_some()
    {
      push_array_row(&mut rows, &mut column_count, row)?;
      row = Vec::new();
      continue;
    }
    cursor.consume_token_kind(LexTokenKind::ArrayClose)?;
    push_array_row(&mut rows, &mut column_count, row)?;
    cursor.skip_ws();
    return cursor.is_end().then_some(ArrayConstant { rows });
  }
}

fn push_array_row<'a>(
  rows: &mut Vec<Vec<ArrayConstantValue<'a>>>,
  column_count: &mut Option<usize>,
  row: Vec<ArrayConstantValue<'a>>,
) -> Option<()> {
  let columns = row.len();
  if columns == 0 {
    return None;
  }
  match *column_count {
    Some(expected) if expected != columns => None,
    Some(_) => {
      rows.push(row);
      Some(())
    }
    None => {
      *column_count = Some(columns);
      rows.push(row);
      Some(())
    }
  }
}

fn parse_array_constant_value<'a>(
  source: &'a str,
  cursor: &mut FormulaCursor<'a>,
) -> Option<ArrayConstantValue<'a>> {
  cursor.skip_ws();
  match cursor.peek_token_raw().map(|token| token.kind) {
    Some(
      LexTokenKind::ArgumentSeparator | LexTokenKind::RowSeparator | LexTokenKind::ArrayClose,
    ) => {
      return Some(ArrayConstantValue::Blank);
    }
    Some(LexTokenKind::Text) => {
      let token = cursor.consume_token_kind(LexTokenKind::Text)?;
      return Some(ArrayConstantValue::Text(formula_text_literal(
        source,
        token.start,
      )?));
    }
    Some(LexTokenKind::Number(value)) => {
      cursor.consume_token_kind(LexTokenKind::Number(value))?;
      return Some(ArrayConstantValue::Number(value));
    }
    Some(LexTokenKind::Error(value)) => {
      cursor.consume_token_kind(LexTokenKind::Error(value))?;
      return Some(ArrayConstantValue::Error(value));
    }
    Some(LexTokenKind::Word) => {
      let token = cursor.consume_token_kind(LexTokenKind::Word)?;
      let word = source.get(token.start..token.end)?;
      if word.eq_ignore_ascii_case("TRUE") {
        return Some(ArrayConstantValue::Boolean(true));
      }
      if word.eq_ignore_ascii_case("FALSE") {
        return Some(ArrayConstantValue::Boolean(false));
      }
      return None;
    }
    Some(LexTokenKind::Operator(LexOperator::Add | LexOperator::Subtract)) => {
      if let Some(value) = parse_signed_array_number(cursor) {
        return Some(ArrayConstantValue::Number(value));
      }
    }
    _ => {}
  }
  None
}

fn parse_signed_array_number(cursor: &mut FormulaCursor<'_>) -> Option<f64> {
  let start = cursor.index();
  let sign = match cursor.consume_operator_where(|operator| {
    matches!(operator, LexOperator::Add | LexOperator::Subtract)
  })? {
    LexOperator::Subtract => -1.0,
    LexOperator::Add => 1.0,
    _ => return None,
  };
  cursor.skip_ws();
  let Some(token) = cursor.peek_token_raw() else {
    let _ = cursor.set_index(start);
    return None;
  };
  let LexTokenKind::Number(value) = token.kind else {
    let _ = cursor.set_index(start);
    return None;
  };
  let _ = cursor.consume_token_kind(LexTokenKind::Number(value))?;
  Some(sign * value)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn array_constants_reject_ragged_rows_and_raw_values() {
    assert!(parse_array_constant("{1,2;3}").is_none());
    assert!(parse_array_constant("{1,Name}").is_none());
    assert!(parse_array_constant("{1,A1+1}").is_none());
  }

  #[test]
  fn array_constants_accept_scalar_literals() {
    let array = parse_array_constant(r#"{1,"a";TRUE,#DIV/0!}"#).expect("array constant");
    assert_eq!(array.rows.len(), 2);
    assert_eq!(array.rows[0].len(), 2);
    assert_eq!(array.rows[1].len(), 2);
  }
}
