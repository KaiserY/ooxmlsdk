# ooxmlsdk-layout Design

## 1. Goal

`ooxmlsdk-layout` is the Office layout layer for `ooxmlsdk`.

It should produce layout trees, page trees, frame fragments, display lists, and
debug dumps for DOCX, XLSX, and PPTX without depending on PDF as the design
center.

This crate is not a PDF preprocessor. It is centered on Office layout behavior:
Writer-like flow layout, Calc print layout, and Impress fixed-page layout.
Consumers may include PDF export, SVG export, raster renderers, layout dump
tests, visual inspectors, accessibility/tagging preparation, and debugging
tools. PDF is only one downstream consumer.

Target integration graph:

```text
ooxmlsdk
  -> ooxmlsdk-formula  -> value-aware XLSX input
  -> ooxmlsdk-layout   -> layout tree / display list
  -> ooxmlsdk-pdf      -> PDF output

ooxmlsdk-fonts
  -> ooxmlsdk-layout   -> metrics and shaping
  -> ooxmlsdk-pdf      -> embedding and glyph painting
```

DOCX and PPTX may enter layout without formula. XLSX should enter Calc layout
through a value-aware workbook model from `ooxmlsdk-formula`.
Fonts are a shared substrate for layout and PDF; they are not downstream of the
formula crate.

## 2. Design Authority

### 2.1 Primary Reference: LibreOffice

LibreOffice is the main design source. `ooxmlsdk-layout` should follow LO's
split between shared infrastructure and application-specific layout engines.

LO does not have one universal layout engine for Writer, Calc, and Impress.
It has shared drawing/font/output infrastructure and separate application
layout models.

| Engine | LibreOffice area |
|---|---|
| Writer/DOCX | `../core/sw/` |
| Calc/XLSX | `../core/sc/` |
| Impress/PPTX | `../core/sd/`, `../core/oox/source/ppt/`, `../core/oox/source/drawingml/` |
| Shared drawing/text | `../core/drawinglayer/`, `../core/svx/`, `../core/editeng/` |
| Shared output/font | `../core/vcl/` |

### 2.1.1 Development Discipline

Every layout change must start from both local evidence sources before code is
written:

1. current Rust code in `crates/ooxmlsdk-layout/` plus related
   `ooxmlsdk-formula`, `ooxmlsdk-fonts`, PDF, and test consumers
2. matching LibreOffice owner code under `../core/` for Writer, Calc, Impress,
   DrawingML, EditEngine, and VCL behavior

Do not implement layout behavior from memory, visual intuition, or convenient
heuristics. In particular:

- do not create layout-local formula/address parsers; extend
  `ooxmlsdk-formula` types and consume their structured output
- do not create layout-local font measurement or fallback logic; extend
  `ooxmlsdk-fonts` and keep layout/PDF on the same shaping path
- do not invent magic numbers for line height, default font size, table row
  height, page geometry, print scaling, anchor offsets, or shape text insets;
  cite the exact LO/OOXML source path or keep the field unset/defaulted
- do not write temporary "looks plausible" fallback behavior that affects page
  breaks, printed ranges, glyph positions, or shape bounds
- when only a skeleton is implemented, preserve explicit source data and
  produce conservative zero/default geometry instead of guessed dimensions

### 2.2 Typst Reference

Typst is a Rust architecture reference, not an Office behavior reference.

Use it for:

- typed regions/fragments/frames
- clean layout output objects
- separation between layout and PDF
- inline shaping pipeline shape
- grid/table implementation structure
- deterministic data flow

Useful paths:

| Area | Typst path |
|---|---|
| Paged document | `../typst/crates/typst-layout/src/document.rs` |
| Flow layout | `../typst/crates/typst-layout/src/flow/` |
| Inline layout | `../typst/crates/typst-layout/src/inline/` |
| Grid/table layout | `../typst/crates/typst-layout/src/grid/` |
| Page collection | `../typst/crates/typst-layout/src/pages/` |
| PDF conversion boundary | `../typst/crates/typst-pdf/src/` |

Do not use Typst semantics for section breaks, Writer tables, fly frames,
Calc print ranges, or PowerPoint placeholders.

### 2.3 Existing ooxmlsdk-pdf Reference

Existing `ooxmlsdk-pdf` code is not the layout design source.

It can provide:

- known fixture evidence
- partial Rust translations of LO behavior that should migrate upward
- code snippets for table/text/fly edge cases
- regression tests to preserve later

It must not define:

- crate boundaries
- public layout model
- frame hierarchy
- display-list API
- formula/font responsibilities

Relevant code paths when mining behavior:

- `crates/ooxmlsdk-layout/src/docx/layout.rs`
- `crates/ooxmlsdk-layout/src/docx/model.rs`
- `crates/ooxmlsdk-layout/src/xlsx/`
- `crates/ooxmlsdk-layout/src/pptx/`
- `crates/ooxmlsdk-pdf/src/xlsx/`

