# WML Paragraph Formatting — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.3.1 (paragraph), §17.3.3
(run properties), ISO/IEC 29500:2016 Part 1 §17.3; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

A paragraph in WordprocessingML is represented by `<w:p>`. Its formatting is
stored in a `<w:pPr>` child that appears as the **first** child of `<w:p>` (if
present). The paragraph mark (`¶`) is a special run whose character properties
(`<w:rPr>` inside `<w:pPr>`) style the mark itself and establish inherited run
properties for the paragraph.

```
<w:p>
  <w:pPr>
    <!-- paragraph properties here -->
    <w:rPr>…</w:rPr>   <!-- paragraph-mark run properties (always last) -->
  </w:pPr>
  <w:r>…</w:r>
  …
</w:p>
```

All elements defined in namespace
`http://schemas.openxmlformats.org/wordprocessingml/2006/main` (prefix `w:`).

---

## 2. CT_PPrBase and CT_PPr Schemas

`CT_PPrBase` defines the core paragraph properties. `CT_PPr` extends it by
appending `<w:rPr>` (paragraph-mark run properties), `<w:sectPr>` (section
properties on the last paragraph of a section), and `<w:pPrChange>` (tracked
change wrapper).

### Child element sequence in CT_PPrBase

All children are optional. **Order is significant** — parsers that encounter
elements out of sequence may fail. The XSD-mandated sequence:

| # | Element | Type | Notes |
|---|---------|------|-------|
| 1 | `pStyle` | CT_String | Paragraph style ID |
| 2 | `keepNext` | CT_OnOff | Keep with next paragraph |
| 3 | `keepLines` | CT_OnOff | Keep all lines together |
| 4 | `pageBreakBefore` | CT_OnOff | Force page break before |
| 5 | `framePr` | CT_FramePr | Frame/text-box anchor |
| 6 | `widowControl` | CT_OnOff | Suppress widows/orphans |
| 7 | `numPr` | CT_NumPr | Numbering reference |
| 8 | `suppressLineNumbers` | CT_OnOff | Suppress line numbers |
| 9 | `pBdr` | CT_PBdr | Paragraph borders |
| 10 | `shd` | CT_Shd | Background shading |
| 11 | `tabs` | CT_Tabs | Custom tab stops |
| 12 | `suppressAutoHyphens` | CT_OnOff | Disable auto-hyphens |
| 13 | `kinsoku` | CT_OnOff | East Asian line break rules |
| 14 | `wordWrap` | CT_OnOff | Wrap words at character boundary |
| 15 | `overflowPunct` | CT_OnOff | Allow punctuation overflow |
| 16 | `topLinePunct` | CT_OnOff | Top-line punctuation |
| 17 | `autoSpaceDE` | CT_OnOff | Auto-space between Latin and CJK |
| 18 | `autoSpaceDN` | CT_OnOff | Auto-space between CJK and digits |
| 19 | `bidi` | CT_OnOff | Right-to-left paragraph |
| 20 | `adjustRightInd` | CT_OnOff | Auto-adjust right indent for mirrored margins |
| 21 | `snapToGrid` | CT_OnOff | Use document grid |
| 22 | `spacing` | CT_Spacing | Before/after/line spacing |
| 23 | `ind` | CT_Ind | Indentation |
| 24 | `contextualSpacing` | CT_OnOff | Suppress spacing between same-style paragraphs |
| 25 | `mirrorIndents` | CT_OnOff | Mirror start/end indents |
| 26 | `suppressOverlap` | CT_OnOff | Prevent text frame overlap |
| 27 | `jc` | CT_Jc | Text alignment |
| 28 | `textDirection` | CT_TextDirection | Text direction |
| 29 | `textAlignment` | CT_TextAlignment | Vertical text alignment |
| 30 | `textboxTightWrap` | CT_TextboxTightWrap | Tight wrap around text box |
| 31 | `outlineLvl` | CT_DecimalNumber | Heading outline level (0–8) |
| 32 | `divId` | CT_DecimalNumber | HTML `<div>` wrapper ID |
| 33 | `cnfStyle` | CT_Cnf | Conditional table formatting |

