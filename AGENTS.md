# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust workspace with three crates under `crates/`:

- `crates/ooxmlsdk`: the runtime library exposed to consumers. Its public entry point is `src/lib.rs`. Generated code lives under `src/schemas/`, `src/deserializers/`, `src/serializers/`, `src/parts/`, plus generated module files such as `src/schemas.rs`, `src/deserializers.rs`, `src/serializers.rs`, and `src/namespaces.rs`. Shared parsing helpers and error types live in `src/common.rs`.
- `crates/ooxmlsdk-build`: the code generation crate. It owns the JSON and Rust code generation pipeline.
- `crates/ooxmlsdk-test`: the integration-test crate. It depends only on `crates/ooxmlsdk` and covers schema round trips plus package-level read/write flows using fixtures under `crates/ooxmlsdk-test/samples/` and `crates/ooxmlsdk-test/doc_samples/`.

Treat the checked-in `sdk_data/` directory and `schemas/OpenPackagingConventions-XMLSchema/` directory as committed generator inputs. The checked-in files under `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, `crates/ooxmlsdk/src/parts/`, `crates/ooxmlsdk/src/schemas.rs`, `crates/ooxmlsdk/src/deserializers.rs`, `crates/ooxmlsdk/src/serializers.rs`, and `crates/ooxmlsdk/src/namespaces.rs` are generated artifacts.

The repository also keeps checked-in `sdk_data/`, but the current ignored generator test still reads upstream data from `../Open-XML-SDK/data` before regenerating `sdk_data/` and runtime code. Do not assume that external checkout is available unless you have explicitly prepared it.

The runtime crate currently exports `common`, `deserializers`, `namespaces`, `parts` behind the `parts` feature, `schemas`, `serializers`, and `simple_type`. The public feature surface in this repository is currently `microsoft365`, `parts`, and `strict`. The `strict` feature gates strict OOXML compatibility coverage and the corresponding feature-gated integration tests. There is no shipped `validators` module.

## Build, Test, and Development Commands
- `cargo build --workspace`: build all workspace crates.
- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`: regenerate `sdk_data/` and the checked-in Rust code in `crates/ooxmlsdk/src/`. This currently requires an external `../Open-XML-SDK/data` checkout.
- `cargo test -p ooxmlsdk-test`: run the integration tests that exercise stable round trips for representative Wordprocessing and Presentation XML samples.
- `cargo test -p ooxmlsdk-test --features strict`: run the integration tests including strict OOXML package coverage such as `Strict01.docx`.
- `cargo test --workspace`: run the full workspace test suite.
- `cargo test --workspace --no-default-features`: run the workspace test suite with default features disabled.
- `cargo bench -p ooxmlsdk-build --bench serde_bench`: run the serde comparison benchmark in the generator crate.
- `cargo fmt --all`: format the workspace.
- `cargo clippy --workspace --all-targets -- -D warnings`: run lints for the default `microsoft365` feature set.
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`: run lints with default features disabled.

Run commands from the repository root. For normal validation, use this order and keep `cargo fmt --all` last:

- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
- `cargo test --workspace`
- `cargo test --workspace --no-default-features`
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all`

Execute Cargo commands in the repository's default `target/` directory. Do not switch `target/` directories or introduce `CARGO_TARGET_DIR`. Run generation, test, clippy, and format commands sequentially, never in parallel. If Cargo reports a lock on the artifact directory, wait for the lock to clear and then continue.

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
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo fmt --all`

Runtime behavior is currently covered by focused round-trip tests in `crates/ooxmlsdk-test/tests/wordprocessing.rs` and `crates/ooxmlsdk-test/tests/presentation.rs`. Add new tests close to the behavior they protect and keep assertions stable: verify both parsed fields and serialized XML where possible.

Additional integration coverage also lives in `crates/ooxmlsdk-test/tests/spreadsheet.rs`, `crates/ooxmlsdk-test/tests/properties.rs`, and `crates/ooxmlsdk-test/tests/packages.rs`. Add new tests close to the behavior they protect.

Feature-gated strict OOXML coverage lives in `crates/ooxmlsdk-test/tests/packages.rs` behind `#[cfg(feature = "strict")]`. Keep strict-only fixtures and assertions under the `strict` feature instead of expanding the default test surface.

When validating work, do not overlap `cargo fmt`, `cargo test`, `cargo clippy`, or generation commands. Finish one command, capture its result, and only then start the next one. Do not switch target directories to avoid locks. If you encounter a target directory lock, wait for the existing lock to release.

## Commit Guidelines
Keep commit subjects short, imperative, and scoped to the affected area, for example `Regenerate spreadsheet serializer output` or `Tighten XML attribute decoding errors`.

Generate a commit message when needed, but do not create a commit unless the user explicitly confirms. There is no PR-specific checklist in this repository guidance.
