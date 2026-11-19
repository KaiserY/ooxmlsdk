//! Native Word field controls: /tmp/ooxmlsdk-tdf164620-20261109.
use super::*;

#[test]
fn nested_entry_field_scopes_keep_only_instruction_sequence_effects() {
  use ooxmlsdk::parts::document_settings_part::DocumentSettingsPart;
  use ooxmlsdk::sdk::WordprocessingDocumentType;
  // Native Word72 TC/XE/TA controls in each code/result/locked configuration,
  // including fldSimple caches written as either w:t or w:instrText.
  // Evidence: /tmp/ooxmlsdk-complex0-4c6c3e0e-20261115/native-field-scope-v8.
  for mode in [12, 15] {
    for kind in ["TC", "XE", "TA"] {
      for child in ["text", "instruction", "complex"] {
        for in_result in [false, true] {
          for locked in [false, true] {
            let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
            let main = package.add_main_document_part().unwrap();
            main.add_new_part_auto_id::<_, DocumentSettingsPart>(&mut package).unwrap().set_data(&mut package, format!(r#"<w:settings xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:compat><w:compatSetting w:name="compatibilityMode" w:uri="http://schemas.microsoft.com/office/word" w:val="{mode}"/></w:compat></w:settings>"#).into_bytes()).unwrap();
            let child = match child {
              "complex" => {
                "<w:r><w:fldChar w:fldCharType=\"begin\"/></w:r><w:r><w:instrText>SEQ sample</w:instrText></w:r><w:r><w:fldChar w:fldCharType=\"separate\"/></w:r><w:r><w:t>37</w:t></w:r><w:r><w:fldChar w:fldCharType=\"end\"/></w:r>"
              }
              "instruction" => {
                "<w:fldSimple w:instr=\"SEQ sample\"><w:r><w:instrText>37</w:instrText></w:r></w:fldSimple>"
              }
              _ => "<w:fldSimple w:instr=\"SEQ sample\"><w:r><w:t>37</w:t></w:r></w:fldSimple>",
            };
            let separator = if in_result {
              "<w:r><w:fldChar w:fldCharType=\"separate\"/></w:r><w:r><w:t>OUTER_CACHE</w:t></w:r>"
            } else {
              ""
            };
            main.set_data(&mut package, format!(r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>BEGIN|</w:t></w:r><w:r><w:fldChar w:fldCharType="begin" w:fldLock="{}"/></w:r><w:r><w:instrText>{kind} "entry"</w:instrText></w:r>{separator}{child}<w:r><w:fldChar w:fldCharType="end"/></w:r><w:r><w:t>|NEXT:</w:t></w:r><w:fldSimple w:instr="SEQ sample"><w:r><w:t>777</w:t></w:r></w:fldSimple><w:r><w:t>|END</w:t></w:r></w:p></w:body></w:document>"#, u8::from(locked)).into_bytes()).unwrap();
            let options = LayoutOptions {
              update_fields_at_print: Some(true),
              ..Default::default()
            };
            let document = extract(&package, &options).unwrap();
            let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
              panic!("paragraph");
            };
            assert_eq!(
              paragraph
                .inlines
                .iter()
                .filter(|item| matches!(item, InlineItem::SequenceCounter(_)))
                .count(),
              usize::from(!in_result)
            );
            let placed = layout::layout(&document, &options).unwrap();
            let text = placed
              .pages
              .iter()
              .flat_map(|page| &page.items)
              .filter_map(|item| {
                if let layout::PageItem::Text(run) = item {
                  Some(run.text.as_str())
                } else {
                  None
                }
              })
              .collect::<String>();
            let value = if in_result { 1 } else { 2 };
            assert_eq!(
              text,
              format!("BEGIN||NEXT:{value}|END"),
              "mode{mode} {kind}, result={in_result}, locked={locked}"
            );
          }
        }
      }
    }
  }
}

#[test]
fn section_pages_keep_both_body_owners_on_a_shared_physical_page() {
  use ooxmlsdk::sdk::WordprocessingDocumentType;
  let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
  let main = package.add_main_document_part().unwrap();
  main.set_data(&mut package,br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:pPr><w:sectPr><w:pgSz w:w="12240" w:h="15840"/><w:pgMar w:top="1440" w:bottom="1440" w:left="1440" w:right="1440"/></w:sectPr></w:pPr><w:fldSimple w:instr="SECTIONPAGES"><w:r><w:t>99</w:t></w:r></w:fldSimple></w:p><w:p><w:fldSimple w:instr="SECTIONPAGES"><w:r><w:t>99</w:t></w:r></w:fldSimple></w:p><w:p><w:r><w:br w:type="page"/></w:r></w:p><w:p><w:fldSimple w:instr="SECTIONPAGES"><w:r><w:t>99</w:t></w:r></w:fldSimple></w:p><w:sectPr><w:type w:val="continuous"/><w:pgSz w:w="12240" w:h="15840"/><w:pgMar w:top="1440" w:bottom="1440" w:left="1440" w:right="1440"/></w:sectPr></w:body></w:document>"#.to_vec()).unwrap();
  let options = LayoutOptions {
    update_fields_at_print: Some(true),
    ..Default::default()
  };
  let document = extract(&package, &options).unwrap();
  assert_eq!(document.sections.len(), 2);
  let placed = layout::layout(&document, &options).unwrap();
  assert_eq!(placed.pages.len(), 2);
  let fields = placed
    .pages
    .iter()
    .flat_map(|page| &page.items)
    .filter_map(|item| match item {
      layout::PageItem::Text(text)
        if matches!(
          text.dynamic_field,
          Some(DynamicFieldKind::SectionPages { .. })
        ) =>
      {
        Some(text.text.as_str())
      }
      _ => None,
    })
    .collect::<Vec<_>>();
  assert_eq!(fields, ["1", "2", "2"]);
}

#[test]
fn unsupported_section_formats_keep_their_cache_instead_of_becoming_decimal() {
  for code in [
    r"SECTIONPAGES \* Ordinal \* MERGEFORMAT",
    r"SECTION \* OrdText",
    r#"SECTIONPAGES \# "0.00""#,
  ] {
    assert!(dynamic_field_kind(code).is_none(), "{code}");
  }
  assert!(matches!(
    dynamic_field_kind(r#"SECTIONPAGES \# "0.00" \* Arabic \* MERGEFORMAT"#),
    Some(DynamicFieldKind::SectionPages {
      number_format: FieldNumberFormat::Decimal,
      ..
    })
  ));
}

#[test]
fn quote_recalculation_does_not_flatten_an_existing_paragraph_story() {
  use ooxmlsdk::sdk::WordprocessingDocumentType;
  let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
  let main = package.add_main_document_part().unwrap();
  main.set_data(&mut package,br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText>QUOTE "foo</w:instrText></w:r></w:p><w:p><w:r><w:instrText> bar"</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r></w:p><w:p><w:r><w:t>KEEP</w:t></w:r></w:p><w:p><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p></w:body></w:document>"#.to_vec()).unwrap();
  let options = LayoutOptions {
    update_fields_at_print: Some(true),
    ..Default::default()
  };
  let document = extract(&package, &options).unwrap();
  let text = document.sections[0]
    .blocks
    .iter()
    .filter_map(|block| match block {
      Block::Paragraph(paragraph) => Some(paragraph),
      _ => None,
    })
    .flat_map(|p| &p.inlines)
    .filter_map(|item| match item {
      InlineItem::Text(run) => Some(run.text.as_str()),
      _ => None,
    })
    .collect::<String>();
  assert_eq!(text.matches("KEEP").count(), 1);
  assert!(!text.contains("foo"));
  let styles = StylesCatalog {
    import_settings: ImportSettings {
      update_fields_at_print: Some(true),
      ..Default::default()
    },
    ..Default::default()
  };
  assert!(refreshed_scalar_field(r#"QUOTE "unfinished"#, &styles).is_none());
}

#[test]
fn sequence_missing_bookmark_uses_the_generated_diagnostic_style() {
  use ooxmlsdk::sdk::WordprocessingDocumentType;
  let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
  let main = package.add_main_document_part().unwrap();
  main.set_data(&mut package, br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:instrText>SEQ a Missing extra \* Arabic</w:instrText></w:r><w:r><w:fldChar w:fldCharType="separate"/></w:r><w:r><w:t>CACHED</w:t></w:r><w:r><w:fldChar w:fldCharType="end"/></w:r></w:p></w:body></w:document>"#.to_vec()).unwrap();
  let document = extract(
    &package,
    &LayoutOptions {
      ui_language: Some("es-MX".into()),
      update_fields_at_print: Some(true),
      ..Default::default()
    },
  )
  .unwrap();
  let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
    panic!("paragraph");
  };
  let InlineItem::Text(run) = &paragraph.inlines[0] else {
    panic!("run");
  };
  assert_eq!(run.text, "¡Error! Marcador no definido.");
  assert!(run.style.bold);
  assert!(run.style.wordprocessingml_generated_field_diagnostic);
  assert!(run.dynamic_field.is_none());
}

#[test]
fn opening_clock_is_distinct_from_the_print_clock_and_optional_for_callers() {
  let opened = FieldUpdateDateTime {
    year: 2026,
    month: 9,
    day: 11,
    hour: 20,
    minute: 54,
    second: 56,
  };
  let updated = FieldUpdateDateTime {
    minute: 55,
    second: 57,
    ..opened
  };
  let mut styles = StylesCatalog {
    import_settings: ImportSettings {
      field_update_datetime: Some(updated),
      field_open_datetime: Some(opened),
      ..Default::default()
    },
    ..Default::default()
  };
  let style = TextStyle {
    language: Some(Arc::from("en-US")),
    ..Default::default()
  };
  for kind in ["DATE", "TIME"] {
    assert_eq!(
      refreshed_date_time_field(&format!(r#"{kind} \@ "HH:mm:ss""#), &style, &styles).as_deref(),
      Some("20:54:56")
    );
  }
  for kind in ["PRINTDATE", "SAVEDATE"] {
    assert_eq!(
      refreshed_date_time_field(&format!(r#"{kind} \@ "HH:mm:ss""#), &style, &styles).as_deref(),
      Some("20:55:00")
    );
  }
  styles.import_settings.field_open_datetime = None;
  assert_eq!(
    refreshed_date_time_field(r#"TIME \@ "HH:mm:ss""#, &style, &styles).as_deref(),
    Some("20:55:57")
  );
}

#[test]
fn quoted_arguments_decode_escapes_before_field_interpretation() {
  for (argument, expected) in [
    (r"a\ b", "a b"),
    (r"a\\b", r"a\b"),
    (r"a\b", "ab"),
    (r#"\"x\""#, "\"x\""),
    (r"a\Q", "aQ"),
    (r"\\", "\\"),
    (r"\d", "d"),
    (r"\\d", r"\d"),
    (r"a\:b", "a:b"),
    (r"a\\ b", r"a\ b"),
    (r#"a\\\"b"#, "a\\\"b"),
  ] {
    assert_eq!(
      field_instruction_tokens(&format!(r#"QUOTE "{argument}" \* MERGEFORMAT"#)),
      ["QUOTE", expected, r"\*", "MERGEFORMAT"],
      "{argument}"
    );
  }
  assert_eq!(field_instruction_tokens(r#"REF "" \h"#), ["REF", "", r"\h"]);
  let date = FieldUpdateDateTime {
    year: 2026,
    month: 11,
    day: 9,
    hour: 13,
    minute: 20,
    second: 30,
  };
  for (picture, expected) in [
    (r"dddd\ dd\ MMMM\ yyyy", "Monday 09 November 2026"),
    (r"dd\/MM\/yyyy", "09/11/2026"),
    (r"\d", "9"),
    (r"\\d", "\\9"),
  ] {
    let tokens = field_instruction_tokens(&format!(r#"DATE \@ "{picture}""#));
    assert_eq!(
      field_datetime::format_date_time_field(&tokens, Some("en-US"), date).as_deref(),
      Some(expected),
      "{picture}"
    );
  }
}

#[test]
fn reserved_date_uses_native_display_namespace_despite_custom_flag() {
  for (ui, native) in [
    ("en-US", "Date"),
    ("fr-CA", "Date"),
    ("es-MX", "Fecha"),
    ("de-DE", "Datum"),
    ("zh-CN", "日期"),
    ("zh-TW", "日期"),
    ("ja-JP", "日付"),
    ("ko-KR", "날짜"),
  ] {
    let styles = StylesCatalog {
      has_styles_part: true,
      locales: OfficeLocaleContext::new(Some(ui), None, Some("en-US")),
      styles: HashMap::from([
        (
          "Date".into(),
          StyleEntry {
            custom_style: true,
            name: Some("Date".into()),
            style_type: Some(w::StyleValues::Paragraph),
            ..Default::default()
          },
        ),
        (
          "DateChar".into(),
          StyleEntry {
            custom_style: true,
            name: Some("Date Char".into()),
            style_type: Some(w::StyleValues::Character),
            ..Default::default()
          },
        ),
      ]),
      ..Default::default()
    };
    assert_eq!(styles.resolved_style_ref_name(native), Some(native), "{ui}");
    assert_eq!(
      styles.resolved_style_ref_name("Date"),
      (native == "Date").then_some(native),
      "{ui}"
    );
    assert_eq!(styles.resolved_style_ref_name("Unrelated Missing"), None);
    assert!(styles.style_ref_keys("Date").iter().any(|key| key.as_ref()
      == format!(
        "{}{native}",
        field_localization::NATIVE_STYLE_REF_KEY_PREFIX
      )));
  }
}

#[test]
fn explicit_template_context_preserves_path_and_existing_refresh_gates() {
  let mut styles = StylesCatalog {
    field_template_path: Some(r"C:\Templates\Actual Name.dotm".into()),
    ..Default::default()
  };
  assert_eq!(refreshed_doc_property_field("TEMPLATE", &styles), None);
  styles.import_settings.update_fields_at_print = Some(true);
  assert_eq!(
    refreshed_doc_property_field("TEMPLATE", &styles).as_deref(),
    Some("Actual Name.dotm")
  );
  assert_eq!(
    refreshed_doc_property_field(r"TEMPLATE \p", &styles).as_deref(),
    Some(r"C:\Templates\Actual Name.dotm")
  );
  assert_eq!(
    refreshed_doc_property_field(r"TEMPLATE \* Upper", &styles).as_deref(),
    Some("ACTUAL NAME.DOTM")
  );
  styles.field_template_path = None;
  assert_eq!(refreshed_doc_property_field("TEMPLATE", &styles), None);
  styles.field_template_path = Some("Other.dotx".into());
  styles.import_settings.update_fields_at_print = Some(false);
  assert_eq!(refreshed_doc_property_field("TEMPLATE", &styles), None);
}

#[test]
fn imported_revision_defaults_only_absent_core_property_and_retains_zero() {
  use ooxmlsdk::sdk::WordprocessingDocumentType;
  for (revision, expected) in [
    (None, Some(1)),
    (Some(""), None),
    (Some("0"), Some(0)),
    (Some("19"), Some(19)),
    (Some("INVALID"), None),
  ] {
    let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
    let main = package.add_main_document_part().unwrap();
    main.set_data(&mut package, br#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body/></w:document>"#.to_vec()).unwrap();
    if let Some(revision) = revision {
      let core = package.add_core_file_properties_part().unwrap();
      core.set_data(&mut package, format!(r#"<cp:coreProperties xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"><cp:revision>{revision}</cp:revision></cp:coreProperties>"#).into_bytes()).unwrap();
    }
    let styles = StylesCatalog::load(
      &package,
      &main,
      ImportSettings::default(),
      &OfficeLocaleContext::default(),
      None,
    )
    .unwrap();
    assert_eq!(styles.revision_number, expected, "{revision:?}");
  }
}

#[test]
fn implicit_reference_print_results_distinguish_missing_cache_and_lock_owners() {
  use ooxmlsdk::sdk::WordprocessingDocumentType;
  // PDF-first Word controls: native-implicit-ref-v2/v3 in the complex1_NOR
  // evidence owner. Missing implicit references preserve even a space cache;
  // an empty result gets UndefinedBookmark, independently of print-update off.
  for simple in [false, true] {
    for update in [None, Some(false), Some(true)] {
      for found in [false, true] {
        for cache in [None, Some(""), Some(" "), Some("OLD")] {
          for locked in [false, true] {
            let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
            let main = package.add_main_document_part().unwrap();
            let bookmark = if found {
              r#"<w:bookmarkStart w:id="1" w:name="TargetCase"/><w:r><w:t>FRESH</w:t></w:r><w:bookmarkEnd w:id="1"/>"#
            } else {
              ""
            };
            let result = cache.map_or_else(String::new, |text| {
              format!(r#"<w:r><w:t xml:space="preserve">{text}</w:t></w:r>"#)
            });
            let field = if simple {
              format!(
                r#"<w:fldSimple w:instr="tArGeTcAsE" w:fldLock="{}">{result}</w:fldSimple>"#,
                u8::from(locked)
              )
            } else {
              let separator = if cache.is_some() {
                r#"<w:r><w:fldChar w:fldCharType="separate"/></w:r>"#
              } else {
                ""
              };
              format!(
                r#"<w:r><w:fldChar w:fldCharType="begin" w:fldLock="{}"/></w:r><w:r><w:instrText>tArGeTcAsE</w:instrText></w:r>{separator}{result}<w:r><w:fldChar w:fldCharType="end"/></w:r>"#,
                u8::from(locked)
              )
            };
            main.set_data(&mut package,format!(r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p>{bookmark}</w:p><w:p><w:r><w:t>BEGIN|</w:t></w:r>{field}<w:r><w:t>|END</w:t></w:r></w:p></w:body></w:document>"#).into_bytes()).unwrap();
            let document = extract(
              &package,
              &LayoutOptions {
                ui_language: Some("es-MX".into()),
                update_fields_at_print: update,
                ..Default::default()
              },
            )
            .unwrap();
            let Block::Paragraph(paragraph) = &document.sections[0].blocks[1] else {
              panic!("field paragraph");
            };
            let text = paragraph
              .inlines
              .iter()
              .filter_map(|item| match item {
                InlineItem::Text(run) => Some(run.text.as_str()),
                _ => None,
              })
              .collect::<String>();
            let expected = if update.is_none() || locked {
              cache.unwrap_or("")
            } else if found {
              "FRESH"
            } else {
              cache
                .filter(|value| !value.is_empty())
                .unwrap_or("¡Error! Marcador no definido.")
            };
            assert_eq!(
              text,
              format!("BEGIN|{expected}|END"),
              "simple={simple},update={update:?},found={found},cache={cache:?},locked={locked}"
            );
          }
        }
      }
    }
  }
}

#[test]
fn implicit_reference_error_uses_separator_or_begin_and_toggles_its_weight() {
  use ooxmlsdk::sdk::WordprocessingDocumentType;
  for separator in [false, true] {
    for begin_bold in [false, true] {
      for separator_bold in [false, true] {
        let mut package = WordprocessingDocument::create(WordprocessingDocumentType::Document);
        let main = package.add_main_document_part().unwrap();
        let separator_xml = if separator {
          format!(
            r#"<w:r><w:rPr><w:rFonts w:ascii="Times New Roman" w:hAnsi="Times New Roman"/><w:sz w:val="24"/><w:b w:val="{}"/></w:rPr><w:fldChar w:fldCharType="separate"/></w:r>"#,
            u8::from(separator_bold)
          )
        } else {
          String::new()
        };
        main.set_data(&mut package,format!(r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>BEGIN|</w:t></w:r><w:r><w:rPr><w:rFonts w:ascii="Cambria" w:hAnsi="Cambria"/><w:sz w:val="22"/><w:b w:val="{}"/></w:rPr><w:fldChar w:fldCharType="begin"/></w:r><w:r><w:rPr><w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/><w:sz w:val="26"/></w:rPr><w:instrText>MissingAuto</w:instrText></w:r>{separator_xml}<w:r><w:fldChar w:fldCharType="end"/></w:r><w:r><w:t>|END</w:t></w:r></w:p></w:body></w:document>"#,u8::from(begin_bold)).into_bytes()).unwrap();
        let document = extract(
          &package,
          &LayoutOptions {
            ui_language: Some("es-MX".into()),
            update_fields_at_print: Some(false),
            ..Default::default()
          },
        )
        .unwrap();
        let Block::Paragraph(paragraph) = &document.sections[0].blocks[0] else {
          panic!("field paragraph");
        };
        let run = paragraph
          .inlines
          .iter()
          .find_map(|item| match item {
            InlineItem::Text(run) if run.text == "¡Error! Marcador no definido." => Some(run),
            _ => None,
          })
          .expect("generated diagnostic");
        assert_eq!(
          run.style.font_family.as_deref(),
          Some(if separator {
            "Times New Roman"
          } else {
            "Cambria"
          })
        );
        assert_eq!(run.style.font_size_pt, if separator { 12.0 } else { 11.0 });
        assert_eq!(
          run.style.bold,
          !(if separator {
            separator_bold
          } else {
            begin_bold
          })
        );
      }
    }
  }
}
