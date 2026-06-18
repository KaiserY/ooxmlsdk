use std::sync::Arc;

use rustc_hash::FxHashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormulaSymbolId(pub u32);

pub type FormulaSymbolText = Arc<str>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormulaSymbolPool {
  pub(crate) symbols: Vec<FormulaSymbol>,
  pub(crate) lookup: FxHashMap<FormulaSymbolText, FormulaSymbolId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormulaSymbol {
  pub(crate) text: FormulaSymbolText,
}
