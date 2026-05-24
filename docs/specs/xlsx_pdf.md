# XLSX PDF Rendering - LibreOffice-Aligned Design

**Status:** planning document for replacing the current smoke-only
`xlsx.rs` path with a LibreOffice Calc-shaped XLSX import, page-layout, and PDF
rendering pipeline. The first implementation should follow the PPTX approach:
port the LibreOffice owner boundaries and state model first, then lower the
already imported model into PDF-visible items.

**Primary authority:** LibreOffice `../core`.

Start with these files before changing behavior:

- `../core/sc/source/filter/oox/workbookfragment.cxx`
- `../core/sc/source/filter/oox/excelfilter.cxx`
- `../core/sc/source/filter/oox/workbookhelper.cxx`
- `../core/sc/source/filter/oox/workbooksettings.cxx`
- `../core/sc/source/filter/oox/viewsettings.cxx`
- `../core/sc/source/filter/oox/worksheetfragment.cxx`
- `../core/sc/source/filter/oox/worksheethelper.cxx`
- `../core/sc/source/filter/oox/worksheetbuffer.cxx`
- `../core/sc/source/filter/oox/worksheetsettings.cxx`
- `../core/sc/source/filter/oox/sheetdatacontext.cxx`
- `../core/sc/source/filter/oox/sheetdatabuffer.cxx`
- `../core/sc/source/filter/oox/stylesfragment.cxx`
- `../core/sc/source/filter/oox/stylesbuffer.cxx`
- `../core/sc/source/filter/oox/themebuffer.cxx`
- `../core/sc/source/filter/oox/pagesettings.cxx`
- `../core/sc/source/filter/oox/drawingbase.cxx`
- `../core/sc/source/filter/oox/drawingfragment.cxx`
- `../core/sc/source/filter/oox/excelchartconverter.cxx`
- `../core/sc/source/filter/oox/tablebuffer.cxx`
- `../core/sc/source/filter/oox/defnamesbuffer.cxx`
- `../core/sc/source/filter/oox/condformatbuffer.cxx`
- `../core/sc/source/filter/oox/autofilterbuffer.cxx`
- `../core/sc/source/filter/oox/externallinkbuffer.cxx`
- `../core/sc/source/filter/oox/externallinkfragment.cxx`
- `../core/sc/source/filter/oox/commentsbuffer.cxx`
- `../core/sc/source/filter/oox/threadedcommentsfragment.cxx`
- `../core/sc/source/filter/oox/querytablebuffer.cxx`
- `../core/sc/source/filter/oox/querytablefragment.cxx`
- `../core/sc/source/filter/oox/scenariobuffer.cxx`
- `../core/sc/source/filter/oox/scenariocontext.cxx`
- `../core/sc/source/filter/oox/pivotcachebuffer.cxx`
- `../core/sc/source/filter/oox/pivottablebuffer.cxx`
- `../core/sc/source/filter/oox/chartsheetfragment.cxx`
- `../core/sc/source/ui/view/printfun.cxx`
- `../core/sc/source/ui/view/output.cxx`
- `../core/sc/source/core/data/table*.cxx`
- `../core/sc/source/core/data/document*.cxx`
- `../core/sc/inc/pagepar.hxx`
- `../core/oox/source/drawingml/shape.cxx`
- `../core/oox/source/drawingml/graphicshapecontext.cxx`
- `../core/oox/source/drawingml/shapecontext.cxx`
- `../core/oox/source/drawingml/shapegroupcontext.cxx`
- `../core/oox/source/drawingml/table/*.cxx`
- `../core/oox/source/drawingml/chart/*.cxx`
- `../core/oox/source/drawingml/diagram/*.cxx`
- `../core/oox/source/shape/ShapeContextHandler.cxx`
- `../core/oox/source/vml/*.cxx`

Use `../typst` and `krilla` only for fixed-page/PDF mechanics:

- `../typst/crates/typst-pdf/src/convert.rs`
- `../typst/crates/typst-pdf/src/text.rs`
- `../typst/crates/typst-pdf/src/paint.rs`
- `../typst/crates/typst-layout/src/grid/`
- `../typst/crates/typst-library/src/layout/`

Typst may guide page frames, clipping, text and image paint, tables as PDF
items, and annotations. It must not define SpreadsheetML import, Calc print
range, page break, number formatting, sheet style, drawing-anchor, or formula
semantics.

---

## 1. Non-Negotiables

Do not evolve XLSX PDF support as a direct
`worksheet.xml -> text lines -> PDF` renderer.

Required flow:

```text
SpreadsheetDocument
  -> ExcelImport / WorkbookFragment / WorksheetFragment
  -> Calc workbook/sheet/style/drawing model
  -> ScPrintFunc-like page ranges and sheet print pages
  -> fixed-page PDF display items
  -> krilla paint
```

Rules:

- Port LibreOffice owner boundaries first, even when the first Rust branch only
  preserves structured fallback state.
- Renderer needs must not define import data structures.
- Display lowering must not parse OOXML, inspect package relationships, resolve
  workbook relationships, calculate style inheritance, choose number formats,
  or infer print settings.
- Keep workbook traversal in `workbook.xml` sheet order. Do not iterate package
  worksheet parts as visible sheet order.
- Preserve sheet identity: workbook sheet index, Calc sheet index, sheet name,
  visibility, relationship id/path, sheet type, active sheet, and page style.
- Preserve formulas and cached values separately. XLSX-to-PDF may initially
  render cached values, but import must keep formula type, shared/array/data
  table identity, formula text/tokens, and stale/recalc markers.
- Preserve unsupported charts, drawings, pivot tables, slicers, controls, OLE,
  comments, threaded comments, query tables, external links, scenarios, and
  custom XML as typed/structured records before any visible fallback.
- Do not lock temporary output into tests as final behavior. Text-only sheet
  dumps are useful only as smoke output, not parity behavior.
- Do not add local magic constants when LibreOffice already has defaults or
  clamps. Copy the owner and rule from `../core`, or leave a structured gap.
- Do not silence skeleton dead-code warnings with `#[allow]`; consume fields by
  porting the corresponding upstream branch.
- Markup compatibility must stay enabled through package loading, like DOCX and
  PPTX. Do not debug or implement XLSX PDF using non-MCE package reads.

---

## 2. Current State

`crates/ooxmlsdk-pdf/src/xlsx/` now has the first LibreOffice-shaped skeleton:

```text
xlsx::layout
  -> ExcelImport::import_document
  -> WorkbookFragment::finalize_import
  -> CalcSheet resource/catalog records
  -> CalcPrintDocument / CalcPrintPage
  -> display bridge into LayoutDocument/PageItem text and image primitives
```

This is still not Calc print parity. The visible output remains a temporary
paint bridge, but the import entry no longer grows from worksheet-part
iteration directly and drawing image bytes flow through imported resources.

Landed owner modules:

- `xlsx/import.rs`: `ExcelImport` entry plus workbook resource catalog from
  typed `WorkbookPart` children.
- `xlsx/comments.rs`: typed legacy worksheet comments and threaded comments
  catalogs. Legacy comments preserve authors, note references, author ids,
  GUIDs, shape ids, rich/phonetic run counts, text, and comment properties;
  threaded comments preserve flat root/reply ids, parent ids, person ids,
  timestamps, done state, text, mentions, and extension markers for later
  attachment to legacy notes.
- `xlsx/workbook.rs`: workbook sheet-order traversal through
  `workbook.sheets.sheet`, relationship id matching to typed sheet parts, and
  the first `finalizeImport` ordering for styles, shared strings, and defined
  names.
- `xlsx/workbook_settings.rs`: typed workbook-global settings for
  `workbookPr`, `bookViews`, `calcPr`, workbook/file sharing/protection
  markers, external-reference/function-group/pivot-cache counts, custom views,
  recovery properties, extensions, and OLE size references.
- `xlsx/workbook_catalog.rs`: typed workbook relationship catalog for external
  links, XML maps, workbook persons, revision headers/logs, custom XML, slicer
  and timeline caches, rich values, rich-value structures, rich arrays, rich
  styles, supporting property bags/structures, rich-value types, rich-value web
  images, custom data properties, workbook user data, VBA, attached toolbars,
  calc chain, cell metadata, volatile dependencies, and feature-property bags.
  Relationship-owned roots preserve relationship ids, child relationship
  markers, declared/root counts, text-bearing ids, and extension flags instead
  of remaining workbook-global bare counters.
