# MCE Clean-Room Specification
## Markup Compatibility and Extensibility for OOXML Parsers

**Purpose:** This document is a clean-room re-statement of the MCE semantics
defined in ISO/IEC 29500-3:2015 (4th edition), synthesised from the publicly
available standard, Microsoft's open-specification documents, and published
reference implementations. It is intended as an implementation guide and test
oracle for `ooxmlsdk` and similar Rust OOXML parsers.

**Sources used:**
- ISO/IEC 29500-3:2015 — structure and normative rules (publicly available
  sample from standards.iteh.ai)
- Eric White / Florian Reuter, "Markup Compatibility and Extensibility" blog
  post (ericwhite.com, 2012)
- Microsoft Learn: `[MS-XLSX]`, `[MS-DOCX]`, `[MS-PPTX]` open-spec pages
- ECMA-376 Part 5, 1st edition, rendered at c-rex.net
- docx4j `AlternateContent` XSD schema (javadoc.io)
- Microsoft Learn: Extension / extLst element documentation

This document contains **no verbatim text** from any copyrighted source.
All language is original. All examples are original or adapted to be clearly
distinct from spec examples.

---

## 1. What MCE Is

MCE is a small set of XML elements and attributes — all in the namespace
`http://schemas.openxmlformats.org/markup-compatibility/2006` (conventionally
prefixed `mc:`) — that a producer can embed in an OOXML document to communicate
two things to a consumer:

1. **"You can safely ignore this part of the markup if you don't understand
   it."** (Ignorable / ProcessContent)
2. **"Pick the first of these alternatives you understand; fall back to the
   last one if you understand none."** (AlternateContent / Choice / Fallback)

The goal is forward compatibility: a document produced by a newer application
can be opened by an older application without that application crashing or
silently corrupting the document.

MCE is processed by a **markup preprocessor** that sits between the raw XML
reader and the consuming application. The preprocessor transforms the XML
infoset before handing it to the application. The application itself never
sees MCE elements or attributes — by the time it receives the XML, the
preprocessor has already resolved everything.

---

## 2. Terminology

| Term | Definition |
|---|---|
| **MCE namespace** | `http://schemas.openxmlformats.org/markup-compatibility/2006` |
| **Understood namespace** | A namespace whose elements and attributes the consuming application can process |
| **Ignorable namespace** | A namespace declared as ignorable via `mc:Ignorable`; elements and attributes in it may be dropped by consumers that do not understand it |
| **Application configuration** | The set of namespace URIs the consuming application declares it understands; provided to the MCE preprocessor at initialisation time |
| **MCE processor** | The software module that implements the rules in this document |
| **Application-defined extension element** | An element whose children are exempt from normal MCE processing (see §8) |
| **Mismatch** | A condition where a `MustUnderstand` namespace is not in the application configuration |

---

## 3. The MCE Namespace URI and Conventional Prefix

```
Namespace URI:  http://schemas.openxmlformats.org/markup-compatibility/2006
Conventional prefix: mc
```

All MCE elements and attributes MUST be in this namespace. Documents may use
any prefix, but `mc` is conventional and used throughout this document.

---

## 4. MCE Elements and Attributes — Reference

### 4.1 `mc:Ignorable` (attribute)

**Where it appears:** Any element in any namespace (most commonly on root
elements of an XML part).

**Value:** A whitespace-delimited list of namespace prefixes. Each prefix MUST
be bound to a namespace URI via an in-scope `xmlns:` declaration at the element
carrying the `mc:Ignorable` attribute or at an ancestor.

**Semantics:** Each prefix in the list identifies a namespace. For any
consuming application that does not have that namespace in its application
configuration (i.e. does not "understand" it), the MCE preprocessor MUST:

- Remove every element whose expanded name is in the identified namespace,
  along with its entire subtree (unless overridden by `mc:ProcessContent`).
- Remove every attribute whose expanded name is in the identified namespace.

