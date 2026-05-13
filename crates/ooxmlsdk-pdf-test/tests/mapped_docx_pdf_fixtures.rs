use ooxmlsdk_pdf_test::{PdfSummary, pdf_summary_for_fixture, pdfexport_fixture_dir};

fn fixture(name: &str) -> std::path::PathBuf {
  pdfexport_fixture_dir().join(name)
}

fn render_summary(name: &str) -> PdfSummary {
  pdf_summary_for_fixture(&fixture(name)).unwrap()
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

fn normalized_page_text(summary: &PdfSummary, page_index: usize) -> String {
  normalize_space(&page_text(summary, page_index))
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

fn assert_page_starts_with(summary: &PdfSummary, page_index: usize, expected: &str) {
  let text = page_text(summary, page_index);
  let normalized_text = normalize_space(&text);
  let normalized_expected = normalize_space(expected);
  assert!(
    normalized_text.starts_with(&normalized_expected),
    "page {page_index} does not start with {expected:?}; page text:\n{text}"
  );
}

fn normalized_occurrences(text: &str, expected: &str) -> usize {
  text.match_indices(&normalize_space(expected)).count()
}

fn page_image_count(summary: &PdfSummary, page_index: usize) -> usize {
  summary
    .images
    .iter()
    .filter(|image| image.page_index == page_index)
    .count()
}

fn assert_page_image_count(summary: &PdfSummary, page_index: usize, expected: usize) {
  assert_eq!(
    page_image_count(summary, page_index),
    expected,
    "images={:?}",
    summary.images
  );
}

fn assert_text_object_fill_color(summary: &PdfSummary, text: &str, color: &str) {
  let objects = summary
    .text_objects
    .iter()
    .filter(|object| normalize_space(&object.text).contains(text))
    .collect::<Vec<_>>();
  assert!(
    !objects.is_empty(),
    "missing text object containing {text:?}"
  );
  assert!(
    objects
      .iter()
      .any(|object| object.fill_color.as_deref() == Some(color)),
    "missing fill color {color:?} for text {text:?}; objects={objects:?}"
  );
}

fn normalize_space(text: &str) -> String {
  text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
// Source: ../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:testFirstPageHeadersAndEmptyFooters
fn mapped_fixture_fdo66145_preserves_first_and_rest_page_headers() {
  let summary = render_summary("fdo66145.docx");
  assert_page_contains(&summary, 0, "This is the FIRST page header.");
  assert_page_contains(&summary, 1, "This is the header for the REST OF THE FILE.");
  assert_page_contains(&summary, 2, "This is the header for the REST OF THE FILE.");
}

#[test]
// Source: ../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:testFirstHeaderFooterImport
fn mapped_fixture_first_header_footer_preserves_section_header_footer_text() {
  let summary = render_summary("first-header-footer.docx");
  assert_eq!(summary.page_count, 6);

  let expected = [
    ("First page header", "First page footer"),
    ("Even page header", "Even page footer"),
    ("Odd page header", "Odd page footer"),
    ("First page header2", "First page footer 2"),
    ("Odd page header 2", "Odd page footer 2"),
    ("Even page header 2", "Even page footer 2"),
  ];

  for (page_index, (header, footer)) in expected.into_iter().enumerate() {
    assert_page_contains(&summary, page_index, header);
    assert_page_contains(&summary, page_index, footer);
  }
}

#[test]
// Source: ../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:testContSectBreakHeaderFooter
fn mapped_fixture_continuous_section_break_preserves_header_footer_text() {
  let summary = render_summary("cont-sect-break-header-footer.docx");

  let expected = [
    (
      "First page header, section 1",
      "First page footer, section 1",
    ),
    (
      "First page header, section 2",
      "First page footer, section 2",
    ),
    ("Header, section 2", "Footer, section 3"),
  ];

  for (page_index, (header, footer)) in expected.into_iter().enumerate() {
    assert_page_contains(&summary, page_index, header);
    assert_page_contains(&summary, page_index, footer);
  }
}

#[test]
// Source: ../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:tdf166205_first_page_header_footer_visible
fn mapped_fixture_tdf166205_preserves_visible_first_page_header_footer() {
  let summary = render_summary("tdf166205_first_page_header_footer_visible.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "HEADER TOP #1");
  assert_page_contains(&summary, 0, "HEADER BOTTOM #1");
  assert_page_contains(&summary, 0, "THIS IS FOOTER #1");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:testTestTitlePage
fn mapped_fixture_test_title_page_preserves_second_page_footer_text() {
  let summary = render_summary("testTitlePage.docx");
  assert_page_contains(&summary, 1, "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:testInheritFirstHeader
fn mapped_fixture_inherit_first_header_preserves_header_sequence() {
  let summary = render_summary("inheritFirstHeader.docx");
  let expected = [
    "First Header",
    "Follow Header",
    "Follow Header",
    "First Header",
    "Last Header",
  ];

  for (page_index, expected_text) in expected.into_iter().enumerate() {
    assert_page_contains(&summary, page_index, expected_text);
  }
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:testN750255
fn mapped_fixture_n750255_preserves_column_break_as_page_break_text() {
  let summary = render_summary("n750255.docx");
  assert_page_contains(&summary, 1, "one");
  assert_page_contains(&summary, 2, "two");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:testN780843
fn mapped_fixture_n780843_preserves_shown_footer_and_two_pages() {
  let summary = render_summary("n780843.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "shown footer");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf155736
fn mapped_fixture_tdf155736_preserves_page_number_footer_text() {
  let summary = render_summary("tdf155736_PageNumbers_footer.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Page * of *");
  assert_page_contains(&summary, 1, "Page * of *");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:testNumOverrideLvltext
fn mapped_fixture_num_override_lvltext_preserves_overridden_number_text_and_color() {
  let summary = render_summary("num-override-lvltext.docx");
  assert_page_contains(&summary, 0, "1.1");
  assert_text_object_fill_color(&summary, "1.1", "#ffffff@ff");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf147646
fn mapped_fixture_tdf147646_preserves_merged_cell_numbering() {
  let summary = render_summary("tdf147646_mergedCellNumbering.docx");
  assert_page_contains(&summary, 0, "2.");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153613_anchoredAfterPgBreak
fn mapped_fixture_tdf153613_anchored_after_page_break_keeps_image_on_first_page() {
  let summary = render_summary("tdf153613_anchoredAfterPgBreak.docx");
  assert_page_image_count(&summary, 0, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153613_anchoredAfterPgBreak2
fn mapped_fixture_tdf153613_anchored_after_page_break2_moves_image_to_second_page() {
  let summary = render_summary("tdf153613_anchoredAfterPgBreak2.docx");
  assert_page_image_count(&summary, 1, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153613_anchoredAfterPgBreak3
fn mapped_fixture_tdf153613_anchored_after_page_break3_moves_image_to_second_page() {
  let summary = render_summary("tdf153613_anchoredAfterPgBreak3.docx");
  assert_page_image_count(&summary, 1, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153613_anchoredAfterPgBreak6
fn mapped_fixture_tdf153613_anchored_after_page_break6_preserves_page_text_and_image_position() {
  let summary = render_summary("tdf153613_anchoredAfterPgBreak6.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "y");
  assert_page_image_count(&summary, 0, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153613_inlineAfterPgBreak
fn mapped_fixture_tdf153613_inline_after_page_break_moves_image_to_second_page() {
  let summary = render_summary("tdf153613_inlineAfterPgBreak.docx");
  assert_page_image_count(&summary, 1, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153613_inlineAfterPgBreak2
fn mapped_fixture_tdf153613_inline_after_page_break2_preserves_text_and_image_position() {
  let summary = render_summary("tdf153613_inlineAfterPgBreak2.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "x");
  assert_page_image_count(&summary, 1, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153613_textboxAfterPgBreak3
fn mapped_fixture_tdf153613_textboxes_after_page_break_stay_on_second_page() {
  let summary = render_summary("tdf153613_textboxAfterPgBreak3.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Page 2 right");
  assert_page_contains(&summary, 1, "Page 2 middle");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf147724
fn mapped_fixture_tdf147724_preserves_external_xml_field_values() {
  let summary = render_summary("tdf147724.docx");
  assert_page_contains(&summary, 0, "Placeholder -> *ABC*");
  let text = normalized_page_text(&summary, 0);
  let has_second_abc = normalized_occurrences(&text, "Placeholder -> *ABC*") >= 2;
  let has_second_herunterladen = text.contains(&normalize_space("Placeholder -> *HERUNTERLADEN*"));
  assert!(
    has_second_abc || has_second_herunterladen,
    "missing allowed second field result; page text:\n{}",
    page_text(&summary, 0)
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testN751077
fn mapped_fixture_n751077_keeps_group_shape_text_on_first_page() {
  let summary = render_summary("n751077.docx");
  assert_page_contains(&summary, 0, "TEXT1");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testTdf136952_pgBreak3
fn mapped_fixture_tdf136952_pg_break3_starts_lorem_ipsum_on_page_six() {
  let summary = render_summary("tdf136952_pgBreak3.docx");
  assert_page_starts_with(&summary, 5, "Lorem ipsum");
}
