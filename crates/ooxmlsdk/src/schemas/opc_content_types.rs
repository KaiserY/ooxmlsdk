//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Content Types.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "pct:Types")]
pub struct Types {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  #[sdk(
        choice(
            child(variant = Default, qname = "pct:Default"),
            child(variant = Override, qname = "pct:Override")
        )
    )]
  pub types_choice: Vec<TypesChoice>,
}
/// Default content type.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix, qname = "pct:Default")]
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
#[sdk(no_prefix, qname = "pct:Override")]
pub struct Override {
  /// content_type
  #[sdk(attr(qname = "ContentType"))]
  pub content_type: crate::simple_type::StringValue,
  /// part_name
  #[sdk(attr(qname = "PartName"))]
  pub part_name: crate::simple_type::StringValue,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TypesChoice {
  /// Default content type.
  Default(std::boxed::Box<Default>),
  /// Override content type.
  Override(std::boxed::Box<Override>),
}
