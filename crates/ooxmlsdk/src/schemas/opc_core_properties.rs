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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cp:coreProperties")]
pub struct CoreProperties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_children: Vec<(usize, std::boxed::Box<[u8]>)>,
  /// cp:category
  #[sdk(text_child(qname = "cp:category"))]
  pub category: Option<Category>,
  /// cp:contentStatus
  #[sdk(text_child(qname = "cp:contentStatus"))]
  pub content_status: Option<ContentStatus>,
  /// dcterms:created
  #[sdk(child(qname = "dcterms:created"))]
  pub created: Option<Created>,
  /// dc:creator
  #[sdk(text_child(qname = "dc:creator"))]
  pub creator: Option<Creator>,
  /// dc:description
  #[sdk(text_child(qname = "dc:description"))]
  pub description: Option<Description>,
  /// dc:identifier
  #[sdk(text_child(qname = "dc:identifier"))]
  pub identifier: Option<Identifier>,
  /// cp:keywords
  #[sdk(child(qname = "cp:keywords"))]
  pub keywords: Option<Keywords>,
  /// dc:language
  #[sdk(text_child(qname = "dc:language"))]
  pub language: Option<Language>,
  /// cp:lastModifiedBy
  #[sdk(text_child(qname = "cp:lastModifiedBy"))]
  pub last_modified_by: Option<LastModifiedBy>,
  /// cp:lastPrinted
  #[sdk(text_child(qname = "cp:lastPrinted"))]
  pub last_printed: Option<LastPrinted>,
  /// dcterms:modified
  #[sdk(child(qname = "dcterms:modified"))]
  pub modified: Option<Modified>,
  /// cp:revision
  #[sdk(text_child(qname = "cp:revision"))]
  pub revision: Option<Revision>,
  /// dc:subject
  #[sdk(text_child(qname = "dc:subject"))]
  pub subject: Option<Subject>,
  /// dc:title
  #[sdk(text_child(qname = "dc:title"))]
  pub title: Option<Title>,
  /// cp:version
  #[sdk(text_child(qname = "cp:version"))]
  pub version: Option<Version>,
}
/// cp:category
pub type Category = crate::simple_type::StringValue;
/// cp:contentStatus
pub type ContentStatus = crate::simple_type::StringValue;
/// dcterms:created
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dcterms:created")]
pub struct Created {
  /// type
  #[sdk(attr(qname = "xsi:type"))]
  pub xsi_type: Option<XsiTypeValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// dc:creator
pub type Creator = crate::simple_type::StringValue;
/// dc:description
pub type Description = crate::simple_type::StringValue;
/// dc:identifier
pub type Identifier = crate::simple_type::StringValue;
/// dc:language
pub type Language = crate::simple_type::StringValue;
/// cp:lastModifiedBy
pub type LastModifiedBy = crate::simple_type::StringValue;
/// cp:lastPrinted
pub type LastPrinted = crate::simple_type::StringValue;
/// dcterms:modified
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "dcterms:modified")]
pub struct Modified {
  /// type
  #[sdk(attr(qname = "xsi:type"))]
  pub xsi_type: Option<XsiTypeValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
/// cp:revision
pub type Revision = crate::simple_type::StringValue;
/// dc:subject
pub type Subject = crate::simple_type::StringValue;
/// dc:title
pub type Title = crate::simple_type::StringValue;
/// cp:version
pub type Version = crate::simple_type::StringValue;
/// Keywords.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cp:keywords")]
pub struct Keywords {
  /// lang
  #[sdk(attr(qname = "xml:lang"))]
  pub lang: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
  /// cp:value
  #[sdk(child(qname = "cp:value"))]
  pub value: Vec<Keyword>,
}
/// Keyword.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "cp:value")]
pub struct Keyword {
  /// lang
  #[sdk(attr(qname = "xml:lang"))]
  pub lang: Option<crate::simple_type::StringValue>,
  #[sdk(text)]
  pub xml_content: Option<crate::simple_type::StringValue>,
}
