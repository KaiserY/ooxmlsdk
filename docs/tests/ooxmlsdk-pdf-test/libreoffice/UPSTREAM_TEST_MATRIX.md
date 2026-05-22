# LibreOffice DOCX/PPTX -> PDF Upstream Test Matrix

This matrix tracks LibreOffice upstream tests that are useful for calibrating
`ooxmlsdk-pdf` DOCX/PPTX -> PDF rendering. The source of truth is the local
LibreOffice checkout at `../core`.

This is intentionally not a DOCX/PPTX parser/import matrix. Pure UNO model checks,
OOXML export XML checks, round-trip checks, editing mechanics, and package/security
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
| Projection required | `mapped` rows | 0 | Active DOCX -> PDF TDD targets; add the required PDF projection or snapshot capability as needed. |
| Review only | `review` rows | 0 | Broad-scan rows have been item-reviewed in this pass. |
| Deferred/excluded | `deferred` / `excluded` rows | 36 | Keep out of the active PDF migration queue. |

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
| Layout dump projection | 103 | Convert LibreOffice layout dump frame/text/table/page assertions to PDF text/path/image bounds, object counts, text order, or page counts. | Do not copy XPath literally; preserve the upstream geometry/text relation after unit conversion and tolerances. |
| Raster/bitmap | 12 | Use PDFium page rendering plus `image`/`crc32fast`/PNG references for page or region snapshots. | Use LibreOffice pixel/color/bitmap expectation or checked-in reference images; do not derive expected images from current Rust output. |
| Metafile/render XML | 6 | Project LibreOffice metafile text/path/polyline assertions to PDF text/path objects or raster snapshots. | Preserve the asserted rendered primitive, not the LibreOffice XML format. |
| Graphics/color/effects | 12 | Assert PDF text/path/image colors, alpha, stroke/fill, bounds, or fall back to raster snapshots for effects. | Theme/color/effect expected values must remain LibreOffice-derived. |
| Other visible output | 34 | Mostly chart/layout/rendered-output cases; use text/path/image extraction first, snapshot when primitives are not stable enough. | Keep as active TDD targets when the final page output is the asserted behavior. |

## Scan Summary

- Local covered PDF-rendering fixtures: 306.
- Direct upstream DOCX -> PDF/object assertions: 10 rows, all covered.
- Planned direct PDF rows remaining: 0.
- Supplemental source-backed PDF-visible assertions already covered locally: 8
  rows.
- Additional visible-output candidates listed individually below: 0 mapped
  tests.
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
| `text.cxx::testNumberPortionNoformat` | `number-portion-noformat.docx` | `../core/sw/qa/core/text/text.cxx:1378` | `covered` | Numbering portion visibility; project to PDF text. |
| `text.cxx::testParaUpperMarginFlyIntersect` | `para-upper-margin-fly-intersect.docx` | `../core/sw/qa/core/text/text.cxx:1577` | `deferred` | Internal fly-portion height sum; no current faithful PDF text/path/image projection for the 521-twip layout-dump metric. |
| `calcmove.cxx::testIgnoreTopMargin` | `ignore-top-margin.docx` | `../core/sw/qa/core/layout/calcmove.cxx:29` | `covered` | Top margin layout; project to text/object bounds. |
| `calcmove.cxx::testIgnoreTopMarginTable` | `ignore-top-margin-table.docx` | `../core/sw/qa/core/layout/calcmove.cxx:46` | `covered` | Table top margin layout; project to PDF table/text bounds. |
| `calcmove.cxx::testIgnoreTopMarginPageStyleChange` | `ignore-top-margin-page-style-change.docx` | `../core/sw/qa/core/layout/calcmove.cxx:87` | `covered` | Page-style top margin layout; project to page/text bounds. |
| `ftnfrm.cxx::testInlineEndnotePosition` | `inline-endnote-position.docx` | `../core/sw/qa/core/layout/ftnfrm.cxx:121` | `covered` | Endnote position; project to page/text bounds. |
| `layout.cxx::testTableFlyOverlap` | `table-fly-overlap.docx` | `../core/sw/qa/core/layout/layout.cxx:52` | `covered` | Table/fly overlap; project to object bounds. |
| `layout.cxx::testTdf128195` | `tdf128195.docx` | `../core/sw/qa/core/layout/layout.cxx:73` | `covered` | Layout dump assertion; project to page/text/object bounds. |
| `layout.cxx::testBorderCollapseCompat` | `border-collapse-compat.docx` | `../core/sw/qa/core/layout/layout.cxx:91` | `covered` | Collapsed border rendering; project to PDF paths. |
| `layout.cxx::testTableFlyOverlapSpacing` | `table-fly-overlap-spacing.docx` | `../core/sw/qa/core/layout/layout.cxx:127` | `covered` | Table/fly spacing; project to object bounds. |
| `layout.cxx::testTextBoxAutoGrowVertical` | `textbox-autogrow-vertical.docx` | `../core/sw/qa/core/layout/layout.cxx:227` | `covered` | Textbox vertical growth; project to text and shape bounds. |
| `layout.cxx::testTextBoxInHeaderIsPositioned` | `header-textbox.docx` | `../core/sw/qa/core/layout/layout.cxx:258` | `covered` | Header textbox position; project to PDF text/shape bounds. |
| `layout.cxx::testVerticallyMergedCellBorder` | `vmerge-cell-border.docx` | `../core/sw/qa/core/layout/layout.cxx:464` | `covered` | Merged-cell border; project to PDF paths. |
| `layout.cxx::testInnerCellBorderIntersect` | `inner-border.docx` | `../core/sw/qa/core/layout/layout.cxx:562` | `covered` | Inner table borders; project to PDF paths. |
| `layout.cxx::testDoubleBorderVertical` | `double-border-vertical.docx` | `../core/sw/qa/core/layout/layout.cxx:677` | `covered` | Vertical double border; project to PDF paths. |
| `layout.cxx::testDoubleBorderHorizontal` | `double-border-horizontal.docx` | `../core/sw/qa/core/layout/layout.cxx:725` | `covered` | Horizontal double border; project to PDF paths. |
| `layout.cxx::testParaBorderInCellClip` | `para-border-in-cell-clip.docx` | `../core/sw/qa/core/layout/layout.cxx:773` | `covered` | Paragraph border clipping; project to PDF clipping/path checks. |
| `layout.cxx::testDoublePageBorder` | `double-page-border.docx` | `../core/sw/qa/core/layout/layout.cxx:793` | `covered` | Page border geometry; project to PDF paths. |
| `paintfrm.cxx::testRTLBorderMerge` | `rtl-table.docx` | `../core/sw/qa/core/layout/paintfrm.cxx:75` | `covered` | RTL table border merge; project to PDF paths. |
| `paintfrm.cxx::testInlineEndnoteSeparatorPosition` | `inline-endnote-position.docx` | `../core/sw/qa/core/layout/paintfrm.cxx:163` | `covered` | Endnote separator position; project to PDF paths. |
| `paintfrm.cxx::testEndnoteContSeparator` | `endnote-cont-separator.docx` | `../core/sw/qa/core/layout/paintfrm.cxx:193` | `covered` | Endnote continuation separator; project to PDF paths. |
| `paintfrm.cxx::testTableRedlineRenderMode` | `redline-table.docx` | `../core/sw/qa/core/layout/paintfrm.cxx:235` | `covered` | Default redline table row polygons; project to filled PDF paths. |
| `tabfrm.cxx::testTablePrintAreaLeft` | `table-print-area-left.docx` | `../core/sw/qa/core/layout/tabfrm.cxx:35` | `covered` | Table print-area position; project to table/text bounds. |
| `objectpositioning.cxx::testVertAlignBottomMarginWithFooter` | `bottom-margin-with-footer.docx` | `../core/sw/qa/core/objectpositioning/objectpositioning.cxx:184` | `deferred` | Test inserts the asserted rectangles through UNO after loading; source DOCX alone does not contain the objects to render. |
| `objectpositioning.cxx::testInsideOutsideVertAlignBottomMargin` | `inside-outside-vert-align.docx` | `../core/sw/qa/core/objectpositioning/objectpositioning.cxx:258` | `covered` | Inside/outside vertical alignment projected to PDF path bounds. |
| `objectpositioning.cxx::testVMLVertAlignBottomMargin` | `vml-vertical-alignment.docx` | `../core/sw/qa/core/objectpositioning/objectpositioning.cxx:279` | `covered` | VML vertical alignment projected to PDF path bounds. |
| `itrpaint.cxx::testRedlineRenderModeOmitInsertDelete` | `redline.docx` | `../core/sw/qa/core/text/itrpaint.cxx:64` | `covered` | Default redline text/color output. |
| `itrpaint.cxx::testMoveRedlineRenderModeOmitDelete` | `redline-move.docx` | `../core/sw/qa/core/text/itrpaint.cxx:156` | `deferred` | Requires non-default `SwRedlineRenderMode::OmitDeletes`; current PDF fixture lane covers the default redline render mode only. |
| `itrpaint.cxx::testAnchoredImageRedlineRenderModeOmitInsertDelete` | `redline-image-anchored.docx` | `../core/sw/qa/core/text/itrpaint.cxx:269` | `covered` | Default anchored image redline rendering; project to image count/color and frame paths. |
| `itrpaint.cxx::testInlineImageRedlineRenderModeOmitInsertDelete` | `redline-image-inline.docx` | `../core/sw/qa/core/text/itrpaint.cxx:325` | `covered` | Default inline image redline rendering; project to image count/color and frame paths. |
| `porfld.cxx::testNumberPortionRedlineRenderMode` | `redline-number-portion.docx` | `../core/sw/qa/core/text/porfld.cxx:31` | `covered` | Default inserted number underline; project to PDF text and decoration path. |
| `porfld.cxx::testTabPortionRedlineRenderMode` | `redline-bullet.docx` | `../core/sw/qa/core/text/porfld.cxx:70` | `covered` | Default deleted tab strikeout; project to PDF text and decoration path. |

