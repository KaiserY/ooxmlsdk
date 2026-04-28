//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Xsdboolean Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxpim:implicitMeasureSupport.
pub type Xsdboolean = crate::simple_type::BooleanValue;
/// Defines the Ignorable Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxpim:ignorableAfterVersion.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxpim:CT_Ignorable/xxpim:ignorableAfterVersion")]
pub struct Ignorable {
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
  /// version
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :version
  #[sdk(attr(qname = ":version"))]
  pub version: crate::simple_type::ByteValue,
}
/// Defines the DataFieldFutureData Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is xxpim:dataFieldFutureData.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxpim:CT_DataFieldFutureData/xxpim:dataFieldFutureData")]
pub struct DataFieldFutureData {
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
  /// version
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :version
  #[sdk(attr(qname = ":version"))]
  pub version: crate::simple_type::ByteValue,
  /// sourceField
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :sourceField
  #[sdk(attr(qname = ":sourceField"))]
  pub source_field: crate::simple_type::UInt32Value,
}
