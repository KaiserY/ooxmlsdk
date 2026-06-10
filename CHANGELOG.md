# Change Log

## Unreleased

## 0.10.1

### Changed

- Refactored generated `SdkType` reader generation so nested structs, tuple wrappers, and choice variants all use a single `SdkType::read_inner` dispatch path.
- Removed `XmlRead::read_inner` from the public read trait and the now-redundant runtime forwarding implementations.
- Consolidated `AnyChild` parsing generation in MCE branches to share clearer end-tag and empty-element handling.
- Updated crate metadata branding (`Open XML` -> `OOXML`) and dependency alignment (`regex` -> `1.12.4`) for the 0.10.1 workspace.

### Fixed

- Simplified parser code paths around `AnyChild` payload collection to avoid incomplete or over-collected raw XML buffers in empty-child and mixed-content cases.
- Reduced generated-structure churn from the `SdkType`/`XmlRead` refactor while preserving compatibility behavior.

## 0.10.0

### Breaking Changes

- Removed the generated XML body writer from the public `SdkType` trait. Generated schema types now use inherent `pub(crate)` XML write helpers, so normal package and generated-schema users should continue using the public read/write APIs, while custom low-level `SdkType` implementations that called or implemented `write_inner` need to move off that internal hook.

### Added

- Added `add_new_part_with_content_type_and_path` for package code that needs to create a part with an explicit content type and package path.
- Added missing `w14` numbering run-property children and additional schema-extension coverage for real-world WordprocessingML, SpreadsheetML, DrawingML, chart, VML, OPC, and Office extension markup.
- Added more focused extension metadata for namespace declarations, choice variants, optional and repeated children, unknown attributes, unknown children, prefix canonicalization, local-name attribute matching, and empty-as-none compatibility parsing.

### Changed

- Regenerated schema metadata and runtime output for the 0.10.0 release while keeping generated XML write paths direct and static.
- Reworked generated no-prefix XML writing so prefixed and no-prefix bodies are selected at compile time instead of passing a runtime boolean through `write_inner`.
- Improved namespace and raw relationship round trips for non-canonical prefixes, Office extension namespaces, spreadsheet extension roots, raw relationship ids, and compatibility-preserved XML.
- Updated the workspace crate versions to `0.10.0`.
- Updated `lopdf` to `0.41.0`.

### Fixed

- Fixed the remaining recorded corpus round-trip failures across the Apache POI, LibreOffice, and Open-XML-SDK fixture sets.
- Fixed additional schema gaps found by corpus fixtures, including missing choice variants, repeated choice children, optional children, namespace attributes, direct unknown children, other attributes, and compatibility-only children.
- Fixed generated parsing and writing for several real-world OOXML compatibility cases, including spreadsheet filters and named sheet views, sparklines and spill metadata, chart data labels and extension lists, WordprocessingML structured document tags, run properties, table borders, table positioning, VML shape children, Strict/core properties, and package manifest edge cases.
- Fixed compatibility handling for namespace aliases and canonical prefixes used by upstream producers, including cases where input prefixes differ from the canonical generated write prefix.
- Fixed round-trip comparison rules for upstream-compatible lossy or normalized behavior where LibreOffice preserves the same effective document state but rewrites child order, duplicate properties, empty properties, or equivalent lexical forms.

### Performance

- Removed runtime no-prefix branching from generated XML body writers by generating separate no-prefix helper bodies only for types that need them.
- Kept child and choice XML write dispatch static in generated code, avoiding extra helper layers on normal prefixed write paths.

### Testing

- Updated external round-trip corpus status: Apache POI `677 passed / 0 failed`, LibreOffice `3368 passed / 0 failed`, and Open-XML-SDK `884 passed / 0 failed`.
- Tracked corpus classifications in the README: Apache POI `602` round-trip candidates, `11` open-only, `64` invalid; LibreOffice `3335` round-trip candidates, `7` open-only, `26` invalid; Open-XML-SDK `874` round-trip candidates, `6` open-only, `4` invalid.
- Validated the release with generator regeneration, formatting, workspace tests, clippy, macro expansion checks, and the external round-trip suite.

