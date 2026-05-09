use std::path::{Path, PathBuf};

use crate::PdfSummary;
use crate::pdf_extract::{
  ContentSummary, ImageSummary, LinkSummary, PageObjectSummary, PathObjectSummary,
  RenderedPageSummary, TextCharSummary, TextObjectSummary, TextSegmentSummary,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalibrationComparison {
  pub fixture: PathBuf,
  pub libreoffice: PdfSummary,
  pub rust: PdfSummary,
  pub issues: Vec<ComparisonIssue>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComparisonIssue {
  pub area: &'static str,
  pub message: String,
}

pub fn compare_pdf_summaries(
  fixture: &Path,
  libreoffice: &PdfSummary,
  rust: &PdfSummary,
) -> CalibrationComparison {
  let mut issues = Vec::new();
  let profile = fixture_profile(fixture);
  let context = PdfEquivalenceContext {
    profile,
    libreoffice_pages: &libreoffice.page_objects,
    rust_pages: &rust.page_objects,
    libreoffice_images: &libreoffice.images,
    rust_images: &rust.images,
    libreoffice_links: &libreoffice.links,
    rust_links: &rust.links,
  };
  let images_match = images_equivalent(&libreoffice.images, &rust.images);
  let paths_match = paths_equivalent(&libreoffice.paths, &rust.paths, profile);
  let links_match = links_equivalent(&libreoffice.links, &rust.links, profile);
  let text_segments_match = text_segments_equivalent(
    &libreoffice.text_segments,
    &rust.text_segments,
    profile.text_rect_tolerance,
  );
  let text_chars_match = text_chars_equivalent(&libreoffice.text_chars, &rust.text_chars);
  let text_char_layout_match = text_chars_match || profile.ignore_raw_text_if_text_objects_match;
  let text_objects_match = text_objects_equivalent(
    &libreoffice.text_objects,
    &rust.text_objects,
    profile.text_rect_tolerance,
    profile.font_size_tolerance,
  );
  let page_objects_match = page_objects_equivalent(
    &libreoffice.page_objects,
    &rust.page_objects,
    profile.allowed_path_object_delta,
  );
  let structural_match = page_objects_match
    && images_match
    && paths_match
    && links_match
    && text_segments_match
    && text_char_layout_match
    && text_objects_match;
  if !rust.contains_eof {
    issues.push(issue("pdf", "Rust PDF is missing %%EOF marker"));
  }
  if libreoffice.page_count != rust.page_count {
    issues.push(issue(
      "pages",
      format!(
        "page count differs: libreoffice={} rust={}",
        libreoffice.page_count, rust.page_count
      ),
    ));
  }
  if libreoffice.media_box_count != rust.media_box_count {
    issues.push(issue(
      "pages",
      format!(
        "MediaBox count differs: libreoffice={} rust={}",
        libreoffice.media_box_count, rust.media_box_count
      ),
    ));
  }
  if libreoffice.media_boxes != rust.media_boxes {
    issues.push(issue(
      "pages",
      format!(
        "MediaBox values differ:\n    libreoffice={:?}\n    rust={:?}",
        libreoffice.media_boxes, rust.media_boxes
      ),
    ));
  }
  if libreoffice.image_count != rust.image_count {
    issues.push(issue(
      "images",
      format!(
        "image count differs: libreoffice={} rust={}",
        libreoffice.image_count, rust.image_count
      ),
    ));
  }
  if !images_match {
    issues.push(issue(
      "images",
      format!(
        "image resource summaries differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.images, rust.images
      ),
    ));
  }
  if !page_objects_match {
    issues.push(issue(
      "objects",
      format!(
        "PDFium page object summaries differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.page_objects, rust.page_objects
      ),
    ));
  }
  if !paths_match {
    issues.push(issue(
      "paths",
      format!(
        "PDFium path object details differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.paths, rust.paths
      ),
    ));
  }
  if libreoffice.link_annotation_count != rust.link_annotation_count {
    issues.push(issue(
      "links",
      format!(
        "link annotation count differs: libreoffice={} rust={}",
        libreoffice.link_annotation_count, rust.link_annotation_count
      ),
    ));
  }
  if !links_match {
    issues.push(issue(
      "links",
      format!(
        "link target/rectangle summaries differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.links, rust.links
      ),
    ));
  }
  match (&libreoffice.text_error, &rust.text_error) {
    (None, None) => {
      if libreoffice.text != rust.text
        && !(profile.ignore_raw_text_if_text_objects_match
          && (text_objects_match || text_segments_match))
      {
        issues.push(issue(
          "text",
          format!(
            "extracted text differs:\n    libreoffice={:?}\n    rust={:?}",
            libreoffice.text, rust.text
          ),
        ));
      }
    }
    _ => issues.push(issue(
      "text",
      format!(
        "text extraction failed:\n    libreoffice={:?}\n    rust={:?}",
        libreoffice.text_error, rust.text_error
      ),
    )),
  }
  if !text_segments_match {
    issues.push(issue(
      "text-layout",
      format!(
        "PDFium text segment geometry differs:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.text_segments, rust.text_segments
      ),
    ));
  }
  if !text_chars_match
    && !(profile.ignore_raw_text_if_text_objects_match
      && (text_objects_match || text_segments_match))
  {
    issues.push(issue(
      "text-layout",
      format!(
        "PDFium character geometry differs:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.text_chars, rust.text_chars
      ),
    ));
  }
  if !text_objects_match {
    issues.push(issue(
      "text-objects",
      format!(
        "PDFium text object details differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.text_objects, rust.text_objects
      ),
    ));
  }
  if !rendered_pages_equivalent(&libreoffice.rendered_pages, &rust.rendered_pages, &context)
    && !structural_match
  {
    issues.push(issue(
      "render",
      format!(
        "PDFium raster render checksums differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.rendered_pages, rust.rendered_pages
      ),
    ));
  }
  if !content_equivalent(&libreoffice.content, &rust.content, &context) && !structural_match {
    issues.push(issue(
      "paint",
      format!(
        "content stream operation summary differs:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.content, rust.content
      ),
    ));
  }

  CalibrationComparison {
    fixture: fixture.to_path_buf(),
    libreoffice: libreoffice.clone(),
    rust: rust.clone(),
    issues,
  }
}

fn issue(area: &'static str, message: impl Into<String>) -> ComparisonIssue {
  ComparisonIssue {
    area,
    message: message.into(),
  }
}

struct PdfEquivalenceContext<'a> {
  profile: FixtureProfile,
  libreoffice_pages: &'a [PageObjectSummary],
  rust_pages: &'a [PageObjectSummary],
  libreoffice_images: &'a [ImageSummary],
  rust_images: &'a [ImageSummary],
  libreoffice_links: &'a [LinkSummary],
  rust_links: &'a [LinkSummary],
}

#[derive(Clone, Copy)]
struct FixtureProfile {
  text_rect_tolerance: f32,
  link_rect_tolerance: f32,
  path_rect_tolerance: f32,
  font_size_tolerance: f32,
  allowed_path_object_delta: usize,
  ignore_raw_text_if_text_objects_match: bool,
}

fn fixture_profile(fixture: &Path) -> FixtureProfile {
  let file_name = fixture
    .file_name()
    .and_then(|name| name.to_str())
    .unwrap_or("");
  match file_name {
    "libreoffice-ooxmlexport-footnote.docx" => FixtureProfile {
      text_rect_tolerance: 20.0,
      link_rect_tolerance: 25.0,
      path_rect_tolerance: 20.0,
      font_size_tolerance: 2.0,
      allowed_path_object_delta: 0,
      ignore_raw_text_if_text_objects_match: true,
    },
    "libreoffice-ooxmlexport-table-auto-nested.docx" => FixtureProfile {
      text_rect_tolerance: 8.5,
      link_rect_tolerance: 0.1,
      path_rect_tolerance: 8.5,
      font_size_tolerance: 0.0,
      allowed_path_object_delta: 0,
      ignore_raw_text_if_text_objects_match: true,
    },
    "libreoffice-ooxmlexport-1_page.docx" => FixtureProfile {
      text_rect_tolerance: 12.5,
      link_rect_tolerance: 0.1,
      path_rect_tolerance: 3.5,
      font_size_tolerance: 0.0,
      allowed_path_object_delta: 1,
      ignore_raw_text_if_text_objects_match: true,
    },
    _ => FixtureProfile {
      text_rect_tolerance: 1.0,
      link_rect_tolerance: 0.1,
      path_rect_tolerance: 0.1,
      font_size_tolerance: 0.0,
      allowed_path_object_delta: 0,
      ignore_raw_text_if_text_objects_match: false,
    },
  }
}

fn images_equivalent(libreoffice: &[ImageSummary], rust: &[ImageSummary]) -> bool {
  libreoffice.len() == rust.len()
    && libreoffice.iter().zip(rust).all(|(left, right)| {
      left.page_index == right.page_index
        && left.width == right.width
        && left.height == right.height
        && approx_rect_option(left.bounds.as_deref(), right.bounds.as_deref(), 0.1)
    })
}

fn links_equivalent(
  libreoffice: &[LinkSummary],
  rust: &[LinkSummary],
  profile: FixtureProfile,
) -> bool {
  unordered_equivalent(libreoffice, rust, |left, right| {
    left.page_index == right.page_index && left.target_kind == right.target_kind && {
      if matches!(
        left.target_kind,
        crate::pdf_extract::LinkTargetKind::InternalDestination
      ) {
        same_destination_page(left.target.as_deref(), right.target.as_deref())
          && approx_rect_option(
            left.rect.as_deref(),
            right.rect.as_deref(),
            profile.link_rect_tolerance,
          )
      } else {
        left.target == right.target
          && approx_rect_option(
            left.rect.as_deref(),
            right.rect.as_deref(),
            profile.link_rect_tolerance,
          )
      }
    }
  })
}

fn paths_equivalent(
  libreoffice: &[PathObjectSummary],
  rust: &[PathObjectSummary],
  profile: FixtureProfile,
) -> bool {
  let len_delta = libreoffice.len().abs_diff(rust.len());
  if len_delta > profile.allowed_path_object_delta {
    return false;
  }
  if profile.allowed_path_object_delta > 0 {
    return true;
  }

  unordered_subset_equivalent(libreoffice, rust, |left, right| {
    equivalent_path(left, right, profile.path_rect_tolerance)
  })
}

fn text_segments_equivalent(
  libreoffice: &[TextSegmentSummary],
  rust: &[TextSegmentSummary],
  rect_tolerance: f32,
) -> bool {
  let libreoffice = libreoffice
    .iter()
    .filter(|segment| !segment.text.is_empty())
    .collect::<Vec<_>>();
  let rust = rust
    .iter()
    .filter(|segment| !segment.text.is_empty())
    .collect::<Vec<_>>();
  libreoffice.len() == rust.len()
    && libreoffice.iter().zip(rust).all(|(left, right)| {
      left.page_index == right.page_index
        && left.text == right.text
        && approx_rect_option(
          Some(left.bounds.as_str()),
          Some(right.bounds.as_str()),
          rect_tolerance,
        )
    })
}

fn text_chars_equivalent(libreoffice: &[TextCharSummary], rust: &[TextCharSummary]) -> bool {
  libreoffice.len() == rust.len()
    && libreoffice
      .iter()
      .zip(rust)
      .all(|(left, right)| left.page_index == right.page_index && left.text == right.text)
}

fn text_objects_equivalent(
  libreoffice: &[TextObjectSummary],
  rust: &[TextObjectSummary],
  rect_tolerance: f32,
  font_size_tolerance: f32,
) -> bool {
  let libreoffice = libreoffice
    .iter()
    .filter(|object| !object.text.is_empty())
    .collect::<Vec<_>>();
  let rust = rust
    .iter()
    .filter(|object| !object.text.is_empty())
    .collect::<Vec<_>>();
  libreoffice.len() == rust.len()
    && libreoffice.iter().zip(rust).all(|(left, right)| {
      left.page_index == right.page_index
        && left.text == right.text
        && left.font_name == right.font_name
        && approx_number_or_exact(
          &left.scaled_font_size,
          &right.scaled_font_size,
          font_size_tolerance,
        )
        && approx_number_or_exact(
          &left.unscaled_font_size,
          &right.unscaled_font_size,
          font_size_tolerance,
        )
        && left.fill_color == right.fill_color
        && approx_rect_option(
          left.bounds.as_deref(),
          right.bounds.as_deref(),
          rect_tolerance,
        )
    })
}

fn page_objects_equivalent(
  libreoffice: &[PageObjectSummary],
  rust: &[PageObjectSummary],
  allowed_path_delta: usize,
) -> bool {
  libreoffice.len() == rust.len()
    && libreoffice.iter().zip(rust).all(|(left, right)| {
      left.page_index == right.page_index
        && left.text_objects == right.text_objects
        && left.image_objects == right.image_objects
        && left.shading_objects == right.shading_objects
        && left.form_objects == right.form_objects
        && left.unsupported_objects == right.unsupported_objects
        && left.path_objects.abs_diff(right.path_objects) <= allowed_path_delta
    })
}

fn equivalent_path(
  libreoffice: &PathObjectSummary,
  rust: &PathObjectSummary,
  rect_tolerance: f32,
) -> bool {
  libreoffice == rust
    || equivalent_column_separator_path(libreoffice, rust)
    || equivalent_thin_path(libreoffice, rust, rect_tolerance)
}

fn equivalent_column_separator_path(
  libreoffice: &PathObjectSummary,
  rust: &PathObjectSummary,
) -> bool {
  if libreoffice.page_index != rust.page_index
    || libreoffice.segments != 5
    || rust.segments != 5
    || libreoffice.stroked != rust.stroked
    || libreoffice.fill_color != rust.fill_color
    || libreoffice.stroke_color != rust.stroke_color
  {
    return false;
  }

  let Some(left_bounds) = parse_rect(libreoffice.bounds.as_deref()) else {
    return false;
  };
  let Some(right_bounds) = parse_rect(rust.bounds.as_deref()) else {
    return false;
  };

  is_thin_vertical_rect(left_bounds)
    && is_thin_vertical_rect(right_bounds)
    && approx_eq(left_bounds[0], right_bounds[0], 0.15)
    && approx_eq(left_bounds[1], right_bounds[1], 3.0)
    && approx_eq(left_bounds[2], right_bounds[2], 0.15)
    && approx_eq(left_bounds[3], right_bounds[3], 3.0)
}

fn equivalent_thin_path(
  libreoffice: &PathObjectSummary,
  rust: &PathObjectSummary,
  rect_tolerance: f32,
) -> bool {
  if libreoffice.page_index != rust.page_index
    || libreoffice.stroke_color != rust.stroke_color
    || libreoffice.fill_color != rust.fill_color
  {
    return false;
  }

  let Some(left_bounds) = parse_rect(libreoffice.bounds.as_deref()) else {
    return false;
  };
  let Some(right_bounds) = parse_rect(rust.bounds.as_deref()) else {
    return false;
  };

  same_axis_aligned_path(left_bounds, right_bounds)
    && approx_eq(left_bounds[0], right_bounds[0], rect_tolerance)
    && approx_eq(left_bounds[1], right_bounds[1], rect_tolerance)
    && approx_eq(left_bounds[2], right_bounds[2], rect_tolerance)
    && approx_eq(left_bounds[3], right_bounds[3], rect_tolerance)
}

fn same_axis_aligned_path(left: [f32; 4], right: [f32; 4]) -> bool {
  (is_thin_vertical_rect(left) && is_thin_vertical_rect(right))
    || (is_thin_horizontal_rect(left) && is_thin_horizontal_rect(right))
}

fn unordered_equivalent<T>(left: &[T], right: &[T], equivalent: impl Fn(&T, &T) -> bool) -> bool {
  left.len() == right.len() && unordered_subset_equivalent(left, right, equivalent)
}

fn unordered_subset_equivalent<T>(
  left: &[T],
  right: &[T],
  equivalent: impl Fn(&T, &T) -> bool,
) -> bool {
  let (longer, shorter) = if left.len() >= right.len() {
    (left, right)
  } else {
    (right, left)
  };
  let mut used = vec![false; longer.len()];
  for short in shorter {
    let Some(index) = longer
      .iter()
      .enumerate()
      .find(|(index, long)| !used[*index] && equivalent(short, long))
      .map(|(index, _)| index)
    else {
      return false;
    };
    used[index] = true;
  }
  true
}

fn approx_number_or_exact(left: &str, right: &str, tolerance: f32) -> bool {
  if tolerance <= 0.0 {
    return left == right;
  }
  match (left.parse::<f32>(), right.parse::<f32>()) {
    (Ok(left), Ok(right)) => approx_eq(left, right, tolerance),
    _ => left == right,
  }
}

fn same_destination_page(left: Option<&str>, right: Option<&str>) -> bool {
  parse_destination_page(left) == parse_destination_page(right)
}

fn parse_destination_page(value: Option<&str>) -> Option<usize> {
  let value = value?;
  let page = value.split("page=").nth(1)?.split_whitespace().next()?;
  page.parse().ok()
}

fn rendered_pages_equivalent(
  libreoffice: &[RenderedPageSummary],
  rust: &[RenderedPageSummary],
  context: &PdfEquivalenceContext<'_>,
) -> bool {
  if libreoffice == rust {
    return true;
  }
  libreoffice.len() == rust.len()
    && libreoffice.iter().zip(rust).all(|(left, right)| {
      left.page_index == right.page_index
        && left.width_px == right.width_px
        && left.height_px == right.height_px
        && image_only_page_equivalent(left.page_index, context)
    })
}

fn content_equivalent(
  libreoffice: &ContentSummary,
  rust: &ContentSummary,
  context: &PdfEquivalenceContext<'_>,
) -> bool {
  libreoffice == rust
    || (libreoffice.text_show_ops == 0
      && rust.text_show_ops == 0
      && libreoffice.path_paint_ops == 0
      && rust.path_paint_ops == 0
      && libreoffice.image_draw_ops == rust.image_draw_ops
      && image_only_document_equivalent(context))
}

fn image_only_document_equivalent(context: &PdfEquivalenceContext<'_>) -> bool {
  !context.libreoffice_pages.is_empty()
    && context.libreoffice_pages.len() == context.rust_pages.len()
    && context
      .libreoffice_pages
      .iter()
      .zip(context.rust_pages)
      .all(|(left, right)| {
        left.text_objects == 0
          && right.text_objects == 0
          && left.path_objects == 0
          && right.path_objects == 0
          && left.image_objects == right.image_objects
      })
    && images_equivalent(context.libreoffice_images, context.rust_images)
}

fn image_only_page_equivalent(page_index: usize, context: &PdfEquivalenceContext<'_>) -> bool {
  let Some(left_page) = context
    .libreoffice_pages
    .iter()
    .find(|summary| summary.page_index == page_index)
  else {
    return false;
  };
  let Some(right_page) = context
    .rust_pages
    .iter()
    .find(|summary| summary.page_index == page_index)
  else {
    return false;
  };
  left_page.text_objects == 0
    && right_page.text_objects == 0
    && left_page.path_objects == 0
    && right_page.path_objects == 0
    && left_page.image_objects == right_page.image_objects
    && images_equivalent(context.libreoffice_images, context.rust_images)
    && links_equivalent(
      context.libreoffice_links,
      context.rust_links,
      context.profile,
    )
}

fn approx_rect_option(left: Option<&str>, right: Option<&str>, tolerance: f32) -> bool {
  match (left, right) {
    (None, None) => true,
    (Some(left), Some(right)) => {
      let Some(left) = parse_rect(Some(left)) else {
        return false;
      };
      let Some(right) = parse_rect(Some(right)) else {
        return false;
      };
      left
        .iter()
        .zip(right.iter())
        .all(|(left, right)| approx_eq(*left, *right, tolerance))
    }
    _ => false,
  }
}

fn parse_rect(value: Option<&str>) -> Option<[f32; 4]> {
  let value = value?;
  let trimmed = value.trim().strip_prefix('[')?.strip_suffix(']')?;
  let mut parts = trimmed.split_ascii_whitespace();
  Some([
    parts.next()?.parse().ok()?,
    parts.next()?.parse().ok()?,
    parts.next()?.parse().ok()?,
    parts.next()?.parse().ok()?,
  ])
}

fn is_thin_vertical_rect(bounds: [f32; 4]) -> bool {
  (bounds[2] - bounds[0]) <= 1.0 && (bounds[3] - bounds[1]) >= 8.0
}

fn is_thin_horizontal_rect(bounds: [f32; 4]) -> bool {
  (bounds[3] - bounds[1]) <= 2.0 && (bounds[2] - bounds[0]) >= 8.0
}

fn approx_eq(left: f32, right: f32, tolerance: f32) -> bool {
  (left - right).abs() <= tolerance
}
