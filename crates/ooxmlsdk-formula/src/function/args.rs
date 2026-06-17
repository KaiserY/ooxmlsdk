use super::FunctionArgReader;
use crate::evaluator::{EvalArgs, FormulaAst, FormulaEvaluator};

pub(crate) type EvalContext<'a, 'doc> = FormulaEvaluator<'a, 'doc>;

#[derive(Clone, Copy)]
pub(crate) enum FunctionArgs<'args, 'doc> {
  Ast(&'args [FormulaAst<'doc>]),
  Lazy(EvalArgs<'args, 'doc>),
}

impl<'args, 'doc> FunctionArgs<'args, 'doc> {
  pub(crate) fn new(ast: &'args [FormulaAst<'doc>]) -> Self {
    Self::Ast(ast)
  }

  pub(crate) fn new_lazy(args: EvalArgs<'args, 'doc>) -> Self {
    Self::Lazy(args)
  }

  pub(crate) fn as_ast(self) -> Option<&'args [FormulaAst<'doc>]> {
    match self {
      Self::Ast(ast) => Some(ast),
      Self::Lazy(_) => None,
    }
  }

  pub(crate) fn reader<'eval>(
    self,
    evaluator: &'eval FormulaEvaluator<'eval, 'doc>,
  ) -> FunctionArgReader<'args, 'eval, 'doc> {
    FunctionArgReader::new(self, evaluator)
  }
}
