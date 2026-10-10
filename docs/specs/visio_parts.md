# Visio package and parts

**Purpose:** How a VSDX-family package is opened, saved, and typed in `ooxmlsdk`.
This is an implementation note for the package layer. Formula evaluation and
PDF rendering are separate tracks and are not described here.

**Sources used:** the checked-in `schemas/VisioXMLSchema/visio-main.xsd`, the
part JSON under `data/parts/Visio*.json`, and the generated `VisioDocument`
package type. Language here is original. Examples are original.

## Namespace

Real drawing files use this namespace on the document XML:

```text
http://schemas.microsoft.com/office/visio/2012/main
```

The published schema file uses `http://schemas.microsoft.com/office/visio/2011/1/core`.
Generation keeps that file URI and writes the 2012 URI on the stage-1 schema.
The prefix in this crate is `visio`.

## Six content types

One package type, `VisioDocument`, covers the family. The document part's
content type selects the kind. The part JSON records only the drawing type.
`VisioDocumentType` accepts all six, and the document part stays classified
as that part after the content type changes.

| Kind | Content type |
| --- | --- |
| Drawing | `application/vnd.ms-visio.drawing.main+xml` |
| MacroEnabledDrawing | `application/vnd.ms-visio.drawing.macroEnabled.main+xml` |
| Stencil | `application/vnd.ms-visio.stencil.main+xml` |
| MacroEnabledStencil | `application/vnd.ms-visio.stencil.macroEnabled.main+xml` |
| Template | `application/vnd.ms-visio.template.main+xml` |
| MacroEnabledTemplate | `application/vnd.ms-visio.template.macroEnabled.main+xml` |

`VisioDocument::create` starts from the requested kind. Adding the document
part first installs the drawing content type from the part descriptor.
`change_document_type` then writes the kind that was requested.

There is no extension table. Word packages in this SDK also have no table
from `.docx` to a content type.

## Parts

The document part is required. These children are optional:

- pages (`application/vnd.ms-visio.pages+xml`), which owns page parts
- masters
- connections, recordsets, comments, extensions
- the shared theme part
- `vbaProject.bin` through the existing `VbaProjectPart`

A stencil opens with no pages part. A drawing opens the same way. Pages are
not required for the package to load.

Macro-enabled kinds may carry `VbaProjectPart`. The part stores bytes. This
layer does not interpret VBA.

Relationship types for Visio parts use
`http://schemas.microsoft.com/visio/2010/relationships/`. The VBA relationship
is the existing Office one,
`http://schemas.microsoft.com/office/2006/relationships/vbaProject`.

## What this layer does not do

- It does not evaluate ShapeSheet formulas.
- It does not draw pages to PDF.
- It does not parse legacy `.vsd` files.
