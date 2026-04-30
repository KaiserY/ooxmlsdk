# Repository Guidelines

## Agent Harness
Start from local evidence. Use `rg`/`rg --files` first, read only the files needed for the task, and keep summaries diff-based rather than conversation-based. Do not paste large generated snippets or broad search output back into the conversation unless the user asks.

Run commands from the repository root. Cargo generation, format, test, clippy, and bench commands must run sequentially in the default `target/` directory; do not set `CARGO_TARGET_DIR`. If Cargo reports a target lock, wait for Cargo rather than probing processes.

This Rust workspace has four crates:

- `crates/ooxmlsdk`: runtime crate and public API. Generated schema/deserializer/serializer/parts output lives under `src/schemas/`, `src/deserializers/`, `src/serializers/`, `src/parts/`, plus `src/schemas.rs`, `src/deserializers.rs`, `src/serializers.rs`, and `src/namespaces.rs`. Shared runtime helpers live in `src/common.rs`.
- `crates/ooxmlsdk-build`: generator and checked-in input model code.
- `crates/ooxmlsdk-derive`: procedural macros used by generated/runtime schema types.
- `crates/ooxmlsdk-test`: integration tests, fixtures, doc samples, upstream coverage matrix, and performance benches.

Runtime features are `microsoft365`, `parts`, and `validators`; defaults are `microsoft365` and `parts`. There is no Cargo `strict` feature. Strict OOXML is handled as fixture/document behavior.

## Upstream Alignment
This project broadly mirrors the API, package model, schema behavior, validators, fixtures, and tests of upstream `dotnet/Open-XML-SDK`.

When comparing behavior, prefer the local checkout at `../Open-XML-SDK` if it exists. Useful upstream paths include:

- `../Open-XML-SDK/src/DocumentFormat.OpenXml*` for public API and packaging behavior.
- `../Open-XML-SDK/generated/DocumentFormat.OpenXml` for generated types.
- `../Open-XML-SDK/data` for upstream-shaped metadata.
- `../Open-XML-SDK/test` and `../Open-XML-SDK/test/DocumentFormat.OpenXml.Tests.Assets` for tests and fixtures.

If the local checkout is missing or clearly stale and exact upstream behavior matters, use the GitHub source as a fallback reference. Keep Rust APIs idiomatic, but preserve upstream concepts such as document/package constructors, save/open flows, part containers, relationship helpers, and part traversal. Do not expose raw storage, relationship sets, generated factories, or internal caches just to match an upstream internal test.

Use `crates/ooxmlsdk-test/UPSTREAM_TEST_MATRIX.md` to decide whether upstream behavior is already covered, missing, partial, blocked by API, or not applicable. Add or update `// Source:` comments on upstream-derived tests so coverage remains auditable.

## Generated Data
Treat checked-in `data/`, `sdk_data/`, and `schemas/OpenPackagingConventions-XMLSchema/` as generator inputs. `crates/ooxmlsdk-build/src/sdk_data/sdk_data_model.rs` is hand-written input model code, not generated output.

For schema modeling and particle decomposition, treat `data/` as source of truth and use checked-in XSDs such as `schemas/OfficeOpenXML-XMLSchema-Transitional/` only as auxiliary references. Do not invent semantic names when upstream data/XSDs do not provide them; keep generic `Choice`/`Sequence` names until a real source-backed name exists.

Avoid editing generated runtime files directly unless also changing generator/input data and intentionally regenerating output. After `test_gen`, run `cargo fmt --all` before reviewing diffs; do not revert generated churn before checking the formatted diff.

## Commands
- `cargo build --workspace`: build all crates.
- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`: regenerate `sdk_data/` and runtime generated code from checked-in `data/` and package schemas.
- `cargo test -p ooxmlsdk-test`: fast integration lane for common runtime and package behavior.
- `cargo test --workspace`: default full test lane.
- `cargo test --workspace --no-default-features`: no-default-features lane.
- `cargo test --workspace --no-default-features --features parts`: Office2007-oriented parts lane without `microsoft365`.
- `cargo test -p ooxmlsdk-test --features validators`: validator-focused lane.
- `cargo fmt --all`: format.
- `cargo clippy --workspace --all-targets -- -D warnings`: default clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`: no-default-features clippy lane.
- `cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings`: Office2007 parts clippy lane.
- `cargo clippy -p ooxmlsdk-test --features validators --all-targets -- -D warnings`: validator clippy lane.
- `cargo bench -p ooxmlsdk-test --bench perf`: package and XML performance benches.

