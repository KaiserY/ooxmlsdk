# WML Styles and Property Inheritance — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.7 (styles), ISO/IEC
29500:2016 Part 1 §17.7; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

The `word/styles.xml` part carries all named style definitions for a document.
It is referenced from `word/document.xml.rels` with relationship type
`http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles`
and content type
`application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml`.

The styles part is **optional** — a document without `word/styles.xml` behaves
as if it had an empty `<w:styles/>` element. However, any document that
references a style by ID must define that style, or the reference is invalid.

---

## 2. Root Element: CT_Styles

```xml
<w:styles xmlns:w="…">
  <w:docDefaults>…</w:docDefaults>      <!-- optional, at most once -->
  <w:latentStyles>…</w:latentStyles>   <!-- optional, at most once -->
  <w:style …>…</w:style>               <!-- 0 or more style definitions -->
</w:styles>
```

Child element order is significant (XSD sequence). The `<w:style>` elements
may appear in any order relative to each other (they are peer elements, not
sequenced relative to one another).

---

## 3. Style Definition: CT_Style

Each `<w:style>` element defines one named style.

### Attributes

| Attribute | Required | Notes |
|-----------|----------|-------|
| `type` | no | ST_StyleType: `paragraph`, `character`, `table`, `numbering` |
| `styleId` | no | Identifier used in `<w:pStyle w:val="…">` / `<w:rStyle w:val="…">` references. Max 253 chars. |
| `default` | no | `1` or `true` = this is the default style for its type. At most one default per type. |
| `customStyle` | no | `1` = user-defined (not a built-in style). |

### Child element sequence

| # | Element | Notes |
|---|---------|-------|
| 1 | `name` | Display name (e.g. `"heading 1"`, `"Normal"`). Used by UI; `styleId` is the programmatic key. |
| 2 | `aliases` | Alternate names (semicolon-separated) |
| 3 | `basedOn` | Parent style ID. Inherits all unset properties from parent. |
| 4 | `next` | Style ID applied to the **next** paragraph after pressing Enter. |
| 5 | `link` | Style ID of the linked style (connects a paragraph style to its character-style twin, or vice versa). |
| 6 | `autoRedefine` | CT_OnOff — update style definition when direct formatting matches it |
| 7 | `hidden` | CT_OnOff — hide from style picker |
| 8 | `uiPriority` | Sort order in the style gallery (0–99) |
| 9 | `semiHidden` | CT_OnOff — hide until used |
| 10 | `unhideWhenUsed` | CT_OnOff — un-hide `semiHidden` once applied |
| 11 | `qFormat` | CT_OnOff — show in the Quick Styles gallery |
| 12 | `locked` | CT_OnOff — prevent user from applying or modifying |
| 13 | `personal` | E-mail message body style |
| 14 | `personalCompose` | E-mail composition style |
| 15 | `personalReply` | E-mail reply style |
| 16 | `rsid` | Revision save ID for this style definition |
| 17 | `pPr` | Paragraph properties for paragraph/table styles |
| 18 | `rPr` | Run/character properties for all style types |
| 19 | `tblPr` | Table properties (table styles only) |
| 20 | `trPr` | Table row properties (table styles only) |
| 21 | `tcPr` | Table cell properties (table styles only) |
| 22 | `tblStylePr` | Conditional table formatting (table styles only) |

---

## 4. Style Types

### ST_StyleType values

| Value | Applied via | Scope |
|-------|-------------|-------|
| `paragraph` | `<w:pPr><w:pStyle w:val="StyleId"/>` | Sets paragraph and run properties for the whole paragraph |
| `character` | `<w:rPr><w:rStyle w:val="StyleId"/>` | Sets run properties for one or more runs |
| `table` | `<w:tblPr><w:tblStyle w:val="StyleId"/>` | Table, row, and cell properties |
| `numbering` | Referenced from abstract num definitions | Numbering list formatting |

**The `default` flag:** Each type may have exactly one default style. The
default paragraph style (`Normal` in English locales) supplies the base
formatting for all paragraphs that do not specify a `<w:pStyle>`. Default
character/table/numbering styles similarly supply base formatting.

---

## 5. Property Inheritance Chain

Formatting resolves from most-specific to least-specific:

```
Direct formatting in <w:pPr> / <w:rPr>       (highest priority)
  └── Named style (via pStyle / rStyle)
        └── basedOn parent style
              └── basedOn grandparent … (chain stops at default style)
                    └── Document defaults (<w:docDefaults>)
                          └── Implied application defaults          (lowest)
```

