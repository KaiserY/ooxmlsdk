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
    return Some(FormulaValue::Error(FormulaErrorValue::Error));
  }
  if let Some(value) =
    book.evaluate_special_formula_text(current_sheet, current_cell, formula.source.as_ref())
  {
    return Some(value);
  }
  if formula.grammar == FormulaGrammar::OpenFormula
    && formula_contains_unquoted_getting_data(formula.source.as_ref())
  {
    return Some(FormulaValue::Error(FormulaErrorValue::PairExpected));
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
  .evaluate_program(formula.program.as_ref()?, program_borrowed_source(formula))
}

fn formula_contains_unquoted_getting_data(formula: &str) -> bool {
  let mut quoted = false;
  let mut index = 0;
  while index < formula.len() {
    let rest = &formula[index..];
    let ch = rest.chars().next().expect("valid formula char");
    match ch {
      '"' => {
        index += ch.len_utf8();
        if quoted && formula[index..].starts_with('"') {
          index += '"'.len_utf8();
        } else {
          quoted = !quoted;
        }
      }
      _ if !quoted
        && rest
          .get(.."#getting_data".len())
          .is_some_and(|value| value.eq_ignore_ascii_case("#getting_data")) =>
      {
        return true;
      }
      _ => {
        index += ch.len_utf8();
      }
    }
  }
  false
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
  pub(crate) fn evaluate_program(
    &self,
    program: &FormulaProgram,
    borrowed_source: Option<&'doc str>,
  ) -> Option<FormulaValue<'doc>> {
    let evaluator = self.compat_evaluator();
    evaluate_program_with_context(program, borrowed_source, &evaluator)
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

fn program_borrowed_source<'doc>(formula: &ParsedFormula<'doc>) -> Option<&'doc str> {
  match &formula.source {
    Cow::Borrowed(source) => Some(*source),
    Cow::Owned(_) => None,
  }
}

pub(crate) fn evaluate_program_with_context<'doc>(
  program: &FormulaProgram,
  borrowed_source: Option<&'doc str>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  let root = program.root?;
  match evaluate_node_value(program, borrowed_source, root, evaluator) {
    DirectEvaluation::Value(value) => Some(value),
    DirectEvaluation::Unsupported => None,
  }
}

enum DirectEvaluation<'doc> {
  Value(FormulaValue<'doc>),
  Unsupported,
}

fn evaluate_node_value<'doc>(
  program: &FormulaProgram,
  borrowed_source: Option<&'doc str>,
  id: FormulaExprId,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> DirectEvaluation<'doc> {
  evaluate_node_operand(program, borrowed_source, id, evaluator)
    .and_then(|value| value.into_value(evaluator))
    .map(DirectEvaluation::Value)
    .unwrap_or(DirectEvaluation::Unsupported)
}

pub(crate) fn evaluate_arg_direct<'doc>(
  arg: EvalArg<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<FormulaValue<'doc>> {
  match evaluate_node_value(arg.program, arg.borrowed_source, arg.id, evaluator) {
    DirectEvaluation::Value(value) => Some(value),
    DirectEvaluation::Unsupported => None,
  }
}

fn evaluate_node_operand<'doc>(
  program: &FormulaProgram,
  borrowed_source: Option<&'doc str>,
  id: FormulaExprId,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  let node = program.node(id)?;
  match &node.kind {
    FormulaNodeKind::Blank | FormulaNodeKind::MissingArgument => {
      Some(EvalOperand::Value(FormulaValue::Blank))
    }
    FormulaNodeKind::Text(value) => Some(EvalOperand::Value(FormulaValue::String(
      crate::code::program_text_cow(program, borrowed_source, node.span, *value)?,
    ))),
    FormulaNodeKind::Number(value) => Some(EvalOperand::Value(FormulaValue::Number(match value {
      FormulaNumberLiteral::Integer(value) => *value as f64,
      FormulaNumberLiteral::Number(value) => *value,
    }))),
    FormulaNodeKind::Boolean(value) => Some(EvalOperand::Value(FormulaValue::Boolean(*value))),
    FormulaNodeKind::Error(value) => Some(EvalOperand::Value(FormulaValue::Error(*value))),
    FormulaNodeKind::Reference(reference) => {
      program_reference_operand(program, borrowed_source, node.span, reference, evaluator)
    }
    FormulaNodeKind::Unary { op, expr } => {
      let value =
        evaluate_node_operand(program, borrowed_source, *expr, evaluator)?.into_value(evaluator)?;
      let value = evaluator.evaluate_unary_value(*op, value)?;
      Some(EvalOperand::Value(value))
    }
    FormulaNodeKind::Binary { op, left, right } => {
      let right = evaluate_node_operand(program, borrowed_source, *right, evaluator)?;
      if matches!(
        op,
        FormulaOperator::Range | FormulaOperator::Union | FormulaOperator::Intersection
      ) {
        let left = evaluate_node_operand(program, borrowed_source, *left, evaluator)?;
        let value = evaluate_reference_binary(evaluator, *op, left, right)?;
        return Some(EvalOperand::Value(value));
      }
      let left =
        evaluate_node_operand(program, borrowed_source, *left, evaluator)?.into_value(evaluator)?;
      let right_evaluator = evaluator.with_current_value(left.clone());
      let right = right.into_value(&right_evaluator)?;
      let value = evaluator.evaluate_binary_values(*op, left, right)?;
      Some(EvalOperand::Value(value))
    }
    FormulaNodeKind::Function { name, args } => {
      let args = program.args(*args)?;
      let name = crate::code::function_name_cow(program, borrowed_source, node.span, *name)?;
      let function = crate::function::resolve_function_name(name.as_ref());
      let eval_args = EvalArgs::new(program, args, borrowed_source);
      let value = match crate::code::control_for_function(function) {
        Some(control) => evaluate_control_call(control, eval_args, evaluator),
        None => crate::function::evaluate_function(
          evaluator,
          function,
          &name,
          crate::function::FunctionArgs::new_lazy(eval_args),
        ),
      }?;
      Some(EvalOperand::Value(value))
    }
    FormulaNodeKind::Array(span) => {
      let elements = program.array_elements(*span)?;
      let cols = usize::from(span.cols);
      let rows_len = usize::from(span.rows);
      let mut rows = Vec::with_capacity(rows_len);
      for row_elements in elements.chunks(cols) {
        let mut row = Vec::with_capacity(row_elements.len());
        for element in row_elements {
          let value = evaluate_node_operand(program, borrowed_source, *element, evaluator)?
            .into_value(evaluator)?;
          row.push(value);
        }
        rows.push(row);
      }
      Some(EvalOperand::Value(FormulaValue::Matrix(rows)))
    }
    FormulaNodeKind::Call { .. } | FormulaNodeKind::Unsupported(_) => None,
  }
}

