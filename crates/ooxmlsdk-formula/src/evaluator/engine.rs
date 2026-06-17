use super::*;

pub(crate) struct FormulaEvaluatorEngine<'book, 'engine, 'doc> {
  pub(crate) book: &'book FormulaEvaluationBook<'doc>,
  pub(crate) engine: &'engine CalcEngine,
  pub(crate) current_sheet: SheetId,
  pub(crate) current_cell: Option<CellAddress>,
  pub(crate) grammar: FormulaGrammar,
  pub(crate) array_context: bool,
}

pub(crate) fn evaluate_parsed_formula_raw<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  current_sheet: SheetId,
  current_cell: Option<CellAddress>,
  formula: &ParsedFormula<'doc>,
  array_context: bool,
) -> Option<FormulaValue<'doc>> {
  if !formula.unsupported.is_empty() {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  if let Some(value) =
    book.evaluate_special_formula_text(current_sheet, current_cell, formula.source.as_ref())
  {
    return Some(value);
  }
  let engine = CalcEngine::new();
  FormulaEvaluatorEngine {
    book,
    engine: &engine,
    current_sheet,
    current_cell,
    grammar: formula.grammar,
    array_context,
  }
  .evaluate_code(formula.code.as_ref()?)
}

pub(crate) fn evaluate_formula_text_raw<'doc>(
  book: &FormulaEvaluationBook<'doc>,
  current_sheet: SheetId,
  current_cell: Option<CellAddress>,
  formula: &str,
  grammar: FormulaGrammar,
) -> Option<FormulaValue<'doc>> {
  let parsed = parse_formula(current_sheet, Cow::Owned(formula.to_string()), grammar);
  evaluate_parsed_formula_raw(book, current_sheet, current_cell, &parsed, false)
    .map(FormulaValue::into_owned)
}

