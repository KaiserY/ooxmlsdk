# Charts (DrawingML Charts) — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §21.2 (DrawingML — Chart); §15.2.10 (Chart Part); ISO/IEC 29500:2016 Part 1; XSD `dml-chart.xsd`.

Shared between SpreadsheetML (XLSX) and PresentationML (PPTX).
WordprocessingML uses the same chart payload via an inline drawing.

---

## 1. Overview

A chart is a separate part (`ChartPart`) holding a
`<c:chartSpace>` document. It is referenced from the *host* part
(worksheet/slide/document) through a `DrawingsPart` for XLSX, an inline
`<w:drawing>` for DOCX, and a slide-level `<p:graphicFrame>` for PPTX.

Two layers cooperate:

| Layer | Role |
|-------|------|
| Drawing (anchor in host part) | Position and size on the page |
| `chart1.xml` (Chart part) | The data, axes, series, legend, formatting |

---

## 2. Part Graph (XLSX example)

```
xl/worksheets/sheet1.xml
xl/worksheets/_rels/sheet1.xml.rels  ─── rId1 drawing ──▶ ../drawings/drawing1.xml
xl/drawings/drawing1.xml
xl/drawings/_rels/drawing1.xml.rels  ─── rId1 chart   ──▶ ../charts/chart1.xml
xl/charts/chart1.xml
```

[Content_Types]:

```xml
<Override PartName="/xl/drawings/drawing1.xml"
  ContentType="application/vnd.openxmlformats-officedocument.drawing+xml"/>
<Override PartName="/xl/charts/chart1.xml"
  ContentType="application/vnd.openxmlformats-officedocument.drawingml.chart+xml"/>
```

DrawingsPart references the chart through a `chart` relationship; the
worksheet references the drawing through a `drawing` relationship.

---

## 3. Worksheet Anchor

The worksheet adds a `<x:drawing>` element after `<x:sheetData>`:

```xml
<x:drawing r:id="rId1"/>
```

`xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"`
must be declared on `<x:worksheet>`.

---

## 4. Drawing Part — chart anchor

```xml
<xdr:wsDr xmlns:xdr="http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing"
          xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
          xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <xdr:twoCellAnchor>
    <xdr:from><xdr:col>3</xdr:col><xdr:colOff>0</xdr:colOff>
              <xdr:row>0</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:from>
    <xdr:to><xdr:col>10</xdr:col><xdr:colOff>0</xdr:colOff>
            <xdr:row>15</xdr:row><xdr:rowOff>0</xdr:rowOff></xdr:to>
    <xdr:graphicFrame macro="">
      <xdr:nvGraphicFramePr>
        <xdr:cNvPr id="2" name="Chart 1"/>
        <xdr:cNvGraphicFramePr/>
      </xdr:nvGraphicFramePr>
      <xdr:xfrm><a:off x="0" y="0"/><a:ext cx="0" cy="0"/></xdr:xfrm>
      <a:graphic>
        <a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
          <c:chart xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
                   r:id="rId1"/>
        </a:graphicData>
      </a:graphic>
    </xdr:graphicFrame>
    <xdr:clientData/>
  </xdr:twoCellAnchor>
</xdr:wsDr>
```

`twoCellAnchor` ties the chart to a from/to cell-range pair. The
alternatives are `oneCellAnchor` (anchor at a cell, fixed cx/cy) and
`absoluteAnchor` (anchor in EMU, ignoring rows/cols).

`<a:graphicData uri>` must be exactly
`http://schemas.openxmlformats.org/drawingml/2006/chart`. The `uri`
namespace is what tells consumers the inner element is a chart pointer.

`<c:chart r:id>` references the ChartPart from the DrawingsPart's
relationships.

---

## 5. Chart Part — `chart1.xml`

### Top-level structure

```xml
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart"
              xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
              xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <c:chart>
    <c:title>?
    <c:autoTitleDeleted val="0"/>?
    <c:plotArea>…</c:plotArea>
    <c:legend>?
    <c:plotVisOnly val="1"/>
    <c:dispBlanksAs val="gap"/>?
  </c:chart>
  <c:externalData r:id="…"/>?
</c:chartSpace>
```

### plotArea

