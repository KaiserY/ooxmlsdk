# WML Section Properties — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.6 (section properties),
§17.15 (document settings); ISO/IEC 29500:2016 Part 1 §17.6; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

**Scope:** This spec covers the section property elements not already detailed
in `wml_headers_footers.md`. For `<w:pgSz>`, `<w:pgMar>`, `<w:type>`,
`<w:pgNumType>`, `<w:headerReference>`, `<w:footerReference>`, and
`<w:titlePg>`, see `docs/specs/wml_headers_footers.md`.

---

## 1. Overview

`<w:sectPr>` appears either as the **last child of `<w:body>`** (the final
section) or inside a paragraph's `<w:pPr>` (marks a mid-document section
break). Beyond page size, margins, and header/footer references, `<w:sectPr>`
controls:

- Multi-column layout (`<w:cols>`)
- Vertical page alignment (`<w:vAlign>`)
- Document grid / snap-to-grid (`<w:docGrid>`)
- Line numbering (`<w:lnNumType>`)
- Page borders (`<w:pgBorders>`)
- Text direction (`<w:textDirection>`)
- Section-level footnote/endnote settings (`<w:footnotePr>`, `<w:endnotePr>`)

---

## 2. CT_SectPr — Full Child Sequence

`EG_HdrFtrReferences` comes before `EG_SectPrContents`. Within
`EG_SectPrContents`, the order is:

| Position | Element | Description |
|----------|---------|-------------|
| — | `headerReference` / `footerReference` | 0–6 instances (before content children) |
| 1 | `footnotePr` | Section-level footnote properties |
| 2 | `endnotePr` | Section-level endnote properties |
| 3 | `type` | Section break type |
| 4 | `pgSz` | Page size |
| 5 | `pgMar` | Page margins |
| 6 | `paperSrc` | Paper source (printer-specific) |
| 7 | `pgBorders` | Page borders |
| 8 | `lnNumType` | Line numbering |
| 9 | `pgNumType` | Page number format and start |
| 10 | `cols` | Column layout |
| 11 | `formProt` | Form protection |
| 12 | `vAlign` | Vertical page alignment |
| 13 | `noEndnote` | Suppress endnotes for section |
| 14 | `titlePg` | First-page header/footer enabled |
| 15 | `textDirection` | Text direction |
| 16 | `bidi` | Right-to-left section layout |
| 17 | `rtlGutter` | Gutter on right side |
| 18 | `docGrid` | Document grid |
| 19 | `printerSettings` | Printer settings part reference |
| 20 | `sectPrChange` | Tracked-change wrapper |

---

## 3. Column Layout: CT_Columns

```xml
<!-- Two equal columns with separator line, 720-twip gap -->
<w:cols w:num="2" w:space="720" w:sep="1"/>

<!-- Three unequal columns with explicit widths -->
<w:cols w:equalWidth="0">
  <w:col w:w="2880" w:space="360"/>
  <w:col w:w="4320" w:space="360"/>
  <w:col w:w="2880"/>
</w:cols>
```

### Attributes

| Attribute | XML name | Type | Notes |
|-----------|----------|------|-------|
| `equal_width` | `w:equalWidth` | ST_OnOff | `1` = equal widths (default). Set `0` when providing explicit `<w:col>` children. |
| `space` | `w:space` | ST_TwipsMeasure | Gap between columns. Default 720 twips (0.5 in). |
| `column_count` | `w:num` | integer (1–45) | Column count when `equalWidth` is true. Default 1. |
| `separator` | `w:sep` | ST_OnOff | Draw a vertical rule between columns. |

### CT_Column children

When `w:equalWidth="0"`, provide one `<w:col>` per column:

| Attribute | XML name | Type | Notes |
|-----------|----------|------|-------|
| `width` | `w:w` | ST_TwipsMeasure | Column width in twips. |
| `space` | `w:space` | ST_TwipsMeasure | Space after this column (gap to the right). Omit on the last column. |

The sum of all column widths plus spaces must equal the text-area width
(page width − left margin − right margin).

### Rust types

```
Columns { equal_width, space, column_count, separator, w_col: Vec<Column> }
Column  { width, space }
```

---

## 4. Vertical Page Alignment: CT_VerticalJc

```xml
<w:vAlign w:val="center"/>
```

Controls how the text block is aligned vertically within the page margins.

### ST_VerticalJc values

| Value | Meaning |
|-------|---------|
| `top` | Content anchored to the top margin (default behaviour) |
| `center` | Content centred between top and bottom margins |
| `both` | Content justified — space distributed between paragraphs |
| `bottom` | Content anchored to the bottom margin |

