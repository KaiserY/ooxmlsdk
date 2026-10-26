use super::*;

// Embedded packages cannot refer to themselves, but each ZIP can contain a
// further package. Bound the recursive importer independently of XML nesting.
const MAX_PACKAGE_DEPTH: usize = 32;

pub(super) struct ImportedContent {
  pub(super) sections: Vec<ImportedSection>,
  pub(super) trailing_blocks: Vec<Block>,
}

pub(super) fn import(
  resource: &AltChunkResource,
  chunk: &w::AltChunk,
  host: &StylesCatalog,
  form_widget_ids: &mut FormWidgetIdAllocator,
  no_column_balance: bool,
) -> Option<ImportedContent> {
  let content_type = resource.content_type.as_deref()?.split(';').next()?.trim();
  // [MS-OI29500] 2.1.527 lists these main-part MIME types for Word's
  // alternative-format *packages*, rather than the standalone DOCX MIME type.
  if ![
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
    "application/vnd.ms-word.document.macroEnabled.main+xml",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml",
    "application/vnd.ms-word.template.macroEnabledTemplate.main+xml",
  ]
  .iter()
  .any(|mime| content_type.eq_ignore_ascii_case(mime))
    || resource.wordprocessing_depth >= MAX_PACKAGE_DEPTH
  {
    return None;
  }
  let package = WordprocessingDocument::new(std::io::Cursor::new(resource.data.as_ref())).ok()?;
  let main = package.main_document_part().ok()?;
  let document = drawing_position_recovery::root(&main.try_data_bytes(&package).ok()?, || {
    main.root_element(&package)
  })
  .ok()?;
  let mut body = document.body.as_deref()?.clone();
  let source = StylesCatalog::load(&package, &main, host.import_settings, &host.locales).ok()?;
  let keep_source = chunk
    .alt_chunk_properties
    .as_deref()
    .and_then(|properties| properties.match_source.as_ref())
    .is_some_and(|value| value.val.is_none_or(|value| value.as_bool()));
  let mut styles = merged_styles(host, source, keep_source);
  styles.preserve_word_text_whitespace =
    document.space == Some(xml::SpaceProcessingModeValues::Preserve);
  let mut numbering =
    NumberingCatalog::load(&package, &main, host.import_settings, &styles).ok()?;
  styles.numbering_template = Some(numbering.fresh_for_story());
  let mut images = ImageCatalog::load(&package, &main);
  images.vml_shape_types = vml_document_shape_types(&body);
  let alt_chunks =
    AltChunkCatalog::load_at_depth(&package, &main, resource.wordprocessing_depth + 1);
  let hyperlinks = HyperlinkCatalog::load(&package, &main);
  let custom_xml_bindings = CustomXmlBindings::load(&package, &main);

  // Word inserts a fragment without its terminal section mark. The first
  // internal mark creates a fresh application-default section; subsequent
  // internal marks retain their source properties. In particular, importing
  // the source final sectPr would overwrite the destination's trailing page
  // setup. Native WordOpenXML controls pin this independently of matchSrc,
  // source dimensions/type and destination A4/Letter geometry.
  reset_first_section(&mut body, &host.locales);
  // An internal section mark ends its paragraph, but cannot be the final
  // paragraph mark of the fragment's following section. Word supplies that
  // empty final paragraph even when the source XML only has the terminal
  // sectPr after the marked paragraph. Retain it at the insertion point;
  // otherwise a source/destination size change loses its final blank page.
  if body.body_choice.last().is_some_and(|choice| {
    matches!(choice, w::BodyChoice::Paragraph(paragraph)
      if paragraph.paragraph_properties.as_deref()
        .is_some_and(|properties| properties.section_properties.is_some()))
  }) {
    body
      .body_choice
      .push(w::BodyChoice::Paragraph(Box::default()));
  }
  // A synthetic terminal owner makes the unclosed fragment explicit even
  // when its final internal mark has no following visible paragraph. Its
  // properties are discarded with the returned trailing frame below.
  body.section_properties = Some(Box::default());
  let mut sections = body_sections(
    &body,
    BodySectionEnv {
      styles: &styles,
      numbering: &mut numbering,
      images: &images,
      alt_chunks: &alt_chunks,
      hyperlinks: &hyperlinks,
      custom_xml_bindings: &custom_xml_bindings,
      form_widget_ids,
      no_column_balance,
      fixed_html_paragraph_auto_spacing: host.import_settings.fixed_html_paragraph_auto_spacing,
    },
  );
  let trailing_blocks = sections.pop()?.blocks;
  Some(ImportedContent {
    sections,
    trailing_blocks,
  })
}

