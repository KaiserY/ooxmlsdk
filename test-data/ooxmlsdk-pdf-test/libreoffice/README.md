# ooxmlsdk-pdf-test fixtures

This directory contains DOCX fixtures used by `crates/ooxmlsdk-pdf-test` for
upstream-aligned PDF export assertions.

`ooxmlsdk-pdf-test` assumes these DOCX files are valid test inputs. DOCX
package/schema/relationship round-trip behavior is covered by
`crates/ooxmlsdk-test`; this directory is only for `docx -> pdf` behavior.

Fixtures in this directory should come from LibreOffice DOCX -> visible PDF
coverage, paired with an existing upstream assertion. Prefer strict
`../core/vcl/qa/cppunit/pdfexport/data/` fixtures when available, then scattered
Writer/SVX/OoXML tests that export or assert visible page output. Avoid adding
hand-crafted fixtures or inferred expectations; copy upstream fixtures and port
the upstream assertion values directly.

Fixtures that are not traceable to `../core` should not live here. Put them in
a sibling `misc/` bucket once that category exists so the LibreOffice boundary
stays explicit.

Strict upstream `pdfexport` / `pdfexport2` DOCX fixtures:

- `content-control-rtl.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/content-control-rtl.docx`.
- `fdo47811-1_Word2013.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/fdo47811-1_Word2013.docx`.
- `tdf142133.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/tdf142133.docx`.
- `tdf145274.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/tdf145274.docx`.
- `tdf156685.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/tdf156685.docx`.
- `tdf129085.docx`: copied from
  `../core/vcl/qa/cppunit/pdfexport/data/tdf129085.docx`.

Additional direct PDF/object fixtures:

- `page-view-draw-layer-clip.docx`: copied from
  `../core/svx/qa/unit/data/page-view-draw-layer-clip.docx`.
- `content-control-header.docx`: copied from
  `../core/sw/qa/core/text/data/content-control-header.docx`.
- `tdf153040.docx`: copied from
  `../core/sw/qa/core/text/data/tdf153040.docx`.
- `tdf131728.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/tdf131728.docx`.

Core-derived visible-output DOCX fixtures:

- `semi-transparent-text.docx`: copied from
  `../core/sw/qa/writerfilter/dmapper/data/semi-transparent-text.docx`.
- `tdf152884_Char_Transparency.docx`: copied from
  `../core/sw/qa/writerfilter/dmapper/data/tdf152884_Char_Transparency.docx`.
- `chart-data-label-char-color.docx`: copied from
  `../core/oox/qa/unit/data/chart-data-label-char-color.docx`.
- `tdf54095_SmartArtThemeTextColor.docx`: copied from
  `../core/oox/qa/unit/data/tdf54095_SmartArtThemeTextColor.docx`.
- `tdf125885_WordArt2.docx`: copied from
  `../core/oox/qa/unit/data/tdf125885_WordArt2.docx`.
- `tdf152840_WordArt_non_accent_color.docx`: copied from
  `../core/oox/qa/unit/data/tdf152840_WordArt_non_accent_color.docx`.
- `tdf152840_theme_color_non_accent.docx`: copied from
  `../core/oox/qa/unit/data/tdf152840_theme_color_non_accent.docx`.
- `tdf152896_WordArt_color_darken.docx`: copied from
  `../core/oox/qa/unit/data/tdf152896_WordArt_color_darken.docx`.

These supplemental DOCX fixtures are not native `pdfexport` cases. They come
from other LibreOffice subsystems, but each one asserts a final visible-output
property that can be checked directly on the generated PDF without depending on
Writer's internal document model.

Mapped visible-output fixtures:

- `fdo66145.docx`: copied from
  `../core/sw/qa/core/header_footer/data/fdo66145.docx`.
- `first-header-footer.docx`: copied from
  `../core/sw/qa/core/header_footer/data/first-header-footer.docx`.
- `cont-sect-break-header-footer.docx`: copied from
  `../core/sw/qa/core/header_footer/data/cont-sect-break-header-footer.docx`.
