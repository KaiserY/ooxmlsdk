use std::borrow::Cow;

use crate::dependency::ExternalReferenceId;
use crate::function::FormulaFunctionId;
use crate::program::{
  FormulaAddressFlags, FormulaFunctionName, FormulaNameScope, FormulaProgram,
  FormulaReferenceFlags, FormulaReferencePoint, FormulaSheetName, FormulaSheetRange,
  FormulaSheetReference, FormulaStructuredReference, FormulaStructuredReferenceItem,
  FormulaStructuredReferencePart, FormulaStructuredReferenceSpecifier,
};
use crate::{
  AddressFlags, CellAddress, CellRange, FormulaErrorValue, FormulaOperator, FormulaValue,
  QualifiedRange, SheetId, SheetName, parser,
};

pub(crate) fn function_name_cow<'doc>(
  program: &FormulaProgram,
  borrowed_source: Option<&'doc str>,
  span: Option<crate::program::SourceSpan>,
  name: FormulaFunctionName,
) -> Option<Cow<'doc, str>> {
  match name {
    FormulaFunctionName::Builtin(id) => Some(Cow::Owned(format!("BUILTIN{}", id.0))),
    FormulaFunctionName::External(id) | FormulaFunctionName::Unknown(id) => {
      program_symbol_cow(program, borrowed_source, span, id)
    }
  }
}

pub(crate) fn qualified_range_from_points<'doc>(
  current_sheet: SheetId,
  program: &FormulaProgram,
  start: FormulaReferencePoint,
  end: FormulaReferencePoint,
  reference_flags: FormulaReferenceFlags,
) -> Option<QualifiedRange<'doc>> {
  let (sheet, sheet_name, end_sheet_name) =
    qualified_sheet_parts(current_sheet, program, start.sheet, end.sheet)?;
  Some(QualifiedRange {
    sheet,
    sheet_name: sheet_name.map(|name| SheetName(Cow::Owned(name))),
    end_sheet_name: end_sheet_name.map(|name| SheetName(Cow::Owned(name))),
    range: CellRange {
      start: start.address,
      end: end.address,
    },
    start_flags: address_flags(start.flags, reference_flags),
    end_flags: address_flags(end.flags, reference_flags),
  })
}

pub(crate) fn qualified_sheet_parts(
  current_sheet: SheetId,
  program: &FormulaProgram,
  start: FormulaSheetReference,
  end: FormulaSheetReference,
) -> Option<(SheetId, Option<String>, Option<String>)> {
  match (start, end) {
    (FormulaSheetReference::Current, FormulaSheetReference::Current) => {
      Some((current_sheet, None, None))
    }
    (
      FormulaSheetReference::Local(FormulaSheetRange::Sheet(start)),
      FormulaSheetReference::Local(FormulaSheetRange::Sheet(end)),
    ) if start == end => {
      let (sheet, name) = qualified_sheet_name(program, start)?;
      Some((sheet, name, None))
    }
    (
      FormulaSheetReference::Local(FormulaSheetRange::Range { start, end }),
      FormulaSheetReference::Local(_),
    )
    | (
      FormulaSheetReference::Local(_),
      FormulaSheetReference::Local(FormulaSheetRange::Range { start, end }),
    ) => {
      let (sheet, start) = qualified_sheet_name(program, start)?;
      let (_, end) = qualified_sheet_name(program, end)?;
      Some((sheet, start, end))
    }
    (
      FormulaSheetReference::Local(FormulaSheetRange::Sheet(start)),
      FormulaSheetReference::Local(FormulaSheetRange::Sheet(end)),
    ) => {
      let (sheet, start) = qualified_sheet_name(program, start)?;
      let (_, end) = qualified_sheet_name(program, end)?;
      Some((sheet, start, end))
    }
    (
      FormulaSheetReference::Local(FormulaSheetRange::Sheet(start)),
      FormulaSheetReference::Current,
    )
    | (
      FormulaSheetReference::Current,
      FormulaSheetReference::Local(FormulaSheetRange::Sheet(start)),
    ) => {
      let (sheet, name) = qualified_sheet_name(program, start)?;
      Some((sheet, name, None))
    }
    _ => None,
  }
}

