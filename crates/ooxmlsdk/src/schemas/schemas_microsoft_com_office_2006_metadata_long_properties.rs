//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the LongProperties Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is lp:LongProperties.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "lp:CT_LongProperties/lp:LongProperties")]
pub struct LongProperties {
  /// _
  #[sdk(child(qname = "lp:CT_LongProp/lp:LongProp"))]
  pub lp_long_prop: Vec<LongProperty>,
}
/// Defines the LongProperty Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is lp:LongProp.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "lp:CT_LongProp/lp:LongProp")]
pub struct LongProperty {
  /// name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
