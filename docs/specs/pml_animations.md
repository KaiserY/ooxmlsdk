# PresentationML Animations — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §19.5 (PresentationML
Animations); §19.5.85 (`p:timing`); §19.5.39 (`p:cTn`); §19.5.5
(`p:animEffect`); §19.5.71 (`p:set`); ISO/IEC 29500:2016 Part 1;
XSD `pml.xsd`.

---

## 1. Overview

Animations are declared in a `<p:timing>` block appended to each `<p:sld>`
after `<p:clrMapOvr>`. The timing tree is a nested hierarchy of parallel and
sequential time containers. Each leaf effect node targets a shape on the slide.

---

## 2. `<p:timing>` Structure

```xml
<p:timing>
  <p:tnLst>
    <p:par>
      <!-- root time node -->
      <p:cTn id="1" dur="indefin" restart="whenNotActive" nodeType="tmRoot">
        <p:childTnLst>
          <p:seq concurrent="1" nextAc="seek">
            <!-- main sequence: advances on click -->
            <p:cTn id="2" dur="indefin" nodeType="mainSeq">
              <p:childTnLst>
                <!-- one <p:par> per click group -->
                <p:par>
                  <p:cTn id="3" fill="hold">
                    <p:stCondLst>
                      <p:cond delay="indefin"/>
                    </p:stCondLst>
                    <p:childTnLst>
                      <p:par>
                        <p:cTn id="4" fill="hold">
                          <p:stCondLst>
                            <p:cond delay="0"/>
                          </p:stCondLst>
                          <p:childTnLst>
                            <!-- effect nodes here -->
                          </p:childTnLst>
                        </p:cTn>
                      </p:par>
                    </p:childTnLst>
                  </p:cTn>
                </p:par>
              </p:childTnLst>
            </p:cTn>
            <p:prevCondLst>
              <p:cond evt="onPrev" delay="0">
                <p:tn><p:tnLst><p:par>
                  <p:cTn id="7" dur="1"/>
                </p:par></p:tnLst></p:tn>
              </p:cond>
            </p:prevCondLst>
            <p:nextCondLst>
              <p:cond evt="onNext" delay="0">
                <p:tn><p:tnLst><p:par>
                  <p:cTn id="8" dur="1"/>
                </p:par></p:tnLst></p:tn>
              </p:cond>
            </p:nextCondLst>
          </p:seq>
        </p:childTnLst>
      </p:cTn>
    </p:par>
  </p:tnLst>
  <p:bldLst>
    <p:bldP spid="2" grpId="0" uiExpand="1" build="allAtOnce"/>
  </p:bldLst>
</p:timing>
```

### Time-node hierarchy

| Level | Element | Role |
|-------|---------|------|
| 1 | `<p:par><p:cTn nodeType="tmRoot">` | Root; wraps the entire tree |
| 2 | `<p:seq nodeType="mainSeq">` | Main sequence; click-to-advance |
| 3 | `<p:par>` outer click group | Groups one click's worth of effects |
| 4 | `<p:par>` inner effect group | Groups simultaneous effects for that click |
| 5 | Effect node | `<p:animEffect>`, `<p:set>`, etc. |

---

## 3. `<p:cTn>` — Common Time Node

| Attribute | Meaning |
|-----------|---------|
| `id` | Unique integer within the timing tree; must be sequential from 1 |
| `dur` | Duration in ms, or `"indefin"` |
| `restart` | `"always"` / `"whenNotActive"` / `"never"` |
| `fill` | `"remove"` (default) / `"freeze"` / `"hold"` / `"auto"` |
| `nodeType` | `tmRoot` / `mainSeq` / `interactiveSeq` / `clickEffect` / `withEffect` / `afterEffect` |
| `grpId` | Integer build group id (matches `<p:bldP grpId>`) |

`<p:childTnLst>` holds child nodes. `<p:stCondLst>` holds start-condition
triggers.

---

## 4. Start Conditions (`<p:stCondLst>`)

```xml
<p:stCondLst>
  <p:cond delay="indefin"/>   <!-- wait for a mouse click -->
  <p:cond delay="0"/>         <!-- start immediately after previous -->
  <p:cond delay="500"/>       <!-- start 500 ms after previous -->
</p:stCondLst>
```

| `delay` value | Trigger |
|---------------|---------|
| `"indefin"` | Advance on click |
| integer (ms) | Auto-advance N ms after the previous node ends |

---

## 5. Effect Elements

### `<p:animEffect>` — Preset filter transition

```xml
<p:animEffect transition="in" filter="fade">
  <p:cBhvr>
    <p:cTn id="5" dur="500" fill="hold"/>
    <p:tgtEl><p:spTgt spid="2"/></p:tgtEl>
  </p:cBhvr>
</p:animEffect>
```

