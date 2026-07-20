use std::sync::Arc;

use krilla::text::Font;
use rustc_hash::FxHashMap as HashMap;

use crate::error::{PdfError, Result};
use ooxmlsdk_layout::fonts::{FontFaceCacheKey, FontFaceData};

pub(super) struct FontSet {
  face_fonts: HashMap<FontFaceCacheKey, Font>,
  last_face_font: Option<(FontFaceCacheKey, Font)>,
}

#[derive(Clone)]
pub(super) struct SelectedFont {
  pub(super) font: Font,
  pub(super) synthetic_bold: bool,
}

impl FontSet {
  pub(super) fn new() -> Self {
    Self {
      face_fonts: HashMap::default(),
      last_face_font: None,
    }
  }

  pub(super) fn select_face(&mut self, face: &FontFaceData) -> Result<SelectedFont> {
    if let Some((key, font)) = self.last_face_font.as_ref()
      && key.matches_face(face)
    {
      return Ok(SelectedFont {
        font: font.clone(),
        synthetic_bold: face.synthetic_bold,
      });
    }
    let key = face.cache_key();
    if let Some(font) = self.face_fonts.get(&key) {
      let font = font.clone();
      self.last_face_font = Some((key, font.clone()));
      return Ok(SelectedFont {
        font,
        synthetic_bold: face.synthetic_bold,
      });
    }

    let loaded = font_from_face(face).ok_or_else(|| {
      PdfError::Krilla(format!(
        "resolved PDF font binary could not be loaded: font_id={} face_index={}",
        face.id(),
        face.index
      ))
    })?;
    let font = self.face_fonts.entry(key.clone()).or_insert(loaded).clone();
    self.last_face_font = Some((key, font.clone()));
    Ok(SelectedFont {
      font,
      synthetic_bold: face.synthetic_bold,
    })
  }
}

fn font_from_face(face: &FontFaceData) -> Option<Font> {
  let data: Arc<dyn AsRef<[u8]> + Send + Sync> = face.data.clone();
  Font::new(data.into(), face.index)
}
