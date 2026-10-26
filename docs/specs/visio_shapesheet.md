# Visio ShapeSheet cells, text, and connects

**Purpose:** How `ooxmlsdk-visio` reads the sheet that the generated XML types
store. The effective value is the saved `V` attribute. This note does not
define a formula engine or a PDF renderer.

**Sources used:** `schemas/VisioXMLSchema/visio-main.xsd` and the generated
types in `schemas_microsoft_com_office_visio_2012_main`. Examples are original.

## Cell and section

A sheet cell is one `Cell` element. The attributes this layer reads are:

| Attribute | Meaning here |
| --- | --- |
| `N` | Cell name, such as `PinX` or `X` |
| `V` | Stored value. This is the effective value. |
| `F` | Formula text. It is not evaluated. |
| `U` | Unit token, such as `IN` |
| `E` | Stored error text |

`xsd:alternative` entries that only assert `@N` or `@T` do not become Rust
types. `PinX` stays a `Cell`. Alternatives that add rows are folded into the
`Section` element the file actually contains. That section holds direct cells
and one row shape. The row can carry `Del`, `N`, `LocalName`, `IX`, `T`,
repeated cells, and `Tab`.

A geometry line looks like this:

```xml
<Section N="Geometry">
  <Cell N="NoFill" V="0"/>
  <Row T="LineTo" IX="1">
    <Cell N="X" V="1" F="Width*0.5" U="IN"/>
    <Cell N="Y" V="2"/>
  </Row>
</Section>
```

`Width*0.5` stays a string. The stored value remains `1`.

## Identity

`Ids` allocates page, master, and shape ids on separate counters, each
starting at 1. Lookup is by those ids:

- `Pages` / `Page/@ID`
- `Masters` / `Master/@ID`
- `Shapes` / `Shape/@ID`, including shapes nested under another shape

## Inheritance

An instance shape points at a master with `Master` and at a shape inside that
master with `MasterShape`.

- If the instance has the cell, that cell's stored `V` wins.
- If the instance does not have the cell, the master shape's stored `V` is used.
- `Del="1"` on the section, or on the row that owns the cell, blocks inheritance
  for that section or row.
- The formula text is copied through with the cell. It is not recalculated.

## Styles

`LineStyle`, `FillStyle`, and `TextStyle` are style sheet ids on the document.
If the shape does not have the requested section cell, the cell is read from
that style sheet. A local cell wins. A formula such as
`THEMEVAL("AccentColor")` is returned as text next to the stored `V`.

## Text

`Text` children are read in order and written back in that order:

- `cp` is a character-row index
- `pp` is a paragraph-row index
- `tp` is a tabs-row index
- `fld` is a field index plus its text

Other shape children, such as `PinX`, stay in place when text is replaced.

## Connects

Each `Connect` on a page names `FromSheet` and `ToSheet`. Both must be shape
ids on that same page, including nested shapes. A missing id is an error.
`FromCell` and `ToCell` are carried as stored names.

## Out of scope

Formula evaluation, dependency recalculation, theme function results, and PDF
rendering are not part of this layer. Writes for cells, geometry, text,
images, groups, layers, comments, hyperlinks, and shape data are specified in
`visio_authoring.md`.
