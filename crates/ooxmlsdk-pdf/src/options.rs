use std::sync::Arc;
use std::{fmt, mem};

use crate::error::{PdfError, Result};

/// OOXML package family being converted to PDF.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PdfDocumentKind {
  Docx,
  Xlsx,
  Pptx,
}

impl fmt::Display for PdfDocumentKind {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(match self {
      Self::Docx => "DOCX",
      Self::Xlsx => "XLSX",
      Self::Pptx => "PPTX",
    })
  }
}

/// Stable capability identifiers used by callers and golden-plan generators.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PdfOptionFeature {
  PdfVersion,
  PdfA,
  PdfUa,
  ContentStreamCompression,
  UiLanguage,
  FormatLocale,
  DocumentLanguage,
  FieldUpdateDateTime,
  FieldUpdateTimeZone,
  PaperSize,
  TaggedPdf,
  Bookmarks,
  PageRange,
  SkipEmptyPages,
  TransparencyFlattening,
  ImageCompression,
  ImageDownsampling,
  Links,
  FormFields,
  ViewerPreferences,
  Metadata,
  Attachments,
  Watermark,
  SinglePageSheets,
}

impl PdfOptionFeature {
  pub const ALL: [Self; 24] = [
    Self::PdfVersion,
    Self::PdfA,
    Self::PdfUa,
    Self::ContentStreamCompression,
    Self::UiLanguage,
    Self::FormatLocale,
    Self::DocumentLanguage,
    Self::FieldUpdateDateTime,
    Self::FieldUpdateTimeZone,
    Self::PaperSize,
    Self::TaggedPdf,
    Self::Bookmarks,
    Self::PageRange,
    Self::SkipEmptyPages,
    Self::TransparencyFlattening,
    Self::ImageCompression,
    Self::ImageDownsampling,
    Self::Links,
    Self::FormFields,
    Self::ViewerPreferences,
    Self::Metadata,
    Self::Attachments,
    Self::Watermark,
    Self::SinglePageSheets,
  ];
}

impl fmt::Display for PdfOptionFeature {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(match self {
      Self::PdfVersion => "pdf-version",
      Self::PdfA => "pdf-a",
      Self::PdfUa => "pdf-ua",
      Self::ContentStreamCompression => "content-stream-compression",
      Self::UiLanguage => "ui-language",
      Self::FormatLocale => "format-locale",
      Self::DocumentLanguage => "document-language",
      Self::FieldUpdateDateTime => "field-update-date-time",
      Self::FieldUpdateTimeZone => "field-update-time-zone",
      Self::PaperSize => "paper-size",
      Self::TaggedPdf => "tagged-pdf",
      Self::Bookmarks => "bookmarks",
      Self::PageRange => "page-range",
      Self::SkipEmptyPages => "skip-empty-pages",
      Self::TransparencyFlattening => "transparency-flattening",
      Self::ImageCompression => "image-compression",
      Self::ImageDownsampling => "image-downsampling",
      Self::Links => "links",
      Self::FormFields => "form-fields",
      Self::ViewerPreferences => "viewer-preferences",
      Self::Metadata => "metadata",
      Self::Attachments => "attachments",
      Self::Watermark => "watermark",
      Self::SinglePageSheets => "single-page-sheets",
    })
  }
}

/// Whether a requested PDF feature is ready for corpus assignment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PdfOptionSupport {
  Supported,
  SupportedWithRestrictions,
  Unsupported,
}

/// Report the current candidate-backend support for one option family.
pub const fn pdf_option_support(
  document_kind: PdfDocumentKind,
  feature: PdfOptionFeature,
) -> PdfOptionSupport {
  use PdfOptionFeature as Feature;
  use PdfOptionSupport as Support;

  match feature {
    Feature::PdfVersion
    | Feature::ContentStreamCompression
    | Feature::FormatLocale
    | Feature::DocumentLanguage
    | Feature::TaggedPdf
    | Feature::Bookmarks
    | Feature::PageRange
    | Feature::ImageCompression
    | Feature::ImageDownsampling
    | Feature::ViewerPreferences
    | Feature::Metadata => Support::Supported,
    Feature::PdfA
    | Feature::PdfUa
    | Feature::UiLanguage
    | Feature::FieldUpdateDateTime
    | Feature::FieldUpdateTimeZone
    | Feature::Links
    | Feature::Attachments => Support::SupportedWithRestrictions,
    Feature::FormFields if matches!(document_kind, PdfDocumentKind::Docx) => {
      Support::SupportedWithRestrictions
    }
    Feature::SkipEmptyPages
    | Feature::TransparencyFlattening
    | Feature::PaperSize
    | Feature::FormFields
    | Feature::Watermark
    | Feature::SinglePageSheets => Support::Unsupported,
  }
}