- `xlsx/styles.rs`: typed stylesheet and defined-name catalogs from
  `WorkbookStylesPart` and `workbook.definedNames`, including custom number
  formats, font/fill/border/XF/table-style counts and records, default
  table/pivot style names, style XF and cell XF records with
  numFmt/font/fill/border/style-XF references, quote/pivot flags, apply flags,
  alignment/protection/extension markers, `Xf::createPattern`-style parent
  XF used-flag resolution, display font records, `Fill::finalizeImport`
  pattern/gradient color mixing, outer border records, and `_xlnm.Print_Area`
  / `_xlnm.Print_Titles` / `_xlnm._FilterDatabase` classification. Theme,
  indexed/palette color resolution, conditional formatting, and the rest of
  the locale-sensitive `SvNumberFormatter` surface remain in this owner before
  broad PDF text parity. The current display-string bridge resolves
  built-in/custom numFmtId strings and covers the Calc print path for General,
  text, boolean, numeric, grouped numeric, percent, currency-prefix, and
  serial date/time branches.
- `xlsx/table.rs`: typed table definition catalog from `TableDefinitionPart`,
  preserving table id/name/displayName/ref/type, header/totals row counts,
  table columns, style flags, auto-filter/sort-state presence, query-table
  child relationships, formulas/XML column markers, and extension markers.
- `xlsx/worksheet.rs`: `CalcSheet`, worksheet/chartsheet identity, rows/cells,
  page settings, worksheet/chartsheet resource catalogs, and first
  `WorksheetFragment`-shaped sheet metrics: dimension, `sheetFormatPr`,
  column spans, merged ranges, hyperlinks, row/column page breaks,
  conditional-format/data-validation/protected-range/scenario counts, and
  worksheet `extLst` condition resources. Cell import now preserves typed
  formula state for normal/shared/array/data-table formulas, shared ids,
  formula refs, calculation flags, data-table inputs, cached values,
  cell/value metadata, phonetic flags, row height/custom-height/style flags,
  and extension markers. Worksheet metrics now expose Calc-shaped cell
  rectangles and spreadsheet drawing marker coordinates; column width follows
  LibreOffice's `Unit::Digit -> Twip` path with the upstream default
  `1 digit = 2mm`, and row height follows the Calc point-to-twip import path.
- `xlsx/sheet_conditions.rs`: typed condition catalog for base
  `conditionalFormatting` / `dataValidations` plus `extLst` x14
  conditional-formatting, x14 data-validation, sparkline groups, ignored
  errors, slicer refs, protected ranges, web extensions, and timeline refs.
  This follows `ExtLstGlobalContext` instead of treating worksheet extensions
  as display-only unknown XML.
  Chartsheet import preserves `ChartsheetFragment` properties, protection,
  chart sheet views, custom chart views, web publish items, and extension
  markers.
- `xlsx/sheet_objects.rs`: typed worksheet OLE/control catalog mirroring
  `WorksheetFragment::importOleObject` / `importControl`, preserving shape ids,
  relationship ids, linked/embedded markers, progIds, update/icon/autoload
  flags, control names, x14 object/control properties, and object anchors.
- `xlsx/object_resources.rs`: worksheet object resource catalog for VML drawing
  relationships, VML child images/legacy diagram text, embedded control
  persistence parts, ActiveX binary children, typed `x14:formControlPr`
  records, embedded OLE object binaries, and embedded package binaries. This
  keeps worksheet XML object records separate from relationship-owned payloads,
  matching Calc's worksheet helper/VML owner split.
- `xlsx/sheet_relationships.rs`: typed worksheet relationship catalog for
  single-cell XML tables, named sheet views, slicers, timelines, worksheet sort
  maps, custom properties, printer settings, model3D relationships, and
  ActiveX binary relationships.
- `xlsx/drawing.rs`: typed `DrawingFragment`-shaped drawing/chart resource
  catalog from `DrawingsPart`, `ChartPart`, and `ExtendedChartPart`. Drawing
  import now preserves `twoCellAnchor` / `oneCellAnchor` / `absoluteAnchor`,
  from/to markers, extents, `editAs`, client data flags, object kind,
  non-visual id/name/description/hidden state, picture relationship ids,
  graphic frame URIs, content-part relationships, xdr solid shape fill,
  no-line, solid line color and line width, and child chart/diagram/media
  resources. Chart import preserves `chartSpace` / `chartEx` root state,
  external-data relationships, pivot/source/protection/title/legend/3D/axis
  markers, chart type groups, print/user-shape markers, extension markers,
  chartex data/series counts, and chart drawing/package/theme/style/color-style
  child relationships for the later `ExcelChartConverter`-shaped bridge.
  SmartArt import preserves the diagram data/layout/style/color/persisted
  drawing roots, relationship ids, data points/connections, layout node
  algorithm/control-flow stats, style/color labels, persisted drawing
  shapes/groups, child images, and extension markers for the later
  `oox::drawingml::diagram`-shaped bridge.
- `xlsx/page_settings.rs`: first `PageSettingsModel`-shaped defaults and typed
  worksheet/chartsheet page setup import, including LibreOffice's
  `PaperSizeConv::spPaperSizeTable` MS paper-size mapping, header/footer text
  channels, header/footer drawing and legacy drawing relationships, background
  picture relationships, and header/footer flags. Display lowering now consumes
  the text channels through a Calc-shaped `&L` / `&C` / `&R` section parser and
  expands page number, page count, sheet name, escaped ampersand, font-name, and
  font-size tokens without reopening worksheet XML.
- `xlsx/print.rs`: first `ScPrintFunc`-shaped page model beyond one-page-per
  sheet. It now resolves simple typed `_xlnm.Print_Area` A1 ranges, records
  print-title row/column ranges, falls back to the worksheet used area for
  visible sheets without explicit print ranges, splits page areas at manual row
  and column breaks and page content dimensions from the page-style paper size,
  subtracts repeated row/column extents from page-local data areas, and
  snapshots page-local cells with cell/row/column style fallback, repeated
  row/column/corner cells, hidden row/column hits, merged-range intersections,
  formula/style/text markers, number-format render state, page-local
  drawing-anchor fallback counts, and `PrintArea`-ordered paint operations.
  `CalcZoom` state now preserves scale-all versus fit-to-width/height
  branches, `ZOOM_MIN`, forced-break minimum page counts, tdf#103516
  fit-to-width adjustment state, skip-empty accounting, and
  top-down/over-then-down traversal ordering. Formula-token range conversion,
  scale-to-page-count input, full chart/SmartArt/VML/comment drawing paint, and
  theme/indexed/conditional color materialization remain in this owner.
- `xlsx/pivot.rs`: typed pivot cache and pivot table catalogs. Workbook cache
  definitions preserve workbook cache ids/relationships, cache field counts,
  record counts, refresh/save/invalid flags, records part presence, optional
  cache children, and extension markers. Worksheet pivot tables preserve
  name/cache id/location/style, field buckets, filters, formats, cache
  definition relationships, flags, and extension markers.
- `xlsx/query.rs`: typed workbook connections and worksheet query-table
  catalogs. Connections preserve ids, names, source/connection files, refresh
  flags, database/OLAP/web/text property markers, parameters, and extensions;
  query tables preserve names, connection ids, refresh field counts, deleted
  fields, sort state, formatting/refresh flags, and extension markers.
- `xlsx/print.rs`: first `ScPrintFunc`-shaped `CalcPrintDocument` /
  `CalcPrintPage` owner before display lowering, with built-in defined-name
  print inputs attached to print pages as structured records. Range extraction
  remains pending on the Calc formula parser owner, matching LibreOffice's
  `DefinedName::convertFormula` path instead of parsing formulas in display.
- `xlsx/sheet_conditions.rs`: typed `CondFormatBuffer` /
  `DataValidationsContext` inputs for conditional-format sqrefs, pivot flags,
  rule type/priority/dxf/operator/time/text/formulas, color scale/data bar/icon
  set markers, validation type/operator/error/prompt/list/formulas, prompt
  suppression, and validation window coordinates.
- `xlsx/sheet_settings.rs`: typed `WorksheetSettings` and
  `AutoFilterBuffer` inputs for `sheetPr`, outline/page setup properties,
  sheet protection flags, auto-filter refs/filter column counts, and sheet or
  auto-filter sort state markers.
