use crate::FormulaOperator;
use crate::program::{
  FormulaExprId, FormulaFunctionName, FormulaNodeKind, FormulaOperandClass, FormulaParamClass,
  FormulaProgram, FormulaReference,
};
use crate::source::FormulaSource;

/// Expose legacy scalar evaluation in the dynamic-array formula-bar dialect.
/// Source spans preserve spelling, whitespace and grouping. This is a display
/// conversion; neither the stored formula nor its evaluation program changes.
pub(crate) fn display_legacy_implicit_intersections(
  source: &str,
  reference_is_multi_cell: impl Fn(&str) -> bool,
) -> String {
  let program = FormulaProgram::from_source(FormulaSource {
    text: source,
    context: Default::default(),
  });
  let Some(root) = program.root.filter(|_| program.diagnostics.is_empty()) else {
    return source.to_string();
  };
  let mut visitor = ImplicitIntersectionDisplay {
    program: &program,
    source,
    reference_is_multi_cell,
    positions: Vec::new(),
  };
  visitor.visit(root, FormulaParamClass::Value, false);
  visitor.positions.sort_unstable();
  visitor.positions.dedup();
  let mut output = String::with_capacity(source.len() + visitor.positions.len());
  let mut start = 0;
  for position in visitor.positions {
    output.push_str(&source[start..position]);
    output.push('@');
    start = position;
  }
  output.push_str(&source[start..]);
  output
}

struct ImplicitIntersectionDisplay<'a, F> {
  program: &'a FormulaProgram,
  source: &'a str,
  reference_is_multi_cell: F,
  positions: Vec<usize>,
}

impl<F: Fn(&str) -> bool> ImplicitIntersectionDisplay<'_, F> {
  fn visit(&mut self, id: FormulaExprId, expected: FormulaParamClass, force_array: bool) {
    let Some(node) = self.program.node(id) else {
      return;
    };
    let scalar = expected == FormulaParamClass::Value && !force_array;
    match &node.kind {
      FormulaNodeKind::Reference(reference) => {
        let Some(span) = node.span else {
          return;
        };
        let multiple = match reference {
          FormulaReference::Range(range) => {
            range.start.address != range.end.address || range.start.sheet != range.end.sheet
          }
          FormulaReference::Named(_) | FormulaReference::Structured(_) => {
            (self.reference_is_multi_cell)(&self.source[span.start..span.end])
          }
          _ => false,
        };
        if scalar && multiple {
          self.positions.push(span.start);
        }
      }
      FormulaNodeKind::Unary {
        op: FormulaOperator::ImplicitIntersection,
        ..
      } => {
        // An authored @ already describes this scalar boundary, including any
        // expression whose array result is its operand.
      }
      FormulaNodeKind::Unary { expr, .. } => {
        self.visit(*expr, FormulaParamClass::Value, force_array)
      }
      FormulaNodeKind::Binary { op, left, right } => {
        if matches!(
          op,
          FormulaOperator::Range | FormulaOperator::Union | FormulaOperator::Intersection
        ) {
          // These operators construct a reference; their operands are not
          // scalar arguments. Complex reference-result grouping stays intact.
          self.visit(*left, FormulaParamClass::Reference, force_array);
          self.visit(*right, FormulaParamClass::Reference, force_array);
        } else {
          self.visit(*left, FormulaParamClass::Value, force_array);
          self.visit(*right, FormulaParamClass::Value, force_array);
        }
      }
      FormulaNodeKind::Function { name, args } => {
        let (FormulaFunctionName::Unknown(symbol) | FormulaFunctionName::External(symbol)) = name
        else {
          return;
        };
        let Some(name) = self.program.symbols.get(*symbol) else {
          return;
        };
        let Some(signature) = display_signature(name) else {
          return;
        };
        let Some(args) = self.program.args(*args) else {
          return;
        };
        if !signature.arity.contains(&args.len()) {
          return;
        }
        if scalar
          && matches!(
            signature.result,
            FormulaOperandClass::Reference | FormulaOperandClass::Array
          )
          && let Some(span) = node.span
        {
          self.positions.push(span.start);
        }
        for (index, &arg) in args.iter().enumerate() {
          let class = signature
            .parameters
            .get(index)
            .copied()
            .unwrap_or(signature.rest);
          self.visit(
            arg,
            class,
            force_array || class == FormulaParamClass::ForceArray,
          );
        }
      }
      // Array literals and lambda bodies use array evaluation internally.
      // Unknown function signatures cannot establish a scalar boundary.
      _ => {}
    }
  }
}

