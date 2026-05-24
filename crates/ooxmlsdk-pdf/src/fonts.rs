use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock};

use crate::docx::TextStyle;

#[derive(Clone, Debug)]
pub(crate) struct FontFaceData {
  pub data: Arc<Vec<u8>>,
  pub index: u32,
  id: Arc<str>,
}

impl PartialEq for FontFaceData {
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index && self.id == other.id
  }
}

impl Eq for FontFaceData {}

impl Hash for FontFaceData {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.index.hash(state);
    self.id.hash(state);
  }
}

pub(crate) fn load_text_face(style: &TextStyle) -> Option<FontFaceData> {
  load_face(style.font_family.as_deref(), style.bold, style.italic)
}

pub(crate) fn load_text_faces(style: &TextStyle) -> Vec<FontFaceData> {
  load_faces(style.font_family.as_deref(), style.bold, style.italic)
}

fn load_face(family: Option<&str>, bold: bool, italic: bool) -> Option<FontFaceData> {
  load_faces(family, bold, italic).into_iter().next()
}

fn load_faces(family: Option<&str>, bold: bool, italic: bool) -> Vec<FontFaceData> {
  let key = FontFaceKey {
    family: family
      .filter(|family| !family.trim().is_empty())
      .map(ToOwned::to_owned),
    bold,
    italic,
  };
  if let Ok(cache) = font_face_cache().lock()
    && let Some(faces) = cache.get(&key)
  {
    return faces.clone();
  }

  let mut families = Vec::new();
  if let Some(family) = family.filter(|family| !family.trim().is_empty()) {
    push_requested_font_families(&mut families, family);
  } else {
    push_generic_font_families(&mut families);
  }

  let mut faces = Vec::new();
  if let Some(face) = query_font_face(&families, family, bold, italic) {
    push_unique_font_face(&mut faces, face);
  }

  let fallback_paths = family
    .filter(|family| !family.trim().is_empty())
    .map(|family| specific_fallback_font_paths(family, bold, italic))
    .unwrap_or_else(|| generic_fallback_font_paths(bold, italic));

  for path in fallback_paths {
    let Ok(data) = std::fs::read(path) else {
      continue;
    };
    if ttf_parser::Face::parse(data.as_slice(), 0).is_ok() {
      push_unique_font_face(&mut faces, font_face_data(data, 0, path.to_string()));
    }
  }

  // Source: LibreOffice vcl/unx/generic/fontmanager/fontconfig.cxx
  // PrintFontManager::Substitute asks fontconfig for a replacement when the
  // requested family is unavailable. Typst's text shaping follows the same
  // broad model by selecting a fallback font after the requested families are
  // exhausted. Keep named-family fallback explicit so exact installed fonts and
  // source-backed aliases remain preferred.
  if family.is_some_and(|family| !family.trim().is_empty()) {
    let mut fallback_families = Vec::new();
    push_generic_font_families(&mut fallback_families);
    if let Some(face) = query_font_face(&fallback_families, family, bold, italic) {
      push_unique_font_face(&mut faces, face);
    }
  }

  if family.is_some_and(|family| !family.trim().is_empty()) {
    for path in generic_fallback_font_paths(bold, italic) {
      let Ok(data) = std::fs::read(path) else {
        continue;
      };
      if ttf_parser::Face::parse(data.as_slice(), 0).is_ok() {
        push_unique_font_face(&mut faces, font_face_data(data, 0, path.to_string()));
      }
    }
  }

  if let Ok(mut cache) = font_face_cache().lock() {
    cache.insert(key, faces.clone());
  }
  faces
}

fn push_unique_font_face(faces: &mut Vec<FontFaceData>, face: FontFaceData) {
  if !faces.contains(&face) {
    faces.push(face);
  }
}

fn font_face_data(data: Vec<u8>, index: u32, id: String) -> FontFaceData {
  FontFaceData {
    data: Arc::new(data),
    index,
    id: Arc::from(id),
  }
}

