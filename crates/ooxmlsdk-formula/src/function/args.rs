use crate::FormulaValue;
use crate::evaluator::{FormulaAst, FormulaEvaluator};

pub(crate) type EvalContext<'a, 'doc> = FormulaEvaluator<'a, 'doc>;

#[derive(Clone, Copy)]
pub(crate) enum FunctionArgs<'args, 'doc> {
  Ast(&'args [FormulaAst<'doc>]),
  Values(&'args [FormulaValue<'doc>]),
}

impl<'args, 'doc> FunctionArgs<'args, 'doc> {
  pub(crate) fn new(ast: &'args [FormulaAst<'doc>]) -> Self {
    Self::Ast(ast)
  }

  pub(crate) fn new_values(values: &'args [FormulaValue<'doc>]) -> Self {
    Self::Values(values)
  }

  pub(crate) fn as_ast(self) -> &'args [FormulaAst<'doc>] {
    match self {
      Self::Ast(ast) => ast,
      Self::Values(_) => &[],
    }
  }

  pub(crate) fn as_values(self) -> Option<&'args [FormulaValue<'doc>]> {
    match self {
      Self::Ast(_) => None,
      Self::Values(values) => Some(values),
    }
  }
}
