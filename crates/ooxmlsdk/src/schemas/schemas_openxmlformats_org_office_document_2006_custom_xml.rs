//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Custom XML Data Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:datastoreItem")]
pub struct DataStoreItem {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  /// Custom XML Data ID
  #[sdk(attr(qname = "ds:itemID"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub item_id: crate::simple_type::StringValue,
  /// Set of Associated XML Schemas
  #[sdk(child(qname = "ds:schemaRefs"))]
  pub schema_references: Option<SchemaReferences>,
}
/// Associated XML Schema.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:schemaRef")]
pub struct SchemaReference {
  /// Target Namespace of Associated XML Schema
  #[sdk(attr(qname = "ds:uri"))]
  pub uri: crate::simple_type::StringValue,
}
/// Set of Associated XML Schemas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:schemaRefs")]
pub struct SchemaReferences {
  /// Associated XML Schema.
  #[sdk(child(qname = "ds:schemaRef"))]
  pub schema_reference: Vec<SchemaReference>,
}
