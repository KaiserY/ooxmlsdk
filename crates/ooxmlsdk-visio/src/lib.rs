//! Visio sheet lookup. Formulas stay text. This crate does not evaluate them.

mod connect;
mod drawing;
mod ids;
mod sheet;
mod text;

pub use connect::{ConnectError, ResolvedConnect, resolve_connects};
pub use drawing::minimal_drawing;
pub use ids::Ids;
pub use sheet::{
  CellRecord, MasterShapes, StyleSlot, effective_shape_cell, master_by_id, page_by_id,
  page_sheet_cell, shape_by_id, shape_cell, styled_shape_cell,
};
pub use text::{TextRun, read_shape_text, write_shape_text};
