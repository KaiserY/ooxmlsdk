use std::sync::Arc;

use krilla::color::rgb;
use krilla::configure::{Configuration, PdfVersion};
use krilla::geom::{PathBuilder, Point};
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
        PageItem::Line(line) => {
          surface.set_fill(None);
          surface.set_stroke(Some(Stroke {
            width: line.width_pt,
            paint: rgb::Color::new(0, 0, 0).into(),
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