Default dev loop after generator work:

- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
- `cargo fmt --all`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all`

Full generator/feature validation:

- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
- `cargo fmt --all`
- `cargo test --workspace`
- `cargo test --workspace --no-default-features`
- `cargo test --workspace --no-default-features --features parts`
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- `cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all`

For runtime/doc-sample iteration, start with `cargo test -p ooxmlsdk-test`. Add broader lanes only when the change touches generator code, shared runtime behavior, feature gates, package behavior, or validators.

## Derive Macro Debugging
When changing `crates/ooxmlsdk-derive` macro logic, dump at least one representative expansion and inspect the generated code shape before trusting tests. Use the ignored helper from the repository root:

```sh
OOXMLSDK_DUMP_KIND=SdkType OOXMLSDK_DUMP_FILE=schemas/schemas_openxmlformats_org_wordprocessingml_2006_main.rs OOXMLSDK_DUMP_TARGET=Fonts cargo test -p ooxmlsdk-derive dump_context_node_expansion -- --ignored --nocapture
```

`OOXMLSDK_DUMP_FILE` is relative to `crates/ooxmlsdk/src`, and the expansion is written under `target/ooxmlsdk_macro_expanded/<Kind>/`. Choose targets that exercise both the changed path and a nearby non-target path; for example, after changing `xml_other_children` ordering, compare a single repeated-child type such as `Fonts` with a multi-field type such as `Numbering`.

## Testing Rules
Place tests near the behavior they protect:

- Schema/simple XML round trips: `wordprocessing.rs`, `presentation.rs`, `spreadsheet.rs`, `properties.rs`, or `simple_types.rs`.
- Package/parts behavior: `packages.rs`, using public APIs such as `parts`, `get_all_parts`, `get_part_by_id`, `get_parts_of_type`, relationship helpers, paths, content/data helpers, and saved package contents.
- Validators: `validators.rs` and `file_validators.rs`, behind `validators`.
- Doc samples: `doc_samples.rs` and `crates/ooxmlsdk-test/build.rs`.

Prefer upstream-derived fixtures and assertions. For package-level validator migrations where upstream reports multiple errors, asserting the first Rust-side validation error is acceptable if the implementation intentionally stops at first failure.

The Office2007 parts lane must not include fixtures that require Office 2010+ relationships, ChartEx, model3d, `stylesWithEffects`, Word 2013+ settings, or other `microsoft365`-gated coverage. Gate only the smallest affected test/assertion with `#[cfg(feature = "microsoft365")]`.

`crates/ooxmlsdk-test/build.rs` classifies `doc_samples/` as `open_failure`, `open_valid`, or `round_trip`. Promote a sample to `round_trip` only when the file-level XML diff is clean; keep schema-valid but non-round-trip samples in `open_valid`.

## Code Style
Keep Rust `rustfmt`-clean. Use snake_case for modules/functions, PascalCase for types, and preserve schema-derived module names such as `schemas_openxmlformats_org_wordprocessingml_2006_main.rs`.

Keep hand-written logic in `crates/ooxmlsdk-build`, `crates/ooxmlsdk-derive`, or small generic runtime helpers. For the `parts` API, keep the public surface aligned with upstream Open XML SDK concepts and established constructors/save entry points such as `new`, `new_with_settings`, `new_from_file`, and `save`; configure eager/lazy loading through `OpenSettings.open_mode`.

When benchmarking, evaluate `cargo bench -p ooxmlsdk-test --bench perf` as a whole. The `packages` and `xml` suites have historically disagreed on `wordprocessing_document/write/parsed`; treat that single metric as a known anomaly unless investigated with surrounding trends.

## Commit Guidelines
Keep commit subjects short, imperative, and scoped, for example `Regenerate spreadsheet serializer output` or `Tighten XML attribute decoding errors`.

When generating a commit message, base it on repository state, not the latest chat turn. Inspect `git status --short`, `git diff --stat`, and relevant `git diff` hunks. If changes are staged, also inspect `git diff --cached --stat` and `git diff --cached`. Do not prefix the copyable commit message with explanatory text such as whether it covers staged or unstaged changes; if that context is useful, place it after the commit message under a separate non-copyable note.

Summarize the coherent change set visible in the diff. Distinguish documentation-only edits from code, metadata, generated output, fixtures, and tests. Mention generated churn only when intended. Do not fold unrelated worktree edits into the message unless the user asks for a message covering all changes.

Generate a commit message when needed, but do not create a commit unless the user explicitly confirms.
