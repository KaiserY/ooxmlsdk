# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust workspace with three crates under `crates/`:

- `crates/ooxmlsdk`: the runtime library exposed to consumers. Its public entry point is `src/lib.rs`. Generated code lives under `src/schemas/`, `src/deserializers/`, `src/serializers/`, `src/parts/`, plus generated module files such as `src/schemas.rs`, `src/deserializers.rs`, `src/serializers.rs`, and `src/namespaces.rs`. Shared parsing helpers and error types live in `src/common.rs`.
- `crates/ooxmlsdk-build`: the code generation crate. It owns the JSON and Rust code generation pipeline.
- `crates/ooxmlsdk-test`: the integration-test crate. It depends only on `crates/ooxmlsdk` and covers schema round trips plus package-level read/write flows using fixtures under `crates/ooxmlsdk-test/samples/` and `crates/ooxmlsdk-test/doc_samples/`.

Treat the checked-in `sdk_data/`, `data/`, and `schemas/OpenPackagingConventions-XMLSchema/` directories as committed generator inputs. The checked-in files under `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, `crates/ooxmlsdk/src/parts/`, `crates/ooxmlsdk/src/schemas.rs`, `crates/ooxmlsdk/src/deserializers.rs`, `crates/ooxmlsdk/src/serializers.rs`, and `crates/ooxmlsdk/src/namespaces.rs` are generated artifacts.

When working on particle decomposition or schema modeling, treat `data/` as the source of truth. Use `schemas/OfficeOpenXML-XMLSchema-Transitional/` and other checked-in XSDs as auxiliary references for coverage checks and real naming sources such as `xsd:group` names. Do not invent semantic names when the true source does not provide one; keep generic `Choice`/`Sequence` naming until a real XSD/data-backed name is available.

`crates/ooxmlsdk-build/src/sdk_data/sdk_data_model.rs` is part of the generator input model, not a generated artifact. Keep it in sync with `sdk_data/` when adding new schema or package-level knobs such as compatibility flags.

The repository also keeps checked-in `sdk_data/`, but the current ignored generator test still reads upstream data from `../Open-XML-SDK/data` before regenerating `sdk_data/` and runtime code. Do not assume that external checkout is available unless you have explicitly prepared it.

The runtime crate currently exports `common`, `deserializers`, `namespaces`, `parts` behind the `parts` feature, `schemas`, `serializers`, `simple_type`, and `validator` behind the `validators` feature. The public feature surface in this repository is currently `microsoft365`, `parts`, `strict`, and `validators`. The `strict` feature gates strict OOXML compatibility coverage and the corresponding feature-gated integration tests. The `validators` feature gates optional schema validator APIs and validator-focused integration tests.

## Build, Test, and Development Commands
- `cargo build --workspace`: build all workspace crates.
- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`: regenerate `sdk_data/` and the checked-in Rust code in `crates/ooxmlsdk/src/`. This currently requires an external `../Open-XML-SDK/data` checkout.
- `cargo test -p ooxmlsdk-test`: run the integration tests that exercise stable round trips for representative Wordprocessing and Presentation XML samples.
- `cargo test -p ooxmlsdk-test --features strict`: run the integration tests including strict OOXML package coverage such as `Strict01.docx`.
- `cargo test --workspace`: run the full workspace test suite.
- `cargo test --workspace --no-default-features`: run the workspace test suite with default features disabled.
- `cargo test --workspace --no-default-features --features parts`: run the workspace test suite for the Office2007-oriented `parts` surface without `microsoft365`.
- `cargo test -p ooxmlsdk-test --features validators`: run validator-focused integration tests, including file-level validate coverage.
- `cargo bench -p ooxmlsdk-build --bench serde_bench`: run the serde comparison benchmark in the generator crate.
- `cargo fmt --all`: format the workspace.
- `cargo clippy --workspace --all-targets -- -D warnings`: run lints for the default `microsoft365` feature set.
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`: run lints with default features disabled.
- `cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings`: run lints for the Office2007-oriented `parts` surface without `microsoft365`.
- `cargo clippy -p ooxmlsdk-test --features validators --all-targets -- -D warnings`: lint the validator-focused integration lane.

Run commands from the repository root. For normal validation, use this order and keep `cargo fmt --all` last:

- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
- `cargo test --workspace`
- `cargo test --workspace --no-default-features`
- `cargo test --workspace --no-default-features --features parts`
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- `cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all`

For fast iteration on runtime or doc-sample work, treat `cargo test -p ooxmlsdk-test` as the primary gate first. Defer workspace-wide, `strict`, and clippy runs unless the change touches generator code, shared runtime code, or feature-gated behavior.

When changing validator IR, validator runtime helpers, or validator-facing derive output, also run the validator-specific lane:

- `cargo test -p ooxmlsdk-test --features validators`
- `cargo clippy -p ooxmlsdk-test --features validators --all-targets -- -D warnings`

Execute Cargo commands in the repository's default `target/` directory. Do not switch `target/` directories or introduce `CARGO_TARGET_DIR`. Run generation, test, clippy, and format commands sequentially, never in parallel. If Cargo reports a lock on the artifact directory, wait for the lock to clear and then continue.

For long-running Cargo commands such as `cargo test`, `cargo clippy`, and generation flows, prefer linear execution and patient waiting over short polling loops. Start the command, let it run, and wait for completion before analyzing results unless there is a concrete reason to interrupt. Do not rely on `ps`, process-list probing, or similar background-process heuristics to decide whether Cargo is still active; sandboxing and wrapper processes can make those signals misleading. Treat Cargo's own lock messages and final exit status as the source of truth.

When checking runtime performance, prefer evaluating `cargo bench -p ooxmlsdk-test` as a whole rather than drawing conclusions from a single case. The `packages` and `xml` suites have repeatedly disagreed on `wordprocessing_document/write/parsed`, so treat that one metric as a known benchmark anomaly until it is investigated; rely on the surrounding read, write, and round-trip trends before deciding whether a performance change is real.

## Coding Style & Naming Conventions
Follow standard Rust formatting and keep the workspace `rustfmt`-clean. Use snake_case for modules and functions, PascalCase for Rust types, and preserve the schema-derived module naming pattern already in use, for example `schemas_openxmlformats_org_wordprocessingml_2006_main.rs`.

Prefer keeping hand-written logic in `crates/ooxmlsdk-build`. Avoid editing generated files in `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, or `crates/ooxmlsdk/src/schemas.rs` unless you are also updating the generator or the source metadata and intentionally regenerating the output. Keep runtime-only helpers in `crates/ooxmlsdk/src/common.rs` small and generic.

