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

  /// JPEG quality used when the PDF filter asks raster graphics to be stored as JPEG.
  pub jpeg_quality: Option<u8>,

  /// Source file name used by spreadsheet formulas such as CELL("filename").
  pub source_file_name: Option<String>,
}

impl Default for PdfOptions {
  fn default() -> Self {
    Self {
      standards: Vec::new(),
      compress_content_streams: true,
      jpeg_quality: None,
      source_file_name: None,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfStandard {
  Pdf17,
}
