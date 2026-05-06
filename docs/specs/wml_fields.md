# WML Fields and Hyperlinks — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.16 (fields), §17.13.4
(hyperlinks); ISO/IEC 29500:2016 Part 1 §17.16; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

Fields in DOCX produce computed or dynamic content (page numbers, dates, TOC
entries, cross-references). There are two field syntaxes:

| Syntax | Element | Scope |
|--------|---------|-------|
| Simple field | `<w:fldSimple>` | Paragraph-level wrapper; instr as attribute |
| Complex field | `<w:fldChar>` + `<w:instrText>` | Run-level markers; instr in separate runs |

Both produce the same logical result. Complex fields are more flexible and
are what Word 2007+ generates for most cases.

Hyperlinks (`<w:hyperlink>`) are structurally similar to `<w:fldSimple>` —
paragraph-level wrappers that contain runs — but use a relationship ID rather
than field instruction text.

---

## 2. Complex Field Pattern

A complex field spans multiple runs within a paragraph, bracketed by
`<w:fldChar>` markers:

```xml
<w:p>
  <!-- BEGIN marker -->
  <w:r>
    <w:rPr><w:rStyle w:val="PlaceholderText"/></w:rPr>
    <w:fldChar w:fldCharType="begin"/>
  </w:r>
  <!-- Instruction runs (one or more) -->
  <w:r>
    <w:instrText xml:space="preserve"> PAGE </w:instrText>
  </w:r>
  <!-- SEPARATE marker — separates instruction from cached result -->
  <w:r>
    <w:fldChar w:fldCharType="separate"/>
  </w:r>
  <!-- Cached result runs -->
  <w:r>
    <w:t>1</w:t>
  </w:r>
  <!-- END marker -->
  <w:r>
    <w:fldChar w:fldCharType="end"/>
  </w:r>
</w:p>
```

### ST_FldCharType values

| Value | Meaning |
|-------|---------|
| `begin` | Opens the field. Required. |
| `separate` | Separates instruction from cached result. Optional — omit when there is no cached result. |
| `end` | Closes the field. Required. |

### CT_FldChar attributes

| Attribute | Type | Notes |
|-----------|------|-------|
| `w:fldCharType` | ST_FldCharType | Required. `begin`, `separate`, or `end`. |
| `w:fldLock` | ST_OnOff | Lock field — prevent updates. |
| `w:dirty` | ST_OnOff | Mark field as needing recalculation on next open. |

Rust struct: `FieldChar { field_char_type, field_lock, dirty, field_char_choice }`.

### instrText element

`<w:instrText>` is a **run child** (part of `EG_RunInnerContent`), not a
paragraph child. It carries the field instruction string:

```xml
<w:r>
  <w:instrText xml:space="preserve"> PAGE \* MERGEFORMAT </w:instrText>
</w:r>
```

`xml:space="preserve"` should be present when the instruction has leading or
trailing spaces (which is nearly always).

Rust struct: `FieldCode { space, xml_content }` (same CT_Text as `<w:t>`).

---

## 3. Simple Field

`<w:fldSimple>` is a **paragraph-level child** that wraps the result content:

```xml
<w:p>
  <w:fldSimple w:instr=" DATE \@ &quot;MMMM d, yyyy&quot; ">
    <w:r>
      <w:rPr><w:rStyle w:val="PlaceholderText"/></w:rPr>
      <w:t>May 2, 2026</w:t>
    </w:r>
  </w:fldSimple>
</w:p>
```

### CT_SimpleField attributes

| Attribute | XML name | Notes |
|-----------|----------|-------|
| `instruction` | `w:instr` | Required. Field instruction string (same syntax as `instrText`). |
| `field_lock` | `w:fldLock` | Lock field from updates. |
| `dirty` | `w:dirty` | Mark for recalculation. |

Rust struct: `SimpleField { instruction, field_lock, dirty, field_data, simple_field_choice }`.

