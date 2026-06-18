#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TableReferenceItems(u8);

impl TableReferenceItems {
  pub(crate) const TABLE: Self = Self(0);
  pub(crate) const ALL: Self = Self(1);
  pub(crate) const HEADERS: Self = Self(2);
  pub(crate) const DATA: Self = Self(4);
  pub(crate) const TOTALS: Self = Self(8);
  pub(crate) const THIS_ROW: Self = Self(16);

  fn add(&mut self, item: Self) {
    self.0 |= item.0;
  }

  pub(crate) fn contains(self, item: Self) -> bool {
    self.0 & item.0 != 0
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TableReferenceColumn<'a> {
  Borrowed(&'a str),
  Owned(String),
}

impl<'a> TableReferenceColumn<'a> {
  pub(crate) fn as_ref(&self) -> &str {
    match self {
      Self::Borrowed(value) => value,
      Self::Owned(value) => value.as_str(),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TableReferenceSelection<'a> {
  pub(crate) table_name: &'a str,
  pub(crate) items: TableReferenceItems,
  pub(crate) columns: Vec<TableReferenceColumn<'a>>,
}

pub(crate) fn parse_table_reference_selection(text: &str) -> Option<TableReferenceSelection<'_>> {
  let (table_name, selector) = text.split_once('[')?;
  let specifiers = table_reference_specifiers(selector)?;
  let mut items = TableReferenceItems::TABLE;
  let mut columns = Vec::new();
  for specifier in specifiers {
    match specifier.as_ref() {
      value if value.eq_ignore_ascii_case("#ALL") => items.add(TableReferenceItems::ALL),
      value if value.eq_ignore_ascii_case("#HEADERS") => {
        items.add(TableReferenceItems::HEADERS);
      }
      value if value.eq_ignore_ascii_case("#DATA") => items.add(TableReferenceItems::DATA),
      value if value.eq_ignore_ascii_case("#TOTALS") => items.add(TableReferenceItems::TOTALS),
      value if value.eq_ignore_ascii_case("#THIS ROW") => {
        items.add(TableReferenceItems::THIS_ROW);
      }
      _ => columns.push(specifier),
    }
  }
  Some(TableReferenceSelection {
    table_name,
    items,
    columns,
  })
}

fn table_reference_specifiers(selector_tail: &str) -> Option<Vec<TableReferenceColumn<'_>>> {
  let selector = selector_tail.trim();
  let selector = selector.strip_suffix(']')?;
  if selector.is_empty() {
    return Some(Vec::new());
  }
  if !selector.starts_with('[') {
    return Some(vec![table_reference_column(selector)]);
  }
  let mut specifiers = Vec::new();
  let mut depth = 0i32;
  let mut start = None;
  for (index, ch) in selector.char_indices() {
    match ch {
      '[' => {
        if depth == 0 {
          start = Some(index + 1);
        }
        depth += 1;
      }
      ']' => {
        depth -= 1;
        if depth == 0 {
          let start = start.take()?;
          specifiers.push(table_reference_column(&selector[start..index]));
        }
      }
      _ => {}
    }
  }
  if depth != 0 {
    return None;
  }
  if specifiers.is_empty() {
    Some(vec![table_reference_column(selector)])
  } else {
    Some(specifiers)
  }
}

fn table_reference_column(value: &str) -> TableReferenceColumn<'_> {
  if !value.contains('\'') {
    return TableReferenceColumn::Borrowed(value);
  }
  let mut result = String::new();
  let mut escaped = false;
  for ch in value.chars() {
    if escaped {
      result.push(ch);
      escaped = false;
    } else if ch == '\'' {
      escaped = true;
    } else {
      result.push(ch);
    }
  }
  TableReferenceColumn::Owned(result)
}
