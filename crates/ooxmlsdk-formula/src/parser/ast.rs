use super::lex::{LexErrorValue, LexLogicalFunction, LexOperator};
use super::semantic::{SemanticSpan, SemanticWordKind};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum FormulaAst {
  Blank,
  Text(SemanticSpan),
  Number(f64),
  Error(LexErrorValue),
  Word {
    span: SemanticSpan,
    kind: SemanticWordKind,
  },
  Unary {
    op: LexOperator,
    expr: Box<FormulaAst>,
  },
  Binary {
    op: LexOperator,
    left: Box<FormulaAst>,
    right: Box<FormulaAst>,
  },
  Function {
    name: SemanticSpan,
    volatile: bool,
    args: Vec<FormulaAst>,
  },
  LogicalFunction {
    function: LexLogicalFunction,
    args: Vec<FormulaAst>,
  },
  Array(Vec<Vec<FormulaAst>>),
}
