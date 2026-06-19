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

The workspace currently has seven crates:

- `crates/ooxmlsdk`: runtime crate and primary public API
- `crates/ooxmlsdk-build`: generator and checked-in input model logic
- `crates/ooxmlsdk-derive`: procedural macros used by generated/runtime schema types
- `crates/ooxmlsdk-fonts`: font discovery, fallback, and measurement helpers
- `crates/ooxmlsdk-formula`: formula parsing/evaluation support
- `crates/ooxmlsdk-layout`: document layout helpers
- `crates/ooxmlsdk-pdf`: DOCX -> PDF conversion pipeline

Focused integration tests, validators, package smoke coverage, and runtime
benches live in the adjacent `../ooxmlsdk-test-suite/crates/ooxmlsdk-test`
crate.

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

### Schema Extension Semantics

Schema extension files under `sdk_data/schema_extensions/` patch gaps in the
upstream-derived `data/` model before Rust code is generated. Keep these patches
small and source-backed. Prefer fixture or upstream evidence for every new
extension entry.

`HaveDirectXmlOtherChildren` is scoped to the current type only. It means the
current element may contain direct unknown XML children that should be preserved
as raw XML on the current struct. In generated Rust this adds:

```rust
pub xml_other_children: Vec<(usize, Box<[u8]>)>
```

Use `HaveDirectXmlOtherChildren` only when the unknown child really belongs to
the current element's own direct-child storage. Do not use it for
`mc:AlternateContent` that wraps an otherwise known child inside a repeated
choice stream.

`ChoiceEnums[].AddXmlAny` is scoped to an existing generated choice enum. It
means the parent choice stream must accept raw XML as another ordered item. In
generated Rust this adds `XmlAny` to that existing choice enum.

For example, if `RunChoice` has `"AddXmlAny": true`, then `w:CT_R/w:r` can be
generated with:

```rust
#[sdk(choice(qname = "w:CT_Drawing/w:drawing", any))]
pub run_choice: Vec<RunChoice>,
```

Use `AddXmlAny` only when an MCE wrapper, such as `mc:AlternateContent`, is
observed inside an already modeled choice stream. If the parent has a direct
child slot, such as a single repeated child list or a named sequence position,
use `HaveDirectXmlOtherChildren` on the parent instead. For a parent with only
one repeated child, `HaveDirectXmlOtherChildren` promotes that child list to a
choice stream with an `XmlAny` variant so the raw MCE wrapper can preserve its
ordered position.

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

This split matters:

- `../ooxmlsdk-test-suite/crates/ooxmlsdk-test` owns focused package/schema correctness and smoke round-trip coverage
- `../ooxmlsdk-test-suite/crates/ooxmlsdk-pdf-test` owns visible-output and PDF-object parity checks
- `../ooxmlsdk-test-suite/crates/ooxmlsdk-layout-test` owns layout parity checks
- `../ooxmlsdk-test-suite` owns corpus-scale package round-trip coverage

Do not mix PDF rendering fixtures into package round-trip harnesses.

## Test Architecture

`../ooxmlsdk-test-suite/crates/ooxmlsdk-test` keeps focused fixture-driven
package/schema lanes. Corpus-scale package round-trip coverage is maintained in
the same adjacent `../ooxmlsdk-test-suite/` checkout.

The package round-trip smoke lane is compatibility smoke coverage in
`../ooxmlsdk-test-suite/crates/ooxmlsdk-test/tests/round_trip.rs`.

This lane is intentionally narrower than the external corpus suite.

### Fixture Buckets

Package fixtures live under `../ooxmlsdk-test-suite/`:

- `corpus/Open-XML-SDK/`: copied from upstream `../Open-XML-SDK`
- `fixtures/ooxmlsdk-test/specs/`: project-owned package smoke fixtures grouped by domain
- `fixtures/ooxmlsdk-test/misc/`: fixtures intentionally outside upstream assets and outside spec buckets
- `fixtures/Open-XML-SDK/`: raw XML assets copied from upstream tests for focused assertions

The remaining `test-data/ooxmlsdk-test/libreoffice/` directory is legacy
LibreOffice package fixture staging; move it to `../ooxmlsdk-test-suite/` before
adding new LibreOffice package round-trip coverage.

