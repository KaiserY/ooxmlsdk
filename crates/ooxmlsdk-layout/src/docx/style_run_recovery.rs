//! Word imports a paragraph style's nested pPr/rPr into its ordinary run
//! properties. Both containers contribute in document order; the typed schema
//! retains their values, but not that order. Only affected styles need this
//! additional lexical pass. Ordinary styles keep the existing typed fast path.

use std::collections::HashSet;

use quick_xml::{events::Event, name::ResolveResult};

use super::{
  LegacyTextRelief, RunProps, RunStyleOverrides, StyleEntry, StylesCatalog, TextStyle,
  imported_style_definition_type, merge_run_style_overrides, properties, run_style_overrides, w,
};

const WORD: &[u8] = b"http://schemas.openxmlformats.org/wordprocessingml/2006/main";

pub(super) fn has_nested_run_properties(style: &w::Style) -> bool {
  matches!(
    imported_style_definition_type(style),
    w::StyleValues::Paragraph
  ) && style
    .style_paragraph_properties
    .as_ref()
    .is_some_and(|properties| properties.paragraph_mark_run_properties.is_some())
}

pub(super) fn import(
  catalog: &StylesCatalog,
  entry: &mut StyleEntry,
  style: &w::Style,
  nested_last: bool,
) {
  let nested = has_nested_run_properties(style)
    .then(|| {
      style
        .style_paragraph_properties
        .as_deref()?
        .paragraph_mark_run_properties
        .as_deref()
        .map(RunProps::ParagraphMark)
    })
    .flatten();
  let direct = style.style_run_properties.as_deref().map(RunProps::Style);
  let layers = if nested_last {
    [direct, nested]
  } else {
    [nested, direct]
  };
  let mut previous = RunStyleOverrides::default();
  for layer in layers.into_iter().flatten() {
    let overrides = run_style_overrides(Some(layer));
    properties::merge_run_style(
      &mut entry.run_style,
      Some(layer),
      &catalog.theme_fonts,
      &catalog.theme_colors,
      catalog.import_settings.compatibility_mode,
    );
    // Word keeps paragraph-style highlighting, but ignores the same element
    // in a character style while retaining that style's independent shading.
    // Native paragraph/character/default controls establish this boundary.
    if matches!(entry.style_type, Some(w::StyleValues::Paragraph))
      && let RunProps::Style(properties) = layer
      && let Some(highlight) = &properties.highlight
    {
      entry.run_style.highlight =
        properties::highlight_color(highlight.val, catalog.import_settings.compatibility_mode);
    }
    catalog.apply_word_font_table_mappings(&mut entry.run_style, layer.run_fonts());
    entry.recovered_run_toggles = InheritedToggles::combine(previous, overrides);
    entry.run_overrides = merge_run_style_overrides(entry.run_overrides, overrides);
    previous = overrides;
  }
}

// Native Word controls distinguish a single absolute style value from two
// imported containers. Each container describes a change relative to the same
// basedOn value: their toggle deltas accumulate. Thus two authored values A/B
// resolve to inherited XOR A XOR B. Missing properties contribute no delta.
// Keep that dependency until the paragraph style chain supplies its base.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct InheritedToggles {
  bold: Option<bool>,
  complex_bold: Option<bool>,
  italic: Option<bool>,
  complex_italic: Option<bool>,
  strikethrough: Option<bool>,
  uppercase: Option<bool>,
  small_caps: Option<bool>,
  hidden: Option<bool>,
  legacy_outline: Option<bool>,
  legacy_shadow: Option<bool>,
  legacy_emboss: Option<bool>,
  legacy_imprint: Option<bool>,
}

impl InheritedToggles {
  fn combine(first: RunStyleOverrides, second: RunStyleOverrides) -> Self {
    fn delta(a: Option<bool>, b: Option<bool>) -> Option<bool> {
      a.zip(b).map(|(a, b)| a ^ b)
    }
    Self {
      bold: delta(first.bold, second.bold),
      complex_bold: delta(first.complex_bold, second.complex_bold),
      italic: delta(first.italic, second.italic),
      complex_italic: delta(first.complex_italic, second.complex_italic),
      strikethrough: delta(first.strikethrough, second.strikethrough),
      uppercase: delta(first.uppercase, second.uppercase),
      small_caps: delta(first.small_caps, second.small_caps),
      hidden: delta(first.hidden, second.hidden),
      legacy_outline: delta(first.legacy_outline, second.legacy_outline),
      legacy_shadow: delta(first.legacy_shadow, second.legacy_shadow),
      legacy_emboss: delta(first.legacy_emboss, second.legacy_emboss),
      legacy_imprint: delta(first.legacy_imprint, second.legacy_imprint),
    }
  }

