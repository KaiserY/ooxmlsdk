# Repository Guidelines

This file is the entrypoint for agents working in this repository. Keep it
short, stable, and map-like. Treat deeper docs as the source of truth.

Read first:

1. `README.md` for public API and feature surface
2. `ARCHITECTURE.md` for workspace layout, data flow, test buckets, and upstreams
3. `docs/tests/` for fixture taxonomy and coverage matrices when touching tests
4. `docs/specs/` for user-facing format notes when touching behavior or docs

## Working Style

- Start from local evidence. Use `rg` / `rg --files` first.
- Read only the files needed for the task.
- Keep summaries diff-based rather than conversation-based.
- Do not paste broad search output or large generated snippets unless asked.
- Run commands from the repository root.

## Command Rules

- Use the default `target/` directory. Do not set `CARGO_TARGET_DIR`.
- Run Cargo generation, format, test, clippy, and bench commands sequentially.
- If Cargo reports a target lock, wait for Cargo instead of probing processes.

Common commands:

- `cargo build --workspace`
- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
- `cargo test -p ooxmlsdk-test`
- `cargo test -p ooxmlsdk-test --test doc_samples -- --nocapture`
- `cargo test -p ooxmlsdk-test round_trip_smoke_test -- --nocapture`
- `cargo test --workspace`
- `cargo test -p ooxmlsdk-test --features validators`
- `cargo fmt --all`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo bench -p ooxmlsdk-test --bench perf`

## Upstream Sources

Prefer local checkouts before browsing:

- `../Open-XML-SDK`: primary upstream for package API, generated metadata, tests, and assets
- `../core`: LibreOffice upstream for PDF rendering fixtures and visible-output expectations
- `../ooxmlsdk`: sibling checkout reference when it is distinct from this repository
- `../open-xml-docs`: Microsoft Learn Open XML docs and samples

Use GitHub only when the local checkout is missing or clearly stale.

## Test Fixture Boundaries

- `test-data/ooxmlsdk-test/Open-XML-SDK/`: upstream Open XML SDK fixture assets
- `test-data/ooxmlsdk-test/specs/`: project-owned spec fixtures
- `test-data/ooxmlsdk-test/misc/`: non-upstream, non-spec package fixtures
- `test-data/ooxmlsdk-pdf-test/libreoffice/`: LibreOffice-derived DOCX -> PDF fixtures

Important:

- `crates/ooxmlsdk-test/tests/doc_samples.rs` covers `Open-XML-SDK/`, `specs/`, and `misc/`
- `crates/ooxmlsdk-test/tests/round_trip.rs::round_trip_smoke_test` scans only `test-data/ooxmlsdk-test/specs/`
- `test-data/ooxmlsdk-test/specs/known_failures.toml` applies only to `specs/`
- `test-data/ooxmlsdk-pdf-test/` is not part of package round-trip coverage

## Generated Code

- Treat `data/`, `sdk_data/`, and `schemas/OpenPackagingConventions-XMLSchema/` as generator inputs.
- `crates/ooxmlsdk-build/src/sdk_data/sdk_data_model.rs` is hand-written input model code.
- Treat `data/` as the source of truth for schema modeling; use checked-in XSDs as auxiliary references.
- Do not invent semantic names when upstream data/XSDs do not provide them; keep generic names such as `Choice` and `Sequence` until a source-backed name exists.
- Avoid editing generated runtime files directly unless you are intentionally changing generator inputs and regenerating output.
- After regeneration, run `cargo fmt --all` before reviewing diffs.

## Editing Guidance

- Keep hand-written logic in `crates/ooxmlsdk-build`, `crates/ooxmlsdk-derive`, `crates/ooxmlsdk-pdf`, or small runtime helpers.
- Preserve upstream Open XML SDK concepts in the public package API:
  `new`, `new_with_settings`, `new_from_file`, `save`, relationship helpers, and typed part traversal.
- Do not expose raw storage or internal caches just to match upstream internals.

## Testing Guidance

Place tests near the behavior they protect:

- schema/simple XML: `wordprocessing.rs`, `presentation.rs`, `spreadsheet.rs`, `properties.rs`, `simple_types.rs`
- package/parts and Flat OPC: `packages.rs`
- MCE: `packages.rs` or `markup_compatibility_calibration.rs`
- validators: `validators.rs` and `file_validators.rs`
- fixture-generated round trips: `doc_samples.rs`

Prefer upstream-derived assertions and add `// Source:` comments on upstream-based tests.

## Documentation Guidance

When updating docs, start with checked-in repository docs before browsing:

- `README.md`
- `CHANGELOG.md`
- `COMPATIBILITY.md`
- `ARCHITECTURE.md`
- `docs/specs/`
- `docs/tests/`

## Commit Guidance

- Keep commit subjects short, imperative, and scoped.
- Base commit messages on repository state, not the latest chat turn.
- Inspect `git status --short`, `git diff --stat`, and relevant diffs before writing a message.
- Do not create a commit unless explicitly asked.
