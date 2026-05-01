//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum RelationValues {
  #[sdk(rename = "ge")]
  #[default]
  GreaterThanOrEqualTo,
  #[sdk(rename = "le")]
  LessThanOrEqualTo,
  #[sdk(rename = "gt")]
  GreaterThan,
  #[sdk(rename = "lt")]
  LessThan,
  #[sdk(rename = "eq")]
  EqualTo,
}
/// Set of Additional Characteristics.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ac:CT_AdditionalCharacteristics/ac:additionalCharacteristics")]
pub struct AdditionalCharacteristicsInfo {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Single Characteristic.
  #[sdk(child(qname = "ac:CT_Characteristic/ac:characteristic"))]
  pub ac_characteristic: Vec<Characteristic>,
}
/// Single Characteristic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ac:CT_Characteristic/ac:characteristic")]
pub struct Characteristic {
  /// Name of Characteristic
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Relationship of Value to Name
  #[sdk(attr(qname = ":relation"))]
  pub relation: RelationValues,
  /// Characteristic Value
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
  /// Characteristic Grammar
  #[sdk(attr(qname = ":vocabulary"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub vocabulary: Option<crate::simple_type::StringValue>,
}