/// A deterministic normalization performed while resolving requested options.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct PdfOptionAdjustment {
  pub feature: PdfOptionFeature,
  pub reason: &'static str,
}

/// Validated options consumed by layout and PDF serialization.
#[derive(Clone, Debug)]
pub struct ResolvedPdfOptions {
  pub effective: PdfOptions,
  pub adjustments: Vec<PdfOptionAdjustment>,
}

impl ResolvedPdfOptions {
  pub fn effective(&self) -> &PdfOptions {
    &self.effective
  }

  pub fn into_effective(self) -> PdfOptions {
    self.effective
  }
}

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

  /// BCP 47 user-interface language for generated document labels.
  pub ui_language: Option<String>,

  /// BCP 47 locale for locale-dependent number, date, currency, and value
  /// formatting. This does not select translated application labels.
  pub format_locale: Option<String>,

  /// BCP 47 language for document authoring defaults and the PDF document
  /// language. This does not select translated application labels or value
  /// formatting conventions.
  pub default_document_language: Option<String>,

  /// Local civil time used to refresh unlocked WordprocessingML DATE, TIME,
  /// PRINTDATE, and SAVEDATE fields. When absent, persisted field results
  /// remain authoritative.
  pub field_update_datetime: Option<ooxmlsdk_layout::options::FieldUpdateDateTime>,

  /// IANA time-zone name used to convert absolute package-property
  /// timestamps when fields such as WordprocessingML CREATEDATE are
  /// refreshed. This is independent from UI language and format locale.
  pub field_update_time_zone: Option<String>,

  pub general: PdfGeneralOptions,
  pub images: PdfImageOptions,
  pub links: PdfLinkOptions,
  pub forms: PdfFormOptions,
  pub viewer: PdfViewerOptions,
  pub metadata: PdfMetadataOptions,
  /// Files embedded in the PDF name tree and, where supported, associated with the document.
  pub attachments: Vec<PdfAttachment>,
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
      ui_language: None,
      format_locale: None,
      default_document_language: None,
      field_update_datetime: None,
      field_update_time_zone: None,
      general: PdfGeneralOptions::default(),
      images: PdfImageOptions::default(),
      links: PdfLinkOptions::default(),
      forms: PdfFormOptions::default(),
      viewer: PdfViewerOptions::default(),
      metadata: PdfMetadataOptions::default(),
      attachments: Vec::new(),
      watermark: None,
      spreadsheet: PdfSpreadsheetOptions::default(),
    }
  }
}

/// A file to embed in the generated PDF.
#[derive(Clone, Debug)]
pub struct PdfAttachment {
  pub path: String,
  pub mime_type: String,
  pub description: String,
  pub association: PdfAttachmentAssociation,
  pub data: Arc<[u8]>,
  pub modification_date: Option<PdfDateTime>,
  pub compress: Option<bool>,
}

/// How an attachment relates to the generated PDF.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PdfAttachmentAssociation {
  Source,
  Data,
  Alternative,
  Supplement,
  #[default]
  Unspecified,
}

/// A deterministic PDF timestamp, including an optional UTC offset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PdfDateTime {
  pub year: u16,
  pub month: Option<u8>,
  pub day: Option<u8>,
  pub hour: Option<u8>,
  pub minute: Option<u8>,
  pub second: Option<u8>,
  pub utc_offset_hour: Option<i8>,
  pub utc_offset_minute: Option<u8>,
}

impl PdfOptions {
  pub(crate) fn effective_jpeg_quality(&self) -> Option<u8> {
    self.images.jpeg_quality.or(self.jpeg_quality)
  }

  pub(crate) fn take_layout_options(&mut self) -> ooxmlsdk_layout::options::LayoutOptions {
    ooxmlsdk_layout::options::LayoutOptions {
      source_file_name: self.source_file_name.clone(),
      ui_language: self.ui_language.clone(),
      format_locale: self.format_locale.clone(),
      default_document_language: self.default_document_language.clone(),
      field_update_datetime: self.field_update_datetime,
      field_update_time_zone: self.field_update_time_zone.clone(),
      ..Default::default()
    }
  }