pub(crate) fn qualified_sheet_name(
  program: &FormulaProgram,
  sheet: FormulaSheetName,
) -> Option<(SheetId, Option<String>)> {
  match sheet {
    FormulaSheetName::Id(sheet) => Some((sheet, None)),
    FormulaSheetName::Name(name) => Some((
      SheetId::default(),
      Some(program.symbols.get(name)?.to_string()),
    )),
  }
}

pub(crate) fn address_flags(
  flags: FormulaAddressFlags,
  reference_flags: FormulaReferenceFlags,
) -> AddressFlags {
  AddressFlags {
    absolute_column: flags.contains(FormulaAddressFlags::COL_ABS),
    absolute_row: flags.contains(FormulaAddressFlags::ROW_ABS),
    whole_column: reference_flags.contains(FormulaReferenceFlags::WHOLE_COLUMN),
    whole_row: reference_flags.contains(FormulaReferenceFlags::WHOLE_ROW),
  }
}

pub(crate) fn external_reference_from_cell<'doc>(
  program: &FormulaProgram,
  book: crate::symbol::FormulaSymbolId,
  sheet: Option<FormulaSheetRange>,
  point: FormulaReferencePoint,
  flags: FormulaReferenceFlags,
) -> Option<ExternalReferenceId<'doc>> {
  let mut name = String::new();
  push_cell_reference_text(point.address, point.flags, flags, &mut name);
  Some(ExternalReferenceId {
    book: Some(Cow::Owned(program.symbols.get(book)?.to_string())),
    sheet: match sheet {
      Some(sheet) => Some(Cow::Owned(sheet_range_text(program, sheet)?)),
      None => None,
    },
    name: Some(Cow::Owned(name)),
  })
}

pub(crate) fn external_reference_from_range<'doc>(
  program: &FormulaProgram,
  book: crate::symbol::FormulaSymbolId,
  sheet: Option<FormulaSheetRange>,
  start: FormulaReferencePoint,
  end: FormulaReferencePoint,
  flags: FormulaReferenceFlags,
) -> Option<ExternalReferenceId<'doc>> {
  let mut name = String::new();
  push_cell_reference_text(start.address, start.flags, flags, &mut name);
  name.push(':');
  push_cell_reference_text(end.address, end.flags, flags, &mut name);
  Some(ExternalReferenceId {
    book: Some(Cow::Owned(program.symbols.get(book)?.to_string())),
    sheet: match sheet {
      Some(sheet) => Some(Cow::Owned(sheet_range_text(program, sheet)?)),
      None => None,
    },
    name: Some(Cow::Owned(name)),
  })
}

pub(crate) fn scoped_name_cow<'doc>(
  program: &FormulaProgram,
  borrowed_source: Option<&'doc str>,
  span: Option<crate::program::SourceSpan>,
  scope: &FormulaNameScope,
  name: crate::symbol::FormulaSymbolId,
) -> Option<Cow<'doc, str>> {
  match scope {
    FormulaNameScope::Workbook => program_symbol_cow(program, borrowed_source, span, name),
    FormulaNameScope::Sheet(sheet) => {
      let mut text = sheet_name_text(program, *sheet)?;
      text.push('!');
      text.push_str(program.symbols.get(name)?);
      Some(Cow::Owned(text))
    }
  }
}

pub(crate) fn program_text_cow<'doc>(
  program: &FormulaProgram,
  borrowed_source: Option<&'doc str>,
  span: Option<crate::program::SourceSpan>,
  symbol: crate::symbol::FormulaSymbolId,
) -> Option<Cow<'doc, str>> {
  if let (Some(source), Some(span)) = (borrowed_source, span)
    && let Some(value) = parser::formula_text_literal(source, span.start)
  {
    return Some(match value {
      parser::TextLiteral::Borrowed(value) => Cow::Borrowed(value),
      parser::TextLiteral::Owned(value) => Cow::Owned(value),
    });
  }
  Some(Cow::Owned(program.symbols.get(symbol)?.to_string()))
}

