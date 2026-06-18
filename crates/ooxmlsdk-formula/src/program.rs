use bitflags::bitflags;

use crate::symbol::{FormulaSymbolId, FormulaSymbolPool};
use crate::{CellAddress, CellRange, FormulaErrorValue, FormulaOperator, SheetId};

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
