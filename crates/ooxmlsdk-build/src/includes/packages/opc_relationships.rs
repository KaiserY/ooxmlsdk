#[derive(Clone)]
pub struct Relationships {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
  pub relationship: Vec<Relationship>,
}

impl Relationships {
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(s: &str) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_str_inner(s)?;

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_reader_inner(reader)?;

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn deserialize_self<'de, R: super::super::common::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut with_xmlns = with_xmlns;

    let mut empty_tag = false;

    let e = match xml_reader.next()? {
      quick_xml::events::Event::Start(e) => e,
      quick_xml::events::Event::Empty(e) => {
        empty_tag = true;
        e
      }
      _ => Err(super::super::common::SdkError::CommonError(
        "Relationships".to_string(),
      ))?,
    };

    let mut xmlns = None;

    let mut xmlns_map = std::collections::HashMap::<String, String>::new();

    let mut mc_ignorable = None;

    let mut relationship = vec![];

    for attr in e.attributes() {
      let attr = attr?;
      match attr.key.as_ref() {
        b"xmlns" => {
          xmlns = Some(
            attr
              .decode_and_unescape_value(xml_reader.decoder())?
              .to_string(),
          );
        }
        b"mc:Ignorable" => {
          mc_ignorable = Some(
            attr
              .decode_and_unescape_value(xml_reader.decoder())?
              .to_string(),
          );
        }
        key => {
          if key.starts_with(b"xmlns:") {
            xmlns_map.insert(
              String::from_utf8_lossy(&key[6..]).to_string(),
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .to_string(),
            );
            if key == b"xmlns:w" {
              with_xmlns = true;
            }
          }
        }
      }
    }

    if with_xmlns {
      if e.name().as_ref() != b"w:Relationships" {
        Err(super::super::common::SdkError::MismatchError {
          expected: "w:Relationships".to_string(),
          found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
        })?;
      }
    } else if e.name().local_name().as_ref() != b"Relationships" {
      Err(super::super::common::SdkError::MismatchError {
        expected: "Relationships".to_string(),
        found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
      })?;
    }

    if !empty_tag {
      loop {
        let peek_event = xml_reader.peek()?;
        match peek_event {
          quick_xml::events::Event::Start(e) | quick_xml::events::Event::Empty(e) => {
            if with_xmlns {
              match e.name().as_ref() {
                b"w:Relationship" => {
                  relationship.push(Relationship::deserialize_self(xml_reader, with_xmlns)?);
                }
                _ => Err(super::super::common::SdkError::CommonError(
                  "Relationships".to_string(),
                ))?,
              }
            } else {
              match e.name().local_name().as_ref() {
                b"Relationship" => {
                  relationship.push(Relationship::deserialize_self(xml_reader, with_xmlns)?);
                }
                _ => Err(super::super::common::SdkError::CommonError(
                  "Relationships".to_string(),
                ))?,
              }
            }
          }
          quick_xml::events::Event::End(e) => {
            if with_xmlns {
              if e.name().as_ref() == b"w:Relationships" {
                break;
              }
            } else if e.name().local_name().as_ref() == b"Relationships" {
              break;
            }

            xml_reader.next()?;
          }
          quick_xml::events::Event::Eof => Err(super::super::common::SdkError::UnknownError)?,
          _ => {
            xml_reader.next()?;
          }
        }
      }
    }

    Ok(Self {
      xmlns,
      xmlns_map,
      mc_ignorable,
      relationship,
    })
  }
}

impl Relationships {
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> Result<String, super::super::common::SdkError> {
    self.to_string_inner(if let Some(xmlns) = &self.xmlns {
      xmlns != "http://schemas.openxmlformats.org/package/2006/relationships"
    } else {
      true
    })
  }

