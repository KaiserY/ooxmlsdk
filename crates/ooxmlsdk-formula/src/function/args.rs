use crate::evaluator::{FormulaAst, FormulaEvaluator};

pub(crate) type EvalContext<'a, 'doc> = FormulaEvaluator<'a, 'doc>;

#[derive(Clone, Copy)]
pub(crate) struct FunctionArgs<'args, 'doc> {
  ast: &'args [FormulaAst<'doc>],
}

impl<'args, 'doc> FunctionArgs<'args, 'doc> {
  pub(crate) fn new(ast: &'args [FormulaAst<'doc>]) -> Self {
    Self { ast }
  }

  pub(crate) fn as_ast(self) -> &'args [FormulaAst<'doc>] {
    self.ast
  }
}
