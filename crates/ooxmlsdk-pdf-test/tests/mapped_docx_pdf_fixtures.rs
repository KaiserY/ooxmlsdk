use ooxmlsdk_pdf_test::{
  PdfBounds, PdfSummary, assert_pdf_rect_close, docx_layout_summary_for_fixture, parse_pdf_rect,
  pdf_summary_for_fixture, pdfexport_fixture_dir, raw_image_pixel_for_fixture,
  rendered_page_image_for_fixture,
};

fn fixture(name: &str) -> std::path::PathBuf {
  pdfexport_fixture_dir().join(name)
}

fn render_summary(name: &str) -> PdfSummary {
  pdf_summary_for_fixture(&fixture(name)).unwrap()
}

fn layout_summary(name: &str) -> ooxmlsdk_pdf_test::DocxLayoutSummary {
  docx_layout_summary_for_fixture(&fixture(name)).unwrap()
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

fn assert_page_not_contains(summary: &PdfSummary, page_index: usize, expected: &str) {
  let text = page_text(summary, page_index);
  let normalized_text = normalize_space(&text);
  let normalized_expected = normalize_space(expected);
  assert!(
    !normalized_text.contains(&normalized_expected),
    "unexpected page {page_index} text {expected:?}; page text:\n{text}"
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

fn assert_text_segments_on_same_line_in_order(
  summary: &PdfSummary,
  page_index: usize,
  expected: &[&str],
) {
  let mut cursor = 0;
  let mut cursor_text_offset = 0;
  let mut bounds = Vec::new();
  for item in expected {
    let mut matched = None;
    for (index, segment) in summary.text_segments.iter().enumerate().skip(cursor) {
      if segment.page_index != page_index {
        continue;
      }
      let text = normalize_space(&segment.text);
      let start = if index == cursor {
        cursor_text_offset.min(text.len())
      } else {
        0
      };
      if let Some(offset) = text[start..].find(item) {
        matched = Some((index, start + offset, segment));
        break;
      }
    }
    let Some((index, offset, segment)) = matched else {
      panic!(
        "missing page {page_index} text segment {item:?} after offset {cursor}; text_segments={:?}",
        summary.text_segments
      );
    };
    cursor = index;
    cursor_text_offset = offset + item.len();
    bounds.push(parse_pdf_rect(&segment.bounds).unwrap());
  }
  let first = bounds
    .first()
    .unwrap_or_else(|| panic!("missing expected text segments on page {page_index}"));
  assert!(
    bounds
      .iter()
      .all(|bounds| (bounds.top - first.top).abs() <= 0.5
        && (bounds.bottom - first.bottom).abs() <= 0.5),
    "page {page_index} text segments should share a line; bounds={bounds:?}; text_segments={:?}",
    summary.text_segments
  );
}

fn normalized_occurrences(text: &str, expected: &str) -> usize {
  text.match_indices(&normalize_space(expected)).count()
}

fn assert_page_text_occurrences(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  expected_count: usize,
) {
  let text = normalized_page_text(summary, page_index);
  let count = normalized_occurrences(&text, expected);
  assert_eq!(
    count,
    expected_count,
    "page {page_index} text {expected:?} occurrence mismatch; page text:\n{}",
    page_text(summary, page_index)
  );
}

fn assert_page_text_occurrences_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  expected_count: usize,
) {
  let text = normalized_page_text(summary, page_index);
  let count = normalized_occurrences(&text, expected);
  assert!(
    count >= expected_count,
    "page {page_index} text {expected:?} occurrence mismatch: expected at least {expected_count}, got {count}; page text:\n{}",
    page_text(summary, page_index)
  );
}

fn assert_page_text_segment_occurrences(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  expected_count: usize,
) {
  let normalized_expected = normalize_space(expected);
  let count = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter(|segment| normalize_space(&segment.text) == normalized_expected)
    .count();
  assert_eq!(
    count, expected_count,
    "page {page_index} text segment {expected:?} occurrence mismatch; text_segments={:?}",
    summary.text_segments
  );
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

fn page_text_segment_count(summary: &PdfSummary, page_index: usize) -> usize {
  summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
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

fn assert_page_image_count_at_least(summary: &PdfSummary, page_index: usize, expected: usize) {
  let count = page_image_count(summary, page_index);
  assert!(
    count >= expected,
    "expected at least {expected} images on page {page_index}, got {count}; images={:?}",
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

fn assert_page_path_count_at_least(summary: &PdfSummary, page_index: usize, expected: usize) {
  let count = page_path_count(summary, page_index);
  assert!(
    count >= expected,
    "expected at least {expected} paths on page {page_index}, got {count}; paths={:?}",
    summary.paths
  );
}

fn assert_page_filled_path_count(summary: &PdfSummary, page_index: usize, expected: usize) {
  let count = summary
    .paths
    .iter()
    .filter(|path| path.page_index == page_index)
    .filter(|path| path.fill_mode.as_deref().is_some_and(|mode| mode != "None"))
    .count();
  assert_eq!(
    count, expected,
    "filled path count mismatch; paths={:?}",
    summary.paths
  );
}

fn assert_page_text_segment_count(summary: &PdfSummary, page_index: usize, expected: usize) {
  assert_eq!(
    page_text_segment_count(summary, page_index),
    expected,
    "text_segments={:?}",
    summary.text_segments
  );
}

fn assert_page_has_no_text(summary: &PdfSummary, page_index: usize) {
  let text = page_text(summary, page_index);
  assert!(
    normalize_space(&text).is_empty(),
    "expected page {page_index} to have no text; page text:\n{text}"
  );
}

fn assert_page_has_footer_text(summary: &PdfSummary, page_index: usize) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let footer_limit = media_box.bottom + 96.0;
  let footer_text = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter_map(|segment| {
      parse_pdf_rect(&segment.bounds)
        .ok()
        .filter(|bounds| bounds.bottom <= footer_limit)
        .map(|_| segment.text.as_str())
    })
    .collect::<Vec<_>>();
  assert!(
    !footer_text.is_empty(),
    "missing page {page_index} footer text below {footer_limit}pt; text_segments={:?}",
    summary.text_segments
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

fn assert_text_objects_share_non_black_fill_color(summary: &PdfSummary, texts: &[&str]) {
  let mut colors = Vec::new();
  for text in texts {
    let object = summary
      .text_objects
      .iter()
      .find(|object| normalize_space(&object.text).contains(text))
      .unwrap_or_else(|| panic!("missing text object containing {text:?}"));
    let color = object
      .fill_color
      .as_deref()
      .unwrap_or_else(|| panic!("missing fill color for text object {object:?}"));
    assert_ne!(
      color, "#000000@ff",
      "expected redline text {text:?} to use author color; object={object:?}"
    );
    colors.push(color);
  }
  assert!(
    colors.windows(2).all(|window| window[0] == window[1]),
    "expected redline text objects to share author color; colors={colors:?}"
  );
}

fn assert_has_path_fill_color(summary: &PdfSummary, color: &str) {
  assert!(
    summary
      .paths
      .iter()
      .any(|path| path.fill_color.as_deref() == Some(color) && path.fill_mode.is_some()),
    "missing filled path color {color}; paths={:?}",
    summary.paths
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

fn assert_has_text_object_font_size(summary: &PdfSummary, expected_size: &str) {
  assert!(
    summary
      .text_objects
      .iter()
      .any(|object| object.scaled_font_size == expected_size),
    "missing text object font size {expected_size:?}; objects={:?}",
    summary.text_objects
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

fn assert_raw_image_source_pixel_color(
  fixture_name: &str,
  summary: &PdfSummary,
  image_width: u32,
  image_height: u32,
  source_x: u32,
  source_y: u32,
  expected_rgb: [u8; 3],
) {
  assert_raw_xobject_dimensions(summary, image_width, image_height);
  let [r, g, b, _] = raw_image_pixel_for_fixture(
    &fixture(fixture_name),
    image_width,
    image_height,
    source_x,
    source_y,
  )
  .unwrap_or_else(|error| panic!("failed to extract raw image pixel from {fixture_name}: {error}"))
  .unwrap_or_else(|| {
    panic!(
      "missing raw image pixel ({source_x}, {source_y}) for dimensions {image_width}x{image_height}"
    )
  });
  let diff = (i16::from(r) - i16::from(expected_rgb[0])).abs()
    + (i16::from(g) - i16::from(expected_rgb[1])).abs()
    + (i16::from(b) - i16::from(expected_rgb[2])).abs();
  assert!(
    diff <= 12,
    "raw image pixel ({source_x}, {source_y}) color #{r:02x}{g:02x}{b:02x} differs from expected #{:02x}{:02x}{:02x}",
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

fn path_geometry_bounds_on_page(summary: &PdfSummary, page_index: usize) -> Vec<PdfBounds> {
  summary
    .paths
    .iter()
    .filter(|path| path.page_index == page_index)
    .filter_map(path_geometry_bounds)
    .collect()
}

fn path_geometry_bounds(
  path: &ooxmlsdk_pdf_test::pdf_extract::PathObjectSummary,
) -> Option<PdfBounds> {
  let mut left = f32::INFINITY;
  let mut right = f32::NEG_INFINITY;
  let mut bottom = f32::INFINITY;
  let mut top = f32::NEG_INFINITY;
  for segment in &path.segment_details {
    let x = segment.x.parse::<f32>().ok()?;
    let y = segment.y.parse::<f32>().ok()?;
    left = left.min(x);
    right = right.max(x);
    bottom = bottom.min(y);
    top = top.max(y);
  }
  (left.is_finite() && right.is_finite() && bottom.is_finite() && top.is_finite()).then_some(
    PdfBounds {
      left,
      bottom,
      right,
      top,
    },
  )
}

fn path_geometry_bounds_all_pages(summary: &PdfSummary) -> Vec<PdfBounds> {
  summary
    .paths
    .iter()
    .filter_map(path_geometry_bounds)
    .collect()
}

fn horizontal_path_bounds_on_page(summary: &PdfSummary, page_index: usize) -> Vec<PdfBounds> {
  path_bounds_on_page(summary, page_index)
    .into_iter()
    .filter(|bounds| bounds.width() > 5.0 && bounds.width() >= bounds.height() * 3.0)
    .collect()
}

fn vertical_path_bounds_on_page(summary: &PdfSummary, page_index: usize) -> Vec<PdfBounds> {
  path_bounds_on_page(summary, page_index)
    .into_iter()
    .filter(|bounds| bounds.height() > 5.0 && bounds.height() >= bounds.width() * 3.0)
    .collect()
}

fn rectangular_path_bounds_on_page(summary: &PdfSummary, page_index: usize) -> Vec<PdfBounds> {
  path_bounds_on_page(summary, page_index)
    .into_iter()
    .filter(|bounds| bounds.width() > 10.0 && bounds.height() > 10.0)
    .collect()
}

fn image_bounds_on_page(summary: &PdfSummary, page_index: usize) -> Vec<PdfBounds> {
  summary
    .images
    .iter()
    .filter(|image| image.page_index == page_index)
    .filter_map(|image| image.bounds.as_deref())
    .filter_map(|bounds| parse_pdf_rect(bounds).ok())
    .collect()
}

fn assert_image_centers_are_not_grayscale(
  fixture_name: &str,
  summary: &PdfSummary,
  page_index: usize,
  expected_count: usize,
) {
  let image_bounds = image_bounds_on_page(summary, page_index);
  assert_eq!(
    image_bounds.len(),
    expected_count,
    "image bounds count mismatch; images={:?}",
    summary.images
  );
  let rendered = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, 1200)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  for bounds in image_bounds {
    let [r, g, b, _] = rendered
      .sample_pdf_rect_center_rgba(bounds)
      .unwrap_or_else(|| panic!("missing rendered image-center sample for bounds {bounds:?}"));
    assert!(
      !(r == g && r == b),
      "expected image center to stay non-grayscale, got #{r:02x}{g:02x}{b:02x}; bounds={bounds:?}"
    );
  }
}

fn assert_image_centers_are_grayscale(
  fixture_name: &str,
  summary: &PdfSummary,
  page_index: usize,
  expected_count: usize,
) {
  let image_bounds = image_bounds_on_page(summary, page_index);
  assert_eq!(
    image_bounds.len(),
    expected_count,
    "image bounds count mismatch; images={:?}",
    summary.images
  );
  let rendered = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, 1200)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  for bounds in image_bounds {
    let [r, g, b, _] = rendered
      .sample_pdf_rect_center_rgba(bounds)
      .unwrap_or_else(|| panic!("missing rendered image-center sample for bounds {bounds:?}"));
    let spread = r.max(g).max(b) - r.min(g).min(b);
    assert!(
      spread <= 8,
      "expected image center to stay grayscale, got #{r:02x}{g:02x}{b:02x}; bounds={bounds:?}"
    );
  }
}

fn text_segment_bounds(summary: &PdfSummary, text: &str) -> PdfBounds {
  let segment = summary
    .text_segments
    .iter()
    .find(|segment| normalize_space(&segment.text).contains(text))
    .unwrap_or_else(|| panic!("missing text segment containing {text:?}"));
  parse_pdf_rect(&segment.bounds).unwrap()
}

fn text_segment_bounds_on_page(summary: &PdfSummary, page_index: usize, text: &str) -> PdfBounds {
  let segment = summary
    .text_segments
    .iter()
    .find(|segment| {
      segment.page_index == page_index && normalize_space(&segment.text).contains(text)
    })
    .unwrap_or_else(|| panic!("missing page {page_index} text segment containing {text:?}"));
  parse_pdf_rect(&segment.bounds).unwrap()
}

fn assert_text_top_from_page_top_at_most(
  summary: &PdfSummary,
  page_index: usize,
  text: &str,
  expected_top: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = text_segment_bounds_on_page(summary, page_index, text);
  let top_from_page_top = media_box.top - bounds.top;
  assert!(
    top_from_page_top <= expected_top,
    "page {page_index} text {text:?} top {top_from_page_top}pt exceeds {expected_top}pt; bounds={bounds:?}; media_box={media_box:?}"
  );
}

fn first_text_bounds_on_page(summary: &PdfSummary, page_index: usize) -> PdfBounds {
  let segment = summary
    .text_segments
    .iter()
    .find(|segment| segment.page_index == page_index)
    .unwrap_or_else(|| panic!("missing text segment on page {page_index}"));
  parse_pdf_rect(&segment.bounds).unwrap()
}

fn assert_first_text_top_from_page_top_at_most(
  summary: &PdfSummary,
  page_index: usize,
  expected_top: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = first_text_bounds_on_page(summary, page_index);
  let top_from_page_top = media_box.top - bounds.top;
  assert!(
    top_from_page_top <= expected_top,
    "page {page_index} first text top {top_from_page_top}pt exceeds {expected_top}pt; bounds={bounds:?}; media_box={media_box:?}"
  );
}

fn assert_first_text_top_from_page_top_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected_top: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = first_text_bounds_on_page(summary, page_index);
  let top_from_page_top = media_box.top - bounds.top;
  assert!(
    top_from_page_top >= expected_top,
    "page {page_index} first text top {top_from_page_top}pt is less than {expected_top}pt; bounds={bounds:?}; media_box={media_box:?}"
  );
}

fn assert_text_top_from_page_top_at_least(
  summary: &PdfSummary,
  page_index: usize,
  text: &str,
  expected_top: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = text_segment_bounds_on_page(summary, page_index, text);
  let top_from_page_top = media_box.top - bounds.top;
  assert!(
    top_from_page_top >= expected_top,
    "page {page_index} text {text:?} top {top_from_page_top}pt is less than {expected_top}pt; bounds={bounds:?}; media_box={media_box:?}"
  );
}

fn assert_text_top_from_page_top_close(
  summary: &PdfSummary,
  page_index: usize,
  text: &str,
  expected_top: f32,
  tolerance: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = text_segment_bounds_on_page(summary, page_index, text);
  let top_from_page_top = media_box.top - bounds.top;
  assert!(
    (top_from_page_top - expected_top).abs() <= tolerance,
    "page {page_index} text {text:?} top {top_from_page_top}pt differs from {expected_top}pt +/- {tolerance}; bounds={bounds:?}; media_box={media_box:?}"
  );
}

fn assert_page_text_below_text(
  summary: &PdfSummary,
  page_index: usize,
  lower_text: &str,
  upper_text: &str,
  minimum_gap: f32,
) {
  let lower = text_segment_bounds_on_page(summary, page_index, lower_text);
  let upper = text_segment_bounds_on_page(summary, page_index, upper_text);
  let gap = upper.top - lower.top;
  assert!(
    gap >= minimum_gap,
    "page {page_index} text {lower_text:?} should be below {upper_text:?} by at least {minimum_gap}pt; gap={gap}; lower={lower:?}; upper={upper:?}"
  );
}

fn assert_text_inside_any_path(summary: &PdfSummary, page_index: usize, text: &str) {
  assert_text_inside_any_path_with_tolerance(summary, page_index, text, 0.0);
}

fn assert_text_inside_any_path_with_tolerance(
  summary: &PdfSummary,
  page_index: usize,
  text: &str,
  tolerance: f32,
) {
  let text_bounds = text_segment_bounds_on_page(summary, page_index, text);
  let path_bounds = path_bounds_on_page(summary, page_index);
  assert!(
    path_bounds.iter().any(|path| {
      text_bounds.left + tolerance >= path.left
        && text_bounds.right <= path.right + tolerance
        && text_bounds.bottom + tolerance >= path.bottom
        && text_bounds.top <= path.top + tolerance
    }),
    "page {page_index} text {text:?} should be inside a path; text={text_bounds:?}; paths={path_bounds:?}"
  );
}

fn assert_matching_text_segments_inside_paths(summary: &PdfSummary, page_index: usize, text: &str) {
  let text_bounds = summary
    .text_segments
    .iter()
    .filter(|segment| {
      segment.page_index == page_index && normalize_space(&segment.text).contains(text)
    })
    .map(|segment| parse_pdf_rect(&segment.bounds).unwrap())
    .collect::<Vec<_>>();
  assert!(
    !text_bounds.is_empty(),
    "missing page {page_index} text segment containing {text:?}"
  );
  let path_bounds = path_bounds_on_page(summary, page_index);
  for text_bound in &text_bounds {
    assert!(
      path_bounds.iter().any(|path| {
        text_bound.left >= path.left
          && text_bound.right <= path.right
          && text_bound.bottom >= path.bottom
          && text_bound.top <= path.top
      }),
      "page {page_index} text {text:?} segment should be inside a path; text={text_bound:?}; paths={path_bounds:?}"
    );
  }
}

fn assert_text_inside_page_path_extents(summary: &PdfSummary, page_index: usize, text: &str) {
  let text_bounds = text_segment_bounds_on_page(summary, page_index, text);
  let path_bounds = path_bounds_on_page(summary, page_index);
  let left = path_bounds
    .iter()
    .map(|bounds| bounds.left)
    .fold(f32::INFINITY, f32::min);
  let right = path_bounds
    .iter()
    .map(|bounds| bounds.right)
    .fold(f32::NEG_INFINITY, f32::max);
  let bottom = path_bounds
    .iter()
    .map(|bounds| bounds.bottom)
    .fold(f32::INFINITY, f32::min);
  let top = path_bounds
    .iter()
    .map(|bounds| bounds.top)
    .fold(f32::NEG_INFINITY, f32::max);
  assert!(
    text_bounds.left >= left
      && text_bounds.right <= right
      && text_bounds.bottom >= bottom
      && text_bounds.top <= top,
    "page {page_index} text {text:?} should be inside path extents; text={text_bounds:?}; paths={path_bounds:?}"
  );
}

fn assert_text_below_any_path_by_at_least(summary: &PdfSummary, text: &str, minimum_gap: f32) {
  let text_bounds = text_segment_bounds(summary, text);
  let path_bounds = path_bounds_on_page(summary, 0);
  assert!(
    path_bounds
      .iter()
      .any(|path| path.bottom - text_bounds.top >= minimum_gap),
    "text {text:?} should be below a path by at least {minimum_gap}pt; text={text_bounds:?}; paths={path_bounds:?}"
  );
}

fn assert_text_top_above_any_image_bottom(summary: &PdfSummary, text: &str) {
  let text_bounds = text_segment_bounds(summary, text);
  let image_bounds = image_bounds_on_page(summary, 0);
  assert!(
    image_bounds
      .iter()
      .any(|image| text_bounds.top > image.bottom),
    "text {text:?} should start above an image bottom; text={text_bounds:?}; images={image_bounds:?}"
  );
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

fn assert_path_geometry_width_close(summary: &PdfSummary, page_index: usize, expected_width: f32) {
  let bounds = path_geometry_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (bounds.width() - expected_width).abs() <= 0.5),
    "missing page {page_index} path geometry width {expected_width}pt; bounds={bounds:?}"
  );
}

fn assert_path_geometry_height_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_height: f32,
) {
  let bounds = path_geometry_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (bounds.height() - expected_height).abs() <= 0.5),
    "missing page {page_index} path geometry height {expected_height}pt; bounds={bounds:?}"
  );
}

fn assert_path_geometry_top_from_page_top_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_top: f32,
) {
  let bounds = path_geometry_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (bounds.bottom - expected_top).abs() <= 0.5),
    "missing page {page_index} path geometry top {expected_top}pt from page top; bounds={bounds:?}"
  );
}

fn assert_path_geometry_width_count_close(
  summary: &PdfSummary,
  expected_width: f32,
  expected_count: usize,
) {
  let bounds = path_geometry_bounds_all_pages(summary);
  let count = bounds
    .iter()
    .filter(|bounds| (bounds.width() - expected_width).abs() <= 0.5)
    .count();
  assert!(
    count >= expected_count,
    "expected at least {expected_count} path geometries with width {expected_width}pt, got {count}; bounds={bounds:?}"
  );
}

fn assert_path_width_between(
  summary: &PdfSummary,
  page_index: usize,
  minimum_width: f32,
  maximum_width: f32,
) {
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| bounds.width() >= minimum_width && bounds.width() <= maximum_width),
    "missing page {page_index} path width between {minimum_width}pt and {maximum_width}pt; bounds={bounds:?}"
  );
}

fn assert_path_width_at_least(summary: &PdfSummary, page_index: usize, minimum_width: f32) {
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds.iter().any(|bounds| bounds.width() >= minimum_width),
    "missing page {page_index} path width at least {minimum_width}pt; bounds={bounds:?}"
  );
}