Treat this code as migration inventory and a warning about over-coupling. The
current refactor direction is to move layout behavior from `ooxmlsdk-pdf` into
`ooxmlsdk-layout` while replacing PDF-local document copies, parsers,
measurement shortcuts, and owned clones with direct typed `ooxmlsdk` traversal,
borrowed source views, shared `ooxmlsdk-formula` values, and shared
`ooxmlsdk-fonts` shaping.

## 3. Crate Responsibility

The crate owns:

- common geometry and unit types
- common display-list primitives
- layout debug dump representation
- Writer-like DOCX layout engine
- Calc-like XLSX print layout engine
- Impress-like PPTX fixed-page layout engine
- adapters from typed `ooxmlsdk` documents into layout models
- consumption of `ooxmlsdk-formula` value models for XLSX
- consumption of `ooxmlsdk-fonts` metrics and shaped runs

All of these responsibilities must remain valid without `ooxmlsdk-pdf` in the
workspace. If an API only makes sense for PDF, it belongs in `ooxmlsdk-pdf`, not
in layout.

The crate does not own:

- formula parsing/evaluation
- font discovery/shaping internals
- PDF object writing
- OOXML package read/write
- generated schema modeling

Import adapters must use `ooxmlsdk`'s typed package and generated schema APIs as
their primary input. Do not parse raw XML for fields that generated `w::*`,
`a::*`, `p::*`, `x::*`, or typed part traversal already expose. Raw XML is only
acceptable for currently unmodeled extension payloads that need structural
preservation or explicit unsupported records.

Existing `ooxmlsdk-pdf` layout/import code is migration evidence, not the input
contract. New layout code should read from typed `WordprocessingDocument`,
`SpreadsheetDocument`, `PresentationDocument`, their parts, and generated schema
roots, then produce borrowed layout views and derived layout state. Do not
recreate broad PDF-local document models in this crate when the same source
facts can be read from `ooxmlsdk` typed roots or represented by stable source
references.

## 4. Engine Split

Use one crate with shared substrate and separate application engines:

```text
ooxmlsdk-layout
  common
    geom
    units
    style
    display
    debug
  docx
    import
    model
    frame
    engine
  xlsx
    import
    model
    print
    engine
  pptx
    import
    model
    shape
    engine
```

Do not force DOCX, XLSX, and PPTX into a single core algorithm. They share
geometry, text, fonts, and display primitives, but not layout semantics.
Use OOXML-family names (`docx`, `xlsx`, `pptx`) for crate modules and public
types. Use LibreOffice Writer/Calc/Impress names in comments, tests, and source
maps when identifying the upstream concept being ported.

## 5. Dependency Direction

Target crate dependencies:

```text
ooxmlsdk-layout -> ooxmlsdk
ooxmlsdk-layout -> ooxmlsdk-fonts
ooxmlsdk-layout -> ooxmlsdk-formula
```

`ooxmlsdk-layout` must not depend on `ooxmlsdk-pdf`.

`ooxmlsdk-formula` is a direct dependency. The XLSX engine must use the shared
formula/address/value model instead of defining a parallel A1 parser or cached
value layer. DOCX and PPTX do not use formula behavior, but keeping the
dependency direct avoids feature-dependent SpreadsheetML semantics during the
current broad implementation phase.

Do not express the module graph as a single strict chain. Formula and fonts are
different support layers: formula prepares spreadsheet values, while fonts
provide metrics and shaping for all layout engines.

The dependency on `ooxmlsdk` is intentional and should be used deeply enough to
avoid duplicate OOXML models. Layout import may define DOCX/XLSX/PPTX views,
but those views should preserve links to typed source concepts and avoid
copying every string or byte payload by default.

## 5.1 Ownership And Borrowing

Use a borrowed import/view model first:

```text
DocxDocument<'doc>
XlsxWorkbook<'doc>
PptxPresentation<'doc>
```

The lifetime represents data borrowed from parsed `ooxmlsdk` package parts,
generated schema structs, formula value models, font requests, and media/font
resources.

The `ooxmlsdk` package/schema tree is the document model. Layout import models
must not become a second editable OOXML document model. Prefer direct references
to typed source structs and stable source identities:

```text
&'doc w::Paragraph / &'doc w::Run / &'doc w::Table
&'doc x::Worksheet / &'doc x::Row / &'doc x::Cell
&'doc p::Slide / &'doc p::Shape / &'doc a::Paragraph
PartId + engine-specific path indexes
```

Use `SourceRef`-style identities for hit testing, debug output, accessibility
mapping, and future editor commands. A layout result may point back to source
locations, but editing and saving must mutate the `ooxmlsdk` document model and
then invalidate/rebuild layout. Do not design a path where layout mutations are
serialized back into OOXML.

Use plain references (`&'doc str`, `&'doc [u8]`) for source payload whenever the
payload is guaranteed to come directly from `ooxmlsdk` data and does not need
computed ownership.

