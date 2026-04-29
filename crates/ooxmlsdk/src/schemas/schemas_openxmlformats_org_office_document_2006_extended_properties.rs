//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Application Specific File Properties.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_Properties/ap:Properties")]
pub struct Properties {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  pub xml_header: crate::common::XmlHeaderType,
  pub xml_other_attrs: Vec<(String, String)>,
  /// Name of Document Template
  #[sdk(text_child(qname = "xsd:string/ap:Template"))]
  pub template: Option<crate::simple_type::StringValue>,
  /// Name of Manager
  #[sdk(text_child(qname = "xsd:string/ap:Manager"))]
  pub manager: Option<crate::simple_type::StringValue>,
  /// Name of Company
  #[sdk(text_child(qname = "xsd:string/ap:Company"))]
  pub company: Option<crate::simple_type::StringValue>,
  /// Total Number of Pages
  #[sdk(text_child(qname = "xsd:int/ap:Pages"))]
  pub pages: Option<crate::simple_type::Int32Value>,
  /// Word Count
  #[sdk(text_child(qname = "xsd:int/ap:Words"))]
  pub words: Option<crate::simple_type::Int32Value>,
  /// Total Number of Characters
  #[sdk(text_child(qname = "xsd:int/ap:Characters"))]
  pub characters: Option<crate::simple_type::Int32Value>,
  /// Intended Format of Presentation
  #[sdk(text_child(qname = "xsd:string/ap:PresentationFormat"))]
  pub presentation_format: Option<crate::simple_type::StringValue>,
  /// Number of Lines
  #[sdk(text_child(qname = "xsd:int/ap:Lines"))]
  pub lines: Option<crate::simple_type::Int32Value>,
  /// Total Number of Paragraphs
  #[sdk(text_child(qname = "xsd:int/ap:Paragraphs"))]
  pub paragraphs: Option<crate::simple_type::Int32Value>,
  /// Slides Metadata Element
  #[sdk(text_child(qname = "xsd:int/ap:Slides"))]
  pub slides: Option<crate::simple_type::Int32Value>,
  /// Number of Slides Containing Notes
  #[sdk(text_child(qname = "xsd:int/ap:Notes"))]
  pub notes: Option<crate::simple_type::Int32Value>,
  /// Total Edit Time Metadata Element
  #[sdk(text_child(qname = "xsd:int/ap:TotalTime"))]
  pub total_time: Option<crate::simple_type::Int32Value>,
  /// Number of Hidden Slides
  #[sdk(text_child(qname = "xsd:int/ap:HiddenSlides"))]
  pub hidden_slides: Option<crate::simple_type::Int32Value>,
  /// Total Number of Multimedia Clips
  #[sdk(text_child(qname = "xsd:int/ap:MMClips"))]
  pub multimedia_clips: Option<crate::simple_type::Int32Value>,
  /// Thumbnail Display Mode
  #[sdk(text_child(qname = "xsd:boolean/ap:ScaleCrop"))]
  pub scale_crop: Option<crate::simple_type::BooleanValue>,
  /// Heading Pairs
  #[sdk(child(qname = "ap:CT_VectorVariant/ap:HeadingPairs"))]
  pub heading_pairs: Option<std::boxed::Box<HeadingPairs>>,
  /// Part Titles
  #[sdk(child(qname = "ap:CT_VectorLpstr/ap:TitlesOfParts"))]
  pub titles_of_parts: Option<std::boxed::Box<TitlesOfParts>>,
  /// Links Up-to-Date
  #[sdk(text_child(qname = "xsd:boolean/ap:LinksUpToDate"))]
  pub links_up_to_date: Option<crate::simple_type::BooleanValue>,
  /// Number of Characters (With Spaces)
  #[sdk(text_child(qname = "xsd:int/ap:CharactersWithSpaces"))]
  pub characters_with_spaces: Option<crate::simple_type::Int32Value>,
  /// Shared Document
  #[sdk(text_child(qname = "xsd:boolean/ap:SharedDoc"))]
  pub shared_document: Option<crate::simple_type::BooleanValue>,
  /// Relative Hyperlink Base
  #[sdk(text_child(qname = "xsd:string/ap:HyperlinkBase"))]
  pub hyperlink_base: Option<crate::simple_type::StringValue>,
  /// Hyperlink List
  #[sdk(child(qname = "ap:CT_VectorVariant/ap:HLinks"))]
  pub hyperlink_list: Option<std::boxed::Box<HyperlinkList>>,
  /// Hyperlinks Changed
  #[sdk(text_child(qname = "xsd:boolean/ap:HyperlinksChanged"))]
  pub hyperlinks_changed: Option<crate::simple_type::BooleanValue>,
  /// Digital Signature
  #[sdk(child(qname = "ap:CT_DigSigBlob/ap:DigSig"))]
  pub digital_signature: Option<DigitalSignature>,
  /// Application Name
  #[sdk(text_child(qname = "xsd:string/ap:Application"))]
  pub application: Option<crate::simple_type::StringValue>,
  /// Application Version
  #[sdk(text_child(qname = "xsd:string/ap:AppVersion"))]
  pub application_version: Option<crate::simple_type::StringValue>,
  /// Document Security
  #[sdk(text_child(qname = "xsd:int/ap:DocSecurity"))]
  pub document_security: Option<crate::simple_type::Int32Value>,
}
/// Name of Document Template.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Template.
pub type Template = crate::simple_type::StringValue;
/// Name of Manager.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Manager.
pub type Manager = crate::simple_type::StringValue;
/// Name of Company.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Company.
pub type Company = crate::simple_type::StringValue;
/// Intended Format of Presentation.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:PresentationFormat.
pub type PresentationFormat = crate::simple_type::StringValue;
/// Relative Hyperlink Base.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:HyperlinkBase.
pub type HyperlinkBase = crate::simple_type::StringValue;
/// Application Name.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Application.
pub type Application = crate::simple_type::StringValue;
/// Application Version.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:AppVersion.
pub type ApplicationVersion = crate::simple_type::StringValue;
/// Total Number of Pages.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Pages.
pub type Pages = crate::simple_type::Int32Value;
/// Word Count.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Words.
pub type Words = crate::simple_type::Int32Value;
/// Total Number of Characters.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Characters.
pub type Characters = crate::simple_type::Int32Value;
/// Number of Lines.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Lines.
pub type Lines = crate::simple_type::Int32Value;
/// Total Number of Paragraphs.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Paragraphs.
pub type Paragraphs = crate::simple_type::Int32Value;
/// Slides Metadata Element.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Slides.
pub type Slides = crate::simple_type::Int32Value;
/// Number of Slides Containing Notes.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:Notes.
pub type Notes = crate::simple_type::Int32Value;
/// Total Edit Time Metadata Element.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:TotalTime.
pub type TotalTime = crate::simple_type::Int32Value;
/// Number of Hidden Slides.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:HiddenSlides.
pub type HiddenSlides = crate::simple_type::Int32Value;
/// Total Number of Multimedia Clips.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:MMClips.
pub type MultimediaClips = crate::simple_type::Int32Value;
/// Number of Characters (With Spaces).
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:CharactersWithSpaces.
pub type CharactersWithSpaces = crate::simple_type::Int32Value;
/// Document Security.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:DocSecurity.
pub type DocumentSecurity = crate::simple_type::Int32Value;
/// Thumbnail Display Mode.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:ScaleCrop.
pub type ScaleCrop = crate::simple_type::BooleanValue;
/// Links Up-to-Date.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:LinksUpToDate.
pub type LinksUpToDate = crate::simple_type::BooleanValue;
/// Shared Document.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:SharedDoc.
pub type SharedDocument = crate::simple_type::BooleanValue;
/// Hyperlinks Changed.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:HyperlinksChanged.
pub type HyperlinksChanged = crate::simple_type::BooleanValue;
/// Heading Pairs.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:HeadingPairs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_VectorVariant/ap:HeadingPairs")]
pub struct HeadingPairs {
  /// Vector
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  pub vt_vector: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
  >,
}
/// Hyperlink List.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:HLinks.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_VectorVariant/ap:HLinks")]
pub struct HyperlinkList {
  /// Vector
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  pub vt_vector: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
  >,
}
/// Defines the VectorVariantType Class.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_VectorVariant/")]
pub struct VectorVariantType {
  /// Vector
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  pub vt_vector: Vec<
    crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
  >,
}
/// Part Titles.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:TitlesOfParts.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_VectorLpstr/ap:TitlesOfParts")]
pub struct TitlesOfParts {
  /// Vector
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  pub vt_vector: std::boxed::Box<
    crate::schemas::schemas_openxmlformats_org_office_document_2006_doc_props_v_types::VtVector,
  >,
}
/// Digital Signature.
///
/// Available in Office2007 and above.
///
/// When the object is serialized out as xml, it's qualified name is ap:DigSig.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_DigSigBlob/ap:DigSig")]
pub struct DigitalSignature {
  /// Binary Blob
  #[sdk(text_child(qname = "xsd:base64Binary/vt:blob"))]
  pub vt_blob: crate::simple_type::Base64BinaryValue,
}
