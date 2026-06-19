# WML Headers, Footers, and Section Properties — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.10 (headers/footers),
§17.6 (sections); ISO/IEC 29500:2016 Part 1 §17.10, §17.6; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

Headers and footers in DOCX are stored as separate parts (`word/header*.xml`,
`word/footer*.xml`), each referenced from `word/document.xml` via relationships
listed in `word/_rels/document.xml.rels`. The references appear inside
`<w:sectPr>`, which also holds page-size, margin, column, and page-number
settings.

### Part relationship types

| Part | Relationship type URI | Content type |
|------|-----------------------|--------------|
| Header | `.../relationships/header` | `…wordprocessingml.header+xml` |
| Footer | `.../relationships/footer` | `…wordprocessingml.footer+xml` |

(Full URIs in §11.)

### Header/footer root elements

| Element | Description |
|---------|-------------|
| `<w:hdr>` | Header part root (CT_HdrFtr) |
| `<w:ftr>` | Footer part root (CT_HdrFtr) |

Both contain block-level content (paragraphs, tables, etc.) — the same children
as `<w:body>` minus `<w:sectPr>`.

---

## 2. Section Properties: CT_SectPr

`<w:sectPr>` appears either as the **last child of `<w:body>`** (section
properties for the final section) or inside a paragraph's `<w:pPr>` (marks
the end of a mid-document section).

```xml
<w:sectPr>
  <!-- header/footer references (before other children) -->
  <w:headerReference w:type="default" r:id="rId1"/>
  <w:footerReference w:type="default" r:id="rId2"/>
  <!-- section content children -->
  <w:type w:val="continuous"/>
  <w:pgSz w:w="12240" w:h="15840"/>
  <w:pgMar w:top="1440" w:right="1440" w:bottom="1440"
           w:left="1440" w:header="720" w:footer="720" w:gutter="0"/>
  <w:pgNumType w:fmt="decimal" w:start="1"/>
  <w:cols w:space="720"/>
  <w:titlePg/>
</w:sectPr>
```

### Key child elements

| Element | Notes |
|---------|-------|
| `headerReference` | 0–3 per section (one per type: default, even, first) |
| `footerReference` | 0–3 per section |
| `type` | Section break type (CT_SectType) |
| `pgSz` | Page size (width, height, orientation) |
| `pgMar` | Page margins (top/right/bottom/left/header/footer/gutter) |
| `pgNumType` | Page number format and starting value |
| `cols` | Column count and spacing |
| `titlePg` | CT_OnOff — enable first-page header/footer distinction |
| `textDirection` | Text direction |
| `vAlign` | Vertical page alignment |
| `docGrid` | Document grid for snap-to-grid |
| `pgBorders` | Page borders |
| `lnNumType` | Line numbering |
| `footnotePr` / `endnotePr` | Footnote/endnote section settings |

### sectPr attributes

| Attribute | Notes |
|-----------|-------|
| `rsidR` | Revision ID for section addition |
| `rsidRPr` | Revision ID for section formatting |
| `rsidDel` | Revision ID for section deletion |
| `rsidSect` | Revision ID for section properties |

---

## 3. Header/Footer References

`<w:headerReference>` and `<w:footerReference>` each have two attributes:

| Attribute | Notes |
|-----------|-------|
| `w:type` | ST_HdrFtr: `default`, `even`, or `first` |
| `r:id` | Relationship ID pointing to the header or footer part |

The `r:` prefix requires `xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"` declared on `<w:document>`.

### ST_HdrFtr values

| Value | Page type |
|-------|-----------|
| `default` | Odd-numbered pages (and all pages when even/odd is not enabled) |
| `even` | Even-numbered pages (only used when `evenAndOddHeaders` is on) |
| `first` | First page of the section (only used when `titlePg` is set) |

A section may have up to **six references** total: three header types × three
footer types. Missing types inherit from the previous section (or are empty if
no previous section).

---

## 4. Enabling First-Page and Even/Odd Headers

### First-page header/footer: `<w:titlePg/>`

Add `<w:titlePg/>` to `<w:sectPr>` and provide both a `type="default"` and a
`type="first"` reference:

```xml
<w:sectPr>
  <w:headerReference w:type="default" r:id="rId1"/>
  <w:headerReference w:type="first"   r:id="rId2"/>
  <w:footerReference w:type="default" r:id="rId3"/>
  <w:titlePg/>
  …
</w:sectPr>
```