| Attribute | Values |
|-----------|--------|
| `transition` | `"in"` (entrance) / `"out"` (exit) |
| `filter` | `"fade"`, `"blinds"`, `"box"`, `"checkerboard"`, `"circle"`, `"diamond"`, `"dissolve"`, `"plus"`, `"random"`, `"randomBar"`, `"slide"`, `"strips"`, `"wedge"`, `"wheel"`, `"wipe"`, `"zoom"` |

`<p:tgtEl><p:spTgt spid>` targets a shape by its `<p:cNvPr id>`.

### `<p:set>` — Discrete property set

```xml
<p:set>
  <p:cBhvr>
    <p:cTn id="6" dur="1" fill="hold"/>
    <p:tgtEl><p:spTgt spid="2"/></p:tgtEl>
    <p:attrNameLst><p:attrName>style.visibility</p:attrName></p:attrNameLst>
  </p:cBhvr>
  <p:to><p:strVal val="visible"/></p:to>
</p:set>
```

Entrance animations pair a `<p:set>` (set `style.visibility = visible` at
t=0) with an `<p:animEffect>` (visual transition). Without the `<p:set>`,
the shape remains hidden until the transition runs, producing a flash.

### Other effect elements

| Element | Purpose |
|---------|---------|
| `<p:animMotion>` | Path-based motion (`<p:path>`) |
| `<p:animClr>` | Colour animation |
| `<p:animScale>` | Scale transform |
| `<p:animRot>` | Rotation animation |
| `<p:audio>` | Play audio at a time offset |

---

## 6. `<p:bldLst>` — Build List

```xml
<p:bldLst>
  <p:bldP spid="2" grpId="0" uiExpand="1" build="allAtOnce"/>
</p:bldLst>
```

`<p:bldP spid>` declares that shape `spid` participates in animations.
`build` values: `"allAtOnce"`, `"p"` (paragraph-by-paragraph),
`"custAnimGrps"`. `grpId` correlates to `<p:cTn grpId>` in the timing tree.

---

## 7. `<p:seq>` — Advance / Back Conditions

`<p:prevCondLst>` and `<p:nextCondLst>` are children of `<p:seq>`:

```xml
<p:prevCondLst>
  <p:cond evt="onPrev" delay="0">
    <p:tn val="3"/>   <!-- val references the click-group par node id -->
  </p:cond>
</p:prevCondLst>
<p:nextCondLst>
  <p:cond evt="onNext" delay="0">
    <p:tn val="3"/>
  </p:cond>
</p:nextCondLst>
```

These define how click-through and back-step advance the main sequence.
`evt="onNext"` fires when the presenter clicks forward; `evt="onPrev"` fires
on back-step. `<p:tn val>` references the `<p:cTn id>` of the click-group par
that should be targeted when the event fires.

---

## 8. SDK Access Pattern

```rust
let slide = pres.slide_parts(&package).next()?;
let timing = slide.root(&package).timing.as_ref()?;
let root_par = &timing.tn_lst.par[0];
// root_par.c_tn.child_tn_lst.seq[0] — main sequence
```

Rust types under
`schemas_openxmlformats_org_presentationml_2006_main`:
`Timing`, `TimeNodeList`, `ParallelTimeNode`, `SequenceTimeNode`,
`CommonTimeNode`, `AnimateEffect`, `SetBehavior`, `BuildParagraph`.

---

## 9. Round-Trip Gotchas

1. **`<p:cTn id>` must be unique and sequential from 1.** Duplicate or
   out-of-order ids corrupt playback; some players refuse to open the file.

2. **`<p:cTn fill="hold">` is required on all ancestor nodes of entrance
   animations.** The default `"remove"` resets the shape to its pre-animation
   state (hidden) after the effect ends, making it disappear again.

3. **`<p:bldLst><p:bldP spid>` must match `<p:spTgt spid>`.** Mismatched ids
   produce an orphaned build-panel entry that targets nothing.

4. **`<p:seq>` on the `mainSeq` requires `<p:prevCondLst>` and
   `<p:nextCondLst>`.** Omitting them disables backward stepping in
   presentation mode.

5. **`transition="in"` and `"out"` are opposite effects.** `filter="fade"` +
   `transition="in"` = shape fades in; `transition="out"` = shape fades out.

6. **`<p:stCondLst>` with `delay="indefin"` on the outer click group and
   `delay="0"` on the inner group are both required.** The outer condition
   arms the click trigger; the inner condition fires the effect immediately
   when the click fires.

---

## 10. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| PML-ANIM-01 | `../ooxmlsdk-test-suite/fixtures/ooxmlsdk-test/pml/slide_animation.pptx` | Single slide with text box (id=2); full `<p:timing>` hierarchy (tmRoot→mainSeq→click→effect); `<p:set>` visibility + `<p:animEffect filter="fade" transition="in">`; `<p:bldLst><p:bldP>`; `<p:prevCondLst>`/`<p:nextCondLst>` on `<p:seq>` |