### Documentation

- Updated the README round-trip coverage table with candidate, open-only, invalid, and zero-failure corpus counts.
- Reordered the README introduction so docs.rs, guide documentation, MSRV, and corpus status are easier to find before the API overview.

## 0.9.0

### Breaking Changes

- Regenerated schema APIs with more precise wrapper and optional-child modeling. Some generated fields that previously exposed required values now use `Option<T>`, and some single repeated child collections that also preserve unknown XML now expose generated choice enums instead of direct `Vec<T>` fields.
- Changed generated leaf-text derived types to newtype wrappers where the schema type is only a text specialization. This reduces duplicate struct fields but can require downstream code to construct or access the wrapped value through the regenerated newtype surface.
- Simplified XML namespace preservation types. `XmlNamespace` now stores known namespaces directly and raw namespace declarations as compact byte payloads; code that matched the older prefix/URI field layout must switch to the current enum-based API.
- Removed the `smallvec` dependency from the public runtime dependency graph.

### Added

- Added stack-safe generated parsing for recursive WordprocessingML table content, covering deeply nested `w:tbl`, `w:tr`, and `w:tc` structures without relying on recursive Rust call depth.
- Added root-element namespace/local-name fallback matching so package parts can load XML whose root element uses a non-canonical prefix while still keeping the normal fast path for canonical names.
- Added UTF-16 XML byte decoding support for package parts that declare or contain UTF-16 encoded XML payloads.
- Added schema extension support for generated child optionality, additional choice variants, XML namespace fields, unknown attributes, and direct unknown children in more WordprocessingML, SpreadsheetML, DrawingML, PresentationML, chart, math, threaded-comment, and Office extension types.
- Added round-trip coverage reporting to the README with current corpus-scale results from `ooxmlsdk-test-suite`.

### Changed

- Simplified generated namespace and relationship namespace handling by replacing the large generated namespace extension table with direct generated namespace metadata and byte-oriented matching.
- Moved corpus-scale round-trip fixtures and generated corpus tests out of `ooxmlsdk-test` and into the adjacent `ooxmlsdk-test-suite` repository.
- Kept generated raw XML preservation more targeted: fixed spreadsheet root namespace preservation for raw children without retaining redundant fixed prefixes on typed-only roots such as shared string tables.
- Updated PDF conversion code to tolerate optional or compatibility-preserved drawing content produced by the regenerated schemas while continuing to consume only typed children for rendering.

### Fixed

- Fixed Open XML SDK corpus round trips; the recorded Open-XML-SDK round-trip lane now passes all 884 tests.
- Fixed additional Apache POI and LibreOffice corpus round-trip cases involving alternate content, non-canonical namespace prefixes, chart extension parts, optional picture `blipFill`, SpreadsheetML styles, macrosheet attributes, table-style row properties, and core-properties compatibility children.
- Fixed Markup Compatibility processing for `mc:AlternateContent` elements that use a non-`mc` prefix bound to the markup-compatibility namespace.
- Fixed generated validator support for choice fields and schema-derived typed values used by validator tests.
- Fixed package and XML compatibility for chart extension roots by preserving unsupported chart extension payloads as raw bytes when they cannot be parsed as classic chart roots.
- Fixed parsing and preservation of unknown attributes and namespace declarations in additional SpreadsheetML calculation properties, macrosheets, fonts, cell formats, and differential formats.
- Fixed generated direct-unknown-child promotion so attributes do not participate in child choice layout decisions.

### Performance

- Reduced generated namespace lookup and write overhead by removing the large namespace extension dispatch table and matching common namespace data directly from bytes.
- Reduced stack usage for deeply nested table XML by using generated stack parsers for recursive table structures.

### Testing

- Updated round-trip corpus status from the external test suite: Apache POI `649 passed / 28 failed`, LibreOffice `3051 passed / 317 failed`, and Open-XML-SDK `884 passed / 0 failed`.
- Added focused tests for stack-safe nested table parsing, non-canonical MCE prefixes, UTF-16 XML package loading, generated validator choice fields, namespace canonicalization, and schema extension regressions.
- Regenerated the checked-in runtime from updated schema extension metadata and kept formatting aligned with the release output.

