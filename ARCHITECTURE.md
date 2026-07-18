# Architecture

This document maps the implementation workspace, its generated-code pipeline,
and its boundary with the adjacent `ooxmlsdk-test-suite` workspace.

## System Boundary

Development is split across two sibling repositories:

| Repository | Owns |
| --- | --- |
| `ooxmlsdk` | Published implementation crates, generator inputs and output, derive macros, and implementation-local unit tests |
| `ooxmlsdk-test-suite` | Focused public-API integration tests, upstream fixtures, package corpora, round-trip harnesses, layout/PDF parity tests, and benchmarks |

The implementation repository should stay usable and publishable without its
large fixture corpora. The test suite depends on the implementation crates by
sibling path during local development and is the primary place to prove public
behavior. A bug fix commonly changes code here and adds or updates its
regression test there.

## Reference Model

The project reimplements established behavior rather than defining a new OOXML
model. Reference priority is subsystem-specific:

| Subsystem | Primary evidence | Secondary evidence |
| --- | --- | --- |
| Core package, schema, MCE, Flat OPC, validators | Open XML SDK | LibreOffice and producer files |
| Formula parsing/evaluation | LibreOffice and imported formula corpora | Open XML SDK/package semantics |
| Fonts and layout | LibreOffice | OOXML producer files and focused Rust assertions |
| PDF rendering/export | LibreOffice | PDF object and rasterized-output comparisons |

Use local checkouts first: `../Open-XML-SDK` and `../core`. When local Rust
behavior differs without a documented intentional divergence, investigate the
Rust implementation as incomplete until source or fixture evidence says
otherwise.

## Implementation Workspace

The root workspace contains seven published crates:

- `crates/ooxmlsdk`: generated OOXML schema types, XML serialization, package
  APIs, relationships, Flat OPC, MCE, and validators.
- `crates/ooxmlsdk-build`: metadata/schema ingestion and Rust code generation.
- `crates/ooxmlsdk-derive`: derives used by generated and hand-written runtime types.
- `crates/ooxmlsdk-fonts`: font discovery, fallback, and measurement.
- `crates/ooxmlsdk-formula`: spreadsheet formula parsing and evaluation.
- `crates/ooxmlsdk-layout`: document layout models and layout pipelines.
- `crates/ooxmlsdk-pdf`: PDF rendering built on layout and graphics dependencies.

The dependency direction is broadly:

```text
data + sdk_data + OPC XSDs
            |
            v
     ooxmlsdk-build -----> generated ooxmlsdk runtime
                                 |
                                 +--> fonts / formula
                                 |          |
                                 +----------+--> layout --> pdf

ooxmlsdk-derive ----------------> generated/runtime types
```

Not every support crate depends directly on `ooxmlsdk`; keep lower-level font
and formula logic independently testable where the current manifests permit it.

## Core Runtime

`crates/ooxmlsdk` combines generated schema/package code with a small
hand-written runtime.

Generated output includes:

- `crates/ooxmlsdk/src/schemas/`
- `crates/ooxmlsdk/src/deserializers/`
- `crates/ooxmlsdk/src/serializers/`
- `crates/ooxmlsdk/src/parts/`
- generated top-level module and namespace files beside those directories

Hand-written shared logic lives mainly in `common.rs`, `common/`, `sdk.rs`,
`simple_type.rs`, and `validator.rs`. The public model exposes typed package
roots, typed part traversal, relationship helpers, and open/save/reopen flows.
Raw storage, generated factories, and caches remain implementation details.

### Runtime Features

- `parts` is the default and enables package-level OOXML read/write support.
- `flat-opc` depends on `parts` and adds Flat OPC conversion.
- `mce` depends on `parts` and adds Markup Compatibility processing.
- `validators` adds optional validator APIs.

There is no `strict` Cargo feature. Strict OOXML compatibility is behavior
covered by tests and fixtures.

## Generation Pipeline

Generator inputs are committed; normal generation does not fetch upstream data.

- `data/`: upstream Open XML SDK metadata snapshot and source of truth for
  schema modeling.
- `sdk_data/`: checked-in intermediate generator data and schema extensions.
- `schemas/OpenPackagingConventions-XMLSchema/`: OPC XSD inputs.
- `crates/ooxmlsdk-build/src/sdk_data/sdk_data_model.rs`: hand-written model
  consumed by generation.

The flow is:

1. `ooxmlsdk-build` reads metadata and OPC schemas.
2. It refreshes intermediate `sdk_data/` where applicable.
3. It emits checked-in Rust into `crates/ooxmlsdk/src/`.
4. `ooxmlsdk-derive` supplies compile-time behavior used by emitted types.

Review generator changes from input to output: validate the source-backed model
change, regenerate, format, then inspect the generated diff. Generated files
are reviewable artifacts, not the normal hand-edit surface.

### Schema Extension Semantics

Files under `sdk_data/schema_extensions/` patch gaps in upstream metadata. Keep
entries small and backed by an upstream schema, source test, or real fixture.

Represent observed MCE positions statically. Use `AlternateContent` to add a
typed `mc:AlternateContent` slot at a known parent position, and declare the
parent fields accepted from that slot with its non-empty `Children` list. A
parent with one slot gets `alternate_content`; multiple slots are numbered
from `alternate_content_0` in schema order. MCE processing rejects a selected
branch child that is outside the slot's declared field set.

Use `AlternateContentChoice` when a repeated direct child and
`mc:AlternateContent` share one ordered stream. Use `AddChoice` to model a
previously untyped ordered child stream, and add a typed `AlternateContent`
variant to an existing choice enum when the wrapper occurs there. Raw
`XmlAny` remains only where the source schema itself declares wildcard content;
schema extensions must not add wildcard fallbacks for MCE.

## Test Workspace

