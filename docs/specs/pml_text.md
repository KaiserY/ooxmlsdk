# PresentationML / DrawingML Text Bodies — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §19.3 (PresentationML shapes),
§21.1 (DrawingML text body); ISO/IEC 29500:2016 Part 1 §21.1; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/dml-main.xsd` and
`pml.xsd`.

---

## 1. Overview

Text in PresentationML is hosted by the DrawingML text body element
`<a:txBody>` (schema type `CT_TextBody`). The same element type is used
wherever a shape carries text: slide shape trees, table cells, connector
shapes, and picture shapes. The DrawingML namespace (`a:`) owns the text
body and all its children; the PresentationML namespace (`p:`) owns the
surrounding shape elements.

```
Namespace URI (DrawingML): http://schemas.openxmlformats.org/drawingml/2006/main
Conventional prefix: a

Namespace URI (PresentationML): http://schemas.openxmlformats.org/presentationml/2006/main
Conventional prefix: p

Namespace URI (relationships): http://schemas.openxmlformats.org/officeDocument/2006/relationships
Conventional prefix: r
```

Schema files: `dml-main.xsd` (DrawingML elements), `pml.xsd`
(PresentationML shape wrappers).

---

## 2. `<a:txBody>` (CT_TextBody)

`<a:txBody>` is the text container. It appears as a child of:

- `<p:sp>` (presentation shape) — the main vehicle for text boxes and
  placeholders
- `<p:graphicFrame>` table cell (`<a:tc>`) — cells inside a DrawingML table
- Connector shapes (`<p:cxnSp>`)
- Picture shapes (`<p:pic>`)

### Child element sequence

**Order is required.** The XSD defines a strict sequence:

| # | Element | Multiplicity | Notes |
|---|---------|-------------|-------|
| 1 | `a:bodyPr` | exactly 1 | Body layout and autofit |
| 2 | `a:lstStyle` | exactly 1 | List paragraph defaults |
| 3 | `a:p` | 1 or more | Paragraphs |

A `<a:txBody>` with zero paragraphs is invalid. Producers must emit at
least one `<a:p>`, even if it is empty (`<a:p><a:endParaRPr/></a:p>`).

```xml
<p:sp>
  <p:nvSpPr>…</p:nvSpPr>
  <p:spPr>…</p:spPr>
  <p:txBody>
    <a:bodyPr/>
    <a:lstStyle/>
    <a:p>
      <a:r><a:t>Hello, world.</a:t></a:r>
    </a:p>
  </p:txBody>
