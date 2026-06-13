use crate::docx::TextStyle;
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

  #[error("lopdf failed to patch PDF output: {0}")]
  Lopdf(String),

  #[error("required PDF font was not found: family={family} bold={bold} italic={italic}")]
  FontUnavailable {
    family: String,
    bold: bool,
    italic: bool,
  },
}

impl PdfError {
  pub(crate) fn font_unavailable(style: &TextStyle) -> Self {
    Self::FontUnavailable {
      family: style
        .font_family
        .as_deref()
        .filter(|family| !family.trim().is_empty())
        .unwrap_or("<document-default>")
        .to_string(),
      bold: style.bold,
      italic: style.italic,
    }
  }
}
