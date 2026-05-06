# WML Footnotes and Endnotes — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.11 (footnotes/endnotes);
ISO/IEC 29500:2016 Part 1 §17.11; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

Footnotes and endnotes are stored in separate parts (`word/footnotes.xml` and
`word/endnotes.xml`). Each part contains a sequence of note elements; the body
document references them by ID.

| Feature | Footnotes | Endnotes |
|---------|-----------|----------|
| Part file | `word/footnotes.xml` | `word/endnotes.xml` |
| Root element | `<w:footnotes>` (CT_Footnotes) | `<w:endnotes>` (CT_Endnotes) |
| Note element | `<w:footnote>` | `<w:endnote>` |
| Reference in body | `<w:footnoteReference>` | `<w:endnoteReference>` |
| Auto-number mark in note | `<w:footnoteRef/>` | `<w:endnoteRef/>` |
| Relationship type | `.../relationships/footnotes` | `.../relationships/endnotes` |

Full relationship URIs are in §10.

---

## 2. Footnote/Endnote Part Structure

```xml
<!-- word/footnotes.xml -->
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:footnotes xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">

  <!-- Special footnotes — always present, IDs -1 and 0 by convention -->
  <w:footnote w:type="separator" w:id="-1">
    <w:p><w:r><w:separator/></w:r></w:p>
  </w:footnote>
  <w:footnote w:type="continuationSeparator" w:id="0">
    <w:p><w:r><w:continuationSeparator/></w:r></w:p>
  </w:footnote>

  <!-- Normal user-defined footnote -->
  <w:footnote w:id="1">
    <w:p>
      <w:pPr><w:pStyle w:val="FootnoteText"/></w:pPr>
      <w:r>
        <w:rPr><w:rStyle w:val="FootnoteReference"/></w:rPr>
        <w:footnoteRef/>
      </w:r>
      <w:r><w:t xml:space="preserve"> Footnote content here.</w:t></w:r>
    </w:p>
  </w:footnote>

</w:footnotes>
```

Endnotes follow the identical structure with `<w:endnotes>` /
`<w:endnote>` / `<w:endnoteRef/>`.

---

## 3. Individual Note: CT_FtnEdn

### Attributes

| Attribute | XML name | Type | Notes |
|-----------|----------|------|-------|
| `r#type` | `w:type` | ST_FtnEdn | Note type. Omit (or `normal`) for regular notes. |
| `id` | `w:id` | integer | Required. Unique within the part. |

### ST_FtnEdn values

| Value | Meaning |
|-------|---------|
| `normal` | Regular footnote/endnote (default; `w:type` may be omitted) |
| `separator` | Horizontal rule separating notes from body text |
| `continuationSeparator` | Separator used when notes continue from a previous page |
| `continuationNotice` | Message indicating the note continues on the next page |

**ID conventions:**
- Special notes use negative or zero IDs (Word uses `-1` for separator, `0`
  for continuationSeparator).
- Normal notes use positive integers starting from `1`.
- IDs must be unique within the part.

Rust struct: `Footnote { r#type, id, footnote_choice }` /
`Endnote { r#type, id, endnote_choice }`.

Rust enum: `FootnoteEndnoteValues` — variants `Normal`, `Separator`,
`ContinuationSeparator`, `ContinuationNotice`.

### Note content

The content of each note is block-level elements (paragraphs, tables), same as
`<w:body>` content. The typical pattern is a single paragraph styled with
`FootnoteText` / `EndnoteText` style, beginning with the auto-number mark.

---

## 4. Anchor in Body: footnoteReference / endnoteReference

A footnote is anchored in the body by placing `<w:footnoteReference>` inside a
run. The run carries a superscript style (`FootnoteReference`):

```xml
<w:p>
  <w:r><w:t xml:space="preserve">Body text with a footnote</w:t></w:r>
  <w:r>
    <w:rPr><w:rStyle w:val="FootnoteReference"/></w:rPr>
    <w:footnoteReference w:id="1"/>
  </w:r>
  <w:r><w:t>.</w:t></w:r>
</w:p>
```

### CT_FtnEdnRef attributes

| Attribute | XML name | Notes |
|-----------|----------|-------|
| `id` | `w:id` | Required. Must match the `w:id` of a note in the footnotes/endnotes part. |
| `custom_mark_follows` | `w:customMarkFollows` | The next run contains a custom reference mark (overrides auto-numbering). |

Rust struct: `FootnoteReference { custom_mark_follows, id }` /
`EndnoteReference { custom_mark_follows, id }`.

Both are **run children** — they appear inside `<w:r>` as part of
`EG_RunInnerContent`.

---

## 5. Auto-Number Mark in Note Content: footnoteRef / endnoteRef

Inside the note's paragraph, the auto-generated reference mark is placed with
the empty element `<w:footnoteRef/>` or `<w:endnoteRef/>`:

```xml
<w:footnote w:id="1">
  <w:p>
    <w:r>
      <w:rPr><w:rStyle w:val="FootnoteReference"/></w:rPr>
      <w:footnoteRef/>   <!-- renders as "1" in superscript -->
    </w:r>
    <w:r><w:t xml:space="preserve"> Note text.</w:t></w:r>
  </w:p>
</w:footnote>
```

These are **run children** (empty element, `CT_Empty` type). They have no
attributes.

---