```xml
<c:plotArea>
  <c:layout/>?
  <!-- one or more chart-type elements -->
  <c:barChart>
    <c:barDir val="col"/>           <!-- col | bar -->
    <c:grouping val="clustered"/>   <!-- clustered | stacked | percentStacked | standard -->
    <c:varyColors val="0"/>?
    <c:ser>…</c:ser>+               <!-- one or more series -->
    <c:axId val="111"/>             <!-- ids must match c:catAx/c:valAx -->
    <c:axId val="222"/>
  </c:barChart>
  <c:catAx>
    <c:axId val="111"/>
    <c:scaling><c:orientation val="minMax"/></c:scaling>
    <c:delete val="0"/>
    <c:axPos val="b"/>
    <c:crossAx val="222"/>
  </c:catAx>
  <c:valAx>
    <c:axId val="222"/>
    <c:scaling><c:orientation val="minMax"/></c:scaling>
    <c:delete val="0"/>
    <c:axPos val="l"/>
    <c:crossAx val="111"/>
  </c:valAx>
</c:plotArea>
```

Chart-type elements in ECMA-376:

| Element | Type |
|---------|------|
| `<c:barChart>` | Bar / column |
| `<c:lineChart>` | Line |
| `<c:pieChart>`, `<c:doughnutChart>`, `<c:ofPieChart>` | Pie family |
| `<c:areaChart>`, `<c:area3DChart>` | Area |
| `<c:scatterChart>` | XY scatter |
| `<c:bubbleChart>` | Bubble |
| `<c:radarChart>` | Radar |
| `<c:stockChart>` | Stock |
| `<c:surface3DChart>`, `<c:surfaceChart>` | Surface |
| `<c:bar3DChart>`, `<c:line3DChart>`, `<c:pie3DChart>` | 3-D variants |

Multiple chart-type elements may coexist (combo charts). They share
axes via `<c:axId>` references.

### Series — `<c:ser>`

```xml
<c:ser>
  <c:idx val="0"/>
  <c:order val="0"/>
  <c:tx>
    <c:strRef>
      <c:f>Sheet1!$B$1</c:f>
      <c:strCache><c:ptCount val="1"/><c:pt idx="0"><c:v>Series 1</c:v></c:pt></c:strCache>
    </c:strRef>
  </c:tx>
  <c:cat>
    <c:strRef>
      <c:f>Sheet1!$A$2:$A$4</c:f>
      <c:strCache>
        <c:ptCount val="3"/>
        <c:pt idx="0"><c:v>Q1</c:v></c:pt>
        <c:pt idx="1"><c:v>Q2</c:v></c:pt>
        <c:pt idx="2"><c:v>Q3</c:v></c:pt>
      </c:strCache>
    </c:strRef>
  </c:cat>
  <c:val>
    <c:numRef>
      <c:f>Sheet1!$B$2:$B$4</c:f>
      <c:numCache>
        <c:formatCode>General</c:formatCode>
        <c:ptCount val="3"/>
        <c:pt idx="0"><c:v>10</c:v></c:pt>
        <c:pt idx="1"><c:v>20</c:v></c:pt>
        <c:pt idx="2"><c:v>30</c:v></c:pt>
      </c:numCache>
    </c:numRef>
  </c:val>
</c:ser>
```

Cached values inside `<c:strCache>` / `<c:numCache>` are required for
charts that should render without re-evaluating formulas. Round-trip
software should preserve them verbatim — recomputing from the referenced
range is what charting software does, but it is not the SDK's job.

`<c:idx>` is the visual stacking order; `<c:order>` is the legend order.
They are usually equal but can differ.

---

## 6. Axes

```xml
<c:catAx>           <!-- category axis -->
  <c:axId val="111"/>
  <c:scaling><c:orientation val="minMax"/></c:scaling>
  <c:delete val="0"/>             <!-- 0 = visible, 1 = hidden -->
  <c:axPos val="b"/>              <!-- t / b / l / r -->
  <c:crossAx val="222"/>          <!-- the axId of the perpendicular axis -->
  <c:crosses val="autoZero"/>?
  <c:auto val="1"/>?
  <c:lblAlgn val="ctr"/>?
  <c:lblOffset val="100"/>?
</c:catAx>
```

