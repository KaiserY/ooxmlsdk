# SpreadsheetML Defined Names — Clean-Room Specification
## Workbook-Scoped and Sheet-Scoped Named Ranges and Constants

**Purpose:** This document is a clean-room synthesis of the SpreadsheetML
defined-names model as defined in ECMA-376 Part 1 (5th edition) and ISO/IEC
29500-1:2016. It covers the `<x:definedNames>` container, the `<x:definedName>`
element, scope rules, built-in reserved names, and dynamic named ranges. It is
intended as an implementation guide and test oracle for `ooxmlsdk` and similar
Rust OOXML parsers.

**Sources used:**
- ECMA-376 Part 1, 5th edition — §18.2.6 (CT_DefinedNames)
- ISO/IEC 29500-1:2016 — cross-reference for name scoping
- `sml.xsd` from this repository
- Microsoft Open Specifications: `[MS-XLSX]`
- Apache POI and openpyxl source code (reference implementations)

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or have been clearly
rewritten.

---

## 1. Location in the Workbook

Defined names are children of the `<x:workbook>` element, inside a single
optional `<x:definedNames>` container. The container is placed after
`<x:externalReferences>` and before `<x:calcPr>` in the workbook's child order.

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<x:workbook xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
            xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <x:sheets>
    <x:sheet name="Sheet1" sheetId="1" r:id="rId1"/>
  </x:sheets>
  <x:definedNames>
    <x:definedName name="TotalRevenue">Sheet1!$B$1:$B$10</x:definedName>
  </x:definedNames>
</x:workbook>
```

---

## 2. `<x:definedName>` — Defined Name Entry (DefinedName / CT_DefinedName)

### 2.1 Attributes

Attribute serialisation order (schema-defined):

| Attribute | Type | Required | Description |
|---|---|---|---|
| `name` | `ST_Xstring` | Yes | The name identifier |
| `comment` | `ST_Xstring` | No | Human-readable comment displayed in Name Manager |
| `customMenu` | `ST_Xstring` | No | Custom menu text |
| `description` | `ST_Xstring` | No | Extended description |
| `help` | `ST_Xstring` | No | Help file topic ID |
| `statusBar` | `ST_Xstring` | No | Text shown in status bar |
| `localSheetId` | `xsd:unsignedInt` | No | 0-based sheet index (scopes name to one sheet) |
| `hidden` | `xsd:boolean` | No | Hides name from Name Manager UI |
| `function` | `xsd:boolean` | No | Name is a user-defined function |
| `vbProcedure` | `xsd:boolean` | No | Name refers to a VBA procedure |
| `xlm` | `xsd:boolean` | No | Name is an XLM macro function |
| `functionGroupId` | `xsd:unsignedInt` | No | Function category group ID |
| `shortcutKey` | `xsd:string` | No | Single character keyboard shortcut |
| `publishToServer` | `xsd:boolean` | No | Publish name to Excel Services |
| `workbookParameter` | `xsd:boolean` | No | Name is a workbook parameter |

### 2.2 Text Content (Formula Value)

The text content of `<x:definedName>` is the formula expression that the name
evaluates to. This may be:
- A cell reference: `Sheet1!$A$1`
- A range reference: `Sheet1!$A$1:$C$3`
- A constant: `42` or `"hello"`
- A formula: `SUM(Sheet1!$A$1:$A$10)`

The content does NOT carry a leading `=` sign.

---

## 3. Scope Rules

### 3.1 Workbook-Scoped Names

When `localSheetId` is **absent**, the name is workbook-scoped and can be
referenced from any sheet without qualification.

```xml
<!-- Workbook-scoped: visible from all sheets as "TotalRevenue" -->
<x:definedName name="TotalRevenue">Sheet1!$B$1:$B$10</x:definedName>
```

### 3.2 Sheet-Scoped Names

When `localSheetId` is **present**, the name is scoped to the sheet at that
zero-based index in the `<x:sheets>` listing. The same name can appear multiple
times with different `localSheetId` values — each binding is distinct.

```xml
<!-- Sheet-scoped to the first sheet (index 0) -->
<x:definedName name="LocalRange" localSheetId="0">Sheet1!$A$1:$A$5</x:definedName>
<!-- Sheet-scoped to the second sheet (index 1) -->
<x:definedName name="LocalRange" localSheetId="1">Sheet2!$A$1:$A$5</x:definedName>
```

A sheet-scoped name is referenced from formulas on its sheet without
qualification, or from other sheets with the sheet name prefix. The
`localSheetId` is a zero-based position index into `<x:sheets>`, not the
`sheetId` attribute value.

### 3.3 Scope Priority

When two names share the same string but one is workbook-scoped and one is
sheet-scoped, a formula on the named sheet resolves to the sheet-scoped name.
Formulas on other sheets resolve to the workbook-scoped name.

---

## 4. Built-in Reserved Names

Excel reserves names prefixed with `_xlnm.` for special purposes. These are
stored as `<x:definedName>` entries with the reserved prefix.

| Name | Description |
|---|---|
| `_xlnm.Print_Area` | Print area for a sheet; always sheet-scoped |
| `_xlnm.Print_Titles` | Row and/or column titles repeated on each printed page |
| `_xlnm._FilterDatabase` | Auto-filter range; set automatically by Excel |
| `_xlnm.Sheet_Title` | Sheet-level title (legacy, rarely used) |
| `_xlnm.Consolidate_Area` | Data consolidation range |
| `_xlnm.Database` | Database range for data functions |
| `_xlnm.Criteria` | Criteria range for database functions |
| `_xlnm.Extract` | Extract range for database functions |
| `_xlnm.Auto_Open` | Macro to run on workbook open |
| `_xlnm.Auto_Close` | Macro to run on workbook close |

**Print area example:**
```xml
<!-- Print area for the first sheet -->
<x:definedName name="_xlnm.Print_Area" localSheetId="0">Sheet1!$A$1:$D$20</x:definedName>
```

**Print titles example (repeat first row on all printed pages):**
```xml
<x:definedName name="_xlnm.Print_Titles" localSheetId="0">Sheet1!$1:$1</x:definedName>
```

---

## 5. Name Syntax Rules

Valid defined names:
- Begin with a letter, underscore, or backslash
- Contain letters, digits, underscores, periods, backslashes, and question
  marks
- Are not A1-style cell references (e.g., `C12` is invalid as a name)
- Are not reserved words (e.g., `TRUE`, `FALSE`, function names)
- Are case-insensitive (`Revenue` and `REVENUE` are the same name)
- Maximum 255 characters

Names that include spaces or special characters must be enclosed in single
quotes when used inside formula expressions: `'Sales Data'!$A$1`.

---

## 6. Dynamic Named Ranges

A named range can point to a formula rather than a static range, enabling
dynamic sizing. The value is any valid formula expression:

```xml
<!-- Dynamic range using OFFSET -->
<x:definedName name="DynamicRevenue">
  OFFSET(Sheet1!$B$1,0,0,COUNTA(Sheet1!$B:$B),1)
