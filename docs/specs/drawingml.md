# DrawingML Shared Model — Clean-Room Specification
## Drawing Layer for OOXML Parsers (DOCX, XLSX, PPTX)

**Purpose:** This document is a clean-room synthesis of the DrawingML shared
model as defined in ECMA-376 Part 1 (5th edition) and ISO/IEC 29500-1:2016.
It covers the drawing layer that all three OOXML formats — WordprocessingML,
SpreadsheetML, and PresentationML — share. It is intended as an implementation
guide and test oracle for `ooxmlsdk` and similar Rust OOXML parsers.

**Sources used:**
- ECMA-376 Part 1, 5th edition — normative XML structure
- ISO/IEC 29500-1:2016 — cross-reference for DrawingML semantics
- `dml-main.xsd` from this repository (`schemas/OfficeOpenXML-XMLSchema-Transitional/`)
- Microsoft Open Specifications: `[MS-ODRAW]`, `[MS-PPTX]`, `[MS-DOCX]`
- Existing `ooxmlsdk` test fixtures (`test-data/`)
- python-pptx and Apache POI source code (reference implementations)

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or have been clearly
rewritten.

---

## 1. Namespace and Conventional Prefix

```
Namespace URI:  http://schemas.openxmlformats.org/drawingml/2006/main
Conventional prefix: a
```

All elements described in this document are in the `a:` namespace unless
otherwise stated. Schema file: `dml-main.xsd`.

Related namespaces that appear alongside DrawingML:

| Prefix | Namespace URI | Usage |
|---|---|---|
| `r` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships` | Relationship references (`r:embed`, `r:link`, `r:id`) |
| `p` | `http://schemas.openxmlformats.org/presentationml/2006/main` | PresentationML host elements |
| `wp` | `http://schemas.openxmlformats.org/drawingml/2006/wordprocessingDrawing` | Word inline/anchor wrappers |
| `pic` | `http://schemas.openxmlformats.org/drawingml/2006/picture` | Picture element in Word/Excel |

---

## 2. Coordinate System and Units

### 2.1 EMU — English Metric Units

All positional and dimensional values in DrawingML are expressed in EMU
(English Metric Units). EMU is an integer type chosen to divide evenly into
both imperial and metric measurements without rounding.

Key conversion factors:

| Conversion | Value |
|---|---|
| 1 inch | 914,400 EMU |
| 1 centimetre | 360,000 EMU |
| 1 point (1/72 inch) | 12,700 EMU |
| 1 pixel at 96 dpi | 9,525 EMU |
| 1 pixel at 72 dpi | 12,700 EMU |

The schema type `ST_CoordinateUnqualified` is a 64-bit signed integer
(`xsd:long`) with range `[−27,273,042,329,600 .. 27,273,042,316,900]`, roughly
±30,000 inches. `ST_PositiveCoordinate` is the same range but non-negative.
`ST_Coordinate32Unqualified` is a 32-bit signed integer, used for quantities
that cannot exceed ~±1.4 metres (line widths, text margins, etc.).

`ST_Coordinate` also accepts strings of the form `"2.54cm"` via the shared
`ST_UniversalMeasure` type, but in practice all producer implementations emit
bare integer EMU values. Parsers must handle both forms.

### 2.2 Size Attributes (`cx`, `cy`) and Offset Attributes (`x`, `y`)

- **`cx`, `cy`** — width and height of a shape or extent, always positive EMU.
  Carried on `<a:ext>` as `CT_PositiveSize2D`.
- **`x`, `y`** — top-left corner offset from the parent coordinate origin,
  signed EMU. Carried on `<a:off>` as `CT_Point2D`.

Both `<a:off>` and `<a:ext>` appear as children of `<a:xfrm>`.

### 2.3 The `<a:xfrm>` Transform Element

`<a:xfrm>` (schema type `CT_Transform2D`) describes the 2D position, size, and
rotation of a shape. It appears as a child of `<a:spPr>`.

```xml
<a:xfrm rot="5400000" flipH="0" flipV="0">
  <a:off x="914400" y="914400"/>
  <a:ext cx="2743200" cy="1143000"/>
</a:xfrm>
```

**Attributes:**

| Attribute | Type | Default | Description |
|---|---|---|---|
| `rot` | `ST_Angle` (xsd:int) | 0 | Clockwise rotation in 1/60,000 of a degree |
| `flipH` | boolean | false | Flip horizontally about vertical axis |
| `flipV` | boolean | false | Flip vertically about horizontal axis |

**Rotation encoding:** `rot` is an integer where 60,000 units equal one degree.
A full 360° turn is 21,600,000 (= 360 × 60,000). A 90° clockwise rotation is
5,400,000. This is distinct from `ST_PositiveFixedAngle`, which represents
angles in the same 1/60,000-degree units but constrains the range to
`[0, 21,600,000)`.

**Rotation application order:** The shape is first placed at its `off`
position, then rotated around its own centre point. If `flipH` or `flipV` are
applied, the flip is applied after rotation.

**Children:**

| Element | Type | Description |
|---|---|---|
| `<a:off>` | `CT_Point2D` | `x`, `y` attributes — top-left offset in EMU |
| `<a:ext>` | `CT_PositiveSize2D` | `cx`, `cy` attributes — width and height in EMU |

For group shapes, `CT_GroupTransform2D` adds `<a:chOff>` and `<a:chExt>` to
express the coordinate system of the group's children independently of the
group's own position. Both carry the same `CT_Point2D` / `CT_PositiveSize2D`
types.

---

## 3. Shape Properties (`<a:spPr>`)

`<a:spPr>` (schema type `CT_ShapeProperties`) is the container element that
groups all visual properties of a shape. It appears as a required child of
shape elements in all three OOXML formats.

**Child element order (must be preserved in this exact sequence):**

```
<a:xfrm>         — transform (optional)
<a:custGeom> or <a:prstGeom>  — geometry (optional)
<a:noFill>, <a:solidFill>, <a:gradFill>, <a:blipFill>, <a:pattFill>, or <a:grpFill>
                 — fill (optional)
<a:ln>           — outline/stroke (optional)
<a:effectLst> or <a:effectDag>  — effects (optional)
<a:scene3d>      — 3D scene (optional)
<a:sp3d>         — 3D shape (optional)
<a:extLst>       — extensions (optional, opaque)
```

The optional `bwMode` attribute on `<a:spPr>` controls how the shape renders
in black-and-white mode (values: `clr`, `auto`, `gray`, `ltGray`, `invGray`,
`grayWhite`, `blackGray`, `blackWhite`, `black`, `white`, `hidden`).

---

## 4. Shape Geometry

### 4.1 Preset Geometry (`<a:prstGeom>`)

Preset geometry refers to one of the ~180 predefined shape outlines defined by
the spec. The rendering engine is responsible for computing the actual path
from the preset name and any adjustment values.

```xml
<a:prstGeom prst="roundRect">
  <a:avLst>
    <a:gd name="adj" fmla="val 16667"/>
  </a:avLst>
</a:prstGeom>
```

**Attribute:** `prst` (required, type `ST_ShapeType`) — the preset shape name.

**Common preset values** (non-exhaustive):

