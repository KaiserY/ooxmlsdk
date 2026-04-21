//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the CommentAuthorMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:cmAuthorMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_CommentAuthorMonikerList/pc:cmAuthorMkLst")]
pub struct CommentAuthorMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<CommentAuthorMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CommentAuthorMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the CommentMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:cmMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_CommentMonikerList/pc:cmMkLst")]
pub struct CommentMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<CommentMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CommentMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the StringTagMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:tagMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_StringTagMonikerList/pc:tagMkLst")]
pub struct StringTagMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<StringTagMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum StringTagMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the CustomShowMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:custShowMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_CustomShowMonikerList/pc:custShowMkLst")]
pub struct CustomShowMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<CustomShowMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CustomShowMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the DocumentMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:docMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_DocumentMonikerList/pc:docMkLst")]
pub struct DocumentMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<DocumentMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum DocumentMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the SectionMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sectionMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SectionMonikerList/pc:sectionMkLst")]
pub struct SectionMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<SectionMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SectionMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the SlideBaseMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldBaseMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SlideBaseMonikerList/pc:sldBaseMkLst")]
pub struct SlideBaseMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<SlideBaseMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SlideBaseMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the SlideLayoutMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldLayoutMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SlideLayoutMonikerList/pc:sldLayoutMkLst")]
pub struct SlideLayoutMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<SlideLayoutMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SlideLayoutMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the MainMasterMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldMasterMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_MainMasterMonikerList/pc:sldMasterMkLst")]
pub struct MainMasterMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<MainMasterMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum MainMasterMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the SlideMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SlideMonikerList/pc:sldMkLst")]
pub struct SlideMonikerList {
  /// _
  #[sdk(child(qname = "pc:CT_DocumentMoniker/pc:docMk"))]
  pub document_moniker: std::boxed::Box<DocumentMoniker>,
  /// _
  #[sdk(child(qname = "pc:CT_SlideMoniker/pc:sldMk"))]
  pub slide_moniker: std::boxed::Box<SlideMoniker>,
  /// _
  #[sdk(any)]
  pub unknown_xml: Vec<String>,
}
/// Defines the SlidePosMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldPosMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SlidePosMonikerList/pc:sldPosMkLst")]
pub struct SlidePosMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<SlidePosMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SlidePosMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the NotesMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:notesMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_NotesMonikerList/pc:notesMkLst")]
pub struct NotesMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<NotesMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum NotesMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the NotesTextMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:notesTxtMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_NotesTextMonikerList/pc:notesTxtMkLst")]
pub struct NotesTextMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<NotesTextMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum NotesTextMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the NotesMasterMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:notesMasterMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_NotesMasterMonikerList/pc:notesMasterMkLst")]
pub struct NotesMasterMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<NotesMasterMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum NotesMasterMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the HandoutMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:handoutMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_HandoutMonikerList/pc:handoutMkLst")]
pub struct HandoutMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<HandoutMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum HandoutMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the AnimEffectMkLstAnimationEffectMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:animEffectMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_AnimationEffectMonikerList/pc:animEffectMkLst")]
pub struct AnimEffectMkLstAnimationEffectMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<AnimEffectMkLstAnimationEffectMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum AnimEffectMkLstAnimationEffectMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the AnimEffectParentMkLstAnimationEffectMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:animEffectParentMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_AnimationEffectMonikerList/pc:animEffectParentMkLst")]
pub struct AnimEffectParentMkLstAnimationEffectMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<AnimEffectParentMkLstAnimationEffectMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum AnimEffectParentMkLstAnimationEffectMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the OpenXmlAnimationEffectMonikerListElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_AnimationEffectMonikerList/")]
pub struct OpenXmlAnimationEffectMonikerListElement {}
/// Defines the OsfTaskPaneAppMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:tkAppMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_OsfTaskPaneAppMonikerList/pc:tkAppMkLst")]
pub struct OsfTaskPaneAppMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<OsfTaskPaneAppMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum OsfTaskPaneAppMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the SummaryZoomMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:tocMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SummaryZoomMonikerList/pc:tocMkLst")]
pub struct SummaryZoomMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<SummaryZoomMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SummaryZoomMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the SectionLinkObjMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sectionLnkObjMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SectionLinkObjMonikerList/pc:sectionLnkObjMkLst")]
pub struct SectionLinkObjMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<SectionLinkObjMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum SectionLinkObjMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the DesignerTagMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:designTagMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_DesignerTagMonikerList/pc:designTagMkLst")]
pub struct DesignerTagMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<DesignerTagMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum DesignerTagMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the CustomXmlPartMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:cXmlMkLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_CustomXmlPartMonikerList/pc:cXmlMkLst")]
pub struct CustomXmlPartMonikerList {
  #[sdk(choice(any))]
  pub xml_children: Vec<CustomXmlPartMonikerListChoice>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum CustomXmlPartMonikerListChoice {
  #[sdk(any)]
  UnknownXml(String),
}
/// Defines the DocumentMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:docMk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_DocumentMoniker/pc:docMk")]
pub struct DocumentMoniker {}
/// Defines the SlideMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldMk.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SlideMoniker/pc:sldMk")]
pub struct SlideMoniker {
  /// cId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :cId
  #[sdk(attr(qname = ":cId"))]
  pub c_id: Option<crate::simple_type::UInt32Value>,
  /// sldId
  ///
  /// Available in Office2016 and above.
  ///
  /// Represents the following attribute in the schema: :sldId
  #[sdk(attr(qname = ":sldId"))]
  #[sdk(number_range(
    source = 1u32,
    min = "256",
    max = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub sld_id: crate::simple_type::UInt32Value,
}
