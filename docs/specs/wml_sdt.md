# WML Content Controls (SDT) — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.5 (structured document
tags); ISO/IEC 29500:2016 Part 1 §17.5; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

Structured Document Tags (SDTs) are named, typed regions of document content.
They provide:
- Named containers for programmatic access (`<w:tag>`)
- User-visible labels (`<w:alias>`)
- Constrained input (date pickers, dropdowns, plain text only)
- Repeating sections and data-bound controls

Four SDT variants exist based on where the content lives:

| Variant | XML element | Level | Rust struct |
|---------|-------------|-------|-------------|
| Block | `<w:sdt>` inside `<w:body>` or `<w:sdtContent>` | Replaces a paragraph or table | `SdtBlock` |
| Run (inline) | `<w:sdt>` inside `<w:p>` | Replaces a group of runs | `SdtRun` |
| Row | `<w:sdt>` inside a table | Replaces a table row | `SdtRow` |
| Cell | `<w:sdt>` inside `<w:tr>` | Replaces a table cell | `SdtCell` |

All four share the same internal structure: `<w:sdtPr>`, `<w:sdtEndPr>`,
`<w:sdtContent>`.

---

## 2. SDT Structure

```xml
<w:sdt>
  <w:sdtPr>
    <!-- metadata + one control type choice -->
    <w:alias w:val="My Text Field"/>
    <w:tag w:val="field_name"/>
    <w:id w:val="42"/>
    <w:lock w:val="sdtContentLocked"/>
    <w:text/>                            <!-- control type: plain text -->
  </w:sdtPr>
  <w:sdtEndPr/>                          <!-- optional end properties -->
  <w:sdtContent>
    <w:p><w:r><w:t>Default text.</w:t></w:r></w:p>
  </w:sdtContent>
</w:sdt>
```

### CT_SdtPr children (in order)

| Element | Notes |
|---------|-------|
| `rPr` | Run properties applied to the content placeholder text |
| `alias` | Human-readable label shown in the UI |
| `tag` | Programmatic identifier (for data binding / code access) |
| `id` | Unique integer ID within the document |
| `lock` | Lock mode (see §6) |
| `placeholder` | Placeholder text shown when empty |
| `temporary` | Remove control on edit (convert to plain content) |
| `showingPlcHdr` | Whether the placeholder text is currently visible |
| `dataBinding` | XPath binding to custom XML |
| *one control type* | Exactly one of the types listed in §4 |

Rust struct: `SdtProperties { sdt_properties_choice: Vec<SdtPropertiesChoice> }`.

All `sdtPr` children appear as variants of `SdtPropertiesChoice`.

### CT_SdtEndPr

```xml
<w:sdtEndPr>
  <w:rPr><w:b/></w:rPr>   <!-- optional run properties for the end mark -->
</w:sdtEndPr>
```

Rust struct: `SdtEndCharProperties { run_properties: Vec<RunProperties> }`.

Commonly empty (`<w:sdtEndPr/>`).

---

## 3. Metadata Elements

### alias

```xml
<w:alias w:val="First Name"/>
```

User-visible label displayed in the content control title bar. Max length not
specified in the XSD.

Rust struct: `SdtAlias { val }`.

### tag

```xml
<w:tag w:val="firstName"/>
```

Programmatic name used in code. Unlike `alias`, this is not shown to the user.

Rust struct: `Tag { val }`.

### id

```xml
<w:id w:val="1001"/>
```

Unique integer for this SDT. Used by data binding and by Word for tracking.
Must be unique across all SDTs in the document.

Rust struct: `SdtId { val: Int32Value }`.

### lock

```xml
<w:lock w:val="sdtContentLocked"/>
```

### ST_Lock values

| Value | Meaning |
|-------|---------|
| `sdtLocked` | The control itself cannot be deleted |
| `contentLocked` | The content inside cannot be edited |
| `sdtContentLocked` | Both control and content are locked |
| `unlocked` | No locking (default) |

Rust struct: `Lock { val: Option<LockingValues> }`.

---

## 4. Control Types

Exactly one control type element appears inside `<w:sdtPr>`. If none is
present, the SDT behaves as a rich text container.

### Plain text: `<w:text>`

```xml
<w:sdtPr>
  <w:text/>                <!-- single-line plain text (default) -->
</w:sdtPr>
<!-- or -->
<w:sdtPr>
  <w:text w:multiLine="1"/> <!-- multi-line plain text -->
</w:sdtPr>
```

Rust struct: `SdtContentText { multi_line }`.

Content: one or more runs containing `<w:t>` text.

### Rich text (no element — implicit)

When no control type element is present, the SDT is a rich text container.
Content can have any run formatting.

Some producers write `<w:richText/>` explicitly; the XSD allows this as a
distinct choice variant.

### Date picker: `<w:date>`

