# ooxmlsdk-formula Design

## 1. Goal

`ooxmlsdk-formula` is the SpreadsheetML formula and workbook value layer for
`ooxmlsdk`.

Its job is to turn typed SpreadsheetML workbook data into a value-aware workbook
model that Calc layout, export, validation, and tests can consume without
implementing formula semantics themselves.

This crate is not a PDF helper. It is centered on SpreadsheetML/Calc formula
and value semantics. Consumers may include layout, validators, workbook
inspection tools, recalculation tools, round-trip diagnostics, or PDF export,
but none of those consumers should define the formula model.

The intended XLSX pipeline is:

```text
ooxmlsdk
  -> ooxmlsdk-formula
  -> ooxmlsdk-layout::calc
  -> ooxmlsdk-pdf
```

`ooxmlsdk-layout` may depend on the formula crate for value-provider types, but
layout must not implement formula parsing, dependency tracking, or evaluation.
`ooxmlsdk-fonts` is deliberately not part of this pipeline: font metrics affect
how the already computed display text is laid out, not how formulas are parsed
or evaluated.

## 2. Design Authority

### 2.1 Primary Reference: LibreOffice Calc

Use LibreOffice Calc as the main design reference. Start from local source
before browsing.

| Area | LibreOffice path |
|---|---|
| XLSX workbook import | `../core/sc/source/filter/oox/workbookfragment.cxx` |
| XLSX worksheet import | `../core/sc/source/filter/oox/worksheetfragment.cxx` |
| Sheet data import | `../core/sc/source/filter/oox/sheetdatacontext.cxx` |
| Sheet data buffering | `../core/sc/source/filter/oox/sheetdatabuffer.cxx` |
| Defined names | `../core/sc/source/filter/oox/defnamesbuffer.cxx` |
| Calc formula compiler | `../core/sc/source/core/tool/compiler.cxx` |
| Formula cell model | `../core/sc/source/core/data/formulacell.cxx` |
| Formula interpreter | `../core/sc/source/core/tool/interpr*.cxx` |
| Interpreter declarations | `../core/sc/source/core/inc/interpre.hxx` |
| Calculation config | `../core/sc/source/core/tool/calcconfig.cxx` |
| Document calculation entry points | `../core/sc/source/core/data/documen*.cxx` |
| Number formatting interaction | `../core/sc/source/core/tool/interpr*.cxx`, `../core/svl/source/numbers/` |

Use these files to preserve Calc concepts: formula cells, token arrays,
address grammar, cached results, dependency state, recalculation policy, and
error propagation.

### 2.2 OOXML Input Reference

Use this repository's generated `ooxmlsdk` types and existing specifications:

- `docs/specs/sml_formulas.md`
- `docs/specs/sml_cells.md`
- `docs/specs/sml_names.md`
- `docs/specs/sml_formatting.md`
- generated SpreadsheetML schema types under `crates/ooxmlsdk/src/schemas/`
- typed package roots such as `SpreadsheetDocument`

Do not parse raw XML when generated schema types expose the data. Raw XML is
only acceptable for currently unmodeled extension payloads that must be
preserved structurally.

The first-class import path must be typed package and schema traversal:

```text
SpreadsheetDocument
  -> WorkbookPart
  -> WorksheetPart / SharedStringTablePart / WorkbookStylesPart
  -> generated x::* SpreadsheetML structs
  -> WorkbookValueModel
```

Do not build formula state from `ooxmlsdk-pdf`'s current Calc structs as the
primary input. That code is useful migration evidence, but the formula crate
must read the same typed `ooxmlsdk` data that non-PDF consumers use.

### 2.3 Typst Reference

Typst is not a formula engine reference. Use it only for Rust implementation
style where applicable:

| Area | Typst path |
|---|---|
| Typed value/data ownership patterns | `../typst/crates/typst-library/src/` |
| Incremental/cached computation examples | `../typst/crates/typst-layout/src/` |

Do not import Typst formula or expression semantics into SpreadsheetML.

## 3. Non-Goals