- `xlsx/sheet_view.rs`: typed `SheetViewSettings` inputs for worksheet views,
  pane splits, active pane/state, selections, pivot selections, zooms,
  top-left cells, selected/right-to-left/grid/header/zero/formula flags, and
  extension markers.
- `xlsx/display.rs`: temporary observation bridge to `LayoutDocument`.

`convert_xlsx` now opens packages with MCE processing settings aligned with the
DOCX/PPTX entries.

The previous smoke bridge was:

```text
SpreadsheetDocument
  -> workbook_part.worksheet_parts(package)
  -> worksheet.sheet_data.row
  -> shared-string/plain cell text
  -> layout text pages
```

This path is intentionally insufficient for Calc parity:

- worksheet parts are not traversed through `workbook.xml` sheet order
- hidden sheets, chart sheets, macro/dialog sheets, active sheet, and views are
  ignored
- theme, styles, number formats, XF inheritance, conditional formatting, row
  and column dimensions, merged cells, page setup, print ranges, manual page
  breaks, headers/footers, drawings, charts, comments, hyperlinks, and print
  options are ignored
- formulas are rendered as raw cached values without retaining formula identity
- output is a text page per worksheet, not a Calc print page sequence

Treat the current display bridge as temporary. New behavior should extend the
LibreOffice-shaped owners under `xlsx/`, not reintroduce a direct
`worksheet.xml -> text lines -> PDF` path.

### Repository Reuse Points

Existing `ooxmlsdk-pdf` code has useful substrate, but the reuse direction must
stay narrow:

- Reuse `render::krilla` and the `layout` paint carriers for PDF mechanics
  only after XLSX has produced Calc-shaped fixed-page items.
- Reuse image/font/text metric helpers when their inputs are already resolved
  by XLSX import/layout owners.
- Reuse PPTX `drawingml` model concepts for shared DrawingML state: color,
  fill, line, effect, theme format scheme, shape properties, text body,
  graphical object frame dispatch, DrawingML tables, chart/diagram/OLE/media
  structured resources, and actual fill/line/effect caching.
- Do not reuse PPTX `SlidePersist`, placeholder inheritance, master/layout
  traversal, or service-name decisions. XLSX has worksheet anchors, Calc draw
  pages, sheet page styles, and Sc print layout.
- Do not reuse DOCX Writer page/frame layout for XLSX. DOCX frame carriers can
  inspire PDF paint plumbing only after Calc print pages are known.
- Do not keep `xlsx.rs` as a feature-growth file. The first implementation
  should introduce the module tree and route the public XLSX entry through
  `ExcelImport`, even if many modules initially preserve structured gaps.

### Typed SDK Boundary

Use generated `ooxmlsdk` types and typed part relationships as the primary
OOXML input:

- `SpreadsheetDocument`, `WorkbookPart`, `WorksheetPart`, `ChartsheetPart`,
  `DrawingsPart`, `ChartPart`, `ExtendedChartPart`, `ThemeOverridePart`,
  `WorkbookStylesPart`, `SharedStringTablePart`, `TableDefinitionPart`,
  worksheet comments/threaded comments, diagram parts, and image/media/package
  parts are already modeled.
- SpreadsheetML types such as `Workbook`, `Worksheet`, `Chartsheet`,
  `SharedStringTable`, `Stylesheet`, `Table`, `TableStyles`, `DefinedName`,
  sheet views, page setup, breaks, hyperlinks, merges, conditional formatting,
  and extension-list choices should be read from generated fields.
- DrawingML types such as `a::GraphicData`, `a::GraphicDataChoice`,
  `a::Table`, `a::Theme`, `a::ThemeOverride`, chart `c::ChartSpace`, chartEx
  `cx::ChartSpace`, chart drawing `cdr::*`, diagram relationship ids, and
  spreadsheet drawing `xdr::WorksheetDrawing` should stay typed through import.

Build resource catalogs from typed child-part APIs before adding any path-level
fallback:

- `WorkbookPart`: theme, styles, shared strings, connections, custom XML maps,
  workbook persons, revision headers, worksheets, chartsheets, dialog sheets,
  macro sheets, external workbooks, pivot cache definitions, slicer/timeline
  caches, rich value parts, and VBA project.
- `WorksheetPart`: printer settings, drawing, VML drawing, comments, threaded
  comments, table definitions, pivot tables, query tables, controls,
  control properties, embedded objects/packages, images, slicers/timelines,
  named sheet views, and model3D relationships.
- `DrawingsPart`: chart, chartEx, diagram data/layout/style/colors/persisted
  drawing, images, custom XML, and web extensions.
- `ChartPart` / `ExtendedChartPart`: chart space, chart drawing/user shapes,
  embedded package, images, theme override, chart style, and chart color style.

Typed child-part iteration is not sheet order. Use workbook `sheets/sheet`
records and relationship ids to select the sheet part in workbook order, then
use typed child parts for resources owned by that sheet or drawing.

Raw XML parsing is allowed only when the generated model exposes a payload as
opaque XML, `xml_children`, or a genuinely unmodeled extension. Document each
such fallback with the missing typed surface and the LibreOffice owner. Do not
parse raw XML merely because it is faster to prototype than following the SDK
part/type graph.

---

## 3. LibreOffice Main Chain

LibreOffice import and print flow to mirror:

```text
ExcelFilter::importDocument
  -> resolve the officeDocument workbook fragment path
  -> import document properties, swallowing property-only failures
  -> WorkbookHelper::constructGlobals
  -> create WorkbookFragment for the workbook path
  -> ScDocument::SetImportingXLSX(true)
  -> ScDocShell::SetInitialLinkUpdate
  -> ScDocOptions::SetIgnoreLineBreaks(true)
  -> importFragment(WorkbookFragment)
  -> map address overflow warnings
  -> ScDocument::SetImportingXLSX(false)

WorkbookFragment::onCreateContext
  -> import workbookPr / calcPr / fileSharing / fileVersion / oleSize
  -> import sheets in workbook.xml order
  -> import workbook views
  -> import external references
  -> import defined names
  -> import pivot caches
  -> import workbook extension lists

WorkbookFragment::finalizeImport
  -> import theme
  -> generate default table styles from theme
  -> import styles
  -> import shared strings
  -> import persons / connections / customXml / xmlMaps
  -> create WorksheetGlobals and WorksheetFragment for every workbook sheet
  -> initialize FormulaBuffer sheet count
  -> finalize tables and defined names
  -> initialize draw layer and lock model
  -> import all worksheet fragments
  -> apply table autofilters
  -> finalize worksheet buffer
  -> finalize workbook settings / view settings / formulas / pivots / scenarios
  -> finalize drawing import per sheet
  -> finalize named sheet views
  -> finalize document import
  -> recalc formula cells when required
  -> import revision headers
  -> attach macros after all target objects exist
  -> restore draw-layer lock state
```

Rust equivalents should stay traceable:

| LibreOffice | Rust |
|-------------|------|
| `ExcelFilter::importDocument` | `ExcelImport::import_document` |
| `WorkbookHelper::constructGlobals` | `WorkbookHelper::construct_globals` |
| `WorkbookFragment::finalizeImport` | `WorkbookFragment::finalize_import` |
| `WorkbookHelper` / `WorkbookGlobals` | `WorkbookHelper` / `WorkbookGlobals` |
| `ScDocOptions::SetIgnoreLineBreaks` | `CalcDocOptions::set_ignore_line_breaks` |
| `WorksheetHelper::constructGlobals` | `WorksheetHelper::construct_globals` |
| `WorksheetFragment::onCreateContext` | `WorksheetFragment::on_create_context` |
| `SheetDataContext` | `SheetDataContext` |
| `StylesFragment` / `StylesBuffer` | `StylesFragment` / `StylesBuffer` |
| `PageSettings::finalizeImport` | `PageSettings::finalize_import` |
| `DrawingFragment` | `DrawingFragment` |
| `ScPrintFunc::InitParam` | `PrintFunc::init_param` |
| `ScPrintFunc::CountPages` / `CalcZoom` / `CalcPages` | `PrintFunc::count_pages` / `calc_zoom` / `calc_pages` |
| `ScPrintFunc::PrintPage` | `PrintFunc::print_page` |

Skeleton constraints:

- `ExcelImport::import_document` is the only XLSX package entry. It creates
  workbook globals before any fragment import, sets document import options
  such as `IgnoreLineBreaks`, owns document property import, and records
  workbook/address warnings.
