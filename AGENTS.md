# Repository Guidelines

This file is the entry point for agents working in the implementation
repository. Keep it short and map-like; deeper documents and the adjacent test
workspace are the sources of truth.

Read first:

1. `README.md` for the public API and feature surface.
2. `ARCHITECTURE.md` for crate boundaries, generated-code flow, and test ownership.
3. `../ooxmlsdk-test-suite/AGENTS.md` before changing or running integration,
   corpus, rendering, or benchmark tests.
4. `../ooxmlsdk-test-suite/docs/` for current upstream coverage matrices.
5. `docs/specs/` for user-facing format and behavior notes.

## Repository Boundary

This repository owns the Rust implementation, generator, generated code, and
implementation-local unit tests. The adjacent `../ooxmlsdk-test-suite` is the
primary test workspace and owns focused public-API integration tests, imported
fixtures and corpora, round-trip tests, rendering/layout parity tests, and
benchmarks.

Put a test in this repository only when it directly tests private crate logic,
generator behavior, derive expansion, or a small implementation-local unit.
Put public behavior and fixture-backed tests in the matching test-suite crate.
Do not add corpus fixtures to this repository.

## Implementation Strategy

The current phase rewrites upstream behavior in idiomatic Rust while reusing
upstream source, tests, and fixture evidence:

- for `ooxmlsdk`, use `../Open-XML-SDK` as the primary reference and
  `../core` (LibreOffice) as supplemental evidence;
- for `ooxmlsdk-pdf`, layout, formula, and visible-output behavior, use the
  subsystem mappings in `ARCHITECTURE.md`; LibreOffice is the primary PDF and
  layout reference;
- prefer translating source-backed behavior over inventing new behavior;
- treat unexplained differences from upstream as gaps to investigate;
- do not broaden the public feature surface without an explicit requirement.

## Working Style

- Start from local evidence with `rg` and `rg --files`.
- Run commands from the root of the repository they belong to.
- Read only the files needed for the task and summarize diffs, not exploration.
- Preserve unrelated changes in a dirty worktree.
- Do not silence warnings with `#[allow(...)]` or `#![allow(...)]`; fix the
  implementation, dead code, or cfg boundary.
- Do not run `git add`, `git commit`, `git commit --amend`, or other index or
  history mutations. When asked whether changes are ready, report verification
  and suggest a commit subject.

## Cargo Discipline

Cargo generation, format, check, test, clippy, and bench commands must run
sequentially in the default `target/` directory of the current repository. Do
not set `CARGO_TARGET_DIR` and never start a second Cargo command while one is
running. Let long commands finish; if Cargo waits on a target lock, wait rather
than probing processes or launching competing commands.

Never create an ad hoc Cargo project, temporary manifest, or throwaway crate for
inspection or debugging. Use checked-in tests and binaries. Ask before adding
temporary instrumentation.

## Commands: Implementation Repository

- `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`: regenerate
  `sdk_data/` and generated runtime code from checked-in inputs.
- `cargo fmt --all`: format the implementation workspace.
- `cargo test --workspace`: run implementation-local tests with default features.
- `cargo test --workspace --no-default-features`: test the minimal feature shape.
- `cargo test --workspace --no-default-features --features parts`: test package APIs.
- `cargo test --workspace --no-default-features --features flat-opc`: test Flat OPC.
- `cargo test --workspace --no-default-features --features mce`: test MCE.
- `cargo clippy --workspace --all-targets -- -D warnings`: lint the default shape.
- Add the same `--no-default-features` and feature selections above when a
  change affects feature gates.

The implementation workspace has no `ooxmlsdk-test` package and no primary
integration benchmark target. Commands for those belong to the adjacent suite.

## Commands: Test Suite

Run these from `../ooxmlsdk-test-suite` and follow its `AGENTS.md`:

For changes to the core `ooxmlsdk` runtime or generated schemas, run this
focused feature matrix in order. Do not substitute the whole test-suite
workspace: it contains independent EMF, OLE/CFB, fonts, formula, layout, and PDF
projects with additional sibling path dependencies.

1. `cargo test -p ooxmlsdk-test --no-default-features`: schema, XML, properties,
   and simple-type behavior without package APIs.
