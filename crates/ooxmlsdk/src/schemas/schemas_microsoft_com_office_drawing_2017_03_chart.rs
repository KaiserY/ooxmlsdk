//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the DataDisplayOptions16 Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16r3:dataDisplayOptions16")]
pub struct DataDisplayOptions16 {
  /// Defines the BooleanFalse Class.
  #[sdk(child(qname = "c16r3:dispNaAsBlank"))]
  pub boolean_false: Option<BooleanFalse>,
}
/// Defines the BooleanFalse Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16r3:dispNaAsBlank")]
pub struct BooleanFalse {
  /// val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
