//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Content Types.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is Types.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "CT_Types/Types")]
pub struct Types {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  #[sdk(choice(qname = "CT_Default/Default", qname = "CT_Override/Override"))]
  pub xml_children: Vec<TypesChoice>,
}
/// Default content type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is Default.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "CT_Default/Default")]
pub struct Default {
  /// extension
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: Extension
  #[sdk(attr(qname = "Extension"))]
  pub extension: crate::simple_type::StringValue,
  /// content_type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ContentType
  #[sdk(attr(qname = "ContentType"))]
  pub content_type: crate::simple_type::StringValue,
}
/// Override content type.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is Override.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "CT_Override/Override")]
pub struct Override {
  /// content_type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ContentType
  #[sdk(attr(qname = "ContentType"))]
  pub content_type: crate::simple_type::StringValue,
  /// part_name
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: PartName
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
