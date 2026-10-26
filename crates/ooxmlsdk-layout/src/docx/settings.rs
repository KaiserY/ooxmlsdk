use super::*;

const MICROSOFT_WORD_COMPATIBILITY_URI: &str = "http://schemas.microsoft.com/office/word";

pub(super) fn legacy_chart_defaults(package: &WordprocessingDocument) -> bool {
  let Some(properties) = package
    .extended_file_properties_part()
    .and_then(|part| part.root_element(package).ok())
  else {
    return false;
  };
  // Office uses the producer's application/version for pre-2010 chart
  // defaults, independently of w:compatibilityMode. Native PDF controls
  // distinguish these exact application identifiers from a Microsoft
  // prefix, case changes, missing metadata and versions >=13. LibreOffice's
  // XmlFilterBase::checkDocumentProperties uses the same metadata owner,
  // but its broader prefix test is not Word's actual recognition rule.
  if !matches!(
    properties.application.as_deref(),
    Some(
      "Microsoft Office Word"
        | "Microsoft Excel"
        | "Microsoft PowerPoint"
        | "Microsoft Macintosh Word"
        | "Microsoft Macintosh Excel"
        | "Microsoft Macintosh PowerPoint"
    )
  ) {
    return false;
  }
  properties
    .application_version
    .as_deref()
    .and_then(|version| version.split_once('.'))
    .filter(|(major, minor)| {
      (1..=2).contains(&major.len())
        && (1..=4).contains(&minor.len())
        && major.bytes().all(|byte| byte.is_ascii_digit())
        && minor.bytes().all(|byte| byte.is_ascii_digit())
    })
    .and_then(|(major, _)| major.parse::<u8>().ok())
    .is_some_and(|major| major <= 12)
}

pub(super) fn hyphenation_settings(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> HyphenationSettings {
  let Some(settings) = main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
  else {
    return HyphenationSettings::default();
  };

  let automatic = settings
    .auto_hyphenation
    .as_ref()
    .is_some_and(|value| value.val.is_none_or(|value| value.as_bool()));
  let consecutive_line_limit = settings
    .consecutive_hyphen_limit
    .as_ref()
    .map_or(0, |value| value.val);
  let zone_pt = settings
    .hyphenation_zone
    .as_ref()
    .and_then(|value| twips_measure_to_points(&value.val))
    .filter(|value| value.is_finite() && *value >= 0.0)
    .unwrap_or(HyphenationSettings::default().zone_pt);
  let do_not_hyphenate_caps = settings
    .do_not_hyphenate_caps
    .as_ref()
    .is_some_and(|value| value.val.is_none_or(|value| value.as_bool()));
  let allow_at_page_bottom = compatibility_setting_value(
    settings,
    w::CompatSettingNameValues::AllowHyphenationAtTrackBottom,
  )
  .unwrap_or(false);
  let use_word2013_page_bottom = compatibility_setting_value(
    settings,
    w::CompatSettingNameValues::UseWord2013TrackBottomHyphenation,
  )
  .unwrap_or(true);

  HyphenationSettings {
    automatic,
    automatic_languages: Default::default(),
    consecutive_line_limit,
    zone_pt,
    do_not_hyphenate_caps,
    page_bottom: page_bottom_hyphenation(allow_at_page_bottom, use_word2013_page_bottom),
  }
}

fn compatibility_setting_value(
  settings: &w::Settings,
  name: w::CompatSettingNameValues,
) -> Option<bool> {
  settings
    .compatibility
    .iter()
    .flat_map(|compatibility| &compatibility.compatibility_setting)
    // SettingsTable::GetCompatSettingHasAndValue() in Writer uses document
    // order and deliberately lets a repeated Microsoft setting win.
    .rfind(|setting| setting.w_name == name && setting.w_uri == MICROSOFT_WORD_COMPATIBILITY_URI)
    .and_then(|setting| parse_compatibility_on_off(setting.w_val.as_str()))
}

fn parse_compatibility_on_off(value: &str) -> Option<bool> {
  if value.eq_ignore_ascii_case("true") || value.eq_ignore_ascii_case("on") || value == "1" {
    Some(true)
  } else if value.eq_ignore_ascii_case("false") || value.eq_ignore_ascii_case("off") || value == "0"
  {
    Some(false)
  } else {
    None
  }
}

pub(super) fn enable_open_type_features(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      compatibility_setting_value(settings, w::CompatSettingNameValues::EnableOpenTypeFeatures)
    })
    // MS-DOCX 2.3.3 makes omission equivalent to false.
    .unwrap_or(false)
}

