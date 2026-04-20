# Open XML SDK for Rust

[![crates.io](https://img.shields.io/crates/v/ooxmlsdk.svg)](https://crates.io/crates/ooxmlsdk)
[![docs](https://docs.rs/ooxmlsdk/badge.svg)](https://docs.rs/ooxmlsdk)

`ooxmlsdk` is a Rust library for reading, writing, and round-tripping Office Open XML documents such as `.docx`, `.xlsx`, and `.pptx`. The overall API shape is inspired by the .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK), but the implementation is code-generated for Rust.

## 0.4.0 Highlights

- Simplified public feature surface around the features that are actually used today: `parts`, `microsoft365`, and `validators`.
- Stronger package-level coverage for Wordprocessing, Spreadsheet, and Presentation documents through the generated `parts` API.
- Validator support remains optional behind the `validators` feature and has dedicated integration coverage.
- Read-path performance work in `ooxmlsdk-derive` now pushes more dispatch decisions into code generation, reducing runtime branching and unnecessary allocations in generated parsers.
- Release validation for `0.4.0` is based on checked-in generated runtime code plus full workspace test and clippy coverage.

## Features

The runtime crate currently exposes these public features:

- `default`: enables `microsoft365` and `parts`
- `parts`: enables package-level OOXML read/write support such as `WordprocessingDocument`, `SpreadsheetDocument`, and `PresentationDocument`
- `microsoft365`: enables newer schema and part coverage that is not available in the Office 2007-oriented surface
- `validators`: enables optional validator APIs

The always-available modules in the crate root are:

- `common`
- `namespaces`
- `schemas`
- `sdk`
- `simple_type`

Feature-gated modules are:

- `parts` behind `parts`
- `validator` behind `validators`

## Quick Start

Add the crate:

```toml
[dependencies]
ooxmlsdk = "0.4.0"
```

Read and save a package:

```rust
use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

fn round_trip(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
  let document = WordprocessingDocument::new_from_file(path)?;
  let mut out = std::io::Cursor::new(Vec::new());
  document.save(&mut out)?;
  Ok(())
}
```

Parse XML into generated schema types:

```rust
use ooxmlsdk::schemas::opc_core_properties::CoreProperties;

fn parse_core_properties(xml: &str) -> Result<CoreProperties, Box<dyn std::error::Error>> {
  Ok(xml.parse()?)
}
```

## What 0.4.0 Focuses On

`0.4.0` is mainly a quality and runtime release rather than a surface-area explosion.

- The derive-generated read path was tightened so more child and choice dispatch work happens in generated code instead of generic runtime fallback logic.
- Package handling remains centered on generated strongly typed parts instead of a thin zip wrapper.
- Feature gating is clearer: Office 2007-oriented validation is done via `--no-default-features --features parts`, while newer Microsoft 365 coverage stays behind `microsoft365`.
- Validator support is still optional and should be treated as an extra capability, not the default path.

## Project Structure

- `crates/ooxmlsdk`: runtime library exposed to downstream users
- `crates/ooxmlsdk-build`: generator that turns checked-in metadata into Rust code
- `crates/ooxmlsdk-derive`: derive macros used by the generated runtime code
- `crates/ooxmlsdk-test`: integration tests and benchmarks
- `sdk_data/`: checked-in intermediate generator data
- `schemas/OpenPackagingConventions-XMLSchema/`: package schema inputs used by the generator

The generated runtime code under `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/parts/`, and related module files is intended to be checked in and reviewed as generated artifacts.

## Validation And Benchmarks

For release validation, this repository uses the full workspace sequence:

```bash
cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture
cargo test --workspace
cargo test --workspace --no-default-features
cargo test --workspace --no-default-features --features parts
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
```

For runtime performance work, prefer evaluating `cargo bench -p ooxmlsdk-test` as a whole. The `packages` and `xml` suites have shown a persistent disagreement on `wordprocessing_document/write/parsed`, so treat that one case as an anomaly rather than as the sole performance signal.

## Known Limitations

- There is no `serde` integration.
- The validator surface is optional and still narrower than the core read/write path.
- Some schema shapes still map to generated enum-based child collections rather than a fully particle-aware hand-modeled API.
- `to_string()` is just `Display`; prefer the XML-oriented APIs when you care about write performance.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## License

MIT OR Apache-2.0

`sdk_data/` is generated from the upstream .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data), and `schemas/OpenPackagingConventions-XMLSchema/` contains package schema inputs derived from the Open Packaging Conventions XSDs. Review upstream licensing before redistributing refreshed snapshots.
