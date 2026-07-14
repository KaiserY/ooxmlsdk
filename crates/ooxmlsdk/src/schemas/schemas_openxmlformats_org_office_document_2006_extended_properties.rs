//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Application Specific File Properties.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, xml_header, qname = "ap:Properties")]
pub struct Properties {
  pub xmlns: Vec<crate::common::XmlNamespace>,
  /// Name of Document Template
  #[sdk(text_child(qname = "ap:Template"))]
  pub template: Option<Template>,
  /// Name of Manager
  #[sdk(text_child(qname = "ap:Manager"))]
  pub manager: Option<Manager>,
  /// Name of Company
  #[sdk(text_child(qname = "ap:Company"))]
  pub company: Option<Company>,
  /// Total Number of Pages
  #[sdk(text_child(qname = "ap:Pages"))]
  pub pages: Option<Pages>,
  /// Word Count
  #[sdk(text_child(qname = "ap:Words"))]
  pub words: Option<Words>,
  /// Total Number of Characters
  #[sdk(text_child(qname = "ap:Characters"))]
  pub characters: Option<Characters>,
  /// Intended Format of Presentation
  #[sdk(text_child(qname = "ap:PresentationFormat"))]
  pub presentation_format: Option<PresentationFormat>,
  /// Number of Lines
  #[sdk(text_child(qname = "ap:Lines"))]
  pub lines: Option<Lines>,
  /// Total Number of Paragraphs
  #[sdk(text_child(qname = "ap:Paragraphs"))]
  pub paragraphs: Option<Paragraphs>,
  /// Slides Metadata Element
  #[sdk(text_child(qname = "ap:Slides"))]
  pub slides: Option<Slides>,
  /// Number of Slides Containing Notes
  #[sdk(text_child(qname = "ap:Notes"))]
  pub notes: Option<Notes>,
  /// Total Edit Time Metadata Element
  #[sdk(text_child(qname = "ap:TotalTime"))]
  pub total_time: Option<TotalTime>,
  /// Number of Hidden Slides
  #[sdk(text_child(qname = "ap:HiddenSlides"))]
  pub hidden_slides: Option<HiddenSlides>,
  /// Total Number of Multimedia Clips
  #[sdk(text_child(qname = "ap:MMClips"))]
  pub multimedia_clips: Option<MultimediaClips>,
  /// Thumbnail Display Mode
  #[sdk(text_child(qname = "ap:ScaleCrop"))]
  pub scale_crop: Option<ScaleCrop>,
  /// Heading Pairs
  #[sdk(child(qname = "ap:HeadingPairs"))]
  pub heading_pairs: Option<std::boxed::Box<HeadingPairs>>,
  /// Part Titles
  #[sdk(child(qname = "ap:TitlesOfParts"))]
  pub titles_of_parts: Option<std::boxed::Box<TitlesOfParts>>,
  /// Links Up-to-Date
  #[sdk(text_child(qname = "ap:LinksUpToDate"))]
  pub links_up_to_date: Option<LinksUpToDate>,
  /// Number of Characters (With Spaces)
  #[sdk(text_child(qname = "ap:CharactersWithSpaces"))]
  pub characters_with_spaces: Option<CharactersWithSpaces>,
  /// Shared Document
  #[sdk(text_child(qname = "ap:SharedDoc"))]
  pub shared_document: Option<SharedDocument>,
  /// Relative Hyperlink Base
  #[sdk(text_child(qname = "ap:HyperlinkBase"))]
  pub hyperlink_base: Option<HyperlinkBase>,
  /// Hyperlink List
  #[sdk(child(qname = "ap:HLinks"))]
  pub hyperlink_list: Option<std::boxed::Box<HyperlinkList>>,
  /// Hyperlinks Changed
  #[sdk(text_child(qname = "ap:HyperlinksChanged"))]
  pub hyperlinks_changed: Option<HyperlinksChanged>,
  /// Digital Signature
  #[sdk(child(qname = "ap:DigSig"))]
  pub digital_signature: Option<DigitalSignature>,
  /// Application Name
  #[sdk(text_child(qname = "ap:Application"))]
  pub application: Option<Application>,
  /// Application Version
  #[sdk(text_child(qname = "ap:AppVersion"))]
  pub application_version: Option<ApplicationVersion>,
  /// Document Security
  #[sdk(text_child(qname = "ap:DocSecurity"))]
  pub document_security: Option<DocumentSecurity>,
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
#[sdk(no_prefix_only, qname = "ap:HeadingPairs")]
pub struct HeadingPairs {
  /// Vector
  #[sdk(child(qname = "vt:vector"))]
  pub vt_vector: crate::schemas::vt::VtVector,
}
/// Hyperlink List.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "ap:HLinks")]
pub struct HyperlinkList {
  /// Vector
  #[sdk(child(qname = "vt:vector"))]
  pub vt_vector: Option<crate::schemas::vt::VtVector>,
}
/// Part Titles.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "ap:TitlesOfParts")]
pub struct TitlesOfParts {
  /// Vector
  #[sdk(child(qname = "vt:vector"))]
  pub vt_vector: crate::schemas::vt::VtVector,
}
/// Digital Signature.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(no_prefix_only, qname = "ap:DigSig")]
pub struct DigitalSignature {
  /// Binary Blob
  #[sdk(text_child(qname = "vt:blob"))]
  pub vt_blob: crate::schemas::vt::VtBlob,
}