fn program_reference_operand<'doc>(
  program: &FormulaProgram,
  borrowed_source: Option<&'doc str>,
  span: Option<crate::program::SourceSpan>,
  reference: &FormulaReference,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  match reference {
    FormulaReference::Cell(reference) => {
      if let FormulaSheetReference::External { book, sheet } = reference.target.sheet {
        return Some(EvalOperand::ExternalReference(
          crate::code::external_reference_from_cell(
            program,
            book,
            sheet,
            reference.target,
            reference.flags,
          )?,
        ));
      }
      Some(EvalOperand::Reference(
        crate::code::qualified_range_from_points(
          evaluator.current_sheet,
          program,
          reference.target,
          reference.target,
          reference.flags,
        )?,
      ))
    }
    FormulaReference::Range(reference) => {
      if let FormulaSheetReference::External { book, sheet } = reference.start.sheet {
        return Some(EvalOperand::ExternalReference(
          crate::code::external_reference_from_range(
            program,
            book,
            sheet,
            reference.start,
            reference.end,
            reference.flags,
          )?,
        ));
      }
      Some(EvalOperand::Reference(
        crate::code::qualified_range_from_points(
          evaluator.current_sheet,
          program,
          reference.start,
          reference.end,
          reference.flags,
        )?,
      ))
    }
    FormulaReference::Named(reference) => Some(EvalOperand::Name(crate::code::scoped_name_cow(
      program,
      borrowed_source,
      span,
      &reference.scope,
      reference.name,
    )?)),
    FormulaReference::Structured(reference) => Some(EvalOperand::Name(Cow::Owned(
      crate::code::structured_reference_text(program, reference)?,
    ))),
    FormulaReference::ExternalName(reference) => {
      Some(EvalOperand::ExternalReference(ExternalReferenceId {
        book: Some(Cow::Owned(program.symbols.get(reference.book)?.to_string())),
        sheet: match reference.sheet {
          Some(sheet) => Some(Cow::Owned(crate::code::sheet_name_text(program, sheet)?)),
          None => None,
        },
        name: Some(Cow::Owned(program.symbols.get(reference.name)?.to_string())),
      }))
    }
    FormulaReference::Deleted(_) => Some(EvalOperand::Value(FormulaValue::Error(
      FormulaErrorValue::Ref,
    ))),
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
    evaluate_arg_direct(arg, evaluator)
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
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let value_arg = args.get(0)?;
  if is_missing_arg(value_arg) {
    return Some(FormulaValue::Error(FormulaErrorValue::Parameter));
  }
  let value = evaluate_arg_direct(value_arg, evaluator)
    .unwrap_or(FormulaValue::Error(FormulaErrorValue::Error));
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
  let index_value = evaluator.scalar_binary_operand(evaluate_arg_direct(args.get(0)?, evaluator)?);
  if let FormulaValue::Error(error) = index_value {
    return Some(FormulaValue::Error(error));
  }
  let Some(index_value) = evaluator.number(&index_value) else {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  };
  let index_value = index_value.floor();
  if !index_value.is_finite() || index_value < 1.0 || index_value > usize::MAX as f64 {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let index = index_value as usize;
  if index == 0 || index >= args.len() {
    return Some(FormulaValue::Error(FormulaErrorValue::Value));
  }
  let selected = args.get(index)?;
  if is_missing_arg(selected) {
    Some(FormulaValue::Blank)
  } else {
    evaluate_arg_direct(selected, evaluator)
  }
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
      (FormulaValue::Boolean(_), FormulaValue::Number(_))
      | (FormulaValue::Number(_), FormulaValue::Boolean(_)) => evaluator
        .number(&selector)
        .zip(evaluator.number(&candidate))
        .is_some_and(|(left, right)| left == right),
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

fn is_missing_arg(arg: EvalArg<'_, '_>) -> bool {
  matches!(
    arg.program.node(arg.id).map(|node| &node.kind),
    Some(FormulaNodeKind::Blank | FormulaNodeKind::MissingArgument)
  )
}

fn value_error_matches(value: &FormulaValue<'_>, na_only: bool) -> bool {
  matches!(
    value,
    FormulaValue::Error(error) if !na_only || *error == FormulaErrorValue::NA
  )
}

fn let_binding_name_from_arg(arg: EvalArg<'_, '_>) -> Option<String> {
  let node = arg.program.node(arg.id)?;
  let FormulaNodeKind::Reference(FormulaReference::Named(reference)) = &node.kind else {
    return None;
  };
  let name = arg.program.symbols.get(reference.name)?;
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