## Writer Extras Layout Candidates

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `layout.cxx::TestTdf136588` | `tdf136588.docx` | `../core/sw/qa/extras/layout/layout.cxx:276` | `covered` | Layout dump; project to page/text/object bounds. |
| `layout.cxx::testTdf88496` | `tdf88496.docx` | `../core/sw/qa/extras/layout/layout.cxx:949` | `covered` | Layout dump; project to page/text/object bounds. |
| `layout.cxx::TestTdf137025` | `tdf137025.docx` | `../core/sw/qa/extras/layout/layout.cxx:1438` | `covered` | Detailed layout dump; project to PDF text/object bounds. |
| `layout.cxx::TestTdf134277` | `tdf134277.docx` | `../core/sw/qa/extras/layout/layout.cxx:2192` | `covered` | Layout regression; project to page/text bounds. |
| `layout.cxx::testTdf116486` | `tdf116486.docx` | `../core/sw/qa/extras/layout/layout.cxx:2206` | `covered` | Layout regression; project to visible page output. |
| `layout.cxx::TestTdf142080` | `fdo43573-2-min.docx` | `../core/sw/qa/extras/layout/layout.cxx:2217` | `covered` | Layout regression; project to page/text output. |
| `layout.cxx::testTdf128198` | `tdf128198-1.docx` | `../core/sw/qa/extras/layout/layout.cxx:2247` | `covered` | Layout regression; project to page/text output. |
| `layout.cxx::testTdf106153` | `tdf106153.docx` | `../core/sw/qa/extras/layout/layout.cxx:2284` | `covered` | Layout regression; project to page/text/object bounds. |
| `layout.cxx::testTdf109137` | `tdf109137.docx` | `../core/sw/qa/extras/layout/layout.cxx:3581` | `covered` | Layout regression; project to visible page output. |
| `layout.cxx::testTdf157628` | `tdf157628.docx` | `../core/sw/qa/extras/layout/layout.cxx:3701` | `covered` | Layout regression; project to visible page output. |
| `layout.cxx::testTdf125893` | `tdf125893.docx` | `../core/sw/qa/extras/layout/layout.cxx:3751` | `deferred` | Internal paragraph `prtBounds@top == 0` layout-dump metric; no faithful PDF projection identified yet. |
| `layout2.cxx::testTdf165322` | `CT-formatted-deletion.docx` | `../core/sw/qa/extras/layout/layout2.cxx:573` | `covered` | Formatted deletion visibility; project to PDF text/strikeout decoration. |
| `layout2.cxx::tdf157596_paragraph_numbering` | `tdf157596_paragraph_numbering.docx` | `../core/sw/qa/extras/layout/layout2.cxx:678` | `covered` | Paragraph numbering layout; project to PDF text. |
| `layout2.cxx::testTdf149711_importDOCXMoveToParagraphMark` | `tdf149711.docx` | `../core/sw/qa/extras/layout/layout2.cxx:787` | `deferred` | Rejects a tracked insertion before the final assertion; not direct source DOCX -> PDF rendering. |
| `layout2.cxx::testTdf152872` | `hidden-para-separator.docx` | `../core/sw/qa/extras/layout/layout2.cxx:805` | `covered` | Hidden paragraph separator layout; project to PDF text/spacing. |
| `layout2.cxx::testTdf151954` | `tdf151954.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1189` | `deferred` | Accepts tracked insertion before the assertion; not direct source DOCX -> PDF rendering. |
| `layout2.cxx::testRedlineMovingDOCX` | `tdf104797.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1501` | `covered` | Redline move rendering; project to PDF text color. |
| `layout2.cxx::testTdf125300` | `tdf125300.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1681` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf122225` | `tdf122225.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1756` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf134247` | `legend-itemorder-min.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1796` | `covered` | Chart legend order; project to PDF text order. |
| `layout2.cxx::testTdf75659` | `tdf75659.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1811` | `covered` | Chart/layout regression; project to PDF text/path output. |
| `layout2.cxx::testTdf126425` | `long_legendentry.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1843` | `covered` | Long legend entry layout; project to PDF text bounds. |
| `layout2.cxx::testUnusedOLEprops` | `tdf138465min.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1860` | `covered` | OLE formula height projects to PDF image/path height. |
| `layout2.cxx::testTdf115630` | `tdf115630.docx` | `../core/sw/qa/extras/layout/layout2.cxx:1920` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf128996` | `tdf128996.docx` | `../core/sw/qa/extras/layout/layout2.cxx:2009` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf126244` | `tdf126244.docx` | `../core/sw/qa/extras/layout/layout2.cxx:2023` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout2.cxx::testTdf69648` | `tdf69648.docx` | `../core/sw/qa/extras/layout/layout2.cxx:2080` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout2.cxx::testTdf116256` | `tdf116256.docx` | `../core/sw/qa/extras/layout/layout2.cxx:2118` | `covered` | Follow-text-flow textbox placement projected to PDF text/path containment. |
| `layout3.cxx::testTdf134463` | `tdf134463.docx` | `../core/sw/qa/extras/layout/layout3.cxx:31` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout3.cxx::testTdf117188` | `tdf117188.docx` | `../core/sw/qa/extras/layout/layout3.cxx:39` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout3.cxx::testTdf161718` | `tdf161718.docx` | `../core/sw/qa/extras/layout/layout3.cxx:222` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout3.cxx::testTdf119908` | `tdf130088.docx` | `../core/sw/qa/extras/layout/layout3.cxx:239` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout3.cxx::testTdf158333` | `tdf130088.docx` | `../core/sw/qa/extras/layout/layout3.cxx:257` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout3.cxx::testTdf158419` | `tdf130088.docx` | `../core/sw/qa/extras/layout/layout3.cxx:314` | `deferred` | Cursor hit-testing/content-index assertion after layout; editor model behavior, not PDF rendering calibration. |
| `layout3.cxx::testTdf164905` | `tdf164905.docx` | `../core/sw/qa/extras/layout/layout3.cxx:567` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout3.cxx::testTdf163149` | `tdf163149.docx` | `../core/sw/qa/extras/layout/layout3.cxx:583` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout3.cxx::testTdf164499` | `tdf164499.docx` | `../core/sw/qa/extras/layout/layout3.cxx:1295` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout4.cxx::testTdf117982` | `tdf117982.docx` | `../core/sw/qa/extras/layout/layout4.cxx:446` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf128959` | `tdf128959.docx` | `../core/sw/qa/extras/layout/layout4.cxx:458` | `covered` | Layout regression; project to PDF text/object bounds. |
| `layout4.cxx::testWriterImageNoCapture` | `writer-image-no-capture.docx` | `../core/sw/qa/extras/layout/layout4.cxx:555` | `covered` | Image capture/layout; project to PDF image/object checks. |
| `layout4.cxx::testTdf124423_DOCX` | `tdf124423.docx` | `../core/sw/qa/extras/layout/layout4.cxx:641` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf138782` | `tdf138782.docx` | `../core/sw/qa/extras/layout/layout4.cxx:706` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf135035_DOCX` | `tdf135035.docx` | `../core/sw/qa/extras/layout/layout4.cxx:722` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf139336_ColumnsWithFootnoteDoNotOccupyEntirePage` | `tdf139336_ColumnsWithFootnoteDoNotOccupyEntirePage.docx` | `../core/sw/qa/extras/layout/layout4.cxx:762` | `covered` | Columns and footnote layout; project to PDF text/page bounds. |
| `layout4.cxx::TestTdf161348` | `fdo48718-1.docx` | `../core/sw/qa/extras/layout/layout4.cxx:825` | `covered` | Layout regression; project to PDF page/text assertions. |
| `layout4.cxx::testTdf159271` | `fld-in-tbl.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1072` | `covered` | Field-in-table layout; project to PDF text/table bounds. |
| `layout4.cxx::testTdf159259` | `sdt+framePr.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1093` | `covered` | Content control with frame properties; project to PDF text/object bounds. |
| `layout4.cxx::TestTdf155229RowAtLeast` | `tdf155229_row_height_at_least.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1670` | `covered` | Row height layout; project to PDF table bounds. |
| `layout4.cxx::TestTdf164907_rowHeightAtLeast` | `tdf164907_rowHeightAtLeast.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1682` | `covered` | Row height layout; project to PDF table bounds. |
| `layout4.cxx::testTdf152298` | `tdf152298.docx` | `../core/sw/qa/extras/layout/layout4.cxx:1918` | `covered` | Layout regression; project to PDF page/text/object assertions. |

## Chart And Metafile Rendering Candidates

These tests are better PDF chart sources than `chart2` import/export tests:
LibreOffice asserts rendered metafile/text/path output instead of chart XML.

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `layout5.cxx::testTdf138194` | `xaxis-labelbreak.docx` | `../core/sw/qa/extras/layout/layout5.cxx:48` | `covered` | Chart axis label wrap; project to PDF text positions. |
| `layout5.cxx::testTdf138773` | `tdf138773.docx` | `../core/sw/qa/extras/layout/layout5.cxx:108` | `covered` | Chart rendered text/path output. |
| `layout5.cxx::testTdf130969` | `tdf130969.docx` | `../core/sw/qa/extras/layout/layout5.cxx:184` | `covered` | Chart rendered geometry. |
| `layout5.cxx::testTdf129054` | `tdf129054.docx` | `../core/sw/qa/extras/layout/layout5.cxx:217` | `covered` | Chart rendered geometry. |
| `layout5.cxx::testTdf129173` | `testAreaChartNumberFormat.docx` | `../core/sw/qa/extras/layout/layout5.cxx:243` | `covered` | Area-chart number-format text. |
| `layout5.cxx::testTdf134866` | `tdf134866.docx` | `../core/sw/qa/extras/layout/layout5.cxx:258` | `covered` | Chart label/shape output. |
| `layout5.cxx::testTdf137116` | `tdf137116.docx` | `../core/sw/qa/extras/layout/layout5.cxx:273` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf137154` | `tdf137154.docx` | `../core/sw/qa/extras/layout/layout5.cxx:293` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf138777` | `outside_long_data_label.docx` | `../core/sw/qa/extras/layout/layout5.cxx:313` | `covered` | Data label placement; project to PDF text bounds. |
| `layout5.cxx::testTdf130031` | `tdf130031.docx` | `../core/sw/qa/extras/layout/layout5.cxx:331` | `covered` | Chart line/path rendering. |
| `layout5.cxx::testTdf138018` | `tdf138018.docx` | `../core/sw/qa/extras/layout/layout5.cxx:392` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf130380` | `tdf130380.docx` | `../core/sw/qa/extras/layout/layout5.cxx:409` | `covered` | Chart geometry. |
| `layout5.cxx::testTdf129095` | `tdf129095.docx` | `../core/sw/qa/extras/layout/layout5.cxx:432` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf132956` | `tdf132956.docx` | `../core/sw/qa/extras/layout/layout5.cxx:447` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf122014` | `tdf122014.docx` | `../core/sw/qa/extras/layout/layout5.cxx:580` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf167202_footnote` | `tdf167202_footnote.docx` | `../core/sw/qa/extras/layout/layout5.cxx:597` | `covered` | Chart in footnote layout. |
| `layout5.cxx::testTdf134659` | `tdf134659.docx` | `../core/sw/qa/extras/layout/layout5.cxx:621` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf134235` | `tdf134235.docx` | `../core/sw/qa/extras/layout/layout5.cxx:638` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf134676` | `tdf134676.docx` | `../core/sw/qa/extras/layout/layout5.cxx:655` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf134146` | `tdf134146.docx` | `../core/sw/qa/extras/layout/layout5.cxx:672` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf136061` | `tdf136061.docx` | `../core/sw/qa/extras/layout/layout5.cxx:691` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf116925` | `tdf116925.docx` | `../core/sw/qa/extras/layout/layout5.cxx:705` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf117028` | `tdf117028.docx` | `../core/sw/qa/extras/layout/layout5.cxx:725` | `covered` | Chart rendered output. |
| `layout5.cxx::testTdf150200_DOCX` | `tdf150200.docx` | `../core/sw/qa/extras/layout/layout5.cxx:924` | `covered` | Chart/metafile text and path output. |
| `layout5.cxx::testTdf150438_DOCX` | `tdf150438.docx` | `../core/sw/qa/extras/layout/layout5.cxx:976` | `covered` | Chart/metafile text and path output. |
| `layout5.cxx::testTdf127606` | `tdf117923.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1027` | `covered` | Numbering portion text/font height projected to PDF text. |
| `layout5.cxx::testTdf127118` | `tdf127118.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1063` | `covered` | Rendered output regression. |
| `layout5.cxx::testTdf141220` | `tdf141220.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1072` | `covered` | Rendered output regression. |
| `layout5.cxx::testTdf134685` | `tdf134685.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1176` | `covered` | Rendered output regression. |
| `layout5.cxx::testTdf109077` | `tdf109077.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1186` | `covered` | Rendered output regression. |
| `layout5.cxx::testTdf164903` | `tdf164903.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1297` | `covered` | Rendered output regression. |
| `layout5.cxx::testTdf153136` | `tdf153136.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1361` | `covered` | Space-character line-height rules projected to PDF text bounds. |
| `layout5.cxx::testTdf167526` | `tdf167526.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1923` | `covered` | Detailed rendered text/line-number output. |
| `layout5.cxx::testTdf167540` | `tdf167540.docx` | `../core/sw/qa/extras/layout/layout5.cxx:1973` | `covered` | Detailed rendered text/line-number output. |

