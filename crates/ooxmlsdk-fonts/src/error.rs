use thiserror::Error;

pub type Result<T> = std::result::Result<T, FontError>;

#[derive(Debug, Error)]
pub enum FontError {
  #[error("font source is unavailable: {0}")]
  SourceUnavailable(String),
  #[error("font face could not be parsed")]
  InvalidFace,
  #[error("no font face matched the request")]
  NoMatch,
  #[error("text shaping failed")]
  ShapingFailed,
}
