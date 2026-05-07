# SpreadsheetML Pivot Tables — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §18.10 (Pivot Tables); §15.2.13 (Pivot Table Part), §15.2.11 (Pivot Cache Definition Part), §15.2.12 (Pivot Cache Records Part); ISO/IEC 29500:2016 Part 1; XSDs `sml-pivotTable.xsd`, `sml-pivotCacheDefinition.xsd`, `sml-pivotCacheRecords.xsd`.

---

## 1. Overview

A pivot table is composed of three cooperating parts:

| Part | File | Role |
|------|------|------|
| `PivotTablePart` | `xl/pivotTables/pivotTableN.xml` | Layout: rows, columns, data, page filters, formatting |
| `PivotTableCacheDefinitionPart` | `xl/pivotCache/pivotCacheDefinitionN.xml` | Field metadata + reference back to the source range |
| `PivotTableCacheRecordsPart` | `xl/pivotCache/pivotCacheRecordsN.xml` | Materialised tuples (one record per source row) |

The workbook owns the cache definitions; each worksheet owns its pivot
table layouts. A single cache may back multiple pivot tables (the
"shared cache" model).

---

## 2. Part Graph

```
xl/workbook.xml
xl/_rels/workbook.xml.rels  ─── rId10 pivotCacheDefinition ──▶ pivotCache/pivotCacheDefinition1.xml
                              (also worksheet refs)

xl/pivotCache/pivotCacheDefinition1.xml
xl/pivotCache/_rels/pivotCacheDefinition1.xml.rels
                              ─── rId1 pivotCacheRecords ──▶ pivotCacheRecords1.xml
xl/pivotCache/pivotCacheRecords1.xml

xl/worksheets/sheet2.xml      (the sheet that *displays* the pivot)
xl/worksheets/_rels/sheet2.xml.rels
                              ─── rId1 pivotTable ──▶ ../pivotTables/pivotTable1.xml

xl/pivotTables/pivotTable1.xml
xl/pivotTables/_rels/pivotTable1.xml.rels
                              ─── rId1 pivotCacheDefinition ──▶ ../pivotCache/pivotCacheDefinition1.xml
```

### `[Content_Types].xml`

```xml
<Override PartName="/xl/pivotCache/pivotCacheDefinition1.xml"
  ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml"/>
<Override PartName="/xl/pivotCache/pivotCacheRecords1.xml"
  ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml"/>
<Override PartName="/xl/pivotTables/pivotTable1.xml"
  ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml"/>
```

### Workbook ↔ caches

```xml
<x:workbook>
  …
  <x:pivotCaches>
    <x:pivotCache cacheId="1" r:id="rId10"/>
  </x:pivotCaches>
</x:workbook>
```

`cacheId` is the workbook-scoped integer that pivot tables reference via
`<x:pivotTableDefinition cacheId>`.

---

## 3. Pivot Cache Definition

```xml
<x:pivotCacheDefinition xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                        r:id="rId1" recordCount="3" refreshOnLoad="1"
                        createdVersion="6" refreshedVersion="6" minRefreshableVersion="3">
  <x:cacheSource type="worksheet">
    <x:worksheetSource ref="A1:C4" sheet="Source"/>
  </x:cacheSource>
  <x:cacheFields count="3">
    <x:cacheField name="Region" numFmtId="0">
      <x:sharedItems count="2">
        <x:s v="North"/>
        <x:s v="South"/>
      </x:sharedItems>
    </x:cacheField>
    <x:cacheField name="Quarter" numFmtId="0">
      <x:sharedItems count="2">
        <x:s v="Q1"/>
        <x:s v="Q2"/>
      </x:sharedItems>
    </x:cacheField>
    <x:cacheField name="Sales" numFmtId="0">
      <x:sharedItems containsSemiMixedTypes="0" containsString="0"
                     containsNumber="1" containsInteger="1"
                     minValue="100" maxValue="400"/>
    </x:cacheField>
  </x:cacheFields>
</x:pivotCacheDefinition>
```

### `<x:cacheSource>` types

| `type` | Body |
|--------|------|
| `worksheet` | `<x:worksheetSource ref="A1:C4" sheet="Source"/>` (or `name="namedRange"`) |
| `external` | `<x:consolidation>` / external connection markup |
| `consolidation` | Multiple `<x:rangeSet>` children |
| `scenario` | Empty body |

### `<x:cacheField>` and `<x:sharedItems>`

Each field declares its name and the distinct values it takes across the
source. **For string columns**, `<x:sharedItems>` enumerates the values
explicitly (the cache records reference them by zero-based index). **For
numeric columns**, `<x:sharedItems>` records range metadata (`minValue`,
`maxValue`, `containsNumber`, etc.) and the records carry raw numbers.