---

## 3. Justification (`<w:jc>`)

**Type:** CT_Jc  
**Attribute:** `w:val` (ST_Jc, **required**)

```xml
<w:jc w:val="center"/>
```

### ST_Jc enum values

| Value | Meaning |
|-------|---------|
| `left` | Left-aligned (default for LTR text) |
| `right` | Right-aligned |
| `center` | Centered |
| `both` | Full justification (aligned to both margins) |
| `distribute` | Distribute space between all characters |
| `start` | Start-aligned (logical; same as `left` for LTR) |
| `end` | End-aligned (logical; same as `right` for LTR) |
| `mediumKashida` | Justified with medium kashida expansion (Arabic) |
| `highKashida` | Justified with high kashida expansion (Arabic) |
| `lowKashida` | Justified with low kashida expansion (Arabic) |
| `thaiDistribute` | Thai distributed alignment |
| `numTab` | Numeric tab alignment |

**Round-trip note:** Some producers emit `start`/`end` instead of `left`/`right`.
The SDK normalises these during read — the Rust type stores the enum value read
from the file verbatim; the generator does not translate `start` → `left`.
Consumers must treat `start` as equivalent to `left` for LTR paragraphs.

---

## 4. Spacing Between Lines and Paragraphs (`<w:spacing>`)

**Type:** CT_Spacing  
All values are **twips** (twentieths of a point, i.e. 1/1440 inch) unless noted.

```xml
<w:spacing w:before="240" w:after="160" w:line="276" w:lineRule="auto"/>
```

### Attributes

| Attribute | Type | Default | Notes |
|-----------|------|---------|-------|
| `before` | ST_TwipsMeasure | 0 | Space above paragraph (twips) |
| `beforeLines` | xsd:integer | 0 | Space above in 1/100 of a line |
| `beforeAutospacing` | ST_OnOff | false | Let application decide before-spacing |
| `after` | ST_TwipsMeasure | 0 | Space below paragraph (twips) |
| `afterLines` | xsd:integer | 0 | Space below in 1/100 of a line |
| `afterAutospacing` | ST_OnOff | false | Let application decide after-spacing |
| `line` | ST_SignedTwipsMeasure | — | Line spacing value (see lineRule) |
| `lineRule` | ST_LineSpacingRule | `auto` | How to interpret `line` |

### ST_LineSpacingRule values

| Value | `line` interpretation |
|-------|-----------------------|
| `auto` | Multiple of single line spacing. 240 = single, 360 = 1.5×, 480 = double. |
| `exact` | Fixed line height in twips. Lines are exactly this tall; content may clip. |
| `atLeast` | Minimum line height in twips. Lines expand if content is taller. |

When `lineRule` is absent the default is `auto`.

### Common spacing values (twips)

| Points | Twips | Common use |
|--------|-------|-----------|
| 0 pt | 0 | No before/after space |
| 6 pt | 120 | Small gap |
| 10 pt | 200 | Before paragraph spacing |
| 12 pt | 240 | Single line spacing (`line` with `auto`) |
| 16 pt | 320 | After paragraph |
| 18 pt | 360 | 1.5× line spacing (`line` with `auto`) |
| 24 pt | 480 | Double line spacing (`line` with `auto`) |

---

## 5. Indentation (`<w:ind>`)

**Type:** CT_Ind

```xml
<w:ind w:left="720" w:right="360" w:firstLine="360"/>
```

### Attributes