### Documentation

- Added `docs/specs/nested_xml_parsing.md` documenting the stack-safe recursive XML parsing model.
- Updated repository and test documentation to describe the external corpus round-trip suite and its role in release validation.

## 0.8.0

### Breaking Changes

- Removed the public `SdkChoice` derive macro. Choice parsing and writing are now generated by `SdkType` from choice metadata attached to the containing field, so custom derive users should migrate choice enums to `#[derive(SdkType)]`-driven field metadata or use the regenerated runtime output.
- Regenerated schema and part APIs with richer choice metadata, namespace metadata, and part metadata. Some generated field types and choice variant surfaces changed as a result of the new `SdkType` and `SdkPartRef` generation model.
- Replaced string-backed XML namespace and unknown-attribute storage with byte-oriented types. `XmlNamespaceDecl` was replaced by `XmlNamespace`, namespace URIs are represented by `XmlNamespaceUri`, and preserved unknown attributes are represented by `XmlOtherAttr`.
- Moved generated part descriptor metadata behind `SdkPartRef` and `SdkPartDescriptor` implementations instead of emitting large module-level constant blocks. Code that depended on generated descriptor constants or derive-internal part plumbing should use the public package/part APIs.
- Removed generated SDK version marker fields from the runtime schema model. Generated structs now focus on XML content and preservation state rather than version bookkeeping.

### Added

- Added the public `ooxmlsdk::namespaces` module with generated `XmlKnownNamespace` and `XmlKnownRelationshipNamespace` enums for static namespace and relationship URI lookup.
- Added `#[derive(SdkPartRef)]` support for the generated `PartRef` enum. Part descriptors, root metadata, and relationship dispatch metadata are now declared on `PartRef` variants and expanded by the derive macro.
- Added namespace extension metadata under `sdk_data/namespace_extensions.json` so generated XML code can canonicalize known prefixes and URI aliases from static data.
- Added schema extension metadata for additional OPC, DrawingML, SpreadsheetML, PresentationML, WordprocessingML, VML, and Office extension namespaces.

### Performance

- Optimized XML namespace handling by matching known namespace and relationship URIs from bytes instead of repeatedly allocating or normalizing strings.
- Optimized raw XML and MCE preservation paths by storing raw names, namespace declarations, unknown attributes, and preserved XML payloads in byte-oriented structures.
- Reduced generated `SdkType` read/write overhead by inlining choice dispatch, flattening common child dispatch paths, and removing generic fallback scaffolding from hot XML loops.
- Reduced generated part loading overhead by using statically known relationship enum variants for typed part dispatch instead of repeated descriptor string matching.
- Kept MCE namespace matching on byte paths so `mc:AlternateContent` and extension preservation do not force extra string conversion.

### Changed

- Consolidated choice parsing and writing into `SdkType`, replacing the separate `sdk_choice` implementation with field-level choice item metadata.
- Consolidated package and part relationship dispatch generation around shared derive helpers for relationship matching, typed child accessors, package child loading, and `PartRef` construction.
- Shortened generated macro input for namespace-qualified names by using byte string tokens and generated known namespace metadata.
- Simplified generated part modules by hiding root accessors, relationship descriptors, child relationship metadata, and part reference construction behind derives.
- Simplified generated XML writing by emitting direct write paths for typed children, text children, wildcard children, and choice sequences.

### Fixed

- Fixed image part detection and relationship matching after the generated part dispatch refactor.
- Improved relationship namespace canonicalization so package relationships using equivalent known URI spellings resolve through the same typed dispatch path.
- Improved preservation of namespace declarations and unknown attributes during XML round trips, including extension namespaces and compatibility markup.

### Testing

- Regenerated the checked-in runtime from updated schema, namespace, and package metadata.
- Added macro expansion dump coverage for `SdkPart`, `SdkPartRef`, and `SdkPackage` to keep generated code direct and predictable during derive refactors.
- Validated the release with generator regeneration, formatting, workspace tests, clippy, and targeted macro expansion dumps.

## 0.7.0

### Breaking Changes

