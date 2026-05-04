# ooxmlsdk Compatibility Matrix

Tracks which OOXML features can be round-tripped (opened and saved without
data loss) using ooxmlsdk. This is a **parser-level** matrix — it covers
structural fidelity, not rendering or layout.

**Legend:**
- ✅ Round-trips correctly in current tests
- ⚠️ Partially working or has known edge cases
- ❌ Known to fail (see linked issue)
- 🔲 Not yet tested

## Open Packaging Conventions (all formats)

| Feature | Status | Notes |
|---|---|---|
| ZIP package open/save | ✅ | Core OPC |
| Content_Types.xml | ✅ | |
| Relationship parts (.rels) | ✅ | |
| Multiple relationships per part | ✅ | `opc/multiple_rels.docx` |
| Core properties part | ✅ | `opc/core_properties.docx` |
| Thumbnail part | ✅ | `opc/thumbnail.docx` |
| MCE AlternateContent | ✅ | Fixed by static MCE post-processing |

## WordprocessingML (DOCX)

| Feature | Status | Notes |
|---|---|---|
| Empty document | ✅ | `document/minimal_empty.docx` |
| Plain text paragraphs | ✅ | `document/minimal_text.docx` |
| Character formatting (bold/italic/underline/strike/size/color) | ✅ | `wml/char_formatting.docx` |
| Character formatting (highlight/vertAlign/caps/smallCaps) | ✅ | `wml/char_formatting.docx` |
| Run fonts (rFonts: ascii/hAnsi/eastAsia/cs/theme/hint) | ✅ | `wml/run_fonts.docx` |
| Run character style (rStyle) | ✅ | `wml/run_fonts.docx` |
| Whitespace preservation (xml:space="preserve") | ✅ | `wml/whitespace.docx` |
| Run breaks (soft return, page break, tab) | ✅ | `wml/breaks.docx` |
| Paragraph alignment (jc: left/center/right/both/distribute) | ✅ | `wml/para_alignment.docx` |
| Paragraph spacing (before/after twips; lineRule auto/exact/atLeast) | ✅ | `wml/para_spacing.docx` |
| Contextual spacing | ✅ | `wml/para_spacing.docx` |
| Paragraph indentation (left/right/firstLine/hanging) | ✅ | `wml/para_indent.docx` |
| Paragraph borders (pBdr: single/double, top/left/bottom/right) | ✅ | `wml/para_borders_shading.docx` |
| Paragraph shading (shd: clear fill, pct pattern) | ✅ | `wml/para_borders_shading.docx` |
| Keep properties (keepNext/keepLines/pageBreakBefore/widowControl) | ✅ | `wml/para_keep.docx` |
| Outline level (outlineLvl) | ✅ | `wml/para_keep.docx` |
| Styles (paragraph/character) | ✅ | `document/minimal_styles.docx` |
| Style inheritance (basedOn chain, docDefaults) | ✅ | `wml/style_inheritance.docx` |
| Linked paragraph+character styles (link/next) | ✅ | `wml/style_linked.docx` |
| Bullet lists (numFmt=bullet, hanging indent) | ✅ | `wml/numbering_bullets.docx` |
| Ordered lists (decimal/lowerLetter/lowerRoman, multi-level) | ✅ | `wml/numbering_ordered.docx` |
| List restart via lvlOverride startOverride | ✅ | `wml/numbering_restart.docx` |
| Tables (basic) | ✅ | `document/minimal_table.docx` |
| Table borders (tblBorders outer+insideH/V, tcBorders override) | ✅ | `wml/table_borders.docx` |
| Tables (horizontal merge via gridSpan) | ✅ | `wml/table_merged.docx` |
| Tables (vertical merge via vMerge restart/continue) | ✅ | `wml/table_merged.docx` |
| Table row properties (tblHeader, trHeight exact, cantSplit) | ✅ | `wml/table_props.docx` |
| Table cell vertical alignment (vAlign top/center/bottom) | ✅ | `wml/table_props.docx` |
| Table cell noWrap, pct table width | ✅ | `wml/table_props.docx` |
| Inline images | ✅ | `document/minimal_image.docx` |
| Inline images (distL/distR, altText, cstate, picLocks) | ✅ | `wml/image_inline_props.docx` |
| Floating images (wp:anchor, wrapSquare, positionH/V) | ✅ | `wml/image_floating.docx` |
| Headers and footers (default header+footer; xmlns:r; sectPr headerReference/footerReference) | ✅ | `wml/header_footer.docx` |
| Headers and footers (first-page header; titlePg; three part relationships) | ✅ | `wml/header_first_page.docx` |
| Page size (pgSz: US Letter twips; orient) | ✅ | `wml/header_footer.docx` |
| Page margins (pgMar: top/right/bottom/left/header/footer/gutter) | ✅ | `wml/header_footer.docx` |
| Footnotes (separator/continuationSeparator special notes; footnoteRef mark; footnoteReference in body) | ✅ | `wml/footnotes.docx` |
| Endnotes (same structure as footnotes; endnoteRef; endnoteReference in body) | ✅ | `wml/endnotes.docx` |
| Fields (complex: PAGE/NUMPAGES with begin/instrText/separate/end; dirty) | ✅ | `wml/fields_complex.docx` |
| Fields (simple: fldSimple with DATE instruction) | ✅ | `wml/fields_hyperlink.docx` |
| Hyperlinks (external via r:id + TargetMode=External) | ✅ | `wml/fields_hyperlink.docx` |
| Hyperlinks (internal anchor via w:anchor) | ✅ | `wml/fields_hyperlink.docx` |
| Bookmarks (bookmarkStart/End as paragraph children; inline, heading, zero-width point) | ✅ | `wml/bookmarks.docx` |
| Hyperlinks (internal anchor targeting bookmark via w:anchor) | ✅ | `wml/bookmarks.docx` |
| Section properties (cols: equal-width multi-column, continuous break) | ✅ | `wml/section_columns.docx` |
| Section properties (vAlign, docGrid, lnNumType) | ✅ | `wml/section_props.docx` |
| Tracked changes (w:ins inserted run; w:del with w:delText; w:rPrChange; w:pPrChange) | ✅ | `wml/tracked_changes.docx` |
| Comments (commentRangeStart/End as paragraph children; commentReference run child; annotationRef in note) | ✅ | `wml/comments.docx` |
| Content controls (SDT block: plain text, alias/tag/id/lock) | ✅ | `wml/content_controls.docx` |
| Content controls (SDT run: date picker with fullDate/dateFormat; dropDownList with listItems) | ✅ | `wml/content_controls.docx` |
| VBA macros (preserve-only) | 🔲 | |
| Custom XML | 🔲 | |
| Embedded objects | 🔲 | |