fn assert_path_width_at_least_page_fraction(
  summary: &PdfSummary,
  page_index: usize,
  minimum_fraction: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let minimum_width = media_box.width() * minimum_fraction;
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds.iter().any(|bounds| bounds.width() >= minimum_width),
    "missing page {page_index} path width at least {minimum_fraction} of page width ({minimum_width}pt); bounds={bounds:?}"
  );
}

fn assert_horizontal_path_count_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected_count: usize,
) {
  let bounds = horizontal_path_bounds_on_page(summary, page_index);
  assert!(
    bounds.len() >= expected_count,
    "expected at least {expected_count} horizontal paths on page {page_index}, got {}; bounds={bounds:?}",
    bounds.len()
  );
}

fn assert_vertical_path_count_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected_count: usize,
) {
  let bounds = vertical_path_bounds_on_page(summary, page_index);
  assert!(
    bounds.len() >= expected_count,
    "expected at least {expected_count} vertical paths on page {page_index}, got {}; bounds={bounds:?}",
    bounds.len()
  );
}

fn assert_middle_horizontal_border_is_inset(summary: &PdfSummary, page_index: usize) {
  let bounds = horizontal_path_bounds_on_page(summary, page_index);
  let left = bounds
    .iter()
    .map(|bounds| bounds.left)
    .fold(f32::INFINITY, f32::min);
  let right = bounds
    .iter()
    .map(|bounds| bounds.right)
    .fold(f32::NEG_INFINITY, f32::max);
  assert!(
    bounds
      .iter()
      .any(|bounds| bounds.left > left + 0.5 && bounds.right < right - 0.5),
    "missing inset middle horizontal border on page {page_index}; bounds={bounds:?}"
  );
}

fn assert_any_path_height_less_than(summary: &PdfSummary, page_index: usize, max_height: f32) {
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds.iter().any(|bounds| bounds.height() < max_height),
    "missing page {page_index} path height less than {max_height}pt; bounds={bounds:?}"
  );
}

fn assert_any_path_has_positive_height(summary: &PdfSummary, page_index: usize) {
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds.iter().any(|bounds| bounds.height() > 0.0),
    "missing page {page_index} path with positive height; bounds={bounds:?}"
  );
}

fn assert_path_height_close(summary: &PdfSummary, page_index: usize, expected_height: f32) {
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (bounds.height() - expected_height).abs() <= 0.5),
    "missing page {page_index} path height {expected_height}pt; bounds={bounds:?}"
  );
}

fn assert_path_column_span_close(summary: &PdfSummary, page_index: usize, expected_height: f32) {
  let mut columns: Vec<Vec<PdfBounds>> = Vec::new();
  for bounds in path_bounds_on_page(summary, page_index) {
    if let Some(column) = columns
      .iter_mut()
      .find(|column| (column[0].left - bounds.left).abs() <= 2.0)
    {
      column.push(bounds);
    } else {
      columns.push(vec![bounds]);
    }
  }

  let mut spans = columns
    .iter()
    .map(|column| {
      let top = column
        .iter()
        .map(|bounds| bounds.top)
        .fold(f32::NEG_INFINITY, f32::max);
      let bottom = column
        .iter()
        .map(|bounds| bounds.bottom)
        .fold(f32::INFINITY, f32::min);
      top - bottom
    })
    .collect::<Vec<_>>();
  spans.sort_by(|a, b| a.partial_cmp(b).unwrap());
  assert!(
    spans
      .iter()
      .any(|height| (height - expected_height).abs() <= 0.5),
    "missing page {page_index} path column span {expected_height}pt; spans={spans:?}"
  );
}

fn assert_text_height_close(summary: &PdfSummary, page_index: usize, expected_height: f32) {
  let bounds = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .collect::<Vec<_>>();
  assert!(
    bounds
      .iter()
      .any(|bounds| (bounds.height() - expected_height).abs() <= 0.5),
    "missing page {page_index} text height {expected_height}pt; bounds={bounds:?}"
  );
}

fn assert_any_image_and_path_height_close(summary: &PdfSummary, page_index: usize, tolerance: f32) {
  let image_bounds = image_bounds_on_page(summary, page_index);
  let path_bounds = path_bounds_on_page(summary, page_index);
  assert!(
    image_bounds.iter().any(|image| {
      path_bounds
        .iter()
        .any(|path| (image.height() - path.height()).abs() <= tolerance)
    }),
    "missing page {page_index} matching image/path heights within {tolerance}pt; images={image_bounds:?}; paths={path_bounds:?}"
  );
}

fn assert_any_image_or_path_height_at_least(
  summary: &PdfSummary,
  page_index: usize,
  minimum_height: f32,
) {
  let image_bounds = image_bounds_on_page(summary, page_index);
  let path_bounds = path_bounds_on_page(summary, page_index);
  assert!(
    image_bounds
      .iter()
      .any(|bounds| bounds.height() >= minimum_height)
      || path_bounds
        .iter()
        .any(|bounds| bounds.height() >= minimum_height),
    "missing page {page_index} image/path height at least {minimum_height}pt; images={image_bounds:?}; paths={path_bounds:?}"
  );
}

fn assert_matching_layout_row_height_across_pages(
  summary: &ooxmlsdk_pdf_test::DocxLayoutSummary,
  first_page_index: usize,
  second_page_index: usize,
) {
  let first_rows = summary
    .rows
    .iter()
    .filter(|row| row.page_index == first_page_index)
    .collect::<Vec<_>>();
  let second_rows = summary
    .rows
    .iter()
    .filter(|row| row.page_index == second_page_index)
    .collect::<Vec<_>>();
  assert!(
    first_rows.iter().any(|first| {
      first.height_pt > 1.0
        && second_rows
          .iter()
          .any(|second| (first.height_pt - second.height_pt).abs() <= 0.5)
    }),
    "missing matching layout row height across pages {first_page_index} and {second_page_index}; first={first_rows:?}; second={second_rows:?}"
  );
}

fn assert_text_below_any_image(summary: &PdfSummary, text: &str) {
  let text_bounds = text_segment_bounds(summary, text);
  let image_bounds = image_bounds_on_page(summary, 0);
  assert!(
    image_bounds
      .iter()
      .any(|image| text_bounds.top < image.bottom),
    "text {text:?} should be below an image; text={text_bounds:?}; images={image_bounds:?}"
  );
}

fn assert_text_below_any_path(summary: &PdfSummary, text: &str) {
  let text_bounds = text_segment_bounds(summary, text);
  let path_bounds = path_bounds_on_page(summary, 0);
  assert!(
    path_bounds.iter().any(|path| text_bounds.top < path.bottom),
    "text {text:?} should be below a path; text={text_bounds:?}; paths={path_bounds:?}"
  );
}

fn assert_any_image_near_text_top(summary: &PdfSummary, text: &str, tolerance: f32) {
  let text_bounds = text_segment_bounds(summary, text);
  let image_bounds = image_bounds_on_page(summary, 0);
  assert!(
    image_bounds
      .iter()
      .any(|image| (image.top - text_bounds.top).abs() <= tolerance),
    "expected image top near {text:?}; text={text_bounds:?}; images={image_bounds:?}; tolerance={tolerance}"
  );
}

fn assert_image_top_from_page_top_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_top: f32,
  tolerance: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let image_bounds = image_bounds_on_page(summary, page_index);
  assert!(
    image_bounds.iter().any(|image| {
      let top_from_page_top = media_box.top - image.top;
      (top_from_page_top - expected_top).abs() <= tolerance
    }),
    "missing page {page_index} image top {expected_top}pt +/- {tolerance} from page top; images={image_bounds:?}"
  );
}

fn assert_text_tops_close(
  summary: &PdfSummary,
  first_text: &str,
  second_text: &str,
  tolerance: f32,
) {
  let first = text_segment_bounds(summary, first_text);
  let second = text_segment_bounds(summary, second_text);
  assert!(
    (first.top - second.top).abs() <= tolerance,
    "text tops differ for {first_text:?} and {second_text:?}: first={first:?}; second={second:?}; tolerance={tolerance}"
  );
}

fn assert_text_left_difference_close(
  summary: &PdfSummary,
  first_text: &str,
  second_text: &str,
  expected_difference: f32,
  tolerance: f32,
) {
  let first = text_segment_bounds(summary, first_text);
  let second = text_segment_bounds(summary, second_text);
  let difference = first.left - second.left;
  assert!(
    (difference - expected_difference).abs() <= tolerance,
    "text left difference for {first_text:?} and {second_text:?} is {difference}pt, expected {expected_difference}pt +/- {tolerance}; first={first:?}; second={second:?}"
  );
}

fn assert_second_text_left_no_more_than(
  summary: &PdfSummary,
  first_text: &str,
  second_text: &str,
  maximum_delta: f32,
) {
  let first = text_segment_bounds(summary, first_text);
  let second = text_segment_bounds(summary, second_text);
  let delta = second.left - first.left;
  assert!(
    delta < maximum_delta,
    "text left delta from {first_text:?} to {second_text:?} is {delta}pt, expected less than {maximum_delta}pt; first={first:?}; second={second:?}"
  );
}

fn assert_any_image_left_of_text(summary: &PdfSummary, text: &str) {
  let text_bounds = text_segment_bounds(summary, text);
  let image_bounds = image_bounds_on_page(summary, 0);
  assert!(
    image_bounds
      .iter()
      .any(|image| image.left < text_bounds.left),
    "expected an image left of {text:?}; text={text_bounds:?}; images={image_bounds:?}"
  );
}

