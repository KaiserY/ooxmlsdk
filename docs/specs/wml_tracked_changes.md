# WML Tracked Changes — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.13 (tracked changes);
ISO/IEC 29500:2016 Part 1 §17.13; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

Tracked changes record edits without discarding the original content. There
are two categories:

| Category | Elements | Description |
|----------|----------|-------------|
| Content changes | `<w:ins>`, `<w:del>` | Inserted or deleted runs/paragraphs |
| Property changes | `*PrChange` elements | Changed formatting (previous state preserved) |

**Round-trip goal:** Preserve all tracked-change markup exactly as read.
ooxmlsdk must neither accept nor reject changes — it passes them through
unchanged.

---

## 2. Common Attributes (CT_TrackChange)

All tracked-change elements share these attributes, inherited from
`CT_TrackChange`:

| Attribute | XML name | Required | Type | Notes |
|-----------|----------|----------|------|-------|
| `id` | `w:id` | **yes** | integer | Unique revision ID within the document. |
| `author` | `w:author` | **yes** | string (≤ 255 chars) | Name of the person who made the change. |
| `date` | `w:date` | no | ISO 8601 dateTime | Timestamp of the change (e.g., `2026-05-02T10:00:00Z`). |

Rust field names: `id: StringValue`, `author: StringValue`,
`date: Option<DateTimeValue>`.

**`w:id` uniqueness:** All `w:id` values across all tracked-change and comment
elements in the document must be unique. Word renumbers them if duplicates are
found.

---

## 3. Inserted Content: `<w:ins>`

`<w:ins>` wraps runs that were inserted. It appears as a **paragraph-level
child** (inside `<w:p>`) and contains one or more `<w:r>` runs:

```xml
<w:p>
  <w:r><w:t xml:space="preserve">Original text </w:t></w:r>
  <w:ins w:id="1" w:author="Alice" w:date="2026-05-02T10:00:00Z">
    <w:r><w:t>inserted words</w:t></w:r>
  </w:ins>
  <w:r><w:t xml:space="preserve"> after insertion.</w:t></w:r>
</w:p>
```

Rust struct: `InsertedRun { author, date, date_utc, id, inserted_run_choice }`.

`InsertedRun` holds the inserted runs via `inserted_run_choice: Vec<InsertedRunChoice>`.

---

## 4. Deleted Content: `<w:del>`

`<w:del>` wraps runs that were deleted. Deleted runs use `<w:delText>`
instead of `<w:t>` to hold the deleted text:

```xml
<w:p>
  <w:r><w:t xml:space="preserve">Keep this </w:t></w:r>
  <w:del w:id="2" w:author="Bob" w:date="2026-05-02T11:00:00Z">
    <w:r>
      <w:rPr><w:strike/></w:rPr>
      <w:delText>deleted text</w:delText>
    </w:r>
  </w:del>
  <w:r><w:t xml:space="preserve"> and this.</w:t></w:r>
</w:p>
```

Rust struct: `DeletedRun { author, date, date_utc, id, deleted_run_choice }`.

### `<w:delText>` vs `<w:t>`

`<w:delText>` is structurally identical to `<w:t>` — same `xml:space`
attribute, same text content. The distinction is semantic: `<w:delText>`
marks text that is considered deleted and is shown in the tracked-changes
view as strikethrough (if `<w:strike/>` is present in `<w:rPr>`).

Rust struct: `DeletedText { xml_content, space }` (same shape as `Text`).

---

## 5. Run Property Change: `<w:rPrChange>`

When run formatting is changed under tracked changes, the **previous** run
properties are stored in `<w:rPrChange>` inside `<w:rPr>`:

```xml
<w:r>
  <w:rPr>
    <w:b/>                    <!-- current: bold -->
    <w:rPrChange w:id="3" w:author="Alice" w:date="2026-05-02T10:00:00Z">
      <w:rPr/>                <!-- previous: no bold -->
    </w:rPrChange>
  </w:rPr>
  <w:t>Bold text (was normal).</w:t>
</w:r>
```

The `<w:rPr>` child of `<w:rPrChange>` holds the **old** run properties.
The outer `<w:rPr>` holds the **new** (current) properties.

Rust struct: `RunPropertiesChange { author, date, date_utc, id, previous_run_properties }`.

`previous_run_properties` is `Box<PreviousRunProperties>` which wraps the
previous `<w:rPr>` content.

---

## 6. Paragraph Property Change: `<w:pPrChange>`

When paragraph formatting is changed, the previous paragraph properties are
stored in `<w:pPrChange>` inside `<w:pPr>`:

```xml
<w:p>
  <w:pPr>
    <w:jc w:val="center"/>   <!-- current: centered -->
    <w:pPrChange w:id="4" w:author="Bob" w:date="2026-05-02T11:00:00Z">
      <w:pPr/>               <!-- previous: no explicit alignment (left) -->
    </w:pPrChange>
  </w:pPr>
  <w:r><w:t>Centered text (was left-aligned).</w:t></w:r>
</w:p>
```