fn query_font_face(
  families: &[fontdb::Family<'_>],
  family: Option<&str>,
  bold: bool,
  italic: bool,
) -> Option<FontFaceData> {
  if let Some(id) = font_db().query(&fontdb::Query {
    families,
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
    return Some(font_face_data(
      data,
      index,
      format!("fontdb:{id:?}:{index}"),
    ));
  }

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

fn font_face_cache() -> &'static Mutex<HashMap<FontFaceKey, Vec<FontFaceData>>> {
  static CACHE: OnceLock<Mutex<HashMap<FontFaceKey, Vec<FontFaceData>>>> = OnceLock::new();
  CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn query_weight(family: Option<&str>, bold: bool) -> fontdb::Weight {
  if family.is_some_and(|family| family.eq_ignore_ascii_case("Arial Black")) {
    return fontdb::Weight::BLACK;
  }
  if family.is_some_and(|family| family.eq_ignore_ascii_case("Calibri Light")) && !bold {
    return fontdb::Weight::LIGHT;
  }
  if bold {
    fontdb::Weight::BOLD
  } else {
    fontdb::Weight::NORMAL
  }
}

fn push_requested_font_families<'a>(families: &mut Vec<fontdb::Family<'a>>, family: &'a str) {
  for family in family
    .split(';')
    .map(str::trim)
    .filter(|family| !family.is_empty())
  {
    families.push(fontdb::Family::Name(family));
    push_font_aliases(families, family);
  }
}

fn push_font_aliases<'a>(families: &mut Vec<fontdb::Family<'a>>, family: &'a str) {
  match family.trim().to_ascii_lowercase().as_str() {
    "courier" => families.push(fontdb::Family::Name("Courier New")),
    "timesnewromanpsmt" => families.push(fontdb::Family::Name("Times New Roman")),
    "dinpro-medium" => families.push(fontdb::Family::Name("DINPro")),
    "univers 45 light" => families.extend([
      fontdb::Family::Name("Univers Light"),
      fontdb::Family::Name("Univers"),
    ]),
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
    "游ゴシック" | "yu gothic" => families.extend([
      fontdb::Family::Name("Yu Gothic"),
      fontdb::Family::Name("Noto Sans CJK JP"),
    ]),
    "biz ud明朝" | "biz ud明朝 medium" | "biz udmincho" | "biz udmincho medium" => families
      .extend([
        fontdb::Family::Name("BIZ UDMincho"),
        fontdb::Family::Name("BIZ UDMincho Medium"),
        fontdb::Family::Name("Noto Serif CJK JP"),
      ]),
    _ => {}
  }
}

fn push_generic_font_families(families: &mut Vec<fontdb::Family<'_>>) {
  families.push(fontdb::Family::SansSerif);
  families.push(fontdb::Family::Serif);
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

  if family.eq_ignore_ascii_case("Calibri Light") {
    return match (bold, italic) {
      (true, true) => &[
        "/usr/share/fonts/truetype/Fonts/calibriz.ttf",
        "/usr/share/fonts/truetype/Fonts/calibrib.ttf",
      ],
      (true, false) => &[
        "/usr/share/fonts/truetype/Fonts/calibrib.ttf",
        "/usr/share/fonts/truetype/Fonts/calibril.ttf",
      ],
      (false, true) => &[
        "/usr/share/fonts/truetype/Fonts/calibrili.ttf",
        "/usr/share/fonts/truetype/Fonts/calibril.ttf",
      ],
      (false, false) => &["/usr/share/fonts/truetype/Fonts/calibril.ttf"],
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

  if family.eq_ignore_ascii_case("TimesNewRomanPSMT") {
    return match italic {
      true => &[
        "/usr/share/fonts/truetype/msttcorefonts/timesi.ttf",
        "/usr/share/fonts/truetype/msttcorefonts/Times_New_Roman_Italic.ttf",
      ],
      false => &[
        "/usr/share/fonts/truetype/msttcorefonts/times.ttf",
        "/usr/share/fonts/truetype/msttcorefonts/Times_New_Roman.ttf",
      ],
    };
  }

  if family.eq_ignore_ascii_case("Courier") {
    return match (bold, italic) {
      (true, true) => &[
        "/usr/share/fonts/truetype/msttcorefonts/courbi.ttf",
        "/usr/share/fonts/truetype/msttcorefonts/Courier_New_Bold_Italic.ttf",
      ],
      (true, false) => &[
        "/usr/share/fonts/truetype/msttcorefonts/courbd.ttf",
        "/usr/share/fonts/truetype/msttcorefonts/Courier_New_Bold.ttf",
      ],
      (false, true) => &[
        "/usr/share/fonts/truetype/msttcorefonts/couri.ttf",
        "/usr/share/fonts/truetype/msttcorefonts/Courier_New_Italic.ttf",
      ],
      (false, false) => &[
        "/usr/share/fonts/truetype/msttcorefonts/cour.ttf",
        "/usr/share/fonts/truetype/msttcorefonts/Courier_New.ttf",
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

  if family.eq_ignore_ascii_case("Yu Gothic") || family.eq_ignore_ascii_case("游ゴシック") {
    return match (bold, italic) {
      (true, true) => &[
        "/usr/share/fonts/truetype/Fonts/YuGothB.ttc",
        "/usr/share/fonts/truetype/Fonts/YuGothR.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
      ],
      (true, false) => &[
        "/usr/share/fonts/truetype/Fonts/YuGothB.ttc",
        "/usr/share/fonts/truetype/Fonts/YuGothR.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
      ],
      (false, true) => &[
        "/usr/share/fonts/truetype/Fonts/YuGothR.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
      ],
      (false, false) => &[
        "/usr/share/fonts/truetype/Fonts/YuGothR.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
      ],
    };
  }

  if family.eq_ignore_ascii_case("BIZ UD明朝 Medium")
    || family.eq_ignore_ascii_case("BIZ UD明朝")
    || family.eq_ignore_ascii_case("BIZ UDMincho Medium")
    || family.eq_ignore_ascii_case("BIZ UDMincho")
  {
    return match (bold, italic) {
      (true, true) | (true, false) | (false, true) | (false, false) => &[
        "/usr/share/fonts/truetype/Fonts/BIZ-UDMinchoM.ttc",
        "/usr/share/fonts/opentype/noto/NotoSerifCJK-Regular.ttc",
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
      "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
      "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    ],
    (true, false) => &[
      "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
      "/usr/share/fonts/opentype/noto/NotoSansCJK-Bold.ttc",
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
      "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    ],
    (false, true) => &[
      "/usr/share/fonts/truetype/dejavu/DejaVuSans-Oblique.ttf",
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
      "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
    ],
    (false, false) => &[
      "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
      "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
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
  fn missing_named_font_uses_system_fallback() {
    let style = TextStyle {
      font_family: Some(Arc::from("CodexDefinitelyMissingFont")),
      ..Default::default()
    };

    assert!(load_text_face(&style).is_some());
  }

  #[test]
  fn din_bold_uses_system_fallback_when_family_is_not_installed() {
    let style = TextStyle {
      font_family: Some(Arc::from("DIN-Bold")),
      ..Default::default()
    };

    assert!(load_text_face(&style).is_some());
  }
}