- Regenerated the schema runtime with revised child and choice naming rules. Some generated field and variant names changed from namespace-prefixed fallbacks such as `a_p`, `a_ext`, `a_font`, `a_tr`, or `w_tab` to source-backed semantic names such as `paragraph`, `extension`, `supplemental_font`, `table_row`, and `tab_stop`.
- Normalized generated choice and sequence naming. Concrete variants now prefer the referenced child type or element name; generic `Choice`, `Choice2`, `Sequence`, and numbered suffixes are kept only when the source data does not provide a clearer stable name or when disambiguation is required.
- Reworked generated simple text wrapper types. Several text elements now wrap a shared text payload type instead of exposing duplicated `xml_content`, `xml_other_attrs`, and `xml:space` fields directly on each element type.
- Replaced the generic `ListValue<T>` wrapper with plain `Vec<T>` for generated list-valued attributes and text content.
- Changed OOXML boolean-like simple types from raw `bool` aliases to explicit enums: `BooleanValue`, `OnOffValue`, `TrueFalseValue`, and `TrueFalseBlankValue`. Use `from_bool()` / `as_bool()` or the `From` conversions when converting to and from Rust booleans.
- Changed many OOXML measure and percentage attributes from loose `String` or integer fields to typed unit unions from `ooxmlsdk::units`, including `TwipsMeasureValue`, `SignedTwipsMeasureValue`, `MeasurementOrPercentValue`, `DrawingmlPercentageValue`, coordinate values, HPS values, text point values, and related aliases.
- Changed `SdkEnum::as_xml_str()` to return `&str` instead of `&'static str` so generated open-enum fallback variants can preserve unknown lexical values.
- Changed the low-level `SdkType` derive contract. Generated types now expose `ELEMENT_NAME`, `read_borrowed`, `read_io`, and `write_inner` instead of the previous deserialize/write internals. This affects custom manual implementations of `SdkType`; normal `#[derive(SdkType)]` users should regenerate or rebuild.
- Replaced some raw wildcard XML surfaces with typed known-child choices. For example, `a:graphicData` now exposes `graphic_data_choice: Vec<GraphicDataChoice>` for known graphic data payloads while still preserving unknown content through `XmlAny`.

### Generated API Naming

- Generated field names now prefer schema class names, summary-backed names, or local element names over namespace prefixes. Namespace prefixes are used only when needed to avoid collisions or when no better source-backed name exists.
- Repeated child fields are named after the repeated item, not after the namespace-qualified XML tag. For example, repeated paragraph/table/extension children become `paragraph`, `table_row`, `extension`, etc.
- Choice variants prefer the concrete child type name. When multiple particles map to the same type or local name, the generator adds stable numeric suffixes rather than inventing semantic names not present in the source data.
- Anonymous wrapper choices are flattened when doing so is unambiguous. Generic names such as `Choice`, `Choice2`, and `Sequence` remain intentionally generic for source-anonymous particles.

### XML and Compatibility

- Preserved more unknown extension attributes and children in generated schemas, including DrawingML, SpreadsheetML, PresentationML, WordprocessingML, chart, comment, and OPC core properties compatibility cases.
- Added direct XML preservation for additional extension-list and alternate-content locations so round trips keep non-standard or newer Office markup instead of dropping it.
- Accepted additional real-world enum values and open enum fallbacks, including compatibility values such as SpreadsheetML sheet state `show`.
- Improved MCE and unknown child routing in generated choices, including content under `mc:AlternateContent` and Office extension namespaces.
- Improved Strict and OPC compatibility by tolerating real-world core-properties children such as `cp:contentType` while preserving unknown XML where appropriate.

### Simple Types and Units

- Added the public `ooxmlsdk::units` module with OOXML unit constants, parsers, lexical formatting, and conversions for EMU, twips, half-points, DrawingML percentages, Word percentages, VML fixed values, DrawingML angles, 1/100 mm, and text point units.
- Added typed simple-value aliases and wrappers for OOXML measure unions, percentage unions, coordinate unions, and text-size units so generated schema fields retain the original lexical category while still offering conversion helpers such as `to_emu()`, `to_twips()`, and ratio conversion.
- Added `Int32ZeroOnOverflowValue` handling for compatibility with fields where Office producers emit unsigned-looking values in signed integer slots.