Use `Cow<'doc, str>` or `Cow<'doc, [u8]>` only for leaf payloads that may be
borrowed from source or computed by layout/formula/font processing:

- computed field display text, formula display text, formatted cell text, and
  line text
- image, embedded object, and font bytes only when the source can actually be
  borrowed and a backend may also supply computed/converted bytes
- display-list text and shaped glyph slices when runs are split, trimmed, or
  synthesized
- debug labels only when the label may be either a static string or a computed
  string

Use owned values for:

- computed line text after case mapping, field expansion, number formatting, or
  shared-formula translation
- generated debug dump strings
- normalized resource keys that must outlive the source document
- display documents explicitly converted to owned output
- derived layout structures such as pages, frames, line boxes, print pages,
  resolved placeholders, and unsupported records

Keep copyable data as plain fields: enums, booleans, addresses, indexes, i32/u32
values, unit wrappers, colors, geometry, transforms, and layout flags should not
use `Cow`.

Layout should provide an explicit owned conversion for consumers that need to
hold output after the source package is dropped. Do not make eager cloning the
default import behavior. Avoid cloning source strings, schema subtrees, and
binary payloads merely to simplify lifetimes; use source references and
derived-state structs instead.

## 6. Common Layer

The common layer should contain only cross-application concepts:

```text
Point
Size
Rect
Insets
Transform
Color
Stroke
Fill
PageSize
DisplayDocument
DisplayPage
DisplayItem
DebugDump
```

Common display items should be PDF-neutral:

```text
TextRun
GlyphRun
Image
Path
Rect
Line
LinkArea
AnnotationHint
Clip
Transform
```

PDF-specific concepts such as PDF object ids, annotations dictionaries,
ToUnicode CMaps, PDF/A policy, and tagged PDF structure remain in
`ooxmlsdk-pdf`.

The common display model should still be rich enough for non-PDF renderers:

- fills may be solid, theme, gradient, image, or pattern fills
- images carry crop information and relationship/resource identity
- display items can carry source paths back to engine-specific model nodes
- pages may carry a neutral background fill
- accessibility and outline hints remain neutral records, not PDF structures

## 7. DOCX Engine

The DOCX engine is for WordprocessingML flow documents. Its behavior should be
calibrated against LibreOffice Writer.

### 7.1 LO Source Map

| Area | LibreOffice path |
|---|---|
| DOCX import mapping | `../core/sw/source/writerfilter/dmapper/` |
| Page desc and section behavior | `../core/sw/source/core/layout/pagechg.cxx`, `../core/sw/source/core/layout/pagedesc.cxx` |
| Layout action | `../core/sw/source/core/layout/layact.cxx` |
| Flow frames | `../core/sw/source/core/layout/flowfrm.cxx` |
| Text frames | `../core/sw/source/core/text/txtfrm.cxx`, `../core/sw/source/core/text/frmform.cxx` |
| Line formatting | `../core/sw/source/core/text/itrform2.cxx` |
| Widow/orphan | `../core/sw/source/core/text/widorp.cxx` |
| Tables | `../core/sw/source/core/layout/tabfrm.cxx` |
| Fly frames | `../core/sw/source/core/layout/fly.cxx`, `../core/sw/source/core/layout/flycnt.cxx` |
| Anchored objects | `../core/sw/source/core/layout/anchoredobject.cxx` |
| Text wrap around flys | `../core/sw/source/core/text/txtfly.cxx` |
| Footnotes/endnotes | `../core/sw/source/core/layout/ftnfrm.cxx`, `../core/sw/source/core/text/txtftn.cxx` |
| Header/footer | `../core/sw/source/core/layout/hffrm.cxx` |
| Layout XML dump | `../core/sw/source/core/layout/xmldump.cxx` |

### 7.2 Model Shape

Use a Writer-like imported model with OOXML-facing names:

```text
DocxDocument
  settings
  styles
  numbering
  resources
  sections
  notes
  comments
```

```text
DocxSection
  break_kind
  page_desc
  columns
  headers
  footers
  body_blocks
```

```text
DocxBlock
  Paragraph
  Table
  FloatingFrame
```

DOCX paragraph and inline state should preserve:

- bookmarks, hyperlinks, comments, footnote/endnote references, and field runs
- tabs, paragraph direction, document grid, hyphenation settings, and outline
  levels
- character spacing, caps/small-caps, highlight, underline/strikeout, and
  baseline shifts
- anchored-object reference kind, alignment, offsets, and wrap mode

### 7.3 Frame Tree

The engine should produce a frame tree close to Writer concepts:

```text
DocxFrameTree
  Root
    Page
      Header
      Body
        Section
        Column
        Text
        Table
          Row
          Cell
        Fly
      Footer
      Footnote
```

Follows and splits are first-class:

```text
FrameFollow
  master_frame
  follow_frame
  reason
  split_cursor
```

Text line output should expose portions close to Writer's line formatter:

