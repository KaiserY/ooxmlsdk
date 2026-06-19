# SpreadsheetML Conditional Formatting — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §18.3.1.18 (conditionalFormatting), §18.3.1.10 (cfRule), §18.8.45 (dxf differential format); ISO/IEC 29500:2016 Part 1; XSDs `sml.xsd`, `sml-styles.xsd`.

---

## 1. Overview

Conditional formatting applies a **differential format (`<x:dxf>`)** to
cells in a sheet range when a rule evaluates true. A worksheet may contain
zero or more `<x:conditionalFormatting>` blocks; each block is bound to a
range (`sqref`) and lists one or more `<x:cfRule>` entries.

The differential formats themselves live in `xl/styles.xml` inside
`<x:dxfs>`. Each rule references one by 0-based `dxfId`. Some rule types
(colorScale, dataBar, iconSet) supply their own format inline and ignore
`dxfId`.

---

## 2. Worksheet Markup

### Position in the worksheet child order

```
<x:worksheet>
  <x:sheetPr/>?
  <x:dimension/>?
  <x:sheetViews/>?
  <x:sheetFormatPr/>?
  <x:cols/>?
  <x:sheetData/>
  <x:sheetCalcPr/>?
  <x:sheetProtection/>?
  <x:autoFilter/>?
  <x:sortState/>?
  <x:dataConsolidate/>?
  <x:customSheetViews/>?
  <x:mergeCells/>?
  <x:phoneticPr/>?
  <x:conditionalFormatting/>*    ← here, after mergeCells
  <x:dataValidations/>?
  …
</x:worksheet>
```

### conditionalFormatting block

```xml
<x:conditionalFormatting sqref="A1:A10 C5:C8" pivot="0">
  <x:cfRule type="cellIs" priority="1" dxfId="0" operator="greaterThan">
    <x:formula>10</x:formula>
  </x:cfRule>
  <x:cfRule type="expression" priority="2" dxfId="1" stopIfTrue="0">
    <x:formula>MOD(ROW(),2)=0</x:formula>
  </x:cfRule>
</x:conditionalFormatting>
```

Attributes on `<x:conditionalFormatting>`:

| Attr | Type | Default | Meaning |
|------|------|---------|---------|
| `sqref` | space-separated A1 ranges | required | Cells the rules apply to |
| `pivot` | bool | 0 | True if the formatting is bound to a pivot table |

---

## 3. cfRule

Common attributes on every rule type:

| Attr | Meaning |
|------|---------|
| `type` | Rule kind (table below) |
| `priority` | Lower = higher priority; integer ≥ 1 |
| `stopIfTrue` | Stop further rule evaluation at this cell when true (default 0) |
| `dxfId` | 0-based reference into `<x:dxfs>`; required for `cellIs`, `expression`, `containsText`, etc.; absent for visualisation rules |

### Rule type taxonomy

| `type` | Extra attributes | Children | Notes |
|--------|------------------|----------|-------|
| `cellIs` | `operator` | one or two `<x:formula>` (per operator) | The cell-comparison rule |
| `expression` | — | one `<x:formula>` (Boolean expression) | True ⇒ apply format |
| `colorScale` | — | one `<x:colorScale>` | 2- or 3-stop colour scale |
| `dataBar` | — | one `<x:dataBar>` | In-cell horizontal bar |
| `iconSet` | — | one `<x:iconSet>` | 3/4/5-icon visualisation |
| `top10` | `bottom`, `percent`, `rank` | — | Top/bottom N (or N percent) |
| `aboveAverage` | `aboveAverage`, `equalAverage`, `stdDev` | — | Statistical |
| `duplicateValues` | — | — | |
| `uniqueValues` | — | — | |
| `containsText` | `operator`, `text` | `<x:formula>` (the search expression) | |
| `notContainsText` | as above | | |
| `beginsWith` | as above | | |
| `endsWith` | as above | | |
| `containsBlanks` | — | — | |
| `notContainsBlanks` | — | — | |
| `containsErrors` | — | — | |
| `notContainsErrors` | — | — | |
| `timePeriod` | `timePeriod` (`today`, `yesterday`, `last7Days`, `lastWeek`, `thisWeek`, `nextWeek`, `lastMonth`, `thisMonth`, `nextMonth`) | `<x:formula>` | |

### `operator` values for `cellIs` / text rules

`lessThan`, `lessThanOrEqual`, `equal`, `notEqual`, `greaterThanOrEqual`,
`greaterThan`, `between`, `notBetween`, `containsText`, `notContains`,
`beginsWith`, `endsWith`.

`between` / `notBetween` require **two** `<x:formula>` children (low,
high).

---

## 4. Visualisation rule children

### colorScale (2- or 3-stop)

```xml
<x:cfRule type="colorScale" priority="1">
  <x:colorScale>
    <x:cfvo type="min"/>
    <x:cfvo type="percentile" val="50"/>
    <x:cfvo type="max"/>
    <x:color rgb="FFF8696B"/>
    <x:color rgb="FFFFEB84"/>
    <x:color rgb="FF63BE7B"/>
  </x:colorScale>
</x:cfRule>
```

