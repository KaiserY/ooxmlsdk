# Particle-Driven Schema Generation

## Status

This document defines the target architecture for the schema-generation
refactor that replaces the current composite-type-driven content-model logic
 with a particle-driven pipeline.

This is a rewrite plan, not a description of the legacy implementation.

During the refactor, this document must be kept current. Any meaningful change
to the architecture, rollout boundary, normalization behavior, or upstream
alignment rules should be reflected here as part of the same development
sequence.

## Goal

The generator must use `Particle` as the single source of truth for schema
content models.

For this refactor, `data/` remains the only source of truth. Upstream
generated output, upstream generator source, and upstream particle test data
are reference inputs only. They may calibrate or explain behavior, but they do
not override local `data/` as the generator input authority.

The long-term goal is to completely replace the current logic that relies on
special handling for shapes such as:

- `OneSequence`
- `OneChoice`
- `OneAll`
- other composite-type-specific branches and fallback paths

The new system must generate IR JSON from particle trees directly. The legacy
`CompositeType` value may still be used during migration to decide which types
opt into the new pipeline first, but it must not participate in content-model
generation decisions.

The main target of this rewrite is `Members` generation. The IR shell should
remain stable.

During migration, the IR may temporarily carry a second member-generation lane
for particle-driven output. This transitional lane exists to keep the old and
new generators separate while preserving a stable final Rust output contract.

## Motivation

The current pipeline is too indirect:

1. `Particle` data exists in `data/`
2. the generator lowers that data into a legacy child-tree model
3. later stages reconstruct structure from that lossy intermediate form
4. more special cases are added to recover shape-specific behavior

This has several problems:

- structure is no longer driven by the original particle tree
- flattening, grouping, naming, and compatibility behavior are tightly coupled
- special handling is spread across multiple layers
- the system is difficult to reason about and easy to regress

The refactor must restore a single main line:

`Particle -> normalized particle model -> IR JSON`

In practice, this means rewriting how `Members` are produced while keeping the
overall Rust-oriented IR contract intact.

Within the IR type shell, the new lane should only replace the member lane.
`ParticleMembers` is the rewrite target. Other existing `TypeDecl`-level logic
and fields should continue to be carried correctly unless and until they are
explicitly redesigned. In particular, the new lane must not accidentally drop
existing behavior for fields such as:

- `base_rust_name`
- `xml_content`
- `support`
- `is_abstract`
- `kind`
- `element_kind`
- `version`
- `xml_qname`

In short, the new logic rewrites member generation, not the rest of the type
shell.

## Design Principles

- `Particle` is the only semantic source for content-model structure.
- Legacy composite-type labels are not semantic inputs.
- The pipeline must be layered so structural normalization is explicit.
- Normalization passes must be isolated and testable.
- Rust-oriented shaping such as flattening is allowed, but must be implemented
  as explicit particle passes, not as ad hoc content-model branches.
- The first stage should prefer faithful modeling over aggressive optimization.

## Scope

### Final scope

The final system must replace the current complex content-model generation
logic for all relevant schema types.

### First-stage scope

The first stage uses the new `particle_builder` only for the current
`OneSequence` range.

This is a migration boundary only. It is not part of the new model.

At the IR-builder entry point, the first-wave `if/else` split should use this
same migration boundary directly:

- if the schema type is currently in the rollout set for `OneSequence`, route
  to `particle_builder`
- otherwise, route to the legacy builder

This entry split is a rollout mechanism only. It must not leak into particle
structure decisions inside `particle_builder`.

Within that first-stage scope:

- generation decisions must come from particle structure only
- no new logic may branch on `OneSequence`, `OneChoice`, or `OneAll`
- `CompositeType` is only allowed as an opt-in switch for rollout
- the new particle-driven member lane may coexist with the legacy member lane
  in IR during migration

## Upstream Alignment

The refactor should stay aligned with upstream Open XML SDK particle data and
behavior:

- `data/schemas/*` particle data is the local primary structural input
- `../Open-XML-SDK/generated/DocumentFormat.OpenXml/DocumentFormat.OpenXml.Generator/DocumentFormat.OpenXml.Generator.OpenXmlGenerator/`
  is the upstream generated output reference for schema and part shape
- `../Open-XML-SDK/gen/DocumentFormat.OpenXml.Generator.Models/Models/Particle.cs`
  is the upstream generator model reference
- `../Open-XML-SDK/test/DocumentFormat.OpenXml.Packaging.Tests/data/Particles.json`
  is the upstream particle-shape calibration reference

The intent is not to reproduce upstream C# code literally. The intent is to
preserve particle semantics while expressing them idiomatically in Rust, while
keeping local `data/` as the authoritative input.

