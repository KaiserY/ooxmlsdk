# Architecture

This document is the top-level map of the `ooxmlsdk` workspace. It is meant to
be read before diving into crate-specific code or fixture directories.

## Design Principles

- Keep the public package API close to upstream `dotnet/Open-XML-SDK` concepts.
- Keep Rust internals idiomatic even when the public surface mirrors upstream naming.
- Treat checked-in metadata and generated code as first-class repository assets.
- Keep fixture buckets explicit so upstream provenance, project-owned specs, and
  out-of-band regressions do not blur together.

## Current Implementation Strategy

At the current stage, the repository is primarily a Rust reimplementation of
upstream `Open-XML-SDK` and selected LibreOffice logic.

Reference priority depends on the subsystem:

- `ooxmlsdk`: `Open-XML-SDK` first, LibreOffice second
- `ooxmlsdk-pdf`: LibreOffice first

This has two consequences:

1. upstream behavior is the default reference model
2. upstream tests and fixtures are the default evidence for deciding whether the
   Rust behavior is correct

The near-term goal is not to invent a new document model or expand the feature
surface beyond upstream-backed behavior. The near-term goal is to faithfully
port, adapt, and harden upstream logic in Rust.

In practice:

- when implementing `ooxmlsdk` package, schema, validator, or MCE behavior,
  inspect `Open-XML-SDK` first
- when implementing `ooxmlsdk-pdf` behavior, inspect LibreOffice first
- prefer re-expressing upstream control flow and invariants with Rust types,
  enums, traits, iterators, and ownership boundaries rather than designing a
  new semantic model
- when the Rust code disagrees with upstream and there is no deliberate local
  divergence documented in the repo, assume the Rust side is incomplete until
  proven otherwise
- avoid adding novel logic just because upstream behavior is inconvenient,
  under-specified, or not yet fully understood
- if exact upstream behavior is still unclear after local inspection, narrow the
  gap with fixtures and source-backed tests instead of guessing

## Workspace Layout

The workspace currently has six crates:

- `crates/ooxmlsdk`: runtime crate and primary public API
- `crates/ooxmlsdk-build`: generator and checked-in input model logic
- `crates/ooxmlsdk-derive`: procedural macros used by generated/runtime schema types
- `crates/ooxmlsdk-pdf`: DOCX -> PDF conversion pipeline
- `crates/ooxmlsdk-pdf-test`: PDF parity helpers and fixture-based PDF assertions
- `crates/ooxmlsdk-test`: integration tests, generated round-trip harness, validators, and benches

## Runtime Architecture

`crates/ooxmlsdk` is split between generated schema/package code and a small
hand-written runtime layer.

Generated runtime output lives under:

- `crates/ooxmlsdk/src/schemas/`
- `crates/ooxmlsdk/src/deserializers/`
- `crates/ooxmlsdk/src/serializers/`
- `crates/ooxmlsdk/src/parts/`
- `crates/ooxmlsdk/src/schemas.rs`
- `crates/ooxmlsdk/src/deserializers.rs`
- `crates/ooxmlsdk/src/serializers.rs`
- `crates/ooxmlsdk/src/namespaces.rs`

Hand-written shared runtime logic lives mainly in:

- `crates/ooxmlsdk/src/common.rs`
- `crates/ooxmlsdk/src/common/`
- `crates/ooxmlsdk/src/sdk.rs`
- `crates/ooxmlsdk/src/simple_type.rs`
- `crates/ooxmlsdk/src/validator.rs`

The intended public package model is:

- typed package roots such as `WordprocessingDocument`, `SpreadsheetDocument`, and `PresentationDocument`
- typed part traversal and relationship helpers
- stable open/save/reopen flows
- feature-gated Flat OPC, validators, and MCE behavior

## Generation Pipeline

Generator inputs are committed and reviewed, not fetched at runtime.

Primary input roots:

- `data/`: upstream-derived metadata snapshots; source of truth for schema modeling
- `sdk_data/`: checked-in intermediate generator data
- `schemas/OpenPackagingConventions-XMLSchema/`: OPC schema inputs

Hand-written generator model code:

- `crates/ooxmlsdk-build/src/sdk_data/sdk_data_model.rs`

High-level flow:

1. upstream metadata in `data/` and package schemas are parsed by `ooxmlsdk-build`
2. checked-in intermediate data is written to `sdk_data/`
3. generated Rust runtime code is emitted into `crates/ooxmlsdk/src/...`
4. `ooxmlsdk-derive` provides macro support used by the generated/runtime layer

