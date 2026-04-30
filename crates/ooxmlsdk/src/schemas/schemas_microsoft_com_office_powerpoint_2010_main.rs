//
// -----------------------------------------------------------------------------
//  THIS FILE WAS @generated AUTOMATICALLY. DO NOT MODIFY THIS FILE MANUALLY.
// -----------------------------------------------------------------------------
//

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionPatternValues {
  #[sdk(rename = "diamond")]
  #[default]
  Diamond,
  #[sdk(rename = "hexagon")]
  Hexagon,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionCenterDirectionTypeValues {
  #[sdk(rename = "center")]
  #[default]
  Center,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionShredPatternValues {
  #[sdk(rename = "strip")]
  #[default]
  Strip,
  #[sdk(rename = "rectangle")]
  Rectangle,
}
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, ooxmlsdk_derive::SdkEnum)]
pub enum TransitionLeftRightDirectionTypeValues {
  #[sdk(rename = "l")]
  #[default]
  Left,
  #[sdk(rename = "r")]
  Right,
}
/// Defines the NonVisualContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_ContentPartNonVisual/p14:nvContentPartPr")]
pub struct NonVisualContentPartProperties {
  /// _
  #[sdk(child(office2010, qname = "a:CT_NonVisualDrawingProps/p14:cNvPr"))]
  pub non_visual_drawing_properties: std::boxed::Box<NonVisualDrawingProperties>,
  /// _
  #[sdk(child(
    office2010,
    qname = "a14:CT_NonVisualInkContentPartProperties/p14:cNvContentPartPr"
  ))]
  pub non_visual_ink_content_part_properties:
    Option<std::boxed::Box<NonVisualInkContentPartProperties>>,
  /// _
  #[sdk(child(office2010, qname = "p:CT_ApplicationNonVisualDrawingProps/p14:nvPr"))]
  pub application_non_visual_drawing_properties:
    std::boxed::Box<ApplicationNonVisualDrawingProperties>,
}
/// Defines the Transform2D Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Transform2D/p14:xfrm")]
pub struct Transform2D {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Rotation
  #[sdk(attr(qname = ":rot"))]
  pub rotation: Option<crate::simple_type::Int32Value>,
  /// Horizontal Flip
  #[sdk(attr(qname = ":flipH"))]
  pub horizontal_flip: Option<crate::simple_type::BooleanValue>,
  /// Vertical Flip
  #[sdk(attr(qname = ":flipV"))]
  pub vertical_flip: Option<crate::simple_type::BooleanValue>,
  /// Offset
  #[sdk(child(qname = "a:CT_Point2D/a:off"))]
  pub offset: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Offset>,
  /// Extents
  #[sdk(child(qname = "a:CT_PositiveSize2D/a:ext"))]
  pub extents: Option<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::Extents>,
}
/// Defines the ExtensionListModify Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_ExtensionListModify/p14:extLst")]
pub struct ExtensionListModify {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Modify
  #[sdk(attr(qname = ":mod"))]
  pub modify: Option<crate::simple_type::BooleanValue>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the Media Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_Media/p14:media")]
pub struct Media {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Embedded Picture Reference
  #[sdk(attr(qname = "r:embed"))]
  pub embed: Option<crate::simple_type::StringValue>,
  /// Linked Picture Reference
  #[sdk(attr(qname = "r:link"))]
  pub link: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "p14:CT_MediaTrim/p14:trim"))]
  pub media_trim: Option<MediaTrim>,
  /// _
  #[sdk(child(office2010, qname = "p14:CT_MediaFade/p14:fade"))]
  pub media_fade: Option<MediaFade>,
  /// _
  #[sdk(child(office2010, qname = "p14:CT_MediaBookmarkList/p14:bmkLst"))]
  pub media_bookmark_list: Option<MediaBookmarkList>,
  /// _
  #[sdk(child(office2010, qname = "p:CT_ExtensionList/p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the VortexTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_SideDirectionTransition/p14:vortex")]
