mod display;
mod drawingml;
mod graphic_shape_context;
mod import;
mod layout_fragment;
mod presentation;
mod shape;
mod shape_context;
mod shape_group_context;
mod shape_properties_context;
mod slide;
mod slide_fragment;

use ooxmlsdk::parts::presentation_document::PresentationDocument;

use crate::PptxLayoutSummary;
use crate::error::Result;
use crate::layout::LayoutDocument;
use crate::options::PdfOptions;

use import::PowerPointImport;

pub(crate) fn layout(
  package: &mut PresentationDocument,
  _options: &PdfOptions,
) -> Result<LayoutDocument> {
  let import = PowerPointImport::import_document(package)?;
  Ok(display::lower_to_layout_document(&import))
}

pub(crate) fn inspect_layout(package: &mut PresentationDocument) -> Result<PptxLayoutSummary> {
  let import = PowerPointImport::import_document(package)?;
  Ok(display::inspect_layout_summary(&import))
}
