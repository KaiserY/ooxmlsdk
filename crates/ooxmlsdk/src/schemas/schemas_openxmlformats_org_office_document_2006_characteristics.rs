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
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ac:additionalCharacteristics.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ac:CT_AdditionalCharacteristics/ac:additionalCharacteristics")]
pub struct AdditionalCharacteristicsInfo {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "ac:CT_Characteristic/ac:characteristic"))]
  pub ac_characteristic: Vec<Characteristic>,
}
/// Single Characteristic.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ac:characteristic.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ac:CT_Characteristic/ac:characteristic")]
pub struct Characteristic {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// Name of Characteristic
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: crate::simple_type::StringValue,
  /// Relationship of Value to Name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :relation
  #[sdk(attr(qname = ":relation"))]
  pub relation: RelationValues,
  /// Characteristic Value
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::StringValue,
  /// Characteristic Grammar
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :vocabulary
  #[sdk(attr(qname = ":vocabulary"))]
  #[sdk(string_format(source = 0u32, kind = "uri"))]
  pub vocabulary: Option<crate::simple_type::StringValue>,
}
