use std::fs::File;
use std::path::Path;

use ooxmlsdk_pdf::{PdfOptions, convert_docx, convert_pptx};

use crate::Result;

pub fn render_fixture_pdf(fixture: &Path) -> Result<Vec<u8>> {
  let file = File::open(fixture)?;
  let options = PdfOptions::default();
  match fixture.extension().and_then(|extension| extension.to_str()) {
    Some("pptx" | "pptm" | "ppsx" | "ppsm") => Ok(convert_pptx(file, options)?),
    _ => Ok(convert_docx(file, options)?),
  }
}
