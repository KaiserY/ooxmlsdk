#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename(serialize = "x:sheets", deserialize = "sheets"))]
pub struct Sheets {
  #[serde(default, rename = "$value")]
  pub children: Vec<SheetsChildChoice>,
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum SheetsChildChoice {
  #[serde(rename(serialize = "x:sheet", deserialize = "sheet"))]
  Sheet(std::boxed::Box<Sheet>),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename(serialize = "x:sheet", deserialize = "sheet"))]
pub struct Sheet {
  /// Sheet Name
  #[serde(rename(serialize = "@name", deserialize = "@name"))]
  pub name: super::simple_type::StringValue,
  /// Sheet Tab Id
  #[serde(rename(serialize = "@sheetId", deserialize = "@sheetId"))]
  pub sheet_id: super::simple_type::UInt32Value,
  /// Visible State
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename(serialize = "@state", deserialize = "@state"))]
  pub state: Option<SheetStateValues>,
  /// Relationship Id
  #[serde(rename(serialize = "@r:id", deserialize = "@id"))]
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

    Self::deserialize_self(&mut xml_reader)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::deserializers::DeError> {
    let mut xml_reader =
      super::deserializers::IoReader::new(quick_xml::Reader::from_reader(reader));

    Self::deserialize_self(&mut xml_reader)
  }

  pub fn deserialize_self<'de, R: super::deserializers::XmlReader<'de>>(
    xml_reader: &mut R,
  ) -> Result<Self, super::deserializers::DeError> {
    if let quick_xml::events::Event::Empty(e) = xml_reader.next()? {
      use std::str::FromStr;

      if e.name().local_name().as_ref() != b"sheet" {
        Err(super::deserializers::DeError::UnknownError)?;
      }

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
          _ => (),
        }
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

    Self::deserialize_self(&mut xml_reader)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::deserializers::DeError> {
    let mut xml_reader =
      super::deserializers::IoReader::new(quick_xml::Reader::from_reader(reader));

    Self::deserialize_self(&mut xml_reader)
  }

  pub fn deserialize_self<'de, R: super::deserializers::XmlReader<'de>>(
    xml_reader: &mut R,
  ) -> Result<Self, super::deserializers::DeError> {
    if let quick_xml::events::Event::Start(e) = xml_reader.next()? {
      if e.name().local_name().as_ref() != b"sheets" {
        Err(super::deserializers::DeError::UnknownError)?;
      }

      let mut children = vec![];

      loop {
        let peek_event = xml_reader.peek()?;

        match peek_event {
          quick_xml::events::Event::Start(e) | quick_xml::events::Event::Empty(e) => {
            match e.name().local_name().as_ref() {
              b"sheet" => {
                children.push(SheetsChildChoice::Sheet(std::boxed::Box::new(
                  Sheet::deserialize_self(xml_reader)?,
                )));
              }
              _ => Err(super::deserializers::DeError::UnknownError)?,
            }
          }
          quick_xml::events::Event::End(e) => {
            if e.name().local_name().as_ref() == b"sheets" {
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
<sheets><sheet r:id="rId7" name="Sheet1" sheetId="1" /></sheets>
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
