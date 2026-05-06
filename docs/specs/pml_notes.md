# PresentationML Speaker Notes — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §13.3.5 (notes slide),
§13.3.4 (notes master); ISO/IEC 29500:2016 Part 1 §13.3; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/pml.xsd`.

---

## 1. Overview

Speaker notes in PresentationML are stored in separate XML parts — one per
slide — called notes slides. A notes slide contains a shape tree with two
primary shapes: the slide thumbnail placeholder and the notes text
placeholder. The notes text area is a standard DrawingML text body; it
supports the same rich-text formatting (bold, italic, bullets, hyperlinks)
as any other text shape.

```
Namespace URI (PresentationML): http://schemas.openxmlformats.org/presentationml/2006/main
Conventional prefix: p

Namespace URI (DrawingML): http://schemas.openxmlformats.org/drawingml/2006/main
Conventional prefix: a

Namespace URI (relationships): http://schemas.openxmlformats.org/officeDocument/2006/relationships
Conventional prefix: r
```

---

## 2. Notes Slide Part

### Part name and content type

| Property | Value |
|----------|-------|
| Typical part name | `ppt/notesSlides/notesSlide1.xml` (N increments per slide) |
| Content type | `application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml` |

The part name is not mandated by the spec (it is discovered via
relationships), but the pattern `ppt/notesSlides/notesSlideN.xml` is
universally used by producers.

### Relationship from slide to notes slide

Each slide that has a notes slide declares a relationship in its `.rels`
file:

| Property | Value |
|----------|-------|
| Relationship type | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide` |
| Target | Relative path from the slide part, e.g. `../notesSlides/notesSlide1.xml` |
| TargetMode | (absent — internal part) |

### Back-relationship from notes slide to slide

The notes slide's own `.rels` file must include a relationship pointing
back to its parent slide:

| Property | Value |
|----------|-------|
| Relationship type | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide` |
| Target | Relative path from the notes slide part, e.g. `../slides/slide1.xml` |

### Content Types entry

Each notes slide part must be declared with an `<Override>` in
`[Content_Types].xml`:

```xml
<Override PartName="/ppt/notesSlides/notesSlide1.xml"
  ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml"/>
```

---

## 3. Notes Master Part

### Part name and content type

| Property | Value |
|----------|-------|
| Typical part name | `ppt/notesMasters/notesMaster1.xml` |
| Content type | `application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml` |

### Relationship from notes slide to notes master

Each notes slide that is connected to a notes master declares:

| Property | Value |
|----------|-------|
| Relationship type | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster` |
| Target | Relative path from the notes slide part, e.g. `../notesMasters/notesMaster1.xml` |

The notes master is optional — a conforming package may omit it — but
virtually all real-world files produced by presentation applications
include one. When present it defines the page dimensions, background,
header/footer placeholder positions, and default paragraph/character
formatting that all notes slides inherit.

### Content Types entry

```xml
<Override PartName="/ppt/notesMasters/notesMaster1.xml"
  ContentType="application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml"/>
```

---

## 4. `<p:notes>` Root Element (CT_NotesSlide)

The root element of a notes slide XML part is `<p:notes>`.

### Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `showMasterSp` | xsd:boolean | true | Show shapes inherited from the notes master |
| `showMasterPhAnim` | xsd:boolean | true | Use animation settings from the notes master placeholder |

### Child element sequence

| # | Element | Multiplicity | Notes |
|---|---------|-------------|-------|
| 1 | `p:cSld` | exactly 1 | Common slide data (shape tree and background) |
| 2 | `p:clrMapOvr` | 0 or 1 | Colour map override |
| 3 | `p:extLst` | 0 or 1 | Extension list |

This structure mirrors `<p:sld>` (the slide root element). The
`<p:cSld>` child contains the `<p:spTree>` (shape tree) with all the
shapes on the notes slide.

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:notes xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
         xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
         xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>…</p:nvGrpSpPr>
      <p:grpSpPr>…</p:grpSpPr>
      <!-- placeholder shapes here -->
    </p:spTree>
  </p:cSld>
