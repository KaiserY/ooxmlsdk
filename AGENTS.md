# Working reference

For Office golden PDF progress, source references, and audit/debug commands, read
`../ooxmlsdk-test-suite/docs/ooxmlsdk-pdf-test/corpus_pdf_conv.md`.

## Ownership

This workspace owns implementation, generators, and private unit tests.
`../ooxmlsdk-test-suite` owns public integration tests, fixtures, corpora, and
benchmarks; follow its `AGENTS.md` when working there. Keep imported fixtures and
their provenance in the suite, not this implementation repository.

Office configured output is the golden target. Some older layout/PDF assertions
come from LibreOffice: establish the relevant Office behavior before changing
them. Unrelated legacy failures do not justify changing golden acceptance gates.

## Commands

Run commands from their owning workspace root, using its default `target/`.
Keep Cargo, GDB, Office, and campaign audits serial; independent read-only
investigation can continue during long commands. Do not set `CARGO_TARGET_DIR`.

Implementation workspace:

```sh
cargo fmt --all
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Test-suite workspace (select the affected packages during iteration):

```sh
cargo fmt --all
cargo test -p ooxmlsdk-layout-test
cargo test -p ooxmlsdk-pdf-test
cargo clippy --workspace --tests -- -D warnings
```

Use workspace formatting instead of standalone `rustfmt` invocations so the
checked-in configuration applies consistently. Choose tests proportionate to
the changed subsystem. Changes to shared XML/package behavior or feature gates
also need the relevant core feature and round-trip lanes documented in the suite.

## Generated code

Change generator inputs/logic rather than patching generated runtime output.
`data/` is the schema-model source; `sdk_data/` and
`schemas/OpenPackagingConventions-XMLSchema/` also supply generation inputs.

```sh
cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture
cargo fmt --all
```

For derive changes, inspect the checked-in expansion test:
`cargo test -p ooxmlsdk-derive dump_context_node_expansion -- --ignored --nocapture`.

## Handoff

Preserve unrelated dirty changes. Fix warnings at their implementation or cfg
boundary rather than suppressing them with `#[allow(...)]`. Report verification
and any unrelated failures explicitly. The user commits: do not mutate the index
or history with `git add`, `git commit`, or amend commands.
