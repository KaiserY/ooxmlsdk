//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Embedded Custom XML Schema Supplementary Data.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "sl:CT_SchemaLibrary/sl:schemaLibrary")]
pub struct SchemaLibrary {
  /// Custom XML Schema Reference.
  #[sdk(child(qname = "sl:CT_Schema/sl:schema"))]
  pub sl_schema: Vec<Schema>,
}
/// Custom XML Schema Reference.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "sl:CT_Schema/sl:schema")]
pub struct Schema {
  /// Custom XML Schema Namespace
  #[sdk(attr(qname = "sl:uri"))]
  pub uri: Option<crate::simple_type::StringValue>,
  /// Resource File Location
  #[sdk(attr(qname = "sl:manifestLocation"))]
  pub manifest_location: Option<crate::simple_type::StringValue>,
  /// Custom XML Schema Location
  #[sdk(attr(qname = "sl:schemaLocation"))]
  pub schema_location: Option<crate::simple_type::StringValue>,
}
