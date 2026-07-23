# DOCX to PDF Rendering Status and Plan

## 1. Goal

Build a DOCX-to-PDF renderer that follows LibreOffice Writer's document and
layout behavior as closely as practical while using `ooxmlsdk`'s generated
OOXML type system as the source model.

This is a DOCX-first plan. The existing XLSX and PPTX PDF entry points are
out of scope for this work and should not be extended while the DOCX renderer
core is being corrected.

The target pipeline is:

```text
WordprocessingDocument
  -> typed DOCX import model
  -> Writer-like document model
  -> Writer-like page/frame layout
  -> PDF paint model
  -> krilla PDF output
```

The current `ooxmlsdk-pdf` implementation is no longer a smoke-only DOCX PDF
path. It has a typed DOCX import model, a Writer-shaped frame/layout substrate,
explicit layout-to-paint conversion, and a LibreOffice-derived calibration
surface. It is still not full LibreOffice parity: exact text layout,
footnote/table/fly interactions, compatibility flags, drawing effects, and PDF
export options remain active implementation areas.

### 1.1 Module Boundary Status

The DOCX renderer is partially split along the major responsibilities that
appear in LibreOffice and Typst. New behavior should continue to follow these
boundaries instead of making `docx/mod.rs` the only owner again:

- `docx/model.rs`: Writer-like imported document, section, paragraph, table,
  floating frame, DrawingML/VML textbox, image, and style value carriers. This
  now includes typed placement carriers shared by floating images, floating
  tables, paragraph `framePr`, and textbox frame content.
- `docx/properties.rs`: typed OOXML property resolution for `pPr`, `rPr`,
  `tblPr`, `trPr`, and `tcPr`, following `writerfilter/dmapper` overlay order
  where that overlay has already been ported.
- `docx/text.rs`: paragraph/run import, fields, notes, numbering labels,
  symbols, tabs, and inline split markers, aligned with Writer text-frame input.
- `docx/table.rs`: table look and conditional-style helpers, aligned with
  `DomainMapperTableManager`, `TblStylePrHandler`, and `SwTabFrame`. Some
  table construction logic still remains in `docx/mod.rs`.
- `docx/drawing.rs`: DrawingML/VML image, textbox, wrap, anchor, crop, and
  transform entry points, aligned with Writer fly-frame import and text-wrap
  data. Lower-level XML fallbacks and geometry helpers are still being drained
  out of `docx/mod.rs`.
- `docx/package.rs`: package-part traversal for headers, footers, notes,
  comments, images, hyperlinks, styles, numbering, and settings.

Keep each split behavior-preserving unless the same patch intentionally adds a
documented Writer-aligned behavior. This avoids hiding layout regressions inside
file moves.

## 2. Authorities

### 2.1 Primary Behavior Reference

LibreOffice is the behavior reference for DOCX import, pagination, layout, and
Writer PDF rendering.

Important local reference areas:

| Area | LibreOffice path |
|------|------------------|
| DOCX import mapping | `../core/sw/source/writerfilter/dmapper/` |
| Section/page/header/footer mapping | `../core/sw/source/writerfilter/dmapper/PropertyMap.cxx` |
| Writer layout action | `../core/sw/source/core/view/viewsh.cxx` |
| Print/PDF page preparation | `../core/sw/source/core/view/vprint.cxx` |
| Page selection for print/PDF | `../core/sw/source/core/doc/doc.cxx` |
| Text frame layout and breaks | `../core/sw/source/core/text/frmform.cxx` |
| Footnote layout | `../core/sw/source/core/text/txtftn.cxx` |
| Floating object text wrap | `../core/sw/source/core/text/txtfly.cxx` |
| Table layout | `../core/sw/source/core/layout/tabfrm.cxx` |

Use LibreOffice logic to decide behavior order and edge cases. Do not invent
layout rules when LibreOffice has an existing rule.

### 2.2 Source Data Reference

Use this repository's generated `ooxmlsdk` types and package APIs:

- `WordprocessingDocument`
- `MainDocumentPart`
- generated `schemas_openxmlformats_org_wordprocessingml_2006_main` types
- generated DrawingML / WordprocessingDrawing / VML types
- typed part relationship APIs for headers, footers, notes, images, styles,
  numbering, and settings

Avoid raw XML scans when a generated type exposes the data. Raw XML fallback is
acceptable only for content that the generated model currently stores in
`xml_children` or otherwise cannot expose structurally.

### 2.3 PDF Technique Reference

Typst is a reference for PDF painting mechanics, not DOCX layout behavior.
Useful areas:

| Area | Typst path |
|------|------------|
| Paged document to PDF | `../typst/crates/typst-pdf/src/convert.rs` |
| Text painting and glyph output | `../typst/crates/typst-pdf/src/text.rs` |
| Paint conversion | `../typst/crates/typst-pdf/src/paint.rs` |
| Frame/page data shape | `../typst/crates/typst-library/src/layout/frame.rs`, `page.rs` |
| PDF tagging ideas | `../typst/crates/typst-pdf/src/tags/` |

The renderer may adopt Typst-style frame-to-PDF paint discipline, but DOCX
pagination, section semantics, table flow, footnotes, and anchored object
behavior must be derived from LibreOffice/Writer.

## 3. Current State

The current `crates/ooxmlsdk-pdf` DOCX path is:

```text
docx::extract
  -> DocxDocument { sections, page, blocks, headers, footers, notes }
  -> layout::layout
  -> LayoutDocument {
       pages, frames, follows, page_replays, backward_moves,
       layout_reruns, page_invalidations, reflow_executions,
       reflow_requests, restart_plan, outline_entries
     }
  -> render::krilla::render
```

This now supports more than smoke-level PDF generation:

- typed ordered section import from paragraph-level and body-level `sectPr`
- section break normalization for directly applicable Writer rules
- section-scoped page setup, columns, title page state, and repeating areas
- default/first/even header/footer import with link-to-previous inheritance
- settings-derived even/odd header selection and default tab stop
- paragraph/run/style import, numbering, tabs, fields, SDT content, hyperlinks,
  comments, notes, and dynamic PAGE/NUMPAGES refresh
- page backgrounds, page borders, header/footer body reservation, and
  negative top/bottom margin fallback direction
- page footnote boss reservation, note frames, and endnote/comment trailing
  fallback
- typed inline/floating DrawingML image anchors, VML image fallback,
  DrawingML/VML textbox frames with block content, crop/transform state, wrap
  distances, behind/in-front ordering, relative size, `layoutInCell` placement,
  and retained page fly exclusions
- typed first-pass `w:framePr` paragraph frames and `w:tblpPr` floating tables
  using a shared `FloatingFramePlacement` model
- table/row/cell frame ownership, grid/span/merge, row follows, repeated
  headers, cell spacing/margins, border conflict selection, table styles, and
  nested cell flow
- `TextFrameLayout`, `TextFrame`, and `LineFrame` line ownership with follow
  records, split cursors, keep/widow/orphan decisions, replay, invalidation,
  backward-move checkpoints, and restart planning
- explicit layout-to-paint conversion before Krilla output, shaped glyph PDF
  paint, link annotations, outline/bookmark export, image paint, clipping, and
  basic transforms

Known architectural gaps before the 100% close:

- `docx/mod.rs` still hosts helper implementations that now have module
  facades; these must continue draining into `properties`, `text`, `drawing`,
  and `package`.
- Footnotes reserve measured same-page areas, but true Writer continuation
  frames, continuation separators, and full table/column interactions are not
  complete.
- Fly frames have typed anchors, placement, z-order direction, relative size,
  retained square/tight exclusions, textbox block content, paragraph `framePr`,
  and table `tblpPr` basics, but full page reassignment, contour wrap, linked
  textbox chains, drawing effects, and table/header/footer fly interactions
  remain.
- Tables have Writer-shaped row/cell/follow ownership and first-pass floating
  table placement, but full rowspan split recalculation, split-fragment border
  conflict handling, floating-table split behavior, and repaint invalidation
  still need closure.
- Text layout has durable frame records and line boxes, but exact Writer line
  breaking, bidi, CJK/kana justification, glyph adjustability, and backward
  line reflow are still open.
- Package/property import needs settings/compat flags, theme defaults,
  revision metadata, more field metadata, and complete property overlay
  centralization.
- PDF export still needs font embedding/substitution policy, page labels,
  general bookmark/document target mapping, metadata/export options, and
  tagged/PDF-A behavior when those are in scope.

## 4. Target Internal Model

The internal model should be Writer-like, but implemented in Rust around
`ooxmlsdk` typed input.

### 4.1 Import Model

The import model preserves OOXML semantics before layout:

