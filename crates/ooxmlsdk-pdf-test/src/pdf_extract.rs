use std::io::Read;
use std::process::Command;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

use flate2::read::ZlibDecoder;
use image::GenericImageView;
use lopdf::{Document as LopdfDocument, Object as LopdfObject};
use pdfium_render::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PdfSummary {
  pub page_count: usize,
  pub image_count: usize,
  pub link_annotation_count: usize,
  pub annotation_count: usize,
  pub outline_marker_count: usize,
  pub media_box_count: usize,
  pub contains_eof: bool,
  pub media_boxes: Vec<String>,
  pub text: Option<String>,
  pub text_error: Option<String>,
  pub text_segments: Vec<TextSegmentSummary>,
  pub text_chars: Vec<TextCharSummary>,
  pub text_objects: Vec<TextObjectSummary>,
  pub images: Vec<ImageSummary>,
  pub paths: Vec<PathObjectSummary>,
  pub links: Vec<LinkSummary>,
  pub annotations: Vec<AnnotationSummary>,
  pub outlines: Vec<String>,
  pub raw_pages: Vec<RawPageSummary>,
  pub page_objects: Vec<PageObjectSummary>,
  pub rendered_pages: Vec<RenderedPageSummary>,
  pub content: ContentSummary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageSummary {
  pub page_index: usize,
  pub width: Option<String>,
  pub height: Option<String>,
  pub bounds: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathObjectSummary {
  pub page_index: usize,
  pub segments: u32,
  pub fill_mode: Option<String>,
  pub stroked: Option<bool>,
  pub fill_color: Option<String>,
  pub stroke_color: Option<String>,
  pub bounds: Option<String>,
  pub segment_details: Vec<PathSegmentSummary>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathSegmentSummary {
  pub segment_type: String,
  pub x: String,
  pub y: String,
  pub closed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkSummary {
  pub page_index: usize,
  pub target_kind: LinkTargetKind,
  pub target: Option<String>,
  pub rect: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnnotationSummary {
  pub page_index: usize,
  pub annotation_type: String,
  pub bounds: Option<String>,
  pub action_uri: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RawPageSummary {
  pub page_index: usize,
  pub annotation_count: usize,
  pub annotations: Vec<RawAnnotationSummary>,
  pub xobjects: Vec<RawXObjectSummary>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawAnnotationSummary {
  pub page_index: usize,
  pub type_name: Option<String>,
  pub subtype_name: Option<String>,
  pub rect: Option<String>,
  pub action_uri: Option<String>,
  pub field_type_name: Option<String>,
  pub field_value: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawXObjectSummary {
  pub page_index: usize,
  pub name: String,
  pub type_name: Option<String>,
  pub subtype_name: Option<String>,
  pub filter_names: Vec<String>,
  pub width_px: Option<u32>,
  pub height_px: Option<u32>,
  pub image_format: Option<String>,
  pub decoded_width_px: Option<u32>,
  pub decoded_height_px: Option<u32>,
  pub bits_per_pixel: Option<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LinkTargetKind {
  ExternalUri,
  InternalDestination,
  Action,
  Unknown,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ContentSummary {
  pub stream_count: usize,
  pub decoded_stream_count: usize,
  pub text_show_ops: usize,
  pub image_draw_ops: usize,
  pub path_paint_ops: usize,
  pub clipping_ops: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextSegmentSummary {
  pub page_index: usize,
  pub text: String,
  pub bounds: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextCharSummary {
  pub page_index: usize,
  pub text: String,
  pub bounds: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextObjectSummary {
  pub page_index: usize,
  pub text: String,
  pub font_name: String,
  pub font_family: String,
  pub scaled_font_size: String,
  pub unscaled_font_size: String,
  pub render_mode: String,
  pub fill_color: Option<String>,
  pub stroke_color: Option<String>,
  pub bounds: Option<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PageObjectSummary {
  pub page_index: usize,
  pub text_objects: usize,
  pub path_objects: usize,
  pub image_objects: usize,
  pub shading_objects: usize,
  pub form_objects: usize,
  pub unsupported_objects: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedPageSummary {
  pub page_index: usize,
  pub width_px: u32,
  pub height_px: u32,
  pub rgba_crc32: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PdfBounds {
  pub left: f32,
  pub bottom: f32,
  pub right: f32,
  pub top: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PixelRect {
  pub left: u32,
  pub top: u32,
  pub width: u32,
  pub height: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RenderedPageImage {
  pub page_index: usize,
  pub width_px: u32,
  pub height_px: u32,
  pub page_width_pt: f32,
  pub page_height_pt: f32,
  pub rgba_crc32: String,
  pub rgba: Vec<u8>,
}

impl PdfBounds {
  pub fn width(self) -> f32 {
    self.right - self.left
  }

  pub fn height(self) -> f32 {
    self.top - self.bottom
  }

  pub fn center(self) -> (f32, f32) {
    (
      self.left + self.width() / 2.0,
      self.bottom + self.height() / 2.0,
    )
  }
}

impl RenderedPageImage {
  pub fn pixel_rgba(&self, x: u32, y: u32) -> Option<[u8; 4]> {
    if x >= self.width_px || y >= self.height_px {
      return None;
    }
    let offset = ((y * self.width_px + x) * 4) as usize;
    Some([
      self.rgba[offset],
      self.rgba[offset + 1],
      self.rgba[offset + 2],
      self.rgba[offset + 3],
    ])
  }

  pub fn sample_pdf_point_rgba(&self, x_pt: f32, y_pt: f32) -> Option<[u8; 4]> {
    let (x, y) = self.pdf_point_to_pixel(x_pt, y_pt)?;
    self.pixel_rgba(x, y)
  }

  pub fn sample_pdf_rect_center_rgba(&self, rect: PdfBounds) -> Option<[u8; 4]> {
    let (x, y) = rect.center();
    self.sample_pdf_point_rgba(x, y)
  }

  pub fn pixel_region_crc32(&self, rect: PixelRect) -> Option<String> {
    if rect.width == 0
      || rect.height == 0
      || rect.left >= self.width_px
      || rect.top >= self.height_px
      || rect.left + rect.width > self.width_px
      || rect.top + rect.height > self.height_px
    {
      return None;
    }

    let mut crc = crc32fast::Hasher::new();
    for y in rect.top..rect.top + rect.height {
      let start = ((y * self.width_px + rect.left) * 4) as usize;
      let end = start + (rect.width * 4) as usize;
      crc.update(&self.rgba[start..end]);
    }
    Some(format!("{:08x}", crc.finalize()))
  }

  pub fn pdf_rect_crc32(&self, rect: PdfBounds) -> Option<String> {
    self.pixel_region_crc32(self.pdf_rect_to_pixel_rect(rect)?)
  }

  pub fn pdf_point_to_pixel(&self, x_pt: f32, y_pt: f32) -> Option<(u32, u32)> {
    if !(0.0..=self.page_width_pt).contains(&x_pt) || !(0.0..=self.page_height_pt).contains(&y_pt) {
      return None;
    }

    let x = (x_pt / self.page_width_pt * self.width_px as f32).floor() as u32;
    let y =
      ((self.page_height_pt - y_pt) / self.page_height_pt * self.height_px as f32).floor() as u32;
    Some((x.min(self.width_px - 1), y.min(self.height_px - 1)))
  }

  pub fn pdf_rect_to_pixel_rect(&self, rect: PdfBounds) -> Option<PixelRect> {
    if rect.width() <= 0.0 || rect.height() <= 0.0 {
      return None;
    }
    let (left, top) = self.pdf_point_to_pixel(rect.left, rect.top)?;
    let (right, bottom) = self.pdf_point_to_pixel(rect.right, rect.bottom)?;
    Some(PixelRect {
      left,
      top,
      width: right.saturating_sub(left).max(1),
      height: bottom.saturating_sub(top).max(1),
    })
  }
}

impl PdfSummary {
  pub fn from_bytes(pdf: &[u8]) -> Result<Self, String> {
    let text = String::from_utf8_lossy(pdf);
    let streams = pdf_streams(pdf);
    let pdfium_summary = pdfium_summary(pdf)?;
    let raw_summary = raw_pdf_summary(pdf)?;
    let pdftotext = pdftotext(pdf);
    Ok(Self {
      page_count: pdfium_summary.page_count,
      image_count: pdfium_summary.images.len(),
      link_annotation_count: pdfium_summary.links.len(),
      annotation_count: pdfium_summary.annotations.len(),
      outline_marker_count: text.matches("/Outlines").count() + text.matches("/Title").count(),
      media_box_count: pdfium_summary.media_boxes.len(),
      contains_eof: pdf.strip_suffix_ascii_whitespace().ends_with(b"%%EOF"),
      media_boxes: pdfium_summary.media_boxes,
      text: pdftotext.clone().ok(),
      text_error: pdftotext.err(),
      text_segments: pdfium_summary.text_segments,
      text_chars: pdfium_summary.text_chars,
      text_objects: pdfium_summary.text_objects,
      images: pdfium_summary.images,
      paths: pdfium_summary.paths,
      links: pdfium_summary.links,
      annotations: pdfium_summary.annotations,
      outlines: raw_summary.outlines,
      raw_pages: raw_summary.pages,
      page_objects: pdfium_summary.page_objects,
      rendered_pages: pdfium_summary.rendered_pages,
      content: content_summary(&streams),
    })
  }
}

pub fn pdf_page_count(pdf: &[u8]) -> Result<usize, String> {
  let document = LopdfDocument::load_mem(pdf)
    .map_err(|error| format!("lopdf could not load PDF bytes: {error}"))?;
  Ok(document.get_pages().len())
}

pub fn parse_pdf_rect(rect: &str) -> Result<PdfBounds, String> {
  let trimmed = rect
    .trim()
    .strip_prefix('[')
    .and_then(|value| value.strip_suffix(']'))
    .ok_or_else(|| format!("invalid PDF rect format: {rect}"))?;
  let values = trimmed
    .split_ascii_whitespace()
    .map(|value| {
      value
        .parse::<f32>()
        .map_err(|error| format!("invalid PDF rect coordinate {value:?}: {error}"))
    })
    .collect::<Result<Vec<_>, _>>()?;
  if values.len() != 4 {
    return Err(format!(
      "expected four PDF rect coordinates, got {} in {rect}",
      values.len()
    ));
  }
  Ok(PdfBounds {
    left: values[0],
    bottom: values[1],
    right: values[2],
    top: values[3],
  })
}

pub fn assert_pdf_rect_close(actual: &str, expected: PdfBounds, tolerance: f32) {
  let actual = parse_pdf_rect(actual).unwrap();
  for (name, actual_value, expected_value) in [
    ("left", actual.left, expected.left),
    ("bottom", actual.bottom, expected.bottom),
    ("right", actual.right, expected.right),
    ("top", actual.top, expected.top),
  ] {
    assert!(
      (actual_value - expected_value).abs() <= tolerance,
      "PDF rect {name} mismatch: actual={actual:?} expected={expected:?} tolerance={tolerance}"
    );
  }
}

pub fn rendered_page_image_from_pdf(
  pdf: &[u8],
  page_index: usize,
  target_width: i32,
) -> Result<RenderedPageImage, String> {
  let _guard = pdfium_lock().lock().unwrap();
  let pdfium = bind_pdfium()?;
  let document = pdfium
    .load_pdf_from_byte_vec(pdf.to_vec(), None)
    .map_err(|error| format!("PDFium could not load PDF bytes: {error}"))?;

  for (current_page_index, page) in document.pages().iter().enumerate() {
    if current_page_index == page_index {
      return rendered_page_image(page_index, &page, target_width);
    }
  }

  Err(format!(
    "PDF page index {page_index} is out of range; page count is {}",
    document.pages().len()
  ))
}

struct PdfiumSummary {
  page_count: usize,
  media_boxes: Vec<String>,
  text_segments: Vec<TextSegmentSummary>,
  text_chars: Vec<TextCharSummary>,
  text_objects: Vec<TextObjectSummary>,
  images: Vec<ImageSummary>,
  paths: Vec<PathObjectSummary>,
  links: Vec<LinkSummary>,
  annotations: Vec<AnnotationSummary>,
  page_objects: Vec<PageObjectSummary>,
  rendered_pages: Vec<RenderedPageSummary>,
}

struct RawPdfSummary {
  pages: Vec<RawPageSummary>,
  outlines: Vec<String>,
}

fn pdfium_summary(pdf: &[u8]) -> Result<PdfiumSummary, String> {
  // PDFium extraction is not reliably parallel-safe in this harness.
  let _guard = pdfium_lock().lock().unwrap();
  let pdfium = bind_pdfium()?;
  let document = pdfium
    .load_pdf_from_byte_vec(pdf.to_vec(), None)
    .map_err(|error| format!("PDFium could not load PDF bytes: {error}"))?;

  let mut media_boxes = Vec::new();
  let mut text_segments = Vec::new();
  let mut text_chars = Vec::new();
  let mut text_objects = Vec::new();
  let mut images = Vec::new();
  let mut paths = Vec::new();
  let mut links = Vec::new();
  let mut annotations = Vec::new();
  let mut page_objects = Vec::new();
  let mut rendered_pages = Vec::new();

  for (page_index, page) in document.pages().iter().enumerate() {
    media_boxes.push(format!(
      "[0.0 0.0 {:.1} {:.1}]",
      page.width().value,
      page.height().value
    ));

    let text = page
      .text()
      .map_err(|error| format!("PDFium could not extract page {page_index} text: {error}"))?;
    for segment in text.segments().iter() {
      text_segments.push(TextSegmentSummary {
        page_index,
        text: normalize_extracted_text(&segment.text()),
        bounds: format_rect(segment.bounds(), 2),
      });
    }
    for char in text.chars().iter() {
      if let Some(value) = char.unicode_string() {
        let bounds = char
          .loose_bounds()
          .map_err(|error| format!("PDFium could not read char bounds: {error}"))?;
        text_chars.push(TextCharSummary {
          page_index,
          text: value,
          bounds: format_rect(bounds, 2),
        });
      }
    }

    let mut object_summary = PageObjectSummary {
      page_index,
      ..PageObjectSummary::default()
    };
    for object in page.objects().iter() {
      match object.object_type() {
        PdfPageObjectType::Text => {
          object_summary.text_objects += 1;
          if let Some(text) = object.as_text_object() {
            let font = text.font();
            text_objects.push(TextObjectSummary {
              page_index,
              text: normalize_extracted_text(&text.text()),
              font_name: normalize_pdf_font_name(&font.name()),
              font_family: normalize_pdf_font_name(&font.family()),
              scaled_font_size: format_points(text.scaled_font_size(), 2),
              unscaled_font_size: format_points(text.unscaled_font_size(), 2),
              render_mode: format_text_render_mode(text.render_mode()),
              fill_color: object.fill_color().ok().map(format_color),
              stroke_color: object.stroke_color().ok().map(format_color),
              bounds: object
                .bounds()
                .ok()
                .map(|bounds| format_rect(bounds.to_rect(), 2)),
            });
          }
        }
        PdfPageObjectType::Path => {
          object_summary.path_objects += 1;
          if let Some(path) = object.as_path_object() {
            let segments = path.segments();
            paths.push(PathObjectSummary {
              page_index,
              segments: segments.len(),
              fill_mode: path.fill_mode().ok().map(|value| format!("{value:?}")),
              stroked: path.is_stroked().ok(),
              fill_color: object.fill_color().ok().map(format_color),
              stroke_color: object.stroke_color().ok().map(format_color),
              bounds: object
                .bounds()
                .ok()
                .map(|bounds| format_rect(bounds.to_rect(), 2)),
              segment_details: segments
                .iter()
                .map(|segment| PathSegmentSummary {
                  segment_type: format!("{:?}", segment.segment_type()),
                  x: format_points(segment.x(), 2),
                  y: format_points(segment.y(), 2),
                  closed: segment.is_close(),
                })
                .collect(),
            });
          }
        }
        PdfPageObjectType::Image => {
          object_summary.image_objects += 1;
          if let Some(image) = object.as_image_object() {
            images.push(ImageSummary {
              page_index,
              width: image.width().ok().map(|value| value.to_string()),
              height: image.height().ok().map(|value| value.to_string()),
              bounds: object
                .bounds()
                .ok()
                .map(|bounds| format_rect(bounds.to_rect(), 2)),
            });
          }
        }
        PdfPageObjectType::Shading => object_summary.shading_objects += 1,
        PdfPageObjectType::XObjectForm => object_summary.form_objects += 1,
        PdfPageObjectType::Unsupported => object_summary.unsupported_objects += 1,
      }
    }
    page_objects.push(object_summary);

    for link in page.links().iter() {
      let (target_kind, target) = pdfium_link_target(&link);
      links.push(LinkSummary {
        page_index,
        target_kind,
        target,
        rect: link.rect().ok().map(|rect| format_rect(rect, 2)),
      });
    }

    for annotation in page.annotations().iter() {
      let action_uri = annotation
        .as_link_annotation()
        .and_then(|annotation| annotation.link().ok())
        .and_then(|link| link.action())
        .and_then(|action| action.as_uri_action().and_then(|action| action.uri().ok()))
        .map(|value| normalize_extracted_text(&value));
      annotations.push(AnnotationSummary {
        page_index,
        annotation_type: format!("{:?}", annotation.annotation_type()),
        bounds: annotation
          .bounds()
          .ok()
          .map(|bounds| format_rect_with_precision(bounds, 3)),
        action_uri,
      });
    }

    rendered_pages.push(rendered_page_summary(page_index, &page)?);
  }

  Ok(PdfiumSummary {
    page_count: document.pages().len() as usize,
    media_boxes,
    text_segments,
    text_chars,
    text_objects,
    images,
    paths,
    links,
    annotations,
    page_objects,
    rendered_pages,
  })
}

fn raw_pdf_summary(pdf: &[u8]) -> Result<RawPdfSummary, String> {
  let document = LopdfDocument::load_mem(pdf)
    .map_err(|error| format!("lopdf could not load PDF bytes: {error}"))?;

  let mut pages = Vec::new();
  for (page_number, page_id) in document.get_pages() {
    let page_index = page_number as usize - 1;
    pages.push(raw_page_summary(&document, page_index, page_id)?);
  }
  pages.sort_by_key(|page| page.page_index);

  let outlines = raw_outline_titles(&document)?;

  Ok(RawPdfSummary { pages, outlines })
}

fn raw_page_summary(
  document: &LopdfDocument,
  page_index: usize,
  page_id: lopdf::ObjectId,
) -> Result<RawPageSummary, String> {
  let page = document
    .get_dictionary(page_id)
    .map_err(|error| format!("lopdf could not read page dictionary {page_id:?}: {error}"))?;

  let mut annotation_refs = Vec::new();
  match page.get(b"Annots") {
    Ok(object) => {
      let annots = lopdf_array(document, object, "page Annots")?;
      annotation_refs.extend_from_slice(annots);
    }
    Err(lopdf::Error::DictKey(_)) => {}
    Err(error) => return Err(format!("lopdf could not read page Annots: {error}")),
  }

  let mut annotations = Vec::new();
  for annotation in &annotation_refs {
    let dictionary = lopdf_dictionary(document, annotation, "annotation dictionary")?;
    let parent = dictionary
      .get(b"Parent")
      .ok()
      .and_then(|parent| lopdf_dictionary(document, parent, "annotation parent dictionary").ok());
    let action_uri = dictionary
      .get(b"A")
      .ok()
      .and_then(|action| lopdf_dictionary(document, action, "annotation action").ok())
      .and_then(|action| action.get(b"URI").ok())
      .and_then(|uri| lopdf_text(document, uri).ok());
    let field_type_name = dictionary
      .get(b"FT")
      .ok()
      .and_then(|value| lopdf_name(document, value).ok())
      .or_else(|| {
        parent
          .and_then(|parent| parent.get(b"FT").ok())
          .and_then(|value| lopdf_name(document, value).ok())
      });
    let field_value = dictionary
      .get(b"V")
      .ok()
      .and_then(|value| lopdf_text(document, value).ok())
      .or_else(|| {
        parent
          .and_then(|parent| parent.get(b"V").ok())
          .and_then(|value| lopdf_text(document, value).ok())
      });

    annotations.push(RawAnnotationSummary {
      page_index,
      type_name: dictionary
        .get(b"Type")
        .ok()
        .and_then(|value| lopdf_name(document, value).ok()),
      subtype_name: dictionary
        .get(b"Subtype")
        .ok()
        .and_then(|value| lopdf_name(document, value).ok()),
      rect: dictionary
        .get(b"Rect")
        .ok()
        .and_then(|value| lopdf_rect(document, value).ok()),
      action_uri,
      field_type_name,
      field_value,
    });
  }

  let xobjects = raw_page_xobjects(document, page_index, page)?;

  Ok(RawPageSummary {
    page_index,
    annotation_count: annotation_refs.len(),
    annotations,
    xobjects,
  })
}

fn raw_outline_titles(document: &LopdfDocument) -> Result<Vec<String>, String> {
  let trailer_root = document
    .trailer
    .get(b"Root")
    .map_err(|error| format!("lopdf could not read trailer Root: {error}"))?;
  let catalog = lopdf_dictionary(document, trailer_root, "catalog dictionary")?;
  let Ok(outlines_object) = catalog.get(b"Outlines") else {
    return Ok(Vec::new());
  };
  let outlines = lopdf_dictionary(document, outlines_object, "Outlines dictionary")?;
  let Ok(first) = outlines.get(b"First") else {
    return Ok(Vec::new());
  };

  let mut titles = Vec::new();
  collect_outline_siblings(document, first, &mut titles)?;
  Ok(titles)
}

fn collect_outline_siblings(
  document: &LopdfDocument,
  first: &LopdfObject,
  titles: &mut Vec<String>,
) -> Result<(), String> {
  let mut current = Some(first.clone());
  while let Some(object) = current {
    let item = lopdf_dictionary(document, &object, "outline item dictionary")?;
    if let Ok(title) = item
      .get(b"Title")
      .map_err(|error| error.to_string())
      .and_then(|value| lopdf_text(document, value))
    {
      titles.push(title);
    }
    if let Ok(child) = item.get(b"First") {
      collect_outline_siblings(document, child, titles)?;
    }
    current = item.get(b"Next").ok().cloned();
  }
  Ok(())
}

fn raw_page_xobjects(
  document: &LopdfDocument,
  page_index: usize,
  page: &lopdf::Dictionary,
) -> Result<Vec<RawXObjectSummary>, String> {
  let resources = match inherited_page_dictionary_value(document, page, b"Resources") {
    Ok(Some(object)) => lopdf_dictionary_owned(document, object, "page Resources")?,
    Ok(None) => return Ok(Vec::new()),
    Err(lopdf::Error::DictKey(_)) => return Ok(Vec::new()),
    Err(error) => return Err(format!("lopdf could not read page Resources: {error}")),
  };
  let xobjects = match resources.get(b"XObject") {
    Ok(object) => lopdf_dictionary(document, object, "page Resources/XObject")?,
    Err(lopdf::Error::DictKey(_)) => return Ok(Vec::new()),
    Err(error) => {
      return Err(format!(
        "lopdf could not read page XObject resources: {error}"
      ));
    }
  };

  let mut summaries = Vec::new();
  collect_terminal_xobjects(document, page_index, None, xobjects, &mut summaries)?;
  summaries.sort_by(|left, right| left.name.cmp(&right.name));

  Ok(summaries)
}

fn collect_terminal_xobjects(
  document: &LopdfDocument,
  page_index: usize,
  prefix: Option<&str>,
  xobjects: &lopdf::Dictionary,
  summaries: &mut Vec<RawXObjectSummary>,
) -> Result<(), String> {
  for (name, object) in xobjects.iter() {
    let current_name = if let Some(prefix) = prefix {
      format!("{prefix}/{}", String::from_utf8_lossy(name))
    } else {
      String::from_utf8_lossy(name).to_string()
    };
    let stream = lopdf_stream(document, object, "page XObject stream")?;
    let subtype_name = stream
      .dict
      .get(b"Subtype")
      .ok()
      .and_then(|value| lopdf_name(document, value).ok());
    if subtype_name.as_deref() == Some("Form")
      && let Ok(resources_object) = stream.dict.get(b"Resources")
      && let Ok(resources) = lopdf_dictionary(document, resources_object, "form Resources")
      && let Ok(nested_object) = resources.get(b"XObject")
      && let Ok(nested_xobjects) = lopdf_dictionary(document, nested_object, "form XObject")
    {
      collect_terminal_xobjects(
        document,
        page_index,
        Some(&current_name),
        nested_xobjects,
        summaries,
      )?;
      continue;
    }

    summaries.push(raw_xobject_summary(
      document,
      page_index,
      &current_name,
      object,
    )?);
  }

  Ok(())
}

fn inherited_page_dictionary_value(
  document: &LopdfDocument,
  page: &lopdf::Dictionary,
  key: &[u8],
) -> Result<Option<LopdfObject>, lopdf::Error> {
  if let Ok(value) = page.get(key) {
    return Ok(Some(value.clone()));
  }

  let mut current_parent = match page.get(b"Parent") {
    Ok(parent) => Some(parent.clone()),
    Err(lopdf::Error::DictKey(_)) => None,
    Err(error) => return Err(error),
  };

  while let Some(parent) = current_parent {
    let parent_dict = lopdf_dictionary(document, &parent, "page parent dictionary")
      .map_err(|_| lopdf::Error::DictKey(String::from("Parent")))?;
    if let Ok(value) = parent_dict.get(key) {
      return Ok(Some(value.clone()));
    }
    current_parent = match parent_dict.get(b"Parent") {
      Ok(next) => Some(next.clone()),
      Err(lopdf::Error::DictKey(_)) => None,
      Err(error) => return Err(error),
    };
  }

  Ok(None)
}

fn raw_xobject_summary(
  document: &LopdfDocument,
  page_index: usize,
  name: &str,
  object: &LopdfObject,
) -> Result<RawXObjectSummary, String> {
  let stream = lopdf_stream(document, object, "page XObject stream")?;
  let type_name = stream
    .dict
    .get(b"Type")
    .ok()
    .and_then(|value| lopdf_name(document, value).ok());
  let subtype_name = stream
    .dict
    .get(b"Subtype")
    .ok()
    .and_then(|value| lopdf_name(document, value).ok());
  let filter_names = stream
    .filters()
    .map(|filters| {
      filters
        .into_iter()
        .map(|filter| String::from_utf8_lossy(filter).to_string())
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();
  let width_px = stream
    .dict
    .get(b"Width")
    .ok()
    .and_then(|value| lopdf_u32(document, value).ok());
  let height_px = stream
    .dict
    .get(b"Height")
    .ok()
    .and_then(|value| lopdf_u32(document, value).ok());
  let image_format = image::guess_format(&stream.content)
    .ok()
    .map(|format| format!("{format:?}"));
  let decoded = image::load_from_memory(&stream.content).ok();
  let (decoded_width_px, decoded_height_px, bits_per_pixel) = if let Some(image) = decoded {
    let (width, height) = image.dimensions();
    (
      Some(width),
      Some(height),
      Some(image.color().bits_per_pixel()),
    )
  } else {
    (None, None, None)
  };

  Ok(RawXObjectSummary {
    page_index,
    name: name.to_string(),
    type_name,
    subtype_name,
    filter_names,
    width_px,
    height_px,
    image_format,
    decoded_width_px,
    decoded_height_px,
    bits_per_pixel,
  })
}

fn lopdf_array<'a>(
  document: &'a LopdfDocument,
  object: &'a LopdfObject,
  context: &str,
) -> Result<&'a Vec<LopdfObject>, String> {
  let (_, object) = document
    .dereference(object)
    .map_err(|error| format!("lopdf could not dereference {context}: {error}"))?;
  object
    .as_array()
    .map_err(|error| format!("lopdf expected array for {context}: {error}"))
}

fn lopdf_dictionary<'a>(
  document: &'a LopdfDocument,
  object: &'a LopdfObject,
  context: &str,
) -> Result<&'a lopdf::Dictionary, String> {
  let (_, object) = document
    .dereference(object)
    .map_err(|error| format!("lopdf could not dereference {context}: {error}"))?;
  object
    .as_dict()
    .map_err(|error| format!("lopdf expected dictionary for {context}: {error}"))
}

fn lopdf_dictionary_owned(
  document: &LopdfDocument,
  object: LopdfObject,
  context: &str,
) -> Result<lopdf::Dictionary, String> {
  let (_, object) = document
    .dereference(&object)
    .map_err(|error| format!("lopdf could not dereference {context}: {error}"))?;
  object
    .as_dict()
    .cloned()
    .map_err(|error| format!("lopdf expected dictionary for {context}: {error}"))
}

fn lopdf_name(document: &LopdfDocument, object: &LopdfObject) -> Result<String, String> {
  let (_, object) = document
    .dereference(object)
    .map_err(|error| format!("lopdf could not dereference name: {error}"))?;
  let name = object
    .as_name()
    .map_err(|error| format!("lopdf expected name object: {error}"))?;
  Ok(String::from_utf8_lossy(name).to_string())
}

fn lopdf_text(document: &LopdfDocument, object: &LopdfObject) -> Result<String, String> {
  let (_, object) = document
    .dereference(object)
    .map_err(|error| format!("lopdf could not dereference string: {error}"))?;
  match object {
    LopdfObject::String(value, _) => Ok(normalize_extracted_text(&String::from_utf8_lossy(value))),
    LopdfObject::Name(value) => Ok(String::from_utf8_lossy(value).to_string()),
    _ => Err(format!(
      "lopdf expected string-like object, found {}",
      object.enum_variant()
    )),
  }
}

fn lopdf_rect(document: &LopdfDocument, object: &LopdfObject) -> Result<String, String> {
  let values = lopdf_array(document, object, "annotation Rect")?;
  if values.len() != 4 {
    return Err(format!(
      "lopdf expected four coordinates in annotation Rect, found {}",
      values.len()
    ));
  }

  let mut numbers = [0.0f32; 4];
  for (index, value) in values.iter().enumerate() {
    let (_, value) = document
      .dereference(value)
      .map_err(|error| format!("lopdf could not dereference Rect coordinate: {error}"))?;
    numbers[index] = value
      .as_float()
      .map_err(|error| format!("lopdf expected numeric Rect coordinate: {error}"))?;
  }

  Ok(format!(
    "[{:.3} {:.3} {:.3} {:.3}]",
    numbers[0], numbers[1], numbers[2], numbers[3]
  ))
}

fn lopdf_stream<'a>(
  document: &'a LopdfDocument,
  object: &'a LopdfObject,
  context: &str,
) -> Result<&'a lopdf::Stream, String> {
  let (_, object) = document
    .dereference(object)
    .map_err(|error| format!("lopdf could not dereference {context}: {error}"))?;
  match object {
    LopdfObject::Stream(stream) => Ok(stream),
    _ => Err(format!(
      "lopdf expected stream for {context}, found {}",
      object.enum_variant()
    )),
  }
}

fn lopdf_u32(document: &LopdfDocument, object: &LopdfObject) -> Result<u32, String> {
  let (_, object) = document
    .dereference(object)
    .map_err(|error| format!("lopdf could not dereference integer: {error}"))?;
  let value = object
    .as_i64()
    .map_err(|error| format!("lopdf expected integer object: {error}"))?;
  u32::try_from(value).map_err(|_| format!("lopdf integer is out of range for u32: {value}"))
}

fn bind_pdfium() -> Result<Pdfium, String> {
  if let Some(path) = std::env::var_os("PDFIUM_DYNAMIC_LIB_PATH") {
    match Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path(&path)) {
      Ok(bindings) => return Ok(Pdfium::new(bindings)),
      Err(PdfiumError::PdfiumLibraryBindingsAlreadyInitialized) => return Ok(Pdfium::default()),
      Err(error) => {
        return Err(format!(
          "could not bind PDFium from PDFIUM_DYNAMIC_LIB_PATH={}: {error}",
          std::path::PathBuf::from(path).display()
        ));
      }
    }
  }

  match Pdfium::bind_to_system_library() {
    Ok(bindings) => Ok(Pdfium::new(bindings)),
    Err(PdfiumError::PdfiumLibraryBindingsAlreadyInitialized) => Ok(Pdfium::default()),
    Err(error) => Err(format!(
      "could not bind system PDFium library; install libpdfium.so or set PDFIUM_DYNAMIC_LIB_PATH: {error}"
    )),
  }
}

fn pdfium_lock() -> &'static Mutex<()> {
  static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
  LOCK.get_or_init(|| Mutex::new(()))
}

fn pdfium_link_target(link: &PdfLink<'_>) -> (LinkTargetKind, Option<String>) {
  if let Some(action) = link.action() {
    if let Some(uri) = action.as_uri_action() {
      return (
        LinkTargetKind::ExternalUri,
        uri.uri().ok().map(|value| normalize_extracted_text(&value)),
      );
    }
    if let Some(local) = action.as_local_destination_action() {
      return (
        LinkTargetKind::InternalDestination,
        local
          .destination()
          .ok()
          .and_then(|dest| format_destination(&dest)),
      );
    }
    return (
      LinkTargetKind::Action,
      Some(format!("{:?}", action.action_type())),
    );
  }

  if let Some(destination) = link.destination() {
    return (
      LinkTargetKind::InternalDestination,
      format_destination(&destination),
    );
  }

  (LinkTargetKind::Unknown, None)
}

fn format_destination(destination: &PdfDestination<'_>) -> Option<String> {
  let page = destination.page_index().ok()?;
  let view = destination
    .view_settings()
    .map(|value| format!("{value:?}"))
    .unwrap_or_else(|_| "Unknown".to_string());
  Some(format!("page={page} view={view}"))
}

fn rendered_page_summary(
  page_index: usize,
  page: &PdfPage<'_>,
) -> Result<RenderedPageSummary, String> {
  let image = rendered_page_image(page_index, page, 1200)?;
  Ok(RenderedPageSummary {
    page_index,
    width_px: image.width_px,
    height_px: image.height_px,
    rgba_crc32: image.rgba_crc32,
  })
}

fn rendered_page_image(
  page_index: usize,
  page: &PdfPage<'_>,
  target_width: i32,
) -> Result<RenderedPageImage, String> {
  let bitmap = page
    .render_with_config(&PdfRenderConfig::new().set_target_width(target_width))
    .map_err(|error| format!("PDFium could not render page {page_index}: {error}"))?;
  let width_px = bitmap.width() as u32;
  let height_px = bitmap.height() as u32;
  let image = bitmap
    .as_image()
    .map_err(|error| format!("PDFium could not convert rendered page {page_index}: {error}"))?;
  let rgba = image.to_rgba8();
  let mut crc = crc32fast::Hasher::new();
  crc.update(rgba.as_raw());
  Ok(RenderedPageImage {
    page_index,
    width_px,
    height_px,
    page_width_pt: page.width().value,
    page_height_pt: page.height().value,
    rgba_crc32: format!("{:08x}", crc.finalize()),
    rgba: rgba.into_raw(),
  })
}

fn format_color(color: PdfColor) -> String {
  format!(
    "#{:02x}{:02x}{:02x}@{:02x}",
    color.red(),
    color.green(),
    color.blue(),
    color.alpha()
  )
}

fn format_points(points: PdfPoints, precision: usize) -> String {
  format!("{value:.precision$}", value = points.value)
}

fn format_text_render_mode(mode: PdfPageTextRenderMode) -> String {
  format!("{mode:?}")
}

fn format_rect(rect: pdfium_render::prelude::PdfRect, precision: usize) -> String {
  format_rect_with_precision(rect, precision)
}

fn format_rect_with_precision(rect: pdfium_render::prelude::PdfRect, precision: usize) -> String {
  format!(
    "[{left:.precision$} {bottom:.precision$} {right:.precision$} {top:.precision$}]",
    left = rect.left().value,
    bottom = rect.bottom().value,
    right = rect.right().value,
    top = rect.top().value,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_pdf_rect_reads_four_point_coordinates() {
    let rect = parse_pdf_rect("[1.5 2.0 30.25 40.75]").unwrap();
    assert_eq!(
      rect,
      PdfBounds {
        left: 1.5,
        bottom: 2.0,
        right: 30.25,
        top: 40.75,
      }
    );
    assert_eq!(rect.width(), 28.75);
    assert_eq!(rect.height(), 38.75);
    assert_eq!(rect.center(), (15.875, 21.375));
  }

  #[test]
  #[should_panic(expected = "PDF rect right mismatch")]
  fn assert_pdf_rect_close_reports_coordinate_mismatch() {
    assert_pdf_rect_close(
      "[0.0 0.0 12.0 10.0]",
      PdfBounds {
        left: 0.0,
        bottom: 0.0,
        right: 10.0,
        top: 10.0,
      },
      0.1,
    );
  }

  #[test]
  fn rendered_page_image_maps_pdf_coordinates_to_pixels() {
    let image = RenderedPageImage {
      page_index: 0,
      width_px: 200,
      height_px: 100,
      page_width_pt: 400.0,
      page_height_pt: 200.0,
      rgba_crc32: String::new(),
      rgba: vec![0; 200 * 100 * 4],
    };

    assert_eq!(image.pdf_point_to_pixel(0.0, 200.0), Some((0, 0)));
    assert_eq!(image.pdf_point_to_pixel(400.0, 0.0), Some((199, 99)));
    assert_eq!(image.pdf_point_to_pixel(200.0, 100.0), Some((100, 50)));
    assert_eq!(
      image.pdf_rect_to_pixel_rect(PdfBounds {
        left: 100.0,
        bottom: 50.0,
        right: 300.0,
        top: 150.0,
      }),
      Some(PixelRect {
        left: 50,
        top: 25,
        width: 100,
        height: 50,
      })
    );
  }

  #[test]
  fn rendered_page_image_samples_pixels_and_hashes_regions() {
    let mut rgba = vec![0; 4 * 4 * 4];
    rgba[(2 * 4 + 1) * 4..(2 * 4 + 2) * 4].copy_from_slice(&[10, 20, 30, 255]);
    let image = RenderedPageImage {
      page_index: 0,
      width_px: 4,
      height_px: 4,
      page_width_pt: 4.0,
      page_height_pt: 4.0,
      rgba_crc32: String::new(),
      rgba,
    };

    assert_eq!(image.pixel_rgba(1, 2), Some([10, 20, 30, 255]));
    assert_eq!(
      image.sample_pdf_point_rgba(1.0, 2.0),
      Some([10, 20, 30, 255])
    );
    assert!(
      image
        .pixel_region_crc32(PixelRect {
          left: 1,
          top: 2,
          width: 1,
          height: 1,
        })
        .is_some()
    );
  }
}

struct PdfStream {
  dictionary: String,
  decoded: Option<Vec<u8>>,
}

fn pdf_streams(pdf: &[u8]) -> Vec<PdfStream> {
  let mut streams = Vec::new();
  let mut search_start = 0usize;
  while let Some(stream_offset) = find_bytes(&pdf[search_start..], b"stream") {
    let stream_keyword = search_start + stream_offset;
    let dict_start =
      rfind_bytes(&pdf[..stream_keyword], b"<<").unwrap_or(stream_keyword.saturating_sub(512));
    let dictionary = String::from_utf8_lossy(&pdf[dict_start..stream_keyword]).to_string();
    let mut data_start = stream_keyword + "stream".len();
    if pdf.get(data_start) == Some(&b'\r') {
      data_start += 1;
    }
    if pdf.get(data_start) == Some(&b'\n') {
      data_start += 1;
    }
    let Some(end_offset) = find_bytes(&pdf[data_start..], b"endstream") else {
      break;
    };
    let mut data_end = data_start + end_offset;
    while data_end > data_start && matches!(pdf[data_end - 1], b'\r' | b'\n') {
      data_end -= 1;
    }
    let data = &pdf[data_start..data_end];
    let decoded = if dictionary.contains("/FlateDecode") {
      let mut decoder = ZlibDecoder::new(data);
      let mut output = Vec::new();
      decoder.read_to_end(&mut output).ok().map(|_| output)
    } else if dictionary.contains("/Filter") {
      None
    } else {
      Some(data.to_vec())
    };
    streams.push(PdfStream {
      dictionary,
      decoded,
    });
    search_start = data_end + "endstream".len();
  }
  streams
}

fn find_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
  haystack
    .windows(needle.len())
    .position(|window| window == needle)
}

fn rfind_bytes(haystack: &[u8], needle: &[u8]) -> Option<usize> {
  haystack
    .windows(needle.len())
    .rposition(|window| window == needle)
}

fn content_summary(streams: &[PdfStream]) -> ContentSummary {
  let mut summary = ContentSummary {
    stream_count: streams.len(),
    ..ContentSummary::default()
  };
  for stream in streams {
    if !stream.dictionary.contains("/Length") {
      continue;
    }
    let Some(decoded) = &stream.decoded else {
      continue;
    };
    summary.decoded_stream_count += 1;
    let content = String::from_utf8_lossy(decoded);
    summary.text_show_ops += operator_count(&content, &["Tj", "TJ", "'", "\""]);
    summary.image_draw_ops += operator_count(&content, &["Do"]);
    summary.path_paint_ops += operator_count(&content, &["S", "s", "f", "F", "f*", "B", "B*"]);
    summary.clipping_ops += operator_count(&content, &["W", "W*"]);
  }
  summary
}

fn operator_count(content: &str, operators: &[&str]) -> usize {
  content
    .split_ascii_whitespace()
    .filter(|token| operators.contains(token))
    .count()
}

fn pdftotext(pdf: &[u8]) -> std::result::Result<String, String> {
  let path = temp_pdf_path();
  std::fs::write(&path, pdf).map_err(|error| error.to_string())?;
  let output = Command::new("pdftotext")
    .args(["-layout", "-nopgbrk"])
    .arg(&path)
    .arg("-")
    .output();
  let _ = std::fs::remove_file(&path);
  let output = output.map_err(|error| format!("pdftotext failed to start: {error}"))?;
  if !output.status.success() {
    return Err(format!(
      "pdftotext failed: status={} stderr={}",
      output.status,
      String::from_utf8_lossy(&output.stderr)
    ));
  }
  Ok(normalize_extracted_text(&String::from_utf8_lossy(
    &output.stdout,
  )))
}

fn temp_pdf_path() -> std::path::PathBuf {
  let nanos = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_nanos();
  std::env::temp_dir().join(format!(
    "ooxmlsdk-pdf-extract-{}-{nanos}.pdf",
    std::process::id()
  ))
}

fn normalize_extracted_text(text: &str) -> String {
  text
    .lines()
    .map(|line| line.trim_end())
    .collect::<Vec<_>>()
    .join("\n")
    .trim()
    .to_string()
}

fn normalize_pdf_font_name(name: &str) -> String {
  let Some((prefix, rest)) = name.split_once('+') else {
    return name.to_string();
  };
  if prefix.len() == 6 && prefix.bytes().all(|byte| byte.is_ascii_uppercase()) {
    rest.to_string()
  } else {
    name.to_string()
  }
}

trait PdfBytesExt {
  fn strip_suffix_ascii_whitespace(&self) -> &[u8];
}

impl PdfBytesExt for [u8] {
  fn strip_suffix_ascii_whitespace(&self) -> &[u8] {
    let mut end = self.len();
    while end > 0 && self[end - 1].is_ascii_whitespace() {
      end -= 1;
    }
    &self[..end]
  }
}
