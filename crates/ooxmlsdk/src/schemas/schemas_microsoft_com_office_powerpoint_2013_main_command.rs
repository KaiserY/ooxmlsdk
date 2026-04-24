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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_CommentAuthorMonikerList/pc:cmAuthorMkLst")]
pub struct CommentAuthorMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the CommentMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:cmMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_CommentMonikerList/pc:cmMkLst")]
pub struct CommentMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the StringTagMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:tagMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_StringTagMonikerList/pc:tagMkLst")]
pub struct StringTagMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the CustomShowMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:custShowMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_CustomShowMonikerList/pc:custShowMkLst")]
pub struct CustomShowMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DocumentMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:docMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_DocumentMonikerList/pc:docMkLst")]
pub struct DocumentMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SectionMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sectionMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SectionMonikerList/pc:sectionMkLst")]
pub struct SectionMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SlideBaseMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldBaseMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SlideBaseMonikerList/pc:sldBaseMkLst")]
pub struct SlideBaseMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SlideLayoutMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldLayoutMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SlideLayoutMonikerList/pc:sldLayoutMkLst")]
pub struct SlideLayoutMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the MainMasterMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldMasterMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_MainMasterMonikerList/pc:sldMasterMkLst")]
pub struct MainMasterMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SlideMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
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
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SlidePosMonikerList/pc:sldPosMkLst")]
pub struct SlidePosMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the NotesMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:notesMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_NotesMonikerList/pc:notesMkLst")]
pub struct NotesMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the NotesTextMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:notesTxtMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_NotesTextMonikerList/pc:notesTxtMkLst")]
pub struct NotesTextMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the NotesMasterMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:notesMasterMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_NotesMasterMonikerList/pc:notesMasterMkLst")]
pub struct NotesMasterMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the HandoutMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:handoutMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_HandoutMonikerList/pc:handoutMkLst")]
pub struct HandoutMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the AnimEffectMkLstAnimationEffectMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:animEffectMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_AnimationEffectMonikerList/pc:animEffectMkLst")]
pub struct AnimEffectMkLstAnimationEffectMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the AnimEffectParentMkLstAnimationEffectMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:animEffectParentMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_AnimationEffectMonikerList/pc:animEffectParentMkLst")]
pub struct AnimEffectParentMkLstAnimationEffectMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the OpenXmlAnimationEffectMonikerListElement Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_AnimationEffectMonikerList/")]
pub struct OpenXmlAnimationEffectMonikerListElement {}
/// Defines the OsfTaskPaneAppMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:tkAppMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_OsfTaskPaneAppMonikerList/pc:tkAppMkLst")]
pub struct OsfTaskPaneAppMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SummaryZoomMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:tocMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SummaryZoomMonikerList/pc:tocMkLst")]
pub struct SummaryZoomMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the SectionLinkObjMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sectionLnkObjMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_SectionLinkObjMonikerList/pc:sectionLnkObjMkLst")]
pub struct SectionLinkObjMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DesignerTagMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:designTagMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_DesignerTagMonikerList/pc:designTagMkLst")]
pub struct DesignerTagMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the CustomXmlPartMonikerList Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:cXmlMkLst.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_CustomXmlPartMonikerList/pc:cXmlMkLst")]
pub struct CustomXmlPartMonikerList {
  #[sdk(any)]
  pub xml_children: Vec<String>,
}
/// Defines the DocumentMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:docMk.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "pc:CT_DocumentMoniker/pc:docMk")]
pub struct DocumentMoniker {}
/// Defines the SlideMoniker Class.
///
/// Available in Office2016 and above.
///
/// When the object is serialized out as xml, it's qualified name is pc:sldMk.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
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
