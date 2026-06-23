//! PDF conversion support for Open XML packages.
//!
//! The crate is intentionally split from layout:
//!
//! 1. `ooxmlsdk-layout` extracts and lays out Office packages.
//! 2. This crate converts the layout display list to PDF through `krilla`.

mod error;
mod options;
mod render;
mod xlsx;

use std::io::{Read, Seek};

use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::sdk::{
  FileFormatVersion, MarkupCompatibilityProcessMode, MarkupCompatibilityProcessSettings,
  OpenSettings,
};

pub use error::{PdfError, Result};
pub use ooxmlsdk_layout::docx::{DocxLayoutLineSummary, DocxLayoutRowSummary, DocxLayoutSummary};
pub use ooxmlsdk_layout::pptx::{
  PptxBulletParagraphSummary, PptxDrawShapeSummary, PptxLayoutSummary,
  PptxSmartArtTextShapeSummary, PptxTextShapeSummary,
};
pub use options::{
  PdfFormOptions, PdfFormSubmitFormat, PdfGeneralOptions, PdfImageOptions, PdfLinkDefaultAction,
  PdfLinkOptions, PdfMetadataOptions, PdfOptions, PdfPageLayout, PdfSpreadsheetOptions,
  PdfStandard, PdfViewerMagnification, PdfViewerOptions, PdfViewerPageMode, PdfWatermarkOptions,
};

/// Convert a DOCX stream into PDF bytes.
pub fn convert_docx<R>(reader: R, options: PdfOptions) -> Result<Vec<u8>>
where
  R: Read + Seek,
{
  let settings = OpenSettings {
    markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
      process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
      target_file_format_version: FileFormatVersion::Microsoft365,
    },
    ..Default::default()
  };
  let mut document = WordprocessingDocument::new_with_settings(reader, settings)?;
  convert_wordprocessing_document(&mut document, options)
}

/// Convert an opened Wordprocessing document into PDF bytes.
pub fn convert_wordprocessing_document(
  document: &mut WordprocessingDocument,
  mut options: PdfOptions,
) -> Result<Vec<u8>> {
  let layout_options = options.take_layout_options();
  let pages = ooxmlsdk_layout::docx::layout_document(document, &layout_options)?;
  render::krilla::render(&pages, &options)
}

/// Inspect DOCX layout line boxes without rendering to PDF.
///
/// This mirrors upstream layout-dump tests that assert line metrics directly
/// instead of going through PDF text extraction.
pub fn inspect_docx_layout<R>(reader: R, options: PdfOptions) -> Result<DocxLayoutSummary>
where
  R: Read + Seek,
{
  let mut options = options;
  let settings = OpenSettings {
    markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
      process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
      target_file_format_version: FileFormatVersion::Microsoft365,
    },
    ..Default::default()
  };
  let mut document = WordprocessingDocument::new_with_settings(reader, settings)?;
  let mut layout_options = options.take_layout_options();
  layout_options.diagnostics = ooxmlsdk_layout::options::LayoutDiagnosticsOptions {
    collect_debug_records: true,
    collect_reflow_records: true,
    preserve_source_links: true,
  };
  Ok(ooxmlsdk_layout::docx::inspect_layout(
    &mut document,
    &layout_options,
  )?)
}

/// Inspect PPTX SmartArt layout without rendering to PDF.
///
/// This mirrors upstream SmartArt import tests that assert shape text
/// distances and `TakeTextAnchorRect()` values directly.
pub fn inspect_pptx_layout<R>(reader: R, _options: PdfOptions) -> Result<PptxLayoutSummary>
where
  R: Read + Seek,
{
  let settings = OpenSettings {
    markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
      process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
      target_file_format_version: FileFormatVersion::Microsoft365,
    },
    ..Default::default()
  };
  let mut document = PresentationDocument::new_with_settings(reader, settings)?;
  ooxmlsdk_layout::pptx::inspect_layout(&mut document).map_err(Into::into)
}

/// Convert an XLSX stream into PDF bytes.
pub fn convert_xlsx<R>(reader: R, options: PdfOptions) -> Result<Vec<u8>>
where
  R: Read + Seek,
{
  let settings = OpenSettings {
    markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
      process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
      target_file_format_version: FileFormatVersion::Microsoft365,
    },
    ..Default::default()
  };
  let mut document = SpreadsheetDocument::new_with_settings(reader, settings)?;
  convert_spreadsheet_document(&mut document, options)
}

/// Convert an opened spreadsheet document into PDF bytes.
pub fn convert_spreadsheet_document(
  document: &mut SpreadsheetDocument,
  mut options: PdfOptions,
) -> Result<Vec<u8>> {
  let pages = xlsx::layout(document, &mut options)?;
  render::krilla::render(&pages, &options)
}

/// Convert a PPTX stream into PDF bytes.
pub fn convert_pptx<R>(reader: R, options: PdfOptions) -> Result<Vec<u8>>
where
  R: Read + Seek,
{
  let settings = OpenSettings {
    markup_compatibility_process_settings: MarkupCompatibilityProcessSettings {
      process_mode: MarkupCompatibilityProcessMode::ProcessLoadedPartsOnly,
      target_file_format_version: FileFormatVersion::Microsoft365,
    },
    ..Default::default()
  };
  let mut document = PresentationDocument::new_with_settings(reader, settings)?;
  convert_presentation_document(&mut document, options)
}

/// Convert an opened presentation document into PDF bytes.
pub fn convert_presentation_document(
  document: &mut PresentationDocument,
  mut options: PdfOptions,
) -> Result<Vec<u8>> {
  let layout_options = options.take_layout_options();
  let pages = ooxmlsdk_layout::pptx::layout_document(document, &layout_options)?;
  render::krilla::render(&pages, &options)
}
