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
  pub(crate) header_footer: HeaderFooterModel,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct HeaderFooterModel {
  pub(crate) different_odd_even: bool,
  pub(crate) different_first: bool,
  pub(crate) scale_with_doc: bool,
  pub(crate) align_with_margins: bool,
  pub(crate) odd_header: Option<String>,
  pub(crate) odd_footer: Option<String>,
  pub(crate) even_header: Option<String>,
  pub(crate) even_footer: Option<String>,
  pub(crate) first_header: Option<String>,
  pub(crate) first_footer: Option<String>,
  pub(crate) legacy_drawing_relationship_id: Option<String>,
  pub(crate) drawing_relationship_id: Option<String>,
  pub(crate) drawing_slot_count: usize,
  pub(crate) background_picture_relationship_id: Option<String>,
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
      header_footer: HeaderFooterModel::default(),
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
    settings.header_footer = HeaderFooterModel::from_worksheet(worksheet);
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
    settings.header_footer = HeaderFooterModel::from_chartsheet(chartsheet);
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

impl HeaderFooterModel {
  fn from_worksheet(worksheet: &x::Worksheet) -> Self {
    let mut model = worksheet
      .header_footer
      .as_deref()
      .map(Self::from_header_footer)
      .unwrap_or_default();
    model.legacy_drawing_relationship_id = worksheet
      .legacy_drawing_header_footer
      .as_ref()
      .map(|drawing| drawing.id.clone());
    model.apply_drawing_header_footer(worksheet.drawing_header_footer.as_ref());
    model.background_picture_relationship_id =
      worksheet.picture.as_ref().map(|picture| picture.id.clone());
    model
  }

  fn from_chartsheet(chartsheet: &x::Chartsheet) -> Self {
    let mut model = chartsheet
      .header_footer
      .as_deref()
      .map(Self::from_header_footer)
      .unwrap_or_default();
    model.legacy_drawing_relationship_id = chartsheet
      .legacy_drawing_header_footer
      .as_ref()
      .map(|drawing| drawing.id.clone());
    model.apply_drawing_header_footer(chartsheet.drawing_header_footer.as_ref());
    model.background_picture_relationship_id = chartsheet
      .picture
      .as_ref()
      .map(|picture| picture.id.clone());
    model
  }

  fn from_header_footer(header_footer: &x::HeaderFooter) -> Self {
    // Source: LibreOffice sc/source/filter/oox/pagesettings.cxx
    // HeaderFooterParser tokenizes these strings later; page settings owns the
    // six text channels and picture relationship state.
    Self {
      different_odd_even: header_footer
        .different_odd_even
        .is_some_and(|value| value.as_bool()),
      different_first: header_footer
        .different_first
        .is_some_and(|value| value.as_bool()),
      scale_with_doc: header_footer
        .scale_with_doc
        .is_some_and(|value| value.as_bool()),
      align_with_margins: header_footer
        .align_with_margins
        .is_some_and(|value| value.as_bool()),
      odd_header: header_footer
        .odd_header
        .as_ref()
        .and_then(|value| value.0.xml_content.clone()),
      odd_footer: header_footer
        .odd_footer
        .as_ref()
        .and_then(|value| value.0.xml_content.clone()),
      even_header: header_footer
        .even_header
        .as_ref()
        .and_then(|value| value.0.xml_content.clone()),
      even_footer: header_footer
        .even_footer
        .as_ref()
        .and_then(|value| value.0.xml_content.clone()),
      first_header: header_footer
        .first_header
        .as_ref()
        .and_then(|value| value.0.xml_content.clone()),
      first_footer: header_footer
        .first_footer
        .as_ref()
        .and_then(|value| value.0.xml_content.clone()),
      ..Self::default()
    }
  }

  fn apply_drawing_header_footer(&mut self, drawing: Option<&x::DrawingHeaderFooter>) {
    if let Some(drawing) = drawing {
      self.drawing_relationship_id = Some(drawing.r_id.clone());
      self.drawing_slot_count = [
        drawing.lho,
        drawing.lhe,
        drawing.lhf,
        drawing.cho,
        drawing.che,
        drawing.chf,
        drawing.rho,
        drawing.rhe,
        drawing.rhf,
        drawing.lfo,
        drawing.lfe,
        drawing.lff,
        drawing.cfo,
        drawing.cfe,
        drawing.cff,
        drawing.rfo,
        drawing.rfe,
        drawing.rff,
      ]
      .into_iter()
      .flatten()
      .count();
    }
  }

  pub(crate) fn text_len(&self) -> usize {
    self.odd_header.as_ref().map_or(0, |value| value.len())
      + self.odd_footer.as_ref().map_or(0, |value| value.len())
      + self.even_header.as_ref().map_or(0, |value| value.len())
      + self.even_footer.as_ref().map_or(0, |value| value.len())
      + self.first_header.as_ref().map_or(0, |value| value.len())
      + self.first_footer.as_ref().map_or(0, |value| value.len())
  }

  pub(crate) fn flag_count(&self) -> usize {
    usize::from(self.different_odd_even)
      + usize::from(self.different_first)
      + usize::from(self.scale_with_doc)
      + usize::from(self.align_with_margins)
      + usize::from(self.legacy_drawing_relationship_id.is_some())
      + usize::from(self.drawing_relationship_id.is_some())
      + usize::from(self.background_picture_relationship_id.is_some())
  }
}
