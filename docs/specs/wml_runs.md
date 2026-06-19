# WordprocessingML Paragraph and Run Model — Clean-Room Specification
## Paragraph, Run, and Run-Property Structure for OOXML Parsers

**Purpose:** This document is a clean-room synthesis of the WordprocessingML
paragraph and run model as defined in ECMA-376 Part 1 (5th edition) and
ISO/IEC 29500-1:2016. It covers the structural elements that make up the
body of a Word document — paragraphs (`<w:p>`), runs (`<w:r>`), text nodes
(`<w:t>`), and all run-level formatting properties (`<w:rPr>`). It is intended
as an implementation guide and test oracle for `ooxmlsdk` and similar Rust
OOXML parsers.

**Sources used:**
- ECMA-376 Part 1, 5th edition — normative XML structure
- ISO/IEC 29500-1:2016 — cross-reference for WML semantics
- `wml.xsd` from this repository (`schemas/OfficeOpenXML-XMLSchema-Transitional/`)
- `shared-commonSimpleTypes.xsd` from the same directory
- Microsoft Open Specifications: `[MS-DOCX]`, `[MS-OI29500]`
- Existing `ooxmlsdk` test fixtures (`../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/document/`)
- python-docx and Apache POI source code (reference implementations)

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or have been clearly
rewritten.

---

## 1. Namespace and Conventional Prefix

```
Namespace URI:  http://schemas.openxmlformats.org/wordprocessingml/2006/main
Conventional prefix: w
Schema file: wml.xsd
```

All elements described in this document are in the `w:` namespace unless
otherwise stated. The `attributeFormDefault="qualified"` declaration in
`wml.xsd` means that all `w:` attributes also require the namespace prefix.

Related namespaces that appear alongside WordprocessingML:

| Prefix | Namespace URI | Usage |
|---|---|---|
| `r` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships` | Relationship IDs on hyperlinks, images, embedded objects |
| `w14` | `http://schemas.microsoft.com/office/word/2010/wordml` | Word 2010+ extension run properties (glow, shadow, reflection) |
| `w15` | `http://schemas.microsoft.com/office/word/2012/wordml` | Word 2013+ extensions |
| `mc` | `http://schemas.openxmlformats.org/markup-compatibility/2006` | MCE Ignorable / AlternateContent (see `docs/specs/mce.md`) |
| `wp` | `http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing` | Inline and anchored drawing wrappers |
| `a` | `http://schemas.openxmlformats.org/drawingml/2006/main` | DrawingML shapes inside `<w:drawing>` |

---

## 2. Document Structure

### 2.1 Root Element: `<w:document>`

The root element of `word/document.xml` is `<w:document>` (schema type
`CT_Document`). It carries the primary namespace declarations and optionally
a `conformance` attribute (`strict` or `transitional`).

```xml
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
    mc:Ignorable="w14">
  <w:body>
    ...
  </w:body>
</w:document>
```

**Schema-defined children of `CT_Document`** (in order):
1. `<w:background>` — optional, document background colour/image
2. `<w:body>` — optional (0 or 1), the document body

The `background` element comes before `body` because `CT_Document` extends
`CT_DocumentBase` (which holds `background`) and then adds `body`.

### 2.2 `<w:body>` — Document Body

Schema type `CT_Body`. Contains:

1. Zero or more block-level elements from the `EG_BlockLevelElts` group
   (which expands to: `<w:p>`, `<w:tbl>`, `<w:sdt>`, `<w:customXml>`,
   `<w:altChunk>`, run-level elements from `EG_RunLevelElts` such as
   `<w:bookmarkStart>`, `<w:bookmarkEnd>`, `<w:ins>`, `<w:del>`, `<w:proofErr>`)
2. Optionally, exactly one `<w:sectPr>` as the very last child

**The `<w:sectPr>` placement rule:** The XSD schema enforces that `<w:sectPr>`
is declared separately from the `EG_BlockLevelElts` group — it appears as a
distinct terminal element in `CT_Body`. A conforming producer MUST place
`<w:sectPr>` as the last child of `<w:body>`. Placing it anywhere else
produces a schema-invalid document.

```xml
<w:body>
  <w:p>...</w:p>
  <w:tbl>...</w:tbl>
  <w:p>...</w:p>
  <w:sectPr>
    <w:pgSz w:w="12240" w:h="15840"/>
    <w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440"
             w:header="720" w:footer="720" w:gutter="0"/>
  </w:sectPr>
</w:body>
```

A body that contains only `<w:sectPr>` and no paragraphs is valid (see
`../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/document/minimal_empty.docx`).

---

## 3. `<w:p>` — Paragraph

Schema type `CT_P`. The paragraph is the fundamental unit of block-level
content in WordprocessingML.

### 3.1 CT_P Schema

```xsd
<xsd:complexType name="CT_P">
  <xsd:sequence>
    <xsd:element name="pPr" type="CT_PPr" minOccurs="0"/>
    <xsd:group ref="EG_PContent" minOccurs="0" maxOccurs="unbounded"/>
  </xsd:sequence>
  <xsd:attribute name="rsidRPr" type="ST_LongHexNumber"/>
  <xsd:attribute name="rsidR"   type="ST_LongHexNumber"/>
  <xsd:attribute name="rsidDel" type="ST_LongHexNumber"/>
  <xsd:attribute name="rsidP"   type="ST_LongHexNumber"/>
  <xsd:attribute name="rsidRDefault" type="ST_LongHexNumber"/>
</xsd:complexType>
```

The `EG_PContent` group expands to:
- `<w:r>` — run (via `EG_ContentRunContent`)
- `<w:hyperlink>` — hyperlink wrapper
- `<w:fldSimple>` — simple field
- `<w:sdt>` — structured document tag (run-level)
- `<w:customXml>` — custom XML run
- `<w:smartTag>` — smart tag (deprecated)
- `<w:ins>` — tracked insertion (via `EG_RunLevelElts`)
- `<w:del>` — tracked deletion
- `<w:moveFrom>` / `<w:moveTo>` — tracked move
- `<w:bookmarkStart>` / `<w:bookmarkEnd>` — bookmark markers
- `<w:proofErr>` — spell/grammar error markers
- `<w:subDoc>` — sub-document reference
- Math content elements

### 3.2 Paragraph Attributes

| Attribute | Type | Description |
|---|---|---|
| `w:rsidR` | `ST_LongHexNumber` (4-byte hex) | Revision save ID when the paragraph was inserted |
| `w:rsidRPr` | `ST_LongHexNumber` | Revision save ID of the last run-property change |
| `w:rsidDel` | `ST_LongHexNumber` | Revision save ID when paragraph was deleted (in tracked changes) |
| `w:rsidP` | `ST_LongHexNumber` | Revision save ID of the last paragraph-property change |
| `w:rsidRDefault` | `ST_LongHexNumber` | Default rsid for runs in the paragraph |

See §12 for the semantics of `rsid*` attributes.

### 3.3 Example Paragraphs

**Minimal paragraph (no properties, one run):**

```xml
<w:p>
  <w:r>
    <w:t>Hello world</w:t>
  </w:r>
</w:p>
```

**Paragraph with style and run formatting:**

```xml
<w:p w:rsidR="00A1B2C3" w:rsidRDefault="00A1B2C3">
  <w:pPr>
    <w:pStyle w:val="Heading1"/>
  </w:pPr>
  <w:r w:rsidR="00A1B2C3">
    <w:rPr>
      <w:b/>
      <w:color w:val="1F4E79"/>
    </w:rPr>
    <w:t>Chapter Introduction</w:t>
  </w:r>
</w:p>
```

**Empty paragraph (paragraph mark only):**

```xml
<w:p/>
```

An empty paragraph contains exactly one invisible character — the paragraph
mark. Its formatting is determined by `<w:pPr><w:rPr>`, which is the run
properties of the paragraph mark.

---

## 4. `<w:pPr>` — Paragraph Properties

Schema type `CT_PPr` extends `CT_PPrBase`. This section provides an overview
of the most important paragraph properties. Deep coverage of paragraph
formatting (indentation, spacing, borders, tabs, list numbering, styles) is
reserved for a forthcoming `docs/specs/wml_paragraphs.md`.

### 4.1 CT_PPr Schema Overview

`CT_PPr` extends `CT_PPrBase` with three additional elements at the end:

```xsd
<xsd:complexType name="CT_PPr">
  <xsd:complexContent>
    <xsd:extension base="CT_PPrBase">
      <xsd:sequence>
        <xsd:element name="rPr"       type="CT_ParaRPr" minOccurs="0"/>
        <xsd:element name="sectPr"    type="CT_SectPr"  minOccurs="0"/>
        <xsd:element name="pPrChange" type="CT_PPrChange" minOccurs="0"/>
      </xsd:sequence>
    </xsd:extension>
  </xsd:complexContent>
</xsd:complexType>
```

