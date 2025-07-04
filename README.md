# Open XML SDK for Rust

[![crates.io](https://img.shields.io/crates/v/ooxmlsdk.svg)](https://crates.io/crates/ooxmlsdk)
[![docs](https://docs.rs/ooxmlsdk/badge.svg)](https://docs.rs/ooxmlsdk)

Open XML SDK for Rust (ooxmlsdk) is a Rust library for working with Office Word, Excel, and PowerPoint documents. The basic idea is inspired by .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK).

## Features

* `default`: `docx`，`xlsx`，`pptx`，`office2007` and `parts`.
* `schemas`: generate xml schemas, deserializers and serializers. this feature is always enabled.
* `parts`: genrate parts for reading and writing office file.
* `validators`: WIP, for validate xml.
* `docx`: generate docx related schemas, parts etc.
* `xlsx`: generate xlsx related schemas, parts etc.
* `pptx`: generate pptx related schemas, parts etc.
* `office2007`: generate office2007 related schemas, parts etc. this feature is always enabled.
* `office2010`: generate office2010 and below related schemas, parts etc.
* `office2013`: generate office2013 and below related schemas, parts etc.
* `office2016`: generate office2016 and below related schemas, parts etc.
* `office2019`: generate office2019 and below related schemas, parts etc.
* `office2021`: generate office2021 and below related schemas, parts etc.
* `microsoft365`: generate microsoft365 related schemas, parts etc.

## What's missing

- No validation (WIP)
- No Particle: all xml children is in a flattern `children` vector of enum (`OneSequence` is supported, xml children are represented as struct's fields)

## Project Structure

- `crates/ooxmlsdk`: generated code for deserialize & serialize OOXML
- `crates/ooxmlsdk-build`: generate code for deserialize & serialize OOXML
- `examples/*`: examples

## Known Issues

- This library raise recursion limit by `#![recursion_limit = "512"]` for `cargo doc`, you may need to add it too, be cautious when using.
- Currently no `serde` support.
- `to_string()` is just for `Display`, which is slower than `to_xml()`. Prefer using `to_xml()`.

## Changelog

See [CHANGELOG.md](./CHANGELOG.md).

## License

MIT OR Apache-2.0

**`crates/ooxmlsdk/data` is directly copied from .NET [Open XML SDK](https://github.com/dotnet/Open-XML-SDK/tree/main/data) so this directory is licensed under [this license](https://github.com/dotnet/Open-XML-SDK/blob/main/LICENSE)**
