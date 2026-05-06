# PresentationML Slide Hierarchy — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 Part 1 4th/5th edition §13 (PresentationML),
ISO/IEC 29500-1:2016 §13; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/pml.xsd` and `dml-main.xsd`.

---

## 1. Overview

A PPTX package organises its slide content across three interdependent XML
parts: the slide, the slide layout, and the slide master. Each level inherits
unresolved properties from the level above. Together they form a three-tier
cascade:

```
SlidePart          ppt/slides/slideN.xml
  └─ SlideLayoutPart  ppt/slideLayouts/slideLayoutN.xml
       └─ SlideMasterPart  ppt/slideMasters/slideMaster1.xml
```

All PresentationML elements are in namespace
`http://schemas.openxmlformats.org/presentationml/2006/main` (prefix `p:`).
DrawingML elements referenced from within a slide use prefix `a:`
(`http://schemas.openxmlformats.org/drawingml/2006/main`).

---

## 2. Part Locations, Content Types, and Relationship Types

### Part locations (canonical conventions)

| Part | Typical path |
|------|-------------|
| Presentation | `ppt/presentation.xml` |
| Slide | `ppt/slides/slideN.xml` (N = 1, 2, …) |
| Slide layout | `ppt/slideLayouts/slideLayoutN.xml` |
| Slide master | `ppt/slideMasters/slideMaster1.xml` |

### Content types

| Part | Content type |
|------|-------------|
| Presentation | `application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml` |
| Slide | `application/vnd.openxmlformats-officedocument.presentationml.slide+xml` |
| Slide layout | `application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml` |
| Slide master | `application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml` |

### Relationship types (abbreviated — full URI prefix is `http://schemas.openxmlformats.org/officeDocument/2006/relationships/`)

| Relationship | Type suffix | Direction |
|-------------|-------------|-----------|
| Presentation → slide | `slide` | Presentation owns slides |
| Slide → slide layout | `slideLayout` | Slide references its layout |
| Slide layout → slide master | `slideMaster` | Layout references its master |
| Master → slide layout | `slideLayout` | Master enumerates its layouts |
| All parts → theme | `theme` | Each part can reference a theme |

