//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PivotCacheDynamicArray Class.
///
/// Available in Microsoft365 and above.
///
/// When the object is serialized out as xml, it's qualified name is xlpda:pivotCacheDynamicArray.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "xlpda:CT_PivotCacheDynamicArray/xlpda:pivotCacheDynamicArray")]
pub struct PivotCacheDynamicArray {
  /// ref
  ///
  /// Available in Microsoft365 and above.
  ///
  /// Represents the following attribute in the schema: :ref
  #[sdk(attr(qname = ":ref"))]
  pub r#ref: crate::simple_type::StringValue,
}
