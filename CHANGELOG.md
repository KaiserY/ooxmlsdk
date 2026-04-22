# Change Log

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
