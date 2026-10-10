//! Visio sheet lookup. Formulas stay text. This crate does not evaluate them.

mod sheet;

pub use sheet::{CellRecord, master_by_id, page_by_id, page_sheet_cell, shape_by_id, shape_cell};