## Late Writer Layout Candidates

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `layout6.cxx::testTdf122878` | `tdf122878.docx` | `../core/sw/qa/extras/layout/layout6.cxx:550` | `covered` | Body/footer table non-overlap projected to PDF text and table paths. |
| `layout6.cxx::testTdf115094` | `tdf115094.docx` | `../core/sw/qa/extras/layout/layout6.cxx:572` | `covered` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf112290` | `tdf112290.docx` | `../core/sw/qa/extras/layout/layout6.cxx:599` | `covered` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf123651` | `tdf123651.docx` | `../core/sw/qa/extras/layout/layout6.cxx:918` | `covered` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf64222` | `tdf64222.docx` | `../core/sw/qa/extras/layout/layout6.cxx:1118` | `covered` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf124600` | `tdf124600-layout.docx` | `../core/sw/qa/extras/layout/layout6.cxx:1319` | `covered` | Single-line body paragraph projected to one PDF text segment. |
| `layout6.cxx::testTdf170381_split_float_table_in_normal_table` | `tdf170381-split-float-table-in-normal-table.docx` | `../core/sw/qa/extras/layout/layout6.cxx:1740` | `covered` | Split floating table layout; project to PDF table bounds and page flow. |
| `layout6.cxx::testTdf170381_split_float_table_in_float_table` | `tdf170381-split-float-table-in-float-table.docx` | `../core/sw/qa/extras/layout/layout6.cxx:1862` | `covered` | Nested split floating table layout; project to PDF table bounds and page flow. |
| `layout6.cxx::testTdf170620_float_table_after_keep_with_next_para` | `tdf170620.docx` | `../core/sw/qa/extras/layout/layout6.cxx:2018` | `covered` | Floating table after keep-with-next paragraph; project to PDF table/page flow. |
| `layout6.cxx::testTdf170630` | `tdf170630.docx` | `../core/sw/qa/extras/layout/layout6.cxx:2085` | `covered` | Layout regression; project to PDF page/text/table output. |
| `layout6.cxx::testTdf170846_1` | `tdf170846_1.docx` | `../core/sw/qa/extras/layout/layout6.cxx:2199` | `covered` | Layout regression; project to PDF page/text output. |
| `layout6.cxx::testTdf170846_2` | `tdf170846_2.docx` | `../core/sw/qa/extras/layout/layout6.cxx:2209` | `covered` | Layout regression; project to PDF page/text output. |

## Tiled Bitmap Rendering Candidates

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `tiledrendering.cxx::testHighlightNumbering` | `tdf114799_highlight.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:2609` | `covered` | Highlighted numbering pixels; project to PDF raster/color region. |
| `tiledrendering.cxx::testHighlightNumbering_shd` | `tdf114799_shd.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:2634` | `covered` | Numbering shading pixels; project to PDF raster/color region. |
| `tiledrendering.cxx::testTdf159626_yellowPatternFill` | `tdf159626_yellowPatternFill.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:3938` | `covered` | Yellow pattern fill pixels. |
| `tiledrendering.cxx::testTdf159626_yellowPatternFillB` | `tdf159626_yellowPatternFillB.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:3969` | `covered` | Yellow pattern fill pixels. |
| `tiledrendering.cxx::testTdf159626_blackPatternFill` | `tdf159626_blackPatternFill.docx` | `../core/sw/qa/extras/tiledrendering/tiledrendering.cxx:4000` | `covered` | Black pattern fill pixels. |

## OoXML And SVX Drawing Candidates

| Upstream test | Fixture | Source file | Status | PDF projection |
|---|---|---|---|---|
| `vml.cxx::tdf112450_vml_polyline` | `tdf112450_vml_polyline.docx` | `../core/oox/qa/unit/vml.cxx:37` | `covered` | VML polyline size geometry projected to PDF path bounds. |
| `vml.cxx::testWatermark` | `watermark.docx` | `../core/oox/qa/unit/vml.cxx:184` | `covered` | Picture watermark washout projected to grayscale PDF image rendering. |
| `helper.cxx::testImportTifCrop` | `tif-crop.docx` | `../core/oox/qa/unit/helper.cxx:30` | `deferred` | UNO `GraphicCrop` model rectangle; defer until crop-aware raster/PDF image projection can assert the crop values without inventing output data. |
| `customshapes.cxx::testTdf153000_MS0_SPT_25_31` | `tdf153000_WordArt_type_25_to_31.docx` | `../core/svx/qa/unit/customshapes.cxx:1377` | `covered` | WordArt preset segment complexity projected to PDF paths. |
| `drawingml.cxx::testCameraRotationRevolution` | `camera-rotation-revolution.docx` | `../core/oox/qa/unit/drawingml.cxx:251` | `covered` | Camera-rotated shapes projected to visible bounded PDF paths. |
| `drawingml.cxx::testToplevelLineHorOffsetDOCX` | `toplevel-line-hori-offset.docx` | `../core/oox/qa/unit/drawingml.cxx:726` | `covered` | Line horizontal offset projected to PDF vertical path bounds. |
| `drawingml.cxx::testDOCXVerticalLineRotation` | `line-vertical-rotation.docx` | `../core/oox/qa/unit/drawingml.cxx:776` | `covered` | Vertical line rotation projected to PDF vertical path geometry. |
| `shape.cxx::testMultipleGroupShapes` | `multiple-group-shapes.docx` | `../core/oox/qa/unit/shape.cxx:307` | `covered` | Separate group shapes projected to multiple PDF paths. |
| `shape.cxx::testCustomshapePosition` | `customshape-position.docx` | `../core/oox/qa/unit/shape.cxx:321` | `covered` | Custom shape vertical offset projected to PDF path bounds. |
| `shape.cxx::testTdf151518VertAnchor` | `tdf151518_SmartArtTextLocation.docx` | `../core/oox/qa/unit/shape.cxx:413` | `covered` | SmartArt text distances projected to PDF text inside shape paths. |
| `shape.cxx::testWriterFontwork` | `tdf125885_WordArt.docx` | `../core/oox/qa/unit/shape.cxx:487` | `covered` | Fontwork light-blue fill projected to PDF path color. |
| `shape.cxx::testWriterFontwork3` | `tdf125885_WordArt3.docx` | `../core/oox/qa/unit/shape.cxx:610` | `covered` | WordArt gradient colors projected to PDF path colors. |
| `shape.cxx::testWriterShapeFillNonAccentColor` | `tdf152840_theme_color_non_accent.docx` | `../core/oox/qa/unit/shape.cxx:736` | `covered` | Theme fill IDs bg2/bg1/tx1/tx2 projected to theme-derived PDF path fill colors. |
| `shape.cxx::testGlowOnGroup` | `tdf156902_GlowOnGroup.docx` | `../core/oox/qa/unit/shape.cxx:971` | `covered` | Group children projected to visible PDF path count. |

## Broad-Scan Calibrated Candidates

These rows are included so the upstream evidence is not lost. Each row from the
expanded scan was item-reviewed against the local LibreOffice source. Rows with
source DOCX -> visible layout/metafile/bitmap evidence are `mapped`; rows that
depend on editing workflows, export XML, or non-source round-trip-only assertions
are `deferred`.

