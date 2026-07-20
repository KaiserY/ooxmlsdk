# OOXML SDK for Rust

[![crates.io](https://img.shields.io/crates/v/ooxmlsdk.svg)](https://crates.io/crates/ooxmlsdk)
[![docs.rs](https://docs.rs/ooxmlsdk/badge.svg)](https://docs.rs/ooxmlsdk)

`ooxmlsdk` is a pure-Rust library for reading, editing, writing, and
round-tripping Office Open XML documents such as `.docx`, `.xlsx`, and `.pptx`.
It provides generated schema types, XML serializers, and strongly typed package
parts modeled after the .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK).

## Usage

```bash
cargo add ooxmlsdk
```

```rust,no_run
use ooxmlsdk::parts::wordprocessing_document::WordprocessingDocument;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let document = WordprocessingDocument::new_from_file("input.docx")?;
    document.save_as_file("output.docx")?;
    Ok(())
}
```

Use `SpreadsheetDocument` and `PresentationDocument` for `.xlsx` and `.pptx`.
Typed accessors such as `main_document_part`, `workbook_part`, and
`presentation_part` expose well-known package parts and their generated root
elements.

## Features

- `parts` (default): package-level OOXML read/write APIs
- `flat-opc`: Wordprocessing Flat OPC read/write helpers; enables `parts`
- `mce`: Markup Compatibility and Extensibility processing; enables `parts`
- `validators`: optional schema and package validation APIs

Generated schema and XML APIs remain available with `--no-default-features`.
The minimum supported Rust version is 1.88. The workspace uses Rust 2024.

## API and compatibility

Office 2007 is the baseline. Checked-in metadata also covers later OOXML
namespaces and package parts from Office 2010 through newer Microsoft 365-era
extensions tracked by Open XML SDK.

Generated names stay traceable to their schemas while using Rust collections,
enums, options, and unit wrappers. Schema-defined extension and wildcard
content is preserved, and known extension children are typed where metadata is
available.

Package metadata is read eagerly, while Part payloads and typed roots are lazy.
Saving serializes loaded roots and reuses original compressed data for unloaded
payloads. Relationship-aware traversal can retain both the typed Part and its
relationship id.

The `mce` feature processes known `mc:AlternateContent` and package-level
compatibility behavior. The `flat-opc` feature supports string and reader APIs
and preserves binary package parts while writing XML-safe parts as XML data.

## Documentation and specifications

- [Rust API documentation](https://docs.rs/ooxmlsdk)
- [Guides and examples](https://github.com/KaiserY/ooxmlsdk-doc)
- [Microsoft Open XML documentation](https://learn.microsoft.com/en-us/office/open-xml/open-xml-sdk)
- [Architecture and contributor map](./ARCHITECTURE.md)
- [Release notes](./CHANGELOG.md)

Generator metadata under `data/` comes from Open XML SDK. `sdk_data/` and the
Open Packaging Conventions schemas are checked-in generator inputs; see
[ARCHITECTURE.md](./ARCHITECTURE.md) for the generation and provenance model.

## Round-trip coverage

Corpus-scale validation lives in
[ooxmlsdk-test-suite](https://github.com/KaiserY/ooxmlsdk-test-suite). Manifest
exceptions remain executable open-only or invalid-package expectations rather
than skipped files.

| Corpus | Files | Round-trip | Open-only | Invalid | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| Apache POI | 682 | 606 | 12 | 64 | 682 passed / 0 failed |
| LibreOffice | 3388 | 3355 | 7 | 26 | 3388 passed / 0 failed |
| Open-XML-SDK | 886 | 877 | 6 | 3 | 886 passed / 0 failed |
| Pandoc | 235 | 234 | 1 | 0 | 235 passed / 0 failed |
| ClosedXML | 286 | 284 | 0 | 2 | 286 passed / 0 failed |
| **Total** | **5477** | **5356** | **26** | **95** | **5477 passed / 0 failed** |

Last full run: 2026-07-20. Per-corpus commands, classifications, and run history
are maintained in the test-suite documentation.

## Status

- There is no `serde` integration.
- Validator coverage is narrower than the core read/write path.
- Unknown-element DOM editing and MCE validator behavior are not yet exposed.
- Some schema particles are represented as generated choice enums.

The project is pre-1.0, so generated APIs can still change between releases.

## License

MIT OR Apache-2.0
