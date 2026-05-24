mod comments;
mod display;
mod drawing;
mod import;
mod object_resources;
mod page_settings;
mod pivot;
mod print;
mod query;
mod sheet_conditions;
mod sheet_objects;
mod sheet_relationships;
mod sheet_settings;
mod sheet_view;
mod styles;
mod table;
mod text;
mod workbook;
mod workbook_catalog;
mod workbook_settings;
mod worksheet;

use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

use crate::error::Result;
use crate::layout::LayoutDocument;
use crate::options::PdfOptions;

use import::ExcelImport;

pub(crate) fn layout(
  package: &mut SpreadsheetDocument,
  _options: &PdfOptions,
) -> Result<LayoutDocument> {
  let import = ExcelImport::import_document(package)?;
  Ok(display::lower_to_layout_document(&import))
}
