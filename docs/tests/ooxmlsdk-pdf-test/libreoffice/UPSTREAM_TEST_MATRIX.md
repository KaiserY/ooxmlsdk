# LibreOffice DOCX -> PDF Upstream Test Matrix

This matrix tracks LibreOffice upstream tests that are useful for calibrating
`ooxmlsdk-pdf` DOCX -> PDF rendering. The source of truth is the local
LibreOffice checkout at `../core`.

This is intentionally not a DOCX parser/import matrix. Pure UNO model checks,
DOCX export XML checks, round-trip checks, editing mechanics, and package/security
tests stay out unless the upstream assertion can be faithfully projected to a
PDF-visible property.

## Inclusion Rules

- Include each upstream test as its own row.
- `covered` means the fixture exists in `test-data/ooxmlsdk-pdf-test/libreoffice/`
  and a Rust PDF test asserts the upstream-backed behavior.
- `planned` means the upstream test already exports or inspects PDF output, but
  no local Rust PDF test exists yet.
- `mapped` means the upstream assertion is DOCX -> PDF visible-output evidence
  and is an active long-term TDD target. It may still need projection to PDF
  text/path/image/color/bounds/raster assertions before a concrete Rust test is
  written.
- `review` means the broad scan found a DOCX test with PDF/layout/metafile/
  bitmap signals, but the test still needs semantic review before it is promoted
  to `mapped`.
- `deferred` means the source is useful background only; do not migrate until a
  concrete PDF-visible assertion is identified.
- Expected values must come from LibreOffice assertions, LibreOffice reference
  output, or fixture evidence. Do not invent expected values from current
  `ooxmlsdk-pdf` output.

## Rust Portability Gate

This matrix is a DOCX -> rendered PDF matrix, not a LibreOffice UNO model
matrix. Missing Rust implementation is not an exclusion reason: matching
LibreOffice behavior is the point of this suite. A row is safe to keep as an
active PDF target when the LibreOffice assertion can be represented now or later
by one of these Rust-side mechanisms:

- direct PDF extraction already used by `ooxmlsdk-pdf-test`: page count, text
  objects, font size, fill/stroke color, images, annotations, widgets,
  bookmarks, or form values
- source-backed projection from LibreOffice layout dump or metafile output to
  PDF text/path/image/color/bounds assertions
- raster/reference comparison, if a future test harness adds Typst-style
  render/PDF reference artifacts instead of asserting values invented from the
  current Rust output

Keep tests out of the active PDF target set only when they are not source
DOCX -> visible PDF behavior, or when no faithful observation path is available.
Do not migrate these as active PDF tests:

- `getShape()`, `XPropertySet`, chart model, style model, importer state, or
  OOXML export XML assertions unless the same expectation is also visible in
  final PDF output
- editing, cursor, undo, sidebar, or command-dispatch tests
- crash/export-completion smoke tests that do not assert visible PDF behavior
- layout dump XPath checks copied literally without a current or planned PDF
  text/path/image/bounds/raster projection

The comparison with `../typst` is only methodological: Typst accepts output
tests through deterministic PDF/SVG/render references and dedicated PDF-tag
outputs. For this matrix, `covered` direct-PDF rows are the direct analogue;
`mapped` rows are active TDD targets that need a faithful projection path, and
`review` rows are not active migration targets yet.

## Portability Tiers

| Tier | Matrix rows | Count | Rust migration meaning |
|---|---:|---:|---|
| Direct PDF/object | `Direct PDF Tests` covered rows | 10 | Expressed with current PDF extraction assertions. |
| Covered supplemental | `Covered Supplemental Tests` rows | 7 | Already represented locally with source-backed visible PDF color/alpha/text assertions. |
| Projection required | `mapped` rows | 277 | Active DOCX -> PDF TDD targets; add the required PDF projection or snapshot capability as needed. |
| Review only | `review` rows | 0 | Broad-scan rows have been item-reviewed in this pass. |
| Deferred/excluded | `deferred` / `excluded` rows | 26 | Keep out of the active PDF migration queue. |

## Rust Feasibility Calibration

The active rows were checked against the Rust-side test stack in
`crates/ooxmlsdk-pdf-test` and the rendering/export stack in `crates/ooxmlsdk-pdf`.
No ecosystem-level blocker was found. Current dependencies already cover the
necessary observation surfaces:

- `pdfium-render`: rendered pages, page objects, text segments/chars, paths,
  images, links, annotations, and object bounds
- `lopdf`: raw PDF dictionaries, XObjects, annotations, outlines/forms, content
  streams, and low-level PDF normalization
- `image`, `png`, `crc32fast`: raster snapshots, image dimensions/content, and
  deterministic page/image hashes

No mandatory new rendering library is required for the listed tests. Future
snapshot ergonomics may justify adding a diff/approval helper crate, but that is
test harness convenience, not a capability blocker.

| Assertion class | Active rows | Rust path | Faithfulness note |
|---|---:|---|---|
| Direct PDF/object | 45 | Existing PDFium/lopdf extraction, plus small extensions for outline/form dictionaries when needed. | Expected values can be copied from LibreOffice PDF assertions. |
| Layout dump projection | 198 | Convert LibreOffice layout dump frame/text/table/page assertions to PDF text/path/image bounds, object counts, text order, or page counts. | Do not copy XPath literally; preserve the upstream geometry/text relation after unit conversion and tolerances. |
| Raster/bitmap | 21 | Use PDFium page rendering plus `image`/`crc32fast`/PNG references for page or region snapshots. | Use LibreOffice pixel/color/bitmap expectation or checked-in reference images; do not derive expected images from current Rust output. |
| Metafile/render XML | 6 | Project LibreOffice metafile text/path/polyline assertions to PDF text/path objects or raster snapshots. | Preserve the asserted rendered primitive, not the LibreOffice XML format. |
| Graphics/color/effects | 12 | Assert PDF text/path/image colors, alpha, stroke/fill, bounds, or fall back to raster snapshots for effects. | Theme/color/effect expected values must remain LibreOffice-derived. |
| Other visible output | 34 | Mostly chart/layout/rendered-output cases; use text/path/image extraction first, snapshot when primitives are not stable enough. | Keep as active TDD targets when the final page output is the asserted behavior. |

## Scan Summary

- Local covered PDF-rendering fixtures: 38.
- Direct upstream DOCX -> PDF/object assertions: 10 rows, all covered.
- Planned direct PDF rows remaining: 0.
- Supplemental source-backed PDF-visible assertions already covered locally: 7
  rows.
- Additional visible-output candidates listed individually below: 277 mapped
  tests. Treat these as active TDD targets that may require new PDF extraction,
  layout projection, or snapshot capability before the Rust assertion can be
  written faithfully.
- Active rows are feasible with the current Rust ecosystem and local dependency
  stack; remaining work is implementation/projection/snapshot harness work.
- Broad-scan review candidates have been item-reviewed; none remain in `review`.
  Rows that are visible source DOCX behavior were promoted to `mapped`, while
  edit workflows, export XML checks, and non-source round-trip checks were
  marked `deferred`.
- Main high-value sources:
  - direct PDF export suites: `vcl/qa/cppunit/pdfexport`, scattered Writer/SVX
    direct PDF export tests
  - Writer layout dump and metafile tests: `sw/qa/core/*`,
    `sw/qa/extras/layout/layout*.cxx`
  - tiled bitmap render tests: `sw/qa/extras/tiledrendering`
  - OoXML/SVX drawing tests with renderable shape/image/color expectations
- Chart2 import/export XML tests are not listed as migration targets. Use the
  Writer layout/metafile chart tests instead, because those assert rendered
  chart output.

## Direct PDF Tests

