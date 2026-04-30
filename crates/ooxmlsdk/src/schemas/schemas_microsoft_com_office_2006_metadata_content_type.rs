//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ContentTypeSchema Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ct:CT_ContentTypeSchema/ct:contentTypeSchema")]
pub struct ContentTypeSchema {
  /// _
  #[sdk(attr(qname = "ct:_"))]
  pub under_score: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(attr(qname = "ma:_"))]
  pub reserved_attribute_string: Option<crate::simple_type::StringValue>,
  /// contentTypeName
  #[sdk(attr(qname = "ma:contentTypeName"))]
  pub content_type_name: Option<crate::simple_type::StringValue>,
  /// contentTypeID
  #[sdk(attr(qname = "ma:contentTypeID"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "0x([0-9A-Fa-f][1-9A-Fa-f]|[1-9A-Fa-f][0-9A-Fa-f]|00[0-9A-Fa-f]{32})*"
  ))]
  #[sdk(string_length(source = 0u32, min = 2u32, max = 1026u32))]
  pub content_type_id: Option<crate::simple_type::StringValue>,
  /// contentTypeVersion
  #[sdk(attr(qname = "ma:contentTypeVersion"))]
  #[sdk(number_range(source = 0u32, min = "0", min_inclusive = true, max_inclusive = false))]
  pub content_type_version: Option<crate::simple_type::Int32Value>,
  /// contentTypeDescription
  #[sdk(attr(qname = "ma:contentTypeDescription"))]
  pub content_type_description: Option<crate::simple_type::StringValue>,
  /// contentTypeScope
  #[sdk(attr(qname = "ma:contentTypeScope"))]
  pub content_type_scope: Option<crate::simple_type::StringValue>,
  /// versionID
  #[sdk(attr(qname = "ma:versionID"))]
  pub version_id: Option<crate::simple_type::StringValue>,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
