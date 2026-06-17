use crate::FormulaValue;
use crate::evaluator::{EvalArgs, FormulaAst, FormulaEvaluator, evaluate_arg_direct};

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

  pub(crate) fn materialize_values(
    self,
    evaluator: &FormulaEvaluator<'_, 'doc>,
  ) -> Option<Vec<FormulaValue<'doc>>> {
    match self {
      Self::Ast(_) => None,
      Self::Lazy(args) => {
        let mut values = Vec::with_capacity(args.len());
        for index in 0..args.len() {
          values.push(evaluate_arg_direct(args.get(index)?, evaluator)?);
        }
        Some(values)
      }
    }
  }
}