Because generated code is checked in, reviews should usually focus on:

- whether the input change is correct
- whether the generated diff matches the intended model shift
- whether format/test/clippy lanes stay green

## Feature Model

The runtime feature surface is intentionally small:

- `parts`: package-level OOXML read/write support; default feature
- `flat-opc`: Flat OPC APIs; depends on `parts`
- `mce`: Markup Compatibility and Extensibility processing; depends on `parts`
- `validators`: optional validator APIs

There is no Cargo `strict` feature. Strict OOXML behavior is tracked through
fixture behavior and compatibility coverage.

## PDF Architecture

The PDF path is separate from the core OOXML runtime:

- `crates/ooxmlsdk-pdf`: renders DOCX content into PDF
- `crates/ooxmlsdk-pdf-test`: renders committed DOCX fixtures and inspects the
  resulting PDF with PDFium/Lopdf-based helpers

This split matters:

- `ooxmlsdk-test` owns package/schema/round-trip correctness
- `ooxmlsdk-pdf-test` owns visible-output and PDF-object parity checks

Do not mix PDF rendering fixtures into package round-trip harnesses.

## Test Architecture

`crates/ooxmlsdk-test` has two different fixture-driven lanes:

1. generated round-trip tests in `tests/doc_samples.rs`
2. compatibility smoke coverage in `tests/round_trip.rs`

They have different scopes by design.

### Fixture Buckets

Package fixtures live under `test-data/ooxmlsdk-test/`:

- `Open-XML-SDK/`: copied from upstream `../Open-XML-SDK/test/DocumentFormat.OpenXml.Tests.Assets/assets/TestFiles`
- `libreoffice/`: copied from `../core` and used for package round-trip coverage
- `specs/`: project-owned fixtures grouped by domain
- `misc/`: fixtures intentionally outside upstream assets and outside spec buckets

PDF fixtures live under `test-data/ooxmlsdk-pdf-test/`:

- `libreoffice/`: fixtures copied from `../core`

### Round-Trip Coverage

Generated round-trip coverage:

- implemented by `crates/ooxmlsdk-test/build.rs`
- emits test cases into `tests/doc_samples.rs`
- walks:
  - `test-data/ooxmlsdk-test/Open-XML-SDK/`
  - `test-data/ooxmlsdk-test/libreoffice/`
  - `test-data/ooxmlsdk-test/specs/`
  - `test-data/ooxmlsdk-test/misc/`

Compatibility smoke coverage:

- implemented by `crates/ooxmlsdk-test/tests/round_trip.rs::round_trip_smoke_test`
- walks only `test-data/ooxmlsdk-test/specs/`
- uses `test-data/ooxmlsdk-test/specs/known_failures.toml`

Known-failure policy is intentionally narrow:

- `specs/known_failures.toml` does not apply to `Open-XML-SDK/`
- `specs/known_failures.toml` does not apply to `libreoffice/`
- `specs/known_failures.toml` does not apply to `misc/`
- `specs/known_failures.toml` does not apply to PDF fixtures

### Test Commands

Use the repository root for all commands below.

