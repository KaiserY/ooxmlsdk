use ooxmlsdk_pdf_test::{
  PdfBounds, PdfSummary, assert_pdf_rect_close, parse_pdf_rect, pdf_summary_for_fixture,
  pdfexport_fixture_dir, rendered_page_image_for_fixture,
};

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

fn page_path_count(summary: &PdfSummary, page_index: usize) -> usize {
  summary
    .paths
    .iter()
    .filter(|path| path.page_index == page_index)
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

fn assert_page_path_count(summary: &PdfSummary, page_index: usize, expected: usize) {
  assert_eq!(
    page_path_count(summary, page_index),
    expected,
    "paths={:?}",
    summary.paths
  );
}

fn assert_page_has_no_text(summary: &PdfSummary, page_index: usize) {
  let text = page_text(summary, page_index);
  assert!(
    normalize_space(&text).is_empty(),
    "expected page {page_index} to have no text; page text:\n{text}"
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

fn assert_has_text_fill_color(summary: &PdfSummary, color: &str) {
  assert!(
    summary
      .text_objects
      .iter()
      .any(|object| object.fill_color.as_deref() == Some(color)),
    "missing text fill color {color}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_text_object_font_size(summary: &PdfSummary, text: &str, expected_size: &str) {
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
      .any(|object| object.scaled_font_size == expected_size),
    "missing font size {expected_size:?} for text {text:?}; objects={objects:?}"
  );
}

fn assert_raw_xobject_dimensions(summary: &PdfSummary, expected_width: u32, expected_height: u32) {
  let xobjects = summary
    .raw_pages
    .iter()
    .flat_map(|page| page.xobjects.iter())
    .collect::<Vec<_>>();
  assert!(
    xobjects.iter().any(|xobject| {
      let width = xobject.decoded_width_px.or(xobject.width_px);
      let height = xobject.decoded_height_px.or(xobject.height_px);
      width == Some(expected_width) && height == Some(expected_height)
    }),
    "missing raw image dimensions {expected_width}x{expected_height}; xobjects={xobjects:?}"
  );
}

fn image_bounds_with_raw_dimensions(
  summary: &PdfSummary,
  expected_width: u32,
  expected_height: u32,
) -> PdfBounds {
  assert_raw_xobject_dimensions(summary, expected_width, expected_height);
  summary
    .images
    .iter()
    .filter_map(|image| image.bounds.as_deref())
    .filter_map(|bounds| parse_pdf_rect(bounds).ok())
    .find(|bounds| bounds.width() > 0.0 && bounds.height() > 0.0)
    .unwrap_or_else(|| {
      panic!("missing rendered image bounds for raw dimensions {expected_width}x{expected_height}")
    })
}

fn assert_rendered_image_source_pixel_color(
  fixture_name: &str,
  summary: &PdfSummary,
  image_width: u32,
  image_height: u32,
  source_x: u32,
  source_y: u32,
  expected_rgb: [u8; 3],
) {
  let image_bounds = image_bounds_with_raw_dimensions(summary, image_width, image_height);
  let rendered = rendered_page_image_for_fixture(&fixture(fixture_name), 0, 1200)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let x = image_bounds.left + (source_x as f32 + 0.5) / image_width as f32 * image_bounds.width();
  let y_from_top = (source_y as f32 + 0.5) / image_height as f32 * image_bounds.height();
  let y = image_bounds.top - y_from_top;
  let [r, g, b, _] = rendered
    .sample_pdf_point_rgba(x, y)
    .unwrap_or_else(|| panic!("missing rendered pixel sample at PDF point ({x}, {y})"));
  let diff = (i16::from(r) - i16::from(expected_rgb[0])).abs()
    + (i16::from(g) - i16::from(expected_rgb[1])).abs()
    + (i16::from(b) - i16::from(expected_rgb[2])).abs();
  assert!(
    diff <= 12,
    "rendered image pixel ({source_x}, {source_y}) color #{r:02x}{g:02x}{b:02x} differs from expected #{:02x}{:02x}{:02x}",
    expected_rgb[0],
    expected_rgb[1],
    expected_rgb[2]
  );
}

fn path_bounds_on_page(summary: &PdfSummary, page_index: usize) -> Vec<PdfBounds> {
  summary
    .paths
    .iter()
    .filter(|path| path.page_index == page_index)
    .filter_map(|path| path.bounds.as_deref())
    .filter_map(|bounds| parse_pdf_rect(bounds).ok())
    .collect()
}

fn assert_path_width_close(summary: &PdfSummary, page_index: usize, expected_width: f32) {
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (bounds.width() - expected_width).abs() <= 0.5),
    "missing page {page_index} path width {expected_width}pt; bounds={bounds:?}"
  );
}

fn assert_path_width_count_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_width: f32,
  expected_count: usize,
) {
  let bounds = path_bounds_on_page(summary, page_index);
  let count = bounds
    .iter()
    .filter(|bounds| (bounds.width() - expected_width).abs() <= 0.5)
    .count();
  assert!(
    count >= expected_count,
    "expected at least {expected_count} page {page_index} paths with width {expected_width}pt, got {count}; bounds={bounds:?}"
  );
}