LibreOffice PDF/layout parity fixtures live under
`../ooxmlsdk-test-suite/corpus/LibreOffice/`. The remaining
`test-data/ooxmlsdk-pdf-test/` directory is legacy fixture staging used by a
small number of local layout unit tests; do not add new coverage there.

### Round-Trip Coverage

Corpus round-trip coverage:

- maintained in `../ooxmlsdk-test-suite/`
- prefer the local checkout; remote: `https://github.com/KaiserY/ooxmlsdk-test-suite`
- covers external package fixture corpora such as Open-XML-SDK, LibreOffice,
  and Apache POI

Compatibility smoke coverage:

- implemented by `../ooxmlsdk-test-suite/crates/ooxmlsdk-test/tests/round_trip.rs::round_trip_smoke_test`
- walks only `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/specs/`
- uses `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/specs/known_failures.toml`

Known-failure policy is intentionally narrow:

- `specs/known_failures.toml` does not apply to `Open-XML-SDK/`
- `specs/known_failures.toml` does not apply to `libreoffice/`
- `specs/known_failures.toml` does not apply to `misc/`
- `specs/known_failures.toml` does not apply to PDF fixtures

### Test Commands

Use the repository root for all commands below.

- `cargo build --workspace`: build all crates.
- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`: regenerate `sdk_data/` and runtime generated code from checked-in `data/` and package schemas.
- `cargo test --workspace`: default full test lane.
- `cargo test --workspace --no-default-features`: no-default-features lane.
- `cargo test --workspace --no-default-features --features parts`: parts lane without validators or MCE-specific behavior.
- `cargo test --workspace --no-default-features --features flat-opc`: Flat OPC lane without validators or MCE-specific behavior.
- `cargo test --workspace --no-default-features --features mce`: MCE lane without validators or Flat OPC-specific behavior.
- `cargo fmt --all`: format.
- `cargo clippy --workspace --all-targets -- -D warnings`: default clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`: no-default-features clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings`: Office2007 parts clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features --features flat-opc -- -D warnings`: Flat OPC clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features --features mce -- -D warnings`: MCE clippy lane.
- `cd ../ooxmlsdk-test-suite && cargo test -p ooxmlsdk-test`: fast integration lane for common runtime and package behavior.
- `cd ../ooxmlsdk-test-suite && cargo test -p ooxmlsdk-test --features validators`: validator-focused lane.
- `cd ../ooxmlsdk-test-suite && cargo clippy -p ooxmlsdk-test --features validators --all-targets -- -D warnings`: validator clippy lane.
- `cd ../ooxmlsdk-test-suite && cargo bench -p ooxmlsdk-test --bench perf`: package and XML performance benches.

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

For runtime/doc-sample iteration, start with
`cd ../ooxmlsdk-test-suite && cargo test -p ooxmlsdk-test`. If the change
touches Flat OPC, also run the test-suite Flat OPC lane. If the change touches
MCE behavior, also run the test-suite MCE lane. Add broader lanes only when the
change touches generator code, shared runtime behavior, feature gates, package
behavior, or validators.

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

### Round-Trip Corpus Suite

Local checkout:

- `../ooxmlsdk-test-suite`

Remote:

- `https://github.com/KaiserY/ooxmlsdk-test-suite`

Use this suite for corpus-scale package round-trip validation instead of adding
generated corpus harnesses to this repository.

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

Do not place new PDF parity fixtures in this repository. Add LibreOffice-backed
PDF fixtures under `../ooxmlsdk-test-suite/corpus/LibreOffice/` and document
the migration in `../ooxmlsdk-test-suite/docs/ooxmlsdk-pdf-test/`.

## Documentation Map

Top-level docs serve different roles:

- `README.md`: public crate overview and feature surface
- `CHANGELOG.md`: release history
- `COMPATIBILITY.md`: current parser-level compatibility matrix
- `docs/specs/`: behavior notes and format-specific references
- `docs/tests/`: fixture taxonomy and coverage mapping

When updating architecture-sensitive behavior, update the relevant document in
the same change instead of leaving the repository map stale.