| Upstream test | Fixture | Upstream source | Status | PDF assertion to port |
|---|---|---|---|---|
| `pdfexport2.cxx::testTdf161346` | `fdo47811-1_Word2013.docx` | `../core/vcl/qa/cppunit/pdfexport/data/fdo47811-1_Word2013.docx` | `covered` | Page count is 2. |
| `pdfexport.cxx::testTdf145274` | `tdf145274.docx` | `../core/vcl/qa/cppunit/pdfexport/data/tdf145274.docx` | `covered` | 1 page, 6 page objects, text font size 11, fill render mode, red fill color. |
| `pdfexport.cxx::testTdf156685` | `tdf156685.docx` | `../core/vcl/qa/cppunit/pdfexport/data/tdf156685.docx` | `covered` | 1 page, 9 page objects, text font size 11, fill render mode, black fill color. |
| `pdfexport.cxx::testTdf142133` | `tdf142133.docx` | `../core/vcl/qa/cppunit/pdfexport/data/tdf142133.docx` | `covered` | 1 link annotation with URI `https://google.com/`. |
| `pdfexport2.cxx::testTdf129085` | `tdf129085.docx` | `../core/vcl/qa/cppunit/pdfexport/data/tdf129085.docx` | `covered` | 1 JPEG XObject, 884x925, 24 bpp. |
| `pdfexport2.cxx::testTdf152246` | `content-control-rtl.docx` | `../core/vcl/qa/cppunit/pdfexport/data/content-control-rtl.docx` | `covered` | 5 widget annotations with upstream rectangle coordinates. |
| `svdraw.cxx::testPageViewDrawLayerClip` | `page-view-draw-layer-clip.docx` | `../core/svx/qa/unit/data/page-view-draw-layer-clip.docx` | `covered` | Page 1 has 3 objects, page 2 has 2 objects. |
| `itrform2.cxx::testContentControlHeaderPDFExport` | `content-control-header.docx` | `../core/sw/qa/core/text/data/content-control-header.docx` | `covered` | Page 2 has 3 text objects. |
| `text.cxx::testDropdownContentControlPDF2` | `tdf153040.docx` | `../core/sw/qa/core/text/data/tdf153040.docx` | `covered` | 4 annotations; first widget is a combo box with value `Apfel`. |
| `uiwriter8.cxx::testTdf131728` | `tdf131728.docx` | `../core/sw/qa/extras/uiwriter/data/tdf131728.docx` | `covered` | PDF bookmark order matches upstream expected outline strings. |

## Covered Supplemental Tests

| Upstream test | Fixture | Upstream source | Status | PDF assertion already represented locally |
|---|---|---|---|---|
| `drawingml.cxx::testChartDataLabelCharColor` | `chart-data-label-char-color.docx` | `../core/oox/qa/unit/data/chart-data-label-char-color.docx` | `covered` | Chart data-label text fill is white. |
| `TextEffectsHandler.cxx::testSemiTransparentText` | `semi-transparent-text.docx` | `../core/sw/qa/writerfilter/dmapper/data/semi-transparent-text.docx` | `covered` | Text alpha matches upstream character transparency. |
| `TextEffectsHandler.cxx::testThemeColorTransparency` | `tdf152884_Char_Transparency.docx` | `../core/sw/qa/writerfilter/dmapper/data/tdf152884_Char_Transparency.docx` | `covered` | Theme-color text alpha matches upstream transparency. |
| `shape.cxx::testTdf54095_SmartArtThemeTextColor` | `tdf54095_SmartArtThemeTextColor.docx` | `../core/oox/qa/unit/data/tdf54095_SmartArtThemeTextColor.docx` | `covered` | SmartArt text color resolves to `0x1f497d`. |
| `shape.cxx::testWriterFontwork2` | `tdf125885_WordArt2.docx` | `../core/oox/qa/unit/data/tdf125885_WordArt2.docx` | `covered` | WordArt fill and stroke color/alpha. |
| `shape.cxx::testWriterFontworkNonAccentColor` | `tdf152840_WordArt_non_accent_color.docx` | `../core/oox/qa/unit/data/tdf152840_WordArt_non_accent_color.docx` | `covered` | WordArt non-accent fill colors. |
| `shape.cxx::testWriterFontworkDarkenTransparency` | `tdf152896_WordArt_color_darken.docx` | `../core/oox/qa/unit/data/tdf152896_WordArt_color_darken.docx` | `covered` | Darkened WordArt fill resolves to upstream color. |

## Writer Text And Layout Candidates

These rows were selected because the test block loads a DOCX and asserts layout
dump, PDF output, rendered metafile XML, bitmap output, or other page-visible
behavior.

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `text.cxx::testNumberPortionNoformat` | `number-portion-noformat.docx` | `../core/sw/qa/core/text/text.cxx:1378` | `mapped` | Numbering portion visibility; project to PDF text. |
| `text.cxx::testParaUpperMarginFlyIntersect` | `para-upper-margin-fly-intersect.docx` | `../core/sw/qa/core/text/text.cxx:1577` | `mapped` | Paragraph/fly intersection layout; project to text and object bounds. |
| `calcmove.cxx::testIgnoreTopMargin` | `ignore-top-margin.docx` | `../core/sw/qa/core/layout/calcmove.cxx:29` | `mapped` | Top margin layout; project to text/object bounds. |
| `calcmove.cxx::testIgnoreTopMarginTable` | `ignore-top-margin-table.docx` | `../core/sw/qa/core/layout/calcmove.cxx:46` | `mapped` | Table top margin layout; project to PDF table/text bounds. |
| `calcmove.cxx::testIgnoreTopMarginPageStyleChange` | `ignore-top-margin-page-style-change.docx` | `../core/sw/qa/core/layout/calcmove.cxx:87` | `mapped` | Page-style top margin layout; project to page/text bounds. |
| `ftnfrm.cxx::testInlineEndnotePosition` | `inline-endnote-position.docx` | `../core/sw/qa/core/layout/ftnfrm.cxx:121` | `mapped` | Endnote position; project to page/text bounds. |
| `layout.cxx::testTableFlyOverlap` | `table-fly-overlap.docx` | `../core/sw/qa/core/layout/layout.cxx:52` | `mapped` | Table/fly overlap; project to object bounds. |
| `layout.cxx::testTdf128195` | `tdf128195.docx` | `../core/sw/qa/core/layout/layout.cxx:73` | `mapped` | Layout dump assertion; project to page/text/object bounds. |
| `layout.cxx::testBorderCollapseCompat` | `border-collapse-compat.docx` | `../core/sw/qa/core/layout/layout.cxx:91` | `mapped` | Collapsed border rendering; project to PDF paths. |
| `layout.cxx::testTableFlyOverlapSpacing` | `table-fly-overlap-spacing.docx` | `../core/sw/qa/core/layout/layout.cxx:127` | `mapped` | Table/fly spacing; project to object bounds. |
| `layout.cxx::testTextBoxAutoGrowVertical` | `textbox-autogrow-vertical.docx` | `../core/sw/qa/core/layout/layout.cxx:227` | `mapped` | Textbox vertical growth; project to text and shape bounds. |
| `layout.cxx::testTextBoxInHeaderIsPositioned` | `header-textbox.docx` | `../core/sw/qa/core/layout/layout.cxx:258` | `mapped` | Header textbox position; project to PDF text/shape bounds. |
| `layout.cxx::testVerticallyMergedCellBorder` | `vmerge-cell-border.docx` | `../core/sw/qa/core/layout/layout.cxx:464` | `mapped` | Merged-cell border; project to PDF paths. |
| `layout.cxx::testInnerCellBorderIntersect` | `inner-border.docx` | `../core/sw/qa/core/layout/layout.cxx:562` | `mapped` | Inner table borders; project to PDF paths. |
| `layout.cxx::testDoubleBorderVertical` | `double-border-vertical.docx` | `../core/sw/qa/core/layout/layout.cxx:677` | `mapped` | Vertical double border; project to PDF paths. |
| `layout.cxx::testDoubleBorderHorizontal` | `double-border-horizontal.docx` | `../core/sw/qa/core/layout/layout.cxx:725` | `mapped` | Horizontal double border; project to PDF paths. |
| `layout.cxx::testParaBorderInCellClip` | `para-border-in-cell-clip.docx` | `../core/sw/qa/core/layout/layout.cxx:773` | `mapped` | Paragraph border clipping; project to PDF clipping/path checks. |
| `layout.cxx::testDoublePageBorder` | `double-page-border.docx` | `../core/sw/qa/core/layout/layout.cxx:793` | `mapped` | Page border geometry; project to PDF paths. |
| `paintfrm.cxx::testRTLBorderMerge` | `rtl-table.docx` | `../core/sw/qa/core/layout/paintfrm.cxx:75` | `mapped` | RTL table border merge; project to PDF paths. |
| `paintfrm.cxx::testInlineEndnoteSeparatorPosition` | `inline-endnote-position.docx` | `../core/sw/qa/core/layout/paintfrm.cxx:163` | `mapped` | Endnote separator position; project to PDF paths. |
| `paintfrm.cxx::testEndnoteContSeparator` | `endnote-cont-separator.docx` | `../core/sw/qa/core/layout/paintfrm.cxx:193` | `mapped` | Endnote continuation separator; project to PDF paths. |
| `paintfrm.cxx::testTableRedlineRenderMode` | `redline-table.docx` | `../core/sw/qa/core/layout/paintfrm.cxx:235` | `mapped` | Redline table render mode; project to visible text/path changes. |
| `tabfrm.cxx::testTablePrintAreaLeft` | `table-print-area-left.docx` | `../core/sw/qa/core/layout/tabfrm.cxx:35` | `mapped` | Table print-area position; project to table/text bounds. |
| `objectpositioning.cxx::testVertAlignBottomMarginWithFooter` | `bottom-margin-with-footer.docx` | `../core/sw/qa/core/objectpositioning/objectpositioning.cxx:184` | `mapped` | Floating object vertical alignment; project to image/shape bounds. |
| `objectpositioning.cxx::testInsideOutsideVertAlignBottomMargin` | `inside-outside-vert-align.docx` | `../core/sw/qa/core/objectpositioning/objectpositioning.cxx:258` | `mapped` | Inside/outside vertical alignment; project to shape bounds. |
| `objectpositioning.cxx::testVMLVertAlignBottomMargin` | `vml-vertical-alignment.docx` | `../core/sw/qa/core/objectpositioning/objectpositioning.cxx:279` | `mapped` | VML vertical alignment; project to shape bounds. |
| `itrpaint.cxx::testRedlineRenderModeOmitInsertDelete` | `redline.docx` | `../core/sw/qa/core/text/itrpaint.cxx:64` | `mapped` | Redline visible text/decoration output. |
| `itrpaint.cxx::testMoveRedlineRenderModeOmitDelete` | `redline-move.docx` | `../core/sw/qa/core/text/itrpaint.cxx:156` | `mapped` | Move redline visible text/decoration output. |
| `itrpaint.cxx::testAnchoredImageRedlineRenderModeOmitInsertDelete` | `redline-image-anchored.docx` | `../core/sw/qa/core/text/itrpaint.cxx:269` | `mapped` | Anchored image redline visibility; project to image/text output. |
| `itrpaint.cxx::testInlineImageRedlineRenderModeOmitInsertDelete` | `redline-image-inline.docx` | `../core/sw/qa/core/text/itrpaint.cxx:325` | `mapped` | Inline image redline visibility; project to image/text output. |
| `porfld.cxx::testNumberPortionRedlineRenderMode` | `redline-number-portion.docx` | `../core/sw/qa/core/text/porfld.cxx:31` | `mapped` | Redline number portion; project to PDF text. |
| `porfld.cxx::testTabPortionRedlineRenderMode` | `redline-bullet.docx` | `../core/sw/qa/core/text/porfld.cxx:70` | `mapped` | Redline bullet/tab portion; project to PDF text/glyphs. |