</x:definedName>
```

Dynamic ranges cannot be used as print areas or print titles.

---

## 7. Complete Example

```xml
<x:definedNames>
  <!-- Workbook-scoped range -->
  <x:definedName name="TotalRevenue">Sheet1!$B$1:$B$10</x:definedName>
  <!-- Sheet-scoped range (first sheet) -->
  <x:definedName name="LocalRange" localSheetId="0">Sheet1!$A$1:$A$5</x:definedName>
  <!-- Built-in print area for first sheet -->
  <x:definedName name="_xlnm.Print_Area" localSheetId="0">Sheet1!$A$1:$D$20</x:definedName>
  <!-- Hidden constant -->
  <x:definedName name="TaxRate" hidden="1">0.085</x:definedName>
</x:definedNames>
```

---

## 8. Common Failure Modes

### 8.1 Confusing `localSheetId` with `sheetId`

**Failure:** Using the sheet's `sheetId` attribute value as the `localSheetId`.

**Impact:** The name is scoped to the wrong sheet. In a workbook with sheets
numbered non-sequentially (e.g., `sheetId=5` for the first sheet), the name
would point to a non-existent sheet.

**Fix:** `localSheetId` is a zero-based position index into the `<x:sheets>`
list, not the `sheetId` attribute value.

### 8.2 Duplicate Workbook-Scoped Names

**Failure:** Emitting two `<x:definedName>` entries with the same `name` and
neither having `localSheetId`.

**Impact:** Schema-invalid document. Excel will display a repair/recovery dialog
or silently discard the duplicate.

**Fix:** Ensure names are unique per scope. For sheet-scoped names, the
combination of (name, localSheetId) must be unique.

### 8.3 Leading `=` in Formula Value

**Failure:** Storing `=Sheet1!$A$1:$C$3` as the text content of `<x:definedName>`.

**Impact:** The `=` is treated as part of the formula expression and will cause
a parse error or produce an incorrect reference.

**Fix:** Omit the leading `=` from the text content, just as with `<x:f>`.

---

*End of SpreadsheetML Defined Names Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