```xml
<w:sdtPr>
  <w:date w:fullDate="2026-05-02T00:00:00Z">
    <w:dateFormat w:val="M/d/yyyy"/>
    <w:lid w:val="en-US"/>
    <w:storeMappedDataAs w:val="dateTime"/>
  </w:date>
</w:sdtPr>
```

| Attribute/child | Notes |
|-----------------|-------|
| `w:fullDate` | ISO 8601 timestamp of the stored value |
| `w:dateFormat` | Display format string (locale-specific picture) |
| `w:lid` | Language for the date picker calendar |
| `w:storeMappedDataAs` | How value is stored in data binding: `dateTime`, `date`, `text` |

Rust struct: `SdtContentDate { full_date, date_format, language_id, sdt_date_mapping_type, calendar }`.

### Drop-down list: `<w:dropDownList>`

```xml
<w:sdtPr>
  <w:dropDownList w:lastValue="opt2">
    <w:listItem w:displayText="Option One" w:value="opt1"/>
    <w:listItem w:displayText="Option Two" w:value="opt2"/>
    <w:listItem w:displayText="Option Three" w:value="opt3"/>
  </w:dropDownList>
</w:sdtPr>
```

`w:lastValue` stores the `value` of the currently selected item.

Rust struct: `SdtContentDropDownList { last_value, w_list_item: Vec<ListItem> }`.

### Combo box: `<w:comboBox>`

Same structure as `dropDownList` but allows the user to type a freeform value
not in the list. `w:lastValue` stores the current (possibly freeform) value.

Rust struct: `SdtContentComboBox { last_value, w_list_item: Vec<ListItem> }`.

### ListItem: CT_SdtListItem

```xml
<w:listItem w:displayText="Option One" w:value="opt1"/>
```

| Attribute | Notes |
|-----------|-------|
| `display_text` | `w:displayText` — text shown to the user |
| `value` | `w:value` — stored key value |

Rust struct: `ListItem { display_text, value }`.

---

## 5. sdtContent Variants

| SDT level | sdtContent wrapper | Rust struct |
|-----------|--------------------|-------------|
| Block | `CT_SdtContentBlock` | `SdtContentBlock` (paragraphs/tables) |
| Run (inline) | `CT_SdtContentRun` | `SdtContentRun` (runs) |
| Row | `CT_SdtContentRow` | `SdtContentRow` (table rows) |
| Cell | `CT_SdtContentCell` | `SdtContentCell` (table cells) |

Block `sdtContent` can hold paragraphs, tables, and nested SDTs.
Run `sdtContent` holds run-level content (same as the interior of `<w:p>`).

---

## 6. Round-Trip Gotchas

1. **`<w:sdtEndPr/>` must be present even if empty.** Some processors require
   all three structural children (sdtPr, sdtEndPr, sdtContent). An SDT missing
   sdtEndPr may be silently repaired or rejected.

2. **Exactly one control type in `<w:sdtPr>`.** The XSD schema uses a choice
   group — only one control-type element is valid. Writing both `<w:text/>` and
   `<w:date>` in the same sdtPr is invalid.

3. **`<w:id>` values must be unique per document.** Unlike tracked-change IDs,
   SDT IDs do not share space with comment/bookmark IDs, but must be unique
   among SDTs. Word renumbers duplicates on open.

4. **`showingPlcHdr` must be consistent with content.** If `showingPlcHdr` is
   `1`, the sdtContent should contain the placeholder text (from `<w:placeholder>`),
   not the real value. If `0` or absent, it contains the real value. Mis-
   matched states cause Word to flash the wrong content.

5. **`<w:temporary/>` (or `<w:temporary w:val="1"/>`) causes the control to
   disappear when edited.** Round-trip must preserve it; do not add it to SDTs
   that did not have it, or the next edit will remove the control unexpectedly.

6. **`w:lastValue` on dropDownList stores the selected item's `w:value`, not
   `w:displayText`.** A common mistake is storing the display text. Word
   matches `lastValue` against `value` attributes to determine the selected
   item.

7. **Block SDTs replace a paragraph — the `sdtContent` must contain at least
   one `<w:p>`.** An empty `<w:sdtContent/>` inside a block SDT is invalid.

8. **Run SDTs (`SdtRun`) replace runs — content must not contain paragraphs.**
   Placing a `<w:p>` inside run-level `sdtContent` is invalid. Use `SdtBlock`
   for paragraph-level content.

9. **`<w:alias>` is a display name only.** Two SDTs with the same alias are
   allowed (and common in repeating sections). For unique identification, use
   `<w:tag>`.

10. **`w:dataBinding` requires a custom XML part.** An SDT with a `dataBinding`
    child that references a custom XML part not present in the package will
    cause Word to show a red error outline. Round-trip must preserve
    `dataBinding` as-is without validating whether the referenced part exists.

---

## 7. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-SDT-01 | `test-data/wml/content_controls.docx` | Block SDT with plain text control (alias, tag, id); inline run SDT with date picker (fullDate, dateFormat); inline run SDT with dropDownList (listItems, lastValue) |
