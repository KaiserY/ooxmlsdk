use std::hash::{Hash, Hasher};
use std::io::Cursor;
use std::sync::Arc;

use image::{GenericImageView, ImageFormat as RasterImageFormat, Rgba};
use krilla::action::{Action, LinkAction};
use krilla::annotation::{Annotation, LinkAnnotation, Target};
use krilla::color::rgb;
use krilla::configure::{Configuration, PdfVersion};
use krilla::geom::{PathBuilder, Point, Rect, Size, Transform};
use krilla::image::{BitsPerComponent, CustomImage, Image, ImageColorspace};
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, Stroke};
use krilla::surface::Surface;
use krilla::text::{Font, GlyphId, KrillaGlyph, TextDirection};
use krilla::{Document, SerializeSettings};

use crate::docx::TextStyle;
use crate::error::{PdfError, Result};
use crate::fonts::load_sans_face;
use crate::layout::{ImageItem, LayoutDocument, PageItem};
use crate::options::PdfOptions;
use crate::text_metrics::{measure_text, shape_text};

pub(crate) fn render(document: &LayoutDocument, options: &PdfOptions) -> Result<Vec<u8>> {
  let mut pdf = Document::new_with(serialize_settings(options));
  let fonts = FontSet::load()?;

  for page in &document.pages {
    let settings =
      PageSettings::from_wh(page.setup.width_pt.max(3.0), page.setup.height_pt.max(3.0))
        .ok_or_else(|| PdfError::Krilla("invalid page size".to_string()))?;

    let mut pdf_page = pdf.start_page_with(settings);
    let mut surface = pdf_page.surface();
    let mut link_annotations = Vec::new();
    for item in &page.items {
      match item {
        PageItem::Text(text) if !text.text.is_empty() => {
          let baseline_y = text.y_pt - text.style.baseline_shift_pt;
          let text_width = measure_text(&text.text, text.style);
          if let Some(color) = text.style.highlight {
            surface.set_stroke(None);
            surface.set_fill(Some(Fill {
              paint: rgb::Color::new(color.r, color.g, color.b).into(),
              opacity: NormalizedF32::ONE,
              rule: Default::default(),
            }));
            let top = baseline_y - text.style.font_size_pt;
            let mut path = PathBuilder::new();
            path.move_to(text.x_pt, top);
            path.line_to(text.x_pt + text_width, top);
            path.line_to(
              text.x_pt + text_width,
              baseline_y + text.style.font_size_pt * 0.25,
            );
            path.line_to(text.x_pt, baseline_y + text.style.font_size_pt * 0.25);
            path.close();
            if let Some(path) = path.finish() {
              surface.draw_path(&path);
            }
          }
          surface.set_stroke(None);
          surface.set_fill(Some(fill(text.style)));
          if let Some(glyphs) = shaped_pdf_glyphs(&text.text, text.style) {
            surface.draw_glyphs(
              Point::from_xy(text.x_pt, baseline_y),
              &glyphs,
              fonts.select(text.style).clone(),
              &text.text,
              text.style.font_size_pt,
              false,
            );
          } else {
            surface.draw_text(
              Point::from_xy(text.x_pt, baseline_y),
              fonts.select(text.style).clone(),
              text.style.font_size_pt,
              &text.text,
              false,
              TextDirection::Auto,
            );
          }
          if text.style.underline {
            surface.set_fill(None);
            surface.set_stroke(Some(Stroke {
              width: (text.style.font_size_pt / 18.0).max(0.5),
              paint: rgb::Color::new(text.style.color.r, text.style.color.g, text.style.color.b)
                .into(),
              ..Default::default()
            }));
            let underline_y = baseline_y + (text.style.font_size_pt * 0.12).max(1.0);
            let mut path = PathBuilder::new();
            path.move_to(text.x_pt, underline_y);
            path.line_to(text.x_pt + text_width, underline_y);
            if let Some(path) = path.finish() {
              surface.draw_path(&path);
            }
          }
          if text.style.strikethrough {
            surface.set_fill(None);
            surface.set_stroke(Some(Stroke {
              width: (text.style.font_size_pt / 18.0).max(0.5),
              paint: rgb::Color::new(text.style.color.r, text.style.color.g, text.style.color.b)
                .into(),
              ..Default::default()
            }));
            let strike_y = baseline_y - (text.style.font_size_pt * 0.32);
            let mut path = PathBuilder::new();
            path.move_to(text.x_pt, strike_y);
            path.line_to(text.x_pt + text_width, strike_y);
            if let Some(path) = path.finish() {
              surface.draw_path(&path);
            }
          }
          if let Some(url) = &text.hyperlink_url {
            let top = baseline_y - text.style.font_size_pt;
            let bottom = baseline_y + text.style.font_size_pt * 0.25;
            if let Some(rect) = Rect::from_ltrb(text.x_pt, top, text.x_pt + text_width, bottom) {
              let target = Target::Action(Action::Link(LinkAction::new(url.clone())));
              let link = LinkAnnotation::new(rect, target);
              link_annotations.push(Annotation::new_link(link, None));
            }
          }
        }
        PageItem::Text(_) => {}
        PageItem::Fill(fill_item) => {
          surface.set_stroke(None);
          surface.set_fill(Some(Fill {
            paint: rgb::Color::new(fill_item.color.r, fill_item.color.g, fill_item.color.b).into(),
            opacity: NormalizedF32::ONE,
            rule: Default::default(),
          }));
          let mut path = PathBuilder::new();
          path.move_to(fill_item.x_pt, fill_item.y_pt);
          path.line_to(fill_item.x_pt + fill_item.width_pt, fill_item.y_pt);
          path.line_to(
            fill_item.x_pt + fill_item.width_pt,
            fill_item.y_pt + fill_item.height_pt,
          );
          path.line_to(fill_item.x_pt, fill_item.y_pt + fill_item.height_pt);
          path.close();
          if let Some(path) = path.finish() {
            surface.draw_path(&path);
          }
        }
        PageItem::Image(image) => {
          let _alt_text = image.alt_text.as_deref();
          match decode_image(&image.data, image.content_type.as_deref()) {
            Ok(pdf_image) => draw_image_item(&mut surface, image, pdf_image),
            Err(_) => {
              surface.set_fill(None);
              surface.set_stroke(Some(Stroke {
                width: 0.5,
                paint: rgb::Color::new(128, 128, 128).into(),
                ..Default::default()
              }));
              let mut path = PathBuilder::new();
              path.move_to(image.x_pt, image.y_pt);
              path.line_to(image.x_pt + image.width_pt, image.y_pt);
              path.line_to(image.x_pt + image.width_pt, image.y_pt + image.height_pt);
              path.line_to(image.x_pt, image.y_pt + image.height_pt);
              path.close();
              if let Some(path) = path.finish() {
                surface.draw_path(&path);
              }
            }
          }
        }
        PageItem::Line(line) => {
          surface.set_fill(None);
          surface.set_stroke(Some(Stroke {
            width: line.width_pt,
            paint: rgb::Color::new(line.color.r, line.color.g, line.color.b).into(),
            ..Default::default()
          }));
          let mut path = PathBuilder::new();
          path.move_to(line.x1_pt, line.y1_pt);
          path.line_to(line.x2_pt, line.y2_pt);
          if let Some(path) = path.finish() {
            surface.draw_path(&path);
          }
        }
      }
    }
    surface.finish();
    for annotation in link_annotations {
      pdf_page.add_annotation(annotation);
    }
  }

  pdf
    .finish()
    .map_err(|err| PdfError::Krilla(format!("{err:?}")))
}

