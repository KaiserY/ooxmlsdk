use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CalcPageSettings {
  pub(crate) has_margins: bool,
  pub(crate) margin_left_in: f64,
  pub(crate) margin_right_in: f64,
  pub(crate) margin_top_in: f64,
  pub(crate) margin_bottom_in: f64,
  pub(crate) margin_header_in: f64,
  pub(crate) margin_footer_in: f64,
  pub(crate) paper_size: u32,
  pub(crate) scale: u32,
  pub(crate) fit_to_width: u32,
  pub(crate) fit_to_height: u32,
  pub(crate) horizontal_dpi: u32,
  pub(crate) vertical_dpi: u32,
  pub(crate) page_order: Option<x::PageOrderValues>,
  pub(crate) orientation: Option<x::OrientationValues>,
  pub(crate) horizontal_centered: bool,
  pub(crate) vertical_centered: bool,
  pub(crate) print_headings: bool,
  pub(crate) print_grid_lines: bool,
}

impl Default for CalcPageSettings {
  fn default() -> Self {
    // Source: LibreOffice sc/source/filter/oox/pagesettings.cxx
    // PageSettingsModel defaults. Keep these in the PageSettings owner.
    Self {
      has_margins: false,
      margin_left_in: 0.748,
      margin_right_in: 0.748,
      margin_top_in: 0.984,
      margin_bottom_in: 0.984,
      margin_header_in: 0.512,
      margin_footer_in: 0.512,
      paper_size: 1,
      scale: 100,
      fit_to_width: 1,
      fit_to_height: 1,
      horizontal_dpi: 600,
      vertical_dpi: 600,
      page_order: Some(x::PageOrderValues::DownThenOver),
      orientation: None,
      horizontal_centered: false,
      vertical_centered: false,
      print_headings: false,
      print_grid_lines: false,
    }
  }
}

impl CalcPageSettings {
  pub(crate) fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    let mut settings = Self::default();
    if let Some(margins) = &worksheet.page_margins {
      settings.apply_margins(margins);
    }
    if let Some(page_setup) = &worksheet.page_setup {
      settings.apply_page_setup(page_setup);
    }
    if let Some(print_options) = &worksheet.print_options {
      settings.apply_print_options(print_options);
    }
    settings
  }

  pub(crate) fn from_chartsheet(chartsheet: &x::Chartsheet) -> Self {
    let mut settings = Self::default();
    if let Some(margins) = &chartsheet.page_margins {
      settings.apply_margins(margins);
    }
    if let Some(page_setup) = &chartsheet.chart_sheet_page_setup {
      settings.paper_size = page_setup.paper_size.unwrap_or(settings.paper_size);
      settings.orientation = page_setup.orientation;
      settings.horizontal_dpi = page_setup.horizontal_dpi.unwrap_or(settings.horizontal_dpi);
      settings.vertical_dpi = page_setup.vertical_dpi.unwrap_or(settings.vertical_dpi);
    }
    settings
  }

  fn apply_margins(&mut self, margins: &x::PageMargins) {
    self.has_margins = true;
    self.margin_left_in = margins.left;
    self.margin_right_in = margins.right;
    self.margin_top_in = margins.top;
    self.margin_bottom_in = margins.bottom;
    self.margin_header_in = margins.header;
    self.margin_footer_in = margins.footer;
  }

  fn apply_page_setup(&mut self, page_setup: &x::PageSetup) {
    self.paper_size = page_setup.paper_size.unwrap_or(self.paper_size);
    self.scale = page_setup.scale.unwrap_or(self.scale);
    self.fit_to_width = page_setup.fit_to_width.unwrap_or(self.fit_to_width);
    self.fit_to_height = page_setup.fit_to_height.unwrap_or(self.fit_to_height);
    self.horizontal_dpi = page_setup.horizontal_dpi.unwrap_or(self.horizontal_dpi);
    self.vertical_dpi = page_setup.vertical_dpi.unwrap_or(self.vertical_dpi);
    self.page_order = page_setup.page_order.or(self.page_order);
    self.orientation = page_setup.orientation;
  }

  fn apply_print_options(&mut self, print_options: &x::PrintOptions) {
    self.horizontal_centered = print_options
      .horizontal_centered
      .is_some_and(|value| value.as_bool());
    self.vertical_centered = print_options
      .vertical_centered
      .is_some_and(|value| value.as_bool());
    self.print_headings = print_options.headings.is_some_and(|value| value.as_bool());
    self.print_grid_lines = print_options
      .grid_lines
      .is_some_and(|value| value.as_bool());
  }
}