- `WorkbookFragment::on_create_context` owns workbook XML child dispatch. Do
  not move workbook settings, sheet registration, defined names, external
  references, or pivot caches into later display or worksheet code.
- `WorkbookFragment::finalize_import` owns global import order. Do not import
  sheet cells before theme/styles/shared strings are available.
- All worksheet/chartsheet fragment objects and worksheet globals are created
  before any sheet data is imported. This constructor phase is allowed to load
  sheet-owned table fragments needed before formulas.
- Table database ranges and defined names are finalized before worksheet cell
  formulas are converted; table autofilters are applied only after worksheet
  fragments have been imported and database ranges exist.
- Draw layer initialization and model lock/unlock belong to workbook finalize.
  Do not simulate worksheet drawing import without the workbook-level draw
  layer lifecycle.
- `WorksheetFragment` owns worksheet child dispatch and relationship-scoped
  imports. Display lowering must not reopen worksheet relationships.
- `SheetDataContext` owns row/cell order, implicit cell coordinates, formula
  state, cached values, inline/shared strings, row metadata, and used area.
- `StylesBuffer` owns default font/style, palette/theme color, number formats,
  XF inheritance, fills, borders, alignment, protection, differential formats,
  and table styles.
- `PageSettings` owns print options, margins, page setup, headers/footers, and
  page style creation.
- `DrawingFragment` owns xdr anchor rectangles and sheet drawing objects.
- `ScPrintFunc` owns print range, page scale, manual/automatic page breaks,
  repeated rows/columns, headers/footers, page-order traversal, and sheet page
  display-list construction.
- `WorksheetBuffer::finalize_import` owns sheet visibility and guarantees the
  active sheet is visible even if workbook metadata marks sheets hidden.

---

## 4. Module Boundaries

XLSX support should move from the single `xlsx.rs` file into
`crates/ooxmlsdk-pdf/src/xlsx/`.

Proposed modules:

- `xlsx/mod.rs`: public crate entry for layout/inspection and import orchestration
- `xlsx/import.rs`: `ExcelImport`, equivalent to the top-level import object
- `xlsx/workbook.rs`: `WorkbookFragment`, workbook sheets, settings, defined
  names, external links, pivot cache registry, active sheet
- `xlsx/worksheet.rs`: `WorksheetFragment`, sheet globals, worksheet settings,
  sheet views, dimensions, merges, hyperlinks, breaks, validations
- `xlsx/sheet_data.rs`: `SheetDataContext`, row/cell import and formula/cached
  value preservation
- `xlsx/styles.rs`: styles, XF resolution, fonts, fills, borders, number
  formats, table styles, differential formats
- `xlsx/theme.rs`: Excel theme/color lookup and palette bridging
- `xlsx/page_settings.rs`: print options, page margins/setup, header/footer
  parser, page styles
- `xlsx/drawing.rs`: spreadsheet drawing anchors, pictures, shapes, charts,
  controls, VML/comment shapes
- `xlsx/table.rs`: structured tables, table columns, totals, table styles,
  autofilters
- `xlsx/formula.rs`: formula identity, shared/array/data-table formulas,
  cached values, recalc flags
- `xlsx/print.rs`: Calc `ScPrintFunc`-shaped print layout and page range model
- `xlsx/display.rs`: fixed-page lowering into existing `LayoutDocument`

Keep generic DrawingML helpers shared with PPTX only where the upstream owner is
actually generic DrawingML. SpreadsheetDrawing anchor, sheet coordinate, and
relationship semantics are XLSX-specific and should stay under `xlsx/`.

Shared modules that should be extracted or reused across DOCX/PPTX/XLSX as the
implementation matures:

- `drawingml/color.rs`, `fill.rs`, `line.rs`, `shape_properties.rs`, `theme.rs`,
  `text_body.rs`, `text_list_style.rs`, and `table.rs` should become neutral
  DrawingML helpers once they no longer depend on PPTX `SlidePersist`.
- `graphical_object_frame_context` URI dispatch should be shared in concept:
  dispatch by exact `a:graphicData/@uri`, preserve typed payloads, and attach
  relationship-scoped resources from the owning document context.
- Chart, chartEx, diagram, OLE, media, and embedded package resource structs
  should be shared or mirrored so XLSX can consume the same renderer-facing
  resource shape as PPTX without borrowing PPTX slide state.
- PDF paint primitives, image embedding, text metrics, link annotations, and
  chart/diagram renderer bridges should live below document-format semantics.

Boundaries that must not be shared:

- PPTX placeholder lookup, master/layout inheritance, `ShapeLocation`, and
  slide persist traversal.
- DOCX Writer frame pagination, footnote/table/fly-frame influence, section
  header/footer inheritance, and paragraph layout.
- XLSX worksheet anchor resolution, Calc print area/page-break/zoom logic,
  print titles, sheet visibility, and Sc output-data paint order.

This mirrors LibreOffice: `sc/source/filter/oox/drawingfragment.cxx` owns
spreadsheet anchors and draw-page insertion, while `oox/source/drawingml/*`,
`oox/source/drawingml/chart/*`, `oox/source/drawingml/diagram/*`, and
`oox/source/vml/*` own shared DrawingML, chart, SmartArt, and VML internals.

---

## 5. Core State

### `ExcelImport`

Mirror the role of LibreOffice's workbook-global import state.

Important fields:

- `workbook`
- `themes`
- `styles`
- `shared_strings`
- `worksheets`
- `defined_names`
- `tables`
- `pivot_caches`
- `pivot_tables`
- `external_links`
- `connections`
- `persons`
- `document_properties`
- `drawing_resources`
- `formula_buffer`
- `active_sheet`
- `calculation_settings`
- `view_settings`

Important methods:

- `import_document`
- `get_current_theme`
- `get_palette_color`
- `get_scheme_color`
- `get_style`
- `get_shared_string`
- `get_sheet_by_workbook_index`
- `get_sheet_by_calc_index`

Theme/style import order must match LibreOffice: theme first, default table
styles from the theme next, styles next, shared strings after styles.
The current Rust importer has the typed `StylesCatalog` and
`DefinedNamesCatalog` entry points wired. `_xlnm.Print_Area`,
`_xlnm.Print_Titles`, and `_xlnm._FilterDatabase` are classified and attached
to print pages. Simple A1 print-area and print-title row/column references are
resolved in `xlsx/print.rs`; full formula-token range extraction remains a
structured gap until the Calc formula parser owner lands. Theme/default-table-
style materialization also remains a structured gap before `StylesCatalog`.

### `WorkbookFragment`

Required state:

- ordered `sheets` from `workbook.xml/sheets/sheet`
- sheet name, id, state, relationship id, path, and type
- workbook settings and calculation properties
- workbook views and active tab
- defined names, including `_xlnm.Print_Area`, `_xlnm.Print_Titles`,
  filters/database ranges, and local sheet ids
- external references, connections, custom XML, xmlMaps, revisions
- pivot cache registry and table registry

`finalize_import` must keep LibreOffice's sequencing even when early Rust only
renders a subset. In particular:

1. create all worksheet globals/fragments before importing sheet data
2. finalize tables and defined names before loading worksheet formulas
3. import all worksheets before final formula/pivot/scenario conversion
4. finalize drawing import after sheet data and document objects exist

### `Worksheet`

Required state:

- workbook index and Calc sheet index
- sheet name, visibility, relationship path, sheet type
- dimension hint and actual used area
- sheet properties, tab color, outline settings, fit-to-page mode
- sheet views, freeze/split panes, selections
- default row/column dimensions
- column spans and row metadata
- cells, merged ranges, validations, conditional formats, hyperlinks
- auto filters, tables, query tables, scenarios
- page settings, page breaks, print options, print ranges, repeated rows/cols
- drawings, VML drawings, comments, threaded comments, controls, OLE objects

Do not drop hidden rows/columns at import. Visibility affects layout and print
range calculation, not cell storage.

### `Cell`

Required state:

- address and style index
- raw cell type (`n`, `b`, `e`, `str`, `s`, `inlineStr`, `d`)
- cached value
- display value after number-format/style resolution
- formula record, when present
- rich text runs for inline/shared strings
- comments and hyperlinks
- effective style id and resolved format

Formula cells must preserve both formula identity and cached result. PDF output
can start with cached results, but the model must still know whether the value
came from normal, shared, array, or data-table formula import.