fn draw_image_item(surface: &mut Surface<'_>, image: &ImageItem, pdf_image: Image) {
  let width = image.width_pt.max(1.0);
  let height = image.height_pt.max(1.0);
  surface.push_transform(&Transform::from_translate(image.x_pt, image.y_pt));
  let mut pop_count = 1;

  if image.rotation_deg.abs() > f32::EPSILON {
    surface.push_transform(&Transform::from_rotate_at(
      image.rotation_deg,
      width / 2.0,
      height / 2.0,
    ));
    pop_count += 1;
  }

  if image_has_crop_or_transform(image)
    && let Some(clip) = rect_path(0.0, 0.0, width, height)
  {
    surface.push_clip_path(&clip, &krilla::paint::FillRule::NonZero);
    pop_count += 1;
  }

  if image.flip_horizontal {
    surface.push_transform(&Transform::from_translate(width, 0.0));
    surface.push_transform(&Transform::from_scale(-1.0, 1.0));
    pop_count += 2;
  }
  if image.flip_vertical {
    surface.push_transform(&Transform::from_translate(0.0, height));
    surface.push_transform(&Transform::from_scale(1.0, -1.0));
    pop_count += 2;
  }

  let visible_width = (1.0 - image.crop.left - image.crop.right).max(0.001);
  let visible_height = (1.0 - image.crop.top - image.crop.bottom).max(0.001);
  let draw_width = width / visible_width;
  let draw_height = height / visible_height;
  if let Some(size) = Size::from_wh(draw_width, draw_height) {
    surface.push_transform(&Transform::from_translate(
      -image.crop.left * draw_width,
      -image.crop.top * draw_height,
    ));
    surface.draw_image(pdf_image, size);
    surface.pop();
  }

  for _ in 0..pop_count {
    surface.pop();
  }
}

