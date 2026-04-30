//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the SymEx Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "w16se:CT_SymEx/w16se:symEx")]
pub struct SymEx {
  /// font
  #[sdk(attr(office2016, qname = "w16se:font"))]
  pub w16se_font: Option<crate::simple_type::StringValue>,
  /// char
  #[sdk(attr(office2016, qname = "w16se:char"))]
  #[sdk(string_length(source = 1u32, union = 0u64, min = 4u32, max = 4u32))]
  pub w16se_char: Option<crate::simple_type::HexBinaryValue>,
}
