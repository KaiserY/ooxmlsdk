use std::sync::Arc;

use krilla::text::Font;
use rustc_hash::FxHashMap as HashMap;

use crate::error::{PdfError, Result};
use ooxmlsdk_layout::common;
use ooxmlsdk_layout::fonts::{FontFaceCacheKey, FontFaceData, FontResolver, FontStyleRef};

pub(super) struct FontSet {
  resolver: FontResolver,
  fallback: Font,
  fonts: HashMap<FontKey, SelectedFont>,
  face_fonts: HashMap<FontFaceCacheKey, Font>,
  last_font: Option<(FontKey, SelectedFont)>,
  last_face_font: Option<(FontFaceCacheKey, Font)>,
}

#[derive(Clone)]
pub(super) struct SelectedFont {
  pub(super) font: Font,
  pub(super) synthetic_bold: bool,
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
      last_face_font: None,
    })
  }

  pub(super) fn select(&mut self, style: &(impl FontStyleRef + ?Sized)) -> SelectedFont {
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
    let selected = load_selected_font(&mut self.resolver, style).unwrap_or_else(|_| SelectedFont {
      font: self.fallback.clone(),
      synthetic_bold: false,
    });
    let selected = self.fonts.entry(key.clone()).or_insert(selected).clone();
    self.store_last_font(key, selected.clone());
    selected
  }

  pub(super) fn select_face(&mut self, face: &FontFaceData) -> SelectedFont {
    if let Some((key, font)) = self.last_face_font.as_ref()
      && key.matches_face(face)
    {
      return SelectedFont {
        font: font.clone(),
        synthetic_bold: face.synthetic_bold,
      };
    }
    let key = face.cache_key();
    if let Some(font) = self.face_fonts.get(&key) {
      let font = font.clone();
      self.last_face_font = Some((key, font.clone()));
      return SelectedFont {
        font,
        synthetic_bold: face.synthetic_bold,
      };
    }

    let loaded = font_from_face(face).unwrap_or_else(|| self.fallback.clone());
    let font = self.face_fonts.entry(key.clone()).or_insert(loaded).clone();
    self.last_face_font = Some((key, font.clone()));
    SelectedFont {
      font,
      synthetic_bold: face.synthetic_bold,
    }
  }

  fn store_last_font(&mut self, key: FontKey, font: SelectedFont) {
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
  if let Some(font) = resolver
    .with_cached_text_face(style, font_from_face)
    .flatten()
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

fn load_selected_font(
  resolver: &mut FontResolver,
  style: &(impl FontStyleRef + ?Sized),
) -> Result<SelectedFont> {
  if let Some(selected) = resolver.with_cached_text_face(style, |face| {
    font_from_face(face).map(|font| SelectedFont {
      font,
      synthetic_bold: face.synthetic_bold,
    })
  }) {
    return selected
      .ok_or_else(|| PdfError::Krilla("resolved PDF font binary could not be loaded".to_string()));
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
