//! Shared font resolution, metrics, shaping, and usage support.
//!
//! The crate follows LibreOffice VCL/EditEngine concepts for font selection,
//! metrics, fallback, and text layout data. PDF output is a downstream
//! consumer, not the source of the model.

mod error;
mod model;

pub use error::{FontError, Result};
pub use model::*;
