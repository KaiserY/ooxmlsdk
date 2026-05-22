use ooxmlsdk_pdf_test::{PdfSummary, pdf_summary_for_fixture, pdfexport_fixture_dir};

fn fixture(name: &str) -> std::path::PathBuf {
  pdfexport_fixture_dir().join(name)
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

fn assert_page_contains_in_order(summary: &PdfSummary, page_index: usize, expected: &[&str]) {
  let text = page_text(summary, page_index);
  let normalized_text = normalize_space(&text);
  let mut cursor = 0;
  for item in expected {
    let normalized_item = normalize_space(item);
    let Some(offset) = normalized_text[cursor..].find(&normalized_item) else {
      panic!("missing page {page_index} text {item:?} after offset {cursor}; page text:\n{text}");
    };
    cursor += offset + normalized_item.len();
  }
}

fn assert_has_stroked_path_color(summary: &PdfSummary, expected: &str) {
  assert!(
    summary
      .paths
      .iter()
      .any(|path| path.stroked == Some(true) && path.stroke_color.as_deref() == Some(expected)),
    "missing stroked path color {expected}; paths={:?}",
    summary.paths
  );
}

#[test]
#[ignore = "PPTX renderer still emits master placeholder text into the slide output"]
// Source: ../core/sd/qa/unit/layout-tests.cxx:numberedList
fn mapped_pptx_numbered_list_preserves_imported_numbering_text_order() {
  let summary = render_summary("pptx/NumberedList-12ab-ab-34.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "1.",
      "Outer, one",
      "2.",
      "Outer, two",
      "a.",
      "Second level, a",
      "b.",
      "Second level, b",
      "Blank second level",
      "a.",
      "Second level restart, a",
      "b.",
      "Second level restart, b",
      "3.",
      "Outer, three",
      "4.",
      "Outer, four",
    ],
  );
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf168010_PPTX
fn mapped_pptx_trailing_paragraphs_keeps_visible_textbox_text() {
  let summary = render_summary("pptx/trailing-paragraphs.pptx");
  assert_page_contains_in_order(&summary, 0, &["textbox"]);
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testBnc480256
fn mapped_pptx_bnc480256_preserves_red_table_inside_border() {
  let summary = render_summary("pptx/bnc480256-2.pptx");
  assert_has_stroked_path_color(&summary, "#ff0000@ff");
}
