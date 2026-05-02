//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentAuthorMonikerList Class.
pub type CommentAuthorMonikerList = Vec<String>;
/// Defines the CommentMonikerList Class.
pub type CommentMonikerList = Vec<String>;
/// Defines the StringTagMonikerList Class.
pub type StringTagMonikerList = Vec<String>;
/// Defines the CustomShowMonikerList Class.
pub type CustomShowMonikerList = Vec<String>;
/// Defines the DocumentMonikerList Class.
pub type DocumentMonikerList = Vec<String>;
/// Defines the SectionMonikerList Class.
pub type SectionMonikerList = Vec<String>;
/// Defines the SlideBaseMonikerList Class.
pub type SlideBaseMonikerList = Vec<String>;
/// Defines the SlideLayoutMonikerList Class.
pub type SlideLayoutMonikerList = Vec<String>;
/// Defines the MainMasterMonikerList Class.
pub type MainMasterMonikerList = Vec<String>;
/// Defines the SlideMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SlideMonikerList/pc:sldMkLst")]
pub struct SlideMonikerList {
  /// Defines the DocumentMoniker Class.
  #[sdk(empty_child(office2016, qname = "pc:CT_DocumentMoniker/pc:docMk"))]
  pub document_moniker: (),
  /// Defines the SlideMoniker Class.
  #[sdk(child(office2016, qname = "pc:CT_SlideMoniker/pc:sldMk"))]
  pub slide_moniker: std::boxed::Box<SlideMoniker>,
  /// _
  #[sdk(any)]
  pub unknown_xml: Vec<String>,
}
/// Defines the SlidePosMonikerList Class.
pub type SlidePosMonikerList = Vec<String>;
/// Defines the NotesMonikerList Class.
pub type NotesMonikerList = Vec<String>;
/// Defines the NotesTextMonikerList Class.
pub type NotesTextMonikerList = Vec<String>;
/// Defines the NotesMasterMonikerList Class.
pub type NotesMasterMonikerList = Vec<String>;
/// Defines the HandoutMonikerList Class.
pub type HandoutMonikerList = Vec<String>;
/// Defines the AnimEffectMkLstAnimationEffectMonikerList Class.
pub type AnimEffectMkLstAnimationEffectMonikerList = Vec<String>;
/// Defines the AnimEffectParentMkLstAnimationEffectMonikerList Class.
pub type AnimEffectParentMkLstAnimationEffectMonikerList = Vec<String>;
/// Defines the OsfTaskPaneAppMonikerList Class.
pub type OsfTaskPaneAppMonikerList = Vec<String>;
/// Defines the SummaryZoomMonikerList Class.
pub type SummaryZoomMonikerList = Vec<String>;
/// Defines the SectionLinkObjMonikerList Class.
pub type SectionLinkObjMonikerList = Vec<String>;
/// Defines the DesignerTagMonikerList Class.
pub type DesignerTagMonikerList = Vec<String>;
/// Defines the CustomXmlPartMonikerList Class.
pub type CustomXmlPartMonikerList = Vec<String>;
/// Defines the SlideMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SlideMoniker/pc:sldMk")]
pub struct SlideMoniker {
  /// cId
  #[sdk(attr(office2016, qname = ":cId"))]
  pub c_id: Option<crate::simple_type::UInt32Value>,
  /// sldId
  #[sdk(attr(office2016, qname = ":sldId"))]
  #[sdk(number_range(range = 256..2147483648))]
  pub sld_id: crate::simple_type::UInt32Value,
}
