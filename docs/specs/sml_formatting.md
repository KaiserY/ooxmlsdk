# SpreadsheetML Cell Formatting — Clean-Room Specification
## StyleSheet Part, Fonts, Fills, Borders, Number Formats, Alignment

**Purpose:** This document is a clean-room synthesis of the SpreadsheetML
formatting model as defined in ECMA-376 Part 1 (5th edition) and ISO/IEC
29500-1:2016. It covers the `xl/styles.xml` part — the XF (cell format) index
chain, font properties, fill patterns, border styles, number format codes,
alignment, and the apply-flag semantics. It is intended as an implementation
guide and test oracle for `ooxmlsdk` and similar Rust OOXML parsers.

**Sources used:**
- ECMA-376 Part 1, 5th edition — §18.8 (Styles)
- ISO/IEC 29500-1:2016 — cross-reference for styling semantics
- `sml.xsd` from this repository
- Microsoft Open Specifications: `[MS-XLSX]`
- Apache POI and openpyxl source code (reference implementations)
- OpenDocument Format specification for comparative number-format semantics

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or have been clearly
rewritten.

---

## 1. StyleSheet Part Overview

### 1.1 Part Location

The styles part is stored at `xl/styles.xml` and related from the workbook via:
- Relationship type: `http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles`
- Content type: `application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml`

### 1.2 `<x:styleSheet>` Root Element (Stylesheet / CT_Stylesheet)

Children appear in strict schema-defined order:

1. `<x:numFmts>` — custom number format definitions (optional)
2. `<x:fonts>` — font definitions (optional but strongly recommended)
3. `<x:fills>` — fill (background) definitions (optional but strongly recommended)
4. `<x:borders>` — border definitions (optional but strongly recommended)
5. `<x:cellStyleXfs>` — master XF table for named styles (optional)
6. `<x:cellXfs>` — cell XF table referenced by cell `s` attribute (optional)
7. `<x:cellStyles>` — named style definitions (optional)
8. `<x:dxfs>` — differential format records (optional)
9. `<x:tableStyles>` — table styles (optional)
10. `<x:colors>` — color customisation (optional)
11. `<x:extLst>` — extension list (optional)

---

## 2. XF Index Chain

### 2.1 How a Cell References Formatting

A cell's `s` attribute holds a zero-based index into `<x:cellXfs>`. Each entry
in `<x:cellXfs>` is an `<x:xf>` element that combines:
- `numFmtId` — number format
- `fontId` — index into `<x:fonts>`
- `fillId` — index into `<x:fills>`
- `borderId` — index into `<x:borders>`
- `xfId` — index into `<x:cellStyleXfs>` (the master style this cell extends)
- Optional `<x:alignment>` for text positioning
- Optional `<x:protection>` for locked/hidden state

### 2.2 `<x:cellStyleXfs>` vs `<x:cellXfs>`

- **`<x:cellStyleXfs>`** holds master XF records for named cell styles
  (e.g. "Normal", "Heading 1"). Cell-level XF records extend these masters.
- **`<x:cellXfs>`** holds the XF records actually referenced by cells via
  the `s` attribute.

A cell XF's `xfId` points to the master XF that it extends. Properties
with `apply*="1"` override the master; those without the flag inherit from
the master.

### 2.3 `<x:xf>` — Cell Format Record (CellFormat / CT_Xf)

Attribute serialisation order (schema-defined):

| Attribute | Description |
|---|---|
| `numFmtId` | 0-based index into built-in formats or `<x:numFmts>` |
| `fontId` | 0-based index into `<x:fonts>` |
| `fillId` | 0-based index into `<x:fills>` |
| `borderId` | 0-based index into `<x:borders>` |
| `xfId` | 0-based index into `<x:cellStyleXfs>` (only in `<x:cellXfs>`) |
| `quotePrefix` | Treat cell value as text (prepend `'`) |
| `pivotButton` | Cell is a pivot button |
| `applyNumberFormat` | Override master's number format |
| `applyFont` | Override master's font |
| `applyFill` | Override master's fill |
| `applyBorder` | Override master's border |
| `applyAlignment` | Apply `<x:alignment>` child |
| `applyProtection` | Apply `<x:protection>` child |

---

## 3. `<x:fonts>` — Font Definitions

### 3.1 `<x:font>` Structure (Font / CT_Font)

Each `<x:font>` element defines one complete font. Children appear in strict
schema-defined order:

