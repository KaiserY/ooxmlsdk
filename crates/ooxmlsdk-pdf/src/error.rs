use thiserror::Error;

pub type Result<T> = std::result::Result<T, PdfError>;

#[derive(Debug, Error)]
pub enum PdfError {
  #[error(transparent)]
  Ooxml(#[from] ooxmlsdk::common::SdkError),

  #[error("krilla failed to produce PDF output: {0}")]
  Krilla(String),

  #[error("no usable system font was found for PDF text output")]
  FontUnavailable,
}
