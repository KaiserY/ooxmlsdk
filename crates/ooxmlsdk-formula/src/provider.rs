use crate::{
  CellAddress, DisplayValue, FormulaErrorValue, FormulaState, FormulaValue, PivotDataRequest,
  SheetId,
};

pub trait CellValueProvider<'a> {
  fn raw_value(&self, sheet: SheetId, cell: CellAddress) -> Option<FormulaValue<'a>>;

  fn formula_value(&self, sheet: SheetId, cell: CellAddress) -> Option<FormulaValue<'a>>;

  fn cached_value(&self, sheet: SheetId, cell: CellAddress) -> Option<FormulaValue<'a>>;

  fn display_text(&self, sheet: SheetId, cell: CellAddress) -> Option<DisplayValue<'a>>;

  fn formula_state(&self, sheet: SheetId, cell: CellAddress) -> Option<FormulaState>;
}

pub trait PivotDataProvider<'a> {
  fn pivot_data(
    &self,
    request: &PivotDataRequest<'_>,
  ) -> Result<FormulaValue<'a>, FormulaErrorValue>;
}
