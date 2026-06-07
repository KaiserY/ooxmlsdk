use super::*;

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
    // Source: LibreOffice sw/source/writerfilter/dmapper/SettingsTable.cxx
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

pub(super) fn default_tab_stop_pt(
  package: &mut WordprocessingDocument,
  main: &MainDocumentPart,
) -> f32 {
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
    .unwrap_or(DEFAULT_TAB_STOP_PT)
}
