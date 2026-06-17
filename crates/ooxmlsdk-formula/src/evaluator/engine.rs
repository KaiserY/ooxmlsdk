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