impl<'book, 'engine, 'doc> FormulaEvaluatorEngine<'book, 'engine, 'doc> {
  pub(crate) fn evaluate_code(&self, code: &FormulaCode<'doc>) -> Option<FormulaValue<'doc>> {
    if let DirectEvaluation::Value(value) = self.evaluate_code_direct(code) {
      return Some(value);
    }
    let ast = ast_from_code(code)?;
    self.compat_evaluator().evaluate(&ast)
  }

  pub(crate) fn compat_evaluator(&self) -> FormulaEvaluator<'_, 'doc> {
    FormulaEvaluator {
      book: self.book,
      engine: self.engine,
      current_sheet: self.current_sheet,
      current_cell: self.current_cell,
      grammar: self.grammar,
      locals: BTreeMap::new(),
      array_context: self.array_context,
      current_value: None,
    }
  }

  fn evaluate_code_direct(&self, code: &FormulaCode<'doc>) -> DirectEvaluation<'doc> {
    let evaluator = self.compat_evaluator();
    let mut stack = Vec::with_capacity(code.ops.len());
    for op in &code.ops {
      match op {
        FormulaOp::PushBlank => stack.push(EvalOperand::Value(FormulaValue::Blank)),
        FormulaOp::PushText(value) => {
          stack.push(EvalOperand::Value(FormulaValue::String(value.clone())));
        }
        FormulaOp::PushNumber(value) => {
          stack.push(EvalOperand::Value(FormulaValue::Number(*value)));
        }
        FormulaOp::PushBoolean(value) => {
          stack.push(EvalOperand::Value(FormulaValue::Boolean(*value)));
        }
        FormulaOp::PushError(value) => {
          stack.push(EvalOperand::Value(FormulaValue::Error(*value)));
        }
        FormulaOp::PushReference(value) => stack.push(EvalOperand::Reference(value.clone())),
        FormulaOp::PushExternal(value) => stack.push(EvalOperand::ExternalReference(value.clone())),
        FormulaOp::PushName(value) => stack.push(EvalOperand::Name(value.clone())),
        FormulaOp::Unary(op) => {
          let Some(value) = stack.pop().and_then(|value| value.into_value(&evaluator)) else {
            return DirectEvaluation::Unsupported;
          };
          let Some(value) = evaluator.evaluate_unary_value(*op, value) else {
            return DirectEvaluation::Unsupported;
          };
          stack.push(EvalOperand::Value(value));
        }
        FormulaOp::Binary(op) => {
          let Some(right) = stack.pop() else {
            return DirectEvaluation::Unsupported;
          };
          if matches!(
            op,
            FormulaOperator::Range | FormulaOperator::Union | FormulaOperator::Intersection
          ) {
            let Some(left) = stack.pop() else {
              return DirectEvaluation::Unsupported;
            };
            let Some(value) = evaluate_reference_binary(&evaluator, *op, left, right) else {
              return DirectEvaluation::Unsupported;
            };
            stack.push(EvalOperand::Value(value));
            continue;
          }
          let Some(left) = stack.pop().and_then(|value| value.into_value(&evaluator)) else {
            return DirectEvaluation::Unsupported;
          };
          let right_evaluator = evaluator.with_current_value(left.clone());
          let Some(right) = right.into_value(&right_evaluator) else {
            return DirectEvaluation::Unsupported;
          };
          let Some(value) = evaluator.evaluate_binary_values(*op, left, right) else {
            return DirectEvaluation::Unsupported;
          };
          stack.push(EvalOperand::Value(value));
        }
        FormulaOp::Call {
          name,
          function,
          argc,
          control,
          ..
        } => {
          if control.is_some() || stack.len() < *argc {
            return DirectEvaluation::Unsupported;
          }
          let args = stack.split_off(stack.len() - *argc);
          let mut values = Vec::with_capacity(*argc);
          for arg in args {
            let Some(value) = arg.into_value(&evaluator) else {
              return DirectEvaluation::Unsupported;
            };
            values.push(value);
          }
          let Some(value) = crate::function::evaluate_function(
            &evaluator,
            *function,
            name,
            crate::function::FunctionArgs::new_values(&values),
          ) else {
            return DirectEvaluation::Unsupported;
          };
          stack.push(EvalOperand::Value(value));
        }
        FormulaOp::Array { row_lengths } => {
          let count = row_lengths.iter().sum::<usize>();
          if stack.len() < count {
            return DirectEvaluation::Unsupported;
          }
          let values = stack.split_off(stack.len() - count);
          let mut values = values.into_iter();
          let mut rows = Vec::with_capacity(row_lengths.len());
          for row_len in row_lengths {
            let mut row = Vec::with_capacity(*row_len);
            for value in values.by_ref().take(*row_len) {
              let Some(value) = value.into_value(&evaluator) else {
                return DirectEvaluation::Unsupported;
              };
              row.push(value);
            }
            rows.push(row);
          }
          stack.push(EvalOperand::Value(FormulaValue::Matrix(rows)));
        }
      }
    }
    match stack.pop() {
      Some(value) if stack.is_empty() => value
        .into_value(&evaluator)
        .map(DirectEvaluation::Value)
        .unwrap_or(DirectEvaluation::Unsupported),
      _ => DirectEvaluation::Unsupported,
    }
  }
}

enum DirectEvaluation<'doc> {
  Value(FormulaValue<'doc>),
  Unsupported,
}

fn evaluate_reference_binary<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  op: FormulaOperator,
  left: EvalOperand<'doc>,
  right: EvalOperand<'doc>,
) -> Option<FormulaValue<'doc>> {
  match op {
    FormulaOperator::Intersection => {
      let left_ranges = reference_ranges_from_operand(evaluator, left)?;
      let right_ranges = reference_ranges_from_operand(evaluator, right)?;
      evaluator.evaluate_intersection_ranges(left_ranges, right_ranges)
    }
    FormulaOperator::Range => {
      let left_ranges = reference_ranges_from_operand(evaluator, left)?;
      let right_ranges = reference_ranges_from_operand(evaluator, right)?;
      let ranges = evaluator.range_reference_ranges_from_ranges(left_ranges, right_ranges);
      evaluator.evaluate_range_ranges(ranges)
    }
    FormulaOperator::Union => {
      let left = left.into_value(evaluator)?;
      let right_evaluator = evaluator.with_current_value(left.clone());
      let right = right.into_value(&right_evaluator)?;
      let left_ranges = evaluator.reference_ranges_from_value(&left);
      let right_ranges = evaluator.reference_ranges_from_value(&right);
      evaluator
        .evaluate_union_ranges(left_ranges, right_ranges)
        .or_else(|| evaluator.evaluate_binary_values(op, left, right))
    }
    _ => None,
  }
}

