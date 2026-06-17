use super::*;
use crate::code::{FormulaArgRange, FormulaOp};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EvalOperand<'doc> {
  Value(FormulaValue<'doc>),
  Reference(QualifiedRange<'doc>),
  ExternalReference(ExternalReferenceId<'doc>),
  Name(Cow<'doc, str>),
}

#[derive(Clone, Copy)]
pub(crate) struct EvalArg<'code, 'doc> {
  pub(crate) ops: &'code [FormulaOp<'doc>],
  pub(crate) range: FormulaArgRange,
}

#[derive(Clone, Copy)]
pub(crate) struct EvalArgs<'code, 'doc> {
  ops: &'code [FormulaOp<'doc>],
  ranges: &'code [FormulaArgRange],
}

impl<'code, 'doc> EvalArgs<'code, 'doc> {
  pub(crate) fn new(ops: &'code [FormulaOp<'doc>], ranges: &'code [FormulaArgRange]) -> Self {
    Self { ops, ranges }
  }

  pub(crate) fn len(self) -> usize {
    self.ranges.len()
  }

  pub(crate) fn get(self, index: usize) -> Option<EvalArg<'code, 'doc>> {
    self.ranges.get(index).copied().map(|range| EvalArg {
      ops: self.ops,
      range,
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
