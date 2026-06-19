# PresentationML Images and Media — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §19.3.1.37 (`p:pic`);
§19.3.1.25 (`p:nvPicPr`); §20.1.8.16 (`a:srcRect`); §20.1.8.58 (`a:stretch`);
§15.2.15 (Image Part); §15.2.2 (Audio Part); §15.2.28 (Video Part);
ISO/IEC 29500:2016 Part 1.

---

## 1. Overview

PPTX embeds images inside the package at `ppt/media/` and references them from
slides via relationships. Images use `<p:pic>` inside the slide shape tree.
Audio and video are attached to any shape or picture via `<p:nvPr>`.

---

## 2. Part Graph

```
ppt/slides/slide1.xml
ppt/slides/_rels/slide1.xml.rels  ─── rId2 image ──▶ ../media/image1.png
ppt/media/image1.png
```

`[Content_Types].xml` — image content types use `Default` entries by
extension or `Override` per file:

```xml
<Default Extension="png" ContentType="image/png"/>
<Default Extension="jpeg" ContentType="image/jpeg"/>
```

Supported image types:

| Extension | Content type |
|-----------|-------------|
| `png` | `image/png` |
| `jpeg` / `jpg` | `image/jpeg` |
| `gif` | `image/gif` |
| `tiff` | `image/tiff` |
| `svg` | `image/svg+xml` |
| `emf` | `image/x-emf` |
| `wmf` | `image/x-wmf` |

---

## 3. `<p:pic>` — Picture Element

```xml
<p:pic>
  <p:nvPicPr>
    <p:cNvPr id="2" name="Picture 1" descr="Alt text"/>
    <p:cNvPicPr>
      <a:picLocks noChangeAspect="1"/>
    </p:cNvPicPr>
    <p:nvPr/>
  </p:nvPicPr>
  <p:blipFill>
    <a:blip r:embed="rId2"/>
    <a:srcRect l="10000" t="10000" r="10000" b="10000"/>
    <a:stretch><a:fillRect/></a:stretch>
  </p:blipFill>
  <p:spPr>
    <a:xfrm>
      <a:off x="914400" y="914400"/>
      <a:ext cx="4572000" cy="3429000"/>
    </a:xfrm>
    <a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
  </p:spPr>
</p:pic>
```

### `<p:nvPicPr>` — Non-visual properties

| Child | Role |
|-------|------|
| `<p:cNvPr id name descr>` | Unique id, display name, optional alt-text description |
| `<p:cNvPicPr>` | Picture-specific NV props; `<a:picLocks noChangeAspect>` prevents aspect distortion |
| `<p:nvPr>` | Empty unless the picture is a media placeholder (`<p:ph>`) |

`<p:cNvPicPr>` is required even when empty (`<p:cNvPicPr/>`).

### `<p:blipFill>` — Image fill

| Child | Role |
|-------|------|
| `<a:blip r:embed>` | Relationship id pointing to the image part |
| `<a:srcRect l t r b>` | Crop: each edge offset as 1/1000th of a percent (0–100000; 10000 = 10%). Absent = no crop |
| `<a:stretch><a:fillRect/>` | Stretch the (possibly cropped) image to fill the bounding box |
| `<a:tile>` | Alternative to stretch: tile the image |

### `<p:spPr>` — Shape properties

Positions and sizes the picture frame via `<a:xfrm>`. `<a:prstGeom prst="rect">` clips the rendered image to a shape boundary and must be present.

---

## 4. Cropping Semantics

`<a:srcRect>` specifies how much of the source image to discard from each edge:

```
l = left fraction (e.g. l="10000" discards 10% from the left)
t = top fraction
r = right fraction
b = bottom fraction
```

All four attributes are optional and default to `0`. The visible area of the
source image is `[l, t, (1 - r), (1 - b)]` in normalised [0, 1]² coordinates.

---

## 5. Audio and Video

Audio and video are attached to any shape via `<p:nvPr>`:

```xml
<!-- Embedded audio on a picture shape -->
<p:nvPicPr>
  <p:cNvPr id="3" name="Audio 1"/>
  <p:cNvPicPr><a:picLocks noChangeAspect="1" noChangeArrowheads="1"/></p:cNvPicPr>
  <p:nvPr>
    <p:ph type="pic" idx="1"/>
    <p:audioFile r:link="rId3"/>
  </p:nvPr>
</p:nvPicPr>
```

| Element | Relationship type | Typical content type |
|---------|------------------|---------------------|
| `<p:audioFile r:link>` | `…/audio` | `audio/mpeg`, `audio/wav` |
| `<p:videoFile r:link>` | `…/video` | `video/mp4`, `video/avi` |

Audio and video use `r:link` (not `r:embed`) even for files stored inside
the package.

---

## 6. SDK Access Pattern

```rust
let slide = pres.slide_parts(&package).next()?;
for pic in &slide.root(&package).c_sld.sp_tree.pic {
  let r_embed = &pic.blip_fill.blip.r_embed;
  let img_part = slide.image_part_by_id(&package, r_embed)?;
}
```

Rust types under
`schemas_openxmlformats_org_presentationml_2006_main`:
`Pic`, `NonVisualPictureProperties`, `BlipFill`.

---

## 7. Round-Trip Gotchas

1. **`<a:blip r:embed>` is a relationship id, not a file path.** It must
   match a `Relationship Id` in the slide's `.rels` file.

2. **`<a:srcRect>` should be absent when there is no crop.** Writing
   `<a:srcRect l="0" t="0" r="0" b="0"/>` is equivalent but unnecessarily
   verbose; ooxmlsdk preserves whatever is in the XML.

3. **`<a:prstGeom prst="rect">` is required on `<p:pic>`.** Some renderers
   skip the picture entirely when this element is absent.

4. **`<p:cNvPicPr>` is required even if empty.** Use `<p:cNvPicPr/>` when
   no picture locks are needed.

5. **Image parts live at `ppt/media/`, not alongside slides.** The rels
   target is `../media/imageN.ext` relative to `ppt/slides/_rels/`.

6. **Audio and video use `r:link`, not `r:embed`.** Using `r:embed` for media
   silently disables playback in PowerPoint.

---

## 8. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| PML-IMG-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/slideshow/minimal_image.pptx` | Basic `<p:pic>` with PNG (already covered) |
| PML-IMG-02 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/pml/slide_pic.pptx` | `<p:pic>` with `<a:srcRect l t r b>` 10% crop all sides; `<a:picLocks noChangeAspect>`; `descr` alt-text on `<p:cNvPr>` |