### Package API

- Added `RelatedPart<T>` and typed related-part traversal helpers on packages and parts, including `related_parts_of_type`, `related_part_of_type`, and relationship-type variants that preserve the relationship id alongside the typed part handle.
- Kept existing typed child accessors and relationship lookup behavior while reducing repeated generated relationship traversal code.

### Derive and Code Generation

- Extended `ooxmlsdk-derive` to parse generated metadata for list-valued attrs/text, simple-type-specific XML parsing/writing, typed child choices, open enum fallback variants, direct XML preservation, and the new `SdkType` read/write contract.
- Improved generated read dispatch for overlapping nested choice branches, shared local names, and extension namespace children.
- Regenerated checked-in runtime schemas from updated `data/`, package schemas, and `sdk_data/schema_extensions`.

### Testing

- Expanded round-trip coverage for real-world LibreOffice and Open XML SDK fixtures, especially MCE, unknown extension content, DrawingML graphic data, SpreadsheetML pivot/cache edge cases, WordprocessingML run-property ordering, and strict/compatibility package inputs.
- Kept generator, formatting, workspace test, and clippy lanes green for the release preparation.

## 0.6.1

### Package API

- Kept the public package and part API stable while splitting public SDK traits from internal package/part state hooks.
- Reduced generated part accessor and relationship-dispatch boilerplate by moving repeated child-part lookup patterns into shared macros and generated compile-time dispatch tables.
- Preserved typed package/part relationship behavior while hiding more implementation-only state behind crate-internal traits and opaque runtime helpers.

### Performance

- Avoided cloning whole part handles when adding a part from the same package; same-package copy helpers now link by `PartId` directly.
- Reduced MCE context string storage overhead by storing read-mostly namespace and QName metadata as boxed string slices.
- Moved more fixed part relationship matching work out of repeated runtime code paths and into generated code.

### Testing

- Added broader SpreadsheetML compatibility fixtures for cell types, defined names, formatting, formulas, freeze panes, merged cells, number formats, row/column dimensions, sheet visibility, and rich shared strings.
- Added SpreadsheetML spec notes and fixture generation coverage for cells, formatting, formulas, layout, and names.
- Updated the compatibility matrix for the expanded committed fixture set.

### Maintenance

- Regenerated checked-in `parts` output after consolidating generated child accessor and relationship dispatch logic.
- Removed lint suppressions around the package trait split and feature-gated helper code.
- Kept the default workspace, MCE feature, clippy, formatting, and round-trip validation lanes green for the release.

## 0.6.0

### Breaking Changes

- Removed the `microsoft365` Cargo feature. Newer Office schema modules, generated types, parts, and document sample coverage now compile as part of the normal runtime surface instead of being hidden behind a version-oriented feature gate.
- Changed the default runtime feature set from `["microsoft365", "parts"]` to `["parts"]`. Consumers that previously named `microsoft365` should remove it; newer Office schema and part support is now part of the checked-in runtime output.

### New Features

- Added the optional `flat-opc` feature for Flat OPC package read/write support, including string and reader entry points plus normal package save/reopen flows.
- Added the optional `mce` feature for Markup Compatibility and Extensibility processing, including static `mc:AlternateContent` selection and package-level `ProcessAllParts` loading behavior.
- Added simple-type helper APIs for hex binary validation and byte conversion.
- Added a compatibility matrix and minimal DOCX/XLSX/PPTX/MCE fixture set for round-trip coverage tracking.

### XML and Package Compatibility

- Improved MCE preservation and processing for ignorable namespaces, `mc:ProcessContent`, `mc:PreserveAttributes`, `mc:MustUnderstand`, nested alternate content, and unknown extension content.
- Preserved relationship XML headers when loading and saving package relationship parts.
- Preserved Flat OPC altChunk parts as raw binary data, including XML-ish content types that should not be eagerly parsed as typed root elements.
- Wrote SVG media parts as Flat OPC XML data while keeping alternative format import parts in binary data.
- Kept saved XML package parts encoded as UTF-8 without a BOM.