The `rPr` inside `CT_PPr` is the paragraph mark's run properties — it formats
the invisible paragraph mark character. It uses type `CT_ParaRPr`, which adds
insertion/deletion tracking fields before the standard `EG_RPrBase` choices.

`sectPr` inside `<w:pPr>` represents a section break at this paragraph:
it is a section properties block for the section that ends here. This is NOT
the same as the `<w:sectPr>` at the end of `<w:body>`.

### 4.2 CT_PPrBase — Core Paragraph Properties

The children of `CT_PPrBase` appear in strict schema order:

| Element | Type | Description |
|---|---|---|
| `<w:pStyle w:val="..."/>` | `CT_String` | Named paragraph style to apply |
| `<w:keepNext/>` | `CT_OnOff` | Keep paragraph on same page as following paragraph |
| `<w:keepLines/>` | `CT_OnOff` | Keep all lines of this paragraph on the same page |
| `<w:pageBreakBefore/>` | `CT_OnOff` | Force page break before this paragraph |
| `<w:framePr .../>` | `CT_FramePr` | Text frame (legacy floating frame) |
| `<w:widowControl/>` | `CT_OnOff` | Control widow/orphan lines |
| `<w:numPr>` | `CT_NumPr` | List numbering reference (see §4.3) |
| `<w:suppressLineNumbers/>` | `CT_OnOff` | Suppress line numbering |
| `<w:pBdr>` | `CT_PBdr` | Paragraph borders |
| `<w:shd .../>` | `CT_Shd` | Paragraph background shading |
| `<w:tabs>` | `CT_Tabs` | Tab stop definitions |
| `<w:suppressAutoHyphens/>` | `CT_OnOff` | Disable automatic hyphenation |
| `<w:kinsoku/>` | `CT_OnOff` | Enable Japanese kinsoku line-break rules |
| `<w:wordWrap/>` | `CT_OnOff` | Enable word wrapping |
| `<w:overflowPunct/>` | `CT_OnOff` | Allow punctuation to overflow margin |
| `<w:topLinePunct/>` | `CT_OnOff` | Compress punctuation at top of line |
| `<w:autoSpaceDE/>` | `CT_OnOff` | Auto-spacing between Latin and East Asian text |
| `<w:autoSpaceDN/>` | `CT_OnOff` | Auto-spacing between East Asian text and numerals |
| `<w:bidi/>` | `CT_OnOff` | Right-to-left paragraph direction |
| `<w:adjustRightInd/>` | `CT_OnOff` | Adjust right indent for document grid |
| `<w:snapToGrid/>` | `CT_OnOff` | Snap to document grid |
| `<w:spacing .../>` | `CT_Spacing` | Space before/after/line spacing (see §4.4) |
| `<w:ind .../>` | `CT_Ind` | Left/right/first-line/hanging indents (see §4.5) |
| `<w:contextualSpacing/>` | `CT_OnOff` | Suppress space between paragraphs of same style |
| `<w:mirrorIndents/>` | `CT_OnOff` | Mirror indents for odd/even pages |
| `<w:suppressOverlap/>` | `CT_OnOff` | Prevent overlap with floating objects |
| `<w:jc w:val="..."/>` | `CT_Jc` | Paragraph justification (see §4.6) |
| `<w:textDirection .../>` | `CT_TextDirection` | Text flow direction |
| `<w:textAlignment .../>` | `CT_TextAlignment` | Vertical text alignment |
| `<w:textboxTightWrap .../>` | `CT_TextboxTightWrap` | Tighten text-to-text-box wrapping |
| `<w:outlineLvl w:val="..."/>` | `CT_DecimalNumber` | Outline level 0–8 (0=Heading 1 style) |
| `<w:divId w:val="..."/>` | `CT_DecimalNumber` | Web-division ID |
| `<w:cnfStyle .../>` | `CT_Cnf` | Conditional table formatting style |

This strict child ordering is enforced by the XML Schema. Reordering these
elements produces a schema-invalid document. See §14 for the common parser
failure this causes.

### 4.3 `<w:numPr>` — List Numbering Reference

`CT_NumPr` contains:

```xml
<w:numPr>
  <w:ilvl w:val="0"/>   <!-- indent level, 0-based -->
  <w:numId w:val="1"/>  <!-- references CT_Num in numbering.xml -->
</w:numPr>
```

`w:numId` of `0` removes numbering inherited from a style. The actual list
definition (format, indent, bullet character, number style) lives in
`word/numbering.xml`, referenced by the `numId` value.

### 4.4 `<w:spacing>` — Paragraph Spacing

Schema type `CT_Spacing`:

| Attribute | Type | Default | Description |
|---|---|---|---|
| `w:before` | `ST_TwipsMeasure` (unsigned int or measure string) | 0 | Space before paragraph in twentieths of a point (twips) |
| `w:beforeLines` | integer | 0 | Space before in hundredths of a line |
| `w:beforeAutospacing` | `ST_OnOff` | off | Automatically set space before |
| `w:after` | `ST_TwipsMeasure` | 0 | Space after paragraph in twips |
| `w:afterLines` | integer | 0 | Space after in hundredths of a line |
| `w:afterAutospacing` | `ST_OnOff` | off | Automatically set space after |
| `w:line` | `ST_SignedTwipsMeasure` | 0 | Line height; interpretation depends on `lineRule` |
| `w:lineRule` | `ST_LineSpacingRule` | `auto` | `auto` (min), `exact` (exact), `atLeast` (min) |

**`lineRule` semantics:**
- `auto`: `line` value is a multiple of the default line height in 240ths (so 240 = single-space, 360 = 1.5×, 480 = double).
- `exact`: `line` is an exact line height in twips; lines that are too tall are clipped.
- `atLeast`: `line` is a minimum height in twips; taller lines expand to fit.

**Example — double-spaced, 12pt space after:**
```xml
<w:spacing w:before="0" w:after="240" w:line="480" w:lineRule="auto"/>
```

### 4.5 `<w:ind>` — Paragraph Indentation

Schema type `CT_Ind`:

| Attribute | Description |
|---|---|
| `w:start` | Logical start-side indent (twips); replaces `left` in strict OOXML |
| `w:startChars` | Start indent in hundredths of a character width |
| `w:end` | Logical end-side indent (twips); replaces `right` |
| `w:endChars` | End indent in hundredths of a character width |
| `w:left` | Left indent in twips (transitional; equivalent to `start` in LTR) |
| `w:leftChars` | Left indent in hundredths of a character width |
| `w:right` | Right indent in twips (transitional) |
| `w:rightChars` | Right indent in hundredths of a character width |
| `w:hanging` | Hanging indent in twips (reduces all lines except first) |
| `w:hangingChars` | Hanging indent in hundredths of a character width |
| `w:firstLine` | First-line indent in twips (increases only the first line) |
| `w:firstLineChars` | First-line indent in hundredths of a character width |

`firstLine` and `hanging` are mutually exclusive — a paragraph has one or the
other. `hanging` takes logical precedence if both are somehow present.

**Example — 0.5-inch left indent, 0.25-inch hanging:**
```xml
<w:ind w:left="720" w:hanging="360"/>
```
(720 twips = 0.5 in; 360 twips = 0.25 in; 1 inch = 1440 twips)

### 4.6 `<w:jc>` — Justification

Schema type `CT_Jc`, attribute `w:val` of type `ST_Jc`:

| Value | Meaning |
|---|---|
| `start` | Logical start alignment (LTR → left, RTL → right) |
| `center` | Centred |
| `end` | Logical end alignment |
| `both` | Full justification |
| `distribute` | Distribute characters evenly (East Asian) |
| `left` | Left-aligned (transitional alias for `start`) |
| `right` | Right-aligned (transitional alias for `end`) |
| `mediumKashida` | Arabic kashida (medium) |
| `highKashida` | Arabic kashida (high) |
| `lowKashida` | Arabic kashida (low) |
| `numTab` | Justify to numeric tab |
| `thaiDistribute` | Thai character distribution |

**Note:** The transitional schema adds `left` and `right` as valid values
alongside the strict-mode `start`/`end`. A parser must accept all values.

### 4.7 `<w:pPrChange>` — Tracked Paragraph Property Change

`CT_PPrChange` extends `CT_TrackChange` with a required `<w:pPr>` child:

```xml
<w:pPrChange w:id="1" w:author="Alice" w:date="2026-04-01T10:00:00Z">
  <w:pPr>
    <!-- original (pre-change) paragraph properties -->
    <w:jc w:val="left"/>
  </w:pPr>
</w:pPrChange>
```

