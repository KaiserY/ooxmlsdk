#[derive(Clone, Debug, Default)]
pub struct Types {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub mc_ignorable: Option<String>,
  pub children: Vec<TypesChildChoice>,
}

#[derive(Clone, Debug, Default)]
pub enum TypesChildChoice {
  Default(std::boxed::Box<Default>),
  Override(std::boxed::Box<Override>),
  #[default]
  None,
}

impl std::str::FromStr for Types {
  type Err = super::super::common::SdkError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = super::super::common::from_str_inner(s)?;

    Self::deserialize_inner(&mut xml_reader, false, false, None)
  }
}

impl Types {
  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_reader_inner(reader)?;

    Self::deserialize_inner(&mut xml_reader, false, false, None)
  }

  #[inline(always)]
  pub(crate) fn deserialize_inner<'de, R: super::super::common::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
    mut empty_tag: bool,
    xml_event: Option<quick_xml::events::BytesStart<'de>>,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut with_xmlns = with_xmlns;
    let mut xmlns = None;
    let mut xmlns_map = std::collections::HashMap::<String, String>::new();
    let mut mc_ignorable = None;

    let mut children = vec![];

    let e = if let Some(e) = xml_event {
      e
    } else {
      let e = match xml_reader.next()? {
        quick_xml::events::Event::Start(e) => e,
        quick_xml::events::Event::Empty(e) => {
          empty_tag = true;

          e
        }
        _ => Err(super::super::common::SdkError::CommonError(
          "Types".to_string(),
        ))?,
      };

      if with_xmlns && e.name().as_ref() != b"w:Types" {
        Err(super::super::common::SdkError::MismatchError {
          expected: "w:Types".to_string(),
          found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
        })?;
      } else if e.name().as_ref() != b"Types" {
        Err(super::super::common::SdkError::MismatchError {
          expected: "Types".to_string(),
          found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
        })?;
      }

      e
    };

    for attr in e.attributes().with_checks(false) {
      let attr = attr?;

      match attr.key.as_ref() {
        b"xmlns" => {
          xmlns = Some(attr.unescape_value()?.into_owned());
        }
        b"mc:Ignorable" => {
          mc_ignorable = Some(attr.unescape_value()?.into_owned());
        }
        key => {
          if key.starts_with(b"xmlns:") {
            xmlns_map.insert(
              String::from_utf8_lossy(&key[6..]).to_string(),
              attr.unescape_value()?.into_owned(),
            );

            if key == b"xmlns:w" {
              with_xmlns = true;
            }
          }
        }
      }
    }

    if !empty_tag {
      loop {
        let mut e_opt: Option<quick_xml::events::BytesStart<'_>> = None;
        let mut e_empty = false;

        match xml_reader.next()? {
          quick_xml::events::Event::Start(e) => {
            e_opt = Some(e);
          }
          quick_xml::events::Event::Empty(e) => {
            e_empty = true;
            e_opt = Some(e);
          }
          quick_xml::events::Event::End(e) => {
            if with_xmlns {
              if e.name().as_ref() == b"w:Types" {
                break;
              }
            } else if e.name().as_ref() == b"Types" {
              break;
            }
          }
          quick_xml::events::Event::Eof => Err(super::super::common::SdkError::UnknownError)?,
          _ => (),
        }

        if let Some(e) = e_opt {
          if with_xmlns {
            match e.name().as_ref() {
              b"w:Default" => {
                children.push(TypesChildChoice::Default(std::boxed::Box::new(
                  Default::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
                )));
              }
              b"w:Override" => {
                children.push(TypesChildChoice::Override(std::boxed::Box::new(
                  Override::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
                )));
              }
              _ => Err(super::super::common::SdkError::CommonError(
                "Types".to_string(),
              ))?,
            }
          } else {
            match e.name().as_ref() {
              b"Default" => {
                children.push(TypesChildChoice::Default(std::boxed::Box::new(
                  Default::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
                )));
              }
              b"Override" => {
                children.push(TypesChildChoice::Override(std::boxed::Box::new(
                  Override::deserialize_inner(xml_reader, with_xmlns, e_empty, Some(e))?,
                )));
              }
              _ => Err(super::super::common::SdkError::CommonError(
                "Types".to_string(),
              ))?,
            }
          }
        }
      }
    }

    Ok(Self {
      xmlns,
      xmlns_map,
      mc_ignorable,
      children,
    })
  }
}

