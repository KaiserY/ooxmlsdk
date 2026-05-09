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
  if libreoffice.image_count != rust.image_count {
    issues.push(issue(
      "images",
      format!(
        "image count differs: libreoffice={} rust={}",
        libreoffice.image_count, rust.image_count
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