The `<w:pPr>` child here uses type `CT_PPrBase` (not full `CT_PPr`) — it does
not carry `rPr`, `sectPr`, or a nested `pPrChange`.

---

## 5. `<w:r>` — Run

Schema type `CT_R`. A run is a contiguous sequence of characters that share
the same formatting.

### 5.1 CT_R Schema

```xsd
<xsd:complexType name="CT_R">
  <xsd:sequence>
    <xsd:group ref="EG_RPr" minOccurs="0"/>
    <xsd:group ref="EG_RunInnerContent" minOccurs="0" maxOccurs="unbounded"/>
  </xsd:sequence>
  <xsd:attribute name="rsidRPr" type="ST_LongHexNumber"/>
  <xsd:attribute name="rsidDel" type="ST_LongHexNumber"/>
  <xsd:attribute name="rsidR"   type="ST_LongHexNumber"/>
</xsd:complexType>
```

`EG_RPr` is a sequence group that optionally contains exactly one `<w:rPr>`.
`EG_RunInnerContent` is a choice group; it may be repeated zero or more times,
allowing multiple content children of different types in any combination.

### 5.2 Run Attributes

| Attribute | Type | Description |
|---|---|---|
| `w:rsidR` | `ST_LongHexNumber` | Session ID when this run was inserted |
| `w:rsidRPr` | `ST_LongHexNumber` | Session ID of last run-property change |
| `w:rsidDel` | `ST_LongHexNumber` | Session ID when run was deleted (tracked change) |

### 5.3 Run Inner Content Elements

The `EG_RunInnerContent` choice group contains the following elements:

| Element | Type | Description |
|---|---|---|
| `<w:t>` | `CT_Text` | Text characters (see §6) |
| `<w:br>` | `CT_Br` | Break — page, column, or line (see §7) |
| `<w:tab/>` | `CT_Empty` | Tab character (see §8) |
| `<w:noBreakHyphen/>` | `CT_Empty` | Non-breaking hyphen |
| `<w:softHyphen/>` | `CT_Empty` | Soft (optional) hyphen |
| `<w:sym>` | `CT_Sym` | Symbol character (font + code point) |
| `<w:cr/>` | `CT_Empty` | Carriage return (legacy soft return equivalent) |
| `<w:instrText>` | `CT_Text` | Field instruction text |
| `<w:delText>` | `CT_Text` | Deleted text in a tracked-change `<w:del>` run |
| `<w:delInstrText>` | `CT_Text` | Deleted field instruction text |
| `<w:fldChar>` | `CT_FldChar` | Field character (begin/separate/end) |
| `<w:drawing>` | `CT_Drawing` | Inline or anchored drawing (images, charts) |
| `<w:object>` | `CT_Object` | OLE object |
| `<w:pict>` | `CT_Picture` | Legacy VML picture |
| `<w:footnoteReference>` | `CT_FtnEdnRef` | Footnote reference mark |
| `<w:endnoteReference>` | `CT_FtnEdnRef` | Endnote reference mark |
| `<w:commentReference>` | `CT_Markup` | Comment reference mark |
| `<w:ruby>` | `CT_Ruby` | Ruby (furigana) annotation |
| `<w:ptab>` | `CT_PTab` | Position tab (left/center/right relative to margin or indent) |
| `<w:lastRenderedPageBreak/>` | `CT_Empty` | Informational: position of last page break on save |
| `<w:contentPart>` | `CT_Rel` | External content reference (transitional) |
| Date/time placeholders | `CT_Empty` | `<w:dayShort/>`, `<w:monthShort/>`, `<w:yearShort/>`, `<w:dayLong/>`, `<w:monthLong/>`, `<w:yearLong/>` |
| Annotation markers | `CT_Empty` | `<w:annotationRef/>`, `<w:footnoteRef/>`, `<w:endnoteRef/>` |
| Separator marks | `CT_Empty` | `<w:separator/>`, `<w:continuationSeparator/>` |
| `<w:pgNum/>` | `CT_Empty` | Page number placeholder |

**Multiple content elements in one run:** The schema allows a single `<w:r>`
to contain multiple `EG_RunInnerContent` elements in sequence. In practice,
Microsoft Word always places one `<w:t>` per run, but parsers must handle
runs with multiple or mixed content children (e.g., a `<w:t>` followed by
a `<w:br/>`).

### 5.4 `<w:sym>` — Symbol

`CT_Sym` carries a symbol from a specific font:

```xml
<w:sym w:font="Wingdings" w:char="F0B7"/>
```

`w:font` is a string font family name. `w:char` is a two-byte hex Unicode
code point within that font (type `ST_ShortHexNumber`). This element is used
for characters that are not in standard Unicode ranges when accessed via a
symbol font.

---

## 6. `<w:t>` — Text

Schema type `CT_Text`. This is the element that carries the actual character
data of a run.

### 6.1 CT_Text Schema

```xsd
<xsd:complexType name="CT_Text">
  <xsd:simpleContent>
    <xsd:extension base="s:ST_String">
      <xsd:attribute ref="xml:space" use="optional"/>
    </xsd:extension>
  </xsd:simpleContent>
</xsd:complexType>
```

`CT_Text` is a simple-content type: the element's text content is the string
value (type `s:ST_String`, which is `xsd:string`). The only attribute is the
standard `xml:space` attribute from the XML namespace.

### 6.2 `xml:space="preserve"` — Whitespace Handling

**The XML specification defines two whitespace-handling modes for element
content:**

- **Default mode** (no `xml:space` attribute, or `xml:space="default"`): XML
  processors MAY trim leading and trailing whitespace from the element's text
  content. Word processors that consume OOXML MUST treat the absence of
  `xml:space` on `<w:t>` as permission to discard leading and trailing spaces.

- **Preserved mode** (`xml:space="preserve"`): The element's whitespace must
  be preserved exactly as written. No trimming is permitted.

**The OOXML rule for producers:** A producer MUST set `xml:space="preserve"` on
`<w:t>` whenever the text content:
- begins with one or more space characters (U+0020), or
- ends with one or more space characters, or
- consists entirely of space characters (the "space-only run").

**The parser rule for consumers:** If `xml:space` is absent from `<w:t>`,
a conforming parser MUST NOT preserve leading or trailing whitespace on the
text value. This is not a Word-specific rule — it is what the XML specification
requires all XML processors to do.

**Correct examples:**

```xml
<!-- Text with leading space: MUST have xml:space="preserve" -->
<w:t xml:space="preserve"> Hello</w:t>

<!-- Text with trailing space: MUST have xml:space="preserve" -->
<w:t xml:space="preserve">Hello </w:t>

<!-- Space-only run: MUST have xml:space="preserve" -->
<w:t xml:space="preserve"> </w:t>

<!-- Text with internal spaces only: xml:space NOT required -->
<w:t>Hello world</w:t>

<!-- Text with no spaces at all: xml:space NOT required -->
<w:t>Hello</w:t>
```

**Incorrect example (space will be lost on round-trip):**

```xml
<!-- WRONG: leading space will be trimmed by XML processors -->
<w:t> Hello</w:t>
```

### 6.3 Entity Encoding in `<w:t>`

The text content is XML CDATA. The XML specification requires entity-encoding of:
- `<` → `&lt;`
- `>` → `&gt;` (required in some contexts)
- `&` → `&amp;`
- the four XML-illegal control characters (U+0000–U+0008, U+000B, U+000C,
  U+000E–U+001F) must be excluded or represented as numeric character references

Normal printable Unicode characters do NOT need entity encoding.

### 6.4 `<w:instrText>` and `<w:delText>`

`<w:instrText>` and `<w:delText>` share the same `CT_Text` type and therefore
have the same `xml:space` rule. A round-trip parser MUST preserve `xml:space`
on all three elements.

---

## 7. `<w:br>` — Break

Schema type `CT_Br`:

```xsd
<xsd:complexType name="CT_Br">
  <xsd:attribute name="type"  type="ST_BrType"  use="optional"/>
  <xsd:attribute name="clear" type="ST_BrClear" use="optional"/>
</xsd:complexType>
```

### 7.1 `w:type` Attribute (ST_BrType)

| Value | Meaning | Default? |
|---|---|---|
| `textWrapping` | Soft line break (continue text on next line within same paragraph) | Yes — this is the default when `w:type` is absent |
| `page` | Page break — content after this point appears at the top of the next page |
| `column` | Column break — content moves to the top of the next column |

When `w:type` is absent, the break is a soft return (`textWrapping`).

### 7.2 `w:clear` Attribute (ST_BrClear)

| Value | Meaning |
|---|---|
| `none` | No clearing — text simply wraps to next line (default) |
| `left` | Move down until the left margin is clear of floating objects |
| `right` | Move down until the right margin is clear |
| `all` | Move down until both margins are clear |

