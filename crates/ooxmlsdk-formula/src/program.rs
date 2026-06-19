use std::cell::Cell;
use std::fmt::Write as _;

use bitflags::bitflags;
use smallvec::SmallVec;
use winnow::Parser;
use winnow::combinator::{Infix, Postfix, Prefix, expression};
use winnow::error::{ContextError, ParserError};
use winnow::stream::{Needed, Offset, Stream, StreamIsPartial};

use crate::source::{FormulaSource, FormulaSourcePosition};
use crate::symbol::{FormulaSymbolId, FormulaSymbolPool};
use crate::{
  AddressFlags, CellAddress, CellRange, FormulaErrorValue, FormulaOperator, QualifiedRange,
  SheetId, parser,
};

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

type FormulaArgBuffer = SmallVec<[FormulaExprId; 8]>;
type ProgramParseResult<T> = winnow::Result<T, ContextError>;
type ProgramPrefix<'p> = Prefix<ProgramSyntaxParser<'p>, FormulaExprId, ContextError>;
type ProgramPostfix<'p> = Postfix<ProgramSyntaxParser<'p>, FormulaExprId, ContextError>;
type ProgramInfix<'p> = Infix<ProgramSyntaxParser<'p>, FormulaExprId, ContextError>;
type ProgramUnaryFold<'p> =
  fn(&mut ProgramSyntaxParser<'p>, FormulaExprId) -> ProgramParseResult<FormulaExprId>;
type ProgramBinaryFold<'p> = fn(
  &mut ProgramSyntaxParser<'p>,
  FormulaExprId,
  FormulaExprId,
) -> ProgramParseResult<FormulaExprId>;

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
    let body_start = parser::formula_body_start(source.text);
    let body = source.text.get(body_start..).unwrap_or(source.text);
    let mut builder = FormulaProgramBuilder::with_capacity(program_node_capacity_hint(body));
    let root = parse_program_root(&mut builder, source, body_start, body);
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
        self.print_structured_reference(reference, output)?;
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

  fn print_structured_reference(
    &self,
    reference: &FormulaStructuredReference,
    output: &mut String,
  ) -> Option<()> {
    if let Some(table) = reference.table {
      output.push_str(self.program.symbols.get(table)?);
    }
    self.print_structured_reference_specifier(&reference.specifier, output)
  }

  fn print_structured_reference_specifier(
    &self,
    specifier: &FormulaStructuredReferenceSpecifier,
    output: &mut String,
  ) -> Option<()> {
    match specifier {
      FormulaStructuredReferenceSpecifier::Table => {}
      FormulaStructuredReferenceSpecifier::Item(item) => {
        output.push('[');
        output.push_str(structured_reference_item_text(*item));
        output.push(']');
      }
      FormulaStructuredReferenceSpecifier::Column(column) => {
        output.push('[');
        print_structured_reference_column(self.program.symbols.get(*column)?, output);
        output.push(']');
      }
      FormulaStructuredReferenceSpecifier::ColumnRange { start, end } => {
        output.push_str("[[");
        print_structured_reference_column(self.program.symbols.get(*start)?, output);
        output.push_str("]:[");
        print_structured_reference_column(self.program.symbols.get(*end)?, output);
        output.push_str("]]");
      }
      FormulaStructuredReferenceSpecifier::Combination(span) => {
        output.push('[');
        for (index, part) in self
          .program
          .structured_reference_parts(*span)?
          .iter()
          .enumerate()
        {
          if index > 0 {
            output.push(',');
          }
          match part {
            FormulaStructuredReferencePart::Item(item) => {
              output.push_str(structured_reference_item_text(*item));
            }
            FormulaStructuredReferencePart::Column(column) => {
              output.push('[');
              print_structured_reference_column(self.program.symbols.get(*column)?, output);
              output.push(']');
            }
            FormulaStructuredReferencePart::ColumnRange { start, end } => {
              output.push('[');
              print_structured_reference_column(self.program.symbols.get(*start)?, output);
              output.push_str("]:[");
              print_structured_reference_column(self.program.symbols.get(*end)?, output);
              output.push(']');
            }
          }
        }
        output.push(']');
      }
    }
    Some(())
  }
}

fn parse_program_root(
  builder: &mut FormulaProgramBuilder,
  source: FormulaSource<'_>,
  body_start: usize,
  body: &str,
) -> Option<FormulaExprId> {
  let mut parser = ProgramSyntaxParser::new(builder, source, body_start, body);
  let root = parser.parse_expression();
  if parser.missing_closing_parenthesis {
    parser
      .builder
      .diagnostic(None, FormulaDiagnosticKind::ParseError);
  }
  parser.diagnose_current_unsupported_token();
  if root.is_none() || !parser.tokens.is_end() {
    parser
      .builder
      .diagnostic(None, FormulaDiagnosticKind::ParseError);
  }
  root
}

