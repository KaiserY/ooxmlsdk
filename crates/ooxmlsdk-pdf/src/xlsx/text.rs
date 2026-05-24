pub(crate) fn decode_excel_escaped_text(text: &str) -> String {
  let mut output = String::with_capacity(text.len());
  let mut index = 0;
  while index < text.len() {
    let rest = &text[index..];
    if rest.len() >= 7 && rest.as_bytes()[0] == b'_' && rest.as_bytes()[1] == b'x' {
      let hex = &rest[2..6];
      if hex.as_bytes().iter().all(u8::is_ascii_hexdigit) && rest.as_bytes()[6] == b'_' {
        if hex.eq_ignore_ascii_case("005F") && rest.as_bytes().get(7) == Some(&b'x') {
          output.push('_');
          index += 7;
          continue;
        }
        if let Ok(value) = u32::from_str_radix(hex, 16)
          && let Some(decoded) = char::from_u32(value)
        {
          output.push(decoded);
          index += 7;
          continue;
        }
      }
    }
    let ch = rest.chars().next().expect("non-empty rest has a char");
    output.push(ch);
    index += ch.len_utf8();
  }
  output
}