`w:clear` is only meaningful for `textWrapping` breaks in documents that have
text flowing around floating objects. For `page` and `column` breaks, `w:clear`
is accepted by the schema but has no practical effect.

### 7.3 Examples

```xml
<!-- Soft return (line break within paragraph) — w:type may be omitted -->
<w:r>
  <w:br/>
</w:r>

<!-- Explicit soft return (same effect) -->
<w:r>
  <w:br w:type="textWrapping"/>
</w:r>

<!-- Page break -->
<w:r>
  <w:br w:type="page"/>
</w:r>

<!-- Column break -->
<w:r>
  <w:br w:type="column"/>
</w:r>
```

**Note on page breaks:** Word also inserts page breaks as a paragraph with
`<w:pPr><w:pageBreakBefore/>` (breaks before the next paragraph) or via field
codes. The `<w:br w:type="page"/>` form is an in-text page break that falls
within a paragraph.

---

## 8. `<w:tab/>` — Tab Character

Schema type `CT_Empty`. The element is self-closing and has no attributes.

```xml
<w:r>
  <w:tab/>
</w:r>
```

A `<w:tab/>` element inserts a tab character at the current position. The
visual advance is to the next tab stop. Tab stops are resolved in priority
order:

1. Tab stops defined in `<w:pPr><w:tabs>` for the current paragraph.
2. Tab stops inherited from the applied paragraph style.
3. Default tab stops, set in `<w:defaultTab>` in `word/settings.xml` (default
   is 720 twips = 0.5 inch if not specified).

`CT_Tabs` contains one or more `CT_TabStop` elements:

```xml
<w:tabs>
  <w:tab w:val="center" w:pos="4320" w:leader="dot"/>
  <w:tab w:val="right"  w:pos="8640"/>
</w:tabs>
```

`CT_TabStop` attributes:
- `w:val` (`ST_TabJc`): alignment at the tab stop — `left`, `center`, `right`,
  `decimal`, `bar`, `clear`, `start`, `end`, `num`
- `w:pos` (`ST_SignedTwipsMeasure`): position from the paragraph's left indent,
  in twips
- `w:leader` (`ST_TabTlc`): fill character before the tab stop — `none`, `dot`,
  `hyphen`, `underscore`, `heavy`, `middleDot`

`w:val="clear"` removes a tab stop inherited from a style.

---

## 9. `<w:rPr>` — Run Properties

Schema type `CT_RPr`. This element carries all character formatting for a run.
It MUST appear as the first child of `<w:r>` if present.

### 9.1 CT_RPr Schema

```xsd
<xsd:complexType name="CT_RPr">
  <xsd:sequence>
    <xsd:group ref="EG_RPrContent" minOccurs="0"/>
  </xsd:sequence>
</xsd:complexType>

<xsd:group name="EG_RPrContent">
  <xsd:sequence>
    <xsd:group ref="EG_RPrBase" minOccurs="0" maxOccurs="unbounded"/>
    <xsd:element name="rPrChange" type="CT_RPrChange" minOccurs="0"/>
  </xsd:sequence>
</xsd:group>
```

`EG_RPrBase` is a choice group — each occurrence of `EG_RPrBase` contributes
one run property element. The `maxOccurs="unbounded"` on the group means that
multiple distinct run property elements may appear, but each element may appear
at most once (because they are distinct choices in the group).

**Important:** The schema does NOT impose a fixed order on the `EG_RPrBase`
members — they form an `xsd:choice` group repeated unbounded times. However,
Microsoft Office consistently emits them in a predictable order (matching the
list below), and most validators expect this order. A conforming producer
SHOULD emit `EG_RPrBase` members in the order they are listed in the XSD
choice group.

### 9.2 Run Property Elements Reference

The `EG_RPrBase` choice group contains the following elements, listed in the
order they appear in the XSD:

#### 9.2.1 `<w:rStyle>` — Character Style

```xml
<w:rStyle w:val="Emphasis"/>
```

Applies a named character style. `w:val` is the style ID (not the display
name). Character styles inherit from each other and from the default run
properties. When present, this is the first element in `<w:rPr>`.

#### 9.2.2 `<w:rFonts>` — Run Fonts

Schema type `CT_Fonts`. Specifies the font family for up to four text scripts:

```xml
<w:rFonts w:ascii="Calibri" w:hAnsi="Calibri" w:eastAsia="SimSun" w:cs="Arial"/>
```

| Attribute | Description |
|---|---|
| `w:ascii` | Font for ASCII characters (U+0000–U+007F) |
| `w:hAnsi` | Font for High-ANSI characters (U+0080–U+00FF and above, non-East-Asian) |
| `w:eastAsia` | Font for East Asian characters (CJK, katakana, hiragana, etc.) |
| `w:cs` | Font for complex-script characters (Arabic, Hebrew, Indic, etc.) |
| `w:hint` | Script hint for ambiguous characters: `default` or `eastAsia` |
| `w:asciiTheme` | Theme font reference for ASCII: one of `majorAscii`, `minorAscii`, `majorHAnsi`, `minorHAnsi`, `majorEastAsia`, `minorEastAsia`, `majorBidi`, `minorBidi` |
| `w:hAnsiTheme` | Theme font reference for High-ANSI |
| `w:eastAsiaTheme` | Theme font reference for East Asian |
| `w:cstheme` | Theme font reference for complex script |

When a theme font attribute is present, it takes precedence over the explicit
font name attribute for the same script. `w:hint` is used when a single
character could be drawn from either the default or the East Asian font;
it biases the selection.

#### 9.2.3 `<w:b/>` and `<w:bCs/>` — Bold

```xml
<w:b/>        <!-- bold for Latin and High-ANSI text -->
<w:bCs/>      <!-- bold for complex-script text (Arabic, Hebrew, etc.) -->
```

Both are type `CT_OnOff`. See §10 for the toggle property rule.

#### 9.2.4 `<w:i/>` and `<w:iCs/>` — Italic

```xml
<w:i/>         <!-- italic for Latin / High-ANSI text -->
<w:iCs/>       <!-- italic for complex-script text -->
```

Both are type `CT_OnOff`. Follow the same toggle rule as bold.

#### 9.2.5 `<w:caps/>` and `<w:smallCaps/>` — Capitalisation

```xml
<w:caps/>       <!-- ALL CAPS (visually, does not change stored text) -->
<w:smallCaps/>  <!-- Small Caps (uppercase at reduced size for lowercase letters) -->
```

Both are `CT_OnOff` toggle properties. `caps` and `smallCaps` are mutually
exclusive in practice (Word UI will not set both); the schema permits both,
but the effective behaviour is undefined when both are active.

#### 9.2.6 `<w:strike/>` and `<w:dstrike/>` — Strikethrough

```xml
<w:strike/>    <!-- single strikethrough -->
<w:dstrike/>   <!-- double strikethrough -->
```

Both are `CT_OnOff` toggle properties.

#### 9.2.7 `<w:outline/>` and `<w:shadow/>` — Outline and Shadow

```xml
<w:outline/>   <!-- draw character outline only, hollow interior -->
<w:shadow/>    <!-- add shadow behind character -->
```

Both are `CT_OnOff` toggle properties.

#### 9.2.8 `<w:emboss/>` and `<w:imprint/>` — Emboss and Engrave

```xml
<w:emboss/>    <!-- raised 3D effect -->
<w:imprint/>   <!-- pressed/engraved 3D effect -->
```

Both are `CT_OnOff` toggle properties. Mutually exclusive in practice.

#### 9.2.9 `<w:noProof/>` — Suppress Proofing

```xml
<w:noProof/>
```

`CT_OnOff`. Excludes this run from spell-checking and grammar-checking.

#### 9.2.10 `<w:snapToGrid/>` — Snap to Grid

```xml
<w:snapToGrid/>
```

`CT_OnOff`. Whether this run snaps to the document grid. Usually omitted
(inherits from document defaults).

#### 9.2.11 `<w:vanish/>` — Hidden Text

```xml
<w:vanish/>
```

`CT_OnOff` toggle. Hidden text: not printed and not displayed (unless the
application shows hidden text). The characters still occupy logical positions
in the document model.

#### 9.2.12 `<w:webHidden/>` — Web Hidden

```xml
<w:webHidden/>
```

`CT_OnOff` toggle. Hidden only when viewed in web mode. The text is visible
in print and normal view.

#### 9.2.13 `<w:color>` — Text Colour

```xml
<w:color w:val="FF0000"/>         <!-- bright red -->
<w:color w:val="auto"/>           <!-- automatic (inherits from context) -->
<w:color w:val="4472C4"
         w:themeColor="accent1"
         w:themeTint="FF"/>
```