## Required Reference Inputs

Every implementation step in this refactor should be checked against these
three repository-local reference inputs:

1. `data/`
2. `../Open-XML-SDK/generated/DocumentFormat.OpenXml/DocumentFormat.OpenXml.Generator/DocumentFormat.OpenXml.Generator.OpenXmlGenerator/`
3. `../Open-XML-SDK/test/DocumentFormat.OpenXml.Packaging.Tests/data/Particles.json`

They serve different purposes:

- `data/` is the local source input that the Rust generator actually consumes
- the upstream `generated/.../OpenXmlGenerator/` directory is a generated-output
  reference that helps calibrate expected schema and part shape; it is not the
  source of truth for Rust generation decisions
- `Particles.json` is the calibration reference for effective particle shape
  after upstream version filtering and structural expansion

The normal development rule is:

- start from local `data/`
- compare structure and output shape against upstream generated output when
  behavior is unclear
- use `Particles.json` to validate particle semantics and version-sensitive
  structure

XSD-backed calibration that already exists in the local generator may continue
to be reused in the new lane when it serves as canonicalization of local input,
for example:

- child/type qname normalization
- known base-class correction
- attribute/simple-type calibration

This is still part of the local `data/` consumption pipeline. It does not make
XSDs a new source of truth.

However, this calibration must not be allowed to retake control of particle
structure generation. In particular:

- it may normalize names and type-facing metadata
- it may not replace particle structure with XSD-derived structure
- it may not reintroduce legacy composite-type strategy decisions

## Architecture

The new pipeline has three layers.

### 1. Particle lowering

This layer converts `OpenXmlSchemaType.particle` into a new internal particle
model.

Responsibilities:

- read the particle tree directly from `data/schemas/*`
- preserve structure and ordering
- preserve cardinality
- preserve version information
- preserve `require_filter`
- resolve leaf element metadata needed for generation

Non-responsibilities:

- no flattening
- no helper-struct shaping
- no choice collapsing
- no legacy content-model classification

### 2. Particle normalization

This layer transforms the internal particle model through explicit,
deterministic passes.

Normalization exists so Rust-oriented shaping can be introduced without
polluting the lowering layer or the IR generation layer.

Each normalization step must be:

- explicit
- local in responsibility
- testable in isolation
- removable without destabilizing the whole pipeline

### 3. IR generation

This layer generates IR JSON from the normalized particle model.

The long-term output target is the member lane used by Rust code generation.
During migration, the new lane should write that output into
`ParticleMembers`, together with any helper structs and choice enums required
to keep Rust code generation direct and stable.

This layer must not re-infer structure from legacy child trees.

During migration, this layer may emit particle-driven members into a dedicated
`ParticleMembers` field that sits beside legacy `Members`. In that mode:

- types handled by the new pipeline should populate `ParticleMembers`
- their legacy `Members` payload should be empty
- types still handled by the legacy pipeline should populate `Members`
- their `ParticleMembers` payload should be empty

The legacy `content_model` field belongs to the old lane only. The new
particle-driven lane must not produce or depend on `content_model`, because it
encodes legacy strategy labels such as `OneSequenceFlatten` and
`OneSequenceStructured` rather than source-backed particle structure.

The legacy `content_structure` field also belongs to the old lane only for the
purposes of this migration. The new particle-driven lane must not reconstruct
or depend on legacy child-lowered `content_structure` data in phase 1.

Legacy schema-extension hooks that target old choice layouts, such as
`parent_choice_has_any_in`, also belong to the old lane only during phase 1.
The new particle-driven lane must not depend on those legacy choice-targeted
patches, because they are tied to old member shaping and can silently pull the
new pipeline back toward legacy flattening behavior.

`ParticleMembers` should be understood as a Rust-oriented member tree whose
structural skeleton comes from `Particle`, while member-facing details are
filled from `Children` and `AdditionalElements`.

Where practical, `ParticleMembers` should reuse the existing IR member
vocabulary and concepts so that the final Rust surface remains stable. This
includes existing ideas such as fields, variants, choice enums, helper
structs, wire kinds, cardinality, and versioned members. The rewrite target is
the generation logic, not a gratuitous redesign of member concepts.

In phase 1, helper and choice naming should stay simple and top-level. If
there is only one choice field or helper sequence in the handled root type,
use unsuffixed names such as `choice`, `TypeChoice`, and `TypeSequence`. If
there are multiple siblings of the same kind, use root-scoped numbered names
such as `choice1`, `choice2`, `TypeChoice1`, `TypeChoice2`, `TypeSequence1`,
and `TypeSequence2`. Do not recursively encode nested owner paths into names.