```text
DocxTextLine
  text_range
  bounds
  baseline
  portions

DocxTextPortion
  Text
  Field
  Tab
  Numbering
  Bullet
  SoftHyphen
  Hidden
  Bookmark
  Comment
  ControlChar
  Combined
  Ruby
  Break
  Footnote
  Fly
```

### 7.4 Algorithm Principles

- Use LO's flow-frame behavior as the source for move forward/backward.
- Keep page/column/section transitions explicit.
- Model keep-with-next, keep-together, widow/orphan, and explicit breaks before
  painting.
- Model table rows and cells as frames, not as painted rectangles only.
- Model fly frames and wrap influence before final line layout.
- Model footnote reservation and continuation as layout state, not PDF state.
- Produce a layout debug dump before producing a display list.

## 8. XLSX Engine

The XLSX engine is for SpreadsheetML sheet print layout. Its behavior should be
calibrated against LibreOffice Calc.

### 8.1 LO Source Map

| Area | LibreOffice path |
|---|---|
| XLSX workbook import | `../core/sc/source/filter/oox/workbookfragment.cxx` |
| Worksheet import | `../core/sc/source/filter/oox/worksheetfragment.cxx` |
| Sheet data import | `../core/sc/source/filter/oox/sheetdatacontext.cxx` |
| Page settings import | `../core/sc/source/filter/oox/pagesettings.cxx` |
| Styles import | `../core/sc/source/filter/oox/stylesbuffer.cxx` |
| Drawing anchors | `../core/sc/source/filter/oox/drawingbase.cxx`, `../core/sc/source/filter/oox/drawingfragment.cxx` |
| Print ranges and pages | `../core/sc/source/ui/view/printfun.cxx` |
| Sheet output | `../core/sc/source/ui/view/output.cxx` |
| Core document/table data | `../core/sc/source/core/data/` |

### 8.2 Input Boundary

XLSX layout consumes a value-aware workbook:

```text
XlsxWorkbook
  sheets
  styles
  page_styles
  drawings
  print_plan
  value_provider
```

The value provider should come from `ooxmlsdk-formula` for formula cells and
from import/style code for constants and formatted display strings.

XLSX layout must not parse or evaluate formulas.
It may parse cell and range references only through `ooxmlsdk-formula` address
types so sheet names, absolute flags, whole-row ranges, and whole-column ranges
do not diverge from dependency graph semantics.

XLSX import should still use generated SpreadsheetML types for worksheet
structure, page settings, columns, rows, merges, hyperlinks, drawings, tables,
and relationships. The formula crate supplies values and formula state; it does
not replace typed sheet import.

### 8.3 Responsibilities

The XLSX engine owns:

- sheet print order
- visible sheet selection
- print ranges
- repeated rows/columns
- manual and automatic page breaks
- page scaling and fit-to-page
- row heights and column widths
- merged cell geometry
- wrapped/shrink-to-fit cell text layout
- gridlines and cell borders
- header/footer placement
- drawing anchors relative to cells
- filters, tables, conditional-format visible effects, sheet protection ranges,
  and freeze/split pane view state when they affect printed output
- comments/notes if printable

Formula values, number formatting, and stale-cache policy are inputs, not print
layout algorithms.

Calc geometry should stay in LibreOffice's twips-based unit family. Column and
row sizes should be imported or derived as twips/points, not as ad hoc pixel or
character-width guesses:

- default column width follows LO `sc/inc/global.hxx` `STD_COL_WIDTH`
  (`64pt`, `1280twips`) until style/font-derived column metrics are ported
- explicit OOXML column character widths are converted into the Calc twips
  model before page/fragment bounds are computed
- default row height follows `sheetFormatPr@defaultRowHeight`; for MSO-style
  imports LO rounds down to a `0.75pt` grid in
  `sc/source/filter/oox/worksheetfragment.cxx`
- if `sheetFormatPr` is absent, use the standard Excel/Calc `15pt` fallback
  until style-font-derived `ScGlobal::nStdRowHeight` behavior is ported
- paper bounds should prefer page style and paper size metadata; A4 is only a
  Calc print fallback, matching LO's `ScPrintFunc` fallback path

The engine should expose print output before lowering to the common display
list:

```text
XlsxPrintPlan
  printed_sheets

XlsxPrintPage
  sheet_range
  paper_bounds
  content_bounds
  cell_fragments
  drawing_fragments
```

This mirrors Calc's split between sheet data/import and `ScPrintFunc` /
`ScOutputData` visible output.

Cell layout state should preserve alignment, text rotation, wrapping,
shrink-to-fit, row/column hidden state, merged ranges, and rich text runs. Notes,
tables, filters, conditional formats, and protected ranges are layout inputs
when they affect visible output or debug dumps; formula evaluation remains in
`ooxmlsdk-formula`.

## 9. PPTX Engine

The PPTX engine is for PresentationML fixed-page slide layout. Its behavior
should be calibrated against LibreOffice Impress.

### 9.1 LO Source Map