Rust struct: `ParagraphPropertiesChange { author, date, date_utc, id, paragraph_properties_extended }`.

The child `<w:pPr>` holds the previous properties.

---

## 7. Table Property Changes

Property changes exist for all table levels:

| Element | Location | Rust struct |
|---------|----------|-------------|
| `<w:tblPrChange>` | Inside `<w:tblPr>` | `TablePropertiesChange` |
| `<w:trPrChange>` | Inside `<w:trPr>` | `TableRowPropertiesChange` |
| `<w:tcPrChange>` | Inside `<w:tcPr>` | `TableCellPropertiesChange` |
| `<w:sectPrChange>` | Inside `<w:sectPr>` | `SectionPropertiesChange` |

All follow the same pattern: the change element wraps the **previous** state
of the parent properties element.

---

## 8. Paragraph-Level Insertion and Deletion

A whole paragraph can be marked as inserted or deleted. This is done with
`<w:ins>` / `<w:del>` at the **body level** (not inside a paragraph):

```xml
<w:body>
  <w:ins w:id="5" w:author="Alice" w:date="2026-05-02T10:00:00Z">
    <w:p><w:r><w:t>Newly inserted paragraph.</w:t></w:r></w:p>
  </w:ins>
  <w:del w:id="6" w:author="Bob" w:date="2026-05-02T11:00:00Z">
    <w:p><w:r><w:delText>Deleted paragraph.</w:delText></w:r></w:p>
  </w:del>
</w:body>
```

Rust struct: `Inserted` (body-level, qname `w:CT_TrackChange/w:ins`) vs
`InsertedRun` (paragraph-level, qname `w:CT_RunTrackChange/w:ins`) — they are
**different Rust types** despite sharing the same XML element name.

---

## 9. Move Tracking

Move operations (cut-and-paste) are tracked with `<w:moveFrom>` and
`<w:moveTo>`. They are structurally identical to `<w:del>` and `<w:ins>`,
and add a `<w:moveFromRangeStart>` / `<w:moveFromRangeEnd>` pair for grouping.
Move tracking is out of scope for the current round-trip fixtures but the
markup must be preserved intact.

---

## 10. Round-Trip Gotchas

1. **`w:id` must be globally unique across all revision markup.** Tracked
   changes, comments, and bookmarks share the same ID space. Duplicate IDs
   cause Word to renumber silently on next save.

2. **Deleted text uses `<w:delText>`, not `<w:t>`.** A `<w:del>` that wraps
   runs with `<w:t>` is technically invalid. Some producers write `<w:t>`
   inside `<w:del>` and Word repairs it; round-trip should reproduce whatever
   was in the source.

3. **`<w:ins>` and `<w:del>` at paragraph level vs run level are different
   Rust types.** Rust: `Inserted`/`Deleted` (body children) vs
   `InsertedRun`/`DeletedRun` (paragraph children). The XML element name is the
   same (`w:ins`, `w:del`) but the context determines which type is used.

4. **`w:rPrChange` holds the _old_ properties, not the new ones.** The outer
   `<w:rPr>` is the current state; the inner `<w:rPr>` inside `<w:rPrChange>`
   is what it was before. A common mistake is swapping these.

5. **`w:date` is optional but conventional.** Omitting it is valid. Word always
   writes it. Round-trip must preserve whatever was in the source — do not add
   it if absent, do not remove it if present.

6. **`<w:rPrChange>` must be the last child of `<w:rPr>`.** The XSD places
   `rPrChange` after all other `<w:rPr>` children. Out-of-order placement may
   cause validation failure.

7. **`<w:pPrChange>` must be the last child of `<w:pPr>`.** Same constraint —
   `pPrChange` appears after all other `<w:pPr>` children in the schema
   sequence.

8. **`<w:ins>` children must be runs, not paragraphs (when inside `<w:p>`).** A
   `<w:ins>` appearing inside `<w:p>` uses `CT_RunTrackChange` which holds run
   content. A `<w:ins>` appearing as a direct body child uses `CT_TrackChange`
   which holds block content. The distinction is structural, not just semantic.

9. **`w:author` max length is 255 characters.** Longer author strings are
   truncated by Word. Round-trip should not truncate — preserve as-is.

10. **`<w:del>` wrapping a paragraph mark.** To track deletion of the paragraph
    mark (merging paragraphs), `<w:pPr>` contains `<w:rPr><w:del .../></w:rPr>`
    — a `<w:del>` inside the run properties of the paragraph mark. This is a
    distinct pattern from `<w:del>` wrapping body-level content.

---

## 11. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-TC-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/tracked_changes.docx` | `w:ins` wrapping inserted run; `w:del` wrapping deleted run with `w:delText`; `w:rPrChange` (previous rPr stored); `w:pPrChange` (previous pPr stored) |