Because version data already exists in `data/` and upstream references, the
new lane must preserve it as normal functional behavior rather than treat it
as an optional enhancement. `ParticleMembers` must carry the effective version
information needed by downstream Rust code generation in the same way that the
legacy lane carries versioned member behavior today.

The final Rust code generation contract must remain unified. Downstream code
generation should produce one consistent Rust type and macro surface regardless
of whether a type came from legacy `Members` plus legacy `content_model`, or
from `ParticleMembers` alone.

The new lane must join the pipeline upstream, at IR-building time, not
downstream after legacy member shaping. A handled schema type should branch to
`particle_builder` before legacy content-model classification, before legacy
member shaping, and before legacy member post-processing rewrites.

For the migration, the new and legacy lanes should be kept structurally
separate. Prefer an explicit `if/else` branch at the IR-builder entry point
that sends a handled type either to the legacy builder function or to
`particle_builder`. Do not route both lanes through a partially shared
content-model-building pipeline.

## First-Stage Normalization Rules

The first stage must be conservative.

Allowed:

- remove `Group` wrapper nodes
- propagate `Group` constraints to the wrapped child

Not yet allowed:

- flattening
- collapsing nested sequences for presentation only
- merging safe choice leaves
- output-shape tuning to resemble current generated Rust

The first stage is meant to establish a faithful particle-driven main line.

Implementation should stay grounded in the actual shapes present in `data/`.
Do not complicate the first stage for hypothetical particle forms that are not
currently present in the checked-in source data. In particular, if a feared
edge case such as multiple wildcard `Any` slots within a single handled type
does not occur in the current `data/` rollout set, it should not drive first-
stage design complexity.

## Group Handling

`Group` should not survive into the normalized first-stage model as a distinct
runtime-facing structural node.

However, removing the wrapper must not discard its semantics.

When lowering or normalizing a `Group`, the implementation must preserve and
propagate:

- `minOccurs`
- `maxOccurs`
- effective version
- `require_filter`

In practice, first-stage normalization should treat `Group` as a transparent
wrapper around a single structural child while preserving constraint flow.

## Internal Particle Model

The new internal particle model should preserve the information required to
generate IR without depending on legacy composite-type branches.

At minimum it should represent:

- particle kind:
  - `Element`
  - `Any`
  - `Choice`
  - `Sequence`
  - `All`
  - temporary `Group` before normalization
- element QName when applicable
- cardinality
- effective version or raw version data
- `require_filter`
- children

It should also carry the metadata needed to build good Rust-facing members for
leaf elements, such as:

- property name
- property comments or docs
- resolved leaf kind where required by code generation

## Relationship To Existing Child Metadata

The new pipeline must not use the legacy child tree as a structural source.

However, schema child metadata is still useful as an auxiliary lookup source
for element leaves, especially for:

- property naming
- property comments
- other element-local metadata that is not encoded in particle nodes

`AdditionalElements` are also outside the particle tree and should remain
available as auxiliary metadata for known-child and compatibility handling.

This means:

- `Particle` drives structure
- `Children` may assist member-level metadata resolution
- non-particle metadata such as `AdditionalElements` may assist member-level
  metadata and known-child modeling where needed

This is acceptable because it does not reintroduce composite-type-driven
structure.

In short:

- `Particle` describes structure
- `Children` and `AdditionalElements` describe member-facing details such as
  naming, type-facing metadata, comments, docs, and known child-element
  information

The refactor should preserve all three inputs, but only `Particle` may control
content-model structure generation.

## IR Generation Rules

The new IR generation path should follow particle structure directly.

The IR must remain a Rust-oriented code generation IR. It is not being
redefined into a generic schema archive or a purely analytical intermediate
format.

The existing high-level role of IR JSON should stay the same:

- it should still drive direct Rust code generation
- it should still carry the information needed by Rust structs, enums, helper
  structs, and macro-oriented generation
- downstream codegen should not need to reconstruct complex schema structure
  from scratch

The overall IR shape should remain stable. This refactor may change the
particle-derived content encoded inside the IR, but the main intended change
surface is `Members`.

`Members` should also remain broadly recognizable in shape. The rewrite may
adjust member contents and supporting fields where needed, but it should not
gratuitously redesign the surrounding IR structure.

For migration, it is acceptable to introduce `ParticleMembers` as a parallel
field beside `Members`, as long as this remains a transitional compatibility
mechanism and not a second long-term output contract.

`content_model` is not part of the new contract. It remains legacy-only
compatibility data for the old generator path and should be removed with that
path rather than reproduced by `particle_builder`.

Examples:

- an element particle produces a child field or leaf payload
- a choice particle produces a choice-oriented IR construct
- a sequence particle produces a sequence-oriented IR construct
- an any particle produces an unknown/raw XML payload construct

Optionality, repetition, ordering, and version flow must come from the
particle model, not from legacy post-processing heuristics.

This includes choice cardinality. The new lane must not preserve the legacy
habit of modeling choice fields as `Option` by default when the particle
structure says the choice is required. If a choice is structurally required in
the particle model, the generated Rust field should also be structurally
required.

Because the Rust surface still derives and uses `Default` in many places,
particle-driven `SdkChoice` enums should support an explicit `impl Default`
when needed by required choice fields. That defaulting behavior is a codegen
compatibility requirement, not a reason to reintroduce fake optional choice
fields. The default selection rule should be deterministic and driven by the
particle-defined choice order rather than by legacy optional wrapping.

That `impl Default` support should be implemented in the `SdkChoice` macro
layer, not emitted ad hoc into generated schema modules. The generated schema
surface should keep using the derive/macro contract rather than carrying
per-type manual `Default` implementations in `schemas/*`.

## Flattening

Flattening is an explicit long-term goal, but it is not part of the first
stage.

The legacy generator already demonstrates that some sequence structures can be
safely merged into more idiomatic Rust forms. The problem is not the existence
of flattening. The problem is that flattening is currently tangled with other
special-case logic and has known bugs.

In the new architecture, flattening must be implemented as one or more
dedicated normalization passes after the faithful particle model is available.

This allows Rust-oriented shaping without compromising the semantic main line.

Examples of future passes may include:

- flattening safe sequence wrappers
- collapsing redundant helper-only sequence layers
- merging unambiguous choice leaves

Those passes are future work and must not be mixed into the first-stage
rewrite.

## Migration Strategy

The migration should proceed in controlled phases.

### Phase 1

- introduce the new internal particle model
- introduce `particle_builder`
- route the current `OneSequence` range through the new path
- branch to the new path at IR-builder entry for handled types, so
  `ParticleMembers` are produced directly from `SchemaType.particle`
- keep the legacy lane and the particle lane in separate builder functions,
  with an explicit entry-time branch between them
- keep first-stage normalization limited to `Group` removal and constraint
  propagation
- generate particle-driven members from normalized particles directly while
  keeping the IR shell stable
- allow `ParticleMembers` to coexist beside legacy `Members` as a migration
  boundary, with only one populated for a given handled type
- keep `content_model` on the legacy lane only; do not introduce a particle
  equivalent

### Phase 2

- validate generated diffs
- calibrate against `schemas_bak`, upstream particle evidence, and repository
  tests
- add explicit normalization passes such as flattening where justified

### Phase 3

- expand the new pipeline beyond the current `OneSequence` rollout range
- remove old content-model branches once coverage is sufficient

## Non-Goals

The first stage does not aim to:

- preserve byte-for-byte identical generated output
- preserve every legacy helper type shape
- keep legacy branching structure alive inside the new path
- optimize for the smallest possible diff at the expense of architecture
- redesign the overall IR JSON format

Some generated schema types are expected to change. That is part of the
refactor goal.

## Legacy Tests

Legacy tests must not be allowed to steer the new design back toward old
content-model classification or old intermediate shapes.

Tests should be handled as follows:

- keep tests that validate final Rust types, macros, runtime behavior,
  serialization, deserialization, and source-backed semantic output
- rewrite tests that still express valid required capability, but currently
  assert legacy intermediate forms
- remove tests whose only purpose is to validate legacy strategy labels or
  legacy intermediate structures such as old `content_model` classifications
  or old child-lowered `content_structure` shapes

In short, legacy tests may protect final behavior, but they must not constrain
new particle-driven structural decisions.

## Acceptance Criteria

The refactor is on the right track when the following become true:

- the new path builds IR from particle data directly
- handled types enter the new path before legacy content-model classification
  and legacy member-shaping rewrites
- the migration uses a clear lane split at builder entry rather than a shared
  partially migrated content-model pipeline
- structural decisions no longer depend on `OneSequence`, `OneChoice`, or
  `OneAll`
- `Group` wrappers are removed through explicit normalization
- first-stage `OneSequence` rollout does not depend on the legacy child-tree
  reconstruction logic
- the main visible IR change surface is member generation, while the IR shell
  remains stable
- if `ParticleMembers` is present during migration, Rust code generation still
  converges to the same final type and macro surface as legacy `Members`
- the new lane does not reproduce legacy `content_model` labels
- future flattening can be added as an isolated pass instead of another
  cross-cutting special case