  pub fn to_string_inner(
    &self,
    with_xmlns: bool,
  ) -> Result<String, super::super::common::SdkError> {
    use std::fmt::Write;

    let mut writer = String::new();

    writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n")?;

    writer.write_char('<')?;

    if with_xmlns {
      writer.write_str("w:Relationships")?;
    } else {
      writer.write_str("Relationships")?;
    }

    if let Some(xmlns) = &self.xmlns {
      writer.write_str(r#" xmlns=""#)?;
      writer.write_str(xmlns)?;
      writer.write_str("\"")?;
    }

    for (k, v) in &self.xmlns_map {
      writer.write_str(" xmlns:")?;
      writer.write_str(k)?;
      writer.write_str("=\"")?;
      writer.write_str(v)?;
      writer.write_str("\"")?;
    }

    if let Some(mc_ignorable) = &self.mc_ignorable {
      writer.write_str(r#" mc:Ignorable=""#)?;
      writer.write_str(mc_ignorable)?;
      writer.write_str("\"")?;
    }

    writer.write_char('>')?;

    for child in &self.relationship {
      let child_str = child.to_string_inner(with_xmlns)?;

      writer.write_str(&child_str)?;
    }

    writer.write_str("</")?;

    if with_xmlns {
      writer.write_str("w:Relationships")?;
    } else {
      writer.write_str("Relationships")?;
    }

    writer.write_char('>')?;

    Ok(writer)
  }
}

#[derive(Clone)]
pub struct Relationship {
  pub target_mode: Option<TargetMode>,
  pub target: String,
  pub r#type: String,
  pub id: String,
}

impl Relationship {
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(s: &str) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_str_inner(s)?;

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_reader_inner(reader)?;

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn deserialize_self<'de, R: super::super::common::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut with_xmlns = with_xmlns;

    if let quick_xml::events::Event::Empty(e) = xml_reader.next()? {
      let mut target_mode = None;

      let mut target = None;

      let mut r#type = None;

      let mut id = None;

      for attr in e.attributes() {
        let attr = attr?;
        match attr.key.as_ref() {
          b"TargetMode" => {
            target_mode = Some(TargetMode::from_str(
              &attr.decode_and_unescape_value(xml_reader.decoder())?,
            )?);
          }
          b"Target" => {
            target = Some(
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .to_string(),
            );
          }
          b"Type" => {
            r#type = Some(
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .to_string(),
            );
          }
          b"Id" => {
            id = Some(
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .to_string(),
            );
          }
          b"xmlns:w" => with_xmlns = true,
          _ => {}
        }
      }

      if with_xmlns {
        if e.name().as_ref() != b"w:Relationship" {
          Err(super::super::common::SdkError::MismatchError {
            expected: "w:Relationship".to_string(),
            found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
          })?;
        }
      } else if e.name().local_name().as_ref() != b"Relationship" {
        Err(super::super::common::SdkError::MismatchError {
          expected: "Relationship".to_string(),
          found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
        })?;
      }

      let target =
        target.ok_or_else(|| super::super::common::SdkError::CommonError("target".to_string()))?;

      let r#type =
        r#type.ok_or_else(|| super::super::common::SdkError::CommonError("type".to_string()))?;

      let id = id.ok_or_else(|| super::super::common::SdkError::CommonError("id".to_string()))?;

      Ok(Self {
        target_mode,
        target,
        r#type,
        id,
      })
    } else {
      Err(super::super::common::SdkError::CommonError(
        "FrameProperties".to_string(),
      ))?
    }
  }
}

impl Relationship {
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> Result<String, super::super::common::SdkError> {
    self.to_string_inner(false)
  }

  pub fn to_string_inner(
    &self,
    with_xmlns: bool,
  ) -> Result<String, super::super::common::SdkError> {
    use std::fmt::Write;

    let mut writer = String::new();

    writer.write_char('<')?;

    if with_xmlns {
      writer.write_str("w:Relationship")?;
    } else {
      writer.write_str("Relationship")?;
    }

    if let Some(target_mode) = &self.target_mode {
      writer.write_char(' ')?;
      writer.write_str("TargetMode")?;
      writer.write_str("=\"")?;
      writer.write_str(&quick_xml::escape::escape(&target_mode.to_string()))?;
      writer.write_char('"')?;
    }

    writer.write_char(' ')?;
    writer.write_str("Target")?;
    writer.write_str("=\"")?;
    writer.write_str(&quick_xml::escape::escape(&self.target.to_string()))?;
    writer.write_char('"')?;

    writer.write_char(' ')?;
    writer.write_str("Type")?;
    writer.write_str("=\"")?;
    writer.write_str(&quick_xml::escape::escape(&self.r#type.to_string()))?;
    writer.write_char('"')?;

    writer.write_char(' ')?;
    writer.write_str("Id")?;
    writer.write_str("=\"")?;
    writer.write_str(&quick_xml::escape::escape(&self.id.to_string()))?;
    writer.write_char('"')?;

    writer.write_str("/>")?;

    Ok(writer)
  }
}

#[derive(Clone)]
pub enum TargetMode {
  External,
  Internal,
}

impl TargetMode {
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(s: &str) -> Result<Self, super::super::common::SdkError> {
    match s {
      "External" => Ok(Self::External),
      "Internal" => Ok(Self::Internal),
      _ => Err(super::super::common::SdkError::CommonError(s.to_string())),
    }
  }
}

impl TargetMode {
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    match self {
      Self::External => "External".to_string(),
      Self::Internal => "Internal".to_string(),
    }
  }
}
