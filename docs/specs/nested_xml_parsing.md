# Nested XML Parsing

## Problem

The package open path is intentionally eager: root elements are parsed during
`WordprocessingDocument::new_from_file()` via `initialize_root_elements()`.
This must stay true. The current generated XML readers are recursive descent
readers, so a structurally valid but deeply nested document can consume one Rust
call frame per XML nesting level.

The Apache POI fixture `document/deep-table-cell.docx` demonstrates the issue.
It contains thousands of nested WordprocessingML table/cell levels. A round-trip
fails before save/reopen: `Document::from_bytes()` enters the generated
`Table -> TableRow -> TableCell -> Table` parse cycle until the process stack
overflows.

The fix is not lazy root parsing, thread stack enlargement, or a one-off table
special case. The eager parser needs a generated stack-safe path for schema
cycles.

## Upstream Evidence

Apache POI is useful as fixture evidence, not as the implementation model to
copy.

- `../poi/poi-integration/src/test/java/org/apache/poi/stress/TestAllFiles.java`
  excludes `document/deep-table-cell.docx` under the "stress docs" section.
- `../poi/poi-integration/src/test/java/org/apache/poi/stress/POIXMLDocumentHandler.java`
  has `cursorRecursive(XmlCursor)` for full XMLBeans cursor traversal.
- `../poi/poi-integration/src/test/java/org/apache/poi/stress/XWPFFileHandler.java`
  opens an `XWPFDocument`, recursively walks the XMLBeans document, and writes it
  back.
- `../poi/poi-ooxml/src/main/java/org/apache/poi/xwpf/usermodel/XWPFTableCell.java`
  constructs nested `XWPFTable` wrappers while iterating cell children.

That means POI also treats this fixture as a stress-depth case. Its XMLBeans and
XWPF layers do not give us a direct stack-safe generated typed parser design.

LibreOffice is the better architectural reference.

- `../core/sax/source/fastparser/fastparser.cxx` uses `maContextStack` on start
  and end element events. XML depth is represented as parser state, not as C++
  call depth.
- `../core/sw/source/writerfilter/dmapper/DomainMapper_Impl.hxx` tracks
  `nTableDepth`, `nTableCellDepth`, stream state stacks, table manager stacks,
  property stacks, and context stacks.
- `../core/sw/source/writerfilter/dmapper/DomainMapper.cxx` updates table and
  cell depth on table/cell start/end tokens.
- `../core/sw/source/writerfilter/dmapper/TableManager.cxx` maintains
  `mTableDataStack` with `startLevel()` and `endLevel()`.

The useful lesson is explicit state: use the XML event stream and generated
metadata to push/pop typed parse frames, especially for recursive structures.

## Rust Parser References

The Rust XML ecosystem points to the same event/state-machine model.

- `quick-xml` documents its streaming API as a StAX-style pull reader/writer and
  explicitly says nested XML depth is tracked by the user. Its event types are
  zero-copy, and this repository already builds generated readers and writers on
  top of `quick_xml::events::Event`.
- `xmlparser` is a low-level pull tokenizer. It is zero-allocation and keeps
  spans into the original input, but it deliberately does not validate tree
  structure for callers. It is a useful reference for allocation discipline, not
  a reason to replace `quick-xml`.
- `xml-rs` also exposes an `EventReader` pull API with subtree skipping, but it
  is not a better fit for this repository's current performance path.
- `roxmltree` builds a read-only tree. That is useful for DOM-style access, but
  it would add an intermediate untyped tree and duplicate memory before building
  the generated typed DOM.

The conclusion is to keep `quick-xml` as the tokenizer/event source and move
schema recursion out of Rust call frames into generated typed state.

Local source references:

- `../roxmltree/src/parse.rs` is the best Rust reference for explicit XML tree
  state. Its parser keeps `parent_id`, `parent_prefixes`, `awaiting_subtree`,
  current attributes, and text buffers in a `Context`; open elements update the
  current parent, and close elements restore the parent. We should borrow the
  explicit-state model, not the untyped DOM.
- `../core/sax/source/fastparser/fastparser.cxx` is the best event/context
  stack reference. It pushes a context on start, sends characters to the top
  context, and pops on end.
- `../core/sw/source/writerfilter/dmapper/` is the best OOXML domain-state
  reference. It keeps table depth, table-cell depth, stream stacks, table
  manager stacks, property stacks, and context stacks.

