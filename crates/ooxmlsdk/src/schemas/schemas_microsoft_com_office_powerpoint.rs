//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Ink Annotation Flag.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is pvml:iscomment.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pvml:CT_Empty/pvml:iscomment")]
pub struct InkAnnotationFlag {}
/// VML Diagram Text.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is pvml:textdata.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pvml:CT_Rel/pvml:textdata")]
pub struct TextData {
  /// Text Reference
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: Option<crate::simple_type::StringValue>,
}
