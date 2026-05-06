# SpreadsheetML Worksheet Layout — Clean-Room Specification
## Merged Cells, Row/Column Dimensions, Freeze Panes, Outline Groups, Hyperlinks

**Purpose:** This document is a clean-room synthesis of the SpreadsheetML
worksheet layout model as defined in ECMA-376 Part 1 (5th edition) and
ISO/IEC 29500-1:2016. It covers merged cell ranges, row height and column width
overrides, default dimension settings, freeze/split pane configuration,
outline grouping, and hyperlinks. It is intended as an implementation guide and
test oracle for `ooxmlsdk` and similar Rust OOXML parsers.

**Sources used:**
- ECMA-376 Part 1, 5th edition — §18.3 (Worksheets)
- ISO/IEC 29500-1:2016 — cross-reference for worksheet layout semantics
- `sml.xsd` from this repository
- Microsoft Open Specifications: `[MS-XLSX]`
- Apache POI and openpyxl source code (reference implementations)

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or have been clearly
rewritten.

---

## 1. Worksheet Child Element Order

A `<x:worksheet>` element's children appear in strict schema-defined order.
The most commonly used subset (from `CT_Worksheet`):

1. `<x:sheetPr>` — sheet properties
2. `<x:dimension>` — used-range hint
3. `<x:sheetViews>` — view configuration (including freeze panes)
4. `<x:sheetFormatPr>` — default row/column dimensions
5. `<x:cols>` — column width overrides (zero or more)
6. `<x:sheetData>` — cell data (required)
7. `<x:sheetCalcPr>` — sheet calculation properties
8. `<x:sheetProtection>` — protection settings
9. `<x:protectedRanges>` — protected range list
10. `<x:scenarios>` — what-if scenarios
11. `<x:autoFilter>` — auto-filter settings
12. `<x:sortState>` — sort state
13. `<x:dataConsolidate>` — data consolidation
14. `<x:customSheetViews>` — custom sheet views
15. `<x:mergeCells>` — merged cell ranges
16. `<x:phoneticPr>` — phonetic properties
17. `<x:conditionalFormatting>` — conditional formatting rules
18. `<x:dataValidations>` — data validation rules
19. `<x:hyperlinks>` — hyperlinks
20. `<x:printOptions>` — print options
21. `<x:pageMargins>` — page margins
22. `<x:pageSetup>` — page setup
23. `<x:headerFooter>` — header/footer
24. `<x:rowBreaks>` — horizontal page breaks
25. `<x:colBreaks>` — vertical page breaks
26. `<x:drawing>` — drawing object reference

---

## 2. Merged Cells

### 2.1 `<x:mergeCells>` Container

Schema type `CT_MergeCells`. Located after `<x:sheetData>` (and after
`<x:customSheetViews>`) in the worksheet child order.

| Attribute | Description |
|---|---|
| `count` | Number of `<x:mergeCell>` entries (optional optimisation hint) |

### 2.2 `<x:mergeCell>` Entry (MergeCell / CT_MergeCell)

Attribute: `ref` — the merged range in A1 notation (e.g. `A1:C3`).

The cell at the top-left corner of the merge range is the "master cell" and
carries the value; all other cells in the range must exist as blank cells
(or may be absent) in `<x:sheetData>`.

```xml
<x:mergeCells count="2">
  <x:mergeCell ref="A1:C1"/>   <!-- three columns merged in row 1 -->
  <x:mergeCell ref="A3:B4"/>   <!-- 2×2 merge starting at A3 -->
</x:mergeCells>
```

**Rules:**
- Merge ranges MUST NOT overlap.
- A cell inside a merge range (but not the top-left) should be blank.
- Merging does not affect the `<x:row>` or `<x:c>` elements — those must still
  be consistent with the grid.

---

## 3. Default Dimensions

### 3.1 `<x:sheetFormatPr>` (SheetFormatProperties / CT_SheetFormatPr)

Attribute serialisation order (schema-defined):

| Attribute | Type | Description |
|---|---|---|
| `baseColWidth` | `xsd:unsignedInt` | Base column width in characters (default 8) |
| `defaultColWidth` | `xsd:double` | Default column width including padding |
| `defaultRowHeight` | `xsd:double` | Default row height in points (required attribute) |
| `customHeight` | `xsd:boolean` | Whether `defaultRowHeight` overrides the calculated default |
| `zeroHeight` | `xsd:boolean` | All rows are hidden by default |
| `thickTop` | `xsd:boolean` | Rows have thick top border by default |
| `thickBottom` | `xsd:boolean` | Rows have thick bottom border by default |
| `outlineLevelRow` | `xsd:unsignedByte` | Maximum outline level used in rows (0–7) |
| `outlineLevelCol` | `xsd:unsignedByte` | Maximum outline level used in columns (0–7) |

```xml
<x:sheetFormatPr defaultColWidth="8" defaultRowHeight="15"/>
```

---

## 4. Column Dimensions

### 4.1 `<x:cols>` Container

