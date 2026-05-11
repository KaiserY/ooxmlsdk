use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use crate::docx::TextStyle;

#[derive(Clone, Debug)]
pub(crate) struct FontFaceData {
  pub data: Vec<u8>,
  pub index: u32,
}

pub(crate) fn load_text_face(style: &TextStyle) -> Option<FontFaceData> {
  load_face(style.font_family.as_deref(), style.bold, style.italic)
}

fn load_face(family: Option<&str>, bold: bool, italic: bool) -> Option<FontFaceData> {
  let key = FontFaceKey {
    family: family
      .filter(|family| !family.trim().is_empty())
      .map(ToOwned::to_owned),
    bold,
    italic,
  };
  let mut cache = font_face_cache().lock().ok()?;
  if let Some(face) = cache.get(&key) {
    return face.clone();
  }

  let mut families = Vec::new();
  if let Some(family) = family.filter(|family| !family.trim().is_empty()) {
    families.push(fontdb::Family::Name(family));
    push_font_aliases(&mut families, family);
  } else {
    families.push(fontdb::Family::SansSerif);
  }

  if let Some(id) = font_db().query(&fontdb::Query {
    families: &families,
    weight: query_weight(family, bold),
    style: if italic {
      fontdb::Style::Italic
    } else {
      fontdb::Style::Normal
    },
    ..fontdb::Query::default()
  }) && let Some((data, index)) =
    font_db().with_face_data(id, |data, index| (data.to_vec(), index))
  {
    let face = Some(FontFaceData { data, index });
    cache.insert(key, face.clone());
    return face;
  }

  let fallback_paths = family
    .filter(|family| !family.trim().is_empty())
    .map(|family| specific_fallback_font_paths(family, bold, italic))
    .unwrap_or_else(|| generic_fallback_font_paths(bold, italic));

  for path in fallback_paths {
    let Ok(data) = std::fs::read(path) else {
      continue;
    };
    if ttf_parser::Face::parse(&data, 0).is_ok() {
      let face = Some(FontFaceData { data, index: 0 });
      cache.insert(key, face.clone());
      return face;
    }
  }

  cache.insert(key, None);
  None
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct FontFaceKey {
  family: Option<String>,
  bold: bool,
  italic: bool,
}

fn font_db() -> &'static fontdb::Database {
  static DB: OnceLock<fontdb::Database> = OnceLock::new();
  DB.get_or_init(|| {
    let mut db = fontdb::Database::new();
    db.load_system_fonts();
    db
  })
}

fn font_face_cache() -> &'static Mutex<HashMap<FontFaceKey, Option<FontFaceData>>> {
  static CACHE: OnceLock<Mutex<HashMap<FontFaceKey, Option<FontFaceData>>>> = OnceLock::new();
  CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn query_weight(family: Option<&str>, bold: bool) -> fontdb::Weight {
  if family.is_some_and(|family| family.eq_ignore_ascii_case("Arial Black")) {
    return fontdb::Weight::BLACK;
  }
  if bold {
    fontdb::Weight::BOLD
  } else {
    fontdb::Weight::NORMAL
  }
}

fn push_font_aliases<'a>(families: &mut Vec<fontdb::Family<'a>>, family: &'a str) {
  match family.trim().to_ascii_lowercase().as_str() {
    "calibri" => families.extend([
      fontdb::Family::Name("Carlito"),
      fontdb::Family::Name("Liberation Sans"),
    ]),
    "cambria" => families.extend([
      fontdb::Family::Name("Caladea"),
      fontdb::Family::Name("Liberation Serif"),
    ]),
    "times new roman" => families.extend([
      fontdb::Family::Name("Liberation Serif"),
      fontdb::Family::Name("Nimbus Roman"),
    ]),
    "arial" => families.extend([
      fontdb::Family::Name("Liberation Sans"),
      fontdb::Family::Name("Arimo"),
    ]),
    "arial black" => families.extend([
      fontdb::Family::Name("Arial"),
      fontdb::Family::Name("Liberation Sans"),
    ]),
    _ => {}
  }
}

fn specific_fallback_font_paths(family: &str, bold: bool, italic: bool) -> &'static [&'static str] {
  if family.eq_ignore_ascii_case("Calibri") {
    return match (bold, italic) {
      (true, true) => &[
        "/usr/share/fonts/truetype/msttcorefonts/calibriz.ttf",
        "/usr/share/fonts/truetype/crosextra/Carlito-BoldItalic.ttf",
      ],
      (true, false) => &[
        "/usr/share/fonts/truetype/msttcorefonts/calibrib.ttf",
        "/usr/share/fonts/truetype/crosextra/Carlito-Bold.ttf",
      ],
      (false, true) => &[
        "/usr/share/fonts/truetype/msttcorefonts/calibrii.ttf",
        "/usr/share/fonts/truetype/crosextra/Carlito-Italic.ttf",
      ],
      (false, false) => &[
        "/usr/share/fonts/truetype/msttcorefonts/calibri.ttf",
        "/usr/share/fonts/truetype/crosextra/Carlito-Regular.ttf",
      ],
    };
  }

  if family.eq_ignore_ascii_case("Times New Roman") {
    return match (bold, italic) {
      (true, true) => &[
        "/usr/share/fonts/truetype/msttcorefonts/timesbi.ttf",
        "/usr/share/fonts/truetype/liberation2/LiberationSerif-BoldItalic.ttf",
      ],
      (true, false) => &[
        "/usr/share/fonts/truetype/msttcorefonts/timesbd.ttf",
        "/usr/share/fonts/truetype/liberation2/LiberationSerif-Bold.ttf",
      ],
      (false, true) => &[
        "/usr/share/fonts/truetype/msttcorefonts/timesi.ttf",
        "/usr/share/fonts/truetype/liberation2/LiberationSerif-Italic.ttf",
      ],
      (false, false) => &[
        "/usr/share/fonts/truetype/msttcorefonts/times.ttf",
        "/usr/share/fonts/truetype/liberation2/LiberationSerif-Regular.ttf",
      ],
    };
  }

  if family.eq_ignore_ascii_case("Arial Black") {
    return match italic {
      true => &[
        "/usr/share/fonts/truetype/msttcorefonts/ariblk.ttf",
        "/usr/share/fonts/truetype/liberation2/LiberationSans-BoldItalic.ttf",
        "/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf",
      ],
      false => &[
        "/usr/share/fonts/truetype/msttcorefonts/ariblk.ttf",
        "/usr/share/fonts/truetype/liberation2/LiberationSans-Bold.ttf",
      ],
    };
  }

  &[]
}

fn generic_fallback_font_paths(bold: bool, italic: bool) -> &'static [&'static str] {
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

#[cfg(test)]
mod tests {
  use std::sync::Arc;

  use crate::docx::TextStyle;

  use super::load_text_face;

  #[test]
  fn missing_named_font_does_not_silently_fall_back_to_generic_sans() {
    let style = TextStyle {
      font_family: Some(Arc::from("CodexDefinitelyMissingFont")),
      ..Default::default()
    };

    assert!(load_text_face(&style).is_none());
  }
}