| Value | Shape |
|---|---|
| `rect` | Rectangle |
| `roundRect` | Rounded rectangle |
| `ellipse` | Ellipse/circle |
| `triangle` | Isosceles triangle |
| `rtTriangle` | Right triangle |
| `diamond` | Diamond |
| `parallelogram` | Parallelogram |
| `trapezoid` | Trapezoid |
| `pentagon` | Pentagon |
| `hexagon` | Hexagon |
| `star5` | 5-pointed star |
| `star6` | 6-pointed star |
| `line` | Line (zero height) |
| `arrow` | Single arrow |
| `bentArrow` | Bent arrow |
| `callout1` | Rectangular callout |
| `wedgeRectCallout` | Wedge callout |

The full enumeration has 187 values in `ST_ShapeType`.

**`<a:avLst>` — Adjustment Value List:** Each `<a:gd>` (guide definition)
element within `<a:avLst>` names an adjustment handle and its formula. The
most common pattern is `fmla="val N"` where N is the adjustment value as
a percentage of a reference dimension × 100,000. For `roundRect`, the single
adjustment `adj` controls the corner radius; `val 16667` represents about
16.667% of the smaller dimension. If `<a:avLst>` is empty or absent, the
shape uses its default adjustment values.

### 4.2 Custom Geometry (`<a:custGeom>`)

Custom geometry allows producers to define arbitrary 2D paths using a small
set of drawing commands. It is used when no preset shape matches and for
preserved fidelity when importing from other formats.

**Schema structure (`CT_CustomGeometry2D`):**

```
<a:custGeom>
  <a:avLst>          — adjustment value guides (optional)
  <a:gdLst>          — computed guide definitions (optional)
  <a:ahLst>          — adjustment handle list (optional)
  <a:cxnLst>         — connection site list (optional)
  <a:rect>           — text box rectangle (optional)
  <a:pathLst>        — path list (required)
</a:custGeom>
```

**`<a:pathLst>` contains one or more `<a:path>` elements.** Each `<a:path>`
has attributes:

| Attribute | Default | Description |
|---|---|---|
| `w` | 0 | Path coordinate space width |
| `h` | 0 | Path coordinate space height |
| `fill` | `norm` | Fill mode: `none`, `norm`, `lighten`, `lightenLess`, `darken`, `darkenLess` |
| `stroke` | `true` | Whether to stroke this path |
| `extrusionOk` | `true` | Whether 3D extrusion is allowed |

Path commands inside `<a:path>` (order arbitrary, repeated freely):

| Element | Description |
|---|---|
| `<a:moveTo><a:pt x="..." y="..."/></a:moveTo>` | Move current point without drawing |
| `<a:lnTo><a:pt x="..." y="..."/></a:lnTo>` | Line to point |
| `<a:cubicBezTo>` | Cubic Bézier: requires exactly 3 `<a:pt>` children (two control points + endpoint) |
| `<a:quadBezTo>` | Quadratic Bézier: requires exactly 2 `<a:pt>` children (one control point + endpoint) |
| `<a:arcTo wR="..." hR="..." stAng="..." swAng="..."/>` | Arc: `wR`/`hR` = half-width/height of the bounding ellipse; `stAng`/`swAng` = start angle and sweep angle in 1/60,000 degrees |
| `<a:close/>` | Close path (line back to last moveTo point) |

Point coordinates (`x`, `y`) inside path commands are in the path's own
coordinate space (`w` × `h`), not in EMU.

**Example — right triangle:**

```xml
<a:custGeom>
  <a:avLst/>
  <a:pathLst>
    <a:path w="100" h="100">
      <a:moveTo><a:pt x="0" y="100"/></a:moveTo>
      <a:lnTo><a:pt x="100" y="100"/></a:lnTo>
      <a:lnTo><a:pt x="0" y="0"/></a:lnTo>
      <a:close/>
    </a:path>
  </a:pathLst>
</a:custGeom>
```

---

## 5. Fills

All fill types are expressed as a choice group (`EG_FillProperties`) that
appears in `<a:spPr>`, `<a:ln>`, `<a:rPr>`, and various other containers.
The six members of the group are:

### 5.1 No Fill (`<a:noFill/>`)

The shape interior (or line, or text) is transparent. This is an explicit
declaration — the absence of any fill child means "inherit from theme or style",
which is different from `<a:noFill/>`.

```xml
<a:noFill/>
```

### 5.2 Solid Fill (`<a:solidFill>`)

A single, uniform colour. Contains exactly one colour child from the
`EG_ColorChoice` group (see §7).

```xml
<a:solidFill>
  <a:srgbClr val="4472C4"/>
</a:solidFill>
```

### 5.3 Gradient Fill (`<a:gradFill>`)

A gradient blending between two or more colour stops. The schema type is
`CT_GradientFillProperties`.

**Attributes:**

| Attribute | Default | Description |
|---|---|---|
| `flip` | `none` | Whether to flip the gradient: `none`, `x`, `y`, `xy` |
| `rotWithShape` | (absent) | Whether gradient direction rotates with shape |

**Children:**

```
<a:gsLst>     — gradient stop list (optional but almost always present)
  <a:gs pos="0">       — stop at 0% (start)
    <colour child>
  </a:gs>
  <a:gs pos="100000">  — stop at 100% (end)
    <colour child>
  </a:gs>
</a:gsLst>
<a:lin ang="..." scaled="0"/>   — linear gradient direction
 —OR—
<a:path path="...">             — radial/rectangular gradient
  <a:fillToRect .../>
</a:path>
<a:tileRect .../>   — tile rectangle (rarely used with gradients)
```

`<a:gsLst>` requires a minimum of 2 `<a:gs>` elements. The `pos` attribute on
`<a:gs>` is a percentage value from `0` to `100000` (0% to 100%) using
`ST_PositiveFixedPercentage`.

**`<a:lin>`** — linear gradient:
- `ang` — angle in 1/60,000 degrees (clockwise from left); 0 = left-to-right,
  5,400,000 = top-to-bottom, 10,800,000 = right-to-left.
- `scaled` — boolean; when `"1"`, the angle is scaled relative to shape
  dimensions; when `"0"` (the common case), it is an absolute angle.

**`<a:path>`** — path-based gradient (radial, circular, or shape-based):
- `path` attribute: `shape`, `circle`, or `rect`.
- The optional `<a:fillToRect>` child defines the focus rectangle using
  `l`, `t`, `r`, `b` as `ST_Percentage` values (0 to 100000).

**Example — horizontal two-stop gradient:**

```xml
<a:gradFill rotWithShape="1">
  <a:gsLst>
    <a:gs pos="0">
      <a:schemeClr val="accent1">
        <a:tint val="50000"/>
      </a:schemeClr>
    </a:gs>
    <a:gs pos="100000">
      <a:schemeClr val="accent1"/>
    </a:gs>
  </a:gsLst>
  <a:lin ang="5400000" scaled="0"/>
</a:gradFill>
```

### 5.4 Pattern Fill (`<a:pattFill>`)

A tiled hatch/pattern using foreground and background colours. The `prst`
attribute selects the pattern (type `ST_PresetPatternVal`).

Common `prst` values: `pct5`, `pct10`, `pct25`, `pct50`, `horz`, `vert`,
`ltHorz`, `ltVert`, `dkHorz`, `dkVert`, `cross`, `diagCross`, `dnDiag`,
`upDiag`, `dotGrid`, `lgGrid`, `wave`, `zigZag`, etc.