Rust enum: `VerticalJustificationValues` — variants `Top`, `Center`, `Both`, `Bottom`.

---

## 5. Document Grid: CT_DocGrid

```xml
<w:docGrid w:type="lines" w:linePitch="360"/>
<w:docGrid w:type="linesAndChars" w:linePitch="312" w:charSpace="4096"/>
```

The document grid enables East Asian CJK snap-to-grid behaviour and controls
line pitch for consistent line spacing.

### Attributes

| Attribute | XML name | Type | Notes |
|-----------|----------|------|-------|
| `r#type` | `w:type` | ST_DocGrid | Grid type |
| `line_pitch` | `w:linePitch` | integer | Lines per page × 20, or pitch in twentieths of a point |
| `character_space` | `w:charSpace` | integer | Char pitch as fixed-point integer |

### ST_DocGrid values

| Value | Meaning |
|-------|---------|
| `default` | No grid — normal layout |
| `lines` | Line grid only — aligns lines to the pitch |
| `linesAndChars` | Line and character grid — aligns both |
| `snapToChars` | Snap characters to grid, ignoring line pitch |

Rust enum: `DocGridValues` — variants `Default`, `Lines`, `LinesAndChars`, `SnapToChars`.

---

## 6. Line Numbering: CT_LineNumber

```xml
<w:lnNumType w:countBy="5" w:start="1" w:restart="newPage"/>
```

Adds line numbers in the left margin. `<w:suppressLineNumbers/>` on a
paragraph's `<w:pPr>` suppresses numbering for that paragraph.

### Attributes

| Attribute | XML name | Type | Notes |
|-----------|----------|------|-------|
| `count_by` | `w:countBy` | integer (1–100) | Print a number every N lines. |
| `start` | `w:start` | integer | First line number. Default 1. |
| `distance` | `w:distance` | ST_TwipsMeasure | Distance from text area to the line number. |
| `restart` | `w:restart` | ST_LineNumberRestart | When to reset the counter. |

### ST_LineNumberRestart values

| Value | Meaning |
|-------|---------|
| `newPage` | Reset to `start` on every page (default) |
| `newSection` | Reset to `start` at the start of each section |
| `continuous` | Never reset — count continues across sections |

Rust enum: `LineNumberRestartValues` — variants `NewPage`, `NewSection`, `Continuous`.

---

## 7. Page Borders: CT_PageBorders

```xml
<w:pgBorders w:offsetFrom="page">
  <w:top    w:val="single" w:sz="4" w:space="24" w:color="000000"/>
  <w:left   w:val="single" w:sz="4" w:space="24" w:color="000000"/>
  <w:bottom w:val="single" w:sz="4" w:space="24" w:color="000000"/>
  <w:right  w:val="single" w:sz="4" w:space="24" w:color="000000"/>
</w:pgBorders>
```

### CT_PageBorders attributes

| Attribute | XML name | Type | Notes |
|-----------|----------|------|-------|
| `z_order` | `w:zOrder` | ST_PageBorderZOrder | `front` (over text) or `back` (behind text). Default `front`. |
| `display` | `w:display` | ST_PageBorderDisplay | Which pages show borders. |
| `offset_from` | `w:offsetFrom` | ST_PageBorderOffset | `page` = measured from page edge; `text` = from text area. Default `text`. |

### ST_PageBorderDisplay values

| Value | Meaning |
|-------|---------|
| `allPages` | Show on all pages (default) |
| `firstPage` | Show only on the first page |
| `notFirstPage` | Show on all pages except the first |

### ST_PageBorderOffset values

| Value | Meaning |
|-------|---------|
| `page` | Distances are from the physical page edge |
| `text` | Distances are from the text area edge |

### Border children

Each border element (top, left, bottom, right) uses CT_Border attributes:

| Attribute | Notes |
|-----------|-------|
| `w:val` | Border style (ST_Border: `single`, `double`, `dotted`, `dashed`, `none`, etc.) |
| `w:sz` | Width in eighths of a point |
| `w:space` | Padding in points |
| `w:color` | Hex RGB (e.g., `000000`) or `auto` |
| `w:shadow` | Draw shadow (CT_OnOff) |

---

## 8. Text Direction: CT_TextDirection

```xml
<w:textDirection w:val="tbRl"/>
```

Controls the text direction for the entire section. Used primarily for
East Asian vertical text layouts.

### ST_TextDirection values

