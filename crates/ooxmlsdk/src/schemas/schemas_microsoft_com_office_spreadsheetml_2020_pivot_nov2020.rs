//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the Xsdboolean Class.
pub type Xsdboolean = crate::simple_type::BooleanValue;
/// Defines the Ignorable Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxpim:ignorableAfterVersion")]
pub struct Ignorable {
  /// version
  #[sdk(attr(qname = ":version"))]
  pub version: crate::simple_type::ByteValue,
}
/// Defines the DataFieldFutureData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xxpim:dataFieldFutureData")]
pub struct DataFieldFutureData {
  /// version
  #[sdk(attr(qname = ":version"))]
  pub version: crate::simple_type::ByteValue,
  /// sourceField
  #[sdk(attr(qname = ":sourceField"))]
  pub source_field: crate::simple_type::UInt32Value,
}
