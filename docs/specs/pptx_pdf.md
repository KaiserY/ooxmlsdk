# PPTX PDF Rendering — LibreOffice-Aligned Design

**Status:** early LibreOffice-shaped PPTX import/model port for
`ooxmlsdk-pdf`. Visible PDF output is still an observation bridge; PPTX
semantics must live in the import and DrawingML model first.

**Primary authority:** LibreOffice `../core`.

Start with these files before changing behavior:

- `../core/oox/source/ppt/pptimport.cxx`
- `../core/oox/source/ppt/presentationfragmenthandler.cxx`
- `../core/oox/source/ppt/slidefragmenthandler.cxx`
- `../core/oox/source/ppt/layoutfragmenthandler.cxx`
- `../core/oox/source/ppt/slidepersist.cxx`
- `../core/oox/source/ppt/pptshapegroupcontext.cxx`
- `../core/oox/source/ppt/pptshapecontext.cxx`
- `../core/oox/source/ppt/pptgraphicshapecontext.cxx`
- `../core/oox/source/ppt/pptshapepropertiescontext.cxx`
- `../core/oox/source/ppt/pptshape.cxx`
- `../core/oox/source/drawingml/shape.cxx`
- `../core/oox/source/drawingml/*`

`../typst` and `krilla` are paint/layout references only. They may guide
fixed-page frames, transforms, glyph painting, image embedding, paths, and PDF
mechanics, but they must not define PPTX import semantics.

---

## 1. Non-Negotiables

Do not evolve PPTX PDF support as a direct `slide.xml -> PDF items` renderer.

Required flow:

```text
PresentationDocument
  -> PowerPointImport / PresentationFragmentHandler / SlideFragmentHandler
  -> SlidePersist / PptShape / drawingml::Shape
  -> PPTX drawing/display objects
  -> krilla paint
```

Rules:

- Port LibreOffice owner boundaries first, even when the first Rust branch only
  preserves structured fallback state.
- Renderer needs must not define import data structures.
- The display layer must not parse OOXML, inspect package relationships,
  resolve placeholders, choose service names, resolve theme colors, or perform
  master/layout inheritance.
- Preserve generated SDK enum values and typed records. Do not encode unresolved
  scheme/preset/system colors as display-looking strings.
- Unsupported charts, SmartArt, tables, media, OLE, comments, and notes must
  keep typed identity and relationship metadata before any visible fallback.
- Do not lock temporary output into tests as final behavior.
- Do not silence skeleton dead-code warnings with `#[allow]`; consume those
  fields by porting the corresponding upstream branch.

---

## 2. Current Calibration

Already structurally aligned:

- Main pipeline exists:
  `PowerPointImport -> PresentationFragmentHandler -> SlideFragmentHandler ->
  PPTShapeGroupContext -> drawingml::Shape -> display`.
- Slide/master traversal is relationship-order based: `p:sldIdLst` /
  `slides_vector` and `slide_master_vector`, not package part iteration order.
- Layout persists are cached by `(master_path, layout_path)`.
- A current `SlidePersist` context is available during background creation,
  shape creation, and theme/color lookup, including before a persist is pushed
  into `draw_pages` or `master_pages`.
- `SlidePersist` stores slide size, path identity, theme path, color maps,
  background records, slide name/visibility/master-shape visibility, shape
  tree, VML lifecycle markers, connector maps, and comments/notes slots.
- Theme import stores `a:clrScheme` and `a:fmtScheme` as structured records
  keyed by theme path. Scheme color lookup has a current-persist path through
  slide or layout color map, master color map, then theme color scheme.
- Shape actual fill/line/effect resolution now consumes theme style refs from
  `a:style` against the current theme format scheme before applying direct
  shape properties. Effect import preserves outer/inner shadow, glow, softEdge,
  and unsupported effect-list/DAG names as structural state instead of a boolean
  marker.
- Shape style refs preserve both the style-matrix index and placeholder color
  (`phClr`) for fill, line, effect, and font refs. `fontRef` without an
  explicit color defaults to scheme `tx1`, matching LibreOffice. Theme
  major/minor latin fonts from `a:fontScheme` are available to display lowering
  for `fontRef`.
- Background `bgPr` and `bgRef` are separate structured states. `bgRef` is not
  treated as a direct solid fill.
- Shape import preserves non-visual metadata, placeholder subtype/index,
  transforms, structured fill/line records, typed geometry, shape style refs,
  text bodies, picture relationship identity and embedded image bytes from the
  owning slide/layout/master relationship context, content-part relationship
  identity, grouped children, connector endpoints, and graphic-frame
  classification.
