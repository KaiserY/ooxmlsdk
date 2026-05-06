# SpreadsheetML Cells and Shared Strings — Clean-Room Specification
## Cell Values, Shared String Table, Rich Text, Multiple Worksheets

**Purpose:** This document is a clean-room synthesis of the SpreadsheetML cell
and shared-string model as defined in ECMA-376 Part 1 (5th edition) and
ISO/IEC 29500-1:2016. It covers the cell element, all value-type codes, the
shared strings table, rich-text inline runs, multiple worksheet support, and row
structure. It is intended as an implementation guide and test oracle for
`ooxmlsdk` and similar Rust OOXML parsers.

**Sources used:**
- ECMA-376 Part 1, 5th edition — §18.3 (Worksheets) and §18.4 (Shared String
  Table)
- ISO/IEC 29500-1:2016 — cross-reference for SML cell semantics
- `sml.xsd` from this repository
  (`schemas/OfficeOpenXML-XMLSchema-Transitional/`)
- Microsoft Open Specifications: `[MS-XLSX]`
- Existing `ooxmlsdk` test fixtures (`test-data/spreadsheet/`)
- Apache POI and openpyxl source code (reference implementations)

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or have been clearly
rewritten.

---

## 1. Namespace and Conventional Prefix

```
Namespace URI:  http://schemas.openxmlformats.org/spreadsheetml/2006/main
Conventional prefix: x
Schema type prefix: CT_ (complex type), ST_ (simple type)
```

All elements described in this document are in the `x:` namespace. The
relationship namespace `r:` (`http://schemas.openxmlformats.org/officeDocument/2006/relationships`)
appears on `<x:sheet r:id="...">` and on hyperlink references.

---

## 2. Multiple Worksheets

### 2.1 `<x:sheets>` — Sheet Listing

The `<x:workbook>` element contains a required `<x:sheets>` child that lists
every worksheet in the workbook. Each entry maps a display name, a numeric ID,
and a relationship ID to a worksheet part.

**Schema type `CT_Sheets`:**

```xml
<x:sheets>
  <x:sheet name="Sheet1" sheetId="1" r:id="rId1"/>
  <x:sheet name="Summary" sheetId="2" r:id="rId2"/>
  <x:sheet name="Archive" sheetId="3" state="hidden" r:id="rId3"/>
</x:sheets>
```

### 2.2 `<x:sheet>` Attributes (CT_Sheet)

Attribute order in the serialised XML (schema-defined order):

| Attribute | Type | Required | Description |
|---|---|---|---|
| `name` | `ST_Xstring` (1–31 chars) | Yes | Display name shown on the tab |
| `sheetId` | `xsd:unsignedInt` | Yes | Permanent integer ID; does not change when sheets are reordered |
| `state` | `ST_SheetState` | No | Visibility (see §2.3) |
| `r:id` | `xsd:string` | Yes | Relationship ID pointing to the worksheet part |

### 2.3 Sheet State Values (ST_SheetState)

| Value | Meaning |
|---|---|
| *(absent)* | Sheet is visible (default) |
| `visible` | Sheet is visible (explicit) |
| `hidden` | Sheet is hidden; can be unhidden via the UI |
| `veryHidden` | Sheet is hidden and cannot be unhidden through normal UI; requires programmatic access |

A sheet with `state="veryHidden"` still requires a complete worksheet part in the
ZIP package — the state attribute controls UI visibility only, not the existence of
the part.

---

## 3. `<x:row>` — Row Element

Schema type `CT_Row`. A row element contains zero or more cell (`<x:c>`)
children and optional extension data.

### 3.1 Row Attributes

Attribute serialisation order (schema-defined):

| Attribute | Type | Description |
|---|---|---|
| `r` | `xsd:unsignedInt` | 1-based row index |
| `spans` | list of `ST_CellSpan` | Optimisation hint: space-separated list of `min:max` column spans |
| `s` | `xsd:unsignedInt` | Default style index (index into `<x:cellXfs>`) |
| `customFormat` | `xsd:boolean` | Whether `s` attribute applies |
| `ht` | `xsd:double` | Custom row height in points |
| `hidden` | `xsd:boolean` | Row is hidden |
| `customHeight` | `xsd:boolean` | Whether `ht` is used instead of default |
| `outlineLevel` | `xsd:unsignedByte` | Outline grouping level (0–7) |
| `collapsed` | `xsd:boolean` | Whether this row's outline group is collapsed |
| `thickTop` | `xsd:boolean` | Row has thick top border |
| `thickBot` | `xsd:boolean` | Row has thick bottom border |
| `ph` | `xsd:boolean` | Show phonetic annotations |

