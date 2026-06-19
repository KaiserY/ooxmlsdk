use super::*;
use crate::program::{FormulaExprId, FormulaProgram};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EvalOperand<'doc> {
  Value(FormulaValue<'doc>),
  Reference(QualifiedRange<'doc>),
  ExternalReference(ExternalReferenceId<'doc>),
  Name(Cow<'doc, str>),
}

#[derive(Clone, Copy)]
pub(crate) struct EvalArg<'program, 'doc> {
  pub(crate) program: &'program FormulaProgram,
  pub(crate) id: FormulaExprId,
  pub(crate) borrowed_source: Option<&'doc str>,
}

#[derive(Clone, Copy)]
pub(crate) struct EvalArgs<'program, 'doc> {
  program: &'program FormulaProgram,
  args: &'program [FormulaExprId],
  borrowed_source: Option<&'doc str>,
}

impl<'program, 'doc> EvalArgs<'program, 'doc> {
  pub(crate) fn new(
    program: &'program FormulaProgram,
    args: &'program [FormulaExprId],
    borrowed_source: Option<&'doc str>,
  ) -> Self {
    Self {
      program,
      args,
      borrowed_source,
    }
  }

  pub(crate) fn len(self) -> usize {
    self.args.len()
  }

  pub(crate) fn get(self, index: usize) -> Option<EvalArg<'program, 'doc>> {
    self.args.get(index).copied().map(|id| EvalArg {
      program: self.program,
      id,
      borrowed_source: self.borrowed_source,
    })
  }
}

impl<'doc> EvalOperand<'doc> {
  pub(crate) fn into_value<'a>(
    self,
    evaluator: &FormulaEvaluator<'a, 'doc>,
  ) -> Option<FormulaValue<'doc>> {
    match self {
      EvalOperand::Value(value) => Some(value),
      EvalOperand::Reference(range) => Some(FormulaValue::Reference(range)),
      EvalOperand::ExternalReference(reference) => {
        evaluator.evaluate_external_reference(&reference)
      }
      EvalOperand::Name(name) => evaluator.evaluate_name(&name),
    }
  }
}
