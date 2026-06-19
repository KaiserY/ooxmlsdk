# WML Numbered and Bulleted Lists — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.9 (numbering), ISO/IEC
29500:2016 Part 1 §17.9; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd`.

---

## 1. Overview

WordprocessingML numbering has three layers:

1. **`word/numbering.xml`** — the numbering definitions part, holding all
   `abstractNum` and `num` definitions.
2. **`<w:pPr><w:numPr>`** in each paragraph — a reference from a paragraph to
   a specific numbering instance and level.
3. **`<w:styles>`** — paragraph styles may embed `numPr` so every paragraph
   using that style is automatically part of a list.

**Relationship type:**
`http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering`

**Content type:**
`application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml`

The numbering part is **optional**. If no numbered paragraphs exist, the part
and its relationship may be omitted entirely.

---

## 2. CT_Numbering — Root Element

```xml
<w:numbering xmlns:w="…">
  <w:numPicBullet …/>     <!-- 0+ picture bullet definitions  -->
  <w:abstractNum …/>      <!-- 0+ abstract numbering definitions -->
  <w:num …/>              <!-- 0+ concrete numbering instances -->
  <w:numIdMacAtCleanup …/> <!-- optional: highest numId seen at last cleanup -->
</w:numbering>
```

**Child element order is significant.** All `numPicBullet` elements precede all
`abstractNum` elements, which precede all `num` elements.

---

## 3. Abstract Numbering Definition: CT_AbstractNum

`<w:abstractNum>` defines a reusable numbering template with up to 9 levels
(0–8). Multiple `<w:num>` instances can reference the same abstract definition,
allowing independent restart counts while sharing the level format.

```xml
<w:abstractNum w:abstractNumId="0">
  <w:nsid w:val="00A12345"/>            <!-- unique ID for this definition -->
  <w:multiLevelType w:val="multilevel"/>
  <w:tmpl w:val="5A9012B4"/>            <!-- template code for Word UI -->
  <w:name w:val="my-list"/>             <!-- optional name -->
  <w:lvl w:ilvl="0">…</w:lvl>
  <w:lvl w:ilvl="1">…</w:lvl>
  …
</w:abstractNum>
```

### Attributes

| Attribute | Required | Notes |
|-----------|----------|-------|
| `abstractNumId` | **yes** | Non-negative integer; unique within the document |

### Child elements

| Element | Notes |
|---------|-------|
| `nsid` | 4-byte uppercase hex string — unique across saves; used for list continuation logic |
| `multiLevelType` | ST_MultiLevelType: `singleLevel`, `multilevel`, `hybridMultilevel` |
| `tmpl` | 4-byte uppercase hex — Word template code |
| `name` | Human-readable name (optional; used by the Styles pane) |
| `styleLink` | Paragraph style ID that this abstract num defines list formatting for |
| `numStyleLink` | Paragraph style ID that provides the list definition |
| `lvl` | Level definitions (0–8); at most 9 |

### ST_MultiLevelType values

| Value | Meaning |
|-------|---------|
| `singleLevel` | Only level 0 is used |
| `multilevel` | Multiple independent levels |
| `hybridMultilevel` | Hybrid list (combines single and multi-level behaviour) |

---

## 4. Level Definition: CT_Lvl

Each `<w:lvl>` defines how one indent level is numbered and formatted.

```xml
<w:lvl w:ilvl="0">
  <w:start w:val="1"/>
  <w:numFmt w:val="decimal"/>
  <w:lvlText w:val="%1."/>
  <w:lvlJc w:val="left"/>
  <w:pPr>
    <w:ind w:left="720" w:hanging="360"/>
  </w:pPr>
  <w:rPr>
    <w:rFonts w:ascii="Arial" w:hAnsi="Arial"/>
  </w:rPr>
