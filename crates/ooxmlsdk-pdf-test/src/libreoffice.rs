use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use crate::{CalibrationError, Result};

const LIBREOFFICE_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LibreOfficeStatus {
  Available(PathBuf),
  Missing,
}

#[derive(Clone, Debug)]
pub struct LibreOffice {
  executable: PathBuf,
}

impl LibreOffice {
  pub fn discover() -> LibreOfficeStatus {
    if let Some(path) = env::var_os("LIBREOFFICE").and_then(executable_path) {
      return LibreOfficeStatus::Available(path);
    }
    for name in ["soffice", "libreoffice"] {
      if let Some(path) = find_on_path(name) {
        return LibreOfficeStatus::Available(path);
      }
    }
    LibreOfficeStatus::Missing
  }

  pub fn new(executable: PathBuf) -> Self {
    Self { executable }
  }

  pub fn render_docx_to_pdf(
    &self,
    fixture: &Path,
    out_dir: &Path,
    profile_dir: &Path,
    env_dir: &Path,
  ) -> Result<PathBuf> {
    std::fs::create_dir_all(out_dir)?;
    std::fs::create_dir_all(profile_dir)?;
    let home_dir = env_dir.join("home");
    let config_dir = env_dir.join("config");
    let cache_dir = env_dir.join("cache");
    let runtime_dir = env_dir.join("runtime");
    std::fs::create_dir_all(&home_dir)?;
    std::fs::create_dir_all(&config_dir)?;
    std::fs::create_dir_all(&cache_dir)?;
    std::fs::create_dir_all(&runtime_dir)?;
    restrict_runtime_dir(&runtime_dir)?;
    let profile_uri = format!("file://{}", profile_dir.display());
    let mut child = Command::new(&self.executable)
      .arg("--headless")
      .arg("--nologo")
      .arg("--nofirststartwizard")
      .arg(format!("-env:UserInstallation={profile_uri}"))
      .arg("--convert-to")
      .arg("pdf")
      .arg("--outdir")
      .arg(out_dir)
      .arg(fixture)
      .env("HOME", &home_dir)
      .env("XDG_CONFIG_HOME", &config_dir)
      .env("XDG_CACHE_HOME", &cache_dir)
      .env("XDG_RUNTIME_DIR", &runtime_dir)
      .env("SAL_USE_VCLPLUGIN", "svp")
      .spawn()?;

    let start = Instant::now();
    loop {
      if child.try_wait()?.is_some() {
        let output = child.wait_with_output()?;
        if !output.status.success() {
          return Err(CalibrationError::LibreOfficeFailed {
            fixture: fixture.to_path_buf(),
            status: output.status.to_string(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
          });
        }
        break;
      }

      if start.elapsed() >= LIBREOFFICE_TIMEOUT {
        let _ = child.kill();
        let _ = child.wait();
        return Err(CalibrationError::LibreOfficeTimedOut {
          fixture: fixture.to_path_buf(),
          seconds: LIBREOFFICE_TIMEOUT.as_secs(),
        });
      }

      std::thread::sleep(Duration::from_millis(100));
    }

    let mut pdf = out_dir.join(
      fixture
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .as_ref(),
    );
    pdf.set_extension("pdf");
    if !pdf.exists() {
      return Err(CalibrationError::MissingLibreOfficePdf {
        fixture: fixture.to_path_buf(),
      });
    }
    Ok(pdf)
  }
}

fn executable_path(value: OsString) -> Option<PathBuf> {
  let path = PathBuf::from(value);
  path.exists().then_some(path)
}

fn find_on_path(name: &str) -> Option<PathBuf> {
  let paths = env::var_os("PATH")?;
  env::split_paths(&paths)
    .map(|dir| dir.join(name))
    .find(|path| path.exists())
}

#[cfg(unix)]
fn restrict_runtime_dir(path: &Path) -> std::io::Result<()> {
  use std::os::unix::fs::PermissionsExt;

  std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
}

#[cfg(not(unix))]
fn restrict_runtime_dir(_path: &Path) -> std::io::Result<()> {
  Ok(())
}
