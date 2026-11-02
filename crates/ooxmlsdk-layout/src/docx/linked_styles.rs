//! ECMA-376 §17.15.1.55 and Word Document.UpdateStyles: a resolved document
//! template replaces styles by primary name. Package-local IDs are remapped;
//! source docDefaults and direct paragraph/run formatting remain independent.

use std::collections::{HashMap, HashSet};

use super::{
  MainDocumentPart, ParagraphProps, RunProps, StyleEntry, StylesCatalog, WordprocessingDocument,
  imported_style_definition_type, merge_paragraph_format_with_theme, merge_run_style_overrides,
  properties, run_style_overrides, style_run_recovery, w,
};

pub(super) struct UpdatedStyles {
  pub styles: w::Styles,
  pub nested_last: HashSet<usize>,
  root_paragraphs: HashSet<usize>,
  defaults: Option<Box<w::DocDefaults>>,
}

impl UpdatedStyles {
  pub fn defaults_for(&self, index: usize) -> Option<&w::DocDefaults> {
    self
      .root_paragraphs
      .contains(&index)
      .then_some(self.defaults.as_deref())
      .flatten()
  }
}

pub(super) fn load(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
  source: Option<&w::Styles>,
  template: Option<&[u8]>,
) -> Option<UpdatedStyles> {
  let template = template?;
  let settings = main
    .document_settings_part(package)?
    .root_element(package)
    .ok()?;
  if !settings
    .link_styles
    .as_ref()
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
  {
    return None;
  }
  // Invalid/unavailable templates are ignored, as required by linkStyles.
  // The host owns resolution, including explicit missing-template handling.
  let template = WordprocessingDocument::new(std::io::Cursor::new(template)).ok()?;
  let template_main = template.main_document_part().ok()?;
  let template_part = template_main.style_definitions_part(&template)?;
  let template_styles =
    super::indentation_recovery::root(&template_part.try_data_bytes(&template).ok()?, || {
      template_part.root_element(&template)
    })
    .ok()?;
  let nested_template = template_part
    .data(&template)
    .map(style_run_recovery::nested_after_direct)
    .unwrap_or_default();
  let nested_source = main
    .style_definitions_part(package)
    .and_then(|part| part.data(package))
    .map(style_run_recovery::nested_after_direct)
    .unwrap_or_default();
  Some(update(
    source,
    &template_styles,
    nested_source,
    &nested_template,
  ))
}

fn update(
  source: Option<&w::Styles>,
  template: &w::Styles,
  nested_source: HashSet<usize>,
  nested_template: &HashSet<usize>,
) -> UpdatedStyles {
  let mut styles = source.cloned().unwrap_or_default();
  let mut used_ids: HashSet<String> = styles
    .style
    .iter()
    .filter_map(|style| style.style_id.as_deref().map(str::to_owned))
    .collect();
  let mut ids = HashMap::<&str, String>::new();
  let mut destinations = Vec::new();
  for (index, style) in template.style.iter().enumerate() {
    let Some(id) = style.style_id.as_deref() else {
      continue;
    };
    let destination = style.style_name.as_ref().and_then(|name| {
      styles.style.iter().position(|candidate| {
        imported_style_definition_type(candidate) == imported_style_definition_type(style)
          && candidate
            .style_name
            .as_ref()
            .is_some_and(|candidate| candidate.val.eq_ignore_ascii_case(&name.val))
      })
    });
    let (destination, destination_id) = if let Some(destination) = destination {
      let destination_id = styles.style[destination]
        .style_id
        .as_deref()
        .unwrap_or(id)
        .to_owned();
      (destination, destination_id)
    } else {
      let mut destination_id = id.to_owned();
      let mut suffix = 1usize;
      while used_ids.contains(&destination_id) {
        destination_id = format!("{id}_{suffix}");
        suffix += 1;
      }
      let destination = styles.style.len();
      styles.style.push(w::Style::default());
      (destination, destination_id)
    };
    used_ids.insert(destination_id.clone());
    ids.insert(id, destination_id);
    destinations.push((index, destination));
  }
  let mut nested_last = nested_source;
  let mut root_paragraphs = HashSet::new();
  for (index, destination) in destinations {
    let original = &template.style[index];
    let mut style = original.clone();
    let destination_style = &styles.style[destination];
    // The document's default role belongs to its existing style identity.
    // A copied nonmatching template default must not replace another default.
    style.default = destination_style.default.or_else(|| {
      (!styles.style.iter().any(|existing| {
        imported_style_definition_type(existing) == imported_style_definition_type(original)
          && existing.default.is_some_and(|value| value.as_bool())
      }))
      .then_some(original.default)
      .flatten()
    });
    style.style_id = original
      .style_id
      .as_deref()
      .and_then(|id| ids.get(id))
      .cloned();
    if let Some(reference) = &mut style.based_on
      && let Some(id) = ids.get(reference.val.as_str())
    {
      reference.val.clone_from(id);
    }
    if let Some(reference) = &mut style.next_paragraph_style
      && let Some(id) = ids.get(reference.val.as_str())
    {
      reference.val.clone_from(id);
    }
    if let Some(reference) = &mut style.linked_style
      && let Some(id) = ids.get(reference.val.as_str())
    {
      reference.val.clone_from(id);
    }
    // Native Word materializes template docDefaults on every copied root
    // paragraph style, rather than replacing the document's own docDefaults.
    if matches!(
      imported_style_definition_type(&style),
      w::StyleValues::Paragraph
    ) && style.based_on.is_none()
    {
      root_paragraphs.insert(destination);
    }
    nested_last.remove(&destination);
    if nested_template.contains(&index) {
      nested_last.insert(destination);
    }
    styles.style[destination] = style;
  }
  UpdatedStyles {
    styles,
    nested_last,
    root_paragraphs,
    defaults: template.doc_defaults.clone(),
  }
}

