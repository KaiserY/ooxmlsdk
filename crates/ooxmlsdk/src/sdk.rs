pub trait SdkEnum {
  fn as_xml_str(&self) -> &'static str;

  fn to_xml(&self) -> String {
    self.as_xml_str().to_string()
  }
}

pub trait SdkType: Sized {}

pub trait SdkChoice: Sized {}

pub trait SdkPart: Sized {}
