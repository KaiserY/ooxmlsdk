use super::*;

pub(crate) struct FormulaEvaluatorEngine<'book, 'engine, 'doc> {
  pub(crate) book: &'book FormulaEvaluationBook<'doc>,
  pub(crate) engine: &'engine CalcEngine,
  pub(crate) current_sheet: SheetId,
  pub(crate) current_cell: Option<CellAddress>,
  pub(crate) grammar: FormulaGrammar,
  pub(crate) array_context: bool,
  pub(crate) calc_a1_indirect_bang_reference: bool,
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
    calc_a1_indirect_bang_reference: formula.grammar == FormulaGrammar::CalcA1
      && !formula.source.trim_start().starts_with('=')
      && formula.source.to_ascii_uppercase().contains("INDIRECT")
      && formula.source.contains('!')
      && !formula.source.to_ascii_uppercase().contains(";0)")
      && !formula.source.to_ascii_uppercase().contains(",0)"),
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
    let evaluator = self.compat_evaluator();
    evaluate_code_with_context(code, &evaluator)
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
      calc_a1_indirect_bang_reference: self.calc_a1_indirect_bang_reference,
    }
  }
}

pub(crate) fn evaluate_code_with_context<'doc>(
  code: &FormulaCode<'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  match evaluate_code_direct(code, evaluator) {
    DirectEvaluation::Value(value) => Some(value),
    DirectEvaluation::Unsupported => None,
  }
}

fn evaluate_code_direct<'doc>(
  code: &FormulaCode<'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> DirectEvaluation<'doc> {
  evaluate_ops_range(
    &code.ops,
    FormulaArgRange {
      start: 0,
      end: code.ops.len(),
    },
    evaluator,
  )
}

enum DirectEvaluation<'doc> {
  Value(FormulaValue<'doc>),
  Unsupported,
}

fn evaluate_ops_range<'doc>(
  ops: &[FormulaOp<'doc>],
  range: FormulaArgRange,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> DirectEvaluation<'doc> {
  if range.start > range.end || range.end > ops.len() {
    return DirectEvaluation::Unsupported;
  }
  let mut stack = Vec::with_capacity(range.end - range.start);
  let mut index = range.start;
  while index < range.end {
    if let Some(call_index) = call_starting_at(ops, index, range.end) {
      let FormulaOp::Call {
        name,
        function,
        arg_ranges,
        control,
        ..
      } = &ops[call_index]
      else {
        return DirectEvaluation::Unsupported;
      };
      let args = EvalArgs::new(ops, arg_ranges);
      let Some(value) = (match control {
        Some(control) => evaluate_control_call(*control, args, evaluator),
        None => crate::function::evaluate_function(
          evaluator,
          *function,
          name,
          crate::function::FunctionArgs::new_lazy(args),
        ),
      }) else {
        return DirectEvaluation::Unsupported;
      };
      stack.push(EvalOperand::Value(value));
      index = call_index + 1;
      continue;
    }
    let op = &ops[index];
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
        let Some(value) = stack.pop().and_then(|value| value.into_value(evaluator)) else {
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
          let Some(value) = evaluate_reference_binary(evaluator, *op, left, right) else {
            return DirectEvaluation::Unsupported;
          };
          stack.push(EvalOperand::Value(value));
          index += 1;
          continue;
        }
        let Some(left) = stack.pop().and_then(|value| value.into_value(evaluator)) else {
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
      FormulaOp::Call { .. } => return DirectEvaluation::Unsupported,
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
            let Some(value) = value.into_value(evaluator) else {
              return DirectEvaluation::Unsupported;
            };
            row.push(value);
          }
          rows.push(row);
        }
        stack.push(EvalOperand::Value(FormulaValue::Matrix(rows)));
      }
    }
    index += 1;
  }
  match stack.pop() {
    Some(value) if stack.is_empty() => value
      .into_value(evaluator)
      .map(DirectEvaluation::Value)
      .unwrap_or(DirectEvaluation::Unsupported),
    _ => DirectEvaluation::Unsupported,
  }
}

