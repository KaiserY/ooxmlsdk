use std::fs::File;
use std::path::Path;

use ooxmlsdk_pdf::{PdfOptions, convert_docx};

use crate::Result;

pub fn render_fixture_pdf(fixture: &Path) -> Result<Vec<u8>> {
  Ok(convert_docx(File::open(fixture)?, PdfOptions::default())?)
}
