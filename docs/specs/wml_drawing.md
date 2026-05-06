# WML Drawing and Images — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §20.4 (wordprocessingDrawing),
§20.2 (picture), §20.1 (DrawingML main); ISO/IEC 29500:2016 Part 1 §20.4, §20.2;
XSD files `dml-wordprocessingDrawing.xsd`, `dml-picture.xsd`, `dml-main.xsd`.

---

## 1. Overview

Images and other graphics in a DOCX are wrapped in `<w:drawing>`, which appears
as a child of `<w:r>` (a run):

```
<w:p>
  <w:r>
    <w:drawing>
      <wp:inline …>  or  <wp:anchor …>
        …
      </wp:inline>
    </w:drawing>
  </w:r>
</w:p>
```

Two placement modes:

| Mode | Element | Behaviour |
|------|---------|-----------|
| Inline | `<wp:inline>` | Image sits in the text flow like a large character |
| Floating | `<wp:anchor>` | Image is positioned absolutely/relatively; text wraps around it |

**Namespaces:**

| Prefix | URI |
|--------|-----|
| `wp` | `http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing` |
| `a` | `http://schemas.openxmlformats.org/drawingml/2006/main` |
| `pic` | `http://schemas.openxmlformats.org/drawingml/2006/picture` |
| `r` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships` |

All namespace declarations must appear on `<w:document>` (or on the element
itself). Omitting a namespace declaration causes parse failure.

---

## 2. EMU — English Metric Units

All dimensions and positions use **EMU** (English Metric Units), a 64-bit
signed integer.

| Real unit | EMU |
|-----------|-----|
| 1 inch | 914,400 |
| 1 centimetre | 360,000 |
| 1 millimetre | 36,000 |
| 1 point (1/72 in) | 12,700 |
| 1 pixel at 96 dpi | 9,525 |
| 1 pixel at 72 dpi | 12,700 |

**Common image sizes:**

| Size | EMU width × height |
|------|--------------------|
| 1 in × 1 in | 914400 × 914400 |
| 2 in × 1.5 in | 1828800 × 1371600 |
| 4 in × 3 in | 3657600 × 2743200 |

---

## 3. Inline Drawing: CT_Inline

`<wp:inline>` positions the image as if it were a text character.

```xml
<wp:inline distT="0" distB="0" distL="114300" distR="114300">
  <wp:extent cx="914400" cy="914400"/>
  <wp:effectExtent l="0" t="0" r="0" b="0"/>
  <wp:docPr id="1" name="Image 1" descr="Alt text for accessibility"/>
  <wp:cNvGraphicFramePr>
    <a:graphicFrameLocks noChangeAspect="1"/>
  </wp:cNvGraphicFramePr>
  <a:graphic>…</a:graphic>
</wp:inline>
```

### CT_Inline attributes

| Attribute | Type | Default | Notes |
|-----------|------|---------|-------|
| `distT` | UInt32 (EMU) | 0 | Space above the image to text |
| `distB` | UInt32 (EMU) | 0 | Space below the image to text |
| `distL` | UInt32 (EMU) | 0 | Space to the left of the image |
| `distR` | UInt32 (EMU) | 0 | Space to the right of the image |

### CT_Inline children (in order)

| # | Element | Required | Notes |
|---|---------|----------|-------|
| 1 | `wp:extent` | **yes** | `cx` and `cy` attributes: display size in EMU |
| 2 | `wp:effectExtent` | no | Extra space for shadows/glow effects (`l`, `t`, `r`, `b` in EMU) |
| 3 | `wp:docPr` | **yes** | `id` (unique positive int per document), `name`, optional `descr` (alt text) |
| 4 | `wp:cNvGraphicFramePr` | no | Non-visual graphic frame properties |
| 5 | `a:graphic` | **yes** | DrawingML graphic container (see §6) |

---

## 4. Floating Drawing: CT_Anchor

`<wp:anchor>` positions the image outside the text flow. The image can be
behind text, in front of text, or wrapped by text.

```xml
<wp:anchor relativeHeight="251658240" behindDoc="0" locked="0"
           layoutInCell="1" allowOverlap="1"
           distT="114300" distB="114300" distL="114300" distR="114300">
  <wp:simplePos x="0" y="0"/>
  <wp:positionH relativeFrom="column">
    <wp:align>left</wp:align>
  </wp:positionH>
  <wp:positionV relativeFrom="paragraph">
    <wp:posOffset>0</wp:posOffset>
  </wp:positionV>
  <wp:extent cx="914400" cy="914400"/>
  <wp:effectExtent l="0" t="0" r="0" b="0"/>
  <wp:wrapSquare wrapText="bothSides" distT="114300" distB="114300"
                 distL="114300" distR="114300"/>
  <wp:docPr id="2" name="Image 2"/>
  <wp:cNvGraphicFramePr>
    <a:graphicFrameLocks noChangeAspect="1"/>
  </wp:cNvGraphicFramePr>
  <a:graphic>…</a:graphic>
