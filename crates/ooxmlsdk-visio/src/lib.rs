//! Visio sheet lookup. Formulas stay text. This crate does not evaluate them.

mod drawing;
mod ids;
mod sheet;

pub use drawing::minimal_drawing;
pub use ids::Ids;
pub use sheet::{
  CellRecord, MasterShapes, effective_shape_cell, master_by_id, page_by_id, page_sheet_cell,
  shape_by_id, shape_cell,
};
