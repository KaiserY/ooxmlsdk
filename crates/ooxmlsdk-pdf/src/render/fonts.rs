use std::sync::Arc;

use krilla::text::Font;
use lopdf::{Document as LopdfDocument, Object as LopdfObject};
use rustc_hash::FxHashMap as HashMap;
use skrifa::{
  FontRef as SkrifaFontRef, MetadataProvider, raw::TableProvider as SkrifaTableProvider,
  string::StringId as SkrifaStringId,
};

use crate::error::{PdfError, Result};
use ooxmlsdk_layout::fonts::{FontFaceCacheKey, FontFaceData};

pub(super) struct FontSet {
  face_fonts: HashMap<FontFaceCacheKey, Font>,
  descriptor_metrics: HashMap<String, OfficeFontDescriptorMetrics>,
  last_face_font: Option<(FontFaceCacheKey, Font)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct OfficeFontDescriptorMetrics {
  bbox: [i64; 4],
  ascent: i64,
  descent: i64,
  cap_height: i64,
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
      descriptor_metrics: HashMap::default(),
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
    if let Some((name, metrics)) = office_font_descriptor_metrics(face) {
      self.descriptor_metrics.entry(name).or_insert(metrics);
    }
    let font = self.face_fonts.entry(key.clone()).or_insert(loaded).clone();
    self.last_face_font = Some((key, font.clone()));
    Ok(SelectedFont {
      font,
      synthetic_bold: face.synthetic_bold,
    })
  }

  pub(super) fn restore_office_font_descriptor_metrics(&self, pdf: Vec<u8>) -> Result<Vec<u8>> {
    if self.descriptor_metrics.is_empty() {
      return Ok(pdf);
    }
    let mut document =
      LopdfDocument::load_mem(&pdf).map_err(|error| PdfError::Lopdf(error.to_string()))?;
    let mut changed = false;
    for object in document.objects.values_mut() {
      let Ok(dictionary) = object.as_dict_mut() else {
        continue;
      };
      if !dictionary
        .get(b"Type")
        .and_then(LopdfObject::as_name)
        .is_ok_and(|name| name == b"FontDescriptor")
      {
        continue;
      }
      let Some(font_name) = dictionary
        .get(b"FontName")
        .and_then(LopdfObject::as_name)
        .ok()
        .and_then(pdf_font_descriptor_base_name)
      else {
        continue;
      };
      let Some(metrics) = self.descriptor_metrics.get(font_name) else {
        continue;
      };

      // Adobe's PDF font descriptor contract expresses these values in a
      // 1000-unit em. Word's fixed-format writer keeps the original face's
      // head horizontal bounds and OS/2 typographic vertical bounds even
      // when the embedded program is subsetted; its separate Ascent and
      // Descent entries come from hhea. Krilla instead narrows FontBBox to
      // the outlines present in the subset, which makes PDFium report a
      // different selection box for rotated text. Restore the source-face
      // metrics after serialization; this changes PDF text semantics only,
      // never the painted glyph program or its placement.
      dictionary.set(
        "FontBBox",
        LopdfObject::Array(metrics.bbox.into_iter().map(LopdfObject::Integer).collect()),
      );
      dictionary.set("Ascent", LopdfObject::Integer(metrics.ascent));
      dictionary.set("Descent", LopdfObject::Integer(metrics.descent));
      dictionary.set("CapHeight", LopdfObject::Integer(metrics.cap_height));
      changed = true;
    }
    if !changed {
      return Ok(pdf);
    }

    let mut output = Vec::new();
    document
      .save_to(&mut output)
      .map_err(|error| PdfError::Lopdf(error.to_string()))?;
    Ok(output)
  }
}

fn font_from_face(face: &FontFaceData) -> Option<Font> {
  let data: Arc<dyn AsRef<[u8]> + Send + Sync> = face.data.clone();
  Font::new(data.into(), face.index)
}

fn office_font_descriptor_metrics(
  face_data: &FontFaceData,
) -> Option<(String, OfficeFontDescriptorMetrics)> {
  let face = SkrifaFontRef::from_index(face_data.data.as_slice(), face_data.index).ok()?;
  let head = face.head().ok()?;
  let hhea = face.hhea().ok()?;
  let os2 = face.os2().ok()?;
  let units_per_em = f32::from(head.units_per_em());
  if units_per_em <= 0.0 {
    return None;
  }
  let postscript_name = face
    .localized_strings(SkrifaStringId::POSTSCRIPT_NAME)
    .english_or_first()?
    .to_string();
  // Krilla reserves room for the six-letter subset tag and the Type0
  // Identity-H suffix even though FontDescriptor/FontName itself does not
  // carry that suffix. Mirror its public-name truncation so the serialized
  // descriptor maps back to the exact source face.
  const KRILLA_DESCRIPTOR_POSTSCRIPT_NAME_LIMIT: usize = 109;
  let postscript_name = postscript_name
    .get(
      ..postscript_name
        .len()
        .min(KRILLA_DESCRIPTOR_POSTSCRIPT_NAME_LIMIT),
    )?
    .to_string();
  let to_pdf_units = |value: i16| (f32::from(value) / units_per_em * 1000.0).round() as i64;
  let ascent = to_pdf_units(hhea.ascender().to_i16());
  let descent = to_pdf_units(hhea.descender().to_i16());
  let typo_ascent = to_pdf_units(os2.s_typo_ascender());
  let typo_descent = to_pdf_units(os2.s_typo_descender());
  (ascent > descent).then_some((
    postscript_name,
    OfficeFontDescriptorMetrics {
      bbox: [
        to_pdf_units(head.x_min()),
        typo_descent,
        to_pdf_units(head.x_max()),
        typo_ascent,
      ],
      ascent,
      descent,
      // Word's fixed-output descriptors use OS/2.sTypoAscender for
      // CapHeight rather than OS/2.sCapHeight. This is independently
      // observable in Calibri and Cambria reference PDFs.
      cap_height: typo_ascent,
    },
  ))
}

fn pdf_font_descriptor_base_name(name: &[u8]) -> Option<&str> {
  let name = if name.len() > 7 && name.get(6) == Some(&b'+') {
    &name[7..]
  } else {
    name
  };
  std::str::from_utf8(name).ok()
}
