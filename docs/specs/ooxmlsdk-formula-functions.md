# Formula Function Registry

This note records the current formula function boundary for `ooxmlsdk-formula`.
It is scoped to the function surface already implemented by the evaluator.

## References

- LibreOffice function names and grammar symbols:
  `../core/formula/inc/core_resource.hrc`
- LibreOffice opcode dispatch:
  `../core/sc/source/core/tool/interpr4.cxx`
- LibreOffice parameter classes:
  `../core/sc/source/core/tool/parclass.cxx`
- POI function metadata registry:
  `../poi/poi/src/main/java/org/apache/poi/ss/formula/function/FunctionMetadataRegistry.java`
- POI evaluator dispatch:
  `../poi/poi/src/main/java/org/apache/poi/ss/formula/OperationEvaluatorFactory.java`
- POI Analysis ToolPak registry:
  `../poi/poi/src/main/java/org/apache/poi/ss/formula/atp/AnalysisToolPak.java`

## Current Boundary

The parser pipeline is:

1. winnow token/syntax parser
2. parser AST
3. `FormulaCode` stack program
4. dependency extraction from `FormulaCode`
5. evaluator facade
6. legacy evaluator internals

`FormulaCode::Call` carries both the original zero-copy function name and an
optional `FormulaFunctionId` enum. The original name keeps unknown, external,
and compatibility paths lossless. The enum id is resolved during lowering and
is used for metadata such as volatility.

The registry is intentionally limited to the names and aliases currently
handled by `FormulaEvaluator::evaluate_function`. It is not a claim of complete
Calc or Excel coverage.

## Design Constraints

- Function name normalization is shared between parser metadata and evaluator
  compatibility dispatch, and returns borrowed text for already-normalized
  uppercase names.
- `_xlfn.` and `_xlws.` prefixes are stripped before lookup.
- Known namespace prefixes keep raw special cases where the current evaluator
  distinguishes them, then use canonical lookup for normal aliases.
- No runtime registry allocation is needed. The current implementation is a
  static match table that resolves aliases into a `Copy` enum id.
- Unknown functions remain evaluable by the compatibility path when supported
  later as external or user-defined functions.

## Next Evaluator Stage

The next stage should move direct evaluator dispatch behind `FormulaFunctionId`
and keep string dispatch only as a compatibility fallback. This mirrors
LibreOffice `OpCode` and POI function indexes: names are parser input, while
the evaluator should operate on stable function identity. That stage should
also import LibreOffice-style parameter classification before broadening the
implemented function set, because scalar/range/array coercion is part of
function semantics rather than parser syntax.
