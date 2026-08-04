use olecfsdk::cfb::CompoundFile;

const EQUATION_NATIVE_HEADER_BYTES: usize = 28;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MathTypeEquation {
  Mtef3(String),
  Mtef5(Mtef5Document),
}

impl MathTypeEquation {
  pub(super) fn semantic_text(&self) -> String {
    match self {
      Self::Mtef3(text) => text.clone(),
      Self::Mtef5(document) => {
        let mut text = String::new();
        document.append_text(&mut text);
        text
      }
    }
  }

  pub(super) fn mtef5_document(&self) -> Option<&Mtef5Document> {
    match self {
      Self::Mtef5(document) => Some(document),
      Self::Mtef3(_) => None,
    }
  }
}

/// Extracts the editable character stream from a MathType `Equation Native`
/// stream.
///
/// MathType stores MTEF after the 28-byte OLE native-data header. MTEF 1-3
/// put record flags in the high nibble of the record tag; MTEF 5 uses a
/// separate options byte and a variable-length application key. Keep both
/// grammars separate so a version-5 options byte can never be mistaken for a
/// version-3 record.
pub(super) fn equation_native(ole_bytes: &[u8]) -> Option<MathTypeEquation> {
  let compound = CompoundFile::from_bytes(ole_bytes).ok()?;
  let stream = compound.stream("/Equation Native")?;
  let mtef = equation_native_payload(stream)?;
  match mtef.first().copied()? {
    1..=3 => mtef3_text(mtef).map(MathTypeEquation::Mtef3),
    5 => Mtef5Reader::new(mtef)
      .document()
      .map(MathTypeEquation::Mtef5),
    _ => None,
  }
}

fn equation_native_payload(stream: &[u8]) -> Option<&[u8]> {
  if stream.len() < EQUATION_NATIVE_HEADER_BYTES {
    return None;
  }
  let header_bytes = usize::from(u16::from_le_bytes(stream[0..2].try_into().ok()?));
  if header_bytes != EQUATION_NATIVE_HEADER_BYTES {
    return None;
  }
  let payload_bytes = usize::try_from(u32::from_le_bytes(stream[8..12].try_into().ok()?)).ok()?;
  let end = header_bytes.checked_add(payload_bytes)?;
  stream.get(header_bytes..end)
}

fn mtef3_text(mtef: &[u8]) -> Option<String> {
  if mtef.len() < 5 {
    return None;
  }
  let version = mtef[0];
  let mut reader = Mtef3Reader {
    bytes: mtef,
    position: 5,
    version,
  };
  let text = reader.records(0)?;
  (!text.is_empty()).then_some(text)
}

struct Mtef3Reader<'a> {
  bytes: &'a [u8],
  position: usize,
  version: u8,
}