```text
ImportedDocument
  settings
  styles
  numbering
  resources
  sections: Vec<ImportedSection>
```

```text
ImportedSection
  sect_pr: w::SectionProperties-derived data
  break_type
  page_style
  header_footer_refs
  columns
  blocks
```

```text
ImportedBlock
  Paragraph
  Table
  FloatingFrame
  SdtBlock
```

```text
ImportedInline
  TextRun
  Break
  Tab
  Field
  Hyperlink
  NoteReference
  DrawingInline
  DrawingAnchor
  ShapeFrame
  TextBoxFrame
  VmlObject
```

The model does not need to store generated `w::*` values by ownership
everywhere, but each value should be derived from generated types and retain
enough OOXML identity to map behavior back to LibreOffice rules.

### 4.2 Layout Model

The layout model should represent Writer-like frames:

```text
LayoutDocument
  pages: Vec<PageFrame>
  frames: Vec<LayoutFrame>
  follows: Vec<FrameFollow>

PageFrame
  page_style
  body_area
  header_area
  footer_area
  footnote_area
  frames

Frame
  TextFrame
  TableFrame
  RowFrame
  CellFrame
  FlyFrame
  ImageFrame
  ShapeFrame
```

Text layout should produce line boxes before painting:

```text
TextFrame
  lines: Vec<LineBox>

LineBox
  inline_boxes
  ascent
  descent
  width
  break_state
```

This avoids adding more behavior directly to `layout_paragraph()`, which is
not a sufficient foundation for Writer-compatible pagination.

### 4.3 Paint Model

The paint model should be a stable display list independent of DOCX import:

```text
PaintDocument
  pages: Vec<PaintPage>

PaintItem
  TextRun
  Path
  FillRect
  StrokeLine
  Image
  LinkAnnotation
  TagMarker
```

`render::krilla` should consume paint pages rather than making layout
decisions.

## 5. Implementation Phases

### Fixture and Verification Strategy

The DOCX/PDF lane should optimize for real LibreOffice-aligned coverage rather
than hand-crafted renderer fixtures. New PDF calibration fixtures live only in
`../ooxmlsdk-test-suite/corpus/LibreOffice/` and should be copied from upstream compatibility
suites, starting with LibreOffice Writer QA files under `../core/sw/qa/`.

Default fixture policy:

- prefer real LibreOffice Writer fixtures from `sw/qa/extras/ooxmlimport/data/`,
  `sw/qa/extras/ooxmlexport/data/`, `sw/qa/extras/ww8import/data/`, and
  `sw/qa/filter/ww8/data/`
- keep `ooxmlsdk-pdf-test` focused on `.docx`, `.docm`, `.dotx`, and `.dotm`
  until a deliberate `.doc` conversion path exists
- do not add hand-crafted DOCX files just to exercise a renderer feature; add a
  LibreOffice regression fixture that already captures the behavior
- record each copied fixture's upstream source path next to the fixture

Each copied fixture should live under
`../ooxmlsdk-test-suite/corpus/LibreOffice/` and record its upstream source path,
for example a file copied from `../core/sw/qa/extras/ooxmlexport/data/`.

Implementation work should also happen in feature batches. A batch may import
more properties than layout consumes immediately, provided the model fields are
typed, source-backed by generated `ooxmlsdk` types, documented in this file,
and marked as pending layout consumption in the matrix/status text.

Current status must be read in two layers:

- **Architecture substrate: about 90% ready.** The main path now has typed DOCX
  package import, section/page-style flow, first/default/even header/footer
  inheritance and selection, basic column flow, paragraph/run/style import,
  shaped text measurement, PDF glyph paint, PAGE/NUMPAGES refresh, page
  backgrounds/borders, measured footnote reservation, raster images, typed
  floating anchors, textbox frame content, `framePr` paragraph frames, `tblpPr`
  floating tables, a Writer-shaped table/frame path, frame/follow records,
  invalidation records, page replay, executable backward-move checkpoints, and
  an explicit paint document before Krilla output. The import path is now
  split across concrete `model`, `text`, `drawing`, `package`, `properties`,
  and `table` modules instead of being centered entirely in `docx/mod.rs`.
- **LibreOffice behavior parity: the old 5-fixture strict number is no longer
  the useful status line.** The calibration surface has expanded into a broad
  LibreOffice-derived fixture lane. The upstream matrix currently records 306
  local covered PDF-rendering fixture rows, and the local fixture directory
  currently contains 304 Word documents. Treat this document as a capability
  map, not a pass/fail report: current green/red status must come from the
  latest `cargo test -p ooxmlsdk-pdf-test` run. Full end-user parity still
  depends on broader Writer frame persistence, line breaking, bidi/CJK
  justification, footnote/table/fly interaction, compatibility flags, drawing
  effects, fields/revisions, and export-option behavior.

The target for this workstream is still full LibreOffice Writer alignment, but
the percentages below must not be read as full-product compatibility. A row can
temporarily reach `100%` only for its explicitly checked tracked surface. Full
LibreOffice parity remains blocked until the full-parity blocker list below is
closed or explicitly declared out of scope.

### Today Tracked-Surface 100% Target

For today's work, `100%` means **100% of the checked DOCX/PDF tracked surface**,
not complete LibreOffice Writer DOCX-to-PDF compatibility. The tracked surface
is:

1. **Sections/page styles:** section boundaries, break normalization,
   first/default/even header/footer inheritance, body reservation, columns,
   page backgrounds, and page borders match the selected LibreOffice Writer QA
   fixtures in `../ooxmlsdk-test-suite/corpus/LibreOffice/`.
2. **Paragraph/text:** paragraph/run property overlay, numbering, tabs, fields,
   page/column breaks, keep/widow/orphan behavior, shaped text measurement, and
   paint portions match the selected LibreOffice Writer QA fixtures.
3. **Tables:** table width/alignment/indent, grid/span/merge, cell margins and
   spacing, row follows, repeated headers, nested cell flow, and border
   conflict behavior match the selected LibreOffice Writer QA fixtures.
4. **Drawing/fly frames:** inline/floating images, DrawingML/VML fallback,
   anchor reference areas, wrap distances, behind/in-front order, retained page
   exclusions, crop/transform state, relative sizing, `layoutInCell`,
   DrawingML/VML textbox frames, paragraph `framePr`, and table `tblpPr` match
   the selected LibreOffice Writer QA fixtures.
5. **Notes:** footnote/endnote references, same-page note reservation, note
   frame ordering, separator behavior, body-area shrinkage, and trailing
   fallback match the selected LibreOffice Writer QA fixtures.
6. **PDF paint:** layout-owned paint items, shaped glyphs, clipping,
   decorations, images, links, outlines, and basic metadata/output validity
   match the current LibreOffice-calibrated expectations.

Do not raise any row to `100%` unless the corresponding upstream-derived
fixtures have concrete assertions for page count, frame/follow ownership, key
text/image/note positions, replay/invalidation behavior when relevant, and PDF
output validity. If a LibreOffice behavior is intentionally outside today's
tracked surface, move it to "deferred parity" with a specific reason instead of
leaving it as an unbounded gap.

### Full LibreOffice Parity Blockers

The renderer is not fully LibreOffice-compatible until these blockers are
closed:

- **Footnotes/endnotes:** true Writer continuation frames, continuation
  separators, multi-page notes, notes inside tables/columns, and separator
  style/settings.
- **Fly frames/drawing:** page reassignment, contour wrap, all anchor classes,
  object formatter behavior, linked textbox chains, effects, complex z-order,
  header/footer/table fly interactions, SVG/PDF image handling, and OLE-like
  fallbacks where applicable. Basic floating anchors, textbox block frames,
  paragraph `framePr`, table `tblpPr`, relative size, and `layoutInCell` are
  already modelled but not complete parity.
- **Text layout:** exact Writer line breaking, bidi ordering, CJK/kana
  justification, glyph-level adjustment, compatibility-mode line spacing,
  hyphenation, fallback fonts, and backward reflow when later frames change.
- **Tables:** full grid conflict resolution, rowspan-aware split
  recalculation, split-fragment borders, compatibility border-distance quirks,
  floating-table split/repositioning, table/fly/footnote interactions, nested
  follow tables, and repaint invalidation.
- **Sections/page styles:** mirrored/gutter pages, page numbering and labels,
  page-master reassignment after backward moves, dynamic header/footer sizing,
  and negative-margin text-frame fallback.
- **Properties/import:** settings and compatibility flags, theme/default style
  resolution, revision metadata, comments/fields beyond the current dynamic
  subset, complete `PropertyMap` overlay order, and all tracked part metadata.
