# PPTX PDF Rendering — LibreOffice-Aligned Design

**Status:** LibreOffice-shaped import skeleton plus early Phase 2/3 structural
ports for `ooxmlsdk-pdf` PPTX work. The current code is not yet a complete
PPTX renderer; it is intentionally building the upstream import model before
expanding visible PDF fidelity.

**Primary source authority:** LibreOffice `../core`, especially:

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

`../typst` and `krilla` are implementation references for fixed-position
frames, transforms, text shaping, image handling, paths, and PDF output. They
must not drive PPTX semantic design. PPTX import semantics follow LibreOffice.

---

## 0. Current Progress And Calibration

This section records the current implementation state so future work starts
from the same calibration point.

Approximate maturity:

- Overall PPTX-to-PDF support is still early, roughly 30%-40% of a useful
  LibreOffice-aligned implementation.
- The important progress is structural: the import pipeline, persist model,
  shape tree, background records, color records, graphic-frame records, and
  table model are now shaped around LibreOffice owner functions.
- Visible PDF output remains an observation path. It must not become the owner
  of PPTX semantics.

Already aligned with LibreOffice structure:

- `PowerPointImport -> PresentationFragmentHandler -> SlideFragmentHandler ->
  PPTShapeGroupContext -> drawingml::Shape -> display` exists as the main
  pipeline.
- Master, layout, and slide import order follows the LibreOffice page-import
  shape. Layout persists are cached by `(master_path, layout_path)`, and early
  layout import may clone master shape/background state before applying
  layout-local state.
- `SlidePersist` is the central state object for slide size, theme path, color
  maps, background records, shape tree, comments/notes slots, and page identity.
- `cSld/bg/bgPr` and `cSld/bg/bgRef` are separate model states. `bgRef` is not
  treated as a direct solid fill.
- Color import preserves generated SDK enum values for scheme, preset, and
  system colors. It does not encode unresolved colors as display-looking
  strings such as `scheme:*`, `preset:*`, or fake `#RRGGBB`.
- Shape import preserves non-visual metadata, placeholder subtype/index,
  transform, fill, line, text body, text list style records, picture
  relationship identity, content-part relationship identity, and grouped
  children.
- `graphicFrame` dispatch uses LibreOffice's exact `a:graphicData/@uri` table
  for presentation OLE, DrawingML diagram, DrawingML chart, Office 2014
  chartEx, and DrawingML table. Unknown graphicData remains a structured
  unsupported record.
- Table graphic data populates a DrawingML table model: table flags, style id,
  grid columns, rows, cells, span/merge flags, margins, and cell text bodies.

Known structural gaps:

- Slide import ordering must be tightened to follow `p:sldIdLst` /
  `slides_vector` exactly. The current implementation records
  `slides_vector`, but future work must not rely on package part iteration order
  as a substitute for LibreOffice's `maSlidesVector` traversal.
- `actual_slide_persist` must behave like LibreOffice's current
  `SlidePersist` pointer during fragment import, background creation, shape
  creation, and color/theme lookup. An index into `draw_pages` is acceptable as
  an implementation detail only when it already refers to the current imported
  slide state at the point where `get_scheme_color_token` and
  `get_current_theme` are called.
  During the import of a not-yet-pushed slide/layout/master persist, Rust must
  still expose that same current persist through the import context; a stale or
  future `draw_pages.len()` index is not equivalent to LibreOffice's pointer.
- Theme import is not complete. `get_scheme_color()` intentionally does not
  return an RGB color until theme color scheme and style-matrix resolution are
  ported from LibreOffice.
- Placeholder inheritance has a first structural port: `PPTShapeContext` now
  applies default subtype handling, index-based subtype inheritance,
  LibreOffice's priority lookup, `apply_shape_reference`, placeholder storage,
  and referenced marking.
- `Shape::apply_shape_reference` now copies placeholder text, table
  properties, master text list style, position, size, rotation, flips, hidden,
  and locked flags in addition to reference fill/line/effect slots. Remaining
  gaps are the explicit `bUseText` branch, generic shape properties, actual
  fill/line/effect resolution against inherited state, and custom geometry.
- Text-style inheritance has an initial upstream-shaped port:
  presentation-level `defaultTextStyle`, slide-master `txStyles`, shape
  `lstStyle`, placeholder `lstStyle`, and shape `master_text_list_style` are
  now represented with DrawingML paragraph-property slots instead of boolean
  markers. The next step is to fill in paragraph/run property application
  against the same style chain.
- Text body import now keeps generated SDK `bodyPr`, `pPr`, `rPr`, `fld/pPr`,
  and `endParaRPr` records for ordinary PPTX shape text and DrawingML table
  cell text. This is intentionally typed data from `ooxmlsdk`; do not reparse
  these text bodies with manual XML code.
- Paragraph style lookup now follows the LibreOffice `TextParagraph` owner
  path: the paragraph level selects the matching text-list level style, and
  missing/out-of-range levels fall back to the first available level style
  before any PDF lowering reads it.
- `SlidePersist::apply_text_styles` now walks the shape tree and lets each
  `TextBody` cache the selected master/current paragraph list style on each
  paragraph. This is still structural state; paragraph/run property lowering
  remains the next LibreOffice port step.
- `SlidePersist::create_background`, `SlidePersist::apply_text_styles`, and
  `SlidePersist::create_connector_shape_connection` are still upstream-shaped
  slots. `PptShape::set_text_master_styles` now owns the current text-list
  style chain until the full `apply_text_styles` pass is ported.
- `Shape::finalize_x_shape`, grab-bag preservation, diagram helper propagation,
  and advanced frame creation are still upstream-shaped slots.
- Chart, SmartArt, OLE, media, notes, comments, and VML are not fully imported.
  They must keep structured identity instead of becoming text or display-only
  fallbacks.
- Text layout currently has a temporary display bridge. It must not be expanded
  into a local PPTX text-layout model outside DrawingML text and placeholder
  inheritance.

Current simplification risks to keep contained:

- `display.rs` uses temporary text inset and line-height constants only for
  early observation. Do not promote those constants into PPTX text semantics.
- `display.rs` currently reads `SlidePersist`/`Shape` directly. This is a
  temporary observation bridge. New PPTX behavior should be implemented in
  `SlidePersist::create_x_shapes`, `PptShape::add_shape`, and
  `Shape::create_and_insert` first, then observed by display lowering.
- Master/layout duplication is currently an early replacement for UNO master
  page binding. Do not add renderer-side fixes for missing master/layout
  semantics; carry the missing state through `SlidePersist` and the shape tree.
- `GraphicData::graphic_data_choice` now exposes typed DrawingML table payloads.
  Table import must use the `GraphicDataChoice::Table` / `a::Table` SDK model
  and must not fall back to quick-xml parsing of `xml_children`.
- Service-name selection currently covers only the first placeholder-to-service
  classification. It must be extended from `pptshape.cxx`, not redesigned
  around renderer item kinds.