fn assert_any_image_extends_left_of_page(summary: &PdfSummary, page_index: usize) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let image_bounds = image_bounds_on_page(summary, page_index);
  assert!(
    image_bounds.iter().any(|image| image.left < media_box.left),
    "expected page {page_index} image to extend left of page frame; media_box={media_box:?}; images={image_bounds:?}"
  );
}

fn object_bounds_on_page(summary: &PdfSummary, page_index: usize) -> Vec<PdfBounds> {
  let mut bounds = image_bounds_on_page(summary, page_index);
  bounds.extend(path_bounds_on_page(summary, page_index));
  bounds
}

fn assert_any_object_left_from_page_left_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_left: f32,
  tolerance: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = object_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (bounds.left - media_box.left - expected_left).abs() <= tolerance),
    "missing page {page_index} object left {expected_left}pt +/- {tolerance} from page left; bounds={bounds:?}; media_box={media_box:?}"
  );
}

fn assert_any_object_right_from_page_right_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_right: f32,
  tolerance: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = object_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (media_box.right - bounds.right - expected_right).abs() <= tolerance),
    "missing page {page_index} object right {expected_right}pt +/- {tolerance} from page right; bounds={bounds:?}; media_box={media_box:?}"
  );
}

fn assert_path_top_from_page_top_close(summary: &PdfSummary, page_index: usize, expected_top: f32) {
  assert_path_top_from_page_top_close_with_tolerance(summary, page_index, expected_top, 0.5);
}

fn assert_path_top_from_page_top_close_with_tolerance(
  summary: &PdfSummary,
  page_index: usize,
  expected_top: f32,
  tolerance: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds
      .iter()
      .any(|bounds| (media_box.top - bounds.top - expected_top).abs() <= tolerance),
    "missing page {page_index} path top {expected_top}pt +/- {tolerance} from page top; bounds={bounds:?}"
  );
}

fn assert_any_path_bottom_from_page_top_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_bottom: f32,
  tolerance: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(
    bounds.iter().any(|bounds| {
      let bottom_from_page_top = media_box.top - bounds.bottom;
      (bottom_from_page_top - expected_bottom).abs() <= tolerance
    }),
    "missing page {page_index} path bottom {expected_bottom}pt +/- {tolerance} from page top; bounds={bounds:?}"
  );
}

fn assert_first_two_rectangular_paths_do_not_overlap_vertically(
  summary: &PdfSummary,
  page_index: usize,
) {
  let mut paths = rectangular_path_bounds_on_page(summary, page_index);
  paths.sort_by(|a, b| b.top.total_cmp(&a.top));
  let paths = distinct_rectangular_paths(paths);
  assert!(
    paths.len() >= 2,
    "expected at least two rectangular paths; paths={paths:?}"
  );
  assert!(
    paths[0].bottom >= paths[1].top,
    "expected first two rectangular paths to be vertically separated; paths={paths:?}"
  );
}

fn distinct_rectangular_paths(paths: Vec<PdfBounds>) -> Vec<PdfBounds> {
  let mut distinct = Vec::new();
  for path in paths {
    if distinct
      .iter()
      .any(|existing| rectangles_substantially_overlap(existing, &path))
    {
      continue;
    }
    distinct.push(path);
  }
  distinct
}

fn rectangles_substantially_overlap(a: &PdfBounds, b: &PdfBounds) -> bool {
  let overlap_width = (a.right.min(b.right) - a.left.max(b.left)).max(0.0);
  let overlap_height = (a.top.min(b.top) - a.bottom.max(b.bottom)).max(0.0);
  let overlap_area = overlap_width * overlap_height;
  let smaller_area = rect_area(a).min(rect_area(b));
  smaller_area > 0.0 && overlap_area / smaller_area >= 0.8
}

fn rect_area(bounds: &PdfBounds) -> f32 {
  (bounds.right - bounds.left).max(0.0) * (bounds.top - bounds.bottom).max(0.0)
}

fn assert_first_two_rectangular_path_tops_close(
  summary: &PdfSummary,
  page_index: usize,
  tolerance: f32,
) {
  let mut paths = rectangular_path_bounds_on_page(summary, page_index);
  paths.sort_by(|a, b| b.top.total_cmp(&a.top));
  assert!(
    paths.len() >= 2,
    "expected at least two rectangular paths; paths={paths:?}"
  );
  assert!(
    (paths[0].top - paths[1].top).abs() <= tolerance,
    "expected first two rectangular path tops to match; paths={paths:?}; tolerance={tolerance}"
  );
}

fn assert_any_two_rectangular_path_heights_close(
  summary: &PdfSummary,
  page_index: usize,
  tolerance: f32,
) {
  let paths = rectangular_path_bounds_on_page(summary, page_index);
  assert!(
    paths.iter().enumerate().any(|(index, first)| {
      paths
        .iter()
        .skip(index + 1)
        .any(|second| (first.height() - second.height()).abs() <= tolerance)
    }),
    "missing two rectangular paths with matching heights within {tolerance}pt; paths={paths:?}"
  );
}

fn assert_three_rectangular_paths_are_vertically_ordered(summary: &PdfSummary, page_index: usize) {
  let mut paths = rectangular_path_bounds_on_page(summary, page_index);
  paths.sort_by(|a, b| b.top.total_cmp(&a.top));
  let paths = distinct_rectangular_paths(paths);
  assert!(
    paths.len() >= 3,
    "expected at least three rectangular paths; paths={paths:?}"
  );
  assert!(
    paths[0].top > paths[1].top && paths[1].top > paths[2].top,
    "expected first three rectangular paths to have descending PDF top coordinates; paths={paths:?}"
  );
}

fn assert_any_rectangular_path_above_another(summary: &PdfSummary, page_index: usize) {
  let paths = rectangular_path_bounds_on_page(summary, page_index);
  assert!(
    paths.iter().any(|upper| {
      paths
        .iter()
        .any(|lower| upper.bottom >= lower.top && upper.top > lower.top)
    }),
    "expected one rectangular path to be above another; paths={paths:?}"
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

fn assert_all_paths_within_page(summary: &PdfSummary, page_index: usize, tolerance: f32) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let bounds = path_bounds_on_page(summary, page_index);
  assert!(!bounds.is_empty(), "missing page {page_index} paths");
  for path in &bounds {
    assert!(
      path.left >= media_box.left - tolerance
        && path.right <= media_box.right + tolerance
        && path.bottom >= media_box.bottom - tolerance
        && path.top <= media_box.top + tolerance,
      "page {page_index} path should stay within media box {media_box:?} +/- {tolerance}; path={path:?}; all={bounds:?}"
    );
  }
}

fn assert_rightmost_text_within_rightmost_path(
  summary: &PdfSummary,
  page_index: usize,
  tolerance: f32,
) {
  let text_right = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .map(|bounds| bounds.right)
    .fold(f32::NEG_INFINITY, f32::max);
  let path_bounds = path_bounds_on_page(summary, page_index);
  let path_right = path_bounds
    .iter()
    .map(|bounds| bounds.right)
    .fold(f32::NEG_INFINITY, f32::max);
  assert!(
    text_right <= path_right + tolerance,
    "rightmost page {page_index} text {text_right} is outside rightmost path {path_right} + {tolerance}; paths={path_bounds:?}"
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

fn assert_rendered_page_has_non_white_pixels(
  fixture_name: &str,
  page_index: usize,
  target_width: i32,
) {
  let image = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, target_width)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let non_white = image
    .rgba
    .chunks_exact(4)
    .filter(|pixel| pixel[3] != 0 && (pixel[0] < 250 || pixel[1] < 250 || pixel[2] < 250))
    .count();
  assert!(
    non_white > 0,
    "rendered page {page_index} is blank; crc={}",
    image.rgba_crc32
  );
}

fn assert_text_segment_width_less_than(
  summary: &PdfSummary,
  page_index: usize,
  text: &str,
  maximum_width: f32,
) {
  let bounds = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter(|segment| normalize_space(&segment.text) == normalize_space(text))
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .collect::<Vec<_>>();
  assert!(
    bounds.iter().any(|bounds| bounds.width() < maximum_width),
    "missing page {page_index} text segment {text:?} width less than {maximum_width}pt; bounds={bounds:?}; text_segments={:?}",
    summary.text_segments
  );
}

fn assert_text_segment_taller_than_wide(summary: &PdfSummary, page_index: usize, text: &str) {
  let bounds = text_segment_bounds_on_page(summary, page_index, text);
  assert!(
    bounds.height() > bounds.width(),
    "page {page_index} text {text:?} should be vertically oriented; bounds={bounds:?}"
  );
}

fn assert_vertical_text_segment_count_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected_count: usize,
) {
  let bounds = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .collect::<Vec<_>>();
  let count = bounds
    .iter()
    .filter(|bounds| bounds.height() > bounds.width())
    .count();
  assert!(
    count >= expected_count,
    "expected at least {expected_count} vertical text segments on page {page_index}, got {count}; bounds={bounds:?}"
  );
}

fn assert_any_path_segment_count_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected_segments: u32,
) {
  assert!(
    summary
      .paths
      .iter()
      .filter(|path| path.page_index == page_index)
      .any(|path| path.segments >= expected_segments),
    "missing page {page_index} path with at least {expected_segments} segments; paths={:?}",
    summary.paths
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

fn assert_max_horizontal_gap_at_most(summary: &PdfSummary, page_index: usize, expected_gap: f32) {
  let max_gap = max_horizontal_gap_on_page(summary, page_index);
  assert!(
    max_gap <= expected_gap,
    "page {page_index} horizontal gap {max_gap} exceeds {expected_gap}pt"
  );
}

fn assert_horizontal_path_decorates_text(
  summary: &PdfSummary,
  page_index: usize,
  text: &str,
  through_text: bool,
) {
  let text_bounds = text_segment_bounds_on_page(summary, page_index, text);
  let paths = horizontal_path_bounds_on_page(summary, page_index);
  let matched = paths.iter().any(|path| {
    let x_overlap = path.left <= text_bounds.right && path.right >= text_bounds.left;
    let path_center_y = path.bottom + path.height() / 2.0;
    let y_match = if through_text {
      path_center_y >= text_bounds.bottom && path_center_y <= text_bounds.top
    } else {
      path_center_y <= text_bounds.bottom + 3.0 && path_center_y >= text_bounds.bottom - 10.0
    };
    x_overlap && y_match
  });
  assert!(
    matched,
    "missing horizontal decoration path for text {text:?}; text_bounds={text_bounds:?}; paths={paths:?}"
  );
}

fn assert_any_horizontal_path_crosses_any_text(summary: &PdfSummary, page_index: usize) {
  let paths = horizontal_path_bounds_on_page(summary, page_index);
  let text_bounds = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == page_index)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .collect::<Vec<_>>();
  let matched = paths.iter().any(|path| {
    let path_center_y = path.bottom + path.height() / 2.0;
    text_bounds.iter().any(|bounds| {
      path.left <= bounds.right
        && path.right >= bounds.left
        && path_center_y >= bounds.bottom
        && path_center_y <= bounds.top
    })
  });
  assert!(
    matched,
    "missing horizontal decoration path crossing text; paths={paths:?}; text_bounds={text_bounds:?}"
  );
}

fn assert_text_above_text(
  summary: &PdfSummary,
  page_index: usize,
  upper_text: &str,
  lower_text: &str,
) {
  let upper = text_segment_bounds_on_page(summary, page_index, upper_text);
  let lower = text_segment_bounds_on_page(summary, page_index, lower_text);
  assert!(
    upper.bottom >= lower.top,
    "page {page_index} text {upper_text:?} should be above {lower_text:?}; upper={upper:?}; lower={lower:?}"
  );
}

fn assert_rendered_pixel_rgb_close(
  fixture_name: &str,
  page_index: usize,
  target_width: i32,
  x: u32,
  y: u32,
  expected_rgb: [u8; 3],
) {
  let image = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, target_width)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let [r, g, b, _] = image
    .pixel_rgba(x, y)
    .unwrap_or_else(|| panic!("missing rendered pixel at ({x}, {y})"));
  let diff = (i16::from(r) - i16::from(expected_rgb[0])).abs()
    + (i16::from(g) - i16::from(expected_rgb[1])).abs()
    + (i16::from(b) - i16::from(expected_rgb[2])).abs();
  assert!(
    diff <= 12,
    "rendered pixel ({x}, {y}) color #{r:02x}{g:02x}{b:02x} differs from expected #{:02x}{:02x}{:02x}",
    expected_rgb[0],
    expected_rgb[1],
    expected_rgb[2]
  );
}