The target shape is therefore:

```text
quick_xml::Event -> generated SCC frame stack -> final typed OOXML structs
```

not:

```text
quick_xml::Event -> untyped XML DOM -> typed OOXML structs
```

## Current Recursion Shape

The known failing WML path is choice-driven:

```text
Body.body_choice -> BodyChoice::Table
Table.table_choice2 -> TableChoice2::TableRow
TableRow.table_row_choice -> TableRowChoice::TableCell
TableCell.table_cell_choice -> TableCellChoice::Table
```

A scan of the checked-in generated schema structs found:

- 3673 generated structs
- 3265 direct child edges
- 3124 choice child edges
- 2 direct-only cycles

The direct-only cycles are:

- `www_w3_org_2003_ink_ml::Mapping -> Mapping`
- `schemas_openxmlformats_org_wordprocessingml_2006_main::Div -> DivsChild -> Div`

The Apache POI overflow is in a choice cycle, but the implementation target is
the whole recursive SCC, not a single choice edge. Direct cycles should use the
same SCC mechanism instead of being handled by an unrelated special case.

## Design

Keep normal generated recursive code for acyclic schema edges. It is simple and
fast for ordinary documents. Generate an iterative path only for edges that are
statically known to participate in a schema cycle.

The generator should build a type graph from schema metadata:

- node: generated schema type
- edge: child, sequence child, or choice variant that constructs another schema
  type
- recursive edge: any edge that stays inside a strongly connected component with
  more than one node, or a self-edge

The derive macro sees only one item at a time, so the global recursion decision
should be emitted by the generator into the generated schema source, for example
as field/choice metadata. `ooxmlsdk-derive` should then generate direct code
from that metadata without recomputing the whole graph.

For read paths, do not generate a second monolithic parser that duplicates all
child handling for the SCC. Instead, extend the normal generated `SdkType`
reader shape so recursive types expose reusable private steps:

- `__ooxmlsdk_read_start_borrowed(...)`: parse the start element into the same
  typed value/builder state the normal reader already creates.
- `__ooxmlsdk_read_child_borrowed_step(...)`: consume one child start/empty
  event for this type.
- The child step returns either "handled normally" or "push this SCC frame" for
  an SCC-internal edge.
- On an SCC-external child, keep the current generated
  `Child::read_*_inner()` call.
- On an end element, finish the top frame, wrap it in the generated choice
  variant or child field, and attach it to the parent frame.
- Schema-declared `any` children continue to use the existing raw
  XML/depth-counting helpers. Unknown children are skipped as complete
  subtrees; malformed XML still returns an error.

The SCC loop is therefore only the control stack. Most parsing code remains the
ordinary per-type generated code, just factored into start/step/finalize pieces
for types that are inside a recursive SCC.

For write paths, apply the same factoring:

- Recursive SCC types expose a private generated write step that emits this
  type's attributes and non-recursive children exactly as the normal writer
  would.
- SCC-internal child/choice values return push tasks instead of recursively
  calling the child writer.
- SCC-external children keep the current generated `write_inner()` calls.
- Task order must preserve the existing serialized child order.
- The public typed DOM remains unchanged.

This keeps the optimized path small: only recursive types get frame code, only
recursive edges return push tasks, and acyclic generated code remains direct.

## Zero-Cost Constraints

The optimized path should be generated as concrete Rust code, not as a dynamic
runtime parser framework. The target is:

```text
acyclic generated paths: zero additional overhead
recursive SCC paths: lower total cost than recursive descent on deep input
```

- No `dyn` parser traits in the hot parse/write loop.
- No per-event heap allocation beyond the typed DOM allocations already required
  by the public structs and vectors.
- No string-name dispatch for known schema children after the existing QName
  match has selected a generated branch.
- No global runtime graph lookup during parsing. The graph is a generator-time
  decision.
- Frame enums and task enums should be concrete generated types, so dispatch is
  a local `match` over known variants.
- Frame vectors should be reused within one root parse/write operation. Capacity
  growth is allowed, but recursion depth should not allocate one boxed parser
  object per element.
- Attribute parsing should stay in the generated start-element handler for each
  concrete type.
- Unknown/raw XML handling should stay on the current depth-counted raw helpers
  to avoid building typed frames for data that remains raw.
