use std::borrow::Cow;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SheetId(pub u32);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CellAddress {
  pub column: u32,
  pub row: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct AddressFlags {
  pub absolute_column: bool,
  pub absolute_row: bool,
  pub whole_column: bool,
  pub whole_row: bool,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct CellRange {
  pub start: CellAddress,
  pub end: CellAddress,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SheetName<'a>(pub Cow<'a, str>);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QualifiedAddress<'a> {
  pub sheet: SheetId,
  pub sheet_name: Option<SheetName<'a>>,
  pub cell: CellAddress,
  pub flags: AddressFlags,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QualifiedRange<'a> {
  pub sheet: SheetId,
  pub sheet_name: Option<SheetName<'a>>,
  pub range: CellRange,
  pub start_flags: AddressFlags,
  pub end_flags: AddressFlags,
}