fn program_node_capacity_hint(body: &str) -> usize {
  (body.len() / 4).clamp(4, 64)
}

struct ProgramSyntaxParser<'p> {
  builder: &'p mut FormulaProgramBuilder,
  source: FormulaSource<'p>,
  body_start: usize,
  body: &'p str,
  tokens: ProgramSyntaxTokens<'p>,
  missing_closing_parenthesis: bool,
}

impl<'p> ProgramSyntaxParser<'p> {
  fn new(
    builder: &'p mut FormulaProgramBuilder,
    source: FormulaSource<'p>,
    body_start: usize,
    body: &'p str,
  ) -> Self {
    Self {
      builder,
      source,
      body_start,
      body,
      tokens: ProgramSyntaxTokens::new(body),
      missing_closing_parenthesis: false,
    }
  }

  fn parse_expression(&mut self) -> Option<FormulaExprId> {
    self.parse_expression_bp(0)
  }

  fn parse_expression_bp(&mut self, min_bp: u8) -> Option<FormulaExprId> {
    let mut left = self.parse_winnow_expression(min_bp)?;
    loop {
      let had_ws = self.tokens.ws_before_next();

      if had_ws && is_intersection_rhs_start(self.tokens.peek()) {
        let (left_bp, right_bp) = infix_binding_power(parser::LexOperator::Intersection)?;
        if left_bp < min_bp {
          break;
        }
        let before_rhs = self.tokens.checkpoint();
        let checkpoint = self.builder.checkpoint();
        if let Some(right) = self.parse_expression_bp(right_bp) {
          left = self
            .builder
            .binary(FormulaOperator::Intersection, left, right);
          continue;
        }
        self.builder.rollback(checkpoint);
        self.tokens.reset(&before_rhs);
      }

      break;
    }
    Some(left)
  }

  fn parse_winnow_expression(&mut self, min_bp: u8) -> Option<FormulaExprId> {
    expression(parse_program_operand)
      .prefix(parse_program_prefix)
      .postfix(parse_program_postfix)
      .infix(parse_program_infix)
      .current_precedence_level(i64::from(min_bp))
      .parse_next(self)
      .ok()
  }

