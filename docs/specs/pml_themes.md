# PresentationML Themes — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §19.3.1.51 (`p:clrMapOvr`); §14.2.7 (Theme Part); §20.1.4 (DrawingML colour scheme); §20.1.5 (DrawingML font scheme); §20.1.6 (DrawingML format scheme); ISO/IEC 29500:2016 Part 1; XSD `dml-main.xsd`, `pml.xsd`.

---

## 1. Overview

Themes define the colour palette and typography applied to a presentation. The
`ThemePart` lives at `ppt/theme/themeN.xml` and is referenced from the
slide master via a `theme` relationship. Each slide and layout either inherits
the master's colour map or overrides it with `<p:clrMapOvr>`.

See also `docs/specs/drawingml.md` §Theme for the DrawingML theme element
itself; this spec focuses on the PML integration layer.

---

## 2. Part Graph

```
ppt/slideMasters/slideMaster1.xml
ppt/slideMasters/_rels/slideMaster1.xml.rels
                    ─── rIdN theme ──▶ ../theme/theme1.xml
ppt/theme/theme1.xml
```

`[Content_Types].xml`:
```xml
<Override PartName="/ppt/theme/theme1.xml"
  ContentType="application/vnd.openxmlformats-officedocument.theme+xml"/>
```

---

## 3. Theme XML Structure

```xml
<a:theme xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main"
         name="Office Theme">
  <a:themeElements>
    <a:clrScheme name="…">…</a:clrScheme>
    <a:fontScheme name="…">…</a:fontScheme>
    <a:fmtScheme name="…">…</a:fmtScheme>
  </a:themeElements>
</a:theme>
```

### `<a:clrScheme>` — 12 named colour slots

| Slot | Role |
|------|------|
| `dk1` | Dark text 1 (typically black) |
| `lt1` | Light background 1 (typically white) |
| `dk2` | Dark text 2 |
| `lt2` | Light background 2 |
| `accent1` – `accent6` | Six accent colours |
| `hlink` | Hyperlink colour |
| `folHlink` | Followed hyperlink colour |

Each slot holds exactly one colour source: `<a:srgbClr val="RRGGBB"/>`,
`<a:sysClr val="…" lastClr="…"/>`, or `<a:prstClr val="…"/>`.

### `<a:fontScheme>` — major and minor fonts

```xml
<a:fontScheme name="Office">
  <a:majorFont>
    <a:latin typeface="Calibri Light"/>
    <a:ea typeface=""/>
    <a:cs typeface=""/>
    <!-- optional per-script substitutions: <a:font script="…" typeface="…"/> -->
  </a:majorFont>
  <a:minorFont>
    <a:latin typeface="Calibri"/>
    <a:ea typeface=""/>
    <a:cs typeface=""/>
  </a:minorFont>
</a:fontScheme>
```

Run properties reference fonts via `<a:latin typeface="+mj-lt"/>` (major) or
`<a:latin typeface="+mn-lt"/>` (minor); the theme resolves these aliases at
render time.

### `<a:fmtScheme>` — format preset lists

Each of the four lists must contain **exactly 3** entries
(subtle / moderate / intense):

| List | Content |
|------|---------|
| `<a:fillStyleLst>` | 3 fill presets |
| `<a:lnStyleLst>` | 3 line presets |
| `<a:effectStyleLst>` | 3 effect presets |
| `<a:bgFillStyleLst>` | 3 background-fill presets |

Preset entries use `<a:schemeClr val="phClr">` as a placeholder colour that
resolves to the shape's effective accent colour at render time.

---

## 4. Colour References and Transforms

Shapes reference the theme palette via `<a:schemeClr val="accent1">` (and the
other 11 slot names). Optional transform children modulate the base colour:

| Element | Meaning | Range |
|---------|---------|-------|
| `<a:tint val="N"/>` | Blend toward white | 0–100000 (100000 = pure white) |
| `<a:shade val="N"/>` | Blend toward black | 0–100000 (100000 = no darkening) |
| `<a:lumMod val="N"/>` | Scale luminance | 0–200000 (100000 = no change) |
| `<a:lumOff val="N"/>` | Offset luminance | −100000–100000 |
| `<a:satMod val="N"/>` | Scale saturation | 0–200000 |
| `<a:alpha val="N"/>` | Opacity | 0–100000 |

Transforms are applied in declaration order; multiple transforms on one
`<a:schemeClr>` are legal.

---

## 5. Colour Map Override (`<p:clrMapOvr>`)

Every `<p:sld>` and `<p:sldLayout>` carries a `<p:clrMapOvr>` element after
`<p:cSld>` and `<p:transition>` (if present):

```xml
<!-- Inherit the master's colour map (most slides) -->
<p:clrMapOvr>
  <a:masterClrMapping/>
</p:clrMapOvr>

<!-- Override: remap slot names for this slide only -->
<p:clrMapOvr>
  <a:overrideClrMapping
      bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2"
      accent1="accent1" accent2="accent2" accent3="accent3"
      accent4="accent4" accent5="accent5" accent6="accent6"
      hlink="hlink" folHlink="folHlink"/>
</p:clrMapOvr>
```

`<a:overrideClrMapping>` remaps logical slot names to source slot names. **All
12 attributes are required** even when most are identity mappings. The attribute
values are source slot names from `<a:clrScheme>`.

A common use: swapping `bg1`/`tx1` with `dk1`/`lt1` produces a "dark slide"
within an otherwise light-themed presentation without a separate theme file.

---

## 6. Theme Inheritance Chain

Precedence (most specific wins):

1. Direct `<a:srgbClr>` on the shape (no theme dependency)
2. `<a:schemeClr>` resolved via the slide's `<p:clrMapOvr>`
3. Mapped slot resolved via `<a:clrScheme>` in the theme

A slide with `<a:overrideClrMapping bg1="dk1" tx1="lt1" …>` renders
`<a:schemeClr val="bg1">` as the theme's `dk1` colour on that slide only.

---

## 7. SDK Access Pattern

```rust
let pres = package.presentation_part()?;
let master = pres.slide_master_parts(&package).next()?;
let theme_part = master.theme_part(&package)?;
let theme: &Theme = theme_part.root(&package);
let clr_scheme = &theme.theme_elements.clr_scheme;
// theme.theme_elements.font_scheme
// theme.theme_elements.fmt_scheme
```

Rust types under `schemas_openxmlformats_org_drawingml_2006_main`:
`Theme`, `ThemeElements`, `ColorScheme`, `FontScheme`, `FormatScheme`.

---

## 8. Round-Trip Gotchas

1. **`<a:fmtScheme>` lists must each have exactly 3 children.** Renderers
   index by 0/1/2. Fewer entries cause index-out-of-range failures.

2. **`<a:overrideClrMapping>` requires all 12 slot attributes.** Missing
   attributes leave those slots undefined; behaviour is renderer-specific.

3. **`<a:masterClrMapping/>` is a self-closing tag with no attributes.** Do
   not add any attributes; the element's mere presence signals "inherit".

4. **The theme relationship lives on the slide master, not the presentation.**
   A theme relationship on `presentation.xml` is silently ignored by
   conforming readers.

5. **Multiple slide masters may reference different theme files.** ooxmlsdk
   handles each master independently; do not assume a single theme for the
   whole presentation.

---

## 9. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| PML-TH-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/drawingml/theme.pptx` | Full theme round-trip via slide master (covered in DrawingML spec) |
| PML-TH-02 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/pml/theme_colors.pptx` | `<a:overrideClrMapping>`: slide with `accent1`/`dk1`/`lt1` scheme-color fills and `<p:clrMapOvr><a:overrideClrMapping bg1="dk1" tx1="lt1" …>` swapping dark/light for that slide |
