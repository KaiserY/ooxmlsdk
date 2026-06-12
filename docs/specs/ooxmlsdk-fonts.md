# ooxmlsdk-fonts Design

## 1. Goal

`ooxmlsdk-fonts` is the shared font, metrics, shaping, and font-usage layer for
layout and PDF output.

It exists to keep layout measurement and final rendering on the same font
decision path. If layout measures text with one font and PDF paints it with
another, page breaks, table widths, line heights, and glyph positions drift.

This crate is not a PDF font helper. It is centered on font discovery,
substitution, metrics, shaping, and usage identity. Consumers may include
layout engines, PDF export, SVG export, raster previews, text inspectors, and
test dump tools. PDF embedding is one downstream consumer, not the design
center.

Target relationship:

```text
ooxmlsdk-fonts
  -> ooxmlsdk-layout
  -> ooxmlsdk-pdf
```

`ooxmlsdk-layout` uses font resolution, metrics, and shaping. `ooxmlsdk-pdf`
uses the resolved font and glyph usage to embed/subset and paint the same text.

## 2. Design Authority

### 2.1 Primary Reference: LibreOffice VCL and Text Stack

Use LibreOffice's font and output-device behavior as the main reference.

| Area | LibreOffice path |
|---|---|
| Font metric initialization | `../core/vcl/source/font/fontmetric.cxx` |
| Font selection and fallback | `../core/vcl/source/font/` |
| Physical font collection | `../core/vcl/source/font/PhysicalFontCollection.cxx` |
| Glyph layout/output device behavior | `../core/vcl/source/gdi/` |
| PDF text output | `../core/vcl/source/pdf/pdfwriter_impl.cxx` |
| Writer text formatting users | `../core/sw/source/core/text/itrform2.cxx` |
| Calc output users | `../core/sc/source/ui/view/output.cxx` |
| Drawing text users | `../core/editeng/`, `../core/svx/` |

Use these sources to model font metrics, fallback, decoration metrics,
substitution, and the distinction between measuring text and embedding fonts in
an output format.

### 2.2 Typst Reference

Typst is a Rust implementation reference for shaping and PDF coordination.

| Area | Typst path |
|---|---|
| Inline shaping | `../typst/crates/typst-layout/src/inline/shaping.rs` |
| Line construction users | `../typst/crates/typst-layout/src/inline/line.rs` |
| PDF text output | `../typst/crates/typst-pdf/src/text.rs` |
| Font abstractions | `../typst/crates/typst-library/src/text/` |

Use Typst for Rust data flow, glyph-run representation, and shaping/cache
discipline. Do not use Typst document semantics to decide Office layout
behavior.

### 2.3 Existing Rust Code Reference

Existing `ooxmlsdk-pdf` code can seed implementation details, but not the crate
design:

- `crates/ooxmlsdk-pdf/src/fonts.rs`
- `crates/ooxmlsdk-pdf/src/text_metrics.rs`
- `crates/ooxmlsdk-pdf/src/render/krilla.rs`

This code currently mixes PDF availability checks, text measurement, fallback,
and rendering needs. The new crate should split those responsibilities.

## 3. Non-Goals

`ooxmlsdk-fonts` must not own:

- paragraph/page/table layout
- PDF object writing
- OOXML package traversal
- formula evaluation
- number formatting
- document style cascade
- DOCX/PPTX/XLSX import semantics

It may accept font requests created by those layers and return resolved fonts,
metrics, glyphs, and usage records.
Do not add font behavior because a PDF backend needs a workaround. Add it when
it belongs to shared font resolution, measurement, shaping, or usage tracking,
then let the backend consume that result.

## 4. Crate Responsibility

The crate owns:

- font source registration
- system font discovery
- embedded font registration
- user-supplied font registration
- family and face matching
- Office/LO-like substitution policy
- theme-family resolution inputs
- script/language fallback
- glyph coverage checks
- vertical metrics
- advance measurement
- shaping with clusters and glyph offsets
- bidi/script-aware run splitting where needed for shaping
- underline/strikeout decoration metrics
- stable font identifiers shared by layout and PDF
- font usage records for embedding/subsetting

## 5. Dependency Direction

Preferred first version:

```text
ooxmlsdk-fonts
  no required dependency on ooxmlsdk
```

The crate should accept generic inputs:

```text
FontSource
FontRequest
ThemeFontMap
EmbeddedFontData
```

If direct OOXML embedded-font extraction becomes useful, add it as an optional
feature:

```text
features:
  ooxml = ["dep:ooxmlsdk"]
```

Do not make PDF or layout dependencies part of the font crate.

