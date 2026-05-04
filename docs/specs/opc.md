# OPC Clean-Room Specification
## Open Packaging Conventions for OOXML Parsers

**Purpose:** This document is a clean-room re-statement of the Open Packaging
Conventions (OPC) semantics defined in ECMA-376 Part 2 (5th edition) and
ISO/IEC 29500-2:2021, synthesised from the publicly available standard,
Microsoft's open-specification documents, and published reference
implementations. It is intended as an implementation guide and test oracle for
`ooxmlsdk` and similar Rust OOXML parsers.

**Sources used:**
- ECMA-376 Part 2, 5th edition — normative structure and rules
- ISO/IEC 29500-2:2021 — cross-reference for conformance classes
- Microsoft Learn: `[MS-OXML]` and Open XML SDK documentation
- ECMA-376 Part 2 rendered at c-rex.net
- python-docx and python-pptx source code (reference implementations)
- Apache OpenOffice OPC implementation notes
- Library of Congress OOXML format description

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or adapted to be clearly
distinct from spec examples.

---

## 1. What OPC Is

OPC (Open Packaging Conventions) defines the container format that wraps all
OOXML document types (DOCX, XLSX, PPTX, and their variants). It specifies:

1. **A ZIP-based physical container** with constraints beyond plain ZIP.
2. **Parts**: the named units of content inside the container.
3. **Relationships**: typed, directed associations between parts (or between
   the package itself and a part).
4. **Content Types**: a declaration of the media type for every part.
5. **Metadata parts**: core properties, digital signatures, thumbnail.

OPC is format-agnostic. OOXML formats (WordprocessingML, SpreadsheetML,
PresentationML) use OPC as their container and add their own part types and
relationship types on top.

---

## 2. ZIP Container Constraints

An OPC package is a valid ZIP archive with the following additional constraints:

### 2.1 Permitted ZIP Features

| Feature | Status |
|---|---|
| STORE compression (method 0) | Permitted |
| DEFLATE compression (method 8) | Permitted |
| ZIP64 extensions | Permitted (required for files > 4 GiB) |
| Password-protected entries | NOT permitted |
| Split/spanned archives | NOT permitted |
| Encryption (beyond standard ZIP) | NOT permitted by OPC itself (use IRM separately) |

### 2.2 Encoding

- All file names (entry names) in the ZIP local-file and central-directory
  headers MUST be UTF-8 encoded (or plain ASCII, which is a valid UTF-8 subset).
- The general-purpose bit flag for UTF-8 (bit 11) SHOULD be set for non-ASCII
  entry names.

### 2.3 Timestamps

ZIP stores file modification times in MS-DOS format: year-month-day
hour:minute:second, 2-second resolution, relative to local time. OPC does not
require any particular timestamp. For deterministic builds, use the MS-DOS
epoch: `1980-01-01 00:00:00` (the earliest representable value).

### 2.4 Entry Order

OPC does not mandate a specific entry order within the ZIP archive, but
`[Content_Types].xml` is conventionally the first entry so that readers can
determine content types before loading other parts.

---

## 3. Parts

### 3.1 Definition

A **part** is a named unit of content in the package — analogous to a file in
a directory tree. Parts have:
- A **part name** (URI-like path)
- A **content type** (MIME type)
- Optionally, one or more **relationships** (declared in a companion `.rels` file)

### 3.2 Part Naming Rules

Part names are URI references following strict rules from RFC 3986:

**Syntax:**
```
part-name = 1*( "/" segment )
segment   = 1*pchar
pchar     = unreserved / pct-encoded / sub-delims / ":" / "@"
```

**Constraints:**
- MUST start with `/` (absolute path, no scheme or authority)
- MUST NOT end with `/`
- MUST NOT contain empty segments (`//`)
- MUST NOT contain `.` or `..` as complete path segments
- MUST NOT end with `.` in any segment
- Part names are **case-insensitive** for lookup and uniqueness checking
- Part names are **case-preserving** in storage (the ZIP entry name records
  the original case)
- MUST NOT use the characters `\`, `|`, `<`, `>`, `"`, `?` unencoded
- Percent-encoding is allowed for any character outside the unreserved set;
  percent-encoding is case-insensitive (e.g. `%2F` == `%2f`)

