//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Key Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is pRoam:key.
pub type Key = crate::simple_type::StringValue;
/// Defines the Value Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is pRoam:value.
pub type Value = crate::simple_type::StringValue;
/// Defines the RoamingProperty Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is pRoam:props.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pRoam:CT_RoamingProperty/pRoam:props")]
pub struct RoamingProperty {
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
  /// _
  #[sdk(text_child(qname = "xsd:string/pRoam:key"))]
  pub key: crate::simple_type::StringValue,
  /// _
  #[sdk(text_child(qname = "xsd:string/pRoam:value"))]
  pub value: crate::simple_type::StringValue,
}