| Upstream test | Fixture | Source file | Status | Signal |
|---|---|---|---|---|
| `doc.cxx::testTextBoxMakeFlyFrame` | `textbox-makeflyframe.docx` | `../core/sw/qa/core/doc/doc.cxx:219` | `deferred` | Copy/paste workflow; not direct source DOCX -> PDF rendering. |
| `doc.cxx::testTextBoxWordWrap` | `text-box-word-wrap.docx` | `../core/sw/qa/core/doc/doc.cxx:730` | `covered` | parseLayoutDump |
| `doc.cxx::testEditListAutofmt` | `edit-list-autofmt.docx` | `../core/sw/qa/core/doc/doc.cxx:785` | `deferred` | Applies an edit command before the layout assertion. |
| `number.cxx::testBadHeadingIndent` | `bad-heading-indent.docx` | `../core/sw/qa/core/doc/number.cxx:26` | `deferred` | Applies a style command before the layout assertion. |
| `HeaderFooterTest.cxx::testFirstPageHeadersAndEmptyFooters` | `fdo66145.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:242` | `covered` | parseLayoutDump |
| `HeaderFooterTest.cxx::testFirstHeaderFooterImport` | `first-header-footer.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:265` | `covered` | parseLayoutDump |
| `HeaderFooterTest.cxx::testContSectBreakHeaderFooter` | `cont-sect-break-header-footer.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:439` | `covered` | parseLayoutDump |
| `HeaderFooterTest.cxx::tdf166205_first_page_header_footer_visible` | `tdf166205_first_page_header_footer_visible.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:496` | `covered` | parseLayoutDump |
| `HeaderFooterTest.cxx::testFirstPageFooterEnabled` | `TestFirstFooterDisabled.docx` | `../core/sw/qa/core/header_footer/HeaderFooterTest.cxx:646` | `covered` | parseLayoutDump |
| `txtnode.cxx::testSplitFlyAnchorSplit` | `floattable-anchor-split.docx` | `../core/sw/qa/core/txtnode/txtnode.cxx:490` | `covered` | Initial split floating table pages projected to PDF page/path output. |
| `unocore.cxx::testParagraphMarkerODFExport` | `paragraph-marker.docx` | `../core/sw/qa/core/unocore/unocore.cxx:851` | `deferred` | Assertion is after ODT save/reload, not source DOCX -> PDF rendering. |
| `unocore.cxx::testParagraphMarkerFormattedRun` | `paragraph-marker-formatted-run.docx` | `../core/sw/qa/core/unocore/unocore.cxx:869` | `deferred` | Assertion is after ODT save/reload, not source DOCX -> PDF rendering. |
| `odfexport3.cxx::testTdf129520` | `tdf129520.docx` | `../core/sw/qa/extras/odfexport/odfexport3.cxx:548` | `deferred` | ODT round-trip text/model assertion. |
| `odfexport4.cxx::tdf151100` | `tdf151100.docx` | `../core/sw/qa/extras/odfexport/odfexport4.cxx:165` | `deferred` | ODT export XML assertion. |
| `odfexport4.cxx::testTdf153090` | `Custom-Style-TOC.docx` | `../core/sw/qa/extras/odfexport/odfexport4.cxx:234` | `deferred` | TOC model/update assertion; broad scan signal belonged to the following test block. |
| `odfexport4.cxx::tdf120972` | `table_number_format_3.docx` | `../core/sw/qa/extras/odfexport/odfexport4.cxx:521` | `deferred` | Assertion is after ODT save/reload; not source DOCX -> PDF rendering. |
| `odfexport4.cxx::testTdf159382_DOCX` | `footnote_spacing_hanging_para.docx` | `../core/sw/qa/extras/odfexport/odfexport4.cxx:903` | `covered` | Footnote number spacing projected to PDF text gap. |
| `odfimport.cxx::testTdf76322_columnBreakInHeader` | `tdf76322_columnBreakInHeader.docx` | `../core/sw/qa/extras/odfimport/odfimport.cxx:925` | `deferred` | Fixture is an ODF package despite the `.docx` suffix, so it is not a source DOCX -> PDF target for this suite. |
| `ooxmlexport.cxx::testfdo81031` | `fdo81031.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:71` | `covered` | Bitmap |
| `ooxmlexport.cxx::testNumOverrideLvltext` | `num-override-lvltext.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:750` | `covered` | parseLayoutDump |
| `ooxmlexport.cxx::testTextboxRightEdge` | `textbox-right-edge.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport.cxx:777` | `covered` | parseLayoutDump |
| `ooxmlexport10.cxx::testWpgNested` | `wpg-nested.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:166` | `covered` | parseLayoutDump |
| `ooxmlexport10.cxx::testPictureWithSchemeColor` | `picture-with-schemecolor.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:502` | `covered` | Bitmap |
| `ooxmlexport10.cxx::testI124106` | `i124106.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:642` | `covered` | Page count assertion projected to PDF page count. |
| `ooxmlexport10.cxx::testLargeTwips` | `large-twips.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:648` | `covered` | Positive cell text width projected to visible PDF text width. |
| `ooxmlexport10.cxx::testNegativeCellMarginTwips` | `negative-cell-margin-twips.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:657` | `covered` | parseLayoutDump |
| `ooxmlexport10.cxx::testFdo38414` | `fdo38414.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:666` | `covered` | Merged cell height equality projected to PDF rectangular path heights. |
| `ooxmlexport10.cxx::testGridBefore` | `gridbefore.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:704` | `covered` | Grid-before cell position projected to PDF text order and x-position. |
| `ooxmlexport10.cxx::testMsoBrightnessContrast` | `msobrightnesscontrast.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport10.cxx:741` | `covered` | Bitmap |
| `ooxmlexport11.cxx::testTdf113183` | `tdf113183.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:441` | `covered` | parseLayoutDump |
| `ooxmlexport11.cxx::testTdf120511_eatenSection` | `tdf120511_eatenSection.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:565` | `covered` | parseLayoutDump |
| `ooxmlexport11.cxx::testTdf119760_positionCellBorder` | `tdf119760_positionCellBorder.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:718` | `covered` | parseLayoutDump |
| `ooxmlexport11.cxx::testTdf116985` | `tdf116985.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport11.cxx:746` | `covered` | parseLayoutDump |
| `ooxmlexport12.cxx::testTd112202` | `090716_Studentische_Arbeit_VWS.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport12.cxx:674` | `covered` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf123636_newlinePageBreak3` | `tdf123636_newlinePageBreak3.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:466` | `covered` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf123636_newlinePageBreak4` | `tdf123636_newlinePageBreak4.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:477` | `covered` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf169802_hidden_shape` | `tdf169802_hidden_shape.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:746` | `covered` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf124594` | `tdf124594.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:773` | `covered` | parseLayoutDump |
| `ooxmlexport13.cxx::testTdf125324` | `tdf125324.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport13.cxx:1056` | `covered` | Floating table top position projected to PDF path bounds. |
| `ooxmlexport14.cxx::testTdf128197` | `128197_compat14.docx`; `128197_compat15.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:69` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf135943_shapeWithText_L0c15` | `tdf135943_shapeWithText_LayoutInCell0_compat15.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:108` | `covered` | Shape text stays inside frame boundaries; projected to PDF text inside shape path. |
| `ooxmlexport14.cxx::testTdf135595_HFtableWrap_c12` | `tdf135595_HFtableWrap_c12.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:131` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf151704_thinColumnHeight` | `tdf151704_thinColumnHeight.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:140` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf78749` | `tdf78749.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:163` | `covered` | Bitmap |
| `ooxmlexport14.cxx::testTdf133000_numStyleFormatting` | `tdf133000_numStyleFormatting.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:338` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf78352` | `tdf78352.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:523` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf83309` | `tdf83309.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:708` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf163894` | `tdf163894.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:795` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf163894_hidden` | `tdf163894_hidden.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:823` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf32363` | `tdf32363.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:855` | `covered` | parseLayoutDump |
| `ooxmlexport14.cxx::testTdf163894_from_top_to_beginning_of_the_documentMarguerite` | `tdf163894_from_top.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport14.cxx:892` | `covered` | parseLayoutDump |
| `ooxmlexport15.cxx::testTdf131801` | `tdf131801.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:54` | `covered` | parseLayoutDump |
| `ooxmlexport15.cxx::testTdf135949_anchoredBeforeBreak` | `tdf135949_anchoredBeforeBreak.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:763` | `covered` | parseLayoutDump |
| `ooxmlexport15.cxx::testTdf134063` | `tdf134063.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:936` | `covered` | parseLayoutDump |
| `ooxmlexport15.cxx::testRelativeAnchorHeightFromBottomMarginHasFooter` | `tdf133070_testRelativeAnchorHeightFromBottomMarginHasFooter.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:1028` | `covered` | parseLayoutDump |
| `ooxmlexport15.cxx::testRelativeAnchorHeightFromBottomMarginNoFooter` | `tdf133070_testRelativeAnchorHeightFromBottomMarginNoFooter.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport15.cxx:1066` | `covered` | Relative anchor height projected to PDF path height. |
| `ooxmlexport16.cxx::testTdf136841` | `tdf136841.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport16.cxx:783` | `covered` | Bitmap |
| `ooxmlexport17.cxx::testTdf149313` | `tdf149313.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport17.cxx:1131` | `covered` | parseLayoutDump |
| `ooxmlexport17.cxx::testTdf148360` | `tdf148360.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport17.cxx:1144` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf147646` | `tdf147646_mergedCellNumbering.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:114` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153042_largeTab` | `tdf153042_largeTab.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:131` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153042_noTab` | `tdf153042_noTab.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:146` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf105035_framePrB` | `tdf105035_framePrB.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:229` | `covered` | FramePr frames do not overlap; project to PDF frame bounds. |
| `ooxmlexport18.cxx::testTdf105035_framePrC` | `tdf105035_framePrC.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:246` | `covered` | FramePr frames overlap at same top; project to PDF frame bounds. |
| `ooxmlexport18.cxx::testTdf153613_anchoredAfterPgBreak` | `tdf153613_anchoredAfterPgBreak.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:340` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_anchoredAfterPgBreak2` | `tdf153613_anchoredAfterPgBreak2.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:347` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_anchoredAfterPgBreak3` | `tdf153613_anchoredAfterPgBreak3.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:354` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_anchoredAfterPgBreak6` | `tdf153613_anchoredAfterPgBreak6.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:361` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_inlineAfterPgBreak` | `tdf153613_inlineAfterPgBreak.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:373` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_inlineAfterPgBreak2` | `tdf153613_inlineAfterPgBreak2.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:380` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153613_textboxAfterPgBreak3` | `tdf153613_textboxAfterPgBreak3.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:397` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf147724` | `tdf147724.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:611` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf153128` | `tdf153128.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:849` | `covered` | parseLayoutDump |
| `ooxmlexport18.cxx::testTdf155736` | `tdf155736_PageNumbers_footer.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport18.cxx:924` | `covered` | parseLayoutDump |
| `ooxmlexport19.cxx::testBnc891663` | `bnc891663.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:154` | `covered` | parseLayoutDump |
| `ooxmlexport19.cxx::testTdf95377` | `tdf95377.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:406` | `covered` | parseLayoutDump |
| `ooxmlexport19.cxx::testTdf150408_isLvl_RoundTrip` | `listWithLgl.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport19.cxx:1118` | `covered` | parseLayoutDump |
| `ooxmlexport2.cxx::testI120928` | `i120928.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport2.cxx:709` | `covered` | Graphic numbering bullet projects to PDF image output. |
| `ooxmlexport20.cxx::testTdf128646` | `tdf128646.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport20.cxx:218` | `covered` | Layout-in-cell shape visibility projected to PDF path geometry. |
| `ooxmlexport21.cxx::testTdf160077_layoutInCell` | `tdf160077_layoutInCell.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:540` | `covered` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf160077_layoutInCellB` | `tdf160077_layoutInCellB.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:566` | `covered` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf160077_layoutInCellC` | `tdf160077_layoutInCellC.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:599` | `covered` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf160077_layoutInCellD` | `tdf160077_layoutInCellD.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:625` | `covered` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf153909_followTextFlow` | `tdf153909_followTextFlow.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:686` | `covered` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf162541` | `tdf162541_notLayoutInCell_paraLeft.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:709` | `covered` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf162551` | `tdf162551_notLayoutInCell_charLeft_fromTop.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:736` | `covered` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf126533_noPageBitmap` | `tdf126533_noPageBitmap.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:867` | `covered` | Ignored background bitmap projects to no PDF images. |
| `ooxmlexport21.cxx::testTdf126533_pageBitmap` | `tdf126533_pageBitmap.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:895` | `covered` | Page bitmap background projects to PDF image output. |
| `ooxmlexport21.cxx::testTdf154369` | `tdf154369.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:923` | `covered` | parseLayoutDump |
| `ooxmlexport21.cxx::testTdf162746` | `tdf162746.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport21.cxx:1204` | `covered` | Body table width below header float projected to PDF path width. |
| `ooxmlexport22.cxx::testTdf167770_marginInsideOutside` | `tdf167770_marginInsideOutside.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:82` | `covered` | Inside/outside horizontal alignment projected to PDF object bounds. |
| `ooxmlexport22.cxx::testTdf165492_exactWithBottomSpacing` | `tdf165492_exactWithBottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:180` | `covered` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf165492_atLeastWithBottomSpacing` | `tdf165492_atLeastWithBottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:202` | `covered` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf165047_consolidatedTopMargin` | `tdf165047_consolidatedTopMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:224` | `covered` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf165047_contextualSpacingTopMargin` | `tdf165047_contextualSpacingTopMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:243` | `covered` | parseLayoutDump |
| `ooxmlexport22.cxx::testTdf139418` | `tdf139418.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:582` | `covered` | Vertical DOCX grid text portions projected to PDF vertical text bounds. |
| `ooxmlexport22.cxx::tdf167527_title_letters_cut_from_below` | `tdf167527_title_letters_cut_from_below.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport22.cxx:728` | `covered` | Field shading height projected to PDF path height. |
| `ooxmlexport23.cxx::testTdf146346` | `tdf146346.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:542` | `covered` | parseLayoutDump |
| `ooxmlexport23.cxx::testTdf165354` | `tdf165354.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:692` | `covered` | parseLayoutDump |
| `ooxmlexport23.cxx::testRelativeAnchorHeightFromTopMarginHasHeader` | `tdf123324_testRelativeAnchorHeightFromTopMarginHasHeader.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:837` | `covered` | parseLayoutDump |
| `ooxmlexport23.cxx::testRelativeAnchorHeightFromTopMarginNoHeader` | `tdf123324_testRelativeAnchorHeightFromTopMarginNoHeader.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:849` | `covered` | parseLayoutDump |
| `ooxmlexport23.cxx::testVmlShapeTextWordWrap` | `tdf97618_testVmlShapeTextWordWrap.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport23.cxx:876` | `covered` | VML shape wrap width projected to PDF path width. |
| `ooxmlexport24.cxx::testTdf37153` | `tdf37153_considerWrapOnObjPos.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport24.cxx:85` | `covered` | Layout-in-cell text position; project to PDF text bounds. |
| `ooxmlexport24.cxx::testTdf107889` | `tdf107889.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport24.cxx:435` | `covered` | Split multipage table projected to PDF page count. |
| `ooxmlexport25.cxx::testTdf166544_noTopMargin_fields` | `tdf166544_noTopMargin_fields.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:70` | `covered` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf138020_all_rows_tblHeader` | `tdf138020_all_rows_tblHeader.docx`; `tdf167843_tblLook_firstRow_tblHeader.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:138` | `covered` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf166510_sectPr_bottomSpacing` | `tdf166510_sectPr_bottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:231` | `covered` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf169986_bottomSpacing` | `tdf169986_bottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:242` | `covered` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf167657_sectPr_bottomSpacing` | `tdf167657_sectPr_bottomSpacing.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:268` | `covered` | parseLayoutDump |
| `ooxmlexport25.cxx::testTdf165478_bottomAligned` | `tdf165478_bottomAligned.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:282` | `covered` | Bottom-aligned cell text and image position projected to PDF bounds. |
| `ooxmlexport25.cxx::testTdf150822` | `tdf150822.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport25.cxx:653` | `covered` | Vertical writing layout; project to PDF text orientation. |
| `ooxmlexport26.cxx::testTdf64264` | `tdf64264.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:352` | `covered` | Repeating table header flow; project to PDF page/text order. |
| `ooxmlexport26.cxx::testTdf58944RepeatingTableHeader` | `tdf58944-repeating-table-header.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:368` | `covered` | Repeating table header flow; project to PDF page/text order. |
| `ooxmlexport26.cxx::testTdf81100` | `tdf81100.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:383` | `covered` | Explicit no-repeat table header flow; project to PDF page/text output. |
| `ooxmlexport26.cxx::testTdf119952_negativeMargins` | `tdf119952_negativeMargins.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport26.cxx:786` | `covered` | Negative-margin header/footer fly text projected to PDF page text. |
| `ooxmlexport3.cxx::testNumberingLevels` | `tdf95495.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:1129` | `covered` | parseLayoutDump |
| `ooxmlexport3.cxx::testRelativeAnchorWidthFromLeftMargin` | `tdf132976_testRelativeAnchorWidthFromLeftMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:1214` | `covered` | parseLayoutDump |
| `ooxmlexport3.cxx::testRelativeAnchorWidthFromInsideOutsideMargin` | `tdf133861_RelativeAnchorWidthFromInsideOutsideMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport3.cxx:1224` | `covered` | parseLayoutDump |
| `ooxmlexport4.cxx::testTextBoxPictureFill` | `textbox_picturefill.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:159` | `covered` | Bitmap |
| `ooxmlexport4.cxx::testTestTitlePage` | `testTitlePage.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:242` | `covered` | parseLayoutDump |
| `ooxmlexport4.cxx::testTdf102466` | `tdf102466.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:1189` | `covered` | parseLayoutDump |
| `ooxmlexport4.cxx::testTdf95367_inheritFollowStyle` | `tdf95367_inheritFollowStyle.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:1287` | `covered` | parseLayoutDump |
| `ooxmlexport4.cxx::testInheritFirstHeader` | `inheritFirstHeader.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:1293` | `covered` | parseLayoutDump |
| `ooxmlexport4.cxx::testRelativeAnchorWidthFromRightMargin` | `tdf133670_testRelativeAnchorWidthFromRightMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport4.cxx:1402` | `covered` | Relative anchor width projected to PDF path width. |
| `ooxmlexport6.cxx::testDMLShapeFillBitmapCrop` | `dml-shape-fillbitmapcrop.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport6.cxx:271` | `covered` | Picture-filled cropped shapes project to PDF image output. |
| `ooxmlexport6.cxx::testRelativeAlignmentFromTopMargin` | `tdf133045_TestShapeAlignmentRelativeFromTopMargin.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport6.cxx:1101` | `covered` | parseLayoutDump |
| `ooxmlexport7.cxx::testTDF87348` | `tdf87348_linkedTextboxes.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport7.cxx:1129` | `covered` | Linked textbox chain members projected to PDF path count. |
| `ooxmlexport8.cxx::testN705956_1` | `n705956-1.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:113` | `covered` | Bitmap |
| `ooxmlexport8.cxx::testN750255` | `n750255.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:184` | `covered` | parseLayoutDump |
| `ooxmlexport8.cxx::testN780843` | `n780843.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:537` | `covered` | parseLayoutDump |
| `ooxmlexport8.cxx::testN793998` | `n793998.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport8.cxx:694` | `covered` | Tab portion past right margin projected to PDF horizontal gap. |
| `ooxmlexport9.cxx::testTdf84678` | `tdf84678.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport9.cxx:727` | `covered` | parseLayoutDump |
| `ooxmlexport9.cxx::testTdf103544` | `tdf103544.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport9.cxx:736` | `covered` | parseLayoutDump |
| `ooxmlexport_de_locale.cxx::testTdf160402` | `StyleRef-DE.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport_de_locale.cxx:31` | `covered` | parseLayoutDump |
| `ooxmlexport_de_locale.cxx::testTdf166850` | `tdf166850.docx` | `../core/sw/qa/extras/ooxmlexport/ooxmlexport_de_locale.cxx:47` | `covered` | StyleRef header expansion projected to PDF page text. |
| `ooxmlimport.cxx::testN751077` | `n751077.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:173` | `covered` | parseLayoutDump |
| `ooxmlimport.cxx::testTdf130804` | `tdf130804.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:372` | `covered` | Fly/text height and following paragraph spacing; project to PDF text/path bounds. |
| `ooxmlimport.cxx::testN758883` | `n758883.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:387` | `covered` | parseLayoutDump |
| `ooxmlimport.cxx::testN777345` | `n777345.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:523` | `covered` | Bitmap |
| `ooxmlimport.cxx::testTdf105143` | `tdf105143.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:802` | `covered` | Shape vertical position; project to PDF path bounds. |
| `ooxmlimport.cxx::testTdf75573` | `tdf75573_page1frame.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1032` | `covered` | parseLayoutDump |
| `ooxmlimport.cxx::testFloatingTableSectionColumns` | `floating-table-section-columns.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1241` | `covered` | Floating table width across section columns; project to PDF path width. |
| `ooxmlimport.cxx::testTdf60351` | `tdf60351.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1431` | `covered` | Contour polygon geometry; project to PDF path segments. |
| `ooxmlimport.cxx::testTdf98882` | `tdf98882.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1520` | `covered` | Image content/frame height; project to PDF image/path bounds. |
| `ooxmlimport.cxx::testTdf106606` | `tdf106606.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1602` | `covered` | Two graphic numbering styles projected to PDF image output. |
| `ooxmlimport.cxx::testTdf100072` | `tdf100072.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1649` | `covered` | Shape visibility and page bounds; project to PDF paths. |
| `ooxmlimport.cxx::testTdf136952_pgBreak3` | `tdf136952_pgBreak3.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport.cxx:1830` | `covered` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf114212` | `tdf114212.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:314` | `covered` | First fly top position; project to PDF path bounds. |
| `ooxmlimport2.cxx::testTdf124600` | `tdf124600.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:453` | `covered` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf120548` | `tdf120548.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:482` | `covered` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf113946` | `tdf113946.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:702` | `covered` | parseLayoutDump |
| `ooxmlimport2.cxx::testTdf156078` | `tdf156078_rightTabOutsideParaRightIndent.docx` | `../core/sw/qa/extras/ooxmlimport/ooxmlimport2.cxx:1233` | `covered` | Bitmap |
| `rtfexport3.cxx::testTdf115180` | `tdf115180.docx` | `../core/sw/qa/extras/rtfexport/rtfexport3.cxx:290` | `covered` | Table and cell widths projected to PDF path widths. |
| `uiwriter10.cxx::testTdf145091` | `tdf145091.docx` | `../core/sw/qa/extras/uiwriter/uiwriter10.cxx:1504` | `deferred` | Redline reject/delete/export workflow, not direct source DOCX -> PDF rendering. |
| `uiwriter2.cxx::testTdfChangeNumberingListAutoFormat` | `tdf117923.docx` | `../core/sw/qa/extras/uiwriter/uiwriter2.cxx:93` | `covered` | parseLayoutDump |
| `uiwriter3.cxx::testTdf147126` | `tdf147126.docx` | `../core/sw/qa/extras/uiwriter/uiwriter3.cxx:669` | `covered` | Group draw object containment projected to visible PDF path count. |
| `uiwriter3.cxx::TestAsCharTextBox` | `AsCharTxBxTest.docx` | `../core/sw/qa/extras/uiwriter/uiwriter3.cxx:2121` | `deferred` | Keyboard navigation occurs before the layout assertion. |
| `uiwriter3.cxx::testTdf140975` | `tdf140975.docx` | `../core/sw/qa/extras/uiwriter/uiwriter3.cxx:2216` | `deferred` | Anchor-change command occurs before the layout assertion. |
| `uiwriter4.cxx::testTdf98987` | `tdf98987.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1078` | `covered` | Rectangle vertical ordering projected to PDF path bounds. |
| `uiwriter4.cxx::testTdf99004` | `tdf99004.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1104` | `covered` | Textbox/rectangle non-overlap projected to PDF rectangular path bounds. |
| `uiwriter4.cxx::testTableRemoveHasTextChangesOnly` | `TC-table-del-add.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1713` | `deferred` | Track-changes edit workflow, not direct source DOCX -> PDF rendering. |
| `uiwriter4.cxx::testTableRemoveHasTextChangesOnly2` | `TC-table-del-add.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1779` | `deferred` | Track-changes edit workflow, not direct source DOCX -> PDF rendering. |
| `uiwriter4.cxx::testTdf147182_AcceptAllChangesInTableSelection` | `TC-table-del-add.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1833` | `deferred` | Accept-changes edit workflow, not direct source DOCX -> PDF rendering. |
| `uiwriter4.cxx::testTdf104492` | `tdf104492.docx` | `../core/sw/qa/extras/uiwriter/uiwriter4.cxx:1954` | `covered` | parseLayoutDump |
| `uiwriter6.cxx::testNestedGroupTextBoxCopyCrash` | `tdf149550.docx` | `../core/sw/qa/extras/uiwriter/uiwriter6.cxx:3098` | `deferred` | Copy/paste workflow; not direct source DOCX -> PDF rendering. |
| `uiwriter9.cxx::testSplitFloatingTable` | `floattable-split.docx` | `../core/sw/qa/extras/uiwriter/uiwriter9.cxx:273` | `covered` | Split floating table projected to two-page PDF table paths. |
| `ww8export2.cxx::testTdf117503` | `tdf117503.docx` | `../core/sw/qa/extras/ww8export/ww8export2.cxx:1109` | `deferred` | Assertion is after DOC export/reload, not direct source DOCX -> PDF rendering. |
| `ww8export4.cxx::testDOCExportDoNotMirrorRtlDrawObjs` | `draw-obj-rtl-no-mirror-vml.docx` | `../core/sw/qa/extras/ww8export/ww8export4.cxx:439` | `deferred` | Assertion is after DOC export/reload, not direct source DOCX -> PDF rendering. |
| `ooxml.cxx::testFloattableMultiNested` | `floattable-multi-nested.docx` | `../core/sw/qa/writerfilter/ooxml/ooxml.cxx:175` | `covered` | parseLayoutDump |

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

## PPTX Migration Scope

The PPTX migration target is 221 Rust `#[test]` functions, not 221 full
PDF-export snapshots and not a fixture-only queue. Each target maps to one
LibreOffice upstream PPTX test function below. The Rust test must carry a
`// Source: ../core/sd/qa/unit/...` comment and must copy its expected values
from the cited LibreOffice assertions, LibreOffice layout/metafile/PNG evidence,
or checked-in LibreOffice reference output. Do not derive expected values from
current `ooxmlsdk-pdf` output.

