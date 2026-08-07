use ooxmlsdk::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main as x;

use crate::units;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PaperMeasure {
  Inches(f32, f32),
  Millimeters(f32, f32),
  Undefined,
}

/// Microsoft `paperSize` identifiers used by the fixed-output fallback path.
///
/// ECMA-376 §18.3.1.63 stores the identifier, while LibreOffice's
/// `filter/source/msfilter/util.cxx::spPaperSizeTable` supplies the dimensions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
enum MsPaperSize {
  Letter = 1,
  A4 = 9,
}

const DEFAULT_PRINT_SCALE_PERCENT: u32 = 100;
const OFFICE_LETTER_TO_DEFAULT_A4_SCALE_PERCENT: u32 = 95;

// [MS-RPRN] 2.2.2.1 describes the public DEVMODEW prefix written by Office
// into the Printer Settings part ([MS-OE376] 2.1.36). Driver-private bytes may
// follow dmSize, but none of them are needed to recover the portable media,
// orientation, scale, or resolution fields below.
const DEVMODEW_SPEC_VERSION_OFFSET: usize = 64;
const DEVMODEW_SIZE_OFFSET: usize = 68;
const DEVMODEW_DRIVER_EXTRA_OFFSET: usize = 70;
const DEVMODEW_FIELDS_OFFSET: usize = 72;
const DEVMODEW_ORIENTATION_OFFSET: usize = 76;
const DEVMODEW_PAPER_SIZE_OFFSET: usize = 78;
const DEVMODEW_PAPER_LENGTH_OFFSET: usize = 80;
const DEVMODEW_PAPER_WIDTH_OFFSET: usize = 82;
const DEVMODEW_SCALE_OFFSET: usize = 84;
const DEVMODEW_PRINT_QUALITY_OFFSET: usize = 90;
const DEVMODEW_Y_RESOLUTION_OFFSET: usize = 96;
const DEVMODEW_MIN_PRINTER_SIZE: usize = 102;