</w:lvl>
```

### Attributes on `<w:lvl>`

| Attribute | Required | Notes |
|-----------|----------|-------|
| `ilvl` | **yes** | Level index 0–8 |
| `tplc` | no | 4-byte hex template code |
| `tentative` | no | CT_OnOff — declared but not currently in use by a list |

### Child elements

| # | Element | Notes |
|---|---------|-------|
| 1 | `start` | Starting count value (default 1) |
| 2 | `numFmt` | Number format (ST_NumberFormat) |
| 3 | `lvlRestart` | When to restart counter: 0 = never; 1 = when any higher level increments (default) |
| 4 | `pStyle` | If set, this level is used automatically for paragraphs with this style ID |
| 5 | `isLgl` | Display all levels using Arabic numerals (legal numbering) |
| 6 | `suff` | Content between number and paragraph text: `tab` (default), `space`, `nothing` |
| 7 | `lvlText` | Template for the number string (see §5) |
| 8 | `lvlPicBulletId` | Reference to a `numPicBullet` ID (for picture bullets) |
| 9 | `legacy` | Legacy numbering properties |
| 10 | `lvlJc` | Number justification: `left`, `right`, `center` |
| 11 | `pPr` | Paragraph properties applied to list paragraphs at this level (mainly indentation) |
| 12 | `rPr` | Run properties applied to the numbering label (font, size, colour) |

---

## 5. Level Text Format: `<w:lvlText>`

The `val` attribute is a format string. `%N` (where N is 1–9) is replaced by
the current counter value at level N−1. Any other characters are literal text.

```
%1.          → "1.", "2.", "3.", …           (decimal, level 0)
%1.%2        → "1.1", "1.2", "2.1", …       (two-level decimal)
(%1)         → "(1)", "(2)", …               (decimal with parens)
%1.%2.%3.    → "1.1.1.", "1.1.2.", …        (three-level)
•            → "•", "•", "•"                 (bullet; numFmt=bullet)
```

For bullet levels, `numFmt` must be `bullet` and `val` is the literal bullet
character (Unicode directly, or a character from a symbol font referenced in
`<w:rPr>`).

**Null-character bullets:** When `<w:lvlText w:null="1"/>` is present (val
absent or empty), the level produces no visible label. Used for continuation
paragraphs in a list.

---

## 6. Common ST_NumberFormat Values

| Value | Description | Example |
|-------|-------------|---------|
| `decimal` | 1, 2, 3, … | `%1.` → "1." |
| `upperRoman` | I, II, III, … | `%1.` → "I." |
| `lowerRoman` | i, ii, iii, … | `%1.` → "i." |
| `upperLetter` | A, B, C, … | `%1.` → "A." |
| `lowerLetter` | a, b, c, … | `%1.` → "a." |
| `ordinal` | 1st, 2nd, 3rd, … | |
| `bullet` | Fixed character (from `lvlText`) | `•` |
| `none` | No numbering label | |
| `decimalZero` | 01, 02, 03, … | |
| `cardinalText` | One, Two, Three, … | |
| `ordinalText` | First, Second, Third, … | |

---

## 7. Concrete Numbering Instance: CT_Num

`<w:num>` binds an `abstractNum` to a specific list instance in the document.
Each independent list (even if it has the same format) should use a separate
`num` with a unique `numId`. This allows independent counters.

```xml
<w:num w:numId="1">
  <w:abstractNumId w:val="0"/>        <!-- references abstractNum/@abstractNumId -->
  <w:lvlOverride w:ilvl="0">          <!-- optional: override level 0 -->
    <w:startOverride w:val="1"/>       <!-- restart at 1 -->
  </w:lvlOverride>
</w:num>
```

### Attributes

| Attribute | Required | Notes |
|-----------|----------|-------|
| `numId` | **yes** | Positive integer; `numId="0"` is reserved to mean "remove numbering" |

### CT_NumLvl (level override)

| Child | Notes |
|-------|-------|
| `startOverride` | Override the starting value for this level |
| `lvl` | Full level definition override (replaces the abstract level) |

**Important:** `numId="0"` in a paragraph's `<w:numPr>` does not reference any
`<w:num>` — it explicitly removes list formatting from that paragraph. Any
`<w:num>` element in the numbering part must have `numId` ≥ 1.

---

## 8. Paragraph Numbering Reference: CT_NumPr

`<w:numPr>` appears as a child of `<w:pPr>` (or `<w:pPr>` inside a style) to
attach a paragraph to a list.

```xml
<w:pPr>
  <w:numPr>
    <w:ilvl w:val="0"/>   <!-- level 0–8; omit = level 0 -->
    <w:numId w:val="1"/>  <!-- references <w:num w:numId="1"/> -->
  </w:numPr>