The DOCX lesson applies here: most upstream PPTX tests are partial assertions
about visible slide output after import. They should become small PDF primitive,
text, path, color, bounds, font-resource, or raster-region assertions. Only about
20 tests should be full PDF/reference tests. In the current upstream set this
breaks down as:

| Rust test style | Count | Rule |
|---|---:|---|
| Full PDF/reference-style tests | 17 | Whole-slide render references from `testDocumentLayout()`, full-page PNG export checks, embedded-font PDF-resource checks, and theme render checks. |
| Partial assertion tests | 204 | Port the specific LibreOffice assertion to a PDF-visible text/path/image/color/bounds/raster assertion. |
| Total Rust `#[test]` targets | 221 | One Rust test per upstream row below. |

PPTX bucket counts:

| Source bucket | Rust `#[test]` targets |
|---|---:|
| `sd/qa/unit/import-tests.cxx` | 45 |
| `sd/qa/unit/import-tests2.cxx` | 35 |
| `sd/qa/unit/import-tests3.cxx` | 33 |
| `sd/qa/unit/import-tests4.cxx` | 37 |
| `sd/qa/unit/import-tests_skia.cxx` | 1 |
| `sd/qa/unit/import-tests-smartart.cxx` | 49 |
| `sd/qa/unit/layout-tests.cxx` | 12 |
| `sd/qa/unit/PNGExportTests.cxx` | 4 |
| `sd/qa/unit/ShapeImportExportTest.cxx` | 2 |
| `sd/qa/unit/ThemeTest.cxx` | 1 |
| `sd/qa/unit/FontEmbeddingTest.cxx` | 2 |

