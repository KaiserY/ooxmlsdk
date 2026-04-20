//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkEnum)]
pub enum TargetMode {
  #[sdk(rename = "External")]
  #[default]
  External,
  #[sdk(rename = "Internal")]
  Internal,
}
/// Relationships.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is Relationships.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "CT_Relationships/Relationships")]
pub struct Relationships {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  pub xml_header: crate::common::XmlHeaderType,
  ///Relationship
  #[sdk(child(qname = "CT_Relationship/Relationship"))]
  pub relationship: Vec<Relationship>,
}
/// Relationship.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is Relationship.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "CT_Relationship/Relationship")]
pub struct Relationship {
  /// TARGET MODE
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: TargetMode
  #[sdk(attr(qname = "TargetMode"))]
  pub target_mode: Option<TargetMode>,
  /// TARGET
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: Target
  #[sdk(attr(qname = "Target"))]
  pub target: crate::simple_type::StringValue,
  /// TYPE
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: Type
  #[sdk(attr(qname = "Type"))]
  pub r#type: crate::simple_type::StringValue,
  /// ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: Id
  #[sdk(attr(qname = "Id"))]
  pub id: crate::simple_type::StringValue,
}
