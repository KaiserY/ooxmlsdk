//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the DesignElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is p16:designElem.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p16:CT_DesignElement/p16:designElem")]
pub struct DesignElement {
  /// val
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: Option<crate::simple_type::BooleanValue>,
}