Schema type `CT_Color`:

| Attribute | Description |
|---|---|
| `w:val` | Hex RGB (six uppercase hex digits) or `auto`. Required. |
| `w:themeColor` | Theme colour slot name (see `ST_ThemeColor` below) |
| `w:themeTint` | Single-byte hex tint adjustment (00–FF) |
| `w:themeShade` | Single-byte hex shade adjustment (00–FF) |

`ST_ThemeColor` values: `dark1`, `light1`, `dark2`, `light2`, `accent1`–`accent6`,
`hyperlink`, `followedHyperlink`, `none`, `background1`, `text1`, `background2`,
`text2`.

When `w:themeColor` is present, the `w:val` attribute holds the RGB value
computed from the theme colour at the time of the last save (a cached resolved
value). A parser that applies themes should recompute it from the theme; a
round-trip parser should preserve the `w:val` verbatim.

#### 9.2.14 `<w:spacing>` — Character Spacing

```xml
<w:spacing w:val="20"/>   <!-- expand by 1pt (20 twips) -->
<w:spacing w:val="-10"/>  <!-- condense by 0.5pt (-10 twips) -->
```

Schema type `CT_SignedTwipsMeasure`. `w:val` is a signed integer in twips
(twentieths of a point). Positive values expand spacing between characters;
negative values condense it.

This is **character** spacing adjustment, not line spacing. It applies uniformly
between each pair of characters in the run.

#### 9.2.15 `<w:w>` — Horizontal Character Scaling

```xml
<w:w w:val="150"/>    <!-- 150% width scaling (stretch) -->
<w:w w:val="80"/>     <!-- 80% width scaling (condense) -->
```

Schema type `CT_TextScale`. `w:val` is an integer 0–600 representing
percentage of normal character width. The value 100 means no scaling.
Alternatively, the `ST_TextScalePercent` accepts the string format `"150%"`.

#### 9.2.16 `<w:kern>` — Kerning Threshold

```xml
<w:kern w:val="16"/>   <!-- enable kerning for fonts 8pt and larger (16 half-points) -->
```

Schema type `CT_HpsMeasure`. `w:val` is in half-points (`ST_HpsMeasure`).
Kerning is applied to characters when the font size exceeds this threshold.
Setting to `0` disables kerning. Common values: `16` (8pt) or `24` (12pt).

#### 9.2.17 `<w:position>` — Vertical Position Offset

```xml
<w:position w:val="12"/>    <!-- raise by 6pt (12 half-points) -->
<w:position w:val="-8"/>    <!-- lower by 4pt (-8 half-points) -->
```

Schema type `CT_SignedHpsMeasure`. `w:val` is a signed integer in half-points.
Positive values raise the baseline; negative values lower it. Used for manual
superscript/subscript positioning (as an alternative to `<w:vertAlign>`).

#### 9.2.18 `<w:sz>` and `<w:szCs>` — Font Size

```xml
<w:sz   w:val="24"/>   <!-- 12pt for Latin/High-ANSI text (24 half-points) -->
<w:szCs w:val="24"/>   <!-- 12pt for complex-script text -->
```

Both are schema type `CT_HpsMeasure`. `w:val` is in **half-points**:
- 12pt = `w:val="24"`
- 14pt = `w:val="28"`
- 18pt = `w:val="36"`
- 24pt = `w:val="48"`
- 36pt = `w:val="72"`

**Critical gotcha:** DrawingML `<a:rPr sz="...">` uses hundredths-of-a-point,
where `sz="2400"` is 24pt. WML `<w:sz w:val="...">` uses half-points, where
`w:val="48"` is 24pt. These systems are completely different. See §14.

#### 9.2.19 `<w:highlight>` — Highlight Colour

```xml
<w:highlight w:val="yellow"/>
<w:highlight w:val="none"/>    <!-- remove highlight -->
```

Schema type `CT_Highlight`. `w:val` is of type `ST_HighlightColor`:

`black`, `blue`, `cyan`, `green`, `magenta`, `red`, `yellow`, `white`,
`darkBlue`, `darkCyan`, `darkGreen`, `darkMagenta`, `darkRed`, `darkYellow`,
`darkGray`, `lightGray`, `none`

These are the 16 classic Windows highlight colours plus `none`. Unlike
`<w:color>`, `<w:highlight>` is a limited enumeration — there is no RGB form.

#### 9.2.20 `<w:u>` — Underline

```xml
<w:u w:val="single"/>
<w:u w:val="double" w:color="FF0000"/>
<w:u w:val="none"/>       <!-- explicitly removes underline -->
```

Schema type `CT_Underline`:

| Attribute | Description |
|---|---|
| `w:val` | Underline style (optional — see `ST_Underline` values below) |
| `w:color` | Underline colour as hex RGB or `auto` |
| `w:themeColor` | Theme colour slot |
| `w:themeTint` | Tint adjustment |
| `w:themeShade` | Shade adjustment |

`ST_Underline` values:
`single`, `words`, `double`, `thick`, `dotted`, `dottedHeavy`, `dash`,
`dashedHeavy`, `dashLong`, `dashLongHeavy`, `dotDash`, `dashDotHeavy`,
`dotDotDash`, `dashDotDotHeavy`, `wave`, `wavyHeavy`, `wavyDouble`, `none`

Note the naming: the XSD uses `dashedHeavy` (not `dashHeavy`), `dashDotHeavy`
(not `dashDotHeavy`), and `wavyDouble` (not `waveDouble`). Consult the XSD
when in doubt about exact enum token spellings.

`w:val="words"` underlines only actual word characters, not the spaces between
words.

#### 9.2.21 `<w:effect>` — Animated Text Effect (Legacy)

```xml
<w:effect w:val="blinkBackground"/>
```

Schema type `CT_TextEffect`. `ST_TextEffect` values: `blinkBackground`,
`lights`, `antsBlack`, `antsRed`, `shimmer`, `sparkle`, `none`. These are
legacy animated effects from older versions of Word. They MUST be preserved
on round-trip but are not rendered by modern implementations.

#### 9.2.22 `<w:bdr>` — Run Border

```xml
<w:bdr w:val="single" w:sz="4" w:space="1" w:color="000000"/>
```

Schema type `CT_Border`. Draws a border around the run. Key attributes:

| Attribute | Description |
|---|---|
| `w:val` | Border style (see `ST_Border`); common: `single`, `double`, `none`, `nil` |
| `w:color` | Border colour as hex RGB or `auto` |
| `w:sz` | Line thickness in eighths of a point (`ST_EighthPointMeasure`) |
| `w:space` | Space between text and border in points (`ST_PointMeasure`) |
| `w:shadow` | Boolean — draw shadow on border |
| `w:frame` | Boolean — 3D frame border |

`w:val="nil"` means no border regardless of inherited value (stronger than
`none`).

#### 9.2.23 `<w:shd>` — Run Shading

```xml
<w:shd w:val="solid" w:color="auto" w:fill="FFFF00"/>
```

Schema type `CT_Shd`. Applies background shading to the run's character
bounding box. Key attributes:

| Attribute | Description |
|---|---|
| `w:val` | Shading pattern (see `ST_Shd`): `nil`, `clear`, `solid`, `horzStripe`, `diagCross`, `pct5`–`pct95`, etc. |
| `w:color` | Foreground/pattern colour as hex RGB or `auto` |
| `w:fill` | Background fill colour as hex RGB or `auto` |
| `w:themeColor`, `w:themeTint`, `w:themeShade` | Theme overrides for `color` |
| `w:themeFill`, `w:themeFillTint`, `w:themeFillShade` | Theme overrides for `fill` |

For a solid highlight-style fill, use `w:val="clear"` with `w:fill="RRGGBB"`.
`w:val="nil"` removes shading regardless of inherited value.

#### 9.2.24 `<w:fitText>` — Fit Text to Width

```xml
<w:fitText w:val="1440" w:id="1"/>
```

Schema type `CT_FitText`. Scales the text to fit into the specified width
in twips (`w:val`, type `ST_TwipsMeasure`). `w:id` groups related `fitText`
runs that must scale together (optional integer).

#### 9.2.25 `<w:vertAlign>` — Vertical Alignment / Super- and Subscript

```xml
<w:vertAlign w:val="superscript"/>
<w:vertAlign w:val="subscript"/>
<w:vertAlign w:val="baseline"/>   <!-- explicitly cancels super/subscript -->
```

Schema type `CT_VerticalAlignRun`. `ST_VerticalAlignRun` values (from
`shared-commonSimpleTypes.xsd`): `baseline`, `superscript`, `subscript`.

This is the canonical way to mark superscript/subscript in WML. The run
size and position are automatically adjusted by the rendering engine.
Using `<w:position>` with a reduced `<w:sz>` is an alternative but less
portable approach.

