mod direct;
mod direct_attachment;
mod direct_conformance;
mod direct_font;
mod direct_form;
mod direct_glyph;
mod direct_gradient;
mod direct_image;
mod direct_link;
mod direct_metadata;
mod direct_outline;
mod direct_path_gradient;
mod direct_pattern;
mod direct_svg;
mod direct_tag;
mod gradient;
mod image;
use ooxmlsdk_layout::render::jpeg_islow;
mod jpeg_islow_encoder;
mod link;
mod marker;
mod metafile;
mod native_png;
mod page;
pub(crate) mod paint;
pub(crate) mod settings;

use crate::error::Result;
use crate::{PdfConversionOutput, PdfFontAuditOutput, PdfOptions};
use ooxmlsdk_layout::common;

pub(crate) fn render(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<Vec<u8>> {
  direct::render(document, options)
}

pub(crate) fn render_with_diagnostics(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<PdfConversionOutput> {
  direct::render_with_diagnostics(document, options)
}

pub(crate) fn render_with_font_audit(
  document: &common::LayoutDocument<'static>,
  options: &PdfOptions,
) -> Result<PdfFontAuditOutput> {
  direct::render_with_font_audit(document, options)
}