  pub(super) fn resolve(
    self,
    base: &TextStyle,
    mut overrides: RunStyleOverrides,
  ) -> RunStyleOverrides {
    fn apply(target: &mut Option<bool>, delta: Option<bool>, base: bool) {
      if let Some(delta) = delta {
        *target = Some(base ^ delta);
      }
    }
    apply(&mut overrides.bold, self.bold, base.bold);
    apply(
      &mut overrides.complex_bold,
      self.complex_bold,
      base.complex_bold.unwrap_or(false),
    );
    apply(&mut overrides.italic, self.italic, base.italic);
    apply(
      &mut overrides.complex_italic,
      self.complex_italic,
      base.complex_italic.unwrap_or(false),
    );
    apply(
      &mut overrides.strikethrough,
      self.strikethrough,
      base.strikethrough,
    );
    apply(&mut overrides.uppercase, self.uppercase, base.uppercase);
    apply(&mut overrides.small_caps, self.small_caps, base.small_caps);
    apply(&mut overrides.hidden, self.hidden, base.hidden);
    apply(
      &mut overrides.legacy_outline,
      self.legacy_outline,
      base.legacy_outline,
    );
    apply(
      &mut overrides.legacy_shadow,
      self.legacy_shadow,
      base.legacy_shadow,
    );
    apply(
      &mut overrides.legacy_emboss,
      self.legacy_emboss,
      base.legacy_relief == LegacyTextRelief::Embossed,
    );
    apply(
      &mut overrides.legacy_imprint,
      self.legacy_imprint,
      base.legacy_relief == LegacyTextRelief::Engraved,
    );
    overrides
  }
}

