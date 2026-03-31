# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust workspace with three crates under `crates/`:

- `crates/ooxmlsdk`: the runtime library exposed to consumers. Its public entry point is `src/lib.rs`. Generated code currently lives in `src/schemas/`, `src/deserializers/`, and `src/serializers/`. Shared parsing helpers and error types live in `src/common.rs`.
- `crates/ooxmlsdk-build`: the code generation crate. Normal repository work uses the checked-in `sdk_data/` JSON snapshots plus package XSD files under `schemas/OpenPackagingConventions-XMLSchema/`, then emits Rust source into `crates/ooxmlsdk/src/`.
- `crates/ooxmlsdk-test`: the integration-test crate. It depends only on `crates/ooxmlsdk` and verifies stable XML parse/serialize round trips using sample fixtures under `crates/ooxmlsdk-test/samples/`.

Treat the checked-in `sdk_data/` directory and `schemas/OpenPackagingConventions-XMLSchema/` as the source of truth for normal generation work in this repository. The checked-in files under `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, `crates/ooxmlsdk/src/parts/`, `crates/ooxmlsdk/src/schemas.rs`, `crates/ooxmlsdk/src/deserializers.rs`, `crates/ooxmlsdk/src/serializers.rs`, and `crates/ooxmlsdk/src/namespaces.rs` are generated artifacts.

Unlike `ofdsdk`, this repository checks in a top-level `sdk_data/` directory that is used for normal code generation. The full upstream sync path still exists in `crates/ooxmlsdk-build` and depends on an external `../../../Open-XML-SDK/data` checkout. Do not assume that path is available unless you have explicitly prepared it.

The feature graph exposes `parts` and `validators`, and the generator has code paths for them, but the current checked-in runtime crate only exports `common`, `schemas`, `deserializers`, and `serializers`. Document or change that state carefully instead of assuming `src/parts/` or `src/validators/` already exist in this repository.

## Build, Test, and Development Commands
- `cargo build --workspace`: build all workspace crates.
- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`: regenerate the checked-in Rust code in `crates/ooxmlsdk/src/` from the checked-in `sdk_data/`.
- `cargo test -p ooxmlsdk-build test_sync_sdk_data -- --ignored --nocapture`: refresh `sdk_data/` from an external Open XML SDK checkout and regenerate `crates/ooxmlsdk/src/`.
- `cargo test -p ooxmlsdk-test`: run the integration tests that exercise stable round trips for representative Wordprocessing and Presentation XML samples.
- `cargo test --workspace`: run the full workspace test suite.
- `cargo test --workspace --no-default-features`: run the workspace test suite against the Office2007 baseline feature set.
- `cargo bench -p ooxmlsdk-build --bench serde_bench`: run the serde comparison benchmark in the generator crate.
- `cargo fmt --all`: format the workspace.
- `cargo clippy --workspace --all-targets -- -D warnings`: run lints before opening a PR.
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`: run lints against the Office2007 baseline feature set.

Run commands from the repository root. Because generated code is committed, regenerate first when changing generator logic or metadata, then lint and test against the generated output, and run formatting last so the generated Rust files are also normalized before review. Before every commit or pull request that affects generation, run `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`, `cargo test --workspace`, `cargo test --workspace --no-default-features`, and `cargo fmt --all`.

Execute Cargo commands in the repository's default `target/` directory. Do not introduce alternate `CARGO_TARGET_DIR` values to bypass locks or isolate builds. Run build, generation, format, lint, and test commands sequentially rather than in parallel. If Cargo reports a lock on the artifact directory, wait for the lock to clear and then continue; assume another local tool such as VS Code may still be using the shared target directory.

## Coding Style & Naming Conventions
Follow standard Rust formatting and keep the workspace `rustfmt`-clean. Use snake_case for modules and functions, PascalCase for Rust types, and preserve the schema-derived module naming pattern already in use, for example `schemas_openxmlformats_org_wordprocessingml_2006_main.rs`.

Prefer keeping hand-written logic in `crates/ooxmlsdk-build`. Avoid editing generated files in `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, or `crates/ooxmlsdk/src/schemas.rs` unless you are also updating the generator or the source metadata and intentionally regenerating the output. Keep runtime-only helpers in `crates/ooxmlsdk/src/common.rs` small and generic.

The fixtures in `crates/ooxmlsdk-test/src/fixtures.rs` are intentionally tied back to upstream .NET Open XML SDK tests. When adding coverage, prefer representative sample XML with a traceable origin instead of ad hoc snippets.

## Testing Guidelines
The generator entry point that is used in normal repository work is `test_gen` in `crates/ooxmlsdk-build/src/lib.rs`. Run it first whenever you change generator code, checked-in metadata under `sdk_data/`, package schemas under `schemas/OpenPackagingConventions-XMLSchema/`, or feature-gated generation behavior. Review the generated diff in `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, `crates/ooxmlsdk/src/parts/`, `crates/ooxmlsdk/src/schemas.rs`, `crates/ooxmlsdk/src/deserializers.rs`, `crates/ooxmlsdk/src/serializers.rs`, and `crates/ooxmlsdk/src/namespaces.rs`.

When validating generator changes, feature-gated code, or generated schema output, cover both feature surfaces explicitly:

- `cargo test --workspace --no-default-features` for the Office2007 baseline
- `cargo test --workspace` for the default `microsoft365` feature set
- `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` for the Office2007 baseline
- `cargo clippy --workspace --all-targets -- -D warnings` for the default `microsoft365` feature set

Runtime behavior is currently covered by focused round-trip tests in `crates/ooxmlsdk-test/tests/wordprocessing.rs` and `crates/ooxmlsdk-test/tests/presentation.rs`. Add new tests close to the behavior they protect and keep assertions stable: verify both parsed fields and serialized XML where possible.

`test_sync_sdk_data` is ignored and depends on an external `Open-XML-SDK` checkout. Only run it when you are explicitly updating the repository from upstream source data and have prepared that checkout locally.

When validating work, do not overlap `cargo fmt`, `cargo test`, `cargo clippy`, or generation commands. Finish one command, capture its result, and only then start the next one. If you encounter a target directory lock, do not work around it with a new target path; wait for the existing lock to release.

## Commit & Pull Request Guidelines
Keep commit subjects short, imperative, and scoped to the affected area, for example `Regenerate spreadsheet serializer output` or `Tighten XML attribute decoding errors`.

In pull requests, include:

- a brief summary of the functional or generation change,
- confirmation that you ran `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo clippy --workspace --all-targets --no-default-features -- -D warnings`, `cargo test --workspace`, `cargo test --workspace --no-default-features`, and `cargo fmt --all`,
- notes on regenerated files when generator code, `sdk_data/`, or `schemas/OpenPackagingConventions-XMLSchema/` changed,
- explicit mention if any work depended on an external `Open-XML-SDK` checkout or the ignored `test_sync_sdk_data` path.