- `graphicFrame` URI dispatch uses LibreOffice's exact URI table for
  presentation OLE, DrawingML diagram, DrawingML chart, Office 2014 chartEx,
  and DrawingML table. The model preserves chart `r:id`, inline chart-space
  presence, diagram `dm/lo/qs/cs` relationship ids, and presentation OLE
  identity/progId/icon metadata before any visible fallback.
- DrawingML table graphic data is imported from typed
  `GraphicDataChoice::Table` / `a::Table`, including flags, style id, grid,
  rows, cells, spans/merges, margins, and cell text bodies.
- Table shapes follow LibreOffice's `Shape::createAndInsert` tdf#90403 branch:
  for `TableShape`, visible size comes from the DrawingML table grid column
  widths and row heights, not from `p:xfrm` extents. The display bridge now
  paints a basic table grid and lowers typed cell text bodies through the same
  PPTX text path.
- `Shape::create_and_insert` is the current owner for actual fill, line, and
  effect state. It caches resolved paint/effect records on `Shape`, including
  direct `grpFill` inheritance from the parent group fill. Actual fill, line,
  and effect precedence must stay aligned with LibreOffice:
  reference/placeholder state, theme style ref, then direct shape properties.
- Placeholder lookup has the first upstream-shaped port: default subtype,
  index-based subtype inheritance, priority lookup, `apply_shape_reference`,
  placeholder storage, and referenced marking.
- `apply_shape_reference` inherits placeholder actual line/fill/effect state
  when available, not only direct placeholder properties. Layout/master
  placeholders often obtain visible paint from theme style refs during
  `create_and_insert`; copying raw direct properties drops that resolved state.
- Shape placeholders inherit placeholder text by default, matching
  `PPTShapeContext`; graphic placeholders use the LibreOffice
  `PPTGraphicShapeContext` `bUseText=false` branch so charts, tables, pictures,
  diagrams, media, and similar objects inherit placeholder geometry/properties
  without copying placeholder prompt text.
- Text body import is typed: generated `bodyPr`, `pPr`, `rPr`, `fld/pPr`, and
  `endParaRPr` are preserved for shape text and table cell text.
- `bodyPr` now has a LibreOffice-shaped structured view mirroring
  `TextBodyPropertiesContext`: wrap/overflow, text insets, column count and
  spacing, text-area rotation, vertical writing mode, anchor/anchorCtr,
  WordArt/upright markers, and EG_TextAutofit state are captured on
  `TextBody` before any display lowering.
- `TextParagraph` owns paragraph-level style lookup and fallback to level 0.
- The temporary display bridge now consumes imported `bodyPr` text insets,
  paragraph margins/indents, master/text-list default run properties, direct
  run properties, run solid fills, and typed bullet records when lowering text
  to PDF items. It also consumes the structured bodyPr view for normal-autofit
  font scaling, line-height scaling, vertical anchors, body/vertical rotation,
  basic multi-column flow, vertical overflow clipping, and shape `fontRef`
  default text color / major-minor latin font selection. This is still paint
  plumbing: the import-side typed text state remains the source of truth.
- Table cell `tcPr` import mirrors LibreOffice's `TableCellContext` shape:
  margins, vertical text mode, anchor/anchorCtr, horizontal overflow, direct
  fill, and all six border line properties (`lnL`, `lnR`, `lnT`, `lnB`,
  `lnTlToBr`, `lnBlToTr`) are preserved on `TableCell`; display lowering paints
  direct solid cell fills and explicit solid cell borders, using the temporary
  grid fallback only when no cell has direct border properties.