</p:sp>
```

---

## 3. `<a:bodyPr>` (CT_TextBodyProperties)

`<a:bodyPr>` controls how the text body is laid out within its bounding
box: text direction, wrapping, internal padding, vertical alignment, and
the autofit mode.

### Attributes

All attributes are optional unless noted.

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `vert` | ST_TextVerticalType | `horz` | Text direction within the body |
| `wrap` | ST_TextWrappingType | `square` | Line-wrap behaviour |
| `lIns` | ST_Coordinate32 | 91440 | Left internal padding (EMU) |
| `rIns` | ST_Coordinate32 | 91440 | Right internal padding (EMU) |
| `tIns` | ST_Coordinate32 | 45720 | Top internal padding (EMU) |
| `bIns` | ST_Coordinate32 | 45720 | Bottom internal padding (EMU) |
| `anchor` | ST_TextAnchoringType | `t` | Vertical anchor of text block |
| `anchorCtr` | xsd:boolean | false | Center text block within anchor cell |
| `numCol` | xsd:int | 1 | Number of text columns (1–16) |
| `spcCol` | ST_PositiveCoordinate32 | 0 | Gap between columns (EMU) |
| `rtlCol` | xsd:boolean | false | Right-to-left column ordering |
| `upright` | xsd:boolean | false | Keep text upright when shape is rotated |
| `compatLnSpc` | xsd:boolean | false | Use Word-compatible line spacing |
| `rot` | ST_Angle | — | Text body rotation (angle units; rare) |
| `spcFirstLastPara` | xsd:boolean | false | Add before/after spacing to first/last paragraph |
| `horzOverflow` | ST_TextHorzOverflowType | `overflow` | How text overflows horizontally |
| `vertOverflow` | ST_TextVertOverflowType | `overflow` | How text overflows vertically |

#### ST_TextVerticalType (`vert`) values

| Value | Meaning |
|-------|---------|
| `horz` | Normal horizontal text (default) |
| `vert` | Vertical text, rotated 90° clockwise (reading bottom to top) |
| `vert270` | Vertical text, rotated 270° clockwise (reading top to bottom) |
| `eaVert` | East Asian vertical text layout |
| `mongolianVert` | Mongolian vertical text |
| `wordArtVert` | WordArt vertical stacked layout |
| `wordArtVertRtl` | WordArt vertical RTL stacked layout |

#### ST_TextAnchoringType (`anchor`) values

| Value | Meaning |
|-------|---------|
| `t` | Align text block to the top of the body rectangle |
| `ctr` | Center text block vertically |
| `b` | Align text block to the bottom |
| `just` | Distribute lines to fill full height |
| `dist` | Distribute lines including empty space above and below |

### Child element sequence

All children are optional. **Order is required:**

| # | Element | Notes |
|---|---------|-------|
| 1 | `a:prstTxWarp` | Preset text warp (curved text shapes) |
| 2 | autofit choice | At most one of the three autofit elements |
| 3 | `a:scene3d` | 3-D scene properties |
| 4 | 3-D shape choice | One of `a:sp3d` or `a:flatTx` |
| 5 | `a:extLst` | Extension list |

### Autofit choice

Exactly zero or one of the following may appear:

| Element | Meaning |
|---------|---------|
| `<a:noAutofit/>` | No autofit — text clips at the boundary |
| `<a:normAutofit/>` | Reduce font size to fit text in the fixed shape box |
| `<a:spAutoFit/>` | Expand the shape bounding box to fit the text |

When the autofit element is absent the behaviour is equivalent to
`<a:noAutofit/>` for most container types. Placeholder shapes on
layouts and masters typically omit the element and rely on the master
default.

```xml
<!-- Text body: vertical text, bottom-anchored, reduce-to-fit -->
<a:bodyPr vert="vert" anchor="b">
  <a:normAutofit/>