- **PDF export:** Krilla now writes metadata, page labels, named bookmark
  destinations, attachments, native SVG, raster ICC/EXIF data, and tagged
  paragraph/heading/table/figure/link structure. PDF/UA output is accepted only
  after Krilla validation; form widgets are rejected in that mode until a
  tagged form API replaces the lopdf widget post-processor. PDF/A output
  intents/color policy, richer list and repeating-area semantics, transparency
  edge cases, and deterministic output comparison remain open.
- **Calibration harness:** systematic LibreOffice reference rendering,
  extraction of comparable page/text/image/annotation geometry, and golden diff
  reporting for upstream-derived samples.

### LibreOffice Calibration Crate

`../ooxmlsdk-test-suite/crates/ooxmlsdk-pdf-test` is the separate crate for full parity calibration.
It stays separate from `ooxmlsdk-pdf` because the calibration lane needs
LibreOffice installed, temporary reference PDFs, PDF text/geometry extraction,
optional raster comparison, and golden artifacts that should not slow the
runtime crate's ordinary unit tests.

Scope boundary: this crate does not re-test DOCX/package/schema round-trip
readiness. `../ooxmlsdk-test-suite/crates/ooxmlsdk-test` owns that layer.
`ooxmlsdk-pdf-test` assumes the input DOCX is test-ready and only gates the
LibreOffice-vs-`ooxmlsdk-pdf` PDF export rendering surface.

LibreOffice QA alignment:

- `../core/sw/qa/inc/swmodeltestbase.hxx` and
  `../core/sw/qa/unit/swmodeltestbase.cxx` expose the Writer model/layout
  harness: tests can force layout calculation, read UNO-visible body text, and
  parse the Writer layout dump. This confirms that exact layout coordinates are
  first-class LibreOffice test data, not a loose visual smoke signal.
- `../core/vcl/qa/cppunit/pdfexport/pdfexport2.cxx` parses exported PDFs with
  PDFium and asserts page counts, link presence, page-object counts, image
  dimensions, path segment counts, and path fill colors. `ooxmlsdk-pdf-test`
  therefore uses `pdfium-render` as the primary PDF observation layer.
- `../core/sw/qa/extras/layout/layout2.cxx` covers PDF/export-triggered Writer
  layout bugs by saving through the PDF path and then asserting exact table,
  row, cell, and text-frame bounds from the layout dump. The Rust calibration
  lane must keep reporting geometry deltas precisely enough to drive the
  renderer toward those Writer frame invariants.
- `../core/sw/qa/extras/ooxmlexport/*.cxx` owns DOCX semantic import/export
  assertions for the selected fixtures, such as footnote text, column
  separator state, and picture hyperlink targets. `ooxmlsdk-pdf-test` consumes
  those upstream DOCX fixtures for PDF rendering parity without duplicating the
  DOCX round-trip layer already covered by `ooxmlsdk-test`.

Implemented crate responsibilities:

- discover Word fixtures from `../ooxmlsdk-test-suite/corpus/LibreOffice/` (`.docx`,
  `.docm`, `.dotx`, `.dotm`)
- render each fixture with LibreOffice headless, for example
  `soffice --headless --convert-to pdf --outdir <tmp> <fixture.docx>`
- render the same fixture through `ooxmlsdk-pdf`
- extract comparable content/layout metadata from both PDFs through
  `pdfium-render`: page count, page geometry, text segments, character
  bounding boxes, text object font/size/color/bounds details, page object
  summaries, path segment coordinates/close flags/fill/stroke/bounds details,
  image object geometry, link targets and rectangles, and deterministic raster
  checksums; Poppler `pdftotext` and decoded content-stream operation summaries
  remain diagnostic views
- emit a concise inventory report that separates render failures from
  comparison issues and lists every mismatch found in a run

Strict comparison contract:

- Allowed PDF container differences: object numbers, xref layout, stream
  compression, producer metadata, subset font names, and harmless numeric
  serialization precision.
- Required content/layout parity: PDFium-observed page geometry, text
  content/geometry, text object font-size/color/bounds, image object geometry,
  path object geometry/paint/segment details, link targets/rectangles, page
  object summaries, and raster checksums must match LibreOffice for every
  fixture.
  Decoded painting-operation summaries are diagnostic because equivalent PDF
  generators may encode the same display list with different operators.
- No known-issue whitelist is allowed in this lane. A failing fixture is the
  work queue, similar to `ooxmlsdk-test` round-trip failures.

Open crate responsibilities:

- expand PDFium extraction for image bitmap format/checksum/SMask details,
  form/XObject nesting, outline trees, page labels, metadata, export-option
  surfaces, and richer annotation action details
- add only PDF-compatibility normalizers, not renderer-behavior tolerances
- emit a concise failure report that names the fixture, page, object kind,
  LibreOffice value, Rust value, delta, and owning parity area

Final LibreOffice QA audit before using this as the development baseline:

- Covered by the calibration lane: PDFium page counts and geometry, page
  object counts/types, text extraction and text geometry, text object
  font/size/color details, path object segment coordinates/close
  flags/fill/stroke/bounds, image object dimensions/bounds, link
  target/rectangle details, and raster render checksums.
- Covered by existing `ooxmlsdk-pdf` unit tests rather than the PDF parity
  lane: pure DOCX import, property resolution, section/header/footer/note/table
  model extraction, and layout-model invariants.
- Still future calibration work from `../core/vcl/qa/cppunit/pdfexport*` and
  Writer layout QA: image bitmap encoding/mask details, nested form/XObject
  object trees, PDF export options, tagged/PDF-A/encryption features when in
  scope, page labels, metadata, and exact Writer layout-dump frame trees beyond
  the geometry currently observable through PDFium.

Implemented layout:

```text
../ooxmlsdk-test-suite/crates/ooxmlsdk-pdf-test/
  Cargo.toml
  src/
    lib.rs
    pdf_extract.rs
    render.rs
  tests/
    core_docx_pdf_fixtures.rs
    mapped_docx_pdf_fixtures.rs
    pdfexport_fixtures.rs
../ooxmlsdk-test-suite/corpus/LibreOffice/
  README.md
  <LibreOffice-derived Word fixtures>
```

This lane currently has these layers:

- direct PDF/object tests projected from LibreOffice PDF export tests
- source-backed Writer layout tests projected into PDF-visible assertions
- mapped visible-output fixture tests over the broad
  `../ooxmlsdk-test-suite/corpus/LibreOffice/` fixture set
- render-summary and PDF extraction helpers that separate open-file/render
  failures from comparison failures

Current repository calibration surface:

- verification command: `cargo test -p ooxmlsdk-pdf-test -- --nocapture`
- fixture directory: `../ooxmlsdk-test-suite/corpus/LibreOffice/`
- upstream source map:
  `../ooxmlsdk-test-suite/docs/ooxmlsdk-pdf-test/LibreOffice.md`
- matrix status: 306 local covered PDF-rendering fixture rows, 10 direct
  upstream DOCX-to-PDF/object assertion rows, and 8 supplemental source-backed
  PDF-visible assertion rows
- filesystem status: the fixture directory currently contains 304 Word
  documents (`.docx`, `.docm`, `.dotx`, `.dotm`)
- pass/fail status: intentionally not recorded here; use the latest
  `ooxmlsdk-pdf-test` run output as the current result
- legacy cleanup: DOCX-specific `%PDF` smoke assertions and PDF byte-string
  checks in `crates/ooxmlsdk-pdf` were removed because they were weaker than,
  and potentially misleading beside, the LibreOffice/PDFium calibration lane.
  Internal import/layout unit tests remain in `ooxmlsdk-pdf`; XLSX/PPTX minimal
  smoke tests remain out of scope for this DOCX parity lane.

Latest local verification snapshot:

- `cargo test -p ooxmlsdk-pdf`: 65 passed / 0 failed.
- `cargo test -p ooxmlsdk-pdf-test --test pdfexport_fixtures -- --nocapture`:
  10 passed / 0 failed.
- `cargo test -p ooxmlsdk-pdf-test --test mapped_docx_pdf_fixtures -- --nocapture`:
  96 passed / 193 failed / 289 total.
- `cargo clippy --workspace --all-targets -- -D warnings`: failed in
  `ooxmlsdk-pdf` on lint cleanup items: `Block` large enum variant,
  `drawingml_shape_geometry()` collapsible match, outline-entry collapsible
  `if`, and Krilla text-portion collapsible `if`.

Next implementation order:

1. Keep the broad mapped fixture lane source-backed and avoid broadening
   compare equivalence unless a LibreOffice/PDF extraction fact proves it
   necessary.
2. Let the largest fixture failure clusters drive renderer work, especially
   text layout, floating frames, tables, notes, and drawing effects.