- PowerPoint table styles are now structured beyond the style id. The importer
  loads the presentation `tableStyles` relationship after the main presentation
  traversal, and inline `a:tableStyle` is preserved on `TableProperties`.
  `TableStyleList` / `TableStyle` / `TableStylePart` capture default style id,
  whole-table, first/last row and column, horizontal/vertical banding, corner
  parts, direct fill, direct outline borders, `fillRef`/`lnRef` style-matrix
  references with `phClr`, and table text style font/fontRef plus
  color/bold/italic markers. The
  display bridge applies the LibreOffice `tablecell.cxx` part order for direct
  or theme-referenced solid fills, outline borders, and table text
  font/color/bold/italic defaults: whole table, first/last row and column,
  horizontal bands, corner cells, vertical bands, then direct `tcPr` and
  paragraph/run overrides. `tcTxStyle` must seed the cell text character
  defaults before paragraph `defRPr` and direct run properties, matching
  LibreOffice's `applyTableStylePart` path. Direct `tcTxStyle/font` latin
  typefaces and `fontRef` major/minor latin theme fonts are applied in the
  temporary display bridge; east Asian, complex-script, and symbol fonts stay
  preserved for the full text-engine port. Table background fill is consumed per
  cell, not as an independent painted rectangle: the display bridge blends the
  table background solid color with the final cell solid fill using the cell
  fill opacity, mirroring LibreOffice's `basegfx::interpolate(bg, cell, 1 -
  transparency)` handoff.
- Display lowering must not paint `master_page.shapes` separately when a slide
  already inherited a layout persist. `PresentationFragmentHandler` clones the
  layout/master shape tree into the slide persist before slide XML is imported,
  so an extra display pass over the master/layout page duplicates visible
  content and breaks LibreOffice z-order expectations.

Known gaps to keep visible:

- Theme color scheme and style-matrix resolution are structurally present.
  Color resolution covers RGB, percentage RGB, HSL, preset, system `lastClr`,
  scheme/theme lookup, placeholder-color fallback, and DrawingML color
  transformations, including `phClr` substitution for theme style
  fill/line/background refs, background-color fallback for `phClr`, CRGB-space
  handling for `scrgbClr` and RGB channel transforms, and fill/line alpha
  propagation to display opacity. Color transform arithmetic must use widened
  intermediates for CRGB tint/shade/mod/off, HSL offsets, and alpha mod/off;
  real LibreOffice presentation fixtures use large DrawingML percentage values
  and debug/release overflow differences here are rendering bugs, not harmless
  implementation details.
  Remaining gaps are theme overrides, extra color schemes, and exact parity for
  edge-case system/palette behavior.
- `Shape::apply_shape_reference` has the explicit `bUseText` branch and
  inherits resolved placeholder paint/effect state, but still lacks the full
  LibreOffice generic shape-property map and custom geometry behavior.
- Theme style-matrix lookup is structurally present for fill, line, effect, and
  background-fill lists, and style refs preserve `phClr`. Color transforms are
  applied after base color resolution, matching LibreOffice's broad
  `Color::getColor` order.
- Paragraph/run property application has a first visible bridge for insets,
  margins, indents, bullet labels, font size/family, bold/italic,
  underline/strike, caps, spacing, baseline, and text solid fill. It is still
  incomplete for exact LibreOffice line breaking, text anchoring, autofit,
  wrapping, columns, vertical text, paragraph spacing, tabs, field values,
  bidi/complex-script font fallback, theme font refs, and bullet numbering
  continuation.
- The bodyPr display bridge is intentionally approximate: normal-autofit scale
  is applied, but shape autofit does not resize shapes yet; vertical/rotated
  text uses fixed-page text rotation rather than LibreOffice's full writing
  mode simulation; columns do simple paragraph flow, not full edit-engine line
  layout.
- Table rendering still lacks predefined LibreOffice table style synthesis when
  a style id has no tableStyles part entry, full `tcTxStyle` east Asian /
  complex-script / symbol font application, merge-aware border suppression,
  non-solid fill/line paint, exact alpha/transparent background edge behavior,
  text-fitting expansion, and shadow/effect handling. Do not treat the current
  grid fallback or direct/theme style paint bridge as final table semantics.
- `SlidePersist::create_background`, `create_connector_shape_connection`,
  `Shape::finalize_x_shape`, grab bags, diagram helper propagation, chart,
  SmartArt, OLE, media, notes, comments, and VML remain structured slots or
  partial ports. Chart/diagram/OLE identities are now carried in the shape
  model, but their relationship target parts are not converted to visible PDF
  content yet.
- Embedded picture display is a first bridge: `a:blip/@r:embed` is resolved
  against the fragment that owns the picture before master/layout inheritance,
  then lowered to `ImageItem` with bounds, crop, rotation, flips, content type,
  and alt text. Linked images are intentionally not fetched; keep their
  relationship identity as structured state until LibreOffice's linked graphic
  lifecycle is ported.
- `display.rs` is an observation bridge. Do not add new PPTX semantic behavior
  there when the LibreOffice owner is `SlidePersist`, `PptShape`,
  `drawingml::Shape`, or a DrawingML context.
- Shared DOCX layout/PDF carriers are allowed only as temporary paint plumbing.
  PPTX semantic state must remain in PPTX types.

Recommended next work:

1. Continue `Shape::apply_shape_reference` and actual fill/line/effect/theme
   resolution from `drawingml::Shape`.
2. Port paragraph/run property application through `SlidePersist::apply_text_styles`.
3. Extend `PptShape::add_shape` / service-name selection from `pptshape.cxx`.
4. Port theme color scheme, style matrix, and table styles.
5. Fill structured imports for chart, SmartArt/diagram, media, OLE, notes,
   comments, and VML before improving their visible fallbacks.

---

## 3. Upstream Pipeline

Preserve this stage order:

```text
PowerPointImport::importDocument
  -> PresentationFragmentHandler::finalizeImport
     -> importMasterSlides
     -> importSlide
        -> SlideFragmentHandler
           -> PPTShapeGroupContext
           -> PPTShapeContext
           -> PPTGraphicShapeContext
           -> PPTShapePropertiesContext
           -> GraphicalObjectFrameContext
        -> SlidePersist::createBackground
        -> SlidePersist::createXShapes
           -> PPTShape::addShape
              -> drawingml::Shape::createAndInsert
