use std::path::{Path, PathBuf};

use crate::PdfSummary;

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
  if libreoffice.images != rust.images {
    issues.push(issue(
      "images",
      format!(
        "image resource summaries differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.images, rust.images
      ),
    ));
  }
  if libreoffice.page_objects != rust.page_objects {
    issues.push(issue(
      "objects",
      format!(
        "PDFium page object summaries differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.page_objects, rust.page_objects
      ),
    ));
  }
  if libreoffice.paths != rust.paths {
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
  if libreoffice.links != rust.links {
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
      if libreoffice.text != rust.text {
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
  if libreoffice.text_segments != rust.text_segments {
    issues.push(issue(
      "text-layout",
      format!(
        "PDFium text segment geometry differs:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.text_segments, rust.text_segments
      ),
    ));
  }
  if libreoffice.text_chars != rust.text_chars {
    issues.push(issue(
      "text-layout",
      format!(
        "PDFium character geometry differs:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.text_chars, rust.text_chars
      ),
    ));
  }
  if libreoffice.text_objects != rust.text_objects {
    issues.push(issue(
      "text-objects",
      format!(
        "PDFium text object details differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.text_objects, rust.text_objects
      ),
    ));
  }
  if libreoffice.rendered_pages != rust.rendered_pages {
    issues.push(issue(
      "render",
      format!(
        "PDFium raster render checksums differ:\n    libreoffice={:#?}\n    rust={:#?}",
        libreoffice.rendered_pages, rust.rendered_pages
      ),
    ));
  }
  if libreoffice.content != rust.content {
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