For a consuming application that **does** understand the namespace, the
preprocessor leaves those elements and attributes in place.

**Scope:** The `mc:Ignorable` declaration applies to the element it appears on
and all its descendants, until overridden by a new `mc:Ignorable` attribute on
a descendant element.

**Validity constraints (producer):**
- Every prefix listed in `mc:Ignorable` MUST have an in-scope `xmlns:`
  declaration.
- A prefix MUST NOT appear in both `mc:Ignorable` and `mc:MustUnderstand` on
  the same element.

**Example:**

```xml
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:w14="http://schemas.microsoft.com/office/word/2010/wordml"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    mc:Ignorable="w14">
  <w:body>
    <w:p>
      <w:r>
        <w:rPr>
          <w:sz w:val="48"/>
          <w14:glow w14:rad="50000">
            <w14:schemeClr w14:val="accent1"/>
          </w14:glow>
        </w:rPr>
        <w:t>styled text</w:t>
      </w:r>
    </w:p>
  </w:body>
</w:document>
```

A consumer that does not understand `w14` receives, after preprocessing:

```xml
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>
    <w:p>
      <w:r>
        <w:rPr>
          <w:sz w:val="48"/>
          <!-- w14:glow and its subtree are removed -->
        </w:rPr>
        <w:t>styled text</w:t>
      </w:r>
    </w:p>
  </w:body>
</w:document>
```

---

### 4.2 `mc:ProcessContent` (attribute)

**Where it appears:** Any element that also carries `mc:Ignorable`, or on
`mc:Choice` or `mc:Fallback` elements.

**Value:** A whitespace-delimited list of qualified element names
(`prefix:localname`). Every prefix in the list MUST be listed in the
co-located `mc:Ignorable` attribute on the same element.

**Semantics:** Normally, when an ignorable element is removed, its entire
subtree is discarded. `mc:ProcessContent` creates an exception: when the MCE
preprocessor removes an ignorable element whose expanded name matches an entry
in `mc:ProcessContent`, it does NOT discard that element's children. Instead,
it promotes the children into the position where the parent element was.

This is the "see-through wrapper" pattern: the wrapper element is removed but
its contents survive.

**Example:**

```xml
<w:document
    xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"
    xmlns:ext="http://example.com/ext"
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    mc:Ignorable="ext"
    mc:ProcessContent="ext:group">
  <w:body>
    <ext:group>
      <w:p><w:r><w:t>first</w:t></w:r></w:p>
      <w:p><w:r><w:t>second</w:t></w:r></w:p>
    </ext:group>
  </w:body>
</w:document>
```

After preprocessing by a consumer that does not understand `ext`:

```xml
<w:document xmlns:w="...">
  <w:body>
    <w:p><w:r><w:t>first</w:t></w:r></w:p>
    <w:p><w:r><w:t>second</w:t></w:r></w:p>
  </w:body>
</w:document>
```

Without `mc:ProcessContent`, `<ext:group>` and all its children would have
been silently discarded.

---

### 4.3 `mc:MustUnderstand` (attribute)

**Where it appears:** Any element in any namespace.

**Value:** A whitespace-delimited list of namespace prefixes. Each prefix MUST
have an in-scope `xmlns:` declaration.

**Semantics:** This is the hard stop mechanism. If any namespace listed in
`mc:MustUnderstand` is NOT in the application's configuration, the MCE
preprocessor MUST signal an error and MUST NOT produce an output document.
Processing halts immediately.

This is the inverse of `mc:Ignorable`. Use it when a feature is so central to
a document's meaning that a consumer ignoring it would produce silently wrong
results.

**Validity constraint:** A prefix MUST NOT appear in both `mc:MustUnderstand`
and `mc:Ignorable` on the same element.

**Example:**

```xml
<w:document
    xmlns:w="..."
    xmlns:w22="http://schemas.microsoft.com/office/word/2022/wordml"
    xmlns:mc="..."
    mc:MustUnderstand="w22">
  <!-- content using w22 features that cannot be gracefully degraded -->
</w:document>
```

