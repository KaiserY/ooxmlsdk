use std::borrow::Cow;

use crate::dependency::{ExternalReferenceId, cow_span_text, external_reference_id_from_spans};
use crate::function::{FormulaFunctionId, resolve_function_name};
use crate::{FormulaErrorValue, FormulaOperator, FormulaValue, QualifiedRange, SheetId, parser};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FormulaCode<'doc> {
  pub(crate) ops: Vec<FormulaOp<'doc>>,
}

impl<'doc> FormulaCode<'doc> {
  pub(crate) fn from_parser_ast(
    sheet: SheetId,
    source: &str,
    borrowed_source: Option<&'doc str>,
    ast: Option<&parser::FormulaAst>,
  ) -> Option<Self> {
    let mut ops = Vec::new();
    lower_node(sheet, source, borrowed_source, ast?, &mut ops)?;
    Some(Self { ops })
  }

  pub(crate) fn into_owned(self) -> FormulaCode<'static> {
    FormulaCode {
      ops: self.ops.into_iter().map(FormulaOp::into_owned).collect(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FormulaOp<'doc> {
  PushBlank,
  PushText(Cow<'doc, str>),
  PushNumber(f64),
  PushBoolean(bool),
  PushError(FormulaErrorValue),
  PushReference(QualifiedRange<'doc>),
  PushExternal(ExternalReferenceId<'doc>),
  PushName(Cow<'doc, str>),
  Unary(FormulaOperator),
  Binary(FormulaOperator),
  Call {
    name: Cow<'doc, str>,
    function: Option<FormulaFunctionId>,
    argc: usize,
    arg_ranges: Vec<FormulaArgRange>,
    volatile: bool,
    control: Option<FormulaControlOp>,
  },
  Array {
    row_lengths: Vec<usize>,
  },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FormulaArgRange {
  pub(crate) start: usize,
  pub(crate) end: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FormulaControlOp {
  IfJump,
  IfErrorJump,
  IfNaJump,
  IfsJump,
  SwitchJump,
  ChooseJump,
  LetBind,
}

impl<'doc> FormulaOp<'doc> {
  fn into_owned(self) -> FormulaOp<'static> {
    match self {
      FormulaOp::PushBlank => FormulaOp::PushBlank,
      FormulaOp::PushText(value) => FormulaOp::PushText(Cow::Owned(value.into_owned())),
      FormulaOp::PushNumber(value) => FormulaOp::PushNumber(value),
      FormulaOp::PushBoolean(value) => FormulaOp::PushBoolean(value),
      FormulaOp::PushError(value) => FormulaOp::PushError(value),
      FormulaOp::PushReference(value) => FormulaOp::PushReference(value.into_owned()),
      FormulaOp::PushExternal(value) => FormulaOp::PushExternal(value.into_owned()),
      FormulaOp::PushName(value) => FormulaOp::PushName(Cow::Owned(value.into_owned())),
      FormulaOp::Unary(value) => FormulaOp::Unary(value),
      FormulaOp::Binary(value) => FormulaOp::Binary(value),
      FormulaOp::Call {
        name,
        function,
        argc,
        arg_ranges,
        volatile,
        control,
      } => FormulaOp::Call {
        name: Cow::Owned(name.into_owned()),
        function,
        argc,
        arg_ranges,
        volatile,
        control,
      },
      FormulaOp::Array { row_lengths } => FormulaOp::Array { row_lengths },
    }
  }
}

fn lower_node<'doc>(
  sheet: SheetId,
  source: &str,
  borrowed_source: Option<&'doc str>,
  ast: &parser::FormulaAst,
  ops: &mut Vec<FormulaOp<'doc>>,
) -> Option<()> {
  match ast {
    parser::FormulaAst::Blank => ops.push(FormulaOp::PushBlank),
    parser::FormulaAst::Text(span) => {
      ops.push(FormulaOp::PushText(formula_text_cow(
        source,
        borrowed_source,
        span.start,
      )?));
    }
    parser::FormulaAst::Number(value) => ops.push(FormulaOp::PushNumber(*value)),
    parser::FormulaAst::Error(error) => {
      ops.push(FormulaOp::PushError(formula_error_from_lex(*error)))
    }
    parser::FormulaAst::Word { span, kind } => {
      lower_word(sheet, source, borrowed_source, *span, *kind, ops)?;
    }
    parser::FormulaAst::Unary { op, expr } => {
      lower_node(sheet, source, borrowed_source, expr, ops)?;
      ops.push(FormulaOp::Unary(unary_operator_from_lex(*op)?));
    }
    parser::FormulaAst::Binary { op, left, right } => {
      lower_node(sheet, source, borrowed_source, left, ops)?;
      lower_node(sheet, source, borrowed_source, right, ops)?;
      ops.push(FormulaOp::Binary(formula_operator_from_lex(*op)));
    }
    parser::FormulaAst::Function {
      name,
      volatile,
      args,
    } => {
      let mut arg_ranges = Vec::with_capacity(args.len());
      for arg in args {
        let start = ops.len();
        lower_node(sheet, source, borrowed_source, arg, ops)?;
        arg_ranges.push(FormulaArgRange {
          start,
          end: ops.len(),
        });
      }
      let name = cow_span_text(source, borrowed_source, *name);
      let function = resolve_function_name(name.as_ref());
      ops.push(FormulaOp::Call {
        name,
        function,
        argc: args.len(),
        arg_ranges,
        volatile: *volatile,
        control: control_for_function(function),
      });
    }
    parser::FormulaAst::LogicalFunction {
      function: logical_function,
      args,
    } => {
      let mut arg_ranges = Vec::with_capacity(args.len());
      for arg in args {
        let start = ops.len();
        lower_node(sheet, source, borrowed_source, arg, ops)?;
        arg_ranges.push(FormulaArgRange {
          start,
          end: ops.len(),
        });
      }
      let name = Cow::Borrowed(logical_function.name());
      let function = resolve_function_name(name.as_ref());
      ops.push(FormulaOp::Call {
        name,
        function,
        argc: args.len(),
        arg_ranges,
        volatile: false,
        control: control_for_function(function),
      });
    }
    parser::FormulaAst::Array(rows) => {
      let mut row_lengths = Vec::with_capacity(rows.len());
      for row in rows {
        row_lengths.push(row.len());
        for item in row {
          lower_node(sheet, source, borrowed_source, item, ops)?;
        }
      }
      ops.push(FormulaOp::Array { row_lengths });
    }
  }
  Some(())
}

fn control_for_function(function: Option<FormulaFunctionId>) -> Option<FormulaControlOp> {
  match function? {
    FormulaFunctionId::If => Some(FormulaControlOp::IfJump),
    FormulaFunctionId::Iferror => Some(FormulaControlOp::IfErrorJump),
    FormulaFunctionId::Ifna => Some(FormulaControlOp::IfNaJump),
    FormulaFunctionId::Ifs => Some(FormulaControlOp::IfsJump),
    FormulaFunctionId::Switch => Some(FormulaControlOp::SwitchJump),
    FormulaFunctionId::Choose => Some(FormulaControlOp::ChooseJump),
    FormulaFunctionId::Let => Some(FormulaControlOp::LetBind),
    _ => None,
  }
}

fn lower_word<'doc>(
  sheet: SheetId,
  source: &str,
  borrowed_source: Option<&'doc str>,
  span: parser::SemanticSpan,
  kind: parser::SemanticWordKind,
  ops: &mut Vec<FormulaOp<'doc>>,
) -> Option<()> {
  let word = source.get(span.start..span.end)?;
  match kind {
    parser::SemanticWordKind::Boolean(value) => ops.push(FormulaOp::PushBoolean(value)),
    parser::SemanticWordKind::ExternalReference(reference) => {
      ops.push(FormulaOp::PushExternal(external_reference_id_from_spans(
        word,
        borrowed_source.and_then(|source| source.get(span.start..span.end)),
        reference,
      )));
    }
    parser::SemanticWordKind::ReferenceCandidate => {
      if let Some(range) = parser::parse_formula_range(sheet, word) {
        ops.push(FormulaOp::PushReference(range));
      } else {
        ops.push(FormulaOp::PushName(cow_span_text(
          source,
          borrowed_source,
          span,
        )));
      }
    }
    parser::SemanticWordKind::Name => {
      ops.push(FormulaOp::PushName(cow_span_text(
        source,
        borrowed_source,
        span,
      )));
    }
  }
  Some(())
}

pub(crate) fn formula_operator_from_lex(operator: parser::LexOperator) -> FormulaOperator {
  match operator {
    parser::LexOperator::Add => FormulaOperator::Add,
    parser::LexOperator::Subtract => FormulaOperator::Subtract,
    parser::LexOperator::Multiply => FormulaOperator::Multiply,
    parser::LexOperator::Divide => FormulaOperator::Divide,
    parser::LexOperator::Power => FormulaOperator::Power,
    parser::LexOperator::Concat => FormulaOperator::Concat,
    parser::LexOperator::Equal => FormulaOperator::Equal,
    parser::LexOperator::NotEqual => FormulaOperator::NotEqual,
    parser::LexOperator::Less => FormulaOperator::Less,
    parser::LexOperator::LessOrEqual => FormulaOperator::LessOrEqual,
    parser::LexOperator::Greater => FormulaOperator::Greater,
    parser::LexOperator::GreaterOrEqual => FormulaOperator::GreaterOrEqual,
    parser::LexOperator::Range => FormulaOperator::Range,
    parser::LexOperator::Union => FormulaOperator::Union,
    parser::LexOperator::Intersection => FormulaOperator::Intersection,
    parser::LexOperator::ImplicitIntersection => FormulaOperator::ImplicitIntersection,
    parser::LexOperator::Percent => FormulaOperator::Percent,
  }
}

pub(crate) fn formula_error_from_lex(error: parser::LexErrorValue) -> FormulaErrorValue {
  match error {
    parser::LexErrorValue::Null => FormulaErrorValue::Null,
    parser::LexErrorValue::Div0 => FormulaErrorValue::Div0,
    parser::LexErrorValue::Value => FormulaErrorValue::Value,
    parser::LexErrorValue::Ref => FormulaErrorValue::Ref,
    parser::LexErrorValue::Name => FormulaErrorValue::Name,
    parser::LexErrorValue::Num => FormulaErrorValue::Num,
    parser::LexErrorValue::NA => FormulaErrorValue::NA,
    parser::LexErrorValue::GettingData => FormulaErrorValue::GettingData,
    parser::LexErrorValue::Spill => FormulaErrorValue::Spill,
    parser::LexErrorValue::Calc => FormulaErrorValue::Calc,
    parser::LexErrorValue::Error => FormulaErrorValue::Error,
    parser::LexErrorValue::NotImplemented => FormulaErrorValue::NotImplemented,
    parser::LexErrorValue::CircularReference => FormulaErrorValue::CircularReference,
    parser::LexErrorValue::IllegalChar => FormulaErrorValue::IllegalChar,
    parser::LexErrorValue::IllegalArgument => FormulaErrorValue::IllegalArgument,
    parser::LexErrorValue::IllegalParameter => FormulaErrorValue::IllegalParameter,
    parser::LexErrorValue::Pair => FormulaErrorValue::Pair,
    parser::LexErrorValue::PairExpected => FormulaErrorValue::PairExpected,
    parser::LexErrorValue::OperatorExpected => FormulaErrorValue::OperatorExpected,
    parser::LexErrorValue::VariableExpected => FormulaErrorValue::VariableExpected,
    parser::LexErrorValue::Parameter => FormulaErrorValue::Parameter,
    parser::LexErrorValue::CodeOverflow => FormulaErrorValue::CodeOverflow,
    parser::LexErrorValue::StringOverflow => FormulaErrorValue::StringOverflow,
    parser::LexErrorValue::StackOverflow => FormulaErrorValue::StackOverflow,
    parser::LexErrorValue::InvalidVariable => FormulaErrorValue::InvalidVariable,
    parser::LexErrorValue::InvalidOpcode => FormulaErrorValue::InvalidOpcode,
    parser::LexErrorValue::InvalidStackValue => FormulaErrorValue::InvalidStackValue,
    parser::LexErrorValue::InvalidToken => FormulaErrorValue::InvalidToken,
    parser::LexErrorValue::NoConvergence => FormulaErrorValue::NoConvergence,
    parser::LexErrorValue::NoAddin => FormulaErrorValue::NoAddin,
    parser::LexErrorValue::NoMacro => FormulaErrorValue::NoMacro,
    parser::LexErrorValue::NestedArray => FormulaErrorValue::NestedArray,
    parser::LexErrorValue::MatrixSize => FormulaErrorValue::MatrixSize,
    parser::LexErrorValue::BadArrayContent => FormulaErrorValue::BadArrayContent,
    parser::LexErrorValue::LinkFormulaNeedingCheck => FormulaErrorValue::LinkFormulaNeedingCheck,
  }
}

fn unary_operator_from_lex(operator: parser::LexOperator) -> Option<FormulaOperator> {
  match operator {
    parser::LexOperator::Add => Some(FormulaOperator::UnaryPlus),
    parser::LexOperator::Subtract => Some(FormulaOperator::UnaryMinus),
    parser::LexOperator::ImplicitIntersection => Some(FormulaOperator::ImplicitIntersection),
    parser::LexOperator::Percent => Some(FormulaOperator::Percent),
    _ => None,
  }
}

fn formula_text_cow<'doc>(
  source: &str,
  borrowed_source: Option<&'doc str>,
  start: usize,
) -> Option<Cow<'doc, str>> {
  if let Some(borrowed_source) = borrowed_source {
    return match parser::formula_text_literal(borrowed_source, start)? {
      parser::TextLiteral::Borrowed(value) => Some(Cow::Borrowed(value)),
      parser::TextLiteral::Owned(value) => Some(Cow::Owned(value)),
    };
  }
  match parser::formula_text_literal(source, start)? {
    parser::TextLiteral::Borrowed(value) => Some(Cow::Owned(value.to_string())),
    parser::TextLiteral::Owned(value) => Some(Cow::Owned(value)),
  }
}

pub(crate) fn formula_value_from_array_constant<'doc>(
  value: parser::ArrayConstantValue<'_>,
) -> FormulaValue<'doc> {
  match value {
    parser::ArrayConstantValue::Blank => FormulaValue::Blank,
    parser::ArrayConstantValue::Number(value) => FormulaValue::Number(value),
    parser::ArrayConstantValue::Boolean(value) => FormulaValue::Boolean(value),
    parser::ArrayConstantValue::Error(value) => FormulaValue::Error(formula_error_from_lex(value)),
    parser::ArrayConstantValue::Text(value) => match value {
      parser::TextLiteral::Borrowed(value) => FormulaValue::String(Cow::Owned(value.into())),
      parser::TextLiteral::Owned(value) => FormulaValue::String(Cow::Owned(value)),
    },
    parser::ArrayConstantValue::Raw(value) => {
      FormulaValue::String(Cow::Owned(value.trim_matches('"').to_string()))
    }
  }
}