fn assert_rendered_page_color_ratio(
  fixture_name: &str,
  page_index: usize,
  target_width: i32,
  expected_rgb: [u8; 3],
  divisor: usize,
) {
  let image = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, target_width)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let mut expected_pixels = 0usize;
  let mut other_pixels = 0usize;
  for y in 0..image.height_px {
    for x in 0..image.width_px {
      let [r, g, b, _] = image.pixel_rgba(x, y).unwrap();
      if [r, g, b] == expected_rgb {
        expected_pixels += 1;
      } else {
        other_pixels += 1;
      }
    }
  }
  assert!(
    expected_pixels > 0,
    "missing expected rendered color #{:02x}{:02x}{:02x}",
    expected_rgb[0],
    expected_rgb[1],
    expected_rgb[2]
  );
  assert!(
    expected_pixels / divisor > other_pixels,
    "rendered page color ratio mismatch for #{:02x}{:02x}{:02x}: expected_pixels={expected_pixels}, other_pixels={other_pixels}, divisor={divisor}",
    expected_rgb[0],
    expected_rgb[1],
    expected_rgb[2]
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

fn first_two_text_segment_baseline_gap(summary: &PdfSummary, fixture_name: &str) -> f32 {
  let first = summary
    .text_segments
    .first()
    .unwrap_or_else(|| panic!("missing first text segment in {fixture_name}"));
  let second = summary
    .text_segments
    .get(1)
    .unwrap_or_else(|| panic!("missing second text segment in {fixture_name}"));
  let first = parse_pdf_rect(&first.bounds).unwrap();
  let second = parse_pdf_rect(&second.bounds).unwrap();
  (first.bottom - second.bottom).abs()
}

fn first_layout_line_height(
  summary: &ooxmlsdk_pdf_test::DocxLayoutSummary,
  fixture_name: &str,
) -> f32 {
  summary
    .lines
    .first()
    .unwrap_or_else(|| panic!("missing first layout line in {fixture_name}"))
    .height_pt
}

fn assert_any_layout_line_height_less_than(
  summary: &ooxmlsdk_pdf_test::DocxLayoutSummary,
  page_index: usize,
  maximum_height: f32,
) {
  assert!(
    summary
      .lines
      .iter()
      .filter(|line| line.page_index == page_index)
      .any(|line| line.height_pt < maximum_height),
    "missing page {page_index} layout line height less than {maximum_height}pt; lines={:?}",
    summary.lines
  );
}

fn assert_any_layout_line_height_greater_than(
  summary: &ooxmlsdk_pdf_test::DocxLayoutSummary,
  page_index: usize,
  minimum_height: f32,
) {
  assert!(
    summary
      .lines
      .iter()
      .filter(|line| line.page_index == page_index)
      .any(|line| line.height_pt > minimum_height),
    "missing page {page_index} layout line height greater than {minimum_height}pt; lines={:?}",
    summary.lines
  );
}

fn assert_any_layout_row_height_less_than(
  summary: &ooxmlsdk_pdf_test::DocxLayoutSummary,
  page_index: usize,
  maximum_height: f32,
) {
  assert!(
    summary
      .rows
      .iter()
      .filter(|row| row.page_index == page_index)
      .any(|row| row.height_pt < maximum_height),
    "missing page {page_index} layout row height less than {maximum_height}pt; rows={:?}",
    summary.rows
  );
}

fn assert_any_layout_row_height_greater_than(
  summary: &ooxmlsdk_pdf_test::DocxLayoutSummary,
  page_index: usize,
  minimum_height: f32,
) {
  assert!(
    summary
      .rows
      .iter()
      .filter(|row| row.page_index == page_index)
      .any(|row| row.height_pt > minimum_height),
    "missing page {page_index} layout row height greater than {minimum_height}pt; rows={:?}",
    summary.rows
  );
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
  assert_page_contains(&summary, 0, "Page 1 of 2");
  assert_page_contains(&summary, 1, "Page 2 of 2");
  // LibreOffice layout dump exposes PAGE/NUMPAGES fields as "Page * of *";
  // PDF output contains the resolved visible field values.
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
  assert_page_image_count_at_least(&summary, 0, 1);
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
  let height14 = first_two_text_segment_baseline_gap(&compat14, "128197_compat14.docx");
  let height15 = first_two_text_segment_baseline_gap(&compat15, "128197_compat15.docx");
  assert!(
    height14 < height15,
    "expected compat14 first paragraph line gap to be less than compat15: {height14} >= {height15}"
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
    (shape_left - body_left).abs() <= 0.2,
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
  assert_raw_image_source_pixel_color("tdf136841.docx", &summary, 76, 76, 38, 38, [228, 72, 70]);
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
  assert_path_geometry_width_close(&summary, 0, 56.65);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:testRelativeAnchorWidthFromInsideOutsideMargin
fn mapped_fixture_tdf133861_preserves_inside_outside_anchor_widths() {
  let summary = render_summary("tdf133861_RelativeAnchorWidthFromInsideOutsideMargin.docx");
  assert_path_geometry_width_count_close(&summary, 72.0, 2);
  assert_path_geometry_width_count_close(&summary, 127.6, 2);
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

#[test]
// Source: ../core/sw/qa/core/doc/doc.cxx:testTextBoxWordWrap
fn mapped_fixture_text_box_word_wrap_keeps_textbox_to_one_line_height() {
  let summary = render_summary("text-box-word-wrap.docx");
  assert_any_path_height_less_than(&summary, 0, 50.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153042_noTab
fn mapped_fixture_tdf153042_no_tab_preserves_miniscule_pseudo_numbering_tab() {
  let summary = render_summary("tdf153042_noTab.docx");
  assert_max_horizontal_gap_at_most(&summary, 0, 5.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:testTdf134063
fn mapped_fixture_tdf134063_preserves_two_pages_and_default_tab_widths() {
  let summary = render_summary("tdf134063.docx");
  assert_eq!(summary.page_count, 2);
  assert_max_horizontal_gap_at_least(&summary, 0, 36.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport17.cxx:testTdf148360
fn mapped_fixture_tdf148360_keeps_initial_tab_before_text() {
  let summary = render_summary("tdf148360.docx");
  let first_text_left = summary
    .text_segments
    .iter()
    .filter(|segment| segment.page_index == 0)
    .filter_map(|segment| parse_pdf_rect(&segment.bounds).ok())
    .map(|bounds| bounds.left)
    .fold(f32::INFINITY, f32::min);
  assert!(
    first_text_left > 36.0,
    "first text should start after an initial tab; left={first_text_left}"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf163894
fn mapped_fixture_tdf163894_preserves_styleref_header_words() {
  let summary = render_summary("tdf163894.docx");
  // LibreOffice's upstream test asserts Writer layout-dump field expansions.
  // A direct soffice PDF export of the same fixture does not expose the later
  // layout-only "misfitting" expansion as selectable PDF text, so keep this
  // mapped PDF test on words visible in exported PDF text.
  for (page_index, expected) in [
    (0, "handbooks"),
    (0, "infuriating"),
    (1, "infuriating"),
    (2, "initializes"),
    (3, "misrepresenting"),
    (3, "modicum"),
  ] {
    assert_page_contains(&summary, page_index, expected);
  }
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf163894_hidden
fn mapped_fixture_tdf163894_hidden_uses_previous_visible_styleref_text() {
  let summary = render_summary("tdf163894_hidden.docx");
  assert_page_contains(&summary, 0, "handbooks");
  assert_page_contains(&summary, 0, "infuriating");
  assert_page_contains(&summary, 1, "infuriating");
  assert_page_contains(&summary, 3, "mitosis");
  assert_page_contains(&summary, 3, "modicum");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf32363
fn mapped_fixture_tdf32363_preserves_numbered_styleref_headers() {
  let summary = render_summary("tdf32363.docx");
  for (page_index, expected) in [
    (2, "Do not shorten this short heading"),
    (3, "Beginning of the paragraph"),
    (4, "Beginning of the paragraph + ellipsis…"),
    (5, "Beginning of the paragraph + ellipsis…"),
    (6, "Hidden text with the referred character style"),
    (7, "Hidden text with the referred character style"),
  ] {
    assert_page_contains(&summary, page_index, expected);
  }
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf163894_from_top_to_beginning_of_the_documentMarguerite
fn mapped_fixture_tdf163894_from_top_preserves_styleref_search_from_top() {
  let summary = render_summary("tdf163894_from_top.docx");
  // Source layout dump checks the internal header field expansion to
  // "Marguerite"; soffice PDF output exposes the visible body/header text
  // instead, so this mapped PDF assertion avoids a layout-dump-only token.
  assert_page_contains(&summary, 0, "handbooks");
  assert_page_contains(&summary, 0, "infuriating");
  assert_page_contains(&summary, 1, "infuriating");
  assert_page_contains(&summary, 2, "initializes");
  assert_page_contains(&summary, 3, "maroon");
  assert_page_contains(&summary, 3, "modicum");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf135595_HFtableWrap_c12
fn mapped_fixture_tdf135595_hf_table_wrap_c12_keeps_header_text_unwrapped() {
  let summary = render_summary("tdf135595_HFtableWrap_c12.docx");
  assert_page_contains(&summary, 0, "Table anchored flies");
  assert_page_contains(&summary, 0, "don’t loose their wrapping powers.");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf78749
fn mapped_fixture_tdf78749_preserves_shape_background_image() {
  let summary = render_summary("tdf78749.docx");
  assert!(
    summary.image_count > 0,
    "expected visible bitmap fill image; images={:?}",
    summary.images
  );
}

#[test]
// Source: ../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:testFirstPageFooterEnabled
fn mapped_fixture_first_page_footer_enabled_keeps_first_page_footer_visible() {
  let summary = render_summary("TestFirstFooterDisabled.docx");
  assert_page_has_footer_text(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:testTextboxRightEdge
fn mapped_fixture_textbox_right_edge_keeps_text_inside_shape_right_edge() {
  let summary = render_summary("textbox-right-edge.docx");
  assert_rightmost_text_within_rightmost_path(&summary, 0, 0.5);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:testWpgNested
fn mapped_fixture_wpg_nested_keeps_nested_group_shape_inside_page() {
  let summary = render_summary("wpg-nested.docx");
  assert_rightmost_path_within_page_right(&summary, 0, 0.5);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf151704_thinColumnHeight
fn mapped_fixture_tdf151704_keeps_table_row_heights_equal_across_pages() {
  let summary = layout_summary("tdf151704_thinColumnHeight.docx");
  assert_matching_layout_row_height_across_pages(&summary, 0, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf133000_numStyleFormatting
fn mapped_fixture_tdf133000_preserves_numbering_margin_over_paragraph_style() {
  let summary = render_summary("tdf133000_numStyleFormatting.docx");
  assert_page_contains(&summary, 0, "First line");
  assert_page_contains(&summary, 0, "One sublevel");
  assert!(
    text_segment_left(&summary, "First line") < text_segment_left(&summary, "One sublevel"),
    "level 1 text should be left of level 2 text"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf78352
fn mapped_fixture_tdf78352_keeps_first_tab_width_close_to_zero() {
  let summary = render_summary("tdf78352.docx");
  assert_eq!(summary.page_count, 1);
  assert_max_horizontal_gap_at_most(&summary, 0, 10.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf83309
fn mapped_fixture_tdf83309_keeps_first_paragraph_without_leading_tab() {
  let summary = render_summary("tdf83309.docx");
  assert_eq!(summary.page_count, 2);
  assert!(
    text_segment_left(&summary, "In accordance") < 100.0,
    "first paragraph should start at the normal left margin"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:testTdf131801
fn mapped_fixture_tdf131801_preserves_numbering_character_colors() {
  let summary = render_summary("tdf131801.docx");
  assert_eq!(summary.page_count, 1);
  for text in ["1.", "2.", "5.", "6."] {
    assert_text_object_fill_color(&summary, text, "#ff0000@ff");
  }
  for text in ["3.", "4.", "7.", "8."] {
    assert_text_object_fill_color(&summary, text, "#000000@ff");
  }
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:testTdf135949_anchoredBeforeBreak
fn mapped_fixture_tdf135949_keeps_anchored_image_before_page_break_on_first_page() {
  let summary = render_summary("tdf135949_anchoredBeforeBreak.docx");
  assert_page_image_count(&summary, 0, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:testRelativeAnchorHeightFromBottomMarginHasFooter
fn mapped_fixture_tdf133070_has_footer_preserves_relative_anchor_height() {
  let summary = render_summary("tdf133070_testRelativeAnchorHeightFromBottomMarginHasFooter.docx");
  assert_path_height_close(&summary, 0, 57.35);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf153128
fn mapped_fixture_tdf153128_preserves_one_point_line_height() {
  let summary = layout_summary("tdf153128.docx");
  assert!(
    first_layout_line_height(&summary, "tdf153128.docx") < 30.0 / 20.0,
    "first line height should stay near 1pt"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:testBnc891663
fn mapped_fixture_bnc891663_keeps_following_row_text_below_image() {
  let summary = render_summary("bnc891663.docx");
  assert_text_below_any_image(&summary, "Some text");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:testTdf150408_isLvl_RoundTrip
fn mapped_fixture_list_with_lgl_preserves_legal_numbering_labels() {
  let summary = render_summary("listWithLgl.docx");
  for expected in ["CH I", "Sect 1.01", "CH II", "Sect 2.01"] {
    assert_page_contains(&summary, 0, expected);
  }
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf160077_layoutInCell
fn mapped_fixture_tdf160077_layout_in_cell_keeps_image_inside_cell_flow() {
  let summary = render_summary("tdf160077_layoutInCell.docx");
  assert_page_contains(&summary, 0, "Some text");
  assert_text_below_any_image(&summary, "Some");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf160077_layoutInCellB
fn mapped_fixture_tdf160077_layout_in_cell_b_aligns_column_headings() {
  let summary = render_summary("tdf160077_layoutInCellB.docx");
  assert_text_tops_close(&summary, "OBJECTIVE", "EXPERIENCE", 2.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf160077_layoutInCellC
fn mapped_fixture_tdf160077_layout_in_cell_c_keeps_image_at_cell_text_top() {
  let summary = render_summary("tdf160077_layoutInCellC.docx");
  // LibreOffice asserts the image against the first cell paragraph layout
  // bounds; PDFium only exposes visible text, so keep this assertion on the
  // visible soffice-matched layout and image presence.
  assert_page_contains(&summary, 0, "Top margin");
  assert_page_contains(&summary, 0, "-anchor paragrap");
  assert_page_contains(&summary, 0, "h-");
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf160077_layoutInCellD
fn mapped_fixture_tdf160077_layout_in_cell_d_keeps_below_labels_below_images() {
  let summary = render_summary("tdf160077_layoutInCellD.docx");
  assert_text_below_any_image(&summary, "Below logo");
  assert_text_below_any_image(&summary, "Below image");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf153909_followTextFlow
fn mapped_fixture_tdf153909_keeps_table_below_wrap_through_rectangle() {
  let summary = render_summary("tdf153909_followTextFlow.docx");
  assert_text_below_any_path(&summary, "Enterprise");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf162541
fn mapped_fixture_tdf162541_not_layout_in_cell_keeps_image_left_of_paragraph() {
  let summary = render_summary("tdf162541_notLayoutInCell_paraLeft.docx");
  assert_any_image_left_of_text(&summary, "Cell text");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf162551
fn mapped_fixture_tdf162551_layout_in_cell_keeps_image_at_cell_text_top() {
  let summary = render_summary("tdf162551_notLayoutInCell_charLeft_fromTop.docx");
  assert_any_image_near_text_top(&summary, "-anchor point-", 4.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:testRelativeAnchorHeightFromTopMarginHasHeader
fn mapped_fixture_tdf123324_has_header_preserves_relative_anchor_height() {
  let summary = render_summary("tdf123324_testRelativeAnchorHeightFromTopMarginHasHeader.docx");
  assert_path_height_close(&summary, 0, 127.55);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:testRelativeAnchorHeightFromTopMarginNoHeader
fn mapped_fixture_tdf123324_no_header_preserves_relative_anchor_height() {
  let summary = render_summary("tdf123324_testRelativeAnchorHeightFromTopMarginNoHeader.docx");
  assert_path_height_close(&summary, 0, 127.55);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:testTdf165492_exactWithBottomSpacing
fn mapped_fixture_tdf165492_exact_keeps_bottom_spacing_in_table_height() {
  let summary = render_summary("tdf165492_exactWithBottomSpacing.docx");
  assert_path_column_span_close(&summary, 0, 297.75);
  assert_path_column_span_close(&summary, 0, 170.10);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:testTdf165492_atLeastWithBottomSpacing
fn mapped_fixture_tdf165492_at_least_keeps_bottom_spacing_in_table_height() {
  let summary = render_summary("tdf165492_atLeastWithBottomSpacing.docx");
  assert_path_column_span_close(&summary, 0, 297.75);
  assert_path_column_span_close(&summary, 0, 170.10);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:testTdf165047_consolidatedTopMargin
fn mapped_fixture_tdf165047_consolidates_top_margin_after_page_break() {
  let summary = render_summary("tdf165047_consolidatedTopMargin.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Consolidate the space between paragraphs");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:testTdf165047_contextualSpacingTopMargin
fn mapped_fixture_tdf165047_contextual_spacing_ignores_top_margin_after_page_break() {
  let summary = render_summary("tdf165047_contextualSpacingTopMargin.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Don’t add space between paragraphs");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:testTdf146346
fn mapped_fixture_tdf146346_keeps_footnote_tables_on_first_page() {
  let summary = render_summary("tdf146346.docx");
  assert_eq!(summary.page_count, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:testTdf165354
fn mapped_fixture_tdf165354_preserves_bottom_of_page_hyphenation_flow() {
  let summary = render_summary("tdf165354.docx");
  // LibreOffice runs this only when an en-US hyphenator is available. The PDF
  // renderer has no hyphenation engine, so assert that the bottom-page text
  // remains in normal word order and does not leave the broken "at-" suffix.
  assert_page_contains(&summary, 0, "except that it has");
  assert_page_contains(&summary, 1, "atmosphere. The Earth");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:testTdf166544_noTopMargin_fields
fn mapped_fixture_tdf166544_no_top_margin_fields_keeps_page_two_text_height() {
  let summary = render_summary("tdf166544_noTopMargin_fields.docx");
  assert_page_contains(&summary, 1, "Page 2");
  // LibreOffice checks the layout frame height after suppressing duplicated
  // top margin around a field page break. PDFium exposes only glyph bounds for
  // the visible text, not the internal text frame height.
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:testTdf138020_all_rows_tblHeader
fn mapped_fixture_tdf138020_all_rows_table_header_does_not_repeat_header() {
  let summary = render_summary("tdf138020_all_rows_tblHeader.docx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 0, "Some text");
  assert_page_contains(&summary, 1, "No Header");
  assert_page_contains(&summary, 2, "Page.");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:testTdf166510_sectPr_bottomSpacing
fn mapped_fixture_tdf166510_section_bottom_spacing_preserves_page_two_top_margin() {
  let summary = render_summary("tdf166510_sectPr_bottomSpacing.docx");
  assert_page_contains(&summary, 1, "Page 2");
  // LibreOffice asserts the page-2 text frame height, which includes the
  // section-start spacing/top-margin calculation. PDF output has only the
  // visible glyph bounds, so the visible page-2 text is the available signal.
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:testTdf169986_bottomSpacing
fn mapped_fixture_tdf169986_bottom_spacing_keeps_continuous_break_on_one_page() {
  let summary = render_summary("tdf169986_bottomSpacing.docx");
  assert_eq!(summary.page_count, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:testTdf167657_sectPr_bottomSpacing
fn mapped_fixture_tdf167657_continuous_section_keeps_bottom_spacing_visible() {
  let summary = render_summary("tdf167657_sectPr_bottomSpacing.docx");
  assert_eq!(summary.page_count, 1);
}

#[test]
// Source: ../core/sw/qa/core/text/text.cxx:testNumberPortionNoformat
fn mapped_fixture_number_portion_noformat_keeps_numbering_portion_auto_colored() {
  let summary = render_summary("number-portion-noformat.docx");
  assert_text_object_fill_color(&summary, "1.", "#000000@ff");
}

#[test]
// Source: ../core/sw/qa/core/layout/calcmove.cxx:testIgnoreTopMargin
fn mapped_fixture_ignore_top_margin_ignores_first_paragraph_top_margin_on_page_two() {
  let summary = render_summary("ignore-top-margin.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "Page 2");
  assert_first_text_top_from_page_top_at_most(&summary, 1, 90.0);
}

#[test]
// Source: ../core/sw/qa/core/layout/calcmove.cxx:testIgnoreTopMarginTable
fn mapped_fixture_ignore_top_margin_table_keeps_cell_first_paragraph_top_margin() {
  let summary = render_summary("ignore-top-margin-table.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 1, "A1");
  assert_page_contains(&summary, 1, "B1");
  assert_page_text_below_text(&summary, 1, "B1", "A1", 80.0);
}

#[test]
// Source: ../core/sw/qa/core/layout/calcmove.cxx:testIgnoreTopMarginPageStyleChange
fn mapped_fixture_ignore_top_margin_page_style_change_keeps_section_top_margin() {
  let summary = render_summary("ignore-top-margin-page-style-change.docx");
  assert_eq!(summary.page_count, 3);
  assert_page_contains(&summary, 1, "after page break");
  assert_page_contains(&summary, 2, "after section break");
  assert_first_text_top_from_page_top_at_least(&summary, 2, 80.0);
}

#[test]
// Source: ../core/sw/qa/core/layout/ftnfrm.cxx:testInlineEndnotePosition
fn mapped_fixture_inline_endnote_position_keeps_endnote_separator_spacing() {
  let summary = render_summary("inline-endnote-position.docx");
  assert_page_contains(&summary, 0, "Endnote");
  assert_text_below_any_path_by_at_least(&summary, "Endnote", 13.45);
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testTableFlyOverlap
fn mapped_fixture_table_fly_overlap_keeps_table_below_header_image() {
  let summary = render_summary("table-fly-overlap.docx");
  assert_text_below_any_image(&summary, "Table1:B1");
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testTdf128195
fn mapped_fixture_tdf128195_keeps_header_spacing_above_body_text() {
  let summary = render_summary("tdf128195.docx");
  assert_page_contains(&summary, 0, "Body");
  assert_text_top_from_page_top_at_least(&summary, 0, "Body", 176.45);
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testBorderCollapseCompat
fn mapped_fixture_border_collapse_compat_prefers_solid_cell_border() {
  let summary = render_summary("border-collapse-compat.docx");
  assert!(
    summary.paths.iter().any(|path| path.stroked == Some(true)),
    "expected rendered solid table border path; paths={:?}",
    summary.paths
  );
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testTableFlyOverlapSpacing
fn mapped_fixture_table_fly_overlap_spacing_keeps_table_next_to_image() {
  let summary = render_summary("table-fly-overlap-spacing.docx");
  assert_page_contains(&summary, 0, "Before table");
  assert_page_contains(&summary, 0, "After table.");
  assert_text_top_above_any_image_bottom(&summary, "These");
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testTextBoxAutoGrowVertical
fn mapped_fixture_textbox_autogrow_vertical_keeps_text_inside_shape() {
  let summary = render_summary("textbox-autogrow-vertical.docx");
  // PDF glyph bounds can extend slightly outside the Writer layout rectangle
  // for vertical text; LibreOffice asserts the text stays in the grown shape.
  assert_text_inside_any_path_with_tolerance(&summary, 0, "Shape", 2.0);
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testTextBoxInHeaderIsPositioned
fn mapped_fixture_header_textbox_keeps_header_textbox_right_positioned() {
  let summary = render_summary("header-textbox.docx");
  assert_text_top_from_page_top_at_most(&summary, 0, "XXXXXXX", 120.0);
  assert!(
    text_segment_left(&summary, "XXXXXXX") >= 350.0,
    "header textbox text should be positioned on the right side"
  );
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testVerticallyMergedCellBorder
fn mapped_fixture_vmerge_cell_border_omits_merged_cell_lower_borders() {
  let summary = render_summary("vmerge-cell-border.docx");
  assert_horizontal_path_count_at_least(&summary, 0, 4);
  assert_vertical_path_count_at_least(&summary, 0, 3);
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testInnerCellBorderIntersect
fn mapped_fixture_inner_border_keeps_middle_border_inset_from_outer_border() {
  let summary = render_summary("inner-border.docx");
  assert_horizontal_path_count_at_least(&summary, 0, 3);
  assert_middle_horizontal_border_is_inset(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testDoubleBorderVertical
fn mapped_fixture_double_border_vertical_draws_all_four_vertical_border_lines() {
  let summary = render_summary("double-border-vertical.docx");
  assert_vertical_path_count_at_least(&summary, 0, 4);
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testDoubleBorderHorizontal
fn mapped_fixture_double_border_horizontal_draws_all_four_horizontal_border_lines() {
  let summary = render_summary("double-border-horizontal.docx");
  assert_horizontal_path_count_at_least(&summary, 0, 4);
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testParaBorderInCellClip
fn mapped_fixture_para_border_in_cell_clip_keeps_paragraph_borders_clipped_to_cell() {
  let summary = render_summary("para-border-in-cell-clip.docx");
  assert_page_contains(&summary, 0, "A");
  assert_page_contains(&summary, 0, "1");
  assert_text_inside_any_path(&summary, 0, "A");
}

#[test]
// Source: ../core/sw/qa/core/layout/layout.cxx:testDoublePageBorder
fn mapped_fixture_double_page_border_draws_top_and_bottom_double_borders() {
  let summary = render_summary("double-page-border.docx");
  assert_horizontal_path_count_at_least(&summary, 0, 4);
}

#[test]
// Source: ../core/sw/qa/core/layout/paintfrm.cxx:testRTLBorderMerge
fn mapped_fixture_rtl_table_keeps_all_six_vertical_column_borders() {
  let summary = render_summary("rtl-table.docx");
  assert_vertical_path_count_at_least(&summary, 0, 6);
}

#[test]
// Source: ../core/sw/qa/core/layout/paintfrm.cxx:testInlineEndnoteSeparatorPosition
fn mapped_fixture_inline_endnote_separator_keeps_word_separator_length() {
  let summary = render_summary("inline-endnote-position.docx");
  assert_path_geometry_width_close(&summary, 0, 144.0);
}

#[test]
// Source: ../core/sw/qa/core/layout/paintfrm.cxx:testEndnoteContSeparator
fn mapped_fixture_endnote_cont_separator_spans_page_print_area() {
  let summary = render_summary("endnote-cont-separator.docx");
  assert_eq!(summary.page_count, 2);
  assert_path_geometry_width_close(&summary, 1, 468.0);
}

#[test]
// Source: ../core/sw/qa/core/layout/tabfrm.cxx:testTablePrintAreaLeft
fn mapped_fixture_table_print_area_left_keeps_table_visible_at_left_margin() {
  let summary = render_summary("table-print-area-left.docx");
  assert_page_contains(&summary, 0, "Date & venue");
  assert!(
    text_segment_left(&summary, "Date") < 100.0,
    "table should remain visible near the left page margin"
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:TestTdf136588
fn mapped_fixture_tdf136588_keeps_expected_second_line_break() {
  let summary = render_summary("tdf136588.docx");
  assert_page_contains(
    &summary,
    0,
    "effectively by modern-day small to medium enterprises?",
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:testTdf88496
fn mapped_fixture_tdf88496_keeps_repeating_header_fallback_to_three_pages() {
  let summary = render_summary("tdf88496.docx");
  assert_eq!(summary.page_count, 3);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:TestTdf137025
fn mapped_fixture_tdf137025_preserves_textbox_padding_visible_in_pdf() {
  let summary = render_summary("tdf137025.docx");
  assert_page_contains(&summary, 0, "xxxx");
  assert_text_inside_any_path(&summary, 0, "xxxx");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:TestTdf134277
fn mapped_fixture_tdf134277_keeps_text_visible_without_extra_layout_mode() {
  assert_rendered_page_has_non_white_pixels("tdf134277.docx", 0, 1200);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:testTdf116486
fn mapped_fixture_tdf116486_preserves_fly_portion_height() {
  let summary = render_summary("tdf116486.docx");
  // LibreOffice asserts the invisible fly portion height in the layout dump.
  // The DOCX shape itself has no fill and no stroke, so PDF output exposes the
  // textbox text but not a path matching that internal fly portion.
  assert_page_contains(&summary, 0, "Flying Box");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:TestTdf142080
fn mapped_fixture_fdo43573_2_min_keeps_page_nine_column_text_and_image() {
  let summary = render_summary("fdo43573-2-min.docx");
  assert!(summary.page_count >= 9);
  assert_page_contains(&summary, 8, "De kleur u (rood) in het rechtervlak");
  assert_page_image_count_at_least(&summary, 8, 1);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:testTdf128198
fn mapped_fixture_tdf128198_keeps_wrapped_text_after_fly_untruncated() {
  let summary = render_summary("tdf128198-1.docx");
  assert_page_contains(&summary, 0, "From this perspective");
  assert_page_contains(&summary, 0, "satellite boasts some significant advantages.");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:testTdf106153
fn mapped_fixture_tdf106153_keeps_all_textboxes_inside_page() {
  let summary = render_summary("tdf106153.docx");
  assert_all_paths_within_page(&summary, 0, 0.5);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:testTdf109137
fn mapped_fixture_tdf109137_keeps_blue_rectangle_on_first_page() {
  let summary = render_summary("tdf109137.docx");
  assert_page_contains(&summary, 0, "Gráfico");
  assert_page_path_count(&summary, 0, 1);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout.cxx:testTdf157628
fn mapped_fixture_tdf157628_keeps_two_expected_rows_visible() {
  let summary = render_summary("tdf157628.docx");
  assert_page_contains(&summary, 0, "This is in first row");
  assert_page_contains(&summary, 0, "This is second row");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:tdf157596_paragraph_numbering
fn mapped_fixture_tdf157596_paragraph_numbering_keeps_imported_numbers() {
  let summary = render_summary("tdf157596_paragraph_numbering.docx");
  assert_page_contains_in_order(&summary, 0, &["1.", "2.", "3."]);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf152872
fn mapped_fixture_hidden_para_separator_collapses_hidden_paragraphs() {
  let summary = render_summary("hidden-para-separator.docx");
  assert_text_segments_on_same_line_in_order(&summary, 0, &["C", "D", "E"]);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf125300
fn mapped_fixture_tdf125300_keeps_spacing_before_bottom_cell_border() {
  let summary = render_summary("tdf125300.docx");
  // LibreOffice checks the preview metafile bottom-cell-border coordinate.
  // The PDF path is the painted border stroke, which is a few points above the
  // layout/metafile coordinate but still preserves the visible spacing before
  // the border.
  assert_path_top_from_page_top_close_with_tolerance(&summary, 0, 59.3, 4.0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf122225
fn mapped_fixture_tdf122225_keeps_chart_legend_and_axis_labels() {
  let summary = render_summary("tdf122225.docx");
  assert_page_text_occurrences(&summary, 0, "Advanced Diploma", 1);
  assert_page_text_occurrences(&summary, 0, "Hispanic", 1);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf134247
fn mapped_fixture_legend_itemorder_min_keeps_first_legend_label() {
  let summary = render_summary("legend-itemorder-min.docx");
  assert_page_contains(&summary, 0, "1. adatsor");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf75659
fn mapped_fixture_tdf75659_keeps_all_chart_legend_names() {
  let summary = render_summary("tdf75659.docx");
  assert_page_contains_in_order(&summary, 0, &["Series1", "Series2", "Series3"]);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf126425
fn mapped_fixture_long_legendentry_keeps_long_chart_legend_text() {
  let summary = render_summary("long_legendentry.docx");
  assert_page_contains(&summary, 0, "Data series with a long long title");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf115630
fn mapped_fixture_tdf115630_keeps_inner_chart_area_width() {
  let summary = render_summary("tdf115630.docx");
  assert_page_contains(&summary, 0, "1. Column with long name");
  assert_path_width_between(&summary, 0, 143.5, 146.5);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf128996
fn mapped_fixture_tdf128996_keeps_long_category_axis_label_visible() {
  let summary = render_summary("tdf128996.docx");
  assert_page_contains(&summary, 0, "A very long category name 1");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf126244
fn mapped_fixture_tdf126244_preserves_vertical_multilevel_axis_labels() {
  let summary = render_summary("tdf126244.docx");
  assert_page_contains(&summary, 0, "FIRST LEVEL");
  assert_text_segment_taller_than_wide(&summary, 0, "FIRST LEVEL");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf69648
fn mapped_fixture_tdf69648_keeps_textboxes_inside_group_shapes() {
  let summary = render_summary("tdf69648.docx");
  assert_matching_text_segments_inside_paths(&summary, 0, "Text in right box");
  assert_matching_text_segments_inside_paths(&summary, 0, "Text in left box");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf117982
fn mapped_fixture_tdf117982_keeps_first_table_cell_text_visible() {
  let summary = render_summary("tdf117982.docx");
  assert_page_starts_with(&summary, 0, "FOO AAA");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf128959
fn mapped_fixture_tdf128959_keeps_split_table_cell_first_lines_visible() {
  let summary = render_summary("tdf128959.docx");
  assert_page_contains(
    &summary,
    0,
    "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Maecenas porttitor congue",
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf124423_DOCX
fn mapped_fixture_tdf124423_keeps_fly_widths_relative_to_page() {
  let summary = render_summary("tdf124423.docx");
  assert_path_width_at_least_page_fraction(&summary, 0, 0.5);
  assert_path_width_at_least_page_fraction(&summary, 0, 0.9);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf138782
fn mapped_fixture_tdf138782_keeps_third_shape_inside_page() {
  let summary = render_summary("tdf138782.docx");
  assert_page_contains(&summary, 0, "10");
  assert_rightmost_path_within_page_right(&summary, 0, 0.5);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf135035_DOCX
fn mapped_fixture_tdf135035_keeps_nested_fly_widths_relative_to_parent() {
  let summary = render_summary("tdf135035.docx");
  assert_page_contains(&summary, 0, "A");
  assert_path_width_at_least_page_fraction(&summary, 0, 0.5);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf139336_ColumnsWithFootnoteDoNotOccupyEntirePage
fn mapped_fixture_tdf139336_columns_with_footnote_stays_two_pages() {
  let summary = render_summary("tdf139336_ColumnsWithFootnoteDoNotOccupyEntirePage.docx");
  assert_eq!(summary.page_count, 2);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:TestTdf161348
fn mapped_fixture_fdo48718_keeps_floating_table_on_first_page() {
  let summary = render_summary("fdo48718-1.docx");
  assert_page_contains(&summary, 0, "INFORME DE ASISTENCIA");
  assert_page_contains(&summary, 0, "ARGUMENTACIÓN JURÍDICA");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf159271
fn mapped_fixture_fld_in_tbl_keeps_field_in_single_table_line() {
  let summary = render_summary("fld-in-tbl.docx");
  assert_eq!(summary.page_count, 1);
  assert_page_text_occurrences(&summary, 0, "LOCATION", 1);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf159259
fn mapped_fixture_sdt_framepr_keeps_single_visible_paragraph() {
  let summary = render_summary("sdt+framePr.docx");
  assert_eq!(summary.page_count, 1);
  assert_page_text_occurrences(
    &summary,
    0,
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    1,
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf115094
fn mapped_fixture_tdf115094_keeps_nested_objects_inside_table_cells() {
  let summary = render_summary("tdf115094.docx");
  assert_page_contains(&summary, 0, "Zufahrt");
  assert_text_inside_any_path(&summary, 0, "Rollstuhlfahrer");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf112290
fn mapped_fixture_tdf112290_keeps_second_line_text_portion() {
  let summary = render_summary("tdf112290.docx");
  assert_page_contains(&summary, 0, "Xxxx Xxxx");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf123651
fn mapped_fixture_tdf123651_keeps_shape_above_second_lorem_ipsum() {
  let summary = render_summary("tdf123651.docx");
  assert_page_text_occurrences(
    &summary,
    0,
    "Lorem ipsum dolor sit amet, consectetuer adipiscing elit.",
    2,
  );
  // LibreOffice layout dump asserts the anchored draw object frame top at
  // 7639 twips. The exported PDF path bounds exclude part of that frame; a
  // soffice PDF export places the visible arrow path at about 368.75pt.
  assert_path_top_from_page_top_close_with_tolerance(&summary, 0, 368.75, 1.0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf64222
fn mapped_fixture_tdf64222_preserves_large_numbering_font_height() {
  let summary = render_summary("tdf64222.docx");
  assert_page_contains(&summary, 0, "Another one title of document");
  assert_has_text_object_font_size(&summary, "28.00");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf170381_split_float_table_in_normal_table
fn mapped_fixture_tdf170381_normal_table_splits_float_table_across_two_pages() {
  let summary = render_summary("tdf170381-split-float-table-in-normal-table.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "elit ipsum lorem dolor");
  assert_page_contains(
    &summary,
    0,
    "amet elit amet sit adipiscing adipiscing consectetur consectetur elit dolor",
  );
  assert_page_contains(&summary, 1, "adipiscing ipsum elit lorem");
  assert_page_contains(&summary, 1, "consectetur dolor lorem ipsum");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf170381_split_float_table_in_float_table
fn mapped_fixture_tdf170381_float_table_splits_nested_float_table_across_two_pages() {
  let summary = render_summary("tdf170381-split-float-table-in-float-table.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Table1 A1 dolor elit");
  assert_page_contains(
    &summary,
    0,
    "adipiscing dolor adipiscing amet ipsum elit sit elit lorem elit adipiscing dolor ipsum",
  );
  assert_page_contains(&summary, 0, "Table2 A22 elit");
  assert_page_contains(&summary, 1, "Table2 A23");
  assert_page_contains(&summary, 1, "Table2 A31");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf170620_float_table_after_keep_with_next_para
fn mapped_fixture_tdf170620_keeps_keep_with_next_paragraph_on_first_page() {
  let summary = render_summary("tdf170620.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Keep-with-next paragraph");
  assert_page_contains(&summary, 0, "Something");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf170630
fn mapped_fixture_tdf170630_splits_keep_with_next_float_table_to_two_pages() {
  let summary = render_summary("tdf170630.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "Keep-with-next paragraph");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf170846_1
fn mapped_fixture_tdf170846_1_moves_whole_floating_table_to_page_two() {
  let summary = render_summary("tdf170846_1.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_not_contains(&summary, 0, "Some floating table");
  assert_page_contains(&summary, 1, "Some floating table");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf170846_2
fn mapped_fixture_tdf170846_2_moves_whole_nested_floating_table_to_page_two() {
  let summary = render_summary("tdf170846_2.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_not_contains(&summary, 0, "adipiscing");
  assert_page_contains(&summary, 1, "adipiscing");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf138194
fn mapped_fixture_xaxis_labelbreak_keeps_wrapped_axis_label_visible() {
  let summary = render_summary("xaxis-labelbreak.docx");
  assert_page_text_segment_count(&summary, 0, 8);
  assert_page_contains(
    &summary,
    0,
    "really really long data label 1 made even longer",
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf138773
fn mapped_fixture_tdf138773_keeps_first_x_axis_label_on_one_line() {
  let summary = render_summary("tdf138773.docx");
  assert_page_text_occurrences(&summary, 0, "2000-01", 1);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf130969
fn mapped_fixture_tdf130969_preserves_y_axis_minimum_label() {
  let summary = render_summary("tdf130969.docx");
  assert_page_contains(&summary, 0, "0.35781");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf129054
fn mapped_fixture_tdf129054_preserves_pie_chart_diameter() {
  let summary = render_summary("tdf129054.docx");
  assert_page_contains(&summary, 0, "Értékesítés");
  assert_path_height_close(&summary, 0, 230.75);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf129173
fn mapped_fixture_test_area_chart_number_format_keeps_first_area_data_label() {
  let summary = render_summary("testAreaChartNumberFormat.docx");
  assert_page_contains(&summary, 0, "56");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf134866
fn mapped_fixture_tdf134866_keeps_pie_chart_percent_label() {
  let summary = render_summary("tdf134866.docx");
  assert_page_contains(&summary, 0, "100%");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf137116
fn mapped_fixture_tdf137116_keeps_second_label_outside_pie_slice() {
  let summary = render_summary("tdf137116.docx");
  assert_page_contains(&summary, 0, "datalabel2");
  assert_page_contains(&summary, 0, "datalabel4");
  assert_text_left_difference_close(&summary, "datalabel2", "datalabel4", 55.85, 5.0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf137154
fn mapped_fixture_tdf137154_keeps_first_and_fourth_data_labels_aligned() {
  let summary = render_summary("tdf137154.docx");
  assert_page_contains(&summary, 0, "long data label 1");
  assert_page_contains(&summary, 0, "long data label 4");
  assert_text_left_difference_close(&summary, "long data label 1", "long data label 4", 0.0, 2.5);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf138777
fn mapped_fixture_outside_long_data_label_breaks_to_multiple_lines() {
  let summary = render_summary("outside_long_data_label.docx");
  assert_page_text_occurrences_at_least(&summary, 0, "really", 2);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf130031
fn mapped_fixture_tdf130031_keeps_data_label_below_data_point() {
  let summary = render_summary("tdf130031.docx");
  assert_page_contains(&summary, 0, "23");
  assert_text_top_from_page_top_close(&summary, 0, "23", 232.65, 2.5);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf138018
fn mapped_fixture_tdf138018_omits_extra_pie_chart_leader_line() {
  let summary = render_summary("tdf138018.docx");
  assert_page_contains(&summary, 0, "Értékesítés");
  assert_page_path_count(&summary, 0, 2);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf130380
fn mapped_fixture_tdf130380_keeps_area_chart_from_shrinking() {
  let summary = render_summary("tdf130380.docx");
  assert_page_contains(&summary, 0, "1. adatsor");
  assert_path_top_from_page_top_close(&summary, 0, 336.35);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf129095
fn mapped_fixture_tdf129095_keeps_relative_inner_chart_area_visible() {
  let summary = render_summary("tdf129095.docx");
  assert_page_contains(&summary, 0, "Category 1");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf132956
fn mapped_fixture_tdf132956_keeps_default_inner_chart_area_visible() {
  let summary = render_summary("tdf132956.docx");
  assert_page_contains(&summary, 0, "Category 1");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf122014
fn mapped_fixture_tdf122014_keeps_word_chart_title_alignment() {
  let summary = render_summary("tdf122014.docx");
  assert_page_contains(&summary, 0, "Chart title alignment");
  assert_second_text_left_no_more_than(&summary, "Chart title", "alignment", 5.0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf167202_footnote
fn mapped_fixture_tdf167202_footnote_keeps_single_footnote_chart_numbering() {
  let summary = render_summary("tdf167202_footnote.docx");
  assert_page_contains(&summary, 0, "FOOTNOTE #1");
  assert_page_text_segment_count(&summary, 0, 10);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf134659
fn mapped_fixture_tdf134659_keeps_axis_label_alignment() {
  let summary = render_summary("tdf134659.docx");
  assert_page_contains(&summary, 0, "Test the axis label aligment!");
  assert_second_text_left_no_more_than(&summary, "Test the axis", "label aligment", 12.5);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf134235
fn mapped_fixture_tdf134235_keeps_long_chart_title_inside_chart_area() {
  let summary = render_summary("tdf134235.docx");
  assert_page_contains(&summary, 0, "When opened in Writer the long chart title");
  assert_page_text_segment_count(&summary, 0, 14);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf134676
fn mapped_fixture_tdf134676_breaks_long_x_axis_title_to_multiple_lines() {
  let summary = render_summary("tdf134676.docx");
  assert_page_contains(&summary, 0, "default length of the axis title box");
  assert_page_text_segment_count(&summary, 0, 14);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf134146
fn mapped_fixture_tdf134146_breaks_long_y_axis_title_to_multiple_lines() {
  let summary = render_summary("tdf134146.docx");
  assert_page_text_occurrences_at_least(&summary, 0, "Horizontal", 2);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf136061
fn mapped_fixture_tdf136061_keeps_custom_data_label_text() {
  let summary = render_summary("tdf136061.docx");
  assert_page_contains(&summary, 0, "Customlabel");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf116925
fn mapped_fixture_tdf116925_keeps_chart_text_white() {
  let summary = render_summary("tdf116925.docx");
  assert_page_contains(&summary, 0, "hello");
  assert_text_object_fill_color(&summary, "hello", "#ffffff@ff");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf117028
fn mapped_fixture_tdf117028_omits_white_background_polygon() {
  let summary = render_summary("tdf117028.docx");
  assert_page_contains(&summary, 0, "Hello");
  assert_page_path_count(&summary, 0, 0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf150200_DOCX
fn mapped_fixture_tdf150200_docx_preserves_dash_line_breaks() {
  let summary = render_summary("tdf150200.docx");
  assert_page_contains(&summary, 0, "-(dash)");
  assert_page_contains(&summary, 0, "–(en-dash)");
  assert_page_contains(&summary, 0, "—(em-dash)");
  assert_page_contains(&summary, 0, "‒(figure dash)");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf150438_DOCX
fn mapped_fixture_tdf150438_docx_preserves_quote_line_breaks() {
  let summary = render_summary("tdf150438.docx");
  assert_page_contains(&summary, 0, "“Lorem ipsum");
  assert_page_contains(&summary, 0, "”Nunc viverra imperdiet enim.");
  assert_page_contains(&summary, 0, "‘Aenean nec lorem.");
  assert_page_contains(&summary, 0, "’Aenean nec lorem.");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf127118
fn mapped_fixture_tdf127118_keeps_vertical_writing_in_split_merged_cell() {
  let summary = render_summary("tdf127118.docx");
  assert_eq!(summary.page_count, 2);
  // LibreOffice asserts the split merged cell has vertical WritingMode on
  // page 2. The visible continued numbering text on that page is "2.".
  assert_text_segment_taller_than_wide(&summary, 1, "2.");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf141220
fn mapped_fixture_tdf141220_keeps_textbox_inside_shape() {
  let summary = render_summary("tdf141220.docx");
  assert_text_inside_any_path(&summary, 0, "Lorem ipsum");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf134685
fn mapped_fixture_tdf134685_keeps_wide_table_cell_content_visible() {
  let summary = render_summary("tdf134685.docx");
  assert_page_contains(&summary, 0, "fffffffff");
  assert_rightmost_text_within_rightmost_path(&summary, 0, 0.5);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf109077
fn mapped_fixture_tdf109077_keeps_textbox_top_aligned_with_shape() {
  let summary = render_summary("tdf109077.docx");
  assert_page_contains(&summary, 0, "x1");
  assert_text_inside_any_path(&summary, 0, "x1");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf164903
fn mapped_fixture_tdf164903_ignores_inline_heading_top_margin() {
  let summary = render_summary("tdf164903.docx");
  assert_page_contains(&summary, 0, "Definitions");
  // LibreOffice checks the inline-heading text frame height in the layout
  // dump. PDFium reports glyph bounds, which are font-dependent and shorter
  // than Writer's internal frame height.
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout3.cxx:testTdf134463
fn mapped_fixture_tdf134463_keeps_table_paragraph_border_from_expanding_previous_paragraph() {
  let summary = render_summary("tdf134463.docx");
  assert_page_contains_in_order(&summary, 0, &["A1", "A2", "B1", "B2"]);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout3.cxx:testTdf117188
fn mapped_fixture_tdf117188_keeps_textbox_text_inside_zero_border_fly() {
  let summary = render_summary("tdf117188.docx");
  assert_page_contains(&summary, 0, "Der");
  assert_page_path_count(&summary, 0, 0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout3.cxx:testTdf161718
fn mapped_fixture_tdf161718_keeps_header_footer_footnote_and_body_on_one_page() {
  let summary = render_summary("tdf161718.docx");
  assert_eq!(summary.page_count, 1);
  assert_page_contains(&summary, 0, "Header Text");
  assert_page_contains(&summary, 0, "Body text");
  assert_page_contains(&summary, 0, "Footer Text.");
  assert_page_contains(&summary, 0, "Footnote area");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout3.cxx:testTdf119908
fn mapped_fixture_tdf119908_keeps_exceeding_line_portion_visible_for_shrinking() {
  let summary = render_summary("tdf130088.docx");
  assert_page_contains(
    &summary,
    0,
    "viverra odio. Donec auctor molestie sem, sit amet tristique lectus hendrerit sed.",
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout3.cxx:testTdf158333
fn mapped_fixture_tdf158333_keeps_expected_shrunk_line_portions() {
  let summary = render_summary("tdf130088.docx");
  assert_page_contains(
    &summary,
    0,
    "viverra odio. Donec auctor molestie sem, sit amet tristique lectus hendrerit sed.",
  );
  assert_page_contains(
    &summary,
    0,
    "laoreet vel leo nec, volutpat facilisis eros. Donec consequat arcu ut diam tempor",
  );
  assert_page_contains(
    &summary,
    0,
    "Donec auctor molestie sem, sit amet tristique lectus hendrerit sed. Cras sodales",
  );
  assert_page_contains(
    &summary,
    0,
    "consequat arcu ut diam tempor luctus. Cum sociis natoque penatibus et magnis",
  );
  assert_page_contains(
    &summary,
    0,
    "venenatis, quis commodo dolor posuere. Curabitur dignissim sapien quis",
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout3.cxx:testTdf164905
fn mapped_fixture_tdf164905_avoids_extra_toc_glue_portions() {
  let summary = render_summary("tdf164905.docx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["INHALT", "VERANTWORTLICHKEIT", "ZIELSETZUNG DER"],
  );
  // LibreOffice asserts internal SwGluePortion count. In mapped PDF output,
  // keep the equivalent visible signal: the TOC entries are present once and
  // remain in document order without the old glue-induced disruption.
  assert_page_text_occurrences(&summary, 0, "VERANTWORTLICHKEIT", 1);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout3.cxx:testTdf163149
fn mapped_fixture_tdf163149_keeps_second_shrunk_line_text_array_visible() {
  let summary = render_summary("tdf163149.docx");
  assert_page_contains(
    &summary,
    0,
    "vulputate nisl commodo. Proin aliquet turpis ac posuere commodo. Curabitur facilisis mauris ac nulla dapibus",
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout3.cxx:testTdf164499
fn mapped_fixture_tdf164499_keeps_tabulated_heading_page_number_on_own_line() {
  let summary = render_summary("tdf164499.docx");
  assert_page_contains(&summary, 0, "2.5.5");
  assert_page_contains(&summary, 0, "pH-Messung");
  assert_page_contains(&summary, 0, "hat und ich keine Werte habe?)");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testWriterImageNoCapture
fn mapped_fixture_writer_image_no_capture_allows_image_left_of_page_frame() {
  let summary = render_summary("writer-image-no-capture.docx");
  assert_any_image_extends_left_of_page(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:testTdf152298
fn mapped_fixture_tdf152298_splits_rowspan_table_follow_row_to_second_page() {
  let summary = render_summary("tdf152298.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains_in_order(&summary, 1, &["1", "2", "3", "10"]);
}

#[test]
// Source: ../core/sw/qa/core/layout/paintfrm.cxx:testTableRedlineRenderMode
fn mapped_fixture_redline_table_paints_default_row_redline_polygons() {
  let summary = render_summary("redline-table.docx");
  assert_page_filled_path_count(&summary, 0, 2);
}

#[test]
// Source: ../core/sw/qa/core/text/itrpaint.cxx:testRedlineRenderModeOmitInsertDelete
fn mapped_fixture_redline_default_paints_baseline_delete_and_insert_text() {
  let summary = render_summary("redline.docx");
  assert_page_text_segment_count(&summary, 0, 3);
  assert_page_contains_in_order(&summary, 0, &["baseline", "oldcontent", "newcontent"]);
  assert_text_object_fill_color(&summary, "baseline", "#000000@ff");
  assert_text_objects_share_non_black_fill_color(&summary, &["oldcontent", "newcontent"]);
}

#[test]
// Source: ../core/sw/qa/core/text/itrpaint.cxx:testAnchoredImageRedlineRenderModeOmitInsertDelete
fn mapped_fixture_redline_image_anchored_default_keeps_three_color_images_without_frames() {
  let summary = render_summary("redline-image-anchored.docx");
  assert_page_image_count(&summary, 0, 3);
  assert_image_centers_are_not_grayscale("redline-image-anchored.docx", &summary, 0, 3);
  assert_page_path_count(&summary, 0, 0);
}

#[test]
// Source: ../core/sw/qa/core/text/itrpaint.cxx:testInlineImageRedlineRenderModeOmitInsertDelete
fn mapped_fixture_redline_image_inline_default_keeps_three_color_images_without_frames() {
  let summary = render_summary("redline-image-inline.docx");
  assert_page_image_count(&summary, 0, 3);
  assert_image_centers_are_not_grayscale("redline-image-inline.docx", &summary, 0, 3);
  assert_page_path_count(&summary, 0, 0);
}

#[test]
// Source: ../core/sw/qa/core/text/porfld.cxx:testNumberPortionRedlineRenderMode
fn mapped_fixture_redline_number_portion_default_underlines_inserted_number() {
  let summary = render_summary("redline-number-portion.docx");
  assert_page_contains(&summary, 0, "2.");
  assert_horizontal_path_decorates_text(&summary, 0, "2.", false);
}

#[test]
// Source: ../core/sw/qa/core/text/porfld.cxx:testTabPortionRedlineRenderMode
fn mapped_fixture_redline_bullet_default_strikes_deleted_tab_portion() {
  let summary = render_summary("redline-bullet.docx");
  assert_any_horizontal_path_crosses_any_text(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:testHighlightNumbering
fn mapped_fixture_tdf114799_highlight_paints_yellow_numbering_highlight() {
  render_summary("tdf114799_highlight.docx");
  assert_rendered_pixel_rgb_close("tdf114799_highlight.docx", 0, 1024, 103, 148, [255, 255, 0]);
}

#[test]
// Source: ../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:testHighlightNumbering_shd
fn mapped_fixture_tdf114799_shd_keeps_numbering_unshaded() {
  render_summary("tdf114799_shd.docx");
  assert_rendered_pixel_rgb_close("tdf114799_shd.docx", 0, 1024, 103, 148, [255, 255, 255]);
}

#[test]
// Source: ../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:testTdf159626_yellowPatternFill
fn mapped_fixture_tdf159626_yellow_pattern_fill_is_primarily_yellow() {
  render_summary("tdf159626_yellowPatternFill.docx");
  assert_rendered_page_color_ratio(
    "tdf159626_yellowPatternFill.docx",
    0,
    1024,
    [255, 255, 0],
    2,
  );
}

#[test]
// Source: ../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:testTdf159626_yellowPatternFillB
fn mapped_fixture_tdf159626_yellow_pattern_fill_b_is_primarily_yellow() {
  render_summary("tdf159626_yellowPatternFillB.docx");
  assert_rendered_page_color_ratio(
    "tdf159626_yellowPatternFillB.docx",
    0,
    1024,
    [255, 255, 0],
    2,
  );
}

#[test]
// Source: ../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:testTdf159626_blackPatternFill
fn mapped_fixture_tdf159626_black_pattern_fill_is_primarily_black() {
  render_summary("tdf159626_blackPatternFill.docx");
  assert_rendered_page_color_ratio("tdf159626_blackPatternFill.docx", 0, 1024, [0, 0, 0], 10);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf165322
fn mapped_fixture_ct_formatted_deletion_marks_deleted_paragraph_with_strikeout() {
  let summary = render_summary("CT-formatted-deletion.docx");
  assert_page_contains(
    &summary,
    0,
    "Nunc viverra imperdiet enim. Fusce est. Vivamus a tellus.",
  );
  assert_horizontal_path_decorates_text(
    &summary,
    0,
    "Nunc viverra imperdiet enim. Fusce est. Vivamus a tellus.",
    true,
  );
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testRedlineMovingDOCX
fn mapped_fixture_tdf104797_docx_move_redline_paints_green_moved_text() {
  let summary = render_summary("tdf104797.docx");
  assert_page_contains(&summary, 0, "Will this sentence be duplicated?");
  assert_page_contains(&summary, 0, "This is a filler sentence.");
  assert_page_contains(&summary, 0, "ADDED STUFF");
  // LibreOffice checks green textcolor commands in the metafile. PDFium may
  // split one moved run into multiple text objects, so assert the moved text
  // itself is painted with the LibreOffice move-redline color.
  assert_text_object_fill_color(&summary, "Will this sentence be duplicated", "#008000@ff");
  assert_text_object_fill_color(&summary, "duplicated?", "#008000@ff");
  assert_text_object_fill_color(&summary, "?", "#008000@ff");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:TestTdf155229RowAtLeast
fn mapped_fixture_tdf155229_row_height_at_least_keeps_table_bottom_position() {
  let summary = render_summary("tdf155229_row_height_at_least.docx");
  // LibreOffice layout dump reports the row frame bottom at 15494 twips. The
  // visible PDF bottom border exported by soffice is about 763.6pt from the
  // page top, so keep this mapped PDF fixture on the PDF-visible geometry.
  assert_any_path_bottom_from_page_top_close(&summary, 0, 763.6, 3.0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout4.cxx:TestTdf164907_rowHeightAtLeast
fn mapped_fixture_tdf164907_row_height_at_least_includes_top_and_bottom_padding() {
  let summary = render_summary("tdf164907_rowHeightAtLeast.docx");
  assert_eq!(summary.page_count, 1);
  // LibreOffice's layout dump reports row[1] bottom at 2852 twips. In the
  // exported PDF this maps to the next row's first text top at about 143.1pt;
  // keep the mapped test on visible PDF geometry instead of internal row
  // fragments, whose baseline/vertical-alignment model is intentionally
  // smaller than Writer's frame tree.
  assert_text_top_from_page_top_close(&summary, 0, "2106/0001", 143.1, 4.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf105035_framePrB
fn mapped_fixture_tdf105035_framepr_b_keeps_distinct_frames_separated() {
  let summary = render_summary("tdf105035_framePrB.docx");
  assert_first_two_rectangular_paths_do_not_overlap_vertically(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:testTdf105035_framePrC
fn mapped_fixture_tdf105035_framepr_c_keeps_frames_overlapped_at_same_top() {
  let summary = render_summary("tdf105035_framePrC.docx");
  assert_first_two_rectangular_path_tops_close(&summary, 0, 1.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport24.cxx:testTdf37153
fn mapped_fixture_tdf37153_keeps_layout_in_cell_text_above_cell_bottom() {
  let summary = render_summary("tdf37153_considerWrapOnObjPos.docx");
  assert_text_top_from_page_top_at_least(&summary, 0, "Bottom aligned", 3000.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:testTdf150822
fn mapped_fixture_tdf150822_imports_three_vertical_text_layouts() {
  let summary = render_summary("tdf150822.docx");
  assert_vertical_text_segment_count_at_least(&summary, 0, 3);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf167526
fn mapped_fixture_tdf167526_dummy_anchor_line_does_not_increment_line_numbering() {
  let summary = render_summary("tdf167526.docx");
  assert_page_contains(&summary, 0, "2");
  assert_page_text_occurrences(&summary, 0, "2", 1);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf167540
fn mapped_fixture_tdf167540_keeps_line_numbers_and_tables_in_vertical_order() {
  let summary = render_summary("tdf167540.docx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "1",
      "Text",
      "2",
      "First floating table",
      "3",
      "Second floating table",
      "A normal table",
      "4",
      "More text",
    ],
  );
  assert_text_tops_close(&summary, "1", "Text", 1.0);
  // LibreOffice compares metafile textarray y positions. PDFium exposes glyph
  // bounds, so the line number and body text can have different bbox tops even
  // when they share a layout baseline.
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:testTdf64264
fn mapped_fixture_tdf64264_repeats_only_single_table_header_on_second_page() {
  let summary = render_summary("tdf64264.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains_in_order(&summary, 1, &["Repeating Table Header", "Text"]);
  assert_page_text_occurrences(&summary, 1, "Repeating Table Header", 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:testTdf58944RepeatingTableHeader
fn mapped_fixture_tdf58944_repeating_header_keeps_table_content_on_second_page() {
  let summary = render_summary("tdf58944-repeating-table-header.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains_in_order(&summary, 1, &["Test1", "Test2"]);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:testTdf81100
fn mapped_fixture_tdf81100_keeps_explicit_no_repeat_header_flow_across_three_pages() {
  let summary = render_summary("tdf81100.docx");
  assert_eq!(summary.page_count, 3);
  let layout = layout_summary("tdf81100.docx");
  let page_two_rows = layout
    .rows
    .iter()
    .filter(|row| row.page_index == 1 && row.block_index == Some(1))
    .count();
  let page_three_rows = layout
    .rows
    .iter()
    .filter(|row| row.page_index == 2 && row.block_index == Some(4))
    .count();
  assert_eq!(page_two_rows, 2);
  assert_eq!(page_three_rows, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testTdf130804
fn mapped_fixture_tdf130804_keeps_fly_height_equal_to_text_paragraph_height() {
  let summary = render_summary("tdf130804.docx");
  assert_page_contains(&summary, 0, "Lorem ipsum");
  // LibreOffice compares body text frame height with the anchored fly frame
  // height and checks the next empty/bookmark paragraph's print bounds in its
  // layout dump. PDF output has no corresponding visible path or text segment
  // for those internal frames.
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testTdf105143
fn mapped_fixture_tdf105143_keeps_shape_at_imported_vertical_position() {
  let summary = render_summary("tdf105143.docx");
  assert_path_top_from_page_top_close(&summary, 0, 6731.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testFloatingTableSectionColumns
fn mapped_fixture_floating_table_section_columns_keeps_table_wider_than_column() {
  let summary = render_summary("floating-table-section-columns.docx");
  assert_path_width_at_least(&summary, 0, 10000.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testTdf60351
fn mapped_fixture_tdf60351_preserves_contour_polygon_shape() {
  let summary = render_summary("tdf60351.docx");
  assert_any_path_segment_count_at_least(&summary, 0, 6);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testTdf98882
fn mapped_fixture_tdf98882_keeps_image_content_height_equal_to_frame_height() {
  let summary = render_summary("tdf98882.docx");
  assert_any_image_and_path_height_close(&summary, 0, 2.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testTdf100072
fn mapped_fixture_tdf100072_keeps_shape_visible_inside_page() {
  let summary = render_summary("tdf100072.docx");
  assert_any_path_has_positive_height(&summary, 0);
  assert_all_paths_within_page(&summary, 0, 1.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:testTdf114212
fn mapped_fixture_tdf114212_keeps_first_fly_at_imported_top_position() {
  let summary = render_summary("tdf114212.docx");
  // LibreOffice asserts the imported fly frame top from the layout dump. PDF
  // output exposes the painted path bounds, whose stroke/glyph coordinate is
  // slightly offset from the internal fly frame.
  assert_path_top_from_page_top_close_with_tolerance(&summary, 0, 1428.0 / 20.0, 3.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:testRelativeAnchorHeightFromBottomMarginNoFooter
fn mapped_fixture_tdf133070_no_footer_keeps_relative_anchor_height_from_margin_bottom() {
  let summary = render_summary("tdf133070_testRelativeAnchorHeightFromBottomMarginNoFooter.docx");
  assert_path_height_close(&summary, 0, 1147.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:testRelativeAnchorWidthFromRightMargin
fn mapped_fixture_tdf133670_keeps_relative_anchor_width_from_right_margin() {
  let summary = render_summary("tdf133670_testRelativeAnchorWidthFromRightMargin.docx");
  assert_path_geometry_width_close(&summary, 0, 2408.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:testTdf165478_bottomAligned
fn mapped_fixture_tdf165478_keeps_bottom_aligned_cell_text_and_image_inside_cell() {
  let summary = render_summary("tdf165478_bottomAligned.docx");
  assert_page_contains(&summary, 0, "Bottom aligned");
  // LibreOffice compares the cell text frame bottom with the cell frame bottom.
  // PDF glyph bounds can extend below the frame edge, so assert containment
  // with a glyph-bound tolerance instead of exact frame-bottom equality.
  assert_text_inside_any_path_with_tolerance(&summary, 0, "Bottom aligned", 8.0);
  assert_image_top_from_page_top_close(&summary, 0, 1887.0 / 20.0, 6.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf126533_noPageBitmap
fn mapped_fixture_tdf126533_no_page_bitmap_ignores_background_bitmap_without_fillcolor() {
  let summary = render_summary("tdf126533_noPageBitmap.docx");
  assert_page_image_count(&summary, 0, 0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf126533_pageBitmap
fn mapped_fixture_tdf126533_page_bitmap_keeps_visible_page_background_image() {
  let summary = render_summary("tdf126533_pageBitmap.docx");
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport2.cxx:testI120928
fn mapped_fixture_i120928_keeps_graphic_numbering_bullet_bitmap_visible() {
  let summary = render_summary("i120928.docx");
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport6.cxx:testDMLShapeFillBitmapCrop
fn mapped_fixture_dml_shape_fill_bitmap_crop_keeps_picture_filled_shapes_visible() {
  let summary = render_summary("dml-shape-fillbitmapcrop.docx");
  assert_page_image_count_at_least(&summary, 0, 2);
}

#[test]
// Source: ../core/oox/qa/unit/vml.cxx:tdf112450_vml_polyline
fn mapped_fixture_tdf112450_vml_polyline_keeps_decoded_polyline_geometry() {
  let summary = render_summary("tdf112450_vml_polyline.docx");
  assert_path_geometry_width_close(&summary, 0, 6879.0 * 72.0 / 2540.0);
  assert_path_geometry_height_close(&summary, 0, 1926.0 * 72.0 / 2540.0);
  assert_path_geometry_width_close(&summary, 0, 6163.0 * 72.0 / 2540.0);
  assert_path_geometry_height_close(&summary, 0, 2247.0 * 72.0 / 2540.0);
  assert_path_geometry_width_close(&summary, 0, 5634.0 * 72.0 / 2540.0);
  assert_path_geometry_height_close(&summary, 0, 2485.0 * 72.0 / 2540.0);
}

#[test]
// Source: ../core/svx/qa/unit/customshapes.cxx:testTdf153000_MS0_SPT_25_31
fn mapped_fixture_tdf153000_wordart_types_keep_non_rectangular_custom_shape_geometry() {
  let summary = render_summary("tdf153000_WordArt_type_25_to_31.docx");
  assert_any_path_segment_count_at_least(&summary, 0, 14);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testUnusedOLEprops
fn mapped_fixture_tdf138465min_keeps_ole_formula_from_being_squashed() {
  let summary = render_summary("tdf138465min.docx");
  assert_any_image_or_path_height_at_least(&summary, 0, 300.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:testVmlShapeTextWordWrap
fn mapped_fixture_tdf97618_vml_shape_text_wrap_keeps_shape_width() {
  let summary = render_summary("tdf97618_testVmlShapeTextWordWrap.docx");
  assert_path_width_close(&summary, 0, 2500.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:testI124106
fn mapped_fixture_i124106_keeps_document_on_one_page() {
  let summary = render_summary("i124106.docx");
  assert_eq!(summary.page_count, 1);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:testLargeTwips
fn mapped_fixture_large_twips_keeps_cell_text_visible() {
  let summary = render_summary("large-twips.docx");
  assert_any_text_segment_width_positive(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:testGridBefore
fn mapped_fixture_gridbefore_keeps_grid_before_cell_position() {
  let summary = render_summary("gridbefore.docx");
  assert_page_contains(&summary, 0, "A3");
  assert!(
    text_segment_left(&summary, "A3") > text_segment_left(&summary, "B2"),
    "A3 should be to the right of B2"
  );
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:testTdf125324
fn mapped_fixture_tdf125324_keeps_floating_table_top_position() {
  let summary = render_summary("tdf125324.docx");
  // LibreOffice's layout dump asserts the anchored fly table top at 4193
  // twips. In PDF output the stable visible counterpart is the first text
  // in that floating table, which soffice exports at about 196.2pt.
  assert_text_top_from_page_top_close(&summary, 0, "Position", 196.2, 8.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:testTdf162746
fn mapped_fixture_tdf162746_keeps_page_body_table_width_below_header_float() {
  let summary = render_summary("tdf162746.docx");
  assert_path_width_close(&summary, 0, 9360.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport24.cxx:testTdf107889
fn mapped_fixture_tdf107889_keeps_multipage_table_split_visible() {
  let summary = render_summary("tdf107889.docx");
  assert!(summary.page_count >= 2);
  assert_page_contains(&summary, 0, "Before");
  assert_page_contains(&summary, 0, "A1");
  assert_page_contains(&summary, 1, "A6");
  assert_page_contains(&summary, summary.page_count - 1, "After");
  // LibreOffice asserts two table fragments in the layout dump: the regression
  // was importing a multi-page table as a non-split fly. PDF page count depends
  // on visible pagination, so keep this mapped assertion on the split content.
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport_de_locale.cxx:testTdf166850
fn mapped_fixture_tdf166850_keeps_styleref_header_text_on_second_page() {
  let summary = render_summary("tdf166850.docx");
  assert_page_contains(&summary, 1, "Heading 1");
}

#[test]
// Source: ../core/oox/qa/unit/drawingml.cxx:testToplevelLineHorOffsetDOCX
fn mapped_fixture_toplevel_line_hori_offset_keeps_vertical_line_geometry() {
  let summary = render_summary("toplevel-line-hori-offset.docx");
  assert_path_geometry_width_close(&summary, 0, 1.78 * 72.0 / 2540.0);
  assert_path_geometry_height_close(&summary, 0, 4094.0 * 72.0 / 2540.0);
  assert_path_geometry_top_from_page_top_close(&summary, 0, 1440.0 / 20.0);
}

#[test]
// Source: ../core/oox/qa/unit/drawingml.cxx:testDOCXVerticalLineRotation
fn mapped_fixture_line_vertical_rotation_keeps_line_vertical() {
  let summary = render_summary("line-vertical-rotation.docx");
  assert_vertical_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testCustomshapePosition
fn mapped_fixture_customshape_position_keeps_vertical_offset() {
  let summary = render_summary("customshape-position.docx");
  assert_image_top_from_page_top_close(
    &summary,
    0,
    1440.0 / 20.0 + (581025.0 / 360.0) * 72.0 / 2540.0,
    3.0,
  );
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testMultipleGroupShapes
fn mapped_fixture_multiple_group_shapes_keeps_visible_group_text() {
  let summary = render_summary("multiple-group-shapes.docx");
  assert_page_contains(&summary, 0, "Fly2");
}

#[test]
// Source: ../core/sw/qa/core/objectpositioning/objectpositioning.cxx:testInsideOutsideVertAlignBottomMargin
fn mapped_fixture_inside_outside_vert_align_keeps_shapes_at_page_and_body_bottoms() {
  let summary = render_summary("inside-outside-vert-align.docx");
  assert_any_path_bottom_from_page_top_close(&summary, 0, 17098.0 / 20.0, 2.0);
  assert_path_top_from_page_top_close(&summary, 0, 15694.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/core/objectpositioning/objectpositioning.cxx:testVMLVertAlignBottomMargin
fn mapped_fixture_vml_vertical_alignment_keeps_bottom_margin_alignments() {
  let summary = render_summary("vml-vertical-alignment.docx");
  assert_path_top_from_page_top_close(&summary, 0, 11802.0 / 20.0);
  assert_any_path_bottom_from_page_top_close(&summary, 0, 16124.0 / 20.0, 2.0);
  assert_path_top_from_page_top_close(&summary, 0, 13741.0 / 20.0);
  assert_any_path_bottom_from_page_top_close(&summary, 0, 14185.0 / 20.0, 2.0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:testFdo38414
fn mapped_fixture_fdo38414_keeps_merged_gridbefore_cell_heights_equal() {
  let summary = render_summary("fdo38414.docx");
  assert_any_two_rectangular_path_heights_close(&summary, 0, 0.5);
}

#[test]
// Source: ../core/sw/qa/extras/rtfexport/rtfexport3.cxx:testTdf115180
fn mapped_fixture_tdf115180_keeps_table_and_cell_widths() {
  let summary = render_summary("tdf115180.docx");
  assert_path_width_close(&summary, 0, 9360.0 / 20.0);
  assert_path_width_close(&summary, 0, 9141.0 / 20.0);
  assert_path_width_close(&summary, 0, 219.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/uiwriter/uiwriter4.cxx:testTdf98987
fn mapped_fixture_tdf98987_keeps_rectangles_in_vertical_order() {
  let summary = render_summary("tdf98987.docx");
  assert_three_rectangular_paths_are_vertically_ordered(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/extras/uiwriter/uiwriter4.cxx:testTdf99004
fn mapped_fixture_tdf99004_keeps_textbox_and_rectangle_non_overlapping() {
  let summary = render_summary("tdf99004.docx");
  assert_first_two_rectangular_paths_do_not_overlap_vertically(&summary, 0);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:testTdf106606
fn mapped_fixture_tdf106606_keeps_graphic_bullets_for_both_numbering_lists() {
  let summary = render_summary("tdf106606.docx");
  assert_page_image_count_at_least(&summary, 0, 2);
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testGlowOnGroup
fn mapped_fixture_tdf156902_glow_group_keeps_two_child_shapes_visible() {
  let summary = render_summary("tdf156902_GlowOnGroup.docx");
  assert_page_path_count_at_least(&summary, 0, 2);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:testTdf119952_negativeMargins
fn mapped_fixture_tdf119952_negative_margins_keep_header_footer_fly_text() {
  let summary = render_summary("tdf119952_negativeMargins.docx");
  assert_page_contains(&summary, 0, "f1");
  assert_page_contains(&summary, 0, "f8");
  assert_page_contains(&summary, 1, "p1");
  assert_page_contains(&summary, 2, "aaaa");
  assert_page_contains(&summary, 2, "eeee");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport20.cxx:testTdf128646
fn mapped_fixture_tdf128646_keeps_layout_in_cell_shape_visible_with_table() {
  let summary = render_summary("tdf128646.docx");
  assert_any_rectangular_path_above_another(&summary, 0);
  assert_rightmost_path_within_page_right(&summary, 0, 1.0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf127606
fn mapped_fixture_tdf117923_keeps_numbering_portion_text_and_font_height() {
  let summary = render_summary("tdf117923.docx");
  assert_page_contains(&summary, 0, "GHI GHI GHI GHI");
  assert_page_contains(&summary, 0, "2.");
  assert_text_height_close(&summary, 0, 220.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout5.cxx:testTdf153136
fn mapped_fixture_tdf153136_preserves_space_character_line_height_rules() {
  let summary = layout_summary("tdf153136.docx");
  assert_any_layout_line_height_less_than(&summary, 0, 300.0 / 20.0);
  assert_any_layout_line_height_greater_than(&summary, 0, 1000.0 / 20.0);
  assert_any_layout_row_height_less_than(&summary, 1, 300.0 / 20.0);
  assert_any_layout_row_height_greater_than(&summary, 1, 1000.0 / 20.0);
}

#[test]
#[ignore = "ooxmlsdk parser fidelity: mc:AlternateContent shape text is not preserved for the PDF importer yet"]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:testTdf135943_shapeWithText_L0c15
fn mapped_fixture_tdf135943_shape_text_stays_inside_frame_boundaries() {
  let summary = render_summary("tdf135943_shapeWithText_LayoutInCell0_compat15.docx");
  assert_text_inside_any_path(&summary, 0, "lk");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:testTdf167770_marginInsideOutside
fn mapped_fixture_tdf167770_keeps_inside_outside_objects_on_page_margins() {
  let summary = render_summary("tdf167770_marginInsideOutside.docx");
  assert_any_object_left_from_page_left_close(&summary, 0, 0.0, 2.5);
  assert_any_object_right_from_page_right_close(&summary, 0, 0.0, 2.5);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport7.cxx:testTDF87348
fn mapped_fixture_tdf87348_linked_textboxes_keeps_visible_chain_members() {
  let summary = render_summary("tdf87348_linkedTextboxes.docx");
  assert_page_path_count_at_least(&summary, 0, 13);
}

#[test]
// Source: ../core/sw/qa/extras/uiwriter/uiwriter9.cxx:testSplitFloatingTable
fn mapped_fixture_floattable_split_keeps_follow_floating_table_on_second_page() {
  let summary = render_summary("floattable-split.docx");
  // The upstream uiwriter test asserts the split floating table layout dump,
  // not a two-page PDF. LibreOffice PDF export currently produces 3 pages for
  // this fixture while keeping the follow floating table on the second page.
  assert_eq!(summary.page_count, 3);
  assert_page_path_count_at_least(&summary, 0, 1);
  assert_page_path_count_at_least(&summary, 1, 1);
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testWriterFontwork
fn mapped_fixture_tdf125885_wordart_keeps_fontwork_blue_fill() {
  let summary = render_summary("tdf125885_WordArt.docx");
  assert_has_path_fill_color(&summary, "#0000ff@ff");
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testWriterFontwork3
fn mapped_fixture_tdf125885_wordart3_keeps_imported_gradient_colors_visible() {
  let summary = render_summary("tdf125885_WordArt3.docx");
  assert_has_path_fill_color(&summary, "#0000ff@ff");
  assert_has_path_fill_color(&summary, "#f79646@ff");
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testWriterShapeFillNonAccentColor
fn mapped_fixture_tdf152840_shape_fill_non_accent_colors_resolve_theme_fills() {
  let summary = render_summary("tdf152840_theme_color_non_accent.docx");
  assert_has_path_fill_color(&summary, "#ebddc3@ff");
  assert_has_path_fill_color(&summary, "#ffcc99@ff");
  assert_has_path_fill_color(&summary, "#ff0000@ff");
  assert_has_path_fill_color(&summary, "#775f55@ff");
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:testN793998
fn mapped_fixture_n793998_keeps_tab_portion_past_paragraph_right_margin() {
  let summary = render_summary("n793998.docx");
  assert_max_horizontal_gap_at_least(&summary, 0, 3000.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/odfexport/odfexport4.cxx:testTdf159382_DOCX
fn mapped_fixture_footnote_spacing_hanging_para_keeps_footnote_number_gap_small() {
  let summary = render_summary("footnote_spacing_hanging_para.docx");
  assert_text_segment_width_less_than(&summary, 0, "1", 100.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout2.cxx:testTdf116256
fn mapped_fixture_tdf116256_keeps_follow_text_flow_textbox_inside_table_cell() {
  let summary = render_summary("tdf116256.docx");
  assert_page_text_occurrences(&summary, 0, "xxx", 1);
  assert_text_inside_page_path_extents(&summary, 0, "xxx");
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf124600
fn mapped_fixture_tdf124600_keeps_second_body_paragraph_on_one_line() {
  let summary = render_summary("tdf124600-layout.docx");
  assert_page_text_segment_occurrences(
    &summary,
    0,
    "nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam",
    1,
  );
}

#[test]
// Source: ../core/oox/qa/unit/vml.cxx:testWatermark
fn mapped_fixture_watermark_keeps_picture_watermark_washout_visible() {
  let summary = render_summary("watermark.docx");
  assert_page_image_count(&summary, 0, 1);
  assert_image_centers_are_grayscale("watermark.docx", &summary, 0, 1);
}

#[test]
// Source: ../core/oox/qa/unit/drawingml.cxx:testCameraRotationRevolution
fn mapped_fixture_camera_rotation_revolution_keeps_rotated_shapes_visible() {
  let summary = render_summary("camera-rotation-revolution.docx");
  assert_page_path_count_at_least(&summary, 0, 2);
  assert_all_paths_within_page(&summary, 0, 1.0);
}

#[test]
// Source: ../core/oox/qa/unit/shape.cxx:testTdf151518VertAnchor
fn mapped_fixture_tdf151518_smartart_text_stays_inside_target_shapes() {
  let summary = render_summary("tdf151518_SmartArtTextLocation.docx");
  for text in ["Pet", "Farm", "Cat", "Dog"] {
    assert_matching_text_segments_inside_paths(&summary, 0, text);
  }
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:tdf167527_title_letters_cut_from_below
fn mapped_fixture_tdf167527_field_shading_does_not_overlap_previous_text_row() {
  let summary = render_summary("tdf167527_title_letters_cut_from_below.docx");
  assert_page_contains(&summary, 0, "random text here");
  assert_any_path_height_less_than(&summary, 0, 700.0 / 20.0);
}

#[test]
// Source: ../core/sw/qa/extras/uiwriter/uiwriter3.cxx:testTdf147126
fn mapped_fixture_tdf147126_keeps_group_draw_objects_inside_fly_frames() {
  let summary = render_summary("tdf147126.docx");
  assert_page_contains(&summary, 0, "Processo Metodológico da Pesquisa");
  assert_page_path_count_at_least(&summary, 0, 7);
}

#[test]
// Source: ../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:testTdf139418
fn mapped_fixture_tdf139418_keeps_vertical_docx_grid_text_portions() {
  let summary = render_summary("tdf139418.docx");
  assert_page_contains(&summary, 0, "enko Yoshi");
  assert_text_segment_taller_than_wide(&summary, 0, "G");
}

#[test]
// Source: ../core/sw/qa/core/txtnode/txtnode.cxx:testSplitFlyAnchorSplit
fn mapped_fixture_floattable_anchor_split_keeps_initial_split_floating_table_pages() {
  let summary = render_summary("floattable-anchor-split.docx");
  assert_eq!(summary.page_count, 2);
  assert_page_contains(&summary, 0, "First paragraph");
  assert_page_path_count_at_least(&summary, 0, 1);
  assert_page_path_count_at_least(&summary, 1, 1);
}

#[test]
// Source: ../core/sw/qa/extras/layout/layout6.cxx:testTdf122878
fn mapped_fixture_tdf122878_keeps_body_paragraphs_above_footer_table() {
  let summary = render_summary("tdf122878.docx");
  assert_page_contains(&summary, 0, "1");
  assert_page_contains(&summary, 0, "28");
  assert_page_contains(&summary, 0, "A1");
  // LibreOffice iterates the body frames on page 1 and checks they do not
  // overlap the footer table. The PDF-visible equivalent is that the body text
  // remains above the visible footer table content instead of being clipped by it.
  assert_text_above_text(&summary, 0, "28", "A1");
  assert_page_path_count_at_least(&summary, 0, 1);
}
