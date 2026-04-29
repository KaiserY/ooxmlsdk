//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the DataDisplayOptions16 Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is c16r3:dataDisplayOptions16.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16r3:CT_DataDisplayOptions16/c16r3:dataDisplayOptions16")]
pub struct DataDisplayOptions16 {
  /// _
  #[sdk(child(qname = "c16r3:CT_BooleanFalse/c16r3:dispNaAsBlank"))]
  pub boolean_false: Option<BooleanFalse>,
}
/// Defines the BooleanFalse Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is c16r3:dispNaAsBlank.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "c16r3:CT_BooleanFalse/c16r3:dispNaAsBlank")]
pub struct BooleanFalse {
  /// val
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: c16r3:val
  #[sdk(attr(qname = "c16r3:val"))]
  pub c16r3_val: Option<crate::simple_type::BooleanValue>,
}
