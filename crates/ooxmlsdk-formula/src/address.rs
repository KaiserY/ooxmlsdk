use std::borrow::Cow;

use crate::{FormulaError, Result};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SheetId(pub u32);

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CellAddress {
  pub column: u32,
  pub row: u32,
}

impl CellAddress {
  pub fn parse_a1(value: &str) -> Result<Self> {
    let trimmed = value.trim();
    let cell = trimmed
      .rsplit_once('!')
      .map(|(_, cell)| cell)
      .unwrap_or(trimmed)
      .trim_matches('\'');
    let mut column = 0u32;
    let mut row = 0u32;
    let mut saw_column = false;
    let mut saw_row = false;

    for ch in cell.chars().filter(|ch| *ch != '$') {
      if ch.is_ascii_alphabetic() {
        if saw_row {
          return Err(FormulaError::InvalidAddress(value.to_string()));
        }
        saw_column = true;
        column = column
          .checked_mul(26)
          .and_then(|base| base.checked_add(ch.to_ascii_uppercase() as u32 - 'A' as u32 + 1))
          .ok_or_else(|| FormulaError::InvalidAddress(value.to_string()))?;
      } else if ch.is_ascii_digit() {
        saw_row = true;
        row = row
          .checked_mul(10)
          .and_then(|base| base.checked_add(ch as u32 - '0' as u32))
          .ok_or_else(|| FormulaError::InvalidAddress(value.to_string()))?;
      } else {
        return Err(FormulaError::InvalidAddress(value.to_string()));
      }
    }

    if !saw_column || !saw_row || column == 0 || row == 0 {
      return Err(FormulaError::InvalidAddress(value.to_string()));
    }

    Ok(Self {
      column: column - 1,
      row: row - 1,
    })
  }
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

impl CellRange {
  pub fn parse_a1(value: &str) -> Result<Self> {
    let range = value
      .rsplit_once('!')
      .map(|(_, range)| range)
      .unwrap_or(value);
    let (start, end) = range.split_once(':').unwrap_or((range, range));
    Ok(Self {
      start: CellAddress::parse_a1(start)?,
      end: CellAddress::parse_a1(end)?,
    })
  }
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parses_a1_cell_addresses_as_zero_based_indexes() {
    assert_eq!(
      CellAddress::parse_a1("$AA$12").unwrap(),
      CellAddress {
        column: 26,
        row: 11
      }
    );
    assert_eq!(
      CellAddress::parse_a1("'Sheet 1'!C3").unwrap(),
      CellAddress { column: 2, row: 2 }
    );
  }

  #[test]
  fn parses_a1_ranges() {
    assert_eq!(
      CellRange::parse_a1("B2:D4").unwrap(),
      CellRange {
        start: CellAddress { column: 1, row: 1 },
        end: CellAddress { column: 3, row: 3 },
      }
    );
  }
}