**Valid examples:**
```
/word/document.xml
/xl/workbook.xml
/ppt/presentation.xml
/docProps/core.xml
/_rels/.rels
/word/_rels/document.xml.rels
```

**Invalid examples:**
```
word/document.xml          ← missing leading /
/word/document.xml/        ← trailing /
/word/../document.xml      ← .. segment
/word//document.xml        ← empty segment
```

### 3.3 ZIP Entry Name Mapping

A part name `/word/document.xml` maps to ZIP entry `word/document.xml` — the
leading `/` is dropped. Implementations MUST strip the leading `/` when writing
to ZIP and re-add it when reading from ZIP.

### 3.4 Interleaving

OPC 1st edition defined a part-interleaving feature (for streaming). ECMA-376
5th edition deprecates interleaving. Implementations SHOULD NOT produce
interleaved packages and MAY reject them.

---

## 4. `[Content_Types].xml`

### 4.1 Location and Purpose

`[Content_Types].xml` MUST be present at the root of every OPC package as ZIP
entry `[Content_Types].xml` (note the square brackets are literal). It declares
the content type for every part in the package.

### 4.2 XML Namespace and Root Element

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  ...
</Types>
```

Namespace URI: `http://schemas.openxmlformats.org/package/2006/content-types`

### 4.3 `Default` Element

Assigns a content type to all parts whose name ends with a given extension.

**Attributes:**
- `Extension` (required): file extension without the leading dot, case-insensitive.
  MUST NOT be empty. Examples: `xml`, `rels`, `png`, `jpeg`.
- `ContentType` (required): MIME type string (see §4.5).

```xml
<Default Extension="rels"
         ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
<Default Extension="xml"
         ContentType="application/xml"/>
<Default Extension="png"
         ContentType="image/png"/>
```

### 4.4 `Override` Element

Assigns a content type to a specific part by its full part name, overriding
any applicable `Default`.

**Attributes:**
- `PartName` (required): the absolute part name (with leading `/`).
- `ContentType` (required): MIME type string.

```xml
<Override PartName="/word/document.xml"
          ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/>
<Override PartName="/docProps/core.xml"
          ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
```

**Override takes precedence over Default.** If both a `Default` and an
`Override` match a part, the `Override` wins.

### 4.5 Content Type Syntax

Content types follow RFC 2045 / RFC 6838:
```
content-type = type "/" subtype *( ";" SP parameter )
type          = token
subtype       = token
parameter     = attribute "=" value
```

OPC-specific content types use structured suffixes (RFC 6838):
- `+xml` — XML part
- `+zip` — nested ZIP (rare)

Content types are **case-insensitive** (`text/XML` == `text/xml`).

### 4.6 Required `[Content_Types].xml` Entries

Every OPC package MUST have entries for:
1. All `.rels` parts (typically covered by a `Default Extension="rels"` entry)
2. Every other part in the package (covered by `Default` or `Override`)

`[Content_Types].xml` itself does NOT appear in `[Content_Types].xml` — it is
implicitly a package infrastructure file.

### 4.7 Common Content Types

| Part | Content Type |
|---|---|
| `.rels` files | `application/vnd.openxmlformats-package.relationships+xml` |
| Core properties | `application/vnd.openxmlformats-package.core-properties+xml` |
| DOCX main document | `application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml` |
| DOCX styles | `application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml` |
| DOCX numbering | `application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml` |
| DOCX settings | `application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml` |
| DOCX header | `application/vnd.openxmlformats-officedocument.wordprocessingml.header+xml` |
| DOCX footer | `application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml` |
| XLSX workbook | `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml` |
| XLSX worksheet | `application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml` |
| XLSX shared strings | `application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml` |
| XLSX styles | `application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml` |
| PPTX presentation | `application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml` |
| PPTX slide | `application/vnd.openxmlformats-officedocument.presentationml.slide+xml` |
| PPTX slide layout | `application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml` |
| PPTX slide master | `application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml` |
| Shared theme | `application/vnd.openxmlformats-officedocument.theme+xml` |
| PNG image | `image/png` |
| JPEG image | `image/jpeg` |
| GIF image | `image/gif` |
| Embedded font | `application/vnd.openxmlformats-officedocument.obfuscatedFont` |
| VBA macros | `application/vnd.ms-office.activeX+xml` |
| Digital signature origin | `application/vnd.openxmlformats-package.digital-signature-origin` |
| XML signature | `application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml` |

