use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeError {
  #[error("StdFmtError")]
  StdFmtError(#[from] std::fmt::Error),
  #[error("unknown error")]
  UnknownError,
}
