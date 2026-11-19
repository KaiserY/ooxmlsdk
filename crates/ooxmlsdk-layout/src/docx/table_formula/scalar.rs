//! Scalar Word calculations share the formula crate's arithmetic AST, but not
//! Excel's result formatting, function semantics, references, or error strings.
use ooxmlsdk_formula::FormulaOperator;
use ooxmlsdk_formula::program::{FormulaNodeKind, FormulaNumberLiteral, FormulaProgram};
use ooxmlsdk_formula::source::FormulaSource;
use smallvec::SmallVec;

use super::Number;
use crate::localization::OfficeNumberSeparators;

pub(super) fn evaluate(expression: &str, symbols: OfficeNumberSeparators) -> Option<Number> {
  // Keep unsupported names, localized diagnostics and scientific notation in
  // their existing cache. Bound recursive parsing before entering the shared AST.
  if expression.len() > 4096
    || expression
      .bytes()
      .filter(|ch| matches!(ch, b'+' | b'-' | b'*' | b'/' | b'^' | b'%'))
      .count()
      > 128
  {
    return None;
  }
  let mut nesting = 0_usize;
  for ch in expression.chars() {
    if ch == '(' {
      nesting += 1;
      if nesting > 64 {
        return None;
      }
    } else if ch == ')' {
      nesting = nesting.checked_sub(1)?;
    } else if !ch.is_ascii_digit()
      && !ch.is_whitespace()
      && ch != symbols.decimal
      && !matches!(ch, '+' | '-' | '*' | '/' | '^' | '%')
    {
      return None;
    }
  }
  let normalized = expression.replace(symbols.decimal, ".");
  let program = FormulaProgram::from_source(FormulaSource {
    text: &normalized,
    context: Default::default(),
  });
  if !program.diagnostics.is_empty() {
    return None;
  }
  let root = program.root?;
  // Word treats a parenthesized literal as an accounting-formatted operand;
  // that requires a separate lexical owner from arithmetic parentheses.
  if expression.trim_start().starts_with('(')
    && matches!(
      program.nodes.get(root.0 as usize)?.kind,
      FormulaNodeKind::Number(_)
    )
  {
    return None;
  }
  // Program builder nodes intentionally omit source spans. Numeric leaves are
  // emitted in lexical order; retain authored precision separately from f64.
  let mut precisions = normalized
    .split(|ch: char| !ch.is_ascii_digit() && ch != '.')
    .filter(|literal| !literal.is_empty())
    .map(|literal| {
      literal
        .split_once('.')
        .map_or(0, |(_, part)| part.len())
        .min(15)
    });
  let mut values = SmallVec::<[Number; 16]>::new();
  for node in &program.nodes {
    let number = match node.kind {
      FormulaNodeKind::Number(value) => Number {
        value: match value {
          FormulaNumberLiteral::Integer(value) => value as f64,
          FormulaNumberLiteral::Number(value) => value,
        },
        decimals: precisions.next()?,
        grouped: false,
      },
      FormulaNodeKind::Unary { op, expr } => {
        let mut number = *values.get(expr.0 as usize)?;
        match op {
          FormulaOperator::UnaryMinus => number.value = -number.value,
          FormulaOperator::Percent => {
            number.value /= 100.0;
            number.decimals = (number.decimals + 2).min(15);
          }
          // Unlike Excel, Word rejects a unary plus.
          _ => return None,
        }
        number
      }
      FormulaNodeKind::Binary { op, left, right } => {
        let left = values.get(left.0 as usize)?;
        let right = values.get(right.0 as usize)?;
        Number {
          value: match op {
            FormulaOperator::Add => left.value + right.value,
            FormulaOperator::Subtract => left.value - right.value,
            FormulaOperator::Multiply => left.value * right.value,
            FormulaOperator::Divide if right.value != 0.0 => left.value / right.value,
            FormulaOperator::Power => left.value.powf(right.value),
            _ => return None,
          },
          decimals: left.decimals.max(right.decimals),
          grouped: false,
        }
      }
      _ => return None,
    };
    if !number.value.is_finite() {
      return None;
    }
    values.push(number);
  }
  if precisions.next().is_some() {
    return None;
  }
  let mut number = *values.get(root.0 as usize)?;
  // Native Word carries the maximum authored operand precision, not the sum
  // of multiplication precisions. Integer operands use two fractional places.
  if number.decimals == 0 {
    number.decimals = 2;
  }
  Some(number)
}