### Generated Runtime

- Simplified generated schema output by routing more parent XML children through choice fields, flattening schema choice wrappers, omitting abstract schema base output, shortening generated schema type paths, and using unit fields for empty schema markers.
- Regenerated checked-in runtime schemas, parts, serializers, and deserializers from the updated generator shape.

### Testing

- Added broad upstream-derived round-trip and compatibility coverage for document samples, MCE samples, package APIs, Flat OPC flows, simple type comparison behavior, and spreadsheet `CellValue` lexical forms.
- Expanded `UPSTREAM_TEST_MATRIX.md` coverage tracking from 118 covered upstream tests to 162 covered tests, with clearer not-applicable classifications for .NET wrapper-only behavior.
- Added release validation lanes for `flat-opc` and `mce` feature combinations.

### Documentation

- Updated repository guidance for the current feature surface and full validation lanes.
- Added a clean-room MCE implementation note under `docs/specs/mce.md`.

## 0.5.1

### Package API

- Kept the public `parts` API aligned with upstream Open XML SDK while reducing generated enum boilerplate in the package implementation.
- Added upstream-derived coverage for `OpenXmlPart.UnloadRootElement()` through the public lazy root load/unload APIs.

### Maintenance

- Regenerated the checked-in `parts` output after consolidating repeated generated `PartRef` and `PartRootElement` dispatch code.
- Updated the upstream test coverage matrix to mark `UnloadRootElementTest` as covered.

## 0.5.0

### Breaking Changes

- Aligned the generated `parts` API more closely with upstream Open XML SDK container concepts. Public package and part operations now favor upstream-style entry points such as `new`, `new_lazy`, `new_from_file`, `save`, `parts`, `get_all_parts`, `get_part_by_id`, `get_parts_of_type`, relationship-specific helpers, and typed child accessors.
- Hid raw package storage, raw relationship storage, generated part factory plumbing, dynamic downcasts, and generic relationship mutation helpers from the public API. Consumers should use public package/part methods instead of depending on internal storage or relationship implementation details.
- Removed generated part descriptor metadata and typed child relationship accessor surfaces that exposed implementation details rather than upstream-aligned package behavior.

### New Features

- Expanded storage-backed package APIs for Wordprocessing, Spreadsheet, and Presentation documents, including shallow part handles, relationship-backed child lookup, package copy/import helpers, data part reference relationships, and save/reopen flows.
- Added broader upstream-derived package coverage for VML drawing parts, extended chart parts, font table parts, SDT mutation, media references, model3d relationships, and supported part creation/import paths.
- Added `UPSTREAM_TEST_MATRIX.md` to track coverage against upstream Open XML SDK tests and to distinguish covered, partial, missing, not applicable, and API-blocked areas.

### XML and MCE Compatibility

- Improved `mc:AlternateContent` handling so body-level alternate content, choices, fallbacks, `mc:Ignorable`, `mc:MustUnderstand`, and fallback raw XML children round-trip more predictably.
- Preserved additional markup compatibility attributes and extension attributes in Wordprocessing and Spreadsheet XML, including `mc:ProcessContent`, `mc:PreserveAttributes`, and Office extension namespace attributes.
- Added coverage for upstream MCE samples such as `mcdoc.docx`, `mcinleaf.docx`, `MCExecl.xlsx`, `excel14.xlsx`, `extlst.xlsx`, and Office 2016 extended chart samples.

### Bug Fixes

- Kept no-default and `parts`-only feature lanes compiling cleanly by gating parts-only helpers and tightening feature-gated test imports.
- Improved package relationship alias matching and descriptor checks used by generated part loading.
- Preserved touched font table namespace declarations such as `w14` through package save/reopen cycles.

### Testing

- Expanded integration coverage in `ooxmlsdk-test` for upstream-aligned package APIs, schema round trips, MCE XML preservation, and document sample package flows.
- Validated the release with the workspace default, `--no-default-features`, and `--no-default-features --features parts` test lanes, plus matching clippy lanes.

## 0.4.1