When the optional OOXML feature exists, keep it as an adapter layer. Generated
`ooxmlsdk` types may be used to extract embedded font bytes, theme font
information, and Office font request fields, but the core registry, resolver,
metrics, and shaping model must remain independent of generated schema structs.
This keeps font behavior reusable by layout, PDF, SVG, raster preview, and test
tools.

## 6. Public Model Shape

Core request:

```text
FontRequest
  family
  theme_family
  bold
  italic
  weight
  slant
  stretch
  size_pt
  script
  language
  region
  charset
  pitch
  variations
  features
```

Resolution:

```text
ResolvedFont
  font_id
  requested_family
  resolved_family
  source
  face_index
  synthetic_bold
  synthetic_italic
  metrics
```

Registry and matching:

```text
FontRegistry
  sources
  faces
  book

FontBook
  face_infos
  family_aliases
  substitution_rules
  fallback_chains

FontFaceInfo
  font_id
  family_names
  postscript_name
  style_name
  weight
  slant
  stretch
  pitch
  coverage
  flags
  axes
  features
```

This follows the LO physical-font collection idea and Typst's `FontBook`
pattern: discovery records all available faces first, then resolution chooses a
face from that book. Do not let shaping or PDF embedding be the only place that
knows which faces exist.

Shaping:

```text
ShapedRun
  font_id
  text_range
  glyphs
  advance_pt
  direction
  script
  language
  safe_breaks
  decorations
```

Glyph:

```text
ShapedGlyph
  glyph_id
  cluster
  text_range
  x_advance_pt
  y_advance_pt
  x_offset_pt
  y_offset_pt
  safe_to_break
  source_char
  justifiable
```

Usage for output:

```text
FontUsage
  font_id
  glyph_ids
  unicode_ranges
  needs_embedding
  subset_policy
  color_glyph_usage
```

`font_id` must be stable within a layout/render session so the PDF backend can
embed the same face that layout used for measurement.

String and byte ownership should be explicit:

- font family names, style names, language tags, and source identifiers may use
  `Cow<'a, str>` at API boundaries that can borrow from layout or typed OOXML
  adapters
- in-memory font data should use shared bytes such as `Arc<[u8]>` or an
  equivalent cheap-clone byte buffer once registered
- shaped glyph collections may use `Cow<'a, [ShapedGlyph]>` when reused by
  layout operations such as trimming, splitting, or display-list lowering
- numeric metrics, glyph ids, enum-like properties, and unit values should stay
  plain copyable fields

Do not force callers to allocate owned strings just to request, resolve, or
shape text.

The model must be ready for OpenType and VCL-level behavior even before those
branches are implemented:

- variable font axes and selected variation values
- OpenType feature tags and explicit feature values
- script/language fallback chains
- color glyph availability and usage
- subset policy for PDF/SVG/raster consumers
- safe break points and justifiability from shaping

## 7. Font Source Model

Support these sources from the start:

- system font database
- explicit filesystem font paths
- in-memory font bytes
- embedded OOXML font bytes supplied by caller
- test fixture fonts supplied by test harnesses

Each source should preserve:

- family names
- style names
- weight/slant/stretch where available
- face index
- raw font data or path reference
- origin priority

Font discovery must be deterministic in tests. Tests should be able to build a
registry from fixed fixture fonts instead of relying on host fonts.

## 8. Resolution Policy

Resolution is a multi-step process:

1. Resolve theme family to a requested family if a theme map is present.
2. Match requested family, weight, and slant.
3. Apply Office/LO-like substitution if no face is available.
4. Split shaping runs by script/language/direction where necessary.
5. Apply fallback per segment or per character for missing glyph coverage.
6. Mark synthetic style if bold/italic is synthesized.

Do not silently replace unavailable fonts without recording the substitution.
Layout and tests must be able to inspect the requested and resolved family.

## 9. Metrics Policy

Metrics needed by layout:

- ascent
- descent
- line gap
- ink height
- baseline offset
- glyph advances
- text run width
- underline offset and thickness
- strikeout offset and thickness

Fallback metrics are allowed only as explicit last-resort behavior. Source the
fallback constants from LibreOffice behavior when possible, and record the
source path in comments.

## 10. Shaping Policy

Use `rustybuzz`/HarfBuzz-style shaping as the implementation basis. Keep the
output format independent of `krilla`.

Shaping must preserve:

- glyph ids
- cluster mapping
- source byte/char ranges
- advances and offsets
- font fallback boundaries
- direction
- script

Layout should prefer shaped widths over character-class approximations.
Approximation may be used for incomplete early branches only when the result is
marked as approximate.

## 11. PDF Boundary

`ooxmlsdk-pdf` should not redo font selection. It should consume layout's
resolved `font_id` and shaped glyph runs, then:

