use ooxmlsdk::parts::spreadsheet_document::SpreadsheetDocument;
use ooxmlsdk::parts::workbook_part::WorkbookPart;
use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::error::Result;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct StylesCatalog {
  pub(crate) custom_number_formats: Vec<NumberFormatRecord>,
  pub(crate) fonts: usize,
  pub(crate) fills: usize,
  pub(crate) borders: usize,
  pub(crate) cell_style_formats: usize,
  pub(crate) cell_formats: usize,
  pub(crate) cell_styles: usize,
  pub(crate) differential_formats: usize,
  pub(crate) table_styles: usize,
  pub(crate) default_table_style: Option<String>,
  pub(crate) default_pivot_style: Option<String>,
  pub(crate) has_colors: bool,
  pub(crate) has_extensions: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NumberFormatRecord {
  pub(crate) id: u32,
  pub(crate) code: String,
}

impl StylesCatalog {
  pub(crate) fn from_workbook_part(
    package: &mut SpreadsheetDocument,
    workbook_part: &WorkbookPart,
  ) -> Result<Self> {
    let Some(styles_part) = workbook_part.workbook_styles_part(package) else {
      return Ok(Self::default());
    };

    let stylesheet = styles_part.root_element(package)?;
    Ok(Self::from_stylesheet(stylesheet))
  }

  fn from_stylesheet(stylesheet: &x::Stylesheet) -> Self {
    let table_styles = stylesheet.table_styles.as_ref();
    Self {
      custom_number_formats: stylesheet
        .numbering_formats
        .as_ref()
        .map(|formats| {
          formats
            .numbering_format
            .iter()
            .map(|format| NumberFormatRecord {
              id: format.number_format_id,
              code: format.format_code.clone(),
            })
            .collect()
        })
        .unwrap_or_default(),
      fonts: stylesheet
        .fonts
        .as_ref()
        .map_or(0, |fonts| fonts.font.len()),
      fills: stylesheet
        .fills
        .as_ref()
        .map_or(0, |fills| fills.fill.len()),
      borders: stylesheet
        .borders
        .as_ref()
        .map_or(0, |borders| borders.border.len()),
      cell_style_formats: stylesheet
        .cell_style_formats
        .as_ref()
        .map_or(0, |formats| formats.cell_format.len()),
      cell_formats: stylesheet
        .cell_formats
        .as_ref()
        .map_or(0, |formats| formats.cell_format.len()),
      cell_styles: stylesheet
        .cell_styles
        .as_ref()
        .map_or(0, |styles| styles.cell_style.len()),
      differential_formats: stylesheet
        .differential_formats
        .as_ref()
        .map_or(0, |formats| formats.differential_format.len()),
      table_styles: table_styles.map_or(0, |styles| styles.table_style.len()),
      default_table_style: table_styles.and_then(|styles| styles.default_table_style.clone()),
      default_pivot_style: table_styles.and_then(|styles| styles.default_pivot_style.clone()),
      has_colors: stylesheet.colors.is_some(),
      has_extensions: stylesheet.stylesheet_extension_list.is_some(),
    }
  }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DefinedNamesCatalog {
  pub(crate) records: Vec<DefinedNameRecord>,
  pub(crate) print_areas: usize,
  pub(crate) print_titles: usize,
  pub(crate) filter_databases: usize,
  pub(crate) local_names: usize,
  pub(crate) hidden_names: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DefinedNameRecord {
  pub(crate) name: String,
  pub(crate) builtin: Option<DefinedNameBuiltin>,
  pub(crate) local_sheet_id: Option<u32>,
  pub(crate) hidden: bool,
  pub(crate) formula: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum DefinedNameBuiltin {
  PrintArea,
  PrintTitles,
  FilterDatabase,
}

impl DefinedNamesCatalog {
  pub(crate) fn from_workbook(workbook: &x::Workbook) -> Self {
    let Some(defined_names) = &workbook.defined_names else {
      return Self::default();
    };

    let records = defined_names
      .defined_name
      .iter()
      .map(|name| DefinedNameRecord {
        name: name.name.clone(),
        builtin: defined_name_builtin(name.name.as_str()),
        local_sheet_id: name.local_sheet_id,
        hidden: name.hidden.map(|hidden| hidden.as_bool()).unwrap_or(false),
        formula: name.xml_content.clone().unwrap_or_default(),
      })
      .collect::<Vec<_>>();

    let mut catalog = Self {
      records,
      ..Self::default()
    };
    for record in &catalog.records {
      match record.builtin {
        Some(DefinedNameBuiltin::PrintArea) => catalog.print_areas += 1,
        Some(DefinedNameBuiltin::PrintTitles) => catalog.print_titles += 1,
        Some(DefinedNameBuiltin::FilterDatabase) => catalog.filter_databases += 1,
        _ => {}
      }
      if record.local_sheet_id.is_some() {
        catalog.local_names += 1;
      }
      if record.hidden {
        catalog.hidden_names += 1;
      }
    }
    catalog
  }

  pub(crate) fn records_for_sheet(
    &self,
    sheet_index: usize,
    builtin: DefinedNameBuiltin,
  ) -> Vec<&DefinedNameRecord> {
    self
      .records
      .iter()
      .filter(|record| {
        record.builtin == Some(builtin) && record.local_sheet_id == Some(sheet_index as u32)
      })
      .collect()
  }
}

fn defined_name_builtin(name: &str) -> Option<DefinedNameBuiltin> {
  let base = name
    .strip_prefix("_xlnm.")
    .or_else(|| name.strip_prefix("_XLNM."))
    .unwrap_or(name);
  if base.eq_ignore_ascii_case("Print_Area") {
    Some(DefinedNameBuiltin::PrintArea)
  } else if base.eq_ignore_ascii_case("Print_Titles") {
    Some(DefinedNameBuiltin::PrintTitles)
  } else if base.eq_ignore_ascii_case("_FilterDatabase") {
    Some(DefinedNameBuiltin::FilterDatabase)
  } else {
    None
  }
}
