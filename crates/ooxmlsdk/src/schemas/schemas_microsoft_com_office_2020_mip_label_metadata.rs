//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the ClassificationLabelList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is clbl:labelList.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "clbl:CT_ClassificationLabelList/clbl:labelList")]
pub struct ClassificationLabelList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub mc_ignorable: Option<String>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "clbl:CT_ClassificationLabel/clbl:label"))]
  pub clbl_label: Vec<ClassificationLabel>,
  /// _
  #[sdk(child(qname = "clbl:CT_ClassificationExtensionList/clbl:extLst"))]
  pub clbl_ext_lst: Option<ClassificationExtensionList>,
}
/// Defines the ClassificationExtension Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is clbl:ext.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "clbl:CT_ClassificationExtension/clbl:ext")]
pub struct ClassificationExtension {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// uri
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :uri
  #[sdk(attr(qname = ":uri"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub uri: crate::simple_type::StringValue,
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the ClassificationLabel Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is clbl:label.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "clbl:CT_ClassificationLabel/clbl:label")]
pub struct ClassificationLabel {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// id
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::StringValue,
  /// enabled
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :enabled
  #[sdk(attr(qname = ":enabled"))]
  pub enabled: crate::simple_type::BooleanValue,
  /// setDate
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :setDate
  #[sdk(attr(qname = ":setDate"))]
  pub set_date: Option<crate::simple_type::StringValue>,
  /// method
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :method
  #[sdk(attr(qname = ":method"))]
  pub method: crate::simple_type::StringValue,
  /// name
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// siteId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :siteId
  #[sdk(attr(qname = ":siteId"))]
  #[sdk(pattern(
    source = 1u32,
    regex = "\\{[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}\\}"
  ))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub site_id: crate::simple_type::StringValue,
  /// actionId
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :actionId
  #[sdk(attr(qname = ":actionId"))]
  pub action_id: Option<crate::simple_type::StringValue>,
  /// contentBits
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :contentBits
  #[sdk(attr(qname = ":contentBits"))]
  pub content_bits: Option<crate::simple_type::UInt32Value>,
  /// removed
  ///
  /// Available in Office2021 and above.
  ///
  /// Represents the following attribute in the schema: :removed
  #[sdk(attr(qname = ":removed"))]
  pub removed: crate::simple_type::BooleanValue,
}
/// Defines the ClassificationExtensionList Class.
///
/// Available in Office2021 and above.
///
/// When the object is serialized out as xml, it's qualified name is clbl:extLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "clbl:CT_ClassificationExtensionList/clbl:extLst")]
pub struct ClassificationExtensionList {
  /// Markup compatibility attribute mc:Ignorable.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:Ignorable
  #[sdk(attr(qname = "mc:Ignorable"))]
  pub mc_ignorable: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:MustUnderstand.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:MustUnderstand
  #[sdk(attr(qname = "mc:MustUnderstand"))]
  pub mc_must_understand: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:ProcessContent.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:ProcessContent
  #[sdk(attr(qname = "mc:ProcessContent"))]
  pub mc_process_content: Option<crate::simple_type::StringValue>,
  /// Markup compatibility attribute mc:PreserveAttributes.
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: mc:PreserveAttributes
  #[sdk(attr(qname = "mc:PreserveAttributes"))]
  pub mc_preserve_attributes: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(qname = "clbl:CT_ClassificationExtension/clbl:ext"))]
  pub clbl_ext: Vec<ClassificationExtension>,
}
