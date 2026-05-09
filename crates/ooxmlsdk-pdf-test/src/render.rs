use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ooxmlsdk_pdf::{PdfOptions, convert_docx};

use crate::libreoffice::{LibreOffice, LibreOfficeStatus};
use crate::{CalibrationError, Result};

#[derive(Debug)]
pub struct RenderedPair {
  pub fixture: PathBuf,
  pub libreoffice_pdf: Vec<u8>,
  pub rust_pdf: Vec<u8>,
}

pub fn render_pair(fixture: &Path) -> Result<RenderedPair> {
  let libreoffice = match LibreOffice::discover() {
    LibreOfficeStatus::Available(path) => LibreOffice::new(path),
    LibreOfficeStatus::Missing => {
      return Err(CalibrationError::LibreOfficeFailed {
        fixture: fixture.to_path_buf(),
        status: "not found".into(),
        stdout: String::new(),
        stderr: "LibreOffice executable not found; set LIBREOFFICE or install soffice".into(),
      });
    }
  };

  let temp = TempDir::new("ooxmlsdk-pdf-test")?;
  let out_dir = temp.path().join("out");
  let profile_dir = temp.path().join("profile");
  let env_dir = temp.path().join("env");
  let libreoffice_path =
    libreoffice.render_docx_to_pdf(fixture, &out_dir, &profile_dir, &env_dir)?;
  let libreoffice_pdf = std::fs::read(libreoffice_path)?;
  let rust_pdf = convert_docx(File::open(fixture)?, PdfOptions::default())?;

  Ok(RenderedPair {
    fixture: fixture.to_path_buf(),
    libreoffice_pdf,
    rust_pdf,
  })
}

struct TempDir {
  path: PathBuf,
}

impl TempDir {
  fn new(prefix: &str) -> std::io::Result<Self> {
    let mut path = std::env::temp_dir();
    let nanos = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_nanos();
    path.push(format!("{prefix}-{}-{nanos}", std::process::id()));
    std::fs::create_dir_all(&path)?;
    Ok(Self { path })
  }

  fn path(&self) -> &Path {
    &self.path
  }
}

impl Drop for TempDir {
  fn drop(&mut self) {
    let _ = std::fs::remove_dir_all(&self.path);
  }
}
