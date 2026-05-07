#[derive(Clone, Debug)]
pub(crate) struct FontFaceData {
  pub data: Vec<u8>,
  pub index: u32,
}

pub(crate) fn load_sans_face(bold: bool, italic: bool) -> Option<FontFaceData> {
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
  {
    return Some(FontFaceData { data, index });
  }

  for path in fallback_font_paths(bold, italic) {
    let Ok(data) = std::fs::read(path) else {
      continue;
    };
    if ttf_parser::Face::parse(&data, 0).is_ok() {
      return Some(FontFaceData { data, index: 0 });
    }
  }

  None
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