pub struct VortexTransition {
    /// Direction
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
}
/// Defines the PanTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_SideDirectionTransition/p14:pan")]
pub struct PanTransition {
    /// Direction
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
}
/// Defines the SwitchTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:switch")]
pub struct SwitchTransition {
  /// dir
  #[sdk(attr(office2010, qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the FlipTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:flip")]
pub struct FlipTransition {
  /// dir
  #[sdk(attr(office2010, qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the FerrisTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:ferris")]
pub struct FerrisTransition {
  /// dir
  #[sdk(attr(office2010, qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the GalleryTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:gallery")]
pub struct GalleryTransition {
  /// dir
  #[sdk(attr(office2010, qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the ConveyorTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_LeftRightDirectionTransition/p14:conveyor")]
pub struct ConveyorTransition {
  /// dir
  #[sdk(attr(office2010, qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the RippleTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_RippleTransition/p14:ripple")]
pub struct RippleTransition {
  /// dir
  #[sdk(attr(office2010, qname = ":dir"))]
  #[sdk(string_set(source = 0u32, union = 0u64, values = &["lu", "ru", "ld", "rd"]))]
  #[sdk(string_set(source = 1u32, union = 0u64, values = &["center"]))]
  pub direction: Option<crate::simple_type::StringValue>,
}
/// Defines the PrismTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_PrismTransition/p14:prism")]
pub struct PrismTransition {
    /// dir
    #[sdk(attr(office2010, qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
    /// isContent
    #[sdk(attr(office2010, qname = ":isContent"))]
    pub is_content: Option<crate::simple_type::BooleanValue>,
    /// isInverted
    #[sdk(attr(office2010, qname = ":isInverted"))]
    pub is_inverted: Option<crate::simple_type::BooleanValue>,
}
/// Defines the DoorsTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_OrientationTransition/p14:doors")]
pub struct DoorsTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction:
    Option<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues>,
}
/// Defines the WindowTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_OrientationTransition/p14:window")]
pub struct WindowTransition {
  /// Transition Direction
  #[sdk(attr(qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction:
    Option<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::DirectionValues>,
}
/// Defines the GlitterTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_GlitterTransition/p14:glitter")]
pub struct GlitterTransition {
    /// dir
    #[sdk(attr(office2010, qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionSlideDirectionValues,
    >,
    /// pattern
    #[sdk(attr(office2010, qname = ":pattern"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub pattern: Option<TransitionPatternValues>,
}
/// Defines the WarpTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_InOutTransition/p14:warp")]
pub struct WarpTransition {
    /// Direction
    #[sdk(attr(qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
}
/// Defines the FlythroughTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_FlyThroughTransition/p14:flythrough")]
pub struct FlythroughTransition {
    /// dir
    #[sdk(attr(office2010, qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
    /// hasBounce
    #[sdk(attr(office2010, qname = ":hasBounce"))]
    pub has_bounce: Option<crate::simple_type::BooleanValue>,
}
/// Defines the ShredTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_ShredTransition/p14:shred")]
pub struct ShredTransition {
    /// pattern
    #[sdk(attr(office2010, qname = ":pattern"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub pattern: Option<TransitionShredPatternValues>,
    /// dir
    #[sdk(attr(office2010, qname = ":dir"))]
    #[sdk(string_format(source = 0u32, kind = "token"))]
    pub direction: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TransitionInOutDirectionValues,
    >,
}
/// Defines the RevealTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_RevealTransition/p14:reveal")]
pub struct RevealTransition {
  /// thruBlk
  #[sdk(attr(office2010, qname = ":thruBlk"))]
  pub through_black: Option<crate::simple_type::BooleanValue>,
  /// dir
  #[sdk(attr(office2010, qname = ":dir"))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub direction: Option<TransitionLeftRightDirectionTypeValues>,
}
/// Defines the WheelReverseTransition Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_WheelTransition/p14:wheelReverse")]
pub struct WheelReverseTransition {
  /// Spokes
  #[sdk(attr(qname = ":spokes"))]
  pub spokes: Option<crate::simple_type::UInt32Value>,
}
/// Defines the BookmarkTarget Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaBookmarkTarget/p14:bmkTgt")]
pub struct BookmarkTarget {
  /// spid
  #[sdk(attr(office2010, qname = ":spid"))]
  pub shape_id: crate::simple_type::UInt32Value,
  /// bmkName
  #[sdk(attr(office2010, qname = ":bmkName"))]
  pub bookmark_name: crate::simple_type::StringValue,
}
/// Defines the SectionProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_SectionProperties/p14:sectionPr")]
pub struct SectionProperties {
  /// _
  #[sdk(child(office2010, qname = "p14:CT_SectionOld/p14:section"))]
  pub p14_section: Vec<SectionOld>,
}
/// Defines the SectionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_SectionList/p14:sectionLst")]
pub struct SectionList {
  /// _
  #[sdk(child(office2010, qname = "p14:CT_Section/p14:section"))]
  pub p14_section: Vec<Section>,
}
/// Defines the BrowseMode Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_BrowseMode/p14:browseMode")]
pub struct BrowseMode {
  /// showStatus
  #[sdk(attr(office2010, qname = ":showStatus"))]
  pub show_status: Option<crate::simple_type::BooleanValue>,
}
/// Defines the LaserColor Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_Color/p14:laserClr")]
pub struct LaserColor {
  #[sdk(choice(
    qname = "a:CT_ScRgbColor/a:scrgbClr",
    qname = "a:CT_SRgbColor/a:srgbClr",
    qname = "a:CT_HslColor/a:hslClr",
    qname = "a:CT_SystemColor/a:sysClr",
    qname = "a:CT_SchemeColor/a:schemeClr",
    qname = "a:CT_PresetColor/a:prstClr"
  ))]
  pub xml_children: Option<LaserColorChoice>,
}
/// Defines the DefaultImageDpi Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_DefaultImageDpi/p14:defaultImageDpi")]
pub struct DefaultImageDpi {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the DiscardImageEditData Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "p14:CT_DiscardImageEditData/p14:discardImageEditData"
)]
pub struct DiscardImageEditData {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the ShowMediaControls Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_ShowMediaControls/p14:showMediaCtrls")]
pub struct ShowMediaControls {
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: crate::simple_type::BooleanValue,
}
/// Defines the LaserTraceList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_LaserTraceList/p14:laserTraceLst")]
pub struct LaserTraceList {
  /// _
  #[sdk(child(office2010, qname = "p14:CT_LaserTrace/p14:tracePtLst"))]
  pub p14_trace_pt_lst: Vec<TracePointList>,
}
/// Defines the CreationId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_RandomId/p14:creationId")]
pub struct CreationId {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the ModificationId Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_RandomId/p14:modId")]
pub struct ModificationId {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// val
  #[sdk(attr(office2010, qname = ":val"))]
  pub val: crate::simple_type::UInt32Value,
}
/// Defines the ShowEventRecordList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_ShowEventRecordList/p14:showEvtLst")]
pub struct ShowEventRecordList {
  #[sdk(choice(
    qname = "p14:CT_TriggerEventRecord/p14:triggerEvt",
    qname = "p14:CT_MediaPlaybackEventRecord/p14:playEvt",
    qname = "p14:CT_MediaPlaybackEventRecord/p14:stopEvt",
    qname = "p14:CT_MediaPlaybackEventRecord/p14:pauseEvt",
    qname = "p14:CT_MediaPlaybackEventRecord/p14:resumeEvt",
    qname = "p14:CT_MediaSeekEventRecord/p14:seekEvt",
    qname = "p14:CT_NullEventRecord/p14:nullEvt"
  ))]
  pub xml_children: Vec<ShowEventRecordListChoice>,
}
/// Defines the NonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "a:CT_NonVisualDrawingProps/p14:cNvPr")]
pub struct NonVisualDrawingProperties {
    pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
    /// Application defined unique identifier.
    #[sdk(attr(qname = ":id"))]
    pub id: crate::simple_type::UInt32Value,
    /// Name compatible with Object Model (non-unique).
    #[sdk(attr(qname = ":name"))]
    pub name: crate::simple_type::StringValue,
    /// Description of the drawing element.
    #[sdk(attr(qname = ":descr"))]
    pub description: Option<crate::simple_type::StringValue>,
    /// Flag determining to show or hide this element.
    #[sdk(attr(qname = ":hidden"))]
    pub hidden: Option<crate::simple_type::BooleanValue>,
    /// Title
    #[sdk(attr(qname = ":title"))]
    pub title: Option<crate::simple_type::StringValue>,
    /// Hyperlink associated with clicking or selecting the element.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkClick"))]
    pub hyperlink_on_click: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnClick,
        >,
    >,
    /// Hyperlink associated with hovering over the element.
    #[sdk(child(qname = "a:CT_Hyperlink/a:hlinkHover"))]
    pub hyperlink_on_hover: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HyperlinkOnHover,
        >,
    >,
    /// Future extension
    #[sdk(child(qname = "a:CT_NonVisualDrawingPropsExtensionList/a:extLst"))]
    pub non_visual_drawing_properties_extension_list: Option<
        crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::NonVisualDrawingPropertiesExtensionList,
    >,
}
/// Defines the NonVisualInkContentPartProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(
  office2010,
  qname = "a14:CT_NonVisualInkContentPartProperties/p14:cNvContentPartPr"
)]
pub struct NonVisualInkContentPartProperties {
  /// isComment
  #[sdk(attr(office2010, qname = ":isComment"))]
  pub is_comment: Option<crate::simple_type::BooleanValue>,
  /// _
  #[sdk(child(office2010, qname = "a14:CT_ContentPartLocking/a14:cpLocks"))]
  pub content_part_locks: Option<
    std::boxed::Box<
      crate::schemas::schemas_microsoft_com_office_drawing_2010_main::ContentPartLocks,
    >,
  >,
  /// _
  #[sdk(child(office2010, qname = "a:CT_OfficeArtExtensionList/a14:extLst"))]
  pub office_art_extension_list:
    Option<crate::schemas::schemas_microsoft_com_office_drawing_2010_main::OfficeArtExtensionList>,
}
/// Defines the ApplicationNonVisualDrawingProperties Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_ApplicationNonVisualDrawingProps/p14:nvPr")]
pub struct ApplicationNonVisualDrawingProperties {
    /// Is a Photo Album
    #[sdk(attr(qname = ":isPhoto"))]
    pub is_photo: Option<crate::simple_type::BooleanValue>,
    /// Is User Drawn
    #[sdk(attr(qname = ":userDrawn"))]
    pub user_drawn: Option<crate::simple_type::BooleanValue>,
    /// Placeholder Shape
    #[sdk(child(qname = "p:CT_Placeholder/p:ph"))]
    pub placeholder_shape: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::PlaceholderShape,
        >,
    >,
    #[sdk(
        choice(
            qname = "a:CT_AudioCD/a:audioCd",
            qname = "a:CT_EmbeddedWAVAudioFile/a:wavAudioFile",
            qname = "a:CT_AudioFile/a:audioFile",
            qname = "a:CT_VideoFile/a:videoFile",
            qname = "a:CT_QuickTimeFile/a:quickTimeFile"
        )
    )]
    pub application_non_visual_drawing_properties_choice: Option<
        ApplicationNonVisualDrawingPropertiesChoice,
    >,
    /// _
    #[sdk(child(qname = "p:CT_CustomerDataList/p:custDataLst"))]
    pub p_cust_data_lst: Option<
        std::boxed::Box<
            crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::CustomerDataList,
        >,
    >,
    /// _
    #[sdk(child(qname = "p:CT_ApplicationNonVisualDrawingPropsExtensionList/p:extLst"))]
    pub p_ext_lst: Option<
        crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::ApplicationNonVisualDrawingPropertiesExtensionList,
    >,
}
/// Defines the MediaBookmark Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaBookmark/p14:bmk")]
pub struct MediaBookmark {
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// time
  #[sdk(attr(office2010, qname = ":time"))]
  pub time: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaTrim Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaTrim/p14:trim")]
pub struct MediaTrim {
  /// st
  #[sdk(attr(office2010, qname = ":st"))]
  pub start: Option<crate::simple_type::StringValue>,
  /// end
  #[sdk(attr(office2010, qname = ":end"))]
  pub end: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaFade Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaFade/p14:fade")]
pub struct MediaFade {
  /// in
  #[sdk(attr(office2010, qname = ":in"))]
  pub in_duration: Option<crate::simple_type::StringValue>,
  /// out
  #[sdk(attr(office2010, qname = ":out"))]
  pub out_duration: Option<crate::simple_type::StringValue>,
}
/// Defines the MediaBookmarkList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaBookmarkList/p14:bmkLst")]
pub struct MediaBookmarkList {
  /// _
  #[sdk(child(office2010, qname = "p14:CT_MediaBookmark/p14:bmk"))]
  pub p14_bmk: Vec<MediaBookmark>,
}
/// Defines the ExtensionList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p:CT_ExtensionList/p14:extLst")]
pub struct ExtensionList {
  pub xmlns: Vec<crate::common::XmlNamespaceDecl>,
  /// Extension.
  #[sdk(child(qname = "p:CT_Extension/p:ext"))]
  pub extension:
    Vec<crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::Extension>,
}
/// Defines the SectionOld Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_SectionOld/p14:section")]
pub struct SectionOld {
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// slideIdLst
  #[sdk(attr(office2010, qname = ":slideIdLst"))]
  #[sdk(number_range(
    source = 0u32,
    min = "256",
    max = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub slide_id_list: Option<crate::simple_type::ListValue<crate::simple_type::UInt32Value>>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "p:CT_ExtensionList/p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the SectionSlideIdListEntry Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_SectionSlideIdListEntry/p14:sldId")]
pub struct SectionSlideIdListEntry {
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(number_range(
    source = 1u32,
    min = "256",
    max = "2147483648",
    min_inclusive = true,
    max_inclusive = false
  ))]
  pub id: crate::simple_type::UInt32Value,
}
/// Defines the SectionSlideIdList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_SectionSlideIdList/p14:sldIdLst")]
pub struct SectionSlideIdList {
  /// _
  #[sdk(child(office2010, qname = "p14:CT_SectionSlideIdListEntry/p14:sldId"))]
  pub p14_sld_id: Vec<SectionSlideIdListEntry>,
}
/// Defines the Section Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_Section/p14:section")]
pub struct Section {
  /// name
  #[sdk(attr(office2010, qname = ":name"))]
  pub name: Option<crate::simple_type::StringValue>,
  /// id
  #[sdk(attr(office2010, qname = ":id"))]
  #[sdk(pattern(
    source = 0u32,
    regex = "\\{[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}\\}"
  ))]
  #[sdk(string_format(source = 0u32, kind = "token"))]
  pub id: Option<crate::simple_type::StringValue>,
  /// _
  #[sdk(child(office2010, qname = "p14:CT_SectionSlideIdList/p14:sldIdLst"))]
  pub section_slide_id_list: std::boxed::Box<SectionSlideIdList>,
  /// _
  #[sdk(child(office2010, qname = "p:CT_ExtensionList/p14:extLst"))]
  pub extension_list: Option<ExtensionList>,
}
/// Defines the TracePoint Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_LaserTracePoint/p14:tracePt")]
pub struct TracePoint {
  /// t
  #[sdk(attr(office2010, qname = ":t"))]
  pub time: crate::simple_type::StringValue,
  /// x
  #[sdk(attr(office2010, qname = ":x"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub x_coordinate: crate::simple_type::Int64Value,
  /// y
  #[sdk(attr(office2010, qname = ":y"))]
  #[sdk(number_range(
    source = 1u32,
    min = "-27273042329600",
    max = "27273042316900",
    min_inclusive = true,
    max_inclusive = true
  ))]
  pub y_coordinate: crate::simple_type::Int64Value,
}
/// Defines the TracePointList Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_LaserTrace/p14:tracePtLst")]
pub struct TracePointList {
  /// _
  #[sdk(child(office2010, qname = "p14:CT_LaserTracePoint/p14:tracePt"))]
  pub p14_trace_pt: Vec<TracePoint>,
}
/// Defines the TriggerEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_TriggerEventRecord/p14:triggerEvt")]
pub struct TriggerEventRecord {
  /// type
  #[sdk(attr(office2010, qname = ":type"))]
  #[sdk(string_format(source = 1u32, kind = "token"))]
  pub r#type:
    crate::schemas::schemas_openxmlformats_org_presentationml_2006_main::TriggerEventValues,
  /// time
  #[sdk(attr(office2010, qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(office2010, qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the PlayEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaPlaybackEventRecord/p14:playEvt")]
pub struct PlayEventRecord {
  /// time
  #[sdk(attr(office2010, qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(office2010, qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the StopEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaPlaybackEventRecord/p14:stopEvt")]
pub struct StopEventRecord {
  /// time
  #[sdk(attr(office2010, qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(office2010, qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the PauseEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaPlaybackEventRecord/p14:pauseEvt")]
pub struct PauseEventRecord {
  /// time
  #[sdk(attr(office2010, qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(office2010, qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the ResumeEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaPlaybackEventRecord/p14:resumeEvt")]
pub struct ResumeEventRecord {
  /// time
  #[sdk(attr(office2010, qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(office2010, qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
/// Defines the SeekEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_MediaSeekEventRecord/p14:seekEvt")]
pub struct SeekEventRecord {
  /// time
  #[sdk(attr(office2010, qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(office2010, qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
  /// seek
  #[sdk(attr(office2010, qname = ":seek"))]
  pub seek: crate::simple_type::StringValue,
}
/// Defines the NullEventRecord Class.
#[derive(Clone, Debug, Default, PartialEq, ooxmlsdk_derive::SdkType)]
#[sdk(office2010, qname = "p14:CT_NullEventRecord/p14:nullEvt")]
pub struct NullEventRecord {
  /// time
  #[sdk(attr(office2010, qname = ":time"))]
  pub time: crate::simple_type::StringValue,
  /// objId
  #[sdk(attr(office2010, qname = ":objId"))]
  pub object_id: crate::simple_type::UInt32Value,
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum LaserColorChoice {
  /// RGB Color Model - Percentage Variant.
  #[sdk(child(qname = "a:CT_ScRgbColor/a:scrgbClr"))]
  AScrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelPercentage,
    >,
  ),
  /// RGB Color Model - Hex Variant.
  #[sdk(child(qname = "a:CT_SRgbColor/a:srgbClr"))]
  ASrgbClr(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::RgbColorModelHex,
    >,
  ),
  /// Hue, Saturation, Luminance Color Model.
  #[sdk(child(qname = "a:CT_HslColor/a:hslClr"))]
  AHslClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::HslColor>,
  ),
  /// System Color.
  #[sdk(child(qname = "a:CT_SystemColor/a:sysClr"))]
  ASysClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SystemColor>,
  ),
  /// Scheme Color.
  #[sdk(child(qname = "a:CT_SchemeColor/a:schemeClr"))]
  ASchemeClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::SchemeColor>,
  ),
  /// Preset Color.
  #[sdk(child(qname = "a:CT_PresetColor/a:prstClr"))]
  APrstClr(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::PresetColor>,
  ),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ShowEventRecordListChoice {
  /// Defines the TriggerEventRecord Class.
  #[sdk(child(office2010, qname = "p14:CT_TriggerEventRecord/p14:triggerEvt"))]
  P14TriggerEvt(std::boxed::Box<TriggerEventRecord>),
  /// Defines the PlayEventRecord Class.
  #[sdk(child(office2010, qname = "p14:CT_MediaPlaybackEventRecord/p14:playEvt"))]
  P14PlayEvt(std::boxed::Box<PlayEventRecord>),
  /// Defines the StopEventRecord Class.
  #[sdk(child(office2010, qname = "p14:CT_MediaPlaybackEventRecord/p14:stopEvt"))]
  P14StopEvt(std::boxed::Box<StopEventRecord>),
  /// Defines the PauseEventRecord Class.
  #[sdk(child(office2010, qname = "p14:CT_MediaPlaybackEventRecord/p14:pauseEvt"))]
  P14PauseEvt(std::boxed::Box<PauseEventRecord>),
  /// Defines the ResumeEventRecord Class.
  #[sdk(child(office2010, qname = "p14:CT_MediaPlaybackEventRecord/p14:resumeEvt"))]
  P14ResumeEvt(std::boxed::Box<ResumeEventRecord>),
  /// Defines the SeekEventRecord Class.
  #[sdk(child(office2010, qname = "p14:CT_MediaSeekEventRecord/p14:seekEvt"))]
  P14SeekEvt(std::boxed::Box<SeekEventRecord>),
  /// Defines the NullEventRecord Class.
  #[sdk(child(office2010, qname = "p14:CT_NullEventRecord/p14:nullEvt"))]
  P14NullEvt(std::boxed::Box<NullEventRecord>),
}
#[derive(Clone, Debug, PartialEq, ooxmlsdk_derive::SdkChoice)]
pub enum ApplicationNonVisualDrawingPropertiesChoice {
  #[sdk(child(qname = "a:CT_AudioCD/a:audioCd"))]
  AAudioCd(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AudioFromCd>,
  ),
  #[sdk(child(qname = "a:CT_EmbeddedWAVAudioFile/a:wavAudioFile"))]
  AWavAudioFile(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::WaveAudioFile>,
  ),
  #[sdk(child(qname = "a:CT_AudioFile/a:audioFile"))]
  AAudioFile(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::AudioFromFile>,
  ),
  #[sdk(child(qname = "a:CT_VideoFile/a:videoFile"))]
  AVideoFile(
    std::boxed::Box<crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::VideoFromFile>,
  ),
  #[sdk(child(qname = "a:CT_QuickTimeFile/a:quickTimeFile"))]
  AQuickTimeFile(
    std::boxed::Box<
      crate::schemas::schemas_openxmlformats_org_drawingml_2006_main::QuickTimeFromFile,
    >,
  ),
}