**Minimal row with index only:**
```xml
<x:row r="5">
  <x:c r="A5"><x:v>100</x:v></x:c>
</x:row>
```

---

## 4. `<x:c>` — Cell Element

Schema type `CT_Cell`. A cell element carries the cell's reference, optional
style index, data type, and child elements for the value, formula, or inline
string.

### 4.1 Cell Attributes

Attribute serialisation order (schema-defined):

| Attribute | Type | Description |
|---|---|---|
| `r` | `ST_CellRef` | Cell reference in A1 notation (e.g. `A1`, `C12`) |
| `s` | `xsd:unsignedInt` | Style index (0-based index into `<x:cellXfs>`) |
| `t` | `ST_CellType` | Data type code (see §4.2) |
| `cm` | `xsd:unsignedInt` | Cell metadata index |
| `vm` | `xsd:unsignedInt` | Value metadata index |
| `ph` | `xsd:boolean` | Show phonetic annotation |

### 4.2 Cell Type Codes (ST_CellType)

| `t=` value | Meaning | `<x:v>` content | Notes |
|---|---|---|---|
| *(absent)* | Numeric | Decimal number string | Default type |
| `n` | Numeric (explicit) | Decimal number string | Equivalent to absent; rarely emitted |
| `s` | Shared string | Zero-based integer index into `<x:sst>` | Most efficient for repeated text |
| `b` | Boolean | `1` (true) or `0` (false) | |
| `d` | ISO 8601 date | Date-time string | Not universally supported; numeric serial dates are more common |
| `e` | Error | Error code string | See §4.3 for valid values |
| `str` | Formula string result | String | `t="str"` means the cell has a formula whose result is a string; `<x:f>` is present alongside `<x:v>` |
| `inlineStr` | Inline string | — | No `<x:v>`; the value is in `<x:is><x:t>` instead |

### 4.3 Error Cell Values

When `t="e"`, the `<x:v>` element contains one of these error-code strings:

| String | Excel error |
|---|---|
| `#NULL!` | Null intersection |
| `#DIV/0!` | Division by zero |
| `#VALUE!` | Wrong value type |
| `#REF!` | Invalid cell reference |
| `#NAME?` | Unrecognised formula name |
| `#NUM!` | Numeric error |
| `#N/A` | Not available |
| `#GETTING_DATA` | External data not yet fetched |

### 4.4 Child Element Order (CT_Cell)

Child elements appear in this schema-defined order:

1. `<x:f>` — optional formula element (see `sml_formulas.md`)
2. `<x:v>` — optional value element
3. `<x:is>` — optional inline string (only when `t="inlineStr"`)
4. `<x:extLst>` — optional extension list

### 4.5 `<x:v>` — Cell Value (CellValue)

Schema type `CT_Xstring`. Simple text content element. The text content is
the serialised value:
- For numeric cells: a decimal string such as `42` or `3.14159`
- For boolean cells: `1` or `0`
- For shared-string cells: zero-based integer index as a string
- For error cells: the error code string
- For formula-string cells (`t="str"`): the computed string value

**Example cells:**
```xml
<!-- Numeric (default type) -->
<x:c r="A1"><x:v>42</x:v></x:c>

<!-- Boolean -->
<x:c r="B1" t="b"><x:v>1</x:v></x:c>

<!-- Shared string index 2 -->
<x:c r="C1" t="s"><x:v>2</x:v></x:c>

<!-- Error cell -->
<x:c r="D1" t="e"><x:v>#DIV/0!</x:v></x:c>

<!-- Formula with string result -->
<x:c r="E1" t="str"><x:f>TEXT(42,"0.00")</x:f><x:v>42.00</x:v></x:c>
```

### 4.6 `<x:is>` — Inline String (InlineString)

Schema type `CT_Rst`. Used only when `t="inlineStr"`. Contains the same
child model as a shared string item: an optional `<x:t>` for plain text, or
one or more `<x:r>` for rich text runs.

```xml
<!-- Inline plain string -->
<x:c r="A1" t="inlineStr">
  <x:is><x:t>Hello world</x:t></x:is>
</x:c>

<!-- Inline rich text -->
<x:c r="A2" t="inlineStr">
  <x:is>
    <x:r><x:rPr><x:b/></x:rPr><x:t>Bold</x:t></x:r>
    <x:r><x:t xml:space="preserve"> normal</x:t></x:r>
  </x:is>
</x:c>
```