pub(crate) fn program_symbol_cow<'doc>(
  program: &FormulaProgram,
  borrowed_source: Option<&'doc str>,
  span: Option<crate::program::SourceSpan>,
  symbol: crate::symbol::FormulaSymbolId,
) -> Option<Cow<'doc, str>> {
  let text = program.symbols.get(symbol)?;
  if let (Some(source), Some(span)) = (borrowed_source, span)
    && source.get(span.start..span.end) == Some(text)
  {
    return Some(Cow::Borrowed(source.get(span.start..span.end)?));
  }
  Some(Cow::Owned(text.to_string()))
}

pub(crate) fn sheet_range_text(
  program: &FormulaProgram,
  range: FormulaSheetRange,
) -> Option<String> {
  let mut text = String::new();
  match range {
    FormulaSheetRange::Sheet(sheet) => text.push_str(&sheet_name_text(program, sheet)?),
    FormulaSheetRange::Range { start, end } => {
      text.push_str(&sheet_name_text(program, start)?);
      text.push(':');
      text.push_str(&sheet_name_text(program, end)?);
    }
  }
  Some(text)
}

pub(crate) fn sheet_name_text(program: &FormulaProgram, sheet: FormulaSheetName) -> Option<String> {
  match sheet {
    FormulaSheetName::Id(sheet) => Some(format!("Sheet{}", sheet.0)),
    FormulaSheetName::Name(name) => Some(program.symbols.get(name)?.to_string()),
  }
}

pub(crate) fn structured_reference_text(
  program: &FormulaProgram,
  reference: &FormulaStructuredReference,
) -> Option<String> {
  let mut text = String::new();
  if let Some(table) = reference.table {
    text.push_str(program.symbols.get(table)?);
  }
  if reference.table.is_some()
    && matches!(
      reference.specifier,
      FormulaStructuredReferenceSpecifier::Table
    )
  {
    text.push_str("[]");
  } else {
    push_structured_reference_specifier(program, &reference.specifier, &mut text)?;
  }
  Some(text)
}

fn push_structured_reference_specifier(
  program: &FormulaProgram,
  specifier: &FormulaStructuredReferenceSpecifier,
  text: &mut String,
) -> Option<()> {
  match specifier {
    FormulaStructuredReferenceSpecifier::Table => {}
    FormulaStructuredReferenceSpecifier::Item(item) => {
      text.push('[');
      text.push_str(structured_reference_item_text(*item));
      text.push(']');
    }
    FormulaStructuredReferenceSpecifier::Column(column) => {
      text.push('[');
      push_structured_reference_column(program.symbols.get(*column)?, text);
      text.push(']');
    }
    FormulaStructuredReferenceSpecifier::ColumnRange { start, end } => {
      text.push_str("[[");
      push_structured_reference_column(program.symbols.get(*start)?, text);
      text.push_str("]:[");
      push_structured_reference_column(program.symbols.get(*end)?, text);
      text.push_str("]]");
    }
    FormulaStructuredReferenceSpecifier::Combination(span) => {
      text.push('[');
      for (index, part) in program
        .structured_reference_parts(*span)?
        .iter()
        .enumerate()
      {
        if index > 0 {
          text.push(',');
        }
        match part {
          FormulaStructuredReferencePart::Item(item) => {
            text.push('[');
            text.push_str(structured_reference_item_text(*item));
            text.push(']');
          }
          FormulaStructuredReferencePart::Column(column) => {
            text.push('[');
            push_structured_reference_column(program.symbols.get(*column)?, text);
            text.push(']');
          }
          FormulaStructuredReferencePart::ColumnRange { start, end } => {
            text.push('[');
            push_structured_reference_column(program.symbols.get(*start)?, text);
            text.push_str("]:[");
            push_structured_reference_column(program.symbols.get(*end)?, text);
            text.push(']');
          }
        }
      }
      text.push(']');
    }
  }
  Some(())
}