---

## 5. Relationships

### 5.1 Concept

A **relationship** is a typed directed association between a **source** and a
**target**. The source is either:
- The package itself (for package-level relationships), or
- A specific part (for part-level relationships).

The target is either:
- Another part within the package (internal relationship), or
- An external URI (external relationship).

Relationships are stored in dedicated **relationship parts** (`.rels` files)
and are NOT embedded in the content of the source part.

### 5.2 Naming Convention for Relationship Parts

| Source | Relationship part location |
|---|---|
| The package | `/_rels/.rels` → ZIP entry `_rels/.rels` |
| Part `/word/document.xml` | `/word/_rels/document.xml.rels` |
| Part `/xl/workbook.xml` | `/xl/_rels/workbook.xml.rels` |
| Part `/ppt/presentation.xml` | `/ppt/_rels/presentation.xml.rels` |

**Rule:** For a part at `/a/b/c.xml`, the relationship part is at
`/a/b/_rels/c.xml.rels`. The `_rels/` directory is inserted before the file
name, and `.rels` is appended to the full original file name.

### 5.3 Relationship Part XML Format

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1"
                Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument"
                Target="word/document.xml"/>
  <Relationship Id="rId2"
                Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties"
                Target="docProps/core.xml"/>
  <Relationship Id="rId3"
                Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail"
                Target="docProps/thumbnail.jpeg"/>
</Relationships>
```

Namespace URI: `http://schemas.openxmlformats.org/package/2006/relationships`

### 5.4 `Relationship` Attributes

| Attribute | Required | Description |
|---|---|---|
| `Id` | Yes | Unique identifier within this `.rels` file. Convention: `rId1`, `rId2`, etc. Uniqueness scope is per `.rels` file only. |
| `Type` | Yes | Relationship type URI (see §5.5). |
| `Target` | Yes | URI of the target. Relative URIs are resolved against the source part's URI. |
| `TargetMode` | No | `Internal` (default) or `External`. MUST be `External` for URIs with a scheme (e.g. `http:`). |

### 5.5 Relationship Type URIs

Relationship types are URI strings that identify the nature of the association.
They are compared as plain strings — not dereferenced.

**OPC infrastructure types:**

| Name | Type URI |
|---|---|
| Core properties | `http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties` |
| Digital signature origin | `http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin` |
| Digital signature | `http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature` |
| Thumbnail | `http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail` |

**OOXML document types (from `/_rels/.rels`):**

| Format | Type URI |
|---|---|
| DOCX main document | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument` |
| XLSX workbook | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument` |
| PPTX presentation | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument` |

**Common part-level relationship types:**

| Name | Type URI |
|---|---|
| Styles | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles` |
| Numbering | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/numbering` |
| Settings | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/settings` |
| Theme | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme` |
| Image | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/image` |
| Hyperlink | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink` |
| Worksheet | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet` |
| Shared strings | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/sharedStrings` |
| Slide | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide` |
| Slide layout | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout` |
| Slide master | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideMaster` |
| Header | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/header` |
| Footer | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/footer` |
| Font table | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/fontTable` |
| End notes | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/endnotes` |
| Foot notes | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/footnotes` |
| Comments | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments` |
| VBA project | `http://schemas.microsoft.com/office/2006/relationships/vbaProject` |

### 5.6 Target URI Resolution

Targets in relationship files are resolved as RFC 3986 relative references,
with the **source** being the part that owns the `.rels` file (not the `.rels`
file itself). For package-level relationships (`/_rels/.rels`), the base URI
is `/`.

Example: if `/word/_rels/document.xml.rels` contains `Target="styles.xml"`,
the absolute part name is `/word/styles.xml`.

### 5.7 Relationship Parts Are Not Content Parts

Relationship parts (`.rels` files) are infrastructure. They:
- Do NOT appear as sources or targets in other relationship parts.
- DO appear in `[Content_Types].xml` (covered by `Default Extension="rels"`).
- MUST NOT have their own `.rels` companion file.

---

## 6. Pack URI Scheme

