# SpreadsheetML Formulas — Clean-Room Specification
## Formula Element, Formula Types, Shared Formulas, Array Formulas

**Purpose:** This document is a clean-room synthesis of the SpreadsheetML
formula model as defined in ECMA-376 Part 1 (5th edition) and ISO/IEC
29500-1:2016. It covers the `<x:f>` formula element, formula type codes,
A1-style reference grammar, shared and array formula patterns, recalculation
flags, and cached result semantics. It is intended as an implementation guide
and test oracle for `ooxmlsdk` and similar Rust OOXML parsers.

**Sources used:**
- ECMA-376 Part 1, 5th edition — §18.3.6 (CT_CellFormula) and §18.17 (formulas)
- ISO/IEC 29500-1:2016 — cross-reference for formula semantics
- `sml.xsd` from this repository
- Microsoft Open Specifications: `[MS-XLSX]`
- Apache POI and openpyxl source code (reference implementations)

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or have been clearly
rewritten.

---

## 1. `<x:f>` — Cell Formula Element

Schema type `CT_CellFormula`. A cell formula element is a direct child of
`<x:c>` and precedes `<x:v>`.

### 1.1 Attribute Order (CT_CellFormula)

Attributes are serialised in this schema-defined order:

| Attribute | Type | Default | Description |
|---|---|---|---|
| `t` | `ST_CellFormulaType` | `normal` | Formula type code (see §2) |
| `aca` | `xsd:boolean` | `false` | Always calculate array — recalculate on every open |
| `ref` | `ST_Ref` | — | Range reference for shared/array formula scope |
| `dt2D` | `xsd:boolean` | `false` | Data table is two-dimensional |
| `dtr` | `xsd:boolean` | `false` | Data table row orientation |
| `del1` | `xsd:boolean` | `false` | Input 1 deleted |
| `del2` | `xsd:boolean` | `false` | Input 2 deleted |
| `r1` | `ST_CellRef` | — | Data table row-input cell |
| `r2` | `ST_CellRef` | — | Data table column-input cell |
| `ca` | `xsd:boolean` | `false` | Calculate cell — forces recalculation on next open |
| `si` | `xsd:unsignedInt` | — | Shared-formula group index (integer, 0-based) |
| `bx` | `xsd:boolean` | `false` | Assigns value to name |
| `xml:space` | (XML namespace) | — | Whitespace handling |

The text content of `<x:f>` is the formula expression string, without the
leading `=` sign.

### 1.2 `<x:v>` as Cached Result

When a formula cell is saved, the `<x:v>` element contains the cached result
of the last calculation. A consumer that does not execute formulas uses this
cached value for display. The cached value type is controlled by the cell's
`t` attribute (same type codes as non-formula cells, except `t="str"` means
the formula result is a string).

```xml
<x:c r="A3">
  <x:f>A1+A2</x:f>   <!-- formula text, no leading = -->
  <x:v>3</x:v>        <!-- cached result -->
</x:c>
```

---

## 2. Formula Type Codes (ST_CellFormulaType)

### 2.1 `normal` — Standard Formula (default)

A regular single-cell formula. The `t` attribute may be omitted.

```xml
<x:c r="C1">
  <x:f>A1*B1</x:f>
  <x:v>42</x:v>
</x:c>
```

### 2.2 `shared` — Shared Formula

A shared formula allows multiple cells to share a single formula definition,
reducing file size. The "definition cell" (first in the range) carries the
formula text and a `ref` attribute declaring the range. Dependent cells carry
only the `si` (shared index) attribute with no formula text.

**Shared formula pattern:**

```xml
<!-- Definition cell — has formula text, t="shared", ref, and si -->
<x:row r="1">
  <x:c r="A1">
    <x:f t="shared" ref="A1:A4" si="0">B1*2</x:f>
    <x:v>20</x:v>
  </x:c>
</x:row>
<!-- Dependent cells — t="shared", si only, no formula text -->
<x:row r="2">
  <x:c r="A2">
    <x:f t="shared" si="0"/>
    <x:v>40</x:v>
  </x:c>
</x:row>
<x:row r="3">
  <x:c r="A3">
    <x:f t="shared" si="0"/>
    <x:v>60</x:v>
  </x:c>
</x:row>
```

**Rules for shared formulas:**
- The definition cell MUST have `ref` and the formula text.
- Dependent cells MUST have `si` matching the definition cell's `si`, and no
  formula text.
- The formula is conceptually adjusted for each cell's position relative to the
  definition cell — the same relative-reference rules as copy-paste apply.
- The `si` values are workbook-scoped per worksheet; each worksheet has its own
  independent numbering starting from 0.

### 2.3 `array` — Array Formula

An array formula operates over ranges and produces a single value (or an array
of values in a multi-cell array formula). The definition cell carries `t="array"`
and a `ref` attribute for the formula's scope.

**Single-cell array formula:**
```xml
<x:c r="A1">
  <x:f t="array" ref="A1">SUM(B1:B3*C1:C3)</x:f>
  <x:v>32</x:v>
</x:c>
```