## Writer Extras Layout Candidates

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `layout.cxx::TestTdf136588` | `tdf136588.docx` | `../core/sw/qa/extras/layout/layout.cxx:276` | `mapped` | Layout dump; project to page/text/object bounds. |
| `layout.cxx::testTdf88496` | `tdf88496.docx` | `../core/sw/qa/extras/layout/layout.cxx:949` | `mapped` | Layout dump; project to page/text/object bounds. |
| `layout.cxx::TestTdf137025` | `tdf137025.docx` | `../core/sw/qa/extras/layout/layout.cxx:1438` | `mapped` | Detailed layout dump; project to PDF text/object bounds. |
| `layout.cxx::TestTdf134277` | `tdf134277.docx` | `../core/sw/qa/extras/layout/layout.cxx:2192` | `mapped` | Layout regression; project to page/text bounds. |
| `layout.cxx::testTdf116486` | `tdf116486.docx` | `../core/sw/qa/extras/layout/layout.cxx:2206` | `mapped` | Layout regression; project to visible page output. |
| `layout.cxx::TestTdf142080` | `fdo43573-2-min.docx` | `../core/sw/qa/extras/layout/layout.cxx:2217` | `mapped` | Layout regression; project to page/text output. |
| `layout.cxx::testTdf128198` | `tdf128198-1.docx` | `../core/sw/qa/extras/layout/layout.cxx:2247` | `mapped` | Layout regression; project to page/text output. |
| `layout.cxx::testTdf106153` | `tdf106153.docx` | `../core/sw/qa/extras/layout/layout.cxx:2284` | `mapped` | Layout regression; project to page/text/object bounds. |
| `layout.cxx::testTdf109137` | `tdf109137.docx` | `../core/sw/qa/extras/layout/layout.cxx:3581` | `mapped` | Layout regression; project to visible page output. |
| `layout.cxx::testTdf157628` | `tdf157628.docx` | `../core/sw/qa/extras/layout/layout.cxx:3701` | `mapped` | Layout regression; project to visible page output. |
| `layout.cxx::testTdf125893` | `tdf125893.docx` | `../core/sw/qa/extras/layout/layout.cxx:3751` | `mapped` | Layout regression; project to visible page output. |
| `layout2.cxx::testTdf165322` | `CT-formatted-deletion.docx` | `../core/sw/qa/extras/layout/layout2.cxx:573` | `mapped` | Formatted deletion visibility; project to PDF text/decoration. |
| `layout2.cxx::tdf157596_paragraph_numbering` | `tdf157596_paragraph_numbering.docx` | `../core/sw/qa/extras/layout/layout2.cxx:678` | `mapped` | Paragraph numbering layout; project to PDF text. |
| `layout2.cxx::testTdf149711_importDOCXMoveToParagraphMark` | `tdf149711.docx` | `../core/sw/qa/extras/layout/layout2.cxx:787` | `mapped` | Move-to paragraph mark rendering; project to PDF text/decoration. |
| `layout2.cxx::testTdf152872` | `hidden-para-separator.docx` | `../core/sw/qa/extras/layout/layout2.cxx:805` | `mapped` | Hidden paragraph separator layout; project to PDF text/spacing. |
| `layout2.cxx::testTdf151954` | `tdf151954.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1189` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testRedlineMovingDOCX` | `tdf104797.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1501` | `mapped` | Redline move rendering; project to PDF text/decoration. |
| `layout2.cxx::testTdf125300` | `tdf125300.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1681` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf122225` | `tdf122225.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1756` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf134247` | `legend-itemorder-min.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1796` | `mapped` | Chart legend order; project to PDF text order. |
| `layout2.cxx::testTdf75659` | `tdf75659.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1811` | `mapped` | Chart/layout regression; project to PDF text/path output. |
| `layout2.cxx::testTdf126425` | `long_legendentry.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1843` | `mapped` | Long legend entry layout; project to PDF text bounds. |
| `layout2.cxx::testUnusedOLEprops` | `tdf138465min.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1860` | `mapped` | OLE preview/layout output; project to image/object checks. |
| `layout2.cxx::testTdf115630` | `tdf115630.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1920` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf128996` | `tdf128996.docx` | `../core/sw/qa/extras/layout/layout2.cxx:2009` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf126244` | `tdf126244.docx` | `../core/sw/qa/extras/layout/layout2.cxx:2023` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout2.cxx::testTdf69648` | `tdf69648.docx` | `../core/sw/qa/extras/layout/layout2.cxx:2080` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf116256` | `tdf116256.docx` | `../core/sw/qa/extras/layout/layout2.cxx:2118` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout3.cxx::testTdf134463` | `tdf134463.docx` | `../core/sw/qa/extras/layout/layout3.cxx:31` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout3.cxx::testTdf117188` | `tdf117188.docx` | `../core/sw/qa/extras/layout/layout3.cxx:39` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout3.cxx::testTdf161718` | `tdf161718.docx` | `../core/sw/qa/extras/layout/layout3.cxx:222` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout3.cxx::testTdf119908` | `tdf130088.docx` | `../core/sw/qa/extras/layout/layout3.cxx:239` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout3.cxx::testTdf158333` | `tdf130088.docx` | `../core/sw/qa/extras/layout/layout3.cxx:257` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout3.cxx::testTdf158419` | `tdf130088.docx` | `../core/sw/qa/extras/layout/layout3.cxx:314` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout3.cxx::testTdf164905` | `tdf164905.docx` | `../core/sw/qa/extras/layout/layout3.cxx:567` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout3.cxx::testTdf163149` | `tdf163149.docx` | `../core/sw/qa/extras/layout/layout3.cxx:583` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout3.cxx::testTdf164499` | `tdf164499.docx` | `../core/sw/qa/extras/layout/layout3.cxx:1295` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout4.cxx::testTdf117982` | `tdf117982.docx` | `../core/sw/qa/extras/layout/layout4.cxx:446` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf128959` | `tdf128959.docx` | `../core/sw/qa/extras/layout/layout4.cxx:458` | `mapped` | Layout regression; project to PDF text/object bounds. |
| `layout4.cxx::testWriterImageNoCapture` | `writer-image-no-capture.docx` | `../core/sw/qa/extras/layout/layout4.cxx:555` | `mapped` | Image capture/layout; project to PDF image/object checks. |
| `layout4.cxx::testTdf124423_DOCX` | `tdf124423.docx` | `../core/sw/qa/extras/layout/layout4.cxx:641` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf138782` | `tdf138782.docx` | `../core/sw/qa/extras/layout/layout4.cxx:706` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf135035_DOCX` | `tdf135035.docx` | `../core/sw/qa/extras/layout/layout4.cxx:722` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf139336_ColumnsWithFootnoteDoNotOccupyEntirePage` | `tdf139336_ColumnsWithFootnoteDoNotOccupyEntirePage.docx` | `../core/sw/qa/extras/layout/layout4.cxx:762` | `mapped` | Columns and footnote layout; project to PDF text/page bounds. |
| `layout4.cxx::TestTdf161348` | `fdo48718-1.docx` | `../core/sw/qa/extras/layout/layout4.cxx:825` | `mapped` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf159271` | `fld-in-tbl.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1072` | `mapped` | Field-in-table layout; project to PDF text/table bounds. |
| `layout4.cxx::testTdf159259` | `sdt+framePr.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1093` | `mapped` | Content control with frame properties; project to PDF text/object bounds. |
| `layout4.cxx::TestTdf155229RowAtLeast` | `tdf155229_row_height_at_least.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1670` | `mapped` | Row height layout; project to PDF table bounds. |
| `layout4.cxx::TestTdf164907_rowHeightAtLeast` | `tdf164907_rowHeightAtLeast.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1682` | `mapped` | Row height layout; project to PDF table bounds. |
| `layout4.cxx::testTdf152298` | `tdf152298.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1918` | `mapped` | Layout regression; project to PDF page/text/object assertions. |

