use super::FunctionArgReader;
use crate::evaluator::{EvalArgs, FormulaEvaluator};

pub(crate) type EvalContext<'a, 'doc> = FormulaEvaluator<'a, 'doc>;

#[derive(Clone, Copy)]
pub(crate) struct FunctionArgs<'args, 'doc> {
  args: EvalArgs<'args, 'doc>,
}

impl<'args, 'doc> FunctionArgs<'args, 'doc> {
  pub(crate) fn new_lazy(args: EvalArgs<'args, 'doc>) -> Self {
    Self { args }
  }

  pub(crate) fn reader<'eval>(
    self,
    evaluator: &'eval FormulaEvaluator<'eval, 'doc>,
  ) -> FunctionArgReader<'args, 'eval, 'doc> {
    FunctionArgReader::new(self, evaluator)
  }

  pub(crate) fn len(self) -> usize {
    self.args.len()
  }

  pub(crate) fn get(self, index: usize) -> Option<crate::evaluator::EvalArg<'args, 'doc>> {
    self.args.get(index)
  }
}