- `tdf166205_first_page_header_footer_visible.docx`: copied from
  `../core/sw/qa/core/header_footer/data/tdf166205_first_page_header_footer_visible.docx`.
- `testTitlePage.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/testTitlePage.docx`.
- `inheritFirstHeader.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/inheritFirstHeader.docx`.
- `n750255.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/n750255.docx`.
- `n780843.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/n780843.docx`.
- `tdf155736_PageNumbers_footer.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf155736_PageNumbers_footer.docx`.
- `num-override-lvltext.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/num-override-lvltext.docx`.
- `tdf147646_mergedCellNumbering.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf147646_mergedCellNumbering.docx`.
- `tdf153613_anchoredAfterPgBreak.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153613_anchoredAfterPgBreak.docx`.
- `tdf153613_anchoredAfterPgBreak2.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153613_anchoredAfterPgBreak2.docx`.
- `tdf153613_anchoredAfterPgBreak3.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153613_anchoredAfterPgBreak3.docx`.
- `tdf153613_anchoredAfterPgBreak6.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153613_anchoredAfterPgBreak6.docx`.
- `tdf153613_inlineAfterPgBreak.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153613_inlineAfterPgBreak.docx`.
- `tdf153613_inlineAfterPgBreak2.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153613_inlineAfterPgBreak2.docx`.
- `tdf153613_textboxAfterPgBreak3.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153613_textboxAfterPgBreak3.docx`.
- `tdf147724.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf147724.docx`.
- `n751077.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/n751077.docx`.
- `tdf136952_pgBreak3.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf136952_pgBreak3.docx`.
- `tdf123636_newlinePageBreak3.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf123636_newlinePageBreak3.docx`.
- `tdf123636_newlinePageBreak4.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf123636_newlinePageBreak4.docx`.
- `tdf169802_hidden_shape.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf169802_hidden_shape.docx`.
- `tdf124594.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf124594.docx`.
- `128197_compat14.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/128197_compat14.docx`.
- `128197_compat15.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/128197_compat15.docx`.
- `tdf149313.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf149313.docx`.
- `tdf154369.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf154369.docx`.
- `tdf75573_page1frame.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf75573_page1frame.docx`.
- `tdf120548.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf120548.docx`.
- `tdf95495.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf95495.docx`.
- `fdo81031.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/fdo81031.docx`.
- `n705956-1.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/n705956-1.docx`.
- `textbox_picturefill.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/textbox_picturefill.docx`.
- `tdf117923.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/tdf117923.docx`.
- `tdf104492.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/tdf104492.docx`.
- `floattable-multi-nested.docx`: copied from
  `../core/sw/qa/writerfilter/ooxml/data/floattable-multi-nested.docx`.
- `tdf124600.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf124600.docx`.
- `tdf113946.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf113946.docx`.
- `picture-with-schemecolor.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/picture-with-schemecolor.docx`.
- `n777345.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/n777345.docx`.
- `msobrightnesscontrast.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/msobrightnesscontrast.docx`.
- `tdf136841.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf136841.docx`.
- `tdf156078_rightTabOutsideParaRightIndent.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf156078_rightTabOutsideParaRightIndent.docx`.
- `tdf132976_testRelativeAnchorWidthFromLeftMargin.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf132976_testRelativeAnchorWidthFromLeftMargin.docx`.
- `tdf133861_RelativeAnchorWidthFromInsideOutsideMargin.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf133861_RelativeAnchorWidthFromInsideOutsideMargin.docx`.
- `tdf133045_TestShapeAlignmentRelativeFromTopMargin.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf133045_TestShapeAlignmentRelativeFromTopMargin.docx`.
- `tdf113183.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf113183.docx`.
- `tdf120511_eatenSection.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf120511_eatenSection.docx`.
- `tdf119760_positionCellBorder.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf119760_positionCellBorder.docx`.
- `tdf116985.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf116985.docx`.
- `090716_Studentische_Arbeit_VWS.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/090716_Studentische_Arbeit_VWS.docx`.
- `tdf102466.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf102466.docx`.
- `tdf95367_inheritFollowStyle.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf95367_inheritFollowStyle.docx`.
- `redline-table.docx`: copied from
  `../core/sw/qa/core/layout/data/redline-table.docx`.