### 4.7 Blank Cell Semantics

A blank cell has no value, no formula, and no type attribute. The element may
be omitted entirely from `<x:sheetData>`, or it may appear as a self-closing
element when the cell carries a style index:

```xml
<!-- Completely omitted — implicit blank -->
<!-- (no entry in <x:row r="1"> for column B) -->

<!-- Self-closing blank — retains style but no value -->
<x:c r="C1"/>

<!-- Blank with style (styled empty cell) -->
<x:c r="D1" s="2"/>
```

A parser MUST treat absent cells and self-closing `<x:c>` elements without
children as blank. A blank cell differs from a zero-value numeric cell.

---

## 5. Shared Strings Table

### 5.1 Overview

The shared strings table is an optional part stored at `xl/sharedStrings.xml`.
It is used to deduplicate string values: multiple cells can share the same
string entry, referenced by a zero-based integer index in `<x:v>`.

The part is related from `xl/workbook.xml` via:
- Relationship type: `http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings`
- Content type: `application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml`

### 5.2 `<x:sst>` — Shared String Table (SharedStringTable)

Root element. Schema type `CT_Sst`.

| Attribute | Type | Description |
|---|---|---|
| `count` | `xsd:unsignedInt` | Total number of string references in the workbook (may exceed `uniqueCount` due to sharing) |
| `uniqueCount` | `xsd:unsignedInt` | Number of unique string entries in this table |

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:sst xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
       count="5" uniqueCount="3">
  <x:si><x:t>Hello</x:t></x:si>
  <x:si><x:t>World</x:t></x:si>
  <x:si><x:t>Total</x:t></x:si>
</x:sst>
```

### 5.3 `<x:si>` — Shared String Item (SharedStringItem)

Schema type `CT_Rst`. Each `<x:si>` is one table entry. Its zero-based position
in the table is the index used in cells. The content is either:
- A single `<x:t>` for plain text, or
- One or more `<x:r>` rich-text runs (possibly mixed with `<x:t>` for a leading
  plain-text prefix)

Child element order (schema-defined):

1. `<x:t>` — optional plain text (type `CT_Xstring`)
2. `<x:r>` — zero or more rich-text runs
3. `<x:rPh>` — zero or more phonetic runs (East Asian annotation)
4. `<x:phoneticPr>` — optional phonetic properties

### 5.4 `<x:t>` — Text Element

Schema type `CT_Xstring`. Simple text content. Carries the `xml:space`
attribute in the XML namespace.

**`xml:space="preserve"` rule:** The same rule as WML `<w:t>` applies:

> A producer MUST set `xml:space="preserve"` on `<x:t>` whenever the text
> value begins with a space, ends with a space, or consists entirely of spaces.

```xml
<x:t>No surrounding spaces</x:t>
<x:t xml:space="preserve"> leading space</x:t>
<x:t xml:space="preserve">trailing space </x:t>
<x:t xml:space="preserve"> </x:t>
```

### 5.5 Rich-Text Runs in SST

A rich-text entry is composed of one or more `<x:r>` run elements. Each run
has optional run properties (`<x:rPr>`) followed by a required `<x:t>` text
element.

**Run element `<x:r>` (CT_RElt):**

```xml
<x:si>
  <x:r>
    <x:rPr>
      <x:b/>
      <x:sz val="14"/>
      <x:color rgb="FFFF0000"/>
      <x:rFont val="Arial"/>
    </x:rPr>
    <x:t>Bold red 14pt</x:t>
  </x:r>
  <x:r>
    <x:rPr>
      <x:sz val="11"/>
      <x:rFont val="Arial"/>
    </x:rPr>
    <x:t xml:space="preserve"> normal 11pt</x:t>
  </x:r>