</wp:anchor>
```

### CT_Anchor attributes

| Attribute | Required | Notes |
|-----------|----------|-------|
| `relativeHeight` | **yes** | Z-order layer. 0–1,677,721,599 = behind text; ≥ 1,677,721,600 = in front. Common default: 251658240. |
| `behindDoc` | **yes** | `1` = image drawn behind text |
| `locked` | **yes** | `1` = position locked by user |
| `layoutInCell` | **yes** | `1` = participate in table cell layout |
| `allowOverlap` | **yes** | `1` = other anchored images may overlap |
| `hidden` | no | `1` = image is hidden |
| `distT/distB/distL/distR` | no | Text-to-image distances in EMU |

### CT_Anchor children (in order)

| # | Element | Required | Notes |
|---|---------|----------|-------|
| 1 | `wp:simplePos` | **yes** | Legacy simple position (`x`, `y`); used when `simplePos=1` attribute is set. Otherwise set to 0,0. |
| 2 | `wp:positionH` | **yes** | Horizontal position (see §5) |
| 3 | `wp:positionV` | **yes** | Vertical position (see §5) |
| 4 | `wp:extent` | **yes** | Display size in EMU |
| 5 | `wp:effectExtent` | no | Shadow/glow padding |
| 6 | wrap type | **yes** | One of: `wp:wrapNone`, `wp:wrapSquare`, `wp:wrapTight`, `wp:wrapThrough`, `wp:wrapTopAndBottom` |
| 7 | `wp:docPr` | **yes** | id + name (+ optional descr) |
| 8 | `wp:cNvGraphicFramePr` | no | Non-visual properties |
| 9 | `a:graphic` | **yes** | DrawingML graphic container |

---

## 5. Positioning: positionH / positionV

```xml
<wp:positionH relativeFrom="column">
  <wp:align>center</wp:align>
  <!-- OR -->
  <wp:posOffset>457200</wp:posOffset>   <!-- explicit EMU offset -->
</wp:positionH>
```

### relativeFrom values for positionH

| Value | Meaning |
|-------|---------|
| `margin` | Relative to text area (inside margins) |
| `page` | Relative to page edge |
| `column` | Relative to current column edge |
| `character` | Relative to the character anchor position |
| `leftMargin` | Relative to left margin |
| `rightMargin` | Relative to right margin |
| `insideMargin` | Relative to inside margin (mirrored pages) |
| `outsideMargin` | Relative to outside margin (mirrored pages) |

### relativeFrom values for positionV

| Value | Meaning |
|-------|---------|
| `margin` | Relative to top text area |
| `page` | Relative to page top |
| `paragraph` | Relative to paragraph top |
| `line` | Relative to line |
| `topMargin` | Relative to top margin |
| `bottomMargin` | Relative to bottom margin |
| `insideMargin` | Mirrored inside margin |
| `outsideMargin` | Mirrored outside margin |

### Align values

`left`, `right`, `center`, `inside`, `outside` (horizontal)
`top`, `bottom`, `center`, `inside`, `outside` (vertical)

---

## 6. Wrap Types

### `<wp:wrapNone/>`
No text wrapping; the image overlaps text. The `behindDoc` attribute controls
whether text appears in front of or behind the image.

### `<wp:wrapSquare>`
Text wraps around the bounding rectangle of the image.
Attributes: `wrapText` (`bothSides`, `left`, `right`, `largest`),
`distT`, `distB`, `distL`, `distR` (in EMU).

### `<wp:wrapTight>`
Text wraps tightly around the image's wrap polygon. Has a `wrapPolygon` child.

### `<wp:wrapThrough>`
Like `wrapTight` but text fills in holes inside the polygon. Has a `wrapPolygon` child.

### `<wp:wrapTopAndBottom>`
Text appears only above and below the image, never beside it.
Attributes: `distT`, `distB` (in EMU).

---

## 7. DrawingML Graphic Container

The `<a:graphic>` element wraps the actual picture content. For JPEG/PNG
raster images, `uri` is the picture namespace URI:

```xml
<a:graphic>
  <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/picture">
    <pic:pic>…</pic:pic>
  </a:graphicData>
</a:graphic>
```

---

## 8. Picture Element: CT_Picture (`<pic:pic>`)

```xml
<pic:pic xmlns:pic="http://schemas.openxmlformats.org/drawingml/2006/picture">
  <pic:nvPicPr>
    <pic:cNvPr id="0" name="image1.png" descr="Alt text"/>
    <pic:cNvPicPr>
      <a:picLocks noChangeAspect="1"/>
    </pic:cNvPicPr>
  </pic:nvPicPr>
  <pic:blipFill>
    <a:blip r:embed="rId1" cstate="print"/>
    <a:stretch>
      <a:fillRect/>
    </a:stretch>
  </pic:blipFill>
  <pic:spPr>
    <a:xfrm>
      <a:off x="0" y="0"/>
      <a:ext cx="914400" cy="914400"/>
    </a:xfrm>
    <a:prstGeom prst="rect">
      <a:avLst/>
    </a:prstGeom>
  </pic:spPr>