**Key rules:**

1. A property set at a more-specific level always wins over less-specific.
2. An unset (absent) property means "inherit from the next level down."
3. CT_OnOff toggle properties follow the same rule — a `<w:b/>` at style level
   can be overridden by `<w:b w:val="0"/>` in direct formatting.
4. The `basedOn` chain is acyclic. A style may not be based on itself.
5. Cycles are malformed; parsers should stop traversal at cycle detection.

### Linked styles (`<w:link>`)

A paragraph style and a character style may be linked so that applying the
paragraph style also defines a character style that captures just the run
properties. The `link` element holds the **other** style's ID:

```xml
<!-- Paragraph style "Heading1" links to character style "Heading1Char" -->
<w:style w:type="paragraph" w:styleId="Heading1">
  <w:link w:val="Heading1Char"/>
  …
</w:style>
<w:style w:type="character" w:styleId="Heading1Char">
  <w:link w:val="Heading1"/>
  …
</w:style>
```

### Next paragraph style (`<w:next>`)

When the user presses Enter at the end of a styled paragraph, the new paragraph
automatically receives the style named by `next`. If absent, the new paragraph
keeps the current style.

---

## 6. Document Defaults: CT_DocDefaults

```xml
<w:docDefaults>
  <w:rPrDefault>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri"/>
      <w:sz w:val="22"/>        <!-- 11pt in half-points -->
      <w:szCs w:val="22"/>
    </w:rPr>
  </w:rPrDefault>
  <w:pPrDefault>
    <w:pPr>
      <w:spacing w:after="160" w:line="259" w:lineRule="auto"/>
    </w:pPr>
  </w:pPrDefault>
</w:docDefaults>
```

`<w:rPrDefault>` carries a single `<w:rPr>` that sets the baseline run
properties for the entire document (font, size, etc.). `<w:pPrDefault>` carries
a single `<w:pPr>` that sets the baseline paragraph properties.

These are the **lowest** priority in the resolution chain — overridden by any
style or direct formatting.

---

## 7. Latent Styles: CT_LatentStyles

Latent styles are built-in style definitions that are known to the application
but not stored in the file. `<w:latentStyles>` supplies overrides for the
default latent-style attributes:

```xml
<w:latentStyles w:defLockedState="0" w:defUIPriority="99"
                w:defSemiHidden="0" w:defUnhideWhenUsed="0"
                w:defQFormat="0" w:count="376">
  <w:lsdException w:name="Normal" w:uiPriority="0" w:qFormat="1"/>
  <w:lsdException w:name="heading 1" w:uiPriority="9" w:qFormat="1"/>
  …
</w:latentStyles>
```

### CT_LatentStyles attributes

| Attribute | Notes |
|-----------|-------|
| `defLockedState` | Default locked setting for all latent styles |
| `defUIPriority` | Default UI priority (0–99) |
| `defSemiHidden` | Default semi-hidden setting |
| `defUnhideWhenUsed` | Default unhide-when-used setting |
| `defQFormat` | Default quick-format setting |
| `count` | Number of latent style definitions in the application's built-in set |

### CT_LsdException attributes

| Attribute | Required | Notes |
|-----------|----------|-------|
| `name` | **yes** | Display name of the built-in style this exception applies to |
| `locked` | no | Override locked state |
| `uiPriority` | no | Override UI sort order |
| `semiHidden` | no | Override semi-hidden |
| `unhideWhenUsed` | no | Override unhide-when-used |
| `qFormat` | no | Override quick-format flag |

**Round-trip note:** The latent styles block is large in Microsoft Office files
(often 376+ entries). It must round-trip verbatim; none of the attributes or
entries should be dropped or reordered.

---

## 8. Style Reference in Paragraphs and Runs

### Paragraph style reference

```xml
<w:p>
  <w:pPr>
    <w:pStyle w:val="Heading1"/>   <!-- matches styleId="Heading1" -->
  </w:pPr>
  …
</w:p>
```

### Character style reference

```xml
<w:r>
  <w:rPr>
    <w:rStyle w:val="Strong"/>     <!-- matches styleId="Strong" -->
  </w:rPr>
  …
</w:r>
```

`<w:pStyle>` and `<w:rStyle>` are the first children of `<w:pPr>` and
`<w:rPr>` respectively, before any other properties.

---

## 9. The Normal (Default Paragraph) Style

Every well-formed document should define a default paragraph style. Its
`styleId` must match the ID referenced as the implicit base for all paragraphs
not specifying a `pStyle`. Conventionally this style is named "Normal" with
`w:default="1"`. Its `<w:basedOn>` is **absent** — the default style has no
parent.

