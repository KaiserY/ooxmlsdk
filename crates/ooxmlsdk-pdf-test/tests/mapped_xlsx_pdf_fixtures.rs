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

fn assert_page_contains_any(summary: &PdfSummary, page_index: usize, expected: &[&str]) {
  let text = page_text(summary, page_index);
  let normalized_text = normalize_space(&text);
  assert!(
    expected
      .iter()
      .any(|item| normalized_text.contains(&normalize_space(item))),
    "missing page {page_index} text any of {expected:?}; page text:\n{text}"
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

fn assert_page_path_count_at_least(summary: &PdfSummary, page_index: usize, expected: usize) {
  let count = summary
    .paths
    .iter()
    .filter(|path| path.page_index == page_index)
    .count();
  assert!(
    count >= expected,
    "expected at least {expected} paths on page {page_index}, got {count}; paths={:?}",
    summary.paths
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

fn assert_any_link_target(summary: &PdfSummary, expected_target: &str) {
  assert!(
    summary
      .links
      .iter()
      .any(|link| link.target.as_deref() == Some(expected_target)),
    "missing link target {expected_target:?}; links={:?}",
    summary.links
  );
}

fn assert_text_object_font_size(summary: &PdfSummary, expected_text: &str, expected_size: &str) {
  assert!(
    summary.text_objects.iter().any(
      |object| normalize_space(&object.text).contains(expected_text)
        && object.scaled_font_size == expected_size
    ),
    "missing text object {expected_text:?} with font size {expected_size}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_text_object_fill_color(summary: &PdfSummary, expected_text: &str, expected_color: &str) {
  assert!(
    summary.text_objects.iter().any(|object| {
      normalize_space(&object.text).contains(expected_text)
        && object.fill_color.as_deref() == Some(expected_color)
    }),
    "missing text object {expected_text:?} with fill color {expected_color}; text_objects={:?}",
    summary.text_objects
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

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testPreserveTextWhitespaceXLSX
fn mapped_xlsx_preserve_whitespace_keeps_visible_text() {
  let summary = render_summary("preserve-whitespace.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_contains(&summary, 0, "abc");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testPreserveTextWhitespace2XLSX
fn mapped_xlsx_preserve_space_keeps_single_line_with_inner_spaces() {
  let summary = render_summary("preserve_space.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_contains(&summary, 0, "abc 123456 456");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testEscapedUnicodeXLSX
fn mapped_xlsx_escape_unicode_keeps_newline_text_without_literal_escape() {
  let summary = render_summary("escape-unicode.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Line 1");
  assert_page_contains(&summary, 0, "Line 2");
  assert_page_contains(&summary, 0, "Line 3");
  assert_page_contains(&summary, 0, "Line 4");
  assert_page_not_contains(&summary, 0, "_x000D_");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testSingleLine_xlsx
fn mapped_xlsx_cell_multi_line_keeps_single_and_multi_paragraph_cells() {
  let summary = render_summary("cell-multi-line.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Line1Line2Line3");
  assert_page_contains(&summary, 0, "Line1 Line2 Line3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testBooleanFormatXLSX
fn mapped_xlsx_check_boolean_renders_boolean_values() {
  let summary = render_summary("check-boolean.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_text_occurrences(&summary, 0, "TRUE", 2);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testCellValueXLSX
fn mapped_xlsx_cell_value_keeps_imported_visible_values() {
  let summary = render_summary("cell-value.xlsx");
  assert_eq!(summary.page_count, 8);
  assert_page_contains(&summary, 0, "-2012");
  assert_page_contains(&summary, 0, "-3.14");
  assert_page_contains(&summary, 0, "Hello, Calc!");
  assert_page_contains(
    &summary,
    0,
    "Calc is the spreadsheet program you've always needed.",
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testFontSizeXLSX
fn mapped_xlsx_font_size_keeps_18pt_textbox_text() {
  let summary = render_summary("fontSize.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "sardfasef");
  assert_text_object_font_size(&summary, "sardfasef", "18.00");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testSheetRunParagraphPropertyXLSX
fn mapped_xlsx_text_color_keeps_red_and_green_rich_text() {
  let summary = render_summary("TextColor.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Red Green");
  assert_text_object_fill_color(&summary, "Red", "#ff0000@ff");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testTextUnderlineColorXLSX
fn mapped_xlsx_underline_color_keeps_two_textbox_labels_visible() {
  let summary = render_summary("underlineColor.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_text_occurrences(&summary, 0, "Text Box", 2);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testEditEngStrikeThroughXLSX
fn mapped_xlsx_strike_through_keeps_rich_text_content_visible() {
  let summary = render_summary("strike-through.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_contains(&summary, 0, "this is strike through this not");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testHiddenSheetsXLSX
fn mapped_xlsx_hidden_sheets_prints_only_visible_sheet() {
  let summary = render_summary("hidden_sheets.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_contains(&summary, 0, "Sheet2");
  assert_page_not_contains(&summary, 0, "Sheet1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf121715_FirstPageHeaderFooterXLSX
fn mapped_xlsx_tdf121715_keeps_first_and_even_page_headers_footers() {
  let summary = render_summary("tdf121715.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "First Page Header");
  assert_page_contains(&summary, 0, "First Page Footer");
  assert_page_contains(&summary, 1, "Even Header");
  assert_page_contains(&summary, 1, "Even Footer");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf134459_HeaderFooterColorXLSX
fn mapped_xlsx_tdf134459_keeps_colored_header_footer_text() {
  let summary = render_summary("tdf134459_HeaderFooterColor.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_text_occurrences(&summary, 0, "l c r", 2);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf134817_HeaderFooterTextWith2SectionXLSX
fn mapped_xlsx_tdf134817_keeps_multi_section_header_footer_text() {
  let summary = render_summary("tdf134817_HeaderFooterTextWith2Section.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_contains(&summary, 0, "aaa bbb");
  assert_page_contains(&summary, 0, "cambdant");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTextDirectionXLSX
fn mapped_xlsx_writing_mode_keeps_visible_text_direction_samples() {
  let summary = render_summary("writingMode.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "English (Yes).");
  assert_page_contains(&summary, 0, "English(Yes).");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testHyperlinkXLSX
fn mapped_xlsx_hyperlink_keeps_internal_sheet_link_annotation() {
  let summary = render_summary("hyperlink.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, ">");
  assert_any_link_target(&summary, "#Sheet2!A1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testSheetTextBoxHyperlinkXLSX
fn mapped_xlsx_textbox_hyperlink_keeps_visible_text_and_link_annotation() {
  let summary = render_summary("textbox-hyperlink.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "text");
  assert!(
    summary.link_annotation_count >= 1,
    "expected textbox hyperlink annotation; links={:?}",
    summary.links
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf123645XLSX
fn mapped_xlsx_chart_hyperlink_keeps_chart_text_and_link_targets() {
  let summary = render_summary("chart_hyperlink.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "Chart Title");
  assert_page_contains(&summary, 1, "Row 1");
  assert_link_target(&summary, "file:///C:/TEMP/test.xlsx");
  assert_any_link_target(&summary, "#Sheet2!A1");
  assert_link_target(
    &summary,
    "https://bugs.documentfoundation.org/show_bug.cgi?id=123645",
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test5.cxx:testTotalRowToggle
fn mapped_xlsx_table_style_keeps_visible_table_and_totals_row() {
  let summary = render_summary("TableStyleTest.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "A B C");
  assert_page_contains(&summary, 0, "Total 3 2.75 Ft");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTotalsRowFunction
fn mapped_xlsx_totals_row_function_keeps_sum_total_visible() {
  let summary = render_summary("totalsRowFunction.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_media_box(&summary, 0, 792.0, 612.0);
  assert_page_contains(&summary, 0, "PRESENT PLANNER");
  assert_page_contains(&summary, 0, "Total £350.00");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTotalsRowShown
fn mapped_xlsx_totals_row_shown_keeps_table_data_without_extra_total_row() {
  let summary = render_summary("totalsRowShown.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Desc Corn Hay Soy");
  assert_page_contains(&summary, 0, "Price $ 1.00 $ 2.00 $ 3.00");
  assert_page_contains(&summary, 0, "Unit ton bushel pound");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testAutofilterHiddenButton
fn mapped_xlsx_hidden_button_keeps_autofilter_header_row_visible() {
  let summary = render_summary("hiddenButton.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Col 1 Col 2 Col 3 Col 4 Col 5");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testAutofilterShowButton
fn mapped_xlsx_autofilter_show_button_keeps_filtered_columns_visible() {
  let summary = render_summary("autofilterShowButton.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "a b c DDD g h III");
  assert_page_contains(&summary, 0, "1 def example");
  assert_page_contains(&summary, 1, "III");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testDateAutofilterXLSX
fn mapped_xlsx_date_autofilter_keeps_filtered_date_rows_visible() {
  let summary = render_summary("dateAutofilter.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "ID Date");
  assert_page_contains(&summary, 0, "one 3/2/2017");
  assert_page_contains(&summary, 0, "three 10/1/2014");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testAutofilterColorsOOXML
fn mapped_xlsx_autofilter_colors_keeps_background_filtered_rows() {
  let summary = render_summary("autofilter-colors.xlsx");
  assert_eq!(summary.page_count, 1);
  // LibreOffice pdftotext keeps the first two adjacent header cells joined;
  // PDFium may segment the same positioned text with an extracted space.
  assert_page_contains_any(
    &summary,
    0,
    &["BackgroundForeground Both", "Background Foreground Both"],
  );
  assert_page_contains(&summary, 0, "2 2 2");
  assert_page_contains(&summary, 0, "3 3 3");
  assert_page_not_contains(&summary, 0, "1 1 1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testAutofilterColorsOOXML2
fn mapped_xlsx_autofilter_colors_fg_keeps_foreground_filtered_rows() {
  let summary = render_summary("autofilter-colors-fg.xlsx");
  assert_eq!(summary.page_count, 1);
  // LibreOffice pdftotext keeps the first two adjacent header cells joined;
  // PDFium may segment the same positioned text with an extracted space.
  assert_page_contains_any(
    &summary,
    0,
    &["BackgroundForeground Both", "Background Foreground Both"],
  );
  assert_page_contains(&summary, 0, "1 1 1");
  assert_page_contains(&summary, 0, "2 2 2");
  assert_page_contains(&summary, 0, "3 3 3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf130104_XLSXIndent
fn mapped_xlsx_tdf130104_keeps_imported_indent_positions_visible() {
  let summary = render_summary("tdf130104_indent.xlsx");
  assert_eq!(summary.page_count, 1);
  // Upstream verifies that imported XLSX indent values 0..10 survive
  // round-trip in styles.xml; PDFium may segment the visible CJK label and
  // trailing digit separately.
  for value in ["0", "5", "10"] {
    assert_page_contains(&summary, 0, &format!("Indent by {value}"));
    assert_page_contains(&summary, 0, "缩进");
    assert_page_contains(&summary, 0, value);
  }
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testHeaderFontStyleXLSX
fn mapped_xlsx_tdf134826_keeps_header_font_style_text_visible() {
  let summary = render_summary("tdf134826.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Bold");
  assert_page_contains(&summary, 0, "Italic");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf151755_stylesLostOnXLSXExport
fn mapped_xlsx_tdf151755_keeps_daily_calendar_template_visible() {
  let summary = render_summary("tdf151755_stylesLostOnXLSXExport.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Daily Calendar Date");
  assert_page_contains(&summary, 0, "Time Appointment To Do Errands Calls");
  assert_page_contains(&summary, 0, "07:00:00");
  assert_page_contains(&summary, 0, "18:30:00");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf152581_bordercolorNotExportedToXLSX
fn mapped_xlsx_tdf152581_keeps_conditional_border_fixture_text_visible() {
  let summary = render_summary("tdf152581_bordercolorNotExportedToXLSX.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "test");
  assert_page_contains(&summary, 0, "x");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testCheckboxFormControlXlsxExport
fn mapped_xlsx_checkbox_form_control_keeps_label_visible() {
  let summary = render_summary("checkbox-form-control.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "1");
  assert_page_contains(&summary, 0, "Check Box 1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testControlImport
fn mapped_xlsx_singlecontrol_keeps_form_control_sheet_text_visible() {
  let summary = render_summary("singlecontrol.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "form control inside cell b517");
  assert_page_contains(&summary, 1, "adfa");
  assert_page_contains(&summary, 2, "adfad");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testActiveXOptionButtonGroup
fn mapped_xlsx_radio_buttons_keep_grouped_button_labels_visible() {
  let summary = render_summary("tdf111980_radioButtons.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Group Box 7");
  assert_page_contains(&summary, 0, "ActiveX button1");
  assert_page_contains(&summary, 0, "Form button2");
  assert_page_contains(&summary, 1, "ActiveX 3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf153767
fn mapped_xlsx_tdf153767_keeps_boolean_strings_visible() {
  let summary = render_summary("tdf153767.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_media_box(&summary, 0, 612.0, 792.0);
  assert_page_contains(&summary, 0, "Contact Name Address City Postal Code Country");
  assert_page_contains(&summary, 1, "TRUE");
  assert_page_contains(&summary, 1, "FALSE");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf161301
fn mapped_xlsx_tdf161301_keeps_japanese_era_date_strings_visible() {
  let summary = render_summary("tdf161301.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_text_occurrences(&summary, 1, "CE784年2月20日", 2);
}

#[test]
// Source: ../core/sc/qa/unit/cond_format.cxx:testNewCondFormatXLSX
fn mapped_xlsx_new_cond_format_keeps_rule_sample_table_visible() {
  let summary = render_summary("new_cond_format_test.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(
    &summary,
    0,
    "top n elements bottom n elements top n percent bottom n percent above average",
  );
  assert_page_contains(
    &summary,
    1,
    "below average above equal average below equal average",
  );
  assert_page_contains(&summary, 2, "2.00 2 1 1.000 4.00 3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf83671_SmartArt_import
fn mapped_xlsx_tdf83671_smartart_import_keeps_diagram_text_visible() {
  let summary = render_summary("tdf83671_SmartArt_import.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "start");
  // Upstream validates that the SmartArt group and background shape import;
  // PDF text extraction order differs between renderers, so assert the
  // converted diagram labels independently.
  assert_page_contains(&summary, 0, "back");
  assert_page_contains(&summary, 0, "middle");
  assert_page_contains(&summary, 0, "front");
  assert_page_contains(&summary, 0, "end");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf151818_SmartArtFontColor
fn mapped_xlsx_tdf151818_smartart_theme_font_text_stays_visible() {
  let summary = render_summary("tdf151818_SmartartThemeFontColor.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "One Two Three");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotCacheExportXLSX
fn mapped_xlsx_pivot_cache_mixed_types_keeps_visible_pivot_values() {
  let summary = render_summary("pivot-table/with-strings-integers-and-dates.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(
    &summary,
    0,
    "mixed strings a Sum of all fields are integers",
  );
  assert_page_contains(&summary, 0, "Total Result 16665");
  assert_page_contains(&summary, 2, "tekst 6/7/09 10:53 AM");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableTwoDataFields
fn mapped_xlsx_two_data_fields_keeps_pivot_data_field_results_visible() {
  let summary = render_summary("pivot-table/two-data-fields.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Name Sum of Value Count of Value2");
  assert_page_contains(&summary, 0, "Total Result 3.6512482152 7");
  assert_page_contains(&summary, 1, "Name Value");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableCompactLayoutXLSX
fn mapped_xlsx_pivotcompact_keeps_compact_layout_output_visible() {
  let summary = render_summary("pivot-table/pivotcompact.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Sum of Val D");
  assert_page_contains(&summary, 1, "Row Labels ddd ddx Total Result");
  assert_page_contains(&summary, 1, "Total Result 40 41 81");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testFirstHeaderRowZero
fn mapped_xlsx_first_header_row_zero_keeps_under_text_visible() {
  let summary = render_summary("pivot-table/first_header_row_zero.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "A Suma de N Suma de V");
  assert_page_contains(&summary, 1, "Total Result 12 12");
  assert_page_contains(&summary, 1, "under");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testCalcFields1XLSX
fn mapped_xlsx_calcfields_keeps_calculated_field_pivot_totals_visible() {
  let summary = render_summary("pivot-table/calcfields.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 2, "Pivot Table_Sheet1_1");
  assert_page_contains(&summary, 2, "TATA 24 6.00 Ft");
  assert_page_contains(&summary, 2, "Total Result 114 18.00 Ft");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testCalcFields2XLSX
fn mapped_xlsx_onlycalcfields_keeps_calculated_only_pivot_visible() {
  let summary = render_summary("pivot-table/onlycalcfields.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 2, "Name (empty)");
  // LibreOffice renders the calculated-only pivot row labels as separate
  // visible lines in PDF; PDFium exposes that line segmentation directly.
  for label in ["TATA", "TITI", "TOTO", "Total Result"] {
    assert_page_contains(&summary, 2, label);
  }
  assert_page_contains(&summary, 3, "5");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testGroupAndCalcFieldXLSX
fn mapped_xlsx_groupwithcalcfields_keeps_grouped_calculated_totals_visible() {
  let summary = render_summary("pivot-table/groupwithcalcfields.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 0, "Data");
  assert_page_contains(&summary, 0, "Group1 45 54 63");
  assert_page_contains(&summary, 0, "Total Result 171 189 207");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testCalcFieldSingleDataDimXLSX
fn mapped_xlsx_tdf126858_keeps_single_calculated_pivot_fixture_visible() {
  let summary = render_summary("pivot-table/tdf126858-1.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "товар (empty)");
  assert_page_contains(&summary, 0, "апельсин банан вишня Total Result");
  // Source test checks the single calculated pivot values.  The source table
  // header remains visible, but its last fields may wrap/extract separately.
  for label in ["товар", "кол-во", "цена", "за", "ед", "стоимость"] {
    assert_page_contains(&summary, 1, label);
  }
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testCalcFieldDiffAggregationXLSX
fn mapped_xlsx_test_diff_aggregation_keeps_sum_and_count_pivots_visible() {
  let summary = render_summary("pivot-table/test_diff_aggregation.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 1, "Pivot Table_Sheet1_1");
  assert_page_contains(&summary, 1, "2010 78");
  assert_page_contains(&summary, 2, "Pivot Table_Sheet1_2");
  assert_page_contains(&summary, 2, "Total Result 6");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableDoubleFieldFilterXLSX
fn mapped_xlsx_pivottable_double_field_filter_keeps_filtered_items_visible() {
  let summary = render_summary("pivottable_double_field_filter.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(
    &summary,
    0,
    "Double field1 Double field2 Double field3 Datas",
  );
  assert_page_contains(&summary, 0, "2 2.00 20,000.00 12");
  assert_page_contains(&summary, 1, "10,000.00 22");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableStringFieldFilterXLSX
fn mapped_xlsx_pivottable_string_field_filter_keeps_country_filter_visible() {
  let summary = render_summary("pivottable_string_field_filter.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Order ID Country Sum - Amount");
  assert_page_contains(&summary, 0, "United States");
  assert_page_not_contains(&summary, 0, "United Kingdom");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableDateFieldFilterXLSX
fn mapped_xlsx_pivottable_date_field_filter_keeps_visible_date_members() {
  let summary = render_summary("pivottable_date_field_filter.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "Date Date2 Date3 Sum - Amount");
  assert_page_contains(&summary, 1, "2016/ January 7/");
  assert_page_contains(&summary, 2, "2016/ 1/ 8. 0:00");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableSharedGroupXLSX
fn mapped_xlsx_shared_group_field_keeps_group_labels_and_totals_visible() {
  let summary = render_summary("pivot-table/shared-group-field.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 0, "a2 Összeg / a Összeg / b");
  assert_page_contains(&summary, 0, "Csoport1 15 20 25");
  assert_page_contains(&summary, 0, "Total Result 171 189 207");
  assert_page_contains(&summary, 1, "Összeg / h Összeg / i");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableSharedDateGroupXLSX
fn mapped_xlsx_shared_dategroup_keeps_year_groups_visible() {
  let summary = render_summary("pivot-table/shared-dategroup.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 2, "a Összeg / c Összeg / d Összeg / e");
  assert_page_contains(&summary, 2, "1965 163877 212212 262738");
  assert_page_contains(&summary, 2, "Total Result 1113132 1301928 2042856");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableSharedNestedDateGroupXLSX
fn mapped_xlsx_shared_nested_dategroup_keeps_nested_year_quarter_months_visible() {
  let summary = render_summary("pivot-table/shared-nested-dategroup.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 2, "Row Labels Összeg / c Összeg / e");
  assert_page_contains(&summary, 2, "1965");
  assert_page_contains(&summary, 2, "Jan 53274 87176");
  assert_page_contains(&summary, 2, "Total Result 1113132 2042856");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableSharedNumGroupXLSX
fn mapped_xlsx_shared_numgroup_keeps_number_group_ranges_visible() {
  let summary = render_summary("pivot-table/shared-numgroup.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 2, "f Összeg / c Összeg / d Összeg / e");
  assert_page_contains(&summary, 2, "32674-47673 193380 194190 414100");
  assert_page_contains(&summary, 2, "Total Result 1113132 1301928 2042856");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableBoolFieldFilterXLSX
fn mapped_xlsx_pivottable_bool_field_filter_keeps_true_member_visible() {
  let summary = render_summary("pivottable_bool_field_filter.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Bool field Sum of Amount");
  assert_page_contains(&summary, 0, "TRUE");
  assert_page_not_contains(&summary, 0, "FALSE");
  assert_page_contains(&summary, 1, "FALSE");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableRowColPageFieldFilterXLSX
fn mapped_xlsx_pivottable_row_col_page_filter_keeps_filtered_fields_visible() {
  let summary = render_summary("pivottable_rowcolpage_field_filter.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "Double3 field - multiple -");
  assert_page_contains(&summary, 0, "Order ID 2");
  assert_page_contains(&summary, 0, "1 $4,270");
  assert_page_contains(&summary, 2, "Double3 field Double4 field");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableErrorItemFilterXLSX
fn mapped_xlsx_pivottable_error_item_filter_keeps_error_member_visible_in_source() {
  let summary = render_summary("pivottable_error_item_filter.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "a b b Sum of a");
  assert_page_contains(&summary, 0, "2 #DIV/0! Total Result 4");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testTdf125046
fn mapped_xlsx_pivottable_long_text_keeps_long_cache_text_visible() {
  let summary = render_summary("pivottable_long_text.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 0, "n (empty)");
  assert_page_contains(&summary, 1, "A very-very long");
  assert_page_contains(&summary, 1, "greater than wto hundred fifty five character");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testTdf125055
fn mapped_xlsx_pivottable_one_second_difference_keeps_distinct_time_members_visible() {
  let summary = render_summary("pivottable_1s_difference.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "n d n (empty)");
  assert_page_contains(&summary, 0, "a 7/10/2017 9:11");
  assert_page_contains(&summary, 0, "b 7/10/2017 9:11");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableXLSX_OutOfSyncPivotTableCachedDefinitionImport
fn mapped_xlsx_pivot_cached_definition_in_sync_keeps_visible_pivot_values() {
  let summary = render_summary("PivotTable_CachedDefinitionAndDataInSync.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "K Sum of A");
  assert_page_contains(&summary, 0, "1 5");
  assert_page_contains(&summary, 0, "Total Result 10");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableXLSX_OutOfSyncPivotTableCachedDefinitionImport2
fn mapped_xlsx_pivot_cached_definition_with_cache_data_keeps_visible_pivot_values() {
  let summary = render_summary(
    "PivotTable_CachedDefinitionAndDataNotInSync_SheetColumnsRemoved_WithCacheData.xlsx",
  );
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "K Sum of A");
  assert_page_contains(&summary, 0, "2 5");
  assert_page_contains(&summary, 1, "A K");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableXLSX_OutOfSyncPivotTableCachedDefinitionImport3
fn mapped_xlsx_pivot_cached_definition_without_cache_data_keeps_visible_pivot_values() {
  let summary = render_summary(
    "PivotTable_CachedDefinitionAndDataNotInSync_SheetColumnsRemoved_WithoutCacheData.xlsx",
  );
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "K Sum of A");
  assert_page_contains(&summary, 0, "Total Result 10");
  assert_page_contains(&summary, 1, "1 1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test6.cxx:testTableStyleCustomRoundtripXLSX
fn mapped_xlsx_book1_custom_keeps_custom_table_data_and_summary_visible() {
  let summary = render_summary("Book1_custom.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Names Numbers Dates Age");
  assert_page_contains(&summary, 0, "Summary 382");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf165180_date1904_XLSX
fn mapped_xlsx_tdf165180_date1904_keeps_early_mac_dates_visible() {
  let summary = render_summary("tdf165180_date1904.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Tuesday, March 1, 1904 Mar 1 1904");
  assert_page_contains(&summary, 0, "Monday, September 12, 2005 Sept 12 2005");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf122191
fn mapped_xlsx_tdf122191_keeps_hungarian_boolean_text_visible() {
  let summary = render_summary("tdf122191.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "IGAZ");
  assert_page_not_contains(&summary, 0, "BOOL00AN");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf123139XLSX
fn mapped_xlsx_tdf123139_apply_alignment_keeps_alignment_samples_visible() {
  let summary = render_summary("tdf123139_applyAlignment.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "foofoofoofoofoofoofoofoofoofoo");
  assert_page_contains(&summary, 0, "bar");
  assert_page_contains(&summary, 1, "hidden formula");
  assert_page_contains(&summary, 1, "unlocked distributed");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testShapeAutofitXLSX
fn mapped_xlsx_test_shape_autofit_keeps_shape_text_visible() {
  let summary = render_summary("testShapeAutofit.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "This one is autofit.");
  assert_page_contains(&summary, 0, "This one is not autofit.");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testCalcFieldNameErrorXLSX
fn mapped_xlsx_pivot_calcfield_nameerror_keeps_source_and_pivot_rows_visible() {
  let summary = render_summary("pivot-table/pivot_calcfield_nameerror.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "Bez Werte Hilfe + Hilfe -");
  assert_page_contains(&summary, 2, "Bez bezeichnung von deinen Pluswerten");
  assert_page_contains(&summary, 2, "Total Result 1300 -1678");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test.cxx:testTdf169326_ignoreLineBreaksInReferencedCells
fn mapped_xlsx_tdf169326_keeps_referenced_line_break_display_visible() {
  let summary = render_summary("tdf169326_ignore_line_breaks_in_referenced_cells.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Hello World Hello World");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testtdf120301_xmlSpaceParsingXLSX
fn mapped_xlsx_tdf120301_keeps_control_labels_with_xml_space_visible() {
  let summary = render_summary("tdf120301_xmlSpaceParsing.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Check Box 1");
  assert_page_contains(&summary, 0, "Option Button 2");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testFunctionsExcel2010XLSX
fn mapped_xlsx_functions_excel_2010_keeps_function_result_table_visible() {
  let summary = render_summary("functions-excel-2010.xlsx");
  assert_eq!(summary.page_count, 6);
  assert_page_contains(
    &summary,
    0,
    "Function Formula Result should be Equal? All equal?",
  );
  assert_page_contains(&summary, 0, "BETA.DIST");
  assert_page_contains(&summary, 0, "TRUE");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testCeilingFloorXLSX
fn mapped_xlsx_ceiling_floor_keeps_formula_check_values_visible() {
  let summary = render_summary("ceiling-floor.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 0, "23.5 -23.5");
  assert_page_contains(&summary, 0, "Err:502");
  assert_page_contains(&summary, 0, "#DIV/0!");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf126024XLSX
fn mapped_xlsx_hyperlink_formula_keeps_formula_link_text_visible() {
  let summary = render_summary("hyperlink_formula.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_text_occurrences(&summary, 0, "formula", 2);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf126177XLSX
fn mapped_xlsx_hyperlink_export_keeps_external_location_visible() {
  let summary = render_summary("hyperlink_export.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "C:\\TEMP\\test.xlsx#Munka1!A5");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf135828_Shape_Rect
fn mapped_xlsx_tdf135828_shape_rect_keeps_visible_shape_path() {
  let summary = render_summary("tdf135828_Shape_Rect.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf137000_handle_upright
fn mapped_xlsx_tdf137000_export_upright_keeps_textbox_text_visible() {
  let summary = render_summary("tdf137000_export_upright.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Simple Text");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTextBoxBodyUpright
fn mapped_xlsx_tdf106197_import_upright_keeps_textbox_text_visible() {
  let summary = render_summary("tdf106197_import_upright.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Simple Text");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testActiveXCheckboxXLSX
fn mapped_xlsx_activex_checkbox_keeps_custom_caption_visible() {
  let summary = render_summary("activex_checkbox.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Custom Caption");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testCellAnchoredHiddenShapesXLSX
fn mapped_xlsx_cell_anchored_hidden_shapes_keeps_visible_sheet_text() {
  let summary = render_summary("cell-anchored-hidden-shapes.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "TAIWAN PROMOTION AIRFARES");
  assert_page_contains(&summary, 0, "BUSINESS CLASS");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testShapeRotationImport
fn mapped_xlsx_shape_rotation_import_keeps_rotated_shape_text_visible() {
  let summary = render_summary("testShapeRotationImport.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "jo");
  assert_page_contains(&summary, 0, "bb");
  assert_page_contains(&summary, 0, "ra");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf134455
fn mapped_xlsx_tdf134455_keeps_timevalue_results_visible() {
  let summary = render_summary("tdf134455.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "00:05");
  assert_page_contains(&summary, 0, "01:05");
  assert_page_contains(&summary, 0, "04:00");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf131424
fn mapped_xlsx_tdf131424_keeps_structured_reference_sums_visible() {
  let summary = render_summary("tdf131424.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "This is the first column");
  assert_page_contains(&summary, 0, "12 23 35");
  assert_page_contains(&summary, 0, "36 45 81");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test5.cxx:testTdf122336
fn mapped_xlsx_tdf122336_keeps_imported_date_and_job_fields_visible() {
  let summary = render_summary("tdf122336.xlsx");
  assert_eq!(summary.page_count, 9);
  assert_page_contains(&summary, 0, "UitvoeringsdatumStarttijd");
  assert_page_contains(&summary, 0, "12/25/2018 11:30");
  assert_page_contains(&summary, 2, "Van Rompaey Marcus");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:shared-formula 3D reference assertions
fn mapped_xlsx_shared_formula_3d_reference_keeps_same_and_another_sheet_values_visible() {
  let summary = render_summary("shared-formula/3d-reference.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(
    &summary,
    0,
    "Value Same sheet Another sheet Same sheet but sheet name shown",
  );
  assert_page_contains(&summary, 0, "1 1 10 1");
  assert_page_contains(&summary, 1, "10 20 30 40 50 60");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testSharedFormulaXLSX
fn mapped_xlsx_shared_formula_basic_keeps_formula_values_visible() {
  let summary = render_summary("shared-formula/basic.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Value Formula");
  assert_page_contains(&summary, 0, "1 10");
  assert_page_contains(&summary, 0, "18 180");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testSharedFormulaStringResultExportXLSX
fn mapped_xlsx_shared_formula_text_results_keep_cached_strings_visible() {
  let summary = render_summary("shared-formula/text-results.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Text Same Sheet Another Sheet");
  assert_page_contains(&summary, 0, "A A AA");
  assert_page_contains(&summary, 0, "F F FF");
  assert_page_contains(&summary, 1, "AA BB CC DD EE FF");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testExternalRefCacheXLSX
fn mapped_xlsx_external_refs_keep_cached_external_values_visible() {
  let summary = render_summary("external-refs.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Name Andy Bruce Charlie");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testRefStringXLSX
fn mapped_xlsx_ref_string_keeps_cached_value_visible() {
  let summary = render_summary("ref_string.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "1 2 3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testTdf139934
fn mapped_xlsx_tdf139934_keeps_imported_date_strings_visible() {
  let summary = render_summary("tdf139934.xlsx");
  assert_eq!(summary.page_count, 9);
  assert_page_contains(&summary, 0, "Absence Requests");
  assert_page_contains(&summary, 0, "1/20/2021 Wednesday Annual Leave");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testNonAsciiWithDotXLSX
fn mapped_xlsx_tdf100154_keeps_non_ascii_sheet_formula_value_visible() {
  let summary = render_summary("tdf100154.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "5");
  assert_page_contains(&summary, 1, "5");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf160371
fn mapped_xlsx_tdf160371_keeps_indirect_intersection_value_visible() {
  let summary = render_summary("tdf160371.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Intersection Example Data Table");
  assert_page_contains(&summary, 0, "Value 1 Row_2 4 5 6");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf100709XLSX
fn mapped_xlsx_tdf100709_keeps_plain_218_values_visible() {
  let summary = render_summary("tdf100709.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "65 218");
  assert_page_contains(&summary, 0, "05-Mar-00 218");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf105272
fn mapped_xlsx_tdf105272_keeps_structured_reference_table_visible() {
  let summary = render_summary("tdf105272.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Gold Silver Bronze Total");
  assert_page_contains(&summary, 0, "13 11 9 33");
  assert_page_contains(&summary, 0, "232 0.14");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testTdf119292
fn mapped_xlsx_tdf119292_keeps_rotated_text_samples_visible() {
  let summary = render_summary("tdf119292.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "text rotated by 270 degrees");
  assert_page_contains(&summary, 0, "text rotated by 90 degrees");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf131536
fn mapped_xlsx_tdf131536_keeps_comparison_formula_display_visible() {
  let summary = render_summary("tdf131536.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Excel - true Calc -true");
  assert_page_contains(&summary, 0, "L0001 L0001");
  assert_page_contains(&summary, 1, "MMR/HEV/LIC/000001");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf137543XLSX
fn mapped_xlsx_tdf137543_keeps_let_function_results_visible() {
  let summary = render_summary("tdf137543.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "25");
  assert_page_contains(&summary, 0, "Source array");
  assert_page_contains(&summary, 1, "A B E F I J M N Q R U V");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf170201_empty_values_in_array_formulas
fn mapped_xlsx_tdf170201_keeps_array_formula_non_empty_values_visible() {
  let summary = render_summary("tdf170201.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Sheet1");
  assert_page_contains(&summary, 0, "1 3");
}

#[test]
// Source: ../core/sc/qa/unit/jumbosheets-test.cxx:testTdf134553
fn mapped_xlsx_tdf134553_keeps_chart_labels_visible() {
  let summary = render_summary("tdf134553.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Chart Title");
  assert_page_contains(&summary, 0, "First data point; 2");
  assert_page_contains(&summary, 0, "Third data point; 8");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf142929_filterLessThanXLSX
fn mapped_xlsx_tdf142929_filter_less_than_keeps_filtered_number_visible() {
  let summary = render_summary("tdf142929.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Numbers");
  assert_page_contains(&summary, 0, "1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testAutofilterTop10XLSX
fn mapped_xlsx_tdf143068_top10_filter_keeps_top_four_numbers_visible() {
  let summary = render_summary("tdf143068_top10filter.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Numbers");
  assert_page_contains(&summary, 0, "7 8 9 10");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testExternalDefinedNameXLSX
fn mapped_xlsx_tdf144397_keeps_external_defined_name_results_visible() {
  let summary = render_summary("tdf144397.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Strings With Range name:");
  assert_page_contains(&summary, 0, "January January");
  assert_page_contains(&summary, 0, "May #N/A");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test.cxx:testTdf162963
fn mapped_xlsx_tdf162963_keeps_table_totals_row_visible() {
  let summary = render_summary("tdf162963_TableWithTotalsEnabled.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Name Sales");
  assert_page_contains(&summary, 0, "Miller 23");
  assert_page_contains(&summary, 0, "All 115");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testAutofilterXLSX
fn mapped_xlsx_autofilter_keeps_filtered_area_rows_visible() {
  let summary = render_summary("autofilter.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "column1 column2 column3");
  assert_page_contains(&summary, 0, "2 3 4");
  assert_page_contains(&summary, 0, "4 5 4");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testNamedTableRef
fn mapped_xlsx_tablerefsnamed_keeps_named_table_formula_results_visible() {
  let summary = render_summary("tablerefsnamed.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Name Score ScoreNames");
  assert_page_contains(&summary, 0, "aaa 3.5 ScoreNames aaa TRUE");
  assert_page_contains(&summary, 0, "fff 4.1 ScoreNames fff TRUE");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testDatabaseRangesXLSX
fn mapped_xlsx_database_ranges_keep_named_and_unnamed_results_visible() {
  let summary = render_summary("database.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Col1 Col2 Col3 Col4");
  assert_page_contains(&summary, 1, "Using named db range Using unnamed db range");
  assert_page_contains(&summary, 1, "Name Age Children");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testMatrixMultiplicationXLSX
fn mapped_xlsx_matrix_multiplication_keeps_array_formula_results_visible() {
  let summary = render_summary("matrix-multiplication.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "1 2 4 5.2");
  assert_page_contains(&summary, 0, "49.2");
  assert_page_contains(&summary, 0, "103.6");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf118990
fn mapped_xlsx_tdf118990_keeps_external_vlookup_results_visible() {
  let summary = render_summary("tdf118990.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "333 C");
  assert_page_text_occurrences(&summary, 0, "333", 3);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf163554
fn mapped_xlsx_tdf163554_keeps_quoted_3d_range_sum_visible() {
  let summary = render_summary("tdf163554.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "time (misc) - last");
  assert_page_contains(&summary, 0, "7 7");
  assert_page_contains(&summary, 1, "time (pnrst)");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf85617
fn mapped_xlsx_tdf85617_keeps_sheet_local_name_result_visible() {
  let summary = render_summary("tdf85617.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Code name Quantity Price");
  assert_page_contains(&summary, 0, "Товар 1 1 4.5");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test5.cxx:testTdf118668
fn mapped_xlsx_tdf118668_prints_visible_second_sheet() {
  let summary = render_summary("tdf118668.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "ПУТЕВОЙ ЛИСТ ТРАКТОРА");
  assert_page_contains(&summary, 1, "Результат работы автомобиля за смену");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableOutlineModeXLSX
fn mapped_xlsx_pivottable_outline_mode_keeps_outline_pivot_visible() {
  let summary = render_summary("pivottable_outline_mode.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "field1 field2 field3");
  assert_page_contains(&summary, 0, "Sum of field3");
  assert_page_contains(&summary, 0, "Total Result 6");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableTabularModeXLSX
fn mapped_xlsx_pivottable_tabular_mode_keeps_tabular_pivot_visible() {
  let summary = render_summary("pivottable_tabular_mode.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 0, "pwdLastSet (empty)");
  assert_page_contains(&summary, 0, "company employeeID Count - mail");
  assert_page_contains(&summary, 0, "Total Result 13");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testTdf124810_pivotDark
fn mapped_xlsx_pivot_dark1_keeps_dark_style_pivot_output_visible() {
  let summary = render_summary("pivot_dark1.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "name date value");
  assert_page_contains(&summary, 0, "Count of v date");
  assert_page_contains(&summary, 1, "Total Result");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testInvalidFormats
fn mapped_xlsx_pivottable_invalid_formats_keeps_formatted_source_rows_visible() {
  let summary = render_summary("pivottable_invalid_formats.xlsx");
  assert_eq!(summary.page_count, 5);
  assert_page_contains(&summary, 0, "Spalte 1 Spalte 2 Spalte 3");
  assert_page_contains(&summary, 0, "A Ur 600 0.65");
  assert_page_contains(&summary, 1, "H Th 1144 149.89");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableExportXLSX
fn mapped_xlsx_tdf89139_pivot_table_keeps_hidden_item_pivot_output_visible() {
  let summary = render_summary("tdf89139_pivot_table.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Name Id Dept Date");
  assert_page_contains(&summary, 1, "Dept Count of Id");
  assert_page_contains(&summary, 1, "Total Result 17");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf165503
fn mapped_xlsx_tdf165503_keeps_chart_date_cache_source_values_visible() {
  let summary = render_summary("tdf165503.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Date Val");
  assert_page_contains(&summary, 0, "1/3/2021 3");
  assert_page_contains(&summary, 0, "10/10/2022 9");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf137091
fn mapped_xlsx_tdf137091_keeps_turkish_locale_formula_result_visible() {
  let summary = render_summary("tdf137091.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "SG STOKKODU Sütun2");
  assert_page_contains(&summary, 0, "IP 1 B J7 0280 4 4 08 YY MR0084 28/4");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf70455
fn mapped_xlsx_tdf70455_keeps_bet_history_returns_visible() {
  let summary = render_summary("tdf70455.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Bet History");
  assert_page_contains(&summary, 0, "Gross: €780.00");
  assert_page_contains(&summary, 0, "€130.00 No 7/1 8.0000 Win €1,040.00 €910.00");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf98481
fn mapped_xlsx_tdf98481_keeps_cross_sheet_sum_results_visible() {
  let summary = render_summary("tdf98481.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(
    &summary,
    0,
    "Horizontal and vertical Sums with source as separate sheet.",
  );
  assert_page_contains(&summary, 0, "Sum 4 0 3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf115022
fn mapped_xlsx_tdf115022_keeps_recalculated_sum_visible() {
  let summary = render_summary("tdf115022.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Index amount");
  assert_page_contains(&summary, 0, "a 1 a 2 a 3");
  assert_page_contains(&summary, 0, "6");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf164895
fn mapped_xlsx_tdf164895_keeps_horizontal_range_operator_result_visible() {
  let summary = render_summary("tdf164895.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "a 1");
  assert_page_contains(&summary, 0, "8 30 5");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testTdf162093
fn mapped_xlsx_tdf162093_keeps_structured_reference_outputs_visible() {
  let summary = render_summary("tdf162093.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Surname Count Region Surname Count Region");
  assert_page_contains(&summary, 0, "Murray 15 North Murray 15 North");
  assert_page_contains(&summary, 0, "Total 296 Total 296");
  assert_page_contains(&summary, 1, "{=myData[#Headers]} =myData[#Headers]");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testTdf147955
fn mapped_xlsx_tdf147955_keeps_profit_and_loss_totals_visible() {
  let summary = render_summary("tdf147955.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Leasingham Community Benefit Society Ltd");
  assert_page_contains(&summary, 0, "Sales 892.75");
  assert_page_contains(&summary, 0, "Food - CoS 130.25");
  assert_page_contains(&summary, 0, "Cleaning 10.98");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testTdf155046
fn mapped_xlsx_tdf155046_keeps_boolean_publication_results_visible() {
  let summary = render_summary("tdf155046.xlsx");
  assert_eq!(summary.page_count, 5);
  assert_page_contains(
    &summary,
    0,
    "Respondent ID Publication ID Submitted Submitted Time",
  );
  assert_page_contains(&summary, 0, "89nre8cuc7i3 69lue27dr864 TRUE");
  assert_page_contains(&summary, 1, "Publication Consent");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf136364
fn mapped_xlsx_tdf136364_keeps_union_formula_results_visible() {
  let summary = render_summary("tdf136364.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "1 1 1 1 27");
  assert_page_contains(&summary, 0, "2 2 2 2 12");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test5.cxx:testTdf157689
fn mapped_xlsx_tdf157689_keeps_multisheet_autofilter_output_visible() {
  let summary = render_summary("tdf157689.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "col1 col2");
  assert_page_contains(&summary, 0, "1 2 2 3");
  assert_page_contains(&summary, 1, "col3 col4");
  assert_page_contains(&summary, 1, "2 2 4 2");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testTdf142905
fn mapped_xlsx_tdf142905_keeps_space_padded_text_visible() {
  let summary = render_summary("tdf142905.xlsx");
  assert_eq!(summary.page_count, 1);
  // PDFium collapses the positioned spaces in this Calc export. LibreOffice's
  // PDF and the local PDF preserve them in layout extraction, matching
  // sc/qa/unit/subsequent_filters_test4.cxx:testTdf142905.
  let text = summary.text.as_deref().unwrap_or("");
  assert!(
    text.contains("3M   3M"),
    "missing space-padded text pair; page text:\n{text}"
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf119190
fn mapped_xlsx_tdf119190_keeps_visible_comment_caption() {
  let summary = render_summary("tdf119190.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Kelemen Gábor 2:");
  assert_page_contains(&summary, 0, "Comment!");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf141495
fn mapped_xlsx_tdf141495_keeps_turkish_locale_date_serial_visible() {
  let summary = render_summary("tdf141495.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "44227 44255 44286 44316");
  assert_page_contains(&summary, 1, "44804 44834 44865 44895 44926");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf79972XLSX
fn mapped_xlsx_tdf79972_keeps_bugzilla_hyperlink_visible() {
  let summary = render_summary("tdf79972.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "123");
  assert_link_target(
    &summary,
    "https://bugs.documentfoundation.org/show_bug.cgi?id=79972",
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test.cxx:testTdf111876
fn mapped_xlsx_tdf111876_keeps_relative_hyperlink_text_visible() {
  let summary = render_summary("tdf111876.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "..\\xls\\bug-fixes.xls");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf119565
fn mapped_xlsx_tdf119565_keeps_themed_textbox_text_visible() {
  let summary = render_summary("tdf119565.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Lorem ipsum dolor");
  assert_page_contains(&summary, 0, "Maecenas porttitor");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testSheetCharacterKerningSpaceXLSX
fn mapped_xlsx_textbox_char_kerning_space_keeps_textbox_text_visible() {
  let summary = render_summary("textbox-CharKerningSpace.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "AVAIL");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test3.cxx:testSheetCondensedCharacterSpaceXLSX
fn mapped_xlsx_textbox_condensed_character_space_keeps_textbox_text_visible() {
  let summary = render_summary("textbox-CondensedCharacterSpace.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "AvaiL");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTextBoxBodyRotateAngle
fn mapped_xlsx_tdf141644_keeps_rotated_textbox_body_text_visible() {
  let summary = render_summary("tdf141644.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Textdir: 270 deg");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf142881
fn mapped_xlsx_tdf142881_keeps_rotated_shape_labels_visible() {
  let summary = render_summary("tdf142881.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Rotated:");
  assert_page_contains(&summary, 0, "35");
  assert_page_contains(&summary, 1, "25");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf145057
fn mapped_xlsx_tdf145057_keeps_color_filtered_row_visible() {
  let summary = render_summary("tdf145057.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Numbers Names abc def fgh");
  assert_page_contains(&summary, 0, "4.00 s 3 4 1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf161365
fn mapped_xlsx_tdf161365_keeps_checkbox_fixture_text_visible() {
  let summary = render_summary("tdf161365.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(
    &summary,
    0,
    "This spreadsheet contains checkbox which dissapeared after re-export",
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf115192XLSX
fn mapped_xlsx_tdf115192_keeps_drawing_hyperlink_labels_visible() {
  let summary = render_summary("test_115192.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Hyperlink: test.xlxs");
  assert_page_contains(&summary, 0, "Hyperlink: Sheet2!A1");
  assert_page_contains(&summary, 0, "Hyperlink: Bug 115192");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:many-fields-in-cache import check
fn mapped_xlsx_pivot_many_fields_in_cache_keeps_pivot_result_visible() {
  let summary = render_summary("pivot-table/many-fields-in-cache.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "F1 F2 F3 F4 F5 F6 F7 F8");
  assert_page_contains(&summary, 0, "Sum of F10 F4");
  assert_page_contains(&summary, 0, "Total Result 5 6 11");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testPivotTableDuplicatedMemberFilterXLSX
fn mapped_xlsx_pivottable_duplicated_member_filter_keeps_page_field_pivot_visible() {
  let summary = render_summary("pivottable_duplicated_member_filter.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "pwdLastSet (empty)");
  assert_page_contains(&summary, 0, "First type 4");
  assert_page_contains(&summary, 0, "Second type 9");
  assert_page_contains(&summary, 0, "Total Result 13");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTdf104310_x14
fn mapped_xlsx_tdf104310_keeps_x14_validation_list_values_visible() {
  let summary = render_summary("tdf104310.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "1 2 3 4 5");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test3.cxx:testTextLengthDataValidityXLSX
fn mapped_xlsx_text_length_data_validity_keeps_checked_values_visible() {
  let summary = render_summary("textLengthDataValidity.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "1234");
  assert_page_contains(&summary, 0, "1234.00");
  assert_page_contains(&summary, 0, "12.3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf120168
fn mapped_xlsx_tdf120168_keeps_alignment_sample_text_visible() {
  let summary = render_summary("tdf120168.xlsx");
  assert_eq!(summary.page_count, 1);
  // Upstream verifies that rotated cells with implicit alignment round-trip as
  // left/right. PDFium drops the isolated rotated "1"/"2" segments here, while
  // Poppler sees the same visible text that LibreOffice exports.
  let text = normalize_space(
    summary
      .text
      .as_deref()
      .unwrap_or_else(|| panic!("missing pdftotext output: {:?}", summary.text_error)),
  );
  assert!(
    text.contains("1 2"),
    "missing rotated digits; text:\n{text}"
  );
  assert!(
    text.contains("Te xt"),
    "missing rotated sample text; text:\n{text}"
  );
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf129985
fn mapped_xlsx_tdf129985_keeps_custom_date_format_visible() {
  let summary = render_summary("tdf129985.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Érvényesség kezdete");
  assert_page_text_occurrences(&summary, 0, "1/13/2020", 2);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf73063
fn mapped_xlsx_tdf73063_keeps_localized_long_date_visible() {
  let summary = render_summary("tdf73063.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Saturday, 17. June 1972");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test.cxx:testExtCondFormatXLSX
fn mapped_xlsx_tdf139021_keeps_ext_cond_format_sample_text_visible() {
  let summary = render_summary("tdf139021.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "hello hello");
  assert_page_contains(&summary, 0, "hello bye");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test.cxx:testTdf139394
fn mapped_xlsx_tdf139394_keeps_conditional_format_marker_text_visible() {
  let summary = render_summary("tdf139394.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_text_occurrences(&summary, 0, "+", 3);
  assert_page_text_occurrences(&summary, 0, "-", 1);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf55417
fn mapped_xlsx_tdf55417_keeps_alignment_style_sample_value_visible() {
  let summary = render_summary("tdf55417.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "29");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test.cxx:testTdf139167
fn mapped_xlsx_tdf139167_keeps_conditional_style_sample_text_visible() {
  let summary = render_summary("tdf139167.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Hello");
}

#[test]
// Source: ../core/sc/qa/unit/jumbosheets-test.cxx:testTdf109061
fn mapped_xlsx_tdf109061_keeps_recalculated_sum_visible() {
  let summary = render_summary("tdf109061.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Test");
  assert_page_contains(&summary, 0, "1 2 3");
  assert_page_contains(&summary, 0, "Sum: 6");
}

#[test]
// Source: ../core/sc/qa/unit/pivottable_filters_test.cxx:testTdf112106
fn mapped_xlsx_tdf112106_keeps_pivot_data_layout_values_visible() {
  let summary = render_summary("tdf112106.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Country - all -");
  assert_page_contains(&summary, 0, "Banana $617");
  assert_page_contains(&summary, 0, "Total Result $13,126");
  assert_page_contains(&summary, 1, "Order ID Product Category Amount Date Country");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test4.cxx:testTdf155402
fn mapped_xlsx_tdf155402_keeps_filename_cell_result_visible() {
  let summary = render_summary("tdf155402.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "[tdf155402.xlsx]Sheet1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test4.cxx:testTdf91251_missingOverflowRoundtrip
fn mapped_xlsx_tdf91251_keeps_textbox_overflow_sample_text_visible() {
  let summary = render_summary("tdf91251_missingOverflowRoundtrip.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Text Box");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf164417
fn mapped_xlsx_tdf164417_keeps_autofilter_date_rows_visible() {
  let summary = render_summary("tdf164417.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Num Text Date");
  // Upstream verifies that both filtered date rows survive export. PDFium
  // segments the adjacent A/B cell text with a space, unlike LO's pdftotext
  // output that may join the positioned glyphs as "1a"/"2a".
  assert_page_contains(&summary, 0, "1 a 31/12/24");
  assert_page_contains(&summary, 0, "2 a 31/12/2024 (text)");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf165886
fn mapped_xlsx_tdf165886_keeps_formula_quote_results_visible() {
  let summary = render_summary("tdf165886.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "0 #NAME? TRUE");
  assert_page_text_occurrences(&summary, 0, "#NAME?", 8);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf166413
fn mapped_xlsx_tdf166413_keeps_conditional_formula_quote_samples_visible() {
  let summary = render_summary("tdf166413.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_text_occurrences(&summary, 0, r#"test ABC "ABC""#, 4);
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testTdf95640_xlsx_to_xlsx
fn mapped_xlsx_tdf95640_keeps_standard_filter_rows_visible() {
  let summary = render_summary("tdf95640.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "AAA BBB");
  assert_page_contains(&summary, 0, "jan 2");
  assert_page_contains(&summary, 0, "feb 1");
  assert_page_contains(&summary, 0, "mar 3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test2.cxx:testTdf97598XLSX
fn mapped_xlsx_tdf97598_keeps_scenario_cell_text_visible() {
  let summary = render_summary("tdf97598_scenarios.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Cell A1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testEscapeCharInNumberFormatXLSX
fn mapped_xlsx_tdf81939_keeps_escaped_number_formats_visible() {
  let summary = render_summary("tdf81939.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "01 23 45 67 89");
  assert_page_contains(&summary, 0, "01.23.45.678.9");
  assert_page_contains(&summary, 0, "123,456,789 €");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testContentXLSXStrict
fn mapped_xlsx_universal_content_strict_keeps_core_values_visible() {
  let summary = render_summary("universal-content-strict.xlsx");
  assert_eq!(summary.page_count, 4);
  assert_page_contains(&summary, 0, "1 String1 6");
  assert_page_contains(&summary, 0, "2 String2 5");
  assert_page_contains(&summary, 0, "-1 11");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_filters_test.cxx:testRowIndex1BasedXLSX
fn mapped_xlsx_row_index_1_based_keeps_first_row_and_multiline_text_visible() {
  let summary = render_summary("row-index-1-based.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Action Plan.Name Action Plan.Description");
  assert_page_contains(&summary, 0, "Jerry This is a longer Text.");
  assert_page_contains(&summary, 0, "Second line.");
  assert_page_contains(&summary, 0, "Third line.");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot1_Row
fn mapped_xlsx_pivot1_row_keeps_row_field_items_visible() {
  let summary = render_summary("pivot/Pivot1_Row.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Name Type Sum of Price");
  assert_page_contains(&summary, 1, "X1 A 100");
  assert_page_contains(&summary, 1, "X5 A 50");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot1_Row_Grand
fn mapped_xlsx_pivot1_row_grand_keeps_grand_total_visible() {
  let summary = render_summary("pivot/Pivot1_Row_Grand.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Name Type Sum of Price");
  assert_page_contains(&summary, 1, "Total Result 600");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot1_Row_Grand_Subtotals
fn mapped_xlsx_pivot1_row_grand_subtotals_keeps_item_subtotals_visible() {
  let summary = render_summary("pivot/Pivot1_Row_Grand_Subtotals.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "X1 Result 100");
  assert_page_contains(&summary, 1, "X5 Result 50");
  assert_page_contains(&summary, 1, "Total Result 600");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot2_Row
fn mapped_xlsx_pivot2_row_keeps_nested_row_items_visible() {
  let summary = render_summary("pivot/Pivot2_Row.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Type Name Sum of Price");
  assert_page_contains(&summary, 1, "A X1 100");
  assert_page_contains(&summary, 1, "B X2 200");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot2_Row_Compact
fn mapped_xlsx_pivot2_row_compact_keeps_compact_labels_visible() {
  let summary = render_summary("pivot/Pivot2_Row_Compact.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Row Labels Sum of Price");
  assert_page_contains(&summary, 1, "X1 100");
  assert_page_contains(&summary, 1, "X4 100");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot2_Row_Grand
fn mapped_xlsx_pivot2_row_grand_keeps_nested_grand_total_visible() {
  let summary = render_summary("pivot/Pivot2_Row_Grand.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Type Name Sum of Price");
  assert_page_contains(&summary, 1, "Total Result 600");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot2_Row_Grand_Subtotals
fn mapped_xlsx_pivot2_row_grand_subtotals_keeps_group_subtotals_visible() {
  let summary = render_summary("pivot/Pivot2_Row_Grand_Subtotals.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "A Result 150");
  assert_page_contains(&summary, 1, "B Result 350");
  assert_page_contains(&summary, 1, "Total Result 600");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot2_Row_Subtotals
fn mapped_xlsx_pivot2_row_subtotals_keeps_group_subtotals_without_grand_total() {
  let summary = render_summary("pivot/Pivot2_Row_Subtotals.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "A Result 150");
  assert_page_contains(&summary, 1, "B Result 350");
  assert_page_contains(&summary, 1, "C Result 100");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot2_Row_Subtotals_SortDescendingAll
fn mapped_xlsx_pivot2_row_subtotals_sort_descending_keeps_sorted_groups_visible() {
  let summary = render_summary("pivot/Pivot2_Row_Subtotals_SortDescendingAll.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "C X4 100");
  assert_page_contains(&summary, 1, "B X3 150");
  assert_page_contains(&summary, 1, "A X5 50");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot3_Column_Grand_Subtotals
fn mapped_xlsx_pivot3_column_grand_subtotals_keeps_column_totals_visible() {
  let summary = render_summary("pivot/Pivot3_Column_Grand_Subtotals.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Type Name");
  assert_page_contains(&summary, 1, "Total Result");
  assert_page_contains(&summary, 1, "100 50 150 200 150 350 100 100 600");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTable_FieldsAndItemsExport.cxx:Pivot4_Column_Grand_Subtotals_SortDescending
fn mapped_xlsx_pivot4_column_grand_subtotals_sort_descending_keeps_column_totals_visible() {
  let summary = render_summary("pivot/Pivot4_Column_Grand_Subtotals_SortDescending.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Type Name");
  assert_page_contains(&summary, 1, "Total Result");
  assert_page_contains(&summary, 1, "100 100 200 150 350 100 50### 600");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTableFormatsImportExport.cxx:PivotTableCellFormatsTest_1_DataFieldInRow_RowLabelColor
fn mapped_xlsx_pivot_format_data_field_in_row_keeps_values_visible() {
  let summary =
    render_summary("pivot-table/PivotTableCellFormatsTest_1_DataFieldInRow_RowLabelColor.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Values 1 2 3");
  assert_page_contains(&summary, 0, "Sum of C 3 3 4");
  assert_page_contains(&summary, 0, "Sum of D 7 2 6");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTableFormatsImportExport.cxx:PivotTableCellFormatsTest_4_DataFieldInColumn_DataColor
fn mapped_xlsx_pivot_format_data_field_in_column_keeps_totals_visible() {
  let summary =
    render_summary("pivot-table/PivotTableCellFormatsTest_4_DataFieldInColumn_DataColor.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "A Sum of C Sum of D");
  assert_page_contains(&summary, 0, "Total Result 15 25");
  assert_page_contains(&summary, 0, "Grand Total 15 25");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTableFormatsImportExport.cxx:PivotTableCellFormatsTest_8_DataFieldInRow_DataColor
fn mapped_xlsx_pivot_format_data_field_row_data_color_keeps_values_visible() {
  let summary =
    render_summary("pivot-table/PivotTableCellFormatsTest_8_DataFieldInRow_DataColor.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Data 1 2");
  assert_page_contains(&summary, 0, "Sum of C 3 3");
  assert_page_contains(&summary, 0, "Sum of D 7 2");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTableFormatsImportExport.cxx:PivotTableCellFormatsTest_9_MultipleSelections
fn mapped_xlsx_pivot_format_multiple_selections_keeps_totals_visible() {
  let summary = render_summary("pivot-table/PivotTableCellFormatsTest_9_MultipleSelections.xlsx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "A Sum of C Sum of D");
  assert_page_contains(&summary, 0, "Total Result 15 25");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTableFormatsImportExport.cxx:PivotTableCellFormatsTest_11_WholeDataColumnSelected
fn mapped_xlsx_pivot_format_whole_data_column_keeps_totals_visible() {
  let summary =
    render_summary("pivot-table/PivotTableCellFormatsTest_11_WholeDataColumnSelected.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "A Sum of B");
  assert_page_contains(&summary, 0, "Total Result 60");
  assert_page_contains(&summary, 0, "Grand Total 60");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTableFormatsImportExport.cxx:PivotTableCellFormatsTest_12_WholeLabelColumnSelected
fn mapped_xlsx_pivot_format_whole_label_column_keeps_totals_visible() {
  let summary =
    render_summary("pivot-table/PivotTableCellFormatsTest_12_WholeLabelColumnSelected.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "A Sum of B");
  assert_page_contains(&summary, 0, "Total Result 60");
  assert_page_contains(&summary, 0, "Grand Total 60");
}

#[test]
// Source: ../core/sc/qa/unit/PivotTableFormatsImportExport.cxx:PivotTableCellFormatsTest_13_SelectionInLabelAndData
fn mapped_xlsx_pivot_format_label_and_data_selection_keeps_values_visible() {
  let summary =
    render_summary("pivot-table/PivotTableCellFormatsTest_13_SelectionInLabelAndData.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "A B C A Sum of B");
  assert_page_contains(&summary, 0, "a 1 15 a 5");
  assert_page_contains(&summary, 0, "f 1 10 f 5");
}

#[test]
// Source: ../core/sc/qa/unit/cond_format.cxx:testConditionalFormatExportXLSX
fn mapped_xlsx_new_cond_format_export_keeps_rule_sample_table_visible() {
  let summary = render_summary("new_cond_format_test_export.xlsx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(
    &summary,
    0,
    "top n elements bottom n elements top n percent bottom n percent above average",
  );
  assert_page_contains(
    &summary,
    1,
    "below average above equal average below equal average",
  );
  assert_page_contains(&summary, 1, "1.00 2 2.00");
}