The **pack URI** scheme (`pack:`) is used to construct absolute URIs that
identify parts within a specific package. It is used primarily in XML signatures
and cross-package references.

**Syntax:**
```
pack-uri = "pack://" authority "/" path
authority = pct-encoded-package-uri
```

The `authority` component is the package's URI, percent-encoded so that `/`
characters become `%2F`. The `path` component is the part name without the
leading `/`.

**Example:** Part `/word/document.xml` inside package `http://example.com/foo.docx`:
```
pack://http%3A%2F%2Fexample.com%2Ffoo.docx/word/document.xml
```

Most OOXML parsers do not need to handle pack URIs in normal document
processing; they appear mainly in digital signature transforms.

---

## 7. Core Properties Part

### 7.1 Location

The core properties part is typically located at `docProps/core.xml`, but its
actual location is defined by the package-level relationship with type:
```
http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties
```
There MUST be at most one core properties relationship per package.

### 7.2 Namespaces

The core properties part uses four namespaces:

| Prefix | Namespace URI | Used for |
|---|---|---|
| `cp` | `http://schemas.openxmlformats.org/package/2006/metadata/core-properties` | Root element and OPC-specific properties |
| `dc` | `http://purl.org/dc/elements/1.1/` | Dublin Core elements |
| `dcterms` | `http://purl.org/dc/terms/` | Dublin Core terms (created, modified) |
| `xsi` | `http://www.w3.org/2001/XMLSchema-instance` | `xsi:type` attribute |

### 7.3 XML Structure

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties
    xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties"
    xmlns:dc="http://purl.org/dc/elements/1.1/"
    xmlns:dcterms="http://purl.org/dc/terms/"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <dc:title>My Document</dc:title>
  <dc:creator>Alice</dc:creator>
  <cp:lastModifiedBy>Alice</cp:lastModifiedBy>
  <cp:revision>3</cp:revision>
  <dcterms:created xsi:type="dcterms:W3CDTF">2025-01-15T09:30:00Z</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">2025-03-22T14:45:00Z</dcterms:modified>
</cp:coreProperties>
```

### 7.4 Properties Reference

All properties are optional. Each appears at most once.

| Element | Namespace | Description |
|---|---|---|
| `dc:creator` | Dublin Core | Author(s) — free text |
| `dc:description` | Dublin Core | Abstract or description |
| `dc:identifier` | Dublin Core | Document identifier (URI or other) |
| `dc:language` | Dublin Core | BCP 47 language tag, e.g. `en-US` |
| `dc:subject` | Dublin Core | Subject or topic |
| `dc:title` | Dublin Core | Document title |
| `dcterms:created` | DC Terms | ISO 8601 creation datetime — requires `xsi:type="dcterms:W3CDTF"` |
| `dcterms:modified` | DC Terms | ISO 8601 last-modified datetime — requires `xsi:type="dcterms:W3CDTF"` |
| `cp:category` | OPC | Category or classification |
| `cp:contentStatus` | OPC | Status: `Draft`, `Final`, etc. |
| `cp:contentType` | OPC | Type of content (not the same as MIME content type) |
| `cp:keywords` | OPC | Keywords, comma- or space-separated |
| `cp:lastModifiedBy` | OPC | Name of the last editor |
| `cp:lastPrinted` | OPC | ISO 8601 datetime of last print — requires `xsi:type="dcterms:W3CDTF"` |
| `cp:revision` | OPC | Save count, stored as a string integer |
| `cp:version` | OPC | Application-defined version string |

### 7.5 W3CDTF Datetime Format

`dcterms:created`, `dcterms:modified`, and `cp:lastPrinted` MUST carry
`xsi:type="dcterms:W3CDTF"`. The W3CDTF profile of ISO 8601 permits:

```
YYYY                              ← year only
YYYY-MM                           ← year-month
YYYY-MM-DD                        ← date only
YYYY-MM-DDTHH:MM:SSZ              ← UTC datetime
YYYY-MM-DDTHH:MM:SS±HH:MM         ← offset datetime
YYYY-MM-DDTHH:MM:SS.sssZ          ← UTC with fractional seconds
```

In practice, Microsoft Office always writes UTC timestamps with second
resolution: `2025-03-22T14:45:00Z`. A parser MUST accept any W3CDTF form and
SHOULD preserve the original form on round-trip.

---

## 8. Thumbnail Part

### 8.1 Location

The thumbnail part is typically located at `docProps/thumbnail.jpeg` or
`docProps/thumbnail.png`, but its actual location is defined by the
package-level relationship with type:
```
http://schemas.openxmlformats.org/package/2006/relationships/metadata/thumbnail
```

There MUST be at most one thumbnail relationship per package.

### 8.2 Content

The thumbnail is a raster image. Supported content types are `image/jpeg` and
`image/png`. The thumbnail is used by file browsers and operating system
previews. OOXML parsers that do not generate thumbnails MUST preserve an
existing thumbnail on round-trip.

---

## 9. Digital Signature Parts

### 9.1 Three-Tier Structure

OPC digital signatures use a three-tier architecture:

**Tier 1 — Package relationship:**  
`/_rels/.rels` contains a relationship of type:
```
http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/origin
```
pointing to the signature origin part (e.g. `/_xmlsignatures/origin.sigs`).

**Tier 2 — Origin part:**  
`/_xmlsignatures/origin.sigs` is an **empty** XML part that serves as an anchor.
It has its own `.rels` file (`/_xmlsignatures/_rels/origin.sigs.rels`) with one
relationship per signature, each of type:
```
http://schemas.openxmlformats.org/package/2006/relationships/digital-signature/signature
```
pointing to individual signature parts (e.g. `/_xmlsignatures/sig1.xml`).

**Tier 3 — Individual signature parts:**  
Each `sig*.xml` is an XML Signature part containing a W3C XMLDSig `<Signature>`
element. The signature covers specific parts of the package (identified by
pack URIs) and the relationships transform.

### 9.2 Origin Part Format

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<mdssi:SignatureTime
    xmlns:mdssi="http://schemas.openxmlformats.org/package/2006/digital-signature">
</mdssi:SignatureTime>
```

