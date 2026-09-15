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
// Excel fixed-format output exposes the physical A4 MediaBox on the 600dpi
// device, but its Letter-to-default-A4 worksheet transform centers against an
// imageable vertical span 23 dots shorter than that physical page. Independent
// top/bottom-margin interpolation keeps the expected +0.975/-0.025 slopes,
// proving this is a device-span term rather than a VML/object adjustment.
const OFFICE_DEFAULT_A4_IMAGEABLE_HEIGHT_TRIM_DOTS: f32 = 23.0;

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
    .as_chunks::<2>()
    .0
    .iter()
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
  default_printer_canvas: bool,
  related_printer_letter_canvas: bool,
  pub(crate) valid_printer_settings: bool,
  pub(crate) fit_to_page: bool,
  pub(crate) scale: u32,
  pub(crate) fit_to_width: u32,
  pub(crate) fit_to_height: u32,
  pub(crate) horizontal_dpi: u32,
  pub(crate) vertical_dpi: u32,
  pub(crate) page_order: Option<x::PageOrderValues>,
  pub(crate) first_page_number: Option<i64>,
  pub(crate) orientation: Option<x::OrientationValues>,
  pub(crate) horizontal_centered: bool,
  pub(crate) vertical_centered: bool,
  pub(crate) print_headings: bool,
  pub(crate) print_grid_lines: bool,
  pub(crate) cell_comments: x::CellCommentsValues,
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
      default_printer_canvas: false,
      related_printer_letter_canvas: false,
      valid_printer_settings: true,
      fit_to_page: false,
      scale: DEFAULT_PRINT_SCALE_PERCENT,
      fit_to_width: 1,
      fit_to_height: 1,
      horizontal_dpi: units::OFFICE_FIXED_OUTPUT_DPI as u32,
      vertical_dpi: units::OFFICE_FIXED_OUTPUT_DPI as u32,
      page_order: Some(x::PageOrderValues::DownThenOver),
      first_page_number: None,
      orientation: None,
      horizontal_centered: false,
      vertical_centered: false,
      print_headings: false,
      print_grid_lines: false,
      cell_comments: x::CellCommentsValues::None,
      header_footer: HeaderFooterModel::default(),
    }
  }
}