The `simple_field_choice` vec holds the result runs (as `SimpleFieldChoice`
variants including `Run`, `Hyperlink`, `FldSimple`, etc.).

---

## 4. Common Field Instructions

Field instructions follow the pattern: `FIELDNAME [switches] [arguments]`.
Leading and trailing spaces are conventional. Switches start with `\`.

### General-purpose fields

| Instruction | Example | Description |
|-------------|---------|-------------|
| `PAGE` | ` PAGE ` | Current page number |
| `NUMPAGES` | ` NUMPAGES ` | Total page count |
| `DATE` | ` DATE \@ "MMMM d, yyyy" ` | Current date |
| `TIME` | ` TIME \@ "h:mm AM/PM" ` | Current time |
| `TITLE` | ` TITLE ` | Document title (from core properties) |
| `AUTHOR` | ` AUTHOR ` | Document author |
| `FILENAME` | ` FILENAME \p ` | File name (\p includes full path) |

### Reference and TOC fields

| Instruction | Example | Description |
|-------------|---------|-------------|
| `TOC` | ` TOC \o "1-3" ` | Table of contents (\o = outline levels) |
| `REF` | ` REF bookmark1 ` | Cross-reference to bookmark |
| `PAGEREF` | ` PAGEREF bookmark1 \h ` | Page number of bookmark (\h = hyperlink) |
| `HYPERLINK` | ` HYPERLINK "https://example.com" ` | External hyperlink as complex field |
| `STYLEREF` | ` STYLEREF "Heading 1" ` | Text of nearest styled paragraph |
| `SEQ` | ` SEQ Figure \* ARABIC ` | Sequential counter |

### Common switches

| Switch | Meaning |
|--------|---------|
| `\*` | Format: `\* MERGEFORMAT` (preserve manual formatting), `\* ARABIC`, `\* roman`, `\* ORDINAL` |
| `\@` | Date/time picture: `\@ "yyyy"` |
| `\#` | Numeric picture: `\# "0.00"` |
| `\!` | Lock result format |
| `\h` | Hyperlink (in PAGEREF, REF) |

---

## 5. Hyperlinks: CT_Hyperlink

`<w:hyperlink>` is a **paragraph-level child** (like `<w:fldSimple>`) that
wraps runs with a link target. It is simpler than a HYPERLINK complex field.

```xml
<!-- External hyperlink — requires r:id relationship -->
<w:hyperlink r:id="rId2" w:history="1">
  <w:r>
    <w:rPr><w:rStyle w:val="Hyperlink"/></w:rPr>
    <w:t>Click here</w:t>
  </w:r>
</w:hyperlink>

<!-- Internal bookmark hyperlink — no relationship needed -->
<w:hyperlink w:anchor="section1" w:history="1">
  <w:r>
    <w:rPr><w:rStyle w:val="Hyperlink"/></w:rPr>
    <w:t>Go to section 1</w:t>
  </w:r>
</w:hyperlink>
```

### CT_Hyperlink attributes

| Attribute | XML name | Notes |
|-----------|----------|-------|
| `id` | `r:id` | Relationship ID for external URL. Requires `xmlns:r`. |
| `anchor` | `w:anchor` | Bookmark name for internal link. |
| `history` | `w:history` | Add to viewed-hyperlinks list. |
| `target_frame` | `w:tgtFrame` | Browser target frame (e.g., `_blank`). |
| `tooltip` | `w:tooltip` | Hover tooltip text. |
| `doc_location` | `w:docLocation` | Location within the target document. |

Rust struct: `Hyperlink { target_frame, tooltip, doc_location, history, anchor, id, hyperlink_choice }`.

### External hyperlink relationship

External hyperlinks need a relationship in `word/_rels/document.xml.rels`:

```xml
<Relationship Id="rId2"
  Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
  Target="https://example.com"
  TargetMode="External"/>
```

