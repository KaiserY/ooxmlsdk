# OOXML SDK for Rust

[![crates.io](https://img.shields.io/crates/v/ooxmlsdk.svg)](https://crates.io/crates/ooxmlsdk)
[![docs](https://docs.rs/ooxmlsdk/badge.svg)](https://docs.rs/ooxmlsdk)

`ooxmlsdk` is a Rust library for reading, writing, and round-tripping Office Open XML documents such as `.docx`, `.xlsx`, and `.pptx`. It uses the .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK) as a primary reference for OOXML package and schema behavior, but exposes Rust-native generated types, serializers, and strongly typed package parts.

## Features

The runtime crate exposes a small public feature surface:

- `default`: enables `parts`; this is the recommended configuration for most users
- `parts`: enables package-level OOXML read/write support such as `WordprocessingDocument`, `SpreadsheetDocument`, and `PresentationDocument`
- `flat-opc`: enables Flat OPC package read/write helpers and depends on `parts`
- `mce`: enables Markup Compatibility and Extensibility processing and depends on `parts`
- `validators`: enables optional validator APIs

The minimum supported Rust version is 1.88, and the workspace uses the Rust
2024 edition.

## Documentation

Rust API documentation is published on [docs.rs/ooxmlsdk](https://docs.rs/ooxmlsdk).

Guides and examples are maintained separately in [ooxmlsdk-doc](https://github.com/KaiserY/ooxmlsdk-doc). That documentation follows the shape of the Microsoft Learn [Open XML SDK documentation](https://learn.microsoft.com/en-us/office/open-xml/open-xml-sdk), but rewrites the material for this Rust crate, Rust naming, Cargo features, generated schema types, and Rust package APIs.

For background on Open XML package concepts, file format structure, WordprocessingML, SpreadsheetML, PresentationML, Flat OPC, and Markup Compatibility, the Microsoft Learn documentation remains the upstream conceptual reference. This crate follows many of the same package and schema concepts while exposing Rust-native generated types and feature flags.

## Round-Trip Coverage

Corpus-scale round-trip data is tracked in [ooxmlsdk-test-suite](https://github.com/KaiserY/ooxmlsdk-test-suite). Latest recorded results:

| Corpus | Files | Round-trip candidates | Open-only | Invalid | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| Apache POI | 677 | 602 | 11 | 64 | 677 passed / 0 failed |
| LibreOffice | 3368 | 3335 | 7 | 26 | 3368 passed / 0 failed |
| Open-XML-SDK | 884 | 874 | 6 | 4 | 884 passed / 0 failed |

Last run: 2026-06-07.

## Version Coverage

Office 2007 is the baseline. The checked-in generated schemas also include newer OOXML namespaces and package parts from the upstream metadata.

Common build shapes:

- default: generated schemas plus package APIs
- `--no-default-features --features parts`: package APIs only
- `--no-default-features --features flat-opc`: package APIs plus Flat OPC helpers
- `--no-default-features --features mce`: package APIs plus Markup Compatibility and Extensibility processing
- `--features validators`: optional validator APIs

The generated runtime includes Office 2010, 2013, 2016, 2019, 2021, Microsoft 365-era extensions, and newer upstream namespace revisions currently present in the checked-in metadata. In practice this covers later DrawingML and chart extensions, SVG and 3D-related parts, threaded comments, dynamic-array-era spreadsheet extensions, and other post-2007 additions tracked by Open XML SDK metadata.

## Package API

The `parts` feature exposes package-level APIs for `.docx`, `.xlsx`, and `.pptx` files. The intended public surface follows upstream Open XML SDK concepts:

- open and create packages with constructors such as `new`, `new_from_file`, and their settings variants
- save packages with `save`
- create custom parts with `add_new_part_with_content_type_and_path` when the caller needs an explicit package path and content type
- inspect package and part relationships with `parts`, `get_all_parts`, `get_part_by_id`, `get_parts_of_type`, and relationship-specific helpers
- traverse typed related parts with helpers such as `related_parts_of_type`, `related_part_of_type`, and relationship-type-specific variants when the relationship id is needed alongside the typed part
- access well-known child parts through typed methods such as `main_document_part`, `workbook_part`, `presentation_part`, `worksheet_parts`, `font_table_part`, and chart-related part accessors
- read, replace, or unload parsed part payloads through public data helpers and root-element helpers

Raw package storage, raw relationship sets, generated factory internals, and unchecked dynamic part plumbing are not part of the public API. Prefer the package and part methods above when writing code that should survive generator updates.

The package API follows Open XML SDK container concepts. When relationship metadata matters, typed traversal helpers return `RelatedPart<T>` so callers can keep the typed part and its `r:id` together.

File-backed packages retain a safe positioned ZIP reader, while reader-backed
packages retain immutable owned bytes. Package metadata is read when opening,
but individual Part payloads are decompressed on first access. The default open
mode keeps typed Part roots lazy. Saving serializes package metadata and every
loaded typed root, while Part payloads that were never loaded as typed roots
reuse their original compressed ZIP data.

## Generated Schema API

The `schemas` module is generated from upstream Open XML SDK metadata plus checked-in schema extensions. Generated names are intended to read like Rust while staying traceable to the source schema:

- repeated child fields are named for their item type, for example `paragraph`, `extension`, or `table_row`
- choices use concrete child names when the schema provides enough information; generic names remain for genuinely anonymous schema groups
- common scalar shapes are typed: lists are `Vec<T>`, OOXML booleans are enums, and measures/percentages use unit wrappers
- extension and wildcard content is preserved, with known children exposed through typed choices where possible

Prefer these generated types and conversion helpers over raw XML strings in new code. See the changelog for release-specific API changes.

## XML And MCE Compatibility

The generated XML reader/writer preserves markup compatibility data needed for stable round trips, including common `mc:*` attributes, `mc:AlternateContent`, choice/fallback content, unknown extension attributes, and extension namespace children used by newer Office documents.

With the `mce` feature enabled, package/root loading can process known Markup Compatibility and Extensibility constructs such as `mc:AlternateContent` and package-level `ProcessAllParts` behavior. Integration coverage includes upstream-derived MCE, strict, OPC, extension, and real-world compatibility samples, with tests focused on public Rust APIs and stable XML/package round trips.

Unknown-element DOM editing and markup compatibility validator behavior are still future work.

## Flat OPC

The `flat-opc` feature exposes Wordprocessing Flat OPC helpers for loading and writing XML package representations. Flat OPC APIs support strings and readers, and written Flat OPC preserves binary package parts such as alternative format import parts while writing XML-safe parts such as SVG media as XML data.

## Project Structure

- `crates/ooxmlsdk`: runtime library exposed to downstream users
- `crates/ooxmlsdk-build`: generator that turns checked-in metadata into Rust code
- `crates/ooxmlsdk-derive`: derive macros used by the generated runtime code
- `sdk_data/`: checked-in intermediate generator data
- `data/`: upstream-derived metadata snapshots consumed by the generator pipeline
- `schemas/OpenPackagingConventions-XMLSchema/`: package schema inputs used by the generator

Focused integration tests, package compatibility smoke tests, and runtime
benchmarks live in the adjacent `../ooxmlsdk-test-suite/crates/ooxmlsdk-test`
crate.

The generated runtime code under `crates/ooxmlsdk/src/schemas/`, `crates/ooxmlsdk/src/deserializers/`, `crates/ooxmlsdk/src/serializers/`, `crates/ooxmlsdk/src/parts/`, and related module files is intended to be checked in and reviewed as generated artifacts.

## Validation And Benchmarks

For release validation, this repository uses the full workspace sequence:

```bash
cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture
cargo fmt --all
cargo test --workspace
cargo test --workspace --no-default-features
cargo test --workspace --no-default-features --features parts
cargo test --workspace --no-default-features --features flat-opc
cargo test --workspace --no-default-features --features mce
cargo clippy --workspace --all-targets --no-default-features -- -D warnings
cargo clippy --workspace --all-targets --no-default-features --features parts -- -D warnings
cargo clippy --workspace --all-targets --no-default-features --features flat-opc -- -D warnings
cargo clippy --workspace --all-targets --no-default-features --features mce -- -D warnings
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
cd ../ooxmlsdk-test-suite && cargo test -p ooxmlsdk-test --features validators
```

For runtime performance work, prefer evaluating
`cd ../ooxmlsdk-test-suite && cargo bench -p ooxmlsdk-test --bench perf` as a
whole. The `packages` and `xml` suites have shown a persistent disagreement on
`wordprocessing_document/write/parsed`, so treat that one case as an anomaly
rather than as the sole performance signal.

The package compatibility smoke lane is maintained in the adjacent test suite:

```bash
cd ../ooxmlsdk-test-suite && cargo test -p ooxmlsdk-test round_trip -- --nocapture
```

Corpus-scale package round-trip validation is also maintained in the adjacent
`../ooxmlsdk-test-suite/` checkout. Prefer that local path; the remote is
`https://github.com/KaiserY/ooxmlsdk-test-suite`.

The committed fixture set includes document, presentation, MCE, OPC, DrawingML, WML, and SpreadsheetML coverage, including spreadsheet cell types, defined names, formatting, formulas, freeze panes, merged cells, number formats, row/column dimensions, sheet visibility, and rich shared strings.

## Known Limitations

- There is no `serde` integration.
- The validator surface is optional and still narrower than the core read/write path.
- Unknown-element DOM APIs and markup compatibility validator behavior are not yet exposed.
- Some schema shapes still map to generated enum-based child collections rather than a fully particle-aware hand-modeled API.
- `to_string()` is just `Display`; prefer the XML-oriented APIs when you care about write performance.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## Data Provenance

`data/` is directly copied from the upstream .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data).

`sdk_data/` is generated from the upstream .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data), and `schemas/OpenPackagingConventions-XMLSchema/` contains package schema inputs derived from the Open Packaging Conventions XSDs. Review upstream licensing before redistributing refreshed snapshots.

## License

MIT OR Apache-2.0
