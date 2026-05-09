//! LibreOffice calibration helpers for `ooxmlsdk-pdf`.
//!
//! This crate is intentionally separate from the runtime PDF converter. The
//! full calibration lane depends on an installed LibreOffice executable and
//! writes temporary reference PDFs, so ordinary runtime tests should not depend
//! on it.

pub mod compare;
pub mod libreoffice;
pub mod pdf_extract;
pub mod render;
pub mod report;

use std::path::{Path, PathBuf};

pub use compare::{CalibrationComparison, ComparisonIssue, compare_pdf_summaries};
pub use libreoffice::{LibreOffice, LibreOfficeStatus};
pub use pdf_extract::PdfSummary;
pub use render::{RenderedPair, render_pair};
pub use report::format_report;

pub type Result<T> = std::result::Result<T, CalibrationError>;

#[derive(Debug, thiserror::Error)]
pub enum CalibrationError {
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),
  #[error("ooxmlsdk-pdf conversion failed: {0}")]
  Pdf(#[from] ooxmlsdk_pdf::PdfError),
  #[error("LibreOffice failed for {fixture}: status={status} stdout={stdout} stderr={stderr}")]
  LibreOfficeFailed {
    fixture: PathBuf,
    status: String,
    stdout: String,
    stderr: String,
  },
  #[error("LibreOffice did not create expected PDF for {fixture}")]
  MissingLibreOfficePdf { fixture: PathBuf },
  #[error("LibreOffice timed out for {fixture} after {seconds}s")]
  LibreOfficeTimedOut { fixture: PathBuf, seconds: u64 },
}

pub fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../..")
    .canonicalize()
    .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

pub fn calibration_fixture_dir() -> PathBuf {
  workspace_root().join("test-data/ooxmlsdk-pdf-test")
}

pub fn calibration_fixtures() -> Vec<PathBuf> {
  let mut fixtures = Vec::new();
  collect_word_documents(&calibration_fixture_dir(), &mut fixtures);
  fixtures.sort();
  fixtures
}

pub fn calibration_for_fixture(fixture: &Path) -> Result<CalibrationComparison> {
  let rendered = render_pair(fixture)?;
  let libreoffice = PdfSummary::from_bytes(&rendered.libreoffice_pdf);
  let rust = PdfSummary::from_bytes(&rendered.rust_pdf);
  Ok(compare_pdf_summaries(fixture, &libreoffice, &rust))
}

pub fn workspace_relative_path(path: &Path) -> String {
  let root = workspace_root();
  path
    .strip_prefix(&root)
    .unwrap_or(path)
    .to_string_lossy()
    .replace('\\', "/")
}

fn collect_word_documents(root: &Path, fixtures: &mut Vec<PathBuf>) {
  if !root.exists() {
    return;
  }

  for entry in walkdir::WalkDir::new(root)
    .sort_by_file_name()
    .into_iter()
    .filter_map(|entry| entry.ok())
    .filter(|entry| entry.file_type().is_file())
  {
    let path = entry.into_path();
    if is_word_document(&path) {
      fixtures.push(path);
    }
  }
}

fn is_word_document(path: &Path) -> bool {
  matches!(
    path.extension().and_then(|extension| extension.to_str()),
    Some("docx" | "docm" | "dotx" | "dotm")
  )
}