## Chart And Metafile Rendering Candidates

These tests are better PDF chart sources than `chart2` import/export tests:
LibreOffice asserts rendered metafile/text/path output instead of chart XML.

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `layout5.cxx::testTdf138194` | `xaxis-labelbreak.docx` | `../core/sw/qa/extras/layout/layout5.cxx:48` | `mapped` | Chart axis label wrap; project to PDF text positions. |
| `layout5.cxx::testTdf138773` | `tdf138773.docx` | `../core/sw/qa/extras/layout/layout5.cxx:108` | `mapped` | Chart rendered text/path output. |
| `layout5.cxx::testTdf130969` | `tdf130969.docx` | `../core/sw/qa/extras/layout/layout5.cxx:184` | `mapped` | Chart rendered geometry. |
| `layout5.cxx::testTdf129054` | `tdf129054.docx` | `../core/sw/qa/extras/layout/layout5.cxx:217` | `mapped` | Chart rendered geometry. |
| `layout5.cxx::testTdf129173` | `testAreaChartNumberFormat.docx` | `../core/sw/qa/extras/layout/layout5.cxx:243` | `mapped` | Area-chart number-format text. |
| `layout5.cxx::testTdf134866` | `tdf134866.docx` | `../core/sw/qa/extras/layout/layout5.cxx:258` | `mapped` | Chart label/shape output. |
| `layout5.cxx::testTdf137116` | `tdf137116.docx` | `../core/sw/qa/extras/layout/layout5.cxx:273` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf137154` | `tdf137154.docx` | `../core/sw/qa/extras/layout/layout5.cxx:293` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf138777` | `outside_long_data_label.docx` | `../core/sw/qa/extras/layout/layout5.cxx:313` | `mapped` | Data label placement; project to PDF text bounds. |
| `layout5.cxx::testTdf130031` | `tdf130031.docx` | `../core/sw/qa/extras/layout/layout5.cxx:331` | `mapped` | Chart line/path rendering. |
| `layout5.cxx::testTdf138018` | `tdf138018.docx` | `../core/sw/qa/extras/layout/layout5.cxx:392` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf130380` | `tdf130380.docx` | `../core/sw/qa/extras/layout/layout5.cxx:409` | `mapped` | Chart geometry. |
| `layout5.cxx::testTdf129095` | `tdf129095.docx` | `../core/sw/qa/extras/layout/layout5.cxx:432` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf132956` | `tdf132956.docx` | `../core/sw/qa/extras/layout/layout5.cxx:447` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf122014` | `tdf122014.docx` | `../core/sw/qa/extras/layout/layout5.cxx:580` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf167202_footnote` | `tdf167202_footnote.docx` | `../core/sw/qa/extras/layout/layout5.cxx:597` | `mapped` | Chart in footnote layout. |
| `layout5.cxx::testTdf134659` | `tdf134659.docx` | `../core/sw/qa/extras/layout/layout5.cxx:621` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf134235` | `tdf134235.docx` | `../core/sw/qa/extras/layout/layout5.cxx:638` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf134676` | `tdf134676.docx` | `../core/sw/qa/extras/layout/layout5.cxx:655` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf134146` | `tdf134146.docx` | `../core/sw/qa/extras/layout/layout5.cxx:672` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf136061` | `tdf136061.docx` | `../core/sw/qa/extras/layout/layout5.cxx:691` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf116925` | `tdf116925.docx` | `../core/sw/qa/extras/layout/layout5.cxx:705` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf117028` | `tdf117028.docx` | `../core/sw/qa/extras/layout/layout5.cxx:725` | `mapped` | Chart rendered output. |
| `layout5.cxx::testTdf150200_DOCX` | `tdf150200.docx` | `../core/sw/qa/extras/layout/layout5.cxx:924` | `mapped` | Chart/metafile text and path output. |
| `layout5.cxx::testTdf150438_DOCX` | `tdf150438.docx` | `../core/sw/qa/extras/layout/layout5.cxx:976` | `mapped` | Chart/metafile text and path output. |
| `layout5.cxx::testTdf127606` | `tdf117923.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1027` | `mapped` | Metafile/text output. |
| `layout5.cxx::testTdf127118` | `tdf127118.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1063` | `mapped` | Rendered output regression. |
| `layout5.cxx::testTdf141220` | `tdf141220.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1072` | `mapped` | Rendered output regression. |
| `layout5.cxx::testTdf134685` | `tdf134685.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1176` | `mapped` | Rendered output regression. |
| `layout5.cxx::testTdf109077` | `tdf109077.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1186` | `mapped` | Rendered output regression. |
| `layout5.cxx::testTdf164903` | `tdf164903.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1297` | `mapped` | Rendered output regression. |
| `layout5.cxx::testTdf153136` | `tdf153136.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1361` | `mapped` | Detailed rendered text/path output. |
| `layout5.cxx::testTdf167526` | `tdf167526.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1923` | `mapped` | Detailed rendered text/path output. |
| `layout5.cxx::testTdf167540` | `tdf167540.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1973` | `mapped` | Detailed rendered text/path output. |

