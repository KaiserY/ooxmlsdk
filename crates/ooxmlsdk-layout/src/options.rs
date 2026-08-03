#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LayoutOptions {
  pub source_file_name: Option<String>,
  /// BCP 47 user-interface language used for application-generated labels,
  /// such as an automatic chart title that is not persisted in the package.
  pub ui_language: Option<String>,
  /// BCP 47 locale used for locale-dependent number, date, currency, and
  /// other value formatting. This is intentionally independent from the
  /// Office user-interface language.
  ///
  /// When absent, the user-interface language remains the compatibility
  /// fallback for callers of the pre-existing API.
  pub format_locale: Option<String>,
  /// BCP 47 language used for document authoring defaults, including a
  /// missing Office theme's script-specific fonts and the PDF document
  /// language. This is independent from translated application resources.
  ///
  /// When absent, the user-interface language remains the compatibility
  /// fallback for callers of the pre-existing API.
  pub default_document_language: Option<String>,
  /// Local civil time to use when an application explicitly refreshes
  /// unlocked WordprocessingML DATE, TIME, PRINTDATE, and SAVEDATE fields or
  /// generated PresentationML `datetime` text fields.
  ///
  /// Leaving this unset preserves the package's cached field results.
  pub field_update_datetime: Option<FieldUpdateDateTime>,
  pub action: LayoutActionOptions,
  pub diagnostics: LayoutDiagnosticsOptions,
}

/// Deterministic local civil time supplied for Office field updates.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FieldUpdateDateTime {
  pub year: u16,
  pub month: u8,
  pub day: u8,
  pub hour: u8,
  pub minute: u8,
  pub second: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LayoutActionOptions {
  pub paint: bool,
  pub complete: bool,
  pub calc_layout: bool,
  pub check_pages: bool,
}

impl Default for LayoutActionOptions {
  fn default() -> Self {
    Self {
      paint: true,
      complete: true,
      calc_layout: true,
      check_pages: true,
    }
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LayoutDiagnosticsOptions {
  pub collect_debug_records: bool,
  pub collect_reflow_records: bool,
  pub preserve_source_links: bool,
}