- `redline.docx`: copied from
  `../core/sw/qa/core/text/data/redline.docx`.
- `redline-image-anchored.docx`: copied from
  `../core/sw/qa/core/text/data/redline-image-anchored.docx`.
- `redline-image-inline.docx`: copied from
  `../core/sw/qa/core/text/data/redline-image-inline.docx`.
- `redline-number-portion.docx`: copied from
  `../core/sw/qa/core/text/data/redline-number-portion.docx`.
- `redline-bullet.docx`: copied from
  `../core/sw/qa/core/text/data/redline-bullet.docx`.
- `tdf114799_highlight.docx`: copied from
  `../core/sw/qa/extras/tiledrendering/data/tdf114799_highlight.docx`.
- `tdf114799_shd.docx`: copied from
  `../core/sw/qa/extras/tiledrendering/data/tdf114799_shd.docx`.
- `tdf159626_yellowPatternFill.docx`: copied from
  `../core/sw/qa/extras/tiledrendering/data/tdf159626_yellowPatternFill.docx`.
- `tdf159626_yellowPatternFillB.docx`: copied from
  `../core/sw/qa/extras/tiledrendering/data/tdf159626_yellowPatternFillB.docx`.
- `tdf159626_blackPatternFill.docx`: copied from
  `../core/sw/qa/extras/tiledrendering/data/tdf159626_blackPatternFill.docx`.
- `CT-formatted-deletion.docx`: copied from
  `../core/sw/qa/extras/layout/data/CT-formatted-deletion.docx`.
- `tdf104797.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf104797.docx`.
- `tdf155229_row_height_at_least.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf155229_row_height_at_least.docx`.
- `tdf164907_rowHeightAtLeast.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf164907_rowHeightAtLeast.docx`.
- `tdf167526.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf167526.docx`.
- `tdf167540.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf167540.docx`.
- `tdf105035_framePrB.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf105035_framePrB.docx`.
- `tdf105035_framePrC.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf105035_framePrC.docx`.
- `tdf37153_considerWrapOnObjPos.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf37153_considerWrapOnObjPos.docx`.
- `tdf150822.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf150822.docx`.
- `tdf64264.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf64264.docx`.
- `tdf58944-repeating-table-header.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf58944-repeating-table-header.docx`.
- `tdf81100.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf81100.docx`.
- `tdf130804.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf130804.docx`.
- `tdf105143.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf105143.docx`.
- `floating-table-section-columns.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/floating-table-section-columns.docx`.
- `tdf60351.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf60351.docx`.
- `tdf98882.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf98882.docx`.
- `tdf100072.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf100072.docx`.
- `tdf114212.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf114212.docx`.
- `tdf84678.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf84678.docx`.
- `tdf103544.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf103544.docx`.
- `StyleRef-DE.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/StyleRef-DE.docx`.
- `n758883.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/n758883.docx`.
- `tdf95377.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf95377.docx`.
- `negative-cell-margin-twips.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/negative-cell-margin-twips.docx`.
- `tdf153042_largeTab.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153042_largeTab.docx`.
- `text-box-word-wrap.docx`: copied from
  `../core/sw/qa/core/doc/data/text-box-word-wrap.docx`.
- `tdf153042_noTab.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153042_noTab.docx`.
- `tdf134063.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf134063.docx`.
- `tdf148360.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf148360.docx`.
- `tdf163894.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf163894.docx`.
- `tdf163894_hidden.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf163894_hidden.docx`.
- `tdf32363.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf32363.docx`.
- `tdf163894_from_top.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf163894_from_top.docx`.
- `tdf135595_HFtableWrap_c12.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf135595_HFtableWrap_c12.docx`.
- `tdf78749.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf78749.docx`.
- `TestFirstFooterDisabled.docx`: copied from
  `../core/sw/qa/core/header_footer/data/TestFirstFooterDisabled.docx`.
