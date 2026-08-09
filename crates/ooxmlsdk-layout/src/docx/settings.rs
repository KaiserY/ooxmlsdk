use super::*;

const MICROSOFT_WORD_COMPATIBILITY_URI: &str = "http://schemas.microsoft.com/office/word";

pub(super) fn hyphenation_settings(
  package: &mut WordprocessingDocument,
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

pub(super) fn compatibility_mode(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> u16 {
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

pub(super) fn no_column_balance(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
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
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .compatibility
        .iter()
        .find_map(|compat| compat.adjust_line_height_in_table.as_ref())
        .map(|setting| setting.val.is_none_or(|value| value.as_bool()))
    })
    .unwrap_or(false)
}

pub(super) fn do_not_use_html_paragraph_auto_spacing(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| {
      settings
        .compatibility
        .iter()
        .find_map(|compat| compat.do_not_use_html_paragraph_auto_spacing.as_ref())
        .map(|setting| setting.val.is_none_or(|value| value.as_bool()))
    })
    .unwrap_or(false)
}

pub(super) fn do_not_break_wrapped_tables(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(do_not_break_wrapped_tables_value)
}

pub(super) fn do_not_expand_shift_return(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .is_some_and(do_not_expand_shift_return_value)
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

pub(super) fn split_page_break_and_paragraph_mark(
  package: &mut WordprocessingDocument,
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
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> bool {
  main
    .document_settings_part(package)
    .and_then(|part| part.root_element(package).ok())
    .and_then(|settings| settings.update_fields_on_open.as_ref())
    .is_some_and(|setting| setting.val.is_none_or(|value| value.as_bool()))
}

pub(super) fn explicit_default_tab_stop_pt(
  package: &mut WordprocessingDocument,
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
    MICROSOFT_WORD_COMPATIBILITY_URI, PageBottomHyphenation, compatibility_setting_value,
    do_not_break_wrapped_tables_value, do_not_expand_shift_return_value, page_bottom_hyphenation,
    parse_compatibility_on_off, w,
  };

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
}