```xml
<a:pattFill prst="ltHorz">
  <a:fgClr>
    <a:srgbClr val="000000"/>
  </a:fgClr>
  <a:bgClr>
    <a:srgbClr val="FFFFFF"/>
  </a:bgClr>
</a:pattFill>
```

`<a:fgClr>` and `<a:bgClr>` are both `CT_Color` types (containing one
`EG_ColorChoice` child each).

### 5.5 Blip Fill — Image Fill (`<a:blipFill>`)

Fills the shape with a raster image. The image is referenced by relationship ID.

**Schema type `CT_BlipFillProperties`:**

```xml
<a:blipFill dpi="0" rotWithShape="1">
  <a:blip r:embed="rId3" cstate="none"/>
  <a:srcRect l="0" t="0" r="0" b="0"/>
  <a:stretch>
    <a:fillRect/>
  </a:stretch>
</a:blipFill>
```

**Attributes on `<a:blipFill>`:**

| Attribute | Description |
|---|---|
| `dpi` | Target DPI for the image; 0 means use the image's native DPI |
| `rotWithShape` | If `true`, the image rotates with the shape |

**`<a:blip>` attributes:**

| Attribute | Description |
|---|---|
| `r:embed` | Relationship ID referencing an internal image part |
| `r:link` | Relationship ID for an externally linked image (`TargetMode="External"`) |
| `cstate` | Compression state hint: `email`, `screen`, `print`, `hqprint`, `none` |

Use `r:embed` for images stored inside the package. Use `r:link` for images
that remain at an external URI. A blip carries at most one of the two.

**`<a:blip>` can also have image-processing effect children** (applied in
order): `alphaBiLevel`, `alphaCeiling`, `alphaFloor`, `alphaInv`, `alphaMod`,
`alphaModFix`, `alphaRepl`, `biLevel`, `blur`, `clrChange`, `clrRepl`,
`duotone`, `fillOverlay`, `grayscl`, `hsl`, `lum`, `tint`.

**`<a:srcRect>`** — crop rectangle. Expresses how much of the source image is
cropped away on each edge, as `ST_Percentage` (0 to 100,000 for 0% to 100%).
`l="10000"` means the left 10% of the image is cropped. Default is all zeros
(no crop).

**Fill mode** — one of:
- `<a:stretch><a:fillRect [l="..." t="..." r="..." b="..."/]></a:stretch>` — scale the image to fill the shape. The optional `fillRect` attributes express insets (in the same percentage format as `srcRect`).
- `<a:tile [tx="..." ty="..." sx="..." sy="..." flip="..." algn="..."]/>` — tile the image. `tx`/`ty` are offset from origin in EMU; `sx`/`sy` are scale percentages; `flip` is `none`/`x`/`y`/`xy`; `algn` is a `ST_RectAlignment` value.

### 5.6 Group Fill (`<a:grpFill/>`)

Indicates that the shape uses the fill of its parent group. Empty element;
no children or attributes. Only valid when the shape is inside a group shape
(`<a:grpSp>` or equivalent). Rare in practice.

---

## 6. Colours

Every colour in DrawingML is expressed as an element from the `EG_ColorChoice`
group. These elements can optionally carry zero or more colour transform
children that modify the base colour.

### 6.1 Colour Source Elements

**`<a:srgbClr val="RRGGBB"/>`** — Hex RGB, uppercase, no `#` prefix. Each
channel is 00–FF. This is the most common colour format in OOXML produced by
interactive applications.

```xml
<a:srgbClr val="4472C4"/>
```

**`<a:schemeClr val="..."/>`** — Reference to a slot in the active theme's
colour scheme. This is the preferred form for brand-consistent documents.
Valid `val` tokens:

| Token | Meaning |
|---|---|
| `dk1` | Dark 1 (maps to `bg1` in theme, resolved via `<clrMap>`) |
| `lt1` | Light 1 |
| `dk2` | Dark 2 |
| `lt2` | Light 2 |
| `accent1`..`accent6` | Six accent colours |
| `hlink` | Hyperlink colour |
| `folHlink` | Followed hyperlink colour |
| `bg1`, `bg2` | Background 1, Background 2 (context-dependent aliases) |
| `tx1`, `tx2` | Text 1, Text 2 (context-dependent aliases) |
| `phClr` | Placeholder colour (inherits from parent placeholder) |

The mapping from `bg1`/`tx1` etc. to the actual `dk1`/`lt1` theme slots is
determined by the `<a:clrMap>` element on the slide master or document.

**`<a:sysClr val="..." lastClr="RRGGBB"/>`** — System colour reference.
`val` is an `ST_SystemColorVal` token (e.g. `windowText`, `btnFace`,
`highlight`). `lastClr` is an optional fallback hex RGB stored when the
document was last saved, used by consumers that cannot query system colours.

**`<a:prstClr val="..."/>`** — One of ~140 predefined CSS-style colour names.
Examples: `red`, `blue`, `darkBlue`, `orange`, `gold`, `lavender`, `salmon`.
The full list in `ST_PresetColorVal` mirrors the CSS named colours with minor
additions and aliases.

**`<a:scrgbClr r="..." g="..." b="..."/>`** — Linear (sRGB gamma-decoded)
percentage RGB. Each channel is a `ST_Percentage` value from `0` to `100000`
(0% to 100%). Rare in practice but present in high-fidelity interchange.

**`<a:hslClr hue="..." sat="..." lum="..."/>`** — HSL colour. `hue` is
`ST_PositiveFixedAngle` (0 to 21,599,999, where 0 and 21,600,000 are both
red); `sat` and `lum` are `ST_Percentage` (0 to 100,000).

### 6.2 Colour Transform Children

Any of the above colour elements can carry zero or more transform children
(from the `EG_ColorTransform` group) that are applied in sequence to modify
the colour. All percentage values use `ST_PositiveFixedPercentage` (0 to
100,000) unless noted:

| Element | Type | Effect |
|---|---|---|
| `<a:tint val="..."/>` | `ST_PositiveFixedPercentage` | Blend toward white; 0=unchanged, 100000=pure white |
| `<a:shade val="..."/>` | `ST_PositiveFixedPercentage` | Blend toward black; 0=unchanged, 100000=pure black |
| `<a:alpha val="..."/>` | `ST_PositiveFixedPercentage` | Set absolute opacity; 0=transparent, 100000=opaque |
| `<a:lumMod val="..."/>` | `ST_Percentage` | Multiply luminance; 100000=unchanged, 50000=half brightness |
| `<a:lumOff val="..."/>` | `ST_Percentage` | Add fixed luminance offset |
| `<a:satMod val="..."/>` | `ST_Percentage` | Multiply saturation |
| `<a:satOff val="..."/>` | `ST_Percentage` | Add fixed saturation offset |
| `<a:hue val="..."/>` | `ST_PositiveFixedAngle` | Set absolute hue |
| `<a:hueMod val="..."/>` | `ST_PositivePercentage` | Multiply hue |
| `<a:hueOff val="..."/>` | `ST_Angle` | Add hue offset (signed) |
| `<a:alphaOff val="..."/>` | `ST_FixedPercentage` | Add alpha offset (signed) |
| `<a:alphaMod val="..."/>` | `ST_PositivePercentage` | Multiply alpha |
| `<a:red val="..."/>` | `ST_Percentage` | Set/offset/modulate red channel |
| `<a:green val="..."/>` | `ST_Percentage` | Set/offset/modulate green channel |
| `<a:blue val="..."/>` | `ST_Percentage` | Set/offset/modulate blue channel |
| `<a:comp/>` | (empty) | Complement (invert) colour |
| `<a:inv/>` | (empty) | Invert all channels |
| `<a:gray/>` | (empty) | Convert to grayscale |
| `<a:gamma/>` | (empty) | Apply sRGB gamma correction |
| `<a:invGamma/>` | (empty) | Invert sRGB gamma correction |