  fn parse_operand(&mut self) -> Option<FormulaExprId> {
    if self
      .tokens
      .consume_token_kind(parser::LexTokenKind::ParenOpen)
      .is_some()
    {
      let expr = self.parse_parenthesized_expression()?;
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ParenClose)
        .is_none()
      {
        self.missing_closing_parenthesis = true;
      }
      return Some(expr);
    }
    if let Some(token) = self.tokens.consume_token_kind(parser::LexTokenKind::Text) {
      return self.push_text_literal(token_span(token));
    }
    if self
      .tokens
      .consume_token_kind(parser::LexTokenKind::ArrayOpen)
      .is_some()
    {
      return self.parse_array();
    }
    if let Some(token) = self.tokens.peek() {
      match token.kind {
        parser::LexTokenKind::Number(value) => {
          self.tokens.advance();
          return Some(self.builder.number(value));
        }
        parser::LexTokenKind::Error(error) => {
          self.tokens.advance();
          return Some(
            self
              .builder
              .error(crate::code::formula_error_from_lex(error)),
          );
        }
        _ => {}
      }
    }
    self.parse_word_or_function()
  }

  fn parse_parenthesized_expression(&mut self) -> Option<FormulaExprId> {
    let mut left = self.parse_expression()?;
    while self
      .tokens
      .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)
      .is_some()
    {
      let right = self.parse_expression_bp(infix_binding_power(parser::LexOperator::Union)?.1)?;
      left = self.builder.binary(FormulaOperator::Union, left, right);
    }
    Some(left)
  }

  fn parse_array(&mut self) -> Option<FormulaExprId> {
    let mark = self.builder.array_mark();
    let mut rows = 0usize;
    let mut cols = None::<usize>;
    let mut row_len = 0usize;
    loop {
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArrayClose)
        .is_some()
      {
        if row_len != 0 {
          rows += 1;
          cols = finish_array_row(cols, row_len)?;
        } else if rows != 0 {
          let blank = self.builder.blank();
          self.builder.push_array_element(blank);
          rows += 1;
          cols = finish_array_row(cols, 1)?;
        }
        break;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        let blank = self.builder.blank();
        self.builder.push_array_element(blank);
        row_len += 1;
        continue;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::RowSeparator)
        .is_some()
      {
        if row_len == 0 {
          let blank = self.builder.blank();
          self.builder.push_array_element(blank);
          row_len = 1;
        }
        rows += 1;
        cols = finish_array_row(cols, row_len)?;
        row_len = 0;
        continue;
      }

      let element = self.parse_expression()?;
      self.builder.push_array_element(element);
      row_len += 1;

      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        continue;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::RowSeparator)
        .is_some()
      {
        rows += 1;
        cols = finish_array_row(cols, row_len)?;
        row_len = 0;
        continue;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArrayClose)
        .is_some()
      {
        rows += 1;
        cols = finish_array_row(cols, row_len)?;
        break;
      }
      return None;
    }

    if rows > u16::MAX as usize || cols.unwrap_or_default() > u16::MAX as usize {
      return None;
    }
    let span = self
      .builder
      .finish_array_span(mark, rows as u16, cols.unwrap_or_default() as u16);
    Some(self.builder.push_value(FormulaNodeKind::Array(span)))
  }

  fn parse_word_or_function(&mut self) -> Option<FormulaExprId> {
    let token = self.tokens.peek()?;
    if token.kind != parser::LexTokenKind::Word {
      return None;
    }
    let name = token_span(token);
    self.tokens.advance();
    if self
      .tokens
      .peek()
      .is_some_and(|token| token.kind == parser::LexTokenKind::ParenOpen)
    {
      let args = self.parse_argument_list(None)?;
      let name_text = self.body.get(name.start..name.end)?;
      let name_symbol = self.builder.intern(name_text);
      let mut labels = FormulaNodeLabels::empty();
      if parser::is_volatile_function_name(name_text) {
        labels |= FormulaNodeLabels::VOLATILE;
      }
      return Some(self.builder.push(
        FormulaNodeKind::Function {
          name: FormulaFunctionName::Unknown(name_symbol),
          args,
        },
        Some(source_span(self.body_start, name)),
        FormulaNodeMetadata {
          labels,
          operand_class: FormulaOperandClass::Unknown,
          param_class: FormulaParamClass::Unknown,
        },
      ));
    }
    let word = self.body.get(name.start..name.end)?;
    self.push_word(name, parser::semantic_word_kind(word))
  }

  fn parse_argument_list(&mut self, first: Option<FormulaExprId>) -> Option<FormulaArgSpan> {
    self
      .tokens
      .consume_token_kind(parser::LexTokenKind::ParenOpen)?;
    let mut args = FormulaArgBuffer::new();
    if let Some(first) = first {
      args.push(first);
    }
    loop {
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ParenClose)
        .is_some()
      {
        break;
      }
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)
        .is_some()
      {
        let blank = self.builder.blank();
        args.push(blank);
        if self
          .tokens
          .peek()
          .is_some_and(|token| token.kind == parser::LexTokenKind::ParenClose)
        {
          let blank = self.builder.blank();
          args.push(blank);
        }
        continue;
      }
      let arg = self.parse_expression()?;
      args.push(arg);
      if self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ParenClose)
        .is_some()
      {
        break;
      }
      self
        .tokens
        .consume_token_kind(parser::LexTokenKind::ArgumentSeparator)?;
      if self
        .tokens
        .peek()
        .is_some_and(|token| token.kind == parser::LexTokenKind::ParenClose)
      {
        let blank = self.builder.blank();
        args.push(blank);
      }
    }
    Some(self.builder.push_arg_span(&args))
  }

  fn push_text_literal(&mut self, span: parser::SemanticSpan) -> Option<FormulaExprId> {
    let value = match parser::formula_text_literal(self.source.text, self.body_start + span.start)?
    {
      parser::TextLiteral::Borrowed(value) => value,
      parser::TextLiteral::Owned(value) => {
        let symbol = self.builder.intern(&value);
        return Some(self.builder.push(
          FormulaNodeKind::Text(symbol),
          Some(source_span(self.body_start, span)),
          FormulaNodeMetadata::default(),
        ));
      }
    };
    let symbol = self.builder.intern(value);
    Some(self.builder.push(
      FormulaNodeKind::Text(symbol),
      Some(source_span(self.body_start, span)),
      FormulaNodeMetadata::default(),
    ))
  }

  fn push_word(
    &mut self,
    span: parser::SemanticSpan,
    kind: parser::SemanticWordKind,
  ) -> Option<FormulaExprId> {
    lower_parser_word(self.builder, self.source, self.body_start, span, kind)
  }

  fn diagnose_current_unsupported_token(&mut self) {
    let Some(token) = self.tokens.peek() else {
      return;
    };
    if token.kind == parser::LexTokenKind::Unsupported {
      self.builder.diagnostic(
        Some(source_span(self.body_start, token_span(token))),
        FormulaDiagnosticKind::Unsupported(FormulaUnsupportedKind::Token),
      );
    }
  }
}

