//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the SymEx Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is w16se:symEx.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "w16se:CT_SymEx/w16se:symEx")]
pub struct SymEx {
  /// font
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: w16se:font
  #[sdk(attr(qname = "w16se:font"))]
  pub w16se_font: Option<crate::simple_type::StringValue>,
  /// char
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: w16se:char
  #[sdk(attr(qname = "w16se:char"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16se_char: Option<crate::simple_type::HexBinaryValue>,
}
