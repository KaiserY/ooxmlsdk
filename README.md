# Open XML SDK for Rust

[![crates.io](https://img.shields.io/crates/v/ooxmlsdk.svg)](https://crates.io/crates/ooxmlsdk)
[![docs](https://docs.rs/ooxmlsdk/badge.svg)](https://docs.rs/ooxmlsdk)

`ooxmlsdk` is a Rust library for reading, writing, and round-tripping Office Open XML documents such as `.docx`, `.xlsx`, and `.pptx`. The public package API is intentionally aligned with the .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK) container model, while the implementation is code-generated for Rust and organized around generated schema types, namespaces, serializers, deserializers, and strongly typed package parts.

## Features

The runtime crate exposes a small public feature surface:

- `default`: enables `parts`; this is the recommended configuration for most users
- `parts`: enables package-level OOXML read/write support such as `WordprocessingDocument`, `SpreadsheetDocument`, and `PresentationDocument`
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

This repository treats Office 2007 as the compatibility baseline while always compiling the checked-in generated runtime for newer OOXML namespaces and parts:

- `--no-default-features --features parts`: package APIs without optional validators
- default build: package APIs plus the full generated schema and part surface

The checked-in generated runtime covers OOXML namespaces and parts associated with:

- Office 2010
- Office 2013
- Office 2016
- Office 2019
- Office 2021
- Microsoft 365-era extensions and newer upstream namespace revisions currently present in the checked-in metadata, including 2022, 2023, and 2024-dated schema additions

In practical terms, the runtime includes support for newer namespaces and package relationships such as later DrawingML, chart extensions, SVG and 3D-related parts, threaded comments, dynamic-array-era spreadsheet extensions, and other post-2007 additions tracked in the upstream Open XML SDK metadata.

## Quick Start

Most users should keep the default features enabled:

```toml
[dependencies]
ooxmlsdk = "0.5.1"
```

If you want package APIs without optional validators or MCE-specific behavior, disable default features and enable only `parts`:

```toml
[dependencies]
ooxmlsdk = { version = "0.5.1", default-features = false, features = ["parts"] }
```

Read, inspect, and save a package:

```rust
use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;
use ooxmlsdk::sdk::SdkPackage;

fn round_trip(path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
  let document = WordprocessingDocument::new_from_file(path)?;
  let main_part = document.main_document_part().expect("main document part");
  assert!(document.get_id_of_part(&main_part).is_some());

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

## Package API

The `parts` feature exposes package-level APIs for `.docx`, `.xlsx`, and `.pptx` files. The intended public surface follows upstream Open XML SDK concepts:

- open and create packages with constructors such as `new`, `new_lazy`, `new_from_file`, and `new_from_file_lazy`
- save packages with `save`
- inspect package and part relationships with `parts`, `get_all_parts`, `get_part_by_id`, `get_parts_of_type`, and relationship-specific helpers
- access well-known child parts through typed methods such as `main_document_part`, `workbook_part`, `presentation_part`, `worksheet_parts`, `font_table_part`, and chart-related part accessors
- read, replace, or unload parsed part payloads through public data helpers and root-element helpers

Raw package storage, raw relationship sets, generated factory internals, and unchecked dynamic part plumbing are not part of the public API. Prefer the package and part methods above when writing code that should survive generator updates.

## XML And MCE Compatibility

The generated XML reader/writer preserves markup compatibility data needed for stable round trips, including common `mc:*` attributes, `mc:AlternateContent`, choice/fallback content, and extension namespace attributes used by newer Office documents.

Current integration coverage includes upstream-derived MCE and extension samples such as `mcdoc.docx`, `mcinleaf.docx`, `MCExecl.xlsx`, `excel14.xlsx`, `extlst.xlsx`, and Office 2016 extended chart packages. These tests focus on public Rust APIs and stable XML/package round trips.

Full Open XML SDK-style `OpenSettings` markup compatibility processing, unknown-element DOM editing, and markup compatibility validator behavior are still future work.

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
- Open XML SDK-style `OpenSettings`, full markup compatibility processing modes, and unknown-element DOM APIs are not yet exposed.
- Some schema shapes still map to generated enum-based child collections rather than a fully particle-aware hand-modeled API.
- `to_string()` is just `Display`; prefer the XML-oriented APIs when you care about write performance.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## Data Provenance

`data/` is directly copied from the upstream .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data).

`sdk_data/` is generated from the upstream .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data), and `schemas/OpenPackagingConventions-XMLSchema/` contains package schema inputs derived from the Open Packaging Conventions XSDs. Review upstream licensing before redistributing refreshed snapshots.

## License

MIT OR Apache-2.0