const DM_ORIENTATION: u32 = 0x0000_0001;
const DM_PAPER_SIZE: u32 = 0x0000_0002;
const DM_PAPER_LENGTH: u32 = 0x0000_0004;
const DM_PAPER_WIDTH: u32 = 0x0000_0008;
const DM_SCALE: u32 = 0x0000_0010;
const DM_PRINT_QUALITY: u32 = 0x0000_0400;
const DM_Y_RESOLUTION: u32 = 0x0000_2000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DevModeByteOrder {
  Little,
  Big,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WindowsPrinterSettings {
  pub(crate) device_name: String,
  pub(crate) paper_size: Option<u32>,
  pub(crate) paper_length_tenth_mm: Option<u32>,
  pub(crate) paper_width_tenth_mm: Option<u32>,
  pub(crate) scale_percent: Option<u32>,
  pub(crate) orientation: Option<x::OrientationValues>,
  pub(crate) horizontal_dpi: Option<u32>,
  pub(crate) vertical_dpi: Option<u32>,
}

impl WindowsPrinterSettings {
  pub(crate) fn from_bytes(bytes: &[u8]) -> Option<Self> {
    let byte_order = [DevModeByteOrder::Little, DevModeByteOrder::Big]
      .into_iter()
      .find(|byte_order| devmode_public_and_private_sizes_are_valid(bytes, *byte_order))?;
    let fields = devmode_u32(bytes, DEVMODEW_FIELDS_OFFSET, byte_order)?;
    let positive_field = |flag, offset| {
      (fields & flag != 0)
        .then(|| devmode_i16(bytes, offset, byte_order))
        .flatten()
        .filter(|value| *value > 0)
        .map(|value| value as u32)
    };
    let orientation =
      positive_field(DM_ORIENTATION, DEVMODEW_ORIENTATION_OFFSET).and_then(|value| match value {
        1 => Some(x::OrientationValues::Portrait),
        2 => Some(x::OrientationValues::Landscape),
        _ => None,
      });

    Some(Self {
      device_name: devmode_utf16_string(&bytes[..DEVMODEW_SPEC_VERSION_OFFSET], byte_order),
      paper_size: positive_field(DM_PAPER_SIZE, DEVMODEW_PAPER_SIZE_OFFSET),
      paper_length_tenth_mm: positive_field(DM_PAPER_LENGTH, DEVMODEW_PAPER_LENGTH_OFFSET),
      paper_width_tenth_mm: positive_field(DM_PAPER_WIDTH, DEVMODEW_PAPER_WIDTH_OFFSET),
      scale_percent: positive_field(DM_SCALE, DEVMODEW_SCALE_OFFSET),
      orientation,
      horizontal_dpi: positive_field(DM_PRINT_QUALITY, DEVMODEW_PRINT_QUALITY_OFFSET),
      vertical_dpi: positive_field(DM_Y_RESOLUTION, DEVMODEW_Y_RESOLUTION_OFFSET),
    })
  }

  fn custom_paper_size_pt(&self) -> Option<(f32, f32)> {
    let width_mm = self.paper_width_tenth_mm? as f32 / 10.0;
    let height_mm = self.paper_length_tenth_mm? as f32 / 10.0;
    Some((
      units::millimeters_to_points(width_mm),
      units::millimeters_to_points(height_mm),
    ))
  }
}

fn devmode_public_and_private_sizes_are_valid(bytes: &[u8], byte_order: DevModeByteOrder) -> bool {
  let Some(dm_size) = devmode_u16(bytes, DEVMODEW_SIZE_OFFSET, byte_order).map(usize::from) else {
    return false;
  };
  let Some(driver_extra) =
    devmode_u16(bytes, DEVMODEW_DRIVER_EXTRA_OFFSET, byte_order).map(usize::from)
  else {
    return false;
  };
  dm_size >= DEVMODEW_MIN_PRINTER_SIZE
    && dm_size % 4 == 0
    && dm_size
      .checked_add(driver_extra)
      .is_some_and(|total| total <= bytes.len())
}

fn devmode_utf16_string(bytes: &[u8], byte_order: DevModeByteOrder) -> String {
  let units = bytes
    .chunks_exact(2)
    .map(|bytes| match byte_order {
      DevModeByteOrder::Little => u16::from_le_bytes([bytes[0], bytes[1]]),
      DevModeByteOrder::Big => u16::from_be_bytes([bytes[0], bytes[1]]),
    })
    .take_while(|value| *value != 0)
    .collect::<Vec<_>>();
  String::from_utf16_lossy(&units)
}

fn devmode_u16(bytes: &[u8], offset: usize, byte_order: DevModeByteOrder) -> Option<u16> {
  let bytes: [u8; 2] = bytes.get(offset..offset + 2)?.try_into().ok()?;
  Some(match byte_order {
    DevModeByteOrder::Little => u16::from_le_bytes(bytes),
    DevModeByteOrder::Big => u16::from_be_bytes(bytes),
  })
}

fn devmode_i16(bytes: &[u8], offset: usize, byte_order: DevModeByteOrder) -> Option<i16> {
  devmode_u16(bytes, offset, byte_order).map(|value| value as i16)
}

fn devmode_u32(bytes: &[u8], offset: usize, byte_order: DevModeByteOrder) -> Option<u32> {
  let bytes: [u8; 4] = bytes.get(offset..offset + 4)?.try_into().ok()?;
  Some(match byte_order {
    DevModeByteOrder::Little => u32::from_le_bytes(bytes),
    DevModeByteOrder::Big => u32::from_be_bytes(bytes),
  })
}

// LibreOffice filter/source/msfilter/util.cxx::spPaperSizeTable. Indices are
// Microsoft paperSize values.
const MS_PAPER_SIZE_TABLE: [PaperMeasure; 91] = [
  PaperMeasure::Undefined,
  PaperMeasure::Inches(8.5, 11.0),
  PaperMeasure::Inches(8.5, 11.0),
  PaperMeasure::Inches(11.0, 17.0),
  PaperMeasure::Inches(17.0, 11.0),
  PaperMeasure::Inches(8.5, 14.0),
  PaperMeasure::Inches(5.5, 8.5),
  PaperMeasure::Inches(7.25, 10.5),
  PaperMeasure::Millimeters(297.0, 420.0),
  PaperMeasure::Millimeters(210.0, 297.0),
  PaperMeasure::Millimeters(210.0, 297.0),
  PaperMeasure::Millimeters(148.0, 210.0),
  PaperMeasure::Millimeters(257.0, 364.0),
  PaperMeasure::Millimeters(182.0, 257.0),
  PaperMeasure::Inches(8.5, 13.0),
  PaperMeasure::Millimeters(215.0, 275.0),
  PaperMeasure::Inches(10.0, 14.0),
  PaperMeasure::Inches(11.0, 17.0),
  PaperMeasure::Inches(8.5, 11.0),
  PaperMeasure::Inches(3.875, 8.875),
  PaperMeasure::Inches(4.125, 9.5),
  PaperMeasure::Inches(4.5, 10.375),
  PaperMeasure::Inches(4.75, 11.0),
  PaperMeasure::Inches(5.0, 11.5),
  PaperMeasure::Inches(17.0, 22.0),
  PaperMeasure::Inches(22.0, 34.0),
  PaperMeasure::Inches(34.0, 44.0),
  PaperMeasure::Millimeters(110.0, 220.0),
  PaperMeasure::Millimeters(162.0, 229.0),
  PaperMeasure::Millimeters(324.0, 458.0),
  PaperMeasure::Millimeters(229.0, 324.0),
  PaperMeasure::Millimeters(114.0, 162.0),
  PaperMeasure::Millimeters(114.0, 229.0),
  PaperMeasure::Millimeters(250.0, 353.0),
  PaperMeasure::Millimeters(176.0, 250.0),
  PaperMeasure::Millimeters(176.0, 125.0),
  PaperMeasure::Millimeters(110.0, 230.0),
  PaperMeasure::Inches(3.875, 7.5),
  PaperMeasure::Inches(3.625, 6.5),
  PaperMeasure::Inches(14.875, 11.0),
  PaperMeasure::Inches(8.5, 12.0),
  PaperMeasure::Inches(8.5, 13.0),
  PaperMeasure::Millimeters(250.0, 353.0),
  PaperMeasure::Millimeters(200.0, 148.0),
  PaperMeasure::Inches(9.0, 11.0),
  PaperMeasure::Inches(10.0, 11.0),
  PaperMeasure::Inches(15.0, 11.0),
  PaperMeasure::Millimeters(220.0, 220.0),
  PaperMeasure::Undefined,
  PaperMeasure::Undefined,
  PaperMeasure::Inches(9.5, 12.0),
  PaperMeasure::Inches(9.5, 15.0),
  PaperMeasure::Inches(11.69, 18.0),
  PaperMeasure::Millimeters(235.0, 322.0),
  PaperMeasure::Inches(8.5, 11.0),
  PaperMeasure::Millimeters(210.0, 297.0),
  PaperMeasure::Inches(9.5, 12.0),
  PaperMeasure::Millimeters(227.0, 356.0),
  PaperMeasure::Millimeters(305.0, 487.0),
  PaperMeasure::Inches(8.5, 12.69),
  PaperMeasure::Millimeters(210.0, 330.0),
  PaperMeasure::Millimeters(148.0, 210.0),
  PaperMeasure::Millimeters(182.0, 257.0),
  PaperMeasure::Millimeters(322.0, 445.0),
  PaperMeasure::Millimeters(174.0, 235.0),
  PaperMeasure::Millimeters(201.0, 276.0),
  PaperMeasure::Millimeters(420.0, 594.0),
  PaperMeasure::Millimeters(297.0, 420.0),
  PaperMeasure::Millimeters(322.0, 445.0),
  PaperMeasure::Millimeters(200.0, 148.0),
  PaperMeasure::Millimeters(105.0, 148.0),
  PaperMeasure::Undefined,
  PaperMeasure::Undefined,
  PaperMeasure::Undefined,
  PaperMeasure::Undefined,
  PaperMeasure::Inches(11.0, 8.5),
  PaperMeasure::Millimeters(420.0, 297.0),
  PaperMeasure::Millimeters(297.0, 210.0),
  PaperMeasure::Millimeters(210.0, 148.0),
  PaperMeasure::Millimeters(364.0, 257.0),
  PaperMeasure::Millimeters(257.0, 182.0),
  PaperMeasure::Millimeters(148.0, 100.0),
  PaperMeasure::Millimeters(148.0, 200.0),
  PaperMeasure::Millimeters(148.0, 105.0),
  PaperMeasure::Undefined,
  PaperMeasure::Undefined,
  PaperMeasure::Undefined,
  PaperMeasure::Undefined,
  PaperMeasure::Millimeters(128.0, 182.0),
  PaperMeasure::Millimeters(182.0, 128.0),
  PaperMeasure::Inches(12.0, 11.0),
];

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
  pub(crate) explicit_paper_size: bool,
  requested_custom_paper_size_pt: Option<(f32, f32)>,
  implicit_microsoft_letter_canvas: bool,
  related_printer_canvas: bool,
  explicit_page_scale: bool,
  pub(crate) valid_printer_settings: bool,
  pub(crate) fit_to_page: bool,
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
    // Excel's application defaults when SpreadsheetML omits pageMargins.
    // These are the values used by Apache POI's XSSFSheet::newSheet and by
    // the ECMA-376 §18.3.1.62 example; explicit pageMargins still replace all
    // six values in apply_margins.
    Self {
      has_margins: false,
      margin_left_in: 0.7,
      margin_right_in: 0.7,
      margin_top_in: 0.75,
      margin_bottom_in: 0.75,
      margin_header_in: 0.3,
      margin_footer_in: 0.3,
      paper_size: MsPaperSize::Letter as u32,
      explicit_paper_size: false,
      requested_custom_paper_size_pt: None,
      implicit_microsoft_letter_canvas: false,
      related_printer_canvas: false,
      explicit_page_scale: false,
      valid_printer_settings: true,
      fit_to_page: false,
      scale: DEFAULT_PRINT_SCALE_PERCENT,
      fit_to_width: 1,
      fit_to_height: 1,
      horizontal_dpi: units::OFFICE_FIXED_OUTPUT_DPI as u32,
      vertical_dpi: units::OFFICE_FIXED_OUTPUT_DPI as u32,
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
  pub(crate) fn from_worksheet(
    worksheet: &x::Worksheet,
    microsoft_office: bool,
    printer_settings: Option<&WindowsPrinterSettings>,
  ) -> Self {
    let mut settings = Self::default();
    if let Some(margins) = &worksheet.page_margins {
      settings.apply_margins(margins);
    }
    if let Some(page_setup) = &worksheet.page_setup {
      settings.apply_page_setup(page_setup);
    }
    if let Some(printer_settings) = printer_settings {
      let setup = worksheet.page_setup.as_ref();
      settings.apply_windows_printer_settings(
        printer_settings,
        setup.is_some_and(|setup| setup.paper_size.is_some()),
        setup.is_some_and(|setup| setup.orientation.is_some()),
        setup.is_some_and(|setup| setup.scale.is_some()),
        setup.is_some_and(|setup| setup.horizontal_dpi.is_some()),
        setup.is_some_and(|setup| setup.vertical_dpi.is_some()),
      );
    }
    settings.fit_to_page = worksheet
      .sheet_properties
      .as_ref()
      .and_then(|properties| properties.page_setup_properties.as_ref())
      .and_then(|properties| properties.fit_to_page)
      .is_some_and(|value| value.as_bool());
    if let Some(print_options) = &worksheet.print_options {
      settings.apply_print_options(print_options);
    }
    settings.header_footer = HeaderFooterModel::from_worksheet(worksheet);
    // Microsoft Excel writes a pageSetup element for an initialized print
    // canvas but may omit both paperSize and the devMode printer-settings
    // relation. SpreadsheetML's paperSize default is Letter; Office fixed
    // output maps that uncalibrated canvas onto the active default page. A
    // worksheet with no pageSetup is the counterexample and retains the native
    // default page. This profile is emitted by both desktop and online Excel.
    settings.implicit_microsoft_letter_canvas = microsoft_office
      && worksheet
        .page_setup
        .as_ref()
        .is_some_and(|setup| setup.paper_size.is_none() && setup.id.is_none());
    settings
  }

  pub(crate) fn from_chartsheet(
    chartsheet: &x::Chartsheet,
    printer_settings: Option<&WindowsPrinterSettings>,
  ) -> Self {
    let mut settings = Self::default();
    if let Some(margins) = &chartsheet.page_margins {
      settings.apply_margins(margins);
    }
    if let Some(page_setup) = &chartsheet.chart_sheet_page_setup {
      settings.explicit_paper_size = page_setup.paper_size.is_some();
      settings.paper_size = page_setup.paper_size.unwrap_or(settings.paper_size);
      settings.valid_printer_settings = page_setup.id.is_none()
        && page_setup.paper_size == Some(MsPaperSize::Letter as u32)
        || page_setup
          .use_printer_defaults
          .is_some_and(|value| value.as_bool());
      settings.orientation = page_setup.orientation;
      settings.horizontal_dpi = page_setup.horizontal_dpi.unwrap_or(settings.horizontal_dpi);
      settings.vertical_dpi = page_setup.vertical_dpi.unwrap_or(settings.vertical_dpi);
    }
    if let Some(printer_settings) = printer_settings {
      let setup = chartsheet.chart_sheet_page_setup.as_ref();
      settings.apply_windows_printer_settings(
        printer_settings,
        setup.is_some_and(|setup| setup.paper_size.is_some()),
        setup.is_some_and(|setup| setup.orientation.is_some()),
        true,
        setup.is_some_and(|setup| setup.horizontal_dpi.is_some()),
        setup.is_some_and(|setup| setup.vertical_dpi.is_some()),
      );
    }
    // LibreOffice PageSettingsConverter treats a chart sheet with default
    // orientation (or invalid printer settings) as landscape.
    if !settings.valid_printer_settings || settings.orientation.is_none() {
      settings.orientation = Some(x::OrientationValues::Landscape);
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
    if let Some(paper_size) = page_setup.paper_size {
      self.paper_size = paper_size;
      self.explicit_paper_size = true;
    }
    // The Printer Settings part contains the printer initialization and
    // environment state, and pageSetup's r:id is its explicit relationship.
    // Office fixed-format export treats the schema/default Letter value as
    // uncalibrated and falls back to the active default paper when that
    // relationship is absent. Non-default explicit sizes remain
    // authoritative without a relationship.
    self.valid_printer_settings = page_setup.id.is_none()
      && page_setup.paper_size == Some(MsPaperSize::Letter as u32)
      || page_setup.paper_size.is_none()
      || page_setup
        .use_printer_defaults
        .is_some_and(|value| value.as_bool());
    if let Some(scale) = page_setup.scale.filter(|scale| *scale > 0) {
      self.scale = scale;
      self.explicit_page_scale = true;
    }
    self.fit_to_width = page_setup.fit_to_width.unwrap_or(self.fit_to_width);
    self.fit_to_height = page_setup.fit_to_height.unwrap_or(self.fit_to_height);
    self.horizontal_dpi = page_setup.horizontal_dpi.unwrap_or(self.horizontal_dpi);
    self.vertical_dpi = page_setup.vertical_dpi.unwrap_or(self.vertical_dpi);
    self.page_order = page_setup.page_order.or(self.page_order);
    self.orientation = page_setup.orientation;
  }

  fn apply_windows_printer_settings(
    &mut self,
    printer_settings: &WindowsPrinterSettings,
    explicit_paper_size: bool,
    explicit_orientation: bool,
    explicit_scale: bool,
    explicit_horizontal_dpi: bool,
    explicit_vertical_dpi: bool,
  ) {
    // SpreadsheetML owns any field it serializes explicitly. The related
    // Printer Settings part fills only omitted device defaults; [MS-RPRN]
    // requires receivers to ignore DEVMODE fields whose dmFields bit is clear.
    if !explicit_paper_size {
      if let Some(custom_size) = printer_settings.custom_paper_size_pt() {
        self.requested_custom_paper_size_pt = Some(custom_size);
        self.explicit_paper_size = true;
        self.related_printer_canvas = true;
      } else if let Some(paper_size) = printer_settings.paper_size {
        self.paper_size = paper_size;
        // This flag means that a concrete print canvas was requested, whether
        // by pageSetup@paperSize or by the related DEVMODE.
        self.explicit_paper_size = true;
        self.related_printer_canvas = true;
      }
    }
    if !explicit_orientation {
      self.orientation = printer_settings.orientation.or(self.orientation);
    }
    if !explicit_scale {
      self.scale = printer_settings.scale_percent.unwrap_or(self.scale);
    }
    if !explicit_horizontal_dpi {
      self.horizontal_dpi = printer_settings
        .horizontal_dpi
        .unwrap_or(self.horizontal_dpi);
    }
    if !explicit_vertical_dpi {
      self.vertical_dpi = printer_settings.vertical_dpi.unwrap_or(self.vertical_dpi);
    }
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

  pub(crate) fn page_size_pt(&self) -> (f32, f32) {
    // PageSettingsModel::mbValidSettings true until pageSetup says otherwise.
    // With valid printer defaults, PageSettingsConverter leaves PROP_Size
    // unchanged, so Calc printing uses the document style default. The LO test
    // profile default is A4.
    let paper_size = if self.valid_printer_settings {
      MsPaperSize::A4 as u32
    } else {
      self.paper_size
    };
    let requested_custom_size = (!self.valid_printer_settings)
      .then_some(self.requested_custom_paper_size_pt)
      .flatten();
    let mut size = if let Some(size) = requested_custom_size {
      size
    } else {
      match MS_PAPER_SIZE_TABLE
        .get(paper_size as usize)
        .copied()
        .unwrap_or(PaperMeasure::Undefined)
      {
        PaperMeasure::Inches(width, height) => (
          width * units::POINTS_PER_INCH,
          height * units::POINTS_PER_INCH,
        ),
        PaperMeasure::Millimeters(width, height) => (
          units::millimeters_to_points(width),
          units::millimeters_to_points(height),
        ),
        PaperMeasure::Undefined => {
          // PageSettingsConverter leaves PROP_Size unchanged for undefined or
          // invalid paper sizes, and sc/source/ui/view/printfun.cxx InitParam
          // falls back a null page size to PAPER_A4.
          let default = MS_PAPER_SIZE_TABLE[MsPaperSize::A4 as usize];
          match default {
            PaperMeasure::Millimeters(width, height) => (
              units::millimeters_to_points(width),
              units::millimeters_to_points(height),
            ),
            _ => unreachable!("Microsoft paper size 9 is A4"),
          }
        }
      }
    };
    if matches!(self.orientation, Some(x::OrientationValues::Landscape)) {
      std::mem::swap(&mut size.0, &mut size.1);
    }
    size
  }

  pub(crate) fn printer_default_paper_scale_percent(&self) -> u32 {
    // Office fixed-format export keeps the active default A4 page when an
    // explicit Letter setup has no printer-settings relationship. Excel maps
    // the Letter worksheet canvas onto that default page at 95%; documents
    // without an explicit paper request remain native A4 at 100%.
    if (self.explicit_paper_size || self.implicit_microsoft_letter_canvas)
      && self.paper_size == MsPaperSize::Letter as u32
      && self.valid_printer_settings
    {
      OFFICE_LETTER_TO_DEFAULT_A4_SCALE_PERCENT
    } else {
      DEFAULT_PRINT_SCALE_PERCENT
    }
  }

  pub(crate) fn fixed_output_paper_scale_percent(&self, has_chart: bool) -> u32 {
    // pageSetup@scale is the worksheet print scale itself. LibreOffice's
    // PageSettingsConverter writes it directly to PageScale, independently
    // of the related printer-settings bytes. The Letter-to-default-paper
    // fallback is only needed when that worksheet scale is implicit; applying
    // both would compound an explicit scale (for example, 83% into 78.85%).
    if has_chart
      || self.implicit_microsoft_letter_canvas
      || self.related_printer_canvas && !self.explicit_page_scale
    {
      self.printer_default_paper_scale_percent()
    } else {
      DEFAULT_PRINT_SCALE_PERCENT
    }
  }

  pub(crate) fn printer_default_paper_body_offset_y_pt(&self, scale: f32) -> f32 {
    if self.printer_default_paper_scale_percent() == DEFAULT_PRINT_SCALE_PERCENT {
      return 0.0;
    }
    let (_, output_height) = self.page_size_pt();
    let mut requested = self.clone();
    requested.valid_printer_settings = false;
    let (_, requested_height) = requested.page_size_pt();
    let vertical_margins =
      (self.margin_top_in + self.margin_bottom_in) as f32 * units::POINTS_PER_INCH;
    let output_body = (output_height - vertical_margins).max(0.0);
    let requested_body = (requested_height - vertical_margins).max(0.0) * scale;
    ((output_body - requested_body) / 2.0).max(0.0)
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
    // HeaderFooterParser tokenizes these strings later; page settings owns the
    // six text channels and picture relationship state.
    Self {
      different_odd_even: header_footer
        .different_odd_even
        .is_some_and(|value| value.as_bool()),
      different_first: header_footer
        .different_first
        .is_some_and(|value| value.as_bool()),
      // sml.xsd CT_HeaderFooter declares both attributes with default=true.
      scale_with_doc: header_footer
        .scale_with_doc
        .is_none_or(|value| value.as_bool()),
      align_with_margins: header_footer
        .align_with_margins
        .is_none_or(|value| value.as_bool()),
      odd_header: header_footer
        .odd_header
        .as_ref()
        .and_then(|value| value.xml_content.clone()),
      odd_footer: header_footer
        .odd_footer
        .as_ref()
        .and_then(|value| value.xml_content.clone()),
      even_header: header_footer
        .even_header
        .as_ref()
        .and_then(|value| value.xml_content.clone()),
      even_footer: header_footer
        .even_footer
        .as_ref()
        .and_then(|value| value.xml_content.clone()),
      first_header: header_footer
        .first_header
        .as_ref()
        .and_then(|value| value.xml_content.clone()),
      first_footer: header_footer
        .first_footer
        .as_ref()
        .and_then(|value| value.xml_content.clone()),
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
  pub(crate) fn has_print_content(&self) -> bool {
    self.text_len() > 0
      || self.legacy_drawing_relationship_id.is_some()
      || self.drawing_relationship_id.is_some()
      || self.drawing_slot_count > 0
      || self.background_picture_relationship_id.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sample_windows_devmode(fields: u32) -> Vec<u8> {
    fn put_u16(bytes: &mut [u8], offset: usize, value: u16) {
      bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }
    fn put_i16(bytes: &mut [u8], offset: usize, value: i16) {
      bytes[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
    }
    fn put_u32(bytes: &mut [u8], offset: usize, value: u32) {
      bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    let mut bytes = vec![0; 224];
    for (index, value) in "Test Printer".encode_utf16().enumerate() {
      put_u16(&mut bytes, index * 2, value);
    }
    put_u16(&mut bytes, DEVMODEW_SPEC_VERSION_OFFSET, 0x0401);
    put_u16(&mut bytes, DEVMODEW_SIZE_OFFSET, 220);
    put_u16(&mut bytes, DEVMODEW_DRIVER_EXTRA_OFFSET, 4);
    put_u32(&mut bytes, DEVMODEW_FIELDS_OFFSET, fields);
    put_i16(&mut bytes, DEVMODEW_ORIENTATION_OFFSET, 1);
    put_i16(&mut bytes, DEVMODEW_PAPER_SIZE_OFFSET, 1);
    put_i16(&mut bytes, DEVMODEW_PAPER_LENGTH_OFFSET, 2_794);
    put_i16(&mut bytes, DEVMODEW_PAPER_WIDTH_OFFSET, 2_159);
    put_i16(&mut bytes, DEVMODEW_SCALE_OFFSET, 75);
    put_i16(&mut bytes, DEVMODEW_PRINT_QUALITY_OFFSET, 600);
    put_i16(&mut bytes, DEVMODEW_Y_RESOLUTION_OFFSET, 300);
    bytes
  }

  #[test]
  fn windows_printer_settings_parse_only_initialized_public_devmode_fields() {
    let settings = WindowsPrinterSettings::from_bytes(&sample_windows_devmode(
      DM_ORIENTATION | DM_PAPER_SIZE | DM_SCALE | DM_PRINT_QUALITY | DM_Y_RESOLUTION,
    ))
    .unwrap();

    assert_eq!(settings.device_name, "Test Printer");
    assert_eq!(settings.paper_size, Some(1));
    assert_eq!(settings.paper_length_tenth_mm, None);
    assert_eq!(settings.paper_width_tenth_mm, None);
    assert_eq!(settings.scale_percent, Some(75));
    assert_eq!(settings.orientation, Some(x::OrientationValues::Portrait));
    assert_eq!(settings.horizontal_dpi, Some(600));
    assert_eq!(settings.vertical_dpi, Some(300));
  }

  #[test]
  fn windows_printer_settings_reject_truncated_driver_private_data() {
    let mut bytes = sample_windows_devmode(DM_PAPER_SIZE);
    bytes.truncate(220);

    assert!(WindowsPrinterSettings::from_bytes(&bytes).is_none());
  }

  #[test]
  fn related_devmode_fills_omitted_page_setup_without_overriding_output_paper() {
    let printer = WindowsPrinterSettings::from_bytes(&sample_windows_devmode(
      DM_ORIENTATION | DM_PAPER_SIZE | DM_SCALE | DM_PRINT_QUALITY | DM_Y_RESOLUTION,
    ))
    .unwrap();
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        id: Some("rId1".to_string()),
        orientation: Some(x::OrientationValues::Portrait),
        ..Default::default()
      }),
      ..Default::default()
    };

    let settings = CalcPageSettings::from_worksheet(&worksheet, true, Some(&printer));
    let (width, height) = settings.page_size_pt();

    assert_eq!(settings.paper_size, MsPaperSize::Letter as u32);
    assert_eq!(settings.scale, 75);
    assert_eq!(settings.horizontal_dpi, 600);
    assert_eq!(settings.vertical_dpi, 300);
    assert_eq!(settings.fixed_output_paper_scale_percent(false), 95);
    assert!((width - units::millimeters_to_points(210.0)).abs() < 0.01);
    assert!((height - units::millimeters_to_points(297.0)).abs() < 0.01);
  }

  #[test]
  fn explicit_page_setup_fields_override_related_devmode() {
    let printer = WindowsPrinterSettings::from_bytes(&sample_windows_devmode(
      DM_ORIENTATION | DM_PAPER_SIZE | DM_SCALE | DM_PRINT_QUALITY | DM_Y_RESOLUTION,
    ))
    .unwrap();
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        paper_size: Some(8),
        scale: Some(80),
        orientation: Some(x::OrientationValues::Landscape),
        horizontal_dpi: Some(1_200),
        vertical_dpi: Some(1_200),
        id: Some("rId1".to_string()),
        ..Default::default()
      }),
      ..Default::default()
    };

    let settings = CalcPageSettings::from_worksheet(&worksheet, true, Some(&printer));

    assert_eq!(settings.paper_size, 8);
    assert_eq!(settings.scale, 80);
    assert_eq!(settings.orientation, Some(x::OrientationValues::Landscape));
    assert_eq!(settings.horizontal_dpi, 1_200);
    assert_eq!(settings.vertical_dpi, 1_200);
    assert_eq!(settings.fixed_output_paper_scale_percent(false), 100);
  }

  #[test]
  fn explicit_page_scale_is_not_compounded_by_related_printer_canvas_fallback() {
    let printer = WindowsPrinterSettings::from_bytes(&sample_windows_devmode(
      DM_ORIENTATION | DM_PAPER_SIZE | DM_SCALE | DM_PRINT_QUALITY | DM_Y_RESOLUTION,
    ))
    .unwrap();
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        scale: Some(83),
        orientation: Some(x::OrientationValues::Landscape),
        id: Some("rId1".to_string()),
        ..Default::default()
      }),
      ..Default::default()
    };

    let settings = CalcPageSettings::from_worksheet(&worksheet, false, Some(&printer));

    assert_eq!(settings.scale, 83);
    assert_eq!(settings.fixed_output_paper_scale_percent(false), 100);
  }

  #[test]
  fn worksheet_without_page_margins_uses_excel_application_defaults() {
    let settings = CalcPageSettings::from_worksheet(&x::Worksheet::default(), false, None);

    assert!(!settings.has_margins);
    assert_eq!(settings.margin_left_in, 0.7);
    assert_eq!(settings.margin_right_in, 0.7);
    assert_eq!(settings.margin_top_in, 0.75);
    assert_eq!(settings.margin_bottom_in, 0.75);
    assert_eq!(settings.margin_header_in, 0.3);
    assert_eq!(settings.margin_footer_in, 0.3);
  }

  #[test]
  fn chartsheet_with_default_orientation_uses_landscape_a4() {
    let settings = CalcPageSettings::from_chartsheet(&x::Chartsheet::default(), None);
    let (width, height) = settings.page_size_pt();

    assert!(width > height);
    assert!((width - units::millimeters_to_points(297.0)).abs() < 0.01);
    assert!((height - units::millimeters_to_points(210.0)).abs() < 0.01);
  }

  #[test]
  fn worksheet_without_explicit_paper_size_keeps_default_a4() {
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        orientation: Some(x::OrientationValues::Portrait),
        ..Default::default()
      }),
      ..Default::default()
    };

    let (width, height) = CalcPageSettings::from_worksheet(&worksheet, false, None).page_size_pt();

    assert!((width - units::millimeters_to_points(210.0)).abs() < 0.01);
    assert!((height - units::millimeters_to_points(297.0)).abs() < 0.01);
  }

  #[test]
  fn worksheet_with_explicit_letter_paper_size_uses_letter() {
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        paper_size: Some(1),
        id: Some("rId1".to_string()),
        orientation: Some(x::OrientationValues::Portrait),
        ..Default::default()
      }),
      ..Default::default()
    };

    let (width, height) = CalcPageSettings::from_worksheet(&worksheet, false, None).page_size_pt();

    assert!((width - 8.5 * units::POINTS_PER_INCH).abs() < 0.01);
    assert!((height - 11.0 * units::POINTS_PER_INCH).abs() < 0.01);
  }

  #[test]
  fn worksheet_without_printer_settings_relationship_uses_default_a4() {
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        paper_size: Some(1),
        orientation: Some(x::OrientationValues::Portrait),
        ..Default::default()
      }),
      ..Default::default()
    };

    let (width, height) = CalcPageSettings::from_worksheet(&worksheet, false, None).page_size_pt();

    assert!((width - units::millimeters_to_points(210.0)).abs() < 0.01);
    assert!((height - units::millimeters_to_points(297.0)).abs() < 0.01);
  }

  #[test]
  fn header_footer_schema_defaults_scale_and_align_with_document() {
    let model = HeaderFooterModel::from_header_footer(&x::HeaderFooter::default());

    assert!(model.scale_with_doc);
    assert!(model.align_with_margins);
  }

  #[test]
  fn worksheet_without_printer_settings_keeps_nondefault_explicit_paper() {
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        paper_size: Some(8),
        orientation: Some(x::OrientationValues::Portrait),
        ..Default::default()
      }),
      ..Default::default()
    };

    let (width, height) = CalcPageSettings::from_worksheet(&worksheet, false, None).page_size_pt();

    assert!((width - units::millimeters_to_points(297.0)).abs() < 0.01);
    assert!((height - units::millimeters_to_points(420.0)).abs() < 0.01);
  }
}