</w:pPr>
```

| Child | Notes |
|-------|-------|
| `ilvl` | Level index 0–8. Default = 0 if absent. |
| `numId` | References `<w:num>/@numId`. Value 0 = remove numbering. |
| `numberingChange` | Track-change wrapper for numbering property change |
| `ins` | Inserted numbering (track changes) |

**Interaction with styles:** If a paragraph style defines `<w:numPr>` in its
`<w:pPr>`, all paragraphs using that style are automatically numbered. Direct
`<w:numPr>` on the paragraph overrides the style's `<w:numPr>`.

---

## 9. Indentation for List Paragraphs

The canonical indentation pattern for list levels is set in each level's
`<w:pPr><w:ind>`:

```
Level 0: left=720  (0.5 in), hanging=360 (0.25 in)
Level 1: left=1440 (1.0 in), hanging=360
Level 2: left=2160 (1.5 in), hanging=360
…
Level N: left=(N+1)*720, hanging=360
```

The `hanging` value creates space for the number label; the text wraps at
`left`, and the label sits `hanging` twips to the left of that (at
`left − hanging`).

**Direct indentation overrides list indentation.** If a paragraph has both
`<w:numPr>` and a direct `<w:ind>`, the direct indent takes precedence.

---

## 10. Level Restart and Override

By default, each level restarts when a higher level (lower ilvl number)
increments. This is `<w:lvlRestart w:val="1"/>`. Restart behaviour:

| `lvlRestart` value | Meaning |
|--------------------|---------|
| `0` | Never restart; count continues across the entire document |
| `1` | Restart whenever a higher level increments (default) |
| `2` | Restart when a level 2 or higher increments (only for level ≥ 1) |

**Starting a new numbered list from 1:** Create a new `<w:num>` referencing
the same `abstractNumId` and add `<w:lvlOverride w:ilvl="0"><w:startOverride
w:val="1"/></w:lvlOverride>`. This gives an independent counter starting at 1
without duplicating the format definition.

---

## 11. Round-Trip Gotchas

1. **`nsid` and `tmpl` must be preserved.** These 4-byte hex strings are used
   by Word to identify list templates across saves. Generating new values breaks
   list continuation in existing documents.

2. **Level count: always 9, often trimmed.** Office files typically define all
   9 levels (ilvl 0–8) even for single-level lists, because Word writes them
   all. A conforming file may define fewer. The parser must not require exactly
   9 levels.

3. **`numId="0"` is not a list.** A paragraph with `<w:numId w:val="0"/>` has
   its list formatting explicitly removed. There is no `<w:num w:numId="0"/>`
   in the numbering part; the value is a sentinel.

4. **Abstract `<w:lvl>` vs override `<w:lvl>`.** When a `<w:num>` contains a
   `<w:lvlOverride>` that has its own `<w:lvl>`, that level definition fully
   replaces the abstract level for that instance — all fields, not just the
   ones specified.

5. **`suff="tab"` and the hanging indent.** The tab character produced by
   `suff="tab"` aligns to the `<w:ind left>` value. If there is no tab stop
   defined at that position, the tab snaps to the nearest default tab stop
   (usually every 720 twips).

6. **Bullet font must match the character.** A `•` (U+2022) renders in most
   body fonts. Characters from Wingdings/Symbol require the matching font to be
   set in the level's `<w:rPr><w:rFonts>`. Mismatched font + character is a
   common source of garbled bullets.

7. **`lvlText` `%N` is 1-indexed.** `%1` refers to level 0's counter, `%2` to
   level 1, etc. Producing `%0` is invalid.

8. **`abstractNum` IDs and `num` IDs are independent.** `abstractNumId` on
   `<w:abstractNum>` and `numId` on `<w:num>` are separate integer spaces.
   There is no requirement that `numId=1` references `abstractNumId=1`.

9. **`numIdMacAtCleanup`.** This optional element records the highest `numId`
   seen before the last "clean up" operation. Parsers should preserve it
   verbatim but need not enforce it.

10. **Numbering and styles co-operate.** The "List Bullet" and "List Number"
    built-in styles embed `<w:numPr>` in their `<w:pPr>`. When these styles are
    applied, the paragraph inherits the numId/ilvl from the style's pPr, not
    from direct formatting.

---

## 12. Minimal Structures

### Single-level bullet list
```xml
<!-- numbering.xml -->
<w:numbering>
  <w:abstractNum w:abstractNumId="0">
    <w:multiLevelType w:val="singleLevel"/>
    <w:lvl w:ilvl="0">
      <w:start w:val="1"/>
      <w:numFmt w:val="bullet"/>
      <w:lvlText w:val="•"/>
      <w:lvlJc w:val="left"/>
      <w:pPr><w:ind w:left="720" w:hanging="360"/></w:pPr>
      <w:rPr><w:rFonts w:ascii="Arial" w:hAnsi="Arial"/></w:rPr>
    </w:lvl>
  </w:abstractNum>
  <w:num w:numId="1">
    <w:abstractNumId w:val="0"/>
  </w:num>
</w:numbering>

<!-- In document.xml: -->
<w:p>
  <w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="1"/></w:numPr></w:pPr>
  <w:r><w:t>Bullet item</w:t></w:r>
</w:p>
```

### Two-level ordered list (restart second list)
```xml
<w:abstractNum w:abstractNumId="1">
  <w:multiLevelType w:val="multilevel"/>
  <w:lvl w:ilvl="0">
    <w:start w:val="1"/>
    <w:numFmt w:val="decimal"/>
    <w:lvlText w:val="%1."/>
    <w:lvlJc w:val="left"/>
    <w:pPr><w:ind w:left="720" w:hanging="360"/></w:pPr>
  </w:lvl>
  <w:lvl w:ilvl="1">
    <w:start w:val="1"/>
    <w:numFmt w:val="lowerLetter"/>
    <w:lvlText w:val="%2."/>
    <w:lvlJc w:val="left"/>
    <w:pPr><w:ind w:left="1440" w:hanging="360"/></w:pPr>
  </w:lvl>
</w:abstractNum>
<w:num w:numId="2">
  <w:abstractNumId w:val="1"/>
</w:num>
<!-- Second independent list: same format, restarts at 1 -->
<w:num w:numId="3">
  <w:abstractNumId w:val="1"/>
  <w:lvlOverride w:ilvl="0">
    <w:startOverride w:val="1"/>
  </w:lvlOverride>
</w:num>
```

---

## 13. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-N-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/numbering_bullets.docx` | Single-level bullet list; bullet numFmt; Unicode bullet char; hanging indent |
| WML-N-02 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/numbering_ordered.docx` | Multi-level list: decimal L0, lowerLetter L1, lowerRoman L2; multi-level lvlText |
| WML-N-03 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/numbering_restart.docx` | Two independent ordered lists; second restarts via lvlOverride startOverride |