`ooxmlsdk-formula` must not own:

- worksheet print pagination
- row/column/cell layout
- font shaping or text measurement
- PDF rendering
- chart rendering
- pivot table layout
- OOXML package read/write APIs
- Writer or Impress document behavior

Formula output may feed these systems, but the formula crate is not their owner.
Do not add formula behavior because a PDF fixture happens to need a displayed
string; add it because Calc formula/value semantics require it, then let PDF
consume the result as one downstream use case.

## 4. Crate Responsibility

The crate owns:

- formula cell records
- formula text normalization
- A1/R1C1 reference parsing
- sheet/workbook address identity
- token or AST representation
- shared formula expansion
- array formula identity
- data table formula identity
- defined-name resolution
- dependency graph construction
- cached result preservation
- stale/recalc state
- formula evaluation where implemented
- error value propagation
- value-provider API for layout and export
- display-value bridge hooks for number formatting

The first implementation may be cache-first. It should preserve enough
structured state to later add a full evaluator without changing the public data
model.

## 5. Dependency Direction

Target dependencies:

```text
ooxmlsdk-formula -> ooxmlsdk
ooxmlsdk-layout  -> ooxmlsdk-formula
```

`ooxmlsdk-formula` should not depend on `ooxmlsdk-layout`, `ooxmlsdk-fonts`, or
`ooxmlsdk-pdf`.

Downstream crates may consume formula results, but dependency direction must
never point back from formula to layout, fonts, or PDF.

The public constructors should make the typed boundary visible, for example by
accepting a `SpreadsheetDocument`, workbook parts, worksheet parts, or borrowed
generated SpreadsheetML roots. Helper constructors for tests may accept already
extracted typed roots, but should not introduce a separate XML parser.

## 6. Public Model Shape

The public model should be Calc-shaped rather than XML-shaped:

```text
WorkbookValueModel
  identity
  sheets
  defined_names
  shared_formula_groups
  calc_chain
  external_references
  calculation_settings
```

```text
WorkbookIdentity
  workbook_name
  sheets
  date_system
  reference_style

WorksheetIdentity
  id
  name
  relationship_id
  visible
```

```text
FormulaCell
  address
  formula_kind
  formula_text
  parsed_formula
  cached_value
  evaluated_value
  formula_state
  number_format_context
```

```text
FormulaKind
  Normal
  SharedDefinition
  SharedDependent
  Array
  DataTable
```

```text
FormulaState
  Clean
  CachedOnly
  Stale
  Unsupported
  External
  Error
```

The model must distinguish formula text, cached value, evaluated value, and
display text. Losing that distinction makes layout and round-trip behavior
incorrect.

Calculation settings should preserve Calc/Excel state even before a full
evaluator exists:

- calculation mode
- 1900 vs 1904 date system
- A1 vs R1C1 reference style
- full-calc-on-load and force-full-calc flags
- iteration enabled/count/delta
- full precision flag

### 6.1 Ownership Model

Use borrowed data where it naturally comes from parsed `ooxmlsdk` structs:

- formula text, cell references, defined-name text, sheet names, relationship
  ids, and cached string values may be `Cow<'doc, str>`
- parsed addresses, ranges, workbook ids, formula kind, state flags, and error
  identities should be compact copyable Rust types
- evaluated values and display strings may be owned when they are computed,
  normalized, translated shared formulas, or number-formatted output

The model should be able to operate as a borrowed view over a parsed workbook
for layout/export, with an explicit conversion path to an owned model when a
consumer needs to keep formula state after the source package is dropped.

## 7. Value Provider API

The key integration contract is a value provider, not a layout callback:

```text
CellValueProvider
  raw_value(sheet, cell)
  formula_value(sheet, cell)
  cached_value(sheet, cell)
  display_text(sheet, cell, format_context)
  formula_state(sheet, cell)
```

`ooxmlsdk-layout::calc` should consume this API to obtain printable cell text.
It should not inspect formula tokens or parse formula expressions.

## 8. Import Stages

The formula crate should follow the Calc import split:

1. Workbook identity
   - sheet order
   - sheet names
   - workbook date system
   - calculation settings
   - external reference metadata

2. Cell collection
   - constants
   - formula cells
   - cached values
   - cell type metadata

3. Formula group registration
   - shared formula definitions
   - dependents
   - array ranges
   - data tables

4. Name resolution
   - workbook names
   - sheet-local names
   - built-in names such as print areas remain visible to layout/import, but
     the formula crate owns name expression parsing.

5. Dependency graph
   - direct cell references
   - range references
   - sheet references
   - volatile formulas
   - external reference placeholders

6. Evaluation or cache resolution
   - use cached values when no evaluator branch exists
   - mark stale/unsupported state explicitly
   - never silently invent values

## 9. Evaluation Strategy

Initial implementation should be staged:

### Stage 1: Cache-Preserving Value Model

- parse formula cell metadata
- preserve formula text and formula kind
- expose cached values
- expand shared formulas enough to expose dependents
- preserve array/data table identity
- expose stale and recalc flags

This is enough for many XLSX layout tests because producers usually save cached
formula results.

### Stage 2: Address and Name Parser

- A1 references
- sheet-qualified references
- workbook/sheet names
- simple ranges
- defined-name dependency edges

### Stage 3: Expression Parser

- operator precedence
- literals
- function calls
- array constants
- references
- error literals

### Stage 4: Core Evaluator

- arithmetic
- comparison
- text concatenation
- boolean conversion
- common aggregate functions
- date/time numeric behavior
- error propagation

### Stage 5: Excel/Calc Compatibility Expansion

- volatile functions
- lookup functions
- text/date/math/statistical functions
- array formulas
- dynamic arrays if in scope
- external links and stale cache policy

Each stage must preserve source-backed formula state even when evaluation is
unsupported.

## 10. Number Formatting Boundary

Formula evaluation produces values. Display strings require number-format and
locale context. Keep this boundary explicit:

```text
FormulaValue
  Number
  String
  Boolean
  Error
  Blank
  Matrix
```

```text
DisplayValue
  text
  source_value
  number_format_id
  stale
```

`ooxmlsdk-formula` may expose display formatting helpers if they are Calc value
semantics, but page layout must only consume formatted text. If number
formatting grows large, split it inside the formula/value crate rather than
moving it into layout.

## 11. Error Policy

Do not collapse all failures into missing text. Preserve error identity:

- parse error
- unsupported function
- external reference unavailable
- stale cached value
- dependency cycle
- calculation overflow
- Excel error value such as `#DIV/0!`

Layout can then choose the visible string according to Calc print behavior.

## 12. Tests

Primary test sources:

| Test class | Source |
|---|---|
| OOXML formula structure | `docs/specs/sml_formulas.md` |
| Calc formula import tests | `../core/sc/qa/unit/` |
| Pivot/formula fixture evidence | `../core/sc/qa/unit/data/xlsx/` |
| Existing XLSX PDF matrix candidates | `docs/tests/ooxmlsdk-pdf-test/libreoffice/UPSTREAM_TEST_MATRIX.md` |

Test buckets:

- formula element parsing
- shared formula expansion
- array/data table identity
- defined names
- cached value fallback
- stale/recalc flags
- dependency graph edges
- simple evaluator functions
- display strings used by layout

Tests must distinguish import/state assertions from printed-output assertions.
Printed-output assertions belong in `ooxmlsdk-layout` or `ooxmlsdk-pdf`.

## 13. Calibration Loop

Use a repeated three-pass loop before implementing each formula area:

1. LO pass
   - identify the Calc owner files
   - name the Calc concepts being ported
   - record source paths in comments or tests

2. Rust design pass
   - map Calc concepts into compact Rust data types
   - prefer explicit states over flags hidden in strings
   - avoid exposing XML internals as the public model

3. Fixture pass
   - choose upstream fixtures
   - prove cached/evaluated/display output separately
   - record unsupported behavior as structured state, not as guessed output

Repeat the loop after tests fail or reveal a missing Calc concept.