fn structured_reference_item_text(item: FormulaStructuredReferenceItem) -> &'static str {
  match item {
    FormulaStructuredReferenceItem::All => "#All",
    FormulaStructuredReferenceItem::Data => "#Data",
    FormulaStructuredReferenceItem::Headers => "#Headers",
    FormulaStructuredReferenceItem::Totals => "#Totals",
    FormulaStructuredReferenceItem::ThisRow => "#This Row",
  }
}

fn push_structured_reference_column(value: &str, text: &mut String) {
  for ch in value.chars() {
    match ch {
      '\'' | '#' | '[' | ']' => {
        text.push('\'');
        text.push(ch);
      }
      _ => text.push(ch),
    }
  }
}

fn push_cell_reference_text(
  address: CellAddress,
  address_flags: FormulaAddressFlags,
  reference_flags: FormulaReferenceFlags,
  text: &mut String,
) {
  if address_flags.contains(FormulaAddressFlags::COL_ABS) {
    text.push('$');
  }
  push_column_name(address.column, text);
  if address_flags.contains(FormulaAddressFlags::ROW_ABS) {
    text.push('$');
  }
  text.push_str(&(address.row + 1).to_string());
  if reference_flags.contains(FormulaReferenceFlags::SPILL) {
    text.push('#');
  }
}

