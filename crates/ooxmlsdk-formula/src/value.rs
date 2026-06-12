use std::borrow::Cow;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum FormulaValue<'a> {
  Number(f64),
  String(Cow<'a, str>),
  Boolean(bool),
  Error(FormulaErrorValue),
  #[default]
  Blank,
  Matrix(Vec<Vec<FormulaValue<'a>>>),
  Reference(crate::QualifiedRange<'a>),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormulaErrorValue {
  Null,
  Div0,
  Value,
  Ref,
  Name,
  Num,
  NA,
  GettingData,
  Spill,
  Calc,
  Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DisplayValue<'a> {
  pub text: Cow<'a, str>,
  pub source_value: FormulaValue<'a>,
  pub number_format_id: Option<u32>,
  pub stale: bool,
  pub error_text: Option<Cow<'a, str>>,
}
