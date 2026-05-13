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
- `mapped` means the upstream assertion is visible-output evidence, but still
  needs projection to PDF text/path/image/color/bounds/raster assertions.
- `deferred` means the source is useful background only; do not migrate until a
  concrete PDF-visible assertion is identified.
- Expected values must come from LibreOffice assertions, LibreOffice reference
  output, or fixture evidence. Do not invent expected values from current
  `ooxmlsdk-pdf` output.

## Scan Summary

- Local covered PDF-rendering fixtures: 13.
- Explicit upstream DOCX -> PDF tests with PDF assertions found: 10.
- Additional visible-output candidates listed individually below: 147 mapped
  tests.
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
| `svdraw.cxx::testPageViewDrawLayerClip` | `page-view-draw-layer-clip.docx` | `../core/svx/qa/unit/data/page-view-draw-layer-clip.docx` | `planned` | Page 1 has 3 objects, page 2 has 2 objects. |
| `itrform2.cxx::testContentControlHeaderPDFExport` | `content-control-header.docx` | `../core/sw/qa/core/text/data/content-control-header.docx` | `planned` | Page 2 has 3 text objects. |
| `text.cxx::testDropdownContentControlPDF2` | `tdf153040.docx` | `../core/sw/qa/core/text/data/tdf153040.docx` | `planned` | 4 annotations; first widget is a combo box with value `Apfel`. |
| `uiwriter8.cxx::testTdf131728` | `tdf131728.docx` | `../core/sw/qa/extras/uiwriter/data/tdf131728.docx` | `planned` | PDF bookmark order matches upstream expected outline strings. |

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