fn push_column_name(mut column: u32, text: &mut String) {
  let mut name = [0u8; 8];
  let mut len = 0usize;
  loop {
    name[len] = b'A' + (column % 26) as u8;
    len += 1;
    column /= 26;
    if column == 0 {
      break;
    }
    column -= 1;
  }
  for ch in name[..len].iter().rev() {
    text.push(*ch as char);
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FormulaControlOp {
  IfJump,
  IfErrorJump,
  IfNaJump,
  IfsJump,
  SwitchJump,
  ChooseJump,
  LetBind,
}

pub(crate) fn control_for_function(
  function: Option<FormulaFunctionId>,
) -> Option<FormulaControlOp> {
  match function? {
    FormulaFunctionId::If => Some(FormulaControlOp::IfJump),
    FormulaFunctionId::Iferror => Some(FormulaControlOp::IfErrorJump),
    FormulaFunctionId::Ifna => Some(FormulaControlOp::IfNaJump),
    FormulaFunctionId::Ifs => Some(FormulaControlOp::IfsJump),
    FormulaFunctionId::Switch => Some(FormulaControlOp::SwitchJump),
    FormulaFunctionId::Choose => Some(FormulaControlOp::ChooseJump),
    FormulaFunctionId::Let => Some(FormulaControlOp::LetBind),
    _ => None,
  }
}

pub(crate) fn formula_operator_from_lex(operator: parser::LexOperator) -> FormulaOperator {
  match operator {
    parser::LexOperator::Add => FormulaOperator::Add,
    parser::LexOperator::Subtract => FormulaOperator::Subtract,
    parser::LexOperator::Multiply => FormulaOperator::Multiply,
    parser::LexOperator::Divide => FormulaOperator::Divide,
    parser::LexOperator::Power => FormulaOperator::Power,
    parser::LexOperator::Concat => FormulaOperator::Concat,
    parser::LexOperator::Equal => FormulaOperator::Equal,
    parser::LexOperator::NotEqual => FormulaOperator::NotEqual,
    parser::LexOperator::Less => FormulaOperator::Less,
    parser::LexOperator::LessOrEqual => FormulaOperator::LessOrEqual,
    parser::LexOperator::Greater => FormulaOperator::Greater,
    parser::LexOperator::GreaterOrEqual => FormulaOperator::GreaterOrEqual,
    parser::LexOperator::Range => FormulaOperator::Range,
    parser::LexOperator::Union => FormulaOperator::Union,
    parser::LexOperator::Intersection => FormulaOperator::Intersection,
    parser::LexOperator::ImplicitIntersection => FormulaOperator::ImplicitIntersection,
    parser::LexOperator::Percent => FormulaOperator::Percent,
  }
}

pub(crate) fn formula_error_from_lex(error: parser::LexErrorValue) -> FormulaErrorValue {
  match error {
    parser::LexErrorValue::Null => FormulaErrorValue::Null,
    parser::LexErrorValue::Div0 => FormulaErrorValue::Div0,
    parser::LexErrorValue::Value => FormulaErrorValue::Value,
    parser::LexErrorValue::Ref => FormulaErrorValue::Ref,
    parser::LexErrorValue::Name => FormulaErrorValue::Name,
    parser::LexErrorValue::Num => FormulaErrorValue::Num,
    parser::LexErrorValue::NA => FormulaErrorValue::NA,
    parser::LexErrorValue::GettingData => FormulaErrorValue::GettingData,
    parser::LexErrorValue::Spill => FormulaErrorValue::Spill,
    parser::LexErrorValue::Calc => FormulaErrorValue::Calc,
    parser::LexErrorValue::Error => FormulaErrorValue::Error,
    parser::LexErrorValue::NotImplemented => FormulaErrorValue::NotImplemented,
    parser::LexErrorValue::CircularReference => FormulaErrorValue::CircularReference,
    parser::LexErrorValue::IllegalChar => FormulaErrorValue::IllegalChar,
    parser::LexErrorValue::IllegalArgument => FormulaErrorValue::IllegalArgument,
    parser::LexErrorValue::IllegalParameter => FormulaErrorValue::IllegalParameter,
    parser::LexErrorValue::Pair => FormulaErrorValue::Pair,
    parser::LexErrorValue::PairExpected => FormulaErrorValue::PairExpected,
    parser::LexErrorValue::OperatorExpected => FormulaErrorValue::OperatorExpected,
    parser::LexErrorValue::VariableExpected => FormulaErrorValue::VariableExpected,
    parser::LexErrorValue::Parameter => FormulaErrorValue::Parameter,
    parser::LexErrorValue::CodeOverflow => FormulaErrorValue::CodeOverflow,
    parser::LexErrorValue::StringOverflow => FormulaErrorValue::StringOverflow,
    parser::LexErrorValue::StackOverflow => FormulaErrorValue::StackOverflow,
    parser::LexErrorValue::InvalidVariable => FormulaErrorValue::InvalidVariable,
    parser::LexErrorValue::InvalidOpcode => FormulaErrorValue::InvalidOpcode,
    parser::LexErrorValue::InvalidStackValue => FormulaErrorValue::InvalidStackValue,
    parser::LexErrorValue::InvalidToken => FormulaErrorValue::InvalidToken,
    parser::LexErrorValue::NoConvergence => FormulaErrorValue::NoConvergence,
    parser::LexErrorValue::NoAddin => FormulaErrorValue::NoAddin,
    parser::LexErrorValue::NoMacro => FormulaErrorValue::NoMacro,
    parser::LexErrorValue::NestedArray => FormulaErrorValue::NestedArray,
    parser::LexErrorValue::MatrixSize => FormulaErrorValue::MatrixSize,
    parser::LexErrorValue::BadArrayContent => FormulaErrorValue::BadArrayContent,
    parser::LexErrorValue::LinkFormulaNeedingCheck => FormulaErrorValue::LinkFormulaNeedingCheck,
  }
}

pub(crate) fn formula_value_from_array_constant<'doc>(
  value: parser::ArrayConstantValue<'_>,
) -> FormulaValue<'doc> {
  match value {
    parser::ArrayConstantValue::Blank => FormulaValue::Blank,
    parser::ArrayConstantValue::Number(value) => FormulaValue::Number(value),
    parser::ArrayConstantValue::Boolean(value) => FormulaValue::Boolean(value),
    parser::ArrayConstantValue::Error(value) => FormulaValue::Error(formula_error_from_lex(value)),
    parser::ArrayConstantValue::Text(value) => match value {
      parser::TextLiteral::Borrowed(value) => FormulaValue::String(Cow::Owned(value.into())),
      parser::TextLiteral::Owned(value) => FormulaValue::String(Cow::Owned(value)),
    },
  }
}
