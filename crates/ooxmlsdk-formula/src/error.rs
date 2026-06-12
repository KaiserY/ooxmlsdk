use thiserror::Error;

pub type Result<T> = std::result::Result<T, FormulaError>;

#[derive(Debug, Error)]
pub enum FormulaError {
  #[error("invalid cell address: {0}")]
  InvalidAddress(String),
  #[error("invalid formula: {0}")]
  InvalidFormula(String),
  #[error("formula feature is not supported: {0}")]
  Unsupported(String),
}
