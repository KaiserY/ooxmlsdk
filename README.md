# Open XML SDK for Rust

[![crates.io](https://img.shields.io/crates/v/ooxmlsdk.svg)](https://crates.io/crates/ooxmlsdk)
[![docs](https://docs.rs/ooxmlsdk/badge.svg)](https://docs.rs/ooxmlsdk)

`ooxmlsdk` is a Rust library for reading, writing, and round-tripping Office Open XML documents such as `.docx`, `.xlsx`, and `.pptx`. The overall API shape is inspired by the .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK), but the implementation is code-generated for Rust and organized around generated schema types, namespaces, serializers, deserializers, and strongly typed package parts.

## Features

The runtime crate exposes a small public feature surface:

- `default`: enables `microsoft365` and `parts`; this is the recommended configuration for most users
- `parts`: enables package-level OOXML read/write support such as `WordprocessingDocument`, `SpreadsheetDocument`, and `PresentationDocument`
- `microsoft365`: enables the post-Office 2007 schema and part surface used by newer Office releases
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

## Version Coverage

This repository treats Office 2007 as the compatibility baseline for the narrower package surface:

- `--no-default-features --features parts`: Office 2007-oriented package and schema coverage
- default build: Office 2007 baseline plus the broader `microsoft365` surface

The `microsoft365` feature name is an umbrella label for everything newer than the Office 2007-oriented surface in this repository. It is not limited to Microsoft 365 subscription documents.

When `microsoft365` is enabled, the checked-in generated runtime covers newer OOXML namespaces and parts associated with:

- Office 2010
- Office 2013
- Office 2016
- Office 2019
- Office 2021
- Microsoft 365-era extensions and newer upstream namespace revisions currently present in the checked-in metadata, including 2022, 2023, and 2024-dated schema additions

In practical terms, this is the feature that pulls in support for newer namespaces and package relationships such as later DrawingML, chart extensions, SVG and 3D-related parts, threaded comments, dynamic-array-era spreadsheet extensions, and other post-2007 additions tracked in the upstream Open XML SDK metadata.

## Quick Start

Most users should keep the default features enabled:

```toml
[dependencies]
ooxmlsdk = "0.4.0"
```

If you want the narrower Office 2007-oriented package surface, disable default features and enable only `parts`:

```toml
[dependencies]
ooxmlsdk = { version = "0.4.0", default-features = false, features = ["parts"] }
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

## Project Structure

- `crates/ooxmlsdk`: runtime library exposed to downstream users
- `crates/ooxmlsdk-build`: generator that turns checked-in metadata into Rust code
- `crates/ooxmlsdk-derive`: derive macros used by the generated runtime code
- `crates/ooxmlsdk-test`: integration tests and benchmarks
- `sdk_data/`: checked-in intermediate generator data
- `data/`: upstream-derived metadata snapshots consumed by the generator pipeline
- `schemas/OpenPackagingConventions-XMLSchema/`: package schema inputs used by the generator

The generated runtime code under `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, `crates/ooxmlsdk/src/parts/`, and related module files is intended to be checked in and reviewed as generated artifacts.

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

## Data Provenance

`data/` is directly copied from the upstream .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data).

`sdk_data/` is generated from the upstream .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data), and `schemas/OpenPackagingConventions-XMLSchema/` contains package schema inputs derived from the Open Packaging Conventions XSDs. Review upstream licensing before redistributing refreshed snapshots.

## License

MIT OR Apache-2.0