```

Rust equivalents should stay traceable:

| LibreOffice | Rust |
|-------------|------|
| `PowerPointImport::importDocument` | `PowerPointImport::import_document` |
| `PresentationFragmentHandler::finalizeImport` | `PresentationFragmentHandler::finalize_import` |
| `PresentationFragmentHandler::importMasterSlides` | `PresentationFragmentHandler::import_master_slides` |
| `PresentationFragmentHandler::importSlide` | `PresentationFragmentHandler::import_slide` |
| `SlideFragmentHandler::onCreateContext` | `SlideFragmentHandler::on_create_context` |
| `SlidePersist::createBackground` | `SlidePersist::create_background` |
| `SlidePersist::createXShapes` | `SlidePersist::create_x_shapes` |
| `PPTShape::addShape` | `PptShape::add_shape` |
| `Shape::createAndInsert` | `Shape::create_and_insert` |

Skeleton constraints:

- `PresentationFragmentHandler::finalize_import` owns slide traversal,
  page-range shape, custom shows, sections, names, and notes-page decisions.
- `PresentationFragmentHandler::import_slide` owns layout/master resolution,
  master+layout cache lookup, slide persist creation, current persist context,
  fragment import, background creation, and shape creation.
- `SlidePersist` remains the state object passed through theme/color lookup,
  background creation, shape creation, placeholder inheritance, comments, notes,
  and VML. Do not split this into renderer convenience records.
- `SlidePersist::create_x_shapes` is the porting boundary equivalent to
  LibreOffice UNO shape creation. It may build Rust drawing records instead of
  UNO objects, but branch order and ownership must remain upstream-shaped.

---

## 4. Module Boundaries

PPTX layout support lives under `crates/ooxmlsdk-layout/src/pptx/`; PDF keeps
only the rendering adapter.

Keep these ownership boundaries:

- `pptx/import.rs`: `PowerPointImport`
- `pptx/presentation.rs`: `PresentationFragmentHandler`
- `pptx/slide_fragment.rs`: `SlideFragmentHandler`
- `pptx/layout_fragment.rs`: `LayoutFragmentHandler`
- `pptx/slide.rs`: `SlidePersist`
- `pptx/shape.rs`: `PptShape`
- `pptx/shape_group_context.rs`: `PPTShapeGroupContext`
- `pptx/shape_context.rs`: `PPTShapeContext`
- `pptx/graphic_shape_context.rs`: `PPTGraphicShapeContext`
- `pptx/shape_properties_context.rs`: `PPTShapePropertiesContext`
- `pptx/drawingml/*`: generic DrawingML helpers and model

Do not merge PPT-specific contexts into generic DrawingML modules. PPT contexts
carry `SlidePersist` and `ShapeLocation`, which are required for placeholder
lookup, inherited styles, SmartArt/ext drawing handoff, and service-name
decisions.

---

## 5. Core State

### `PowerPointImport`

Mirror `oox::ppt::PowerPointImport`.

Important fields:

- `themes`
- `draw_pages`
- `master_pages`
- `notes_pages`
- `actual_slide_persist` plus current-persist context for not-yet-pushed
  persists
- `table_style_list_path`
- `table_style_list`
- `chart_converter`

Important methods:

- `import_document`
- `get_current_theme`
- `get_current_theme_ptr`
- `get_scheme_color_token`
- `get_scheme_color`
- `get_table_styles`
- `get_vml_drawing`
- `get_chart_converter`

Color lookup order:

1. Current slide/layout color map.
2. Bound master color map.
3. Theme color scheme.

The theme color scheme is imported from `a:themeElements/a:clrScheme` and
stored with the theme path. Keep unsupported theme color models as unresolved
records instead of converting them to display strings.

The theme format scheme is imported from `a:themeElements/a:fmtScheme` and owns
fill, background-fill, line, and effect style lists. Style-matrix indexes are
1-based and clamp to the last available item, matching LibreOffice
`Theme::get*Style`; fill indexes `>= 1000` select the background-fill list with
`index - 1000`.

`presProps` is imported after presentation fragment handling; it is not
persistent `PowerPointImport` state in LibreOffice.

### `PresentationFragmentHandler`

Required state:

- `slides_vector`
- `slide_master_vector`
- `notes_master_vector`
- `slide_id_to_index_map`
- `custom_show_list`
- `section_list`
- `slide_size`
- `notes_size`
- `default_text_list_style`
- comment-author and section parsing slots

`finalize_import` must keep the page-range/custom-show/notes-page concept even
while early Rust imports all pages.

### `SlidePersist`

Central slide/master/layout state. Do not collapse into a generic page object.

Required state includes:

- path, relationship id, layout path, master path, master page index
- slide size, theme path, color map, master color map
- name, visibility, `show_master_shapes`
- background color/properties
- shape tree
- default/title/body/notes/other text styles
- shape map and connector shape map
- connector connection pass marker
- timing, header/footer, comments, comment authors
- VML drawing lifecycle state

`p:sld/@show`, `p:sld/@showMasterSp`, and `p:cSld/@name` must be imported into
`SlidePersist` during `SlideFragmentHandler`, not inferred by display lowering.

VML lifecycle must preserve two facts: a VML/legacy drawing fragment was found
before shape import, and the conversion/insertion slot ran during finalization.

---

## 6. Shape Tree

### `ShapeLocation`

Preserve the enum:

```rust
enum ShapeLocation {
    Master,
    Layout,
    Slide,
}
```

It affects placeholder lookup, visibility, and service-name selection. Do not
replace it with booleans.

### `PptShape`

Mirror `oox::ppt::PPTShape`, layered over DrawingML `Shape`.

Required state:

- `model_id`
- `shape_location`
- `referenced`
- `placeholder`
- `has_noninherited_shape_properties`

Required methods/concepts:

- `add_shape`
- placeholder lookup by subtype/index
- `get_sub_type_text_list_style`
- `is_placeholder_candidate`
- `set_text_master_styles`
- service-name selection before rendering

### Placeholder Resolution

`PPTShapeContext::on_create_context(ph)` owns placeholder handling:

1. Set subtype from generated `p::PlaceholderValues`, defaulting like
   LibreOffice.
2. Set subtype index unless it is the `SAL_MAX_UINT32` skip value.
3. If type is missing but index exists, inherit the type from layout/master.
4. For slide and layout shapes, find referenced placeholder by subtype/index.
5. Call `Shape::apply_shape_reference`.
6. Mark the source placeholder as referenced.

Lookup priority must match LibreOffice:

1. first subtype with same index
2. first subtype without same index
3. second subtype with same index
4. second subtype without same index
5. same index

Two-candidate subtype fallback:

| Requested | First | Second |
|-----------|-------|--------|
| `ctrTitle` | `ctrTitle` | `title` |
| `subTitle` | `subTitle` | `body` |
| `obj` | `obj` | `body` |
| other known placeholders | own subtype | none |

Placeholder inheritance is not ad hoc style merging and must not move to the
renderer.

### Service Names

Represent LibreOffice service decisions internally, including:

- `CustomShape`
- `GraphicObjectShape`
- `GroupShape`
- `LineShape`
- `ConnectorShape`
- `Ole2Shape`
- `TitleTextShape`
- `SubtitleShape`
- `OutlinerShape`
- `NotesShape`
- `DateTimeShape`
- `HeaderShape`
- `FooterShape`
- `SlideNumberShape`
- `PageShape`
- `ChartShape`
- `TableShape`
- `MediaShape`

Keep service-name selection in `PptShape::add_shape`,
`Shape::finalize_service_name`, or their upstream-equivalent owners. The
display layer must never infer placeholder semantics from raw XML or labels.

---

## 7. DrawingML Shape Model

The generic DrawingML layer mirrors `oox::drawingml::Shape`.

Required state:

- service name
- non-visual metadata: id, name, description, title
- hidden, hidden-master-shape, locked
- placeholder subtype/index
- position, size, child position, child size
- rotation, diagram rotation, horizontal/vertical flip
- children
- direct fill, line, effect properties
- custom shape properties: typed `custGeom` / `prstGeom`
- shape style refs: typed `lnRef`, `fillRef`, `effectRef`, `fontRef`
- picture/media records
- table properties
- text body
- master text list style
- reference fill/line/effect properties
- connector endpoint records: `stCxn` / `endCxn`
- frame type and graphic data record

Required methods/concepts:

- `set_defaults`
- `apply_shape_reference`
- `add_shape` / `add_children`
- `create_and_insert`
- `finalize_service_name`
- `finalize_x_shape`
- grab-bag preservation
- actual fill/line/effect resolution
- chart/table/diagram/OLE/media classification
- diagram helper migration/propagation

Import rules:

- Preserve `a:xfrm` and group transform state as EMU model data. Do not convert
  to PDF coordinates during import.
- Preserve `spPr/noFill`, `solidFill`, `gradFill`, `blipFill`, `pattFill`,
  `grpFill`, `ln`, `custGeom`, `prstGeom`, and style refs as structured state.
  Theme/style-matrix resolution belongs to DrawingML theme code and
  `create_and_insert`.
- Style refs are not just indexes: preserve `phClr` and transformations for
  substitution into theme style fills/lines/background refs/effects. `fontRef`
  must default to `tx1` when no color child exists, and must seed text defaults
  before paragraph and run properties are applied.
- Preserve and apply color transformations such as `tint`, `shade`, `alpha`,
  `alphaMod`, `lumMod`, `lumOff`, `satMod`, `hueMod`, RGB channel transforms,
  `gray`, `comp`, `inv`, `gamma`, and `invGamma` after resolving the color
  base. Do not render a transformed color as its untransformed base color, and
  do not drop resolved alpha before PDF lowering.
- `ln/noFill` is a real direct line property that suppresses inherited/theme
  line state. Do not represent it as missing line properties.
- Cache actual fill, line, and effect state on `Shape` during
  `create_and_insert`. Display lowering must read those cached fields instead
  of recomputing reference/direct precedence. Do not let direct effect
  properties short-circuit inherited or theme effect state; LibreOffice uses
  `assignUsed` in reference, theme, direct order.
- A preset `line` geometry may select line service for non-connector shapes,
  matching LibreOffice; do not turn geometry into PDF path data in the parser.
- `apply_shape_reference` must stay aligned with
  `drawingml::Shape::applyShapeReference`: text according to `bUseText`,
  shape properties, actual fill/line/effect state, custom geometry, table
  properties, master text list style, position, size, rotation, flips, hidden,
  and locked flags. Do not regress actual placeholder paint/effect inheritance
  back to raw direct placeholder properties.

---

## 8. Graphic Frames, Tables, and Connectors

### Graphic Frames

`GraphicalObjectFrameContext` dispatches by exact `a:graphicData/@uri`.

Known classifications:

- presentation OLE
- DrawingML diagram
- DrawingML chart
- Office 2014 chartEx
- DrawingML table
- unsupported graphic data

Do not use substring matching such as `/chart` or `/table`.

Table payload must come from typed `GraphicDataChoice::Table` / `a::Table`.
Do not resurrect quick-xml parsing of `GraphicData::xml_children` for table
cells, text, style id, or grid.

Chart, chartEx, diagram, and OLE/package targets are relationship-scoped.
Import them from the owning slide/layout/master part before the shape tree is
walked, cache them on that `SlidePersist`, and snapshot the resolved resource
onto `GraphicDataRecord` during `GraphicalObjectFrameContext` dispatch. Do not
resolve these targets later from the final slide persist by raw `rId`: inherited
master/layout graphicFrames can collide with slide relationship IDs after
cloning.

Current PPTX PDF lowering preserves structured graphicFrame identity and target
payloads for chart, diagram, OLE object, and embedded package records. Visible
chart/SmartArt/OLE rendering is still a conversion gap; the preserved payloads
are the handoff point for the renderer, not an excuse to parse package
relationships from display code.

Chart resources are not just `c:chartSpace`. LibreOffice's chart path carries
the chart drawing fragment, embedded workbook/package, chart-local images,
theme override, chart style, and chart color style into the converter. Keep
these children attached to the chart/chartex resource snapshot so later visible
chart rendering can consume a complete model without reopening relationships or
losing chart-local theme/style context.

### Connectors

`cxnSp` import stores structured `stCxn` / `endCxn` endpoint records on the
connector shape. It does not resolve endpoints.

`SlidePersist::create_x_shapes` owns:

1. applying text styles
2. creating/finalizing shapes
3. rebuilding page shape maps
4. rebuilding connector shape maps
5. applying connector connections

This mirrors `SlidePersist::createXShapes`. Do not resolve connector endpoints
while parsing the shape tree or in display lowering.

---

## 9. Theme, Color, Fill, Line, and Effects

Fill resolution order:

1. reference shape fill
2. theme fill style from `fillRef`
3. direct shape fill
4. parent group fill when direct fill is `grpFill`

Line resolution order:

1. reference shape line
2. theme line style from `lnRef`
3. direct shape line

Effects have their own actual-resolution layer. Do not fold effects into fill
or line.

Effect import belongs to shape properties and theme format-scheme import:
`spPr/effectLst`, `spPr/effectDag`, and theme `effectStyle` must all lower into
the same cached effect model. Display should consume the cached actual effect
state only. Current rendering may still skip painting shadows/glow/soft edges,
but import must retain their distances, directions, blur/radius, scale, colors,
and unsupported effect names so later lowering does not need to reparse XML.

Theme style refs from `a:style` are resolved between inherited/reference shape
properties and direct shape properties. Do not resolve `fillRef`, `lnRef`, or
`effectRef` in the parser or display layer; they need the current
`PowerPointImport`/`SlidePersist` theme context.

The style ref placeholder color (`phClr`) must travel with the style ref. It is
not equivalent to the resolved theme style color, and it should not be dropped
when the current renderer cannot yet apply substitutions or transformations.

Fill and line import must preserve unresolved structural variants even when the
current display bridge cannot paint them. Dropping `gradFill`, `blipFill`,
`pattFill`, or `ln/noFill` during import loses inheritance and theme semantics.
Display may ignore unsupported paint variants temporarily, but the model must
not.

The actual-resolution owner is `drawingml::Shape::create_and_insert`, mirroring
LibreOffice's `Shape::createAndInsert` / `getActual*Properties` flow. `display`
is only allowed to consume `Shape::actual_*_properties`; adding a fallback there
is a regression because it bypasses layout/master/theme context.

Scheme colors resolve through `PowerPointImport` and current `SlidePersist`.
Do not add a standalone `schemeClr -> RGB` shortcut disconnected from the
current slide, master, and theme.

Theme style colors that use `phClr` must be resolved with the placeholder color
from the corresponding style reference. Do not clone theme fill/line/background
style records and then resolve colors with `None`; that silently drops the
substitution LibreOffice passes into `Color::getColor`.

Background `phClr` is not only the inline `bgRef` color child. LibreOffice also
passes the slide/layout/master background color into `createBackground`; use
that inherited background color as the fallback placeholder when the background
fill/style contains `phClr`.

Do not treat `scrgbClr` as plain RGB percentages. LibreOffice stores it as CRGB
and applies RGB channel transforms, `shade`, `tint`, `inv`, `gamma`, and
`invGamma` in CRGB space before converting back to RGB.

Theme color and format-scheme import are only the first stages. Full
LibreOffice parity still requires theme overrides, extra color schemes, and
complete effect semantics. Do not treat basic `clrScheme` RGB lookup or
style-list lookup as complete theme resolution.

---

## 10. Text Model

DrawingML text should mirror LibreOffice text ownership:

- `TextBody`
- `TextBodyProperties`
- `TextListStyle`
- `TextParagraph`
- `TextParagraphProperties`
- `TextCharacterProperties`
- `TextRun`
- `TextField`

Preserve:

- body properties: insets, vertical anchor, rotation, autofit, wrap, overflow
- paragraph level and paragraph properties
- run properties
- line breaks
- field type and field paragraph properties
- end paragraph run properties
- presentation default text style
- master `txStyles`
- placeholder/list-style inheritance chain

Owner rules:

- `p:defaultTextStyle` belongs to `PresentationFragmentHandler`.
- master `p:txStyles` belongs to master import and populates `SlidePersist`.
- `PptShape::set_text_master_styles` merges subtype/master style, placeholder
  master/list style, then current shape `txBody/lstStyle`.
- `TextParagraph::get_paragraph_style` owns level lookup and fallback to level
  0. The renderer must not reimplement this.
- `SlidePersist::apply_text_styles` remains the pass that prepares paragraph
  style state.

Lowering to PDF may borrow Typst-like fixed-frame text mechanics, but must not
collapse the upstream text-style model.

---

## 11. Display and PDF Backend

The final renderer should consume PPTX drawing/display objects similar to a
fixed-page frame model:

```text
PptxDocument
  pages: Vec<PptxPage>

PptxPage
  size
  background
  frame/items

PptxItem
  Group(transform, children)
  Shape(geometry, fill, line, effects)
  TextBox(bounds, text_body, transform)
  Image(bounds, crop, transform)
  Table(table_model, transform)
  Link(bounds, target)
```

This display list is an output of the LibreOffice-aligned import/model layer,
not the source model.

`krilla` may handle PDF pages, paths, fills, strokes, image embedding, glyph
output, and annotations.

`krilla` must not handle PPTX relationships, placeholders, master/layout
inheritance, theme mapping, service-name decisions, or raw OOXML.

Current `pptx::display` may observe already imported `SlidePersist`/`Shape`
state while the drawing-object layer is incomplete. It must not become the
owner of PPTX semantics.

---

## 12. Structured Fallbacks

Unsupported or incomplete objects must keep structured records:

| Area | Preserve before visible fallback |
|------|----------------------------------|
| picture | blip embed/link relationship ids, owning-fragment image resource, bounds, crop, non-visual metadata |
| chart | frame type, chart relationship id or inline chart-space marker, bounds, text cache if available |
| SmartArt/diagram | diagram dm/lo/qs/cs relationship ids, ext drawing ids, model id, bounds |
| table | table grid/cell model, style ids, cell text bodies |
| media/contentPart | relationship id, poster/image ids, bounds |
| OLE | OLE relationship id, name, progId, icon flag, preview relationship, bounds |
| comments/notes | parsed metadata tied to slide/page identity |
| VML/control | drawing lifecycle, control metadata, relationship ids |

Visible text from these objects may be used as temporary paint only after the
structured object identity is preserved.

Media is a DrawingML shape concern, not a display fallback. LibreOffice resolves
`a:wavAudioFile/@r:embed`, `a:audioFile/@r:link`, `a:videoFile/@r:link`, and
`p14:media` against the current slide/layout/master relationships while parsing
the shape. Keep the relationship id, relationship type, package target or
external target, embedded bytes/content type when available, and contentPart
identity on the `Shape`/`SlidePersist` model. Do not infer media by inspecting
poster images in display lowering.

Comments are slide/presentation import state, not display-layer XML recovery.
LibreOffice reads legacy `ppt/commentAuthors.xml` once at presentation scope,
imports each slide's legacy comments relationship, and also preserves modern
PowerPoint 2018 comments/authors. Keep author ids/names/initials, legacy color
and last-index metadata, modern user/provider ids, comment ids, position,
created/datetime, status, tags/likes/assignments, task fields, reply counts,
plain text, and the modern text-body structure on `SlidePersist`. Do not parse
comment XML manually or resolve comments from display lowering; use the typed
SDK parts so legacy and modern relationship scopes stay distinct.

Notes are imported as their own slide persists, not as visible slide items.
LibreOffice imports the visible slide first, then follows the slide-owned
`notesSlide` relationship when notes import is enabled. The notes slide must use
`notesSz`, carry `is_notes=true`, keep its own relationship-scoped images,
media, graphicFrames, VML markers, background, color-map override, and shape
tree, and link to a notes master persist imported from the notes slide's
`notesMaster` relationship. Notes masters are master persists with
`is_notes=true`; they keep their theme, color map, header/footer flags,
`notesStyle`, background, and placeholder shapes. Do not append notes pages to
the normal draw-page PDF sequence unless an explicit export mode later asks for
notes output.

---

## 13. Tests and Fixtures

Testing order:

1. Import model snapshot: slide ids, master ids, layout ids, themes, color maps.
2. Shape model snapshot: placeholder subtype/index, service name, transform,
   fill/line/text inheritance, relationship ids.
3. Display-list snapshot: page size, z-order, bounds, transforms, item kinds.
4. PDF observation: page count, text/image/path geometry, links, validity.

Prefer LibreOffice-derived fixtures:

- `../core/sd/qa/unit/data/pptx/*.pptx`
- `../core/sd/qa/filter/data/pptx/*.pptx`
- `../core/vcl/qa/cppunit/pdfexport/data/*.pptx`

Avoid broad hand-authored PPTX files unless the behavior is named, upstream
backed, and no useful LibreOffice fixture exists.

---

## 14. Anti-Patterns

Avoid:

1. Rendering `slide.xml` directly without master/layout resolution.
2. Treating placeholder inheritance as optional style cleanup.
3. Resolving theme colors outside `PowerPointImport` and `SlidePersist`.
4. Converting unsupported objects to text before preserving identity.
5. Flattening groups before transform and child coordinate semantics exist.
6. Adding local service/type names that do not map to LibreOffice decisions.
7. Locking temporary text-only or smoke PDF output into tests.
8. Merging PPT and generic DrawingML contexts prematurely.
9. Using DOCX section/frame/page-style types as persistent PPTX semantic state.
10. Letting display or `krilla` inspect raw OOXML or resolve PPTX inheritance.
11. Iterating package slide parts as presentation order without `p:sldIdLst`.
12. Making `actual_slide_persist` a stale or best-effort global.
13. Fixing missing master/layout/placeholder behavior in display lowering.
14. Adding text, bullet, font, auto-fit, table, image, or connector semantics
    directly to `display.rs`.

---

## 15. Development Checklist

Before implementing a PPTX PDF feature, identify the LibreOffice owner:

| Feature | LibreOffice source area |
|---------|-------------------------|
| package traversal | `oox/source/ppt/pptimport.cxx` |
| slide/master/layout sequencing | `oox/source/ppt/presentationfragmenthandler.cxx` |
| slide XML contexts | `oox/source/ppt/slidefragmenthandler.cxx` |
| layout-specific behavior | `oox/source/ppt/layoutfragmenthandler.cxx` |
| slide state and shape creation | `oox/source/ppt/slidepersist.cxx` |
| placeholder inheritance | `oox/source/ppt/pptshapecontext.cxx`, `pptshape.cxx` |
| shape tree parsing | `oox/source/ppt/pptshapegroupcontext.cxx` |
| generic DrawingML shape creation | `oox/source/drawingml/shape.cxx` |
| fill/line/effects/theme | `oox/source/drawingml/fillproperties.cxx`, `lineproperties.cxx`, `theme.cxx` |
| DrawingML text | `oox/source/drawingml/text*.cxx` |
| tables | `oox/source/drawingml/table/*.cxx` |
| charts | `oox/source/drawingml/chart/*.cxx` |
| SmartArt | `oox/source/drawingml/diagram/*.cxx` |

If ownership is unclear, inspect LibreOffice first and only then add Rust
behavior.