If `<w:titlePg/>` is absent, the `first` reference is ignored.

### Even/odd headers: `<w:evenAndOddHeaders/>` in settings

Even/odd header support is a **document-level** setting, not per-section. It
lives in `word/settings.xml` (the `<w:settings>` root):

```xml
<w:settings>
  <w:evenAndOddHeaders/>
  …
</w:settings>
```

With `evenAndOddHeaders` enabled, both `type="default"` (odd pages) and
`type="even"` (even pages) references are active. Without it, only `default`
is used regardless of `even` references present.

---

## 5. Header/Footer Part Structure

A header or footer part has the root element `<w:hdr>` or `<w:ftr>`. The
content is block-level elements, identical to the body:

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:hdr xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:p>
    <w:pPr><w:jc w:val="center"/></w:pPr>
    <w:r><w:t>Page Header</w:t></w:r>
  </w:p>
</w:hdr>
```

Headers and footers can contain:
- Text paragraphs (with all run/paragraph formatting)
- Tables
- Images (inline or floating)
- Fields (e.g., `PAGE`, `NUMPAGES`, `DATE`)
- Shapes

An **empty paragraph** is required for an intentionally blank header/footer:
```xml
<w:hdr …><w:p/></w:hdr>
```

---

## 6. Page Size: CT_PageSz

```xml
<w:pgSz w:w="12240" w:h="15840"/>
<w:pgSz w:w="11906" w:h="16838" w:orient="portrait"/>   <!-- A4 -->
<w:pgSz w:w="15840" w:h="12240" w:orient="landscape"/>  <!-- US Letter landscape -->
```

### Attributes

| Attribute | Type | Notes |
|-----------|------|-------|
| `w:w` | twips | Page width. Range 1–31680. |
| `w:h` | twips | Page height. Range 1–31680. |
| `w:orient` | `portrait` or `landscape` | Orientation hint; `w`/`h` values must be consistent. |
| `w:code` | integer | Printer-specific paper code. |

### Common page sizes

| Size | Width (twips) | Height (twips) |
|------|---------------|----------------|
| US Letter | 12240 | 15840 |
| US Legal | 12240 | 20160 |
| A4 | 11906 | 16838 |
| A3 | 16838 | 23814 |
| US Letter landscape | 15840 | 12240 |

---

## 7. Page Margins: CT_PageMar

```xml
<w:pgMar w:top="1440" w:right="1800" w:bottom="1440"
         w:left="1800" w:header="720" w:footer="720" w:gutter="0"/>