| Attribute | Type | Notes |
|-----------|------|-------|
| `left` | ST_SignedTwipsMeasure | Left indent (LTR; OOXML 2007 name) |
| `start` | ST_SignedTwipsMeasure | Logical start indent (Office 2010+ bidi-aware alias for `left`) |
| `leftChars` | xsd:integer | Left indent in 1/100 character widths |
| `startChars` | xsd:integer | Logical start in 1/100 character widths |
| `right` | ST_SignedTwipsMeasure | Right indent (LTR; OOXML 2007 name) |
| `end` | ST_SignedTwipsMeasure | Logical end indent (Office 2010+ alias for `right`) |
| `rightChars` | xsd:integer | Right indent in character units |
| `endChars` | xsd:integer | Logical end in character units |
| `hanging` | ST_TwipsMeasure | Hanging indent: how far the first line is pulled left relative to the rest |
| `hangingChars` | xsd:integer | Hanging indent in character units |
| `firstLine` | ST_TwipsMeasure | Additional first-line indent (positive only; mutually exclusive with `hanging`) |
| `firstLineChars` | xsd:integer | First-line indent in character units |

**Mutual exclusivity:** `firstLine` and `hanging` are mutually exclusive. If
both appear the file is invalid. `left` and `start` may coexist (for
transitional compatibility) but represent the same axis — `start` takes
precedence in bidi paragraphs.

### Common indent values

| Inches | Twips | Typical use |
|--------|-------|-------------|
| 0.5" | 720 | Single tab stop / list first level |
| 1.0" | 1440 | Double indent / second level |
| 0.25" | 360 | Hanging indent for bullets |

---

## 6. Paragraph Borders (`<w:pBdr>`)

**Type:** CT_PBdr — contains up to 6 border child elements.

```xml
<w:pBdr>
  <w:top    w:val="single" w:sz="6" w:space="1" w:color="000000"/>
  <w:left   w:val="single" w:sz="6" w:space="4" w:color="000000"/>
  <w:bottom w:val="single" w:sz="6" w:space="1" w:color="000000"/>
  <w:right  w:val="single" w:sz="6" w:space="4" w:color="000000"/>
</w:pBdr>
```

### Child elements

| Element | Role |
|---------|------|
| `top` | Top border |
| `left` | Left border |
| `bottom` | Bottom border |
| `right` | Right border |
| `between` | Border between consecutive paragraphs sharing this style |
| `bar` | Vertical bar on the far edge of the page margin |

### CT_Border attributes

| Attribute | Required | Type | Notes |
|-----------|----------|------|-------|
| `val` | **yes** | ST_Border | Border style |
| `color` | no | ST_HexColor | 6-hex RGB or `auto` |
| `themeColor` | no | ST_ThemeColor | Named theme colour slot |
| `themeTint` | no | ST_UcharHexNumber | Tint applied to theme colour |
| `themeShade` | no | ST_UcharHexNumber | Shade applied to theme colour |
| `sz` | no | ST_EighthPointMeasure | Width in eighths-of-a-point (8 = 1pt) |
| `space` | no | ST_PointMeasure | Gap between border and text (0–31 pt) |
| `shadow` | no | ST_OnOff | Drop shadow on border |
| `frame` | no | ST_OnOff | Frame effect |

### ST_Border common values

| Value | Description |
|-------|-------------|
| `nil` | No border (explicit removal; different from `none`) |
| `none` | No border |
| `single` | Single line |
| `thick` | Thick single line |
| `double` | Double line |
| `dotted` | Dotted |
| `dashed` | Dashed |
| `dotDash` | Dot-dash |
| `dotDotDash` | Dot-dot-dash |
| `triple` | Triple line |
| `thinThickSmallGap` | Thin-thick, small gap |
| `thickThinSmallGap` | Thick-thin, small gap |
| `wave` | Wave |
| `doubleWave` | Double wave |
| `dashSmallGap` | Dashed small gap |
| `threeDEmboss` | 3-D emboss |
| `threeDEngrave` | 3-D engrave |
| `outset` | Outset |
| `inset` | Inset |

---

## 7. Shading (`<w:shd>`)

**Type:** CT_Shd  
Applies a background pattern and/or fill colour to the paragraph.

```xml
<w:shd w:val="clear" w:color="auto" w:fill="FFFF00"/>
```

### Attributes