pub(super) fn differentiate_multirow_table_headers(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
  mode: u16,
) -> bool {
  let settings = main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok());
  differentiate_multirow_table_headers_value(settings, mode)
}

fn differentiate_multirow_table_headers_value(settings: Option<&w::Settings>, mode: u16) -> bool {
  settings
    .and_then(|settings| {
      compatibility_setting_value(
        settings,
        w::CompatSettingNameValues::DifferentiateMultirowTableHeaders,
      )
    })
    // MS-DOCX 2.3.4 documents the setting. Native Word controls resolve an
    // omitted value to enabled in modern mode, disabled in Word 2007 mode.
    .unwrap_or(mode >= 15)
}

fn page_bottom_hyphenation(
  allow_at_page_bottom: bool,
  use_word2013_page_bottom: bool,
) -> PageBottomHyphenation {
  if allow_at_page_bottom || !use_word2013_page_bottom {
    // MS-DOCX describes explicit false as moving only the hyphenated word,
    // and LibreOffice imports it that way. Microsoft 365 fixed output
    // 16.0.20131 nevertheless leaves the selected hyphen at the page bottom
    // (tdf165354.docx). Follow the Office PDF calibration used by this
    // renderer; explicit true or an omitted setting retains the Word 2013
    // whole-line behavior below.
    PageBottomHyphenation::Allow
  } else {
    PageBottomHyphenation::MoveLine
  }
}

pub(super) fn compatibility_mode(package: &WordprocessingDocument, main: &MainDocumentPart) -> u16 {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings.compatibility.iter().find_map(|compat| {
        compat
          .compatibility_setting
          .iter()
          .find(|setting| setting.w_name == w::CompatSettingNameValues::CompatibilityMode)
          .and_then(|setting| setting.w_val.as_str().parse::<u16>().ok())
      })
    })
    // defaults a missing DOCX compatibilityMode to Word 2007 / mode 12.
    .unwrap_or(12)
}

pub(super) fn no_column_balance(package: &WordprocessingDocument, main: &MainDocumentPart) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .compatibility
        .iter()
        .find_map(|compat| compat.no_column_balance.as_ref())
        .map(|value| on_off_only_value(value.val))
    })
    .unwrap_or(false)
}

