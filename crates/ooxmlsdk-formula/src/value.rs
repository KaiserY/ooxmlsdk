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
  RefList(Vec<crate::QualifiedRange<'a>>),
}

impl<'a> FormulaValue<'a> {
  pub(crate) fn into_owned(self) -> FormulaValue<'static> {
    match self {
      FormulaValue::Number(value) => FormulaValue::Number(value),
      FormulaValue::String(value) => FormulaValue::String(Cow::Owned(value.into_owned())),
      FormulaValue::Boolean(value) => FormulaValue::Boolean(value),
      FormulaValue::Error(value) => FormulaValue::Error(value),
      FormulaValue::Blank => FormulaValue::Blank,
      FormulaValue::Matrix(rows) => FormulaValue::Matrix(
        rows
          .into_iter()
          .map(|row| row.into_iter().map(FormulaValue::into_owned).collect())
          .collect(),
      ),
      FormulaValue::Reference(value) => FormulaValue::Reference(value.into_owned()),
      FormulaValue::RefList(values) => {
        FormulaValue::RefList(values.into_iter().map(|value| value.into_owned()).collect())
      }
    }
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
  Error,
  NotImplemented,
  CircularReference,
  IllegalChar,
  IllegalArgument,
  IllegalParameter,
  Pair,
  PairExpected,
  OperatorExpected,
  VariableExpected,
  Parameter,
  CodeOverflow,
  StringOverflow,
  StackOverflow,
  InvalidVariable,
  InvalidOpcode,
  InvalidStackValue,
  InvalidToken,
  NoConvergence,
  NoAddin,
  NoMacro,
  NestedArray,
  MatrixSize,
  BadArrayContent,
  LinkFormulaNeedingCheck,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DisplayValue<'a> {
  pub text: Cow<'a, str>,
  pub source_value: FormulaValue<'a>,
  pub number_format_id: Option<u32>,
  pub stale: bool,
  pub error_text: Option<Cow<'a, str>>,
}

impl<'a> DisplayValue<'a> {
  pub(crate) fn into_owned(self) -> DisplayValue<'static> {
    DisplayValue {
      text: Cow::Owned(self.text.into_owned()),
      source_value: self.source_value.into_owned(),
      number_format_id: self.number_format_id,
      stale: self.stale,
      error_text: self.error_text.map(|value| Cow::Owned(value.into_owned())),
    }
  }
}
