use std::{
  borrow::Cow,
  collections::{HashMap, HashSet},
};

use super::{StyleEntry, StylesCatalog, w};

// [MS-OI29500] §2.1.237(d), name (§17.7.4.9): canonical reserved paragraph
// names, in ASCII order. Custom styles and other style types have independent
// identities even when their displayed names match.
const BUILTIN_PARAGRAPH_NAMES: &[&str] = &[
  "annotation subject",
  "annotation text",
  "balloon text",
  "bibliography",
  "block text",
  "body text",
  "body text 2",
  "body text 3",
  "body text first indent",
  "body text first indent 2",
  "body text indent",
  "body text indent 2",
  "body text indent 3",
  "caption",
  "closing",
  "date",
  "document map",
  "e-mail signature",
  "endnote text",
  "envelope address",
  "envelope return",
  "footer",
  "footnote text",
  "header",
  "heading 1",
  "heading 2",
  "heading 3",
  "heading 4",
  "heading 5",
  "heading 6",
  "heading 7",
  "heading 8",
  "heading 9",
  "html address",
  "html bottom of form",
  "html preformatted",
  "html top of form",
  "index 1",
  "index 2",
  "index 3",
  "index 4",
  "index 5",
  "index 6",
  "index 7",
  "index 8",
  "index 9",
  "index heading",
  "intense quote",
  "list",
  "list 2",
  "list 3",
  "list 4",
  "list 5",
  "list bullet",
  "list bullet 2",
  "list bullet 3",
  "list bullet 4",
  "list bullet 5",
  "list continue",
  "list continue 2",
  "list continue 3",
  "list continue 4",
  "list continue 5",
  "list number",
  "list number 2",
  "list number 3",
  "list number 4",
  "list number 5",
  "list paragraph",
  "macro",
  "message header",
  "no spacing",
  "normal",
  "normal (web)",
  "normal indent",
  "note heading",
  "plain text",
  "quote",
  "revision",
  "salutation",
  "signature",
  "subtitle",
  "table of authorities",
  "table of figures",
  "title",
  "toa heading",
  "toc 1",
  "toc 2",
  "toc 3",
  "toc 4",
  "toc 5",
  "toc 6",
  "toc 7",
  "toc 8",
  "toc 9",
  "toc heading",
];

fn builtin_paragraph_name(entry: &StyleEntry) -> Option<&'static str> {
  if entry.custom_style || entry.style_type != Some(w::StyleValues::Paragraph) {
    return None;
  }
  let name = entry.name.as_deref()?.to_ascii_lowercase();
  BUILTIN_PARAGRAPH_NAMES
    .binary_search(&name.as_str())
    .ok()
    .map(|index| BUILTIN_PARAGRAPH_NAMES[index])
}

pub(super) fn reconcile_builtin_paragraph_definitions(catalog: &mut StylesCatalog) {
  // Word imports built-ins by canonical name, not producer styleId. Native
  // duplicate/order/rename controls replace the complete earlier definition
  // with the last one, including omitted properties. Custom-name/customStyle
  // opposites retain separate styles. Keep every source ID usable by pStyle
  // and basedOn, and resolve this before links and inherited formats.
  let mut winners: HashMap<&str, (&str, &StyleEntry)> = HashMap::new();
  for (id, entry) in &catalog.styles {
    let Some(name) = builtin_paragraph_name(entry) else {
      continue;
    };
    let winner = winners.entry(name).or_insert((id, entry));
    if entry.source_order > winner.1.source_order {
      *winner = (id, entry);
    }
  }
  let replacements: Vec<_> = catalog
    .styles
    .iter()
    .filter_map(|(id, entry)| {
      let (winner_id, winner) = winners.get(builtin_paragraph_name(entry)?)?;
      (id != winner_id).then(|| (id.clone(), (*winner).clone()))
    })
    .collect();
  for (id, entry) in replacements {
    catalog.styles.insert(id, entry);
  }
}

fn custom_reserved_paragraph_name<'a>(
  entry: &'a StyleEntry,
  ui_language: Option<&str>,
) -> Option<Cow<'a, str>> {
  if !entry.custom_style || entry.style_type != Some(w::StyleValues::Paragraph) {
    return None;
  }
  let name = entry.name.as_deref()?;
  let canonical = name.to_ascii_lowercase();
  BUILTIN_PARAGRAPH_NAMES
    .binary_search(&canonical.as_str())
    .ok()?;
  if let Some(native) =
    super::field_localization::native_builtin_style_ref_name(name, entry.style_type, ui_language)
  {
    return Some(Cow::Borrowed(native.name));
  }
  let locale = ui_language.and_then(crate::localization::canonical_locale)?;
  match locale.id.language.as_str() {
    "en" => Some(Cow::Borrowed(name)),
    "fr" => {
      // Actual NameLocal observations cover these reserved names and all
      // nine French headings. Other UI names keep their existing recovery.
      let native = match canonical.as_str() {
        "signature" => "Signature",
        "normal" => "Normal",
        "body text" => "Corps de texte",
        "toc 1" => "TM 1",
        _ => {
          const HEADINGS: [&str; 9] = [
            "Titre 1", "Titre 2", "Titre 3", "Titre 4", "Titre 5", "Titre 6", "Titre 7", "Titre 8",
            "Titre 9",
          ];
          let level =
            super::field_localization::canonical_builtin_heading_level(name, entry.style_type)?;
          HEADINGS[usize::from(level - 1)]
        }
      };
      Some(Cow::Borrowed(native))
    }
    _ => None,
  }
}