| Area | LibreOffice path |
|---|---|
| PPTX import | `../core/oox/source/ppt/pptimport.cxx` |
| Presentation fragment | `../core/oox/source/ppt/presentationfragmenthandler.cxx` |
| Slide fragment | `../core/oox/source/ppt/slidefragmenthandler.cxx` |
| Layout fragment | `../core/oox/source/ppt/layoutfragmenthandler.cxx` |
| Slide persist | `../core/oox/source/ppt/slidepersist.cxx` |
| PPT shape context | `../core/oox/source/ppt/pptshapecontext.cxx` |
| DrawingML shape | `../core/oox/source/drawingml/shape.cxx` |
| DrawingML text | `../core/oox/source/drawingml/text*.cxx` |
| DrawingML table | `../core/oox/source/drawingml/table/` |
| Impress model/render tests | `../core/sd/qa/` |

### 9.2 Responsibilities

The PPTX engine owns:

- slide size
- slide order and visibility
- master/layout/theme inheritance
- placeholder resolution
- shape tree order
- group transforms
- slide background, transitions, custom shows, and timing tree records when
  imported
- DrawingML text body layout
- bodyPr insets, columns, anchors, text rotation
- table cell text and borders
- image/chart/diagram placeholder display records
- notes/handout layout when in scope

PPTX is fixed-page drawing layout. Do not route it through the Writer flow
engine.

The PPTX model must keep non-text drawing objects structured enough for later
renderers and tests:

```text
PptxShape
  Shape
  Group
  Picture
  Media
  GraphicFrame
  Table
  Chart
  Diagram
  OleObject
  Connector
```

Tables should have row/cell geometry and text bodies as first-class model data,
not be flattened immediately to painted rectangles. Chart, diagram, media, and
OLE objects may initially be placeholders, but they should remain typed
placeholders with relationship ids instead of generic unsupported strings.

DrawingML shape state should preserve custom geometry, adjustment values, fill,
line, effects, hidden/decorative flags, placeholder source, theme style
references, text body properties, paragraph spacing, bullet kinds, and text
vertical mode. Rendering may be incomplete, but the model should not discard
this information during import.

## 10. Display List Boundary

Each engine should lower layout output into a shared display document:

```text
LayoutDocument
  engine_kind
  pages
  frames
  debug_records
```

```text
DisplayDocument
  pages
  resources
  outlines
  links
  accessibility_hints
```

`DisplayDocument` is still layout-owned and PDF-neutral. PDF rendering is a
consumer.

Other consumers must be kept realistic during design:

- layout dump and XPath-like assertions
- SVG/debug visualization
- raster preview or screenshot testing
- accessibility/tagging preparation
- object inspection APIs
- PDF rendering

If a display primitive only works for PDF, redesign it as a neutral primitive
or move it to the PDF backend.

Display-list data may retain borrowed text and bytes through `Cow` while it is
owned by the layout session. PDF, SVG, raster, and debug consumers should accept
that borrowed display document. If a backend needs a `'static` artifact, it must
request or build an owned display document explicitly.

Text display primitives should prefer font-shaped runs from `ooxmlsdk-fonts`.
They may carry borrowed source text for accessibility/debugging and shaped glyph
collections by `Cow<'doc, [ShapedGlyph]>` when runs are split or trimmed without
reshaping. PDF must consume the same resolved font ids and glyph runs instead
of re-resolving text.

## 11. Debug Dump Boundary

The crate should provide LO-style debug dumps early. This is essential because
many LO tests assert layout, not PDF.

DOCX dump should expose Writer-calibrated state:

- page count
- frame tree
- frame type
- bounds
- print bounds
- text line boxes
- table row/cell fragments
- fly frame position
- follow chains
- footnote/header/footer frames

XLSX dump should expose Calc-calibrated state:

- printed sheet pages
- print ranges
- row/column positions
- cell rectangles
- page breaks
- drawing anchors
- header/footer slots

PPTX dump should expose Impress-calibrated state:

- slide pages
- shape tree
- resolved placeholder source
- transforms
- text body boxes
- table cell boxes
- display order

## 12. Testing Strategy

### 12.1 DOCX Tests

Use LO layout-level tests first:

- `../core/sw/qa/core/layout/`
- `../core/sw/qa/core/text/`
- `../core/sw/qa/core/objectpositioning/`
- `../core/sw/qa/core/header_footer/`
- `../core/sw/qa/extras/layout/`
- `../core/sw/qa/extras/ooxmlimport/`
- `../core/sw/qa/extras/ooxmlexport/` cases using `parseLayoutDump()`

Many rows currently listed as PDF projections in
`../ooxmlsdk-test-suite/docs/ooxmlsdk-pdf-test/LibreOffice.md` should move
to layout tests when this crate exists.

### 12.2 XLSX Tests

Use Calc import/model tests only when they affect printed layout. Keep pure
model tests outside layout.

Good layout candidates:

- row/column metric tests
- print range/page break tests
- header/footer print tests
- visible sheet selection
- merged-cell print geometry
- drawing-anchor output
- formatted cell display text