The `rId` value on each `<Relationship>` element matches the `r:id` attribute
used by elements inside the source XML part (for example, `<p:sldLayoutId r:id="rId1"/>`
on the master, and `<p:sldId r:id="rId2"/>` on the presentation's slide list).

---

## 3. Slide Root Element: `<p:sld>`

`<p:sld>` (schema type `CT_Slide`) is the root element of every slide part.

### Child element order

All children are optional except `<p:cSld>`. **Order is significant:**

| # | Element | Notes |
|---|---------|-------|
| 1 | `p:cSld` | Common slide data — shape tree and background. Required. |
| 2 | `p:clrMapOvr` | Colour map override — inherit from master or provide a full override. |
| 3 | `p:transition` | Slide transition animation. |
| 4 | `p:timing` | Slide timing and animation sequences. |
| 5 | `p:hf` | Header/footer placeholder visibility flags. |
| 6 | `p:extLst` | Extension list (preserve opaquely). |

### Attributes on `<p:sld>`

| Attribute | Type | Default | Notes |
|-----------|------|---------|-------|
| `show` | boolean | true | Whether this slide appears in the presentation. |
| `showMasterSp` | boolean | true | Show master shapes on this slide. |
| `showMasterPhAnim` | boolean | true | Show master placeholder animations. |

### Minimal valid slide

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr>
    <a:masterClrMapping/>
  </p:clrMapOvr>
</p:sld>
```

---

## 4. Slide Layout Root Element: `<p:sldLayout>`

`<p:sldLayout>` (schema type `CT_SlideLayout`) is the root element of a slide
layout part. A layout defines a named arrangement of placeholder shapes that
slides using this layout inherit.

### Attributes on `<p:sldLayout>`

| Attribute | Type | Default | Notes |
|-----------|------|---------|-------|
| `type` | ST_SlideLayoutType | `custom` | Predefined layout category (see below). |
| `preserve` | boolean | false | Prevent removal when no slide references this layout. |
| `userDrawn` | boolean | false | Whether a user created this layout manually. |
| `showMasterSp` | boolean | true | Show master shapes on slides using this layout. |
| `showMasterPhAnim` | boolean | true | Show master placeholder animations. |
| `matchingName` | string | (absent) | Internal matching name, used to find the layout from a legacy format. |

### ST_SlideLayoutType enum values

| Value | Description |
|-------|-------------|
| `blank` | Blank layout (no placeholders) |
| `title` | Title slide layout (two text areas: title + subtitle) |
| `titleOnly` | Title only — single title placeholder, no body |
| `obj` | Title + content — one title and one generic content placeholder |
| `twoObj` | Title + two content placeholders side-by-side |
| `twoContent` | Alias used by some producers; behaves like `twoObj` |
| `picTx` | Picture with caption |
| `txAndPic` | Text and picture |
| `tbl` | Title + table |
| `dgm` | Title + diagram |
| `chart` | Title + chart |
| `media` | Title + media clip |
| `vertTx` | Title + vertical text body |
| `txOverPic` | Text over picture |
| `ctrTitle` | Centered title layout |
| `other` | Generic; used when no enumerated type matches |
| `custom` | Custom layout (user-defined; default) |
| `sldNum` | Slide number |
| `ftr` | Footer |
| `dt` | Date and time |

### Child element order for `<p:sldLayout>`

| # | Element | Notes |
|---|---------|-------|
| 1 | `p:cSld` | Shape tree for layout-level shapes and placeholders. Required. |
| 2 | `p:clrMapOvr` | Colour map override (same semantics as on `<p:sld>`). |
| 3 | `p:transition` | Default transition for slides using this layout. |
| 4 | `p:timing` | Default timing for slides using this layout. |
| 5 | `p:hf` | Header/footer flags. |
| 6 | `p:extLst` | Extensions. |

---

## 5. Slide Master Root Element: `<p:sldMaster>`

`<p:sldMaster>` (schema type `CT_SlideMaster`) is the root element of a slide
master part. The master establishes default formatting for all layouts and
slides that reference it.

### Child element order for `<p:sldMaster>`

| # | Element | Notes |
|---|---------|-------|
| 1 | `p:cSld` | Shape tree for master-level shapes. Required. |
| 2 | `p:clrMap` | Colour slot mapping — 12 required attributes. Required. |
| 3 | `p:sldLayoutIdLst` | List of layouts belonging to this master. |
| 4 | `p:transition` | Default slide transition. |
| 5 | `p:timing` | Default slide timing. |
| 6 | `p:hf` | Header/footer flags. |
| 7 | `p:txStyles` | Default text styles for title, body, and other placeholders. |
| 8 | `p:extLst` | Extensions. |

---

## 6. Colour Map: `<p:clrMap>` and `<p:clrMapOvr>`

### `<p:clrMap>` on slide master

`<p:clrMap>` (schema type `CT_ColorMapping`) maps the 12 logical colour names
used in DrawingML to the 12 theme colour slots. All 12 attributes are required.

```xml
<p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2"
          accent1="accent1" accent2="accent2" accent3="accent3"
          accent4="accent4" accent5="accent5" accent6="accent6"
          hlink="hlink" folHlink="folHlink"/>
```

The 12 attribute names (logical colour names):

| Attribute | Typical mapping |
|-----------|----------------|
| `bg1` | `lt1` (light background) |
| `tx1` | `dk1` (dark text) |
| `bg2` | `lt2` |
| `tx2` | `dk2` |
| `accent1` | `accent1` |
| `accent2` | `accent2` |
| `accent3` | `accent3` |
| `accent4` | `accent4` |
| `accent5` | `accent5` |
| `accent6` | `accent6` |
| `hlink` | `hlink` |
| `folHlink` | `folHlink` |

The attribute values are tokens from `ST_ColorSchemeIndex`:
`dk1`, `lt1`, `dk2`, `lt2`, `accent1`–`accent6`, `hlink`, `folHlink`.

### `<p:clrMapOvr>` on slides and layouts

`<p:clrMapOvr>` (schema type `CT_ColorMappingOverride`) contains exactly one
of two children — a choice:

- `<a:masterClrMapping/>` — inherit the colour mapping from the slide master.
  This is the default for most slides; the element has no attributes or children.
- `<a:overrideClrMapping .../>` — provide a full 12-attribute override with the
  same structure as `<p:clrMap>`. This replaces the master's mapping entirely
  for this slide or layout.

```xml
<!-- Inherit from master (most common) -->
<p:clrMapOvr>
  <a:masterClrMapping/>
</p:clrMapOvr>

<!-- Override all 12 slots -->
<p:clrMapOvr>
  <a:overrideClrMapping bg1="dk1" tx1="lt1" bg2="dk2" tx2="lt2"
    accent1="accent1" accent2="accent2" accent3="accent3"
    accent4="accent4" accent5="accent5" accent6="accent6"
    hlink="hlink" folHlink="folHlink"/>
</p:clrMapOvr>
```

---

## 7. Layout ID List: `<p:sldLayoutIdLst>`

`<p:sldLayoutIdLst>` on the slide master enumerates every slide layout that
belongs to this master. Each entry is `<p:sldLayoutId>`:

```xml
<p:sldLayoutIdLst>
  <p:sldLayoutId id="2147483649" r:id="rId1"/>
  <p:sldLayoutId id="2147483650" r:id="rId2"/>
</p:sldLayoutIdLst>
```

| Attribute | Notes |
|-----------|-------|
| `id` | Unique numeric ID for this layout across the presentation. Convention: values start at 2147483649 (2^31 + 1). |
| `r:id` | Relationship ID pointing to the slide layout part in the master's `.rels` file. |

The order of `<p:sldLayoutId>` entries determines the order layouts appear in
layout galleries.

---

## 8. Master Text Styles: `<p:txStyles>`

`<p:txStyles>` (schema type `CT_SlideMasterTextStyles`) defines default text
formatting for three categories of placeholder on this master:

| Child element | Applies to |
|---------------|-----------|
| `p:titleStyle` | Title placeholders (`type="title"` or `type="ctrTitle"`) |
| `p:bodyStyle` | Body/content placeholders |
| `p:otherStyle` | All other placeholders (date, footer, slide number, header) |

All three are optional individually, but `<p:txStyles>` is typically present
with all three defined. Each child is `CT_TextListStyle`, which may contain:

- `<a:defPPr>` — default paragraph properties applied to any paragraph in this
  placeholder type.
- `<a:lvl1pPr>` through `<a:lvl9pPr>` — paragraph properties for outline
  levels 1–9 (corresponding to `<a:pPr lvl="0"/>` through `<a:pPr lvl="8"/>`).

```xml
<p:txStyles>
  <p:titleStyle>
    <a:lvl1pPr algn="ctr">
      <a:defRPr lang="en-US" sz="4400" b="1" dirty="0">
        <a:solidFill>
          <a:schemeClr val="tx1"/>
        </a:solidFill>
        <a:latin typeface="+mj-lt"/>
      </a:defRPr>
    </a:lvl1pPr>
  </p:titleStyle>
  <p:bodyStyle>
    <a:lvl1pPr marL="342900" indent="-342900">
      <a:buFont typeface="Arial" panose="020B0604020202020204"
                pitchFamily="34" charset="0"/>
      <a:buChar char="•"/>
      <a:defRPr lang="en-US" sz="2800" dirty="0">
        <a:solidFill><a:schemeClr val="tx1"/></a:solidFill>
        <a:latin typeface="+mn-lt"/>
      </a:defRPr>
    </a:lvl1pPr>
    <!-- lvl2pPr through lvl9pPr follow with smaller font sizes and deeper indents -->
  </p:bodyStyle>
  <p:otherStyle>
    <a:defPPr>
      <a:defRPr lang="en-US" dirty="0"/>
    </a:defPPr>
  </p:otherStyle>
</p:txStyles>
```

---

## 9. Placeholder System

### `<p:ph>` element

`<p:ph>` (schema type `CT_Placeholder`) appears inside `<p:nvPr>` (non-visual
properties) within a shape's `<p:nvSpPr>`. It declares that the shape is a
placeholder and specifies which slot it occupies.

```xml
<p:nvSpPr>
  <p:cNvPr id="2" name="Title 1"/>
  <p:cNvSpPr>
    <a:spLocks noGrp="1"/>
  </p:cNvSpPr>
  <p:nvPr>
    <p:ph type="title"/>
  </p:nvPr>
</p:nvSpPr>
```

### `<p:ph>` attributes

| Attribute | Type | Default | Notes |
|-----------|------|---------|-------|
| `type` | ST_PlaceholderType | `obj` | The slot type (see below). |
| `orient` | ST_Direction | `horz` | Orientation: `horz` or `vert`. |
| `sz` | ST_PlaceholderSize | `full` | Relative size: `full`, `half`, `quarter`. |
| `idx` | xsd:unsignedInt | 0 | Numeric index. Slide matches to layout by this value. |
| `hasCustomGeom` | boolean | false | True if the placeholder has a custom geometry. |

### ST_PlaceholderType enum values

| Value | Meaning |
|-------|---------|
| `title` | Title placeholder |
| `body` | Body/content placeholder |
| `ctrTitle` | Centred title (typically on title slides) |
| `subTitle` | Subtitle (on title slides, below ctrTitle) |
| `dt` | Date and time (auto-updating field) |
| `sldNum` | Slide number (auto-updating field) |
| `ftr` | Footer text |
| `hdr` | Header text |
| `obj` | Generic content (default) — accepts text, table, chart, etc. |
| `pic` | Picture |
| `tbl` | Table |
| `chart` | Chart |
| `dgm` | Diagram (SmartArt) |
| `media` | Media clip |
| `clipArt` | Clip art |

### Index conventions

By convention, `idx="0"` is the title placeholder (whichever `type` is used for
the title on this layout). `idx="1"` is typically the first body or content
placeholder. The `dt`, `ftr`, and `sldNum` placeholders have fixed indices
(10, 11, 12 by convention) that match across the three hierarchy levels.

### `<a:spLocks noGrp="1"/>` on `<p:cNvSpPr>`

The `noGrp="1"` attribute on `<a:spLocks>` within `<p:cNvSpPr>` marks the shape
as a placeholder — it cannot be grouped with other shapes. This attribute must be
present on all placeholder shapes.

### Inheritance chain

When a property is not explicitly set on a shape, it is resolved by walking up
the three-tier hierarchy:

```
Slide placeholder shape
  → Layout placeholder with matching (type, idx)
      → Master placeholder with matching type
          → Master <p:txStyles> for the appropriate category
              → Theme font/colour scheme defaults
```

An empty `<p:spPr/>` on a slide shape means "inherit all geometry from the
matched layout placeholder." An absent `<p:txBody>` means "inherit all text
formatting from layout or master."

---

## 10. Example: Slide Layout with Title and Body Placeholders

A `type="obj"` layout provides the two most common placeholders. The layout
part defines their positions; slides that use this layout inherit those
positions unless they supply their own `<a:xfrm>`.

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sldLayout xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
             xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
             xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
             type="obj" preserve="1">
  <p:cSld name="Title and Content">
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="9144000" cy="6858000"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="9144000" cy="6858000"/>
        </a:xfrm>
      </p:grpSpPr>

      <!-- Title placeholder -->
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="title"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="457200" y="274638"/>
            <a:ext cx="8229600" cy="1143000"/>
          </a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>Click to edit title</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>

      <!-- Body (content) placeholder -->
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Content Placeholder 2"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr>
          <a:xfrm>
            <a:off x="457200" y="1600200"/>
            <a:ext cx="8229600" cy="4525963"/>
          </a:xfrm>
          <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
        </p:spPr>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p><a:r><a:t>Click to edit content</a:t></a:r></a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>
</p:sldLayout>
```

---

## 11. Example: Slide Using Layout Placeholders with Multi-Level Text

A slide that uses the `type="obj"` layout above. The title placeholder on the
slide provides only text — its geometry is inherited from the layout. The body
placeholder demonstrates multi-level text using `<a:pPr lvl="..."/>`.

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
       xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
       xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="9144000" cy="6858000"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="9144000" cy="6858000"/>
        </a:xfrm>
      </p:grpSpPr>

      <!-- Title: no spPr — inherits position from layout -->
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="2" name="Title 1"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="title"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="en-US" dirty="0"/>
              <a:t>Quarterly Results</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>

      <!-- Body: multi-level content; lvl="0" is top level, lvl="1" is indented -->
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Content Placeholder 2"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <!-- Level 0 — top-level bullet -->
          <a:p>
            <a:pPr lvl="0"/>
            <a:r><a:rPr lang="en-US" dirty="0"/><a:t>Revenue up 12%</a:t></a:r>
          </a:p>
          <!-- Level 1 — sub-bullet -->
          <a:p>
            <a:pPr lvl="1"/>
            <a:r><a:rPr lang="en-US" dirty="0"/><a:t>APAC region led growth</a:t></a:r>
          </a:p>
          <!-- Level 1 — another sub-bullet -->
          <a:p>
            <a:pPr lvl="1"/>
            <a:r><a:rPr lang="en-US" dirty="0"/><a:t>EMEA flat year-on-year</a:t></a:r>
          </a:p>
          <!-- Level 0 — next top-level item -->
          <a:p>
            <a:pPr lvl="0"/>
            <a:r><a:rPr lang="en-US" dirty="0"/><a:t>Costs reduced 4%</a:t></a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
  <p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>
</p:sld>
```

The `lvl` attribute on `<a:pPr>` is an integer from 0 to 8 corresponding to
outline levels. Level 0 is the top-level bullet; higher values are nested
sub-items. The bullet character, indentation, and font size for each level are
resolved from the matched placeholder's `<a:lvl1pPr>` … `<a:lvl9pPr>` in the
layout or master `<p:txStyles>`.

---

## 12. Example: Relationship Chain from Master to Layout to Slide

### `ppt/slideMasters/_rels/slideMaster1.xml.rels`

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout"
    Target="../slideLayouts/slideLayout1.xml"/>
  <Relationship Id="rId2"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout"
    Target="../slideLayouts/slideLayout2.xml"/>
  <Relationship Id="rId20"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme"
    Target="../theme/theme1.xml"/>
</Relationships>
```

### `ppt/slideLayouts/_rels/slideLayout1.xml.rels`

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster"
    Target="../slideMasters/slideMaster1.xml"/>
</Relationships>
```

### `ppt/slides/_rels/slide1.xml.rels`

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout"
    Target="../slideLayouts/slideLayout1.xml"/>
</Relationships>
```

### `ppt/_rels/presentation.xml.rels` (excerpt)

```xml
<Relationship Id="rId2"
  Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
  Target="slides/slide1.xml"/>
<Relationship Id="rId10"
  Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster"
  Target="slideMasters/slideMaster1.xml"/>
```

The `<p:sldLayoutIdLst>` inside `ppt/slideMasters/slideMaster1.xml` must list
`<p:sldLayoutId r:id="rId1"/>` and `<p:sldLayoutId r:id="rId2"/>` — matching
the `Id` values in the master's `.rels` file.

---

## 13. Round-Trip Gotchas

1. **`<p:spPr/>` with no children means "inherit geometry."** An empty
   `<p:spPr/>` on a placeholder shape is valid and intentional. Do not remove
   or populate it — doing so breaks the inheritance chain and forces the shape
   to use a default position (typically 0,0) rather than the layout's position.

2. **`<p:clrMapOvr>` must be present on slides.** Most slides contain
   `<p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr>`. Omitting this element
   is valid per the schema but Microsoft Office always writes it. Round-tripping
   must not drop it if present.

3. **`idx` matching, not `type` matching.** When resolving a slide placeholder
   to its layout counterpart, the primary match key is `idx`. `type` is a
   secondary hint used only when `idx` is absent or ambiguous. Two placeholders
   with the same `type` but different `idx` values are distinct slots.

4. **`preserve="1"` prevents layout removal.** A layout with `preserve="1"`
   must not be removed from the master even when no slide references it. This
   attribute is commonly set on all layouts shipped with the theme.

5. **`<p:sldLayoutIdLst>` order is significant.** The order determines the
   order layouts appear in the slide layout gallery. Round-tripping must
   preserve the original order.

6. **`id` on `<p:sldLayoutId>` is a document-scoped integer.** It must be
   unique across all layouts in the presentation. Values must not be reused
   even if a layout is removed. Conventionally, values start at 2147483649
   (0x80000001) and increment, but the spec permits any unique positive integer.

7. **`<p:txStyles>` child order is fixed:** `p:titleStyle`, `p:bodyStyle`,
   `p:otherStyle`. Validators reject out-of-sequence children.

8. **Multi-level `lvl` is 0-indexed in XML but displayed as 1-indexed.**
   `<a:pPr lvl="0"/>` corresponds to Level 1 in UI; `lvl="1"` is Level 2, etc.
   The `<a:lvl1pPr>` element in `<p:txStyles>` applies to paragraphs with
   `lvl="0"`.

---

## 14. Fixture Plan

| ID | File | Properties covered |
|----|------|-------------------|
| PML-H-01 | `test-data/pml/hierarchy_basic.pptx` | Single master, two layouts (title + obj), one slide per layout; relationship chain; clrMap; sldLayoutIdLst |
| PML-H-02 | `test-data/pml/hierarchy_multilevel.pptx` | Body placeholder with lvl 0–2 text; inherited geometry (empty spPr); txStyles titleStyle/bodyStyle |
| PML-H-03 | `test-data/pml/hierarchy_clrmap_override.pptx` | Slide with overrideClrMapping (dark/inverted palette); contrast with masterClrMapping sibling slide |
| PML-H-04 | `test-data/pml/hierarchy_preserve.pptx` | Layout with preserve="1" not referenced by any slide; verifies layout is retained on round-trip |
