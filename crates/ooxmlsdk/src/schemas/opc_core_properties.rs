//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum XsiTypeValue {
  #[sdk(rename = "dcterms:W3CDTF")]
  #[default]
  DctermsW3cdtf,
}
/// Core File Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:coreProperties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cp:CT_CoreProperties/cp:coreProperties")]
pub struct CoreProperties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  /// cp:category
  #[sdk(text_child(qname = "cp:CT_Category/cp:category"))]
  pub category: Option<crate::simple_type::StringValue>,
  /// cp:contentStatus
  #[sdk(text_child(qname = "cp:CT_ContentStatus/cp:contentStatus"))]
  pub content_status: Option<crate::simple_type::StringValue>,
  /// dcterms:created
  #[sdk(child(qname = "dcterms:CT_Created/dcterms:created"))]
  pub created: Option<Created>,
  /// dc:creator
  #[sdk(text_child(qname = "dc:CT_Creator/dc:creator"))]
  pub creator: Option<crate::simple_type::StringValue>,
  /// dc:description
  #[sdk(text_child(qname = "dc:CT_Description/dc:description"))]
  pub description: Option<crate::simple_type::StringValue>,
  /// dc:identifier
  #[sdk(text_child(qname = "dc:CT_Identifier/dc:identifier"))]
  pub identifier: Option<crate::simple_type::StringValue>,
  /// cp:keywords
  #[sdk(child(qname = "cp:CT_Keywords/cp:keywords"))]
  pub keywords: Option<Keywords>,
  /// dc:language
  #[sdk(text_child(qname = "dc:CT_Language/dc:language"))]
  pub language: Option<crate::simple_type::StringValue>,
  /// cp:lastModifiedBy
  #[sdk(text_child(qname = "cp:CT_LastModifiedBy/cp:lastModifiedBy"))]
  pub last_modified_by: Option<crate::simple_type::StringValue>,
  /// cp:lastPrinted
  #[sdk(text_child(qname = "cp:CT_LastPrinted/cp:lastPrinted"))]
  pub last_printed: Option<crate::simple_type::StringValue>,
  /// dcterms:modified
  #[sdk(child(qname = "dcterms:CT_Modified/dcterms:modified"))]
  pub modified: Option<Modified>,
  /// cp:revision
  #[sdk(text_child(qname = "cp:CT_Revision/cp:revision"))]
  pub revision: Option<crate::simple_type::StringValue>,
  /// dc:subject
  #[sdk(text_child(qname = "dc:CT_Subject/dc:subject"))]
  pub subject: Option<crate::simple_type::StringValue>,
  /// dc:title
  #[sdk(text_child(qname = "dc:CT_Title/dc:title"))]
  pub title: Option<crate::simple_type::StringValue>,
  /// cp:version
  #[sdk(text_child(qname = "cp:CT_Version/cp:version"))]
  pub version: Option<crate::simple_type::StringValue>,
}
/// cp:category
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:category.
pub type Category = crate::simple_type::StringValue;
/// cp:contentStatus
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:contentStatus.
pub type ContentStatus = crate::simple_type::StringValue;
/// dcterms:created
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is dcterms:created.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dcterms:CT_Created/dcterms:created")]
pub struct Created {
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xsi:type
  #[sdk(attr(qname = "xsi:type"))]
  pub xsi_type: Option<XsiTypeValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// dc:creator
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is dc:creator.
pub type Creator = crate::simple_type::StringValue;
/// dc:description
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is dc:description.
pub type Description = crate::simple_type::StringValue;
/// dc:identifier
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is dc:identifier.
pub type Identifier = crate::simple_type::StringValue;
/// dc:language
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is dc:language.
pub type Language = crate::simple_type::StringValue;
/// cp:lastModifiedBy
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:lastModifiedBy.
pub type LastModifiedBy = crate::simple_type::StringValue;
/// cp:lastPrinted
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:lastPrinted.
pub type LastPrinted = crate::simple_type::StringValue;
/// dcterms:modified
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is dcterms:modified.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dcterms:CT_Modified/dcterms:modified")]
pub struct Modified {
  /// type
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xsi:type
  #[sdk(attr(qname = "xsi:type"))]
  pub xsi_type: Option<XsiTypeValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// cp:revision
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:revision.
pub type Revision = crate::simple_type::StringValue;
/// dc:subject
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is dc:subject.
pub type Subject = crate::simple_type::StringValue;
/// dc:title
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is dc:title.
pub type Title = crate::simple_type::StringValue;
/// cp:version
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:version.
pub type Version = crate::simple_type::StringValue;
/// Keywords.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:keywords.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cp:CT_Keywords/cp:keywords")]
pub struct Keywords {
  /// lang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:lang
  #[sdk(attr(qname = "xml:lang"))]
  pub lang: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
  /// cp:value
  #[sdk(child(qname = "cp:CT_Keyword/cp:value"))]
  pub value: Vec<Keyword>,
}
/// Keyword.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is cp:value.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cp:CT_Keyword/cp:value")]
pub struct Keyword {
  /// lang
  ///
  /// Available in Office2007 and above.
  ///
  /// Represents the following attribute in the schema: xml:lang
  #[sdk(attr(qname = "xml:lang"))]
  pub lang: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
