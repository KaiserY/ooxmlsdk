use crate::CellAddress;

pub(crate) fn translate_shared_formula_text(
  formula: &str,
  origin: CellAddress,
  target: CellAddress,
) -> String {
  let delta_col = i64::from(target.column) - i64::from(origin.column);
  let delta_row = i64::from(target.row) - i64::from(origin.row);
  if delta_col == 0 && delta_row == 0 {
    return formula.to_string();
  }

  let mut output = String::with_capacity(formula.len());
  let mut index = 0;
  while let Some((ch, next)) = next_char(formula, index) {
    match ch {
      '"' => {
        let start = index;
        index = skip_quoted(formula, index, '"');
        output.push_str(&formula[start..index]);
      }
      '\'' => {
        let start = index;
        index = skip_quoted(formula, index, '\'');
        if next_char(formula, index).is_some_and(|(ch, _)| ch == '!') {
          index += '!'.len_utf8();
          while let Some((ch, next)) = next_char(formula, index) {
            if !is_a1_tail_char(ch) {
              break;
            }
            index = next;
          }
          output.push_str(&translate_reference_token(
            &formula[start..index],
            delta_col,
            delta_row,
          ));
        } else {
          output.push_str(&formula[start..index]);
        }
      }
      ch if is_formula_token_start(ch) => {
        let start = index;
        index = next;
        while let Some((ch, next)) = next_char(formula, index) {
          if !is_formula_token_char(ch) {
            break;
          }
          index = next;
        }
        output.push_str(&translate_reference_token(
          &formula[start..index],
          delta_col,
          delta_row,
        ));
      }
      _ => {
        output.push_str(&formula[index..next]);
        index = next;
      }
    }
  }
  output
}

fn next_char(source: &str, index: usize) -> Option<(char, usize)> {
  let ch = source.get(index..)?.chars().next()?;
  Some((ch, index + ch.len_utf8()))
}

fn skip_quoted(source: &str, mut index: usize, quote: char) -> usize {
  index += quote.len_utf8();
  while let Some((ch, next)) = next_char(source, index) {
    index = next;
    if ch == quote {
      if next_char(source, index).is_some_and(|(next, _)| next == quote) {
        index += quote.len_utf8();
        continue;
      }
      break;
    }
  }
  index
}

fn is_formula_token_start(ch: char) -> bool {
  ch.is_ascii_alphabetic() || ch == '$' || ch == '[' || ch == '_'
}

fn is_formula_token_char(ch: char) -> bool {
  ch.is_ascii_alphanumeric() || matches!(ch, '$' | '.' | '_' | ':' | '!' | '[' | ']')
}

fn is_a1_tail_char(ch: char) -> bool {
  ch.is_ascii_alphanumeric() || matches!(ch, '$' | ':' | '.')
}

fn translate_reference_token(token: &str, delta_col: i64, delta_row: i64) -> String {
  if token.contains('[') && !token.starts_with('[') {
    return token.to_string();
  }
  let Some((prefix, range)) = split_reference_prefix(token) else {
    return token.to_string();
  };
  let Some(translated) = translate_a1_range(range, delta_col, delta_row) else {
    return token.to_string();
  };
  format!("{prefix}{translated}")
}

fn split_reference_prefix(token: &str) -> Option<(&str, &str)> {
  if let Some((prefix, range)) = token.rsplit_once('!') {
    return Some((&token[..prefix.len() + 1], range));
  }
  Some(("", token))
}

fn translate_a1_range(range: &str, delta_col: i64, delta_row: i64) -> Option<String> {
  let (start, end) = range.split_once(':').unwrap_or((range, ""));
  let start = translate_a1_reference(start, delta_col, delta_row)?;
  if end.is_empty() {
    return Some(start);
  }
  let end = translate_a1_reference(end, delta_col, delta_row)?;
  Some(format!("{start}:{end}"))
}

fn translate_a1_reference(reference: &str, delta_col: i64, delta_row: i64) -> Option<String> {
  let parsed = A1Reference::parse(reference)?;
  Some(parsed.translate(delta_col, delta_row).format())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct A1Reference {
  absolute_col: bool,
  col: u32,
  absolute_row: bool,
  row: u32,
}

impl A1Reference {
  fn parse(reference: &str) -> Option<Self> {
    let mut chars = reference.chars().peekable();
    let absolute_col = chars.next_if_eq(&'$').is_some();
    let mut col = 0u32;
    while chars.peek().is_some_and(|ch| ch.is_ascii_alphabetic()) {
      let ch = chars.next()?.to_ascii_uppercase();
      col = col
        .saturating_mul(26)
        .saturating_add(ch as u32 - 'A' as u32 + 1);
    }
    let absolute_row = chars.next_if_eq(&'$').is_some();
    let mut row = 0u32;
    while chars.peek().is_some_and(|ch| ch.is_ascii_digit()) {
      let ch = chars.next()?;
      row = row
        .saturating_mul(10)
        .saturating_add(ch as u32 - '0' as u32);
    }
    (col > 0 && row > 0 && chars.next().is_none()).then_some(Self {
      absolute_col,
      col,
      absolute_row,
      row,
    })
  }

  fn translate(self, delta_col: i64, delta_row: i64) -> Self {
    Self {
      absolute_col: self.absolute_col,
      col: if self.absolute_col {
        self.col
      } else {
        translate_one_based_index(self.col, delta_col)
      },
      absolute_row: self.absolute_row,
      row: if self.absolute_row {
        self.row
      } else {
        translate_one_based_index(self.row, delta_row)
      },
    }
  }

  fn format(self) -> String {
    format!(
      "{}{}{}{}",
      if self.absolute_col { "$" } else { "" },
      column_name(self.col),
      if self.absolute_row { "$" } else { "" },
      self.row
    )
  }
}

fn translate_one_based_index(index: u32, delta: i64) -> u32 {
  u32::try_from((i64::from(index) + delta).max(1)).unwrap_or(u32::MAX)
}

fn column_name(mut col: u32) -> String {
  let mut chars = Vec::new();
  while col > 0 {
    col -= 1;
    chars.push(char::from_u32('A' as u32 + col % 26).unwrap_or('A'));
    col /= 26;
  }
  chars.into_iter().rev().collect()
}
