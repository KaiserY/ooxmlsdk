# Particle Modeling Audit

This document tracks known and suspected bugs in the Rust schema generation
model around upstream Open XML SDK particles.

The goal is not to rebuild the upstream particle validator. The Rust model
should remain idiomatic and parse-oriented, but it must preserve enough
particle semantics to parse and serialize all upstream-supported documents
without relying on a separate particle verification step.

## Reference Model

Primary references:

- Local input: `data/schemas/*.json`
- Upstream generated shape:
  `../Open-XML-SDK/generated/DocumentFormat.OpenXml/DocumentFormat.OpenXml.Generator/DocumentFormat.OpenXml.Generator.OpenXmlGenerator/`
- Upstream particle calibration:
  `../Open-XML-SDK/test/DocumentFormat.OpenXml.Packaging.Tests/data/Particles.json`
- Upstream model/writer:
  `../Open-XML-SDK/gen/DocumentFormat.OpenXml.Generator.Models/Models/Particle.cs`
  and
  `../Open-XML-SDK/gen/DocumentFormat.OpenXml.Generator.Models/Generators/Elements/ParticleWriterExtensions.cs`

Reference data roles:

- `data/schemas/*.json` is the generator input for this repository. Its
  `Types[].Particle` tree is the source data that Rust generation consumes.
- `../Open-XML-SDK/generated/` is upstream generated output. Its
  `builder.Particle = new CompositeParticle.Builder(...)` blocks show how
  upstream lowers schema particles into runtime validator graphs. Use this to
  calibrate shape, occurrence, version, and branch ordering, but do not copy C#
  implementation structure blindly.
- `../Open-XML-SDK/test/DocumentFormat.OpenXml.Packaging.Tests/data/Particles.json`
  is upstream test calibration data. It serializes effective particle shapes by
  SDK class and version set, after upstream filtering/expansion. Use it to
  compare tree shape and occurrence behavior, especially when local `data/`
  and generated C# appear to differ.

Comparison policy:

- Start with local `data/schemas/*.json`; fixes should explain how the local
  particle was interpreted.
- Check upstream generated output for the same class to see the effective
  `CompositeParticle` tree and occurrence values.
- Check `Particles.json` when version-specific shape, `Group` expansion, or
  occurrence semantics are unclear.
- Treat disagreements between our generated Rust shape and upstream particle
  semantics as suspected generator bugs unless a deliberate Rust parse-model
  simplification is documented and proven equivalent.
- Do not use upstream generated output or `Particles.json` to replace local
  input data. They are reference and calibration data, not new generator input
  sources.

Upstream `Particle` is a recursive tree. Each node can carry:

- `Kind`: `Element`, `Any`, `Choice`, `Sequence`, `All`, or `Group`
- `Occurs`: min/max cardinality, defaulting to `1..1`
- `InitialVersion`
- `RequireFilter`
- nested `Items`

The upstream generated C# uses this tree as a validator-oriented
`CompositeParticle` graph. The Rust generator does not need to mirror that graph
literally, but any Rust-side flattening must preserve the parse-visible effects
of cardinality, versioning, and ordered choices.

## Rust-Side Principle

The Rust schema model is the public parsing model. It should favor:

- fields for stable single children
- `Option<T>` for absent-or-present children
- `Vec<T>` for repeatable children
- choice enums for ordered heterogeneous child streams
- helper structs only when a sequence branch needs to preserve grouped content

The model is allowed to flatten upstream `Sequence`, `Choice`, and `Group`
wrappers when the wrapper has no parse-visible semantics of its own.

The model must not flatten when doing so changes whether a child is optional,
repeatable, version-gated, ordered relative to siblings, or part of a grouped
branch.

## Safe Flattening Rule

A wrapper is safe to flatten only when all of these are true:

- The wrapper has no element name.
- The wrapper has no user-facing property name or comments.
- The wrapper has no effective `optional` or `repeated` cardinality.
- The wrapper has no effective non-default version gate.
- The wrapper does not separate ordered siblings that must stay in one branch.
- The wrapper contains only children whose Rust names and element qnames remain
  unambiguous after flattening.

If any of these are false, flattening should either be blocked or explicitly
carry the lost state into the generated field/variant/helper struct.

## Known Problems

### 1. Sequence Variant Repetition Is Dropped

Location:

- `crates/ooxmlsdk-build/src/sdk_code/helpers.rs`
- `structure_sequence_variant_child`

Problem:

- The function computes `let repeated = parent_repeated;`.
- It ignores `child.repeated`.
- A repeated child inside a structured sequence variant can therefore become a
  single field or `Option<T>` instead of `Vec<T>`.

Why this is a bug:

- Upstream particle repetition belongs to the node carrying the occurrence.
- If a sequence branch is preserved as a Rust helper struct, the helper fields
  still need the leaf occurrence semantics.

Expected fix:

- Propagate repetition as `parent_repeated || child.repeated`.
- Add a generator unit test with a sequence variant containing a repeated leaf.

### 2. Single-Child Choice Collapse Ignores Wrapper Occurs

Location:

- `crates/ooxmlsdk-build/src/sdk_code/codegen_ir_builder.rs`
- `collapse_single_child_choice_wrapper`

Problem:

- The function collapses generic single-child choice wrappers without checking
  `optional` or `repeated`.
- It preserves only version information.
- If the collapsed choice wrapper carried `0..1` or `0..unbounded`, that
  occurrence can be lost or applied to the wrong generated member.

Why this is a bug:

- A single-child `Choice` is still a particle node with cardinality.
- Collapsing it is only semantics-preserving when the wrapper is effectively
  `1..1`.

Expected fix:

- Collapse only when the outer wrapper and all collapsed inner wrappers have no
  `optional` or `repeated` cardinality.
- If collapse is still desired for non-`1..1` wrappers, carry cardinality
  explicitly into the generated variant or field.

### 3. One-Sequence Flattening Allows Occurs-Carrying Wrappers

Location:

- `crates/ooxmlsdk-build/src/sdk_code/helpers.rs`
- `can_flatten_one_sequence_child`
- `flatten_one_sequence_child`

Problem:

- `can_flatten_one_sequence_child` allows nested `Sequence` wrappers to be
  flattened based only on child shape.
- The later flattening pass propagates `optional` and `repeated`, but the
  decision to flatten does not distinguish harmless wrappers from wrappers that
  should remain as grouped structure.

Why this is risky:

- Propagating occurrence to every descendant is correct for some simple
  wrapper shapes, but not for all grouped branches.
- A wrapper with its own occurrence can mean "repeat this branch", not "repeat
  every leaf independently".

Expected fix:

- Tighten flatten eligibility so occurrence-carrying wrappers are not flattened
  unless the generated shape is proven equivalent.
- Prefer preserving such wrappers as helper structs or nested choice variants.

### 4. Structured One-Sequence Choice Variants Lose Wrapper Cardinality

Location:

- `crates/ooxmlsdk-build/src/sdk_code/helpers.rs`
- `structure_one_sequence_choice_variant`

Problem:

- The function recurses through nested `Choice` nodes and single-child
  `Sequence` nodes while carrying only `initial_version`.
- It does not carry `optional` or `repeated` from those wrappers into the
  resulting structured variant.

Why this is a bug:

- Wrapper cardinality is part of the parse model.
- Dropping it can produce required fields where upstream permits absence, or
  single fields where upstream permits repetition.

Expected fix:

- Either block flattening through occurrence-carrying wrappers, or thread
  `optional` and `repeated` through the recursion and apply them to the
  resulting leaf/sequence particles.

### 5. Choice Field Cardinality Is Influenced By Variant Cardinality

Location:

- `crates/ooxmlsdk-build/src/sdk_code/codegen_ir_builder.rs`
- `choice_field_cardinality`

