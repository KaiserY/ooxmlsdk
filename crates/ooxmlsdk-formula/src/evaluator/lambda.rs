use super::*;
use std::sync::Arc;

// Closures are interpreter operands, not worksheet cell values. Keep their
// program and lexical bindings alive across returned and higher-order calls.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LambdaClosure<'doc> {
  program: Arc<FormulaProgram>,
  body: FormulaExprId,
  parameters: Vec<(String, bool)>,
  locals: BTreeMap<String, EvalOperand<'doc>>,
  sheet: SheetId,
  cell: Option<CellAddress>,
  grammar: FormulaGrammar,
}

// Bound interpreter recursion independently of Excel's operand-stack limit.
const MAX_CALL_DEPTH: usize = 32;

fn error<'doc>(value: FormulaErrorValue) -> EvalOperand<'doc> {
  EvalOperand::Value(FormulaValue::Error(value))
}

fn excel(evaluator: &FormulaEvaluator<'_, '_>) -> bool {
  matches!(
    evaluator.grammar,
    FormulaGrammar::ExcelA1 | FormulaGrammar::ExcelR1C1
  )
}

fn local_key(name: &str) -> String {
  let upper = name.to_ascii_uppercase();
  upper
    .strip_prefix("_XLPM.")
    .or_else(|| upper.strip_prefix("_XLOP."))
    .unwrap_or(&upper)
    .to_string()
}

pub(super) fn evaluate_argument<'doc>(
  arg: EvalArg<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  let value = engine::evaluate_node_operand(arg.program, arg.borrowed_source, arg.id, evaluator)?;
  resolve(value, evaluator)
}

fn resolve<'doc>(
  value: EvalOperand<'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  match value {
    EvalOperand::Name(name) => resolve_name_operand(name, evaluator),
    EvalOperand::ExternalReference(reference) => evaluator
      .evaluate_external_reference(&reference)
      .map(EvalOperand::Value),
    value => Some(value),
  }
}

pub(super) fn resolve_name_operand<'doc>(
  name: Cow<'doc, str>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  if let Some(value) = evaluator.locals.get(&local_key(&name)) {
    return Some(value.clone());
  }
  if excel(evaluator) {
    if let Some(builtin) = name.to_ascii_uppercase().strip_prefix("_XLETA.")
      && crate::function::resolve_function_name(builtin).is_some()
    {
      return Some(EvalOperand::Builtin(builtin.to_string()));
    }
    if let Some((scope, defined_name)) = evaluator
      .book
      .defined_name_scope(Some(evaluator.current_sheet), &name)
      && let Some(formula) = evaluator.book.defined_name_formula(scope, defined_name)
    {
      if evaluator.call_depth >= MAX_CALL_DEPTH {
        return Some(error(FormulaErrorValue::Num));
      }
      let mut scoped = evaluator.with_current_sheet(scope.unwrap_or(evaluator.current_sheet));
      scoped.locals.clear();
      scoped.call_depth += 1;
      let parsed = parse_formula(
        scoped.current_sheet,
        Cow::Owned(formula.to_string()),
        scoped.grammar,
      );
      if !parsed.unsupported.is_empty() {
        return evaluator.evaluate_name(&name).map(EvalOperand::Value);
      }
      let program = parsed.program.as_ref()?;
      return evaluate_argument(
        EvalArg {
          program,
          id: program.root?,
          borrowed_source: None,
        },
        &scoped,
      );
    }
  }
  evaluator.evaluate_name(&name).map(EvalOperand::Value)
}

// Some(None) is a recognized but unevaluated function; None lets the ordinary
// builtin/UDF dispatcher handle the name. Builtins win over same-named locals.
pub(super) fn evaluate_function_operand<'doc>(
  name: &str,
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<Option<EvalOperand<'doc>>> {
  if !excel(evaluator) {
    return None;
  }
  let upper = name.to_ascii_uppercase();
  let canonical = upper.strip_prefix("_XLFN.").unwrap_or(&upper);
  match canonical {
    "LAMBDA" => return Some(make_closure(args, evaluator)),
    "ISOMITTED" => return Some(is_omitted(args, evaluator)),
    "IF" => {
      let condition = evaluate_arg_direct(args.get(0)?, evaluator)?;
      let scalar_condition = match &condition {
        FormulaValue::Reference(reference) => {
          reference.range.start == reference.range.end && reference.end_sheet_name.is_none()
        }
        FormulaValue::Matrix(rows) => rows.len() == 1 && rows[0].len() == 1,
        FormulaValue::RefList(_) => false,
        _ => true,
      };
      if !evaluator.array_context || scalar_condition {
        if let FormulaValue::Error(value) = evaluator.first_value(&condition) {
          return Some(Some(error(value)));
        }
        let selected = usize::from(!evaluator.truthy(&condition)) + 1;
        return Some(match args.get(selected) {
          Some(arg) => evaluate_argument(arg, evaluator),
          None => Some(EvalOperand::Value(FormulaValue::Boolean(selected == 1))),
        });
      }
    }
    _ => {}
  }
  if crate::function::resolve_function_name(name).is_none()
    && (evaluator.locals.contains_key(&local_key(name))
      || evaluator
        .book
        .defined_name_formula(Some(evaluator.current_sheet), name)
        .is_some())
  {
    return Some(invoke(
      EvalOperand::Name(Cow::Owned(name.to_string())),
      args,
      evaluator,
    ));
  }
  None
}