- `textbox-right-edge.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/textbox-right-edge.docx`.
- `wpg-nested.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/wpg-nested.docx`.
- `tdf151704_thinColumnHeight.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf151704_thinColumnHeight.docx`.
- `tdf133000_numStyleFormatting.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf133000_numStyleFormatting.docx`.
- `tdf78352.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf78352.docx`.
- `tdf83309.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf83309.docx`.
- `tdf131801.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf131801.docx`.
- `tdf135949_anchoredBeforeBreak.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf135949_anchoredBeforeBreak.docx`.
- `tdf133070_testRelativeAnchorHeightFromBottomMarginHasFooter.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf133070_testRelativeAnchorHeightFromBottomMarginHasFooter.docx`.
- `tdf153128.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153128.docx`.
- `bnc891663.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/bnc891663.docx`.
- `listWithLgl.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/listWithLgl.docx`.
- `tdf160077_layoutInCell.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf160077_layoutInCell.docx`.
- `tdf160077_layoutInCellB.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf160077_layoutInCellB.docx`.
- `tdf160077_layoutInCellC.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf160077_layoutInCellC.docx`.
- `tdf160077_layoutInCellD.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf160077_layoutInCellD.docx`.
- `tdf153909_followTextFlow.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf153909_followTextFlow.docx`.
- `tdf162541_notLayoutInCell_paraLeft.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf162541_notLayoutInCell_paraLeft.docx`.
- `tdf162551_notLayoutInCell_charLeft_fromTop.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf162551_notLayoutInCell_charLeft_fromTop.docx`.
- `tdf123324_testRelativeAnchorHeightFromTopMarginHasHeader.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf123324_testRelativeAnchorHeightFromTopMarginHasHeader.docx`.
- `tdf123324_testRelativeAnchorHeightFromTopMarginNoHeader.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf123324_testRelativeAnchorHeightFromTopMarginNoHeader.docx`.
- `tdf165492_exactWithBottomSpacing.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf165492_exactWithBottomSpacing.docx`.
- `tdf165492_atLeastWithBottomSpacing.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf165492_atLeastWithBottomSpacing.docx`.
- `tdf165047_consolidatedTopMargin.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf165047_consolidatedTopMargin.docx`.
- `tdf165047_contextualSpacingTopMargin.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf165047_contextualSpacingTopMargin.docx`.
- `tdf146346.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf146346.docx`.
- `tdf165354.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf165354.docx`.
- `tdf166544_noTopMargin_fields.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf166544_noTopMargin_fields.docx`.
- `tdf138020_all_rows_tblHeader.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf138020_all_rows_tblHeader.docx`.
- `tdf166510_sectPr_bottomSpacing.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf166510_sectPr_bottomSpacing.docx`.
- `tdf169986_bottomSpacing.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf169986_bottomSpacing.docx`.
- `tdf167657_sectPr_bottomSpacing.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf167657_sectPr_bottomSpacing.docx`.
- `number-portion-noformat.docx`: copied from
  `../core/sw/qa/core/text/data/number-portion-noformat.docx`.
- `ignore-top-margin.docx`: copied from
  `../core/sw/qa/core/layout/data/ignore-top-margin.docx`.
- `ignore-top-margin-table.docx`: copied from
  `../core/sw/qa/core/layout/data/ignore-top-margin-table.docx`.
- `ignore-top-margin-page-style-change.docx`: copied from
  `../core/sw/qa/core/layout/data/ignore-top-margin-page-style-change.docx`.
- `inline-endnote-position.docx`: copied from
  `../core/sw/qa/core/layout/data/inline-endnote-position.docx`.
- `table-fly-overlap.docx`: copied from
  `../core/sw/qa/core/layout/data/table-fly-overlap.docx`.
- `table-fly-overlap-spacing.docx`: copied from
  `../core/sw/qa/core/layout/data/table-fly-overlap-spacing.docx`.
- `tdf128195.docx`: copied from
  `../core/sw/qa/core/layout/data/tdf128195.docx`.
- `border-collapse-compat.docx`: copied from
  `../core/sw/qa/core/layout/data/border-collapse-compat.docx`.
