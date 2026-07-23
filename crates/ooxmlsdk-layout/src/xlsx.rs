pub(crate) mod chartex;
mod comments;
mod display;
mod drawing;
mod formula;
mod import;
mod model;
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
use crate::options::LayoutOptions;

use import::ExcelImport;

pub use model::*;

pub fn layout(
  package: &mut SpreadsheetDocument,
  options: &LayoutOptions,
) -> Result<crate::common::LayoutDocument<'static>> {
  layout_document(package, options)
}

pub fn layout_document(
  package: &mut SpreadsheetDocument,
  options: &LayoutOptions,
) -> Result<crate::common::LayoutDocument<'static>> {
  let import = ExcelImport::import_document(package, options)?;
  Ok(display::lower_to_layout_document(&import, options))
}

fn normalize_hyperlink_target(target: &str) -> String {
  if target.starts_with("file:///") {
    target.replace('\\', "/")
  } else {
    target.to_string()
  }
}
