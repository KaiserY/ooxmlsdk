use crate::calc::numeric::{approx_floor, approx_sub};

pub fn decimal_text_to_number(value: &str, base: u32) -> Option<f64> {
  if !(2..=36).contains(&base) {
    return None;
  }
  let mut value = value.trim_start();
  if base == 16 {
    value = value
      .strip_prefix("0x")
      .or_else(|| value.strip_prefix("0X"))
      .or_else(|| value.strip_prefix('x'))
      .or_else(|| value.strip_prefix('X'))
      .unwrap_or(value);
  }
  if value.is_empty() {
    return Some(0.0);
  }
  let mut result = 0.0;
  let mut chars = value.chars().peekable();
  while let Some(ch) = chars.next() {
    if chars.peek().is_none() && is_allowed_radix_suffix(ch, base) {
      break;
    }
    let digit = ch.to_digit(base)?;
    result = result * f64::from(base) + f64::from(digit);
  }
  Some(result)
}

fn is_allowed_radix_suffix(ch: char, base: u32) -> bool {
  matches!((base, ch), (2, 'b' | 'B') | (16, 'h' | 'H'))
}

pub fn convert_to_decimal(value: &str, base: u32, char_limit: usize) -> Option<f64> {
  if !(2..=36).contains(&base) {
    return None;
  }
  let value = value.trim();
  if value.len() > char_limit {
    return None;
  }
  if value.is_empty() {
    return Some(0.0);
  }
  let mut result = 0.0;
  let mut first_digit = None;
  for ch in value.chars() {
    let digit = ch.to_digit(base)?;
    if first_digit.is_none() {
      first_digit = Some(digit);
    }
    result = result * f64::from(base) + f64::from(digit);
  }
  if value.len() == char_limit && first_digit.is_some_and(|digit| digit >= base / 2) {
    result += -f64::from(base).powi(char_limit as i32);
  }
  Some(result)
}

pub fn convert_from_decimal(
  value: f64,
  min: f64,
  max: f64,
  base: u32,
  places: Option<i32>,
  max_places: usize,
) -> Option<String> {
  let value = approx_floor(value);
  if value < approx_floor(min) || value > approx_floor(max) {
    return None;
  }
  if places.is_some_and(|places| places <= 0 || places as usize > max_places) {
    return None;
  }
  let negative = value < 0.0;
  let mut number = value as i128;
  if negative {
    number += i128::from(base).pow(max_places as u32);
  }
  let mut output = format_radix(number, base)?;
  if let Some(places) = places {
    let places = places as usize;
    if !negative && output.len() > places {
      return None;
    }
    let target = if negative { max_places } else { places };
    if output.len() < target {
      let pad = if negative { max_digit_char(base)? } else { '0' };
      let mut padded = String::with_capacity(target);
      padded.extend(std::iter::repeat_n(pad, target - output.len()));
      padded.push_str(&output);
      output = padded;
    }
  }
  Some(output)
}

pub fn format_radix(mut value: i128, base: u32) -> Option<String> {
  if !(2..=36).contains(&base) || value < 0 {
    return None;
  }
  if value == 0 {
    return Some("0".to_string());
  }
  let mut digits = Vec::new();
  while value > 0 {
    let digit = (value % i128::from(base)) as u32;
    digits.push(char::from_digit(digit, base)?.to_ascii_uppercase());
    value /= i128::from(base);
  }
  digits.reverse();
  Some(digits.into_iter().collect())
}

fn max_digit_char(base: u32) -> Option<char> {
  char::from_digit(base.checked_sub(1)?, base).map(|ch| ch.to_ascii_uppercase())
}

pub fn base_number_text(value: f64, base: u32, min_len: usize) -> Option<String> {
  if value < 0.0 || !(2..=36).contains(&base) {
    return None;
  }
  let mut text = if value <= u64::MAX as f64 {
    format_radix(value as i128, base)?
  } else {
    format_large_radix(value, base)?
  };
  if text.len() < min_len {
    text = format!("{}{}", "0".repeat(min_len - text.len()), text);
  }
  Some(text)
}

fn format_large_radix(mut value: f64, base: u32) -> Option<String> {
  let mut digits = Vec::new();
  let base_value = f64::from(base);
  let mut dirty = false;
  while value != 0.0 {
    let quotient = approx_floor(value / base_value);
    let product = quotient * base_value;
    let digit = if value < product {
      dirty = true;
      0
    } else {
      let mut digit = approx_floor(approx_sub(value, product));
      if dirty {
        dirty = false;
        digit -= 1.0;
      }
      if digit <= 0.0 {
        0
      } else if digit >= base_value {
        base - 1
      } else {
        digit as u32
      }
    };
    digits.push(char::from_digit(digit, base)?.to_ascii_uppercase());
    value = quotient;
    if digits.len() > u16::MAX as usize {
      return None;
    }
  }
  if digits.is_empty() {
    return Some("0".to_string());
  }
  digits.reverse();
  Some(digits.into_iter().collect())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn decimal_text_accepts_core_prefixes_and_suffixes() {
    assert_eq!(decimal_text_to_number(" 0xFF", 16), Some(255.0));
    assert_eq!(decimal_text_to_number("xFF", 16), Some(255.0));
    assert_eq!(decimal_text_to_number("FFh", 16), Some(255.0));
    assert_eq!(decimal_text_to_number("101b", 2), Some(5.0));
    assert_eq!(decimal_text_to_number("102b", 2), None);
  }

  #[test]
  fn converts_twos_complement_addin_values() {
    assert_eq!(convert_to_decimal("1111111111", 2, 10), Some(-1.0));
    assert_eq!(
      convert_from_decimal(-1.0, -512.0, 511.0, 2, None, 10).as_deref(),
      Some("1111111111")
    );
    assert_eq!(
      convert_from_decimal(31.0, -512.0, 511.0, 16, Some(4), 10).as_deref(),
      Some("001F")
    );
  }

  #[test]
  fn formats_base_numbers_with_padding() {
    assert_eq!(base_number_text(31.0, 16, 4).as_deref(), Some("001F"));
    assert_eq!(base_number_text(0.0, 2, 1).as_deref(), Some("0"));
  }
}
