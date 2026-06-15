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
    Ok(QualifiedAddress::parse_a1(SheetId::default(), value)?.cell)
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
  pub fn new(start: CellAddress, end: CellAddress) -> Self {
    Self { start, end }
  }

  pub fn parse_a1(value: &str) -> Result<Self> {
    Ok(QualifiedRange::parse_a1(SheetId::default(), value)?.range)
  }

  pub fn cell_count_hint(self) -> u64 {
    let columns = self.start.column.abs_diff(self.end.column) as u64 + 1;
    let rows = self.start.row.abs_diff(self.end.row) as u64 + 1;
    columns.saturating_mul(rows)
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SheetName<'a>(pub Cow<'a, str>);

impl<'a> SheetName<'a> {
  pub(crate) fn into_owned(self) -> SheetName<'static> {
    SheetName(Cow::Owned(self.0.into_owned()))
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QualifiedAddress<'a> {
  pub sheet: SheetId,
  pub sheet_name: Option<SheetName<'a>>,
  pub cell: CellAddress,
  pub flags: AddressFlags,
}

impl<'a> QualifiedAddress<'a> {
  pub fn parse_a1(sheet: SheetId, value: &str) -> Result<Self> {
    let (sheet_name, cell) = split_sheet_name(value.trim());
    let (cell, flags) = parse_cell_ref(cell.trim())?;
    Ok(Self {
      sheet,
      sheet_name: sheet_name.map(|name| SheetName(Cow::Owned(unquote_sheet_name(name)))),
      cell,
      flags,
    })
  }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QualifiedRange<'a> {
  pub sheet: SheetId,
  pub sheet_name: Option<SheetName<'a>>,
  pub range: CellRange,
  pub start_flags: AddressFlags,
  pub end_flags: AddressFlags,
}

impl<'a> QualifiedRange<'a> {
  pub(crate) fn into_owned(self) -> QualifiedRange<'static> {
    QualifiedRange {
      sheet: self.sheet,
      sheet_name: self.sheet_name.map(SheetName::into_owned),
      range: self.range,
      start_flags: self.start_flags,
      end_flags: self.end_flags,
    }
  }
}

impl<'a> QualifiedRange<'a> {
  pub fn parse_a1(sheet: SheetId, value: &str) -> Result<Self> {
    let (sheet_name, range_text) = split_sheet_name(value.trim());
    let (start_text, end_text) = range_text
      .split_once(':')
      .unwrap_or((range_text, range_text));
    let (start, start_flags, end, end_flags) =
      parse_range_bounds(start_text.trim(), end_text.trim())?;
    Ok(Self {
      sheet,
      sheet_name: sheet_name.map(|name| SheetName(Cow::Owned(unquote_sheet_name(name)))),
      range: CellRange { start, end },
      start_flags,
      end_flags,
    })
  }
}

const EXCEL_MAX_COLUMN_ZERO_BASED: u32 = 16_383;
const EXCEL_MAX_ROW_ZERO_BASED: u32 = 1_048_575;

fn split_sheet_name(value: &str) -> (Option<&str>, &str) {
  let mut quoted = false;
  let mut last_bang = None;
  let mut last_dot = None;
  let mut chars = value.char_indices().peekable();
  while let Some((index, ch)) = chars.next() {
    match ch {
      '\'' => {
        if quoted && chars.peek().is_some_and(|(_, next)| *next == '\'') {
          chars.next();
        } else {
          quoted = !quoted;
        }
      }
      '!' if !quoted => last_bang = Some(index),
      '.' if !quoted => last_dot = Some(index),
      _ => {}
    }
  }
  last_bang
    .or(last_dot)
    .filter(|index| *index > 0)
    .map(|index| (Some(&value[..index]), &value[index + 1..]))
    .unwrap_or((None, value.trim_start_matches('.')))
}

fn unquote_sheet_name(value: &str) -> String {
  let trimmed = value.trim();
  if trimmed.len() >= 2 && trimmed.starts_with('\'') && trimmed.ends_with('\'') {
    trimmed[1..trimmed.len() - 1].replace("''", "'")
  } else {
    trimmed.to_string()
  }
}

fn parse_range_bounds(
  start: &str,
  end: &str,
) -> Result<(CellAddress, AddressFlags, CellAddress, AddressFlags)> {
  match (parse_cell_ref(start), parse_cell_ref(end)) {
    (Ok((start, start_flags)), Ok((end, end_flags))) => Ok((start, start_flags, end, end_flags)),
    _ => parse_whole_axis_range(start, end),
  }
}