fn parse_program_operand<'p>(
  input: &mut ProgramSyntaxParser<'p>,
) -> ProgramParseResult<FormulaExprId> {
  input.parse_operand().ok_or_else(|| program_error(input))
}

fn parse_program_prefix<'p>(
  input: &mut ProgramSyntaxParser<'p>,
) -> ProgramParseResult<ProgramPrefix<'p>> {
  let Some(token) = input.tokens.peek() else {
    return program_fail(input);
  };
  let parser::LexTokenKind::Operator(operator) = token.kind else {
    return program_fail(input);
  };
  let fold: ProgramUnaryFold<'p> = match operator {
    parser::LexOperator::Add => fold_prefix_plus as ProgramUnaryFold<'p>,
    parser::LexOperator::Subtract => fold_prefix_minus as ProgramUnaryFold<'p>,
    parser::LexOperator::ImplicitIntersection => {
      fold_prefix_implicit_intersection as ProgramUnaryFold<'p>
    }
    _ => return program_fail(input),
  };
  input.tokens.advance();
  Ok(Prefix(i64::from(prefix_binding_power()), fold))
}

fn parse_program_postfix<'p>(
  input: &mut ProgramSyntaxParser<'p>,
) -> ProgramParseResult<ProgramPostfix<'p>> {
  if let Some(token) = input.tokens.peek()
    && token.kind == parser::LexTokenKind::Operator(parser::LexOperator::Percent)
  {
    input.tokens.advance();
    return Ok(Postfix(
      i64::from(percent_postfix_binding_power()),
      fold_postfix_percent,
    ));
  }

  let Some(function) = input.tokens.consume_logical_function_call() else {
    return program_fail(input);
  };
  let fold: ProgramUnaryFold<'p> = match function {
    parser::LexLogicalFunction::And => fold_postfix_logical_and as ProgramUnaryFold<'p>,
    parser::LexLogicalFunction::Or => fold_postfix_logical_or as ProgramUnaryFold<'p>,
    parser::LexLogicalFunction::Not => fold_postfix_logical_not as ProgramUnaryFold<'p>,
  };
  Ok(Postfix(i64::from(logical_binding_power()), fold))
}

fn parse_program_infix<'p>(
  input: &mut ProgramSyntaxParser<'p>,
) -> ProgramParseResult<ProgramInfix<'p>> {
  let Some(token) = input.tokens.peek() else {
    return program_fail(input);
  };
  let parser::LexTokenKind::Operator(operator) = token.kind else {
    return program_fail(input);
  };
  let (left_bp, right_associative, fold): (u8, bool, ProgramBinaryFold<'p>) = match operator {
    parser::LexOperator::Equal => (2, false, fold_infix_equal),
    parser::LexOperator::NotEqual => (2, false, fold_infix_not_equal),
    parser::LexOperator::Less => (2, false, fold_infix_less),
    parser::LexOperator::LessOrEqual => (2, false, fold_infix_less_or_equal),
    parser::LexOperator::Greater => (2, false, fold_infix_greater),
    parser::LexOperator::GreaterOrEqual => (2, false, fold_infix_greater_or_equal),
    parser::LexOperator::Union => (4, false, fold_infix_union),
    parser::LexOperator::Intersection => (6, false, fold_infix_intersection),
    parser::LexOperator::Range => (8, false, fold_infix_range),
    parser::LexOperator::Concat => (10, false, fold_infix_concat),
    parser::LexOperator::Add => (12, false, fold_infix_add),
    parser::LexOperator::Subtract => (12, false, fold_infix_subtract),
    parser::LexOperator::Multiply => (14, false, fold_infix_multiply),
    parser::LexOperator::Divide => (14, false, fold_infix_divide),
    parser::LexOperator::Power => (16, true, fold_infix_power),
    parser::LexOperator::ImplicitIntersection | parser::LexOperator::Percent => {
      return program_fail(input);
    }
  };
  input.tokens.advance();
  if right_associative {
    Ok(Infix::Right(i64::from(left_bp), fold))
  } else {
    Ok(Infix::Left(i64::from(left_bp), fold))
  }
}

fn program_fail<T>(input: &ProgramSyntaxParser<'_>) -> ProgramParseResult<T> {
  Err(program_error(input))
}

fn program_error(input: &ProgramSyntaxParser<'_>) -> ContextError {
  ContextError::from_input(input)
}

fn fold_prefix_plus(
  input: &mut ProgramSyntaxParser<'_>,
  expr: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.unary(FormulaOperator::UnaryPlus, expr))
}

fn fold_prefix_minus(
  input: &mut ProgramSyntaxParser<'_>,
  expr: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.unary(FormulaOperator::UnaryMinus, expr))
}

fn fold_prefix_implicit_intersection(
  input: &mut ProgramSyntaxParser<'_>,
  expr: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(
    input
      .builder
      .unary(FormulaOperator::ImplicitIntersection, expr),
  )
}

