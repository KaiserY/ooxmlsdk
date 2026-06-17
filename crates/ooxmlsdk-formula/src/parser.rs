mod array;
mod formula;
mod lex;
mod reference;
mod semantic;
mod syntax;

pub(crate) use array::{ArrayConstantValue, parse_array_constant};
pub(crate) use formula::{
  FormulaBodyTokenKind, FormulaNode, FormulaParseIssue, normalize_calc_formula_text,
  normalize_excel_formula_text, normalize_open_formula_text, normalize_r1c1_formula_text,
  parse_formula_body, parse_formula_syntax, r1c1_reference_to_a1, r1c1_whole_axis_reference_to_a1,
};
pub(crate) use lex::{
  FormulaCursor, LexErrorValue, LexLogicalFunction, LexOperator, LexToken, LexTokenKind,
  TextLiteral, formula_body_start, formula_error_value, formula_text_literal,
  grouped_formula_number,
};
pub(crate) use reference::parse_formula_range;
pub(crate) use semantic::{ExternalReferenceSpans, SemanticSpan, SemanticWordKind};
