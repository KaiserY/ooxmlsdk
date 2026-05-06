# PresentationML Slide Transitions — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §19.5 (slide transitions);
ISO/IEC 29500:2016 Part 1 §19.5; XSD in
`schemas/OfficeOpenXML-XMLSchema-Transitional/pml.xsd`.

---

## 1. Overview

A slide transition is the animated visual effect that plays when the
presentation advances from one slide to the next. Transition settings live in
the slide part itself (or in a layout or master that slides inherit from) as
the `<p:transition>` element.

```
Namespace URI (PresentationML): http://schemas.openxmlformats.org/presentationml/2006/main
Conventional prefix: p

Namespace URI (DrawingML): http://schemas.openxmlformats.org/drawingml/2006/main
Conventional prefix: a

Namespace URI (relationships): http://schemas.openxmlformats.org/officeDocument/2006/relationships
Conventional prefix: r
```

Schema file: `pml.xsd`.

---

## 2. `<p:transition>` (CT_SlideTransition)

### Placement

`<p:transition>` is a child of:

- `<p:sld>` (slide) — most common; overrides any layout or master setting
- `<p:sldLayout>` (slide layout) — applies to all slides using that layout
  unless the slide overrides
- `<p:sldMaster>` (slide master) — widest default scope

When `<p:transition>` is present, it must appear **after** `<p:clrMapOvr>`
in the parent element's child sequence. A slide with no `<p:transition>`
element has no animated transition — the advance is an instant cut, and click-
to-advance is the default behaviour.

### Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `spd` | ST_TransitionSpeed | `fast` | Transition speed |
| `dur` | xsd:unsignedInt | — | Duration in milliseconds (Office 2007 extension; use `spd` for broader compatibility) |
| `advClick` | xsd:boolean | true | Advance slide on mouse click |
| `advTm` | xsd:unsignedInt | — | Auto-advance after this many milliseconds; absent means no auto-advance |

#### ST_TransitionSpeed values

| Value | Typical duration |
|-------|-----------------|
| `slow` | Approximately 1 000 ms |
| `med` | Approximately 750 ms |
| `fast` | Approximately 500 ms (default) |

When `dur` is present it takes precedence over `spd` in Office 2010 and
later; older consumers ignore `dur` and use `spd`. For maximum
compatibility, emit `spd` and omit `dur` unless the exact millisecond
duration is needed.

### Child element sequence

| # | Element | Multiplicity | Notes |
|---|---------|-------------|-------|
| 1 | transition type choice | 0 or 1 | One of the type elements listed in §3 |
| 2 | `a:sndAc` | 0 or 1 | Sound action |
| 3 | `a:extLst` | 0 or 1 | Extension list |

When the transition type element is absent, the slide uses a cross-fade or
cut (application-defined) with the speed and advance settings from the
attributes.

---

## 3. Transition Type Elements

Each transition type element is a child of `<p:transition>`. At most one may
appear. All elements listed below are in the `p:` namespace. The schema
defines them as choices inside `CT_SlideTransition`; a file containing more
than one is invalid.

### 3.1 `<p:cut>` — instant cut

No animation; the new slide appears immediately. The optional `thruBlk`
attribute cuts through a single black frame before showing the new slide.

```xml
<p:transition spd="fast" advClick="1">
  <p:cut thruBlk="0"/>
</p:transition>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `thruBlk` | xsd:boolean | false | Flash through a black frame during the cut |

### 3.2 `<p:fade>` — cross-fade

The current slide fades out while the new slide fades in.

```xml
<p:transition spd="med" advClick="1">
  <p:fade thruBlk="0"/>
</p:transition>
```

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `thruBlk` | xsd:boolean | false | Fade through black instead of cross-fading |

### 3.3 `<p:push>` — push

The new slide pushes the current slide off the screen.

```xml
<p:transition spd="med">
  <p:push dir="l"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `l` `r` `u` `d` | Direction the new slide enters from |

### 3.4 `<p:wipe>` — wipe

The new slide is progressively revealed over the current slide by a moving
edge.

```xml
<p:transition spd="med">
  <p:wipe dir="r"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `l` `r` `u` `d` | Direction from which the wipe edge enters |

### 3.5 `<p:split>` — split wipe

Two edges move from the centre outward (or from the edges inward),
revealing or concealing the slide.

```xml
<p:transition spd="med">
  <p:split orient="horz" dir="out"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `orient` | yes | `horz` `vert` | Orientation of the split axis |
