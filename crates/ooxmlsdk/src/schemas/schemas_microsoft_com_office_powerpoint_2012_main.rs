//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

/// Defines the PresetTransition Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:prstTrans.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_PresetTransition/p15:prstTrans")]
pub struct PresetTransition {
  /// prst
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :prst
  #[sdk(attr(qname = ":prst"))]
  pub preset: Option<crate::simple_type::StringValue>,
  /// invX
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :invX
  #[sdk(attr(qname = ":invX"))]
  pub inv_x: Option<crate::simple_type::BooleanValue>,
  /// invY
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :invY
  #[sdk(attr(qname = ":invY"))]
  pub inv_y: Option<crate::simple_type::BooleanValue>,
}
/// Defines the PresenceInfo Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:presenceInfo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_PresenceInfo/p15:presenceInfo")]
pub struct PresenceInfo {
  /// userId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :userId
  #[sdk(attr(qname = ":userId"))]
  pub user_id: crate::simple_type::StringValue,
  /// providerId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :providerId
  #[sdk(attr(qname = ":providerId"))]
  pub provider_id: crate::simple_type::StringValue,
}
/// Defines the ThreadingInfo Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:threadingInfo.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_CommentThreading/p15:threadingInfo")]
pub struct ThreadingInfo {
  /// timeZoneBias
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :timeZoneBias
  #[sdk(attr(qname = ":timeZoneBias"))]
  pub time_zone_bias: Option<crate::simple_type::Int32Value>,
  /// _
  #[sdk(child(qname = "p15:CT_ParentCommentIdentifier/p15:parentCm"))]
  pub parent_comment_identifier: Option<ParentCommentIdentifier>,
}
/// Defines the SlideGuideList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:sldGuideLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_ExtendedGuideList/p15:sldGuideLst")]
pub struct SlideGuideList {
  /// _
  #[sdk(child(qname = "p15:CT_ExtendedGuide/p15:guide"))]
  pub p15_guide: Vec<ExtendedGuide>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p15:extLst"))]
  pub p15_ext_lst: Option<ExtensionList>,
}
/// Defines the NotesGuideList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:notesGuideLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_ExtendedGuideList/p15:notesGuideLst")]
pub struct NotesGuideList {
  /// _
  #[sdk(child(qname = "p15:CT_ExtendedGuide/p15:guide"))]
  pub p15_guide: Vec<ExtendedGuide>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p15:extLst"))]
  pub p15_ext_lst: Option<ExtensionList>,
}
/// Defines the ExtendedGuideList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is .
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_ExtendedGuideList/")]
pub struct ExtendedGuideList {
  #[sdk(choice)]
  pub xml_children: Vec<ExtendedGuideListChoice>,
}
/// Defines the ChartTrackingReferenceBased Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:chartTrackingRefBased.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_ChartTrackingRefBased/p15:chartTrackingRefBased")]
pub struct ChartTrackingReferenceBased {
  pub xmlns: Option<String>,
  pub xmlns_map: std::collections::HashMap<String, String>,
  /// val
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :val
  #[sdk(attr(qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the ParentCommentIdentifier Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:parentCm.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_ParentCommentIdentifier/p15:parentCm")]
pub struct ParentCommentIdentifier {
  /// authorId
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :authorId
  #[sdk(attr(qname = ":authorId"))]
  pub author_id: Option<crate::simple_type::UInt32Value>,
  /// idx
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :idx
  #[sdk(attr(qname = ":idx"))]
  pub index: Option<crate::simple_type::UInt32Value>,
}
/// Defines the ColorType Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:clr.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "a:CT_Color/p15:clr")]
pub struct ColorType {
  #[sdk(choice)]
  pub xml_children: Option<ColorTypeChoice>,
}
/// Defines the ExtensionList Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:extLst.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p:CT_ExtensionList/p15:extLst")]
pub struct ExtensionList {
  ///Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the ExtendedGuide Class.
///
/// Available in Office2013 and above.
///
/// When the object is serialized out as xml, it's qualified name is p15:guide.
#[derive(Clone, Debug, Default, ooxmlsdk_derive::SdkType)]
#[sdk(qname = "p15:CT_ExtendedGuide/p15:guide")]
pub struct ExtendedGuide {
  /// id
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :id
  #[sdk(attr(qname = ":id"))]
  pub id: crate::simple_type::UInt32Value,
  /// name
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :name
  #[sdk(attr(qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// orient
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :orient
  #[sdk(attr(qname = ":orient"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub orientation:
    Option<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues>,
  /// pos
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :pos
  #[sdk(attr(qname = ":pos"))]
  pub position: Option<crate::simple_type::Int32Value>,
  /// userDrawn
  ///
  /// Available in Office2013 and above.
  ///
  /// Represents the following attribute in the schema: :userDrawn
  #[sdk(attr(qname = ":userDrawn"))]
  pub is_user_drawn: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(qname = "a:CT_Color/p15:clr"))]
  pub color_type: std::boxed::Box<ColorType>,
  /// _
  #[sdk(child(qname = "p:CT_ExtensionList/p15:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ExtendedGuideListChoice {
  #[sdk(child(qname = "p15:CT_ExtendedGuide/p15:guide"))]
  P15Guide(std::boxed::Box<ExtendedGuide>),
  #[sdk(child(qname = "p:CT_ExtensionList/p15:extLst"))]
  P15ExtLst(std::boxed::Box<ExtensionList>),
}
#[derive(Clone, Debug, ooxmlsdk_derive::SdkChoice)]
pub enum ColorTypeChoice {
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