### 12.3 PPTX Tests

Use PPTX tests where the asserted behavior is visible layout or fixed-page
object structure:

- placeholder inheritance
- text body geometry
- slide/master/layout resolution
- table geometry
- grouped transforms
- notes/handout visible output

Pure export XML round-trip tests stay outside layout.

### 12.4 LibreOffice Coverage Gate

During the migration from `ooxmlsdk-pdf` to `ooxmlsdk-layout`, keep
`../ooxmlsdk-test-suite/crates/ooxmlsdk-pdf-test` as the temporary regression gate. The current PDF
suite contains many source-backed LO regressions and should catch behavior
breaks while layout and formula behavior are moved upward:

- DOCX line, page, table, header/footer, floating object, field, redline, and
  drawing placement
- XLSX row/column metrics, page setup, print ranges, visible sheet/cell text,
  drawing anchors, filters, tables, and pivot/table display output
- PPTX slide/master/layout resolution, placeholder inheritance, text-body
  geometry, grouped shape transforms, tables, SmartArt, and theme-derived
  drawing output

Treat the existing PDF projection as evidence and temporary safety coverage, not
as the final owner of layout behavior. As each behavior stabilizes in
`ooxmlsdk-layout`, identify the earliest stable layout/debug representation that
should eventually own the assertion. PDF text/path/image/raster checks remain
useful during migration and should later narrow to renderer/PDF-object behavior.

The temporary acceptance gate for each migrated behavior is:

- the behavior is represented in `ooxmlsdk-layout` with a LO-backed model
- formula/address/value behavior used by the layout path lives in
  `ooxmlsdk-formula`
- `ooxmlsdk-pdf` consumes the migrated layout/formula state without keeping a
  competing PDF-local parser, document model, or layout algorithm
- the existing `../ooxmlsdk-test-suite/crates/ooxmlsdk-pdf-test` coverage still passes or the failure
  is explained by a deliberate, LO-backed expectation update
- expected values come from LO assertions, layout dumps, rendered metafile/
  bitmap evidence, or fixture facts; never from current Rust PDF output
- unsupported behavior is recorded structurally, so a test can fail on a
  missing LO concept instead of passing through dropped output
- planned final-owner tests are recorded for the later layout/formula/PDF test
  redesign

## 13. Development Stages

### Stage 1: Common Substrate

- geometry and units
- display-list types
- debug dump infrastructure
- `ooxmlsdk-fonts` integration

### Stage 2: DOCX Minimal Core

- sections/pages
- paragraphs
- text lines
- page overflow
- frame tree
- debug dump tests

### Stage 3: DOCX Tables and Follows

- table frame tree
- row/cell geometry
- row splitting
- repeated headers
- border conflict records

### Stage 4: DOCX Flys and Notes

- anchored objects
- wrap influence
- headers/footers
- footnotes/endnotes
- reflow/move backward

### Stage 5: XLSX Print Core

- workbook/sheet print model
- value-provider integration
- row/column/page geometry
- merged cells and page breaks

### Stage 6: PPTX Fixed Pages

- slide pages
- shape tree
- placeholders
- text body layout
- table/shape display list

### Stage 7: PDF Migration

Migrate `ooxmlsdk-pdf` to consume `ooxmlsdk-layout` outputs incrementally. Each
migrated area should delete or bypass the corresponding PDF-local import,
formula, or layout code rather than wrapping it in another intermediate model.

Do not use PDF migration pressure to weaken the layout model. The model is
validated by LO-style source evidence, layout/debug records, and temporary
`ooxmlsdk-pdf-test` parity until the dedicated layout/formula/PDF test split is
rebuilt.

## 14. Calibration Loop

Run this loop for each new feature area:

0. Existing-code preflight
   - inspect the current `docx`, `xlsx`, `pptx`, and `common` modules before
     adding new import, geometry, parser, or display structures
   - reuse `ooxmlsdk-formula` for SpreadsheetML references and values; do not
     create layout-local A1 parsers, formula caches, or dependency models
   - reuse `ooxmlsdk-fonts` for text measurement and shaping; do not add
     renderer-specific width estimates to layout unless they are explicit,
     LO-sourced fallback behavior
   - search for existing unit conversions and constants before adding numeric
     defaults, and attach LO/OOXML source paths to any fallback that remains
   - record in the change summary which existing APIs were reused or extended
   - inspect the matching LO owner files in `../core/sw/`, `../core/sc/`,
     `../core/sd/`, `../core/oox/`, `../core/editeng/`, or `../core/vcl/`
     during the same pass; every new geometry default, line metric, repeated
     range rule, shape rule, or print-layout shortcut must be source-backed

1. LO design pass
   - identify the application owner: Writer, Calc, or Impress
   - list exact source files
   - name the LO concept being modeled

2. Rust architecture pass
   - map LO concepts to Rust structs/enums
   - keep engine-specific behavior out of common
   - keep display output PDF-neutral

