use ooxmlsdk_pdf_test::pdf_extract::TextObjectSummary;
use ooxmlsdk_pdf_test::{PdfSummary, pdf_summary_for_fixture, pdfexport_fixture_dir};

fn fixture(name: &str) -> std::path::PathBuf {
  pdfexport_fixture_dir().join(name)
}

fn render_summary(name: &str) -> PdfSummary {
  pdf_summary_for_fixture(&fixture(name)).unwrap()
}

fn text_objects(summary: &PdfSummary) -> impl Iterator<Item = &TextObjectSummary> {
  summary.text_objects.iter()
}

fn color_alpha(color: &str) -> &str {
  color
    .rsplit_once('@')
    .map(|(_, alpha)| alpha)
    .unwrap_or_else(|| panic!("missing alpha component in color {color}"))
}

fn color_rgb(color: &str) -> &str {
  color
    .split_once('@')
    .map(|(rgb, _)| rgb)
    .unwrap_or_else(|| panic!("missing alpha component in color {color}"))
}

fn assert_has_text_fill_color(summary: &PdfSummary, expected: &str) {
  assert!(
    text_objects(summary).any(|object| object.fill_color.as_deref() == Some(expected)),
    "missing text fill color {expected}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_has_text_fill_rgb(summary: &PdfSummary, expected_rgb: &str) {
  assert!(
    text_objects(summary).any(|object| {
      object
        .fill_color
        .as_deref()
        .is_some_and(|color| color_rgb(color) == expected_rgb)
    }),
    "missing text fill rgb {expected_rgb}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_has_text_stroke_color(summary: &PdfSummary, expected: &str) {
  assert!(
    text_objects(summary).any(|object| object.stroke_color.as_deref() == Some(expected)),
    "missing text stroke color {expected}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_has_text_alpha(summary: &PdfSummary, expected_alpha: &str) {
  assert!(
    text_objects(summary).any(|object| {
      object
        .fill_color
        .as_deref()
        .is_some_and(|color| color_alpha(color) == expected_alpha)
    }),
    "missing text alpha {expected_alpha}; text_objects={:?}",
    summary.text_objects
  );
}

#[test]
// Source: ../core/sw/qa/writerfilter/dmapper/TextEffectsHandler.cxx:testSemiTransparentText
fn core_docx_pdf_fixture_semi_transparent_text_preserves_alpha() {
  let summary = render_summary("semi-transparent-text.docx");
  // LibreOffice checks CharTransparence=74, which corresponds to 26% opacity.
  assert_has_text_alpha(&summary, "42");
}

#[test]
// Source: ../core/sw/qa/writerfilter/dmapper/TextEffectsHandler.cxx:testThemeColorTransparency
fn core_docx_pdf_fixture_theme_color_transparency_preserves_alpha() {
  let summary = render_summary("tdf152884_Char_Transparency.docx");
  // LibreOffice checks CharTransparence=74, which corresponds to 26% opacity.
  assert_has_text_alpha(&summary, "42");
}

#[test]
// Source: ../core/oox/qa/unit/drawingml.cxx:testChartDataLabelCharColor
fn core_docx_pdf_fixture_chart_data_label_char_color_preserves_white_label() {
  let summary = render_summary("chart-data-label-char-color.docx");
  assert_has_text_fill_color(&summary, "#ffffff@ff");
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testTdf54095_SmartArtThemeTextColor
fn core_docx_pdf_fixture_smartart_theme_text_color_preserves_dark2_text() {
  let summary = render_summary("tdf54095_SmartArtThemeTextColor.docx");
  assert_has_text_fill_color(&summary, "#1f497d@ff");
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testWriterFontwork2
fn core_docx_pdf_fixture_wordart2_preserves_fill_and_stroke_colors() {
  let summary = render_summary("tdf125885_WordArt2.docx");
  assert_has_text_fill_color(&summary, "#d7e4bd@ff");
  // LibreOffice checks LineColor=#953735 with 20% transparency.
  assert_has_text_stroke_color(&summary, "#953735@cc");
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testWriterFontworkNonAccentColor
fn core_docx_pdf_fixture_wordart_non_accent_colors_preserve_all_expected_fills() {
  let summary = render_summary("tdf152840_WordArt_non_accent_color.docx");
  for expected in ["#ffcc99@ff", "#ff0000@ff", "#ebddc3@ff", "#775f55@ff"] {
    assert_has_text_fill_color(&summary, expected);
  }
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testWriterFontworkDarkenTransparency
fn core_docx_pdf_fixture_wordart_darken_color_preserves_result_fill() {
  let summary = render_summary("tdf152896_WordArt_color_darken.docx");
  assert_has_text_fill_rgb(&summary, "#d0af72");
}