3. Push recently modelled frame/fly semantics deeper into layout and paint:
   DrawingML/VML textboxes, paragraph `framePr`, table `tblpPr`, relative
   anchors, `layoutInCell`, wrap, and shape effects.
4. Keep the validation loop as `cargo fmt --all`,
   `cargo test -p ooxmlsdk-pdf`, `cargo test -p ooxmlsdk-pdf-test -- --nocapture`,
   and `cargo clippy --workspace --all-targets -- -D warnings`.

The main path now covers typed package import, section/page-style flow,
first/default/even header/footer inheritance and selection, negative top/bottom
margin header/footer fallback, basic column flow, paragraph/run/style import,
shaped text measurement and PDF glyph paint, PAGE/NUMPAGES field refresh, page
backgrounds/borders, measured footnote reservation, raster images, typed
floating anchors with paragraph-local and page-level square/tight wrap
influence, DrawingML/VML textbox frames with block content, `w:framePr`
paragraph frames, `w:tblpPr` floating table placement, and a Writer-shaped
table/frame path with row follows, repeated header rows, cell margins/spacing,
vertical merge, rowspan-aware split guarding, repeated-header fit checks that
account for cell spacing, and table-style cascades. Cluster DOCX layout
snapshots now cover the section,
paragraph, table, drawing, and notes lanes so broad regressions are easier to
catch before pixel/PDF-byte comparison exists. `LayoutDocument` now carries
concrete frame records and block-level follow metadata for paragraph, table,
and note frame transitions: each frame has a page/column owner, display-list
range, retained display-list items, bounds, synthesized line boxes, split
cursors, paragraph/table/note fragments, table fragment split ownership,
frame-owned insertion influences for footnote reservation, fly wrapping, and
table splitting, invalidation state, page-level invalidations, scoped
reflow-request records, move-backward suppression keys, retained frame replay,
per-page replacement records for replayed body ranges, and page replay
application records that splice replayed frame items back into the live page
display list before page decorations are applied. Influence replay now also
has an executable backward-move record and a block-entry checkpoint rerun path:
when a non-suppressed backward move is selected, layout restores the saved
pages/current frame/follows/frames/footnote state at the owning block and
derives rerun constraints from the triggering insertion influence, shrinking
the restored flow's bottom edge for footnote/table reservations or narrowing
the column for fly wrapping before it reformats the remaining body plus
trailing note frames. Decoration-driven invalidations are then handled as a
separate retained-frame stabilization pass. PDF export now builds an explicit
paint document from layout pages and frame/line ownership before touching the
Krilla backend, so rendering consumes paint items instead of interpreting the
DOCX layout display list directly. Text shaping, width measurement, baseline
placement, highlight rectangles, underline/strike strokes, and hyperlink
annotation rectangles are now performed while building that paint document, so
the PDF backend receives frame/line-owned paint portions rather than deriving
text geometry during paint. Text paint portions are explicitly classified as
text, tab, dynamic field, or hyperlink portions and each portion carries its
own text range, x-position, glyph subset, decoration geometry, and link
geometry. Each text portion now also carries the owning line paint/clip
rectangle and the PDF backend clips portion painting through that line rect,
matching Writer's `SwTextPainter::DrawTextLine` clipping discipline. Tab
portions now preserve Writer's tab-over-margin paint rule by expanding the
portion clip to the page paint right edge when the tab run exceeds the line
clip, instead of truncating it to the normal text line width. Paragraph
`w:outlineLvl` is now imported through paragraph/style/numbering format
resolution, retained as layout outline entries with page coordinates, and
exported as a hierarchical PDF outline/bookmark tree through Krilla
destinations, matching Writer's PDF navigation export shape for outline-level
paragraphs.

The import architecture is no longer only a plan. `docx/text.rs` owns paragraph
model assembly and the paragraph/run import boundary; `docx/drawing.rs` owns the
DrawingML/VML entry points consumed by text import; `docx/package.rs` owns image
and external hyperlink relationship catalogs for main/header/footer/notes/
comments parts; and `docx/properties.rs` now owns paragraph format resolution
plus direct run-style resolution. Some helper code still remains in `docx/mod.rs`
while it is drained toward those modules, but the call graph now follows the
LibreOffice-style import boundaries rather than adding new behavior directly to
the root module.

Remaining requests are reduced to a first invalid page/frame restart plan with
frame/column/page scope, matching the shape of Writer's first-invalid-page loop
and Typst's relayout checkpoint. This moves the renderer past a flat
display-list-only model and gives the next Writer-style rollback and backward
invalidation work something concrete to operate on.

Progress matrix, using LibreOffice Writer as the behavior target and Typst as
the frame/paint technique reference. These numbers are implementation-readiness
estimates, not current test pass rates:

| Area | Current parity | 100% gate |
|------|----------------|-----------|
| DOCX package/import backbone | 68% | Settings/compat flags, theme/default style data, revisions, field metadata, notes/comments/hyperlinks/images, and tracked part traversal all imported through generated `ooxmlsdk` types or documented typed fallbacks. |
| Sections/page styles/repeating areas | 55% | LibreOffice `SectionPropertyMap` behavior for break normalization, inheritance, first/even/default slots, negative margins, mirrored/gutter pages, page numbering, columns, and page-master reassignment is covered by upstream-derived fixtures. |
| Paragraph/run properties | 48% | A dedicated property resolver matches Writer overlay order for doc defaults, styles, numbering, table style, direct `pPr/rPr`, `framePr`, tabs, numbering, bidi/CJK options, fields, compatibility flags, and theme/default font resolution. |
| Text layout | 48% | Writer-like `SwTextFrame` master/follow state, font/theme selection, character spacing, run coalescing, portion clipping, and dynamic field paint are in place; exact line breaking, bidi, CJK/kana justification, and broader repaint behavior remain open. |
| Tables | 55% | `SwTabFrame`/row/cell follow behavior covers repeated headers, nested cell flow, border conflict selection, row height, cell spacing, table styles, first-pass floating `tblpPr` placement, and split guards; full rowspan split recalculation, floating-table split behavior, and broader repaint invalidation remain open. |
| Footnotes/endnotes | 38% | Same-page note reservation, inline note markers, note frame ordering, and trailing fallback exist; true continuation frames, continuation separators, and broader table/column note interaction remain open. |
| Drawing/floating objects | 48% | Fly-frame import/layout covers inline/floating anchors, paragraph/page reference areas, relative size, `layoutInCell`, wrap distances, z-order direction, textbox block frames, paragraph `framePr`, and basic shape/image paint; page reassignment, contour wrap, linked textboxes, effects, SVG/PDF image details, and table/header/footer interactions remain open. |
| PDF paint/export quality | 55% | Paint output covers shaped glyph portions, line clipping, decorations, native raster/SVG images with ICC/EXIF handling, links, outlines, metadata, page labels, named destinations, attachments, transforms, and tagged PDF/PDF-UA structure. Font embedding/substitution policy, tagged form widgets, PDF/A output intents/color policy, richer structure semantics, transparency, and LibreOffice-like export edge cases remain open. |

100% close order:

1. Keep the mapped LibreOffice fixture lane broad and source-backed while using
   current failures, not stale pass counts, to pick implementation batches.
2. Reduce fixture-aware compare equivalence by fixing residual renderer
   semantics where they are clearly attributable.
3. Continue draining helper code from `docx/mod.rs` into `docx/properties.rs`,
   `docx/text.rs`, `docx/drawing.rs`, and `docx/package.rs` whenever a behavior
   patch touches those call boundaries, but do not treat module motion by
   itself as parity progress.
4. A row reaches `100%` only when its 100% gate is covered by fixtures and no
   tracked behavior is left as a known gap.

### Phase 0: Freeze Non-DOCX Scope

- Keep `xlsx.rs` and `pptx.rs` compiling.
- Do not expand XLSX/PPTX PDF behavior during DOCX renderer work.
- Keep existing public entry points stable unless a change is required for
  feature gating or API safety.

### Phase 1: Section and Page Style Backbone

LibreOffice reference:

- `SectionPropertyMap::CloseSectionGroup`
- `SectionPropertyMap::InheritOrFinalizePageStyles`
- `SectionPropertyMap::HandleMarginsHeaderFooter`
- `SectionPropertyMap::PrepareHeaderFooterProperties`
- `SectionPropertyMap::CopyLastHeaderFooter`

Implement:

- Collect paragraph-level `sectPr` and final body-level `sectPr`.
- Build an ordered section list.
- Model section break type:
  - continuous
  - nextPage
  - nextColumn
  - oddPage
  - evenPage
- Apply LibreOffice-compatible break normalization where practical:
  - missing break type defaults to next page
  - nextColumn may act as nextPage depending on columns/compatibility
  - continuous section with changed orientation becomes nextPage