  pub(crate) fn canonical_document_language(&self) -> Option<String> {
    ooxmlsdk_layout::localization::canonical_office_locale_tag(
      self.default_document_language.as_deref(),
    )
    .or_else(|| {
      ooxmlsdk_layout::localization::canonical_office_locale_tag(self.ui_language.as_deref())
    })
  }

  pub(crate) fn resolve_for(self, document_kind: PdfDocumentKind) -> Result<ResolvedPdfOptions> {
    let mut effective = self;
    let mut adjustments = Vec::new();

    resolve_layout_environment(&mut effective, &mut adjustments)?;
    resolve_standards(&mut effective, &mut adjustments)?;
    resolve_images(&mut effective, &mut adjustments)?;
    resolve_bookmarks(&mut effective, &mut adjustments)?;
    resolve_forms(document_kind, &mut effective, &mut adjustments)?;
    resolve_links(document_kind, &effective)?;
    resolve_viewer(&effective)?;
    resolve_metadata(&effective)?;
    reject_unimplemented_options(document_kind, &effective)?;

    Ok(ResolvedPdfOptions {
      effective,
      adjustments,
    })
  }
}

fn resolve_layout_environment(
  options: &mut PdfOptions,
  adjustments: &mut Vec<PdfOptionAdjustment>,
) -> Result<()> {
  canonicalize_locale(
    &mut options.ui_language,
    PdfOptionFeature::UiLanguage,
    "UI language",
    adjustments,
  )?;
  canonicalize_locale(
    &mut options.format_locale,
    PdfOptionFeature::FormatLocale,
    "format locale",
    adjustments,
  )?;
  canonicalize_locale(
    &mut options.default_document_language,
    PdfOptionFeature::DocumentLanguage,
    "document language",
    adjustments,
  )?;

  if options.ui_language.as_deref().is_some_and(|language| {
    !language.eq_ignore_ascii_case("en")
      && !language.starts_with("en-")
      && !language.eq_ignore_ascii_case("zh")
      && !language.starts_with("zh-")
  }) {
    adjustments.push(PdfOptionAdjustment {
      feature: PdfOptionFeature::UiLanguage,
      reason: "application-generated UI strings currently fall back to English outside English and Chinese resource packs",
    });
  }

  if let Some(value) = options.field_update_datetime {
    let year = i16::try_from(value.year).map_err(|_| {
      PdfError::Options(format!(
        "field update year must be in the supported civil range, got {}",
        value.year
      ))
    })?;
    let component = |name: &str, value: u8| {
      i8::try_from(value).map_err(|_| {
        PdfError::Options(format!(
          "field update {name} is outside the supported civil range: {value}"
        ))
      })
    };
    let month = component("month", value.month)?;
    let day = component("day", value.day)?;
    let hour = component("hour", value.hour)?;
    let minute = component("minute", value.minute)?;
    let second = component("second", value.second)?;
    jiff::civil::DateTime::new(year, month, day, hour, minute, second, 0)
      .map_err(|error| PdfError::Options(format!("invalid field update date-time: {error}")))?;
  }

  if let Some(time_zone) = options.field_update_time_zone.as_deref() {
    if options.field_update_datetime.is_none() {
      options.field_update_time_zone = None;
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::FieldUpdateTimeZone,
        reason: "field update time zone is inactive without a field update date-time",
      });
    } else {
      jiff::tz::TimeZone::get(time_zone).map_err(|error| {
        PdfError::Options(format!(
          "invalid IANA field update time zone {time_zone:?}: {error}"
        ))
      })?;
    }
  }
  Ok(())
}