</x:si>
```

**`<x:rPr>` — Run Properties (RunProperties / CT_RPrElt)**

The run properties element for rich-text SST runs uses `CT_RPrElt`, which
differs from the font-level `CT_Font` type in naming:

| Element | Description |
|---|---|
| `<x:b/>` | Bold (CT_BooleanProperty) |
| `<x:i/>` | Italic |
| `<x:strike/>` | Strikethrough |
| `<x:condense/>` | Condense character spacing |
| `<x:extend/>` | Extend character spacing |
| `<x:outline/>` | Outline effect |
| `<x:shadow/>` | Shadow effect |
| `<x:u val="..."/>` | Underline style |
| `<x:vertAlign val="..."/>` | Vertical alignment (superscript/subscript/baseline) |
| `<x:sz val="..."/>` | Font size in points |
| `<x:color rgb="..." theme="..." tint="..."/>` | Text color |
| `<x:rFont val="..."/>` | Font family name (note: `rFont`, not `name`) |
| `<x:family val="..."/>` | Font family numeric code |
| `<x:charset val="..."/>` | Character set |
| `<x:scheme val="..."/>` | Font scheme (minor/major) |

**Critical difference:** Run properties in `CT_RPrElt` use `<x:rFont val="..."/>`
(not `<x:name val="..."/>` as in `CT_Font`). Confusing these two produces a
schema-invalid document.

Run properties elements appear in the order listed above (b, i, strike,
condense, extend, outline, shadow, u, vertAlign, sz, color, rFont, family,
charset, scheme).

---

## 6. Worksheet Structure

### 6.1 `<x:sheetData>` — Sheet Data

All cell data lives inside `<x:sheetData>`, which is a required child of
`<x:worksheet>`. It contains zero or more `<x:row>` elements in ascending row
index order.

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:worksheet xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
  <x:sheetData>
    <x:row r="1">
      <x:c r="A1" t="s"><x:v>0</x:v></x:c>
      <x:c r="B1"><x:v>100</x:v></x:c>
    </x:row>
  </x:sheetData>
</x:worksheet>
```

The `<x:worksheet>` element's children are defined in strict schema order
(see `sml_layout.md` for the full ordered list). The most commonly used
children in order are:

1. `<x:sheetPr>` — sheet properties (optional)
2. `<x:dimension>` — used range hint (optional)
3. `<x:sheetViews>` — view settings including freeze panes (optional)
4. `<x:sheetFormatPr>` — default row/column dimensions (optional)
5. `<x:cols>` — column dimension overrides (zero or more)
6. `<x:sheetData>` — cell data (required)
7. `<x:mergeCells>` — merged cell ranges (optional)
8. `<x:conditionalFormatting>` — conditional formats (zero or more)
9. `<x:hyperlinks>` — hyperlink list (optional)
10. `<x:pageMargins>` — page margins (optional)

---

## 7. Common Failure Modes

### 7.1 Missing `xml:space="preserve"` on `<x:t>`

**Failure:** Writing `<x:t> leading space</x:t>` without `xml:space="preserve"`.

**Impact:** Leading and trailing spaces are silently stripped on re-parse.
String values appear wrong or truncated.

**Fix:** Apply the same rule as WML: emit `xml:space="preserve"` whenever the
text starts or ends with a space character.

### 7.2 Confusing `<x:rFont>` with `<x:name>` in Rich-Text Runs

**Failure:** Using `<x:name val="Arial"/>` inside `<x:rPr>` (run properties)
instead of `<x:rFont val="Arial"/>`.

**Impact:** The element name is schema-invalid inside `CT_RPrElt`. The font
setting is ignored or the document fails to parse.

**Fix:** Run properties for SST and inline strings use `CT_RPrElt`, which
requires `<x:rFont>`. Font-level properties in `<x:font>` inside
`<x:styleSheet>` use `CT_Font`, which requires `<x:name>`.

### 7.3 Incorrect `count` / `uniqueCount` on `<x:sst>`

**Failure:** Emitting wrong values for the `count` or `uniqueCount` attributes.

**Impact:** Some consumers validate or use these counts for pre-allocation.
Incorrect values may cause memory over-allocation or early truncation.

**Fix:** `uniqueCount` is the number of `<x:si>` entries. `count` is the sum
of all shared-string cell references across all worksheets.

### 7.4 Using `t="str"` for a Plain String Cell

**Failure:** Marking a cell `t="str"` without a `<x:f>` formula element.

**Impact:** `t="str"` means "the cell has a formula whose result is a string."
A plain literal string cell must use `t="s"` (shared string index) or
`t="inlineStr"` (inline string). A `t="str"` cell without `<x:f>` is
technically malformed.

**Fix:** Use `t="s"` + SST index or `t="inlineStr"` + `<x:is><x:t>` for literal
string cells. Reserve `t="str"` for formula result cells.

---

*End of SpreadsheetML Cells and Shared Strings Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
