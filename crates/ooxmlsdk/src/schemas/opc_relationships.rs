//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TargetMode {
  #[sdk(rename = "External")]
  #[default]
  External,
  #[sdk(rename = "Internal")]
  Internal,
}
/// Relationships.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(xml_header, qname = "Relationships")]
pub struct Relationships {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Relationship
  #[sdk(child(qname = "Relationship"))]
  pub relationship: Vec<Relationship>,
}
/// Relationship.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "Relationship")]
pub struct Relationship {
  /// TARGET MODE
  #[sdk(attr(qname = "TargetMode"))]
  pub target_mode: Option<TargetMode>,
  /// TARGET
  #[sdk(attr(qname = "Target"))]
  pub target: crate::simple_type::StringValue,
  /// TYPE
  #[sdk(attr(qname = "Type"))]
  pub r#type: crate::simple_type::StringValue,
  /// ID
  #[sdk(attr(qname = "Id"))]
  pub id: crate::simple_type::StringValue,
}