- Borrowed parsing must keep `quick_xml::events::BytesStart<'de>` borrowed from
  the input slice where possible. Do not call `into_owned()` on known recursive
  starts in the `SliceReader` path.
- The IO reader can keep its existing owned-event behavior. The package root
  parse path that matters for round-trip is byte-slice based and should get the
  zero-copy implementation first.
- Generated frame constructors should parse attributes immediately from the
  borrowed start event and then drop the event unless the type already needs to
  preserve it. A frame should not store XML event objects just to remember the
  element name.

For the common acyclic case, generated code should remain equivalent to today:
one direct function call per child and no additional frame checks. The iterative
path is selected only at statically marked recursive edges.

The recursive SCC path should outperform the current implementation on deep
input because it replaces thousands of nested Rust calls with a tight loop over
`quick_xml::Event`, one `Vec` stack, and generated `match` dispatch. The shallow
recursive case may pay a small `Vec` push/pop cost, but only for schema types
inside recursive SCCs.

## Switching Boundary

The switching unit is the recursive SCC, not an individual "source" edge and
not the whole document tree.

For a cycle such as:

```text
Table -> TableRow -> TableCell -> Table
```

all generated reader entries inside the SCC must switch to the SCC loop:

```text
Table::read_borrowed_inner    -> __read_scc_N_borrowed(TableFrame)
TableRow::read_borrowed_inner -> __read_scc_N_borrowed(TableRowFrame)
TableCell::read_borrowed_inner -> __read_scc_N_borrowed(TableCellFrame)
```

and all SCC-internal child edges must push frames:

```text
Table sees TableRow       -> push TableRowFrame
TableRow sees TableCell   -> push TableCellFrame
TableCell sees Table      -> push TableFrame
```

Changing only the apparent back edge, for example `TableCell -> Table`, is not
general enough. It leaves other SCC entries and internal edges on the old call
stack path. At the same time, switching `Document`, `Body`, or unrelated
children is unnecessary unless they are part of a recursive SCC.

For the known WML table failure, the special stack frames are only the three
element structs in the recursive component:

```text
Table
TableRow
TableCell
```

The surrounding choices are not independent frame types. They remain normal
generated enum wrapping/dispatch metadata. This is why folding the stack parser
into the existing generated read/write path is appropriate: the large majority
of code is already generated per concrete type, and the SCC loop only needs to
intercept the three internal typed transitions.

The boundary is:

```text
type in recursive SCC      -> entry delegates to SCC parser/writer
edge inside same SCC       -> push frame/task
edge outside current SCC   -> call existing generated read/write path
acyclic type and edge      -> unchanged
```

## Release Path

Stack-safe read and write are not enough for deeply nested owned DOMs. After a
round trip has parsed and written successfully, Rust still has to release the
typed tree. The default drop glue recursively drops owned fields:

```text
Table
-> Vec<TableChoice2>
-> TableChoice2::TableRow(Box<TableRow>)
-> TableRow
-> Vec<TableRowChoice>
-> TableRowChoice::TableCell(Box<TableCell>)
-> TableCell
-> Vec<TableCellChoice>
-> TableCellChoice::Table(Box<Table>)
-> ...
```

This is normal Rust behavior for recursive owned structures. It is the same
shape as the classic `Box` linked-list drop overflow: construction can succeed,
but releasing the root recursively walks one owned child at a time on the call
stack.

The release fix should be a generated clear hook, not public `Drop`
implementations on generated schema structs or choice enums.

Why not implement `Drop` on public schema types:

- Generated schema structs and enums are public typed DOM values.
- Adding `Drop` to `Table`, `TableRow`, `TableCell`, or choice enums changes
  field-move behavior for user code.
- `Drop` would spread to every recursive SCC and turn an internal robustness
  fix into an API constraint.
- A type-level `Drop` still does not match the intended boundary: the package
  cache is the owner that needs stack-safe release during eager round trips.

The intended release boundary is package-owned root elements:

```text
package/root cache release -> generated root clear hook -> default shallow drop
```

Before a package-owned root element is removed from the cache or dropped with
the package, the package layer should call a generated clear function for root
types that can contain recursive SCCs. The clear function mutates the owned DOM
only at release time. It must not run before save, before validation, or while a
user-visible root reference is still in use.

For the WML table SCC, the generated clear hook should only drain SCC-internal
recursive edges:

```text
Table.table_choice2           -> TableChoice2::TableRow(Box<TableRow>)
TableRow.table_row_choice     -> TableRowChoice::TableCell(Box<TableCell>)
TableCell.table_cell_choice   -> TableCellChoice::Table(Box<Table>)
```

The hook uses an explicit stack:

```text
push root tables
while stack not empty:
  Table:
    take(table.table_choice2)
    push TableRow payloads from TableChoice2::TableRow
    let other variants drop normally
  TableRow:
    take(row.table_row_choice)
    push TableCell payloads from TableRowChoice::TableCell
    let other variants drop normally
  TableCell:
    take(cell.table_cell_choice)
    push Table payloads from TableCellChoice::Table
    let other variants drop normally
```

After the hook runs, the recursive owned edges are empty, so Rust's normal drop
glue only sees a shallow remaining object graph for the SCC. Non-recursive
fields, attributes, properties, paragraphs, and external child types continue to
drop normally.

This is intentionally different from the `roxmltree` storage model. `roxmltree`
avoids recursive drop by storing every node in a flat `Vec<NodeData>` and using
node ids for parent and sibling links. Replacing this repository's public typed
DOM with an arena would be a larger API and ownership redesign. The generated
clear hook keeps the public typed DOM intact while applying the same underlying
principle at the package-owned release boundary: represent recursion with an
explicit stack instead of the call stack.

LibreOffice also supports this boundary choice. Its importer resolves nested
tables through event and table-data stacks (`maContextStack`,
`mTableDataStack`, `startLevel()`, `endLevel()`), not by relying on recursive
object destruction. For this repository, read, write, and package-owned release
should all use generated explicit state for recursive SCCs.

Release-path zero-cost constraints:

- Do not add `Drop` to public generated schema structs or public choice enums.
- Do not change public field types, field visibility, or typed DOM ownership.
- Do not run a clear hook during normal read, mutation, validation, or write.
- Only package/root-cache release paths call clear hooks.
- Acyclic root elements and root elements without recursive SCC descendants do
  not call any hook.
- The hook dispatch is static: root variant -> generated clear function.
- SCC edge handling is static: generated match arms for known recursive
  variants, no runtime schema graph lookup.
- The hook may allocate one `Vec` stack proportional to recursive depth. It
  should not allocate wrapper parser objects or clone DOM nodes.

Open edge case:

- If user code unloads a root element and then owns a deeply recursive public
  DOM directly, the package can no longer clear it on drop. Covering that case
  requires an explicit public release helper or a larger ownership-wrapper
  design. It should not be solved by adding `Drop` to public schema types.

This preserves zero additional overhead for acyclic generated paths while making
every entry into the recursive region stack-safe.

## Zero-Copy Read Mechanics

The borrowed root parse path should use the existing `SliceReader<'de>`:

```rust
pub fn next(&mut self) -> Result<quick_xml::events::Event<'de>, SdkError>
```

That keeps event data borrowed from the package XML bytes. The SCC parser should
consume those events directly:

```text
Event::Start(e) -> parse attributes from e, push concrete frame
Event::Empty(e) -> parse attributes from e, finish child immediately
Event::Text(t)  -> decode only for fields that need text
Event::End(e)   -> finish top frame
```

Do not introduce an owned event queue for recursive parsing. The only stack
state should be typed frames. If a field needs string data, keep the same
allocation behavior as today's generated parser: parse into the final typed
field, not into an intermediate XML tree.

QName dispatch should stay byte based:

```text
event.name().as_ref() -> generated byte-string match arms
```

This preserves the current no-string-allocation matching model.

## Generated Frame Shape

For each recursive strongly connected component, generate a small closed enum
that represents only concrete element frames in that component. For the WML
table cycle this would be shaped like:

```rust
enum WmlTableFrame {
  Table(TableFrame),
  TableRow(TableRowFrame),
  TableCell(TableCellFrame),
}
```

Each concrete frame wraps the same typed value/builder state that the generated
reader already uses:

```rust
struct TableFrame {
  value: Table,
}
```

The start event is consumed immediately. The generated frame constructor parses
attributes from `BytesStart<'de>`, initializes the typed value, and then drops
the event. The frame should not store `BytesStart` just to remember the element
name; the end tag is known from the frame variant.

