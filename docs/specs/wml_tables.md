# WML Tables — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.4 (tables), ISO/IEC
29500:2016 Part 1 §17.4; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

A table in WordprocessingML is represented by `<w:tbl>`. Tables may appear
as block-level children of `<w:body>`, inside table cells (nested tables),
or inside text frames.

The three-layer structure:

```
<w:tbl>            — table container
  <w:tblPr/>       — table-level properties
  <w:tblGrid/>     — column width grid
  <w:tr>           — row
    <w:trPr/>      — row properties
    <w:tc>         — cell
      <w:tcPr/>    — cell properties
      <w:p/>       — cell content (at least one paragraph required)
    </w:tc>
  </w:tr>
</w:tbl>
```

All elements in namespace
`http://schemas.openxmlformats.org/wordprocessingml/2006/main` (prefix `w:`).

---

## 2. Table Properties: CT_TblPr

`<w:tblPr>` is the **first** child of `<w:tbl>` (required).

### Key child elements

| Element | Notes |
|---------|-------|
| `tblStyle` | Table style ID reference (matches `<w:style w:type="table">`) |
| `tblW` | Total table width (CT_TblWidth — see §8) |
| `jc` | Table horizontal alignment: `left`, `center`, `right`, `start`, `end` |
| `tblInd` | Left indent from margin (CT_TblWidth) |
| `tblBorders` | Table-level borders: outer edges and inside lines |
| `shd` | Table background shading |
| `tblLayout` | `fixed` (column widths fixed) or `autofit` |
| `tblCellMar` | Default cell margins (top/left/bottom/right) |
| `tblCellSpacing` | Space between cells |
| `tblLook` | Conditional formatting flags (header row, banded rows/columns, etc.) |
| `tblStyleRowBandSize` | Number of rows per band for row-banded styles |
| `tblStyleColBandSize` | Number of columns per band for column-banded styles |
| `bidiVisual` | Right-to-left visual layout |

---

## 3. Column Grid: CT_TblGrid

`<w:tblGrid>` is the **second** child of `<w:tbl>` (required). It defines one
`<w:gridCol>` per logical column in the table, with an explicit width in twips.

```xml
<w:tblGrid>
  <w:gridCol w:w="4320"/>   <!-- 3 inches (4320 twips) -->
  <w:gridCol w:w="4320"/>
</w:tblGrid>
```

The grid column count determines how many cells a row spans. A row with fewer
`<w:tc>` elements than grid columns must account for the remaining columns via
`gridSpan` on its cells, or via `gridAfter` in `<w:trPr>`.

**Grid column width (`w:w`):** In twips. Sum of all `gridCol` widths should
equal the table's effective content width.

---

## 4. Table Row: CT_Tr

Each `<w:tr>` represents one row. Children, in order:

1. `<w:tblPrEx>` — table-level property exceptions (rare; overrides tblPr for this row only)
2. `<w:trPr>` — row properties
3. One or more `<w:tc>` (or custom XML / SDT wrappers around cells)

### Row attributes

| Attribute | Notes |
|-----------|-------|
| `rsidR` | Revision ID for row addition |
| `rsidRPr` | Revision ID for row formatting change |
| `rsidDel` | Revision ID for row deletion |
| `rsidTr` | Revision ID for row properties |

### CT_TrPr — row property elements

| Element | Notes |
|---------|-------|
| `trHeight` | Row height. Attribute `w:val` = height in twips; `w:hRule`: `auto`, `atLeast`, `exact` |
| `tblHeader` | Repeat this row as a header on each page |
| `cantSplit` | Prevent row from breaking across pages |
| `jc` | Row-level horizontal alignment override |
| `tblCellSpacing` | Row-level cell spacing override |
| `gridBefore` | Skip N grid columns before the first cell |
| `gridAfter` | Skip N grid columns after the last cell |

---

## 5. Table Cell: CT_Tc

Each `<w:tc>` contains:

1. `<w:tcPr>` (optional) — cell properties
2. One or more block-level elements — **required**. Every cell must contain
   at least one `<w:p>`. A cell with no text still needs an empty `<w:p/>`.

```xml
<w:tc>
  <w:tcPr>…</w:tcPr>
  <w:p><w:r><w:t>Cell text</w:t></w:r></w:p>
</w:tc>
```

---

## 6. Cell Properties: CT_TcPr

### Key child elements

| Element | Notes |
|---------|-------|
| `tcW` | Cell width (CT_TblWidth) |
| `gridSpan` | Number of grid columns this cell spans (horizontal merge) |
| `vMerge` | Vertical merge: `restart` = first cell; absent val or `continue` = continuation |
| `hMerge` | Horizontal merge alternative (legacy; `gridSpan` is preferred) |
| `tcBorders` | Individual cell borders (overrides table borders) |
| `shd` | Cell background shading |
| `noWrap` | Prevent text from wrapping |
| `tcMar` | Cell-specific margin overrides |
| `vAlign` | Vertical text alignment: `top`, `center`, `bottom` |
| `textDirection` | Text direction |
| `hideMark` | Hide the paragraph mark character in the cell |

