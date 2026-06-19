use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

use crate::error::Result;
use crate::layout::LayoutDocument;
use crate::options::PdfOptions;

pub(crate) fn layout(
  package: &mut SpreadsheetDocument,
  options: &PdfOptions,
) -> Result<LayoutDocument> {
  let options = ooxmlsdk_layout::options::LayoutOptions {
    source_file_name: options.source_file_name.clone(),
  };
  let layout = ooxmlsdk_layout::xlsx::layout_document(package, &options)?;
  Ok(crate::layout::from_common_document(layout))
}
