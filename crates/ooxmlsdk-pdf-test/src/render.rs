use std::fs::File;
use std::path::Path;

use ooxmlsdk_pdf::{PdfOptions, convert_docx, convert_pptx, convert_xlsx};

use crate::Result;

pub fn render_fixture_pdf(fixture: &Path) -> Result<Vec<u8>> {
  render_fixture_pdf_with_options(fixture, PdfOptions::default())
}

pub fn render_fixture_pdf_with_options(fixture: &Path, options: PdfOptions) -> Result<Vec<u8>> {
  let file = File::open(fixture)?;
  match fixture.extension().and_then(|extension| extension.to_str()) {
    Some("pptx" | "pptm" | "ppsx" | "ppsm") => Ok(convert_pptx(file, options)?),
    Some("xlsx" | "xlsm") => Ok(convert_xlsx(file, options)?),
    _ => Ok(convert_docx(file, options)?),
  }
}
