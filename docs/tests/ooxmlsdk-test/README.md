# ooxmlsdk-test Fixture Layout

The focused package test crate and its fixtures have moved to the adjacent
`../ooxmlsdk-test-suite/` checkout:

- crate: `../ooxmlsdk-test-suite/crates/ooxmlsdk-test`
- project-owned fixtures: `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test`
- Open XML SDK package corpus: `../ooxmlsdk-test-suite/corpus/Open-XML-SDK`
- Open XML SDK raw XML fixtures: `../ooxmlsdk-test-suite/fixtures/Open-XML-SDK`

Moved buckets:

- `Open-XML-SDK/`: former package fixtures now resolve through the suite corpus.
- `specs/`: project-owned fixtures for spec-focused coverage and the
  `known_failures.toml` file used by the `round_trip_smoke_test` compatibility
  sweep.
- `misc/`: fixtures that are intentionally outside upstream Open XML SDK assets
  and outside the spec bucket. Keep this bucket small and explicitly justified.

Round-trip policy:

- Package round-trip coverage lives in the adjacent `../ooxmlsdk-test-suite/`
  checkout. Prefer the local checkout; its remote is
  `https://github.com/KaiserY/ooxmlsdk-test-suite`.
- `../ooxmlsdk-test-suite/crates/ooxmlsdk-test/tests/round_trip.rs::round_trip_smoke_test`
  walks only `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/specs/` as a
  lightweight smoke test, and only that subtree participates in
  `specs/known_failures.toml`.
- PDF/layout parity coverage lives in
  `../ooxmlsdk-test-suite/crates/ooxmlsdk-pdf-test` and
  `../ooxmlsdk-test-suite/crates/ooxmlsdk-layout-test`.