</a:bodyPr>
```

---

## 4. Paragraph — `<a:p>`

Each paragraph is one `<a:p>` element (schema type `CT_TextParagraph`).

### Child element sequence

| # | Element(s) | Multiplicity | Notes |
|---|-----------|-------------|-------|
| 1 | `a:pPr` | 0 or 1 | Paragraph properties (must be first if present) |
| 2 | `a:r` \| `a:br` \| `a:fld` | 0 or more | Inline runs, line breaks, and fields (in any order) |
| 3 | `a:endParaRPr` | 0 or 1 | Paragraph-mark run properties (must be last if present) |

### 4.1 `<a:pPr>` (CT_TextParagraphProperties)

Paragraph-level formatting. All attributes are optional.

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `marL` | ST_TextMargin | — | Left margin (EMU) |
| `marR` | ST_TextMargin | — | Right margin (EMU) |
| `lvl` | xsd:int | 0 | List nesting level (0–8) |
| `indent` | ST_TextIndent | — | First-line indent (EMU; negative = hanging) |
| `algn` | ST_TextAlignType | — | Horizontal text alignment |
| `defTabSz` | ST_Coordinate32 | — | Default tab stop size (EMU) |
| `rtl` | xsd:boolean | false | Right-to-left paragraph |
| `eaLnBrk` | xsd:boolean | true | Allow East Asian line breaks |
| `fontAlgn` | ST_TextFontAlignType | `auto` | Vertical font alignment within line |
| `latinLnBrk` | xsd:boolean | true | Allow line breaks in Latin text |
| `hangingPunct` | xsd:boolean | false | Allow punctuation to hang outside margin |

#### ST_TextAlignType (`algn`) values

| Value | Meaning |
|-------|---------|
| `l` | Left-aligned |
| `ctr` | Centered |
| `r` | Right-aligned |
| `just` | Justified (aligned to both margins) |
| `justLow` | Justified with low kashida (Arabic) |
| `dist` | Distributed (space between all characters) |
| `thaiDist` | Thai distributed alignment |

#### ST_TextFontAlignType (`fontAlgn`) values

| Value | Meaning |
|-------|---------|
| `auto` | Application-defined (default) |
| `t` | Top of the line box |
| `ctr` | Center of the line box |
| `b` | Bottom (baseline) of the line box |
| `base` | Typographic baseline |

#### Children of `<a:pPr>` (in sequence)

| # | Element | Notes |
|---|---------|-------|
| 1 | `a:lnSpc` | Line spacing |
| 2 | `a:spcBef` | Space before paragraph |
| 3 | `a:spcAft` | Space after paragraph |
| 4 | bullet group | One of: `a:buClrTx`, `a:buClr`, `a:buSzTx`, `a:buSzPct`, `a:buSzPts`, `a:buFontTx`, `a:buFont`, `a:buNone`, `a:buAutoNum`, `a:buChar`, `a:buBlip` |
| 5 | `a:tabLst` | Custom tab stop list |
| 6 | `a:defRPr` | Default run properties for the paragraph |
| 7 | `a:extLst` | Extension list |

#### Spacing elements

Spacing elements (`a:lnSpc`, `a:spcBef`, `a:spcAft`) each contain exactly
one spacing child:

- `<a:spcPts val="N"/>` — spacing in hundredths of a point (e.g. `val="1200"` = 12 pt)
- `<a:spcPct val="N"/>` — spacing as thousandths of a percent (e.g. `val="100000"` = 100%)

```xml
<!-- 1.5× line spacing, 12 pt before paragraph -->
<a:pPr>
  <a:lnSpc><a:spcPct val="150000"/></a:lnSpc>
  <a:spcBef><a:spcPts val="1200"/></a:spcBef>
</a:pPr>
```

### 4.2 Second-level bullet paragraph

The `lvl` attribute drives indentation and bullet formatting via the
`<a:lstStyle>` hierarchy. `lvl="0"` is the first level (default); `lvl="1"` is
the second level; the maximum is `lvl="8"`.

```xml
<a:p>
  <a:pPr lvl="1"/>
  <a:r>
    <a:t>Second-level bullet item.</a:t>
  </a:r>
