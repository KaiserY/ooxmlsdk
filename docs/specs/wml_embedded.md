# WML Embedded Objects — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §17.3.3.18 (object), §15.2.7 (Embedded Object Part), §15.2.8 (Embedded Package Part); ISO/IEC 29500:2016 Part 1; XSD `wml.xsd`.

---

## 1. Overview

Embedded objects let a Word document carry **opaque foreign content** — OLE
controls, ActiveX controls, embedded spreadsheets, embedded packages — as
referenced parts. ooxmlsdk treats the binary payload as an **opaque blob**
(read, store, write back unchanged), exactly the same model used for VBA
projects.

Three flavours exist:

| Variant | Part | Relationship type | Typical use |
|---------|------|-------------------|-------------|
| OLE object | `EmbeddedObjectPart` (`word/embeddings/oleObjectN.bin`) | `oleObject` | Linked or embedded OLE 2 documents (Excel workbook, equation, etc.) |
| Package | `EmbeddedPackagePart` (`word/embeddings/MicrosoftWord.docx` etc.) | `package` | A whole OOXML package embedded inside another |
| ActiveX control | `EmbeddedControlPersistencePart` (`word/activeX/activeXN.bin`) | `control` | Persisted ActiveX state (`control1.xml` + `control1.bin`) |

The body markup that *references* the part is `<w:object>` (legacy VML
fallback) or a `<w:drawing>` containing an `<o:OLEObject>` / `<v:imagedata>`.

---

## 2. Body Markup — `<w:object>`

A typical OLE-object embed wraps a VML shape that holds the icon image and
references the binary part:

```xml
<w:p>
  <w:r>
    <w:object w:dxaOrig="1440" w:dyaOrig="1440">
      <v:shape id="_x0000_i1025" type="#_x0000_t75"
               style="width:50pt;height:50pt"
               xmlns:v="urn:schemas-microsoft-com:vml">
        <v:imagedata r:id="rId4" o:title=""
                     xmlns:o="urn:schemas-microsoft-com:office:office"
                     xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
      </v:shape>
      <o:OLEObject Type="Embed" ProgID="Excel.Sheet.12"
                   ShapeID="_x0000_i1025" DrawAspect="Icon" ObjectID="_1693817012"
                   r:id="rId5"
                   xmlns:o="urn:schemas-microsoft-com:office:office"
                   xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"/>
    </w:object>
  </w:r>
</w:p>
```

Attribute summary on `<w:object>`:

| Attr | Meaning |
|------|--------|
| `w:dxaOrig` | Original width (twips) |
| `w:dyaOrig` | Original height (twips) |

`<o:OLEObject>` attributes:

| Attr | Meaning |
|------|--------|
| `Type` | `Embed` (inline payload) or `Link` (external reference) |
| `ProgID` | OLE programmatic ID (`Excel.Sheet.12`, `Equation.3`, `Package`) |
| `ShapeID` | Matches the `<v:shape>` `id` attribute |
| `DrawAspect` | `Icon` or `Content` |
| `ObjectID` | A document-unique ID used for tab-stop ordering |
| `r:id` | Relationship to the binary part (`oleObject`) |

`<v:imagedata>` `r:id` references the **icon image** part (a separate
ImagePart, usually EMF or PNG).

---

## 3. Part Graph

```
word/document.xml
  rId4 image          ─▶ word/media/image1.emf      (icon preview)
  rId5 oleObject      ─▶ word/embeddings/oleObject1.bin   (the OLE payload)
```

`[Content_Types].xml` requirements:

```xml
<Default Extension="bin" ContentType="application/vnd.openxmlformats-officedocument.oleObject"/>
<Default Extension="emf" ContentType="image/x-emf"/>
```

The OLE payload uses the **package default `bin`** content type. ActiveX
persistence and VBA also use `.bin`, so the application disambiguates by
relationship type, not content type.

### Embedded Package variant

Whole-package embedding uses `package` rel type. The target is an OOXML
ZIP (e.g. `MicrosoftWord.docx`) and the `Package` ProgID:

```xml
<o:OLEObject ProgID="Package" Type="Embed" r:id="rId5" .../>
```

```xml
<Override PartName="/word/embeddings/MicrosoftWord.docx"
  ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document"/>
```

### ActiveX variant