A consumer that does not understand `w22` MUST report an error. It MUST NOT
silently produce a partial document.

---

### 4.4 `mc:AlternateContent` (element)

**Where it appears:** Anywhere a regular element could appear in the host
markup (as a child of any element, including elements in the MCE namespace
itself).

**Children:** Zero or more `mc:Choice` elements, optionally followed by
exactly one `mc:Fallback` element. The spec allows zero `mc:Choice` elements
(though this is not useful in practice) and allows zero or one `mc:Fallback`.

**Semantics:** This is the `switch/case/default` pattern. The MCE preprocessor
evaluates `mc:Choice` elements in document order. The first `mc:Choice` whose
`Requires` attribute is fully satisfied is selected; its children replace the
entire `mc:AlternateContent` element in the output. If no `mc:Choice` is
selected and a `mc:Fallback` exists, its children replace the
`mc:AlternateContent` element. If no `mc:Choice` is selected and there is no
`mc:Fallback`, the `mc:AlternateContent` element is removed with no children
promoted.

**The `mc:AlternateContent` element itself is always removed from the output.**
The output contains either the selected `mc:Choice`'s children, the
`mc:Fallback`'s children, or nothing.

**XSD schema (informative):**

```xsd
<xs:element name="AlternateContent">
  <xs:complexType>
    <xs:sequence>
      <xs:element name="Choice" minOccurs="0" maxOccurs="unbounded">
        <xs:complexType>
          <xs:sequence>
            <xs:any minOccurs="0" maxOccurs="unbounded"/>
          </xs:sequence>
          <xs:attribute name="Requires" type="xs:string" use="required"/>
          <xs:anyAttribute namespace="##other"/>
        </xs:complexType>
      </xs:element>
      <xs:element name="Fallback" minOccurs="0" maxOccurs="1">
        <xs:complexType>
          <xs:sequence>
            <xs:any minOccurs="0" maxOccurs="unbounded"/>
          </xs:sequence>
          <xs:anyAttribute namespace="##other"/>
        </xs:complexType>
      </xs:element>
    </xs:sequence>
    <xs:anyAttribute namespace="##other"/>
  </xs:complexType>
</xs:element>
```

---

### 4.5 `mc:Choice` (element)

**Where it appears:** As a direct child of `mc:AlternateContent` only.

**Attributes:**
- `Requires` (required): A whitespace-delimited list of namespace prefixes.
  Each prefix MUST have an in-scope `xmlns:` declaration. This attribute is
  in no namespace (it is a plain attribute of `mc:Choice`).
- May carry `mc:Ignorable`, `mc:ProcessContent`, and `mc:MustUnderstand` for
  use within its own subtree.

**Semantics:** The MCE preprocessor selects this `mc:Choice` element if and
only if every namespace URI bound by every prefix in the `Requires` list is in
the application's configuration. If the `mc:Choice` is selected, its children
are emitted into the output in place of the surrounding `mc:AlternateContent`.
The `mc:Choice` element itself is stripped; only its children survive.

**Multiple prefixes in `Requires`:** All named namespaces must be understood
for the `mc:Choice` to be selected. It is an AND condition, not OR.

**Example with multiple choices:**

```xml
<mc:AlternateContent
    xmlns:mc="http://schemas.openxmlformats.org/markup-compatibility/2006"
    xmlns:v3="http://example.com/v3"
    xmlns:v2="http://example.com/v2">
  <mc:Choice Requires="v3">
    <v3:newWidget color="blue"/>
  </mc:Choice>
  <mc:Choice Requires="v2">
    <v2:widget color="blue"/>
  </mc:Choice>
  <mc:Fallback>
    <legacyWidget/>
  </mc:Fallback>
</mc:AlternateContent>
```