impl Mtef3Reader<'_> {
  fn records(&mut self, depth: usize) -> Option<String> {
    if depth > 1024 {
      return None;
    }

    let mut text = String::new();
    loop {
      let tag = self.byte()?;
      match tag & 0x0f {
        0 => return Some(text), // END
        1 => self.nudge(tag)?,  // LINE
        2 => {
          let character = self.character()?;
          if let Some(character) = character {
            text.push(character);
          }
          if tag & 0x20 != 0 {
            text.push_str(&self.records(depth + 1)?);
          }
        }
        3 => {
          self.byte()?; // selector
          self.byte()?; // variation
          self.byte()?; // template option
          text.push_str(&self.records(depth + 1)?);
        }
        4 => {
          self.byte()?; // horizontal alignment
          self.byte()?; // vertical alignment
          text.push_str(&self.records(depth + 1)?);
        }
        5 => {
          self.byte()?; // vertical alignment
          self.byte()?; // horizontal justification
          self.byte()?; // vertical justification
          let rows = self.byte()?;
          let columns = self.byte()?;
          let row_bytes = (usize::from(rows + 1) * 2).div_ceil(8);
          let column_bytes = (usize::from(columns + 1) * 2).div_ceil(8);
          self.skip(row_bytes + column_bytes)?;
          text.push_str(&self.records(depth + 1)?);
        }
        6 => self.embellishments()?,
        7 => {
          let count = usize::from(self.byte()?);
          self.skip(count.checked_mul(3)?)?;
        }
        8 => {
          self.byte()?;
          self.byte()?;
          self.null_terminated()?;
        }
        9 => self.size_record()?,
        10..=14 => {}
        _ => return None,
      }
    }
  }

  fn character(&mut self) -> Option<Option<char>> {
    self.byte()?; // Typeface number.
    let value = if self.version < 3 {
      u16::from(self.byte()?)
    } else {
      self.u16()?
    };
    let character = char::from_u32(u32::from(value)).filter(|character| *character >= ' ');
    Some(character)
  }

  fn nudge(&mut self, tag: u8) -> Option<()> {
    if tag & 0x80 == 0 {
      return Some(());
    }
    let x = self.byte()?;
    let y = self.byte()?;
    if x == 128 && y == 128 {
      self.skip(4)?;
    }
    Some(())
  }

  fn embellishments(&mut self) -> Option<()> {
    loop {
      let embellishment = self.byte()?;
      if embellishment == 0 || self.version < 3 {
        return Some(());
      }
    }
  }

  fn size_record(&mut self) -> Option<()> {
    let first = self.byte()?;
    match first {
      100 => {
        self.byte()?;
        self.skip(2)?;
      }
      101 => self.skip(2)?,
      _ => {
        self.byte()?;
      }
    }
    Some(())
  }

  fn byte(&mut self) -> Option<u8> {
    let byte = *self.bytes.get(self.position)?;
    self.position += 1;
    Some(byte)
  }

  fn u16(&mut self) -> Option<u16> {
    let low = u16::from(self.byte()?);
    let high = u16::from(self.byte()?);
    Some(low | high << 8)
  }

  fn skip(&mut self, count: usize) -> Option<()> {
    self.position = self.position.checked_add(count)?;
    (self.position <= self.bytes.len()).then_some(())
  }

  fn null_terminated(&mut self) -> Option<()> {
    loop {
      if self.byte()? == 0 {
        return Some(());
      }
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Mtef5Document {
  pub(super) application_key: String,
  pub(super) inline: bool,
  pub(super) records: Vec<Mtef5Node>,
}

impl Mtef5Document {
  fn append_text(&self, text: &mut String) {
    for record in &self.records {
      record.append_text(text);
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Mtef5Node {
  Line {
    null: bool,
    records: Vec<Self>,
  },
  Character(Mtef5Character),
  Template {
    selector: u8,
    variation: u16,
    records: Vec<Self>,
  },
  Pile {
    horizontal_alignment: u8,
    vertical_alignment: u8,
    records: Vec<Self>,
  },
  Matrix {
    rows: u8,
    columns: u8,
    records: Vec<Self>,
  },
  Embellishment(u8),
}

impl Mtef5Node {
  fn append_text(&self, text: &mut String) {
    match self {
      Self::Line {
        null: false,
        records,
      }
      | Self::Template { records, .. }
      | Self::Matrix { records, .. } => {
        for record in records {
          record.append_text(text);
        }
      }
      Self::Pile { records, .. } => {
        for record in records {
          record.append_text(text);
        }
      }
      Self::Character(character) => {
        if let Some(value) = character.semantic_character() {
          text.push(value);
        }
      }
      Self::Line { null: true, .. } | Self::Embellishment(_) => {}
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Mtef5Character {
  pub(super) typeface: i32,
  pub(super) size: u8,
  pub(super) mt_code: Option<u16>,
  pub(super) font_position: Option<u16>,
  pub(super) function_start: bool,
}

impl Mtef5Character {
  pub(super) fn semantic_character(&self) -> Option<char> {
    self
      .mt_code
      .and_then(|value| char::from_u32(u32::from(value)))
      .filter(|value| *value >= ' ')
      .or_else(|| {
        self
          .font_position
          .and_then(|value| char::from_u32(u32::from(value)))
          .filter(|value| *value >= ' ')
      })
  }
}

struct Mtef5Reader<'a> {
  bytes: &'a [u8],
  position: usize,
  size: u8,
}

impl<'a> Mtef5Reader<'a> {
  fn new(bytes: &'a [u8]) -> Self {
    Self {
      bytes,
      position: 0,
      size: 0,
    }
  }

  fn document(mut self) -> Option<Mtef5Document> {
    if self.byte()? != 5 {
      return None;
    }
    self.skip(4)?; // platform, product, product version, product subversion
    let application_key = self.string()?;
    let inline = self.byte()? & 1 != 0;
    let records = self.records(0)?;
    Some(Mtef5Document {
      application_key,
      inline,
      records,
    })
  }

  fn records(&mut self, depth: usize) -> Option<Vec<Mtef5Node>> {
    if depth > 1024 {
      return None;
    }
    let mut records = Vec::new();
    loop {
      let record_type = self.byte()?;
      if record_type == 0 {
        return Some(records);
      }
      if record_type >= 100 {
        let bytes = self.unsigned()?;
        self.skip(usize::from(bytes))?;
        continue;
      }
      match record_type {
        1 => records.push(self.line(depth)?),
        2 => records.push(self.character(depth)?),
        3 => records.push(self.template(depth)?),
        4 => records.push(self.pile(depth)?),
        5 => records.push(self.matrix(depth)?),
        6 => records.push(self.embellishment()?),
        7 => self.ruler()?,
        8 => {
          self.unsigned()?; // font definition index
          self.byte()?; // character style bits
        }
        9 => self.size()?,
        10..=14 => self.size = record_type - 10,
        15 => {
          self.unsigned()?; // color definition index
        }
        16 => self.color_definition()?,
        17 => {
          self.unsigned()?; // encoding definition index
          self.string()?; // font name
        }
        18 => self.equation_preferences()?,
        19 => {
          self.string()?; // encoding name
        }
        _ => return None,
      }
    }
  }

  fn line(&mut self, depth: usize) -> Option<Mtef5Node> {
    let options = self.byte()?;
    self.nudge(options)?;
    if options & 0x04 != 0 {
      self.skip(2)?;
    }
    if options & 0x02 != 0 {
      self.required_ruler()?;
    }
    let null = options & 0x01 != 0;
    let records = if null {
      Vec::new()
    } else {
      self.records(depth + 1)?
    };
    Some(Mtef5Node::Line { null, records })
  }

  fn character(&mut self, depth: usize) -> Option<Mtef5Node> {
    let options = self.byte()?;
    self.nudge(options)?;
    let typeface = self.signed()?;
    let mt_code = if options & 0x20 == 0 {
      Some(self.u16()?)
    } else {
      None
    };
    let font_position = if options & 0x04 != 0 {
      Some(u16::from(self.byte()?))
    } else if options & 0x10 != 0 {
      Some(self.u16()?)
    } else {
      None
    };
    let character = Mtef5Character {
      typeface,
      size: self.size,
      mt_code,
      font_position,
      function_start: options & 0x02 != 0,
    };
    if options & 0x01 != 0 {
      let embellishments = self.records(depth + 1)?;
      if !embellishments
        .iter()
        .all(|record| matches!(record, Mtef5Node::Embellishment(_)))
      {
        return None;
      }
    }
    Some(Mtef5Node::Character(character))
  }

  fn template(&mut self, depth: usize) -> Option<Mtef5Node> {
    let options = self.byte()?;
    self.nudge(options)?;
    let selector = self.byte()?;
    let first_variation = self.byte()?;
    let variation = if first_variation & 0x80 != 0 {
      u16::from(first_variation & 0x7f) | u16::from(self.byte()?) << 8
    } else {
      u16::from(first_variation)
    };
    self.byte()?; // template-specific options
    let records = self.records(depth + 1)?;
    Some(Mtef5Node::Template {
      selector,
      variation,
      records,
    })
  }

  fn pile(&mut self, depth: usize) -> Option<Mtef5Node> {
    let options = self.byte()?;
    self.nudge(options)?;
    let horizontal_alignment = self.byte()?;
    let vertical_alignment = self.byte()?;
    if options & 0x02 != 0 {
      self.required_ruler()?;
    }
    Some(Mtef5Node::Pile {
      horizontal_alignment,
      vertical_alignment,
      records: self.records(depth + 1)?,
    })
  }

  fn matrix(&mut self, depth: usize) -> Option<Mtef5Node> {
    let options = self.byte()?;
    self.nudge(options)?;
    self.skip(3)?; // vertical alignment, horizontal and vertical justification
    let rows = self.byte()?;
    let columns = self.byte()?;
    let row_bytes = (usize::from(rows + 1) * 2).div_ceil(8);
    let column_bytes = (usize::from(columns + 1) * 2).div_ceil(8);
    self.skip(row_bytes + column_bytes)?;
    Some(Mtef5Node::Matrix {
      rows,
      columns,
      records: self.records(depth + 1)?,
    })
  }

  fn embellishment(&mut self) -> Option<Mtef5Node> {
    let options = self.byte()?;
    self.nudge(options)?;
    Some(Mtef5Node::Embellishment(self.byte()?))
  }

  fn required_ruler(&mut self) -> Option<()> {
    (self.byte()? == 7).then_some(())?;
    self.ruler()
  }

  fn ruler(&mut self) -> Option<()> {
    let stops = usize::from(self.byte()?);
    self.skip(stops.checked_mul(3)?)
  }

  fn size(&mut self) -> Option<()> {
    match self.byte()? {
      100 => {
        self.size = self.byte()?;
        self.skip(2) // signed 16-bit delta
      }
      101 => {
        self.size = u8::MAX;
        self.skip(2) // explicit signed 16-bit point size
      }
      size => {
        self.size = size;
        self.skip(1) // one-byte delta
      }
    }
  }

  fn color_definition(&mut self) -> Option<()> {
    let options = self.byte()?;
    let components = if options & 0x01 != 0 { 4 } else { 3 };
    self.skip(components * 2)?;
    if options & 0x04 != 0 {
      self.string()?;
    }
    Some(())
  }

  fn equation_preferences(&mut self) -> Option<()> {
    self.byte()?; // options
    self.dimension_array()?;
    self.dimension_array()?;
    let styles = usize::from(self.byte()?);
    for _ in 0..styles {
      if self.unsigned()? != 0 {
        self.byte()?; // character style bits
      }
    }
    Some(())
  }

  fn dimension_array(&mut self) -> Option<()> {
    let dimensions = usize::from(self.byte()?);
    let mut completed = 0usize;
    while completed < dimensions {
      let byte = self.byte()?;
      completed += usize::from(byte >> 4 == 0x0f);
      if completed < dimensions {
        completed += usize::from(byte & 0x0f == 0x0f);
      }
    }
    Some(())
  }

  fn nudge(&mut self, options: u8) -> Option<()> {
    if options & 0x08 == 0 {
      return Some(());
    }
    let x = self.byte()?;
    let y = self.byte()?;
    if x == 128 && y == 128 {
      self.skip(4)?;
    }
    Some(())
  }

  fn signed(&mut self) -> Option<i32> {
    let first = self.byte()?;
    if first == 255 {
      Some(i32::from(self.u16()?) - 32768)
    } else {
      Some(i32::from(first) - 128)
    }
  }

  fn unsigned(&mut self) -> Option<u16> {
    let first = self.byte()?;
    if first == 255 {
      self.u16()
    } else {
      Some(u16::from(first))
    }
  }

  fn string(&mut self) -> Option<String> {
    let start = self.position;
    while self.byte()? != 0 {}
    String::from_utf8(self.bytes[start..self.position - 1].to_vec()).ok()
  }

  fn byte(&mut self) -> Option<u8> {
    let byte = *self.bytes.get(self.position)?;
    self.position += 1;
    Some(byte)
  }

  fn u16(&mut self) -> Option<u16> {
    let low = u16::from(self.byte()?);
    let high = u16::from(self.byte()?);
    Some(low | high << 8)
  }

  fn skip(&mut self, count: usize) -> Option<()> {
    self.position = self.position.checked_add(count)?;
    (self.position <= self.bytes.len()).then_some(())
  }
}

#[cfg(test)]
mod tests {
  use super::{Mtef3Reader, Mtef5Node, Mtef5Reader, equation_native_payload};

  #[test]
  fn reads_v3_utf16_char_records_in_record_order() {
    let bytes = [
      2, 0, b'A', 0, // CHAR, face 0, UTF-16 A
      2, 0, b'=', 0, // CHAR, face 0, UTF-16 =
      0,
    ];
    let mut reader = Mtef3Reader {
      bytes: &bytes,
      position: 0,
      version: 3,
    };
    assert_eq!(reader.records(0).as_deref(), Some("A="));
  }

  #[test]
  fn reads_v5_header_options_and_separate_record_options() {
    let bytes = [
      5, 1, 0, 6, 0, b'D', b'S', b'M', b'T', b'6', 0, 1, // header
      1, 0, // non-null LINE
      2, 0, 130, b'A', 0, // CHAR, fnFUNCTION, MTCode A
      2, 0, 136, b'2', 0, // CHAR, fnNUMBER, MTCode 2
      0, // end LINE
      0, // end root
    ];
    let document = Mtef5Reader::new(&bytes).document().unwrap();
    let mut text = String::new();
    document.append_text(&mut text);
    assert_eq!(document.application_key, "DSMT6");
    assert!(document.inline);
    assert_eq!(text, "A2");
  }

  #[test]
  fn reads_v5_pile_alignment_and_nested_records() {
    let bytes = [
      5, 1, 0, 6, 0, b'D', b'S', b'M', b'T', b'6', 0, 0, // header
      4, 0, 1, 1, // PILE, no options, left/top alignment
      1, 0, // non-null LINE
      2, 0, 130, b'A', 0, // CHAR, fnFUNCTION, MTCode A
      0, // end LINE
      0, // end PILE
      0, // end root
    ];
    let document = Mtef5Reader::new(&bytes).document().unwrap();
    let Mtef5Node::Pile {
      horizontal_alignment,
      vertical_alignment,
      records,
    } = &document.records[0]
    else {
      panic!("expected MTEF 5 PILE");
    };
    assert_eq!((*horizontal_alignment, *vertical_alignment), (1, 1));
    assert!(matches!(records.as_slice(), [Mtef5Node::Line { .. }]));
  }

  #[test]
  fn bounds_equation_native_payload_by_declared_size() {
    let mut stream = vec![0; 28];
    stream[0..2].copy_from_slice(&28u16.to_le_bytes());
    stream[8..12].copy_from_slice(&3u32.to_le_bytes());
    stream.extend_from_slice(&[5, 0, 0, 99]);
    assert_eq!(equation_native_payload(&stream), Some(&[5, 0, 0][..]));
  }
}