2. `cargo test -p ooxmlsdk-test`: default `parts` lane and focused package tests.
3. `cargo test -p ooxmlsdk-test --features flat-opc`: default parts plus Flat OPC.
4. `cargo test -p ooxmlsdk-test --features mce`: default parts plus MCE package
   and calibration tests.
5. `cargo test -p ooxmlsdk-test --features validators`: default parts plus
   schema and package validator tests.

After that matrix, package-fidelity changes must run these three generated
round-trip lanes, in order. They are ignored by default and each command runs
one upstream corpus:

1. `cargo test -p ooxmlsdk-roundtrip-tests --test apache_poi_roundtrip -- --ignored`
2. `cargo test -p ooxmlsdk-roundtrip-tests --test libreoffice_roundtrip -- --ignored`
3. `cargo test -p ooxmlsdk-roundtrip-tests --test open_xml_sdk_roundtrip -- --ignored`

Stop at the first failure when the task asks for failure analysis; do not start
later feature or corpus lanes after a failure.

Run other suite crates only when their subsystem is affected:

- `cargo test -p ooxmlsdk-formula-test`: formula parser/evaluator/import tests.
- `cargo test -p ooxmlsdk-fonts-test`: font and font/layout integration tests.
- `cargo test -p ooxmlsdk-layout-test`: DOCX, PPTX, and XLSX layout parity tests.
- `cargo test -p ooxmlsdk-pdf-test`: PDF rendering and object/visible-output tests.
- `cargo bench -p ooxmlsdk-bench`: run package and XML benchmark targets.

Test-suite infrastructure or release-wide work may additionally use
`cargo check --workspace --tests`, `cargo test --workspace`, and
`cargo clippy --workspace --tests -- -D warnings`, but these are not the core
`ooxmlsdk` validation lane. Run
`cargo test -p ooxmlsdk-corpus-test-support --test float_rules_sync -- --ignored`
only when schema float normalization rules change.

Use the smallest relevant suite crate during iteration. Run the complete
`ooxmlsdk-test` matrix and three round-trip lanes when changing generated
schemas, shared runtime XML/package behavior, or package fidelity.

## Generated Code

- `data/`, `sdk_data/`, and `schemas/OpenPackagingConventions-XMLSchema/` are
  generator inputs; `data/` is the schema-model source of truth.
- `crates/ooxmlsdk-build/src/sdk_data/sdk_data_model.rs` is hand-written input
  model code.
- Runtime files under `crates/ooxmlsdk/src/{schemas,deserializers,serializers,parts}/`
  and their generated module files are generated artifacts.
- Change generator inputs or generator logic, regenerate, then run
  `cargo fmt --all`; do not patch generated runtime output as a substitute.
- Keep generic schema names such as `Choice` and `Sequence` when upstream data
  or XSDs provide no semantic name.

## Derive Expansion

After changing `crates/ooxmlsdk-derive`, run the checked-in ignored expansion
test before deciding the implementation is correct:

```sh
cargo test -p ooxmlsdk-derive dump_context_node_expansion -- --ignored --nocapture
```

Select a target with `OOXMLSDK_DUMP_KIND`, `OOXMLSDK_DUMP_FILE`, and
`OOXMLSDK_DUMP_TARGET`. Output is written under
`target/ooxmlsdk_macro_expanded/`. Keep expansions direct and predictable; do
not create external macro-dump crates.

## Upstream And Fixture Map

- `../Open-XML-SDK`: package API, schema metadata, validators, tests, and assets.
- `../core`: LibreOffice source and primary PDF/layout visible-output evidence.
- `../ooxmlsdk-test-suite`: all focused integration tests, corpora, benchmarks,
  and test coverage documentation.
- `../open-xml-docs`: Microsoft Open XML documentation and samples.

Within the test suite:

- `crates/ooxmlsdk-test/` owns focused core runtime integration tests;
- `crates/ooxmlsdk-{fonts,formula,layout,pdf}-test/` own subsystem tests;
- `crates/ooxmlsdk-roundtrip-tests/` and
  `crates/ooxmlsdk-corpus-test-support/` own package corpus round trips;
- `crates/ooxmlsdk-bench/` owns runtime benchmarks;
- `corpus/` stores imported package corpora and corpus-local manifests;
- `fixtures/` stores focused, provenance-specific test inputs.

Follow `../ooxmlsdk-test-suite/AGENTS.md` for corpus manifests, ignored-test
semantics, locks, licensing, and coverage-report updates.
