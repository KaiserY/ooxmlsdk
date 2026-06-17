use super::*;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FormulaAst<'doc> {
  Literal(FormulaValue<'doc>),
  Reference(QualifiedRange<'doc>),
  ExternalReference(ExternalReferenceId<'doc>),
  Name(Cow<'doc, str>),
  Unary {
    op: FormulaOperator,
    expr: Box<FormulaAst<'doc>>,
  },
  Binary {
    op: FormulaOperator,
    left: Box<FormulaAst<'doc>>,
    right: Box<FormulaAst<'doc>>,
  },
  Function {
    name: Cow<'doc, str>,
    function: Option<FormulaFunctionId>,
    args: Vec<FormulaAst<'doc>>,
  },
  Array(Vec<Vec<FormulaAst<'doc>>>),
}

pub(crate) fn is_missing_argument(ast: &FormulaAst<'_>) -> bool {
  matches!(ast, FormulaAst::Literal(FormulaValue::Blank))
}

pub(crate) fn has_missing_required_argument(args: &[FormulaAst<'_>], indices: &[usize]) -> bool {
  indices
    .iter()
    .any(|index| args.get(*index).is_some_and(is_missing_argument))
}
