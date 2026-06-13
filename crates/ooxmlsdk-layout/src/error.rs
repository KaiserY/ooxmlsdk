use thiserror::Error;

pub type Result<T> = std::result::Result<T, LayoutError>;

#[derive(Debug, Error)]
pub enum LayoutError {
  #[error(transparent)]
  Ooxml(#[from] ooxmlsdk::common::SdkError),
  #[error("unsupported layout feature: {0}")]
  Unsupported(String),
  #[error("invalid layout input: {0}")]
  InvalidInput(String),
  #[error("missing required layout resource: {0}")]
  MissingResource(String),
  #[error("font error: {0}")]
  Font(#[from] ooxmlsdk_fonts::FontError),
}