pub(super) fn adjust_line_height_in_table(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> (bool, bool) {
  let Some(part) = main.document_settings_part(package) else {
    return resolve_adjust_line_height_in_table(false, None);
  };

  let authored = part
    .root_element(package)
    .ok()
    .and_then(adjust_line_height_in_table_value);
  // Keep omission distinct from false. A present Settings part with no
  // authored switch participates in Word's application repair only when the
  // caller also repairs missing paragraph defaults and docGrid. This is the
  // tdf131203 state. tdf109306 omits the part entirely, while
  // Normalize/conflicting IDs.docx has complete styles and an explicit grid;
  // neither may receive the repaired table-grid behavior.
  resolve_adjust_line_height_in_table(true, authored)
}

fn adjust_line_height_in_table_value(settings: &w::Settings) -> Option<bool> {
  settings
    .compatibility
    .iter()
    .find_map(|compat| compat.adjust_line_height_in_table.as_ref())
    .map(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

fn resolve_adjust_line_height_in_table(
  has_settings_part: bool,
  authored: Option<bool>,
) -> (bool, bool) {
  if !has_settings_part {
    (false, false)
  } else {
    authored.map_or((false, true), |value| (value, false))
  }
}

pub(super) fn do_not_use_html_paragraph_auto_spacing(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(do_not_use_html_paragraph_auto_spacing_value)
}

fn do_not_use_html_paragraph_auto_spacing_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compat| compat.do_not_use_html_paragraph_auto_spacing.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

pub(super) fn do_not_use_indent_as_numbering_tab_stop(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(do_not_use_indent_as_numbering_tab_stop_value)
}

fn do_not_use_indent_as_numbering_tab_stop_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compat| compat.do_not_use_indent_as_numbering_tab_stop.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

pub(super) fn do_not_break_wrapped_tables(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(do_not_break_wrapped_tables_value)
}

pub(super) fn do_not_expand_shift_return(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(do_not_expand_shift_return_value)
}

pub(super) fn suppress_top_spacing(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(suppress_top_spacing_value)
}

pub(super) fn use_far_east_layout(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(use_far_east_layout_value)
}

pub(super) fn auto_space_like_word95(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(|settings| {
      settings
        .compatibility
        .iter()
        .find_map(|compatibility| compatibility.auto_space_like_word95.as_ref())
        .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
    })
}

pub(super) fn strict_first_and_last_chars(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| settings.strict_first_and_last_chars.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

pub(super) fn no_punctuation_kerning(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| settings.no_punctuation_kerning.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

pub(super) fn use_word97_line_break_rules(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(use_word97_line_break_rules_value)
}

fn use_word97_line_break_rules_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compatibility| compatibility.use_word97_line_break_rules.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

pub(super) fn balance_single_byte_double_byte_width(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(balance_single_byte_double_byte_width_value)
}

pub(super) fn no_leading(package: &WordprocessingDocument, main: &MainDocumentPart) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(no_leading_value)
}

fn do_not_break_wrapped_tables_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compatibility| compatibility.do_not_break_wrapped_tables.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

fn do_not_expand_shift_return_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compatibility| compatibility.do_not_expand_shift_return.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

fn suppress_top_spacing_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compatibility| compatibility.suppress_top_spacing.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

fn use_far_east_layout_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compatibility| compatibility.use_far_east_layout.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

fn balance_single_byte_double_byte_width_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compatibility| compatibility.balance_single_byte_double_byte_width.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

fn no_leading_value(settings: &w::Settings) -> bool {
  settings
    .compatibility
    .iter()
    .find_map(|compatibility| compatibility.no_leading.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

pub(super) fn split_page_break_and_paragraph_mark(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .compatibility
        .iter()
        .find_map(|compat| compat.split_page_break_and_paragraph_mark.as_ref())
        .map(|setting| setting.val.is_none_or(|value| value.as_bool()))
    })
    .unwrap_or(false)
}

pub(super) fn update_fields_on_open(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| settings.update_fields_on_open.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

pub(super) fn explicit_default_tab_stop_pt(
  package: &WordprocessingDocument,
  main: &MainDocumentPart,
) -> Option<f32> {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .default_tab_stop
        .as_ref()
        .and_then(|stop| twips_measure_to_points(&stop.val))
    })
    .filter(|value| value.is_finite() && *value > 0.0)
}

#[cfg(test)]
mod tests {
  use super::{
    MICROSOFT_WORD_COMPATIBILITY_URI, PageBottomHyphenation,
    balance_single_byte_double_byte_width_value, compatibility_setting_value,
    do_not_break_wrapped_tables_value, do_not_expand_shift_return_value,
    do_not_use_html_paragraph_auto_spacing_value, do_not_use_indent_as_numbering_tab_stop_value,
    no_leading_value, page_bottom_hyphenation, parse_compatibility_on_off,
    resolve_adjust_line_height_in_table, use_far_east_layout_value, w,
  };

  #[test]
  fn word97_line_break_setting_preserves_absent_and_false() {
    use ooxmlsdk::sdk::SdkType;
    for (xml, expected) in [
      ("", false),
      ("<w:useWord97LineBreakRules/>", true),
      ("<w:useWord97LineBreakRules w:val=\"0\"/>", false),
      ("<w:useWord97LineBreakRules w:val=\"1\"/>", true),
    ] {
      let settings = w::Settings::from_bytes(
        format!(
          "<w:settings xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:compat>{xml}</w:compat></w:settings>"
        )
        .as_bytes(),
      )
      .expect("settings");
      assert_eq!(
        super::use_word97_line_break_rules_value(&settings),
        expected
      );
    }
  }

  #[test]
  fn table_grid_setting_keeps_part_omission_and_authored_false_distinct() {
    assert_eq!(
      resolve_adjust_line_height_in_table(false, None),
      (false, false),
      "a missing Settings part has no application-repair signal",
    );
    assert_eq!(
      resolve_adjust_line_height_in_table(true, None),
      (false, true),
      "omission inside a present Settings part is deferred to the document-default repair gate",
    );
    assert_eq!(
      resolve_adjust_line_height_in_table(true, Some(true)),
      (true, false),
    );
    assert_eq!(
      resolve_adjust_line_height_in_table(true, Some(false)),
      (false, false),
      "authored false must not be reinterpreted as omission",
    );
  }

  #[test]
  fn page_bottom_hyphenation_precedence_matches_office_fixed_output() {
    assert_eq!(
      page_bottom_hyphenation(true, true),
      PageBottomHyphenation::Allow
    );
    assert_eq!(
      page_bottom_hyphenation(true, false),
      PageBottomHyphenation::Allow
    );
    assert_eq!(
      page_bottom_hyphenation(false, true),
      PageBottomHyphenation::MoveLine
    );
    assert_eq!(
      page_bottom_hyphenation(false, false),
      PageBottomHyphenation::Allow
    );
  }

  #[test]
  fn compatibility_on_off_accepts_the_wordprocessingml_lexical_forms() {
    for value in ["1", "true", "TRUE", "on", "ON"] {
      assert_eq!(parse_compatibility_on_off(value), Some(true));
    }
    for value in ["0", "false", "FALSE", "off", "OFF"] {
      assert_eq!(parse_compatibility_on_off(value), Some(false));
    }
    assert_eq!(parse_compatibility_on_off("15"), None);
  }

  #[test]
  fn opentype_feature_setting_preserves_omission_and_explicit_off() {
    use ooxmlsdk::sdk::SdkType;
    for (value, expected) in [
      (None, None),
      (Some("0"), Some(false)),
      (Some("off"), Some(false)),
      (Some("1"), Some(true)),
      (Some("true"), Some(true)),
    ] {
      let content = value.map_or_else(String::new, |value| {
        format!(
          "<w:compatSetting w:name=\"enableOpenTypeFeatures\" w:uri=\"http://schemas.microsoft.com/office/word\" w:val=\"{value}\"/>"
        )
      });
      let settings = w::Settings::from_bytes(
        format!(
          "<w:settings xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:compat>{content}</w:compat></w:settings>"
        )
        .as_bytes(),
      )
      .expect("settings");
      assert_eq!(
        compatibility_setting_value(
          &settings,
          w::CompatSettingNameValues::EnableOpenTypeFeatures
        ),
        expected
      );
    }
  }

  #[test]
  fn microsoft_compatibility_setting_requires_its_uri_and_last_value_wins() {
    let name = w::CompatSettingNameValues::AllowHyphenationAtTrackBottom;
    let setting = |uri: &str, value: &str| w::CompatibilitySetting {
      w_name: name,
      w_uri: uri.to_owned(),
      w_val: value.to_owned(),
    };
    let settings = w::Settings {
      compatibility: vec![
        w::Compatibility {
          compatibility_setting: vec![
            setting("http://www.example.com/not-word", "1"),
            setting(MICROSOFT_WORD_COMPATIBILITY_URI, "1"),
          ],
          ..w::Compatibility::default()
        },
        w::Compatibility {
          compatibility_setting: vec![setting(MICROSOFT_WORD_COMPATIBILITY_URI, "0")],
          ..w::Compatibility::default()
        },
      ],
      ..w::Settings::default()
    };

    assert_eq!(compatibility_setting_value(&settings, name), Some(false));
  }

  #[test]
  fn multirow_header_setting_preserves_explicit_values_and_mode_defaults() {
    for mode in [12, 15] {
      assert_eq!(
        super::differentiate_multirow_table_headers_value(None, mode),
        mode >= 15
      );
      assert_eq!(
        super::differentiate_multirow_table_headers_value(Some(&w::Settings::default()), mode),
        mode >= 15
      );
      for (val, expected) in [("1", true), ("0", false)] {
        let settings = w::Settings {
          compatibility: vec![w::Compatibility {
            compatibility_setting: vec![w::CompatibilitySetting {
              w_name: w::CompatSettingNameValues::DifferentiateMultirowTableHeaders,
              w_uri: MICROSOFT_WORD_COMPATIBILITY_URI.to_owned(),
              w_val: val.to_owned(),
            }],
            ..Default::default()
          }],
          ..Default::default()
        };
        assert_eq!(
          super::differentiate_multirow_table_headers_value(Some(&settings), mode),
          expected
        );
      }
    }
  }

  #[test]
  fn do_not_break_wrapped_tables_honors_on_off_and_omission() {
    let settings = |value: Option<Option<ooxmlsdk::simple_type::OnOffValue>>| w::Settings {
      compatibility: vec![w::Compatibility {
        do_not_break_wrapped_tables: value.map(|val| w::DoNotBreakWrappedTables { val }),
        ..w::Compatibility::default()
      }],
      ..w::Settings::default()
    };

    assert!(do_not_break_wrapped_tables_value(&settings(Some(None))));
    assert!(do_not_break_wrapped_tables_value(&settings(Some(Some(
      ooxmlsdk::simple_type::OnOffValue::True,
    )))));
    assert!(!do_not_break_wrapped_tables_value(&settings(Some(Some(
      ooxmlsdk::simple_type::OnOffValue::False,
    )))));
    assert!(!do_not_break_wrapped_tables_value(&settings(None)));
  }

  #[test]
  fn numbering_indent_tab_compatibility_honors_on_off_and_omission() {
    let settings = |value: Option<Option<ooxmlsdk::simple_type::OnOffValue>>| w::Settings {
      compatibility: vec![w::Compatibility {
        do_not_use_indent_as_numbering_tab_stop: value
          .map(|val| w::DoNotUseIndentAsNumberingTabStop { val }),
        ..w::Compatibility::default()
      }],
      ..w::Settings::default()
    };
    assert!(do_not_use_indent_as_numbering_tab_stop_value(&settings(
      Some(None)
    )));
    assert!(do_not_use_indent_as_numbering_tab_stop_value(&settings(
      Some(Some(ooxmlsdk::simple_type::OnOffValue::True,))
    )));
    assert!(!do_not_use_indent_as_numbering_tab_stop_value(&settings(
      Some(Some(ooxmlsdk::simple_type::OnOffValue::False,))
    )));
    assert!(!do_not_use_indent_as_numbering_tab_stop_value(&settings(
      None
    )));
  }

  #[test]
  fn do_not_expand_shift_return_honors_on_off_and_omission() {
    let settings = |value: Option<Option<ooxmlsdk::simple_type::OnOffValue>>| w::Settings {
      compatibility: vec![w::Compatibility {
        do_not_expand_shift_return: value.map(|val| w::DoNotExpandShiftReturn { val }),
        ..w::Compatibility::default()
      }],
      ..w::Settings::default()
    };

    assert!(do_not_expand_shift_return_value(&settings(Some(None))));
    assert!(do_not_expand_shift_return_value(&settings(Some(Some(
      ooxmlsdk::simple_type::OnOffValue::True,
    )))));
    assert!(!do_not_expand_shift_return_value(&settings(Some(Some(
      ooxmlsdk::simple_type::OnOffValue::False,
    )))));
    assert!(!do_not_expand_shift_return_value(&settings(None)));
  }

  #[test]
  fn non_html_paragraph_spacing_honors_on_off_and_omission() {
    let settings = |value: Option<Option<ooxmlsdk::simple_type::OnOffValue>>| w::Settings {
      compatibility: vec![w::Compatibility {
        do_not_use_html_paragraph_auto_spacing: value
          .map(|val| w::DoNotUseHtmlParagraphAutoSpacing { val }),
        ..w::Compatibility::default()
      }],
      ..w::Settings::default()
    };

    assert!(do_not_use_html_paragraph_auto_spacing_value(&settings(
      Some(None)
    )));
    assert!(do_not_use_html_paragraph_auto_spacing_value(&settings(
      Some(Some(ooxmlsdk::simple_type::OnOffValue::True))
    )));
    assert!(!do_not_use_html_paragraph_auto_spacing_value(&settings(
      Some(Some(ooxmlsdk::simple_type::OnOffValue::False))
    )));
    assert!(!do_not_use_html_paragraph_auto_spacing_value(&settings(
      None
    )));
  }

  #[test]
  fn far_east_layout_honors_on_off_and_omission() {
    let settings = |value: Option<Option<ooxmlsdk::simple_type::OnOffValue>>| w::Settings {
      compatibility: vec![w::Compatibility {
        use_far_east_layout: value.map(|val| w::UseFarEastLayout { val }),
        ..w::Compatibility::default()
      }],
      ..w::Settings::default()
    };

    assert!(use_far_east_layout_value(&settings(Some(None))));
    assert!(use_far_east_layout_value(&settings(Some(Some(
      ooxmlsdk::simple_type::OnOffValue::True,
    )))));
    assert!(!use_far_east_layout_value(&settings(Some(Some(
      ooxmlsdk::simple_type::OnOffValue::False,
    )))));
    assert!(!use_far_east_layout_value(&settings(None)));
  }

  #[test]
  fn single_double_byte_balance_honors_on_off_and_omission() {
    let settings = |value: Option<Option<ooxmlsdk::simple_type::OnOffValue>>| w::Settings {
      compatibility: vec![w::Compatibility {
        balance_single_byte_double_byte_width: value
          .map(|val| w::BalanceSingleByteDoubleByteWidth { val }),
        ..w::Compatibility::default()
      }],
      ..w::Settings::default()
    };

    assert!(balance_single_byte_double_byte_width_value(&settings(
      Some(None)
    )));
    assert!(balance_single_byte_double_byte_width_value(&settings(
      Some(Some(ooxmlsdk::simple_type::OnOffValue::True),)
    )));
    assert!(!balance_single_byte_double_byte_width_value(&settings(
      Some(Some(ooxmlsdk::simple_type::OnOffValue::False),)
    )));
    assert!(!balance_single_byte_double_byte_width_value(&settings(
      None
    )));
  }

  #[test]
  fn no_leading_honors_on_off_and_omission() {
    let settings = |value: Option<Option<ooxmlsdk::simple_type::OnOffValue>>| w::Settings {
      compatibility: vec![w::Compatibility {
        no_leading: value.map(|val| w::NoLeading { val }),
        ..w::Compatibility::default()
      }],
      ..w::Settings::default()
    };

    assert!(no_leading_value(&settings(Some(None))));
    assert!(no_leading_value(&settings(Some(Some(
      ooxmlsdk::simple_type::OnOffValue::True,
    )))));
    assert!(!no_leading_value(&settings(Some(Some(
      ooxmlsdk::simple_type::OnOffValue::False,
    )))));
    assert!(!no_leading_value(&settings(None)));
  }
}
