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
      if e.name().local_name().as_ref() != b"sheet" {
        Err(super::deserializers::DeError::UnknownError)?;
      }

      let mut name = None;
      let mut sheet_id = None;

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
          _ => (),
        }
      }

      let name = name.ok_or_else(|| super::deserializers::DeError::UnknownError)?;
      let sheet_id = sheet_id.ok_or_else(|| super::deserializers::DeError::UnknownError)?;

      Ok(Self {
        name,
        sheet_id,
        state: None,
        id: "".to_string(),
      })
    } else {
      Err(super::deserializers::DeError::UnknownError)?
    }
  }
}

pub fn gen() {
  let xml = "visible".trim();

  let value: SheetStateValues = quick_xml::de::from_str(xml).unwrap();

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