</a:p>
```

### 4.3 `<a:endParaRPr>`

The paragraph-mark run properties element (`CT_TextCharacterProperties`)
controls the appearance of the invisible paragraph mark (the cursor when
the paragraph is empty) and provides fallback formatting for the
paragraph. It must appear as the **last** child of `<a:p>` if present.
It carries the same attributes as `<a:rPr>` but applies only to the
paragraph mark itself.

---

## 5. Run — `<a:r>` and Run Properties

### 5.1 `<a:r>` (CT_RegularTextRun)

A run is a contiguous span of text sharing the same character formatting.

**Child element sequence:**

| # | Element | Multiplicity | Notes |
|---|---------|-------------|-------|
| 1 | `a:rPr` | 0 or 1 | Run properties |
| 2 | `a:t` | exactly 1 | Text content |

### 5.2 `<a:rPr>` (CT_TextCharacterProperties)

All attributes are optional.

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `lang` | xsd:string | — | BCP 47 language tag (e.g. `"en-US"`) |
| `altLang` | xsd:string | — | Alternate language for rendering |
| `sz` | ST_TextFontSize | — | Font size in hundredths of a point (e.g. `2400` = 24 pt) |
| `b` | xsd:boolean | false | Bold |
| `i` | xsd:boolean | false | Italic |
| `u` | ST_TextUnderlineType | `none` | Underline style |
| `strike` | ST_TextStrikeType | `noStrike` | Strikethrough style |
| `kern` | ST_TextNonNegativePoint | — | Kerning minimum size (hundredths of a point) |
| `spc` | ST_TextPoint | — | Character spacing (hundredths of a point; negative = tighter) |
| `baseline` | ST_Percentage | — | Baseline shift in thousandths of a percent (positive = superscript, negative = subscript) |
| `dirty` | xsd:boolean | true | Spell-check dirty flag |
| `smtId` | xsd:unsignedInt | — | Smart tag identifier |
| `bmk` | xsd:string | — | Bookmark name for linking |
| `err` | xsd:boolean | false | Known spelling error |
| `noProof` | xsd:boolean | false | Skip spell/grammar check |
| `normalizeH` | xsd:boolean | false | Normalize character heights |
| `kumimoji` | xsd:boolean | false | Kumimoji (CJK upright characters in vertical text) |

#### ST_TextUnderlineType (`u`) values (selected)

| Value | Description |
|-------|-------------|
| `none` | No underline (default) |
| `sng` | Single underline |
| `dbl` | Double underline |
| `heavy` | Heavy (thick) underline |
| `dotted` | Dotted underline |
| `dottedHeavy` | Dotted heavy underline |
| `dash` | Dashed underline |
| `dashHeavy` | Dashed heavy underline |
| `dashLong` | Long-dash underline |
| `dashLongHeavy` | Long-dash heavy underline |
| `dotDash` | Dot-dash underline |
| `dotDashHeavy` | Dot-dash heavy underline |
| `dotDotDash` | Dot-dot-dash underline |
| `dotDotDashHeavy` | Dot-dot-dash heavy underline |
| `wavy` | Wavy underline |
| `wavyHeavy` | Wavy heavy underline |
| `wavyDbl` | Wavy double underline |
| `words` | Underline words only (not spaces) |

#### ST_TextStrikeType (`strike`) values

| Value | Description |
|-------|-------------|
| `noStrike` | No strikethrough (default) |
| `sngStrike` | Single strikethrough |
| `dblStrike` | Double strikethrough |

#### Children of `<a:rPr>` (in sequence)

| # | Element(s) | Notes |
|---|-----------|-------|
| 1 | `a:ln` | Outline (stroke) on the character |
| 2 | fill choice | One of `a:noFill`, `a:solidFill`, `a:gradFill`, `a:blipFill`, `a:pattFill`, `a:grpFill` |
| 3 | effect choice | One of `a:effectLst`, `a:effectDag` |
| 4 | `a:highlight` | Highlight fill colour |
| 5 | uLn choice | One of `a:uLnTx`, `a:uLn` (underline line properties) |
| 6 | uFill choice | One of `a:uFillTx`, `a:uFill` (underline fill) |
| 7 | font substitution group | `a:latin`, `a:ea`, `a:cs`, `a:sym` |
| 8 | `a:hlinkClick` | Hyperlink on click |
| 9 | `a:hlinkMouseOver` | Hyperlink on hover |
| 10 | `a:rtl` | Right-to-left rendering override |
| 11 | `a:extLst` | Extension list |

### 5.3 `<a:t>` — text content

The `<a:t>` element carries the raw text of the run. Use the
`xml:space="preserve"` attribute when the text begins or ends with
whitespace characters — without it, XML parsers are permitted to strip
leading and trailing spaces.

```xml
<a:r>
  <a:rPr lang="en-US" sz="1800" b="1"/>
  <a:t xml:space="preserve"> Bold text with leading space.</a:t>
</a:r>
```

---

## 6. Hyperlink in Run — `<a:hlinkClick>`

Hyperlinks are attached to runs by placing `<a:hlinkClick>` inside
`<a:rPr>`. The relationship pointed to by `r:id` must be declared in
the slide's `.rels` file.

### Attributes

| Attribute | Type | Notes |
|-----------|------|-------|
| `r:id` | xsd:string | Relationship ID (required for functional hyperlinks) |
| `invalidUrl` | xsd:string | Original URL if the relationship target is invalid |
| `action` | xsd:string | Action string (e.g. `"ppaction://hlinksldjump"` for slide jumps) |
| `tgtFrame` | xsd:string | HTML target frame (`"_blank"`, `"_self"`, etc.) |
| `tooltip` | xsd:string | Tooltip text displayed on hover |
| `highlightClick` | xsd:boolean | Whether the hyperlink is highlighted after clicking |
| `endSnd` | xsd:boolean | Stop currently playing sound when hyperlink is activated |

For external URLs, the relationship in the `.rels` file must use
`TargetMode="External"` and relationship type
`http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink`.

For internal slide jumps (e.g. slide N), use the `action` attribute with
the `ppaction://` scheme; `r:id` may be absent.