fn assert_path_top_from_page_top_close(summary: &PdfSummary, page_index: usize, expected_top: f32) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (media_box.top - bounds.top - expected_top).abs() <= 0.5),
    "missing page {page_index} path top {expected_top}pt from page top; bounds={bounds:?}"
  );
}

fn assert_rightmost_path_within_page_right(
  summary: &PdfSummary,
  page_index: usize,
  tolerance: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = path_bounds_on_page(summary, page_index);
  let rightmost = bounds
    .iter()
    .map(|bounds| bounds.right)
    .fold(f32::NEG_INFINITY, f32::max);
  assert!(
    rightmost <= media_box.right + tolerance,
    "rightmost page {page_index} path {rightmost} is outside page right {} + {tolerance}; bounds={bounds:?}",
    media_box.right
  );
}

fn assert_any_text_segment_width_positive(summary: &PdfSummary, page_index: usize) {
  let widths = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .map(|bounds| bounds.width())
    .collect::<Vec<_>>();
  assert!(
    widths.iter().any(|width| *width > 0.0),
    "missing positive text width on page {page_index}; widths={widths:?}"
  );
}

fn max_horizontal_gap_on_page(summary: &PdfSummary, page_index: usize) -> f32 {
  let mut bounds = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .collect::<Vec<_>>();
  bounds.sort_by(|a, b| {
    b.top
      .partial_cmp(&a.top)
      .unwrap()
      .then_with(|| a.left.partial_cmp(&b.left).unwrap())
  });
  let mut max_gap = 0.0;
  for pair in bounds.windows(2) {
    let previous = pair[0];
    let next = pair[1];
    if (previous.top - next.top).abs() <= 2.0 {
      let gap = next.left - previous.right;
      if gap > max_gap {
        max_gap = gap;
      }
    }
  }
  max_gap
}

fn assert_max_horizontal_gap_at_least(summary: &PdfSummary, page_index: usize, expected_gap: f32) {
  let max_gap = max_horizontal_gap_on_page(summary, page_index);
  assert!(
    max_gap >= expected_gap,
    "missing page {page_index} horizontal gap >= {expected_gap}pt; max_gap={max_gap}"
  );
}

fn assert_rendered_region_has_dark_pixel(
  fixture_name: &str,
  page_index: usize,
  target_width: i32,
  x_range: std::ops::Range<u32>,
  y_range: std::ops::Range<u32>,
) {
  let image = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, target_width)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let mut found = false;
  for y in y_range.clone() {
    for x in x_range.clone() {
      if let Some([r, g, b, _]) = image.pixel_rgba(x, y)
        && r < 80
        && g < 80
        && b < 80
      {
        found = true;
        break;
      }
    }
  }
  assert!(
    found,
    "missing dark pixel in rendered page {page_index} region x={x_range:?} y={y_range:?}"
  );
}

fn assert_media_box_close(summary: &PdfSummary, page_index: usize, expected: PdfBounds) {
  let media_box = summary
    .media_boxes
    .get(page_index)
    .unwrap_or_else(|| panic!("missing media box for page {page_index}"));
  assert_pdf_rect_close(media_box, expected, 0.1);
}

