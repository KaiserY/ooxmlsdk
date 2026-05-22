use ooxmlsdk_pdf_test::{
  PdfBounds, PdfSummary, parse_pdf_rect, pdf_summary_for_fixture, pdfexport_fixture_dir,
};

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

fn assert_page_count(summary: &PdfSummary, expected: usize) {
  assert_eq!(
    summary.page_count, expected,
    "page count mismatch; summary={summary:?}"
  );
}

fn assert_text_fill_color(summary: &PdfSummary, expected_text: &str, expected_color: &str) {
  assert!(
    summary.text_objects.iter().any(
      |object| normalize_space(&object.text).contains(expected_text)
        && object.fill_color.as_deref() == Some(expected_color)
    ),
    "missing text {expected_text:?} with fill color {expected_color}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_has_text_fill_color(summary: &PdfSummary, expected_color: &str) {
  assert!(
    summary
      .text_objects
      .iter()
      .any(|object| object.fill_color.as_deref() == Some(expected_color)),
    "missing text fill color {expected_color}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_has_path_fill_color(summary: &PdfSummary, expected_color: &str) {
  assert!(
    summary
      .paths
      .iter()
      .any(|path| path.fill_mode.is_some() && path.fill_color.as_deref() == Some(expected_color)),
    "missing filled path color {expected_color}; paths={:?}",
    summary.paths
  );
}

fn assert_page_has_stroked_path(summary: &PdfSummary, page_index: usize) {
  assert!(
    summary
      .paths
      .iter()
      .any(|path| path.page_index == page_index && path.stroked == Some(true)),
    "missing stroked path on page {page_index}; paths={:?}",
    summary.paths
  );
}

fn assert_page_has_horizontal_stroked_path(summary: &PdfSummary, page_index: usize) {
  assert!(
    summary
      .paths
      .iter()
      .filter(|path| path.page_index == page_index)
      .filter(|path| path.stroked == Some(true))
      .filter_map(|path| path.bounds.as_deref())
      .filter_map(|bounds| parse_pdf_rect(bounds).ok())
      .any(|bounds| bounds.width() > 10.0 && bounds.height() <= 3.0),
    "missing horizontal stroked path on page {page_index}; paths={:?}",
    summary.paths
  );
}

fn text_bounds_containing(summary: &PdfSummary, page_index: usize, expected: &str) -> PdfBounds {
  summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .find(|segment| normalize_space(&segment.text).contains(expected))
    .and_then(|segment| parse_pdf_rect(&segment.bounds).ok())
    .unwrap_or_else(|| {
      panic!(
        "missing text segment containing {expected:?} on page {page_index}; text_segments={:?}",
        summary.text_segments
      )
    })
}

fn assert_text_near_libreoffice_metafile_point(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  x_100mm: f32,
  y_100mm: f32,
  tolerance_pt: f32,
) {
  let bounds = text_bounds_containing(summary, page_index, expected);
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let expected_left = x_100mm * 72.0 / 2540.0;
  let expected_top = media_box.top - y_100mm * 72.0 / 2540.0;

  assert!(
    (bounds.left - expected_left).abs() <= tolerance_pt
      && (bounds.top - expected_top).abs() <= tolerance_pt,
    "text {expected:?} bounds {bounds:?} are not near LibreOffice metafile point ({x_100mm}, {y_100mm}) -> ({expected_left:.2}, {expected_top:.2})pt"
  );
}

fn assert_text_y_near_libreoffice_metafile_point(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  y_100mm: f32,
  tolerance_pt: f32,
) {
  let bounds = text_bounds_containing(summary, page_index, expected);
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let expected_top = media_box.top - y_100mm * 72.0 / 2540.0;

  assert!(
    (bounds.top - expected_top).abs() <= tolerance_pt,
    "text {expected:?} bounds {bounds:?} are not near LibreOffice metafile y {y_100mm} -> {expected_top:.2}pt"
  );
}

fn assert_text_near_libreoffice_relative_metafile_point(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  map_x_100mm: f32,
  map_y_100mm: f32,
  text_x_100mm: f32,
  text_y_100mm: f32,
  tolerance_pt: f32,
) {
  assert_text_near_libreoffice_metafile_point(
    summary,
    page_index,
    expected,
    map_x_100mm + text_x_100mm,
    map_y_100mm + text_y_100mm,
    tolerance_pt,
  );
}

fn assert_vertical_text_shape(summary: &PdfSummary, page_index: usize, expected: &str) {
  let bounds = text_bounds_containing(summary, page_index, expected);
  assert!(
    bounds.height() > bounds.width(),
    "expected {expected:?} to render as vertical text; bounds={bounds:?}"
  );
}

fn page_path_count(summary: &PdfSummary, page_index: usize) -> usize {
  summary
    .paths
    .iter()
    .filter(|path| path.page_index == page_index)
    .count()
}

fn assert_has_tall_stroked_path(summary: &PdfSummary, page_index: usize) {
  assert!(
    summary
      .paths
      .iter()
      .filter(|path| path.page_index == page_index)
      .filter(|path| path.stroked == Some(true))
      .filter_map(|path| path.bounds.as_deref())
      .filter_map(|bounds| parse_pdf_rect(bounds).ok())
      .any(|bounds| bounds.height() > bounds.width() * 8.0),
    "missing tall stroked path on page {page_index}; paths={:?}",
    summary.paths
  );
}

fn assert_page_has_clipping_ops(summary: &PdfSummary, expected_min: usize) {
  assert!(
    summary.content.clipping_ops >= expected_min,
    "expected at least {expected_min} clipping ops; content={:?}",
    summary.content
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

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf104722
fn mapped_pptx_tdf104722_places_subtitle_at_upstream_metafile_position() {
  let summary = render_summary("pptx/tdf104722.pptx");
  assert_page_contains_in_order(&summary, 0, &["Subtitle for this part"]);
  assert_text_near_libreoffice_metafile_point(&summary, 0, "Subtitle", 2093.0, 9273.0, 36.0);
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf135843
fn mapped_pptx_tdf135843_keeps_table_border_vertical() {
  let summary = render_summary("pptx/tdf135843.pptx");
  assert_has_tall_stroked_path(&summary, 0);
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf135843_InsideHBorders
fn mapped_pptx_tdf135843_inside_horizontal_borders_do_not_duplicate_vertical_borders() {
  let summary = render_summary("pptx/tdf135843_insideH.pptx");
  assert!(
    page_path_count(&summary, 0) <= 34,
    "LibreOffice layout has 34 draw pushes for this case; paths={:?}",
    summary.paths
  );
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTableVerticalText
fn mapped_pptx_table_vertical_text_preserves_cell_text_rotation() {
  let summary = render_summary("pptx/tcPr-vert-roundtrip.pptx");
  assert_vertical_text_shape(&summary, 0, "Abcdefg-90-degrees");
  assert_vertical_text_shape(&summary, 0, "12345-270-degrees");
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf164622
fn mapped_pptx_tdf164622_preserves_clip_region() {
  let summary = render_summary("pptx/tdf164622.pptx");
  assert_page_has_clipping_ops(&summary, 1);
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf128212
fn mapped_pptx_tdf128212_keeps_rotated_text_at_upstream_metafile_position() {
  let summary = render_summary("pptx/tdf128212.pptx");
  assert_page_contains_in_order(&summary, 0, &["Vertical it should be!"]);
  assert_text_near_libreoffice_relative_metafile_point(
    &summary,
    0,
    "Vertical it should be!",
    331.0,
    9420.0,
    4760.0,
    -2250.0,
    48.0,
  );
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf148966
fn mapped_pptx_tdf148966_ignores_break_after_multiline_field() {
  let summary = render_summary("pptx/tdf148966.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "Some multi line hyperlink/field",
      "text that follows after a",
      "linebreak",
    ],
  );
  assert_text_y_near_libreoffice_metafile_point(&summary, 0, "linebreak", 5952.0, 48.0);
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf128206
fn mapped_pptx_tdf128206_keeps_arrow_text_at_upstream_metafile_position() {
  let summary = render_summary("pptx/tdf128206.pptx");
  assert_page_contains_in_order(&summary, 0, &["a b c d e f g h I j k l m n o p q"]);
  assert_text_near_libreoffice_relative_metafile_point(
    &summary,
    0,
    "a b c d e f g h I j k l m n o p q",
    14416.0,
    1658.0,
    -11031.0,
    3617.0,
    48.0,
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testSmoketest
fn mapped_pptx_smoketest_preserves_three_imported_pages() {
  let summary = render_summary("smoketest.pptx");
  assert_page_count(&summary, 3);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf150770
fn mapped_pptx_tdf150770_preserves_four_imported_slides() {
  let summary = render_summary("pptx/tdf150770.pptx");
  assert_page_count(&summary, 4);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc591147
fn mapped_pptx_bnc591147_preserves_two_media_slides() {
  let summary = render_summary("pptx/bnc591147.pptx");
  assert_page_count(&summary, 2);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf103792
fn mapped_pptx_tdf103792_keeps_visible_title_placeholder_text() {
  let summary = render_summary("pptx/tdf103792.pptx");
  assert_page_contains_in_order(&summary, 0, &["Click to add Title"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf119649
fn mapped_pptx_tdf119649_splits_colored_text_run_before_closing_parenthesis() {
  let summary = render_summary("pptx/tdf119649.pptx");
  assert_page_contains_in_order(&summary, 0, &["default_color(", "colored_text", ")"]);
  assert_text_fill_color(&summary, "colored_text", "#ce181e@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf103800
fn mapped_pptx_tdf103800_preserves_red_text_color() {
  let summary = render_summary("pptx/tdf103800.pptx");
  assert_page_contains_in_order(&summary, 0, &["test"]);
  assert_text_fill_color(&summary, "test", "#c00000@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf89927
fn mapped_pptx_tdf89927_preserves_white_text_color() {
  let summary = render_summary("pptx/tdf89927.pptx");
  assert_page_contains_in_order(&summary, 0, &["TEST"]);
  assert_text_fill_color(&summary, "TEST", "#ffffff@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testHyperlinkColor
fn mapped_pptx_tdf137367_preserves_hyperlink_text_colors() {
  let summary = render_summary("pptx/tdf137367.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "hyperlink color 1",
      "hyperlink color 2",
      "hyperlink color 3",
    ],
  );
  assert_has_text_fill_color(&summary, "#4472c4@ff");
  assert_has_text_fill_color(&summary, "#ff0000@ff");
  assert_has_text_fill_color(&summary, "#548235@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN828390_2
fn mapped_pptx_n828390_2_preserves_two_line_platform_text() {
  let summary = render_summary("pptx/n828390_2.pptx");
  assert_page_contains_in_order(&summary, 0, &["Linux", "Standard Platform"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf150719
fn mapped_pptx_tdf150719_preserves_underlined_text_decoration() {
  let summary = render_summary("pptx/tdf150719.pptx");
  assert_page_contains_in_order(&summary, 0, &["Jump", "to", "Slide 2"]);
  assert_page_has_horizontal_stroked_path(&summary, 0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf103876
fn mapped_pptx_tdf103876_preserves_placeholder_text_color() {
  let summary = render_summary("pptx/tdf103876.pptx");
  assert_has_text_fill_color(&summary, "#ff0000@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf104015
fn mapped_pptx_tdf104015_inherits_master_shape_fill_and_line() {
  let summary = render_summary("pptx/tdf104015.pptx");
  assert_has_path_fill_color(&summary, "#ff0000@ff");
  assert_has_stroked_path_color(&summary, "#0000ff@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf104201
fn mapped_pptx_tdf104201_uses_group_shape_green_fill() {
  let summary = render_summary("pptx/tdf104201.pptx");
  assert_has_path_fill_color(&summary, "#00ff00@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf103477
fn mapped_pptx_tdf103477_keeps_bullet_text_black() {
  let summary = render_summary("pptx/tdf103477.pptx");
  assert_page_contains_in_order(&summary, 0, &["nnnn"]);
  assert_has_text_fill_color(&summary, "#000000@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTableStyle
fn mapped_pptx_tdf156718_preserves_table_style_text_and_fill_colors() {
  let summary = render_summary("pptx/tdf156718.pptx");
  assert_has_text_fill_color(&summary, "#000000@ff");
  assert_has_path_fill_color(&summary, "#5b9bd5@ff");
  assert_page_has_stroked_path(&summary, 0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc584721_1
fn mapped_pptx_bnc584721_1_preserves_master_title_text() {
  let summary = render_summary("pptx/bnc584721_1_2.pptx");
  assert_page_contains_in_order(&summary, 0, &["Click to edit Master title style"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc584721_4
fn mapped_pptx_bnc584721_4_preserves_black_master_text() {
  let summary = render_summary("pptx/bnc584721_4.pptx");
  assert_has_text_fill_color(&summary, "#000000@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc904423
fn mapped_pptx_bnc904423_applies_fill_properties_in_upstream_order() {
  let summary = render_summary("pptx/bnc904423.pptx");
  assert_has_path_fill_color(&summary, "#00cc99@ff");
  assert_has_path_fill_color(&summary, "#3333cc@ff");
  assert_has_path_fill_color(&summary, "#ff0000@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testShapeLineStyle
fn mapped_pptx_shape_line_style_applies_line_properties_in_upstream_order() {
  let summary = render_summary("pptx/ShapeLineProperties.pptx");
  assert_has_stroked_path_color(&summary, "#ff0000@ff");
  assert_has_stroked_path_color(&summary, "#3333cc@ff");
  assert_has_stroked_path_color(&summary, "#7030a0@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc862510_6
fn mapped_pptx_bnc862510_6_preserves_gray_text_color() {
  let summary = render_summary("pptx/bnc862510_6.pptx");
  assert_has_text_fill_color(&summary, "#8b8b8b@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf127129
fn mapped_pptx_tdf127129_preserves_black_text_and_green_highlight() {
  let summary = render_summary("pptx/tdf127129.pptx");
  assert_has_text_fill_color(&summary, "#000000@ff");
  assert_has_path_fill_color(&summary, "#00ff00@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf151767
fn mapped_pptx_tdf151767_preserves_table_cell_borders() {
  let summary = render_summary("pptx/tdf151767.pptx");
  assert_page_has_stroked_path(&summary, 0);
}
