use std::borrow::Cow;

use super::{
  ArrayConstantValue, ExternalReferenceSpans, FormulaBodyParse, FormulaBodyTokenKind,
  FormulaParseIssue, LexErrorValue, LexOperator, SemanticSpan, SemanticWordKind, TextLiteral,
  formula_text_literal, parse_array_constant, parse_formula_range,
};
use crate::{
  FormulaAst as ModelFormulaAst, FormulaDependency, FormulaErrorValue, FormulaGrammar,
  FormulaOperator, FormulaSeparator, FormulaToken, FormulaValue, ParsedFormula, SheetId,
  UnsupportedFormulaFeature,
};

pub(crate) fn parse_formula<'doc>(
  sheet: SheetId,
  source: Cow<'doc, str>,
  grammar: FormulaGrammar,
) -> ParsedFormula<'doc> {
  let parsed = super::FormulaParser::new(source.as_ref()).parse();
  let text = parsed.body;
  let borrowed_text = match &source {
    Cow::Borrowed(value) => Some(value.get(parsed.body_start..).unwrap_or(value)),
    Cow::Owned(_) => None,
  };
  let lowered = lower_formula_parser_body(sheet, text, borrowed_text, parsed.body_parse);

  ParsedFormula {
    source,
    grammar,
    tokens: lowered.tokens,
    ast: lowered.ast,
    dependencies: lowered.dependencies,
    unsupported: lowered.unsupported,
  }
}

pub(crate) fn parse_formula_ast<'doc>(
  sheet: SheetId,
  text: &str,
) -> (
  Option<ModelFormulaAst<'doc>>,
  Vec<UnsupportedFormulaFeature<'doc>>,
) {
  let parsed = super::FormulaParser::new(text).parse_syntax();
  let mut unsupported =
    formula_parse_issues_to_unsupported(parsed.body, None, parsed.syntax_parse.issues);
  let ast = formula_ast_from_parser_ast(
    sheet,
    parsed.body,
    None,
    parsed.syntax_parse.ast.as_ref(),
    &mut unsupported,
  );
  (ast, unsupported)
}

pub(crate) fn parse_array_constant_formula<'doc>(
  formula: &str,
) -> Option<Vec<Vec<FormulaValue<'doc>>>> {
  parse_array_constant(formula).map(|array| {
    array
      .rows
      .into_iter()
      .map(|row| {
        row
          .into_iter()
          .map(formula_value_from_array_constant)
          .collect()
      })
      .collect()
  })
}

struct LoweredFormula<'doc> {
  tokens: Vec<FormulaToken<'doc>>,
  ast: Option<ModelFormulaAst<'doc>>,
  dependencies: Vec<FormulaDependency<'doc>>,
  unsupported: Vec<UnsupportedFormulaFeature<'doc>>,
}

