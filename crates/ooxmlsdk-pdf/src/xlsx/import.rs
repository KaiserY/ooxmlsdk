use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::error::Result;

use super::pivot::PivotCacheCatalog;
use super::query::ConnectionsCatalog;
use super::styles::{DefinedNamesCatalog, StylesCatalog};
use super::workbook::WorkbookFragment;
use super::workbook_settings::WorkbookGlobals;
use super::worksheet::CalcSheet;

#[derive(Debug)]
pub(crate) struct ExcelImport {
  pub(crate) workbook: x::Workbook,
  pub(crate) sheets: Vec<CalcSheet>,
  pub(crate) shared_strings: Vec<String>,
  pub(crate) globals: WorkbookGlobals,
  pub(crate) pivot_caches: PivotCacheCatalog,
  pub(crate) connections: ConnectionsCatalog,
  pub(crate) styles: StylesCatalog,
  pub(crate) defined_names: DefinedNamesCatalog,
  pub(crate) workbook_resources: WorkbookResourceCatalog,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorkbookResourceCatalog {
  pub(crate) has_theme: bool,
  pub(crate) has_styles: bool,
  pub(crate) has_shared_strings: bool,
  pub(crate) connections: usize,
  pub(crate) custom_xml_maps: usize,
  pub(crate) custom_xml_parts: usize,
  pub(crate) external_workbooks: usize,
  pub(crate) pivot_cache_definitions: usize,
  pub(crate) workbook_persons: usize,
  pub(crate) revision_headers: usize,
  pub(crate) vba_project: bool,
}

impl ExcelImport {
  pub(crate) fn import_document(package: &mut SpreadsheetDocument) -> Result<Self> {
    // Source: LibreOffice sc/source/filter/oox/excelfilter.cxx
    // ExcelFilter::importDocument creates workbook-global state and delegates
    // workbook XML/substream order to WorkbookFragment. Document properties,
    // link-update state, and overflow warnings are represented as structured
    // gaps until the Calc model owner exists in Rust.
    let workbook_part = package.workbook_part()?;
    let workbook = workbook_part.root_element(package)?.clone();
    let globals = WorkbookGlobals::from_workbook(&workbook);
    let pivot_caches = PivotCacheCatalog::from_workbook_part(package, &workbook_part, &workbook)?;
    let connections =
      ConnectionsCatalog::from_part(package, workbook_part.connections_part(package))?;
    let workbook_resources = WorkbookResourceCatalog::from_part(package, &workbook_part);

    let mut fragment = WorkbookFragment::new(workbook_part, workbook.clone());
    let sheets = fragment.finalize_import(package)?;

    Ok(Self {
      workbook,
      sheets,
      shared_strings: fragment.shared_strings,
      globals,
      pivot_caches,
      connections,
      styles: fragment.styles,
      defined_names: fragment.defined_names,
      workbook_resources,
    })
  }
}

impl WorkbookResourceCatalog {
  fn from_part(package: &mut SpreadsheetDocument, workbook_part: &WorkbookPart) -> Self {
    Self {
      has_theme: workbook_part.theme_part(package).is_some(),
      has_styles: workbook_part.workbook_styles_part(package).is_some(),
      has_shared_strings: workbook_part.shared_string_table_part(package).is_some(),
      connections: usize::from(workbook_part.connections_part(package).is_some()),
      custom_xml_maps: usize::from(workbook_part.custom_xml_mappings_part(package).is_some()),
      custom_xml_parts: workbook_part.custom_xml_parts(package).count(),
      external_workbooks: workbook_part.external_workbook_parts(package).count(),
      pivot_cache_definitions: workbook_part
        .pivot_table_cache_definition_parts(package)
        .count(),
      workbook_persons: workbook_part.workbook_person_parts(package).count(),
      revision_headers: usize::from(
        workbook_part
          .workbook_revision_header_part(package)
          .is_some(),
      ),
      vba_project: workbook_part.vba_project_part(package).is_some(),
    }
  }
}