pub(super) fn import_root_defaults(
  catalog: &StylesCatalog,
  entry: &mut StyleEntry,
  defaults: &w::DocDefaults,
) {
  merge_paragraph_format_with_theme(
    &mut entry.paragraph_format,
    defaults
      .paragraph_properties_default
      .as_deref()
      .and_then(|default| default.paragraph_properties_base_style.as_deref())
      .map(ParagraphProps::BaseStyle),
    catalog.import_settings,
    &catalog.theme_colors,
  );
  if let Some(run) = defaults
    .run_properties_default
    .as_deref()
    .and_then(|default| default.run_properties_base_style.as_deref())
  {
    properties::merge_doc_default_run_style(
      &mut entry.run_style,
      Some(run),
      &catalog.theme_fonts,
      &catalog.theme_colors,
      catalog.import_settings.compatibility_mode,
    );
    let mut effective = run.clone();
    effective.position = None;
    entry.run_overrides = merge_run_style_overrides(
      entry.run_overrides,
      run_style_overrides(Some(RunProps::BaseStyle(&effective))),
    );
    catalog.apply_word_font_table_mappings(&mut entry.run_style, run.run_fonts.as_ref());
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::parts::{
    document_settings_part::DocumentSettingsPart, style_definitions_part::StyleDefinitionsPart,
  };
  use ooxmlsdk::sdk::SdkType;
  use ooxmlsdk::sdk::WordprocessingDocumentType;

  const WORD: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";

  fn styles(body: &str) -> w::Styles {
    w::Styles::from_bytes(format!(r#"<w:styles xmlns:w="{WORD}">{body}</w:styles>"#).as_bytes())
      .unwrap()
  }

  #[test]
  fn linked_template_styles_match_names_and_remap_package_local_references() {
    let source = styles(
      r#"<w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style><w:style w:type="paragraph" w:styleId="Local"><w:name w:val="Shared"/></w:style><w:style w:type="paragraph" w:styleId="Collision"><w:name w:val="Document Only"/></w:style>"#,
    );
    let template = styles(
      r#"<w:style w:type="paragraph" w:default="1" w:styleId="a"><w:name w:val="Normal"/></w:style><w:style w:type="paragraph" w:styleId="b"><w:name w:val="Shared"/><w:basedOn w:val="a"/><w:next w:val="Collision"/><w:link w:val="c"/><w:rPr><w:b/></w:rPr></w:style><w:style w:type="character" w:styleId="c"><w:name w:val="Shared Char"/></w:style><w:style w:type="paragraph" w:styleId="Collision"><w:name w:val="Template Only"/><w:basedOn w:val="b"/></w:style>"#,
    );
    let result = update(
      Some(&source),
      &template,
      HashSet::from([1]),
      &HashSet::from([0]),
    );
    assert_eq!(result.styles.style.len(), 5);
    assert_eq!(result.styles.style[0].style_id.as_deref(), Some("Normal"));
    assert!(result.styles.style[0].default.unwrap().as_bool());
    let shared = &result.styles.style[1];
    assert_eq!(shared.style_id.as_deref(), Some("Local"));
    assert_eq!(shared.based_on.as_ref().unwrap().val, "Normal");
    assert_eq!(
      shared.next_paragraph_style.as_ref().unwrap().val,
      "Collision_1"
    );
    assert_eq!(shared.linked_style.as_ref().unwrap().val, "c");
    assert_eq!(result.styles.style[2], source.style[2]);
    assert_eq!(
      result.styles.style[4].based_on.as_ref().unwrap().val,
      "Local"
    );
    assert_eq!(result.nested_last, HashSet::from([0]));
    assert_eq!(result.root_paragraphs, HashSet::from([0]));
  }

  #[test]
  fn linked_template_requires_enabled_setting_and_a_valid_resolved_package() {
    let mut template = WordprocessingDocument::create(WordprocessingDocumentType::Template);
    let template_main = template.add_main_document_part().unwrap();
    let template_part = template_main
      .add_new_part_auto_id::<_, StyleDefinitionsPart>(&mut template)
      .unwrap();
    template_part.set_data(&mut template, format!(r#"<w:styles xmlns:w="{WORD}"><w:style w:type="paragraph" w:default="1" w:styleId="a"><w:name w:val="Normal"/></w:style></w:styles>"#).into_bytes()).unwrap();
    let bytes = template.to_package_bytes().unwrap();
    for (setting, expected) in [
      ("", false),
      ("<w:linkStyles w:val=\"0\"/>", false),
      ("<w:linkStyles/>", true),
      ("<w:linkStyles w:val=\"true\"/>", true),
    ] {
      let mut document = WordprocessingDocument::create(WordprocessingDocumentType::Document);
      let main = document.add_main_document_part().unwrap();
      let settings_part = main
        .add_new_part_auto_id::<_, DocumentSettingsPart>(&mut document)
        .unwrap();
      settings_part
        .set_data(
          &mut document,
          format!(r#"<w:settings xmlns:w="{WORD}">{setting}</w:settings>"#).into_bytes(),
        )
        .unwrap();
      assert_eq!(
        load(&document, &main, None, Some(&bytes)).is_some(),
        expected
      );
      assert!(load(&document, &main, None, None).is_none());
      assert!(load(&document, &main, None, Some(b"invalid template")).is_none());
    }
  }

  #[test]
  fn linked_template_materializes_root_defaults_and_preserves_direct_formatting() {
    use super::super::{Block, ImportSettings, InlineItem, ParagraphAlignment, extract};
    use crate::localization::OfficeLocaleContext;
    use crate::options::LayoutOptions;

    fn package(body: &str, style_xml: &str, template: bool) -> WordprocessingDocument {
      let mut package = WordprocessingDocument::create(if template {
        WordprocessingDocumentType::Template
      } else {
        WordprocessingDocumentType::Document
      });
      let main = package.add_main_document_part().unwrap();
      main
        .set_data(
          &mut package,
          format!(r#"<w:document xmlns:w="{WORD}"><w:body>{body}</w:body></w:document>"#)
            .into_bytes(),
        )
        .unwrap();
      let styles_part = main
        .add_new_part_auto_id::<_, StyleDefinitionsPart>(&mut package)
        .unwrap();
      styles_part
        .set_data(
          &mut package,
          format!(r#"<w:styles xmlns:w="{WORD}">{style_xml}</w:styles>"#).into_bytes(),
        )
        .unwrap();
      let settings_part = main
        .add_new_part_auto_id::<_, DocumentSettingsPart>(&mut package)
        .unwrap();
      settings_part
        .set_data(
          &mut package,
          format!(r#"<w:settings xmlns:w="{WORD}"><w:linkStyles/></w:settings>"#).into_bytes(),
        )
        .unwrap();
      package
    }
    let source = package(
      r#"<w:p><w:r><w:t>Inherited template</w:t></w:r></w:p><w:p><w:pPr><w:jc w:val="right"/></w:pPr><w:r><w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/><w:sz w:val="28"/><w:lang w:val="de-DE"/></w:rPr><w:t>Direct formatting</w:t></w:r></w:p>"#,
      r#"<w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="Times New Roman"/><w:lang w:val="de-DE"/></w:rPr></w:rPrDefault></w:docDefaults><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/><w:pPr><w:jc w:val="both"/></w:pPr><w:rPr><w:rFonts w:ascii="Arial"/><w:sz w:val="22"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="SourceOnly"><w:name w:val="Source Only"/><w:basedOn w:val="Normal"/><w:rPr><w:u w:val="single"/></w:rPr></w:style>"#,
      false,
    );
    let template = package(
      "<w:p/>",
      r#"<w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/><w:kern w:val="16"/><w:sz w:val="22"/><w:lang w:val="en-US"/></w:rPr></w:rPrDefault><w:pPrDefault><w:pPr><w:spacing w:after="160" w:line="278" w:lineRule="auto"/></w:pPr></w:pPrDefault></w:docDefaults><w:style w:type="paragraph" w:default="1" w:styleId="a"><w:name w:val="Normal"/></w:style><w:style w:type="paragraph" w:customStyle="1" w:styleId="Root"><w:name w:val="Independent Root"/><w:rPr><w:b/></w:rPr></w:style>"#,
      true,
    );
    let bytes: std::sync::Arc<[u8]> = template.to_package_bytes().unwrap().into();
    let main = source.main_document_part().unwrap();
    let catalog = StylesCatalog::load(
      &source,
      &main,
      ImportSettings::default(),
      &OfficeLocaleContext::default(),
      Some(&bytes),
    )
    .unwrap();
    assert_eq!(
      catalog.doc_default_run.font_family.as_deref(),
      Some("Times New Roman")
    );
    assert_eq!(catalog.doc_default_run.language.as_deref(), Some("de-DE"));
    for id in ["Normal", "Root", "SourceOnly"] {
      let style = catalog.run_style_with_base(Some(id), Default::default(), Default::default());
      assert_eq!(style.font_family.as_deref(), Some("Calibri"), "{id}");
      assert_eq!(style.font_size_pt, 11.0, "{id}");
      assert_eq!(style.language.as_deref(), Some("en-US"), "{id}");
      assert_eq!(style.kerning_minimum_size_pt, Some(8.0), "{id}");
      assert_eq!(style.bold, id == "Root");
      assert_eq!(style.underline, id == "SourceOnly");
    }
    let document = extract(
      &source,
      &LayoutOptions {
        linked_styles_template: Some(bytes),
        ..Default::default()
      },
    )
    .unwrap();
    for (index, (family, size, language, alignment)) in [
      ("Calibri", 11.0, "en-US", ParagraphAlignment::Left),
      ("Arial", 14.0, "de-DE", ParagraphAlignment::Right),
    ]
    .into_iter()
    .enumerate()
    {
      let Block::Paragraph(paragraph) = &document.sections[0].blocks[index] else {
        panic!("paragraph")
      };
      let InlineItem::Text(run) = &paragraph.inlines[0] else {
        panic!("text run")
      };
      assert_eq!(run.style.font_family.as_deref(), Some(family));
      assert_eq!(run.style.font_size_pt, size);
      assert_eq!(run.style.language.as_deref(), Some(language));
      assert_eq!(paragraph.format.alignment, alignment);
    }
    let unchanged = StylesCatalog::load(
      &source,
      &main,
      ImportSettings::default(),
      &OfficeLocaleContext::default(),
      None,
    )
    .unwrap();
    assert_eq!(
      unchanged
        .run_style_with_base(None, Default::default(), Default::default())
        .font_family
        .as_deref(),
      Some("Arial")
    );
  }
}
