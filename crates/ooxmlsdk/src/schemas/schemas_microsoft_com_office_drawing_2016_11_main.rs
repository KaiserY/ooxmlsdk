//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PictureAttributionSourceURL Class.
///
/// Available in Office2019 and above.
///
/// When the object is serialized out as xml, it's qualified name is a1611:picAttrSrcUrl.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a1611:CT_PictureAttributionSourceURL/a1611:picAttrSrcUrl")]
pub struct PictureAttributionSourceUrl {
  /// id
  ///
  /// Available in Office2019 and above.
  ///
  /// Represents the following attribute in the schema: r:id
  #[sdk(attr(qname = "r:id"))]
  pub r_id: crate::simple_type::StringValue,
}