- Consumer understanding `v3`: receives `<v3:newWidget color="blue"/>`
- Consumer understanding `v2` but not `v3`: receives `<v2:widget color="blue"/>`
- Consumer understanding neither: receives `<legacyWidget/>`

---

### 4.6 `mc:Fallback` (element)

**Where it appears:** As the last direct child of `mc:AlternateContent` only.
There may be at most one `mc:Fallback` per `mc:AlternateContent`.

**Semantics:** Selected when no `mc:Choice` sibling was selected. Its children
are emitted in place of `mc:AlternateContent`. May carry `mc:Ignorable`,
`mc:ProcessContent`, and `mc:MustUnderstand` for use within its own subtree.

---

## 5. The Three-Step Processing Model

ISO/IEC 29500-3:2015 §9 defines MCE processing as three sequential passes over
the XML infoset. A conforming MCE processor MUST produce the same result as
this reference model, though it MAY use a different implementation strategy
(e.g. a single-pass streaming approach) as long as the output is identical.

### Step 1 — Resolve Ignorable and ProcessContent

Traverse the XML tree depth-first. At each element:

1. Determine the set of ignorable namespaces in scope by collecting all
   `mc:Ignorable` attribute values on the element and its ancestors
   (inner declarations shadow outer ones for the same prefix).
2. Determine the set of process-content element names from `mc:ProcessContent`
   attributes in scope.
3. For each child element:
   a. If the child's namespace URI is in the ignorable set AND that namespace
      URI is NOT in the application configuration:
      - If the child's qualified name is in the process-content set: remove the
        child element but keep its children (promote them to the current level).
      - Otherwise: remove the child element and its entire subtree.
   b. Otherwise: recurse into the child element.
4. For each attribute on the current element:
   - If the attribute's namespace URI is in the ignorable set AND that
     namespace URI is NOT in the application configuration: remove the
     attribute.
5. Remove all `mc:Ignorable` and `mc:ProcessContent` attributes from the
   output.

**Important:** Namespace declarations (`xmlns:*`) are removed from the output
for namespaces that have been fully ignored.

### Step 2 — Resolve AlternateContent

Traverse the result of Step 1 depth-first. At each `mc:AlternateContent`
element:

1. Evaluate `mc:Choice` children in document order.
2. For each `mc:Choice`, check if every namespace URI in its `Requires`
   attribute list is in the application configuration.
3. Select the first `mc:Choice` where all required namespaces are understood.
4. If a `mc:Choice` is selected: replace the `mc:AlternateContent` element
   with the selected `mc:Choice`'s children (not the `mc:Choice` element
   itself).
5. If no `mc:Choice` is selected and a `mc:Fallback` exists: replace the
   `mc:AlternateContent` element with the `mc:Fallback`'s children.
6. If no `mc:Choice` is selected and no `mc:Fallback` exists: remove the
   `mc:AlternateContent` element entirely (no children promoted).

**Recursion:** Steps 1 and 2 apply recursively within the selected
`mc:Choice` or `mc:Fallback` content.

### Step 3 — Resolve MustUnderstand and Produce Output

Traverse the result of Steps 1 and 2. At each element:

1. If the element has a `mc:MustUnderstand` attribute:
   a. For each prefix in the attribute value, resolve the namespace URI.
   b. If any resolved namespace URI is NOT in the application configuration:
      signal an error and halt. No output is produced.
2. Remove all `mc:MustUnderstand` attributes from surviving elements.
3. Remove all remaining attributes in the MCE namespace.
4. The resulting tree (with all MCE namespace attributes stripped) is the
   output document.

---

## 6. Application-Defined Extension Elements (`extLst` Pattern)

This is the most important practical nuance for OOXML parsers, and the root
cause of ooxmlsdk issues #8 and #11.

### 6.1 The Pattern

OOXML uses a parallel extension mechanism alongside MCE called **extension
lists** (`extLst`). The spec defines `extLst` elements at specific locations
in WordprocessingML, SpreadsheetML, and PresentationML. Each `extLst` contains
zero or more `ext` elements. Each `ext` has a `uri` attribute that identifies
the extension type, and arbitrary children.

