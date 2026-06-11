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
- partial Rust translations of LO behavior
- code snippets for table/text/fly edge cases
- regression tests to preserve later

It must not define:

- crate boundaries
- public layout model
- frame hierarchy
- display-list API
- formula/font responsibilities

Relevant code paths when mining behavior:

- `crates/ooxmlsdk-pdf/src/layout/mod.rs`
- `crates/ooxmlsdk-pdf/src/docx/model.rs`
- `crates/ooxmlsdk-pdf/src/xlsx/`
- `crates/ooxmlsdk-pdf/src/pptx/`

Treat this code as migration evidence and a warning about over-coupling.

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
  writer
    import
    model
    frame
    engine
  calc
    import
    model
    print
    engine
  impress
    import
    model
    shape
    engine
```

Do not force DOCX, XLSX, and PPTX into a single core algorithm. They share
geometry, text, fonts, and display primitives, but not layout semantics.

## 5. Dependency Direction

Target crate dependencies:

```text
ooxmlsdk-layout -> ooxmlsdk
ooxmlsdk-layout -> ooxmlsdk-fonts
ooxmlsdk-layout -> ooxmlsdk-formula
```

`ooxmlsdk-layout` must not depend on `ooxmlsdk-pdf`.

If the formula dependency becomes too heavy for DOCX/PPTX-only consumers, gate
Calc formula integration behind a Cargo feature while keeping the intended XLSX
pipeline value-aware.

Do not express the module graph as a single strict chain. Formula and fonts are
different support layers: formula prepares spreadsheet values, while fonts
provide metrics and shaping for all layout engines.

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

## 7. Writer Engine

The Writer engine is for DOCX and Writer-like flow documents.

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

Use a Writer-like imported model:

```text
WriterDocument
  settings
  styles
  numbering
  resources
  sections
```

```text
WriterSection
  break_kind
  page_desc
  columns
  headers
  footers
  body_blocks
```

```text
WriterBlock
  Paragraph
  Table
  FloatingFrame
```

### 7.3 Frame Tree

The engine should produce a frame tree close to Writer concepts:

```text
RootFrame
  PageFrame
    HeaderFrame
    BodyFrame
      SectionFrame
      ColumnFrame
      TextFrame
      TableFrame
        RowFrame
        CellFrame
      FlyFrame
    FooterFrame
    FootnoteFrame
```

Follows and splits are first-class:

```text
FrameFollow
  master_frame
  follow_frame
  reason
  split_cursor
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

## 8. Calc Engine

The Calc engine is for XLSX sheet print layout.

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

Calc layout consumes a value-aware workbook:

```text
CalcWorkbook
  sheets
  styles
  page_styles
  drawings
  value_provider
```

The value provider should come from `ooxmlsdk-formula` for formula cells and
from import/style code for constants and formatted display strings.

Calc layout must not parse or evaluate formulas.

### 8.3 Responsibilities

The Calc engine owns:

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
- comments/notes if printable

Formula values, number formatting, and stale-cache policy are inputs, not print
layout algorithms.

## 9. Impress Engine

The Impress engine is for PPTX fixed-page slide layout.

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

The Impress engine owns:

- slide size
- slide order and visibility
- master/layout/theme inheritance
- placeholder resolution
- shape tree order
- group transforms
- DrawingML text body layout
- bodyPr insets, columns, anchors, text rotation
- table cell text and borders
- image/chart/diagram placeholder display records
- notes/handout layout when in scope

PPTX is fixed-page drawing layout. Do not route it through the Writer flow
engine.

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

## 11. Debug Dump Boundary

The crate should provide LO-style debug dumps early. This is essential because
many LO tests assert layout, not PDF.

Writer dump should expose:

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

Calc dump should expose:

- printed sheet pages
- print ranges
- row/column positions
- cell rectangles
- page breaks
- drawing anchors
- header/footer slots

Impress dump should expose:

- slide pages
- shape tree
- resolved placeholder source
- transforms
- text body boxes
- table cell boxes
- display order

## 12. Testing Strategy

### 12.1 Writer Tests

Use LO layout-level tests first:

- `../core/sw/qa/core/layout/`
- `../core/sw/qa/core/text/`
- `../core/sw/qa/core/objectpositioning/`
- `../core/sw/qa/core/header_footer/`
- `../core/sw/qa/extras/layout/`
- `../core/sw/qa/extras/ooxmlimport/`
- `../core/sw/qa/extras/ooxmlexport/` cases using `parseLayoutDump()`

Many rows currently listed as PDF projections in
`docs/tests/ooxmlsdk-pdf-test/libreoffice/UPSTREAM_TEST_MATRIX.md` should move
to layout tests when this crate exists.

### 12.2 Calc Tests

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

### 12.3 Impress Tests

Use PPTX tests where the asserted behavior is visible layout or fixed-page
object structure:

- placeholder inheritance
- text body geometry
- slide/master/layout resolution
- table geometry
- grouped transforms
- notes/handout visible output

Pure export XML round-trip tests stay outside layout.

## 13. Development Stages

### Stage 1: Common Substrate

- geometry and units
- display-list types
- debug dump infrastructure
- `ooxmlsdk-fonts` integration

### Stage 2: Writer Minimal Core

- sections/pages
- paragraphs
- text lines
- page overflow
- frame tree
- debug dump tests

### Stage 3: Writer Tables and Follows

- table frame tree
- row/cell geometry
- row splitting
- repeated headers
- border conflict records

### Stage 4: Writer Flys and Notes

- anchored objects
- wrap influence
- headers/footers
- footnotes/endnotes
- reflow/move backward

### Stage 5: Calc Print Core

- workbook/sheet print model
- value-provider integration
- row/column/page geometry
- merged cells and page breaks

### Stage 6: Impress Fixed Pages

- slide pages
- shape tree
- placeholders
- text body layout
- table/shape display list

### Stage 7: PDF Integration

Only after layout outputs are stable, migrate `ooxmlsdk-pdf` to consume
`DisplayDocument`.

Do not use PDF migration pressure to change the layout model. The model is
validated first by LO-style layout dumps and engine-specific layout tests; PDF
parity is a later consumer check.

## 14. Calibration Loop

Run this loop for each new feature area:

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
`ooxmlsdk-pdf` module shape, restart from the LO source map.