### `Chartsheet`

Chartsheets are workbook sheets with their own fragment and sheet identity.
They must not be flattened into worksheet cells or skipped during sheet-order
traversal.

Required state:

- workbook index and Calc sheet index
- sheet name, visibility, relationship path, and sheet type
- chartsheet properties/protection
- chart sheet views
- page margins/setup/header-footer
- drawing relationship and drawing resource snapshot
- page style and print settings

`ChartsheetFragment` owns chartsheet XML dispatch. It imports `sheetPr`,
`sheetViews/sheetView`, `pageMargins`, `pageSetup`, `headerFooter`, and
`drawing`, then finalizes through the same worksheet-helper infrastructure
where Calc does. Do not process a chartsheet as an empty worksheet plus a loose
chart object; its page setup and drawing are the printable page.

---

## 6. Worksheet Import Rules

`WorksheetFragment::onCreateContext` shows the import owner for the main
worksheet children. Keep this dispatch typed through `x::Worksheet` and
worksheet-part relationships where the SDK exposes them; extension contexts
remain structured records when visible behavior is not yet ported.

| Element | Owner |
|---------|-------|
| `sheetPr` / `pageSetUpPr` | `WorksheetSettings` and `PageSettings::setFitToPagesMode` |
| `dimension` | `WorksheetFragment` / used-area hint |
| `sheetViews` / `sheetView` / `pane` / `selection` | `SheetViewSettings` |
| `sheetFormatPr` | worksheet default row/column dimensions |
| `cols` / `col` | column formatting, width, hidden, outline |
| `sheetData` | `SheetDataContext` |
| `mergeCells` | merge range model |
| `conditionalFormatting` | `CondFormatContext` |
| `dataValidations` | `DataValidationsContext`, including MCE list fallback |
| `autoFilter` | `AutoFilterContext` |
| `hyperlinks` | worksheet relationship-scoped hyperlink model |
| `rowBreaks` / `colBreaks` | manual page-break model |
| `printOptions` | `PageSettings` |
| `pageMargins` | `PageSettings` |
| `pageSetup` | `PageSettings` |
| `headerFooter` | `PageSettings` |
| `drawing` | `DrawingFragment` |
| `legacyDrawing` | VML/comment/control drawing |
| `oleObjects` / `controls` | worksheet object/control model |
| `extLst` | extension records, not silent discard |

Relationship-scoped imports:

- normal drawing and VML drawing paths are stored on the worksheet helper and
  imported during worksheet finalization
- comments import after VML/DFF callout shapes have created note captions
- threaded comments import after ordinary comments create the `ScPostIt`
  records they extend
- table, query table, pivot table, control, OLE, hyperlink, image, chart, and
  diagram relationships must be resolved against the owning worksheet or nested
  drawing/chart part, not a workbook-global `rId` map

`SheetDataContext` specifics:

- row `r` is 1-based; implicit rows advance from the previous row
- cell `r` is optional; implicit cells advance from the previous column
- row height `ht` is in points and, for MSO documents, LibreOffice rounds down
  to a 0.75pt multiple
- row `s` is default style index 0 when absent
- cell `s` is `-1` when absent and resolved through row/column/default style
- cell data type defaults to numeric
- formulas are processed before plain value fallback
- empty `<f/>` without `ca="1"` is skipped; empty `<f ca="1"/>` is a matrix
  reference placeholder
- formula cached values are stored for formula cells but must not replace the
  formula record
- shared formulas require the shared id and valid formula range
- array/data-table formulas create range-level formula records and blank cell
  formatting where appropriate
- inline strings finalize through rich-string import, not plain text joining

---

## 7. Styles, Number Formats, and Colors

Style resolution must mirror `StylesBuffer`, not a local renderer shortcut.

Required state:

- default font model
- indexed palette
- theme color lookup and tint transformations
- number format table, including built-ins and custom codes
- font table
- fill table
- border table
- `cellStyleXfs`
- `cellXfs`
- named cell styles
- `dxfs`
- table styles
- apply flags and inheritance chain

Required behavior:

- resolve a cell style from cell XF, row style, column style, and default style
  in the same owner/order as Calc
- apply `applyNumberFormat`, `applyFont`, `applyFill`, `applyBorder`,
  `applyAlignment`, and `applyProtection` flags when inheriting from style XFs
- resolve theme/indexed/RGB colors through Excel theme/palette helpers
- keep tint/shade arithmetic in widened intermediates, as in the PPTX color
  lessons; debug/release overflow differences are rendering bugs
- implement number-format display before asserting PDF text parity
- preserve rich text run formatting separately from cell XF defaults

Number formatting is a rendering-critical import behavior. Do not render raw
`<v>` values into tests unless the test is explicitly for raw cached values.

---

## 8. Names, Tables, Filters, and External Data

These features are easy to misclassify as non-visual, but they drive print
layout and visible cell data.

### Defined Names

`DefinedNamesBuffer` owns workbook and sheet-local names:

- `_xlnm.Print_Area`
- `_xlnm.Print_Titles`
- `_xlnm._FilterDatabase`
- criteria and extract ranges for advanced filters
- user names and external-link names

Import rules:

- preserve original name, final Calc name, local sheet id, hidden/function/VBA
  flags, formula text, token index, and converted formula tokens
- create name objects before converting formulas that refer to names
- convert formulas with external link info after external links are known
- apply built-in names to print ranges, repeated titles, and filter ranges in
  the same owner; do not let `PrintFunc` parse defined-name formula strings

### Tables and AutoFilters

`TableBuffer` and `AutoFilterBuffer` own structured tables and filter state:

- table import creates database ranges and stores the token index used by
  formulas and filters
- table columns finalize after the database range exists
- table autofilters are applied after table/database-range creation
- worksheet-level autofilter can fall back to advanced filter ranges through
  criteria/extract defined names
- Excel permits filter conditions that cannot be represented as a simple list
  of independent fields; preserve the structured filter model even if early
  PDF output only needs visible rows

Print/layout implications:

- filters can hide rows and therefore change visible page slices
- table header/totals rows, column names, and styles can affect visible cell
  formatting
- table ranges are not substitutes for worksheet print areas unless the
  defined-name/page setup path says so

### External Links, Connections, Query Tables, Pivots, and Scenarios

Preserve these as typed model state before visible fallback:

- external workbook path/type, external sheet names, cached external values,
  external defined names, and formula link info
- workbook connections and query table identities
- pivot caches and pivot tables, including destination ranges and style info
- scenarios, including generated hidden scenario sheets and cell values
- slicers/timelines/sparklines extension records

Early PDF may render cached values only, but import must keep the dependency
graph needed by formulas, charts, pivots, filters, and print ranges. Do not
drop these records because the first display bridge cannot refresh data.

---

## 9. Page Settings and Header/Footer

`PageSettingsModel` defaults from LibreOffice:

- left/right margin: `0.748in`
- top/bottom margin: `0.984in`
- header/footer margin: `0.512in`
- paper size index: `1`
- scale: `100`
- first page: `1`
- fit width/height: `1`
- horizontal/vertical DPI: `600`
- page order: `downThenOver`
- print errors: `displayed`

Do not replace these with local constants unless copied into the same owner.

`PageSettings::finalizeImport` creates a sheet-specific page style named from
the sheet and writes page settings properties through the page-settings
converter before assigning the style to the Calc sheet. The print path reads
the resulting page style, not raw worksheet XML. Keep this page-style boundary
even if the Rust model stores it as a struct instead of UNO styles.

Required state:

- print options: horizontal/vertical centering, grid lines, row/column headings
- margins: left, right, top, bottom, header, footer
- page setup: paper size, custom paper width/height, orientation, copies, scale,
  first page, fit-to-width/height, DPI, page order, comments, print errors,
  black/white, draft, use-first-page-number, valid-printer-settings flag
- header/footer: odd/even/first content, different odd/even, different first
- header/footer picture relationship data
- sheet page style name and generated page style properties

Header/footer parsing must follow LibreOffice's token model:

- `&L`, `&C`, `&R` switch left/center/right portions
- `&P`, `&N`, `&A`, `&F`, `&Z`, `&D`, `&T` create dynamic fields
- `&B`, `&I`, `&U`, `&E`, `&S` toggle character attributes
- `&"font,style"` changes font/style
- `&K` supports RGB and theme+tint color syntax
- line breaks contribute to measured header/footer height