| Value | Layout |
|-------|--------|
| `lrTb` | Left-to-right, top-to-bottom (default Latin) |
| `tbRl` | Top-to-bottom, right-to-left (vertical CJK, traditional) |
| `btLr` | Bottom-to-top, left-to-right |
| `lrTbV` | Left-to-right, top-to-bottom rotated |
| `tbRlV` | Top-to-bottom, right-to-left rotated |
| `tbLrV` | Top-to-bottom, left-to-right rotated |

Plus 2010-only aliases: `tb`, `rl`, `lr`, `tbV`, `rlV`, `lrV`.

Rust enum: `TextDirectionValues`.

---

## 9. Mid-Document Section Breaks

A section break that is not the final section of the document is marked by
placing `<w:sectPr>` inside the `<w:pPr>` of a paragraph:

```xml
<w:p>
  <w:pPr>
    <w:sectPr>
      <w:type w:val="continuous"/>
      <w:cols w:num="2" w:space="720"/>
    </w:sectPr>
  </w:pPr>
</w:p>
```

The paragraph containing `<w:pPr><w:sectPr>` is the **last paragraph of the
ending section** — it is considered part of the section being ended, not the
new section that follows.

### Continuous section breaks and column changes

A `<w:type w:val="continuous"/>` section break on the same page allows
switching column layouts mid-page:

- The sections share the same physical page.
- The earlier section typically has `<w:cols w:num="1"/>` (single column).
- The later section has `<w:cols w:num="2"/>` (double column).
- The final `<w:sectPr>` in `<w:body>` defines the last section and usually
  omits `<w:type>` (defaults to `nextPage`).

---

## 10. Inherited vs. Empty Section Properties

For mid-document sections (sectPr inside pPr), omitted children are
**inherited from the following section**, not from the previous one. The
inheritance chain flows **forward** through sections:

- The body's final `<w:sectPr>` is the base.
- Mid-document sections explicitly override only what they need.
- `<w:pgSz>` and `<w:pgMar>` on a continuous section are often omitted and
  inherited.

---

## 11. Round-Trip Gotchas

1. **sectPr order is strict.** `EG_HdrFtrReferences` must precede
   `EG_SectPrContents`. Within `EG_SectPrContents`, elements must appear in
   the XSD sequence order. Out-of-order children cause schema-validation failure.

2. **`cols` with explicit children needs `equalWidth="0"`.** If you supply
   `<w:col>` children without setting `w:equalWidth="0"`, the column
   definitions are ignored by processors that assume equal widths.

3. **`cols` child count must match `num`.** When using equal-width columns,
   `w:num` determines the column count. When using explicit `<w:col>` children,
   the number of `<w:col>` elements determines the count; `w:num` is
   redundant/ignored.

4. **`vAlign` default is `top`, not `bottom`.** Omitting `<w:vAlign>` and
   setting `<w:vAlign w:val="top"/>` are equivalent. Do not add it if the
   round-trip source omits it.

5. **`docGrid` linePitch is required when type is not `default`.** When
   `w:type` is `lines`, `linesAndChars`, or `snapToChars`, `w:linePitch`
   should be present. Processors may ignore the grid if pitch is missing.

6. **`lnNumType` does not suppress numbered styling.** Line numbers in the
   margin are purely a rendering hint. They have no effect on `<w:t>` content.
   The `countBy` attribute controls which numbered line shows a printed number,
   not which lines are counted.

7. **`pgBorders offsetFrom="page"` distances are from page edge.** The
   `w:space` attribute on each border element is the gap **from the reference
   point** (`page` or `text`). If `offsetFrom="text"`, space is measured from
   the text area.

8. **Continuous sections may lack `pgSz`/`pgMar`.** A continuous section
   that differs only in column layout legitimately omits page size and margin
   elements — they inherit from the next full-page section. Round-trip must
   not add them.

9. **`textDirection` affects both text rendering and margin interpretation.**
   When `textDirection` is vertical (`tbRl` etc.), `w:pgMar` attributes `top`
   and `bottom` switch roles (top/bottom become the side margins for vertical
   text). Processors that only read `pgMar` without considering `textDirection`
   will get wrong layout.

10. **`sectPrChange` preserves revision history.** When tracked changes are
    enabled, the previous section properties are wrapped in `<w:sectPrChange>`.
    Round-trip must preserve this element and its `w:id` attribute.

---

## 12. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-S-01 | `test-data/wml/section_columns.docx` | Two equal columns; `w:cols num="2" space="720"`; continuous section break followed by single-column section |
| WML-S-02 | `test-data/wml/section_props.docx` | `vAlign=center`; `docGrid type="lines" linePitch="360"`; `lnNumType countBy="5"` |