The origin part is minimal. Microsoft Office produces a slightly different
empty document; the key fact is that the origin part has no meaningful content.

### 9.3 Parser Obligations

A round-trip OPC parser that does not implement digital signature verification:
- MUST preserve all `_xmlsignatures/` parts verbatim.
- MUST preserve the digital signature origin relationship in `/_rels/.rels`.
- MUST preserve the signature relationships in `/_xmlsignatures/_rels/origin.sigs.rels`.

Stripping digital signature parts on round-trip is data loss.

---

## 10. Cross-Cutting Implementation Gotchas

### 10.1 Part Name Case Sensitivity

OPC part names are case-insensitive for uniqueness checking but
case-preserving in storage. However, ZIP itself is case-sensitive. This
creates a mismatch: `word/document.xml` and `Word/Document.xml` are the same
OPC part but different ZIP entries.

**Practical impact:** When looking up a part by name (e.g. from a relationship
`Target`), always normalise to lowercase before comparison. When reading from a
real ZIP, build a case-insensitive index of all entries.

### 10.2 `[Content_Types].xml` Square Brackets

The file name `[Content_Types].xml` contains literal square brackets that are
valid in ZIP entry names but would be percent-encoded in a URI. Do NOT
percent-encode the brackets when writing the ZIP entry name. The ZIP entry
name is `[Content_Types].xml`, not `%5BContent_Types%5D.xml`.

### 10.3 Empty `.rels` Files Are Permitted

If a part has no relationships, there is no requirement to create a companion
`.rels` file. Similarly, `/_rels/.rels` must exist (it usually references the
main document and core properties), but parts with no outgoing relationships
simply have no `.rels` file.

### 10.4 Relationship `Id` Scope

Relationship `Id` values (`rId1`, etc.) are scoped to a single `.rels` file.
The same `Id` string may appear in different `.rels` files and they are
completely independent. Within a single `.rels` file, all `Id` values MUST be
unique.

### 10.5 Relative Target URIs and `.rels` Part URIs

Relative `Target` URIs in a `.rels` file are resolved against the URI of the
**source part** (the part that owns the `.rels` file), NOT against the `.rels`
file's own location. This distinction matters for nested parts.

Example: `/word/_rels/document.xml.rels` with `Target="styles.xml"` resolves to
`/word/styles.xml` (treating the source as `/word/document.xml`).

### 10.6 `Override` Beats `Default`