Early display lowering may render field placeholders, but import must preserve
field identity so page number/page count/sheet name/date/time can be refreshed
once page count is known.

---

## 10. Print Layout

`ScPrintFunc` is the XLSX-to-PDF layout authority.

Internal target:

```text
CalcPrintDocument
  sheets: Vec<CalcSheet>
  pages: Vec<CalcPrintPage>

CalcPrintPage
  sheet_index
  page_number
  page_size
  page_rect
  print_range
  repeat_rows
  repeat_cols
  zoom
  items
```

Required stages:

1. `init_param`: read page style, margins, headers/footers, print flags, scale
   flags, print area, repeat rows/cols, and page order.
2. `adjust_print_area`: use explicit print ranges when present; otherwise find
   used area, including printable drawings when Calc would extend the print
   area.
3. `count_pages`: count each print range or implicit used range.
4. `calc_zoom`: apply scale-to-pages, scale-to-width/height, explicit page
   scale, or 100%.
5. `calc_pages`: update page breaks and compute horizontal/vertical page ranges.
6. `print_page`: lower header/footer, repeated rows/cols, row/column headings,
   grid lines, cell content, cell backgrounds/borders, notes/comments, and
   drawings in Calc paint order.

LibreOffice page-range behavior to preserve:

- visible rows/columns only determine page splits
- hidden rows/columns do not create visible page progress
- manual row/column page breaks are honored after visibility filtering
- skip-empty mode hides empty horizontal page slices
- page order is `downThenOver` or `overThenDown`
- repeated rows/columns are inserted into every page before the page-local
  content range
- grid lines and row/column headings are print options, not worksheet view flags
- page scaling feeds both page breaks and painting transforms

Do not reuse DOCX flow layout for spreadsheet pages. XLSX print layout is a
grid/page-break problem, not a paragraph/frame pagination problem.

### Units and Coordinate Ownership

LibreOffice Calc print and drawing code crosses several unit systems. Keep the
conversion owners explicit:

- worksheet storage uses row/column dimensions and Calc document twips for
  cell geometry
- `GetMMRect` / draw-page sizing gives spreadsheet drawing anchors their
  1/100 mm reference rectangle
- `ShapeAnchor` converts between 1/100 mm and EMUs for xdr anchors, including
  one-twip endpoint adjustments
- `ScPrintFunc::InitModes` owns print map modes, zoom, manual zoom, twip mode,
  offset mode, and 1/100 mm output mode
- `ScPrintFunc::GetDocPageSize` owns page-body size after margins,
  headers/footers, and zoom are applied
- final PDF items use points only after Calc print geometry has been resolved

Do not introduce an XLSX-wide "points as model units" shortcut. Points are a
PDF lowering unit here, not the Calc import/layout unit.

Current `LayoutDocument` is DOCX-shaped in several fields (`follows`, Writer
frames, reflow/restart records, `docx::FormWidget`). XLSX may use it as a
transport into `render::krilla`, but the Calc print owner should produce a
separate `CalcPrintDocument`/display list first and only then adapt to the
shared `PageItem` paint primitives. Do not let DOCX layout bookkeeping become
part of the XLSX model.

### `ScPrintFunc` Trunk

Keep the Rust print owner shaped like LibreOffice's `ScPrintFunc`, not a
spreadsheet-specific shortcut inside the PDF renderer.

`InitParam` is the first owner for print state:

- read page margins from the page style item set and clamp negative left/right
  margins to zero in this owner
- read page usage, orientation, field number type, horizontal/vertical
  centering, and page size; the fallback paper size belongs here
- read and measure header/footer item sets through the header/footer height
  update path
- read scale mode from page scale, scale-to-size, and scale-to-page-count
  items; preserve the distinct `none`, `all`, `to width/height`, and
  `to page count` branches
- read table print flags for cell content, notes, grid, row/column headers,
  formulas, zero values, charts, objects, drawings, top-down/left-right page
  order, first page number, skip-empty, and forced breaks
- choose the print area using LibreOffice priority: user area/selection first,
  explicit sheet print ranges second, visible whole-sheet used area last
- hidden sheets do not print through the implicit whole-sheet path; a visible
  sheet without explicit ranges prints its used area
- read repeated rows and columns from document print title ranges

`UpdatePages` must reset zoom to LibreOffice's branch default, apply repeated
row/column areas, and reset print breaks on every sheet sharing the same page
style name. Do not restrict this to the current sheet.

`CountPages` and `CalcZoom` must stay range-aware:

- explicit multi-area print ranges run page adjustment and zoom calculation per
  print range
- skip-empty counting uses visible page-row slices instead of raw horizontal by
  vertical page multiplication
- scale-to-page-count, scale-to-width/height, explicit scale, and default 100%
  scale are separate branches
- manual breaks constrain fit-to-page calculations before final automatic page
  range calculation
- LibreOffice's `tdf#103516` fit-to-width adjustment is upstream-owned
  behavior; document and port the branch as-is rather than introducing a local
  replacement constant

The current Rust `CalcPrintDocument` records these branches and emits page
models in LibreOffice traversal order. Calc row/column metric coordinates and
LibreOffice's MS `paperSize` table now drive page-body dimensions, automatic
page slices, manual-break slices, repeat-row/repeat-column reserved space,
fit-to-width/height zoom, and display placement. Exact `CalcPages` still needs
the remaining scale-to-page-count branch and the full Calc hidden-page/notes
behavior before broad fit/skip-empty assertions should be treated as final.

`DoPrint` owns page traversal:

- apply print settings and output map modes before constructing page strings
- traverse page ranges according to `downThenOver` or `overThenDown`
- filter by selected PDF page ranges
- print note pages after sheet pages when note printing is enabled

`PrintPage` and `PrintArea` own visible page construction:

- account for RTL sheet layout and mirrored margins
- insert repeated rows/columns before page-local content
- compute horizontal/vertical centering from content, repeats, row/column
  headers, borders, and shadows
- paint headers/footers before the sheet body
- paint repeated row/column intersections, repeated columns, repeated rows, and
  page-local data in Calc order
- route cell painting through the `PrintArea -> FillInfo -> ScOutputData`
  printer path, including back drawing layer, cell/background/text/grid passes,
  and subsequent drawing-layer passes
- grid and page-break line painting must account for hidden rows/columns,
  merges, covered cells, and page breaks through the Calc output-data model,
  not by drawing a uniform rectangle grid

The current paint bridge records the `PrintArea` operation order (back drawing
layer, repeated columns/rows, cell area, grid, front drawing layer), lowers
page-local and repeated-title cell text through Calc row/column metrics,
expands merged master-cell rectangles and suppresses covered merged cells,
paints resolved font/fill/outer-border state from the stylesheet catalog,
paints grid lines and row/column headings when print options request them,
emits worksheet hyperlinks as `LinkArea` items from relationship-scoped
targets, paints spreadsheet pictures from imported drawing `ImagePart` bytes,
and emits anchored xdr shape rectangles from imported anchors plus resolved
solid fill/solid line/no-line paint. Chart output, SmartArt/VML/comments,
conditional formatting, theme/indexed/scheme color resolution, preset geometry
paths, diagonal/inner border conflict behavior, and complete front/back
drawing-layer paint must be fed from the imported worksheet/drawing resources;
display lowering must not reopen OOXML relationships to synthesize them.

---

## 11. Cell Paint and Text

Cell display lowering must be based on resolved Calc cell state:

- page-local cell rectangle from cumulative column widths and row heights
- merged range rectangle from master cell and covered cells
- background fill, pattern/gradient state, and conditional-format overlays
- borders, including diagonal borders and conflict behavior
- horizontal and vertical alignment
- text rotation and stacked text
- wrap text, shrink-to-fit, indentation, justify/distributed modes
- rich text runs and phonetic text markers
- number-format display string
- hyperlinks and comments

Initial implementation order should be conservative:

1. exact grid geometry, page setup, print range, and text values
2. resolved number formats and basic font/fill/border paint
3. merged cells and alignment/wrap
4. conditional formatting and rich text run styling
5. rotated/stacked text, shrink-to-fit, and precise text clipping

Do not add a renderer-only width heuristic for Excel column widths. Port the
Calc conversion path that depends on default font metrics and column width
units, and keep unresolved metric differences visible in tests.

---

## 12. Drawings, Charts, and Objects