```xml
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="1" name="Shape 1"/>
    <p:cNvSpPr>
      <a:extLst>
        <a:ext uri="{FF2B5EF4-FFF2-40B4-BE49-F238E27FC236}">
          <a16:creationId
              xmlns:a16="http://schemas.microsoft.com/office/drawing/2014/main"
              id="{E6D9B6CF-E7AF-4A69-84AC-8A6C0B04A2A4}"/>
        </a:ext>
      </a:extLst>
    </p:cNvSpPr>
  </p:nvSpPr>
</p:sp>
```

### 6.2 Why This Breaks Naïve MCE Processors

The namespace `a16:` used inside the `ext` element is NOT declared as
`mc:Ignorable` anywhere in the document. From a strict MCE perspective, a
processor encountering `a16:creationId` with no `mc:Ignorable` declaration
should treat it as an error — an unknown, non-ignorable element.

But that is wrong. The correct behaviour is that `extLst`/`ext` elements
are **application-defined extension elements**: by spec definition, the MCE
processor MUST suspend normal MCE processing inside them. The contents of an
`ext` element are opaque to the MCE processor. The consumer application
decides whether it can process the extension based on the `uri` attribute.

### 6.3 The Rule

From ISO/IEC 29500-3 §8 and the normative text in Part 1:

> Markup namespaces within application-defined extension elements SHALL NOT
> be required to be listed in the `mc:Ignorable` attribute. An MCE processor
> MUST NOT apply ignorable-namespace processing to the content of an
> application-defined extension element.

The consuming application — not the MCE preprocessor — is responsible for
deciding what to do with extension content. If the consumer does not
understand the extension URI, it MUST preserve the `ext` element and its
children verbatim (so long as the parent structure is not removed).

### 6.4 Which Elements Are Application-Defined Extension Elements?

The following elements in OOXML act as application-defined extension element
containers. The MCE processor MUST suspend MCE namespace enforcement inside
them:

**DrawingML (all three formats):**
- `a:extLst` and its `a:ext` children
- Any element named `extLst` in DrawingML-related namespaces

**WordprocessingML:**
- `w:extLst` / `w:ext`

**SpreadsheetML:**
- `x:extLst` / `x:ext`

**PresentationML:**
- `p:extLst` / `p:ext`

**The key rule:** Any element with local name `extLst` in an OOXML namespace,
and by extension any `ext` child, is an application-defined extension element.
The MCE processor MUST pass through their contents verbatim.

---

## 7. Producer Requirements (for Round-Trip Fidelity)

A parser that round-trips OOXML documents (reads and re-serialises) MUST
preserve MCE markup faithfully, because the documents will be opened by other
applications that need the MCE information. This means:

1. **Preserve `mc:Ignorable` attributes.** Do not remove them during
   deserialisation and re-add them during serialisation.
2. **Preserve `mc:ProcessContent` attributes** under the same conditions.
3. **Preserve `mc:AlternateContent` structures.** A round-trip parser that
   is not the final consumer MUST NOT resolve AlternateContent; it must keep
   all `mc:Choice` and `mc:Fallback` branches intact.
4. **Preserve extension content.** `extLst` / `ext` children must survive
   round-trip unchanged, even if their namespaces are unknown to the parser.
5. **Preserve namespace declarations** for all prefixes used in `mc:Ignorable`,
   `mc:ProcessContent`, `mc:MustUnderstand`, and `Requires`.

**For `ooxmlsdk` specifically:** The library is a structural parser, not a
consumer-final application. It MUST preserve all MCE markup. It MUST NOT
resolve `mc:AlternateContent` or strip ignorable namespaces during a round-
trip operation. Issues #8 and #11 are both failures of this requirement — the
parser is rejecting or failing on MCE constructs that it should be storing
opaquely and re-emitting unchanged.

---

## 8. Conformance Classes