**Example — theme colour at 75% brightness:**

```xml
<a:schemeClr val="accent1">
  <a:lumMod val="75000"/>
</a:schemeClr>
```

**Key gotcha:** All percentage-based transforms use values scaled by 1,000 such
that 100% is represented as `100000`. A value of `50000` means exactly 50%.
This is a common source of off-by-1000x errors.

---

## 7. Line / Outline Properties (`<a:ln>`)

`<a:ln>` (schema type `CT_LineProperties`) defines the stroke of a shape or
the properties of a connector. It appears as a child of `<a:spPr>`.

**Attributes:**

| Attribute | Type | Description |
|---|---|---|
| `w` | `ST_LineWidth` (int, 0–20,116,800) | Stroke width in EMU; max ~1587 pt |
| `cap` | `ST_LineCap` | Stroke cap: `rnd` (round), `sq` (square), `flat` (flat/butt) |
| `cmpd` | `ST_CompoundLine` | Compound line style: `sng` (single), `dbl` (double), `thickThin`, `thinThick`, `tri` (triple) |
| `algn` | `ST_PenAlignment` | Pen alignment: `ctr` (centered on boundary), `in` (inside boundary) |

**Child element order:**

```
<a:noFill/>  or  <a:solidFill>  or  <a:gradFill>  or  <a:pattFill>
             — line fill (optional; absence = inherit)
<a:prstDash> or <a:custDash>
             — dash pattern (optional)
<a:round/>   or  <a:bevel/>  or  <a:miter>
             — line join (optional)
<a:headEnd>  — start arrowhead (optional)
<a:tailEnd>  — end arrowhead (optional)
<a:extLst>   — extensions (optional)
```

Line fills use the same four types as shape fills (`noFill`, `solidFill`,
`gradFill`, `pattFill`). `blipFill` and `grpFill` are not valid for lines.

### 7.1 Dash Patterns

**`<a:prstDash val="..."/>`** — preset dash pattern:

| Value | Pattern |
|---|---|
| `solid` | No dashes |
| `dot` | Dots |
| `dash` | Short dashes |
| `lgDash` | Long dashes |
| `dashDot` | Dash-dot |
| `lgDashDot` | Long dash-dot |
| `lgDashDotDot` | Long dash-dot-dot |
| `sysDash` | System dash |
| `sysDot` | System dot |
| `sysDashDot` | System dash-dot |
| `sysDashDotDot` | System dash-dot-dot |

**`<a:custDash>`** — custom dash pattern, containing zero or more `<a:ds>`
children. Each `<a:ds d="..." sp="..."/>` element defines one dash (length `d`)
followed by one space (`sp`), both as `ST_PositivePercentage` values relative
to the line width.

### 7.2 Line Joins

Three mutually exclusive join styles:

- `<a:round/>` — Rounded join
- `<a:bevel/>` — Bevel (flat cut) join
- `<a:miter [lim="..."]/>` — Miter join; `lim` limits the miter length as
  a `ST_PositivePercentage`

### 7.3 Arrowheads

`<a:headEnd>` (at the start of the path) and `<a:tailEnd>` (at the end):

| Attribute | Values | Default |
|---|---|---|
| `type` | `none`, `triangle`, `stealth`, `diamond`, `oval`, `arrow` | `none` |
| `w` | `sm`, `med`, `lg` | (absent = `med`) |
| `len` | `sm`, `med`, `lg` | (absent = `med`) |

**Example — connector with arrows:**

```xml
<a:ln w="12700">
  <a:solidFill><a:srgbClr val="FF0000"/></a:solidFill>
  <a:prstDash val="dash"/>
  <a:headEnd type="oval" w="sm" len="sm"/>
  <a:tailEnd type="triangle" w="med" len="med"/>
</a:ln>
```

---

## 8. Effects

### 8.1 Effect Containers

Effects in DrawingML are expressed one of two ways:

**`<a:effectLst>` (schema type `CT_EffectList`)** — a flat, ordered list of
at most one of each named effect. Child order is fixed:

```
<a:blur>       — blur
<a:fillOverlay>— fill overlay
<a:glow>       — glow
<a:innerShdw>  — inner shadow
<a:outerShdw>  — outer shadow
<a:prstShdw>   — preset shadow
<a:reflection> — reflection
<a:softEdge>   — soft edges
```

This is the common form produced by Microsoft Office.

**`<a:effectDag>` (schema type `CT_EffectContainer`)** — a directed acyclic
graph of effects that can reference and reuse earlier results. This is the
more powerful but rarely-seen form. Parsers must preserve it opaquely if they
do not implement full effect rendering.

The two forms are mutually exclusive as `EG_EffectProperties`.

### 8.2 Individual Effects

**`<a:outerShdw>` — Outer Shadow** (most common shadow type):

```xml
<a:outerShdw blurRad="40000" dist="23000" dir="5400000"
             algn="ctr" rotWithShape="0">
  <a:srgbClr val="000000">
    <a:alpha val="35000"/>
  </a:srgbClr>
</a:outerShdw>
```

| Attribute | Type | Default | Description |
|---|---|---|---|
| `blurRad` | `ST_PositiveCoordinate` | 0 | Shadow blur radius in EMU |
| `dist` | `ST_PositiveCoordinate` | 0 | Shadow offset distance in EMU |
| `dir` | `ST_PositiveFixedAngle` | 0 | Shadow direction in 1/60,000 degrees |
| `sx` | `ST_Percentage` | 100% | Horizontal scale |
| `sy` | `ST_Percentage` | 100% | Vertical scale |
| `kx` | `ST_FixedAngle` | 0 | Horizontal skew in 1/60,000 degrees |
| `ky` | `ST_FixedAngle` | 0 | Vertical skew |
| `algn` | `ST_RectAlignment` | `b` | Shadow origin alignment |
| `rotWithShape` | boolean | true | Whether shadow rotates with shape |

Contains one colour child.

**`<a:innerShdw>` — Inner Shadow:**

| Attribute | Type | Default | Description |
|---|---|---|---|
| `blurRad` | `ST_PositiveCoordinate` | 0 | Blur radius in EMU |
| `dist` | `ST_PositiveCoordinate` | 0 | Offset distance in EMU |
| `dir` | `ST_PositiveFixedAngle` | 0 | Direction in 1/60,000 degrees |

Contains one colour child.

**`<a:glow rad="...">` — Glow:**

`rad` is `ST_PositiveCoordinate` (EMU). Contains one colour child.

```xml
<a:glow rad="63500">
  <a:schemeClr val="accent1">
    <a:alpha val="40000"/>
  </a:schemeClr>
</a:glow>
```

**`<a:softEdge rad="..."/>` — Soft Edge:**

