use thiserror::Error;

pub type Result<T> = std::result::Result<T, PdfError>;

#[derive(Debug, Error)]
pub enum PdfError {
  #[error(transparent)]
  Ooxml(#[from] ooxmlsdk::common::SdkError),

  #[error(transparent)]
  Layout(#[from] ooxmlsdk_layout::error::LayoutError),

  #[error("krilla failed to produce PDF output: {0}")]
  Krilla(String),

  #[error("invalid PDF options: {0}")]
  Options(String),

  #[error("lopdf failed to patch PDF output: {0}")]
  Lopdf(String),
}
