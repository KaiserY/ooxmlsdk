//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Embedded Custom XML Schema Supplementary Data.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is sl:schemaLibrary.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "sl:CT_SchemaLibrary/sl:schemaLibrary")]
pub struct SchemaLibrary {
  /// _
  #[sdk(child(qname = "sl:CT_Schema/sl:schema"))]
  pub sl_schema: Vec<Schema>,
}
/// Custom XML Schema Reference.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is sl:schema.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "sl:CT_Schema/sl:schema")]
pub struct Schema {
  /// Custom XML Schema Namespace
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: sl:uri
  #[sdk(attr(qname = "sl:uri"))]
  pub uri: Option<crate::simple_type::StringValue>,
  /// Resource File Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: sl:manifestLocation
  #[sdk(attr(qname = "sl:manifestLocation"))]
  pub manifest_location: Option<crate::simple_type::StringValue>,
  /// Custom XML Schema Location
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: sl:schemaLocation
  #[sdk(attr(qname = "sl:schemaLocation"))]
  pub schema_location: Option<crate::simple_type::StringValue>,
}
