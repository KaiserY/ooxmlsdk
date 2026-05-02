//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Application Specific File Properties.
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
pub type Template = crate::simple_type::StringValue;
/// Name of Manager.
pub type Manager = crate::simple_type::StringValue;
/// Name of Company.
pub type Company = crate::simple_type::StringValue;
/// Intended Format of Presentation.
pub type PresentationFormat = crate::simple_type::StringValue;
/// Relative Hyperlink Base.
pub type HyperlinkBase = crate::simple_type::StringValue;
/// Application Name.
pub type Application = crate::simple_type::StringValue;
/// Application Version.
pub type ApplicationVersion = crate::simple_type::StringValue;
/// Total Number of Pages.
pub type Pages = crate::simple_type::Int32Value;
/// Word Count.
pub type Words = crate::simple_type::Int32Value;
/// Total Number of Characters.
pub type Characters = crate::simple_type::Int32Value;
/// Number of Lines.
pub type Lines = crate::simple_type::Int32Value;
/// Total Number of Paragraphs.
pub type Paragraphs = crate::simple_type::Int32Value;
/// Slides Metadata Element.
pub type Slides = crate::simple_type::Int32Value;
/// Number of Slides Containing Notes.
pub type Notes = crate::simple_type::Int32Value;
/// Total Edit Time Metadata Element.
pub type TotalTime = crate::simple_type::Int32Value;
/// Number of Hidden Slides.
pub type HiddenSlides = crate::simple_type::Int32Value;
/// Total Number of Multimedia Clips.
pub type MultimediaClips = crate::simple_type::Int32Value;
/// Number of Characters (With Spaces).
pub type CharactersWithSpaces = crate::simple_type::Int32Value;
/// Document Security.
pub type DocumentSecurity = crate::simple_type::Int32Value;
/// Thumbnail Display Mode.
pub type ScaleCrop = crate::simple_type::BooleanValue;
/// Links Up-to-Date.
pub type LinksUpToDate = crate::simple_type::BooleanValue;
/// Shared Document.
pub type SharedDocument = crate::simple_type::BooleanValue;
/// Hyperlinks Changed.
pub type HyperlinksChanged = crate::simple_type::BooleanValue;
/// Heading Pairs.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_VectorVariant/ap:HeadingPairs")]
pub struct HeadingPairs {
  /// Vector
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  pub vt_vector: std::boxed::Box<crate::schemas::vt::VtVector>,
}
/// Hyperlink List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_VectorVariant/ap:HLinks")]
pub struct HyperlinkList {
  /// Vector
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  pub vt_vector: std::boxed::Box<crate::schemas::vt::VtVector>,
}
/// Part Titles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_VectorLpstr/ap:TitlesOfParts")]
pub struct TitlesOfParts {
  /// Vector
  #[sdk(child(qname = "vt:CT_Vector/vt:vector"))]
  pub vt_vector: std::boxed::Box<crate::schemas::vt::VtVector>,
}
/// Digital Signature.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "ap:CT_DigSigBlob/ap:DigSig")]
pub struct DigitalSignature {
  /// Binary Blob
  #[sdk(text_child(qname = "xsd:base64Binary/vt:blob"))]
  pub vt_blob: crate::simple_type::Base64BinaryValue,
}
