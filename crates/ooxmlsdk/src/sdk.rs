pub trait SdkEnum {
  fn as_xml_str(&self) -> &'static str;

  fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}

pub trait SdkType: Sized {
  fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
    _xml_reader: &mut R,
    _xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
  ) -> Result<Self, crate::common::SdkError> {
    Err(crate::common::SdkError::CommonError(
      "SdkType::deserialize_inner is not implemented".to_string(),
    ))
  }

  fn write_xml<W: std::fmt::Write>(
    &self,
    _writer: &mut W,
    _xmlns_prefix: &str,
  ) -> Result<(), std::fmt::Error> {
    Ok(())
  }
}

pub trait SdkChoice: Sized {
  fn deserialize_inner<'de, R: crate::common::XmlReader<'de>>(
    _xml_reader: &mut R,
    _xml_event: Option<(quick_xml::events::BytesStart<'de>, bool)>,
  ) -> Result<Self, crate::common::SdkError> {
    Err(crate::common::SdkError::CommonError(
      "SdkChoice::deserialize_inner is not implemented".to_string(),
    ))
  }

  fn write_xml<W: std::fmt::Write>(
    &self,
    _writer: &mut W,
    _xmlns_prefix: &str,
  ) -> Result<(), std::fmt::Error> {
    Ok(())
  }
}

pub trait SdkPart: Sized {}