struct DisplaySignature {
  result: FormulaOperandClass,
  parameters: &'static [FormulaParamClass],
  rest: FormulaParamClass,
  arity: std::ops::RangeInclusive<usize>,
}

fn display_signature(name: &str) -> Option<DisplaySignature> {
  use FormulaOperandClass::{Array, Reference, Value};
  use FormulaParamClass::{ForceArray as A, Reference as R, Value as V};
  let upper = name.to_ascii_uppercase();
  if upper.starts_with("_XLFN.")
    && !matches!(upper.as_str(), "_XLFN.SINGLE" | "_XLFN.ANCHORARRAY")
    && !include_str!("excel_future_function_names.txt")
      .split_ascii_whitespace()
      .any(|known| known == upper)
  {
    return None;
  }
  let name = upper.strip_prefix("_XLFN.").unwrap_or(&upper);
  let name = name.strip_prefix("_XLWS.").unwrap_or(name);
  // Microsoft Formula/Formula2 documents scalar SQRT arguments and reference
  // arguments to SUM; its @ documentation identifies INDEX/OFFSET result coercion.
  // DynamicArrayFixture's configured Office PDF additionally establishes UNIQUE,
  // ANCHORARRAY, TYPE/ISREF and the aggregate spill-reference arguments.
  let (result, parameters, rest, arity): (_, &'static [_], _, _) = match name {
    "SINGLE" => (Value, &[A], A, 1..=1),
    "ANCHORARRAY" => (Reference, &[R], R, 1..=1),
    "INDEX" => (Reference, &[R], V, 2..=4),
    "OFFSET" => (Reference, &[R], V, 3..=5),
    "UNIQUE" => (Array, &[A], V, 1..=3),
    "SORT" => (Array, &[A], V, 1..=4),
    "TRANSPOSE" | "MINVERSE" => (Array, &[], A, 1..=1),
    "MMULT" => (Array, &[], A, 2..=2),
    "SUMPRODUCT" => (Value, &[], A, 1..=255),
    "SUM" | "SUMSQ" | "PRODUCT" | "MIN" | "MAX" | "AVERAGE" | "COUNT" | "COUNTA" | "AND" | "OR"
    | "TYPE" | "ISREF" | "ISFORMULA" | "ROWS" | "COLUMNS" | "AREAS" => (Value, &[], R, 1..=255),
    "SUBTOTAL" => (Value, &[V], R, 2..=255),
    "COUNTIF" | "AVERAGEIF" | "SUMIF" => (Value, &[R, V], R, 2..=3),
    "ABS" | "SQRT" | "EXP" | "LN" | "LOG" | "LOG10" | "SIGN" | "INT" | "TRUNC" | "ROUND"
    | "ROUNDDOWN" | "ROUNDUP" | "MOD" | "POWER" | "SIN" | "COS" | "TAN" | "ASIN" | "ACOS"
    | "ATAN" | "ATAN2" | "NOT" | "ISNUMBER" | "ISTEXT" | "ISNONTEXT" | "ISLOGICAL" | "ISBLANK"
    | "ISERROR" | "ISERR" | "ISNA" | "LEN" | "LOWER" | "UPPER" | "TRIM" | "LEFT" | "RIGHT"
    | "MID" => (Value, &[], V, 1..=255),
    _ => return None,
  };
  Some(DisplaySignature {
    result,
    parameters,
    rest,
    arity,
  })
}