Current local coverage is intentionally small and upstream-assertion focused:
32 matrix rows are covered by active PPTX PDF tests. They currently span
LibreOffice import text/color/fill/line/border/page assertions and layout
metafile-position assertions.
`layout-tests.cxx::numberedList` is present as an ignored TDD assertion because
the current PPTX renderer still emits master placeholder text. Keep the
LibreOffice text-order expectation unchanged.

### PPTX Upstream Test Matrix

| Upstream test | Fixture | Upstream source | Status | Rust assertion shape |
|---|---|---|---|---|
| `import-tests.cxx::testFdo47434` | `fdo47434.pptx` | `../core/sd/qa/unit/import-tests.cxx:140` | `mapped` | full render reference. |
| `import-tests.cxx::testN819614` | `n819614.pptx` | `../core/sd/qa/unit/import-tests.cxx:155` | `mapped` | full render reference. |
| `import-tests.cxx::testN820786` | `n820786.pptx` | `../core/sd/qa/unit/import-tests.cxx:163` | `mapped` | full render reference. |
| `import-tests.cxx::testN762695` | `n762695.pptx` | `../core/sd/qa/unit/import-tests.cxx:168` | `mapped` | full render reference. |
| `import-tests.cxx::testN593612` | `n593612.pptx` | `../core/sd/qa/unit/import-tests.cxx:173` | `mapped` | full render reference. |
| `import-tests.cxx::testFdo71434` | `fdo71434.pptx` | `../core/sd/qa/unit/import-tests.cxx:178` | `mapped` | full render reference. |
| `import-tests.cxx::testN902652` | `n902652.pptx` | `../core/sd/qa/unit/import-tests.cxx:183` | `mapped` | full render reference. |
| `import-tests.cxx::testTdf90403` | `tdf90403.pptx` | `../core/sd/qa/unit/import-tests.cxx:188` | `mapped` | full render reference. |
| `import-tests.cxx::testTdf100491` | `tdf100491.pptx` | `../core/sd/qa/unit/import-tests.cxx:203` | `mapped` | full render reference. |
| `import-tests.cxx::testTdf109317` | `tdf109317.pptx` | `../core/sd/qa/unit/import-tests.cxx:208` | `mapped` | full render reference. |
| `import-tests.cxx::testTdf157216` | `pptx/tdf157216.pptx` | `../core/sd/qa/unit/import-tests.cxx:213` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTableStyle` | `pptx/tdf156718.pptx` | `../core/sd/qa/unit/import-tests.cxx:242` | `covered` | partial visible-output assertion. |
| `import-tests.cxx::testFreeformShapeGluePoints` | `pptx/tdf156829.pptx` | `../core/sd/qa/unit/import-tests.cxx:311` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf154363` | `pptx/tdf154363.pptx` | `../core/sd/qa/unit/import-tests.cxx:339` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf154858` | `pptx/tdf154858.pptx` | `../core/sd/qa/unit/import-tests.cxx:363` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf153466` | `pptx/tdf153466.pptx` | `../core/sd/qa/unit/import-tests.cxx:372` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf152434` | `pptx/tdf152434.pptx` | `../core/sd/qa/unit/import-tests.cxx:391` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testConnectors` | `pptx/connectors.pptx` | `../core/sd/qa/unit/import-tests.cxx:402` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf153036_resizedConnectorL` | `pptx/tdf153036_resizedConnectorL.pptx` | `../core/sd/qa/unit/import-tests.cxx:426` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf150719` | `pptx/tdf150719.pptx` | `../core/sd/qa/unit/import-tests.cxx:440` | `covered` | partial visible-output assertion. |
| `import-tests.cxx::testTdf149314` | `pptx/tdf149314.pptx` | `../core/sd/qa/unit/import-tests.cxx:453` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf149124` | `pptx/tdf149124.pptx` | `../core/sd/qa/unit/import-tests.cxx:479` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf148965` | `pptx/tdf148965.pptx` | `../core/sd/qa/unit/import-tests.cxx:496` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf89449` | `pptx/tdf89449.pptx` | `../core/sd/qa/unit/import-tests.cxx:529` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testGluePointLeavingDirections` | `pptx/glue_point_leaving_directions.pptx` | `../core/sd/qa/unit/import-tests.cxx:564` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf147459` | `pptx/tdf147459.pptx` | `../core/sd/qa/unit/import-tests.cxx:592` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf146223` | `pptx/tdf146223.pptx` | `../core/sd/qa/unit/import-tests.cxx:621` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf144918` | `pptx/tdf144918.pptx` | `../core/sd/qa/unit/import-tests.cxx:635` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf144917` | `pptx/tdf144917.pptx` | `../core/sd/qa/unit/import-tests.cxx:658` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testHyperlinkOnImage` | `pptx/hyperlinkOnImage.pptx` | `../core/sd/qa/unit/import-tests.cxx:674` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf142645` | `pptx/tdf142645.pptx` | `../core/sd/qa/unit/import-tests.cxx:699` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf141704` | `pptx/tdf141704.pptx` | `../core/sd/qa/unit/import-tests.cxx:709` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf142915` | `pptx/tdf142915.pptx` | `../core/sd/qa/unit/import-tests.cxx:764` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf142913` | `pptx/tdf142913.pptx` | `../core/sd/qa/unit/import-tests.cxx:778` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testTdf142590` | `pptx/tdf142590.pptx` | `../core/sd/qa/unit/import-tests.cxx:792` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testCustomSlideShow` | `pptx/tdf131390.pptx` | `../core/sd/qa/unit/import-tests.cxx:806` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testInternalHyperlink` | `pptx/tdf65724.pptx` | `../core/sd/qa/unit/import-tests.cxx:819` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testHyperlinkColor` | `pptx/tdf137367.pptx` | `../core/sd/qa/unit/import-tests.cxx:842` | `covered` | partial visible-output assertion. |
| `import-tests.cxx::testSmoketest` | `smoketest.pptx` | `../core/sd/qa/unit/import-tests.cxx:879` | `covered` | partial visible-output assertion. |
| `import-tests.cxx::testN759180` | `n759180.pptx` | `../core/sd/qa/unit/import-tests.cxx:912` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testN862510_1` | `pptx/n862510_1.pptx` | `../core/sd/qa/unit/import-tests.cxx:942` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testN862510_2` | `pptx/n862510_2.pptx` | `../core/sd/qa/unit/import-tests.cxx:966` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testN862510_4` | `pptx/n862510_4.pptx` | `../core/sd/qa/unit/import-tests.cxx:983` | `mapped` | partial visible-output assertion. |
| `import-tests.cxx::testN828390_2` | `pptx/n828390_2.pptx` | `../core/sd/qa/unit/import-tests.cxx:1005` | `covered` | partial visible-output assertion. |
| `import-tests.cxx::testN828390_3` | `pptx/n828390_3.pptx` | `../core/sd/qa/unit/import-tests.cxx:1018` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf157529` | `pptx/tdf157529.pptx` | `../core/sd/qa/unit/import-tests2.cxx:71` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf160490` | `pptx/tdf160490.pptx` | `../core/sd/qa/unit/import-tests2.cxx:96` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf165321` | `pptx/tdf165321.pptx` | `../core/sd/qa/unit/import-tests2.cxx:119` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf165341` | `pptx/tdf165341.pptx` | `../core/sd/qa/unit/import-tests2.cxx:142` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf157285` | `pptx/tdf157285.pptx` | `../core/sd/qa/unit/import-tests2.cxx:157` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf152186` | `pptx/tdf152186.pptx` | `../core/sd/qa/unit/import-tests2.cxx:180` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf93868` | `pptx/tdf93868.pptx` | `../core/sd/qa/unit/import-tests2.cxx:195` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf95932` | `pptx/tdf95932.pptx` | `../core/sd/qa/unit/import-tests2.cxx:210` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf99030` | `pptx/tdf99030.pptx` | `../core/sd/qa/unit/import-tests2.cxx:224` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf103473` | `pptx/tdf103473.pptx` | `../core/sd/qa/unit/import-tests2.cxx:267` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf103792` | `pptx/tdf103792.pptx` | `../core/sd/qa/unit/import-tests2.cxx:380` | `covered` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf148685` | `pptx/tdf148685.pptx` | `../core/sd/qa/unit/import-tests2.cxx:397` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf103876` | `pptx/tdf103876.pptx` | `../core/sd/qa/unit/import-tests2.cxx:435` | `covered` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf79007` | `pptx/tdf79007.pptx` | `../core/sd/qa/unit/import-tests2.cxx:454` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf119649` | `pptx/tdf119649.pptx` | `../core/sd/qa/unit/import-tests2.cxx:507` | `covered` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf118776` | `pptx/tdf118776.pptx` | `../core/sd/qa/unit/import-tests2.cxx:545` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf129686` | `pptx/tdf129686.pptx` | `../core/sd/qa/unit/import-tests2.cxx:563` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf104015` | `pptx/tdf104015.pptx` | `../core/sd/qa/unit/import-tests2.cxx:581` | `covered` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf104201` | `pptx/tdf104201.pptx` | `../core/sd/qa/unit/import-tests2.cxx:613` | `covered` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf103477` | `pptx/tdf103477.pptx` | `../core/sd/qa/unit/import-tests2.cxx:642` | `covered` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf105150` | `pptx/tdf105150.pptx` | `../core/sd/qa/unit/import-tests2.cxx:713` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf123684` | `pptx/tdf123684.pptx` | `../core/sd/qa/unit/import-tests2.cxx:726` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf104445` | `pptx/tdf104445.pptx` | `../core/sd/qa/unit/import-tests2.cxx:750` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf100926` | `pptx/tdf100926.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1020` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf89064` | `pptx/tdf89064.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1045` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf109067` | `pptx/tdf109067.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1069` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf109187` | `pptx/tdf109187.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1078` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf100065` | `pptx/tdf100065.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1107` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf90626` | `pptx/tdf90626.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1126` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf138148` | `pptx/tdf138148.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1143` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf134174` | `pptx/tdf134174.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1181` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf134210` | `pptx/tdf134210.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1196` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf114913` | `pptx/tdf114913.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1211` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf114821` | `pptx/tdf114821.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1272` | `mapped` | partial visible-output assertion. |
| `import-tests2.cxx::testTdf115394` | `pptx/tdf115394.pptx` | `../core/sd/qa/unit/import-tests2.cxx:1324` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testN778859` | `pptx/n778859.pptx` | `../core/sd/qa/unit/import-tests3.cxx:131` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testPlaceholderPriority` | `ppt/placeholder-priority.pptx` | `../core/sd/qa/unit/import-tests3.cxx:159` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testFdo72998` | `pptx/cshapes.pptx` | `../core/sd/qa/unit/import-tests3.cxx:180` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc870237` | `pptx/bnc870237.pptx` | `../core/sd/qa/unit/import-tests3.cxx:348` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf150789` | `pptx/tdf150789.pptx` | `../core/sd/qa/unit/import-tests3.cxx:363` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testCreationDate` | `fdo71434.pptx` | `../core/sd/qa/unit/import-tests3.cxx:394` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testMultiColTexts` | `pptx/multicol.pptx` | `../core/sd/qa/unit/import-tests3.cxx:408` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testPredefinedTableStyle` | `pptx/predefined-table-style.pptx` | `../core/sd/qa/unit/import-tests3.cxx:429` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc887225` | `pptx/bnc887225.pptx` | `../core/sd/qa/unit/import-tests3.cxx:455` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc584721_1` | `pptx/bnc584721_1_2.pptx` | `../core/sd/qa/unit/import-tests3.cxx:498` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc584721_2` | `pptx/bnc584721_1_2.pptx` | `../core/sd/qa/unit/import-tests3.cxx:512` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc591147` | `pptx/bnc591147.pptx` | `../core/sd/qa/unit/import-tests3.cxx:522` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc584721_4` | `pptx/bnc584721_4.pptx` | `../core/sd/qa/unit/import-tests3.cxx:555` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc904423` | `pptx/bnc904423.pptx` | `../core/sd/qa/unit/import-tests3.cxx:575` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testShapeLineStyle` | `pptx/ShapeLineProperties.pptx` | `../core/sd/qa/unit/import-tests3.cxx:617` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testTableBorderLineStyle` | `pptx/tableBorderLineStyle.pptx` | `../core/sd/qa/unit/import-tests3.cxx:670` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testTableMergedCellsBorderLineStyle` | `pptx/tdf149865.pptx` | `../core/sd/qa/unit/import-tests3.cxx:707` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc862510_6` | `pptx/bnc862510_6.pptx` | `../core/sd/qa/unit/import-tests3.cxx:729` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc862510_7` | `pptx/bnc862510_7.pptx` | `../core/sd/qa/unit/import-tests3.cxx:749` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testBulletSuffix` | `pptx/n83889.pptx` | `../core/sd/qa/unit/import-tests3.cxx:892` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testBnc910045` | `pptx/bnc910045.pptx` | `../core/sd/qa/unit/import-tests3.cxx:907` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testRowHeight_n80340` | `pptx/n80340.pptx` | `../core/sd/qa/unit/import-tests3.cxx:925` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testRowHeight_tableScale` | `pptx/tablescale.pptx` | `../core/sd/qa/unit/import-tests3.cxx:941` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf93830` | `pptx/tdf93830.pptx` | `../core/sd/qa/unit/import-tests3.cxx:962` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf165732` | `pptx/tdf165732.pptx` | `../core/sd/qa/unit/import-tests3.cxx:979` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf127129` | `pptx/tdf127129.pptx` | `../core/sd/qa/unit/import-tests3.cxx:1008` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf93097` | `pptx/tdf93097.pptx` | `../core/sd/qa/unit/import-tests3.cxx:1025` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf62255` | `pptx/tdf62255.pptx` | `../core/sd/qa/unit/import-tests3.cxx:1037` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf89927` | `pptx/tdf89927.pptx` | `../core/sd/qa/unit/import-tests3.cxx:1061` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf103800` | `pptx/tdf103800.pptx` | `../core/sd/qa/unit/import-tests3.cxx:1074` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf151767` | `pptx/tdf151767.pptx` | `../core/sd/qa/unit/import-tests3.cxx:1087` | `covered` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf152070` | `pptx/tdf152070.pptx` | `../core/sd/qa/unit/import-tests3.cxx:1114` | `mapped` | partial visible-output assertion. |
| `import-tests3.cxx::testTdf111927` | `pptx/tdf163239.pptx` | `../core/sd/qa/unit/import-tests3.cxx:1143` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf51340` | `pptx/tdf51340.pptx` | `../core/sd/qa/unit/import-tests4.cxx:61` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf149206` | `pptx/tdf149206.pptx` | `../core/sd/qa/unit/import-tests4.cxx:164` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testtdf163852` | `pptx/tdf163852.pptx` | `../core/sd/qa/unit/import-tests4.cxx:183` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf149785` | `pptx/tdf149785.pptx` | `../core/sd/qa/unit/import-tests4.cxx:198` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf149985` | `pptx/tdf149985.pptx` | `../core/sd/qa/unit/import-tests4.cxx:207` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf150770` | `pptx/tdf150770.pptx` | `../core/sd/qa/unit/import-tests4.cxx:228` | `covered` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf120028` | `pptx/tdf120028.pptx` | `../core/sd/qa/unit/import-tests4.cxx:236` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testDescriptionImport` | `pptx/altdescription.pptx` | `../core/sd/qa/unit/import-tests4.cxx:267` | `mapped` | partial tagged-PDF/accessibility assertion. |
| `import-tests4.cxx::testTdf47365` | `pptx/loopNoPause.pptx` | `../core/sd/qa/unit/import-tests4.cxx:318` | `mapped` | partial slideshow-state assertion; do not turn into a full PDF export test. |
| `import-tests4.cxx::testOOXTheme` | `pptx/ooxtheme.pptx` | `../core/sd/qa/unit/import-tests4.cxx:358` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testCropToShape` | `pptx/crop-to-shape.pptx` | `../core/sd/qa/unit/import-tests4.cxx:380` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf127964` | `pptx/tdf127964.pptx` | `../core/sd/qa/unit/import-tests4.cxx:401` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf106638` | `pptx/tdf106638.pptx` | `../core/sd/qa/unit/import-tests4.cxx:437` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf128684` | `pptx/tdf128684.pptx` | `../core/sd/qa/unit/import-tests4.cxx:458` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf113198` | `pptx/tdf113198.pptx` | `../core/sd/qa/unit/import-tests4.cxx:479` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testShapeGlowEffectPPTXImpoer` | `pptx/shape-glow-effect.pptx` | `../core/sd/qa/unit/import-tests4.cxx:504` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testShapeTextGlowEffectPPTXImport` | `pptx/shape-text-glow-effect.pptx` | `../core/sd/qa/unit/import-tests4.cxx:520` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testShapeBlurPPTXImport` | `pptx/shape-blur-effect.pptx` | `../core/sd/qa/unit/import-tests4.cxx:536` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testMirroredGraphic` | `pptx/mirrored-graphic.pptx` | `../core/sd/qa/unit/import-tests4.cxx:550` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf134210CropPosition` | `pptx/crop-position.pptx` | `../core/sd/qa/unit/import-tests4.cxx:563` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testGreysScaleGraphic` | `pptx/greysscale-graphic.pptx` | `../core/sd/qa/unit/import-tests4.cxx:578` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf103347` | `pptx/tdf103347.pptx` | `../core/sd/qa/unit/import-tests4.cxx:591` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testHyperlinksOnShapes` | `pptx/tdf144616.pptx` | `../core/sd/qa/unit/import-tests4.cxx:609` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf112209` | `pptx/tdf112209.pptx` | `../core/sd/qa/unit/import-tests4.cxx:664` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf128596` | `pptx/tdf128596.pptx` | `../core/sd/qa/unit/import-tests4.cxx:684` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testCropToZero` | `pptx/croppedTo0.pptx` | `../core/sd/qa/unit/import-tests4.cxx:720` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf144092TableHeight` | `pptx/tdf144092-tableHeight.pptx` | `../core/sd/qa/unit/import-tests4.cxx:727` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf89928BlackWhiteThreshold` | `pptx/tdf89928-blackWhiteEffectThreshold.pptx` | `../core/sd/qa/unit/import-tests4.cxx:740` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf151547TransparentWhiteText` | `pptx/tdf151547-transparent-white-text.pptx` | `../core/sd/qa/unit/import-tests4.cxx:777` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf149961AutofitIndentation` | `pptx/tdf149961-autofitIndentation.pptx` | `../core/sd/qa/unit/import-tests4.cxx:795` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf149588TransparentSolidFill` | `pptx/tdf149588_transparentSolidFill.pptx` | `../core/sd/qa/unit/import-tests4.cxx:829` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testIndentDuplication` | `pptx/formatting-bullet-indent.pptx` | `../core/sd/qa/unit/import-tests4.cxx:867` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::test_srcRect_smallNegBound` | `pptx/tdf153008-srcRect-smallNegBound.pptx` | `../core/sd/qa/unit/import-tests4.cxx:885` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf153012` | `pptx/chart_pt_color_bg1.pptx` | `../core/sd/qa/unit/import-tests4.cxx:904` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testMasterSlides` | `pptx/master-slides.pptx` | `../core/sd/qa/unit/import-tests4.cxx:935` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::tdf158512` | `pptx/tdf158512.pptx` | `../core/sd/qa/unit/import-tests4.cxx:1017` | `mapped` | partial visible-output assertion. |
| `import-tests4.cxx::testTdf169524` | `pptx/tdf169524.pptx` | `../core/sd/qa/unit/import-tests4.cxx:1030` | `mapped` | partial visible-output assertion. |
| `import-tests_skia.cxx::testTdf156856` | `pptx/tdf156856.pptx` | `../core/sd/qa/unit/import-tests_skia.cxx:28` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testBase` | `pptx/smartart1.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:78` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testChildren` | `pptx/smartart-children.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:144` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testText` | `pptx/smartart-text.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:179` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testCnt` | `pptx/smartart-cnt.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:192` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testDir` | `pptx/smartart-dir.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:207` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf148665` | `pptx/tdf148665.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:218` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf148921` | `pptx/tdf148921.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:233` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testMaxDepth` | `pptx/smartart-maxdepth.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:255` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testRotation` | `pptx/smartart-rotation.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:272` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTextAutoRotation` | `pptx/smartart-autoTxRot.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:290` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testPyramidOneChild` | `pptx/smartart-pyramid-1child.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:378` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testChevron` | `pptx/smartart-chevron.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:390` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testCycle` | `pptx/smartart-cycle.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:413` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testMultidirectional` | `pptx/smartart-multidirectional.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:448` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testBaseRtoL` | `pptx/smartart-rightoleftblockdiagram.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:467` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testVerticalBoxList` | `pptx/smartart-vertical-box-list.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:533` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testVerticalBracketList` | `pptx/vertical-bracket-list.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:565` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTableList` | `pptx/table-list.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:578` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testAccentProcess` | `pptx/smartart-accent-process.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:607` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testContinuousBlockProcess` | `pptx/smartart-continuous-block-process.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:695` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testOrgChart` | `pptx/smartart-org-chart.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:720` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testCycleMatrix` | `pptx/smartart-cycle-matrix.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:844` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testPictureStrip` | `pptx/smartart-picture-strip.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:932` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testInteropGrabBag` | `pptx/smartart-interopgrabbag.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1010` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testBackground` | `pptx/smartart-background.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1032` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testBackgroundDrawingmlFallback` | `pptx/smartart-background-drawingml-fallback.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1064` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testCenterCycle` | `pptx/smartart-center-cycle.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1098` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testFontSize` | `pptx/smartart-font-size.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1123` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testVerticalBlockList` | `pptx/smartart-vertical-block-list.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1150` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testMissingBulletAndIndent` | `pptx/smartart-missing-bullet.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1192` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testBulletList` | `pptx/smartart-bullet-list.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1224` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testRecursion` | `pptx/smartart-recursion.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1249` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testDataFollow` | `pptx/smartart-data-follow.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1290` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testOrgChart2` | `pptx/smartart-org-chart2.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1331` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf131553` | `pptx/tdf131553.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1370` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testFillColorList` | `pptx/fill-color-list.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1382` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf134221` | `pptx/smartart-tdf134221.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1414` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testLinearRule` | `pptx/smartart-linear-rule.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1427` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testLinearRuleVert` | `pptx/smartart-linear-rule-vert.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1462` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testAutofitSync` | `pptx/smartart-autofit-sync.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1480` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testSnakeRows` | `pptx/smartart-snake-rows.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1517` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testCompositeInferRight` | `pptx/smartart-composite-infer-right.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1541` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf149551Pie` | `pptx/tdf149551_SmartArt_Pie.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1561` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf149551Pyramid` | `pptx/tdf149551_SmartArt_Pyramid.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1582` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf149551Venn` | `pptx/tdf149551_SmartArt_Venn.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1603` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf149551Gear` | `pptx/tdf149551_SmartArt_Gear.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1624` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf145528Matrix` | `pptx/tdf145528_SmartArt_Matrix.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1645` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf135953TextPosition` | `pptx/tdf135953_SmartArt_textposition.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1676` | `mapped` | partial visible-output assertion. |
| `import-tests-smartart.cxx::testTdf132302RightArrow` | `pptx/tdf132302_SmartArt_rightArrow.pptx` | `../core/sd/qa/unit/import-tests-smartart.cxx:1699` | `mapped` | partial visible-output assertion. |
| `layout-tests.cxx::testTdf104722` | `pptx/tdf104722.pptx` | `../core/sd/qa/unit/layout-tests.cxx:36` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTdf135843` | `pptx/tdf135843.pptx` | `../core/sd/qa/unit/layout-tests.cxx:49` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTdf128212` | `pptx/tdf128212.pptx` | `../core/sd/qa/unit/layout-tests.cxx:103` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::numberedList` | `pptx/NumberedList-12ab-ab-34.pptx` | `../core/sd/qa/unit/layout-tests.cxx:273` | `mapped/ignored` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTdf146731` | `pptx/tdf146731.pptx` | `../core/sd/qa/unit/layout-tests.cxx:305` | `mapped` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTdf135843_InsideHBorders` | `pptx/tdf135843_insideH.pptx` | `../core/sd/qa/unit/layout-tests.cxx:321` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testBnc480256` | `pptx/bnc480256-2.pptx` | `../core/sd/qa/unit/layout-tests.cxx:334` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTdf148966` | `pptx/tdf148966.pptx` | `../core/sd/qa/unit/layout-tests.cxx:386` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTableVerticalText` | `pptx/tcPr-vert-roundtrip.pptx` | `../core/sd/qa/unit/layout-tests.cxx:417` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTdf164622` | `pptx/tdf164622.pptx` | `../core/sd/qa/unit/layout-tests.cxx:496` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTdf168010_PPTX` | `pptx/trailing-paragraphs.pptx` | `../core/sd/qa/unit/layout-tests.cxx:595` | `covered` | partial metafile/PDF primitive assertion. |
| `layout-tests.cxx::testTdf128206` | `pptx/tdf128206.pptx` | `../core/sd/qa/unit/layout-tests.cxx:641` | `covered` | partial metafile/PDF primitive assertion. |
| `PNGExportTests.cxx::testTdf156808` | `pptx/tdf156808.pptx` | `../core/sd/qa/unit/PNGExportTests.cxx:250` | `mapped` | full page raster assertion. |
| `PNGExportTests.cxx::testTdf157793` | `pptx/tdf157793.pptx` | `../core/sd/qa/unit/PNGExportTests.cxx:503` | `mapped` | full page raster assertion. |
| `PNGExportTests.cxx::testTdf157635` | `pptx/tdf157635.pptx` | `../core/sd/qa/unit/PNGExportTests.cxx:553` | `mapped` | full page raster assertion. |
| `PNGExportTests.cxx::testTdf113163` | `pptx/tdf113163.pptx` | `../core/sd/qa/unit/PNGExportTests.cxx:603` | `mapped` | full page raster assertion. |
| `ThemeTest.cxx::testThemeChange` | `theme.pptx` | `../core/sd/qa/unit/ThemeTest.cxx:65` | `mapped` | full theme-render assertion. |
| `ShapeImportExportTest.cxx::testTextDistancesOOXML` | `TextDistancesInsets1.pptx` | `../core/sd/qa/unit/ShapeImportExportTest.cxx:58` | `mapped` | partial text-inset geometry assertion. |
| `ShapeImportExportTest.cxx::testTextDistancesOOXML_LargerThanTextAreaSpecialCase` | `TextDistancesInsets2.pptx` | `../core/sd/qa/unit/ShapeImportExportTest.cxx:167` | `mapped` | partial text-inset geometry assertion. |
| `FontEmbeddingTest.cxx::testRoundtripEmbeddedFontsPPTX` | `BoldonseFontEmbedded.pptx` | `../core/sd/qa/unit/FontEmbeddingTest.cxx:34` | `mapped` | full font/PDF-resource assertion. |
| `FontEmbeddingTest.cxx::testTdf167214` | `pptx/tdf167214.pptx` | `../core/sd/qa/unit/FontEmbeddingTest.cxx:137` | `mapped` | full font/PDF-resource assertion. |