Each `<x:cols>` element contains one or more `<x:col>` entries. Multiple
`<x:cols>` elements are permitted (though a single one is typical).

### 4.2 `<x:col>` — Column Override (Column / CT_Col)

Attribute serialisation order (schema-defined):

| Attribute | Type | Required | Description |
|---|---|---|---|
| `min` | `xsd:unsignedInt` | Yes | First column index (1-based) |
| `max` | `xsd:unsignedInt` | Yes | Last column index (1-based, inclusive) |
| `width` | `xsd:double` | No | Column width in character units |
| `style` | `xsd:unsignedInt` | No | Default cell style index for this column |
| `hidden` | `xsd:boolean` | No | Column is hidden |
| `bestFit` | `xsd:boolean` | No | Width was auto-fitted to content |
| `customWidth` | `xsd:boolean` | No | Width was manually set |
| `phonetic` | `xsd:boolean` | No | Show phonetic annotations |
| `outlineLevel` | `xsd:unsignedByte` | No | Outline grouping level (0–7) |
| `collapsed` | `xsd:boolean` | No | Outline group is collapsed |

```xml
<x:cols>
  <x:col min="1" max="1" width="20" customWidth="1"/>
  <x:col min="2" max="3" width="10" customWidth="1"/>
  <x:col min="4" max="4" hidden="1" width="8"/>
</x:cols>
```

**Column width units:** The `width` attribute is measured in character units —
specifically the average width of the widest digit (0–9) in the normal style's
font. A width of `8` means approximately 8 average digit widths.

---

## 5. Row Dimensions

Row dimension overrides are applied via attributes on the `<x:row>` element
(see `sml_cells.md §3.1` for the full attribute list). The key layout-related
attributes are:

| Attribute | Description |
|---|---|
| `ht` | Custom row height in points |
| `customHeight` | Enables the `ht` override |
| `hidden` | Hides the row |
| `outlineLevel` | Outline grouping level (0–7) |
| `collapsed` | Outline group is collapsed |

```xml
<x:row r="1" ht="30" customHeight="1">
  <x:c r="A1"><x:v>Tall row</x:v></x:c>
</x:row>
<x:row r="2" hidden="1">
  <x:c r="A2"><x:v>Hidden row</x:v></x:c>
</x:row>
```

---

## 6. Outline Groups

Rows and columns can be grouped into collapsible outline levels. Excel
supports up to 7 levels (1–7; level 0 means no grouping).

### 6.1 Column Outline Groups

Set `outlineLevel` (1–7) on `<x:col>` entries. The group button appears above
the column headers. Set `collapsed="1"` to render the group as collapsed.

### 6.2 Row Outline Groups

Set `outlineLevel` (1–7) on `<x:row>` entries. The group button appears to the
left of row numbers. Set `collapsed="1"` to render collapsed.

The `<x:sheetFormatPr>` attributes `outlineLevelRow` and `outlineLevelCol`
record the maximum outline level used (optimisation hint for readers).

---

## 7. Freeze Panes

### 7.1 `<x:sheetViews>` and `<x:sheetView>`

The `<x:sheetViews>` element contains one or more `<x:sheetView>` elements.
Each `<x:sheetView>` represents one view of the sheet (most sheets have one).

`<x:sheetView>` required attribute: `workbookViewId` — zero-based index into
the workbook's `<x:bookViews>`.

### 7.2 `<x:pane>` — Pane Configuration (Pane / CT_Pane)

The optional `<x:pane>` child of `<x:sheetView>` configures frozen or split
panes.

Attribute serialisation order (schema-defined):

| Attribute | Type | Description |
|---|---|---|
| `xSplit` | `xsd:double` | Horizontal position: column index (frozen) or pixel offset (split) |
| `ySplit` | `xsd:double` | Vertical position: row index (frozen) or pixel offset (split) |
| `topLeftCell` | `ST_CellRef` | First visible cell in the bottom-right pane |
| `activePane` | `ST_Pane` | Which pane is currently active |
| `state` | `ST_PaneState` | `frozen`, `frozenSplit`, or `split` |

**For frozen panes:** `xSplit` and `ySplit` are column and row counts
(e.g., `xSplit="1"` freezes the first column, `ySplit="1"` freezes the first row).

### 7.3 PaneValues (ST_Pane)

| Value | Description |
|---|---|
| `topLeft` | Top-left pane |
| `topRight` | Top-right pane (appears when columns are frozen) |
| `bottomLeft` | Bottom-left pane (appears when rows are frozen) |
| `bottomRight` | Bottom-right pane (appears when both rows and columns are frozen) |

### 7.4 PaneStateValues (ST_PaneState)

| Value | Description |
|---|---|
| `split` | Pane is a moveable split (not frozen) |
| `frozen` | Pane is frozen |
| `frozenSplit` | Pane was frozen and then split (legacy) |

### 7.5 `<x:selection>` — Active Cell in Pane (Selection / CT_Selection)