#### 9.2.26 `<w:rtl/>` — Right-to-Left Character Direction

```xml
<w:rtl/>
```

`CT_OnOff`. Forces this run's characters to be laid out right-to-left,
regardless of Unicode bidi algorithm. Used for explicitly marking bidi
direction of mixed-direction text.

#### 9.2.27 `<w:cs/>` — Complex Script

```xml
<w:cs/>
```

`CT_OnOff`. Marks this run as complex-script (Hebrew, Arabic, Indic, etc.).
When active, the `<w:szCs>`, `<w:bCs>`, `<w:iCs>`, and `<w:rFonts w:cs="..."/>`
properties are applied instead of their Latin equivalents.

#### 9.2.28 `<w:em>` — Emphasis Mark (East Asian)

```xml
<w:em w:val="dot"/>
<w:em w:val="none"/>   <!-- remove emphasis mark -->
```

Schema type `CT_Em`. `ST_Em` values: `none`, `dot`, `comma`, `circle`,
`underDot`. Emphasis marks are small characters drawn above (or below, in
some locales) each character, used in Chinese and Japanese typography to draw
attention to a word.

#### 9.2.29 `<w:lang>` — Language

```xml
<w:lang w:val="en-US" w:eastAsia="zh-CN" w:bidi="ar-SA"/>
```

Schema type `CT_Language`. All attributes are optional BCP-47 language tags:

| Attribute | Description |
|---|---|
| `w:val` | Language for Latin/High-ANSI characters |
| `w:eastAsia` | Language for East Asian characters |
| `w:bidi` | Language for complex-script (bidi) characters |

The language tag selects the spell-checking dictionary and hyphenation rules.
It does NOT control text rendering direction (that is `<w:bidi>` in `<w:pPr>`
and `<w:rtl>` in `<w:rPr>`).

#### 9.2.30 `<w:eastAsianLayout>` — East Asian Layout

```xml
<w:eastAsianLayout w:id="1" w:combine="1" w:combineBrackets="round"/>
```

Schema type `CT_EastAsianLayout`. Controls East Asian typography features such
as two-in-one (combining two characters into one character space):

| Attribute | Description |
|---|---|
| `w:id` | Identifier for grouping (integer) |
| `w:combine` | Enable two-in-one character combination |
| `w:combineBrackets` | Brackets for combined characters: `none`, `round`, `square`, `angle`, `curly` |
| `w:vert` | Enable vertical Latin character rotation in vertical text |
| `w:vertCompress` | Compress character height when using vertical rotation |

#### 9.2.31 `<w:specVanish/>` — Special Vanish

```xml
<w:specVanish/>
```

`CT_OnOff`. A specialised hidden-text property used primarily on paragraph
marks in list numbering contexts. Unlike `<w:vanish/>`, `<w:specVanish/>` causes
the paragraph mark to be hidden while still contributing to page layout
calculations. Parsers MUST preserve it.

#### 9.2.32 `<w:oMath/>` — Office Math

```xml
<w:oMath/>
```

`CT_OnOff`. Marks this run as part of an Office Math region. Used internally
by Word to track math-mode character formatting. Parsers MUST preserve it.

### 9.3 `<w:rPrChange>` — Tracked Run Property Change

```xml
<w:rPrChange w:id="3" w:author="Bob" w:date="2026-04-15T09:30:00Z">
  <w:rPr>
    <!-- run properties before the tracked change -->
    <w:b/>
    <w:sz w:val="28"/>
  </w:rPr>
</w:rPrChange>
```

Schema type `CT_RPrChange` extends `CT_TrackChange` with a required `<w:rPr>`
child (type `CT_RPrOriginal`). `CT_RPrOriginal` contains `EG_RPrBase` members
but not `rPrChange` itself (no nested tracking). The `<w:rPr>` child holds the
original (pre-change) run properties.

`CT_TrackChange` attributes: `w:id` (required integer), `w:author` (required
string), `w:date` (optional `xsd:dateTime`).

---

## 10. Toggle Properties

Several run properties in `EG_RPrBase` follow the "toggle" inheritance model.
The toggle properties are: `<w:b>`, `<w:i>`, `<w:strike>`, `<w:caps>`,
`<w:smallCaps>`, `<w:emboss>`, `<w:imprint>`, `<w:outline>`, `<w:shadow>`,
`<w:vanish>`, `<w:webHidden>`.

### 10.1 How Toggle Properties Work

In a traditional property inheritance model, a property in a style hierarchy
always overrides the same property at a less specific level. Toggle properties
are different: a toggle property in a character style does not unconditionally
set the property — instead, it *inverts* the property's current inherited value.

The rule:

> If a document style and a direct run property both specify the same toggle
> property (e.g. both set `<w:b/>`), the effective value is **off** — the two
> toggles cancel each other out. If only one specifies the property, the
> effective value is **on**.

More precisely:

1. Start with the document default value for the property (usually `off`).
2. For each style in the hierarchy (paragraph style, character style, run
   direct formatting), in order from most general to most specific:
   - If the style or formatting level contains `<w:b/>` (or equivalent with
     `w:val` absent or `true`), flip the current value.
   - If it contains `<w:b w:val="false"/>` (explicit disable), set the value
     to `off` absolutely (this is NOT a toggle — it is an explicit clear).

### 10.2 Explicit vs. Toggle

The key distinction:

| Markup | Behaviour |
|---|---|
| `<w:b/>` | Toggle: invert the inherited bold value |
| `<w:b w:val="true"/>` or `<w:b w:val="1"/>` or `<w:b w:val="on"/>` | Same as `<w:b/>` — toggle on |
| `<w:b w:val="false"/>` or `<w:b w:val="0"/>` or `<w:b w:val="off"/>` | Explicit disable: force bold to off, regardless of inherited value |

**The most common parser mistake:** Treating `<w:b/>` as an absolute "set bold
to true" rather than a toggle. This is correct when the run has no style
inheritance, but will produce wrong results when the run's character style
also sets bold.

### 10.3 Impact on Parser Design

A parser that:
- Only tracks whether `<w:b/>` is present/absent without applying the toggle
  rule will produce correct results for direct formatting with no style
  inheritance.
- Needs to resolve the effective bold state for rendering must implement the
  full toggle-inheritance chain: document defaults → named styles (normal →
  paragraph style → character style) → direct formatting.

For `ooxmlsdk` as a round-trip structural parser (not a renderer), the
toggle rule does not need to be implemented. The parser MUST however preserve
the `w:val` attribute exactly as given — removing a `w:val="false"` attribute
changes the semantics from "explicit disable" to "toggle on".

---

## 11. `w:val` Attribute Convention for CT_OnOff Properties

Properties that use `CT_OnOff` (bold, italic, vanish, etc.) have an optional
`w:val` attribute of type `s:ST_OnOff`.

The `ST_OnOff` type is defined in `shared-commonSimpleTypes.xsd` as a union
of `xsd:boolean` and `ST_OnOff1`:

```xsd
<xsd:simpleType name="ST_OnOff">
  <xsd:union memberTypes="xsd:boolean ST_OnOff1"/>
</xsd:simpleType>
<xsd:simpleType name="ST_OnOff1">
  <xsd:restriction base="xsd:string">
    <xsd:enumeration value="on"/>
    <xsd:enumeration value="off"/>
  </xsd:restriction>
</xsd:simpleType>
```

`xsd:boolean` accepts: `true`, `false`, `1`, `0`.

**Complete set of valid `w:val` values:**

| `w:val` value | Property state |
|---|---|
| Absent | Active (equivalent to `true`) |
| `true` | Active |
| `1` | Active |
| `on` | Active |
| `false` | Explicitly disabled |
| `0` | Explicitly disabled |
| `off` | Explicitly disabled |

**Producer guidance:** Emit `<w:b/>` (no `w:val`) to activate the property.
Emit `<w:b w:val="false"/>` or `<w:b w:val="0"/>` to explicitly disable.
Do not emit `<w:b w:val="true"/>` — it is redundant but valid.

**Parser guidance:** A parser MUST handle all seven forms. Mapping `"1"` → true,
`"true"` → true, `"on"` → true, and absent → true is required for correctness.

---

## 12. `w:rsid*` Attributes — Revision Save IDs

### 12.1 What rsid Is

Revision Save IDs (`rsid*`) are session-level edit identifiers assigned by
Microsoft Word. Each editing session generates a random 4-byte hex integer
(hence the `ST_LongHexNumber` type: `<xsd:length value="4"/>`). Word writes
this identifier onto every paragraph, run, and property block that is created
or modified during that session.

### 12.2 Which Elements Carry rsid Attributes