pub(super) fn nested_after_direct(xml: &[u8]) -> HashSet<usize> {
  let mut reader = quick_xml::NsReader::from_reader(xml);
  let mut depth = 0;
  let mut ordinal = 0;
  let mut style = None;
  let mut paragraph = false;
  let mut direct = false;
  let mut result = HashSet::new();
  loop {
    match reader.read_resolved_event() {
      Ok((namespace, event @ (Event::Start(_) | Event::Empty(_)))) => {
        let empty = matches!(event, Event::Empty(_));
        let (Event::Start(element) | Event::Empty(element)) = event else {
          unreachable!()
        };
        let word = matches!(namespace, ResolveResult::Bound(uri) if uri.as_ref() == WORD);
        let name = element.local_name();
        if word && depth == 1 && name.as_ref() == b"style" {
          style = Some(ordinal);
          ordinal += 1;
          direct = false;
        } else if word && style.is_some() && depth == 2 {
          if name.as_ref() == b"rPr" {
            direct = true;
          } else if name.as_ref() == b"pPr" && !empty {
            paragraph = true;
          }
        } else if word
          && depth == 3
          && paragraph
          && name.as_ref() == b"rPr"
          && direct
          && let Some(index) = style
        {
          result.insert(index);
        }
        if !empty {
          depth += 1;
        } else if depth == 1 {
          style = None;
        }
      }
      Ok((_, Event::End(_))) => {
        depth -= 1;
        if depth == 2 {
          paragraph = false;
        } else if depth == 1 {
          style = None;
        }
      }
      Ok((_, Event::Eof)) | Err(_) => break,
      _ => {}
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::super::{ImportSettings, WordprocessingDocument};
  use super::*;
  use crate::localization::OfficeLocaleContext;
  use ooxmlsdk::{
    parts::style_definitions_part::StyleDefinitionsPart, sdk::WordprocessingDocumentType,
  };

  fn catalog(base: &str, kind: &str, layers: &str) -> StylesCatalog {
    let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
    let main = package.add_main_document_part().unwrap();
    let part = main
      .add_new_part_auto_id::<_, StyleDefinitionsPart>(&mut package)
      .unwrap();
    part
      .set_data(
        &mut package,
        format!(
          r#"<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
      <w:style w:type="paragraph" w:styleId="Base"><w:rPr>{base}</w:rPr></w:style>
      <w:style w:type="{kind}" w:styleId="Recovered"><w:basedOn w:val="Base"/>{layers}</w:style>
    </w:styles>"#
        )
        .into_bytes(),
      )
      .unwrap();
    StylesCatalog::load(
      &package,
      &main,
      ImportSettings::default(),
      &OfficeLocaleContext::default(),
      None,
    )
    .unwrap()
  }

  fn resolved(base: &str, layers: &str) -> TextStyle {
    let catalog = catalog(base, "paragraph", layers);
    catalog.run_style_with_base(
      Some("Recovered"),
      TextStyle::default(),
      RunStyleOverrides::default(),
    )
  }

  #[test]
  fn character_style_ignores_highlight_but_retains_shading() {
    for (properties, expected) in [
      (r#"<w:highlight w:val="yellow"/>"#, None),
      (
        r#"<w:highlight w:val="yellow"/><w:shd w:val="clear" w:fill="FF8000"/>"#,
        Some(super::super::RgbColor {
          r: 255,
          g: 128,
          b: 0,
        }),
      ),
    ] {
      let catalog = catalog("", "character", &format!("<w:rPr>{properties}</w:rPr>"));
      let actual = catalog.character_run_style(Some("Recovered"), TextStyle::default());
      assert_eq!(actual.highlight, expected);
    }
  }

  #[test]
  fn nested_paragraph_style_properties_recover_in_source_order() {
    for (layers, size, color) in [
      (
        r#"<w:pPr><w:rPr><w:highlight w:val="green"/></w:rPr></w:pPr><w:rPr/>"#,
        24.0,
        Some(super::super::RgbColor { r: 0, g: 255, b: 0 }),
      ),
      (
        r#"<w:pPr><w:rPr><w:sz w:val="40"/><w:highlight w:val="green"/></w:rPr></w:pPr><w:rPr><w:sz w:val="64"/><w:highlight w:val="cyan"/></w:rPr>"#,
        32.0,
        Some(super::super::RgbColor {
          r: 0,
          g: 255,
          b: 255,
        }),
      ),
      (
        r#"<w:rPr><w:sz w:val="64"/><w:highlight w:val="cyan"/></w:rPr><w:pPr><w:rPr><w:sz w:val="40"/><w:highlight w:val="green"/></w:rPr></w:pPr>"#,
        20.0,
        Some(super::super::RgbColor { r: 0, g: 255, b: 0 }),
      ),
      (
        r#"<w:pPr><w:rPr><w:highlight w:val="green"/></w:rPr></w:pPr><w:rPr><w:highlight w:val="none"/></w:rPr>"#,
        24.0,
        None,
      ),
    ] {
      let actual = resolved(r#"<w:sz w:val="48"/>"#, layers);
      assert_eq!((actual.font_size_pt, actual.highlight), (size, color));
    }
  }

  #[test]
  fn recovered_style_toggles_use_the_shared_inherited_basis() {
    // Native Word's 12-control truth table, including the unchanged single
    // container route. These outcomes come from actual body font observations.
    for (base, nested, direct, expected) in [
      (false, None, false, false),
      (false, None, true, true),
      (false, Some(false), false, false),
      (false, Some(false), true, true),
      (false, Some(true), false, true),
      (false, Some(true), true, false),
      (true, None, false, false),
      (true, None, true, true),
      (true, Some(false), false, true),
      (true, Some(false), true, false),
      (true, Some(true), false, false),
      (true, Some(true), true, true),
    ] {
      let base = format!(r#"<w:b w:val="{}"/>"#, u8::from(base));
      let nested = nested.map_or(String::new(), |value| {
        format!(
          r#"<w:pPr><w:rPr><w:b w:val="{}"/></w:rPr></w:pPr>"#,
          u8::from(value)
        )
      });
      let layers = format!(
        r#"{nested}<w:rPr><w:b w:val="{}"/></w:rPr>"#,
        u8::from(direct)
      );
      assert_eq!(resolved(&base, &layers).bold, expected, "{base} {layers}");
    }
  }

  #[test]
  fn recovery_order_tracks_style_ordinals_and_namespaces() {
    let xml = br#"<s:styles xmlns:s="http://schemas.openxmlformats.org/wordprocessingml/2006/main" xmlns:other="urn:other">
      <s:style/>
      <s:style><s:pPr><s:rPr/></s:pPr><s:rPr/></s:style>
      <s:style><s:rPr/><s:pPr><s:rPr/></s:pPr></s:style>
      <s:style><s:rPr/><other:pPr><s:rPr/></other:pPr></s:style>
      <s:style><s:rPr/><s:pPr><other:rPr/></s:pPr></s:style>
      <s:style><s:rPr/><s:tblStylePr><s:pPr><s:rPr/></s:pPr></s:tblStylePr></s:style>
      <s:style><s:rPr></s:rPr><s:pPr><s:rPr/></s:pPr></s:style>
    </s:styles>"#;
    assert_eq!(nested_after_direct(xml), HashSet::from([2, 6]));
  }
}
