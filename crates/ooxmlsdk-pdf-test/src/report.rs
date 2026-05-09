use crate::CalibrationComparison;

pub fn format_report(comparison: &CalibrationComparison) -> String {
  if comparison.issues.is_empty() {
    return format!("{}: OK", comparison.fixture.display());
  }

  let mut report = format!("{}:\n", comparison.fixture.display());
  for issue in &comparison.issues {
    report.push_str(&format!("  [{}] {}\n", issue.area, issue.message));
  }
  report
}