| Attribute | Required | Notes |
|-----------|----------|-------|
| `val` | **yes** | ST_Shd — the pattern type |
| `color` | no | Foreground (pattern) colour: 6-hex RGB or `auto` |
| `themeColor` | no | Pattern theme colour slot |
| `themeTint` | no | Pattern theme tint (1–2 hex chars) |
| `themeShade` | no | Pattern theme shade (1–2 hex chars) |
| `fill` | no | Background fill colour: 6-hex RGB or `auto` |
| `themeFill` | no | Background theme colour slot |
| `themeFillTint` | no | Background theme tint |
| `themeFillShade` | no | Background theme shade |

### ST_Shd key values

| Value | Description |
|-------|-------------|
| `nil` | No shading (explicit remove) |
| `clear` | Solid fill using `fill` colour, no pattern |
| `solid` | Solid foreground colour (`color`) |
| `horzStripe` | Horizontal stripes (foreground on fill) |
| `vertStripe` | Vertical stripes |
| `diagStripe` | Diagonal stripes |
| `diagCross` | Diagonal cross-hatch |
| `horzCross` | Horizontal/vertical cross-hatch |
| `pct5` … `pct95` | Percentage dot-fill (5 % to 95 %) |

**Common pattern:** `val="clear"` + `fill="RRGGBB"` = solid background colour.
`val="solid"` + `color="RRGGBB"` = solid foreground (rare; inverts the visual).

---

## 8. Keep / Break Control Properties

These are all CT_OnOff (no required attributes; presence = on, `w:val="0"` or
`w:val="false"` = explicit off — see wml_runs.md §CT_OnOff semantics).

| Element | Effect |
|---------|--------|
| `keepNext` | Keep this paragraph on the same page as the next one |
| `keepLines` | Keep all lines of this paragraph together (no page break inside) |
| `pageBreakBefore` | Force a page break immediately before this paragraph |
| `widowControl` | Prevent widow and orphan lines (default on; explicit `w:val="0"` turns off) |
| `suppressLineNumbers` | Exclude from the document line-number sequence |
| `contextualSpacing` | Suppress `before`/`after` spacing when adjacent paragraph shares the same style |

---

## 9. Outline Level (`<w:outlineLvl>`)

**Type:** CT_DecimalNumber  
**Attribute:** `w:val` (required integer 0–8)

Value 0 = Heading 1, 1 = Heading 2, …, 8 = Heading 9. Value 9 means "body
text" (not a heading). This attribute drives the navigation pane, table of
contents generation, and `<w:bookmarkStart>` heading anchors.

```xml
<w:outlineLvl w:val="0"/>   <!-- Heading 1 -->
```

---

## 10. Property Ordering and Inheritance

**Within `<w:pPr>`:** the child sequence must follow the CT_PPrBase order
(§2 above). Validators reject out-of-sequence children. The most common subset
used in practice:

```
pStyle → keepNext → keepLines → pageBreakBefore → widowControl →
pBdr → shd → tabs → spacing → ind → contextualSpacing → jc → outlineLvl →
[rPr]
```

**Inheritance chain** (most-specific wins):

1. Direct formatting in `<w:pPr>` (highest priority)
2. Named paragraph style (`pStyle` reference)
3. `basedOn` chain from the named style
4. `docDefaults/pPrDefault` in the styles part
5. Implied defaults from the application

---

## 11. Round-Trip Gotchas

1. **Twips vs. other units.** `before`, `after`, and `line` accept CSS-style
   strings in some contexts (`"1.5cm"`, `"0.5in"`) but the canonical XML
   representation is always an integer twip string. Always write twip integers.

2. **`left` vs. `start` on `<w:ind>`.** Older documents use `left`/`right`;
   Office 2010+ and RTL-aware producers use `start`/`end`. A round-trip must
   preserve whichever attribute was present; the SDK stores the raw attribute
   value, so no translation occurs.

3. **`firstLine` vs. `hanging` mutual exclusivity.** A file with both is
   malformed. The SDK ignores `hanging` if `firstLine` is also present.