| `dir` | yes | `in` `out` | Whether the edges move inward or outward |

### 3.6 `<p:cover>` — cover

The new slide slides in over the current slide without pushing it.

```xml
<p:transition spd="fast">
  <p:cover dir="l"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `l` `r` `u` `d` `ld` `lu` `rd` `ru` | Direction from which the new slide enters |

### 3.7 `<p:pull>` — pull (uncover)

The current slide is pulled away, revealing the new slide underneath.

```xml
<p:transition spd="fast">
  <p:pull dir="l"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `l` `r` `u` `d` `ld` `lu` `rd` `ru` | Direction in which the old slide exits |

### 3.8 `<p:blinds>` — venetian blinds

Horizontal or vertical blinds open to reveal the new slide.

```xml
<p:transition spd="med">
  <p:blinds dir="horz"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `horz` `vert` | Orientation of the blind slats |

### 3.9 `<p:checker>` — checker wipe

A checkerboard pattern wipes the new slide in.

```xml
<p:transition spd="med">
  <p:checker dir="horz"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `horz` `vert` | Orientation of the checker wipe |

### 3.10 `<p:dissolve>` — random dissolve

Pixels of the new slide dissolve in randomly over the current slide. No
attributes.

```xml
<p:transition spd="med">
  <p:dissolve/>
</p:transition>
```

### 3.11 `<p:zoom>` — zoom

The new slide grows from the center (zoom in) or the current slide shrinks
away (zoom out).

```xml
<p:transition spd="med">
  <p:zoom dir="in"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `in` `out` | Whether the effect zooms in or out |

### 3.12 `<p:fly>` — fly in / out

An object or the entire slide flies in from outside the slide boundary.

```xml
<p:transition spd="fast">
  <p:fly dir="l"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `l` `r` `u` `d` `ld` `lu` `rd` `ru` | Direction from which the slide enters |

### 3.13 `<p:strips>` — diagonal strips

Diagonal strip wipes reveal the new slide.

```xml
<p:transition spd="med">
  <p:strips dir="ld"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `ld` `lu` `rd` `ru` | Diagonal direction of the strips |

### 3.14 `<p:wheel>` — wheel wipe

A spinning wheel wipe (clock wipe) reveals the new slide. The `spokes`
attribute controls how many wedge sectors make up the wheel.

```xml
<p:transition spd="med">
  <p:wheel spokes="4"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `spokes` | yes | `1` `2` `3` `4` `8` | Number of wheel spokes |

### 3.15 `<p:random>` — random transition

The application picks a random transition at play time. No attributes.

```xml
<p:transition>
  <p:random/>
</p:transition>
```

### 3.16 `<p:randomBar>` — random bars

Random horizontal or vertical bars build up to reveal the new slide.

```xml
<p:transition spd="med">
  <p:randomBar dir="horz"/>
</p:transition>
```

| Attribute | Required | Values | Description |
|-----------|----------|--------|-------------|
| `dir` | yes | `horz` `vert` | Orientation of the bars |

### 3.17 `<p:circle>` — circle wipe

A circle expands from the centre to reveal the new slide. No attributes.

```xml
<p:transition spd="med">
  <p:circle/>
</p:transition>
```

### 3.18 `<p:diamond>` — diamond wipe

A diamond expands from the centre. No attributes.

```xml
<p:transition spd="med">
  <p:diamond/>
</p:transition>
```

### 3.19 `<p:plus>` — plus wipe

A plus sign expands from the centre. No attributes.

```xml
<p:transition spd="med">
  <p:plus/>
</p:transition>
```

### 3.20 `<p:wedge>` — wedge wipe

Two diagonal lines sweep outward from the centre like clock hands. No
attributes.

```xml
<p:transition spd="med">
  <p:wedge/>
</p:transition>
```

### 3.21 `<p:newsflash>` — newsflash (rotate-zoom in)

The new slide rotates and zooms in from a small point at the centre. No
attributes.

```xml
<p:transition spd="fast">
  <p:newsflash/>
</p:transition>
```

---

## 4. `<a:sndAc>` — Sound Action

`<a:sndAc>` (schema type `CT_SoundAction`) is an optional child of
`<p:transition>` that specifies a sound to play or stop during the
transition.

### Choices

Exactly one of the following child elements must appear:

| Element | Description |
|---------|-------------|
| `<a:stSnd>` | Start playing a sound |
| `<a:endSnd/>` | Stop the currently playing sound |

