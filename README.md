# Open XML SDK for Rust

[![crates.io](https://img.shields.io/crates/v/ooxmlsdk.svg)](https://crates.io/crates/ooxmlsdk)
[![docs](https://docs.rs/ooxmlsdk/badge.svg)](https://docs.rs/ooxmlsdk)

Open XML SDK for Rust (ooxmlsdk) is a Rust library for working with Office Word, Excel, and PowerPoint documents. The basic idea is inspired by .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK).

## Features

- Deserialize/serialize OOXML (basically all of them) to/from Rust structs
- Support namespace (WIP: serialize with namespace)
- Read `docx`, `xlsx` and `pptx` file
- Write `docx`, `xlsx` and `pptx` file (WIP)
- All Rust structs and ser/de code are generated from metadata (`crates/ooxmlsdk/data`)
- Documentation for Rust structs

## What's missing

- No validation
- No Particle: all xml children is in a flattern `children` vector of enum

## Project Structure

- `crates/ooxmlsdk`: generated code for deserialize & serialize OOXML
- `crates/ooxmlsdk-build`: generate code for deserialize & serialize OOXML

## Known Issues

- This library raise recursion limit by `#![recursion_limit = "1024"]`, be cautious when use.

## License

MIT OR Apache-2.0

**`crates/ooxmlsdk/data`** is directly copied from .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data) so this directory is licensed under [this license](https://github.com/dotnet/Open-XML-SDK/blob/main/LICENSE)