fn fold_postfix_percent(
  input: &mut ProgramSyntaxParser<'_>,
  expr: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.unary(FormulaOperator::Percent, expr))
}

fn fold_postfix_logical_and(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  fold_postfix_logical(input, left, parser::LexLogicalFunction::And)
}

fn fold_postfix_logical_or(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  fold_postfix_logical(input, left, parser::LexLogicalFunction::Or)
}

fn fold_postfix_logical_not(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  fold_postfix_logical(input, left, parser::LexLogicalFunction::Not)
}

fn fold_postfix_logical(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  function: parser::LexLogicalFunction,
) -> ProgramParseResult<FormulaExprId> {
  let Some(args) = input.parse_argument_list(Some(left)) else {
    return program_fail(input);
  };
  let name = input.builder.intern(function.name());
  Ok(input.builder.push_value(FormulaNodeKind::Function {
    name: FormulaFunctionName::Unknown(name),
    args,
  }))
}

fn fold_infix_equal(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Equal, left, right))
}

fn fold_infix_not_equal(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::NotEqual, left, right))
}

fn fold_infix_less(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Less, left, right))
}

fn fold_infix_less_or_equal(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(
    input
      .builder
      .binary(FormulaOperator::LessOrEqual, left, right),
  )
}

fn fold_infix_greater(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Greater, left, right))
}

fn fold_infix_greater_or_equal(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(
    input
      .builder
      .binary(FormulaOperator::GreaterOrEqual, left, right),
  )
}

fn fold_infix_union(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Union, left, right))
}

fn fold_infix_intersection(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(
    input
      .builder
      .binary(FormulaOperator::Intersection, left, right),
  )
}

fn fold_infix_range(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Range, left, right))
}

fn fold_infix_concat(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Concat, left, right))
}

fn fold_infix_add(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Add, left, right))
}

fn fold_infix_subtract(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Subtract, left, right))
}

fn fold_infix_multiply(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Multiply, left, right))
}

fn fold_infix_divide(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Divide, left, right))
}

fn fold_infix_power(
  input: &mut ProgramSyntaxParser<'_>,
  left: FormulaExprId,
  right: FormulaExprId,
) -> ProgramParseResult<FormulaExprId> {
  Ok(input.builder.binary(FormulaOperator::Power, left, right))
}

#[derive(Clone, Debug)]
struct ProgramSyntaxCheckpoint {
  offset: usize,
  previous_end: usize,
  peeked: Option<parser::LexToken>,
}

impl Offset for ProgramSyntaxCheckpoint {
  fn offset_from(&self, start: &Self) -> usize {
    self.offset.saturating_sub(start.offset)
  }
}

impl Offset<ProgramSyntaxCheckpoint> for ProgramSyntaxParser<'_> {
  fn offset_from(&self, start: &ProgramSyntaxCheckpoint) -> usize {
    self.tokens.position().saturating_sub(start.offset)
  }
}

impl std::fmt::Debug for ProgramSyntaxParser<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ProgramSyntaxParser")
      .field("index", &self.tokens.position())
      .field("remaining", &self.tokens.eof_offset())
      .finish()
  }
}

impl<'p> Stream for ProgramSyntaxParser<'p> {
  type Token = parser::LexToken;
  type Slice = &'p str;
  type IterOffsets = ProgramSyntaxTokenOffsets<'p>;
  type Checkpoint = ProgramSyntaxCheckpoint;

  fn iter_offsets(&self) -> Self::IterOffsets {
    self.tokens.iter_offsets()
  }

  fn eof_offset(&self) -> usize {
    self.tokens.eof_offset()
  }

  fn next_token(&mut self) -> Option<Self::Token> {
    self.tokens.advance()
  }

  fn peek_token(&self) -> Option<Self::Token> {
    self.tokens.peek()
  }

  fn offset_for<P>(&self, predicate: P) -> Option<usize>
  where
    P: Fn(Self::Token) -> bool,
  {
    self
      .tokens
      .iter_offsets()
      .find_map(|(offset, token)| predicate(token).then_some(offset))
  }

  fn offset_at(&self, tokens: usize) -> Result<usize, Needed> {
    self
      .tokens
      .offset_after_tokens(tokens)
      .map(|offset| offset.saturating_sub(self.tokens.position()))
      .ok_or_else(|| Needed::new(tokens))
  }

  fn next_slice(&mut self, offset: usize) -> Self::Slice {
    self.tokens.next_slice(offset)
  }

  fn peek_slice(&self, offset: usize) -> Self::Slice {
    self.tokens.peek_slice(offset)
  }

  fn checkpoint(&self) -> Self::Checkpoint {
    self.tokens.checkpoint()
  }