- Reusing DOCX layout/PDF carriers is acceptable only as paint plumbing. Do not
  let DOCX page, paragraph, section, table, or run models become persistent
  PPTX semantic state.

Next recommended development priority:

1. Tighten the LibreOffice skeleton: import slides by `slides_vector`, make
   `actual_slide_persist` point at the current slide/layout/notes persist at
   the same stage LibreOffice does, and keep page-range/custom-show/notes
   concepts in `PresentationFragmentHandler::finalize_import`.
2. Complete `Shape::apply_shape_reference` against LibreOffice
   `drawingml::Shape::applyShapeReference`, including geometry, hidden/locked,
   table/custom-geometry, and master text-list style state.
3. Continue text style inheritance by porting paragraph/run property
   application through `SlidePersist::apply_text_styles` while preserving the
   current `PptShape::set_text_master_styles` merge order.
4. Extend placeholder behavior for remaining `pptshape.cxx` service-name,
   notes, layout, custom-geometry, and hide/replacement branches.
5. Port theme color scheme and style matrix resolution through
   `PowerPointImport`, `SlidePersist`, and DrawingML theme structures.
6. Continue structured imports for chart, SmartArt/diagram, OLE, media, notes,
   comments, and VML before improving their visible PDF fallbacks.

---

## 1. Design Rule

Do not evolve PPTX PDF support from a direct `slide.xml -> PDF items` renderer.
The implementation must first mirror LibreOffice's PPTX import structure, then
lower the imported model to a fixed-page display list rendered by `krilla`.

The implementation must not add "minimum viable" local models that merely make
current fixtures render. Every PPTX semantic feature must start from the
LibreOffice owner function and data structure, even when the first Rust version
only stores structured state. A partial Rust port is acceptable only when it is
named and scoped as the same upstream concept, and when missing upstream
branches remain explicit gaps. Do not replace upstream concepts with simplified
local equivalents such as "theme path plus RGB map", "text-only slide model",
or "relationship lookup in display lowering".

Key rule:

1. Parse and resolve the presentation using LibreOffice-like import objects.
2. Build a LibreOffice-like slide/shape model, including master/layout
   inheritance and placeholder resolution.
3. Lower that model into a fixed-position page/frame display list.
4. Render the display list with `krilla`.

Strict non-negotiables:

- Do not implement a local "small" version of a LibreOffice subsystem and later
  grow it independently. Port the upstream subsystem boundary first.
- Do not let renderer needs define import data structures. Renderer fallback is
  only an observer of already imported upstream-shaped state.
- Do not return placeholder values that look resolved. For example, scheme
  color token mapping is not theme color resolution; `get_scheme_color` must
  not claim to produce a color until theme import has been ported.
- Do not encode upstream tokens as display-looking strings such as `#RRGGBB`,
  `scheme:*`, or `preset:*` unless the upstream value is already that concrete
  representation. Preserve generated SDK enum values through the import model.
- Do not add tests that lock in temporary fallback output as final behavior.
- When a branch is not implemented, preserve a typed record or explicit empty
  upstream slot instead of inventing a simpler behavior.

---

## 2. Required Upstream Pipeline

The Rust PPTX PDF path should preserve these LibreOffice stage boundaries:

```text
PowerPointImport::importDocument
  -> PresentationFragmentHandler
     -> finalizeImport
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

The Rust function names may use snake_case, but module boundaries and semantic
responsibilities should remain recognizable:

| LibreOffice | Rust equivalent |
|-------------|-----------------|
| `PowerPointImport::importDocument` | `PowerPointImport::import_document` |
| `PresentationFragmentHandler::importMasterSlides` | `PresentationFragmentHandler::import_master_slides` |
| `PresentationFragmentHandler::finalizeImport` | `PresentationFragmentHandler::finalize_import` |
| `PresentationFragmentHandler::importSlide` | `PresentationFragmentHandler::import_slide` |
| `SlideFragmentHandler::onCreateContext` | `SlideFragmentHandler::on_create_context` |
| `LayoutFragmentHandler::onCreateContext` | `LayoutFragmentHandler::on_create_context` |
| `SlidePersist::createBackground` | `SlidePersist::create_background` |
| `SlidePersist::createXShapes` | `SlidePersist::create_x_shapes` |
| `PPTShape::addShape` | `PptShape::add_shape` |
| `Shape::createAndInsert` | `Shape::create_and_insert` |

This naming is intentional. It keeps later bug work traceable to upstream.

Skeleton fidelity requirements:

- `PresentationFragmentHandler::finalize_import` owns page selection and slide
  traversal. It must walk `slides_vector` / `p:sldIdLst` order like
  LibreOffice's `maSlidesVector`; package part iteration order is not a valid
  semantic source.
- Master slides are imported through `import_master_slides` before the first
  ordinary slide page is imported. Rust may structure the call differently for
  simplicity, but the observable state must match LibreOffice: master persists
  and themes exist before slide layout/master binding and placeholder lookup.
- `PresentationFragmentHandler::import_slide` owns slide-layout resolution,
  layout-to-master resolution, master+layout cache lookup, slide persist
  creation, `actual_slide_persist` selection, `SlideFragmentHandler` import,
  `create_background`, and `create_x_shapes`. None of these decisions belong
  in PDF display lowering.
- Layout persists are cached by the pair `(master_path, layout_path)`, not by
  layout path alone, master id alone, slide id, or theme path.
- `SlidePersist` must remain the object passed through background creation,
  shape creation, current theme lookup, color map lookup, placeholder
  inheritance, comments, notes, and VML slots. Do not split these into renderer
  convenience records that cannot map back to LibreOffice.
- `SlidePersist::create_x_shapes` is not optional plumbing. It is the porting
  boundary equivalent to LibreOffice's UNO shape creation pass. Early Rust
  versions may store Rust drawing objects instead of UNO objects, but the
  branch order and ownership must stay traceable to `slidepersist.cxx`,
  `pptshape.cxx`, and `drawingml/shape.cxx`.

---

## 3. Module Plan

Implement PPTX PDF support under `crates/ooxmlsdk-pdf/src/pptx/` rather than
one large `pptx.rs` file.

Recommended modules:

```text
pptx/
  mod.rs
  import.rs
  presentation.rs
  slide.rs
  slide_fragment.rs
  layout_fragment.rs
  shape.rs
  shape_group_context.rs
  shape_context.rs
  graphic_shape_context.rs
  shape_properties_context.rs
  drawingml/
    mod.rs
    shape.rs
    shape_context.rs
    shape_group_context.rs
    graphic_shape_context.rs
    graphical_object_frame_context.rs
    shape_properties.rs
    fill.rs
    line.rs
    color.rs
    theme.rs
    text_body.rs
    text_list_style.rs
    table.rs
