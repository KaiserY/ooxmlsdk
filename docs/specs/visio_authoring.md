# Visio sheet authoring

**Purpose:** How `ooxmlsdk-visio` writes stored ShapeSheet values, geometry,
text, images, groups, connects, layers, comments, hyperlinks, and shape data.
A later PPTX conversion can call these operations. This note does not define
that conversion, a formula engine, or a PDF renderer.

**Status:** This is the contract for the next work in `ooxmlsdk-visio`. The
readers in `visio_shapesheet.md` and the package in `visio_parts.md` are
already implemented. The writes below are not.

**Sources used:** `schemas/VisioXMLSchema/visio-main.xsd` and the generated
types in `schemas_microsoft_com_office_visio_2012_main`. Examples are original.

Cell names below are the `N` values from that XSD. `xsd:alternative` entries
that only assert `@N` or `@T` stay `Cell` and `Row` values. They do not become
separate Rust types. Generated `LayerRow` and `HyperlinkRow` structs are not
referenced by the document model. This layer addresses `Section` and `Row`.

## What already exists

`visio_shapesheet.md` covers reading stored `V`, master and style inheritance,
text marks, and resolving connects. `Ids` allocates page, master, and shape
ids. `minimal_drawing` builds one page and one shape. Those stay.

## Direct cells

A direct cell is a `Cell` child of a shape or a page sheet. `PinX`, `PageWidth`,
`LayerMember`, and `BeginX` are direct cells. `shape_cell` keeps looking only
at sections.

Reading a direct cell returns its stored `V`, and `U`, `F`, and `E` when
present. Writing `V` updates that attribute. An existing `F` stays unless the
caller also sets `F`. A new cell omits `F`. Missing cells are appended. Other
children stay in place.

Helpers write these direct cells by name. Callers pass the stored text. This
layer does not convert EMU, flip a Y axis, or resolve a theme color.

| Concern | Cell names |
| --- | --- |
| Placement | `PinX`, `PinY`, `Width`, `Height`, `LocPinX`, `LocPinY`, `Angle`, `FlipX`, `FlipY` |
| Line and fill | `LineWeight`, `LineColor`, `LineColorTrans`, `FillForegnd`, `FillForegndTrans` |
| One-dimensional ends | `BeginX`, `BeginY`, `EndX`, `EndY` |
| Page sheet | `PageWidth`, `PageHeight`, `PageScale`, `DrawingScale` |
| Image box | `ImgOffsetX`, `ImgOffsetY`, `ImgWidth`, `ImgHeight`, `ClippingPath` |
| Layer membership | `LayerMember` |

Any other direct `N` from `ShapeSheet_Type` uses the same read and write.
The helpers above are the names this layer adds first.

## Section cells

A section cell is a `Cell` inside a `Row` inside a `Section`. The section is
addressed by `N`, and by `IX` when the file has one. A row is addressed by
`IX`, or by row `N` when the row is named.

Reading returns the stored cell. Writing `V` follows the same `F` rule as a
direct cell. A missing section or row is created.

## Geometry

The section name is `Geometry`. A second geometry section on the same shape
uses another `IX`. Each row sets `T` and the cells for that `T`.

| `T` | Cells |
| --- | --- |
| `MoveTo`, `LineTo`, `RelMoveTo`, `RelLineTo` | `X`, `Y` |
| `ArcTo`, `SplineKnot`, `PolylineTo` | `X`, `Y`, `A` |
| `InfiniteLine`, `RelQuadBezTo` | `X`, `Y`, `A`, `B` |
| `Ellipse`, `EllipticalArcTo`, `RelEllipticalArcTo`, `SplineStart`, `RelCubBezTo` | `X`, `Y`, `A`, `B`, `C`, `D` |
| `NURBSTo` | `X`, `Y`, `A`, `B`, `C`, `D`, `E` |

Section cells `NoFill`, `NoLine`, `NoShow`, `NoSnap`, `NoQuickDrag`, and
`Path` sit on the geometry section itself, not on a row.

Appending a row assigns the next `IX` in that section.

## Text

The published `Text_Type` is a choice of `cp`, `pp`, `tp`, and `fld` only.
Drawing files also store character data between those marks:

```xml
<Text><cp IX="0"/><pp IX="0"/>Hello</Text>
```

This layer keeps that character data in document order with the marks. Reading
returns both. Writing replaces the `Text` child and leaves other shape
children, such as `PinX`, in place. `fld` still carries its own text.

Character formatting is section `Character`. Rows are indexed. Cells are
`Font`, `Color`, `Style`, `Case`, `Pos`, `FontScale`, `Size`, `DblUnderline`,
`Overline`, `Strikethru`, `DoubleStrikethrough`, `Letterspace`, `ColorTrans`,
`AsianFont`, `ComplexScriptFont`, `ComplexScriptSize`, and `LangID`.

Paragraph formatting is section `Paragraph`. Cells are `IndFirst`, `IndLeft`,
`IndRight`, `SpLine`, `SpBefore`, `SpAfter`, `HorzAlign`, `Bullet`,
`BulletStr`, `BulletFont`, `BulletFontSize`, `TextPosAfterBullet`, and
`Flags`.

`cp` and `pp` store the row index. This layer does not invent a row the
caller did not write.

## Pages

`PageWidth` and `PageHeight` are direct cells on the page sheet. Adding a
page appends a `Page` to the pages part and a page part for its contents.
`Ids` supplies the page id. `minimal_drawing` remains the one-page example.

## Images

A page part already owns image parts. The shape stores `ForeignData` with
`ForeignType` and the extent attributes from the XSD, plus a `Rel` child.

`Rel_Type` has no attributes in the published schema. Files identify the
image with the package relationship id on `Rel`. The writer stores the bytes
on an image part of the same page and emits that id on `Rel`. The image box
is the direct cells `ImgOffsetX`, `ImgOffsetY`, `ImgWidth`, and `ImgHeight`.

## Groups

A shape already has an optional `Shapes` child. This layer appends a child
shape there and takes the child id from `Ids`. The caller sets `Type` when
the shape is a group. Child placement is ordinary direct cells.

## Connects

`resolve_connects` still checks that `FromSheet` and `ToSheet` are shape ids
on the same page. Writing adds a `Connect` with those two ids. `FromCell`,
`FromPart`, `ToCell`, and `ToPart` are stored when the caller sets them. The
one-dimensional ends are the direct cells `BeginX`, `BeginY`, `EndX`, and
`EndY`. This layer does not compute glue.

## Layers

The page sheet section name is `Layer`. Each row has `IX`. Cells are `Name`,
`Color`, `Status`, `Visible`, `Print`, `Active`, `Lock`, `Snap`, `Glue`,
`NameUniv`, and `ColorTrans`.

A shape's membership is the direct cell `LayerMember`. This layer reads and
writes that stored value. It does not decide which layers are visible.

## Comments

Comments live in the generated comments part, not in the shape sheet. An
author row has `ID`, and may have `Name`, `Initials`, `SIP`, `SMTP`, and
`ResolutionID`.

A comment has `AuthorID`, `PageID`, `CommentID`, `Date`, and text. It may
have `ShapeID`, `EditDate`, `Done`, and `AutoCommentType`. This layer lists
and appends those records. It does not import PowerPoint comments.

## Hyperlinks

The section name is `Hyperlink`. Each row has `N`. Cells are `Description`,
`Address`, `SubAddress`, `ExtraInfo`, `Frame`, `NewWindow`, `Default`,
`Invisible`, and `SortKey`.

## Shape data

The section name is `Property`. Each row has `N`. Cells are `Value`, `Prompt`,
`Label`, `Format`, `SortKey`, `Type`, `Invisible`, `Verify`, `DataLinked`,
`LangID`, and `Calendar`.

## Tests

Each write has a round trip through the generated XML in `ooxmlsdk-visio`
tests: set the value, serialize, parse, and read the same stored fields.
Opening the file in Visio is a later check.

## Out of scope

- A function that reads PPTX and writes VSDX.
- Converting slide units, theme colors, tables, charts, SmartArt, media, or animation.
- Evaluating `F`, including `THEMEVAL`.
- Drawing Visio to PDF.
- Legacy `.vsd` files.
