use std::fmt::Write as _;

use bitflags::bitflags;

mod syntax;

use crate::source::FormulaSource;
use crate::symbol::{FormulaSymbolId, FormulaSymbolPool};
use crate::{AddressFlags, CellAddress, CellRange, FormulaErrorValue, FormulaOperator, SheetId};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FormulaProgram {
  pub symbols: FormulaSymbolPool,
  pub nodes: Vec<FormulaNode>,
  pub args: Vec<FormulaExprId>,
  pub array_elements: Vec<FormulaExprId>,
  pub structured_reference_parts: Vec<FormulaStructuredReferencePart>,
  pub root: Option<FormulaExprId>,
  pub root_class: FormulaOperandClass,
  pub diagnostics: Vec<FormulaDiagnostic>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormulaExprId(pub u32);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormulaArgSpan {
  pub offset: u32,
  pub len: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormulaArraySpan {
  pub offset: u32,
  pub rows: u16,
  pub cols: u16,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormulaStructuredReferenceSpan {
  pub offset: u32,
  pub len: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormulaNode {
  pub kind: FormulaNodeKind,
  pub span: Option<SourceSpan>,
  pub metadata: FormulaNodeMetadata,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct FormulaNodeMetadata {
  pub labels: FormulaNodeLabels,
  pub operand_class: FormulaOperandClass,
  pub param_class: FormulaParamClass,
}

bitflags! {
  #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
  pub struct FormulaNodeLabels: u32 {
    const VOLATILE = 0x0000_0001;
    const RETURNS_REFERENCE = 0x0000_0002;
    const CONTAINS_REFERENCE = 0x0000_0004;
    const CONTAINS_STRUCTURED_REFERENCE = 0x0000_0008;
    const CONTAINS_EXTERNAL_REFERENCE = 0x0000_0010;
    const CONTAINS_NAME = 0x0000_0020;
    const CONTAINS_ARRAY = 0x0000_0040;
    const NEEDS_GRAMMAR_SPECIFIC_PRINT = 0x0000_0080;
    const UNSUPPORTED_FOR_EVAL = 0x0000_0100;
    const UNSUPPORTED_FOR_REWRITE = 0x0000_0200;
    const PRESERVE_AS_TOKEN = 0x0000_0400;
    const GENERATED = 0x0000_0800;
  }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FormulaOperandClass {
  #[default]
  Unknown,
  Reference,
  Value,
  Array,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum FormulaParamClass {
  #[default]
  Unknown,
  Bounds,
  Value,
  Reference,
  ReferenceOrRefArray,
  Array,
  ForceArray,
  ReferenceOrForceArray,
  SuppressedReferenceOrForceArray,
  ForceArrayReturn,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FormulaNodeKind {
  Blank,
  Text(FormulaSymbolId),
  Number(FormulaNumberLiteral),
  Boolean(bool),
  Error(FormulaErrorValue),
  Reference(FormulaReference),
  Unary {
    op: FormulaOperator,
    expr: FormulaExprId,
  },
  Binary {
    op: FormulaOperator,
    left: FormulaExprId,
    right: FormulaExprId,
  },
  Function {
    name: FormulaFunctionName,
    args: FormulaArgSpan,
  },
  Call {
    callee: FormulaExprId,
    args: FormulaArgSpan,
  },
  Array(FormulaArraySpan),
  MissingArgument,
  Unsupported(FormulaUnsupportedKind),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FormulaNumberLiteral {
  Integer(i64),
  Number(f64),
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormulaFunctionName {
  Builtin(FormulaBuiltinFunctionId),
  External(FormulaSymbolId),
  Unknown(FormulaSymbolId),
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FormulaBuiltinFunctionId(pub u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceSpan {
  pub start: usize,
  pub end: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FormulaReference {
  Cell(FormulaCellReference),
  Range(FormulaRangeReference),
  Named(FormulaNamedReference),
  Structured(FormulaStructuredReference),
  ExternalName(FormulaExternalNameReference),
  Deleted(FormulaDeletedReference),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FormulaCellReference {
  pub target: FormulaReferencePoint,
  pub flags: FormulaReferenceFlags,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FormulaRangeReference {
  pub start: FormulaReferencePoint,
  pub end: FormulaReferencePoint,
  pub flags: FormulaReferenceFlags,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FormulaReferencePoint {
  pub sheet: FormulaSheetReference,
  pub address: CellAddress,
  pub flags: FormulaAddressFlags,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormulaSheetReference {
  Current,
  Local(FormulaSheetRange),
  External {
    book: FormulaSymbolId,
    sheet: Option<FormulaSheetRange>,
  },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormulaSheetRange {
  Sheet(FormulaSheetName),
  Range {
    start: FormulaSheetName,
    end: FormulaSheetName,
  },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormulaSheetName {
  Id(SheetId),
  Name(FormulaSymbolId),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FormulaNamedReference {
  pub name: FormulaSymbolId,
  pub scope: FormulaNameScope,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FormulaNameScope {
  Workbook,
  Sheet(FormulaSheetName),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FormulaExternalNameReference {
  pub book: FormulaSymbolId,
  pub sheet: Option<FormulaSheetName>,
  pub name: FormulaSymbolId,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FormulaDeletedReference {
  pub kind: FormulaDeletedReferenceKind,
  pub sheet: Option<FormulaSheetReference>,
  pub address_flags: FormulaAddressFlags,
  pub reference_flags: FormulaReferenceFlags,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormulaDeletedReferenceKind {
  Reference,
  Name,
  Sheet,
  External,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FormulaStructuredReference {
  pub table: Option<FormulaSymbolId>,
  pub specifier: FormulaStructuredReferenceSpecifier,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FormulaStructuredReferenceSpecifier {
  Table,
  Item(FormulaStructuredReferenceItem),
  Column(FormulaSymbolId),
  ColumnRange {
    start: FormulaSymbolId,
    end: FormulaSymbolId,
  },
  Combination(FormulaStructuredReferenceSpan),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FormulaStructuredReferencePart {
  Item(FormulaStructuredReferenceItem),
  Column(FormulaSymbolId),
  ColumnRange {
    start: FormulaSymbolId,
    end: FormulaSymbolId,
  },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FormulaStructuredReferenceItem {
  All,
  Data,
  Headers,
  Totals,
  ThisRow,
}

bitflags! {
  #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
  pub struct FormulaAddressFlags: u32 {
    const COL_ABS = 0x0001;
    const ROW_ABS = 0x0002;
    const TAB_ABS = 0x0004;
    const TAB_3D = 0x0008;
    const ROW_VALID = 0x0100;
    const COL_VALID = 0x0200;
    const TAB_VALID = 0x0400;
    const FORCE_DOC = 0x0800;
    const VALID = 0x8000;
    const COL_DELETED = 0x0001_0000;
    const ROW_DELETED = 0x0002_0000;
    const TAB_DELETED = 0x0004_0000;
    const REL_NAME = 0x0008_0000;

    const BITS = Self::COL_ABS.bits()
      | Self::ROW_ABS.bits()
      | Self::TAB_ABS.bits()
      | Self::TAB_3D.bits()
      | Self::ROW_VALID.bits()
      | Self::COL_VALID.bits()
      | Self::TAB_VALID.bits();
    const ADDR_ABS = Self::VALID.bits()
      | Self::COL_ABS.bits()
      | Self::ROW_ABS.bits()
      | Self::TAB_ABS.bits();
    const ADDR_ABS_3D = Self::ADDR_ABS.bits() | Self::TAB_3D.bits();
  }
}

bitflags! {
  #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
  pub struct FormulaReferenceFlags: u32 {
    const WHOLE_COLUMN = 0x0001;
    const WHOLE_ROW = 0x0002;
    const SPILL = 0x0004;
    const IMPLICIT_INTERSECTION = 0x0008;
    const TRIM_TO_DATA = 0x0010;
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaUnsupportedKind {
  Token,
  Function,
  Reference,
  Grammar,
  OperandClass,
  ParamClass,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormulaDiagnostic {
  pub span: Option<SourceSpan>,
  pub kind: FormulaDiagnosticKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FormulaDiagnosticKind {
  Unsupported(FormulaUnsupportedKind),
  ParseError,
  ReferenceError,
  OperandClassError,
  ParamClassError,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaEditOp {
  Copy {
    source: FormulaEditRange,
    target: FormulaEditRange,
  },
  Move {
    source: FormulaEditRange,
    target: FormulaEditRange,
  },
  InsertRows {
    sheet: SheetId,
    row: u32,
    count: u32,
  },
  DeleteRows {
    sheet: SheetId,
    row: u32,
    count: u32,
  },
  InsertColumns {
    sheet: SheetId,
    column: u32,
    count: u32,
  },
  DeleteColumns {
    sheet: SheetId,
    column: u32,
    count: u32,
  },
  MoveSheet {
    from: SheetId,
    to: SheetId,
  },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaEditStatus {
  Unchanged,
  Changed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaEditError {
  UnsupportedOperation,
  InvalidExpression,
}

pub type FormulaEditResult = std::result::Result<FormulaEditStatus, FormulaEditError>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FormulaEditRange {
  pub sheet: SheetId,
  pub range: CellRange,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormulaPrintOptions {
  pub grammar: FormulaPrintGrammar,
  pub include_leading_equals: bool,
}

impl Default for FormulaPrintOptions {
  fn default() -> Self {
    Self {
      grammar: FormulaPrintGrammar::ExcelA1,
      include_leading_equals: false,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FormulaPrintGrammar {
  ExcelA1,
  ExcelR1C1,
  OpenFormula,
  CalcA1,
}

#[derive(Clone, Debug, Default)]
pub struct FormulaProgramBuilder {
  program: FormulaProgram,
}

pub struct FormulaProgramEditor<'a> {
  program: &'a mut FormulaProgram,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FormulaProgramCheckpoint {
  nodes: usize,
  args: usize,
  array_elements: usize,
  structured_reference_parts: usize,
}

impl FormulaProgram {
  pub fn builder() -> FormulaProgramBuilder {
    FormulaProgramBuilder::new()
  }

  pub fn from_source(source: FormulaSource<'_>) -> Self {
    let body_start = crate::parser::formula_body_start(source.text);
    let body = source.text.get(body_start..).unwrap_or(source.text);
    let mut builder =
      FormulaProgramBuilder::with_capacity(syntax::program_node_capacity_hint(body));
    let root = syntax::parse_program_root(&mut builder, source, body_start, body);
    builder.finish_root(root)
  }

  pub fn print_formula(&self, options: &FormulaPrintOptions) -> Option<String> {
    let root = self.root?;
    let mut output = String::with_capacity(self.nodes.len().saturating_mul(8));
    if options.include_leading_equals {
      output.push('=');
    }
    FormulaPrinter {
      program: self,
      options,
    }
    .print_expr(root, 0, FormulaPrintSide::None, &mut output)?;
    Some(output)
  }

  pub fn edit(&mut self) -> FormulaProgramEditor<'_> {
    FormulaProgramEditor { program: self }
  }

  pub fn apply_edit(&mut self, op: FormulaEditOp) -> FormulaEditResult {
    self.edit().apply(op)
  }

  pub fn args(&self, span: FormulaArgSpan) -> Option<&[FormulaExprId]> {
    let start = span.offset as usize;
    let end = start.checked_add(span.len as usize)?;
    self.args.get(start..end)
  }

  pub fn array_elements(&self, span: FormulaArraySpan) -> Option<&[FormulaExprId]> {
    let start = span.offset as usize;
    let len = usize::from(span.rows).checked_mul(usize::from(span.cols))?;
    let end = start.checked_add(len)?;
    self.array_elements.get(start..end)
  }

  pub fn structured_reference_parts(
    &self,
    span: FormulaStructuredReferenceSpan,
  ) -> Option<&[FormulaStructuredReferencePart]> {
    let start = span.offset as usize;
    let end = start.checked_add(span.len as usize)?;
    self.structured_reference_parts.get(start..end)
  }

  pub(crate) fn node(&self, id: FormulaExprId) -> Option<&FormulaNode> {
    self.nodes.get(id.0 as usize)
  }
}

impl FormulaProgramEditor<'_> {
  pub fn apply(&mut self, _op: FormulaEditOp) -> FormulaEditResult {
    Err(FormulaEditError::UnsupportedOperation)
  }

  pub fn replace_root(&mut self, root: Option<FormulaExprId>) -> FormulaEditResult {
    if let Some(root) = root
      && self.program.node(root).is_none()
    {
      return Err(FormulaEditError::InvalidExpression);
    }
    if self.program.root == root {
      return Ok(FormulaEditStatus::Unchanged);
    }
    self.program.root = root;
    Ok(FormulaEditStatus::Changed)
  }
}

impl FormulaProgramBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_capacity(nodes: usize) -> Self {
    Self {
      program: FormulaProgram {
        nodes: Vec::with_capacity(nodes),
        args: Vec::with_capacity(nodes),
        ..FormulaProgram::default()
      },
    }
  }

  pub fn intern(&mut self, text: &str) -> FormulaSymbolId {
    self.program.symbols.intern(text)
  }

  pub fn push(
    &mut self,
    kind: FormulaNodeKind,
    span: Option<SourceSpan>,
    metadata: FormulaNodeMetadata,
  ) -> FormulaExprId {
    assert!(self.program.nodes.len() <= u32::MAX as usize);
    let id = FormulaExprId(self.program.nodes.len() as u32);
    self.program.nodes.push(FormulaNode {
      kind,
      span,
      metadata,
    });
    id
  }

  pub fn blank(&mut self) -> FormulaExprId {
    self.push_value(FormulaNodeKind::Blank)
  }

  pub fn text(&mut self, value: &str) -> FormulaExprId {
    let symbol = self.intern(value);
    self.push_value(FormulaNodeKind::Text(symbol))
  }

  pub fn integer(&mut self, value: i64) -> FormulaExprId {
    self.push_value(FormulaNodeKind::Number(FormulaNumberLiteral::Integer(
      value,
    )))
  }

  pub fn number(&mut self, value: f64) -> FormulaExprId {
    self.push_value(FormulaNodeKind::Number(FormulaNumberLiteral::Number(value)))
  }

  pub fn boolean(&mut self, value: bool) -> FormulaExprId {
    self.push_value(FormulaNodeKind::Boolean(value))
  }

  pub fn error(&mut self, value: FormulaErrorValue) -> FormulaExprId {
    self.push_value(FormulaNodeKind::Error(value))
  }

  pub fn missing_argument(&mut self) -> FormulaExprId {
    self.push_value(FormulaNodeKind::MissingArgument)
  }

  pub fn named_reference(&mut self, name: &str) -> FormulaExprId {
    let name = self.intern(name);
    self.push_reference(FormulaReference::Named(FormulaNamedReference {
      name,
      scope: FormulaNameScope::Workbook,
    }))
  }

  pub fn cell_reference(&mut self, address: CellAddress) -> FormulaExprId {
    self.push_reference(FormulaReference::Cell(FormulaCellReference {
      target: FormulaReferencePoint {
        sheet: FormulaSheetReference::Current,
        address,
        flags: valid_address_flags(AddressFlags::default(), false),
      },
      flags: FormulaReferenceFlags::empty(),
    }))
  }

  pub fn range_reference(&mut self, start: CellAddress, end: CellAddress) -> FormulaExprId {
    self.push_reference(FormulaReference::Range(FormulaRangeReference {
      start: FormulaReferencePoint {
        sheet: FormulaSheetReference::Current,
        address: start,
        flags: valid_address_flags(AddressFlags::default(), false),
      },
      end: FormulaReferencePoint {
        sheet: FormulaSheetReference::Current,
        address: end,
        flags: valid_address_flags(AddressFlags::default(), false),
      },
      flags: FormulaReferenceFlags::empty(),
    }))
  }

  pub fn unary(&mut self, op: FormulaOperator, expr: FormulaExprId) -> FormulaExprId {
    self.push_value(FormulaNodeKind::Unary { op, expr })
  }

  pub fn binary(
    &mut self,
    op: FormulaOperator,
    left: FormulaExprId,
    right: FormulaExprId,
  ) -> FormulaExprId {
    self.push_value(FormulaNodeKind::Binary { op, left, right })
  }

  pub fn function(&mut self, name: &str, args: &[FormulaExprId]) -> FormulaExprId {
    let name = self.intern(name);
    let args = self.push_arg_span(args);
    self.push_value(FormulaNodeKind::Function {
      name: FormulaFunctionName::Unknown(name),
      args,
    })
  }

  pub fn array(&mut self, rows: u16, cols: u16, elements: &[FormulaExprId]) -> FormulaExprId {
    assert_eq!(usize::from(rows) * usize::from(cols), elements.len());
    let span = self.push_array_span(rows, cols, elements);
    self.push_value(FormulaNodeKind::Array(span))
  }

  pub fn structured_reference(
    &mut self,
    table: Option<&str>,
    specifier: FormulaStructuredReferenceSpecifier,
  ) -> FormulaExprId {
    let table = table.map(|table| self.intern(table));
    self.push_reference(FormulaReference::Structured(FormulaStructuredReference {
      table,
      specifier,
    }))
  }

  pub fn structured_reference_parts(
    &mut self,
    parts: &[FormulaStructuredReferencePart],
  ) -> FormulaStructuredReferenceSpan {
    assert!(parts.len() <= u16::MAX as usize);
    assert!(self.program.structured_reference_parts.len() <= u32::MAX as usize);
    let span = FormulaStructuredReferenceSpan {
      offset: self.program.structured_reference_parts.len() as u32,
      len: parts.len() as u16,
    };
    self
      .program
      .structured_reference_parts
      .extend_from_slice(parts);
    span
  }

  pub fn diagnostic(&mut self, span: Option<SourceSpan>, kind: FormulaDiagnosticKind) {
    self
      .program
      .diagnostics
      .push(FormulaDiagnostic { span, kind });
  }

  pub fn finish(self, root: FormulaExprId) -> FormulaProgram {
    self.finish_root(Some(root))
  }

  pub fn finish_root(mut self, root: Option<FormulaExprId>) -> FormulaProgram {
    self.program.root = root;
    self.program
  }

  fn push_value(&mut self, kind: FormulaNodeKind) -> FormulaExprId {
    self.push(kind, None, FormulaNodeMetadata::default())
  }

  fn push_reference(&mut self, reference: FormulaReference) -> FormulaExprId {
    self.push(
      FormulaNodeKind::Reference(reference),
      None,
      FormulaNodeMetadata {
        labels: FormulaNodeLabels::RETURNS_REFERENCE | FormulaNodeLabels::CONTAINS_REFERENCE,
        operand_class: FormulaOperandClass::Reference,
        param_class: FormulaParamClass::Unknown,
      },
    )
  }

  fn push_arg_span(&mut self, args: &[FormulaExprId]) -> FormulaArgSpan {
    assert!(args.len() <= u16::MAX as usize);
    assert!(self.program.args.len() <= u32::MAX as usize);
    let span = FormulaArgSpan {
      offset: self.program.args.len() as u32,
      len: args.len() as u16,
    };
    self.program.args.extend_from_slice(args);
    span
  }

  fn push_array_span(
    &mut self,
    rows: u16,
    cols: u16,
    elements: &[FormulaExprId],
  ) -> FormulaArraySpan {
    assert!(self.program.array_elements.len() <= u32::MAX as usize);
    let span = FormulaArraySpan {
      offset: self.program.array_elements.len() as u32,
      rows,
      cols,
    };
    self.program.array_elements.extend_from_slice(elements);
    span
  }

  fn checkpoint(&self) -> FormulaProgramCheckpoint {
    FormulaProgramCheckpoint {
      nodes: self.program.nodes.len(),
      args: self.program.args.len(),
      array_elements: self.program.array_elements.len(),
      structured_reference_parts: self.program.structured_reference_parts.len(),
    }
  }

  fn rollback(&mut self, checkpoint: FormulaProgramCheckpoint) {
    self.program.nodes.truncate(checkpoint.nodes);
    self.program.args.truncate(checkpoint.args);
    self
      .program
      .array_elements
      .truncate(checkpoint.array_elements);
    self
      .program
      .structured_reference_parts
      .truncate(checkpoint.structured_reference_parts);
  }

  fn array_mark(&self) -> usize {
    self.program.array_elements.len()
  }

  fn push_array_element(&mut self, element: FormulaExprId) {
    self.program.array_elements.push(element);
  }

  fn finish_array_span(&self, mark: usize, rows: u16, cols: u16) -> FormulaArraySpan {
    assert!(mark <= u32::MAX as usize);
    assert_eq!(
      self.program.array_elements.len() - mark,
      usize::from(rows) * usize::from(cols)
    );
    FormulaArraySpan {
      offset: mark as u32,
      rows,
      cols,
    }
  }
}

#[derive(Clone, Copy)]
enum FormulaPrintSide {
  None,
  Left,
  Right,
}

struct FormulaPrinter<'a> {
  program: &'a FormulaProgram,
  options: &'a FormulaPrintOptions,
}

impl FormulaPrinter<'_> {
  fn print_expr(
    &self,
    id: FormulaExprId,
    parent_precedence: u8,
    side: FormulaPrintSide,
    output: &mut String,
  ) -> Option<u8> {
    let node = self.program.node(id)?;
    let precedence = node_precedence(&node.kind);
    let parens = needs_parentheses(precedence, parent_precedence, side, &node.kind);
    if parens {
      output.push('(');
    }
    match &node.kind {
      FormulaNodeKind::Blank | FormulaNodeKind::MissingArgument => {}
      FormulaNodeKind::Text(value) => self.print_text(*value, output)?,
      FormulaNodeKind::Number(value) => print_number(*value, output),
      FormulaNodeKind::Boolean(value) => output.push_str(if *value { "TRUE" } else { "FALSE" }),
      FormulaNodeKind::Error(value) => output.push_str(error_text(*value)),
      FormulaNodeKind::Reference(reference) => self.print_reference(reference, output)?,
      FormulaNodeKind::Unary { op, expr } => self.print_unary(*op, *expr, output)?,
      FormulaNodeKind::Binary { op, left, right } => {
        self.print_binary(*op, *left, *right, output)?
      }
      FormulaNodeKind::Function { name, args } => self.print_function(*name, *args, output)?,
      FormulaNodeKind::Call { callee, args } => self.print_call(*callee, *args, output)?,
      FormulaNodeKind::Array(span) => self.print_array(*span, output)?,
      FormulaNodeKind::Unsupported(_) => return None,
    }
    if parens {
      output.push(')');
    }
    Some(precedence)
  }

  fn print_text(&self, id: FormulaSymbolId, output: &mut String) -> Option<()> {
    output.push('"');
    for ch in self.program.symbols.get(id)?.chars() {
      if ch == '"' {
        output.push('"');
      }
      output.push(ch);
    }
    output.push('"');
    Some(())
  }

  fn print_unary(
    &self,
    op: FormulaOperator,
    expr: FormulaExprId,
    output: &mut String,
  ) -> Option<()> {
    match op {
      FormulaOperator::UnaryPlus => output.push('+'),
      FormulaOperator::UnaryMinus => output.push('-'),
      FormulaOperator::ImplicitIntersection => output.push('@'),
      FormulaOperator::Percent => {
        self.print_expr(expr, unary_precedence(op), FormulaPrintSide::Right, output)?;
        output.push('%');
        return Some(());
      }
      _ => return None,
    }
    self.print_expr(expr, unary_precedence(op), FormulaPrintSide::Right, output)?;
    Some(())
  }

  fn print_binary(
    &self,
    op: FormulaOperator,
    left: FormulaExprId,
    right: FormulaExprId,
    output: &mut String,
  ) -> Option<()> {
    let precedence = binary_precedence(op)?;
    self.print_expr(left, precedence, FormulaPrintSide::Left, output)?;
    output.push_str(binary_operator_text(op)?);
    self.print_expr(right, precedence, FormulaPrintSide::Right, output)?;
    Some(())
  }

  fn print_function(
    &self,
    name: FormulaFunctionName,
    args: FormulaArgSpan,
    output: &mut String,
  ) -> Option<()> {
    self.print_function_name(name, output)?;
    output.push('(');
    self.print_args(args, output)?;
    output.push(')');
    Some(())
  }

  fn print_call(
    &self,
    callee: FormulaExprId,
    args: FormulaArgSpan,
    output: &mut String,
  ) -> Option<()> {
    self.print_expr(callee, 0, FormulaPrintSide::None, output)?;
    output.push('(');
    self.print_args(args, output)?;
    output.push(')');
    Some(())
  }

  fn print_function_name(&self, name: FormulaFunctionName, output: &mut String) -> Option<()> {
    match name {
      FormulaFunctionName::Builtin(id) => write!(output, "BUILTIN{}", id.0).ok()?,
      FormulaFunctionName::External(id) | FormulaFunctionName::Unknown(id) => {
        output.push_str(self.program.symbols.get(id)?);
      }
    }
    Some(())
  }

  fn print_args(&self, span: FormulaArgSpan, output: &mut String) -> Option<()> {
    let separator = argument_separator(self.options.grammar);
    for (index, arg) in self.program.args(span)?.iter().copied().enumerate() {
      if index > 0 {
        output.push_str(separator);
      }
      self.print_expr(arg, 0, FormulaPrintSide::None, output)?;
    }
    Some(())
  }

  fn print_array(&self, span: FormulaArraySpan, output: &mut String) -> Option<()> {
    let elements = self.program.array_elements(span)?;
    let cols = usize::from(span.cols);
    output.push('{');
    for row in 0..usize::from(span.rows) {
      if row > 0 {
        output.push(';');
      }
      for col in 0..cols {
        if col > 0 {
          output.push(',');
        }
        let index = row.checked_mul(cols)?.checked_add(col)?;
        self.print_expr(*elements.get(index)?, 0, FormulaPrintSide::None, output)?;
      }
    }
    output.push('}');
    Some(())
  }

  fn print_reference(&self, reference: &FormulaReference, output: &mut String) -> Option<()> {
    match reference {
      FormulaReference::Cell(reference) => {
        self.print_sheet_reference(reference.target.sheet, output)?;
        print_cell_address(reference.target.address, reference.target.flags, output);
        if reference.flags.contains(FormulaReferenceFlags::SPILL) {
          output.push('#');
        }
      }
      FormulaReference::Range(reference) => {
        self.print_sheet_reference(reference.start.sheet, output)?;
        print_cell_address(reference.start.address, reference.start.flags, output);
        output.push(':');
        if reference.end.sheet != reference.start.sheet {
          self.print_sheet_reference(reference.end.sheet, output)?;
        }
        print_cell_address(reference.end.address, reference.end.flags, output);
        if reference.flags.contains(FormulaReferenceFlags::SPILL) {
          output.push('#');
        }
      }
      FormulaReference::Named(reference) => {
        self.print_name_scope(&reference.scope, output)?;
        output.push_str(self.program.symbols.get(reference.name)?);
      }
      FormulaReference::Structured(reference) => {
        output.push_str(&crate::code::structured_reference_text(
          self.program,
          reference,
        )?);
      }
      FormulaReference::ExternalName(reference) => {
        output.push('[');
        output.push_str(self.program.symbols.get(reference.book)?);
        output.push(']');
        if let Some(sheet) = reference.sheet {
          self.print_sheet_name(sheet, output)?;
          output.push('!');
        }
        output.push_str(self.program.symbols.get(reference.name)?);
      }
      FormulaReference::Deleted(reference) => {
        if let Some(sheet) = reference.sheet {
          self.print_sheet_reference(sheet, output)?;
        }
        output.push_str("#REF!");
      }
    }
    Some(())
  }

  fn print_name_scope(&self, scope: &FormulaNameScope, output: &mut String) -> Option<()> {
    match scope {
      FormulaNameScope::Workbook => {}
      FormulaNameScope::Sheet(sheet) => {
        self.print_sheet_name(*sheet, output)?;
        output.push('!');
      }
    }
    Some(())
  }

  fn print_sheet_reference(&self, sheet: FormulaSheetReference, output: &mut String) -> Option<()> {
    match sheet {
      FormulaSheetReference::Current => {}
      FormulaSheetReference::Local(range) => {
        self.print_sheet_range(range, output)?;
        output.push('!');
      }
      FormulaSheetReference::External { book, sheet } => {
        output.push('[');
        output.push_str(self.program.symbols.get(book)?);
        output.push(']');
        if let Some(sheet) = sheet {
          self.print_sheet_range(sheet, output)?;
        }
        output.push('!');
      }
    }
    Some(())
  }

  fn print_sheet_range(&self, sheet: FormulaSheetRange, output: &mut String) -> Option<()> {
    match sheet {
      FormulaSheetRange::Sheet(sheet) => self.print_sheet_name(sheet, output),
      FormulaSheetRange::Range { start, end } => {
        self.print_sheet_name(start, output)?;
        output.push(':');
        self.print_sheet_name(end, output)
      }
    }
  }

  fn print_sheet_name(&self, sheet: FormulaSheetName, output: &mut String) -> Option<()> {
    match sheet {
      FormulaSheetName::Id(id) => write!(output, "Sheet{}", id.0).ok(),
      FormulaSheetName::Name(id) => {
        print_quoted_sheet_name(self.program.symbols.get(id)?, output);
        Some(())
      }
    }
  }
}

fn valid_address_flags(flags: AddressFlags, sheet_valid: bool) -> FormulaAddressFlags {
  let mut result =
    FormulaAddressFlags::VALID | FormulaAddressFlags::COL_VALID | FormulaAddressFlags::ROW_VALID;
  if sheet_valid {
    result |= FormulaAddressFlags::TAB_VALID | FormulaAddressFlags::TAB_3D;
  }
  if flags.absolute_column {
    result |= FormulaAddressFlags::COL_ABS;
  }
  if flags.absolute_row {
    result |= FormulaAddressFlags::ROW_ABS;
  }
  result
}

fn reference_flags(start: AddressFlags, end: AddressFlags) -> FormulaReferenceFlags {
  let mut flags = FormulaReferenceFlags::empty();
  if start.whole_column && end.whole_column {
    flags |= FormulaReferenceFlags::WHOLE_COLUMN;
  }
  if start.whole_row && end.whole_row {
    flags |= FormulaReferenceFlags::WHOLE_ROW;
  }
  flags
}

fn node_precedence(kind: &FormulaNodeKind) -> u8 {
  match kind {
    FormulaNodeKind::Binary { op, .. } => binary_precedence(*op).unwrap_or(0),
    FormulaNodeKind::Unary { op, .. } => unary_precedence(*op),
    FormulaNodeKind::Function { .. } | FormulaNodeKind::Call { .. } => 12,
    FormulaNodeKind::Array(_) => 12,
    _ => 13,
  }
}

fn needs_parentheses(
  precedence: u8,
  parent_precedence: u8,
  side: FormulaPrintSide,
  kind: &FormulaNodeKind,
) -> bool {
  if parent_precedence == 0 || precedence > parent_precedence {
    return false;
  }
  if precedence < parent_precedence {
    return true;
  }
  matches!(
    (side, kind),
    (FormulaPrintSide::Right, FormulaNodeKind::Binary { .. })
      | (
        FormulaPrintSide::Left,
        FormulaNodeKind::Binary {
          op: FormulaOperator::Power,
          ..
        }
      )
  )
}

fn unary_precedence(operator: FormulaOperator) -> u8 {
  match operator {
    FormulaOperator::Percent => 11,
    FormulaOperator::UnaryPlus
    | FormulaOperator::UnaryMinus
    | FormulaOperator::ImplicitIntersection => 10,
    _ => 0,
  }
}

fn binary_precedence(operator: FormulaOperator) -> Option<u8> {
  Some(match operator {
    FormulaOperator::Union => 1,
    FormulaOperator::Intersection => 2,
    FormulaOperator::Range => 3,
    FormulaOperator::Equal
    | FormulaOperator::NotEqual
    | FormulaOperator::Less
    | FormulaOperator::LessOrEqual
    | FormulaOperator::Greater
    | FormulaOperator::GreaterOrEqual => 4,
    FormulaOperator::Concat => 5,
    FormulaOperator::Add | FormulaOperator::Subtract => 6,
    FormulaOperator::Multiply | FormulaOperator::Divide => 7,
    FormulaOperator::Power => 9,
    _ => return None,
  })
}

fn binary_operator_text(operator: FormulaOperator) -> Option<&'static str> {
  Some(match operator {
    FormulaOperator::Add => "+",
    FormulaOperator::Subtract => "-",
    FormulaOperator::Multiply => "*",
    FormulaOperator::Divide => "/",
    FormulaOperator::Power => "^",
    FormulaOperator::Concat => "&",
    FormulaOperator::Equal => "=",
    FormulaOperator::NotEqual => "<>",
    FormulaOperator::Less => "<",
    FormulaOperator::LessOrEqual => "<=",
    FormulaOperator::Greater => ">",
    FormulaOperator::GreaterOrEqual => ">=",
    FormulaOperator::Range => ":",
    FormulaOperator::Union => ",",
    FormulaOperator::Intersection => " ",
    _ => return None,
  })
}

fn print_number(value: FormulaNumberLiteral, output: &mut String) {
  match value {
    FormulaNumberLiteral::Integer(value) => {
      let _ = write!(output, "{value}");
    }
    FormulaNumberLiteral::Number(value) => {
      let _ = write!(output, "{value}");
    }
  }
}

fn error_text(value: FormulaErrorValue) -> &'static str {
  match value {
    FormulaErrorValue::Null => "#NULL!",
    FormulaErrorValue::Div0 => "#DIV/0!",
    FormulaErrorValue::Value => "#VALUE!",
    FormulaErrorValue::Ref => "#REF!",
    FormulaErrorValue::Name => "#NAME?",
    FormulaErrorValue::Num => "#NUM!",
    FormulaErrorValue::NA => "#N/A",
    FormulaErrorValue::GettingData => "#GETTING_DATA",
    FormulaErrorValue::Spill => "#SPILL!",
    FormulaErrorValue::Calc => "#CALC!",
    FormulaErrorValue::Error => "#ERROR!",
    FormulaErrorValue::NotImplemented => "#N/IMPL!",
    FormulaErrorValue::CircularReference => "#CIRC!",
    FormulaErrorValue::IllegalChar => "Err:501",
    FormulaErrorValue::IllegalArgument => "Err:502",
    FormulaErrorValue::IllegalParameter => "Err:504",
    FormulaErrorValue::Pair => "Err:507",
    FormulaErrorValue::PairExpected => "Err:508",
    FormulaErrorValue::OperatorExpected => "Err:509",
    FormulaErrorValue::VariableExpected => "Err:510",
    FormulaErrorValue::Parameter => "Err:511",
    FormulaErrorValue::CodeOverflow => "Err:512",
    FormulaErrorValue::StringOverflow => "Err:513",
    FormulaErrorValue::StackOverflow => "Err:514",
    FormulaErrorValue::InvalidVariable => "Err:516",
    FormulaErrorValue::InvalidOpcode => "Err:517",
    FormulaErrorValue::InvalidStackValue => "Err:518",
    FormulaErrorValue::InvalidToken => "Err:520",
    FormulaErrorValue::NoConvergence => "Err:523",
    FormulaErrorValue::NoAddin => "Err:530",
    FormulaErrorValue::NoMacro => "Err:531",
    FormulaErrorValue::NestedArray => "Err:533",
    FormulaErrorValue::MatrixSize => "Err:538",
    FormulaErrorValue::BadArrayContent => "Err:539",
    FormulaErrorValue::LinkFormulaNeedingCheck => "Err:540",
  }
}

fn print_cell_address(address: CellAddress, flags: FormulaAddressFlags, output: &mut String) {
  if flags.contains(FormulaAddressFlags::COL_ABS) {
    output.push('$');
  }
  print_column_name(address.column, output);
  if flags.contains(FormulaAddressFlags::ROW_ABS) {
    output.push('$');
  }
  let _ = write!(output, "{}", address.row + 1);
}

fn print_column_name(mut column: u32, output: &mut String) {
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
    output.push(*ch as char);
  }
}

fn print_quoted_sheet_name(value: &str, output: &mut String) {
  if is_unquoted_sheet_name(value) {
    output.push_str(value);
    return;
  }
  output.push('\'');
  for ch in value.chars() {
    if ch == '\'' {
      output.push('\'');
    }
    output.push(ch);
  }
  output.push('\'');
}

fn is_unquoted_sheet_name(value: &str) -> bool {
  !value.is_empty()
    && value
      .chars()
      .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
    && !value.chars().next().is_some_and(|ch| ch.is_ascii_digit())
}

fn argument_separator(grammar: FormulaPrintGrammar) -> &'static str {
  match grammar {
    FormulaPrintGrammar::OpenFormula | FormulaPrintGrammar::CalcA1 => ";",
    FormulaPrintGrammar::ExcelA1 | FormulaPrintGrammar::ExcelR1C1 => ",",
  }
}
