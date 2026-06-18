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

impl FormulaSymbolPool {
  pub fn intern(&mut self, text: &str) -> FormulaSymbolId {
    if let Some(id) = self.lookup.get(text) {
      return *id;
    }
    let id = FormulaSymbolId(self.symbols.len() as u32);
    let text: FormulaSymbolText = Arc::from(text);
    self.symbols.push(FormulaSymbol { text: text.clone() });
    self.lookup.insert(text, id);
    id
  }

  pub fn get(&self, id: FormulaSymbolId) -> Option<&str> {
    self
      .symbols
      .get(id.0 as usize)
      .map(|symbol| symbol.text.as_ref())
  }

  pub fn len(&self) -> usize {
    self.symbols.len()
  }

  pub fn is_empty(&self) -> bool {
    self.symbols.is_empty()
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormulaSymbol {
  pub(crate) text: FormulaSymbolText,
}
