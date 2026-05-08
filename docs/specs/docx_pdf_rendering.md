# DOCX to PDF Rendering Plan

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

The current `ooxmlsdk-pdf` implementation proves that typed OOXML input can
produce PDF bytes, but its layout logic is too simplified to grow into a
complete renderer without drifting away from Word/LibreOffice behavior.

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
  -> DocxDocument { page, blocks, headers, footers, notes }
  -> layout::layout
  -> LayoutDocument { pages: Vec<Page { items: Vec<PageItem> }> }
  -> render::krilla::render
```

This supports smoke-level PDF generation and a set of focused features:

- text extraction
- basic run styling
- paragraph spacing, indentation, tabs, and alignment
- styles, numbering, simple fields, hyperlinks, SDT content
- headers/footers for a single effective section
- footnotes/endnotes/comments as appended note blocks
- basic inline images and anchor images treated as inline images
- basic table rows/cells/borders/shading
- VML and DrawingML textbox text extraction in limited cases

Known architectural drift:

- Only the final body-level `sectPr` is treated as the page setup.
- Mid-document section breaks are not modeled as section boundaries.
- Header/footer link-to-previous and even/odd variants are not modeled.
- Header/footer dimensions are not computed like Writer page styles.
- Footnotes do not reserve space on the current page.
- Anchored/floating drawings are collapsed into inline images.
- Tables are laid out using simplified row heights and no real splitting model.
- Paragraph layout is a direct greedy write to page items, leaving no line/frame
  model for Writer-like pagination.

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

The DOCX/PDF lane should optimize for large LibreOffice-aligned coverage
increments, not one fixture per tiny property. New coverage should prefer
feature-cluster fixtures that exercise a whole Writer behavior area at once.

Default fixture groups:

- `docx_sections_layout.docx`: section breaks, page size/orientation, page
  parity, first/default/even header/footer inheritance, columns, hard column
  breaks, and explicit non-equal column definitions.
- `docx_paragraph_flow.docx`: spacing, indents, tabs, borders/shading,
  page-break-before, keep-next, keep-lines, widow/orphan scenarios, numbering,
  and common run properties.
- `docx_tables_layout.docx`: table width/indent/alignment, grid/span/merge,
  cell margins, row height, cantSplit, repeated header rows, row/page splitting,
  borders, shading, and nested tables.
- `docx_drawing_flow.docx`: inline drawings, anchors, relative positioning,
  wrap modes, behind/in-front state, VML fallback, and text boxes.
- `docx_notes_layout.docx`: footnote/endnote references, separators,
  continuation separators, bottom note areas, notes in columns/tables, and
  fallback ordering.

Small single-purpose DOCX fixtures are reserved for bug regressions or cases
where a minimal isolated document is necessary to expose an import/layout
failure. Existing small fixtures can remain while behavior is unstable, but new
coverage should be added to the clustered fixtures first. Tests should assert a
layout snapshot for the cluster: page count, key text positions, repeated
content counts, line/fill/image counts, and note/header/table placement. This
keeps verification broad enough for rapid development while preserving the path
to precise LibreOffice parity.

Implementation work should also happen in feature batches. A batch may import
more properties than layout consumes immediately, provided the model fields are
typed, source-backed by generated `ooxmlsdk` types, documented in this file,
and marked as pending layout consumption in the matrix/status text.

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

- Added a typed section collector in `crates/ooxmlsdk-pdf/src/docx/mod.rs`.
- The collector reads `w::Body.body_choice`, paragraph
  `w::ParagraphProperties.section_properties`, and final
  `w::Body.w_sect_pr` through generated `ooxmlsdk` types.
- `DocxDocument.sections` now carries ordered `ImportedSection` entries with
  section break kind, section properties, page setup, `titlePg`, and blocks.
- The existing flat `DocxDocument.blocks` field is still populated for
  compatibility, but `layout::layout` now consumes `DocxDocument.sections`
  when present and uses each section's page setup for body flow.
- Section page breaks currently start new pages for `nextPage`, `nextColumn`,
  `evenPage`, and `oddPage`; `continuous` keeps flowing on the current page.
  This is still an adapter layer, not the final Writer frame tree.
- `evenPage` and `oddPage` section breaks insert an adapter-level blank page
  when needed so the following section starts on the requested page parity.
- Header/footer import now resolves default/first/even slots per section.
- Missing section header/footer references inherit the previous section's slot,
  matching LibreOffice's `CopyLastHeaderFooter` direction at the content level.
- `w:evenAndOddHeaders` is read from settings before selecting even-page
  header/footer slots.
- Layout pages now track their source section index so repeating content can be
  selected per section instead of only from document-level fallback slots.
- Header/footer repeating areas now follow the Writer margin-distance shape from
  `PrepareHeaderFooterProperties`: header starts at `w:pgMar/@header` and ends
  at the top body margin, footer starts at the bottom body margin and ends at
  `w:pgMar/@footer`, with a 1mm minimum area height.
- Section `w:cols` is now imported through generated `w::Columns` into
  `SectionColumns`, including count, gap, separator flag, and explicit
  non-equal `w:col` width/space definitions when `equalWidth` is off.
- Layout has an adapter-level equal-column flow at block boundaries, plus
  separator painting when `w:cols/@sep` is enabled. Paragraph-internal and
  table-internal column splitting remain frame-model work.
- Run-level `w:br w:type="column"` is imported as a distinct typed
  `InlineItem::ColumnBreak`. In multi-column sections it advances following
  block content to the next column; in single-column or final-column contexts
  it follows the LibreOffice/Typst fallback shape and behaves like a page
  break. Inline continuation after the break is still deferred until the
  Writer-like line/frame model lands.
- Footnotes/endnotes are now imported as typed id-to-block maps, and paragraphs
  retain generated-type-derived footnote/endnote reference ids.
- Footnote content is laid out immediately after the referencing paragraph in
  the current section flow, instead of only being appended after all document
  body content. This is still an adapter step toward Writer's true page
  footnote area.
- Paragraph keep properties are now imported through generated paragraph
  property types: `w:keepNext` maps to `keep_with_next`, `w:keepLines` maps to
  `keep_lines`, and `w:pageBreakBefore` remains the page-break-before signal.
  Layout has an adapter-level preflight that advances to the next column/page
  when a keep group does not fit in the current flow region.
- Table row properties are imported through generated `w::TableRowProperties`:
  `w:tblHeader` marks consecutive leading repeated header rows and
  `w:cantSplit` is retained on the row model. Layout repeats valid table header
  rows after page breaks, following LibreOffice's constraints that all-row
  headers and more than 10 header rows do not repeat.
- Table and cell margins are imported through generated `w::TableCellMarginDefault`
  and `w::TableCellMargin` types. Layout uses those margins for row height,
  content inset, and vertical alignment instead of a fixed padding constant,
  matching LibreOffice's cell border-distance import path.
- DrawingML anchors are imported through generated `wp::Anchor` types into a
  typed floating image placement model. The adapter consumes `positionH`,
  `positionV`, wrap mode, `behindDoc`, and anchor text distances for initial
  page/margin/column/paragraph-relative placement. This follows LibreOffice's
  `GraphicImport` direction while keeping full wrap avoidance and z-ordering as
  frame-model work.
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
- Real DOCX fixture coverage includes section header/footer inheritance and
  even-page header selection via `test-data/wml/header_section_inheritance.docx`.
- Real DOCX fixture coverage includes first-page header/footer selection on a later
  section via `test-data/wml/header_section_first_page.docx`.
- Real DOCX fixture coverage includes block-level section column flow, a hard
  column break, and explicit non-equal column definitions via
  `test-data/wml/section_columns_flow.docx`,
  `test-data/wml/section_column_break.docx`, and
  `test-data/wml/section_columns_explicit.docx`.
- Real DOCX fixture coverage includes keep-next page migration via
  `test-data/wml/para_keep_flow.docx`.
- Real DOCX fixture coverage includes repeated table headers across pages via
  `test-data/wml/table_header_repeat.docx`.
- Real DOCX fixture coverage includes floating DrawingML image anchor import
  and placement via `test-data/wml/image_floating.docx`.
- Verification for DOCX/PDF iteration currently includes
  `cargo test -p ooxmlsdk-pdf` and
  `cargo clippy -p ooxmlsdk-pdf --all-targets -- -D warnings`. Broader
  workspace clippy is reserved for shared/runtime/generator changes.

Remaining Phase 1 work:

- Handle negative header/footer margins using LibreOffice's text-frame fallback
  direction from `HandleMarginsHeaderFooter`.
- Replace the adapter with a Writer-like page/frame layout that can enforce
  paragraph-internal column splitting and inline continuation after column
  breaks, bottom footnote areas, and floating frames.

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
- Finish paragraph keep behavior beyond the current adapter preflight:
  - line-accurate `keepLines`
  - widow/orphan control
- Model tabs, indents, spacing, borders, shading, bidi, and justification as
  layout properties.
- Resolve run font properties sufficiently to choose fonts and measure text.

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
- Preserve text shaping and bidi measurement through `rustybuzz` or equivalent.
- Add justification as line-level adjustment, not paragraph-level item shifting.
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
- Allocate footnote content into page footnote areas.
- Reduce body text area for footnote content on the page where references
  occur.
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

- Introduce table/row/cell frames.
- Resolve table grid, preferred width, cell width, grid span, vertical merge.
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
- Resolve relative horizontal/vertical positions.
- Reserve text wrap exclusion areas during line layout.
- Support basic z-order and page association.

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
- Render text using shaped glyph output where possible.
- Add images, fills, strokes, clipping, and transforms.
- Add link annotations and outline/bookmark destinations after layout is
  stable.
- Add tagging/PDF-UA/PDF-A only after structure and font policy are ready.

Minimal tests:

- PDF bytes valid.
- Text is extractable.
- Images are embedded.
- Links become PDF annotations.
- Basic page labels are preserved when page numbering is implemented.

## 6. Testing Strategy

Tests should be behavior-oriented and tied to source references.

Each new renderer behavior should include:

- OOXML fixture, preferably generated in `crates/ooxmlsdk-test` or copied from
  local LibreOffice/Open XML references when licensing allows.
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
- Do not add PDF/A or PDF/UA public claims until tagging, font embedding, color
  policy, and metadata are intentionally implemented.

## 8. Immediate Next Step

Implement Phase 1 in small commits:

1. Add a typed section collector for `w:sectPr`.
2. Add internal section/page-style data structures.
3. Keep existing layout output working through an adapter.
4. Add section-focused tests before touching paragraph, table, or floating
   layout.

This establishes the page and section backbone needed for every later
LibreOffice-aligned feature.