Problem:

- The choice field becomes `Vec<Choice>` if any variant is repeated.
- It becomes `Option<Choice>` if any variant is optional.
- This can conflate choice node occurrence with branch-internal occurrence.

Why this is risky:

- The choice field cardinality should primarily reflect the choice particle's
  own occurrence.
- Variant cardinality can require a helper branch shape rather than changing
  the whole choice stream.

Expected fix:

- Do not change this broadly without a failing fixture or generated-diff
  evidence.
- Audit remaining round-trip failures first.
- Where wrong, preserve repeated/optional branch structure instead of promoting
  variant cardinality to the outer choice field.

### 6. Anonymous Choice Flattening Is Too Local

Location:

- `crates/ooxmlsdk-build/src/sdk_data/schemas.rs`
- `flatten_anonymous_choice_children`
- `collect_flattenable_choice_leafs`

Problem:

- Anonymous choice wrappers are flattened when the immediate wrapper and leaf
  nodes look required and unique.
- The check does not account for all ancestor particle semantics that may have
  already been folded into the intermediate child tree.

Why this is risky:

- By this stage, the model has already converted particle occurrence into
  `optional` and `repeated` flags.
- Flattening based only on the local child tree can erase evidence that a
  wrapper existed to preserve branch grouping.

Expected fix:

- Keep the current flattening for known-safe required wrappers.
- Add tests for wrappers with parent occurrence and nested required leaves.
- If a wrapper has any effective cardinality/version state after lowering,
  treat it as not flattenable.

### 7. Sequence Collapse Does Not Require Harmless Wrapper State

Location:

- `crates/ooxmlsdk-build/src/sdk_data/schemas.rs`
- `collapse_single_anonymous_sequence_child`

Problem:

- The function collapses nested anonymous sequences and ORs
  `optional`/`repeated`.
- This preserves simple cardinality flags but may still turn a repeated grouped
  sequence into independent repeated leaves in later passes.

Why this is risky:

- Group repetition is not always equivalent to each leaf being independently
  repeatable.
- Example shape: `(a, b)*` differs from `a*, b*`.

Expected fix:

- Only collapse nested sequences automatically when both wrappers are
  effectively `1..1`.
- If either wrapper is repeatable, preserve the helper/sequence boundary unless
  a later pass can prove equivalence.

### 8. Single Nested Child Fallback Recurses Through All Wrappers

Location:

- `crates/ooxmlsdk-build/src/sdk_code/codegen_ir_builder.rs`
- `resolve_single_nested_schema_child`

Problem:

- The fallback recursively unwraps single-child `Choice` and `Sequence` nodes.
- It ORs cardinality flags, but it does not preserve the distinction between a
  single optional leaf and an optional/repeated branch wrapper.

Why this is risky:

- This is acceptable for simple `1..1` wrappers.
- It can be wrong when the wrapper itself is repeatable or when wrapper order is
  meaningful.

Expected fix:

- Keep the fallback, but block recursion through repeatable wrappers until a
  concrete fixture proves the flattened field is equivalent.

### 9. Version Propagation Is Better Than Cardinality Propagation But Still Needs Audit

Location:

- `effective_version` use in schema lowering and codegen helpers.

Problem:

- Several paths preserve version while not preserving occurrence with the same
  rigor.
- Some version propagation is applied after collapse, making it hard to tell
  whether the collapse was safe.

Why this matters:

- The parser must keep version-gated extension children in the correct choice
  stream even when validation is not performed.

Expected fix:

- Treat non-empty/non-Office2007 version on wrappers as a reason not to flatten
  unless explicitly tested.

### 10. MCE Positions Must Be Represented Statically

Status: fixed.

Location:

- `Types[].AlternateContent` extension handling
- `Types[].AlternateContentChoice` and `Types[].AddChoice` handling
- typed `AlternateContent` variants in choice enum extensions
- schema extensions under `sdk_data/schema_extensions/`

