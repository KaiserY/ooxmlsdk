use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;

use ooxmlsdk::parts::{
  font_part::FontPart, main_document_part::MainDocumentPart,
  wordprocessing_document::WordprocessingDocument,
};
use ooxmlsdk::schemas::schemas_openxmlformats_org_wordprocessingml_2006_main as w;
use ooxmlsdk::sdk::SdkPart;
use ooxmlsdk_fonts::{FontRegistry, FontRequest};

use crate::fonts::EmbeddedFontSet;

pub(super) fn load(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> Option<Arc<EmbeddedFontSet>> {
  let part = main.font_table_part(package)?;
  let parts = part
    .related_parts_of_type::<_, FontPart>(package)
    .map(|related| {
      (
        related.relationship_id().to_string(),
        related.part().clone(),
      )
    })
    .collect::<HashMap<_, _>>();
  if parts.is_empty() {
    return None;
  }
  let table = part.root_element(package).ok()?;
  let mut system = FontRegistry::with_default_policy();
  let mut sources = Vec::new();
  for choice in &table.xml_children {
    let w::FontsChoice::Font(font) = choice else {
      continue;
    };
    if (font.embed_regular_font.is_none()
      && font.embed_bold_font.is_none()
      && font.embed_italic_font.is_none()
      && font.embed_bold_italic_font.is_none())
      || installed(&mut system, &font.name)
    {
      continue;
    }
    macro_rules! embedded {
      ($field:ident) => {
        if let Some(embedded) = &font.$field
          && let Some(part) = parts.get(embedded.id.as_str())
          && let Ok(data) = part.try_data_bytes(package)
          && let Some(data) = decode(&data, embedded.font_key.as_deref())
        {
          sources.push((format!("{}:{}", font.name, stringify!($field)), data));
        }
      };
    }
    embedded!(embed_regular_font);
    embedded!(embed_bold_font);
    embedded!(embed_italic_font);
    embedded!(embed_bold_italic_font);
  }
  EmbeddedFontSet::new(sources)
}

fn installed(system: &mut FontRegistry<'static>, family: &str) -> bool {
  // Native same-name Arial controls select the installed face even when
  // the document contains a different, valid private font with that name.
  // Query before reading font data. Substitution faces do not establish that
  // the authored family is installed.
  let _ = system.register_system_query_fonts(&FontRequest {
    family: Some(Cow::Borrowed(family)),
    ..FontRequest::default()
  });
  system.faces.iter().any(|face| {
    face
      .family_names
      .iter()
      .any(|name| name.eq_ignore_ascii_case(family))
  })
}

fn decode(data: &[u8], font_key: Option<&str>) -> Option<Vec<u8>> {
  let mut data = data.to_vec();
  let Some(key) = font_key else {
    return Some(data);
  };
  // ECMA-376 Part1 §17.8.1: reverse the sixteen GUID bytes, then XOR
  // that key twice across the first32 bytes. This is not GUID's mixed-endian
  // in-memory Windows representation. The remainder of the font is intact.
  let hex = key.trim().trim_matches(['{', '}']).replace('-', "");
  if hex.len() != 32 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) || data.len() < 32 {
    return None;
  }
  let mut key = [0_u8; 16];
  for (index, byte) in key.iter_mut().enumerate() {
    *byte = u8::from_str_radix(&hex[2 * index..2 * index + 2], 16).ok()?;
  }
  for (index, byte) in data[..32].iter_mut().enumerate() {
    *byte ^= key[15 - index % 16];
  }
  Some(data)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn embedded_font_fallback_queries_do_not_imply_an_installed_family() {
    let mut system = FontRegistry::with_default_policy();
    assert!(!installed(
      &mut system,
      "OOXMLSDK Missing Embedded Font 823a1"
    ));
    assert!(installed(&mut system, "Arial"));
  }

  #[test]
  fn embedded_font_guid_reversal_changes_only_the_first_thirty_two_bytes() {
    let key = "{00010203-0405-0607-0809-0A0B0C0D0E0F}";
    let source = [0_u8; 48];
    let decoded = decode(&source, Some(key)).unwrap();
    let expected = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    assert_eq!(&decoded[..16], &expected);
    assert_eq!(&decoded[16..32], &expected);
    assert_eq!(&decoded[32..], &[0_u8; 16]);
    assert_eq!(decode(&decoded, Some(key)).unwrap(), source);
    assert_eq!(decode(&source, None).unwrap(), source);
    assert!(decode(&source[..31], Some(key)).is_none());
    for key in ["bad", "{00010203-0405-0607-0809-0A0B0C0D0E0Z}"] {
      assert!(decode(&source, Some(key)).is_none());
    }
  }
}