If a document has no Normal style, the parser must fall back to
`<w:docDefaults>` as the effective base.

---

## 10. Round-Trip Gotchas

1. **`styleId` vs. `name`.** Style references (`pStyle`, `rStyle`, `basedOn`,
   `next`, `link`) all use `styleId`, not the display `name`. These are
   different strings. `styleId="Heading1"` with `name="heading 1"` is the
   canonical Office pair.

2. **Case sensitivity.** `styleId` is case-sensitive. A reference to
   `"heading1"` will not match `styleId="Heading1"`.

3. **`basedOn` stops at the default style.** The default paragraph style (the
   one with `w:default="1"`) should have no `basedOn`. If a chain reaches a
   style with no `basedOn`, resolution continues at `docDefaults`.

4. **Character style `rPr` vs. paragraph style `rPr`.** A paragraph style's
   `<w:rPr>` sets properties for runs in that paragraph. A character style's
   `<w:rPr>` is applied only when the style is referenced via `rStyle`. Both
   are `CT_RPrStyle`, not `CT_RPr`.

5. **`pPr` in styles is `CT_PPrStyle`, not `CT_PPr`.** Style-level paragraph
   properties use a slightly different type — they do not include `<w:sectPr>`
   or `<w:pPrChange>`. The generated Rust type is `StyleParagraphProperties`,
   not `ParagraphProperties`.

6. **Preservation of `<w:latentStyles>`.** This block must be round-tripped
   exactly. Stripping it causes Microsoft Word to show "unknown style" warnings
   for built-in styles.

7. **Empty `<w:rPr>` in default style.** The Normal style in Office files
   commonly has an empty `<w:rPr/>` or no `<w:rPr>` at all. Both must
   round-trip correctly.

8. **Style ordering.** The XSD allows styles in any order within `<w:styles>`.
   Real Office files list `docDefaults`, then `latentStyles`, then styles
   ordered: default paragraph style first, then other built-ins, then custom.
   A round-trip must not reorder styles.

9. **`w:rsid` on styles.** Like rsid on runs and paragraphs, this must be
   preserved verbatim. It is a 4-byte uppercase hex string (e.g. `"00A1B2C3"`).

10. **`type` attribute can be absent.** Some producers omit `w:type` on
    paragraph styles. The resolver treats an absent `type` as `paragraph`.

---

## 11. Minimal Valid Structures

### Minimal styles part (Normal + Heading 1)
```xml
<w:styles xmlns:w="…">
  <w:style w:type="paragraph" w:default="1" w:styleId="Normal">
    <w:name w:val="Normal"/>
  </w:style>
  <w:style w:type="paragraph" w:styleId="Heading1">
    <w:name w:val="heading 1"/>
    <w:basedOn w:val="Normal"/>
    <w:next w:val="Normal"/>
    <w:pPr><w:outlineLvl w:val="0"/></w:pPr>
    <w:rPr><w:b/><w:sz w:val="32"/></w:rPr>
  </w:style>
</w:styles>
```

### Linked paragraph + character style
```xml
<w:style w:type="paragraph" w:styleId="Quote">
  <w:name w:val="Quote"/>
  <w:basedOn w:val="Normal"/>
  <w:link w:val="QuoteChar"/>
  <w:pPr><w:ind w:left="720" w:right="720"/></w:pPr>
  <w:rPr><w:i/></w:rPr>
</w:style>
<w:style w:type="character" w:styleId="QuoteChar">
  <w:name w:val="Quote Char"/>
  <w:basedOn w:val="DefaultParagraphFont"/>
  <w:link w:val="Quote"/>
  <w:rPr><w:i/></w:rPr>
</w:style>
```

### Document defaults
```xml
<w:docDefaults>
  <w:rPrDefault>
    <w:rPr>
      <w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:cs="Times New Roman"/>
      <w:sz w:val="22"/>
      <w:szCs w:val="22"/>
    </w:rPr>
  </w:rPrDefault>
  <w:pPrDefault>
    <w:pPr>
      <w:spacing w:after="160" w:line="259" w:lineRule="auto"/>
    </w:pPr>
  </w:pPrDefault>
</w:docDefaults>
```

---

## 12. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-S-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/style_inheritance.docx` | 3-level basedOn chain; paragraph style references; docDefaults |
| WML-S-02 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/style_linked.docx` | Linked paragraph+character style pair; next style reference; character rStyle |