fn make_closure<'doc>(
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  if args.len() == 0 || args.len() > 254 {
    return Some(error(FormulaErrorValue::Value));
  }
  let body = args.get(args.len() - 1)?;
  let mut parameters = Vec::new();
  let mut optional_seen = false;
  for index in 0..args.len() - 1 {
    let arg = args.get(index)?;
    let FormulaNodeKind::Reference(FormulaReference::Named(reference)) =
      &arg.program.node(arg.id)?.kind
    else {
      return Some(error(FormulaErrorValue::Value));
    };
    if reference.scope != crate::program::FormulaNameScope::Workbook {
      return Some(error(FormulaErrorValue::Value));
    }
    let name = arg.program.symbols.get(reference.name)?;
    let optional = name.to_ascii_uppercase().starts_with("_XLOP.");
    let key = local_key(name);
    if key.is_empty()
      || parameters.iter().any(|(prior, _)| *prior == key)
      || (optional_seen && !optional)
    {
      return Some(error(FormulaErrorValue::Value));
    }
    optional_seen |= optional;
    parameters.push((key, optional));
  }
  Some(EvalOperand::Lambda(Arc::new(LambdaClosure {
    program: Arc::new(body.program.clone()),
    body: body.id,
    parameters,
    locals: evaluator.locals.clone(),
    sheet: evaluator.current_sheet,
    cell: evaluator.current_cell,
    grammar: evaluator.grammar,
  })))
}

fn is_omitted<'doc>(
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  if args.len() != 1 {
    return Some(error(FormulaErrorValue::Value));
  }
  let arg = args.get(0)?;
  let omitted = matches!(
    arg.program.node(arg.id)?.kind,
    FormulaNodeKind::Blank | FormulaNodeKind::MissingArgument
  ) || matches!(evaluate_argument(arg, evaluator)?, EvalOperand::Omitted);
  Some(EvalOperand::Value(FormulaValue::Boolean(omitted)))
}

pub(super) fn invoke<'doc>(
  callee: EvalOperand<'doc>,
  args: EvalArgs<'_, 'doc>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  let callee = resolve(callee, evaluator)?;
  let mut values = Vec::with_capacity(args.len());
  for index in 0..args.len() {
    let arg = args.get(index)?;
    values.push(
      if matches!(
        arg.program.node(arg.id)?.kind,
        FormulaNodeKind::Blank | FormulaNodeKind::MissingArgument
      ) {
        EvalOperand::Omitted
      } else {
        evaluate_argument(arg, evaluator)?
      },
    );
  }
  invoke_values(callee, values, evaluator)
}

fn invoke_values<'doc>(
  callee: EvalOperand<'doc>,
  values: Vec<EvalOperand<'doc>>,
  evaluator: &FormulaEvaluator<'_, 'doc>,
) -> Option<EvalOperand<'doc>> {
  if evaluator.call_depth >= MAX_CALL_DEPTH {
    return Some(error(FormulaErrorValue::Num));
  }
  match callee {
    EvalOperand::Lambda(closure) => {
      let required = closure
        .parameters
        .iter()
        .take_while(|(_, optional)| !optional)
        .count();
      if values.len() < required || values.len() > closure.parameters.len() {
        return Some(error(FormulaErrorValue::Value));
      }
      let mut scoped = evaluator.clone();
      scoped.call_depth += 1;
      scoped.current_sheet = closure.sheet;
      scoped.current_cell = closure.cell;
      scoped.grammar = closure.grammar;
      scoped.array_context = true;
      scoped.locals = closure.locals.clone();
      for ((name, _), value) in closure.parameters.iter().zip(
        values
          .into_iter()
          .chain(std::iter::repeat(EvalOperand::Omitted)),
      ) {
        scoped.locals.insert(name.clone(), value);
      }
      evaluate_argument(
        EvalArg {
          program: &closure.program,
          id: closure.body,
          borrowed_source: None,
        },
        &scoped,
      )
    }
    EvalOperand::Builtin(name) => {
      // Feed bound operands through the existing lazy builtin interface, so
      // reference/array coercion stays identical to an ordinary function call.
      let mut scoped = evaluator.clone();
      scoped.call_depth += 1;
      let mut names = Vec::with_capacity(values.len());
      for (index, value) in values.into_iter().enumerate() {
        let key = format!("__lambda_arg_{index}");
        scoped.locals.insert(key.to_ascii_uppercase(), value);
        names.push(key);
      }
      let source = format!("{name}({})", names.join(","));
      let parsed = parse_formula(scoped.current_sheet, Cow::Owned(source), scoped.grammar);
      let program = parsed.program.as_ref()?;
      evaluate_argument(
        EvalArg {
          program,
          id: program.root?,
          borrowed_source: None,
        },
        &scoped,
      )
    }
    EvalOperand::Value(FormulaValue::Error(value)) => Some(error(value)),
    _ => Some(error(FormulaErrorValue::Value)),
  }
}