**Known generator limitation:** `ST_VerticalJc` value `"both"` (vertical
justification) is not represented in the generated `TableVerticalAlignmentValues`
enum. Documents using `<w:vAlign w:val="both"/>` cannot represent that value in
the typed field.

---

## 7. Width Model: CT_TblWidth

Used for table width, cell width, indentation, and cell spacing.

```xml
<w:tblW w:w="8640" w:type="dxa"/>
<w:tcW w:w="4320" w:type="dxa"/>
<w:tblW w:w="5000" w:type="pct"/>   <!-- 50.00% -->
<w:tblW w:w="0"    w:type="auto"/>
```

### ST_TblWidth values

| Value | Unit for `w:w` | Notes |
|-------|----------------|-------|
| `dxa` | Twips (1/1440 inch) | Most common; explicit measurement |
| `pct` | Fiftieths of a percent | `5000` = 100%, `2500` = 50% |
| `auto` | Ignored | Auto-fit to content or container |
| `nil` | Ignored | No width specified |

**Note:** When `type="pct"`, the `w` value is in **fiftieths of a percent**,
not hundredths — `5000` means 100%, `2500` means 50%, `1000` means 20%.

---

## 8. Merge Model

### Horizontal merge (column span)

Use `<w:gridSpan w:val="N"/>` in the spanning cell's `<w:tcPr>`. The row must
contain **fewer** `<w:tc>` elements than the grid has columns — the spanning
cell accounts for N grid columns, and the row's remaining cells account for the
rest. There are **no placeholder cells** for the spanned columns.

```
Grid:   [col1]  [col2]  [col3]
Row:    [  TC spanning 2  ] [TC]
tcPr:   gridSpan=2             gridSpan absent (=1)
```

```xml
<w:tr>
  <w:tc>
    <w:tcPr><w:tcW w:w="2880" w:type="dxa"/><w:gridSpan w:val="2"/></w:tcPr>
    <w:p><w:r><w:t>Spans two columns</w:t></w:r></w:p>
  </w:tc>
  <w:tc>
    <w:tcPr><w:tcW w:w="1440" w:type="dxa"/></w:tcPr>
    <w:p><w:r><w:t>Single column</w:t></w:r></w:p>
  </w:tc>
</w:tr>
```

### Vertical merge (row span)

Use `<w:vMerge w:val="restart"/>` on the **top** cell of the merged block, and
`<w:vMerge/>` (or `w:val="continue"`) on each continuation cell below it.

Every row in the merge block must still have a `<w:tc>` at that grid position —
the continuation cells are present in the XML but display as merged. The
continuation cell must still contain at least one `<w:p/>`.

```
Row 1: [vMerge=restart] [normal]
Row 2: [vMerge=continue] [normal]   ← still needs a <w:tc> with <w:p/>
```

```xml
<!-- Row 1 -->
<w:tr>
  <w:tc>
    <w:tcPr><w:tcW w:w="4320" w:type="dxa"/><w:vMerge w:val="restart"/></w:tcPr>
    <w:p><w:r><w:t>Top of merged cell</w:t></w:r></w:p>
  </w:tc>
  <w:tc>…</w:tc>
</w:tr>
<!-- Row 2 -->
<w:tr>
  <w:tc>
    <w:tcPr><w:tcW w:w="4320" w:type="dxa"/><w:vMerge/></w:tcPr>
    <w:p/>   <!-- empty paragraph — required -->
  </w:tc>
  <w:tc>…</w:tc>
</w:tr>
```

---

## 9. Border Hierarchy

Table borders resolve at two levels: table-level (`<w:tblBorders>`) and
cell-level (`<w:tcBorders>`). Cell-level always wins.

### CT_TblBorders elements

| Element | Applies to |
|---------|-----------|
| `top` | Top outer edge |
| `left` | Left outer edge |
| `bottom` | Bottom outer edge |
| `right` | Right outer edge |
| `insideH` | All horizontal inside lines (between rows) |
| `insideV` | All vertical inside lines (between columns) |

### CT_TcBorders elements

All the above plus:

| Element | Applies to |
|---------|-----------|
| `tl2br` | Top-left to bottom-right diagonal |
| `tr2bl` | Top-right to bottom-left diagonal |

### Border val="nil" vs absent

`<w:top w:val="nil"/>` explicitly removes a border that would otherwise be
inherited from the table style. An absent `<w:top>` means "inherit from the
next level up". Do not confuse `nil` with `none` (`none` = no border but
does not override inheritance).