fn shaped_pdf_glyphs(text: &str, style: TextStyle) -> Option<Vec<KrillaGlyph>> {
  let shaped = shape_text(text, style)?;
  Some(
    shaped
      .glyphs
      .into_iter()
      .map(|glyph| {
        KrillaGlyph::new(
          GlyphId::new(glyph.glyph_id),
          glyph.x_advance_em,
          glyph.x_offset_em,
          glyph.y_offset_em,
          glyph.y_advance_em,
          glyph.text_range,
          None,
        )
      })
      .collect(),
  )
}

fn image_has_crop_or_transform(image: &ImageItem) -> bool {
  image.crop.left > 0.0
    || image.crop.top > 0.0
    || image.crop.right > 0.0
    || image.crop.bottom > 0.0
    || image.rotation_deg.abs() > f32::EPSILON
    || image.flip_horizontal
    || image.flip_vertical
}

fn rect_path(x: f32, y: f32, width: f32, height: f32) -> Option<krilla::geom::Path> {
  let mut path = PathBuilder::new();
  path.move_to(x, y);
  path.line_to(x + width, y);
  path.line_to(x + width, y + height);
  path.line_to(x, y + height);
  path.close();
  path.finish()
}

fn decode_image(data: &[u8], content_type: Option<&str>) -> Result<Image> {
  let format = content_type
    .and_then(image_format_from_content_type)
    .or_else(|| image::guess_format(data).ok());

  if matches!(format, Some(RasterImageFormat::Jpeg))
    && let Ok(image) = Image::from_jpeg(data.to_vec().into(), true)
  {
    return Ok(image);
  }
  if matches!(format, Some(RasterImageFormat::Png))
    && let Ok(image) = decode_png_relaxed(data)
  {
    return Image::from_custom(image, true).map_err(PdfError::Krilla);
  }

  let raster = match format {
    Some(format) => image::load_from_memory_with_format(data, format),
    None => image::load_from_memory(data),
  };

  let raster =
    raster.map_err(|err| PdfError::Krilla(format!("failed to decode raster image: {err}")))?;
  Image::from_custom(PdfRasterImage::from_dynamic(raster), true).map_err(PdfError::Krilla)
}

fn decode_png_relaxed(data: &[u8]) -> std::result::Result<PdfRasterImage, String> {
  let mut decoder = png::Decoder::new(Cursor::new(data));
  decoder.ignore_checksums(true);
  decoder.set_transformations(png::Transformations::normalize_to_color8());
  let mut reader = decoder.read_info().map_err(|err| err.to_string())?;
  let buffer_size = reader
    .output_buffer_size()
    .ok_or_else(|| "PNG output buffer size is unavailable".to_string())?;
  let mut buffer = vec![0; buffer_size];
  let info = reader
    .next_frame(&mut buffer)
    .map_err(|err| err.to_string())?;
  buffer.truncate(info.buffer_size());
  Ok(PdfRasterImage::from_png_frame(
    info.width,
    info.height,
    info.color_type,
    &buffer,
  ))
}

fn image_format_from_content_type(content_type: &str) -> Option<RasterImageFormat> {
  match content_type {
    "image/png" => Some(RasterImageFormat::Png),
    "image/jpeg" | "image/jpg" => Some(RasterImageFormat::Jpeg),
    "image/gif" => Some(RasterImageFormat::Gif),
    "image/webp" => Some(RasterImageFormat::WebP),
    _ => None,
  }
}

#[derive(Clone, Debug)]
struct PdfRasterImage {
  pixels: Arc<PdfRasterPixels>,
}

#[derive(Debug)]
struct PdfRasterPixels {
  width: u32,
  height: u32,
  rgb: Vec<u8>,
  alpha: Option<Vec<u8>>,
}