fn canonicalize_locale(
  value: &mut Option<String>,
  feature: PdfOptionFeature,
  label: &str,
  adjustments: &mut Vec<PdfOptionAdjustment>,
) -> Result<()> {
  let Some(requested) = value.take() else {
    return Ok(());
  };
  let canonical = ooxmlsdk_layout::localization::canonical_office_locale_tag(Some(&requested))
    .ok_or_else(|| PdfError::Options(format!("invalid BCP 47 {label} {requested:?}")))?;
  if canonical != requested {
    adjustments.push(PdfOptionAdjustment {
      feature,
      reason: "the BCP 47 locale identifier was canonicalized",
    });
  }
  *value = Some(canonical);
  Ok(())
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

fn resolve_standards(
  options: &mut PdfOptions,
  adjustments: &mut Vec<PdfOptionAdjustment>,
) -> Result<()> {
  let mut version = None;
  let mut archival = None;
  let mut requests_pdf_ua = options.general.pdf_ua_compliance;
  let mut saw_pdf_ua_standard = false;
  let mut duplicate_version = false;
  let mut duplicate_archival = false;
  let mut duplicate_pdf_ua = false;

  for standard in mem::take(&mut options.standards) {
    if standard.is_version() {
      duplicate_version |= version == Some(standard);
      set_unique_standard(&mut version, standard, "PDF version")?;
    } else if standard.is_archival() {
      duplicate_archival |= archival == Some(standard);
      set_unique_standard(&mut archival, standard, "PDF/A conformance")?;
    } else {
      duplicate_pdf_ua |= requests_pdf_ua;
      saw_pdf_ua_standard = true;
      requests_pdf_ua = true;
    }
  }

  let mut standards = Vec::with_capacity(3);
  if let Some(version) = version {
    standards.push(version);
  }
  if let Some(archival) = archival {
    standards.push(archival);
    if archival.requires_tagging() && !options.general.tagged_pdf {
      options.general.tagged_pdf = true;
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::TaggedPdf,
        reason: "the requested PDF/A conformance requires tagged PDF",
      });
    }
  }
  if requests_pdf_ua {
    standards.push(PdfStandard::PdfUa1);
    if !options.general.pdf_ua_compliance {
      options.general.pdf_ua_compliance = true;
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::PdfUa,
        reason: "PdfUa1 and pdf_ua_compliance are canonicalized to one effective request",
      });
    } else if !saw_pdf_ua_standard {
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::PdfUa,
        reason: "pdf_ua_compliance was represented as the effective PdfUa1 standard",
      });
    }
    if !options.general.tagged_pdf {
      options.general.tagged_pdf = true;
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::TaggedPdf,
        reason: "PDF/UA requires tagged PDF",
      });
    }
    if !options.general.export_bookmarks {
      options.general.export_bookmarks = true;
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::Bookmarks,
        reason: "PDF/UA export keeps the document outline available",
      });
    }
    if !options.viewer.display_document_title {
      options.viewer.display_document_title = true;
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::ViewerPreferences,
        reason: "PDF/UA requires the viewer to display the document title",
      });
    }
  }
  if duplicate_version {
    adjustments.push(PdfOptionAdjustment {
      feature: PdfOptionFeature::PdfVersion,
      reason: "duplicate PDF version requests were removed",
    });
  }
  if duplicate_archival {
    adjustments.push(PdfOptionAdjustment {
      feature: PdfOptionFeature::PdfA,
      reason: "duplicate PDF/A requests were removed",
    });
  }
  if duplicate_pdf_ua {
    adjustments.push(PdfOptionAdjustment {
      feature: PdfOptionFeature::PdfUa,
      reason: "duplicate PDF/UA requests were removed",
    });
  }
  options.standards = standards;
  Ok(())
}

fn set_unique_standard(
  slot: &mut Option<PdfStandard>,
  standard: PdfStandard,
  family: &str,
) -> Result<()> {
  if let Some(previous) = *slot
    && previous != standard
  {
    return Err(PdfError::Options(format!(
      "{family} cannot request both {previous:?} and {standard:?}"
    )));
  }
  *slot = Some(standard);
  Ok(())
}

impl PdfStandard {
  const fn is_version(self) -> bool {
    matches!(
      self,
      Self::Pdf14 | Self::Pdf15 | Self::Pdf16 | Self::Pdf17 | Self::Pdf20
    )
  }

  const fn is_archival(self) -> bool {
    !self.is_version() && !matches!(self, Self::PdfUa1)
  }

  const fn requires_tagging(self) -> bool {
    matches!(self, Self::PdfA1a | Self::PdfA2a | Self::PdfA3a)
  }
}