- Derive page setup per section.
- Add header/footer reference state per section.
- Implement link-to-previous behavior by carrying/copying prior section
  header/footer content.
- Add first/default/even header/footer slots. Even/odd document setting must be
  honored once settings support exists.

Minimal tests:

- Two sections with different page sizes.
- Continuous section with same page setup.
- Continuous section with changed orientation normalized to page break.
- Header/footer inherited from previous section.
- First-page header/footer selected when `titlePg` is present.

Current progress:

- Added a typed section collector in `crates/ooxmlsdk-layout/src/docx/mod.rs`.
- The collector reads `w::Body.body_choice`, paragraph
  `w::ParagraphProperties.section_properties`, and final
  `w::Body.w_sect_pr` through generated `ooxmlsdk` types.
- `DocxDocument.sections` now carries ordered `ImportedSection` entries with
  section break kind, section properties, page setup, `titlePg`, and blocks.
- The existing flat `DocxDocument.blocks` field is still populated for
  compatibility, but `layout::layout` now consumes `DocxDocument.sections`
  when present and uses each section's page setup for body flow.
- `layout::layout` now enters through a Writer-like root frame driver. The
  driver owns page frame creation, section body frames, column flow, page
  decoration order, and footnote boss reservation before converting the current
  frame content into `PageItem`s.
- Section page breaks currently start new pages for `nextPage`, `nextColumn`,
  `evenPage`, and `oddPage`; `continuous` keeps flowing on the current page.
- `evenPage` and `oddPage` section breaks insert a page frame when needed so
  the following section starts on the requested page parity.
- Header/footer import now resolves default/first/even slots per section.
- Missing section header/footer references inherit the previous section's slot,
  matching LibreOffice's `CopyLastHeaderFooter` direction at the content level.
- `w:evenAndOddHeaders` is read from settings before selecting even-page
  header/footer slots.
- Layout pages now track their source section index and page index within that
  section so repeating content can be selected from the section page style
  state instead of by scanning neighboring pages. Hard page breaks,
  `pageBreakBefore`, and natural overflow all advance the same section page
  counter before first/default/even header/footer slots are resolved.
- Header/footer repeating areas now follow the Writer margin-distance shape from
  `PrepareHeaderFooterProperties`: header starts at `w:pgMar/@header` and ends
  at the top body margin, footer starts at the bottom body margin and ends at
  `w:pgMar/@footer`, with a 1mm minimum area height.
- Body text flow now reserves those repeating areas when header/footer
  distances overlap body margins, so the page body starts below an intrusive
  header and ends above an intrusive footer instead of relying on paint order.
  The reservation is page-style aware: first/default/even header and footer
  slots are resolved per body leaf, and the same slot-selection state is reused
  when section/column flow advances to a new page.
- Negative top/bottom page margins are now preserved from generated
  `w:pgMar` import as page-style state. When a header or footer is present,
  body flow no longer reserves header/footer space for those negative-margin
  sides, matching the direction of LibreOffice
  `SectionPropertyMap::HandleMarginsHeaderFooter()` where Writer avoids body
  overlap by converting header/footer content into anchored text-frame fallback
  instead of expanding the ordinary body print area.
- Section `w:cols` is now imported through generated `w::Columns` into
  `SectionColumns`, including count, gap, separator flag, and explicit
  non-equal `w:col` width/space definitions when `equalWidth` is off.
- Layout has section body-frame equal-column flow at block boundaries, plus
  separator painting when `w:cols/@sep` is enabled. Paragraph-internal and
  table-internal column splitting remain frame-model work.
- Run-level `w:br w:type="column"` is imported as a distinct typed
  `InlineItem::ColumnBreak`. In multi-column sections it advances the active
  text frame to the next column and continues following inline content there;
  in single-column or final-column contexts it follows the LibreOffice/Typst
  fallback shape and behaves like a page break.
- Footnotes/endnotes are now imported as typed id-to-block maps, and paragraphs
  retain generated-type-derived footnote/endnote reference ids. Document and
  section numbering format/start/restart settings are resolved in section
  order; `eachPage` labels are reassigned from the final page ownership of the
  reference and backlink runs instead of trusting a pre-pagination counter.
- Footnote content now uses a page footnote boss area. Referenced footnotes are
  measured through the shared paragraph/table layout path before laying out the
  referencing paragraph, the body frame bottom is reduced for that page, and the
  note separator/content is emitted near the bottom margin. This follows
  Writer's `ftnfrm` direction more closely than single-line estimation, but
  still needs true continuation separators and full table/column interaction.
- Paragraph keep properties are now imported through generated paragraph
  property types: `w:keepNext` maps to `keep_with_next`, `w:keepLines` maps to
  `keep_lines`, and `w:pageBreakBefore` remains the page-break-before signal.
  Layout has a body-frame preflight that advances to the next column/page when
  a keep group does not fit in the current flow region.
- Table row properties are imported through generated `w::TableRowProperties`:
  `w:tblHeader` marks consecutive leading repeated header rows and
  `w:cantSplit` is retained on the row model. Layout repeats valid table header
  rows after page breaks, following LibreOffice's constraints that all-row
  headers and more than 10 header rows do not repeat. Repeated header fit
  checks include the following cell-spacing gutter so a headline is not
  repainted when the header plus the next row cannot actually fit in the follow
  region.
- Table and cell margins are imported through generated `w::TableCellMarginDefault`
  and `w::TableCellMargin` types. Layout uses those margins for row height,
  content inset, and vertical alignment instead of a fixed padding constant,
  matching LibreOffice's cell border-distance import path.
- Table preferred width now preserves absolute `dxa` widths and `pct` widths,
  while `w:jc` and `w:tblInd` are imported through generated table property
  types. Layout resolves scaled grid columns plus left/center/right table
  placement through `TableFrameLayout`, matching the Writer table positioning
  direction before full row split/follow behavior lands.
- Table cells now retain generated `w::TableCellWidth` absolute and percent
  widths. When `tblGrid` is absent or incomplete, `TableFrameLayout` can seed
  its column widths from preferred cell widths before applying the table
  preferred width. This now handles `gridSpan` by distributing a spanned cell's
  preferred width across the covered grid columns, following the Writer width
  negotiation direction instead of falling back directly to equal columns.
- Table layout ownership now follows `SwTabFrame`/`SwRowFrame`/`SwCellFrame`:
  `TableFrameLayout` owns follow moves through the same section leaf traversal
  as body text, repeated headline rows, and the destination `FlowContext` for
  following body blocks. `RowFrame` owns row bounds and horizontal borders, and
  `CellFrame` owns cell background, leading border, vertical-merge continuation
  paint, and nested content formatting. Vertical merge continuation cells now
  retain the origin cell shading in the display list and row horizontal borders
  are segmented so the border does not cut through the merged column.
- Internal table borders now resolve competing adjacent cell borders before
  painting the shared edge. The current rule prefers the stronger stroke width
  and paints internal horizontal edges once from the preceding row, following
  Writer's collapsed table-edge direction and Typst's resolved-grid stroke
  priority model instead of blindly overpainting both neighboring edges.
- When `tblCellMar` and the table style hierarchy omit a margin, cell margins
  use the ECMA-376 Part 1 §17.4 defaults: start/end are `115` twips and
  top/bottom are zero. When recovering a package without a Styles part, Word
  positions a top-level left-aligned table so the first cell's content begins
  at the text margin; styled and nested tables retain their border-relative
  placement. Table-level `tblCellMar` and cell-level `tcMar` override these
  defaults.
- Table cell spacing now imports `w:tblCellSpacing` through the generated
  `TableCellSpacing` type and treats it as table gutter. This follows the OOXML
  table-property entry LibreOffice receives and the same separation model Typst
  uses for grid/table gutter: nonzero spacing separates cell frames and paints
  per-cell edges instead of collapsed shared borders.
- Table rows now import `w:gridBefore` and `w:gridAfter` through generated row
  property types and carry those skipped grid columns into layout. Cell
  placement, preferred-width negotiation, row height estimation, vertical-merge
  lookup, and adjacent-border lookup now operate on the row's real grid
  coordinates, matching LibreOffice's `TableManager` row grid bookkeeping and
  Typst's grid-coordinate resolution direction.