## SpreadsheetML (XLSX)

| Feature | Status | Notes |
|---|---|---|
| Empty workbook | ✅ | `spreadsheet/minimal_empty.xlsx` |
| String cell values | ✅ | `spreadsheet/minimal_values.xlsx` |
| Numeric cell values | ✅ | `spreadsheet/minimal_values.xlsx` |
| Boolean cell values | ✅ | `spreadsheet/minimal_values.xlsx` |
| Date cell values | ✅ | `spreadsheet/minimal_values.xlsx` |
| Shared strings table | ✅ | `spreadsheet/minimal_values.xlsx` |
| Inline strings | ✅ | `spreadsheet/minimal_multisheet.xlsx` |
| Formulas (arithmetic) | ✅ | `spreadsheet/minimal_formula.xlsx` |
| Formulas (cached values) | ✅ | `spreadsheet/minimal_formula.xlsx` |
| Named ranges | 🔲 | |
| Multiple worksheets | ✅ | `spreadsheet/minimal_multisheet.xlsx` |
| Merged cells | 🔲 | |
| Cell formatting (font) | ✅ | `spreadsheet/minimal_styles.xlsx` |
| Cell formatting (fill/color) | ✅ | `spreadsheet/minimal_styles.xlsx` |
| Cell formatting (borders) | ✅ | `spreadsheet/minimal_styles.xlsx` |
| Cell formatting (number format) | ✅ | `spreadsheet/minimal_styles.xlsx` |
| Column/row dimensions | 🔲 | |
| Conditional formatting | 🔲 | |
| Data validation | 🔲 | |
| Charts | 🔲 | |
| Pivot tables | 🔲 | |
| Defined names | 🔲 | |
| VBA macros (preserve-only) | 🔲 | |

## PresentationML (PPTX)

| Feature | Status | Notes |
|---|---|---|
| Empty presentation | ✅ | `slideshow/minimal_empty.pptx` |
| Text boxes | ✅ | `slideshow/minimal_text.pptx` |
| Slide layout / master | ✅ | `slideshow/minimal_layout.pptx` |
| Themes | ✅ | `drawingml/theme.pptx` |
| Tables | ✅ | `slideshow/minimal_table.pptx` |
| Inline images | ✅ | `slideshow/minimal_image.pptx` |
| Shapes (basic) | ✅ | `slideshow/minimal_text.pptx` |
| Shapes (with fill/border) | ✅ | `drawingml/solid_fill.pptx`, `drawingml/shape_line.pptx` |
| Charts | 🔲 | |
| Slide transitions | 🔲 | |
| Animations | 🔲 | |
| Notes | 🔲 | |
| Embedded videos | 🔲 | |
| Speaker notes | 🔲 | |

## DrawingML (shared across DOCX/XLSX/PPTX)

| Feature | Status | Notes |
|---|---|---|
| Solid fill (srgbClr) | ✅ | `drawingml/solid_fill.pptx` |
| Solid fill (schemeClr + colour transforms) | ✅ | `drawingml/solid_fill.pptx` |
| Gradient fill (linear, scheme stops) | ✅ | `drawingml/gradient_fill.pptx` |
| Shape outline (width, color, dash) | ✅ | `drawingml/shape_line.pptx` |
| Connector with arrowheads | ✅ | `drawingml/shape_line.pptx` |
| Effects (outer shadow + glow) | ✅ | `drawingml/effects.pptx` |
| Text run properties (bold/italic/underline/color/sz/typeface) | ✅ | `drawingml/text_run_props.pptx` |
| Paragraph properties (alignment/spacing/indent) | ✅ | `drawingml/text_run_props.pptx` |
| Superscript / subscript (baseline) | ✅ | `drawingml/text_run_props.pptx` |
| Theme part (clrScheme/fontScheme/fmtScheme) | ✅ | `drawingml/theme.pptx` |
| Pattern fill | 🔲 | |
| Custom geometry (custGeom) | 🔲 | |
| 3D effects (scene3d/sp3d) | 🔲 | |

## MCE (Markup Compatibility and Extensibility)

| Feature | Status | Notes |
|---|---|---|
| mc:Ignorable unknown namespace | ✅ | `mce/ignorable_unknown_ns.docx` |
| mc:ProcessContent wrapper | ✅ | `mce/process_content.docx` |
| mc:AlternateContent fallback selection | ✅ | `mce/alternate_content_fallback.docx` |
| mc:AlternateContent in PPTX parts | ✅ | `mce/alternate_content_pptx.pptx` |
| extLst with unknown child namespace | ✅ | `mce/extlst_unknown_ns.pptx` |
| mc:MustUnderstand on known namespace | ✅ | `mce/must_understand_ok.docx` |
| Nested mc:AlternateContent | ✅ | `mce/nested_alternate_content.docx` |