### Children

| Element | Notes |
|---------|-------|
| `a:snd` | Sound played when the hyperlink is clicked |
| `a:extLst` | Extension list |

```xml
<!-- Run with a hyperlink to an external URL -->
<a:r>
  <a:rPr lang="en-US" sz="1800">
    <a:hlinkClick r:id="rId2"/>
  </a:rPr>
  <a:t>Visit our website</a:t>
</a:r>
```

The corresponding `.rels` entry:

```xml
<Relationship Id="rId2"
  Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink"
  Target="https://example.com"
  TargetMode="External"/>
```

---

## 7. Line Break — `<a:br>`

A soft return within a paragraph (analogous to Shift+Enter in a
presentation editor) is represented by `<a:br>` (schema type
`CT_TextLineBreak`). It is a sibling of `<a:r>` inside `<a:p>` and
creates a new line without starting a new paragraph.

### Child element

| Element | Multiplicity | Notes |
|---------|-------------|-------|
| `a:rPr` | 0 or 1 | Run properties for the break character |

```xml
<a:p>
  <a:r><a:t>First line</a:t></a:r>
  <a:br>
    <a:rPr lang="en-US"/>
  </a:br>
  <a:r><a:t>Second line (same paragraph)</a:t></a:r>
</a:p>
```

---

## 8. `<a:lstStyle>` (CT_TextListStyle)

`<a:lstStyle>` carries default paragraph properties for each nesting
level. It appears as the second child of `<a:txBody>` (between
`<a:bodyPr>` and the first `<a:p>`).

### Children

| Element | Notes |
|---------|-------|
| `a:defPPr` | Default paragraph properties (applies to all levels) |
| `a:lvl1pPr` | Level 1 (lvl=0) paragraph properties |
| `a:lvl2pPr` | Level 2 (lvl=1) paragraph properties |
| … | … |
| `a:lvl9pPr` | Level 9 (lvl=8) paragraph properties |

Each `a:lvlNpPr` element is of type `CT_TextParagraphProperties` — the
same type as `<a:pPr>` — and carries the same set of attributes and
children.

In slide bodies, `<a:lstStyle/>` is almost always empty; the actual
list style is inherited from the slide layout, then the slide master.
Slide masters and note masters typically populate `<a:lstStyle>` with
level-specific bullet fonts, sizes, and indents.

```xml
<!-- Typical empty lstStyle on a content slide -->
<a:lstStyle/>
```

---

## 9. Property Inheritance

Text properties in PresentationML are resolved through an inheritance
chain. The most specific value wins:

1. Direct formatting in `<a:rPr>` / `<a:pPr>` (highest priority)
2. Default run properties `<a:defRPr>` in the paragraph's `<a:pPr>`
3. Level-specific properties `<a:lvlNpPr>` in the slide's `<a:lstStyle>`
4. Level-specific properties in the slide layout's corresponding placeholder
5. Level-specific properties in the slide master's corresponding placeholder
6. Level-specific properties in the slide master's `<p:txStyles>` (title/body/other)
7. Application defaults

Shape-level properties (font, size, colour) defined in `<p:spPr>` and
`<p:style>` apply to the shape boundary and fill, not to run text.

---

## 10. XML Examples

### Example 1 — Vertical text body, bottom-anchored, reduce-to-fit