```

The initial implementation may leave many branches as upstream-shaped structured
records, but the type and function locations should exist before broad behavior
work starts. Avoid generic "fallback" records when LibreOffice already has a
named owner type, context, or method for the same branch.

Keep the same boundary as LibreOffice:

- `pptx/shape_group_context.rs` mirrors `oox::ppt::PPTShapeGroupContext`.
- `pptx/shape_context.rs` mirrors `oox::ppt::PPTShapeContext`.
- `pptx/graphic_shape_context.rs` mirrors `oox::ppt::PPTGraphicShapeContext`.
- `pptx/shape_properties_context.rs` mirrors
  `oox::ppt::PPTShapePropertiesContext`.
- `pptx/drawingml/*` mirrors generic `oox::drawingml::*` helpers.

Do not merge the PPT-specific contexts into generic DrawingML modules. The PPT
wrappers carry `SlidePersist` and `ShapeLocation`, which are required for
placeholder lookup, inherited styles, SmartArt/ext drawing handoff, and service
name decisions.

---

## 4. Core Import Data Structures

### 4.1 `PowerPointImport`

Mirror `oox::ppt::PowerPointImport`.

Required fields:

| Field | Purpose |
|-------|---------|
| `themes` | Theme parts keyed by fragment path |
| `draw_pages` | Imported slide `SlidePersist` records |
| `master_pages` | Imported master/layout `SlidePersist` records |
| `notes_pages` | Notes-page persists; may remain fallback-only initially |
| `actual_slide_persist` | Current slide context for theme/color lookup |
| `table_style_list_path` | Related table style list path |
| `table_style_list` | Cached table style list |
| `chart_converter` | Chart converter/cache; may be structured fallback initially |

Required methods:

- `import_document`
- `get_current_theme`
- `get_current_theme_ptr`
- `get_scheme_color_token`
- `get_scheme_color`
- `get_table_styles`
- `get_vml_drawing`
- `get_chart_converter`

`presProps` is not persistent `PowerPointImport` state in LibreOffice. It is
looked up in `PowerPointImport::importDocument` as a local relationship path
and imported after the presentation fragment when present.

`get_scheme_color` must follow LibreOffice's lookup order:

1. Current slide `ClrMap`.
2. Master slide `ClrMap`, when available.
3. Current theme color scheme.

Do not create a local theme shortcut that bypasses `SlidePersist`.

### 4.2 `PresentationFragmentHandler`

Mirror `oox::ppt::PresentationFragmentHandler`.

Required state:

| Field | Purpose |
|-------|---------|
| `slides_vector` | Slide relationship ids in presentation order |
| `slide_master_vector` | Master relationship ids |
| `notes_master_vector` | Notes master relationship ids |
| `slide_id_to_index_map` | Mapping from OOXML slide id to zero-based import index |
| `custom_show_list` | Custom show metadata |
| `section_list` | Slide section metadata |
| `slide_size` | Presentation slide size |
| `notes_size` | Notes size |
| `default_text_list_style` | Presentation-level default text style |
| `author_list` | Comment authors read from `commentAuthors.xml` |
| `comment_authors_read` | Guard: comment authors are read once |
| `embed_true_type_fonts` | Presentation embedded-font flag |
| `in_section_extension` | Section extension parsing state |

Required methods:

- `import_master_slides`
- `import_master_slide`
- `import_slide`
- `finalize_import`
- `import_slide_names`
- `save_sections`
- `save_theme_to_grab_bag`
- `save_color_map_to_grab_bag`
- `import_custom_slide_show`

`finalize_import` must preserve LibreOffice's page-range shape even if the
early implementation always imports all pages. Keep the concept so partial
rendering can be added without redesign.

### 4.3 `SlidePersist`

Mirror `oox::ppt::SlidePersist`. This is the central object. It must not be
collapsed into a generic page object.

Required fields:

| Field | Purpose |
|-------|---------|
| `path` | Current slide/master/layout fragment path |
| `layout_path` | Layout path for master+layout cache identity |
| `is_master_page` | Shape-location and placeholder behavior |
| `is_notes_page` | Notes/header/footer behavior |
| `master_persist` | Resolved master/layout persist |
| `theme` | Effective theme for this persist |
| `clr_map` | Effective color mapping |
| `name` | Slide/page name from `p:cSld/@name` |
| `visible` | Slide visibility from `p:sld/@show` |
| `show_master_shapes` | Master-shape visibility from `showMasterSp` |
| `shapes` | Root `PptShape` tree |
| `background_color` | Placeholder background color |
| `background_properties` | `bgPr` fill state or structured `bgRef` style reference |
| `default_text_style` | Default text list style |
| `title_text_style` | Title placeholder text style |
| `body_text_style` | Body placeholder text style |
| `notes_text_style` | Notes placeholder text style |
| `other_text_style` | Other-shape text style |
| `shape_map` | Shape id/model id map for connectors and hyperlinks |
| `connector_shape_map` | Connector-only shape map built after shape creation |
| `connector_connections_applied` | Whether the connector connection pass ran |
| `time_node_list` | Animation timing tree; may be import-only initially |
| `header_footer` | Header/footer flags and text |
| `comments` | Slide comments; may be import-only initially |
| `comment_authors` | Slide comment authors |
| `drawing` | VML drawing/control state for the slide |

Required methods:

- `get_layout_from_value_token`
- `hide_shapes_as_master_shapes`
- `create_background`
- `create_x_shapes`
- `apply_text_styles`
- `create_connector_shape_connection`

`create_x_shapes` should apply text styles first, walk root shape children,
call `PptShape::add_shape`, build connector maps, then resolve connector
connections. In Rust, "XShape" creation means creating `PptxDrawShape` or a
similar intermediate drawing object, not UNO objects.
The connector map must be built after shape creation/finalization, not during
raw `cxnSp` import. `cxnSp` import owns only structured endpoint records
(`stCxn`/`endCxn`) and normal shape state; the later `create_x_shapes` pass owns
shape-map collection and connection application, matching
`SlidePersist::createXShapes`.

`SlideFragmentHandler` construction must import a related `vmlDrawing` or
`legacyDrawing` fragment into `SlidePersist::drawing` when present. Its
destruction/finalization phase must have an explicit VML conversion/insertion
slot, even if the first Rust implementation only records unsupported controls.
If only a fallback marker exists, keep both lifecycle facts: the VML fragment
was discovered before shape import, and the conversion/insertion slot ran during
finalization. Do not collapse this into a display-layer boolean.

Required local state:

| Field | Purpose |
|-------|---------|
| `slide_name` | Imported slide name from `p:cSld` |
| `slide_properties` | Slide-level properties gathered during fragment parse |
| `char_vector` | Character buffer used by comment/author parsing |

Required methods:

- `finalize_import`
- `on_create_context`
- `on_characters`
- `get_char_vector`

---

## 5. Shape Model

### 5.1 `ShapeLocation`

Preserve LibreOffice's `ShapeLocation` enum exactly:

```rust
enum ShapeLocation {
    Master,
    Layout,
    Slide,
}
```

The value affects placeholder lookup, visibility, and service-name selection.
Do not replace it with booleans.

### 5.2 `PptShape`

Mirror `oox::ppt::PPTShape`, layered on top of a DrawingML `Shape`.

Required fields:

| Field | Purpose |
|-------|---------|
| `model_id` | SmartArt/fallback shape model reference |
| `shape_location` | `Master`, `Layout`, or `Slide` |
| `referenced` | Layout placeholders referenced by slide shapes |
| `placeholder` | Resolved placeholder source shape |
| `has_noninherited_shape_properties` | Whether this shape has direct `spPr` |

Required methods:

- `add_shape`
- `find_placeholder`
- `find_placeholder_by_index`
- `get_sub_type_text_list_style`
- `is_placeholder_candidate`
- `set_text_master_styles`

### 5.3 `PPTShapeGroupContext`

Mirror `oox::ppt::PPTShapeGroupContext`.

Required fields:

| Field | Purpose |
|-------|---------|
| `slide_persist` | Current slide/master/layout persist |
| `shape_location` | `Master`, `Layout`, or `Slide` |
| `graphic_shape` | Temporary graphic-frame shape used by ext drawings |

Required methods:

- `on_create_context`
- `import_ext_drawings`
- `apply_font_ref_color`

`import_ext_drawings` is the SmartArt/ext-drawing handoff. It must resolve the
relationship ids stored on the temporary graphic shape, import the ext drawing
fragment, keep the diagram drawing, and apply `fontRef` color recursively when
LibreOffice would do so.

### 5.4 `PPTShapeContext`

Mirror `oox::ppt::PPTShapeContext`.

Required fields:

| Field | Purpose |
|-------|---------|
| `slide_persist` | Current slide/master/layout persist |

Required method:

- `on_create_context`

`on_create_context` owns non-visual shape properties, placeholder handling,
`PPTShapePropertiesContext`, `ShapeStyleContext`, `TextBodyContext`, and
`Transform2DContext` for `txXfrm`.

### 5.5 `PPTGraphicShapeContext`

Mirror `oox::ppt::PPTGraphicShapeContext`.

Required fields:

| Field | Purpose |
|-------|---------|
| `slide_persist` | Current slide/master/layout persist |

Required method:

- `on_create_context`

### 5.6 `PPTShapePropertiesContext`

Mirror `oox::ppt::PPTShapePropertiesContext`.

Required method:

- `on_create_context`

This context must mark a `PptShape` as having non-inherited shape properties
when the upstream context would call `setHasNoninheritedShapeProperties`.

For `a:xfrm`, it must also set the shape property equivalent of
`PROP_IsPlaceholderDependent` to `false` before delegating to the generic
DrawingML shape-properties context.

### 5.7 `GraphicalObjectFrameContext`

Mirror `oox::drawingml::GraphicalObjectFrameContext`.

Required behavior:

1. Parse frame transform and non-visual properties through the generic DrawingML
   paths.
2. Dispatch `a:graphicData/@uri` to the matching generic context:
   `OleObjectGraphicDataContext`, `DiagramGraphicDataContext`,
   `ChartGraphicDataContext`, or table import.
3. On the end of `p:graphicFrame`, call the parent
   `PPTShapeGroupContext::import_ext_drawings` hook when LibreOffice would call
   it.

This context is generic DrawingML state, but the parent callback is PPT-specific.
Keep that direction of dependency explicit.

### 5.8 Placeholder Resolution

Placeholder behavior is a first-class requirement.

`PPTShapeContext::onCreateContext(ph)` must:

1. Set `sub_type` from `p:ph/@type` as the generated
   `p::PlaceholderValues` enum, defaulting like LibreOffice. Do not convert it
   to `Debug` strings or renderer labels.
2. Set `sub_type_index` from `p:ph/@idx` unless the value is `SAL_MAX_UINT32`
   equivalent.
3. When `@type` is missing but `@idx` exists, look up the type from the
   matching layout/master placeholder.
4. For slide and layout shapes, find the referenced placeholder by subtype and
   index.
5. Call `apply_shape_reference` on the current shape.
6. Mark the source placeholder as `referenced`.

The fallback subtype mapping must preserve LibreOffice's two-candidate lookup:

| Requested subtype | First lookup | Second lookup |
|-------------------|--------------|---------------|
| `ctrTitle` | `ctrTitle` | `title` |
| `subTitle` | `subTitle` | `body` |
| `obj` | `obj` | `body` |
| other known placeholders | own subtype | none |

Do not implement placeholder inheritance as ad hoc style merging.

Implementation checkpoint:

- `PPTShapeContext` must be called from shape import immediately after
  non-visual properties and before direct `spPr` import, so inherited shape
  state is available before direct properties override it.
- Slide import must carry the effective layout persist's shapes into the slide
  persist before slide-local shapes are imported; otherwise slide placeholder
  lookup has no layout/master tree to search.
- Lookup priority must match LibreOffice `PPTShape::findPlaceholder`: first
  subtype with same index, first subtype without same index, second subtype
  with same index, second subtype without same index, then same index.
- `SAL_MAX_UINT32` placeholder index must skip placeholder lookup and must not
  be treated as an ordinary index.
- Referenced placeholder state belongs on the imported model. Do not infer it
  from display output.

### 5.9 Service-Name Selection

`PptShape::add_shape` must preserve LibreOffice's service-name decision tree,
even though Rust will not create UNO services.

Represent service names as an enum or string-like internal value, including:

- `CustomShape`
- `GraphicObjectShape`
- `GroupShape`
- `ConnectorShape`
- `OLE2Shape`
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

Preserve these decisions:

- `title` / `ctrTitle` -> title shape.
- `subTitle` -> subtitle shape.
- `obj` and `body` -> outliner shape, except notes body -> notes shape.
- `dt`, `hdr`, `ftr`, `sldNum` can become special placeholder properties and
  may discard the visible shape if LibreOffice would do so.
- Layout placeholders for `chart`, `tbl`, `pic`, `media` fall back to outliner
  shapes.
- Slide placeholders with non-default custom geometry may become custom shapes
  instead of presentation placeholders.

Keep this logic before rendering. The renderer should never infer placeholder
semantics from raw XML.

Implementation checkpoint:

- `Shape::finalize_service_name` is the first owner of placeholder-to-service
  classification. Early branches may stay conservative, but they must use
  `p::PlaceholderValues` and `ShapeService`; do not add string matching in the
  display layer or tests.

---

## 6. DrawingML Shape Layer

The shared DrawingML layer should mirror `oox::drawingml::Shape`.

Required fields:

| Field | Purpose |
|-------|---------|
| `service_name` | Shape service/type selected by import |
| `id`, `name`, `description`, `title` | Non-visual metadata |
| `hidden`, `hidden_master_shape`, `locked` | Visibility/lock flags |
| `sub_type`, `sub_type_index` | Placeholder metadata |
| `position`, `size` | EMU shape transform data |
| `child_position`, `child_size` | Group child coordinate system |
| `rotation`, `diagram_rotation` | Rotation values |
| `flip_h`, `flip_v` | Flip flags |
| `children` | Nested shapes |
| `line_properties` | Direct line properties |
| `fill_properties` | Direct fill properties |
| `graphic_properties` | Image/media properties |
| `custom_shape_properties` | Preset/custom geometry |
| `table_properties` | Table model |
| `effect_properties` | Shadow/glow/etc. |
| `shape_style_refs` | Theme style references |
| `connector_shape_properties` | `stCxn`/`endCxn` endpoint records |
| `text_body` | DrawingML text body |
| `master_text_list_style` | Inherited text list style |
| `shape_ref_line_properties` | Placeholder/reference line props |
| `shape_ref_fill_properties` | Placeholder/reference fill props |
| `shape_ref_effect_properties` | Placeholder/reference effect props |
| `frame_type` | Generic/chart/table/diagram/OLE classification |

Required methods:

- `set_defaults`
- `apply_shape_reference`
- `add_shape`
- `add_children`
- `create_and_insert`
- `finalize_service_name`
- `finalize_x_shape`
- `put_property_to_grab_bag`
- `put_properties_to_grab_bag`
- `get_actual_fill_properties`
- `get_actual_line_properties`
- `get_actual_effect_properties`
- `set_text_body`
- `set_master_text_list_style`
- `set_chart_type`
- `set_table_type`
- `set_diagram_type`
- `set_ole_object_type`
- `clone_fill_properties`
- `keep_diagram_drawing`
- `prepare_diagram_helper`
- `propagate_diagram_helper`
- `migrate_diagram_helper_to_new_shape`

`create_and_insert` is the boundary where the LibreOffice import model becomes
the Rust drawing model. Keep the function name and branch structure close to
upstream even when early branches are fallback-only.

`ShapeGroupContext` is a `FragmentHandler2`-derived context in LibreOffice, not
just a passive node visitor. Preserve this distinction in the Rust parser shape:
group context owns child context creation and adds children to the group shape.

---

## 7. Theme, Color, Fill, and Line Rules

Theme resolution must follow LibreOffice's layering:

### Fill

`Shape::getActualFillProperties` order:

1. Reference shape fill properties.
2. Theme fill style from `fillRef`.
3. Direct shape fill properties.
4. Parent group fill when direct fill is `grpFill`.

### Line

`Shape::getActualLineProperties` order:

1. Reference shape line properties.
2. Theme line style from `lnRef`.
3. Direct shape line properties.

### Effects

`Shape::getActualEffectProperties` must exist as a separate layer even if only
basic effects are implemented at first.

### Color Mapping

Scheme colors must be resolved through the current `PowerPointImport` context:

1. Active slide `ClrMap`.
2. Master `ClrMap`.
3. Theme color scheme.

Do not use a standalone `schemeClr -> RGB` table disconnected from
`SlidePersist`.

---

## 8. Text Model

DrawingML text should mirror the LibreOffice text model:

- `TextBody`
- `TextBodyProperties`
- `TextListStyle`
- `TextParagraph`
- `TextParagraphProperties`
- `TextCharacterProperties`
- `TextRun`
- `TextField`

Text import should preserve:

- paragraph level
- paragraph properties
- run properties
- field type, including slide number and date/time
- body properties, including insets, vertical anchor, rotation, autofit,
  wrapping, and overflow
- master text list style application
- placeholder text style inheritance

Current implementation boundary:

- `TextListStyle` must mirror the generated OpenXML structure:
  `defPPr` plus `lvl1pPr` through `lvl9pPr`. Do not replace this with a
  reduced "has style" or "level count" model.
- `TextBody` must keep the generated `a:BodyProperties`; `TextParagraph` must
  keep generated `a:ParagraphProperties` and `a:EndParagraphRunProperties`;
  `TextRun` must keep generated `a:RunProperties` where the SDK exposes them.
  Do not reparse these with manual XML code.
- `p:defaultTextStyle` belongs to `PresentationFragmentHandler` and is copied
  into relevant `SlidePersist` instances before shape import.
- `p:txStyles` belongs to slide-master import and populates
  `title_text_style`, `body_text_style`, and `other_text_style` on
  `SlidePersist`.
- `PptShape::set_text_master_styles` must follow LibreOffice's owner path:
  subtype/master style first, placeholder master/list style next, current
  shape `txBody/lstStyle` last. Later paragraph-level application should extend
  this chain, not bypass it in `display.rs`.
- `TextParagraph::get_paragraph_style` is the owner of level-to-list-style
  selection, matching LibreOffice's fallback to level 0 when the requested
  level is unavailable. The renderer must not reimplement this lookup.
- `SlidePersist::apply_text_styles` must remain the pass that prepares this
  paragraph style state. Do not move level lookup into the PDF display list.

Lowering text to PDF may use the existing text metrics/shaping code and may
borrow Typst's fixed-frame text item approach. That lowering must not collapse
the upstream text-style inheritance model.

---

## 9. Display List and PDF Backend

The final renderer should use a fixed-position display list similar to Typst's
`PagedDocument` / `Frame` / `FrameItem` model:

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

This display list is an output of the LibreOffice-aligned import layer. It is
not the source data model.

`krilla` should handle:

- PDF page creation
- paths and fills
- strokes
- image embedding
- text glyph output
- link annotations

`krilla` should not handle:

- placeholder lookup
- master/layout inheritance
- theme color mapping
- service-name selection
- shape-type semantic fallback

---

## 10. DOCX Lessons Applied to PPTX

The DOCX renderer started with simple output and later had to move behavior
back toward LibreOffice-shaped import, layout, and paint boundaries. Do not
repeat that path for PPTX. These rules are implementation constraints, not
style preferences.

### 10.1 Replace the Bootstrap, Do Not Extend It

The current `crates/ooxmlsdk-pdf/src/pptx.rs` path extracts text from slide
shape trees and sends it through `layout::text_pages`. That is useful only as
a smoke placeholder.

Do not add features to that path, including:

- shape position handling
- text style extraction
- image extraction
- master/layout fallback hacks
- direct theme-color lookup
- ad hoc slide background paint

The first real PPTX change should introduce the LibreOffice-shaped skeleton
from this document and route `convert_pptx` through it. Temporary blank-slide
output is preferable to a richer text-only renderer that locks in the wrong
architecture.

### 10.2 Keep Import, Drawing Model, Display List, and PDF Paint Separate

The current DOCX implementation now has an explicit sequence:

```text
typed package import -> Writer-like model -> layout model -> paint model -> krilla
```

PPTX needs the equivalent sequence:

```text
PresentationDocument
  -> PowerPointImport / PresentationFragmentHandler / SlidePersist
  -> DrawingML/PPT shape model
  -> fixed-slide display list
  -> krilla paint
```

Do not let lower layers pull OOXML semantics upward:

- The Krilla backend must not inspect relationships, placeholder types, theme
  references, master/layout paths, or raw XML.
- The display list must not perform placeholder inheritance or service-name
  selection.
- `SlidePersist::create_x_shapes` / `PptShape::add_shape` is the semantic
  boundary for LibreOffice shape decisions.
- The final PDF renderer may reuse shared text shaping, image embedding, path,
  and transform helpers, but it must receive already-resolved paint data.

### 10.3 Avoid DOCX Semantic Leakage

Shared infrastructure is allowed, but DOCX concepts must not become the PPTX
source model.

Allowed reuse:

- `units`
- `fonts`
- `text_metrics`
- neutral Krilla rendering helpers
- neutral image/path/transform helpers
- a shared fixed-page paint item layer, if it stays format-neutral

Do not model PPTX semantics with DOCX-specific types such as `DocxDocument`,
`Paragraph`, `Block`, section/page-style state, Writer frame follows, or
`docx::PageSetup` as persistent PPTX state. Slide size, slide background,
shape tree, placeholder inheritance, and text list styles should live in PPTX
types first, then be lowered to any shared fixed-page paint layer.

If a shared type is needed, extract or introduce a neutral type instead of
threading more PPTX behavior through `crate::docx::*`.

### 10.4 Keep Structured Fallbacks Instead of Text Fallbacks

DOCX work showed that converting unsupported content into plain text early
makes later parity work harder because the original object identity is gone.

For PPTX, unsupported content must keep a typed placeholder record:

| Unsupported area | Required fallback record |
|------------------|--------------------------|
| image/picture | blip embed/link relationship ids, bounds, non-visual metadata |
| chart | frame type, relationship id, bounds, title/text cache if available |
| SmartArt/diagram | diagram relationship ids, ext drawing ids, model id, bounds |
| table | DrawingML table grid/cell model, bounds, style ids, cell text bodies |
| media/contentPart | content-part/media relationship id, poster/image relationship, bounds |
| OLE | OLE relationship id, preview relationship, bounds |
| comments/notes | parsed metadata tied to slide/page identity |

Visible text extracted from these objects may be used as a temporary paint
fallback only after the structured object is preserved.

### 10.5 Source Comments and Upstream Ownership

Each non-trivial import or layout rule must name its upstream owner in code or
tests. Follow the DOCX convention of short `Source:` comments near the behavior.

Examples:

- slide sequencing: `Source: LibreOffice oox/source/ppt/presentationfragmenthandler.cxx`
- placeholder lookup: `Source: LibreOffice oox/source/ppt/pptshapecontext.cxx`
- service-name selection: `Source: LibreOffice oox/source/ppt/pptshape.cxx`
- actual fill/line resolution: `Source: LibreOffice oox/source/drawingml/shape.cxx`
- PDF glyph/path/image paint mechanics: `Source: Typst ...` only when the rule
  is about paint mechanics, not PPTX semantics

If a behavior cannot be tied to an upstream function yet, keep it as an explicit
fallback branch and do not promote it to a general rule.

### 10.6 Test Shape Before PDF Bytes

The DOCX lane now relies on import/layout/paint assertions before PDF smoke
checks. PPTX should follow the same testing order once implementation starts:

1. Import model snapshot: slide ids, master ids, layout ids, themes, color maps.
2. Shape model snapshot: placeholder subtype/index, service name, transform,
   fill/line/text inheritance, relationship ids.
3. Display-list snapshot: page size, z-order, resolved bounds, transforms,
   fill/line/text/image items.
4. PDF observation: page count, text/image/path geometry, links, and validity.

Avoid tests that only assert that output starts with `%PDF-`, and avoid tests
that lock in the current text-only PPTX output. Once a LibreOffice-derived
fixture is added, include its upstream path in a `// Source:` comment or source
map.

### 10.7 Fixture Discipline

Use real LibreOffice-derived PPTX fixtures first:

- `../core/sd/qa/unit/data/pptx/*.pptx`
- `../core/sd/qa/filter/data/pptx/*.pptx` when relevant
- `../core/vcl/qa/cppunit/pdfexport/data/*.pptx`

Do not create broad hand-authored PPTX files to justify local behavior. Small
project-owned fixtures are acceptable only for parser gaps that have a named
upstream behavior and no usable LibreOffice fixture.

### 10.8 Refactor Timing

For PPTX, first translate the LibreOffice structure with recognizable Rust
equivalents. Delay broad cleanup until the skeleton, key paths, and tests exist.

Allowed before parity tests:

- file/module split matching upstream ownership
- small neutral helpers for units, relationships, and paths
- explicit fallback enums and records

Avoid before parity tests:

- merging PPT and generic DrawingML contexts
- replacing `SlidePersist` with a simpler slide struct
- replacing `ShapeLocation` with booleans
- centralizing theme/color resolution outside `PowerPointImport`
- redesigning service-name selection around local renderer item kinds
- moving behavior into Krilla rendering because it is easier to paint there

---

## 11. Implementation Order

### Phase 1: Upstream Skeleton

Create the modules, data structures, and method boundaries listed above.
Replace `pptx.rs` with a module directory but keep public `convert_pptx` API
unchanged.

Minimum behavior:

- load presentation root
- collect slide ids and master ids
- compute slide size using upstream defaults
- create `PowerPointImport`
- create `PresentationFragmentHandler`
- create `SlidePersist` records for slides
- produce one blank page per imported presentation slide

Implementation checkpoint:

- `pptx::layout` may lower `SlidePersist::size` to a neutral fixed-page
  `LayoutDocument` only after `PowerPointImport::import_document` has built the
  upstream-shaped presentation state.
- The Phase 1 skeleton imports presentation-order slide parts. Visibility,
  `PageRange`, custom show selection, notes-page import, and `p:sld/@show`
  filtering remain `PresentationFragmentHandler::finalize_import` work, not
  shortcuts in the PDF backend.
- Presentation order means `p:sldIdLst` / `slides_vector` order. Do not use
  package part iteration order unless it is explicitly proven to be the same
  and still routed through `slides_vector` lookup.
- A helper such as `layout::fixed_pages_with_items` is acceptable because it
  contains no PPTX semantics. It must remain a paint/layout bridge fed by
  upstream-shaped PPTX state, not an import shortcut.
- Do not reintroduce `layout::text_pages` for PPTX. Text must enter later
  through `SlideFragmentHandler`, `PPTShapeGroupContext`, `PPTShapeContext`,
  `TextBody`, and `SlidePersist::create_x_shapes`.
- The first non-blank output path should be `PowerPointImport` ->
  `SlidePersist` -> DrawingML `Shape`/`TextBody` -> neutral `PageItem`s. The
  lowering layer may use existing PDF/layout primitives, but it must not parse
  OOXML, resolve placeholders, or duplicate LibreOffice import decisions.
- Reusing a shared paint carrier such as `docx::TextStyle` is acceptable only
  as a temporary layout/PDF bridge. PPTX semantic state must stay in the PPTX
  model (`Shape`, `TextBody`, future run properties, theme and placeholder
  records) so it can be replaced with a neutral style carrier during later
  refactoring without changing import behavior.
- Dead-code warnings are expected during this phase because LibreOffice-shaped
  fields and methods are intentionally present before every branch consumes
  them. Do not suppress these warnings with `#[allow]`; consume the fields by
  implementing the corresponding upstream path.

### Phase 2: Master/Layout/Slide Import

Implement:

- `import_master_slides`
- master/layout path resolution
- master+layout cache identity using `(master_path, layout_path)`
- slide `master_persist` assignment
- theme assignment
- `SlideFragmentHandler` parsing for `cSld`, `spTree`, `bg`, `bgPr`,
  `bgRef`, `clrMap`, and `clrMapOvr`

Implementation checkpoint:

- `PresentationFragmentHandler::import_slide` should resolve the slide layout
  and master paths before importing the slide fragment, matching LibreOffice's
  `importSlide` order.
- `actual_slide_persist` must be active for the persist being imported and
  shaped before any code can ask `PowerPointImport` for the current theme,
  scheme color token, VML drawing, chart converter, or table styles. If Rust
  stores an index instead of a pointer, the index must refer to the same logical
  persist that LibreOffice would expose through `mpActualSlidePersist`.
  If that persist has not yet been appended to `draw_pages` or `master_pages`,
  keep a temporary current-persist context rather than letting lookup fall back
  to the first theme or default color mapping.
- Layout persists should be cached by `(master_path, layout_path)` in
  `PowerPointImport::master_pages`. Do not replace this with a single master id
  lookup; placeholder inheritance depends on the master+layout pair.
- Until the full UNO-style master-page binding is replaced by a Rust drawing
  page model, a layout persist may clone the already imported master shapes
  and background state before importing layout-local shapes. This mirrors
  LibreOffice's effective master+layout page order and keeps
  `master_page_index` useful for early PDF output.
- Slide fragments may initially record only structural state, but the parser
  must enter through `SlideFragmentHandler::import_common_slide_data` and
  `PPTShapeGroupContext`, not through ad hoc shape-tree loops in `pptx::layout`.
- `cSld/bg/bgPr` and `cSld/bg/bgRef` must populate `SlidePersist` background
  state during slide-fragment import. `create_background`/display lowering may
  consume that state later, but should not inspect background XML directly.
  `bgRef` is a style-matrix reference plus placeholder color, not a direct
  solid fill; preserve it as a structured style reference until theme
  `fmtScheme/fillStyleLst` resolution is ported from LibreOffice.
- Master `clrMap` and slide/layout `clrMapOvr` must be stored on
  `SlidePersist` before shape and background conversion needs scheme colors.
  Follow LibreOffice's lookup order: current slide/layout color map first, then
  the bound master color map, then theme colors.
- Master theme relationships must be resolved during master import and stored
  in `PowerPointImport::themes` by theme part path. `SlidePersist::theme_path`
  should point at the active theme so later scheme color and style matrix
  resolution does not have to rediscover package relationships in display code.
- `showMasterSp=false` handling belongs in the layout-fragment/persist path.
  It must mark inherited master shapes before layout-local shapes are appended,
  and it must not be approximated in the renderer.
- Slide visibility, page ranges, notes-page import, comment import, custom
  shows, and slide names are `PresentationFragmentHandler` responsibilities.
  Keep structured no-op slots there until they are ported; do not spread them
  into `pptx::display` or the PDF backend.
- `p:sld/@show`, `p:sld/@showMasterSp`, and `p:cSld/@name` must be imported
  into `SlidePersist` during `SlideFragmentHandler`, before display lowering.
  Do not treat hidden slides or master-shape visibility as renderer filters;
  page-range/custom-show selection can decide whether to emit them later.

### Phase 3: Shape Tree

Implement:

- `PPTShapeGroupContext`
- `PPTShapeContext`
- `PPTGraphicShapeContext`
- `PPTShapePropertiesContext`
- `GraphicalObjectFrameContext`
- base DrawingML `ShapeContext`
- shape ids/names/hidden flags
- transform, fill, line, style refs, and text body
- placeholder lookup and `apply_shape_reference`
- ext drawing import slots and `fontRef` color propagation

Implementation checkpoint:

- Shape tree traversal should dispatch the generated OOXML variants
  `Shape`, `GroupShape`, `GraphicFrame`, `ConnectionShape`, `Picture`, and
  `ContentPart` in `PPTShapeGroupContext`, matching LibreOffice's
  `PPTShapeGroupContext::onCreateContext` ownership.
- For every supported branch, preserve non-visual drawing metadata first:
  `id`, `name`, `descr`, `hidden`, `title`, placeholder subtype, and
  placeholder index.
- Preserve `a:xfrm` / grouped transform state in the DrawingML shape model:
  position, size, rotation, horizontal/vertical flip, child offset, and child
  extents. Do not convert this directly to PDF coordinates during import.
- Preserve basic `spPr` paint state in the DrawingML shape model: `noFill`,
  `solidFill`, `grpFill`, and `ln` width/color. Unsupported fills and strokes
  should remain absent or structured placeholders until theme/color resolution
  is ported; do not invent PDF-only color behavior.
- Preserve `spPr/custGeom` and `spPr/prstGeom` as DrawingML custom-shape
  properties during import. A preset `line` geometry may select the line-shape
  service for non-connector shapes, matching LibreOffice, but geometry must not
  be converted to PDF path data in the parser.
- Preserve `style/lnRef`, `style/fillRef`, `style/effectRef`, and
  `style/fontRef` as typed shape style references. Theme/style-matrix
  resolution belongs to later DrawingML theme code and `create_and_insert`, not
  to `PPTShapeGroupContext` or display lowering.
- `graphicFrame` must enter the generic
  `drawingml::GraphicalObjectFrameContext` dispatch by `a:graphicData/@uri`,
  then return to the PPT parent for `PPTShapeGroupContext::import_ext_drawings`.
  Keep chart/table/diagram/OLE/media classification as structured frame state
  even while rendering is fallback-only.
  The URI dispatch must use LibreOffice's exact URI table for presentation OLE,
  DrawingML diagram, DrawingML chart, Office 2014 chartEx, and DrawingML table.
  Do not use substring matching such as `/chart` or `/table`, because unknown
  graphicData must remain an unsupported structured record rather than being
  misclassified.
  For table graphic data, dispatch must populate a DrawingML table model owned
  by `Shape::table_properties`: table flags, style id, grid columns, rows,
  cells, merge/span flags, margins, and cell text bodies. This mirrors
  LibreOffice `table::TableContext`, `TableRowContext`, and
  `TableCellContext`; rendering may ignore the table temporarily, but the
  import model must not collapse it to a generic graphic frame.
  The table payload must come from the typed SDK branch
  `GraphicDataChoice::Table`. Do not resurrect raw `GraphicData::xml_children`
  parsing for table cells, table text, or style id.
- `txBody` must become a DrawingML `TextBody` on the shape model before any
  visible text fallback exists. Preserve paragraph level, run text, line breaks,
  field text, and field type as structured text runs; later style inheritance
  must extend this model rather than reparsing PDF text items.
- `apply_shape_reference` must copy the same categories of state as
  LibreOffice `drawingml::Shape::applyShapeReference`: inherited text according
  to the `bUseText` branch, shape properties, actual fill/line/effect state,
  custom geometry, table properties, master text list style, position, size,
  rotation, flips, hidden, and locked flags. Partial ports should leave
  explicit missing branches rather than silently treating placeholder
  inheritance as text-style merging.
- Unsupported branches should become structured records or empty structured
  slots. Do not convert them to visible text or drop relationship identity.
  `Picture` must preserve blip `r:embed`/`r:link`; `ContentPart` must preserve
  its `r:id` and transform/non-visual metadata before later media-specific
  handling is added.
- Group-shape recursion may initially flatten into the slide persist's shape
  vector only as an early bootstrap. The preferred path is to preserve
  `Shape::children` during `PPTShapeGroupContext` import, matching
  LibreOffice's group shape ownership, and let temporary PDF lowering recurse
  over that tree.

### Phase 4: Create XShapes Equivalent

Implement `SlidePersist::create_x_shapes` and `PptShape::add_shape` to build
Rust drawing objects:

- background
- basic custom shapes
- text boxes
- images
- groups
- connectors as structured fallback
- tables as structured fallback
- charts/SmartArt as structured fallback

Implementation checkpoint:

- `create_x_shapes` must stay the single pass that applies text styles, calls
  `PptShape::add_shape` / `Shape::create_and_insert`, records connector shape
  maps, and later applies connector connections. Avoid adding new visible PPTX
  behavior only in `display.rs`.
- `PptShape::add_shape` owns PPT-specific service-name and placeholder
  decisions; generic `drawingml::Shape::create_and_insert` owns generic shape
  creation. Keep this split even when Rust stores drawing records instead of
  UNO `XShape`s.
- Connector, animation, SmartArt helper, chart, media, and OLE branches may be
  structured fallback records at first, but the fallback records must be
  produced by this pass or by earlier import contexts, not invented by the PDF
  backend.
- Connector endpoint metadata belongs on the DrawingML connector shape as
  structured `stCxn`/`endCxn` records. Do not resolve connector endpoints while
  parsing the shape tree; LibreOffice waits until `createXShapes`, after the
  page shape map exists.

### Phase 5: Lowering to PDF Display List

Lower Rust drawing objects to a dedicated PPTX display list. It may later share
a neutral fixed-page paint layer with DOCX, but do not use DOCX-specific source
types as the PPTX model.

- page size
- z-order
- groups and transforms
- fill/line paths
- images and crops
- text boxes
- links

Implementation checkpoint:

- `pptx::display` is a compatibility observation layer while the drawing-object
  layer is being ported. It may lower `SlidePersist` state into current neutral
  `LayoutDocument`/`PageItem` primitives, but it must never become the owner of
  PPTX semantics.
- Master and layout persists must still pass through `create_background` and
  `create_x_shapes` before being stored, matching LibreOffice's import order,
  even while some branches only preserve structured upstream state.
- This lowering must consume only the upstream-shaped PPTX model: `SlidePersist`
  order, DrawingML `Shape` bounds, and structured `TextBody` paragraphs/runs.
  It must not inspect package parts, relationship XML, or generated OOXML
  element trees directly.
- Display lowering may prepend the slide's resolved `master_page_index` persist
  before slide-local shapes. The decision about which master/layout persist is
  active must already be made by `PresentationFragmentHandler::import_slide`.
- Background observation may draw a full-page neutral rectangle from
  `SlidePersist::background_properties`. It must only use already imported
  direct `bgPr` fill state; `bgRef` must remain invisible until theme
  fill-style lookup is implemented in the import/model layer.
- Group output must follow `Shape::children` and carry group transform offset
  state as a temporary approximation of LibreOffice's transformation matrix.
  Do not flatten groups in the display layer to work around missing transform
  handling.
- Basic shape observation may emit neutral rectangle items from already imported
  `Shape` bounds/fill/line state. It must not inspect `spPr` XML or resolve
  theme colors in the display layer; unsupported scheme/preset colors should
  stay invisible until theme resolution is implemented in the import/model path.
- Text observation may use default font and line metrics only as a temporary
  PDF bridge, but all
  paragraph/run data required for later LibreOffice-aligned inheritance must be
  preserved before lowering. Do not make PDF text output the only source of
  paragraph level, fields, breaks, or future run properties.
- Once `SlidePersist::create_x_shapes` has a real drawing-object equivalent,
  move display lowering to
  consume that equivalent drawing-object layer. The early `SlidePersist` ->
  `PageItem` path is a compatibility bridge, not an implementation strategy.

### Phase 6: Fixture Tests and Refinement

Only after the upstream skeleton is in place, add focused fixture tests from
LibreOffice:

- `../core/vcl/qa/cppunit/pdfexport/data/*.pptx`
- `../core/sd/qa/unit/data/pptx/*.pptx`

Keep `// Source:` comments in tests. Use PDFium/lopdf assertions for visible
output only after import structure is stable.

---

## 12. Anti-Patterns

Avoid these patterns:

1. Directly rendering `slide.xml` shapes without master/layout resolution.
2. Treating placeholder inheritance as optional style cleanup.
3. Resolving theme colors without `PowerPointImport` and `SlidePersist`.
4. Converting every unsupported object to text too early.
5. Flattening groups before transform and child coordinate semantics are known.
6. Adding local service/type names that do not map to LibreOffice's service
   decisions.
7. Building tests that lock in temporary text-only rendering.
8. Refactoring shared DrawingML abstractions before PPTX behavior is stable.
9. Using DOCX-specific section/frame/page-style types as persistent PPTX
   semantic state.
10. Letting `krilla` rendering code inspect raw OOXML or resolve PPTX
    inheritance.
11. Replacing unsupported charts, SmartArt, tables, media, or OLE with plain
    text before preserving their relationship metadata and frame classification.
12. Iterating slide parts from the package and assuming that order is the
    presentation order without going through `p:sldIdLst`.
13. Making `actual_slide_persist` a best-effort global that is unset or points
    at stale state during theme/color/background/shape creation.
14. Fixing missing master/layout/placeholder behavior by duplicating logic in
    display lowering.
15. Adding text, bullet, font, auto-fit, table, image, or connector behavior
    directly to `display.rs` when the corresponding LibreOffice owner is
    `SlidePersist`, `PptShape`, `drawingml::Shape`, or a DrawingML context.

---

## 13. Initial Scope Boundaries

The first implementation does not need full fidelity for:

- animations
- transitions
- comments
- notes rendering
- charts
- SmartArt layout
- media playback
- advanced effects
- OLE rendering

However, each must have a structured import location that matches LibreOffice,
so later work extends existing branches instead of redesigning the pipeline.

Charts, SmartArt, tables, media, and OLE must not disappear silently. Preserve
their frame/type classification and enough relationship metadata for later
implementation.

---

## 14. Development Checklist

Before implementing a PPTX PDF feature, identify its LibreOffice owner first:

| Feature | LibreOffice source area |
|---------|-------------------------|
| presentation package traversal | `oox/source/ppt/pptimport.cxx` |
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

If the upstream owner is unclear, inspect LibreOffice first and only then add
Rust behavior.