</p:notes>
```

---

## 5. Notes Placeholder Types

Shapes in the notes slide shape tree use the `<p:ph>` element
(placeholder) inside `<p:nvSpPr>/<p:nvPr>`. The following placeholder
types are relevant to notes slides:

| `type` value | `idx` | Description |
|-------------|-------|-------------|
| `body` | `1` | Notes text area — the actual speaker notes content |
| `sldImg` | (absent) | Slide thumbnail image |
| `dt` | (absent) | Date/time field |
| `sldNum` | (absent) | Slide number field |
| `hdr` | (absent) | Header text |
| `ftr` | (absent) | Footer text |

**`idx` attribute:** The body placeholder uses `idx="1"`. The `sldImg`
placeholder conventionally has no `idx` (or `idx` is absent). All other
placeholders follow the convention established on the notes master.

The `sldImg` placeholder is typically defined on the **notes master** and
not repeated on each notes slide. Notes slides that set `showMasterSp="1"`
(the default) inherit it automatically.

### Slide thumbnail placeholder on the notes master

The slide image placeholder carries an empty `<p:spPr/>` and empty
`<p:txBody/>` — it has no real XML content. The rendering application
draws the slide thumbnail into the bounding box defined by the shape's
transform (`<a:xfrm>`) at render time.

```xml
<!-- On notesMaster1.xml, not repeated on each notes slide -->
<p:sp>
  <p:nvSpPr>
    <p:cNvPr id="2" name="Slide Image Placeholder 1"/>
    <p:cNvSpPr><a:spLocks noGrp="1" noRot="1" noChangeAspect="1"/></p:cNvSpPr>
    <p:nvPr><p:ph type="sldImg"/></p:nvPr>
  </p:nvSpPr>
  <p:spPr/>
</p:sp>
```

---

## 6. Notes Text Area

The body placeholder (`type="body"`, `idx="1"`) on the notes slide holds
the speaker notes. Its `<p:txBody>` is a standard `CT_TextBody` with the
same structure as any other DrawingML text body:

```
<p:txBody>
  <a:bodyPr/>
  <a:lstStyle/>
  <a:p>…</a:p>   <!-- one or more paragraphs -->
</p:txBody>
```

Rich text is fully supported. Each `<a:p>` may contain `<a:r>` runs with
`<a:rPr>` for bold, italic, underline, font size, colour, and hyperlinks.
Multiple paragraphs produce multiple lines in the notes panel.

An empty notes area (no text entered by the user) typically appears as a
single empty paragraph:

```xml
<p:txBody>
  <a:bodyPr/>
  <a:lstStyle/>
  <a:p><a:endParaRPr lang="en-US" dirty="0"/></a:p>
</p:txBody>
```

---

## 7. XML Examples

### Example 1 — Notes slide `.rels` (back-reference to slide)

File path: `ppt/notesSlides/_rels/notesSlide1.xml.rels`

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide"
    Target="../slides/slide1.xml"/>
  <Relationship Id="rId2"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster"
    Target="../notesMasters/notesMaster1.xml"/>
</Relationships>
```

### Example 2 — `<p:notes>` with two paragraphs of speaker notes

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:notes xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
         xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main"
         xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
  <p:cSld>
    <p:spTree>
      <p:nvGrpSpPr>
        <p:cNvPr id="1" name=""/>
        <p:cNvGrpSpPr/>
        <p:nvPr/>
      </p:nvGrpSpPr>
      <p:grpSpPr>
        <a:xfrm>
          <a:off x="0" y="0"/>
          <a:ext cx="0" cy="0"/>
          <a:chOff x="0" y="0"/>
          <a:chExt cx="0" cy="0"/>
        </a:xfrm>
      </p:grpSpPr>

      <!-- Notes text area -->
      <p:sp>
        <p:nvSpPr>
          <p:cNvPr id="3" name="Notes Placeholder 2"/>
          <p:cNvSpPr><a:spLocks noGrp="1"/></p:cNvSpPr>
          <p:nvPr><p:ph type="body" idx="1"/></p:nvPr>
        </p:nvSpPr>
        <p:spPr/>
        <p:txBody>
          <a:bodyPr/>
          <a:lstStyle/>
          <a:p>
            <a:r>
              <a:rPr lang="en-US" dirty="0"/>
              <a:t>First paragraph of speaker notes.</a:t>
            </a:r>
          </a:p>
          <a:p>
            <a:r>
              <a:rPr lang="en-US" dirty="0"/>
              <a:t>Second paragraph with additional context.</a:t>
            </a:r>
          </a:p>
        </p:txBody>
      </p:sp>
    </p:spTree>
  </p:cSld>
