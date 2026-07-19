use std::borrow::Cow;

/// Converts the legacy character code used by Symbol-family fonts to the
/// Unicode scalar that the glyph represents.
///
/// DrawingML marks a bullet's font separately from its character. LibreOffice
/// likewise treats these named fonts as symbol encodings in
/// `oox/source/drawingml/textparagraphproperties.cxx`. The mappings mirror the
/// conversion tables in `unotools/source/misc/fontcvt.cxx` and
/// `sal/textenc/convertadobe.tab` so PDF text semantics do not expose the
/// font's ASCII/PUA transport code.
pub(crate) fn font_symbol_code(font: Option<&str>, code: u32) -> Option<char> {
  let low_byte = code & 0xFF;
  let font = font.unwrap_or("").to_ascii_lowercase();

  if font.contains("wingdings") {
    return wingdings_symbol(low_byte).or_else(|| char::from_u32(code));
  }
  if font == "symbol" || font.ends_with(" symbol") {
    return symbol_font_symbol(low_byte).or_else(|| char::from_u32(code));
  }

  char::from_u32(code).or_else(|| {
    if (0xF000..=0xF0FF).contains(&code) {
      char::from_u32(low_byte)
    } else {
      None
    }
  })
}

pub(crate) fn font_symbol_transport_text<'a>(font: Option<&str>, text: &'a str) -> Cow<'a, str> {
  let font = font.unwrap_or("");
  let is_symbol_font = font.eq_ignore_ascii_case("Symbol")
    || font
      .get(font.len().saturating_sub(" symbol".len())..)
      .is_some_and(|suffix| suffix.eq_ignore_ascii_case(" symbol"))
    || font.to_ascii_lowercase().contains("wingdings");
  if !is_symbol_font {
    return Cow::Borrowed(text);
  }

  let mut mapped: Option<String> = None;

  for (byte_index, character) in text.char_indices() {
    // LibreOffice's DrawingML importer preserves the selected symbol-font
    // glyph by converting its single-byte character to U+F0XX. Existing PUA
    // transport codes and already-Unicode characters remain unchanged.
    let replacement = if character as u32 <= 0xFF {
      char::from_u32(0xF000 | character as u32).unwrap_or(character)
    } else {
      character
    };
    if replacement == character {
      if let Some(output) = mapped.as_mut() {
        output.push(character);
      }
      continue;
    }

    let output = mapped.get_or_insert_with(|| {
      let mut output = String::with_capacity(text.len());
      output.push_str(&text[..byte_index]);
      output
    });
    output.push(replacement);
  }

  mapped.map(Cow::Owned).unwrap_or(Cow::Borrowed(text))
}

