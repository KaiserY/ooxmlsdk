#[derive(Clone, Debug)]
pub struct Sheets {
  pub children: Vec<SheetsChildChoice>,
}
#[derive(Clone, Debug)]
pub enum SheetsChildChoice {
  Sheet(std::boxed::Box<Sheet>),
}

#[derive(Clone, Debug)]
pub struct Sheet {
  /// Sheet Name
  pub name: super::simple_type::StringValue,
  /// Sheet Tab Id
  pub sheet_id: super::simple_type::UInt32Value,
  /// Visible State
  pub state: Option<SheetStateValues>,
  /// Relationship Id
  pub id: super::simple_type::StringValue,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum SheetStateValues {
  #[serde(rename = "visible")]
  Visible,
  #[serde(rename = "hidden")]
  Hidden,
  #[serde(rename = "veryHidden")]
  VeryHidden,
}

impl std::str::FromStr for SheetStateValues {
  type Err = super::deserializer_common::DeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "visible" => Ok(Self::Visible),
      "hidden" => Ok(Self::Hidden),
      "veryHidden" => Ok(Self::VeryHidden),
      _ => Err(Self::Err::UnknownError),
    }
  }
}

impl SheetStateValues {
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> String {
    match self {
      Self::Visible => "visible".to_string(),
      Self::Hidden => "hidden".to_string(),
      Self::VeryHidden => "veryHidden".to_string(),
    }
  }
}

impl Sheet {
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self, with_xmlns: bool) -> Result<String, super::serializer_common::SeError> {
    use std::fmt::Write;

    let mut writer = String::new();

    writer.write_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#)?;

    writer.write_char('<')?;

    if with_xmlns {
      writer.write_str("x:sheet")?;
    } else {
      writer.write_str("sheet")?;
    }

    writer.write_char('<')?;

    Ok(writer)
  }
}