- Table style import now has a first-pass style bus for generated OOXML table
  style types. `tblStyle` resolves through the style chain, style-level
  `tblPr/tcPr` can provide table borders, default cell margins, cell spacing,
  cell shading, cell margins, and vertical alignment. Style-level and
  conditional `pPr/rPr` are also merged into the paragraph and run styles used
  by table-cell content, so header-row alignment, bold, and text color come
  from the same table-style cascade as borders and shading. Those table-style
  `pPr/rPr` values are applied as the base for cell paragraph/run import before
  paragraph style, character style, and direct `pPr/rPr`, matching Writer's
  property stack direction and Typst's inner-value-wins fold model. `tblStylePr`
  currently applies `wholeTable`, first/last row, first/last column, horizontal
  and vertical band, and corner-cell formatting. Conditional `tblPr` now feeds
  table alignment, indentation, and cell spacing into the same table-level style
  cascade; conditional `trPr` is imported into a row-style model for row header
  repetition, cant-split behavior, and row-level cell spacing, with direct
  table/row properties applied after the table style. Explicit row/cell
  `cnfStyle` masks are parsed from generated OOXML types and combined like
  LibreOffice's row mask plus cell mask before corner conditions are applied.
  Collapsed adjacent border selection now carries enough style information to
  apply LibreOffice's width-first, simple-over-compound tie breaker instead of
  comparing only numeric widths. Direct `tcBorders` now overlays table-style
  cell borders per side, so an unspecified direct side keeps the style side
  while an explicit `none` clears only that side, matching the property-map
  fold direction in Writer and Typst's per-side stroke fold.
  This follows
  LibreOffice's `StyleSheetTable` / `TblStylePrHandler` split while keeping the
  final resolved row/cell properties in the Typst-like grid style shape.
- Table cell content now uses nested cell flow instead of the old one-line
  clipping path. Paragraphs and nested tables inside cells reuse the same
  paragraph/table layout functions used by body flow, so wrapping, run styles,
  tabs, inline images, borders, and paragraph spacing are no longer separately
  hand-implemented for cells. The nested cell flow now formats against an
  unbounded content bottom and clips emitted items to the current cell fragment,
  matching Writer's cell-frame direction and avoiding body-style page follow
  reflow inside ordinary table cells.
- Paragraph layout ownership now enters through `TextFrameLayout`, with a
  `TextFrame` carrying paragraph geometry and `LineFrame` carrying current line
  bounds, height, baseline y, and cursor x. `TextFrameLayout` now owns line
  advance, page break reset, column break reset, and wrap-bound recalculation.
  It also records `LineFragment` split candidates with inline cursors, matching
  Writer's offset-based text follow direction. Column breaks now update the
  active `FlowContext`, so later inline content and following body blocks keep
  flowing in the destination column/page. Page-line overflow records
  `TextFrameFollow` entries at the split cursor so page follows can be governed
  by the text frame state instead of anonymous y advancement. The text frame
  split policy now rejects `keepLines` splits and applies the LibreOffice
  orphan/widow minimum-line shape to recorded split candidates. Rejected page
  splits now roll back the current text frame output, advance to the next
  column/page leaf, and format the paragraph follow there once before allowing
  a forced split. This follows the Writer `MoveFwd`/master-follow direction
  while staying in the frame layout path. The current formatter still emits
  `PageItem`s directly, preserving behavior while preparing the full Writer
  `SwTextFrame` follow path for column/page splits.
- Natural line overflow inside `TextFrameLayout` now advances through the same
  section leaf traversal as explicit column/page movement and returns the
  destination `FlowContext` to the body frame. Following blocks therefore keep
  formatting in the column/page that owns the paragraph follow, matching the
  Writer master/follow invariant rather than restarting from the old body leaf.
- PDF text paint now consumes the same rustybuzz-shaped glyph advances used by
  layout measurement and emits krilla glyph runs rather than relying on
  `draw_text` auto shaping. This follows Typst's `typst-pdf/src/text.rs`
  discipline, where shaped glyphs are the stable paint input, and reduces
  layout/paint drift for ligatures, complex scripts, and RTL runs.
- DrawingML anchors are imported through generated `wp::Anchor` types into a
  typed floating image placement model. The layout path consumes `positionH`,
  `positionV`, `wp:align`, wrap mode, `behindDoc`, and anchor text distances
  for initial page/margin/column/paragraph-relative placement. Aligned anchors
  resolve against the selected reference area using the image extents, matching
  the Writer fly-frame positioning path and Typst placed-frame alignment
  technique. Square/tight wrapping now creates paragraph-local exclusion bounds
  that shorten affected text lines, matching the Writer `SwTextFly`/fly portion
  direction. Square/tight exclusions are now retained on the current page, so
  following paragraphs on that page continue to avoid the fly frame instead of
  forgetting it at the paragraph boundary. This is still an incremental
  approximation of Writer fly influence: true page reassignment, contour wrap,
  table/header/footer fly interactions, and full z-ordering remain fly-frame
  work. `wp:effectExtent` is now retained from typed inline/anchor values and
  expands the image frame used for alignment, line height, and wrap bounds,
  following LibreOffice's `GraphicImport::lcl_expandRectangleByEffectExtent()`
  direction. Wrap-level `dist*` values from `wrapSquare`, `wrapTight`,
  `wrapThrough`, and `wrapTopAndBottom` are merged with anchor distances, and
  `wrapText` left/right/largest/both is preserved in the paragraph/page
  exclusion model. Floating images are tagged as floating page items, while
  inline images keep their line position. Floating images that import
  `behindDoc` are ordered after page backgrounds but before body text;
  foreground floating images are ordered after body text, matching the Writer
  page-object layer direction while preserving Typst-style stable display-list
  output. Top/bottom floating images that exhaust the current column/page now
  advance following text through the same section leaf traversal as text-frame
  follows.
- Raster image painting now follows the same broad division as Writer and
  Typst: the DOCX importer keeps image bytes/resources on image frames, layout
  places inline/floating image items, and the PDF renderer performs only paint
  conversion. JPEG is passed through when krilla can consume it directly; PNG
  is decoded into an RGB/alpha sampled image using a tolerant decoder that can
  ignore bad CRC/checksum metadata in OOXML fixtures, matching LibreOffice's
  preference to paint recoverable graphic content instead of dropping the
  frame. Other raster formats fall back to the `image` crate and are emitted
  through a Typst-style `CustomImage` path. Inline and floating DOCX image
  fixtures now assert that generated PDF bytes contain image XObjects.
- DrawingML picture crop and basic transform state now stays on the image
  frame: `a:srcRect` becomes a source crop rectangle and `a:xfrm` rotation plus
  horizontal/vertical flips are carried through layout into PDF painting. The
  renderer follows the Typst group/image pattern by applying a local transform,
  clipping to the image frame, then painting an expanded source image for crop.
  This matches LibreOffice's direction of preserving DrawingML graphic geometry
  on the frame instead of dropping the picture or guessing a new layout size.
- VML `v:imagedata` crop attributes (`cropleft`, `croptop`, `cropright`,
  `cropbottom`) now feed the same image frame crop path. This follows
  LibreOffice's VML import path in `vmlshapecontext.cxx`, where the crop strings
  are copied onto the graphic type model before Writer's graphic frame paint
  resolves the visible rectangle.
- VML shape/image style `rotation` and `flip` now feed the same image transform
  path used by DrawingML. The import mirrors LibreOffice's `setStyle()` and
  `decodeRotation()` behavior, including fixed-degree `fd` rotation values and
  the VML convention that the decoded angle is negated before paint.
- VML absolute-positioned image styles now map into the floating image frame
  path instead of being forced into inline flow. `position:absolute`,
  `left`/`top` or `margin-left`/`margin-top`, `mso-position-*-relative`,
  `mso-wrap-style`, `mso-wrap-distance-*`, and negative `z-index` are carried
  into the existing placement/wrap/behind-text fields. This follows the
  LibreOffice VML style model and `GraphicZOrderHelper` direction while still
  using the same Typst-style stable image display-list paint path.
- Break normalization currently follows the directly applicable
  `SectionPropertyMap::CloseSectionGroup` rules:
  - missing `w:type` is treated as `nextPage`
  - `continuous` with changed page orientation is treated as `nextPage`
  - `nextColumn` without a valid matching multi-column context is treated as
    `nextPage`; the matching-column check also uses explicit non-equal `w:col`
    definitions when present
- Tests cover paragraph-level/body-level section collection, continuous
  same-orientation behavior, orientation-change normalization, and next-column
  normalization.
- The former hand-authored PDF-layout WML fixtures for section inheritance,
  column flow, keep-next pagination, repeated table headers, page background,
  and borders were removed. Future PDF parity coverage for these behaviors
  should come from LibreOffice QA fixtures copied into
  `../ooxmlsdk-test-suite/corpus/LibreOffice/`.
- Layout model coverage checks that a table row following into the next section
  column also moves later body blocks into that destination column, matching the
  Writer `SwTabFrame` follow invariant before full row split/follow-flow-line
  behavior is implemented.