`containsSemiMixedTypes` defaults to true; setting it to `0` is an
optimisation flag indicating the column is purely one type.

`<x:cacheFields count>` must equal the child count.

---

## 4. Pivot Cache Records

```xml
<x:pivotCacheRecords xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                     count="3">
  <x:r>
    <x:x v="0"/>           <!-- Region = sharedItems index 0 (North) -->
    <x:x v="0"/>           <!-- Quarter = Q1 -->
    <x:n v="100"/>         <!-- Sales = 100 -->
  </x:r>
  <x:r>
    <x:x v="0"/><x:x v="1"/><x:n v="200"/>
  </x:r>
  <x:r>
    <x:x v="1"/><x:x v="0"/><x:n v="300"/>
  </x:r>
</x:pivotCacheRecords>
```

Cell-value tag dispatch (must match the `<x:cacheField>` definition):

| Tag | Meaning |
|-----|---------|
| `<x:s v="…"/>` | String literal |
| `<x:n v="…"/>` | Number |
| `<x:b v="0\|1"/>` | Boolean |
| `<x:d v="…"/>` | Date (ISO 8601) |
| `<x:e v="#…"/>` | Error |
| `<x:m/>` | Missing/blank |
| `<x:x v="…"/>` | Index into the field's `<x:sharedItems>` |

`<x:pivotCacheRecords count>` must equal the number of `<x:r>` children
and must equal the cache definition's `recordCount`.

---

## 5. Pivot Table Definition

```xml
<x:pivotTableDefinition xmlns:x="http://schemas.openxmlformats.org/spreadsheetml/2006/main"
                        xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"
                        name="PivotTable1" cacheId="1" applyNumberFormats="0"
                        applyBorderFormats="0" applyFontFormats="0" applyPatternFormats="0"
                        applyAlignmentFormats="0" applyWidthHeightFormats="1"
                        dataCaption="Values" updatedVersion="6" minRefreshableVersion="3"
                        useAutoFormatting="1" itemPrintTitles="1" createdVersion="6"
                        indent="0" outline="1" outlineData="1" multipleFieldFilters="0">
  <x:location ref="E2:G6" firstHeaderRow="1" firstDataRow="2" firstDataCol="1"/>
  <x:pivotFields count="3">
    <x:pivotField axis="axisRow" showAll="0">
      <x:items count="3">
        <x:item x="0"/><x:item x="1"/><x:item t="default"/>
      </x:items>
    </x:pivotField>
    <x:pivotField axis="axisCol" showAll="0">
      <x:items count="3">
        <x:item x="0"/><x:item x="1"/><x:item t="default"/>
      </x:items>
    </x:pivotField>
    <x:pivotField dataField="1" showAll="0"/>
  </x:pivotFields>
  <x:rowFields count="1"><x:field x="0"/></x:rowFields>
  <x:rowItems count="3">
    <x:i><x:x/></x:i>
    <x:i><x:x v="1"/></x:i>
    <x:i t="grand"><x:x/></x:i>
  </x:rowItems>
  <x:colFields count="1"><x:field x="1"/></x:colFields>
  <x:colItems count="3">
    <x:i><x:x/></x:i>
    <x:i><x:x v="1"/></x:i>
    <x:i t="grand"><x:x/></x:i>
  </x:colItems>
  <x:dataFields count="1">
    <x:dataField name="Sum of Sales" fld="2" baseField="0" baseItem="0"/>
  </x:dataFields>
  <x:pivotTableStyleInfo name="PivotStyleLight16" showRowHeaders="1" showColHeaders="1"
                         showRowStripes="0" showColStripes="0" showLastColumn="1"/>
</x:pivotTableDefinition>
```

### `<x:location>`

Defines the cell range the pivot occupies on the sheet.

| Attr | Meaning |
|------|---------|
| `ref` | A1 range covering the full pivot block |
| `firstHeaderRow` | Row offset (within `ref`) of the data header |
| `firstDataRow` | Row offset of the first data value |
| `firstDataCol` | Column offset of the first data value |

### `<x:pivotField>`

One per cache field, in cache order. The `axis` attribute places the
field on rows / columns / pages, or omitted for a value-only field.
Possible `axis` values: `axisRow`, `axisCol`, `axisPage`, `axisValues`
(rare).

`<x:items>` enumerates the value-list shown for that field, including the
default ("grand total") sentinel `<x:item t="default"/>`. `t` values:

| `t` | Meaning |
|-----|---------|
| (omitted) | Data item; `x` references `sharedItems` |
| `default` | Grand total / "(All)" item |
| `sum`, `count`, `avg`, `max`, `min`, `product`, `countNums`, `stdDev`, `stdDevP`, `var`, `varP` | Subtotal items |
| `blank` | Blank source value |