3. Fixture/test pass
   - select upstream layout/render tests
   - assert layout dump before display/PDF
   - record unsupported behavior as structured state

Repeat after implementation review. If the design starts copying
`ooxmlsdk-pdf` module shape or adding owned clone-heavy intermediate models,
restart from the LO source map and the typed `ooxmlsdk` source boundary.

## 15. Logic Kickoff Plan

Start layout logic by proving each engine can import typed `ooxmlsdk` input and
emit inspectable layout/debug state. PDF backend migration should happen
incrementally only after the corresponding layout/formula owner exists, and the
existing PDF tests should remain the temporary regression gate for migrated
behavior.

### 15.0 Next Broad Development Focus

The next implementation cycle is a split-and-performance refactor. Move layout
capabilities currently embedded in `ooxmlsdk-pdf` into `ooxmlsdk-layout`, move
formula/value capabilities into `ooxmlsdk-formula`, and keep
`ooxmlsdk-pdf-test` as the temporary behavior gate while the dedicated
layout/formula/PDF test split is redesigned.

The cycle should advance four large areas together while keeping shared
ownership boundaries intact:

1. XLSX Calc print core
   - expand `XlsxWorkbook` typed import into a Calc-like print model
   - keep values and references supplied by `ooxmlsdk-formula`
   - remove duplicated PDF-local formula/address/display-value helpers as their
     behavior moves into `ooxmlsdk-formula`
   - port row/column twips geometry, print ranges, repeats, page breaks,
     fit-to-page, merged-cell bounds, hidden row/column handling, and
     header/footer placement from `../core/sc/source/ui/view/printfun.cxx`,
     `../core/sc/source/ui/view/output.cxx`, and
     `../core/sc/source/filter/oox/worksheetfragment.cxx`

2. Shared text/font shaping pipeline
   - route DOCX/XLSX/PPTX text measurement through `ooxmlsdk-fonts`
   - avoid local width heuristics except explicit LO-sourced fallback states
   - keep shaped glyph runs reusable by PDF/SVG/raster consumers

3. DOCX Writer frame and flow skeleton
   - move from typed import records toward Writer-like pages, body frames,
     paragraphs, line boxes, table frames, headers/footers, notes, and anchored
     objects
   - migrate PDF-local Writer layout behavior upward without cloning broad
     `ooxmlsdk` schema subtrees into a second document model
   - use `../core/sw/source/core/layout/`,
     `../core/sw/source/core/text/itrform2.cxx`, and
     `../core/sw/source/writerfilter/dmapper/` as the owner sources

4. PPTX fixed-page shape and text layout
   - expand slide display construction from typed shape-tree records
   - migrate PDF-local Impress/Draw import and fixed-page layout behavior into
     borrowed typed-source views and derived display state
   - port placeholder inheritance, text body properties, paragraph/run style
     cascade, basic preset geometry, picture/graphic-frame bounds, and text
     autofit from `../core/oox/source/drawingml/`,
     `../core/oox/source/ppt/`, and `../core/sd/`

Before implementing any item, run the calibration loop below. In particular,
read the existing `ooxmlsdk-layout`, `ooxmlsdk-formula`, and relevant
`ooxmlsdk-pdf` modules as migration inventory; then extend shared APIs instead
of adding a second parser, formatter, unit converter, font measurer, geometry
model, or owned document copy.

### 15.1 Shared Layout Substrate

Reference:

- `../typst/crates/typst-layout/src/document.rs`
- `../typst/crates/typst-layout/src/pages/`
- `../typst/crates/typst-pdf/src/convert.rs` for consumer-boundary shape only

Scope:

- keep geometry, units, display items, source links, unsupported records, and
  debug dump records independent of any renderer
- add engine entry points that return `LayoutDocument` / `DisplayDocument`
  without PDF objects
- route all text measurement and shaping through `ooxmlsdk-fonts`

Done when DOCX/XLSX/PPTX smoke imports can emit empty or minimal layout
documents with stable debug records and no PDF dependency.

### 15.2 DOCX Typed Import And Minimal Flow

Reference:

- `../core/sw/source/writerfilter/dmapper/`
- `../core/sw/source/core/layout/pagechg.cxx`
- `../core/sw/source/core/layout/pagedesc.cxx`
- `../core/sw/source/core/text/itrform2.cxx`
- `../core/sw/source/core/text/inftxt.hxx`

Scope:

- import settings, sections, page descriptions, headers/footers, paragraphs,
  runs, fields, bookmarks, notes, tables, and inline/floating shapes from typed
  `WordprocessingDocument`
- build a minimal frame tree: root, pages, body, section, text frames, and line
  records
- line breaking may start conservative, but text metrics must come from
  `ooxmlsdk-fonts`
- expose LO-style debug records before lowering to display items

Done when simple DOCX fixtures produce page count, frame tree, text line boxes,
and display text without using `ooxmlsdk-pdf`.