  fn reset(&mut self, checkpoint: &Self::Checkpoint) {
    self.tokens.reset(checkpoint);
  }

  fn trace(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "ProgramSyntaxParser(index={}, remaining={})",
      self.tokens.position(),
      self.tokens.eof_offset()
    )
  }
}

impl StreamIsPartial for ProgramSyntaxParser<'_> {
  type PartialState = ();

  fn complete(&mut self) -> Self::PartialState {}

  fn restore_partial(&mut self, _state: Self::PartialState) {}

  fn is_partial_supported() -> bool {
    false
  }
}

struct ProgramSyntaxTokens<'p> {
  source: &'p str,
  offset: usize,
  previous_end: usize,
  peeked: Cell<Option<parser::LexToken>>,
}

impl<'p> ProgramSyntaxTokens<'p> {
  fn new(source: &'p str) -> Self {
    Self {
      source,
      offset: 0,
      previous_end: 0,
      peeked: Cell::new(None),
    }
  }

  fn is_end(&self) -> bool {
    self.peek().is_none()
  }

  fn eof_offset(&self) -> usize {
    self
      .peek()
      .map(|token| self.source.len().saturating_sub(token.start))
      .unwrap_or_default()
  }

  fn position(&self) -> usize {
    self.offset
  }

  fn peek(&self) -> Option<parser::LexToken> {
    if let Some(token) = self.peeked.get() {
      return Some(token);
    }
    let token = parser::lex_token_at(self.source, self.offset);
    self.peeked.set(token);
    token
  }

  fn advance(&mut self) -> Option<parser::LexToken> {
    let token = self.peek()?;
    self.offset = token.end;
    self.previous_end = token.end;
    self.peeked.set(None);
    Some(token)
  }

  fn ws_before_next(&self) -> bool {
    let token = match self.peek() {
      Some(token) => token,
      None => return false,
    };
    self
      .source
      .get(self.previous_end..token.start)
      .is_some_and(|text| !text.is_empty())
  }

  fn consume_token_kind(&mut self, kind: parser::LexTokenKind) -> Option<parser::LexToken> {
    let token = self.peek()?;
    if token.kind != kind {
      return None;
    }
    self.advance()
  }

  fn consume_logical_function_call(&mut self) -> Option<parser::LexLogicalFunction> {
    let token = self.peek()?;
    if token.kind != parser::LexTokenKind::Word {
      return None;
    }
    let word = self.source.get(token.start..token.end)?;
    let function = logical_function_name(word)?;
    let next = self.token_after(token.end)?;
    if next.kind != parser::LexTokenKind::ParenOpen {
      return None;
    }
    self.advance();
    Some(function)
  }

  fn token_after(&self, offset: usize) -> Option<parser::LexToken> {
    parser::lex_token_at(self.source, offset)
  }

  fn checkpoint(&self) -> ProgramSyntaxCheckpoint {
    ProgramSyntaxCheckpoint {
      offset: self.offset,
      previous_end: self.previous_end,
      peeked: self.peeked.get(),
    }
  }

  fn reset(&mut self, checkpoint: &ProgramSyntaxCheckpoint) {
    self.offset = checkpoint.offset.min(self.source.len());
    self.previous_end = checkpoint.previous_end.min(self.source.len());
    self.peeked.set(checkpoint.peeked);
  }

  fn iter_offsets(&self) -> ProgramSyntaxTokenOffsets<'p> {
    ProgramSyntaxTokenOffsets {
      source: self.source,
      offset: self.offset,
      token_offset: 0,
    }
  }

  fn offset_after_tokens(&self, tokens: usize) -> Option<usize> {
    let mut offset = self.offset;
    for _ in 0..tokens {
      offset = parser::lex_token_at(self.source, offset)?.end;
    }
    Some(offset)
  }

  fn next_slice(&mut self, tokens: usize) -> &'p str {
    let start = self.offset;
    let end = self
      .offset_after_tokens(tokens)
      .unwrap_or(self.source.len());
    self.offset = end;
    self.previous_end = end;
    self.peeked.set(None);
    self.source.get(start..end).unwrap_or_default()
  }

  fn peek_slice(&self, tokens: usize) -> &'p str {
    let start = self.offset;
    let end = self
      .offset_after_tokens(tokens)
      .unwrap_or(self.source.len());
    self.source.get(start..end).unwrap_or_default()
  }
}

struct ProgramSyntaxTokenOffsets<'p> {
  source: &'p str,
  offset: usize,
  token_offset: usize,
}

impl Iterator for ProgramSyntaxTokenOffsets<'_> {
  type Item = (usize, parser::LexToken);

  fn next(&mut self) -> Option<Self::Item> {
    let token = parser::lex_token_at(self.source, self.offset)?;
    let offset = self.token_offset;
    self.offset = token.end;
    self.token_offset += 1;
    Some((offset, token))
  }
}

