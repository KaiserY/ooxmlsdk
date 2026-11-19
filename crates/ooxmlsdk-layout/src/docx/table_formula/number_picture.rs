//! Word numeric pictures (ECMA-376 §17.16.4.2), independently of spreadsheet
//! formats: `#` pads with spaces and `%` is a literal, not a scale operation.
use ooxmlsdk_formula::calc::numeric::round_to_decimal_places;

use crate::localization::OfficeNumberSeparators;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Item {
  Zero,
  Optional,
  Truncate,
  Decimal,
  Group,
  Plus,
  Minus,
  Literal(char),
}

impl Item {
  fn digit(self) -> bool {
    matches!(self, Self::Zero | Self::Optional | Self::Truncate)
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Picture {
  sections: Vec<Vec<Item>>,
}

impl Picture {
  pub(super) fn parse(text: &str, symbols: OfficeNumberSeparators) -> Option<Self> {
    if text.is_empty() || text.len() > 4096 {
      return None;
    }
    let mut sections = vec![Vec::new()];
    let mut quoted = false;
    for ch in text.chars() {
      if ch == '\'' {
        quoted = !quoted;
        continue;
      }
      if !quoted && ch == ';' {
        if sections.len() == 3 {
          return None;
        }
        sections.push(Vec::new());
        continue;
      }
      let item = if quoted {
        Item::Literal(ch)
      } else {
        match ch {
          '0' => Item::Zero,
          '#' => Item::Optional,
          'x' => Item::Truncate,
          '+' => Item::Plus,
          '-' => Item::Minus,
          ch if ch == symbols.decimal => Item::Decimal,
          ch if ch == symbols.grouping => Item::Group,
          // Numbered-item references need their own SEQ/caption owner.
          '`' | '\\' | '\r' | '\n' => return None,
          ch => Item::Literal(ch),
        }
      };
      sections.last_mut()?.push(item);
    }
    if quoted
      || sections.iter().any(|items| {
        let mut decimals = items.iter().filter(|&&item| item == Item::Decimal);
        decimals.next().is_some() && decimals.next().is_some()
          || items
            .iter()
            .position(|&item| item == Item::Decimal)
            .is_some_and(|index| {
              items[index + 1..]
                .iter()
                .filter(|item| item.digit())
                .count()
                > 15
            })
      })
    {
      return None;
    }
    Some(Self { sections })
  }

  pub(super) fn format(&self, value: f64, symbols: OfficeNumberSeparators) -> String {
    let section = if value == 0.0 && self.sections.len() == 3 {
      2
    } else if value < 0.0 && self.sections.len() >= 2 {
      1
    } else {
      0
    };
    let items = &self.sections[section];
    let decimal = items
      .iter()
      .position(|&item| item == Item::Decimal)
      .unwrap_or(items.len());
    let precision = items[decimal..].iter().filter(|item| item.digit()).count();
    let rounded = round_to_decimal_places(value.abs(), precision as i32);
    let digits = format!("{rounded:.precision$}");
    let (integral, fractional) = digits.split_once('.').unwrap_or((&digits, ""));
    let integral = if rounded == 0.0 { "" } else { integral };
    let fractional = if rounded == 0.0 && precision > 0 {
      "0"
    } else {
      fractional.trim_end_matches('0')
    };
    let negative = value < 0.0 && rounded != 0.0;
    let truncate = items[..decimal].contains(&Item::Truncate);
    let automatic_sign = negative
      && section == 0
      && !truncate
      && !items
        .iter()
        .any(|item| matches!(item, Item::Plus | Item::Minus));
    let grouped = items[..decimal].contains(&Item::Group);
    let mut slots = vec![String::new(); items.len()];
    let mut cursor = integral.len();
    let mut first = None;
    for (index, item) in items[..decimal].iter().enumerate().rev() {
      if !item.digit() {
        continue;
      }
      first = Some(index);
      let digit = if cursor > 0 {
        cursor -= 1;
        char::from(integral.as_bytes()[cursor])
      } else if *item == Item::Zero {
        '0'
      } else {
        ' '
      };
      slots[index].push(digit);
    }
    if let Some(first) = first
      && cursor > 0
      && !truncate
    {
      slots[first].insert_str(0, &integral[..cursor]);
    }
    let mut fractional_index = 0;
    for (index, item) in items.iter().enumerate().skip(decimal) {
      if item.digit() {
        if let Some(&digit) = fractional.as_bytes().get(fractional_index) {
          slots[index].push(char::from(digit));
        } else if *item == Item::Zero {
          slots[index].push('0');
        } else if *item == Item::Optional {
          slots[index].push(' ');
        }
        fractional_index += 1;
      }
    }
    let mut remaining_digits = slots[..decimal]
      .iter()
      .flat_map(|slot| slot.bytes())
      .filter(u8::is_ascii_digit)
      .count();
    let mut wrote_digit = false;
    let first_digit = items.iter().position(|item| item.digit());
    let mut output = String::new();
    for (index, item) in items.iter().enumerate() {
      if automatic_sign && Some(index) == first_digit {
        output.push('-');
      }
      match item {
        Item::Zero | Item::Optional | Item::Truncate => {
          for ch in slots[index].chars() {
            if index < decimal && ch.is_ascii_digit() {
              if grouped && wrote_digit && remaining_digits.is_multiple_of(3) {
                output.push(symbols.grouping);
              }
              remaining_digits -= 1;
              wrote_digit = true;
            }
            output.push(ch);
          }
        }
        Item::Decimal => output.push(symbols.decimal),
        Item::Group => {}
        Item::Plus | Item::Minus => output.push(if negative {
          '-'
        } else if *item == Item::Plus && rounded != 0.0 {
          '+'
        } else {
          ' '
        }),
        Item::Literal(ch) => output.push(*ch),
      }
    }
    output
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn native_word_pictures_preserve_positions_signs_and_literal_percent() {
    let symbols = OfficeNumberSeparators::default();
    // Native Word matrix: signs, zero, fractions, overflow, literal NBSP,
    // grouping, section selection and round-half boundaries. No PDF tolerance.
    for (expression, picture, expected) in [
      ("50*40", "0", "2000"),
      ("50*40", "0.00", "2000.00"),
      ("50*40", "#,##0.00", "2,000.00"),
      ("50*40", "# ##0,00", "   2,000"),
      ("50*40", "0,00", "2,000"),
      ("50*40", "0%", "2000%"),
      ("50*40", "0.00%", "2000.00%"),
      ("50*40", "0,00%", "2,000%"),
      ("50*40", "# ##0", "2 000"),
      ("50*40", "# ##0,00 Ft;(# ##0,00 Ft)", "   2,000 Ft"),
      ("50*40", "00000", "02000"),
      ("50*40", "###", "2000"),
      ("50*40", "+0.0", "+2000.0"),
      ("50*40", "-0.0", " 2000.0"),
      ("50*40", "0.0;(0.0);'zero'", "2000.0"),
      ("50*40", "x##", "000"),
      ("50*40", "0.00x", "2000.00"),
      ("-50*40", "0", "-2000"),
      ("-50*40", "0.00", "-2000.00"),
      ("-50*40", "#,##0.00", "-2,000.00"),
      ("-50*40", "# ##0,00", "-   2,000"),
      ("-50*40", "0,00", "-2,000"),
      ("-50*40", "0%", "-2000%"),
      ("-50*40", "0.00%", "-2000.00%"),
      ("-50*40", "0,00%", "-2,000%"),
      ("-50*40", "# ##0", "-2 000"),
      ("-50*40", "# ##0,00 Ft;(# ##0,00 Ft)", "(   2,000 Ft)"),
      ("-50*40", "00000", "-02000"),
      ("-50*40", "###", "-2000"),
      ("-50*40", "+0.0", "-2000.0"),
      ("-50*40", "-0.0", "-2000.0"),
      ("-50*40", "0.0;(0.0);'zero'", "(2000.0)"),
      ("-50*40", "x##", "000"),
      ("-50*40", "0.00x", "-2000.00"),
      ("0", "0", "0"),
      ("0", "0.00", "0.00"),
      ("0", "#,##0.00", "   0.00"),
      ("0", "# ##0,00", "    000"),
      ("0", "0,00", "000"),
      ("0", "0%", "0%"),
      ("0", "0.00%", "0.00%"),
      ("0", "0,00%", "000%"),
      ("0", "# ##0", "    0"),
      ("0", "# ##0,00 Ft;(# ##0,00 Ft)", "    000 Ft"),
      ("0", "00000", "00000"),
      ("0", "###", "   "),
      ("0", "+0.0", " 0.0"),
      ("0", "-0.0", " 0.0"),
      ("0", "0.0;(0.0);'zero'", "zero"),
      ("0", "x##", "   "),
      ("0", "0.00x", "0.00"),
      ("1/8", "0", "0"),
      ("1/8", "0.00", "0.13"),
      ("1/8", "#,##0.00", "   0.13"),
      ("1/8", "# ##0,00", "    000"),
      ("1/8", "0,00", "000"),
      ("1/8", "0%", "0%"),
      ("1/8", "0.00%", "0.13%"),
      ("1/8", "0,00%", "000%"),
      ("1/8", "# ##0", "    0"),
      ("1/8", "# ##0,00 Ft;(# ##0,00 Ft)", "    000 Ft"),
      ("1/8", "00000", "00000"),
      ("1/8", "###", "   "),
      ("1/8", "+0.0", "+0.1"),
      ("1/8", "-0.0", " 0.1"),
      ("1/8", "0.0;(0.0);'zero'", "0.1"),
      ("1/8", "x##", "   "),
      ("1/8", "0.00x", "0.125"),
      ("3/4", "0", "1"),
      ("3/4", "0.00", "0.75"),
      ("3/4", "#,##0.00", "   0.75"),
      ("3/4", "# ##0,00", "    001"),
      ("3/4", "0,00", "001"),
      ("3/4", "0%", "1%"),
      ("3/4", "0.00%", "0.75%"),
      ("3/4", "0,00%", "001%"),
      ("3/4", "# ##0", "    1"),
      ("3/4", "# ##0,00 Ft;(# ##0,00 Ft)", "    001 Ft"),
      ("3/4", "00000", "00001"),
      ("3/4", "###", "  1"),
      ("3/4", "+0.0", "+0.8"),
      ("3/4", "-0.0", " 0.8"),
      ("3/4", "0.0;(0.0);'zero'", "0.8"),
      ("3/4", "x##", "  1"),
      ("3/4", "0.00x", "0.75"),
      ("1234567/100", "0", "12346"),
      ("1234567/100", "0.00", "12345.67"),
      ("1234567/100", "#,##0.00", "12,345.67"),
      ("1234567/100", "# ##0,00", "  12,346"),
      ("1234567/100", "0,00", "12,346"),
      ("1234567/100", "0%", "12346%"),
      ("1234567/100", "0.00%", "12345.67%"),
      ("1234567/100", "0,00%", "12,346%"),
      ("1234567/100", "# ##0", "12 346"),
      ("1234567/100", "# ##0,00 Ft;(# ##0,00 Ft)", "  12,346 Ft"),
      ("1234567/100", "00000", "12346"),
      ("1234567/100", "###", "12346"),
      ("1234567/100", "+0.0", "+12345.7"),
      ("1234567/100", "-0.0", " 12345.7"),
      ("1234567/100", "0.0;(0.0);'zero'", "12345.7"),
      ("1234567/100", "x##", "346"),
      ("1234567/100", "0.00x", "12345.67"),
      ("0", "#,#0", "  0"),
      ("0", "000,000", "000,000"),
      ("0", "# 000", "  000"),
      ("0", "0.##", "0.0 "),
      ("0", "0.000", "0.000"),
      ("0", "000-00", "000 00"),
      ("0", "+0.00", " 0.00"),
      ("7", "#,#0", "  7"),
      ("7", "000,000", "000,007"),
      ("7", "# 000", "  007"),
      ("7", "0.##", "7.  "),
      ("7", "0.000", "7.000"),
      ("7", "000-00", "000 07"),
      ("7", "+0.00", "+7.00"),
      ("-7", "#,#0", "-  7"),
      ("-7", "000,000", "-000,007"),
      ("-7", "# 000", "-  007"),
      ("-7", "0.##", "-7.  "),
      ("-7", "0.000", "-7.000"),
      ("-7", "000-00", "000-07"),
      ("-7", "+0.00", "-7.00"),
      ("1234567", "#,#0", "1,234,567"),
      ("1234567", "000,000", "1,234,567"),
      ("1234567", "# 000", "1234 567"),
      ("1234567", "0.##", "1234567.  "),
      ("1234567", "0.000", "1234567.000"),
      ("1234567", "000-00", "12345 67"),
      ("1234567", "+0.00", "+1234567.00"),
      ("-0.004", "#,#0", "  0"),
      ("-0.004", "000,000", "000,000"),
      ("-0.004", "# 000", "  000"),
      ("-0.004", "0.##", "0.0 "),
      ("-0.004", "0.000", "-0.004"),
      ("-0.004", "000-00", "000 00"),
      ("-0.004", "+0.00", " 0.00"),
      ("1.005", "#,#0", "  1"),
      ("1.005", "000,000", "000,001"),
      ("1.005", "# 000", "  001"),
      ("1.005", "0.##", "1.01"),
      ("1.005", "0.000", "1.005"),
      ("1.005", "000-00", "000 01"),
      ("1.005", "+0.00", "+1.01"),
      ("2.675", "#,#0", "  3"),
      ("2.675", "000,000", "000,003"),
      ("2.675", "# 000", "  003"),
      ("2.675", "0.##", "2.68"),
      ("2.675", "0.000", "2.675"),
      ("2.675", "000-00", "000 03"),
      ("2.675", "+0.00", "+2.68"),
      ("12", "'USD '000", "USD 012"),
      ("-12", "'USD '000", "USD -012"),
      ("0", "0.00;[0.00];ZERO", "ZERO"),
      ("12", "0.00;[0.00];ZERO", "12.00"),
      ("-12", "0.00;[0.00];ZERO", "[12.00]"),
      ("0", "x##", "   "),
      ("2000", "x##", "000"),
      ("-2001", "x##", "001"),
      ("1/8", "0.0x", "0.13"),
      ("3/4", ".x", ".8"),
      ("0", "0.##", "0.0 "),
      ("1.2", "0.##", "1.2 "),
      ("1.02", "0.##", "1.02"),
      ("1.234", "0.0x0", "1.234"),
      ("1.234", "0.x00", "1.234"),
      ("12", "00x00", "00 12"),
      ("12345", "00x00", "12345"),
      ("1.005", "0.00", "1.01"),
      ("2.675", "0.00", "2.68"),
      ("1/3", "0.0000", "0.3333"),
    ] {
      let number = super::super::scalar::evaluate(expression, symbols).unwrap();
      let picture = Picture::parse(picture, symbols).unwrap();
      assert_eq!(
        picture.format(number.value, symbols),
        expected,
        "{expression}: {picture:?}"
      );
    }
  }
}