Spreadsheet drawings are anchored to sheet coordinates, not page coordinates.
`DrawingFragment` is the owner for:

- `absoluteAnchor`
- `oneCellAnchor`
- `twoCellAnchor`
- `editAs`
- `from` / `to` cell positions
- `colOff` / `rowOff`
- shape position/size from anchor rectangle
- rotation-specific anchor rectangle correction
- visibility from anchor validity
- cell anchoring/resizing behavior
- shape bounding box extension for print area

LibreOffice XLSX drawing flow to preserve:

```text
WorksheetFragment
  -> drawing relationship
  -> DrawingFragment(wsDr)
     -> ShapeAnchor(absoluteAnchor / oneCellAnchor / twoCellAnchor)
     -> xdr shape context (sp / cxnSp / pic / graphicFrame / grpSp)
     -> compute anchor rectangle from sheet geometry
     -> apply twoCellAnchor editAs size behavior
     -> apply Excel rotation anchor-rectangle correction
     -> set shape position and size before addShape/createAndInsert
     -> use shared DrawingML/VML/chart/diagram import for internals
     -> extend worksheet shape bounding box for print area
     -> set cell anchoring and resize-with-cell behavior
```

The anchor rectangle is spreadsheet state. The shape internals are DrawingML
state. Keep this split even if the first Rust structs store both in one
worksheet-owned record.

Required structured records:

| Area | Preserve before visible fallback |
|------|----------------------------------|
| pictures | relationship id, owning worksheet path, image bytes/content type, crop, transform, alt text |
| shapes | service name, geometry, fill, line, text body, macro/textlink flags |
| connectors | endpoints and shape ids |
| charts | chart relationship id, chart space, chart drawing, embedded workbook/package, chart-local theme/style/color resources |
| SmartArt/diagram | diagram relationship ids and ext drawing state |
| VML/comments | client data, note shape mapping, object type, anchor, text |
| controls | object type, macro/event metadata, linked cell/list fill range |
| OLE/package | relationship id, object/progId/icon/preview metadata |
| hyperlinks | relationship-scoped URL plus internal targets |

Do not resolve drawing anchors in display lowering by re-reading XML. The
anchor rectangle and shape model must be created during XLSX drawing import.

### Shared DrawingML Resource Import

XLSX should reuse the PPTX DrawingML lessons but with worksheet-owned resource
scope:

- `xdr:graphicFrame` dispatch must use exact `a:graphicData/@uri`, matching
  LibreOffice `GraphicalObjectFrameContext`; do not use substring matching.
- DrawingML table payloads must come from typed `GraphicDataChoice::Table` /
  `a::Table`, not from `xml_children`.
- Chart and chartEx payloads must snapshot the relationship-scoped
  `ChartPart`/`ExtendedChartPart` resource from the owning worksheet,
  chartsheet, chart drawing, or nested chart context before display lowering.
- Chart resources must include chart space, chart drawing/user shapes, embedded
  workbook/package, chart-local images, theme override, chart style, and chart
  color style when the relationships exist. LibreOffice's chart converter does
  not treat `c:chartSpace` alone as complete.
- Excel charts need an `ExcelChartConverter`-shaped data provider layer. Formula
  data sequences are sheet formulas resolved through the workbook formula
  parser/current sheet, while literal series data stays as chart-local cached
  data. Do not route XLSX charts through a presentation-only converter.
- SmartArt/diagram records must preserve dm/lo/qs/cs relationship ids and the
  resolved typed diagram resources: data points/connections, layout
  definitions, style/color transforms, persisted drawing shape trees, child
  images, and extension markers. Excel has a specific `tdf#83671` path where
  diagram import may need the anchor-derived size before regenerating shapes;
  model this as a worksheet drawing finalization branch, not as display repair.
- VML/comment/control/OLE drawing state must stay in worksheet/VML owners. VML
  can share parsing concepts with DOCX/PPTX, but comment/control anchoring,
  macro links, linked-cell/list-fill ranges, and object visibility are Calc
  worksheet state.
- Picture `a:blip` resources are relationship-scoped to the owning worksheet,
  chartsheet, chart drawing, or VML part. Linked images should keep link
  identity; do not fetch external resources in PDF rendering.

Theme and color resolution for drawing objects should use shared DrawingML
theme logic where possible, but through the Excel owner:

- `ExcelFilter::getCurrentTheme` and `ExcelGraphicHelper` bridge workbook theme,
  palette, scheme colors, and chart/theme consumers.
- The workbook theme is imported before styles and before worksheet fragments.
- Spreadsheet drawing shape style refs, fill/line/effect refs, chart formatter
  theme lookups, and conditional-format DrawingML colors must all resolve
  through the workbook/current-sheet context, not a standalone color shortcut.
- Preserve unresolved color/fill/line/effect variants even when display cannot
  paint them yet. The XLSX display bridge should consume cached actual
  properties, like PPTX, rather than recomputing inheritance or theme refs.

### Comments, Controls, and Form Widgets

Calc comments and controls are drawing-backed objects:

- legacy comments use worksheet comments plus VML note shapes; note text and
  note shape identity must be joined by cell position
- threaded comments use workbook persons and sheet threaded-comment fragments,
  and are finalized after ordinary comments
- form controls preserve object type, drop style, macro/event metadata, linked
  cell/list-fill range, caption text, font, foreground/background colors, and
  cell anchor; relationship-owned payloads preserve control persistence
  binaries, ActiveX binary children, and typed `x14:formControlPr`
- OLE/package objects preserve linked/embedded identity, relationship ids,
  preview/icon metadata, and embedded data bytes

`render::form_widgets` can be reused for final PDF widget annotations only
after XLSX import/layout has produced Calc control geometry and field state.
Do not create PDF annotations by reading VML or control XML in the render
module.

---

## 13. Structured Fallbacks

Unsupported areas must remain modeled:

- pivot caches and pivot tables
- slicers/timelines
- query tables and connections
- external links
- data validations
- conditional formatting
- sparklines
- scenarios
- threaded comments and persons
- revision headers
- custom XML and XML maps
- macros/VBA project relationship metadata
- rich values, rich styles, arrays, feature property bags, model3D, web
  extensions, slicer/timeline caches, and modern extension-list payloads

Visible PDF output may temporarily skip unsupported features, but the import
model must preserve enough identity for later implementation without reparsing
package relationships from `display.rs`.

Fallback records should store at least: owning part path/id, relationship id,
relationship target/content type when available, typed root payload when
generated, and raw opaque extension payload only when no typed payload exists.

---

## 14. Tests and Fixtures

Follow the PPTX/DOCX calibration pattern:

1. Import model snapshot: sheet order, visibility, active sheet, styles,
   shared strings, page settings, print areas, dimensions.
2. Layout snapshot: print page count, page size, page ranges, zoom, repeated
   rows/columns, cell rectangles, drawing bounds.
3. Display-list snapshot: text, grid/border/fill/path/image objects and z-order.
4. PDF observation: page count, text geometry, image/path bounds, links, raster
   checksums.

Preferred upstream fixture sources:

- `../core/sc/qa/unit/data/xlsx/`
- `../core/sc/qa/unit/data/functions/`
- `../core/sc/qa/extras/`
- `../core/sc/qa/filter/`
- `../core/vcl/qa/cppunit/pdfexport/data/*.xlsx`

Use `// Source:` comments for assertions projected from LibreOffice tests.

Add a mapped fixture lane analogous to PPTX/DOCX:

```text
crates/ooxmlsdk-pdf-test/tests/mapped_xlsx_pdf_fixtures.rs
test-data/ooxmlsdk-pdf-test/libreoffice/xlsx/
```

Reference generation should use LibreOffice headless:

```text
soffice --headless --convert-to pdf --outdir <tmp> <fixture.xlsx>
```

Do not add broad tolerances for behavior differences. Normalizers are allowed
only for PDF container-level differences or proven PDF extraction artifacts.

### PDF Backend Boundaries

The existing backend already supports text, images, paths, links, outlines,
form-widget annotation injection, clipping, transforms, and basic chart/diagram
bridges. XLSX should lower to those paint primitives only after Calc import and
print layout are complete.

Backend rules:

- `render::krilla` must not know workbook, worksheet, table, chart, or drawing
  relationships
- link annotations consume resolved URL/internal targets and geometry from the
  XLSX display list