fn parse_whole_axis_range(
  start: &str,
  end: &str,
) -> Result<(CellAddress, AddressFlags, CellAddress, AddressFlags)> {
  if let (Some((start_col, start_abs)), Some((end_col, end_abs))) =
    (parse_column_ref(start), parse_column_ref(end))
  {
    return Ok((
      CellAddress {
        column: start_col,
        row: 0,
      },
      AddressFlags {
        absolute_column: start_abs,
        whole_column: true,
        ..AddressFlags::default()
      },
      CellAddress {
        column: end_col,
        row: EXCEL_MAX_ROW_ZERO_BASED,
      },
      AddressFlags {
        absolute_column: end_abs,
        whole_column: true,
        ..AddressFlags::default()
      },
    ));
  }
  if let (Some((start_row, start_abs)), Some((end_row, end_abs))) =
    (parse_row_ref(start), parse_row_ref(end))
  {
    return Ok((
      CellAddress {
        column: 0,
        row: start_row,
      },
      AddressFlags {
        absolute_row: start_abs,
        whole_row: true,
        ..AddressFlags::default()
      },
      CellAddress {
        column: EXCEL_MAX_COLUMN_ZERO_BASED,
        row: end_row,
      },
      AddressFlags {
        absolute_row: end_abs,
        whole_row: true,
        ..AddressFlags::default()
      },
    ));
  }
  Err(FormulaError::InvalidAddress(format!("{start}:{end}")))
}

fn parse_cell_ref(value: &str) -> Result<(CellAddress, AddressFlags)> {
  let mut text = value.trim();
  let mut flags = AddressFlags::default();
  if let Some(rest) = text.strip_prefix('$') {
    flags.absolute_column = true;
    text = rest;
  }

  let split = text
    .char_indices()
    .find(|(_, ch)| !ch.is_ascii_alphabetic())
    .map(|(index, _)| index)
    .unwrap_or(text.len());
  let (column_text, mut row_text) = text.split_at(split);
  if let Some(rest) = row_text.strip_prefix('$') {
    flags.absolute_row = true;
    row_text = rest;
  }
  if column_text.is_empty() || row_text.is_empty() {
    return Err(FormulaError::InvalidAddress(value.to_string()));
  }
  let column = parse_column_letters(column_text)
    .ok_or_else(|| FormulaError::InvalidAddress(value.to_string()))?;
  let row =
    parse_row_digits(row_text).ok_or_else(|| FormulaError::InvalidAddress(value.to_string()))?;
  Ok((CellAddress { column, row }, flags))
}

fn parse_column_ref(value: &str) -> Option<(u32, bool)> {
  let (text, absolute) = value
    .trim()
    .strip_prefix('$')
    .map(|text| (text, true))
    .unwrap_or((value.trim(), false));
  parse_column_letters(text).map(|column| (column, absolute))
}

fn parse_row_ref(value: &str) -> Option<(u32, bool)> {
  let (text, absolute) = value
    .trim()
    .strip_prefix('$')
    .map(|text| (text, true))
    .unwrap_or((value.trim(), false));
  parse_row_digits(text).map(|row| (row, absolute))
}

fn parse_column_letters(value: &str) -> Option<u32> {
  if value.is_empty() || !value.chars().all(|ch| ch.is_ascii_alphabetic()) {
    return None;
  }
  let mut column = 0u32;
  for ch in value.chars() {
    column = column
      .checked_mul(26)?
      .checked_add(ch.to_ascii_uppercase() as u32 - 'A' as u32 + 1)?;
  }
  let column = column.checked_sub(1)?;
  (column <= EXCEL_MAX_COLUMN_ZERO_BASED).then_some(column)
}

fn parse_row_digits(value: &str) -> Option<u32> {
  if value.is_empty() || !value.chars().all(|ch| ch.is_ascii_digit()) {
    return None;
  }
  let row = value.parse::<u32>().ok()?.checked_sub(1)?;
  (row <= EXCEL_MAX_ROW_ZERO_BASED).then_some(row)
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
    assert_eq!(
      CellAddress::parse_a1("XFD1048576").unwrap(),
      CellAddress {
        column: EXCEL_MAX_COLUMN_ZERO_BASED,
        row: EXCEL_MAX_ROW_ZERO_BASED,
      }
    );
    assert!(CellAddress::parse_a1("XFE1").is_err());
    assert!(CellAddress::parse_a1("A1048577").is_err());
    assert!(CellAddress::parse_a1("column1").is_err());
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

  #[test]
  fn parses_qualified_and_axis_ranges() {
    let range = QualifiedRange::parse_a1(SheetId(7), "'Q1 Report'!$A:$C").unwrap();
    assert_eq!(range.sheet, SheetId(7));
    assert_eq!(range.sheet_name.unwrap().0, "Q1 Report");
    assert_eq!(range.range.start.column, 0);
    assert_eq!(range.range.end.column, 2);
    assert!(range.start_flags.absolute_column);
    assert!(range.start_flags.whole_column);

    let rows = QualifiedRange::parse_a1(SheetId(1), "$2:$4").unwrap();
    assert_eq!(rows.range.start.row, 1);
    assert_eq!(rows.range.end.row, 3);
    assert_eq!(rows.range.end.column, EXCEL_MAX_COLUMN_ZERO_BASED);
    assert!(rows.end_flags.whole_row);
  }
}