- Layout model coverage checks that a `cantSplit` row moves forward to the
  next section column when it can fit there, and table formatting records rows
  that have already moved so an oversized row is placed once in its destination
  leaf instead of oscillating through empty pages.
- Real DOCX fixture coverage includes floating DrawingML image anchor import
  and placement via `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/image_floating.docx`.
- Real DOCX fixture coverage includes external `w:hyperlink` relationship
  import through the OOXML SDK part relationship model and krilla PDF link
  annotation emission via `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/fields_hyperlink.docx`.
- Real DOCX fixture coverage includes complex `w:fldChar`/`w:instrText` PAGE
  and NUMPAGES fields via `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/fields_complex.docx`. The same dynamic
  field path also covers simple PAGE/NUMPAGES fields and legacy `w:pgNum`. The
  import keeps a dynamic field marker and resolves visible text after body flow
  and repeating header/footer layout, following Writer's field-expansion
  direction while avoiding stale cached results.
- Layout support for document-level `w:background` color and section
  `w:pgBorders` remains in the renderer, but PDF parity tests for those
  behaviors should be reintroduced from upstream LibreOffice fixtures rather
  than locally generated WML samples.
- Verification for DOCX/PDF iteration currently includes
  `cargo test -p ooxmlsdk-pdf` and
  `cargo clippy -p ooxmlsdk-pdf --all-targets -- -D warnings`. Broader
  workspace clippy is reserved for shared/runtime/generator changes.

Remaining Phase 1 work:

- Handle negative header/footer margins using LibreOffice's text-frame fallback
  direction from `HandleMarginsHeaderFooter`.
- Continue tightening page-style body geometry for inherited/linked header and
  footer edge cases, including negative margins and page-master reassignment
  after backward movement.
- Continue replacing the remaining direct block formatters with Writer-like
  text/fly frames so paragraph-internal column splitting, inline continuation
  after column breaks, bottom footnote areas, and floating frames are enforced
  by frame ownership.

### Phase 2: Paragraph and Run Property Mapping

LibreOffice reference:

- `writerfilter/dmapper/DomainMapper.cxx`
- `writerfilter/dmapper/PropertyMap.cxx`
- `writerfilter/dmapper/StyleSheetTable.cxx`
- `sw/source/core/text/`

Implement:

- Move extraction toward a typed property resolver rather than ad hoc merging.
- Preserve direct, paragraph style, character style, doc default, and numbering
  precedence.
- Table-cell paragraph/run import now takes resolved table-style `pPr/rPr` as a
  base, so direct paragraph and run properties still override table formatting
  instead of being overwritten by a post-import table-style pass.
- Table-style `trPr` now participates in row import for conditional
  `tblHeader` and `cantSplit`, and direct row properties override those values.
  Row-level `tblCellSpacing` also flows into layout as a row gutter override,
  following Writer's row/table spacing path and Typst's resolved grid gutter
  direction. Remaining row-style work is exact row-height propagation for
  conditional styles and row-level justification exceptions.
- Style-chain run property resolution now keeps explicit boolean overrides
  separate from the concrete `TextStyle`, so derived paragraph/character styles
  can turn off base-style values such as bold and underline. This matches the
  Writer `PropertyMap` overlay direction where `w:b w:val="false"` is a real
  override, not absence of a property.
- Continue paragraph keep behavior beyond the current body/text-frame checks:
  line-accurate `keepLines` and widow/orphan split decisions are present as a
  text-frame policy. Rejected page splits now trigger a one-shot move-forward
  reflow to the next column/page leaf; next refinements should replace this
  with a persistent master/follow chain that can move lines backward as well as
  forward when later frames change.
- Continue the current `TextFrameLayout`/`TextFrame`/`LineFrame` path toward
  Writer-like page text follows instead of direct page-item emission. Line
  advance, page-break reset, column-break reset, and wrap-bound recalculation
  are now owned by `TextFrameLayout`, line fragments carry inline cursor split
  candidates, column-break inline continuation is active, and page overflow
  records follow starts. Widow/orphan and `keepLines` decisions are applied to
  those split candidates; rejected decisions now drive one forward reflow pass.
  Natural line overflow also propagates the destination flow to subsequent
  body blocks. Wrapped justified lines now expand word spacing at the line
  boundary, matching the Writer `SpaceAdd` direction and Typst's line-level
  justifiable adjustment rather than paragraph-level shifting. Remaining work
  is durable master/follow state, backward reflow, and repaint invalidation when
  following frames shrink or grow.
- Model tabs, indents, spacing, borders, shading, bidi, and justification as
  layout properties.
- Import `w:contextualSpacing` through the OOXMLSDK paragraph property model and
  suppress adjacent matching paragraph before/after spacing in the block
  sequence, following Writer's contextual frame spacing direction instead of
  blindly adding both paragraph margins.
- Resolve run font properties sufficiently to choose fonts and measure text.
- Preserve field runs as typed dynamic markers where the value depends on
  layout. PAGE and NUMPAGES are resolved after pagination for complex fields,
  simple fields, and `w:pgNum`; unsupported fields continue to use cached
  results until their Writer field type is mapped.
- Import document background and section page borders from generated
  `w::DocumentBackground` and `w::PageBorders` types. Solid background colors
  and basic single-line page borders are present; theme colors, art borders,
  background shapes/images, shadow/frame effects, and full border display rules
  remain later Writer page-frame work.

Minimal tests:

- Style inheritance matching existing fixtures.
- Keep-next paragraph pair does not split when avoidable.
- Page-break-before starts a new page.
- First-line/hanging indent affects list text placement.

### Phase 3: Text Frame and Line Layout

LibreOffice reference:

- `sw/source/core/text/frmform.cxx`
- `sw/source/core/text/itrtxt.cxx`
- `sw/source/core/text/itrform2.cxx`
- `sw/source/core/text/porlay.cxx`
- `sw/source/core/text/txtpaint.cxx`

Typst technique reference:

- `typst-layout/src/inline/linebreak.rs`
- `typst-layout/src/inline/line.rs`
- `typst-pdf/src/text.rs`

Implement:

- Separate line breaking from page item emission.
- Produce `LineBox` values with ascent/descent/width.
- Support manual line breaks and page breaks.
- Preserve imported paragraph line height across every formatted line in a
  `TextFrame`, matching Writer's line-space calculation path instead of
  resetting follows to the renderer default.
- Preserve `w:spacing/@lineRule` as layout state: `auto` keeps proportional
  240ths-of-a-line behavior, `atLeast` supplies a minimum line box, and `exact`
  keeps the imported fixed advance even when run glyph extents are taller.
- Apply justification as line-level word-space expansion for wrapped lines;
  mandatory breaks and final paragraph lines are left unexpanded.
- Include run baseline shifts in line advance so superscript/subscript text
  expands the inline line extent before PDF painting, matching Writer's line
  portion sizing and Typst's inline box behavior.
- Preserve text shaping and bidi measurement through `rustybuzz` or equivalent.
- Extend justification beyond simple word spaces to CJK/kana compression and
  shaped glyph adjustability.
- Keep fallback behavior deterministic when system fonts are missing.

Minimal tests:

- Unicode/CJK line breaking.
- Bidi text order does not corrupt measurement/paint.
- Justified line spacing changes glyph/word positions.
- Superscript/subscript affect line ascent/descent.

### Phase 4: Footnotes and Endnotes

LibreOffice reference:

- `sw/source/core/text/txtftn.cxx`
- `sw/source/core/layout/ftnfrm.cxx`
- `sw/source/core/text/porftn.hxx`

Implement:

- Keep footnote/endnote references in the inline stream.
- Allocate footnote content into page footnote areas. Adapter-level support is
  present; continue toward real Writer footnote frames and continuation areas.
- Reduce body text area for footnote content on the page where references
  occur. Adapter-level pre-reservation is present.
- Support separator presence/absence and basic separator alignment.
- Handle table and row split interactions incrementally.

Minimal tests:

- Footnote appears on same page as reference.
- Body text is pushed when footnote area consumes space.
- Multiple footnotes preserve reference order.
- Missing separator produces no separator line.

### Phase 5: Tables

LibreOffice reference:

- `sw/source/core/layout/tabfrm.cxx`
- `sw/source/core/inc/tabfrm.hxx`
- `sw/source/core/layout/frmtool.cxx`
- `sw/source/core/layout/calcmove.cxx`

Implement:

- Extend the current table/row/cell frames toward full Writer follow-table
  behavior.