After `<x:pane>`, zero or more `<x:selection>` elements specify the active
cell and selection in each pane.

Attribute serialisation order (schema-defined):

| Attribute | Type | Description |
|---|---|---|
| `pane` | `ST_Pane` | Which pane this selection applies to |
| `activeCell` | `ST_CellRef` | The active (focused) cell |
| `activeCellId` | `xsd:unsignedInt` | Index of active cell within sqref |
| `sqref` | `ST_Sqref` | Selected cell range(s) as space-separated list |

### 7.6 Freeze Panes Example

**Freeze first row and first column (1×1 freeze):**

```xml
<x:sheetViews>
  <x:sheetView workbookViewId="0">
    <x:pane xSplit="1" ySplit="1" topLeftCell="B2" activePane="bottomRight"
            state="frozen"/>
    <x:selection pane="topLeft" activeCell="A1" sqref="A1"/>
    <x:selection pane="topRight" activeCell="B1" sqref="B1"/>
    <x:selection pane="bottomLeft" activeCell="A2" sqref="A2"/>
    <x:selection pane="bottomRight" activeCell="B2" sqref="B2"/>
  </x:sheetView>
</x:sheetViews>
```

When both `xSplit` and `ySplit` are set:
- `topLeft` pane: rows ≤ ySplit, columns ≤ xSplit (the frozen rows+columns)
- `topRight` pane: rows ≤ ySplit, columns > xSplit
- `bottomLeft` pane: rows > ySplit, columns ≤ xSplit
- `bottomRight` pane: rows > ySplit, columns > xSplit (the scrollable area)

`topLeftCell` is the top-left cell of the `bottomRight` pane after freezing;
it should be one row and one column past the freeze boundary.

---

## 8. Hyperlinks

### 8.1 `<x:hyperlinks>` Container

Located after `<x:dataValidations>` in the worksheet child order. Contains
one or more `<x:hyperlink>` elements.

### 8.2 `<x:hyperlink>` Attributes (Hyperlink / CT_Hyperlink)

Attribute serialisation order (schema-defined):

| Attribute | Type | Description |
|---|---|---|
| `ref` | `ST_Ref` | Cell reference the hyperlink is attached to |
| `r:id` | `xsd:string` | Relationship ID for external URL |
| `location` | `xsd:string` | In-workbook target (sheet name + cell ref) |
| `tooltip` | `xsd:string` | Tooltip text shown on hover |
| `display` | `xsd:string` | Display text override |

### 8.3 External Hyperlinks

External hyperlinks (URLs) use an OPC relationship of type:
`http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink`

The relationship is in `xl/worksheets/_rels/sheet1.xml.rels`:

```xml
<Relationship Id="rId1"
  Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
  Target="https://example.com"
  TargetMode="External"/>
```

The worksheet's `<x:hyperlink>` element references this via `r:id="rId1"`:

```xml
<x:hyperlinks>
  <x:hyperlink ref="A1" r:id="rId1" tooltip="Visit example.com"/>
</x:hyperlinks>
```

**Note:** The `TargetMode="External"` attribute is required for external URLs.
Internal hyperlinks (within the workbook, e.g., to a named range or cell) use
the `location` attribute and do not require a relationship.

---

## 9. Common Failure Modes

### 9.1 Wrong `topLeftCell` for Frozen Panes

**Failure:** Setting `topLeftCell` to the freeze boundary itself (e.g.
`topLeftCell="A1"` for a 1×1 freeze) instead of the first cell in the
scrollable pane.

**Impact:** The view snaps back to the top-left, making the freeze appear
non-functional on re-open.

**Fix:** `topLeftCell` should be one row down and one column right of the freeze
point. For `xSplit="1" ySplit="1"`, `topLeftCell="B2"`.

### 9.2 Missing `<x:selection>` After `<x:pane>`

**Failure:** Including `<x:pane>` without any `<x:selection>` children.

**Impact:** Some consumers do not restore the active cell correctly on re-open.
Excel adds selection elements automatically.

**Fix:** Emit one `<x:selection>` per pane, specifying `pane`, `activeCell`,
and `sqref`.

### 9.3 `<x:mergeCells>` Before `<x:sheetData>`

**Failure:** Placing the `<x:mergeCells>` element before `<x:sheetData>` in
the worksheet XML.

**Impact:** Schema-invalid document. The schema requires `<x:mergeCells>` after
`<x:sheetData>` (specifically after `<x:customSheetViews>`).

**Fix:** Follow the strict child ordering in §1. In summary:
`sheetViews` → `sheetFormatPr` → `cols` → `sheetData` → `mergeCells`.

### 9.4 Incorrect Column Width Units

**Failure:** Emitting column width as pixels instead of character units.

**Impact:** Columns are either much too narrow or much too wide.

**Fix:** Column width is measured in average character widths of the normal
style's font, not in pixels. A reasonable default is `8` (character units),
which corresponds roughly to 64 pixels at 96 DPI with an 8-pt font.

---

*End of SpreadsheetML Worksheet Layout Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