`rad` is `ST_PositiveCoordinate` (EMU, required). No children.

**`<a:blur rad="..." grow="..."/>` — Blur:**

`rad` is `ST_PositiveCoordinate`, default 0. `grow` is boolean, default true
(whether the blur grows the bounding box).

**`<a:reflection>` — Reflection:**

| Attribute | Default | Description |
|---|---|---|
| `blurRad` | 0 | Blur in EMU |
| `stA` | 100% | Start alpha (opacity at start of reflection) |
| `stPos` | 0% | Gradient start position |
| `endA` | 0% | End alpha |
| `endPos` | 100% | Gradient end position |
| `dist` | 0 | Offset from shape |
| `dir` | 0 | Direction of offset |
| `fadeDir` | 5400000 | Fade direction (default = downward) |
| `sx`, `sy` | 100% | Scale factors |
| `kx`, `ky` | 0 | Skew angles |
| `algn` | `b` | Alignment |
| `rotWithShape` | true | Whether reflection rotates |

No children (the colour/fade is implicit).

---

## 9. Text Body (`<a:txBody>`)

`<a:txBody>` (schema type `CT_TextBody`) is the text container used by shapes,
table cells, and other drawing objects. It always contains three structural
sections in this order:

```xml
<a:txBody>
  <a:bodyPr [attributes...]/>         <!-- REQUIRED, even if empty -->
  <a:lstStyle/>                       <!-- REQUIRED, almost always empty -->
  <a:p>...</a:p>                      <!-- One or more paragraphs, REQUIRED -->
</a:txBody>
```

A `<a:txBody>` with no visible text still has at least one empty `<a:p>`.

### 9.1 Body Properties (`<a:bodyPr>`)

Schema type `CT_TextBodyProperties`. All attributes are optional.

| Attribute | Type | Default | Description |
|---|---|---|---|
| `rot` | `ST_Angle` | (absent) | Text body rotation in 1/60,000 degrees |
| `vert` | `ST_TextVerticalType` | `horz` | Text direction: `horz`, `vert`, `vert270`, `wordArtVert`, `eaVert`, `mongolianVert`, `wordArtVertRtl` |
| `wrap` | `ST_TextWrappingType` | (absent) | `none` = no wrap; `square` = wrap in bounding box |
| `lIns` | `ST_Coordinate32` | (absent) | Left inset in EMU |
| `tIns` | `ST_Coordinate32` | (absent) | Top inset in EMU |
| `rIns` | `ST_Coordinate32` | (absent) | Right inset in EMU |
| `bIns` | `ST_Coordinate32` | (absent) | Bottom inset in EMU |
| `anchor` | `ST_TextAnchoringType` | (absent) | Vertical anchor: `t`, `ctr`, `b`, `just`, `dist` |
| `anchorCtr` | boolean | (absent) | Centre text block within anchor space |
| `upright` | boolean | false | Keep text upright regardless of shape rotation |
| `numCol` | `ST_TextColumnCount` (1–16) | (absent) | Number of text columns |
| `spcCol` | `ST_PositiveCoordinate32` | (absent) | Space between columns in EMU |
| `rtlCol` | boolean | (absent) | Right-to-left column order |
| `vertOverflow` | `ST_TextVertOverflowType` | (absent) | `overflow`, `ellipsis`, `clip` |
| `horzOverflow` | `ST_TextHorzOverflowType` | (absent) | `overflow`, `clip` |
| `spcFirstLastPara` | boolean | (absent) | Add space before first / after last paragraph |
| `forceAA` | boolean | (absent) | Force anti-aliasing |
| `compatLnSpc` | boolean | (absent) | Use legacy line spacing calculation |

Default insets when `lIns`/`rIns` are absent are 91,440 EMU (≈ 0.1 in).
Default `tIns`/`bIns` are 45,720 EMU (≈ 0.05 in). Some hosts define their own
defaults; always emit explicit values for portable documents.

**`<a:bodyPr>` children:** `<a:prstTxWarp>` (text warp preset), auto-fit group
(`<a:noAutofit/>`, `<a:normAutofit>`, `<a:spAutoFit/>`), `<a:scene3d>`, 3D
text group (`<a:sp3d>` or `<a:flatTx>`), `<a:extLst>`.

### 9.2 List Style (`<a:lstStyle>`)

Schema type `CT_TextListStyle`. Almost always written as an empty element
`<a:lstStyle/>` but must be present. Can contain paragraph property overrides
for levels 1–9: `<a:defPPr>`, `<a:lvl1pPr>` through `<a:lvl9pPr>`, and
`<a:extLst>`. Each of these carries `CT_TextParagraphProperties`.

### 9.3 Paragraph (`<a:p>`)

Schema type `CT_TextParagraph`. Contains:

```
<a:pPr>        — paragraph properties (optional)
<a:r>          — text run (zero or more)
<a:br>         — line break (zero or more)
<a:fld>        — field (zero or more)
<a:endParaRPr> — paragraph-end run properties (optional)
```

`<a:r>`, `<a:br>`, and `<a:fld>` may appear in any order and interleaved
with each other (they all belong to the `EG_TextRun` group).

### 9.4 Paragraph Properties (`<a:pPr>`)

Schema type `CT_TextParagraphProperties`. All attributes are optional.

| Attribute | Type | Description |
|---|---|---|
| `algn` | `ST_TextAlignType` | Horizontal alignment: `l`, `ctr`, `r`, `just`, `justLow`, `dist`, `thaiDist` |
| `marL` | `ST_TextMargin` | Left margin in EMU |
| `marR` | `ST_TextMargin` | Right margin in EMU |
| `indent` | `ST_TextIndent` | First-line indent in EMU (can be negative for hanging indent) |
| `lvl` | `ST_TextIndentLevelType` (0–8) | Outline level |
| `defTabSz` | `ST_Coordinate32` | Default tab stop size in EMU |
| `rtl` | boolean | Right-to-left paragraph |
| `eaLnBrk` | boolean | Enable East Asian line breaking rules |
| `latinLnBrk` | boolean | Enable Latin line breaking rules |
| `hangingPunct` | boolean | Allow punctuation to hang outside margin |
| `fontAlgn` | `ST_TextFontAlignType` | Font baseline alignment: `auto`, `t`, `ctr`, `base`, `b` |

**Children of `<a:pPr>`** (in schema order):

- `<a:lnSpc>` — line spacing
- `<a:spcBef>` — space before paragraph
- `<a:spcAft>` — space after paragraph
- `<a:buClrTx/>` or `<a:buClr>` — bullet colour
- `<a:buSzTx/>`, `<a:buSzPct>`, or `<a:buSzPts>` — bullet size
- `<a:buFontTx/>` or `<a:buFont>` — bullet font
- `<a:buNone/>`, `<a:buAutoNum>`, `<a:buChar>`, or `<a:buBlip>` — bullet type
- `<a:tabLst>` — tab stops
- `<a:defRPr>` — default run properties for the paragraph
- `<a:extLst>` — extensions

**Line spacing** (`<a:lnSpc>`, `<a:spcBef>`, `<a:spcAft>`) all use
`CT_TextSpacing`, which contains either:

- `<a:spcPct val="..."/>` — spacing as `ST_TextSpacingPercent` (0 to
  13,200,000, where 100,000 = 100%)
