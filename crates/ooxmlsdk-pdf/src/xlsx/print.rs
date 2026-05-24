use super::import::ExcelImport;
use super::page_settings::CalcPageSettings;
use super::styles::{DefinedNameBuiltin, DefinedNameRecord};
use super::worksheet::CalcSheet;

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintDocument<'a> {
  pub(crate) pages: Vec<CalcPrintPage<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct CalcPrintPage<'a> {
  pub(crate) sheet: &'a CalcSheet,
  pub(crate) sheet_page_index: usize,
  pub(crate) page_number: usize,
  pub(crate) zoom: u32,
  pub(crate) page_settings: &'a CalcPageSettings,
  pub(crate) named_ranges: CalcPrintNamedRanges<'a>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CalcPrintNamedRanges<'a> {
  pub(crate) print_areas: Vec<&'a DefinedNameRecord>,
  pub(crate) print_titles: Vec<&'a DefinedNameRecord>,
  pub(crate) filter_databases: Vec<&'a DefinedNameRecord>,
}

impl<'a> CalcPrintDocument<'a> {
  pub(crate) fn from_import(import: &'a ExcelImport) -> Self {
    // Source: LibreOffice sc/source/ui/view/printfun.cxx
    // This is the first ScPrintFunc-shaped owner. Full range, break, and page
    // count logic lands here; display only consumes the resulting print pages.
    let pages = import
      .sheets
      .iter()
      .filter(|sheet| sheet.visible())
      .enumerate()
      .map(|(page_index, sheet)| CalcPrintPage {
        sheet,
        sheet_page_index: 0,
        page_number: page_index + 1,
        zoom: sheet.page_settings.scale,
        page_settings: &sheet.page_settings,
        named_ranges: CalcPrintNamedRanges::from_import(import, sheet),
      })
      .collect();
    Self { pages }
  }
}

impl<'a> CalcPrintNamedRanges<'a> {
  fn from_import(import: &'a ExcelImport, sheet: &CalcSheet) -> Self {
    // Source: LibreOffice sc/source/filter/oox/defnamesbuffer.cxx
    // DefinedName::convertFormula extracts print areas, repeated titles, and
    // filter database ranges after the defined-name formula token model exists.
    // Keep the built-in defined names attached to the ScPrintFunc owner; range
    // extraction belongs here once the Calc formula parser lands.
    Self {
      print_areas: import
        .defined_names
        .records_for_sheet(sheet.workbook_index, DefinedNameBuiltin::PrintArea),
      print_titles: import
        .defined_names
        .records_for_sheet(sheet.workbook_index, DefinedNameBuiltin::PrintTitles),
      filter_databases: import
        .defined_names
        .records_for_sheet(sheet.workbook_index, DefinedNameBuiltin::FilterDatabase),
    }
  }

  pub(crate) fn unresolved_formula_count(&self) -> usize {
    self
      .print_areas
      .iter()
      .chain(&self.print_titles)
      .chain(&self.filter_databases)
      .filter(|record| !record.formula.is_empty())
      .count()
  }
}
