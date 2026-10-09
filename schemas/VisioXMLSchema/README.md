# Visio XML schema inputs

Generation inputs for Visio (VSDX family) support, mirroring
`schemas/OpenPackagingConventions-XMLSchema/`. Both files are Microsoft Open
Specifications schemas and carry the redistribution notice in their
`<xsd:annotation>`.

## `visio-main.xsd`

- **What:** full Visio 2013 file-format XML schema ("Schema map (Visio XML)").
- **Canonical:** <https://learn.microsoft.com/en-us/office/client-developer/visio/schema-mapvisio-xml>
- **Retrieved from:** `MicrosoftDocs/office-developer-client-docs`,
  `docs/visio/schema-mapvisio-xml.md` (raw), 2026-10-09.
- **Target namespace:** `http://schemas.microsoft.com/office/visio/2011/1/core`
- **Dialect:** XSD 1.1 — uses `xsd:alternative` (566) and `xsd:assert` (460) to
  discriminate the single `Cell`/`Section` element by `@N`.
- **Size:** 620 named complexTypes (including per-cell types such as
  `PinX_Type`), 134 distinct element names, 1 named simpleType.

## `ms-vsdx.xsd`

- **What:** `[MS-VSDX]` Appendix A: Full XML Schema (Visio Graphics Service VSDX
  File Format — the web-drawing protocol).
- **Canonical:** <https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/5d9a5a4b-c3d1-4d7b-902f-354f25fe66f4>
- **Retrieved:** 2026-10-09 (HTML entities and non-breaking spaces normalized to
  plain XML).
- **Target namespace:** `http://schemas.microsoft.com/office/visio/2011/1/core`
- **Dialect:** XSD 1.0 style (no `alternative`/`assert`).
- **Size:** 84 named complexTypes, 100 element declarations. It is a subset of
  `visio-main.xsd` (no `DocumentProperties`, `Validation`, `Solutions`,
  `Windows`, `DataTransferInfo`, and no per-cell types).

## Namespace reconciliation (known)

Real Visio 2013+ files use
`http://schemas.microsoft.com/office/visio/2012/main` (per the per-element
"Visio XML" reference pages and Apache POI's vendored `visio.xsd`), while the
publicly published full schema is under `2011/1/core`. The element/type structure
is the same family; the generator must reconcile the namespace (config knob) or
obtain the `2012/main` schema from the Visio SDK. This is tracked in
`docs/specs/visio.md`.