- `<a:spcPts val="..."/>` — spacing as `ST_TextSpacingPoint` (0 to 158,400,
  where 100 = 1 point). Note: **100 units = 1 point**, not 1 unit.

Examples: `<a:spcPts val="1200"/>` = 12 pt, `<a:spcPct val="150000"/>` = 150%.

**Bullets:**

- `<a:buNone/>` — explicit no bullet (overrides inherited bullet from `lstStyle`)
- `<a:buChar char="•"/>` — character bullet
- `<a:buAutoNum type="arabicPeriod" startAt="1"/>` — auto-numbered bullet
- `<a:buBlip><a:blip r:embed="rId..."/></a:buBlip>` — image bullet

### 9.5 Text Runs (`<a:r>`)

Schema type `CT_RegularTextRun`.

```xml
<a:r>
  <a:rPr lang="en-US" sz="2400" b="1" dirty="0"/>
  <a:t>Hello world</a:t>
</a:r>
```

`<a:rPr>` (optional) and `<a:t>` (required). The `<a:t>` element contains
the raw text string; it should preserve leading and trailing whitespace by
using `xml:space="preserve"` when significant whitespace is present, though
in practice many producers omit this.

### 9.6 Run Properties (`<a:rPr>`)

Schema type `CT_TextCharacterProperties`. All attributes optional.

| Attribute | Type | Description |
|---|---|---|
| `lang` | `ST_Lang` (BCP-47) | Spell-check language, e.g. `"en-US"`, `"zh-CN"` |
| `altLang` | `ST_Lang` | Alternate language for East Asian/bidi text |
| `sz` | `ST_TextFontSize` (100–400,000) | Font size in hundredths of a point; 2400 = 24pt, 1100 = 11pt |
| `b` | boolean | Bold |
| `i` | boolean | Italic |
| `u` | `ST_TextUnderlineType` | Underline: `none`, `sng`, `dbl`, `heavy`, `dotted`, `dottedHeavy`, `dash`, `dashHeavy`, `dashLong`, `dashLongHeavy`, `dotDash`, `dotDashHeavy`, `dotDotDash`, `dotDotDashHeavy`, `wavy`, `wavyHeavy`, `wavyDbl` |
| `strike` | `ST_TextStrikeType` | Strike-through: `noStrike`, `sngStrike`, `dblStrike` |
| `cap` | `ST_TextCapsType` | Capitalisation: `none`, `small`, `all` |
| `kern` | `ST_TextNonNegativePoint` | Kerning threshold in hundredths of a point; below this size kerning is disabled |
| `baseline` | `ST_Percentage` | Superscript/subscript offset as percentage of font size; positive = superscript, negative = subscript |
| `spc` | `ST_TextPoint` | Character spacing adjustment in hundredths of a point |
| `dirty` | boolean | Marks text for spell-check; preserve as-is |
| `err` | boolean | Spell-check error flag; preserve as-is |
| `noProof` | boolean | Exclude from spell-check |
| `bmk` | string | Bookmark name |

**Children of `<a:rPr>`** (in schema order):

- `<a:ln>` — underline line properties
- Fill group — character highlight fill
- `<a:effectLst>` or `<a:effectDag>` — character effects
- `<a:highlight>` — highlight colour (`CT_Color`)
- `<a:uLnTx/>` or `<a:uLn>` — underline line override
- `<a:uFillTx/>` or `<a:uFill>` — underline fill override
- `<a:latin>`, `<a:ea>`, `<a:cs>` — font references
- `<a:sym>` — symbol font
- `<a:hlinkClick>` — hyperlink on click
- `<a:hlinkMouseOver>` — hyperlink on hover
- `<a:rtl/>` — right-to-left override
- `<a:extLst>` — extensions

### 9.7 Font References

Font references appear as `<a:latin>`, `<a:ea>` (East Asian), `<a:cs>`
(complex script), `<a:sym>` (symbol) on `<a:rPr>`, `<a:pPr>`, and theme
font collections. Schema type `CT_TextFont`.

**Attributes:**

| Attribute | Description |
|---|---|
| `typeface` | Font family name or theme font placeholder |
| `panose` | PANOSE-1 font description (optional) |
| `pitchFamily` | Font pitch/family byte (optional, default 0) |
| `charset` | Font charset byte (optional, default 1 = ANSI) |

**Special `typeface` values:**

| Value | Resolved font |
|---|---|
| `+mj-lt` | Major (heading) Latin theme font |
| `+mn-lt` | Minor (body) Latin theme font |
| `+mj-ea` | Major East Asian theme font |
| `+mn-ea` | Minor East Asian theme font |
| `+mj-cs` | Major complex script theme font |
| `+mn-cs` | Minor complex script theme font |
| (empty string) | Same as absent — inherit from parent |

### 9.8 Line Break (`<a:br>`)

A soft line break within a paragraph. Schema type `CT_TextLineBreak`.
Contains an optional `<a:rPr>` (the formatting of the break character itself).

```xml
<a:br>
  <a:rPr lang="en-US" dirty="0"/>
</a:br>
```

### 9.9 Field (`<a:fld>`)

An auto-updating field. Schema type `CT_TextField`.

| Attribute | Description |
|---|---|
| `id` | Globally unique identifier (GUID string, e.g. `{ABCD1234-...}`) |
| `type` | Field type string, e.g. `"slidenum"`, `"datetime"`, `"datetime1"` |

Contains optional `<a:rPr>`, `<a:pPr>`, and `<a:t>` (the last-computed value
of the field, used as a fallback).

### 9.10 End Paragraph Run Properties (`<a:endParaRPr>`)

`<a:endParaRPr>` uses the same `CT_TextCharacterProperties` type as `<a:rPr>`.
It carries the formatting of the "paragraph mark" — the invisible character
at the end of every paragraph. It defines what the next run typed by a user
would look like, and also determines the cursor height when the paragraph is
empty. **Must be preserved.** Its default font size is often the canonical font
size for the paragraph even when there are no text runs.

---

## 10. Theme (`<a:theme>`)

The `<a:theme>` element (schema type `CT_OfficeStyleSheet`) is the root of a
theme part. Theme parts are stored at locations such as:
- `word/theme/theme1.xml` (DOCX)
- `xl/theme/theme1.xml` (XLSX)
- `ppt/theme/theme1.xml` (PPTX)

The root element is declared with `xmlns:a="..."` and an optional `name`
attribute.

```xml
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
         name="Office Theme">
  <a:themeElements>
    ...
  </a:themeElements>
  <a:objectDefaults/>
  <a:extraClrSchemeLst/>
</a:theme>
```

**Children of `<a:theme>` (`CT_OfficeStyleSheet`):**

| Element | Required | Description |
|---|---|---|
| `<a:themeElements>` | Yes | Core theme content |
| `<a:objectDefaults>` | No | Default shape, line, text styles |
| `<a:extraClrSchemeLst>` | No | Alternate colour schemes |
| `<a:custClrLst>` | No | Custom colour list |
| `<a:extLst>` | No | Extensions |

### 10.1 Theme Elements (`<a:themeElements>`)

Schema type `CT_BaseStyles`. Contains three required children:

1. `<a:clrScheme>` — colour scheme
2. `<a:fontScheme>` — font scheme
3. `<a:fmtScheme>` — format scheme (fill, line, effect, background styles)

