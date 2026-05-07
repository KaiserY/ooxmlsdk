use std::sync::Arc;

use krilla::color::rgb;
use krilla::configure::{Configuration, PdfVersion};
use krilla::geom::{PathBuilder, Point, Size, Transform};
use krilla::image::Image;
use krilla::num::NormalizedF32;
use krilla::page::PageSettings;
use krilla::paint::{Fill, Stroke};
use krilla::text::{Font, TextDirection};
use krilla::{Document, SerializeSettings};

use crate::docx::TextStyle;
use crate::error::{PdfError, Result};
use crate::layout::{LayoutDocument, PageItem};
use crate::options::PdfOptions;

pub(crate) fn render(document: &LayoutDocument, options: &PdfOptions) -> Result<Vec<u8>> {
  let mut pdf = Document::new_with(serialize_settings(options));
  let fonts = FontSet::load()?;

  for page in &document.pages {
    let settings =
      PageSettings::from_wh(page.setup.width_pt.max(3.0), page.setup.height_pt.max(3.0))
        .ok_or_else(|| PdfError::Krilla("invalid page size".to_string()))?;

    let mut pdf_page = pdf.start_page_with(settings);
    let mut surface = pdf_page.surface();
    for item in &page.items {
      match item {
        PageItem::Text(text) if !text.text.is_empty() => {
          surface.set_stroke(None);
          surface.set_fill(Some(fill(text.style)));
          surface.draw_text(
            Point::from_xy(text.x_pt, text.y_pt),
            fonts.select(text.style).clone(),
            text.style.font_size_pt,
            &text.text,
            false,
            TextDirection::Auto,
          );
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
          if let Some(size) = Size::from_wh(image.width_pt.max(1.0), image.height_pt.max(1.0)) {
            match decode_image(&image.data, image.content_type.as_deref()) {
              Ok(pdf_image) => {
                surface.push_transform(&Transform::from_translate(image.x_pt, image.y_pt));
                surface.draw_image(pdf_image, size);
                surface.pop();
              }
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
  }

  pdf
    .finish()
    .map_err(|err| PdfError::Krilla(format!("{err:?}")))
}

fn decode_image(data: &[u8], content_type: Option<&str>) -> Result<Image> {
  let bytes = data.to_vec().into();
  let result = match content_type {
    Some("image/png") if valid_png_crc(data) => Image::from_png(bytes, true),
    Some("image/png") => Err("invalid PNG CRC".to_string()),
    Some("image/jpeg") | Some("image/jpg") => Image::from_jpeg(bytes, true),
    Some("image/gif") => Image::from_gif(bytes, true),
    Some("image/webp") => Image::from_webp(bytes, true),
    _ if data.starts_with(b"\x89PNG\r\n\x1a\n") && valid_png_crc(data) => {
      Image::from_png(bytes, true)
    }
    _ if data.starts_with(b"\x89PNG\r\n\x1a\n") => Err("invalid PNG CRC".to_string()),
    _ if data.starts_with(b"\xff\xd8\xff") => Image::from_jpeg(bytes, true),
    _ if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") => Image::from_gif(bytes, true),
    _ if data.len() >= 12 && &data[0..4] == b"RIFF" && &data[8..12] == b"WEBP" => {
      Image::from_webp(bytes, true)
    }
    _ => Err(format!(
      "unsupported image content type {}",
      content_type.unwrap_or("unknown")
    )),
  };

  result.map_err(PdfError::Krilla)
}

fn valid_png_crc(data: &[u8]) -> bool {
  if !data.starts_with(b"\x89PNG\r\n\x1a\n") {
    return false;
  }

  let mut offset = 8;
  while offset + 12 <= data.len() {
    let length = u32::from_be_bytes([
      data[offset],
      data[offset + 1],
      data[offset + 2],
      data[offset + 3],
    ]) as usize;
    let chunk_type_start = offset + 4;
    let chunk_data_start = offset + 8;
    let chunk_data_end = chunk_data_start.saturating_add(length);
    let crc_end = chunk_data_end.saturating_add(4);
    if crc_end > data.len() {
      return false;
    }

    let expected = u32::from_be_bytes([
      data[chunk_data_end],
      data[chunk_data_end + 1],
      data[chunk_data_end + 2],
      data[chunk_data_end + 3],
    ]);
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(&data[chunk_type_start..chunk_data_end]);
    if hasher.finalize() != expected {
      return false;
    }
    if &data[chunk_type_start..chunk_type_start + 4] == b"IEND" {
      return true;
    }
    offset = crc_end;
  }

  false
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
  let mut db = fontdb::Database::new();
  db.load_system_fonts();
  if let Some(id) = db.query(&fontdb::Query {
    families: &[fontdb::Family::SansSerif],
    weight: if bold {
      fontdb::Weight::BOLD
    } else {
      fontdb::Weight::NORMAL
    },
    style: if italic {
      fontdb::Style::Italic
    } else {
      fontdb::Style::Normal
    },
    ..fontdb::Query::default()
  }) && let Some((data, index)) = db.with_face_data(id, |data, index| (data.to_vec(), index))
    && let Some(font) = Font::new(Arc::new(data).into(), index)
  {
    return Ok(font);
  }

  for path in fallback_font_paths(bold, italic) {
    let Ok(data) = std::fs::read(path) else {
      continue;
    };
    if let Some(font) = Font::new(Arc::new(data).into(), 0) {
      return Ok(font);
    }
  }

  Err(PdfError::FontUnavailable)
}

fn fallback_font_paths(bold: bool, italic: bool) -> &'static [&'static str] {
  match (bold, italic) {
    (true, true) => &[
      "/usr/share/fonts/truetype/dejavu/DejaVuSans-BoldOblique.ttf",
      "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ],
    (true, false) => &[
      "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ],
    (false, true) => &[
      "/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf",
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ],
    (false, false) => &[
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
      "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
      "/usr/share/fonts/truetype/liberation2/LiberationSans-Regular.ttf",
      "/usr/share/fonts/truetype/ubuntu/Ubuntu[wdth,wght].ttf",
    ],
  }
}

fn fill(style: TextStyle) -> Fill {
  Fill {
    paint: rgb::Color::new(style.color.r, style.color.g, style.color.b).into(),
    opacity: NormalizedF32::ONE,
    rule: Default::default(),
  }
}