| Element | rsid attributes |
|---|---|
| `<w:p>` | `w:rsidR`, `w:rsidRPr`, `w:rsidDel`, `w:rsidP`, `w:rsidRDefault` |
| `<w:r>` | `w:rsidR`, `w:rsidRPr`, `w:rsidDel` |
| `<w:rPr>` | (none — rsid on the parent `<w:r>`) |
| `<w:pPr>` | (none — rsid on the parent `<w:p>`) |
| `<w:tbl>` | `w:rsidR`, `w:rsidTr`, `w:rsidDel` |
| `<w:tr>` | `w:rsidR`, `w:rsidTrPr`, `w:rsidDel` |

### 12.3 Semantics

`rsid*` values identify which editing session created or last modified each
structural unit. They enable Word to:
- Track which sentences were typed together in the same session.
- Merge documents from multiple authors (rsid helps resolve merge conflicts).
- Provide intelligent "smart select" (selecting text from the same session).

**For a parser:** rsid values are opaque. They MUST be preserved verbatim
on round-trip. A parser MUST NOT validate, interpret, or rewrite them. A
parser that drops rsid attributes will produce a document that opens correctly
in Word but has lost revision-session metadata. This is visible as corrupted
behaviour in Word's "select similar formatting" and as incorrect merge
behaviour when combining documents.

### 12.4 Example

```xml
<w:p w:rsidR="00C41A2B" w:rsidRDefault="00C41A2B" w:rsidRPr="00F83D1E">
  <w:r w:rsidR="00C41A2B">
    <w:t>First sentence, first session.</w:t>
  </w:r>
  <w:r w:rsidR="00F83D1E">
    <w:t xml:space="preserve"> Added later in a different session.</w:t>
  </w:r>
</w:p>
```

The two runs have different `w:rsidR` values, indicating they were typed in
different editing sessions.

---

## 13. Whitespace Rules

### 13.1 XML Whitespace Specification

The XML 1.0 specification says: the `xml:space` attribute with the value
`preserve` signals to an application that whitespace in the element's content
must be preserved. The value `default` (or absence of the attribute) allows
applications to decide whether to preserve whitespace.

OOXML processors (Word, LibreOffice, online editors) treat absence of
`xml:space` on `<w:t>` as authorisation to strip leading and trailing
whitespace from the text node.

### 13.2 Producer Rules: When MUST `xml:space="preserve"` Be Set?

A producer MUST add `xml:space="preserve"` to `<w:t>` in all of these cases:

1. The text starts with a space: `" Hello"`, `"  Hello"`.
2. The text ends with a space: `"Hello "`, `"Hello  "`.
3. The text is entirely whitespace (space characters): `" "`, `"   "`.
4. The text has no other characters and would become empty after stripping.

**Mnemonic:** If stripping leading and trailing spaces from the text would
change what the document displays, `xml:space="preserve"` is required.

### 13.3 Consumer Rules: What to Do When `xml:space` Is Absent

When `<w:t>` has no `xml:space` attribute, a conforming consumer MUST apply
XML whitespace stripping: trim leading and trailing space characters. Internal
whitespace (between non-space characters) is preserved in both modes.

### 13.4 Common Whitespace Patterns

**Sentence with inter-run space:**

A single space between two words often appears as its own run or must be
preserved in an adjacent run:

```xml
<!-- Pattern 1: space in its own run -->
<w:r><w:t xml:space="preserve"> </w:t></w:r>

<!-- Pattern 2: space at end of first run -->
<w:r>
  <w:rPr><w:b/></w:rPr>
  <w:t xml:space="preserve">Bold word </w:t>
</w:r>
<w:r>
  <w:t>normal word</w:t>
</w:r>

<!-- Pattern 3: space at start of second run -->
<w:r>
  <w:rPr><w:b/></w:rPr>
  <w:t>Bold word</w:t>
</w:r>
<w:r>
  <w:t xml:space="preserve"> normal word</w:t>
</w:r>
```

All three patterns are valid. The space-only run (`Pattern 1`) is the most
common in documents produced by Word when the space character has its own
distinct formatting.

**Incorrect — space will be lost:**

```xml
<!-- WRONG: leading space stripped because xml:space absent -->
<w:r><w:t> </w:t></w:r>

<!-- WRONG: trailing space stripped -->
<w:r><w:t>Hello </w:t></w:r>
```

### 13.5 `xml:space` on `<w:instrText>` and `<w:delText>`

The same rules apply to `<w:instrText>` and `<w:delText>`, which share the
`CT_Text` type. Field instruction text commonly contains significant leading
spaces (e.g., `" HYPERLINK \"url\""`) and MUST carry `xml:space="preserve"`.

---

## 14. Common Failure Modes for Parsers

This section documents the most frequent correctness failures in OOXML parsers
that handle WML paragraphs and runs.

### 14.1 Stripping `xml:space="preserve"` on `<w:t>`

**Failure:** The parser reads `<w:t xml:space="preserve"> </w:t>` but writes
`<w:t> </w:t>` (dropping the attribute). On re-read, the space is stripped.

**Impact:** Silent data loss. Single spaces between formatted runs disappear.
Spaces adjacent to tab characters, field codes, or line breaks vanish. The
rendered document appears to have words merged together.

**Fix:** Preserve the `xml:space` attribute exactly as read. When constructing
new `<w:t>` elements, set `xml:space="preserve"` whenever the rule in §13.2
is triggered.

### 14.2 Reordering `<w:pPr>` Children

**Failure:** The parser rebuilds `<w:pPr>` content in a different order than
the XSD sequence — for example, placing `<w:jc>` before `<w:spacing>`.

**Impact:** The saved document fails schema validation. More practically,
some strict validators (Word in strict mode, OOXML validators) will reject
the document or display validation warnings. Interoperability with tools
that consume the XSD strictly is broken.

**Fix:** Emit `CT_PPrBase` children in the exact order given by the XSD
`CT_PPrBase` sequence (see §4.2). The order is: `pStyle`, `keepNext`,
`keepLines`, `pageBreakBefore`, `framePr`, `widowControl`, `numPr`, ...,
`spacing`, `ind`, ..., `jc`, ..., `outlineLvl`, `divId`, `cnfStyle`.

### 14.3 Losing `rsid*` Attributes

**Failure:** The parser drops `w:rsidR`, `w:rsidRPr`, `w:rsidDel`, `w:rsidP`,
or `w:rsidRDefault` attributes when deserialising and re-serialising.

**Impact:** Documents round-trip silently but with altered revision-session
metadata. Word's edit-session-based selection, document comparison, and
merge features produce incorrect results.

**Fix:** Deserialise all `rsid*` attributes as opaque hex strings. Preserve
and re-emit them unchanged on serialisation.

### 14.4 Confusing Toggle `<w:b/>` with Explicit `<w:b w:val="true"/>`

**Failure:** A parser treats `<w:b/>` as "absolutely set bold" when generating
new markup, and omits `<w:b/>` from the output to mean "bold is off". This
is wrong because the absence of `<w:b/>` in a run does NOT mean bold is off —
it means bold is inherited from the style chain.

**Impact:** When a character style defines bold (via toggle), a run without
any explicit `<w:b/>` should be bold. A parser that inserts `<w:b w:val="false"/>`
to "turn off bold" will produce incorrect results for runs inside bold character
styles.

**Fix:** Understand that absence of `<w:b/>` means "inherit from style chain".
Use `<w:b w:val="false"/>` (explicit disable) only when intending to override
an inherited bold and force bold off. Never emit `<w:b/>` when the intent is
to keep whatever the style says.

### 14.5 WML `<w:sz>` Half-Points vs. DrawingML `<a:rPr sz>` Hundredths-of-a-Point

**Failure:** When handling `<w:sz w:val="...">`, applying the DrawingML
`<a:rPr sz="...">` unit interpretation (hundredths of a point) or vice versa.

**Impact:** Font sizes in generated or converted documents are wrong by a
factor of 50. A 12pt font becomes either 600pt or 0.24pt.

**Conversion table:**

| Point size | WML `<w:sz w:val="...">` | DrawingML `<a:rPr sz="...">` |
|---|---|---|
| 10 pt | 20 | 1000 |
| 11 pt | 22 | 1100 |
| 12 pt | 24 | 1200 |
| 14 pt | 28 | 1400 |
| 18 pt | 36 | 1800 |
| 24 pt | 48 | 2400 |
| 36 pt | 72 | 3600 |
| 48 pt | 96 | 4800 |

**Fix:** Keep WML and DrawingML size values strictly separate. Never mix them.

### 14.6 Space-Only `<w:t>` Without `xml:space="preserve"`

**Failure:** Generating `<w:t> </w:t>` for a space-only text node.

