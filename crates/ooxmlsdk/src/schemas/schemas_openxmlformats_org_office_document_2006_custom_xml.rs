//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Custom XML Data Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:CT_DatastoreItem/ds:datastoreItem")]
pub struct DataStoreItem {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Custom XML Data ID
  #[sdk(attr(qname = "ds:itemID"))]
  #[sdk(pattern(regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"))]
  #[sdk(string_format(kind = "token"))]
  pub item_id: crate::simple_type::StringValue,
  /// Set of Associated XML Schemas
  #[sdk(child(qname = "ds:CT_DatastoreSchemaRefs/ds:schemaRefs"))]
  pub schema_references: Option<SchemaReferences>,
}
/// Associated XML Schema.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:CT_DatastoreSchemaRef/ds:schemaRef")]
pub struct SchemaReference {
  /// Target Namespace of Associated XML Schema
  #[sdk(attr(qname = "ds:uri"))]
  pub uri: crate::simple_type::StringValue,
}
/// Set of Associated XML Schemas.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:CT_DatastoreSchemaRefs/ds:schemaRefs")]
pub struct SchemaReferences {
  /// Associated XML Schema.
  #[sdk(child(qname = "ds:CT_DatastoreSchemaRef/ds:schemaRef"))]
  pub ds_schema_ref: Vec<SchemaReference>,
}
