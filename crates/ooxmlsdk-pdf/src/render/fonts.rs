use std::sync::Arc;

use krilla::text::Font;
use rustc_hash::FxHashMap as HashMap;

use crate::error::{PdfError, Result};
use ooxmlsdk_layout::common;
use ooxmlsdk_layout::fonts::{FontFaceData, FontResolver, FontStyleRef};

pub(super) struct FontSet {
  resolver: FontResolver,
  fallback: Font,
  fonts: HashMap<FontKey, Font>,
  face_fonts: HashMap<FontFaceData, Font>,
  last_font: Option<(FontKey, Font)>,
}

impl FontSet {
  pub(super) fn load(mut resolver: FontResolver) -> Result<Self> {
    let fallback_style = common::TextStyle::default();
    let fallback = load_font(&mut resolver, &fallback_style)?;
    Ok(Self {
      resolver,
      fallback,
      fonts: HashMap::default(),
      face_fonts: HashMap::default(),
      last_font: None,
    })
  }

  pub(super) fn select(&mut self, style: &(impl FontStyleRef + ?Sized)) -> Font {
    if let Some((key, font)) = self.last_font.as_ref()
      && key.matches_style(style)
    {
      return font.clone();
    }

    let key = FontKey::from_style(style);
    if let Some(font) = self.fonts.get(&key) {
      let font = font.clone();
      self.store_last_font(key, font.clone());
      return font;
    }

    let loaded = load_font(&mut self.resolver, style).unwrap_or_else(|_| self.fallback.clone());
    let font = self.fonts.entry(key.clone()).or_insert(loaded).clone();
    self.store_last_font(key, font.clone());
    font
  }

  pub(super) fn select_face(&mut self, face: &FontFaceData) -> Font {
    if let Some(font) = self.face_fonts.get(face) {
      return font.clone();
    }

    let loaded = font_from_face(face).unwrap_or_else(|| self.fallback.clone());
    self
      .face_fonts
      .entry(face.clone())
      .or_insert(loaded)
      .clone()
  }

  fn store_last_font(&mut self, key: FontKey, font: Font) {
    self.last_font = Some((key, font));
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontKey {
  family: Option<String>,
  bold: bool,
  italic: bool,
}

impl FontKey {
  fn from_style(style: &(impl FontStyleRef + ?Sized)) -> Self {
    Self {
      family: style.font_family().map(str::to_string),
      bold: style.bold(),
      italic: style.italic(),
    }
  }

  fn matches_style(&self, style: &(impl FontStyleRef + ?Sized)) -> bool {
    self.family.as_deref() == style.font_family()
      && self.bold == style.bold()
      && self.italic == style.italic()
  }
}

fn load_font(resolver: &mut FontResolver, style: &(impl FontStyleRef + ?Sized)) -> Result<Font> {
  if let Some(face) = resolver.cached_text_face(style)
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
