//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ClassificationLabelList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "clbl:CT_ClassificationLabelList/clbl:labelList")]
pub struct ClassificationLabelList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Defines the ClassificationLabel Class.
  #[sdk(child(office2021, qname = "clbl:CT_ClassificationLabel/clbl:label"))]
  pub clbl_label: Vec<ClassificationLabel>,
  /// Defines the ClassificationExtensionList Class.
  #[sdk(child(office2021, qname = "clbl:CT_ClassificationExtensionList/clbl:extLst"))]
  pub clbl_ext_lst: Option<ClassificationExtensionList>,
}
/// Defines the ClassificationExtension Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "clbl:CT_ClassificationExtension/clbl:ext")]
pub struct ClassificationExtension {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// uri
  #[sdk(attr(office2021, qname = ":uri"))]
  #[sdk(string_format(kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ClassificationLabel Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "clbl:CT_ClassificationLabel/clbl:label")]
pub struct ClassificationLabel {
  /// id
  #[sdk(attr(office2021, qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// enabled
  #[sdk(attr(office2021, qname = ":enabled"))]
  pub enabled: crate::simple_type::BooleanValue,
  /// setDate
  #[sdk(attr(office2021, qname = ":setDate"))]
  pub set_date: Option<crate::simple_type::StringValue>,
  /// method
  #[sdk(attr(office2021, qname = ":method"))]
  pub method: crate::simple_type::StringValue,
  /// name
  #[sdk(attr(office2021, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// siteId
  #[sdk(attr(office2021, qname = ":siteId"))]
  #[sdk(pattern(
    regex = "\\{[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}\\}"
  ))]
  #[sdk(string_format(kind = "token"))]
  pub site_id: crate::simple_type::StringValue,
  /// actionId
  #[sdk(attr(office2021, qname = ":actionId"))]
  pub action_id: Option<crate::simple_type::StringValue>,
  /// contentBits
  #[sdk(attr(office2021, qname = ":contentBits"))]
  pub content_bits: Option<crate::simple_type::UInt32Value>,
  /// removed
  #[sdk(attr(office2021, qname = ":removed"))]
  pub removed: crate::simple_type::BooleanValue,
}
/// Defines the ClassificationExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2021, qname = "clbl:CT_ClassificationExtensionList/clbl:extLst")]
pub struct ClassificationExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Defines the ClassificationExtension Class.
  #[sdk(child(office2021, qname = "clbl:CT_ClassificationExtension/clbl:ext"))]
  pub clbl_ext: Vec<ClassificationExtension>,
}