### 10.2 Colour Scheme (`<a:clrScheme>`)

Schema type `CT_ColorScheme`. Required `name` attribute.

Contains exactly 12 named colour slots, each containing one `EG_ColorChoice`
colour element:

| Slot | Semantic |
|---|---|
| `<a:dk1>` | Dark 1 (typically the primary text colour) |
| `<a:lt1>` | Light 1 (typically the document background) |
| `<a:dk2>` | Dark 2 (secondary dark colour) |
| `<a:lt2>` | Light 2 (secondary light colour) |
| `<a:accent1>` through `<a:accent6>` | Six accent colours |
| `<a:hlink>` | Hyperlink colour |
| `<a:folHlink>` | Followed hyperlink colour |

```xml
<a:clrScheme name="Office">
  <a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1>
  <a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1>
  <a:dk2><a:srgbClr val="44546A"/></a:dk2>
  <a:lt2><a:srgbClr val="E7E6E6"/></a:lt2>
  <a:accent1><a:srgbClr val="4472C4"/></a:accent1>
  <a:accent2><a:srgbClr val="ED7D31"/></a:accent2>
  <a:accent3><a:srgbClr val="A9D18E"/></a:accent3>
  <a:accent4><a:srgbClr val="FFC000"/></a:accent4>
  <a:accent5><a:srgbClr val="5B9BD5"/></a:accent5>
  <a:accent6><a:srgbClr val="70AD47"/></a:accent6>
  <a:hlink><a:srgbClr val="0563C1"/></a:hlink>
  <a:folHlink><a:srgbClr val="954F72"/></a:folHlink>
</a:clrScheme>
```

### 10.3 Font Scheme (`<a:fontScheme>`)

Schema type `CT_FontScheme`. Required `name` attribute.

Contains two required children:

- `<a:majorFont>` — heading/title fonts
- `<a:minorFont>` — body/content fonts

Each is `CT_FontCollection`:

```xml
<a:majorFont>
  <a:latin typeface="Calibri Light"/>
  <a:ea typeface=""/>
  <a:cs typeface=""/>
  <a:font script="Arab" typeface="Times New Roman"/>
  <a:font script="Hebr" typeface="Times New Roman"/>
  <!-- ... more script supplements ... -->
</a:majorFont>
```

`<a:latin>`, `<a:ea>`, `<a:cs>` are required even if empty. The optional
`<a:font script="..." typeface="..."/>` elements provide script-specific
overrides keyed by the four-letter ISO 15924 script code (e.g., `Arab`,
`Hebr`, `Hans`, `Hant`, `Jpan`, `Kore`).

### 10.4 Format Scheme (`<a:fmtScheme>`)

Schema type `CT_StyleMatrix`. Required `name` attribute (may be empty).

Contains four required children, each holding exactly 3 or more style
matrix entries (indexed 1–3 for the subtlest to most emphatic):

| Element | Type | Contents |
|---|---|---|
| `<a:fillStyleLst>` | `CT_FillStyleList` | 3+ fill styles |
| `<a:lnStyleLst>` | `CT_LineStyleList` | 3+ `<a:ln>` elements |
| `<a:effectStyleLst>` | `CT_EffectStyleList` | 3+ `<a:effectStyle>` elements |
| `<a:bgFillStyleLst>` | `CT_BackgroundFillStyleList` | 3+ background fills |

These define the preset style options exposed in shape style galleries.
Index 1 is the least emphatic (often "subtle"), index 3 is most emphatic
(often "intense"). Consumer parsers that do not render style galleries
can store these as opaque XML.

---

## 11. Image Embedding

### 11.1 Relationship Type

Image relationships are declared in the `.rels` file alongside the part that
contains the `<a:blipFill>` (or equivalent):

```xml
<Relationship Id="rId2"
  Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/image"
  Target="../media/image1.png"/>
```

The `Id` value (e.g., `"rId2"`) matches the `r:embed` attribute on `<a:blip>`.

For external links, `TargetMode="External"` and `r:link` is used instead of
`r:embed`.

### 11.2 Supported Image Content Types

Producers and consumers SHOULD support at minimum:

| Content type | Format |
|---|---|
| `image/png` | PNG (lossless raster) |
| `image/jpeg` | JPEG (lossy raster) |
| `image/gif` | GIF |
| `image/tiff` | TIFF |
| `image/bmp` | Windows Bitmap |
| `image/x-wmf` or `image/wmf` | Windows Metafile (vector) |
| `image/x-emf` or `image/emf` | Enhanced Metafile (vector) |
| `image/svg+xml` | SVG |

Content type is declared in `[Content_Types].xml` for the specific image part
(or via a default entry for the extension).

### 11.3 Complete Example — Shape with Embedded Image

This example shows a picture element as it appears in a PowerPoint slide
(`ppt/slides/slide1.xml`):

```xml
<p:pic>
  <p:nvPicPr>
    <p:cNvPr id="2" name="Image 1"/>
    <p:cNvPicPr>
      <a:picLocks noChangeAspect="1"/>
    </p:cNvPicPr>
    <p:nvPr/>
  </p:nvPicPr>
  <p:blipFill>
    <a:blip r:embed="rId2"/>
    <a:stretch>
      <a:fillRect/>
    </a:stretch>
  </p:blipFill>
  <p:spPr>
    <a:xfrm>
      <a:off x="914400" y="914400"/>
      <a:ext cx="2743200" cy="2057400"/>
    </a:xfrm>
    <a:prstGeom prst="rect">
      <a:avLst/>
    </a:prstGeom>
  </p:spPr>
</p:pic>
```

In Word documents (`word/document.xml`), images appear inside
`<w:drawing>/<wp:inline>/<a:graphic>/<a:graphicData
uri="...drawingml/2006/picture">/<pic:pic>` instead.

---

## 12. Common Structures — Annotated Examples

### 12.1 Minimal Text Shape (PPTX slide)

```xml
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="2" name="TextBox 1"/>
    <p:cNvSpPr txBox="1"/>
    <p:nvPr/>
  </p:nvSpPr>
  <p:spPr>
    <a:xfrm>
      <a:off x="914400" y="914400"/>    <!-- 1 inch from top-left -->
      <a:ext cx="2743200" cy="1143000"/> <!-- 3 in × 1.25 in -->
    </a:xfrm>
    <a:prstGeom prst="rect">
      <a:avLst/>
    </a:prstGeom>
    <a:noFill/>
  </p:spPr>
  <p:txBody>
    <a:bodyPr/>          <!-- required even if empty -->
    <a:lstStyle/>        <!-- required even if empty -->
    <a:p>
      <a:r>
        <a:rPr lang="en-US" sz="2400" dirty="0"/>
        <a:t>Hello world</a:t>
      </a:r>
    </a:p>
  </p:txBody>
</p:sp>
```

### 12.2 Styled Shape with Gradient Fill and Shadow