## Late Writer Layout Candidates

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `layout6.cxx::testTdf122878` | `tdf122878.docx` | `../core/sw/qa/extras/layout/layout6.cxx:550` | `mapped` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf115094` | `tdf115094.docx` | `../core/sw/qa/extras/layout/layout6.cxx:572` | `mapped` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf112290` | `tdf112290.docx` | `../core/sw/qa/extras/layout/layout6.cxx:599` | `mapped` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf123651` | `tdf123651.docx` | `../core/sw/qa/extras/layout/layout6.cxx:918` | `mapped` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf64222` | `tdf64222.docx` | `../core/sw/qa/extras/layout/layout6.cxx:1118` | `mapped` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf124600` | `tdf124600.docx` | `../core/sw/qa/extras/layout/layout6.cxx:1319` | `mapped` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf170381_split_float_table_in_normal_table` | `tdf170381-split-float-table-in-normal-table.docx` | `../core/sw/qa/extras/layout/layout6.cxx:1740` | `mapped` | Split floating table layout; project to PDF table bounds and page flow. |
| `layout6.cxx::testTdf170381_split_float_table_in_float_table` | `tdf170381-split-float-table-in-float-table.docx` | `../core/sw/qa/extras/layout/layout6.cxx:1862` | `mapped` | Nested split floating table layout; project to PDF table bounds and page flow. |
| `layout6.cxx::testTdf170620_float_table_after_keep_with_next_para` | `tdf170620.docx` | `../core/sw/qa/extras/layout/layout6.cxx:2018` | `mapped` | Floating table after keep-with-next paragraph; project to PDF table/page flow. |
| `layout6.cxx::testTdf170630` | `tdf170630.docx` | `../core/sw/qa/extras/layout/layout6.cxx:2085` | `mapped` | Layout regression; project to PDF page/text/table output. |
| `layout6.cxx::testTdf170846_1` | `tdf170846_1.docx` | `../core/sw/qa/extras/layout/layout6.cxx:2199` | `mapped` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf170846_2` | `tdf170846_2.docx` | `../core/sw/qa/extras/layout/layout6.cxx:2209` | `mapped` | Layout regression; project to PDF page/text output. |

## Tiled Bitmap Rendering Candidates

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `tiledrendering.cxx::testHighlightNumbering` | `tdf114799_highlight.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:2609` | `mapped` | Highlighted numbering pixels; project to PDF raster/color region. |
| `tiledrendering.cxx::testHighlightNumbering_shd` | `tdf114799_shd.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:2634` | `mapped` | Numbering shading pixels; project to PDF raster/color region. |
| `tiledrendering.cxx::testTdf159626_yellowPatternFill` | `tdf159626_yellowPatternFill.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:3938` | `mapped` | Yellow pattern fill pixels. |
| `tiledrendering.cxx::testTdf159626_yellowPatternFillB` | `tdf159626_yellowPatternFillB.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:3969` | `mapped` | Yellow pattern fill pixels. |
| `tiledrendering.cxx::testTdf159626_blackPatternFill` | `tdf159626_blackPatternFill.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:4000` | `mapped` | Black pattern fill pixels. |

## OoXML And SVX Drawing Candidates

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `vml.cxx::tdf112450_vml_polyline` | `tdf112450_vml_polyline.docx` | `../core/oox/qa/unit/vml.cxx:37` | `mapped` | VML polyline geometry; project to PDF paths. |
| `vml.cxx::testWatermark` | `watermark.docx` | `../core/oox/qa/unit/vml.cxx:184` | `mapped` | Watermark visibility; project to PDF text/path/image output. |
| `helper.cxx::testImportTifCrop` | `tif-crop.docx` | `../core/oox/qa/unit/helper.cxx:30` | `mapped` | TIFF crop rectangle; project to PDF image crop/bounds. |
| `customshapes.cxx::testTdf153000_MS0_SPT_25_31` | `tdf153000_WordArt_type_25_to_31.docx` | `../core/svx/qa/unit/customshapes.cxx:1377` | `mapped` | WordArt preset geometry; project to PDF paths/snapshot. |
| `drawingml.cxx::testCameraRotationRevolution` | `camera-rotation-revolution.docx` | `../core/oox/qa/unit/drawingml.cxx:251` | `mapped` | 3D camera rotation; needs rendered shape/path or snapshot assertion. |
| `drawingml.cxx::testToplevelLineHorOffsetDOCX` | `toplevel-line-hori-offset.docx` | `../core/oox/qa/unit/drawingml.cxx:726` | `mapped` | Line horizontal offset; needs PDF path bounds. |
| `drawingml.cxx::testDOCXVerticalLineRotation` | `line-vertical-rotation.docx` | `../core/oox/qa/unit/drawingml.cxx:776` | `mapped` | Vertical line rotation; needs PDF path geometry. |
| `shape.cxx::testMultipleGroupShapes` | `multiple-group-shapes.docx` | `../core/oox/qa/unit/shape.cxx:307` | `mapped` | Group shape transforms; needs PDF path/text bounds. |
| `shape.cxx::testCustomshapePosition` | `customshape-position.docx` | `../core/oox/qa/unit/shape.cxx:321` | `mapped` | Custom shape position; needs PDF path bounds. |
| `shape.cxx::testTdf151518VertAnchor` | `tdf151518_SmartArtTextLocation.docx` | `../core/oox/qa/unit/shape.cxx:413` | `mapped` | SmartArt text location; needs PDF text bounds. |
| `shape.cxx::testWriterFontwork` | `tdf125885_WordArt.docx` | `../core/oox/qa/unit/shape.cxx:487` | `mapped` | WordArt fill/color; needs PDF text/path color assertion. |
| `shape.cxx::testWriterFontwork3` | `tdf125885_WordArt3.docx` | `../core/oox/qa/unit/shape.cxx:610` | `mapped` | WordArt gradient; needs PDF path/color or raster assertion. |
| `shape.cxx::testWriterShapeFillNonAccentColor` | `tdf152840_theme_color_non_accent.docx` | `../core/oox/qa/unit/shape.cxx:736` | `mapped` | Theme fill color; needs PDF path/text color assertion. |
| `shape.cxx::testGlowOnGroup` | `tdf156902_GlowOnGroup.docx` | `../core/oox/qa/unit/shape.cxx:971` | `mapped` | Group glow effect; needs raster/snapshot assertion. |

## Broad-Scan Calibrated Candidates

These rows are included so the upstream evidence is not lost. Each row from the
expanded scan was item-reviewed against the local LibreOffice source. Rows with
source DOCX -> visible layout/metafile/bitmap evidence are `mapped`; rows that
depend on editing workflows, export XML, or non-source round-trip-only assertions
are `deferred`.

| Upstream test | Fixture | Source file | Status | Signal |
|---|---|---|---|---|
| `doc.cxx::testTextBoxMakeFlyFrame` | `textbox-makeflyframe.docx` | `../core/sw/qa/core/doc/doc.cxx:219` | `deferred` | Copy/paste workflow; not direct source DOCX -> PDF rendering. |
| `doc.cxx::testTextBoxWordWrap` | `text-box-word-wrap.docx` | `../core/sw/qa/core/doc/doc.cxx:730` | `mapped` | parseLayoutDump |
| `doc.cxx::testEditListAutofmt` | `edit-list-autofmt.docx` | `../core/sw/qa/core/doc/doc.cxx:785` | `deferred` | Applies an edit command before the layout assertion. |
| `number.cxx::testBadHeadingIndent` | `bad-heading-indent.docx` | `../core/sw/qa/core/doc/number.cxx:26` | `deferred` | Applies a style command before the layout assertion. |
| `HeaderFooterTest.cxx::testFirstPageHeadersAndEmptyFooters` | `fdo66145.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:242` | `covered` | parseLayoutDump |
| `HeaderFooterTest.cxx::testFirstHeaderFooterImport` | `first-header-footer.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:265` | `covered` | parseLayoutDump |
| `HeaderFooterTest.cxx::testContSectBreakHeaderFooter` | `cont-sect-break-header-footer.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:439` | `covered` | parseLayoutDump |
| `HeaderFooterTest.cxx::tdf166205_first_page_header_footer_visible` | `tdf166205_first_page_header_footer_visible.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:496` | `covered` | parseLayoutDump |
| `HeaderFooterTest.cxx::testFirstPageFooterEnabled` | `TestFirstFooterDisabled.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:646` | `mapped` | parseLayoutDump |
| `txtnode.cxx::testSplitFlyAnchorSplit` | `floattable-anchor-split.docx` | `../core/sw/qa/core/txtnode/txtnode.cxx:490` | `mapped` | parseLayoutDump |
| `unocore.cxx::testParagraphMarkerODFExport` | `paragraph-marker.docx` | `../core/sw/qa/core/unocore/unocore.cxx:851` | `deferred` | Assertion is after ODT save/reload, not source DOCX -> PDF rendering. |
| `unocore.cxx::testParagraphMarkerFormattedRun` | `paragraph-marker-formatted-run.docx` | `../core/sw/qa/core/unocore/unocore.cxx:869` | `deferred` | Assertion is after ODT save/reload, not source DOCX -> PDF rendering. |
| `odfexport3.cxx::testTdf129520` | `tdf129520.docx` | `../core/sw/qa/extras/odfexport/odfexport3.cxx:548` | `deferred` | ODT round-trip text/model assertion. |
| `odfexport4.cxx::tdf151100` | `tdf151100.docx` | `../core/sw/qa/extras/odfexport/odfexport4.cxx:165` | `deferred` | ODT export XML assertion. |
| `odfexport4.cxx::testTdf153090` | `Custom-Style-TOC.docx` | `../core/sw/qa/extras/odfexport/odfexport4.cxx:234` | `deferred` | TOC model/update assertion; broad scan signal belonged to the following test block. |
| `odfexport4.cxx::tdf120972` | `table_number_format_3.docx` | `../core/sw/qa/extras/odfexport/odfexport4.cxx:521` | `deferred` | Assertion is after ODT save/reload; not source DOCX -> PDF rendering. |
| `odfexport4.cxx::testTdf159382_DOCX` | `footnote_spacing_hanging_para.docx` | `../core/sw/qa/extras/odfexport/odfexport4.cxx:903` | `mapped` | parseLayoutDump |
| `odfimport.cxx::testTdf76322_columnBreakInHeader` | `tdf76322_columnBreakInHeader.docx` | `../core/sw/qa/extras/odfimport/odfimport.cxx:925` | `deferred` | Fixture is an ODF package despite the `.docx` suffix, so it is not a source DOCX -> PDF target for this suite. |
| `ooxmlexport.cxx::testfdo81031` | `fdo81031.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:71` | `mapped` | Bitmap |
| `ooxmlexport.cxx::testNumOverrideLvltext` | `num-override-lvltext.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:750` | `covered` | parseLayoutDump |
| `ooxmlexport.cxx::testTextboxRightEdge` | `textbox-right-edge.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:777` | `mapped` | parseLayoutDump |
| `ooxmlexport10.cxx::testWpgNested` | `wpg-nested.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:166` | `mapped` | parseLayoutDump |
| `ooxmlexport10.cxx::testPictureWithSchemeColor` | `picture-with-schemecolor.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:502` | `mapped` | Bitmap |
| `ooxmlexport10.cxx::testI124106` | `i124106.docx`; `large-twips.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:642` | `mapped` | parseLayoutDump |
| `ooxmlexport10.cxx::testNegativeCellMarginTwips` | `negative-cell-margin-twips.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:657` | `mapped` | parseLayoutDump |
| `ooxmlexport10.cxx::testFdo38414` | `fdo38414.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:666` | `mapped` | parseLayoutDump |
| `ooxmlexport10.cxx::testGridBefore` | `gridbefore.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:704` | `mapped` | parseLayoutDump |
| `ooxmlexport10.cxx::testMsoBrightnessContrast` | `msobrightnesscontrast.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:741` | `mapped` | Bitmap |
| `ooxmlexport11.cxx::testTdf113183` | `tdf113183.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:441` | `mapped` | parseLayoutDump |
| `ooxmlexport11.cxx::testTdf120511_eatenSection` | `tdf120511_eatenSection.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:565` | `mapped` | parseLayoutDump |
| `ooxmlexport11.cxx::testTdf119760_positionCellBorder` | `tdf119760_positionCellBorder.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:718` | `mapped` | parseLayoutDump |
| `ooxmlexport11.cxx::testTdf116985` | `tdf116985.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:746` | `mapped` | parseLayoutDump |
| `ooxmlexport12.cxx::testTd112202` | `090716_Studentische_Arbeit_VWS.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport12.cxx:674` | `mapped` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf123636_newlinePageBreak3` | `tdf123636_newlinePageBreak3.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:466` | `mapped` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf123636_newlinePageBreak4` | `tdf123636_newlinePageBreak4.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:477` | `mapped` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf169802_hidden_shape` | `tdf169802_hidden_shape.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:746` | `mapped` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf124594` | `tdf124594.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:773` | `mapped` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf125324` | `tdf125324.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:1056` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf128197` | `128197_compat14.docx`; `128197_compat15.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:69` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf135595_HFtableWrap` | `tdf135943_shapeWithText_LayoutInCell0_compat15.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:85` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf135595_HFtableWrap_c12` | `tdf135595_HFtableWrap_c12.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:131` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf151704_thinColumnHeight` | `tdf151704_thinColumnHeight.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:140` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf78749` | `tdf78749.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:163` | `mapped` | Bitmap |
| `ooxmlexport14.cxx::testTdf133000_numStyleFormatting` | `tdf133000_numStyleFormatting.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:338` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf78352` | `tdf78352.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:523` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf83309` | `tdf83309.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:708` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf163894` | `tdf163894.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:795` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf163894_hidden` | `tdf163894_hidden.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:823` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf32363` | `tdf32363.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:855` | `mapped` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf163894_from_top_to_beginning_of_the_documentMarguerite` | `tdf163894_from_top.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:892` | `mapped` | parseLayoutDump |
| `ooxmlexport15.cxx::testTdf131801` | `tdf131801.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:54` | `mapped` | parseLayoutDump |
| `ooxmlexport15.cxx::testTdf135949_anchoredBeforeBreak` | `tdf135949_anchoredBeforeBreak.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:763` | `mapped` | parseLayoutDump |
| `ooxmlexport15.cxx::testTdf134063` | `tdf134063.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:936` | `mapped` | parseLayoutDump |
| `ooxmlexport15.cxx::testRelativeAnchorHeightFromBottomMarginHasFooter` | `tdf133070_testRelativeAnchorHeightFromBottomMarginHasFooter.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:1028` | `mapped` | parseLayoutDump |
| `ooxmlexport15.cxx::testRelativeAnchorHeightFromBottomMarginNoFooter` | `tdf133070_testRelativeAnchorHeightFromBottomMarginNoFooter.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:1066` | `mapped` | parseLayoutDump |
| `ooxmlexport16.cxx::testTdf136841` | `tdf136841.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport16.cxx:783` | `mapped` | Bitmap |
| `ooxmlexport17.cxx::testTdf149313` | `tdf149313.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport17.cxx:1131` | `mapped` | parseLayoutDump |
| `ooxmlexport17.cxx::testTdf148360` | `tdf148360.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport17.cxx:1144` | `mapped` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf147646` | `tdf147646_mergedCellNumbering.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:114` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153042_largeTab` | `tdf153042_largeTab.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:131` | `mapped` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153042_noTab` | `tdf153042_noTab.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:146` | `mapped` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf105035_framePrB` | `tdf105035_framePrB.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:229` | `mapped` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf105035_framePrC` | `tdf105035_framePrC.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:246` | `mapped` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_anchoredAfterPgBreak` | `tdf153613_anchoredAfterPgBreak.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:340` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_anchoredAfterPgBreak2` | `tdf153613_anchoredAfterPgBreak2.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:347` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_anchoredAfterPgBreak3` | `tdf153613_anchoredAfterPgBreak3.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:354` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_anchoredAfterPgBreak6` | `tdf153613_anchoredAfterPgBreak6.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:361` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_inlineAfterPgBreak` | `tdf153613_inlineAfterPgBreak.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:373` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_inlineAfterPgBreak2` | `tdf153613_inlineAfterPgBreak2.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:380` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_textboxAfterPgBreak3` | `tdf153613_textboxAfterPgBreak3.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:397` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf147724` | `tdf147724.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:611` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153128` | `tdf153128.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:849` | `mapped` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf155736` | `tdf155736_PageNumbers_footer.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:924` | `covered` | parseLayoutDump |
| `ooxmlexport19.cxx::testBnc891663` | `bnc891663.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:154` | `mapped` | parseLayoutDump |
| `ooxmlexport19.cxx::testTdf95377` | `tdf95377.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:406` | `mapped` | parseLayoutDump |
| `ooxmlexport19.cxx::testTdf150408_isLvl_RoundTrip` | `listWithLgl.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:1118` | `mapped` | parseLayoutDump |
| `ooxmlexport2.cxx::testI120928` | `i120928.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport2.cxx:709` | `mapped` | Bitmap |
| `ooxmlexport20.cxx::testTdf128646` | `tdf128646.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport20.cxx:218` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf160077_layoutInCell` | `tdf160077_layoutInCell.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:540` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf160077_layoutInCellB` | `tdf160077_layoutInCellB.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:566` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf160077_layoutInCellC` | `tdf160077_layoutInCellC.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:599` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf160077_layoutInCellD` | `tdf160077_layoutInCellD.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:625` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf153909_followTextFlow` | `tdf153909_followTextFlow.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:686` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf162541` | `tdf162541_notLayoutInCell_paraLeft.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:709` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf162551` | `tdf162551_notLayoutInCell_charLeft_fromTop.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:736` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf126533_noPageBitmap` | `tdf126533_noPageBitmap.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:867` | `mapped` | Bitmap |
| `ooxmlexport21.cxx::testTdf126533_pageBitmap` | `tdf126533_pageBitmap.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:895` | `mapped` | Bitmap |
| `ooxmlexport21.cxx::testTdf154369` | `tdf154369.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:923` | `mapped` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf162746` | `tdf162746.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:1204` | `mapped` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf167770_marginInsideOutside` | `tdf167770_marginInsideOutside.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:82` | `mapped` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf165492_exactWithBottomSpacing` | `tdf165492_exactWithBottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:180` | `mapped` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf165492_atLeastWithBottomSpacing` | `tdf165492_atLeastWithBottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:202` | `mapped` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf165047_consolidatedTopMargin` | `tdf165047_consolidatedTopMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:224` | `mapped` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf165047_contextualSpacingTopMargin` | `tdf165047_contextualSpacingTopMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:243` | `mapped` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf139418` | `tdf139418.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:582` | `mapped` | parseLayoutDump |
| `ooxmlexport22.cxx::tdf167527_title_letters_cut_from_below` | `tdf167527_title_letters_cut_from_below.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:728` | `mapped` | dumpAndParse, MetafileXmlDump |
| `ooxmlexport23.cxx::testTdf146346` | `tdf146346.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:542` | `mapped` | parseLayoutDump |
| `ooxmlexport23.cxx::testTdf165354` | `tdf165354.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:692` | `mapped` | parseLayoutDump |
| `ooxmlexport23.cxx::testRelativeAnchorHeightFromTopMarginHasHeader` | `tdf123324_testRelativeAnchorHeightFromTopMarginHasHeader.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:837` | `mapped` | parseLayoutDump |
| `ooxmlexport23.cxx::testRelativeAnchorHeightFromTopMarginNoHeader` | `tdf123324_testRelativeAnchorHeightFromTopMarginNoHeader.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:849` | `mapped` | parseLayoutDump |
| `ooxmlexport23.cxx::testVmlShapeTextWordWrap` | `tdf97618_testVmlShapeTextWordWrap.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:876` | `mapped` | parseLayoutDump |
| `ooxmlexport24.cxx::testTdf37153` | `tdf37153_considerWrapOnObjPos.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport24.cxx:85` | `mapped` | parseLayoutDump |
| `ooxmlexport24.cxx::testTdf107889` | `tdf107889.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport24.cxx:435` | `mapped` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf166544_noTopMargin_fields` | `tdf166544_noTopMargin_fields.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:70` | `mapped` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf138020_all_rows_tblHeader` | `tdf138020_all_rows_tblHeader.docx`; `tdf167843_tblLook_firstRow_tblHeader.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:138` | `mapped` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf166510_sectPr_bottomSpacing` | `tdf166510_sectPr_bottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:231` | `mapped` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf169986_bottomSpacing` | `tdf169986_bottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:242` | `mapped` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf167657_sectPr_bottomSpacing` | `tdf167657_sectPr_bottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:268` | `mapped` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf165478_bottomAligned` | `tdf165478_bottomAligned.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:282` | `mapped` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf150822` | `tdf150822.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:653` | `mapped` | parseLayoutDump |
| `ooxmlexport26.cxx::testTdf64264` | `tdf64264.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:352` | `mapped` | parseLayoutDump |
| `ooxmlexport26.cxx::testTdf58944RepeatingTableHeader` | `tdf58944-repeating-table-header.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:368` | `mapped` | parseLayoutDump |
| `ooxmlexport26.cxx::testTdf81100` | `tdf81100.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:383` | `mapped` | parseLayoutDump |
| `ooxmlexport26.cxx::testTdf119952_negativeMargins` | `tdf119952_negativeMargins.docx`; `tdf143384_tableInFoot_negativeMargins.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:786` | `mapped` | parseLayoutDump |
| `ooxmlexport3.cxx::testNumberingLevels` | `tdf95495.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:1129` | `mapped` | parseLayoutDump |
| `ooxmlexport3.cxx::testRelativeAnchorWidthFromLeftMargin` | `tdf132976_testRelativeAnchorWidthFromLeftMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:1214` | `mapped` | parseLayoutDump |
| `ooxmlexport3.cxx::testRelativeAnchorWidthFromInsideOutsideMargin` | `tdf133861_RelativeAnchorWidthFromInsideOutsideMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:1224` | `mapped` | parseLayoutDump |
| `ooxmlexport4.cxx::testTextBoxPictureFill` | `textbox_picturefill.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:159` | `mapped` | Bitmap |
| `ooxmlexport4.cxx::testTestTitlePage` | `testTitlePage.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:242` | `covered` | parseLayoutDump |
| `ooxmlexport4.cxx::testTdf102466` | `tdf102466.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:1189` | `mapped` | parseLayoutDump |
| `ooxmlexport4.cxx::testTdf95367_inheritFollowStyle` | `tdf95367_inheritFollowStyle.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:1287` | `mapped` | parseLayoutDump |
| `ooxmlexport4.cxx::testInheritFirstHeader` | `inheritFirstHeader.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:1293` | `covered` | parseLayoutDump |
| `ooxmlexport4.cxx::testRelativeAnchorWidthFromRightMargin` | `tdf133670_testRelativeAnchorWidthFromRightMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:1402` | `mapped` | parseLayoutDump |
| `ooxmlexport6.cxx::testDMLShapeFillBitmapCrop` | `dml-shape-fillbitmapcrop.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport6.cxx:271` | `mapped` | Bitmap |
| `ooxmlexport6.cxx::testRelativeAlignmentFromTopMargin` | `tdf133045_TestShapeAlignmentRelativeFromTopMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport6.cxx:1101` | `mapped` | parseLayoutDump |
| `ooxmlexport7.cxx::testTDF87348` | `tdf87348_linkedTextboxes.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport7.cxx:1129` | `mapped` | parseLayoutDump |
| `ooxmlexport8.cxx::testN705956_1` | `n705956-1.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:113` | `mapped` | Bitmap |
| `ooxmlexport8.cxx::testN750255` | `n750255.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:184` | `covered` | parseLayoutDump |
| `ooxmlexport8.cxx::testN780843` | `n780843.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:537` | `covered` | parseLayoutDump |
| `ooxmlexport8.cxx::testN793998` | `n793998.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:694` | `mapped` | parseLayoutDump |
| `ooxmlexport9.cxx::testTdf84678` | `tdf84678.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport9.cxx:727` | `mapped` | parseLayoutDump |
| `ooxmlexport9.cxx::testTdf103544` | `tdf103544.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport9.cxx:736` | `mapped` | parseLayoutDump |
| `ooxmlexport_de_locale.cxx::testTdf160402` | `StyleRef-DE.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport_de_locale.cxx:31` | `mapped` | parseLayoutDump |
| `ooxmlexport_de_locale.cxx::testTdf166850` | `tdf166850.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport_de_locale.cxx:47` | `mapped` | parseLayoutDump |
| `ooxmlimport.cxx::testN751077` | `n751077.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:173` | `covered` | parseLayoutDump |
| `ooxmlimport.cxx::testTdf130804` | `tdf130804.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:372` | `mapped` | parseLayoutDump |
| `ooxmlimport.cxx::testN758883` | `n758883.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:387` | `mapped` | parseLayoutDump |
| `ooxmlimport.cxx::testN777345` | `n777345.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:523` | `mapped` | Bitmap |
| `ooxmlimport.cxx::testTdf105143` | `tdf105143.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:802` | `mapped` | parseLayoutDump |
| `ooxmlimport.cxx::testTdf75573` | `tdf75573_page1frame.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1032` | `mapped` | parseLayoutDump |
| `ooxmlimport.cxx::testFloatingTableSectionColumns` | `floating-table-section-columns.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1241` | `mapped` | parseLayoutDump |
| `ooxmlimport.cxx::testTdf60351` | `tdf60351.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1431` | `mapped` | parseLayoutDump |
| `ooxmlimport.cxx::testTdf98882` | `tdf98882.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1520` | `mapped` | parseLayoutDump |
| `ooxmlimport.cxx::testTdf106606` | `tdf106606.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1602` | `mapped` | Bitmap |
| `ooxmlimport.cxx::testTdf100072` | `tdf100072.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1649` | `mapped` | dumpAndParse, MetafileXmlDump, polyline |
| `ooxmlimport.cxx::testTdf136952_pgBreak3` | `tdf136952_pgBreak3.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1830` | `covered` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf114212` | `tdf114212.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:314` | `mapped` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf124600` | `tdf124600.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:453` | `mapped` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf120548` | `tdf120548.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:482` | `mapped` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf113946` | `tdf113946.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:702` | `mapped` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf156078` | `tdf156078_rightTabOutsideParaRightIndent.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:1233` | `mapped` | Bitmap |
| `rtfexport3.cxx::testTdf115180` | `tdf115180.docx` | `../core/sw/qa/extras/rtfexport/rtfexport3.cxx:290` | `mapped` | parseLayoutDump |
| `uiwriter10.cxx::testTdf145091` | `tdf145091.docx` | `../core/sw/qa/extras/uiwriter/uiwriter10.cxx:1504` | `deferred` | Redline reject/delete/export workflow, not direct source DOCX -> PDF rendering. |
| `uiwriter2.cxx::testTdfChangeNumberingListAutoFormat` | `tdf117923.docx` | `../core/sw/qa/extras/uiwriter/uiwriter2.cxx:93` | `mapped` | parseLayoutDump |
| `uiwriter3.cxx::testTdf147126` | `tdf147126.docx` | `../core/sw/qa/extras/uiwriter/uiwriter3.cxx:669` | `mapped` | parseLayoutDump |
| `uiwriter3.cxx::TestAsCharTextBox` | `AsCharTxBxTest.docx` | `../core/sw/qa/extras/uiwriter/uiwriter3.cxx:2121` | `deferred` | Keyboard navigation occurs before the layout assertion. |
| `uiwriter3.cxx::testTdf140975` | `tdf140975.docx` | `../core/sw/qa/extras/uiwriter/uiwriter3.cxx:2216` | `deferred` | Anchor-change command occurs before the layout assertion. |
| `uiwriter4.cxx::testTdf98987` | `tdf98987.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1078` | `mapped` | parseLayoutDump |
| `uiwriter4.cxx::testTdf99004` | `tdf99004.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1104` | `mapped` | parseLayoutDump |
| `uiwriter4.cxx::testTableRemoveHasTextChangesOnly` | `TC-table-del-add.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1713` | `deferred` | Track-changes edit workflow, not direct source DOCX -> PDF rendering. |
| `uiwriter4.cxx::testTableRemoveHasTextChangesOnly2` | `TC-table-del-add.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1779` | `deferred` | Track-changes edit workflow, not direct source DOCX -> PDF rendering. |
| `uiwriter4.cxx::testTdf147182_AcceptAllChangesInTableSelection` | `TC-table-del-add.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1833` | `deferred` | Accept-changes edit workflow, not direct source DOCX -> PDF rendering. |
| `uiwriter4.cxx::testTdf104492` | `tdf104492.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1954` | `mapped` | parseLayoutDump |
| `uiwriter6.cxx::testNestedGroupTextBoxCopyCrash` | `tdf149550.docx` | `../core/sw/qa/extras/uiwriter/uiwriter6.cxx:3098` | `deferred` | Copy/paste workflow; not direct source DOCX -> PDF rendering. |
| `uiwriter9.cxx::testSplitFloatingTable` | `floattable-split.docx` | `../core/sw/qa/extras/uiwriter/uiwriter9.cxx:273` | `mapped` | parseLayoutDump |
| `ww8export2.cxx::testTdf117503` | `tdf117503.docx` | `../core/sw/qa/extras/ww8export/ww8export2.cxx:1109` | `mapped` | parseLayoutDump |
| `ww8export4.cxx::testDOCExportDoNotMirrorRtlDrawObjs` | `draw-obj-rtl-no-mirror-vml.docx` | `../core/sw/qa/extras/ww8export/ww8export4.cxx:439` | `mapped` | parseLayoutDump |
| `ooxml.cxx::testFloattableMultiNested` | `floattable-multi-nested.docx` | `../core/sw/qa/writerfilter/ooxml/ooxml.cxx:175` | `mapped` | parseLayoutDump |

## Deferred Sources

| Source | Status | Reason |
|---|---|---|
| `../core/chart2/qa/extras/chart2import*.cxx` | `deferred` | Mostly chart model import assertions, not PDF rendering. Promote only individual tests with a concrete PDF-visible projection. |
| `../core/chart2/qa/extras/chart2export*.cxx` | `deferred` | Mostly exported chart XML or round-trip assertions. Prefer `sw/qa/extras/layout/layout5.cxx` chart render tests. |
| `../core/sw/qa/extras/ooxmlexport/ooxmlexport*.cxx` | `deferred` | Mostly DOCX export/round-trip assertions. Promote only if the assertion is visible layout/render evidence. |
| `../core/sw/qa/extras/ooxmlimport/ooxmlimport*.cxx` | `deferred` | Mostly DOCX import/model assertions. Promote only visible layout/render assertions. |
| `../core/oox/qa/unit/wpc_drawing_canvas.cxx` | `deferred` | WPC tests mostly assert UNO import properties, not final page render output. Promote individual tests only after a source-backed PDF text/path/raster assertion is defined. |
| `text.cxx::testTdf120715_CursorMoveWhenTypingSpaceAtCenteredLineEnd` and `text.cxx::testTdf43100_CursorMoveToSpacesOverMargin` | `deferred` | Cursor movement assertions are editor behavior, not PDF rendering calibration. |
| `uiwriter8.cxx::testTdf131684` and `uiwriter8.cxx::testTdf159026` | `deferred` | These are primarily UI edit/undo workflows; the layout checks are after commands, not source DOCX -> PDF rendering assertions. |
| `flycnt.cxx::testSplitFlyThenTable` | `deferred` | Upstream only verifies PDF export completion. Keep out of the active migration list until paired with a visible PDF assertion. |
| `../core/sw/qa/extras/globalfilter/globalfilter.cxx::testTdf143311` | `excluded` | Mixed DOCX/ODT/PDF-UA decorative-artifact test; belongs to tagged PDF/PDF-UA scope. |
