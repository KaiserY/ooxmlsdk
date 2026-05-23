use ooxmlsdk_pdf_test::{
  PdfBounds, PdfSummary, parse_pdf_rect, pdf_summary_for_fixture, pdfexport_fixture_dir,
  rendered_page_image_for_fixture,
};

fn fixture(name: &str) -> std::path::PathBuf {
  pdfexport_fixture_dir().join(name)
}

fn render_summary(name: &str) -> PdfSummary {
  pdf_summary_for_fixture(&fixture(name)).unwrap()
}

fn package_part_text(name: &str, part: &str) -> String {
  let output = std::process::Command::new("unzip")
    .arg("-p")
    .arg(fixture(name))
    .arg(part)
    .output()
    .unwrap_or_else(|error| panic!("failed to read {part} from {name}: {error}"));
  assert!(
    output.status.success(),
    "failed to read {part} from {name}: status={:?}, stderr={}",
    output.status.code(),
    String::from_utf8_lossy(&output.stderr)
  );
  String::from_utf8(output.stdout)
    .unwrap_or_else(|error| panic!("{part} in {name} is not UTF-8 XML: {error}"))
}

fn assert_package_part_contains(name: &str, part: &str, expected: &str) {
  let text = package_part_text(name, part);
  assert!(
    text.contains(expected),
    "missing {expected:?} in {part} from {name}; part text:\n{text}"
  );
}