fn lower_formula_parser_body<'doc>(
  sheet: SheetId,
  text: &str,
  borrowed_text: Option<&'doc str>,
  parsed: FormulaBodyParse,
) -> LoweredFormula<'doc> {
  let mut tokens = Vec::with_capacity(parsed.tokens.len());
  let mut dependencies = Vec::new();
  let mut unsupported = Vec::new();

  for token in parsed.tokens {
    match token.kind {
      FormulaBodyTokenKind::Text => {
        tokens.push(FormulaToken::Literal(formula_text_value(
          text,
          borrowed_text,
          token.span.start,
        )));
      }
      FormulaBodyTokenKind::Number(value) => {
        tokens.push(FormulaToken::Literal(FormulaValue::Number(value)));
      }
      FormulaBodyTokenKind::Error(error) => {
        tokens.push(FormulaToken::Literal(FormulaValue::Error(
          formula_error_from_lex(error),
        )));
      }
      FormulaBodyTokenKind::Operator(operator) => {
        tokens.push(FormulaToken::Operator(formula_operator_from_lex(operator)));
      }
      FormulaBodyTokenKind::ArrayOpen => {
        tokens.push(FormulaToken::ArrayOpen);
      }
      FormulaBodyTokenKind::ArrayClose => {
        tokens.push(FormulaToken::ArrayClose);
      }
      FormulaBodyTokenKind::ArgumentSeparator => {
        tokens.push(FormulaToken::Separator(FormulaSeparator::Argument));
      }
      FormulaBodyTokenKind::RowSeparator => {
        tokens.push(FormulaToken::Separator(FormulaSeparator::Row));
      }
      FormulaBodyTokenKind::Function { volatile } => {
        let word = cow_span_text(text, borrowed_text, token.span);
        if volatile {
          dependencies.push(FormulaDependency::Volatile);
        }
        tokens.push(FormulaToken::Function(word));
      }
      FormulaBodyTokenKind::Boolean(value) => {
        tokens.push(FormulaToken::Literal(FormulaValue::Boolean(value)));
      }
      FormulaBodyTokenKind::ExternalReference(reference) => {
        let external = external_reference_id_from_spans(text, borrowed_text, reference);
        dependencies.push(FormulaDependency::External(external.clone()));
        tokens.push(FormulaToken::ExternalReference(external));
      }
      FormulaBodyTokenKind::ReferenceCandidate => {
        let word = cow_span_text(text, borrowed_text, token.span);
        if let Some(range) = parse_formula_range(sheet, word.as_ref()) {
          dependencies.push(dependency_from_range(sheet, &range));
          tokens.push(FormulaToken::Reference(range));
        } else {
          dependencies.push(FormulaDependency::Name(word.clone()));
          tokens.push(FormulaToken::Name(word));
        }
      }
      FormulaBodyTokenKind::Name => {
        let word = cow_span_text(text, borrowed_text, token.span);
        dependencies.push(FormulaDependency::Name(word.clone()));
        tokens.push(FormulaToken::Name(word));
      }
      FormulaBodyTokenKind::Unsupported => {
        tokens.push(FormulaToken::Unsupported(cow_span_text(
          text,
          borrowed_text,
          token.span,
        )));
      }
    }
  }

  unsupported.extend(formula_parse_issues_to_unsupported(
    text,
    borrowed_text,
    parsed.issues,
  ));
  let ast = formula_ast_from_parser_ast(
    sheet,
    text,
    borrowed_text,
    parsed.ast.as_ref(),
    &mut unsupported,
  );

  LoweredFormula {
    tokens,
    ast,
    dependencies,
    unsupported,
  }
}

fn formula_operator_from_lex(operator: LexOperator) -> FormulaOperator {
  match operator {
    LexOperator::Add => FormulaOperator::Add,
    LexOperator::Subtract => FormulaOperator::Subtract,
    LexOperator::Multiply => FormulaOperator::Multiply,
    LexOperator::Divide => FormulaOperator::Divide,
    LexOperator::Power => FormulaOperator::Power,
    LexOperator::Concat => FormulaOperator::Concat,
    LexOperator::Equal => FormulaOperator::Equal,
    LexOperator::NotEqual => FormulaOperator::NotEqual,
    LexOperator::Less => FormulaOperator::Less,
    LexOperator::LessOrEqual => FormulaOperator::LessOrEqual,
    LexOperator::Greater => FormulaOperator::Greater,
    LexOperator::GreaterOrEqual => FormulaOperator::GreaterOrEqual,
    LexOperator::Range => FormulaOperator::Range,
    LexOperator::Union => FormulaOperator::Union,
    LexOperator::Intersection => FormulaOperator::Intersection,
    LexOperator::Percent => FormulaOperator::Percent,
  }
}

pub(crate) fn formula_error_from_lex(error: LexErrorValue) -> FormulaErrorValue {
  match error {
    LexErrorValue::Null => FormulaErrorValue::Null,
    LexErrorValue::Div0 => FormulaErrorValue::Div0,
    LexErrorValue::Value => FormulaErrorValue::Value,
    LexErrorValue::Ref => FormulaErrorValue::Ref,
    LexErrorValue::Name => FormulaErrorValue::Name,
    LexErrorValue::Num => FormulaErrorValue::Num,
    LexErrorValue::NA => FormulaErrorValue::NA,
    LexErrorValue::GettingData => FormulaErrorValue::GettingData,
    LexErrorValue::Spill => FormulaErrorValue::Spill,
    LexErrorValue::Calc => FormulaErrorValue::Calc,
    LexErrorValue::IllegalArgument => FormulaErrorValue::IllegalArgument,
    LexErrorValue::Parameter => FormulaErrorValue::Parameter,
    LexErrorValue::Unknown => FormulaErrorValue::Unknown,
  }
}

fn formula_ast_from_parser_ast<'doc>(
  sheet: SheetId,
  text: &str,
  borrowed_text: Option<&'doc str>,
  ast: Option<&super::FormulaAst>,
  unsupported: &mut Vec<UnsupportedFormulaFeature<'doc>>,
) -> Option<ModelFormulaAst<'doc>> {
  let converted = ast.and_then(|ast| formula_ast_from_node(sheet, text, borrowed_text, ast));
  if converted.is_none()
    && ast.is_some()
    && !unsupported
      .iter()
      .any(|issue| issue.reason.as_ref() == "formula expression is not fully parsed")
  {
    unsupported.push(UnsupportedFormulaFeature {
      feature: borrowed_text
        .map(Cow::Borrowed)
        .unwrap_or_else(|| Cow::Owned(text.to_string())),
      reason: Cow::Borrowed("formula expression is not fully parsed"),
    });
  }
  converted
}

