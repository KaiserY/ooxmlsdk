mod chart;
mod custom_geometry;
mod display;
pub(crate) mod drawingml;
mod gradient;
mod import;
mod presentation;
mod preset_geometry;
mod shadow;
mod shape;
mod shape_context;
mod shape_group_context;
mod slide;
mod slide_fragment;

use ooxmlsdk::parts::presentation_document::PresentationDocument;

use crate::error::Result;
use crate::options::LayoutOptions;

use import::PowerPointImport;

#[derive(Clone, Debug, Default)]
pub struct PptxLayoutSummary {
  pub is_endless: bool,
  pub is_automatic: bool,
  pub first_page_name: Option<String>,
  pub custom_show_name: Option<String>,
  pub embed_true_type_fonts: bool,
  pub save_subset_fonts: bool,
  pub embedded_font_typefaces: Vec<String>,
  pub notes_page_shape_counts: Vec<usize>,
  pub draw_page_shape_counts: Vec<usize>,
  pub draw_shapes: Vec<PptxDrawShapeSummary>,
  pub master_text_shapes: Vec<PptxTextShapeSummary>,
  pub smartart_text_shapes: Vec<PptxSmartArtTextShapeSummary>,
  pub bullet_paragraphs: Vec<PptxBulletParagraphSummary>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxDrawShapeSummary {
  pub page_index: usize,
  pub shape_path: Vec<usize>,
  pub service_name: String,
  pub geometry: Option<String>,
  pub text: String,
  pub left_100mm: i32,
  pub top_100mm: i32,
  pub right_100mm: i32,
  pub bottom_100mm: i32,
  pub width_100mm: i32,
  pub height_100mm: i32,
  pub fill_style: String,
  pub fill_uses_slide_background: bool,
  pub gradient_style: Option<String>,
  pub gradient_angle: Option<i16>,
  pub text_left_distance_100mm: Option<i32>,
  pub text_upper_distance_100mm: Option<i32>,
  pub text_right_distance_100mm: Option<i32>,
  pub text_lower_distance_100mm: Option<i32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxTextShapeSummary {
  pub master_page_index: usize,
  pub shape_index: usize,
  pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxSmartArtTextShapeSummary {
  pub page_index: usize,
  pub text: String,
  pub text_left_distance_100mm: i32,
  pub text_upper_distance_100mm: i32,
  pub text_anchor_left_100mm: i32,
  pub text_anchor_top_100mm: i32,
  pub text_anchor_right_100mm: i32,
  pub text_anchor_bottom_100mm: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PptxBulletParagraphSummary {
  pub page_index: usize,
  pub paragraph_index: usize,
  pub text: String,
  pub character: Option<String>,
  pub font: Option<String>,
  pub graphic_width_100mm: Option<i32>,
  pub graphic_height_100mm: Option<i32>,
}

pub fn layout(
  package: &mut PresentationDocument,
  options: &LayoutOptions,
) -> Result<crate::common::LayoutDocument<'static>> {
  layout_document(package, options)
}

pub fn layout_document(
  package: &mut PresentationDocument,
  options: &LayoutOptions,
) -> Result<crate::common::LayoutDocument<'static>> {
  let import = PowerPointImport::import_document(package)?;
  let mut document = display::lower_to_layout_document(&import, options);
  if options.diagnostics.collect_debug_records {
    document
      .debug_records
      .extend(display::debug_records(&import));
  }
  Ok(document)
}

pub fn inspect_layout(package: &mut PresentationDocument) -> Result<PptxLayoutSummary> {
  let import = PowerPointImport::import_document(package)?;
  Ok(display::inspect_layout_summary(&import))
}
