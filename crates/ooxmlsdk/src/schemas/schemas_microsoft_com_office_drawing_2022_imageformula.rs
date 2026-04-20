//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ImageFormula Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is aif:imageFormula.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "aif:CT_ImageFormula/aif:imageFormula")]
pub struct ImageFormula {
  /// formula
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :formula
  #[sdk(attr(qname = ":formula"))]
  pub formula: Option<crate::simple_type::StringValue>,
}
