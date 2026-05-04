# WML Bookmarks and Hyperlinks — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.13.6 (bookmarks),
§17.13.4 (hyperlinks); ISO/IEC 29500:2016 Part 1 §17.13.6, §17.13.4; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

**Note on hyperlinks:** External and internal hyperlinks (`<w:hyperlink>`) were
also covered in `docs/specs/wml_fields.md` §5. This spec focuses on bookmarks
as named anchor targets, and adds the cross-reference between bookmark names
and `w:anchor` hyperlinks.

---

## 1. Overview

Bookmarks mark named ranges within a document. They are used as:
- Navigation targets (`<w:hyperlink w:anchor="name">`)
- Cross-reference targets (`REF name` and `PAGEREF name` fields)
- Table-of-contents entry anchors

A bookmark is defined by a pair of markers:

| Element | Type | Purpose |
|---------|------|---------|
| `<w:bookmarkStart>` | CT_Bookmark | Opens the named range; carries the `w:name` |
| `<w:bookmarkEnd>` | CT_MarkupRange | Closes the range; `w:id` matches bookmarkStart |

Both appear as **paragraph-level children** (inside `<w:p>` via
`ParagraphChoice`) or **body-level children** (direct children of `<w:body>`
via `BodyChoice`).

---

## 2. bookmarkStart: CT_Bookmark

```xml
<w:bookmarkStart w:id="10" w:name="section_overview"/>
```

### Attributes

| Attribute | XML name | Required | Notes |
|-----------|----------|----------|-------|
| `id` | `w:id` | **yes** | Integer, unique across all revision markup in the document. |
| `name` | `w:name` | **yes** | Bookmark name. Max 40 characters. Must be unique within the document. |
| `column_first` | `w:colFirst` | no | First column index for table-column bookmarks (0-based). |
| `column_last` | `w:colLast` | no | Last column index for table-column bookmarks (0-based, inclusive). |
| `displaced_by_custom_xml` | `w:displacedByCustomXml` | no | Tracks displacement by custom XML. |

Rust struct: `BookmarkStart { name, column_first, column_last, displaced_by_custom_xml, id }`.

**Bookmark names:**
- Maximum 40 characters.
- Case-sensitive.
- Must not contain spaces (Word replaces spaces with underscores on save).
- Must be unique per document; duplicate names cause Word to keep only the
  last definition.
- The name `_GoBack` is reserved by Word for tracking the last edit position.

---

## 3. bookmarkEnd: CT_MarkupRange

```xml
<w:bookmarkEnd w:id="10"/>
```

### Attributes

| Attribute | XML name | Required | Notes |
|-----------|----------|----------|-------|
| `id` | `w:id` | **yes** | Must match the `w:id` of the corresponding `bookmarkStart`. |
| `displaced_by_custom_xml` | `w:displacedByCustomXml` | no | Tracks displacement. |

Rust struct: `BookmarkEnd { displaced_by_custom_xml, id }`.

---

## 4. Placement in the Document

### Inline bookmark (within a paragraph)

Both markers are paragraph children, placed around the text being bookmarked:

```xml
<w:p>
  <w:r><w:t xml:space="preserve">See </w:t></w:r>
  <w:bookmarkStart w:id="10" w:name="appendix_a"/>
  <w:r><w:t>Appendix A</w:t></w:r>
  <w:bookmarkEnd w:id="10"/>
  <w:r><w:t xml:space="preserve"> for details.</w:t></w:r>
</w:p>
```

### Heading bookmark (whole paragraph)

For a heading that is a cross-reference target, both markers typically wrap
the entire paragraph content. The `bookmarkStart` goes before any runs and
`bookmarkEnd` goes after:

```xml
<w:p>
  <w:pPr><w:pStyle w:val="Heading1"/></w:pPr>
  <w:bookmarkStart w:id="11" w:name="intro"/>
  <w:r><w:t>Introduction</w:t></w:r>
  <w:bookmarkEnd w:id="11"/>
</w:p>
```

### Point bookmark (zero-width)

`bookmarkStart` and `bookmarkEnd` can be adjacent (or even on the same
position) to mark a single insertion point rather than a range:

```xml
<w:p>
  <w:bookmarkStart w:id="12" w:name="target"/>
  <w:bookmarkEnd w:id="12"/>
  <w:r><w:t>Bookmark is at the start of this paragraph.</w:t></w:r>
</w:p>
```

---

## 5. Hyperlinks Targeting Bookmarks

An internal hyperlink uses `w:anchor` to name a bookmark target — no
relationship is needed:

```xml
<w:hyperlink w:anchor="appendix_a" w:history="1">
  <w:r>
    <w:rPr><w:rStyle w:val="Hyperlink"/></w:rPr>
    <w:t>Go to Appendix A</w:t>
  </w:r>
</w:hyperlink>
```

An external hyperlink uses `r:id` (relationship ID, with `TargetMode="External"`)
and requires `xmlns:r` on `<w:document>`:

```xml
<w:hyperlink r:id="rId2" w:history="1">
  <w:r>
    <w:rPr><w:rStyle w:val="Hyperlink"/></w:rPr>
    <w:t>Visit example.com</w:t>
  </w:r>
</w:hyperlink>
```

For full hyperlink attribute details see `docs/specs/wml_fields.md` §5.

---

## 6. Table-Column Bookmarks

`w:colFirst` and `w:colLast` mark a range of columns within a table. When
both are present, the bookmark covers the specified column range across all
rows rather than a text range. These are used for column-level formatting
references and are uncommon in general documents.

---

## 7. Round-Trip Gotchas

1. **`w:id` is shared with tracked-change and comment IDs.** Bookmark IDs,
   comment IDs, and tracked-change IDs must all be unique across the document.
   When adding bookmarks programmatically, scan all existing IDs first.

2. **`w:name` max length is 40 characters.** Longer names are truncated by
   Word. The SDK enforces `string_length(max = 40)`.

3. **Spaces in bookmark names are not allowed.** Word replaces spaces with
   `_` on save. Round-trip must preserve the original name as stored; do not
   convert.

4. **`bookmarkStart` and `bookmarkEnd` do not have to be in the same
   paragraph.** A cross-paragraph bookmark is valid. The start and end markers
   simply need matching `w:id` values.

5. **`bookmarkEnd` comes after `bookmarkStart` in document order.** The XSD
   does not enforce this, but reversing the order produces undefined behaviour
   in processors.

6. **`_GoBack` bookmark.** Word inserts a `_GoBack` bookmark at the last
   cursor position. Round-trip must preserve it without modification; do not
   add it if it was absent.

7. **Bookmark names are case-sensitive.** `AppendixA` and `appendixa` are two
   different bookmarks. The `w:anchor` value on a hyperlink must match the
   case exactly.

8. **`bookmarkStart` must precede all run content it covers.** As a paragraph
   child, `bookmarkStart` should appear before the first run of the bookmarked
   text. Word may silently reorder out-of-sequence markers on save.

9. **Zero-width (point) bookmarks are valid and common.** TOC entries often
   use adjacent `bookmarkStart`/`bookmarkEnd` with no content between them.
   Round-trip must not collapse these.

10. **Column-range bookmarks (`colFirst`/`colLast`) apply only inside
    `<w:tbl>`.** Using column attributes on a paragraph-context bookmark has
    no defined meaning and is ignored by most processors.

---

## 8. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-BM-01 | `test-data/wml/bookmarks.docx` | Inline bookmark wrapping text range; heading-style bookmark wrapping full paragraph; zero-width point bookmark; internal anchor hyperlink targeting the inline bookmark |
