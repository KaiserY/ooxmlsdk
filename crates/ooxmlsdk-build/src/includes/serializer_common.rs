use thiserror::Error;

#[derive(Error, Debug)]
pub enum SeError {
  #[error("unknown error")]
  UnknownError,
}
