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
  pub fn from_str<'de>(input: &'de str) -> Result<Self, super::deserializers::DeError> {
    Err(super::deserializers::DeError::UnknownError)
  }

  pub fn from_reader<R: std::io::BufRead>(
    reader: R,
  ) -> Result<Self, super::deserializers::DeError> {
    Err(super::deserializers::DeError::UnknownError)
  }
}

pub fn gen() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_gen() {
    gen();
  }
}
