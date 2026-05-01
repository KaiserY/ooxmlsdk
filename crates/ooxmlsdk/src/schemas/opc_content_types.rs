//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Content Types.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "CT_Types/Types")]
pub struct Types {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  #[sdk(choice(qname = "CT_Default/Default", qname = "CT_Override/Override"))]
  pub types_choice: Vec<TypesChoice>,
}
/// Default content type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "CT_Default/Default")]
pub struct Default {
  /// extension
  #[sdk(attr(qname = "Extension"))]
  pub extension: crate::simple_type::StringValue,
  /// content_type
  #[sdk(attr(qname = "ContentType"))]
  pub content_type: crate::simple_type::StringValue,
}
/// Override content type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "CT_Override/Override")]
pub struct Override {
  /// content_type
  #[sdk(attr(qname = "ContentType"))]
  pub content_type: crate::simple_type::StringValue,
  /// part_name
  #[sdk(attr(qname = "PartName"))]
  pub part_name: crate::simple_type::StringValue,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum TypesChoice {
  /// Default content type.
  #[sdk(child(qname = "CT_Default/Default"))]
  Default(std::boxed::Box<Default>),
  /// Override content type.
  #[sdk(child(qname = "CT_Override/Override"))]
  Override(std::boxed::Box<Override>),
}
