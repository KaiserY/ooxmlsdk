# PresentationML Tables — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §20.1.3 (DrawingML Table);
§19.3.1.21 (`p:graphicFrame`); §20.1.3.2 (`a:tbl`); §20.1.3.11 (`a:tc`);
ISO/IEC 29500:2016 Part 1; XSD `dml-main.xsd`, `pml.xsd`.

Tables in PPTX are DrawingML tables embedded in a `<p:graphicFrame>`. The
`<a:tbl>` element belongs to the `drawingml/2006/main` namespace and is
identical in structure whether used inside PPTX, DOCX inline drawings, or
XLSX drawings.

---

## 1. Container Structure

```xml
<p:graphicFrame>
  <p:nvGraphicFramePr>
    <p:cNvPr id="2" name="Table 1"/>
    <p:cNvGraphicFramePr>
      <a:graphicFrameLocks noGrp="1"/>
    </p:cNvGraphicFramePr>
    <p:nvPr/>
  </p:nvGraphicFramePr>
  <p:xfrm>
    <a:off x="457200" y="457200"/>
    <a:ext cx="8229600" cy="1714500"/>
  </p:xfrm>
  <a:graphic>
    <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/table">
      <a:tbl>…</a:tbl>
    </a:graphicData>
  </a:graphic>
</p:graphicFrame>
```

The `uri` on `<a:graphicData>` must be exactly
`http://schemas.openxmlformats.org/drawingml/2006/table`.

---

## 2. `<a:tbl>` — Table Root

```xml
<a:tbl>
  <a:tblPr firstRow="1" lastRow="0" firstCol="0" lastCol="0"
           bandRow="1" bandCol="0">
    <a:tableStyleId>{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}</a:tableStyleId>
  </a:tblPr>
  <a:tblGrid>
    <a:gridCol w="2743200"/>
    <a:gridCol w="2743200"/>
    <a:gridCol w="2743200"/>
  </a:tblGrid>
  <a:tr h="571500">…</a:tr>
  <a:tr h="571500">…</a:tr>
</a:tbl>
```

### `<a:tblPr>` — Table properties

| Attribute | Meaning |
|-----------|---------|
| `firstRow` | Apply first-row style (header formatting) |
| `lastRow` | Apply last-row style (totals row) |
| `firstCol` | Apply first-column style |
| `lastCol` | Apply last-column style |
| `bandRow` | Alternating row-band shading |
| `bandCol` | Alternating column-band shading |

`<a:tableStyleId>` references a built-in GUID table style, e.g.
`{5C22544A-7EE6-4342-B048-85BDC9FD1C3A}` ("Medium Style 2 – Accent 1").
May be omitted for a plain unstyled table; `<a:tblPr/>` itself must be
present.

### `<a:tblGrid>` — Column definitions

One `<a:gridCol w>` per logical column, `w` in EMU. The sum of all widths
should equal the frame's `cx`.

---

## 3. `<a:tr>` and `<a:tc>` — Rows and Cells

```xml
<a:tr h="571500">
  <a:tc>
    <a:txBody>
      <a:bodyPr/>
      <a:lstStyle/>
      <a:p><a:r><a:t>Header A</a:t></a:r></a:p>
    </a:txBody>
    <a:tcPr>
      <a:lnB w="12700" cap="flat" cmpd="sng">
        <a:solidFill><a:schemeClr val="accent1"/></a:solidFill>
        <a:prstDash val="solid"/>
      </a:lnB>
    </a:tcPr>
  </a:tc>
  <!-- merged leading cell: spans 2 columns -->
  <a:tc gridSpan="2">
    <a:txBody>
      <a:bodyPr/>
      <a:lstStyle/>
      <a:p><a:r><a:t>Merged B+C</a:t></a:r></a:p>
    </a:txBody>
    <a:tcPr/>
  </a:tc>
  <!-- horizontal merge continuation cell -->
  <a:tc hMerge="1">
    <a:txBody><a:bodyPr/><a:lstStyle/><a:p/></a:txBody>
    <a:tcPr/>
  </a:tc>
</a:tr>
```