```
word/activeX/activeX1.xml      ← persistence properties (ActiveX schema)
word/activeX/activeX1.bin      ← persisted control bytes (opaque)
word/activeX/_rels/activeX1.xml.rels  ← rel from XML to binary
```

The XML part references the binary via `relationships/activeXControlBinary`.
Body markup uses `<w:object>` with a `<w:control r:id="…"/>` child.

---

## 4. Relationship and Content-Type URIs

```
OLE object rel:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject

Embedded package rel:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/package

ActiveX control rel:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/control

ActiveX binary rel:
http://schemas.microsoft.com/office/2006/relationships/activeXControlBinary

OLE object content type (Default Extension="bin"):
application/vnd.openxmlformats-officedocument.oleObject

Embedded package content type:
application/vnd.openxmlformats-officedocument.wordprocessingml.document   (or .xlsx, .pptx)
```

---

## 5. SDK Access Pattern

```rust
let package = ooxmlsdk::package::OpcPackage::from_file("doc.docx")?;
let doc_part = package.main_document_part()?;

for ole in doc_part.embedded_object_parts(&package) {
  let bytes: Option<&[u8]> = ole.data(&package);   // raw OLE2 stream
}
for pkg in doc_part.embedded_package_parts(&package) {
  let inner_bytes: Option<&[u8]> = pkg.data(&package);  // an entire OOXML zip
}
for ax in doc_part.embedded_control_persistence_parts(&package) {
  let xml_root = ax.root(&package);   // ActiveX XML schema
  let bytes = ax.activex_persistence_data_part(&package).and_then(|p| p.data(&package));
}
```

Rust types:
- `EmbeddedObjectPart` — opaque-binary `SdkPart`.
- `EmbeddedPackagePart` — opaque-binary `SdkPart`.
- `EmbeddedControlPersistencePart` — typed `SdkPart` with XML schema root,
  plus a child `EmbeddedControlPersistenceBinaryDataPart` for the `.bin`.

---

## 6. Round-Trip Gotchas

1. **`<o:OLEObject ShapeID>` must match `<v:shape id>` exactly.** The two
   are joined by string equality, not by a relationship. Re-numbering one
   without the other breaks the icon link and the object renders as a
   broken placeholder.

2. **VML namespaces are required even in pure DOCX.** `urn:schemas-microsoft-com:vml`
   and `urn:schemas-microsoft-com:office:office` must be declared on the
   shape (or on `<w:document>`). Without them, parsers may treat the VML
   as unknown and discard the embed.

3. **`<Default Extension="bin">` is mandatory.** Same as VBA — the
   `bin` extension must be registered in `[Content_Types].xml` for the OLE
   payload to be valid. The package will silently strip the part on save
   in some consumers if the default is missing.

4. **Embedded package payloads are *complete* OOXML zips.** They have
   their own `[Content_Types].xml`, their own `_rels/`, and their own
   parts. Round-trip code must not unzip and re-zip them — preserve the
   bytes.

5. **`Type="Link"` OLE objects have no part.** Linked objects reference
   an external file and emit only `<o:OLEObject Type="Link">` markup with
   no relationship. ooxmlsdk preserves the markup but no part exists to
   round-trip.

6. **OLE payload is an OLE2 compound document (CFB).** Same magic bytes as
   VBA (`D0 CF 11 E0 A1 B1 1A E1`). The SDK does not parse the structure;
   bytes are immutable from the round-trip perspective.

7. **`ObjectID` should remain unique.** Word generates a numeric ObjectID
   per object; collisions inside a single document confuse focus order
   and accessibility tools. Round-trip must preserve the values
   verbatim.

8. **Icon image and OLE payload are *separate* relationships.** The
   `<v:imagedata r:id>` and the `<o:OLEObject r:id>` must point to two
   different parts (ImagePart + EmbeddedObjectPart). A common bug is to
   share one `rId` between them, which loads the icon as the OLE blob.

---

## 7. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| WML-EO-01 | `test-data/wml/embedded_object.docx` | `<w:object>` with VML shape and `<o:OLEObject>` referencing an `EmbeddedObjectPart` (`oleObject1.bin`, opaque OLE2 placeholder header) and a PNG icon (`media/image1.png`); `<Default Extension="bin">` content-type entry; two relationships (`image`, `oleObject`) on the document part |