fn reference_ranges_from_operand<'doc>(
  evaluator: &FormulaEvaluator<'_, 'doc>,
  operand: EvalOperand<'doc>,
) -> Option<Vec<QualifiedRange<'doc>>> {
  match operand {
    EvalOperand::Reference(reference) => Some(vec![reference]),
    EvalOperand::Value(value) => Some(evaluator.reference_ranges_from_value(&value)),
    EvalOperand::ExternalReference(_) | EvalOperand::Name(_) => {
      let value = operand.into_value(evaluator)?;
      Some(evaluator.reference_ranges_from_value(&value))
    }
  }
}

pub(crate) fn ast_from_code<'doc>(code: &FormulaCode<'doc>) -> Option<FormulaAst<'doc>> {
  let mut stack = Vec::with_capacity(code.ops.len());
  for op in &code.ops {
    match op {
      FormulaOp::PushBlank => stack.push(FormulaAst::Literal(FormulaValue::Blank)),
      FormulaOp::PushText(value) => {
        stack.push(FormulaAst::Literal(FormulaValue::String(value.clone())));
      }
      FormulaOp::PushNumber(value) => {
        stack.push(FormulaAst::Literal(FormulaValue::Number(*value)));
      }
      FormulaOp::PushBoolean(value) => {
        stack.push(FormulaAst::Literal(FormulaValue::Boolean(*value)));
      }
      FormulaOp::PushError(value) => {
        stack.push(FormulaAst::Literal(FormulaValue::Error(*value)));
      }
      FormulaOp::PushReference(value) => stack.push(FormulaAst::Reference(value.clone())),
      FormulaOp::PushExternal(value) => stack.push(FormulaAst::ExternalReference(value.clone())),
      FormulaOp::PushName(value) => stack.push(FormulaAst::Name(value.clone())),
      FormulaOp::Unary(value) => {
        let expr = stack.pop()?;
        stack.push(FormulaAst::Unary {
          op: *value,
          expr: Box::new(expr),
        });
      }
      FormulaOp::Binary(value) => {
        let right = stack.pop()?;
        let left = stack.pop()?;
        stack.push(FormulaAst::Binary {
          op: *value,
          left: Box::new(left),
          right: Box::new(right),
        });
      }
      FormulaOp::Call {
        name,
        function,
        argc,
        control,
        ..
      } => {
        let _ = control;
        if stack.len() < *argc {
          return None;
        }
        let args = stack.split_off(stack.len() - *argc);
        stack.push(FormulaAst::Function {
          name: name.clone(),
          function: *function,
          args,
        });
      }
      FormulaOp::Array { row_lengths } => {
        let count = row_lengths.iter().sum::<usize>();
        if stack.len() < count {
          return None;
        }
        let values = stack.split_off(stack.len() - count);
        let mut values = values.into_iter();
        let mut rows = Vec::with_capacity(row_lengths.len());
        for row_len in row_lengths {
          rows.push(values.by_ref().take(*row_len).collect());
        }
        stack.push(FormulaAst::Array(rows));
      }
    }
  }
  if stack.len() == 1 { stack.pop() } else { None }
}