impl CalcPageSettings {
  pub(crate) fn header_footer_page_number(&self, sheet_page_index: usize, automatic: usize) -> i64 {
    self.first_page_number.map_or(automatic as i64, |first| {
      first.saturating_add(sheet_page_index as i64)
    })
  }

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
    // canvas but may omit paperSize. A related DEVMODE whose media flags are
    // clear supplies no paper size either. SpreadsheetML defaults to Letter;
    // Office fixed output maps that canvas onto the active default page. A
    // worksheet with no pageSetup is the counterexample and retains the native
    // default page. This profile is emitted by both desktop and online Excel.
    settings.implicit_microsoft_letter_canvas = microsoft_office
      && worksheet
        .page_setup
        .as_ref()
        .is_some_and(|setup| setup.paper_size.is_none())
      && !settings.explicit_paper_size;
    // 61652's omitted paper, explicit Letter, and absent relationship controls
    // produce identical PDFs. Unflagged DEVMODE media bytes do not calibrate
    // an explicit Letter request; explicit paperSize=0 remains unscaled.
    if microsoft_office
      && settings.paper_size == MsPaperSize::Letter as u32
      && printer_settings.is_some_and(|printer| {
        printer.paper_size.is_none() && printer.custom_paper_size_pt().is_none()
      })
    {
      settings.valid_printer_settings = true;
    }
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
      settings.first_page_number = page_setup
        .use_first_page_number
        .is_some_and(|value| value.as_bool())
        .then(|| i64::from(page_setup.first_page_number.unwrap_or(1)));
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
    // Excel exports a chartsheet's Letter request on the active default
    // A4 page even when Letter came from a related DEVMODE. Native controls
    // retain an explicit A3 request and its portrait/landscape orientation.
    if settings.paper_size == MsPaperSize::Letter as u32
      && settings.requested_custom_paper_size_pt.is_none()
    {
      settings.valid_printer_settings = true;
      if chartsheet
        .chart_sheet_page_setup
        .as_ref()
        .and_then(|setup| setup.use_printer_defaults)
        .is_some_and(|value| !value.as_bool())
      {
        // Excel initializes this explicit opt-out to the active A4 paper;
        // it does not retain the Letter canvas's print-origin adjustment.
        settings.paper_size = MsPaperSize::A4 as u32;
        settings.related_printer_letter_canvas = false;
      } else {
        settings.implicit_microsoft_letter_canvas = true;
      }
    }
    if settings.orientation.is_none() {
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
    // ECMA-376 18.3.1.63: useFirstPageNumber enables the value whose schema
    // default is 1. Office tdf163554.xlsx restarts its second sheet this way.
    self.first_page_number = page_setup
      .use_first_page_number
      .is_some_and(|value| value.as_bool())
      .then(|| page_setup.first_page_number.unwrap_or(1));
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
    }
    self.fit_to_width = page_setup.fit_to_width.unwrap_or(self.fit_to_width);
    self.fit_to_height = page_setup.fit_to_height.unwrap_or(self.fit_to_height);
    self.horizontal_dpi = page_setup.horizontal_dpi.unwrap_or(self.horizontal_dpi);
    self.vertical_dpi = page_setup.vertical_dpi.unwrap_or(self.vertical_dpi);
    self.page_order = page_setup.page_order.or(self.page_order);
    self.orientation = page_setup.orientation;
    self.cell_comments = page_setup.cell_comments.unwrap_or_default();
    // CT_PageSetup defaults usePrinterDefaults to true when pageSetup exists.
    // An explicit false keeps the authored worksheet canvas (e.g. 56295.xlsx).
    self.default_printer_canvas = page_setup
      .use_printer_defaults
      .is_none_or(|value| value.as_bool());
  }

  fn apply_windows_printer_settings(
    &mut self,
    printer_settings: &WindowsPrinterSettings,
    page_setup_has_paper_size: bool,
    explicit_orientation: bool,
    explicit_scale: bool,
    explicit_horizontal_dpi: bool,
    explicit_vertical_dpi: bool,
  ) {
    // SpreadsheetML owns any field it serializes explicitly. The related
    // Printer Settings part fills only omitted device defaults; [MS-RPRN]
    // requires receivers to ignore DEVMODE fields whose dmFields bit is clear.
    if !page_setup_has_paper_size {
      if let Some(custom_size) = printer_settings.custom_paper_size_pt() {
        self.requested_custom_paper_size_pt = Some(custom_size);
        self.explicit_paper_size = true;
      } else if let Some(paper_size) = printer_settings.paper_size {
        self.paper_size = paper_size;
        self.explicit_paper_size = true;
        self.related_printer_letter_canvas = paper_size == MsPaperSize::Letter as u32;
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
    // GetDeviceCaps reports physical printer dimensions in integer device
    // units. Office's fixed-format MediaBox follows the same 600dpi grid
    // (A4 is 4,961 x 7,016 dots), not the unrounded millimetre conversion.
    size.0 = units::quantize_points_to_office_print_grid(size.0);
    size.1 = units::quantize_points_to_office_print_grid(size.1);
    size
  }

  pub(crate) fn printer_default_paper_scale_percent(&self) -> u32 {
    // Office fixed-format export keeps the active default A4 page when an
    // requested Letter canvas resolves onto that default page. Excel maps the
    // worksheet canvas at 95% for the chart fixed-output profile, whether the
    // concrete Letter request came from pageSetup or its related DEVMODE.
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
    // Frozen Office fixed-output profile, backed by independent Golden states:
    // 45540_classic_{Footer,Header}.xlsx require 95% for a related Letter
    // printer canvas, while 56295.xlsx keeps an unrelated explicit Letter
    // setup at 100%. barOfPieChart.xlsx and Microsoft's implicit Letter canvas
    // are the other positive profiles. tdf105272.xlsx is the fit-to-page
    // counterexample: fit owns the worksheet scale and must not gain 95%.
    // Initialized pageSetup with usePrinterDefaults (default true) also uses
    // that paper mapping; explicit false remains the unscaled counterexample.
    if self.fit_to_page {
      return DEFAULT_PRINT_SCALE_PERCENT;
    }
    if has_chart
      || self.implicit_microsoft_letter_canvas
      || self.related_printer_letter_canvas
      || self.default_printer_canvas
    {
      self.printer_default_paper_scale_percent()
    } else {
      DEFAULT_PRINT_SCALE_PERCENT
    }
  }

  pub(crate) fn fixed_output_pagination_paper_scale_percent(&self, has_chart: bool) -> u32 {
    // Office's two opposite states occur in the same 45540 workbook. The
    // ordinary Profile sheet keeps worksheet-zoom automatic column breaks
    // while its visible content is transformed to 95%. The Top Five
    // Industries chart sheet instead needs the transformed drawing footprint
    // to determine its single-page extent. Keep this pagination selector
    // independent from the frozen visible-output selector above.
    if has_chart {
      self.fixed_output_paper_scale_percent(true)
    } else {
      DEFAULT_PRINT_SCALE_PERCENT
    }
  }

  pub(crate) fn fixed_output_pagination_page_size_pt(&self, has_chart: bool) -> (f32, f32) {
    // With worksheet-zoom pagination, the requested printer canvas owns the
    // width before its Letter-to-A4 transform. 59264.xlsx starts its second
    // horizontal page at K: ten 50.05pt columns fit the Letter body. Chart
    // pages already paginate with the transformed output-page footprint.
    // Row breaks retain the output height: GraphPaper_TP10193274 fits three
    // explicit-height grids on A4; the shorter Letter height splits two grids.
    if self.fixed_output_paper_scale_percent(has_chart) < DEFAULT_PRINT_SCALE_PERCENT
      && self.fixed_output_pagination_paper_scale_percent(has_chart) == DEFAULT_PRINT_SCALE_PERCENT
    {
      let mut requested = self.clone();
      requested.valid_printer_settings = false;
      (requested.page_size_pt().0, self.page_size_pt().1)
    } else {
      self.page_size_pt()
    }
  }

  pub(crate) fn fixed_output_body_top_pt(&self, paper_scale_percent: u32) -> f32 {
    self.fixed_output_body_origin_pt(paper_scale_percent).1
  }

  pub(crate) fn fixed_output_body_origin_pt(&self, paper_scale_percent: u32) -> (f32, f32) {
    let left = self.margin_left_in as f32 * units::POINTS_PER_INCH;
    let top = self.margin_top_in as f32 * units::POINTS_PER_INCH;
    if paper_scale_percent >= DEFAULT_PRINT_SCALE_PERCENT {
      return (left, top);
    }
    let scale = paper_scale_percent as f32 / 100.0;
    if self.orientation == Some(x::OrientationValues::Landscape) {
      // The default-paper mapping follows the long paper axis. Landscape
      // worksheets, like chartsheets, translate horizontally: Office's
      // centered/un-centered Letter/A4 controls retain the physical top
      // margin while exposing the same independently derived paper offset.
      let mut portrait = self.clone();
      portrait.orientation = Some(x::OrientationValues::Portrait);
      portrait.margin_top_in = self.margin_left_in;
      portrait.margin_bottom_in = self.margin_right_in;
      (
        left + portrait.printer_default_paper_body_offset_y_pt(scale),
        top,
      )
    } else {
      (
        left,
        top + self.printer_default_paper_body_offset_y_pt(scale),
      )
    }
  }

  pub(crate) fn fixed_output_chartsheet_origin_pt(&self) -> (f32, f32) {
    let left = self.margin_left_in as f32 * units::POINTS_PER_INCH;
    let top = self.margin_top_in as f32 * units::POINTS_PER_INCH;
    let scale = self.printer_default_paper_scale_percent() as f32 / 100.0;
    if self.orientation == Some(x::OrientationValues::Landscape) {
      // The default-paper centering follows the long paper axis. Chart
      // anchors retain their physical extents even when this origin moves.
      let mut portrait = self.clone();
      portrait.orientation = Some(x::OrientationValues::Portrait);
      portrait.margin_top_in = self.margin_left_in;
      portrait.margin_bottom_in = self.margin_right_in;
      (
        left + portrait.printer_default_paper_body_offset_y_pt(scale),
        top,
      )
    } else {
      (
        left,
        top + self.printer_default_paper_body_offset_y_pt(scale),
      )
    }
  }

  fn printer_default_paper_body_offset_y_pt(&self, scale: f32) -> f32 {
    if self.printer_default_paper_scale_percent() == DEFAULT_PRINT_SCALE_PERCENT {
      return 0.0;
    }
    let (_, output_height) = self.page_size_pt();
    let mut requested = self.clone();
    requested.valid_printer_settings = false;
    let (_, requested_height) = requested.page_size_pt();
    let vertical_margins =
      (self.margin_top_in + self.margin_bottom_in) as f32 * units::POINTS_PER_INCH;
    let imageable_trim_pt = OFFICE_DEFAULT_A4_IMAGEABLE_HEIGHT_TRIM_DOTS * units::POINTS_PER_INCH
      / units::OFFICE_FIXED_OUTPUT_DPI;
    let output_body = (output_height - vertical_margins - imageable_trim_pt).max(0.0);
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

  #[test]
  fn worksheet_and_chartsheet_first_page_numbers() {
    for (use_first, first, expected) in [
      (None, None, 8),
      (None, Some(7_u32), 8),
      (Some(false), Some(7), 8),
      (Some(true), None, 3),
      (Some(true), Some(7), 9),
    ] {
      let worksheet = x::Worksheet {
        page_setup: Some(x::PageSetup {
          use_first_page_number: use_first.map(Into::into),
          first_page_number: first.map(i64::from),
          ..Default::default()
        }),
        ..Default::default()
      };
      let chartsheet = x::Chartsheet {
        chart_sheet_page_setup: Some(x::ChartSheetPageSetup {
          use_first_page_number: use_first.map(Into::into),
          first_page_number: first,
          ..Default::default()
        }),
        ..Default::default()
      };
      for settings in [
        CalcPageSettings::from_worksheet(&worksheet, true, None),
        CalcPageSettings::from_chartsheet(&chartsheet, None),
      ] {
        assert_eq!(
          settings.header_footer_page_number(2, 8),
          expected,
          "useFirstPageNumber={use_first:?}, firstPageNumber={first:?}"
        );
      }
    }
  }

  #[test]
  fn fixed_output_body_top_keeps_physical_margins_independent_of_headers() {
    for margin_top_in in [0.5, 1.025, 1.5] {
      for paper_scale_percent in [95, 100] {
        let mut settings = CalcPageSettings {
          margin_top_in,
          margin_bottom_in: 1.025,
          paper_size: MsPaperSize::Letter as u32,
          implicit_microsoft_letter_canvas: true,
          ..CalcPageSettings::default()
        };
        let top = settings.fixed_output_body_top_pt(paper_scale_percent);
        let expected_offset = if paper_scale_percent == 95 {
          settings.printer_default_paper_body_offset_y_pt(0.95)
        } else {
          0.0
        };
        assert!((top - margin_top_in as f32 * 72.0 - expected_offset).abs() < 1.0e-5);
        for (header, footer) in [(true, false), (false, true), (true, true)] {
          settings.header_footer.odd_header = header.then(|| "&C&A".to_string());
          settings.header_footer.odd_footer = footer.then(|| "&CPage &P".to_string());
          assert_eq!(settings.fixed_output_body_top_pt(paper_scale_percent), top);
        }
      }
    }
  }

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
  fn related_letter_devmode_keeps_device_and_paper_scales_separate() {
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
    assert_eq!(settings.printer_default_paper_scale_percent(), 95);
    assert_eq!(settings.fixed_output_paper_scale_percent(false), 95);
    assert_eq!(settings.fixed_output_paper_scale_percent(true), 95);
    assert_eq!(width, 595.32);
    assert_eq!(height, 841.92);
    assert!((settings.printer_default_paper_body_offset_y_pt(0.95) - 40.68).abs() < 1.0e-4);
  }

  #[test]
  fn landscape_default_paper_mapping_preserves_the_top_margin() {
    let printer = WindowsPrinterSettings::from_bytes(&sample_windows_devmode(
      DM_ORIENTATION | DM_PAPER_SIZE | DM_SCALE,
    ))
    .unwrap();
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        id: Some("rId1".to_string()),
        orientation: Some(x::OrientationValues::Landscape),
        ..Default::default()
      }),
      ..Default::default()
    };
    let mut settings = CalcPageSettings::from_worksheet(&worksheet, true, Some(&printer));
    settings.margin_left_in = 0.25;
    settings.margin_right_in = 0.25;
    settings.margin_top_in = 1.18;
    settings.margin_bottom_in = 1.0;
    // Office worksheet controls with centering disabled: the Letter-to-A4
    // map translates the long horizontal axis by 42.48pt. Explicit A4 keeps
    // the 18pt left margin. Both retain the authored 84.96pt top margin.
    let mapped = settings.fixed_output_body_origin_pt(95);
    let unmapped = settings.fixed_output_body_origin_pt(100);
    assert!((mapped.0 - 60.48).abs() < 0.001);
    assert!((mapped.1 - 84.96).abs() < 0.001);
    assert!((unmapped.0 - 18.0).abs() < 0.001);
    assert!((unmapped.1 - 84.96).abs() < 0.001);
  }

  #[test]
  fn related_devmode_without_media_preserves_the_default_letter_canvas() {
    for unflagged_paper in [1_u16, 9] {
      let mut bytes = sample_windows_devmode(DM_ORIENTATION);
      bytes[DEVMODEW_PAPER_SIZE_OFFSET..DEVMODEW_PAPER_SIZE_OFFSET + 2]
        .copy_from_slice(&unflagged_paper.to_le_bytes());
      let printer = WindowsPrinterSettings::from_bytes(&bytes).unwrap();
      assert_eq!(printer.paper_size, None);
      for (paper_size, expected_scale) in [(None, 95), (Some(1), 95), (Some(0), 100)] {
        let worksheet = x::Worksheet {
          page_setup: Some(x::PageSetup {
            paper_size,
            id: Some("rId1".to_string()),
            ..Default::default()
          }),
          ..Default::default()
        };
        let settings = CalcPageSettings::from_worksheet(&worksheet, true, Some(&printer));
        assert_eq!(
          settings.fixed_output_paper_scale_percent(false),
          expected_scale
        );
        assert_eq!(
          settings.fixed_output_body_top_pt(expected_scale) > 54.0,
          expected_scale == 95
        );
      }
    }
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
  fn fit_to_page_suppresses_the_related_letter_paper_transform() {
    let mut printer = WindowsPrinterSettings::from_bytes(&sample_windows_devmode(
      DM_ORIENTATION | DM_PAPER_SIZE | DM_SCALE | DM_PRINT_QUALITY | DM_Y_RESOLUTION,
    ))
    .unwrap();
    printer.scale_percent = Some(83);
    let worksheet = x::Worksheet {
      sheet_properties: Some(Box::new(x::SheetProperties {
        page_setup_properties: Some(x::PageSetupProperties {
          fit_to_page: Some(ooxmlsdk::simple_type::BooleanValue::One),
          ..Default::default()
        }),
        ..Default::default()
      })),
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
    assert!(settings.fit_to_page);
    assert_eq!(settings.fixed_output_paper_scale_percent(false), 100);
  }

  #[test]
  fn authored_scale_and_related_letter_paper_transform_remain_independent() {
    let printer = WindowsPrinterSettings::from_bytes(&sample_windows_devmode(
      DM_ORIENTATION | DM_PAPER_SIZE | DM_PRINT_QUALITY | DM_Y_RESOLUTION,
    ))
    .unwrap();
    assert_eq!(printer.scale_percent, None);
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        scale: Some(47),
        orientation: Some(x::OrientationValues::Portrait),
        id: Some("rId1".to_string()),
        ..Default::default()
      }),
      ..Default::default()
    };

    let settings = CalcPageSettings::from_worksheet(&worksheet, true, Some(&printer));
    assert_eq!(settings.scale, 47);
    assert_eq!(settings.printer_default_paper_scale_percent(), 95);
    assert_eq!(settings.fixed_output_paper_scale_percent(false), 95);
    assert_eq!(settings.fixed_output_paper_scale_percent(true), 95);
  }

  #[test]
  fn letter_to_a4_fixed_output_selector_is_frozen_by_state_matrix() {
    let native = CalcPageSettings::default();

    let mut unrelated_explicit_letter = native.clone();
    unrelated_explicit_letter.explicit_paper_size = true;

    let mut related_letter = unrelated_explicit_letter.clone();
    related_letter.related_printer_letter_canvas = true;

    let mut implicit_microsoft_letter = native.clone();
    implicit_microsoft_letter.implicit_microsoft_letter_canvas = true;

    let mut fit_related_letter = related_letter.clone();
    fit_related_letter.fit_to_page = true;

    let mut native_a4 = native.clone();
    native_a4.paper_size = MsPaperSize::A4 as u32;
    native_a4.explicit_paper_size = true;

    for (profile, settings, has_chart, expected_output, expected_pagination) in [
      ("native-default", native, false, 100, 100),
      (
        "unrelated-explicit-letter-56295",
        unrelated_explicit_letter.clone(),
        false,
        100,
        100,
      ),
      (
        "related-letter-45540-profile-sheet",
        related_letter.clone(),
        false,
        95,
        100,
      ),
      (
        "related-letter-45540-chart-sheet",
        related_letter,
        true,
        95,
        95,
      ),
      (
        "implicit-microsoft-letter",
        implicit_microsoft_letter,
        false,
        95,
        100,
      ),
      (
        "fit-related-letter-tdf105272",
        fit_related_letter,
        false,
        100,
        100,
      ),
      (
        "chart-letter-barOfPieChart",
        unrelated_explicit_letter,
        true,
        95,
        95,
      ),
      ("native-a4", native_a4, true, 100, 100),
    ] {
      assert_eq!(
        settings.fixed_output_paper_scale_percent(has_chart),
        expected_output,
        "fixed-output visible paper-scale profile changed: {profile}"
      );
      assert_eq!(
        settings.fixed_output_pagination_paper_scale_percent(has_chart),
        expected_pagination,
        "fixed-output pagination paper-scale profile changed: {profile}"
      );
    }
  }

  #[test]
  fn worksheet_printer_defaults_map_letter_canvas_without_charts() {
    for (use_defaults, expected_scale) in [(None, 95), (Some(true), 95), (Some(false), 100)] {
      let worksheet = x::Worksheet {
        page_setup: Some(x::PageSetup {
          paper_size: Some(MsPaperSize::Letter as u32),
          use_printer_defaults: use_defaults.map(Into::into),
          ..Default::default()
        }),
        ..Default::default()
      };
      let settings = CalcPageSettings::from_worksheet(&worksheet, false, None);
      assert_eq!(
        settings.fixed_output_paper_scale_percent(false),
        expected_scale
      );
      assert_eq!(
        settings.fixed_output_pagination_paper_scale_percent(false),
        100
      );
      assert_eq!(settings.scale, 100);
      assert_eq!(
        settings.fixed_output_body_top_pt(expected_scale) > 54.0,
        expected_scale < 100
      );
    }
  }

  #[test]
  fn explicit_letter_chart_keeps_the_independent_fixed_output_canvas_scale() {
    let worksheet = x::Worksheet {
      page_setup: Some(x::PageSetup {
        paper_size: Some(MsPaperSize::Letter as u32),
        scale: Some(100),
        use_printer_defaults: Some(false.into()),
        orientation: Some(x::OrientationValues::Portrait),
        ..Default::default()
      }),
      ..Default::default()
    };

    let settings = CalcPageSettings::from_worksheet(&worksheet, false, None);
    assert_eq!(settings.scale, 100);
    assert_eq!(settings.fixed_output_paper_scale_percent(false), 100);
    assert_eq!(settings.fixed_output_paper_scale_percent(true), 95);
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
  fn chartsheet_letter_uses_default_paper_and_preserves_explicit_orientation() {
    let printer = WindowsPrinterSettings {
      paper_size: Some(1),
      ..Default::default()
    };
    for (paper_size, use_defaults, orientation, expected) in [
      (
        None,
        None,
        x::OrientationValues::Landscape,
        (841.92, 595.32),
      ),
      (
        Some(1),
        None,
        x::OrientationValues::Landscape,
        (841.92, 595.32),
      ),
      (
        None,
        Some(false),
        x::OrientationValues::Landscape,
        (841.92, 595.32),
      ),
      (
        Some(1),
        Some(false),
        x::OrientationValues::Landscape,
        (841.92, 595.32),
      ),
      (
        Some(8),
        None,
        x::OrientationValues::Landscape,
        (1190.52, 841.92),
      ),
      (
        Some(8),
        None,
        x::OrientationValues::Portrait,
        (841.92, 1190.52),
      ),
    ] {
      let sheet = x::Chartsheet {
        chart_sheet_page_setup: Some(x::ChartSheetPageSetup {
          id: Some("rId1".into()),
          paper_size,
          use_printer_defaults: use_defaults.map(Into::into),
          orientation: Some(orientation),
          ..Default::default()
        }),
        ..Default::default()
      };
      let settings = CalcPageSettings::from_chartsheet(&sheet, Some(&printer));
      assert_eq!(settings.page_size_pt(), expected);
      assert_eq!(settings.orientation, Some(orientation));
    }
  }

  #[test]
  fn chartsheet_with_default_orientation_uses_landscape_a4() {
    let settings = CalcPageSettings::from_chartsheet(&x::Chartsheet::default(), None);
    let (width, height) = settings.page_size_pt();

    assert!(width > height);
    assert_eq!(
      width,
      units::quantize_points_to_office_print_grid(units::millimeters_to_points(297.0))
    );
    assert_eq!(
      height,
      units::quantize_points_to_office_print_grid(units::millimeters_to_points(210.0))
    );
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

    assert_eq!(width, 595.32);
    assert_eq!(height, 841.92);
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

    assert_eq!(width, 595.32);
    assert_eq!(height, 841.92);
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

    assert_eq!(
      width,
      units::quantize_points_to_office_print_grid(units::millimeters_to_points(297.0))
    );
    assert_eq!(
      height,
      units::quantize_points_to_office_print_grid(units::millimeters_to_points(420.0))
    );
  }
}