- `textbox-autogrow-vertical.docx`: copied from
  `../core/sw/qa/core/layout/data/textbox-autogrow-vertical.docx`.
- `header-textbox.docx`: copied from
  `../core/sw/qa/core/layout/data/header-textbox.docx`.
- `vmerge-cell-border.docx`: copied from
  `../core/sw/qa/core/layout/data/vmerge-cell-border.docx`.
- `inner-border.docx`: copied from
  `../core/sw/qa/core/layout/data/inner-border.docx`.
- `double-border-vertical.docx`: copied from
  `../core/sw/qa/core/layout/data/double-border-vertical.docx`.
- `double-border-horizontal.docx`: copied from
  `../core/sw/qa/core/layout/data/double-border-horizontal.docx`.
- `para-border-in-cell-clip.docx`: copied from
  `../core/sw/qa/core/layout/data/para-border-in-cell-clip.docx`.
- `double-page-border.docx`: copied from
  `../core/sw/qa/core/layout/data/double-page-border.docx`.
- `rtl-table.docx`: copied from
  `../core/sw/qa/core/layout/data/rtl-table.docx`.
- `endnote-cont-separator.docx`: copied from
  `../core/sw/qa/core/layout/data/endnote-cont-separator.docx`.
- `table-print-area-left.docx`: copied from
  `../core/sw/qa/core/layout/data/table-print-area-left.docx`.
- `tdf136588.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf136588.docx`.
- `tdf88496.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf88496.docx`.
- `tdf137025.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf137025.docx`.
- `tdf134277.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf134277.docx`.
- `tdf116486.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf116486.docx`.
- `fdo43573-2-min.docx`: copied from
  `../core/sw/qa/extras/layout/data/fdo43573-2-min.docx`.
- `tdf128198-1.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf128198-1.docx`.
- `tdf106153.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf106153.docx`.
- `tdf109137.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf109137.docx`.
- `tdf157628.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf157628.docx`.
- `tdf157596_paragraph_numbering.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf157596_paragraph_numbering.docx`.
- `hidden-para-separator.docx`: copied from
  `../core/sw/qa/extras/layout/data/hidden-para-separator.docx`.
- `tdf125300.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf125300.docx`.
- `tdf122225.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf122225.docx`.
- `legend-itemorder-min.docx`: copied from
  `../core/sw/qa/extras/layout/data/legend-itemorder-min.docx`.
- `tdf75659.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf75659.docx`.
- `long_legendentry.docx`: copied from
  `../core/sw/qa/extras/layout/data/long_legendentry.docx`.
- `tdf115630.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf115630.docx`.
- `tdf128996.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf128996.docx`.
- `tdf126244.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf126244.docx`.
- `tdf69648.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf69648.docx`.
- `tdf117982.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf117982.docx`.
- `tdf128959.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf128959.docx`.
- `tdf124423.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf124423.docx`.
- `tdf138782.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf138782.docx`.
- `tdf135035.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf135035.docx`.
- `tdf139336_ColumnsWithFootnoteDoNotOccupyEntirePage.docx`: copied
  from
  `../core/sw/qa/extras/layout/data/tdf139336_ColumnsWithFootnoteDoNotOccupyEntirePage.docx`.
- `fdo48718-1.docx`: copied from
  `../core/sw/qa/extras/layout/data/fdo48718-1.docx`.
- `fld-in-tbl.docx`: copied from
  `../core/sw/qa/extras/layout/data/fld-in-tbl.docx`.
- `sdt+framePr.docx`: copied from
  `../core/sw/qa/extras/layout/data/sdt+framePr.docx`.
- `tdf115094.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf115094.docx`.
- `tdf112290.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf112290.docx`.
- `tdf123651.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf123651.docx`.
- `tdf64222.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf64222.docx`.
- `tdf170381-split-float-table-in-normal-table.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf170381-split-float-table-in-normal-table.docx`.
- `tdf170381-split-float-table-in-float-table.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf170381-split-float-table-in-float-table.docx`.
- `tdf170620.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf170620.docx`.
- `tdf170630.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf170630.docx`.
- `tdf170846_1.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf170846_1.docx`.
- `tdf170846_2.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf170846_2.docx`.
- `xaxis-labelbreak.docx`: copied from
  `../core/sw/qa/extras/layout/data/xaxis-labelbreak.docx`.