---

## 10. Cell Width and `gridSpan` Relationship

When a cell spans N columns (`gridSpan=N`), its `<w:tcW>` should equal the
**sum** of the N spanned grid column widths:

```
tcW.w = gridCol[i].w + gridCol[i+1].w + … + gridCol[i+N-1].w
```

This is a strong convention, not a hard rule — applications accept discrepancies
but may lay out the table differently.

---

## 11. Round-Trip Gotchas

1. **Every cell needs at least one `<w:p>`.** A `<w:tc>` with no block-level
   children is invalid. Continuation merge cells with no visible content still
   need an empty `<w:p/>`.

2. **`vMerge` with no `val` attribute = "continue".** The XSD default for
   the `val` attribute is `continue`. Round-tripping must not add `w:val="continue"`
   when it was absent, and must not remove it when it was present.

3. **Row cell count ≠ grid column count when merging.** Do not expect one `<w:tc>`
   per `<w:gridCol>`. Always count grid positions consumed (accounting for
   `gridSpan`) to find the matching grid column.

4. **`tblW type="auto"` with `w="0"`.** The canonical "auto-width" encoding
   is `<w:tblW w:w="0" w:type="auto"/>`. The `w` value is meaningless when
   `type="auto"` but must still be present and round-tripped.

5. **`tcW` is redundant with the grid but must be preserved.** Some producers
   omit `tcW` on cells — this is valid. Others always write it. Round-tripping
   must not add `tcW` where it was absent.

6. **`tblGrid` is required.** A table without `<w:tblGrid>` is technically
   invalid. Some producers emit `<w:tblGrid/>` (empty, no `gridCol` children)
   for auto-width tables. The parser must accept both forms.

7. **`tblPr` is required.** A table without `<w:tblPr>` is invalid even though
   all `tblPr` children are optional. In practice, at minimum
   `<w:tblPr><w:tblW w:w="0" w:type="auto"/></w:tblPr>` is present.

8. **Percentage widths: fiftieths, not hundredths.** `pct` type uses 1/50th of
   a percent, so 100% = 5000, 50% = 2500, 25% = 1250. A common mistake is to
   write 100 for 100%.

9. **`tblBorders insideH/insideV` apply between cells, not at the edges.**
   To create a full box + grid, set all six borders (top, left, bottom, right,
   insideH, insideV).

10. **`trHeight` `hRule` default is `auto`.** Omitting `hRule` means the row
    height is a minimum (at least `val` twips). Use `hRule="exact"` to fix
    the height.

---

## 12. Minimal Structures

### 3×2 table with full borders

```xml
<w:tbl>
  <w:tblPr>
    <w:tblW w:w="0" w:type="auto"/>
    <w:tblBorders>
      <w:top    w:val="single" w:sz="4" w:space="0" w:color="000000"/>
      <w:left   w:val="single" w:sz="4" w:space="0" w:color="000000"/>
      <w:bottom w:val="single" w:sz="4" w:space="0" w:color="000000"/>
      <w:right  w:val="single" w:sz="4" w:space="0" w:color="000000"/>
      <w:insideH w:val="single" w:sz="4" w:space="0" w:color="000000"/>
      <w:insideV w:val="single" w:sz="4" w:space="0" w:color="000000"/>
    </w:tblBorders>
  </w:tblPr>
  <w:tblGrid>
    <w:gridCol w:w="2880"/>
    <w:gridCol w:w="2880"/>
    <w:gridCol w:w="2880"/>
  </w:tblGrid>
  <w:tr>
    <w:tc><w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr><w:p><w:r><w:t>A1</w:t></w:r></w:p></w:tc>
    <w:tc><w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr><w:p><w:r><w:t>A2</w:t></w:r></w:p></w:tc>
    <w:tc><w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr><w:p><w:r><w:t>A3</w:t></w:r></w:p></w:tc>
  </w:tr>
  <w:tr>
    <w:tc><w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr><w:p><w:r><w:t>B1</w:t></w:r></w:p></w:tc>
    <w:tc><w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr><w:p><w:r><w:t>B2</w:t></w:r></w:p></w:tc>
    <w:tc><w:tcPr><w:tcW w:w="2880" w:type="dxa"/></w:tcPr><w:p><w:r><w:t>B3</w:t></w:r></w:p></w:tc>
  </w:tr>
</w:tbl>
```

---

## 13. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-T-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/table_borders.docx` | tblBorders (outer + insideH/V), tcBorders overrides, cell shading, dxa widths |
| WML-T-02 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/table_merged.docx` | gridSpan (horizontal merge), vMerge restart/continue (vertical merge) |
| WML-T-03 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/table_props.docx` | trHeight exact, tblHeader, cantSplit, vAlign top/center/bottom, noWrap, pct table width |
