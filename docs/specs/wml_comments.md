# WML Comments — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.13.4 (comments);
ISO/IEC 29500:2016 Part 1 §17.13.4; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

Comments are stored in a separate part (`word/comments.xml`). The body
document uses three marker elements to associate comments with text ranges:

| Marker | Location | Purpose |
|--------|----------|---------|
| `<w:commentRangeStart>` | Paragraph or body child | Opens the highlighted range |
| `<w:commentRangeEnd>` | Paragraph or body child | Closes the highlighted range |
| `<w:commentReference>` | Run child | Anchors the comment balloon |

All three use `w:id` to refer to the matching comment in `word/comments.xml`.

---

## 2. comments.xml Part Structure

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:comments xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:comment w:id="1" w:author="Alice" w:date="2026-05-02T10:00:00Z" w:initials="A">
    <w:p>
      <w:pPr><w:pStyle w:val="CommentText"/></w:pPr>
      <w:r>
        <w:rPr><w:rStyle w:val="CommentReference"/></w:rPr>
        <w:annotationRef/>
      </w:r>
      <w:r><w:t xml:space="preserve"> Comment text here.</w:t></w:r>
    </w:p>
  </w:comment>
</w:comments>
```

---

## 3. CT_Comment Attributes

| Attribute | XML name | Required | Type | Notes |
|-----------|----------|----------|------|-------|
| `id` | `w:id` | **yes** | integer | Unique within the part. Matches `w:id` on range/reference markers. |
| `author` | `w:author` | **yes** | string (≤ 255 chars) | Display name of the commenter. |
| `date` | `w:date` | no | ISO 8601 dateTime | Timestamp when comment was created. |
| `initials` | `w:initials` | no | string (≤ 9 chars) | Commenter's initials (shown in the comment balloon label). |

Rust struct: `Comment { id, author, date, date_utc, initials, comment_choice }`.

### Comment content

The `comment_choice` field holds block-level elements (paragraphs, tables).
Typically a single paragraph styled with `CommentText`, opened by an
`<w:annotationRef/>` run child:

```xml
<w:r>
  <w:rPr><w:rStyle w:val="CommentReference"/></w:rPr>
  <w:annotationRef/>    <!-- auto-number mark for this comment -->
</w:r>
```

`<w:annotationRef/>` is an empty run child (`CT_Empty`) that renders the
auto-generated comment number/letter. It has no attributes.

---

## 4. Range Markers in the Body

Comment range markers are **paragraph-level children** (inside `<w:p>`) or
**body-level children** (direct children of `<w:body>`). The typical placement
is as paragraph children, interleaved with runs:

```xml
<w:p>
  <w:r><w:t xml:space="preserve">Text before the comment. </w:t></w:r>
  <w:commentRangeStart w:id="1"/>
  <w:r><w:t>Commented text.</w:t></w:r>
  <w:commentRangeEnd w:id="1"/>
  <w:r>
    <w:rPr><w:rStyle w:val="CommentReference"/></w:rPr>
    <w:commentReference w:id="1"/>
  </w:r>
  <w:r><w:t xml:space="preserve"> Text after.</w:t></w:r>
</w:p>
```

### CT_MarkupRange attributes

| Attribute | XML name | Notes |
|-----------|----------|-------|
| `id` | `w:id` | Required. Matches the `w:id` of the comment in `comments.xml`. |
| `displaced_by_custom_xml` | `w:displacedByCustomXml` | Optional. Used when custom XML shifts the marker position. |

Rust struct: `CommentRangeStart { id, displaced_by_custom_xml }` /
`CommentRangeEnd { id, displaced_by_custom_xml }`.

---

## 5. Comment Reference in Runs

`<w:commentReference>` is a **run child** (part of `EG_RunInnerContent`).
It marks the exact anchor point — typically placed immediately after
`<w:commentRangeEnd>` in the same paragraph, in a run with the
`CommentReference` character style:

```xml
<w:r>
  <w:rPr><w:rStyle w:val="CommentReference"/></w:rPr>
  <w:commentReference w:id="1"/>