The `ref` attribute for a single-cell array formula is just the cell itself.
For a multi-cell array formula (a range), the definition cell is the top-left
of the range and dependents are empty `<x:f>` elements.

### 2.4 `dataTable` — Data Table Formula

Used for Excel data tables (what-if analysis). The formula is generated by Excel
and is not editable. Key attributes: `dt2D`, `dtr`, `r1`, `r2`, `del1`, `del2`.

---

## 3. A1-Style Cell Reference Grammar

### 3.1 Basic References

A1-style references consist of a column letter(s) and a row number.

| Form | Example | Meaning |
|---|---|---|
| Relative column, relative row | `A1` | Both column and row adjust on copy |
| Absolute column, relative row | `$A1` | Column fixed; row adjusts |
| Relative column, absolute row | `A$1` | Column adjusts; row fixed |
| Absolute column, absolute row | `$A$1` | Both fixed — never adjusts |

Column letters are case-insensitive in the spec, but producers typically emit
uppercase. Parsers MUST accept both.

Column numbering: A=1, B=2, …, Z=26, AA=27, …, XFD=16384.

### 3.2 Range References

A range reference uses `:` to separate two cell references:

```
A1:C3       — columns A–C, rows 1–3 (nine cells)
$A$1:$C$3   — same range, absolute
A:A         — entire column A
1:1         — entire row 1
```

### 3.3 3D References (Multi-Sheet)

A 3D reference prefixes the cell or range with a sheet name (or sheet range),
separated by `!`:

```
Sheet1!A1           — cell A1 in Sheet1
Sheet1!$A$1:$C$3    — absolute range in Sheet1
Sheet1:Sheet3!A1    — cell A1 across sheets Sheet1 through Sheet3
'My Sheet'!A1       — sheet names with spaces must be quoted with single quotes
```

If the sheet name contains a space, apostrophe (`'`), or special character, it
must be enclosed in single quotes. A literal apostrophe in a sheet name is
doubled: `'O''Reilly'!A1`.

---

## 4. Recalculation Flags

### 4.1 `ca` — Calculate Cell

When `ca="1"` is set on `<x:f>`, the formula cell is marked for recalculation
on the next workbook open. This flag is used when the cached `<x:v>` result is
known to be stale — for example, when a shared formula chain has been updated
but not all dependents have been recalculated.

```xml
<x:c r="B5">
  <x:f ca="1">SUM(B1:B4)</x:f>
  <x:v>0</x:v>   <!-- stale cached value, will be recalculated -->
</x:c>
```

A round-trip parser MUST preserve `ca` verbatim. Dropping it can prevent
recalculation of stale formulas.

### 4.2 `bx` — Assigns Value to Name

The `bx` flag is set on formula cells that are the result cell for a defined
name assignment (an array formula that assigns results to a name). This is a
rare, advanced feature.

---

## 5. Relationship Between Formula Types and Cell `t` Attribute

The cell's `t` attribute (type) and the formula's `t` attribute (formula type)
are independent:

| Cell `t` | Meaning |
|---|---|
| absent / `n` | Formula result is numeric |
| `s` | Unusual — formula result is a shared-string index |
| `b` | Formula result is boolean |
| `e` | Formula result is an error |
| `str` | Formula result is a string (most common for text formulas) |

When a formula cell has `t="str"`, the cached `<x:v>` content is the string
value itself, not a shared-string index:

```xml
<x:c r="E1" t="str">
  <x:f>TEXT(42,"0.00")</x:f>
  <x:v>42.00</x:v>   <!-- string, not a number -->
</x:c>
```

---

## 6. Common Failure Modes

### 6.1 Formula Text with Leading `=`

**Failure:** Storing the formula with a leading `=` sign, e.g. `<x:f>=A1+A2</x:f>`.

**Impact:** The formula fails to evaluate. Application-level parsers may reject
the document or silently produce wrong results.

**Fix:** Always strip the `=` from the formula text. The element is already
unambiguously a formula container; the `=` is an input-mode artefact.

### 6.2 Incorrect Shared Formula Structure

**Failure:** Emitting formula text on dependent cells, or omitting the `ref`
attribute on the definition cell.

**Impact:** Consumers that reconstruct shared formulas from the pattern will
fail. Some consumers may interpret each cell as an independent formula and
re-enter the string "B1*2" literally in every cell.

**Fix:** Follow the pattern strictly: definition cell has both `ref` and formula
text; dependent cells have only `si` (and no text content in `<x:f>`).

### 6.3 Missing Cached Value

**Failure:** Saving a formula cell without a `<x:v>` child (no cached result).

**Impact:** Read-only consumers (importers, viewers, converters) that do not
execute formulas will display a blank or zero value.

**Fix:** Always emit the last-calculated result as a `<x:v>` child. If the
value is unknown, emit `<x:v>0</x:v>` and set `ca="1"` to signal that
recalculation is needed.

---

*End of SpreadsheetML Formulas Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
