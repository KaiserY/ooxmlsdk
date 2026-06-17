use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum EvalOperand<'doc> {
  Value(FormulaValue<'doc>),
  Reference(QualifiedRange<'doc>),
  ExternalReference(ExternalReferenceId<'doc>),
  Name(Cow<'doc, str>),
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
