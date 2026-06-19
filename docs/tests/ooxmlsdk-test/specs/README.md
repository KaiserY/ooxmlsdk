# specs

This bucket moved to
`../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/specs`. It is for project-owned
fixtures that exercise OOXML spec behavior without claiming provenance from
upstream `../Open-XML-SDK`.

Rules:

- Keep fixtures grouped by domain such as `wml/`, `spreadsheet/`, `pml/`,
  `drawingml/`, `opc/`, `mce/`, and document-level smoke sets.
- Keep `known_failures.toml` here because it belongs to the test-suite
  `fixtures/ooxmlsdk-test/specs/` compatibility sweep, not to upstream fixture
  inventory.
- `../ooxmlsdk-test-suite/crates/ooxmlsdk-test/tests/round_trip.rs::round_trip_smoke_test`
  scans only this subtree.
- If a fixture can be traced to upstream Open XML SDK assets, move it to the
  suite Open XML SDK corpus or raw XML fixture bucket instead of duplicating it
  here.
- If a fixture does not fit a spec-backed bucket, move it to the suite
  `fixtures/ooxmlsdk-test/misc/` bucket.