Every chart-type element with axes must list two `<c:axId>` and there
must be matching `<c:catAx>` and `<c:valAx>` (or `<c:dateAx>`, `<c:serAx>`)
under `<c:plotArea>`.

Pie family charts have no axes.

---

## 7. SDK Access Pattern

```rust
let workbook = package.workbook_part()?;
let sheet1 = workbook.worksheet_parts(&package).next()?;
let drawings = sheet1.drawings_part(&package)?;
let chart_part = drawings.chart_parts(&package).next()?;
let chart_space: &ChartSpace = chart_part.root(&package);

if let Some(plot) = chart_space.chart.plot_area.as_ref() {
  for bar in &plot.bar_chart {
    for ser in &bar.ser {
      let title = ser.tx.as_ref()
        .and_then(|t| t.str_ref.as_ref())
        .and_then(|s| s.str_cache.as_ref())
        .map(|c| &c.pt[0].v);
    }
  }
}
```

Rust types live under
`crates/ooxmlsdk/src/schemas/schemas_openxmlformats_org_drawingml_2006_chart.rs`
(`ChartSpace`, `Chart`, `PlotArea`, `BarChart`, `LineChart`, `Series`,
`CategoryAxis`, `ValueAxis`, etc.).

---

## 8. Round-Trip Gotchas

1. **`<c:axId>` must match across chart-type and axis elements.** Bar
   chart with `<c:axId val="111"/>` requires a `<c:catAx>` containing the
   same `axId`. A typo silently breaks the chart's axis binding.

2. **`<a:graphicData uri>` is *exact-match*.** The value must be the
   literal `http://schemas.openxmlformats.org/drawingml/2006/chart`.
   Any normalisation (trailing slash, scheme change) breaks consumer
   recognition.

3. **`<c:strRef>` requires both `<c:f>` *and* a cache.** Excel populates
   the cache when computing the chart; round-tripping a chart with a
   reference but no cache is legal but produces an empty rendering until
   the workbook is reopened with a calculation engine. Preserve caches
   when present.

4. **`<c:plotVisOnly val="1"/>` is a chart-level toggle.** Do not omit
   it on round-trip; some consumers default it to 0 (show hidden cells)
   when it's missing, which subtly changes the rendering.

5. **Cached numerical values in `<c:numCache>` must use `<c:formatCode>`.**
   Without it, Excel renders the cache values with no formatting (raw
   doubles). Inherit from the referenced cell's number format if present.

6. **Combo charts share `<c:catAx>` / `<c:valAx>` definitions.** Two
   chart-type elements (e.g., `<c:barChart>` + `<c:lineChart>`) may both
   reference the same `<c:axId>` pair. Do not duplicate axis definitions
   per chart-type.

7. **`<c:smooth>` on line series defaults to false.** ECMA-376 specifies
   that the default is 0 (no smoothing). Re-emitting `<c:smooth val="0"/>`
   on round-trip is verbose but valid; stripping it is also valid.
   Round-trip code should preserve the original presence/absence.

8. **PPTX uses `<p:graphicFrame>` not `<xdr:graphicFrame>`.** The slide
   layer wraps the chart in a `<p:graphicFrame>` with the same
   `<a:graphicData uri>`. Cross-format round-trip code that swaps
   prefixes will produce a slide that fails to open.

9. **Chart relationships originate from the DrawingsPart, not the
   worksheet.** A common mistake is to add a `chart` relationship to
   `xl/_rels/workbook.xml.rels`. The chart is owned by the drawing, not
   the workbook.

---

## 9. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| CHART-01 | `test-data/spreadsheet/chart_bar.xlsx` | Worksheet with a `<x:drawing>` ref; `xl/drawings/drawing1.xml` `twoCellAnchor` graphicFrame referencing `<c:chart r:id>`; `xl/charts/chart1.xml` `<c:chartSpace>` with `<c:barChart barDir="col" grouping="clustered">` containing one `<c:ser>` with `<c:tx>` `strRef`, `<c:cat>` `strRef` (3 categories cached), `<c:val>` `numRef` (3 numerical values cached); `<c:catAx>`/`<c:valAx>` pair sharing `axId` |