```xml
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="3" name="Shape 2"/>
    <p:cNvSpPr/>
    <p:nvPr/>
  </p:nvSpPr>
  <p:spPr>
    <a:xfrm rot="900000">             <!-- rotated 15 degrees clockwise -->
      <a:off x="1828800" y="914400"/>
      <a:ext cx="3657600" cy="1828800"/>
    </a:xfrm>
    <a:prstGeom prst="roundRect">
      <a:avLst>
        <a:gd name="adj" fmla="val 20000"/>
      </a:avLst>
    </a:prstGeom>
    <a:gradFill rotWithShape="1">
      <a:gsLst>
        <a:gs pos="0">
          <a:schemeClr val="accent1">
            <a:lumMod val="110000"/>
            <a:satMod val="105000"/>
          </a:schemeClr>
        </a:gs>
        <a:gs pos="100000">
          <a:schemeClr val="accent1">
            <a:lumMod val="60000"/>
            <a:satMod val="99000"/>
          </a:schemeClr>
        </a:gs>
      </a:gsLst>
      <a:lin ang="5400000" scaled="0"/>
    </a:gradFill>
    <a:ln w="19050">
      <a:solidFill>
        <a:schemeClr val="accent1">
          <a:lumMod val="75000"/>
        </a:schemeClr>
      </a:solidFill>
    </a:ln>
    <a:effectLst>
      <a:outerShdw blurRad="57150" dist="19050" dir="5400000"
                   algn="ctr" rotWithShape="0">
        <a:srgbClr val="000000">
          <a:alpha val="63000"/>
        </a:srgbClr>
      </a:outerShdw>
    </a:effectLst>
  </p:spPr>
  <p:txBody>
    <a:bodyPr anchor="ctr"/>
    <a:lstStyle/>
    <a:p>
      <a:pPr algn="ctr"/>
      <a:r>
        <a:rPr lang="en-US" sz="1800" b="1" dirty="0">
          <a:solidFill><a:srgbClr val="FFFFFF"/></a:solidFill>
          <a:latin typeface="+mj-lt"/>
        </a:rPr>
        <a:t>Button Label</a:t>
      </a:r>
    </a:p>
  </p:txBody>
</p:sp>
```

---

## 13. Round-Trip Parser Gotchas

This section catalogues the specific numeric encoding and structural
requirements that are most commonly handled incorrectly by parsers.

### 13.1 Numeric Encoding Pitfalls

| Value | Encoding | Common Mistake |
|---|---|---|
| Rotation in `<a:xfrm rot="...">` | 1/60,000 of a degree | Treating it as degrees or radians |
| Colour transform percentages (`lumMod`, `tint`, `shade`, etc.) | Integer, 100,000 = 100% | Off-by-1000 (writing 100 instead of 100000) |
| Line spacing `<a:spcPts val="...">` | 100 = 1 point | Off-by-100 (writing 12 instead of 1200 for 12pt) |
| Font size `<a:rPr sz="...">` | 100 = 1 point (so 2400 = 24pt) | Confusing with half-points (Word's `w:sz`) — DrawingML sz is hundredths-of-a-point, not half-points |
| Gradient stop position `<a:gs pos="...">` | 100,000 = 100% | Writing 100 instead of 100000 |
| `<a:srcRect l="..." ...>` | 100,000 = 100% | Same as above |

**DrawingML `sz` vs WordprocessingML `sz`:** In DrawingML `<a:rPr sz="2400">` means
24pt. In WordprocessingML `<w:sz w:val="48">` also means 24pt because Word's `sz`
is in half-points. These two systems are NOT compatible; never cross-apply them.

### 13.2 Structural Requirements

1. **`<a:bodyPr/>` is required.** A `<a:txBody>` without `<a:bodyPr>` is
   invalid. Write the element even when all attributes use defaults.

2. **`<a:lstStyle/>` is required.** Must be present as the second child of
   `<a:txBody>`, even when empty. Some strict validators reject documents
   that omit it.

3. **`<a:txBody>` must have at least one `<a:p>`.** A text body with no
   paragraph elements is invalid.

4. **`<a:endParaRPr>` must be preserved.** It is not merely decorative — it
   defines the formatting of the paragraph terminus and carries the default
   character properties for the "empty" paragraph slot. Round-trip parsers
   that discard it will alter the effective formatting of documents.

5. **`<a:noFill/>` is not the same as absent fill.** An explicit `<a:noFill/>`
   means "this element has no fill, do not inherit one." Absence of any fill
   child means "inherit from the style or theme." Dropping `<a:noFill/>`
   during round-trip will change the rendered output.

6. **`<a:extLst>` must be passed through opaquely.** The `<a:extLst>` element
   appears as an optional child of almost every DrawingML complex type. Per the
   MCE rules (see `docs/specs/mce.md` §6), the MCE processor must suspend
   namespace enforcement inside `<a:extLst>` / `<a:ext>` children. A parser
   must preserve these elements and all their children verbatim, even when
   the children's namespaces are unknown.

7. **Child element order in `<a:spPr>` is significant.** The XML Schema
   enforces a specific sequence (`xfrm`, then geometry, then fill, then `ln`,
   then effects, then 3D). Reordering these children produces a schema-invalid
   document.

8. **`<a:schemeClr>` and `<a:srgbClr>` are context-dependent.** A
   `<a:schemeClr val="dk1">` does not have a fixed colour value — its actual
   rendered colour depends on the active colour scheme and the `<a:clrMap>`
   mapping on the slide master. Parsers that "resolve" scheme colours to RGB
   during parsing will produce incorrect output on theme changes.

9. **`lang` on `<a:rPr>` is for spell-checking, not display language.**
   The `lang` attribute selects the language for spell/grammar checking.
   The actual display script is inferred from the `typeface` and the characters
   themselves. It is common for `lang="en-US"` to appear on runs containing
   non-Latin characters.

10. **Rotation origin is the shape centre, not the top-left corner.** The
    `<a:off>` coordinates define the top-left of the shape's bounding box
    *before rotation*. After applying `rot`, the bounding box may move, but
    the visual centre of the shape remains at `(off.x + ext.cx/2, off.y + ext.cy/2)`.
    Round-trip fidelity requires storing and re-emitting `off` as given, not
    recomputing it from a rotated bounding box.

---

## 14. Colour Mapping and Theme Resolution

Slides in PresentationML use a colour mapping layer to remap the 12 theme
colour slots. The `<a:clrMap>` element (on slide masters and layouts) carries
12 required attributes whose names are the logical names (`bg1`, `tx1`, etc.)
and whose values are theme slot names (`dk1`, `lt1`, etc.).

For example, `<a:clrMap bg1="lt1" tx1="dk1" .../>` means that wherever a
shape references `schemeClr val="bg1"`, it resolves to the `lt1` slot in the
colour scheme.

This indirection allows the same slide content to change its visual palette
by simply changing the slide master's colour mapping, without editing individual
shapes. Parsers that perform colour resolution at parse time must be aware of
this layer.

Slides can also carry a `<p:clrMapOvr>` that either defers to the master
(`<a:masterClrMapping/>`) or provides a full override (`<a:overrideClrMapping>`).

---

## 15. Relationship Summary

| Relationship type URI | Purpose |
|---|---|
| `.../relationships/image` | Embedded image (used by `r:embed` on `<a:blip>`) |
| `.../relationships/theme` | Theme part (from document to `word/theme/theme1.xml` etc.) |
| `.../relationships/slideLayout` | Slide to its layout (PPTX) |
| `.../relationships/slideMaster` | Layout to its master (PPTX) |
| `.../relationships/hyperlink` | External hyperlink (used by `r:id` on `<a:hlinkClick>`) |

---

*End of DrawingML Shared Model Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