fn resolve_images(
  options: &mut PdfOptions,
  adjustments: &mut Vec<PdfOptionAdjustment>,
) -> Result<()> {
  if let Some(quality) = options.jpeg_quality {
    if let Some(image_quality) = options.images.jpeg_quality
      && image_quality != quality
    {
      return Err(PdfError::Options(format!(
        "jpeg_quality ({quality}) conflicts with images.jpeg_quality ({image_quality})"
      )));
    }
    if options.images.jpeg_quality.is_none() {
      options.images.jpeg_quality = Some(quality);
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::ImageCompression,
        reason: "the compatibility jpeg_quality field was moved to images.jpeg_quality",
      });
    }
    options.jpeg_quality = None;
  }

  if let Some(quality) = options.images.jpeg_quality
    && !(1..=100).contains(&quality)
  {
    return Err(PdfError::Options(format!(
      "JPEG quality must be between 1 and 100, got {quality}"
    )));
  }

  if options.images.use_lossless_compression && options.images.jpeg_quality.take().is_some() {
    adjustments.push(PdfOptionAdjustment {
      feature: PdfOptionFeature::ImageCompression,
      reason: "lossless image compression makes JPEG quality inapplicable",
    });
  }

  if options.images.reduce_resolution {
    let dpi = options.images.max_resolution_dpi.unwrap_or(300);
    if !(51..=2400).contains(&dpi) {
      return Err(PdfError::Options(format!(
        "image downsampling DPI must be between 51 and 2400, got {dpi}"
      )));
    }
    options.images.max_resolution_dpi = Some(dpi);
  } else {
    let inactive_dpi = options.images.max_resolution_dpi.take();
    if inactive_dpi.is_some_and(|dpi| dpi != 300) {
      adjustments.push(PdfOptionAdjustment {
        feature: PdfOptionFeature::ImageDownsampling,
        reason: "maximum image resolution is inactive when downsampling is disabled",
      });
    }
  }
  Ok(())
}

fn resolve_bookmarks(
  options: &mut PdfOptions,
  adjustments: &mut Vec<PdfOptionAdjustment>,
) -> Result<()> {
  if let Some(levels) = options.general.open_bookmark_levels
    && levels < -1
  {
    return Err(PdfError::Options(format!(
      "open bookmark levels must be -1 or non-negative, got {levels}"
    )));
  }
  if !options.general.export_bookmarks && options.general.open_bookmark_levels.take().is_some() {
    adjustments.push(PdfOptionAdjustment {
      feature: PdfOptionFeature::Bookmarks,
      reason: "open bookmark levels are inactive when bookmark export is disabled",
    });
  }
  Ok(())
}

fn resolve_forms(
  document_kind: PdfDocumentKind,
  options: &mut PdfOptions,
  adjustments: &mut Vec<PdfOptionAdjustment>,
) -> Result<()> {
  if options.forms.export_form_fields && !matches!(document_kind, PdfDocumentKind::Docx) {
    return unsupported(
      document_kind,
      PdfOptionFeature::FormFields,
      "the current layout model exposes form widgets only for DOCX",
    );
  }
  if options.forms.export_form_fields {
    if !matches!(options.forms.submit_format, PdfFormSubmitFormat::Pdf) {
      return unsupported(
        document_kind,
        PdfOptionFeature::FormFields,
        "non-PDF form submit actions are not implemented",
      );
    }
    if options.forms.allow_duplicate_field_names {
      return unsupported(
        document_kind,
        PdfOptionFeature::FormFields,
        "duplicate AcroForm field names are not implemented",
      );
    }
  } else if !matches!(options.forms.submit_format, PdfFormSubmitFormat::Pdf)
    || options.forms.allow_duplicate_field_names
  {
    options.forms.submit_format = PdfFormSubmitFormat::Pdf;
    options.forms.allow_duplicate_field_names = false;
    adjustments.push(PdfOptionAdjustment {
      feature: PdfOptionFeature::FormFields,
      reason: "form sub-options are inactive when form field export is disabled",
    });
  }
  Ok(())
}

fn resolve_links(document_kind: PdfDocumentKind, options: &PdfOptions) -> Result<()> {
  if options.links.export_relative_filesystem_links {
    return unsupported(
      document_kind,
      PdfOptionFeature::Links,
      "relative filesystem link rewriting needs an explicit source/output base path",
    );
  }
  if matches!(
    options.links.default_action,
    PdfLinkDefaultAction::UriDestination | PdfLinkDefaultAction::Launch
  ) {
    return unsupported(
      document_kind,
      PdfOptionFeature::Links,
      "remote-destination and launch actions are not available in the current PDF backend",
    );
  }
  Ok(())
}

fn resolve_viewer(options: &PdfOptions) -> Result<()> {
  if options.viewer.initial_page == 0 {
    return Err(PdfError::Options(
      "viewer initial page is one-based and must be at least 1".to_string(),
    ));
  }
  if let PdfViewerMagnification::Zoom(zoom) = options.viewer.magnification
    && (!zoom.is_finite() || !(0.5..=16.0).contains(&zoom))
  {
    return Err(PdfError::Options(format!(
      "viewer zoom must be a finite scale between 0.5 and 16.0, got {zoom}"
    )));
  }
  if options.viewer.first_page_left
    && !matches!(options.viewer.page_layout, PdfPageLayout::ContinuousFacing)
  {
    return Err(PdfError::Options(
      "first_page_left is only meaningful with ContinuousFacing page layout".to_string(),
    ));
  }
  Ok(())
}