- `tdf138773.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf138773.docx`.
- `tdf130969.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf130969.docx`.
- `tdf129054.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf129054.docx`.
- `testAreaChartNumberFormat.docx`: copied from
  `../core/sw/qa/extras/layout/data/testAreaChartNumberFormat.docx`.
- `tdf134866.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf134866.docx`.
- `tdf137116.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf137116.docx`.
- `tdf137154.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf137154.docx`.
- `outside_long_data_label.docx`: copied from
  `../core/sw/qa/extras/layout/data/outside_long_data_label.docx`.
- `tdf130031.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf130031.docx`.
- `tdf138018.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf138018.docx`.
- `tdf130380.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf130380.docx`.
- `tdf129095.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf129095.docx`.
- `tdf132956.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf132956.docx`.
- `tdf122014.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf122014.docx`.
- `tdf167202_footnote.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf167202_footnote.docx`.
- `tdf134659.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf134659.docx`.
- `tdf134235.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf134235.docx`.
- `tdf134676.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf134676.docx`.
- `tdf134146.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf134146.docx`.
- `tdf136061.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf136061.docx`.
- `tdf116925.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf116925.docx`.
- `tdf117028.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf117028.docx`.
- `tdf150200.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf150200.docx`.
- `tdf150438.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf150438.docx`.
- `tdf127118.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf127118.docx`.
- `tdf141220.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf141220.docx`.
- `tdf134685.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf134685.docx`.
- `tdf109077.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf109077.docx`.
- `tdf164903.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf164903.docx`.
- `tdf134463.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf134463.docx`.
- `tdf117188.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf117188.docx`.
- `tdf161718.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf161718.docx`.
- `tdf130088.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf130088.docx`.
- `tdf164905.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf164905.docx`.
- `tdf163149.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf163149.docx`.
- `tdf164499.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf164499.docx`.
- `writer-image-no-capture.docx`: copied from
  `../core/sw/qa/extras/layout/data/writer-image-no-capture.docx`.
- `tdf152298.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf152298.docx`.
- `tdf133070_testRelativeAnchorHeightFromBottomMarginNoFooter.docx`: copied
  from
  `../core/sw/qa/extras/ooxmlexport/data/tdf133070_testRelativeAnchorHeightFromBottomMarginNoFooter.docx`.
- `tdf133670_testRelativeAnchorWidthFromRightMargin.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf133670_testRelativeAnchorWidthFromRightMargin.docx`.
- `tdf165478_bottomAligned.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf165478_bottomAligned.docx`.
- `tdf126533_noPageBitmap.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf126533_noPageBitmap.docx`.
- `tdf126533_pageBitmap.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf126533_pageBitmap.docx`.
- `i120928.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/i120928.docx`.
- `dml-shape-fillbitmapcrop.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/dml-shape-fillbitmapcrop.docx`.
- `tdf112450_vml_polyline.docx`: copied from
  `../core/oox/qa/unit/data/tdf112450_vml_polyline.docx`.
- `tdf153000_WordArt_type_25_to_31.docx`: copied from
  `../core/svx/qa/unit/data/tdf153000_WordArt_type_25_to_31.docx`.
- `tdf138465min.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf138465min.docx`.
- `tdf97618_testVmlShapeTextWordWrap.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf97618_testVmlShapeTextWordWrap.docx`.
- `i124106.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/i124106.docx`.
- `large-twips.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/large-twips.docx`.
- `gridbefore.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/gridbefore.docx`.
- `tdf125324.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf125324.docx`.
- `tdf162746.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf162746.docx`.
- `tdf107889.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf107889.docx`.
- `tdf166850.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf166850.docx`.
- `toplevel-line-hori-offset.docx`: copied from
  `../core/oox/qa/unit/data/toplevel-line-hori-offset.docx`.