fn formula_parse_issues_to_unsupported<'doc>(
  text: &str,
  borrowed_text: Option<&'doc str>,
  issues: Vec<FormulaParseIssue>,
) -> Vec<UnsupportedFormulaFeature<'doc>> {
  issues
    .into_iter()
    .map(|issue| match issue {
      FormulaParseIssue::UnrecognizedCharacter(span) => UnsupportedFormulaFeature {
        feature: cow_span_text(text, borrowed_text, span),
        reason: Cow::Borrowed("unrecognized formula character"),
      },
      FormulaParseIssue::MissingClosingParenthesis => UnsupportedFormulaFeature {
        feature: Cow::Borrowed("parenthesized expression"),
        reason: Cow::Borrowed("missing closing parenthesis"),
      },
      FormulaParseIssue::IncompleteExpression => UnsupportedFormulaFeature {
        feature: borrowed_text
          .map(Cow::Borrowed)
          .unwrap_or_else(|| Cow::Owned(text.to_string())),
        reason: Cow::Borrowed("formula expression is not fully parsed"),
      },
    })
    .collect()
}

fn formula_ast_from_node<'doc>(
  sheet: SheetId,
  text: &str,
  borrowed_text: Option<&'doc str>,
  ast: &super::FormulaAst,
) -> Option<ModelFormulaAst<'doc>> {
  match ast {
    super::FormulaAst::Blank => Some(ModelFormulaAst::Literal(FormulaValue::Blank)),
    super::FormulaAst::Text(span) => Some(ModelFormulaAst::Literal(formula_text_value(
      text,
      borrowed_text,
      span.start,
    ))),
    super::FormulaAst::Number(value) => {
      Some(ModelFormulaAst::Literal(FormulaValue::Number(*value)))
    }
    super::FormulaAst::Error(error) => Some(ModelFormulaAst::Literal(FormulaValue::Error(
      formula_error_from_lex(*error),
    ))),
    super::FormulaAst::Word { span, kind } => {
      formula_ast_from_node_word(sheet, text, borrowed_text, *span, *kind)
    }
    super::FormulaAst::Unary { op, expr } => {
      let op = match op {
        LexOperator::Add => FormulaOperator::UnaryPlus,
        LexOperator::Subtract => FormulaOperator::UnaryMinus,
        LexOperator::Percent => FormulaOperator::Percent,
        _ => return None,
      };
      Some(ModelFormulaAst::Unary {
        op,
        expr: Box::new(formula_ast_from_node(sheet, text, borrowed_text, expr)?),
      })
    }
    super::FormulaAst::Binary { op, left, right } => Some(ModelFormulaAst::Binary {
      op: formula_operator_from_lex(*op),
      left: Box::new(formula_ast_from_node(sheet, text, borrowed_text, left)?),
      right: Box::new(formula_ast_from_node(sheet, text, borrowed_text, right)?),
    }),
    super::FormulaAst::Function { name, args } => Some(ModelFormulaAst::Function {
      name: cow_span_text(text, borrowed_text, *name),
      args: formula_ast_args_from_node(sheet, text, borrowed_text, args)?,
    }),
    super::FormulaAst::LogicalFunction { function, args } => Some(ModelFormulaAst::Function {
      name: Cow::Borrowed(function.name()),
      args: formula_ast_args_from_node(sheet, text, borrowed_text, args)?,
    }),
    super::FormulaAst::Array(rows) => Some(ModelFormulaAst::Array(
      rows
        .iter()
        .map(|row| formula_ast_args_from_node(sheet, text, borrowed_text, row))
        .collect::<Option<Vec<_>>>()?,
    )),
  }
}

fn formula_ast_args_from_node<'doc>(
  sheet: SheetId,
  text: &str,
  borrowed_text: Option<&'doc str>,
  args: &[super::FormulaAst],
) -> Option<Vec<ModelFormulaAst<'doc>>> {
  args
    .iter()
    .map(|arg| formula_ast_from_node(sheet, text, borrowed_text, arg))
    .collect()
}

