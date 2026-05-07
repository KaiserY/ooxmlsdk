/// Options for OOXML to PDF conversion.
#[derive(Clone, Debug)]
pub struct PdfOptions {
  /// Standards to request from the PDF backend.
  ///
  /// The initial renderer uses regular PDF 1.7 output. PDF/A and PDF/UA need
  /// layout tagging and font policy work before they should be exposed here.
  pub standards: Vec<PdfStandard>,

  /// Whether PDF content streams should be compressed.
  pub compress_content_streams: bool,
}

impl Default for PdfOptions {
  fn default() -> Self {
    Self {
      standards: Vec::new(),
      compress_content_streams: true,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfStandard {
  Pdf17,
}