```

All values in **twips**.

| Attribute | Type | Notes |
|-----------|------|-------|
| `w:top` | signed twips | Top margin (can be negative for bleed) |
| `w:right` | twips | Right margin |
| `w:bottom` | signed twips | Bottom margin |
| `w:left` | twips | Left margin |
| `w:header` | twips | Distance from top of page to top of header |
| `w:footer` | twips | Distance from bottom of page to bottom of footer |
| `w:gutter` | twips | Extra margin for binding |

### Common margin values

| Inches | Twips |
|--------|-------|
| 0.5" | 720 |
| 1.0" | 1440 |
| 1.25" | 1800 |
| 1.5" | 2160 |

**Header/footer margin semantics:** The `header` value is the distance from the
**page edge** (not the text area) to the start of the header text. Similarly,
`footer` is the distance from the **bottom page edge** to the end of the footer.
The header text area sits between the top of the page and the top text margin.

---

## 8. Section Break Types

`<w:type w:val="…"/>` controls how the section starts. If absent, `nextPage`
is the default.

| Value | Meaning |
|-------|---------|
| `nextPage` | Section starts at the top of the next page (default) |
| `continuous` | Section starts inline, no page break |
| `evenPage` | Section starts at the top of the next even-numbered page |
| `oddPage` | Section starts at the top of the next odd-numbered page |
| `nextColumn` | Section starts at the top of the next column |

**Continuous sections** share the same page as the previous section and can
have different column layouts. The `<w:sectPr>` for a continuous section
typically omits `pgSz` and `pgMar` (inherits from the next explicit section).

---

## 9. Page Numbering: CT_PageNumber

```xml
<w:pgNumType w:fmt="decimal" w:start="1"/>
```

| Attribute | Notes |
|-----------|-------|
| `w:fmt` | Number format: `decimal`, `upperRoman`, `lowerRoman`, `upperLetter`, `lowerLetter`, etc. |
| `w:start` | Starting page number for this section |
| `w:chapStyle` | Style ID for chapter number |
| `w:chapSep` | Separator character between chapter and page numbers |

---

## 10. Round-Trip Gotchas

1. **headerReference/footerReference must come before other sectPr children.**
   The XSD defines them via `EG_HdrFtrReferences` which precedes
   `EG_SectPrContents` in the sequence. Out-of-order placements may fail in
   strict parsers.

2. **`r:id` requires the `r:` namespace declaration.** The `r:id` attribute on
   `<w:headerReference>` uses the relationships namespace prefix. If
   `xmlns:r="…"` is not declared on `<w:document>`, the attribute is invalid.

3. **Inherited headers.** A section with no `headerReference` inherits the
   header from the **previous** section. An explicit empty-paragraph header
   (a reference to a part containing only `<w:p/>`) is different from no
   reference — the former explicitly blanks the header, the latter inherits.

4. **`titlePg` presence vs value.** `<w:titlePg/>` (no `val` attribute) means
   "on". `<w:titlePg w:val="0"/>` explicitly turns it off. Omission inherits
   from the style. Round-trip must not add or remove `val="0"`.

5. **Even/odd headers need settings.xml.** Adding `even` header references
   to `sectPr` without `<w:evenAndOddHeaders/>` in `word/settings.xml` has
   no visible effect — Word ignores even references unless the document-level
   switch is on.

6. **Header/footer parts need their own `word/_rels/` entries.** Header and
   footer parts that contain images, hyperlinks, or other relationship-backed
   content need their own `.rels` files at `word/_rels/header1.xml.rels`, etc.

7. **Content type via Override.** Header and footer parts should be declared
   with `<Override PartName="/word/header1.xml" ContentType="…header+xml"/>`
   in `[Content_Types].xml`, not via `<Default>` extension (since `.xml` is
   already mapped to `application/xml`).

8. **Empty header must have at least one `<w:p>`.** A header or footer part
   with no block-level children is invalid. Use `<w:p/>` for a blank header.

9. **Page size and orientation must be consistent.** If `w:orient="landscape"`,
   `w:w` should be the larger value and `w:h` the smaller. Some producers swap
   `w` and `h` and set `orient="landscape"` instead — both are accepted but
   may render differently in strict mode.

10. **`pgMar` header/footer values are from page edge, not text area.** A
    common mistake is treating `header` as the gap between the top text margin
    and the header — it is the gap from the physical **top of the page**. The
    effective header height = `top margin − header value`.

---

## 11. Relationship and Content Type URIs

```
Header relationship type:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/header

Footer relationship type:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer

Header content type:
application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml

Footer content type:
application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml
```

---

## 12. Minimal Structures

### Document with default header and footer

```xml
<!-- word/_rels/document.xml.rels -->
<Relationship Id="rId1" Type="…/header" Target="header1.xml"/>
<Relationship Id="rId2" Type="…/footer" Target="footer1.xml"/>

<!-- word/document.xml (body tail) -->
<w:sectPr>
  <w:headerReference w:type="default" r:id="rId1"/>
  <w:footerReference w:type="default" r:id="rId2"/>
  <w:pgSz w:w="12240" w:h="15840"/>
  <w:pgMar w:top="1440" w:right="1440" w:bottom="1440"
           w:left="1440" w:header="720" w:footer="720" w:gutter="0"/>
</w:sectPr>

<!-- word/header1.xml -->
<w:hdr xmlns:w="…"><w:p><w:r><w:t>Header text</w:t></w:r></w:p></w:hdr>

<!-- word/footer1.xml -->
<w:ftr xmlns:w="…"><w:p><w:r><w:t>Footer text</w:t></w:r></w:p></w:ftr>
```

### Document with first-page header

```xml
<w:sectPr>
  <w:headerReference w:type="default" r:id="rId1"/>
  <w:headerReference w:type="first"   r:id="rId2"/>
  <w:footerReference w:type="default" r:id="rId3"/>
  <w:titlePg/>
  <w:pgSz w:w="12240" w:h="15840"/>
  <w:pgMar w:top="1440" w:right="1440" w:bottom="1440"
           w:left="1440" w:header="720" w:footer="720" w:gutter="0"/>
</w:sectPr>
```

---

## 13. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-H-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/header_footer.docx` | Default header + footer; pgSz (US Letter); pgMar (1-inch margins); xmlns:r on document |
| WML-H-02 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/header_first_page.docx` | Default header + first-page header; default footer; titlePg; three part relationships |