fn formula_ast_from_node_word<'doc>(
  sheet: SheetId,
  text: &str,
  borrowed_text: Option<&'doc str>,
  span: SemanticSpan,
  kind: SemanticWordKind,
) -> Option<ModelFormulaAst<'doc>> {
  let word = text.get(span.start..span.end)?;
  match kind {
    SemanticWordKind::Boolean(value) => {
      return Some(ModelFormulaAst::Literal(FormulaValue::Boolean(value)));
    }
    SemanticWordKind::ExternalReference(reference) => {
      return Some(ModelFormulaAst::ExternalReference(
        external_reference_id_from_spans(
          word,
          borrowed_text.and_then(|source| source.get(span.start..span.end)),
          reference,
        ),
      ));
    }
    SemanticWordKind::ReferenceCandidate => {
      if let Some(range) = parse_formula_range(sheet, word) {
        return Some(ModelFormulaAst::Reference(range));
      }
    }
    SemanticWordKind::Name => {}
  }
  Some(ModelFormulaAst::Name(cow_span_text(
    text,
    borrowed_text,
    span,
  )))
}

fn external_reference_id_from_spans<'doc>(
  source: &str,
  borrowed_source: Option<&'doc str>,
  reference: ExternalReferenceSpans,
) -> crate::ExternalReferenceId<'doc> {
  crate::ExternalReferenceId {
    book: Some(cow_span_text(source, borrowed_source, reference.book)),
    sheet: reference.sheet.map(|sheet| {
      let text = span_text(source, sheet);
      if text.contains("''") {
        Cow::Owned(text.replace("''", "'"))
      } else {
        cow_span_text(source, borrowed_source, sheet)
      }
    }),
    name: reference
      .name
      .map(|name| cow_span_text(source, borrowed_source, name)),
  }
}

fn span_text(source: &str, span: SemanticSpan) -> &str {
  source.get(span.start..span.end).unwrap_or_default()
}

fn cow_span_text<'doc>(
  source: &str,
  borrowed_source: Option<&'doc str>,
  span: SemanticSpan,
) -> Cow<'doc, str> {
  borrowed_source
    .and_then(|source| source.get(span.start..span.end))
    .map(Cow::Borrowed)
    .unwrap_or_else(|| Cow::Owned(span_text(source, span).to_string()))
}

fn dependency_from_range<'doc>(
  sheet: SheetId,
  range: &crate::QualifiedRange<'doc>,
) -> FormulaDependency<'doc> {
  if range.sheet_name.is_none() && range.range.start == range.range.end {
    FormulaDependency::Cell {
      sheet,
      address: range.range.start,
    }
  } else {
    FormulaDependency::Range(range.clone())
  }
}

fn formula_text_value<'doc>(
  text: &str,
  borrowed_text: Option<&'doc str>,
  start: usize,
) -> FormulaValue<'doc> {
  if let Some(borrowed_text) = borrowed_text {
    return match formula_text_literal(borrowed_text, start) {
      Some(TextLiteral::Borrowed(value)) => FormulaValue::String(Cow::Borrowed(value)),
      Some(TextLiteral::Owned(value)) => FormulaValue::String(Cow::Owned(value)),
      None => FormulaValue::String(Cow::Borrowed("")),
    };
  }
  formula_text_value_owned(text, start)
}

fn formula_text_value_owned<'doc>(text: &str, start: usize) -> FormulaValue<'doc> {
  match formula_text_literal(text, start) {
    Some(TextLiteral::Borrowed(value)) => FormulaValue::String(Cow::Owned(value.to_string())),
    Some(TextLiteral::Owned(value)) => FormulaValue::String(Cow::Owned(value)),
    None => FormulaValue::String(Cow::Borrowed("")),
  }
}

fn formula_value_from_array_constant<'doc>(value: ArrayConstantValue<'_>) -> FormulaValue<'doc> {
  match value {
    ArrayConstantValue::Blank => FormulaValue::Blank,
    ArrayConstantValue::Number(value) => FormulaValue::Number(value),
    ArrayConstantValue::Boolean(value) => FormulaValue::Boolean(value),
    ArrayConstantValue::Error(value) => FormulaValue::Error(formula_error_from_lex(value)),
    ArrayConstantValue::Text(value) => match value {
      TextLiteral::Borrowed(value) => FormulaValue::String(Cow::Owned(value.into())),
      TextLiteral::Owned(value) => FormulaValue::String(Cow::Owned(value)),
    },
    ArrayConstantValue::Raw(value) => {
      FormulaValue::String(Cow::Owned(value.trim_matches('"').to_string()))
    }
  }
}