### `<a:tr h>` — Row height

`h` specifies the row height in EMU.

### `<a:tc>` — Cell attributes for merged cells

| Attribute | Meaning |
|-----------|---------|
| `gridSpan` | Number of grid columns this cell spans (horizontal merge, integer ≥ 2) |
| `rowSpan` | Number of rows this cell spans (vertical merge, integer ≥ 2) |
| `hMerge="1"` | This cell is a horizontal-merge continuation (content in the leading cell) |
| `vMerge="1"` | This cell is a vertical-merge continuation |

The **leading cell** carries the content and the `gridSpan` / `rowSpan`
count. Each **continuation cell** must have `hMerge="1"` or `vMerge="1"`,
an empty `<a:txBody>` (with `<a:bodyPr/>`, `<a:lstStyle/>`, and `<a:p/>`),
and an empty `<a:tcPr/>`.

### `<a:tcPr>` — Cell properties

| Child | Role |
|-------|------|
| `<a:lnL>` / `<a:lnR>` / `<a:lnT>` / `<a:lnB>` | Border lines (left/right/top/bottom) |
| `<a:lnTlToBr>` / `<a:lnBlToTr>` | Diagonal lines |
| `<a:solidFill>` | Cell background fill |
| `<a:noFill/>` | Explicit transparent fill |
| `anchor` attr | Vertical text anchor: `t` / `ctr` / `b` |
| `marL` / `marR` / `marT` / `marB` attrs | Cell text margins in EMU |

Border line elements share the `<a:ln>` schema: `w` (EMU width), fill
child (`<a:solidFill>` etc.), and `<a:prstDash val>`.

---

## 4. SDK Access Pattern

```rust
let slide = pres.slide_parts(&package).next()?;
let sp_tree = &slide.root(&package).c_sld.sp_tree;
for frame in &sp_tree.graphic_frame {
  // frame.graphic.graphic_data.xml_children holds the raw <a:tbl> bytes
}
```

Rust types under
`schemas_openxmlformats_org_presentationml_2006_main`:
`GraphicFrame`, `NonVisualGraphicFrameProperties`.

`<a:tbl>` content inside `<a:graphicData>` is stored as raw XML in the
any-payload field (`xml_children: Vec<Box<str>>`).

---

## 5. Round-Trip Gotchas

1. **Continuation cells require a non-empty `<a:txBody>`.** An `<a:tc
   hMerge="1">` without `<a:bodyPr/>`, `<a:lstStyle/>`, and `<a:p/>` is
   schema-invalid.

2. **`gridSpan` + continuations must sum to the column count per row.**
   A 3-column row with one `gridSpan="2"` cell needs exactly 3 `<a:tc>`
   entries: the leading, its one continuation, and the third column's cell.

3. **The `uri` on `<a:graphicData>` is case-sensitive and exact.** Any
   variation makes the table invisible to conforming readers.

4. **`<a:tblGrid>` widths should sum to the frame's `cx`.** Mismatched
   totals cause the last column to overflow or truncate.

5. **`<a:tableStyleId>` braces are mandatory.** The GUID value must include
   enclosing `{` and `}`: `{5C22544A-…}`.

6. **`<a:tblPr/>` is required even for an unstyled table.** Omit attributes
   freely, but the element itself must be present.

---

## 6. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| PML-TBL-01 | `test-data/slideshow/minimal_table.pptx` | Basic 2×2 table (already covered) |
| PML-TBL-02 | `test-data/pml/slide_table.pptx` | 3-column 3-row table: `firstRow="1"` `bandRow="1"` on `<a:tblPr>`; `<a:tcPr>` bottom-border on header cells; horizontal merge spanning cols 2–3 (`gridSpan="2"` + `hMerge="1"` continuation) |