4. **`widowControl` default is ON.** Most documents omit it. `<w:widowControl
   w:val="0"/>` explicitly disables it. Presence without `w:val` also enables
   it (no change from default). Round-tripping must preserve explicit `val="0"`.

5. **`pBdr val="nil"` vs. absent.** `nil` means "explicitly remove any inherited
   border". An absent `<w:top>` means "inherit from style". These are different
   and must not be conflated.

6. **Colour hex is UPPERCASE in canonical output.** The spec allows either case;
   Microsoft Office normalises to uppercase. The SDK preserves whatever was
   written.

7. **`contextualSpacing` removes inter-paragraph space.** When two consecutive
   paragraphs both have `contextualSpacing` and share the same `pStyle`, the
   `after` spacing of the first is suppressed. This is purely a rendering hint
   and does not affect serialised values.

8. **`sz` on `<w:border>` is in eighths-of-a-point.** `sz="6"` = 0.75 pt
   (hairline). `sz="4"` = 0.5 pt. Common values: 4 (thin), 6, 8 (1 pt), 12,
   18, 24 (3 pt). Unlike WML run `sz` (half-points) or DrawingML `sz`
   (hundredths-of-a-point), border `sz` is eighths-of-a-point.

9. **`pBdr between` and `bar` are uncommon.** `between` applies between two
   consecutive paragraphs only when both have identical `between` borders.
   `bar` is a full-height vertical bar in the gutter margin.

10. **`jc="both"` is "justified"**, not "centre-aligned". Centres in the x-axis
    sense is `jc="center"`.

---

## 12. Minimal Valid Structures

### Justified paragraph with spacing
```xml
<w:p>
  <w:pPr>
    <w:jc w:val="both"/>
    <w:spacing w:before="120" w:after="120" w:line="276" w:lineRule="auto"/>
  </w:pPr>
  <w:r><w:t>Justified text with spacing.</w:t></w:r>
</w:p>
```

### Indented paragraph (1-inch left, 0.5-inch hanging)
```xml
<w:p>
  <w:pPr>
    <w:ind w:left="1440" w:hanging="720"/>
  </w:pPr>
  <w:r><w:t>Hanging indent paragraph.</w:t></w:r>
</w:p>
```

### Paragraph with box border and yellow fill
```xml
<w:p>
  <w:pPr>
    <w:pBdr>
      <w:top    w:val="single" w:sz="4" w:space="1" w:color="000000"/>
      <w:left   w:val="single" w:sz="4" w:space="4" w:color="000000"/>
      <w:bottom w:val="single" w:sz="4" w:space="1" w:color="000000"/>
      <w:right  w:val="single" w:sz="4" w:space="4" w:color="000000"/>
    </w:pBdr>
    <w:shd w:val="clear" w:color="auto" w:fill="FFFF00"/>
  </w:pPr>
  <w:r><w:t>Bordered and filled.</w:t></w:r>
</w:p>
```

### Heading with outline level
```xml
<w:p>
  <w:pPr>
    <w:keepNext/>
    <w:outlineLvl w:val="0"/>
  </w:pPr>
  <w:r><w:rPr><w:b/></w:rPr><w:t>Chapter 1</w:t></w:r>
</w:p>
```

---

## 13. Fixture Plan

| ID | File | Properties covered |
|----|------|-------------------|
| WML-P-01 | `test-data/wml/para_alignment.docx` | `jc` left/center/right/both/distribute |
| WML-P-02 | `test-data/wml/para_spacing.docx` | `spacing` before/after twips; lineRule auto/exact/atLeast; `contextualSpacing` |
| WML-P-03 | `test-data/wml/para_indent.docx` | `ind` left/right; firstLine; hanging |
| WML-P-04 | `test-data/wml/para_borders_shading.docx` | `pBdr` top/left/bottom/right; `shd` clear fill; `shd` pct fill |
| WML-P-05 | `test-data/wml/para_keep.docx` | `keepNext`, `keepLines`, `pageBreakBefore`, `widowControl`, `outlineLvl` |
