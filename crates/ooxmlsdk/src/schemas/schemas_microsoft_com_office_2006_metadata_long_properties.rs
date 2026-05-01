//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the LongProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "lp:CT_LongProperties/lp:LongProperties")]
pub struct LongProperties {
  /// Defines the LongProperty Class.
  #[sdk(child(qname = "lp:CT_LongProp/lp:LongProp"))]
  pub lp_long_prop: Vec<LongProperty>,
}
/// Defines the LongProperty Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "lp:CT_LongProp/lp:LongProp")]
pub struct LongProperty {
  /// name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
