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
// Source: ../core/sc/qa/unit/subsequent_filters_test5.cxx:testTableStyleXLSX
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
  assert_page_contains(&summary, 0, "BackgroundForeground Both");
  assert_page_contains(&summary, 0, "2 2 2");
  assert_page_contains(&summary, 0, "3 3 3");
  assert_page_not_contains(&summary, 0, "1 1 1");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test2.cxx:testAutofilterColorsOOXML2
fn mapped_xlsx_autofilter_colors_fg_keeps_foreground_filtered_rows() {
  let summary = render_summary("autofilter-colors-fg.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "BackgroundForeground Both");
  assert_page_contains(&summary, 0, "1 1 1");
  assert_page_contains(&summary, 0, "2 2 2");
  assert_page_contains(&summary, 0, "3 3 3");
}

#[test]
// Source: ../core/sc/qa/unit/subsequent_export_test5.cxx:testTdf130104_XLSXIndent
fn mapped_xlsx_tdf130104_keeps_imported_indent_positions_visible() {
  let summary = render_summary("tdf130104_indent.xlsx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Indent by 0 缩进0");
  assert_page_contains(&summary, 0, "Indent by 5 缩进5");
  assert_page_contains(&summary, 0, "Indent by 10 缩进10");
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
