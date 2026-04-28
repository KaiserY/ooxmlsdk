//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Custom XML Data Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ds:datastoreItem.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:CT_DatastoreItem/ds:datastoreItem")]
pub struct DataStoreItem {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Custom XML Data ID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ds:itemID
  #[sdk(attr(qname = "ds:itemID"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub item_id: crate::simple_type::StringValue,
  /// Set of Associated XML Schemas
  #[sdk(child(qname = "ds:CT_DatastoreSchemaRefs/ds:schemaRefs"))]
  pub schema_references: Option<SchemaReferences>,
}
/// Associated XML Schema.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ds:schemaRef.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:CT_DatastoreSchemaRef/ds:schemaRef")]
pub struct SchemaReference {
  /// Target Namespace of Associated XML Schema
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ds:uri
  #[sdk(attr(qname = "ds:uri"))]
  pub uri: crate::simple_type::StringValue,
}
/// Set of Associated XML Schemas.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ds:schemaRefs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ds:CT_DatastoreSchemaRefs/ds:schemaRefs")]
pub struct SchemaReferences {
  /// _
  #[sdk(child(qname = "ds:CT_DatastoreSchemaRef/ds:schemaRef"))]
  pub ds_schema_ref: Vec<SchemaReference>,
}
