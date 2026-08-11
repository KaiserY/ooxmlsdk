use std::{fmt::Write as _, sync::Arc};

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
  explicit_notdef_semantics: HashMap<String, Option<String>>,
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
      explicit_notdef_semantics: HashMap::default(),
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

  pub(super) fn record_explicit_notdef_semantic(&mut self, face: &FontFaceData, text: &str) {
    if text.is_empty()
      || text
        .chars()
        .any(|character| matches!(character, '\0' | '\u{feff}' | '\u{fffe}'))
    {
      return;
    }
    let Some(postscript_name) = font_postscript_name(face) else {
      return;
    };
    merge_explicit_notdef_semantic(&mut self.explicit_notdef_semantics, postscript_name, text);
  }

  pub(super) fn restore_office_font_metadata(&self, pdf: Vec<u8>) -> Result<Vec<u8>> {
    if self.descriptor_metrics.is_empty() && self.explicit_notdef_semantics.is_empty() {
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
    let notdef_updates = document
      .objects
      .values()
      .filter_map(|object| {
        let dictionary = object.as_dict().ok()?;
        if !dictionary
          .get(b"Type")
          .and_then(LopdfObject::as_name)
          .is_ok_and(|name| name == b"Font")
          || !dictionary
            .get(b"Subtype")
            .and_then(LopdfObject::as_name)
            .is_ok_and(|name| name == b"Type0")
        {
          return None;
        }
        let base_name = dictionary
          .get(b"BaseFont")
          .and_then(LopdfObject::as_name)
          .ok()
          .and_then(pdf_type0_base_name)?;
        let semantic = self.explicit_notdef_semantics.get(base_name)?.as_deref()?;
        let cmap_id = dictionary
          .get(b"ToUnicode")
          .and_then(LopdfObject::as_reference)
          .ok()?;
        Some((cmap_id, semantic.to_string()))
      })
      .collect::<Vec<_>>();
    for (cmap_id, semantic) in notdef_updates {
      let Some(LopdfObject::Stream(stream)) = document.objects.get_mut(&cmap_id) else {
        continue;
      };
      if insert_notdef_tounicode_mapping(stream, &semantic)? {
        changed = true;
      }
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
  let postscript_name = font_postscript_name(face_data)?;
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

fn font_postscript_name(face_data: &FontFaceData) -> Option<String> {
  let face = SkrifaFontRef::from_index(face_data.data.as_slice(), face_data.index).ok()?;
  face
    .localized_strings(SkrifaStringId::POSTSCRIPT_NAME)
    .english_or_first()
    .map(|name| name.to_string())
}

fn merge_explicit_notdef_semantic(
  mappings: &mut HashMap<String, Option<String>>,
  postscript_name: String,
  text: &str,
) {
  let entry = mappings
    .entry(postscript_name)
    .or_insert_with(|| Some(text.to_string()));
  if entry.as_deref().is_some_and(|existing| existing != text) {
    *entry = None;
  }
}

fn pdf_type0_base_name(name: &[u8]) -> Option<&str> {
  let name = pdf_font_descriptor_base_name(name)?;
  Some(name.strip_suffix("-Identity-H").unwrap_or(name))
}

fn insert_notdef_tounicode_mapping(stream: &mut lopdf::Stream, text: &str) -> Result<bool> {
  let content = stream
    .decompressed_content()
    .map_err(|error| PdfError::Lopdf(error.to_string()))?;
  let content_text = std::str::from_utf8(&content)
    .map_err(|error| PdfError::Lopdf(format!("ToUnicode CMap is not ASCII-compatible: {error}")))?;
  if cmap_has_notdef_mapping(content_text) {
    return Ok(false);
  }
  let Some(insert_at) = content_text.rfind("endcmap") else {
    return Ok(false);
  };
  let mut utf16_hex = String::with_capacity(text.len() * 4);
  for unit in text.encode_utf16() {
    write!(&mut utf16_hex, "{unit:04X}").expect("writing to a String cannot fail");
  }
  if utf16_hex.is_empty() {
    return Ok(false);
  }

  // Krilla reserves CID 0 for .notdef and deliberately omits it from the
  // generated ToUnicode CMap. Word's fixed writer, and LibreOffice's subset
  // mapping loop, retain a mapping when that displayed glyph has one
  // unambiguous source string. Preserve that semantic text without changing
  // the subset glyph, its advance, or any painted outline.
  let mapping = format!("1 beginbfchar\n<0000> <{utf16_hex}>\nendbfchar\n");
  let mut patched = Vec::with_capacity(content.len() + mapping.len());
  patched.extend_from_slice(&content[..insert_at]);
  patched.extend_from_slice(mapping.as_bytes());
  patched.extend_from_slice(&content[insert_at..]);
  stream.set_plain_content(patched);
  stream
    .compress()
    .map_err(|error| PdfError::Lopdf(error.to_string()))?;
  Ok(true)
}

fn cmap_has_notdef_mapping(content: &str) -> bool {
  let mut in_mapping = false;
  for token in content.split_ascii_whitespace() {
    if token.eq_ignore_ascii_case("beginbfchar") || token.eq_ignore_ascii_case("beginbfrange") {
      in_mapping = true;
    } else if token.eq_ignore_ascii_case("endbfchar") || token.eq_ignore_ascii_case("endbfrange") {
      in_mapping = false;
    } else if in_mapping && token.eq_ignore_ascii_case("<0000>") {
      return true;
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::*;
  use lopdf::{Stream, dictionary};

  const EMPTY_CMAP: &str =
    "begincmap\n1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\nendcmap\n";

  #[test]
  fn explicit_notdef_semantic_requires_one_value_per_font() {
    let mut mappings = HashMap::default();
    merge_explicit_notdef_semantic(&mut mappings, "SymbolFace".to_string(), "\u{f081}");
    merge_explicit_notdef_semantic(&mut mappings, "SymbolFace".to_string(), "\u{f081}");
    assert_eq!(
      mappings.get("SymbolFace").and_then(Option::as_deref),
      Some("\u{f081}")
    );

    merge_explicit_notdef_semantic(&mut mappings, "SymbolFace".to_string(), "\u{f082}");
    assert_eq!(mappings.get("SymbolFace"), Some(&None));
    merge_explicit_notdef_semantic(&mut mappings, "SymbolFace".to_string(), "\u{f081}");
    assert_eq!(mappings.get("SymbolFace"), Some(&None));
  }

  #[test]
  fn inserts_cid_zero_tounicode_once_without_overwriting_existing_mapping() {
    let mut stream = Stream::new(dictionary! {}, EMPTY_CMAP.as_bytes().to_vec());
    assert!(insert_notdef_tounicode_mapping(&mut stream, "\u{f081}").unwrap());
    let content = String::from_utf8(stream.decompressed_content().unwrap()).unwrap();
    assert!(content.contains("<0000> <F081>"));
    assert_eq!(content.matches("<0000> <F081>").count(), 1);
    assert!(!insert_notdef_tounicode_mapping(&mut stream, "\u{f081}").unwrap());

    let existing = EMPTY_CMAP.replace(
      "endcmap",
      "1 beginbfchar\n<0000> <0041>\nendbfchar\nendcmap",
    );
    let mut existing_stream = Stream::new(dictionary! {}, existing.into_bytes());
    assert!(!insert_notdef_tounicode_mapping(&mut existing_stream, "\u{f081}").unwrap());
    let content = String::from_utf8(existing_stream.decompressed_content().unwrap()).unwrap();
    assert!(content.contains("<0000> <0041>"));
    assert!(!content.contains("<0000> <F081>"));
  }

  #[test]
  fn type0_base_name_matches_truetype_and_cff_subset_names() {
    assert_eq!(pdf_type0_base_name(b"ABCDEF+SymbolMT"), Some("SymbolMT"));
    assert_eq!(
      pdf_type0_base_name(b"ABCDEF+ExampleCFF-Identity-H"),
      Some("ExampleCFF")
    );
  }
}