fn resolve_metadata(options: &PdfOptions) -> Result<()> {
  if options
    .standards
    .iter()
    .any(|standard| standard.is_archival())
    && options.metadata.creation_date.is_none()
  {
    return Err(PdfError::Options(
      "PDF/A export requires a deterministic metadata creation date".to_string(),
    ));
  }
  if let Some(date) = options.metadata.creation_date {
    validate_pdf_date_time(date, "metadata creation date")?;
  }
  for attachment in &options.attachments {
    if let Some(date) = attachment.modification_date {
      validate_pdf_date_time(date, "attachment modification date")?;
    }
  }
  Ok(())
}

fn validate_pdf_date_time(value: PdfDateTime, label: &str) -> Result<()> {
  if value.year == 0 || value.year > 9999 {
    return Err(PdfError::Options(format!(
      "{label} year must be between 1 and 9999, got {}",
      value.year
    )));
  }
  if value.month.is_none()
    && (value.day.is_some()
      || value.hour.is_some()
      || value.minute.is_some()
      || value.second.is_some())
  {
    return Err(PdfError::Options(format!(
      "{label} components must form a contiguous year-to-second prefix"
    )));
  }
  if value.day.is_none()
    && (value.hour.is_some() || value.minute.is_some() || value.second.is_some())
  {
    return Err(PdfError::Options(format!(
      "{label} components must form a contiguous year-to-second prefix"
    )));
  }
  if value.hour.is_none() && (value.minute.is_some() || value.second.is_some()) {
    return Err(PdfError::Options(format!(
      "{label} components must form a contiguous year-to-second prefix"
    )));
  }
  if value.minute.is_none() && value.second.is_some() {
    return Err(PdfError::Options(format!(
      "{label} components must form a contiguous year-to-second prefix"
    )));
  }
  if value.utc_offset_hour.is_none() && value.utc_offset_minute.is_some() {
    return Err(PdfError::Options(format!(
      "{label} UTC offset minutes require an offset hour"
    )));
  }
  if value.utc_offset_hour.is_some() && value.hour.is_none() {
    return Err(PdfError::Options(format!(
      "{label} UTC offset requires a time component"
    )));
  }
  if let Some(hour) = value.utc_offset_hour
    && !(-23..=23).contains(&hour)
  {
    return Err(PdfError::Options(format!(
      "{label} UTC offset hour must be between -23 and 23, got {hour}"
    )));
  }
  if let Some(minute) = value.utc_offset_minute
    && minute > 59
  {
    return Err(PdfError::Options(format!(
      "{label} UTC offset minute must be between 0 and 59, got {minute}"
    )));
  }

  let year = i16::try_from(value.year).expect("validated four-digit PDF year fits i16");
  let component = |name: &str, value: u8| {
    i8::try_from(value)
      .map_err(|_| PdfError::Options(format!("{label} {name} is out of range: {value}")))
  };
  let month = component("month", value.month.unwrap_or(1))?;
  let day = component("day", value.day.unwrap_or(1))?;
  let hour = component("hour", value.hour.unwrap_or(0))?;
  let minute = component("minute", value.minute.unwrap_or(0))?;
  let second = component("second", value.second.unwrap_or(0))?;
  jiff::civil::DateTime::new(year, month, day, hour, minute, second, 0)
    .map_err(|error| PdfError::Options(format!("invalid {label}: {error}")))?;
  Ok(())
}

fn reject_unimplemented_options(
  document_kind: PdfDocumentKind,
  options: &PdfOptions,
) -> Result<()> {
  if options.general.skip_empty_pages {
    return unsupported(
      document_kind,
      PdfOptionFeature::SkipEmptyPages,
      "the layout model does not yet distinguish application-inserted blank pages",
    );
  }
  if options.general.remove_transparencies {
    return unsupported(
      document_kind,
      PdfOptionFeature::TransparencyFlattening,
      "transparent groups do not yet have a document-wide flattening pass",
    );
  }
  if options.watermark.is_some() {
    return unsupported(
      document_kind,
      PdfOptionFeature::Watermark,
      "watermarks need shaped Unicode text and archival-safe font embedding",
    );
  }
  if options.spreadsheet.single_page_sheets {
    return unsupported(
      document_kind,
      PdfOptionFeature::SinglePageSheets,
      "single-page sheets require spreadsheet pagination and drawing-scale integration",
    );
  }
  Ok(())
}

