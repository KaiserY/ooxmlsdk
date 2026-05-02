//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RichValueFallbackType {
  #[sdk(rename = "b")]
  #[default]
  B,
  #[sdk(rename = "n")]
  N,
  #[sdk(rename = "e")]
  E,
  #[sdk(rename = "s")]
  S,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RichValueValueType {
  #[sdk(rename = "d")]
  #[default]
  D,
  #[sdk(rename = "i")]
  I,
  #[sdk(rename = "b")]
  B,
  #[sdk(rename = "e")]
  E,
  #[sdk(rename = "s")]
  S,
  #[sdk(rename = "r")]
  R,
  #[sdk(rename = "a")]
  A,
  #[sdk(rename = "spb")]
  Spb,
}
/// Defines the RichValueBlock Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd:CT_RichValueBlock/xlrd:rvb")]
pub struct RichValueBlock {
  /// i
  #[sdk(attr(office2019, qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
}
/// Defines the RichValueData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd:CT_RichValueData/xlrd:rvData")]
pub struct RichValueData {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// count
  #[sdk(attr(office2019, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the RichValue Class.
  #[sdk(child(office2019, qname = "xlrd:CT_RichValue/xlrd:rv"))]
  pub xlrd_rv: Vec<RichValue>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd:extLst"))]
  pub xlrd_ext_lst: Option<ExtensionList>,
}
/// Defines the RichValueStructures Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd:CT_RichValueStructures/xlrd:rvStructures")]
pub struct RichValueStructures {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// count
  #[sdk(attr(office2019, qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// Defines the RichValueStructure Class.
  #[sdk(child(office2019, qname = "xlrd:CT_RichValueStructure/xlrd:s"))]
  pub xlrd_s: Vec<RichValueStructure>,
  /// Defines the ExtensionList Class.
  #[sdk(child(office2019, qname = "x:CT_ExtensionList/xlrd:extLst"))]
  pub xlrd_ext_lst: Option<ExtensionList>,
}
/// Defines the RichValue Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd:CT_RichValue/xlrd:rv")]
pub struct RichValue {
  /// s
  #[sdk(attr(office2019, qname = ":s"))]
  pub s: crate::simple_type::UInt32Value,
  /// Defines the RichValueFallback Class.
  #[sdk(child(office2019, qname = "xlrd:CT_RichValueFallback/xlrd:fb"))]
  pub rich_value_fallback: Option<RichValueFallback>,
  /// Defines the Value Class.
  #[sdk(text_child(office2019, qname = "xlrd:CT_Value/xlrd:v"))]
  pub xlrd_v: Vec<crate::simple_type::StringValue>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "x:CT_ExtensionList/xlrd:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub x_ext: Vec<crate::schemas::x::Extension>,
}
/// Defines the RichValueFallback Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd:CT_RichValueFallback/xlrd:fb")]
pub struct RichValueFallback {
  /// t
  #[sdk(attr(office2019, qname = ":t"))]
  pub t: Option<RichValueFallbackType>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Value Class.
pub type Value = crate::simple_type::StringValue;
/// Defines the RichValueStructure Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd:CT_RichValueStructure/xlrd:s")]
pub struct RichValueStructure {
  /// t
  #[sdk(attr(office2019, qname = ":t"))]
  pub t: crate::simple_type::StringValue,
  /// Defines the Key Class.
  #[sdk(child(office2019, qname = "xlrd:CT_Key/xlrd:k"))]
  pub xlrd_k: Vec<Key>,
}
/// Defines the Key Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2019, qname = "xlrd:CT_Key/xlrd:k")]
pub struct Key {
  /// n
  #[sdk(attr(office2019, qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// t
  #[sdk(attr(office2019, qname = ":t"))]
  pub t: Option<RichValueValueType>,
}
