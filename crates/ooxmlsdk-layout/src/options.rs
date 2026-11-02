#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LayoutOptions {
  pub source_file_name: Option<String>,
  /// Complete storage path used by Word's FILENAME \\p field. When supplied,
  /// its final component also owns bare FILENAME; no filesystem access occurs.
  pub source_file_path: Option<String>,
  /// Explicit application user name for unlocked Word USERNAME fields when
  /// field evaluation is requested. This is independent of document authors.
  /// `None` preserves the cached result; an empty string is an empty name.
  pub field_user_name: Option<String>,
  /// Explicit application initials for Word USERINITIALS fields. Initials
  /// are independent of the name and are never inferred from the host.
  pub field_user_initials: Option<String>,
  /// Resolved Word document template, as an OOXML package. When the document
  /// enables `w:linkStyles`, import its style definitions on open. The caller
  /// resolves either the attached template or its application Normal template;
  /// no filesystem/network lookup occurs. An absent or invalid template leaves
  /// document styles unchanged. Clones share these immutable package bytes.
  pub linked_styles_template: Option<std::sync::Arc<[u8]>>,
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
  /// Languages whose automatic hyphenation dictionaries are available to
  /// the producing application. `None` uses all bundled dictionaries; an
  /// empty list disables automatic dictionary breaks. Authored soft hyphens
  /// remain available independently of this installation setting.
  pub automatic_hyphenation_languages: Option<Vec<String>>,
  /// Optional English proofing data for matching an application's dictionary.
  /// Immutable parsed data is shared across runs and repeated layout passes.
  pub english_hyphenation_dictionary:
    Option<std::sync::Arc<crate::hyphenation::EnglishHyphenationDictionary>>,
  /// Word's print-time field update policy. `Some(false)` preserves cached
  /// body fields that require an explicit print update, such as TOC and
  /// MERGEFIELD. Pagination/display fields and repeating headers/footers
  /// still follow their own update rules; field locks remain effective.
  /// `None` retains the existing update behavior. This does not request a
  /// document-open update or provide a clock for date/time fields.
  pub update_fields_at_print: Option<bool>,
  /// Local civil time to use when an application explicitly refreshes
  /// unlocked WordprocessingML DATE, TIME, PRINTDATE, and SAVEDATE fields or
  /// generated PresentationML `datetime` text fields. SpreadsheetML formula
  /// recalculation uses its date for TODAY(), in the workbook's date system.
  /// SpreadsheetML header/footer `&D` and `&T` fields use this local time
  /// with the format locale. Without it, their legacy literal output remains.
  ///
  /// Leaving this unset preserves Word/Presentation cached field results;
  /// spreadsheet TODAY() retains the formula evaluator's system-clock fallback.
  pub field_update_datetime: Option<FieldUpdateDateTime>,
  /// IANA time-zone name used to convert absolute package-property
  /// timestamps, such as `dcterms:created`, when fields are refreshed.
  ///
  /// This is independent from the UI language and formatting locale. When
  /// absent or invalid, fields that require an absolute-to-local conversion
  /// preserve their cached results.
  pub field_update_time_zone: Option<String>,
  /// Include slides marked as hidden in PresentationML fixed-format output.
  pub include_hidden_slides: bool,
  /// Fixed-format bitmap working density requested by the caller.  `None`
  /// retains the Office print-compatible 200-DPI default.
  pub fixed_output_raster_dpi: Option<u32>,
  /// Explicit native Word text-picture realization density, independent of
  /// fixed-output PDF settings. When present, glyphs, 3-D surfaces and text
  /// effects are rasterized together before a lossless PNG is produced.
  /// `None` preserves the existing fixed-output pipeline.
  pub native_picture_dpi: Option<u32>,
  /// Whether the fixed-output conformance profile forbids transparent paint.
  ///
  /// This is an output constraint rather than a source-document property.
  /// Callers targeting profiles such as PDF/A-1 set it so layout can follow
  /// the producing application's profile-specific drawing behavior before
  /// the backend serializes the fixed pages.
  pub fixed_output_forbids_transparency: bool,
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
