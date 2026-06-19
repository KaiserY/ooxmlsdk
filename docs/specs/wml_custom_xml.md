# WML Custom XML — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §15.2.4 (Custom XML Data Storage Part) and §17.5.1 (Custom XML); ISO/IEC 29500:2016 Part 1 §15.2.4, §17.5.1; XSDs in `schemas/OfficeOpenXML-XMLSchema-Transitional/wml.xsd` and `customXmlData.xsd`.

---

## 1. Overview

Custom XML lets a document carry **arbitrary user-supplied XML data** alongside
its main content, without that data being interpreted as document body.

There are two orthogonal mechanisms:

| Mechanism | Where the data lives | Wire markup |
|-----------|---------------------|-------------|
| **Custom XML Data Storage** | Separate parts (`customXml/itemN.xml`) | None in `document.xml` — purely a part-graph feature |
| **Custom XML Tagging (legacy)** | Inside `document.xml` | `<w:customXml>` block/run/cell elements |

ooxmlsdk supports both. The data-storage form is the modern Word approach
(2007+) and the tagging form is the deprecated 2003-era approach kept for
back-compatibility.

---

## 2. Custom XML Data Storage

### Part graph

```
word/document.xml
word/_rels/document.xml.rels  ─── rId10 customXml ──▶ ../customXml/item1.xml
                                                      │
customXml/item1.xml                                   │
customXml/_rels/item1.xml.rels  ─── rId1 customXmlProps ──▶ itemProps1.xml
customXml/itemProps1.xml
```

Three parts cooperate per data store:

1. **`customXml/itemN.xml`** — `CustomXmlPart`. The user's raw XML payload.
   Treated as opaque XML by the SDK (no schema). No `Override` content-type
   entry — the file uses the `application/xml` Default extension entry.
2. **`customXml/itemPropsN.xml`** — `CustomXmlPropertiesPart`. A
   `<ds:datastoreItem>` element (`DataStoreItem` schema root) with
   `<ds:itemID>` (a GUID identifying this store) and zero or more
   `<ds:schemaRef>` children pointing at the namespace URIs the item
   conforms to.
3. **`word/_rels/document.xml.rels`** entry of type `customXml` targeting
   `../customXml/itemN.xml`.

### Relationship URIs

```
Custom XML data storage rel:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXml

Custom XML properties rel:
http://schemas.openxmlformats.org/officeDocument/2006/relationships/customXmlProps

Custom XML properties content type:
application/vnd.openxmlformats-officedocument.customXmlProperties+xml
```

### itemPropsN.xml structure

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<ds:datastoreItem ds:itemID="{D9D1AB4F-6A5F-4F8D-9EF6-0F61D62E2A98}"
                  xmlns:ds="http://schemas.openxmlformats.org/officeDocument/2006/customXml">
  <ds:schemaRefs>
    <ds:schemaRef ds:uri="http://example.com/schema/contacts"/>
  </ds:schemaRefs>
</ds:datastoreItem>
```

`ds:itemID` is a GUID in registry format (curly braces, hex with hyphens).
The same GUID is what `<w:dataBinding>` on a content control uses to
target this store via XPath.

### itemN.xml payload

Whatever the application wants. It must be well-formed XML and have a
single root element. The SDK does not validate against the schema URIs in
`schemaRefs`.

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<contacts xmlns="http://example.com/schema/contacts">
  <contact><name>Alice</name><email>alice@example.com</email></contact>
</contacts>
```

---

## 3. Custom XML Tagging (legacy)

Inline tagging in `document.xml` uses `<w:customXml>` to wrap content with
named element/attribute pairs. Three variants by container:

| Element | Where | Wraps |
|---------|-------|-------|
| `<w:customXml>` block | `<w:body>`/`<w:tc>` | Block content (paragraphs, tables) |
| `<w:customXml>` run | `<w:p>` | Run content |
| `<w:customXmlPr>` properties | inside `<w:customXml>` | Element-level data attributes |

### Structure

```xml
<w:customXml w:uri="http://example.com/schema/contacts" w:element="contact">
  <w:customXmlPr>
    <w:attr w:uri="http://example.com/schema/contacts" w:name="id" w:val="42"/>
  </w:customXmlPr>
  <w:p><w:r><w:t>Alice</w:t></w:r></w:p>
</w:customXml>
```

Attributes:
- `w:uri` — namespace of the custom-XML element
- `w:element` — local element name

Children:
- `<w:customXmlPr>` (optional, must be first if present): zero or more
  `<w:attr>` elements describing element attributes.
- The wrapped content (paragraphs, runs, tables, cells — depends on level).

This form is **deprecated** in favour of data storage + content controls
with data binding, but ooxmlsdk preserves it on round-trip.

---

## 4. SDK Access Pattern

### Data storage

```rust
let package = ooxmlsdk::package::OpcPackage::from_file("doc.docx")?;
let doc_part = package.main_document_part()?;

for custom_xml in doc_part.custom_xml_parts(&package) {
  let raw_xml: Option<&[u8]> = custom_xml.data(&package);
  if let Some(props) = custom_xml.custom_xml_properties_part(&package) {
    let datastore_item = props.root(&package);
    let item_id = &datastore_item.item_id;
    // …
  }
}
```

Rust types:
- `CustomXmlPart` — derives `SdkPart`, stores raw bytes (`data()` /
  `set_data()`).
- `CustomXmlPropertiesPart` — derives `SdkPart`; root type
  `DataStoreItem` from
  `schemas_openxmlformats_org_office_document_2006_custom_xml`.

### Inline tagging

The `<w:customXml>` variants appear as block/run children inside the
generated `Document`/`Paragraph` types. They round-trip through the
schema-driven parser without explicit access by application code unless
the application walks the body tree.

---

## 5. Round-Trip Gotchas

1. **`item.xml` has no `Override` content type.** It is delivered through
   the `<Default Extension="xml">` registration. Adding an unnecessary
   `Override` entry inflates `[Content_Types].xml` and may cause some
   consumers to flag it.

2. **`itemProps.xml` *does* have an `Override`.** Its content type is
   `application/vnd.openxmlformats-officedocument.customXmlProperties+xml`.
   Without it the part fails consumer validation.

3. **`ds:itemID` is required and must be a GUID.** Word generates these
   server-side; they appear in curly-brace registry format
   (`{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`). Round-trip must preserve
   the GUID byte-for-byte; any normalisation breaks data-binding paths
   that reference it.

4. **`<ds:schemaRefs>` may be empty.** An item with no schema declaration
   is valid; the SDK must not strip `<ds:schemaRefs>` when its content is
   empty if the original file emitted it.

5. **Multiple data stores are permitted.** A document can have any number
   of `customXml/itemN.xml` parts, each with its own properties part and
   `rId`. Numbering is by the part naming counter, not contiguous from 1.

6. **Custom XML parts use `..` in target paths.**
   `word/_rels/document.xml.rels` references
   `Target="../customXml/item1.xml"` because the parts live outside the
   `word/` directory.

7. **`<w:customXml>` inline tagging cannot be silently dropped.** Some
   round-trip implementations filter unknown wrappers; ooxmlsdk preserves
   them as schema-typed elements. Stripping them changes the document
   semantics for consumers that treat the wrapped data as authoritative.

---

## 6. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| WML-CX-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/wml/custom_xml.docx` | Single CustomXmlPart with item1.xml payload, CustomXmlPropertiesPart with `ds:itemID` GUID and `ds:schemaRefs` listing two URIs; document body uses inline `<w:customXml>` block tagging with `<w:customXmlPr>` and `<w:attr>` |