Problem:

- Extension-added raw XML made ordering possible but erased which children an
  MCE branch may contain.
- Generic raw storage also caused MCE processing to be generated for types
  that have no observed MCE position.

Why this matters:

- `mc:AlternateContent` can select zero or more sibling children, so it belongs
  to the parent content model rather than any one child.
- Typed positions preserve round-trip order while keeping normal child parsing
  static.

Implemented shape:

- Fixed parent positions use typed `AlternateContent` fields; one position has
  no numeric suffix and multiple positions are numbered from zero.
- Every fixed position declares its supported parent fields explicitly. One
  selected branch may flatten zero or more of those children into the parent;
  children outside the position's declared set are rejected.
- Ordered streams use a typed choice containing every known child and
  `AlternateContent`.
- Generic extension-added wildcard fallbacks were removed. The only retained
  extension raw route is the QName-scoped `GraphicData` compatibility case for
  `wp:wsp`; schema-native wildcard content remains raw by design.

### 11. Mixed Sequence Direct Children Were Forced Optional

Status: fixed.

Location:

- `crates/ooxmlsdk-build/src/sdk_data/schemas.rs`
- removed `mark_mixed_sequence_direct_children_optional`

Problem:

- After lowering a sequence that contained a choice, the generator marked every
  direct child outside that choice as optional.
- This ignored the direct child's own `Occurs` and changed required `1..1`
  particles into `Option<T>`.

Examples:

- `GroupCommand`: `dgMkLst` is an upstream
  `ElementParticle(..., 1, 1)`, but was generated as
  `Option<DrawingMonikerList>`.
- `TaskHistoryEvent`: `atrbtn` is an upstream
  `ElementParticle(..., 1, 1)`, but was generated as
  `Option<AtrbtnTaskAssignUnassignUser>`.

Expected model:

- Direct children keep their own occurrence.
- The sibling choice keeps the choice particle occurrence, such as `0..1` to
  `Option<Choice>` or repeated choice to `Vec<Choice>`.
- Parse-not-verify does not mean required children are globally rewritten to
  optional; absence can still be represented by `Default` for generated structs
  and handled by validation later.

## Current Working Hypothesis

The current Rust type system is mostly sound. The known round-trip failures are
more likely caused by narrow generator bugs than by the overall Rust modeling
strategy.

The highest-value small fixes are:

1. Preserve repeated flags in structured sequence variants.
2. Stop collapsing single-child choice wrappers that carry occurrence.
3. Tighten one-sequence and anonymous wrapper flattening so only semantic-free
   wrappers are flattened.
4. Only then revisit choice field cardinality promotion.

## Validation Plan

For each fix:

1. Add or update a generator unit test that models the minimal particle shape.
2. Regenerate with:
   `cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture`
3. Run:
   `cargo fmt --all`
4. Run the focused package lane:
   `cd ../ooxmlsdk-test-suite && cargo test -p ooxmlsdk-test round_trip_smoke_test -- --nocapture`
5. If the change touches common schema generation, run:
   `cd ../ooxmlsdk-test-suite && cargo test -p ooxmlsdk-test`

Do not broaden the model or add a new particle verifier unless a concrete
fixture proves the Rust parse model cannot represent the required shape.

## Review Checklist

When reviewing generated diffs, check:

- Did a field change from `Option<T>` to `Vec<T>` or the reverse?
- Did a direct child field become a choice enum or helper struct?
- Did a helper struct disappear where upstream has a repeatable sequence branch?
- Did a choice enum lose nested grouping around sequence variants?
- Did typed `AlternateContent` remain in the correct parent slot or ordered
  choice stream for MCE-sensitive parents?
- Did Office version attributes remain on the field or variant that actually
  owns the version gate?

Generated diffs are acceptable when they make the Rust parse model closer to
the upstream particle semantics without turning the runtime into a validator.
