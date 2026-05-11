# ooxmlsdk-test Fixture Layout

`test-data/ooxmlsdk-test/` is the package round-trip fixture tree for
`crates/ooxmlsdk-test`.

Buckets:

- `Open-XML-SDK/`: fixtures copied from `../Open-XML-SDK` assets and tests.
  These are the upstream compatibility fixtures used by validator, package, and
  round-trip coverage.
- `libreoffice/`: fixtures copied from `../core` and used for package
  round-trip coverage as supplemental real-world OOXML evidence.
- `specs/`: project-owned fixtures for spec-focused coverage and the
  `known_failures.toml` file used by the `round_trip_smoke_test` compatibility
  sweep.
- `misc/`: fixtures that are intentionally outside upstream Open XML SDK assets
  and outside the spec bucket. Keep this bucket small and explicitly justified.

Round-trip policy:

- The generated `crates/ooxmlsdk-test/tests/doc_samples.rs` coverage now reads
  upstream fixtures from `test-data/ooxmlsdk-test/Open-XML-SDK/`.
- The broader generated round-trip coverage in
  `crates/ooxmlsdk-test/tests/doc_samples.rs` also walks `libreoffice/`,
  `specs/`, and `misc/`.
- `crates/ooxmlsdk-test/tests/round_trip.rs::round_trip_smoke_test` walks only
  `test-data/ooxmlsdk-test/specs/`, and only that subtree participates in
  `specs/known_failures.toml`.
- `test-data/ooxmlsdk-pdf-test/` is excluded from this harness. Those fixtures
  are for `docx -> pdf` parity only.
