//! SpreadsheetML formula and workbook value model.
//!
//! The model follows LibreOffice Calc concepts: formula cells, cached values,
//! defined names, shared formula groups, dependency state, and display-value
//! boundaries.

mod address;
mod error;
mod model;
mod provider;
mod value;

pub use address::*;
pub use error::{FormulaError, Result};
pub use model::*;
pub use provider::*;
pub use value::*;
