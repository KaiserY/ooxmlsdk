use ooxmlsdk::parts::chartsheet_part::ChartsheetPart;
use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::parts::worksheet_part::WorksheetPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;
use ooxmlsdk::sdk::SdkPart;

use crate::error::Result;
use crate::model::RgbColor;

use super::styles::{DefinedNamesCatalog, StylesCatalog};
use super::text::decode_excel_escaped_text;
use super::worksheet::{
  CalcSheet, SheetIdentity, SheetResourceCatalog, WorksheetResourceImportContext,
};

#[derive(Debug)]
pub(crate) struct WorkbookFragment {
  workbook_part: WorkbookPart,
  workbook: x::Workbook,
  pub(crate) shared_strings: Vec<SharedStringModel>,
  pub(crate) styles: StylesCatalog,
  pub(crate) defined_names: DefinedNamesCatalog,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SharedStringModel {
  pub(crate) text: String,
  pub(crate) runs: Vec<SharedStringRun>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SharedStringRun {
  pub(crate) text: String,
  pub(crate) font_size_pt: Option<f32>,
  pub(crate) color: Option<RgbColor>,
  pub(crate) bold: bool,
  pub(crate) italic: bool,
  pub(crate) underline: bool,
  pub(crate) strikethrough: bool,
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
    mso_document: bool,
  ) -> Result<Vec<CalcSheet>> {
    // WorkbookFragment::finalizeImport imports theme/styles/shared strings
    // before creating all sheet globals/fragments in workbook sheet order.
    self.styles = StylesCatalog::from_workbook_part(package, &self.workbook_part)?;
    self.shared_strings = shared_strings(package, &self.workbook_part)?;
    self.defined_names = DefinedNamesCatalog::from_workbook(&self.workbook);
    let date_1904 = self
      .workbook
      .workbook_properties
      .as_ref()
      .and_then(|properties| properties.date1904)
      .is_some_and(|value| value.as_bool());

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
            WorkbookSheetContext {
              workbook_index,
              state,
              active: active_workbook_sheet == Some(workbook_index),
              shared_strings: &self.shared_strings,
              styles: &self.styles,
              date_1904,
              mso_document,
            },
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

        Ok(CalcSheet::unresolved(SheetIdentity {
          workbook_index,
          name: sheet.name.as_str().to_string(),
          state,
          active: active_workbook_sheet == Some(workbook_index),
        }))
      })
      .collect()
  }
}

struct WorkbookSheetContext<'a> {
  workbook_index: usize,
  state: Option<x::SheetStateValues>,
  active: bool,
  shared_strings: &'a [SharedStringModel],
  styles: &'a StylesCatalog,
  date_1904: bool,
  mso_document: bool,
}

fn worksheet_sheet(
  package: &mut SpreadsheetDocument,
  part: &WorksheetPart,
  sheet: &x::Sheet,
  context: WorkbookSheetContext<'_>,
) -> Result<CalcSheet> {
  let worksheet = part.root_element(package)?.clone();
  let resources = SheetResourceCatalog::from_worksheet_part(
    package,
    part,
    WorksheetResourceImportContext {
      sheet_name: sheet.name.as_str(),
      worksheet: &worksheet,
      shared_strings: context.shared_strings,
      styles: context.styles,
      date_1904: context.date_1904,
    },
  )?;
  let sheet = CalcSheet::from_worksheet(
    SheetIdentity {
      workbook_index: context.workbook_index,
      name: sheet.name.as_str().to_string(),
      state: context.state,
      active: context.active,
    },
    worksheet,
    resources,
    context.shared_strings,
    context.styles,
    context.mso_document,
  );
  Ok(sheet)
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
    SheetIdentity {
      workbook_index,
      name: sheet.name.as_str().to_string(),
      state,
      active,
    },
    chartsheet,
    resources,
  ))
}

fn shared_strings(
  package: &mut SpreadsheetDocument,
  workbook_part: &WorkbookPart,
) -> Result<Vec<SharedStringModel>> {
  let Some(shared_string_part) = workbook_part.shared_string_table_part(package) else {
    return Ok(Vec::new());
  };
  let table = shared_string_part.root_element(package)?;
  Ok(
    table
      .shared_string_item
      .iter()
      .map(shared_string_item_model)
      .collect(),
  )
}

fn shared_string_item_text(item: &x::SharedStringItem) -> String {
  if let Some(text) = &item.text
    && let Some(content) = &text.xml_content
  {
    return decode_excel_escaped_text(content);
  }

  decode_excel_escaped_text(
    &item
      .run
      .iter()
      .filter_map(|run| run.text.xml_content.as_deref())
      .collect::<String>(),
  )
}

fn shared_string_item_model(item: &x::SharedStringItem) -> SharedStringModel {
  let text = shared_string_item_text(item);
  let runs = item.run.iter().map(shared_string_run).collect::<Vec<_>>();
  SharedStringModel { text, runs }
}

fn shared_string_run(run: &x::Run) -> SharedStringRun {
  let mut model = SharedStringRun {
    text: run
      .text
      .xml_content
      .as_deref()
      .map(decode_excel_escaped_text)
      .unwrap_or_default(),
    ..SharedStringRun::default()
  };
  if let Some(properties) = &run.run_properties {
    for choice in &properties.run_properties_choice {
      match choice {
        x::RunPropertiesChoice::Bold(value) => {
          model.bold = value.val.is_none_or(|value| value.as_bool());
        }
        x::RunPropertiesChoice::Italic(value) => {
          model.italic = value.val.is_none_or(|value| value.as_bool());
        }
        x::RunPropertiesChoice::Strike(value) => {
          model.strikethrough = value.val.is_none_or(|value| value.as_bool());
        }
        x::RunPropertiesChoice::Underline(value) => {
          model.underline = !matches!(value.val, Some(x::UnderlineValues::None));
        }
        x::RunPropertiesChoice::FontSize(value) => {
          model.font_size_pt = Some(value.val as f32);
        }
        x::RunPropertiesChoice::Color(value) => {
          model.color = run_color(value);
        }
        _ => {}
      }
    }
  }
  model
}

fn run_color(color: &x::Color) -> Option<RgbColor> {
  let rgb = color.rgb.as_deref()?;
  let value = rgb.strip_prefix('#').unwrap_or(rgb);
  let value = if value.len() == 8 { &value[2..] } else { value };
  if value.len() != 6 {
    return None;
  }
  Some(RgbColor {
    r: u8::from_str_radix(&value[0..2], 16).ok()?,
    g: u8::from_str_radix(&value[2..4], 16).ok()?,
    b: u8::from_str_radix(&value[4..6], 16).ok()?,
  })
}

fn active_workbook_sheet(workbook: &x::Workbook) -> Option<usize> {
  Some(
    super::workbook_settings::WorkbookGlobals::from_workbook(workbook)
      .active_tab()
      .map(|index| index as usize)
      .unwrap_or(0),
  )
}