- Resolve table grid, preferred width, cell width, grid span, vertical merge.
  Absolute/percent preferred table width, table alignment, indentation,
  `tcW` cell preferred widths including spanned cells, `gridSpan`, vertical
  merge import/continuation shading, and table/row/cell frame ownership are
  present; remaining work is full grid conflict resolution, rowspan-aware split
  recalculation, and full Word border-style precedence. Cell content now flows
  through the shared paragraph/table layout path instead of clipped single-line
  paint, adjacent internal borders prefer the stronger visible stroke, and
  missing `tblCellMar` uses the ECMA `115` twip start/end default instead of
  synthetic equal padding. `tblCellSpacing` is present as a gutter
  between cells/rows, including row-level spacing overrides; remaining work is
  exact compatibility-mode border-distance adjustment. Row `gridBefore/gridAfter`
  skipped columns are included in the table grid and cell placement. Table style
  `wholeTable`, `firstRow`, `lastRow`, and horizontal band formatting is wired
  for table alignment/indent/spacing, row header/cantSplit/spacing, and cell
  shading/borders/margins/vertical alignment, including first/last column,
  vertical banding, and corner cells. Row/cell `w:cnfStyle` is imported through
  generated `ConditionalFormatStyle` and folded into the same Writer-style row
  mask plus cell mask that LibreOffice uses before resolving corner conditions.
  Adjacent collapsed borders now follow LibreOffice's `SvxBorderLine`
  priority shape by comparing rendered width first, then preferring a simple
  line over a compound line when widths tie. Direct cell borders are resolved
  side-by-side instead of replacing the full style border set whenever
  `tcBorders` is present. Multi-row vertical merge continuations now walk past
  intermediate continuation cells to find the real origin cell, matching
  Writer's row-span lookup direction for shading and border ownership. The
  origin cell's content layout height now spans continuous `vMerge`
  continuation rows, including row spacing, which moves the implementation
  closer to Writer and Typst rowspan layout semantics.
  Remaining work is full Word priority/exception handling for conflicting
  direct and style borders.
- Table row overflow now advances through section leaves and returns the
  destination flow to subsequent body blocks. Rows with `cantSplit=false` can
  emit basic follow-flow-line fragments across leaves: the current fragment is
  clipped to the remaining body area, follow fragments continue cell content
  through the nested cell flow, and repeated header rows are inserted before
  follow fragments when they fit. Remaining follow-table work is rowspan-aware
  split recalculation, exact border conflict handling across split fragments,
  and backward move/repaint invalidation.
- Rows that participate in vertical merges are now detected before the ordinary
  splittable-row fragment path. Those rows move as merge-aware follow units
  instead of being cut like unrelated cells, avoiding the worst rowspan
  corruption until full Writer-style rowspan split recalculation is implemented.
- `cantSplit` rows now use the same one-move guard as other row follows:
  a row that still does not fit after moving to its destination leaf is laid out
  there, matching Writer's loop-control direction.
- Compute row height from cell content frames.
- Support row splitting and `cantSplit`.
- Support table headers repeating across pages.
- Preserve border conflict behavior incrementally.

Minimal tests:

- Multi-page table splits rows.
- `cantSplit` row moves to next page.
- Header row repeats.
- Vertical merge across split boundary behaves consistently.

### Phase 6: Anchored and Floating Objects

LibreOffice reference:

- `sw/source/core/text/txtfly.cxx`
- `sw/source/core/layout/fly*.cxx`
- `sw/source/core/layout/anchoredobject.cxx`
- `sw/source/core/layout/objectformatter*.cxx`
- `writerfilter/dmapper/GraphicImport.cxx`

Implement:

- Distinguish:
  - inline drawing
  - as-character anchored object
  - paragraph/character/page anchored floating object
- Preserve `wp:anchor` position and wrap properties.
- Resolve relative horizontal/vertical positions, including basic
  left/center/right and top/center/bottom `wp:align` values.
- Top/bottom wrap now moves following inline text to the next column/page when
  the object leaves no remaining body space in the current leaf.
- Reserve text wrap exclusion areas during line layout. Paragraph-local
  square/tight exclusion is present; multi-paragraph/page-level fly influence is
  still pending.
- Support basic z-order and page association. `behindDoc` and foreground
  floating image ordering are present for page image items; page reassignment
  after master/follow movement and cross-paragraph wrap influence remain.

Minimal tests:

- Inline image participates in line height.
- Floating image with square wrap changes line widths.
- Behind/in-front wrapping modes do not reserve text area.
- Page-relative anchor stays on expected page.

### Phase 7: PDF Paint Quality

LibreOffice reference:

- `filter/source/pdf/pdfexport.cxx`
- `sw/source/core/text/EnhancedPDFExportHelper.cxx`

Typst technique reference:

- `typst-pdf/src/convert.rs`
- `typst-pdf/src/text.rs`
- `typst-pdf/src/paint.rs`
- `typst-pdf/src/tags/`

Implement:

- Convert layout frames to paint items.
- Render text using shaped glyph output where possible. The renderer now emits
  krilla glyph runs from rustybuzz-shaped text, matching the Typst paint path
  more closely than string-level `draw_text`.
- Add images, fills, strokes, clipping, and transforms.
- Raster image output is active for inline/floating DOCX images. Current scope
  covers JPEG direct output and decoded sampled-image output for PNG/GIF/WebP
  style raster inputs, plus DrawingML `srcRect` crop and basic `xfrm`
  rotation/flip paint transforms, VML `v:imagedata` crop, and VML style
  rotation/flip/absolute floating placement; SVG/PDF image embedding, artistic
  effects, and full graphic attributes remain later paint quality work.
- Carry external hyperlink relationships into text/image layout and emit PDF
  link annotations. Footnote/endnote reference backlinks now use internal PDF
  destinations. Export paragraph outline levels as a hierarchical PDF
  outline/bookmark tree. General clicked bookmark destinations remain future
  work and should follow LibreOffice's document target mapping rather than
  ad-hoc anchors.
- Add tagging/PDF-UA/PDF-A only after structure and font policy are ready.

Minimal DOCX PDF smoke tests are no longer the primary gate. DOCX export
behavior should be added to `ooxmlsdk-pdf-test` as LibreOffice/PDFium parity
coverage. `crates/ooxmlsdk-pdf` may keep targeted unit tests for pure import,
layout, or paint-model transformations, but it should not grow new byte-string
PDF assertions for DOCX rendering behavior.

## 6. Testing Strategy

Tests should be behavior-oriented and tied to source references.

Each new renderer behavior should include:

- OOXML fixture, preferably generated in
  `../ooxmlsdk-test-suite/crates/ooxmlsdk-test` or copied from local
  LibreOffice/Open XML references when licensing allows.
- A comment naming the LibreOffice reference file/function used.
- Assertions against the internal model or layout model before asserting only
  PDF bytes.

Recommended test layers:

1. Import model tests: section list, property resolution, resources.
2. Layout model tests: pages, frames, line boxes, anchors, footnote areas.
3. Paint model tests: deterministic paint item order and geometry.
4. PDF smoke tests: valid PDF output and essential object presence.

Avoid relying only on "PDF starts with `%PDF-`" once layout behavior exists.

## 7. Guardrails

- Do not add new DOCX behavior by guessing from visual intuition.
- Do not collapse `wp:anchor` into inline content except as an explicit,
  documented temporary fallback.
- Do not append footnotes/endnotes/comments as normal body content once page
  layout begins to support note areas.
- Do not extend XLSX/PPTX conversion while DOCX layout is being corrected.
- Do not bypass generated `ooxmlsdk` types when typed access exists.
- Do not claim PDF/A conformance until font embedding, output intents, color
  policy, and metadata are intentionally implemented. Claim PDF/UA only for
  output accepted by the Krilla validator and never mutate that output with an
  untagged post-processor afterward.

## 8. Immediate Next Step

The section/page-style backbone, upstream DOCX/PDF calibration fixture lane,
link annotation parity for the current 5-fixture set, and concrete layout frame
records are now in place. Continue with Writer-frame alignment in larger
behavior batches:

1. Make checkpoint reruns influence-aware by feeding fly/footnote/table
   reservations back into the rerun's available region before formatting the
   restarted block.
2. Extend paint glyph runs with full Writer line portions: bidi clusters,
   tab/field portions, justification expansion, and per-portion clipping.
3. Use table row/cell fragment ranges for row-span split recalculation,
   split-border ownership, and repeated header repaint after row fragments.
4. Extend floating frame influence from paragraph-local exclusions to
   page/frame-associated fly influence that can affect following paragraphs and
   table cell content.
5. Expand `../ooxmlsdk-test-suite/corpus/LibreOffice/` with focused LibreOffice QA DOCX files
   only when the next behavior batch is ready to make those fixtures pass
   without known-issue whitelists.

The fastest route toward LibreOffice parity is to keep each batch anchored to a
specific LibreOffice function or fixture group, then assert import/layout
snapshots before PDF-byte smoke checks.