fn logical_function_name(value: &str) -> Option<parser::LexLogicalFunction> {
  if value.eq_ignore_ascii_case("AND") {
    Some(parser::LexLogicalFunction::And)
  } else if value.eq_ignore_ascii_case("OR") {
    Some(parser::LexLogicalFunction::Or)
  } else if value.eq_ignore_ascii_case("NOT") {
    Some(parser::LexLogicalFunction::Not)
  } else {
    None
  }
}

fn logical_binding_power() -> u8 {
  1
}

fn infix_binding_power(operator: parser::LexOperator) -> Option<(u8, u8)> {
  match operator {
    parser::LexOperator::Equal
    | parser::LexOperator::NotEqual
    | parser::LexOperator::Less
    | parser::LexOperator::LessOrEqual
    | parser::LexOperator::Greater
    | parser::LexOperator::GreaterOrEqual => Some((2, 3)),
    parser::LexOperator::Union => Some((4, 5)),
    parser::LexOperator::Intersection => Some((6, 7)),
    parser::LexOperator::Range => Some((8, 9)),
    parser::LexOperator::Concat => Some((10, 11)),
    parser::LexOperator::Add | parser::LexOperator::Subtract => Some((12, 13)),
    parser::LexOperator::Multiply | parser::LexOperator::Divide => Some((14, 15)),
    parser::LexOperator::Power => Some((16, 16)),
    parser::LexOperator::ImplicitIntersection | parser::LexOperator::Percent => None,
  }
}

fn prefix_binding_power() -> u8 {
  17
}

fn percent_postfix_binding_power() -> u8 {
  18
}

fn token_span(token: parser::LexToken) -> parser::SemanticSpan {
  parser::SemanticSpan {
    start: token.start,
    end: token.end,
  }
}

fn is_intersection_rhs_start(token: Option<parser::LexToken>) -> bool {
  token.is_some_and(|token| {
    matches!(
      token.kind,
      parser::LexTokenKind::Text
        | parser::LexTokenKind::Number(_)
        | parser::LexTokenKind::Error(_)
        | parser::LexTokenKind::ArrayOpen
        | parser::LexTokenKind::ParenOpen
        | parser::LexTokenKind::Word
    )
  })
}

fn finish_array_row(cols: Option<usize>, row_len: usize) -> Option<Option<usize>> {
  match cols {
    Some(cols) if cols != row_len => None,
    Some(cols) => Some(Some(cols)),
    None => Some(Some(row_len)),
  }
}

fn lower_parser_word(
  builder: &mut FormulaProgramBuilder,
  source: FormulaSource<'_>,
  body_start: usize,
  span: parser::SemanticSpan,
  kind: parser::SemanticWordKind,
) -> Option<FormulaExprId> {
  let word = source
    .text
    .get(body_start + span.start..body_start + span.end)?;
  match kind {
    parser::SemanticWordKind::Boolean(value) => Some(builder.boolean(value)),
    parser::SemanticWordKind::ReferenceCandidate => {
      let sheet = match source.context.position {
        FormulaSourcePosition::Cell(cell) => cell.sheet,
        FormulaSourcePosition::Sheet(sheet) => sheet,
      };
      if let Some(range) = parser::parse_formula_range(sheet, word) {
        let reference = reference_from_qualified_range(builder, range);
        return Some(builder.push_reference(reference));
      }
      Some(named_reference_with_span(
        builder,
        word,
        Some(source_span(body_start, span)),
      ))
    }
    parser::SemanticWordKind::Name => Some(named_reference_with_span(
      builder,
      word,
      Some(source_span(body_start, span)),
    )),
    parser::SemanticWordKind::ExternalReference(reference) => {
      let reference = external_reference_from_spans(builder, word, reference)?;
      Some(builder.push_reference(reference))
    }
  }
}

fn named_reference_with_span(
  builder: &mut FormulaProgramBuilder,
  name: &str,
  span: Option<SourceSpan>,
) -> FormulaExprId {
  let name = builder.intern(name);
  builder.push(
    FormulaNodeKind::Reference(FormulaReference::Named(FormulaNamedReference {
      name,
      scope: FormulaNameScope::Workbook,
    })),
    span,
    FormulaNodeMetadata {
      labels: FormulaNodeLabels::RETURNS_REFERENCE
        | FormulaNodeLabels::CONTAINS_REFERENCE
        | FormulaNodeLabels::CONTAINS_NAME,
      operand_class: FormulaOperandClass::Reference,
      param_class: FormulaParamClass::Unknown,
    },
  )
}

