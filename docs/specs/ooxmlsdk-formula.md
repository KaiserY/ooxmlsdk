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
  -> ooxmlsdk-layout::xlsx
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

### 2.1.1 Development Discipline

Every formula change must start from both local evidence sources before code is
written:

1. current Rust code in `crates/ooxmlsdk-formula/` and its layout/PDF/test
   consumers
2. matching LibreOffice Calc code under `../core/sc/`

Do not implement behavior from memory, spreadsheet folklore, or convenient
heuristics. In particular:

- do not add another A1/range parser when `QualifiedAddress` or
  `QualifiedRange` can be extended
- do not hard-code volatile, external, macro, unsupported, or function-family
  lists unless the list is translated from LO formula metadata or checked-in
  OOXML data
- do not invent magic numbers for limits, grammar widths, date systems, cache
  states, or recalculation policy; reuse existing constants/types or cite the
  LO/OOXML source path next to the value
- do not silently approximate formula semantics; preserve unsupported or
  partial features as structured metadata until LO-backed behavior is mapped
- when LO behavior is not yet identified, leave conservative fields unset or
  defaulted instead of writing guessed fallback logic

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
  array_formula_groups
  data_tables
  calc_chain
  dependency_graph
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
  dirty
  volatile
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

Parsed formula state should be rich enough for later evaluator work without
changing the public model:

```text
ParsedFormula
  source
  tokens
  ast
  dependencies
  unsupported
```

Tokens should preserve Calc-like structure:

- literals, references, names, functions, operators
- array open/close and separators
- opcode-like markers for cells, areas, external references, functions, names,
  matrices, and missing arguments

References must preserve absolute/relative row and column flags, whole-row and
whole-column references, sheet identity, and external placeholders. Dependency
graph edges should be structured separately from formula text so stale,
volatile, unsupported, and external states can be inspected without evaluating
the workbook.

The A1 parser is a shared SpreadsheetML service. Downstream layout code must
not keep a second cell/range parser for worksheet geometry; it should convert
from `QualifiedAddress` and `QualifiedRange` into layout-local address structs
when needed. This keeps Calc dependency tracking, print ranges, merged ranges,
view selections, and data-table references on one interpretation of quoted
sheet names, absolute flags, and whole-row/whole-column ranges.

The expression parser is allowed to build a Calc-like AST above that shared
address service, but it must not create a separate incompatible A1 grammar.
When parser work discovers a missing address case, extend the shared
`QualifiedAddress` / `QualifiedRange` layer first, then use it from tokens,
AST, dependency graph, layout bridges, and tests.

For Excel/Calc sheet limits, use LO's `sc/inc/address.hxx` constants:
`MAXCOL = 16383` and `MAXROW = 1048575` for normal XLSX sheets. Jumbo sheet
limits should become explicit model state if they are later supported, not a
silent replacement for standard XLSX bounds.

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

`ooxmlsdk-layout::xlsx` should consume this API to obtain printable cell text.
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
   - follow Calc's compiler/interpreter split: parsed formula state is not the
     same object as evaluated cell value

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
- unsupported tail/grammar fragments preserved as metadata

### Stage 4: Core Evaluator

- arithmetic
- comparison
- text concatenation
- boolean conversion
- common aggregate functions
- error propagation
- `IF`/`IFERROR`/`IFNA` control flow without eagerly evaluating unused branches
- date/time numeric behavior

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

0. Existing-code preflight
   - inspect `crates/ooxmlsdk-formula` for existing parsers, address/value
     types, dependency records, and workbook import helpers before adding new
     code
   - inspect downstream callers such as `ooxmlsdk-layout::xlsx` to find
     duplicated formula/address logic that should move back into this crate
   - prefer extending shared types such as `QualifiedAddress`,
     `QualifiedRange`, `WorkbookValueModel`, and `CellValueProvider` over
     introducing parallel helpers
   - record why an existing API is insufficient before adding a new parser,
     model, or evaluator entry point
   - inspect the corresponding LO files in `../core/sc/` during the same pass;
     every new constant, fallback list, function classification, or parse
     shortcut must have an LO/OOXML source or remain unsupported
   - if `ooxmlsdk-pdf` already has a passing behavior, treat it only as fixture
     evidence and migration inventory; re-implement through the formula crate's
     current model and LO's Calc owner files, not by copying the PDF helper
     parser/evaluator

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

## 14. Logic Kickoff Plan

Start with a cache-preserving typed import path. Do not begin with a full
evaluator.

### 14.0 Next Broad Development Focus

The next formula step should continue turning parsed formula state into a
Calc-shaped evaluator without trying to shortcut the full Calc interpreter:

- keep the shared token/dependency skeleton from formula text during typed cell
  import
- preserve references, functions, operators, literals, names, arrays, AST, and
  unsupported fragments in `ParsedFormula`
- keep `QualifiedAddress` and `QualifiedRange` as the only A1 parser surface
  used by layout and formula internals
- collect defined-name and external-reference placeholders structurally, even
  before resolution/evaluation is complete
- keep cached values as the printable result unless a supported evaluator path
  explicitly computes a value
- grow evaluator branches from LO `ScInterpreter` methods, including their
  control-flow behavior, error handling, and argument iteration rules

This stage should support the XLSX print core by making formulas inspectable,
dependency-aware, and evaluatable for LO-backed common paths while still
avoiding speculative function semantics.

### 14.1 Workbook And Sheet Identity Import

Reference:

- `../core/sc/source/filter/oox/workbookfragment.cxx`
- `../core/sc/source/filter/oox/worksheetfragment.cxx`
- `../core/sc/source/filter/oox/workbooksettings.cxx`
- generated `ooxmlsdk` SpreadsheetML package parts and `x::*` schema types

Scope:

- import workbook sheet order, sheet names, visibility, relationship ids, date
  system, reference style, and calculation settings
- preserve external reference metadata without trying to resolve remote files
- expose `WorkbookIdentity` and `WorksheetIdentity` before importing cell
  values

Done when tests can load a typed `SpreadsheetDocument` and inspect workbook
identity without reading raw XML or touching layout.

### 14.2 Cell And Formula Metadata Import

Reference:

- `../core/sc/source/filter/oox/sheetdatacontext.cxx`
- `../core/sc/source/filter/oox/sheetdatabuffer.cxx`
- `../core/sc/source/filter/oox/formulabuffer.cxx`
- current `crates/ooxmlsdk-formula/src/model.rs` import and dependency model

Scope:

- import constants, cached values, formula text, formula kind, shared formula
  ids, array ranges, data table flags, dirty/stale flags, and cell type metadata
- preserve cached/evaluated/display distinctions
- expand shared formulas only enough to expose dependent records, addresses,
  parsed tokens, and dependency skeletons
- mark unsupported or external formulas explicitly instead of inventing values

Done when cache-first XLSX fixtures expose the printable cell text and formula
state needed by `ooxmlsdk-layout::xlsx`.

### 14.3 Address, Name, And Dependency Skeleton

Reference:

- `../core/sc/source/core/tool/compiler.cxx`
- `../core/sc/source/filter/oox/defnamesbuffer.cxx`
- `../core/sc/source/filter/oox/addressconverter.cxx`

Scope:

- parse A1 addresses/ranges with absolute/relative flags
- preserve workbook and sheet-local defined names
- create dependency graph edges for cells, ranges, defined names, externals,
  and volatile placeholders
- keep parser failures as structured unsupported records
- do not duplicate A1 parsing; extend `QualifiedAddress` and
  `QualifiedRange` when the grammar needs to grow

Done when dependency graph tests can assert edges separately from formula
evaluation.

### 14.4 Evaluator Gate

Reference:

- `../core/sc/source/core/tool/interpr*.cxx`
- `../core/sc/source/core/inc/interpre.hxx`
- `../core/sc/source/core/tool/calcconfig.cxx`

Scope:

- implement arithmetic, comparison, concatenation, boolean conversion, simple
  aggregates, short-circuit control-flow functions, and error propagation from
  LO `ScInterpreter` behavior
- evaluate only through explicit public entry points such as
  `evaluate_supported_formulas`; import remains cache-preserving
- use calculation-chain entries when OOXML provides them; do not invent local
  iteration counts or recalc-pass limits
- range evaluation must operate over workbook model state and Calc/OOXML
  address bounds, not guessed expansion limits
- never let evaluator work block layout from consuming cached values

Done when common formula fixtures pass while unsupported functions still
produce structured state.

### 14.5 Current Implementation Checkpoint

Implemented in this stage:

- workbook and worksheet identity import from typed SpreadsheetML parts
- cell cached value/display value import, including shared and inline strings
- formula kind import for normal, shared, array, and data table cells
- shared formula group metadata with definition cell, `si`, `ref`, and
  dependent addresses
- shared formula dependent `parsed_formula` derivation from the definition
  formula, with relative A1 references translated according to existing
  absolute/relative address flags
- `ParsedFormula` preserves external reference placeholders and LO-backed
  volatile-function dependencies without evaluating values
- workbook defined names carry parsed dependency skeletons, and
  `DependencyGraph` exposes defined-name nodes/edges separately from cell
  formula edges
- workbook `externalReferences` are preserved as unavailable relationship-id
  placeholders until external workbook parts are resolved
- calculation-chain entries are imported from the typed calculation-chain part
  without guessing a sheet when OOXML omits `c@i`
- array formula and data table ranges, dirty/always-calculate flags, and data
  table input references/deleted-input flags where OOXML provides them
- lightweight dependency graph edges for directly parseable A1 cell/range
  tokens, names, externals, and volatile placeholders
- expression AST parsing for literals, references, functions, arrays, unary
  and binary operators, and error literals, built on the shared formula
  address/range parser
- explicit `evaluate_supported_formulas` entry point that updates
  `evaluated_value` and display bridge values only for supported formula ASTs
- LO-backed evaluator branches for arithmetic, comparison, concatenation,
  boolean conversion, `IF`, `IFERROR`, `IFNA`, `SUM`, `PRODUCT`, `AVERAGE`,
  `COUNT`, `COUNTA`, `MIN`, `MAX`, `AND`, `OR`, `ABS`, and `ROUND`
- range reads scan existing workbook cell records instead of expanding guessed
  whole-sheet loops; missing cells remain blank through the workbook value model
- display text created by evaluator is a value bridge only. Full number-format
  rendering remains a separate formula/value concern and must use LO number
  formatter behavior before adding locale/date/precision rules

Still intentionally not implemented:

- source-text rendering of translated shared formulas
- full Calc compiler tokenization, external workbook part resolution, and
  defined-name formula evaluation
- broad lookup/text/date/math/statistical function coverage
- dynamic arrays, spilled ranges, matrix evaluation beyond literal matrix
  values, and array formula result distribution
- full number-format rendering for evaluated values

Do not fill these gaps with local guesses. Add LO-derived tests first, then
translate the relevant Calc import/compiler behavior.
