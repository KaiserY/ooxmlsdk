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

use crate::error::Result;
use crate::layout::{self, LayoutDocument};
use crate::options::PdfOptions;

use import::PowerPointImport;

pub(crate) fn layout(
  package: &mut PresentationDocument,
  _options: &PdfOptions,
) -> Result<LayoutDocument> {
  let import = PowerPointImport::import_document(package)?;
  let setups = import
    .draw_pages
    .iter()
    .map(|slide| slide.size.to_page_setup())
    .collect();

  Ok(layout::fixed_pages(setups))
}