`../ooxmlsdk-test-suite` is a separate Cargo workspace. Its current OOXML
responsibilities are:

| Crate | Responsibility |
| --- | --- |
| `ooxmlsdk-test` | Focused core package, schema, XML, Flat OPC, MCE, and validator integration tests |
| `ooxmlsdk-fonts-test` | Font backend and OOXML font/layout integration |
| `ooxmlsdk-formula-test` | Formula parsing, printing, evaluation, FODS/XLSX import, and shared formulas |
| `ooxmlsdk-layout-test` | DOCX, PPTX, and XLSX layout parity |
| `ooxmlsdk-pdf-test` | PDF export, metafile handling, PDF-object checks, and mapped visible-output fixtures |
| `ooxmlsdk-corpus-test-support` | Shared package round-trip and manifest logic |
| `ooxmlsdk-roundtrip-tests` | Generated per-file Apache POI, LibreOffice, and Open XML SDK package tests |
| `ooxmlsdk-bench` | Package and XML Criterion benchmarks |

The workspace also contains EMF and legacy OLE/CFB test crates because those
formats feed Office rendering and compatibility work. Their ownership and
commands are documented in the test suite itself.

### Fixture And Corpus Taxonomy

- `corpus/<upstream>/` contains imported corpus files plus a corpus-local
  `manifest.toml`; defaults come from scanning, while manifests record only
  exceptional expectations such as invalid or open-only files.
- `corpus-manifest.toml` is the workspace-level corpus index.
- `fixtures/<upstream>/` contains focused provenance-specific inputs used by
  targeted integration tests.
- `licenses/<upstream>/` preserves third-party license and notice material.
- `docs/round-trip/` records corpus results and run history.
- `docs/ooxmlsdk-*-test/` records subsystem migration and upstream coverage.

Synthetic package smoke fixtures and the former directory-scanning
`round_trip_smoke_test` were removed. Focused regressions should use an upstream
fixture where possible or an explicit public-API assertion. Corpus-scale
packages belong in `corpus/`, never in this implementation repository.

### Round-Trip Model

`ooxmlsdk-roundtrip-tests` generates one ignored test per supported package
file. Its support crate opens, saves, and reopens a package, compares the part
graph and ZIP entries, and uses canonical XML equivalence where applicable.
Manifest exceptions remain executable expectations rather than skipped data.

Ignored corpus lanes are intentionally excluded from the suite's default
`cargo test --workspace`; they must be selected explicitly. Static schema float
normalization rules are checked in with the support crate, and the ignored
`float_rules_sync` test detects drift against this repository's `data/`.

## Where Tests Go

Use the narrowest owner:

- private generator/model behavior: unit tests in `ooxmlsdk-build`;
- derive behavior and expansion: tests in `ooxmlsdk-derive`;
- private implementation helpers: unit tests beside the relevant crate code;
- public core runtime behavior: `ooxmlsdk-test`;
- formula, fonts, layout, or PDF public behavior: the matching `*-test` crate;
- one upstream package regression useful to targeted assertions: a provenance
  fixture plus a focused test in the suite;
- broad open/save/reopen compatibility: corpus manifest and generated
  round-trip tests;
- performance: `ooxmlsdk-bench`.

Do not duplicate a corpus package as a focused fixture merely to make it easier
to locate. Tests can address corpus assets directly when that is the established
suite convention.

## Validation Strategy

Commands and Cargo discipline are listed in `AGENTS.md`. The important split is:

- run generator, formatting, implementation unit tests, feature-shape checks,
  and implementation clippy in this repository;
- run focused integration, subsystem, rendering, benchmarks, and corpus lanes
  from `../ooxmlsdk-test-suite`;
- for core runtime and generated-schema changes, validate the five
  `ooxmlsdk-test` feature shapes followed by the five
  `ooxmlsdk-roundtrip-tests` corpus lanes listed in `AGENTS.md`; do not replace
  them with the unrelated test-suite-wide workspace lane;
- during subsystem iteration, select the affected implementation crate and its
  matching focused test crate;
- reserve all ignored corpus lanes for changes that affect package fidelity,
  canonical comparison, manifests, or release-level confidence.

Cargo commands are sequential within each repository and use its default
`target/`. Do not overlap Cargo work across the two sibling repositories when
validating a change set; path dependencies can otherwise make results and lock
behavior difficult to interpret.

## Upstream Map

### Open XML SDK

Use `../Open-XML-SDK` for core API semantics, generated metadata, validators,
package tests, and assets. Important roots include `src/`, `generated/`,
`data/`, and `test/`.

### LibreOffice

Use `../core` as the primary source for layout/PDF behavior and as supplemental
core OOXML evidence. Relevant roots vary by subsystem, notably `oox/qa/`,
`sw/qa/`, `sc/qa/`, `sd/qa/`, and `vcl/qa/cppunit/pdfexport/`.

### Coverage Documentation

The live migration and coverage records now belong to
`../ooxmlsdk-test-suite/docs/`. Historical matrices still present under this
repository's `docs/tests/` are useful migration context, but should not be
treated as the current fixture inventory.

## Documentation Map

- `README.md`: public crate overview and feature surface.
- `ARCHITECTURE.md`: implementation/test ownership and data flow.
- `AGENTS.md`: operational rules and current command map.
- `CHANGELOG.md`: release history.
- `COMPATIBILITY.md`: parser and format compatibility status.
- `docs/specs/`: behavior and format notes.
- `../ooxmlsdk-test-suite/README.md`: recorded corpus results.
- `../ooxmlsdk-test-suite/AGENTS.md`: test workspace rules and corpus commands.
- `../ooxmlsdk-test-suite/docs/`: active upstream migration and coverage matrices.

Update the owning document when architecture, commands, fixture placement, or
coverage responsibility changes.
