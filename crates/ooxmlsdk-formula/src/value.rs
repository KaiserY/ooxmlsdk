use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaValue<'a> {
  Number(f64),
  String(Cow<'a, str>),
  Boolean(bool),
  Error(FormulaErrorValue),
  Blank,
  Matrix(Vec<Vec<FormulaValue<'a>>>),
}

impl Default for FormulaValue<'_> {
  fn default() -> Self {
    Self::Blank
  }
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
}