</pic:pic>
```

### pic:nvPicPr — non-visual picture properties
- `pic:cNvPr`: `id` (unique), `name` (filename), `descr` (alt text for accessibility)
- `pic:cNvPicPr`: optional `<a:picLocks>` (commonly `noChangeAspect="1"`)

### pic:blipFill — image fill
- `<a:blip r:embed="rId1"/>` — references the image relationship by ID
- `cstate` attribute on `a:blip`: `email`, `screen`, `print`, `hqprint` — compression hint; `print` is typical
- `<a:stretch><a:fillRect/></a:stretch>` — stretch to fill; `fillRect` = fill the entire rectangle
  (alternatively `<a:tile/>` for tiled fill, but stretch is standard for images)

### pic:spPr — shape properties
- `<a:xfrm>`: `<a:off x="0" y="0"/>` (origin, usually 0,0) and `<a:ext cx="…" cy="…"/>` (size in EMU; must match `wp:extent`)
- `<a:prstGeom prst="rect">`: rectangular clip shape (always `rect` for photographs)

---

## 9. Image Relationship and Content Type

### Relationship (in `word/_rels/document.xml.rels`)

```xml
<Relationship Id="rId1"
  Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  Target="media/image1.png"/>
```

Image parts live in `word/media/` by convention (not required; any valid part
name works, but `media/` is universal).

### Content type (in `[Content_Types].xml`)

```xml
<Default Extension="png"  ContentType="image/png"/>
<Default Extension="jpg"  ContentType="image/jpeg"/>
<Default Extension="jpeg" ContentType="image/jpeg"/>
<Default Extension="gif"  ContentType="image/gif"/>
<Default Extension="tiff" ContentType="image/tiff"/>
<Default Extension="bmp"  ContentType="image/bmp"/>
<Default Extension="emf"  ContentType="image/x-emf"/>
<Default Extension="wmf"  ContentType="image/x-wmf"/>
```

**Use `Default` (extension-based) rather than `Override` (part-name-based)**
for image content types. Using `Override` for every image part is technically
valid but unusual and creates large `[Content_Types].xml` entries.

---

## 10. Round-Trip Gotchas

1. **`wp:extent` and `pic:spPr a:ext` must match.** Both specify the display
   size of the image. If they differ, Word uses `wp:extent` and may rewrite
   `a:ext` on next save. Always keep them identical.

2. **`wp:docPr id` must be unique within the document.** Each `<wp:docPr>` and
   `<wp:cNvPr>` `id` must be a unique positive integer. Duplicate IDs cause
   Word to renumber them on open.

3. **Inline `distL`/`distR` defaults to 0 but must be preserved.** Even when
   `distL="0"`, the attribute should be kept if it was written — some
   applications treat omitted vs explicit zero differently.

4. **`<wp:cNvGraphicFramePr>` is optional but commonly present.** Its
   `<a:graphicFrameLocks noChangeAspect="1"/>` prevents aspect-ratio distortion
   in Word's UI. Round-trip must preserve it exactly.

5. **Anchor `<wp:simplePos>` is always present even when not used.** When
   `simplePos` attribute is `0` (or absent), `<wp:simplePos x="0" y="0"/>`
   still appears in the XML. It is a required child.

6. **`a:blip cstate` is optional but significant.** The `cstate` attribute
   (`email`, `screen`, `print`, `hqprint`) hints at compression state. Round-
   tripping must preserve it if present; do not add it if absent.

7. **`pic:cNvPr id` and `wp:docPr id` are independent integer spaces.** Both
   are present and both must be ≥ 1 (or ≥ 0 for `pic:cNvPr`). They need not
   match. `pic:cNvPr` commonly uses `id="0"`.

8. **Namespace declarations must be on `<w:document>`.** The `wp:`, `pic:`,
   `a:`, and `r:` namespaces must all be declared before they are used.
   Declaring them only on the element that uses them is valid XML but causes
   issues with some OOXML processors.

9. **EMU coordinates fit in `i64`, not `u32`.** `wp:extent cx`/`cy` and
   `a:ext cx`/`cy` are non-negative, but `a:off x`/`y` and `effectExtent`
   values can be negative. Use signed 64-bit integers.

10. **`<a:avLst/>` inside `prstGeom` must be present.** Even if empty, the
    `<a:avLst/>` child is required by the schema. Omitting it produces invalid
    XML that Word silently repairs.

---

## 11. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| WML-I-01 | `test-data/wml/image_inline_props.docx` | Inline image with real 1-inch dimensions; non-zero distL/distR; altText on docPr; `cstate="print"` on blip; noChangeAspect lock |
| WML-I-02 | `test-data/wml/image_floating.docx` | Floating anchor with wrapSquare; column-relative horizontal pos (align=left); paragraph-relative vertical pos (posOffset); explicit distances |
