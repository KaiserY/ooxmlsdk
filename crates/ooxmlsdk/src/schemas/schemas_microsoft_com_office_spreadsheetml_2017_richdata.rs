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
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:rvb.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd:CT_RichValueBlock/xlrd:rvb")]
pub struct RichValueBlock {
  /// i
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :i
  #[sdk(attr(qname = ":i"))]
  pub i: crate::simple_type::UInt32Value,
}
/// Defines the RichValueData Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:rvData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd:CT_RichValueData/xlrd:rvData")]
pub struct RichValueData {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// count
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "xlrd:CT_RichValue/xlrd:rv"))]
  pub xlrd_rv: Vec<RichValue>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd:extLst"))]
  pub xlrd_ext_lst: Option<ExtensionList>,
}
/// Defines the RichValueStructures Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:rvStructures.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd:CT_RichValueStructures/xlrd:rvStructures")]
pub struct RichValueStructures {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// count
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :count
  #[sdk(attr(qname = ":count"))]
  pub count: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "xlrd:CT_RichValueStructure/xlrd:s"))]
  pub xlrd_s: Vec<RichValueStructure>,
  /// _
  #[sdk(child(qname = "x:CT_ExtensionList/xlrd:extLst"))]
  pub xlrd_ext_lst: Option<ExtensionList>,
}
/// Defines the RichValue Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:rv.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd:CT_RichValue/xlrd:rv")]
pub struct RichValue {
  /// s
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :s
  #[sdk(attr(qname = ":s"))]
  pub s: crate::simple_type::UInt32Value,
  /// _
  #[sdk(child(qname = "xlrd:CT_RichValueFallback/xlrd:fb"))]
  pub rich_value_fallback: Option<RichValueFallback>,
  /// _
  #[sdk(text_child(qname = "xlrd:CT_Value/xlrd:v"))]
  pub xlrd_v: Vec<crate::simple_type::StringValue>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "x:CT_ExtensionList/xlrd:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "x:CT_Extension/x:ext"))]
  pub extension: Vec<crate::schemas::schemas_openxmlformats_org_spreadsheetml_2006_main::Extension>,
}
/// Defines the RichValueFallback Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:fb.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd:CT_RichValueFallback/xlrd:fb")]
pub struct RichValueFallback {
  /// t
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<RichValueFallbackType>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// Defines the Value Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:v.
pub type Value = crate::simple_type::StringValue;
/// Defines the RichValueStructure Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:s.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd:CT_RichValueStructure/xlrd:s")]
pub struct RichValueStructure {
  /// t
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: crate::simple_type::StringValue,
  /// _
  #[sdk(child(qname = "xlrd:CT_Key/xlrd:k"))]
  pub xlrd_k: Vec<Key>,
}
/// Defines the Key Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlrd:k.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlrd:CT_Key/xlrd:k")]
pub struct Key {
  /// n
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :n
  #[sdk(attr(qname = ":n"))]
  pub n: crate::simple_type::StringValue,
  /// t
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: :t
  #[sdk(attr(qname = ":t"))]
  pub t: Option<RichValueValueType>,
}