- `cargo build --workspace`: build all crates.
- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`: regenerate `sdk_data/` and runtime generated code from checked-in `data/` and package schemas.
- `cargo test -p ooxmlsdk-test`: fast integration lane for common runtime and package behavior.
- `cargo test --workspace`: default full test lane.
- `cargo test --workspace --no-default-features`: no-default-features lane.
- `cargo test --workspace --no-default-features --features parts`: parts lane without validators or MCE-specific behavior.
- `cargo test --workspace --no-default-features --features flat-opc`: Flat OPC lane without validators or MCE-specific behavior.
- `cargo test --workspace --no-default-features --features mce`: MCE lane without validators or Flat OPC-specific behavior.
- `cargo test -p ooxmlsdk-test --features validators`: validator-focused lane.
- `cargo fmt --all`: format.
- `cargo clippy --workspace --all-targets -- -D warnings`: default clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`: no-default-features clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings`: Office2007 parts clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features --features flat-opc -- -D warnings`: Flat OPC clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features --features mce -- -D warnings`: MCE clippy lane.
- `cargo clippy -p ooxmlsdk-test --features validators --all-targets -- -D warnings`: validator clippy lane.
- `cargo bench -p ooxmlsdk-test --bench perf`: package and XML performance benches.

**Dev Loop**

Default dev loop after generator work:

1. `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
2. `cargo fmt --all`
3. `cargo test --workspace`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo fmt --all`

Full generator/feature validation:

1. `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
2. `cargo fmt --all`
3. `cargo test --workspace`
4. `cargo test --workspace --no-default-features`
5. `cargo test --workspace --no-default-features --features parts`
6. `cargo test --workspace --no-default-features --features flat-opc`
7. `cargo test --workspace --no-default-features --features mce`
8. `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
9. `cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings`
10. `cargo clippy --workspace --all-targets --no-default-features --features flat-opc -- -D warnings`
11. `cargo clippy --workspace --all-targets --no-default-features --features mce -- -D warnings`
12. `cargo clippy --workspace --all-targets -- -D warnings`
13. `cargo fmt --all`

For runtime/doc-sample iteration, start with `cargo test -p ooxmlsdk-test`. If the change touches Flat OPC, also run Flat OPC test. If the change touches MCE behavior, also run MCE tests. Add broader lanes only when the change touches generator code, shared runtime behavior, feature gates, package behavior, or validators.

### Derive Macro Debugging

When changing `crates/ooxmlsdk-derive`, inspect at least one representative
expanded output before trusting green tests.

Example:

```sh
OOXMLSDK_DUMP_KIND=SdkType \
OOXMLSDK_DUMP_FILE=schemas/schemas_openxmlformats_org_wordprocessingml_2006_main.rs \
OOXMLSDK_DUMP_TARGET=Fonts \
cargo test -p ooxmlsdk-derive dump_context_node_expansion -- --ignored --nocapture
```

`OOXMLSDK_DUMP_FILE` is relative to `crates/ooxmlsdk/src`, and expansions are
written under `target/ooxmlsdk_macro_expanded/<Kind>/`.

### Coverage Metadata

Test documentation lives under `docs/tests/`:

- `docs/tests/ooxmlsdk-test/README.md`
- `docs/tests/ooxmlsdk-test/Open-XML-SDK/UPSTREAM_TEST_MATRIX.md`
- `docs/tests/ooxmlsdk-test/specs/README.md`
- `docs/tests/ooxmlsdk-test/misc/README.md`
- `docs/tests/ooxmlsdk-pdf-test/README.md`
- `docs/tests/ooxmlsdk-pdf-test/libreoffice/UPSTREAM_TEST_MATRIX.md`

Use these files when deciding where a new fixture belongs.

## Upstream Alignment

The repository currently aligns against two primary upstreams.

### Open XML SDK Upstream

Local checkout:

- `../Open-XML-SDK`

Use it for:

- package API and behavior for `ooxmlsdk`
- generated metadata shape
- upstream fixtures
- upstream tests and coverage intent

Key paths:

- `../Open-XML-SDK/src/DocumentFormat.OpenXml*`
- `../Open-XML-SDK/generated/DocumentFormat.OpenXml`
- `../Open-XML-SDK/data`
- `../Open-XML-SDK/test`
- `../Open-XML-SDK/test/DocumentFormat.OpenXml.Tests.Assets`

### LibreOffice Upstream

Local checkout:

- `../core`

Use it for:

- primary behavior reference for `ooxmlsdk-pdf`
- PDF rendering fixtures
- visible-output expectations
- PDF export assertions
- supplemental behavioral evidence for `ooxmlsdk` when Open XML SDK does not cover the case
- DOCX rendering regressions that are observable in emitted PDFs

Key paths:

- `../core/vcl/qa/cppunit/pdfexport/`
- `../core/vcl/qa/cppunit/pdfexport/data/`
- `../core/oox/qa/unit/`
- `../core/sw/qa/writerfilter/dmapper/`

Do not place non-`../core` PDF fixtures into `test-data/ooxmlsdk-pdf-test/libreoffice/`.

## Documentation Map

Top-level docs serve different roles:

- `README.md`: public crate overview and feature surface
- `CHANGELOG.md`: release history
- `COMPATIBILITY.md`: current parser-level compatibility matrix
- `docs/specs/`: behavior notes and format-specific references
- `docs/tests/`: fixture taxonomy and coverage mapping

When updating architecture-sensitive behavior, update the relevant document in
the same change instead of leaving the repository map stale.