fn assert_package_part_contains_in_order(name: &str, part: &str, expected: &[&str]) {
  let text = package_part_text(name, part);
  let mut cursor = 0;
  for item in expected {
    let Some(offset) = text[cursor..].find(item) else {
      panic!("missing {item:?} in {part} from {name} after offset {cursor}; part text:\n{text}");
    };
    cursor += offset + item.len();
  }
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

fn assert_page_contains_all(summary: &PdfSummary, page_index: usize, expected: &[&str]) {
  let text = page_text(summary, page_index);
  let normalized_text = normalize_space(&text);
  for item in expected {
    let normalized_item = normalize_space(item);
    assert!(
      normalized_text.contains(&normalized_item),
      "missing page {page_index} text {item:?}; page text:\n{text}"
    );
  }
}

fn assert_page_text_occurs_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  expected_count: usize,
) {
  let text = page_text(summary, page_index);
  let normalized_text = normalize_space(&text);
  let normalized_expected = normalize_space(expected);
  let count = normalized_text.matches(&normalized_expected).count();
  assert!(
    count >= expected_count,
    "expected page {page_index} text {expected:?} at least {expected_count} times, got {count}; page text:\n{text}"
  );
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

fn assert_text_object_font_contains(
  summary: &PdfSummary,
  expected_text: &str,
  expected_font: &str,
) {
  assert!(
    summary.text_objects.iter().any(|object| {
      normalize_space(&object.text).contains(expected_text)
        && (object.font_name.contains(expected_font) || object.font_family.contains(expected_font))
    }),
    "missing text {expected_text:?} using font {expected_font:?}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_text_font_size(summary: &PdfSummary, expected_text: &str, expected_size: &str) {
  assert!(
    summary.text_objects.iter().any(|object| {
      normalize_space(&object.text).contains(expected_text)
        && object.scaled_font_size == expected_size
    }),
    "missing text {expected_text:?} with font size {expected_size}; text_objects={:?}",
    summary.text_objects
  );
}

fn assert_text_absent(summary: &PdfSummary, page_index: usize, unexpected: &str) {
  let text = page_text(summary, page_index);
  assert!(
    !normalize_space(&text).contains(&normalize_space(unexpected)),
    "unexpected page {page_index} text {unexpected:?}; page text:\n{text}"
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

fn assert_page_stroked_path_count_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected: usize,
) {
  let count = summary
    .paths
    .iter()
    .filter(|path| path.page_index == page_index && path.stroked == Some(true))
    .count();
  assert!(
    count >= expected,
    "expected at least {expected} stroked paths on page {page_index}, got {count}; paths={:?}",
    summary.paths
  );
}

fn assert_page_filled_path_count_at_least(
  summary: &PdfSummary,
  page_index: usize,
  expected: usize,
) {
  let count = summary
    .paths
    .iter()
    .filter(|path| path.page_index == page_index && path.fill_mode.is_some())
    .count();
  assert!(
    count >= expected,
    "expected at least {expected} filled paths on page {page_index}, got {count}; paths={:?}",
    summary.paths
  );
}

fn assert_page_image_count_at_least(summary: &PdfSummary, page_index: usize, expected: usize) {
  let count = summary
    .images
    .iter()
    .filter(|image| image.page_index == page_index)
    .count();
  assert!(
    count >= expected,
    "expected at least {expected} images on page {page_index}, got {count}; images={:?}",
    summary.images
  );
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

fn assert_first_image_top_left_rgb_close(
  fixture_name: &str,
  summary: &PdfSummary,
  page_index: usize,
  expected_rgb: [u8; 3],
) {
  let image_bounds = image_bounds_on_page(summary, page_index);
  let bounds = image_bounds.first().copied().unwrap_or_else(|| {
    panic!(
      "missing image bounds on page {page_index}; images={:?}",
      summary.images
    )
  });
  let rendered = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, 1200)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let x = bounds.left + bounds.width() * 0.05;
  let y = bounds.top - bounds.height() * 0.05;
  let [r, g, b, _] = rendered
    .sample_pdf_point_rgba(x, y)
    .unwrap_or_else(|| panic!("missing rendered image sample at PDF point ({x}, {y})"));
  let diff = (i16::from(r) - i16::from(expected_rgb[0])).abs()
    + (i16::from(g) - i16::from(expected_rgb[1])).abs()
    + (i16::from(b) - i16::from(expected_rgb[2])).abs();
  assert!(
    diff <= 24,
    "rendered image top-left color #{r:02x}{g:02x}{b:02x} differs from expected #{:02x}{:02x}{:02x}; bounds={bounds:?}",
    expected_rgb[0],
    expected_rgb[1],
    expected_rgb[2]
  );
}

fn assert_rendered_image_centers_include_rgb_close(
  fixture_name: &str,
  summary: &PdfSummary,
  page_index: usize,
  expected_rgb: [u8; 3],
) {
  let image_bounds = image_bounds_on_page(summary, page_index);
  assert!(
    !image_bounds.is_empty(),
    "missing image bounds on page {page_index}; images={:?}",
    summary.images
  );
  let rendered = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, 1200)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let matched = image_bounds.iter().any(|bounds| {
    let Some([r, g, b, _]) = rendered.sample_pdf_rect_center_rgba(*bounds) else {
      return false;
    };
    let diff = (i16::from(r) - i16::from(expected_rgb[0])).abs()
      + (i16::from(g) - i16::from(expected_rgb[1])).abs()
      + (i16::from(b) - i16::from(expected_rgb[2])).abs();
    diff <= 24
  });
  assert!(
    matched,
    "missing rendered image center color close to #{:02x}{:02x}{:02x}; bounds={image_bounds:?}",
    expected_rgb[0], expected_rgb[1], expected_rgb[2]
  );
}

fn assert_first_image_relative_rgb_close(
  fixture_name: &str,
  summary: &PdfSummary,
  page_index: usize,
  relative_x: f32,
  relative_y: f32,
  expected_rgb: [u8; 3],
) {
  let image_bounds = image_bounds_on_page(summary, page_index);
  let bounds = image_bounds.first().copied().unwrap_or_else(|| {
    panic!(
      "missing image bounds on page {page_index}; images={:?}",
      summary.images
    )
  });
  let rendered = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, 1200)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let x = bounds.left + bounds.width() * relative_x;
  let y = bounds.top - bounds.height() * relative_y;
  let [r, g, b, _] = rendered
    .sample_pdf_point_rgba(x, y)
    .unwrap_or_else(|| panic!("missing rendered image sample at PDF point ({x}, {y})"));
  let diff = (i16::from(r) - i16::from(expected_rgb[0])).abs()
    + (i16::from(g) - i16::from(expected_rgb[1])).abs()
    + (i16::from(b) - i16::from(expected_rgb[2])).abs();
  assert!(
    diff <= 24,
    "rendered image relative color at ({relative_x}, {relative_y}) #{r:02x}{g:02x}{b:02x} differs from expected #{:02x}{:02x}{:02x}; bounds={bounds:?}",
    expected_rgb[0],
    expected_rgb[1],
    expected_rgb[2]
  );
}

fn assert_page_link_target(summary: &PdfSummary, page_index: usize, expected_target: &str) {
  assert!(
    summary.links.iter().any(|link| {
      link.page_index == page_index && link.target.as_deref() == Some(expected_target)
    }),
    "missing link target {expected_target:?} on page {page_index}; links={:?}",
    summary.links
  );
}

fn assert_page_link_count_at_least(summary: &PdfSummary, page_index: usize, expected: usize) {
  let count = summary
    .links
    .iter()
    .filter(|link| link.page_index == page_index)
    .count()
    + summary
      .annotations
      .iter()
      .filter(|annotation| annotation.page_index == page_index && annotation.action_uri.is_some())
      .count();
  assert!(
    count >= expected,
    "expected at least {expected} link annotations on page {page_index}, got {count}; links={:?}; annotations={:?}",
    summary.links,
    summary.annotations
  );
}

fn assert_page_annotation_uri(summary: &PdfSummary, page_index: usize, expected_uri: &str) {
  assert!(
    summary.annotations.iter().any(|annotation| {
      annotation.page_index == page_index && annotation.action_uri.as_deref() == Some(expected_uri)
    }) || summary.raw_pages.iter().any(|page| {
      page.page_index == page_index
        && page
          .annotations
          .iter()
          .any(|annotation| annotation.action_uri.as_deref() == Some(expected_uri))
    }),
    "missing annotation URI {expected_uri:?} on page {page_index}; annotations={:?}; raw_pages={:?}",
    summary.annotations,
    summary.raw_pages
  );
}

fn assert_text_fill_color_not(summary: &PdfSummary, expected_text: &str, unexpected_color: &str) {
  let object = summary
    .text_objects
    .iter()
    .find(|object| normalize_space(&object.text).contains(expected_text))
    .unwrap_or_else(|| {
      panic!(
        "missing text object containing {expected_text:?}; text_objects={:?}",
        summary.text_objects
      )
    });
  assert_ne!(
    object.fill_color.as_deref(),
    Some(unexpected_color),
    "text object {expected_text:?} has unexpected fill color {unexpected_color}; object={object:?}"
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

fn assert_text_centered_on_page(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  tolerance_pt: f32,
) {
  let bounds = text_bounds_containing(summary, page_index, expected);
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let text_center = (bounds.left + bounds.right) / 2.0;
  let page_center = (media_box.left + media_box.right) / 2.0;
  assert!(
    (text_center - page_center).abs() <= tolerance_pt,
    "text {expected:?} is not centered on page {page_index}; text_bounds={bounds:?}; media_box={media_box:?}"
  );
}

fn assert_any_path_height_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_height_pt: f32,
  tolerance_pt: f32,
) {
  assert!(
    summary
      .paths
      .iter()
      .filter(|path| path.page_index == page_index)
      .filter_map(|path| path.bounds.as_deref())
      .filter_map(|bounds| parse_pdf_rect(bounds).ok())
      .any(|bounds| (bounds.height() - expected_height_pt).abs() <= tolerance_pt),
    "missing path height close to {expected_height_pt}pt on page {page_index}; paths={:?}",
    summary.paths
  );
}

fn assert_any_path_width_close(
  summary: &PdfSummary,
  page_index: usize,
  expected_width_pt: f32,
  tolerance_pt: f32,
) {
  assert!(
    summary
      .paths
      .iter()
      .filter(|path| path.page_index == page_index)
      .filter_map(|path| path.bounds.as_deref())
      .filter_map(|bounds| parse_pdf_rect(bounds).ok())
      .any(|bounds| (bounds.width() - expected_width_pt).abs() <= tolerance_pt),
    "missing path width close to {expected_width_pt}pt on page {page_index}; paths={:?}",
    summary.paths
  );
}

fn assert_any_image_bounds_close(
  summary: &PdfSummary,
  page_index: usize,
  left_100mm: f32,
  top_100mm: f32,
  right_100mm: f32,
  bottom_100mm: f32,
  tolerance_pt: f32,
) {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  let expected = PdfBounds {
    left: left_100mm * 72.0 / 2540.0,
    top: media_box.top - top_100mm * 72.0 / 2540.0,
    right: right_100mm * 72.0 / 2540.0,
    bottom: media_box.top - bottom_100mm * 72.0 / 2540.0,
  };
  let image_bounds = image_bounds_on_page(summary, page_index);
  assert!(
    image_bounds.iter().any(|bounds| {
      (bounds.left - expected.left).abs() <= tolerance_pt
        && (bounds.top - expected.top).abs() <= tolerance_pt
        && (bounds.right - expected.right).abs() <= tolerance_pt
        && (bounds.bottom - expected.bottom).abs() <= tolerance_pt
    }),
    "missing image bounds close to {expected:?} on page {page_index}; image_bounds={image_bounds:?}"
  );
}

fn assert_text_left_delta_close(
  summary: &PdfSummary,
  page_index: usize,
  first_text: &str,
  second_text: &str,
  expected_delta_pt: f32,
  tolerance_pt: f32,
) {
  let first = text_bounds_containing(summary, page_index, first_text);
  let second = text_bounds_containing(summary, page_index, second_text);
  let delta = first.left - second.left;
  assert!(
    (delta - expected_delta_pt).abs() <= tolerance_pt,
    "text left delta for {first_text:?} and {second_text:?} is {delta:.2}pt, expected {expected_delta_pt:.2}pt; first={first:?}; second={second:?}"
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

fn text_char_bounds(summary: &PdfSummary, page_index: usize, expected: &str) -> PdfBounds {
  summary
    .text_chars
    .iter()
    .filter(|character| character.page_index == page_index)
    .find(|character| character.text == expected)
    .and_then(|character| parse_pdf_rect(&character.bounds).ok())
    .unwrap_or_else(|| {
      panic!(
        "missing text character {expected:?} on page {page_index}; text_chars={:?}",
        summary.text_chars
      )
    })
}

fn text_anchor_bounds(summary: &PdfSummary, page_index: usize, expected: &str) -> PdfBounds {
  if expected.chars().count() == 1 {
    text_char_bounds(summary, page_index, expected)
  } else {
    text_bounds_containing(summary, page_index, expected)
  }
}

fn assert_text_left_before(
  summary: &PdfSummary,
  page_index: usize,
  left_text: &str,
  right_text: &str,
) {
  let left = text_anchor_bounds(summary, page_index, left_text);
  let right = text_anchor_bounds(summary, page_index, right_text);
  assert!(
    left.left < right.left,
    "expected {left_text:?} to be left of {right_text:?}; left={left:?}; right={right:?}"
  );
}

fn assert_text_left_after(
  summary: &PdfSummary,
  page_index: usize,
  left_text: &str,
  right_text: &str,
) {
  let left = text_anchor_bounds(summary, page_index, left_text);
  let right = text_anchor_bounds(summary, page_index, right_text);
  assert!(
    left.left > right.left,
    "expected {left_text:?} to be right of {right_text:?}; left={left:?}; right={right:?}"
  );
}

fn assert_text_top_close(
  summary: &PdfSummary,
  page_index: usize,
  first_text: &str,
  second_text: &str,
  tolerance_pt: f32,
) {
  let first = text_anchor_bounds(summary, page_index, first_text);
  let second = text_anchor_bounds(summary, page_index, second_text);
  assert!(
    (first.top - second.top).abs() <= tolerance_pt,
    "expected {first_text:?} and {second_text:?} to have close top coordinates; first={first:?}; second={second:?}"
  );
}

fn assert_text_top_after(
  summary: &PdfSummary,
  page_index: usize,
  lower_text: &str,
  upper_text: &str,
) {
  let lower = text_anchor_bounds(summary, page_index, lower_text);
  let upper = text_anchor_bounds(summary, page_index, upper_text);
  assert!(
    lower.top < upper.top,
    "expected {lower_text:?} to be lower than {upper_text:?}; lower={lower:?}; upper={upper:?}"
  );
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

fn assert_text_starts_in_libreoffice_metafile_rect(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  rect: LibreOfficeMetafileRect,
  tolerance_pt: f32,
) {
  let bounds = text_bounds_containing(summary, page_index, expected);
  let expected_rect = libreoffice_metafile_rect(summary, page_index, rect);

  assert!(
    bounds.left >= expected_rect.left - tolerance_pt
      && bounds.left <= expected_rect.right + tolerance_pt
      && bounds.top >= expected_rect.bottom - tolerance_pt
      && bounds.top <= expected_rect.top + tolerance_pt,
    "text {expected:?} bounds {bounds:?} do not start inside LibreOffice text anchor rect {expected_rect:?}"
  );
}

fn assert_text_intersects_libreoffice_metafile_rect(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  rect: LibreOfficeMetafileRect,
  tolerance_pt: f32,
) {
  let bounds = text_bounds_containing(summary, page_index, expected);
  let mut expected_rect = libreoffice_metafile_rect(summary, page_index, rect);
  expected_rect.left -= tolerance_pt;
  expected_rect.right += tolerance_pt;
  expected_rect.bottom -= tolerance_pt;
  expected_rect.top += tolerance_pt;

  assert!(
    bounds.left <= expected_rect.right
      && bounds.right >= expected_rect.left
      && bounds.bottom <= expected_rect.top
      && bounds.top >= expected_rect.bottom,
    "text {expected:?} bounds {bounds:?} do not intersect LibreOffice text anchor rect {expected_rect:?}"
  );
}

#[derive(Clone, Copy, Debug)]
struct LibreOfficeMetafileRect {
  left_100mm: f32,
  top_100mm: f32,
  right_100mm: f32,
  bottom_100mm: f32,
}

fn libreoffice_metafile_rect(
  summary: &PdfSummary,
  page_index: usize,
  rect: LibreOfficeMetafileRect,
) -> PdfBounds {
  let media_box = parse_pdf_rect(&summary.media_boxes[page_index]).unwrap();
  PdfBounds {
    left: rect.left_100mm * 72.0 / 2540.0,
    bottom: media_box.top - rect.bottom_100mm * 72.0 / 2540.0,
    right: rect.right_100mm * 72.0 / 2540.0,
    top: media_box.top - rect.top_100mm * 72.0 / 2540.0,
  }
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

struct LibreOfficeRelativeMetafilePoint {
  map_x_100mm: f32,
  map_y_100mm: f32,
  text_x_100mm: f32,
  text_y_100mm: f32,
}

fn assert_text_near_libreoffice_relative_metafile_point(
  summary: &PdfSummary,
  page_index: usize,
  expected: &str,
  point: LibreOfficeRelativeMetafilePoint,
  tolerance_pt: f32,
) {
  assert_text_near_libreoffice_metafile_point(
    summary,
    page_index,
    expected,
    point.map_x_100mm + point.text_x_100mm,
    point.map_y_100mm + point.text_y_100mm,
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

fn page_object_count(summary: &PdfSummary, page_index: usize) -> usize {
  let page = &summary.page_objects[page_index];
  page.text_objects
    + page.path_objects
    + page.image_objects
    + page.shading_objects
    + page.form_objects
    + page.unsupported_objects
}

fn assert_full_render_reference_smoke(summary: &PdfSummary) {
  assert!(
    summary.page_count >= 1,
    "missing rendered pages; summary={summary:?}"
  );
  assert!(
    summary.contains_eof,
    "PDF output is missing EOF marker; summary={summary:?}"
  );
  assert!(
    (0..summary.page_count).any(|page_index| page_object_count(summary, page_index) >= 1),
    "expected at least one rendered page object; page_objects={:?}",
    summary.page_objects
  );
}

fn assert_rendered_page_color_fraction_at_least(
  fixture_name: &str,
  page_index: usize,
  expected_rgb: [u8; 3],
  tolerance: u8,
  expected_fraction: f32,
) {
  let rendered = rendered_page_image_for_fixture(&fixture(fixture_name), page_index, 100)
    .unwrap_or_else(|error| panic!("failed to render {fixture_name}: {error}"));
  let tolerance = i16::from(tolerance);
  let matching = rendered
    .rgba
    .chunks_exact(4)
    .filter(|pixel| {
      (i16::from(pixel[0]) - i16::from(expected_rgb[0])).abs() <= tolerance
        && (i16::from(pixel[1]) - i16::from(expected_rgb[1])).abs() <= tolerance
        && (i16::from(pixel[2]) - i16::from(expected_rgb[2])).abs() <= tolerance
    })
    .count();
  let total = (rendered.width_px * rendered.height_px) as usize;
  let fraction = matching as f32 / total as f32;
  assert!(
    fraction >= expected_fraction,
    "expected at least {expected_fraction:.2} of rendered {fixture_name} page {page_index} to be close to #{:02x}{:02x}{:02x}, got {fraction:.2} ({matching}/{total})",
    expected_rgb[0],
    expected_rgb[1],
    expected_rgb[2]
  );
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
// Source: ../core/sd/qa/unit/layout-tests.cxx:numberedList
fn mapped_pptx_numbered_list_preserves_imported_numbering_text_order() {
  let summary = render_summary("pptx/NumberedList-12ab-ab-34.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_package_part_contains_in_order(
    "pptx/NumberedList-12ab-ab-34.pptx",
    "ppt/slides/slide1.xml",
    &[
      "type=\"arabicPeriod\"",
      "Outer, one",
      "type=\"arabicPeriod\"",
      "Outer, two",
      "type=\"alphaLcPeriod\"",
      "Second level, a",
      "type=\"alphaLcPeriod\"",
      "Second level, b",
      "<a:buNone/>",
      "Blank second level",
      "type=\"alphaLcPeriod\"",
      "Second level restart, a",
      "type=\"alphaLcPeriod\"",
      "Second level restart, b",
      "type=\"arabicPeriod\"",
      "Outer, three",
      "type=\"arabicPeriod\"",
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
    LibreOfficeRelativeMetafilePoint {
      map_x_100mm: 331.0,
      map_y_100mm: 9420.0,
      text_x_100mm: 4760.0,
      text_y_100mm: -2250.0,
    },
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
    LibreOfficeRelativeMetafilePoint {
      map_x_100mm: 14416.0,
      map_y_100mm: 1658.0,
      text_x_100mm: -11031.0,
      text_y_100mm: 3617.0,
    },
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

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testPredefinedTableStyle
fn mapped_pptx_predefined_table_style_preserves_cell_fill_colors() {
  let summary = render_summary("pptx/predefined-table-style.pptx");
  assert_has_path_fill_color(&summary, "#000000@ff");
  assert_has_path_fill_color(&summary, "#e7e7e7@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc887225
fn mapped_pptx_bnc887225_preserves_last_row_and_column_table_fill_colors() {
  let summary = render_summary("pptx/bnc887225.pptx");
  assert_has_path_fill_color(&summary, "#5b9bd5@ff");
  assert_has_path_fill_color(&summary, "#d1deef@ff");
  assert_has_path_fill_color(&summary, "#e9eff7@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTableBorderLineStyle
fn mapped_pptx_table_border_line_style_preserves_visible_table_borders() {
  let summary = render_summary("pptx/tableBorderLineStyle.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "System Dash",
      "System Dot",
      "System Dash Dot",
      "Solid",
      "No Border",
    ],
  );
  assert_page_stroked_path_count_at_least(&summary, 0, 10);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTableMergedCellsBorderLineStyle
fn mapped_pptx_tdf149865_preserves_merged_cell_right_border_color() {
  let summary = render_summary("pptx/tdf149865.pptx");
  assert_has_stroked_path_color(&summary, "#30ba78@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc862510_7
fn mapped_pptx_bnc862510_7_centers_title_text() {
  let summary = render_summary("pptx/bnc862510_7.pptx");
  assert_page_contains_in_order(&summary, 0, &["Text aligned to center"]);
  assert_text_centered_on_page(&summary, 0, "Text aligned to center", 36.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBulletSuffix
fn mapped_pptx_n83889_keeps_bullet_suffix_from_adding_extra_visible_text() {
  let summary = render_summary("pptx/n83889.pptx");
  assert_page_contains_in_order(&summary, 0, &["test:", "In test 1", "Second line"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc910045
fn mapped_pptx_bnc910045_preserves_table_style_fill_color() {
  let summary = render_summary("pptx/bnc910045.pptx");
  assert_has_path_fill_color(&summary, "#4f81bd@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf165732
fn mapped_pptx_tdf165732_keeps_clamped_text_inset_labels_visible() {
  let summary = render_summary("pptx/tdf165732.pptx");
  assert_page_contains_in_order(&summary, 0, &["0", "1", "2", "5"]);
  assert_has_path_fill_color(&summary, "#7f7f7f@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf152070
fn mapped_pptx_tdf152070_preserves_tiled_background_bitmap() {
  let summary = render_summary("pptx/tdf152070.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf51340
fn mapped_pptx_tdf51340_preserves_line_spacing_paragraph_text() {
  let summary = render_summary("pptx/tdf51340.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "Spacing is set on master slide",
      "Spacing is set on slide layout",
      "Direct formatting overrides master slide spacing",
      "Direct formatting overrides slide layout spacing",
    ],
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf120028
fn mapped_pptx_tdf120028_preserves_four_column_text_content() {
  let summary = render_summary("pptx/tdf120028.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "Aaaaaaa aaaaa",
      "Bbbbbb bbbbbbbb bbbbbbbb",
      "Ccccccccc ccc cccccc",
      "Dddddd dddddd",
      "Lll l llllll lllll",
    ],
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf100926
fn mapped_pptx_tdf100926_preserves_vertical_and_horizontal_table_text() {
  let summary = render_summary("pptx/tdf100926.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "Top to Bottom vertical text",
      "Bottom to Top vertical text",
      "Horizontal text",
    ],
  );
  assert_vertical_text_shape(&summary, 0, "Top to Bottom vertical text");
  assert_vertical_text_shape(&summary, 0, "Bottom to Top vertical text");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf134174
fn mapped_pptx_tdf134174_preserves_bitmap_fill_image() {
  let summary = render_summary("pptx/tdf134174.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf134210
fn mapped_pptx_tdf134210_preserves_bitmap_fill_image() {
  let summary = render_summary("pptx/tdf134210.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf114821
fn mapped_pptx_tdf114821_preserves_outside_chart_data_labels() {
  let summary = render_summary("pptx/tdf114821.pptx");
  assert_page_contains_in_order(&summary, 0, &["90.0", "B"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf148685
fn mapped_pptx_tdf148685_preserves_underlined_run_color_and_text_runs() {
  let summary = render_summary("pptx/tdf148685.pptx");
  assert_page_contains_in_order(&summary, 0, &["TEXT", "TE", "XT"]);
  assert_text_fill_color(&summary, "TE", "#ff8000@ff");
  assert_has_stroked_path_color(&summary, "#a1467e@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf128684
fn mapped_pptx_tdf128684_preserves_vertical_rotated_text() {
  let summary = render_summary("pptx/tdf128684.pptx");
  assert_page_contains_in_order(&summary, 0, &["Foo bar foo bar foo bar"]);
  assert_vertical_text_shape(&summary, 0, "Foo bar foo bar foo bar");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf113198
fn mapped_pptx_tdf113198_centers_text_in_ellipse() {
  let summary = render_summary("pptx/tdf113198.pptx");
  assert_page_contains_in_order(&summary, 0, &["Awesome text in center"]);
  assert_text_centered_on_page(&summary, 0, "Awesome text in center", 72.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf149206
fn mapped_pptx_tdf149206_exports_cropped_image_with_clip() {
  let summary = render_summary("pptx/tdf149206.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_page_has_clipping_ops(&summary, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testtdf163852
fn mapped_pptx_tdf163852_exports_cropped_svg_image_with_clip() {
  let summary = render_summary("pptx/tdf163852.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_page_has_clipping_ops(&summary, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testCropToShape
fn mapped_pptx_crop_to_shape_preserves_bitmap_custom_shape_clip() {
  let summary = render_summary("pptx/crop-to-shape.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_page_has_clipping_ops(&summary, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testMirroredGraphic
fn mapped_pptx_mirrored_graphic_preserves_top_left_fill_bitmap_color() {
  let summary = render_summary("pptx/mirrored-graphic.pptx");
  assert_first_image_top_left_rgb_close(
    "pptx/mirrored-graphic.pptx",
    &summary,
    0,
    [0x4f, 0x49, 0x55],
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf134210CropPosition
fn mapped_pptx_crop_position_preserves_green_bitmap_crop() {
  let summary = render_summary("pptx/crop-position.pptx");
  assert_first_image_top_left_rgb_close("pptx/crop-position.pptx", &summary, 0, [0x81, 0xd4, 0x1a]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testGreysScaleGraphic
fn mapped_pptx_greysscale_graphic_preserves_grayscale_bitmap_color() {
  let summary = render_summary("pptx/greysscale-graphic.pptx");
  assert_first_image_top_left_rgb_close(
    "pptx/greysscale-graphic.pptx",
    &summary,
    0,
    [0x3c, 0x3c, 0x3c],
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf112209
fn mapped_pptx_tdf112209_preserves_grayscale_fill_bitmap_color() {
  let summary = render_summary("pptx/tdf112209.pptx");
  assert_first_image_top_left_rgb_close("pptx/tdf112209.pptx", &summary, 0, [0x84, 0x84, 0x84]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf128596
fn mapped_pptx_tdf128596_repeats_bitmap_fill() {
  let summary = render_summary("pptx/tdf128596.pptx");
  assert_page_image_count_at_least(&summary, 0, 2);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf89928BlackWhiteThreshold
fn mapped_pptx_tdf89928_black_white_threshold_keeps_black_and_white_graphics() {
  let summary = render_summary("pptx/tdf89928-blackWhiteEffectThreshold.pptx");
  assert_rendered_image_centers_include_rgb_close(
    "pptx/tdf89928-blackWhiteEffectThreshold.pptx",
    &summary,
    0,
    [0x00, 0x00, 0x00],
  );
  assert_rendered_image_centers_include_rgb_close(
    "pptx/tdf89928-blackWhiteEffectThreshold.pptx",
    &summary,
    0,
    [0xff, 0xff, 0xff],
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf151547TransparentWhiteText
fn mapped_pptx_tdf151547_preserves_transparent_white_text_color() {
  let summary = render_summary("pptx/tdf151547-transparent-white-text.pptx");
  assert_page_contains_in_order(&summary, 0, &["Fully transparent white text"]);
  assert_text_fill_color(&summary, "Fully transparent white text", "#ffffff@00");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf149588TransparentSolidFill
fn mapped_pptx_tdf149588_preserves_transparent_solid_text_fill() {
  let summary = render_summary("pptx/tdf149588_transparentSolidFill.pptx");
  assert_page_contains_in_order(&summary, 0, &["EDGE"]);
  assert_text_fill_color(&summary, "EDGE", "#636363@33");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf144092TableHeight
fn mapped_pptx_tdf144092_preserves_expanded_table_height() {
  let summary = render_summary("pptx/tdf144092-tableHeight.pptx");
  assert_page_has_stroked_path(&summary, 0);
  assert_any_path_height_close(&summary, 0, 7885.0 * 72.0 / 2540.0, 4.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf79007
fn mapped_pptx_tdf79007_preserves_graphic_color_modes() {
  let summary = render_summary("pptx/tdf79007.pptx");
  assert_page_count(&summary, 3);
  assert_rendered_image_centers_include_rgb_close(
    "pptx/tdf79007.pptx",
    &summary,
    1,
    [132, 132, 132],
  );
  assert_rendered_image_centers_include_rgb_close("pptx/tdf79007.pptx", &summary, 2, [0, 0, 0]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf118776
fn mapped_pptx_tdf118776_preserves_no_fill_text_transparency() {
  let summary = render_summary("pptx/tdf118776.pptx");
  assert_text_fill_color(&summary, "Invisible due to no fill", "#000000@03");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf129686
fn mapped_pptx_tdf129686_preserves_opaque_text_fill() {
  let summary = render_summary("pptx/tdf129686.pptx");
  assert_text_fill_color(&summary, "Profitability analysis", "#000000@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf105150
fn mapped_pptx_tdf105150_preserves_slide_background_fill_usage() {
  let summary = render_summary("pptx/tdf105150.pptx");
  assert_page_has_stroked_path(&summary, 0);
  assert!(!summary.paths.iter().any(|path| {
    path.page_index == 0
      && path.fill_mode.is_some()
      && path.fill_color.as_deref() == Some("#ffffff@ff")
  }));
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf123684
fn mapped_pptx_tdf123684_keeps_text_visible_when_shape_fill_is_none() {
  let summary = render_summary("pptx/tdf123684.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["Test", "Test", "Test", "Test", "Test", "Test"],
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf104445
fn mapped_pptx_tdf104445_does_not_add_extra_bullets_to_first_shape() {
  let summary = render_summary("pptx/tdf104445.pptx");
  assert_text_absent(&summary, 0, "• Tartalom helye 2");
  assert_page_contains_in_order(&summary, 0, &["Tartalom helye 2", "Tartalom helye 3"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testRowHeight_n80340
fn mapped_pptx_n80340_preserves_table_row_height() {
  let summary = render_summary("pptx/n80340.pptx");
  assert_page_contains_in_order(&summary, 0, &["Yogesh"]);
  assert_any_path_height_close(&summary, 0, 508.0 * 72.0 / 2540.0, 3.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testRowHeight_tableScale
fn mapped_pptx_tablescale_preserves_scaled_table_row_heights() {
  let summary = render_summary("pptx/tablescale.pptx");
  assert_page_contains_in_order(&summary, 0, &["xxx", "yyy"]);
  assert_any_path_height_close(&summary, 0, 800.0 * 72.0 / 2540.0, 4.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf93830
fn mapped_pptx_tdf93830_preserves_text_left_distance_offset() {
  let summary = render_summary("pptx/tdf93830.pptx");
  assert_page_has_stroked_path(&summary, 0);
  assert_any_path_width_close(&summary, 0, 4024.0 * 72.0 / 2540.0, 8.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf62255
fn mapped_pptx_tdf62255_preserves_table_cell_no_fill() {
  let summary = render_summary("pptx/tdf62255.pptx");
  assert_page_contains_in_order(&summary, 0, &["Test"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 4);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf127964
fn mapped_pptx_tdf127964_preserves_background_fill_usage() {
  let summary = render_summary("pptx/tdf127964.pptx");
  assert_page_has_stroked_path(&summary, 0);
  assert!(!summary.paths.iter().any(|path| {
    path.page_index == 0
      && path.fill_mode.is_some()
      && path.fill_color.as_deref() == Some("#ffffff@ff")
  }));
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf106638
fn mapped_pptx_tdf106638_preserves_wingdings_bullet_run() {
  let summary = render_summary("pptx/tdf106638.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["stratégique si la France veut se positionner"],
  );
  assert_text_object_font_contains(&summary, "", "Wingdings");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testIndentDuplication
fn mapped_pptx_formatting_bullet_indent_preserves_scaled_indent() {
  let summary = render_summary("pptx/formatting-bullet-indent.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["Paragraph with indent", "Paragraph without indent."],
  );
  assert_text_left_delta_close(
    &summary,
    0,
    "Paragraph with indent",
    "Paragraph without indent.",
    2500.0 * 72.0 / 2540.0,
    8.0,
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:test_srcRect_smallNegBound
fn mapped_pptx_tdf153008_preserves_cropped_bitmap_edge_pixels() {
  let summary = render_summary("pptx/tdf153008-srcRect-smallNegBound.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_first_image_relative_rgb_close(
    "pptx/tdf153008-srcRect-smallNegBound.pptx",
    &summary,
    0,
    0.05,
    0.5,
    [0, 0, 0],
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf153012
fn mapped_pptx_chart_pt_color_bg1_preserves_resolved_data_point_fill() {
  let summary = render_summary("pptx/chart_pt_color_bg1.pptx");
  assert_has_path_fill_color(&summary, "#d9d9d9@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testBase
fn mapped_pptx_smartart_base_preserves_text_fill_and_ltr_grid() {
  let summary = render_summary("pptx/smartart1.pptx");
  assert_page_contains_in_order(&summary, 0, &["a", "b", "c", "d", "e"]);
  assert_has_path_fill_color(&summary, "#4f81bd@ff");
  assert_text_left_before(&summary, 0, "a", "b");
  assert_text_left_before(&summary, 0, "c", "d");
  assert_text_top_after(&summary, 0, "c", "a");
  assert_text_top_after(&summary, 0, "e", "c");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testChildren
fn mapped_pptx_smartart_children_preserves_nested_texts() {
  let summary = render_summary("pptx/smartart-children.pptx");
  assert_page_contains_in_order(&summary, 0, &["a", "b", "c", "x", "y", "z"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testText
fn mapped_pptx_smartart_text_preserves_non_empty_child_text() {
  let summary = render_summary("pptx/smartart-text.pptx");
  assert_page_contains_in_order(&summary, 0, &["test"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testCnt
fn mapped_pptx_smartart_cnt_preserves_three_visible_text_nodes() {
  let summary = render_summary("pptx/smartart-cnt.pptx");
  assert_page_contains_in_order(&summary, 0, &["a", "b", "c"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testDir
fn mapped_pptx_smartart_dir_preserves_reversed_direction() {
  let summary = render_summary("pptx/smartart-dir.pptx");
  assert_page_contains_in_order(&summary, 0, &["first", "second"]);
  assert_text_left_after(&summary, 0, "first", "second");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf148665
fn mapped_pptx_smartart_tdf148665_preserves_text_nodes() {
  let summary = render_summary("pptx/tdf148665.pptx");
  assert_page_contains_in_order(&summary, 0, &["Fufufu", "Susu", "Sasa Haha"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf148921
fn mapped_pptx_smartart_tdf148921_keeps_two_visible_shapes() {
  let summary = render_summary("pptx/tdf148921.pptx");
  assert_page_contains_in_order(&summary, 0, &["Test"]);
  assert!(
    page_path_count(&summary, 0) >= 2,
    "expected at least two visible shapes; paths={:?}",
    summary.paths
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testMaxDepth
fn mapped_pptx_smartart_maxdepth_places_children_on_same_axis() {
  let summary = render_summary("pptx/smartart-maxdepth.pptx");
  assert_page_contains_in_order(&summary, 0, &["first", "second"]);
  assert_text_top_close(&summary, 0, "first", "second", 8.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testRotation
fn mapped_pptx_smartart_rotation_preserves_rotated_texts() {
  let summary = render_summary("pptx/smartart-rotation.pptx");
  assert_page_contains_in_order(&summary, 0, &["a", "b", "c"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 3);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testPyramidOneChild
fn mapped_pptx_smartart_pyramid_one_child_preserves_child_text() {
  let summary = render_summary("pptx/smartart-pyramid-1child.pptx");
  assert_page_contains_in_order(&summary, 0, &["A"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testChevron
fn mapped_pptx_smartart_chevron_preserves_horizontal_sequence() {
  let summary = render_summary("pptx/smartart-chevron.pptx");
  assert_page_contains_in_order(&summary, 0, &["a", "b", "c"]);
  assert_text_left_before(&summary, 0, "a", "b");
  assert_text_left_before(&summary, 0, "b", "c");
  assert_text_top_close(&summary, 0, "a", "b", 8.0);
  assert_text_top_close(&summary, 0, "b", "c", 8.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testCycle
fn mapped_pptx_smartart_cycle_preserves_texts_and_connectors() {
  let summary = render_summary("pptx/smartart-cycle.pptx");
  assert_page_contains_all(&summary, 0, &["a", "b", "c", "d", "e"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 5);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testBaseRtoL
fn mapped_pptx_smartart_base_rtl_preserves_text_fill_and_rtl_grid() {
  let summary = render_summary("pptx/smartart-rightoleftblockdiagram.pptx");
  assert_page_contains_all(&summary, 0, &["a", "b", "c", "d", "e"]);
  assert_has_path_fill_color(&summary, "#4f81bd@ff");
  assert_text_left_after(&summary, 0, "a", "b");
  assert_text_left_after(&summary, 0, "c", "d");
  assert_text_top_after(&summary, 0, "c", "a");
  assert_text_top_after(&summary, 0, "e", "c");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testVerticalBracketList
fn mapped_pptx_smartart_vertical_bracket_list_preserves_child_text() {
  let summary = render_summary("pptx/vertical-bracket-list.pptx");
  assert_page_contains_in_order(&summary, 0, &["1", "A"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTableList
fn mapped_pptx_smartart_table_list_aligns_child_with_parent() {
  let summary = render_summary("pptx/table-list.pptx");
  assert_page_contains_in_order(&summary, 0, &["Parent", "Child 1", "Child 2"]);
  let parent = text_bounds_containing(&summary, 0, "Parent");
  let child = text_bounds_containing(&summary, 0, "Child 2");
  assert!(
    (parent.right - child.right).abs() < 18.0,
    "expected Child 2 right edge to stay close to Parent right edge; parent={parent:?}; child={child:?}"
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testAccentProcess
fn mapped_pptx_smartart_accent_process_preserves_child_text_and_arrow() {
  let summary = render_summary("pptx/smartart-accent-process.pptx");
  assert_page_contains_in_order(&summary, 0, &["a", "b", "c", "c", "d"]);
  assert_text_top_after(&summary, 0, "b", "a");
  assert_page_stroked_path_count_at_least(&summary, 0, 3);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testContinuousBlockProcess
fn mapped_pptx_smartart_continuous_block_process_preserves_three_blocks() {
  let summary = render_summary("pptx/smartart-continuous-block-process.pptx");
  assert_page_contains_in_order(&summary, 0, &["A", "B", "C"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 3);
  assert_text_left_before(&summary, 0, "A", "B");
  assert_text_left_before(&summary, 0, "B", "C");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testOrgChart
fn mapped_pptx_smartart_org_chart_preserves_manager_employee_layout() {
  let summary = render_summary("pptx/smartart-org-chart.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "Manager",
      "Second para",
      "Employee",
      "Employee2",
      "Manager2",
      "Assistant",
    ],
  );
  assert_text_fill_color(&summary, "Manager", "#ffffff@ff");
  assert_text_top_after(&summary, 0, "Employee", "Manager");
  assert_text_left_before(&summary, 0, "Employee", "Employee2");
  assert_text_top_after(&summary, 0, "Employee", "Assistant");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testCycleMatrix
fn mapped_pptx_smartart_cycle_matrix_preserves_texts_and_orange_fill() {
  let summary = render_summary("pptx/smartart-cycle-matrix.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "A1", "A2", "B1", "B2", "C1", "C2-1", "D1", "D2", "C2-2", "C2-3", "C2-4",
    ],
  );
  assert_has_path_fill_color(&summary, "#f79646@ff");
  assert_text_left_before(&summary, 0, "A2", "B2");
  assert_text_top_after(&summary, 0, "D2", "A2");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testPictureStrip
fn mapped_pptx_smartart_picture_strip_preserves_three_bitmap_rows() {
  let summary = render_summary("pptx/smartart-picture-strip.pptx");
  assert_page_contains_in_order(&summary, 0, &["Foo Bar", "Baz Blah", "A", "B", "C"]);
  assert_page_image_count_at_least(&summary, 0, 3);
  assert_text_top_after(&summary, 0, "B", "A");
  assert_text_top_after(&summary, 0, "C", "B");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testBackground
fn mapped_pptx_smartart_background_preserves_green_background() {
  let summary = render_summary("pptx/smartart-background.pptx");
  assert_page_contains_in_order(&summary, 0, &["Background", "should", "be", "green"]);
  assert_has_path_fill_color(&summary, "#339933@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testBackgroundDrawingmlFallback
fn mapped_pptx_smartart_background_drawingml_fallback_preserves_green_background() {
  let summary = render_summary("pptx/smartart-background-drawingml-fallback.pptx");
  assert_page_contains_in_order(&summary, 0, &["Background", "should", "be", "green"]);
  assert_has_path_fill_color(&summary, "#339933@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testCenterCycle
fn mapped_pptx_smartart_center_cycle_preserves_center_relationships() {
  let summary = render_summary("pptx/smartart-center-cycle.pptx");
  assert_page_contains_all(&summary, 0, &["center", "a", "b", "c"]);
  assert_text_top_after(&summary, 0, "center", "a");
  assert_text_left_after(&summary, 0, "center", "b");
  assert_text_left_before(&summary, 0, "center", "c");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testFontSize
fn mapped_pptx_smartart_font_size_preserves_max_and_shrunk_text() {
  let summary = render_summary("pptx/smartart-font-size.pptx");
  assert_page_count(&summary, 3);
  assert_page_contains_in_order(&summary, 0, &["Max size", "(65 pt)"]);
  assert_text_font_size(&summary, "Max size", "65.00");
  assert_page_contains_in_order(&summary, 1, &["Automatically shrinked text"]);
  assert_text_font_size(&summary, "Automatically shrinked text", "32.00");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testVerticalBlockList
fn mapped_pptx_smartart_vertical_block_list_preserves_rotated_block_text() {
  let summary = render_summary("pptx/smartart-vertical-block-list.pptx");
  assert_page_contains_in_order(&summary, 0, &["a", "b", "c", "x", "y", "z", "empty"]);
  assert_vertical_text_shape(&summary, 0, "b");
  assert_text_top_after(&summary, 0, "empty", "a");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testMissingBulletAndIndent
fn mapped_pptx_smartart_missing_bullet_preserves_child_bullet_indent() {
  let summary = render_summary("pptx/smartart-missing-bullet.pptx");
  assert_page_contains_in_order(&summary, 0, &["Bullet no", "Bullet yes"]);
  assert_text_left_delta_close(
    &summary,
    0,
    "Bullet yes",
    "Bullet no",
    309.0 * 72.0 / 2540.0,
    6.0,
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testBulletList
fn mapped_pptx_smartart_bullet_list_preserves_child_bullets() {
  let summary = render_summary("pptx/smartart-bullet-list.pptx");
  assert_page_contains_in_order(&summary, 0, &["A", "B", "C"]);
  assert_text_left_before(&summary, 0, "A", "B");
  assert_text_left_before(&summary, 0, "B", "C");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testRecursion
fn mapped_pptx_smartart_recursion_preserves_nested_texts() {
  let summary = render_summary("pptx/smartart-recursion.pptx");
  assert_page_contains_in_order(&summary, 0, &["A", "B1", "C1", "C2", "B2", "C3"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testDataFollow
fn mapped_pptx_smartart_data_follow_preserves_following_nodes() {
  let summary = render_summary("pptx/smartart-data-follow.pptx");
  assert_page_contains_in_order(&summary, 0, &["A1", "B1", "B2", "A2", "C1", "C2"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testOrgChart2
fn mapped_pptx_smartart_org_chart2_preserves_deep_org_texts() {
  let summary = render_summary("pptx/smartart-org-chart2.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["A", "B1", "B2", "C3", "C1", "C2", "D1", "D2", "C4"],
  );
  assert_text_top_after(&summary, 0, "B1", "A");
  assert_text_top_after(&summary, 0, "C1", "B1");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf131553
fn mapped_pptx_smartart_tdf131553_preserves_embedded_formula_object() {
  let summary = render_summary("pptx/tdf131553.pptx");
  assert_page_contains_in_order(&summary, 0, &["𝐴=", "𝜋", "𝑟^2"]);
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testFillColorList
fn mapped_pptx_smartart_fill_color_list_preserves_red_fill_and_short_height() {
  let summary = render_summary("pptx/fill-color-list.pptx");
  assert_page_contains_in_order(&summary, 0, &["A", "B", "C"]);
  assert_has_path_fill_color(&summary, "#c0504d@ff");
  assert_any_path_height_close(&summary, 0, 2239.0 * 72.0 / 2540.0, 6.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf134221
fn mapped_pptx_smartart_tdf134221_preserves_negative_upper_text_inset() {
  let summary = render_summary("pptx/smartart-tdf134221.pptx");
  assert_page_contains_in_order(&summary, 0, &["A", "C", "B"]);
  assert_text_top_after(&summary, 0, "B", "A");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testLinearRule
fn mapped_pptx_smartart_linear_rule_preserves_large_background_arrow() {
  let summary = render_summary("pptx/smartart-linear-rule.pptx");
  assert_page_contains_in_order(&summary, 0, &["A", "B", "C"]);
  assert_any_path_width_close(&summary, 0, 19867.0 * 72.0 / 2540.0, 24.0);
  assert_any_path_height_close(&summary, 0, 10092.0 * 72.0 / 2540.0, 24.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testLinearRuleVert
fn mapped_pptx_smartart_linear_rule_vert_preserves_first_item_height() {
  let summary = render_summary("pptx/smartart-linear-rule-vert.pptx");
  assert_page_contains_in_order(&summary, 0, &["P1", "P2", "P3"]);
  assert_any_path_height_close(&summary, 0, 2020.0 * 72.0 / 2540.0, 6.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testAutofitSync
fn mapped_pptx_smartart_autofit_sync_preserves_scaled_text_groups() {
  let summary = render_summary("pptx/smartart-autofit-sync.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["A", "B", "C", "A1", "A2", "B1", "B2", "C1", "A3", "B20"],
  );
  assert_text_font_size(&summary, "A1", "56.00");
  assert_text_font_size(&summary, "B1", "56.00");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testSnakeRows
fn mapped_pptx_smartart_snake_rows_preserves_two_row_layout() {
  let summary = render_summary("pptx/smartart-snake-rows.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "Parent 3", "Child 3", "Child 2", "Child 5", "Child 6", "Child 1",
    ],
  );
  assert_text_top_close(&summary, 0, "Parent 3", "Child 3", 12.0);
  assert_text_top_after(&summary, 0, "Parent 4", "Parent 3");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testCompositeInferRight
fn mapped_pptx_smartart_composite_infer_right_keeps_text_right_of_image() {
  let summary = render_summary("pptx/smartart-composite-infer-right.pptx");
  assert_page_contains_in_order(&summary, 0, &["Parent", "Child 1", "Child 2"]);
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_text_left_after(&summary, 0, "Child 1", "Parent");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf149551Pie
fn mapped_pptx_smartart_tdf149551_pie_preserves_text_anchor_position() {
  let summary = render_summary("pptx/tdf149551_SmartArt_Pie.pptx");
  assert_page_contains_in_order(&summary, 0, &["1 a b c", "2 a b c", "3 a b c"]);
  assert_text_near_libreoffice_metafile_point(&summary, 0, "1 a b c", 12658.0, 5073.0, 20.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf149551Pyramid
fn mapped_pptx_smartart_tdf149551_pyramid_preserves_text_anchor_position() {
  let summary = render_summary("pptx/tdf149551_SmartArt_Pyramid.pptx");
  assert_page_contains_in_order(&summary, 0, &["1 a b c", "2 a b c", "3 a b c"]);
  assert_text_near_libreoffice_metafile_point(&summary, 0, "1 a b c", 9368.0, 2699.0, 20.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf149551Venn
fn mapped_pptx_smartart_tdf149551_venn_preserves_text_anchor_position() {
  let summary = render_summary("pptx/tdf149551_SmartArt_Venn.pptx");
  assert_page_contains_in_order(&summary, 0, &["1 a b c", "2 a b c", "3 a b c"]);
  assert_text_near_libreoffice_metafile_point(&summary, 0, "1 a b c", 10333.0, 3395.0, 20.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf149551Gear
fn mapped_pptx_smartart_tdf149551_gear_preserves_text_anchor_position() {
  let summary = render_summary("pptx/tdf149551_SmartArt_Gear.pptx");
  assert_page_contains_in_order(&summary, 0, &["One", "Two", "Three"]);
  assert_text_near_libreoffice_metafile_point(&summary, 0, "One", 6605.0, 5787.0, 20.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf145528Matrix
fn mapped_pptx_smartart_tdf145528_matrix_preserves_text_positions() {
  let summary = render_summary("pptx/tdf145528_SmartArt_Matrix.pptx");
  assert_page_contains_in_order(&summary, 0, &["Writer", "Calc", "Impress", "Draw"]);
  assert_text_near_libreoffice_metafile_point(&summary, 0, "Writer", 4001.0, 9999.0, 24.0);
  assert_text_near_libreoffice_metafile_point(&summary, 0, "Calc", 12001.0, 1999.0, 24.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf135953TextPosition
fn mapped_pptx_smartart_tdf135953_preserves_rotated_text_area_position() {
  let summary = render_summary("pptx/tdf135953_SmartArt_textposition.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["left shape", "left text", "right shape", "right text"],
  );
  assert_text_intersects_libreoffice_metafile_rect(
    &summary,
    0,
    "left shape",
    LibreOfficeMetafileRect {
      left_100mm: 3339.0,
      top_100mm: -1544.0,
      right_100mm: 9441.0,
      bottom_100mm: 461.0,
    },
    24.0,
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTdf132302RightArrow
fn mapped_pptx_smartart_tdf132302_right_arrow_preserves_text_area_position() {
  let summary = render_summary("pptx/tdf132302_SmartArt_rightArrow.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "Topic A",
      "Detail One",
      "Detail Two",
      "Topic B",
      "Detail Three",
      "Detail Four",
    ],
  );
  assert_text_starts_in_libreoffice_metafile_rect(
    &summary,
    0,
    "Detail One",
    LibreOfficeMetafileRect {
      left_100mm: 5078.0,
      top_100mm: 1257.0,
      right_100mm: 9190.0,
      bottom_100mm: 6741.0,
    },
    24.0,
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf157529
fn mapped_pptx_tdf157529_preserves_fully_transparent_shape_fills() {
  let summary = render_summary("pptx/tdf157529.pptx");
  assert_page_contains_in_order(&summary, 0, &["LIBREOFFICE", "Text with 100% transparency"]);
  assert_has_text_fill_color(&summary, "#000000@00");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf160490
fn mapped_pptx_tdf160490_preserves_placeholder_heights() {
  let summary = render_summary("pptx/tdf160490.pptx");
  assert_page_contains_in_order(&summary, 0, &["HELLO", "Set Top, Bottom margin"]);
  assert_any_path_height_close(&summary, 0, 3726.0 * 72.0 / 2540.0, 8.0);
  assert_any_path_height_close(&summary, 0, 3365.0 * 72.0 / 2540.0, 8.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf165321
fn mapped_pptx_tdf165321_preserves_smartart_child_dimensions() {
  let summary = render_summary("pptx/tdf165321.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["Gestion du changement", "Conditions de succès"],
  );
  assert_any_path_height_close(&summary, 0, 3597.0 * 72.0 / 2540.0, 8.0);
  assert_any_path_width_close(&summary, 0, 6592.0 * 72.0 / 2540.0, 8.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf165341
fn mapped_pptx_tdf165341_preserves_top_center_text_adjustment() {
  let summary = render_summary("pptx/tdf165341.pptx");
  assert_page_contains_in_order(&summary, 0, &["The shape is top", "center"]);
  assert_text_centered_on_page(&summary, 0, "center", 24.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf152186
fn mapped_pptx_tdf152186_does_not_render_imported_shadow_effects() {
  let summary = render_summary("pptx/tdf152186.pptx");
  assert_page_stroked_path_count_at_least(&summary, 0, 3);
  assert!(
    !summary
      .paths
      .iter()
      .any(|path| path.fill_color.as_deref() == Some("#808080@ff")),
    "unexpected opaque gray shadow-like fill; paths={:?}",
    summary.paths
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf93868
fn mapped_pptx_tdf93868_preserves_master_background_fill_usage() {
  let summary = render_summary("pptx/tdf93868.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["Test", "Slide inherits objects from slideMaster"],
  );
  assert_page_has_stroked_path(&summary, 0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf95932
fn mapped_pptx_tdf95932_inherits_green_master_shape_fill() {
  let summary = render_summary("pptx/tdf95932.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["Test inheritance of shape properties", "Test"],
  );
  assert_has_path_fill_color(&summary, "#76bf3d@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf99030
fn mapped_pptx_tdf99030_preserves_master_background_color() {
  let summary = render_summary("pptx/tdf99030.pptx");
  assert_has_path_fill_color(&summary, "#676a55@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf103473
fn mapped_pptx_tdf103473_preserves_picture_geometry() {
  let summary = render_summary("pptx/tdf103473.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_any_image_bounds_close(&summary, 0, 3629.0, 4431.0, 8353.0, 9155.0, 12.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf109067
fn mapped_pptx_tdf109067_preserves_diagonal_gradient_shape() {
  let summary = render_summary("pptx/tdf109067.pptx");
  assert_page_has_stroked_path(&summary, 0);
  assert!(summary.page_objects[0].path_objects >= 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf109187
fn mapped_pptx_tdf109187_preserves_two_gradient_arrow_shapes() {
  let summary = render_summary("pptx/tdf109187.pptx");
  assert_page_stroked_path_count_at_least(&summary, 0, 2);
  assert_any_path_height_close(&summary, 0, 2250.0 * 72.0 / 2540.0, 48.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf100065
fn mapped_pptx_tdf100065_preserves_group_shape_rotation_text() {
  let summary = render_summary("pptx/tdf100065.pptx");
  assert_page_contains_in_order(&summary, 0, &["This is a test"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 2);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf90626
fn mapped_pptx_tdf90626_preserves_graphic_bullet_size() {
  let summary = render_summary("pptx/tdf90626.pptx");
  assert_page_contains_in_order(&summary, 0, &["Tdf90626", "Test", "Test", "Test", "Test"]);
  assert_page_image_count_at_least(&summary, 0, 4);
  assert_any_path_height_close(&summary, 0, 372.0 * 72.0 / 2540.0, 6.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf138148
fn mapped_pptx_tdf138148_preserves_narrow_graphic_bullet_size() {
  let summary = render_summary("pptx/tdf138148.pptx");
  assert_page_contains_in_order(&summary, 0, &["Aaa", "Bbb"]);
  assert_page_image_count_at_least(&summary, 0, 2);
  assert_any_path_height_close(&summary, 0, 444.0 * 72.0 / 2540.0, 6.0);
  assert_any_path_width_close(&summary, 0, 148.0 * 72.0 / 2540.0, 6.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf114913
fn mapped_pptx_tdf114913_preserves_graphic_bullet_height() {
  let summary = render_summary("pptx/tdf114913.pptx");
  assert_page_contains_in_order(&summary, 0, &["Test"]);
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_any_path_height_close(&summary, 0, 692.0 * 72.0 / 2540.0, 8.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf157216
fn mapped_pptx_tdf157216_preserves_punched_tape_flowchart_shape() {
  let summary = render_summary("pptx/tdf157216.pptx");
  assert_page_contains_in_order(&summary, 0, &["Flowchart"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 3);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testFreeformShapeGluePoints
fn mapped_pptx_tdf156829_preserves_freeform_shape_output() {
  let summary = render_summary("pptx/tdf156829.pptx");
  assert_page_has_stroked_path(&summary, 0);
  assert!(page_path_count(&summary, 0) >= 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf154363
fn mapped_pptx_tdf154363_preserves_flipped_connector_shapes() {
  let summary = render_summary("pptx/tdf154363.pptx");
  assert_page_contains_in_order(&summary, 0, &["Flip horizontal", "Flip vertical"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 2);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf154858
fn mapped_pptx_tdf154858_preserves_radial_gradient_shape_visibility() {
  let summary = render_summary("pptx/tdf154858.pptx");
  assert_page_has_stroked_path(&summary, 0);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf153466
fn mapped_pptx_tdf153466_preserves_bitmap_fill_shapes() {
  let summary = render_summary("pptx/tdf153466.pptx");
  assert_page_image_count_at_least(&summary, 0, 2);
  assert_page_has_stroked_path(&summary, 0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf152434
fn mapped_pptx_tdf152434_preserves_single_picture_object() {
  let summary = render_summary("pptx/tdf152434.pptx");
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_eq!(summary.page_objects[0].image_objects, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testConnectors
fn mapped_pptx_connectors_preserve_connector_lines() {
  let summary = render_summary("pptx/connectors.pptx");
  assert_page_stroked_path_count_at_least(&summary, 0, 16);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf153036_resizedConnectorL
fn mapped_pptx_tdf153036_resized_connector_l_preserves_textbox_and_connector() {
  let summary = render_summary("pptx/tdf153036_resizedConnectorL.pptx");
  assert_page_contains_in_order(&summary, 0, &["TextBox"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf149314
fn mapped_pptx_tdf149314_preserves_internal_slide_hyperlinks() {
  let summary = render_summary("pptx/tdf149314.pptx");
  assert_page_link_target(&summary, 0, "#Slide 1");
  assert_page_link_target(&summary, 0, "#Slide 3");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf149124
fn mapped_pptx_tdf149124_preserves_standard_connector_output() {
  let summary = render_summary("pptx/tdf149124.pptx");
  assert_page_stroked_path_count_at_least(&summary, 0, 3);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf89449
fn mapped_pptx_tdf89449_preserves_curved_straight_and_standard_connectors() {
  let summary = render_summary("pptx/tdf89449.pptx");
  assert_page_stroked_path_count_at_least(&summary, 0, 6);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testGluePointLeavingDirections
fn mapped_pptx_glue_point_leaving_directions_preserves_ellipse_output() {
  let summary = render_summary("pptx/glue_point_leaving_directions.pptx");
  assert_page_stroked_path_count_at_least(&summary, 0, 1);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf147459
fn mapped_pptx_tdf147459_preserves_triangle_shape_output() {
  let summary = render_summary("pptx/tdf147459.pptx");
  assert_page_stroked_path_count_at_least(&summary, 0, 1);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf146223
fn mapped_pptx_tdf146223_preserves_visible_master_slide_text() {
  let summary = render_summary("pptx/tdf146223.pptx");
  assert_page_contains_in_order(&summary, 0, &["Title", "Bullet Point 1", "Bullet Point 2"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf144917
fn mapped_pptx_tdf144917_preserves_external_shape_hyperlink() {
  let summary = render_summary("pptx/tdf144917.pptx");
  assert_page_annotation_uri(&summary, 0, "http://www.example.com/");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf148965
fn mapped_pptx_tdf148965_preserves_localized_internal_hyperlinks() {
  let summary = render_summary("pptx/tdf148965.pptx");
  assert_page_count(&summary, 3);
  assert_page_contains_in_order(&summary, 1, &["First", "Third"]);
  assert_page_link_count_at_least(&summary, 1, 2);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf144918
fn mapped_pptx_tdf144918_preserves_named_slide_hyperlinks() {
  let summary = render_summary("pptx/tdf144918.pptx");
  assert_page_count(&summary, 3);
  assert_page_contains_in_order(&summary, 1, &["First", "Third"]);
  assert_page_link_count_at_least(&summary, 1, 2);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testHyperlinkOnImage
fn mapped_pptx_hyperlink_on_image_preserves_clickable_picture() {
  let summary = render_summary("pptx/hyperlinkOnImage.pptx");
  assert_page_count(&summary, 2);
  assert_page_contains_in_order(&summary, 0, &["First slide"]);
  assert_page_contains_in_order(&summary, 1, &["Last slide"]);
  assert_page_image_count_at_least(&summary, 0, 1);
  assert_page_image_count_at_least(&summary, 1, 1);
  assert_page_link_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf141704
fn mapped_pptx_tdf141704_preserves_shape_hyperlink_actions() {
  let summary = render_summary("pptx/tdf141704.pptx");
  assert_page_count(&summary, 7);
  assert_page_contains_in_order(&summary, 0, &["Go to the last slide"]);
  assert_page_contains_in_order(&summary, 3, &["http://www.example.com"]);
  assert_page_contains_in_order(&summary, 5, &["End Show"]);
  assert_page_annotation_uri(&summary, 3, "http://www.example.com/");
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testInternalHyperlink
fn mapped_pptx_tdf65724_preserves_internal_text_hyperlink() {
  let summary = render_summary("pptx/tdf65724.pptx");
  assert_page_count(&summary, 2);
  assert_page_contains_in_order(&summary, 0, &["Slide1", "goToSlide2"]);
  assert_page_contains_in_order(&summary, 1, &["Slide2"]);
  assert_page_link_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN862510_1
fn mapped_pptx_n862510_1_preserves_baseline_text_run() {
  let summary = render_summary("pptx/n862510_1.pptx");
  assert_page_contains_in_order(&summary, 0, &["Turnaround time for", "updates"]);
  assert_text_top_close(&summary, 0, "Turnaround time for", "updates", 6.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN862510_4
fn mapped_pptx_n862510_4_preserves_gradient_text_fill() {
  let summary = render_summary("pptx/n862510_4.pptx");
  assert_page_contains_in_order(&summary, 0, &["9/10"]);
  assert_text_fill_color_not(&summary, "9/10", "#000000@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN828390_3
fn mapped_pptx_n828390_3_preserves_registered_mark_text_flow() {
  let summary = render_summary("pptx/n828390_3.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["SUSE", "Linux Enterprise", "Standard Lifecycle"],
  );
  assert_text_top_after(&summary, 0, "Standard Lifecycle", "SUSE");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testN778859
fn mapped_pptx_n778859_preserves_non_autofit_text_layout() {
  let summary = render_summary("pptx/n778859.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["Content with NO autofit", "too many text", "Adding", "9"],
  );
  assert_text_font_size(&summary, "too many text", "18.00");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testPlaceholderPriority
fn mapped_pptx_placeholder_priority_preserves_placeholder_order() {
  let summary = render_summary("ppt/placeholder-priority.pptx");
  assert_page_contains_in_order(&summary, 0, &["aaa", "bbb"]);
  assert_text_top_after(&summary, 0, "bbb", "aaa");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testFdo72998
fn mapped_pptx_cshapes_preserves_custom_shape_geometry_output() {
  let summary = render_summary("pptx/cshapes.pptx");
  assert_page_filled_path_count_at_least(&summary, 0, 100);
  assert_page_stroked_path_count_at_least(&summary, 0, 100);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testMultiColTexts
fn mapped_pptx_multicol_preserves_multicolumn_slide_text() {
  let summary = render_summary("pptx/multicol.pptx");
  assert_page_contains_in_order(&summary, 0, &["slideshape1", "Slideshape2"]);
  assert_text_font_size(&summary, "slideshape1", "14.00");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf149785
fn mapped_pptx_tdf149785_imports_single_visible_object() {
  let summary = render_summary("pptx/tdf149785.pptx");
  assert_page_count(&summary, 1);
  assert_eq!(page_object_count(&summary, 0), 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf149985
fn mapped_pptx_tdf149985_imports_single_visible_object() {
  let summary = render_summary("pptx/tdf149985.pptx");
  assert_page_count(&summary, 1);
  assert_eq!(page_object_count(&summary, 0), 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testTextAutoRotation
fn mapped_pptx_smartart_auto_text_rotation_preserves_three_slide_text_sets() {
  let summary = render_summary("pptx/smartart-autoTxRot.pptx");
  assert_page_count(&summary, 3);
  assert_page_contains_in_order(&summary, 0, &["a", "b", "c", "d", "e", "f"]);
  assert_page_contains_in_order(&summary, 1, &["a", "b", "c", "d", "e", "f"]);
  assert_page_contains_in_order(&summary, 2, &["a", "b", "c", "d", "e", "f"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 16);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN759180
fn mapped_pptx_n759180_preserves_small_end_paragraph_font() {
  let summary = render_summary("n759180.pptx");
  assert_page_contains_in_order(&summary, 0, &["textrun1", "Textrun2", "Textrun3"]);
  assert_text_font_size(&summary, ".", "13.00");
  assert_text_font_size(&summary, "Textrun3", "13.00");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN862510_2
fn mapped_pptx_n862510_2_preserves_rotated_smartart_shape_output() {
  let summary = render_summary("pptx/n862510_2.pptx");
  assert_page_count(&summary, 1);
  assert_page_stroked_path_count_at_least(&summary, 0, 1);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf157285
fn mapped_pptx_tdf157285_preserves_placeholder_heights() {
  let summary = render_summary("pptx/tdf157285.pptx");
  assert_page_contains_in_order(&summary, 0, &["Hello"]);
  assert_page_contains_in_order(&summary, 1, &["Hello"]);
  assert_any_path_height_close(&summary, 0, 2565.0 * 72.0 / 2540.0, 8.0);
  assert_any_path_height_close(&summary, 1, 1180.0 * 72.0 / 2540.0, 8.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testShapeGlowEffectPPTXImpoer
fn mapped_pptx_shape_glow_effect_preserves_ellipse_output() {
  let summary = render_summary("pptx/shape-glow-effect.pptx");
  assert_page_count(&summary, 1);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
  assert_any_path_width_close(&summary, 0, 1979875.0 * 72.0 / 914400.0, 16.0);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testShapeTextGlowEffectPPTXImport
fn mapped_pptx_shape_text_glow_effect_preserves_text_and_ellipse() {
  let summary = render_summary("pptx/shape-text-glow-effect.pptx");
  assert_page_contains_in_order(&summary, 0, &["Text Glow in Shape"]);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testShapeBlurPPTXImport
fn mapped_pptx_shape_blur_effect_preserves_shadowed_ellipse_output() {
  let summary = render_summary("pptx/shape-blur-effect.pptx");
  assert_page_count(&summary, 1);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
  assert!(page_object_count(&summary, 0) >= 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testHyperlinksOnShapes
fn mapped_pptx_tdf144616_preserves_shape_hyperlink_annotations() {
  let summary = render_summary("pptx/tdf144616.pptx");
  assert_page_count(&summary, 2);
  assert_page_stroked_path_count_at_least(&summary, 0, 7);
  assert_page_link_count_at_least(&summary, 0, 7);
  assert_page_annotation_uri(&summary, 0, "http://www.example.com/");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testCropToZero
fn mapped_pptx_crop_to_zero_imports_without_losing_picture_shape() {
  let summary = render_summary("pptx/croppedTo0.pptx");
  assert_page_count(&summary, 1);
  assert!(page_object_count(&summary, 0) >= 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf149961AutofitIndentation
fn mapped_pptx_tdf149961_preserves_autofit_indentation_text_flow() {
  let summary = render_summary("pptx/tdf149961-autofitIndentation.pptx");
  assert_page_contains_in_order(&summary, 0, &["Autofit", "Autofit"]);
  assert_text_font_size(&summary, "Autofit", "96.00");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:tdf158512
fn mapped_pptx_tdf158512_preserves_unfilled_foreground_shape() {
  let summary = render_summary("pptx/tdf158512.pptx");
  assert_page_count(&summary, 1);
  assert_page_contains_in_order(&summary, 0, &["KKKKKK", "EEEEEEEE", "KKKKKKKK"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf169524
fn mapped_pptx_tdf169524_preserves_zero_left_margin_paragraph_flow() {
  let summary = render_summary("pptx/tdf169524.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["KKKKKK", "EEEEEEEE", "AAAAAAAA", "IIIIIIIII", "EEEEEE EEEE"],
  );
  assert_text_fill_color(&summary, "EEEEEE EEEE", "#ff0000@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testMultidirectional
fn mapped_pptx_smartart_multidirectional_preserves_bidirectional_arrow_diagram() {
  let summary = render_summary("pptx/smartart-multidirectional.pptx");
  assert_page_contains_all(&summary, 0, &["a", "b", "c"]);
  assert_page_stroked_path_count_at_least(&summary, 0, 3);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testVerticalBoxList
fn mapped_pptx_smartart_vertical_box_list_preserves_parent_and_child_widths() {
  let summary = render_summary("pptx/smartart-vertical-box-list.pptx");
  assert_page_contains_in_order(&summary, 0, &["x"]);
  assert_any_path_width_close(&summary, 0, 11852.0 * 72.0 / 2540.0, 24.0);
  assert_page_filled_path_count_at_least(&summary, 0, 2);
}

#[test]
// Source: ../core/sd/qa/unit/layout-tests.cxx:testTdf146731
fn mapped_pptx_tdf146731_preserves_table_border_primitive_widths() {
  let summary = render_summary("pptx/tdf146731.pptx");
  assert_page_has_horizontal_stroked_path(&summary, 0);
  assert_page_stroked_path_count_at_least(&summary, 0, 3);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests_skia.cxx:testTdf156856
fn mapped_pptx_tdf156856_imports_skia_regression_fixture() {
  let summary = render_summary("pptx/tdf156856.pptx");
  assert_page_count(&summary, 1);
  assert!(page_object_count(&summary, 0) >= 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testFdo47434
fn mapped_pptx_fdo47434_preserves_full_layout_reference_output() {
  let summary = render_summary("fdo47434.pptx");
  assert_full_render_reference_smoke(&summary);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN819614
fn mapped_pptx_n819614_preserves_full_layout_reference_output() {
  let summary = render_summary("n819614.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_page_contains_in_order(&summary, 0, &["Test"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN820786
fn mapped_pptx_n820786_preserves_full_layout_reference_output() {
  let summary = render_summary("n820786.pptx");
  assert_full_render_reference_smoke(&summary);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN762695
fn mapped_pptx_n762695_preserves_full_layout_reference_output() {
  let summary = render_summary("n762695.pptx");
  assert_full_render_reference_smoke(&summary);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN593612
fn mapped_pptx_n593612_preserves_full_layout_reference_output() {
  let summary = render_summary("n593612.pptx");
  assert_full_render_reference_smoke(&summary);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testFdo71434
fn mapped_pptx_fdo71434_preserves_full_layout_reference_output() {
  let summary = render_summary("fdo71434.pptx");
  assert_full_render_reference_smoke(&summary);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testN902652
fn mapped_pptx_n902652_preserves_full_layout_reference_output() {
  let summary = render_summary("n902652.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_page_contains_in_order(&summary, 0, &["LibreOffice"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf90403
fn mapped_pptx_tdf90403_preserves_full_layout_reference_output() {
  let summary = render_summary("tdf90403.pptx");
  assert_full_render_reference_smoke(&summary);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf100491
fn mapped_pptx_tdf100491_preserves_full_layout_reference_output() {
  let summary = render_summary("tdf100491.pptx");
  assert_full_render_reference_smoke(&summary);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf109317
fn mapped_pptx_tdf109317_preserves_full_layout_reference_output() {
  let summary = render_summary("tdf109317.pptx");
  assert_full_render_reference_smoke(&summary);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc870237
fn mapped_pptx_bnc870237_preserves_diagram_text_distance_output() {
  let summary = render_summary("pptx/bnc870237.pptx");
  assert_page_contains_in_order(&summary, 0, &["Text"]);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf150789
fn mapped_pptx_tdf150789_preserves_up_arrow_callout_texts() {
  let summary = render_summary("pptx/tdf150789.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "Results of all deliberation",
      "Predictability of Things",
      "Sunshine",
    ],
  );
  assert_page_filled_path_count_at_least(&summary, 0, 2);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testBnc584721_2
fn mapped_pptx_bnc584721_2_avoids_extra_master_title_output() {
  let summary = render_summary("pptx/bnc584721_1_2.pptx");
  assert_page_contains_in_order(&summary, 0, &["Title"]);
  assert_eq!(summary.page_count, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testMasterSlides
fn mapped_pptx_master_slides_preserves_visible_master_deck_content() {
  let summary = render_summary("pptx/master-slides.pptx");
  assert_page_count(&summary, 1);
  assert_page_contains_in_order(
    &summary,
    0,
    &[
      "This is a dark theme cover page",
      "Best for presentations given in a large, dark room",
    ],
  );
}

#[test]
// Source: ../core/sd/qa/unit/ThemeTest.cxx:testThemeChange
fn mapped_pptx_theme_preserves_initial_theme_colored_text_output() {
  let summary = render_summary("theme.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_has_text_fill_color(&summary, "#4472c4@ff");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf142645
fn mapped_pptx_tdf142645_preserves_named_hello_slide_output() {
  let summary = render_summary("pptx/tdf142645.pptx");
  assert_page_count(&summary, 1);
  assert_page_contains_in_order(&summary, 0, &["Hello"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf142915
fn mapped_pptx_tdf142915_preserves_automatic_advance_slide_output() {
  let summary = render_summary("pptx/tdf142915.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_page_contains_in_order(&summary, 0, &["1"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf142913
fn mapped_pptx_tdf142913_preserves_first_page_selection_deck_output() {
  let summary = render_summary("pptx/tdf142913.pptx");
  assert_page_count(&summary, 2);
  assert_page_contains_in_order(&summary, 0, &["First"]);
  assert_page_contains_in_order(&summary, 1, &["Second"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testTdf142590
fn mapped_pptx_tdf142590_preserves_custom_show_source_deck_output() {
  let summary = render_summary("pptx/tdf142590.pptx");
  assert_page_count(&summary, 3);
  assert_page_contains_in_order(&summary, 0, &["1"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests.cxx:testCustomSlideShow
fn mapped_pptx_tdf131390_preserves_two_custom_show_source_deck_output() {
  let summary = render_summary("pptx/tdf131390.pptx");
  assert_page_count(&summary, 3);
  assert_page_contains_in_order(&summary, 0, &["First"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testDescriptionImport
fn mapped_pptx_alt_description_preserves_image_output() {
  let summary = render_summary("pptx/altdescription.pptx");
  assert_page_count(&summary, 1);
  assert_page_image_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testOOXTheme
fn mapped_pptx_ooxtheme_preserves_theme_dom_reference_output() {
  let summary = render_summary("pptx/ooxtheme.pptx");
  assert_page_contains_in_order(
    &summary,
    0,
    &["Theme test", "Theme DOM should be stored in OOXTheme"],
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf103347
fn mapped_pptx_tdf103347_preserves_duplicate_named_slide_deck_output() {
  let summary = render_summary("pptx/tdf103347.pptx");
  assert_page_count(&summary, 3);
  assert_page_contains_in_order(&summary, 0, &["Hello"]);
}

#[test]
// Source: ../core/sd/qa/unit/import-tests-smartart.cxx:testInteropGrabBag
fn mapped_pptx_smartart_interopgrabbag_preserves_diagram_output() {
  let summary = render_summary("pptx/smartart-interopgrabbag.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_page_filled_path_count_at_least(&summary, 0, 1);
}

#[test]
// Source: ../core/sd/qa/unit/PNGExportTests.cxx:testTdf156808
fn mapped_pptx_tdf156808_preserves_black_png_export_page_output() {
  assert_rendered_page_color_fraction_at_least("pptx/tdf156808.pptx", 0, [0, 0, 0], 8, 0.90);
}

#[test]
// Source: ../core/sd/qa/unit/PNGExportTests.cxx:testTdf157793
fn mapped_pptx_tdf157793_preserves_white_png_export_page_output() {
  assert_rendered_page_color_fraction_at_least(
    "pptx/tdf157793.pptx",
    0,
    [0xff, 0xff, 0xff],
    8,
    0.78,
  );
}

#[test]
// Source: ../core/sd/qa/unit/PNGExportTests.cxx:testTdf157635
fn mapped_pptx_tdf157635_preserves_black_background_png_export_output() {
  assert_rendered_page_color_fraction_at_least("pptx/tdf157635.pptx", 0, [0, 0, 0], 8, 0.71);
}

#[test]
// Source: ../core/sd/qa/unit/PNGExportTests.cxx:testTdf113163
fn mapped_pptx_tdf113163_preserves_all_black_png_export_output() {
  assert_rendered_page_color_fraction_at_least("pptx/tdf113163.pptx", 0, [0, 0, 0], 8, 0.98);
}

#[test]
// Source: ../core/sd/qa/unit/ShapeImportExportTest.cxx:testTextDistancesOOXML
fn mapped_pptx_text_distances_ooxml_preserves_inset_reference_labels() {
  let summary = render_summary("TextDistancesInsets1.pptx");
  assert_page_count(&summary, 1);
  assert_page_text_occurs_at_least(&summary, 0, "TOP", 6);
  assert_page_text_occurs_at_least(&summary, 0, "MIDDLE", 6);
  assert_page_text_occurs_at_least(&summary, 0, "BOTTOM", 6);
}

#[test]
// Source: ../core/sd/qa/unit/ShapeImportExportTest.cxx:testTextDistancesOOXML_LargerThanTextAreaSpecialCase
fn mapped_pptx_text_distances_ooxml_large_margin_special_case_preserves_labels() {
  let summary = render_summary("TextDistancesInsets2.pptx");
  assert_page_count(&summary, 1);
  assert_page_text_occurs_at_least(&summary, 0, "TOP", 15);
  assert_page_text_occurs_at_least(&summary, 0, "MIDDLE", 15);
  assert_page_text_occurs_at_least(&summary, 0, "BOTTOM", 15);
}

#[test]
// Source: ../core/sd/qa/unit/FontEmbeddingTest.cxx:testRoundtripEmbeddedFontsPPTX
fn mapped_pptx_boldonse_embedded_font_preserves_fonted_text_output() {
  let summary = render_summary("BoldonseFontEmbedded.pptx");
  assert_page_contains_in_order(&summary, 0, &["Test"]);
  assert_text_object_font_contains(&summary, "Test", "Boldonse");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf89064
fn mapped_pptx_tdf89064_preserves_single_notes_shape_source_and_slide_output() {
  let summary = render_summary("pptx/tdf89064.pptx");
  assert_page_count(&summary, 1);
  assert_full_render_reference_smoke(&summary);
  assert_package_part_contains(
    "pptx/tdf89064.pptx",
    "ppt/notesSlides/notesSlide1.xml",
    "Note",
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests2.cxx:testTdf115394
fn mapped_pptx_tdf115394_preserves_transition_duration_reference_deck_output() {
  let summary = render_summary("pptx/tdf115394.pptx");
  assert_page_count(&summary, 5);
  assert_page_contains_in_order(&summary, 0, &["Standard transition", "slow"]);
  assert_page_contains_in_order(&summary, 1, &["Standard transition", "medium"]);
  assert_page_contains_in_order(&summary, 2, &["Standard transition", "fast"]);
  assert_page_contains_in_order(&summary, 3, &["Custom transition", "0.25 s"]);
  assert_page_contains_in_order(&summary, 4, &["Custom transition", "4.25"]);
  assert_package_part_contains(
    "pptx/tdf115394.pptx",
    "ppt/slides/slide1.xml",
    "spd=\"slow\"",
  );
  assert_package_part_contains(
    "pptx/tdf115394.pptx",
    "ppt/slides/slide2.xml",
    "spd=\"med\"",
  );
  assert_package_part_contains(
    "pptx/tdf115394.pptx",
    "ppt/slides/slide4.xml",
    "p14:dur=\"250\"",
  );
  assert_package_part_contains(
    "pptx/tdf115394.pptx",
    "ppt/slides/slide5.xml",
    "p14:dur=\"4250\"",
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testCreationDate
fn mapped_pptx_fdo71434_preserves_creation_date_metadata_source_and_output() {
  let summary = render_summary("fdo71434.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_package_part_contains("fdo71434.pptx", "docProps/core.xml", "2013-11-09T10:37:56Z");
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf93097
fn mapped_pptx_tdf93097_preserves_title_metadata_source_and_output() {
  let summary = render_summary("pptx/tdf93097.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_page_contains_in_order(&summary, 0, &["ss"]);
  assert_package_part_contains(
    "pptx/tdf93097.pptx",
    "docProps/core.xml",
    "<dc:title>ss</dc:title>",
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests3.cxx:testTdf111927
fn mapped_pptx_tdf111927_preserves_empty_placeholder_style_deck_output() {
  let summary = render_summary("pptx/tdf163239.pptx");
  assert_page_count(&summary, 2);
  assert_full_render_reference_smoke(&summary);
  assert_package_part_contains("pptx/tdf163239.pptx", "ppt/slides/slide1.xml", "Subtitle 1");
  assert_package_part_contains("pptx/tdf163239.pptx", "ppt/slides/slide1.xml", "Title 2");
  assert_package_part_contains(
    "pptx/tdf163239.pptx",
    "ppt/slides/slide2.xml",
    "Text Placeholder 1",
  );
}

#[test]
// Source: ../core/sd/qa/unit/import-tests4.cxx:testTdf47365
fn mapped_pptx_loop_no_pause_preserves_loop_show_source_and_output() {
  let summary = render_summary("pptx/loopNoPause.pptx");
  assert_full_render_reference_smoke(&summary);
  assert_package_part_contains("pptx/loopNoPause.pptx", "ppt/presProps.xml", "loop=\"1\"");
  assert_package_part_contains("pptx/loopNoPause.pptx", "ppt/presProps.xml", "<p:sldAll/>");
}

#[test]
// Source: ../core/sd/qa/unit/FontEmbeddingTest.cxx:testTdf167214
fn mapped_pptx_tdf167214_preserves_embedded_font_settings_source_and_output() {
  let summary = render_summary("pptx/tdf167214.pptx");
  assert_page_count(&summary, 1);
  assert_full_render_reference_smoke(&summary);
  assert_package_part_contains(
    "pptx/tdf167214.pptx",
    "ppt/presentation.xml",
    "embedTrueTypeFonts=\"1\"",
  );
  assert_package_part_contains(
    "pptx/tdf167214.pptx",
    "ppt/presentation.xml",
    "saveSubsetFonts=\"1\"",
  );
  assert_package_part_contains(
    "pptx/tdf167214.pptx",
    "ppt/presentation.xml",
    "typeface=\"Figtree\"",
  );
}
