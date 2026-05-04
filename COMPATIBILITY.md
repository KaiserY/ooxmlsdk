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
| Character formatting (bold/italic/underline) | 🔲 | |
| Paragraph formatting (alignment/spacing/indent) | 🔲 | |
| Styles (paragraph/character) | ✅ | `document/minimal_styles.docx` |
| Numbered and bulleted lists | 🔲 | |
| Tables (basic) | ✅ | `document/minimal_table.docx` |
| Tables (merged cells) | 🔲 | |
| Inline images | ✅ | `document/minimal_image.docx` |
| Floating images | 🔲 | |
| Headers and footers | 🔲 | |
| Footnotes and endnotes | 🔲 | |
| Hyperlinks | 🔲 | |
| Bookmarks | 🔲 | |
| Fields (PAGE, TOC, REF) | 🔲 | |
| Section properties | 🔲 | |
| Tracked changes | 🔲 | |
| Comments | 🔲 | |
| Content controls (SDT) | 🔲 | |
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