## 6. Separator and Continuation Elements

Special notes use `<w:separator/>` and `<w:continuationSeparator/>` as run
children — also empty elements:

```xml
<w:footnote w:type="separator" w:id="-1">
  <w:p><w:r><w:separator/></w:r></w:p>
</w:footnote>
<w:footnote w:type="continuationSeparator" w:id="0">
  <w:p><w:r><w:continuationSeparator/></w:r></w:p>
</w:footnote>
```

Processors that do not want visual separators can include these special
footnotes with an empty paragraph: `<w:footnote w:type="separator" w:id="-1"><w:p/></w:footnote>`.

---

## 7. Section-Level Note Properties: footnotePr / endnotePr

`<w:footnotePr>` and `<w:endnotePr>` inside `<w:sectPr>` override the
document-wide defaults for that section:

```xml
<w:sectPr>
  <w:footnotePr>
    <w:pos w:val="beneathText"/>
    <w:numFmt w:val="lowerRoman"/>
    <w:numStart w:val="1"/>
    <w:numRestart w:val="eachSect"/>
  </w:footnotePr>
  …
</w:sectPr>
```

### Footnote position values (ST_FtnPos)

| Value | Meaning |
|-------|---------|
| `pageBottom` | Bottom of the page (default) |
| `beneathText` | Immediately below the last text line |
| `sectEnd` | End of the section (collects all section footnotes) |
| `docEnd` | End of the document (makes them act like endnotes) |

### Endnote position values (ST_EdnPos)

| Value | Meaning |
|-------|---------|
| `sectEnd` | End of the section (default) |
| `docEnd` | End of the document |

### Number restart values (ST_RestartNumber)

| Value | Meaning |
|-------|---------|
| `continuous` | Never restart — numbers run across sections (default) |
| `eachSect` | Restart at the start of each section |
| `eachPage` | Restart at the start of each page (footnotes only) |

---

## 8. Document-Wide Note Settings

Document-wide footnote/endnote defaults live in `word/settings.xml` as
children of the `<w:settings>` root:

```xml
<w:settings>
  <w:footnotePr>
    <w:numFmt w:val="decimal"/>
    <w:footnote w:id="-1"/>   <!-- reference to separator special note -->
    <w:footnote w:id="0"/>    <!-- reference to continuationSeparator -->
  </w:footnotePr>
  <w:endnotePr>
    <w:numFmt w:val="lowerRoman"/>
  </w:endnotePr>
  …
</w:settings>
```

The `<w:footnote w:id="N"/>` children of document-wide `footnotePr` reference
the IDs of the special separator/continuation notes in `word/footnotes.xml`.

---

## 9. Round-Trip Gotchas

1. **Special footnotes must be present.** Word always writes the separator
   (`id="-1"`) and continuationSeparator (`id="0"`) special notes, even if
   empty. Many processors reject a `word/footnotes.xml` that lacks them or use
   unexpected IDs.

2. **`w:footnoteRef` and `w:footnoteReference` are different elements.** The
   former (`w:footnoteRef`) goes inside the note to mark the rendered number.
   The latter (`w:footnoteReference w:id="N"`) goes in the body to reference
   the note. Mixing them up produces invisible or duplicated marks.

3. **`footnoteReference` is a run child, not a paragraph child.** It must be
   inside `<w:r>`. Placing it directly in `<w:p>` is invalid.

4. **IDs must be signed integers.** The `w:id` type is ST_DecimalNumber which
   maps to a signed integer. Special notes use `-1` and `0`. Unsigned integer
   parsers will fail on `-1`.

5. **No relationship is needed between the document and each note.** The entire
   footnotes part is referenced by a single relationship (`rId` to
   `word/footnotes.xml`). All notes within that part are addressed by the
   `w:id` attribute, not by separate relationships.

6. **Content types require Override entries.** Footnotes and endnotes parts
   must be declared with `<Override>` in `[Content_Types].xml` because `.xml`
   is already mapped to `application/xml` by a `<Default>` entry.

7. **Section `footnotePr` takes precedence over document `footnotePr`.** When
   both exist, the section-level properties win for that section's notes. Round-
   trip must preserve both levels without merging them.

8. **`numFmt` in `footnotePr` applies to the reference mark style.** The
   actual superscript character (1, 2, 3 or i, ii, iii) is controlled by the
   numbering format in `footnotePr`, not by the `FootnoteReference` character
   style.

9. **`w:continuationSeparator` is a run child, not a text element.** It renders
   as a full-width horizontal rule. Some producers mistakenly use `<w:t>` text
   for the separator content.

10. **Empty `<w:p/>` inside a note is valid.** A note containing only
    `<w:footnote w:id="1"><w:p/></w:footnote>` is a blank footnote with no
    content. It is used for continuation notice notes that should display
    nothing.

---

## 10. Relationship and Content Type URIs

```
Footnotes relationship type:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes

Endnotes relationship type:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes

Footnotes content type:
application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml

Endnotes content type:
application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml
```

---

## 11. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-N-01 | `test-data/wml/footnotes.docx` | footnotes.xml with special separator notes (id=-1, id=0) and two normal footnotes; body with footnoteReference marks inside runs; Override content type entry |
| WML-N-02 | `test-data/wml/endnotes.docx` | endnotes.xml with special separator notes and one normal endnote; body with endnoteReference mark |
