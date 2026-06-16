use encoding_rs::WINDOWS_1252;
use icu_properties::{CodePointMapData, props::GeneralCategory};

pub fn trim_formula_text(value: &str) -> String {
  value.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn proper_formula_text(value: &str) -> String {
  let mut result = String::with_capacity(value.len());
  let mut previous_was_letter = false;
  for ch in value.chars() {
    if ch.is_alphabetic() {
      if previous_was_letter {
        result.extend(ch.to_lowercase());
      } else {
        result.extend(ch.to_uppercase());
      }
      previous_was_letter = true;
    } else {
      result.push(ch);
      previous_was_letter = false;
    }
  }
  result
}

pub fn rot13_formula_text(value: &str) -> String {
  value
    .chars()
    .map(|ch| match ch {
      'a'..='m' | 'A'..='M' => char::from_u32(ch as u32 + 13).unwrap_or(ch),
      'n'..='z' | 'N'..='Z' => char::from_u32(ch as u32 - 13).unwrap_or(ch),
      _ => ch,
    })
    .collect()
}

pub fn clean_formula_text(value: &str) -> String {
  let chars = value.chars().collect::<Vec<_>>();
  let mut output = String::with_capacity(value.len());
  let mut index = 0usize;
  while index < chars.len() {
    if chars.get(index).copied() == Some('\u{00c2}')
      && chars.get(index + 1).copied() == Some('\u{00a0}')
    {
      index += 1;
      continue;
    }
    if legacy_c1_text_len(&chars[index..]).is_some() {
      index += 2;
      continue;
    }
    let ch = chars[index];
    if !ch.is_control() && is_unicode_defined(ch) && !is_unicode_noncharacter(ch) {
      output.push(ch);
    }
    index += 1;
  }
  output
}

pub fn legacy_char_text(code: f64) -> Option<String> {
  if !(0.0..256.0).contains(&code) {
    return None;
  }
  let byte = code as u8;
  let bytes = [byte];
  let (text, _had_errors) = WINDOWS_1252.decode_without_bom_handling(&bytes);
  Some(text.into_owned())
}

pub fn legacy_text_code(ch: char) -> u8 {
  let text = ch.to_string();
  let (bytes, _encoding, had_errors) = WINDOWS_1252.encode(&text);
  if had_errors {
    b'?'
  } else {
    bytes.first().copied().unwrap_or(0)
  }
}

fn legacy_c1_text_len(chars: &[char]) -> Option<usize> {
  if chars.len() < 2 || chars.first().copied()? != '\u{00c2}' {
    return None;
  }
  (0x80..=0x9f)
    .filter_map(|byte| legacy_char_text(byte as f64))
    .filter_map(|text| text.chars().next())
    .any(|ch| chars.get(1).copied() == Some(ch))
    .then_some(2)
}

fn is_unicode_noncharacter(ch: char) -> bool {
  let code = ch as u32;
  (0xfdd0..=0xfdef).contains(&code) || code & 0xfffe == 0xfffe
}

fn is_unicode_defined(ch: char) -> bool {
  CodePointMapData::<GeneralCategory>::new().get(ch) != GeneralCategory::Unassigned
}

pub fn text_byte_len(text: &str) -> usize {
  text.chars().map(char_byte_len).sum()
}

pub fn char_byte_len(ch: char) -> usize {
  match ch as u32 {
    0x1100..=0x11FF
    | 0x2E80..=0xA4CF
    | 0xAC00..=0xD7AF
    | 0xF900..=0xFAFF
    | 0xFE30..=0xFE4F
    | 0xFF00..=0xFFEF
    | 0x20000..=0x2FA1F => 2,
    0x10000.. => 2,
    _ => 1,
  }
}

pub fn leftb(text: &str, mut count: usize) -> String {
  let mut result = String::new();
  for ch in text.chars() {
    if count == 0 {
      break;
    }
    let len = char_byte_len(ch);
    if count < len {
      result.push(' ');
      break;
    }
    result.push(ch);
    count -= len;
  }
  result
}

pub fn rightb(text: &str, mut count: usize) -> String {
  let mut chars = Vec::new();
  for ch in text.chars().rev() {
    if count == 0 {
      break;
    }
    let len = char_byte_len(ch);
    if count < len {
      chars.push(' ');
      break;
    }
    chars.push(ch);
    count -= len;
  }
  chars.into_iter().rev().collect()
}

pub fn baht_text(value: f64) -> String {
  const TH_ZERO: &str = "ศูนย์";
  const TH_BAHT: &str = "บาท";
  const TH_SATANG: &str = "สตางค์";
  const TH_EXACT: &str = "ถ้วน";
  const TH_MINUS: &str = "ลบ";

  let formatted = format!("{:.2}", value.abs());
  let (baht, satang) = formatted.split_once('.').unwrap_or((&formatted, "00"));
  let no_baht = baht == "0";
  let no_satang = satang == "00";
  if no_baht && no_satang {
    return format!("{TH_ZERO}{TH_BAHT}{TH_EXACT}");
  }

  let mut text = String::new();
  if value < 0.0 {
    text.push_str(TH_MINUS);
  }
  if !no_baht {
    let mut rest = baht;
    let mut block_size = rest.len() % 6;
    if block_size == 0 {
      block_size = 6;
    }
    while !rest.is_empty() {
      let (block, tail) = rest.split_at(block_size);
      append_thai_number_block(&mut text, block);
      rest = tail;
      block_size = 6;
      if !rest.is_empty() {
        text.push_str("ล้าน");
      }
    }
    text.push_str(TH_BAHT);
  }
  if no_satang {
    text.push_str(TH_EXACT);
  } else {
    append_thai_number_block(&mut text, satang);
    text.push_str(TH_SATANG);
  }
  text
}

fn append_thai_number_block(text: &mut String, block: &str) {
  let digits = block.as_bytes();
  for (index, digit) in digits.iter().enumerate() {
    let remaining = digits.len() - index - 1;
    if remaining >= 2 && *digit != b'0' {
      append_thai_digit(text, *digit);
      text.push_str(match remaining {
        2 => "ร้อย",
        3 => "พัน",
        4 => "หมื่น",
        5 => "แสน",
        _ => "",
      });
    }
  }
  let ten = if digits.len() > 1 {
    digits[digits.len() - 2]
  } else {
    b'0'
  };
  let one = *digits.last().unwrap_or(&b'0');
  if ten >= b'1' {
    if ten >= b'3' {
      append_thai_digit(text, ten);
    } else if ten == b'2' {
      text.push_str("ยี่");
    }
    text.push_str("สิบ");
  }
  if ten > b'0' && one == b'1' {
    text.push_str("เอ็ด");
  } else if one > b'0' {
    append_thai_digit(text, one);
  }
}

fn append_thai_digit(text: &mut String, digit: u8) {
  text.push_str(match digit {
    b'1' => "หนึ่ง",
    b'2' => "สอง",
    b'3' => "สาม",
    b'4' => "สี่",
    b'5' => "ห้า",
    b'6' => "หก",
    b'7' => "เจ็ด",
    b'8' => "แปด",
    b'9' => "เก้า",
    _ => "",
  });
}

pub fn roman_text_libreoffice(value: u16, mode: u16) -> String {
  const CHARS: [char; 7] = ['M', 'D', 'C', 'L', 'X', 'V', 'I'];
  const VALUES: [u16; 7] = [1000, 500, 100, 50, 10, 5, 1];
  let max_index = VALUES.len() - 1;
  let mut number = value;
  let mut output = String::new();
  for i in 0..=(max_index / 2) {
    let mut index = 2 * i;
    let digit = number / VALUES[index];
    if digit % 5 == 4 {
      let index2 = if digit == 4 { index - 1 } else { index - 2 };
      let mut steps = 0;
      while steps < mode && index < max_index {
        steps += 1;
        if VALUES[index2] - VALUES[index + 1] <= number {
          index += 1;
        } else {
          steps = mode;
        }
      }
      output.push(CHARS[index]);
      output.push(CHARS[index2]);
      number = number + VALUES[index] - VALUES[index2];
    } else {
      if digit > 4 {
        output.push(CHARS[index - 1]);
      }
      for _ in 0..(digit % 5) {
        output.push(CHARS[index]);
      }
      number %= VALUES[index];
    }
  }
  output
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn trims_propers_and_rot13s_text() {
    assert_eq!(trim_formula_text("  one\t two\nthree  "), "one two three");
    assert_eq!(proper_formula_text("hello 2world"), "Hello 2World");
    assert_eq!(rot13_formula_text("Abc-Mno-Zz"), "Nop-Zab-Mm");
  }

  #[test]
  fn legacy_char_and_code_use_windows_1252() {
    assert_eq!(legacy_char_text(65.0).as_deref(), Some("A"));
    assert_eq!(legacy_char_text(128.0).as_deref(), Some("€"));
    assert_eq!(legacy_text_code('€'), 128);
    assert_eq!(legacy_text_code('\u{2603}'), b'?');
  }

  #[test]
  fn cleans_control_and_legacy_c1_text() {
    assert_eq!(clean_formula_text("a\u{0007}b"), "ab");
    assert_eq!(clean_formula_text("a\u{00c2}€b"), "ab");
    assert_eq!(clean_formula_text("a\u{fdd0}b"), "ab");
  }

  #[test]
  fn byte_text_helpers_use_spreadsheet_widths() {
    assert_eq!(text_byte_len("Aあ😀"), 5);
    assert_eq!(leftb("AあB", 2), "A ");
    assert_eq!(leftb("AあB", 3), "Aあ");
    assert_eq!(rightb("AあB", 2), " B");
    assert_eq!(rightb("AあB", 3), "あB");
  }

  #[test]
  fn formats_roman_and_baht_text() {
    assert_eq!(roman_text_libreoffice(1999, 0), "MCMXCIX");
    assert_eq!(roman_text_libreoffice(499, 4), "ID");
    assert_eq!(baht_text(0.0), "ศูนย์บาทถ้วน");
    assert_eq!(baht_text(21.25), "ยี่สิบเอ็ดบาทยี่สิบห้าสตางค์");
  }
}
