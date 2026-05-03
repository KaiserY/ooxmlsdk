#![cfg(feature = "parts")]

use std::collections::HashSet;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use ooxmlsdk::parts::{
  presentation_document::PresentationDocument, spreadsheet_document::SpreadsheetDocument,
  wordprocessing_document::WordprocessingDocument,
};

struct KnownFailure {
  file: String,
  reason: String,
  issue: u32,
}

fn workspace_root() -> PathBuf {
  Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .expect("ooxmlsdk-test should be under a crates dir")
    .parent()
    .expect("crates dir should be under workspace root")
    .to_path_buf()
}

fn load_known_failures(workspace_root: &Path) -> Vec<KnownFailure> {
  let path = workspace_root.join("test-data/known_failures.toml");
  if !path.exists() {
    return Vec::new();
  }
  let content = std::fs::read_to_string(&path).expect("read test-data/known_failures.toml");
  let parsed: toml::Value =
    toml::from_str(&content).expect("parse test-data/known_failures.toml as TOML");

  let Some(arr) = parsed.get("failure").and_then(|v| v.as_array()) else {
    return Vec::new();
  };

  arr
    .iter()
    .filter_map(|entry| {
      let file = entry.get("file")?.as_str()?.to_string();
      let reason = entry.get("reason")?.as_str()?.to_string();
      let issue = entry.get("issue").and_then(|v| v.as_integer()).unwrap_or(0) as u32;
      Some(KnownFailure {
        file,
        reason,
        issue,
      })
    })
    .collect()
}

enum DocKind {
  Wordprocessing,
  Spreadsheet,
  Presentation,
}

fn doc_kind(path: &Path) -> Option<DocKind> {
  match path.extension().and_then(|e| e.to_str()) {
    Some("docx") | Some("dotx") => Some(DocKind::Wordprocessing),
    Some("xlsx") | Some("xltx") => Some(DocKind::Spreadsheet),
    Some("pptx") | Some("potx") => Some(DocKind::Presentation),
    _ => None,
  }
}

fn try_round_trip(path: &Path) -> Result<(), String> {
  match doc_kind(path).expect("try_round_trip called on non-document file") {
    DocKind::Wordprocessing => {
      let doc = WordprocessingDocument::new_from_file(path)
        .map_err(|e| format!("  Step: open()\n  Error: {e}"))?;
      let mut buf = Cursor::new(Vec::new());
      doc
        .save(&mut buf)
        .map_err(|e| format!("  Step: save()\n  Error: {e}"))?;
      WordprocessingDocument::new(Cursor::new(buf.into_inner()))
        .map_err(|e| format!("  Step: reopen()\n  Error: {e}"))?;
    }
    DocKind::Spreadsheet => {
      let doc = SpreadsheetDocument::new_from_file(path)
        .map_err(|e| format!("  Step: open()\n  Error: {e}"))?;
      let mut buf = Cursor::new(Vec::new());
      doc
        .save(&mut buf)
        .map_err(|e| format!("  Step: save()\n  Error: {e}"))?;
      SpreadsheetDocument::new(Cursor::new(buf.into_inner()))
        .map_err(|e| format!("  Step: reopen()\n  Error: {e}"))?;
    }
    DocKind::Presentation => {
      let doc = PresentationDocument::new_from_file(path)
        .map_err(|e| format!("  Step: open()\n  Error: {e}"))?;
      let mut buf = Cursor::new(Vec::new());
      doc
        .save(&mut buf)
        .map_err(|e| format!("  Step: save()\n  Error: {e}"))?;
      PresentationDocument::new(Cursor::new(buf.into_inner()))
        .map_err(|e| format!("  Step: reopen()\n  Error: {e}"))?;
    }
  }
  Ok(())
}

#[test]
fn round_trip_smoke_test() {
  let root = workspace_root();
  let test_data = root.join("test-data");

  if !test_data.exists() {
    eprintln!(
      "warning: test-data/ not found at {}, run \
             `cargo run -p ooxmlsdk-test --example create_fixtures` first",
      test_data.display()
    );
    return;
  }

  let known_failures = load_known_failures(&root);
  let known_failure_map: HashSet<String> = known_failures.iter().map(|f| f.file.clone()).collect();

  let mut passed = 0usize;
  let mut known_failed = 0usize;
  let mut unexpected_successes: Vec<String> = Vec::new();
  let mut unexpected_failures: Vec<String> = Vec::new();

  let mut entries: Vec<_> = walkdir::WalkDir::new(&test_data)
    .sort_by_file_name()
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.file_type().is_file())
    .collect();
  entries.sort_by(|a, b| a.path().cmp(b.path()));

  for entry in entries {
    let path = entry.path();
    if doc_kind(path).is_none() {
      continue;
    }

    // Normalise to forward-slash relative path for cross-platform lookup
    let rel = path
      .strip_prefix(&root)
      .unwrap_or(path)
      .to_string_lossy()
      .replace('\\', "/");

    let is_known = known_failure_map.contains(&rel);
    let result = try_round_trip(path);

    match (result, is_known) {
      (Ok(()), false) => {
        passed += 1;
      }
      (Ok(()), true) => {
        let kf = known_failures
          .iter()
          .find(|f| f.file == rel)
          .expect("in map");
        unexpected_successes.push(format!(
          "UNEXPECTED SUCCESS: {rel}\n  \
                     Was expected to fail: {}\n  \
                     Remove the entry from test-data/known_failures.toml.",
          kf.reason
        ));
      }
      (Err(_), true) => {
        let kf = known_failures
          .iter()
          .find(|f| f.file == rel)
          .expect("in map");
        let issue_str = if kf.issue > 0 {
          format!("issue #{}", kf.issue)
        } else {
          "no issue filed yet".to_string()
        };
        println!("KNOWN FAILURE: {rel} — {} ({issue_str})", kf.reason);
        known_failed += 1;
      }
      (Err(err), false) => {
        unexpected_failures.push(format!(
          "ROUND-TRIP FAILED: {rel}\n{err}\n\n  \
                     If this is a known limitation, add it to test-data/known_failures.toml.\n  \
                     If this is a regression, please file an issue."
        ));
      }
    }
  }

  if !unexpected_successes.is_empty() || !unexpected_failures.is_empty() {
    let mut msg = String::new();
    for s in &unexpected_successes {
      msg.push_str(s);
      msg.push('\n');
    }
    for f in &unexpected_failures {
      msg.push_str(f);
      msg.push('\n');
    }
    panic!("{msg}");
  }

  println!("round_trip: {passed} passed, {known_failed} known failures");
}