### `<a:stSnd>` (CT_SoundActionType)

| Attribute | Type | Description |
|-----------|------|-------------|
| `loop` | xsd:boolean | Loop the sound repeatedly |

`<a:stSnd>` contains one `<a:snd>` child:

```xml
<a:sndAc>
  <a:stSnd loop="0">
    <a:snd r:embed="rId3" name="chime.wav"/>
  </a:stSnd>
</a:sndAc>
```

### `<a:snd>` (CT_EmbeddedWAVAudioFile) attributes

| Attribute | Notes |
|-----------|-------|
| `r:embed` | Relationship ID for an embedded sound file in the package |
| `r:link` | Relationship ID for a linked (external) sound file |
| `name` | Human-readable name of the sound |

Either `r:embed` or `r:link` should be present; `r:embed` is preferred
for self-contained packages.

---

## 5. Advance Interaction

The `advClick` and `advTm` attributes work together:

| `advClick` | `advTm` | Behaviour |
|------------|---------|-----------|
| `1` (true) | absent | Click to advance; no automatic advance |
| `1` (true) | present | Advance on click **or** after the timer, whichever comes first |
| `0` (false) | present | Advance only after the timer (kiosk / auto-run mode) |
| `0` (false) | absent | Slide never advances; effectively a dead end |

When `<p:transition>` is absent from a slide, the rendering application
applies its own default (typically click-to-advance, instant cut).

---

## 6. XML Examples

### Example 1 — Fade at medium speed, click-advance only

```xml
<p:transition spd="med" advClick="1">
  <p:fade/>
</p:transition>
```

No `advTm` means the slide waits indefinitely for a click.

### Example 2 — Push-right with 2 s auto-advance plus click

```xml
<p:transition spd="med" advClick="1" advTm="2000">
  <p:push dir="r"/>
</p:transition>
```

The slide advances when the user clicks **or** after 2 000 ms, whichever
occurs first. The new slide enters from the right side.

---

## 7. Round-Trip Gotchas

1. **`spd` vs. `dur` compatibility.** `dur` (millisecond duration) was
   introduced in Office 2007 as an extension attribute. Older consumers
   ignore it and fall back to `spd`. Emit both only when targeting
   modern consumers and you require an exact duration.

2. **Absent `<p:transition>` is not the same as `<p:cut/>`.** A missing
   element means the application uses its default; `<p:cut/>` is an
   explicit instant cut. Round-trip must preserve the distinction.

3. **`advTm="0"` is a valid value** meaning the slide advances immediately
   after the transition animation completes — not the same as an absent
   `advTm`.

4. **At most one transition type element per `<p:transition>`.** A file
   with two type elements is schema-invalid. Parsers that see multiple
   should treat all but the first as invalid.

5. **Direction strings are case-sensitive.** `"L"` is not the same as
   `"l"`. The schema defines these as restricted strings; preserve them
   verbatim.

6. **`<p:random>` picks the transition at render time**, not at parse
   time. The SDK stores the literal `<p:random/>` element and must not
   resolve it to a concrete transition type.

7. **Sound requires a relationship.** `r:embed` on `<a:snd>` must resolve
   to a media part in the package (typically `ppt/media/audio1.wav`).
   A missing relationship target is a broken reference, not a parse error.

8. **Layout and master `<p:transition>` elements are the lowest-priority
   defaults.** A slide-level `<p:transition>` always wins. When
   round-tripping, do not merge or inherit from layout/master elements.

---

## 8. Fixture Plan

| ID | File | Coverage |
|----|------|---------|
| PML-TR-01 | `test-data/pml/transition_fade.pptx` | `<p:fade/>` at `spd="med"`, click-advance only |
| PML-TR-02 | `test-data/pml/transition_push.pptx` | `<p:push dir="r"/>` with `advClick="1"` and `advTm="2000"` |
| PML-TR-03 | `test-data/pml/transition_none.pptx` | Slide with no `<p:transition>` element |
| PML-TR-04 | `test-data/pml/transition_kiosk.pptx` | `advClick="0"` and `advTm="5000"` (auto-run) |
| PML-TR-05 | `test-data/pml/transition_sound.pptx` | Transition with `<a:sndAc>` embedded sound |
| PML-TR-06 | `test-data/pml/transition_variety.pptx` | Multiple slides using `split`, `wheel`, `checker`, `blinds`, `dissolve` |
