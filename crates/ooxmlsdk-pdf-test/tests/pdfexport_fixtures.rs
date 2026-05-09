use ooxmlsdk_pdf_test::{PdfSummary, pdf_summary_for_fixture, pdfexport_fixture_dir};

fn fixture(name: &str) -> std::path::PathBuf {
  pdfexport_fixture_dir().join(name)
}

fn render_summary(name: &str) -> PdfSummary {
  pdf_summary_for_fixture(&fixture(name)).unwrap()
}

fn raw_page(summary: &PdfSummary, page_index: usize) -> &ooxmlsdk_pdf_test::RawPageSummary {
  summary
    .raw_pages
    .iter()
    .find(|page| page.page_index == page_index)
    .unwrap_or_else(|| panic!("missing raw page summary for page {page_index}"))
}

fn page_object_count(summary: &PdfSummary, page_index: usize) -> usize {
  let page = summary
    .page_objects
    .iter()
    .find(|page| page.page_index == page_index)
    .unwrap_or_else(|| panic!("missing page object summary for page {page_index}"));
  page.text_objects
    + page.path_objects
    + page.image_objects
    + page.shading_objects
    + page.form_objects
    + page.unsupported_objects
}

fn parse_rect(rect: &str) -> [f64; 4] {
  let trimmed = rect
    .trim()
    .strip_prefix('[')
    .and_then(|value| value.strip_suffix(']'))
    .unwrap_or_else(|| panic!("invalid rect format: {rect}"));
  let values = trimmed
    .split_ascii_whitespace()
    .map(|value| value.parse::<f64>().unwrap())
    .collect::<Vec<_>>();
  assert_eq!(values.len(), 4, "expected four rect coordinates in {rect}");
  [values[0], values[1], values[2], values[3]]
}

fn assert_rect_eq(actual: &str, expected: [f64; 4]) {
  let actual = parse_rect(actual);
  for (actual, expected) in actual.into_iter().zip(expected) {
    assert!(
      (actual - expected).abs() <= 0.000001,
      "rect mismatch: actual={actual} expected={expected}"
    );
  }
}

#[test]
fn pdfexport_fixture_fdo47811_word2013_has_two_pages() {
  let summary = render_summary("fdo47811-1_Word2013.docx");
  assert_eq!(summary.page_count, 2);
}

#[test]
fn pdfexport_fixture_tdf145274_matches_upstream_text_object_expectations() {
  let summary = render_summary("tdf145274.docx");
  assert_eq!(summary.page_count, 1);
  assert_eq!(page_object_count(&summary, 0), 6);

  let text_objects = summary
    .text_objects
    .iter()
    .filter(|object| object.page_index == 0)
    .collect::<Vec<_>>();

  for object in text_objects {
    assert_eq!(object.scaled_font_size, "11.00");
    assert_eq!(object.render_mode, "FilledUnstroked");
    assert_eq!(object.fill_color.as_deref(), Some("#ff0000@ff"));
  }
}

#[test]
fn pdfexport_fixture_tdf156685_matches_upstream_text_object_expectations() {
  let summary = render_summary("tdf156685.docx");
  assert_eq!(summary.page_count, 1);
  assert_eq!(page_object_count(&summary, 0), 9);

  let text_objects = summary
    .text_objects
    .iter()
    .filter(|object| object.page_index == 0)
    .collect::<Vec<_>>();

  for object in text_objects {
    assert_eq!(object.scaled_font_size, "11.00");
    assert_eq!(object.render_mode, "FilledUnstroked");
    assert_eq!(object.fill_color.as_deref(), Some("#000000@ff"));
  }
}

#[test]
fn pdfexport_fixture_tdf142133_preserves_google_link_annotation() {
  let summary = render_summary("tdf142133.docx");
  assert_eq!(summary.page_count, 1);
  let page = raw_page(&summary, 0);
  assert_eq!(page.annotation_count, 1);
  assert_eq!(page.annotations.len(), 1);
  assert_eq!(page.annotations[0].type_name.as_deref(), Some("Annot"));
  assert_eq!(page.annotations[0].subtype_name.as_deref(), Some("Link"));
  assert_eq!(
    page.annotations[0].action_uri.as_deref(),
    Some("https://google.com/")
  );
}

#[test]
fn pdfexport_fixture_content_control_rtl_matches_upstream_widget_rects() {
  let summary = render_summary("content-control-rtl.docx");
  assert_eq!(summary.page_count, 1);
  let page = raw_page(&summary, 0);
  assert_eq!(page.annotation_count, 5);

  let widgets = page
    .annotations
    .iter()
    .filter(|annotation| annotation.type_name.as_deref() == Some("Annot"))
    .filter(|annotation| annotation.subtype_name.as_deref() == Some("Widget"))
    .collect::<Vec<_>>();

  assert_eq!(widgets.len(), 5);

  let expected = [
    [55.699, 706.701, 132.401, 722.499],
    [197.499, 706.701, 274.201, 722.499],
    [302.349, 679.101, 379.051, 694.899],
    [479.599, 679.101, 556.301, 694.899],
    [55.699, 651.501, 132.401, 667.299],
  ];

  for (widget, expected_rect) in widgets.iter().zip(expected) {
    let bounds = widget
      .rect
      .as_deref()
      .unwrap_or_else(|| panic!("missing widget bounds for {:?}", widget));
    assert_rect_eq(bounds, expected_rect);
  }
}