| Element | Description |
|---|---|
| `<x:b/>` | Bold |
| `<x:i/>` | Italic |
| `<x:strike/>` | Strikethrough |
| `<x:condense/>` | Condense (legacy) |
| `<x:extend/>` | Extend (legacy) |
| `<x:outline/>` | Outline effect |
| `<x:shadow/>` | Shadow effect |
| `<x:u val="..."/>` | Underline style |
| `<x:vertAlign val="..."/>` | Superscript / subscript / baseline |
| `<x:sz val="..."/>` | Font size in points |
| `<x:color .../>` | Font color |
| `<x:name val="..."/>` | Font family name |
| `<x:family val="..."/>` | Font family numeric class (0–5) |
| `<x:charset val="..."/>` | Character set |
| `<x:scheme val="..."/>` | Font scheme: `none`, `major`, `minor` |

**Minimal font definition:**
```xml
<x:font>
  <x:sz val="11"/>
  <x:name val="Calibri"/>
</x:font>
```

**Bold font:**
```xml
<x:font>
  <x:b/>
  <x:sz val="11"/>
  <x:name val="Calibri"/>
</x:font>
```

The first two entries in `<x:fills>` (indices 0 and 1) and the first entry
in each table are required to be the defaults, by convention:
- `fillId=0`: none (no fill)
- `fillId=1`: gray125 (Excel compatibility fill)
- `fontId=0`: the document default font

### 3.2 `<x:u>` — Underline

The `val` attribute takes `ST_UnderlineValues`: `single`, `double`,
`singleAccounting`, `doubleAccounting`, `none`.

### 3.3 `<x:vertAlign>` — Vertical Alignment

The `val` attribute takes `ST_VerticalAlignRun`: `baseline`, `superscript`,
`subscript`.

### 3.4 `<x:color>` — Color Element (Color / CT_Color)

Attribute serialisation order: `auto`, `indexed`, `rgb`, `theme`, `tint`.

| Attribute | Description |
|---|---|
| `auto` | `xsd:boolean` — automatic color (inherits) |
| `indexed` | Color palette index (legacy) |
| `rgb` | ARGB hex string: 4 bytes, e.g. `FFFF0000` (opaque red) |
| `theme` | Theme color slot index (0-based) |
| `tint` | Tint/shade adjustment (−1.0 to +1.0) |

The `rgb` value is 4 bytes (8 hex digits): first byte is alpha (opacity),
`FF` = fully opaque. The remaining three bytes are RGB. Example:
- `FF000000` — opaque black
- `FFFF0000` — opaque red
- `FF0000FF` — opaque blue

---

## 4. `<x:fills>` — Fill Definitions

### 4.1 `<x:fill>` Structure (Fill / CT_Fill)

Each `<x:fill>` contains exactly one child, either `<x:patternFill>` or
`<x:gradientFill>`.

### 4.2 `<x:patternFill>` (PatternFill / CT_PatternFill)

Attribute: `patternType` (ST_PatternType).

Children (in order):
1. `<x:fgColor>` — foreground/pattern color
2. `<x:bgColor>` — background color

**Common `patternType` values:**

| Value | Description |
|---|---|
| `none` | No fill (transparent) |
| `solid` | Solid fill using `<x:fgColor>` |
| `gray125` | 12.5% gray pattern (required as fills index 1) |
| `gray0625` | 6.25% gray |
| `darkGray`, `mediumGray`, `lightGray` | Gray shading levels |
| `darkHorizontal`, `darkVertical`, `darkDown`, `darkUp`, `darkGrid`, `darkTrellis` | Dark pattern fills |
| `lightHorizontal`, `lightVertical`, `lightDown`, `lightUp`, `lightGrid`, `lightTrellis` | Light pattern fills |

**Solid fill example:**
```xml
<x:fill>
  <x:patternFill patternType="solid">
    <x:fgColor rgb="FFADD8E6"/>   <!-- light blue -->
    <x:bgColor indexed="64"/>      <!-- default background index -->
  </x:patternFill>
</x:fill>
```

---

## 5. `<x:borders>` — Border Definitions

### 5.1 `<x:border>` Structure (Border / CT_Border)

Attributes on `<x:border>`:
- `diagonalUp` — show diagonal from bottom-left to top-right
- `diagonalDown` — show diagonal from top-left to bottom-right
- `outline` — apply border to outline (defaults to `true` for cells)

Children appear in this schema-defined order:

1. `<x:start>` — logical start border (Office 2010+, equivalent to left in LTR)
2. `<x:end>` — logical end border (Office 2010+)
3. `<x:left>` — left border
4. `<x:right>` — right border
5. `<x:top>` — top border
6. `<x:bottom>` — bottom border
7. `<x:diagonal>` — diagonal border
8. `<x:vertical>` — inner vertical border (for table use)
9. `<x:horizontal>` — inner horizontal border (for table use)

