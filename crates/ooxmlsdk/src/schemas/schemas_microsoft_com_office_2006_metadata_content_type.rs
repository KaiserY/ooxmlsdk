//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ContentTypeSchema Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ct:contentTypeSchema.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ct:CT_ContentTypeSchema/ct:contentTypeSchema")]
pub struct ContentTypeSchema {
  /// _
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ct:_
  #[sdk(attr(qname = "ct:_"))]
  pub under_score: Option<crate::simple_type::StringValue>,
  /// _
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ma:_
  #[sdk(attr(qname = "ma:_"))]
  pub reserved_attribute_string: Option<crate::simple_type::StringValue>,
  /// contentTypeName
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ma:contentTypeName
  #[sdk(attr(qname = "ma:contentTypeName"))]
  pub content_type_name: Option<crate::simple_type::StringValue>,
  /// contentTypeID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ma:contentTypeID
  #[sdk(attr(qname = "ma:contentTypeID"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "0x([0-9A-Fa-f][1-9A-Fa-f]|[1-9A-Fa-f][0-9A-Fa-f]|00[0-9A-Fa-f]{32})*"
  ))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 1026u32))]
  pub content_type_id: Option<crate::simple_type::StringValue>,
  /// contentTypeVersion
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ma:contentTypeVersion
  #[sdk(attr(qname = "ma:contentTypeVersion"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub content_type_version: Option<crate::simple_type::Int32Value>,
  /// contentTypeDescription
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ma:contentTypeDescription
  #[sdk(attr(qname = "ma:contentTypeDescription"))]
  pub content_type_description: Option<crate::simple_type::StringValue>,
  /// contentTypeScope
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ma:contentTypeScope
  #[sdk(attr(qname = "ma:contentTypeScope"))]
  pub content_type_scope: Option<crate::simple_type::StringValue>,
  /// versionID
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: ma:versionID
  #[sdk(attr(qname = "ma:versionID"))]
  pub version_id: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