The `TargetMode="External"` attribute is **required** for external links.
Internal (anchor) hyperlinks use no relationship.

---

## 6. Nested Fields

Complex fields and simple fields can be nested. A common case is a PAGEREF
inside a TOC entry:

```xml
<w:r><w:fldChar w:fldCharType="begin"/></w:r>
<w:r><w:instrText xml:space="preserve"> PAGEREF _Toc1 \h </w:instrText></w:r>
<w:r><w:fldChar w:fldCharType="separate"/></w:r>
<w:r><w:t>5</w:t></w:r>
<w:r><w:fldChar w:fldCharType="end"/></w:r>
```

Nested complex fields stack their markers. The outermost `end` closes the most
recent unclosed `begin`.

---

## 7. fldLock and dirty

- **`w:fldLock="1"`** on `fldChar type="begin"` or `fldSimple`: prevents the
  field from being updated when the user presses F9 or when opening the
  document. Use for fields whose value must be preserved exactly.

- **`w:dirty="1"`** on `fldChar type="begin"` or `fldSimple`: signals that the
  cached result is stale and should be refreshed on next open. Applications
  that write fields for the first time (no cached value yet) should set
  `dirty="1"` to ensure the value is computed on first open.

---

## 8. Round-Trip Gotchas

1. **`instrText` belongs inside a run, not directly in a paragraph.** A common
   mistake is writing `<w:instrText>` as a paragraph child. It must be inside
   `<w:r>`.

2. **`instrText` spaces matter.** The instruction ` PAGE ` (with leading/
   trailing spaces) is conventional. `PAGE` (no spaces) parses the same but
   `xml:space="preserve"` must be present if spaces are wanted.

3. **The separate marker is optional.** A field with no cached result (e.g., a
   freshly inserted field with `dirty="1"`) has only `begin` and `end` markers.
   The `separate` marker is absent when there is no cached result.

4. **All three runs for a complex field are required for a complete field.**
   Missing the `end` marker makes the document invalid. Word will often repair
   it but the result is unpredictable.

5. **External hyperlinks need `TargetMode="External"` in the relationship.**
   Without it, some processors treat the target as an internal part name.

6. **`r:id` requires `xmlns:r` on `<w:document>`.** The `r:id` attribute on
   `<w:hyperlink>` uses the relationships namespace. Declaring it only locally
   on the element is valid XML but problematic for some processors.

7. **Internal anchor hyperlinks (`w:anchor`) need no relationship.** Only
   external links use `r:id`. Internal links use `w:anchor` with the bookmark
   name. Do not add a relationship for bookmark-only hyperlinks.

8. **`w:history="1"` is conventional but optional.** Word sets it to mark the
   link as visited. Round-trip must preserve it if present; do not add it when
   absent.

9. **`w:fldSimple w:instr` must XML-escape quotes.** The instruction
   ` DATE \@ "MMMM d, yyyy" ` contains double-quotes that must be escaped as
   `&quot;` in attribute values: `w:instr=" DATE \@ &quot;MMMM d, yyyy&quot; "`.

10. **Run formatting applies to the cached result, not the instruction.** Run
    properties on the runs between `separate` and `end` control how the cached
    value is rendered. Run properties on instruction runs (between `begin` and
    `separate`) are usually kept minimal.

---

## 9. Relationship and Content Type URIs

```
Hyperlink relationship type:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink
```

No special content type is needed — hyperlinks are embedded in the document
part, not stored as separate parts.

---

## 10. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-FLD-01 | `test-data/wml/fields_complex.docx` | Complex PAGE field (begin/instrText/separate/result/end); complex NUMPAGES field; `dirty="1"` on a field with no cached result; `xmlns:r` on document |
| WML-FLD-02 | `test-data/wml/fields_hyperlink.docx` | External hyperlink with `r:id` + `TargetMode=External`; internal anchor hyperlink (`w:anchor`); `w:fldSimple` with DATE instruction |