### 15.3 XLSX Typed Import And Print Plan

Reference:

- `../core/sc/source/filter/oox/workbookfragment.cxx`
- `../core/sc/source/filter/oox/worksheetfragment.cxx`
- `../core/sc/source/filter/oox/sheetdatacontext.cxx`
- `../core/sc/source/ui/view/printfun.cxx`
- `../core/sc/source/ui/view/output.cxx`

Scope:

- import workbook/sheet print state, page settings, rows, columns, cells,
  merges, hyperlinks, tables, filters, notes, drawings, and visible styles from
  typed `SpreadsheetDocument`
- consume `ooxmlsdk-formula` only through value-provider/display-value APIs
- compute row/column geometry, print ranges, repeated rows/columns, page
  breaks, and `XlsxPrintPlan`
- lower cells and drawings to neutral display/debug records

Done when simple XLSX fixtures produce printed sheet pages, cell rectangles,
formatted display text, and page-break debug records without formula parsing in
layout.

### 15.4 PPTX Typed Import And Fixed Pages

Reference:

- `../core/oox/source/ppt/pptimport.cxx`
- `../core/oox/source/ppt/presentationfragmenthandler.cxx`
- `../core/oox/source/ppt/slidefragmenthandler.cxx`
- `../core/oox/source/ppt/slidepersist.cxx`
- `../core/oox/source/drawingml/shape.cxx`
- `../core/oox/source/drawingml/textbody.cxx`
- `../core/oox/source/drawingml/textparagraphpropertiescontext.cxx`

Scope:

- import slide order, size, masters, layouts, themes, placeholders, backgrounds,
  shape trees, group transforms, text bodies, tables, media/chart/diagram/OLE
  placeholders, notes, transitions, and timing records from typed
  `PresentationDocument`
- resolve placeholder inheritance enough to expose source records in debug
  output
- produce fixed pages with shape order, bounds, text boxes, table boxes, and
  display items

Done when simple PPTX fixtures produce slide pages, shape-tree debug records,
resolved placeholders, and basic text display without PDF.

### 15.5 PDF Migration Gate

Migrate `ooxmlsdk-pdf` incrementally once the target behavior has a
typed-source layout/formula owner. For each migrated slice:

- fonts resolve and shape through `ooxmlsdk-fonts`
- XLSX values and references come from `ooxmlsdk-formula`
- layout produces display/debug output for the affected DOCX, XLSX, or PPTX
  fixture path
- the existing `../ooxmlsdk-test-suite/crates/ooxmlsdk-pdf-test` coverage for that behavior remains
  green, or the changed expectation is documented with LO-backed evidence
- PDF-local duplicate layout/formula code is removed, bypassed, or clearly
  marked as pending migration inventory
- unsupported features are recorded as structured records instead of being
  silently dropped

After the broad migration refactor is complete, redesign tests by owner:
formula import/evaluation tests in `ooxmlsdk-formula`, layout/debug tests in
`ooxmlsdk-layout` or `../ooxmlsdk-test-suite`, and PDF object/rendering tests in
`ooxmlsdk-pdf-test`.

### 15.6 Current Implementation Checkpoint

Implemented in this stage:

- DOCX/XLSX/PPTX layout entry points that emit `LayoutDocument` debug records
  without depending on `ooxmlsdk-pdf`
- XLSX `XlsxPrintPlan` construction for visible sheets, explicit print ranges,
  LO-style implicit A1-start occupied ranges, hidden rows, merged-range
  expansion, and printable drawing anchors
- XLSX manual `rowBreaks`/`colBreaks` split print ranges into multiple
  `XlsxPrintPage` records using LibreOffice's `IsStartOfNewPage` import
  behavior: only manual breaks become hard page starts, and `brk@id` is kept as
  the start row/column of the following page
- XLSX repeated rows/columns from `_xlnm.Print_Titles` are carried on each
  print page and included in page cell fragments with address de-duplication
- `layout_xlsx_model` consumes the print plan for cell debug records instead of
  re-parsing formula or inventing display text
- `layout_xlsx_model` now creates display pages from `XlsxPrintPage`
  `paper_bounds` and emits cached/display cell text as `TextRun` items from
  the formula value model
- DOCX and PPTX expose `layout_*_model_with_fonts` entry points that route text
  through `ooxmlsdk-fonts::FontRegistry::shape_text_runs`
- the display list can emit shared `GlyphRun` items with shaped glyphs,
  resolved font ids, fallback diagnostics, and run advances while preserving
  the old text-only entry points for callers without a font registry

Still intentionally not implemented:

- full paper size mapping beyond the current LO-sourced fallback geometry,
  automatic Calc pagination from page content size, scaling/fit-to-page, and
  cross-page merged-cell splitting
- full DOCX line layout, PPTX text-body layout, and XLSX cell text shaping
- any PDF adapter migration

Unknown geometry must remain zero/default until a LO-backed page setup,
row/column, or drawing anchor rule is translated.
