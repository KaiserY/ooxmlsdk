# ooxmlsdk-test Fixture Layout

`test-data/ooxmlsdk-test/` is the focused package fixture tree for
`crates/ooxmlsdk-test`.

Buckets:

- `Open-XML-SDK/`: fixtures copied from `../Open-XML-SDK` assets and tests.
  These are upstream compatibility fixtures used by validator and package
  behavior tests.
- `libreoffice/`: fixtures copied from `../core` and kept as supplemental
  real-world OOXML evidence.
- `specs/`: project-owned fixtures for spec-focused coverage and the
  `known_failures.toml` file used by the `round_trip_smoke_test` compatibility
  sweep.
- `misc/`: fixtures that are intentionally outside upstream Open XML SDK assets
  and outside the spec bucket. Keep this bucket small and explicitly justified.

Round-trip policy:

- Corpus-scale package round-trip coverage lives in the adjacent
  `../ooxmlsdk-test-suite/` checkout. Prefer the local checkout; its remote is
  `https://github.com/KaiserY/ooxmlsdk-test-suite`.
- `crates/ooxmlsdk-test/tests/round_trip.rs::round_trip_smoke_test` walks only
  `test-data/ooxmlsdk-test/specs/` as a lightweight smoke test, and only that
  subtree participates in `specs/known_failures.toml`.
- `test-data/ooxmlsdk-pdf-test/` is excluded from package round-trip coverage. Those fixtures
  are for `docx -> pdf` parity only.