- embed/subset font data
- write ToUnicode maps
- choose PDF font object types
- apply PDF/A or PDF/UA font policy
- paint glyphs

If PDF cannot embed a font that layout used, that is a PDF backend error or
policy decision. It is not a reason for PDF to choose a different measuring
font.

## 12. Layout Boundary

`ooxmlsdk-layout` should call fonts for:

- measuring text
- shaping text
- computing line heights
- computing decoration positions
- calculating column width effects in Calc
- fitting/shrink-to-fit decisions
- PPTX body text auto-fit

`ooxmlsdk-layout` owns how these metrics affect frames/pages. `ooxmlsdk-fonts`
only owns the metrics and shaped runs.

## 13. Tests

Test buckets:

- deterministic registry from fixture font bytes
- family/style matching
- fallback ordering
- missing glyph fallback
- vertical metrics
- decoration metrics
- shaping cluster ranges
- bidi/script run splitting
- stable `font_id` reuse
- layout/PDF consistency smoke through shared shaped runs

Reference tests should use:

- LO-derived expected metrics where available
- small fixture fonts for deterministic behavior
- existing `ooxmlsdk-pdf` font cases only as regression evidence, not design
  authority

## 14. Calibration Loop

Use this loop before implementing each font feature:

1. LO pass
   - identify VCL/EditEngine/Writer/Calc owner behavior
   - record the source file path
   - decide whether behavior is metric, substitution, shaping, or output policy

2. Rust pass
   - design stable request/result structs
   - keep output independent of PDF
   - keep tests deterministic by avoiding host-font assumptions

3. Consumer pass
   - prove layout can measure with the result
   - prove PDF can embed/paint the same result
   - reject any path where PDF re-resolves a different font

Repeat when a fixture reveals a substitution or fallback difference.

## 15. Logic Kickoff Plan

Start implementation with the shared path that every renderer will need:

### 15.1 Deterministic Font Registry

Reference:

- `../core/vcl/source/font/PhysicalFontCollection.cxx`
- `../core/vcl/source/font/PhysicalFontFace.cxx`
- `../typst/crates/typst-library/src/text/font/book.rs`

Scope:

- register path and in-memory sources
- parse face metadata, family names, style names, weight, slant, stretch, pitch,
  coverage, variation axes, and feature tags
- build `FontBook` family indexes and aliases
- keep system-font discovery optional for tests; fixture fonts must be enough
  to run deterministic assertions

Done when tests can construct a registry from fixed font bytes and inspect the
same face metadata independent of host fonts.

### 15.2 Face Matching And Substitution

Reference:

- `../core/vcl/source/font/PhysicalFontFace.cxx`
- `../core/vcl/source/font/PhysicalFontCollection.cxx`
- `../core/vcl/source/font/DirectFontSubstitution.cxx`

Scope:

- match requested family, style name, pitch, weight, slant, and stretch
- record candidate scores and rejection reasons in `FontMatchDiagnostics`
- apply aliases and substitution rules before last-resort fallback
- record synthetic bold/italic explicitly

Done when tests can assert requested family, resolved family, substitution
reason, fallback level, and candidate diagnostics.

### 15.3 Metrics And Shaping

Reference:

- `../core/vcl/source/font/fontmetric.cxx`
- `../core/vcl/source/gdi/`
- `../typst/crates/typst-layout/src/inline/shaping.rs`
- existing `crates/ooxmlsdk-pdf/src/text_metrics.rs` as migration evidence

Scope:

- compute vertical, decoration, script, and fallback metrics
- split runs by script/language/direction where needed
- shape with rustybuzz and preserve clusters, source ranges, safe breaks,
  offsets, advances, and justifiability
- apply glyph fallback without changing layout's measuring font silently

Done when layout can measure and shape text through `ooxmlsdk-fonts` without
calling PDF code.

### 15.4 Current Implementation Checkpoint

Implemented in this stage:

- deterministic in-memory face registration and family alias resolution
- TTF face metadata import for names, weight, slant, stretch, pitch, and
  PDF/LO-aligned vertical/decoration metric signs
- match diagnostics based on explicit attribute comparison instead of arbitrary
  weighted scores
- resolved-font metrics scaling by requested point size
- approximate shaping records that preserve clusters, source ranges, safe break
  positions, direction, script, and language

Still intentionally not implemented:

- host/system font discovery
- LO-backed substitution tables and last-resort fallback chains
- rustybuzz glyph shaping, glyph ids, advances, offsets, fallback splitting,
  and script/language itemization

Approximate shaping must keep `approximate = true` and must not invent glyph
advances or glyph ids. Real advances come only after rustybuzz/font-table
translation is implemented.