fn symbol_font_symbol(code: u32) -> Option<char> {
  Some(match code {
    0x2D => '−',
    0x41 => 'Α',
    0x42 => 'Β',
    0x43 => 'Χ',
    0x44 => 'Δ',
    0x45 => 'Ε',
    0x46 => 'Φ',
    0x47 => 'Γ',
    0x48 => 'Η',
    0x49 => 'Ι',
    0x4A => 'ϑ',
    0x4B => 'Κ',
    0x4C => 'Λ',
    0x4D => 'Μ',
    0x4E => 'Ν',
    0x4F => 'Ο',
    0x50 => 'Π',
    0x51 => 'Θ',
    0x52 => 'Ρ',
    0x53 => 'Σ',
    0x54 => 'Τ',
    0x55 => 'Υ',
    0x56 => 'ς',
    0x57 => 'Ω',
    0x58 => 'Ξ',
    0x59 => 'Ψ',
    0x5A => 'Ζ',
    0x61 => 'α',
    0x62 => 'β',
    0x63 => 'χ',
    0x64 => 'δ',
    0x65 => 'ε',
    0x66 => 'φ',
    0x67 => 'γ',
    0x68 => 'η',
    0x69 => 'ι',
    0x6A => 'ϕ',
    0x6B => 'κ',
    0x6C => 'λ',
    0x6D => 'μ',
    0x6E => 'ν',
    0x6F => 'ο',
    0x70 => 'π',
    0x71 => 'θ',
    0x72 => 'ρ',
    0x73 => 'σ',
    0x74 => 'τ',
    0x75 => 'υ',
    0x76 => 'ϖ',
    0x77 => 'ω',
    0x78 => 'ξ',
    0x79 => 'ψ',
    0x7A => 'ζ',
    0xA2 => '′',
    0xA3 => '≤',
    0xA5 => '∞',
    0xA7 => '♣',
    0xA8 => '♦',
    0xA9 => '♥',
    0xAA => '♠',
    0xB1 => '±',
    0xB4 => '×',
    0xB5 => '∝',
    0xB6 => '∂',
    0xB7 => '•',
    0xB8 => '÷',
    0xB9 => '≠',
    0xBA => '≡',
    0xBB => '≈',
    0xBC => '…',
    0xBD => '⏐',
    0xBE => '⎯',
    0xBF => '↵',
    0xC0 => 'ℵ',
    0xC1 => 'ℑ',
    0xC2 => 'ℜ',
    0xC3 => '℘',
    0xC4 => '⊗',
    0xC5 => '⊕',
    0xC6 => '∅',
    0xC7 => '∩',
    0xC8 => '∪',
    0xC9 => '⊃',
    0xCA => '⊇',
    0xCB => '⊄',
    0xCC => '⊂',
    0xCD => '⊆',
    0xCE => '∈',
    0xCF => '∉',
    0xD0 => '∠',
    0xD1 => '∇',
    0xD2 => '®',
    0xD3 => '©',
    0xD4 => '™',
    0xD5 => '∏',
    0xD6 => '√',
    0xD7 => '⋅',
    0xD8 => '¬',
    0xD9 => '∧',
    0xDA => '∨',
    0xDB => '⇔',
    0xDC => '⇐',
    0xDD => '⇑',
    0xDE => '⇒',
    0xDF => '⇓',
    0xE0 => '◊',
    0xE1 => '〈',
    0xE2 => '®',
    0xE3 => '©',
    0xE4 => '™',
    0xE5 => '∑',
    0xE6 => '⎛',
    0xE7 => '⎜',
    0xE8 => '⎝',
    0xE9 => '⎡',
    0xEA => '⎢',
    0xEB => '⎣',
    0xEC => '⎧',
    0xED => '⎨',
    0xEE => '⎩',
    0xEF => '⎪',
    0xF1 => '〉',
    0xF2 => '∫',
    0xF3 => '⌠',
    0xF4 => '⎮',
    0xF5 => '⌡',
    0xF6 => '⎞',
    0xF7 => '⎟',
    0xF8 => '⎠',
    0xF9 => '⎤',
    0xFA => '⎥',
    0xFB => '⎦',
    0xFC => '⎫',
    0xFD => '⎬',
    0xFE => '⎭',
    _ => return char::from_u32(code),
  })
}

fn wingdings_symbol(code: u32) -> Option<char> {
  Some(match code {
    0x4A => '☺',
    0x4C => '●',
    0x6C => '●',
    0x6D => '■',
    0x6E => '□',
    0x71 => '❑',
    0x72 => '❒',
    0x73 => '⬧',
    0x74 => '◆',
    0x75 => '❖',
    0x76 => '⬥',
    0x77 => '⌧',
    0x78 => '⌦',
    0x9F => '•',
    0xA8 => '◻',
    0xF0 => '➔',
    0xFC => '✓',
    0xFD => '☒',
    0xFE => '☑',
    _ => return None,
  })
}

#[cfg(test)]
mod tests {
  use std::borrow::Cow;

  use super::{font_symbol_code, font_symbol_transport_text};

  #[test]
  fn wingdings_transport_codes_map_to_unicode() {
    assert_eq!(font_symbol_code(Some("Wingdings"), 'q' as u32), Some('❑'));
    assert_eq!(font_symbol_code(Some("Wingdings"), 0xF071), Some('❑'));
  }

  #[test]
  fn adobe_symbol_minus_maps_to_unicode_minus() {
    assert_eq!(font_symbol_code(Some("Symbol"), 0xF02D), Some('−'));
  }

  #[test]
  fn ordinary_text_is_borrowed_and_unchanged() {
    let text = font_symbol_transport_text(Some("Calibri"), "q");
    assert!(matches!(text, Cow::Borrowed("q")));
  }

  #[test]
  fn symbol_font_ascii_uses_pua_transport_code() {
    let text = font_symbol_transport_text(Some("Wingdings"), "q");
    assert_eq!(text, "\u{f071}");
    assert!(matches!(text, Cow::Owned(_)));
  }
}