fn merged_styles(host: &StylesCatalog, source: StylesCatalog, keep_source: bool) -> StylesCatalog {
  // ECMA-376 17.17.2.3: absent/false matchSrc uses destination definitions
  // for collisions; true preserves the source formatting within the import.
  // Resolve in a fragment-local catalog so colliding IDs and relationships
  // cannot modify any preceding/following destination content.
  let mut result = host.clone();
  if keep_source {
    result.doc_default_paragraph = source.doc_default_paragraph;
    result.doc_default_run = source.doc_default_run;
    result.default_paragraph_style_id = source.default_paragraph_style_id;
    result.default_table_style_id = source.default_table_style_id;
    result.theme_fonts = source.theme_fonts;
    result.theme_colors = source.theme_colors;
    result.theme_fills = source.theme_fills;
    result.theme_lines = source.theme_lines;
    result.theme_effects = source.theme_effects;
  }
  for (id, entry) in source.styles {
    if keep_source {
      result.styles.insert(id, entry);
    } else {
      result.styles.entry(id).or_insert(entry);
    }
  }
  result
}

fn reset_first_section(body: &mut w::Body, locales: &OfficeLocaleContext) {
  for choice in &mut body.body_choice {
    if let w::BodyChoice::Paragraph(paragraph) = choice
      && let Some(properties) = paragraph.paragraph_properties.as_deref_mut()
      && let Some(section) = properties.section_properties.as_deref_mut()
    {
      *section = imported_first_section(locales);
      break;
    }
  }
}

