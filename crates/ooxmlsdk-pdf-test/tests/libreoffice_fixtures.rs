use ooxmlsdk_pdf_test::{
  LibreOffice, LibreOfficeStatus, calibration_fixtures, calibration_for_fixture, format_report,
  workspace_relative_path,
};

#[test]
fn libreoffice_smoke_skips_when_soffice_is_missing() {
  match LibreOffice::discover() {
    LibreOfficeStatus::Available(path) => assert!(path.exists()),
    LibreOfficeStatus::Missing => {}
  }
}

#[test]
fn libreoffice_pdf_fixtures_match_basics() {
  if matches!(LibreOffice::discover(), LibreOfficeStatus::Missing) {
    eprintln!("skipping: LibreOffice executable not found; set LIBREOFFICE or install soffice");
    return;
  }

  let fixtures = calibration_fixtures();
  assert!(
    !fixtures.is_empty(),
    "no ooxmlsdk-pdf-test fixtures discovered"
  );

  let mut reports = Vec::new();
  for fixture in fixtures {
    match calibration_for_fixture(&fixture) {
      Ok(comparison) if comparison.issues.is_empty() => {}
      Ok(comparison) => reports.push(format_report(&comparison)),
      Err(error) => reports.push(format!("{}:\n  [render] {error}", fixture.display())),
    }
  }

  assert!(reports.is_empty(), "{}", reports.join("\n"));
}

#[test]
fn libreoffice_pdf_fixtures_have_calibration_inventory() {
  if matches!(LibreOffice::discover(), LibreOfficeStatus::Missing) {
    eprintln!("skipping: LibreOffice executable not found; set LIBREOFFICE or install soffice");
    return;
  }

  let fixtures = calibration_fixtures();
  assert!(
    !fixtures.is_empty(),
    "no ooxmlsdk-pdf-test fixtures discovered"
  );

  let mut rendered = 0usize;
  let mut comparison_issues = 0usize;
  let mut render_failures = Vec::new();
  let mut comparison_reports = Vec::new();

  for fixture in &fixtures {
    let rel = workspace_relative_path(fixture);
    match calibration_for_fixture(fixture) {
      Ok(comparison) if comparison.issues.is_empty() => {
        rendered += 1;
      }
      Ok(comparison) => {
        rendered += 1;
        comparison_issues += 1;
        comparison_reports.push(format_report(&comparison));
      }
      Err(error) => {
        render_failures.push(format!("{rel}:\n  [render] {error}"));
      }
    }
  }

  println!(
    "libreoffice calibration inventory: {} fixtures, {rendered} rendered, {} render failures, {comparison_issues} comparison issue reports",
    fixtures.len(),
    render_failures.len()
  );

  if !render_failures.is_empty() {
    println!(
      "render failures:\n{}",
      render_failures
        .iter()
        .take(25)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n")
    );
  }
  if !comparison_reports.is_empty() {
    println!(
      "comparison issues:\n{}",
      comparison_reports
        .iter()
        .take(25)
        .cloned()
        .collect::<Vec<_>>()
        .join("\n")
    );
  }

  assert!(
    rendered > 0,
    "no Word fixtures reached PDF comparison; first failures:\n{}",
    render_failures
      .iter()
      .take(10)
      .cloned()
      .collect::<Vec<_>>()
      .join("\n")
  );
}
