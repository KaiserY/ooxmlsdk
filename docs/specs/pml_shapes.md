# PresentationML Shape Tree Model — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 Part 1 4th/5th edition §13.2 (PresentationML
common slide data), §19–20 (DrawingML shape tree), ISO/IEC 29500-1:2016 §19–20;
XSD in `schemas/OfficeOpenXML-XMLSchema-Transitional/pml.xsd` and
`dml-main.xsd`.

---

## 1. Overview

Every slide, slide layout, and slide master carries a `<p:cSld>` (common slide
data) element that contains exactly one `<p:spTree>` (shape tree). The shape
tree is a flat, ordered list of shape elements — there is no implicit z-order
other than document order (later elements render on top of earlier ones).

```
<p:cSld>
  <p:bg/>          — optional background
  <p:spTree>       — shape tree (required)
    <p:nvGrpSpPr>  — non-visual group properties (always first)
    <p:grpSpPr>    — group shape properties (always second)
    <p:sp>         — regular shape (0 or more)
    <p:grpSp>      — group shape (0 or more)
    <p:graphicFrame> — graphic frame for tables/charts/diagrams
    <p:cxnSp>      — connector shape
    <p:pic>        — picture
    <p:contentPart> — content part (rare; external content reference)
  </p:spTree>
</p:cSld>
```

PresentationML elements are in namespace
`http://schemas.openxmlformats.org/presentationml/2006/main` (prefix `p:`).
DrawingML shared elements (geometry, transforms, fills, text) are in
`http://schemas.openxmlformats.org/drawingml/2006/main` (prefix `a:`).

---

## 2. Shape Tree Root: `<p:spTree>` and Group Properties

### Fixed first two children

`<p:spTree>` (schema type `CT_GroupShape`) always starts with exactly two
fixed children before any shape content:

**`<p:nvGrpSpPr>`** — non-visual group shape properties. Contains:
- `<p:cNvPr id="1" name=""/>` — the shape tree's own identity; `id` is always
  1, `name` is always empty for the root group.
- `<p:cNvGrpSpPr/>` — non-visual connector group properties; usually empty.
- `<p:nvPr/>` — application-defined non-visual properties; usually empty.

**`<p:grpSpPr>`** — group shape properties (schema type
`CT_GroupShapeProperties`). Contains the root group's transform. For the root
shape tree this transform defines the slide coordinate space itself:

```xml
<p:grpSpPr>
  <a:xfrm>
    <a:off x="0" y="0"/>
    <a:ext cx="9144000" cy="6858000"/>  <!-- full slide at 10×7.5 inches -->
    <a:chOff x="0" y="0"/>
    <a:chExt cx="9144000" cy="6858000"/>
  </a:xfrm>
</p:grpSpPr>
```

A 10×7.5-inch widescreen slide (the standard 4:3 presentation) has
`cx="9144000" cy="6858000"` (EMU). A 16:9 widescreen slide uses
`cx="9144000" cy="5143500"`. The `chOff`/`chExt` on the root group match
`off`/`ext` exactly — the root group maps its coordinate space 1:1.

---

## 3. Regular Shape: `<p:sp>`

`<p:sp>` (schema type `CT_Shape`) is the most common shape element. It
represents a single rectangular or non-rectangular shape that may contain
text, a fill, a border, and effects.

### Child element order

| # | Element | Required | Notes |
|---|---------|----------|-------|
| 1 | `p:nvSpPr` | Yes | Non-visual shape properties |
| 2 | `p:spPr` | Yes | Visual shape properties |
| 3 | `p:style` | No | Shape style reference (theme style matrix index) |
| 4 | `p:txBody` | No | Text body (absent = no text; see drawingml.md §9) |
| 5 | `p:extLst` | No | Extensions |

### `<p:nvSpPr>` — non-visual shape properties

```xml
<p:nvSpPr>
  <p:cNvPr id="5" name="Rectangle 4" descr="A blue rectangle"/>
  <p:cNvSpPr txBox="1"/>
  <p:nvPr/>
</p:nvSpPr>
```

**`<p:cNvPr>`** attributes:

| Attribute | Required | Notes |
|-----------|----------|-------|
| `id` | Yes | Integer shape ID, unique within the part. Values start at 2. |
| `name` | Yes | Display name shown in the selection pane. |
| `descr` | No | Accessibility description (alt text). |
| `hidden` | No | Boolean; hides from selection but still renders. |

**`<p:cNvSpPr>`** attributes and children:

| Item | Notes |
|------|-------|
| `txBox="1"` | Marks the shape as a text box (no preset geometry required). |
| `<a:spLocks>` child | Locking flags: `noGrp="1"` (placeholder), `noSelect="1"`, `noMove="1"`, `noResize="1"`, `noRot="1"`, `noChangeShapeType="1"`. |

**`<p:nvPr>`** children:

| Element | Notes |
|---------|-------|
| `<p:ph>` | Placeholder declaration (see pml_hierarchy.md §9). When present, the shape is a placeholder. |
| `<p:custDataLst>` | Custom application data (preserve opaquely). |
| `<p:extLst>` | Extensions. |

### `<p:spPr>` — shape properties

`<p:spPr>` (schema type `CT_ShapeProperties`) groups all visual properties.
Child element order is mandated by the schema:

```
<a:xfrm>                       — transform (position, size, rotation)
<a:prstGeom> or <a:custGeom>   — geometry (choice; one or neither)
fill choice                    — noFill | solidFill | gradFill | blipFill | pattFill | grpFill
<a:ln>                         — outline / stroke
effect choice                  — effectLst | effectDag
<a:scene3d>                    — 3D scene
<a:sp3d>                       — 3D shape
<a:extLst>                     — extensions
```

All children are optional. An empty `<p:spPr/>` on a placeholder inherits
all geometry from the matched layout placeholder.

---

## 4. Transform: `<a:xfrm>` (CT_Transform2D)

`<a:xfrm>` defines the position, size, rotation, and flip of a shape.

### Attributes

| Attribute | Type | Default | Notes |
|-----------|------|---------|-------|
| `rot` | xsd:int | 0 | Clockwise rotation in 1/60,000 of a degree. |
| `flipH` | boolean | false | Mirror horizontally about the shape's vertical axis. |
| `flipV` | boolean | false | Mirror vertically about the shape's horizontal axis. |

### Children

| Element | Type | Attributes | Notes |
|---------|------|-----------|-------|
| `<a:off>` | CT_Point2D | `x`, `y` | Top-left corner offset in EMU (signed). |
| `<a:ext>` | CT_PositiveSize2D | `cx`, `cy` | Width and height in EMU (positive). |

### Rotation encoding

`rot` is an integer where 60,000 units equal one degree. Common values:

| Degrees | `rot` value |
|---------|------------|
| 0° | 0 |
| 1° | 60,000 |
| 15° | 900,000 |
| 30° | 1,800,000 |
| 45° | 2,700,000 |
| 90° | 5,400,000 |
| 180° | 10,800,000 |
| 270° | 16,200,000 |
| 360° | 21,600,000 |

Positive values rotate clockwise. A full rotation is 21,600,000.

### Flip and rotation application order

