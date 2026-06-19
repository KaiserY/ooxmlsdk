use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use krilla::text::Font;

use crate::error::{PdfError, Result};
use ooxmlsdk_layout::common;
use ooxmlsdk_layout::fonts::{FontFaceData, FontStyleRef, cached_text_face};

pub(super) struct FontSet {
  fallback: Font,
  fonts: Mutex<HashMap<FontKey, Font>>,
  face_fonts: Mutex<HashMap<FontFaceData, Font>>,
}

impl FontSet {
  pub(super) fn load() -> Result<Self> {
    let fallback_style = common::TextStyle::default();
    Ok(Self {
      fallback: load_font(&fallback_style)?,
      fonts: Mutex::new(HashMap::new()),
      face_fonts: Mutex::new(HashMap::new()),
    })
  }

  pub(super) fn select(&self, style: &(impl FontStyleRef + ?Sized)) -> Font {
    let key = FontKey {
      family: style.font_family().map(str::to_string),
      bold: style.bold(),
      italic: style.italic(),
    };
    if let Ok(fonts) = self.fonts.lock()
      && let Some(font) = fonts.get(&key)
    {
      return font.clone();
    }

    let loaded = load_font(style).unwrap_or_else(|_| self.fallback.clone());
    let Ok(mut fonts) = self.fonts.lock() else {
      return loaded;
    };
    fonts.entry(key).or_insert(loaded).clone()
  }

  pub(super) fn select_face(&self, face: &FontFaceData) -> Font {
    if let Ok(fonts) = self.face_fonts.lock()
      && let Some(font) = fonts.get(face)
    {
      return font.clone();
    }

    let loaded = font_from_face(face).unwrap_or_else(|| self.fallback.clone());
    let Ok(mut fonts) = self.face_fonts.lock() else {
      return loaded;
    };
    fonts.entry(face.clone()).or_insert(loaded).clone()
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontKey {
  family: Option<String>,
  bold: bool,
  italic: bool,
}

fn load_font(style: &(impl FontStyleRef + ?Sized)) -> Result<Font> {
  if let Some(face) = cached_text_face(style)
    && let Some(font) = font_from_face(&face)
  {
    return Ok(font);
  }

  Err(PdfError::Krilla(format!(
    "required PDF font was not found: family={} bold={} italic={}",
    style
      .font_family()
      .filter(|family| !family.trim().is_empty())
      .unwrap_or("<document-default>"),
    style.bold(),
    style.italic()
  )))
}

fn font_from_face(face: &FontFaceData) -> Option<Font> {
  let data: Arc<dyn AsRef<[u8]> + Send + Sync> = face.data.clone();
  Font::new(data.into(), face.index)
}
