# Change Log

## Unreleased

### Breaking Changes

- Removed the `microsoft365` Cargo feature. Newer Office schema modules, generated types, parts, and document sample coverage now compile as part of the normal runtime surface instead of being hidden behind a version-oriented feature gate.

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