1. The shape is placed at its `<a:off>` position with dimensions `<a:ext>`.
2. `flipH` and/or `flipV` are applied (mirroring around the shape's centre).
3. `rot` is applied (rotating around the shape's centre).

Because flip happens before rotation, a shape with `flipH="1"` and `rot="5400000"`
(90° CW) is first mirrored horizontally, then rotated 90° — the combined effect
is a vertical flip, not equivalent to `flipV` alone.

The `<a:off>` coordinates always describe the top-left of the **pre-rotation**
bounding box. After rotation the rendered bounding box shifts, but `off` must
be stored and round-tripped as-is.

---

## 5. Preset Geometry: `<a:prstGeom>`

`<a:prstGeom prst="...">` (schema type `CT_PresetGeometry2D`) selects one of
the ~180 named shape outlines from `ST_ShapeType`. The rendering engine
computes the path from the preset name plus any adjustment values.

### Key `prst` values

| Value | Shape |
|-------|-------|
| `rect` | Rectangle |
| `roundRect` | Rounded rectangle |
| `ellipse` | Ellipse / circle |
| `triangle` | Isosceles triangle |
| `rtTriangle` | Right triangle |
| `diamond` | Diamond |
| `hexagon` | Regular hexagon |
| `star5` | 5-pointed star |
| `star6` | 6-pointed star |
| `bentArrow` | Bent arrow |
| `callout1` | Rectangular callout bubble |
| `wedgeRectCallout` | Wedge rectangular callout |
| `line` | Straight line (zero height) |
| `straightConnector1` | Straight connector |

### `<a:avLst>` — adjustment value list

`<a:avLst>` contains zero or more `<a:gd name="..." fmla="val N"/>` guide
definitions that customise the preset's shape handles. If `<a:avLst>` is
empty, the preset uses its default adjustment values.

```xml
<!-- Rectangle with default geometry -->
<a:prstGeom prst="rect">
  <a:avLst/>
</a:prstGeom>

<!-- Rounded rectangle with a 20% corner radius -->
<a:prstGeom prst="roundRect">
  <a:avLst>
    <a:gd name="adj" fmla="val 20000"/>
  </a:avLst>
</a:prstGeom>
```

The `fmla="val N"` expression sets the named guide to the literal integer N.
For `roundRect`, `adj` controls the corner radius as a fraction of the smaller
dimension × 100,000 — so `val 20000` means 20%.

### No fill

`<a:noFill/>` inside `<p:spPr>` means the shape interior is transparent. This
is an explicit override — absence of any fill child means "inherit from the
applied shape style or theme," which may produce a visible fill. The two are
not equivalent.

---

## 6. Group Shape: `<p:grpSp>`

`<p:grpSp>` (schema type `CT_GroupShape`) nests one or more shapes inside a
single logical group. The group has its own coordinate space that maps to the
slide through the group transform.

### Child element order

| # | Element | Required | Notes |
|---|---------|----------|-------|
| 1 | `p:nvGrpSpPr` | Yes | Non-visual group properties (same structure as on `<p:spTree>`) |
| 2 | `p:grpSpPr` | Yes | Group shape properties including the coordinate transform |
| 3+ | Shape elements | No | Any mix of `p:sp`, `p:grpSp`, `p:graphicFrame`, `p:cxnSp`, `p:pic` |
| last | `p:extLst` | No | Extensions |

### `<p:nvGrpSpPr>` for a nested group

```xml
<p:nvGrpSpPr>
  <p:cNvPr id="10" name="Group 9"/>
  <p:cNvGrpSpPr/>
  <p:nvPr/>
</p:nvGrpSpPr>
```

Unlike the root shape tree's `<p:nvGrpSpPr>`, a nested group has a meaningful
`id` (unique across the part) and a `name` shown in the selection pane.

### `<p:grpSpPr>` and the coordinate transform

`<p:grpSpPr>` (schema type `CT_GroupShapeProperties`) carries a
`CT_GroupTransform2D` transform — identical to `CT_Transform2D` but with two
additional child elements: `<a:chOff>` and `<a:chExt>`.

```
<a:xfrm>
  <a:off x="..." y="..."/>    — group's top-left in slide coordinates (EMU)
  <a:ext cx="..." cy="..."/>  — group's size in slide coordinates (EMU)
  <a:chOff x="..." y="..."/> — origin of child coordinate space
  <a:chExt cx="..." cy="..."/> — size of child coordinate space
</a:xfrm>
```

The relationship between these four values defines a linear mapping. A child
shape at child-space position `(chX, chY)` maps to slide position:

```
slideX = off.x + (chX - chOff.x) * (ext.cx / chExt.cx)
slideY = off.y + (chY - chOff.y) * (ext.cy / chExt.cy)
```

When `chOff` equals the top-left of the children's bounding box and `chExt`
equals the size of the children's bounding box, the group's slide-space
transform (`off` + `ext`) directly maps the group's visual rectangle to the
slide without any scaling or translation of child coordinates.

---

## 7. Other Shape Elements

### `<p:graphicFrame>` — tables, charts, and diagrams

`<p:graphicFrame>` (schema type `CT_GraphicalObjectFrame`) hosts a DrawingML
`<a:graphicData>` payload for embedded tables (`<a:tbl>`), charts
(`c:chart`), and SmartArt diagrams (`dgm:relIds`). Structure:

```xml
<p:graphicFrame>
  <p:nvGraphicFramePr>
    <p:cNvPr id="6" name="Table 5"/>
    <p:cNvGraphicFramePr><a:graphicFrameLocks noGrp="1"/></p:cNvGraphicFramePr>
    <p:nvPr/>
  </p:nvGraphicFramePr>
  <p:xfrm>
    <a:off x="1143000" y="1600200"/>
    <a:ext cx="6858000" cy="3600000"/>
  </p:xfrm>
  <a:graphic>
    <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
      <a:tbl>…</a:tbl>
    </a:graphicData>
  </a:graphic>
</p:graphicFrame>
```

Note: `<p:xfrm>` on `<p:graphicFrame>` is a PML-namespace element, not
`<a:xfrm>`. The child transform is `<a:off>` and `<a:ext>` inside `<p:xfrm>`.

### `<p:cxnSp>` — connector

`<p:cxnSp>` (schema type `CT_Connector`) is a line or bent connector. It has
the same non-visual property structure as `<p:sp>` but uses
`<p:cNvCxnSpPr>` with optional `<a:stCxn>` / `<a:endCxn>` to anchor
endpoints to other shapes. The `<p:spPr>` geometry uses line preset names such
as `straightConnector1`, `bentConnector3`.

### `<p:pic>` — picture

`<p:pic>` (schema type `CT_Picture`) embeds a raster or vector image using a
`<p:blipFill>` that references the image part via `r:embed`. See
`drawingml.md §11` for the full blipFill structure.

---

## 8. Example: Two Shapes Demonstrating Rotation and Flip

```xml
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

  <!-- Shape A: rotated 45° clockwise; no flip -->
  <p:sp>
    <p:nvSpPr>
      <p:cNvPr id="2" name="Diamond Rotated"/>
      <p:cNvSpPr/>
      <p:nvPr/>
    </p:nvSpPr>
    <p:spPr>
      <a:xfrm rot="2700000">            <!-- 45° CW: 45 × 60000 = 2700000 -->
        <a:off x="914400" y="914400"/>  <!-- 1 in from top-left -->
        <a:ext cx="1828800" cy="1828800"/>  <!-- 2 in × 2 in -->
      </a:xfrm>
      <a:prstGeom prst="rect">
        <a:avLst/>
      </a:prstGeom>
      <a:solidFill>
        <a:schemeClr val="accent1"/>
      </a:solidFill>
    </p:spPr>
  </p:sp>

  <!-- Shape B: horizontally and vertically flipped, no rotation -->
  <!-- flipH + flipV combined = 180° rotation of the interior content -->
  <p:sp>
    <p:nvSpPr>
      <p:cNvPr id="3" name="Triangle FlipHV"/>
      <p:cNvSpPr/>
      <p:nvPr/>
    </p:nvSpPr>
    <p:spPr>
      <a:xfrm flipH="1" flipV="1">
        <a:off x="3657600" y="914400"/>  <!-- 4 in from left -->
        <a:ext cx="1828800" cy="1828800"/>
      </a:xfrm>
      <a:prstGeom prst="rtTriangle">    <!-- right triangle pointing to bottom-right -->
        <a:avLst/>
      </a:prstGeom>
      <a:solidFill>
        <a:schemeClr val="accent2"/>
      </a:solidFill>
    </p:spPr>
  </p:sp>
</p:spTree>
```

The `rtTriangle` preset has its right angle at the bottom-left by default.
With `flipH="1" flipV="1"` the right angle moves to the top-right — the shape
is first mirrored horizontally (right angle moves to bottom-right), then
mirrored vertically (right angle moves to top-right).

A shape with only `rot` applied still has its `<a:off>` coordinates describe
the top-left of the pre-rotation bounding box. After a 45° rotation the
visual bounding box is larger than `ext` and shifts position, but the stored
`off` values do not change. Renderers compute the visual bounding box by
rotating the four corners of the `off`+`ext` rectangle and finding their
axis-aligned envelope.

---

## 9. Example: Group Shape with `chOff`/`chExt` Matching Children's Bounding Box

This example groups two shapes. The group's `chOff`/`chExt` are set to the
bounding box of the two children so that the group's slide-space `off`/`ext`
maps 1:1 to its visual footprint with no internal scaling.

The two child shapes are at child-space positions:
- Shape A: `off (1000000, 500000)`, `ext (2000000, 1500000)` — right edge at 3000000, bottom at 2000000
- Shape B: `off (3500000, 800000)`, `ext (1500000, 1000000)` — right edge at 5000000, bottom at 1800000

Children's bounding box:
- `chOff`: left=1000000, top=500000
- `chExt`: width=4000000 (5000000−1000000), height=1500000 (2000000−500000)

The group occupies `off (2000000, 1500000)` on the slide with `ext (4000000, 1500000)`,
which matches `chExt` exactly — no scaling occurs.

```xml
<p:grpSp>
  <p:nvGrpSpPr>
    <p:cNvPr id="10" name="Group 9"/>
    <p:cNvGrpSpPr/>
    <p:nvPr/>
  </p:nvGrpSpPr>
  <p:grpSpPr>
    <a:xfrm>
      <!-- Group's position and size in slide coordinates -->
      <a:off x="2000000" y="1500000"/>
      <a:ext cx="4000000" cy="1500000"/>
      <!-- Child coordinate space: origin at children's top-left, size = children's bounding box -->
      <a:chOff x="1000000" y="500000"/>
      <a:chExt cx="4000000" cy="1500000"/>
    </a:xfrm>
    <a:noFill/>
  </p:grpSpPr>

  <!-- Child shape A — coordinates in child space -->
  <p:sp>
    <p:nvSpPr>
      <p:cNvPr id="11" name="Rectangle 10"/>
      <p:cNvSpPr/>
      <p:nvPr/>
    </p:nvSpPr>
    <p:spPr>
      <a:xfrm>
        <a:off x="1000000" y="500000"/>
        <a:ext cx="2000000" cy="1500000"/>
      </a:xfrm>
      <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
      <a:solidFill><a:srgbClr val="4472C4"/></a:solidFill>
    </p:spPr>
  </p:sp>

  <!-- Child shape B — coordinates in child space -->
  <p:sp>
    <p:nvSpPr>
      <p:cNvPr id="12" name="Ellipse 11"/>
      <p:cNvSpPr/>
      <p:nvPr/>
    </p:nvSpPr>
    <p:spPr>
      <a:xfrm>
        <a:off x="3500000" y="800000"/>
        <a:ext cx="1500000" cy="1000000"/>
      </a:xfrm>
      <a:prstGeom prst="ellipse"><a:avLst/></a:prstGeom>
      <a:solidFill><a:srgbClr val="ED7D31"/></a:solidFill>
    </p:spPr>
  </p:sp>
</p:grpSp>
```

When `ext` equals `chExt` and `off` is chosen independently of `chOff`, the
mapping reduces to a pure translation: child coordinates shift by
`(off.x − chOff.x, off.y − chOff.y)` to reach slide coordinates. Here,
`off − chOff = (2000000−1000000, 1500000−500000) = (1000000, 1000000)`, so
every child shape moves 1,000,000 EMU right and 1,000,000 EMU down to reach
its slide position.

---

## 10. Shape ID Rules

Every shape element (`<p:sp>`, `<p:grpSp>`, `<p:graphicFrame>`, `<p:cxnSp>`,
`<p:pic>`) carries a numeric `id` on its `<p:cNvPr>`. These IDs must be:

- **Unique within a single part** (slide, layout, or master). The same ID
  value may reappear in a different part without conflict.
- **Greater than zero.** `id="1"` is conventionally reserved for the root
  `<p:spTree>`'s `<p:nvGrpSpPr>`.
- **Not reused** after a shape is deleted; monotonically increasing IDs are
  conventional but not required by the schema.

When constructing a new shape tree, a safe approach is to scan all existing
`<p:cNvPr id="...">` values in the part and assign `max(existing) + 1` to any
new shape.

---

## 11. Z-Order

Shapes render in document order — the first child of `<p:spTree>` after the
two fixed headers renders at the bottom (back), and the last child renders at
the top (front). There is no explicit z-index attribute.

To bring a shape to the front, move its element to be the last sibling in
`<p:spTree>`. To send it to the back, move it immediately after `<p:grpSpPr>`.
Within a `<p:grpSp>`, the same rule applies to the group's children.

---

## 12. Round-Trip Gotchas

1. **`<p:nvGrpSpPr>` and `<p:grpSpPr>` must be the first two children.** The
   XSD enforces this sequence. Any shape element appearing before them produces
   a schema-invalid document.

2. **`id` on `<p:cNvPr>` must be unique within the part.** A round-trip that
   duplicates IDs (e.g. from careless copy-paste logic) will cause silent
   presentation errors in consuming applications.

3. **Group `chOff`/`chExt` must match the children's actual bounding box** for
   correct round-trip rendering. If a child shape's position is changed
   without updating the group's `chOff`/`chExt`, the rendered positions of all
   children inside the group shift.

4. **`rot` is in 60,000ths of a degree, not degrees or radians.** A 90°
   rotation is `rot="5400000"`, not `rot="90"`. Writing `rot="90"` produces
   a 0.0015° rotation — imperceptible and wrong.

5. **`flipH`/`flipV` are not the same as 180° rotation.** `flipH="1"` alone
   mirrors the shape about its vertical centre axis; it does not rotate it.
   `rot="10800000"` (180°) rotates all content including any text; `flipH="1"`
   mirrors geometry but may treat text differently (text may or may not mirror
   depending on the consuming application).

6. **Empty `<p:spPr/>` on a placeholder inherits all geometry.** Do not
   populate geometry into an empty `<p:spPr/>` when round-tripping placeholder
   shapes — doing so overrides layout inheritance and forces the shape to a
   fixed position.

7. **`<p:grpSp>` child shapes use child-space coordinates, not slide
   coordinates.** The `<a:off>` values on shapes inside a group are in the
   group's child coordinate space defined by `chOff`/`chExt`. They are not
   directly comparable to the `<a:off>` values on shapes at the slide level.

8. **`<a:noFill/>` vs absent fill on `<p:grpSpPr>`.** An absent fill on a
   group means "show the slide background through the group." An explicit
   `<a:noFill/>` makes the group interior transparent. In practice these render
   identically, but `<a:noFill/>` is more explicit and should be preserved
   as-is.

9. **`txBox="1"` on `<p:cNvSpPr>` indicates a text box, not a placeholder.**
   Text boxes have no `<p:ph>` in `<p:nvPr>`. The geometry of a text box is
   always explicitly set in `<p:spPr>` — there is nothing to inherit from a
   layout.

10. **Shape elements after `<p:extLst>` are invalid.** `<p:extLst>` must be
    the last child of `<p:sp>`, `<p:grpSp>`, and other shape elements. A
    serialiser that appends an extension list before the final `<p:txBody>`
    produces a schema-invalid document.

---

## 13. Fixture Plan

| ID | File | Properties covered |
|----|------|-------------------|
| PML-S-01 | `test-data/pml/shapes_basic.pptx` | spTree structure; two sp elements with preset geometry; xfrm with off/ext; solidFill; noFill |
| PML-S-02 | `test-data/pml/shapes_rotation.pptx` | rot at 45°/90°/270°; flipH only; flipV only; flipH+flipV combined; off preserved through round-trip |
| PML-S-03 | `test-data/pml/shapes_group.pptx` | grpSp with two child shapes; chOff/chExt matching child bounding box; nested group inside a group |
| PML-S-04 | `test-data/pml/shapes_txbox.pptx` | txBox="1" shapes; multi-paragraph txBody; endParaRPr preservation; bodyPr wrap/anchor attributes |
| PML-S-05 | `test-data/pml/shapes_graphicframe.pptx` | graphicFrame wrapping a DrawingML table; p:xfrm structure; graphicData uri |
