/// Options for OOXML to PDF conversion.
#[derive(Clone, Debug)]
pub struct PdfOptions {
  /// PDF version or conformance standards requested from the backend.
  pub standards: Vec<PdfStandard>,

  /// Whether PDF content streams should be compressed.
  pub compress_content_streams: bool,

  /// JPEG quality used when the PDF filter asks raster graphics to be stored as JPEG.
  pub jpeg_quality: Option<u8>,

  /// Input file name used by spreadsheet formulas such as CELL("filename").
  pub source_file_name: Option<String>,

  pub general: PdfGeneralOptions,
  pub images: PdfImageOptions,
  pub links: PdfLinkOptions,
  pub forms: PdfFormOptions,
  pub viewer: PdfViewerOptions,
  pub metadata: PdfMetadataOptions,
  pub watermark: Option<PdfWatermarkOptions>,
  pub spreadsheet: PdfSpreadsheetOptions,
}

impl Default for PdfOptions {
  fn default() -> Self {
    Self {
      standards: Vec::new(),
      compress_content_streams: true,
      jpeg_quality: None,
      source_file_name: None,
      general: PdfGeneralOptions::default(),
      images: PdfImageOptions::default(),
      links: PdfLinkOptions::default(),
      forms: PdfFormOptions::default(),
      viewer: PdfViewerOptions::default(),
      metadata: PdfMetadataOptions::default(),
      watermark: None,
      spreadsheet: PdfSpreadsheetOptions::default(),
    }
  }
}

impl PdfOptions {
  pub(crate) fn effective_jpeg_quality(&self) -> Option<u8> {
    self.images.jpeg_quality.or(self.jpeg_quality)
  }

  pub(crate) fn take_layout_options(&mut self) -> ooxmlsdk_layout::options::LayoutOptions {
    ooxmlsdk_layout::options::LayoutOptions {
      source_file_name: self.source_file_name.take(),
      ..Default::default()
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfStandard {
  Pdf14,
  Pdf15,
  Pdf16,
  Pdf17,
  Pdf20,
  PdfA1a,
  PdfA1b,
  PdfA2a,
  PdfA2b,
  PdfA2u,
  PdfA3a,
  PdfA3b,
  PdfA3u,
  PdfA4,
  PdfA4f,
  PdfA4e,
  PdfUa1,
}

#[derive(Clone, Debug)]
pub struct PdfGeneralOptions {
  pub tagged_pdf: bool,
  pub pdf_ua_compliance: bool,
  pub export_bookmarks: bool,
  pub open_bookmark_levels: Option<i32>,
  pub page_range: Option<String>,
  pub skip_empty_pages: bool,
  pub remove_transparencies: bool,
}

impl Default for PdfGeneralOptions {
  fn default() -> Self {
    Self {
      tagged_pdf: false,
      pdf_ua_compliance: false,
      export_bookmarks: true,
      open_bookmark_levels: None,
      page_range: None,
      skip_empty_pages: false,
      remove_transparencies: false,
    }
  }
}

#[derive(Clone, Debug)]
pub struct PdfImageOptions {
  pub use_lossless_compression: bool,
  pub jpeg_quality: Option<u8>,
  pub reduce_resolution: bool,
  pub max_resolution_dpi: Option<u32>,
}

impl Default for PdfImageOptions {
  fn default() -> Self {
    Self {
      use_lossless_compression: false,
      jpeg_quality: None,
      reduce_resolution: false,
      max_resolution_dpi: Some(300),
    }
  }
}

#[derive(Clone, Debug)]
pub struct PdfLinkOptions {
  pub export_relative_filesystem_links: bool,
  pub convert_office_targets_to_pdf_targets: bool,
  pub export_bookmarks_to_pdf_destinations: bool,
  pub default_action: PdfLinkDefaultAction,
}

impl Default for PdfLinkOptions {
  fn default() -> Self {
    Self {
      export_relative_filesystem_links: false,
      convert_office_targets_to_pdf_targets: false,
      export_bookmarks_to_pdf_destinations: false,
      default_action: PdfLinkDefaultAction::Uri,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PdfLinkDefaultAction {
  #[default]
  Uri,
  UriDestination,
  Launch,
  RemoveExternalLinks,
}

#[derive(Clone, Debug)]
pub struct PdfFormOptions {
  pub export_form_fields: bool,
  pub submit_format: PdfFormSubmitFormat,
  pub allow_duplicate_field_names: bool,
}

impl Default for PdfFormOptions {
  fn default() -> Self {
    Self {
      export_form_fields: false,
      submit_format: PdfFormSubmitFormat::Pdf,
      allow_duplicate_field_names: false,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PdfFormSubmitFormat {
  Html,
  Xml,
  Fdf,
  #[default]
  Pdf,
}

#[derive(Clone, Debug)]
pub struct PdfViewerOptions {
  pub page_mode: PdfViewerPageMode,
  pub page_layout: PdfPageLayout,
  pub magnification: PdfViewerMagnification,
  pub initial_page: u32,
  pub hide_toolbar: bool,
  pub hide_menubar: bool,
  pub hide_window_controls: bool,
  pub fit_window: bool,
  pub center_window: bool,
  pub display_document_title: bool,
  pub full_screen: bool,
  pub first_page_left: bool,
}

impl Default for PdfViewerOptions {
  fn default() -> Self {
    Self {
      page_mode: PdfViewerPageMode::Default,
      page_layout: PdfPageLayout::Default,
      magnification: PdfViewerMagnification::Default,
      initial_page: 1,
      hide_toolbar: false,
      hide_menubar: false,
      hide_window_controls: false,
      fit_window: false,
      center_window: false,
      display_document_title: true,
      full_screen: false,
      first_page_left: false,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PdfViewerPageMode {
  #[default]
  Default,
  UseOutlines,
  UseThumbs,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PdfPageLayout {
  #[default]
  Default,
  SinglePage,
  Continuous,
  ContinuousFacing,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum PdfViewerMagnification {
  #[default]
  Default,
  FitInWindow,
  FitWidth,
  FitVisible,
  Zoom(f32),
}

#[derive(Clone, Debug, Default)]
pub struct PdfMetadataOptions {
  pub title: Option<String>,
  pub author: Option<String>,
  pub subject: Option<String>,
  pub keywords: Option<String>,
  pub creator: Option<String>,
  pub producer: Option<String>,
}

#[derive(Clone, Debug)]
pub struct PdfWatermarkOptions {
  pub text: String,
  pub color_rgb: Option<u32>,
  pub font_name: Option<String>,
  pub font_height_pt: Option<f32>,
  pub rotate_degrees: Option<f32>,
  pub tiled_text: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct PdfSpreadsheetOptions {
  pub single_page_sheets: bool,
}