```xml
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="3" name="TextBox 2"/>
    <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
    <p:nvPr/>
  </p:nvSpPr>
  <p:spPr>
    <a:xfrm>
      <a:off x="457200" y="274638"/>
      <a:ext cx="2743200" cy="1828800"/>
    </a:xfrm>
    <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
  </p:spPr>
  <p:txBody>
    <a:bodyPr vert="vert" anchor="b">
      <a:normAutofit/>
    </a:bodyPr>
    <a:lstStyle/>
    <a:p>
      <a:r>
        <a:rPr lang="en-US" dirty="0"/>
        <a:t>Rotated text, bottom-anchored.</a:t>
      </a:r>
    </a:p>
  </p:txBody>
</p:sp>
```

### Example 2 — Second-level bullet paragraph

```xml
<a:p>
  <a:pPr lvl="1"/>
  <a:r>
    <a:rPr lang="en-US" sz="1800" dirty="0"/>
    <a:t>Indented second-level bullet.</a:t>
  </a:r>
</a:p>
```

### Example 3 — Run with a hyperlink

```xml
<a:p>
  <a:r>
    <a:rPr lang="en-US" sz="2000" dirty="0">
      <a:hlinkClick r:id="rId2"/>
    </a:rPr>
    <a:t>Click here</a:t>
  </a:r>
</a:p>
```

---

## 11. Round-Trip Gotchas

1. **`<a:txBody>` requires at least one `<a:p>`.** A text body with zero
   paragraphs is schema-invalid. Producers must always emit at least one
   paragraph, typically `<a:p><a:endParaRPr lang="en-US" dirty="0"/></a:p>`.

2. **`<a:lstStyle/>` must be present, even when empty.** Omitting it
   produces an invalid `<a:txBody>` because the schema requires the three
   children in order.

3. **`a:rPr` precedes `a:t` inside `<a:r>`.** Placing `<a:t>` before
   `<a:rPr>` is out-of-sequence and may be rejected by strict validators.

4. **`xml:space="preserve"` on `<a:t>`.** Without this attribute, XML
   parsers normalise whitespace. Any run whose text starts or ends with a
   space must carry the attribute.

5. **`sz` is in hundredths of a point.** A 24 pt font is `sz="2400"`.
   This differs from WordprocessingML which uses half-points.

6. **`baseline` is in thousandths of a percent.** Superscript is commonly
   `baseline="30000"` (30 %). Subscript is `baseline="-25000"` (-25 %).

7. **`lvl` range is 0–8.** Level 0 is the top-level bullet. Value 9 is not
   defined. Out-of-range values should be clamped or rejected.

8. **`<a:endParaRPr>` must be last in the paragraph.** If present, no
   further run siblings may follow it.

9. **Hyperlinks require a matching relationship entry.** A `r:id` with no
   corresponding `<Relationship>` in the `.rels` file produces a broken
   link at render time; the SDK should treat missing targets as a warning,
   not a parse error.

10. **`<a:br>` is a paragraph child, not a run child.** It appears directly
    inside `<a:p>`, not inside `<a:r>`.

---

## 12. Fixture Plan

| ID | File | Properties covered |
|----|------|--------------------|
| PML-T-01 | `test-data/pml/txbody_basic.pptx` | `<a:txBody>` with `bodyPr`, `lstStyle`, single `<a:p>` with one run |
| PML-T-02 | `test-data/pml/txbody_vert.pptx` | `vert="vert"`, `anchor="b"`, `<a:normAutofit/>` |
| PML-T-03 | `test-data/pml/txbody_bullets.pptx` | Multi-level paragraphs with `lvl="0"` through `lvl="2"`; `<a:lstStyle>` with level defaults |
| PML-T-04 | `test-data/pml/txbody_hyperlink.pptx` | `<a:hlinkClick r:id="rId2"/>` with matching External relationship |
| PML-T-05 | `test-data/pml/txbody_linebreak.pptx` | `<a:br>` soft return within a paragraph |
| PML-T-06 | `test-data/pml/txbody_richtext.pptx` | Mixed bold/italic/underline runs; superscript `baseline`; `xml:space="preserve"` |
