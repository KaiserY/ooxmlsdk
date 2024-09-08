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
  type Err = super::deserializers::DeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "visible" => Ok(Self::Visible),
      "hidden" => Ok(Self::Hidden),
      "veryHidden" => Ok(Self::VeryHidden),
      _ => Err(Self::Err::UnknownError),
    }
  }
}

impl Sheet {
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(s: &str) -> Result<Self, super::deserializers::DeError> {
    let mut xml_reader = super::deserializers::SliceReader::new(quick_xml::Reader::from_str(s));

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::deserializers::DeError> {
    let mut xml_reader =
      super::deserializers::IoReader::new(quick_xml::Reader::from_reader(reader));

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn deserialize_self<'de, R: super::deserializers::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
  ) -> Result<Self, super::deserializers::DeError> {
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
          Err(super::deserializers::DeError::UnknownError)?;
        }
      } else if e.name().local_name().as_ref() != b"sheet" {
        Err(super::deserializers::DeError::UnknownError)?;
      }

      let name = name.ok_or_else(|| super::deserializers::DeError::UnknownError)?;
      let sheet_id = sheet_id.ok_or_else(|| super::deserializers::DeError::UnknownError)?;
      let id = id.ok_or_else(|| super::deserializers::DeError::UnknownError)?;

      Ok(Self {
        name,
        sheet_id,
        state,
        id,
      })
    } else {
      Err(super::deserializers::DeError::UnknownError)?
    }
  }
}

impl Sheets {
  #[allow(clippy::should_implement_trait)]
  pub fn from_str(s: &str) -> Result<Self, super::deserializers::DeError> {
    let mut xml_reader = super::deserializers::SliceReader::new(quick_xml::Reader::from_str(s));

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::deserializers::DeError> {
    let mut xml_reader =
      super::deserializers::IoReader::new(quick_xml::Reader::from_reader(reader));

    Self::deserialize_self(&mut xml_reader, false)
  }

  pub fn deserialize_self<'de, R: super::deserializers::XmlReader<'de>>(
    xml_reader: &mut R,
    with_xmlns: bool,
  ) -> Result<Self, super::deserializers::DeError> {
    let mut with_xmlns = with_xmlns;

    if let quick_xml::events::Event::Start(e) = xml_reader.next()? {
      let mut children = vec![];

      for attr in e.attributes() {
        let attr = attr?;

        if attr.key.as_ref() == b"xmlns:x" {
          with_xmlns = true
        }
      }

      if with_xmlns {
        if e.name().as_ref() != b"x:sheets" {
          Err(super::deserializers::DeError::UnknownError)?;
        }
      } else if e.name().local_name().as_ref() != b"sheets" {
        Err(super::deserializers::DeError::UnknownError)?;
      }

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
                _ => Err(super::deserializers::DeError::UnknownError)?,
              }
            } else {
              match e.name().local_name().as_ref() {
                b"sheet" => {
                  children.push(SheetsChildChoice::Sheet(std::boxed::Box::new(
                    Sheet::deserialize_self(xml_reader, with_xmlns)?,
                  )));
                }
                _ => Err(super::deserializers::DeError::UnknownError)?,
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
          quick_xml::events::Event::Eof => Err(super::deserializers::DeError::UnknownError)?,
          _ => (),
        }
      }

      Ok(Self { children })
    } else {
      Err(super::deserializers::DeError::UnknownError)?
    }
  }
}

pub fn gen() {
  let xml = r###"
<x:sheets xmlns:x="aa"><x:sheet r:id="rId7" name="Sheet1" sheetId="1" /></x:sheets>
"###
    .trim();

  let value: Sheets = Sheets::from_str(xml).unwrap();

  println!("{:?}", value);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gen() {
    gen();
  }
}
