use ooxmlsdk::parts::chartsheet_part::ChartsheetPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

use crate::error::Result;

use super::styles::{DefinedNamesCatalog, StylesCatalog};
use super::worksheet::{CalcSheet, SheetResourceCatalog};

#[derive(Debug)]
pub(crate) struct WorkbookFragment {
  workbook_part: WorkbookPart,
  workbook: x::Workbook,
  pub(crate) shared_strings: Vec<String>,
  pub(crate) styles: StylesCatalog,
  pub(crate) defined_names: DefinedNamesCatalog,
}

impl WorkbookFragment {
  pub(crate) fn new(workbook_part: WorkbookPart, workbook: x::Workbook) -> Self {
    Self {
      workbook_part,
      workbook,
      shared_strings: Vec::new(),
      styles: StylesCatalog::default(),
      defined_names: DefinedNamesCatalog::default(),
    }
  }

  pub(crate) fn finalize_import(
    &mut self,
    package: &mut SpreadsheetDocument,
  ) -> Result<Vec<CalcSheet>> {
    // Source: LibreOffice sc/source/filter/oox/workbookfragment.cxx
    // WorkbookFragment::finalizeImport imports theme/styles/shared strings
    // before creating all sheet globals/fragments in workbook sheet order.
    self.styles = StylesCatalog::from_workbook_part(package, &self.workbook_part)?;
    self.shared_strings = shared_strings(package, &self.workbook_part)?;
    self.defined_names = DefinedNamesCatalog::from_workbook(&self.workbook);

    let worksheet_parts = self
      .workbook_part
      .worksheet_parts(package)
      .collect::<Vec<_>>();
    let chartsheet_parts = self
      .workbook_part
      .chartsheet_parts(package)
      .collect::<Vec<_>>();
    let active_workbook_sheet = active_workbook_sheet(&self.workbook);

    self
      .workbook
      .sheets
      .sheet
      .iter()
      .enumerate()
      .map(|(workbook_index, sheet)| {
        let rel_id = sheet.id.as_str();
        let state = sheet.state;
        if let Some(part) = worksheet_parts
          .iter()
          .find(|part| part.relationship_id() == Some(rel_id))
        {
          return worksheet_sheet(
            package,
            part,
            sheet,
            workbook_index,
            state,
            active_workbook_sheet == Some(workbook_index),
            &self.shared_strings,
          );
        }

        if let Some(part) = chartsheet_parts
          .iter()
          .find(|part| part.relationship_id() == Some(rel_id))
        {
          return chartsheet(
            package,
            part,
            sheet,
            workbook_index,
            state,
            active_workbook_sheet == Some(workbook_index),
          );
        }

        Ok(CalcSheet::unresolved(
          workbook_index,
          sheet.name.as_str().to_string(),
          sheet.sheet_id,
          rel_id.to_string(),
          state,
          active_workbook_sheet == Some(workbook_index),
        ))
      })
      .collect()
  }
}

fn worksheet_sheet(
  package: &mut SpreadsheetDocument,
  part: &WorksheetPart,
  sheet: &x::Sheet,
  workbook_index: usize,
  state: Option<x::SheetStateValues>,
  active: bool,
  shared_strings: &[String],
) -> Result<CalcSheet> {
  let worksheet = part.root_element(package)?.clone();
  let resources = SheetResourceCatalog::from_worksheet_part(package, part)?;
  Ok(CalcSheet::from_worksheet(
    workbook_index,
    sheet.name.as_str().to_string(),
    sheet.sheet_id,
    sheet.id.as_str().to_string(),
    state,
    active,
    worksheet,
    resources,
    shared_strings,
  ))
}

fn chartsheet(
  package: &mut SpreadsheetDocument,
  part: &ChartsheetPart,
  sheet: &x::Sheet,
  workbook_index: usize,
  state: Option<x::SheetStateValues>,
  active: bool,
) -> Result<CalcSheet> {
  let chartsheet = part.root_element(package)?.clone();
  let resources = SheetResourceCatalog::from_chartsheet_part(package, part)?;
  Ok(CalcSheet::from_chartsheet(
    workbook_index,
    sheet.name.as_str().to_string(),
    sheet.sheet_id,
    sheet.id.as_str().to_string(),
    state,
    active,
    chartsheet,
    resources,
  ))
}

fn shared_strings(
  package: &mut SpreadsheetDocument,
  workbook_part: &WorkbookPart,
) -> Result<Vec<String>> {
  let Some(shared_string_part) = workbook_part.shared_string_table_part(package) else {
    return Ok(Vec::new());
  };
  let table = shared_string_part.root_element(package)?;
  Ok(
    table
      .shared_string_item
      .iter()
      .map(shared_string_item_text)
      .collect(),
  )
}

fn shared_string_item_text(item: &x::SharedStringItem) -> String {
  if let Some(text) = &item.text
    && let Some(content) = &text.xml_content
  {
    return content.to_string();
  }

  item
    .run
    .iter()
    .filter_map(|run| run.text.xml_content.as_deref())
    .collect()
}

fn active_workbook_sheet(workbook: &x::Workbook) -> Option<usize> {
  super::workbook_settings::WorkbookGlobals::from_workbook(workbook)
    .active_tab()
    .map(|index| index as usize)
}