pub(crate) fn evaluate_arg_direct<'doc>(
  arg: EvalArg<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  match evaluate_ops_range(arg.ops, arg.range, evaluator) {
    DirectEvaluation::Value(value) => Some(value),
    DirectEvaluation::Unsupported => None,
  }
}

fn evaluate_control_call<'doc>(
  control: FormulaControlOp,
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  match control {
    FormulaControlOp::IfJump => evaluate_if_control(args, evaluator),
    FormulaControlOp::IfErrorJump => evaluate_if_error_control(args, evaluator, false),
    FormulaControlOp::IfNaJump => evaluate_if_error_control(args, evaluator, true),
    FormulaControlOp::ChooseJump => evaluate_choose_control(args, evaluator),
    FormulaControlOp::IfsJump => evaluate_ifs_control(args, evaluator),
    FormulaControlOp::SwitchJump => evaluate_switch_control(args, evaluator),
    FormulaControlOp::LetBind => evaluate_let_control(args, evaluator),
  }
}

fn evaluate_if_control<'doc>(
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let condition = evaluate_arg_direct(args.get(0)?, evaluator)?;
  let if_value = |arg: Option<EvalArg<'_, 'doc>>, default: FormulaValue<'doc>| {
    let Some(arg) = arg else {
      return Some(default);
    };
    Some(match evaluate_arg_direct(arg, evaluator)? {
      FormulaValue::Blank => FormulaValue::Number(0.0),
      value => value,
    })
  };
  if evaluator.array_context
    && matches!(
      condition,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    )
  {
    let true_value = if_value(args.get(1), FormulaValue::Boolean(true))?;
    let false_value = if_value(args.get(2), FormulaValue::Boolean(false))?;
    return evaluator.map_if_values(condition, true_value, false_value);
  }
  if let FormulaValue::Error(error) = evaluator.first_value(&condition) {
    return Some(FormulaValue::Error(error));
  }
  if evaluator.truthy(&condition) {
    if_value(args.get(1), FormulaValue::Boolean(true))
  } else {
    if_value(args.get(2), FormulaValue::Boolean(false))
  }
}

fn evaluate_if_error_control<'doc>(
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
  na_only: bool,
) -> Option<FormulaValue<'doc>> {
  if args.len() != 2 {
    return Some(FormulaValue::Error(FormulaErrorValue::Unknown));
  }
  let value_arg = args.get(0)?;
  if is_missing_arg(value_arg) {
    return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
  }
  let value = evaluate_arg_direct(value_arg, evaluator)
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::Unknown));
  if evaluator.array_context
    && matches!(
      value,
      FormulaValue::Reference(_) | FormulaValue::RefList(_) | FormulaValue::Matrix(_)
    )
  {
    let fallback = evaluate_arg_direct(args.get(1)?, evaluator)?;
    return evaluator.map_if_error_values(value, fallback, na_only);
  }
  let value = evaluator.scalar_value(value);
  if value_error_matches(&value, na_only) {
    evaluate_arg_direct(args.get(1)?, evaluator)
  } else {
    Some(value)
  }
}

fn evaluate_choose_control<'doc>(
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let index = evaluator
    .number(&evaluate_arg_direct(args.get(0)?, evaluator)?)?
    .floor() as usize;
  if index == 0 || index >= args.len() {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  evaluate_arg_direct(args.get(index)?, evaluator)
}

fn evaluate_ifs_control<'doc>(
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.len() < 2 || !args.len().is_multiple_of(2) {
    return None;
  }
  let mut index = 0;
  while index < args.len() {
    let condition = evaluate_arg_direct(args.get(index)?, evaluator)?;
    if let FormulaValue::Error(error) = condition {
      return Some(FormulaValue::Error(error));
    }
    if evaluator.truthy(&condition) {
      return evaluate_arg_direct(args.get(index + 1)?, evaluator);
    }
    index += 2;
  }
  Some(FormulaValue::Error(FormulaErrorValue::NA))
}