impl PdfRasterImage {
  fn from_dynamic(image: image::DynamicImage) -> Self {
    let (width, height) = image.dimensions();
    let rgba = image.to_rgba8();
    let mut rgb = Vec::with_capacity(width as usize * height as usize * 3);
    let mut alpha = Vec::with_capacity(width as usize * height as usize);
    let mut opaque = true;

    for Rgba([r, g, b, a]) in rgba.pixels() {
      rgb.extend_from_slice(&[*r, *g, *b]);
      alpha.push(*a);
      opaque &= *a == u8::MAX;
    }

    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha: (!opaque).then_some(alpha),
      }),
    }
  }

  fn from_png_frame(width: u32, height: u32, color_type: png::ColorType, data: &[u8]) -> Self {
    let pixel_count = width as usize * height as usize;
    let mut rgb = Vec::with_capacity(pixel_count * 3);
    let mut alpha = Vec::with_capacity(pixel_count);
    let mut opaque = true;

    match color_type {
      png::ColorType::Grayscale => {
        for value in data {
          rgb.extend_from_slice(&[*value, *value, *value]);
        }
      }
      png::ColorType::GrayscaleAlpha => {
        for pixel in data.chunks_exact(2) {
          rgb.extend_from_slice(&[pixel[0], pixel[0], pixel[0]]);
          alpha.push(pixel[1]);
          opaque &= pixel[1] == u8::MAX;
        }
      }
      png::ColorType::Rgb => {
        rgb.extend_from_slice(data);
      }
      png::ColorType::Rgba => {
        for pixel in data.chunks_exact(4) {
          rgb.extend_from_slice(&pixel[..3]);
          alpha.push(pixel[3]);
          opaque &= pixel[3] == u8::MAX;
        }
      }
      png::ColorType::Indexed => {}
    }

    Self {
      pixels: Arc::new(PdfRasterPixels {
        width,
        height,
        rgb,
        alpha: (!opaque && !alpha.is_empty()).then_some(alpha),
      }),
    }
  }
}

impl Hash for PdfRasterImage {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.pixels.width.hash(state);
    self.pixels.height.hash(state);
    self.pixels.rgb.hash(state);
    self.pixels.alpha.hash(state);
  }
}

impl CustomImage for PdfRasterImage {
  fn color_channel(&self) -> &[u8] {
    &self.pixels.rgb
  }

  fn alpha_channel(&self) -> Option<&[u8]> {
    self.pixels.alpha.as_deref()
  }

  fn bits_per_component(&self) -> BitsPerComponent {
    BitsPerComponent::Eight
  }

  fn size(&self) -> (u32, u32) {
    (self.pixels.width, self.pixels.height)
  }

  fn icc_profile(&self) -> Option<&[u8]> {
    None
  }

  fn color_space(&self) -> ImageColorspace {
    ImageColorspace::Rgb
  }
}

fn serialize_settings(options: &PdfOptions) -> SerializeSettings {
  SerializeSettings {
    compress_content_streams: options.compress_content_streams,
    no_device_cs: true,
    ascii_compatible: false,
    xmp_metadata: true,
    cmyk_profile: None,
    configuration: Configuration::new_with_version(PdfVersion::Pdf17),
    enable_tagging: false,
    render_svg_glyph_fn: krilla_svg::render_svg_glyph,
  }
}

#[derive(Clone)]
struct FontSet {
  regular: Font,
  bold: Font,
  italic: Font,
  bold_italic: Font,
}

impl FontSet {
  fn load() -> Result<Self> {
    let regular = load_font(false, false)?;
    let bold = load_font(true, false).unwrap_or_else(|_| regular.clone());
    let italic = load_font(false, true).unwrap_or_else(|_| regular.clone());
    let bold_italic = load_font(true, true).unwrap_or_else(|_| bold.clone());

    Ok(Self {
      regular,
      bold,
      italic,
      bold_italic,
    })
  }

  fn select(&self, style: TextStyle) -> &Font {
    match (style.bold, style.italic) {
      (true, true) => &self.bold_italic,
      (true, false) => &self.bold,
      (false, true) => &self.italic,
      (false, false) => &self.regular,
    }
  }
}

fn load_font(bold: bool, italic: bool) -> Result<Font> {
  if let Some(face) = load_sans_face(bold, italic)
    && let Some(font) = Font::new(Arc::new(face.data).into(), face.index)
  {
    return Ok(font);
  }

  Err(PdfError::FontUnavailable)
}

fn fill(style: TextStyle) -> Fill {
  Fill {
    paint: rgb::Color::new(style.color.r, style.color.g, style.color.b).into(),
    opacity: NormalizedF32::ONE,
    rule: Default::default(),
  }
}