impl Types {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    self.to_string_inner(if let Some(xmlns) = &self.xmlns {
      xmlns != "http://schemas.openxmlformats.org/package/2006/content-types"
    } else {
      true
    })
  }

  pub fn to_string_inner(&self, with_xmlns: bool) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut writer = String::new();

    writer.write_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n")?;

    writer.write_char('<')?;

    if with_xmlns {
      writer.write_str("w:Types")?;
    } else {
      writer.write_str("Types")?;
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

    for child in &self.children {
      let child_str = match child {
        TypesChildChoice::Default(child) => child.to_string_with_xmlns(with_xmlns)?,
        TypesChildChoice::Override(child) => child.to_string_with_xmlns(with_xmlns)?,
        TypesChildChoice::None => "".to_string(),
      };
      writer.write_str(&child_str)?;
    }

    writer.write_str("</")?;

    if with_xmlns {
      writer.write_str("w:Types")?;
    } else {
      writer.write_str("Types")?;
    }

    writer.write_char('>')?;

    Ok(writer)
  }
}

#[derive(Clone, Debug, Default)]
pub struct Default {
  pub extension: String,
  pub content_type: String,
}

impl std::str::FromStr for Default {
  type Err = super::super::common::SdkError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = super::super::common::from_str_inner(s)?;

    Self::deserialize_inner(&mut xml_reader, false, false, None)
  }
}

impl Default {
  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_reader_inner(reader)?;

    Self::deserialize_inner(&mut xml_reader, false, false, None)
  }

  #[inline(always)]
  pub fn deserialize_inner<'de, R: super::super::common::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
    mut empty_tag: bool,
    xml_event: Option<quick_xml::events::BytesStart<'de>>,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut extension = None;
    let mut content_type = None;

    let e = if let Some(e) = xml_event {
      e
    } else {
      let e = match xml_reader.next()? {
        quick_xml::events::Event::Start(e) => e,
        quick_xml::events::Event::Empty(e) => {
          empty_tag = true;

          e
        }
        _ => {
          return Err(super::super::common::SdkError::CommonError(
            "Default".into(),
          ))
        }
      };

      if with_xmlns {
        if e.name().as_ref() != b"w:Default" {
          Err(super::super::common::SdkError::MismatchError {
            expected: "w:Default".to_string(),
            found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
          })?;
        }
      } else if e.name().as_ref() != b"Default" {
        Err(super::super::common::SdkError::MismatchError {
          expected: "Default".to_string(),
          found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
        })?;
      }

      e
    };

    for attr in e.attributes().with_checks(false) {
      let attr = attr?;

      match attr.key.as_ref() {
        b"Extension" => {
          extension = Some(attr.unescape_value()?.into_owned());
        }
        b"ContentType" => {
          content_type = Some(attr.unescape_value()?.into_owned());
        }
        _ => {}
      }
    }

    if !empty_tag {
      loop {
        match xml_reader.next()? {
          quick_xml::events::Event::End(e) => {
            if with_xmlns {
              if e.name().as_ref() == b"w:Default" {
                break;
              }
            } else if e.name().as_ref() == b"Default" {
              break;
            }
          }
          quick_xml::events::Event::Eof => Err(super::super::common::SdkError::UnknownError)?,
          _ => (),
        }
      }
    }

    let extension = extension
      .ok_or_else(|| super::super::common::SdkError::CommonError("extension".to_string()))?;

    let content_type = content_type
      .ok_or_else(|| super::super::common::SdkError::CommonError("content_type".to_string()))?;

    Ok(Self {
      extension,
      content_type,
    })
  }
}