- `line-vertical-rotation.docx`: copied from
  `../core/oox/qa/unit/data/line-vertical-rotation.docx`.
- `customshape-position.docx`: copied from
  `../core/oox/qa/unit/data/customshape-position.docx`.
- `multiple-group-shapes.docx`: copied from
  `../core/oox/qa/unit/data/multiple-group-shapes.docx`.
- `inside-outside-vert-align.docx`: copied from
  `../core/sw/qa/core/objectpositioning/data/inside-outside-vert-align.docx`.
- `vml-vertical-alignment.docx`: copied from
  `../core/sw/qa/core/objectpositioning/data/vml-vertical-alignment.docx`.
- `fdo38414.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/fdo38414.docx`.
- `tdf115180.docx`: copied from
  `../core/sw/qa/extras/rtfexport/data/tdf115180.docx`.
- `tdf98987.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/tdf98987.docx`.
- `tdf99004.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/tdf99004.docx`.
- `tdf106606.docx`: copied from
  `../core/sw/qa/extras/ooxmlimport/data/tdf106606.docx`.
- `tdf156902_GlowOnGroup.docx`: copied from
  `../core/oox/qa/unit/data/tdf156902_GlowOnGroup.docx`.
- `tdf119952_negativeMargins.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf119952_negativeMargins.docx`.
- `tdf128646.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf128646.docx`.
- `tdf117923.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf117923.docx`.
- `tdf153136.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf153136.docx`.
- `tdf135943_shapeWithText_LayoutInCell0_compat15.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf135943_shapeWithText_LayoutInCell0_compat15.docx`.
- `tdf167770_marginInsideOutside.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf167770_marginInsideOutside.docx`.
- `tdf87348_linkedTextboxes.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf87348_linkedTextboxes.docx`.
- `floattable-split.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/floattable-split.docx`.
- `tdf125885_WordArt.docx`: copied from
  `../core/oox/qa/unit/data/tdf125885_WordArt.docx`.
- `tdf125885_WordArt3.docx`: copied from
  `../core/oox/qa/unit/data/tdf125885_WordArt3.docx`.
- `n793998.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/n793998.docx`.
- `footnote_spacing_hanging_para.docx`: copied from
  `../core/sw/qa/extras/odfexport/data/footnote_spacing_hanging_para.docx`.
- `tdf116256.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf116256.docx`.
- `tdf124600-layout.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf124600.docx`.
- `watermark.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/watermark.docx`.
- `camera-rotation-revolution.docx`: copied from
  `../core/oox/qa/unit/data/camera-rotation-revolution.docx`.
- `tdf151518_SmartArtTextLocation.docx`: copied from
  `../core/oox/qa/unit/data/tdf151518_SmartArtTextLocation.docx`.
- `tdf167527_title_letters_cut_from_below.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf167527_title_letters_cut_from_below.docx`.
- `tdf147126.docx`: copied from
  `../core/sw/qa/extras/uiwriter/data/tdf147126.docx`.
- `tdf139418.docx`: copied from
  `../core/sw/qa/extras/ooxmlexport/data/tdf139418.docx`.
- `floattable-anchor-split.docx`: copied from
  `../core/sw/qa/core/txtnode/data/floattable-anchor-split.docx`.
- `tdf122878.docx`: copied from
  `../core/sw/qa/extras/layout/data/tdf122878.docx`.

Assertion policy:

- Render the fixture only through `ooxmlsdk-pdf`; do not shell out to
  `soffice` or compare against a dynamically generated LibreOffice PDF.
- Prefer PDFium-observable assertions that mirror upstream `pdfexport` tests:
  page counts, annotation counts and bounds, text object font/color details,
  and link targets.
- When upstream checks PDF object dictionaries directly and PDFium cannot expose
  the same signal, add a narrow PDF-structure assertion instead of inventing a
  proxy metric.
- Tests are allowed to fail while the renderer is being brought up to parity;
  the goal is to expose gaps against upstream expectations, not to normalize
  them away.