Each border side element (`CT_BorderPr`) has:
- `style` attribute (`ST_BorderStyle`) — border line style
- `<x:color>` child — border color

**`ST_BorderStyle` values:**

| Value | Description |
|---|---|
| `none` | No border |
| `thin` | Thin single line |
| `medium` | Medium single line |
| `thick` | Thick single line |
| `dashed` | Dashed line |
| `dotted` | Dotted line |
| `double` | Double line |
| `hair` | Hairline (thinnest possible) |
| `mediumDashed` | Medium dashed |
| `dashDot` | Dash-dot pattern |
| `mediumDashDot` | Medium dash-dot |
| `dashDotDot` | Dash-dot-dot pattern |
| `mediumDashDotDot` | Medium dash-dot-dot |
| `slantDashDot` | Slanted dash-dot |

**Required empty border (index 0):**
```xml
<x:border>
  <x:left/>
  <x:right/>
  <x:top/>
  <x:bottom/>
  <x:diagonal/>
</x:border>
```

**Thin black border:**
```xml
<x:border>
  <x:left style="thin"><x:color rgb="FF000000"/></x:left>
  <x:right style="thin"><x:color rgb="FF000000"/></x:right>
  <x:top style="thin"><x:color rgb="FF000000"/></x:top>
  <x:bottom style="thin"><x:color rgb="FF000000"/></x:bottom>
  <x:diagonal/>
</x:border>
```

---

## 6. Number Formats

### 6.1 Built-in Number Format IDs

The following IDs (0–49) are pre-defined by the spec and do not require a
`<x:numFmt>` entry:

| ID | Format code | Example output |
|---|---|---|
| 0 | General | (general) |
| 1 | `0` | 42 |
| 2 | `0.00` | 42.00 |
| 3 | `#,##0` | 1,234 |
| 4 | `#,##0.00` | 1,234.56 |
| 9 | `0%` | 42% |
| 10 | `0.00%` | 42.00% |
| 11 | `0.00E+00` | 4.20E+01 |
| 12 | `# ?/?` | fraction |
| 13 | `# ??/??` | fraction |
| 14 | `mm-dd-yy` | date |
| 15 | `d-mmm-yy` | date |
| 16 | `d-mmm` | date |
| 17 | `mmm-yy` | date |
| 18 | `h:mm AM/PM` | time |
| 19 | `h:mm:ss AM/PM` | time |
| 20 | `h:mm` | time |
| 21 | `h:mm:ss` | time |
| 22 | `m/d/yy h:mm` | datetime |
| 37 | `#,##0 ;(#,##0)` | accounting |
| 38 | `#,##0 ;[Red](#,##0)` | accounting red |
| 39 | `#,##0.00;(#,##0.00)` | accounting |
| 40 | `#,##0.00;[Red](#,##0.00)` | accounting red |
| 45 | `mm:ss` | time |
| 46 | `[h]:mm:ss` | elapsed time |
| 47 | `mmss.0` | time |
| 48 | `##0.0E+0` | scientific |
| 49 | `@` | text format |

IDs 5–8 and 23–36 are locale-specific and not shown.

### 6.2 Custom Number Formats (`<x:numFmts>` / `<x:numFmt>`)

Custom formats use IDs ≥ 164. Each entry has:
- `numFmtId` — the unique integer ID
- `formatCode` — the format string

```xml
<x:numFmts count="3">
  <x:numFmt numFmtId="164" formatCode="#,##0.00"/>
  <x:numFmt numFmtId="165" formatCode="yyyy-mm-dd"/>
  <x:numFmt numFmtId="166" formatCode="[Red]0.00%;[Blue]-0.00%"/>
</x:numFmts>
```

**Format code conventions:**
- `0` — required digit placeholder
- `#` — optional digit placeholder
- `,` — thousands separator (or scale trigger: `#,` divides by 1000)
- `.` — decimal separator
- `%` — multiply by 100 and display as percent
- `@` — text placeholder
- `[Red]`, `[Blue]`, `[Color N]` — conditional color for the section
- Multiple sections separated by `;` for positive; negative; zero; text

---

## 7. Alignment (`<x:alignment>` / Alignment / CT_CellAlignment)

### 7.1 Alignment Attributes

Attribute serialisation order (schema-defined):