### Row / column / data field projections

`<x:rowFields>` and `<x:colFields>` list the **pivotField indices** placed
on each axis. `<x:rowItems>` / `<x:colItems>` enumerate the rendered row
/ column tuples (one `<x:i>` per visible row, with nested `<x:x>` indices
selecting items per axis level). `t="grand"` marks the totals tuple.

`<x:dataFields>` declares aggregation columns. `fld` is the cache field
index; `baseField` / `baseItem` apply for "% of" calculations.

### Aggregation-function values

Values for `<x:dataField subtotal>`: `sum` (default), `count`, `average`,
`max`, `min`, `product`, `countNums`, `stdDev`, `stdDevP`, `var`, `varP`.

---

## 6. SDK Access Pattern

```rust
let workbook = package.workbook_part()?;
let cache_def = workbook.pivot_table_cache_definition_parts(&package).next()?;
let cache_root: &PivotCacheDefinition = cache_def.root(&package);

let cache_records = cache_def.pivot_table_cache_records_part(&package);
let pivot = workbook.worksheet_parts(&package).find_map(|ws|
  ws.pivot_table_parts(&package).next()
)?;
let layout: &PivotTableDefinition = pivot.root(&package);
```

Rust types under
`schemas_openxmlformats_org_spreadsheetml_2006_main`:
`PivotTableDefinition`, `Location`, `PivotField`, `RowFields`, `ColumnFields`,
`DataFields`, `PivotCacheDefinition`, `CacheField`, `SharedItems`,
`PivotCacheRecords`, `PivotCacheRecord`.

---

## 7. Round-Trip Gotchas

1. **`recordCount` (def) and `count` (records) and the actual `<x:r>`
   count must all match.** Mismatch is a hard error in Excel and pivot
   refresh fails silently — the reported counts are trusted.

2. **Row/col items must include the totals sentinels.** A pivot with
   `<x:rowItems count="N">` must include the grand-total `<x:i t="grand">`
   as the last item if the layout shows totals. Stripping it leaves the
   pivot unable to compute the totals row.

3. **`<x:pivotFields count>` must equal the number of cache fields.**
   Even fields not placed on any axis must appear as `<x:pivotField>`
   entries (with `dataField="0"` or `axis` omitted). Skipping them makes
   the pivot reject the cache binding on refresh.

4. **`<x:cacheFields count>` and `<x:sharedItems count>` are
   self-redundant.** The XML includes counts that must equal the
   underlying child counts. ooxmlsdk does not auto-recompute these on
   serialise — fixtures must keep them aligned.

5. **`<x:item x="…">` indices reference `<x:sharedItems>`, not pivot
   data.** A common confusion: pivot field items use the cache's shared
   items array, not the cache record positions. Stale indices show
   blanks in the pivot.

6. **`<x:r>` cell tags must match the cache field type.** A field
   declared `containsString` must use `<x:s>` or `<x:x>`; a numeric field
   must use `<x:n>`. Cross-typing produces "value not in list" refresh
   errors.

7. **`refreshOnLoad="1"` triggers Excel to recompute the cache** at open
   time. Round-trip code that flips this flag changes the document
   semantics — preserve the original value.

8. **Cache definition lives in `xl/pivotCache/`, pivot table in
   `xl/pivotTables/` — both relative to the *workbook*.** The slash
   conventions are `pivotCache` (camelCase, plural cache files) and
   `pivotTables` (camelCase, plural table files). Subtle typos here
   produce silent rejection.

9. **The pivot table's *workbook-scoped* `cacheId` must equal the
   `<x:pivotCache cacheId>` declared on the workbook.** Pivot tables in
   different sheets sharing one cache all carry the same `cacheId`
   value.

10. **Cache records may be omitted with `recordCount="0"` and an empty
    records part.** Excel still expects the part to exist (`Override`
    content type entry plus relationship); omitting the part entirely
    breaks the cache definition on next open.

---

## 8. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| SML-PT-01 | `test-data/spreadsheet/pivot_table.xlsx` | Two sheets: `Source` (3-row dataset Region/Quarter/Sales) and `Pivot` (the pivot layout); `xl/pivotCache/pivotCacheDefinition1.xml` with worksheetSource ref + 3 cacheFields (two string with sharedItems, one numeric with min/max metadata); `xl/pivotCache/pivotCacheRecords1.xml` with 3 records using `<x:x>` for indexed strings and `<x:n>` for the number; `xl/pivotTables/pivotTable1.xml` with row/col/data fields and pivotTableStyleInfo; workbook `<x:pivotCaches>` entry; both content-type Overrides; sheet2 → pivotTable rel and pivotTable → cacheDefinition rel |