fn unsupported<T>(
  document_kind: PdfDocumentKind,
  feature: PdfOptionFeature,
  reason: &'static str,
) -> Result<T> {
  Err(PdfError::UnsupportedOption {
    feature,
    document_kind,
    reason,
  })
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
  /// Deterministic creation date written to both PDF document information and XMP metadata.
  ///
  /// The current backend also uses this value as the modification date so the
  /// two representations remain consistent for PDF/A validation.
  pub creation_date: Option<PdfDateTime>,
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pdf_ua_request_is_resolved_to_effective_tagging_and_outline_options() {
    let mut options = PdfOptions::default();
    options.standards.push(PdfStandard::PdfUa1);
    options.general.export_bookmarks = false;
    options.viewer.display_document_title = false;

    let resolved = options.resolve_for(PdfDocumentKind::Docx).unwrap();

    assert_eq!(resolved.effective.standards, vec![PdfStandard::PdfUa1]);
    assert!(resolved.effective.general.pdf_ua_compliance);
    assert!(resolved.effective.general.tagged_pdf);
    assert!(resolved.effective.general.export_bookmarks);
    assert!(resolved.effective.viewer.display_document_title);
    assert!(
      resolved
        .adjustments
        .iter()
        .any(|adjustment| adjustment.feature == PdfOptionFeature::TaggedPdf)
    );
  }

  #[test]
  fn inactive_image_suboptions_are_removed_from_effective_configuration() {
    let mut options = PdfOptions::default();
    options.images.max_resolution_dpi = Some(600);

    let resolved = options.resolve_for(PdfDocumentKind::Pptx).unwrap();

    assert_eq!(resolved.effective.images.max_resolution_dpi, None);
    assert!(
      resolved
        .adjustments
        .iter()
        .any(|adjustment| adjustment.feature == PdfOptionFeature::ImageDownsampling)
    );
  }

  #[test]
  fn lossless_images_remove_inapplicable_jpeg_quality() {
    let mut options = PdfOptions::default();
    options.images.use_lossless_compression = true;
    options.images.jpeg_quality = Some(80);

    let resolved = options.resolve_for(PdfDocumentKind::Xlsx).unwrap();

    assert_eq!(resolved.effective.images.jpeg_quality, None);
    assert!(
      resolved
        .adjustments
        .iter()
        .any(|adjustment| adjustment.feature == PdfOptionFeature::ImageCompression)
    );
  }

  #[test]
  fn layout_environment_is_canonicalized_and_records_ui_resource_fallback() {
    let mut options = PdfOptions {
      ui_language: Some("de_de".to_string()),
      format_locale: Some("fr_fr".to_string()),
      default_document_language: Some("ja_jp".to_string()),
      ..Default::default()
    };
    options.field_update_datetime = Some(ooxmlsdk_layout::options::FieldUpdateDateTime {
      year: 2024,
      month: 2,
      day: 29,
      hour: 23,
      minute: 59,
      second: 58,
    });
    options.field_update_time_zone = Some("Asia/Shanghai".to_string());

    let resolved = options.resolve_for(PdfDocumentKind::Docx).unwrap();

    assert_eq!(resolved.effective.ui_language.as_deref(), Some("de-DE"));
    assert_eq!(resolved.effective.format_locale.as_deref(), Some("fr-FR"));
    assert_eq!(
      resolved.effective.default_document_language.as_deref(),
      Some("ja-JP")
    );
    assert!(resolved.adjustments.iter().any(|adjustment| {
      adjustment.feature == PdfOptionFeature::UiLanguage
        && adjustment.reason.contains("fall back to English")
    }));
  }

  #[test]
  fn invalid_layout_environment_values_are_rejected_before_conversion() {
    let invalid_locale = PdfOptions {
      format_locale: Some("not a locale".to_string()),
      ..Default::default()
    };
    assert!(matches!(
      invalid_locale.resolve_for(PdfDocumentKind::Xlsx),
      Err(PdfError::Options(message)) if message.contains("invalid BCP 47 format locale")
    ));

    let invalid_date = PdfOptions {
      field_update_datetime: Some(ooxmlsdk_layout::options::FieldUpdateDateTime {
        year: 2023,
        month: 2,
        day: 29,
        hour: 0,
        minute: 0,
        second: 0,
      }),
      ..Default::default()
    };
    assert!(matches!(
      invalid_date.resolve_for(PdfDocumentKind::Docx),
      Err(PdfError::Options(message)) if message.contains("invalid field update date-time")
    ));

    let invalid_time_zone = PdfOptions {
      field_update_datetime: Some(ooxmlsdk_layout::options::FieldUpdateDateTime {
        year: 2024,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
      }),
      field_update_time_zone: Some("Mars/Olympus_Mons".to_string()),
      ..Default::default()
    };
    assert!(matches!(
      invalid_time_zone.resolve_for(PdfDocumentKind::Pptx),
      Err(PdfError::Options(message)) if message.contains("invalid IANA field update time zone")
    ));
  }

  #[test]
  fn pdf_a_requires_a_valid_deterministic_document_date() {
    let mut missing = PdfOptions::default();
    missing.standards.push(PdfStandard::PdfA3a);
    assert!(matches!(
      missing.resolve_for(PdfDocumentKind::Docx),
      Err(PdfError::Options(message)) if message.contains("requires a deterministic metadata creation date")
    ));

    let mut valid = PdfOptions::default();
    valid.standards.push(PdfStandard::PdfA3a);
    valid.metadata.creation_date = Some(PdfDateTime {
      year: 2026,
      month: Some(8),
      day: Some(17),
      hour: Some(12),
      minute: Some(30),
      second: Some(45),
      utc_offset_hour: Some(8),
      utc_offset_minute: Some(0),
    });
    assert!(valid.resolve_for(PdfDocumentKind::Docx).is_ok());

    let mut invalid = PdfOptions::default();
    invalid.metadata.creation_date = Some(PdfDateTime {
      year: 2025,
      month: Some(2),
      day: Some(29),
      hour: None,
      minute: None,
      second: None,
      utc_offset_hour: None,
      utc_offset_minute: None,
    });
    assert!(matches!(
      invalid.resolve_for(PdfDocumentKind::Pptx),
      Err(PdfError::Options(message)) if message.contains("invalid metadata creation date")
    ));
  }

  #[test]
  fn inactive_field_update_time_zone_is_removed_from_effective_configuration() {
    let options = PdfOptions {
      field_update_time_zone: Some("Asia/Shanghai".to_string()),
      ..Default::default()
    };

    let resolved = options.resolve_for(PdfDocumentKind::Docx).unwrap();

    assert_eq!(resolved.effective.field_update_time_zone, None);
    assert!(
      resolved
        .adjustments
        .iter()
        .any(|adjustment| { adjustment.feature == PdfOptionFeature::FieldUpdateTimeZone })
    );
  }

  #[test]
  fn xlsx_form_export_is_a_typed_unsupported_option() {
    let mut options = PdfOptions::default();
    options.forms.export_form_fields = true;

    assert!(matches!(
      options.resolve_for(PdfDocumentKind::Xlsx),
      Err(PdfError::UnsupportedOption {
        feature: PdfOptionFeature::FormFields,
        document_kind: PdfDocumentKind::Xlsx,
        ..
      })
    ));
  }

  #[test]
  fn unimplemented_watermark_is_not_silently_ignored() {
    let options = PdfOptions {
      watermark: Some(PdfWatermarkOptions {
        text: "Draft".to_string(),
        color_rgb: None,
        font_name: None,
        font_height_pt: None,
        rotate_degrees: None,
        tiled_text: None,
      }),
      ..Default::default()
    };

    assert!(matches!(
      options.resolve_for(PdfDocumentKind::Docx),
      Err(PdfError::UnsupportedOption {
        feature: PdfOptionFeature::Watermark,
        ..
      })
    ));
  }

  #[test]
  fn capability_report_matches_document_specific_form_support() {
    assert_eq!(
      pdf_option_support(PdfDocumentKind::Docx, PdfOptionFeature::FormFields),
      PdfOptionSupport::SupportedWithRestrictions
    );
    assert_eq!(
      pdf_option_support(PdfDocumentKind::Pptx, PdfOptionFeature::FormFields),
      PdfOptionSupport::Unsupported
    );
    assert_eq!(
      pdf_option_support(PdfDocumentKind::Xlsx, PdfOptionFeature::PageRange),
      PdfOptionSupport::Supported
    );
    assert_eq!(
      pdf_option_support(PdfDocumentKind::Docx, PdfOptionFeature::UiLanguage),
      PdfOptionSupport::SupportedWithRestrictions
    );
    assert_eq!(
      pdf_option_support(PdfDocumentKind::Xlsx, PdfOptionFeature::PaperSize),
      PdfOptionSupport::Unsupported
    );
  }
}