- form widgets consume resolved control state and geometry from XLSX layout
- chart/diagram render helpers consume structured chart/diagram resources, not
  package paths or raw XML
- page size, page labels, outline entries, and metadata must be supplied by the
  XLSX print/export owner when implemented; do not infer them in krilla

`../typst` remains useful for fixed-page paint discipline, clipping, image
embedding, glyph output, and link/destination mechanics. It must not define
Calc page breaks, print range selection, table/filter semantics, or chart data
provider behavior.

---

## 15. Anti-Patterns

Avoid:

1. Iterating worksheet package parts instead of workbook sheet order.
2. Starting at worksheet fragments or worksheet parts without the
   `ExcelFilter::importDocument -> WorkbookGlobals -> WorkbookFragment` trunk.
3. Rendering raw `<v>` values without style/number-format resolution.
4. Treating the used-range `dimension` as authoritative when actual cells or
   drawings extend the printable area.
5. Dropping formulas once cached values are read.
6. Parsing headers/footers as literal text.
7. Reusing DOCX paragraph layout for spreadsheet pagination.
8. Letting display code inspect `workbook.xml`, `styles.xml`, worksheet rels,
   drawing rels, or shared strings.
9. Resolving theme colors without the Excel theme/palette owner.
10. Flattening merged cells before retaining master/covered-cell identity.
11. Ignoring hidden rows/columns during import instead of layout.
12. Adding gridline/header behavior from worksheet view settings instead of
    print options.
13. Hardcoding page size/margins/scale defaults outside `PageSettings`.
14. Treating drawings as page-level absolute objects before sheet anchor
    conversion.
15. Calculating page breaks only for the current sheet when LibreOffice resets
    breaks for all sheets sharing the page style.
16. Drawing grid lines as a uniform visible-cell rectangle grid instead of
    using the Calc output-data path that knows hidden rows/columns, merged
    cells, covered cells, and page breaks.
17. Parsing raw OOXML for workbook, worksheet, drawing, chart, table, theme, or
    shared-string data when generated `ooxmlsdk` types and part relationships
    already expose the structure.
18. Moving shared DrawingML behavior into XLSX-only code because the first
    fixture is spreadsheet-specific. Port or extract the shared owner instead.
19. Moving spreadsheet anchor or print-area behavior into generic DrawingML
    code because the child shape is shared. Anchor resolution belongs to XLSX.
20. Resolving chart, SmartArt, OLE, or media relationships from `display.rs` by
    final `rId`; inherited/nested resources need the owning part context.
21. Treating `c:chartSpace` as a complete chart resource without chart drawing,
    embedded workbook/package, local images, theme override, style, and color
    style relationships.
22. Reusing PPTX slide/master placeholder state or DOCX Writer frame layout as
    XLSX semantics.
23. Treating chartsheets as hidden or empty worksheets instead of
    `ChartsheetFragment`-owned printable sheets.
24. Applying filters after page-break calculation; row visibility must be known
    before print page slices are counted.
25. Finalizing comments before VML note shapes or threaded comments before
    ordinary comments.
26. Dropping external-link cached values, query tables, pivots, scenarios, or
    extension records because PDF output initially renders cached cells only.
27. Adding workbook/worksheet/package knowledge to `render::krilla`,
    `render::chart`, `render::diagram`, or `render::form_widgets`.
28. Using typed child-part iteration as visible sheet order. It is a resource
    discovery API, not workbook sheet order.
29. Converting Calc geometry to PDF points before row/column/page-break,
    header/footer, zoom, and drawing-anchor owners have finished.
30. Reusing `LayoutDocument` Writer frame/reflow fields as XLSX model state.
    Adapt Calc print pages to paint primitives only at the final bridge.
31. Reading raw page setup XML in print layout after `PageSettings` has already
    created the sheet page style.
32. Running drawing import without the workbook-level draw-layer lifecycle.
33. Using tests to enshrine the current text-only smoke output.

---

## 16. Development Checklist

Before implementing an XLSX PDF feature, identify the LibreOffice owner:

| Feature | LibreOffice source area |
|---------|-------------------------|
| XLSX package entry and document import setup | `sc/source/filter/oox/excelfilter.cxx` |
| workbook traversal and finalize order | `sc/source/filter/oox/workbookfragment.cxx` |
| workbook-global state | `sc/source/filter/oox/workbookhelper.cxx` |
| typed package relationship graph | `crates/ooxmlsdk/src/parts/*_part.rs` generated SDK parts |
| worksheet XML dispatch | `sc/source/filter/oox/worksheetfragment.cxx` |
| row/cell import | `sc/source/filter/oox/sheetdatacontext.cxx`, `sheetdatabuffer.cxx` |
| styles, XF, fonts, fills, borders | `sc/source/filter/oox/stylesbuffer.cxx` |
| theme/palette colors | `sc/source/filter/oox/themebuffer.cxx`, `stylesbuffer.cxx` |
| number formats | `sc/source/filter/oox/numberformatsbuffer.cxx` |
| page settings and headers/footers | `sc/source/filter/oox/pagesettings.cxx` |
| print ranges / titles | `sc/source/filter/oox/defnamesbuffer.cxx` |
| tables/autofilters | `sc/source/filter/oox/tablebuffer.cxx`, `autofilterbuffer.cxx` |
| external links/connections/query tables | `sc/source/filter/oox/externallink*.cxx`, `connectionsfragment.cxx`, `querytable*.cxx` |
| conditional formatting | `sc/source/filter/oox/condformatbuffer.cxx` |
| data validation | `sc/source/filter/oox/worksheetfragment.cxx` |
| chartsheets | `sc/source/filter/oox/chartsheetfragment.cxx` |
| comments/threaded comments/persons | `sc/source/filter/oox/commentsbuffer.cxx`, `threadedcommentsfragment.cxx`, `personsfragment.cxx` |
| scenarios | `sc/source/filter/oox/scenariobuffer.cxx`, `scenariocontext.cxx` |
| pivot caches/tables | `sc/source/filter/oox/pivotcachebuffer.cxx`, `pivottablebuffer.cxx` |
| worksheet drawings | `sc/source/filter/oox/drawingfragment.cxx` |
| shared DrawingML shape internals | `oox/source/drawingml/shape.cxx`, `shapecontext.cxx`, `graphicshapecontext.cxx` |
| DrawingML graphic frames and tables | `oox/source/drawingml/graphicshapecontext.cxx`, `drawingml/table/*.cxx` |
| charts and chart data provider | `oox/source/drawingml/chart/*.cxx`, `sc/source/filter/oox/excelchartconverter.cxx` |
| SmartArt/diagram | `oox/source/drawingml/diagram/*.cxx`, `oox/source/shape/ShapeContextHandler.cxx` |
| VML/comments/controls | `oox/source/vml/*.cxx`, `sc/source/filter/oox/drawingfragment.cxx` |
| Calc document/sheet storage | `sc/source/core/data/document*.cxx`, `table*.cxx` |
| print layout/page breaks | `sc/source/ui/view/printfun.cxx`, `sc/source/core/data/table5.cxx` |
| cell/grid painting | `sc/source/ui/view/output*.cxx` |

Recommended implementation order:

1. Replace `xlsx.rs` with an `xlsx/` module tree and a structured
   `ExcelImport -> display` skeleton.
2. Port workbook traversal order, sheet identity, theme/styles/shared strings,
   and basic worksheet cell import.
3. Establish typed SDK resource catalogs for workbook, worksheet, drawings,
   charts, diagrams, images, comments, VML, and relationships. Add raw XML only
   for explicitly untyped extension payloads.
4. Port or extract shared DrawingML theme/color/fill/line/effect/text/table
   helpers from PPTX without importing PPTX slide/master state.
5. Port style/XF/number-format display strings before broad PDF text tests.
6. Port page settings, print ranges, page breaks, scaling, and `ScPrintFunc`
   page range calculation.
7. Lower basic grid/cell text/fill/border display items.
8. Add merged cells, repeated rows/columns, headers/footers, hyperlinks, and
   comments.
9. Port drawing anchors and visible pictures/shapes/charts as structured
   records with basic display fallback, keeping chart/diagram/OLE resource
   snapshots complete before painting.
10. Build the mapped XLSX PDF fixture lane and let failure clusters drive the
   remaining parity work.

Keep this document updated as implementation lands. Like PPTX, the plan should
record which LibreOffice-shaped owners are structurally aligned and which
visible bridges remain temporary.