fn text_segment_left(summary: &PdfSummary, text: &str) -> f32 {
  let segment = summary
    .text_segments
    .iter()
    .find(|segment| normalize_space(&segment.text).contains(text))
    .unwrap_or_else(|| panic!("missing text segment containing {text:?}"));
  parse_pdf_rect(&segment.bounds).unwrap().left
}

fn first_text_segment_height(summary: &PdfSummary, fixture_name: &str) -> f32 {
  let segment = summary
    .text_segments
    .first()
    .unwrap_or_else(|| panic!("missing first text segment in {fixture_name}"));
  parse_pdf_rect(&segment.bounds).unwrap().height()
}

fn first_path_top_from_page_top(summary: &PdfSummary, fixture_name: &str) -> f32 {
  let path = summary
    .paths
    .iter()
    .find(|path| path.page_index == 0)
    .unwrap_or_else(|| panic!("missing first page path in {fixture_name}"));
  let path_bounds = parse_pdf_rect(
    path
      .bounds
      .as_deref()
      .unwrap_or_else(|| panic!("missing path bounds in {fixture_name}")),
  )
  .unwrap();
  let media_box = summary
    .media_boxes
    .first()
    .unwrap_or_else(|| panic!("missing media box in {fixture_name}"));
  let page_bounds = parse_pdf_rect(media_box).unwrap();
  page_bounds.top - path_bounds.top
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

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:testTdf123636_newlinePageBreak3
fn mapped_fixture_tdf123636_newline_page_break3_keeps_last_line_on_first_page() {
  let summary = render_summary("tdf123636_newlinePageBreak3.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Last line on page 1");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:testTdf123636_newlinePageBreak4
fn mapped_fixture_tdf123636_newline_page_break4_keeps_second_page_empty() {
  let summary = render_summary("tdf123636_newlinePageBreak4.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_has_no_text(&summary, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:testTdf169802_hidden_shape
fn mapped_fixture_tdf169802_hidden_shape_does_not_render_fly_content() {
  let summary = render_summary("tdf169802_hidden_shape.docx");
  assert!(!summary.text_segments.is_empty());
  assert_page_image_count(&summary, 0, 0);
  assert_page_path_count(&summary, 0, 0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:testTdf124594
fn mapped_fixture_tdf124594_keeps_shape_margin_from_splitting_first_line() {
  let summary = render_summary("tdf124594.docx");
  assert_page_contains(
    &summary,
    0,
    "Er horte leise Schritte hinter sich. Das bedeutete nichts Gutes. Wer wu",
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf128197
fn mapped_fixture_tdf128197_preserves_compat15_larger_first_paragraph_height() {
  let compat14 = render_summary("128197_compat14.docx");
  let compat15 = render_summary("128197_compat15.docx");
  let height14 = first_text_segment_height(&compat14, "128197_compat14.docx");
  let height15 = first_text_segment_height(&compat15, "128197_compat15.docx");
  assert!(
    height14 < height15,
    "expected compat14 first text height to be less than compat15: {height14} >= {height15}"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport17.cxx:testTdf149313
fn mapped_fixture_tdf149313_preserves_two_pages_and_section_page_sizes() {
  let summary = render_summary("tdf149313.docx");
  assert_eq!(summary.page_count, 2);
  assert_media_box_close(
    &summary,
    0,
    PdfBounds {
      left: 0.0,
      bottom: 0.0,
      right: 249.45,
      top: 249.45,
    },
  );
  assert_media_box_close(
    &summary,
    1,
    PdfBounds {
      left: 0.0,
      bottom: 0.0,
      right: 400.0,
      top: 249.45,
    },
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf154369
fn mapped_fixture_tdf154369_preserves_green_heading_numbering() {
  let summary = render_summary("tdf154369.docx");
  assert_text_object_fill_color(&summary, "A.", "#527d55@ff");
  assert_text_object_fill_color(&summary, "B.", "#527d55@ff");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testTdf75573
fn mapped_fixture_tdf75573_keeps_frame_text_on_first_page() {
  let summary = render_summary("tdf75573_page1frame.docx");
  assert_page_contains(&summary, 0, "lorem ipsum");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:testTdf120548
fn mapped_fixture_tdf120548_preserves_red_numbering_portion() {
  let summary = render_summary("tdf120548.docx");
  assert_has_text_fill_color(&summary, "#ff0000@ff");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:testNumberingLevels
fn mapped_fixture_tdf95495_preserves_numbering_level_visible_text() {
  let summary = render_summary("tdf95495.docx");
  assert_page_contains(&summary, 0, "A.2.1");
  assert_page_contains(&summary, 0, ".DESCRIPTION");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:testfdo81031
fn mapped_fixture_fdo81031_preserves_vml_image_bitmap_dimensions() {
  let summary = render_summary("fdo81031.docx");
  assert_raw_xobject_dimensions(&summary, 381, 148);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:testN705956_1
fn mapped_fixture_n705956_1_preserves_grouped_inline_image_bitmap_dimensions() {
  let summary = render_summary("n705956-1.docx");
  assert_eq!(summary.image_count, 1);
  assert_raw_xobject_dimensions(&summary, 120, 106);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:testTextBoxPictureFill
fn mapped_fixture_textbox_picture_fill_preserves_bitmap_fill_dimensions() {
  let summary = render_summary("textbox_picturefill.docx");
  assert_raw_xobject_dimensions(&summary, 447, 528);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:testPictureWithSchemeColor
fn mapped_fixture_picture_with_scheme_color_preserves_bitmap_dimensions() {
  let summary = render_summary("picture-with-schemecolor.docx");
  assert_raw_xobject_dimensions(&summary, 341, 181);
  assert_rendered_image_source_pixel_color(
    "picture-with-schemecolor.docx",
    &summary,
    341,
    181,
    120,
    30,
    [0xad, 0xc5, 0xdb],
  );
  assert_rendered_image_source_pixel_color(
    "picture-with-schemecolor.docx",
    &summary,
    341,
    181,
    260,
    130,
    [0xad, 0xc5, 0xdb],
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testN777345
fn mapped_fixture_n777345_preserves_vml_rect_image_dimensions() {
  let summary = render_summary("n777345.docx");
  assert_raw_xobject_dimensions(&summary, 17, 16);
  assert_rendered_image_source_pixel_color("n777345.docx", &summary, 17, 16, 0, 0, [0, 0, 0]);
  assert_rendered_image_source_pixel_color("n777345.docx", &summary, 17, 16, 16, 15, [0, 0, 0]);
  assert_rendered_image_source_pixel_color("n777345.docx", &summary, 17, 16, 16, 0, [153, 0, 0]);
  assert_rendered_image_source_pixel_color("n777345.docx", &summary, 17, 16, 0, 15, [153, 0, 0]);
}

#[test]
// Source: ../core/sw/qa/extras/uiwriter/uiwriter2.cxx:testTdfChangeNumberingListAutoFormat
fn mapped_fixture_tdf117923_preserves_numbering_portion_text_and_height() {
  let summary = render_summary("tdf117923.docx");
  assert_page_contains(&summary, 0, "GHI GHI GHI GHI");
  assert_text_object_font_size(&summary, "2.", "11.00");
}

#[test]
// Source: ../core/sw/qa/extras/uiwriter/uiwriter4.cxx:testTdf104492
fn mapped_fixture_tdf104492_splits_table_over_three_pages() {
  let summary = render_summary("tdf104492.docx");
  assert_eq!(summary.page_count, 3);
}

#[test]
// Source: ../core/sw/qa/writerfilter/ooxml/ooxml.cxx:testFloattableMultiNested
fn mapped_fixture_floattable_multi_nested_splits_floating_tables_across_two_pages() {
  let summary = render_summary("floattable-multi-nested.docx");
  assert_eq!(summary.page_count, 2);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:testTdf124600
fn mapped_fixture_tdf124600_aligns_header_shape_text_with_body_text() {
  let summary = render_summary("tdf124600.docx");
  let shape_left = text_segment_left(&summary, "Shape 1 text");
  let body_left = text_segment_left(&summary, "X");
  assert!(
    (shape_left - body_left).abs() <= 0.05,
    "shape text left {shape_left} differs from body text left {body_left}"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:testTdf113946
fn mapped_fixture_tdf113946_preserves_anchored_line_top_position() {
  let summary = render_summary("tdf113946.docx");
  let top_from_page_top = first_path_top_from_page_top(&summary, "tdf113946.docx");
  assert!(
    (top_from_page_top - 84.75).abs() <= 0.1,
    "anchored line top from page top {top_from_page_top} differs from expected 84.75pt"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:testMsoBrightnessContrast
fn mapped_fixture_mso_brightness_contrast_preserves_bitmap_dimensions() {
  let summary = render_summary("msobrightnesscontrast.docx");
  assert_raw_xobject_dimensions(&summary, 58, 320);
  assert_rendered_image_source_pixel_color(
    "msobrightnesscontrast.docx",
    &summary,
    58,
    320,
    20,
    30,
    [0xce, 0xce, 0xce],
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport16.cxx:testTdf136841
fn mapped_fixture_tdf136841_preserves_cropped_bitmap_dimensions() {
  let summary = render_summary("tdf136841.docx");
  assert_raw_xobject_dimensions(&summary, 76, 76);
  assert_rendered_image_source_pixel_color(
    "tdf136841.docx",
    &summary,
    76,
    76,
    38,
    38,
    [228, 72, 70],
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:testTdf156078
fn mapped_fixture_tdf156078_keeps_right_tab_number_at_top_right() {
  assert_rendered_region_has_dark_pixel(
    "tdf156078_rightTabOutsideParaRightIndent.docx",
    0,
    816,
    680..720,
    90..130,
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:testRelativeAnchorWidthFromLeftMargin
fn mapped_fixture_tdf132976_preserves_anchor_width_from_left_margin() {
  let summary = render_summary("tdf132976_testRelativeAnchorWidthFromLeftMargin.docx");
  assert_path_width_close(&summary, 0, 56.65);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:testRelativeAnchorWidthFromInsideOutsideMargin
fn mapped_fixture_tdf133861_preserves_inside_outside_anchor_widths() {
  let summary = render_summary("tdf133861_RelativeAnchorWidthFromInsideOutsideMargin.docx");
  assert_path_width_count_close(&summary, 0, 72.0, 2);
  assert_path_width_count_close(&summary, 0, 127.6, 2);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport6.cxx:testRelativeAlignmentFromTopMargin
fn mapped_fixture_tdf133045_preserves_relative_alignment_from_top_margin() {
  let summary = render_summary("tdf133045_TestShapeAlignmentRelativeFromTopMargin.docx");
  assert_path_top_from_page_top_close(&summary, 0, 75.1);
  assert_path_top_from_page_top_close(&summary, 0, 134.15);
  assert_path_top_from_page_top_close(&summary, 0, 15.65);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:testTdf113183
fn mapped_fixture_tdf113183_keeps_second_shape_inside_page_bounds() {
  let summary = render_summary("tdf113183.docx");
  assert_rightmost_path_within_page_right(&summary, 0, 1.05);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:testTdf120511_eatenSection
fn mapped_fixture_tdf120511_eaten_section_preserves_page_orientations() {
  let summary = render_summary("tdf120511_eatenSection.docx");
  let page1 = parse_pdf_rect(&summary.media_boxes[0]).unwrap();
  let page2 = parse_pdf_rect(&summary.media_boxes[1]).unwrap();
  assert!(
    page1.width() < page1.height(),
    "page 1 is not portrait: {page1:?}"
  );
  assert!(
    page2.width() > page2.height(),
    "page 2 is not landscape: {page2:?}"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:testTdf119760_positionCellBorder
fn mapped_fixture_tdf119760_keeps_cell_text_inside_row_left_border() {
  let summary = render_summary("tdf119760_positionCellBorder.docx");
  let row_left = path_bounds_on_page(&summary, 0)
    .iter()
    .map(|bounds| bounds.left)
    .fold(f32::INFINITY, f32::min);
  let text_left = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == 0)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .map(|bounds| bounds.left)
    .fold(f32::INFINITY, f32::min);
  assert!(
    row_left < text_left,
    "row left {row_left} is not less than text left {text_left}; paths={:?}",
    summary.paths
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:testTdf116985
fn mapped_fixture_tdf116985_preserves_relative_body_frame_width() {
  let summary = render_summary("tdf116985.docx");
  assert!(
    path_bounds_on_page(&summary, 0)
      .iter()
      .any(|bounds| bounds.width() > 200.0),
    "missing frame wider than 4000 twips / 200pt; paths={:?}",
    summary.paths
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport12.cxx:testTd112202
fn mapped_fixture_td112202_keeps_third_page_body_heading_after_header_layout() {
  let summary = render_summary("090716_Studentische_Arbeit_VWS.docx");
  assert_page_contains(&summary, 2, "AUFGABENSTELLUNG");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:testTdf102466
fn mapped_fixture_tdf102466_preserves_eleven_page_layout() {
  let summary = render_summary("tdf102466.docx");
  assert_eq!(summary.page_count, 11);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:testTdf95367_inheritFollowStyle
fn mapped_fixture_tdf95367_inherits_follow_style_header_on_second_page() {
  let summary = render_summary("tdf95367_inheritFollowStyle.docx");
  assert_page_contains(&summary, 1, "header");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport9.cxx:testTdf84678
fn mapped_fixture_tdf84678_preserves_textbox_left_inset() {
  let summary = render_summary("tdf84678.docx");
  let leftmost_path = path_bounds_on_page(&summary, 0)
    .iter()
    .map(|bounds| bounds.left)
    .fold(f32::INFINITY, f32::min);
  let leftmost_text = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == 0)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .map(|bounds| bounds.left)
    .fold(f32::INFINITY, f32::min);
  assert!(
    (leftmost_text - leftmost_path - 28.35).abs() <= 1.0,
    "textbox left inset differs from 567 twips / 28.35pt: path={leftmost_path}, text={leftmost_text}"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport9.cxx:testTdf103544
fn mapped_fixture_tdf103544_keeps_image_in_frame_export_case() {
  let summary = render_summary("tdf103544.docx");
  assert!(
    summary.image_count > 0,
    "expected at least one image object; images={:?}",
    summary.images
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport_de_locale.cxx:testTdf160402
fn mapped_fixture_tdf160402_preserves_styleref_header_expansions() {
  let summary = render_summary("StyleRef-DE.docx");
  let expected = [
    "Heading 1",
    "Nunc viverra imperdiet enim. Fusce est. Vivamus a tellus.",
    "Cras faucibus condimentum odio. Sed ac ligula. Aliquam at eros.",
    "Nunc viverra imperdiet enim. Fusce est. Vivamus a tellus.",
    "Aenean nec lorem. In porttitor. Donec laoreet nonummy augue.",
  ];
  for (page_index, expected_text) in expected.into_iter().enumerate() {
    assert_page_contains(&summary, page_index, expected_text);
  }
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testN758883
fn mapped_fixture_n758883_preserves_numbering_font_height() {
  let summary = render_summary("n758883.docx");
  assert_text_object_font_size(&summary, "1.", "11.00");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:testTdf95377
fn mapped_fixture_tdf95377_preserves_visible_numbering_for_first_paragraphs() {
  let summary = render_summary("tdf95377.docx");
  assert_text_object_font_size(&summary, "a.", "11.00");
  assert_page_contains(&summary, 0, "a.");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:testNegativeCellMarginTwips
fn mapped_fixture_negative_cell_margin_twips_keeps_cell_text_visible() {
  let summary = render_summary("negative-cell-margin-twips.docx");
  assert_any_text_segment_width_positive(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153042_largeTab
fn mapped_fixture_tdf153042_large_tab_preserves_pseudo_numbering_tab_width() {
  let summary = render_summary("tdf153042_largeTab.docx");
  assert_max_horizontal_gap_at_least(&summary, 0, 85.05);
}