### PPTX Deferrals And Non-Targets

| Source or fixture | Status | Reason |
|---|---|---|
| `sd/qa/unit/export-tests*.cxx` | `deferred` | Mostly PPTX export XML, save/reload, or round-trip assertions. Promote only individual rows with direct source-PPTX visible output evidence. |
| `ShapeImportExportTest.cxx::testTextDistancesOOXML_Export` and `TextDistancesInsets3.pptx` | `deferred` | Exported OOXML attribute assertions, not source PPTX -> PDF-visible output. |
| `import-tests2.cxx::testTdf108926` | `deferred` | Fixture path is `pptx/tdf108926.ppt`; this is a binary PPT notes-page import test, not a PPTX row. |
| `import-tests3.cxx::testStrictOOXML` and `strict_ooxml.pptx` | `deferred` | Strict package parsing is not PDF-visible rendering evidence. |
| Animation/transition-only rows beyond `import-tests4.cxx::testTdf47365` | `deferred` | Static PDF output does not represent slideshow timing, transitions, or animation playback. |
| `sd/qa/unit/SlideSectionTest.cxx` | `deferred` | Section metadata and custom show behavior are not static PDF rendering. |
| `sd/qa/unit/activex-controls-tests.cxx` | `deferred` | ActiveX/control UNO state is not PDF output unless a visible rendered-control or PDF widget assertion is defined. |
| `sd/qa/unit/uiimpress.cxx` | `deferred` | UI command/editor tests are not source PPTX -> static rendered PDF tests. |
| `sd/qa/unit/TextFittingTest.cxx` | `deferred` | PPTX references are comment-only visual comparison documents, not automated LibreOffice assertions to port. |
