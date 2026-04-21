pub trait SdkEnum {
  fn as_xml_str(&self) -> &'static str;

  fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}

pub trait SdkType: Sized {}

pub trait SdkChoice: Sized {
  fn matches_specific_start_qname(name: &[u8]) -> bool;

  #[inline]
  fn matches_start_qname(name: &[u8]) -> bool {
    Self::matches_specific_start_qname(name)
  }

  #[inline]
  fn accepts_any_child() -> bool {
    false
  }

  #[inline]
  fn accepts_text() -> bool {
    false
  }

  #[inline]
  fn from_text_value(_value: &str) -> Option<Self> {
    None
  }
}

pub trait SdkPart: Sized {}
