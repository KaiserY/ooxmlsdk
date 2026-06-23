use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

use crate::error::Result;
use crate::options::PdfOptions;

pub(crate) fn layout(
  package: &mut SpreadsheetDocument,
  options: &mut PdfOptions,
) -> Result<ooxmlsdk_layout::common::LayoutDocument<'static>> {
  let options = options.take_layout_options();
  Ok(ooxmlsdk_layout::xlsx::layout_document(package, &options)?)
}
