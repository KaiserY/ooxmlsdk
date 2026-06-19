# Repository Guidelines

This file is the entrypoint for agents working in this repository. Keep it
short, stable, and map-like. Treat deeper docs as the source of truth.

Read first:

1. `README.md` for public API and feature surface
2. `ARCHITECTURE.md` for workspace layout, data flow, test buckets, and upstreams
3. `docs/tests/` for fixture taxonomy and coverage matrices when touching tests
4. `docs/specs/` for user-facing format notes when touching behavior or docs

## Project Stage

The current phase of this repository is focused on rewriting upstream
`Open-XML-SDK` and LibreOffice logic in Rust while reusing as much upstream
test coverage and fixture evidence as possible.

Priority by subsystem:

- `ooxmlsdk`: use `Open-XML-SDK` as the primary reference and LibreOffice as a secondary reference
- `ooxmlsdk-pdf`: use LibreOffice as the primary reference

Until the implementation reaches a higher maturity level:

- prefer translating upstream behavior into idiomatic Rust over inventing new behavior
- prefer upstream tests, fixtures, and source code over local guesswork
- for `ooxmlsdk`, check `Open-XML-SDK` first and use LibreOffice mainly for supplemental evidence
- for `ooxmlsdk-pdf`, check LibreOffice first
- when behavior is unclear, inspect upstream first instead of inferring a new rule
- do not broaden the feature surface or add novel logic unless the task explicitly requires it
- treat unexplained behavioral differences from upstream as bugs or gaps to investigate, not as opportunities to design a new model

## Working Style

- Start from local evidence. Use `rg` / `rg --files` first.
- Read only the files needed for the task.
- Keep summaries diff-based rather than conversation-based.
- Do not paste broad search output or large generated snippets unless asked.
- Run commands from the repository root.
- Do not fix clippy or compiler warnings by adding `#[allow(...)]` or `#![allow(...)]`; remove dead code, tighten cfgs, or improve the implementation instead.
- Cargo generation, format, test, clippy, and bench commands must run sequentially in the default `target/` directory; do not set `CARGO_TARGET_DIR`.
- This repository has long Cargo build/test times. After starting any Cargo command, let it run to completion and wait for the final result.
- Never start a second Cargo command while another Cargo command is still running, even for a quick verification, retry, status check, or no-op probe.
- While a Cargo command is running, do not launch other repository commands just to inspect progress. Do not poll with extra Cargo invocations. If output is quiet for a while, keep waiting.
- After starting a Cargo command, wait for the background command to return before doing any other repository work. Do not interrupt yourself to inspect files, diffs, or partial results while that Cargo command is still running; only look at the result after the command exits.
- For long-running Cargo commands, use the longest practical command wait interval available. If the command returns a background session, poll with long intervals, preferably several minutes, so the terminal does not emit frequent intermediate wait messages.
- Do not send progress commentary while waiting for Cargo. Report only after the Cargo command exits, unless the user explicitly asks for a status update.
- If Cargo reports a target lock or another Cargo process is already running, do not probe processes or start competing commands; just wait for the active Cargo command to finish and then continue.
- If Cargo reports a target lock, wait for Cargo rather than probing processes.
- Never create ad hoc Cargo projects, temporary manifests, or throwaway crates
  for inspection, reproduction, debugging, or code execution. This ban applies
  everywhere: `/tmp`, other workspace directories, sibling checkouts, nested
  repository subdirectories, and any other path.
- Do not work around the ban by generating temporary Cargo files, helper
  projects, or one-off crates outside the repository. Analyze with existing
  repository commands, checked-in fixtures, and existing tests only.
- If inspecting runtime output requires instrumentation, prefer an existing
  test or existing repository binary. Ask the user before adding temporary
  debugging code.

## Commands

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

### Dev Loop

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

### Macro Expansion Checks

When changing `crates/ooxmlsdk-derive`, dump macro expansion with the existing
ignored test before deciding the implementation is correct:

- `cargo test -p ooxmlsdk-derive dump_context_node_expansion -- --ignored --nocapture`

The dump test expands one derive target at a time. Select the target with
environment variables:

- `OOXMLSDK_DUMP_KIND`: one of `SdkEnum`, `SdkType`, `SdkChoice`, `SdkPart`, or
  `SdkPackage`; defaults to `SdkPart`
- `OOXMLSDK_DUMP_FILE`: runtime source file under `crates/ooxmlsdk/src/`;
  defaults to `parts/main_document_part.rs`
- `OOXMLSDK_DUMP_TARGET`: item name to expand; defaults to `MainDocumentPart`

Examples:

- `OOXMLSDK_DUMP_KIND=SdkType OOXMLSDK_DUMP_FILE=schemas/schemas_microsoft_com_office_word_2010_wordml.rs OOXMLSDK_DUMP_TARGET=Shadow cargo test -p ooxmlsdk-derive dump_context_node_expansion -- --ignored --nocapture`
- `OOXMLSDK_DUMP_KIND=SdkChoice OOXMLSDK_DUMP_FILE=schemas/schemas_microsoft_com_office_word_2010_wordml.rs OOXMLSDK_DUMP_TARGET=ShadowChoice cargo test -p ooxmlsdk-derive dump_context_node_expansion -- --ignored --nocapture`

The dump writes expanded Rust under `target/ooxmlsdk_macro_expanded/`. Inspect
the relevant expanded file and keep generated code direct, predictable, and
cheap: prefer static decisions from generator metadata, avoid redundant runtime
branches, avoid duplicate parsing/writing paths, and remove helper code that no
longer appears in expansion. Do not add temporary macro-dump crates or external
tools.

## Upstream Sources

Prefer local checkouts before browsing:

- `../Open-XML-SDK`: primary upstream for package API, generated metadata, tests, and assets
- `../core`: LibreOffice upstream for PDF rendering fixtures and visible-output expectations
- `../ooxmlsdk-test-suite`: package round-trip corpus suite for Open-XML-SDK, LibreOffice, Apache POI, and other external fixture sets; remote: `https://github.com/KaiserY/ooxmlsdk-test-suite`
- `../ooxmlsdk`: sibling checkout reference when it is distinct from this repository
- `../open-xml-docs`: Microsoft Learn Open XML docs and samples

Use GitHub only when the local checkout is missing or clearly stale.

## Test Fixture Boundaries

Important:

- full package round-trip corpus coverage belongs in `../ooxmlsdk-test-suite`, not in this repository
- focused package fixtures live in `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test`
- LibreOffice package/PDF/layout corpora live in `../ooxmlsdk-test-suite/corpus/LibreOffice`
- do not add new fixture corpus files under this repository

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
- In implementation work, rewrite upstream logic with Rust language features and Rust library structure, but do not replace upstream semantics with speculative local designs.
- Do not expose raw storage or internal caches just to match upstream internals.

## Testing Guidance

Place tests near the behavior they protect:

- schema/simple XML: `wordprocessing.rs`, `presentation.rs`, `spreadsheet.rs`, `properties.rs`, `simple_types.rs`
- package/parts and Flat OPC: `packages.rs`
- MCE: `packages.rs` or `markup_compatibility_calibration.rs`
- validators: `validators.rs` and `file_validators.rs`
- corpus-scale package round trips: `../ooxmlsdk-test-suite`

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
- Do not run `git add`, `git commit`, `git commit --amend`, or other git index/history update commands. The user creates commits.
- When the user asks whether changes can be committed, report the verification status and provide a suggested commit message instead of staging or committing.
