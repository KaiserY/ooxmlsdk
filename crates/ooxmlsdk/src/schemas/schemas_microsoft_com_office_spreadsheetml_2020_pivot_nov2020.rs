//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Xsdboolean Class.
pub type Xsdboolean = crate::simple_type::BooleanValue;
/// Defines the Ignorable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "xxpim:CT_Ignorable/xxpim:ignorableAfterVersion")]
pub struct Ignorable {
  /// version
  #[sdk(attr(office2021, qname = ":version"))]
  pub version: crate::simple_type::ByteValue,
}
/// Defines the DataFieldFutureData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2021,
  qname = "xxpim:CT_DataFieldFutureData/xxpim:dataFieldFutureData"
)]
pub struct DataFieldFutureData {
  /// version
  #[sdk(attr(office2021, qname = ":version"))]
  pub version: crate::simple_type::ByteValue,
  /// sourceField
  #[sdk(attr(office2021, qname = ":sourceField"))]
  pub source_field: crate::simple_type::UInt32Value,
}