pub(super) fn recover_custom_paragraph_names(catalog: &mut StylesCatalog) {
  let ui_language = catalog.locales.ui_language();
  if !catalog
    .styles
    .values()
    .any(|entry| custom_reserved_paragraph_name(entry, ui_language).is_some())
  {
    return;
  }
  // A custom style cannot claim its built-in's primary name. Word first
  // localizes that name, then appends the first available positive suffix.
  // Import order matters: a later ordinary name does not reserve a suffix
  // retroactively. Only the display name changes; pStyle/basedOn/link IDs
  // and the independent custom definition retain their source identities.
  let mut order: Vec<_> = catalog
    .styles
    .iter()
    .map(|(id, entry)| (entry.source_order, id.clone()))
    .collect();
  order.sort_unstable();
  let mut occupied = HashSet::with_capacity(order.len());
  for (_, id) in order {
    let entry = catalog.styles.get_mut(&id).expect("declared style");
    if let Some(base) = custom_reserved_paragraph_name(entry, ui_language) {
      let mut suffix = 1;
      let name = loop {
        let candidate = format!("{base}{suffix}");
        if !occupied.contains(&candidate.to_lowercase()) {
          break candidate;
        }
        suffix += 1;
      };
      entry.name = Some(name);
    }
    if let Some(name) = &entry.name {
      occupied.insert(name.to_lowercase());
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::{ImportSettings, ParagraphFormat, RunStyleOverrides, WordprocessingDocument};
  use super::*;
  use crate::{localization::OfficeLocaleContext, model::TextStyle};
  use ooxmlsdk::{
    parts::style_definitions_part::StyleDefinitionsPart, sdk::WordprocessingDocumentType,
  };

  fn load(definitions: &str) -> StylesCatalog {
    load_for_ui(definitions, "ja-JP")
  }

  fn load_for_ui(definitions: &str, ui_language: &str) -> StylesCatalog {
    let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
    let main = package.add_main_document_part().unwrap();
    let styles = main
      .add_new_part_auto_id::<_, StyleDefinitionsPart>(&mut package)
      .unwrap();
    styles.set_data(&mut package, format!(r#"<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
      <w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Arial"/><w:sz w:val="22"/><w:szCs w:val="22"/></w:rPr></w:rPrDefault><w:pPrDefault><w:pPr><w:spacing w:after="160"/></w:pPr></w:pPrDefault></w:docDefaults>
      <w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style>
      {definitions}</w:styles>"#).into_bytes()).unwrap();
    StylesCatalog::load(
      &package,
      &main,
      ImportSettings::default(),
      &OfficeLocaleContext::new(Some(ui_language), None, None),
      None,
    )
    .unwrap()
  }

  #[test]
  fn custom_reserved_names_use_the_native_display_namespace() {
    for (name, english, french) in [
      ("Signature", "Signature1", "Signature1"),
      ("Body Text", "Body Text1", "Corps de texte1"),
      ("heading 1", "Heading 11", "Titre 11"),
      ("Normal", "Normal1", "Normal1"),
      ("Date", "Date1", "Date1"),
      ("toc 1", "TOC 11", "TM 11"),
    ] {
      for (ui, expected) in [("en-US", english), ("fr-CA", french)] {
        let catalog = load_for_ui(
          &format!(
            r#"<w:style w:type="paragraph" w:customStyle="1" w:styleId="Opaque"><w:name w:val="{name}"/><w:basedOn w:val="Normal"/><w:rPr><w:sz w:val="20"/></w:rPr></w:style>"#
          ),
          ui,
        );
        assert!(
          catalog.styles["Opaque"]
            .name
            .as_deref()
            .unwrap()
            .eq_ignore_ascii_case(expected),
          "{name}/{ui}"
        );
        assert_eq!(
          catalog
            .run_style_with_base(
              Some("Opaque"),
              TextStyle::default(),
              RunStyleOverrides::default()
            )
            .font_size_pt,
          10.0
        );
      }
    }
  }

  #[test]
  fn custom_reserved_suffixes_use_only_preceding_names() {
    let occupied = r#"<w:style w:type="paragraph" w:customStyle="1" w:styleId="Occupied"><w:name w:val="Signature1"/></w:style>"#;
    let signature = r#"<w:style w:type="paragraph" w:customStyle="1" w:styleId="Opaque"><w:name w:val="Signature"/><w:basedOn w:val="Normal"/><w:rPr><w:sz w:val="20"/></w:rPr></w:style>"#;
    for before in [false, true] {
      let definitions = if before {
        format!("{occupied}{signature}")
      } else {
        format!("{signature}{occupied}")
      };
      let catalog = load_for_ui(&definitions, "fr-CA");
      assert_eq!(
        catalog.styles["Opaque"].name.as_deref(),
        Some(if before { "Signature2" } else { "Signature1" })
      );
      // Native Word retains the later ordinary display name, even when it
      // now duplicates the recovered name. XML IDs remain independent.
      assert_eq!(
        catalog.styles["Occupied"].name.as_deref(),
        Some("Signature1")
      );
    }
  }

  #[test]
  fn native_builtin_duplicates_replace_complete_properties_before_inheritance() {
    for name in [
      "footnote text",
      "endnote text",
      "heading 2",
      "Body Text",
      "Subtitle",
    ] {
      for reverse in [false, true] {
        let first = format!(
          r#"<w:style w:type="paragraph" w:styleId="First"><w:name w:val="{name}"/><w:basedOn w:val="Normal"/><w:pPr><w:bidi/><w:spacing w:after="0"/></w:pPr><w:rPr><w:rFonts w:ascii="Traditional Arabic" w:hAnsi="Traditional Arabic" w:cs="Traditional Arabic"/><w:sz w:val="20"/><w:szCs w:val="20"/></w:rPr></w:style>"#
        );
        let upper = name.to_ascii_uppercase();
        let last = format!(
          r#"<w:style w:type="paragraph" w:styleId="Last"><w:name w:val="{upper}"/><w:basedOn w:val="Normal"/><w:pPr/><w:rPr/></w:style>"#
        );
        let definitions = if reverse {
          format!("{last}{first}")
        } else {
          format!("{first}{last}")
        };
        let catalog = load(&format!(
          r#"{definitions}<w:style w:type="paragraph" w:customStyle="1" w:styleId="Derived"><w:name w:val="Descendant"/><w:basedOn w:val="First"/></w:style>"#
        ));
        for id in ["First", "Last", "Derived"] {
          let run = catalog.run_style_with_base(
            Some(id),
            TextStyle::default(),
            RunStyleOverrides::default(),
          );
          let paragraph = catalog.paragraph_format_with_base(Some(id), ParagraphFormat::default());
          assert_eq!(
            run.font_family.as_deref(),
            Some(if reverse {
              "Traditional Arabic"
            } else {
              "Calibri"
            }),
            "{name}/{reverse}/{id}"
          );
          assert_eq!(
            run.complex_font_family.as_deref(),
            Some(if reverse {
              "Traditional Arabic"
            } else {
              "Arial"
            })
          );
          assert_eq!(run.font_size_pt, if reverse { 10.0 } else { 11.0 });
          assert_eq!(paragraph.bidi, reverse);
          assert_eq!(paragraph.spacing_after_pt, if reverse { 0.0 } else { 8.0 });
        }
      }
    }
  }

  #[test]
  fn native_custom_names_and_custom_flags_keep_source_style_identity() {
    for (name, first_custom, last_custom) in [
      ("Probe Note", false, false),
      ("Probe Note", true, true),
      ("footnote text", true, true),
      ("footnote text", true, false),
      ("footnote text", false, true),
    ] {
      for reverse in [false, true] {
        let first = format!(
          r#"<w:style w:type="paragraph" w:customStyle="{}" w:styleId="First"><w:name w:val="{name}"/><w:basedOn w:val="Normal"/><w:rPr><w:sz w:val="20"/></w:rPr></w:style>"#,
          u8::from(first_custom)
        );
        let upper = name.to_ascii_uppercase();
        let last = format!(
          r#"<w:style w:type="paragraph" w:customStyle="{}" w:styleId="Last"><w:name w:val="{upper}"/><w:basedOn w:val="Normal"/></w:style>"#,
          u8::from(last_custom)
        );
        let catalog = load(&if reverse {
          format!("{last}{first}")
        } else {
          format!("{first}{last}")
        });
        for (id, size) in [("First", 10.0), ("Last", 11.0)] {
          assert_eq!(
            catalog
              .run_style_with_base(Some(id), TextStyle::default(), RunStyleOverrides::default())
              .font_size_pt,
            size,
            "{name}/{first_custom}/{last_custom}/{reverse}/{id}"
          );
        }
      }
    }
  }
}