</w:r>
```

### CT_Markup attributes

| Attribute | XML name | Notes |
|-----------|----------|-------|
| `id` | `w:id` | Required. Must match a comment's `w:id` in `comments.xml`. |

Rust struct: `CommentReference { id }`.

---

## 6. Multi-paragraph Comments

A comment can contain multiple paragraphs:

```xml
<w:comment w:id="2" w:author="Bob" w:date="2026-05-02T11:00:00Z">
  <w:p>
    <w:pPr><w:pStyle w:val="CommentText"/></w:pPr>
    <w:r><w:annotationRef/></w:r>
    <w:r><w:t xml:space="preserve"> First paragraph of comment.</w:t></w:r>
  </w:p>
  <w:p>
    <w:pPr><w:pStyle w:val="CommentText"/></w:pPr>
    <w:r><w:t>Second paragraph of comment.</w:t></w:r>
  </w:p>
</w:comment>
```

---

## 7. Cross-paragraph Comment Ranges

A comment range can span multiple paragraphs. The `commentRangeStart` goes
in the first paragraph and `commentRangeEnd` + `commentReference` go in the
last paragraph:

```xml
<w:p>
  <w:commentRangeStart w:id="3"/>
  <w:r><w:t>First paragraph of commented range.</w:t></w:r>
</w:p>
<w:p>
  <w:r><w:t>Second paragraph of commented range.</w:t></w:r>
  <w:commentRangeEnd w:id="3"/>
  <w:r>
    <w:rPr><w:rStyle w:val="CommentReference"/></w:rPr>
    <w:commentReference w:id="3"/>
  </w:r>
</w:p>
```

---

## 8. Round-Trip Gotchas

1. **`commentReference` is a run child, not a paragraph child.** Placing
   `<w:commentReference>` directly inside `<w:p>` is invalid. It must be
   inside `<w:r>`.

2. **`commentRangeEnd` and `commentReference` must use the same `w:id`.** The
   end marker and the reference mark both need to match the comment's ID. A
   common mistake is incrementing only one of them.

3. **`w:id` is shared across all revision elements.** Comment IDs, tracked-
   change IDs, and bookmark IDs all draw from the same ID space. Duplicate IDs
   cause Word to reassign them on next save.

4. **`<w:annotationRef/>` belongs inside the comment, not the body.** This
   empty element marks where the comment's auto-number renders inside the
   comment balloon. It is not the same as `<w:commentReference>` in the body.

5. **Omitting `w:initials` is valid.** Word generates it from the author
   name if absent. Round-trip must not add it if the source omitted it.

6. **`commentRangeStart` and `commentRangeEnd` are paragraph children via
   ParagraphChoice, and body children via BodyChoice.** Placing them in either
   context is valid. Word typically places them as paragraph children.

7. **Comment content must have at least one `<w:p>`.** An empty `<w:comment>`
   (no children) is invalid. Use `<w:p/>` for a comment with no text.

8. **The comments part always needs an Override content-type entry.** Because
   `.xml` is already mapped to `application/xml` by a Default extension entry,
   `word/comments.xml` needs an explicit Override in `[Content_Types].xml`.

9. **Range markers can span across tables and table cells.** A comment range
   may start inside one table cell and end in another. The XSD allows
   `commentRangeStart/End` as body-level children for this case (they appear
   between block-level elements outside any paragraph).

10. **`w:date` on comments uses ISO 8601 with timezone.** The value
    `2026-05-02T10:00:00Z` (UTC) is conventional. Local time with offset
    (`2026-05-02T10:00:00-08:00`) is also valid. Round-trip must preserve
    the original form without normalising to UTC.

---

## 9. Relationship and Content Type URIs

```
Comments relationship type:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments

Comments content type:
application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml
```

---

## 10. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-C-01 | `test-data/wml/comments.docx` | comments.xml with two comments (id=1 with date+initials, id=2 with date only); body with commentRangeStart/End as paragraph children; commentReference as run child; annotationRef inside comment content |