`<x:cfvo>` (conditional-format value object) types: `min`, `max`,
`num` (numeric), `percent`, `percentile`, `formula`. The number of
`cfvo` and `color` children must match (2 or 3).

### dataBar

```xml
<x:cfRule type="dataBar" priority="1">
  <x:dataBar>
    <x:cfvo type="min"/>
    <x:cfvo type="max"/>
    <x:color rgb="FF638EC6"/>
  </x:dataBar>
</x:cfRule>
```

### iconSet

```xml
<x:cfRule type="iconSet" priority="1">
  <x:iconSet iconSet="3TrafficLights1">
    <x:cfvo type="percent" val="0"/>
    <x:cfvo type="percent" val="33"/>
    <x:cfvo type="percent" val="67"/>
  </x:iconSet>
</x:cfRule>
```

`iconSet` values include `3Arrows`, `3ArrowsGray`, `3Flags`,
`3TrafficLights1`, `3TrafficLights2`, `3Signs`, `3Symbols`, `3Symbols2`,
`4Arrows`, `4ArrowsGray`, `4RedToBlack`, `4Rating`, `4TrafficLights`,
`5Arrows`, `5ArrowsGray`, `5Rating`, `5Quarters`.

---

## 5. dxf differential format

Sits in `xl/styles.xml` after `<x:cellXfs>`:

```xml
<x:dxfs count="2">
  <x:dxf>
    <x:font><x:b/><x:color rgb="FFFF0000"/></x:font>
    <x:fill><x:patternFill><x:bgColor rgb="FFFFFF00"/></x:patternFill></x:fill>
  </x:dxf>
  <x:dxf>
    <x:font><x:i/></x:font>
  </x:dxf>
</x:dxfs>
```

A `<x:dxf>` is a *partial* format — only the attributes that should
override the cell's existing format are present. It does **not** carry
`<x:alignment>` defaults the way `<x:xf>` does; missing children mean
"don't override".

`dxfId` from a `cfRule` is the 0-based index into `<x:dxfs>`.

`<x:dxfs count="N">` must equal the actual child count.

---

## 6. SDK Access Pattern

```rust
let workbook = package.workbook_part()?;
let sheet1: WorksheetPart = workbook.worksheet_parts(&package).next()?;
let ws: &Worksheet = sheet1.root(&package);

for cf in &ws.conditional_formatting {
  for rule in &cf.cf_rule {
    match rule.r#type {
      ConditionalFormatValues::CellIs => { /* … */ }
      ConditionalFormatValues::ColorScale => {
        let cs = rule.color_scale.as_ref()?;
        // cs.cfvo, cs.color
      }
      _ => {}
    }
  }
}
```

The `dxfs` collection on the workbook stylesheet exposes the differential
formats:

```rust
let styles = workbook.workbook_styles_part(&package).root(&package);
let dxf = &styles.dxfs.as_ref()?.dxf[rule.dxf_id?? as usize];
```

---

## 7. Round-Trip Gotchas

1. **`priority` is not a position — it's a relative ordering.** Two
   rules with the same priority is undefined behaviour. Round-trip code
   must preserve the values exactly; renumbering them changes which rule
   wins on overlapping ranges.

2. **`sqref` is space-separated A1 ranges, not comma-separated.** Excel
   accepts spaces only. Round-trip code that re-emits with commas will be
   silently rejected by some consumers.

3. **`<x:cfvo>` ordering matters.** The `cfvo` children are consumed
   positionally — the first is the lower bound, the last is the upper
   bound. Sorting them alphabetically would invert colour gradients.

4. **`<x:colorScale>`'s `<x:color>` and `<x:cfvo>` counts must match
   (and be 2 or 3).** Mismatched counts cause Excel to silently drop the
   rule.

5. **`dxfId` is 0-based and refers to `<x:dxfs>`, not `<x:cellXfs>`.**
   Code that confuses the two will reference the wrong format.

6. **`<x:dxfs count="…">` must equal the child count.** ooxmlsdk does
   not auto-recompute this on serialise — fixtures must keep the value
   in sync with the actual child elements.

7. **Visualisation rules (colorScale / dataBar / iconSet) ignore
   `dxfId`.** If round-trip code carries a stale `dxfId` attribute on
   these, Excel ignores it but may flag the file as inconsistent.

8. **`stopIfTrue` is per-rule, not per-block.** Setting it on the
   `conditionalFormatting` element is invalid; it lives on `<x:cfRule>`.

---

## 8. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| SML-CF-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/spreadsheet/conditional_cellis.xlsx` | `cellIs` operator `greaterThan` with `<x:formula>` and `dxfId`; `expression` rule referencing a different `<x:dxf>`; `<x:dxfs count="2">` in styles.xml with font + fill formats |
| SML-CF-02 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/spreadsheet/conditional_visual.xlsx` | 3-stop `<x:colorScale>` with min/percentile/max `<x:cfvo>` and three colours; `<x:dataBar>` rule on a separate range; `<x:iconSet iconSet="3TrafficLights1">` with three cfvo entries |
