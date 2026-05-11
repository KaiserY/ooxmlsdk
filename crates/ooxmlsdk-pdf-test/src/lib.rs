//! PDF export test helpers for `ooxmlsdk-pdf`.
//!
//! This crate is intentionally separate from the runtime PDF converter. It
//! renders checked-in DOCX fixtures through `ooxmlsdk-pdf` and exposes PDFium-
//! based summaries for tests that mirror upstream LibreOffice `pdfexport`
//! assertions.

pub mod pdf_extract;
pub mod render;

use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

pub use pdf_extract::{
  AnnotationSummary, LinkTargetKind, PdfSummary, RawAnnotationSummary, RawPageSummary,
  RawXObjectSummary,
};
pub use render::render_fixture_pdf;

pub type Result<T> = std::result::Result<T, CalibrationError>;

#[derive(Debug, thiserror::Error)]
pub enum CalibrationError {
  #[error("I/O error: {0}")]
  Io(#[from] std::io::Error),
  #[error("ooxmlsdk-pdf conversion failed: {0}")]
  Pdf(#[from] ooxmlsdk_pdf::PdfError),
  #[error("PDFium extraction failed: {0}")]
  PdfiumExtraction(String),
}

pub fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("../..")
    .canonicalize()
    .unwrap_or_else(|_| Path::new(env!("CARGO_MANIFEST_DIR")).join("../.."))
}

pub fn pdfexport_fixture_dir() -> PathBuf {
  workspace_root().join("test-data/ooxmlsdk-pdf-test/libreoffice")
}

pub fn pdfexport_fixtures() -> Vec<PathBuf> {
  let mut fixtures = Vec::new();
  collect_word_documents(&pdfexport_fixture_dir(), &mut fixtures);
  fixtures.sort();
  fixtures
}

pub fn pdf_summary_for_fixture(fixture: &Path) -> Result<PdfSummary> {
  // PDFium extraction is not reliably parallel-safe in this harness.
  let _guard = pdf_test_lock().lock().unwrap();
  let pdf = render_fixture_pdf(fixture)?;
  PdfSummary::from_bytes(&pdf).map_err(CalibrationError::PdfiumExtraction)
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

fn pdf_test_lock() -> &'static Mutex<()> {
  static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
  LOCK.get_or_init(|| Mutex::new(()))
}