ISO/IEC 29500-3 defines three conformance classes:

| Class | Requirement |
|---|---|
| **MCE producer** | MAY use MCE elements and attributes; MUST produce conformant MCE markup when doing so |
| **MCE preprocessor** | MUST implement the three-step processing model; MUST be initialised with an application configuration |
| **MCE consumer** | A consumer that uses a conformant MCE preprocessor to process documents before consuming them |

`ooxmlsdk` in its current role is an **MCE producer** (when writing) and
a partial **MCE preprocessor** (when reading). It does not need to be a
full preprocessor — it just needs to not crash on valid MCE input.

---

## 9. Common Failure Modes in Parsers

This section catalogues the specific MCE-related failures that OOXML parsers
commonly exhibit, mapped to the rules above.

| Failure | Rule violated | ooxmlsdk issue |
|---|---|---|
| Parser rejects `mc:AlternateContent` element as unknown element | §4.4 — AlternateContent must be accepted as a valid child anywhere | #11 |
| Parser rejects extension namespace element inside `extLst` | §6.3 — MCE processing must be suspended inside extension elements | #8 |
| Parser strips `mc:Ignorable` attributes on round-trip | §7 — Ignorable attributes must be preserved for downstream consumers | N/A |
| Parser resolves `mc:AlternateContent` instead of preserving all branches | §7 — Round-trip parsers must not resolve AlternateContent | N/A |
| Parser crashes on `mc:MustUnderstand` for unknown namespace | §4.3 — MustUnderstand triggers an error, not a crash | N/A |
| `mc:Choice.Requires` namespace not understood → parser silently drops content | §5 Step 2 — Fallback must be used if no Choice is satisfied | N/A |
| Namespace declared in `mc:Ignorable` but prefix not in scope | §4.1 validity constraint — must be an error at produce time | N/A |

---

## 10. MCE Namespace URIs Seen in Real OOXML Files

These are the most common extension namespaces that appear inside `mc:Ignorable`
or `mc:AlternateContent` structures in Microsoft Office-produced files. A parser
that crashes on any of these is not handling real-world documents.

| Prefix (typical) | Namespace URI | Introduced |
|---|---|---|
| `w14` | `http://schemas.microsoft.com/office/word/2010/wordml` | Word 2010 |
| `w15` | `http://schemas.microsoft.com/office/word/2012/wordml` | Word 2013 |
| `w16cid` | `http://schemas.microsoft.com/office/word/2016/wordml/cid` | Word 2016 |
| `w16se` | `http://schemas.microsoft.com/office/word/2015/wordml/symex` | Word 2016 |
| `cx` | `http://schemas.microsoft.com/office/drawing/2014/chartex` | Office 2016 |
| `a14` | `http://schemas.microsoft.com/office/drawing/2010/main` | Office 2010 |
| `a16` | `http://schemas.microsoft.com/office/drawing/2014/main` | Office 2016 |
| `p14` | `http://schemas.microsoft.com/office/powerpoint/2010/main` | PPT 2010 |
| `p15` | `http://schemas.microsoft.com/office/powerpoint/2012/main` | PPT 2013 |
| `x14` | `http://schemas.microsoft.com/office/spreadsheetml/2009/9/main` | Excel 2010 |
| `x15` | `http://schemas.microsoft.com/office/spreadsheetml/2010/11/main` | Excel 2013 |
| `xr` | `http://schemas.microsoft.com/office/spreadsheetml/2014/revision` | Excel 2016 |
| `xr2` | `http://schemas.microsoft.com/office/spreadsheetml/2015/revision2` | Excel 2016 |
| `xr3` | `http://schemas.microsoft.com/office/spreadsheetml/2016/revision3` | Excel 2019 |

---

## 11. Minimal Fixture Files for Testing

These minimal XML fragments define the test cases that a conformant MCE
parser must handle. Each is described as what the raw XML part contains and
what the expected behaviour is.