</p:notes>
```

### Example 3 — Slide `.rels` entry adding the notesSlide relationship

File path: `ppt/slides/_rels/slide1.xml.rels`

```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
  <Relationship Id="rId1"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout"
    Target="../slideLayouts/slideLayout1.xml"/>
  <Relationship Id="rId2"
    Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide"
    Target="../notesSlides/notesSlide1.xml"/>
</Relationships>
```

---

## 8. Round-Trip Gotchas

1. **Back-relationship is required.** The notes slide's `.rels` file must
   include a relationship pointing back to the slide (type `.../slide`).
   Without it, strict validators will reject the package and many
   applications will fail to associate the notes with the correct slide.

2. **`idx="1"` on the body placeholder.** The body placeholder must carry
   `idx="1"`. An absent `idx` or a different value causes applications to
   misidentify the placeholder type and the notes text will not appear.

3. **Empty notes still require an `<a:p>`.** A body placeholder with a
   `<p:txBody>` containing zero paragraphs is schema-invalid. Emit at
   least one empty paragraph with `<a:endParaRPr/>`.

4. **Slide numbering in part names is not meaningful.** The SDK discovers
   which notes slide belongs to which presentation slide through the
   relationship, not the filename. `notesSlide3.xml` might belong to
   slide 1 if the relationships are set up that way.

5. **`showMasterSp="0"` suppresses inherited shapes.** Setting this
   attribute hides all shapes inherited from the notes master, including
   the slide image placeholder. Most producers never set it.

6. **Content Types `<Override>` is required for each notes slide.** The
   `.xml` extension is typically covered by a `<Default>` entry with type
   `application/xml`, which is different from the notes slide content type.
   Missing `<Override>` entries cause content type misidentification.

7. **The notes master is referenced from the notes slide, not from the
   presentation.** Unlike slide masters (which are referenced from
   `ppt/presentation.xml`), the notes master relationship originates in
   each notes slide's `.rels` file.

8. **`<p:spPr/>` may be empty on placeholder shapes.** The body and image
   placeholders on notes slides typically have an empty `<p:spPr/>` to
   inherit geometry, position, and size from the corresponding placeholder
   on the notes master.

9. **Rich text in notes is fully preserved.** Bold, italic, bullets, and
   hyperlinks in the `<p:txBody>` of the body placeholder round-trip
   without loss, because the notes text body is a standard
   `CT_TextBody`.

10. **`<p:txBody>` on the sldImg placeholder should be absent or empty.**
    Including paragraph content in the slide image placeholder produces
    undefined behaviour; most applications ignore it.

---

## 9. Relationship and Content Type URIs

```
Notes slide relationship type (slide → notes slide):
http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesSlide

Back-relationship type (notes slide → slide):
http://schemas.openxmlformats.org/officeDocument/2006/relationships/slide

Notes master relationship type (notes slide → notes master):
http://schemas.openxmlformats.org/officeDocument/2006/relationships/notesMaster

Notes slide content type:
application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml

Notes master content type:
application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml
```

---

## 10. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| PML-NO-01 | `test-data/pml/notes_basic.pptx` | Single slide with one-paragraph notes; slide `.rels` notesSlide entry; notes slide back-rels; `<Override>` in Content_Types |
| PML-NO-02 | `test-data/pml/notes_multipara.pptx` | Notes body placeholder with three paragraphs; one paragraph with a bold run |
| PML-NO-03 | `test-data/pml/notes_empty.pptx` | Slide with an explicit empty notes slide (single empty `<a:p>`) vs. slide with no notesSlide relationship |
| PML-NO-04 | `test-data/pml/notes_master.pptx` | Full notes master part with `sldImg` and `body` placeholders; notes slide references master via `rId` |