The fixtures in `crates/ooxmlsdk-test/src/fixtures.rs` are intentionally tied back to upstream .NET Open XML SDK tests. When adding coverage, prefer representative sample XML with a traceable origin instead of ad hoc snippets.

## Testing Guidelines
The generator entry point is `test_gen` in `crates/ooxmlsdk-build/src/lib.rs`. Run it first whenever you change generator code, checked-in metadata under `sdk_data/`, package schemas under `schemas/OpenPackagingConventions-XMLSchema/`, or feature-gated generation behavior, but note that it currently depends on an external `../Open-XML-SDK/data` checkout. Review the generated diff in `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, `crates/ooxmlsdk/src/parts/`, `crates/ooxmlsdk/src/schemas.rs`, `crates/ooxmlsdk/src/deserializers.rs`, `crates/ooxmlsdk/src/serializers.rs`, and `crates/ooxmlsdk/src/namespaces.rs`.

When validating generator changes, feature-gated code, or generated schema output, use this sequence:

- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
- `cargo test --workspace`
- `cargo test --workspace --no-default-features`
- `cargo test --workspace --no-default-features --features parts`
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- `cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all`

Runtime behavior is currently covered by focused round-trip tests in `crates/ooxmlsdk-test/tests/wordprocessing.rs` and `crates/ooxmlsdk-test/tests/presentation.rs`. Add new tests close to the behavior they protect and keep assertions stable: verify both parsed fields and serialized XML where possible.

Additional integration coverage also lives in `crates/ooxmlsdk-test/tests/spreadsheet.rs`, `crates/ooxmlsdk-test/tests/properties.rs`, and `crates/ooxmlsdk-test/tests/packages.rs`. Add new tests close to the behavior they protect.

Validator-focused integration coverage lives in `crates/ooxmlsdk-test/tests/validators.rs` and `crates/ooxmlsdk-test/tests/file_validators.rs`. Keep these tests behind `validators` and prefer traceable upstream Open XML SDK fixtures and assertions. For package-level validator migrations where upstream reports multiple errors, it is acceptable to assert the first Rust-side validation error when the implementation intentionally stops at first failure.

The `cargo test --workspace --no-default-features --features parts` lane is the Office2007-oriented `parts` validation surface. Keep fixtures that require Office 2010+ relationships, ChartEx, model3d, `stylesWithEffects`, Word 2013+ compatibility settings, or other `microsoft365`-gated runtime coverage out of that lane by gating the affected tests with `#[cfg(feature = "microsoft365")]`. Do not hide true Office2007 regressions behind that gate.

Strict OOXML samples may still need separate gating from the default surface. If a test genuinely depends on strict-only runtime behavior, keep it out of the Office2007 `parts` lane and document why.

`crates/ooxmlsdk-test/build.rs` classifies `doc_samples/` fixtures into `open_failure`, `open_valid`, and `round_trip`. Only promote a fixture to `round_trip` when the file-level XML diff is clean; keep schema-valid but non-round-trip samples in `open_valid` so they do not get revisited as false positives.

When adjusting `doc_samples` or package tests, gate only fixtures that are demonstrably version- or feature-specific. Prefer the smallest possible `#[cfg(...)]` around a test or assertion. Do not broadly gate failing tests until you have distinguished between a real runtime bug and a fixture that depends on disabled feature coverage.

When validating work, do not overlap `cargo fmt`, `cargo test`, `cargo clippy`, or generation commands. Finish one command, capture its result, and only then start the next one. Do not switch target directories to avoid locks. If you encounter a target directory lock, wait for the existing lock to release.

When a validation command is expected to take a long time, avoid frequent status checks that only poll for intermediate output. Prefer waiting for the command to complete, then inspect the final output and failures in one pass. This keeps execution predictable and avoids false conclusions from partial logs.

## Commit Guidelines
Keep commit subjects short, imperative, and scoped to the affected area, for example `Regenerate spreadsheet serializer output` or `Tighten XML attribute decoding errors`.

Generate a commit message when needed, but do not create a commit unless the user explicitly confirms. There is no PR-specific checklist in this repository guidance.
