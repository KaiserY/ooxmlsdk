mod array;
mod formula;
mod lex;
mod reference;
mod semantic;
mod shared;
mod table;

pub(crate) use array::{ArrayConstantValue, parse_array_constant};
pub(crate) use formula::{
  FormulaBodyParse, FormulaBodyTokenKind, FormulaParseIssue, FormulaParser,
  normalize_calc_formula_text, normalize_excel_formula_text, normalize_open_formula_text,
  normalize_r1c1_formula_text, r1c1_reference_to_a1, r1c1_whole_axis_reference_to_a1,
};
pub(crate) use lex::{
  LexErrorValue, LexLogicalFunction, LexOperator, LexToken, LexTokenKind, TextLiteral,
  formula_body_start, formula_error_value, formula_text_literal, grouped_formula_number,
  lex_token_at,
};
pub(crate) use reference::parse_formula_range;
pub(crate) use semantic::{
  ExternalReferenceSpans, SemanticSpan, SemanticWordKind, is_volatile_function_name,
  semantic_word_kind,
};
pub(crate) use shared::translate_shared_formula_text;
pub(crate) use table::{
  TableReferenceColumn, TableReferenceItems, TableReferenceSelection,
  parse_table_reference_selection,
};