fn evaluate_switch_control<'doc>(
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.len() < 3 {
    return None;
  }
  let selector = evaluator.scalar_value(evaluate_arg_direct(args.get(0)?, evaluator)?);
  if let FormulaValue::Error(error) = &selector {
    return Some(FormulaValue::Error(*error));
  }
  let pairs_len = if args.len().is_multiple_of(2) {
    args.len() - 2
  } else {
    args.len() - 1
  };
  let mut index = 1;
  while index <= pairs_len {
    let candidate = evaluator.scalar_value(evaluate_arg_direct(args.get(index)?, evaluator)?);
    if let FormulaValue::Error(error) = &candidate {
      return Some(FormulaValue::Error(*error));
    }
    let matches = match (&selector, &candidate) {
      (FormulaValue::String(left), FormulaValue::String(right)) => left.eq_ignore_ascii_case(right),
      _ => evaluator.compare(&selector, &candidate, FormulaOperator::Equal),
    };
    if matches {
      return Some(evaluator.scalar_value(evaluate_arg_direct(args.get(index + 1)?, evaluator)?));
    }
    index += 2;
  }
  if args.len().is_multiple_of(2) {
    Some(evaluator.scalar_value(evaluate_arg_direct(args.get(args.len() - 1)?, evaluator)?))
  } else {
    Some(FormulaValue::Error(FormulaErrorValue::NA))
  }
}

fn evaluate_let_control<'doc>(
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  if args.len() < 3 || args.len().is_multiple_of(2) {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let mut local_evaluator = FormulaEvaluator {
    book: evaluator.book,
    engine: evaluator.engine,
    current_sheet: evaluator.current_sheet,
    current_cell: evaluator.current_cell,
    grammar: evaluator.grammar,
    locals: evaluator.locals.clone(),
    array_context: evaluator.array_context,
    current_value: evaluator.current_value.clone(),
    calc_a1_indirect_bang_reference: evaluator.calc_a1_indirect_bang_reference,
  };
  let mut local_names = BTreeMap::new();
  let mut index = 0;
  while index + 2 < args.len() {
    let name = let_binding_name_from_arg(args.get(index)?)?;
    if name.is_empty() || local_names.insert(name.clone(), ()).is_some() {
      return Some(FormulaValue::Error(FormulaErrorValue::Value));
    }
    let value = evaluate_arg_direct(args.get(index + 1)?, &local_evaluator)?.into_owned();
    local_evaluator.locals.insert(name, value);
    index += 2;
  }
  evaluate_arg_direct(args.get(args.len() - 1)?, &local_evaluator)
}

fn call_starting_at<'doc>(ops: &[FormulaOp<'doc>], index: usize, end: usize) -> Option<usize> {
  let mut selected = None;
  for (call_index, op) in ops.iter().enumerate().take(end).skip(index) {
    let FormulaOp::Call {
      arg_ranges, argc, ..
    } = op
    else {
      continue;
    };
    if (*argc == 0 && call_index == index)
      || (arg_ranges
        .first()
        .is_some_and(|range| range.start == index && range.end <= call_index)
        && arg_ranges
          .iter()
          .all(|range| range.start <= range.end && range.end <= call_index))
    {
      selected = Some(call_index);
    }
  }
  selected
}

fn is_missing_arg(arg: EvalArg<'_, '_>) -> bool {
  let range = arg.range;
  range.end == range.start + 1 && matches!(arg.ops.get(range.start), Some(FormulaOp::PushBlank))
}

fn value_error_matches(value: &FormulaValue<'_>, na_only: bool) -> bool {
  matches!(
    value,
    FormulaValue::Error(error) if !na_only || *error == FormulaErrorValue::NA
  )
}

fn let_binding_name_from_arg(arg: EvalArg<'_, '_>) -> Option<String> {
  let range = arg.range;
  if range.end != range.start + 1 {
    return None;
  }
  let Some(FormulaOp::PushName(name)) = arg.ops.get(range.start) else {
    return None;
  };
  Some(name.trim_start_matches("_xlpm.").to_ascii_uppercase())
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