**Impact:** The single space is stripped on re-read by any conforming XML
parser. The document loses all spaces that were represented as single-space
runs.

**Fix:** Always emit `<w:t xml:space="preserve"> </w:t>` for space-only runs.

### 14.7 `<w:sectPr>` Not Last in `<w:body>`

**Failure:** The parser places `<w:sectPr>` at a position other than the last
child of `<w:body>`.

**Impact:** The document is schema-invalid. The `CT_Body` XSD declares
`<w:sectPr>` as a distinct terminal element after the `EG_BlockLevelElts`
group. Word may open the document but with incorrect section properties, or
may refuse to open it.

**Fix:** Always emit `<w:sectPr>` as the absolute last child of `<w:body>`,
after all `<w:p>`, `<w:tbl>`, and other block-level content.

### 14.8 `CT_RPr` `EG_RPrBase` Ordering

**Failure:** The parser emits `<w:rPr>` children in arbitrary order.

**Impact:** Technically, the `EG_RPrBase` choice group with `maxOccurs=
"unbounded"` does not impose a sequence order. However, real-world validators
and interoperability tools expect the elements in the order they appear in the
choice group. Emitting `<w:sz>` before `<w:b>` can trigger validation warnings.

**Fix:** Emit `EG_RPrBase` members in XSD declaration order: `rStyle`, `rFonts`,
`b`, `bCs`, `i`, `iCs`, `caps`, `smallCaps`, `strike`, `dstrike`, `outline`,
`shadow`, `emboss`, `imprint`, `noProof`, `snapToGrid`, `vanish`, `webHidden`,
`color`, `spacing`, `w`, `kern`, `position`, `sz`, `szCs`, `highlight`, `u`,
`effect`, `bdr`, `shd`, `fitText`, `vertAlign`, `rtl`, `cs`, `em`, `lang`,
`eastAsianLayout`, `specVanish`, `oMath`, then `rPrChange`.

---

## 15. Complete Structural Example

This example shows a paragraph with multiple runs demonstrating paragraph
properties, run properties, whitespace handling, and break elements:

```xml
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>

    <!-- Paragraph 1: bold heading, style applied -->
    <w:p w:rsidR="00AA0011" w:rsidRDefault="00AA0011">
      <w:pPr>
        <w:pStyle w:val="Heading1"/>
        <w:jc w:val="center"/>
      </w:pPr>
      <w:r w:rsidR="00AA0011">
        <w:t>Document Title</w:t>
      </w:r>
    </w:p>

    <!-- Paragraph 2: mixed formatting, space preservation -->
    <w:p w:rsidR="00BB0022" w:rsidRDefault="00BB0022">
      <w:pPr>
        <w:spacing w:before="120" w:after="120" w:line="276" w:lineRule="auto"/>
      </w:pPr>

      <!-- Bold run with trailing space preserved -->
      <w:r>
        <w:rPr>
          <w:b/>
          <w:sz w:val="24"/>
          <w:color w:val="1F4E79"/>
        </w:rPr>
        <w:t xml:space="preserve">Important: </w:t>
      </w:r>

      <!-- Normal run -->
      <w:r>
        <w:rPr>
          <w:sz w:val="24"/>
        </w:rPr>
        <w:t>This text has</w:t>
      </w:r>

      <!-- Space-only run between differently formatted runs -->
      <w:r>
        <w:t xml:space="preserve"> </w:t>
      </w:r>

      <!-- Underlined run -->
      <w:r>
        <w:rPr>
          <w:u w:val="single"/>
          <w:sz w:val="24"/>
        </w:rPr>
        <w:t xml:space="preserve">underlined content</w:t>
      </w:r>

      <!-- Run with soft line break -->
      <w:r>
        <w:t xml:space="preserve"> and then</w:t>
        <w:br/>
        <w:t>a soft line break.</w:t>
      </w:r>
    </w:p>

    <!-- Paragraph 3: page break run, tab, superscript -->
    <w:p w:rsidR="00CC0033" w:rsidRDefault="00CC0033">
      <!-- Page break as inline run -->
      <w:r>
        <w:br w:type="page"/>
      </w:r>

      <!-- Tab character advancing to next tab stop -->
      <w:r>
        <w:tab/>
      </w:r>

      <!-- Normal text -->
      <w:r>
        <w:t xml:space="preserve">Footnote marker</w:t>
      </w:r>

      <!-- Superscript run -->
      <w:r>
        <w:rPr>
          <w:vertAlign w:val="superscript"/>
        </w:rPr>
        <w:t>1</w:t>
      </w:r>
    </w:p>

    <!-- Section properties — MUST be last child of w:body -->
    <w:sectPr>
      <w:pgSz w:w="12240" w:h="15840"/>
      <w:pgMar w:top="1440" w:right="1440" w:bottom="1440" w:left="1440"
               w:header="720" w:footer="720" w:gutter="0"/>
    </w:sectPr>

  </w:body>
</w:document>
```

---

## 16. Fixture Plan

The following test fixtures should be created to verify correct round-trip
behaviour. Existing fixtures in `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/document/` cover minimal structure
(empty body, text, styles, table, image). The fixtures below target run-level
correctness.

### Fixture WML-R-01: Character Formatting

**File:** `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/char_formatting.docx`

**Content:** A single paragraph with multiple runs, each demonstrating a
different character formatting property:
- Bold (`<w:b/>`)
- Italic (`<w:i/>`)
- Bold italic combined
- Single underline (`<w:u w:val="single"/>`)
- Double underline (`<w:u w:val="double"/>`)
- Single strikethrough (`<w:strike/>`)
- Double strikethrough (`<w:dstrike/>`)
- Font size 14pt (`<w:sz w:val="28"/>`)
- Explicit colour (`<w:color w:val="C00000"/>`)
- Highlight yellow (`<w:highlight w:val="yellow"/>`)
- Superscript (`<w:vertAlign w:val="superscript"/>`)
- Subscript (`<w:vertAlign w:val="subscript"/>`)

**Round-trip assertions:**
1. Open without error.
2. Save without error.
3. Re-open from saved bytes without error.
4. The `<w:rPr>` elements in the saved document contain the same properties in
   the same order as the input.
5. `<w:b/>`, `<w:i/>`, etc. are preserved without gaining or losing `w:val` attributes.

### Fixture WML-R-02: Run Fonts

**File:** `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/run_fonts.docx`

**Content:** Runs exercising `<w:rFonts>`:
- `w:ascii="Arial" w:hAnsi="Arial"` (explicit font override)
- `w:eastAsia="SimSun"` (CJK font)
- `w:cs="Times New Roman"` (complex script font)
- `w:asciiTheme="minorAscii"` (theme font reference)
- `w:hint="eastAsia"` (hint for ambiguous characters)
- A run with `<w:rStyle w:val="Emphasis"/>` (character style reference)

**Round-trip assertions:**
1. All `<w:rFonts>` attributes preserved verbatim.
2. `w:hint` attribute preserved (parsers commonly lose optional attributes).
3. Theme font attributes (`w:asciiTheme`, etc.) preserved.

### Fixture WML-R-03: Whitespace Preservation

**File:** `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/whitespace.docx`

**Content:** Runs specifically testing `xml:space` behaviour:
- A run with text starting with a space: `<w:t xml:space="preserve"> leading`
- A run with text ending with a space: `<w:t xml:space="preserve">trailing `
- A space-only run: `<w:t xml:space="preserve"> </w:t>`
- A run with internal spaces only (no leading/trailing): `<w:t>word word</w:t>`
  — this MUST NOT gain `xml:space="preserve"`
- A run with no spaces at all: `<w:t>nospace</w:t>`
- Two adjacent formatted runs bridged by a space-only run

**Round-trip assertions:**
1. All `xml:space="preserve"` attributes are preserved.
2. Runs without `xml:space` do not gain the attribute.
3. The character count of all text content is identical before and after
   round-trip (extracting plain text via ZIP + xmllint and comparing).

### Fixture WML-R-04: Break Elements

**File:** `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/breaks.docx`

**Content:** Runs demonstrating break elements:
- Soft return: `<w:br/>` (no `w:type` attribute)
- Explicit soft return: `<w:br w:type="textWrapping"/>`
- Page break: `<w:br w:type="page"/>` (as its own paragraph or inline)
- Column break: `<w:br w:type="column"/>` (in a two-column section)
- Tab character: `<w:tab/>` between two words

**Round-trip assertions:**
1. Soft returns preserved: `<w:br/>` without `w:type` must remain without
   `w:type` (parsers must not normalise to `<w:br w:type="textWrapping"/>`).
2. Page break preserved.
3. Tab preserved.
4. `w:clear` attribute (if present) preserved.

---

*End of WordprocessingML Paragraph and Run Model Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
