//! Diagnostics captured at the layout-to-PDF text boundary.
//!
//! The normal conversion APIs do not collect these records. Call one of the
//! explicit `*_with_diagnostics` entry points when exact font selection,
//! shaping, or glyph positioning data is needed.

/// PDF bytes together with the font and glyph data used to produce them.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfConversionOutput {
  pub pdf: Vec<u8>,
  pub diagnostics: PdfConversionDiagnostics,
}

/// PDF bytes together with a bounded, low-overhead font-integrity audit.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfFontAuditOutput {
  pub pdf: Vec<u8>,
  pub audit: PdfFontAudit,
}

/// Candidate-side font and glyph invariants checked before PDF comparison.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PdfFontAudit {
  /// Resolved font faces, deduplicated across all pages.
  pub fonts: Vec<PdfFontFaceDiagnostics>,
  pub text_portion_count: usize,
  /// Non-tab portions that are actually emitted as visible PDF text.
  pub painted_text_portion_count: usize,
  pub explicit_glyph_portion_count: usize,
  pub glyph_run_count: usize,
  pub glyph_count: usize,
  /// Consecutive multi-glyph clusters that require PDF ActualText spans.
  pub actual_text_cluster_count: usize,
  /// A bounded set of integrity failures. An empty set is required to pass.
  pub issues: Vec<PdfFontAuditIssue>,
}

/// Location and stable category for one font-integrity failure.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfFontAuditIssue {
  pub page_index: usize,
  pub text_run_index: usize,
  pub portion_index: Option<usize>,
  pub glyph_run_index: Option<usize>,
  pub glyph_index: Option<usize>,
  pub kind: PdfFontAuditIssueKind,
  pub detail: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfFontAuditIssueKind {
  FontParse,
  KrillaFontLoad,
  MissingShapedGlyphs,
  MissingGlyph,
  PortionTextRange,
  GlyphTextRange,
  GlyphIdOutOfRange,
  NonFiniteGlyphMetric,
  InvalidGlyphBounds,
}

impl PdfFontAuditIssueKind {
  pub fn as_str(self) -> &'static str {
    match self {
      Self::FontParse => "font-parse",
      Self::KrillaFontLoad => "krilla-font-load",
      Self::MissingShapedGlyphs => "missing-shaped-glyphs",
      Self::MissingGlyph => "missing-glyph",
      Self::PortionTextRange => "portion-text-range",
      Self::GlyphTextRange => "glyph-text-range",
      Self::GlyphIdOutOfRange => "glyph-id-out-of-range",
      Self::NonFiniteGlyphMetric => "non-finite-glyph-metric",
      Self::InvalidGlyphBounds => "invalid-glyph-bounds",
    }
  }
}

/// Document-wide diagnostics captured immediately before PDF serialization.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PdfConversionDiagnostics {
  /// Resolved font faces, deduplicated across all pages.
  pub fonts: Vec<PdfFontFaceDiagnostics>,
  /// Page-local text and glyph placement records.
  pub pages: Vec<PdfPageDiagnostics>,
}

/// Stable identifying information and OpenType metrics for one resolved face.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfFontFaceDiagnostics {
  pub font_id: String,
  pub face_index: u32,
  pub data_len: usize,
  pub parse_error: Option<String>,
  pub checksum_adjustment: Option<u32>,
  pub postscript_name: Option<String>,
  pub family_names: Vec<String>,
  pub style_name: Option<String>,
  pub units_per_em: u16,
  pub glyph_count: u16,
  pub ascender_em: f32,
  pub descender_em: f32,
  pub cap_height_em: Option<f32>,
  pub global_bounds_em: PdfGlyphBoundsDiagnostics,
  pub monospaced: bool,
}

/// Diagnostics for one PDF page.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfPageDiagnostics {
  pub page_index: usize,
  pub width_pt: f32,
  pub height_pt: f32,
  pub text_runs: Vec<PdfTextRunDiagnostics>,
}

/// One positioned layout text run before it is emitted through Krilla.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfTextRunDiagnostics {
  pub text: String,
  /// Layout frame that owns this PDF text run, when the engine exposes one.
  pub source_frame_index: Option<usize>,
  /// Line within `source_frame_index` that owns this run.
  pub source_line_index: Option<usize>,
  /// Stable engine-local source path carried by `LayoutDocument`.
  pub source_path: Vec<usize>,
  pub x_pt: f32,
  pub y_pt: f32,
  pub baseline_y_pt: f32,
  pub line_height_pt: f32,
  pub width_pt: f32,
  pub font_size_pt: f32,
  pub character_spacing_pt: f32,
  pub baseline_shift_pt: f32,
  pub requested_font_family: Option<String>,
  pub requested_east_asia_font_family: Option<String>,
  pub requested_complex_font_family: Option<String>,
  pub bold: bool,
  pub italic: bool,
  pub small_caps: bool,
  pub portions: Vec<PdfTextPortionDiagnostics>,
}

/// A text, field, link, or tab portion emitted as one positioned unit.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfTextPortionDiagnostics {
  pub kind: PdfTextPortionKind,
  pub text_range_start: usize,
  pub text_range_end: usize,
  pub x_pt: f32,
  pub baseline_y_pt: f32,
  pub width_pt: f32,
  /// Whether layout supplied explicit shaped glyphs for this portion. A
  /// visible non-tab portion without them is a font-audit failure.
  pub has_explicit_glyphs: bool,
  pub glyph_runs: Vec<PdfGlyphRunDiagnostics>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PdfTextPortionKind {
  Text,
  Tab,
  Field,
  Link,
}

/// Consecutive glyphs that use the same resolved font face.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfGlyphRunDiagnostics {
  /// Index into [`PdfConversionDiagnostics::fonts`].
  pub font_index: usize,
  pub font_size_pt: f32,
  pub x_offset_pt: f32,
  pub synthetic_bold: bool,
  pub synthetic_italic: bool,
  pub glyphs: Vec<PdfGlyphDiagnostics>,
}

/// Exact normalized glyph values passed to Krilla's `Glyph` interface.
#[derive(Clone, Debug, PartialEq)]
pub struct PdfGlyphDiagnostics {
  pub glyph_id: u32,
  pub text_range_start: usize,
  pub text_range_end: usize,
  pub x_advance_em: f32,
  pub x_offset_em: f32,
  pub y_offset_em: f32,
  pub y_advance_em: f32,
  pub bounds_em: Option<PdfGlyphBoundsDiagnostics>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PdfGlyphBoundsDiagnostics {
  pub x_min_em: f32,
  pub y_min_em: f32,
  pub x_max_em: f32,
  pub y_max_em: f32,
}
