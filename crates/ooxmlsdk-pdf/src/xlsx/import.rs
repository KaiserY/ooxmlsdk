use crate::error::Result;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

use super::styles::{DefinedNamesCatalog, StylesCatalog};
use super::workbook::WorkbookFragment;
use super::workbook_catalog::WorkbookCatalog;
use super::workbook_settings::WorkbookGlobals;
use super::worksheet::CalcSheet;

#[derive(Debug)]
pub(crate) struct ExcelImport {
  pub(crate) sheets: Vec<CalcSheet>,
  pub(crate) globals: WorkbookGlobals,
  pub(crate) styles: StylesCatalog,
  pub(crate) defined_names: DefinedNamesCatalog,
  pub(crate) workbook_catalog: WorkbookCatalog,
}

impl ExcelImport {
  pub(crate) fn import_document(
    package: &mut SpreadsheetDocument,
    options: &crate::options::PdfOptions,
  ) -> Result<Self> {
    // Source: LibreOffice sc/source/filter/oox/excelfilter.cxx
    // ExcelFilter::importDocument creates workbook-global state and delegates
    // workbook XML/substream order to WorkbookFragment. Document properties,
    // link-update state, and overflow warnings are represented as structured
    // gaps until the Calc model owner exists in Rust.
    let workbook_part = package.workbook_part()?;
    let workbook = workbook_part.root_element(package)?.clone();
    let globals = WorkbookGlobals::from_workbook(&workbook);
    let workbook_catalog = WorkbookCatalog::from_workbook_part(package, &workbook_part)?;
    let mso_document = is_mso_document(package);

    let mut fragment = WorkbookFragment::new(workbook_part, workbook.clone());
    let mut sheets = fragment.finalize_import(package, mso_document)?;
    super::formula::recalculate_formula_cells(
      &mut sheets,
      &fragment.defined_names,
      options.source_file_name.as_deref(),
      &workbook_catalog,
    );

    Ok(Self {
      sheets,
      globals,
      styles: fragment.styles,
      defined_names: fragment.defined_names,
      workbook_catalog,
    })
  }
}

fn is_mso_document(package: &mut SpreadsheetDocument) -> bool {
  let extended_properties_part = {
    package
      .get_parts_of_type::<
        ooxmlsdk::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
      >()
      .next()
  };
  let application = extended_properties_part
    .and_then(|part| part.root_element(package).ok())
    .and_then(|properties| properties.application.as_deref());
  application.is_some_and(|value| value.contains("Microsoft"))
}
