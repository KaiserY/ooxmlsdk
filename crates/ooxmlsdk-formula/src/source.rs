use crate::model::FormulaGrammar;
use crate::{CellAddress, SheetId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormulaSource<'a> {
  pub text: &'a str,
  pub context: FormulaCompileContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormulaCompileContext {
  pub grammar: FormulaGrammar,
  pub position: FormulaSourcePosition,
  pub kind: FormulaSourceKind,
}

impl Default for FormulaCompileContext {
  fn default() -> Self {
    Self {
      grammar: FormulaGrammar::ExcelA1,
      position: FormulaSourcePosition::Cell(FormulaCellAddress {
        sheet: SheetId(1),
        cell: CellAddress { column: 0, row: 0 },
      }),
      kind: FormulaSourceKind::Cell,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormulaCellAddress {
  pub sheet: SheetId,
  pub cell: CellAddress,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaSourcePosition {
  Cell(FormulaCellAddress),
  Sheet(SheetId),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaSourceKind {
  Cell,
  SharedFormula,
  ArrayFormula,
  DataTable,
  DefinedName,
  ConditionalFormat,
  DataValidation,
  DataValidationList,
  TableFormula,
  ChartDataSource,
  Control,
  WebQuery,
}
