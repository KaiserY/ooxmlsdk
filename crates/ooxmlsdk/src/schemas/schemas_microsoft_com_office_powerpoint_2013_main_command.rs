//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentAuthorMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_CommentAuthorMonikerList/pc:cmAuthorMkLst")]
pub struct CommentAuthorMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the CommentMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_CommentMonikerList/pc:cmMkLst")]
pub struct CommentMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the StringTagMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_StringTagMonikerList/pc:tagMkLst")]
pub struct StringTagMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the CustomShowMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_CustomShowMonikerList/pc:custShowMkLst")]
pub struct CustomShowMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DocumentMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_DocumentMonikerList/pc:docMkLst")]
pub struct DocumentMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SectionMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SectionMonikerList/pc:sectionMkLst")]
pub struct SectionMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SlideBaseMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SlideBaseMonikerList/pc:sldBaseMkLst")]
pub struct SlideBaseMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SlideLayoutMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SlideLayoutMonikerList/pc:sldLayoutMkLst")]
pub struct SlideLayoutMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the MainMasterMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_MainMasterMonikerList/pc:sldMasterMkLst")]
pub struct MainMasterMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SlideMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SlideMonikerList/pc:sldMkLst")]
pub struct SlideMonikerList {
  /// Defines the DocumentMoniker Class.
  #[sdk(empty_child(office2016, qname = "pc:CT_DocumentMoniker/pc:docMk"))]
  pub document_moniker: (),
  /// _
  #[sdk(child(office2016, qname = "pc:CT_SlideMoniker/pc:sldMk"))]
  pub slide_moniker: std::boxed::Box<SlideMoniker>,
  /// _
  #[sdk(any)]
  pub unknown_xml: Vec<String>,
}
/// Defines the SlidePosMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SlidePosMonikerList/pc:sldPosMkLst")]
pub struct SlidePosMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the NotesMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_NotesMonikerList/pc:notesMkLst")]
pub struct NotesMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the NotesTextMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_NotesTextMonikerList/pc:notesTxtMkLst")]
pub struct NotesTextMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the NotesMasterMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_NotesMasterMonikerList/pc:notesMasterMkLst")]
pub struct NotesMasterMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the HandoutMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_HandoutMonikerList/pc:handoutMkLst")]
pub struct HandoutMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the AnimEffectMkLstAnimationEffectMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "pc:CT_AnimationEffectMonikerList/pc:animEffectMkLst"
)]
pub struct AnimEffectMkLstAnimationEffectMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the AnimEffectParentMkLstAnimationEffectMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "pc:CT_AnimationEffectMonikerList/pc:animEffectParentMkLst"
)]
pub struct AnimEffectParentMkLstAnimationEffectMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the OsfTaskPaneAppMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_OsfTaskPaneAppMonikerList/pc:tkAppMkLst")]
pub struct OsfTaskPaneAppMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SummaryZoomMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SummaryZoomMonikerList/pc:tocMkLst")]
pub struct SummaryZoomMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SectionLinkObjMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2016,
  qname = "pc:CT_SectionLinkObjMonikerList/pc:sectionLnkObjMkLst"
)]
pub struct SectionLinkObjMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DesignerTagMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_DesignerTagMonikerList/pc:designTagMkLst")]
pub struct DesignerTagMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the CustomXmlPartMonikerList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_CustomXmlPartMonikerList/pc:cXmlMkLst")]
pub struct CustomXmlPartMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SlideMoniker Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2016, qname = "pc:CT_SlideMoniker/pc:sldMk")]
pub struct SlideMoniker {
  /// cId
  #[sdk(attr(office2016, qname = ":cId"))]
  pub c_id: Option<crate::simple_type::UInt32Value>,
  /// sldId
  #[sdk(attr(office2016, qname = ":sldId"))]
  #[sdk(number_range(
    source = 1u32,
    min = "256",
    max = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub sld_id: crate::simple_type::UInt32Value,
}