impl Default {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    self.to_string_with_xmlns(false)
  }

  pub fn to_string_with_xmlns(&self, with_xmlns: bool) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut writer = String::new();

    writer.write_char('<')?;

    if with_xmlns {
      writer.write_str("w:Default")?;
    } else {
      writer.write_str("Default")?;
    }

    writer.write_char(' ')?;
    writer.write_str("Extension")?;
    writer.write_str("=\"")?;
    writer.write_str(&quick_xml::escape::escape(self.extension.to_string()))?;
    writer.write_char('"')?;

    writer.write_char(' ')?;
    writer.write_str("ContentType")?;
    writer.write_str("=\"")?;
    writer.write_str(&quick_xml::escape::escape(self.content_type.to_string()))?;
    writer.write_char('"')?;

    writer.write_str("/>")?;

    Ok(writer)
  }
}

#[derive(Clone, Debug, Default)]
pub struct Override {
  pub content_type: String,
  pub part_name: String,
}

impl std::str::FromStr for Override {
  type Err = super::super::common::SdkError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut xml_reader = super::super::common::from_str_inner(s)?;

    Self::deserialize_inner(&mut xml_reader, false, false, None)
  }
}

impl Override {
  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut xml_reader = super::super::common::from_reader_inner(reader)?;

    Self::deserialize_inner(&mut xml_reader, false, false, None)
  }

  #[inline(always)]
  pub fn deserialize_inner<'de, R: super::super::common::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
    mut empty_tag: bool,
    xml_event: Option<quick_xml::events::BytesStart<'de>>,
  ) -> Result<Self, super::super::common::SdkError> {
    let mut content_type = None;
    let mut part_name = None;

    let e = if let Some(e) = xml_event {
      e
    } else {
      let e = match xml_reader.next()? {
        quick_xml::events::Event::Start(e) => e,
        quick_xml::events::Event::Empty(e) => {
          empty_tag = true;

          e
        }
        _ => {
          return Err(super::super::common::SdkError::CommonError(
            "Override".into(),
          ))
        }
      };

      if with_xmlns {
        if e.name().as_ref() != b"w:Override" {
          Err(super::super::common::SdkError::MismatchError {
            expected: "w:Override".to_string(),
            found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
          })?;
        }
      } else if e.name().as_ref() != b"Override" {
        Err(super::super::common::SdkError::MismatchError {
          expected: "Override".to_string(),
          found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
        })?;
      }

      e
    };

    for attr in e.attributes().with_checks(false) {
      let attr = attr?;

      match attr.key.as_ref() {
        b"ContentType" => {
          content_type = Some(attr.unescape_value()?.into_owned());
        }
        b"PartName" => {
          part_name = Some(attr.unescape_value()?.into_owned());
        }
        _ => {}
      }
    }

    if !empty_tag {
      loop {
        match xml_reader.next()? {
          quick_xml::events::Event::End(e) => {
            if with_xmlns {
              if e.name().as_ref() == b"w:Override" {
                break;
              }
            } else if e.name().as_ref() == b"Override" {
              break;
            }
          }
          quick_xml::events::Event::Eof => Err(super::super::common::SdkError::UnknownError)?,
          _ => (),
        }
      }
    }

    let content_type = content_type
      .ok_or_else(|| super::super::common::SdkError::CommonError("content_type".into()))?;

    let part_name = part_name
      .ok_or_else(|| super::super::common::SdkError::CommonError("part_name".to_string()))?;

    Ok(Self {
      content_type,
      part_name,
    })
  }
}

impl std::fmt::Display for Override {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.to_string_with_xmlns(false)?)
  }
}

impl Override {
  pub fn to_xml(&self) -> Result<String, std::fmt::Error> {
    self.to_string_with_xmlns(false)
  }

  pub fn to_string_with_xmlns(&self, with_xmlns: bool) -> Result<String, std::fmt::Error> {
    use std::fmt::Write;

    let mut writer = String::new();

    if with_xmlns {
      writer.write_str("<w:Override")?;
    } else {
      writer.write_str("<Override")?;
    }

    writer.write_str(" ContentType=\"")?;
    writer.write_str(&quick_xml::escape::escape(&self.content_type))?;
    writer.write_char('"')?;

    writer.write_str(" PartName=\"")?;
    writer.write_str(&quick_xml::escape::escape(&self.part_name))?;
    writer.write_char('"')?;

    writer.write_str("/>")?;

    Ok(writer)
  }
}