### Fixture MCE-01: Ignorable attribute, unknown namespace

**File:** `test-data/mce/ignorable_unknown_ns.docx`

**Expected behaviour (round-trip parser):** Opens without error. Saves without
error. The saved file re-opens without error. The `mc:Ignorable` attribute and
the `ext:decoration` element are preserved in the saved output.

---

### Fixture MCE-02: ProcessContent, wrapper element

**File:** `test-data/mce/process_content.docx`

**Expected:** Opens and saves without error. Attributes preserved.

---

### Fixture MCE-03: AlternateContent with fallback

**File:** `test-data/mce/alternate_content_fallback.docx`

**Expected:** Opens and saves without error. All three elements
(`mc:AlternateContent`, `mc:Choice`, `mc:Fallback`) are preserved in saved
output.

---

### Fixture MCE-04: AlternateContent in PPTX (DrawingML)

**File:** `test-data/mce/alternate_content_pptx.pptx`

**Expected:** Opens and saves without error. This is the core of issue #11.

---

### Fixture MCE-05: extLst with unknown extension namespace

**File:** `test-data/mce/extlst_unknown_ns.pptx`

Note: `a16` is NOT declared as `mc:Ignorable` anywhere. The MCE processor
must treat `a:extLst` as an application-defined extension element and pass
through `a16:creationId` without error.

**Expected:** Opens and saves without error. This is the core of issue #8.

---

### Fixture MCE-06: MustUnderstand with understood namespace

**File:** `test-data/mce/must_understand_ok.docx`

`w` is in every OOXML parser's understood set. This MUST succeed.

**Expected:** Opens and saves without error.

---

### Fixture MCE-07: Nested AlternateContent

**File:** `test-data/mce/nested_alternate_content.docx`

**Expected:** Opens and saves without error. Nested structure preserved intact.

---

## 12. Test Assertions for ooxmlsdk

For each fixture, the round-trip test MUST assert:

1. `WordprocessingDocument::new_from_file(path)` or
   `PresentationDocument::new_from_file(path)` returns `Ok(_)`.
2. `.save(&mut cursor)` returns `Ok(())`.
3. Re-opening from the cursor bytes returns `Ok(_)`.
4. For fixtures MCE-01 through MCE-07: the saved XML parts contain the
   expected MCE attributes and elements (parse the saved ZIP, extract the
   relevant part, check for presence of key strings or elements).

Assertion 4 is the difference between a round-trip test and a corruption test.
It is what distinguishes "the parser didn't crash" from "the parser actually
preserved the MCE information correctly."

---

## 13. Relationship to ooxmlsdk Issues

| Issue | Root cause (per this spec) | Relevant fixtures |
|---|---|---|
| #11 — PPTX rejects `mc:AlternateContent` | The generated part schemas do not include `mc:AlternateContent` as a valid child element (missing "ubiquitous-child" declaration in code generator). Per §4.4, `mc:AlternateContent` may appear as a child of ANY element. | MCE-03, MCE-04, MCE-07 |
| #8 — `NonVisualDrawingPropertiesExtension` parse error | The parser attempts to validate the contents of an `extLst`-style extension against a known schema. Per §6.3, extension element contents are opaque and must not be MCE-validated. | MCE-05 |

**Fix direction for #11:** The code generator (`ooxmlsdk-build`) must inject
`mc:AlternateContent` as a permitted child into every generated element type,
not just those that explicitly reference it in the OOXML schemas. The ISO spec
is clear that `mc:AlternateContent` is universally valid — it does not appear
in individual element XSDs because it is a cross-cutting concern.

**Fix direction for #8:** The deserialiser for `extLst`/`ext` (and equivalent
elements) must not attempt to deserialise their children against any known
schema. They must be preserved as raw XML bytes or a generic `Any` node type.

---

*End of MCE Clean-Room Specification.*
*Document version: 1.0 — compiled May 2026.*
*This document may be freely used, modified, and redistributed.*