### New Features

- Simplified generated schema types for single-`any` child content so wildcard XML children are stored more directly and cleanly.

### Performance

- Improved XML attribute parsing by specializing enum, boolean-like, and integer simple-type reads around raw byte fast paths instead of routing common cases through string parsing.
- Flattened common generated child dispatch paths so direct children, text children, and flat choice groups are handled with less runtime branching in the hot read path.
- Reduced generated parser overhead for pure-`any` content models by trimming dead fallback scaffolding from common child loops.

### Bug Fixes

- Preserved `mc:AlternateContent` as explicit round-trip content instead of transparently backfilling its inner children into parent typed fields.
- Kept generated child dispatch stable when `cfg`-gated children are removed from a build, avoiding fallback dispatch mismatches across feature sets.

## 0.4.0

### Breaking Changes

- Simplified the public crate feature surface. The old version-sliced feature layout from earlier releases is no longer the supported model; the runtime crate now centers on `parts`, `microsoft365`, and `validators`.
- The default feature set is now `["microsoft365", "parts"]`. Consumers that previously depended on a schema-only or narrower custom feature mix may need to switch to `default-features = false` and opt back into the exact features they want.
- Office 2007-oriented validation and compatibility checks are now expected to be exercised with `--no-default-features --features parts`, while newer Microsoft 365 coverage remains behind `microsoft365`.
- `ooxmlsdk` now depends on `ooxmlsdk-derive` instead of `ooxmlsdk-build`. Downstream users should treat `ooxmlsdk-build` as generator-side tooling rather than as a runtime dependency path.
- The workspace now targets Rust 2024 edition.
- The minimum supported Rust version is now `1.88`.

### New Features

- Expanded checked-in generated runtime coverage for package parts and schema modules used by Wordprocessing, Spreadsheet, and Presentation documents.
- Kept validator support available behind the optional `validators` feature with dedicated integration coverage.
- Added release-oriented repository guidance around checked-in generated runtime artifacts and the full validation sequence used before publishing.

### Performance

- Improved derive-generated read-path performance by pushing more child and choice dispatch work into code generation instead of generic runtime fallback logic.
- Reduced runtime branching and unnecessary allocations in generated parsers, especially on package and XML read paths.

### Bug Fixes

- Tightened package relationship handling and compatibility aliases in generated part loading.
- Improved generated read-path correctness around child and choice dispatch while keeping the workspace validation matrix green.

### Misc Changes

- Bumped `quick-xml` to `0.39.2`.
- Bumped `regex` to `1.12.3`.
- Refreshed release validation and benchmark guidance in the repository documentation.

## 0.3.0

### New Features

### Bug Fixes

### Misc Changes

- Bump the `quick-xml` version to `v0.38.0`
- Use `decode_and_unescape_value` instead of `unescape_value`, fix `quick-xml` `encoding` feature issue, see https://github.com/tafia/quick-xml/pull/864

## 0.2.3

### New Features

### Bug Fixes

### Misc Changes

- Bump the deps version (`zip` to `4.2.0` etc.)

## 0.2.2

### New Features

### Bug Fixes

### Misc Changes

- Improve read & write performance.

## 0.2.1

### New Features

- Breaking change in part methods, add `new` and `save` align with `zip` crate.

### Bug Fixes

### Misc Changes

- Reduce `zip` dependency features.
- Format doc comment.

## 0.2.0

### New Features

- Add `FromStr` and `Display`.
- Performance improvement.
- Split features for smaller codegen and faster build.

### Bug Fixes

### Misc Changes

- Bump dependencies version

## 0.1.18

### New Features

- Add `new_from_reader` function for docx, xlsx and pptx parts

### Bug Fixes

### Misc Changes

- Bump dependencies version

## 0.1.17

### New Features

- Add `Debug` and `Default` traits for schemas and parts
- Refactor validators

### Bug Fixes

### Misc Changes

## 0.1.16

### New Features

- Split features for more compact library
- Add initial validator support
- More Particle support: more flatten `Sequence` children as struct fields

### Bug Fixes

### Misc Changes

- Add `CHANGELOG.md`