fn source_span(body_start: usize, span: parser::SemanticSpan) -> SourceSpan {
  SourceSpan {
    start: body_start + span.start,
    end: body_start + span.end,
  }
}

fn external_reference_from_spans(
  builder: &mut FormulaProgramBuilder,
  source: &str,
  reference: parser::ExternalReferenceSpans,
) -> Option<FormulaReference> {
  let book = builder.intern(span_text(source, reference.book));
  let sheet = reference
    .sheet
    .map(|sheet| FormulaSheetName::Name(builder.intern(&external_sheet_text(source, sheet))));
  let name = span_text(source, reference.name?);

  if let Some(range) = parser::parse_formula_range(SheetId::default(), name) {
    let sheet = FormulaSheetReference::External {
      book,
      sheet: sheet.map(FormulaSheetRange::Sheet),
    };
    let start_flags = valid_address_flags(range.start_flags, true);
    let end_flags = valid_address_flags(range.end_flags, true);
    let reference_flags = reference_flags(range.start_flags, range.end_flags);
    if range.range.start == range.range.end
      && !range.start_flags.whole_column
      && !range.start_flags.whole_row
      && !range.end_flags.whole_column
      && !range.end_flags.whole_row
    {
      return Some(FormulaReference::Cell(FormulaCellReference {
        target: FormulaReferencePoint {
          sheet,
          address: range.range.start,
          flags: start_flags,
        },
        flags: reference_flags,
      }));
    }
    return Some(FormulaReference::Range(FormulaRangeReference {
      start: FormulaReferencePoint {
        sheet,
        address: range.range.start,
        flags: start_flags,
      },
      end: FormulaReferencePoint {
        sheet,
        address: range.range.end,
        flags: end_flags,
      },
      flags: reference_flags,
    }));
  }

  Some(FormulaReference::ExternalName(
    FormulaExternalNameReference {
      book,
      sheet,
      name: builder.intern(name),
    },
  ))
}

fn span_text(source: &str, span: parser::SemanticSpan) -> &str {
  source.get(span.start..span.end).unwrap_or_default()
}

fn external_sheet_text(source: &str, span: parser::SemanticSpan) -> String {
  let text = span_text(source, span);
  if text.contains("''") {
    text.replace("''", "'")
  } else {
    text.to_string()
  }
}

fn reference_from_qualified_range(
  builder: &mut FormulaProgramBuilder,
  range: QualifiedRange<'_>,
) -> FormulaReference {
  let sheet = sheet_reference_from_qualified_range(builder, &range);
  let start_flags = valid_address_flags(range.start_flags, sheet != FormulaSheetReference::Current);
  let end_flags = valid_address_flags(range.end_flags, sheet != FormulaSheetReference::Current);
  let reference_flags = reference_flags(range.start_flags, range.end_flags);
  if range.range.start == range.range.end
    && !range.start_flags.whole_column
    && !range.start_flags.whole_row
    && !range.end_flags.whole_column
    && !range.end_flags.whole_row
  {
    FormulaReference::Cell(FormulaCellReference {
      target: FormulaReferencePoint {
        sheet,
        address: range.range.start,
        flags: start_flags,
      },
      flags: reference_flags,
    })
  } else {
    FormulaReference::Range(FormulaRangeReference {
      start: FormulaReferencePoint {
        sheet,
        address: range.range.start,
        flags: start_flags,
      },
      end: FormulaReferencePoint {
        sheet,
        address: range.range.end,
        flags: end_flags,
      },
      flags: reference_flags,
    })
  }
}

fn sheet_reference_from_qualified_range(
  builder: &mut FormulaProgramBuilder,
  range: &QualifiedRange<'_>,
) -> FormulaSheetReference {
  let Some(start) = range.sheet_name.as_ref() else {
    return FormulaSheetReference::Current;
  };
  let start = FormulaSheetName::Name(builder.intern(start.0.as_ref()));
  let sheet = if let Some(end) = range.end_sheet_name.as_ref() {
    FormulaSheetRange::Range {
      start,
      end: FormulaSheetName::Name(builder.intern(end.0.as_ref())),
    }
  } else {
    FormulaSheetRange::Sheet(start)
  };
  FormulaSheetReference::Local(sheet)
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

fn print_structured_reference_column(value: &str, output: &mut String) {
  for ch in value.chars() {
    match ch {
      '\'' | '#' | '[' | ']' => {
        output.push('\'');
        output.push(ch);
      }
      _ => output.push(ch),
    }
  }
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

fn argument_separator(grammar: FormulaPrintGrammar) -> &'static str {
  match grammar {
    FormulaPrintGrammar::OpenFormula | FormulaPrintGrammar::CalcA1 => ";",
    FormulaPrintGrammar::ExcelA1 | FormulaPrintGrammar::ExcelR1C1 => ",",
  }
}