impl Sheet {
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(s: &str) -> Result<Self, super::deserializer_common::DeError> {
    let mut xml_reader =
      super::deserializer_common::SliceReader::new(quick_xml::Reader::from_str(s));

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::deserializer_common::DeError> {
    let mut xml_reader =
      super::deserializer_common::IoReader::new(quick_xml::Reader::from_reader(reader));

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn deserialize_self<'de, R: super::deserializer_common::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
  ) -> Result<Self, super::deserializer_common::DeError> {
    let mut with_xmlns = with_xmlns;

    if let quick_xml::events::Event::Empty(e) = xml_reader.next()? {
      use std::str::FromStr;

      let mut name = None;
      let mut sheet_id = None;
      let mut state = None;
      let mut id = None;

      for attr in e.attributes() {
        let attr = attr?;

        match attr.key.as_ref() {
          b"name" => {
            name = Some(
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .to_string(),
            )
          }
          b"sheetId" => {
            sheet_id = Some(
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .parse::<super::simple_type::UInt32Value>()?,
            )
          }
          b"state" => {
            state = Some(SheetStateValues::from_str(
              &attr.decode_and_unescape_value(xml_reader.decoder())?,
            )?)
          }
          b"r:id" => {
            id = Some(
              attr
                .decode_and_unescape_value(xml_reader.decoder())?
                .to_string(),
            )
          }
          b"xmlns:x" => with_xmlns = true,
          _ => (),
        }
      }

      if with_xmlns {
        if e.name().as_ref() != b"x:sheet" {
          Err(super::deserializer_common::DeError::MismatchError {
            expected: "x:sheet".to_string(),
            found: String::from_utf8_lossy(e.name().as_ref()).to_string(),
          })?;
        }
      } else if e.name().local_name().as_ref() != b"sheet" {
        Err(super::deserializer_common::DeError::MismatchError {
          expected: "sheet".to_string(),
          found: String::from_utf8_lossy(e.name().into_inner()).to_string(),
        })?;
      }

      let name =
        name.ok_or_else(|| super::deserializer_common::DeError::CommonError("name".to_string()))?;
      let sheet_id = sheet_id
        .ok_or_else(|| super::deserializer_common::DeError::CommonError("sheet_id".to_string()))?;
      let id =
        id.ok_or_else(|| super::deserializer_common::DeError::CommonError("id".to_string()))?;

      Ok(Self {
        name,
        sheet_id,
        state,
        id,
      })
    } else {
      Err(super::deserializer_common::DeError::CommonError(
        "Sheet".to_string(),
      ))?
    }
  }
}

impl Sheets {
  #[allow(clippy::inherent_to_string)]
  pub fn to_string(&self) -> Result<String, super::serializer_common::SeError> {
    use std::fmt::Write;

    let mut writer = String::new();

    writer.write_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#)?;

    Ok(writer)
  }
}

impl Sheets {
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(s: &str) -> Result<Self, super::deserializer_common::DeError> {
    use crate::includes::deserializer_common::XmlReader;

    let mut xml_reader = quick_xml::Reader::from_str(s);

    xml_reader.config_mut().trim_text(true);

    let mut xml_reader = super::deserializer_common::SliceReader::new(xml_reader);

    if let quick_xml::events::Event::Decl(_) = xml_reader.peek()? {
      xml_reader.next()?;
    }

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::deserializer_common::DeError> {
    use crate::includes::deserializer_common::XmlReader;

    let mut xml_reader = quick_xml::Reader::from_reader(reader);

    xml_reader.config_mut().trim_text(true);

    let mut xml_reader = super::deserializer_common::IoReader::new(xml_reader);

    if let quick_xml::events::Event::Decl(_) = xml_reader.peek()? {
      xml_reader.next()?;
    }

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn deserialize_self<'de, R: super::deserializer_common::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
  ) -> Result<Self, super::deserializer_common::DeError> {
    let mut with_xmlns = with_xmlns;

    let mut empty_tag = false;

    let e = match xml_reader.next()? {
      quick_xml::events::Event::Start(e) => e,
      quick_xml::events::Event::Empty(e) => {
        empty_tag = true;

        e
      }
      _ => Err(super::deserializer_common::DeError::UnknownError)?,
    };

    let mut children = vec![];

    for attr in e.attributes() {
      let attr = attr?;

      if attr.key.as_ref() == b"xmlns:x" {
        with_xmlns = true
      }
    }

    if with_xmlns {
      if e.name().as_ref() != b"x:sheets" {
        Err(super::deserializer_common::DeError::UnknownError)?;
      }
    } else if e.name().local_name().as_ref() != b"sheets" {
      Err(super::deserializer_common::DeError::UnknownError)?;
    }

    if !empty_tag {
      loop {
        let peek_event = xml_reader.peek()?;

        match peek_event {
          quick_xml::events::Event::Start(e) | quick_xml::events::Event::Empty(e) => {
            if with_xmlns {
              match e.name().as_ref() {
                b"x:sheet" => {
                  children.push(SheetsChildChoice::Sheet(std::boxed::Box::new(
                    Sheet::deserialize_self(xml_reader, with_xmlns)?,
                  )));
                }
                _ => Err(super::deserializer_common::DeError::UnknownError)?,
              }
            } else {
              match e.name().local_name().as_ref() {
                b"sheet" => {
                  children.push(SheetsChildChoice::Sheet(std::boxed::Box::new(
                    Sheet::deserialize_self(xml_reader, with_xmlns)?,
                  )));
                }
                _ => Err(super::deserializer_common::DeError::UnknownError)?,
              }
            }
          }
          quick_xml::events::Event::End(e) => {
            if with_xmlns {
              if e.name().as_ref() == b"x:sheets" {
                xml_reader.next()?;

                break;
              }
            } else if e.name().local_name().as_ref() == b"sheets" {
              xml_reader.next()?;

              break;
            }
          }
          quick_xml::events::Event::Eof => Err(super::deserializer_common::DeError::UnknownError)?,
          _ => (),
        }
      }
    }

    Ok(Self { children })
  }
}

pub fn gen() {
  let xml = r###"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<sheets><sheet r:id="rId7" name="Sheet1" sheetId="1" /></sheets>
"###
    .trim();

  let value: Sheets = Sheets::from_str(xml).unwrap();

  println!("{:?}", value);

  println!("{}", value.to_string().unwrap());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gen() {
    gen();
  }
}