The actual generated frame can be just the typed value when that is already the
cheapest builder representation. It can split parsed attributes from child
storage if a specific type benefits from that, but the important constraint is
that it is concrete and statically known. It should not store closures, boxed
parser objects, dynamic callbacks, or owned XML events.

Use `Vec<WmlTableFrame>` as the default stack representation. The stack depth is
runtime data, so the vector itself cannot be replaced by a fixed-size static
array. The important static property is that the vector element type and every
transition are generated at compile time.

`Vec<FrameEnum>` is the preferred first implementation because it is compact in
codegen terms:

- one contiguous stack allocation per SCC parse
- no per-frame heap allocation
- no vtable
- one enum discriminant and one generated `match` per event
- direct move of finished typed values into parent fields

If a future SCC has very uneven frame sizes and benchmarks show enum padding is
material, the generator can select a second representation for that SCC:

```text
Vec<FrameTag> + one Vec<ConcreteFrame> per frame type
```

That lowers per-depth memory for uneven SCCs but adds more generated bookkeeping
and more vector traffic. It should be a generated fallback selected by size
heuristics, not the default path.

The generated loop should look like a small pushdown controller over the normal
type steps:

```text
read event
match top frame
  Table     -> Table::__read_child_step(...)
  TableRow  -> TableRow::__read_child_step(...)
  TableCell -> TableCell::__read_child_step(...)
step result:
  recursive start/empty -> push or finish child frame
  nonrecursive start/empty -> already handled by the normal generated branch
  end -> finish top frame and attach to parent
  unknown/raw -> existing depth-counted raw XML helper
```

This matches the way `quick-xml` expects callers to track nested depth, but the
state is generated from schema metadata instead of interpreted from a runtime
schema table.

## Codegen Strategy

Generate two kinds of code.

For acyclic types and acyclic child edges, keep the current shape:

```text
match child qname
  known child -> Child::read_borrowed_inner(reader, start, empty)
```

No SCC helper is called and no recursion metadata is checked at runtime.

For recursive SCC types, factor the existing generated code and replace only the
public `SdkType` entry methods:

```text
TypeInScc::read_borrowed_inner(reader, start, empty)
  -> __read_scc_N_borrowed(reader, __FrameSccN::TypeInScc(TypeInScc::__read_start(...)))
```

Inside `__read_scc_N_borrowed`:

- call the top frame's generated child step
- child type in the same SCC: push a generated frame
- child type outside the SCC: the generated child step calls the existing child
  reader
- child type from another SCC: the generated child step calls that SCC's normal
  entry method
- unknown/raw child: the generated child step calls the existing raw XML helper

This makes recursive SCC parsing composable without requiring a global parser.
It also covers self-cycles such as `Mapping -> Mapping`; a self-cycle is an SCC
with one type and at least one edge back to itself.

The module-level generator still has work that derive cannot do alone:

- choose the SCC id/stem
- generate the closed `Frame` enum
- generate the attach enum/function for internal edges
- generate the shared stack loop for the SCC

The per-type derive work should be local:

- parse the optional `#[sdk(stack_parser(...))]` or SCC metadata
- generate the private start/child/finalize/write-step helpers
- route the normal `SdkType` entry to the SCC loop only for types in the SCC

This avoids a huge function-like macro invocation full of per-child arms. The
large match arms stay where they already belong: in each type's generated
derive expansion.

Attach code is generated per parent field:

```text
choice child -> parent.choice_vec.push(ChoiceVariant::Child(child))
direct child -> parent.child_vec.push(child)
optional child -> parent.child = Some(child)
single child -> parent.child = child
```

The frame stack owns only unfinished typed values. Finished children are moved
directly into the final parent field. There is no intermediate `Vec<Event>`,
DOM node, or erased "parsed child" box.

## Final Implementation Decisions

- Generate one private frame enum per recursive SCC.
- Generate one private borrowed SCC loop per recursive SCC.
- Generate one private write task loop per recursive SCC.
- Reuse the normal per-type generated read/write branches by factoring them into
  private start/child/write-step helpers.
- Switch every generated read/write entry for every type in that SCC.
- Push frames/tasks for every edge inside the same SCC.
- Keep SCC-external and acyclic edges on the existing generated direct call
  path.
- Start with the borrowed `SliceReader<'de>` path. The IO path can initially
  continue to use the existing recursive implementation if it is not used by the
  package eager root parse path; once the borrowed path is stable, mirror the SCC
  mechanism for IO.
