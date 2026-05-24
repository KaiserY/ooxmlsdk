use ooxmlsdk_pdf_test::{
  LinkTargetKind, PdfSummary, parse_pdf_rect, pdf_summary_for_fixture, pdfexport_fixture_dir,
};

fn fixture(name: &str) -> std::path::PathBuf {
  pdfexport_fixture_dir().join("xlsx").join(name)
}

fn render_summary(name: &str) -> PdfSummary {
  pdf_summary_for_fixture(&fixture(name)).unwrap()
}

fn normalize_space(text: &str) -> String {
  text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn page_text(summary: &PdfSummary, page_index: usize) -> String {
  summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .map(|segment| segment.text.as_str())
    .collect::<Vec<_>>()
    .join("\n")
}

fn assert_page_contains(summary: &PdfSummary, page_index: usize, expected: &str) {
  let text = page_text(summary, page_index);
  let normalized_text = normalize_space(&text);
  let normalized_expected = normalize_space(expected);
  assert!(
    normalized_text.contains(&normalized_expected),
    "missing page {page_index} text {expected:?}; page text:\n{text}"
  );
}

fn assert_page_not_contains(summary: &PdfSummary, page_index: usize, unexpected: &str) {
  let text = page_text(summary, page_index);
  let normalized_text = normalize_space(&text);
  let normalized_unexpected = normalize_space(unexpected);
  assert!(
    !normalized_text.contains(&normalized_unexpected),
    "unexpected page {page_index} text {unexpected:?}; page text:\n{text}"
  );
}

fn assert_page_text_occurrences(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  expected_count: usize,
) {
  let text = normalize_space(&page_text(summary, page_index));
  let expected = normalize_space(expected);
  let count = text.matches(&expected).count();
  assert_eq!(
    count,
    expected_count,
    "page {page_index} text {expected:?} occurrence mismatch; page text:\n{}",
    page_text(summary, page_index)
  );
}

fn assert_media_box(
  summary: &PdfSummary,
  page_index: usize,
  expected_width: f32,
  expected_height: f32,
) {
  let media_box = summary
    .media_boxes
    .get(page_index)
    .unwrap_or_else(|| panic!("missing media box for page {page_index}; summary={summary:?}"));
  let bounds =
    parse_pdf_rect(media_box).unwrap_or_else(|_| panic!("invalid media box {media_box}"));
  assert!(
    (bounds.right - expected_width).abs() <= 0.05 && (bounds.top - expected_height).abs() <= 0.05,
    "page {page_index} media box mismatch: expected {expected_width}x{expected_height}, got {media_box}"
  );
}

fn assert_page_image_count(summary: &PdfSummary, page_index: usize, expected: usize) {
  let count = summary
    .images
    .iter()
    .filter(|image| image.page_index == page_index)
    .count();
  assert_eq!(
    count, expected,
    "page {page_index} image count mismatch; images={:?}",
    summary.images
  );
}

fn assert_link_target(summary: &PdfSummary, expected_target: &str) {
  assert!(
    summary.links.iter().any(|link| {
      link.target_kind == LinkTargetKind::ExternalUri
        && link.target.as_deref() == Some(expected_target)
    }),
    "missing link target {expected_target:?}; links={:?}",
    summary.links
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf123026_optimalRowHeight
fn mapped_xlsx_tdf123026_optimal_row_height_keeps_wrapped_text_visible() {
  let summary = render_summary("tdf123026_optimalRowHeight.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Sales Summary Report");
  assert_page_contains(&summary, 0, "Single level semi attached");
  assert_page_contains(&summary, 0, "Reflects $3,526/sqm.");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf159581_optimalRowHeight
fn mapped_xlsx_tdf159581_optimal_row_height_keeps_sheet2_rows_compact() {
  let summary = render_summary("tdf159581_optimalRowHeight.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "One honking big, optimal cell size");
  assert_page_contains(&summary, 0, "Should not affect other sheets");
  assert_page_contains(&summary, 1, "still optimally sized row heights on sheet2");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf144642_RowHeightRounding_saveByCalc
fn mapped_xlsx_tdf144642_calc_saved_row_height_keeps_last_row_on_second_page() {
  let summary = render_summary("tdf144642_RowHeight_10mm_SavedByCalc.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "25 ___");
  assert_page_contains(&summary, 1, "26 ___");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf144642_RowHeightRounding_saveByExcel
fn mapped_xlsx_tdf144642_excel_saved_row_height_keeps_all_rows_on_one_page() {
  let summary = render_summary("tdf144642_RowHeight_28.35pt_SavedByExcel.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "26 ___");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf145129_DefaultRowHeightRounding
fn mapped_xlsx_tdf145129_default_row_height_paginates_like_libreoffice() {
  let summary = render_summary("tdf145129_DefaultRowHeight_28.35pt_SavedByExcel.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "1");
  assert_page_contains(&summary, 0, "2");
  assert_page_contains(&summary, 1, "28");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test.cxx:testMiscRowHeightExport
fn mapped_xlsx_misc_row_heights_keep_visible_height_labels() {
  let summary = render_summary("miscrowheights.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_text_occurrences(&summary, 0, "30", 6);
  assert_page_text_occurrences(&summary, 0, "50", 4);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testSecondsWithoutTruncateAndDecimals
fn mapped_xlsx_seconds_without_truncate_keeps_decimal_seconds_display() {
  let summary = render_summary("seconds-without-truncate-and-decimals.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_contains(&summary, 0, "271433.61");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testEmbeddedTextInDecimal
fn mapped_xlsx_embedded_text_in_decimal_keeps_literal_format_text() {
  let summary = render_summary("embedded-text-in-decimal.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "6,543,210.123 456 78");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test.cxx:testCellBordersXLSX
fn mapped_xlsx_cell_borders_keeps_visible_border_style_labels() {
  let summary = render_summary("cell-borders.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "hair");
  assert_page_contains(&summary, 0, "mediumDashDotDot");
  assert_page_contains(&summary, 0, "double");
  assert_page_contains(
    &summary,
    1,
    "Screenshot of how the borders look in Excel XP",
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf136721_paper_size
fn mapped_xlsx_tdf136721_uses_imported_letter_landscape_page_size() {
  let summary = render_summary("tdf136721_letter_sized_paper.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 419.56, 297.64);
  assert_page_contains(&summary, 0, "Start");
  assert_page_contains(&summary, 0, "End");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf166724_cellAnchor
fn mapped_xlsx_tdf166724_cell_anchor_keeps_checkbox_caption_visible() {
  let summary = render_summary("tdf166724_cellAnchor.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "B3 checkBox");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf91634XLSX
fn mapped_xlsx_image_hyperlink_keeps_visible_image_and_google_link() {
  let summary = render_summary("image_hyperlink.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_image_count(&summary, 0, 1);
  assert_link_target(&summary, "https://www.google.com/");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testHiddenShapeXLSX
fn mapped_xlsx_hidden_shape_omits_hidden_drawing_output() {
  let summary = render_summary("hiddenShape.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_image_count(&summary, 0, 0);
  assert_eq!(
    normalize_space(&page_text(&summary, 0)),
    "",
    "hidden shape page should have no visible text; text={:?}",
    page_text(&summary, 0)
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testtdf169496_hidden_graphic
fn mapped_xlsx_tdf169496_hidden_graphic_omits_hidden_picture() {
  let summary = render_summary("tdf169496_hidden_graphic.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_image_count(&summary, 0, 0);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testHyperlinksXLSX
fn mapped_xlsx_hyperlinks_keeps_visible_imported_cell_text() {
  let summary = render_summary("hyperlinks.xlsx");
  assert_eq!(summary.page_count, 6);
  assert_page_contains(&summary, 0, "10:ABC10");
  assert_page_contains(&summary, 0, "10:ABC11");
  assert_page_contains(&summary, 0, "10:ABC12");
  assert_page_contains(&summary, 1, "Invalid Value");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf91634XLSX
fn mapped_xlsx_image_hyperlink_does_not_emit_unrelated_cell_text() {
  let summary = render_summary("image_hyperlink.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_not_contains(&summary, 0, "https://www.google.com/");
}