fn imported_first_section(locales: &OfficeLocaleContext) -> w::SectionProperties {
  let chinese = locales
    .default_document_resource_locale()
    .is_simplified_chinese();
  w::SectionProperties {
    page_size: Some(w::PageSize {
      width: Some(TwipsMeasureValue::Twips(11906)),
      height: Some(TwipsMeasureValue::Twips(16838)),
      ..Default::default()
    }),
    page_margin: Some(w::PageMargin {
      top: Some(SignedTwipsMeasureValue::Twips(1440)),
      right: Some(TwipsMeasureValue::Twips(1800)),
      bottom: Some(SignedTwipsMeasureValue::Twips(1440)),
      left: Some(TwipsMeasureValue::Twips(1800)),
      header: Some(TwipsMeasureValue::Twips(if chinese { 851 } else { 720 })),
      footer: Some(TwipsMeasureValue::Twips(if chinese { 992 } else { 720 })),
      gutter: Some(TwipsMeasureValue::Twips(0)),
    }),
    columns: Some(w::Columns {
      space: Some(TwipsMeasureValue::Twips(if chinese { 425 } else { 720 })),
      ..Default::default()
    }),
    doc_grid: chinese.then_some(w::DocGrid {
      r#type: Some(w::DocGridValues::Lines),
      line_pitch: Some(312),
      ..Default::default()
    }),
    ..Default::default()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use ooxmlsdk::parts::style_definitions_part::StyleDefinitionsPart;
  use ooxmlsdk::sdk::{AlternativeFormatImportPartType, WordprocessingDocumentType};

  const W: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
  const R: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
  const HOST_SECTION: &str = r#"<w:sectPr><w:type w:val="continuous"/><w:pgSz w:w="11906" w:h="16838"/><w:pgMar w:top="1417" w:left="1417" w:bottom="1417" w:right="1417"/></w:sectPr>"#;
  const INNER_SECTION: &str = r#"<w:sectPr><w:pgSz w:w="5669" w:h="5669"/><w:pgMar w:top="400" w:left="400" w:bottom="400" w:right="400"/></w:sectPr>"#;
  const HOST_STYLES: &str = r#"<w:docDefaults><w:rPrDefault><w:rPr><w:sz w:val="22"/></w:rPr></w:rPrDefault><w:pPrDefault><w:pPr/></w:pPrDefault></w:docDefaults><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style><w:style w:type="paragraph" w:styleId="Shared"><w:name w:val="Shared"/><w:rPr><w:color w:val="1122CC"/><w:sz w:val="28"/></w:rPr></w:style>"#;
  const CHILD_STYLES: &str = r#"<w:docDefaults><w:rPrDefault><w:rPr><w:sz w:val="22"/></w:rPr></w:rPrDefault><w:pPrDefault><w:pPr/></w:pPrDefault></w:docDefaults><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/><w:rPr><w:color w:val="8844CC"/><w:sz w:val="34"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="Shared"><w:name w:val="Shared"/><w:rPr><w:color w:val="CC2211"/><w:sz w:val="40"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="OnlyInner"><w:name w:val="OnlyInner"/><w:basedOn w:val="Normal"/><w:rPr><w:color w:val="008844"/><w:sz w:val="38"/></w:rPr></w:style>"#;

  fn package(body: &str, styles: &str) -> WordprocessingDocument {
    let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
    let main = package.add_main_document_part().unwrap();
    main
      .set_data(
        &mut package,
        format!(r#"<w:document xmlns:w="{W}" xmlns:r="{R}"><w:body>{body}</w:body></w:document>"#)
          .into_bytes(),
      )
      .unwrap();
    main
      .add_new_part_auto_id::<_, StyleDefinitionsPart>(&mut package)
      .unwrap()
      .set_data(
        &mut package,
        format!(r#"<w:styles xmlns:w="{W}">{styles}</w:styles>"#).into_bytes(),
      )
      .unwrap();
    package
  }

  fn add_chunk(host: &mut WordprocessingDocument, child: &WordprocessingDocument) {
    host
      .main_document_part()
      .unwrap()
      .add_alternative_format_import_part_by_type_with_id(
        host,
        AlternativeFormatImportPartType::WordprocessingMl,
        "chunk",
      )
      .unwrap()
      .set_data(host, child.to_package_bytes().unwrap())
      .unwrap();
  }

  fn imported(host: &WordprocessingDocument) -> DocxDocument {
    extract(
      host,
      &LayoutOptions {
        ui_language: Some("ko-KR".to_string()),
        default_document_language: Some("zh-CN".to_string()),
        ..Default::default()
      },
    )
    .unwrap()
  }

  fn texts(blocks: &[Block]) -> Vec<String> {
    blocks
      .iter()
      .filter_map(|block| match block {
        Block::Paragraph(p) => Some(
          p.inlines
            .iter()
            .filter_map(|inline| match inline {
              InlineItem::Text(run) => Some(run.text.as_str()),
              _ => None,
            })
            .collect(),
        ),
        _ => None,
      })
      .collect()
  }

  fn runs(document: &DocxDocument) -> Vec<&TextRun> {
    document
      .sections
      .iter()
      .flat_map(|section| &section.blocks)
      .filter_map(|b| match b {
        Block::Paragraph(p) => Some(p),
        _ => None,
      })
      .flat_map(|p| &p.inlines)
      .filter_map(|i| match i {
        InlineItem::Text(run) => Some(run),
        _ => None,
      })
      .collect()
  }

  #[test]
  fn embedded_docx_splices_sections_into_the_open_destination_section() {
    let child = package(
      &format!(
        "<w:p><w:r><w:t>inner first</w:t></w:r></w:p><w:p><w:pPr>{INNER_SECTION}</w:pPr></w:p><w:p><w:r><w:t>inner last</w:t></w:r></w:p>{INNER_SECTION}"
      ),
      HOST_STYLES,
    );
    let mut host = package(
      &format!(
        "<w:p><w:pPr>{INNER_SECTION}</w:pPr><w:r><w:t>before</w:t></w:r></w:p><w:p><w:r><w:t>after</w:t></w:r></w:p><w:altChunk r:id=\"chunk\"/>{HOST_SECTION}"
      ),
      HOST_STYLES,
    );
    add_chunk(&mut host, &child);
    let doc = imported(&host);
    assert_eq!(doc.sections.len(), 3);
    assert_eq!(texts(&doc.sections[0].blocks), ["before"]);
    assert_eq!(texts(&doc.sections[1].blocks), ["after", "inner first"]);
    assert_eq!(texts(&doc.sections[2].blocks), ["inner last"]);
    assert_eq!(doc.sections[1].break_kind, SectionBreakKind::NextPage);
    assert_eq!(doc.sections[1].page.width_pt, 11906.0 / 20.0);
    assert_eq!(doc.sections[1].page.margin_left_pt, 90.0);
    assert_eq!(doc.sections[1].page.doc_grid_line_pitch_pt, Some(15.6));
    assert_eq!(doc.sections[2].page.margin_left_pt, 1417.0 / 20.0);
    assert_eq!(doc.sections[2].break_kind, SectionBreakKind::Continuous);
  }

  #[test]
  fn sectionless_chunk_keeps_the_destination_flow_and_its_empty_paragraphs() {
    let child = package(
      &format!("<w:p><w:r><w:t>inner</w:t></w:r></w:p><w:p/>{INNER_SECTION}"),
      HOST_STYLES,
    );
    let mut host = package(
      &format!(
        "<w:p><w:r><w:t>outer</w:t></w:r></w:p><w:altChunk r:id=\"chunk\"/><w:p><w:r><w:t>following</w:t></w:r></w:p>{HOST_SECTION}"
      ),
      HOST_STYLES,
    );
    add_chunk(&mut host, &child);
    let doc = imported(&host);
    assert_eq!(doc.sections.len(), 1);
    assert_eq!(
      texts(&doc.sections[0].blocks),
      ["outer", "inner", "", "following"]
    );
    assert_eq!(doc.sections[0].page.margin_left_pt, 1417.0 / 20.0);
  }

  #[test]
  fn only_first_internal_section_is_reset_and_an_empty_tail_stays_explicit() {
    let later = r#"<w:sectPr><w:type w:val="continuous"/><w:pgSz w:w="8000" w:h="12000"/><w:pgMar w:left="600" w:right="600" w:top="600" w:bottom="600"/></w:sectPr>"#;
    let child = package(
      &format!(
        "<w:p><w:pPr>{INNER_SECTION}</w:pPr><w:r><w:t>first</w:t></w:r></w:p><w:p><w:pPr>{later}</w:pPr><w:r><w:t>second</w:t></w:r></w:p>{INNER_SECTION}"
      ),
      HOST_STYLES,
    );
    let mut host = package(
      &format!("<w:altChunk r:id=\"chunk\"/>{HOST_SECTION}"),
      HOST_STYLES,
    );
    add_chunk(&mut host, &child);
    let doc = imported(&host);
    assert_eq!(doc.sections.len(), 3);
    assert_eq!(doc.sections[0].page.width_pt, 11906.0 / 20.0);
    assert_eq!(doc.sections[1].page.width_pt, 400.0);
    assert_eq!(doc.sections[1].page.margin_left_pt, 30.0);
    assert_eq!(texts(&doc.sections[1].blocks), ["second"]);
    assert_eq!(texts(&doc.sections[2].blocks), [""]);
    assert_eq!(doc.sections[2].page.width_pt, 11906.0 / 20.0);
  }

  #[test]
  fn match_source_resolves_style_collisions_without_changing_the_host() {
    let child = package(
      &format!(
        "<w:p><w:pPr><w:pStyle w:val=\"Shared\"/></w:pPr><w:r><w:t>shared</w:t></w:r></w:p><w:p><w:pPr><w:pStyle w:val=\"OnlyInner\"/></w:pPr><w:r><w:t>new</w:t></w:r></w:p><w:p><w:r><w:t>normal</w:t></w:r></w:p>{INNER_SECTION}"
      ),
      CHILD_STYLES,
    );
    for (property, keep) in [
      ("", false),
      (
        r#"<w:altChunkPr><w:matchSrc w:val="false"/></w:altChunkPr>"#,
        false,
      ),
      (r#"<w:altChunkPr><w:matchSrc/></w:altChunkPr>"#, true),
    ] {
      let mut host = package(
        &format!(
          "<w:p><w:pPr><w:pStyle w:val=\"Shared\"/></w:pPr><w:r><w:t>outer before</w:t></w:r></w:p><w:altChunk r:id=\"chunk\">{property}</w:altChunk><w:p><w:pPr><w:pStyle w:val=\"Shared\"/></w:pPr><w:r><w:t>outer after</w:t></w:r></w:p>{HOST_SECTION}"
        ),
        HOST_STYLES,
      );
      add_chunk(&mut host, &child);
      let doc = imported(&host);
      let r = runs(&doc);
      assert_eq!(
        r.iter().map(|r| r.text.as_str()).collect::<Vec<_>>(),
        ["outer before", "shared", "new", "normal", "outer after"]
      );
      assert_eq!(r[0].style.font_size_pt, 14.0);
      assert_eq!(r[4].style.font_size_pt, 14.0);
      assert_eq!(r[1].style.font_size_pt, if keep { 20.0 } else { 14.0 });
      assert_eq!(
        r[1].style.color,
        if keep {
          RgbColor {
            r: 204,
            g: 34,
            b: 17,
          }
        } else {
          RgbColor {
            r: 17,
            g: 34,
            b: 204,
          }
        }
      );
      assert_eq!(r[2].style.font_size_pt, 19.0);
      assert_eq!(
        r[2].style.color,
        RgbColor {
          r: 0,
          g: 136,
          b: 68
        }
      );
      assert_eq!(r[3].style.font_size_pt, if keep { 17.0 } else { 11.0 });
    }
  }

  #[test]
  fn nested_docx_imports_use_their_own_relationship_scope() {
    let inner = package("<w:p><w:r><w:t>deep</w:t></w:r></w:p>", HOST_STYLES);
    let mut middle = package(
      "<w:p><w:r><w:t>middle</w:t></w:r></w:p><w:altChunk r:id=\"chunk\"/>",
      HOST_STYLES,
    );
    add_chunk(&mut middle, &inner);
    let mut host = package(
      &format!("<w:p><w:r><w:t>host</w:t></w:r></w:p><w:altChunk r:id=\"chunk\"/>{HOST_SECTION}"),
      HOST_STYLES,
    );
    add_chunk(&mut host, &middle);
    let doc = imported(&host);
    assert_eq!(doc.sections.len(), 1);
    assert_eq!(texts(&doc.sections[0].blocks), ["host", "middle", "deep"]);
  }

  #[test]
  fn unsupported_or_invalid_packages_leave_the_existing_alt_chunk_policy_intact() {
    for (content_type, data) in [
      ("application/rtf", &b"ignored"[..]),
      (
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
        &b"not a ZIP"[..],
      ),
    ] {
      let resource = AltChunkResource {
        data: Bytes::copy_from_slice(data),
        content_type: Some(content_type.to_string()),
        wordprocessing_depth: 0,
      };
      assert!(
        import(
          &resource,
          &w::AltChunk::default(),
          &StylesCatalog::default(),
          &mut FormWidgetIdAllocator::default(),
          false
        )
        .is_none()
      );
    }
  }
}