- Use `Vec<FrameEnum>` as the default stack representation.
- Do not store `BytesStart` in frames. Parse attributes from the start event and
  drop it.
- Do not introduce a runtime schema table, untyped DOM, owned event queue, or
  dynamic parser trait.

## Allocation Strategy

Use one frame stack per SCC parse call:

```rust
let mut stack = Vec::<__FrameSccN>::new();
```

The implementation can start with `Vec::new()` and rely on amortized growth. If
benchmarks show shallow SCC documents regress, add a small generated initial
capacity such as `Vec::with_capacity(8)` for SCCs that are common in WML/PML/SML.
Do not allocate a frame on the heap with `Box`; the enum value is pushed directly
into the vector.

Large enum size is a risk for SCCs with uneven frame sizes. If a generated SCC
contains one unusually large frame, split stored state so the common frame size
stays close to the current call-frame locals. Use concrete helper structs, not
trait objects.

Text and attribute allocations should be exactly the allocations required for
the final typed fields. Known element names and attribute names remain borrowed
byte comparisons against generated literals.

## MCE And Raw XML

Default round-trip does not depend on the `mce` feature. MCE replacement and
compatibility traversal are separate behavior and should not block the first
fix.

Schema-declared raw XML paths are already depth-based rather than
schema-recursive:

- `any`
- MCE raw-element helpers when the feature is enabled

After the default parser fix, MCE should get a focused follow-up audit to ensure
its transformed traversal does not reintroduce schema-depth recursion.

## Implementation Steps

1. Add schema graph SCC classification in the generator.
2. Emit SCC id and SCC-internal edge metadata into generated schema definitions.
3. Update `ooxmlsdk-derive` to factor recursive SCC types into private
   start/child/finalize/write-step helpers.
4. Generate module-level frame/attach/task enums and SCC loops from the SCC
   metadata.
5. Route recursive SCC type entries to the SCC loop while keeping acyclic types
   unchanged.
6. Validate with the Apache POI deep table fixture in `../ooxmlsdk-test-suite`.
7. Validate direct/self cycles (`Mapping`, `Div/DivsChild`) with focused
   fixtures or generated minimal XML samples.

## Validation

Primary target:

```text
../ooxmlsdk-test-suite Apache POI round-trip for document/deep-table-cell.docx
```

Performance targets:

- Acyclic document parse/write should not regress beyond normal benchmark noise.
- Deep recursive SCC input should parse without stack overflow and should be
  faster than a debug-build recursive descent that approaches stack exhaustion.
- Borrowed root parsing should not increase owned XML event allocations.
- The optimized path should not allocate an untyped DOM or raw XML buffer for
  known typed children.

Local repository checks for the first implementation should include:

```text
cargo test -p ooxmlsdk-derive dump_context_node_expansion -- --ignored --nocapture
cd ../ooxmlsdk-test-suite && cargo test -p ooxmlsdk-test
```

When generator output changes, follow the repository generator loop:

```text
cargo test -p ooxmlsdk-build test_gen -- --ignored --nocapture
cargo fmt --all
```

The macro dump should be inspected for the WML `Document`/table-related types to
confirm that recursive SCC entries and SCC-internal edges use the frame path and
ordinary acyclic edges still use the direct path.

Benchmark follow-up:

```text
cd ../ooxmlsdk-test-suite && cargo bench -p ooxmlsdk-test --bench perf
```

Add or reuse a fixture that has moderate nested tables, not only the 5000-level
stress file, so benchmark data covers realistic recursive input and adversarial
recursive input separately.

## Non-Goals

- Switching root elements to lazy parsing.
- Increasing process or thread stack size.
- Parsing the whole package through an untyped DOM as an intermediate model.
- Hand-patching only `w:tbl`, `w:tr`, or `w:tc`.
- Rewriting MCE in the first parser fix.

## Open Decisions

- Exact generated metadata spelling for SCC ids and SCC-internal edges.
- Whether validation fixtures for direct/self cycles are added before or after
  the Apache POI deep table fixture is fixed. The implementation mechanism
  should still be SCC-wide, not choice-only.
- Whether benchmark fixtures for moderate recursive nesting live in this
  repository or only in `../ooxmlsdk-test-suite`.