| Attribute | Type | Description |
|---|---|---|
| `horizontal` | `ST_HorizontalAlignment` | Horizontal text alignment |
| `vertical` | `ST_VerticalAlignment` | Vertical text alignment |
| `textRotation` | 0–255 | Degrees of text rotation (0–90 counterclockwise; 91–180 = 90+normal downward) |
| `wrapText` | boolean | Wrap long text within cell |
| `indent` | unsignedInt | Indent level (each unit is 3 character widths) |
| `relativeIndent` | int | Indent relative to style |
| `justifyLastLine` | boolean | Justify last line of wrapped text |
| `shrinkToFit` | boolean | Shrink content to fit cell |
| `readingOrder` | 0/1/2 | 0=context, 1=LTR, 2=RTL |
| `mergeCell` | string | Internal hint; not for direct use |

### 7.2 Horizontal Alignment Values (ST_HorizontalAlignment)

| Value | Meaning |
|---|---|
| `general` | Default: numbers right-aligned, text left-aligned |
| `left` | Left-aligned |
| `center` | Centred |
| `right` | Right-aligned |
| `fill` | Repeat content to fill cell width |
| `justify` | Justify text (multi-line) |
| `centerContinuous` | Centre across selection |
| `distributed` | Distribute characters evenly |

### 7.3 Vertical Alignment Values (ST_VerticalAlignment)

| Value | Meaning |
|---|---|
| `top` | Align to top of cell |
| `center` | Vertically centred |
| `bottom` | Align to bottom (default) |
| `justify` | Justify vertical spacing |
| `distributed` | Distribute vertically |

### 7.4 `applyAlignment` Flag

Alignment is only active when the XF record's `applyAlignment="1"` flag is set.
Without this flag, the alignment is inherited from the master `cellStyleXfs` entry.

```xml
<x:xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"
      applyAlignment="1">
  <x:alignment horizontal="center" vertical="center" wrapText="1"/>
</x:xf>
```

---

## 8. Named Cell Styles (`<x:cellStyles>`)

Named cell styles assign a user-visible name to a master XF entry in
`<x:cellStyleXfs>`. The "Normal" style at `xfId=0` is always present in a
conforming workbook.

```xml
<x:cellStyles count="1">
  <x:cellStyle name="Normal" xfId="0" builtinId="0"/>
</x:cellStyles>
```

`builtinId` maps to standard Excel style names (0=Normal, 3=Comma, 4=Currency,
5=Percent, 6=Comma[0], 7=Currency[0]).

---

## 9. Complete Minimal StyleSheet Example

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:styleSheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:fonts count="2">
    <x:font><x:sz val="11"/><x:name val="Calibri"/></x:font>
    <x:font><x:b/><x:sz val="11"/><x:name val="Calibri"/></x:font>
  </x:fonts>
  <x:fills count="2">
    <x:fill><x:patternFill patternType="none"/></x:fill>
    <x:fill><x:patternFill patternType="gray125"/></x:fill>
  </x:fills>
  <x:borders count="1">
    <x:border><x:left/><x:right/><x:top/><x:bottom/><x:diagonal/></x:border>
  </x:borders>
  <x:cellStyleXfs count="1">
    <x:xf numFmtId="0" fontId="0" fillId="0" borderId="0"/>
  </x:cellStyleXfs>
  <x:cellXfs count="2">
    <x:xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/>
    <x:xf numFmtId="0" fontId="1" fillId="0" borderId="0" xfId="0" applyFont="1"/>
  </x:cellXfs>
</x:styleSheet>
```

---

## 10. Common Failure Modes

### 10.1 Missing Required Default Fills (Indices 0 and 1)

**Failure:** The fills list starts without `patternType="none"` at index 0 and
`patternType="gray125"` at index 1.

**Impact:** Excel will reject the workbook or display a repair message.

**Fix:** Always include the two required fills as the first entries, even if no
cells use fills.

### 10.2 `cellStyleXfs` Without Required First Entry

**Failure:** Omitting `<x:cellStyleXfs>` entirely, or omitting the Normal
master XF at index 0.

**Impact:** Cell XF records cannot resolve their `xfId` reference and the style
chain is broken. Excel may default-format all cells.

**Fix:** Always include at least one entry in `<x:cellStyleXfs>` as the Normal
style anchor.

### 10.3 Child Ordering in `<x:font>`

**Failure:** Emitting font children in arbitrary order (e.g. `<x:name>` before
`<x:sz>`, or `<x:b>` after `<x:sz>`).

**Impact:** Schema-invalid document. Some consumers are tolerant, but strict
validators fail.

**Fix:** Emit `<x:font>` children in schema order: b, i, strike, condense,
extend, outline, shadow, u, vertAlign, sz, color, name, family, charset, scheme.

---

*End of SpreadsheetML Cell Formatting Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
