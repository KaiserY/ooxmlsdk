# specs

This bucket is for project-owned fixtures that exercise OOXML spec behavior
without claiming provenance from upstream `../Open-XML-SDK`.

Rules:

- Keep fixtures grouped by domain such as `wml/`, `spreadsheet/`, `pml/`,
  `drawingml/`, `opc/`, `mce/`, and document-level smoke sets.
- Keep `known_failures.toml` here because it belongs to the
  `test-data/ooxmlsdk-test/specs/` compatibility sweep, not to upstream
  fixture inventory.
- `crates/ooxmlsdk-test/tests/round_trip.rs::round_trip_smoke_test` scans only
  this subtree.
- If a fixture can be traced to upstream Open XML SDK assets, move it to
  `../Open-XML-SDK/` instead of duplicating it here.
- If a fixture does not fit a spec-backed bucket, move it to `../misc/`.
