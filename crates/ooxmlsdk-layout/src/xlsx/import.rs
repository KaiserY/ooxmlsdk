use std::sync::OnceLock;

use crate::error::Result;
use crate::localization::OfficeLocaleContext;
use crate::options::LayoutOptions;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;

use super::formula::{FormulaContext, RelativeFormulaEvaluationContext};
use super::styles::{DefinedNamesCatalog, StylesCatalog};
use super::workbook::WorkbookFragment;
use super::workbook_catalog::WorkbookCatalog;
use super::workbook_settings::WorkbookGlobals;
use super::worksheet::{CalcSheet, SpreadsheetProducerProfile};

#[derive(Debug)]
pub(crate) struct ExcelImport {
  pub(crate) sheets: Vec<CalcSheet>,
  pub(crate) globals: WorkbookGlobals,
  pub(crate) styles: StylesCatalog,
  pub(crate) defined_names: DefinedNamesCatalog,
  pub(crate) workbook_catalog: WorkbookCatalog,
  pub(crate) source_file_name: Option<String>,
  pub(crate) field_update_datetime: Option<crate::options::FieldUpdateDateTime>,
  formula_context: FormulaContext,
  relative_formula_context: OnceLock<RelativeFormulaEvaluationContext>,
}

impl ExcelImport {
  pub(crate) fn import_document(
    package: &SpreadsheetDocument,
    options: &LayoutOptions,
  ) -> Result<Self> {
    // ExcelFilter::importDocument creates workbook-global state and delegates
    // workbook XML/substream order to WorkbookFragment. Document properties,
    // link-update state, and overflow warnings are represented as structured
    // gaps until the Calc model owner exists in Rust.
    let workbook_part = package.workbook_part()?;
    let workbook = workbook_part.root_element(package)?.clone();
    let globals = WorkbookGlobals::from_workbook(&workbook);
    let formula_context = FormulaContext::new(
      globals.settings.date_1904,
      options.field_update_datetime,
      options.ui_language.as_deref(),
    );
    let workbook_catalog = WorkbookCatalog::from_workbook_part(package, &workbook_part)?;
    let producer = spreadsheet_producer_profile(package, &workbook);
    let locales = OfficeLocaleContext::new(
      options.ui_language.as_deref(),
      options.format_locale.as_deref(),
      options.default_document_language.as_deref(),
    );

    let mut fragment = WorkbookFragment::new(workbook_part, workbook.clone());
    let mut sheets = fragment.finalize_import(package, producer, &locales)?;
    super::formula::recalculate_formula_cells(
      &mut sheets,
      &fragment.defined_names,
      options.source_file_name.as_deref(),
      &workbook_catalog,
      &formula_context,
    );
    Ok(Self {
      sheets,
      globals,
      styles: fragment.styles,
      defined_names: fragment.defined_names,
      workbook_catalog,
      source_file_name: options.source_file_name.clone(),
      field_update_datetime: options.field_update_datetime,
      formula_context,
      relative_formula_context: OnceLock::new(),
    })
  }

  pub(crate) fn relative_formula_context(&self) -> &RelativeFormulaEvaluationContext {
    self.relative_formula_context.get_or_init(|| {
      RelativeFormulaEvaluationContext::from_import(
        &self.sheets,
        &self.defined_names,
        &self.workbook_catalog,
        &self.formula_context,
      )
    })
  }
}

fn spreadsheet_producer_profile(
  package: &SpreadsheetDocument,
  workbook: &ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook,
) -> SpreadsheetProducerProfile {
  let extended_properties_part = {
    package
      .get_parts_of_type::<
        ooxmlsdk::parts::extended_file_properties_part::ExtendedFilePropertiesPart,
      >()
      .next()
  };
  let properties = extended_properties_part
    .and_then(|part| part.root_element(package).ok())
    .cloned();
  let application = properties
    .as_ref()
    .and_then(|properties| properties.application.as_deref())
    .filter(|application| !application.trim().is_empty());
  let workbook_application = workbook
    .file_version
    .as_ref()
    .and_then(|version| version.application_name.as_deref());
  let excel_major_version = properties
    .as_ref()
    .and_then(|properties| properties.application_version.as_deref())
    .and_then(|version| version.split('.').next())
    .and_then(|major| major.parse::<u16>().ok());
  let lowest_edited_version = workbook
    .file_version
    .as_ref()
    .and_then(|version| version.lowest_edited.as_deref())
    .and_then(|version| version.parse::<u16>().ok());
  SpreadsheetProducerProfile {
    // Excel templates can omit the extended Application property while
    // retaining fileVersion@appName="xl". Use that workbook declaration when
    // the producer name is absent; a declared later producer still wins.
    mso_document: application.map_or(workbook_application == Some("xl"), |value| {
      value.contains("Microsoft")
    }),
    macintosh_excel: application.is_some_and(|value| value == "Microsoft Macintosh Excel"),
    excel_online: application.is_some_and(|value| value == "Microsoft Excel Online"),
    libreoffice_document: application.is_some_and(|value| value.contains("LibreOffice")),
    excel_major_version,
    lowest_edited_version,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Workbook;
  use ooxmlsdk::sdk::{SdkType, SpreadsheetDocumentType};

  #[test]
  fn workbook_application_identifies_excel_only_without_a_declared_producer() {
    let workbook = Workbook::from_bytes(br#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><fileVersion appName="xl" lowestEdited="4"/><sheets/></workbook>"#).unwrap();
    for (application, expected_excel, expected_libreoffice) in [
      (None, true, false),
      (Some(""), true, false),
      (Some("Microsoft Excel"), true, false),
      (Some("LibreOffice"), false, true),
      (Some("Another Producer"), false, false),
    ] {
      let mut package = SpreadsheetDocument::create(SpreadsheetDocumentType::Workbook);
      if let Some(application) = application {
        let part = package.add_extended_file_properties_part().unwrap();
        part.set_data(&mut package, format!(r#"<Properties xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"><Application>{application}</Application><AppVersion>12.0000</AppVersion></Properties>"#).into_bytes()).unwrap();
      }
      let profile = spreadsheet_producer_profile(&package, &workbook);
      assert_eq!(profile.mso_document, expected_excel, "{application:?}");
      assert_eq!(profile.libreoffice_document, expected_libreoffice);
      assert_eq!(profile.lowest_edited_version, Some(4));
    }
    let package = SpreadsheetDocument::create(SpreadsheetDocumentType::Workbook);
    let unknown = Workbook::from_bytes(br#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main"><sheets/></workbook>"#).unwrap();
    assert!(!spreadsheet_producer_profile(&package, &unknown).mso_document);
  }
}