When `[Content_Types].xml` contains both a `Default` entry matching a part's
extension and an `Override` entry matching the part's full name, the `Override`
always wins. This is commonly seen for the main document part, which would
match `Default Extension="xml"` but has a more specific `Override`.

### 10.7 Content Type Omission Is an Error

If `[Content_Types].xml` contains no `Default` or `Override` entry for a part,
the content type is undefined and the package is non-conformant. A lenient
parser MAY attempt to infer the content type from the file extension; a strict
parser MUST reject the package. In practice, Microsoft Office-produced files
always have complete content type declarations.

### 10.8 The `standalone` XML Declaration

Most OPC XML parts (including `[Content_Types].xml` and `.rels` files) are
produced with `standalone="yes"` in the XML declaration. This is technically
advisory (there are no external DTD references), but parsers should not
reject it. On round-trip, preserve `standalone` if present.

### 10.9 BOM in XML Parts

XML parts MUST NOT contain a Byte Order Mark (BOM) if the encoding is UTF-8
(since UTF-8 has no byte-order ambiguity). BOM is only valid for UTF-16 or
UTF-32. Some tools produce UTF-8 with BOM; a lenient parser SHOULD strip the
BOM before passing bytes to an XML parser.

---

## 11. Minimal Package Structure

### 11.1 DOCX Minimal Package

```
[Content_Types].xml
_rels/.rels
word/document.xml
word/_rels/document.xml.rels
docProps/core.xml
```

`/_rels/.rels` contains at minimum:
- Relationship to `word/document.xml` (type: officeDocument)
- Relationship to `docProps/core.xml` (type: core-properties)

### 11.2 XLSX Minimal Package

```
[Content_Types].xml
_rels/.rels
xl/workbook.xml
xl/_rels/workbook.xml.rels
xl/worksheets/sheet1.xml
docProps/core.xml
```

### 11.3 PPTX Minimal Package

```
[Content_Types].xml
_rels/.rels
ppt/presentation.xml
ppt/_rels/presentation.xml.rels
ppt/slides/slide1.xml
ppt/slides/_rels/slide1.xml.rels
ppt/slideLayouts/slideLayout1.xml
ppt/slideLayouts/_rels/slideLayout1.xml.rels
ppt/slideMasters/slideMaster1.xml
ppt/slideMasters/_rels/slideMaster1.xml.rels
docProps/core.xml
```

The minimum PPTX has a deeper nesting because a slide requires a slide
layout, and a slide layout requires a slide master.

---

## 12. Conformance Classes

ECMA-376 Part 2 defines conformance at two levels:

| Class | Requirement |
|---|---|
| **Package consumer** | MUST accept well-formed OPC packages; SHOULD handle case-insensitive part names |
| **Package producer** | MUST produce well-formed OPC packages; MUST include `[Content_Types].xml`; MUST include `/_rels/.rels` |

`ooxmlsdk` is both a consumer and a producer (for round-trip). It MUST:
1. Read `[Content_Types].xml` and make content types available.
2. Read `/_rels/.rels` and all part-level `.rels` files.
3. Write a valid `[Content_Types].xml` covering every part it emits.
4. Write valid `.rels` files for all relationships.
5. Preserve any parts it does not understand (including digital signatures and
   thumbnails) verbatim.

---

## 13. Test Fixture Plan for OPC

Each fixture exercises one OPC behaviour in isolation. Fixtures are DOCX files
unless otherwise noted (DOCX is the simplest host format).

| Fixture | Part name | What to verify |
|---|---|---|
| `opc/core_properties.docx` | `docProps/core.xml` | All 16 core property fields round-trip intact; `xsi:type="dcterms:W3CDTF"` preserved |
| `opc/no_core_properties.docx` | (no `docProps/core.xml`) | Opens and saves without error even with no core properties relationship |
| `opc/thumbnail.docx` | `docProps/thumbnail.jpeg` | Thumbnail bytes identical before and after round-trip |
| `opc/override_content_type.docx` | `[Content_Types].xml` | Part with `Override` entry is read and written with the correct content type |
| `opc/multiple_rels.docx` | `word/_rels/document.xml.rels` | Multiple relationships in one `.rels` file; all `Id` values unique; all targets resolve |

---

*End of OPC Clean-Room Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
